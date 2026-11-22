import type { SupportTier } from '../status/createWorkbenchStatus';
import type {
  UpdateActionResult,
  UpdateCheckContext,
  UpdateManifestView,
  UpdateReleaseTruth,
} from './updateActionModel';
import { sanitizeReportableOutput } from '../feedback/evidence/sanitizeReportableOutput';

/**
 * Classify the honest outcome of a bounded SD-11 update check over accepted SD-12 release truth.
 *
 * This function is the product-truth gate. It never applies, downloads, or mutates anything; it only
 * decides which honest visible state the tester sees. The decisive SD-12 rules it enforces:
 *  - official update truth comes only from governed release units (never feature/local builds)
 *  - `automatic` is reachable only on Linux first-class with the full integrity gate satisfied
 *  - macOS/Windows are never promoted to automatic parity, even if a manifest claims it
 *  - withdrawn/blocked/superseded/recovery truth stays visible rather than collapsing into success/failure
 */
export function deriveWorkbenchUpdateAction(
  context: UpdateCheckContext,
  truth: UpdateReleaseTruth
): UpdateActionResult {
  if (truth.kind === 'no-official-release') {
    const reason = sanitizeReportableOutput(truth.reason);
    return baseResult(context, {
      state: 'no-official-release-for-this-build',
      headline: 'No official tester release for this build',
      detail:
        `This is a local, non-governed build (${truth.buildLabel}), so there is no governed GitHub-backed ` +
        `release unit to check against. ${reason} Official update truth only comes from governed release ` +
        `units, never from a feature or local build, so no update outcome is claimed.`,
    });
  }

  if (truth.kind === 'check-failed') {
    const reason = sanitizeReportableOutput(truth.reason);
    return baseResult(context, {
      state: 'check-failed',
      headline: 'Update check failed',
      detail:
        `The bounded update check could not complete and no success is claimed. Reason: ${reason}. ` +
        `Re-run the check once the desktop runtime boundary can prove official release truth.`,
    });
  }

  return deriveFromManifest(context, truth.manifest);
}

function deriveFromManifest(
  context: UpdateCheckContext,
  manifest: UpdateManifestView
): UpdateActionResult {
  const evidenceNotes = manifest.notes.slice();
  const operatorPromotionPathReference = manifest.operatorPromotionPathReference;

  const mismatches: string[] = [];
  if (manifest.platform.toLowerCase() !== context.platformLabel.toLowerCase()) {
    mismatches.push(`platform=${manifest.platform} (expected ${context.platformLabel})`);
  }
  if (manifest.channel !== context.testerChannelLabel) {
    mismatches.push(`channel=${manifest.channel} (expected ${context.testerChannelLabel})`);
  }
  if (manifest.supportTier !== context.platformTier) {
    mismatches.push(`supportTier=${manifest.supportTier} (expected ${context.platformTier})`);
  }

  if (mismatches.length) {
    return baseResult(context, {
      state: 'check-failed',
      headline: 'Update check failed',
      detail:
        `Governed release truth did not match the running build context (${mismatches.join(', ')}). ` +
        'Refusing to classify update state against a mismatched manifest.',
      operatorPromotionPathReference,
      evidenceNotes,
    });
  }

  const withResult = (
    partial: Pick<UpdateActionResult, 'state' | 'headline' | 'detail'> &
      Partial<UpdateActionResult>
  ): UpdateActionResult =>
    baseResult(context, { operatorPromotionPathReference, evidenceNotes, ...partial });
  // Withdrawn takes precedence: a withdrawn build must surface a recovery/replacement path,
  // never normal update state.
  if (manifest.lifecycleState === 'withdrawn' || manifest.eligibilityState === 'withdrawn') {
    const recovery = manifest.recoveryTarget ?? manifest.replacementReleaseId;
    return withResult({
      state: 'withdrawn',
      headline: 'This build was withdrawn',
      detail:
        `This build has been withdrawn and must no longer be used. ${describeNotes(manifest.notes)}` +
        (recovery
          ? `Recover to the approved governed build: ${recovery}.`
          : 'No replacement target was recorded; do not continue on this build until a governed recovery target is published.'),
      replacementTarget: manifest.replacementReleaseId,
      recoveryDirection: recovery,
    });
  }

  // Blocked: known build that must not be offered automatically.
  if (manifest.lifecycleState === 'blocked' || manifest.eligibilityState === 'blocked') {
    return withResult({
      state: 'blocked',
      headline: 'Automatic update blocked',
      detail:
        `Automatic update is blocked for this build and no apply path is offered. ${describeNotes(manifest.notes)}` +
        'Resolve the blocking condition or reclassify the release before update is re-enabled.',
      recoveryDirection: manifest.recoveryTarget,
    });
  }

  // Unsupported platform/channel for updates.
  if (manifest.eligibilityState === 'unsupported') {
    return withResult({
      state: 'unsupported',
      headline: `Updates unsupported for ${context.platformLabel}`,
      detail:
        `Governed updates are not supported for ${context.platformLabel} on the ${context.testerChannelLabel} ` +
        `channel in this tranche. ${describeNotes(manifest.notes)}This is an explicit unsupported posture, not a failure.`,
    });
  }

  const hasNewer =
    manifest.lifecycleState === 'superseded' ||
    manifest.latestEligibleBuild.releaseId !== manifest.currentBuild.releaseId;

  if (!hasNewer) {
    return withResult({
      state: 'up-to-date',
      headline: 'Up to date',
      detail:
        `This build (${manifest.currentBuild.buildLabel}) is the current governed ${context.testerChannelLabel} ` +
        `release for ${context.platformLabel}. No newer eligible build is published.`,
    });
  }

  const gateSatisfied = integrityGateSatisfied(manifest, context.platformTier);
  const automaticEligible =
    context.platformTier === 'first-class' && manifest.eligibilityState === 'automatic' && gateSatisfied;

  if (automaticEligible) {
    return withResult({
      state: 'update-available',
      headline: 'Update available',
      detail:
        `A newer governed ${context.testerChannelLabel} release is available for ${context.platformLabel}: ` +
        `${manifest.latestEligibleBuild.buildLabel} (${manifest.latestEligibleBuild.releaseId}). ` +
        'This Linux build satisfies the SD-12 integrity gate, so it is automatic-update eligible. ' +
        'This slice surfaces the eligible state only; it does not apply installers.',
      automaticEligible: true,
      replacementTarget: manifest.replacementReleaseId,
    });
  }

  return withResult({
    state: 'manual-only',
    headline: 'Manual update only',
    detail:
      `A newer governed ${context.testerChannelLabel} release exists for ${context.platformLabel}: ` +
      `${manifest.latestEligibleBuild.buildLabel} (${manifest.latestEligibleBuild.releaseId}), but automatic ` +
      'update is not offered. Retrieve and install it manually from the governed release unit.',
    manualReason: manualReason(context.platformTier, manifest.eligibilityState, gateSatisfied, context.platformLabel),
    replacementTarget: manifest.replacementReleaseId,
  });
}

