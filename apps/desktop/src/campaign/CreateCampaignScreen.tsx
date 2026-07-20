import { useState, type CSSProperties, type FormEvent } from 'react';
import { RULE_SETS } from '../characterHub/LandingScreen';
import { createCampaign, syncCampaignDriveArtifacts } from './campaignModel';

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

export function CreateCampaignScreen(props: { onCancel: () => void; onCreated: (campaignId: string) => void }) {
  const [name, setName] = useState('');
  const [ruleSetId, setRuleSetId] = useState(RULE_SETS[0].id);
  const [description, setDescription] = useState('');
  const [memberEmails, setMemberEmails] = useState<string[]>(['']);
  const [submitting, setSubmitting] = useState(false);
  const [driveActionSummary, setDriveActionSummary] = useState<string | null>(null);

  const selectedRuleSet = RULE_SETS.find((ruleSet) => ruleSet.id === ruleSetId) ?? RULE_SETS[0];

  function updateMember(index: number, value: string) {
    setMemberEmails((prev) => prev.map((email, i) => (i === index ? value : email)));
  }

  function addMemberRow() {
    setMemberEmails((prev) => [...prev, '']);
  }

  function removeMemberRow(index: number) {
    setMemberEmails((prev) => (prev.length > 1 ? prev.filter((_, i) => i !== index) : prev));
  }

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    setSubmitting(true);
    try {
      const cleanedEmails = memberEmails.map((email) => email.trim()).filter(Boolean);
      const { campaign } = createCampaign({
        name: name.trim(),
        ruleSetId: selectedRuleSet.id,
        ruleSetLabel: selectedRuleSet.name,
        description: description.trim(),
        memberEmails: cleanedEmails,
      });

      const result = await syncCampaignDriveArtifacts(campaign.id);
      setDriveActionSummary(
        result.ok
          ? `Campaign folder created at ${result.campaignFolderPath}.`
          : `Campaign saved, but the Drive folder could not be written: ${result.error}`
      );
      props.onCreated(campaign.id);
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
          <p style={LABEL_STYLE}>Campaign members</p>
          <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
            {memberEmails.map((email, index) => (
              <div key={index} style={{ display: 'flex', gap: '0.5rem' }}>
                <input
                  type="email"
                  placeholder="player@example.com"
                  value={email}
                  onChange={(event) => updateMember(index, event.target.value)}
                  style={INPUT_STYLE}
                />
                <button
                  type="button"
                  onClick={() => removeMemberRow(index)}
                  disabled={memberEmails.length === 1}
                  aria-label="Remove member"
                  style={{
                    background: 'none',
                    border: '1px solid var(--color-border)',
                    borderRadius: 8,
                    color: 'var(--color-text-muted)',
                    cursor: memberEmails.length === 1 ? 'not-allowed' : 'pointer',
                    flexShrink: 0,
                    opacity: memberEmails.length === 1 ? 0.5 : 1,
                    padding: '0.3rem 0.6rem',
                  }}
                >
                  ×
                </button>
              </div>
            ))}
          </div>
          <button
            type="button"
            onClick={addMemberRow}
            style={{
              background: 'none',
              border: '1px dashed var(--color-border)',
              borderRadius: 8,
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
          disabled={submitting || !name.trim()}
          style={{
            backgroundColor: 'var(--color-accent)',
            border: 'none',
            borderRadius: 8,
            color: 'var(--color-on-accent)',
            cursor: submitting || !name.trim() ? 'not-allowed' : 'pointer',
            marginTop: '0.5rem',
            opacity: submitting || !name.trim() ? 0.6 : 1,
            padding: '0.6rem 1.25rem',
          }}
        >
          {submitting ? 'Creating…' : 'Create campaign'}
        </button>

        {driveActionSummary ? (
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', lineHeight: 1.6, marginTop: '0.9rem' }}>{driveActionSummary}</p>
        ) : null}
      </form>
    </section>
  );
}
