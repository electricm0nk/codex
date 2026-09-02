import { useEffect, useMemo, useState, type CSSProperties } from 'react';
import type {
  AdoptedRaceOptionDto,
  AdoptiveParentageOptionDto,
  AlternateRacialTraitsResponse,
  RacePickerDto,
  RaceSelectionResponse,
} from '../boundary/loadAlternateRacialTraits';
import type { CharacterSummaryDto } from '../boundary/loadListSavedCharacters';
import {
  alternateTraitPickerAvailable,
  loadAlternateRacialTraitsRuntime,
  loadCharacterContextsRuntime,
  loadHeldFeatsRuntime,
  resolveRaceAlternateSelectionRuntime,
  NO_RUNTIME_MESSAGE,
} from './alternateTraitPickerRuntime';
import {
  blocksByAlternateKey,
  describeAdoptedRaceGrants,
  describeAdoptionOptions,
  describeAdoptiveParentageGrants,
  describeBlock,
  describeCharacterContext,
  describePicker,
  describeReplacement,
  describeSelectionOutcome,
  descriptionsByTraitKey,
  orderRacesByAlternateCount,
  selectionWarnings,
  suppressionsByTraitKey,
  toggleSelection,
  traitDescription,
} from './alternateTraitPickerModel';

const panel: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 10,
};

const muted: CSSProperties = { color: 'var(--color-text-muted)', fontSize: '0.75rem' };

/**
 * The Alternate Racial Traits picker. Book-agnostic: it serves whatever
 * `race_catalog::RACE_CORPUS_BOOKS` loads, and each row carries its own book
 * code. The book list is deliberately not enumerated here — three successive
 * versions of this comment went stale within one bundle as SD-29's race-trait
 * lane landed Monster Codex, then APG, then Inner Sea Races.
 *
 * For a chosen race this shows every alternate, the standard trait(s) each
 * one replaces, and — once selected — the standard traits that really were
 * suppressed and the sibling alternates the choice locked out.
 *
 * Every one of those three answers is the backend's:
 *
 * - the replace links come from `race_trait_picker.rs`, matched on the corpus
 *   replace-flag, never on trait names;
 * - the suppressions come from `RaceCorpus::resolve`, the one implementation of
 *   `decisions.md §26`'s protocol;
 * - the lock-outs come from ARG's own `PREMULT` self-exclusion guard.
 *
 * This component chooses layout and wording. It does not decide rules.
 *
 * # Whose numbers are these?
 *
 * A racial trait's prose *states magnitudes* — "three times per day", "a +1
 * bonus on attack rolls" — and feats a character holds change several of them.
 * Every description below is therefore the engine's rendering of the corpus
 * row's own `DESC:` tokens against a character's display values, never the
 * stored prose (which has its numbers collapsed at ingest, and for
 * `Halfling ~ Adaptable Luck` lost outright).
 *
 * The "showing numbers for" selector is how the character reaches the engine:
 * it loads a **saved** character and passes that character's real persisted
 * `selectedFeats`. With no character chosen the screen shows the racial base
 * and says so. Nothing here computes a display value, and nothing invents a
 * character to demonstrate one.
 */
