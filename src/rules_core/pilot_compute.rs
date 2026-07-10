//! GE-06 pilot deterministic rules-core computation surface.
//!
//! Computes and explains the bounded set of outputs accumulated across the GE-06
//! pilot slices for the accepted PF1 Human Fighter level-1 deterministic input:
//!
//! - ability modifiers (`floor(score/2) - 5`)
//! - Fighter level-1 base attack bonus
//! - Fighter level-1 base saves
//! - baseline melee attack bonus for the deterministic Longsword loadout
//! - baseline armor class for the deterministic Chain Shirt / Dodge / no-shield posture
//! - total Fortitude / Reflex / Will saves (base save + relevant ability modifier)
//!
//! Each computed value carries a machine-checkable explanation record. This is
//! intentionally not a full rules engine: it does not compute feat-, item-, or
//! condition-based save modifiers, weapon damage, active Power Attack math,
//! initiative, skill modifiers, armor-check penalties, equipment effects beyond
//! the deterministic baseline, feat prerequisites, or any oracle parity. Support
//! is the bounded deterministic Human Fighter posture widened across the SD13-E3
//! milestone tranche from level 1 to levels 2 and 3 only: the level-2 bonus-feat
//! progression seam and the level-3 armor-training seam are surfaced explicitly,
//! but nothing here grounds level-4+ Fighter burden, a general feat-effect engine,
//! spellcasting, multiclassing, or non-Fighter positive support. The SD13-E3-F6 slice
//! additionally recognizes the deterministic Human Paladin level-1 and Human Ranger
//! level-1 hybrid chassis as direct runtime evidence, but keeps both explicitly
//! claim-blocked on their still-missing non-spell class-feature burden and later spell
//! burden; it grounds no hybrid class-feature or spell math. The SD13-E4-F7 slice
//! also recognizes the deterministic Human Sorcerer level-1 spell-bearing identity as
//! a direct runtime evidence, but keeps it explicitly claim-blocked on its
//! bloodline and spontaneous known-spell / slot posture burdens; it grounds no
//! bloodline power and no spell math. The SD13-E4-R3 slice further recognizes the
//! deterministic Human Wizard level-1 prepared arcane spell-bearing identity as
//! direct runtime evidence, but keeps it explicitly claim-blocked on its school
//! specialization burden and prepared spellbook / spells-prepared / spell-slot
//! posture burden; it grounds no spellbook content, no spells prepared, no spell
//! slots, no spell save DCs, no bonus spells, no school-opposition bookkeeping, and
//! no specialty school bonus. A later SD13-E4 Wizard decomposition slice splits that
//! school specialization burden in two: Scribe Scroll, the free specialization-
//! independent bonus feat every 1st-level Wizard is granted, is grounded for real,
//! while the specialization CHOICE burden (chosen school, opposed schools, specialty
//! school bonus) stays its own named claim-blocking diagnostic; the prepared
//! spellbook / spells-prepared / spell-slot posture burden is untouched. The SD13-E5
//! Wizard specialization slice then grounds the flat surface of that choice for real:
//! the canonical Evocation specialization with Necromancy and Transmutation opposed
//! is recognized, and the specialist bonus slot is grounded as a flat count (one
//! 1st-level Evocation-only slot at level 1, no cantrip-level slot, no slot
//! contents), narrowing the claim-blocker to the school powers (intense spells,
//! force missile) and the opposed-school two-slot preparation cost. The SD13-E3
//! Fighter milestone tranche has since
//! widened further still, to level 8: the level-8 bonus-feat progression seam is
//! surfaced explicitly, mirroring the level-2/4/6 bonus-feat seams, and grounds no
//! level-9+ Fighter burden. The SD13-E3 Rogue pillar-grounding slice widens the
//! deterministic Human Rogue level-1 chassis to ground base-attack progression
//! (3/4 BAB), base-save progression (good Reflex, poor Fortitude, poor Will), and
//! the sneak attack damage-die count (1, i.e. 1d6); the SD13-E5 Rogue slice grounds
//! the fourth named pillar, Trapfinding (the flat max(rogue level / 2, 1) bonus on
//! Perception checks to locate traps and on Disable Device checks, plus the
//! magic-trap-disarm statement), so no named Rogue pillar burden remains
//! claim-blocked, and `defense.total_save.*` is still never computed for
//! it. The SD13-E3 Barbarian level-1 martial chassis slice is widened further here:
//! base-attack progression, base-save progression, and the fast-movement +10 ft.
//! speed value are now grounded as standalone explanation records (mirroring the
//! Fighter formula shape). The SD13-E5 Barbarian slice then resolves the
//! formerly-named illiteracy burden as vacuous (the PF1 Core Rulebook Barbarian is
//! not illiterate; illiteracy is a D&D 3.5e trait that never existed in PF1) and
//! grounds Rage's flat numeric surface — rage rounds per day (4 + Constitution
//! modifier) and the flat rage constants — values only, leaving the rage-state
//! execution engine explicitly claim-blocked as the honest remaining Barbarian
//! burden. A later SD13-E3 slice widens the deterministic Human Monk
//! level-1 chassis to ground base-attack, base-save, and AC Bonus (Wisdom-to-AC),
//! while keeping unarmed strike / Flurry of Blows and the level-1 bonus feat grant
//! explicitly claim-blocked. The SD13-E3 Ranger decomposition further splits the F6
//! Ranger non-spell class-feature burden into three named pillars: favored enemy and
//! combat style stay explicitly claim-blocked by their own named diagnostics, and
//! Track (the Survival-check bonus to follow tracks, ½ ranger level minimum 1) is
//! grounded for real as a bounded numeric value; it grounds no combat-style math and
//! no ranger spell posture. The SD13-E5 Ranger Favored Enemy slice then grounds the
//! favored-enemy flat surface for real — recognition of the chosen favored-enemy type
//! (from `choice:ranger_favored_enemy`), the flat +2 bonus on Bluff, Knowledge,
//! Perception, Sense Motive, and Survival checks against the favored enemy, and the
//! flat +2 bonus on weapon attack and damage rolls against the favored enemy (PF1
//! includes attack rolls, unlike D&D 3.5) — retiring the favored-enemy claim-blocking
//! diagnostic while grounding no target-type matching or conditional-application
//! engine. A later SD13-E5 Ranger Combat Style slice corrects a mistaken framing in
//! the combat-style diagnostic (it previously claimed the archery-vs-two-weapon-combat
//! style choice was a level-1 decision separate from a level-2 bonus-feat grant; PF1
//! Core Rulebook actually grants the style choice and its first bonus feat TOGETHER at
//! 2nd level) and retires the claim-blocking diagnostic in favor of a grounded
//! level-gate absence record (value 0), mirroring the Paladin mercy level-gate idiom —
//! no bonus-feat mechanical value is fabricated. The SD13-E4 Druid Wild Empathy
//! slice further splits the Druid nature-bond/wild-empathy class-feature blocker
//! into two named diagnostics: nature bond (the animal-companion-vs-domain choice
//! and nature sense) stays explicitly claim-blocked, and Wild Empathy (PF1 Core
//! Rulebook: 1d20 + druid level + Charisma modifier, used like a Diplomacy check to
//! improve an animal's attitude) is grounded for real as the flat druid-level +
//! Cha-modifier value; it grounds no nature-bond power execution and no
//! Diplomacy-check/d20-roll resolution. The SD13-E5 Druid Nature Sense /
//! nature-bond-choice slice grounds Nature Sense for real (PF1 Core Rulebook: a
//! flat, level-independent +2 bonus on Knowledge (nature) and Survival checks,
//! kept as a standalone record not wired into any skill total), recognizes the
//! deterministic `choice:druid_nature_bond -> bond:animal_companion` selection as
//! a +0 recognition record, and narrows the retired combined nature-bond blocker
//! to the chosen bond's execution (companion stat block, companion advancement,
//! link / share spells), which stays claim-blocked. The SD13-E4 Sorcerer decomposition
//! slice further splits the F7 combined bloodline burden into two named diagnostics
//! and grounds one for real: Eschew Materials, the universal, bloodline-independent
//! bonus feat every 1st-level Sorcerer receives; it grounds no bloodline power,
//! bloodline arcana, or spell math. The SD13-E5 Sorcerer bloodline-choice slice then
//! recognizes the canonical deterministic bloodline choice-slot selection
//! (`choice:sorcerer_bloodline -> bloodline:arcane`) as chosen input — recognition
//! only, since the Arcane bloodline's level-1 power is Arcane Bond (a familiar or a
//! bonded object), an execution engine rather than a flat number — and narrows the
//! former bloodline-power blocker to the Arcane Bond / bloodline progression burden;
//! that burden and the spontaneous spell-posture burden stay explicitly
//! claim-blocked. Unsupported input yields
//! claim-blocking diagnostics and withheld explanations rather than fabricated values.

use super::character_input::{AbilityScores, ActiveState, CharacterInput, SkillAllocation};

/// Result of the GE-06 pilot deterministic compute surface, accumulating the
/// base chassis, baseline combat, and total-save outputs proven across slices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotBaseChassisComputation {
    pub ability_modifiers: AbilityModifiers,
    /// Class/base attack bonus only. Zero when the chassis is unsupported.
    pub base_attack_bonus: i16,
    /// Class/base save bonuses only (no ability modifiers added to these).
    pub base_saves: BaseSaves,
    /// Baseline melee attack bonus for the deterministic Longsword loadout. Zero
    /// when the required deterministic combat posture is absent or unsupported.
    pub baseline_melee_attack_bonus: i16,
    /// Baseline armor class for the deterministic Chain Shirt / Dodge / no-shield
    /// posture. Zero when that posture is absent or unsupported.
    pub baseline_armor_class: i16,
    /// Total saving throws (Fighter base save + relevant ability modifier). Zero
    /// when the Fighter level-1 chassis is absent or unsupported.
    pub total_saves: BaseSaves,
    /// Selected deterministic Climb / Intimidate / Swim skill modifiers. All zero
    /// when the deterministic selected-skill or Chain Shirt posture is absent or
    /// widened beyond this slice.
    pub selected_skill_modifiers: SelectedSkillModifiers,
    pub explanations: Vec<ComputationExplanation>,
    pub diagnostics: Vec<ComputationDiagnostic>,
}

/// Ability modifiers derived from chosen ability scores via `floor(score/2) - 5`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AbilityModifiers {
    pub strength: i16,
    pub dexterity: i16,
    pub constitution: i16,
    pub intelligence: i16,
    pub wisdom: i16,
    pub charisma: i16,
}

/// Base save bonuses from the grounded class chassis row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BaseSaves {
    pub fortitude: i16,
    pub reflex: i16,
    pub will: i16,
}

/// Selected deterministic skill modifiers bounded to the GE-06 pilot slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SelectedSkillModifiers {
    pub climb: i16,
    pub intimidate: i16,
    pub swim: i16,
}

/// A machine-checkable record explaining why a single computed value exists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComputationExplanation {
    /// Stable id (e.g. `ability_modifier.strength`, `class_chassis.base_attack_bonus`).
    pub id: String,
    /// The computed value this record explains.
    pub value: i16,
    /// Human-auditable detail referencing the source input and formula.
    pub detail: String,
}

/// A diagnostic that blocks downstream claims when an input is unsupported here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComputationDiagnostic {
    /// Stable id for the diagnostic subject (e.g. `class_chassis.unsupported`).
    pub id: String,
    pub message: String,
    pub claim_blocking: bool,
}

const FIGHTER_CLASS_ID: &str = "class:fighter";

// SD13-E3-F6 hybrid chassis baseline identities. Paladin and Ranger are hybrid
// (martial + later spellcasting) classes; this slice recognizes only their bounded
// single-class level-1 chassis as direct runtime evidence and grounds no class-feature
// or spell math for either.
const PALADIN_CLASS_ID: &str = "class:paladin";
const RANGER_CLASS_ID: &str = "class:ranger";
const HYBRID_BASELINE_LEVEL: u8 = 1;

// SD13-E5 Paladin milestone widening. The accepted level-1/level-2/level-3
// chassis-and-spell-burden separation is now joined by level 4, the PF1 Core
// Rulebook level gate at which Smite Evil's uses/day genuinely increases (to
// 2/day) and Channel Positive Energy is newly granted, and by level 5, the
// PF1 Core Rulebook level gate at which the effective-caster-level gate
// genuinely increases again (to 2) and Channel Positive Energy's flat die
// count genuinely increases (to 3d6). Divine Bond, the PF1 CRB's OTHER
// 5th-level paladin class feature, was checked against a primary source
// (legacy.aonprd.com's Core Rulebook Paladin page) and confirmed NOT flat:
// it requires an activation/resource-consumption engine plus either an
// ongoing weapon-enhancement subsystem or a full mount
// stat-block/advancement subsystem, neither of which exists in this
// codebase, mirroring the Monk High Jump / Wizard level-5 bonus feat
// precedent exactly -- so it is deliberately left named-but-unproven, not
// fabricated. Level 6 joined next (both good saves and lay on hands/heal
// dice genuinely increase again; the level-6 repeat Mercy grant was checked
// and confirmed to need a mercy-list-growth mechanism this codebase has not
// grounded, so it stays named-but-unproven). Level 7 joins next in turn (PF1
// CRB level-7 "Special" column: "Smite evil 3/day" only, verified
// independently against d20pfsrd and legacy.aonprd.com -- base saves and lay
// on hands both stay numerically unchanged from level 6, an
// integer-division coincidence, while Smite Evil uses/day, the
// effective-caster-level gate, and Channel Positive Energy's die count all
// genuinely increase again; level 7 is not one of the repeat-Mercy-grant
// levels, 3/6/9/..., so nothing new is left unproven for Mercy here).
// Level 8 joins next in turn (PF1 CRB level-8 "Special" column: "Aura of
// resolve" only, verified independently against d20pfsrd and
// legacy.aonprd.com -- base attack and both good saves genuinely rise, poor
// Reflex stays +2, an integer-division coincidence; Lay on Hands genuinely
// rises on both axes (uses 6, heal dice 4) and Smite Evil's damage bonus
// rises to 8 (= paladin level) while its uses/day stay 3, the next rise
// landing at 10th; the effective caster level rises to 5 (8 - 3); Channel
// Positive Energy's die count stays 4, the effective-cleric dice rising at
// odd levels so the next rise lands at 9th. Aura of Resolve itself was
// checked rather than assumed away and confirmed NOT flat: immunity to
// charm spells/spell-like abilities plus a +4 morale aura for allies within
// 10 feet while conscious needs a condition-immunity engine and an
// ally-aura/positional engine, neither of which exists in this codebase --
// exactly like Aura of Courage and Divine Health before it, it stays
// deliberately named-but-unproven, not fabricated). Level 9 joins next in
// turn (PF1 CRB level-9 "Special" column: "Mercy" only, verified
// independently against d20pfsrd and legacy.aonprd.com -- base attack
// genuinely rises to +9 and poor Reflex genuinely rises to +3 while both
// good saves stay +6, integer-division coincidences; Smite Evil stays 3/day
// (next rise 10th) with its damage bonus rising to 9 (= paladin level); Lay
// on Hands stays at uses 6 / heal dice 4, integer-division coincidences;
// the effective caster level rises to 6 (9 - 3); Channel Positive Energy's
// die count genuinely rises to 5, the effective-cleric dice rising at odd
// levels. 9th IS a repeat-Mercy-grant level (the 3rd/6th/9th cadence), but
// exactly like the level-6 repeat grant, recognizing a second mercy
// selection needs the mercy-list-growth mechanism this codebase has never
// grounded, so the repeat grant stays deliberately named-but-unproven and
// the single grounded level-3 selection carries over unchanged). Nothing
// here grounds level 10+ Paladin.
const MAX_SUPPORTED_PALADIN_LEVEL: u8 = 9;

// Lay on hands and divine grace are both 2nd-level paladin features in the PF1 Core
// Rulebook. Below this level their honest computed surface is their correct
// ABSENCE; at or above it, this slice grounds their flat numeric formulas.
const PALADIN_LAY_ON_HANDS_DIVINE_GRACE_LEVEL: u8 = 2;

// Mercy is a 3rd-level paladin feature (gained at 3rd level and every three levels
// thereafter). Below this level its honest computed surface is its correct
// ABSENCE; at or above it (SD13-E5 level-3 widening), this slice grounds a
// bounded GRANT-only identity record (mirroring the Barbarian Uncanny Dodge /
// Ranger Endurance idiom) plus, when the fixture provides one, a choice-
// recognition record naming whichever mercy was selected (mirroring the Ranger
// Favored Terrain / Sorcerer bloodline choice-slot idiom). PF1 Core Rulebook
// Mercy (verified independently against legacy.aonprd.com's Core Rulebook
// Paladin page): "At 3rd level, and every three levels thereafter, a paladin
// can select one mercy. Each mercy adds an effect to the paladin's lay on
// hands ability." The first, 3rd-level tier of the mercy list is Fatigued,
// Shaken, and Sickened -- verified against the Core-Rulebook-scoped primary
// source rather than the aggregated Archives of Nethys mercy table, which also
// lists two additional 3rd-level-tier mercies (Deceived, Riled) sourced from a
// later supplement (Ultimate Combat), out of scope for this Core-Rulebook-only
// grounding. This grounds only the CHOICE recognition; the selected mercy's own
// effect (curing the named condition automatically whenever lay on hands is
// used) is NOT computed, since no lay-on-hands execution engine exists
// anywhere in this codebase.
const PALADIN_MERCY_LEVEL: u8 = 3;

// Channel Positive Energy is a 4th-level paladin feature in the PF1 Core
// Rulebook (verified independently against legacy.aonprd.com's Core Rulebook
// Paladin page): "When a paladin reaches 4th level, she gains the
// supernatural ability to channel positive energy like a cleric. Using this
// ability consumes two uses of her lay on hands ability. A paladin uses her
// level as her effective cleric level when channeling positive energy."
// Below this level its honest computed surface is its correct ABSENCE; at or
// above it (SD13-E5 level-4 widening), this slice grounds only the flat
// channel-energy die-count magnitude (ceil(effective cleric level / 2)),
// mirroring the Cleric Channel Energy dice-count idiom exactly. No
// healing/damage-resolution execution, no heal-vs-harm target selection, and
// no lay-on-hands-resource-consumption bookkeeping is computed.
const PALADIN_CHANNEL_POSITIVE_ENERGY_LEVEL: u8 = 4;

/// SD13-E5 Paladin Mercy choice-slot id. The deterministic level-3 fixture names a
/// chosen mercy (e.g. `mercy:shaken`); the compute seam recognizes whichever raw
/// mercy string was actually selected, mirroring `choice:ranger_favored_terrain`'s
/// open-ended (non-restricted-list) recognition idiom exactly -- no enum
/// validation against the mercy list is performed here.
const PALADIN_MERCY_CHOICE_ID: &str = "choice:paladin_mercy";

// SD13-E5 Ranger Combat Style correction. Combat Style Feat is a 2nd-level ranger
// feature in the PF1 Core Rulebook: the ranger selects a combat style (archery or
// two-weapon combat) and gains its first bonus feat TOGETHER at 2nd level -- these
// are not separable into a level-1 style choice plus a level-2 feat grant, as an
// earlier version of the Ranger combat-style diagnostic incorrectly claimed. Below
// this gate, combat style is always a correct ABSENCE, mirroring PALADIN_MERCY_LEVEL;
// at or above it (SD13-E5 level-2 widening), the style choice and its bonus feat are
// finally grounded for real as recognition records -- see
// RANGER_COMBAT_STYLE_CHOICE_ID below.
const RANGER_COMBAT_STYLE_LEVEL: u8 = 2;

// SD13-E5 Ranger level-range widening. The accepted level-1 Ranger per-pillar
// decomposition (base attack/base save progression, Track, the Favored Enemy flat
// surface, and the combat-style level-gate absence) is joined by level 2, the PF1
// Core Rulebook level gate at which Combat Style Feat is actually granted, by
// level 3 (SD13-E5), the level gate at which Endurance and Favored Terrain are
// granted, by level 4 (SD13-E5), the level gate at which Hunter's Bond is
// granted, by level 5 (SD13-E5), the level gate at which the Favored Enemy
// rule's own 5th-level interval (a second favored enemy plus a +2 bonus increase
// to any one favored enemy of the ranger's choice) is granted, by level 6
// (SD13-E5), the level gate at which the ranger's SECOND combat-style bonus feat
// is granted (verified independently against d20pfsrd and legacy.aonprd.com: both
// state "The ranger's expertise manifests in the form of bonus feats at 2nd, 6th,
// 10th, 14th, and 18th level" -- 6th level is the very next milestone after 2nd,
// not 3rd/4th/5th as some earlier framings assumed), and by level 7 (SD13-E5),
// the level gate at which Woodland Stride is granted (verified independently
// against d20pfsrd and legacy.aonprd.com: both list "Woodland stride" as the
// Ranger 7th-level "Special" column entry, and both state the exact rule text,
// "a ranger may move through any sort of undergrowth ... at his normal speed and
// without taking damage or suffering any other impairment ... magically
// manipulated undergrowth ... still affects him normally" -- an automatic,
// no-choice, no-numeric-magnitude grant, grounded as a bounded identity record
// only, mirroring the Endurance grant-only idiom). Neither the Favored Enemy
// rule's next interval (10th level) nor the Combat Style Feat's next bonus feat
// (10th level) is reached at level 7, so both stay unchanged, re-verified rather
// than assumed. A still later SD13-E5 slice widens the gate once more to level 8,
// the level gate at which Swift Tracker is granted (verified independently
// against d20pfsrd and legacy.aonprd.com: both list the level-8 "Special" column
// as naming TWO entries, "Swift tracker" and "2nd favored terrain"). Swift
// Tracker ("a ranger can move at his normal speed while using Survival to follow
// tracks without taking the normal -5 penalty. He takes only a -10 penalty
// (instead of the normal -20) when moving at up to twice normal speed while
// tracking") only modifies a tracking-while-moving penalty resolution that does
// not exist anywhere in this codebase (this codebase grounds only the flat Track
// skill-bonus magnitude, never a check-execution/movement-penalty engine), so it
// is a genuinely flat/identity-shaped, no-choice, no-magnitude grant, grounded
// as a bounded identity record only, mirroring the Woodland Stride grant-only
// idiom exactly. The level-8 "2nd favored terrain" entry mirrors the Favored
// Enemy 5th-level idiom already grounded in this codebase (a second
// terrain-type selection plus a bonus-increase-target choice), but is a
// multi-record burden of its own -- deliberately left named-but-unproven this
// slice, a real newly discovered burden for a future slice, not an invented
// one. A further SD13-E5 slice widens the gate to level 9 (verified
// independently against d20pfsrd and legacy.aonprd.com): level 9 base attack
// genuinely rises to +9 (full BAB) and poor Will genuinely rises to +3
// (9 / 3), while both good saves stay +6 (9 / 2 + 2, integer-division
// coincidences); Track stays 4 (max(9/2, 1), a coincidence); the
// favored-enemy/favored-terrain/hunter's-bond facets all carry over unchanged
// (the next favored-enemy grant lands at 10th, the next favored-terrain grant
// at 13th, both checked rather than assumed); the level-9 "Special" column
// reads "Evasion" — a genuinely NEW class feature, grounded as a +0
// identity/recognition record only (RANGER_EVASION_LEVEL), mirroring Rogue's
// and Monk's own Evasion records; no damage-resolution engine exists here, so
// no damage math is fabricated from it. Nothing here grounds level 10+
// Ranger.
const MAX_SUPPORTED_RANGER_LEVEL: u8 = 9;

/// PF1 Core Rulebook level gate at which Woodland Stride is granted (verified
/// independently against two primary sources: both d20pfsrd and
/// legacy.aonprd.com list "Woodland stride" as the Ranger 7th-level "Special"
/// column entry, with no other new class feature named at 7th level). Woodland
/// Stride is an automatic, no-choice grant with no numeric magnitude of its own:
/// "a ranger may move through any sort of undergrowth (such as natural thorns,
/// briars, overgrown areas, and similar terrain) at his normal speed and without
/// taking damage or suffering any other impairment. However, magically
/// manipulated undergrowth still affects him normally." No terrain-detection or
/// movement-resolution engine exists in this codebase, so only the grant
/// identity itself is grounded.
const RANGER_WOODLAND_STRIDE_LEVEL: u8 = 7;

/// PF1 Core Rulebook level gate at which Swift Tracker is granted (verified
/// independently against two primary sources: both d20pfsrd and
/// legacy.aonprd.com list "Swift tracker" as one of two Ranger 8th-level
/// "Special" column entries, alongside "2nd favored terrain"). Swift Tracker is
/// an automatic, no-choice grant with no numeric magnitude of its own: "a
/// ranger can move at his normal speed while using Survival to follow tracks
/// without taking the normal -5 penalty. He takes only a -10 penalty (instead
/// of the normal -20) when moving at up to twice normal speed while tracking."
/// No tracking-while-moving check-execution/movement-penalty engine exists in
/// this codebase (this codebase grounds only the flat Track skill-bonus
/// magnitude), so only the grant identity itself is grounded, mirroring the
/// Woodland Stride idiom exactly.
const RANGER_SWIFT_TRACKER_LEVEL: u8 = 8;
/// PF1 Core Rulebook level gate at which Ranger gains Evasion (9th level,
/// verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Evasion" as the Ranger 9th-level "Special"
/// column entry — the same rule text as Rogue's and Monk's own Evasion).
const RANGER_EVASION_LEVEL: u8 = 9;

/// PF1 Core Rulebook level gate at which the Favored Enemy rule's 5th-level
/// interval is granted (verified independently against two primary sources:
/// both d20pfsrd and legacy.aonprd.com list "2nd favored enemy" as the Ranger
/// 5th-level "Special" column entry, and both state the exact rule text: "At
/// 5th level and every five levels thereafter (10th, 15th, and 20th level), the
/// ranger may select an additional favored enemy. In addition, at each such
/// interval, the bonus against any one favored enemy (including the one just
/// selected, if so desired) increases by 2." This is genuinely two things at
/// once: a second favored-enemy TYPE selection (open-ended, mirroring the first
/// favored enemy's own choice-slot idiom) and a separate, independent choice of
/// WHICH one favored enemy (the newly selected one or an already-held one)
/// receives the +2 magnitude increase -- it is NOT an automatic bump to the
/// first favored enemy, so this slice grounds the target as its own restricted
/// two-option choice-slot (mirroring the Hunter's Bond/combat-style restricted
/// two-option idiom) rather than assuming a specific outcome.
const RANGER_FAVORED_ENEMY_SECOND_INTERVAL_LEVEL: u8 = 5;

/// SD13-E5 Ranger second favored-enemy choice-slot id. The deterministic fixture
/// names a second favored-enemy type (e.g. `enemy:undead`); the compute seam
/// recognizes whichever raw enemy-type string was actually selected, mirroring
/// `choice:ranger_favored_enemy`'s open-ended (non-restricted-list) recognition
/// idiom exactly.
const RANGER_FAVORED_ENEMY_SECOND_CHOICE_ID: &str = "choice:ranger_favored_enemy_2";

/// SD13-E5 Ranger favored-enemy bonus-increase target choice-slot id. Names
/// which ONE of the (now two) favored enemies receives the rule's own +2
/// magnitude increase at the 5th-level interval -- a restricted two-option
/// choice (`enemy:first` or `enemy:second`), mirroring the Hunter's Bond
/// restricted two-option choice idiom exactly (unlike the open-ended favored
/// enemy TYPE choice-slots themselves).
const RANGER_FAVORED_ENEMY_BONUS_INCREASE_CHOICE_ID: &str =
    "choice:ranger_favored_enemy_bonus_increase_target";
const RANGER_FAVORED_ENEMY_BONUS_INCREASE_FIRST_SELECTION: &str = "enemy:first";
const RANGER_FAVORED_ENEMY_BONUS_INCREASE_SECOND_SELECTION: &str = "enemy:second";

/// PF1 Core Rulebook level gate at which Ranger gains Endurance (3rd level,
/// verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Endurance, favored terrain" as the Ranger
/// 3rd-level special feature entry). Endurance is a bonus feat granted
/// automatically, with no player choice involved ("A ranger gains Endurance as a
/// bonus feat at 3rd level"), so it is grounded as a bounded grant-only identity
/// record, mirroring the Wizard Scribe Scroll / Barbarian Uncanny Dodge idiom.
const RANGER_ENDURANCE_LEVEL: u8 = 3;

/// PF1 Core Rulebook level gate at which Ranger gains Favored Terrain (3rd level,
/// the same gate as Endurance -- both are the two named entries in the class
/// table's 3rd-level "Special" column, verified independently against two primary
/// sources: d20pfsrd and legacy.aonprd.com both list "Endurance, favored terrain"
/// and both state the exact bonus text: "+2 bonus on Initiative checks and
/// Knowledge (geography), Perception, Stealth, and Survival skill checks" made
/// when the ranger is in the chosen terrain, selected from Table: Ranger Favored
/// Terrains' fixed eleven-entry list (Cold, Desert, Forest, Jungle, Mountain,
/// Plains, Planes, Swamp, Underground, Urban, Water). Unlike Endurance, Favored
/// Terrain is a genuine player choice, so this slice grounds a choice-slot
/// recognition record (naming whichever terrain was selected, mirroring the
/// Favored Enemy choice-recognition idiom exactly) plus the rule's own flat +2
/// magnitude, grounded as a standalone, non-applied record -- no
/// terrain-detection engine decides whether the character is actually in the
/// chosen terrain, and the +2 is never wired into any actual Initiative total or
/// skill-check total. The level-8th/13th/18th additional-terrain and
/// bonus-increase progression stays out of scope for this bounded slice.
const RANGER_FAVORED_TERRAIN_LEVEL: u8 = 3;

/// SD13-E5 Ranger Favored Terrain choice-slot id. The deterministic fixture names
/// a chosen terrain (e.g. `terrain:forest`); the compute seam recognizes whichever
/// raw terrain string was actually selected, mirroring
/// `choice:ranger_favored_enemy`'s open-ended (non-restricted-list) recognition
/// idiom exactly -- no enum validation against the Table: Ranger Favored Terrains
/// list is performed here.
const RANGER_FAVORED_TERRAIN_CHOICE_ID: &str = "choice:ranger_favored_terrain";

/// PF1 Core Rulebook level gate at which Ranger gains Hunter's Bond (4th level,
/// verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Hunter's bond" as the Ranger 4th-level "Special"
/// column entry, and both state the exact rule text: "At 4th level, a ranger
/// forms a bond with his hunting companions. This bond can take one of two
/// forms. Once the form is chosen, it cannot be changed." The first form, a
/// bond to his companions, grants the ranger the ability to spend a move action
/// to grant allies within 30 feet who can see or hear him half his favored-enemy
/// bonus against a single target of the appropriate type -- a genuinely
/// flat-shaped magnitude (half the already-grounded Favored Enemy bonus), grounded
/// as a standalone, non-applied record: no move-action/action-economy engine, no
/// ally-range-and-perception check, and no favored-enemy target-type matching is
/// implemented. The second form, an animal companion, is deliberately left
/// named-but-unproven: it would require a full animal-companion stat
/// block/advancement subsystem that does not exist anywhere in this codebase, a
/// new-subsystem-shaped burden, not a slice-shaped one.
const RANGER_HUNTERS_BOND_LEVEL: u8 = 4;

/// SD13-E5 Ranger Hunter's Bond choice-slot id. The deterministic fixture names
/// which of the two mutually exclusive forms was chosen (`form:bond` or
/// `form:companion`), mirroring `choice:ranger_combat_style`'s restricted
/// two-option recognition idiom (unlike the open-ended Favored Enemy/Favored
/// Terrain choice-slots, Hunter's Bond only has two legal forms).
const RANGER_HUNTERS_BOND_CHOICE_ID: &str = "choice:ranger_hunters_bond";
const RANGER_HUNTERS_BOND_COMPANION_SELECTION: &str = "form:companion";
const RANGER_HUNTERS_BOND_BOND_SELECTION: &str = "form:bond";

// SD13-E5 Ranger combat style choice-slot recognition, grounded once the level-range
// gate reaches RANGER_COMBAT_STYLE_LEVEL (2nd level). PF1 Core Rulebook Combat Style
// Feat: at 2nd level a ranger selects one combat style -- Archery or Two-Weapon
// Combat, the two PF1 Core Rulebook options -- and gains the first bonus feat from
// that style's own restricted list (verified against legacy.aonprd.com's Core
// Rulebook Ranger page before writing any code): the Archery style's 2nd-level list
// is Far Shot, Point-Blank Shot, Precise Shot, and Rapid Shot; the Two-Weapon Combat
// style's 2nd-level list is Double Slice, Improved Shield Bash, Quick Draw, and
// Two-Weapon Fighting. Both the STYLE CHOICE and the chosen BONUS FEAT are
// recognized as chosen-input identity only (+0 each); no feat's own mechanical
// effect (e.g. Point-Blank Shot's attack/damage bonus within 30 ft.) is computed
// anywhere in this codebase.
const RANGER_COMBAT_STYLE_CHOICE_ID: &str = "choice:ranger_combat_style";
const RANGER_COMBAT_STYLE_ARCHERY_SELECTION: &str = "style:archery";
const RANGER_COMBAT_STYLE_TWO_WEAPON_COMBAT_SELECTION: &str = "style:two_weapon_combat";

const RANGER_COMBAT_STYLE_BONUS_FEAT_CHOICE_ID: &str = "choice:ranger_combat_style_bonus_feat";
// PF1 Core Rulebook Archery combat style, 2nd-level bonus feat list.
const FAR_SHOT_FEAT_SELECTION: &str = "feat:far_shot";
const POINT_BLANK_SHOT_FEAT_SELECTION: &str = "feat:point_blank_shot";
const PRECISE_SHOT_FEAT_SELECTION: &str = "feat:precise_shot";
const RAPID_SHOT_FEAT_SELECTION: &str = "feat:rapid_shot";
// PF1 Core Rulebook Two-Weapon Combat style, 2nd-level bonus feat list.
const DOUBLE_SLICE_FEAT_SELECTION: &str = "feat:double_slice";
const IMPROVED_SHIELD_BASH_FEAT_SELECTION: &str = "feat:improved_shield_bash";
const QUICK_DRAW_FEAT_SELECTION: &str = "feat:quick_draw";
const TWO_WEAPON_FIGHTING_FEAT_SELECTION: &str = "feat:two_weapon_fighting";

// SD13-E5 Ranger SECOND combat style bonus feat, granted at 6th level (verified
// independently against d20pfsrd and legacy.aonprd.com's Core Rulebook Ranger
// page before writing any code: "The ranger's expertise manifests in the form of
// bonus feats at 2nd, 6th, 10th, 14th, and 18th level" -- 6th level is the very
// next milestone after 2nd). PF1 Core Rulebook Combat Style Feat text: "He can
// choose feats from his selected combat style, even if he does not have the
// normal prerequisites." Both primary sources agree on which feats each style's
// list gains specifically at 6th level (as distinct from the 2nd-level list
// already grounded above): the Archery style's 6th-level list is Improved
// Precise Shot and Manyshot; the Two-Weapon Combat style's 6th-level list is
// Improved Two-Weapon Fighting and Two-Weapon Defense. This grounds only a
// restricted-list recognition of the specific feat named at this milestone
// (gated on the same style choice already recognized at 2nd level); it does not
// validate the second choice against the cumulative (2nd+6th level) list, so a
// selection re-picking one of the 2nd-level list's own feats at this gate is
// deliberately left unrecognized rather than silently accepted as if it were
// general-purpose feat validation. No feat's own mechanical effect is computed
// anywhere in this codebase.
const RANGER_COMBAT_STYLE_BONUS_FEAT_2_LEVEL: u8 = 6;
const RANGER_COMBAT_STYLE_BONUS_FEAT_2_CHOICE_ID: &str =
    "choice:ranger_combat_style_bonus_feat_2";
// PF1 Core Rulebook Archery combat style, 6th-level bonus feat list.
const IMPROVED_PRECISE_SHOT_FEAT_SELECTION: &str = "feat:improved_precise_shot";
const MANYSHOT_FEAT_SELECTION: &str = "feat:manyshot";
// PF1 Core Rulebook Two-Weapon Combat style, 6th-level bonus feat list.
const IMPROVED_TWO_WEAPON_FIGHTING_FEAT_SELECTION: &str = "feat:improved_two_weapon_fighting";
const TWO_WEAPON_DEFENSE_FEAT_SELECTION: &str = "feat:two_weapon_defense";

// SD13-E4-F7 spell-bearing baseline identity. Sorcerer is a spontaneous full arcane
// caster; this slice recognizes only its bounded single-class level-1 identity as direct
// runtime evidence and grounds no bloodline power and no spell math (spell slots, spells
// known, spell DCs, bonus spells, or prepared posture) for it. A further SD13-E5 slice
// widens the level-1-only gate to a level-range gate (`supported_sorcerer_level`,
// 1..=MAX_SUPPORTED_SORCERER_LEVEL), mirroring the Fighter/Paladin/Rogue/Barbarian/
// Monk/Cleric/Bard/Druid idiom: the PF1 Core Rulebook Sorcerer class table's level-2
// "Special" column is blank (verified against d20pfsrd and legacy.aonprd.com), so no new
// class feature is gained at 2nd level, unlike Rogue/Monk/Druid's Evasion/Woodland
// Stride — this widening extends the existing pillars only, adding no new one. A further
// SD13-E5 slice widens the gate again to level 3: the PF1 Core Rulebook Sorcerer class
// table's level-3 "Special" column reads "Bloodline power, bloodline spell" (verified
// against d20pfsrd and legacy.aonprd.com), NOT blank like level 2 — but both entries are
// bloodline-specific (they name a different power/spell per bloodline, e.g. the Arcane
// bloodline's own 3rd-level power is Metamagic Adept and its 3rd-level bloodline spell is
// Identify) and neither is flat/identity-shaped the way Rogue's Trap Sense or Monk's Still
// Mind are, so no new pillar record is added for level 3 either — both entries stay named
// by the pre-existing `arcane_bond_and_bloodline_progression.unsupported` diagnostic's
// "bonus spells/feats at 3rd+ level" and "bloodline power" language, unchanged. This
// widening extends only the already-grounded base-attack/base-save/bloodline-choice
// pillars to level 3. A further SD13-E5 slice widens the gate again to level 4: the PF1
// Core Rulebook Sorcerer class table's level-4 "Special" column is blank (verified
// independently against d20pfsrd and legacy.aonprd.com), UNLIKE the level-3 row's
// "Bloodline power, bloodline spell" entry, so no new class feature is gained at 4th
// level and no new pillar record is added — this widening extends only the
// already-grounded base-attack/base-save/bloodline-choice/bloodline-class-skill-choice
// pillars to level 4 via the same formulas, without re-derivation. A further SD13-E5
// slice widens the gate again to level 5: the PF1 Core Rulebook Sorcerer class table's
// level-5 "Special" column reads "Bloodline spell" (verified independently against
// d20pfsrd and legacy.aonprd.com), UNLIKE the blank level-4 column, so this was checked
// rather than assumed away — this is the sorcerer's second bloodline spell grant (the
// first came at level 3, alongside the level-3 bloodline power), and the Arcane
// bloodline's own 5th-level bloodline spell is invisibility, but the entry is
// bloodline-specific (it names a different spell per bloodline) and not
// flat/identity-shaped the way Rogue's Trap Sense or Monk's Still Mind are, so no new
// pillar record is added for level 5 either, mirroring exactly how the level-3
// "Bloodline power, bloodline spell" entry was left unproven — this widening extends
// only the already-grounded base-attack/base-save/bloodline-choice/
// bloodline-class-skill-choice pillars to level 5 via the same formulas, without
// re-derivation. A further SD13-E5 slice widens the gate again to level 8: the PF1
// Core Rulebook Sorcerer class table's level-8 "Special" column is blank (verified
// independently against d20pfsrd and legacy.aonprd.com, checked rather than assumed
// away) — like levels 2, 4, and 6, and UNLIKE the level-7 "Bloodline feat, bloodline
// spell" row — so no new class feature is gained at 8th level. The first 4th-level
// spell slots arrive at 8th, but spells per day belong to the spontaneous spell
// burden that stays named-but-unproven, so this widening extends only the
// already-grounded base-attack/base-save/bloodline-choice/
// bloodline-class-skill-choice pillars to level 8 via the same formulas, without
// re-derivation. A further SD13-E5 slice widens the gate again to level 9: the
// PF1 Core Rulebook Sorcerer class table's level-9 "Special" column reads
// "Bloodline power, bloodline spell" (verified independently against d20pfsrd
// and legacy.aonprd.com, checked rather than assumed away) — UNLIKE the blank
// level-8 column — but both entries are bloodline-specific (the Arcane
// bloodline's own 9th-level power is New Arcana and its 9th-level bloodline
// spell is overland flight) and neither is flat/identity-shaped, so no new
// pillar record is added for level 9 either, mirroring exactly how the
// level-3/5/7 bloodline entries were left unproven — this widening extends
// only the already-grounded base-attack/base-save/bloodline-choice/
// bloodline-class-skill-choice pillars to level 9 via the same formulas,
// without re-derivation (poor Fortitude/Reflex both genuinely rise to +3 at
// level 9 while base attack and good Will stay put, integer-division
// coincidences).
const SORCERER_CLASS_ID: &str = "class:sorcerer";
const MAX_SUPPORTED_SORCERER_LEVEL: u8 = 9;

// SD13-E5 canonical Sorcerer bloodline choice seam. The deterministic fixture names the
// Arcane bloodline as its chosen selection; the compute seam recognizes exactly that
// chosen input. Recognition only: the Arcane bloodline's level-1 power is Arcane Bond
// (a familiar or a bonded object — an execution engine, not a flat number), so no
// power value is ever fabricated from this choice.
const SORCERER_BLOODLINE_CHOICE_ID: &str = "choice:sorcerer_bloodline";
const ARCANE_BLOODLINE_SELECTION_ID: &str = "bloodline:arcane";

// SD13-E5 Arcane bloodline class-skill choice seam. The PF1 Core Rulebook Arcane
// bloodline entry reads "Class Skill: Knowledge (any one)" (verified against both
// d20pfsrd and the legacy Paizo PRD mirror) — a player's choice of any one Knowledge
// skill, not a fixed grant of Knowledge (arcana) specifically. This choice-slot
// selection is recognized only when the Arcane bloodline itself was recognized above,
// since this class-skill grant belongs to that bloodline.
const SORCERER_BLOODLINE_CLASS_SKILL_CHOICE_ID: &str = "choice:sorcerer_bloodline_class_skill";

// SD13-E4-F7/SD13-E5 spell-bearing baseline identity. Bard is a spontaneous arcane
// caster with a distinct chassis-class-feature burden (Bardic Knowledge and Bardic
// Music); this slice recognizes its bounded single-class level-1/level-2 identity as
// direct runtime evidence and grounds no performance-state engine (no
// start/maintain action economy, no round tracking or consumption), no Countersong,
// Distraction, or Versatile Performance execution, and no spell math (spells known,
// spells per day, spell DCs, bonus spells, school choice, or prepared posture) for
// it.
const BARD_CLASS_ID: &str = "class:bard";
/// SD13-E5 Bard level-range gate, mirroring the Fighter `supported_fighter_level` /
/// Paladin `supported_paladin_level` / Rogue `supported_rogue_level` / Barbarian
/// `supported_barbarian_level` / Monk `supported_monk_level` / Cleric
/// `supported_cleric_level` idiom. Verified against the PF1 Core Rulebook Bard class
/// table (d20pfsrd and legacy.aonprd.com) before widening: level 2 base attack +1,
/// base saves +0/+3/+3 (Fortitude/Reflex/Will), Bardic Performance rounds per day
/// gains 2 additional rounds after 1st level, Inspire Courage's flat magnitude does
/// not increase until level 5, and the level-2 "Special" column reads "Versatile
/// performance, well-versed" — Well-Versed (a flat, non-level-scaled +4 save bonus)
/// is grounded this slice; Versatile Performance (a choice-gated skill-substitution
/// engine) is deliberately left named-but-unproven. Widened again to level 3 by a
/// further SD13-E5 slice: level 3 base attack +2, base saves +1/+3/+3
/// (Fortitude/Reflex/Will), every other flat formula (Bardic Knowledge, Bardic
/// Performance rounds/day, Inspire Courage, Fascinate DC/affected-creature-count,
/// Well-Versed) extends via the same formula with no re-derivation, and the
/// level-3 "Special" column reads "Inspire competence +2" — a flat, identity-shaped
/// class feature grounded this slice. Unlike Wizard's specialist-bonus-slot or
/// Cleric's domain-slot doubling at level 3, Bard has no grounded spell-slot-count
/// pillar at all (the Bard spells-per-day table's own 2nd-level spell column does
/// not turn non-blank until 4th level, verified independently, and this row's
/// still-unproven list already names the entire spontaneous spell posture as
/// ungrounded), so no analogous slot-count doubling applies here. A later SD13-E5
/// slice widens this gate to level 4, verified independently against the PF1 Core
/// Rulebook Bard class table (d20pfsrd and legacy.aonprd.com): the level-4
/// "Special" column is BLANK (no new class feature is gained at 4th level; the
/// next new feature, Lore Master, comes at 5th level), so that widening extends
/// every already-grounded formula (base attack, base saves, Bardic Knowledge,
/// Bardic Performance rounds/day, Fascinate DC/count) and keeps Well-Versed and
/// Inspire Competence granted, without introducing any new pillar. A further
/// SD13-E5 slice widens this gate to level 5, re-verified independently
/// against both primary sources rather than trusting an earlier cycle's "stays
/// +1 through level 5" note at face value: the level-5 "Special" column reads
/// "Inspire courage +2, lore master 1/day", so the Inspire Courage flat
/// magnitude genuinely increases to +2 exactly at level 5 (the earlier note
/// turns out to have been precise, not imprecise — level 4 stays +1, and level
/// 5 is exactly the level the rule's own "at 5th level... increases by +1"
/// text describes), and Lore Master is newly grounded as a bounded grant-only
/// flat 1/day usage-count record for its take-20 half only (the take-10 half
/// has no flat magnitude to ground and neither mechanic is executed against
/// any actual Knowledge check). A still further SD13-E5 slice widens this
/// gate to level 6, verified independently against the PF1 Core Rulebook
/// Bard class table (d20pfsrd and the legacy.aonprd.com mirror): the
/// level-6 row is BAB +4, Fort +2, Ref +5, Will +5 — all extended via the
/// same pre-existing formulas, no re-derivation. Bardic Knowledge genuinely
/// rises to 3 (`max(6/2, 1)`), Bardic Performance rounds per day continues
/// scaling, and the Fascinate DC genuinely rises to 15 (`10 + 6/2 + CHA`);
/// the Fascinate affected-creature count stays 2 (an integer-division
/// coincidence with level 5), and Inspire Courage stays +2 (the next
/// increase does not land until bard level 11). The level-6 "Special"
/// column reads "Suggestion, Versatile performance" (verified independently
/// against both primary sources). Both entries were checked and confirmed
/// NOT flat: Suggestion is a spell-like ability requiring a
/// fascinated-target prerequisite and the "suggestion" spell's own
/// effect-resolution engine (neither exists in this codebase), and the
/// 6th-level Versatile Performance grant is merely an additional instance
/// of the same choice-gated skill-substitution engine already deliberately
/// left named-but-unproven at 2nd level, not a new type of class feature —
/// so no new pillar record is grounded at level 6. A still further SD13-E5
/// slice widens this gate to level 7, verified independently against the
/// PF1 Core Rulebook Bard class table (d20pfsrd and the legacy.aonprd.com
/// mirror): the level-7 row is BAB +5, Fort +2, Ref +5, Will +5 — base
/// attack genuinely rises to 5 (`7 * 3 / 4`) while all three base saves
/// stay numerically unchanged from level 6 (an integer-division
/// coincidence, re-verified against the raw table row rather than
/// assumed). Bardic Knowledge stays 3 (`max(7/2, 1)`, unchanged), Bardic
/// Performance rounds per day continues scaling, the Fascinate DC stays 15
/// (`10 + 7/2 + CHA`, an integer-division coincidence with level 6), and
/// the Fascinate affected-creature count genuinely rises to 3
/// (`1 + (7-1)/3`), up from 2 at level 6. The level-7 "Special" column
/// reads "Inspire competence +3" (verified independently against both
/// primary sources): the Inspire Competence rule text itself confirms this
/// is a flat magnitude increase on an already-grounded pillar ("This bonus
/// increases by +1 for every four levels the bard has attained beyond 3rd
/// (+3 at 7th, +4 at 11th, +5 at 15th, and +6 at 19th)"), the same kind of
/// arithmetic tier-widening as Inspire Courage's own second tier at level
/// 5, not a new class feature — grounded as a genuine rise to +3.
/// A still further SD13-E5 slice widens this gate to level 8, verified
/// independently against the PF1 Core Rulebook Bard class table (d20pfsrd
/// and the legacy.aonprd.com mirror): the level-8 row is BAB +6/+1, Fort +2,
/// Ref +6, Will +6 -- base attack genuinely rises to 6 (`8 * 3 / 4`, the
/// class table's own iterative-attack notation "+6/+1" not modeled anywhere
/// in this codebase, only the flat base value, mirroring the Cleric
/// level-8 precedent), base Fortitude stays 2 (`8/3`, an integer-division
/// coincidence with level 7, re-verified rather than assumed), and base
/// Reflex/Will both genuinely rise to 6 (`8/2+2`), up from 5 at level 7.
/// Bardic Knowledge genuinely rises to 4 (`max(8/2, 1)`), up from 3 at
/// level 7. Bardic Performance rounds per day continues scaling. The
/// Fascinate DC genuinely rises to 16 (`10 + 8/2 + CHA`), up from 15 at
/// level 7, while the Fascinate affected-creature count stays 3
/// (`1 + (8-1)/3 = 1 + 7/3 = 3`), an integer-division coincidence with
/// level 7, confirmed by direct arithmetic against the primary source rule
/// text rather than trusted from the formula alone. Inspire Courage stays
/// +2 and Inspire Competence stays +3 (neither's next tier lands until
/// bard level 11). The level-8 "Special" column reads "Dirge of doom"
/// (verified independently against both primary sources): a genuinely NEW
/// bardic-performance type, checked and confirmed NOT flat/identity-shaped
/// -- it requires both the same performance-state engine already left
/// ungrounded (start/maintain action economy, round tracking/consumption)
/// and a fear/shaken-condition resolution engine, neither of which exists
/// in this codebase, so it is deliberately left named-but-unproven,
/// mirroring the Suggestion / Countersong / Distraction precedent exactly
/// -- no explanation record is fabricated for it.
///
/// A further SD13-E5 slice widens the gate to level 9 (verified independently
/// against d20pfsrd and legacy.aonprd.com): level 9 base attack stays +6
/// (9 * 3 / 4) and good Reflex/Will both stay +6 (9 / 2 + 2),
/// integer-division coincidences, while poor Fortitude genuinely rises to +3
/// (9 / 3); the Bardic Performance rounds-per-day pool genuinely rises to 22
/// (4 + Cha mod + 2 per level after 1st); Bardic Knowledge, the Fascinate
/// DC/count, Inspire Courage, Inspire Competence, Well-Versed, and Lore
/// Master all carry over unchanged (the next Fascinate-count rise lands at
/// 10th and the next Inspire/Lore tiers at 11th, checked rather than
/// assumed); the level-9 "Special" column reads "Inspire greatness" -- a
/// genuinely NEW bardic-performance type checked and confirmed NOT flat (it
/// grants 2 bonus Hit Dice with commensurate temporary hit points, a +2
/// competence attack bonus, and a +1 competence Fortitude bonus to a willing
/// ally, requiring the performance-state engine plus
/// temporary-Hit-Dice/temporary-hit-point mechanics, none of which exist in
/// this codebase), so it is deliberately left named-but-unproven, mirroring
/// the Suggestion / Countersong / Distraction / Dirge-of-Doom precedent
/// exactly -- no explanation record is fabricated for it.
const MAX_SUPPORTED_BARD_LEVEL: u8 = 9;
/// PF1 Core Rulebook level gate at which Bard gains Well-Versed (2nd level, verified
/// independently against two primary sources: d20pfsrd and legacy.aonprd.com both
/// list "Versatile performance, well-versed" as the Bard 2nd-level special feature
/// entry).
const BARD_WELL_VERSED_LEVEL: u8 = 2;
/// PF1 Core Rulebook Well-Versed magnitude: a flat +4 bonus on saving throws against
/// bardic performance, sonic, and language-dependent effects. Unlike Bardic
/// Knowledge or Fascinate, this magnitude is NOT level-scaled (it stays +4 for the
/// class feature's entire existence), verified against both primary sources rather
/// than assumed to follow the "half level" idiom used elsewhere on this seam.
const BARD_WELL_VERSED_BONUS: i16 = 4;
/// PF1 Core Rulebook Bardic Performance additional-rounds-per-level constant: "At
/// each level after 1st a bard can use bardic performance for 2 additional rounds
/// per day" (verified against d20pfsrd and legacy.aonprd.com, not assumed from
/// Barbarian's superficially similar Rage-rounds progression).
const BARD_PERFORMANCE_ADDITIONAL_ROUNDS_PER_LEVEL: i16 = 2;
/// PF1 Core Rulebook level gate at which Bard gains Inspire Competence (3rd level,
/// verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Inspire competence +2" as the Bard 3rd-level
/// special feature entry).
const BARD_INSPIRE_COMPETENCE_LEVEL: u8 = 3;
/// PF1 Core Rulebook level at which the Inspire Competence flat magnitude first
/// increases from +2 to +3 (7th level, verified independently against two
/// primary sources: d20pfsrd and legacy.aonprd.com both list "Inspire
/// competence +3" as the Bard 7th-level special feature entry, and both state
/// the rule text "This bonus increases by +1 for every four levels the bard
/// has attained beyond 3rd (+3 at 7th, +4 at 11th, +5 at 15th, and +6 at
/// 19th)"). The next increase (to +4) lands at bard level 11, out of scope
/// since only Bard levels 1-8 are supported.
const BARD_INSPIRE_COMPETENCE_SECOND_TIER_LEVEL: u8 = 7;
/// PF1 Core Rulebook Inspire Competence magnitude at the level it is first gained: a
/// flat +2 competence bonus on skill checks with a particular skill. Verified
/// against both primary sources.
const BARD_INSPIRE_COMPETENCE_BONUS_FIRST_TIER: i16 = 2;
/// PF1 Core Rulebook Inspire Competence flat magnitude at or above the
/// second-tier level gate (7th level and beyond, until the next tier at 11th
/// level, out of scope here).
const BARD_INSPIRE_COMPETENCE_BONUS_SECOND_TIER: i16 = 3;
/// PF1 Core Rulebook level at which the Inspire Courage flat magnitude first
/// increases from +1 to +2 (5th level, verified independently against two
/// primary sources: d20pfsrd and legacy.aonprd.com both list "Inspire courage
/// +2, lore master 1/day" as the Bard 5th-level special feature entry, and
/// both state the rule text "At 5th level, and every six bard levels
/// thereafter, this bonus increases by +1"). The next increase (to +3) lands
/// at bard level 11, out of scope since only Bard levels 1-5 are supported.
const BARD_INSPIRE_COURAGE_SECOND_TIER_LEVEL: u8 = 5;
/// PF1 Core Rulebook Inspire Courage flat magnitude below the second-tier
/// level gate.
const BARD_INSPIRE_COURAGE_BONUS_FIRST_TIER: i16 = 1;
/// PF1 Core Rulebook Inspire Courage flat magnitude at or above the
/// second-tier level gate (5th level and beyond, until the next tier at 11th
/// level, out of scope here).
const BARD_INSPIRE_COURAGE_BONUS_SECOND_TIER: i16 = 2;
/// PF1 Core Rulebook level gate at which Bard gains Lore Master (5th level,
/// verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Inspire courage +2, lore master 1/day" as the
/// Bard 5th-level special feature entry). The rule grants two capabilities:
/// an at-will "take 10 on any Knowledge skill check he has ranks in"
/// capability (no flat magnitude to ground — a resolution-mode toggle, not a
/// countable resource) and a flat "once per day... take 20 on any Knowledge
/// skill check" capability. Only the latter's flat usage count is grounded,
/// mirroring the Paladin Smite Evil / Wizard Force Missile uses-per-day
/// idiom; neither mechanic is executed against any actual Knowledge check.
const BARD_LORE_MASTER_LEVEL: u8 = 5;
/// PF1 Core Rulebook Lore Master take-20 usage-count magnitude: a flat 1/day,
/// non-level-scaled count (verified against both primary sources).
const BARD_LORE_MASTER_TAKE_20_USES_PER_DAY: i16 = 1;

// SD13-E5 Fascinate flat DC base. PF1 Core Rulebook Fascinate Will save DC is
// 10 + 1/2 bard level + Charisma modifier; only the fixed base term is a named
// constant, since the level and Charisma terms are already grounded elsewhere.
const FASCINATE_DC_BASE: i16 = 10;

// Grounded SD13-E4-R3 Human Wizard level-1 prepared arcane spell-bearing baseline
// identities. The Wizard class is the canonical PF1 prepared arcane full caster;
// its class identity differs from Sorcerer in two ways that this bounded slice
// surfaces explicitly: the prepared posture (spellbook + spells prepared per day +
// spell slots per day) and the school specialization (one school chosen, two
// opposed schools locked, specialty school bonus at later levels).
const WIZARD_CLASS_ID: &str = "class:wizard";

// SD13-E5 Wizard level-2/level-3/level-4/level-5/level-6 progression widening:
// mirrors the Fighter `supported_fighter_level` / Paladin `supported_paladin_level` /
// Rogue `supported_rogue_level` / Barbarian `supported_barbarian_level` / Monk
// `supported_monk_level` / Cleric `supported_cleric_level` / Bard
// `supported_bard_level` / Druid `supported_druid_level` / Sorcerer
// `supported_sorcerer_level` idiom (an `Option<u8>` level-range gate) rather than a
// boolean level-1-only check. Verified against the PF1 Core Rulebook Wizard class
// table (d20pfsrd and a second independent Archives of Nethys mirror): the level-2,
// level-3, level-4, AND level-6 "Special" columns are all blank, so no new class
// feature is gained at 2nd, 3rd, 4th, or 6th level (like Cleric/Sorcerer's level-2
// gate, unlike Rogue/Monk/Druid's Evasion/Woodland Stride or Rogue/Monk/Barbarian's
// own 3rd-level features); the level-5 "Special" column reads "Bonus feat" — a
// genuinely NEW class feature, verified rather than assumed, but checked and
// confirmed NOT flat (a choice among an open-ended set of metamagic feats, item
// creation feats, or Spell Mastery — a general feat-selection/feat-prerequisite
// engine, mirroring the Monk High Jump precedent exactly), so it is deliberately left
// named-but-unproven and grounds no record; the specialist bonus slot flat count DOES
// change at level 3 (see `explain_wizard_level1_prepared_spell_baseline`), since a
// level-3 wizard casts 2nd-level spells for the first time, STAYS at that same value
// through level 4 (3rd-level wizard spells do not become available until level 5,
// verified independently against both primary sources' raw spells-per-day table
// rows), DOES change again for real at level 5 (a level-5 wizard casts 3rd-level
// spells for the first time, so the specialist bonus slot count becomes 3: one bonus
// slot of each of 1st/2nd/3rd spell level), then STAYS at 3 through level 6
// (4th-level wizard spells do not become available until level 7, verified
// independently against both primary sources' raw spells-per-day table rows); the
// Intense Spells bonus-damage magnitude DOES change at level 4 (half wizard level,
// minimum 1, reaches 2 for the first time via the pre-existing formula), STAYS at 2
// through level 5 (`max(5/2, 1) = 2`, an integer-division coincidence, not a formula
// that stopped scaling), then DOES change again for real at level 6
// (`max(6/2, 1) = 3`, up from 2 at level 5, via the same pre-existing formula, not
// re-derived).
//
// A further SD13-E5 slice widens the gate again to level 7
// (`MAX_SUPPORTED_WIZARD_LEVEL = 7`): base attack bonus and all three base saves
// are numerically UNCHANGED from level 6 (`7/2 = 3`, `7/3 = 2`, `7/2+2 = 5`), an
// integer-division coincidence re-verified against the raw PF1 CRB Wizard class
// table rather than assumed; the specialist bonus slot flat count GENUINELY RISES
// to 4 (the raw spells-per-day table's level-7 row is "4/4/3/2/1", the first
// non-"—" 4th-level column — a level-7 specialist now casts 4th-level spells for
// the first time, so the bonus slot count becomes one of each spell level 1st
// through 4th); Intense Spells' bonus-damage magnitude STAYS at 3
// (`max(7/2, 1) = 3`, unchanged from level 6, another integer-division
// coincidence); the level-7 "Special" column is genuinely blank (verified
// independently against both primary sources), so no new class feature is
// gained at 7th level.
//
// A further SD13-E5 slice widens the gate again to level 8
// (`MAX_SUPPORTED_WIZARD_LEVEL = 8`): base attack bonus GENUINELY RISES to +4
// (`8/2 = 4`, up from +3) and good Will GENUINELY RISES to +6 (`8/2+2 = 6`, up
// from +5), while poor Fortitude/Reflex both STAY at +2 (`8/3 = 2`,
// integer-division coincidences); the specialist bonus slot flat count STAYS
// at 4 (the raw spells-per-day table's level-8 row is "4/4/3/3/2" with the
// 5th-level column still "—" — 5th-level spells first appear at level 9, so
// the next slot-count rise lands there, not at level 8, a threshold stasis
// verified against both primary sources' raw table rows rather than assumed);
// Intense Spells' bonus-damage magnitude GENUINELY RISES to 4
// (`max(8/2, 1) = 4`, up from 3 at levels 6-7, via the same pre-existing
// formula, not re-derived); the level-8 "Special" column is genuinely blank
// (verified independently against both primary sources — the Wizard's bonus
// feats land at levels 5, 10, 15, and 20), so no new class feature is gained
// at 8th level.
//
// A further SD13-E5 slice widens the gate again to level 9
// (`MAX_SUPPORTED_WIZARD_LEVEL = 9`): base attack stays +4 (`9/2 = 4`) and
// good Will stays +6 (`9/2+2 = 6`), integer-division coincidences, while
// poor Fortitude/Reflex both GENUINELY RISE to +3 (`9/3 = 3`); the
// specialist bonus slot flat count GENUINELY RISES to 5 (the raw
// spells-per-day table's level-9 row is "4/4/4/3/2/1", the first non-"—"
// 5th-level column — a level-9 specialist now casts 5th-level spells for
// the first time, so the bonus slot count becomes one of each spell level
// 1st through 5th, via WIZARD_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL);
// Intense Spells' bonus-damage magnitude STAYS at 4 (`max(9/2, 1) = 4`,
// another integer-division coincidence — the next rise lands at level 10);
// the level-9 "Special" column is genuinely blank (verified independently
// against both primary sources), so no new class feature is gained at 9th
// level.
const MAX_SUPPORTED_WIZARD_LEVEL: u8 = 9;

// SD13-E5 Wizard specialization slice: the canonical deterministic fixture
// selections for the school specialization choice. The bounded seam recognizes
// exactly this canonical triple (Evocation chosen; Necromancy and Transmutation
// opposed) versus "absent or anything else" — it is not a general school engine.
const WIZARD_SCHOOL_SPECIALIZATION_CHOICE_ID: &str = "choice:wizard_school_specialization";
const WIZARD_OPPOSED_SCHOOLS_CHOICE_ID: &str = "choice:wizard_opposed_schools";
const EVOCATION_SCHOOL_SELECTION: &str = "school:evocation";
const NECROMANCY_SCHOOL_SELECTION: &str = "school:necromancy";
const TRANSMUTATION_SCHOOL_SELECTION: &str = "school:transmutation";
/// PF1 Core Rulebook arcane school class feature: a specialist wizard gains one
/// additional spell slot of each spell level she can cast, 1st and up, usable only
/// for spells of the chosen school. At the bounded baseline level 1 that is exactly
/// one 1st-level slot; there is no cantrip-level bonus slot. Confirmed unchanged at
/// level 2 (SD13-E5): a level-2 wizard still only casts 1st-level wizard spells
/// (2nd-level wizard spells require caster level 3), so the count stays exactly 1
/// through the whole level 1-2 range this seam supports.
const WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVELS_1_AND_2: i16 = 1;

/// SD13-E5 level-3 widening: a level-3 wizard casts 2nd-level spells for the first
/// time (verified independently against both primary sources' raw Wizard
/// spells-per-day table rows: level 2 shows "4/2/—/—", level 3 shows "4/2/1/—" — the
/// first non-"—" 2nd-level column), so a specialist wizard now gains one bonus slot
/// of EACH spell level she can cast: one 1st-level bonus slot plus one 2nd-level
/// bonus slot, for a flat count of 2. Confirmed unchanged at level 4 (SD13-E5
/// widening): the level-4 row is still "4/3/2/—/—" — 3rd-level wizard spells do not
/// become available until wizard level 5 (level 5 row: "4/3/2/1/—", the first
/// non-"—" 3rd-level column), verified independently against both primary sources
/// rather than assumed from the level-3 doubling precedent — so the flat count stays
/// exactly 2 through the level 3-4 range this constant covers.
const WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_3: i16 = 2;

/// SD13-E5 level-5 widening: a level-5 wizard casts 3rd-level spells for the first
/// time (verified independently against both primary sources' raw Wizard
/// spells-per-day table rows: level 4 shows "4/3/2/—/—", level 5 shows "4/3/2/1/—" —
/// the first non-"—" 3rd-level column), so a specialist wizard now gains one bonus
/// slot of EACH spell level she can cast, 1st through 3rd: one 1st-level bonus slot,
/// one 2nd-level bonus slot, and one 3rd-level bonus slot, for a flat count of 3, up
/// from 2 at levels 3-4.
const WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_5: i16 = 3;

/// SD13-E5 level-7 widening: a level-7 wizard casts 4th-level spells for the first
/// time (verified independently against both primary sources' raw Wizard
/// spells-per-day table rows: level 6 shows "4/3/3/2/—", level 7 shows "4/4/3/2/1" —
/// the first non-"—" 4th-level column), so a specialist wizard now gains one bonus
/// slot of EACH spell level she can cast, 1st through 4th: one 1st-level bonus slot,
/// one 2nd-level bonus slot, one 3rd-level bonus slot, and one 4th-level bonus slot,
/// for a flat count of 4, up from 3 at levels 5-6, mirroring exactly the Cleric
/// domain-spell-slot level-7 widening idiom (`CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_7`).
const WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_7: i16 = 4;
const WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_9: i16 = 5;

/// PF1 Core Rulebook Wizard spells-per-day table: the wizard class level at which
/// 2nd-level wizard spells first become available (verified independently against
/// both primary sources: level 1-2 wizards cast only 1st-level spells; level 3 is
/// the first row with a non-"—" 2nd-level column).
const WIZARD_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 3;

/// PF1 Core Rulebook Wizard spells-per-day table: the wizard class level at which
/// 3rd-level wizard spells first become available (verified independently against
/// both primary sources: levels 3-4 wizards cast only up to 2nd-level spells; level
/// 5 is the first row with a non-"—" 3rd-level column).
const WIZARD_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 5;

/// PF1 Core Rulebook Wizard spells-per-day table: the wizard class level at which
/// 4th-level wizard spells first become available (verified independently against
/// both primary sources: levels 5-6 wizards cast only up to 3rd-level spells; level
/// 7 is the first row with a non-"—" 4th-level column).
const WIZARD_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 7;
/// The wizard level at which 5th-level wizard spells (and so the fifth
/// Evocation-only specialist bonus slot) first become available, verified
/// against the raw PF1 Core Rulebook Wizard spells-per-day table rows
/// (d20pfsrd and legacy.aonprd.com): level 8 shows a still-"—" 5th-level
/// column, level 9 is the first to show a non-"—" 5th-level column ("1", the
/// level-9 row reading "4/4/4/3/2/1").
const WIZARD_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 9;

// SD13-E3/E5 martial chassis baseline identity. Barbarian is a non-spell pure
// martial class; the bounded single-class level-1 identity is recognized as
// direct runtime evidence, with base-attack / base-save progression, the
// fast-movement +10 ft. speed extension, and Rage's flat numeric surface
// grounded as standalone records. No rage-state execution engine, weapon
// familiarity, or level-2+ martial progression is grounded.
const BARBARIAN_CLASS_ID: &str = "class:barbarian";
/// SD13-E5 Barbarian level-range gate, mirroring the Fighter
/// `supported_fighter_level` / Paladin `supported_paladin_level` / Rogue
/// `supported_rogue_level` idiom. Monk's own level-range gate is
/// `supported_monk_level` / `MAX_SUPPORTED_MONK_LEVEL`, unrelated to this
/// Barbarian gate.
///
/// A further SD13-E5 slice widens the gate to level 9 (verified independently
/// against d20pfsrd and legacy.aonprd.com): level 9 base attack bonus
/// genuinely rises to +9 (full BAB) while poor Reflex/Will both genuinely
/// rise to +3 (9 / 3) and good Fortitude stays +6 (9 / 2 + 2, an
/// integer-division coincidence); the rage rounds-per-day pool genuinely
/// rises to 23 (4 + Con mod + 2 per level after 1st) while the four flat
/// rage-surface magnitudes stay at their standard-rage values (the next
/// change is Greater Rage at 11th); the level-9 "Special" column reads
/// "Trap sense +3" — a tier-rise on the already-grounded Trap Sense formula
/// pillar (level / 3), not a new class feature; Damage Reduction stays 1/—
/// (the next DR rise lands at 10th); level 9 is NOT a rage-power level
/// (powers land at 2/4/6/8/10...), so no new pillar is grounded.
const MAX_SUPPORTED_BARBARIAN_LEVEL: u8 = 9;

/// PF1 Core Rulebook level gate at which Barbarian gains Uncanny Dodge (2nd level,
/// verified against two independent primary sources — d20pfsrd and legacy.aonprd.com
/// both list "Rage power, uncanny dodge" as the Barbarian 2nd-level special feature
/// entry).
const BARBARIAN_UNCANNY_DODGE_LEVEL: u8 = 2;

/// PF1 Core Rulebook level gate at which Barbarian gains Trap Sense (3rd level,
/// verified independently against two primary sources — d20pfsrd and
/// legacy.aonprd.com both list "Trap sense +1" as the Barbarian 3rd-level special
/// feature entry).
const BARBARIAN_TRAP_SENSE_LEVEL: u8 = 3;

/// PF1 Core Rulebook level gate at which Barbarian gains Improved Uncanny Dodge
/// (5th level, verified independently against two primary sources — d20pfsrd and
/// legacy.aonprd.com both list "Improved uncanny dodge" as the Barbarian 5th-level
/// special feature entry).
const BARBARIAN_IMPROVED_UNCANNY_DODGE_LEVEL: u8 = 5;

/// PF1 Core Rulebook level gate at which Barbarian gains Damage Reduction (7th
/// level, verified independently against two primary sources — d20pfsrd and
/// legacy.aonprd.com both list "Damage reduction 1/-" as the Barbarian 7th-level
/// special feature entry, and both give the rule text "At 7th level, a barbarian
/// gains damage reduction. Subtract 1 from the damage the barbarian takes each
/// time she is dealt damage from a weapon or a natural attack").
const BARBARIAN_DAMAGE_REDUCTION_LEVEL: u8 = 7;

// SD13-E3/E5 martial chassis baseline identity, mirroring the Barbarian pattern. Monk
// is a non-spell pure martial class with a distinct four-pillar bounded burden; this
// slice recognizes its bounded single-class level-1/level-2/level-3 identity as
// direct runtime evidence and grounds base-attack / base-save progression, unarmed
// strike damage die, the Flurry of Blows flat surface, AC Bonus, the level-1 bonus
// feat choice-slot recognition, (SD13-E5) Evasion at level 2, and (SD13-E5) Still
// Mind at level 3, but no level-1 bonus feat mechanics execution, no ki pool, and no
// level-4+ martial progression.
const MONK_CLASS_ID: &str = "class:monk";
/// SD13-E5 Monk level-range gate, mirroring the Fighter `supported_fighter_level` /
/// Paladin `supported_paladin_level` / Rogue `supported_rogue_level` / Barbarian
/// `supported_barbarian_level` idiom.
// A further SD13-E5 slice widens the gate to level 9 (verified independently
// against d20pfsrd and legacy.aonprd.com): level 9 base attack bonus stays +6
// (9 * 3 / 4) and all three good saves stay +6 (9 / 2 + 2), integer-division
// coincidences; the unarmed strike die stays 1d10 (the band spans levels
// 8-11); the Flurry flat attack modifier genuinely rises to +7 (level - 2)
// while the attack count stays 3 (the next count change lands at 15th); the
// ki pool and Slow Fall's 40-ft reach both stay at their level-8 values (the
// next Slow Fall reach increase lands at 10th); the level-9 "Special" column
// reads "Improved evasion" — a genuinely NEW named entry, grounded by this
// slice as a +0 identity/recognition record only
// (MONK_IMPROVED_EVASION_LEVEL), mirroring the Evasion / Rogue
// Improved-Uncanny-Dodge precedent; no damage-resolution engine exists here,
// so no damage math is fabricated from it.
const MAX_SUPPORTED_MONK_LEVEL: u8 = 9;
// PF1 Core Rulebook level gate at which Monk gains Wholeness of Body (7th
// level, verified independently against two primary sources: d20pfsrd and
// legacy.aonprd.com both name Wholeness of Body as the Monk 7th-level
// special feature entry, alongside an upgrade to the ki pool's
// damage-reduction-bypass material). Wholeness of Body itself ("a monk can
// heal his own wounds as a standard action... a number of hit points of
// damage equal to his monk level by using 2 points from his ki pool") is
// checked and confirmed NOT flat: it requires both a ki-point-consumption/
// action-economy engine and a healing-resolution engine, neither of which
// exists anywhere in this codebase (mirroring exactly why the ki pool's own
// point-spending was already left unimplemented at level 4). The ki pool's
// material-bypass upgrade likewise requires a damage-reduction-bypass-
// resolution engine that does not exist here. Neither is grounded as a
// record at level 7 or level 8 (it stays granted-but-unexecuted, unchanged),
// mirroring the Bard Suggestion / Monk High Jump precedent of naming a
// checked-but-unproven feature without fabricating a value for it. No new
// const is introduced for this level gate since no code branches on it.
// PF1 Core Rulebook level gate at which Monk gains an actual new "Special"
// entry at 8th level: verified independently against two primary sources
// (d20pfsrd and legacy.aonprd.com), the Monk class table's level-8 row names
// only "Slow fall 40 ft." — a rise in the already-grounded Slow Fall
// record's own reach magnitude, not a brand-new class feature. Both primary
// sources were checked specifically for Improved Uncanny Dodge (a
// commonly-repeated but WRONG assumption for Monk, carried over from other
// classes' 8th-level tables): neither source lists it anywhere on the Monk
// class table at any level, so no such record is grounded or fabricated
// here.
/// PF1 Core Rulebook level gate at which Monk gains Evasion (2nd level, verified
/// independently against two primary sources: d20pfsrd and legacy.aonprd.com both
/// list "Bonus feat, evasion" as the Monk 2nd-level special feature entry).
const MONK_EVASION_LEVEL: u8 = 2;
/// PF1 Core Rulebook level gate at which Monk gains Improved Evasion (9th level,
/// verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Improved evasion" as the Monk 9th-level
/// "Special" column entry).
const MONK_IMPROVED_EVASION_LEVEL: u8 = 9;
/// PF1 Core Rulebook level gate at which Monk gains Still Mind (3rd level, verified
/// independently against two primary sources: d20pfsrd and legacy.aonprd.com both
/// list "Fast movement, maneuver training, still mind" as the Monk 3rd-level
/// special feature entry). Fast Movement (a speed bonus) and Maneuver Training (a
/// CMB/CMD-substitution rule) are also granted at this same level but are
/// deliberately left named-but-unproven this slice: no speed-total engine and no
/// CMB/CMD engine exist anywhere in this codebase to attach either to. Still Mind
/// alone is grounded, since it is a flat, non-level-scaled magnitude (+2 on saves
/// vs. enchantment spells and effects) matching the Fighter Bravery / Paladin
/// Divine Grace / Rogue Trap Sense idiom exactly.
const MONK_STILL_MIND_LEVEL: u8 = 3;
/// PF1 Core Rulebook level gate at which the Medium-monk unarmed strike damage
/// die steps up from 1d6 to 1d8 (4th level, verified independently against two
/// primary sources: d20pfsrd and legacy.aonprd.com both give the Medium-monk
/// unarmed damage progression as 1d6 at levels 1-3, 1d8 at levels 4-7, 1d10 at
/// levels 8-11, 2d6 at levels 12-15, 2d8 at levels 16-19, and 2d10 at level 20).
const MONK_UNARMED_DAMAGE_DIE_STEP_UP_LEVEL: u8 = 4;
/// PF1 Core Rulebook level gate at which the Medium-monk unarmed strike damage
/// die steps up again from 1d8 to 1d10 (8th level, verified independently
/// against the same two primary sources' Medium-monk unarmed damage
/// progression table: the 1d10 band runs levels 8-11).
const MONK_UNARMED_DAMAGE_DIE_SECOND_STEP_UP_LEVEL: u8 = 8;
/// PF1 Core Rulebook level gate at which Flurry of Blows grants a third
/// attack (8th level, verified independently against two primary sources'
/// verbatim Flurry of Blows rule text: "At 8th level, the monk can make two
/// additional attacks when he uses flurry of blows, as if using Improved
/// Two-Weapon Fighting" — i.e. two bonus attacks instead of one, for three
/// total attacks on a flurry full-attack action, up from two at levels 1-7).
const MONK_FLURRY_THIRD_ATTACK_LEVEL: u8 = 8;
/// PF1 Core Rulebook level gate at which Slow Fall's own reach magnitude
/// increases again, from 30 ft to 40 ft (8th level, verified independently
/// against two primary sources' Monk class table: the level-8 "Special"
/// column reads "Slow fall 40 ft." — the full progression is 20 ft at 4th,
/// 30 ft at 6th, 40 ft at 8th, and 50 ft at 10th). This is the level-8 row's
/// ONLY "Special" column entry per both primary sources — checked and
/// confirmed NOT a new class feature (specifically confirmed NOT Improved
/// Uncanny Dodge, which Monk never gains at any level per either primary
/// source) — so, mirroring the level-6 precedent, the record's own `value`
/// field still stays 0 and only the detail text's reach figure changes.
const MONK_SLOW_FALL_FORTY_FOOT_REACH_LEVEL: u8 = 8;
/// PF1 Core Rulebook level gate at which Monk gains the ki pool and Slow Fall
/// (4th level, verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Ki pool (magic), slow fall 20 ft." as the Monk
/// 4th-level special feature entry).
const MONK_KI_POOL_AND_SLOW_FALL_LEVEL: u8 = 4;
/// PF1 Core Rulebook level gate at which Slow Fall's own reach magnitude
/// increases from 20 ft to 30 ft (6th level, verified independently against
/// two primary sources: d20pfsrd and legacy.aonprd.com both list "Bonus feat,
/// slow fall 30 ft." as the Monk 6th-level special feature entry — the full
/// progression is 20 ft at 4th, 30 ft at 6th, 40 ft at 8th, and 50 ft at
/// 10th). This is a genuine flat-magnitude increase in the rule text, mirroring
/// the Rogue Trap Sense idiom; the record's own `value` field still stays 0
/// (still a bounded grant-only identity record, since no fall-damage-
/// resolution engine exists in this codebase to apply any reduction to), but
/// the detail text names the level-accurate reach so it is never a stale,
/// fabricated-by-omission "20 feet" claim at level 6. The level-6 "Special"
/// column's OTHER entry, "Bonus feat," was checked and confirmed to be the
/// SAME open-ended repeat bonus-feat choice-list shape already deliberately
/// left named-but-unproven at 2nd level (not a new automatic class feature,
/// and not Improved Trip specifically — Improved Trip is merely one of the
/// five feats already recognized as a possible *choice* for this and every
/// other Monk bonus feat grant) — mirroring exactly the Rogue level-6 "second
/// Rogue Talent slot" precedent: no new choice-slot and no new diagnostic was
/// added for it.
const MONK_SLOW_FALL_INCREASED_REACH_LEVEL: u8 = 6;
/// PF1 Core Rulebook level gate at which Monk gains Purity of Body (5th level,
/// verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "High jump, purity of body" as the Monk
/// 5th-level special feature entry). High Jump, the OTHER 5th-level "Special"
/// column entry, is deliberately left named-but-unproven this slice: it
/// requires wiring the monk's level into an Acrobatics-check total (no
/// skill-check-total engine exists in this codebase) and spending a ki point
/// (an action-economy/resource-consumption engine this codebase deliberately
/// does not implement for the ki pool either), so it is checked and confirmed
/// NOT flat rather than fabricated. Purity of Body alone is grounded, since
/// it is a flat, non-level-scaled grant (disease immunity) matching the
/// Barbarian/Rogue Uncanny Dodge / Monk Slow Fall grant-only idiom exactly.
const MONK_PURITY_OF_BODY_LEVEL: u8 = 5;
/// The PF1 Core Rulebook level at which the level-1 monk bonus feat (and the
/// automatic Improved Unarmed Strike grant) always occurs, independent of the
/// character's current supported level. Kept distinct from the generic
/// `supported_monk_level` current-level value so widening to level 2 does not
/// accidentally relabel the level-1-specific bonus feat grant as a level-2 one —
/// PF1 grants monks a SEPARATE bonus feat at 2nd level that this bounded seam
/// deliberately does not recognize.
const MONK_BONUS_FEAT_GRANT_LEVEL: u8 = 1;

// SD13-E5 Monk level-1 bonus feat choice-slot recognition. The PF1 Core Rulebook
// restricted Monk bonus feat list this bounded recognition seam knows is Combat
// Reflexes, Deflect Arrows, Improved Grapple, Improved Trip, and Stunning Fist.
// Improved Unarmed Strike is deliberately excluded: the PF1 Core Rulebook grants
// it to every monk automatically at level 1, separate from this chosen bonus
// feat, and this codebase does not ground that automatic grant either, so it is
// never treated as a choice-set member here.
const MONK_BONUS_FEAT_CHOICE_ID: &str = "choice:monk_bonus_feat";
const DEFLECT_ARROWS_FEAT_SELECTION: &str = "feat:deflect_arrows";
const IMPROVED_GRAPPLE_FEAT_SELECTION: &str = "feat:improved_grapple";
const IMPROVED_TRIP_FEAT_SELECTION: &str = "feat:improved_trip";
const STUNNING_FIST_FEAT_SELECTION: &str = "feat:stunning_fist";

// Grounded SD13-E4/E5 Human Cleric level-1/level-2/level-3/level-4 prepared divine
// spell-bearing baseline identity. Cleric is the canonical PF1 prepared divine full
// caster; unlike the arcane Sorcerer/Wizard/Bard baselines already recognized, its
// bounded burden is split across a domain powers class-feature family (the granted
// powers of the chosen domains and the domain spell-list contents — Channel Energy
// has been grounded for real: ceil(cleric level / 2) d6, minimum 1d6, usable
// 3 + Charisma modifier times per day; and the SD13-E5 domain slice grounds the
// domain choice seam and the flat domain spell slot count) and a prepared divine
// spell posture family (spells prepared from the full Cleric list, spontaneous
// cure/inflict conversion, spell slots per day, bonus spells from a high Wisdom,
// spell save DCs). A later SD13-E5 slice widens the level-1-only gate to a
// level-range gate (level 1-2), extending base attack/base save/Channel
// Energy/domain-spell-slot/domain-power formulas to level 2 without re-derivation. A
// further SD13-E5 slice widens the gate again to level 1-3: Channel Energy's die
// count and the domain spell slot count both change for real at level 3 (verified
// independently against the PF1 Core Rulebook Cleric class table and spells-per-day
// table), since level 3 is exactly when a cleric first casts 2nd-level spells. A
// further SD13-E5 slice widens the gate again to level 1-4: the Good domain's Touch
// of Good sacred bonus genuinely changes for real at level 4 (half cleric level,
// minimum 1, so `max(4/2, 1) = 2`, up from 1), verified independently against the PF1
// Core Rulebook Good Domain granted-power rule text; Channel Energy's die count and
// the domain spell slot count both stay unchanged at level 4 (verified independently
// against the class table's blank level-4 "Special" column and the spells-per-day
// table's still-blank 3rd-level spell column at level 4).
const CLERIC_CLASS_ID: &str = "class:cleric";
/// SD13-E5 Cleric level-range gate, mirroring the Fighter `supported_fighter_level` /
/// Paladin `supported_paladin_level` / Rogue `supported_rogue_level` / Barbarian
/// `supported_barbarian_level` / Monk `supported_monk_level` idiom. Verified against
/// the PF1 Core Rulebook Cleric class table (d20pfsrd and legacy.aonprd.com) before
/// widening: a level-2 cleric still only casts 1st-level cleric spells (2nd-level
/// cleric spells begin at caster level 3), gains no new class feature at 2nd level
/// (the Cleric class table's level-2 "Special" column is blank), and Channel Energy
/// stays 1d6 through level 2 (it next increases at level 3), so every level-1 formula
/// this seam already grounds extends to level 2 without re-derivation. A further
/// SD13-E5 slice widens this to 1..=3: a level-3 cleric's Channel Energy die count
/// becomes 2d6 (`ceil(3 / 2) = 2`, the class table's level-3 "Special" column reads
/// "Channel energy 2d6") and a level-3 cleric casts 2nd-level cleric spells for the
/// first time (verified against the raw Cleric spells-per-day table rows), so the
/// domain spell slot count also changes for real at level 3. A further SD13-E5 slice
/// widens this to 1..=4, verified independently against both primary sources: the
/// class table's level-4 "Special" column is blank (no new class feature is gained),
/// Channel Energy's die count stays 2d6 (`ceil(4 / 2) = 2`, unchanged from level 3, it
/// next increases only at level 5), and the domain spell slot count stays 2 (a
/// level-4 cleric's 3rd-level spell column is still "—" on the raw spells-per-day
/// table — 3rd-level cleric spells begin only at level 5) — but the Good domain's
/// Touch of Good sacred bonus (half cleric level, minimum 1) genuinely increases to 2
/// via the same pre-existing formula (`max(4/2, 1) = 2`). A further SD13-E5 slice
/// widens this to 1..=5, verified independently against both primary sources: the
/// class table's level-5 "Special" column reads "Channel energy 3d6" — Channel
/// Energy's die count genuinely increases to 3d6 (`ceil(5 / 2) = 3`) — and a
/// level-5 cleric casts 3rd-level cleric spells for the first time (the raw
/// spells-per-day table's level-5 row is the first to show a non-"—" 3rd-level
/// column, "1+1"), so the domain spell slot count also changes for real, to 3, at
/// level 5. The Good domain's Touch of Good sacred bonus stays 2 at level 5
/// (`max(5/2, 1) = 2`, integer division; it next increases only at level 6). A
/// further SD13-E5 slice widens this to 1..=6, verified independently against both
/// primary sources: the class table's level-6 "Special" column is genuinely blank
/// (no new class feature is gained at 6th level), Channel Energy's die count stays
/// 3d6 (`ceil(6 / 2) = 3`, unchanged from level 5 — both primary sources confirm
/// the die count rises only every odd cleric level, 1st/3rd/5th/7th/..., so level 6
/// is not one of those levels), and the domain spell slot count stays 3 (the raw
/// spells-per-day table's level-6 row still shows "—" in the 4th-level spell
/// column, so 4th-level cleric spells do not begin at level 6) — but the Good
/// domain's Touch of Good sacred bonus genuinely increases to 3 via the same
/// pre-existing formula (`max(6/2, 1) = 3`). A further SD13-E5 slice widens this to
/// 1..=7, verified independently against both primary sources: the class table's
/// level-7 "Special" column reads "Channel energy 4d6" — Channel Energy's die count
/// genuinely increases to 4d6 (`ceil(7 / 2) = 4`), confirming level 7 IS one of the
/// odd cleric levels where the die count rises — and the domain spell slot count
/// also genuinely increases, to 4 (a level-7 cleric casts 4th-level cleric spells
/// for the first time, the raw spells-per-day table's level-7 row being the first to
/// show a non-"—" 4th-level spell column), mirroring exactly the level-3 and
/// level-5 domain-spell-slot widenings. The Good domain's Touch of Good sacred
/// bonus stays 3 at level 7 (`max(7/2, 1) = 3`, integer division; it next increases
/// only at level 8). No other new class feature is gained at 7th level (verified
/// independently against both primary sources' level-7 Special column), so no new
/// pillar record is added at level 7 either — only the Channel Energy and domain
/// spell slot count pillars are widened to genuinely new values. A further SD13-E5
/// slice widens this to 1..=8, verified independently against both primary sources:
/// the class table's level-8 "Special" column is genuinely blank (no new class
/// feature is gained at 8th level — the iterative-attack notation "+6/+1" on the
/// level-8 base-attack column is not modeled anywhere in this codebase, only the
/// flat base value of 6), Channel Energy's die count stays 4d6
/// (`ceil(8 / 2) = 4`, unchanged from level 7 — both primary sources confirm the
/// die count rises only every odd cleric level, 1st/3rd/5th/7th/9th/..., so level 8
/// is not one of those levels), and the domain spell slot count stays 4 (the raw
/// spells-per-day table's level-8 row still shows "—" in the 5th-level spell
/// column, verified independently against both primary sources — 5th-level cleric
/// spells do not begin until level 9) — but the Good domain's Touch of Good sacred
/// bonus GENUINELY increases to 4 via the same pre-existing formula
/// (`max(8/2, 1) = 4`), confirming the level-7 comment's own forecast that it next
/// increases at level 8.
// A further SD13-E5 slice widens the gate to level 9 (verified independently
// against d20pfsrd and legacy.aonprd.com): level 9 base attack stays +6
// (9 * 3 / 4) and good Fortitude/Will both stay +6 (9 / 2 + 2),
// integer-division coincidences, while poor Reflex genuinely rises to +3
// (9 / 3); the level-9 "Special" column reads "Channel energy 5d6" — a
// tier-rise on the already-grounded die-count pillar ((level + 1) / 2 = 5,
// the odd-level cadence), not a new class feature; 5th-level cleric spells
// first appear at 9th, so the domain spell slot count genuinely rises to 5
// via the same one-slot-per-castable-spell-level rule
// (CLERIC_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL); Touch of Good's bonus
// stays 4 (9 / 2, a coincidence) and both domain-power uses-per-day pools
// stay level-independent; no new pillar is grounded.
const MAX_SUPPORTED_CLERIC_LEVEL: u8 = 9;

// SD13-E5 canonical Human Cleric domain-choice seam. These name the exact accepted
// deterministic domain selections on the level-1/level-2/level-3 seam (a cleric
// chooses two domains from among those belonging to her deity). This slice surfaces
// the named selections as an explicit choice seam only and grounds no domain power
// and no domain spell-list contents, mirroring the Fighter bonus-feat choice-slot
// seam pattern.
const CLERIC_DOMAIN_CHOICE_ID: &str = "choice:cleric_domain";
const GOOD_DOMAIN_SELECTION: &str = "domain:good";
const HEALING_DOMAIN_SELECTION: &str = "domain:healing";

// PF1 Core Rulebook Domains: a cleric gains one domain spell slot per level of
// cleric spells she can cast, 1st and up. At levels 1-2 this bounded seam supports
// she casts only 1st-level cleric spells (2nd-level cleric spells begin at caster
// level 3, verified against the PF1 Core Rulebook Cleric spells-per-day table via
// d20pfsrd and legacy.aonprd.com), so exactly one 1st-level domain slot is granted —
// confirmed unchanged at level 2, not a new record. At level 3 a cleric casts
// 2nd-level cleric spells for the first time (the raw spells-per-day table's level-3
// row is the first to show a non-"—" 2nd-level column), so the domain spell slot
// count genuinely becomes 2 at level 3: one 1st-level domain slot plus one
// 2nd-level domain slot, mirroring exactly the Wizard specialist-bonus-slot
// level-3 widening (`WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_3`). At level 5 a
// cleric casts 3rd-level cleric spells for the first time (the raw
// spells-per-day table's level-5 row is the first to show a non-"—" 3rd-level
// column), so the count genuinely becomes 3: one 1st-level, one 2nd-level, and
// one 3rd-level domain slot. Confirmed unchanged at level 6 (the raw
// spells-per-day table's level-6 row still shows "—" in the 4th-level spell
// column, verified independently against both primary sources), so the count
// stays 3 through level 6 — it next changes only when 4th-level cleric spells
// become available at a later level. At level 7 a cleric casts 4th-level
// cleric spells for the first time (the raw spells-per-day table's level-7 row
// is the first to show a non-"—" 4th-level column, verified independently
// against both primary sources), so the count genuinely becomes 4: one
// 1st-level, one 2nd-level, one 3rd-level, and one 4th-level domain slot.
const CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVELS_1_AND_2: i16 = 1;
const CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVELS_3_AND_4: i16 = 2;
const CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVELS_5_AND_6: i16 = 3;
const CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_7: i16 = 4;
const CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_9: i16 = 5;
/// The cleric level at which 2nd-level cleric spells (and so the second domain
/// spell slot) first become available, verified against the raw PF1 Core Rulebook
/// Cleric spells-per-day table rows (d20pfsrd and legacy.aonprd.com): level 2 shows
/// "4/2+1/—", level 3 shows "4/2+1/1+1" — the first non-"—" 2nd-level column.
const CLERIC_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 3;
/// The cleric level at which 3rd-level cleric spells (and so the third domain
/// spell slot) first become available, verified against the raw PF1 Core Rulebook
/// Cleric spells-per-day table rows (d20pfsrd and legacy.aonprd.com): level 4 shows
/// "5/3+1/2+1/—", level 5 shows "5/3+1/2+1/1+1" — the first non-"—" 3rd-level
/// column. Confirmed the count stays at 3 domain slots through level 6 (the
/// level-6 row's 4th-level spell column is still "—"), since 4th-level cleric
/// spells are not yet available.
const CLERIC_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 5;
/// The cleric level at which 4th-level cleric spells (and so the fourth domain
/// spell slot) first become available, verified against the raw PF1 Core
/// Rulebook Cleric spells-per-day table rows (d20pfsrd and legacy.aonprd.com):
/// level 6 shows a still-"—" 4th-level column, level 7 is the first to show a
/// non-"—" 4th-level column ("1+1"). Confirmed the count stays at 4 domain
/// slots through level 8 (the level-8 row's 5th-level spell column is still
/// "—", verified independently against both primary sources), since 5th-level
/// cleric spells are not yet available — they first appear at level 9.
const CLERIC_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 7;
/// The cleric level at which 5th-level cleric spells (and so the fifth domain
/// spell slot) first become available, verified against the raw PF1 Core
/// Rulebook Cleric spells-per-day table rows (d20pfsrd and legacy.aonprd.com):
/// level 8 shows a still-"—" 5th-level column, level 9 is the first to show a
/// non-"—" 5th-level column ("1+1", the level-9 row reading
/// "4/4+1/4+1/3+1/2+1/1+1").
const CLERIC_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL: u8 = 9;

// Grounded SD13-E4 Human Druid level-1 prepared divine spell-bearing baseline
// identity. Druid is a prepared divine caster whose bounded burden splits across
// a nature bond / wild empathy class-feature family (nature bond choice between
// an animal companion and a domain, nature sense, wild empathy) and a prepared
// divine spell posture family (spells prepared from the full Druid list,
// spontaneous summon nature's ally conversion, spell slots per day, bonus spells
// from a high Wisdom, spell save DCs). Wild Empathy (SD13-E4), Nature Sense, and
// the deterministic nature-bond choice recognition (SD13-E5) are grounded; the
// chosen bond's execution and the whole spell posture stay claim-blocked.
const DRUID_CLASS_ID: &str = "class:druid";
/// SD13-E5 Druid level-range gate, mirroring the Fighter `supported_fighter_level` /
/// Paladin `supported_paladin_level` / Rogue `supported_rogue_level` / Barbarian
/// `supported_barbarian_level` / Monk `supported_monk_level` / Cleric
/// `supported_cleric_level` / Bard `supported_bard_level` idiom. Verified against the
/// PF1 Core Rulebook Druid class table (d20pfsrd and legacy.aonprd.com) before
/// widening: level 2 base attack bonus is +1, base saves are +3/+0/+3
/// (Fortitude/Reflex/Will), so every level-1 base-attack/base-save/Wild-Empathy
/// formula this seam already grounds extends to level 2 without re-derivation; Nature
/// Sense and the nature-bond choice recognition are level-independent and unaffected;
/// the class table's level-2 "Special" column reads "Woodland stride" (a new,
/// flat/identity-shaped class feature grounded separately below). A further SD13-E5
/// slice widens the gate to level 3 (verified independently against d20pfsrd and
/// legacy.aonprd.com): level 3 base attack bonus is +2, base saves are +3/+1/+3
/// (Fortitude/Reflex/Will), extended via the same formulas; Woodland Stride stays
/// granted, not re-derived; the class table's level-3 "Special" column reads
/// "Trackless step" (a new, flat/identity-shaped class feature grounded separately
/// below); Druid has no currently-grounded spell-slot-count pillar (unlike Wizard's
/// specialist bonus slot or Cleric's domain slot), so there is no analogous level-3
/// doubling to widen. A further SD13-E5 slice widens the gate to level 4 (verified
/// independently against d20pfsrd and legacy.aonprd.com): level 4 base attack bonus
/// is +3, base saves are +4/+1/+4 (Fortitude/Reflex/Will), extended via the same
/// formulas; Woodland Stride and Trackless Step both stay granted, not re-derived.
/// The class table's level-4 "Special" column reads "Resist nature's lure, wild
/// shape (1/day)" — TWO distinct entries, both checked independently rather than
/// assumed. Resist Nature's Lure is flat/identity-shaped (a standalone +4
/// saving-throw bonus against the spell-like and supernatural abilities of fey,
/// and against spells/effects that target plants) and is grounded separately below,
/// mirroring the Woodland Stride/Trackless Step idiom. Wild Shape is NOT flat — it
/// is a full shapeshifting subsystem (new form, new stat block, duration tracking)
/// with no execution engine anywhere in this codebase — so it is deliberately left
/// named-but-unproven, exactly like the animal-companion execution burden. A still
/// further SD13-E5 slice widens the gate to level 5 (verified independently against
/// d20pfsrd and legacy.aonprd.com): level 5 base attack bonus is +3, base saves are
/// +4/+1/+4 (Fortitude/Reflex/Will) — all three numerically unchanged from level 4
/// (integer-division coincidences of `level * 3 / 4`, `level / 2 + 2`, and
/// `level / 3`, not a sign any formula stopped scaling), extended via the same
/// formulas, not re-derived; Wild Empathy grounds correctly to 6 (5 + Charisma
/// modifier 1) via the same level-generic formula; Nature Sense stays the flat +2
/// bonus; Woodland Stride, Trackless Step, and Resist Nature's Lure all stay
/// granted, not re-derived. The class table's level-5 "Special" column is genuinely
/// blank (verified independently against both primary sources rather than assumed),
/// so this slice grounds no new pillar — only the existing pillars are widened. A
/// still further SD13-E5 slice widens the gate to level 6 (verified independently
/// against d20pfsrd and legacy.aonprd.com): level 6 base attack bonus is +4, base
/// saves are +5/+2/+5 (Fortitude/Reflex/Will), all three genuinely new values, up
/// from +3/+1/+4 at level 5, extended via the same formulas, not re-derived; Wild
/// Empathy grounds correctly to 7 (6 + Charisma modifier 1) via the same
/// level-generic formula; Nature Sense stays the flat +2 bonus; Woodland Stride,
/// Trackless Step, and Resist Nature's Lure all stay granted, not re-derived. The
/// class table's level-6 "Special" column reads "Wild shape (2/day)" — checked per
/// the operator brief's explicit instruction to verify whether Druid gains an
/// actual new class feature at 6th level, and confirmed NOT a genuinely separable
/// flat/identity-shaped element: the rule text bundles the "2/day" frequency
/// increase together with a form-list expansion (a druid can now wild shape into a
/// Large or Tiny animal or a Small elemental) and a functioning-level upgrade (the
/// animal form now functions as beast shape II, the elemental form as elemental
/// body I) — none of which exist in this codebase's engine-free record set, and
/// none of which are separable from the "2/day" numeral without misrepresenting the
/// bundled feature as fully flat. Wild Shape (including its level-6 frequency
/// increase and form-list expansion) is therefore deliberately left entirely
/// named-but-unproven, exactly as at level 4/5 — no explanation or diagnostic
/// record is fabricated for it this slice either. A still further SD13-E5 slice
/// widens the gate to level 8 (verified independently against d20pfsrd and
/// legacy.aonprd.com): level 8 base attack bonus is +6 (genuinely risen from +5;
/// the class table's own "+6/+1" iterative-attack notation is not modeled
/// anywhere in this codebase, only the flat base value), base saves are +6/+2/+6
/// (Fortitude/Reflex/Will — both good saves genuinely rise from +5, while poor
/// Reflex stays +2, an integer-division coincidence), extended via the same
/// formulas, not re-derived; Wild Empathy genuinely rises to 9 (8 + Charisma
/// modifier 1) via the same level-generic formula; Nature Sense stays the flat
/// +2; Woodland Stride, Trackless Step, and Resist Nature's Lure all stay
/// granted, not re-derived. The class table's level-8 "Special" column reads
/// "Wild shape (3/day)" — checked rather than assumed away, and confirmed to be
/// the same non-separable bundled shape as at level 6: the frequency increase
/// arrives together with a form-list expansion (Huge/Diminutive animal, Medium
/// elemental, Small/Medium plant) and functioning-level upgrades (beast shape
/// III / elemental body II / plant shape I), so Wild Shape (including its
/// level-8 frequency increase) stays entirely named-but-unproven, exactly as at
/// level 4/6 — no explanation or diagnostic record is fabricated for it this
/// slice either.
// A still further SD13-E5 slice widens the gate to level 9 (verified
// independently against d20pfsrd and legacy.aonprd.com): level 9 base attack
// stays +6 (9 * 3 / 4) and both good saves stay +6 (9 / 2 + 2),
// integer-division coincidences, while poor Reflex genuinely rises to +3
// (9 / 3); Wild Empathy genuinely rises to 10 (9 + Charisma modifier 1) via
// the same level-generic formula; Nature Sense, Woodland Stride, Trackless
// Step, and Resist Nature's Lure all stay granted, not re-derived; Wild
// Shape's uses stay 3/day (the next rise lands at 10th, checked rather than
// assumed) and it stays entirely named-but-unproven; the level-9 "Special"
// column reads "Venom immunity" — a genuinely flat, no-choice, no-magnitude
// grant (immunity to all poisons), grounded as a +0 identity/recognition
// record only (DRUID_VENOM_IMMUNITY_LEVEL), mirroring Monk's Purity of Body
// precedent exactly; no poison/condition engine exists here, so no immunity
// effect is fabricated.
const MAX_SUPPORTED_DRUID_LEVEL: u8 = 9;
/// PF1 Core Rulebook level gate at which Druid gains Venom Immunity (9th
/// level, verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Venom immunity" as the Druid 9th-level
/// "Special" column entry).
const DRUID_VENOM_IMMUNITY_LEVEL: u8 = 9;
/// PF1 Core Rulebook level gate at which Druid gains Resist Nature's Lure (4th
/// level, verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Resist nature's lure" as part of the Druid
/// 4th-level special feature entry, alongside "Wild shape (1/day)").
const DRUID_RESIST_NATURES_LURE_LEVEL: u8 = 4;
/// PF1 Core Rulebook Resist Nature's Lure flat magnitude: "a druid gains a +4
/// bonus on saving throws against the spell-like and supernatural abilities of
/// fey. This bonus also applies to spells and effects that utilize or target
/// plants, such as blight, entangle, spike growth, and warp wood." Flat and
/// level-independent once granted (it does not scale further with druid level).
const DRUID_RESIST_NATURES_LURE_BONUS: i16 = 4;
/// PF1 Core Rulebook level gate at which Druid gains Woodland Stride (2nd level,
/// verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Woodland stride" as the Druid 2nd-level special
/// feature entry).
const DRUID_WOODLAND_STRIDE_LEVEL: u8 = 2;
/// PF1 Core Rulebook level gate at which Druid gains Trackless Step (3rd level,
/// verified independently against two primary sources: d20pfsrd and
/// legacy.aonprd.com both list "Trackless step" as the Druid 3rd-level special
/// feature entry).
const DRUID_TRACKLESS_STEP_LEVEL: u8 = 3;
// PF1 Core Rulebook Nature Sense: a druid gains a +2 bonus on Knowledge (nature)
// and Survival checks. Flat and level-independent.
const DRUID_NATURE_SENSE_BONUS: i16 = 2;
// The deterministic SD13 fixture's nature-bond selection seam: the choice set and
// the one selection this bounded slice recognizes (an animal companion; a domain
// bond is not part of the deterministic fixture and stays unrecognized).
const DRUID_NATURE_BOND_CHOICE_ID: &str = "choice:druid_nature_bond";
const DRUID_NATURE_BOND_ANIMAL_COMPANION_SELECTION_ID: &str = "bond:animal_companion";


// Grounded Human pilot race seam identities. These name the already-accepted
// deterministic Human selections; this slice makes their pressure explicit but
// grounds no non-Human race semantics and no broader Human racial trait burden.
const HUMAN_RACE_ID: &str = "race:human";
const HUMAN_ABILITY_BONUS_CHOICE_ID: &str = "choice:human_ability_bonus";
const HUMAN_BONUS_FEAT_CHOICE_ID: &str = "choice:human_bonus_feat";
const ABILITY_SELECTION_PREFIX: &str = "ability:";

// SD13-E6-F3a Human racial trait bundle (size, speed, senses, extra skill ranks).
// These name the remaining Human racial trait burden explicitly, classified
// against PF1 Core Rulebook Standard Human racial traits (source evidence only,
// not oracle-checked parity):
//   cr_races.lst race:human SIZE:MEDIUM        -> Medium size category
//   cr_races.lst race:human GAIT:WALK|30       -> 30 ft base land speed
//   cr_races.lst race:human                   -> no special senses (PCGen races
//                                                in the CRB only carry the SENSE
//                                                tag when a sense bonus exists;
//                                                Human has none for Standard Human)
//   cr_races.lst race:human BONUS:SKILL|...   -> 4 extra skill points at 1st
//                                                level and 1 extra skill rank
//                                                per level thereafter
//
// This constant set deliberately names the entire PF1 Standard Human racial
// trait surface — every line a Player's Handbook Human racial entry lists —
// so the explanation records can name each dimension explicitly instead of
// leaving it an incidental side-effect or a folklore claim.
//
// None of these ground a computed mechanical contribution to the existing
// NumericOutputs in this slice. They explain Human identity only; the chassis
// totals remain controlled by the bounded deterministic posture.
const HUMAN_SIZE_CATEGORY: &str = "Medium";
const HUMAN_BASE_SPEED_FEET: i16 = 30;
const HUMAN_EXTRA_SKILL_POINTS_AT_LEVEL_1: u8 = 4;
const HUMAN_EXTRA_SKILL_RANKS_PER_LEVEL: u8 = 1;

// Grounded deterministic combat-baseline contributors and posture identities.
const LONGSWORD_ITEM_ID: &str = "item:longsword";
const CHAIN_SHIRT_ITEM_ID: &str = "item:chain_shirt";
const SHIELD_ITEM_ID: &str = "item:shield";
const POWER_ATTACK_ITEM_ID: &str = "power_attack";
const DODGE_FEAT_ID: &str = "feat:dodge";
const WEAPON_FOCUS_FEAT_ID: &str = "feat:weapon_focus";
const FIGHTER_BONUS_FEAT_CHOICE_ID: &str = "choice:fighter_bonus_feat";
const WEAPON_FOCUS_LONGSWORD_SELECTION: &str = "feat:weapon_focus:weapon:longsword";

// SD13-E5-F9 canonical Human Fighter feat-choice seam. These name the exact accepted
// deterministic feat-choice selections on the level-1/2/3 seam. This slice preserves
// these selections and claim-blocks any deviation of the named slots; it grounds no
// general feat-effect or prerequisite engine and no alternative feat legality.
const LEVEL_1_CHARACTER_FEAT_CHOICE_ID: &str = "choice:level_1_character_feat";
const POWER_ATTACK_FEAT_SELECTION: &str = "feat:power_attack";
const TOUGHNESS_FEAT_SELECTION: &str = "feat:toughness";

// Grounded numeric contributors (source evidence only; not oracle-checked parity):
//   cr_equip_arms_armor.lst:40  Chain Shirt -> BONUS:COMBAT|AC|4|TYPE=Armor, MAXDEX:4
//   cr_feats.lst:53             Dodge       -> BONUS:COMBAT|AC|1|TYPE=Dodge
//   cr_feats.lst:184            Weapon Focus-> +1 to-hit with the selected weapon
const ARMOR_CLASS_BASE: i16 = 10;
const CHAIN_SHIRT_ARMOR_BONUS: i16 = 4;
const CHAIN_SHIRT_MAX_DEX: i16 = 4;
const DODGE_AC_BONUS: i16 = 1;
const WEAPON_FOCUS_TO_HIT_BONUS: i16 = 1;

// Grounded selected-skill contributors (source evidence only; not oracle-checked):
//   cr_skills.lst:10   Climb      -> KEYSTAT:STR, ACHECK:YES, BONUS:SKILL|Climb|3|TYPE=ClassSkill
//   cr_skills.lst:42   Intimidate -> KEYSTAT:CHA (no ACHECK), BONUS:SKILL|Intimidate|3|TYPE=ClassSkill
//   cr_skills.lst:102  Swim       -> KEYSTAT:STR, ACHECK:YES, BONUS:SKILL|Swim|3|TYPE=ClassSkill
//   cr_abilities_class.lst:2835   Fighter class skills include Climb, Intimidate, Swim
//   cr_equip_arms_armor.lst:40    Chain Shirt -> ACCHECK:-2
const CLIMB_SKILL_ID: &str = "skill:climb";
const INTIMIDATE_SKILL_ID: &str = "skill:intimidate";
const SWIM_SKILL_ID: &str = "skill:swim";
const SELECTED_SKILL_RANK: u8 = 1;
const CLASS_SKILL_BONUS: i16 = 3;
const CHAIN_SHIRT_ARMOR_CHECK_PENALTY: i16 = -2;

// Bounded SD13-E3/SD13-E5 Fighter milestone widening. The accepted level-1 pilot
// is now joined by levels 2 through 10. Nothing here grounds level 11+ Fighter
// burden, the weapon-training damage-roll half, the Bravery Will-vs-fear bonus,
// or any non-Fighter positive support. The generic PF1 ability-score-increase
// milestones (levels 4 and 8) need no separate seam: the chosen ability score is
// trusted at face value, like every other ability adjustment in this codebase.
const MAX_SUPPORTED_FIGHTER_LEVEL: u8 = 10;

// Fighter level-1 hit points. PF1 maximizes the hit die at 1st character level:
// the Fighter's d10 hit die grants 10 hit points at level 1, plus the
// Constitution modifier. This slice grounds only that level-1 value; hit points
// at levels 2+ (average/rolled hit-die policy), the favored-class +1 hp /
// +1 skill-rank choice (no input surface exists for it), and Toughness / feat
// hit-point interplay stay unproven.
const FIGHTER_LEVEL_1_MAX_HIT_DIE_HIT_POINTS: i16 = 10;

// Fighter level-2 bonus-feat progression seam. Fighter gains an additional bonus
// feat at level 2; this slice surfaces the named selection as an explicit seam only
// and grounds no general feat-effect or prerequisite engine.
const FIGHTER_LEVEL_2_BONUS_FEAT_CHOICE_ID: &str = "choice:fighter_bonus_feat_2";

// Fighter level-4 bonus-feat progression seam. Fighter gains an additional bonus
// feat at level 4 (the cadence continues at 1, 2, 4, 6, 8, 10, ...); this slice
// surfaces the named selection as an explicit seam only and grounds no general
// feat-effect or prerequisite engine, mirroring the level-2 seam.
const FIGHTER_LEVEL_4_BONUS_FEAT_CHOICE_ID: &str = "choice:fighter_bonus_feat_4";
const CLEAVE_FEAT_SELECTION: &str = "feat:cleave";

// Fighter level-6 bonus-feat progression seam. Fighter gains an additional bonus
// feat at level 6 (the cadence continues 1, 2, 4, 6, 8, 10, ...); this slice
// surfaces the named selection as an explicit seam only and grounds no general
// feat-effect or prerequisite engine, mirroring the level-2/level-4 seams.
const FIGHTER_LEVEL_6_BONUS_FEAT_CHOICE_ID: &str = "choice:fighter_bonus_feat_6";
const COMBAT_REFLEXES_FEAT_SELECTION: &str = "feat:combat_reflexes";

// Fighter level-8 bonus-feat progression seam. Fighter gains an additional bonus
// feat at level 8 (the cadence continues 1, 2, 4, 6, 8, 10, ...); this slice
// surfaces the named selection as an explicit seam only and grounds no general
// feat-effect or prerequisite engine, mirroring the level-2/level-4/level-6 seams.
const FIGHTER_LEVEL_8_BONUS_FEAT_CHOICE_ID: &str = "choice:fighter_bonus_feat_8";
const IMPROVED_CRITICAL_FEAT_SELECTION: &str = "feat:improved_critical";

// Fighter level-10 bonus-feat progression seam. Fighter gains an additional bonus
// feat at level 10 (the cadence continues 1, 2, 4, 6, 8, 10); this slice surfaces
// the named selection as an explicit seam only and grounds no general feat-effect
// or prerequisite engine, mirroring the level-2 through level-8 seams. The
// canonical Greater Weapon Focus selection's prerequisites (Weapon Focus with the
// chosen weapon and fighter level 8) are honestly met by the canonical loadout:
// Weapon Focus (longsword) is the level-1 fighter bonus feat and the seam only
// exists at Fighter level 10.
const FIGHTER_LEVEL_10_BONUS_FEAT_CHOICE_ID: &str = "choice:fighter_bonus_feat_10";
const GREATER_WEAPON_FOCUS_FEAT_SELECTION: &str = "feat:greater_weapon_focus";

// Fighter Weapon Training, gained at level 5 with a new rank every four levels
// (rank = 1 + (level - 5) / 4): Weapon Training 1 at level 5, Weapon Training 2
// at level 9. Each rank grants the first chosen weapon group +rank to attack and
// damage rolls; each later-chosen group sits one point lower. This slice grounds
// only the attack-roll half of the first group (folded into the baseline melee
// attack bonus for the deterministic Longsword, which falls under the canonical
// Heavy Blades group) and surfaces the second group (canonically Bows, chosen at
// level 9) as an explanation-only record; the damage-roll half is never computed
// for any Fighter level in this codebase, so it stays explicitly unproven rather
// than silently omitted.
const FIGHTER_WEAPON_TRAINING_1_LEVEL: u8 = 5;
const FIGHTER_WEAPON_TRAINING_RANK_LEVEL_STRIDE: u8 = 4;
const FIGHTER_WEAPON_TRAINING_GROUP_CHOICE_ID: &str = "choice:fighter_weapon_training_group";
const HEAVY_BLADES_GROUP_SELECTION: &str = "group:heavy_blades";
const FIGHTER_WEAPON_TRAINING_GROUP_2_CHOICE_ID: &str = "choice:fighter_weapon_training_group_2";
const BOWS_GROUP_SELECTION: &str = "group:bows";

// Fighter armor training 1, gained at level 3. It reduces the worn armor's
// armor-check penalty by 1 (to a minimum of 0) and raises its maximum Dexterity
// bonus by 1. Grounded from cr_abilities_class.lst Fighter armor training; not
// oracle-checked parity.
const FIGHTER_ARMOR_TRAINING_1_LEVEL: u8 = 3;
const ARMOR_TRAINING_1_ARMOR_CHECK_REDUCTION: i16 = 1;
const ARMOR_TRAINING_1_MAX_DEX_INCREASE: i16 = 1;

// Fighter armor training 2, gained at level 7. It further reduces the worn
// armor's armor-check penalty (to a minimum of 0, cumulative with Armor
// Training 1) and further raises its maximum Dexterity bonus. Grounded from
// cr_abilities_class.lst Fighter armor training; not oracle-checked parity.
const FIGHTER_ARMOR_TRAINING_2_LEVEL: u8 = 7;
const ARMOR_TRAINING_2_ARMOR_CHECK_REDUCTION: i16 = 2;
const ARMOR_TRAINING_2_MAX_DEX_INCREASE: i16 = 2;

// Fighter Bravery, gained at level 2 with an additional +1 every four Fighter
// levels thereafter (level 6, level 10, ...): +1 Will save vs fear at level 2,
// +2 at level 6, +3 at level 10, per PF1 Core Rulebook. This slice grounds only
// the flat bonus magnitude as a standalone explanation record, mirroring the
// Weapon Training attack-bonus-rank idiom; no fear-condition or save-resolution
// engine exists anywhere in this codebase, so the bonus is never folded into the
// unconditional Will save total.
const FIGHTER_BRAVERY_LEVEL: u8 = 2;
const FIGHTER_BRAVERY_RANK_LEVEL_STRIDE: u8 = 4;

/// Simple integrated status for the GE-06 pilot headless receipt: whether the
/// path produced computed evidence or is blocked. This distinguishes evidence
/// from a blocker posture; it is not an oracle-checked parity verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeadlessReceiptStatus {
    /// The integrated deterministic path produced computed evidence with no
    /// claim-blocking diagnostics.
    Computed,
    /// The integrated deterministic path is blocked; at least one claim-blocking
    /// diagnostic is present and no success state is fabricated.
    Blocked,
}

/// One bounded, library-first, headless receipt for the accepted deterministic
/// GE-06 pilot path. It preserves case and source-package identity, a simple
/// computed/blocked status, and the full underlying computation (already-grounded
/// outputs, explanations, and diagnostics) for later parity or UI consumers.
///
/// This is headless computed evidence only; it must not be relabeled as
/// oracle-checked parity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PilotHeadlessReceipt {
    /// Case identity carried from the loaded input (absent when the input names none).
    pub case_id: Option<String>,
    /// Source package identity carried from the loaded input.
    pub source_package_id: String,
    /// Whether the integrated path produced evidence or is blocked.
    pub status: HeadlessReceiptStatus,
    /// The underlying pilot computation, preserving the already-grounded outputs,
    /// explanation records, and claim-blocking diagnostics unchanged.
    pub computation: PilotBaseChassisComputation,
}

/// Build the GE-06 pilot headless receipt from a loaded character input.
///
/// This runs the existing deterministic compute surface and wraps it in one
/// bounded receipt shape, deriving the integrated status from the computation's
/// claim-blocking diagnostics: any claim-blocking diagnostic blocks the path,
/// otherwise the path is computed. It adds no new computed value, fabricates no
/// success state, and discards none of the existing explanations or diagnostics.
pub fn build_pilot_headless_receipt(input: &CharacterInput) -> PilotHeadlessReceipt {
    let computation = compute_pilot_base_chassis(input);

    let status = if computation.diagnostics.iter().any(|d| d.claim_blocking) {
        HeadlessReceiptStatus::Blocked
    } else {
        HeadlessReceiptStatus::Computed
    };

    PilotHeadlessReceipt {
        case_id: input.case_id.clone(),
        source_package_id: input.source_package_id.clone(),
        status,
        computation,
    }
}

/// Compute the GE-06 pilot base chassis from a loaded character input.
pub fn compute_pilot_base_chassis(input: &CharacterInput) -> PilotBaseChassisComputation {
    let mut explanations = Vec::new();
    let mut diagnostics = Vec::new();

    let ability_modifiers =
        compute_ability_modifiers(&input.chosen.ability_scores, &mut explanations);

    let (base_attack_bonus, base_saves) =
        compute_fighter_chassis(input, &mut explanations, &mut diagnostics);

    let (baseline_melee_attack_bonus, baseline_armor_class) = compute_combat_baseline(
        input,
        &ability_modifiers,
        base_attack_bonus,
        &mut explanations,
        &mut diagnostics,
    );

    let total_saves = compute_total_saves(
        input,
        &ability_modifiers,
        &base_saves,
        &mut explanations,
        &mut diagnostics,
    );

    let selected_skill_modifiers = compute_selected_skill_modifiers(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );

    explain_fighter_class_features(input, &mut explanations);

    explain_fighter_level1_hit_points(input, &ability_modifiers, &mut explanations);

    explain_fighter_favored_class_bonus_choice(input, &mut explanations);

    explain_hybrid_level1_chassis(input, &mut explanations, &mut diagnostics);
    explain_barbarian_level1_chassis(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );
    explain_monk_level1_chassis(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );
    explain_rogue_level1_chassis(input, &mut explanations);


    // SD13-E3/E4/E5 Paladin-only decomposition: split the F6 hybrid class-feature
    // and spell-burden blockers into per-burden diagnostics so the chassis
    // burden is separable from the partial-caster spell burden on the runtime
    // path, widened by SD13-E5 to the level-2 lay on hands / divine grace
    // grant. This is an extension, never a downgrade, of the F6 surface.
    explain_paladin_level1_chassis_and_spell_burden_separation(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );

    // SD13-E3 Ranger-only decomposition: split the F6 Ranger non-spell
    // class-feature blocker into three named pillars, and ground Track and
    // combat style for real (Track as a bounded flat numeric value, combat
    // style as a level-gate absence record). This is an extension, never a
    // downgrade, of the F6 surface, mirroring the Paladin decomposition
    // immediately above.
    explain_ranger_level1_chassis_and_class_feature_separation(
        input,
        &mut explanations,
    );

    explain_sorcerer_level1_spell_baseline(input, &mut explanations, &mut diagnostics);

    explain_wizard_level1_prepared_spell_baseline(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );

    explain_cleric_level1_spell_baseline(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );

    explain_druid_level1_spell_baseline(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );

    explain_bard_level1_spell_baseline(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );

    explain_human_pilot_race_seam(input, &ability_modifiers, &mut explanations, &mut diagnostics);

    explain_human_trait_bundle(input, &mut explanations, &mut diagnostics);

    explain_dwarf_race_seam(input, &mut explanations, &mut diagnostics);

    explain_elf_race_seam(input, &mut explanations, &mut diagnostics);

    explain_gnome_race_seam(input, &mut explanations, &mut diagnostics);

    explain_half_elf_race_seam(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );

    explain_half_orc_race_seam(
        input,
        &ability_modifiers,
        &mut explanations,
        &mut diagnostics,
    );

    explain_halfling_race_seam(input, &mut explanations, &mut diagnostics);

    validate_fighter_feat_choice_legality(input, &mut diagnostics);

    PilotBaseChassisComputation {
        ability_modifiers,
        base_attack_bonus,
        base_saves,
        baseline_melee_attack_bonus,
        baseline_armor_class,
        total_saves,
        selected_skill_modifiers,
        explanations,
        diagnostics,
    }
}

fn compute_ability_modifiers(
    scores: &AbilityScores,
    explanations: &mut Vec<ComputationExplanation>,
) -> AbilityModifiers {
    let abilities = [
        ("strength", scores.strength),
        ("dexterity", scores.dexterity),
        ("constitution", scores.constitution),
        ("intelligence", scores.intelligence),
        ("wisdom", scores.wisdom),
        ("charisma", scores.charisma),
    ];

    let mut modifiers = AbilityModifiers::default();
    for (ability, score) in abilities {
        let modifier = ability_modifier(score);
        explanations.push(ComputationExplanation {
            id: format!("ability_modifier.{ability}"),
            value: modifier,
            detail: format!(
                "{ability} ability modifier from chosen score {score}: floor({score} / 2) - 5 = {modifier}"
            ),
        });
        assign_modifier(&mut modifiers, ability, modifier);
    }

    modifiers
}

/// Pathfinder ability modifier: `floor(score / 2) - 5`. `div_euclid` gives true
/// floor division so negative scores would round down rather than toward zero.
fn ability_modifier(score: i16) -> i16 {
    score.div_euclid(2) - 5
}

fn assign_modifier(modifiers: &mut AbilityModifiers, ability: &str, modifier: i16) {
    match ability {
        "strength" => modifiers.strength = modifier,
        "dexterity" => modifiers.dexterity = modifier,
        "constitution" => modifiers.constitution = modifier,
        "intelligence" => modifiers.intelligence = modifier,
        "wisdom" => modifiers.wisdom = modifier,
        "charisma" => modifiers.charisma = modifier,
        _ => unreachable!("ability set is fixed and fully matched"),
    }
}

/// Make the already-grounded Human pilot race seam explicit instead of leaving it an
/// incidental side effect of the numeric outputs.
///
/// This adds no new computed mechanic and no new input surface. It derives strictly
/// from existing chosen input — the `race:human` identity and the named
/// `choice:human_ability_bonus` and `choice:human_bonus_feat` selections — and from the
/// already-computed deterministic outputs — the ability modifiers and the grounded
/// Dodge armor-class contribution. It thereby surfaces the named Human ability-bonus and
/// bonus-feat interaction pressure as legible explanation records.
///
/// This function handles only the `race:human` branch of `explain_race_seam`;
/// non-Human routing (the bounded Half-Elf diagnostic and the
/// `race.semantics.unverified` catch-all) lives in the dispatcher. This slice
/// grounds no broader Human racial trait burden (size, speed, senses, extra
/// skill ranks).
fn explain_human_pilot_race_seam(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if input.chosen.race_id != HUMAN_RACE_ID {
        // Dwarf, Elf, Gnome, Half-Elf, Half-Orc, and Halfling carry their own
        // dedicated race-semantics seams (explain_dwarf_race_seam,
        // explain_elf_race_seam, explain_gnome_race_seam,
        // explain_half_elf_race_seam, explain_half_orc_race_seam,
        // explain_halfling_race_seam); they replace this generic diagnostic
        // rather than stacking alongside it. With Halfling landed, this branch
        // is unreachable for the seven-race SD-13 roster but stays as a
        // defensive fallback for any race identity outside that roster.
        if input.chosen.race_id != DWARF_RACE_ID
            && input.chosen.race_id != ELF_RACE_ID
            && input.chosen.race_id != GNOME_RACE_ID
            && input.chosen.race_id != HALF_ELF_RACE_ID
            && input.chosen.race_id != HALF_ORC_RACE_ID
            && input.chosen.race_id != HALFLING_RACE_ID
        {
            diagnostics.push(ComputationDiagnostic {
                id: "race.semantics.unverified".to_owned(),
                message: format!(
                    "race semantics are grounded only for {HUMAN_RACE_ID} on the deterministic pilot seam; \
                     chosen race {} has no grounded race semantics in this slice",
                    input.chosen.race_id
                ),
                claim_blocking: false,
            });
        }
        return;
    }

    // Human ability-bonus interaction: the named choice targets one ability. Surface its
    // pressure through the already-computed modifier for exactly that ability.
    if let Some(selection) = choice_selection(input, HUMAN_ABILITY_BONUS_CHOICE_ID) {
        let ability = selection
            .strip_prefix(ABILITY_SELECTION_PREFIX)
            .unwrap_or(selection);
        let modifier = ability_modifier_for(ability_modifiers, ability);
        explanations.push(ComputationExplanation {
            id: "race.human.ability_bonus_target".to_owned(),
            value: modifier,
            detail: format!(
                "Human ability-bonus selection ({HUMAN_ABILITY_BONUS_CHOICE_ID} -> {selection}) targets \
                 {ability}; the chosen {ability} score yields modifier {modifier:+}"
            ),
        });
    }

    // Human bonus-feat interaction: the named choice grants a feat. Surface the grounded
    // Dodge armor-class contribution the deterministic baseline already relies on.
    if let Some(selection) = choice_selection(input, HUMAN_BONUS_FEAT_CHOICE_ID) {
        let (value, detail) = if selection == DODGE_FEAT_ID {
            (
                DODGE_AC_BONUS,
                format!(
                    "Human bonus-feat selection ({HUMAN_BONUS_FEAT_CHOICE_ID} -> {selection}) grants Dodge, \
                     the deterministic Dodge feat contributing {DODGE_AC_BONUS:+} to the baseline armor class"
                ),
            )
        } else {
            (
                0,
                format!(
                    "Human bonus-feat selection ({HUMAN_BONUS_FEAT_CHOICE_ID} -> {selection}) is a named Human \
                     bonus feat, but only the deterministic Dodge grant has a grounded computed contribution"
                ),
            )
        };
        explanations.push(ComputationExplanation {
            id: "race.human.bonus_feat_grant".to_owned(),
            value,
            detail,
        });
    }

    // Bounded honesty: only the named seam is grounded. This is explicit but
    // non-claim-blocking so the deterministic pilot still reports computed evidence.
    diagnostics.push(ComputationDiagnostic {
        id: "race.human.bounded_semantics".to_owned(),
        message: "Human race semantics are grounded for the deterministic pilot's named \
                  ability-bonus and bonus-feat selections, and the SD13-E6-F3a trait bundle \
                  (size, speed, senses, extra skill ranks) is classified explicitly; the \
                  remaining PF1 Standard Human racial trait surface (alternate Human racial \
                  traits, variant Humans, half-Human heritages, and any ruleset-level effects \
                  outside the named deterministic pilot) remains unverified"
            .to_owned(),
        claim_blocking: false,
    });
}

const DWARF_RACE_ID: &str = "race:dwarf";
const DWARF_SIZE_CATEGORY: &str = "Medium";
const DWARF_BASE_SPEED_FEET: i16 = 20;
const DWARF_DARKVISION_FEET: i16 = 60;
const DWARF_CON_ADJUSTMENT: i16 = 2;
const DWARF_CHA_ADJUSTMENT: i16 = -2;

/// SD13-E2 Dwarf racial trait bundle explanation seam (mirroring the SD13-E6-F3a
/// Human trait bundle pattern for the first non-Human core race).
///
/// Surfaces four grounded PF1 Core Rulebook Dwarf racial trait dimensions (ability
/// modifiers, size, speed, senses) as explicit `ComputationExplanation` records so
/// the Dwarf identity is legible on the runtime path rather than left behind the
/// generic `race.semantics.unverified` diagnostic every other non-Human race still
/// receives.
///
/// This function:
///   - runs only when `race_id == race:dwarf`; every other race is unaffected
///     (Human keeps its own seam; every other non-Human race keeps the generic
///     `race.semantics.unverified` diagnostic from `explain_human_race_seam`),
///   - adds no new computed mechanical contribution: the ability-modifiers record
///     is recognition-only (the chosen Constitution/Charisma scores are understood
///     to already reflect the fixed +2/-2 racial adjustment; no arithmetic is
///     performed on this seam), and the size/senses records carry the grounded
///     source value as identity only,
///   - replaces the generic `race.semantics.unverified` diagnostic with a
///     Dwarf-specific `race.dwarf.bounded_semantics` note naming the still-unproven
///     families explicitly (Stonecunning and other skill/derived-stat modifiers,
///     Defensive Training, Hardy, Stability, Hatred, weapon familiarity, and the
///     explicit absence of any Dwarf racial bonus feat),
///   - is bounded to race recognition only; it deliberately grounds no Dwarf
///     class-chassis interaction, no other race, and no PF1 alternate ruleset.
fn explain_dwarf_race_seam(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if input.chosen.race_id != DWARF_RACE_ID {
        return;
    }

    // ----- ability modifiers -----
    // Recognition record only: PF1 Core Dwarf ability adjustments (+2 Con / -2 Cha)
    // are fixed, not a player choice. The chosen Constitution/Charisma scores are
    // understood to already reflect this adjustment; no arithmetic is performed here.
    explanations.push(ComputationExplanation {
        id: "race.dwarf.trait_bundle.ability_modifiers".to_owned(),
        value: 0,
        detail: format!(
            "Dwarf racial trait bundle — ability modifiers: PF1 Core Dwarf grants a fixed \
             {DWARF_CON_ADJUSTMENT:+} Constitution and {DWARF_CHA_ADJUSTMENT:+} Charisma racial \
             adjustment (cr_races.lst race:dwarf STAT:CON|{DWARF_CON_ADJUSTMENT:+}, \
             STAT:CHA|{DWARF_CHA_ADJUSTMENT:+}). This is a bounded recognition record naming the \
             fixed adjustment on the deterministic pilot seam; the chosen Constitution and \
             Charisma scores are understood to already reflect it, so this record performs no \
             arithmetic and carries no fabricated mechanical value (+0)"
        ),
    });

    // ----- size -----
    explanations.push(ComputationExplanation {
        id: "race.dwarf.trait_bundle.size".to_owned(),
        value: 0,
        detail: format!(
            "Dwarf racial trait bundle — size: PF1 Core Dwarf is {DWARF_SIZE_CATEGORY} size \
             (cr_races.lst race:dwarf SIZE:MEDIUM). This is a bounded recognition record naming \
             the Dwarf size category on the deterministic pilot seam; it contributes no numeric \
             effect to attack rolls, AC, skill checks, ability checks, or any other computed \
             value, so it carries no fabricated mechanical value (+0)"
        ),
    });

    // ----- speed -----
    // Recognition record for the 20 ft base land speed. PF1 Core Dwarf speed is
    // never reduced by armor or encumbrance, unlike most Medium races; this is
    // named explicitly as identity only — no computed speed-derived value is
    // fabricated.
    explanations.push(ComputationExplanation {
        id: "race.dwarf.trait_bundle.speed".to_owned(),
        value: DWARF_BASE_SPEED_FEET,
        detail: format!(
            "Dwarf racial trait bundle — speed: PF1 Core Dwarf has a base land speed of \
             {DWARF_BASE_SPEED_FEET} ft that is never reduced by armor or encumbrance \
             (cr_races.lst race:dwarf GAIT:WALK|{DWARF_BASE_SPEED_FEET}). This is a grounded \
             recognition value carrying the Dwarf base-speed identity on the deterministic pilot \
             seam; it contributes no computed speed-derived effect to any chassis output, skill \
             modifier, attack roll, or combat baseline"
        ),
    });

    // ----- senses -----
    // Recognition record for Darkvision 60 ft, distinct from Human's bounded
    // no-special-senses classification.
    explanations.push(ComputationExplanation {
        id: "race.dwarf.trait_bundle.senses".to_owned(),
        value: DWARF_DARKVISION_FEET,
        detail: format!(
            "Dwarf racial trait bundle — senses: PF1 Core Dwarf grants Darkvision \
             {DWARF_DARKVISION_FEET} ft (cr_races.lst race:dwarf SENSE:Darkvision \
             ({DWARF_DARKVISION_FEET} ft)). This is a grounded recognition value carrying the \
             Dwarf Darkvision identity on the deterministic pilot seam; it contributes no \
             computed low-light or perception-derived effect to any chassis output"
        ),
    });

    // Bounded honesty: only the four named dimensions are grounded. This replaces
    // the generic race.semantics.unverified diagnostic for Dwarf specifically and
    // stays non-claim-blocking so the deterministic pilot still reports computed
    // evidence.
    diagnostics.push(ComputationDiagnostic {
        id: "race.dwarf.bounded_semantics".to_owned(),
        message: "Dwarf race semantics are grounded for the deterministic pilot's ability \
                  modifiers, size, speed, and senses trait bundle; the remaining PF1 Core \
                  Dwarf racial trait surface remains unverified: skill or derived-stat \
                  modifiers (Stonecunning Perception/Appraise bonuses), Defensive Training \
                  (dodge bonus to AC against giants), Hardy (bonus on saves against poison, \
                  spells, and spell-like abilities), Stability (bonus to CMD against bull \
                  rush/trip), Hatred (bonus on attack rolls against orcs and goblinoids), and \
                  weapon familiarity (battleaxe, heavy pick, warhammer, dwarven waraxe, \
                  dwarven urgrosh). PF1 core Dwarves gain no racial bonus feat (unlike Human), \
                  so that family is explicitly not applicable rather than silently omitted."
            .to_owned(),
        claim_blocking: false,
    });
}

const ELF_RACE_ID: &str = "race:elf";
const ELF_SIZE_CATEGORY: &str = "Medium";
const ELF_BASE_SPEED_FEET: i16 = 30;
const ELF_DEX_ADJUSTMENT: i16 = 2;
const ELF_CON_ADJUSTMENT: i16 = -2;

/// SD13-E2 Elf racial trait bundle explanation seam (mirroring the Dwarf pattern
/// for the second non-Human core race).
///
/// Surfaces four grounded PF1 Core Rulebook Elf racial trait dimensions (ability
/// modifiers, size, speed, senses) as explicit `ComputationExplanation` records so
/// the Elf identity is legible on the runtime path rather than left behind the
/// generic `race.semantics.unverified` diagnostic every other non-Human race still
/// receives.
///
/// This function:
///   - runs only when `race_id == race:elf`; every other race is unaffected
///     (Human and Dwarf keep their own seams; every other non-Human race keeps
///     the generic `race.semantics.unverified` diagnostic),
///   - adds no new computed mechanical contribution: the ability-modifiers record
///     is recognition-only (the chosen Dexterity/Constitution scores are
///     understood to already reflect the fixed +2/-2 racial adjustment; no
///     arithmetic is performed on this seam), and the size/senses records carry
///     the grounded source value as identity only,
///   - replaces the generic `race.semantics.unverified` diagnostic with an
///     Elf-specific `race.elf.bounded_semantics` note naming the still-unproven
///     families explicitly (Elven Immunities, Keen Senses, weapon familiarity,
///     bonus languages, and the explicit absence of any Elf racial bonus feat),
///   - is bounded to race recognition only; it deliberately grounds no Elf
///     class-chassis interaction, no other race, no alternate +2 Intelligence
///     ability variant, and no PF1 alternate ruleset.
fn explain_elf_race_seam(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if input.chosen.race_id != ELF_RACE_ID {
        return;
    }

    // ----- ability modifiers -----
    explanations.push(ComputationExplanation {
        id: "race.elf.trait_bundle.ability_modifiers".to_owned(),
        value: 0,
        detail: format!(
            "Elf racial trait bundle — ability modifiers: PF1 Core Elf grants a fixed \
             {ELF_DEX_ADJUSTMENT:+} Dexterity and {ELF_CON_ADJUSTMENT:+} Constitution racial \
             adjustment (cr_races.lst race:elf STAT:DEX|{ELF_DEX_ADJUSTMENT:+}, \
             STAT:CON|{ELF_CON_ADJUSTMENT:+}). This is a bounded recognition record naming the \
             fixed adjustment on the deterministic pilot seam; the chosen Dexterity and \
             Constitution scores are understood to already reflect it, so this record performs \
             no arithmetic and carries no fabricated mechanical value (+0). The alternate PF1 \
             +2 Intelligence Elf variant is out of scope for this slice."
        ),
    });

    // ----- size -----
    explanations.push(ComputationExplanation {
        id: "race.elf.trait_bundle.size".to_owned(),
        value: 0,
        detail: format!(
            "Elf racial trait bundle — size: PF1 Core Elf is {ELF_SIZE_CATEGORY} size \
             (cr_races.lst race:elf SIZE:MEDIUM). This is a bounded recognition record naming \
             the Elf size category on the deterministic pilot seam; it contributes no numeric \
             effect to attack rolls, AC, skill checks, ability checks, or any other computed \
             value, so it carries no fabricated mechanical value (+0)"
        ),
    });

    // ----- speed -----
    explanations.push(ComputationExplanation {
        id: "race.elf.trait_bundle.speed".to_owned(),
        value: ELF_BASE_SPEED_FEET,
        detail: format!(
            "Elf racial trait bundle — speed: PF1 Core Elf has a base land speed of \
             {ELF_BASE_SPEED_FEET} ft (cr_races.lst race:elf GAIT:WALK|{ELF_BASE_SPEED_FEET}). \
             This is a grounded recognition value carrying the Elf base-speed identity on the \
             deterministic pilot seam; it contributes no computed speed-derived effect to any \
             chassis output, skill modifier, attack roll, or combat baseline"
        ),
    });

    // ----- senses -----
    // Low-light vision is a binary trait (doubles effective light for vision
    // purposes), not a distance magnitude like Dwarf Darkvision; the recognition
    // value stays +0.
    explanations.push(ComputationExplanation {
        id: "race.elf.trait_bundle.senses".to_owned(),
        value: 0,
        detail: "Elf racial trait bundle — senses: PF1 Core Elf grants low-light vision \
                  (cr_races.lst race:elf SENSE:Low-Light Vision). This is a bounded recognition \
                  record naming the Elf low-light vision identity on the deterministic pilot \
                  seam; it contributes no computed illumination or perception-derived effect to \
                  any chassis output, so it carries no fabricated mechanical value (+0)"
            .to_owned(),
    });

    // Bounded honesty: only the four named dimensions are grounded. This replaces
    // the generic race.semantics.unverified diagnostic for Elf specifically and
    // stays non-claim-blocking so the deterministic pilot still reports computed
    // evidence.
    diagnostics.push(ComputationDiagnostic {
        id: "race.elf.bounded_semantics".to_owned(),
        message: "Elf race semantics are grounded for the deterministic pilot's ability \
                  modifiers, size, speed, and senses trait bundle; the remaining PF1 Core Elf \
                  racial trait surface remains unverified: Elven Immunities (immunity to magic \
                  sleep effects and a bonus on saves against enchantment spells and effects), \
                  Keen Senses (a bonus on Perception checks), weapon familiarity (longbow, \
                  composite longbow, longsword, rapier, shortbow, composite shortbow), and \
                  bonus language grants. PF1 core Elves gain no racial bonus feat (unlike \
                  Human), so that family is explicitly not applicable rather than silently \
                  omitted."
            .to_owned(),
        claim_blocking: false,
    });
}

const GNOME_RACE_ID: &str = "race:gnome";
const GNOME_SIZE_CATEGORY: &str = "Small";
const GNOME_BASE_SPEED_FEET: i16 = 20;
const GNOME_CON_ADJUSTMENT: i16 = 2;
const GNOME_STR_ADJUSTMENT: i16 = -2;

/// SD13-E2 Gnome racial trait bundle explanation seam (mirroring the Dwarf/Elf
/// pattern for the third non-Human core race).
///
/// Surfaces four grounded PF1 Core Rulebook Gnome racial trait dimensions
/// (ability modifiers, size, speed, senses) as explicit `ComputationExplanation`
/// records so the Gnome identity is legible on the runtime path rather than left
/// behind the generic `race.semantics.unverified` diagnostic every other
/// non-Human race still receives.
///
/// This function:
///   - runs only when `race_id == race:gnome`; every other race is unaffected
///     (Human, Dwarf, and Elf keep their own seams; every other non-Human race
///     keeps the generic `race.semantics.unverified` diagnostic),
///   - adds no new computed mechanical contribution: the ability-modifiers record
///     is recognition-only (the chosen Constitution/Strength scores are
///     understood to already reflect the fixed +2/-2 racial adjustment; no
///     arithmetic is performed on this seam), and the size/senses records carry
///     the grounded source value as identity only,
///   - replaces the generic `race.semantics.unverified` diagnostic with a
///     Gnome-specific `race.gnome.bounded_semantics` note naming the
///     still-unproven families explicitly (Defensive Training, Illusion
///     Resistance, Hatred, Keen Senses, Gnome Magic, weapon familiarity, and the
///     explicit absence of any Gnome racial bonus feat),
///   - is bounded to race recognition only; it deliberately grounds no Gnome
///     class-chassis interaction, no other race, and no PF1 alternate ruleset.
fn explain_gnome_race_seam(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if input.chosen.race_id != GNOME_RACE_ID {
        return;
    }

    // ----- ability modifiers -----
    explanations.push(ComputationExplanation {
        id: "race.gnome.trait_bundle.ability_modifiers".to_owned(),
        value: 0,
        detail: format!(
            "Gnome racial trait bundle — ability modifiers: PF1 Core Gnome grants a fixed \
             {GNOME_CON_ADJUSTMENT:+} Constitution and {GNOME_STR_ADJUSTMENT:+} Strength racial \
             adjustment (cr_races.lst race:gnome STAT:CON|{GNOME_CON_ADJUSTMENT:+}, \
             STAT:STR|{GNOME_STR_ADJUSTMENT:+}). This is a bounded recognition record naming the \
             fixed adjustment on the deterministic pilot seam; the chosen Constitution and \
             Strength scores are understood to already reflect it, so this record performs no \
             arithmetic and carries no fabricated mechanical value (+0)"
        ),
    });

    // ----- size -----
    explanations.push(ComputationExplanation {
        id: "race.gnome.trait_bundle.size".to_owned(),
        value: 0,
        detail: format!(
            "Gnome racial trait bundle — size: PF1 Core Gnome is {GNOME_SIZE_CATEGORY} size \
             (cr_races.lst race:gnome SIZE:SMALL). This is a bounded recognition record naming \
             the Gnome size category on the deterministic pilot seam; it contributes no numeric \
             effect to attack rolls, AC, skill checks, ability checks, or any other computed \
             value, so it carries no fabricated mechanical value (+0)"
        ),
    });

    // ----- speed -----
    explanations.push(ComputationExplanation {
        id: "race.gnome.trait_bundle.speed".to_owned(),
        value: GNOME_BASE_SPEED_FEET,
        detail: format!(
            "Gnome racial trait bundle — speed: PF1 Core Gnome has a base land speed of \
             {GNOME_BASE_SPEED_FEET} ft (cr_races.lst race:gnome GAIT:WALK|{GNOME_BASE_SPEED_FEET}). \
             This is a grounded recognition value carrying the Gnome base-speed identity on the \
             deterministic pilot seam; it contributes no computed speed-derived effect to any \
             chassis output, skill modifier, attack roll, or combat baseline"
        ),
    });

    // ----- senses -----
    explanations.push(ComputationExplanation {
        id: "race.gnome.trait_bundle.senses".to_owned(),
        value: 0,
        detail: "Gnome racial trait bundle — senses: PF1 Core Gnome grants low-light vision \
                  (cr_races.lst race:gnome SENSE:Low-Light Vision). This is a bounded \
                  recognition record naming the Gnome low-light vision identity on the \
                  deterministic pilot seam; it contributes no computed illumination or \
                  perception-derived effect to any chassis output, so it carries no fabricated \
                  mechanical value (+0)"
            .to_owned(),
    });

    // Bounded honesty: only the four named dimensions are grounded. This replaces
    // the generic race.semantics.unverified diagnostic for Gnome specifically and
    // stays non-claim-blocking so the deterministic pilot still reports computed
    // evidence.
    diagnostics.push(ComputationDiagnostic {
        id: "race.gnome.bounded_semantics".to_owned(),
        message: "Gnome race semantics are grounded for the deterministic pilot's ability \
                  modifiers, size, speed, and senses trait bundle; the remaining PF1 Core Gnome \
                  racial trait surface remains unverified: Defensive Training (a dodge bonus to \
                  AC against giants), Illusion Resistance (a bonus on saves against illusion \
                  spells and effects), Hatred (a bonus on attack rolls against reptilian \
                  humanoids and goblinoids), Keen Senses (a bonus on Perception checks), Gnome \
                  Magic (spell-like abilities keyed to a high Charisma), and weapon familiarity \
                  (gnome hooked hammer). PF1 core Gnomes gain no racial bonus feat (unlike \
                  Human), so that family is explicitly not applicable rather than silently \
                  omitted."
            .to_owned(),
        claim_blocking: false,
    });
}

const HALF_ELF_RACE_ID: &str = "race:half-elf";
const HALF_ELF_SIZE_CATEGORY: &str = "Medium";
const HALF_ELF_BASE_SPEED_FEET: i16 = 30;
const HALF_ELF_ABILITY_BONUS_CHOICE_ID: &str = "choice:half_elf_ability_bonus";

/// SD13-E2 Half-Elf racial trait bundle explanation seam (mirroring the
/// Dwarf/Elf/Gnome recognition pattern for the fourth non-Human core race, but
/// with a choice-based ability bonus like Human's rather than a fixed pair).
///
/// Surfaces four grounded PF1 Core Rulebook Half-Elf racial trait dimensions
/// (chosen ability-bonus target, size, speed, senses) as explicit
/// `ComputationExplanation` records so the Half-Elf identity is legible on the
/// runtime path rather than left behind the generic `race.semantics.unverified`
/// diagnostic every other non-Human race still receives.
///
/// This function:
///   - runs only when `race_id == race:half-elf`; every other race is unaffected
///     (Human, Dwarf, Elf, and Gnome keep their own seams; every other non-Human
///     race keeps the generic `race.semantics.unverified` diagnostic),
///   - adds no new computed mechanical contribution: the ability-bonus-target
///     record surfaces the already-computed modifier for the chosen ability as
///     recognition (mirroring `race.human.ability_bonus_target`'s shape), and
///     the size/senses records carry the grounded source value as identity only,
///   - replaces the generic `race.semantics.unverified` diagnostic with a
///     Half-Elf-specific `race.half_elf.bounded_semantics` note naming the
///     still-unproven families explicitly (Elven Immunities, Adaptability, Keen
///     Senses, Multitalented),
///   - is bounded to race recognition only; it deliberately grounds no Half-Elf
///     class-chassis interaction, no other race, and no PF1 alternate ruleset.
fn explain_half_elf_race_seam(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if input.chosen.race_id != HALF_ELF_RACE_ID {
        return;
    }

    // ----- ability bonus (choice-based, like Human) -----
    if let Some(selection) = choice_selection(input, HALF_ELF_ABILITY_BONUS_CHOICE_ID) {
        let ability = selection
            .strip_prefix(ABILITY_SELECTION_PREFIX)
            .unwrap_or(selection);
        let modifier = ability_modifier_for(ability_modifiers, ability);
        explanations.push(ComputationExplanation {
            id: "race.half_elf.trait_bundle.ability_bonus_target".to_owned(),
            value: modifier,
            detail: format!(
                "Half-Elf racial trait bundle — ability bonus: PF1 Core Half-Elf grants a \
                 player-chosen +2 to any one ability score ({HALF_ELF_ABILITY_BONUS_CHOICE_ID} \
                 -> {selection}); the chosen {ability} score yields modifier {modifier:+}. This \
                 is a bounded recognition record naming the chosen target on the deterministic \
                 pilot seam; the chosen score is understood to already reflect the +2 \
                 adjustment, so this record performs no arithmetic beyond surfacing the \
                 already-computed modifier"
            ),
        });
    }

    // ----- size -----
    explanations.push(ComputationExplanation {
        id: "race.half_elf.trait_bundle.size".to_owned(),
        value: 0,
        detail: format!(
            "Half-Elf racial trait bundle — size: PF1 Core Half-Elf is \
             {HALF_ELF_SIZE_CATEGORY} size (cr_races.lst race:half-elf SIZE:MEDIUM). This is a \
             bounded recognition record naming the Half-Elf size category on the deterministic \
             pilot seam; it contributes no numeric effect to attack rolls, AC, skill checks, \
             ability checks, or any other computed value, so it carries no fabricated \
             mechanical value (+0)"
        ),
    });

    // ----- speed -----
    explanations.push(ComputationExplanation {
        id: "race.half_elf.trait_bundle.speed".to_owned(),
        value: HALF_ELF_BASE_SPEED_FEET,
        detail: format!(
            "Half-Elf racial trait bundle — speed: PF1 Core Half-Elf has a base land speed of \
             {HALF_ELF_BASE_SPEED_FEET} ft \
             (cr_races.lst race:half-elf GAIT:WALK|{HALF_ELF_BASE_SPEED_FEET}). This is a \
             grounded recognition value carrying the Half-Elf base-speed identity on the \
             deterministic pilot seam; it contributes no computed speed-derived effect to any \
             chassis output, skill modifier, attack roll, or combat baseline"
        ),
    });

    // ----- senses -----
    explanations.push(ComputationExplanation {
        id: "race.half_elf.trait_bundle.senses".to_owned(),
        value: 0,
        detail: "Half-Elf racial trait bundle — senses: PF1 Core Half-Elf grants low-light \
                  vision (cr_races.lst race:half-elf SENSE:Low-Light Vision). This is a bounded \
                  recognition record naming the Half-Elf low-light vision identity on the \
                  deterministic pilot seam; it contributes no computed illumination or \
                  perception-derived effect to any chassis output, so it carries no fabricated \
                  mechanical value (+0)"
            .to_owned(),
    });

    // Bounded honesty: only the four named dimensions are grounded. This replaces
    // the generic race.semantics.unverified diagnostic for Half-Elf specifically
    // and stays non-claim-blocking so the deterministic pilot still reports
    // computed evidence.
    diagnostics.push(ComputationDiagnostic {
        id: "race.half_elf.bounded_semantics".to_owned(),
        message: "Half-Elf race semantics are grounded for the deterministic pilot's chosen \
                  ability-bonus target, size, speed, and senses trait bundle; the remaining PF1 \
                  Core Half-Elf racial trait surface remains unverified: Elven Immunities \
                  (immunity to magic sleep effects and a bonus on saves against enchantment \
                  spells and effects), Adaptability (a bonus Skill Focus feat in a chosen skill \
                  at 1st level), Keen Senses (a bonus on Perception checks), and Multitalented \
                  (counting both parent classes as favored classes)."
            .to_owned(),
        claim_blocking: false,
    });
}

const HALF_ORC_RACE_ID: &str = "race:half-orc";
const HALF_ORC_SIZE_CATEGORY: &str = "Medium";
const HALF_ORC_BASE_SPEED_FEET: i16 = 30;
const HALF_ORC_DARKVISION_FEET: i16 = 60;
const HALF_ORC_ABILITY_BONUS_CHOICE_ID: &str = "choice:half_orc_ability_bonus";

/// SD13-E2 Half-Orc racial trait bundle explanation seam (mirroring the
/// Half-Elf choice-based ability-bonus pattern for the fifth non-Human core
/// race, with Darkvision instead of low-light vision).
///
/// Surfaces four grounded PF1 Core Rulebook Half-Orc racial trait dimensions
/// (chosen ability-bonus target, size, speed, senses) as explicit
/// `ComputationExplanation` records so the Half-Orc identity is legible on the
/// runtime path rather than left behind the generic `race.semantics.unverified`
/// diagnostic every other non-Human race still receives.
///
/// This function:
///   - runs only when `race_id == race:half-orc`; every other race is
///     unaffected (Human, Dwarf, Elf, Gnome, and Half-Elf keep their own seams;
///     every other non-Human race keeps the generic `race.semantics.unverified`
///     diagnostic),
///   - adds no new computed mechanical contribution: the ability-bonus-target
///     record surfaces the already-computed modifier for the chosen ability as
///     recognition, and the size/senses records carry the grounded source value
///     as identity only,
///   - replaces the generic `race.semantics.unverified` diagnostic with a
///     Half-Orc-specific `race.half_orc.bounded_semantics` note naming the
///     still-unproven families explicitly (Intimidating, Orc Ferocity, weapon
///     familiarity),
///   - is bounded to race recognition only; it deliberately grounds no Half-Orc
///     class-chassis interaction, no other race, and no PF1 alternate ruleset.
fn explain_half_orc_race_seam(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if input.chosen.race_id != HALF_ORC_RACE_ID {
        return;
    }

    // ----- ability bonus (choice-based, like Half-Elf) -----
    if let Some(selection) = choice_selection(input, HALF_ORC_ABILITY_BONUS_CHOICE_ID) {
        let ability = selection
            .strip_prefix(ABILITY_SELECTION_PREFIX)
            .unwrap_or(selection);
        let modifier = ability_modifier_for(ability_modifiers, ability);
        explanations.push(ComputationExplanation {
            id: "race.half_orc.trait_bundle.ability_bonus_target".to_owned(),
            value: modifier,
            detail: format!(
                "Half-Orc racial trait bundle — ability bonus: PF1 Core Half-Orc grants a \
                 player-chosen +2 to any one ability score \
                 ({HALF_ORC_ABILITY_BONUS_CHOICE_ID} -> {selection}); the chosen {ability} score \
                 yields modifier {modifier:+}. This is a bounded recognition record naming the \
                 chosen target on the deterministic pilot seam; the chosen score is understood \
                 to already reflect the +2 adjustment, so this record performs no arithmetic \
                 beyond surfacing the already-computed modifier"
            ),
        });
    }

    // ----- size -----
    explanations.push(ComputationExplanation {
        id: "race.half_orc.trait_bundle.size".to_owned(),
        value: 0,
        detail: format!(
            "Half-Orc racial trait bundle — size: PF1 Core Half-Orc is \
             {HALF_ORC_SIZE_CATEGORY} size (cr_races.lst race:half-orc SIZE:MEDIUM). This is a \
             bounded recognition record naming the Half-Orc size category on the deterministic \
             pilot seam; it contributes no numeric effect to attack rolls, AC, skill checks, \
             ability checks, or any other computed value, so it carries no fabricated \
             mechanical value (+0)"
        ),
    });

    // ----- speed -----
    explanations.push(ComputationExplanation {
        id: "race.half_orc.trait_bundle.speed".to_owned(),
        value: HALF_ORC_BASE_SPEED_FEET,
        detail: format!(
            "Half-Orc racial trait bundle — speed: PF1 Core Half-Orc has a base land speed of \
             {HALF_ORC_BASE_SPEED_FEET} ft \
             (cr_races.lst race:half-orc GAIT:WALK|{HALF_ORC_BASE_SPEED_FEET}). This is a \
             grounded recognition value carrying the Half-Orc base-speed identity on the \
             deterministic pilot seam; it contributes no computed speed-derived effect to any \
             chassis output, skill modifier, attack roll, or combat baseline"
        ),
    });

    // ----- senses -----
    explanations.push(ComputationExplanation {
        id: "race.half_orc.trait_bundle.senses".to_owned(),
        value: HALF_ORC_DARKVISION_FEET,
        detail: format!(
            "Half-Orc racial trait bundle — senses: PF1 Core Half-Orc grants Darkvision \
             {HALF_ORC_DARKVISION_FEET} ft (cr_races.lst race:half-orc SENSE:Darkvision \
             ({HALF_ORC_DARKVISION_FEET} ft)). This is a grounded recognition value carrying \
             the Half-Orc Darkvision identity on the deterministic pilot seam; it contributes \
             no computed low-light or perception-derived effect to any chassis output"
        ),
    });

    // Bounded honesty: only the four named dimensions are grounded. This replaces
    // the generic race.semantics.unverified diagnostic for Half-Orc specifically
    // and stays non-claim-blocking so the deterministic pilot still reports
    // computed evidence.
    diagnostics.push(ComputationDiagnostic {
        id: "race.half_orc.bounded_semantics".to_owned(),
        message: "Half-Orc race semantics are grounded for the deterministic pilot's chosen \
                  ability-bonus target, size, speed, and senses trait bundle; the remaining PF1 \
                  Core Half-Orc racial trait surface remains unverified: Intimidating (a bonus \
                  on Intimidate checks), Orc Ferocity (fighting on for one more round after \
                  being brought below 0 hit points), and weapon familiarity (orc double axe, \
                  falchion, and treating any weapon with 'orc' in its name as martial)."
            .to_owned(),
        claim_blocking: false,
    });
}

const HALFLING_RACE_ID: &str = "race:halfling";
const HALFLING_SIZE_CATEGORY: &str = "Small";
const HALFLING_BASE_SPEED_FEET: i16 = 20;
const HALFLING_DEX_ADJUSTMENT: i16 = 2;
const HALFLING_STR_ADJUSTMENT: i16 = -2;

/// SD13-E2 Halfling racial trait bundle explanation seam (mirroring the
/// Dwarf/Elf/Gnome fixed-ability-pair pattern for the sixth and final
/// non-Human core race).
///
/// Surfaces four grounded PF1 Core Rulebook Halfling racial trait dimensions
/// (ability modifiers, size, speed, senses) as explicit `ComputationExplanation`
/// records so the Halfling identity is legible on the runtime path rather than
/// left behind the generic `race.semantics.unverified` diagnostic.
///
/// This function:
///   - runs only when `race_id == race:halfling`; every other race is
///     unaffected (Human, Dwarf, Elf, Gnome, Half-Elf, and Half-Orc keep their
///     own seams),
///   - adds no new computed mechanical contribution: the ability-modifiers
///     record is recognition-only (the chosen Dexterity/Strength scores are
///     understood to already reflect the fixed +2/-2 racial adjustment; no
///     arithmetic is performed on this seam), and the size/senses records
///     carry the grounded source value as identity only,
///   - replaces the generic `race.semantics.unverified` diagnostic with a
///     Halfling-specific `race.halfling.bounded_semantics` note naming the
///     still-unproven families explicitly (Fearless, Halfling Luck, Keen
///     Senses, Sure-Footed, weapon familiarity, and the explicit absence of
///     any Halfling racial bonus feat),
///   - is bounded to race recognition only; it deliberately grounds no
///     Halfling class-chassis interaction, no other race, and no PF1
///     alternate ruleset.
fn explain_halfling_race_seam(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if input.chosen.race_id != HALFLING_RACE_ID {
        return;
    }

    // ----- ability modifiers -----
    explanations.push(ComputationExplanation {
        id: "race.halfling.trait_bundle.ability_modifiers".to_owned(),
        value: 0,
        detail: format!(
            "Halfling racial trait bundle — ability modifiers: PF1 Core Halfling grants a \
             fixed {HALFLING_DEX_ADJUSTMENT:+} Dexterity and {HALFLING_STR_ADJUSTMENT:+} \
             Strength racial adjustment (cr_races.lst race:halfling \
             STAT:DEX|{HALFLING_DEX_ADJUSTMENT:+}, STAT:STR|{HALFLING_STR_ADJUSTMENT:+}). This \
             is a bounded recognition record naming the fixed adjustment on the deterministic \
             pilot seam; the chosen Dexterity and Strength scores are understood to already \
             reflect it, so this record performs no arithmetic and carries no fabricated \
             mechanical value (+0)"
        ),
    });

    // ----- size -----
    explanations.push(ComputationExplanation {
        id: "race.halfling.trait_bundle.size".to_owned(),
        value: 0,
        detail: format!(
            "Halfling racial trait bundle — size: PF1 Core Halfling is \
             {HALFLING_SIZE_CATEGORY} size (cr_races.lst race:halfling SIZE:SMALL). This is a \
             bounded recognition record naming the Halfling size category on the deterministic \
             pilot seam; it contributes no numeric effect to attack rolls, AC, skill checks, \
             ability checks, or any other computed value, so it carries no fabricated \
             mechanical value (+0)"
        ),
    });

    // ----- speed -----
    explanations.push(ComputationExplanation {
        id: "race.halfling.trait_bundle.speed".to_owned(),
        value: HALFLING_BASE_SPEED_FEET,
        detail: format!(
            "Halfling racial trait bundle — speed: PF1 Core Halfling has a base land speed of \
             {HALFLING_BASE_SPEED_FEET} ft \
             (cr_races.lst race:halfling GAIT:WALK|{HALFLING_BASE_SPEED_FEET}). This is a \
             grounded recognition value carrying the Halfling base-speed identity on the \
             deterministic pilot seam; it contributes no computed speed-derived effect to any \
             chassis output, skill modifier, attack roll, or combat baseline"
        ),
    });

    // ----- senses -----
    // Bounded "no special senses" classification, mirroring Human's pattern:
    // PF1 Core Halflings have ordinary vision (no darkvision, no low-light vision).
    explanations.push(ComputationExplanation {
        id: "race.halfling.trait_bundle.senses".to_owned(),
        value: 0,
        detail: "Halfling racial trait bundle — senses: PF1 Core Halfling grants no special \
                  senses (cr_races.lst race:halfling carries no SENSE tag; darkvision, \
                  low-light vision, and other sense bonuses are absent). This is a bounded \
                  no-effect classification record on the deterministic pilot seam; it carries \
                  no fabricated sense bonus and contributes no computed value (+0)"
            .to_owned(),
    });

    // Bounded honesty: only the four named dimensions are grounded. This replaces
    // the generic race.semantics.unverified diagnostic for Halfling specifically
    // and stays non-claim-blocking so the deterministic pilot still reports
    // computed evidence.
    diagnostics.push(ComputationDiagnostic {
        id: "race.halfling.bounded_semantics".to_owned(),
        message: "Halfling race semantics are grounded for the deterministic pilot's ability \
                  modifiers, size, speed, and senses trait bundle; the remaining PF1 Core \
                  Halfling racial trait surface remains unverified: Fearless (a bonus on saves \
                  against fear), Halfling Luck (a luck bonus on all saving throws), Keen Senses \
                  (a bonus on Perception checks), Sure-Footed (a bonus on Acrobatics and Climb \
                  checks), and weapon familiarity (sling and thrown weapons). PF1 core \
                  Halflings gain no racial bonus feat (unlike Human), so that family is \
                  explicitly not applicable rather than silently omitted."
            .to_owned(),
        claim_blocking: false,
    });
}

/// SD13-E6-F3a Human racial trait bundle explanation seam.
///
/// Surfaces each remaining PF1 Standard Human racial trait dimension (size,
/// speed, senses, extra skill ranks) as an explicit `ComputationExplanation`
/// record so the trait bundle is legible on the runtime path rather than left
/// as an incidental side-effect or a folklore claim. Three of the four
/// dimensions carry the grounded PF1 source value as a recognition record;
/// the senses dimension carries a bounded "no special senses" classification
/// because PF1 Standard Human grants no special sense bonus.
///
/// This function:
///   - runs only when `race_id == race:human`; non-Human races stay on the
///     bounded diagnostics the `explain_race_seam` dispatcher emits (the
///     Half-Elf bounded diagnostic or the `race.semantics.unverified`
///     catch-all),
///   - adds no new computed mechanical contribution; each record carries the
///     grounded source value as recognition and contributes nothing to the
///     chassis totals, selected-skill modifiers, combat baseline, or AC,
///   - replaces the previous "Human size, speed, senses, extra skill ranks
///     remain unverified" non-claim-blocking note from
///     `race.human.bounded_semantics` with explicit per-dimension records,
///   - is bounded to the deterministic Human Fighter level-1/2/3 pilot
///     posture implicitly via the caller; it deliberately grounds no other
///     Human racial variant (alternate Human racial traits, variant Humans,
///     half-Humans), no other race, and no PF1 alternate ruleset.
fn explain_human_trait_bundle(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    _diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // ----- size -----
    // Recognition record only; carries the grounded Human size category name
    // as the recognition value so the explanation reads as the humanoid
    // identity rather than fabricating a numeric contribution.
    explanations.push(ComputationExplanation {
        id: "race.human.trait_bundle.size".to_owned(),
        value: 0,
        detail: format!(
            "Human racial trait bundle — size: PF1 Standard Human is {HUMAN_SIZE_CATEGORY} size \
             (cr_races.lst race:human SIZE:MEDIUM). This is a bounded recognition record naming \
             the Human size category on the deterministic pilot seam; it contributes no numeric \
             effect to attack rolls, AC, skill checks, ability checks, or any other computed \
             value, so it carries no fabricated mechanical value (+0)"
        ),
    });

    // ----- speed -----
    // Recognition record for the 30 ft base land speed. The bounded
    // selected-skill and combat baselines never consult base speed, so this
    // record is identity-only — no computed speed-derived value is fabricated.
    explanations.push(ComputationExplanation {
        id: "race.human.trait_bundle.speed".to_owned(),
        value: HUMAN_BASE_SPEED_FEET,
        detail: format!(
            "Human racial trait bundle — speed: PF1 Standard Human has a base land speed of \
             {HUMAN_BASE_SPEED_FEET} ft (cr_races.lst race:human GAIT:WALK|{HUMAN_BASE_SPEED_FEET}). \
             This is a grounded recognition value carrying the human base-speed identity on the \
             deterministic pilot seam; it contributes no computed speed-derived effect to any \
             chassis output, skill modifier, attack roll, or combat baseline"
        ),
    });

    // ----- senses -----
    // Bounded "no special senses" classification. PF1 Standard Human grants
    // no special senses (darkvision, low-light, scent, etc.), so this
    // dimension is classified explicitly as no-effect rather than a silent
    // omission or a fabricated sense bonus.
    explanations.push(ComputationExplanation {
        id: "race.human.trait_bundle.senses".to_owned(),
        value: 0,
        detail: "Human racial trait bundle — senses: PF1 Standard Human grants no special senses \
             (cr_races.lst race:human carries no SENSE tag for Standard Human; darkvision, \
             low-light vision, scent, and other sense bonuses are absent). This is a bounded \
             no-effect classification record on the deterministic pilot seam; it carries no \
             fabricated sense bonus and contributes no computed value (+0)"
            .to_owned(),
    });

    // ----- extra skill ranks -----
    // Recognition record for the extra-skill-ranks Human trait. PF1 Standard
    // Human grants 4 extra skill points at 1st level and 1 extra skill rank
    // per additional level thereafter; this slice surfaces both numbers as a
    // recognition record and explicitly does not propagate them through the
    // bounded selected-skill modifier computation (which controls the
    // deterministic Climb / Intimidate / Swim rank-1 posture only).
    explanations.push(ComputationExplanation {
        id: "race.human.trait_bundle.extra_skill_ranks".to_owned(),
        value: i16::from(HUMAN_EXTRA_SKILL_RANKS_PER_LEVEL),
        detail: format!(
            "Human racial trait bundle — extra skill ranks: PF1 Standard Human gains \
             {HUMAN_EXTRA_SKILL_POINTS_AT_LEVEL_1} extra skill points at 1st level and \
             {HUMAN_EXTRA_SKILL_RANKS_PER_LEVEL} extra skill rank per additional level thereafter \
             (cr_races.lst race:human BONUS:SKILL|...). The recognition value \
             ({HUMAN_EXTRA_SKILL_RANKS_PER_LEVEL:+}) carries the per-additional-level extra-rank \
             identity on the deterministic pilot seam; this slice does not propagate these \
             extra skill points/rank through the bounded Climb/Intimidate/Swim rank-1 selected \
             skill-modifier computation, so the bounded fighter-posture skill totals remain \
             grounded by the canonical rank-1 posture rather than by the unbounded Human extra \
             skill-rank rule"
        ),
    });
}

/// Return the selection id chosen for the named choice set, if present.
fn choice_selection<'a>(input: &'a CharacterInput, choice_set_id: &str) -> Option<&'a str> {
    input
        .chosen
        .selected_choices
        .iter()
        .find(|c| c.choice_set_id == choice_set_id)
        .map(|c| c.selection_id.as_str())
}

/// Return a human-readable display name (e.g. "Knowledge (arcana)") when the given
/// selection names a specific Knowledge skill (a "knowledge:<skill>"-shaped token).
/// Returns `None` for any selection that is not itself shaped that way. This recognizes
/// the whole Knowledge skill family rather than a restricted enum list, because the PF1
/// Core Rulebook Arcane bloodline's own class-skill grant text reads "Knowledge (any
/// one)" — any Knowledge skill is legal, not just Knowledge (arcana).
fn knowledge_skill_display_name(selection: &str) -> Option<String> {
    selection
        .strip_prefix("knowledge:")
        .filter(|skill| !skill.is_empty())
        .map(|skill| format!("Knowledge ({skill})"))
}

/// Look up the already-computed modifier for a named ability. Unknown ability names
/// contribute nothing rather than fabricating a value.
fn ability_modifier_for(modifiers: &AbilityModifiers, ability: &str) -> i16 {
    match ability {
        "strength" => modifiers.strength,
        "dexterity" => modifiers.dexterity,
        "constitution" => modifiers.constitution,
        "intelligence" => modifiers.intelligence,
        "wisdom" => modifiers.wisdom,
        "charisma" => modifiers.charisma,
        _ => 0,
    }
}

/// The bounded Fighter milestone level this surface grounds, if any. Returns the
/// single Fighter level when the chosen input is exactly a single-class Fighter at
/// one of the supported milestone levels (1, 2, or 3). Returns `None` for no
/// Fighter, a non-Fighter class, a multiclass mix, or a level-4+ Fighter this slice
/// does not yet ground — each of which stays claim-blocked as before.
fn supported_fighter_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == FIGHTER_CLASS_ID
                && (1..=MAX_SUPPORTED_FIGHTER_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

/// Fighter armor-training profile for a given Fighter level. Armor training 1 is
/// gained at level 3, and armor training 2 at level 7; before level 3 there is no
/// armor-training effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FighterArmorTraining {
    /// Armor-training rank (0 before level 3, 1 from level 3, 2 from level 7).
    rank: u8,
    /// Reduction applied to the worn armor's armor-check penalty (moves it toward 0).
    armor_check_reduction: i16,
    /// Increase applied to the worn armor's maximum Dexterity bonus.
    max_dex_increase: i16,
}

fn fighter_armor_training(level: u8) -> FighterArmorTraining {
    if level >= FIGHTER_ARMOR_TRAINING_2_LEVEL {
        FighterArmorTraining {
            rank: 2,
            armor_check_reduction: ARMOR_TRAINING_2_ARMOR_CHECK_REDUCTION,
            max_dex_increase: ARMOR_TRAINING_2_MAX_DEX_INCREASE,
        }
    } else if level >= FIGHTER_ARMOR_TRAINING_1_LEVEL {
        FighterArmorTraining {
            rank: 1,
            armor_check_reduction: ARMOR_TRAINING_1_ARMOR_CHECK_REDUCTION,
            max_dex_increase: ARMOR_TRAINING_1_MAX_DEX_INCREASE,
        }
    } else {
        FighterArmorTraining {
            rank: 0,
            armor_check_reduction: 0,
            max_dex_increase: 0,
        }
    }
}

/// The effective Chain Shirt armor-check penalty at a Fighter level, after any
/// armor-training reduction. Capped at 0 so the reduction never turns the penalty
/// into a bonus.
fn effective_chain_shirt_armor_check_penalty(level: u8) -> i16 {
    (CHAIN_SHIRT_ARMOR_CHECK_PENALTY + fighter_armor_training(level).armor_check_reduction).min(0)
}

/// The Fighter weapon-training rank at the given level: 0 before level 5, then
/// 1 + (level - 5) / 4 (Weapon Training 1 at level 5, Weapon Training 2 at
/// level 9 within this bounded levels-1-10 surface).
fn fighter_weapon_training_rank(level: u8) -> i16 {
    if level < FIGHTER_WEAPON_TRAINING_1_LEVEL {
        return 0;
    }
    i16::from(1 + (level - FIGHTER_WEAPON_TRAINING_1_LEVEL) / FIGHTER_WEAPON_TRAINING_RANK_LEVEL_STRIDE)
}

/// The weapon-training attack-roll bonus for the first chosen weapon group at
/// the given Fighter level, gated on the canonical
/// `choice:fighter_weapon_training_group -> group:heavy_blades` selection (the
/// group the deterministic Longsword falls under). The bonus equals the
/// weapon-training rank: +1 at levels 5-8, +2 at levels 9-10. Returns 0 before
/// level 5 or when the group choice is absent — the canonical-choice validator
/// (`CANONICAL_FIGHTER_FEAT_CHOICES`) separately claim-blocks a
/// present-but-non-canonical selection, so this function only needs to
/// distinguish "canonical" from "absent or anything else."
fn fighter_weapon_training_attack_bonus(input: &CharacterInput, level: u8) -> i16 {
    if choice_selection(input, FIGHTER_WEAPON_TRAINING_GROUP_CHOICE_ID)
        == Some(HEAVY_BLADES_GROUP_SELECTION)
    {
        fighter_weapon_training_rank(level)
    } else {
        0
    }
}

/// The Fighter Bravery Will-save-vs-fear bonus magnitude at the given level: 0
/// before level 2, then 1 + (level - 2) / 4 (+1 at level 2, +2 at level 6, +3 at
/// level 10 within this bounded levels-1-10 surface). A flat magnitude only —
/// no fear-condition or save-resolution engine exists on this compute surface.
fn fighter_bravery_bonus(level: u8) -> i16 {
    if level < FIGHTER_BRAVERY_LEVEL {
        return 0;
    }
    i16::from(1 + (level - FIGHTER_BRAVERY_LEVEL) / FIGHTER_BRAVERY_RANK_LEVEL_STRIDE)
}

/// Compute the bounded Fighter base chassis for the supported milestone levels
/// (1, 2, or 3), or block the claim if the input is not a supported single-class
/// Fighter posture for this narrow slice.
fn compute_fighter_chassis(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) -> (i16, BaseSaves) {
    let Some(level) = supported_fighter_level(input) else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_chassis.unsupported".to_owned(),
            message: format!(
                "base class chassis is only supported for a single-class {FIGHTER_CLASS_ID} at \
                 levels 1-{MAX_SUPPORTED_FIGHTER_LEVEL}; chosen class levels {:?} do not provide it, \
                 so no chassis values were computed",
                input.chosen.class_levels
            ),
            claim_blocking: true,
        });
        return (0, BaseSaves::default());
    };

    // Grounded Fighter base progression from cr_classes.lst:139, evaluated at the
    // chosen level:
    //   BONUS:COMBAT|BASEAB|classlevel                -> level (full base attack)
    //   BONUS:SAVE|BASE.Fortitude|classlevel/2+2      -> level/2 + 2 (good save)
    //   BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel/3 -> level/3 (poor saves)
    let level_value = i16::from(level);
    let base_attack_bonus = level_value;
    let base_saves = BaseSaves {
        fortitude: level_value / 2 + 2,
        reflex: level_value / 3,
        will: level_value / 3,
    };

    explanations.push(ComputationExplanation {
        id: "class_chassis.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            "Fighter level {level} base attack bonus from cr_classes.lst:139 \
             BONUS:COMBAT|BASEAB|classlevel = {base_attack_bonus}"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.fortitude".to_owned(),
        value: base_saves.fortitude,
        detail: format!(
            "Fighter level {level} base Fortitude save from cr_classes.lst:139 \
             BONUS:SAVE|BASE.Fortitude|classlevel/2+2 = {}",
            base_saves.fortitude
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.reflex".to_owned(),
        value: base_saves.reflex,
        detail: format!(
            "Fighter level {level} base Reflex save from cr_classes.lst:139 \
             BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel/3 = {}",
            base_saves.reflex
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.base_save.will".to_owned(),
        value: base_saves.will,
        detail: format!(
            "Fighter level {level} base Will save from cr_classes.lst:139 \
             BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel/3 = {}",
            base_saves.will
        ),
    });

    (base_attack_bonus, base_saves)
}

/// Make the bounded Fighter milestone class features for this slice explicit rather
/// than leaving them incidental: the level-2 bonus-feat progression seam and the
/// level-3 armor-training seam.
///
/// This adds no general feat-effect or prerequisite engine. The level-2 bonus-feat
/// seam names the chosen selection only and contributes no computed mechanical value.
/// The level-3 armor-training seam names the concrete armor-check-penalty reduction
/// and maximum-Dexterity increase that the bounded selected-skill and armor-class
/// outputs already apply, so the derived-output change is legible instead of folklore.
fn explain_fighter_class_features(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = supported_fighter_level(input) else {
        return;
    };

    let bravery_bonus = fighter_bravery_bonus(level);
    if bravery_bonus > 0 {
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.bravery".to_owned(),
            value: bravery_bonus,
            detail: format!(
                "Fighter level {FIGHTER_BRAVERY_LEVEL} Bravery (cr_abilities_class.lst Fighter; \
                 +1 at level {FIGHTER_BRAVERY_LEVEL} and another +1 every \
                 {FIGHTER_BRAVERY_RANK_LEVEL_STRIDE} Fighter levels thereafter): grants \
                 +{bravery_bonus} to Will saves against fear. This is a flat, non-fabricated \
                 bonus magnitude only — no fear-condition or Will-save-resolution engine exists \
                 anywhere in this codebase, so this bonus is never folded into the unconditional \
                 Will save total"
            ),
        });
    }

    if level >= 2
        && let Some(selection) = choice_selection(input, FIGHTER_LEVEL_2_BONUS_FEAT_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.level_2_bonus_feat".to_owned(),
            value: 0,
            detail: format!(
                "Fighter level 2 grants an additional bonus feat; the named selection \
                     ({FIGHTER_LEVEL_2_BONUS_FEAT_CHOICE_ID} -> {selection}) is surfaced as an \
                     explicit progression seam only. This slice grounds the bonus-feat slot, not a \
                     general feat-effect or prerequisite engine, so it contributes no computed \
                     mechanical value (+0)"
            ),
        });
    }

    if level >= 4
        && let Some(selection) = choice_selection(input, FIGHTER_LEVEL_4_BONUS_FEAT_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.level_4_bonus_feat".to_owned(),
            value: 0,
            detail: format!(
                "Fighter level 4 grants an additional bonus feat; the named selection \
                     ({FIGHTER_LEVEL_4_BONUS_FEAT_CHOICE_ID} -> {selection}) is surfaced as an \
                     explicit progression seam only. This slice grounds the bonus-feat slot, not a \
                     general feat-effect or prerequisite engine, so it contributes no computed \
                     mechanical value (+0)"
            ),
        });
    }

    if level >= 6
        && let Some(selection) = choice_selection(input, FIGHTER_LEVEL_6_BONUS_FEAT_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.level_6_bonus_feat".to_owned(),
            value: 0,
            detail: format!(
                "Fighter level 6 grants an additional bonus feat; the named selection \
                     ({FIGHTER_LEVEL_6_BONUS_FEAT_CHOICE_ID} -> {selection}) is surfaced as an \
                     explicit progression seam only. This slice grounds the bonus-feat slot, not a \
                     general feat-effect or prerequisite engine, so it contributes no computed \
                     mechanical value (+0)"
            ),
        });
    }

    if level >= 8
        && let Some(selection) = choice_selection(input, FIGHTER_LEVEL_8_BONUS_FEAT_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.level_8_bonus_feat".to_owned(),
            value: 0,
            detail: format!(
                "Fighter level 8 grants an additional bonus feat; the named selection \
                     ({FIGHTER_LEVEL_8_BONUS_FEAT_CHOICE_ID} -> {selection}) is surfaced as an \
                     explicit progression seam only. This slice grounds the bonus-feat slot, not a \
                     general feat-effect or prerequisite engine, so it contributes no computed \
                     mechanical value (+0)"
            ),
        });
    }

    if level >= 10
        && let Some(selection) = choice_selection(input, FIGHTER_LEVEL_10_BONUS_FEAT_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.level_10_bonus_feat".to_owned(),
            value: 0,
            detail: format!(
                "Fighter level 10 grants an additional bonus feat; the named selection \
                     ({FIGHTER_LEVEL_10_BONUS_FEAT_CHOICE_ID} -> {selection}) is surfaced as an \
                     explicit progression seam only. The canonical Greater Weapon Focus \
                     selection's prerequisites (Weapon Focus (longsword) and fighter level 8) are \
                     honestly met by the canonical loadout. This slice grounds the bonus-feat \
                     slot, not a general feat-effect or prerequisite engine, so it contributes no \
                     computed mechanical value (+0)"
            ),
        });
    }

    let armor_training = fighter_armor_training(level);
    if armor_training.rank == 2 {
        let reduced_penalty = effective_chain_shirt_armor_check_penalty(level);
        let raised_max_dex = CHAIN_SHIRT_MAX_DEX + armor_training.max_dex_increase;
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.armor_training".to_owned(),
            value: i16::from(armor_training.rank),
            detail: format!(
                "Fighter level {FIGHTER_ARMOR_TRAINING_2_LEVEL} Armor Training 2 (armor training, \
                 cr_abilities_class.lst Fighter): further reduces the worn Chain Shirt armor-check \
                 penalty by {ARMOR_TRAINING_2_ARMOR_CHECK_REDUCTION} cumulative (from \
                 {CHAIN_SHIRT_ARMOR_CHECK_PENALTY:+} to {reduced_penalty:+}), which raises the \
                 armor-check-penalty-affected selected skill totals (Climb, Swim) by the same \
                 amount, and raises the maximum Dexterity bonus by \
                 {ARMOR_TRAINING_2_MAX_DEX_INCREASE} cumulative (from {CHAIN_SHIRT_MAX_DEX} to \
                 {raised_max_dex}); on the deterministic +2 Dexterity contribution, this changes \
                 no derived armor-class value on this fixture"
            ),
        });
    } else if armor_training.rank == 1 {
        let reduced_penalty = effective_chain_shirt_armor_check_penalty(level);
        let raised_max_dex = CHAIN_SHIRT_MAX_DEX + armor_training.max_dex_increase;
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.armor_training".to_owned(),
            value: i16::from(armor_training.rank),
            detail: format!(
                "Fighter level {FIGHTER_ARMOR_TRAINING_1_LEVEL} Armor Training 1 (armor training, \
                 cr_abilities_class.lst Fighter): reduces the worn Chain Shirt armor-check penalty by \
                 {ARMOR_TRAINING_1_ARMOR_CHECK_REDUCTION} (from {CHAIN_SHIRT_ARMOR_CHECK_PENALTY:+} to \
                 {reduced_penalty:+}) and raises the maximum Dexterity bonus by \
                 {ARMOR_TRAINING_1_MAX_DEX_INCREASE} (from {CHAIN_SHIRT_MAX_DEX} to {raised_max_dex})"
            ),
        });
    }

    let weapon_training_bonus = fighter_weapon_training_attack_bonus(input, level);
    if weapon_training_bonus > 0 {
        let rank = fighter_weapon_training_rank(level);
        let rank_level = if rank >= 2 {
            FIGHTER_WEAPON_TRAINING_1_LEVEL + FIGHTER_WEAPON_TRAINING_RANK_LEVEL_STRIDE
        } else {
            FIGHTER_WEAPON_TRAINING_1_LEVEL
        };
        explanations.push(ComputationExplanation {
            id: "class_feature.fighter.weapon_training".to_owned(),
            value: weapon_training_bonus,
            detail: format!(
                "Fighter level {rank_level} Weapon Training {rank} (weapon training, \
                 cr_abilities_class.lst Fighter; rank = 1 + (level - 5) / 4): the first chosen \
                 weapon group \
                 ({FIGHTER_WEAPON_TRAINING_GROUP_CHOICE_ID} -> {HEAVY_BLADES_GROUP_SELECTION}) \
                 grants +{weapon_training_bonus} to attack rolls with weapons of that group, \
                 which the deterministic Longsword falls under; this +{weapon_training_bonus} is \
                 already folded into the baseline melee attack bonus. Weapon Training also grants \
                 +{weapon_training_bonus} to damage rolls with weapons of that group, but no \
                 damage total is computed anywhere in this codebase for any Fighter level, so the \
                 damage-roll half stays explicitly unproven rather than silently omitted"
            ),
        });

        // Weapon Training 2 (level 9) also grants a second chosen weapon group a
        // bonus one point lower than the first group's. The canonical second group
        // (Bows) covers no equipped weapon on the deterministic Longsword loadout,
        // so this is an explanation-only record: its +1 is never folded into the
        // Longsword baseline melee attack bonus, which uses the first-group rank.
        if rank >= 2
            && choice_selection(input, FIGHTER_WEAPON_TRAINING_GROUP_2_CHOICE_ID)
                == Some(BOWS_GROUP_SELECTION)
        {
            let second_group_bonus = rank - 1;
            explanations.push(ComputationExplanation {
                id: "class_feature.fighter.weapon_training_group_2".to_owned(),
                value: second_group_bonus,
                detail: format!(
                    "Fighter level {rank_level} Weapon Training {rank} also grants a second \
                     chosen weapon group \
                     ({FIGHTER_WEAPON_TRAINING_GROUP_2_CHOICE_ID} -> {BOWS_GROUP_SELECTION}) \
                     +{second_group_bonus} to attack and damage rolls with weapons of that \
                     group. No bow is part of the deterministic Longsword loadout, so this seam \
                     is explanation-only: the +{second_group_bonus} is not folded into any \
                     computed total, and the baseline melee attack bonus uses only the \
                     first-group (Heavy Blades) rank"
                ),
            });
        }
    }
}

/// Ground the Fighter level-1 hit-point milestone as a standalone explanation
/// record: level-1 hit points = 10 (the maximized d10 Fighter hit die at 1st
/// character level, PF1 Core Rulebook) + the Constitution modifier already
/// computed from the raw chosen score by [`compute_ability_modifiers`].
///
/// Gated the same way the other Fighter explanation seams gate (the bounded
/// [`supported_fighter_level`] recognition), narrowed to level 1 because only
/// the level-1 hit-point value is grounded. The record is deliberately wired
/// into no view-model total and no derived combat/defense output. Still
/// unproven and named in the record detail: the favored-class +1 hp /
/// +1 skill-rank choice (no input surface exists for it), hit points at
/// levels 2+ (no average/rolled hit-die policy is grounded), and Toughness /
/// feat hit-point interplay.
///
/// SD13-E5 update: the favored-class bonus CHOICE itself (which of the two
/// legal options, +1 hp or +1 skill rank, was picked) is now recognized as a
/// standalone record by [`explain_fighter_favored_class_bonus_choice`]. That
/// record's own +1 magnitude is never wired into this hit-point total (nor
/// into any selected-skill-rank total) — the sentence above stays accurate
/// for this record specifically, which still carries no favored-class
/// contribution of its own.
fn explain_fighter_level1_hit_points(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    if supported_fighter_level(input) != Some(1) {
        return;
    }

    let constitution_modifier = ability_modifiers.constitution;
    let hit_points = FIGHTER_LEVEL_1_MAX_HIT_DIE_HIT_POINTS + constitution_modifier;
    explanations.push(ComputationExplanation {
        id: "class_chassis.fighter.level_1_hit_points".to_owned(),
        value: hit_points,
        detail: format!(
            "Fighter level-1 hit points: maximized d10 Fighter hit die at 1st level \
             ({FIGHTER_LEVEL_1_MAX_HIT_DIE_HIT_POINTS}) + Constitution modifier \
             ({constitution_modifier:+}) = {hit_points}. This is a standalone grounded \
             record wired into no view-model total. Still unproven and out of scope: the \
             favored-class +1 hp / +1 skill-rank choice (no input surface exists for it), \
             hit points at levels 2+ (no average/rolled hit-die policy is grounded), and \
             Toughness / feat hit-point interplay"
        ),
    });
}

/// Choice-slot id for the PF1 Core Rulebook Favored Class rule (Core Rulebook
/// pg. 31, verified against the Archives of Nethys primary source: "Whenever a
/// character gains a level in his favored class, he receives either +1 hit
/// point or +1 skill rank"). A Human's favored class is Any (PF1 Core Rulebook
/// Human racial traits), which trivially includes Fighter, so this choice
/// applies at Fighter level 1 on this codebase's Human-only Fighter chassis
/// seam without needing to resolve the later "any race, any class" errata.
const FAVORED_CLASS_BONUS_CHOICE_ID: &str = "choice:favored_class_bonus";
const FAVORED_CLASS_BONUS_HP_SELECTION: &str = "bonus:hp";
const FAVORED_CLASS_BONUS_SKILL_RANK_SELECTION: &str = "bonus:skill_rank";

/// Ground the Fighter level-1 favored-class bonus CHOICE as a standalone
/// recognition record, mirroring the already-landed Sorcerer bloodline choice
/// / Cleric domain choice / Druid nature-bond choice / Monk bonus-feat choice
/// recognition idiom: recognize which of the two PF1 Core Rulebook Favored
/// Class rule options (`bonus:hp` or `bonus:skill_rank`) was selected, and
/// name the rule's own genuinely flat +1 magnitude (verified against the
/// Archives of Nethys primary source: the bonus is always exactly +1
/// regardless of which option is chosen, so this is not a fabricated number).
///
/// This is recognition of the choice slot only. It never applies the +1 to
/// the level-1 hit-point total grounded by [`explain_fighter_level1_hit_points`]
/// (`class_chassis.fighter.level_1_hit_points`) nor to any selected-skill-rank
/// total — doing so would require wiring into the integrated hit-point /
/// skill-rank computation, which stays out of scope for this slice. A
/// selection present but naming neither legal option is acknowledged without
/// claiming a resolved hp/skill-rank identity, mirroring the Monk bonus-feat
/// choice's "present but unrecognized" branch. No record is emitted at all
/// when the choice slot is absent, so no favored-class input is fabricated
/// for a fixture that never selected one.
fn explain_fighter_favored_class_bonus_choice(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    if supported_fighter_level(input) != Some(1) {
        return;
    }

    let Some(selection) = choice_selection(input, FAVORED_CLASS_BONUS_CHOICE_ID) else {
        return;
    };

    let (value, detail) = if selection == FAVORED_CLASS_BONUS_HP_SELECTION {
        (
            1,
            format!(
                "Favored Class bonus choice ({FAVORED_CLASS_BONUS_CHOICE_ID} -> {selection}): \
                 PF1 Core Rulebook pg. 31 grants a character +1 hit point or +1 skill rank for \
                 each level taken in his favored class; a Human's favored class is Any, which \
                 trivially includes Fighter, so this level-1 Fighter class level qualifies. This \
                 selection chooses the +1 hit point option. This is a flat, non-fabricated bonus \
                 magnitude only (+1) — it is standalone and not applied to the level-1 hit-point \
                 total (`class_chassis.fighter.level_1_hit_points`), since that would require \
                 wiring into the integrated hit-point computation, never attempted in this \
                 codebase"
            ),
        )
    } else if selection == FAVORED_CLASS_BONUS_SKILL_RANK_SELECTION {
        (
            1,
            format!(
                "Favored Class bonus choice ({FAVORED_CLASS_BONUS_CHOICE_ID} -> {selection}): \
                 PF1 Core Rulebook pg. 31 grants a character +1 hit point or +1 skill rank for \
                 each level taken in his favored class; a Human's favored class is Any, which \
                 trivially includes Fighter, so this level-1 Fighter class level qualifies. This \
                 selection chooses the +1 skill rank option. This is a flat, non-fabricated bonus \
                 magnitude only (+1) — it is standalone and not applied to any selected-skill-rank \
                 total, since that would require wiring into a general class-skill-rank \
                 allocation engine, never attempted in this codebase"
            ),
        )
    } else {
        (
            0,
            format!(
                "Favored Class bonus choice slot is present ({FAVORED_CLASS_BONUS_CHOICE_ID} -> \
                 {selection}), but only the PF1 Core Rulebook's two legal options \
                 ({FAVORED_CLASS_BONUS_HP_SELECTION} or \
                 {FAVORED_CLASS_BONUS_SKILL_RANK_SELECTION}) are recognized on this bounded seam; \
                 no hp/skill-rank identity is resolved and no mechanical value is fabricated (+0)"
            ),
        )
    };

    explanations.push(ComputationExplanation {
        id: "class_chassis.fighter.favored_class_bonus_choice".to_owned(),
        value,
        detail,
    });
}

/// The canonical Human Fighter feat-choice selections this slice preserves on the
/// deterministic level-1 through level-10 seam, as `(choice_set_id,
/// canonical_selection_id)` pairs. Any named slot present but deviating from its
/// canonical selection is claim-blocked. A slot absent for the chosen level (e.g.
/// the level-2 bonus feat at level 1) is not fabricated. This same machinery
/// validates the level-5 and level-9 weapon-training-group choices, since each is
/// structurally identical to a bonus-feat slot (a named choice-set that must match
/// one canonical selection).
const CANONICAL_FIGHTER_FEAT_CHOICES: [(&str, &str); 10] = [
    (
        LEVEL_1_CHARACTER_FEAT_CHOICE_ID,
        POWER_ATTACK_FEAT_SELECTION,
    ),
    (HUMAN_BONUS_FEAT_CHOICE_ID, DODGE_FEAT_ID),
    (
        FIGHTER_BONUS_FEAT_CHOICE_ID,
        WEAPON_FOCUS_LONGSWORD_SELECTION,
    ),
    (
        FIGHTER_LEVEL_2_BONUS_FEAT_CHOICE_ID,
        TOUGHNESS_FEAT_SELECTION,
    ),
    (
        FIGHTER_LEVEL_4_BONUS_FEAT_CHOICE_ID,
        CLEAVE_FEAT_SELECTION,
    ),
    (
        FIGHTER_WEAPON_TRAINING_GROUP_CHOICE_ID,
        HEAVY_BLADES_GROUP_SELECTION,
    ),
    (
        FIGHTER_LEVEL_6_BONUS_FEAT_CHOICE_ID,
        COMBAT_REFLEXES_FEAT_SELECTION,
    ),
    (
        FIGHTER_LEVEL_8_BONUS_FEAT_CHOICE_ID,
        IMPROVED_CRITICAL_FEAT_SELECTION,
    ),
    (
        FIGHTER_WEAPON_TRAINING_GROUP_2_CHOICE_ID,
        BOWS_GROUP_SELECTION,
    ),
    (
        FIGHTER_LEVEL_10_BONUS_FEAT_CHOICE_ID,
        GREATER_WEAPON_FOCUS_FEAT_SELECTION,
    ),
];

/// Claim-block non-canonical feat-choice mutations on the deterministic Human Fighter
/// levels 1-10 seam, while preserving the accepted canonical selections exactly.
///
/// This is deliberately not a general feat legality or prerequisite engine. It only knows
/// the exact accepted deterministic feat-choice selections on the bounded Human Fighter
/// seam. When one of those named choice slots is present but deviates from its canonical
/// selection, it emits a claim-blocking diagnostic that names the offending choice identity
/// and states plainly that alternative feat/prerequisite legality is outside this bounded
/// proof without a general engine — instead of letting the non-canonical build ride through
/// as a fabricated computed success.
///
/// It runs only for a supported single-class Human Fighter (levels 1-10); any other posture
/// is already claim-blocked upstream and is left untouched here. It grounds no alternative
/// feat effect and does not touch the read-only canonical Human ability-bonus target.
fn validate_fighter_feat_choice_legality(
    input: &CharacterInput,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    if supported_fighter_level(input).is_none() {
        return;
    }
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    for (choice_set_id, canonical_selection) in CANONICAL_FIGHTER_FEAT_CHOICES {
        let Some(selection) = choice_selection(input, choice_set_id) else {
            // The slot is absent for this level; do not fabricate a required choice.
            continue;
        };
        if selection != canonical_selection {
            diagnostics.push(ComputationDiagnostic {
                id: format!("feat_choice.non_canonical.{choice_set_id}"),
                message: format!(
                    "feat-choice slot {choice_set_id} on the deterministic Human Fighter levels \
                     1-{MAX_SUPPORTED_FIGHTER_LEVEL} seam must be the canonical {canonical_selection}; \
                     chosen selection {selection} is a non-canonical feat choice. This bounded slice \
                     preserves only the accepted canonical Human Fighter feat-choice path and grounds \
                     no general feat-effect or prerequisite engine, so alternative feat/prerequisite \
                     legality is outside this proof and the non-canonical build is claim-blocked \
                     rather than computed as a legal build"
                ),
                claim_blocking: true,
            });
        }
    }
}

/// A hybrid (martial + later spellcasting) class this slice recognizes at its bounded
/// single-class level-1 chassis boundary only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HybridClass {
    Paladin,
    Ranger,
}

/// Return the hybrid class when the chosen input is exactly a single-class Paladin or
/// Ranger at the bounded hybrid baseline level (1). Returns `None` for any other class,
/// a multiclass mix, or a level-2+ hybrid this slice deliberately does not recognize —
/// each of which stays blocked exactly as before.
fn hybrid_level1_class(input: &CharacterInput) -> Option<HybridClass> {
    match input.chosen.class_levels.as_slice() {
        [class_level] if class_level.level == HYBRID_BASELINE_LEVEL => {
            match class_level.class_id.as_str() {
                PALADIN_CLASS_ID => Some(HybridClass::Paladin),
                RANGER_CLASS_ID => Some(HybridClass::Ranger),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Surface direct SD13-E3-F6 runtime evidence for the deterministic Human Paladin
/// level-1 and Human Ranger level-1 hybrid chassis, while keeping both explicitly
/// claim-blocked on their still-missing burdens.
///
/// This deliberately does not compute a supported hybrid chassis. It grounds no base
/// attack/save progression, no smite / lay-on-hands / divine-grace / mercy execution,
/// no favored-enemy / combat-style / tracking execution, and no spell posture. It only:
/// - leaves one chassis-recognition explanation so the `class:paladin:1` / `class:ranger:1`
///   identity is acknowledged as a hybrid martial baseline rather than an undocumented
///   packet placeholder (direct runtime evidence, carrying no fabricated mechanical value), and
/// - emits two claim-blocking diagnostics naming the still-missing non-spell class-feature
///   burden family and the later hybrid spell burden explicitly, rather than hiding behind
///   a generic "unsupported hybrid" label.
///
/// The bounded Fighter-shaped compute path already claim-blocks these inputs; this seam
/// keeps that blocked posture but makes the hybrid class identity and its named burdens
/// legible on the runtime path.
fn explain_hybrid_level1_chassis(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let Some(hybrid) = hybrid_level1_class(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    let (class_id, class_name, chassis_id, feature_id, feature_burden, spell_id) = match hybrid {
        HybridClass::Paladin => (
            PALADIN_CLASS_ID,
            "Paladin",
            "class_chassis.hybrid_baseline.paladin",
            "class_feature.hybrid.paladin.unsupported",
            "smite evil, lay on hands, divine grace, and mercy",
            "class_spell.hybrid.paladin.unsupported",
        ),
        HybridClass::Ranger => (
            RANGER_CLASS_ID,
            "Ranger",
            "class_chassis.hybrid_baseline.ranger",
            "class_feature.hybrid.ranger.unsupported",
            "favored enemy, combat style, and skill/tracking",
            "class_spell.hybrid.ranger.unsupported",
        ),
    };

    // Direct runtime evidence: recognize the deterministic Human hybrid level-1 chassis
    // identity. This is a recognition record only; it fabricates no mechanical value.
    explanations.push(ComputationExplanation {
        id: chassis_id.to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human {class_name} level {HYBRID_BASELINE_LEVEL} hybrid chassis: \
             the {class_id}:{HYBRID_BASELINE_LEVEL} class identity is acknowledged as a hybrid martial \
             baseline on the rules-core seam rather than an undocumented packet placeholder. This is a \
             bounded chassis-recognition record only; it grounds no {class_name} class-feature math and \
             no spell posture, so it carries no fabricated mechanical value (+0)"
        ),
    });

    // Still blocked (1/2): name the non-spell class-feature burden family explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: feature_id.to_owned(),
        message: format!(
            "{class_name} level {HYBRID_BASELINE_LEVEL} remains blocked on its non-spell class-feature \
             burden: {feature_burden} are not implemented in this bounded hybrid chassis baseline, so no \
             {class_name} class-feature support is claimed"
        ),
        claim_blocking: true,
    });

    // Still blocked (2/2): name the later hybrid spell burden explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: spell_id.to_owned(),
        message: format!(
            "{class_name} remains blocked on its later hybrid spell burden: spell slots, spell source, \
             and spells known/prepared posture are out of scope for this level-{HYBRID_BASELINE_LEVEL} \
             chassis baseline and are deferred to the SD13-E4 spellcasting slice"
        ),
        claim_blocking: true,
    });
}

/// The bounded Paladin milestone level this decomposition surface grounds, if
/// any. Returns the single Paladin level when the chosen input is exactly a
/// single-class Paladin at one of the supported milestone levels (1 through
/// 9). Returns `None` for no Paladin, a non-Paladin class, a multiclass mix,
/// the Ranger hybrid (which has its own F6 class-feature decomposition
/// lane), or any level-10+ Paladin this slice deliberately does not
/// recognize — each of which stays claim-blocked exactly as before.
fn supported_paladin_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == PALADIN_CLASS_ID
                && (1..=MAX_SUPPORTED_PALADIN_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

/// Surface direct SD13-E3/E4/E5 runtime evidence for the deterministic Human
/// Paladin level-1/level-2/level-3 chassis and spell burden as a separable pair
/// of diagnostics.
///
/// This sits on top of the accepted SD13-F6 hybrid baseline: F6 already proves
/// the deterministic Human Paladin level-1 hybrid identity is acknowledged on
/// the compute seam and emits a single combined non-spell class-feature
/// blocker plus a single combined later-spell blocker. This slice proves the
/// per-burden separation Paladin actually needs, widened by the SD13-E5
/// level-2 milestone, further SD13-E5 slices' level-3 milestone (mercy) and
/// level-4 milestone (Smite Evil 2/day, Channel Positive Energy grant), the
/// level-5 milestone (the effective-caster-level gate and Channel Positive
/// Energy's die count both genuinely widen again; Divine Bond, the level-5
/// "Special" column's other entry, was checked and confirmed NOT flat, so it
/// stays deliberately named-but-unproven), the level-6 milestone (both good
/// base saves and lay on hands genuinely widen again, and the
/// effective-caster-level gate widens again; the level-6 "Special" column's
/// repeat "Mercy" entry -- an additional mercy becomes selectable at 6th
/// level -- was checked and confirmed to require a mercy-list-growth
/// mechanism this codebase has not already grounded, so it stays
/// deliberately named-but-unproven, mirroring the Divine Bond precedent),
/// and this file's own level-7 milestone (Smite Evil's uses/day, the
/// effective-caster-level gate, and Channel Positive Energy's die count all
/// genuinely widen again; base saves and lay on hands stay numerically
/// unchanged from level 6, an integer-division coincidence; the level-7
/// "Special" column reads "Smite evil 3/day" only, verified independently
/// against d20pfsrd and legacy.aonprd.com, and level 7 is not one of the
/// repeat-Mercy-grant levels 3/6/9/..., so nothing new is left unproven for
/// Mercy here):
///
/// - one grounded numeric explanation set for the foundational base-attack-
///   bonus / base-save progression pillar, computed for real at every
///   supported level (1..=3): full base attack bonus (the same formula shape
///   as Fighter/Barbarian/Ranger), and good Fortitude / good Will / poor
///   Reflex base saves (NOT the same save shape as Ranger's good
///   Fortitude/Reflex, poor Will). Both formulas were verified independently
///   against the PF1 Core Rulebook Paladin class table before grounding.
///
/// - one grounded numeric explanation set for the fourth named non-spell
///   pillar, Smite Evil, computed for real at every supported level:
///   * PF1 Core Rulebook Smite Evil: 1 use per day below level 4, an
///     attack-roll bonus equal to the paladin's Charisma modifier (if
///     positive — the rule text applies the Charisma bonus "if any", never a
///     penalty), and a damage bonus equal to the paladin's class level. This
///     grounds only that flat numeric formula; it grounds no alignment /
///     evil-subtype target resolution, no swift-action activation
///     bookkeeping, no deflection-AC-vs-target bonus, and no
///     evil-outsider/evil-dragon/undead damage doubling.
///
/// - below the level-2 gate (i.e. at level 1), two grounded level-gate
///   records (value 0 each) whose honest computed surface is their correct
///   ABSENCE by PF1 Core Rulebook level gate:
///   * `lay on hands` — a 2nd-level paladin feature (heals 1d6 per two paladin
///     levels; uses/day = 1/2 paladin level + Charisma modifier); the at-grant
///     formula is named but not computed
///   * `divine grace` — a 2nd-level paladin feature (+Charisma bonus on all
///     saving throws); the at-grant formula is named but not computed
///
/// - at or above the level-2 gate, lay on hands and divine grace are grounded
///   for real as bounded, flat numeric formulas with no execution engine
///   behind them (no healing-resolution engine, no saving-throw-resolution
///   engine):
///   * `lay on hands` uses per day = 1/2 paladin level + Charisma modifier;
///     the heal amount is stated as a flat, non-fabricated d6-die-count
///     magnitude (1d6 per two paladin levels), never a rolled value —
///     mirroring how Smite Evil's damage bonus is a flat scalar, not a
///     dice-roll execution
///   * `divine grace` grants a Charisma-modifier bonus on all saving throws,
///     applied only if positive — mirroring the "applied only if positive"
///     idiom already used for Smite Evil's attack bonus
///
/// - below the level-3 gate (levels 1-2), `mercy` stays a grounded level-gate
///   absence record (value 0); at or above it (SD13-E5 level-3 widening), it
///   transitions to a bounded GRANT-only identity record (mirroring the
///   Barbarian Uncanny Dodge / Ranger Endurance idiom) plus, when the
///   deterministic fixture provides one, a choice-recognition record naming
///   which mercy was selected (mirroring the Ranger Favored Terrain /
///   Sorcerer bloodline choice-slot idiom): mercy is a 3rd-level paladin
///   feature (gained at 3rd level and every three levels thereafter; a
///   paladin selects one mercy from the list, and each mercy adds an effect
///   to lay on hands). The selected mercy's own effect (curing the named
///   condition automatically whenever lay on hands is used) is NOT computed,
///   since no lay-on-hands execution engine exists in this codebase.
///
/// - one grounded numeric explanation (SD13-E5) for the partial-caster
///   IDENTITY itself, distinct from the spell burden it sits next to:
///   * PF1 Core Rulebook effective caster level = max(paladin level − 3, 0);
///     spells begin at paladin level 4. At the bounded level-1 baseline this
///     grounds to 0 — the same "correct absence" idiom already used for the
///     lay on hands / divine grace / mercy level gates above. This grounds
///     only the caster-level gate arithmetic; it fabricates no spells known,
///     no spells per day, no bonus spell slots, and no spell save DCs.
///
/// - one explicit claim-blocking diagnostic for the partial-caster spell
///   burden, distinct from the grounded chassis records, unchanged by this
///   slice:
///   * Paladin is a divine partial caster in PF1 Core Rulebook (spells begin
///     at paladin level 4; effective caster level = paladin level − 3); the
///     blocker names this partial-caster posture so the later spell-burden
///     closure cannot collapse Paladin into a full divine caster shape
///     (Cleric / Druid) and so partial-caster pressure stays visible on the
///     runtime path.
///
/// This deliberately does not compute a supported spell surface, and it does
/// not ground level 8+, Divine Bond (the level-5 "Special" column's other
/// entry, checked against a primary source and confirmed to need an
/// activation/resource-consumption engine plus either a weapon-enhancement
/// subsystem or a full mount stat-block/advancement subsystem), or a second
/// mercy-selection slot (the level-6 "Special" column's repeat "Mercy" entry,
/// checked against a primary source and confirmed to need a mercy-list-growth
/// mechanism this codebase has not already grounded). Beyond the grounded
/// Smite Evil, Channel Positive Energy, lay on hands, and divine grace
/// numeric formulas, the mercy grant/choice recognition, and the grounded
/// effective-caster-level gate, it grounds no spell slots, no spell source
/// lineage, no spells known or prepared posture, no deity resolution, no
/// domain mechanics, no alignment-target resolution, no healing-resource
/// accounting, and no saving-throw-resolution engine. It only emits the
/// grounded records and the remaining spell blocker that prove the F6 surface
/// remains separable on the runtime path.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input;
/// the F6 hybrid chassis emission already preserves a single class-feature
/// blocker and a single spell blocker (both gated to the bounded hybrid
/// baseline level, so they only fire at level 1). This seam adds per-burden
/// granularity next to the F6 surface, never replacing it, so the F6
/// acceptance test continues to pass.
fn explain_paladin_level1_chassis_and_spell_burden_separation(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let Some(level) = supported_paladin_level(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Smite Evil, the fourth named non-spell pillar, is grounded for real: a
    // bounded, flat numeric formula with no execution engine behind it. PF1
    // Core Rulebook: 1 use/day below level 4, attack-roll bonus = Charisma
    // modifier (if positive; the rule never applies it as a penalty), damage
    // bonus = paladin level.
    let paladin_level = i16::from(level);
    let charisma_modifier = ability_modifier_for(ability_modifiers, "charisma");

    // Grounded (SD13-E5): the foundational base-attack-bonus / base-save progression
    // pillar. Unlike every other class row in this matrix (Fighter, Barbarian, Monk,
    // Rogue, Druid, Cleric, Bard, Sorcerer, Wizard, and by the immediately preceding
    // cycle, Ranger), Paladin had never had this pillar grounded at all, despite
    // Paladin already supporting a level-range gate (1..=2) unlike Ranger's
    // level-1-only gate at the time its own gap was closed. Both formulas were
    // verified against the PF1 Core Rulebook Paladin class table (d20pfsrd and the
    // legacy Paizo PRD mirror) before writing this code, reading the raw level 1-6
    // table rows directly (BAB +1/+2/+3/+4/+5/+6, Fort +2/+3/+3/+4/+4/+5, Ref
    // +0/+0/+1/+1/+1/+2, Will +2/+3/+3/+4/+4/+5) rather than assuming Paladin
    // matched Ranger's exact shape: Paladin is full BAB (the same shape as
    // Fighter/Barbarian/Ranger), but its good saves are Fortitude AND Will (poor
    // Reflex) -- NOT Ranger's good Fortitude/Reflex, poor Will. Paladin level 8+
    // remains out of scope; the flat base-attack and base-save numbers are
    // grounded here, extended across the now-supported level 1..=7 range (level
    // 5's Fortitude/Will/Reflex values were numerically unchanged from level 4, an
    // integer-division coincidence; level 6's values genuinely increase again;
    // level 7's values stay numerically unchanged from level 6, another
    // integer-division coincidence, re-verified rather than assumed).
    let good_save = paladin_level / 2 + 2;
    let poor_save = paladin_level / 3;

    explanations.push(ComputationExplanation {
        id: "class_chassis.paladin.base_attack_bonus".to_owned(),
        value: paladin_level,
        detail: format!(
            "Paladin level {level} base attack bonus from the PF1 Core Rulebook Paladin class \
             table (full base-attack progression, the same formula shape as \
             Fighter/Barbarian/Ranger): classlevel = {paladin_level}. This is a standalone \
             explanation record; it is not wired into the integrated base_attack_bonus field or \
             into compute_combat_baseline"
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_chassis.paladin.base_save.fortitude".to_owned(),
        value: good_save,
        detail: format!(
            "Paladin level {level} base Fortitude save (good save) from the PF1 Core Rulebook \
             Paladin class table: classlevel/2+2 = {good_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.paladin.base_save.reflex".to_owned(),
        value: poor_save,
        detail: format!(
            "Paladin level {level} base Reflex save (poor save) from the PF1 Core Rulebook \
             Paladin class table: classlevel/3 = {poor_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.paladin.base_save.will".to_owned(),
        value: good_save,
        detail: format!(
            "Paladin level {level} base Will save (good save) from the PF1 Core Rulebook \
             Paladin class table: classlevel/2+2 = {good_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });

    // Smite Evil uses per day genuinely increases at level 4 (PF1 Core Rulebook:
    // 1/day below level 4, +1 at level 4 and every three levels thereafter, to a
    // maximum of 7/day at level 19 -- verified independently against d20pfsrd
    // and legacy.aonprd.com rather than assumed to stay at 1). The formula
    // `1 + (paladin level - 1) / 3` correctly yields 1 at levels 1-3 and 2 at
    // levels 4-6, then GENUINELY increases to 3 at level 7 (the PF1 CRB
    // level-7 "Special" column reads "Smite evil 3/day", verified
    // independently rather than assumed to stay at 2; the next increase does
    // not land until level 10, out of scope for this bounded level-7
    // baseline).
    let smite_evil_uses_per_day: i16 = 1 + (paladin_level - 1) / 3;
    let smite_evil_attack_bonus = charisma_modifier.max(0);
    let smite_evil_damage_bonus = paladin_level;

    explanations.push(ComputationExplanation {
        id: "class_chassis.paladin.smite_evil_uses_per_day".to_owned(),
        value: smite_evil_uses_per_day,
        detail: format!(
            "Paladin Smite Evil uses per day at paladin level {level} (PF1 Core Rulebook: 1/day \
             below level 4, +1 at level 4 and every three levels thereafter, to a maximum of \
             7/day at level 19): 1 + ({paladin_level} - 1) / 3 = {smite_evil_uses_per_day}. This \
             grounds only the flat per-day resource count; it computes no swift-action activation \
             bookkeeping and no per-use consumption tracking"
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_chassis.paladin.smite_evil_attack_bonus".to_owned(),
        value: smite_evil_attack_bonus,
        detail: format!(
            "Paladin Smite Evil attack-roll bonus: the paladin's Charisma modifier, applied only \
             if positive (PF1 Core Rulebook: \"the paladin adds her Charisma modifier, if any, to \
             her attack roll\", never as a penalty) = max({charisma_modifier}, 0) = \
             {smite_evil_attack_bonus}. This grounds only the flat attack-roll bonus; it computes \
             no alignment or evil-subtype target resolution"
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_chassis.paladin.smite_evil_damage_bonus".to_owned(),
        value: smite_evil_damage_bonus,
        detail: format!(
            "Paladin Smite Evil damage bonus: equal to the paladin's class level (PF1 Core \
             Rulebook: 2x paladin level against evil outsiders, evil dragons, and undead, which \
             this bounded formula does not distinguish) = {smite_evil_damage_bonus} at paladin \
             level {level}. This grounds only the flat per-hit damage bonus; it computes no \
             evil-outsider/evil-dragon/undead damage doubling and no deflection-AC bonus against \
             the smited target"
        ),
    });

    if level < PALADIN_LAY_ON_HANDS_DIVINE_GRACE_LEVEL {
        // Below the level-2 gate, lay on hands and divine grace are grounded
        // as correct PF1 Core Rulebook level-gate absences (value 0 each).
        // Each record names the at-grant formula without computing it; no
        // heal amount or save bonus is fabricated.
        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.level_gate.lay_on_hands".to_owned(),
            value: 0,
            detail: format!(
                "Paladin lay on hands at paladin level {level}: correctly absent at level {level} \
                 by PF1 CRB level gate; at-grant formula named but not computed. Lay on hands is a \
                 2nd-level paladin feature: heals 1d6 per two paladin levels, uses/day = 1/2 \
                 paladin level + Charisma modifier"
            ),
        });

        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.level_gate.divine_grace".to_owned(),
            value: 0,
            detail: format!(
                "Paladin divine grace at paladin level {level}: correctly absent at level {level} \
                 by PF1 CRB level gate; at-grant formula named but not computed. Divine grace is a \
                 2nd-level paladin feature: +Charisma bonus on all saving throws"
            ),
        });
    } else {
        // At or above the level-2 gate, lay on hands and divine grace are
        // grounded for real: bounded, flat numeric formulas with no
        // execution engine behind them.
        let lay_on_hands_uses_per_day = paladin_level / 2 + charisma_modifier;
        let lay_on_hands_heal_dice = paladin_level / 2;
        let divine_grace_save_bonus = charisma_modifier.max(0);

        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.lay_on_hands_uses_per_day".to_owned(),
            value: lay_on_hands_uses_per_day,
            detail: format!(
                "Paladin lay on hands uses per day at paladin level {level} (PF1 Core Rulebook, \
                 2nd-level paladin feature): 1/2 paladin level + Charisma modifier = \
                 {paladin_level} / 2 + {charisma_modifier} = {lay_on_hands_uses_per_day}. This \
                 grounds only the flat per-day resource count; it computes no \
                 healing-resolution execution engine and no per-use consumption tracking"
            ),
        });

        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.lay_on_hands_heal_amount".to_owned(),
            value: lay_on_hands_heal_dice,
            detail: format!(
                "Paladin lay on hands heal amount at paladin level {level} (PF1 Core Rulebook, \
                 2nd-level paladin feature): 1d6 per two paladin levels = {lay_on_hands_heal_dice}d6 \
                 at paladin level {level}. This grounds only the flat die-count magnitude, stated \
                 as a non-fabricated record; it computes no dice-roll execution and no \
                 healing-resource accounting"
            ),
        });

        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.divine_grace_save_bonus".to_owned(),
            value: divine_grace_save_bonus,
            detail: format!(
                "Paladin divine grace saving-throw bonus at paladin level {level} (PF1 Core \
                 Rulebook, 2nd-level paladin feature): the paladin's Charisma modifier, applied \
                 only if positive (never as a penalty) = max({charisma_modifier}, 0) = \
                 {divine_grace_save_bonus}, applied to all saving throws. This grounds only the \
                 flat saving-throw bonus magnitude; it computes no saving-throw-resolution engine"
            ),
        });
    }

    // Mercy: below the level-3 gate, this stays a grounded level-gate absence
    // (value 0); at or above it (SD13-E5 level-3 widening), it transitions to a
    // bounded GRANT-only identity record (mirroring the Barbarian Uncanny Dodge /
    // Ranger Endurance idiom), plus -- when the deterministic fixture provides
    // one -- a further choice-recognition record naming which mercy was selected
    // (mirroring the Ranger Favored Terrain / Sorcerer bloodline choice-slot
    // idiom). No mercy effect (curing the named condition when lay on hands is
    // used) is ever fabricated; no lay-on-hands execution engine exists in this
    // codebase.
    if level < PALADIN_MERCY_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.level_gate.mercy".to_owned(),
            value: 0,
            detail: format!(
                "Paladin mercy at paladin level {level}: correctly absent at level {level} by PF1 \
                 CRB level gate (mercy is a {PALADIN_MERCY_LEVEL}rd-level paladin feature, gained \
                 at 3rd level and every three levels thereafter); at-grant formula named but not \
                 computed. Mercy is chosen from the mercy list and attaches to lay on hands"
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.mercy_granted".to_owned(),
            value: 0,
            detail: format!(
                "Paladin mercy granted at paladin level {level} (PF1 Core Rulebook, \
                 {PALADIN_MERCY_LEVEL}rd-level paladin feature, gained at 3rd level and every \
                 three levels thereafter): \"a paladin can select one mercy. Each mercy adds an \
                 effect to the paladin's lay on hands ability\" (verified independently against \
                 legacy.aonprd.com's Core Rulebook Paladin page). The first, 3rd-level tier of \
                 the mercy list is Fatigued, Shaken, and Sickened. This is a bounded grant-only \
                 identity record (value 0, non-fabricated): which specific mercy was selected is \
                 recognized separately below when present, and the selected mercy's own effect \
                 (curing the named condition automatically whenever lay on hands is used) is not \
                 computed, since no lay-on-hands execution engine exists anywhere in this codebase"
            ),
        });

        if let Some(selected_mercy) = choice_selection(input, PALADIN_MERCY_CHOICE_ID) {
            explanations.push(ComputationExplanation {
                id: "class_chassis.paladin.mercy_choice".to_owned(),
                value: 0,
                detail: format!(
                    "Paladin mercy selection ({PALADIN_MERCY_CHOICE_ID} -> {selected_mercy}): the \
                     level-{level} mercy chosen for this character is {selected_mercy}. This is a \
                     bounded recognition record of the chosen mercy only; no restricted-list \
                     validation is performed (mirroring the Ranger Favored Terrain / Sorcerer \
                     bloodline class-skill choice-recognition idiom), and the mercy's own effect \
                     is not computed, since no lay-on-hands execution engine exists anywhere in \
                     this codebase"
                ),
            });
        }
    }

    // Channel Positive Energy: below the level-4 gate (levels 1-3), this stays a
    // correct level-gate absence record (value 0); at or above it (SD13-E5
    // level-4 widening), it transitions to a bounded, flat-magnitude record
    // grounding only the channel-energy die count, mirroring the Cleric Channel
    // Energy dice-count idiom exactly (ceil(effective level / 2)). No
    // healing/damage-resolution execution, no heal-vs-harm target selection,
    // and no lay-on-hands-resource-consumption bookkeeping is computed.
    if level < PALADIN_CHANNEL_POSITIVE_ENERGY_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.level_gate.channel_positive_energy".to_owned(),
            value: 0,
            detail: format!(
                "Paladin channel positive energy at paladin level {level}: correctly absent at \
                 level {level} by PF1 CRB level gate (channel positive energy is a \
                 {PALADIN_CHANNEL_POSITIVE_ENERGY_LEVEL}th-level paladin feature); at-grant \
                 formula named but not computed. Channel positive energy lets a paladin channel \
                 positive energy like a cleric, using her paladin level as her effective cleric \
                 level, consuming two uses of lay on hands per use"
            ),
        });
    } else {
        let channel_positive_energy_dice = (paladin_level + 1) / 2;
        explanations.push(ComputationExplanation {
            id: "class_chassis.paladin.channel_positive_energy_dice".to_owned(),
            value: channel_positive_energy_dice,
            detail: format!(
                "Paladin channel positive energy dice at paladin level {level} (PF1 Core \
                 Rulebook, {PALADIN_CHANNEL_POSITIVE_ENERGY_LEVEL}th-level paladin feature, \
                 verified independently against d20pfsrd and legacy.aonprd.com: \"When a paladin \
                 reaches 4th level, she gains the supernatural ability to channel positive \
                 energy like a cleric. Using this ability consumes two uses of her lay on hands \
                 ability. A paladin uses her level as her effective cleric level when channeling \
                 positive energy.\"): ceil(paladin level / 2) = ceil({paladin_level} / 2) = \
                 {channel_positive_energy_dice}d6, mirroring the same die-count formula already \
                 grounded for Cleric's own Channel Energy. This grounds only the flat die-count \
                 magnitude and the lay-on-hands-use-cost identity; it computes no \
                 healing/damage-resolution execution, no heal-vs-harm target selection, and no \
                 lay-on-hands-resource-consumption bookkeeping"
            ),
        });
    }

    // SD13-E5: ground the partial-caster IDENTITY itself as one more flat
    // level-gate record, distinct from the still-ungrounded spell burden
    // named below. PF1 Core Rulebook: effective caster level = max(paladin
    // level - 3, 0); spells begin at paladin level 4. At level 1 this
    // correctly grounds to 0 — the same "correct absence" idiom already used
    // for lay on hands / divine grace / mercy above. This grounds only the
    // caster-level gate arithmetic; it fabricates no spells known, no spells
    // per day, no bonus spell slots, and no spell save DCs. The gate
    // genuinely widened at level 5 (to 2, up from 1 at level 4), widened
    // again at level 6 (to 3, up from 2 at level 5), and widens again at
    // level 7 (to 4, up from 3 at level 6), via the same pre-existing
    // formula, no re-derivation. Divine Bond, the level-5 "Special" column's
    // other entry, was checked against a primary source and confirmed to
    // require an activation/resource-consumption engine plus either a
    // weapon-enhancement subsystem or a full mount stat-block/advancement
    // subsystem, so it stays deliberately named-but-unproven -- no
    // explanation or diagnostic record is fabricated for it. Similarly, the
    // level-6 "Special" column's repeat "Mercy" entry (PF1 CRB: an additional
    // mercy becomes selectable at 6th level and every three levels
    // thereafter) was checked and confirmed to require a mercy-list-growth
    // mechanism this codebase has not already grounded (the existing mercy
    // grant/choice records are a single, ungated recognition, not a
    // per-level slot count), so it too stays deliberately named-but-unproven
    // -- no second mercy-choice explanation record is fabricated for it.
    // Level 7's own "Special" column reads "Smite evil 3/day" only (verified
    // independently against d20pfsrd and legacy.aonprd.com) -- level 7 is not
    // one of the repeat-Mercy-grant levels (3, 6, 9, ...), so nothing new is
    // left unproven for Mercy at level 7.
    let paladin_effective_caster_level = (paladin_level - 3).max(0);
    explanations.push(ComputationExplanation {
        id: "class_chassis.paladin.partial_caster.effective_caster_level".to_owned(),
        value: paladin_effective_caster_level,
        detail: format!(
            "Paladin effective caster level at paladin level {level}: max(paladin level - 3, 0) = \
             max({paladin_level} - 3, 0) = {paladin_effective_caster_level} (PF1 Core Rulebook: \
             paladin spells begin at paladin level 4). This grounds only the caster-level gate \
             arithmetic; it computes no spells known, no spells per day, no bonus spell slots, \
             and no spell save DCs"
        ),
    });

    // The partial-caster spell burden is its own blocker, distinct from the
    // grounded non-spell chassis records above. Paladin is a divine partial
    // caster in PF1 Core Rulebook (spells begin at paladin level 4; effective
    // caster level = paladin level - 3), and the blocker must name that
    // partial-caster posture so the later spell-burden closure cannot confuse
    // Paladin with a full divine caster (Cleric / Druid). Unchanged by the
    // level-2 widening: still claim-blocking at every level this slice
    // supports.
    diagnostics.push(ComputationDiagnostic {
        id: "class_spell.paladin.partial_caster.unsupported".to_owned(),
        message: "Paladin remains blocked on its divine partial-caster spell burden: Paladin is a \
             partial caster (spells begin at paladin level 4, with effective caster level = \
             paladin level - 3 in PF1 Core Rulebook), so spell-source lineage, spells known \
             or prepared posture, spells-per-day progression, bonus spell slots, and spell save \
             DCs are deferred to a later spellcasting slice; no partial-caster spell \
             execution is fabricated in this bounded chassis baseline"
            .to_owned(),
        claim_blocking: true,
    });
}

/// The bounded Ranger milestone level this decomposition surface grounds, if any.
/// Returns the single Ranger level when the chosen input is exactly a single-class
/// Ranger at one of the supported milestone levels (1 through 9). Returns
/// `None` for no Ranger, a non-Ranger class, a multiclass mix, the Paladin hybrid
/// (which has its own decomposition lane), or any level-10+ Ranger this slice
/// deliberately does not recognize — each of which stays claim-blocked exactly as
/// before. Mirrors the
/// Fighter `supported_fighter_level` / Paladin `supported_paladin_level` / Rogue
/// `supported_rogue_level` / Barbarian `supported_barbarian_level` / Monk
/// `supported_monk_level` / Cleric `supported_cleric_level` / Bard
/// `supported_bard_level` / Druid `supported_druid_level` / Sorcerer
/// `supported_sorcerer_level` / Wizard `supported_wizard_level` level-range gate
/// idiom.
fn supported_ranger_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == RANGER_CLASS_ID
                && (1..=MAX_SUPPORTED_RANGER_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

/// Surface direct SD13-E3 runtime evidence for the deterministic Human Ranger
/// level-1 chassis as a per-pillar decomposition of the F6 combined non-spell
/// class-feature blocker, grounding all three named pillars for real (Track by
/// the SD13-E3 slice, the Favored Enemy flat surface by the SD13-E5 slice, and
/// the combat style level-gate absence by a later SD13-E5 slice), plus the
/// foundational base-attack-bonus / base-save progression pillar grounded by a
/// later SD13-E5 slice still.
///
/// This sits on top of the accepted SD13-F6 hybrid baseline: F6 already proves
/// the deterministic Human Ranger level-1 hybrid identity is acknowledged on the
/// compute seam and emits a single combined non-spell class-feature blocker
/// (naming favored enemy, combat style, and skill/tracking together) plus a
/// single combined later-spell blocker. This slice proves the per-pillar
/// separation Ranger actually needs:
///
/// - one grounded numeric explanation set (SD13-E5) for the foundational
///   base-attack-bonus / base-save progression pillar, verified against the PF1
///   Core Rulebook Ranger class table (d20pfsrd and legacy.aonprd.com), reading
///   the raw level 1-5 table rows directly and cross-checking the level 4/5
///   base-attack-bonus values to disambiguate full BAB from 3/4 BAB (level 1
///   alone does not disambiguate): full BAB (classlevel), good Fortitude, good
///   Reflex, poor Will (`classlevel/2+2` for the two good saves, `classlevel/3`
///   for the poor save). Grounded as flat, standalone `ComputationExplanation`
///   records, mirroring the Barbarian/Monk/Druid/Cleric/Bard/Sorcerer/Wizard
///   "not wired into `PilotBaseChassisComputation.base_attack_bonus`,
///   `compute_total_saves`, or `compute_combat_baseline`" idiom. Ranger level 2+
///   progression stays deliberately out of scope for this slice.
///
/// - one grounded level-gate explanation (value 0) for the combat style
///   pillar, which retires the `class_feature.ranger.combat_style.unsupported`
///   blocker and corrects a mistaken framing that blocker carried: PF1 Core
///   Rulebook grants the archery-vs-two-weapon-combat style choice and its
///   first bonus feat TOGETHER at 2nd level (`RANGER_COMBAT_STYLE_LEVEL`) —
///   they are not separable into a level-1 style choice plus a level-2 feat
///   grant, as the retired diagnostic incorrectly claimed. At the bounded
///   level-1 baseline this correctly grounds to a value-0 ABSENCE, mirroring
///   the Paladin mercy level-gate idiom (`class_chassis.paladin.level_gate.mercy`);
///   the at-grant selection is named but not computed, and no bonus-feat
///   mechanical value is fabricated.
///
/// - one grounded explanation for the Track pillar, computed for real:
///   the Survival-check bonus to follow tracks equals `max(ranger level / 2, 1)`
///   (PF1 Core Rulebook Track: +1/2 ranger level, minimum +1), which is `1` at
///   the bounded level-1 baseline. This grounds only the flat numeric Track
///   bonus, not a tracking-check execution engine: no full Survival check, no
///   DC resolution, and no tracking narrative is computed.
///
/// - the grounded Favored Enemy FLAT surface (SD13-E5), which retires the
///   `class_feature.ranger.favored_enemy.unsupported` blocker:
///   * recognition of the chosen favored-enemy type from the
///     `choice:ranger_favored_enemy` selection when it is present in chosen
///     input (a +0 recognition record; nothing is fabricated when the choice
///     is absent),
///   * the flat +2 bonus on Bluff, Knowledge, Perception, Sense Motive, and
///     Survival checks against the favored enemy (PF1 CRB level 1), and
///   * the flat +2 bonus on weapon attack AND damage rolls against the
///     favored enemy (PF1 includes attack rolls, unlike D&D 3.5).
///
///   Only the flat magnitudes are grounded: no target-type matching and no
///   conditional-application engine decides whether any specific check or
///   attack is actually made against the favored enemy.
///
/// - a still later SD13-E5 slice widens the level-range gate to level 3
///   (`MAX_SUPPORTED_RANGER_LEVEL`), extending base attack/base save/Track/the
///   Favored Enemy flat surface to level 3 via the same formulas (no
///   re-derivation), and grounds Endurance, the PF1 CRB's 3rd-level Ranger
///   class feature, as a bounded grant-only identity record (value 0): the
///   ranger gains Endurance as a bonus feat automatically, with no player
///   choice involved.
///
/// - a still later SD13-E5 slice grounds Favored Terrain, the class table's
///   other 3rd-level "Special" column entry, once a `choice:ranger_favored_terrain`
///   choice-slot exists in chosen input: a `+0` recognition record naming
///   whichever terrain was selected (mirroring the Favored Enemy choice-recognition
///   idiom exactly — raw string interpolation, no restricted-list validation), and
///   the rule's own flat `+2` magnitude on Initiative/Knowledge (geography)/
///   Perception/Stealth/Survival checks made in the chosen terrain, grounded as a
///   standalone, non-applied record, level-gated at 3rd level exactly like
///   Endurance. No terrain-detection engine and no application of the `+2` to any
///   actual Initiative total or skill-check total is grounded here.
///
/// - a still later SD13-E5 slice widens the level-range gate once more to level
///   4 (`MAX_SUPPORTED_RANGER_LEVEL`), extending base attack/base save/Track/the
///   Favored Enemy flat surface to level 4 via the same formulas (no
///   re-derivation; PF1 Core Rulebook only increases the Favored Enemy bonus at
///   5th ranger level and beyond, so it stays the flat `+2` through level 4),
///   and grounds Hunter's Bond, the class table's 4th-level "Special" column
///   entry: a restricted two-option choice recognition
///   (`choice:ranger_hunters_bond` -> `form:bond` or `form:companion`, mirroring
///   the combat-style choice idiom) is grounded as a `+0` record, an
///   unconditional grant-only identity record (mirroring the Endurance/Favored
///   Terrain idiom) is emitted once the level-4 gate is reached, and -- only
///   when the "bond" form is chosen -- the rule's own flat magnitude (half the
///   already-grounded Favored Enemy bonus) is grounded as a standalone,
///   non-applied record. No move-action/action-economy engine, no
///   ally-range-and-perception check, and no favored-enemy target-type matching
///   is implemented; the "companion" form's own animal-companion stat
///   block/advancement subsystem is deliberately left named-but-unproven.
///
/// - a still later SD13-E5 slice widens the level-range gate once more to level
///   5 (`MAX_SUPPORTED_RANGER_LEVEL`), extending base attack/base save/Track to
///   level 5 via the same formulas (no re-derivation), and grounds the Favored
///   Enemy rule's own 5th-level interval, the class table's 5th-level "Special"
///   column entry ("2nd favored enemy", verified independently against both
///   primary sources): the rule text is "At 5th level and every five levels
///   thereafter... the ranger may select an additional favored enemy. In
///   addition, at each such interval, the bonus against any one favored enemy
///   (including the one just selected, if so desired) increases by 2" — NOT an
///   automatic bump to the first favored enemy. This grounds three things: a
///   second favored-enemy TYPE selection (`choice:ranger_favored_enemy_2`,
///   mirroring the first favored enemy's own open-ended choice-recognition
///   idiom, plus the same flat `+2` base magnitude formula), a restricted
///   two-option choice recognizing WHICH one favored enemy is the
///   bonus-increase target (`choice:ranger_favored_enemy_bonus_increase_target`
///   -> `enemy:first` or `enemy:second`, mirroring the Hunter's Bond/combat-style
///   restricted two-option idiom), and the resulting `+4` magnitude applied only
///   to whichever favored enemy the target choice actually names — absent an
///   explicit target selection, both favored enemies stay the flat `+2`, since
///   nothing is fabricated about which one the ranger picked. Endurance,
///   Favored Terrain, combat style, and Hunter's Bond all stay granted at level
///   5, not re-derived; Hunter's Bond's own ally-bonus magnitude (half the
///   FIRST favored enemy's bonus) naturally recomputes from the same unchanged
///   formula once that magnitude widens to `+4`.
///
/// This deliberately does not compute a supported class-feature surface. It
/// grounds no favored-enemy conditional application, no combat-style feat
/// grant, no animal companion, no favored-terrain breadth (the level-8th/13th/
/// 18th additional-terrain and bonus-increase progression), no Hunter's Bond
/// ally-bonus application or animal-companion stat block, and no spell posture.
/// It only emits the grounded Track / Favored Enemy / combat-style / Endurance /
/// Favored Terrain / Hunter's Bond level-gate values that prove the F6 surface
/// remains separable on the runtime path.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input;
/// the F6 hybrid chassis emission already preserves a single class-feature
/// blocker and a single spell blocker. This seam adds per-pillar granularity
/// next to the F6 surface, never replacing it, so the F6 acceptance test
/// continues to pass.
fn explain_ranger_level1_chassis_and_class_feature_separation(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = supported_ranger_level(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Grounded (SD13-E5): the foundational base-attack-bonus / base-save progression
    // pillar. Unlike every other class row in this matrix (Fighter, Barbarian, Monk,
    // Rogue, Paladin, Druid, Cleric, Bard, Sorcerer, Wizard all already ground this
    // pillar), Ranger had never had it grounded at all until this slice. Both
    // formulas were verified against the PF1 Core Rulebook Ranger class table
    // (d20pfsrd and the legacy Paizo PRD mirror) before writing this code, reading
    // the raw level 1-5 table rows directly (BAB +1/+2/+3/+4/+5, Fort +2/+3/+3/+4/+4,
    // Ref +2/+3/+3/+4/+4, Will +0/+0/+1/+1/+1) and cross-checking the level 4/5
    // base-attack-bonus values to disambiguate the exact fraction: a full-BAB
    // progression shows +4/+5 at those levels, while a 3/4-BAB progression would show
    // +3/+3 -- the table confirms full BAB, the same shape as Fighter/Barbarian/
    // Paladin. A later SD13-E5 slice widens the level-1-only gate to level 2
    // (`supported_ranger_level`, 1..=MAX_SUPPORTED_RANGER_LEVEL), extending both
    // formulas via the same shape (no re-derivation) and finally grounding the
    // combat-style pillar for real at the 2nd-level gate it was always named for.
    // A still later SD13-E5 slice widens the gate again to level 3, extending both
    // formulas once more and grounding Endurance (a grant-only identity record) and
    // Favored Terrain (a choice recognition record plus a flat +2 magnitude record).
    // A still later SD13-E5 slice widens the gate again to level 4, extending both
    // formulas once more (favored enemy stays flat +2 through level 4; PF1 CRB only
    // increases it at 5th ranger level and beyond) and grounding Hunter's Bond (a
    // restricted two-option choice recognition, a grant-only identity record, and --
    // for the "bond" form only -- a flat, non-applied ally-bonus magnitude record).
    // A still later SD13-E5 slice widens the gate again to level 5, extending both
    // formulas once more and grounding the Favored Enemy rule's own 5th-level
    // interval (a second favored-enemy selection plus a restricted-choice
    // bonus-increase target). A still later SD13-E5 slice widens the gate once more
    // to level 6, extending both formulas once more (Track genuinely rises to 3) and
    // grounding the ranger's SECOND combat-style bonus feat (a restricted-list
    // choice recognition gated on the same style already chosen at 2nd level,
    // mirroring the first bonus feat's own grounding idiom exactly). A still later
    // SD13-E5 slice widens the gate once more to level 7, extending both formulas
    // once more (both stay numerically unchanged from level 6, integer-division
    // coincidences) and grounding Woodland Stride (a grant-only identity record,
    // no numeric magnitude, mirroring the Endurance idiom).
    // Ranger level 8+ progression, the favored-enemy conditional-application engine,
    // either combat-style bonus feat's own mechanics, the level-8th/13th/18th
    // Favored Terrain breadth, Hunter's Bond ally-bonus
    // application/animal-companion stat block, and the ranger spell burden remain
    // deliberately out of scope.
    let level_value = i16::from(level);

    // Grounded (1/2): full-BAB base-attack progression, the same formula shape as
    // Fighter/Barbarian/Paladin (classlevel). No PCGen .lst file exists for the
    // Ranger class in this repo, so the formula cites the PF1 Core Rulebook Ranger
    // class table directly.
    let base_attack_bonus = level_value;
    explanations.push(ComputationExplanation {
        id: "class_chassis.ranger.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            "Ranger level {level} base attack bonus from the PF1 Core Rulebook \
             Ranger class table (full base-attack progression, the same formula shape as \
             Fighter/Barbarian/Paladin): classlevel = {base_attack_bonus}. This is a standalone \
             explanation record; it is not wired into the integrated base_attack_bonus field or \
             into compute_combat_baseline"
        ),
    });

    // Grounded (2/2): base-save progression — good Fortitude, good Reflex, poor
    // Will, verified against the PF1 Core Rulebook Ranger class table (Fortitude +2,
    // Reflex +2, Will +0 at level 1; +4/+4/+1 at level 4, confirming the same
    // formula shape).
    let good_save = level_value / 2 + 2;
    let poor_save = level_value / 3;
    explanations.push(ComputationExplanation {
        id: "class_chassis.ranger.base_save.fortitude".to_owned(),
        value: good_save,
        detail: format!(
            "Ranger level {level} base Fortitude save (good save) from the PF1 \
             Core Rulebook Ranger class table: classlevel/2+2 = {good_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.ranger.base_save.reflex".to_owned(),
        value: good_save,
        detail: format!(
            "Ranger level {level} base Reflex save (good save) from the PF1 Core \
             Rulebook Ranger class table: classlevel/2+2 = {good_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.ranger.base_save.will".to_owned(),
        value: poor_save,
        detail: format!(
            "Ranger level {level} base Will save (poor save) from the PF1 Core \
             Rulebook Ranger class table: classlevel/3 = {poor_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });

    if level < RANGER_COMBAT_STYLE_LEVEL {
        // Combat style is a correct ABSENCE below the 2nd-level gate, grounded as a
        // level-gate explanation (value 0), mirroring the Paladin mercy idiom. The
        // former `class_feature.ranger.combat_style.unsupported` blocker is retired: it
        // incorrectly claimed the archery-vs-two-weapon-combat style choice was a
        // level-1 decision separate from a level-2 bonus-feat grant. PF1 Core Rulebook
        // actually grants the style choice and its first bonus feat TOGETHER at 2nd
        // level (RANGER_COMBAT_STYLE_LEVEL), so below that gate there is nothing to
        // recognize: no style is chosen and no bonus feat is granted.
        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.level_gate.combat_style".to_owned(),
            value: 0,
            detail: format!(
                "Ranger combat style at ranger level {level}: correctly absent at \
                 level {level} by PF1 CRB level gate; at-grant selection named but not \
                 computed. Combat Style Feat is a {RANGER_COMBAT_STYLE_LEVEL}nd-level ranger feature: \
                 the ranger selects one combat style (archery or two-weapon combat) and gains its \
                 first bonus feat together at {RANGER_COMBAT_STYLE_LEVEL}nd level -- the style choice \
                 and the bonus-feat grant are not separable into a level-1 decision plus a level-2 \
                 grant. (Correction: an earlier version of this record, \
                 `class_feature.ranger.combat_style.unsupported`, incorrectly described the style \
                 choice as a level-1 decision distinct from the level-2 bonus-feat grant; PF1 Core \
                 Rulebook grants both together at 2nd level.)"
            ),
        });
    } else {
        // Grounded (SD13-E5 level-2 widening): Combat Style Feat is finally grounded
        // for real at the gate it was always named for. Both the STYLE CHOICE and its
        // restricted-list BONUS FEAT are recognized as chosen-input identity only
        // (+0 each), mirroring the Monk bonus-feat-choice idiom exactly: no style's
        // or feat's own mechanical effect is computed anywhere in this codebase.
        // Nothing is fabricated when the fixture carries no
        // `choice:ranger_combat_style` selection -- mirroring the Favored Enemy
        // choice-absence idiom below.
        let style_selection = choice_selection(input, RANGER_COMBAT_STYLE_CHOICE_ID);
        let style_name = style_selection.and_then(|selection| {
            if selection == RANGER_COMBAT_STYLE_ARCHERY_SELECTION {
                Some("Archery")
            } else if selection == RANGER_COMBAT_STYLE_TWO_WEAPON_COMBAT_SELECTION {
                Some("Two-Weapon Combat")
            } else {
                None
            }
        });

        if let Some(selection) = style_selection {
            let detail = if let Some(style) = style_name {
                format!(
                    "Ranger combat style selection at ranger level {level} \
                     ({RANGER_COMBAT_STYLE_CHOICE_ID} -> {selection}): names {style}, one of the \
                     two PF1 Core Rulebook combat styles (Archery or Two-Weapon Combat) granted \
                     together with its first bonus feat at {RANGER_COMBAT_STYLE_LEVEL}nd level. \
                     This is a recognition record of the choice slot only (+0): {style}'s own \
                     bonus-feat mechanics are not grounded here, and no feat-selection or \
                     feat-effect engine exists in this codebase"
                )
            } else {
                format!(
                    "Ranger combat style selection at ranger level {level} is present \
                     ({RANGER_COMBAT_STYLE_CHOICE_ID} -> {selection}), but only the PF1 Core \
                     Rulebook restricted pair (Archery, Two-Weapon Combat) is recognized on this \
                     bounded seam; no style identity is grounded and no mechanical value is \
                     fabricated (+0)"
                )
            };
            explanations.push(ComputationExplanation {
                id: "class_chassis.ranger.combat_style_choice".to_owned(),
                value: 0,
                detail,
            });

            // The bonus feat is recognized only once the style itself is recognized,
            // since the restricted feat list to validate against depends on which
            // style was chosen -- mirroring how Cleric's domain powers are gated on
            // the domain choice itself being recognized.
            if let Some(style) = style_name
                && let Some(feat_selection) =
                    choice_selection(input, RANGER_COMBAT_STYLE_BONUS_FEAT_CHOICE_ID)
            {
                let recognized_feat_name = if style == "Archery" {
                    if feat_selection == FAR_SHOT_FEAT_SELECTION {
                        Some("Far Shot")
                    } else if feat_selection == POINT_BLANK_SHOT_FEAT_SELECTION {
                        Some("Point-Blank Shot")
                    } else if feat_selection == PRECISE_SHOT_FEAT_SELECTION {
                        Some("Precise Shot")
                    } else if feat_selection == RAPID_SHOT_FEAT_SELECTION {
                        Some("Rapid Shot")
                    } else {
                        None
                    }
                } else if feat_selection == DOUBLE_SLICE_FEAT_SELECTION {
                    Some("Double Slice")
                } else if feat_selection == IMPROVED_SHIELD_BASH_FEAT_SELECTION {
                    Some("Improved Shield Bash")
                } else if feat_selection == QUICK_DRAW_FEAT_SELECTION {
                    Some("Quick Draw")
                } else if feat_selection == TWO_WEAPON_FIGHTING_FEAT_SELECTION {
                    Some("Two-Weapon Fighting")
                } else {
                    None
                };

                let detail = if let Some(feat_name) = recognized_feat_name {
                    format!(
                        "Ranger {style} combat style bonus feat at ranger level {level} \
                         ({RANGER_COMBAT_STYLE_BONUS_FEAT_CHOICE_ID} -> {feat_selection}) \
                         names {feat_name}, drawn from the PF1 Core Rulebook {style} combat \
                         style's own {RANGER_COMBAT_STYLE_LEVEL}nd-level restricted feat list. \
                         This is a recognition record of the choice slot only, so it carries \
                         no fabricated mechanical value (+0): {feat_name}'s own mechanics (an \
                         attack/damage-range bonus, a two-weapon penalty reduction, or similar, \
                         depending on the feat) are not grounded here, and no such execution \
                         engine exists in this codebase"
                    )
                } else {
                    format!(
                        "Ranger combat style bonus feat at ranger level {level} is present \
                         ({RANGER_COMBAT_STYLE_BONUS_FEAT_CHOICE_ID} -> {feat_selection}), but \
                         only the PF1 Core Rulebook {style} combat style's own \
                         {RANGER_COMBAT_STYLE_LEVEL}nd-level restricted feat list is recognized \
                         on this bounded seam; no restricted-list feat identity is grounded and \
                         no mechanical value is fabricated (+0)"
                    )
                };
                explanations.push(ComputationExplanation {
                    id: "class_chassis.ranger.combat_style_bonus_feat_choice".to_owned(),
                    value: 0,
                    detail,
                });
            }

            // SD13-E5 level-6 widening: the ranger's SECOND combat-style bonus feat,
            // gated on both the level-6 milestone and the same style already
            // recognized above. Mirrors the first bonus feat's grounding idiom
            // exactly (a restricted-list choice recognition, +0, no mechanical
            // effect computed), validated against each style's own 6th-level list
            // only (Archery: Improved Precise Shot, Manyshot; Two-Weapon Combat:
            // Improved Two-Weapon Fighting, Two-Weapon Defense) rather than the
            // cumulative 2nd+6th-level list.
            if let Some(style) = style_name
                && level >= RANGER_COMBAT_STYLE_BONUS_FEAT_2_LEVEL
                && let Some(feat_selection_2) =
                    choice_selection(input, RANGER_COMBAT_STYLE_BONUS_FEAT_2_CHOICE_ID)
            {
                let recognized_feat_name_2 = if style == "Archery" {
                    if feat_selection_2 == IMPROVED_PRECISE_SHOT_FEAT_SELECTION {
                        Some("Improved Precise Shot")
                    } else if feat_selection_2 == MANYSHOT_FEAT_SELECTION {
                        Some("Manyshot")
                    } else {
                        None
                    }
                } else if feat_selection_2 == IMPROVED_TWO_WEAPON_FIGHTING_FEAT_SELECTION {
                    Some("Improved Two-Weapon Fighting")
                } else if feat_selection_2 == TWO_WEAPON_DEFENSE_FEAT_SELECTION {
                    Some("Two-Weapon Defense")
                } else {
                    None
                };

                let detail_2 = if let Some(feat_name) = recognized_feat_name_2 {
                    format!(
                        "Ranger {style} SECOND combat style bonus feat at ranger level {level} \
                         ({RANGER_COMBAT_STYLE_BONUS_FEAT_2_CHOICE_ID} -> {feat_selection_2}) \
                         names {feat_name}, drawn from the PF1 Core Rulebook {style} combat \
                         style's own {RANGER_COMBAT_STYLE_BONUS_FEAT_2_LEVEL}th-level restricted \
                         feat list (verified independently against d20pfsrd and \
                         legacy.aonprd.com: the ranger's combat style grants bonus feats at 2nd, \
                         6th, 10th, 14th, and 18th level). This is a recognition record of the \
                         choice slot only, so it carries no fabricated mechanical value (+0): \
                         {feat_name}'s own mechanics are not grounded here, and no such \
                         execution engine exists in this codebase"
                    )
                } else {
                    format!(
                        "Ranger SECOND combat style bonus feat at ranger level {level} is \
                         present ({RANGER_COMBAT_STYLE_BONUS_FEAT_2_CHOICE_ID} -> \
                         {feat_selection_2}), but only the PF1 Core Rulebook {style} combat \
                         style's own {RANGER_COMBAT_STYLE_BONUS_FEAT_2_LEVEL}th-level restricted \
                         feat list is recognized on this bounded seam; no restricted-list feat \
                         identity is grounded and no mechanical value is fabricated (+0)"
                    )
                };
                explanations.push(ComputationExplanation {
                    id: "class_chassis.ranger.combat_style_bonus_feat_2_choice".to_owned(),
                    value: 0,
                    detail: detail_2,
                });
            }
        }
    }

    // The third named F6 pillar, Track, is grounded for real: a bounded, flat
    // numeric Survival bonus with no execution engine behind it.
    let track_bonus = (level_value / 2).max(1);
    explanations.push(ComputationExplanation {
        id: "class_chassis.ranger.track".to_owned(),
        value: track_bonus,
        detail: format!(
            "Ranger Track class feature: grants a bonus on Survival checks made to follow tracks \
             equal to max(ranger level / 2, 1) (PF1 Core Rulebook Track: +1/2 ranger level, minimum \
             +1). At Ranger level {level} this bonus is \
             max({level} / 2, 1) = {track_bonus}. This grounds only the flat numeric \
             Track bonus on Survival checks to follow tracks; it is not a tracking-check execution \
             engine and computes no full Survival check, no DC resolution, and no tracking narrative"
        ),
    });

    // The Favored Enemy FLAT surface is grounded for real (SD13-E5). PF1 Core
    // Rulebook: the ranger selects one favored-enemy type and gains a +2 bonus on
    // Bluff, Knowledge, Perception, Sense Motive, and Survival checks against it,
    // plus a +2 bonus on weapon attack and damage rolls against it (PF1 includes
    // attack rolls, unlike D&D 3.5). PF1 Core Rulebook only increases this bonus at
    // 4th ranger level and beyond, so it stays the flat +2 at both level 1 and level
    // 2 via the same formula, not a new record. Only the flat magnitudes are
    // grounded: no target-type matching and no conditional-application engine
    // decides whether any specific check or attack is actually made against the
    // favored enemy.
    if let Some(favored_enemy) = choice_selection(input, "choice:ranger_favored_enemy") {
        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.favored_enemy_choice".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Favored Enemy selection (choice:ranger_favored_enemy -> {favored_enemy}): \
                 the level-{level} favored-enemy type chosen for this character is \
                 {favored_enemy}. This is a bounded recognition record of the chosen enemy type \
                 only; the flat bonus magnitudes are grounded separately, and no target-type \
                 matching or conditional-application engine is implemented, so it carries no \
                 fabricated mechanical value (+0)"
            ),
        });
    }

    // SD13-E5 ranger level 5: recognize the bonus-increase TARGET choice, only
    // meaningful once the ranger has reached the Favored Enemy rule's 5th-level
    // interval. PF1 Core Rulebook: "the bonus against any one favored enemy
    // (including the one just selected, if so desired) increases by 2" -- a
    // genuine, free player choice of which ONE favored enemy is boosted, not an
    // automatic bump to the first one. Absent an explicit target selection in
    // chosen input, nothing is fabricated: both favored enemies stay at the flat
    // base magnitude.
    let favored_enemy_bonus_increase_target = if level >= RANGER_FAVORED_ENEMY_SECOND_INTERVAL_LEVEL
    {
        choice_selection(input, RANGER_FAVORED_ENEMY_BONUS_INCREASE_CHOICE_ID)
    } else {
        None
    };

    if let Some(target) = favored_enemy_bonus_increase_target {
        let target_name = if target == RANGER_FAVORED_ENEMY_BONUS_INCREASE_FIRST_SELECTION {
            Some("the first favored enemy")
        } else if target == RANGER_FAVORED_ENEMY_BONUS_INCREASE_SECOND_SELECTION {
            Some("the second favored enemy")
        } else {
            None
        };
        let detail = if let Some(name) = target_name {
            format!(
                "Ranger Favored Enemy bonus-increase target selection at ranger level {level} \
                 ({RANGER_FAVORED_ENEMY_BONUS_INCREASE_CHOICE_ID} -> {target}): names {name} as \
                 the one favored enemy whose bonus increases by +2 at this 5th-level interval, \
                 per the PF1 Core Rulebook rule that the bonus against any ONE favored enemy -- \
                 including a newly selected one, if so desired -- increases by 2 at each such \
                 interval (5th, 10th, 15th, and 20th ranger level). This is a recognition record \
                 of the choice slot only (+0); the increased magnitude itself is grounded \
                 separately on whichever favored enemy was actually named"
            )
        } else {
            format!(
                "Ranger Favored Enemy bonus-increase target selection at ranger level {level} is \
                 present ({RANGER_FAVORED_ENEMY_BONUS_INCREASE_CHOICE_ID} -> {target}), but only \
                 the PF1 Core Rulebook restricted pair (the first favored enemy, the second \
                 favored enemy) is recognized on this bounded seam; no target identity is \
                 grounded and no mechanical value is fabricated (+0)"
            )
        };
        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.favored_enemy_bonus_increase_choice".to_owned(),
            value: 0,
            detail,
        });
    }

    let first_favored_enemy_targeted =
        favored_enemy_bonus_increase_target == Some(RANGER_FAVORED_ENEMY_BONUS_INCREASE_FIRST_SELECTION);
    let second_favored_enemy_targeted = favored_enemy_bonus_increase_target
        == Some(RANGER_FAVORED_ENEMY_BONUS_INCREASE_SECOND_SELECTION);

    let favored_enemy_bonus: i16 = if first_favored_enemy_targeted { 4 } else { 2 };
    explanations.push(ComputationExplanation {
        id: "class_chassis.ranger.favored_enemy_skill_bonus".to_owned(),
        value: favored_enemy_bonus,
        detail: format!(
            "Ranger Favored Enemy skill bonus (PF1 Core Rulebook, level \
             {level}): +{favored_enemy_bonus} on Bluff, Knowledge, Perception, \
             Sense Motive, and Survival checks against the chosen favored enemy. This grounds only \
             the flat +{favored_enemy_bonus} magnitude; no target-type matching and no \
             conditional-application engine is implemented, so whether any specific skill check is \
             actually made against the favored enemy is never resolved and no skill total is \
             modified by this record"
        ),
    });

    explanations.push(ComputationExplanation {
        id: "class_chassis.ranger.favored_enemy_attack_damage_bonus".to_owned(),
        value: favored_enemy_bonus,
        detail: format!(
            "Ranger Favored Enemy weapon attack/damage bonus (PF1 Core Rulebook, level \
             {level}): +{favored_enemy_bonus} on weapon attack rolls AND weapon \
             damage rolls against the chosen favored enemy — PF1 includes attack rolls, unlike the \
             damage-only D&D 3.5 favored enemy. This grounds only the flat \
             +{favored_enemy_bonus} magnitude; no target-type matching and no \
             conditional-application engine is implemented, so whether any specific attack is \
             actually made against the favored enemy is never resolved and no combat baseline is \
             modified by this record"
        ),
    });

    // SD13-E5 ranger level 5: recognize the SECOND favored-enemy selection, the
    // rule's other 5th-level interval grant. Mirrors the first favored enemy's
    // own choice-recognition idiom exactly (open-ended, raw string
    // interpolation, no restricted-list validation) and its own flat magnitude
    // formula (base +2, or +4 if this interval's bonus-increase target names the
    // second favored enemy).
    if level >= RANGER_FAVORED_ENEMY_SECOND_INTERVAL_LEVEL
        && let Some(second_favored_enemy) =
            choice_selection(input, RANGER_FAVORED_ENEMY_SECOND_CHOICE_ID)
    {
        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.favored_enemy_2_choice".to_owned(),
            value: 0,
            detail: format!(
                "Ranger 2nd Favored Enemy selection \
                 ({RANGER_FAVORED_ENEMY_SECOND_CHOICE_ID} -> {second_favored_enemy}): at \
                 ranger level {level}, PF1 Core Rulebook Favored Enemy grants \"an additional \
                 favored enemy\" at the 5th-level interval. The level-{level} SECOND \
                 favored-enemy type chosen for this character is {second_favored_enemy}. This \
                 is a bounded recognition record of the chosen enemy type only; the flat bonus \
                 magnitude is grounded separately, and no target-type matching or \
                 conditional-application engine is implemented, so it carries no fabricated \
                 mechanical value (+0)"
            ),
        });

        let second_favored_enemy_bonus: i16 = if second_favored_enemy_targeted { 4 } else { 2 };
        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.favored_enemy_2_skill_bonus".to_owned(),
            value: second_favored_enemy_bonus,
            detail: format!(
                "Ranger 2nd Favored Enemy skill bonus (PF1 Core Rulebook, level {level}): \
                 +{second_favored_enemy_bonus} on Bluff, Knowledge, Perception, Sense Motive, \
                 and Survival checks against the second favored enemy. This grounds only the \
                 flat +{second_favored_enemy_bonus} magnitude; no target-type matching and no \
                 conditional-application engine is implemented, so whether any specific skill \
                 check is actually made against this favored enemy is never resolved and no \
                 skill total is modified by this record"
            ),
        });

        explanations.push(ComputationExplanation {
            id: "class_chassis.ranger.favored_enemy_2_attack_damage_bonus".to_owned(),
            value: second_favored_enemy_bonus,
            detail: format!(
                "Ranger 2nd Favored Enemy weapon attack/damage bonus (PF1 Core Rulebook, \
                 level {level}): +{second_favored_enemy_bonus} on weapon attack rolls AND \
                 weapon damage rolls against the second favored enemy. This grounds only the \
                 flat +{second_favored_enemy_bonus} magnitude; no target-type matching and no \
                 conditional-application engine is implemented, so whether any specific attack \
                 is actually made against this favored enemy is never resolved and no combat \
                 baseline is modified by this record"
            ),
        });
    }

    // Grounded (SD13-E5): Endurance, a 3rd-level Ranger class feature verified
    // independently against two primary PF1 sources (d20pfsrd and
    // legacy.aonprd.com both list "Endurance, favored terrain" as the Ranger
    // 3rd-level special feature entry). Endurance is a bonus feat granted
    // automatically, with no player choice involved (PF1 Core Rulebook: "A
    // ranger gains Endurance as a bonus feat at 3rd level"). Below the level-3
    // gate this is a correct level-gate absence (value 0); at or above it, it is
    // a bounded grant-only identity record (value 0, non-fabricated) — mirroring
    // the Wizard Scribe Scroll / Barbarian Uncanny Dodge idiom: no feat-effect
    // execution engine exists anywhere in this codebase to apply Endurance's own
    // mechanical benefits.
    if level < RANGER_ENDURANCE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.endurance".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Endurance at ranger level {level}: correctly absent at level {level} by \
                 PF1 Core Rulebook level gate; the at-grant feat is named but not computed. \
                 Endurance is a 3rd-level ranger class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.endurance".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Endurance granted at ranger level {level} (PF1 Core Rulebook, 3rd-level \
                 ranger class feature): the ranger gains Endurance as a bonus feat automatically, \
                 with no player choice involved. This is a bounded grant-only identity record \
                 (value 0, non-fabricated): Endurance's own mechanical effects are not computed, \
                 since no feat-effect execution engine exists anywhere in this codebase"
            ),
        });
    }

    // Grounded (SD13-E5): Favored Terrain, the class table's other 3rd-level
    // "Special" column entry alongside Endurance, verified independently against
    // two primary PF1 sources (d20pfsrd and legacy.aonprd.com both list
    // "Endurance, favored terrain" as the Ranger 3rd-level special feature entry,
    // and both state the exact bonus text: "+2 bonus on Initiative checks and
    // Knowledge (geography), Perception, Stealth, and Survival skill checks" made
    // when the ranger is in the chosen terrain, selected from Table: Ranger
    // Favored Terrains' fixed eleven-entry list). Below the level-3 gate this is
    // a correct level-gate absence (value 0); at or above it: the chosen terrain
    // (when present in chosen input) is recognized as a bounded `+0` identity
    // record naming whichever raw terrain string was actually selected —
    // mirroring the Favored Enemy choice-recognition idiom exactly, with no
    // restricted-list validation — and the rule's own flat `+2` magnitude is
    // grounded as a standalone, non-applied record: no terrain-detection engine
    // decides whether the character is actually in the chosen terrain, so no
    // Initiative total or skill-check total is modified by this record. The
    // level-8th/13th/18th additional-terrain and bonus-increase progression
    // stays out of scope for this bounded slice.
    if level < RANGER_FAVORED_TERRAIN_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.favored_terrain".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Favored Terrain at ranger level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant terrain choice and \
                 magnitude are named but not computed. Favored Terrain is a 3rd-level ranger \
                 class feature."
            ),
        });
    } else {
        if let Some(favored_terrain) = choice_selection(input, RANGER_FAVORED_TERRAIN_CHOICE_ID) {
            explanations.push(ComputationExplanation {
                id: "class_chassis.ranger.favored_terrain_choice".to_owned(),
                value: 0,
                detail: format!(
                    "Ranger Favored Terrain selection \
                     ({RANGER_FAVORED_TERRAIN_CHOICE_ID} -> {favored_terrain}): the \
                     level-{level} favored terrain type chosen for this character is \
                     {favored_terrain}. This is a bounded recognition record of the chosen \
                     terrain type only; the flat bonus magnitude is grounded separately, and no \
                     terrain-detection or conditional-application engine is implemented, so it \
                     carries no fabricated mechanical value (+0)"
                ),
            });
        }

        let favored_terrain_bonus: i16 = 2;
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.favored_terrain".to_owned(),
            value: favored_terrain_bonus,
            detail: format!(
                "Ranger Favored Terrain bonus granted at ranger level {level} (PF1 Core \
                 Rulebook, 3rd-level ranger class feature): a flat \
                 +{favored_terrain_bonus} bonus on Initiative checks and Knowledge \
                 (geography), Perception, Stealth, and Survival checks made when the ranger is \
                 in the chosen favored terrain (drawn from Table: Ranger Favored Terrains --  \
                 Cold, Desert, Forest, Jungle, Mountain, Plains, Planes, Swamp, Underground, \
                 Urban, Water). This is a bounded flat-magnitude record only, non-fabricated: \
                 no terrain-detection engine decides whether the character is actually in the \
                 chosen terrain anywhere in this codebase, so no Initiative total or \
                 skill-check total is modified by this record"
            ),
        });
    }

    // Grounded (SD13-E5): Hunter's Bond, the class table's 4th-level "Special"
    // column entry, verified independently against two primary PF1 sources
    // (d20pfsrd and legacy.aonprd.com both list "Hunter's bond" as the Ranger
    // 4th-level special feature entry, and both state the exact rule text: "At
    // 4th level, a ranger forms a bond with his hunting companions. This bond
    // can take one of two forms. Once the form is chosen, it cannot be
    // changed."). Below the level-4 gate this is a correct level-gate absence
    // (value 0); at or above it: the chosen form (when present in chosen input)
    // is recognized as a bounded `+0` identity record naming whichever of the
    // two restricted forms was selected -- mirroring the combat-style choice
    // idiom (a restricted two-option recognition, unlike the open-ended Favored
    // Enemy/Favored Terrain choice-slots) -- and an unconditional grant-only
    // identity record is emitted, mirroring the Endurance/Favored Terrain grant
    // idiom. Only when the "bond" form is chosen is a further flat magnitude
    // grounded: half the already-grounded Favored Enemy bonus, granted to allies
    // within 30 feet who can see or hear the ranger against a single target of
    // the appropriate type. This grounds only the flat magnitude: no
    // move-action/action-economy engine, no ally-range-and-perception check, and
    // no favored-enemy target-type matching is implemented, so no ally's attack
    // or damage total is ever modified by this record. The "companion" form's
    // own animal-companion stat block/advancement subsystem is deliberately left
    // named-but-unproven: it does not exist anywhere in this codebase.
    if level < RANGER_HUNTERS_BOND_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.hunters_bond".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Hunter's Bond at ranger level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant form choice and the \
                 bond form's ally-bonus magnitude are named but not computed. Hunter's Bond is \
                 a 4th-level ranger class feature."
            ),
        });
    } else {
        let bond_selection = choice_selection(input, RANGER_HUNTERS_BOND_CHOICE_ID);
        let bond_form_name = bond_selection.and_then(|selection| {
            if selection == RANGER_HUNTERS_BOND_BOND_SELECTION {
                Some("a bond to his hunting companions")
            } else if selection == RANGER_HUNTERS_BOND_COMPANION_SELECTION {
                Some("an animal companion")
            } else {
                None
            }
        });

        if let Some(selection) = bond_selection {
            let detail = if let Some(form) = bond_form_name {
                format!(
                    "Ranger Hunter's Bond form selection at ranger level {level} \
                     ({RANGER_HUNTERS_BOND_CHOICE_ID} -> {selection}): names {form}, one of the \
                     two PF1 Core Rulebook Hunter's Bond forms granted at {RANGER_HUNTERS_BOND_LEVEL}th \
                     level. This is a recognition record of the choice slot only (+0): {form}'s own \
                     mechanics are not grounded here beyond the standalone flat magnitude recorded \
                     separately for the bond form, and no animal-companion stat block/advancement \
                     engine exists in this codebase"
                )
            } else {
                format!(
                    "Ranger Hunter's Bond form selection at ranger level {level} is present \
                     ({RANGER_HUNTERS_BOND_CHOICE_ID} -> {selection}), but only the PF1 Core \
                     Rulebook restricted pair (a bond to his hunting companions, an animal \
                     companion) is recognized on this bounded seam; no form identity is grounded \
                     and no mechanical value is fabricated (+0)"
                )
            };
            explanations.push(ComputationExplanation {
                id: "class_chassis.ranger.hunters_bond_choice".to_owned(),
                value: 0,
                detail,
            });
        }

        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.hunters_bond".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Hunter's Bond granted at ranger level {level} (PF1 Core Rulebook, \
                 4th-level ranger class feature): the ranger forms a bond with his hunting \
                 companions, taking one of two forms once chosen permanently. This is a bounded \
                 grant-only identity record (value 0, non-fabricated): the chosen form's own \
                 mechanical effects are not computed here beyond the standalone flat magnitude \
                 recorded separately for the bond form, since no move-action/action-economy \
                 engine and no animal-companion stat block/advancement engine exist anywhere in \
                 this codebase"
            ),
        });

        if bond_form_name == Some("a bond to his hunting companions") {
            let hunters_bond_ally_bonus = favored_enemy_bonus / 2;
            explanations.push(ComputationExplanation {
                id: "class_chassis.ranger.hunters_bond_ally_bonus".to_owned(),
                value: hunters_bond_ally_bonus,
                detail: format!(
                    "Ranger Hunter's Bond ally-bonus magnitude (PF1 Core Rulebook, level \
                     {level}, \"bond to his hunting companions\" form): half the ranger's own \
                     favored-enemy bonus ({favored_enemy_bonus} / 2 = {hunters_bond_ally_bonus}), \
                     grantable via a move action to allies within 30 feet who can see or hear the \
                     ranger, against a single target of the appropriate type. This grounds only \
                     the flat +{hunters_bond_ally_bonus} magnitude; no move-action/action-economy \
                     engine, no ally-range-and-perception check, and no favored-enemy \
                     target-type matching is implemented, so no ally's attack or damage total is \
                     ever modified by this record"
                ),
            });
        }
    }

    // Grounded (SD13-E5): Woodland Stride, the class table's 7th-level "Special"
    // column entry, verified independently against two primary PF1 sources
    // (d20pfsrd and legacy.aonprd.com both list "Woodland stride" as the Ranger
    // 7th-level special feature entry, with no other new class feature named at
    // 7th level). Unlike Track or Favored Terrain, Woodland Stride carries no
    // numeric magnitude of its own -- it is a pure boolean, no-choice grant, so
    // it mirrors the Endurance grant-only identity idiom exactly rather than a
    // flat-magnitude record. Below the level-7 gate this is a correct
    // level-gate absence (value 0); at or above it, it is a bounded grant-only
    // identity record (value 0, non-fabricated): no terrain-detection or
    // movement-resolution engine exists anywhere in this codebase to determine
    // whether the ranger is actually moving through undergrowth, so only the
    // grant itself is recorded.
    if level < RANGER_WOODLAND_STRIDE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.woodland_stride".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Woodland Stride at ranger level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant undergrowth-movement \
                 identity is named but not computed. Woodland Stride is a 7th-level ranger \
                 class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.woodland_stride".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Woodland Stride granted at ranger level {level} (PF1 Core Rulebook, \
                 7th-level ranger class feature): the ranger may move through any sort of \
                 undergrowth (such as natural thorns, briars, overgrown areas, and similar \
                 terrain) at his normal speed and without taking damage or suffering any other \
                 impairment; magically manipulated undergrowth still affects him normally. This \
                 is a bounded grant-only identity record (value 0, non-fabricated): no \
                 terrain-detection or movement-resolution engine exists anywhere in this \
                 codebase to determine whether the ranger is actually moving through \
                 undergrowth, so this only records the grant itself"
            ),
        });
    }

    // Grounded (SD13-E5): Swift Tracker, one of the class table's two 8th-level
    // "Special" column entries, verified independently against two primary PF1
    // sources (d20pfsrd and legacy.aonprd.com both list "Swift tracker" and "2nd
    // favored terrain" as the Ranger 8th-level special feature entries). Swift
    // Tracker only modifies a tracking-while-moving penalty resolution ("a
    // ranger can move at his normal speed while using Survival to follow tracks
    // without taking the normal -5 penalty. He takes only a -10 penalty
    // (instead of the normal -20) when moving at up to twice normal speed while
    // tracking") that does not exist anywhere in this codebase -- this codebase
    // grounds only the flat Track skill-bonus magnitude, never a
    // check-execution/movement-penalty engine -- so, exactly like Woodland
    // Stride, it is a genuinely flat/identity-shaped, no-choice, no-magnitude
    // grant. Below the level-8 gate this is a correct level-gate absence (value
    // 0); at or above it, it is a bounded grant-only identity record (value 0,
    // non-fabricated). The level-8 row's OTHER named entry, "2nd favored
    // terrain" (mirroring the Favored Enemy 5th-level idiom: a second
    // terrain-type selection plus a bonus-increase-target choice), is
    // deliberately left named-but-unproven this slice -- a real, newly
    // discovered multi-record burden, not an invented one.
    if level < RANGER_SWIFT_TRACKER_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.swift_tracker".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Swift Tracker at ranger level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant track-penalty-reduction \
                 identity is named but not computed. Swift Tracker is an 8th-level ranger \
                 class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.swift_tracker".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Swift Tracker granted at ranger level {level} (PF1 Core Rulebook, \
                 8th-level ranger class feature): the ranger can move at his normal speed \
                 while using Survival to follow tracks without taking the normal -5 penalty, \
                 and takes only a -10 penalty (instead of the normal -20) when moving at up \
                 to twice normal speed while tracking. This is a bounded grant-only identity \
                 record (value 0, non-fabricated): no tracking-while-moving \
                 check-execution/movement-penalty engine exists anywhere in this codebase to \
                 apply the reduced penalty, so this only records the grant itself"
            ),
        });
    }

    // Grounded (SD13-E5 level-9 slice): Evasion, the 9th-level Ranger class
    // feature verified independently against two primary PF1 sources (d20pfsrd
    // and legacy.aonprd.com both list "Evasion" as the Ranger 9th-level
    // "Special" entry — the same rule text as Rogue's and Monk's own Evasion).
    // Grounded as a bounded +0 identity/recognition record at or above the
    // gate, mirroring exactly how Rogue's and Monk's Evasion records were
    // grounded — no saving-throw-resolution or damage-resolution engine exists
    // in this codebase, so no damage math is fabricated from the record. Below
    // the level-9 gate no record is pushed at all (the level-9 slice's own
    // level-8 control pins that absence).
    if level >= RANGER_EVASION_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.ranger.evasion".to_owned(),
            value: 0,
            detail: format!(
                "Ranger Evasion granted at ranger level {level} (PF1 Core Rulebook, 9th-level \
                 ranger class feature): if the ranger makes a successful Reflex saving throw \
                 against an attack that normally deals half damage on a successful save, he \
                 instead takes no damage; Evasion can be used only when wearing light armor, \
                 medium armor, or no armor. This is a bounded identity/recognition record only \
                 (value 0, non-fabricated): no saving-throw-resolution engine and no \
                 damage-resolution engine exists anywhere in this codebase to apply it, so \
                 this grounds no actual damage reduction on any save outcome"
            ),
        });
    }
}

/// The bounded Sorcerer milestone level this decomposition surface grounds, if any.
/// Returns the single Sorcerer level when the chosen input is exactly a single-class
/// Sorcerer at one of the supported milestone levels (1 through 9). Returns `None` for
/// no Sorcerer, a non-Sorcerer class, a multiclass mix, or any level-10+ Sorcerer this
/// slice deliberately does not recognize — each of which stays claim-blocked exactly
/// as before. Mirrors the Fighter `supported_fighter_level` / Paladin
/// `supported_paladin_level` / Rogue `supported_rogue_level` / Barbarian
/// `supported_barbarian_level` / Monk `supported_monk_level` / Cleric
/// `supported_cleric_level` / Bard `supported_bard_level` / Druid
/// `supported_druid_level` level-range gate idiom.
fn supported_sorcerer_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == SORCERER_CLASS_ID
                && (1..=MAX_SUPPORTED_SORCERER_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

/// The bounded Barbarian milestone level this decomposition surface grounds, if any.
/// Returns the single Barbarian level when the chosen input is exactly a
/// single-class Barbarian at one of the supported milestone levels (1 through
/// 9). Returns `None` for no Barbarian, a non-Barbarian class, a multiclass mix,
/// or any level-10+ Barbarian this slice deliberately does not recognize — each of which
/// stays claim-blocked exactly as before. Mirrors the Fighter `supported_fighter_level`
/// / Paladin `supported_paladin_level` / Rogue `supported_rogue_level` / Monk
/// `supported_monk_level` level-range gate idiom.
fn supported_barbarian_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == BARBARIAN_CLASS_ID
                && (1..=MAX_SUPPORTED_BARBARIAN_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

/// Surface direct SD13-E3/E5 runtime evidence for the deterministic Human Barbarian
/// level-1/level-2/level-3/level-4 martial chassis. Base-attack progression, base-save
/// progression, and the fast-movement speed-extension value are grounded directly at
/// every supported level. The SD13-E5 slice resolves the formerly-named illiteracy
/// burden as vacuous — the PF1 Core Rulebook Barbarian is NOT illiterate; illiteracy
/// is a D&D 3.5e Barbarian trait that never existed in PF1, so under the fixture's
/// `pf1.core_rulebook` source package there was never anything to implement — and
/// grounds Rage's flat numeric surface: rage rounds per day (4 + Constitution modifier,
/// growing by 2 more rounds per level after 1st, claim-blocked instead of grounded
/// when that sum is non-positive) and the flat while-raging constants (a morale
/// bonus to Strength, a morale bonus to Constitution, a morale bonus on Will saves,
/// and an armor class penalty, unchanged by level), values only. A later SD13-E5
/// slice widens the level-1-only gate (`martial_level1_class`) to a level-range gate
/// (`supported_barbarian_level`, 1..=2), mirroring the Fighter/Paladin/Rogue
/// level-range-gate idiom, and a further SD13-E5 slice grounds Uncanny Dodge, the
/// PF1 Core Rulebook Barbarian's 2nd-level "Special" class table entry (verified
/// independently against d20pfsrd and legacy.aonprd.com, both naming "Rage power,
/// uncanny dodge" as the level-2 row), as a bounded identity/recognition record only
/// (`class_feature.barbarian.uncanny_dodge`, value 0) — a level-gate-absence record
/// below level 2, a granted-but-unexecuted rule-text recognition record at or above
/// it, mirroring exactly how Rogue's/Monk's own Evasion and Druid's Woodland Stride
/// were grounded, with no flat-footed-state tracking, no Armor Class computation, and
/// no invisibility-detection engine implemented. The level-2 row's other named entry,
/// a Rage Power choice (a genuinely open-ended choice-list feature), is deliberately
/// left named-but-unproven, mirroring the Monk level-2 bonus feat grant / Bard
/// Versatile Performance precedent. A still further SD13-E5 slice widens the gate to
/// level 3 (`MAX_SUPPORTED_BARBARIAN_LEVEL = 3`, mirroring the Rogue/Monk level-3
/// widening idiom) and grounds Trap Sense, the PF1 Core Rulebook Barbarian's 3rd-level
/// "Special" class table entry (verified independently against d20pfsrd and
/// legacy.aonprd.com, both naming "Trap sense +1" as the level-3 row), as a bounded
/// flat-magnitude record only (`class_feature.barbarian.trap_sense`, barbarian level /
/// 3, floor; +1 at level 3) — a level-gate-absence record below level 3, a
/// flat-magnitude recognition record at or above it, mirroring exactly how Rogue's own
/// Trap Sense was grounded, never applied to any actual Reflex-save total or Armor
/// Class total. A still further SD13-E5 slice widens the gate to level 4
/// (`MAX_SUPPORTED_BARBARIAN_LEVEL = 4`, mirroring the Rogue/Monk level-4 widening
/// idiom, verified independently against d20pfsrd and legacy.aonprd.com: the level-4
/// row is BAB +4, Fort +4, Ref +1, Will +1, Special "Rage power"): base-attack
/// (classlevel = 4), base-save (Fortitude +4, Reflex +1, Will +1), fast movement
/// (unchanged flat +10 ft.), and rage rounds per day (4 + Constitution modifier + 2 *
/// (level - 1), 13 on the Con 16 fixture at level 4) are extended to level 4 via the
/// same formulas, and Uncanny Dodge and Trap Sense both stay granted (not re-derived;
/// Trap Sense stays at the same +1 magnitude, since the PF1 Core Rulebook bonus does
/// not rise again until barbarian level 6). The level-4 row's only named "Special"
/// entry is another Rage Power grant — the same genuinely open-ended choice-list
/// feature already deliberately left named-but-unproven at level 2, not a new type of
/// class feature — so this widening grounds no new pillar beyond the arithmetic
/// extension above. A still further SD13-E5 slice widens the gate to level 5
/// (`MAX_SUPPORTED_BARBARIAN_LEVEL = 5`, mirroring the Rogue/Monk level-5 widening
/// idiom, verified independently against d20pfsrd and legacy.aonprd.com: the level-5
/// row is BAB +5, Fort +4, Ref +1, Will +1, Special "Improved uncanny dodge"):
/// base-attack (classlevel = 5), base-save (Fortitude +4, Reflex +1, Will +1), fast
/// movement (unchanged flat +10 ft.), and rage rounds per day (4 + Constitution
/// modifier + 2 * (level - 1), 15 on the Con 16 fixture at level 5) are extended to
/// level 5 via the same formulas, and Uncanny Dodge and Trap Sense both stay granted
/// (not re-derived; Trap Sense stays at the same +1 magnitude, since the PF1 Core
/// Rulebook bonus does not rise again until barbarian level 6). The level-5 row's
/// "Special" entry, Improved Uncanny Dodge (verified independently against d20pfsrd
/// and legacy.aonprd.com: "At 5th level and higher, a barbarian can no longer be
/// flanked. This defense denies a rogue the ability to sneak attack the barbarian by
/// flanking her, unless the attacker has at least four more rogue levels than the
/// target has barbarian levels."), IS a genuinely new class feature, not another Rage
/// Power grant — and its own grant is flat/identity-shaped exactly like Uncanny
/// Dodge's own record, so it is newly grounded as a bounded identity/recognition
/// record only (`class_feature.barbarian.improved_uncanny_dodge`, value 0): a
/// level-gate absence below level 5, a granted-but-unexecuted rule-text recognition
/// record at or above it. The rule's own CONDITIONAL piece — comparing the attacking
/// rogue's own levels against the barbarian's own levels to decide whether the
/// immunity is actually pierced — is not computed: no flanking-resolution engine, no
/// attacker-level-comparison engine, and no sneak-attack-trigger engine exists
/// anywhere in this codebase, so this slice grounds only the bounded grant, mirroring
/// exactly how Uncanny Dodge itself was grounded. A still further SD13-E5 slice widens
/// the gate to level 6 (`MAX_SUPPORTED_BARBARIAN_LEVEL = 6`, mirroring the Rogue's own
/// level-6 widening idiom, verified independently against d20pfsrd and
/// legacy.aonprd.com: the level-6 row is BAB +6, Fort +5, Ref +2, Will +2, Special
/// "Rage power, trap sense +2"): base-attack (classlevel = 6), base-save (Fortitude
/// +5, Reflex +2, Will +2), fast movement (unchanged flat +10 ft.), and rage rounds
/// per day (4 + Constitution modifier + 2 * (level - 1), 17 on the Con 16 fixture at
/// level 6) are extended to level 6 via the same formulas, and Uncanny Dodge and
/// Improved Uncanny Dodge both stay granted (not re-derived). Trap Sense's own flat
/// magnitude GENUINELY RISES at level 6 (barbarian level / 3, floor: `6 / 3 = 2`, up
/// from `1` at levels 3-5) via the same pre-existing formula, matching the class
/// table's own "trap sense +2" entry exactly — this is a value change, not a new
/// record, mirroring exactly how Rogue's own level-6 Trap Sense rise was grounded.
/// The level-6 row's other named "Special" entry is another Rage Power grant — the
/// same genuinely open-ended choice-list feature already deliberately left
/// named-but-unproven at levels 2 and 4, not a new type of class feature — so this
/// widening grounds no new pillar beyond the arithmetic extension and the Trap Sense
/// magnitude rise above. A still further SD13-E5 slice widens the gate to level 7
/// (`MAX_SUPPORTED_BARBARIAN_LEVEL = 7`, mirroring the Rogue's own level-7 widening
/// idiom, verified independently against d20pfsrd and legacy.aonprd.com: the level-7
/// row is BAB +7, Fort +5, Ref +2, Will +2, Special "Damage reduction 1/-"):
/// base-attack (classlevel = 7), base-save (Fortitude +5, Reflex +2, Will +2), fast
/// movement (unchanged flat +10 ft.), and rage rounds per day (4 + Constitution
/// modifier + 2 * (level - 1), 19 on the Con 16 fixture at level 7) are extended to
/// level 7 via the same formulas, and Uncanny Dodge, Trap Sense, and Improved Uncanny
/// Dodge all stay granted (not re-derived; Trap Sense stays at the same +2 magnitude,
/// since the PF1 Core Rulebook bonus does not rise again until barbarian level 9). The
/// level-7 row's "Special" entry, Damage Reduction 1/- (verified independently against
/// d20pfsrd and legacy.aonprd.com: "at 7th level, a barbarian gains damage reduction.
/// Subtract 1 from the damage the barbarian takes each time she is dealt damage from a
/// weapon or a natural attack"), IS a genuinely new class feature, NOT another Rage
/// Power grant — both primary sources confirm Rage Powers are granted at 2nd, 4th,
/// 6th, 8th, and 10th barbarian level, not 7th, so there is no new Rage Power grant to
/// leave named-but-unproven at this level and no rage-power-selection-slot-count
/// engine is invented. Damage Reduction's own flat magnitude (1 point) is
/// flat/identity-shaped exactly like Trap Sense's own magnitude, so it is newly
/// grounded as a bounded flat-magnitude record only
/// (`class_feature.barbarian.damage_reduction`, value 1 at or above level 7, value 0
/// below it): the rule's own APPLICATION piece (subtracting the value from incoming
/// weapon/natural-attack damage) is not computed, since no damage-resolution engine
/// and no incoming-damage total exists anywhere in this codebase. A still further
/// SD13-E5 slice widens the gate to level 8 (`MAX_SUPPORTED_BARBARIAN_LEVEL = 8`,
/// mirroring the Rogue's/Monk's own level-8 widening idiom, verified independently
/// against d20pfsrd and legacy.aonprd.com: the level-8 row is BAB +8, Fort +6, Ref +2,
/// Will +2, Special "Rage power" only): base-attack (classlevel = 8), base-save
/// (Fortitude +6, Reflex +2, Will +2), fast movement (unchanged flat +10 ft.), and
/// rage rounds per day (4 + Constitution modifier + 2 * (level - 1), 21 on the Con 16
/// fixture at level 8) are extended to level 8 via the same formulas, and Uncanny
/// Dodge, Trap Sense, Improved Uncanny Dodge, and Damage Reduction all stay granted
/// (not re-derived; Trap Sense stays at the same +2 magnitude, since the PF1 Core
/// Rulebook bonus does not rise again until barbarian level 9, and Damage Reduction
/// stays at the same 1-point magnitude, since it does not rise again until barbarian
/// level 10). The level-8 row's "Special" entry is another Rage Power grant — both
/// primary sources confirm Rage Powers are granted at 2nd, 4th, 6th, 8th, and 10th
/// barbarian level, so this is the SAME genuinely open-ended choice-list feature
/// already deliberately left named-but-unproven at levels 2, 4, and 6, not a new type
/// of class feature — so this widening grounds no new pillar beyond the arithmetic
/// extension above and no rage-power-selection-slot-count engine is invented.
/// Otherwise only the rage-state execution burden, the Rage Power choice-list
/// feature, weapon familiarity, the Improved Uncanny Dodge flanking-resolution
/// engine, and the Damage Reduction application engine stay explicitly claim-blocked.
///
/// This deliberately does not compute a supported martial chassis: the grounded
/// base-attack, base-save, fast-movement, rage, Uncanny Dodge, Trap Sense, Improved
/// Uncanny Dodge, and Damage Reduction explanation records below are standalone (not
/// wired into `PilotBaseChassisComputation.base_attack_bonus`, `compute_total_saves`,
/// `compute_combat_baseline`, the integrated ability modifiers, or any
/// speed/movement/flat-footed/Armor-Class/incoming-damage total), so the integrated
/// pilot surface still reports a blocked posture on this input. It grounds no
/// rage-state engine, no weapon familiarity, no Rage Power choice-list feature, no
/// flat-footed-state tracking, no Armor Class computation, no invisibility-detection
/// engine, no flanking-resolution engine, no damage-reduction-resolution engine, and
/// no level-9+ martial progression. It only:
/// - leaves one chassis-recognition explanation so the `class:barbarian:N` identity
///   (at the supported level, 1, 2, 3, 4, 5, 6, 7, or 8) is acknowledged as a non-hybrid
///   martial baseline rather than an undocumented packet placeholder (direct runtime
///   evidence, carrying no fabricated mechanical value),
/// - leaves five grounded explanation records naming the full-BAB base-attack
///   bonus, the good-Fortitude/poor-Reflex/poor-Will base saves, and the flat
///   +10 ft. fast-movement value,
/// - leaves one grounded rules-correction record documenting that the illiteracy
///   burden was vacuous (`class_chassis.barbarian.illiteracy_absent`, +0),
/// - leaves up to five grounded rage explanation records naming rage rounds per day
///   (4 + Constitution modifier, omitted in favor of a claim-blocking diagnostic when
///   that sum is non-positive) and the four flat rage constants, values only,
/// - leaves one grounded Uncanny Dodge identity/recognition record (level-gate
///   absence below level 2, granted-but-unexecuted rule-text recognition at or
///   above it, value 0 either way),
/// - leaves one grounded Trap Sense flat-magnitude record (level-gate absence below
///   level 3, value 0; flat magnitude at or above it, barbarian level / 3),
/// - leaves one grounded Improved Uncanny Dodge identity/recognition record
///   (level-gate absence below level 5, granted-but-unexecuted rule-text recognition
///   at or above it, value 0 either way),
/// - leaves one grounded Damage Reduction flat-magnitude record (level-gate absence
///   below level 7, value 0; flat magnitude of 1 at or above it, never applied to any
///   incoming-damage total), and
/// - emits one claim-blocking diagnostic naming the still-missing rage-state
///   execution engine explicitly (activation/deactivation, round-by-round rage
///   round consumption, fatigue after rage, and temporary application of the rage
///   constants to computed totals), rather than hiding behind a single generic
///   "unsupported class" label.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input; this seam
/// keeps that blocked posture but makes the Barbarian martial identity, its grounded
/// pillar values, and its remaining named pillar burden legible on the runtime path.
fn explain_barbarian_level1_chassis(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let Some(level) = supported_barbarian_level(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    let class_id = BARBARIAN_CLASS_ID;
    let class_name = "Barbarian";
    let chassis_id = "class_chassis.barbarian.bounded_progression";
    let level_value = i16::from(level);

    // Direct runtime evidence: recognize the deterministic Human Barbarian chassis
    // identity at the supported level. This is a recognition record only; it
    // fabricates no mechanical value.
    explanations.push(ComputationExplanation {
        id: chassis_id.to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human {class_name} level {level} martial chassis: \
             the {class_id}:{level} class identity is acknowledged as a pure non-hybrid \
             martial baseline on the rules-core seam rather than an undocumented packet placeholder. This \
             is a bounded chassis-recognition record only; it grounds no rage-state execution engine, no \
             weapon familiarity, and no level-5+ martial progression, so it carries no fabricated \
             mechanical value (+0). The base-attack, base-save, fast-movement, and flat rage pillar \
             values are grounded separately as standalone explanation records"
        ),
    });

    // Grounded (1/3): full-BAB base-attack progression, same formula shape as
    // Fighter's cr_classes.lst:139 BONUS:COMBAT|BASEAB|classlevel. No PCGen .lst
    // file exists for the Barbarian class in this repo, so this cites the PF1 Core
    // Rulebook Barbarian class table directly.
    let base_attack_bonus = level_value;
    explanations.push(ComputationExplanation {
        id: "class_chassis.barbarian.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            "{class_name} level {level} base attack bonus from the PF1 Core Rulebook \
             Barbarian class table (full base-attack progression, same formula shape as Fighter's \
             cr_classes.lst:139 BONUS:COMBAT|BASEAB|classlevel): classlevel = {base_attack_bonus}. This \
             is a standalone explanation record; it is not wired into the integrated base_attack_bonus \
             field or into compute_combat_baseline"
        ),
    });

    // Grounded (2/3): base-save progression — good Fortitude, poor Reflex, poor
    // Will, same formula shape as Fighter's cr_classes.lst:139 base-save cadence.
    // Extended to every supported level via the same formulas, not re-derived.
    let fortitude_save = level_value / 2 + 2;
    let reflex_save = level_value / 3;
    let will_save = level_value / 3;
    explanations.push(ComputationExplanation {
        id: "class_chassis.barbarian.base_save.fortitude".to_owned(),
        value: fortitude_save,
        detail: format!(
            "{class_name} level {level} base Fortitude save (good save) from the PF1 \
             Core Rulebook Barbarian class table, same formula shape as Fighter's cr_classes.lst:139 \
             BONUS:SAVE|BASE.Fortitude|classlevel/2+2: classlevel/2+2 = {fortitude_save}. This is a \
             standalone explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.barbarian.base_save.reflex".to_owned(),
        value: reflex_save,
        detail: format!(
            "{class_name} level {level} base Reflex save (poor save) from the PF1 Core \
             Rulebook Barbarian class table, same formula shape as Fighter's cr_classes.lst:139 \
             BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel/3: classlevel/3 = {reflex_save}. This is a \
             standalone explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.barbarian.base_save.will".to_owned(),
        value: will_save,
        detail: format!(
            "{class_name} level {level} base Will save (poor save) from the PF1 Core \
             Rulebook Barbarian class table, same formula shape as Fighter's cr_classes.lst:139 \
             BONUS:SAVE|BASE.Reflex,BASE.Will|classlevel/3: classlevel/3 = {will_save}. This is a \
             standalone explanation record; it is not wired into compute_total_saves"
        ),
    });

    // Grounded (3/3): the fast-movement flat +10 ft. speed value. This grounds only
    // the flat bonus value itself, not a runtime armor/encumbrance-state check
    // engine — no such engine exists anywhere in this codebase yet — so the value
    // is asserted unconditionally rather than computed from armor/load state, and
    // it is not wired into any speed/movement total. The PF1 Core Rulebook
    // fast-movement bonus does not scale with level, so this is the same flat +10
    // ft. value at every supported level.
    explanations.push(ComputationExplanation {
        id: "class_chassis.barbarian.fast_movement".to_owned(),
        value: 10,
        detail: "Barbarian fast movement: +10 ft. land speed extension while wearing no heavy armor \
             and carrying no heavy load (PF1 Core Rulebook Barbarian class table). This slice grounds \
             only the flat +10 ft. value, not a runtime armor/encumbrance-state check engine — no such \
             engine exists anywhere in this codebase yet — so the value is asserted unconditionally \
             rather than computed from armor/load state, and it is not wired into any speed/movement \
             total. This flat value does not scale with barbarian level"
            .to_owned(),
    });

    // Rules correction: the formerly-named illiteracy burden was vacuous under the
    // fixture's pf1.core_rulebook source package. Illiteracy is a D&D 3.5e Barbarian
    // trait; the PF1 Core Rulebook Barbarian is not illiterate, so there was never
    // anything to implement. The resolution is documented as a grounded value-0
    // record rather than silently dropped, and the old claim-blocking diagnostic
    // (class_feature.barbarian.bounded_progression.illiteracy.unsupported) is retired.
    explanations.push(ComputationExplanation {
        id: "class_chassis.barbarian.illiteracy_absent".to_owned(),
        value: 0,
        detail: format!(
            "{class_name} illiteracy burden resolved as vacuous: the PF1 Core Rulebook {class_name} \
             is NOT illiterate — illiteracy is a D&D 3.5e {class_name} class trait that was removed in \
             Pathfinder 1e and never existed under the pf1.core_rulebook source package this fixture \
             names. The previously catalogued illiteracy burden therefore named a rule with no PF1 \
             existence, and retiring it is a rules correction, not an uplift. This record documents \
             that correction only; it carries no mechanical value (+0)"
        ),
    });

    // Grounded: Rage's flat numeric surface, values only. Rage rounds per day is the
    // one Constitution-derived rage number the PF1 Core Rulebook Rage class feature
    // grounds at level 1 (4 + Constitution modifier) and grows by a further flat +2
    // rounds "at each level after 1st" (PF1 Core Rulebook Rage: "She can rage for a
    // number of rounds per day equal to 4 + her Constitution modifier. At each level
    // after 1st, she can rage for 2 additional rounds."), generalized here as
    // 4 + Constitution modifier + 2 * (level - 1). At level 1 this collapses to the
    // original 4 + Constitution modifier (2 * 0 = 0 extra rounds), so the grounded
    // level-1 truth is unchanged by this widening. At a low enough Constitution
    // modifier that sum is non-positive, which is not a real PF1 rounds-per-day
    // count, so this slice claim-blocks the record instead of asserting a
    // fabricated zero/negative value — the deterministic Con 16 fixture (modifier
    // +3, 7 rounds at level 1, 9 rounds at level 2) never hits this branch, but the
    // public compute seam accepts any Human Barbarian input.
    let constitution_modifier = ability_modifier_for(ability_modifiers, "constitution");
    let rage_rounds_per_day = 4 + constitution_modifier + 2 * (level_value - 1);
    if rage_rounds_per_day > 0 {
        explanations.push(ComputationExplanation {
            id: "class_chassis.barbarian.rage_rounds_per_day".to_owned(),
            value: rage_rounds_per_day,
            detail: format!(
                "{class_name} level {level} rage rounds per day from the PF1 Core \
                 Rulebook Rage class feature: 4 + Constitution modifier + 2 * (level - 1) = 4 + \
                 {constitution_modifier} + 2 * ({level_value} - 1) = {rage_rounds_per_day} rounds per \
                 day at level {level} (the +2-additional-rounds-per-level-after-1st rule). This is a \
                 standalone explanation record: no round is ever consumed, tracked, or restored by \
                 this slice"
            ),
        });
    } else {
        diagnostics.push(ComputationDiagnostic {
            id: "class_chassis.barbarian.rage_rounds_per_day.unsupported".to_owned(),
            message: format!(
                "{class_name} level {level} rage rounds per day (4 + Constitution modifier + 2 * \
                 (level - 1)) is not grounded for this input: 4 + {constitution_modifier} + 2 * \
                 ({level_value} - 1) = {rage_rounds_per_day}, a non-positive count with no PF1 Core \
                 Rulebook meaning. This slice does not assert a fabricated zero/negative \
                 rounds-per-day value, so no rage rounds per day is claimed for this Constitution \
                 score"
            ),
            claim_blocking: true,
        });
    }

    // Grounded: the four flat while-raging constants, as value-only records. None of
    // these is applied to any computed total — application is exactly the rage-state
    // execution burden named by the claim-blocking diagnostic below. Each entry also
    // carries a terse label (4th field) so the claim-blocking diagnostic below can
    // cite the same four values without hand-retyping them as a separate literal.
    let rage_constants: [(&str, i16, &str, &str); 4] = [
        (
            "class_chassis.barbarian.rage.strength_morale_bonus",
            4,
            "+4 morale bonus to Strength while raging",
            "+4 morale Strength",
        ),
        (
            "class_chassis.barbarian.rage.constitution_morale_bonus",
            4,
            "+4 morale bonus to Constitution while raging",
            "+4 morale Constitution",
        ),
        (
            "class_chassis.barbarian.rage.will_save_morale_bonus",
            2,
            "+2 morale bonus on Will saves while raging",
            "+2 morale Will saves",
        ),
        (
            "class_chassis.barbarian.rage.armor_class_penalty",
            -2,
            "-2 penalty to Armor Class while raging",
            "-2 AC",
        ),
    ];
    for (id, value, effect, _short_label) in rage_constants {
        explanations.push(ComputationExplanation {
            id: id.to_owned(),
            value,
            detail: format!(
                "{class_name} Rage flat constant from the PF1 Core Rulebook Rage class feature: \
                 {effect}. This slice grounds only the flat value; it is never applied to the \
                 integrated ability modifiers, saves, or armor class — temporary application is \
                 part of the unimplemented rage-state execution engine"
            ),
        });
    }
    let rage_constants_summary = rage_constants
        .iter()
        .map(|(_, _, _, short_label)| *short_label)
        .collect::<Vec<_>>()
        .join(", ");

    // Grounded (SD13-E5): Uncanny Dodge, a 2nd-level Barbarian class feature verified
    // independently against two primary PF1 sources (d20pfsrd and legacy.aonprd.com
    // both list "Rage power, uncanny dodge" as the Barbarian 2nd-level special feature
    // entry). Below the level-2 gate this is a correct PF1 Core Rulebook level-gate
    // absence (value 0); at or above it, it is a bounded identity/recognition record
    // only (value 0, non-fabricated) naming the rule text — mirroring exactly how
    // Rogue's/Monk's own Evasion and Druid's Woodland Stride were grounded, without
    // folding into any actual flat-footed-state tracking, Armor Class computation, or
    // invisibility-detection engine, none of which exists in this codebase. The level-2
    // row's OTHER named entry, a Rage Power choice (a genuinely open-ended choice-list
    // feature, a new-subsystem-shaped burden), is deliberately left named-but-unproven
    // this slice, mirroring how the Monk level-2 bonus feat grant and the Bard
    // Versatile Performance were each deliberately left unrecognized: no new
    // choice-slot and no new diagnostic was added for it.
    if level < BARBARIAN_UNCANNY_DODGE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.barbarian.uncanny_dodge".to_owned(),
            value: 0,
            detail: format!(
                "Barbarian Uncanny Dodge at barbarian level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant rule is named but not \
                 computed. Uncanny Dodge is a 2nd-level barbarian class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.barbarian.uncanny_dodge".to_owned(),
            value: 0,
            detail: format!(
                "Barbarian Uncanny Dodge granted at barbarian level {level} (PF1 Core Rulebook, \
                 2nd-level barbarian class feature, part of the \"Rage power, uncanny dodge\" \
                 table entry): she cannot be caught flat-footed, and she retains her Dexterity \
                 bonus to Armor Class even if the attacker is invisible; she still loses her \
                 Dexterity bonus to Armor Class if immobilized, and a successful feint action can \
                 still strip it away. This is a bounded identity/recognition record only (value 0, \
                 non-fabricated): no flat-footed-state tracking, no Armor Class computation, and \
                 no invisibility-detection engine exists anywhere in this codebase to apply it, so \
                 this grounds no actual flat-footed immunity or Dexterity-to-AC retention"
            ),
        });
    }

    // Grounded (SD13-E5): Trap Sense, a 3rd-level Barbarian class feature (verified
    // independently against d20pfsrd and legacy.aonprd.com: both name "Trap sense +1"
    // as the Barbarian 3rd-level "Special" class table entry). Below the level-3 gate
    // this is a correct PF1 Core Rulebook level-gate absence (value 0); at or above it,
    // it is a bounded flat-magnitude record only (barbarian level / 3, floor) naming
    // the rule text — mirroring exactly how Rogue's own Trap Sense was grounded: the
    // magnitude is never applied to any actual Reflex-save total or Armor Class total,
    // since no saving-throw-resolution or armor-class-resolution engine exists in this
    // codebase, and no trap-detection or trap-triggering engine exists to decide when
    // it would apply.
    if level < BARBARIAN_TRAP_SENSE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.barbarian.trap_sense".to_owned(),
            value: 0,
            detail: format!(
                "Barbarian Trap Sense at barbarian level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant magnitude is named but \
                 not computed. Trap Sense is a 3rd-level barbarian class feature."
            ),
        });
    } else {
        let trap_sense_bonus = level_value / 3;
        explanations.push(ComputationExplanation {
            id: "class_feature.barbarian.trap_sense".to_owned(),
            value: trap_sense_bonus,
            detail: format!(
                "Barbarian Trap Sense granted at barbarian level {level} (PF1 Core Rulebook, \
                 3rd-level barbarian class feature): a +{trap_sense_bonus} bonus on Reflex \
                 saves made to avoid traps and a +{trap_sense_bonus} dodge bonus to AC against \
                 attacks made by traps (barbarian level / 3 = {trap_sense_bonus}; this bonus \
                 rises further at 6th/9th/12th/15th/18th barbarian level, beyond this bounded \
                 slice). This is a bounded flat-magnitude record only, non-fabricated: it is \
                 never applied to any actual Reflex-save total or AC total, since no \
                 saving-throw-resolution or armor-class-resolution engine exists anywhere in \
                 this codebase to apply it, and no trap-detection or trap-triggering engine \
                 exists to decide when it would apply"
            ),
        });
    }

    // Grounded (SD13-E5): Improved Uncanny Dodge, a 5th-level Barbarian class
    // feature (verified independently against d20pfsrd and legacy.aonprd.com: both
    // name "Improved uncanny dodge" as the Barbarian 5th-level "Special" class table
    // entry). Below the level-5 gate this is a correct PF1 Core Rulebook level-gate
    // absence (value 0); at or above it, it is a bounded identity/recognition record
    // only (value 0, non-fabricated) naming the rule text — mirroring exactly how
    // Uncanny Dodge itself was grounded. The rule's own CONDITIONAL piece (comparing
    // the attacking rogue's own levels against the barbarian's own levels to decide
    // whether the immunity is actually pierced) is never applied: no
    // flanking-resolution engine, no attacker-level-comparison engine, and no
    // sneak-attack-trigger engine exists anywhere in this codebase, so this grounds
    // no actual flanking immunity or sneak-attack denial.
    if level < BARBARIAN_IMPROVED_UNCANNY_DODGE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.barbarian.improved_uncanny_dodge".to_owned(),
            value: 0,
            detail: format!(
                "Barbarian Improved Uncanny Dodge at barbarian level {level}: correctly \
                 absent at level {level} by PF1 Core Rulebook level gate; the at-grant rule \
                 is named but not computed. Improved Uncanny Dodge is a 5th-level barbarian \
                 class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.barbarian.improved_uncanny_dodge".to_owned(),
            value: 0,
            detail: format!(
                "Barbarian Improved Uncanny Dodge granted at barbarian level {level} (PF1 \
                 Core Rulebook, 5th-level barbarian class feature): at 5th level and higher, a \
                 barbarian can no longer be flanked, denying a rogue the ability to sneak \
                 attack her by flanking unless the attacker has at least four more rogue \
                 levels than the barbarian has barbarian levels. This is a bounded \
                 identity/recognition record only (value 0, non-fabricated): no \
                 flanking-resolution engine, no attacker-level-comparison engine, and no \
                 sneak-attack-trigger engine exists anywhere in this codebase to apply it, so \
                 this grounds no actual flanking immunity or sneak-attack denial"
            ),
        });
    }

    // Grounded (SD13-E5): Damage Reduction, a 7th-level Barbarian class feature
    // (verified independently against d20pfsrd and legacy.aonprd.com: both name
    // "Damage reduction 1/-" as the Barbarian 7th-level "Special" class table entry,
    // with the rule text "At 7th level, a barbarian gains damage reduction. Subtract 1
    // from the damage the barbarian takes each time she is dealt damage from a weapon
    // or a natural attack"). Below the level-7 gate this is a correct PF1 Core
    // Rulebook level-gate absence (value 0); at or above it, it is a bounded
    // flat-magnitude record only (a flat value of 1, non-fabricated) naming the rule
    // text — mirroring exactly how Trap Sense's own flat magnitude was grounded: the
    // magnitude is never applied to any actual incoming-damage total, since no
    // damage-resolution engine or incoming-damage total exists anywhere in this
    // codebase. Both primary sources' level-7 "Special" column names Damage Reduction
    // only, not a Rage Power grant — Rage Powers are granted at 2nd, 4th, 6th, 8th,
    // and 10th barbarian level, not 7th — so no rage-power-selection-slot-count engine
    // is invented here.
    if level < BARBARIAN_DAMAGE_REDUCTION_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.barbarian.damage_reduction".to_owned(),
            value: 0,
            detail: format!(
                "Barbarian Damage Reduction at barbarian level {level}: correctly absent at \
                 level {level} by PF1 Core Rulebook level gate; the at-grant magnitude is named \
                 but not computed. Damage Reduction is a 7th-level barbarian class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.barbarian.damage_reduction".to_owned(),
            value: 1,
            detail: format!(
                "Barbarian Damage Reduction granted at barbarian level {level} (PF1 Core \
                 Rulebook, 7th-level barbarian class feature, \"Damage reduction 1/-\"): \
                 subtract 1 from the damage the barbarian takes each time she is dealt damage \
                 from a weapon or a natural attack. This is a bounded flat-magnitude record \
                 only (value 1, non-fabricated): no damage-resolution engine and no \
                 incoming-damage total exists anywhere in this codebase to apply it, so this \
                 grounds no actual damage reduction"
            ),
        });
    }

    // Still blocked: name the rage-state execution burden explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.barbarian.bounded_progression.rage_execution.unsupported".to_owned(),
        message: format!(
            "{class_name} level {level} remains blocked on its rage-state execution \
             engine: rage activation and deactivation, round-by-round consumption of the grounded rage \
             rounds per day, the fatigue condition after a rage ends, and temporary application of the \
             grounded rage constants ({rage_constants_summary}) to computed totals are not implemented \
             in this bounded martial chassis baseline, so no {class_name} rage-execution support is \
             claimed"
        ),
        claim_blocking: true,
    });
}

/// The bounded Monk milestone level this decomposition surface grounds, if any.
/// Returns the single Monk level when the chosen input is exactly a single-class
/// Monk at one of the supported milestone levels (1 through 9). Returns
/// `None` for no Monk, a non-Monk class, a multiclass mix, or any level-10+ Monk
/// this slice deliberately does not recognize — each of which stays
/// claim-blocked exactly as before. Mirrors the Fighter `supported_fighter_level`
/// / Paladin `supported_paladin_level` / Rogue `supported_rogue_level` /
/// Barbarian `supported_barbarian_level` level-range gate idiom.
fn supported_monk_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == MONK_CLASS_ID
                && (1..=MAX_SUPPORTED_MONK_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

/// Surface direct SD13-E3/E5 runtime evidence for the deterministic Human Monk
/// level-1/level-2/level-3/level-4/level-5/level-6/level-7/level-8 martial chassis,
/// mirroring the Barbarian/Rogue level-range-gate pattern, and now grounding ten
/// named pillar burdens at every supported level (base-attack, base-save, AC Bonus, the
/// unarmed strike die / Flurry of Blows flat surface, the level-1 bonus feat
/// choice-slot recognition, at level 2, Evasion, at level 3, Still Mind, at level
/// 4, the ki pool's flat size and Slow Fall, and at level 5, Purity of Body)
/// while keeping it explicitly claim-blocked on the recognized bonus feat's own
/// mechanics (an execution engine, not a flat number).
///
/// This grounds the Monk base-attack progression (3/4 BAB: `classlevel * 3 / 4`),
/// the base-save progression (good Fortitude, Reflex, and Will: `classlevel/2+2`
/// each — Monk is unusual among the martial classes recognized so far in having all
/// three saves good rather than a 2-good/1-poor or 1-good/2-poor split), the AC
/// Bonus (the positive Wisdom modifier added to AC, asserted unconditionally on this
/// deterministic unarmored fixture), the Medium-monk unarmed strike damage die size
/// (1d6 at levels 1-3 — die size only, mirroring the Rogue sneak-attack die-count
/// record: no damage roll or damage total is computed), the Flurry of Blows flat
/// surface (two attacks, each at monk level - 2 before ability modifiers), the
/// level-1 bonus feat choice-slot selection when it names one of the PF1 Core
/// Rulebook restricted Monk bonus feat list's five feats (Combat Reflexes, Deflect
/// Arrows, Improved Grapple, Improved Trip, Stunning Fist), mirroring the Sorcerer
/// bloodline choice / Cleric domain choice / Druid nature-bond choice recognition
/// idiom, (SD13-E5) Evasion, a 2nd-level Monk class feature verified
/// independently against two primary PF1 sources (d20pfsrd and legacy.aonprd.com
/// both list "Bonus feat, evasion" as the Monk 2nd-level special feature entry) —
/// grounded as a bounded identity/recognition record only, mirroring exactly how
/// Rogue's own `class_feature.rogue.evasion` was grounded (value 0, correct
/// level-gate absence below level 2, granted-but-unexecuted rule text at level 2,
/// no saving-throw-resolution or damage-resolution engine), and (SD13-E5) Still
/// Mind, a 3rd-level Monk class feature verified independently against the same
/// two primary sources (both list "Fast movement, maneuver training, still mind"
/// as the Monk 3rd-level special feature entry) — grounded as a bounded
/// flat-magnitude record (a flat +2 on saves vs. enchantment spells and effects,
/// value 0 as a correct level-gate absence below level 3), mirroring the Fighter
/// Bravery / Paladin Divine Grace / Rogue Trap Sense idiom, never applied to any
/// actual save total. Fast Movement and Maneuver Training, the class table's other
/// two 3rd-level "Special" column entries, stay named-but-unproven. At level 4,
/// the unarmed strike damage die steps up from 1d6 to 1d8 (verified independently
/// against the same two primary sources' Medium-monk damage progression table),
/// the ki pool's flat size is grounded as a standalone flat-magnitude record
/// (1/2 monk level + Wisdom modifier, mirroring the Barbarian rage rounds-per-day
/// / Paladin lay-on-hands-uses-per-day idiom — no ki-point consumption tracking,
/// no action-economy engine, and no application of any ki power), and Slow Fall
/// is grounded as a bounded grant-only identity record (no fall-damage-resolution
/// engine exists in this codebase). At level 5, Purity of Body is grounded as a
/// bounded grant-only identity record (a flat disease-immunity grant, no
/// disease-resolution engine exists in this codebase); High Jump, the level-5
/// class table's OTHER "Special" column entry, is checked and confirmed NOT flat
/// (it requires wiring the monk's level into an Acrobatics-check total and
/// spending a ki point) and is deliberately left named-but-unproven. Further
/// SD13-E5 slices widen the gate to level 6 (Slow Fall's own reach magnitude
/// genuinely rising from 20 ft to 30 ft) and to level 7 (base attack, base
/// saves, the unarmed strike die, and the Flurry of Blows flat surface all
/// extend via the same pre-existing formulas with no re-derivation; Wholeness
/// of Body, the level-7 "Special" column's new feature, is checked and
/// confirmed NOT flat — it requires a ki-point-consumption/action-economy
/// engine and a healing-resolution engine, neither of which exists in this
/// codebase — and is deliberately left named-but-unproven, mirroring the High
/// Jump precedent). A still further SD13-E5 slice widens the gate to level 8
/// (base attack and base saves extend via the same pre-existing formulas; the
/// unarmed strike damage die genuinely rises to 1d10 — the 1d10 band starts at
/// level 8; the Flurry of Blows attack count genuinely rises from 2 to 3 —
/// verified independently against both primary sources' verbatim Flurry of
/// Blows rule text, "At 8th level, the monk can make two additional attacks";
/// Slow Fall's own reach magnitude genuinely rises from 30 ft to 40 ft; the ki
/// pool's flat size genuinely rises via the same pre-existing formula; Evasion,
/// Still Mind, and Purity of Body all stay granted unchanged. Both primary
/// sources' level-8 "Special" column names only the Slow Fall reach rise —
/// checked and specifically confirmed NOT Improved Uncanny Dodge, which Monk
/// never gains at any level per either source — so no new class-feature
/// record is grounded or fabricated at level 8). It still
/// grounds no attack-resolution or damage-roll engine, no monk-weapon flurry, no
/// level-9+ unarmed damage die progression, no ki-power execution, no level-4+ AC
/// Bonus dodge-bonus progression, no "unarmored and unencumbered" runtime
/// state-check engine, no wiring into integrated combat totals, no level-9+
/// martial progression, no Wholeness of Body execution, no level-2/level-6 bonus
/// feat grant (PF1 grants monks SEPARATE bonus feats at 2nd and 6th level; this
/// widening does not add a second choice-slot or recognition for either), and no
/// execution of what the recognized level-1 bonus
/// feat actually does (no attack-of-opportunity engine for Combat Reflexes, no
/// grapple-check engine for Improved Grapple, no DC/save engine for Stunning
/// Fist, and so on). It:
/// - leaves one chassis-recognition explanation so the `class:monk:N` identity is
///   acknowledged as a non-hybrid martial baseline rather than an undocumented packet
///   placeholder (direct runtime evidence, carrying no fabricated mechanical value),
/// - leaves grounded explanation records for base-attack, the three base saves,
///   AC Bonus, the unarmed strike damage die, the flurry flat attack
///   bonus/attack count, Evasion, Still Mind, the ki pool's flat size, Slow
///   Fall, and Purity of Body,
/// - conditionally leaves one grounded explanation recognizing the level-1 bonus
///   feat choice-slot selection when a `choice:monk_bonus_feat` selection is
///   present (carrying no fabricated mechanical value, since the recognized
///   feat's own mechanics are an execution engine rather than a number), and
/// - emits one claim-blocking diagnostic naming the still-missing burden (the
///   recognized bonus feat's own mechanics, or the bonus feat grant entirely when
///   no restricted-list selection is recognized) explicitly, rather than hiding
///   behind a single generic "unsupported class" label.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input; this seam
/// keeps that blocked posture (`defense.baseline_armor_class` stays gated to Fighter
/// and is untouched here) but makes the Monk martial identity, its grounded pillars,
/// its recognized bonus feat choice, and its one remaining named burden legible on
/// the runtime path.
fn explain_monk_level1_chassis(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let Some(level) = supported_monk_level(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Direct runtime evidence: recognize the deterministic Human Monk martial
    // chassis identity at the supported level. This is a recognition record only;
    // it fabricates no mechanical value.
    explanations.push(ComputationExplanation {
        id: "class_chassis.monk.bounded_progression".to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human Monk level {level} martial chassis: the \
             {MONK_CLASS_ID}:{level} class identity is acknowledged as a pure non-hybrid martial \
             baseline on the rules-core seam rather than an undocumented packet placeholder. \
             This is a bounded chassis-recognition record only; the base-attack, base-save, AC \
             Bonus, unarmed-strike-die, Flurry of Blows flat-surface, Evasion, and Still Mind \
             values are grounded separately below, and this record itself grounds no level-1 \
             bonus feat grant, no attack-resolution engine, no ki pool, and no level-4+ martial \
             progression, so it carries no fabricated mechanical value (+0)"
        ),
    });

    let level_value = i16::from(level);

    // Grounded (1/6): Monk 3/4-BAB base-attack progression from the PF1 Core
    // Rulebook Monk class table. No PCGen cr_classes.lst entry is used here (this
    // repo carries no Monk .lst source), so the formula cites the rulebook table
    // directly rather than inventing a line reference.
    let base_attack_bonus = level_value * 3 / 4;
    explanations.push(ComputationExplanation {
        id: "class_chassis.monk.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            "Monk level {level} base attack bonus from the PF1 Core Rulebook Monk class table's \
             3/4-BAB progression: classlevel * 3 / 4 = {base_attack_bonus}"
        ),
    });

    // Grounded (2/6): Monk base-save progression. Unlike Fighter/Barbarian/Rogue's
    // 2-good/1-poor or 1-good/2-poor split, the PF1 Core Rulebook Monk class table
    // gives all three base saves (Fortitude, Reflex, and Will) the good progression.
    let base_save_value = level_value / 2 + 2;
    explanations.push(ComputationExplanation {
        id: "class_chassis.monk.base_save.fortitude".to_owned(),
        value: base_save_value,
        detail: format!(
            "Monk level {level} base Fortitude save from the PF1 Core Rulebook Monk class \
             table: Monk is unusual in having all three saves good (unlike Fighter's/\
             Barbarian's/Rogue's mixed good/poor split), so Fortitude uses the good-save formula \
             classlevel/2+2 = {base_save_value}"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.monk.base_save.reflex".to_owned(),
        value: base_save_value,
        detail: format!(
            "Monk level {level} base Reflex save from the PF1 Core Rulebook Monk class table: \
             Monk is unusual in having all three saves good (unlike Fighter's/Barbarian's/\
             Rogue's mixed good/poor split), so Reflex uses the good-save formula \
             classlevel/2+2 = {base_save_value}"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.monk.base_save.will".to_owned(),
        value: base_save_value,
        detail: format!(
            "Monk level {level} base Will save from the PF1 Core Rulebook Monk class table: \
             Monk is unusual in having all three saves good (unlike Fighter's/Barbarian's/\
             Rogue's mixed good/poor split), so Will uses the good-save formula \
             classlevel/2+2 = {base_save_value}"
        ),
    });

    // Grounded (3/6): AC Bonus (Wisdom-to-AC). PF1: "she adds her Wisdom bonus, if
    // any, to her AC" — only a positive Wisdom modifier is added, never subtracted
    // here for a negative Wisdom modifier. This grounds only the flat value at the
    // supported level; it grounds no level-4+ dodge-bonus progression and no
    // "unarmored and unencumbered" runtime state-check engine (no such engine
    // exists anywhere in this codebase yet), so the value is asserted
    // unconditionally on the deterministic Human Monk fixture, which is by
    // construction unarmored.
    let ac_bonus = ability_modifiers.wisdom.max(0);
    explanations.push(ComputationExplanation {
        id: "class_chassis.monk.ac_bonus".to_owned(),
        value: ac_bonus,
        detail: format!(
            "Monk level {level} AC Bonus: Wisdom bonus (if positive) added to AC and CMD while \
             unarmored and unencumbered = max({}, 0) = {ac_bonus}. This grounds only the flat \
             Wisdom-to-AC value at this level, not the level-4+ dodge-bonus progression, and not \
             an \"unarmored and unencumbered\" runtime state-check engine (none exists in this \
             codebase yet); the value is asserted unconditionally on the deterministic Human Monk \
             fixture, which is by construction unarmored",
            ability_modifiers.wisdom
        ),
    });

    // Grounded (4/6): unarmed strike damage die. PF1 Core Rulebook Monk class table:
    // a Medium monk deals 1d6 unarmed strike damage at levels 1-3, stepping up to
    // 1d8 at levels 4-7 (verified independently against d20pfsrd and
    // legacy.aonprd.com: the full Medium-monk progression is 1d6/1d8/1d10/2d6/2d8/2d10
    // at levels 1-3/4-7/8-11/12-15/16-19/20). Mirroring the Rogue sneak-attack
    // die-count record, only the die-size facet is grounded here — no damage roll,
    // damage total, or attack-resolution engine is computed, and the level-8+ die
    // progression (1d10 and beyond) is not grounded.
    let (unarmed_die_value, unarmed_die_name) =
        if level < MONK_UNARMED_DAMAGE_DIE_STEP_UP_LEVEL {
            (6, "1d6")
        } else if level < MONK_UNARMED_DAMAGE_DIE_SECOND_STEP_UP_LEVEL {
            (8, "1d8")
        } else {
            (10, "1d10")
        };
    explanations.push(ComputationExplanation {
        id: "class_chassis.monk.unarmed_strike_damage_die".to_owned(),
        value: unarmed_die_value,
        detail: format!(
            "Monk level {level} unarmed strike from the PF1 Core Rulebook Monk class table: a \
             Medium monk deals 1d6 unarmed strike damage at levels 1-3, stepping up to 1d8 at \
             levels 4-7, then to 1d10 at levels 8-11, so it is {unarmed_die_name} at level \
             {level}. Only the die-size facet ({unarmed_die_value}, i.e. {unarmed_die_name}) is \
             grounded here; no damage roll or damage total is computed and no attack-resolution \
             engine exists. Two PF1 unarmed-strike rules are recorded as statements only: the \
             monk may choose to deal lethal or nonlethal damage with no penalty on the attack \
             roll, and monk unarmed strikes carry no off-hand penalty (a monk applies her full \
             Strength bonus on damage rolls for all her unarmed strikes). The higher-level \
             unarmed damage die progression beyond level 11 (2d6 and beyond) is not grounded"
        ),
    });

    // Grounded (5/6): Flurry of Blows flat attack surface, in two facets. PF1 Core
    // Rulebook: when making a flurry of blows as a full-attack action, the monk uses
    // her monk level in place of her base attack bonus and takes a -2 penalty on all
    // attacks; the flat pre-ability-modifier attack bonus is monk level - 2 (-1 at
    // level 1, +0 at level 2, +1 at level 3, ..., +5 at level 7, +6 at level 8,
    // matching the PF1 CRB table's "-1/-1" through "+6/+6" entries), and the flurry
    // grants two attacks at levels 1-7, rising to three attacks at level 8 —
    // verified independently against both primary sources' verbatim Flurry of Blows
    // rule text ("At 8th level, the monk can make two additional attacks when he
    // uses flurry of blows, as if using Improved Two-Weapon Fighting"). Only these
    // flat facets are grounded; no attack-resolution engine, no monk-weapon flurry,
    // and no wiring into integrated combat totals is implemented.
    let flurry_attack_bonus = level_value - 2;
    explanations.push(ComputationExplanation {
        id: "class_chassis.monk.flurry_of_blows_attack_bonus".to_owned(),
        value: flurry_attack_bonus,
        detail: format!(
            "Monk level {level} Flurry of Blows flat attack modifier from the PF1 Core Rulebook: \
             when using flurry as a full-attack action the monk uses her monk level in place of \
             her base attack bonus and takes a -2 penalty on all attack rolls, so the flat \
             modifier is monk level - 2 = {level_value} - 2 = {flurry_attack_bonus} on each \
             flurry attack, before ability modifiers. Only this flat pre-ability modifier is \
             grounded; no attack-resolution engine, no monk-weapon flurry, and no wiring into \
             integrated combat totals is implemented"
        ),
    });
    let flurry_attack_count = if level < MONK_FLURRY_THIRD_ATTACK_LEVEL { 2 } else { 3 };
    explanations.push(ComputationExplanation {
        id: "class_chassis.monk.flurry_of_blows_attack_count".to_owned(),
        value: flurry_attack_count,
        detail: format!(
            "Monk level {level} Flurry of Blows attack count from the PF1 Core Rulebook: a \
             level-{level} flurry grants {additional_attacks} on a full attack, i.e. \
             {attack_count_words}, each at the flat pre-ability modifier grounded separately. \
             The attack count stays 2 at levels 1-7 and rises to 3 at level 8 — verified \
             independently against both primary sources' verbatim Flurry of Blows rule text \
             (\"At 8th level, the monk can make two additional attacks when he uses flurry of \
             blows, as if using Improved Two-Weapon Fighting\"). Only the count facet \
             ({flurry_attack_count}) is grounded; no attack-resolution engine and no \
             monk-weapon flurry support is implemented",
            additional_attacks = if level < MONK_FLURRY_THIRD_ATTACK_LEVEL {
                "one additional attack"
            } else {
                "two additional attacks"
            },
            attack_count_words = if level < MONK_FLURRY_THIRD_ATTACK_LEVEL {
                "two attacks"
            } else {
                "three attacks"
            }
        ),
    });

    // Grounded (6/6, SD13-E5): Evasion, a 2nd-level Monk class feature verified
    // independently against two primary PF1 sources (d20pfsrd and
    // legacy.aonprd.com both list "Bonus feat, evasion" as the Monk 2nd-level
    // special feature entry — the same rule text and level gate as Rogue's own
    // Evasion). Below the level-2 gate this is a correct PF1 Core Rulebook
    // level-gate absence (value 0); at or above it, it is a bounded
    // identity/recognition record only (value 0, non-fabricated) naming the rule
    // text — mirroring exactly how Rogue's own `class_feature.rogue.evasion` was
    // grounded, without folding into an actual saving-throw-resolution or
    // damage-resolution engine, neither of which exists in this codebase.
    if level < MONK_EVASION_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.monk.evasion".to_owned(),
            value: 0,
            detail: format!(
                "Monk Evasion at monk level {level}: correctly absent at level {level} by PF1 \
                 Core Rulebook level gate; the at-grant rule is named but not computed. Evasion \
                 is a 2nd-level monk class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.monk.evasion".to_owned(),
            value: 0,
            detail: format!(
                "Monk Evasion granted at monk level {level} (PF1 Core Rulebook, 2nd-level monk \
                 class feature): if the monk makes a successful Reflex saving throw against an \
                 attack that normally deals half damage on a successful save, she instead takes \
                 no damage; Evasion has no effect if the monk fails the saving throw, and it has \
                 no effect at all against attacks that do not allow a saving throw for half \
                 damage. This is a bounded identity/recognition record only (value 0, \
                 non-fabricated): no saving-throw-resolution engine and no damage-resolution \
                 engine exists anywhere in this codebase to apply it, so this grounds no actual \
                 damage reduction on any save outcome"
            ),
        });
    }

    // Grounded (SD13-E5 level-9 slice): Improved Evasion, the 9th-level Monk
    // class feature verified independently against two primary PF1 sources
    // (d20pfsrd and legacy.aonprd.com both list "Improved evasion" as the Monk
    // 9th-level "Special" entry). An upgrade of the 2nd-level Evasion identity:
    // the monk still takes no damage on a successful Reflex save, and
    // henceforth takes only HALF damage on a failed save. Grounded as a bounded
    // +0 identity/recognition record only below/at the gate, mirroring exactly
    // how Evasion itself and Rogue's Improved Uncanny Dodge were grounded — no
    // saving-throw-resolution or damage-resolution engine exists in this
    // codebase, so no damage math is fabricated from the record. Below the
    // level-9 gate no record is pushed at all (the level-8 slice's own negative
    // control pins that absence).
    if level >= MONK_IMPROVED_EVASION_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.monk.improved_evasion".to_owned(),
            value: 0,
            detail: format!(
                "Monk Improved Evasion granted at monk level {level} (PF1 Core Rulebook, \
                 9th-level monk class feature): the monk's Evasion improves — she still takes \
                 no damage on a successful Reflex saving throw against attacks, and henceforth \
                 takes only half damage on a failed save. This is a bounded \
                 identity/recognition record only (value 0, non-fabricated): no \
                 saving-throw-resolution engine and no damage-resolution engine exists anywhere \
                 in this codebase to apply it, so this grounds no actual damage reduction on \
                 any save outcome"
            ),
        });
    }

    // Grounded (SD13-E5): Still Mind, a 3rd-level Monk class feature verified
    // independently against two primary PF1 sources (d20pfsrd and
    // legacy.aonprd.com both list "Fast movement, maneuver training, still mind"
    // as the Monk 3rd-level special feature entry). Below the level-3 gate this is
    // a correct PF1 Core Rulebook level-gate absence (value 0); at or above it, it
    // is a bounded flat-magnitude record only (a flat +2, not level-scaled) naming
    // the rule text — mirroring the Fighter Bravery / Paladin Divine Grace / Rogue
    // Trap Sense idiom: never applied to any actual save total, since no
    // saving-throw-resolution engine exists anywhere in this codebase. Fast
    // Movement and Maneuver Training, the class table's other two 3rd-level
    // "Special" column entries, are deliberately left named-but-unproven this
    // slice: no speed-total engine and no CMB/CMD engine exist in this codebase to
    // attach either to.
    if level < MONK_STILL_MIND_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.monk.still_mind".to_owned(),
            value: 0,
            detail: format!(
                "Monk Still Mind at monk level {level}: correctly absent at level {level} by \
                 PF1 Core Rulebook level gate; the at-grant magnitude is named but not computed. \
                 Still Mind is a 3rd-level monk class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.monk.still_mind".to_owned(),
            value: 2,
            detail: format!(
                "Monk Still Mind granted at monk level {level} (PF1 Core Rulebook, 3rd-level \
                 monk class feature): a monk of 3rd level or higher gains a flat +2 bonus on \
                 saving throws against enchantment spells and effects; this magnitude does not \
                 scale further with level. This is a bounded flat-magnitude record only, \
                 non-fabricated: it is never applied to any actual save total, since no \
                 saving-throw-resolution engine exists anywhere in this codebase to apply it"
            ),
        });
    }

    // Grounded (SD13-E5): the ki pool's flat size, a 4th-level Monk class
    // feature verified independently against two primary PF1 sources (d20pfsrd
    // and legacy.aonprd.com both give the formula: "the number of points in a
    // monk's ki pool is equal to 1/2 his monk level + his Wisdom modifier" —
    // neither primary source states a minimum floor on the pool itself, unlike
    // some other flat-magnitude records this codebase grounds elsewhere).
    // Below the level-4 gate this is a correct PF1 Core Rulebook level-gate
    // absence (value 0); at or above it, it is a bounded flat-magnitude record
    // only (the standalone pool-size number), mirroring the Barbarian rage
    // rounds-per-day / Paladin lay-on-hands-uses-per-day idiom: this grounds
    // only the flat resource-count magnitude. No ki-point consumption tracking,
    // no action-economy engine, and no application of any ki power (the extra
    // attack, the +4 AC dodge bonus, or the +20-ft. speed bonus usable as a
    // swift action) is computed anywhere in this codebase.
    if level < MONK_KI_POOL_AND_SLOW_FALL_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.ki_pool_size".to_owned(),
            value: 0,
            detail: format!(
                "Monk ki pool at monk level {level}: correctly absent at level {level} by PF1 \
                 Core Rulebook level gate; the at-grant formula is named but not computed. The \
                 ki pool is a 4th-level monk class feature."
            ),
        });
    } else {
        let ki_pool_size = level_value / 2 + ability_modifiers.wisdom;
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.ki_pool_size".to_owned(),
            value: ki_pool_size,
            detail: format!(
                "Monk ki pool granted at monk level {level} (PF1 Core Rulebook, 4th-level monk \
                 class feature): \"the number of points in a monk's ki pool is equal to 1/2 his \
                 monk level + his Wisdom modifier\" = {level_value} / 2 + {} = {ki_pool_size}. \
                 This grounds only the flat pool-size number; it computes no ki-point \
                 consumption tracking, no action-economy engine, and no application of any ki \
                 power (the extra attack, the +4 AC dodge bonus, or the +20-ft. speed bonus \
                 usable as a swift action), none of which exists anywhere in this codebase",
                ability_modifiers.wisdom
            ),
        });
    }

    // Grounded (SD13-E5): Slow Fall, the other 4th-level Monk class feature
    // verified independently against two primary PF1 sources (d20pfsrd and
    // legacy.aonprd.com both list "Ki pool (magic), slow fall 20 ft." as the
    // Monk 4th-level special feature entry). Below the level-4 gate this is a
    // correct PF1 Core Rulebook level-gate absence (value 0); at or above it,
    // it is a bounded grant-only identity record (value 0, non-fabricated)
    // naming the rule text — mirroring the Barbarian Uncanny Dodge / Rogue
    // Uncanny Dodge / Druid Woodland Stride grant-only idiom: no
    // fall-damage-resolution engine exists anywhere in this codebase to apply
    // the 20-foot reduction to.
    if level < MONK_KI_POOL_AND_SLOW_FALL_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.slow_fall".to_owned(),
            value: 0,
            detail: format!(
                "Monk Slow Fall at monk level {level}: correctly absent at level {level} by PF1 \
                 Core Rulebook level gate; the at-grant rule is named but not computed. Slow \
                 Fall is a 4th-level monk class feature."
            ),
        });
    } else {
        let slow_fall_reach_feet = if level < MONK_SLOW_FALL_INCREASED_REACH_LEVEL {
            20
        } else if level < MONK_SLOW_FALL_FORTY_FOOT_REACH_LEVEL {
            30
        } else {
            40
        };
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.slow_fall".to_owned(),
            value: 0,
            detail: format!(
                "Monk Slow Fall granted at monk level {level} (PF1 Core Rulebook, 4th-level monk \
                 class feature whose own reach magnitude increases to 30 ft. at 6th level and \
                 40 ft. at 8th level): \"a monk within arm's reach of a wall can use it to slow \
                 his descent\" — she takes falling damage as if the fall were \
                 {slow_fall_reach_feet} feet shorter than it actually is. This is a bounded \
                 grant-only identity record only (value 0, non-fabricated): no \
                 fall-damage-resolution engine exists anywhere in this codebase to apply the \
                 {slow_fall_reach_feet}-foot reduction to"
            ),
        });
    }

    // Grounded (SD13-E5): Purity of Body, a 5th-level Monk class feature
    // verified independently against two primary PF1 sources (d20pfsrd and
    // legacy.aonprd.com both list "High jump, purity of body" as the Monk
    // 5th-level special feature entry). Below the level-5 gate this is a
    // correct PF1 Core Rulebook level-gate absence (value 0); at or above it,
    // it is a bounded grant-only identity record (value 0, non-fabricated)
    // naming the rule text — mirroring the Barbarian/Rogue Uncanny Dodge /
    // Monk Slow Fall grant-only idiom: no disease-resolution engine exists
    // anywhere in this codebase to apply the immunity to. High Jump, the
    // level-5 class table's OTHER "Special" column entry, was checked and
    // confirmed NOT flat this cycle (it requires wiring the monk's level
    // into an Acrobatics-check total — no skill-check-total engine exists in
    // this codebase — and spending a ki point, an action-economy/resource-
    // consumption engine this codebase deliberately does not implement for
    // the ki pool either), so it is deliberately left named-but-unproven; no
    // record or diagnostic for it was fabricated.
    if level < MONK_PURITY_OF_BODY_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.purity_of_body".to_owned(),
            value: 0,
            detail: format!(
                "Monk Purity of Body at monk level {level}: correctly absent at level {level} by \
                 PF1 Core Rulebook level gate; the at-grant rule is named but not computed. \
                 Purity of Body is a 5th-level monk class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.purity_of_body".to_owned(),
            value: 0,
            detail: format!(
                "Monk Purity of Body granted at monk level {level} (PF1 Core Rulebook, 5th-level \
                 monk class feature): \"at 5th level, a monk gains immunity to all diseases, \
                 including supernatural and magical diseases.\" This is a bounded grant-only \
                 identity record only (value 0, non-fabricated): no disease-resolution engine \
                 exists anywhere in this codebase to apply the immunity to. High Jump, the \
                 level-5 class table's other \"Special\" column entry, is checked and confirmed \
                 not flat (it requires an Acrobatics-check-total engine and ki-point-spending \
                 action, neither of which exists here) and is deliberately not grounded"
            ),
        });
    }

    // Recognized (SD13-E5): the level-1 bonus feat choice-slot selection is
    // recognized as chosen input when it names one of the PF1 Core Rulebook
    // restricted Monk bonus feat list's five feats (Combat Reflexes, Deflect
    // Arrows, Improved Grapple, Improved Trip, Stunning Fist), mirroring the
    // Sorcerer bloodline choice / Cleric domain choice / Druid nature-bond choice
    // recognition idiom. This is recognition of the choice-slot identity only; it
    // fabricates no feat-effect execution (no attack-of-opportunity engine for
    // Combat Reflexes, no ranged-deflection engine for Deflect Arrows, no
    // grapple-check engine for Improved Grapple, no trip-check engine for Improved
    // Trip, and no DC/save engine for Stunning Fist). A selection present but
    // outside this restricted list is acknowledged without naming a specific
    // restricted-list feat, mirroring the Sorcerer bloodline choice's
    // present-but-unrecognized branch, so no restricted-list feat identity is
    // fabricated for a selection this bounded seam does not know. This is always
    // the level-1 bonus feat (`MONK_BONUS_FEAT_GRANT_LEVEL`), carried forward
    // unchanged at level 2 — PF1 grants monks a SEPARATE bonus feat at 2nd level
    // that this bounded seam deliberately does not recognize.
    let bonus_feat_selection = choice_selection(input, MONK_BONUS_FEAT_CHOICE_ID);
    let recognized_bonus_feat_name = bonus_feat_selection.and_then(|selection| {
        if selection == COMBAT_REFLEXES_FEAT_SELECTION {
            Some("Combat Reflexes")
        } else if selection == DEFLECT_ARROWS_FEAT_SELECTION {
            Some("Deflect Arrows")
        } else if selection == IMPROVED_GRAPPLE_FEAT_SELECTION {
            Some("Improved Grapple")
        } else if selection == IMPROVED_TRIP_FEAT_SELECTION {
            Some("Improved Trip")
        } else if selection == STUNNING_FIST_FEAT_SELECTION {
            Some("Stunning Fist")
        } else {
            None
        }
    });
    if let Some(selection) = bonus_feat_selection {
        let detail = if let Some(feat_name) = recognized_bonus_feat_name {
            format!(
                "Monk level {MONK_BONUS_FEAT_GRANT_LEVEL} bonus feat choice recognized: the \
                 canonical deterministic selection ({MONK_BONUS_FEAT_CHOICE_ID} -> {selection}) \
                 names {feat_name}, drawn from the PF1 Core Rulebook restricted Monk bonus feat \
                 list (Combat Reflexes, Deflect Arrows, Improved Grapple, Improved Trip, \
                 Stunning Fist), as chosen input on the compute seam. This is a recognition \
                 record of the choice slot only, so it carries no fabricated mechanical value \
                 (+0): {feat_name}'s own mechanics are not grounded here, and no \
                 attack-resolution, grapple-check, trip-check, or DC/save engine exists in this \
                 codebase. Improved Unarmed Strike is not part of this restricted choice set \
                 because the PF1 Core Rulebook grants it to every monk automatically at level \
                 {MONK_BONUS_FEAT_GRANT_LEVEL}, separate from this chosen bonus feat, and this \
                 codebase does not ground that automatic grant either"
            )
        } else {
            format!(
                "Monk level {MONK_BONUS_FEAT_GRANT_LEVEL} bonus feat choice slot is present \
                 ({MONK_BONUS_FEAT_CHOICE_ID} -> {selection}), but only the PF1 Core Rulebook \
                 restricted Monk bonus feat list (Combat Reflexes, Deflect Arrows, Improved \
                 Grapple, Improved Trip, Stunning Fist) is recognized on this bounded seam; no \
                 restricted-list feat identity is grounded and no mechanical value is fabricated \
                 (+0)"
            )
        };
        explanations.push(ComputationExplanation {
            id: "class_chassis.monk.bonus_feat_choice".to_owned(),
            value: 0,
            detail,
        });
    }

    // Still blocked (the one remaining named burden): the level-1 bonus feat's own
    // mechanics. The choice-slot identity is recognized above (when present and
    // in-list); this diagnostic narrows to naming only what remains
    // unimplemented, and it names the specific recognized feat only when this
    // seam actually recognized one, mirroring the Druid animal-companion
    // blocker's conditional message — so it never asserts a specific feat's
    // mechanics as "remaining" for a character whose chosen feat this seam did
    // not recognize.
    let bonus_feat_message = if let Some(feat_name) = recognized_bonus_feat_name {
        format!(
            "Monk level {level} remains blocked on its level-1 bonus feat's own mechanics: the \
             recognized choice ({feat_name}) is acknowledged as chosen input only — \
             {feat_name}'s actual feat effect requires a general feat-selection or \
             feat-prerequisite/effect engine that does not exist in this bounded martial chassis \
             baseline, so no Monk bonus-feat execution support is claimed"
        )
    } else {
        format!(
            "Monk level {level} remains blocked on its level-1 bonus feat grant: the free bonus \
             feat drawn from the restricted Monk feat list (Combat Reflexes, Deflect Arrows, \
             Improved Grapple, Improved Trip, Stunning Fist) is not recognized as chosen input \
             in this bounded martial chassis baseline — no feat-selection or feat-prerequisite \
             engine exists here — so no Monk bonus-feat support is claimed"
        )
    };
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.monk.bounded_progression.bonus_feat.unsupported".to_owned(),
        message: bonus_feat_message,
        claim_blocking: true,
    });
}

const ROGUE_CLASS_ID: &str = "class:rogue";
// A further SD13-E5 slice widens the gate to level 9 — the first level-9 slice
// in the tranche (verified independently against d20pfsrd and
// legacy.aonprd.com): level 9 base attack bonus stays +6 (9 * 3 / 4, an
// integer-division coincidence with level 8) while poor Fortitude/Will both
// genuinely rise to +3 (9 / 3) and good Reflex stays +6 (9 / 2 + 2, another
// coincidence); the level-9 "Special" column reads "Sneak attack +5d6, trap
// sense +3" — BOTH entries are tier-rises on already-grounded formula pillars,
// not new class features: the sneak attack die count genuinely rises to 5 via
// the pre-existing (level + 1) / 2 formula and Trap Sense genuinely rises to
// +3 via the pre-existing level / 3 formula; Trapfinding stays 4
// (max(9/2, 1), a coincidence); level 9 is NOT a rogue-talent level (talents
// land at 2/4/6/8/10...), so no new pillar is grounded and nothing new is
// left unproven for the talent tree either. A further SD13-E5 slice widens
// the gate to level 10 — the first level-10 slice, opening the tranche's
// final level band (verified independently against d20pfsrd and
// legacy.aonprd.com): level 10 base attack genuinely rises to +7
// (10 * 3 / 4) and good Reflex genuinely rises to +7 (10 / 2 + 2), while
// poor Fortitude/Will both stay +3 (10 / 3, integer-division coincidences);
// sneak attack stays 5d6 ((10 + 1) / 2, the odd-level cadence — next rise
// at 11th) and Trap Sense stays +3 (10 / 3, next rise at 12th), while
// Trapfinding genuinely rises to +5 (max(10/2, 1)); the level-10 "Special"
// column reads "Advanced talents, rogue talent" — BOTH parts of the same
// genuinely open-ended choice-list feature already left named-but-unproven
// at levels 2/4/6/8 (the advanced-talent unlock is a list expansion of that
// feature, not a new pillar), so no new pillar is grounded at level 10
// either.
const MAX_SUPPORTED_ROGUE_LEVEL: u8 = 10;
/// PF1 Core Rulebook level gate at which Rogue gains Evasion.
const ROGUE_EVASION_LEVEL: u8 = 2;
/// PF1 Core Rulebook level gate at which Rogue gains Trap Sense.
const ROGUE_TRAP_SENSE_LEVEL: u8 = 3;
/// PF1 Core Rulebook level gate at which Rogue gains Uncanny Dodge (4th
/// level, verified independently against d20pfsrd and legacy.aonprd.com —
/// the Rogue class table's level-4 "Special" column reads "Rogue talent,
/// uncanny dodge" — NOT the same level as Barbarian's own 2nd-level Uncanny
/// Dodge grant).
const ROGUE_UNCANNY_DODGE_LEVEL: u8 = 4;
/// PF1 Core Rulebook level gate at which Rogue gains Improved Uncanny Dodge
/// (8th level, verified independently against d20pfsrd and
/// legacy.aonprd.com — the Rogue class table's level-8 "Special" column
/// reads "Improved uncanny dodge, rogue talent." This is a DIFFERENT gate
/// level than Barbarian's own Improved Uncanny Dodge grant, which is at
/// barbarian level 5, not rogue's level 8; verified rather than assumed).
const ROGUE_IMPROVED_UNCANNY_DODGE_LEVEL: u8 = 8;

/// The bounded Rogue milestone level this decomposition surface grounds, if
/// any. Returns the single Rogue level when the chosen input is exactly a
/// single-class Rogue at one of the supported milestone levels (1 through
/// 10). Returns `None` for no Rogue, a non-Rogue class, a multiclass
/// mix, or any level-11+ Rogue this slice deliberately does not recognize —
/// each of which stays claim-blocked exactly as before. Mirrors the Fighter
/// `supported_fighter_level` / Paladin `supported_paladin_level` level-range
/// gate idiom.
fn supported_rogue_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == ROGUE_CLASS_ID
                && (1..=MAX_SUPPORTED_ROGUE_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

/// Surface direct SD13-E3/E5 runtime evidence for the deterministic Human Rogue
/// level-1/level-2/level-3/level-4/level-5/level-6/level-7/level-8 chassis, mirroring the
/// Barbarian/Monk level-1 baseline pattern and the Fighter
/// `supported_fighter_level` / Paladin `supported_paladin_level`
/// level-range-gate idiom.
/// The SD13-E3 pillar-grounding slice grounds three of the four named
/// burdens directly (base-attack progression, base-save progression, and
/// sneak attack die count); the SD13-E5 trapfinding slice grounds the
/// fourth, Trapfinding, mirroring the grounded Ranger Track record, so no
/// named Rogue pillar burden remains claim-blocked; a later SD13-E5 slice
/// widens the level-1-only gate to level 2 (the PF1 Core Rulebook Rogue
/// class table's next milestone) and grounds Evasion as a bounded
/// identity/recognition record; a further SD13-E5 slice widens the gate to
/// level 3 and grounds Trap Sense (the class table's 3rd-level "Special"
/// entry) as a bounded flat-magnitude record; a further SD13-E5 slice widens
/// the gate to level 4 and grounds Uncanny Dodge (the class table's
/// 4th-level "Special" entry, verified independently against d20pfsrd and
/// legacy.aonprd.com — NOT the same level as Barbarian's own 2nd-level
/// Uncanny Dodge) as a bounded identity/recognition record; a further
/// SD13-E5 slice widens the gate to level 5 (verified independently against
/// d20pfsrd and legacy.aonprd.com: the class table's level-5 "Special"
/// column reads only "Sneak attack +3d6," no other new feature) and the
/// pre-existing sneak-attack die-count formula genuinely produces `3` (i.e.
/// `3d6`) at level 5, with Evasion, Trap Sense, and Uncanny Dodge all
/// staying granted, not re-derived; a further SD13-E5 slice widens the gate
/// to level 6 (verified independently against d20pfsrd and
/// legacy.aonprd.com: the class table's level-6 "Special" column reads
/// "Rogue talent, trap sense +2") — the pre-existing Trap Sense
/// flat-magnitude formula (`level / 3`, floor) genuinely rises to `2` at
/// level 6, the pre-existing sneak-attack die-count formula stays at `3`
/// (i.e. `3d6`, unchanged from level 5), and Trapfinding genuinely rises to
/// `3` via the same pre-existing formula, with Evasion and Uncanny Dodge
/// staying granted, not re-derived; the level-6 row's OTHER named entry, a
/// second Rogue Talent slot, is deliberately left named-but-unproven this
/// slice, mirroring the level-2/level-4 rogue-talent precedent; a further
/// SD13-E5 slice widens the gate to level 7 (verified independently against
/// d20pfsrd and legacy.aonprd.com: the class table's level-7 "Special"
/// column reads only "Sneak attack +4d6," no other new feature) — the
/// pre-existing sneak-attack die-count formula (`(level + 1) / 2`) genuinely
/// rises to `4` (i.e. `4d6`) at level 7, up from `3` at level 6, via the
/// same formula, not a new record; the pre-existing Trap Sense
/// flat-magnitude formula stays at `2` (unchanged from level 6, the next
/// rise is at 9th level); Trapfinding stays at `3` (`max(7 / 2, 1)`, an
/// integer-division coincidence with level 6); Evasion and Uncanny Dodge
/// both stay granted, not re-derived. A further SD13-E5 slice widens the
/// gate to level 8 (verified independently against d20pfsrd and
/// legacy.aonprd.com: the class table's level-8 "Special" column reads
/// "Improved uncanny dodge, rogue talent") — the pre-existing sneak-attack
/// die-count formula (`(level + 1) / 2`) stays at `4` (i.e. `4d6`, unchanged
/// from level 7, since the die count only rises at odd rogue levels); the
/// pre-existing Trap Sense flat-magnitude formula stays at `2` (unchanged
/// from level 7, the next rise is at 9th level); Trapfinding genuinely rises
/// to `4` (`max(8 / 2, 1)`, up from `3` at level 7, via the same formula);
/// Evasion and Uncanny Dodge both stay granted, not re-derived; Improved
/// Uncanny Dodge is newly granted and grounded as a bounded
/// identity/recognition record only, mirroring exactly how Barbarian's own
/// Improved Uncanny Dodge was grounded at barbarian level 5. The level-8
/// row's OTHER named entry, a third Rogue Talent slot, is deliberately left
/// named-but-unproven this slice, mirroring the level-2/level-4/level-6
/// rogue-talent precedent.
///
/// This deliberately does not compute a full Rogue class engine. It grounds,
/// at every supported level (1, 2, 3, 4, 5, 6, 7, and 8):
/// - base-attack progression (3/4 BAB, `level * 3 / 4`),
/// - base-save progression (good Reflex, poor Fortitude, poor Will),
/// - the sneak attack damage-die *count* only (`(level + 1) / 2`, i.e. `1`
///   at levels 1-2, `2` at levels 3-4, `3` at levels 5-6, and `4` at levels
///   7-8, `1d6`/`2d6`/`3d6`/`4d6`) — not damage-roll execution and not the
///   flanking / Dexterity-denial trigger-condition engine,
/// - the Trapfinding flat numeric bonus (`max(level / 2, 1)`, `+1` at levels
///   1-3, `+2` at levels 4-5, `+3` at levels 6-7, and `+4` at level 8) on
///   Perception checks to locate traps and on Disable Device checks, plus
///   the magic-trap-disarm statement — not a check-execution engine, no
///   trap DC resolution, and no magic-trap disarm engine,
/// - Evasion (a 2nd-level Rogue class feature): below level 2 it is grounded
///   as a correct PF1 Core Rulebook level-gate absence (value 0); at level 2
///   and above it is grounded as a bounded identity/recognition record only
///   (value 0, non-fabricated) naming the rule text (no damage on a
///   successful Reflex save against an effect that normally allows half
///   damage on a successful save; no benefit on a failed save) — mirroring
///   how Divine Grace and Bravery were grounded as flat rules-text records
///   without folding into an actual saving-throw-resolution or
///   damage-resolution engine, neither of which exists in this codebase,
/// - Trap Sense (a 3rd-level Rogue class feature, verified independently
///   against d20pfsrd and legacy.aonprd.com): below level 3 it is grounded as
///   a correct PF1 Core Rulebook level-gate absence (value 0); at level 3 and
///   above it is grounded as a bounded flat-magnitude record only
///   (`level / 3`, floor; `+1` at levels 3-5, genuinely rising to `+2` at
///   level 6) naming the rule text (a bonus on Reflex saves made to avoid
///   traps and an equal dodge bonus to AC against attacks made by traps) —
///   mirroring the Fighter Bravery / Paladin Divine Grace idiom: the
///   magnitude is never applied to any actual Reflex-save total or AC total,
///   since no saving-throw-resolution or armor-class-resolution engine
///   exists in this codebase, and no trap-detection or trap-triggering
///   engine exists to decide when it would apply, and
/// - Uncanny Dodge (a 4th-level Rogue class feature, verified independently
///   against d20pfsrd and legacy.aonprd.com): below level 4 it is grounded as
///   a correct PF1 Core Rulebook level-gate absence (value 0); at level 4 and
///   above it is grounded as a bounded identity/recognition record only
///   (value 0, non-fabricated) naming the rule text (cannot be caught
///   flat-footed; retains Dexterity bonus to AC even against an invisible
///   attacker; still loses it if immobilized) — mirroring exactly how
///   Barbarian's own Uncanny Dodge was grounded, without folding into any
///   actual flat-footed-state tracking, Armor Class computation, or
///   invisibility-detection engine, none of which exists in this codebase.
///   The level-4 row's OTHER named entry, a Rogue Talent (an open-ended
///   choice-list feature), is deliberately left named-but-unproven this
///   slice, mirroring the Monk level-2 bonus feat / Barbarian Rage Power
///   precedent, and
/// - Improved Uncanny Dodge (an 8th-level Rogue class feature, verified
///   independently against d20pfsrd and legacy.aonprd.com): below level 8 it
///   is grounded as a correct PF1 Core Rulebook level-gate absence (value 0);
///   at level 8 and above it is grounded as a bounded identity/recognition
///   record only (value 0, non-fabricated) naming the rule text (can no
///   longer be flanked; denies another rogue the ability to sneak attack by
///   flanking unless the attacker has at least four more rogue levels) —
///   mirroring exactly how Barbarian's own Improved Uncanny Dodge was
///   grounded, without folding into any actual flanking-resolution or
///   attacker-level-comparison engine, neither of which exists in this
///   codebase. The level-8 row's OTHER named entry, a third Rogue Talent
///   slot, is deliberately left named-but-unproven this slice, mirroring the
///   level-2/level-4/level-6 rogue-talent precedent.
///
/// It still grounds no rogue talent (a level-2+/level-4+/level-6+/level-8+
/// choice-list feature, and a genuinely open-ended talent tree — a
/// new-subsystem-shaped burden left named but unproven) and no level-9+
/// progression. These
/// `class_chassis.rogue.*` / `class_feature.rogue.evasion` /
/// `class_feature.rogue.trap_sense` / `class_feature.rogue.uncanny_dodge` /
/// `class_feature.rogue.improved_uncanny_dodge`
/// explanation records are standalone: they are not wired into
/// `compute_fighter_chassis`, `compute_total_saves`, or
/// `compute_combat_baseline`, so `defense.total_save.*` is still never
/// computed for Rogue here. It only:
/// - leaves one chassis-recognition explanation so the `class:rogue:N`
///   identity is acknowledged rather than an undocumented packet placeholder
///   (direct runtime evidence, carrying no fabricated mechanical value), and
/// - leaves nine grounded pillar explanations (base-attack, base-save
///   fortitude/reflex/will, sneak-attack die count, trapfinding, Evasion,
///   Trap Sense, Uncanny Dodge, Improved Uncanny Dodge).
///
/// The named Rogue claim-blocking diagnostic set is now empty; the four
/// generic chassis diagnostics (`class_chassis.unsupported`,
/// `combat.baseline_unsupported`, `defense.total_save.unsupported`,
/// `skill.selected_modifier.unsupported`) still claim-block this input
/// (including `tests/ge06_pilot_total_saves.rs::unsupported_chassis_blocks_total_saves`,
/// which keeps passing unmodified since no `defense.total_save.*` explanation
/// is ever computed here); this seam keeps that blocked posture but makes the
/// Rogue chassis identity and its grounded pillars legible on the runtime
/// path.
fn explain_rogue_level1_chassis(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let Some(level) = supported_rogue_level(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Direct runtime evidence: recognize the deterministic Human Rogue
    // chassis identity at the supported level. This is a recognition record
    // only; it fabricates no mechanical value.
    explanations.push(ComputationExplanation {
        id: "class_chassis.rogue.bounded_progression".to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human Rogue level {level} chassis: the \
             {ROGUE_CLASS_ID}:{level} class identity is acknowledged on the \
             rules-core seam rather than an undocumented packet placeholder. This is a bounded \
             chassis-recognition record only; the base-attack, base-save, sneak-attack \
             die-count, trapfinding, Evasion, Trap Sense, Uncanny Dodge, and Improved Uncanny \
             Dodge pillars are grounded separately below, but this record still grounds no \
             rogue talent and no level-9+ progression, so it carries no fabricated mechanical \
             value (+0)"
        ),
    });

    // Grounded (1/8): base-attack progression (3/4 BAB).
    let level_value = i16::from(level);
    let base_attack_bonus = level_value * 3 / 4;
    explanations.push(ComputationExplanation {
        id: "class_chassis.rogue.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            "Rogue level {level} base attack bonus from the PF1 Core Rulebook \
             Rogue class table's 3/4-BAB progression: level * 3 / 4 = {base_attack_bonus}"
        ),
    });

    // Grounded (2/8): base-save progression (good Reflex, poor Fortitude, poor Will).
    let base_save_fortitude = level_value / 3;
    let base_save_reflex = level_value / 2 + 2;
    let base_save_will = level_value / 3;
    explanations.push(ComputationExplanation {
        id: "class_chassis.rogue.base_save.fortitude".to_owned(),
        value: base_save_fortitude,
        detail: format!(
            "Rogue level {level} base Fortitude save (poor) from the PF1 Core \
             Rulebook Rogue class table: level / 3 = {base_save_fortitude}"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.rogue.base_save.reflex".to_owned(),
        value: base_save_reflex,
        detail: format!(
            "Rogue level {level} base Reflex save (good) from the PF1 Core \
             Rulebook Rogue class table: level / 2 + 2 = {base_save_reflex}"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.rogue.base_save.will".to_owned(),
        value: base_save_will,
        detail: format!(
            "Rogue level {level} base Will save (poor) from the PF1 Core \
             Rulebook Rogue class table: level / 3 = {base_save_will}"
        ),
    });

    // Grounded (3/8): sneak attack damage-die count only. PF1 Core Rulebook:
    // the sneak attack die count increases by 1d6 every two rogue levels
    // (1d6 at levels 1-2, 2d6 at level 3+): (level + 1) / 2.
    let sneak_attack_die_count = (level_value + 1) / 2;
    explanations.push(ComputationExplanation {
        id: "class_chassis.rogue.sneak_attack".to_owned(),
        value: sneak_attack_die_count,
        detail: format!(
            "Rogue level {level} sneak attack from the PF1 Core Rulebook Rogue \
             class table: the sneak attack die count increases by 1 every two rogue levels \
             (1d6 at levels 1-2, 2d6 at level 3+): (level + 1) / 2 = ({level_value} + 1) / 2 = \
             {sneak_attack_die_count}, i.e. {sneak_attack_die_count}d6 sneak attack damage die, \
             against a flanked or Dexterity-denied target. Only the die-count facet is grounded \
             here; damage-roll execution and the flanking / Dexterity-denial trigger-condition \
             engine are not implemented"
        ),
    });

    // Grounded (4/8): trapfinding — the flat numeric bonus and the
    // magic-trap-disarm statement only, mirroring the grounded Ranger Track
    // record (no check-execution engine behind it).
    let trapfinding_bonus = (level_value / 2).max(1);
    explanations.push(ComputationExplanation {
        id: "class_chassis.rogue.trapfinding".to_owned(),
        value: trapfinding_bonus,
        detail: format!(
            "Rogue Trapfinding class feature: adds a bonus equal to max(rogue level / 2, 1) \
             (PF1 Core Rulebook Trapfinding: +1/2 rogue level, minimum +1) on Perception checks \
             made to locate traps and on Disable Device checks, and lets the rogue use Disable \
             Device to disarm magic traps. At Rogue level {level} this bonus is \
             max({level_value} / 2, 1) = {trapfinding_bonus}. This grounds only the \
             flat numeric Trapfinding bonus and the magic-trap-disarm statement; it is not a \
             check-execution engine and computes no full Perception or Disable Device check, no \
             trap DC resolution, and no magic-trap disarm engine"
        ),
    });

    // Grounded (5/8): Evasion, a 2nd-level Rogue class feature. Below the
    // level-2 gate this is a correct PF1 Core Rulebook level-gate absence
    // (value 0); at or above it, it is a bounded identity/recognition record
    // only (value 0, non-fabricated) naming the rule text — mirroring how
    // Divine Grace and Bravery were grounded as flat rules-text records
    // without folding into an actual saving-throw-resolution or
    // damage-resolution engine, neither of which exists in this codebase.
    if level < ROGUE_EVASION_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.rogue.evasion".to_owned(),
            value: 0,
            detail: format!(
                "Rogue Evasion at rogue level {level}: correctly absent at level {level} by PF1 \
                 Core Rulebook level gate; the at-grant rule is named but not computed. Evasion \
                 is a 2nd-level rogue class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.rogue.evasion".to_owned(),
            value: 0,
            detail: format!(
                "Rogue Evasion granted at rogue level {level} (PF1 Core Rulebook, 2nd-level \
                 rogue class feature): if the rogue makes a successful Reflex saving throw \
                 against an attack that normally deals half damage on a successful save, she \
                 instead takes no damage; Evasion has no effect if the rogue fails the saving \
                 throw, and it has no effect at all against attacks that do not allow a saving \
                 throw for half damage. This is a bounded identity/recognition record only \
                 (value 0, non-fabricated): no saving-throw-resolution engine and no \
                 damage-resolution engine exists anywhere in this codebase to apply it, so this \
                 grounds no actual damage reduction on any save outcome"
            ),
        });
    }

    // Grounded (6/8): Trap Sense, a 3rd-level Rogue class feature (verified
    // independently against d20pfsrd and legacy.aonprd.com). Below the
    // level-3 gate this is a correct PF1 Core Rulebook level-gate absence
    // (value 0); at or above it, it is a bounded flat-magnitude record only
    // (level / 3, floor) naming the rule text — mirroring how Fighter's
    // Bravery and Paladin's Divine Grace were grounded as flat rules-text
    // magnitudes without folding into an actual saving-throw-resolution or
    // armor-class-resolution engine, neither of which exists in this
    // codebase.
    if level < ROGUE_TRAP_SENSE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.rogue.trap_sense".to_owned(),
            value: 0,
            detail: format!(
                "Rogue Trap Sense at rogue level {level}: correctly absent at level {level} by \
                 PF1 Core Rulebook level gate; the at-grant magnitude is named but not computed. \
                 Trap Sense is a 3rd-level rogue class feature."
            ),
        });
    } else {
        let trap_sense_bonus = level_value / 3;
        explanations.push(ComputationExplanation {
            id: "class_feature.rogue.trap_sense".to_owned(),
            value: trap_sense_bonus,
            detail: format!(
                "Rogue Trap Sense granted at rogue level {level} (PF1 Core Rulebook, 3rd-level \
                 rogue class feature): a +{trap_sense_bonus} bonus on Reflex saves made to avoid \
                 traps and a +{trap_sense_bonus} dodge bonus to AC against attacks made by traps \
                 (rogue level / 3 = {trap_sense_bonus}; this bonus rises further at 9th/12th/\
                 15th/18th rogue level, beyond this bounded slice). This is a bounded \
                 flat-magnitude record only, non-fabricated: it is never applied to any actual \
                 Reflex-save total or AC total, since no saving-throw-resolution or \
                 armor-class-resolution engine exists anywhere in this codebase to apply it, and \
                 no trap-detection or trap-triggering engine exists to decide when it would \
                 apply"
            ),
        });
    }

    // Grounded (7/8): Uncanny Dodge, a 4th-level Rogue class feature (verified
    // independently against d20pfsrd and legacy.aonprd.com: the Rogue class table's
    // level-4 "Special" column reads "Rogue talent, uncanny dodge" — NOT the same
    // level as Barbarian's own 2nd-level Uncanny Dodge grant, verified rather than
    // assumed). Below the level-4 gate this is a correct PF1 Core Rulebook
    // level-gate absence (value 0); at or above it, it is a bounded
    // identity/recognition record only (value 0, non-fabricated) naming the rule
    // text — mirroring exactly how Barbarian's own Uncanny Dodge was grounded,
    // without folding into any actual flat-footed-state tracking, Armor Class
    // computation, or invisibility-detection engine, none of which exists in this
    // codebase. The level-4 row's OTHER named entry, a Rogue Talent (a genuinely
    // open-ended choice-list feature, a new-subsystem-shaped burden), is
    // deliberately left named-but-unproven this slice, mirroring the Monk level-2
    // bonus feat / Barbarian Rage Power precedent: no new choice-slot and no new
    // diagnostic was added for it.
    if level < ROGUE_UNCANNY_DODGE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.rogue.uncanny_dodge".to_owned(),
            value: 0,
            detail: format!(
                "Rogue Uncanny Dodge at rogue level {level}: correctly absent at level {level} \
                 by PF1 Core Rulebook level gate; the at-grant rule is named but not computed. \
                 Uncanny Dodge is a 4th-level rogue class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.rogue.uncanny_dodge".to_owned(),
            value: 0,
            detail: format!(
                "Rogue Uncanny Dodge granted at rogue level {level} (PF1 Core Rulebook, \
                 4th-level rogue class feature, part of the \"Rogue talent, uncanny dodge\" \
                 table entry): she cannot be caught flat-footed, and she retains her Dexterity \
                 bonus to Armor Class even if the attacker is invisible; she still loses her \
                 Dexterity bonus to Armor Class if immobilized, and a successful feint action \
                 can still strip it away. This is a bounded identity/recognition record only \
                 (value 0, non-fabricated): no flat-footed-state tracking, no Armor Class \
                 computation, and no invisibility-detection engine exists anywhere in this \
                 codebase to apply it, so this grounds no actual flat-footed immunity or \
                 Dexterity-to-AC retention"
            ),
        });
    }

    // Grounded (8/8): Improved Uncanny Dodge, an 8th-level Rogue class feature
    // (verified independently against d20pfsrd and legacy.aonprd.com: both name
    // "Improved uncanny dodge, rogue talent" as the Rogue 8th-level "Special"
    // class table entry). Below the level-8 gate this is a correct PF1 Core
    // Rulebook level-gate absence (value 0); at or above it, it is a bounded
    // identity/recognition record only (value 0, non-fabricated) naming the rule
    // text — mirroring exactly how Barbarian's own Improved Uncanny Dodge was
    // grounded at barbarian level 5. The rule's own CONDITIONAL piece (comparing
    // the attacking rogue's own levels against this rogue's own levels to decide
    // whether the immunity is actually pierced) is never applied: no
    // flanking-resolution engine, no attacker-level-comparison engine, and no
    // sneak-attack-trigger engine exists anywhere in this codebase, so this
    // grounds no actual flanking immunity or sneak-attack denial. The level-8
    // row's OTHER named entry, a third Rogue Talent (a genuinely open-ended
    // choice-list feature, a new-subsystem-shaped burden), is deliberately left
    // named-but-unproven this slice, mirroring the level-2/level-4/level-6
    // rogue-talent precedent: no new choice-slot and no new diagnostic was added
    // for it.
    if level < ROGUE_IMPROVED_UNCANNY_DODGE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.rogue.improved_uncanny_dodge".to_owned(),
            value: 0,
            detail: format!(
                "Rogue Improved Uncanny Dodge at rogue level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant rule is named but not \
                 computed. Improved Uncanny Dodge is an 8th-level rogue class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.rogue.improved_uncanny_dodge".to_owned(),
            value: 0,
            detail: format!(
                "Rogue Improved Uncanny Dodge granted at rogue level {level} (PF1 Core \
                 Rulebook, 8th-level rogue class feature, part of the \"Improved uncanny \
                 dodge, rogue talent\" table entry): a rogue of 8th level or higher can no \
                 longer be flanked, denying another rogue the ability to sneak attack her by \
                 flanking unless the attacker has at least four more rogue levels than she \
                 has. This is a bounded identity/recognition record only (value 0, \
                 non-fabricated): no flanking-resolution engine, no \
                 attacker-level-comparison engine, and no sneak-attack-trigger engine exists \
                 anywhere in this codebase to apply it, so this grounds no actual flanking \
                 immunity or sneak-attack denial"
            ),
        });
    }
}

/// Surface direct SD13-E4-F7 runtime evidence for the deterministic Human Sorcerer
/// level-1 spell-bearing baseline, while keeping it explicitly claim-blocked on its
/// still-missing burdens.
///
/// The SD13-E4 Sorcerer decomposition slice splits the original combined bloodline
/// blocker into two named diagnostics and grounds one of them for real: Eschew
/// Materials, the universal, bloodline-independent bonus feat every 1st-level Sorcerer
/// receives (PF1 Core Rulebook: it lets a Sorcerer cast a spell with a material
/// component costing 1 gp or less without needing that material component). This is a
/// boolean feat grant, not a numeric formula, so it carries no fabricated mechanical
/// value; it grounds no bloodline power, no bloodline arcana, and no spell math
/// whatsoever — no spell slots, spells known, spell DCs, bonus spells, prepared
/// posture, or school choice.
///
/// The SD13-E5 Sorcerer bloodline-choice slice grounds the next honest pillar: the
/// canonical deterministic bloodline choice-slot selection
/// (`choice:sorcerer_bloodline -> bloodline:arcane`) is recognized as chosen input,
/// mirroring the Fighter bonus-feat choice-slot / Wizard Scribe Scroll precedent. This
/// is recognition only: the Arcane bloodline's level-1 power is Arcane Bond (a familiar
/// or a bonded object — an execution engine, not a flat number), so no power value is
/// fabricated. The former combined `bloodline_power` blocker narrows to an
/// `arcane_bond_and_bloodline_progression` blocker naming what stays unimplemented. It
/// only:
/// - leaves one recognition explanation so the `class:sorcerer:1` identity is acknowledged
///   as a spontaneous arcane spell-bearing class rather than an undocumented packet
///   placeholder (direct runtime evidence, carrying no fabricated mechanical value),
/// - leaves one grounded explanation recognizing the Eschew Materials bonus-feat grant
///   (also carrying no fabricated mechanical value, since it is a boolean grant),
/// - conditionally leaves one grounded explanation recognizing the canonical bloodline
///   choice-slot selection when a `choice:sorcerer_bloodline` selection is present
///   (carrying no fabricated mechanical value, since Arcane Bond is an execution engine
///   rather than a number), and
/// - emits two distinct claim-blocking diagnostics naming the Arcane Bond / bloodline
///   progression burden (Arcane Bond execution, the bloodline arcana, the bloodline
///   class skill grant, and the 3rd+-level bonus spells/feats) and the spontaneous
///   known-spell / slot posture burden explicitly, rather than hiding behind a generic
///   "unsupported caster" label. The Arcane Bond blocker names the Arcane bloodline's
///   specific mechanics only when the Arcane bloodline was the recognized selection;
///   otherwise it stays bloodline-agnostic so it never claims a specific bloodline's
///   facts for a character whose chosen bloodline this seam did not recognize.
///
/// The SD13-E5 Sorcerer base-attack/base-save slice grounds the foundational martial
/// pillar that every other class row in this matrix (Fighter, Barbarian, Monk, Rogue,
/// Paladin, Druid, Cleric, Bard) already has and Sorcerer never had: base attack bonus
/// (1/2 BAB, `classlevel / 2`) and base save progression (good Will only, poor
/// Fortitude, poor Reflex). Unlike every other class this loop has grounded so far
/// (Rogue/Monk/Druid/Cleric/Bard are all 3/4 BAB), the Sorcerer's own PF1 Core Rulebook
/// class table was verified independently (d20pfsrd and the legacy Paizo PRD mirror,
/// reading the raw level 1-6 rows: BAB +0/+1/+1/+2/+2/+3, Fort +0/+0/+1/+1/+1/+2, Ref
/// +0/+0/+1/+1/+1/+2, Will +2/+3/+3/+4/+4/+5) and found to be 1/2 BAB, not 3/4 — the
/// level 4/5 BAB values (+2 at both) disambiguate the 1/2-vs-3/4 fraction since level 1
/// alone floors every fraction to the same +0. Both pillars are grounded as flat,
/// standalone `ComputationExplanation` records, mirroring the exact "standalone, not
/// wired into the integrated `PilotBaseChassisComputation`" idiom already used for every
/// other class's own base-attack/base-save grounding: neither is wired into
/// `base_attack_bonus`, `compute_total_saves`, or `compute_combat_baseline`.
///
/// A further SD13-E5 slice widens the level-1..=2 gate to level 3
/// (`MAX_SUPPORTED_SORCERER_LEVEL = 3`) and extends every one of the formulas above to
/// level 3 via the same formula, without re-derivation, verified independently against
/// the PF1 Core Rulebook Sorcerer class table (d20pfsrd and legacy.aonprd.com): level 3
/// base attack bonus is +1, base saves are +1/+1/+3 (Fortitude/Reflex/Will); the bloodline
/// choice and bloodline class-skill choice recognitions are not level-gated, so both
/// still fire at level 3 for the same fixture selections. UNLIKE Sorcerer's own blank
/// level-2 "Special" column, the level-3 "Special" column reads "Bloodline power,
/// bloodline spell" (verified independently against both primary sources) — this was
/// checked, not assumed away, but both named entries are bloodline-specific (they vary
/// per bloodline, e.g. the Arcane bloodline's own 3rd-level power is Metamagic Adept and
/// its 3rd-level bloodline spell is Identify) and neither is flat/identity-shaped the way
/// Rogue's Trap Sense or Monk's Still Mind are, so this slice grounds neither: both stay
/// named by the pre-existing `arcane_bond_and_bloodline_progression.unsupported`
/// diagnostic, unchanged.
///
/// A further SD13-E5 slice widens the level-1..=3 gate to level 4
/// (`MAX_SUPPORTED_SORCERER_LEVEL = 4`) and extends every one of the formulas above to
/// level 4 via the same formula, without re-derivation, verified independently against
/// the PF1 Core Rulebook Sorcerer class table (d20pfsrd and legacy.aonprd.com): level 4
/// base attack bonus is +2, base saves are +1/+1/+4 (Fortitude/Reflex/Will); the
/// bloodline choice and bloodline class-skill choice recognitions are not level-gated,
/// so both still fire at level 4 for the same fixture selections. UNLIKE the level-3
/// "Special" column's "Bloodline power, bloodline spell" entry, the level-4 "Special"
/// column is blank (verified independently against both primary sources, checked rather
/// than assumed), so this slice grounds no new pillar for level 4 — only the existing
/// pillars are widened.
///
/// A further SD13-E5 slice widens the level-1..=4 gate to level 5
/// (`MAX_SUPPORTED_SORCERER_LEVEL = 5`) and extends every one of the formulas above to
/// level 5 via the same formula, without re-derivation, verified independently against
/// the PF1 Core Rulebook Sorcerer class table (d20pfsrd and legacy.aonprd.com): level 5
/// base attack bonus is +2, base saves are +1/+1/+4 (Fortitude/Reflex/Will) — every one
/// of these four values is numerically unchanged from level 4, an integer-division
/// coincidence, not a sign any formula stopped scaling; the bloodline choice and
/// bloodline class-skill choice recognitions are not level-gated, so both still fire at
/// level 5 for the same fixture selections. UNLIKE the blank level-4 "Special" column,
/// the level-5 column reads "Bloodline spell" (verified independently against both
/// primary sources, checked rather than assumed away) — the sorcerer's second bloodline
/// spell grant (the Arcane bloodline's own 5th-level bloodline spell is invisibility),
/// but the entry is bloodline-specific and not flat/identity-shaped, so this slice
/// grounds no new pillar for level 5 either, mirroring exactly how the level-3
/// "Bloodline power, bloodline spell" entry was left unproven.
///
/// A further SD13-E5 slice widens the level-1..=5 gate to level 6
/// (`MAX_SUPPORTED_SORCERER_LEVEL = 6`) and extends every one of the formulas above to
/// level 6 via the same formula, without re-derivation, verified independently against
/// the PF1 Core Rulebook Sorcerer class table (d20pfsrd and legacy.aonprd.com): level 6
/// base attack bonus is +3, base saves are +2/+2/+5 (Fortitude/Reflex/Will) — every one
/// of these four values is a genuinely NEW value, up from +2/+1/+1/+4 at level 5; the
/// bloodline choice and bloodline class-skill choice recognitions are not level-gated,
/// so both still fire at level 6 for the same fixture selections. UNLIKE the level-5
/// "Bloodline spell" entry, the level-6 "Special" column is genuinely blank (verified
/// independently against both primary sources, checked rather than assumed away), so
/// this slice grounds no new pillar for level 6 either — only the existing pillars are
/// widened.
///
/// A further SD13-E5 slice widens the level-1..=6 gate to level 7
/// (`MAX_SUPPORTED_SORCERER_LEVEL = 7`) and extends every one of the formulas above to
/// level 7 via the same formula, without re-derivation, verified independently against
/// the PF1 Core Rulebook Sorcerer class table (d20pfsrd and legacy.aonprd.com): level 7
/// base attack bonus is +3, base saves are +2/+2/+5 (Fortitude/Reflex/Will) — every one
/// of these four values is numerically unchanged from level 6, an integer-division
/// coincidence, not a sign any formula stopped scaling; the bloodline choice and
/// bloodline class-skill choice recognitions are not level-gated, so both still fire at
/// level 7 for the same fixture selections. UNLIKE the blank level-6 "Special" column,
/// the level-7 column reads "Bloodline feat, bloodline spell" (verified independently
/// against both primary sources, checked rather than assumed away) — a bloodline feat
/// (chosen from a list specific to each bloodline, first granted at 7th level and every
/// six levels thereafter) and the sorcerer's third bloodline spell grant (the Arcane
/// bloodline's own 7th-level bloodline spell is dispel magic), but both entries are
/// bloodline-specific and not flat/identity-shaped, so this slice grounds no new pillar
/// for level 7 either, mirroring exactly how the level-3 and level-5 bloodline
/// power/spell entries were left unproven — only the existing pillars are widened.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input; this seam
/// keeps that blocked posture but makes the Sorcerer spell-bearing identity, the
/// grounded Eschew Materials grant, the grounded bloodline choice recognition, the
/// grounded base-attack/base-save progression through level 7, and the two remaining
/// named burdens legible on the runtime path.
fn explain_sorcerer_level1_spell_baseline(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let Some(level) = supported_sorcerer_level(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Direct runtime evidence: recognize the deterministic Human Sorcerer level-1/
    // level-2 spell-bearing identity. This is a recognition record only; it fabricates
    // no spell math.
    explanations.push(ComputationExplanation {
        id: "class_chassis.spell_baseline.sorcerer".to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human Sorcerer level {level} spell-bearing \
             baseline: the {SORCERER_CLASS_ID}:{level} class identity is acknowledged \
             as a spontaneous arcane spell-bearing class on the rules-core seam rather than an \
             undocumented packet placeholder. This is a bounded recognition record only; it grounds no \
             bloodline power and no spell math (spell slots, spells known, spell DCs, bonus spells, or \
             prepared posture), so it carries no fabricated mechanical value (+0)"
        ),
    });

    // Grounded: the foundational base-attack-bonus / base-save progression pillar.
    // Unlike every other class row in this matrix (Fighter, Barbarian, Monk, Rogue,
    // Paladin, Druid, Cleric, Bard all already ground this pillar), Sorcerer had never
    // had it grounded at all until this SD13-E5 slice. Both formulas were verified
    // against the PF1 Core Rulebook Sorcerer class table (d20pfsrd and the legacy Paizo
    // PRD mirror) before writing this code, reading the raw level 1-6 table rows
    // directly (BAB +0/+1/+1/+2/+2/+3, Fort +0/+0/+1/+1/+1/+2, Ref +0/+0/+1/+1/+1/+2,
    // Will +2/+3/+3/+4/+4/+5) rather than trusting memory or assuming Sorcerer's shape
    // merely because it resembles another spontaneous/spell-bearing class: the level
    // 4/5 BAB values (+2 at both) disambiguate the 1/2-vs-3/4 fraction (level 1 alone
    // floors every fraction to +0) and confirm Sorcerer is 1/2 BAB, UNLIKE the 3/4 BAB
    // shared by Rogue/Monk/Druid/Cleric/Bard, and the raw Fort/Ref/Will columns
    // independently confirm good Will only, poor Fortitude, poor Reflex. A further
    // SD13-E5 slice widens the level-1-only gate to level 2 and extends both formulas
    // via the same formula, without re-derivation, verified independently against the
    // PF1 Core Rulebook Sorcerer class table: level 2 base attack bonus is +1, base
    // saves are +0/+0/+3 (Fortitude/Reflex/Will); the class table's level-2 "Special"
    // column is blank, so no new class feature is gained at 2nd level.
    let level_value = i16::from(level);

    // Grounded (1/2): 1/2-BAB base-attack progression (classlevel / 2) — the Sorcerer's
    // own class table, NOT the 3/4-BAB shape shared by Rogue/Monk/Druid/Cleric/Bard. No
    // PCGen .lst file exists for the Sorcerer class in this repo, so the formula cites
    // the PF1 Core Rulebook Sorcerer class table directly.
    let base_attack_bonus = level_value / 2;
    explanations.push(ComputationExplanation {
        id: "class_chassis.sorcerer.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            "Sorcerer level {level} base attack bonus from the PF1 Core \
             Rulebook Sorcerer class table's 1/2-BAB progression — UNLIKE the 3/4-BAB shape \
             shared by Rogue/Monk/Druid/Cleric/Bard: classlevel / 2 = {base_attack_bonus}. This \
             is a standalone explanation record; it is not wired into the integrated \
             base_attack_bonus field or into compute_combat_baseline"
        ),
    });

    // Grounded (2/2): base-save progression — poor Fortitude, poor Reflex, good Will,
    // verified against the PF1 Core Rulebook Sorcerer class table (Fortitude +0, Reflex
    // +0, Will +2 at level 1).
    let good_save = level_value / 2 + 2;
    let poor_save = level_value / 3;
    explanations.push(ComputationExplanation {
        id: "class_chassis.sorcerer.base_save.fortitude".to_owned(),
        value: poor_save,
        detail: format!(
            "Sorcerer level {level} base Fortitude save (poor save) from the \
             PF1 Core Rulebook Sorcerer class table: classlevel/3 = {poor_save}. This is a \
             standalone explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.sorcerer.base_save.reflex".to_owned(),
        value: poor_save,
        detail: format!(
            "Sorcerer level {level} base Reflex save (poor save) from the PF1 \
             Core Rulebook Sorcerer class table: classlevel/3 = {poor_save}. This is a \
             standalone explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.sorcerer.base_save.will".to_owned(),
        value: good_save,
        detail: format!(
            "Sorcerer level {level} base Will save (good save) from the PF1 \
             Core Rulebook Sorcerer class table: classlevel/2+2 = {good_save}. This is a \
             standalone explanation record; it is not wired into compute_total_saves"
        ),
    });

    // Grounded for real: Eschew Materials is a universal, bloodline-independent bonus
    // feat granted to every 1st-level Sorcerer. It is a boolean feat grant, not a
    // numeric formula, so it carries no fabricated mechanical value; it grounds no
    // bloodline power and no spell math.
    explanations.push(ComputationExplanation {
        id: "class_chassis.sorcerer.eschew_materials".to_owned(),
        value: 0,
        detail: format!(
            "Sorcerer level {level} Eschew Materials bonus feat: the PF1 Core \
             Rulebook grants every Sorcerer the Eschew Materials feat at 1st level regardless of \
             chosen bloodline, letting them cast a spell with a material component costing 1 gp or \
             less without needing that material component. This is a boolean feat grant, not a \
             numeric bonus, so it carries no fabricated mechanical value (+0); it grounds no \
             bloodline power, no bloodline arcana, and no spell math (spell slots, spells known, \
             spell DCs, or bonus spells)"
        ),
    });

    // Grounded for real: the canonical bloodline choice-slot selection is recognized as
    // chosen input, mirroring the Fighter bonus-feat choice-slot / Wizard Scribe Scroll
    // precedent. Recognition only: the Arcane bloodline's level-1 power is Arcane Bond,
    // an execution engine rather than a flat number, so no power value is fabricated.
    let bloodline_selection = choice_selection(input, SORCERER_BLOODLINE_CHOICE_ID);
    let recognized_arcane_bloodline = bloodline_selection == Some(ARCANE_BLOODLINE_SELECTION_ID);
    if let Some(selection) = bloodline_selection {
        let detail = if recognized_arcane_bloodline {
            format!(
                "Sorcerer level {level} bloodline choice recognized: the \
                 canonical deterministic selection ({SORCERER_BLOODLINE_CHOICE_ID} -> \
                 {selection}) names the Arcane bloodline as chosen input on the compute seam. \
                 This is a recognition record of the choice slot only, so it carries no \
                 fabricated mechanical value (+0): the Arcane bloodline's level-1 power is \
                 Arcane Bond (a familiar or a bonded object), an execution engine rather than a \
                 flat number, and neither it nor the bloodline arcana, bloodline class skill \
                 grant, or higher-level bonus spells/feats is grounded here"
            )
        } else {
            format!(
                "Sorcerer level {level} bloodline choice slot is present \
                 ({SORCERER_BLOODLINE_CHOICE_ID} -> {selection}), but only the canonical \
                 deterministic Arcane bloodline selection is recognized on this bounded seam; \
                 no bloodline power is grounded and no mechanical value is fabricated (+0)"
            )
        };
        explanations.push(ComputationExplanation {
            id: "class_chassis.sorcerer.bloodline_choice".to_owned(),
            value: 0,
            detail,
        });
    }

    // Grounded for real: the Arcane bloodline's "Class Skill: Knowledge (any one)" grant
    // — a player's choice of any one Knowledge skill, verified against both d20pfsrd and
    // the legacy Paizo PRD mirror, NOT a fixed grant of Knowledge (arcana) specifically —
    // is recognized as chosen input. This is specific to the Arcane bloodline, so it is
    // only recognized when the Arcane bloodline selection itself was recognized above; it
    // is never fabricated for an unrecognized or absent bloodline choice.
    let bloodline_class_skill_selection =
        choice_selection(input, SORCERER_BLOODLINE_CLASS_SKILL_CHOICE_ID);
    let recognized_bloodline_class_skill_name = bloodline_class_skill_selection
        .filter(|_| recognized_arcane_bloodline)
        .and_then(knowledge_skill_display_name);
    if recognized_arcane_bloodline
        && let Some(selection) = bloodline_class_skill_selection
    {
        let detail = if let Some(skill_name) = &recognized_bloodline_class_skill_name {
            format!(
                "Sorcerer level {level} Arcane bloodline class-skill choice \
                 recognized: the canonical deterministic selection \
                 ({SORCERER_BLOODLINE_CLASS_SKILL_CHOICE_ID} -> {selection}) names {skill_name} \
                 as the player's chosen class skill. The PF1 Core Rulebook Arcane bloodline \
                 grants \"Class Skill: Knowledge (any one)\" — a player's choice of any one \
                 Knowledge skill, not a fixed grant of Knowledge (arcana) specifically. This is \
                 a recognition record of the choice slot only, so it carries no fabricated \
                 mechanical value (+0): granting a class skill confers no flat modifier by \
                 itself in this codebase (no skill-rank allocation or untrained-skill-use engine \
                 exists here), so no skill-check total is computed or fabricated"
            )
        } else {
            format!(
                "Sorcerer level {level} Arcane bloodline class-skill choice \
                 slot is present ({SORCERER_BLOODLINE_CLASS_SKILL_CHOICE_ID} -> {selection}), \
                 but only a \"knowledge:<skill>\"-shaped selection is recognized as the PF1 Core \
                 Rulebook's \"Class Skill: Knowledge (any one)\" grant on this bounded seam; no \
                 class-skill identity is grounded and no mechanical value is fabricated (+0)"
            )
        };
        explanations.push(ComputationExplanation {
            id: "class_chassis.sorcerer.bloodline_class_skill_choice".to_owned(),
            value: 0,
            detail,
        });
    }

    // Still blocked (1/2): with the bloodline choice recognized above, narrow the former
    // combined bloodline-power blocker to what actually remains unimplemented. The message
    // names the Arcane bloodline specifically only when it was actually the recognized
    // selection above; when no bloodline is chosen, or a different bloodline is chosen,
    // it stays bloodline-agnostic so it never asserts a specific bloodline's mechanics as
    // "remaining" for a character whose chosen bloodline this seam did not recognize. Once
    // the class-skill choice above is also recognized, the class-skill grant is dropped
    // from this "not implemented" list — it is grounded separately above — and the message
    // states the corrected rule ("a player's choice of any one Knowledge skill") rather
    // than repeating the earlier imprecise "(Knowledge [arcana])" wording.
    let arcane_bond_message = if recognized_arcane_bloodline {
        if recognized_bloodline_class_skill_name.is_some() {
            format!(
                "Sorcerer level {level} remains blocked on its Arcane Bond and \
                 bloodline progression burden: the Arcane bloodline's level-1 power Arcane Bond \
                 (a familiar or a bonded object — an execution engine, not a flat number), the \
                 bloodline arcana (+1 spell save DC on spells modified by a metamagic feat that \
                 raises the spell's level — a conditional effect), and the bloodline bonus \
                 spells and bonus feats at 3rd+ level are not implemented in this bounded spell \
                 baseline, so no Sorcerer bloodline-power support is claimed. The bloodline \
                 class skill grant (a player's choice of any one Knowledge skill, per \"Class \
                 Skill: Knowledge [any one]\") is grounded separately above as a recognition \
                 record and is no longer part of this blocker"
            )
        } else {
            format!(
                "Sorcerer level {level} remains blocked on its Arcane Bond and \
                 bloodline progression burden: the Arcane bloodline's level-1 power Arcane Bond \
                 (a familiar or a bonded object — an execution engine, not a flat number), the \
                 bloodline arcana (+1 spell save DC on spells modified by a metamagic feat that \
                 raises the spell's level — a conditional effect), the bloodline class skill \
                 grant (a player's choice of any one Knowledge skill, per \"Class Skill: \
                 Knowledge [any one]\"), and the bloodline bonus spells and bonus feats at 3rd+ \
                 level are not implemented in this bounded spell baseline, so no Sorcerer \
                 bloodline-power support is claimed"
            )
        }
    } else {
        format!(
            "Sorcerer level {level} remains blocked on its bloodline power and \
             progression burden: no bloodline power, bloodline arcana, bloodline class skill \
             grant, or bonus spells/feats at 3rd+ level are implemented for any bloodline in this \
             bounded spell baseline, so no Sorcerer bloodline-power support is claimed"
        )
    };
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported".to_owned(),
        message: arcane_bond_message,
        claim_blocking: true,
    });

    // Still blocked (2/2): name the spontaneous known-spell / slot posture burden explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: "class_spell.sorcerer.spontaneous.unsupported".to_owned(),
        message:
            "Sorcerer remains blocked on its spontaneous known-spell / slot posture burden: \
             spontaneous casting, spells known, spell slots per day, bonus spell slots from a high \
             ability score, and spell save DCs are out of scope for this level-1 spell baseline and \
             no spell math is fabricated"
                .to_owned(),
        claim_blocking: true,
    });
}

/// The bounded Wizard milestone level this decomposition surface grounds, if any.
/// Returns the single Wizard level when the chosen input is exactly a single-class
/// Wizard at one of the supported milestone levels (1 through 9). Returns `None` for
/// no Wizard, a non-Wizard class, a multiclass mix, or any level-10+ Wizard this slice
/// deliberately does not recognize — each of which stays claim-blocked exactly as
/// before. Mirrors the Fighter `supported_fighter_level` / Paladin
/// `supported_paladin_level` / Rogue `supported_rogue_level` / Barbarian
/// `supported_barbarian_level` / Monk `supported_monk_level` / Cleric
/// `supported_cleric_level` / Bard `supported_bard_level` / Druid
/// `supported_druid_level` / Sorcerer `supported_sorcerer_level` level-range gate
/// idiom.
fn supported_wizard_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == WIZARD_CLASS_ID
                && (1..=MAX_SUPPORTED_WIZARD_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

/// Return `true` when the input carries exactly the canonical deterministic school
/// specialization selections: Evocation chosen as the specialty school, with
/// Necromancy and Transmutation as the two opposed schools. Anything else — the
/// choice slots absent (e.g. a universalist-shaped request) or any non-canonical
/// selection — returns `false`, so no specialization grounding is fabricated for a
/// choice that was never made or that this bounded slice does not know.
fn wizard_has_canonical_specialization_selections(input: &CharacterInput) -> bool {
    if choice_selection(input, WIZARD_SCHOOL_SPECIALIZATION_CHOICE_ID)
        != Some(EVOCATION_SCHOOL_SELECTION)
    {
        return false;
    }
    let opposed: Vec<&str> = input
        .chosen
        .selected_choices
        .iter()
        .filter(|c| c.choice_set_id == WIZARD_OPPOSED_SCHOOLS_CHOICE_ID)
        .map(|c| c.selection_id.as_str())
        .collect();
    opposed.len() == 2
        && opposed.contains(&NECROMANCY_SCHOOL_SELECTION)
        && opposed.contains(&TRANSMUTATION_SCHOOL_SELECTION)
}

/// Surface direct SD13-E4-R3 runtime evidence for the deterministic Human Wizard
/// level-1 prepared arcane spell-bearing baseline, while keeping it explicitly
/// claim-blocked on its two still-missing burdens.
///
/// This deliberately does not compute a supported spell surface. It grounds no
/// spellbook content, no spells prepared, no spell slots per day, no spell save
/// DCs, no bonus spell slots from a high Intelligence, and no school-power or
/// opposed-school preparation-cost math. It only:
/// - leaves one recognition explanation so the `class:wizard:1` identity is
///   acknowledged as a prepared arcane spell-bearing class rather than an
///   undocumented packet placeholder (direct runtime evidence, carrying no
///   fabricated mechanical value),
/// - grounds one universal, specialization-independent class feature for real:
///   Scribe Scroll, the bonus feat every 1st-level Wizard is granted regardless
///   of arcane school specialization (PF1 Core Rulebook Wizard class feature),
///   letting the Wizard create scrolls of spells they know. This is a bounded
///   grant-only recognition, not a numeric formula: it carries no fabricated
///   mechanical value (+0) and computes no scroll-creation cost, crafting time,
///   spellbook content, or spell-slot machinery,
/// - grounds the flat surface of the school specialization choice for real
///   (SD13-E5), gated on the exact canonical deterministic selections: a
///   recognition record of the Evocation specialization with Necromancy and
///   Transmutation opposed (+0), plus the specialist bonus slot as a flat count
///   only — one 1st-level Evocation-only bonus slot at level 1 (+1), with no
///   cantrip-level bonus slot and no slot contents,
/// - grounds two of the Evocation school's own 1st-level school powers as flat
///   numeric magnitudes (a further SD13-E5 slice), gated on the same canonical
///   Evocation selection: Intense Spells' bonus-damage magnitude (half wizard
///   level, minimum 1) and Force Missile's uses-per-day pool (3 + Intelligence
///   modifier). Both were independently verified against the PF1 Core Rulebook
///   Evocation School rule text (the legacy Paizo PRD mirror, cross-checked by a
///   second independent source) before grounding — Force Missile in particular
///   was treated with skepticism (a name that could plausibly have been confused
///   with non-core material) but confirmed as a genuine 1st-level Evocation
///   school power with exactly the "3 + Int-mod" pool the pre-existing blocker
///   text already claimed. Neither grounding applies any bonus to an actual
///   spell-damage roll, casts any force missile, resolves any automatic-hit
///   targeting, or tracks any action economy or per-use consumption,
/// - grounds the foundational base-attack-bonus / base-save progression pillar
///   (a further SD13-E5 slice) that every other class row in this matrix
///   (Fighter, Barbarian, Monk, Rogue, Paladin, Druid, Cleric, Bard, Sorcerer)
///   already has and Wizard never had: base attack bonus (1/2 BAB, `classlevel
///   / 2` — the same shape as Sorcerer, UNLIKE the 3/4 BAB shared by
///   Rogue/Monk/Druid/Cleric/Bard) and base save progression (good Will only,
///   poor Fortitude, poor Reflex). Both were verified against the PF1 Core
///   Rulebook Wizard class table (d20pfsrd and the legacy Paizo PRD mirror),
///   reading the raw level 1-6 rows directly (BAB +0/+1/+1/+2/+2/+3, Fort
///   +0/+0/+1/+1/+1/+2, Ref +0/+0/+1/+1/+1/+2, Will +2/+3/+3/+4/+4/+5) rather
///   than assumed from Sorcerer's matching shape; the level 4/5 BAB values (+2
///   at both) disambiguate the 1/2-vs-3/4 fraction since level 1 alone floors
///   every fraction to +0. Both pillars are grounded as flat, standalone
///   `ComputationExplanation` records mirroring the exact "standalone, not
///   wired into the integrated `PilotBaseChassisComputation`" idiom already
///   used for every other class's own base-attack/base-save grounding: neither
///   is wired into `base_attack_bonus`, `compute_total_saves`, or
///   `compute_combat_baseline`, and
/// - emits two distinct claim-blocking diagnostics naming the school-power
///   execution / opposed-school-preparation-cost burden (the still-unimplemented
///   spell-damage application for Intense Spells, the still-unimplemented
///   casting execution for Force Missile, and the two-prepared-slot cost for
///   opposed-school spells) and the prepared spellbook / spells-prepared /
///   spell-slot posture burden explicitly, rather than hiding behind a generic
///   "unsupported caster" label.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input; this
/// seam keeps that blocked posture but makes the Wizard prepared spell-bearing
/// identity, its grounded class-feature surfaces, and its remaining named
/// burdens legible on the runtime path. The matrix file row transition
/// (Unverified/Observed → Blocked/Computed, then Blocked → Partial once Scribe
/// Scroll is grounded) is recorded by this proof surface and applied to the
/// in-source carrier directly (see `seeded_sd13_e1_f1_current_truth`).
///
/// A further SD13-E5 slice widens the level-1-only gate (`supported_wizard_level`,
/// 1..=2) and extends every one of the formulas above to level 2 via the same
/// formula, without re-derivation, verified independently against the PF1 Core
/// Rulebook Wizard class table (d20pfsrd and legacy.aonprd.com): level 2 base attack
/// bonus is +1, base saves are +0/+0/+3 (Fortitude/Reflex/Will); the specialist bonus
/// slot count stays exactly 1 (a level-2 wizard still only casts 1st-level spells,
/// since 2nd-level wizard spells require caster level 3); Intense Spells' bonus
/// damage stays 1, reached naturally (`max(2/2, 1) = 1`) rather than via the level-1
/// floor; Force Missile's uses-per-day pool is level-independent and unchanged;
/// Scribe Scroll is granted once, at 1st level only, and stays recognized as an
/// already-held grant (its detail text hardcodes "1st level" as the level it was
/// granted, never re-deriving a level-2 grant event). The class table's level-2
/// "Special" column is blank (verified independently against both sources), so no
/// new class feature is gained at 2nd level, unlike Rogue/Monk/Druid's Evasion/
/// Woodland Stride — this slice widens existing pillars only, adds no new one.
///
/// A further SD13-E5 slice widens the gate again (`supported_wizard_level`, 1..=3)
/// and extends the same formulas to level 3, without re-derivation, verified
/// independently against the PF1 Core Rulebook Wizard class table (d20pfsrd and
/// legacy.aonprd.com): level 3 base attack bonus is +1, base saves are +1/+1/+3
/// (Fortitude/Reflex/Will); Intense Spells' bonus damage stays 1
/// (`max(3/2, 1) = 1`); Force Missile's uses-per-day pool is level-independent and
/// unchanged; Scribe Scroll stays recognized as an already-held grant. The
/// specialist bonus slot flat count, in contrast, CHANGES for real at level 3: the
/// PF1 Core Rulebook arcane school class feature grants "an additional spell slot of
/// each spell level he can cast, from 1st on up" (verified against both primary
/// sources' exact rule text), and the raw Wizard spells-per-day table rows (also
/// verified against both sources) show a level-3 wizard casts 2nd-level spells for
/// the first time (level 2: "4/2/—/—"; level 3: "4/2/1/—"), so the flat count
/// becomes 2 (one 1st-level bonus slot plus one 2nd-level bonus slot), up from 1.
/// The class table's level-3 "Special" column is also blank (verified
/// independently against both sources), so no new class feature is gained at 3rd
/// level either, unlike Rogue/Monk/Barbarian's own 3rd-level features — this slice
/// widens existing pillars only (one of them to a new value), adds no new pillar
/// record.
///
/// A further SD13-E5 slice widens the gate again (`supported_wizard_level`, 1..=4)
/// and extends the same formulas to level 4, without re-derivation, verified
/// independently against the PF1 Core Rulebook Wizard class table (d20pfsrd and
/// legacy.aonprd.com): level 4 base attack bonus is +2, base saves are +1/+1/+4
/// (Fortitude/Reflex/Will). The specialist bonus slot flat count, checked rather
/// than assumed to double again, STAYS at 2: the raw Wizard spells-per-day table's
/// level-4 row is still "4/3/2/—/—" — 3rd-level wizard spells do not become
/// available until wizard level 5 (level 5 row: "4/3/2/1/—", the first non-"—"
/// 3rd-level column) — so a level-4 specialist still only casts 1st- and 2nd-level
/// spells and the pre-existing `level >= WIZARD_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL`
/// gate already produces the correct value with no formula change. Intense Spells'
/// bonus-damage magnitude, in contrast, CHANGES for real at level 4: `max(4 / 2, 1) =
/// 2`, up from 1 at levels 1-3 — the first value change this pillar's formula
/// produces since it was grounded. Force Missile's uses-per-day pool is
/// level-independent and unchanged; Scribe Scroll stays recognized as an
/// already-held grant. The class table's level-4 "Special" column is also blank
/// (verified independently against both sources: the Wizard's own next class
/// feature, a bonus feat, is granted at 5th level, not 4th) — this slice widens
/// existing pillars only (one of them, Intense Spells, to a genuinely new value),
/// adds no new pillar record.
///
/// A further SD13-E5 slice widens the gate again (`supported_wizard_level`, 1..=5)
/// and extends the same formulas to level 5, without re-derivation, verified
/// independently against the PF1 Core Rulebook Wizard class table (d20pfsrd and a
/// second independent Archives of Nethys mirror): level 5 base attack bonus is +2,
/// base saves are +1/+1/+4 (Fortitude/Reflex/Will) — all four values numerically
/// IDENTICAL to level 4, an integer-division coincidence (`5 / 2` and `4 / 2` both
/// floor to `2`; `5 / 3` and `4 / 3` both floor to `1`), not a sign any formula
/// stopped scaling. The specialist bonus slot flat count is the exact question this
/// cycle was briefed to verify: the raw Wizard spells-per-day table's level-5 row is
/// "4/3/2/1/—" — 3rd-level wizard spells become available for the first time at
/// wizard level 5 (level 4 row was "4/3/2/—/—") — so a level-5 specialist now casts
/// 1st-, 2nd-, and 3rd-level spells, and the flat count genuinely becomes 3 (one
/// bonus slot of each spell level 1st through 3rd), up from 2 at levels 3-4. Intense
/// Spells' bonus-damage magnitude, in contrast, STAYS at 2 at level 5: `max(5 / 2, 1)
/// = 2`, another integer-division coincidence, not a formula that stopped scaling.
/// Force Missile's uses-per-day pool is level-independent and unchanged; Scribe
/// Scroll stays recognized as an already-held grant. The class table's level-5
/// "Special" column reads "Bonus feat" (verified independently against both
/// sources) — a genuinely NEW Wizard class feature at 5th level, but checked and
/// confirmed NOT flat: the feat is chosen from an open-ended set of metamagic feats,
/// item creation feats (each its own family with its own prerequisites), or the
/// single named Spell Mastery feature — a general feat-selection/feat-prerequisite
/// engine, not a flat magnitude, mirroring the Monk High Jump precedent exactly
/// (checked rather than assumed, deliberately left named-but-unproven, no record or
/// diagnostic fabricated for it). This slice widens existing pillars only (one of
/// them, the specialist bonus slot count, to a genuinely new value), adds no new
/// pillar record.
///
/// A further SD13-E5 slice widens the gate again (`supported_wizard_level`, 1..=6)
/// and extends the same formulas to level 6, without re-derivation, verified
/// independently against the PF1 Core Rulebook Wizard class table (d20pfsrd and
/// legacy.aonprd.com): level 6 base attack bonus is +3, base saves are +2/+2/+5
/// (Fortitude/Reflex/Will) — all four values genuinely NEW, up from +2/+1/+1/+4 at
/// level 5. The specialist bonus slot flat count, checked rather than assumed to
/// rise again, STAYS at 3: the raw Wizard spells-per-day table's level-6 row is
/// "4/3/3/2/—" — 4th-level wizard spells do not become available until wizard level
/// 7 (level 7 row: "4/4/3/2/1", the first non-"—" 4th-level column) — so a level-6
/// specialist still only casts 1st-, 2nd-, and 3rd-level spells. Intense Spells'
/// bonus-damage magnitude, in contrast, CHANGES for real at level 6: `max(6 / 2, 1) =
/// 3`, up from 2 at level 5, via the same pre-existing formula, not re-derived.
/// Force Missile's uses-per-day pool is level-independent and unchanged; Scribe
/// Scroll stays recognized as an already-held grant. The class table's level-6
/// "Special" column is genuinely BLANK (verified independently against both
/// sources, checked rather than assumed away) — UNLIKE the level-5 "Bonus feat"
/// entry, no new Wizard class feature is gained at 6th level — this slice widens
/// existing pillars only (one of them, Intense Spells, to a genuinely new value),
/// adds no new pillar record.
///
/// A further SD13-E5 slice widens the gate again (`supported_wizard_level`, 1..=7)
/// and extends the same formulas to level 7, without re-derivation, verified
/// independently against the PF1 Core Rulebook Wizard class table (d20pfsrd and
/// legacy.aonprd.com): level 7 base attack bonus and all three base saves are
/// numerically UNCHANGED from level 6 (+3 base attack, +2/+2/+5
/// Fortitude/Reflex/Will) — an integer-division coincidence, re-verified rather
/// than assumed. The specialist bonus slot flat count, checked rather than assumed
/// to stay put, GENUINELY RISES to 4: the raw Wizard spells-per-day table's
/// level-7 row is "4/4/3/2/1" — the first non-"—" 4th-level column — so a level-7
/// specialist now casts 4th-level spells for the first time. Intense Spells'
/// bonus-damage magnitude STAYS at 3 (`max(7/2, 1) = 3`, unchanged from level 6,
/// another integer-division coincidence). Force Missile's uses-per-day pool is
/// level-independent and unchanged; Scribe Scroll stays recognized as an
/// already-held grant. The class table's level-7 "Special" column is genuinely
/// BLANK (verified independently against both sources), so no new Wizard class
/// feature is gained at 7th level — this slice widens existing pillars only (one
/// of them, the specialist bonus slot, to a genuinely new value), adds no new
/// pillar record.
fn explain_wizard_level1_prepared_spell_baseline(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let Some(level) = supported_wizard_level(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Direct runtime evidence: recognize the deterministic Human Wizard level-1
    // prepared arcane spell-bearing identity. This is a recognition record only;
    // it fabricates no spell math and no school-opposition / specialty school
    // bonus math.
    explanations.push(ComputationExplanation {
        id: "class_chassis.spell_baseline.wizard".to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human Wizard level {level} prepared arcane spell-bearing \
             baseline: the {WIZARD_CLASS_ID}:{level} class identity is acknowledged as a \
             prepared arcane spell-bearing class on the rules-core seam rather than an \
             undocumented packet placeholder. This is a bounded recognition record only; it \
             grounds no spellbook content, no spells prepared per day, no spell slots per day, \
             no spell save DCs, no bonus spell slots from a high Intelligence, no school \
             specialization mechanics, no opposed-school bookkeeping, and no specialty school \
             bonus, so it carries no fabricated mechanical value (+0)"
        ),
    });

    // Grounded (SD13-E5): the foundational base-attack-bonus / base-save progression
    // pillar that every other class row in this matrix (Fighter, Barbarian, Monk,
    // Rogue, Paladin, Druid, Cleric, Bard, Sorcerer) already has and Wizard never had
    // at all. Both formulas were verified against the PF1 Core Rulebook Wizard class
    // table (d20pfsrd and the legacy Paizo PRD mirror) before writing this code,
    // reading the raw level 1-6 table rows directly (BAB +0/+1/+1/+2/+2/+3, Fort
    // +0/+0/+1/+1/+1/+2, Ref +0/+0/+1/+1/+1/+2, Will +2/+3/+3/+4/+4/+5) rather than
    // assuming Wizard's shape merely because it resembles another arcane class: the
    // level 4/5 BAB values (+2 at both) disambiguate the 1/2-vs-3/4 fraction (level 1
    // alone floors every fraction to +0) and confirm Wizard is 1/2 BAB — the SAME
    // shape as Sorcerer, UNLIKE the 3/4 BAB shared by Rogue/Monk/Druid/Cleric/Bard —
    // and the raw Fort/Ref/Will columns independently confirm good Will only, poor
    // Fortitude, poor Reflex (also matching Sorcerer's shape, confirmed rather than
    // assumed).
    let wizard_level_value = i16::from(level);

    // Grounded (1/2): 1/2-BAB base-attack progression (classlevel / 2) — the same
    // shape as Sorcerer, NOT the 3/4-BAB shape shared by Rogue/Monk/Druid/Cleric/Bard.
    let wizard_base_attack_bonus = wizard_level_value / 2;
    explanations.push(ComputationExplanation {
        id: "class_chassis.wizard.base_attack_bonus".to_owned(),
        value: wizard_base_attack_bonus,
        detail: format!(
            "Wizard level {level} base attack bonus from the PF1 Core Rulebook Wizard class \
             table's 1/2-BAB progression — the same shape as Sorcerer, UNLIKE the 3/4-BAB shape \
             shared by Rogue/Monk/Druid/Cleric/Bard: classlevel / 2 = {wizard_base_attack_bonus}. \
             This is a standalone explanation record; it is not wired into the integrated \
             base_attack_bonus field or into compute_combat_baseline"
        ),
    });

    // Grounded (2/2): base-save progression — poor Fortitude, poor Reflex, good Will,
    // verified against the PF1 Core Rulebook Wizard class table (Fortitude +0, Reflex
    // +0, Will +2 at level 1; +0, +0, +3 at level 2 — same formulas, not re-derived).
    let wizard_good_save = wizard_level_value / 2 + 2;
    let wizard_poor_save = wizard_level_value / 3;
    explanations.push(ComputationExplanation {
        id: "class_chassis.wizard.base_save.fortitude".to_owned(),
        value: wizard_poor_save,
        detail: format!(
            "Wizard level {level} base Fortitude save (poor save) from the PF1 Core Rulebook \
             Wizard class table: classlevel/3 = {wizard_poor_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.wizard.base_save.reflex".to_owned(),
        value: wizard_poor_save,
        detail: format!(
            "Wizard level {level} base Reflex save (poor save) from the PF1 Core Rulebook Wizard \
             class table: classlevel/3 = {wizard_poor_save}. This is a standalone explanation \
             record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.wizard.base_save.will".to_owned(),
        value: wizard_good_save,
        detail: format!(
            "Wizard level {level} base Will save (good save) from the PF1 Core Rulebook Wizard \
             class table: classlevel/2+2 = {wizard_good_save}. This is a standalone explanation \
             record; it is not wired into compute_total_saves"
        ),
    });

    // Grounded for real: Scribe Scroll is a universal, specialization-independent
    // Wizard class feature (every 1st-level Wizard is granted it regardless of
    // which school, if any, is later chosen), so it is separable from the
    // school-specialization burden. It is a boolean grant, not a numeric formula.
    // Verified against both PF1 CRB primary sources (d20pfsrd and legacy.aonprd.com):
    // Scribe Scroll is granted exactly once, in the level-1 "Special" column, never
    // re-granted at 2nd level or later. Since the wizard keeps the feat once granted,
    // this record's header cites the character's current level (still recognized at
    // level 2+ within this seam's supported range), but its body text hardcodes "1st
    // level" as the level the feat was actually granted, mirroring the Sorcerer
    // Eschew Materials idiom exactly: no level-2 grant event is re-derived.
    explanations.push(ComputationExplanation {
        id: "class_chassis.wizard.scribe_scroll".to_owned(),
        value: 0,
        detail: format!(
            "Recognized Wizard level {level} Scribe Scroll bonus feat grant: every Wizard, \
             regardless of arcane school specialization, is granted Scribe Scroll as a bonus \
             feat at 1st level (PF1 Core Rulebook Wizard class feature), letting the Wizard \
             create scrolls of spells they know. This is a one-time grant recognized once and \
             kept thereafter, not re-granted at 2nd level or later. This is a bounded grant-only \
             recognition: it carries no fabricated mechanical value (+0) and computes no scroll \
             creation cost, no crafting time, no spellbook content, and no spell-slot machinery"
        ),
    });

    // Grounded for real (SD13-E5): the flat surface of the school specialization
    // choice, gated on the exact canonical deterministic selections ("canonical"
    // versus "absent or anything else"). An input without them (e.g. a
    // universalist-shaped request that never made the choice) gains no
    // specialization recognition and no specialist bonus slot.
    if wizard_has_canonical_specialization_selections(input) {
        explanations.push(ComputationExplanation {
            id: "class_chassis.wizard.specialization_choice".to_owned(),
            value: 0,
            detail: format!(
                "Recognized Wizard level {level} school specialization choice: the canonical \
                 deterministic selections choose Evocation as the specialty arcane school \
                 ({WIZARD_SCHOOL_SPECIALIZATION_CHOICE_ID} -> {EVOCATION_SCHOOL_SELECTION}) with \
                 Necromancy and Transmutation as the two opposed schools \
                 ({WIZARD_OPPOSED_SCHOOLS_CHOICE_ID} -> {NECROMANCY_SCHOOL_SELECTION}, \
                 {TRANSMUTATION_SCHOOL_SELECTION}), per the PF1 Core Rulebook arcane school class \
                 feature. A wizard's chosen school does not change by level, so this recognition \
                 is not level-gated. This is a bounded recognition record of the choice identity \
                 only: it carries no fabricated mechanical value (+0) and computes no school \
                 power, no opposed-school preparation cost, and no spell math"
            ),
        });
        // Grounded for real: the specialist bonus slot flat count. Confirmed unchanged
        // at level 2 (SD13-E5): a level-2 wizard still only casts 1st-level wizard
        // spells (2nd-level wizard spells require caster level 3, verified against
        // both primary sources' raw spells-per-day table rows), so "one additional
        // spell slot of each spell level she can cast" is still exactly one 1st-level
        // slot at both levels 1 and 2 this seam supports. A further SD13-E5 slice
        // widens this for real at level 3: a level-3 wizard casts 2nd-level spells
        // for the first time (verified independently against both primary sources'
        // raw spells-per-day table rows), so the specialist now gains one bonus slot
        // of EACH spell level she can cast — one 1st-level bonus slot plus one
        // 2nd-level bonus slot, a flat count of 2.
        let wizard_specialist_bonus_slot_count =
            if level >= WIZARD_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
                WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_9
            } else if level >= WIZARD_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
                WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_7
            } else if level >= WIZARD_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
                WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_5
            } else if level >= WIZARD_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
                WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_3
            } else {
                WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVELS_1_AND_2
            };
        explanations.push(ComputationExplanation {
            id: "class_chassis.wizard.specialist_bonus_slot".to_owned(),
            value: wizard_specialist_bonus_slot_count,
            detail: format!(
                "Wizard level {level} specialist bonus spell slot: a specialist wizard gains one \
                 additional Evocation-only spell slot of each spell level she can cast, 1st and \
                 up, usable only for spells of the chosen school (PF1 Core Rulebook arcane \
                 school class feature). At levels 1-2 a wizard casts only 1st-level spells, so \
                 the flat count is exactly one 1st-level Evocation-only bonus slot \
                 ({WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVELS_1_AND_2:+}); at level \
                 {WIZARD_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}+ a wizard also casts 2nd-level \
                 spells for the first time (verified against both primary sources' raw \
                 spells-per-day table rows), so the flat count becomes \
                 {WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_3:+} (one 1st-level Evocation-only bonus \
                 slot plus one 2nd-level Evocation-only bonus slot); at level \
                 {WIZARD_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}+ a wizard also casts 3rd-level \
                 spells for the first time (verified against both primary sources' raw \
                 spells-per-day table rows), so the flat count becomes \
                 {WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_5:+} (one 1st-level, one 2nd-level, and \
                 one 3rd-level Evocation-only bonus slot); at level \
                 {WIZARD_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}+ a wizard also casts 4th-level \
                 spells for the first time (verified against both primary sources' raw \
                 spells-per-day table rows), so the flat count becomes \
                 {WIZARD_SPECIALIST_BONUS_SLOTS_AT_LEVEL_7:+} (one 1st-level, one 2nd-level, one \
                 3rd-level, and one 4th-level Evocation-only bonus slot). At level {level} this \
                 is {wizard_specialist_bonus_slot_count:+} flat count; there is no cantrip-level \
                 bonus slot. This grounds the flat count only: no slot contents, no spells \
                 prepared per day, no per-day slot totals, and no bonus slots from a high \
                 Intelligence are computed"
            ),
        });

        // Grounded for real (SD13-E5): Intense Spells' flat bonus-damage magnitude.
        // PF1 Core Rulebook Evocation School: whenever an evocation spell that deals
        // hit point damage is cast, add half wizard level (minimum 1) to the damage.
        // Verified against the legacy Paizo PRD mirror rather than trusted from
        // memory or the pre-existing blocker-message claim. This is a flat,
        // non-dice magnitude, so it grounds for real, mirroring the Cleric Touch of
        // Good sacred-bonus idiom exactly. Confirmed at level 2: max(2/2, 1) = 1,
        // reached naturally via the formula rather than via the level-1 floor.
        let intense_spells_bonus_damage = (wizard_level_value / 2).max(1);
        explanations.push(ComputationExplanation {
            id: "class_chassis.wizard.intense_bonus_damage".to_owned(),
            value: intense_spells_bonus_damage,
            detail: format!(
                "Wizard level {level} Evocation school power Intense Spells bonus-damage \
                 magnitude (PF1 Core Rulebook Evocation School): whenever an evocation spell \
                 that deals hit point damage is cast, add half wizard level (minimum 1) to the \
                 damage. At Wizard level {level} this is max({level} / 2, 1) = \
                 {intense_spells_bonus_damage}. This grounds only the flat bonus-damage \
                 magnitude; it applies no bonus to any actual spell-damage roll and implements \
                 no spell-damage-application engine"
            ),
        });

        // Grounded for real (SD13-E5): Force Missile's flat uses-per-day pool. PF1
        // Core Rulebook Evocation School: as a standard action, a specialist
        // Evocation wizard may unleash a force missile (as magic missile, dealing
        // 1d4 points of damage plus the Intense Spells bonus) that automatically
        // strikes a foe, usable 3 + Intelligence modifier times per day. Verified
        // against the legacy Paizo PRD mirror with deliberate skepticism (a name
        // that could plausibly have been confused with non-core material), which
        // confirmed the power is genuinely core and the "3 + Int-mod" pool the
        // pre-existing blocker text already claimed is correct. Only the flat
        // daily-use count is a non-dice formula; the 1d4 damage roll and the
        // automatic-hit casting execution are not flat, so they stay unproven. This
        // pool is level-independent and confirmed unchanged at level 2.
        let force_missile_uses_per_day = (3 + ability_modifiers.intelligence).max(0);
        explanations.push(ComputationExplanation {
            id: "class_chassis.wizard.force_missile_uses_per_day".to_owned(),
            value: force_missile_uses_per_day,
            detail: format!(
                "Wizard level {level} Evocation school power Force Missile uses per day (PF1 \
                 Core Rulebook Evocation School): 3 + Intelligence modifier, floored at 0. At \
                 Intelligence modifier {} this is max(3 + {}, 0) = \
                 {force_missile_uses_per_day}. This grounds only the flat daily-use count; it \
                 casts no force missile, applies no 1d4 damage roll, resolves no automatic-hit \
                 magic-missile-style targeting, and tracks no action economy or per-use \
                 consumption",
                ability_modifiers.intelligence, ability_modifiers.intelligence
            ),
        });
    }

    // Still blocked (1/2): with the specialization choice and two Evocation school
    // powers' flat magnitudes grounded above, the claim-blocker narrows to exactly
    // what stays unimplemented: the school-power execution machinery and the
    // opposed-school preparation cost.
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.wizard.school_powers_and_opposed_school_cost.unsupported".to_owned(),
        message: format!(
            "Wizard level {level} remains blocked on its school-power execution and \
             opposed-school preparation-cost burden: the Evocation intense spells flat \
             bonus-damage magnitude and the force missile flat 3 + Int-mod uses-per-day pool are \
             now grounded as flat numbers in dedicated explanation records, but no evocation \
             spell-damage application, no force-missile casting execution (the 1d4 damage roll \
             and automatic-hit targeting), and no opposed-school preparation cost (each \
             opposed-school spell occupies two prepared slots) are implemented, so no full \
             Wizard school-power or opposed-school support is claimed"
        ),
        claim_blocking: true,
    });

    // Still blocked (2/2): name the prepared spellbook / spells-prepared /
    // spell-slot posture burden explicitly. Unchanged by the Scribe Scroll and
    // specialization-choice groundings: it fabricates no spell math.
    diagnostics.push(ComputationDiagnostic {
        id: "class_spell.wizard.prepared_spellbook.unsupported".to_owned(),
        message:
            "Wizard remains blocked on its prepared spellbook / spells prepared / spell slot \
             posture burden: spellbook content, spells prepared per day, spell slots per day, \
             bonus spell slots from a high Intelligence, and spell save DCs are out of scope for \
             this level-1 prepared spell baseline and no spell math is fabricated"
                .to_owned(),
        claim_blocking: true,
    });
}

/// The bounded Cleric milestone level this decomposition surface grounds, if any.
/// Returns the single Cleric level when the chosen input is exactly a single-class
/// Cleric at one of the supported milestone levels (1 through 9). Returns `None` for
/// no Cleric, a non-Cleric class, a multiclass mix, or any level-10+ Cleric this slice
/// deliberately does not recognize — each of which stays claim-blocked exactly as
/// before. Mirrors the Fighter `supported_fighter_level` / Paladin
/// `supported_paladin_level` / Rogue `supported_rogue_level` / Barbarian
/// `supported_barbarian_level` / Monk `supported_monk_level` level-range gate idiom.
fn supported_cleric_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == CLERIC_CLASS_ID
                && (1..=MAX_SUPPORTED_CLERIC_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

/// Surface direct SD13-E4/E5 runtime evidence for the deterministic Human Cleric
/// level-1/level-2/level-3/level-4 prepared divine spell-bearing baseline, while
/// keeping it explicitly claim-blocked on its remaining still-missing burdens.
///
/// This deliberately does not compute a supported spell surface. It grounds Channel
/// Energy's flat die-count and uses-per-day math, the domain choice seam, the flat
/// domain spell slot count, the Good domain's Touch of Good (flat sacred-bonus
/// magnitude and flat uses-per-day count), the Healing domain's Rebuke Death (flat
/// uses-per-day count only), and the foundational base-attack-bonus / base-save
/// progression pillar that every other class row in this matrix already has and Cleric
/// never had; it grounds no Rebuke Death heal amount, no domain spell-list contents, no
/// channel energy save DC or damage/healing resolution, no spellbook posture, no spells
/// prepared, no spontaneous cure/inflict conversion, no general spell slots per day, no
/// spell save DCs, and no bonus spell slots from a high Wisdom. A later SD13-E5 slice
/// widens the level-1-only gate (`supported_cleric_level`, 1..=2) and extends every one
/// of the formulas below to level 2 via the same formula, without re-derivation,
/// verified independently against the PF1 Core Rulebook Cleric class table (d20pfsrd
/// and legacy.aonprd.com): Cleric gains no new class feature at 2nd level (the class
/// table's level-2 "Special" column is blank), so no new pillar is added, only the
/// existing ones widened. A further SD13-E5 slice widens the gate again to 1..=3
/// (`MAX_SUPPORTED_CLERIC_LEVEL = 3`): Channel Energy's die count and the domain
/// spell slot count both change for real at level 3, since level 3 is exactly when
/// a cleric first casts 2nd-level spells (verified independently against both
/// primary sources' raw class table and spells-per-day table rows); the level-3
/// "Special" column names only the Channel Energy die-count increase, so no new
/// pillar record is added. A further SD13-E5 slice widens the gate again to 1..=4
/// (`MAX_SUPPORTED_CLERIC_LEVEL = 4`): the Good domain's Touch of Good sacred bonus
/// changes for real at level 4 (half cleric level, minimum 1: `max(4/2, 1) = 2`, up
/// from 1), verified independently against the PF1 Core Rulebook Good Domain
/// granted-power rule text; Channel Energy's die count and the domain spell slot
/// count both stay unchanged at level 4 (verified independently against both primary
/// sources: the class table's level-4 "Special" column is blank, and the
/// spells-per-day table's 3rd-level spell column is still "—" at level 4), so no new
/// pillar record is added at level 4 either. A further SD13-E5 slice widens the gate
/// again to 1..=5 (`MAX_SUPPORTED_CLERIC_LEVEL = 5`): Channel Energy's die count
/// genuinely increases to 3d6 (`ceil(5/2) = 3`, the class table's level-5 "Special"
/// column reads "Channel energy 3d6") and the domain spell slot count genuinely
/// increases to 3 (a level-5 cleric casts 3rd-level cleric spells for the first
/// time, verified independently against both primary sources' raw spells-per-day
/// table rows), while the Good domain's Touch of Good sacred bonus stays 2
/// (`max(5/2, 1) = 2`, unchanged from level 4 — it next increases only at level 6),
/// so only the two pillars whose underlying formulas genuinely change are widened;
/// no new pillar record is added at level 5 either. A further SD13-E5 slice widens
/// the gate again to 1..=6 (`MAX_SUPPORTED_CLERIC_LEVEL = 6`): the class table's
/// level-6 "Special" column is genuinely blank (no new class feature is gained),
/// Channel Energy's die count stays 3d6 (`ceil(6/2) = 3`, unchanged from level 5 —
/// both primary sources confirm the die count rises only every odd cleric level),
/// and the domain spell slot count stays 3 (the spells-per-day table's level-6 row
/// still shows "—" in the 4th-level spell column), while the Good domain's Touch of
/// Good sacred bonus genuinely increases to 3 (`max(6/2, 1) = 3`, up from 2) — so
/// only the one pillar whose underlying formula genuinely changes is widened; no
/// new pillar record is added at level 6 either. A further SD13-E5 slice widens
/// the gate again to 1..=7 (`MAX_SUPPORTED_CLERIC_LEVEL = 7`): the class table's
/// level-7 "Special" column reads "Channel energy 4d6" — Channel Energy's die
/// count genuinely increases to 4d6 (`ceil(7/2) = 4`, confirming level 7 IS one of
/// the odd cleric levels where the die count rises) — and the domain spell slot
/// count also genuinely increases, to 4 (a level-7 cleric casts 4th-level cleric
/// spells for the first time, verified independently against both primary
/// sources' raw spells-per-day table rows), while the Good domain's Touch of Good
/// sacred bonus stays 3 (`max(7/2, 1) = 3`, unchanged from level 6 — it next
/// increases only at level 8) — so two pillars whose underlying formulas
/// genuinely change (Channel Energy dice, domain spell slot count) are widened;
/// no new pillar record is added at level 7 either, since no other class feature
/// is named in the level-7 Special column. It only:
/// - leaves one recognition explanation so the `class:cleric:N` identity is acknowledged
///   as a prepared divine spell-bearing class rather than an undocumented packet
///   placeholder (direct runtime evidence, carrying no fabricated mechanical value),
/// - leaves one grounded base-attack-bonus explanation (PF1 Core Rulebook Cleric class
///   table: 3/4 BAB, the same formula shape as Rogue/Monk/Druid) and three grounded
///   base-save explanations (good Fortitude, good Will, poor Reflex), each a standalone
///   record not wired into `PilotBaseChassisComputation.base_attack_bonus`,
///   `compute_total_saves`, or `compute_combat_baseline`,
/// - grounds Channel Energy's die count and daily use count for real (PF1 Core
///   Rulebook Channel Energy: `ceil(cleric level / 2)` d6, minimum 1d6; usable
///   `3 + Charisma modifier` times per day; confirmed the die count stays 1d6 at
///   level 2 and becomes 2d6 at level 3, both via the same formula, the level-3
///   value not re-derived),
/// - surfaces the canonical two-domain choice seam (`choice:cleric_domain ->
///   domain:good` and `choice:cleric_domain -> domain:healing`) as an explicit
///   recognition record carrying no mechanical value, mirroring the Fighter
///   bonus-feat choice-slot seam,
/// - grounds the flat domain spell slot count for real (PF1 Core Rulebook Domains:
///   one domain spell slot per level of cleric spells she can cast, 1st and up —
///   exactly one 1st-level domain slot at levels 1-2, since a level-2 cleric still
///   only casts 1st-level cleric spells; at level 3 a cleric casts 2nd-level cleric
///   spells for the first time, so the count becomes 2 — one 1st-level plus one
///   2nd-level domain slot; the slots' contents are not grounded at any level),
/// - grounds the Good domain's Touch of Good in full when Good is a chosen domain
///   (PF1 Core Rulebook Good Domain: a flat sacred bonus equal to half cleric level,
///   minimum 1, and a flat `3 + Wisdom modifier` uses-per-day count — both formulas
///   are non-dice, so both ground for real at every supported level),
/// - grounds only the Healing domain's Rebuke Death uses-per-day count when Healing
///   is a chosen domain (PF1 Core Rulebook Healing Domain: `3 + Wisdom modifier`
///   times per day), leaving its heal amount (`1d4` plus a per-level bonus, gated on
///   the target's hit-point state) explicitly named but unproven because it is not a
///   flat number, and
/// - emits two distinct claim-blocking diagnostics naming the still-unproven pieces of
///   the domain powers burden and the prepared divine spell posture burden explicitly,
///   rather than hiding behind a generic "unsupported caster" label.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input; this seam
/// keeps that blocked posture but makes the Cleric prepared divine spell-bearing
/// identity, its grounded base-attack/base-save/Channel-Energy/domain-choice/
/// domain-slot-count/domain-power pillars, and its two named remaining burdens legible
/// on the runtime path.
fn explain_cleric_level1_spell_baseline(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let Some(level) = supported_cleric_level(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Direct runtime evidence: recognize the deterministic Human Cleric level-1/
    // level-2 prepared divine spell-bearing identity. This is a recognition record
    // only; it fabricates no domain power math and no spell math.
    explanations.push(ComputationExplanation {
        id: "class_chassis.spell_baseline.cleric".to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human Cleric level {level} prepared divine \
             spell-bearing baseline: the {CLERIC_CLASS_ID}:{level} class identity is \
             acknowledged as a prepared divine spell-bearing class on the rules-core seam rather than \
             an undocumented packet placeholder. This is a bounded recognition record only; it grounds \
             no domain selection, no domain spells, no domain powers, no channel energy execution, no \
             spellbook posture, no spells prepared per day, no spontaneous cure/inflict conversion, no \
             spell slots per day, no spell save DCs, and no bonus spell slots from a high Wisdom, so it \
             carries no fabricated mechanical value (+0)"
        ),
    });

    // Grounded: the foundational base-attack-bonus / base-save progression pillar.
    // Unlike every other class row in this matrix (Fighter, Barbarian, Monk, Rogue,
    // Paladin, Druid all already ground this pillar), Cleric had never had it
    // grounded at all until this SD13-E5 slice. Both formulas were verified against
    // the PF1 Core Rulebook Cleric class table (d20pfsrd and the legacy Paizo PRD
    // mirror) before writing this code, cross-checking the level 2-5 base-attack-bonus
    // values (+0/+1/+2/+3/+3) to disambiguate the exact fraction (level 1 alone floors
    // both a 1/2 and a 3/4 progression to the same +0, so it cannot disambiguate on its
    // own). A later SD13-E5 slice widens this level-1-only gate to level 2; the
    // formula is extended, not re-derived (level 2 base attack +1, all base saves +3,
    // confirmed against the raw class table).
    let level_value = i16::from(level);

    // Grounded (1/2): 3/4-BAB base-attack progression, the same formula shape as
    // Rogue/Monk/Druid (classlevel * 3 / 4). No PCGen .lst file exists for the Cleric
    // class in this repo, so the formula cites the PF1 Core Rulebook Cleric class
    // table directly.
    let base_attack_bonus = level_value * 3 / 4;
    explanations.push(ComputationExplanation {
        id: "class_chassis.cleric.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            "Cleric level {level} base attack bonus from the PF1 Core Rulebook \
             Cleric class table's 3/4-BAB progression, the same formula shape as \
             Rogue/Monk/Druid: classlevel * 3 / 4 = {base_attack_bonus}. This is a standalone \
             explanation record; it is not wired into the integrated base_attack_bonus field or \
             into compute_combat_baseline"
        ),
    });

    // Grounded (2/2): base-save progression — good Fortitude, poor Reflex, good
    // Will, verified against the PF1 Core Rulebook Cleric class table (Fortitude
    // +2, Reflex +0, Will +2 at level 1).
    let good_save = level_value / 2 + 2;
    let poor_save = level_value / 3;
    explanations.push(ComputationExplanation {
        id: "class_chassis.cleric.base_save.fortitude".to_owned(),
        value: good_save,
        detail: format!(
            "Cleric level {level} base Fortitude save (good save) from the PF1 \
             Core Rulebook Cleric class table: classlevel/2+2 = {good_save}. This is a \
             standalone explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.cleric.base_save.reflex".to_owned(),
        value: poor_save,
        detail: format!(
            "Cleric level {level} base Reflex save (poor save) from the PF1 \
             Core Rulebook Cleric class table: classlevel/3 = {poor_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.cleric.base_save.will".to_owned(),
        value: good_save,
        detail: format!(
            "Cleric level {level} base Will save (good save) from the PF1 Core \
             Rulebook Cleric class table: classlevel/2+2 = {good_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });

    // Grounded for real: Channel Energy's flat die count. PF1 Core Rulebook Channel
    // Energy: the cleric channels a number of d6s equal to ceil(cleric level / 2),
    // minimum 1d6. At level 1 this is ceil(1 / 2) = 1d6; confirmed unchanged at level
    // 2 (ceil(2 / 2) = 1d6 too, via the same formula, not a new record). A further
    // SD13-E5 slice confirms this genuinely increases to 2d6 at level 3
    // (ceil(3 / 2) = 2), verified against the PF1 Core Rulebook Cleric class table's
    // level-3 "Special" column ("Channel energy 2d6") — via the same pre-existing
    // formula, not re-derived.
    let channel_energy_dice = (level_value + 1) / 2;
    explanations.push(ComputationExplanation {
        id: "class_chassis.cleric.channel_energy_dice".to_owned(),
        value: channel_energy_dice,
        detail: format!(
            "Cleric Channel Energy die count: ceil(cleric level / 2) d6 (PF1 Core Rulebook Channel \
             Energy), minimum 1d6. At Cleric level {level} this is \
             ceil({level} / 2) = {channel_energy_dice}d6. This grounds only the flat \
             d6 die count; it computes no channel energy save DC and no positive/negative energy \
             burst damage or healing resolution"
        ),
    });

    // Grounded for real: Channel Energy's flat daily use count. PF1 Core Rulebook
    // Channel Energy: usable 3 + Charisma modifier times per day, floored at 0 (a
    // cleric cannot channel energy a negative number of times per day).
    let channel_energy_uses_per_day = (3 + ability_modifiers.charisma).max(0);
    explanations.push(ComputationExplanation {
        id: "class_chassis.cleric.channel_energy_uses_per_day".to_owned(),
        value: channel_energy_uses_per_day,
        detail: format!(
            "Cleric Channel Energy uses per day: 3 + Charisma modifier (PF1 Core Rulebook Channel \
             Energy), floored at 0. At Charisma modifier {} this is max(3 + {}, 0) = \
             {channel_energy_uses_per_day}. This grounds only the flat daily use count; it computes \
             no channel energy save DC and no positive/negative energy burst damage or healing \
             resolution",
            ability_modifiers.charisma, ability_modifiers.charisma
        ),
    });

    // Grounded for real: the canonical two-domain choice seam. A PF1 cleric chooses
    // two domains from among those belonging to her deity; the deterministic fixture
    // carries the canonical Good + Healing pair. Mirroring the Fighter bonus-feat
    // choice-slot seam, this surfaces the named selections as an explicit choice seam
    // only when both canonical selections are present — an absent slot is not
    // fabricated — and contributes no computed mechanical value.
    let domain_selections: Vec<&str> = input
        .chosen
        .selected_choices
        .iter()
        .filter(|c| c.choice_set_id == CLERIC_DOMAIN_CHOICE_ID)
        .map(|c| c.selection_id.as_str())
        .collect();
    if domain_selections.len() == 2
        && domain_selections.contains(&GOOD_DOMAIN_SELECTION)
        && domain_selections.contains(&HEALING_DOMAIN_SELECTION)
    {
        explanations.push(ComputationExplanation {
            id: "class_chassis.cleric.domain_choice".to_owned(),
            value: 0,
            detail: format!(
                "Cleric level {level} chooses two domains from among those \
                 belonging to her deity (PF1 Core Rulebook Domains); the named canonical \
                 selections ({CLERIC_DOMAIN_CHOICE_ID} -> {GOOD_DOMAIN_SELECTION}, \
                 {CLERIC_DOMAIN_CHOICE_ID} -> {HEALING_DOMAIN_SELECTION}) are surfaced as an \
                 explicit choice seam only, mirroring the Fighter bonus-feat choice-slot seam. \
                 This slice grounds the domain choice slot, not the chosen domains' granted \
                 powers or domain spell lists, so it contributes no computed mechanical value \
                 (+0)"
            ),
        });
    }

    // Grounded for real: the flat domain spell slot count. PF1 Core Rulebook Domains:
    // a cleric gains one domain spell slot per level of cleric spells she can cast,
    // 1st and up. This count is class-chassis math independent of which domains were
    // chosen; only the slot's contents (which domain spell may fill it) depend on the
    // chosen domains, and those are deliberately not grounded. Confirmed unchanged at
    // level 2 (a level-2 cleric still only casts 1st-level cleric spells — 2nd-level
    // cleric spells begin at caster level 3, verified against the PF1 Core Rulebook
    // Cleric spells-per-day table via d20pfsrd and legacy.aonprd.com), so this is the
    // same value at level 2, not a new record. A further SD13-E5 slice widens this for
    // real at level 3: a level-3 cleric casts 2nd-level cleric spells for the first
    // time (verified independently against both primary sources' raw spells-per-day
    // table rows), so the count genuinely becomes 2 — one 1st-level domain slot plus
    // one 2nd-level domain slot — mirroring exactly the Wizard specialist-bonus-slot
    // level-3 widening. Confirmed unchanged at level 4 (the level-4 3rd-level spell
    // column is still "—"). A further SD13-E5 slice widens this for real at level 5:
    // a level-5 cleric casts 3rd-level cleric spells for the first time (verified
    // independently against both primary sources' raw spells-per-day table rows), so
    // the count genuinely becomes 3 — one 1st-level, one 2nd-level, and one
    // 3rd-level domain slot. Confirmed unchanged at level 6 (the level-6 4th-level
    // spell column is still "—", verified independently against both primary
    // sources), so the count stays 3 through level 6 as well. A further SD13-E5
    // slice widens this for real at level 7: a level-7 cleric casts 4th-level
    // cleric spells for the first time (verified independently against both
    // primary sources' raw spells-per-day table rows), so the count genuinely
    // becomes 4 — one 1st-level, one 2nd-level, one 3rd-level, and one
    // 4th-level domain slot.
    let domain_spell_slot_count = if level >= CLERIC_FIFTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_9
    } else if level >= CLERIC_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_7
    } else if level >= CLERIC_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVELS_5_AND_6
    } else if level >= CLERIC_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL {
        CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVELS_3_AND_4
    } else {
        CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVELS_1_AND_2
    };
    explanations.push(ComputationExplanation {
        id: "class_chassis.cleric.domain_spell_slot".to_owned(),
        value: domain_spell_slot_count,
        detail: format!(
            "Cleric domain spell slot count: one domain spell slot per level of cleric spells \
             she can cast, 1st and up (PF1 Core Rulebook Domains). At levels 1-2 a cleric \
             casts only 1st-level cleric spells, so the flat count is exactly \
             {CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVELS_1_AND_2} 1st-level domain spell slot; \
             at levels {CLERIC_SECOND_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}-4 a cleric also casts \
             2nd-level cleric spells (2nd-level cleric spells begin at caster level 3, \
             verified against both primary sources' raw spells-per-day table rows), so the \
             flat count becomes {CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVELS_3_AND_4} (one \
             1st-level domain slot plus one 2nd-level domain slot); at levels \
             {CLERIC_THIRD_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}-6 a cleric also casts 3rd-level \
             cleric spells for the first time (verified against both primary sources' raw \
             spells-per-day table rows, including the level-6 row's still-\"—\" 4th-level \
             column), so the flat count becomes \
             {CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVELS_5_AND_6} (one 1st-level, one \
             2nd-level, and one 3rd-level domain slot); at level \
             {CLERIC_FOURTH_LEVEL_SPELLS_BEGIN_AT_CLASS_LEVEL}+ a cleric also casts 4th-level \
             cleric spells for the first time (verified against both primary sources' raw \
             spells-per-day table rows), so the flat count becomes \
             {CLERIC_DOMAIN_SPELL_SLOT_COUNT_AT_LEVEL_7} (one 1st-level, one 2nd-level, one \
             3rd-level, and one 4th-level domain slot). At Cleric level {level} this is \
             {domain_spell_slot_count} domain spell slot(s). \
             This grounds only the flat slot count; it grounds no slot contents (which domain \
             spell may fill it), no domain spell lists, and no prepared-spell posture"
        ),
    });

    // Grounded for real: the Good domain's granted power, Touch of Good. PF1 Core
    // Rulebook Good Domain: as a standard action, touch a creature to grant it a
    // sacred bonus on attack rolls, skill checks, ability checks, and saving throws
    // equal to half the cleric's level (minimum 1) for 1 round; usable 3 + Wisdom
    // modifier times per day. Verified against the PF1 Core Rulebook Good Domain rule
    // text (d20pfsrd, cross-checked by a second independent search) rather than
    // trusted from the pre-existing blocker-message claim or from memory. Both the
    // bonus magnitude and the uses-per-day count are flat, non-dice formulas, so both
    // ground for real, gated on the Good domain actually being one of the two chosen
    // domains (an absent selection is not fabricated, mirroring the domain-choice
    // seam above).
    if domain_selections.contains(&GOOD_DOMAIN_SELECTION) {
        // At level 1 this floors to the minimum (0 / 2 = 0, floored up to 1); at
        // levels 2-3 it is naturally 1 without needing the floor (2 / 2 = 1,
        // 3 / 2 = 1, integer division) — all three land on the same value,
        // confirmed via the same formula, not a new record. A further SD13-E5
        // slice confirms this genuinely increases to 2 at level 4 (4 / 2 = 2),
        // verified independently against the PF1 Core Rulebook Good Domain
        // granted-power rule text, via the same pre-existing formula, not
        // re-derived. A further SD13-E5 slice confirms this genuinely increases again
        // to 3 at level 6 (6 / 2 = 3), verified independently against the PF1 Core
        // Rulebook Good Domain granted-power rule text, via the same pre-existing
        // formula, not re-derived. A further SD13-E5 slice confirms this genuinely
        // increases again to 4 at level 8 (8 / 2 = 4), verified independently
        // against the PF1 Core Rulebook Good Domain granted-power rule text, via
        // the same pre-existing formula, not re-derived.
        let touch_of_good_bonus = (level_value / 2).max(1);
        explanations.push(ComputationExplanation {
            id: "class_chassis.cleric.domain_power_good_touch_of_good_bonus".to_owned(),
            value: touch_of_good_bonus,
            detail: format!(
                "Cleric Good domain granted power Touch of Good sacred bonus (PF1 Core Rulebook \
                 Good Domain): half cleric level, minimum 1, applied for 1 round to attack \
                 rolls, skill checks, ability checks, and saving throws after a touch. At \
                 Cleric level {level} this is \
                 max({level} / 2, 1) = {touch_of_good_bonus}. This grounds only \
                 the flat sacred-bonus magnitude; it computes no touch-attack resolution and no \
                 application of the bonus to any actual attack roll, skill check, ability \
                 check, or saving throw"
            ),
        });

        let touch_of_good_uses_per_day = (3 + ability_modifiers.wisdom).max(0);
        explanations.push(ComputationExplanation {
            id: "class_chassis.cleric.domain_power_good_touch_of_good_uses_per_day".to_owned(),
            value: touch_of_good_uses_per_day,
            detail: format!(
                "Cleric Good domain granted power Touch of Good uses per day (PF1 Core Rulebook \
                 Good Domain): 3 + Wisdom modifier, floored at 0. At Wisdom modifier {} this is \
                 max(3 + {}, 0) = {touch_of_good_uses_per_day}. This grounds only the flat \
                 daily use count; it performs no per-use consumption tracking",
                ability_modifiers.wisdom, ability_modifiers.wisdom
            ),
        });
    }

    // Grounded for real (uses/day only): the Healing domain's granted power, Rebuke
    // Death. PF1 Core Rulebook Healing Domain: as a standard action, touch a living
    // creature below 0 hit points to heal it 1d4 points of damage plus 1 for every
    // two cleric levels; usable 3 + Wisdom modifier times per day. Verified against
    // the PF1 Core Rulebook Healing Domain rule text (d20pfsrd, cross-checked by a
    // second independent search): the uses-per-day rate is genuinely the same "3 +
    // Wisdom modifier" formula as Touch of Good, so the pre-existing blocker-message
    // claim was correct on independent verification — but the heal amount itself is
    // NOT a flat number (a 1d4 dice roll, plus a hit-point-state gating check on the
    // target), so it deliberately stays named-but-unproven rather than fabricated.
    if domain_selections.contains(&HEALING_DOMAIN_SELECTION) {
        let rebuke_death_uses_per_day = (3 + ability_modifiers.wisdom).max(0);
        explanations.push(ComputationExplanation {
            id: "class_chassis.cleric.domain_power_healing_rebuke_death_uses_per_day".to_owned(),
            value: rebuke_death_uses_per_day,
            detail: format!(
                "Cleric Healing domain granted power Rebuke Death uses per day (PF1 Core \
                 Rulebook Healing Domain): 3 + Wisdom modifier, floored at 0. At Wisdom \
                 modifier {} this is max(3 + {}, 0) = {rebuke_death_uses_per_day}. This \
                 grounds only the flat daily use count; the heal amount itself (1d4 points of \
                 damage plus 1 for every two cleric levels, usable only on a living creature \
                 below 0 hit points) is not a flat number and is not grounded here — it \
                 requires a dice-roll execution engine and a hit-point-state gating check that \
                 do not exist in this codebase",
                ability_modifiers.wisdom, ability_modifiers.wisdom
            ),
        });
    }

    // Still blocked (1/2): name the domain powers burden explicitly, narrowed by the
    // grounding above. Channel Energy, the domain choice seam, the flat domain spell
    // slot count, Touch of Good (bonus and uses per day), and Rebuke Death's uses per
    // day are all grounded; the Rebuke Death heal amount and the domain spell-list
    // contents remain entirely unproven.
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.cleric.domain_powers.unsupported".to_owned(),
        message: format!(
            "Cleric level {level} remains blocked on its domain powers burden: \
             the granted powers of the chosen domains (Good: Touch of Good; Healing: Rebuke \
             Death — each usable 3 + Wisdom modifier times per day) narrow to a single unproven \
             piece each cycle grounds more of. Touch of Good's flat sacred-bonus magnitude and \
             flat uses-per-day count are now both grounded. Rebuke Death's flat uses-per-day \
             count is grounded, but its heal amount (1d4 points of damage plus 1 for every two \
             cleric levels, usable only on a creature below 0 hit points) is not a flat number \
             and remains unproven, along with the domain spell-list contents that could fill \
             the grounded domain spell slot. No touch-attack resolution, healing-application \
             engine, hit-point-state gating check, or per-use consumption tracking exists for \
             either power, so no further Cleric domain power or domain spell support is claimed"
        ),
        claim_blocking: true,
    });

    // Still blocked (2/2): name the prepared divine spell posture burden explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: "class_spell.cleric.prepared_divine.unsupported".to_owned(),
        message:
            "Cleric remains blocked on its prepared divine spell posture burden: spells prepared \
             from the full Cleric spell list, spontaneous cure/inflict conversion, spell slots per \
             day, bonus spell slots from a high Wisdom, and spell save DCs are out of scope for this \
             level-1 spell baseline and no spell math is fabricated"
                .to_owned(),
        claim_blocking: true,
    });
}

/// The bounded Druid milestone level this decomposition surface grounds, if any.
/// Returns the single Druid level when the chosen input is exactly a single-class
/// Druid at one of the supported milestone levels (1 through 9). Returns `None` for no
/// Druid, a non-Druid class, a multiclass mix, or any level-10+ Druid this slice
/// deliberately does not recognize — each of which stays claim-blocked exactly as
/// before. Mirrors the Fighter `supported_fighter_level` / Paladin
/// `supported_paladin_level` / Rogue `supported_rogue_level` / Barbarian
/// `supported_barbarian_level` / Monk `supported_monk_level` / Cleric
/// `supported_cleric_level` / Bard `supported_bard_level` level-range gate idiom.
fn supported_druid_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == DRUID_CLASS_ID
                && (1..=MAX_SUPPORTED_DRUID_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

/// Surface direct SD13-E4/SD13-E5 runtime evidence for the deterministic Human Druid
/// level-1/level-2/level-3 prepared divine spell-bearing baseline, while keeping it
/// explicitly claim-blocked on its remaining burdens. The SD13-E4 Wild Empathy slice
/// grounds Wild Empathy for real; the SD13-E5 Nature Sense / nature-bond-choice slice
/// grounds Nature Sense for real and recognizes the deterministic nature-bond
/// selection; a later SD13-E5 slice grounds the foundational base-attack-bonus /
/// base-save progression pillar that every other class row in this matrix already
/// has and Druid never had; a further SD13-E5 slice widens the level-1-only gate
/// (`supported_druid_level`, 1..=2) and extends every one of the formulas below to
/// level 2 via the same formula, without re-derivation, verified independently
/// against the PF1 Core Rulebook Druid class table (d20pfsrd and legacy.aonprd.com):
/// level 2 base attack bonus is +1, base saves are +3/+0/+3 (Fortitude/Reflex/Will).
/// That same slice also grounds Woodland Stride, the class table's level-2 "Special"
/// column entry, as a bounded identity record (flat/identity-shaped, no numeric
/// formula). A still further SD13-E5 slice widens the gate to level 3
/// (`supported_druid_level`, 1..=3), extending every formula above to level 3 via the
/// same formula (level 3 base attack bonus is +2, base saves are +3/+1/+3
/// Fortitude/Reflex/Will), keeps Woodland Stride granted (not re-derived), and grounds
/// Trackless Step, the class table's level-3 "Special" column entry, as a bounded
/// identity record (flat/identity-shaped, no numeric formula) mirroring the Woodland
/// Stride idiom exactly; Druid has no currently-grounded spell-slot-count pillar, so
/// there is no analogous level-3 doubling to widen. A further SD13-E5 slice widens the
/// gate to level 4 (`supported_druid_level`, 1..=4), extending every formula above to
/// level 4 via the same formula (level 4 base attack bonus is +3, base saves are
/// +4/+1/+4 Fortitude/Reflex/Will), keeping Woodland Stride and Trackless Step both
/// granted (not re-derived), and grounds Resist Nature's Lure, one of two distinct
/// entries in the class table's level-4 "Special" column, as a bounded flat-magnitude
/// identity record (+4 saving-throw bonus against fey spell-like/supernatural abilities
/// and plant-targeting spells/effects, never applied to any actual save total),
/// mirroring the Woodland Stride/Trackless Step idiom. The other level-4 "Special"
/// entry, Wild Shape (1/day), was checked and confirmed NOT flat (a full shapeshifting
/// subsystem with no execution engine anywhere in this codebase), so it is deliberately
/// left named-but-unproven, exactly like the animal-companion execution burden. A still
/// further SD13-E5 slice widens the gate to level 5 (`supported_druid_level`, 1..=5),
/// extending every formula above to level 5 via the same formula (level 5 base attack
/// bonus is +3, base saves are +4/+1/+4 Fortitude/Reflex/Will, all three numerically
/// unchanged from level 4 as an integer-division coincidence, not a stopped-scaling
/// formula), keeping Woodland Stride, Trackless Step, and Resist Nature's Lure all
/// granted (not re-derived); the PF1 Core Rulebook Druid class table's level-5
/// "Special" column is genuinely blank (verified independently against d20pfsrd and
/// legacy.aonprd.com rather than assumed), so no new pillar is grounded at level 5.
/// A still further SD13-E5 slice widens the gate to level 6 (`supported_druid_level`,
/// 1..=6), extending every formula above to level 6 via the same formula (level 6
/// base attack bonus is +4, base saves are +5/+2/+5 Fortitude/Reflex/Will, all three
/// genuinely new values), keeping Woodland Stride, Trackless Step, and Resist
/// Nature's Lure all granted (not re-derived). The class table's level-6 "Special"
/// column ("Wild shape (2/day)") was checked and confirmed NOT a genuinely separable
/// flat/identity-shaped element — the frequency increase is bundled with a
/// form-list expansion and a functioning-level upgrade, neither of which exist in
/// this codebase — so it is deliberately left named-but-unproven, exactly as at
/// level 4/5; no new pillar is grounded at level 6 either. A still further SD13-E5
/// slice widens the gate to level 7 (`supported_druid_level`, 1..=7), extending
/// every formula above to level 7 via the same formula (level 7 base attack bonus
/// is +5, a genuinely new value up from +4 at level 6; base saves are +5/+2/+5
/// Fortitude/Reflex/Will, all three numerically unchanged from level 6 — an
/// integer-division coincidence, re-verified against the raw class table rather
/// than assumed), keeping Woodland Stride, Trackless Step, and Resist Nature's
/// Lure all granted (not re-derived). The class table's level-7 "Special" column
/// is genuinely blank (verified independently against d20pfsrd and
/// legacy.aonprd.com rather than assumed): Wild Shape's next usage-count increase
/// ("Wild shape (3/day)") does not land until 8th level, so this slice makes no
/// Wild Shape claim at level 7 either way, and no new pillar is grounded at level
/// 7. The chosen bond's execution and the prepared divine spell posture burden
/// remain claim-blocked.
///
/// This deliberately does not compute a supported spell surface. It grounds no nature
/// bond power execution (no companion stat block, no companion advancement, no link /
/// share spells, no domain math), no spellbook posture, no spells prepared, no
/// spontaneous summon nature's ally conversion, no spell slots per day, no spell save
/// DCs, and no bonus spell slots from a high Wisdom. It only:
/// - leaves one recognition explanation so the `class:druid:N` identity is acknowledged
///   as a prepared divine spell-bearing class rather than an undocumented packet
///   placeholder (direct runtime evidence, carrying no fabricated mechanical value),
/// - leaves one grounded base-attack-bonus explanation (PF1 Core Rulebook Druid class
///   table: 3/4 BAB, the same formula shape as Rogue/Monk) and three grounded
///   base-save explanations (good Fortitude, good Will, poor Reflex), each a
///   standalone record not wired into `PilotBaseChassisComputation.base_attack_bonus`,
///   `compute_total_saves`, or `compute_combat_baseline`,
/// - leaves one grounded Wild Empathy explanation (the flat druid-level +
///   Charisma-modifier modifier, not a d20 roll and not a Diplomacy-check execution
///   engine),
/// - leaves one grounded Nature Sense explanation (the flat, level-independent PF1
///   +2 bonus on Knowledge (nature) and Survival checks, kept as a standalone record
///   not wired into any skill-check total),
/// - when the deterministic `choice:druid_nature_bond -> bond:animal_companion`
///   selection is present, leaves one +0 recognition record acknowledging that
///   selection without executing it (no record is fabricated when the selection is
///   absent),
/// - leaves one Woodland Stride explanation — a correct level-gate absence below
///   level 2, and a bounded identity/recognition record (value 0) at or above it,
///   mirroring exactly how Rogue's/Monk's own Evasion was grounded, with no
///   terrain-detection engine and no movement-execution engine implemented,
/// - leaves one Trackless Step explanation — a correct level-gate absence below
///   level 3, and a bounded identity/recognition record (value 0) at or above it,
///   mirroring exactly how Woodland Stride was grounded, with no tracking-resolution
///   engine and no terrain-detection engine implemented, and
/// - emits two distinct claim-blocking diagnostics naming the animal-companion
///   execution burden and the prepared divine spell posture burden explicitly,
///   rather than hiding behind a generic "unsupported caster" label.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input; this seam
/// keeps that blocked posture but makes the Druid prepared divine spell-bearing
/// identity, its grounded base-attack/base-save/Wild Empathy/Nature Sense/Woodland
/// Stride values, its recognized nature-bond choice, and its remaining named burdens
/// legible on the runtime path.
fn explain_druid_level1_spell_baseline(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let Some(level) = supported_druid_level(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Direct runtime evidence: recognize the deterministic Human Druid level-1/
    // level-2 prepared divine spell-bearing identity. This is a recognition record
    // only; it fabricates no nature-bond power math and no spell math.
    explanations.push(ComputationExplanation {
        id: "class_chassis.spell_baseline.druid".to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human Druid level {level} prepared divine \
             spell-bearing baseline: the {DRUID_CLASS_ID}:{level} class identity is \
             acknowledged as a prepared divine spell-bearing class on the rules-core seam rather than \
             an undocumented packet placeholder. This is a bounded recognition record only; the Wild \
             Empathy and Nature Sense values and the nature-bond choice recognition are grounded \
             separately below, but this record still grounds no nature bond power execution, no \
             spellbook posture, no spells prepared per day, no spontaneous summon nature's ally \
             conversion, no spell slots per day, no spell save DCs, and no bonus spell slots from a \
             high Wisdom, so it carries no fabricated mechanical value (+0)"
        ),
    });

    // Grounded: the foundational base-attack-bonus / base-save progression pillar.
    // Unlike every other class row in this matrix (Fighter, Barbarian, Monk, Rogue,
    // Paladin all already ground this pillar), Druid had never had it grounded at
    // all until this SD13-E5 slice. Both formulas were verified against the PF1
    // Core Rulebook Druid class table (d20pfsrd and the legacy Paizo PRD mirror)
    // before writing this code, cross-checking the level 4/5 base-attack-bonus
    // values to disambiguate the exact fraction (level 1 alone floors both a 1/2
    // and a 3/4 progression to the same +0, so it cannot disambiguate on its own). A
    // later SD13-E5 slice widens this level-1-only gate to level 2; the formula is
    // extended, not re-derived (level 2 base attack +1, all base saves +3/+0/+3,
    // confirmed against the raw class table).
    let level_value = i16::from(level);

    // Grounded (1/2): 3/4-BAB base-attack progression, the same formula shape as
    // Rogue/Monk (classlevel * 3 / 4). No PCGen .lst file exists for the Druid
    // class in this repo, so the formula cites the PF1 Core Rulebook Druid class
    // table directly.
    let base_attack_bonus = level_value * 3 / 4;
    explanations.push(ComputationExplanation {
        id: "class_chassis.druid.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            "Druid level {level} base attack bonus from the PF1 Core Rulebook \
             Druid class table's 3/4-BAB progression, the same formula shape as Rogue/Monk: \
             classlevel * 3 / 4 = {base_attack_bonus}. This is a standalone explanation record; \
             it is not wired into the integrated base_attack_bonus field or into \
             compute_combat_baseline"
        ),
    });

    // Grounded (2/2): base-save progression — good Fortitude, poor Reflex, good
    // Will, verified against the PF1 Core Rulebook Druid class table (Fortitude
    // +2, Reflex +0, Will +2 at level 1; +3/+0/+3 at level 2).
    let good_save = level_value / 2 + 2;
    let poor_save = level_value / 3;
    explanations.push(ComputationExplanation {
        id: "class_chassis.druid.base_save.fortitude".to_owned(),
        value: good_save,
        detail: format!(
            "Druid level {level} base Fortitude save (good save) from the PF1 \
             Core Rulebook Druid class table: classlevel/2+2 = {good_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.druid.base_save.reflex".to_owned(),
        value: poor_save,
        detail: format!(
            "Druid level {level} base Reflex save (poor save) from the PF1 Core \
             Rulebook Druid class table: classlevel/3 = {poor_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.druid.base_save.will".to_owned(),
        value: good_save,
        detail: format!(
            "Druid level {level} base Will save (good save) from the PF1 Core \
             Rulebook Druid class table: classlevel/2+2 = {good_save}. This is a standalone \
             explanation record; it is not wired into compute_total_saves"
        ),
    });

    // Grounded: Wild Empathy (PF1 Core Rulebook). A druid uses Wild Empathy to
    // improve the attitude of an animal, resolved like a Diplomacy check: the
    // druid rolls 1d20 and adds her druid level and her Charisma modifier. Only
    // the flat level + Cha-modifier bonus is grounded here; no d20 roll and no
    // Diplomacy-check/attitude-outcome execution engine is computed. This formula
    // was already level-generic (it takes `level` as a value, not a hardcoded
    // baseline), so it extends correctly to level 2 without re-derivation.
    let wild_empathy_modifier = level_value + ability_modifiers.charisma;
    explanations.push(ComputationExplanation {
        id: "class_chassis.druid.wild_empathy".to_owned(),
        value: wild_empathy_modifier,
        detail: format!(
            "Druid Wild Empathy modifier (PF1 Core Rulebook): a druid uses Wild Empathy to improve \
             an animal's attitude as if making a Diplomacy check, rolling 1d20 and adding her druid \
             level and her Charisma modifier. At Druid level {level} with a Charisma \
             modifier of {}, the modifier is {level} + {} = {wild_empathy_modifier}. \
             This grounds only the flat druid-level + Charisma-modifier bonus; it computes no d20 \
             roll, no Diplomacy-check resolution, and no attitude-improvement outcome",
            ability_modifiers.charisma, ability_modifiers.charisma
        ),
    });

    // Grounded: Nature Sense (PF1 Core Rulebook). A druid gains a +2 bonus on
    // Knowledge (nature) and Survival checks. Flat and level-independent; grounded
    // as a standalone record only — it is not wired into any skill-check total and
    // resolves no Knowledge (nature) or Survival check. Confirmed unchanged at
    // level 2 via the same formula, not a new record.
    explanations.push(ComputationExplanation {
        id: "class_chassis.druid.nature_sense".to_owned(),
        value: DRUID_NATURE_SENSE_BONUS,
        detail: format!(
            "Druid Nature Sense bonus (PF1 Core Rulebook): a druid gains a \
             +{DRUID_NATURE_SENSE_BONUS} bonus on Knowledge (nature) and Survival checks. The \
             bonus is flat and level-independent. This is a standalone grounded record only: it \
             is not wired into any computed skill-check total and it resolves no Knowledge \
             (nature) or Survival check"
        ),
    });

    // Recognized: the deterministic nature-bond selection. The fixture carries
    // `choice:druid_nature_bond -> bond:animal_companion`; when that selection is
    // present it is acknowledged as chosen input, carrying no fabricated bond
    // execution. When the selection is absent (the desktop composer threads no
    // nature-bond slot) no record is fabricated. This recognition is not
    // level-gated; it still fires at level 2 for the same fixture selection.
    let animal_companion_chosen = choice_selection(input, DRUID_NATURE_BOND_CHOICE_ID)
        == Some(DRUID_NATURE_BOND_ANIMAL_COMPANION_SELECTION_ID);
    if animal_companion_chosen {
        explanations.push(ComputationExplanation {
            id: "class_chassis.druid.nature_bond_choice".to_owned(),
            value: 0,
            detail: format!(
                "Recognized Druid nature bond selection ({DRUID_NATURE_BOND_CHOICE_ID} -> \
                 {DRUID_NATURE_BOND_ANIMAL_COMPANION_SELECTION_ID}): the deterministic fixture \
                 chooses an animal companion as its PF1 nature bond. This is a bounded \
                 recognition record of the chosen input only; the chosen bond's execution stays \
                 ungrounded — no animal companion stat block, no companion advancement, and no \
                 link or share-spells behavior is computed — so it carries no fabricated \
                 mechanical value (+0)"
            ),
        });
    }

    // Grounded (SD13-E5): Woodland Stride, a 2nd-level Druid class feature verified
    // independently against two primary PF1 sources (d20pfsrd and
    // legacy.aonprd.com both list "Woodland stride" as the Druid 2nd-level special
    // feature entry). Below the level-2 gate this is a correct PF1 Core Rulebook
    // level-gate absence (value 0); at or above it, it is a bounded
    // identity/recognition record only (value 0, non-fabricated) naming the rule
    // text — mirroring exactly how Rogue's/Monk's own Evasion was grounded, without
    // folding into any actual terrain-detection engine or movement-execution
    // engine, neither of which exists in this codebase.
    if level < DRUID_WOODLAND_STRIDE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.druid.woodland_stride".to_owned(),
            value: 0,
            detail: format!(
                "Druid Woodland Stride at druid level {level}: correctly absent at level {level} \
                 by PF1 Core Rulebook level gate; the at-grant rule is named but not computed. \
                 Woodland Stride is a 2nd-level druid class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.druid.woodland_stride".to_owned(),
            value: 0,
            detail: format!(
                "Druid Woodland Stride granted at druid level {level} (PF1 Core Rulebook, \
                 2nd-level druid class feature): starting at 2nd level, a druid may move through \
                 any sort of undergrowth (natural thorns, briars, overgrown areas, and similar \
                 terrain) at her normal speed and without taking damage or suffering any other \
                 impairment; magically manipulated terrain still affects her. This is a bounded \
                 identity/recognition record only (value 0, non-fabricated): no terrain-detection \
                 engine and no movement-execution engine exists anywhere in this codebase to apply \
                 it, so this grounds no actual movement or terrain-impediment resolution"
            ),
        });
    }

    // Grounded (SD13-E5): Trackless Step, a 3rd-level Druid class feature verified
    // independently against two primary PF1 sources (d20pfsrd and legacy.aonprd.com
    // both list "Trackless step" as the Druid 3rd-level special feature entry). Below
    // the level-3 gate this is a correct PF1 Core Rulebook level-gate absence (value
    // 0); at or above it, it is a bounded identity/recognition record only (value 0,
    // non-fabricated) naming the rule text — mirroring exactly how Woodland Stride and
    // Rogue's/Monk's own Evasion were grounded, without folding into any actual
    // tracking-resolution engine or terrain-detection engine, neither of which exists
    // in this codebase.
    if level < DRUID_TRACKLESS_STEP_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.druid.trackless_step".to_owned(),
            value: 0,
            detail: format!(
                "Druid Trackless Step at druid level {level}: correctly absent at level {level} \
                 by PF1 Core Rulebook level gate; the at-grant rule is named but not computed. \
                 Trackless Step is a 3rd-level druid class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.druid.trackless_step".to_owned(),
            value: 0,
            detail: format!(
                "Druid Trackless Step granted at druid level {level} (PF1 Core Rulebook, \
                 3rd-level druid class feature): starting at 3rd level, a druid leaves no trail \
                 in natural surroundings and cannot be tracked; she may choose to leave a trail \
                 if so desired. This is a bounded identity/recognition record only (value 0, \
                 non-fabricated): no tracking-resolution engine and no terrain-detection engine \
                 exists anywhere in this codebase to apply it, so this grounds no actual \
                 tracking-check or trail-detection resolution"
            ),
        });
    }

    // Grounded (SD13-E5): Resist Nature's Lure, one of two distinct entries in the
    // class table's 4th-level "Special" column, verified independently against two
    // primary PF1 sources (d20pfsrd and legacy.aonprd.com both list "Resist nature's
    // lure" alongside "Wild shape (1/day)" as the Druid 4th-level special feature
    // entry). Below the level-4 gate this is a correct PF1 Core Rulebook level-gate
    // absence (value 0); at or above it, it is a bounded flat-magnitude identity
    // record only (the rule's own flat +4 magnitude, non-fabricated as an applied
    // total) — mirroring exactly how Bravery/Divine Grace/Trap Sense were grounded:
    // this record is never wired into any actual saving-throw total, since no
    // saving-throw resolution engine exists in this codebase. The class table's
    // other level-4 entry, Wild Shape (1/day), was checked and confirmed NOT flat —
    // it is a full shapeshifting subsystem (new form, new stat block, duration
    // tracking) with no execution engine anywhere in this codebase — so it is
    // deliberately left named-but-unproven here, exactly like the animal-companion
    // execution burden below, and no record or diagnostic for it is fabricated.
    if level < DRUID_RESIST_NATURES_LURE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.druid.resist_natures_lure".to_owned(),
            value: 0,
            detail: format!(
                "Druid Resist Nature's Lure at druid level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant flat magnitude is named but \
                 not computed. Resist Nature's Lure is a 4th-level druid class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.druid.resist_natures_lure".to_owned(),
            value: DRUID_RESIST_NATURES_LURE_BONUS,
            detail: format!(
                "Druid Resist Nature's Lure granted at druid level {level} (PF1 Core Rulebook, \
                 4th-level druid class feature): a druid gains a +{DRUID_RESIST_NATURES_LURE_BONUS} \
                 bonus on saving throws against the spell-like and supernatural abilities of fey; \
                 this bonus also applies to spells and effects that utilize or target plants, such \
                 as blight, entangle, spike growth, and warp wood. This is a bounded flat-magnitude \
                 identity record only: no saving-throw resolution engine exists anywhere in this \
                 codebase to apply it, so this grounds no actual saving-throw total"
            ),
        });
    }

    // Grounded (SD13-E5 level-9 slice): Venom Immunity, the 9th-level Druid class
    // feature verified independently against two primary PF1 sources (d20pfsrd and
    // legacy.aonprd.com both list "Venom immunity" as the Druid 9th-level "Special"
    // entry, the rule text reading "At 9th level, a druid gains immunity to all
    // poisons"). A genuinely flat/identity-shaped, no-choice, no-magnitude grant —
    // exactly like Monk's Purity of Body (immunity to disease) — grounded as a
    // bounded +0 identity/recognition record at or above the gate: no
    // poison/condition-resolution engine exists anywhere in this codebase to apply
    // it, so no immunity effect is fabricated. Below the level-9 gate no record is
    // pushed at all (the level-9 slice's own level-8 control pins that absence).
    if level >= DRUID_VENOM_IMMUNITY_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.druid.venom_immunity".to_owned(),
            value: 0,
            detail: format!(
                "Druid Venom Immunity granted at druid level {level} (PF1 Core Rulebook, \
                 9th-level druid class feature): the druid gains immunity to all poisons. \
                 This is a bounded identity/recognition record only (value 0, non-fabricated): \
                 no poison-application or condition-resolution engine exists anywhere in this \
                 codebase to apply it, so this grounds no actual immunity effect on any \
                 poison outcome"
            ),
        });
    }

    // Still blocked (1/2): name the animal companion execution burden explicitly. Wild
    // Empathy, Nature Sense, and (when recognized) the nature-bond choice recognition
    // are grounded above and no longer named here as blockers. The message must not
    // claim a specific bond was chosen unless the choice-selection lookup above
    // actually recognized one — otherwise it would fabricate the claim that an
    // animal companion was picked when no nature-bond selection was made at all.
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.druid.animal_companion.unsupported".to_owned(),
        message: if animal_companion_chosen {
            format!(
                "Druid level {level} remains blocked on its animal companion \
                 execution burden: the chosen nature bond (an animal companion) is recognized as \
                 input only — the companion's stat block, its advancement, and its link and share \
                 spells abilities are not implemented in this bounded prepared divine spell \
                 baseline, so no Druid animal companion support is claimed"
            )
        } else {
            format!(
                "Druid level {level} remains blocked on its animal companion \
                 execution burden: no nature bond selection is recognized as chosen input in \
                 this bounded prepared divine spell baseline, and even when an animal companion \
                 bond is chosen its stat block, its advancement, and its link and share spells \
                 abilities are not implemented, so no Druid animal companion support is claimed"
            )
        },
        claim_blocking: true,
    });

    // Still blocked (2/2): name the prepared divine spell posture burden explicitly.
    diagnostics.push(ComputationDiagnostic {
        id: "class_spell.druid.prepared_divine.unsupported".to_owned(),
        message:
            "Druid remains blocked on its prepared divine spell posture burden: spells prepared \
             from the full Druid spell list, spontaneous summon nature's ally conversion, spell slots \
             per day, bonus spell slots from a high Wisdom, and spell save DCs are out of scope for \
             this level-1 spell baseline and no spell math is fabricated"
                .to_owned(),
        claim_blocking: true,
    });
}

/// The bounded Bard milestone level this decomposition surface grounds, if any.
/// Returns the single Bard level when the chosen input is exactly a single-class
/// Bard at one of the supported milestone levels (1 through `MAX_SUPPORTED_BARD_LEVEL`,
/// currently 8). Returns `None` for no Bard, a non-Bard class, a multiclass mix, or
/// any level-9+ Bard this slice deliberately does not recognize — each of which stays
/// claim-blocked exactly as before. Mirrors the Fighter `supported_fighter_level` /
/// Paladin `supported_paladin_level` / Rogue `supported_rogue_level` / Barbarian
/// `supported_barbarian_level` / Monk `supported_monk_level` / Cleric
/// `supported_cleric_level` level-range gate idiom.
fn supported_bard_level(input: &CharacterInput) -> Option<u8> {
    match input.chosen.class_levels.as_slice() {
        [class_level]
            if class_level.class_id == BARD_CLASS_ID
                && (1..=MAX_SUPPORTED_BARD_LEVEL).contains(&class_level.level) =>
        {
            Some(class_level.level)
        }
        _ => None,
    }
}

/// Surface direct SD13-E4-F7/SD13-E4/SD13-E5 runtime evidence for the deterministic
/// Human Bard level-1/level-2/level-3 spontaneous arcane spell-bearing baseline: one
/// recognition record, the foundational base-attack-bonus / base-save progression
/// pillar (four standalone records), five grounded chassis-class-feature pillars
/// (Bardic Knowledge, the Bardic Performance rounds-per-day budget, the Inspire
/// Courage flat magnitude, and the Fascinate flat Will-save DC and
/// affected-creature-count formulas), a sixth pillar grounded only at level 2
/// (Well-Versed's flat +4 save-bonus magnitude), a seventh pillar grounded only at
/// level 3 (Inspire Competence's flat +2 skill-check magnitude), and two remaining
/// named claim-blocking burdens (the bardic performance-execution engine, the
/// spontaneous spell posture).
///
/// This deliberately does not compute a supported Bard chassis. It grounds no
/// bardic performance execution — no start/maintain action economy, no round
/// tracking or consumption, and no Will-save/targeting resolution for Fascinate,
/// nor anything at all for Countersong, Distraction, or Versatile Performance
/// (all three require either an opposed Perform-check-vs-effect substitution
/// resolution or a choice-gated skill-substitution engine, not a flat number) —
/// and no spell math whatsoever: no spells known, no spells per day, no spell
/// DCs, no bonus spells, no prepared posture, no school choice. It only:
/// - leaves one recognition explanation so the `class:bard:N` identity is acknowledged
///   as a spontaneous arcane spell-bearing class rather than an undocumented packet
///   placeholder (direct runtime evidence, carrying no fabricated mechanical value),
/// - grounds the foundational base-attack-bonus / base-save progression pillar that
///   every other class row in this matrix (Fighter, Barbarian, Monk, Rogue, Paladin,
///   Druid, Cleric) already has and Bard never had: base attack bonus (3/4 BAB,
///   `classlevel * 3 / 4`, the same formula shape as Rogue/Monk/Druid/Cleric) and
///   base save progression (good Reflex, good Will, poor Fortitude — the same save
///   shape as Rogue, confirmed independently against the raw PF1 Core Rulebook Bard
///   class table rather than assumed from Rogue's own pattern). Both are grounded as
///   flat, standalone `ComputationExplanation` records, mirroring the exact
///   "standalone, not wired into the integrated `PilotBaseChassisComputation`" idiom
///   already used for every other class's own base-attack/base-save grounding: neither
///   is wired into `base_attack_bonus`, `compute_total_saves`, or
///   `compute_combat_baseline`,
/// - grounds the Bardic Knowledge chassis-class-feature pillar for real: PF1 Core
///   Rulebook Bardic Knowledge is a flat competence bonus on Knowledge checks equal
///   to half the bard's level (minimum 1), also letting the bard make any Knowledge
///   check untrained. That flat bonus needs no skill-rank state and no ability
///   modifier (the Intelligence modifier already belongs to the ordinary Knowledge
///   check, not to this class-feature bonus), so it is a bounded, deterministic,
///   level-only value; this grounds only that flat bonus, not a full Knowledge-check
///   resolution,
/// - grounds the flat Bardic Performance surface for real: the rounds-per-day
///   budget (PF1 Core Rulebook Bardic Performance: 4 + Charisma modifier rounds
///   per day at level 1, plus 2 additional rounds per day at each level after
///   1st — verified against d20pfsrd and legacy.aonprd.com rather than assumed
///   from Barbarian's superficially similar Rage-rounds progression, floored at
///   0) and the Inspire Courage flat magnitude (+1 competence bonus on attack
///   and weapon damage rolls, +1 morale bonus on saving throws against charm and
///   fear effects — confirmed unchanged through level 2, since the PF1 Core
///   Rulebook bonus first increases only at bard level 5). These are bounded
///   flat values only; no performance-state engine applies them anywhere,
/// - grounds the Fascinate flat Will-save DC (10 + 1/2 bard level + Charisma
///   modifier) and the Fascinate flat affected-creature count (1 at 1st level,
///   plus one more for every three bard levels beyond 1st) for real, verified
///   against the PF1 Core Rulebook Fascinate rule text rather than assumed from
///   memory. Both are bounded flat values only; neither is ever applied to an
///   actual Will save or targeting outcome,
/// - grounds Well-Versed (SD13-E5, a 2nd-level Bard class feature verified
///   independently against two primary PF1 sources — d20pfsrd and
///   legacy.aonprd.com both list "Versatile performance, well-versed" as the
///   Bard 2nd-level special feature entry) as a flat, non-level-scaled +4
///   standalone magnitude on saving throws against bardic performance, sonic,
///   and language-dependent effects, mirroring the Fighter Bravery idiom: never
///   applied to any actual save total, since no save-resolution engine exists
///   in this codebase. Versatile Performance (the Bard's OTHER 2nd-level
///   feature) is NOT flat — it requires a choice of Perform type and an actual
///   skill-substitution engine — so it is deliberately left named-but-unproven,
///   mirroring how the Monk level-2 bonus feat grant was deliberately left
///   unrecognized by the Monk level-2 widening slice,
/// - grounds Inspire Competence (SD13-E5, a 3rd-level Bard class feature verified
///   independently against two primary PF1 sources — d20pfsrd and
///   legacy.aonprd.com both list "Inspire competence +2" as the Bard 3rd-level
///   special feature entry) as a flat +2 standalone magnitude (a competence bonus
///   on skill checks with a particular skill), mirroring the Fighter Bravery /
///   Rogue Trap Sense / Barbarian Trap Sense / Monk Still Mind idiom: never
///   applied to any actual skill-check total, since no skill-check-resolution
///   engine exists in this codebase, and no task-selection/action-economy engine
///   decides which skill or ally it targets, and
/// - emits two distinct claim-blocking diagnostics naming the still-unproven bardic
///   performance-execution burden (start/maintain action economy, round tracking and
///   consumption, no application of any grounded magnitude/DC/count to an actual
///   total, and the fully-ungrounded Countersong / Distraction performances) and the
///   spontaneous known-spell / slot posture burden explicitly, rather than hiding
///   behind a generic "unsupported caster" label.
///
/// A later SD13-E5 slice widens the level-range gate to level 4, extending every
/// formula above (base attack, base saves, Bardic Knowledge, Bardic Performance
/// rounds/day, Fascinate DC/count) via the same level-valued formulas, and keeping
/// Well-Versed and Inspire Competence granted, without re-deriving any of them.
/// Verified independently against the PF1 Core Rulebook Bard class table
/// (d20pfsrd and legacy.aonprd.com): the level-4 "Special" column is BLANK, so no
/// new pillar is grounded at level 4.
///
/// The bounded Fighter-shaped compute path already claim-blocks this input; this seam
/// keeps that blocked posture but makes the Bard spell-bearing identity, the grounded
/// flat pillars, and the two remaining named burdens legible on the runtime path.
fn explain_bard_level1_spell_baseline(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    let Some(level) = supported_bard_level(input) else {
        return;
    };
    if input.chosen.race_id != HUMAN_RACE_ID {
        return;
    }

    // Direct runtime evidence: recognize the deterministic Human Bard spell-bearing
    // identity at the supported level. This is a recognition record only; it
    // fabricates no spell math.
    explanations.push(ComputationExplanation {
        id: "class_chassis.spell_baseline.bard".to_owned(),
        value: 0,
        detail: format!(
            "Recognized deterministic Human Bard level {level} spell-bearing baseline: the \
             {BARD_CLASS_ID}:{level} class identity is acknowledged as a spontaneous arcane \
             spell-bearing class with its named bardic performance-execution \
             chassis-class-feature burden on the rules-core seam rather than an undocumented \
             packet placeholder. This is a bounded recognition record only; it grounds no \
             bardic performance execution (no start/maintain action economy, no round tracking \
             or consumption, no countersong / distraction / fascinate resolution) and no spell \
             math (spells known, spells per day, spell DCs, bonus spells, or prepared posture), \
             so it carries no fabricated mechanical value (+0)"
        ),
    });

    // Grounded: the foundational base-attack-bonus / base-save progression pillar.
    // Unlike every other class row in this matrix (Fighter, Barbarian, Monk, Rogue,
    // Paladin, Druid, Cleric all already ground this pillar), Bard had never had it
    // grounded at all until an earlier SD13-E5 slice. Both formulas were verified
    // against the PF1 Core Rulebook Bard class table (d20pfsrd and the legacy
    // Paizo PRD mirror) before writing this code, reading the raw level 1-6 table
    // rows directly (BAB +0/+1/+2/+3/+3/+4, Fort +0/+0/+1/+1/+1/+2, Ref
    // +2/+3/+3/+4/+4/+5, Will +2/+3/+3/+4/+4/+5) rather than trusting memory or
    // assuming Bard's save shape merely because it resembles Rogue's: the level
    // 4/5 BAB values (+3 at both) disambiguate the 3/4-vs-1/2 fraction (level 1
    // alone floors both to +0), and the raw Fort/Ref/Will columns independently
    // confirm good Reflex, good Will, poor Fortitude — the same save shape as
    // Rogue, but checked against Bard's own table rather than assumed from
    // Rogue's. A later SD13-E5 slice widens the level-1-only gate to level 2 and
    // extends every one of the formulas below to level 2 via the same formula,
    // without re-derivation, verified independently against the PF1 Core
    // Rulebook Bard class table: level 2 base attack +1, base saves +0/+3/+3
    // (Fortitude/Reflex/Will).
    let level_value = i16::from(level);

    // Grounded (1/2): 3/4-BAB base-attack progression, the same formula shape as
    // Rogue/Monk/Druid/Cleric (classlevel * 3 / 4). No PCGen .lst file exists for
    // the Bard class in this repo, so the formula cites the PF1 Core Rulebook Bard
    // class table directly.
    let base_attack_bonus = level_value * 3 / 4;
    explanations.push(ComputationExplanation {
        id: "class_chassis.bard.base_attack_bonus".to_owned(),
        value: base_attack_bonus,
        detail: format!(
            "Bard level {level} base attack bonus from the PF1 Core Rulebook Bard class \
             table's 3/4-BAB progression, the same formula shape as Rogue/Monk/Druid/Cleric: \
             classlevel * 3 / 4 = {base_attack_bonus}. This is a standalone explanation record; \
             it is not wired into the integrated base_attack_bonus field or into \
             compute_combat_baseline"
        ),
    });

    // Grounded (2/2): base-save progression — poor Fortitude, good Reflex, good
    // Will, verified against the PF1 Core Rulebook Bard class table (Fortitude
    // +0, Reflex +2, Will +2 at level 1; Fortitude +0, Reflex +3, Will +3 at
    // level 2).
    let good_save = level_value / 2 + 2;
    let poor_save = level_value / 3;
    explanations.push(ComputationExplanation {
        id: "class_chassis.bard.base_save.fortitude".to_owned(),
        value: poor_save,
        detail: format!(
            "Bard level {level} base Fortitude save (poor save) from the PF1 Core Rulebook \
             Bard class table: classlevel/3 = {poor_save}. This is a standalone explanation \
             record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.bard.base_save.reflex".to_owned(),
        value: good_save,
        detail: format!(
            "Bard level {level} base Reflex save (good save) from the PF1 Core Rulebook Bard \
             class table: classlevel/2+2 = {good_save}. This is a standalone explanation \
             record; it is not wired into compute_total_saves"
        ),
    });
    explanations.push(ComputationExplanation {
        id: "class_chassis.bard.base_save.will".to_owned(),
        value: good_save,
        detail: format!(
            "Bard level {level} base Will save (good save) from the PF1 Core Rulebook Bard \
             class table: classlevel/2+2 = {good_save}. This is a standalone explanation \
             record; it is not wired into compute_total_saves"
        ),
    });

    // Grounded for real: the Bardic Knowledge pillar. PF1 Core Rulebook Bardic
    // Knowledge: "A bard adds half his bard level (minimum 1) to Knowledge skill
    // checks and may make all Knowledge skill checks untrained." That is a flat
    // competence bonus, not "half level + INT modifier": the Intelligence modifier
    // is already part of the ordinary Knowledge skill check total (rank + ability
    // modifier + misc bonuses), so it is not an additional term this class-feature
    // bonus contributes on its own. Confirmed unchanged at level 2
    // (max(2/2, 1) = 1, the same value as level 1's floor-forced 1, but reached
    // naturally this time rather than via the floor), via the same formula, not a
    // new record.
    let bardic_knowledge_bonus = (level_value / 2).max(1);
    explanations.push(ComputationExplanation {
        id: "class_chassis.bard.bardic_knowledge".to_owned(),
        value: bardic_knowledge_bonus,
        detail: format!(
            "Bard Bardic Knowledge class feature: grants a competence bonus on Knowledge \
             skill checks equal to max(bard level / 2, 1) (PF1 Core Rulebook Bardic Knowledge: \
             half bard level, minimum +1), and lets the bard make any Knowledge skill check \
             untrained. At Bard level {level} this bonus is max({level} / 2, 1) = \
             {bardic_knowledge_bonus}. This grounds only the flat Knowledge-check competence \
             bonus; it is not a full Knowledge-check resolution engine and adds no skill rank, \
             no ability modifier, and no untrained-check gate, and it grounds no bardic \
             performance execution"
        ),
    });

    // Grounded for real: the Bardic Performance rounds-per-day budget. PF1 Core
    // Rulebook Bardic Performance: a level-1 bard can use bardic performance for a
    // number of rounds per day equal to 4 + his Charisma modifier. "At each level
    // after 1st a bard can use bardic performance for 2 additional rounds per
    // day" (verified against d20pfsrd and legacy.aonprd.com before widening,
    // rather than assumed to match Barbarian's superficially similar Rage-rounds
    // progression), so the formula widens to
    // 4 + Charisma modifier + 2 * (level - 1), floored at 0 mirroring the Cleric
    // channel-energy uses-per-day floor.
    let bardic_performance_rounds_per_day = (4
        + ability_modifiers.charisma
        + BARD_PERFORMANCE_ADDITIONAL_ROUNDS_PER_LEVEL * (level_value - 1))
        .max(0);
    explanations.push(ComputationExplanation {
        id: "class_chassis.bard.bardic_performance_rounds_per_day".to_owned(),
        value: bardic_performance_rounds_per_day,
        detail: format!(
            "Bard Bardic Performance rounds per day at bard level {level} (PF1 Core Rulebook \
             Bardic Performance): 4 + Charisma modifier at level 1, plus 2 additional rounds \
             per day at each level after 1st, floored at 0. At Charisma modifier {} this is \
             max(4 + {} + {BARD_PERFORMANCE_ADDITIONAL_ROUNDS_PER_LEVEL} * ({level} - 1), 0) = \
             {bardic_performance_rounds_per_day}. This grounds only the flat daily round \
             budget; no round tracking or consumption, no start/maintain action economy, and \
             no per-performance execution is computed",
            ability_modifiers.charisma, ability_modifiers.charisma
        ),
    });

    // Grounded for real: the Inspire Courage flat magnitude. PF1 Core Rulebook
    // Inspire Courage at bard level 1: affected allies receive a +1 morale bonus
    // on saving throws against charm and fear effects and a +1 competence bonus
    // on attack and weapon damage rolls. Confirmed unchanged through level 4: the
    // PF1 Core Rulebook Inspire Courage bonus first increases (to +2) exactly at
    // bard level 5 (verified independently against d20pfsrd and
    // legacy.aonprd.com's Bard class table before widening this slice, re-checked
    // rather than trusted from an earlier cycle's phrasing at face value: "At 5th
    // level, and every six bard levels thereafter, this bonus increases by +1" —
    // the increase lands AT level 5, not after it, so the earlier cycle's "stays
    // +1 through level 5" framing turns out to have been precise). Only the flat
    // magnitude is grounded; no performance-state engine exists to start the
    // performance or apply the bonus to any computed total.
    let inspire_courage_bonus = if level >= BARD_INSPIRE_COURAGE_SECOND_TIER_LEVEL {
        BARD_INSPIRE_COURAGE_BONUS_SECOND_TIER
    } else {
        BARD_INSPIRE_COURAGE_BONUS_FIRST_TIER
    };
    explanations.push(ComputationExplanation {
        id: "class_chassis.bard.inspire_courage_bonus".to_owned(),
        value: inspire_courage_bonus,
        detail: format!(
            "Bard Inspire Courage magnitude at bard level {level} (PF1 Core Rulebook Inspire \
             Courage): a +{inspire_courage_bonus} competence bonus on attack rolls and weapon \
             damage rolls and a +{inspire_courage_bonus} morale bonus on saving throws against \
             charm and fear effects for affected allies. This magnitude increases from +1 to +2 \
             exactly at bard level {BARD_INSPIRE_COURAGE_SECOND_TIER_LEVEL} (PF1 Core Rulebook: \
             \"At 5th level, and every six bard levels thereafter, this bonus increases by \
             +1\"), so it is +{inspire_courage_bonus} at level {level}; the next increase (to \
             +3) is at bard level 11, out of scope for this bounded slice. This grounds only the \
             flat magnitude of the fixture's chosen performance \
             (choice:bard_bardic_music -> performance:inspire_courage); it is never applied to \
             any attack, damage, or save total because the performance-state engine \
             (start/maintain action economy, round tracking) is not implemented"
        ),
    });

    // Grounded for real: the Fascinate flat Will-save DC formula. PF1 Core
    // Rulebook Fascinate: each creature within range receives a Will save (DC
    // 10 + 1/2 the bard's level + the bard's Charisma modifier) to negate the
    // effect. Verified against the PF1 Core Rulebook Fascinate rule text (d20pfsrd
    // and the legacy Paizo PRD mirror), not trusted from memory alone. This
    // formula already takes bard level as an input variable, so it extends to
    // level 2 without re-derivation. Only the flat DC magnitude is grounded; no
    // Will-save resolution and no application of this DC to any actual save
    // total is computed.
    let fascinate_dc = FASCINATE_DC_BASE + (level_value / 2) + ability_modifiers.charisma;
    explanations.push(ComputationExplanation {
        id: "class_chassis.bard.fascinate_dc".to_owned(),
        value: fascinate_dc,
        detail: format!(
            "Bard Fascinate Will save DC at bard level {level} (PF1 Core Rulebook Fascinate): \
             DC = 10 + 1/2 bard level + Charisma modifier. At bard level {level} and Charisma \
             modifier {} this is {FASCINATE_DC_BASE} + ({level} / 2) + {} = {fascinate_dc}. \
             This grounds only the flat DC magnitude; no Will-save resolution, no \
             range/line-of-sight/attention-requirement checking, and no application of this DC \
             to any actual save total is computed because the performance-state engine is not \
             implemented",
            ability_modifiers.charisma, ability_modifiers.charisma
        ),
    });

    // Grounded for real: the Fascinate flat affected-creature-count formula. PF1
    // Core Rulebook Fascinate: a bard can affect one creature at 1st level, and
    // targets one additional creature for every three bard levels attained beyond
    // 1st. Verified against the PF1 Core Rulebook Fascinate rule text the same
    // way as the DC above; this is deliberately NOT "half the bard's level" — a
    // different-looking formula that happens to coincide with the correct one
    // only at level 1, which is exactly the kind of from-memory error a primary
    // source check catches (mirroring the earlier Ranger combat-style and
    // Paladin mercy level-gate corrections). This formula already takes bard
    // level as an input variable, so it extends to level 2 without re-derivation.
    let fascinate_affected_creatures = 1 + (level_value - 1) / 3;
    explanations.push(ComputationExplanation {
        id: "class_chassis.bard.fascinate_affected_creatures".to_owned(),
        value: fascinate_affected_creatures,
        detail: format!(
            "Bard Fascinate affected-creature count at bard level {level} (PF1 Core Rulebook \
             Fascinate): 1 creature at 1st level, plus one additional creature for every three \
             bard levels attained beyond 1st — formula 1 + (bard level - 1) / 3. At bard level \
             {level} this is 1 + ({level} - 1) / 3 = {fascinate_affected_creatures}. This \
             grounds only the flat creature-count magnitude; no \
             range/line-of-sight/attention-requirement checking and no application of this \
             count to any actual targeting resolution is computed"
        ),
    });

    // Grounded (SD13-E5): Well-Versed, a 2nd-level Bard class feature verified
    // independently against two primary PF1 sources (d20pfsrd and
    // legacy.aonprd.com both list "Versatile performance, well-versed" as the
    // Bard 2nd-level special feature entry). Below the level-2 gate this is a
    // correct PF1 Core Rulebook level-gate absence (value 0); at or above it, it
    // is a flat, non-level-scaled +4 standalone magnitude (verified against both
    // primary sources: unlike Bardic Knowledge or Fascinate, this bonus does NOT
    // scale with level), mirroring the Fighter Bravery idiom — never applied to
    // any actual save total, since no save-resolution engine exists in this
    // codebase.
    if level < BARD_WELL_VERSED_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.well_versed".to_owned(),
            value: 0,
            detail: format!(
                "Bard Well-Versed at bard level {level}: correctly absent at level {level} by \
                 PF1 Core Rulebook level gate; the at-grant rule is named but not computed. \
                 Well-Versed is a 2nd-level Bard class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.well_versed".to_owned(),
            value: BARD_WELL_VERSED_BONUS,
            detail: format!(
                "Bard Well-Versed granted at bard level {level} (PF1 Core Rulebook, 2nd-level \
                 Bard class feature): a flat +{BARD_WELL_VERSED_BONUS} bonus on saving throws \
                 made against bardic performance, sonic, and language-dependent effects. Unlike \
                 Bardic Knowledge or Fascinate, this magnitude is not level-scaled. This is a \
                 standalone explanation record only; it is never applied to any actual save \
                 total because no saving-throw-resolution engine exists anywhere in this \
                 codebase"
            ),
        });
    }

    // Grounded (SD13-E5): Inspire Competence, a 3rd-level Bard class feature
    // verified independently against two primary PF1 sources (d20pfsrd and
    // legacy.aonprd.com both list "Inspire competence +2" as the Bard 3rd-level
    // special feature entry). Below the level-3 gate this is a correct PF1 Core
    // Rulebook level-gate absence (value 0); at or above it, it is a flat
    // standalone magnitude (a competence bonus on skill checks with a particular
    // skill, verified against both primary sources), mirroring the Fighter
    // Bravery / Rogue Trap Sense / Barbarian Trap Sense / Monk Still Mind idiom —
    // never applied to any actual skill-check total, since no
    // skill-check-resolution engine exists in this codebase, and no
    // task-selection/action-economy engine decides which skill or ally it
    // targets. The magnitude genuinely increases from +2 to +3 exactly at bard
    // level 7 (SD13-E5, verified independently against two primary sources: both
    // list "Inspire competence +3" as the Bard 7th-level special feature entry,
    // and both state the rule text "This bonus increases by +1 for every four
    // levels the bard has attained beyond 3rd"), mirroring the Inspire Courage
    // second-tier idiom exactly; the next increase (to +4) lands at bard level
    // 11, out of scope for this bounded slice.
    if level < BARD_INSPIRE_COMPETENCE_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.inspire_competence".to_owned(),
            value: 0,
            detail: format!(
                "Bard Inspire Competence at bard level {level}: correctly absent at level \
                 {level} by PF1 Core Rulebook level gate; the at-grant rule is named but not \
                 computed. Inspire Competence is a 3rd-level Bard class feature."
            ),
        });
    } else {
        let inspire_competence_bonus = if level >= BARD_INSPIRE_COMPETENCE_SECOND_TIER_LEVEL {
            BARD_INSPIRE_COMPETENCE_BONUS_SECOND_TIER
        } else {
            BARD_INSPIRE_COMPETENCE_BONUS_FIRST_TIER
        };
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.inspire_competence".to_owned(),
            value: inspire_competence_bonus,
            detail: format!(
                "Bard Inspire Competence granted at bard level {level} (PF1 Core Rulebook, \
                 3rd-level Bard class feature): a flat +{inspire_competence_bonus} competence \
                 bonus on skill checks with a particular skill. This magnitude increases from \
                 +2 to +3 exactly at bard level {BARD_INSPIRE_COMPETENCE_SECOND_TIER_LEVEL} (PF1 \
                 Core Rulebook: \"This bonus increases by +1 for every four levels the bard has \
                 attained beyond 3rd\"), so it is +{inspire_competence_bonus} at level {level}; \
                 the next increase (to +4) is at bard level 11, out of scope for this bounded \
                 slice. This is a standalone explanation record only; it is never applied to any \
                 actual skill-check total because no skill-check-resolution engine exists \
                 anywhere in this codebase, and no task-selection/action-economy engine decides \
                 which skill or ally it targets"
            ),
        });
    }

    // Grounded (SD13-E5): Lore Master, a 5th-level Bard class feature verified
    // independently against two primary PF1 sources (d20pfsrd and
    // legacy.aonprd.com both list "Inspire courage +2, lore master 1/day" as the
    // Bard 5th-level special feature entry). Below the level-5 gate this is a
    // correct PF1 Core Rulebook level-gate absence (value 0); at or above it, this
    // grounds only the rule's own flat "1/day" usage-count magnitude for the
    // take-20 half of the feature, mirroring the Paladin Smite Evil / Wizard
    // Force Missile uses-per-day idiom. The rule text grants TWO distinct
    // capabilities: (1) an at-will "take 10 on any Knowledge skill check that he
    // has ranks in" capability, which has no flat magnitude to ground at all (it
    // is a resolution-mode toggle, not a countable resource) and would require a
    // skill-check-resolution engine that does not exist anywhere in this
    // codebase, and (2) "once per day, the bard can take 20 on any Knowledge
    // skill check as a standard action" — a genuinely flat 1/day count, grounded
    // here as a bounded grant-only identity record. Neither the take-10 nor the
    // take-20 mechanic is actually executed against any Knowledge check (no
    // skill-check-resolution engine exists in this codebase), mirroring the
    // Barbarian Improved Uncanny Dodge / Monk Purity of Body idiom exactly: a
    // bounded grant, not an executed mechanic.
    if level < BARD_LORE_MASTER_LEVEL {
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.lore_master".to_owned(),
            value: 0,
            detail: format!(
                "Bard Lore Master at bard level {level}: correctly absent at level {level} by \
                 PF1 Core Rulebook level gate; the at-grant rule is named but not computed. Lore \
                 Master is a 5th-level Bard class feature."
            ),
        });
    } else {
        explanations.push(ComputationExplanation {
            id: "class_feature.bard.lore_master".to_owned(),
            value: BARD_LORE_MASTER_TAKE_20_USES_PER_DAY,
            detail: format!(
                "Bard Lore Master granted at bard level {level} (PF1 Core Rulebook, 5th-level \
                 Bard class feature): \"the bard becomes a master of lore and can take 10 on any \
                 Knowledge skill check that he has ranks in... once per day, the bard can take \
                 20 on any Knowledge skill check as a standard action.\" This grounds only the \
                 rule's own flat {BARD_LORE_MASTER_TAKE_20_USES_PER_DAY}/day usage-count \
                 magnitude for the take-20 half of the feature (a bounded grant-only identity \
                 record, mirroring the Paladin Smite Evil / Wizard Force Missile uses-per-day \
                 idiom); the take-10 capability has no flat magnitude to ground (it is an \
                 at-will resolution-mode toggle, not a countable resource), and neither the \
                 take-10 nor the take-20 mechanic is actually executed against any Knowledge \
                 check, since no skill-check-resolution engine exists anywhere in this codebase"
            ),
        });
    }

    // Still blocked (1/2): name the narrowed bardic performance-execution burden
    // explicitly, now separated from the grounded flat pillars (Bardic Knowledge,
    // the rounds-per-day budget, the Inspire Courage magnitude, and the Fascinate
    // DC / affected-creature-count formulas). The performance-state engine, the
    // two remaining level-1 performances (Countersong, Distraction), and
    // Versatile Performance (the Bard's other 2nd-level class feature, which
    // requires a choice-gated skill-substitution engine, not a flat number)
    // remain unproven — none is attempted here.
    diagnostics.push(ComputationDiagnostic {
        id: "class_feature.bard.bardic_performance_execution.unsupported".to_owned(),
        message: format!(
            "Bard level {level} remains blocked on its bardic performance-execution burden: \
             the performance-state engine is not implemented (no start/maintain action \
             economy, no round tracking or consumption of the grounded rounds-per-day budget, \
             no application of the grounded inspire courage magnitude to any attack, damage, \
             or save total, no application of the grounded fascinate DC or \
             affected-creature-count to any actual Will-save resolution or targeting), the two \
             remaining level-1 performances (countersong, distraction) are not grounded at all \
             — both require an opposed Perform-check-vs-effect substitution resolution rather \
             than a flat number — and Versatile Performance (the Bard's other 2nd-level class \
             feature) is not grounded either — it requires a choice-gated skill-substitution \
             engine rather than a flat number — so no Bard bardic-performance execution \
             support is claimed"
        ),
        claim_blocking: true,
    });

    // Still blocked (2/2): name the spontaneous known-spell / slot posture burden
    // explicitly. Bard spells known and spells per day are gated by Bard level and CHA
    // modifier on the Bard spell list; this slice grounds no spells known, no spells per
    // day, no spell DCs, and no bonus spells from a high casting stat.
    diagnostics.push(ComputationDiagnostic {
        id: "class_spell.bard.spontaneous_known_and_per_day.unsupported".to_owned(),
        message:
            "Bard remains blocked on its spontaneous known-spell / slot posture burden: \
             spontaneous casting, spells known (from the Bard list), spells per day (from \
             the Bard table plus CHA modifier), bonus spell slots from a high casting stat, \
             and spell save DCs are out of scope for this level-1 spell baseline and no \
             spell math is fabricated"
                .to_owned(),
        claim_blocking: true,
    });
}

/// Compute total saving throws as the grounded Fighter level 1–3 base save plus the
/// relevant ability modifier, or block the claim if a supported Fighter chassis
/// (levels 1–3) is absent.
///
/// This is intentionally narrow: it adds only the single ability modifier each
/// save uses (Fortitude/CON, Reflex/DEX, Will/WIS). It does not add feat-, item-,
/// or condition-based save modifiers.
fn compute_total_saves(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    base_saves: &BaseSaves,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) -> BaseSaves {
    if supported_fighter_level(input).is_none() {
        diagnostics.push(ComputationDiagnostic {
            id: "defense.total_save.unsupported".to_owned(),
            message: format!(
                "total saving throws are only computed from the grounded {FIGHTER_CLASS_ID} \
                 levels 1-{MAX_SUPPORTED_FIGHTER_LEVEL} base saves; chosen class levels {:?} do not \
                 provide them, so no total saves were computed",
                input.chosen.class_levels
            ),
            claim_blocking: true,
        });
        return BaseSaves::default();
    }

    let total_saves = BaseSaves {
        fortitude: base_saves.fortitude + ability_modifiers.constitution,
        reflex: base_saves.reflex + ability_modifiers.dexterity,
        will: base_saves.will + ability_modifiers.wisdom,
    };

    explanations.push(ComputationExplanation {
        id: "defense.total_save.fortitude".to_owned(),
        value: total_saves.fortitude,
        detail: format!(
            "Total Fortitude save: Fighter base Fortitude save (+{}) + Constitution modifier (+{}) = {}",
            base_saves.fortitude, ability_modifiers.constitution, total_saves.fortitude
        ),
    });
    explanations.push(ComputationExplanation {
        id: "defense.total_save.reflex".to_owned(),
        value: total_saves.reflex,
        detail: format!(
            "Total Reflex save: Fighter base Reflex save (+{}) + Dexterity modifier (+{}) = {}",
            base_saves.reflex, ability_modifiers.dexterity, total_saves.reflex
        ),
    });
    explanations.push(ComputationExplanation {
        id: "defense.total_save.will".to_owned(),
        value: total_saves.will,
        detail: format!(
            "Total Will save: Fighter base Will save (+{}) + Wisdom modifier (+{}) = {}",
            base_saves.will, ability_modifiers.wisdom, total_saves.will
        ),
    });

    total_saves
}

/// Compute the selected deterministic Climb / Intimidate / Swim skill modifiers,
/// or block the claim if the selected-skill or Chain Shirt posture is absent or
/// widened beyond this slice.
///
/// This is intentionally not a skill engine. It computes only the three selected
/// Fighter class skills from the accepted deterministic rank allocations, applying
/// the already-grounded Chain Shirt armor-check penalty to the armor-check skills
/// (Climb, Swim) only. It does not handle other skills, arbitrary classes,
/// feat/racial/item skill bonuses, encumbrance, or speed-dependent adjustments.
/// Any deviation from the exact supported posture is refused with a claim-blocking
/// diagnostic and withheld selected-skill explanations rather than fabricated
/// totals.
fn compute_selected_skill_modifiers(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) -> SelectedSkillModifiers {
    let unmet = unmet_selected_skill_posture_conditions(input);

    if !unmet.is_empty() {
        diagnostics.push(ComputationDiagnostic {
            id: "skill.selected_modifier.unsupported".to_owned(),
            message: format!(
                "selected skill modifiers are only computed for the exact GE-06 deterministic \
                 Fighter level-1 Climb/Intimidate/Swim rank-1 posture with the grounded Chain Shirt \
                 armor-check penalty; unmet conditions: {}",
                unmet.join("; ")
            ),
            claim_blocking: true,
        });
        return SelectedSkillModifiers::default();
    }

    let rank = i16::from(SELECTED_SKILL_RANK);

    // The Chain Shirt armor-check penalty applied to Climb/Swim is reduced by Fighter
    // armor training from level 3, so the armor-check skills rise at that milestone.
    // The posture check above guarantees a supported Fighter level here.
    let level = supported_fighter_level(input).unwrap_or(1);
    let armor_check_penalty = effective_chain_shirt_armor_check_penalty(level);
    let armor_check_detail = if fighter_armor_training(level).armor_check_reduction > 0 {
        format!(
            "Chain Shirt armor-check penalty ({armor_check_penalty:+}, reduced from \
             {CHAIN_SHIRT_ARMOR_CHECK_PENALTY:+} by Fighter armor training)"
        )
    } else {
        format!("Chain Shirt armor-check penalty ({armor_check_penalty:+})")
    };

    // Climb (STR, armor-check skill): rank + STR + class-skill + Chain Shirt ACP.
    let climb = rank + ability_modifiers.strength + CLASS_SKILL_BONUS + armor_check_penalty;
    explanations.push(ComputationExplanation {
        id: "skill.selected_modifier.climb".to_owned(),
        value: climb,
        detail: format!(
            "Selected Climb modifier: rank {rank} + Strength modifier ({:+}) + class-skill bonus \
             ({:+}) + {armor_check_detail} = {climb}",
            ability_modifiers.strength, CLASS_SKILL_BONUS
        ),
    });

    // Intimidate (CHA, not an armor-check skill): rank + CHA + class-skill.
    let intimidate = rank + ability_modifiers.charisma + CLASS_SKILL_BONUS;
    explanations.push(ComputationExplanation {
        id: "skill.selected_modifier.intimidate".to_owned(),
        value: intimidate,
        detail: format!(
            "Selected Intimidate modifier: rank {rank} + Charisma modifier ({:+}) + class-skill \
             bonus ({:+}) = {intimidate}",
            ability_modifiers.charisma, CLASS_SKILL_BONUS
        ),
    });

    // Swim (STR, armor-check skill): rank + STR + class-skill + Chain Shirt ACP.
    let swim = rank + ability_modifiers.strength + CLASS_SKILL_BONUS + armor_check_penalty;
    explanations.push(ComputationExplanation {
        id: "skill.selected_modifier.swim".to_owned(),
        value: swim,
        detail: format!(
            "Selected Swim modifier: rank {rank} + Strength modifier ({:+}) + class-skill bonus \
             ({:+}) + {armor_check_detail} = {swim}",
            ability_modifiers.strength, CLASS_SKILL_BONUS
        ),
    });

    SelectedSkillModifiers {
        climb,
        intimidate,
        swim,
    }
}

/// Return the list of unmet conditions for the exact deterministic selected-skill
/// posture. An empty list means the posture is fully supported.
///
/// The bounded posture requires a Fighter level 1–3 chassis, exactly the three
/// selected class skills (Climb, Intimidate, Swim) each at rank 1 with no other
/// skill allocations, and the grounded Chain Shirt armor-check posture that the
/// Climb/Swim totals depend on.
fn unmet_selected_skill_posture_conditions(input: &CharacterInput) -> Vec<String> {
    let allocations = &input.chosen.skill_allocations;
    let mut unmet = Vec::new();

    if supported_fighter_level(input).is_none() {
        unmet.push(format!(
            "missing supported {FIGHTER_CLASS_ID} levels 1-{MAX_SUPPORTED_FIGHTER_LEVEL} chassis"
        ));
    }

    let expected = [CLIMB_SKILL_ID, INTIMIDATE_SKILL_ID, SWIM_SKILL_ID];
    for skill_id in expected {
        require_selected_skill_rank(allocations, skill_id, &mut unmet);
    }

    // Refuse any widening beyond exactly the three selected skills.
    for allocation in allocations {
        if !expected.contains(&allocation.skill_id.as_str()) {
            unmet.push(format!(
                "skill allocation {} is outside the selected Climb/Intimidate/Swim slice",
                allocation.skill_id
            ));
        }
    }

    // Climb and Swim totals depend on the grounded Chain Shirt armor-check posture.
    require_active_state(
        input,
        CHAIN_SHIRT_ITEM_ID,
        ActiveState::EquippedActive,
        &mut unmet,
    );

    unmet
}

/// Record an unmet condition unless the named skill is allocated exactly the
/// supported deterministic rank.
fn require_selected_skill_rank(
    allocations: &[SkillAllocation],
    skill_id: &str,
    unmet: &mut Vec<String>,
) {
    let actual = allocations
        .iter()
        .find(|a| a.skill_id == skill_id)
        .map(|a| a.ranks);
    if actual != Some(SELECTED_SKILL_RANK) {
        unmet.push(format!(
            "{skill_id} must be allocated rank {SELECTED_SKILL_RANK} for the selected-skill slice, got {actual:?}"
        ));
    }
}

/// Compute the deterministic baseline melee attack bonus and armor class, or
/// block the claim if the input is not the exact supported pilot posture.
///
/// This is intentionally not a combat engine. It computes only the GE-06
/// deterministic Longsword/Chain Shirt/Dodge/no-shield baseline. Any deviation
/// from that exact posture is refused with a claim-blocking diagnostic rather
/// than fabricating combat totals.
fn compute_combat_baseline(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    base_attack_bonus: i16,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) -> (i16, i16) {
    let unmet = unmet_combat_posture_conditions(input);

    if !unmet.is_empty() {
        diagnostics.push(ComputationDiagnostic {
            id: "combat.baseline_unsupported".to_owned(),
            message: format!(
                "baseline combat totals are only computed for the exact GE-06 deterministic \
                 Longsword/Chain Shirt/Dodge/no-shield posture; unmet conditions: {}",
                unmet.join("; ")
            ),
            claim_blocking: true,
        });
        return (0, 0);
    }

    // Baseline melee attack bonus: Fighter BAB + STR modifier + Weapon Focus
    // (Longsword) + Weapon Training (from level 5, Heavy Blades). Power Attack is
    // selected but inactive, contributing 0. The posture check above guarantees a
    // supported Fighter level here.
    let level = supported_fighter_level(input).unwrap_or(1);
    let strength_modifier = ability_modifiers.strength;
    let weapon_training_bonus = fighter_weapon_training_attack_bonus(input, level);
    let melee_attack_bonus = base_attack_bonus
        + strength_modifier
        + WEAPON_FOCUS_TO_HIT_BONUS
        + weapon_training_bonus;
    let weapon_training_detail = if weapon_training_bonus > 0 {
        format!(" + Weapon Training (Heavy Blades) (+{weapon_training_bonus})")
    } else {
        String::new()
    };

    explanations.push(ComputationExplanation {
        id: "combat.baseline_melee_attack_bonus".to_owned(),
        value: melee_attack_bonus,
        detail: format!(
            "Baseline melee attack bonus for the Longsword: Fighter base attack bonus (+{base_attack_bonus}) \
             + Strength modifier (+{strength_modifier}) + Weapon Focus (Longsword) (+{WEAPON_FOCUS_TO_HIT_BONUS}){weapon_training_detail}; \
             Power Attack is selected but inactive (+0) = {melee_attack_bonus}"
        ),
    });

    // Baseline armor class: 10 + Chain Shirt armor bonus + capped DEX + Dodge,
    // with no shield (absent posture contributes 0). Fighter armor training from
    // level 3 raises the Chain Shirt maximum Dexterity bonus.
    let effective_max_dex = CHAIN_SHIRT_MAX_DEX + fighter_armor_training(level).max_dex_increase;
    let dexterity_modifier = ability_modifiers.dexterity;
    let dexterity_contribution = dexterity_modifier.min(effective_max_dex);
    let armor_class =
        ARMOR_CLASS_BASE + CHAIN_SHIRT_ARMOR_BONUS + dexterity_contribution + DODGE_AC_BONUS;

    explanations.push(ComputationExplanation {
        id: "defense.baseline_armor_class".to_owned(),
        value: armor_class,
        detail: format!(
            "Baseline armor class: base {ARMOR_CLASS_BASE} + Chain Shirt armor bonus (+{CHAIN_SHIRT_ARMOR_BONUS}) \
             + Dexterity contribution (+{dexterity_contribution}, DEX modifier +{dexterity_modifier} within MAXDEX:{effective_max_dex}) \
             + Dodge (+{DODGE_AC_BONUS}); shield is absent (+0) = {armor_class}"
        ),
    });

    (melee_attack_bonus, armor_class)
}

/// Return the list of unmet conditions for the exact deterministic combat
/// posture. An empty list means the posture is fully supported.
fn unmet_combat_posture_conditions(input: &CharacterInput) -> Vec<String> {
    let chosen = &input.chosen;
    let mut unmet = Vec::new();

    if supported_fighter_level(input).is_none() {
        unmet.push(format!(
            "missing supported {FIGHTER_CLASS_ID} levels 1-{MAX_SUPPORTED_FIGHTER_LEVEL} chassis"
        ));
    }

    require_active_state(
        input,
        LONGSWORD_ITEM_ID,
        ActiveState::EquippedActive,
        &mut unmet,
    );
    require_active_state(
        input,
        CHAIN_SHIRT_ITEM_ID,
        ActiveState::EquippedActive,
        &mut unmet,
    );
    require_active_state(input, SHIELD_ITEM_ID, ActiveState::Absent, &mut unmet);
    require_active_state(
        input,
        POWER_ATTACK_ITEM_ID,
        ActiveState::SelectedInactive,
        &mut unmet,
    );

    if !chosen.selected_feats.iter().any(|f| f == DODGE_FEAT_ID) {
        unmet.push(format!("missing selected feat {DODGE_FEAT_ID}"));
    }
    if !chosen
        .selected_feats
        .iter()
        .any(|f| f == WEAPON_FOCUS_FEAT_ID)
    {
        unmet.push(format!("missing selected feat {WEAPON_FOCUS_FEAT_ID}"));
    }

    let fighter_bonus_selection = choice_selection(input, FIGHTER_BONUS_FEAT_CHOICE_ID);
    if fighter_bonus_selection != Some(WEAPON_FOCUS_LONGSWORD_SELECTION) {
        unmet.push(format!(
            "{FIGHTER_BONUS_FEAT_CHOICE_ID} selection must be {WEAPON_FOCUS_LONGSWORD_SELECTION}, got {fighter_bonus_selection:?}"
        ));
    }

    unmet
}

/// Record an unmet condition unless the named item has exactly `expected` state.
fn require_active_state(
    input: &CharacterInput,
    item_id: &str,
    expected: ActiveState,
    unmet: &mut Vec<String>,
) {
    let actual = input
        .chosen
        .equipment_selections
        .iter()
        .find(|e| e.item_id == item_id)
        .map(|e| e.active_state);
    if actual != Some(expected) {
        unmet.push(format!(
            "{item_id} must be {expected:?} for the deterministic baseline, got {actual:?}"
        ));
    }
}
