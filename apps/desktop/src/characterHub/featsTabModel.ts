import type { ChosenFeatTargetsDto } from '../boundary/loadSavedCharacterDetail';
import type { ItemPickerEntry } from './itemPickerFilter';

/**
 * Pure logic backing `FeatsTab`: resolving each raw string in
 * `selectedFeats` to its feat-catalog entry so the sheet can render a real
 * name + description instead of the raw internal selection string.
 *
 * `selectedFeats: string[]` genuinely mixes two real shapes today (traced
 * against source, not assumed):
 *  - the feat catalog's own `key` field, human-readable verbatim, e.g.
 *    `"Deflect Arrows"` (`key` equals `name` for every one of the 185 CRB
 *    records and all 129 ACG records; 6 APG records carry a distinct
 *    corpus `KEY:`, e.g. `"Elemental Spell ~ Acid"` displaying as
 *    `"Elemental Spell (Acid)"`) -- what the Feats-tab "Add Feat" picker
 *    itself pushes (`CharacterSheet.tsx`'s `handleAddFeat` /
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
   * resolves -- e.g. a feat from a book the engine has not ingested (the
   * catalog spans CRB, APG and ACG today; see `feat_catalog.rs`'s own doc
   * comment) or a genuinely unrecognized token. Callers must fall back to
   * rendering `raw` rather than dropping the row or showing a blank -- a
   * silent gap is worse than an ugly one.
   */
  entry: ItemPickerEntry | null;
  /**
   * What this feat's chosen target names -- `'Weapon'`, `'Skill'`,
   * `'SpellSchool'` -- or `null` for a feat that takes no target.
   *
   * Comes from the backend's resolved `chosenFeatTargets`, never inferred
   * from the feat's name here: which feats take a target is rules
   * knowledge, and the engine is the only thing that knows which targets it
   * actually consumes.
   */
  targetKind: string | null;
  /**
   * The targets recorded for this feat, already prefix-stripped by the
   * backend. Empty for a feat that takes no target AND for a chooser feat
   * held without one -- `targetKind` distinguishes those two cases, and
   * callers must render the second as "no target chosen" rather than
   * silently as though the feat were complete.
   */
  targets: string[];
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
export function resolveSelectedFeatEntries(
  selectedFeats: string[],
  catalog: ItemPickerEntry[],
  chosenFeatTargets: ChosenFeatTargetsDto[] = []
): ResolvedFeatEntry[] {
  const byIdentity = new Map<string, ItemPickerEntry>();
  for (const entry of catalog) {
    byIdentity.set(normalizeFeatIdentity(entry.key), entry);
  }
  // Joined on the same identity fold as the catalog, because the backend
  // reports `featId` verbatim from `selectedFeats` -- so it arrives in
  // whichever of the two shapes that character happened to record, exactly
  // like the raw strings themselves.
  const targetsByIdentity = new Map<string, ChosenFeatTargetsDto>();
  for (const resolved of chosenFeatTargets) {
    targetsByIdentity.set(normalizeFeatIdentity(resolved.featId), resolved);
  }
  return selectedFeats.map((raw) => {
    const identity = normalizeFeatIdentity(raw);
    const resolvedTargets = targetsByIdentity.get(identity) ?? null;
    return {
      raw,
      entry: byIdentity.get(identity) ?? null,
      targetKind: resolvedTargets?.targetKind ?? null,
      targets: resolvedTargets?.targets ?? [],
    };
  });
}

/**
 * How one feat row's target should read, or `null` when the feat takes no
 * target and the row should say nothing about targets at all.
 *
 * Kept here rather than in the component so it stays testable -- this repo
 * has no DOM test harness, so anything asserted about feat rendering has to
 * live in this module.
 */
export function describeFeatTarget(entry: ResolvedFeatEntry): string | null {
  if (entry.targetKind === null) {
    return null;
  }
  if (entry.targets.length === 0) {
    return `No ${targetNoun(entry.targetKind)} chosen yet — this feat has no effect until one is`;
  }
  return entry.targets.join(', ');
}

function targetNoun(targetKind: string): string {
  switch (targetKind) {
    case 'Weapon':
      return 'weapon';
    case 'Skill':
      return 'skill';
    case 'SpellSchool':
      return 'school';
    default:
      return 'target';
  }
}

/**
 * The `chosenFeatTargets` list as it will look after a just-applied feat
 * add, mirroring exactly what the backend recorded.
 *
 * Optimistic only in timing, never in content: with no target it returns
 * the list unchanged, so adding a chooser feat without naming a target
 * never invents one. With a target it appends to that feat's existing entry
 * (a repeatable feat naming a second weapon) or adds a new entry.
 *
 * Needed because `add_feat_selection`'s response is a create-character
 * outcome that carries no feat-target field — the same reason
 * `selectedFeats` is already passed explicitly through
 * `toCharacterMutationRefresh`.
 */
export function mergeChosenFeatTarget(
  existing: ChosenFeatTargetsDto[],
  featId: string,
  target: string | null,
  targetKind: string | null
): ChosenFeatTargetsDto[] {
  if (target === null || targetKind === null) {
    return existing;
  }
  const identity = normalizeFeatIdentity(featId);
  const index = existing.findIndex((entry) => normalizeFeatIdentity(entry.featId) === identity);
  if (index === -1) {
    return [...existing, { featId, targetKind, targets: [target] }];
  }
  const matched = existing[index];
  if (matched.targets.some((name) => name.toLowerCase() === target.toLowerCase())) {
    return existing;
  }
  const updated = [...existing];
  updated[index] = { ...matched, targets: [...matched.targets, target] };
  return updated;
}