export function AlternateTraitPicker() {
  const [menu, setMenu] = useState<AlternateRacialTraitsResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [raceKey, setRaceKey] = useState<string | null>(null);
  const [selectedByRace, setSelectedByRace] = useState<Record<string, string[]>>({});
  const [selection, setSelection] = useState<RaceSelectionResponse | null>(null);
  const [selectionError, setSelectionError] = useState<string | null>(null);
  const [characters, setCharacters] = useState<CharacterSummaryDto[]>([]);
  const [characterId, setCharacterId] = useState<string>('');
  const [heldFeats, setHeldFeats] = useState<string[]>([]);
  const [characterError, setCharacterError] = useState<string | null>(null);
  // Adoptive Parentage / Adopted Race options: two independent single-select
  // pickers, one per option kind. Both `text_only` — the whole bar for
  // reaching a player is the description rendered below once selected.
  const [parentageKey, setParentageKey] = useState<string | null>(null);
  const [adoptedRaceKey, setAdoptedRaceKey] = useState<string | null>(null);

  useEffect(() => {
    if (!alternateTraitPickerAvailable()) {
      setError(NO_RUNTIME_MESSAGE);
      return;
    }
    loadAlternateRacialTraitsRuntime()
      .then((response) => {
        setMenu(response);
        const first = orderRacesByAlternateCount(response.races)[0];
        setRaceKey((current) => current ?? first?.raceKey ?? null);
        setParentageKey((current) => current ?? response.adoptiveParentageOptions[0]?.key ?? null);
        setAdoptedRaceKey((current) => current ?? response.adoptedRaceOptions[0]?.key ?? null);
      })
      .catch((cause: unknown) => {
        setError(cause instanceof Error ? cause.message : 'Unknown alternate racial traits failure');
      });
    loadCharacterContextsRuntime()
      .then(setCharacters)
      .catch((cause: unknown) => {
        // A failure to list characters must not blank the catalogue: the base
        // numbers are still true and still worth showing. It is reported on the
        // selector's own line rather than swallowed.
        setCharacterError(cause instanceof Error ? cause.message : 'Unknown saved-character listing failure');
      });
  }, []);

  // The chosen character's real persisted feats. Cleared the moment the
  // selection changes, so the screen never shows one character's numbers under
  // another's name while the load is in flight.
  useEffect(() => {
    if (!characterId) {
      setHeldFeats([]);
      setCharacterError(null);
      return;
    }
    let current = true;
    setHeldFeats([]);
    setCharacterError(null);
    loadHeldFeatsRuntime(characterId)
      .then((feats) => {
        if (current) setHeldFeats(feats);
      })
      .catch((cause: unknown) => {
        if (current) {
          setCharacterError(cause instanceof Error ? cause.message : 'Unknown character load failure');
        }
      });
    return () => {
      current = false;
    };
  }, [characterId]);

  const races = useMemo(() => orderRacesByAlternateCount(menu?.races ?? []), [menu]);
  const race: RacePickerDto | null = useMemo(
    () => races.find((candidate) => candidate.raceKey === raceKey) ?? null,
    [races, raceKey],
  );
  const selected = useMemo(() => (raceKey ? (selectedByRace[raceKey] ?? []) : []), [selectedByRace, raceKey]);

  // Ask the engine what this selection resolves to. Every render of the
  // suppressed/applied lists below is this response, never a local re-run of
  // the protocol.
  useEffect(() => {
    if (!raceKey || !alternateTraitPickerAvailable()) {
      return;
    }
    let current = true;
    setSelection(null);
    setSelectionError(null);
    resolveRaceAlternateSelectionRuntime(raceKey, selected, heldFeats)
      .then((response) => {
        if (current) setSelection(response);
      })
      .catch((cause: unknown) => {
        if (current) setSelectionError(cause instanceof Error ? cause.message : 'Unknown resolution failure');
      });
    return () => {
      current = false;
    };
  }, [raceKey, selected, heldFeats]);

  const suppressed = useMemo(() => suppressionsByTraitKey(selection), [selection]);
  const blocked = useMemo(() => blocksByAlternateKey(selection), [selection]);
  const warnings = useMemo(() => selectionWarnings(selection), [selection]);
  const rendered = useMemo(() => descriptionsByTraitKey(selection), [selection]);
  const characterLabel = useMemo(
    () => characters.find((candidate) => candidate.characterId === characterId)?.displayLabel ?? null,
    [characters, characterId],
  );

  const parentageOptions = menu?.adoptiveParentageOptions ?? [];
  const adoptedRaceOptions = menu?.adoptedRaceOptions ?? [];
  const selectedParentage: AdoptiveParentageOptionDto | null = useMemo(
    () => parentageOptions.find((option) => option.key === parentageKey) ?? null,
    [parentageOptions, parentageKey],
  );
  const selectedAdoptedRace: AdoptedRaceOptionDto | null = useMemo(
    () => adoptedRaceOptions.find((option) => option.key === adoptedRaceKey) ?? null,
    [adoptedRaceOptions, adoptedRaceKey],
  );

  function onToggle(key: string) {
    if (!raceKey) return;
    setSelectedByRace((current) => ({ ...current, [raceKey]: toggleSelection(current[raceKey] ?? [], key) }));
  }

  if (error) {
    return <p style={{ color: 'var(--color-danger, #d33)', margin: 0 }}>{error}</p>;
  }
  if (!menu || !race) {
    return <p style={{ ...muted, margin: 0 }}>Loading alternate racial traits…</p>;
  }

  return (
    <>
      <p style={{ ...muted, margin: '0 0 0.75rem' }}>
        Alternate racial traits from every ingested book — {describePicker(menu)}. Choosing one replaces the
        standard trait it names; the engine resolves the swap, this screen only shows it.
      </p>

      {menu.diagnostics.map((diagnostic) => (
        <p key={diagnostic} style={{ color: 'var(--color-danger, #d33)', fontSize: '0.75rem', margin: '0 0 0.5rem' }}>
          {diagnostic}
        </p>
      ))}
      {menu.findings.map((finding) => (
        <p key={finding} style={{ ...muted, margin: '0 0 0.5rem' }}>
          Corpus finding: {finding}
        </p>
      ))}

      <div style={{ alignItems: 'baseline', display: 'flex', flexWrap: 'wrap', gap: '0.5rem', marginBottom: '0.5rem' }}>
        <label htmlFor="art-character-context" style={{ ...muted, fontWeight: 600 }}>
          Showing numbers for
        </label>
        <select
          id="art-character-context"
          value={characterId}
          onChange={(event) => setCharacterId(event.target.value)}
          disabled={characters.length === 0}
          style={{
            backgroundColor: 'var(--color-surface-2)',
            border: '1px solid var(--color-border)',
            borderRadius: 6,
            color: 'var(--color-text)',
            fontSize: '0.8rem',
            padding: '0.25rem 0.5rem',
          }}
        >
          <option value="">No character — the book&apos;s printed values</option>
          {characters.map((candidate) => (
            <option key={candidate.characterId} value={candidate.characterId}>
              {candidate.displayLabel}
            </option>
          ))}
        </select>
        <span style={muted}>{describeCharacterContext(characterLabel, selection)}</span>
      </div>
      {characters.length === 0 && !characterError ? (
        <p style={{ ...muted, margin: '0 0 0.5rem' }}>
          No saved characters yet, so only the printed values can be shown. Create one to see the numbers their
          feats give them.
        </p>
      ) : null}
      {characterError ? (
        <p style={{ color: 'var(--color-danger, #d33)', fontSize: '0.75rem', margin: '0 0 0.5rem' }}>
          {characterError}
        </p>
      ) : null}

      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.5rem', marginBottom: '0.75rem' }}>
        {races.map((candidate) => (
          <button
            key={candidate.raceKey}
            type="button"
            onClick={() => setRaceKey(candidate.raceKey)}
            style={pillStyle(candidate.raceKey === raceKey)}
          >
            {candidate.raceName} ({candidate.alternates.length})
          </button>
        ))}
      </div>

      {selectionError ? (
        <p style={{ color: 'var(--color-danger, #d33)', fontSize: '0.75rem', margin: '0 0 0.5rem' }}>
          {selectionError}
        </p>
      ) : null}
      {warnings.map((warning) => (
        <p key={warning} style={{ color: 'var(--color-danger, #d33)', fontSize: '0.75rem', margin: '0 0 0.5rem' }}>
          {warning}
        </p>
      ))}

      <div style={{ display: 'grid', gap: '1rem', gridTemplateColumns: 'minmax(0, 1fr) minmax(0, 1.2fr)' }}>
        <div>
          <h3 style={{ fontSize: '0.9rem', margin: '0 0 0.4rem' }}>{race.raceName} — standard traits</h3>
          <p style={{ ...muted, margin: '0 0 0.5rem' }}>{describeSelectionOutcome(selection)}</p>
          <div style={{ ...panel, maxHeight: 420, overflowY: 'auto', padding: '0.25rem 0.9rem' }}>
            {race.standardTraits.map((standard) => {
              const swap = suppressed.get(standard.key);
              return (
                <div key={standard.key} style={{ borderBottom: '1px solid var(--color-border)', padding: '0.45rem 0' }}>
                  <span
                    style={{
                      fontWeight: 600,
                      opacity: swap ? 0.5 : 1,
                      textDecoration: swap ? 'line-through' : 'none',
                    }}
                  >
                    {standard.name}
                  </span>
                  <span style={{ ...muted, marginLeft: '0.5rem' }}>{standard.book}</span>
                  {swap ? (
                    <p style={{ ...muted, color: 'var(--color-accent)', margin: '0.15rem 0 0' }}>
                      Replaced by {swap.setByTraitName} (flag {swap.flag})
                    </p>
                  ) : null}
                  <TraitProse
                    text={traitDescription(rendered, standard.key, standard.description)}
                    row={rendered.get(standard.key)}
                    dimmed={swap !== undefined}
                  />
                </div>
              );
            })}
            {selection?.appliedTraits
              .filter((applied) => applied.role === 'flagGranted')
              .map((applied) => (
                <div key={applied.key} style={{ borderBottom: '1px solid var(--color-border)', padding: '0.45rem 0' }}>
                  <span style={{ fontWeight: 600 }}>{applied.name}</span>
                  <span style={{ ...muted, marginLeft: '0.5rem' }}>{applied.book}</span>
                  <p style={{ ...muted, color: 'var(--color-accent)', margin: '0.15rem 0 0' }}>
                    Granted by your selection
                  </p>
                </div>
              ))}
          </div>
        </div>

        <div>
          <h3 style={{ fontSize: '0.9rem', margin: '0 0 0.4rem' }}>
            Alternate racial traits ({race.alternates.length})
          </h3>
          <p style={{ ...muted, margin: '0 0 0.5rem' }}>
            {selected.length === 0
              ? 'None selected.'
              : `${selected.length} selected. ${blocked.size} further option${blocked.size === 1 ? '' : 's'} locked out.`}
          </p>
          <div style={{ ...panel, maxHeight: 420, overflowY: 'auto', padding: '0.25rem 0.9rem' }}>
            {race.alternates.length === 0 ? (
              <p style={{ ...muted, margin: '0.75rem 0' }}>
                No ingested book declares an alternate racial trait for {race.raceName}.
              </p>
            ) : null}
            {race.alternates.map((alternate) => {
              const isSelected = selected.includes(alternate.key);
              const block = blocked.get(alternate.key);
              const disabled = !isSelected && block !== undefined;
              return (
                <div
                  key={alternate.key}
                  style={{ borderBottom: '1px solid var(--color-border)', opacity: disabled ? 0.55 : 1, padding: '0.5rem 0' }}
                >
                  <label style={{ alignItems: 'baseline', cursor: disabled ? 'not-allowed' : 'pointer', display: 'flex', gap: '0.5rem' }}>
                    <input
                      type="checkbox"
                      checked={isSelected}
                      disabled={disabled}
                      onChange={() => onToggle(alternate.key)}
                    />
                    <span>
                      <span style={{ fontWeight: 700 }}>{alternate.name}</span>
                      <span style={{ ...muted, marginLeft: '0.5rem' }}>
                        {alternate.book}
                        {alternate.sourcePage ? ` ${alternate.sourcePage}` : ''}
                      </span>
                    </span>
                  </label>
                  <p style={{ ...muted, color: 'var(--color-accent)', margin: '0.2rem 0 0' }}>
                    {describeReplacement(alternate)}
                  </p>
                  <TraitProse
                    text={traitDescription(rendered, alternate.key, alternate.description)}
                    row={rendered.get(alternate.key)}
                  />
                  {block ? (
                    <p style={{ color: 'var(--color-danger, #d33)', fontSize: '0.72rem', margin: '0.2rem 0 0' }}>
                      {describeBlock(block)}
                    </p>
                  ) : null}
                </div>
              );
            })}
          </div>
        </div>
      </div>

      <div style={{ marginTop: '1.5rem' }}>
        <h3 style={{ fontSize: '0.9rem', margin: '0 0 0.4rem' }}>Adoptive Parentage &amp; Adopted Race options</h3>
        <p style={{ ...muted, margin: '0 0 0.5rem' }}>{describeAdoptionOptions(menu)}</p>
        <div style={{ display: 'grid', gap: '1rem', gridTemplateColumns: 'minmax(0, 1fr) minmax(0, 1fr)' }}>
          <div>
            <h4 style={{ fontSize: '0.82rem', margin: '0 0 0.4rem' }}>
              Adoptive Parentage ({parentageOptions.length})
            </h4>
            <p style={{ ...muted, margin: '0 0 0.4rem' }}>
              A Human character who replaces Bonus Feat with the &quot;Adoptive Parentage&quot; alternate
              trait picks one of these — which other race raised them.
            </p>
            {parentageOptions.length === 0 ? (
              <p style={{ ...muted, margin: '0.5rem 0' }}>No Adoptive Parentage options loaded.</p>
            ) : (
              <>
                <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.4rem', marginBottom: '0.5rem' }}>
                  {parentageOptions.map((option) => (
                    <button
                      key={option.key}
                      type="button"
                      onClick={() => setParentageKey(option.key)}
                      style={pillStyle(option.key === parentageKey)}
                    >
                      {option.name}
                    </button>
                  ))}
                </div>
                {selectedParentage ? (
                  <div style={{ ...panel, padding: '0.6rem 0.9rem' }}>
                    <span style={{ fontWeight: 700 }}>{selectedParentage.name}</span>
                    <span style={{ ...muted, marginLeft: '0.5rem' }}>{selectedParentage.book}</span>
                    <p style={{ margin: '0.3rem 0 0' }}>{selectedParentage.description}</p>
                    <p style={{ ...muted, color: 'var(--color-accent)', margin: '0.3rem 0 0' }}>
                      {describeAdoptiveParentageGrants(selectedParentage)}
                    </p>
                  </div>
                ) : null}
              </>
            )}
          </div>

          <div>
            <h4 style={{ fontSize: '0.82rem', margin: '0 0 0.4rem' }}>
              Adopted Race selectors ({adoptedRaceOptions.length})
            </h4>
            <p style={{ ...muted, margin: '0 0 0.4rem' }}>
              A character of the named race's own type may pick one trait from that race's real Trait
              pool.
            </p>
            {adoptedRaceOptions.length === 0 ? (
              <p style={{ ...muted, margin: '0.5rem 0' }}>No Adopted Race selectors loaded.</p>
            ) : (
              <>
                <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.4rem', marginBottom: '0.5rem' }}>
                  {adoptedRaceOptions.map((option) => (
                    <button
                      key={option.key}
                      type="button"
                      onClick={() => setAdoptedRaceKey(option.key)}
                      style={pillStyle(option.key === adoptedRaceKey)}
                    >
                      {option.adoptedRace}
                    </button>
                  ))}
                </div>
                {selectedAdoptedRace ? (
                  <div style={{ ...panel, padding: '0.6rem 0.9rem' }}>
                    <span style={{ fontWeight: 700 }}>{selectedAdoptedRace.name}</span>
                    <span style={{ ...muted, marginLeft: '0.5rem' }}>{selectedAdoptedRace.book}</span>
                    <p style={{ ...muted, color: 'var(--color-accent)', margin: '0.3rem 0 0' }}>
                      {describeAdoptedRaceGrants(selectedAdoptedRace)}
                    </p>
                    {selectedAdoptedRace.grants.map((grant) => (
                      <div key={grant.key} style={{ borderTop: '1px solid var(--color-border)', margin: '0.4rem 0 0', padding: '0.4rem 0 0' }}>
                        <span style={{ fontWeight: 600 }}>{grant.name}</span>
                        <span style={{ ...muted, marginLeft: '0.5rem' }}>{grant.book}</span>
                        <p style={{ ...muted, margin: '0.2rem 0 0' }}>
                          {grant.description ?? 'No corpus description ingested for this grant.'}
                        </p>
                      </div>
                    ))}
                  </div>
                ) : null}
              </>
            )}
          </div>
        </div>
      </div>
    </>
  );
}

