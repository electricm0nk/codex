/**
 * SD-16-E6 barrel.
 *
 * Exports the public surface of F3b (eligibility + diagnostics model) and
 * F3c (shell-update-UI). The runtime (F4 wiring) imports the page-level
 * entry `UpdateUi`; downstream F3a / F3b modules import the controller
 * types to satisfy the seam.
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

export {
  ChannelSelector,
  type ChannelSelectorProps,
} from './ChannelSelector';
export { CheckPanel } from './CheckPanel';
export { InstallControl } from './InstallControl';
export {
  InstalledPanel,
  INSTALLED_PANEL_ID,
} from './installedPanel';
export {
  LastCheckPanel,
  LAST_CHECK_PANEL_ID,
} from './lastCheckPanel';
export {
  PendingRollbackPanel,
  PENDING_ROLLBACK_PANEL_ID,
} from './pendingRollbackPanel';
export { UpdateUi, UPDATE_UI_ID } from './Ui';
export type { UpdateUiProps } from './Ui';

export {
  UPDATE_CHANNEL_OPTIONS,
  UNWIRED_CONTROLLER,
  buildUnwiredUpdateDeps,
  emptyInstalledState,
  emptyLastCheckState,
  emptyPendingRollbackState,
  UI_CONTAINER_STYLE,
  UI_PANEL_STYLE,
  UI_PANEL_TITLE_STYLE,
  UI_FIELD_LABEL_STYLE,
  UI_BADGE_DISABLED_STYLE,
  UI_BUTTON_BASE_STYLE,
  UI_BUTTON_DISABLED_STYLE,
  UI_BUTTON_ENABLED_STYLE,
  UI_NOTES_BLOCK_STYLE,
  type UpdateChannelLabel,
  type EligibilityResult,
  type InstallKind,
  type InstalledState,
  type LastCheckFetchStatus,
  type LastCheckState,
  type PendingRollbackState,
  type ReleaseNotes,
  type UpdateController,
  type UpdateControllerDeps,
} from './updateModel';
