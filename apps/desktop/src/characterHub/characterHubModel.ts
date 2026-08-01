/**
 * Data-driven catalogues for the Character Hub picker.
 *
 * These are structured as arrays/lookups (not hardcoded UI branches) so
 * later phases can widen them (more races, more game systems) without a
 * component redesign.
 */

export const ABILITY_KEYS = ['strength', 'dexterity', 'constitution', 'intelligence', 'wisdom', 'charisma'] as const;
export type AbilityKey = (typeof ABILITY_KEYS)[number];

export const ABILITY_ABBREVIATIONS: Record<AbilityKey, string> = {
  strength: 'STR',
  dexterity: 'DEX',
  constitution: 'CON',
  intelligence: 'INT',
  wisdom: 'WIS',
  charisma: 'CHA',
};

export type Sex = 'male' | 'female';

/**
 * Random height/weight profile for one sex of a race (approximate PF1 Core
 * values). Height is `baseHeightInches` plus a roll of `heightModDice`; weight
 * is `baseWeightLb` plus a roll of the same dice times `weightMultiplierLb`.
 */
export interface BodyProfile {
  baseHeightInches: number;
  heightModDice: { count: number; sides: number };
  baseWeightLb: number;
  weightMultiplierLb: number;
}

/**
 * One race the creation form offers.
 *
 * Built from the backend's corpus-derived roster by
 * `raceRoster.raceOptionsFromChassis` — **not** declared as a constant here.
 * A seven-entry `RACE_OPTIONS` table used to live at this spot; it offered
 * the Core Rulebook races only, while the corpus carried 18. See
 * `raceRoster.ts` for the full account.
 */
export interface RaceOption {
  /** The `race:<slug>` token submitted with the character. */
  id: string;
  label: string;
  /** Short sourcebook code, e.g. `CRB`, `B1`. */
  book: string;
  /** Fixed PF1 racial ability modifiers applied to every member of the race. */
  abilityAdjustments: Partial<Record<AbilityKey, number>>;
  /**
   * Floating ability points the player distributes freely (PF1's "+2 to one
   * ability" races: Human, Half-Elf, Half-Orc). `0` means the race has no
   * player-selectable enhancement.
   */
  floatingBonusPoints: number;
  /** `Small` or `Medium` — a string, not a union, because the roster is data. */
  size: string;
  vision: string;
  /** Base land speed in feet. */
  baseSpeedFt: number;
  /**
   * Random height/weight profile, or `null` for a race this repo carries no
   * profile for. Height and weight are display-only in the form (neither is
   * part of `CreateCharacterRequest`), and the corpus carries them for no
   * race at all — see `RACE_BODY_PROFILES`.
   */
  body: Record<Sex, BodyProfile> | null;
}

/**
 * What the sheet prints for a race-derived field it has no profile for.
 *
 * Not a PF1 value, and deliberately not a plausible one. The Character
 * Sheet's Details panel tells the player "Vision and Size are calculated
 * from race", so anything printed there is read as a derived rules fact. A
 * saved character whose `raceId` is not in the roster the caller supplies —
 * a sheet written by a later build, or any character opened before the
 * roster has arrived from the backend — must not be handed
 * "Medium"/"Normal"/"30 ft." as though they had been calculated.
 */
export const UNKNOWN_RACE_TRAIT = 'Unknown';

export interface RaceDerivedTraits {
  size: string;
  vision: string;
  /** Base land speed, already formatted (`"30 ft."`), or `UNKNOWN_RACE_TRAIT`. */
  landSpeed: string;
}

/**
 * Size, vision and land speed for a saved character's `raceId`, read off the
 * roster the backend served, or `UNKNOWN_RACE_TRAIT` for each when that
 * roster carries no entry for the race. Never guesses.
 *
 * `roster` is a parameter rather than a module constant because the roster is
 * corpus-derived and arrives asynchronously (`raceRoster.ts`). An empty
 * roster — still loading, or no desktop backend — therefore reads as "not
 * known", which is the truth at that moment.
 */
