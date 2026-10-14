import type { CSSProperties } from 'react';
import type { LayoutBox } from './sheetLayout';

/**
 * Declarative box placement for the Cyberpunk RED character sheet (`CyberpunkCharacterSheet.tsx`),
 * the registry shape of `sheetLayout.ts`: `column` says which stack a box lives in, `order` its
 * position there, `boxStyle` the style its own content takes. Moving a box is an edit to its
 * entry here, not a JSX move.
 *
 * Cyberpunk RED has no carrying-capacity rule, so the sheet has no box for one (D-12: no page of
 * the book states one, and the book's sample sheet prints none).
 */

/** The sheet's boxes. `other` holds every engine row no box above names, so none is dropped. */
export type CyberpunkSheetBoxId =
  | 'stats'
  | 'skills'
  | 'vitals'
  | 'woundTrack'
  | 'roleAbility'
  | 'armour'
  | 'weapons'
  | 'rangeDv'
  | 'cyberware'
  | 'netrunning'
  | 'reputation'
  | 'improvementPoints'
  | 'lifepath'
  | 'criticalInjuries'
  | 'other';

/** Left: the ten STATs and the skills. Right: what they give, the wound track, and the rows not
 * yet laid out. */
export type CyberpunkSheetColumn = 'statBlock' | 'vitals';

/** The registered columns, left to right. */
export const CYBERPUNK_SHEET_COLUMNS: readonly CyberpunkSheetColumn[] = ['statBlock', 'vitals'];

const STACK: CSSProperties = { display: 'flex', flexDirection: 'column', gap: '0.75rem', minWidth: 0 };

/** Style of each column (a vertical stack). */
export const CYBERPUNK_SHEET_COLUMN_STYLES: Readonly<Record<CyberpunkSheetColumn, CSSProperties>> = {
  statBlock: { ...STACK, flex: '1 1 300px' },
  vitals: { ...STACK, flex: '2 1 420px' },
};

const BOX: CSSProperties = { border: '1px solid var(--color-border)', borderRadius: 8, minWidth: 0, padding: '0.75rem' };

export const CYBERPUNK_SHEET_LAYOUT: readonly LayoutBox<CyberpunkSheetBoxId, CyberpunkSheetColumn>[] = [
  // INT to EMP: each STAT as entered, and its current value where something lowers or raises it
  // (armour on REF, DEX and MOVE; cyberware on BODY; Humanity Loss on EMP).
  { id: 'stats', column: 'statBlock', order: 0, boxStyle: BOX },
  // The 66 skills under their 9 categories, each with its level and its Skill Base (STAT + level,
  // the REF and DEX skills from the armour-penalised STAT).
  { id: 'skills', column: 'statBlock', order: 1, boxStyle: BOX },
  // Hit Points, Seriously Wounded, Death Save, Humanity (current of max), MOVE, LUCK.
  { id: 'vitals', column: 'vitals', order: 0, boxStyle: BOX },
  // The Wound States with the HP each begins below and its Stabilization DV, then the Death Save
  // and the Base Death Save Penalty.
  { id: 'woundTrack', column: 'vitals', order: 1, boxStyle: BOX },
  // The Role Ability: its Rank, then the values the Rank gives (Combat Awareness points, NET
  // Actions, ...), under the Role's name.
  { id: 'roleAbility', column: 'statBlock', order: 2, boxStyle: BOX },
  // Reputation Level, the Facedown (COOL + Reputation) and its roll with the die literal, then
  // every recorded Reputation event (the defining one marked).
  { id: 'reputation', column: 'statBlock', order: 3, boxStyle: BOX },
  // Improvement Points current / total.
  { id: 'improvementPoints', column: 'statBlock', order: 4, boxStyle: BOX },
  // The Lifepath answers, each under its table and column.
  { id: 'lifepath', column: 'statBlock', order: 5, boxStyle: BOX },
  // The Critical Injury checklist: both tables, the character's injuries ticked; ticking saves.
  { id: 'criticalInjuries', column: 'vitals', order: 2, boxStyle: BOX },
  // SP and Armor Penalty per location (head, body), the shield's HP and penalty, and the Armor
  // Penalty taken once on REF, DEX and MOVE.
  { id: 'armour', column: 'vitals', order: 3, boxStyle: BOX },
  // One row per weapon: damage dice (printed), ROF, Hands, Concealable, magazine, Skill Base.
  { id: 'weapons', column: 'vitals', order: 4, boxStyle: BOX },
  // Each weapon's single-shot range DV row (D-1) and Autofire row, under the range bands.
  { id: 'rangeDv', column: 'vitals', order: 5, boxStyle: BOX },
  // Each installed piece by name with its Humanity Loss, the total and maximum Humanity, the
  // cyberware SP and Initiative, and option slots used / total per container (D-11).
  { id: 'cyberware', column: 'vitals', order: 6, boxStyle: BOX },
  // A Netrunner's deck: NET Actions, slots used / available, the deck, its Programs and Hardware.
  { id: 'netrunning', column: 'vitals', order: 7, boxStyle: BOX },
  { id: 'other', column: 'vitals', order: 8, boxStyle: BOX },
];
