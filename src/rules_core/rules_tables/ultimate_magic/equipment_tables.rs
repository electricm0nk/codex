//! UM equipment table -- full in-scope corpus coverage.
//!
//! Record coverage: every real, active record across `um_equip_general.lst`
//! (24 pregenerated `Spellbook.COPY=<name>` records -- General category) and
//! `um_equip_arms_armor.lst` (2 Scrollmaster Gear records -- ArmsArmor
//! category, `Scroll Shield`/`Scroll Blade`) -- 26 total. UM has no
//! `um_equip_magic_items.lst`/`um_equipmods.lst` at all (confirmed by
//! directory listing), so this book contributes only these two categories,
//! not all four `EquipmentCategory::ALL` in general.
//!
//! **The `_pfs/pfs_um_equip_general.lst` file is NOT a second population of
//! items.** It carries `TYPE:PFSNotLegal|!PRECHARACTERTYPE:1,PC` legality
//! flags for 17 of the 24 spellbooks already declared in
//! `um_equip_general.lst` -- same `KEY:`, no new mechanical content, an
//! Organized-Play legality overlay rather than a distinct record. Traced
//! before extraction specifically to rule out a double-count (`decisions.md`
//! `§53`/`§55`/`§56`'s own recurring lesson: verify a file split is really
//! two populations before trusting a declared count). Confirmed: exactly 24
//! `Spellbook.COPY=` lines in `um_equip_general.lst`, exactly 26 total with
//! the 2 Scrollmaster Gear records -- matching `docs/work-inventory.json`'s
//! own 26-unit declared count for this book exactly, with zero double-count.
//!
//! `description` is sourced from the corpus `SPROP:` ("Special Property")
//! token, joined with `"; "` when a record carries more than one -- the same
//! convention ACG/ARG already established (neither this file nor
//! `um_equip_arms_armor.lst` carries a `DESC:` token anywhere, confirmed by
//! direct grep). The 2 Scrollmaster Gear records have no `SPROP:` at all, so
//! their `description` is genuinely `None`, not an extraction gap.
//!
//! Every spellbook's `ABILITY:...|Preparation Ritual ~ <name>` grant (13 of
//! 24 carry one) is NOT modelled as a separate mechanical record here --
//! this table's scope, matching every other book's equipment table in this
//! program, is the reference catalog (key/cost/weight/description), not full
//! effect grounding. The grant's own name still appears verbatim inside the
//! joined `SPROP:Preparation Ritual - <name>` description text, so it is not
//! silently dropped, only not separately computed.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentCategory {
    General,
    ArmsArmor,
}

impl EquipmentCategory {
    pub const ALL: &'static [EquipmentCategory] =
        &[EquipmentCategory::General, EquipmentCategory::ArmsArmor];
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EquipmentTableEntry {
    /// UM's equipment records carry no explicit `KEY:` token anywhere in
    /// either source file -- the corpus identity is the record's own
    /// display `name` (the `Spellbook.COPY=<name>` value, or the plain
    /// leading field for the two Scrollmaster Gear rows).
    pub key: &'static str,
    pub category: EquipmentCategory,
    pub name: &'static str,
    /// Cost in gold pieces from the corpus `COST:` token. Every one of
    /// UM's 26 records carries a real `COST:` token (both Scrollmaster
    /// Gear rows are `COST:0`, a real corpus value, not a missing one).
    pub cost_gp: Option<f64>,
    /// Weight in pounds from the corpus `WT:` token. 13 of 26 carry one
    /// (11 higher-level spellbooks with a real `WT:` token, both
    /// Scrollmaster Gear rows at `WT:0`); the other 13 spellbooks genuinely
    /// carry no `WT:` token at all in the corpus.
    pub weight_lbs: Option<f64>,
    /// Descriptive text, sourced from the corpus `SPROP:` token(s) -- see
    /// this module's own doc comment. `None` only when the corpus record
    /// has no `SPROP:` token at all (both Scrollmaster Gear records).
    pub description: Option<&'static str>,
}

/// SD-28-E15 equipment field-coverage audit row, mirroring the shape
/// `advanced_race_guide::equipment_tables::EquipmentFieldCoverage` and its
/// siblings already establish.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EquipmentFieldCoverage {
    pub total_records: u32,
    pub records_expected: u32,
    pub has_cost: u32,
    pub has_weight: u32,
    pub has_description: u32,
}