export function deriveRaceTraits(
  raceId: string | null | undefined,
  roster: RaceOption[]
): RaceDerivedTraits {
  const race = roster.find((option) => option.id === raceId);
  if (!race) {
    return { size: UNKNOWN_RACE_TRAIT, vision: UNKNOWN_RACE_TRAIT, landSpeed: UNKNOWN_RACE_TRAIT };
  }
  return { size: race.size, vision: race.vision, landSpeed: `${race.baseSpeedFt} ft.` };
}

/**
 * `full` — reaches `Computed` for any race the creation roster offers.
 * `partial-human-only` — reaches `Computed` for `race:human` only; every
 * other race falls back to the same 4 generic diagnostics as a `none`
 * class.
 * `human-diagnostics-only` — never reaches `Computed` for any race,
 * including Human — the compute engine's named class seam only grounds
 * explanation/diagnostic *text* for `race:human` (e.g. naming the specific
 * missing rage-execution or spellcasting burden), not an actual computed
 * build. Every other race falls back to the same 4 generic diagnostics as
 * a `none` class. Do not present this as "partial support" — Human never
 * produces a savable build here either, it only explains why in more
 * detail.
 * `full-except-human-level-1` — the inverse of `partial-human-only`:
 * reaches `Computed` for every race at every level offered here, EXCEPT a
 * single-class Human at level 1 specifically, which stays claim-blocked
 * (`explain_hybrid_level1_chassis` in `pilot_compute.rs` unconditionally
 * names the still-missing non-spell class-feature burden and later spell
 * burden for a single-class Human at exactly level 1 — a historical
 * boundary from the class's original hybrid-baseline slice, not touched by
 * the later per-pillar/spell-posture work that unblocked every other
 * race/level combination). Human reaches the identical computed build from
 * level 2 onward. Live-verified: a fresh Human at level 1 stays `Blocked`
 * with the named hybrid diagnostics; a fresh Elf at level 1 reaches
 * `Computed`/`Saved`, and leveling that same character up through level 4
 * (past the point spells first become accessible) stays `Computed` at
 * every step with no new blocker.
 * `none` — no dedicated compute seam exists; every race produces the same 4
 * generic diagnostics.
 * `headless-only` — the compute engine has a real `Computed` path for this
 * class (given a specific build-time choice: e.g. a bloodline, a domain, a
 * nature bond), but no picker for that choice exists anywhere in the real
 * creation UI today — `CreateCharacterForm`'s create-character request has
 * no field for it, and no default is seeded on the backend either. Every
 * live creation attempt stays `Blocked`, for any race (the gate is
 * race-independent, unlike `human-diagnostics-only`'s Human-gets-nicer-
 * diagnostics split), carrying the same named "missing choice" diagnostic
 * regardless of race. This is a UI/wire-contract gap, not an engine gap —
 * distinguishing it from `human-diagnostics-only` matters because the fix
 * is "add a picker + a request field," not "compute more of the class."
 */
export type ClassSupportLevel =
  | 'full'
  | 'partial-human-only'
  | 'human-diagnostics-only'
  | 'full-except-human-level-1'
  | 'headless-only'
  | 'none';

export interface ClassOption {
  id: string;
  label: string;
  supportLevel: ClassSupportLevel;
  levelOptions: number[];
  /** PF1 hit die size (e.g. 10 for a d10). Level-1 HP is the max of this plus the CON modifier. */
  hitDie: number;
}

/**
 * PF1's own class ceiling, and the exact ceiling
 * `src/bin/v06_class_state_dump.rs` sweeps to (`MAX_LEVEL: u8 = 20`) — the
 * two must agree, or `levelOptions` would be claiming a level the engine
 * dump never checked.
 */
export const MAX_CLASS_LEVEL = 20;

/**
 * Every level 1-20, shared by each class the engine dump reports `Computed`
 * at all of them. A shared frozen array rather than 11 copies: these entries
 * are not independent judgements that happen to coincide, they are one fact
 * ("the dump says all 20 levels compute") read off in eleven places.
 */
