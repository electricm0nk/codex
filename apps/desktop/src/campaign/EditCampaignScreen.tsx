import { useEffect, useState, type CSSProperties } from 'react';
import { RULE_SETS } from '../characterHub/LandingScreen';
import { getCampaign, syncCampaignDriveArtifacts, updateCampaign, type Campaign } from './campaignModel';
import { loadCharacterHubListSurfaceRuntime } from '../characterHub/characterHubRuntime';
import type { CharacterHubListRowSurface } from '../characterHub/buildCharacterHubListSurface';
import { formatHeldClasses } from '../characterHub/characterProgression';
import { getFriends } from '../settings/friends';

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

/**
 * Edit Campaign: campaign details (name / rule set / description / members)
 * plus party building — selecting from saved characters to add or remove
 * them from the campaign's party.
 *
 * Real design: characters would sync from the campaign's Google Drive
 * folder as validated codex-format JSON files. That sync and validation
 * don't exist yet, so this lists the app's own saved characters (the same
 * source Load Character uses) as the stand-in pool to build a party from.
 */
export function EditCampaignScreen(props: { campaignId: string; onCancel: () => void; onSaved: () => void }) {
  const [campaign, setCampaign] = useState<Campaign | null>(() => getCampaign(props.campaignId));
  const [name, setName] = useState(campaign?.name ?? '');
  const [ruleSetId, setRuleSetId] = useState(campaign?.ruleSetId ?? RULE_SETS[0].id);
  const [description, setDescription] = useState(campaign?.description ?? '');
  const [memberEmails, setMemberEmails] = useState<string[]>(campaign?.members.map((member) => member.email) ?? ['']);
  const [partyIds, setPartyIds] = useState<string[]>(campaign?.partyCharacterIds ?? []);
  const [availableCharacters, setAvailableCharacters] = useState<CharacterHubListRowSurface[]>([]);
  const [status, setStatus] = useState<string | null>(null);
  const friends = getFriends();

  useEffect(() => {
    loadCharacterHubListSurfaceRuntime()
      .then((surface) => setAvailableCharacters(surface.rows))
      .catch(() => setAvailableCharacters([]));
  }, []);

  if (!campaign) {
    return (
      <section style={{ marginTop: '2rem' }}>
        <p style={{ color: 'var(--color-error)' }}>Campaign not found.</p>
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

  function updateMember(index: number, value: string) {
    setMemberEmails((prev) => prev.map((email, i) => (i === index ? value : email)));
  }

  function addMemberRow() {
    setMemberEmails((prev) => [...prev, '']);
  }

  function removeMemberRow(index: number) {
    setMemberEmails((prev) => (prev.length > 1 ? prev.filter((_, i) => i !== index) : prev));
  }

  function addFriendAsMember(gmailAddress: string) {
    setMemberEmails((prev) => {
      if (prev.includes(gmailAddress)) {
        return prev;
      }
      const firstEmptyIndex = prev.findIndex((email) => email.trim() === '');
      if (firstEmptyIndex !== -1) {
        return prev.map((email, i) => (i === firstEmptyIndex ? gmailAddress : email));
      }
      return [...prev, gmailAddress];
    });
  }

  function toggleParty(characterId: string) {
    setPartyIds((prev) => (prev.includes(characterId) ? prev.filter((id) => id !== characterId) : [...prev, characterId]));
  }

  async function handleSave() {
    if (!campaign) {
      return;
    }
    const selectedRuleSet = RULE_SETS.find((ruleSet) => ruleSet.id === ruleSetId) ?? RULE_SETS[0];
    const cleanedEmails = memberEmails.map((email) => email.trim()).filter(Boolean);
    const updated = updateCampaign(campaign.id, {
      name: name.trim(),
      ruleSetId: selectedRuleSet.id,
      ruleSetLabel: selectedRuleSet.name,
      description: description.trim(),
      members: cleanedEmails.map((email) => ({
        email,
        invited: campaign.members.find((member) => member.email === email)?.invited ?? true,
      })),
      partyCharacterIds: partyIds,
    });
    if (updated) {
      setCampaign(updated);
    }
    setStatus('Saving…');
    const result = await syncCampaignDriveArtifacts(campaign.id);
    setStatus(
      result.ok
        ? `Saved. Wrote .config/${name.trim()}.json to ${result.campaignFolderPath}.`
        : `Saved, but the Drive folder could not be written: ${result.error}`
    );
    props.onSaved();
  }

  return (
    <section style={{ marginTop: '2rem' }}>
      <div style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between', marginBottom: '1rem' }}>
        <h2 style={{ margin: 0 }}>Edit campaign</h2>
        <button
          type="button"
          onClick={props.onCancel}
          style={{ background: 'none', border: '1px solid var(--color-border)', borderRadius: 8, cursor: 'pointer', padding: '0.5rem 1rem' }}
        >
          Back
        </button>
      </div>

      <div style={{ display: 'flex', gap: '1.5rem' }}>
        {/* Left: campaign details */}
        <form
          onSubmit={(event) => {
            event.preventDefault();
            handleSave();
          }}
          style={{ border: '1px solid var(--color-border)', borderRadius: 12, flex: '1 1 380px', padding: '1.25rem' }}
        >
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

            {friends.length ? (
              <div style={{ marginBottom: '0.75rem' }}>
                <p style={{ color: 'var(--color-text-muted)', fontSize: '0.78rem', margin: '0 0 0.4rem' }}>
                  Add from your friends list:
                </p>
                <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.4rem' }}>
                  {friends.map((friend) => {
                    const alreadyAdded = memberEmails.includes(friend.gmailAddress);
                    return (
                      <button
                        key={friend.id}
                        type="button"
                        onClick={() => addFriendAsMember(friend.gmailAddress)}
                        disabled={alreadyAdded}
                        title={friend.gmailAddress}
                        style={{
                          backgroundColor: alreadyAdded ? 'var(--color-surface-2)' : 'transparent',
                          border: '1px solid var(--color-border)',
                          borderRadius: 999,
                          color: alreadyAdded ? 'var(--color-text-muted)' : 'var(--color-text)',
                          cursor: alreadyAdded ? 'not-allowed' : 'pointer',
                          fontSize: '0.8rem',
                          opacity: alreadyAdded ? 0.6 : 1,
                          padding: '0.3rem 0.7rem',
                        }}
                      >
                        {alreadyAdded ? '✓ ' : '+ '}
                        {friend.name}
                      </button>
                    );
                  })}
                </div>
              </div>
            ) : null}

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
            style={{
              backgroundColor: 'var(--color-accent)',
              border: 'none',
              borderRadius: 8,
              color: 'var(--color-on-accent)',
              cursor: 'pointer',
              padding: '0.6rem 1.25rem',
            }}
          >
            Save
          </button>

          {status ? <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', lineHeight: 1.6, marginTop: '0.9rem' }}>{status}</p> : null}
        </form>

        {/* Right: party builder */}
        <div style={{ border: '1px solid var(--color-border)', borderRadius: 12, flex: '1 1 320px', padding: '1.25rem' }}>
          <p style={LABEL_STYLE}>Party ({partyIds.length})</p>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.78rem', lineHeight: 1.5, margin: '0 0 1rem' }}>
            Characters from your saved list — Drive-synced character sync and format validation aren't wired yet, so
            this uses the same characters Load Character does.
          </p>

          {availableCharacters.length === 0 ? (
            <p style={{ color: 'var(--color-text-secondary)' }}>No saved characters yet.</p>
          ) : (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
              {availableCharacters.map((row) => {
                const inParty = partyIds.includes(row.characterId);
                return (
                  <div
                    key={row.characterId}
                    style={{
                      alignItems: 'center',
                      backgroundColor: inParty ? 'var(--color-surface-2)' : 'transparent',
                      border: `1px solid ${inParty ? 'var(--color-accent)' : 'var(--color-border)'}`,
                      borderRadius: 8,
                      display: 'flex',
                      gap: '0.75rem',
                      justifyContent: 'space-between',
                      padding: '0.6rem 0.8rem',
                    }}
                  >
                    <div style={{ minWidth: 0 }}>
                      <p style={{ fontWeight: 700, margin: 0 }}>{row.displayLabel}</p>
                      <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.78rem', margin: '0.15rem 0 0' }}>
                        {row.raceLabel} {formatHeldClasses(row.classSummary)}
                      </p>
                    </div>
                    <button
                      type="button"
                      onClick={() => toggleParty(row.characterId)}
                      style={{
                        backgroundColor: inParty ? 'var(--color-surface)' : 'var(--color-accent)',
                        border: '1px solid var(--color-border)',
                        borderRadius: 6,
                        color: inParty ? 'var(--color-text)' : 'var(--color-on-accent)',
                        cursor: 'pointer',
                        flexShrink: 0,
                        fontSize: '0.8rem',
                        fontWeight: 600,
                        padding: '0.3rem 0.7rem',
                      }}
                    >
                      {inParty ? 'Remove' : 'Add'}
                    </button>
                  </div>
                );
              })}
            </div>
          )}
        </div>
      </div>
    </section>
  );
}