/**
 * The SD-12 automatic eligibility gate. Only Linux first-class can satisfy it in this tranche, and only
 * when checksum, provenance, manifest asset resolution, the Linux artifact, and recovery posture are all present.
 * macOS and Windows can never satisfy it here — one platform's trust posture never authorizes another's.
 */
function integrityGateSatisfied(manifest: UpdateManifestView, tier: SupportTier): boolean {
  if (tier !== 'first-class') {
    return false;
  }
  const i = manifest.integrity;
  return (
    i.checksumAvailable &&
    i.provenanceAvailable &&
    i.manifestAssetResolved &&
    i.linuxArtifactPresent &&
    i.recoveryPostureDefined
  );
}

function manualReason(
  tier: SupportTier,
  eligibility: UpdateManifestView['eligibilityState'],
  gateSatisfied: boolean,
  platformLabel: string
): string {
  if (tier === 'second-class') {
    return `${platformLabel} is a second-class platform in this tranche and is never promoted to automatic update parity. Update manually from the governed release unit.`;
  }
  if (tier === 'third-class') {
    return `${platformLabel} is explicitly bounded as third-class in this tranche with no automatic-update claim. Update manually from the governed release unit.`;
  }
  if (tier === 'unknown') {
    return `${platformLabel} is outside the named support matrix, so no automatic-update claim is made.`;
  }
  if (eligibility !== 'automatic') {
    return 'The governed manifest marks this release manual-only rather than automatic for this platform.';
  }
  if (!gateSatisfied) {
    return 'The SD-12 integrity gate is not fully satisfied (checksum, provenance, manifest asset, Linux artifact, and recovery posture must all be present), so automatic update is withheld.';
  }
  return 'Automatic update is not available; update manually from the governed release unit.';
}

function describeNotes(notes: string[]): string {
  return notes.length ? `${notes.join(' ')} ` : '';
}

function baseResult(
  context: UpdateCheckContext,
  partial: Pick<UpdateActionResult, 'state' | 'headline' | 'detail'> & Partial<UpdateActionResult>
): UpdateActionResult {
  return {
    state: partial.state,
    headline: partial.headline,
    detail: partial.detail,
    platformLabel: context.platformLabel,
    platformTier: context.platformTier,
    testerChannelLabel: context.testerChannelLabel,
    automaticEligible: partial.automaticEligible ?? false,
    manualReason: partial.manualReason ?? null,
    replacementTarget: partial.replacementTarget ?? null,
    recoveryDirection: partial.recoveryDirection ?? null,
    checkedBuildLabel: context.buildLabel,
    checkedVersion: context.buildVersion,
    operatorPromotionPathReference: partial.operatorPromotionPathReference ?? null,
    evidenceNotes: partial.evidenceNotes ?? [],
  };
}