const EVERY_CLASS_LEVEL: number[] = Object.freeze(
  Array.from({ length: MAX_CLASS_LEVEL }, (_unused, index) => index + 1)
) as number[];

/**
 * The classes this app offers: all 27 the engine can build — the eleven PF1
 * Core Rulebook classes, APG's six (Alchemist, Cavalier, Inquisitor, Oracle,
 * Summoner, Witch) and ACG's ten (Arcanist, Bloodrager, Brawler, Hunter,
 * Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest). This is
 * still deliberately not "every class in every book": a class belongs here
 * only when `v06_class_state_dump` gives it real levels to offer. As of the
 * 2026-07-29 sweep that happens to be every class the dump knows about, so
 * the list and the dump's roster coincide — but they coincide as a result,
 * not by construction. A class added to the engine but not yet computing
 * must not be added here.
 *
 * `supportLevel` reflects the compute engine's real gating, not just
 * whether it recognizes the class — verified directly against
 * `pilot_compute.rs`'s per-class `explain_*` functions (each of Monk and
 * the other still-`human-diagnostics-only` classes carries its own "This
 * deliberately does not compute a supported ... chassis/surface" doc
 * comment, and the compute path stays claim-blocked for Human exactly as
 * it does for every other race) and live-verified for Barbarian
 * specifically back when it was in this same bucket (a fresh Human
 * Barbarian creation attempt returned `Blocked` with named rage-burden
 * diagnostics instead of the 4 generic ones) — since superseded now that
 * Barbarian's rage-execution engine is real; see its own note below.
 * Cleric and Druid have since moved out of this bucket entirely (see the
 * `full` note below) once their own domain-powers/animal-companion
 * burdens stopped being *permanently* unconditional.
 *
 * Paladin and Ranger are now plain `full`. They were
 * `full-except-human-level-1` from 2026-07-25 (class-breadth epic:
 * `b7642d97` Ranger, `ee3c50ce` Paladin gave both a genuinely computed
 * spell posture), because both still shared
 * `explain_hybrid_level1_chassis`'s single-class-Human-at-level-1 gate —
 * that one race/level combination stayed `Blocked` while every other one
 * computed. **That gate is gone** (corrected 2026-07-29): the level sweep
 * in `cargo run --bin v06_class_state_dump` runs on the Human fixture and
 * reports both classes `Computed` at every level 1-20, level 1 included.
 * Live-verified through the real dev build, not inferred from the dump: a
 * fresh Human Paladin 1 and a fresh Human Ranger 1 each reached
 * `Computed`/`Saved` with real distinct stat blocks (Paladin BAB +1 /
 * Fort +4 / Will +3; Ranger BAB +1 / Fort +4 / Ref +4), and both also
 * computed and saved at level 20. The `full-except-human-level-1` support
 * level itself is deliberately kept in `ClassSupportLevel` — no class
 * currently uses it, but it describes a real shape the engine can produce
 * again, and its copy is still covered by this file's tests.
 *
 * Wizard and Rogue are `full`, not `partial-human-only` as this file
 * previously (incorrectly) had them: `supported_wizard_level` /
 * `supported_rogue_level` and every gate downstream of them
 * (`wizard_has_canonical_specialization_selections`,
 * `unmet_wizard_spellbook_conditions`, `compose_character_input`'s choice
 * seeding) check only `class_id` and level, never `race_id` — confirmed via
 * `git log -p` on the seeding block back to its original commit (3484b5d4),
 * which never had a Human condition. Live-verified end-to-end via the real
 * creation UI: a fresh Elf Wizard 1 and a fresh Elf Rogue 1 both reached
 * `Computed`/`Saved` with real, distinct stat blocks, disk-confirmed
 * (`race_id=race:elf` alongside a real `class_level` entry). The "Human
 * only" framing was a stale/unverified assumption baked in when the Wizard
 * spellbook fix first landed — the doc comments on the compute-path
 * functions reference "Human Wizard"/"Human Rogue" descriptively (naming
 * the deterministic baseline that was built and tested first), not as an
 * enforced restriction.
 *
 * Barbarian is also `full` (v0.6 alpha swarm, combat-time activation-state
 * epic, 2026-07-25): unlike Paladin/Ranger, its remaining gap
 * (`ground_or_block_barbarian_rage`) is keyed only on class-ownership, not
 * on `hybrid_level1_class`'s shared level-1/Human gate — Barbarian never
 * appears in that function's match arms at all. Not raging is itself a
 * genuinely valid PF1 posture (an honest recognition record, no
 * diagnostic), so there is no race/level carve-out. Live-verified: a fresh
 * Human Barbarian 1 (the default, not-raging posture) and a fresh Dwarf
 * Barbarian 1 both reached `Computed`/`Saved`; leveling the Dwarf character
 * up through the real `LevelUpDialog` reached level 2 cleanly, disk-
 * confirmed (`class_level=class:barbarian:2`).
 *
 * Bard is also `full` (v0.6 alpha swarm, class-breadth epic, 2026-07-25):
 * its known-spell posture (`unmet_bard_known_spell_conditions`) works
 * purely off `spells_selected`, same mechanism the other spontaneous/
 * prepared casters already use — zero known spells is honestly valid, same
 * "empty is valid" shape as Ranger's prepared-spell posture. No special
 * choice gate, so no picker gap either. Live-verified: a fresh Human Bard 1
 * (default settings, no spells picked) reached `Computed`/`Saved`,
 * disk-confirmed; leveling that character up through the real
 * `LevelUpDialog` reached level 2 cleanly.
 *
 * Sorcerer, Cleric, and Druid moved from `headless-only` to `full`
 * (v0.6 alpha swarm, choice-picker Path A closure, `9bafe303`,
 * 2026-07-25): per `docs/release/v0.6/choice-picker-ui-gap-scoping.md`,
 * `compose_character_input` now silently seeds each class's own canonical
 * choice (Sorcerer: Arcane bloodline + a familiar Arcane Bond; Cleric:
 * Good domain; Druid: an animal-companion nature bond, which resolves to
 * Wolf automatically), mirroring Wizard's own pre-existing school-
 * specialization default — a real, working choice, but not yet a player-
 * facing pick (that picker is Path B, deliberately deferred, tracked in
 * the same scoping doc).
 *
 * None of the three share Paladin/Ranger's hybrid-level-1-Human gate —
 * checked directly, not assumed: none of the three appear in
 * `hybrid_level1_class`'s match arms, and each of their own
 * bloodline/domain/nature-bond checks is explicitly coded and documented
 * as race-independent, evaluated before any Human-only gate.
 *
 * Arcanist is `full` too, and was the first non-CRB class offered here
 * (v0.6 alpha swarm, class-picker breadth, 2026-07-29). Its Path A gap
 * closed the same way Wizard's did: `compose_character_input`
 * (`pf1_adapter.rs`) seeds the canonical Metamagic Knowledge exploit plus
 * a starter spellbook on the real creation path, and does the same on the
 * multiclass-dip path. The backend has accepted `class:arcanist` since
 * then; this picker simply never listed it, so no player could reach it.
 *
 * The remaining fifteen APG/ACG classes were added the same day, for the
 * same reason and on the same evidence: every one of them already reached
 * `Computed` at all 20 levels in the engine dump, and every one was
 * unreachable purely because this list did not name it. Ten of the fifteen
 * need a canonical Path A seed on the real creation path (Alchemist,
 * Cavalier, Inquisitor, Investigator, Oracle, Shaman, Summoner, Warpriest,
 * Witch, Bloodrager) and `compose_character_input` already applies each —
 * checked constant-by-constant against the dump's own `canonical_seeds_for`
 * rather than assumed, since the two tables are separate hand-maintained
 * mirrors in different crates and nothing enforces that they agree. The
 * other five (Brawler, Hunter, Skald, Slayer, Swashbuckler) need no seed at
 * all — they reach `Computed` from the bare chassis.
 *
 * `hitDie` for all fifteen comes from each class's own `HD:` token on its
 * real `CLASS:` record (`apg_classes.lst` / `acg_classes.lst`), cross-checked
 * against the engine's own per-class `HIT_DIE` constants
 * (`rules_tables/apg/class_*.rs`, `rules_tables/acg/class_*.rs`) — the two
 * agree on all fifteen. Note the engine's CRB `CLASS_META` table carries no
 * row for any non-CRB class, so `crb::class_tables::hit_die_for` is not the
 * cross-check for these; `apg::hit_die_for`/`acg::hit_die_for` are.
 *
 * That same cross-check turned up one genuine disagreement, in a CRB class
 * this change did not otherwise touch: **Monk**. **RESOLVED 2026-07-29 in
 * this table's favour, by operator ruling** (risks item 91). This table
 * said `8` while `cr_classes.lst`'s `CLASS:Monk` record carries `HD:10` and
 * `CLASS_META` mirrored that `10` — so the shipped product displayed d8
 * while computing HP from d10 (a Monk 20 is 143 HP at d8 versus 164 at
 * d10). The operator ruled the published PF1 CRB p.56 value, d8, is
 * correct; `CLASS_META`'s Monk row was corrected to `8` and now carries a
 * documented corpus-defect override comment explaining why it deliberately
 * does not transcribe its own `HD:` token. The corpus itself was
 * intentionally NOT edited — it is this project's independent parity
 * oracle. So the `8` below needed no change; it is now the engine that
 * agrees with it, not the other way round.
 *
 * ## Where `levelOptions` comes from
 *
 * `levelOptions` used to lag well behind the engine by convention — each
 * entry only listed the levels somebody had personally driven through the
 * running app, so Wizard/Rogue/Druid sat at `[1]` and Sorcerer at `[1, 2]`
 * long after the engine computed them at all 20. That convention has been
 * replaced with a checkable one: **`levelOptions` is exactly what
 * `cargo run --bin v06_class_state_dump` reports as `Computed`**, spot-
 * checked live rather than exhaustively re-driven.
 *
 * That dump is not a summary of this file — it sweeps every class over
 * levels 1-20 through the real `build_pilot_headless_receipt` pipeline,
 * under the exact input posture `compose_character_input` composes for a
 * freshly created character (same fixture, same canonical choice/spell
 * seeds). Its 2026-07-29 run reports **all 27 classes `Computed` at every
 * level 1-20** (`computed_count: 27`, every row `levels_blocked: []`), so
 * every entry above offers the full 1-20 range and none is capped.
 *
 * Monk's earlier `[1]` cap is gone with this change. It was never a claim
 * that Monk broke past level 1 — the dump already reported it `Computed` at
 * all 20 — only that nobody had driven it live, so the cap was held
 * deliberately rather than widened on dump evidence alone. That live pass
 * has now happened (see below), so the exception is retired.
 *
 * Re-run the dump before editing any `levelOptions` value; do not widen a
 * class the dump reports `Blocked`, and do not leave a `Computed` class
 * artificially capped.
 *
 * Live-verified 2026-07-29 through the real dev build at the new ceiling,
 * rather than at every one of the 220 class/level pairs this opens: all
 * eleven classes were created at level **20** and each reached
 * `Computed`/`Saved` with real, class-distinct stat blocks, disk-confirmed
 * (`class_level=class:<name>:20` in each `authoritative_character_input`).
 * Level 1 was driven for Human Paladin/Ranger specifically (the old
 * carve-out) and for Wizard. The `LevelUpDialog` path was driven
 * separately, because creating at a level and leveling up to it are
 * different code paths: a Wizard created at level 1 leveled cleanly to 2
 * and then 3 (the level-3 feat grant correctly routed through the real
 * feat picker first), disk-confirmed at `class:wizard:3`.
 *
 * Intermediate levels (2-19) are covered by the engine dump but were not
 * each driven by hand. If one of them ever turns out to break the running
 * UI while computing headlessly, the fix is to carve that level out of
 * this list and say so here — not to widen quietly.
 *
 * ## The 2026-07-29 breadth pass (Monk + the fifteen APG/ACG classes)
 *
 * Every class added or widened in that change was created through the real
 * running app, not just the dump. All sixteen (Monk plus the fifteen) were
 * created at level **20** and each reached `Computed`/`Saved` with a real,
 * class-distinct stat block — distinct in the way the rules require, not
 * merely non-identical: Alchemist came out Fort/Ref-good, Summoner
 * Will-only, Warpriest Fort/Will, Monk good in all three, each with the
 * right 3/4- or full-BAB progression. All sixteen are disk-confirmed
 * (`class_level=class:<name>:20` in each `authoritative_character_input`).
 * Level **1** was driven for Slayer and level **10** for Witch, so the
 * bottom and middle of the range are covered too, not only the ceiling.
 *
 * `LevelUpDialog` was driven separately, because creating at a level and
 * levelling up to it are different code paths: the Witch created at 10 was
 * levelled to 11 through the real dialog (which previewed "Hit die: d6",
 * this table's own Witch value, reached through a different code path than
 * the creation-form HP preview), the level-11 feat grant correctly routed
 * through the real feat picker first, and the result persisted —
 * disk-confirmed at `class:witch:11`, HP 62 → 68 and skill points 24 → 27.
 *
 * Character sheets were opened for Warpriest 20, Witch 10/11 and Hunter 20
 * and render with no blank panels and no `undefined`: the Warpriest's
 * Spells tab shows its seeded `Light` as both Known and Prepared under its
 * own source class (which is what proves `compose_character_input`'s seed
 * fires on the real creation path and not only in the dump), and the
 * Hunter's Pets tab renders a fully grounded 16-HD Wolf companion. The
 * other thirteen classes were created and saved but their sheets were not
 * individually opened; only Human was driven, and only the levels named
 * above.
 */