/// Computes this book's equipment field-coverage audit row.
pub fn field_coverage_report() -> EquipmentFieldCoverage {
    let table = equipment_tables();
    EquipmentFieldCoverage {
        total_records: table.len() as u32,
        records_expected: 26,
        has_cost: table.iter().filter(|entry| entry.cost_gp.is_some()).count() as u32,
        has_weight: table.iter().filter(|entry| entry.weight_lbs.is_some()).count() as u32,
        has_description: table.iter().filter(|entry| entry.description.is_some()).count() as u32,
    }
}

/// Full UM equipment table store: every real corpus record across both
/// source files -- 24 General (spellbooks) + 2 ArmsArmor (Scrollmaster
/// Gear) = 26.
const GENERAL_TABLE: &[EquipmentTableEntry] = &[
    EquipmentTableEntry { key: "Defensive Primer", category: EquipmentCategory::General, name: "Defensive Primer", cost_gp: Some(185.0_f64), weight_lbs: None, description: Some("Level 1 Abjurer; Protection - Average lock (DC 25); Opposition Schools - Conjuration, enchantment; Spells - 1.burning hands, detect undead, expeditious retreat, magic missile, protection from evil(S), ray of enfeeblement, shield(S)") }, // um_equip_general.lst:12
    EquipmentTableEntry { key: "Apprentice Chapbook of Rul Thaven", category: EquipmentCategory::General, name: "Apprentice Chapbook of Rul Thaven", cost_gp: Some(195.0_f64), weight_lbs: None, description: Some("Level 2 Diviner; Opposition Schools - Illusion, transmutation; Spells - 1.comprehend languages(S), detect secret doors(S), detect undead(S), identify(S), protection from evil, protection from law, summon monster I, true strike(S)") }, // um_equip_general.lst:13
    EquipmentTableEntry { key: "Unnamed Journal", category: EquipmentCategory::General, name: "Unnamed Journal", cost_gp: Some(275.0_f64), weight_lbs: None, description: Some("Level 3 Universalist; Spells - 2.fox's cunning, scorching ray; 1.alarm, feather fall, obscuring mist, mount, shield, shocking grasp, silent image, sleep") }, // um_equip_general.lst:14
    EquipmentTableEntry { key: "Traveler's Tome", category: EquipmentCategory::General, name: "Traveler's Tome", cost_gp: Some(365.0_f64), weight_lbs: None, description: Some("Level 4 Transmuter; Protection - Simple lock (DC 20); Opposition Schools - Conjuration, enchantment; Spells - 2.glide(S), knock(S), levitate(S), rope trick(S); 1.ant haul(S), enlarge person(S), erase(S), expeditious retreat(S), gravity bow(S), hold portal, jump(S), magic weapon(S), shield") }, // um_equip_general.lst:15
    EquipmentTableEntry { key: "Book of Harms", category: EquipmentCategory::General, name: "Book of Harms", cost_gp: Some(900.0_f64), weight_lbs: None, description: Some("Level 5 Evoker; Protection - Average lock (DC 25); Opposition Schools - Divination, transmutation; Spells - 3.fireball(S), lightning bolt(S); 2.acid arrow, darkness(S), ghoul touch, gust of wind(S); 1.burning hands(S), color spray, corrosive touch, hydraulic push(S), hypnotism, magic missile(S), ray of enfeeblement, shocking grasp(S); Preparation Ritual - Harmful Surge") }, // um_equip_general.lst:16
    EquipmentTableEntry { key: "Lab Journal of Constance Inflix", category: EquipmentCategory::General, name: "Lab Journal of Constance Inflix", cost_gp: Some(770.0_f64), weight_lbs: None, description: Some("Level 5 Alchemist; Formulae - 2.blur, delay poison, fire breath, transmute potion to poison, vomit swarm; 1.bomber's eye, comprehend languages, cure light wounds, jump, keen senses, negate aroma, resist energy, touch of the sea; Preparation Ritual - Alchemical Protection") }, // um_equip_general.lst:17
    EquipmentTableEntry { key: "Book of the Grave", category: EquipmentCategory::General, name: "Book of the Grave", cost_gp: Some(1265.0_f64), weight_lbs: None, description: Some("Level 6 Necromancer; Protection - The edges of the first three pages are coated with nitharit poison (contact; save Fort DC 13; frequency 1/minute for 6 minutes; effect 1d3 Con damage; cure 1 save).; Opposition Schools - Enchantment, transmutation; Spells - 3.blood biography, ray of exhaustion(S), vampiric touch(S), vitriol; 2.command undead(S), create treasure map, ghoul touch(S), spectral hand(S), stone call, web; 1.burning hands, chill touch(S), disguise self, mage armor, protection from good, ray of enfeeblement(S), sculpt corpse(S), shield, true strike, ventriloquism; Preparation Ritual - Breath of the Grave") }, // um_equip_general.lst:18
    EquipmentTableEntry { key: "Grimoire of Glittering Eyes", category: EquipmentCategory::General, name: "Grimoire of Glittering Eyes", cost_gp: Some(1755.0_f64), weight_lbs: None, description: Some("Level 7 Illusionist; Protection - The entire book is warded with illusory script (Will DC 16; on failure, the suggestion is for the reader to \"Close the book and leave\").; Opposition Schools - Evocation, necromancy; Spells - 4.phantasmal killer(S), wandering star motes(S); 3.illusory script(S), invisibility sphere(S), madness(S), major image(S); 2.magic mouth(S), misdirection(S), phantom trap(S), scare(OP); 1.alarm, animate rope, color spray(S), comprehend languages, disguise self(S), hypnotism, magic aura(S), magic weapon, shield, sleep; Preparation Ritual - Glittering Eyes") }, // um_equip_general.lst:19
    EquipmentTableEntry { key: "Tome of the Transmuter", category: EquipmentCategory::General, name: "Tome of the Transmuter", cost_gp: Some(2635.0_f64), weight_lbs: None, description: Some("Level 8 Transmuter; Protection - Good lock (DC 30), explosive runes (Reflex DC 16); Opposition Schools - Illusion, necromancy; Spells - 4.beast shape II(S), calcific touch(S), confusion, dimension door, stone shape(S); 3.arcane sight, dispel magic, explosive runes, lightning bolt, greater magic weapon(S), slow(S); 2.alter self(S), flaming sphere, knock(S), pyrotechnics(S), resist energy, see invisibility, whispering wind(S); 1.animate rope(S), charm person, color spray(OP), erase(S), floating disk, hush, mage armor, magic missile, protection from chaos, unseen servant; Preparation Ritual - Defensive Transmutation") }, // um_equip_general.lst:20
    EquipmentTableEntry { key: "Journeyman Book of Rul Thaven", category: EquipmentCategory::General, name: "Journeyman Book of Rul Thaven", cost_gp: Some(3320.0_f64), weight_lbs: None, description: Some("Level 9 Diviner; Protection - Average lock (DC 25), and the 5th-level spells are hidden with secret page. The special word is \"reveal\".; Opposition Schools - Illusion, transmutation; Spells - 5.contact other plane(S), major creation, telepathic bond(S); 4.detect scrying(S), locate creature(S), remove curse, scrying(S); 3.arcane sight(S), clairaudience/clairvoyance, secret page(OP), tongues; 2.detect thoughts(S), flaming sphere, locate object(S), resist energy, see invisibility(S), summon monster II; 1.cause fear, comprehend languages(S), detect secret doors(S), detect undead(S), disguise self(OP), identify(S), protection from evil, protection from law, true strike(S); Preparation Ritual - Eyes of Rul Thaven") }, // um_equip_general.lst:21
    EquipmentTableEntry { key: "Journal of the Beast Within", category: EquipmentCategory::General, name: "Journal of the Beast Within", cost_gp: Some(2165.0_f64), weight_lbs: None, description: Some("Level 10 Alchemist; Formulae - 4.dragon's breath; 3.beast shape I, rage, tongues, water breathing; 2.barkskin, bull's strength, cure moderate wounds, elemental touch, resist energy; 1.bomber's eye, comprehend languages, crafter's fortune, cure light wounds, disguise self, enlarge person, identify, shield; Preparation Ritual - Beastly Concoction") }, // um_equip_general.lst:22
    EquipmentTableEntry { key: "Manuscript of Jack Were-Son", category: EquipmentCategory::General, name: "Manuscript of Jack Were-Son", cost_gp: Some(2835.0_f64), weight_lbs: None, description: Some("Level 10 Universalist; Protection - Average lock with arcane lock (DC 35), explosive runes (Reflex DC 16).; Spells - 5.break enchantment, mage's private sanctum, polymorph, treasure stitching; 4.charm monster, dimension door, greater invisibility, moonstruck, true form; 3.cloak of winds, explosive runes, haste, lightning bolt, nondetection, twilight knife; 2.acid arrow, burning gaze, dust of twilight, fog cloud, metabolize, protection from arrows, spider climb; 1.alter winds, ant haul, burning hands, cause fear, hold portal, identify, magic missile, sleep, stone fist, vanish") }, // um_equip_general.lst:23
    EquipmentTableEntry { key: "Arctic Call", category: EquipmentCategory::General, name: "Arctic Call", cost_gp: Some(5985.0_f64), weight_lbs: Some(6.0_f64), description: Some("Level 11 Evoker; Protection - fire trap modified to do cold damage (Reflex DC 17); Opposition Schools - Illusion, necromancy; Spells - 6.contingency(S), freezing sphere(S), repulsion; 5.cone of cold(S), mage's private sanctum, permanency, teleport, wall of force(S); 4.ice storm(S), shout(S), stoneskin, symbol of revelation, wall of ice(S); 3.arcane sight, dispel magic, elemental aura(S), fire trap, heroism, lightning bolt(S), phantom steed, seek thoughts; 2.bull's strength, false life(OP), glitterdust, hideous laughter, make whole, resist energy, rope trick; 1.detect secret doors, endure elements, enlarge person, feather fall, mage armor, magic missile(S), obscuring mist, shield, shocking grasp(S); Preparation Ritual - Rime Bite") }, // um_equip_general.lst:24
    EquipmentTableEntry { key: "Insights of Far-Seeing Taernis", category: EquipmentCategory::General, name: "Insights of Far-Seeing Taernis", cost_gp: Some(6355.0_f64), weight_lbs: Some(6.0_f64), description: Some("Level 12 Conjurer; Protection - Sepia snake sigil (Reflex DC 16); Opposition Schools - Divination, necromancy; Spells - 6.getaway(S), planar binding (S), summon monster VI(S), wall of iron(S); 5.contact other plane(OP), life bubble, overland flight, teleport(S); 4.acid pit(S), black tentacles(S), dimensional anchor, lesser geas, phantasmal killer, solid fog(S); 3.gaseous form, greater magic weapon, magic circle against evil, sepia snake sigil(S), spiked pit(S), stinking cloud, tiny hut; 2.arrow eruption(S), daze monster, dust of twilight(S), fantastic reach, glitterdust(S), metabolize(S), shatter, stone call(S); 1.comprehend languages(OP), disguise self, jump, mage armor(S), magic aura, mount(S), sleep, stumble gap(S), unseen servant(S); Preparation Ritual - Sturdy Summoning") }, // um_equip_general.lst:25
    EquipmentTableEntry { key: "Quest Eternal", category: EquipmentCategory::General, name: "Quest Eternal", cost_gp: Some(8395.0_f64), weight_lbs: Some(6.0_f64), description: Some("Level 13 Diviner; Opposition Schools - Conjuration, transmutation; Spells - 7.greater arcane sight(S), greater scrying(S), prismatic spray; 6.chain lightning, contingency, eyebite, legend lore(S), true seeing(S); 5.dominate person, fire snake, prying eyes(S), teleport(OP), telepathic bond(S); 4.arcane eye(S), confusion, detect scrying(S), fear, ice storm, wall of fire; 3.clairaudience/clairvoyance(S), dispel magic, displacement, greater magic weapon(OP), hydraulic torrent, lightning bolt, seek thoughts, tongues(S); 2.arcane lock, false life, fox's cunning(OP), locate object(S), resist energy, scorching ray, see invisibility(S), touch of idiocy; 1.alarm, charm person, comprehend languages(S), color spray, detect secret doors(S), expeditious excavation(OP), floating disk, protection from evil, shield, true strike(S), ventriloquism; Preparation Ritual - Travel Sage") }, // um_equip_general.lst:26
    EquipmentTableEntry { key: "Grandfather's Legacy", category: EquipmentCategory::General, name: "Grandfather's Legacy", cost_gp: Some(7635.0_f64), weight_lbs: Some(6.0_f64), description: Some("Level 14 Necromancy; Protection - A series of three superior locks each augmented with arcane lock (DC 50) and a symbol of weakness (Fortitude DC 20) on the second page of each book.; Opposition Schools - Enchantment, illusion; Spells - 7.control undead(S), finger of death(S), symbol of weakness(S), temporary resurrection(S); 6.acid fog, chain lightning, circle of death(S), create undead(S), unwilling shield(S); 5.dismissal, hold monster, magic jar(S), possess object(S), wall of force, waves of fatigue(S); 4.animate dead(S), bestow curse(S), contagion(S), enervation(S), shadow projection(S), shout, wall of ice; 3.gentle repose(S), halt undead(S), lightning bolt, magic circle against good, nondetection, ray of exhaustion(S), stinking cloud, vampiric touch(S); 2.arcane lock, blindness(S), darkness, false life(S), obscure object, resist energy, scare(S), summon swarm; 1.cause fear(S), chill touch(S), floating disk, hypnotism, obsuring fog, ray of enfeeblement(S), restore corpse(S), sculpt corpse(S), shield, sleep") }, // um_equip_general.lst:27
    EquipmentTableEntry { key: "Chymist's Guidebook", category: EquipmentCategory::General, name: "Chymist's Guidebook", cost_gp: Some(2545.0_f64), weight_lbs: None, description: Some("Level 15 Alchemist; Protection - Good lock (DC 30).; Formulae -5.delayed consumption, magic jar, spell resistance; 4.detonate, freedom of movement, greater invisibility, restoration, stoneskin; 3.amplify elixir, arcane sight, haste, heroism, protection from energy, thorn body; 2.alter self, bear's endurance, false life, fire breath, see invisibility, transmute potion to poison; 1.ant haul, comprehend languages, cure light wounds, detect secret doors, enlarge person, jump, keen senses, negate aroma, shield, true strike") }, // um_equip_general.lst:28
    EquipmentTableEntry { key: "Guardian Grimoire", category: EquipmentCategory::General, name: "Guardian Grimoire", cost_gp: Some(13055.0_f64), weight_lbs: Some(9.0_f64), description: Some("Level 15 Abjurer; Protection - Explosive runes (Reflex DC 26) and fire trap (Reflex DC 17); Opposition Schools - Illusion, necromancy; Spells - 8.binding, maze, protection from spells(S); 7.banishment(S), delayed blast fireball, expend(S), form of the dragon II, grasping hand; 6.analyze dweomer, antimagic field(S), chain lightning, greater dispel magic(S), true seeing; 5.break enchantment(S), cone of cold, dismissal(S), polymorph, summon monster V, wall of force; 4.confusion, dimension door, dimensional anchor(S), fire trap(S), mass enlarge person, secure shelter, true form(S); 3.cloak of winds(S), daylight, enter image, explosive runes, haste, lightning bolt, magic circle against evil(S), water breathing; 2.cushioning bands, detect thoughts, glitterdust, knock, protection from arrows(S), resist energy(S), scorching ray, whispering wind; 1.animate rope, detect secret doors, detect undead, endure elements(S), feather fall, obscuring mist, protection from chaos(S), shadow blade(S), shield(S), shocking grasp, touch of gracelessness; Preparation Ritual - Guardian Trick") }, // um_equip_general.lst:29
    EquipmentTableEntry { key: "Mysteries of Shadow", category: EquipmentCategory::General, name: "Mysteries of Shadow", cost_gp: Some(15065.0_f64), weight_lbs: Some(9.0_f64), description: Some("Level 16 Illusionist; Protection - Illusory script (Will DC 16; on failure the suggestion is \"Forget the existence of the descriptions and notes\"), phantom trap (opening any of the books seems to set off a trap); Opposition Schools - Divination, evocation; Spells - 8.greater shadow evocation(S), mind blank, scintillating pattern(S), temporal stasis; 7.finger of death, greater shadow conjuration(S), project image(S), simulacrum(S), spell turning; 6.eyebite, guards and wards, limited wish, permanent image(S), programmed image(S), shadow walk(S), symbol of persuasion; 5.cloudkill, mirage arcana(S), mind fog, nightmare(S), seeming(S), shadow evocation(S), telekinesis; 4.black tentacles, dimension door, greater invisibility(S), lesser globe of invulnerability, phantasmal killer(S), remove curse, scrying(OP), shadow conjuration(S); 3.dispel magic, heroism, illusory script(S), magic circle against good, major image(S), phantom steed, shrink item, slow; 2.arcane lock, darkness(OP), false life, glitterdust, magic mouth(S), mirror image(S), obscure object, phantom trap(S), whispering wind; 1.color spray(S), disguise self(S), endure elements, feather fall, grease, magic aura(S), minor image(S), obscuring mist, ray of enfeeblement, silent image(S), ventriloquism(S); Preparation Ritual - Shadow Knowledge") }, // um_equip_general.lst:30
    EquipmentTableEntry { key: "Master Books of Rul Thaven", category: EquipmentCategory::General, name: "Master Books of Rul Thaven", cost_gp: Some(16550.0_f64), weight_lbs: Some(9.0_f64), description: Some("Level 17 Diviner; Protection - Each book features a superior lock (DC 40); secret page hides the 9th- and 7th-level spells, and the first book is warded with explosive runes (Reflex DC 17).; Opposition Schools - Illusion, transmutation; Spells - 9.foresight(S), freedom; 8.demand, discern location(S), greater prying eyes(S), trap the soul; 7.instant summons, greater arcane sight(S), greater scrying(S), plane shift, vision(S); 6.analyze dweomer(S), greater dispel magic, guards and wards, legend lore(S), repulsion, true seeing(S); 5.contact other plane(S), major creation, secret chest, sending, telepathic bond(S), planar adaptation(OP); 4.bestow curse, dimensional anchor, lesser geas, lesser globe of invulnerability, locate creature(S), remove curse, secure shelter, share senses(S); 3.arcane sight(S), blood biography(S), clairaudience/clairvoyance(S), explosive runes, secret page(OP), seek senses(S), tongues(S); 2.continual flame, detect thoughts(S), flaming sphere, hold person, locate object(S), resist energy, see invisibility(S), summon monster II; 1.cause fear, comprehend languages(S), detect secret doors(S), detect undead(S), disguise self(OP), identify(S), protection from evil, protection from law, true strike(S); Preparation Ritual - Improved Eyes of Rul Thaven") }, // um_equip_general.lst:31
    EquipmentTableEntry { key: "Manual of Binding", category: EquipmentCategory::General, name: "Manual of Binding", cost_gp: Some(21215.0_f64), weight_lbs: Some(9.0_f64), description: Some("Level 18 Conjurer; Protection - magic aura (spellbook appears nonmagical), secret page hides the notes on constructing a lich's phylactery, sepia snake sigil (Reflex DC 17); Opposition Schools - Abjuration, enchantment; Spells - 9.gate(S), soul bind, summon monster IX(S), time stop, wish; 8.create greater undead, horrid wilting, greater planar binding(S), maze(S), screen; 7.banishment(OP), finger of death, grasping hand, greater teleport(S), reverse gravity, summon monster VII(S); 6.contagious flames, disintegrate(S), greater dispel magic(OP), legend lore, planar binding(S), wall of iron(S); 5.cloudkill(S), cone of cold, contact other plane, hungry pit(S), magic jar, overland flight, permanency, secret chest(S), symbol of pain; 4.ball lightning, black tentacles(S), detect scrying, dimension door(S), enervation, greater invisibility, resilient sphere, remove curse(OP); 3.blink, daylight, enter image, fireball, gentle repose, secret page, sepia snake sigil(S), stinking cloud(S); 2.darkvision, false life, glitterdust(S), make whole, mirror image, resist energy(OP), see invisibility, summon swarm(S), web(S); 1.burning hands, detect undead, disguise self, grease(S), mage armor(S), magic aura, magic missile, ray of enfeeblement, unseen servant(S); Preparation Ritual - Revivifying Contingency") }, // um_equip_general.lst:32
    EquipmentTableEntry { key: "Library of the Dancer of Skins", category: EquipmentCategory::General, name: "Library of the Dancer of Skins", cost_gp: Some(20710.0_f64), weight_lbs: Some(12.0_f64), description: Some("Level 19 Universalist; Spells - 9.shapechange, weird, wish, world wave; 8.giant form II, polar ray, scintillating pattern, seamantle, trap the soul; 7.control weather, elemental body IV, phantasmal revenge, power word blind, rampart, spell turning; 6.beast shape IV, cloak of dreams, control water, fluid form, getaway, lightning field, plant shape II; 5.comet, fabricate, fire snake, geyser, hold monster, overland flight, planar adaptation, telekinesis; 4.dragon's breath, dimension door, firefall, greater invisibility, moonstruck, resilient sphere, true form, wandering star motes; 3.blink, cloak of winds, elemental aura, fly, lightwall, shifting sands, twilight knife, versatile weapon; 2.accelerate poison, arrow eruption, dust of twilight, elemental speech, alter self, fire breath, share language, slipstream; 1.alter winds, ant haul, endure elements, feather fall, flare burst, identify, stone fist, touch of the sea, true strike, vanish; Preparation Ritual - Defensive Boon") }, // um_equip_general.lst:33
    EquipmentTableEntry { key: "The Formulae of Master Gebr", category: EquipmentCategory::General, name: "The Formulae of Master Gebr", cost_gp: Some(11115.0_f64), weight_lbs: Some(12.0_f64), description: Some("Level 20 alchemist; Protection - The cover is treated with tears of death poison (contact; save Fort DC 22; onset 1 minute; frequency 1/minute for 6 minutes; effect 1d6 Con damage and paralyzed for 1 minute).; Formulae - 6.elemental body III, form of the dragon I, heal, shadow walk, transformation, twin form; 5.delayed consumption, dream, elude time, nightmare, overland flight, resurgent transformation, spell resistance; 4.detonate, dragon's breath, fire shield, fluid form, greater invisibility, neutralize poison, stoneskin, universal formula; 3.absorbing touch, amplify elixir, bloodhound, displacement, draconic reservoir, haste, heroism, seek thoughts, thorn body; 2.alchemical allocation, barkskin, bull's strength, cat's grace, elemental touch, fire breath, perceive cues, see invisibility, transmute potion to poison, vomit swarm; 1.bomber's eye, comprehend languages, crafter's fortune, disguise self, endure elements, keen senses, negate aroma, shield, stone fist, touch of the sea; Preparation Ritual - Spontaneous Bomb") }, // um_equip_general.lst:34
    EquipmentTableEntry { key: "Mastery of Word and Thought", category: EquipmentCategory::General, name: "Mastery of Word and Thought", cost_gp: Some(27265.0_f64), weight_lbs: Some(12.0_f64), description: Some("Level 20 enchanter; Protection - Symbol of death on the first page of the book (Fort DC 22); Opposition Schools - Conjuration, illusion; Spells - 9.dominate monster(S), foresight, mage's disjunction, mass hold monster(S), mass suffocation, power word kill(S), time stop; 8.binding(S), clone, irresistible dance(S), mind blank, polymorph any object, power word stun(S), sunburst; 7.deflection, greater arcane sight, instant summons(OP), limited wish, mass hold person(S), plane shift(OP), power word blind(S), resonating word, symbol of stunning(S); 6.cloak of dreams(S), contingency, enemy hammer, forceful hand, greater dispel magic, geas/quest(S), legend lore, mage's lucubration, true seeing; 5.dismissal, dominate person(S), fabricate, icy prison, feeblemind(S), mage's private sanctum, permanency, prying eyes, teleport(OP); 4.bestow curse, crushing despair(S), dimensional anchor, fear, moonstruck(S), resilient sphere, stone shape, stoneskin; 3.fireball, fly, greater magic weapon, heroism(S), loathsome veil, nondetection, protection from energy, symbol of peace(S), suggestion(S); 2.arcane lock, continual flame, false life, hideous laughter(S), magic mouth(OP), mirror image(OP), resist energy, rope trick, see invisibility, touch of idiocy(S); 1.alarm, charm person(S), feather fall, forced quiet, hydraulic push, identify, magic missile, memory lapse(S), ray of enfeeblement, shield, unseen servant(OP); Preparation Ritual - Curse of Names") }, // um_equip_general.lst:35
];

