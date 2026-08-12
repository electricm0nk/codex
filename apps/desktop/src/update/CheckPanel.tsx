import type { UpdateControllerDeps, UpdateChannelLabel } from './updateModel';
import {
  UI_BUTTON_BASE_STYLE,
  UI_BUTTON_DISABLED_STYLE,
  UI_NOTES_BLOCK_STYLE,
  UI_PANEL_STYLE,
  UI_PANEL_TITLE_STYLE,
} from './updateModel';

export interface CheckPanelProps {
  deps: UpdateControllerDeps;
  checkInProgress: boolean;
  onCheck: (channel: UpdateChannelLabel) => void;
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
export function CheckPanel({ deps, checkInProgress, onCheck }: CheckPanelProps) {
  const { controller, lastCheck, releaseNotes } = deps;
  const channel = lastCheck.selectedChannel;
  const buttonStyle =
    checkInProgress || lastCheck.manifestStatus === 'in-progress'
      ? UI_BUTTON_DISABLED_STYLE
      : UI_BUTTON_BASE_STYLE;

  return (
    <section
      id="check-panel"
      data-testid="check-panel"
      style={UI_PANEL_STYLE}
    >
      <h2 style={UI_PANEL_TITLE_STYLE}>Check for updates</h2>
      <p style={{ margin: '0 0 8px 0' }}>
        Channel:{' '}
        <strong id="check-channel" data-testid="check-channel">
          {channel}
        </strong>
      </p>
      <p style={{ margin: '0 0 12px 0' }}>
        Last check:{' '}
        <span id="check-status" data-testid="check-status">
          {describeCheckStatus(lastCheck.indexStatus, lastCheck.manifestStatus)}
        </span>
      </p>
      <button
        id={CHECK_BUTTON_ID}
        data-testid="check-button"
        type="button"
        disabled={
          checkInProgress ||
          lastCheck.indexStatus === 'in-progress' ||
          lastCheck.manifestStatus === 'in-progress'
        }
        style={buttonStyle}
        onClick={() => onCheck(channel)}
      >
        {checkInProgress ? 'Checking\u2026' : 'Check'}
      </button>
      <div
        id={RELEASE_NOTES_ID}
        data-testid="release-notes"
        style={{ marginTop: '12px' }}
      >
        <h3 style={{ ...UI_PANEL_TITLE_STYLE, marginTop: '8px' }}>
          Release notes
        </h3>
        {renderReleaseNotes(releaseNotes)}
      </div>
    </section>
  );
}

function describeCheckStatus(
  indexStatus: UpdateControllerDeps['lastCheck']['indexStatus'],
  manifestStatus: UpdateControllerDeps['lastCheck']['manifestStatus'],
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
  notes: UpdateControllerDeps['releaseNotes'],
) {
  if (!notes) {
    return (
      <p
        data-testid="release-notes-empty"
        style={{ margin: 0, fontStyle: 'italic', color: 'var(--color-text-muted)' }}
      >
        No release notes available yet. Run Check to load them.
      </p>
    );
  }
  return (
    <div style={UI_NOTES_BLOCK_STYLE} aria-label={`Release notes for ${notes.releaseVersion}`}>
      {notes.body ?? '(release notes body is unavailable)'}
    </div>
  );
}