export const CLASS_OPTIONS: ClassOption[] = [
  { id: 'class:fighter', label: 'Fighter', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 10 },
  { id: 'class:paladin', label: 'Paladin', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 10 },
  { id: 'class:ranger', label: 'Ranger', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 10 },
  { id: 'class:sorcerer', label: 'Sorcerer', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 6 },
  { id: 'class:wizard', label: 'Wizard', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 6 },
  { id: 'class:bard', label: 'Bard', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 8 },
  { id: 'class:barbarian', label: 'Barbarian', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 12 },
  { id: 'class:rogue', label: 'Rogue', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 8 },
  { id: 'class:cleric', label: 'Cleric', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 8 },
  { id: 'class:druid', label: 'Druid', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 8 },
  { id: 'class:monk', label: 'Monk', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 8 },
  // APG (`advanced_players_guide/apg_classes.lst`).
  { id: 'class:alchemist', label: 'Alchemist', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 8 },
  { id: 'class:cavalier', label: 'Cavalier', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 10 },
  { id: 'class:inquisitor', label: 'Inquisitor', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 8 },
  { id: 'class:oracle', label: 'Oracle', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 8 },
  { id: 'class:summoner', label: 'Summoner', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 8 },
  { id: 'class:witch', label: 'Witch', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 6 },
  // ACG (`advanced_class_guide/acg_classes.lst`).
  { id: 'class:arcanist', label: 'Arcanist', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 6 },
  { id: 'class:bloodrager', label: 'Bloodrager', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 10 },
  { id: 'class:brawler', label: 'Brawler', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 10 },
  { id: 'class:hunter', label: 'Hunter', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 8 },
  { id: 'class:investigator', label: 'Investigator', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 8 },
  { id: 'class:shaman', label: 'Shaman', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 8 },
  { id: 'class:skald', label: 'Skald', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 8 },
  { id: 'class:slayer', label: 'Slayer', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 10 },
  { id: 'class:swashbuckler', label: 'Swashbuckler', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 10 },
  { id: 'class:warpriest', label: 'Warpriest', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 8 },
  /*
   * Pathfinder Unchained (`pathfinder_unchained/pu_abilities_class.lst`),
   * added SD-27 2026-07-31.
   *
   * These four are REPLACEMENTS for the CRB Barbarian / Monk / Rogue and
   * the APG Summoner, not additions to them. PCGen models each as a
   * selection ability in that class's single-slot pool, so a campaign
   * picks one of the pair and a character never holds both. Both members
   * stay in this list because the *app* has no campaign-level rules
   * toggle: the player picks the version their table uses. They are kept
   * apart on three axes and none of them is cosmetic — a distinct `id`
   * (`class:unchained_*`, which is what is persisted to disk), a distinct
   * `label`, and a distinct engine path
   * (`rules_tables::pathfinder_unchained`, rule set `Pu`).
   *
   * `hitDie` is the engine's own value per class, not the base class's by
   * assumption: three of the four genuinely borrow it (their corpus record
   * overrides no chassis field) and the Unchained Monk does not — d10
   * here against the CRB Monk's d8 above. That row is the reason this
   * table could not simply alias the four.
   *
   * All four are `full` on the same evidence every other row uses:
   * `cargo run --bin v06_class_state_dump` reports 31/31 classes reaching
   * `Computed` at every level 1-20, with the original 27 byte-identical to
   * their pre-change dump. Their remaining feature gaps are carried as
   * non-claim-blocking `class_feature.pu.*.other_features_deferred`
   * diagnostics on the character's own receipt — most notably the
   * Unchained Summoner's 202-spell list, which is NOT transcribed.
   */
  { id: 'class:unchained_barbarian', label: 'Unchained Barbarian', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 12 },
  { id: 'class:unchained_monk', label: 'Unchained Monk', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 10 },
  { id: 'class:unchained_rogue', label: 'Unchained Rogue', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 8 },
  { id: 'class:unchained_summoner', label: 'Unchained Summoner', supportLevel: 'full', levelOptions: EVERY_CLASS_LEVEL, hitDie: 8 },
];

