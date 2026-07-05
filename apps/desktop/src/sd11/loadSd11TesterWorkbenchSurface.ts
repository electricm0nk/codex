import type {
  Sd12ReleaseTruthRequest,
  Sd12ReleaseTruthSnapshot,
} from '../boundary/loadSd12ReleaseTruth';
import type { Ge08AuthoringWorkbenchRequest, Ge08AuthoringWorkbenchSnapshot } from '../boundary/loadGe08AuthoringWorkbench';
import type { PilotShellSnapshot } from '../boundary/loadPilotShellSnapshot';
import type {
  Sd13SupportStateMatrixSnapshot,
  Sd13SupportStateRow,
} from '../boundary/loadSd13SupportStateMatrix';
import {
  buildFallbackDiagnostics,
  buildFallbackExplanationRefs,
  buildGe08Diagnostics,
  buildGe08ExplanationRefs,
  buildGe08ProvenanceRefs,
  type Sd11WorkbenchDiagnostic,
  type Sd11WorkbenchReference,
} from './diagnostics/buildSd11WorkbenchEvidence';
import {
  createSd11WorkbenchStatus,
  formatSd11WorkbenchBuildLabel,
  type Sd11SupportTier,
  type Sd11WorkbenchStatus,
} from './status/createSd11WorkbenchStatus';
import { formatError } from '../boundary/runtime';

export interface Sd11WorkbenchRuntimeContext {
  buildVersion: string;
  platformLabel: string;
}

export interface Sd11WorkbenchDependencies {
  loadGe08AuthoringWorkbench: (
    request: Ge08AuthoringWorkbenchRequest
  ) => Promise<Ge08AuthoringWorkbenchSnapshot>;
  loadPilotShellSnapshot: () => Promise<PilotShellSnapshot>;
  loadSd12ReleaseTruth: (
    request: Sd12ReleaseTruthRequest
  ) => Promise<Sd12ReleaseTruthSnapshot>;
  /**
   * Read-only SD-13 support-state bridge. Optional so SD-11 callers that predate
   * this slice remain valid; when absent, the support/debt section reports an
   * explicit unavailable notice instead of fabricating support labels.
   */
  loadSd13SupportStateMatrix?: () => Promise<Sd13SupportStateMatrixSnapshot>;
}

/**
 * One read-only SD-13 support/debt row projected for tester presentation.
 *
 * Every field mirrors the SD-13 matrix truth verbatim. `hasDebtNote` is a pure
 * convenience flag derived from the presence of a blocker/lossiness note; it does
 * not alter, hide, or promote any state.
 */
export interface Sd11SupportDebtRow {
  rowId: string;
  subjectType: string;
  subjectId: string;
  dimension: string;
  supportState: string;
  evidenceTier: string;
  testerFacingStateLabel: string;
  groundingRef: string;
  blockerOrLossinessNote: string;
  nextRequiredUplift: string;
  hasDebtNote: boolean;
}

/** A per-state tally used only to orient testers; it never suppresses rows. */
export interface Sd11SupportDebtStateCount {
  supportState: string;
  count: number;
}

/**
 * Bounded SD-13 support/debt presentation structure derived from the matrix.
 *
 * This is intentionally separate from feedback evidence capture and from
 * update/support-tier status. It is read-only truth presentation only.
 */
export interface Sd11SupportDebtPresentation {
  sectionLabel: string;
  lead: string;
  dataSource: string | null;
  note: string | null;
  rows: Sd11SupportDebtRow[];
  stateCounts: Sd11SupportDebtStateCount[];
  unavailableNotice: string | null;
}

/**
 * One row of the bounded SD13-E7-F13 breadth-claim / evidence-refresh audit.
 *
 * Every field mirrors the projected SD-13 truth verbatim. `refreshConfirmed` and
 * `positiveBreadthClaim` are derived read-only flags over that truth; they never
 * mutate, hide, or promote a row. Grounding and evidence context are preserved so
 * the audit stays subordinate to the support/debt row truth it reads.
 */
export interface Sd11BreadthClaimAuditRow {
  rowId: string;
  subjectType: string;
  subjectId: string;
  supportState: string;
  evidenceTier: string;
  evidenceFreshness: string;
  refreshAuditLabel: string;
  groundingRef: string;
  blockerOrLossinessNote: string;
  nextRequiredUplift: string;
  /** True only when the projected freshness posture is a confirmed refresh. */
  refreshConfirmed: boolean;
  /** True only when the row is both `supported` and refresh-confirmed. */
  positiveBreadthClaim: boolean;
}

