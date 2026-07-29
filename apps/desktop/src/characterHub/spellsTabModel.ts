import { CLASS_OPTIONS } from './characterHubModel';
import type { SpellCatalogEntryDto } from '../boundary/loadSpellCatalog';
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
 */

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
  /** `null` rather than a fabricated value when nothing resolved. */
  level: number | null;
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
 * Resolves every persisted spell selection against the loaded catalog.
 * Returns one row per input, in input order — never fewer, so a caller can
 * rely on this list being parallel to `spellsSelected`.
 */
export function resolveSelectedSpellEntries(
  spellsSelected: SpellSelectionDto[],
  catalog: SpellCatalogEntryDto[]
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

  return spellsSelected.map((selection) => {
    const matched = byIdentity.get(normalizeSpellIdentity(selection.spellId)) ?? null;
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
 * The book + school + level line, e.g. `"CRB · Evocation · Level 1"`.
 *
 * Each part is included only when it is genuinely known, so a resolved APG
 * record whose corpus row carries no `SCHOOL:`/`CLASSES:` token reads
 * `"APG"` rather than claiming a school or level it does not have.
 * Returns `null` when nothing at all is known — an unresolved row must
 * make no claim rather than render a placeholder that looks like data.
 */
export function describeSpellSchoolAndLevel(entry: ResolvedSpellEntry): string | null {
  const parts: string[] = [];
  if (entry.book !== null) {
    parts.push(entry.book);
  }
  if (entry.school !== null) {
    parts.push(entry.school);
  }
  if (entry.level !== null) {
    parts.push(`Level ${entry.level}`);
  }
  return parts.length === 0 ? null : parts.join(' · ');
}