const ARMS_ARMOR_TABLE: &[EquipmentTableEntry] = &[
    EquipmentTableEntry { key: "Scroll Shield", category: EquipmentCategory::ArmsArmor, name: "Scroll Shield", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // um_equip_arms_armor.lst:7
    EquipmentTableEntry { key: "Scroll Blade", category: EquipmentCategory::ArmsArmor, name: "Scroll Blade", cost_gp: Some(0.0_f64), weight_lbs: Some(0.0_f64), description: None }, // um_equip_arms_armor.lst:8
];

pub fn equipment_tables() -> &'static [EquipmentTableEntry] {
    static TABLES: std::sync::OnceLock<Vec<EquipmentTableEntry>> = std::sync::OnceLock::new();
    TABLES.get_or_init(|| {
        let mut all = Vec::with_capacity(GENERAL_TABLE.len() + ARMS_ARMOR_TABLE.len());
        all.extend_from_slice(GENERAL_TABLE);
        all.extend_from_slice(ARMS_ARMOR_TABLE);
        all
    })
}

/// UM has no equipment *modifiers* file at all (confirmed by directory
/// listing) -- this returns an empty slice, matching the shape
/// `equipment_resolver.rs`'s chain expects from every book it includes
/// (`ui`/`ue` both expose one; UM's own is genuinely empty, not omitted).
pub fn equipmod_tables() -> &'static [EquipmentTableEntry] {
    &[]
}

