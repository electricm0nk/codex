import { useEffect, useState } from 'react';
import { createPortal } from 'react-dom';
import type { HeldClass } from './characterProgression';
import type { ExplanationDto } from '../boundary/loadSavedCharacterDetail';
import {
  DEFAULT_PICK_MARKER,
  SKILLS,
  skillPointsStatus,
  isClassSkill,
  type ClassSkillLookup,
  maxClassSkillRanks,
  maxCrossClassSkillRanks,
  sheetSkillTotal,
  skillPointsSpent,
} from './skillsModel';

/**
 * "Manage skill allocation" popup: adjust ranks per skill against the
 * character's earned point pool. PF1 (CRB Chapter 4): one point buys one rank,
 * class skill or not (SD-36 F7a: the 3.5 two-point cross-class rank is gone);
 * the max-rank caps below are still the pre-F7 `maxClassSkillRanks` /
 * `maxCrossClassSkillRanks` (named in the F7a receipt).
 *
 * `onAccept` only hands the draft allocation back to the caller — this
 * component has no I/O of its own (matching `LevelUpDialog`'s split). The
 * caller (`CharacterSheet`'s `handleSkillAllocationAccept`) persists it via
 * the real `set_skill_allocations` Tauri command. The engine reads each skill at
 * its own rank, and one with no rank untrained; only a build it refuses for
 * something else comes back `Blocked`, with real diagnostics.
 *
 * Each row's total is the saved character's `skill.total.<skill>` row (E8.7), computed in Rust; a
 * row whose ranks the draft has changed reads "on save" until the engine states its new total. With
 * no saved character (the Create form) no total is shown: nothing is summed here.
 */

export function SkillAllocationDialog(props: {
  open: boolean;
  onClose: () => void;
  heldClasses: HeldClass[];
  /** SD-36 F6a: the engine's class skills for the held classes (`classSkillLookup`). */
  classSkills: ClassSkillLookup;
  characterLevel: number;
  /** `null` when a held class states no skill ranks per level: nothing can be allocated. */
  totalPoints: number | null;
  allocation: Record<string, number>;
  /** The saved character's explanations, whose `skill.total.<skill>` rows each row prints. */
  explanations?: readonly ExplanationDto[];
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

  const spent = skillPointsSpent(draft);
  const remaining = props.totalPoints === null ? null : props.totalPoints - spent;
  const pointsStatus = skillPointsStatus(props.totalPoints, remaining ?? 0);
  /** Spendable budget for the +/- controls; an Unknown total allows no spend. */
  const budget = remaining ?? 0;

  function adjustRank(skillName: string, classSkill: boolean, delta: 1 | -1) {
    setDraft((prev) => {
      const current = prev[skillName] ?? 0;
      const next = current + delta;
      const max = classSkill ? maxClassSkillRanks(props.characterLevel) : maxCrossClassSkillRanks(props.characterLevel);
      if (next < 0 || next > max) {
        return prev;
      }
      if (delta === 1) {
        if (props.totalPoints === null || skillPointsSpent(prev) + 1 > props.totalPoints) {
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
            <p
              role="status"
              style={{
                color: pointsStatus.tone === 'ok' ? 'var(--color-accent)' : pointsStatus.tone === 'over' ? 'var(--color-error)' : 'var(--color-warn)',
                fontSize: '0.95rem',
                fontWeight: 700,
                margin: '0.35rem 0 0',
              }}
            >
              {pointsStatus.text}
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
            const classSkill = isClassSkill(props.classSkills, skill.name);
            const ranks = draft[skill.name] ?? 0;
            const saved = ranks === (props.allocation[skill.name] ?? 0);
            const total = props.explanations && saved ? sheetSkillTotal(props.explanations, skill.name) : null;
            const max = classSkill ? maxClassSkillRanks(props.characterLevel) : maxCrossClassSkillRanks(props.characterLevel);
            return (
              <div key={skill.name} style={{ alignItems: 'center', borderBottom: '1px solid var(--color-border)', display: 'flex', gap: '0.5rem', padding: '0.4rem 0' }}>
                <span style={{ flex: 1, fontSize: '0.85rem' }}>
                  {skill.name}
                  {classSkill ? <span style={{ color: 'var(--color-accent)', fontSize: '0.68rem', fontWeight: 700 }}> · class</span> : null}
                  {classSkill && props.classSkills.isDefaultPick(skill.name) ? (
                    <span style={{ color: 'var(--color-text-muted)', fontSize: '0.68rem' }}> ({DEFAULT_PICK_MARKER})</span>
                  ) : null}
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
                  disabled={ranks >= max || budget < 1}
                  style={{
                    background: 'var(--color-accent)',
                    border: '1px solid var(--color-border)',
                    borderRadius: 6,
                    color: 'var(--color-on-accent)',
                    cursor: ranks < max && budget >= 1 ? 'pointer' : 'not-allowed',
                    fontWeight: 800,
                    height: 22,
                    opacity: ranks < max && budget >= 1 ? 1 : 0.5,
                    width: 22,
                  }}
                >
                  +
                </button>
                {props.explanations ? (
                  <span style={{ color: 'var(--color-text-secondary)', minWidth: 34, textAlign: 'right' }}>
                    {!saved ? 'on save' : total === null ? '—' : total >= 0 ? `+${total}` : total}
                  </span>
                ) : null}
              </div>
            );
          })}
        </div>

        <footer style={{ alignItems: 'center', borderTop: '1px solid var(--color-border)', display: 'flex', justifyContent: 'space-between', padding: '0.85rem 1.5rem' }}>
          <span />
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