const DEFAULT_LEVEL_OPTIONS: number[] = [1];

export function getLevelOptionsForClass(classId: string): number[] {
  return CLASS_OPTIONS.find((option) => option.id === classId)?.levelOptions ?? DEFAULT_LEVEL_OPTIONS;
}

/**
 * Keep a chosen level valid for the class it is being applied to. The class
 * and level pickers are independent controls, so switching class can strand a
 * level the new class does not offer (Fighter 20 → Monk, whose only offered
 * level is 1). Falls back to the highest level the class does offer, so
 * switching between two fully-computed classes preserves the player's level
 * rather than silently resetting them to 1.
 */
export function clampLevelForClass(classId: string, level: number): number {
  const options = getLevelOptionsForClass(classId);
  if (options.includes(level)) {
    return level;
  }
  const highest = options[options.length - 1] ?? 1;
  return level < highest ? options[0] ?? 1 : highest;
}

/**
 * Whether `LevelUpDialog` may offer another level in this class, given how
 * many levels of it the character already holds (`0` for a class they have
 * never taken). This is the same `levelOptions` claim the creation picker
 * makes, applied to the other way into a level: a class the engine dump
 * reports `Blocked` past level 1 must not be reachable by leveling up either,
 * and no class may pass PF1's own level-20 ceiling — the dump sweeps exactly
 * 1-20, so a 21st level is unverified reach, not merely untested.
 */
