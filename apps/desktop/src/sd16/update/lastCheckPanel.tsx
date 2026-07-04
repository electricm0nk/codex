import type { Sd16UpdateControllerDeps } from './updateModel';
import {
  SD16_UI_FIELD_LABEL_STYLE,
  SD16_UI_PANEL_STYLE,
  SD16_UI_PANEL_TITLE_STYLE,
} from './updateModel';

export const LAST_CHECK_PANEL_ID = 'last-check-panel';

export function Sd16LastCheckPanel({ deps }: { deps: Sd16UpdateControllerDeps }) {
  const { lastCheck } = deps;
  return (
    <section
      id={LAST_CHECK_PANEL_ID}
      data-testid="sd16-last-check-panel"
      style={SD16_UI_PANEL_STYLE}
    >
      <h2 style={SD16_UI_PANEL_TITLE_STYLE}>Last check</h2>
      <dl style={{ margin: 0, padding: 0 }}>
        {row('Selected channel', lastCheck.selectedChannel)}
        {row('Index URL', lastCheck.indexUrl || 'not yet resolved')}
        {row('Index status', lastCheck.indexStatus)}
        {row('Manifest status', lastCheck.manifestStatus)}
        {row('Release version', lastCheck.releaseVersion ?? 'unknown')}
        {row('Release notes', lastCheck.releaseNotesStatus)}
        {row('Eligibility', lastCheck.eligibilityResult)}
        {row('Install disabled reason', lastCheck.installDisabledReason ?? 'none')}
      </dl>
    </section>
  );
}

function row(label: string, value: string) {
  return (
    <div
      key={label}
      style={{ display: 'flex', margin: '2px 0' }}
      data-last-check-row={label.toLowerCase().replace(/[^a-z0-9]+/g, '-')}
    >
      <dt style={{ ...SD16_UI_FIELD_LABEL_STYLE, fontWeight: 500 }}>{label}</dt>
      <dd
        style={{
          margin: 0,
          fontFamily:
            'ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, monospace',
          fontSize: '12px',
          wordBreak: 'break-all',
        }}
      >
        {value}
      </dd>
    </div>
  );
}