/** A per-freshness tally used only to orient testers; it never suppresses rows. */
export interface Sd11BreadthClaimFreshnessCount {
  evidenceFreshness: string;
  count: number;
}

/**
 * Bounded SD-13 breadth-claim / evidence-refresh audit derived from the projected
 * matrix. It is intentionally separate from, and subordinate to, the support/debt
 * presentation: it reports whether the current breadth claim surface is
 * refresh-backed or refresh-required, and never promotes an unsupported or
 * unconfirmed row into a positive breadth claim.
 */
export interface Sd11BreadthClaimAuditPresentation {
  sectionLabel: string;
  lead: string;
  overallPosture: 'refresh-backed' | 'refresh-required';
  overallLabel: string;
  refreshRequiredCount: number;
  positiveBreadthClaimCount: number;
  freshnessCounts: Sd11BreadthClaimFreshnessCount[];
  rows: Sd11BreadthClaimAuditRow[];
  dataSource: string | null;
  note: string | null;
  unavailableNotice: string | null;
}

export interface Sd11WorkbenchSummaryRow {
  label: string;
  value: string;
}

export interface Sd11TesterWorkbenchSurface {
  surfaceLabel: string;
  headline: string;
  lead: string;
  buildLabel: string;
  channelLabel: 'alpha';
  platformLabel: string;
  supportTierLabel: string;
  workflowName: string;
  workflowState: string;
  dataTruthLabel: string;
  fallbackNotice: string | null;
  boundedScopeNotice: string;
  feedbackStatusNotice: string;
  updateStatusLabel: string;
  summaryRows: Sd11WorkbenchSummaryRow[];
  diagnostics: Sd11WorkbenchDiagnostic[];
  blockedClaims: string[];
  explanationRefs: Sd11WorkbenchReference[];
  provenanceRefs: Sd11WorkbenchReference[];
  /**
   * SD-13 support/debt presentation. Optional in the type so SD-11 surface
   * literals that predate this slice stay valid; the live loader always
   * populates it.
   */
  supportDebt?: Sd11SupportDebtPresentation;
  /**
   * SD13-E7-F13 breadth-claim / evidence-refresh audit. Optional for the same
   * back-compat reason; the live loader always populates it. It is subordinate to
   * `supportDebt` and never promotes a row.
   */
  breadthClaimAudit?: Sd11BreadthClaimAuditPresentation;
  notes: string[];
  status: Sd11WorkbenchStatus;
}

const SUPPORT_DEBT_SECTION_LABEL = 'SD-13 core roster support and debt';
const SUPPORT_DEBT_LEAD =
  'Read-only SD-13 support-state and debt truth for the current bounded PF1 Core Rulebook roster. ' +
  'Support state, evidence tier, blocker/lossiness notes, grounding references, and next uplifts come ' +
  'straight from the SD-13 support-state matrix. Nothing here is promoted by app, build, or platform ' +
  'success, and no blocked, partial, lossy, or unverified row is hidden. SD13-E6-F12 remains deferred; ' +
  'this section does not capture evidence, submit issues, persist support truth, or alter update behavior.';

function mapSupportDebtRow(row: Sd13SupportStateRow): Sd11SupportDebtRow {
  return {
    rowId: row.rowId,
    subjectType: row.subjectType,
    subjectId: row.subjectId,
    dimension: row.dimension,
    supportState: row.supportState,
    evidenceTier: row.evidenceTier,
    testerFacingStateLabel: row.testerFacingStateLabel,
    groundingRef: row.groundingRef,
    blockerOrLossinessNote: row.blockerOrLossinessNote,
    nextRequiredUplift: row.nextRequiredUplift,
    hasDebtNote: row.blockerOrLossinessNote.trim().length > 0,
  };
}

