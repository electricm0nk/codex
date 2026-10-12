import type { DraftLevelUpPreviewDto } from '../../boundary/draftLevelUpPreview';
import { buildLevelEntries, describeLevelGains, type HeldClass, type LevelEntry } from '../characterProgression';

/**
 * The engine's class features for each line (Brief 2 L4b), keyed by character level: the
 * automatic grant names `draft_level_up_preview` reports for that class level, verbatim.
 */
export type LevelFeaturesState =
  | { kind: 'loading' }
  | { kind: 'ready'; byLevel: Readonly<Record<number, readonly string[]>> }
  | { kind: 'error'; message: string };

export const LEVEL_FEATURES_LOADING = 'Loading the class features from the engine…';

/**
 * Asks the engine once per line what that class level grants (`fromLevel` is the class level
 * before it). One refused call refuses the whole answer: the panel shows the refusal, never a
 * partial list that reads as complete.
 */
export async function loadLevelFeatures(
  entries: readonly LevelEntry[],
  load: (classId: string, fromLevel: number) => Promise<DraftLevelUpPreviewDto>,
): Promise<Record<number, string[]>> {
  const answers = await Promise.all(entries.map((entry) => load(entry.classId, entry.classLevel - 1)));
  const byLevel: Record<number, string[]> = {};
  entries.forEach((entry, index) => {
    byLevel[entry.characterLevel] = answers[index].automaticFeatures;
  });
  return byLevel;
}

function classFeaturesSuffix(names: readonly string[] | undefined): string {
  if (names === undefined) {
    return '';
  }
  return `; class features: ${names.length === 0 ? 'none reported by the engine' : names.join(', ')}`;
}

/**
 * The read-only per-level summary under the Create screen's HP roller (Brief 2 L4): one line per
 * character level, `Monk 1: +1 feat, +5 skills; class features: …`. Feat, skills and ability
 * increase come from `buildLevelEntries`; class features from the engine (`classFeatures`).
 * Informational only; nothing here is a choice.
 */
export function LevelGainsPanel(props: {
  heldClasses: HeldClass[];
  intelligenceModifier: number;
  isHuman: boolean;
  classFeatures?: LevelFeaturesState;
}) {
  const entries = buildLevelEntries(props.heldClasses);
  if (entries.length === 0) {
    return null;
  }
  const features = props.classFeatures;
  const note = features?.kind === 'loading' ? LEVEL_FEATURES_LOADING : features?.kind === 'error' ? features.message : null;
  return (
    <section
      aria-label="Level gains"
      style={{
        backgroundColor: 'var(--color-surface)',
        border: '1px solid var(--color-border)',
        borderRadius: 10,
        minWidth: 0,
        padding: '0.75rem 1rem',
      }}
    >
      <p style={{ fontSize: '0.85rem', fontWeight: 700, margin: '0 0 0.35rem' }}>Gains per level</p>
      {note === null ? null : (
        <p role="note" style={{ color: 'var(--color-text-secondary)', fontSize: '0.75rem', margin: '0 0 0.35rem' }}>
          {note}
        </p>
      )}
      <ol style={{ listStyle: 'none', margin: 0, padding: 0 }}>
        {entries.map((entry) => (
          <li key={entry.characterLevel} data-level-gain style={{ color: 'var(--color-text-secondary)', fontSize: '0.8rem', padding: '0.15rem 0' }}>
            {describeLevelGains(entry, props.intelligenceModifier, props.isHuman) +
              classFeaturesSuffix(features?.kind === 'ready' ? features.byLevel[entry.characterLevel] : undefined)}
          </li>
        ))}
      </ol>
    </section>
  );
}