/**
 * One trait's prose, plus the two things the engine says *about* that prose and
 * this screen must not hide.
 *
 * - `movedByFeats` marks a sentence whose number the selected character's feats
 *   changed from the printed one. Without the mark a player reading "5 times
 *   per day" has no way to tell it apart from the book's own text.
 * - `droppedArgs` names `DESC:` arguments the engine could not resolve. No
 *   shipped record reports one today (derived, not assumed —
 *   `race_trait_picker.rs` prints the live count), and the branch stays because
 *   the alternative to showing it is a silently incomplete sentence.
 */
function TraitProse({
  text,
  row,
  dimmed = false,
}: {
  text: string;
  row?: { movedByFeats: boolean; droppedArgs: string[] };
  dimmed?: boolean;
}) {
  return (
    <>
      <p style={{ ...muted, margin: '0.2rem 0 0', opacity: dimmed ? 0.6 : 1 }}>
        {text}
        {row?.movedByFeats ? (
          <span style={{ color: 'var(--color-accent)', fontWeight: 700, marginLeft: '0.35rem' }}>
            ← your feats changed this value
          </span>
        ) : null}
      </p>
      {row && row.droppedArgs.length > 0 ? (
        <p style={{ color: 'var(--color-danger, #d33)', fontSize: '0.72rem', margin: '0.15rem 0 0' }}>
          This engine cannot state {row.droppedArgs.join(', ')} for this trait, so the sentence above is missing
          that magnitude rather than guessing it.
        </p>
      ) : null}
    </>
  );
}

function pillStyle(active: boolean): CSSProperties {
  return {
    backgroundColor: active ? 'var(--color-accent)' : 'var(--color-surface-2)',
    border: '1px solid var(--color-border)',
    borderRadius: 999,
    color: active ? 'var(--color-on-accent)' : 'var(--color-text)',
    cursor: 'pointer',
    fontSize: '0.8rem',
    padding: '0.4rem 0.9rem',
  };
}
