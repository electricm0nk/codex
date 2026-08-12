import { useState, type CSSProperties } from 'react';
import { getCampaigns, type Campaign } from './campaignModel';

/**
 * Campaign Manager: a selectable campaign list on the left, a brief-details
 * pane on the right, and a bottom action bar (Load / Create / Edit / Cancel)
 * — patterned after the Load Character screen.
 */

const barButton = (variant: 'primary' | 'default', enabled: boolean): CSSProperties => ({
  backgroundColor: variant === 'primary' && enabled ? 'var(--color-accent)' : 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 8,
  color: variant === 'primary' && enabled ? 'var(--color-on-accent)' : 'var(--color-text)',
  cursor: enabled ? 'pointer' : 'not-allowed',
  fontSize: '0.9rem',
  fontWeight: variant === 'primary' ? 700 : 500,
  opacity: enabled ? 1 : 0.5,
  padding: '0.55rem 1.4rem',
});

function formatDate(iso: string): string {
  const parsed = new Date(iso);
  return Number.isNaN(parsed.getTime()) ? iso : parsed.toLocaleDateString(undefined, { year: 'numeric', month: 'short', day: 'numeric' });
}

function DetailPane(props: { campaign: Campaign | null }) {
  const { campaign } = props;
  if (!campaign) {
    return <p style={{ color: 'var(--color-text-muted)', margin: 0 }}>Select a campaign to preview it, then press Load or Edit.</p>;
  }
  return (
    <div>
      <div style={{ alignItems: 'baseline', display: 'flex', gap: '1rem', justifyContent: 'space-between' }}>
        <h2 style={{ margin: 0 }}>{campaign.name}</h2>
        <span style={{ color: 'var(--color-text-secondary)', fontWeight: 700 }}>{campaign.ruleSetLabel}</span>
      </div>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: '0.6rem 0 0.9rem' }}>
        Created {formatDate(campaign.createdAt)} · Updated {formatDate(campaign.updatedAt)}
      </p>

      <hr style={{ border: 'none', borderTop: '1px solid var(--color-border)', margin: '0.75rem 0' }} />

      <p style={{ color: 'var(--color-text-secondary)', lineHeight: 1.6, margin: '0 0 1rem' }}>
        {campaign.description || <span style={{ color: 'var(--color-text-faint)' }}>No description yet.</span>}
      </p>

      <div style={{ display: 'grid', gap: '1rem', gridTemplateColumns: 'repeat(2, 1fr)' }}>
        <div>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.7rem', letterSpacing: '0.04em', margin: 0, textTransform: 'uppercase' }}>
            Party
          </p>
          <p style={{ fontWeight: 700, margin: '0.2rem 0 0' }}>
            {campaign.partyCharacterIds.length} character{campaign.partyCharacterIds.length === 1 ? '' : 's'}
          </p>
        </div>
        <div>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.7rem', letterSpacing: '0.04em', margin: 0, textTransform: 'uppercase' }}>
            Members
          </p>
          <p style={{ fontWeight: 700, margin: '0.2rem 0 0' }}>{campaign.members.length}</p>
        </div>
      </div>

      {campaign.members.length ? (
        <div style={{ marginTop: '1rem' }}>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.7rem', letterSpacing: '0.04em', margin: '0 0 0.4rem', textTransform: 'uppercase' }}>
            Invited
          </p>
          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.4rem' }}>
            {campaign.members.map((member) => (
              <span
                key={member.email}
                style={{
                  backgroundColor: 'var(--color-surface-2)',
                  border: '1px solid var(--color-border)',
                  borderRadius: 6,
                  color: 'var(--color-text-secondary)',
                  fontSize: '0.78rem',
                  padding: '0.15rem 0.5rem',
                }}
              >
                {member.email}
              </span>
            ))}
          </div>
        </div>
      ) : null}
    </div>
  );
}

export function CampaignManagerScreen(props: {
  onCancel: () => void;
  onCreate: () => void;
  onEdit: (campaignId: string) => void;
  onLoad: (campaignId: string) => void;
}) {
  const [campaigns] = useState<Campaign[]>(() => getCampaigns());
  const [selectedId, setSelectedId] = useState<string | null>(null);

  const selected = campaigns.find((campaign) => campaign.id === selectedId) ?? null;

  return (
    <section style={{ display: 'flex', flexDirection: 'column', height: 'calc(100vh - 8rem)', marginTop: '1rem' }}>
      <h2 style={{ margin: '0 0 1rem' }}>Campaign Manager</h2>

      <div style={{ display: 'flex', flex: 1, gap: '1rem', minHeight: 0 }}>
        {/* Campaign list */}
        <div
          style={{
            border: '1px solid var(--color-border)',
            borderRadius: 12,
            display: 'flex',
            flexDirection: 'column',
            minWidth: 280,
            overflowY: 'auto',
            width: 320,
          }}
        >
          {campaigns.length === 0 ? (
            <p style={{ color: 'var(--color-text-secondary)', padding: '1rem' }}>
              No campaigns yet — press Create to start one.
            </p>
          ) : null}
          {campaigns.map((campaign) => {
            const isSelected = campaign.id === selectedId;
            return (
              <button
                key={campaign.id}
                type="button"
                onClick={() => setSelectedId(campaign.id)}
                style={{
                  background: isSelected ? 'var(--color-surface-2)' : 'none',
                  border: 'none',
                  borderBottom: '1px solid var(--color-border)',
                  cursor: 'pointer',
                  padding: '0.75rem 1rem',
                  textAlign: 'left',
                }}
              >
                <p style={{ color: isSelected ? 'var(--color-accent)' : 'var(--color-text)', fontWeight: 700, margin: 0 }}>
                  {campaign.name}
                </p>
                <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.82rem', margin: '0.2rem 0 0' }}>
                  {campaign.ruleSetLabel} · {campaign.partyCharacterIds.length} in party
                </p>
                <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.15rem 0 0' }}>
                  Updated {formatDate(campaign.updatedAt)}
                </p>
              </button>
            );
          })}
        </div>

        {/* Detail pane */}
        <div style={{ border: '1px solid var(--color-border)', borderRadius: 12, flex: 1, overflowY: 'auto', padding: '1.25rem' }}>
          <DetailPane campaign={selected} />
        </div>
      </div>

      {/* Action bar */}
      <div style={{ borderTop: '1px solid var(--color-border)', display: 'flex', gap: '0.6rem', marginTop: '1rem', paddingTop: '1rem' }}>
        <button type="button" onClick={() => selected && props.onLoad(selected.id)} disabled={!selected} style={barButton('primary', Boolean(selected))}>
          Load
        </button>
        <button type="button" onClick={props.onCreate} style={barButton('default', true)}>
          Create
        </button>
        <button type="button" onClick={() => selected && props.onEdit(selected.id)} disabled={!selected} style={barButton('default', Boolean(selected))}>
          Edit
        </button>
        <div style={{ flex: 1 }} />
        <button type="button" onClick={props.onCancel} style={barButton('default', true)}>
          Cancel
        </button>
      </div>
    </section>
  );
}