export function canTakeAnotherLevelIn(classId: string, currentClassLevel: number): boolean {
  return getLevelOptionsForClass(classId).includes(currentClassLevel + 1);
}

export function describeClassSupportLevel(supportLevel: ClassSupportLevel, classLabel: string): string {
  switch (supportLevel) {
    case 'full':
      return `${classLabel} is fully computed at every level offered here.`;
    case 'partial-human-only':
      return `${classLabel} is only computed for Human today — other races show what's still missing.`;
    case 'human-diagnostics-only':
      return `${classLabel} isn't computed by the engine for any race yet, including Human — Human just shows more specific detail about what's missing.`;
    case 'full-except-human-level-1':
      return `${classLabel} is computed at every level offered here for every race — except Human at level 1 specifically, which stays blocked; Human reaches the same computed build from level 2 on.`;
    case 'headless-only':
      return `${classLabel}'s engine can compute a real build, but the picker it needs (a bloodline, a domain, a nature bond, etc.) doesn't exist in this UI yet, so every character created here stays blocked today, for any race.`;
    case 'none':
      return `${classLabel} isn't computed by the engine yet.`;
  }
}

/** The dropdown-option suffix for `CreateCharacterForm`'s Class select — kept alongside `describeClassSupportLevel` so both stay honest about the same distinction. */
export function classSupportLevelSuffix(supportLevel: ClassSupportLevel): string {
  switch (supportLevel) {
    case 'full':
      return '';
    case 'partial-human-only':
      return ' (Human only, partial)';
    case 'human-diagnostics-only':
      return ' (not yet computed for any race)';
    case 'full-except-human-level-1':
      return ' (Human: not at level 1)';
    case 'headless-only':
      return ' (blocked today — needs a choice picker not yet built)';
    case 'none':
      return ' (not yet computed)';
  }
}

