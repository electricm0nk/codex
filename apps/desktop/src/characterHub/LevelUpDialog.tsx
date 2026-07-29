import { useEffect, useState } from 'react';
import { createPortal } from 'react-dom';
import { CLASS_OPTIONS, canTakeAnotherLevelIn, describeClassSupportLevel } from './characterHubModel';
import { previewLevelUp, totalSkillPoints, type HeldClass } from './characterProgression';

/**
 * "Level up" popup: pick a class to take the next character level in and see
 * exactly what that level grants (hit die, skill points, class features)
 * before committing. Patterned after ThemeBrowserModal's portal-based overlay
 * shell so it matches the rest of the app's modal conventions.
 *
 * Accepting calls `onAccept(classId)` and closes; the caller
 * (`CharacterSheet`'s `handleLevelUpAccept`) persists the level-up via the
 * real `level_up_character` Tauri command — including a recorded hit-die
 * choice — and refreshes the sheet on success, or surfaces the engine's
 * real diagnostics if the resulting build doesn't reach `Computed`.
 */

export function LevelUpDialog(props: {
  open: boolean;
  onClose: () => void;
  heldClasses: HeldClass[];
  intelligenceModifier: number;
  isHuman: boolean;
  onAccept: (classId: string) => void;
}) {
  const [classId, setClassId] = useState<string>('');

  /**
   * Only the classes whose *next* level this app actually claims to build.
   * `levelOptions` (via `canTakeAnotherLevelIn`) is the same engine-dump-
   * derived claim the creation picker makes, and leveling up is the other way
   * into a level — so a class the dump reports `Blocked` past level 1 must not
   * be reachable here either, and nothing may pass PF1's level-20 ceiling.
   * Filtering rather than disabling: an option that can never be chosen is
   * noise, and the character's current level is already shown in the sheet.
   */
  const levelableOptions = CLASS_OPTIONS.filter((option) =>
    canTakeAnotherLevelIn(option.id, props.heldClasses.find((held) => held.classId === option.id)?.level ?? 0)
  );

  useEffect(() => {
    if (!props.open) {
      return;
    }
    setClassId((current) => {
      if (current && levelableOptions.some((option) => option.id === current)) {
        return current;
      }
      const firstHeldAndLevelable = props.heldClasses.find((held) =>
        levelableOptions.some((option) => option.id === held.classId)
      );
      return firstHeldAndLevelable?.classId ?? levelableOptions[0]?.id ?? '';
    });
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        props.onClose();
      }
    };
    window.addEventListener('keydown', onKeyDown);
    return () => window.removeEventListener('keydown', onKeyDown);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [props.open]);

  if (!props.open) {
    return null;
  }

  const selectedOption = CLASS_OPTIONS.find((option) => option.id === classId);
  const preview = classId ? previewLevelUp(props.heldClasses, classId) : null;
  const skillPoints = preview ? totalSkillPoints(preview.skillPointsBase, props.intelligenceModifier, props.isHuman) : 0;
  const isNewClass = !props.heldClasses.some((held) => held.classId === classId);

  return createPortal(
    <div
      role="presentation"
      onClick={props.onClose}
      style={{
        alignItems: 'center',
        backgroundColor: 'rgba(0, 0, 0, 0.6)',
        display: 'flex',
        inset: 0,
        justifyContent: 'center',
        padding: '2rem',
        position: 'fixed',
        zIndex: 1100,
      }}
    >
      <div
        role="dialog"
        aria-modal="true"
        aria-label="Level up"
        onClick={(event) => event.stopPropagation()}
        style={{
          backgroundColor: 'var(--color-surface)',
          border: '1px solid var(--color-border)',
          borderRadius: 12,
          boxShadow: '0 24px 60px rgba(0, 0, 0, 0.55)',
          display: 'flex',
          flexDirection: 'column',
          width: 'min(480px, 94vw)',
        }}
      >
        <header
          style={{
            alignItems: 'center',
            borderBottom: '1px solid var(--color-border)',
            display: 'flex',
            justifyContent: 'space-between',
            padding: '1rem 1.5rem',
          }}
        >
          <h2 style={{ fontSize: '1.1rem', margin: 0 }}>Level up</h2>
          <button
            type="button"
            aria-label="Close"
            onClick={props.onClose}
            style={{ background: 'none', border: 'none', color: 'var(--color-text-muted)', cursor: 'pointer', fontSize: '1.4rem', lineHeight: 1, padding: '0.15rem 0.35rem' }}
          >
            ×
          </button>
        </header>

        <div style={{ padding: '1.25rem 1.5rem' }}>
          <label style={{ color: 'var(--color-text-muted)', display: 'block', fontSize: '0.75rem', fontWeight: 700, letterSpacing: '0.04em', marginBottom: '0.35rem', textTransform: 'uppercase' }}>
            Class
          </label>
          <select
            value={classId}
            onChange={(event) => setClassId(event.target.value)}
            style={{
              backgroundColor: 'var(--color-surface-2)',
              border: '1px solid var(--color-border)',
              borderRadius: 6,
              boxSizing: 'border-box',
              color: 'var(--color-text)',
              padding: '0.5rem 0.6rem',
              width: '100%',
            }}
          >
            {levelableOptions.map((option) => {
              const held = props.heldClasses.find((entry) => entry.classId === option.id);
              return (
                <option key={option.id} value={option.id}>
                  {option.label}
                  {held ? ` (currently ${held.level})` : ' (new class)'}
                </option>
              );
            })}
          </select>

          {selectedOption && selectedOption.supportLevel !== 'full' ? (
            <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.5rem 0 0' }}>
              {describeClassSupportLevel(selectedOption.supportLevel, selectedOption.label)}
            </p>
          ) : null}

          {preview ? (
            <div
              style={{
                backgroundColor: 'var(--color-surface-2)',
                border: '1px solid var(--color-border)',
                borderRadius: 8,
                marginTop: '1rem',
                padding: '0.7rem 0.85rem',
              }}
            >
              <p style={{ color: 'var(--color-accent)', fontSize: '0.85rem', fontWeight: 800, margin: 0 }}>
                {isNewClass ? `${preview.classLabel} 1 (new class)` : `${preview.classLabel} ${preview.classLevel}`} — character level {preview.characterLevel}
              </p>
              <p style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', margin: '0.2rem 0 0.5rem' }}>
                Hit die: d{selectedOption?.hitDie ?? 8} · Skill points: {skillPoints}
              </p>
              <ul style={{ margin: 0, paddingLeft: '1.1rem' }}>
                {preview.features.map((feature) => (
                  <li key={feature} style={{ color: 'var(--color-text-secondary)', fontSize: '0.85rem', marginBottom: '0.15rem' }}>
                    {feature}
                  </li>
                ))}
              </ul>
            </div>
          ) : null}
        </div>

        <footer style={{ borderTop: '1px solid var(--color-border)', display: 'flex', gap: '0.6rem', justifyContent: 'flex-end', padding: '0.85rem 1.5rem' }}>
          <button
            type="button"
            onClick={props.onClose}
            style={{
              backgroundColor: 'var(--color-surface-2)',
              border: '1px solid var(--color-border)',
              borderRadius: 6,
              color: 'var(--color-text)',
              cursor: 'pointer',
              fontSize: '0.85rem',
              fontWeight: 600,
              padding: '0.45rem 0.9rem',
            }}
          >
            Cancel
          </button>
          <button
            type="button"
            onClick={() => {
              props.onAccept(classId);
              props.onClose();
            }}
            disabled={!classId}
            style={{
              backgroundColor: 'var(--color-accent)',
              border: '1px solid var(--color-border)',
              borderRadius: 6,
              color: 'var(--color-on-accent)',
              cursor: 'pointer',
              fontSize: '0.85rem',
              fontWeight: 600,
              opacity: classId ? 1 : 0.6,
              padding: '0.45rem 0.9rem',
            }}
          >
            Accept
          </button>
        </footer>
      </div>
    </div>,
    document.body
  );
}
