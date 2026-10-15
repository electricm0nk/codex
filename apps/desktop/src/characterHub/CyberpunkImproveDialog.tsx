import { useEffect, useRef, useState, type CSSProperties } from 'react';
import {
  applyCyberpunkImprovement,
  previewCyberpunkImprovement,
  type CyberpunkImprovePlan,
  type CyberpunkImprovePreview,
  type CyberpunkImproveRow,
} from '../boundary/cprImprove';
import type { LoadSavedCharacterResponse } from '../boundary/loadSavedCharacterDetail';
import {
  EMPTY_CYBERPUNK_IMPROVE_PLAN,
  addPurchase,
  canSpend,
  improvePlanFromIpText,
  improveTotals,
  nextTarget,
  purchaseCost,
  purchaseLabel,
  removePurchase,
  skillPurchaseKey,
} from './cyberpunkImproveModel';

/**
 * The Cyberpunk RED improve dialog (SD-38 E5.1): the player enters the IP the GM awarded and buys
 * Skill Levels and Role Ability Ranks one step at a time. Every change is priced by the engine
 * (`preview_cyberpunk_improvement`): each purchase's cost, the IP available, the plan's cost and the
 * IP left are its rows, and a skipped Level or an overspend shows its refusal. "Spend IP" saves
 * through `apply_cyberpunk_improvement` and hands the reloaded sheet to `onApplied`.
 */