function buildSupportDebtStateCounts(rows: Sd11SupportDebtRow[]): Sd11SupportDebtStateCount[] {
  const order = ['supported', 'partial', 'lossy', 'blocked', 'unverified'];
  const counts = new Map<string, number>();
  for (const row of rows) {
    counts.set(row.supportState, (counts.get(row.supportState) ?? 0) + 1);
  }

  const ordered: Sd11SupportDebtStateCount[] = [];
  for (const state of order) {
    const count = counts.get(state);
    if (count) {
      ordered.push({ supportState: state, count });
    }
  }
  // Preserve any unexpected state token rather than silently dropping it.
  for (const [state, count] of counts) {
    if (!order.includes(state)) {
      ordered.push({ supportState: state, count });
    }
  }
  return ordered;
}

function buildSupportDebtPresentation(
  snapshot: Sd13SupportStateMatrixSnapshot
): Sd11SupportDebtPresentation {
  const rows = snapshot.rows.map(mapSupportDebtRow);
  return {
    sectionLabel: SUPPORT_DEBT_SECTION_LABEL,
    lead: SUPPORT_DEBT_LEAD,
    dataSource: snapshot.dataSource,
    note: snapshot.note,
    rows,
    stateCounts: buildSupportDebtStateCounts(rows),
    unavailableNotice: null,
  };
}

function buildUnavailableSupportDebt(reason: string): Sd11SupportDebtPresentation {
  return {
    sectionLabel: SUPPORT_DEBT_SECTION_LABEL,
    lead: SUPPORT_DEBT_LEAD,
    dataSource: null,
    note: null,
    rows: [],
    stateCounts: [],
    unavailableNotice:
      `SD-13 support-state matrix unavailable: ${reason}. This section shows nothing rather than ` +
      'inventing local support labels; SD-13 truth must load through the read-only bridge before it can be presented.',
  };
}

const BREADTH_CLAIM_AUDIT_SECTION_LABEL = 'SD-13 breadth-claim evidence-refresh audit';
const BREADTH_CLAIM_AUDIT_LEAD =
  'Read-only SD13-E7-F13 audit derived from the same SD-13 matrix truth shown above. It is subordinate to the ' +
  'support/debt section: it does not own support state and it promotes nothing. It reports whether the current ' +
  'breadth claim surface is refresh-backed by confirmed evidence or still refresh-required, using the freshness ' +
  'posture carried by the SD-13 carrier. This first slice records no completed refresh checkpoint, so the whole ' +
  'surface is refresh-required; no partial, lossy, blocked, or unverified row — and no row without confirmed ' +
  'fresh evidence — is collapsed into a positive breadth claim.';

/**
 * Whether a projected freshness token asserts a completed refresh checkpoint.
 *
 * No token in the current SD-13 seed does, so this is `false` today. It is the
 * single place that decision lives, so a later slice that introduces a genuinely
 * refresh-confirmed posture only edits here rather than scattering the rule.
 */
function isRefreshConfirmedFreshness(_evidenceFreshness: string): boolean {
  return false;
}

function mapBreadthClaimAuditRow(row: Sd13SupportStateRow): Sd11BreadthClaimAuditRow {
  const refreshConfirmed = isRefreshConfirmedFreshness(row.evidenceFreshness);
  return {
    rowId: row.rowId,
    subjectType: row.subjectType,
    subjectId: row.subjectId,
    supportState: row.supportState,
    evidenceTier: row.evidenceTier,
    evidenceFreshness: row.evidenceFreshness,
    refreshAuditLabel: row.refreshAuditLabel,
    groundingRef: row.groundingRef,
    blockerOrLossinessNote: row.blockerOrLossinessNote,
    nextRequiredUplift: row.nextRequiredUplift,
    refreshConfirmed,
    // A positive breadth claim requires BOTH support and confirmed-fresh evidence.
    // This is what keeps partial/lossy/blocked/unverified or unconfirmed rows from
    // ever being read as a promoted claim.
    positiveBreadthClaim: row.supportState === 'supported' && refreshConfirmed,
  };
}

function buildBreadthClaimFreshnessCounts(
  rows: Sd11BreadthClaimAuditRow[]
): Sd11BreadthClaimFreshnessCount[] {
  const order = ['refreshable-from-live-proof', 'awaiting-initial-evidence'];
  const counts = new Map<string, number>();
  for (const row of rows) {
    counts.set(row.evidenceFreshness, (counts.get(row.evidenceFreshness) ?? 0) + 1);
  }

  const ordered: Sd11BreadthClaimFreshnessCount[] = [];
  for (const freshness of order) {
    const count = counts.get(freshness);
    if (count) {
      ordered.push({ evidenceFreshness: freshness, count });
    }
  }
  // Preserve any unexpected freshness token rather than silently dropping it.
  for (const [freshness, count] of counts) {
    if (!order.includes(freshness)) {
      ordered.push({ evidenceFreshness: freshness, count });
    }
  }
  return ordered;
}

