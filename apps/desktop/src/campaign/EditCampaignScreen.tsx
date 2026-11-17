import { useEffect, useState, type CSSProperties } from 'react';
import { RULE_SETS } from '../characterHub/LandingScreen';
import { openCampaignFolder, saveCampaignJson } from '../boundary/campaignFolder/knownCampaigns';
import type { FolderMember, OpenedCampaign } from '../boundary/campaignFolder/types';
import { addMember, membersForSave, removeMember, renameMember } from './campaignMembersModel';
import { CampaignHelpButton } from './CampaignHelpScreen';

const LABEL_STYLE: CSSProperties = {
  color: 'var(--color-text-secondary)',
  display: 'block',
  fontSize: '0.875rem',
  fontWeight: 600,
  marginBottom: '0.35rem',
};
const INPUT_STYLE: CSSProperties = {
  backgroundColor: 'var(--color-surface-2)',
  border: '1px solid var(--color-border)',
  borderRadius: 8,
  boxSizing: 'border-box',
  color: 'var(--color-text)',
  padding: '0.5rem 0.65rem',
  width: '100%',
};
const FIELD_STYLE: CSSProperties = { marginBottom: '1rem' };

const SMALL_BUTTON: CSSProperties = {
  background: 'none',
  border: '1px solid var(--color-border)',
  borderRadius: 8,
  color: 'var(--color-text-muted)',
  flexShrink: 0,
  padding: '0.3rem 0.6rem',
};

/**
 * Edit Campaign over its folder (SD-42 `decisions.md §13.1`): name, rule set, description and the
 * typed member list (add, rename, remove any but the first — the creator). Saving writes
 * `campaign.json` through the conflict-detecting nonce path. The party is the folder's
 * `Characters/` (E2.10), not a list kept here.
 */