/// Resolves a UM equipment item by key.
pub fn equipment_resolve(key: &str) -> Option<&'static EquipmentTableEntry> {
    equipment_tables().iter().find(|entry| entry.key == key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_has_26_records_24_general_2_arms_armor() {
        assert_eq!(GENERAL_TABLE.len(), 24, "24 real Spellbook.COPY= records in um_equip_general.lst");
        assert_eq!(ARMS_ARMOR_TABLE.len(), 2, "Scroll Shield + Scroll Blade in um_equip_arms_armor.lst");
        assert_eq!(equipment_tables().len(), 26);
    }

    #[test]
    fn keys_are_unique() {
        let mut keys: Vec<&str> = equipment_tables().iter().map(|e| e.key).collect();
        keys.sort_unstable();
        let before = keys.len();
        keys.dedup();
        assert_eq!(keys.len(), before, "every UM equipment key must be unique");
    }

    /// Every record carries a real COST: token (including the two
    /// Scrollmaster Gear rows' real COST:0) -- no record should ever read
    /// `cost_gp: None` for this book.
    #[test]
    fn every_record_carries_a_real_cost() {
        for entry in equipment_tables() {
            assert!(entry.cost_gp.is_some(), "{} must carry a real cost_gp", entry.key);
        }
    }

    /// The Scrollmaster Gear pair carries no SPROP: token at all -- `None`
    /// is the real corpus shape, not an extraction gap. Every General
    /// (spellbook) record DOES carry real SPROP: text.
    #[test]
    fn scrollmaster_gear_has_no_description_every_spellbook_does() {
        for entry in equipment_tables() {
            match entry.category {
                EquipmentCategory::ArmsArmor => {
                    assert!(entry.description.is_none(), "{} (Scrollmaster Gear) must carry no description", entry.key)
                }
                EquipmentCategory::General => {
                    assert!(entry.description.is_some(), "{} (spellbook) must carry a real SPROP-sourced description", entry.key)
                }
            }
        }
    }

    /// Regression guard against the double-count this module's own doc
    /// comment traced and ruled out: the 17 `_pfs/pfs_um_equip_general.lst`
    /// legality-flag rows must never appear as separate table entries.
    #[test]
    fn pfs_legality_overlay_rows_are_not_separate_entries() {
        assert_eq!(
            equipment_tables().len(),
            26,
            "if this ever reads more than 26, the PFS legality-overlay rows have been              double-counted as new records instead of recognized as flags on existing ones"
        );
    }

    /// Field-coverage audit row pins the exact counts named in this
    /// module's own doc comment.
    #[test]
    fn field_coverage_matches_documented_counts() {
        let report = field_coverage_report();
        assert_eq!(report.total_records, 26);
        assert_eq!(report.records_expected, 26);
        assert_eq!(report.has_cost, 26);
        assert_eq!(report.has_weight, 13);
        assert_eq!(report.has_description, 24);
    }
}
