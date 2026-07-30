import { CLASS_OPTIONS } from './characterHubModel';
import type { SpellCatalogEntryDto } from '../boundary/loadSpellCatalog';
import type { ClassSpellLevelsDto } from '../boundary/loadClassSpellLevels';
import type { SpellSelectionDto } from '../boundary/loadSavedCharacterDetail';
import type { AcquisitionModeDto } from '../boundary/addSpellSelection';

/**
 * Pure logic backing `SpellsTab`: resolving each persisted
 * `spellsSelected` entry to its real spell-catalog record so the sheet can
 * render a name, school, level and effect text instead of a bare internal
 * id.
 *
 * Why this exists (traced against source, not assumed): `SpellsTab`
 * previously rendered *only* `corpusDerived.schoolCoverage`, and
 * `load_saved_character` (`character_hub.rs`, the
 * `compute_pilot_with_corpus(&envelope.character_input,
 * corpus_fixture_bundle())` call) builds that section against
 * `corpus_fixtures.rs`'s bundled fixture set — whose `SPELL_FIXTURES` is
 * literally two files, `spell_abjuration.txt` and `spell_illusion.txt`.
 * Any other spell a character actually held resolved against nothing, fell
 * into `CorpusDerivedDto.unresolvedSpellIds`, and reached the player as a
 * raw id string in a "not shown (outside demo corpus)" line. All 652 real
 * records — with level and effect text — were present in
 * `rules_tables::crb::spell_list::SPELL_LIST` the whole time and already
 * served over the working `list_spells` command that the Add Spell picker
 * uses. Same defect shape as the feats one `featsTabModel.ts` fixed: the
 * data was ingested and correct everywhere except where a player looks.
 *
 * Per the operator's standing ruling, a spell's real description *is* a
 * legitimate deliverable — this tab computes no slots, DCs or
 * prepared/known posture, and does not pretend to.
 *
 * ## Spell level is per class, and this module resolves it per row
 *
 * A catalog record's own `level` is the MINIMUM spell level across every
 * class named in its corpus `CLASSES:` tag — it belongs to whichever class
 * gets the spell earliest, not to the character reading the sheet.
 * `Hideous Laughter` is `CLASSES:Bard=1|Sorcerer,Wizard=2`, so the record
 * carries 1 and every Wizard saw "Level 1" for a spell a Wizard learns at
 * 2. Re-derived from the shipped tables: **67 of the 580 spells on the
 * Wizard list** showed a wrong number that way, every one biased low; the
 * other casting classes have the same defect at their own rates (Druid
 * 19.2%, Cleric 15.6%, Bard 4.9%).
 *
 * **The multiclass rule: each row is resolved against its own persisted
 * `sourceClassId`.** Every `SpellSelectionDto` already records which held
 * class the spell was learned from, so there is nothing to arbitrate — a
 * Wizard 3 / Bard 2 who holds Hideous Laughter twice sees the Bard row at
 * 1 and the Wizard row at 2, both correct, with no "primary class"
 * tiebreak and no rule that could silently pick the wrong one. This is
 * deliberately NOT a per-character choice: the alternative (resolve every
 * spell against one class) would be a fresh way to show a wrong number to
 * the exact characters the fix is for.
 *
 * **Three outcomes, kept distinct** (`ResolvedSpellEntry.classLevelStatus`),
 * because they are different facts and read differently to a player:
 * the class's level is known; the class has a list and this spell is not
 * on it; or no list for that class exists here. The last case still shows
 * the record's own number — it is real data — but explicitly labelled as
 * the lowest class level rather than presented as this class's. Nothing
 * ever falls back to the record level wearing a class's name, which is
 * precisely the defect being removed (`docs/governance/no-stub-mvp-doctrine.md`).
 */

/**
 * Why a row does or does not carry a class-specific spell level.
 *
 * - `'class-level'` — the engine has a list for the row's source class and
 *   the spell is on it; `classLevel` is that class's real level.
 * - `'not-on-class-list'` — the engine has a list for that class and the
 *   spell is not on it, i.e. no member of that class can cast it at all.
 * - `'class-list-unknown'` — no list for that source class reached this
 *   model (never ingested, not requested, or not loaded yet). Knowable in
 *   principle for Magus/Summoner/Oracle, but not known here.
 */
export type SpellClassLevelStatus = 'class-level' | 'not-on-class-list' | 'class-list-unknown';