export function EditCampaignScreen(props: { campaignId: string; onCancel: () => void; onSaved: () => void }) {
  // The hub's campaign id is the campaign folder's path (SD-42 E2.2).
  const campaignDir = props.campaignId;
  const [opened, setOpened] = useState<OpenedCampaign | null>(null);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [name, setName] = useState('');
  const [ruleSetId, setRuleSetId] = useState<string>(RULE_SETS[0].id);
  const [description, setDescription] = useState('');
  const [members, setMembers] = useState<FolderMember[]>([]);
  const [saving, setSaving] = useState(false);
  const [status, setStatus] = useState<string | null>(null);

  useEffect(() => {
    openCampaignFolder(campaignDir)
      .then((result) => {
        setOpened(result);
        setName(result.campaign.name);
        setRuleSetId(result.campaign.ruleSetId);
        setDescription(result.campaign.description);
        setMembers(result.campaign.members);
      })
      .catch((cause: unknown) => setLoadError(cause instanceof Error ? cause.message : String(cause)));
  }, [campaignDir]);

  if (!opened) {
    return (
      <section style={{ marginTop: '2rem' }}>
        <p style={{ color: loadError ? 'var(--color-error)' : 'var(--color-text-muted)' }}>{loadError ?? 'Opening the campaign folder…'}</p>
        <button
          type="button"
          onClick={props.onCancel}
          style={{ background: 'none', border: '1px solid var(--color-border)', borderRadius: 8, cursor: 'pointer', padding: '0.5rem 1rem' }}
        >
          Back
        </button>
      </section>
    );
  }

  async function handleSave() {
    if (!opened) {
      return;
    }
    const now = new Date().toISOString();
    const selectedRuleSet = RULE_SETS.find((ruleSet) => ruleSet.id === ruleSetId) ?? RULE_SETS[0];
    setSaving(true);
    try {
      const campaign = {
        ...opened.campaign,
        name: name.trim(),
        ruleSetId: selectedRuleSet.id,
        ruleSetLabel: selectedRuleSet.name,
        description: description.trim(),
        members: membersForSave(members, now),
        updatedAt: now,
      };
      const saved = await saveCampaignJson(opened.campaignDir, campaign, opened.nonce);
      setOpened({ ...opened, campaign, nonce: saved.nonce });
      setMembers(campaign.members);
      if (saved.conflictDir) {
        setStatus(
          `Saved. Someone else changed this campaign since you opened it; their version of campaign.json is kept in ${saved.conflictDir}.`
        );
        return;
      }
      props.onSaved();
    } catch (cause: unknown) {
      setStatus(cause instanceof Error ? cause.message : String(cause));
    } finally {
      setSaving(false);
    }
  }

  return (
    <section style={{ marginTop: '2rem' }}>
      <div style={{ alignItems: 'center', display: 'flex', gap: '0.6rem', justifyContent: 'space-between', marginBottom: '1rem' }}>
        <h2 style={{ margin: 0 }}>Edit campaign</h2>
        <div style={{ flex: 1 }} />
        <CampaignHelpButton campaignDir={campaignDir} />
        <button
          type="button"
          onClick={props.onCancel}
          style={{ background: 'none', border: '1px solid var(--color-border)', borderRadius: 8, cursor: 'pointer', padding: '0.5rem 1rem' }}
        >
          Back
        </button>
      </div>

      <form
        onSubmit={(event) => {
          event.preventDefault();
          handleSave();
        }}
        style={{ border: '1px solid var(--color-border)', borderRadius: 12, maxWidth: 560, padding: '1.25rem' }}
      >
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0 0 1rem', overflowWrap: 'anywhere' }}>{opened.campaignDir}</p>

        <div style={FIELD_STYLE}>
          <label style={LABEL_STYLE} htmlFor="campaign-name">
            Campaign name
          </label>
          <input id="campaign-name" style={INPUT_STYLE} value={name} onChange={(event) => setName(event.target.value)} required />
        </div>

          <div style={FIELD_STYLE}>
            <label style={LABEL_STYLE} htmlFor="campaign-rule-set">
              Campaign rule set
            </label>
            <select id="campaign-rule-set" style={INPUT_STYLE} value={ruleSetId} onChange={(event) => setRuleSetId(event.target.value as typeof ruleSetId)}>
              {RULE_SETS.map((ruleSet) => (
                <option key={ruleSet.id} value={ruleSet.id}>
                  {ruleSet.name}
                </option>
              ))}
            </select>
          </div>

          <div style={FIELD_STYLE}>
            <label style={LABEL_STYLE} htmlFor="campaign-description">
              Campaign description
            </label>
            <textarea
              id="campaign-description"
              rows={4}
              style={{ ...INPUT_STYLE, fontFamily: 'inherit', resize: 'vertical' }}
              value={description}
              onChange={(event) => setDescription(event.target.value)}
            />
          </div>

          <div style={FIELD_STYLE}>
            <p style={LABEL_STYLE}>Campaign members ({members.length})</p>
            <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
              {members.map((member, index) => (
                <div key={index} style={{ display: 'flex', gap: '0.5rem' }}>
                  <input
                    aria-label={index === 0 ? 'Creator name' : `Member ${index + 1} name`}
                    value={member.name}
                    onChange={(event) => setMembers((prev) => renameMember(prev, index, event.target.value))}
                    required={index === 0}
                    style={INPUT_STYLE}
                  />
                  <button
                    type="button"
                    onClick={() => setMembers((prev) => removeMember(prev, index))}
                    disabled={index === 0}
                    aria-label="Remove member"
                    title={index === 0 ? 'The campaign creator stays member #1' : undefined}
                    style={{ ...SMALL_BUTTON, cursor: index === 0 ? 'not-allowed' : 'pointer', opacity: index === 0 ? 0.5 : 1 }}
                  >
                    ×
                  </button>
                </div>
              ))}
            </div>
            <button
              type="button"
              onClick={() => setMembers((prev) => addMember(prev, new Date().toISOString()))}
              style={{
                ...SMALL_BUTTON,
                border: '1px dashed var(--color-border)',
                color: 'var(--color-text-secondary)',
                cursor: 'pointer',
                fontSize: '0.85rem',
                marginTop: '0.5rem',
                padding: '0.4rem 0.75rem',
              }}
            >
              + Add member
            </button>
          </div>

          <button
            type="submit"
            disabled={saving}
            style={{
              backgroundColor: 'var(--color-accent)',
              border: 'none',
              borderRadius: 8,
              color: 'var(--color-on-accent)',
              cursor: saving ? 'not-allowed' : 'pointer',
              opacity: saving ? 0.6 : 1,
              padding: '0.6rem 1.25rem',
            }}
          >
            {saving ? 'Saving…' : 'Save'}
          </button>

          {status ? <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', lineHeight: 1.6, marginTop: '0.9rem' }}>{status}</p> : null}
      </form>
    </section>
  );
}
