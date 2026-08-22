import { invoke } from '@tauri-apps/api/core';
import { formatError, hasTauriRuntime } from './runtime';

/**
 * Read-only desktop boundary over PF1's intelligent/legendary item build
 * system (SD-31 wave-18, `intelligent_items:desktop` lane).
 *
 * Invokes the `list_intelligent_item_catalog` Tauri command
 * (`intelligent_item_catalog.rs`), which reads every `equipmods` corpus
 * record under `data/corpus/*\/equipment/equipmods/*.json` whose `KEY:`
 * carries `"Intelligent Item"` — the classic `core_rulebook` system and the
 * parallel `mythic_adventures` Legendary Item system — and drops the 17
 * hidden `VISIBLE: NO` trigger rows a player never picks directly.
 *
 * An item's TOTAL Ego score is never served as a resolved number: which
 * components a given item carries is a build-time choice this corpus does
 * not fix, so every Ego-bearing mechanic ships as its own literal
 * contribution (or, for the shared Base row, the literal price-bracket
 * formula) rather than a fabricated sum. See the Rust module's own doc
 * comment for the full rationale.
 */
export interface IntelligentItemMechanicDto {
  /** The corpus `VAR` name verbatim, e.g. `"IntelligentItemEgo"`. */
  variable: string;
  /** Human label for `variable`, e.g. `"Ego"`, `"Intelligence"`. */
  effect: string;
  /**
   * The formula/value, already rendered readably — a signed integer
   * (`"+2"`), the Base row's price-bracket sentence, or a mechanically
   * simplified formula. Never a resolved character-specific number.
   */
  formula: string;
  /** The gating condition in plain prose, when the row states one. */
  condition: string | null;
  /** The bonus-stacking type tag (`"Purpose"`, `"Boolean"`), when stated. */
  bonusType: string | null;
}

export interface IntelligentItemComponentDto {
  /** Corpus book directory, e.g. `"core_rulebook"`, `"mythic_adventures"`. */
  book: string;
  /**
   * Grouping label derived from the `KEY:` token's own structure —
   * `"Base"`, `"Ability Score"`, `"Alignment"`, `"Communication"`,
   * `"Sense"`, `"Power"`, `"Purpose"`, `"Purpose Power"`, `"Movement"`,
   * `"Skill Ranks"`, `"Spellcasting"`.
   */
  family: string;
  key: string;
  name: string;
  costGp: number | null;
  /** Rendered from the record's `SPROP` token(s); `null` when none is real. */
  description: string | null;
  mechanics: IntelligentItemMechanicDto[];
  /**
   * Convenience read for the common case: `Some` only when this row states
   * exactly one bare-integer `IntelligentItemEgo` mechanic. `null` for the
   * shared Base row (whose Ego is a price-dependent formula, not a number)
   * and for any row that states no Ego contribution at all.
   */
  egoDelta: number | null;
}

export async function loadIntelligentItemCatalog(): Promise<IntelligentItemComponentDto[]> {
  if (!hasTauriRuntime()) {
    return [];
  }

  try {
    return await invoke<IntelligentItemComponentDto[]>('list_intelligent_item_catalog');
  } catch (cause: unknown) {
    throw new Error(`Failed to load intelligent item catalog: ${formatError(cause)}`);
  }
}