function buildBreadthClaimAudit(
  snapshot: Sd13SupportStateMatrixSnapshot
): Sd11BreadthClaimAuditPresentation {
  const rows = snapshot.rows.map(mapBreadthClaimAuditRow);
  const refreshBacked = rows.length > 0 && rows.every((row) => row.refreshConfirmed);
  const positiveBreadthClaimCount = rows.filter((row) => row.positiveBreadthClaim).length;
  return {
    sectionLabel: BREADTH_CLAIM_AUDIT_SECTION_LABEL,
    lead: BREADTH_CLAIM_AUDIT_LEAD,
    overallPosture: refreshBacked ? 'refresh-backed' : 'refresh-required',
    overallLabel: refreshBacked
      ? 'Breadth claim surface is refresh-backed: every row carries confirmed fresh evidence.'
      : 'Breadth claim surface is refresh-required: no row carries a confirmed evidence-refresh checkpoint yet, so no breadth claim is promoted on freshness grounds.',
    refreshRequiredCount: rows.filter((row) => !row.refreshConfirmed).length,
    positiveBreadthClaimCount,
    freshnessCounts: buildBreadthClaimFreshnessCounts(rows),
    rows,
    dataSource: snapshot.dataSource,
    note: snapshot.note,
    unavailableNotice: null,
  };
}

function buildUnavailableBreadthClaimAudit(reason: string): Sd11BreadthClaimAuditPresentation {
  return {
    sectionLabel: BREADTH_CLAIM_AUDIT_SECTION_LABEL,
    lead: BREADTH_CLAIM_AUDIT_LEAD,
    overallPosture: 'refresh-required',
    overallLabel:
      'Breadth claim audit unavailable: with no SD-13 truth loaded, the surface must stay refresh-required rather than imply freshness.',
    refreshRequiredCount: 0,
    positiveBreadthClaimCount: 0,
    freshnessCounts: [],
    rows: [],
    dataSource: null,
    note: null,
    unavailableNotice:
      `SD-13 breadth-claim audit unavailable: ${reason}. This section shows nothing rather than inventing local ` +
      'freshness labels; SD-13 truth must load through the read-only bridge before any audit posture can be presented.',
  };
}

const DEFAULT_REQUEST: Ge08AuthoringWorkbenchRequest = {
  packageRoot: 'resources/ge08/guard-stance-package',
};

export async function loadSd11TesterWorkbenchSurface(
  context: Sd11WorkbenchRuntimeContext,
  dependencies: Sd11WorkbenchDependencies
): Promise<Sd11TesterWorkbenchSurface> {
  const releaseTruthRequest = buildReleaseTruthRequest(context);
  const releaseTruthPromise = dependencies.loadSd12ReleaseTruth(releaseTruthRequest).catch((cause: unknown) => {
    const reason = `Release-truth bridge failed: ${formatError(cause)}`;
    const normalised = releaseTruthRequest.platformLabel.trim().toLowerCase();
    const platformTier: Sd11SupportTier =
      normalised === 'linux'
        ? 'first-class'
        : normalised === 'macos'
          ? 'second-class'
          : normalised === 'windows'
            ? 'third-class'
            : 'unknown';

    return {
      truth: {
        kind: 'check-failed' as const,
        reason,
        buildLabel: releaseTruthRequest.buildLabel,
        version: releaseTruthRequest.buildVersion,
      },
      updateAction: {
        state: 'check-failed' as const,
        headline: 'Update check failed',
        detail: reason,
        platformLabel: releaseTruthRequest.platformLabel,
        platformTier,
        testerChannelLabel: releaseTruthRequest.testerChannelLabel,
        automaticEligible: false,
        manualReason: null,
        replacementTarget: null,
        recoveryDirection: null,
        checkedBuildLabel: releaseTruthRequest.buildLabel,
        checkedVersion: releaseTruthRequest.buildVersion,
        operatorPromotionPathReference: null,
        evidenceNotes: [],
      },
      issueCapture: {
        releaseUnitId: null,
        sourceRevision: null,
        manifestPath: null,
        updateEligibilityState: 'check-failed',
        trustGateStatus: 'unverified-runtime-check-failed',
        replacementReleaseId: null,
        officialSurface:
          'GitHub release assets published by .github/workflows/publish-tester-release.yml and consumed via apps/desktop/src/boundary/loadSd11UpdateAction.ts over the F3a fetch pipeline',
        localBuildAuthority: reason,
      },
    };
  });
  const sd13Promise = loadSd13Presentations(dependencies);

  try {
    const [snapshot, releaseTruth] = await Promise.all([
      dependencies.loadGe08AuthoringWorkbench(DEFAULT_REQUEST),
      releaseTruthPromise,
    ]);
    return mapGe08Snapshot(context, snapshot, releaseTruth, await sd13Promise);
  } catch (cause: unknown) {
    const [fallbackSnapshot, releaseTruth] = await Promise.all([
      dependencies.loadPilotShellSnapshot(),
      releaseTruthPromise,
    ]);
    return mapPilotFallback(context, fallbackSnapshot, formatError(cause), releaseTruth, await sd13Promise);
  }
}

