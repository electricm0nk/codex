import { useEffect, useMemo, useState, type CSSProperties } from 'react';
import { loadMonsterCatalog, type MonsterCatalogEntryDto } from '../boundary/loadMonsterCatalog';
import { loadListSavedCharacters, type CharacterSummaryDto } from '../boundary/loadListSavedCharacters';
import { rateEncounter, type RateEncounterResponse } from '../boundary/rateEncounter';
import { hasTauriRuntime } from '../boundary/runtime';
import { addMonster, addSavedMember, addTypedMember, emptyBuilder, removeMember, removeMonster, setMonsterCount, toRequest, type EncounterBuilder } from './encounterBuilderModel';
import { buildEncounterView, describeTier, formatCr } from './encounterViewModel';

/**
 * Encounter builder (v0.8 E-2): build a party from saved characters or
 * typed levels, add monsters from the real catalog, and read the engine's
 * rating as it changes. Every number on this screen is a field of the
 * `rate_encounter` response. The two disclosures — an unverified rating
 * when any CR is outside the engine's verified table, and the four-tier
 * scale collapsed from the rulebook's five — are the screen's main job,
 * not decoration: a "Medium" the DM takes to the table must be one the
 * engine actually stands behind.
 */

const mono: CSSProperties = { fontFamily: 'ui-monospace, monospace', fontWeight: 800, letterSpacing: '0.12em', textTransform: 'uppercase' };
const pane: CSSProperties = { border: '1px solid var(--color-border)', borderRadius: 12, display: 'flex', flexDirection: 'column', minHeight: 0 };
const heading: CSSProperties = { ...mono, borderBottom: '1px solid var(--color-border)', color: 'var(--color-text-muted)', fontSize: '0.62rem', margin: '0 0 0.75rem', paddingBottom: '0.4rem' };
const accentButton: CSSProperties = { background: 'var(--color-accent)', border: 'none', borderRadius: 8, color: 'var(--color-on-accent)', cursor: 'pointer', fontSize: '0.75rem', fontWeight: 700, padding: '0.35rem 0.7rem' };
const quietButton: CSSProperties = { background: 'none', border: '1px solid var(--color-border)', borderRadius: 8, color: 'var(--color-text)', cursor: 'pointer', fontSize: '0.75rem', fontWeight: 600, padding: '0.35rem 0.7rem' };
const inputStyle: CSSProperties = { background: 'var(--color-surface)', border: '1px solid var(--color-border)', borderRadius: 8, boxSizing: 'border-box', color: 'var(--color-text)', fontFamily: 'inherit', fontSize: '0.9rem', padding: '0.5rem 0.7rem' };

