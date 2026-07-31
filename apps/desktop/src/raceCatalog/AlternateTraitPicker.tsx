import { useEffect, useMemo, useState, type CSSProperties } from 'react';
import type {
  AlternateRacialTraitsResponse,
  RacePickerDto,
  RaceSelectionResponse,
} from '../boundary/loadAlternateRacialTraits';
import {
  alternateTraitPickerAvailable,
  loadAlternateRacialTraitsRuntime,
  resolveRaceAlternateSelectionRuntime,
  NO_RUNTIME_MESSAGE,
} from './alternateTraitPickerRuntime';
import {
  blocksByAlternateKey,
  describeBlock,
  describePicker,
  describeReplacement,
  describeSelectionOutcome,
  orderRacesByAlternateCount,
  selectionWarnings,
  suppressionsByTraitKey,
  toggleSelection,
} from './alternateTraitPickerModel';

const panel: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 10,
};

const muted: CSSProperties = { color: 'var(--color-text-muted)', fontSize: '0.75rem' };

/**
 * The Advanced Race Guide's Alternate Racial Traits picker.
 *
 * For a chosen race this shows every ARG alternate, the standard trait(s) each
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
 */
export function AlternateTraitPicker() {
  const [menu, setMenu] = useState<AlternateRacialTraitsResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [raceKey, setRaceKey] = useState<string | null>(null);
  const [selectedByRace, setSelectedByRace] = useState<Record<string, string[]>>({});
  const [selection, setSelection] = useState<RaceSelectionResponse | null>(null);
  const [selectionError, setSelectionError] = useState<string | null>(null);

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
      })
      .catch((cause: unknown) => {
        setError(cause instanceof Error ? cause.message : 'Unknown alternate racial traits failure');
      });
  }, []);

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
    resolveRaceAlternateSelectionRuntime(raceKey, selected)
      .then((response) => {
        if (current) setSelection(response);
      })
      .catch((cause: unknown) => {
        if (current) setSelectionError(cause instanceof Error ? cause.message : 'Unknown resolution failure');
      });
    return () => {
      current = false;
    };
  }, [raceKey, selected]);

  const suppressed = useMemo(() => suppressionsByTraitKey(selection), [selection]);
  const blocked = useMemo(() => blocksByAlternateKey(selection), [selection]);
  const warnings = useMemo(() => selectionWarnings(selection), [selection]);

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
        The Advanced Race Guide&apos;s alternate racial traits — {describePicker(menu)}. Choosing one replaces the
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
                The Advanced Race Guide declares no alternate racial traits for {race.raceName}.
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
                  <p style={{ ...muted, margin: '0.2rem 0 0' }}>{alternate.description}</p>
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