export interface ResolvedSpellEntry {
  /** The exact `spellId` as persisted, unchanged. */
  raw: string;
  /**
   * The catalog record's `key` when one resolved, else `raw` verbatim.
   * Callers render this directly — a selection is never dropped or blanked
   * just because the catalog has no row for it.
   */
  name: string;
  /** Which book the resolved record came from, or `null` when unresolved. */
  book: string | null;
  /** `null` rather than a fabricated value when nothing resolved. */
  school: string | null;
  /**
   * The catalog record's own level — the MINIMUM across every class named
   * in its corpus `CLASSES:` tag, so it is nobody's level in particular.
   * Kept verbatim for provenance; render `classLevel` instead wherever a
   * character's own class is in play.
   */
  level: number | null;
  /**
   * The spell's level for THIS row's `sourceClassId`. `null` whenever
   * `classLevelStatus` is not `'class-level'` — never back-filled from
   * `level`, which is the wrong number this field exists to replace.
   */
  classLevel: number | null;
  /** Which of the three per-class outcomes this row landed in. */
  classLevelStatus: SpellClassLevelStatus;
  /** The spell's real effect text, or `null` when nothing resolved. */
  effectText: string | null;
  /** Whether a real catalog record backed this row. */
  resolved: boolean;
  /** Persisted verbatim — how the character holds the spell. */
  acquisitionMode: AcquisitionModeDto;
  /** Persisted verbatim — which held class the spell was learned from. */
  sourceClassId: string;
}

/**
 * Folds a persisted `spellId` or a catalog `key` to a comparable identity:
 * lowercase, alphanumeric only.
 *
 * Exact-match is the primary path and the common case — `spell_resolver.rs`
 * matches `spell.name == spell_id` and `SPELL_LIST` by `entry.key ==
 * spell_id`, and the Add Spell picker pushes `entry.key` straight from the
 * catalog, so a spell added in-app round-trips byte-identical. The fold is
 * defensive only, covering apostrophe/case drift in ids that entered by
 * another route (e.g. an imported character, or `"mages disjunction"` for
 * `"Mage's Disjunction"`). It deliberately does NOT strip a prefix the way
 * `normalizeFeatIdentity` does: spell ids carry no `spell:` prefix anywhere
 * in the backend, so inventing one here would be guessing.
 */
function normalizeSpellIdentity(raw: string): string {
  return raw.toLowerCase().replace(/[^a-z0-9]/g, '');
}

/**
 * The distinct `sourceClassId`s these selections reference, sorted.
 *
 * This is exactly the argument `loadClassSpellLevels` should be called
 * with: a character never pulls a spell list it holds no spells from, and
 * sorting keeps the request stable across re-renders so the result can be
 * cached on it.
 */
export function spellSourceClassIds(spellsSelected: SpellSelectionDto[]): string[] {
  return [...new Set(spellsSelected.map((selection) => selection.sourceClassId))].sort();
}

/**
 * Resolves every persisted spell selection against the loaded catalog and
 * the per-class spell levels. Returns one row per input, in input order —
 * never fewer, so a caller can rely on this list being parallel to
 * `spellsSelected`.
 *
 * `classSpellLevels` is the `classes` array of a `list_class_spell_levels`
 * response. An empty array — a failed or not-yet-finished load — leaves
 * every row `'class-list-unknown'`, which renders as a labelled record
 * level rather than a class level nobody verified.
 */