export const DEFAULT_ABILITY_SCORES = {
  strength: 16,
  dexterity: 14,
  constitution: 14,
  intelligence: 10,
  wisdom: 12,
  charisma: 8,
};

export const GAME_SYSTEM_LABELS: Record<string, string> = {
  pf1: 'Pathfinder 1st Edition',
};

/** The nine standard PF1 alignments. */
export const ALIGNMENT_OPTIONS: readonly string[] = [
  'Lawful Good',
  'Neutral Good',
  'Chaotic Good',
  'Lawful Neutral',
  'True Neutral',
  'Chaotic Neutral',
  'Lawful Evil',
  'Neutral Evil',
  'Chaotic Evil',
];

export const PHYSICAL_ABILITIES: readonly AbilityKey[] = ['strength', 'dexterity', 'constitution'];
export const MENTAL_ABILITIES: readonly AbilityKey[] = ['intelligence', 'wisdom', 'charisma'];

export type AgeCategory = 'Adult' | 'Middle Age' | 'Old' | 'Venerable';
export const AGE_OPTIONS: readonly AgeCategory[] = ['Adult', 'Middle Age', 'Old', 'Venerable'];

/**
 * Cumulative PF1 aging ability modifiers: physical abilities (Str/Dex/Con)
 * take the penalty, mental abilities (Int/Wis/Cha) take the bonus.
 */
