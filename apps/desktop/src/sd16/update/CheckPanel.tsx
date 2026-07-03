import type { Sd16UpdateControllerDeps, Sd16UpdateChannelLabel } from './updateModel';
import {
  SD16_UI_BUTTON_BASE_STYLE,
  SD16_UI_BUTTON_DISABLED_STYLE,
  SD16_UI_NOTES_BLOCK_STYLE,
  SD16_UI_PANEL_STYLE,
  SD16_UI_PANEL_TITLE_STYLE,
} from './updateModel';

export interface Sd16CheckPanelProps {
  deps: Sd16UpdateControllerDeps;
  checkInProgress: boolean;
  onCheck: (channel: Sd16UpdateChannelLabel) => void;
}

const CHECK_BUTTON_ID = 'check-button';
const RELEASE_NOTES_ID = 'release-notes';

/**
 * The bounded Check control. Surfaces the action button and the resulting
 * release notes block. AV-UI-3 asserts that release notes and the
 * diagnostics panel are both surfaced after a successful Check; this
 * component renders the notes block and reads the installed/last-check
 * state to drive the button label.
 */
export function Sd16CheckPanel({ deps, checkInProgress, onCheck }: Sd16CheckPanelProps) {
  const { controller, lastCheck, releaseNotes } = deps;
  const channel = lastCheck.selectedChannel;
  const buttonStyle =
    checkInProgress || lastCheck.manifestStatus === 'in-progress'
      ? SD16_UI_BUTTON_DISABLED_STYLE
      : SD16_UI_BUTTON_BASE_STYLE;

  return (
    <section
      id="check-panel"
      data-testid="sd16-check-panel"
      style={SD16_UI_PANEL_STYLE}
    >
      <h2 style={SD16_UI_PANEL_TITLE_STYLE}>Check for updates</h2>
      <p style={{ margin: '0 0 8px 0' }}>
        Channel:{' '}
        <strong id="check-channel" data-testid="sd16-check-channel">
          {channel}
        </strong>
      </p>
      <p style={{ margin: '0 0 12px 0' }}>
        Last check:{' '}
        <span id="check-status" data-testid="sd16-check-status">
          {describeCheckStatus(lastCheck.indexStatus, lastCheck.manifestStatus)}
        </span>
      </p>
      <button
        id={CHECK_BUTTON_ID}
        data-testid="sd16-check-button"
        type="button"
        disabled={checkInProgress || controller === undefined}
        style={buttonStyle}
        onClick={() => onCheck(channel)}
      >
        {checkInProgress ? 'Checking…' : 'Check'}
      </button>
      <div
        id={RELEASE_NOTES_ID}
        data-testid="sd16-release-notes"
        style={{ marginTop: '12px' }}
      >
        <h3 style={{ ...SD16_UI_PANEL_TITLE_STYLE, marginTop: '8px' }}>
          Release notes
        </h3>
        {renderReleaseNotes(releaseNotes)}
      </div>
    </section>
  );
}

function describeCheckStatus(
  indexStatus: Sd16UpdateControllerDeps['lastCheck']['indexStatus'],
  manifestStatus: Sd16UpdateControllerDeps['lastCheck']['manifestStatus'],
): string {
  if (indexStatus === 'not-loaded' && manifestStatus === 'not-loaded') {
    return 'check has not been run yet';
  }
  if (indexStatus === 'in-progress' || manifestStatus === 'in-progress') {
    return 'check in progress';
  }
  if (indexStatus === 'failed' || manifestStatus === 'failed') {
    return 'check failed — see Install disabled reason';
  }
  if (indexStatus === 'ok' && manifestStatus === 'ok') {
    return 'check ok';
  }
  return 'check partial';
}

function renderReleaseNotes(
  notes: Sd16UpdateControllerDeps['releaseNotes'],
) {
  if (!notes) {
    return (
      <p
        data-testid="sd16-release-notes-empty"
        style={{ margin: 0, fontStyle: 'italic', color: '#57606a' }}
      >
        No release notes available yet. Run Check to load them.
      </p>
    );
  }
  return (
    <div style={SD16_UI_NOTES_BLOCK_STYLE} aria-label={`Release notes for ${notes.releaseVersion}`}>
      {notes.body ?? '(release notes body is unavailable)'}
    </div>
  );
}
