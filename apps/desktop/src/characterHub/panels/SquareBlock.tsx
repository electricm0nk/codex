import { Fragment, type CSSProperties, type ReactNode } from 'react';
import type { EncumbranceDto } from '../../boundary/loadCreateCharacter';
import type { ExplanationDto } from '../../boundary/loadSavedCharacterDetail';
import { describeEncumbrance } from '../encumbranceTabModel';
import { SQUARE_BLOCK_LAYOUT, SQUARE_BLOCK_STYLE, boxesInColumn, type SquareBoxId } from '../sheetLayout';
import { fmt, panel } from './StatBox';

/**
 * The 2x2 square block under the character's name (Brief 2 L2): Hit Points, Initiative,
 * Encumbrance, Conditions, placed by `SQUARE_BLOCK_LAYOUT`.
 */

/** The engine's initiative sheet total (`codex::rules_core::pilot_compute::INITIATIVE_TOTAL_ID`). */
export const INITIATIVE_TOTAL_ID = 'initiative.total';

/** The `initiative.total` row, or `null` when the engine emitted none (never a defaulted 0). */
export function initiativeRow(explanations: readonly ExplanationDto[]): ExplanationDto | null {
  return explanations.find((row) => row.id === INITIATIVE_TOTAL_ID) ?? null;
}

/** `Normal` when the character is under no condition (operator ruling), else the conditions. */
export function conditionsLabel(conditions: readonly string[]): string {
  return conditions.length === 0 ? 'Normal' : conditions.join(', ');
}

const CAPTION: CSSProperties = {
  color: 'var(--color-text-muted)',
  fontSize: '0.66rem',
  letterSpacing: '0.06em',
  margin: 0,
  textTransform: 'uppercase',
};
const VALUE: CSSProperties = { fontSize: '1.15rem', fontWeight: 800, margin: '0.2rem 0 0' };
const NOTE: CSSProperties = { color: 'var(--color-text-secondary)', fontSize: '0.7rem', margin: '0.3rem 0 0' };

function SquareBox(props: { title: string; style: CSSProperties; children: ReactNode }) {
  return (
    <section
      aria-label={props.title}
      style={{ ...panel, ...props.style, display: 'flex', flexDirection: 'column', minWidth: 0, overflow: 'auto', padding: '0.6rem 0.75rem' }}
    >
      <p style={CAPTION}>{props.title}</p>
      {props.children}
    </section>
  );
}

export interface SquareBlockProps {
  /** `null` when a held class's hit die is unknown to the engine's hit-point fold: Unknown, never a guess. */
  hp: number | null;
  /** The engine's `initiative.total` row; `null` reads as absent. */
  initiative: ExplanationDto | null;
  /** The engine's carried load (`corpusDerived.encumbrance`); absent reads as absent. */
  encumbrance: EncumbranceDto | undefined;
  /** `CharacterCustom.conditions`. */
  conditions: readonly string[];
  /**
   * Why a value is absent, shown in its box in place of the em dash (or `Unknown` hit points):
   * Create's draft preview is loading, has no level yet, or was refused by the engine. The sheet
   * passes none and keeps its em dash.
   */
  absent?: string;
}

export function SquareBlock(props: SquareBlockProps) {
  const absentValue = (fallback: string) =>
    props.absent === undefined ? <p style={VALUE}>{fallback}</p> : <p role="note" style={NOTE}>{props.absent}</p>;
  function renderBox(id: SquareBoxId, style: CSSProperties): ReactNode {
    switch (id) {
      case 'hitPoints':
        return (
          <SquareBox title="Hit Points" style={style}>
            {props.hp === null ? absentValue('Unknown') : <p style={VALUE}>{`${props.hp} / ${props.hp}`}</p>}
          </SquareBox>
        );
      case 'initiative':
        return (
          <SquareBox title="Initiative" style={style}>
            {props.initiative ? (
              <>
                <p style={{ ...VALUE, fontSize: '2rem' }}>{fmt(props.initiative.value)}</p>
                <p style={NOTE}>{props.initiative.detail}</p>
              </>
            ) : (
              absentValue('—')
            )}
          </SquareBox>
        );
      case 'encumbrance': {
        const described = props.encumbrance ? describeEncumbrance(props.encumbrance) : null;
        return (
          <SquareBox title="Encumbrance" style={style}>
            {described ? (
              <>
                <p style={VALUE}>{described.levelLabel}</p>
                <p style={NOTE}>{described.totalWeightLabel} carried</p>
                <p style={NOTE}>Light / Med / Heavy {described.capacityLabel}</p>
              </>
            ) : (
              absentValue('—')
            )}
          </SquareBox>
        );
      }
      case 'conditions':
        return (
          <SquareBox title="Conditions" style={style}>
            <p style={VALUE}>{conditionsLabel(props.conditions)}</p>
          </SquareBox>
        );
    }
  }
  return (
    <div style={SQUARE_BLOCK_STYLE}>
      {boxesInColumn(SQUARE_BLOCK_LAYOUT, 'square').map((box) => (
        <Fragment key={box.id}>{renderBox(box.id, box.boxStyle)}</Fragment>
      ))}
    </div>
  );
}
