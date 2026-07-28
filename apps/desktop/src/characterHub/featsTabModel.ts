import type { ItemPickerEntry } from './itemPickerFilter';

/**
 * Pure logic backing `FeatsTab`: resolving each raw string in
 * `selectedFeats` to its feat-catalog entry so the sheet can render a real
 * name + description instead of the raw internal selection string.
 *
 * `selectedFeats: string[]` genuinely mixes two real shapes today (traced
 * against source, not assumed):
 *  - the feat catalog's own `key` field, human-readable verbatim, e.g.
 *    `"Deflect Arrows"` (`feats.rs`'s doc comment: `key` equals `name` for
 *    every one of the 185 CRB records) -- what the Feats-tab "Add Feat"
 *    picker itself pushes (`CharacterSheet.tsx`'s `handleAddFeat` /
 *    `handleLevelUpFeatPick` both append `entry.key` straight from the
 *    catalog).
 *  - the rules engine's own lowercase, `feat:`-prefixed, snake_case
 *    selection token, e.g. `"feat:deflect_arrows"` -- what
 *    `pf1_adapter.rs`'s `compose_character_input` seeds a freshly created
 *    character with (`selected_feats: vec!["feat:power_attack",
 *    "feat:dodge", "feat:weapon_focus"]`) and what `pilot_compute.rs`'s
 *    feat-gate checks (e.g. line ~4781/4822 for Deflect Arrows) match
 *    against verbatim.
 * No normalization layer sits between the two anywhere in the backend --
 * `rule_system_adapter.rs` and `pf1_adapter.rs` both clone/append
 * `selected_feats` raw (confirmed by direct read, not inferred), so a real
 * saved character's list can hold either shape depending on how each feat
 * was added (initial creation vs. the in-sheet picker).
 */

export interface ResolvedFeatEntry {
  /** The exact string as it appears in `selectedFeats`, unchanged. */
  raw: string;
  /**
   * The matching catalog entry, or `null` when nothing in the catalog
   * resolves -- e.g. a non-CRB feat (today's catalog is CRB-only, see
   * `feat_catalog.rs`'s own doc comment) or a genuinely unrecognized
   * token. Callers must fall back to rendering `raw` rather than dropping
   * the row or showing a blank -- a silent gap is worse than an ugly one.
   */
  entry: ItemPickerEntry | null;
}

/**
 * Folds a raw `selectedFeats` token or a catalog `key` down to a
 * comparable identity: lowercase, alphanumeric characters only. This one
 * fold absorbs every real difference between the two known shapes:
 *  - a leading `"feat:"` prefix (present on engine tokens, absent on
 *    catalog keys);
 *  - a compound token carrying a sub-choice, e.g.
 *    `"feat:weapon_focus:weapon:longsword"` -- only the segment
 *    immediately after `"feat:"` identifies the feat itself, so segments
 *    after the second colon are dropped;
 *  - spaces in the catalog key vs. underscores in the engine token (both
 *    stripped);
 *  - punctuation the corpus keeps in the display name but the engine
 *    token drops, e.g. catalog `"Gorgon's Fist"` vs. token
 *    `"feat:gorgons_fist"` (both fold to `"gorgonsfist"`).
 */
function normalizeFeatIdentity(raw: string): string {
  const withoutPrefix = raw.startsWith('feat:') ? raw.slice('feat:'.length) : raw;
  const baseSegment = withoutPrefix.split(':')[0];
  return baseSegment.toLowerCase().replace(/[^a-z0-9]/g, '');
}

/**
 * Resolves every raw `selectedFeats` string against the loaded catalog.
 * Returns one row per input, in input order -- never fewer, so a caller
 * rendering this list can rely on it being parallel to `selectedFeats`.
 */
export function resolveSelectedFeatEntries(selectedFeats: string[], catalog: ItemPickerEntry[]): ResolvedFeatEntry[] {
  const byIdentity = new Map<string, ItemPickerEntry>();
  for (const entry of catalog) {
    byIdentity.set(normalizeFeatIdentity(entry.key), entry);
  }
  return selectedFeats.map((raw) => ({ raw, entry: byIdentity.get(normalizeFeatIdentity(raw)) ?? null }));
}