export const AGE_EFFECTS: Record<AgeCategory, { physical: number; mental: number }> = {
  Adult: { physical: 0, mental: 0 },
  'Middle Age': { physical: -1, mental: 1 },
  Old: { physical: -3, mental: 2 },
  Venerable: { physical: -6, mental: 3 },
};

export function ageEffectForAbility(age: AgeCategory, ability: AbilityKey): number {
  const effect = AGE_EFFECTS[age];
  if (PHYSICAL_ABILITIES.includes(ability)) {
    return effect.physical;
  }
  if (MENTAL_ABILITIES.includes(ability)) {
    return effect.mental;
  }
  return 0;
}

/** PF1 ability modifier: floor((score - 10) / 2). */
export function abilityModifier(score: number): number {
  return Math.floor((score - 10) / 2);
}

// `maxHitPointsAtLevelOne` used to live here. Its only caller was
// `CreateCharacterForm`'s HP preview, which could assume level 1 back when the
// form hardcoded that level. Now that the form has a real Level picker it uses
// `characterProgression`'s `maxHitPoints` instead — the same function the
// character sheet uses, so the previewed HP and the created character's HP
// cannot disagree. Nothing else referenced the level-1-only version.

/** Sum of `count` dice with `sides` faces. */
export function rollDice(count: number, sides: number): number {
  let total = 0;
  for (let index = 0; index < count; index += 1) {
    total += Math.floor(Math.random() * sides) + 1;
  }
  return total;
}

/** Inches to a `f'i"` string. */
export function formatHeight(totalInches: number): string {
  const feet = Math.floor(totalInches / 12);
  const inches = totalInches % 12;
  return `${feet}'${inches}"`;
}
