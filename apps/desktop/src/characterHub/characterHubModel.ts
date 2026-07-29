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

export interface RaceOption {
  id: string;
  label: string;
  /** Fixed PF1 racial ability modifiers applied to every member of the race. */
  abilityAdjustments: Partial<Record<AbilityKey, number>>;
  /**
   * Floating ability points the player distributes freely (PF1's "+2 to one
   * ability" races: Human, Half-Elf, Half-Orc). `0` means the race has no
   * player-selectable enhancement.
   */
  floatingBonusPoints: number;
  size: 'Small' | 'Medium';
  vision: string;
  body: Record<Sex, BodyProfile>;
}

/** The full PF1 core rulebook race roster, with core ability modifiers and physical profiles. */
export const RACE_OPTIONS: RaceOption[] = [
  {
    id: 'race:human',
    label: 'Human',
    abilityAdjustments: {},
    floatingBonusPoints: 2,
    size: 'Medium',
    vision: 'Normal',
    body: {
      male: { baseHeightInches: 58, heightModDice: { count: 2, sides: 10 }, baseWeightLb: 120, weightMultiplierLb: 5 },
      female: { baseHeightInches: 53, heightModDice: { count: 2, sides: 10 }, baseWeightLb: 85, weightMultiplierLb: 5 },
    },
  },
  {
    id: 'race:dwarf',
    label: 'Dwarf',
    abilityAdjustments: { constitution: 2, wisdom: 2, charisma: -2 },
    floatingBonusPoints: 0,
    size: 'Medium',
    vision: 'Darkvision 60 ft.',
    body: {
      male: { baseHeightInches: 45, heightModDice: { count: 2, sides: 4 }, baseWeightLb: 150, weightMultiplierLb: 7 },
      female: { baseHeightInches: 43, heightModDice: { count: 2, sides: 4 }, baseWeightLb: 120, weightMultiplierLb: 7 },
    },
  },
  {
    id: 'race:elf',
    label: 'Elf',
    abilityAdjustments: { dexterity: 2, intelligence: 2, constitution: -2 },
    floatingBonusPoints: 0,
    size: 'Medium',
    vision: 'Low-light vision',
    body: {
      male: { baseHeightInches: 60, heightModDice: { count: 2, sides: 6 }, baseWeightLb: 100, weightMultiplierLb: 3 },
      female: { baseHeightInches: 60, heightModDice: { count: 2, sides: 6 }, baseWeightLb: 90, weightMultiplierLb: 3 },
    },
  },
  {
    id: 'race:gnome',
    label: 'Gnome',
    abilityAdjustments: { constitution: 2, charisma: 2, strength: -2 },
    floatingBonusPoints: 0,
    size: 'Small',
    vision: 'Low-light vision',
    body: {
      male: { baseHeightInches: 36, heightModDice: { count: 2, sides: 4 }, baseWeightLb: 35, weightMultiplierLb: 1 },
      female: { baseHeightInches: 34, heightModDice: { count: 2, sides: 4 }, baseWeightLb: 30, weightMultiplierLb: 1 },
    },
  },
  {
    id: 'race:half-elf',
    label: 'Half-Elf',
    abilityAdjustments: {},
    floatingBonusPoints: 2,
    size: 'Medium',
    vision: 'Low-light vision',
    body: {
      male: { baseHeightInches: 55, heightModDice: { count: 2, sides: 8 }, baseWeightLb: 110, weightMultiplierLb: 5 },
      female: { baseHeightInches: 55, heightModDice: { count: 2, sides: 8 }, baseWeightLb: 90, weightMultiplierLb: 5 },
    },
  },
  {
    id: 'race:half-orc',
    label: 'Half-Orc',
    abilityAdjustments: {},
    floatingBonusPoints: 2,
    size: 'Medium',
    vision: 'Darkvision 60 ft.',
    body: {
      male: { baseHeightInches: 58, heightModDice: { count: 2, sides: 12 }, baseWeightLb: 150, weightMultiplierLb: 7 },
      female: { baseHeightInches: 58, heightModDice: { count: 2, sides: 12 }, baseWeightLb: 120, weightMultiplierLb: 7 },
    },
  },
  {
    id: 'race:halfling',
    label: 'Halfling',
    abilityAdjustments: { dexterity: 2, charisma: 2, strength: -2 },
    floatingBonusPoints: 0,
    size: 'Small',
    vision: 'Normal',
    body: {
      male: { baseHeightInches: 32, heightModDice: { count: 2, sides: 4 }, baseWeightLb: 30, weightMultiplierLb: 1 },
      female: { baseHeightInches: 30, heightModDice: { count: 2, sides: 4 }, baseWeightLb: 25, weightMultiplierLb: 1 },
    },
  },
];

