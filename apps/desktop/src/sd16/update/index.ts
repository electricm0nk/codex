/**
 * SD16-E6-F3b — public barrel for the shell update eligibility + diagnostics
 * surface. F3a (fetch/parse) and F3c (UI) import the types and helpers from
 * here rather than reaching into individual modules.
 */

export {
  compareVersions,
  decideEligibility,
} from './eligibility';
export type {
  ChannelLabel,
  EligibilityDecision,
  EligibilityInput,
  EligibilityResult,
  FetchStatus,
  InstallKind,
} from './eligibility';

export {
  DIAGNOSTIC_PLACEHOLDER,
  defaultInstalledDiagnostics,
  defaultLastCheckDiagnostics,
  defaultPendingRollbackDiagnostics,
  renderInstalledDiagnostics,
  renderLastCheckDiagnostics,
  renderPendingRollbackDiagnostics,
} from './diagnostics';
export type {
  InstalledDiagnostics,
  InstalledDiagnosticsView,
  LastCheckDiagnostics,
  LastCheckDiagnosticsView,
  PendingRollbackDiagnostics,
  PendingRollbackDiagnosticsView,
  PendingUpdateState,
  RollbackState,
} from './diagnostics';