/**
 * The two read-only SD-13 presentations the workbench derives from one snapshot:
 * the existing support/debt truth and the SD13-E7-F13 breadth-claim audit that is
 * subordinate to it.
 */
interface Sd13WorkbenchPresentations {
  supportDebt: Sd11SupportDebtPresentation;
  breadthClaimAudit: Sd11BreadthClaimAuditPresentation;
}

/**
 * Load the SD-13 support/debt presentation and the subordinate breadth-claim audit
 * from a single read-only snapshot. The dependency is optional so existing SD-11
 * callers that predate this slice keep compiling; when it is absent or fails, both
 * sections render explicit unavailable notices rather than inventing local support
 * or freshness labels. The snapshot is fetched once so both derivations read the
 * exact same projected truth.
 */
async function loadSd13Presentations(
  dependencies: Sd11WorkbenchDependencies
): Promise<Sd13WorkbenchPresentations> {
  if (!dependencies.loadSd13SupportStateMatrix) {
    const reason = 'no SD-13 support-state bridge was provided to the workbench';
    return {
      supportDebt: buildUnavailableSupportDebt(reason),
      breadthClaimAudit: buildUnavailableBreadthClaimAudit(reason),
    };
  }

  try {
    const snapshot = await dependencies.loadSd13SupportStateMatrix();
    return {
      supportDebt: buildSupportDebtPresentation(snapshot),
      breadthClaimAudit: buildBreadthClaimAudit(snapshot),
    };
  } catch (cause: unknown) {
    const reason = formatError(cause);
    return {
      supportDebt: buildUnavailableSupportDebt(reason),
      breadthClaimAudit: buildUnavailableBreadthClaimAudit(reason),
    };
  }
}

function buildReleaseTruthRequest(context: Sd11WorkbenchRuntimeContext): Sd12ReleaseTruthRequest {
  return {
    buildVersion: context.buildVersion,
    buildLabel: formatSd11WorkbenchBuildLabel(context.buildVersion),
    platformLabel: context.platformLabel,
    testerChannelLabel: 'alpha',
  };
}

