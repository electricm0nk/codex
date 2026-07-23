import { useEffect, useState } from 'react';
import { createPortal } from 'react-dom';
import type { AbilityScoresDto } from '../boundary/loadCreateCharacter';
import type { HeldClass } from './characterProgression';
import {
  SKILLS,
  isClassSkill,
  maxClassSkillRanks,
  maxCrossClassSkillRanks,
  skillModifier,
  skillRankCost,
} from './skillsModel';

/**
 * "Manage skill allocation" popup: adjust ranks per skill against the
 * character's earned point pool, respecting PF1's class-skill (1 point/rank)
 * vs. cross-class (2 points/rank, half the max ranks) costs.
 *
 * `onAccept` only hands the draft allocation back to the caller — this
 * component has no I/O of its own (matching `LevelUpDialog`'s split). The
 * caller (`CharacterSheet`'s `handleSkillAllocationAccept`) persists it via
 * the real `set_skill_allocations` Tauri command. Note the compute engine's
 * `Computed` path only accepts one exact hardcoded posture today
 * (Climb/Intimidate/Swim at rank 1, chain shirt equipped — see
 * `pilot_compute.rs`), so most allocations will legitimately come back
 * `Blocked` with real diagnostics rather than silently applying.
 */

export function SkillAllocationDialog(props: {
  open: boolean;
  onClose: () => void;
  heldClasses: HeldClass[];
  characterLevel: number;
  abilities: AbilityScoresDto;
  totalPoints: number;
  allocation: Record<string, number>;
  onAccept: (allocation: Record<string, number>) => void;
}) {
  const [draft, setDraft] = useState<Record<string, number>>(props.allocation);

  useEffect(() => {
    if (props.open) {
      setDraft(props.allocation);
    }
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

  const spent = SKILLS.reduce((sum, skill) => {
    const ranks = draft[skill.name] ?? 0;
    return sum + ranks * skillRankCost(isClassSkill(props.heldClasses, skill.name));
  }, 0);
  const remaining = props.totalPoints - spent;

  function adjustRank(skillName: string, classSkill: boolean, delta: 1 | -1) {
    setDraft((prev) => {
      const current = prev[skillName] ?? 0;
      const next = current + delta;
      const max = classSkill ? maxClassSkillRanks(props.characterLevel) : maxCrossClassSkillRanks(props.characterLevel);
      if (next < 0 || next > max) {
        return prev;
      }
      if (delta === 1) {
        const cost = skillRankCost(classSkill);
        const currentSpent = SKILLS.reduce((sum, skill) => sum + (prev[skill.name] ?? 0) * skillRankCost(isClassSkill(props.heldClasses, skill.name)), 0);
        if (currentSpent + cost > props.totalPoints) {
          return prev;
        }
      }
      return { ...prev, [skillName]: next };
    });
  }

  return createPortal(
    <div
      role="presentation"
      onClick={props.onClose}
      style={{ alignItems: 'center', backgroundColor: 'rgba(0, 0, 0, 0.6)', display: 'flex', inset: 0, justifyContent: 'center', padding: '2rem', position: 'fixed', zIndex: 1100 }}
    >
      <div
        role="dialog"
        aria-modal="true"
        aria-label="Manage skill allocation"
        onClick={(event) => event.stopPropagation()}
        style={{
          backgroundColor: 'var(--color-surface)',
          border: '1px solid var(--color-border)',
          borderRadius: 12,
          boxShadow: '0 24px 60px rgba(0, 0, 0, 0.55)',
          display: 'flex',
          flexDirection: 'column',
          height: 'min(680px, 90vh)',
          width: 'min(640px, 94vw)',
        }}
      >
        <header style={{ alignItems: 'center', borderBottom: '1px solid var(--color-border)', display: 'flex', justifyContent: 'space-between', padding: '1rem 1.5rem' }}>
          <div>
            <h2 style={{ fontSize: '1.1rem', margin: 0 }}>Manage skill allocation</h2>
            <p style={{ color: 'var(--color-text-muted)', fontSize: '0.78rem', margin: '0.15rem 0 0' }}>
              Class skills cost 1 point/rank; cross-class skills cost 2 and cap at half the ranks.
            </p>
          </div>
          <button
            type="button"
            aria-label="Close"
            onClick={props.onClose}
            style={{ background: 'none', border: 'none', color: 'var(--color-text-muted)', cursor: 'pointer', fontSize: '1.4rem', lineHeight: 1, padding: '0.15rem 0.35rem' }}
          >
            ×
          </button>
        </header>

        <div style={{ flex: 1, overflowY: 'auto', padding: '0.5rem 1.5rem' }}>
          {SKILLS.map((skill) => {
            const classSkill = isClassSkill(props.heldClasses, skill.name);
            const ranks = draft[skill.name] ?? 0;
            const abilityMod = props.abilities[skill.ability];
            const total = skillModifier(abilityMod, ranks, classSkill);
            const max = classSkill ? maxClassSkillRanks(props.characterLevel) : maxCrossClassSkillRanks(props.characterLevel);
            return (
              <div key={skill.name} style={{ alignItems: 'center', borderBottom: '1px solid var(--color-border)', display: 'flex', gap: '0.5rem', padding: '0.4rem 0' }}>
                <span style={{ flex: 1, fontSize: '0.85rem' }}>
                  {skill.name}
                  {classSkill ? <span style={{ color: 'var(--color-accent)', fontSize: '0.68rem', fontWeight: 700 }}> · class</span> : null}
                </span>
                <button
                  type="button"
                  aria-label={`Decrease ${skill.name}`}
                  onClick={() => adjustRank(skill.name, classSkill, -1)}
                  disabled={ranks <= 0}
                  style={{ background: 'var(--color-surface-2)', border: '1px solid var(--color-border)', borderRadius: 6, color: 'var(--color-text)', cursor: ranks > 0 ? 'pointer' : 'not-allowed', fontWeight: 800, height: 22, opacity: ranks > 0 ? 1 : 0.5, width: 22 }}
                >
                  −
                </button>
                <span style={{ minWidth: 20, textAlign: 'center' }}>{ranks}</span>
                <button
                  type="button"
                  aria-label={`Increase ${skill.name}`}
                  onClick={() => adjustRank(skill.name, classSkill, 1)}
                  disabled={ranks >= max || remaining < skillRankCost(classSkill)}
                  style={{
                    background: 'var(--color-accent)',
                    border: '1px solid var(--color-border)',
                    borderRadius: 6,
                    color: 'var(--color-on-accent)',
                    cursor: ranks < max && remaining >= skillRankCost(classSkill) ? 'pointer' : 'not-allowed',
                    fontWeight: 800,
                    height: 22,
                    opacity: ranks < max && remaining >= skillRankCost(classSkill) ? 1 : 0.5,
                    width: 22,
                  }}
                >
                  +
                </button>
                <span style={{ color: 'var(--color-text-secondary)', minWidth: 34, textAlign: 'right' }}>
                  {total >= 0 ? `+${total}` : total}
                </span>
              </div>
            );
          })}
        </div>

        <footer style={{ alignItems: 'center', borderTop: '1px solid var(--color-border)', display: 'flex', justifyContent: 'space-between', padding: '0.85rem 1.5rem' }}>
          <span style={{ color: remaining >= 0 ? 'var(--color-accent)' : 'var(--color-error)', fontWeight: 700 }}>
            {remaining} of {props.totalPoints} points remaining
          </span>
          <div style={{ display: 'flex', gap: '0.6rem' }}>
            <button
              type="button"
              onClick={props.onClose}
              style={{ backgroundColor: 'var(--color-surface-2)', border: '1px solid var(--color-border)', borderRadius: 6, color: 'var(--color-text)', cursor: 'pointer', fontSize: '0.85rem', fontWeight: 600, padding: '0.45rem 0.9rem' }}
            >
              Cancel
            </button>
            <button
              type="button"
              onClick={() => {
                props.onAccept(draft);
                props.onClose();
              }}
              style={{ backgroundColor: 'var(--color-accent)', border: '1px solid var(--color-border)', borderRadius: 6, color: 'var(--color-on-accent)', cursor: 'pointer', fontSize: '0.85rem', fontWeight: 600, padding: '0.45rem 0.9rem' }}
            >
              Accept
            </button>
          </div>
        </footer>
      </div>
    </div>,
    document.body
  );
}
