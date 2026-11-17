import { useEffect, useState } from 'react';
import { createPortal } from 'react-dom';
import { listKnownCampaigns } from '../boundary/campaignFolder/knownCampaigns';
import { setCharacterCampaigns, type CampaignMembershipDto } from '../boundary/setCharacterCampaigns';
import { applyCampaignChoices, campaignChoices, type CampaignChoices } from './campaignMembershipModel';

type Listing = { kind: 'loading' } | { kind: 'failed'; message: string } | ({ kind: 'listed' } & CampaignChoices);

/**
 * The sheet's Campaign dialog (SD-41 `decisions.md §29`, SD-42 `§11`): the known campaign folders,
 * a checkbox each (checked = every save of the character is copied to that folder's `Characters/`,
 * which is the campaign's party). Save writes the character's `campaigns.json`
 * (`set_character_campaigns`, which copies the character at once). Unchecking keeps the campaign's
 * copy and stops updating it.
 */
export function CampaignMembershipDialog(props: {
  characterId: string;
  memberships: readonly CampaignMembershipDto[];
  /** The saved memberships and every copy or party write that failed ("<dir>: <reason>"). */
  onSaved: (memberships: CampaignMembershipDto[], warnings: string[]) => void;
  onClose: () => void;
}) {
  const [listing, setListing] = useState<Listing>({ kind: 'loading' });
  const [checkedDirs, setCheckedDirs] = useState<string[]>([]);
  const [saving, setSaving] = useState(false);
  const [saveError, setSaveError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    const settle = (next: Listing) => {
      if (cancelled) return;
      setListing(next);
      if (next.kind === 'listed') {
        setCheckedDirs(next.choices.filter((choice) => choice.checked).map((choice) => choice.campaignDir));
      }
    };
    listKnownCampaigns()
      .then((known) => settle({ kind: 'listed', ...campaignChoices(known, props.memberships) }))
      .catch((cause: unknown) => settle({ kind: 'failed', message: cause instanceof Error ? cause.message : String(cause) }));
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') props.onClose();
    };
    window.addEventListener('keydown', onKeyDown);
    return () => {
      cancelled = true;
      window.removeEventListener('keydown', onKeyDown);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  async function handleSave() {
    if (listing.kind !== 'listed') return;
    const memberships = applyCampaignChoices(props.memberships, listing.choices, checkedDirs);
    setSaving(true);
    setSaveError(null);
    let warnings: string[];
    try {
      warnings = await setCharacterCampaigns({ characterId: props.characterId, memberships });
    } catch (cause: unknown) {
      setSaving(false);
      setSaveError(cause instanceof Error ? cause.message : String(cause));
      return;
    }
    setSaving(false);
    props.onSaved(memberships, warnings);
  }

  return createPortal(
    <div
      role="presentation"
      onClick={props.onClose}
      style={{ alignItems: 'center', backgroundColor: 'rgba(0,0,0,0.6)', display: 'flex', inset: 0, justifyContent: 'center', position: 'fixed', zIndex: 60 }}
    >
      <div
        role="dialog"
        aria-modal="true"
        aria-label="Campaigns"
        data-testid="campaign-membership-dialog"
        onClick={(event) => event.stopPropagation()}
        style={{
          backgroundColor: 'var(--color-surface)',
          border: '1px solid var(--color-border)',
          borderRadius: 12,
          display: 'flex',
          flexDirection: 'column',
          gap: '0.7rem',
          maxHeight: '80vh',
          padding: '1.1rem 1.4rem',
          width: 'min(520px, 95vw)',
        }}
      >
        <h2 style={{ fontSize: '1.1rem', margin: 0 }}>Campaigns</h2>
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: 0 }}>
          A checked campaign gets a copy of this character in its folder, updated on every save. Unchecking keeps the copy and stops updating it.
        </p>
        <div style={{ flex: 1, overflowY: 'auto' }}>
          {listing.kind === 'loading' ? <p style={{ margin: 0 }}>Reading your campaign folders…</p> : null}
          {listing.kind === 'failed' ? <p role="alert" style={{ color: 'var(--color-error)', margin: 0 }}>{listing.message}</p> : null}
          {listing.kind === 'listed' ? (
            <>
              {listing.choices.length === 0 ? (
                <p data-testid="campaign-membership-none" style={{ margin: 0 }}>No campaign folders yet. Create or open one in the Campaign Manager.</p>
              ) : null}
              {listing.choices.map((choice) => (
                <label key={choice.campaignDir} style={{ alignItems: 'center', display: 'flex', gap: '0.5rem', padding: '0.3rem 0' }}>
                  <input
                    type="checkbox"
                    data-testid={`campaign-membership-check-${choice.campaignName}`}
                    checked={checkedDirs.includes(choice.campaignDir)}
                    onChange={(event) => {
                      const checked = event.currentTarget.checked;
                      setCheckedDirs((dirs) => (checked ? [...dirs, choice.campaignDir] : dirs.filter((dir) => dir !== choice.campaignDir)));
                    }}
                  />
                  <span style={{ fontWeight: 600 }}>{choice.campaignName}</span>
                </label>
              ))}
              {listing.unreadable > 0 ? (
                <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.4rem 0 0' }}>
                  {listing.unreadable} known campaign folder{listing.unreadable === 1 ? ' does' : 's do'} not open as a campaign and {listing.unreadable === 1 ? 'is' : 'are'} not listed (see the Campaign Manager).
                </p>
              ) : null}
            </>
          ) : null}
        </div>
        {saveError ? <p role="alert" style={{ color: 'var(--color-error)', fontSize: '0.8rem', margin: 0 }}>{saveError}</p> : null}
        <div style={{ display: 'flex', gap: '0.6rem', justifyContent: 'flex-end' }}>
          <button type="button" onClick={props.onClose} style={buttonStyle}>Cancel</button>
          <button
            type="button"
            data-testid="campaign-membership-save"
            disabled={saving || listing.kind !== 'listed'}
            onClick={() => void handleSave()}
            style={{ ...buttonStyle, backgroundColor: 'var(--color-accent)', color: 'var(--color-on-accent)' }}
          >
            {saving ? 'Saving…' : 'Save'}
          </button>
        </div>
      </div>
    </div>,
    document.body
  );
}

const buttonStyle = {
  background: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 6,
  color: 'var(--color-text)',
  cursor: 'pointer',
  fontSize: '0.8rem',
  fontWeight: 600,
  padding: '0.3rem 0.8rem',
} as const;
