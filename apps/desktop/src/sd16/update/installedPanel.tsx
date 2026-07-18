import type { UpdateControllerDeps } from './updateModel';
import {
  UI_FIELD_LABEL_STYLE,
  UI_PANEL_STYLE,
  UI_PANEL_TITLE_STYLE,
} from './updateModel';

export const INSTALLED_PANEL_ID = 'installed-panel';

export function InstalledPanel({ deps }: { deps: UpdateControllerDeps }) {
  const { installed } = deps;
  return (
    <section
      id={INSTALLED_PANEL_ID}
      data-testid="sd16-installed-panel"
      style={UI_PANEL_STYLE}
    >
      <h2 style={UI_PANEL_TITLE_STYLE}>Installed</h2>
      <dl style={{ margin: 0, padding: 0 }}>
        {row('Channel', installed.channel)}
        {row('Version', installed.version)}
        {row('Source commit', installed.sourceCommit ?? 'unknown')}
        {row('Artifact SHA-256', installed.artifactSha256 ?? 'unknown')}
        {row('Install kind', installed.installKind)}
        {row('Managed path', installed.managedExecutablePath ?? 'unknown')}
        {row('Update eligible', installed.updateEligible ? 'yes' : 'no')}
        {row('Ineligible reason', installed.ineligibleReason ?? 'n/a')}
      </dl>
    </section>
  );
}

function row(label: string, value: string) {
  return (
    <div
      key={label}
      style={{ display: 'flex', margin: '2px 0' }}
      data-installed-row={label.toLowerCase().replace(/[^a-z0-9]+/g, '-')}
    >
      <dt style={{ ...UI_FIELD_LABEL_STYLE, fontWeight: 500 }}>{label}</dt>
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
