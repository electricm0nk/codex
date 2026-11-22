import type { CreateCharacterRequest } from '../../boundary/loadCreateCharacter';
import type { DraftSheetPreviewDto } from '../../boundary/draftSheetPreview';
import type { SquareBlockProps } from './SquareBlock';

/**
 * Create's copy of the sheet's 2x2 block (Brief 2 L5). Initiative and Encumbrance are the engine's
 * own numbers for the unsaved draft (`draft_sheet_preview`); Hit Points is the Levels list total;
 * Conditions is the draft's custom record. A preview that is loading, impossible (no level yet) or
 * refused says so in the boxes instead of showing an em dash or a made-up number.
 */
export type DraftPreviewState =
  | { kind: 'noLevels' }
  | { kind: 'loading' }
  | { kind: 'ready'; preview: DraftSheetPreviewDto }
  | { kind: 'error'; message: string };

export const NO_LEVELS_REASON = 'Add a level first: the engine computes this from the class levels.';
export const LOADING_REASON = 'Loading the engine preview…';

export function squareBlockForDraft(state: DraftPreviewState, hp: number | null, conditions: readonly string[]): SquareBlockProps {
  switch (state.kind) {
    case 'ready':
      return { hp, initiative: state.preview.initiative, encumbrance: state.preview.encumbrance, conditions };
    case 'error':
      return { hp, initiative: null, encumbrance: undefined, conditions, absent: state.message };
    case 'loading':
      return { hp, initiative: null, encumbrance: undefined, conditions, absent: LOADING_REASON };
    case 'noLevels':
      return { hp, initiative: null, encumbrance: undefined, conditions, absent: NO_LEVELS_REASON };
  }
}

/**
 * What the preview depends on: the whole draft request except the fresh id, the timestamp and the
 * name, so a change of levels, abilities, race, traits, feats or equipment recomputes and a
 * re-render does not. `null` when there is no draft (no level yet).
 */
export function draftPreviewKey(draft: CreateCharacterRequest | null): string | null {
  if (draft === null) {
    return null;
  }
  const { characterId: _id, savedAt: _at, displayLabel: _name, ...rest } = draft;
  return JSON.stringify(rest);
}