export function resolveSelectedSpellEntries(
  spellsSelected: SpellSelectionDto[],
  catalog: SpellCatalogEntryDto[],
  classSpellLevels: ClassSpellLevelsDto[] = []
): ResolvedSpellEntry[] {
  const byIdentity = new Map<string, SpellCatalogEntryDto>();
  for (const entry of catalog) {
    // First writer wins, so an exact-key record is never displaced by a
    // later record that merely folds to the same identity.
    const identity = normalizeSpellIdentity(entry.key);
    if (!byIdentity.has(identity)) {
      byIdentity.set(identity, entry);
    }
  }

  // Folded on the same identity as the catalog, so a stored id that needs
  // the fold to reach its catalog record reaches its class level too.
  const levelsByClass = new Map<string, Map<string, number>>();
  for (const classEntry of classSpellLevels) {
    if (!classEntry.known) {
      // A class the engine cannot answer for is deliberately absent from
      // this map — it must read as unknown, not as an empty list, which
      // would wrongly claim the class casts nothing.
      continue;
    }
    const byKey = new Map<string, number>();
    for (const entry of classEntry.entries) {
      const identity = normalizeSpellIdentity(entry.key);
      if (!byKey.has(identity)) {
        byKey.set(identity, entry.level);
      }
    }
    levelsByClass.set(classEntry.classId, byKey);
  }

  return spellsSelected.map((selection) => {
    const identity = normalizeSpellIdentity(selection.spellId);
    const matched = byIdentity.get(identity) ?? null;
    const classList = levelsByClass.get(selection.sourceClassId) ?? null;

    let classLevel: number | null = null;
    let classLevelStatus: SpellClassLevelStatus = 'class-list-unknown';
    if (classList !== null && matched !== null) {
      // Look up under the catalog record's canonical key as well as the
      // stored id's own fold, so either spelling reaches the same answer.
      const found = classList.get(normalizeSpellIdentity(matched.key)) ?? classList.get(identity);
      if (found === undefined) {
        // The class has a real list and this spell is not on it — a fact,
        // not a gap. Saying so beats showing a level that is not theirs.
        classLevelStatus = 'not-on-class-list';
      } else {
        classLevel = found;
        classLevelStatus = 'class-level';
      }
    }
    // An unresolved row (matched === null) stays 'class-list-unknown':
    // with no catalog record behind it, the row makes no claims at all.

    return {
      raw: selection.spellId,
      name: matched ? matched.key : selection.spellId,
      book: matched?.book ?? null,
      // These stay `null` both when nothing resolved AND when the resolved
      // record genuinely lacks the field (a real `apg_spells.lst` gap —
      // see `SpellCatalogEntryDto`). Callers render the absence either way
      // rather than substituting a plausible-looking value.
      school: matched?.school ?? null,
      level: matched?.level ?? null,
      classLevel,
      classLevelStatus,
      effectText: matched?.description ?? null,
      resolved: matched !== null,
      acquisitionMode: selection.acquisitionMode,
      sourceClassId: selection.sourceClassId,
    };
  });
}

/**
 * Human-readable label for a `class:foo` id, via the same `CLASS_OPTIONS`
 * table the rest of the hub uses, falling back to a title-cased derivation
 * of the id itself (mirroring `characterProgression.ts`'s `parseOneClass`)
 * rather than showing the raw token or inventing a name.
 */
function describeSourceClass(classId: string): string {
  const option = CLASS_OPTIONS.find((entry) => entry.id === classId);
  if (option) {
    return option.label;
  }
  const derived = classId
    .split(':')
    .slice(1)
    .join(' ')
    .replace(/\b\w/g, (character) => character.toUpperCase());
  return derived || classId;
}

/** How one spell row's provenance line reads, e.g. `"Prepared · Wizard"`. */
export function describeSpellAcquisition(entry: ResolvedSpellEntry): string {
  return `${entry.acquisitionMode} · ${describeSourceClass(entry.sourceClassId)}`;
}

/**
 * The level clause of the detail line, or `null` when no honest one can be
 * written. Never states a bare "Level N" — every number it emits is
 * qualified by whose level it is.
 *
 * - class level known -> `"Wizard level 2"`
 * - spell off that class's list -> `"Not on the Bard spell list"`
 * - no list for that class -> `"Lowest class level 1"`, the record's own
 *   number under a label naming what it actually measures (the minimum
 *   across every class on the record), or `null` if the record has none.
 */
function describeSpellLevelClause(entry: ResolvedSpellEntry): string | null {
  const className = describeSourceClass(entry.sourceClassId);
  switch (entry.classLevelStatus) {
    case 'class-level':
      return entry.classLevel === null ? null : `${className} level ${entry.classLevel}`;
    case 'not-on-class-list':
      return `Not on the ${className} spell list`;
    case 'class-list-unknown':
      return entry.level === null ? null : `Lowest class level ${entry.level}`;
  }
}

/**
 * The book + school + level line, e.g.
 * `"CRB · Enchantment · Wizard level 2"`.
 *
 * Each part is included only when it is genuinely known, so a resolved APG
 * record whose corpus row carries no `SCHOOL:`/`CLASSES:` token reads
 * `"APG"` rather than claiming a school or level it does not have.
 * Returns `null` when nothing at all is known — an unresolved row must
 * make no claim rather than render a placeholder that looks like data.
 *
 * The level clause is always attributed (see `describeSpellLevelClause`).
 * The bare `"Level 1"` this line once carried was the record's minimum
 * across classes presented as if it were the reader's own — the defect
 * this model now fixes.
 */
export function describeSpellSchoolAndLevel(entry: ResolvedSpellEntry): string | null {
  const parts: string[] = [];
  if (entry.book !== null) {
    parts.push(entry.book);
  }
  if (entry.school !== null) {
    parts.push(entry.school);
  }
  const levelClause = describeSpellLevelClause(entry);
  if (levelClause !== null) {
    parts.push(levelClause);
  }
  return parts.length === 0 ? null : parts.join(' · ');
}
