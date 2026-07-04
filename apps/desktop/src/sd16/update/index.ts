/**
 * SD-16-E6 barrel.
 *
 * Exports the public surface of F3b (eligibility + diagnostics model) and
 * F3c (shell-update-UI). The runtime (F4 wiring) imports the page-level
 * entry `Sd16UpdateUi`; downstream F3a / F3b modules import the controller
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
  Sd16ChannelSelector,
  type Sd16ChannelSelectorProps,
} from './ChannelSelector';
export { Sd16CheckPanel } from './CheckPanel';
export { Sd16InstallControl } from './InstallControl';
export {
  Sd16InstalledPanel,
  INSTALLED_PANEL_ID,
} from './installedPanel';
export {
  Sd16LastCheckPanel,
  LAST_CHECK_PANEL_ID,
} from './lastCheckPanel';
export {
  Sd16PendingRollbackPanel,
  PENDING_ROLLBACK_PANEL_ID,
} from './pendingRollbackPanel';
export { Sd16UpdateUi, SD16_UPDATE_UI_ID } from './Ui';
export type { Sd16UpdateUiProps } from './Ui';

export {
  SD16_UPDATE_CHANNEL_OPTIONS,
  SD16_UNWIRED_CONTROLLER,
  buildUnwiredUpdateDeps,
  emptyInstalledState,
  emptyLastCheckState,
  emptyPendingRollbackState,
  SD16_UI_CONTAINER_STYLE,
  SD16_UI_PANEL_STYLE,
  SD16_UI_PANEL_TITLE_STYLE,
  SD16_UI_FIELD_LABEL_STYLE,
  SD16_UI_BADGE_DISABLED_STYLE,
  SD16_UI_BUTTON_BASE_STYLE,
  SD16_UI_BUTTON_DISABLED_STYLE,
  SD16_UI_BUTTON_ENABLED_STYLE,
  SD16_UI_NOTES_BLOCK_STYLE,
  type Sd16UpdateChannelLabel,
  type Sd16EligibilityResult,
  type Sd16InstallKind,
  type Sd16InstalledState,
  type Sd16LastCheckFetchStatus,
  type Sd16LastCheckState,
  type Sd16PendingRollbackState,
  type Sd16ReleaseNotes,
  type Sd16UpdateController,
  type Sd16UpdateControllerDeps,
} from './updateModel';
