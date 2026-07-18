import type { UpdateControllerDeps } from './updateModel';
import {
  UI_BADGE_DISABLED_STYLE,
  UI_BUTTON_DISABLED_STYLE,
  UI_BUTTON_ENABLED_STYLE,
  UI_PANEL_STYLE,
  UI_PANEL_TITLE_STYLE,
} from './updateModel';

export interface InstallControlProps {
  deps: UpdateControllerDeps;
  installInProgress: boolean;
  onInstall: () => void;
}

const INSTALL_BUTTON_ID = 'install-button';
const INSTALL_DISABLED_REASON_ID = 'install-disabled-reason';

/**
 * The Install button plus the disabled-reason badge.
 *
 * The button is **disabled unless `eligibility_result === "eligible"`**.
 * Every non-eligible state must surface an explicit reason string in the
 * `#install-disabled-reason` DOM hook so AV-UI-4, AV-UI-5, and AV-UI-6 all
 * have a deterministic handle to assert against.
 */
export function InstallControl({
  deps,
  installInProgress,
  onInstall,
}: InstallControlProps) {
  const { controller, installed, lastCheck } = deps;
  const eligibility = controller.computeEligibility(installed, lastCheck);
  const disabledReason = controller.disabledReason(installed, lastCheck);
  const eligible = eligibility === 'eligible';
  const blocked = !eligible || installInProgress;

  return (
    <section
      id="install-panel"
      data-testid="sd16-install-control"
      style={UI_PANEL_STYLE}
    >
      <h2 style={UI_PANEL_TITLE_STYLE}>Install</h2>
      <p style={{ margin: '0 0 8px 0' }}>
        Eligibility:{' '}
        <strong
          id="install-eligibility"
          data-testid="sd16-install-eligibility"
          data-eligibility={eligibility}
        >
          {eligibility}
        </strong>
        {!eligible ? (
          <span
            id={INSTALL_DISABLED_REASON_ID}
            data-testid="sd16-install-disabled-reason"
            data-reason={disabledReason ?? ''}
            style={UI_BADGE_DISABLED_STYLE}
          >
            {disabledReason ?? 'Install is unavailable for this build'}
          </span>
        ) : null}
      </p>
      <button
        id={INSTALL_BUTTON_ID}
        data-testid="sd16-install-button"
        type="button"
        disabled={blocked}
        data-eligible={eligible ? 'true' : 'false'}
        style={
          blocked
            ? UI_BUTTON_DISABLED_STYLE
            : UI_BUTTON_ENABLED_STYLE
        }
        onClick={() => {
          if (!blocked) {
            onInstall();
          }
        }}
      >
        {installInProgress ? 'Installing…' : 'Install'}
      </button>
    </section>
  );
}