/**
 * `full` — reaches `Computed` for any race in `RACE_OPTIONS`.
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
 * The full PF1 core rulebook class roster.
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
 * Paladin and Ranger are `full-except-human-level-1` (v0.6 alpha swarm,
 * class-breadth epic, 2026-07-25): both reached real `Computed` status once
 * their spell posture was genuinely computed (`b7642d97` Ranger,
 * `ee3c50ce` Paladin) rather than left as an unconditional blocker. Both
 * still share `explain_hybrid_level1_chassis`'s original, untouched
 * single-class-Human-at-level-1 gate, so that one combination stays
 * `Blocked` while every other race/level combination genuinely computes —
 * see the `full-except-human-level-1` doc above for the live-verification
 * detail (repeated independently for each class: Human blocked at level 1,
 * Elf computed at level 1 and through level 4 of leveling up).
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
 * as race-independent, evaluated before any Human-only gate. But two of
 * the three have a real, separate LEVEL cap this file's `levelOptions`
 * must respect, or the `full` label would overclaim:
 * - **Sorcerer**: this entry NO LONGER has an engine level cap (corrected
 *   2026-07-29). It previously read: "`ARCANE_BLOODLINE_BONUS_LEVEL = 3` —
 *   Computed for any race at levels 1-2 only; level 3+ stays genuinely
 *   `Blocked` (bloodline bonus spells/feats at 3rd+ aren't grounded)",
 *   which was live-verified true at the time. It is now false:
 *   `ground_sorcerer_arcane_bloodline_progression` grounds the Arcane
 *   bloodline's whole 3rd-and-above progression from the corpus, and
 *   `cargo run --bin v06_class_state_dump` reports Sorcerer `Computed` at
 *   every level 1-20 under this exact seeded posture (Arcane bloodline +
 *   familiar bond, which `pf1_adapter.rs` already applies). The `[1, 2]`
 *   below is therefore no longer an engine blocker — it is only this
 *   file's conservative live-verified-range convention, the same lag that
 *   leaves Wizard at `[1]` while the engine computes it at all 20 levels.
 *   Raising it is a UI-side change needing its own live `LevelUpDialog`
 *   verification, deliberately not made here alongside the engine fix.
 * - **Cleric**: no level cap found (Good domain, without Healing, has no
 *   level-gated condition anywhere in `explain_cleric_level1_spell_baseline`).
 *   Live-verified: a fresh Human Cleric 1 reached `Computed`/`Saved`,
 *   disk-confirmed; leveled cleanly through level 3 with no blocker.
 * - **Druid**: Computed only at EXACTLY level 1 — the code's own condition
 *   is `animal_companion_chosen_top && druid_level == 1`; level 2+ falls
 *   to the catch-all `Blocked` diagnostic (companion advancement isn't
 *   grounded past level 1). Live-verified: a fresh Human Druid 1 reached
 *   `Computed`/`Saved`, disk-confirmed; attempting level 2 through the
 *   real `LevelUpDialog` correctly stayed at level 1 with the real
 *   `class_feature.druid.animal_companion.unsupported` diagnostic shown.
 *
 * `levelOptions` reflects exactly this: Sorcerer `[1, 2]`, Cleric
 * `[1, 2, 3]` (Fighter's own conservative verified-range convention, not
 * the theoretical max), Druid `[1]` only.
 */
export const CLASS_OPTIONS: ClassOption[] = [
  { id: 'class:fighter', label: 'Fighter', supportLevel: 'full', levelOptions: [1, 2, 3], hitDie: 10 },
  { id: 'class:paladin', label: 'Paladin', supportLevel: 'full-except-human-level-1', levelOptions: [1, 2, 3, 4, 5], hitDie: 10 },
  { id: 'class:ranger', label: 'Ranger', supportLevel: 'full-except-human-level-1', levelOptions: [1, 2, 3, 4, 5], hitDie: 10 },
  { id: 'class:sorcerer', label: 'Sorcerer', supportLevel: 'full', levelOptions: [1, 2], hitDie: 6 },
  { id: 'class:wizard', label: 'Wizard', supportLevel: 'full', levelOptions: [1], hitDie: 6 },
  { id: 'class:bard', label: 'Bard', supportLevel: 'full', levelOptions: [1, 2, 3], hitDie: 8 },
  { id: 'class:barbarian', label: 'Barbarian', supportLevel: 'full', levelOptions: [1, 2, 3], hitDie: 12 },
  { id: 'class:rogue', label: 'Rogue', supportLevel: 'full', levelOptions: [1], hitDie: 8 },
  { id: 'class:cleric', label: 'Cleric', supportLevel: 'full', levelOptions: [1, 2, 3], hitDie: 8 },
  { id: 'class:druid', label: 'Druid', supportLevel: 'full', levelOptions: [1], hitDie: 8 },
  { id: 'class:monk', label: 'Monk', supportLevel: 'human-diagnostics-only', levelOptions: [1], hitDie: 8 },
];

const DEFAULT_LEVEL_OPTIONS: number[] = [1];

export function getLevelOptionsForClass(classId: string): number[] {
  return CLASS_OPTIONS.find((option) => option.id === classId)?.levelOptions ?? DEFAULT_LEVEL_OPTIONS;
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

/** Level-1 HP: the class hit die maximum plus the constitution modifier (floored at 1). */
export function maxHitPointsAtLevelOne(hitDie: number, constitutionScore: number): number {
  return Math.max(1, hitDie + abilityModifier(constitutionScore));
}

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
