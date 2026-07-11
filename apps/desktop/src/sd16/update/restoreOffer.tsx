/**
 * SD-16-E7-F3c restore-offer panel.
 *
 * On `verification_failed`, the desktop shell renders `#restore-prior` with
 * the prior version string so the operator can elect to roll back via
 * F3b's `perform_restore_previous`. The DOM contract surface is the
 * `#restore-prior` id; the F2 handoff names this AV-RB-2.
 *
 * F3c owns this surface; F3b owns the restore mechanism. Until F3b lands,
 * `RestoreOffer` displays the offer text and a `data-restore-available`
 * attribute that F3b's wiring will toggle when the restore path is wired.
 */

import type { CSSProperties } from 'react';

export const RESTORE_OFFER_ID = 'restore-prior';

const RESTORE_OFFER_STYLE: CSSProperties = {
  padding: '8px 12px',
  margin: '8px 0',
  border: '1px solid var(--color-border)',
  borderRadius: '4px',
  backgroundColor: 'var(--color-warn-bg)',
  fontSize: '13px',
  color: 'var(--color-text-muted)',
};

export interface RestoreOfferProps {
  /**
   * The version the operator can roll back to. Sourced from the
   * `pending-update.json` `from_version` field — F3a's pending record
   * remains on disk after a verification mismatch so this value is
   * recoverable without F3b integration.
   */
  priorVersion: string;
  /**
   * Whether the restore path is currently wired. False until F3b lands
   * `perform_restore_previous`; the operator-facing copy degrades to a
   * "restore not yet wired" notice so the UI never advertises an
   * action it cannot perform.
   */
  restoreAvailable: boolean;
}

/**
 * Render the `#restore-prior` panel with the prior version and the
 * restore-available state. Pure: no Tauri invocation, no side effects.
 * The parent UI is responsible for fetching the prior version from
 * `pending-update.json` and forwarding it as a prop.
 */
export function Sd16RestoreOffer({ priorVersion, restoreAvailable }: RestoreOfferProps) {
  return (
    <section
      id={RESTORE_OFFER_ID}
      data-testid="sd16-restore-offer"
      data-prior-version={priorVersion}
      data-restore-available={restoreAvailable ? 'true' : 'false'}
      style={RESTORE_OFFER_STYLE}
    >
      {restoreAvailable
        ? `Restore previous version: ${priorVersion}`
        : `Restore previous version (${priorVersion}) — restore mechanism not yet wired by F3b`}
    </section>
  );
}

/**
 * Pure renderer used by the test surface — produces the markup string
 * without React's runtime. Mirrors `Sd16RestoreOffer` 1:1 so tests can
 * assert on the wire contract in node tsx hosts without a React renderer.
 */
export function buildRestoreOfferMarkup(props: RestoreOfferProps): string {
  const restoreAttr = props.restoreAvailable ? 'true' : 'false';
  const text = props.restoreAvailable
    ? `Restore previous version: ${escapeText(props.priorVersion)}`
    : `Restore previous version (${escapeText(props.priorVersion)}) — restore mechanism not yet wired by F3b`;
  const attrs = [
    `id="${RESTORE_OFFER_ID}"`,
    'data-testid="sd16-restore-offer"',
    `data-prior-version="${escapeAttr(props.priorVersion)}"`,
    `data-restore-available="${restoreAttr}"`,
  ].join(' ');
  return `<section ${attrs}>${text}</section>`;
}

function escapeAttr(value: string): string {
  return value.replace(/&/g, '&amp;').replace(/"/g, '&quot;').replace(/</g, '&lt;');
}

function escapeText(value: string): string {
  return value.replace(/&/g, '&amp;').replace(/</g, '&lt;');
}