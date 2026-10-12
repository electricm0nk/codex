import { buildLevelEntries, describeLevelGains, type HeldClass } from '../characterProgression';

/**
 * The read-only per-level summary under the Create screen's HP roller (Brief 2 L4): one line per
 * character level, `Monk 1: +1 feat, +5 skills`. Feat, skills and ability increase come from
 * `buildLevelEntries`. Informational only; nothing here is a choice.
 */
export function LevelGainsPanel(props: { heldClasses: HeldClass[]; intelligenceModifier: number; isHuman: boolean }) {
  const entries = buildLevelEntries(props.heldClasses);
  if (entries.length === 0) {
    return null;
  }
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
      <ol style={{ listStyle: 'none', margin: 0, padding: 0 }}>
        {entries.map((entry) => (
          <li key={entry.characterLevel} data-level-gain style={{ color: 'var(--color-text-secondary)', fontSize: '0.8rem', padding: '0.15rem 0' }}>
            {describeLevelGains(entry, props.intelligenceModifier, props.isHuman)}
          </li>
        ))}
      </ol>
    </section>
  );
}