function mapGe08Snapshot(
  context: Sd11WorkbenchRuntimeContext,
  snapshot: Ge08AuthoringWorkbenchSnapshot,
  releaseTruth: Sd12ReleaseTruthSnapshot,
  sd13: Sd13WorkbenchPresentations
): Sd11TesterWorkbenchSurface {
  const status = createSd11WorkbenchStatus(context, releaseTruth);

  return {
    surfaceLabel: 'SD-11 tester workbench',
    headline: 'Bounded tester workbench over a real desktop command surface',
    lead:
      'This frame is the first SD-11 tester workbench slice: it presents one real bounded workflow over the Tauri command boundary, keeps diagnostics visible, and refuses to pretend broader product readiness.',
    buildLabel: status.build.label,
    channelLabel: status.channel.testerFacingLabel,
    platformLabel: status.support.platformLabel,
    supportTierLabel: status.support.tierMatrixLabel,
    workflowName: 'GE08 Guard Stance authoring workbench',
    workflowState: `${snapshot.packageState} / ${snapshot.preview.previewStatus}`,
    dataTruthLabel: 'Real Tauri command snapshot',
    fallbackNotice: null,
    boundedScopeNotice:
      'Bounded scope only: this slice proves the GE08 authoring workflow, not a full character-builder surface, GitHub submission transport, or updater mechanics.',
    feedbackStatusNotice:
      'Feedback intake is intentionally deferred in this slice. The frame keeps diagnostics, workflow identity, and synchronized release-truth evidence visible so later GitHub flows can consume honest evidence.',
    updateStatusLabel: status.update.label,
    summaryRows: [
      {
        label: 'Package',
        value: snapshot.packageManifest.packageId,
      },
      {
        label: 'Preview',
        value: snapshot.preview.previewStatus,
      },
      {
        label: 'Data source',
        value: snapshot.dataSource,
      },
      {
        label: 'Baseline AC',
        value: formatBaselineArmorClass(snapshot.preview.baselineArmorClass),
      },
    ],
    diagnostics: buildGe08Diagnostics(snapshot.preview.diagnostics),
    blockedClaims: snapshot.preview.blockedClaims,
    explanationRefs: buildGe08ExplanationRefs(snapshot.preview.explanationRefs),
    provenanceRefs: buildGe08ProvenanceRefs(snapshot.preview.provenanceRefs),
    supportDebt: sd13.supportDebt,
    breadthClaimAudit: sd13.breadthClaimAudit,
    notes: [snapshot.note],
    status,
  };
}

function mapPilotFallback(
  context: Sd11WorkbenchRuntimeContext,
  snapshot: PilotShellSnapshot,
  failure: string,
  releaseTruth: Sd12ReleaseTruthSnapshot,
  sd13: Sd13WorkbenchPresentations
): Sd11TesterWorkbenchSurface {
  const status = createSd11WorkbenchStatus(context, releaseTruth);

  return {
    surfaceLabel: 'SD-11 tester workbench',
    headline: 'Bounded tester workbench fallback over the pilot seam',
    lead:
      'The preferred GE08 workbench could not load, so the app drops to an explicitly labeled fallback. The fallback exists to preserve truthful runtime seams and must not masquerade as full product state.',
    buildLabel: status.build.label,
    channelLabel: status.channel.testerFacingLabel,
    platformLabel: status.support.platformLabel,
    supportTierLabel: status.support.tierMatrixLabel,
    workflowName: 'GE07 pilot snapshot seam',
    workflowState: snapshot.receiptStatus,
    dataTruthLabel:
      snapshot.dataSource === 'tauri-command'
        ? 'Explicit fallback over a Tauri pilot seam'
        : 'Explicit fallback placeholder',
    fallbackNotice:
      `GE08 authoring workbench unavailable: ${failure}. This fallback exists because the real bounded snapshot could not load and the UI must not counterfeit product truth.`,
    boundedScopeNotice:
      'Bounded scope only: this fallback preserves the pilot runtime seam and visible failure context. It does not claim the broader tester workbench, GitHub feedback transport, or updater behavior are implemented.',
    feedbackStatusNotice:
      'Feedback intake remains deferred. Use the visible fallback reason, diagnostics, workflow identity, and synchronized release-truth evidence as the current evidence surface until the richer SD-11 flows land.',
    updateStatusLabel: status.update.label,
    summaryRows: [
      {
        label: 'Case',
        value: snapshot.caseId,
      },
      {
        label: 'Receipt status',
        value: snapshot.receiptStatus,
      },
      {
        label: 'Data source',
        value: snapshot.dataSource,
      },
    ],
    diagnostics: buildFallbackDiagnostics(snapshot.diagnostics),
    blockedClaims: [],
    explanationRefs: buildFallbackExplanationRefs(snapshot.explanationRefs),
    provenanceRefs: [],
    supportDebt: sd13.supportDebt,
    breadthClaimAudit: sd13.breadthClaimAudit,
    notes: [snapshot.note],
    status,
  };
}

function formatBaselineArmorClass(
  baselineArmorClass: Ge08AuthoringWorkbenchSnapshot['preview']['baselineArmorClass']
): string {
  if (baselineArmorClass.kind === 'Computed') {
    return `${baselineArmorClass.value}`;
  }

  return `Blocked: ${baselineArmorClass.reason}`;
}