export function CyberpunkImproveDialog(props: {
  characterId: string;
  onApplied: (detail: LoadSavedCharacterResponse) => void;
  onClose: () => void;
}) {
  const { characterId } = props;
  const [plan, setPlan] = useState<CyberpunkImprovePlan>(EMPTY_CYBERPUNK_IMPROVE_PLAN);
  const [ipText, setIpText] = useState('');
  const [ipError, setIpError] = useState<string | null>(null);
  const [preview, setPreview] = useState<CyberpunkImprovePreview | null>(null);
  const [previewError, setPreviewError] = useState<string | null>(null);
  const [skillKey, setSkillKey] = useState('');
  const [specialty, setSpecialty] = useState('');
  const [skillTarget, setSkillTarget] = useState('');
  const [rankTarget, setRankTarget] = useState('');
  const [spending, setSpending] = useState(false);
  const [spendError, setSpendError] = useState<string | null>(null);
  const mounted = useRef(true);
  useEffect(
    () => () => {
      mounted.current = false;
    },
    [],
  );

  useEffect(() => {
    let live = true;
    previewCyberpunkImprovement(characterId, plan)
      .then((answer) => {
        if (live) {
          setPreview(answer);
          setPreviewError(null);
        }
      })
      .catch((cause: unknown) => {
        if (live) {
          setPreviewError(cause instanceof Error ? cause.message : String(cause));
        }
      });
    return () => {
      live = false;
    };
  }, [characterId, plan]);

  const option = preview?.skills.find((s) => s.key === skillKey) ?? null;
  const key = option === null ? null : skillPurchaseKey(option, specialty);
  const skillDefault = option === null || key === null ? null : nextTarget(plan, { kind: 'skill', key }, option.level);
  const role = preview?.role ?? null;
  const rankDefault = role === null ? null : nextTarget(plan, { kind: 'roleRank' }, role.rank);
  const totals = improveTotals(preview);

  const targetOf = (text: string, fallback: number | null): number | null => {
    const trimmed = text.trim();
    if (trimmed === '') {
      return fallback;
    }
    return /^\d+$/.test(trimmed) ? Number(trimmed) : null;
  };
  const skillTo = targetOf(skillTarget, skillDefault);
  const rankTo = targetOf(rankTarget, rankDefault);

  const onSpend = () => {
    setSpending(true);
    setSpendError(null);
    applyCyberpunkImprovement(characterId, plan)
      .then((reloaded) => {
        if (mounted.current) {
          props.onApplied(reloaded);
        }
      })
      .catch((cause: unknown) => {
        if (mounted.current) {
          setSpendError(cause instanceof Error ? cause.message : String(cause));
        }
      })
      .finally(() => {
        if (mounted.current) {
          setSpending(false);
        }
      });
  };

  return (
    <div data-testid="cpr-improve-dialog" role="dialog" aria-label="Improve Improvement Points" style={dialogStyle}>
      <p style={headingStyle}>Improve: spend Improvement Points</p>
      <p style={noteStyle}>
        The cost is based on the next Level/Rank; you can&apos;t skip Levels/Ranks (p. 411). Typical Skills cost 20 IP x the next Level,
        Difficult (x2) Skills 40 IP x, Role Ability Ranks 60 IP x the next Rank.
      </p>

      <label style={fieldStyle}>
        IP awarded
        <input
          aria-label="IP entered"
          placeholder="IP entered"
          inputMode="numeric"
          value={ipText}
          onChange={(event) => {
            setIpText(event.target.value);
            const next = improvePlanFromIpText(plan, event.target.value);
            setIpError(next.error);
            setPlan(next.plan);
          }}
          style={inputStyle}
        />
      </label>
      {ipError === null ? null : <p style={errorStyle}>{ipError}</p>}
      <p data-testid="cpr-improve-available" style={{ margin: '0.4rem 0' }}>
        IP available: <Figure row={totals.available} />
      </p>

      <div style={rowStyle}>
        <select
          aria-label="Skill to improve"
          value={skillKey}
          onChange={(event) => {
            setSkillKey(event.target.value);
            setSpecialty('');
            setSkillTarget('');
          }}
          style={inputStyle}
        >
          <option value="">— choose a skill —</option>
          {(preview?.skills ?? []).map((s) => (
            <option key={s.key} value={s.key}>
              {s.specialtyRequired ? `${s.name} (new specialty)` : s.key} — Level {s.level}
              {s.next === null ? ', Level 10' : `, next Level ${s.next.value} IP`}
            </option>
          ))}
        </select>
        {option?.specialtyRequired === true ? (
          <input aria-label="Specialty" placeholder="Specialty" value={specialty} onChange={(event) => setSpecialty(event.target.value)} style={inputStyle} />
        ) : null}
        <input
          aria-label="Skill Level to buy"
          placeholder={skillDefault === null ? 'Level' : `Level ${skillDefault}`}
          inputMode="numeric"
          value={skillTarget}
          onChange={(event) => setSkillTarget(event.target.value)}
          style={{ ...inputStyle, width: '6rem' }}
        />
        <button
          type="button"
          disabled={key === null || skillTo === null}
          onClick={() => {
            if (key !== null && skillTo !== null) {
              setPlan(addPurchase(plan, { kind: 'skill', key, to: skillTo }));
              setSkillTarget('');
            }
          }}
          style={buttonStyle}
        >
          Add skill level
        </button>
      </div>

      {role === null ? (
        <p style={noteStyle}>This character has no Role, so it has no Role Ability Rank to raise.</p>
      ) : (
        <div style={rowStyle}>
          <span>
            {role.name} Role Ability, Rank {role.rank}
            {role.next === null ? '' : `, next Rank ${role.next.value} IP`}
          </span>
          <input
            aria-label="Role Rank to buy"
            placeholder={rankDefault === null ? 'Rank' : `Rank ${rankDefault}`}
            inputMode="numeric"
            value={rankTarget}
            onChange={(event) => setRankTarget(event.target.value)}
            style={{ ...inputStyle, width: '6rem' }}
          />
          <button
            type="button"
            disabled={rankTo === null}
            onClick={() => {
              if (rankTo !== null) {
                setPlan(addPurchase(plan, { kind: 'roleRank', to: rankTo }));
                setRankTarget('');
              }
            }}
            style={buttonStyle}
          >
            Add Role Rank
          </button>
        </div>
      )}

      {plan.purchases.length === 0 ? null : (
        <table data-testid="cpr-improve-plan" style={{ borderCollapse: 'collapse', margin: '0.5rem 0', width: '100%' }}>
          <tbody>
            {plan.purchases.map((purchase, index) => (
              <tr key={`${index}:${purchaseLabel(purchase)}`}>
                <th scope="row" style={{ fontWeight: 600, textAlign: 'left' }}>
                  {purchaseLabel(purchase)}
                </th>
                <td style={{ textAlign: 'right' }}>
                  <Figure row={purchaseCost(preview, index)} /> IP
                </td>
                <td style={{ textAlign: 'right' }}>
                  <button type="button" aria-label={`Remove ${purchaseLabel(purchase)}`} onClick={() => setPlan(removePurchase(plan, index))} style={buttonStyle}>
                    Remove
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
      <p data-testid="cpr-improve-totals" style={{ margin: '0.4rem 0' }}>
        Plan cost: <Figure row={totals.cost} /> IP; IP left: <Figure row={totals.remaining} />
      </p>

      {preview?.refusal == null ? null : (
        <p data-testid="cpr-improve-refusal" style={errorStyle}>
          {preview.refusal}
        </p>
      )}
      {previewError === null ? null : <p style={errorStyle}>{previewError}</p>}
      {spendError === null ? null : <p style={errorStyle}>{spendError}</p>}

      <div style={{ ...rowStyle, justifyContent: 'flex-end' }}>
        <button type="button" onClick={props.onClose} style={buttonStyle}>
          Cancel
        </button>
        <button type="button" disabled={spending || !canSpend(plan, preview)} onClick={onSpend} style={buttonStyle}>
          Spend IP
        </button>
      </div>
    </div>
  );
}

/** An engine row's value (its derivation as the title), or a dash when the engine gave none. */
function Figure(props: { row: CyberpunkImproveRow | null }) {
  if (props.row === null) {
    return <span style={{ color: 'var(--color-text-faint)' }}>—</span>;
  }
  return (
    <span data-cpr-row={props.row.id} title={props.row.detail} style={{ fontWeight: 800 }}>
      {props.row.value}
    </span>
  );
}

const dialogStyle: CSSProperties = {
  border: '1px solid var(--color-border)',
  borderRadius: 8,
  margin: '0 0 1rem',
  padding: '0.75rem 1rem',
};
const headingStyle: CSSProperties = { fontWeight: 700, margin: '0 0 0.4rem' };
const noteStyle: CSSProperties = { color: 'var(--color-text-muted)', fontSize: '0.8rem', margin: '0 0 0.5rem' };
const errorStyle: CSSProperties = { color: 'var(--color-danger, #c0392b)', fontSize: '0.85rem', margin: '0.4rem 0' };
const fieldStyle: CSSProperties = { alignItems: 'center', display: 'flex', gap: '0.5rem' };
const rowStyle: CSSProperties = { alignItems: 'center', display: 'flex', flexWrap: 'wrap', gap: '0.5rem', margin: '0.4rem 0' };
const inputStyle: CSSProperties = { padding: '0.3rem 0.5rem' };
const buttonStyle: CSSProperties = {
  background: 'none',
  border: '1px solid var(--color-border)',
  borderRadius: 8,
  cursor: 'pointer',
  padding: '0.4rem 0.8rem',
};