export function EncounterBuilderScreen(props: { onBack: () => void }) {
  const [builder, setBuilder] = useState<EncounterBuilder>(emptyBuilder);
  const [catalog, setCatalog] = useState<MonsterCatalogEntryDto[] | null>(null);
  const [catalogError, setCatalogError] = useState<string | null>(null);
  const [saved, setSaved] = useState<CharacterSummaryDto[]>([]);
  const [savedError, setSavedError] = useState<string | null>(null);
  const [search, setSearch] = useState('');
  const [typedLevel, setTypedLevel] = useState('1');
  const [rating, setRating] = useState<RateEncounterResponse | null>(null);
  const [ratingError, setRatingError] = useState<string | null>(null);
  const [ratingBusy, setRatingBusy] = useState(false);

  useEffect(() => {
    let live = true;
    loadMonsterCatalog()
      .then((response) => live && setCatalog(response.entries))
      .catch((cause: unknown) => live && setCatalogError(cause instanceof Error ? cause.message : String(cause)));
    loadListSavedCharacters()
      .then((response) => live && setSaved(response.characters))
      .catch((cause: unknown) => live && setSavedError(cause instanceof Error ? cause.message : String(cause)));
    return () => {
      live = false;
    };
  }, []);

  // Re-rate on every change. A stale response never overwrites a newer one.
  const request = useMemo(() => toRequest(builder), [builder]);
  useEffect(() => {
    if (request === null) {
      setRating(null);
      setRatingError(null);
      return;
    }
    if (!hasTauriRuntime()) {
      setRating(null);
      setRatingError('Rating requires the desktop runtime — the engine does the rating; nothing is computed here.');
      return;
    }
    let live = true;
    setRatingBusy(true);
    rateEncounter(request)
      .then((response) => {
        if (live) {
          setRating(response);
          setRatingError(null);
        }
      })
      .catch((cause: unknown) => {
        if (live) {
          setRating(null);
          setRatingError(cause instanceof Error ? cause.message : String(cause));
        }
      })
      .finally(() => live && setRatingBusy(false));
    return () => {
      live = false;
    };
  }, [request]);

  const byKey = useMemo(() => new Map((catalog ?? []).map((entry) => [entry.key, entry])), [catalog]);
  const savedById = useMemo(() => new Map(saved.map((entry) => [entry.characterId, entry])), [saved]);
  const needle = search.trim().toLowerCase();
  const matches = needle === '' ? [] : (catalog ?? []).filter((entry) => entry.name.toLowerCase().includes(needle)).slice(0, 40);
  const view = rating ? buildEncounterView(rating) : null;

  return (
    <section style={{ display: 'flex', flexDirection: 'column', height: 'calc(100vh - 6rem)', marginTop: '1rem' }}>
      <div style={{ alignItems: 'center', borderBottom: '1px solid var(--color-border)', display: 'flex', gap: '1rem', paddingBottom: '0.75rem' }}>
        <span style={{ ...mono, border: '2px solid var(--color-accent)', color: 'var(--color-accent)', fontSize: '1rem', padding: '0.1rem 0.5rem' }}>DM</span>
        <span style={{ fontWeight: 700 }}>Encounter builder</span>
        <span style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem' }}>strawman — party-strength rating only; no XP budget, treasure or saving yet</span>
        <button type="button" onClick={props.onBack} style={{ ...quietButton, marginLeft: 'auto' }}>
          Back to console
        </button>
      </div>

      <div style={{ display: 'grid', flex: 1, gap: '1rem', gridTemplateColumns: 'minmax(0, 1fr) minmax(0, 1fr) minmax(0, 1.2fr)', marginTop: '1rem', minHeight: 0 }}>
        {/* Party */}
        <div style={{ ...pane, overflowY: 'auto', padding: '1rem' }}>
          <h2 style={heading}>Party</h2>
          {builder.party.length === 0 ? <p style={{ color: 'var(--color-text-muted)', fontSize: '0.85rem' }}>No party yet. Add saved characters or typed levels.</p> : null}
          {builder.party.map((member, index) => {
            const rated = rating?.party[index];
            const label = member.kind === 'saved' ? savedById.get(member.characterId)?.displayLabel ?? member.characterId : `Level ${member.level} (typed)`;
            return (
              <div key={`${index}:${member.kind === 'saved' ? member.characterId : member.level}`} style={{ alignItems: 'center', borderBottom: '1px solid var(--color-border)', display: 'flex', gap: '0.5rem', padding: '0.45rem 0' }}>
                <span style={{ flex: 1, fontSize: '0.85rem' }}>
                  {label}
                  {member.kind === 'saved' ? (
                    <span style={{ color: 'var(--color-text-muted)', display: 'block', fontSize: '0.72rem' }}>
                      {rated ? `Level ${rated.level} — read from the saved build by the engine` : 'level resolved by the engine when rated'}
                    </span>
                  ) : null}
                </span>
                <button type="button" aria-label={`Remove ${label}`} onClick={() => setBuilder((b) => removeMember(b, index))} style={{ ...quietButton, padding: '0 0.5rem' }}>
                  ×
                </button>
              </div>
            );
          })}
          <div style={{ marginTop: '1rem' }}>
            <h3 style={{ ...heading, marginTop: 0 }}>Add a saved character</h3>
            {savedError ? <p style={{ color: 'var(--color-error)', fontSize: '0.8rem' }}>{savedError}</p> : null}
            {saved.length === 0 && !savedError ? <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem' }}>No saved characters.</p> : null}
            {saved.map((entry) => (
              <button key={entry.characterId} type="button" onClick={() => setBuilder((b) => addSavedMember(b, entry.characterId))} style={{ ...quietButton, display: 'block', marginBottom: '0.3rem', textAlign: 'left', width: '100%' }}>
                {entry.displayLabel}
              </button>
            ))}
          </div>
          <div style={{ marginTop: '1rem' }}>
            <h3 style={{ ...heading, marginTop: 0 }}>Add a typed level</h3>
            <div style={{ display: 'flex', gap: '0.4rem' }}>
              <input aria-label="Level" type="number" min={1} max={20} value={typedLevel} onChange={(event) => setTypedLevel(event.target.value)} style={{ ...inputStyle, width: 80 }} />
              <button type="button" onClick={() => setBuilder((b) => addTypedMember(b, Number(typedLevel)))} style={accentButton}>
                Add
              </button>
            </div>
          </div>
        </div>

        {/* Monsters */}
        <div style={{ ...pane, overflowY: 'auto', padding: '1rem' }}>
          <h2 style={heading}>Monsters</h2>
          {builder.monsters.length === 0 ? <p style={{ color: 'var(--color-text-muted)', fontSize: '0.85rem' }}>No monsters yet. Search the catalog below.</p> : null}
          {builder.monsters.map((row) => {
            const entry = byKey.get(row.catalogKey);
            const rated = rating?.monsters.find((m) => m.catalogKey === row.catalogKey);
            return (
              <div key={row.catalogKey} style={{ alignItems: 'center', borderBottom: '1px solid var(--color-border)', display: 'flex', gap: '0.5rem', padding: '0.45rem 0' }}>
                <input aria-label={`Count of ${entry?.name ?? row.catalogKey}`} type="number" min={0} value={row.count} onChange={(event) => setBuilder((b) => setMonsterCount(b, row.catalogKey, Number(event.target.value)))} style={{ ...inputStyle, width: 60 }} />
                <span style={{ flex: 1, fontSize: '0.85rem' }}>
                  {entry?.name ?? row.catalogKey}
                  <span style={{ color: rated?.outsideVerifiedRange ? 'var(--color-warn)' : 'var(--color-text-muted)', display: 'block', fontSize: '0.72rem' }}>
                    CR {entry ? formatCr(entry.challengeRating) : '?'}
                    {rated?.outsideVerifiedRange ? ` — rated as CR ${rated.crAsRated}: ${rated.reason ?? 'outside the verified table'}` : ''}
                  </span>
                </span>
                <button type="button" aria-label={`Remove ${entry?.name ?? row.catalogKey}`} onClick={() => setBuilder((b) => removeMonster(b, row.catalogKey))} style={{ ...quietButton, padding: '0 0.5rem' }}>
                  ×
                </button>
              </div>
            );
          })}
          <div style={{ marginTop: '1rem' }}>
            <input aria-label="Search monsters" placeholder={catalog ? `Search ${catalog.length} monsters…` : 'Loading catalog…'} value={search} onChange={(event) => setSearch(event.target.value)} style={{ ...inputStyle, width: '100%' }} />
            {catalogError ? <p style={{ color: 'var(--color-error)', fontSize: '0.8rem' }}>{catalogError}</p> : null}
            {matches.map((entry) => (
              <button key={entry.key} type="button" onClick={() => setBuilder((b) => addMonster(b, entry.key))} style={{ ...quietButton, display: 'flex', justifyContent: 'space-between', marginTop: '0.3rem', textAlign: 'left', width: '100%' }}>
                <span>{entry.name}</span>
                <span style={{ ...mono, color: 'var(--color-accent)', fontSize: '0.6rem' }}>CR {formatCr(entry.challengeRating)}</span>
              </button>
            ))}
            {needle !== '' && catalog && matches.length === 0 ? <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem' }}>No monsters match “{search.trim()}”.</p> : null}
          </div>
        </div>

        {/* Rating */}
        <div style={{ ...pane, overflowY: 'auto', padding: '1.25rem' }}>
          <h2 style={heading}>Rating {ratingBusy ? '(updating…)' : ''}</h2>
          {ratingError ? <p style={{ color: 'var(--color-error)', fontSize: '0.85rem' }}>{ratingError}</p> : null}
          {!view && !ratingError ? <p style={{ color: 'var(--color-text-muted)', fontSize: '0.85rem' }}>Add at least one party member and one monster. The engine rates the encounter; nothing is computed on this screen.</p> : null}
          {view ? (
            <>
              <div style={{ background: view.tierConfidence === 'unverified' ? 'var(--color-warn-bg)' : 'var(--color-surface)', border: `1px solid ${view.tierConfidence === 'unverified' ? 'var(--color-warn-border)' : 'var(--color-border)'}`, borderRadius: 10, padding: '0.9rem 1rem' }}>
                <div style={{ fontSize: '1.6rem', fontWeight: 800, letterSpacing: '-0.02em' }}>{describeTier(rating!.difficulty)}</div>
                {view.tierConfidence === 'unverified' ? (
                  <p style={{ color: 'var(--color-warn)', fontSize: '0.85rem', fontWeight: 700, margin: '0.4rem 0 0' }}>
                    Unverified: one or more creatures are outside the engine’s verified {view.verifiedRangeLabel} XP table. See the marked monsters — their CR was re-rated before this tier was computed.
                  </p>
                ) : (
                  <p style={{ color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: '0.4rem 0 0' }}>Every creature is inside the engine’s verified {view.verifiedRangeLabel} XP table.</p>
                )}
              </div>
              <div style={{ display: 'grid', gap: '0.5rem', gridTemplateColumns: 'repeat(3, 1fr)', margin: '1rem 0' }}>
                {[
                  ['Avg party level', view.apl],
                  ['Encounter level', view.el],
                  ['EL − APL', view.elMinusApl],
                ].map(([label, value]) => (
                  <div key={label} style={{ background: 'var(--color-surface)', border: '1px solid var(--color-border)', borderRadius: 8, padding: '0.6rem' }}>
                    <span style={{ ...mono, color: 'var(--color-text-muted)', display: 'block', fontSize: '0.55rem' }}>{label}</span>
                    <strong style={{ fontSize: '1.1rem' }}>{value}</strong>
                  </div>
                ))}
              </div>
              {view.monsters.some((m) => m.flag) ? (
                <div style={{ marginBottom: '1rem' }}>
                  <h3 style={heading}>Outside the verified table</h3>
                  {view.monsters
                    .filter((m) => m.flag)
                    .map((m, i) => (
                      <p key={`${m.name}:${i}`} style={{ color: 'var(--color-warn)', fontSize: '0.8rem', margin: '0 0 0.4rem' }}>
                        <strong>{m.name}</strong> — CR {m.crShown}. {m.flag}
                      </p>
                    ))}
                </div>
              ) : null}
              <h3 style={heading}>How to read this</h3>
              <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.8rem', margin: '0 0 0.5rem' }}>Difficulty scale: {view.scaleNote}</p>
              <p style={{ color: 'var(--color-text-secondary)', fontSize: '0.8rem', margin: '0 0 0.5rem' }}>
                Party size: the engine rates {rating!.party.length} member{rating!.party.length === 1 ? '' : 's'} on their levels alone and applies no adjustment for how many there are.
              </p>
              {view.caveats.map((caveat) => (
                <p key={caveat} style={{ color: 'var(--color-text-secondary)', fontSize: '0.8rem', margin: '0 0 0.5rem' }}>
                  {caveat}
                </p>
              ))}
            </>
          ) : null}
        </div>
      </div>
    </section>
  );
}
