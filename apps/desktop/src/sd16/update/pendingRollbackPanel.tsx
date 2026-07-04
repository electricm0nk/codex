import type { Sd16UpdateControllerDeps } from './updateModel';
import {
  SD16_UI_FIELD_LABEL_STYLE,
  SD16_UI_PANEL_STYLE,
  SD16_UI_PANEL_TITLE_STYLE,
} from './updateModel';

export const PENDING_ROLLBACK_PANEL_ID = 'pending-rollback-panel';

/**
 * E7's carryover panel. Until the AppImage staged transaction lands, the
 * panel surfaces deterministic placeholders (`unknown`, `0`, `false`).
 * The contract this slice owns is that every named key in
 * `technical-requirements.md` §"Diagnostics Requirements" is rendered as
 * a row — values being unknown is acceptable, an absent field is not.
 */
export function Sd16PendingRollbackPanel({
  deps,
}: {
  deps: Sd16UpdateControllerDeps;
}) {
  const { pendingRollback } = deps;
  return (
    <section
      id={PENDING_ROLLBACK_PANEL_ID}
      data-testid="sd16-pending-rollback-panel"
      style={SD16_UI_PANEL_STYLE}
    >
      <h2 style={SD16_UI_PANEL_TITLE_STYLE}>Pending / Rollback</h2>
      <dl style={{ margin: 0, padding: 0 }}>
        {row('Pending update state', pendingRollback.pendingUpdateState)}
        {row(
          'Previous version available',
          pendingRollback.previousVersionAvailable ? 'yes' : 'no',
        )}
        {row('Rollback state', pendingRollback.rollbackState)}
        {row('Backup count', String(pendingRollback.backupCount))}
        {row(
          'Retained update bytes',
          String(pendingRollback.retainedUpdateStorageBytes),
        )}
      </dl>
      <p
        id="sd16-pending-rollback-source-note"
        data-testid="sd16-pending-rollback-source-note"
        style={{
          margin: '8px 0 0 0',
          fontSize: '11px',
          color: '#57606a',
          fontStyle: 'italic',
        }}
      >
        Values are populated by the SD-16-E7 staged-transaction module;
        until that surface lands, defaults are deterministic placeholders.
      </p>
    </section>
  );
}

function row(label: string, value: string) {
  return (
    <div
      key={label}
      style={{ display: 'flex', margin: '2px 0' }}
      data-pending-rollback-row={label.toLowerCase().replace(/[^a-z0-9]+/g, '-')}
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
