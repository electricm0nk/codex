import { useState, type CSSProperties, type FormEvent } from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import { RULE_SETS } from '../characterHub/LandingScreen';
import { hasTauriRuntime } from '../boundary/runtime';
import { createCampaignFolder } from '../boundary/campaignFolder/knownCampaigns';
import { membersForCreate, readYourName, rememberYourName } from './campaignMembersModel';

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
  color: 'var(--color-text-secondary)',
  cursor: 'pointer',
  flexShrink: 0,
  padding: '0.3rem 0.6rem',
};

/**
 * Create a campaign folder (SD-42 `decisions.md §11`, §13.1): "Your name" is member #1 and
 * required; further members are typed names. The campaign is created as `<chosen folder>/<name>/`.
 */
export function CreateCampaignScreen(props: { onCancel: () => void; onCreated: (campaignDir: string) => void }) {
  const [name, setName] = useState('');
  const [ruleSetId, setRuleSetId] = useState(RULE_SETS[0].id);
  const [description, setDescription] = useState('');
  const [yourName, setYourName] = useState(() => readYourName());
  const [otherNames, setOtherNames] = useState<string[]>([]);
  const [parentDir, setParentDir] = useState('');
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const selectedRuleSet = RULE_SETS.find((ruleSet) => ruleSet.id === ruleSetId) ?? RULE_SETS[0];
  const ready = Boolean(name.trim() && yourName.trim() && parentDir);

  async function chooseParentDir() {
    if (!hasTauriRuntime()) {
      setError('Choosing a folder needs the desktop app.');
      return;
    }
    const picked = await open({ directory: true, multiple: false, title: 'Create the campaign folder in…' });
    if (typeof picked === 'string') {
      setParentDir(picked);
    }
  }

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    setSubmitting(true);
    setError(null);
    try {
      const now = new Date().toISOString();
      const members = membersForCreate(yourName, otherNames, now);
      const opened = await createCampaignFolder(parentDir, {
        schemaVersion: 2,
        name: name.trim(),
        ruleSetId: selectedRuleSet.id,
        ruleSetLabel: selectedRuleSet.name,
        description: description.trim(),
        members,
        createdAt: now,
        updatedAt: now,
      });
      rememberYourName(yourName);
      props.onCreated(opened.campaignDir);
    } catch (cause: unknown) {
      setError(cause instanceof Error ? cause.message : String(cause));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <section style={{ marginTop: '2rem' }}>
      <div style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between', marginBottom: '1rem' }}>
        <h2 style={{ margin: 0 }}>Create a campaign</h2>
        <button
          type="button"
          onClick={props.onCancel}
          style={{ background: 'none', border: '1px solid var(--color-border)', borderRadius: 8, cursor: 'pointer', padding: '0.5rem 1rem' }}
        >
          Back
        </button>
      </div>

      <form onSubmit={handleSubmit} style={{ border: '1px solid var(--color-border)', borderRadius: 12, maxWidth: 560, padding: '1.25rem' }}>
        <div style={FIELD_STYLE}>
          <label style={LABEL_STYLE} htmlFor="campaign-your-name">
            Your name
          </label>
          <input
            id="campaign-your-name"
            style={INPUT_STYLE}
            value={yourName}
            onChange={(event) => setYourName(event.target.value)}
            required
          />
        </div>

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
          <p style={LABEL_STYLE}>Other members</p>
          <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
            {otherNames.map((memberName, index) => (
              <div key={index} style={{ display: 'flex', gap: '0.5rem' }}>
                <input
                  placeholder="Player name"
                  value={memberName}
                  onChange={(event) => setOtherNames((prev) => prev.map((n, i) => (i === index ? event.target.value : n)))}
                  style={INPUT_STYLE}
                />
                <button
                  type="button"
                  onClick={() => setOtherNames((prev) => prev.filter((_, i) => i !== index))}
                  aria-label="Remove member"
                  style={SMALL_BUTTON}
                >
                  ×
                </button>
              </div>
            ))}
          </div>
          <button
            type="button"
            onClick={() => setOtherNames((prev) => [...prev, ''])}
            style={{ ...SMALL_BUTTON, border: '1px dashed var(--color-border)', fontSize: '0.85rem', marginTop: '0.5rem', padding: '0.4rem 0.75rem' }}
          >
            + Add member
          </button>
        </div>

        <div style={FIELD_STYLE}>
          <p style={LABEL_STYLE}>Create the campaign folder in</p>
          <div style={{ alignItems: 'center', display: 'flex', gap: '0.5rem' }}>
            <span style={{ ...INPUT_STYLE, color: parentDir ? 'var(--color-text)' : 'var(--color-text-faint)', overflowWrap: 'anywhere' }}>
              {parentDir ? `${parentDir}/${name.trim() || '…'}` : 'No folder chosen'}
            </span>
            <button type="button" onClick={chooseParentDir} style={SMALL_BUTTON}>
              Choose folder…
            </button>
          </div>
        </div>

        <button
          type="submit"
          disabled={submitting || !ready}
          style={{
            backgroundColor: 'var(--color-accent)',
            border: 'none',
            borderRadius: 8,
            color: 'var(--color-on-accent)',
            cursor: submitting || !ready ? 'not-allowed' : 'pointer',
            marginTop: '0.5rem',
            opacity: submitting || !ready ? 0.6 : 1,
            padding: '0.6rem 1.25rem',
          }}
        >
          {submitting ? 'Creating…' : 'Create campaign'}
        </button>

        {error ? <p style={{ color: 'var(--color-error)', fontSize: '0.8rem', lineHeight: 1.6, marginTop: '0.9rem' }}>{error}</p> : null}
      </form>
    </section>
  );
}
