import type { SupportTier } from '../status/createWorkbenchStatus';

/**
 * Visible/result vocabulary for the SD-11 check/update action surface.
 *
 * These are the tester-facing outcome distinctions SD-11 must preserve when it
 * consumes accepted SD-12 release truth. SD-11 is a *consumer* of that truth, not
 * the release control plane, so this module never mints governed release state — it
 * only classifies the honest outcome of a bounded check.
 */
export type UpdateResultState =
  | 'up-to-date'
  | 'update-available'
  | 'manual-only'
  | 'blocked'
  | 'withdrawn'
  | 'unsupported'
  | 'check-failed'
  | 'no-official-release-for-this-build';

/** SD-12 manifest `updateEligibilityState`. */
export type ManifestEligibilityState =
  | 'automatic'
  | 'manual-only'
  | 'unsupported'
  | 'withdrawn'
  | 'blocked';

/** SD-12 rollback/withdrawal lifecycle state for a release unit. */
export type ReleaseLifecycleState = 'active' | 'superseded' | 'withdrawn' | 'blocked';

export type TesterChannelLabel = 'alpha' | 'beta' | 'stable';

/** Running-build identity used to classify the check honestly against the actual platform. */
export interface UpdateCheckContext {
  buildLabel: string;
  buildVersion: string;
  platformLabel: string;
  platformTier: SupportTier;
  testerChannelLabel: TesterChannelLabel;
}

/** Minimum build identity carried by an SD-12 manifest entry. */
export interface UpdateBuildIdentity {
  releaseId: string;
  version: string;
  buildLabel: string;
  commitOrProvenanceHandle: string;
  publishedAt: string;
}

/**
 * Integrity material the SD-12 provenance/eligibility gate requires before any
 * platform may be treated as `automatic`. Linux requires all of these *and*
 * first-class tier; macOS/Windows can never satisfy the automatic gate in this tranche.
 */
export interface UpdateIntegrityMaterial {
  checksumAvailable: boolean;
  provenanceAvailable: boolean;
  manifestAssetResolved: boolean;
  linuxArtifactPresent: boolean;
  recoveryPostureDefined: boolean;
}

/** Consumer view over the SD-12 channel/platform manifest minimum fields. */
export interface UpdateManifestView {
  manifestVersion: string;
  channel: string;
  operatorPromotionPathReference: string;
  platform: string;
  supportTier: string;
  currentBuild: UpdateBuildIdentity;
  latestEligibleBuild: UpdateBuildIdentity;
  lifecycleState: ReleaseLifecycleState;
  eligibilityState: ManifestEligibilityState;
  integrity: UpdateIntegrityMaterial;
  replacementReleaseId: string | null;
  recoveryTarget: string | null;
  notes: string[];
}

/**
 * Release truth as returned by the runtime boundary. Official update truth comes
 * only from governed GitHub-backed release units; a local/feature build resolves to
 * `no-official-release`, and a boundary that cannot prove truth resolves to `check-failed`
 * rather than counterfeiting success.
 */
export type UpdateReleaseTruth =
  | {
      kind: 'no-official-release';
      reason: string;
      buildLabel: string;
      version: string;
    }
  | {
      kind: 'check-failed';
      reason: string;
      buildLabel: string;
      version: string;
    }
  | {
      kind: 'governed-release';
      manifest: UpdateManifestView;
    };

export interface UpdateActionResult {
  state: UpdateResultState;
  headline: string;
  detail: string;
  platformLabel: string;
  platformTier: SupportTier;
  testerChannelLabel: TesterChannelLabel;
  /** True only when Linux first-class AND eligibility automatic AND the full integrity gate is satisfied. */
  automaticEligible: boolean;
  /** Why automatic update is not offered (platform tier, integrity gate, or manifest posture). */
  manualReason: string | null;
  /** Preferred replacement release id when superseded/withdrawn. */
  replacementTarget: string | null;
  /** Recovery direction when withdrawn/blocked. */
  recoveryDirection: string | null;
  checkedBuildLabel: string;
  checkedVersion: string;
  operatorPromotionPathReference: string | null;
  evidenceNotes: string[];
}
