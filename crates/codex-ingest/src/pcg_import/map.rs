//! From `.pcg` lines to the file `import_character` reads.
//!
//! [`MAPPING`] says what happens to each key, and `docs/architecture/pcg-import.md` prints it. A key
//! not in the table, or a name Codex does not hold, becomes a custom record with a report line: a
//! line of the file is never dropped, and every line has exactly one report line.
//!
//! **Rows the file does not hold.** The app saves a character only when the rules engine computes
//! it, and today the engine reads its combat baseline off a held Weapon Focus and each class's
//! default picks, the rows Create gives every new character. Those rows are added here,
//! each with an `Added` report line, rather than the import being refused for lacking them. A spell
//! the engine's class list refuses would refuse the whole character, so it becomes a custom spell.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use codex::rules_core::body_slots::{BodySlot, Enhanceable};
use codex::rules_core::class_seeds::canonical_seeds_for;
use codex::rules_core::corpus_loader::{BookCorpusRoot, load_equipment_corpus};
use codex::rules_core::equipment_effects::stand_in_record;
use codex::rules_core::equipment_resolver::equipment_converted_resolve;
use codex::rules_core::game_system::{GameSystem, runtime_repo_root};
use codex::rules_core::money::gp_to_copper;
use codex::rules_core::settled_corpus::corpus_book_dirs;
use codex::rules_core::size::SizeCategory;
use codex::rules_core::source_content::SourcePackageContent;
use codex::rules_core::character_input::{
    AbilityScores, AcquisitionMode, ActiveState, CharacterClassLevel, CharacterInput, ChosenCharacterState, EquipmentSelection,
    SelectedChoice, SkillAllocation, SpellSelection,
};
use codex::rules_core::feat_effects::chooser_contract_for_feat;
use codex::rules_core::feat_identity;
use codex::rules_core::pilot_compute::build_pilot_headless_receipt;
use codex::rules_core::pilot_compute::sf_abilities::{self, SfAbilityBuild};
use codex::rules_core::pilot_compute::sf_chassis::{self, SfChassisBuild};
use codex::rules_core::pilot_compute::sf_defense::{self, SfBuild};
use codex::rules_core::pilot_compute::sf_loadout::{self, SfLoadout};
use codex::rules_core::pilot_compute::{sf_attack, sf_skills, sf_spells};
use codex::rules_core::sheet_rule::{Ability, SheetRulePackage, offer_selects, split_rule_id};
use codex::saved_character::equipment_sets::{
    CopyFacts, CopyKey, DEFAULT_SET_NAME, EquipmentSet, EquipmentSets, Location, SetEntry, copy_keys, eqmod_option_key, project,
};

use super::catalog::{Catalog, fold, slug};
use super::parse::{Line, Tag, looks_like_a_character, parse};
use super::wire::*;

/// What the importer does with a key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyRule {
    /// A character field; the text says which.
    Field(&'static str),
    /// A custom record: a thing about the character Codex has no field for.
    Record(&'static str),
    /// Codex derives it; the file's value is reported, not read.
    Derived(&'static str),
    /// PCGen's own setup.
    Setting,
}

/// Every key PCGen 6.x writes that the fixtures hold. A key missing here is an unknown key.
pub const MAPPING: &[(&str, KeyRule)] = &[
    ("CLASS", KeyRule::Field("classLevels")),
    ("RACE", KeyRule::Field("raceId")),
    ("STAT", KeyRule::Field("abilityScores, plus the race's adjustments")),
    ("ABILITY", KeyRule::Field("CATEGORY:FEAT to selectedFeats + a target choice; other categories to a custom feat")),
    ("FEAT", KeyRule::Field("selectedFeats + a target choice (the older form of ABILITY)")),
    ("SKILL", KeyRule::Field("skillAllocations, ranks summed over CLASSBOUGHT")),
    ("SPELLNAME", KeyRule::Field("spellsSelected")),
    ("EQUIPNAME", KeyRule::Field("one equipmentSelections row and one equipmentSets stack per place the line is in, its QUANTITY the stack's pieces; BASEITEM first, PLUSnW/A/S to an applied +n modifier; the custom name and every DATA property per CUSTOM_PROPERTIES")),
    ("EQUIPSET", KeyRule::Field("equipmentSets: a slot word to a Slot (PCGEN_SLOTS), Equipped, Carried, Not Carried, or inside the entry above it; activeState is the set's projection")),
    ("CALCEQUIPSET", KeyRule::Field("which EQUIPSET is the set imported, and its name")),
    ("MONEY", KeyRule::Field("money.totalCopper, gp x 100")),
    ("LANGUAGE", KeyRule::Field("the Languages custom feat (the Pathfinder sheet has no language list to fill)")),
    ("ALIGN", KeyRule::Field("bio.alignment")),
    ("DEITY", KeyRule::Field("bio.deity")),
    ("CHARACTERNAME", KeyRule::Field("displayLabel")),
    ("PLAYERNAME", KeyRule::Field("bio.playerName")),
    ("GENDER", KeyRule::Field("bio.sex")),
    ("AGE", KeyRule::Field("bio.age")),
    ("HEIGHT", KeyRule::Field("bio.height")),
    ("WEIGHT", KeyRule::Field("bio.weight")),
    ("HAIRCOLOR", KeyRule::Field("bio.hair")),
    ("EYECOLOR", KeyRule::Field("bio.eyes")),
    ("CHARACTERBIO", KeyRule::Record("Biography")),
    ("CHARACTERDESC", KeyRule::Record("Biography")),
    ("CHARACTERCOMP", KeyRule::Record("Biography")),
    ("CHARACTERASSET", KeyRule::Record("Biography")),
    ("CHARACTERMAGIC", KeyRule::Record("Biography")),
    ("CHARACTERDMNOTES", KeyRule::Record("Biography")),
    ("TABNAME", KeyRule::Record("Biography")),
    ("EXPERIENCE", KeyRule::Record("Biography")),
    ("HANDED", KeyRule::Record("Biography")),
    ("SKINCOLOR", KeyRule::Record("Biography")),
    ("HAIRSTYLE", KeyRule::Record("Biography")),
    ("LOCATION", KeyRule::Record("Biography")),
    ("CITY", KeyRule::Record("Biography")),
    ("BIRTHDAY", KeyRule::Record("Biography")),
    ("BIRTHPLACE", KeyRule::Record("Biography")),
    ("PERSONALITYTRAIT1", KeyRule::Record("Biography")),
    ("PERSONALITYTRAIT2", KeyRule::Record("Biography")),
    ("SPEECHPATTERN", KeyRule::Record("Biography")),
    ("PHOBIAS", KeyRule::Record("Biography")),
    ("INTERESTS", KeyRule::Record("Biography")),
    ("CATCHPHRASE", KeyRule::Record("Biography")),
    ("REGION", KeyRule::Record("Biography")),
    ("NOTE", KeyRule::Record("a custom feat named by the note")),
    ("TEMPLATESAPPLIED", KeyRule::Record("a custom feat per template")),
    ("TEMPLATE", KeyRule::Record("a custom feat per template")),
    ("DOMAIN", KeyRule::Record("a custom feat per domain (the cleric's engine pick stays its default)")),
    ("KIT", KeyRule::Record("a custom feat")),
    ("MASTER", KeyRule::Record("a custom feat")),
    ("FOLLOWER", KeyRule::Record("a custom feat")),
    ("CHRONICLEENTRY", KeyRule::Record("a custom feat")),
    ("TEMPBONUS", KeyRule::Record("a custom feat")),
    ("CLASSABILITIESLEVEL", KeyRule::Derived("hit points and skill points per level")),
    ("WEAPONPROF", KeyRule::Derived("proficiencies")),
    ("FEATPOOL", KeyRule::Derived("feat budget")),
    ("USERPOOL", KeyRule::Derived("ability pools")),
    ("FAVOREDCLASS", KeyRule::Derived("favored class")),
    ("AGESET", KeyRule::Derived("age category")),
    ("SPELLBOOK", KeyRule::Derived("the spell lists")),
    ("SPELLLIST", KeyRule::Derived("the spell lists")),
    ("PCGVERSION", KeyRule::Setting),
    ("VERSION", KeyRule::Setting),
    ("CAMPAIGN", KeyRule::Setting),
    ("CAMPAIGNS", KeyRule::Setting),
    ("GAMEMODE", KeyRule::Setting),
    ("CHARACTERTYPE", KeyRule::Setting),
    ("ROLLMETHOD", KeyRule::Setting),
    ("PURCHASEPOINTS", KeyRule::Setting),
    ("POOLPOINTS", KeyRule::Setting),
    ("POOLPOINTSAVAIL", KeyRule::Setting),
    ("PREVIEWSHEET", KeyRule::Setting),
    ("TABLABEL", KeyRule::Setting),
    ("AUTOSPELLS", KeyRule::Setting),
    ("USEHIGHERKNOWN", KeyRule::Setting),
    ("USEHIGHERPREPPED", KeyRule::Setting),
    ("LOADCOMPANIONS", KeyRule::Setting),
    ("USETEMPMODS", KeyRule::Setting),
    ("SKILLSOUTPUTORDER", KeyRule::Setting),
    ("SKILLFILTER", KeyRule::Setting),
    ("AUTOSORTSKILLS", KeyRule::Setting),
    ("IGNORECOST", KeyRule::Setting),
    ("ALLOWDEBT", KeyRule::Setting),
    ("AUTORESIZEGEAR", KeyRule::Setting),
    ("PORTRAIT", KeyRule::Setting),
    ("PORTRAITTHUMBNAILRECT", KeyRule::Setting),
    ("SUPPRESSBIOFIELDS", KeyRule::Setting),
    ("EXPERIENCETABLE", KeyRule::Setting),
];

const SOURCE_PACKAGE_ID: &str = "pf1.core_rulebook";
/// A Starfinder character's `sourcePackageId` and `gameSystem`: the id the app's Starfinder
/// adapter is routed by (`rule_system_adapter.rs`).
pub const SF_SOURCE_PACKAGE_ID: &str = GameSystem::Starfinder1e.id();
const PF_GAME_SYSTEM: &str = "pf1";
/// The app's Starfinder credits row (`sf_adapter.rs` `MARKET_CREDITS_CHOICE`): credits beyond what
/// the held items' prices explain.
const SF_CREDITS_CHOICE: &str = "market.credits_adjustment";
const WORN: &str = "EquippedActive";
const CARRIED: &str = "SelectedInactive";
const POWER_ATTACK_ROW: &str = "power_attack";
const WEAPON_FOCUS_FEAT: &str = "feat:weapon_focus";
const FIGHTER_BONUS_CHOICE: &str = "choice:fighter_bonus_feat";
const FIGHTER_BONUS_SELECTION: &str = "feat:weapon_focus:weapon:longsword";
const ABILITY_NAMES: [(&str, &str); 6] = [
    ("STR", "strength"),
    ("DEX", "dexterity"),
    ("CON", "constitution"),
    ("INT", "intelligence"),
    ("WIS", "wisdom"),
    ("CHA", "charisma"),
];

/// `custom.json`'s limits (`character_custom.rs`): a record past them refuses the whole file.
const MAX_RECORDS: usize = 200;
const MAX_NAME: usize = 120;
const MAX_DESCRIPTION: usize = 10_000;
const MAX_STATS: usize = 40;
const MAX_STAT_TEXT: usize = 400;

/// PCGen's location words with a place on the body: the 26 `EQSLOT` names of the pinned oracle's
/// `system/gameModes/Pathfinder/equipmentslots.lst` (as `data/equipment_types.json` maps them), then
/// the hand and natural-weapon words of PCGen's `Constants.java`. `BodySlot::None` reads as
/// Equipped; a word in no row reads as Equipped with one warning per import (`decisions.md` SD-a).
pub const PCGEN_SLOTS: &[(&str, BodySlot)] = &[
    ("Armor", BodySlot::Armor),
    ("Belt", BodySlot::Belt),
    ("Body", BodySlot::Body),
    ("Chest", BodySlot::Chest),
    ("Eyes", BodySlot::Eyes),
    ("Feet", BodySlot::Feet),
    ("Hands", BodySlot::Hands),
    ("Head", BodySlot::Head),
    ("Headband", BodySlot::Headband),
    ("Neck", BodySlot::Neck),
    ("Ring", BodySlot::Ring),
    ("Shield", BodySlot::Shield),
    ("Shoulders", BodySlot::Shoulders),
    ("Wrist", BodySlot::Wrist),
    ("Torso", BodySlot::None),
    ("Weapon", BodySlot::WieldMain),
    ("Tattoo Head", BodySlot::None),
    ("Tattoo Shoulders", BodySlot::None),
    ("Tattoo Neck", BodySlot::None),
    ("Tattoo Chest", BodySlot::None),
    ("Tattoo Body", BodySlot::None),
    ("Tattoo Belt", BodySlot::None),
    ("Tattoo Wrist", BodySlot::None),
    ("Tattoo Hands", BodySlot::None),
    ("Tattoo Ring", BodySlot::None),
    ("Tattoo Feet", BodySlot::None),
    ("Primary Hand", BodySlot::WieldMain),
    ("Secondary Hand", BodySlot::WieldOff),
    ("Both Hands", BodySlot::WieldTwoHanded),
    ("Double Weapon", BodySlot::WieldTwoHanded),
    ("Two Weapons", BodySlot::WieldMain),
    ("Unarmed", BodySlot::None),
    ("Natural-Primary", BodySlot::None),
    ("Natural-Secondary", BodySlot::None),
];

/// What an `EQUIPNAME` line's customisation becomes (`decisions.md` §6). A `DATA` key in no row, and
/// an eqmod Buy Custom has no option for, is a property line and a warning (SD-e).
pub const CUSTOM_PROPERTIES: &[(&str, &str)] = &[
    ("NAME", "CopyFacts.name, when it is not the record's own name"),
    ("EQMOD PLUSnW/A/S", "applied_modifiers, the copy's +n"),
    ("EQMOD material, masterwork, special ability", "eqmods: the Buy Custom option key (equipment_sets::eqmod_option_key)"),
    ("EQMOD / ALTEQMOD _DAMAGE|<die>", "CopyFacts.damage_override, the die as written"),
    ("SIZE", "CopyFacts.size (Medium is no size)"),
    ("SPROP", "CopyFacts.properties, one line per '; '-separated part"),
    ("COST (line)", "CopyFacts.cost_override_copper, when it differs from the record's price; COSTMOD is already in it"),
    ("WT (line)", "CopyFacts.weight_override_lbs, when it differs from the record's weight"),
    ("KEY", "a lookup candidate for the record"),
];

/// Converts the text of one `.pcg`. Refuses only a file that is not a PCGen character.
pub fn convert(text: &str, source_file: &str, catalog: &Catalog) -> Result<ImportFile, String> {
    let lines = parse(text);
    if !looks_like_a_character(&lines) {
        return Err(format!(
            "{source_file} is not a PCGen 6 character: it has no PCGVERSION or CHARACTERNAME line (PCGen 5 and older wrote another format)"
        ));
    }
    // `decisions.md §8`: the game mode picks the catalog, so a Starfinder file reads Starfinder records.
    let game_mode = lines.iter().find(|line| line.key() == "GAMEMODE").map(|line| line.value().to_owned()).unwrap_or_default();
    let catalog = catalog.for_game_mode(&game_mode)?;
    let mut importer = Importer::new(source_file, catalog);
    importer.read_equipment_sets(&lines);
    for line in &lines {
        importer.read(line);
    }
    let file = importer.finish(&lines);
    // `decisions.md §21` Q-A: a companion file (a `MASTER` line: `sf_mechanic_drone`'s drone) imports
    // as its own character only when the Starfinder engine computes it as one; Codex computes a drone
    // on its master's sheet (`sf_drone_print`), so otherwise the file is refused by name.
    if let (Some(package), Some(master)) = (catalog.starfinder_package(), lines.iter().find(|line| line.key() == "MASTER"))
        && let Err(reason) = starfinder_verdict(package, &character_input_of(&file))
    {
        let kind = master.get("TYPE").unwrap_or("companion");
        let master_file = master.get("FILE").unwrap_or("its master's file");
        return Err(format!(
            "{source_file} is a {kind} file, a companion of {}: Codex computes a {kind} on its master's sheet, so import the master ({master_file}). The Starfinder engine does not compute it alone: {reason}",
            master.value()
        ));
    }
    Ok(file)
}

/// Some of an `EQUIPNAME` line's pieces in one place of the imported set: one stack.
struct Placement {
    /// The `EQUIPSET` entry's `ID`; `None` for pieces no entry places (Not Carried).
    entry: Option<String>,
    /// The top-level location word (`Armor`, `Carried`, …); empty inside a container.
    word: String,
    /// The entry this one is inside.
    parent: Option<String>,
    pieces: u32,
    /// One piece's weight, the line's `WT`; what a Create item container holds is summed from it.
    weight_lbs: f64,
    /// The stack once the line is read: a row of `equipment`, or a custom record's id.
    held: Option<Held>,
    /// The line's custom name and properties, the same in every place it is in.
    facts: CopyFacts,
}

enum Held {
    Row(usize),
    Custom(String),
}

/// What [`Importer::customised`] reads from one mapped line.
#[derive(Default)]
struct Customised {
    facts: CopyFacts,
    modifiers: Vec<String>,
    eqmods: Vec<String>,
}

/// The Notes block's sub-headings, in the order the block prints them (`decisions.md §7`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum NoteKind {
    Item,
    Property,
    NoCopies,
    UnknownKey,
    Class,
    Race,
    Spell,
    Eqmod,
    Location,
    CustomFeat,
}

impl NoteKind {
    fn heading(self) -> &'static str {
        match self {
            NoteKind::Item => "Items with no Codex record",
            NoteKind::Property => "Properties kept as text",
            NoteKind::NoCopies => "Lines with 0 copies",
            NoteKind::UnknownKey => "PCGen keys with no mapping",
            NoteKind::Class => "Classes with no Codex record",
            NoteKind::Race => "Race with no Codex record",
            NoteKind::Spell => "Spells not kept",
            NoteKind::Eqmod => "Eqmods kept as text",
            NoteKind::Location => "Locations read as Equipped",
            NoteKind::CustomFeat => "Lines kept as custom feats",
        }
    }
}

/// A spell the file names and Codex holds, until the engine has had its say.
struct FileSpell {
    wire: SpellWire,
    class: String,
    level: Option<String>,
    book: Option<String>,
    report_line: usize,
}

struct Importer<'a> {
    catalog: &'a Catalog,
    report: Report,
    label: String,
    race_name: String,
    classes: Vec<(String, u8)>,
    scores: BTreeMap<&'static str, i16>,
    floating: Vec<&'static str>,
    feats: Vec<String>,
    choices: Vec<ChoiceWire>,
    skills: Vec<SkillWire>,
    equipment: Vec<EquipmentWire>,
    spells: Vec<SpellWire>,
    file_spells: Vec<FileSpell>,
    languages: Vec<String>,
    bio: BioWire,
    money: MoneyWire,
    biography: Vec<(String, String)>,
    custom: CustomWire,
    /// Each `EQUIPNAME` line's places, in file order, and the next line's index.
    placements: Vec<Vec<Placement>>,
    next_item: usize,
    set_name: String,
    /// The Notes block's lines: kind, the report line they come from (file order), text.
    notes: Vec<(NoteKind, usize, String)>,
    /// Starfinder's ability increases as the file states them: (ability, the +n PCGen applied).
    increases: Vec<(&'static str, i16)>,
    /// Starfinder picks read from selection lines, until the engine has had its say: the pick, its
    /// line and the report line it wrote.
    picks: Vec<(ChoiceWire, Line, usize)>,
}

impl<'a> Importer<'a> {
    fn new(source_file: &str, catalog: &'a Catalog) -> Self {
        Importer {
            catalog,
            report: Report { source_file: source_file.to_owned(), ..Report::default() },
            label: String::new(),
            race_name: String::new(),
            classes: Vec::new(),
            scores: BTreeMap::new(),
            floating: Vec::new(),
            feats: Vec::new(),
            choices: Vec::new(),
            skills: Vec::new(),
            equipment: Vec::new(),
            spells: Vec::new(),
            file_spells: Vec::new(),
            languages: Vec::new(),
            bio: BioWire::default(),
            money: MoneyWire::default(),
            biography: Vec::new(),
            custom: CustomWire::default(),
            placements: Vec::new(),
            next_item: 0,
            set_name: DEFAULT_SET_NAME.to_owned(),
            notes: Vec::new(),
            increases: Vec::new(),
            picks: Vec::new(),
        }
    }

    /// EQUIPSET lines may sit before or after what they name, so they are read first. Each entry of
    /// the imported set (`CALCEQUIPSET`) is matched to the `EQUIPNAME` line it names (by name, then
    /// by the line's `KEY` or `BASEITEM`) and takes up to that line's pieces; what no entry places
    /// is Not Carried (Carried in a file with no set). One line in two places is two stacks (`decisions.md` SD-c).
    fn read_equipment_sets(&mut self, lines: &[Line]) {
        let worn_set = lines
            .iter()
            .find(|line| line.key() == "CALCEQUIPSET")
            .map(|line| line.value().to_owned())
            .filter(|set| !set.is_empty())
            .or_else(|| {
                lines
                    .iter()
                    .filter(|line| line.key() == "EQUIPSET")
                    .filter_map(|line| line.get("ID"))
                    .find(|id| id.split('.').count() == 2)
                    .map(str::to_owned)
            })
            .unwrap_or_default();
        if let Some(name) = lines
            .iter()
            .filter(|line| line.key() == "EQUIPSET" && line.get("ID") == Some(worn_set.as_str()))
            .map(|line| line.value().trim())
            .find(|name| !name.is_empty())
        {
            self.set_name = name.chars().take(codex::saved_character::equipment_sets::SET_NAME_MAX_CHARS).collect();
        }
        let items: Vec<(String, Vec<String>)> = lines
            .iter()
            .filter(|line| line.key() == "EQUIPNAME")
            .map(|line| {
                let (base_item, data) = customization(line);
                let others = [data.get("KEY").cloned(), base_item].into_iter().flatten().map(|name| fold(&name)).collect();
                (fold(line.value()), others)
            })
            .collect();
        let mut left: Vec<u32> = lines.iter().filter(|line| line.key() == "EQUIPNAME").map(|line| quantity(line.get("QUANTITY"))).collect();
        let weights: Vec<f64> = lines.iter().filter(|line| line.key() == "EQUIPNAME").map(|line| amount(line.get("WT")).unwrap_or(0.0)).collect();
        self.placements = items.iter().map(|_| Vec::new()).collect();
        let top_level = worn_set.split('.').count() + 1;
        for line in lines.iter().filter(|line| line.key() == "EQUIPSET") {
            let (Some(id), Some(value)) = (line.get("ID"), line.get("VALUE")) else { continue };
            if !id.starts_with(&format!("{worn_set}.")) {
                continue;
            }
            let value = fold(value);
            let by_name = items.iter().enumerate().position(|(at, (name, _))| *name == value && left[at] > 0);
            let by_key = || items.iter().enumerate().position(|(at, (_, others))| others.contains(&value) && left[at] > 0);
            let Some(at) = by_name.or_else(by_key) else {
                self.report.warnings.push(format!("EQUIPSET {id} ({}) names no line with pieces left; not read", line.get("VALUE").unwrap_or_default()));
                continue;
            };
            let pieces = quantity(line.get("QUANTITY")).min(left[at]);
            left[at] -= pieces;
            let depth = id.split('.').count();
            self.placements[at].push(Placement {
                entry: Some(id.to_owned()),
                word: if depth == top_level { line.value().trim().to_owned() } else { String::new() },
                parent: (depth > top_level).then(|| id.rsplit_once('.').map_or_else(String::new, |(parent, _)| parent.to_owned())),
                pieces,
                weight_lbs: weights[at],
                held: None,
                facts: CopyFacts::default(),
            });
        }
        // PCGen counts only the set's entries as carried (its export lists nothing else): pieces no
        // entry places are Not Carried, unless the file states no set at all.
        let has_set = lines.iter().any(|line| line.key() == "EQUIPSET" && line.get("ID").is_some_and(|id| id.starts_with(&format!("{worn_set}."))));
        let unplaced = if has_set { "Not Carried" } else { "Carried" };
        for ((places, rest), weight_lbs) in self.placements.iter_mut().zip(left).zip(weights) {
            if rest > 0 {
                places.push(Placement { entry: None, word: unplaced.to_owned(), parent: None, pieces: rest, weight_lbs, held: None, facts: CopyFacts::default() });
            }
        }
    }

    fn read(&mut self, line: &Line) {
        let key = line.key().to_owned();
        match key.as_str() {
            "CHARACTERNAME" => {
                self.label = line.value().to_owned();
                self.mapped(line, "displayLabel");
            }
            "CLASS" => self.read_class(line),
            "RACE" => self.read_race(line),
            "STAT" => self.read_stat(line),
            "ABILITY" => self.read_ability(line),
            "FEAT" => self.read_old_feat(line),
            "SKILL" => self.read_skill(line),
            "SPELLNAME" => self.read_spell(line),
            "EQUIPNAME" => self.read_item(line),
            "MONEY" => self.read_money(line),
            "LANGUAGE" => {
                self.languages.extend(line.tags.iter().filter(|tag| tag.key == "LANGUAGE").map(|tag| tag.value.clone()));
                self.mapped(line, "the Languages custom feat");
            }
            "ALIGN" => {
                self.bio.alignment = alignment_name(line.value());
                self.mapped(line, "bio.alignment");
            }
            "DEITY" => {
                // PCGen's word for no deity. The bio's Deity is read by the rules, so it stays empty.
                if line.value() == "None" {
                    self.note(line, Outcome::Derived, "no deity");
                } else {
                    self.bio.deity = line.value().to_owned();
                    self.mapped(line, "bio.deity");
                }
            }
            "PLAYERNAME" => self.bio_field(line, "bio.playerName", |bio| &mut bio.player_name),
            "GENDER" => self.bio_field(line, "bio.sex", |bio| &mut bio.sex),
            "AGE" => self.bio_field(line, "bio.age", |bio| &mut bio.age),
            "HEIGHT" => self.bio_field(line, "bio.height", |bio| &mut bio.height),
            "WEIGHT" => self.bio_field(line, "bio.weight", |bio| &mut bio.weight),
            "HAIRCOLOR" => self.bio_field(line, "bio.hair", |bio| &mut bio.hair),
            "EYECOLOR" => self.bio_field(line, "bio.eyes", |bio| &mut bio.eyes),
            "EQUIPSET" | "CALCEQUIPSET" => self.mapped(line, "activeState"),
            _ => self.read_by_rule(line, &key),
        }
    }

    fn read_by_rule(&mut self, line: &Line, key: &str) {
        match MAPPING.iter().find(|(name, _)| *name == key).map(|(_, rule)| *rule) {
            Some(KeyRule::Setting) => self.note(line, Outcome::Setting, ""),
            Some(KeyRule::Derived(what)) => self.note(line, Outcome::Derived, what),
            Some(KeyRule::Record("Biography")) => {
                if !line.value().trim().is_empty() {
                    self.biography.push((key.to_owned(), line.value().trim().to_owned()));
                }
                self.note(line, Outcome::Custom, "the Biography custom feat");
            }
            Some(KeyRule::Record(_)) => self.custom_feat(line, "Imported from PCGen."),
            Some(KeyRule::Field(_)) | None => {
                if !self.report.unknown_keys.iter().any(|known| known == key) {
                    self.report.unknown_keys.push(key.to_owned());
                }
                self.unmapped(NoteKind::UnknownKey, format!("{key}: kept as a custom feat"));
                self.custom_feat(line, "A PCGen line Codex has no mapping for.");
            }
        }
    }

    fn read_class(&mut self, line: &Line) {
        let level: u8 = line.get("LEVEL").and_then(|level| level.trim().parse().ok()).unwrap_or(0);
        let name = line.value().to_owned();
        if level == 0 {
            self.note(line, Outcome::Derived, "a class held at level 0 grants nothing");
            return;
        }
        match self.catalog.class_id(&name) {
            Some(id) => {
                self.classes.push((id.to_owned(), level));
                if let Some(subclass) = line.get("SUBCLASS").filter(|value| *value != "None" && *value != name) {
                    self.report.warnings.push(format!(
                        "{name} {level}: PCGen's subclass {subclass} is not read; the class's picks are Codex's defaults"
                    ));
                }
                let to = format!("{id} {level}");
                self.mapped(line, &to);
            }
            None => {
                self.report.unmapped_classes.push(format!("{name} {level}"));
                self.unmapped(NoteKind::Class, format!("{name} {level}: kept as a custom feat"));
                self.custom_feat(line, "A class Codex has no record of.");
            }
        }
    }

    fn read_race(&mut self, line: &Line) {
        self.race_name = line.value().to_owned();
        match self.catalog.race(&self.race_name).map(|race| race.id.clone()) {
            Some(id) => self.mapped(line, &id),
            None => {
                self.report.unmapped_race = Some(self.race_name.clone());
                self.unmapped(NoteKind::Race, format!("{}: kept as a custom feat; no racial ability adjustment applied", self.race_name));
                self.custom_feat(line, "A race Codex has no record of; no racial ability adjustment was applied.");
            }
        }
    }

    fn read_stat(&mut self, line: &Line) {
        let target = ABILITY_NAMES.iter().find(|(short, _)| *short == line.value()).map(|(_, long)| *long);
        let score = line.get("SCORE").and_then(|score| score.trim().parse::<i16>().ok());
        match (target, score) {
            (Some(ability), Some(score)) => {
                self.scores.insert(ability, score);
                self.mapped(line, &format!("abilityScores.{ability}"));
            }
            _ => self.custom_feat(line, "A statistic Pathfinder has no such score for."),
        }
    }

    fn read_ability(&mut self, line: &Line) {
        let name = line.get("KEY").unwrap_or(line.value()).to_owned();
        let category = line.get("CATEGORY").unwrap_or_default().to_owned();
        if category == "FEAT" {
            let targets = applied_to(line);
            self.read_feat(line, &name, &targets);
            return;
        }
        if self.catalog.starfinder_package().is_some() && (self.read_starfinder_ability(line, &name, &category) || self.read_starfinder_pick(line, &name)) {
            return;
        }
        if let Some(ability) = floating_bonus_ability(&name) {
            self.floating.push(ability);
            self.mapped(line, &format!("abilityScores.{ability} +2"));
            return;
        }
        if line.get("TYPE") == Some("VIRTUAL") || line.get("TYPE") == Some("AUTOMATIC") {
            self.note(line, Outcome::Derived, "granted by PCGen's rules");
            return;
        }
        let description = line.get("DESC").unwrap_or("Imported from PCGen.").to_owned();
        self.custom_named_feat(line, &name, &description);
    }

    /// A Starfinder theme, a class's key-ability choice (`Soldier Key Ability ~ DEX`) or an ability
    /// increase (`PC Level 5 Stat Boost ~ CON +2`). False for any other line.
    fn read_starfinder_ability(&mut self, line: &Line, name: &str, category: &str) -> bool {
        if category == "Theme" {
            let Some(id) = self.catalog.theme_id(name).map(str::to_owned) else { return false };
            self.feats.push(id.clone());
            self.mapped(line, &format!("selectedFeats {id} (theme)"));
            return true;
        }
        let Some((left, right)) = name.split_once(" ~ ") else { return false };
        let ability = |word: &str| ABILITY_NAMES.iter().find(|(short, _)| short.eq_ignore_ascii_case(word)).map(|(short, long)| (*short, *long));
        if let Some(class) = left.strip_suffix(" Key Ability") {
            let (Some(class_id), Some((short, _))) = (self.catalog.class_id(class).map(str::to_owned), ability(right.trim())) else { return false };
            self.choices.push(ChoiceWire { choice_set_id: class_id.clone(), selection_id: short.to_owned() });
            self.mapped(line, &format!("selectedChoices {class_id} {short}"));
            return true;
        }
        if left.starts_with("PC Level ") && left.ends_with(" Stat Boost") {
            let mut words = right.split_whitespace();
            let (Some((_, long)), Some(Ok(plus))) = (words.next().and_then(ability), words.next().map(|n| n.trim_start_matches('+').parse::<i16>())) else {
                return false;
            };
            self.increases.push((long, plus));
            self.mapped(line, &format!("abilityScores.{long} +{plus}"));
            return true;
        }
        false
    }

    /// A Starfinder selection line (`PRIMARY FIGHTING STYLE ~ Blitz`; `Opening Volley Choice` keyed
    /// `Choose Combat Feat`) is the pick the app's choices dialog makes: the option record the key
    /// names (alone, or after the line's own name) under the first rule that offers it. False when
    /// no record or no offer matches.
    fn read_starfinder_pick(&mut self, line: &Line, key: &str) -> bool {
        let Some(package) = self.catalog.starfinder_package() else { return false };
        let wanted = [slug(key), slug(&format!("{} {key}", line.value()))];
        let Some(option) = package
            .rules
            .values()
            .filter(|rule| !rule.id.contains('#') && wanted.iter().any(|w| split_rule_id(&rule.id).2 == w))
            .min_by_key(|rule| !rule.id.starts_with("core:"))
        else {
            return false;
        };
        let Some(slot) = package.rules.values().find(|rule| rule.offers.as_ref().is_some_and(|offer| offer_selects(offer, option))) else {
            return false;
        };
        let pick = ChoiceWire { choice_set_id: slot.id.clone(), selection_id: option.id.clone() };
        self.mapped(line, &format!("selectedChoices {} {}", pick.choice_set_id, pick.selection_id));
        self.picks.push((pick.clone(), line.clone(), self.report.lines.len() - 1));
        self.choices.push(pick);
        true
    }

    /// A pick the Starfinder engine refuses would refuse the whole character, so picks are undone,
    /// last first, until its verdict is the one it gives without them (the credits the file states
    /// are held after this); each undone line is a custom feat, as before picks were read.
    fn keep_the_picks_the_engine_takes(&mut self, package: &SheetRulePackage, race_id: &str, class_levels: &[ClassLevelWire], scores: &AbilitiesWire) {
        let verdict = |this: &Self| starfinder_verdict(package, &engine_input(&this.chosen_so_far(race_id, class_levels, scores)));
        let mut without = self.chosen_so_far(race_id, class_levels, scores);
        without.selected_choices.retain(|choice| !self.picks.iter().any(|(pick, _, _)| pick == choice));
        let unpicked = starfinder_verdict(package, &engine_input(&without));
        for (pick, line, report_line) in std::mem::take(&mut self.picks).into_iter().rev() {
            if verdict(self) == unpicked {
                break;
            }
            self.choices.retain(|choice| *choice != pick);
            let name = line.get("KEY").unwrap_or(line.value()).to_owned();
            self.custom_named_feat(&line, &name, line.get("DESC").unwrap_or("Imported from PCGen."));
            // The custom feat wrote a new report line; it replaces the pick's own.
            let written = self.report.lines.pop().expect("the custom feat's report line");
            self.report.lines[report_line] = written;
            self.report.warnings.push(format!("{name}: the Starfinder engine refuses the pick, so it is a custom feat"));
        }
    }

    /// The older form: `FEAT:Weapon Focus (Longsword)|...`, or a separate `APPLIEDTO`.
    fn read_old_feat(&mut self, line: &Line) {
        let raw = line.value().to_owned();
        let mut targets = applied_to(line);
        let (name, inline) = split_parenthesised(&raw);
        if self.catalog.feat_key(&raw).is_some() {
            self.read_feat(line, &raw, &targets);
        } else {
            targets.extend(inline.map(str::to_owned));
            self.read_feat(line, name, &targets);
        }
    }

    fn read_feat(&mut self, line: &Line, name: &str, targets: &[String]) {
        let Some(key) = self.catalog.feat_key(name).map(str::to_owned) else {
            let description = line.get("DESC").unwrap_or("A feat Codex has no record of.").to_owned();
            self.custom_named_feat(line, name, &description);
            return;
        };
        let Some(contract) = chooser_contract_for_feat(&key) else {
            self.feats.push(key.clone());
            if !targets.is_empty() {
                self.report.warnings.push(format!("{key}: its target {} has no place to go on this feat", targets.join(", ")));
            }
            self.mapped(line, &format!("selectedFeats {key}"));
            return;
        };
        if targets.is_empty() {
            self.feats.push(key.clone());
        }
        for target in targets {
            self.feats.push(key.clone());
            self.choices.push(ChoiceWire {
                choice_set_id: contract.choice_set_id.to_owned(),
                selection_id: format!("{}{}", contract.selection_prefix, target.trim()),
            });
        }
        self.mapped(line, &format!("selectedFeats {key} {}", targets.join(", ")));
    }

    fn read_skill(&mut self, line: &Line) {
        let name = line.value().to_owned();
        let ranks: f32 = line
            .tags
            .iter()
            .filter(|tag| tag.key == "CLASSBOUGHT")
            .flat_map(Tag::nested)
            .filter(|tag| tag.key == "RANKS")
            .filter_map(|tag| tag.value.trim().parse::<f32>().ok())
            .sum();
        if ranks <= 0.0 {
            self.note(line, Outcome::Derived, "no ranks bought");
            return;
        }
        // A half rank is a 3.5e cross-class rank; Pathfinder has none, so it does not round up.
        let whole = ranks.floor().min(f32::from(u8::MAX)) as u8;
        match self.catalog.skill_id(&name) {
            Some(skill_id) if whole > 0 => {
                let to = format!("{skill_id} {whole}");
                self.skills.push(SkillWire { skill_id, ranks: whole });
                self.mapped(line, &to);
            }
            Some(_) => self.note(line, Outcome::Derived, "a half rank buys nothing in Pathfinder"),
            None => {
                let description = format!("A skill Codex has no record of: {ranks} ranks.");
                self.custom_named_feat(line, &name, &description);
            }
        }
    }

    fn read_spell(&mut self, line: &Line) {
        let name = line.value().to_owned();
        let class = line.get("CLASS").unwrap_or_default();
        let class_id = self.catalog.class_id(class).map(str::to_owned);
        let (true, Some(source_class_id)) = (self.catalog.holds_spell(&name), class_id) else {
            let why = if self.catalog.holds_spell(&name) { "no Codex class to cast it" } else { "no Codex record" };
            self.unmapped(NoteKind::Spell, format!("{name} ({class}): {why}; kept as a custom spell"));
            self.custom_spell(line, &name, class);
            return;
        };
        let granted = line
            .tags
            .iter()
            .filter(|tag| tag.key == "SOURCE")
            .flat_map(Tag::nested)
            .any(|tag| tag.key == "TYPE" && tag.value != "CLASS");
        let mode = if granted {
            AcquisitionMode::Granted
        } else if line.get("BOOK").unwrap_or_default().contains("Prepared") {
            AcquisitionMode::Prepared
        } else {
            AcquisitionMode::Known
        };
        let wire = SpellWire { spell_id: name.clone(), source_class_id: source_class_id.clone(), acquisition_mode: mode_name(mode) };
        self.mapped(line, &format!("spellsSelected {name} ({source_class_id})"));
        self.file_spells.push(FileSpell {
            wire,
            class: class.to_owned(),
            level: line.get("SPELLLEVEL").map(str::to_owned),
            book: line.get("BOOK").map(str::to_owned),
            report_line: self.report.lines.len() - 1,
        });
    }

    fn read_item(&mut self, line: &Line) {
        let index = self.next_item;
        self.next_item += 1;
        let name = line.value().to_owned();
        if quantity(line.get("QUANTITY")) == 0 {
            // `decisions.md` SD-d: nothing is held, and the player is told.
            self.report.warnings.push(format!("{name}: 0 copies in the file"));
            self.unmapped(NoteKind::NoCopies, format!("{name}: 0 copies in the file"));
            self.mapped(line, "not held: 0 copies in the file");
            return;
        }
        let (base_item, data) = customization(line);
        // Worn: a top-level place that is neither Carried nor Not Carried.
        let places = self.placements.get(index).map_or(&[][..], Vec::as_slice);
        let worn: u32 = places.iter().filter(|place| !matches!(place.word.as_str(), "" | "Carried" | "Not Carried")).map(|place| place.pieces).sum();
        let carried = places.iter().map(|place| place.pieces).sum::<u32>() - worn;
        let item_key = [base_item.as_deref(), data.get("KEY").map(String::as_str), Some(name.as_str())]
            .into_iter()
            .flatten()
            .find_map(|candidate| self.catalog.equipment_key(candidate))
            .map(str::to_owned);
        let Some(item_key) = item_key else {
            let before = self.custom.equipment.len();
            self.custom_item(line, &name, base_item.as_deref().unwrap_or(&name), &data);
            if self.custom.equipment.len() > before {
                let id = self.custom.equipment[before].id.clone();
                for place in self.placements.get_mut(index).into_iter().flatten() {
                    place.held = Some(Held::Custom(id.clone()));
                }
            }
            return;
        };

        let custom = self.customised(line, &name, base_item.as_deref(), &data, &item_key);
        for place in self.placements.get_mut(index).into_iter().flatten() {
            place.held = Some(Held::Row(self.equipment.len()));
            place.facts = custom.facts.clone();
            // The state is the set's projection, set in `place_copies`.
            self.equipment.push(EquipmentWire {
                item_id: item_key.clone(),
                active_state: CARRIED.to_owned(),
                applied_modifiers: custom.modifiers.clone(),
                eqmods: custom.eqmods.clone(),
            });
        }
        self.mapped(line, &format!("equipmentSelections {item_key}: {worn} worn, {carried} carried"));
    }

    /// A mapped line's customisation under [`CUSTOM_PROPERTIES`]; a property Codex has no field
    /// for is kept as a property line, with a warning naming it (SD-e).
    fn customised(&mut self, line: &Line, name: &str, base_item: Option<&str>, data: &BTreeMap<String, String>, item_key: &str) -> Customised {
        let mut out = Customised::default();
        let mut kept_as_text: Vec<String> = Vec::new();
        let custom_name = data.get("NAME").map_or(name, String::as_str).trim();
        let record_names = [Some(item_key), base_item].into_iter().flatten().map(fold).collect::<Vec<_>>();
        if !custom_name.is_empty() && !record_names.contains(&fold(custom_name)) {
            out.facts.name = Some(custom_name.to_owned());
        }
        let eqmods = ["EQMOD", "ALTEQMOD"].into_iter().filter_map(|field| data.get(field)).flat_map(|mods| mods.split('.'));
        for eqmod in eqmods.map(|eqmod| eqmod.trim().replace("&pipe;", "|")).filter(|eqmod| !eqmod.is_empty()) {
            if let Some(die) = eqmod.strip_prefix("_DAMAGE|") {
                out.facts.damage_override.get_or_insert_with(|| die.trim().to_owned());
            } else if let Some(modifier) = enhancement_key(&eqmod) {
                if !out.modifiers.contains(&modifier) {
                    out.modifiers.push(modifier);
                }
            } else if let Some(option) = eqmod_option_key(&eqmod) {
                if !out.eqmods.contains(&option) {
                    out.eqmods.push(option);
                }
                // The option key drops a parameter (`|SPELLNAME[...]`); the line keeps it as text.
                if eqmod.contains('|') {
                    out.facts.properties.push(eqmod);
                }
            } else {
                kept_as_text.push(format!("EQMOD {eqmod}"));
            }
        }
        out.facts.properties.extend(kept_as_text.iter().cloned());
        if let Some(sprop) = data.get("SPROP") {
            out.facts.properties.extend(sprop.split("; ").map(str::trim).filter(|part| !part.is_empty()).map(str::to_owned));
        }
        if let Some(code) = data.get("SIZE") {
            match SizeCategory::from_base_size_code(code) {
                Some(SizeCategory::Medium) => {}
                Some(size) => out.facts.size = Some(size),
                None => {
                    out.facts.properties.push(format!("SIZE {code}"));
                    kept_as_text.push("SIZE".to_owned());
                }
            }
        }
        for (key, value) in data.iter().filter(|(key, _)| !matches!(key.as_str(), "NAME" | "EQMOD" | "ALTEQMOD" | "SPROP" | "SIZE" | "COSTMOD" | "KEY")) {
            out.facts.properties.push(format!("{key} {value}"));
            kept_as_text.push(key.clone());
        }
        for field in kept_as_text {
            let kind = if field.starts_with("EQMOD ") { NoteKind::Eqmod } else { NoteKind::Property };
            self.unmapped(kind, format!("{name}: {field} kept as text"));
            self.report.warnings.push(format!("{name}: {field} kept as text"));
        }
        // A record the converter cannot resolve states nothing to differ from.
        if let Some((weight, cost)) = record_weight_and_cost(item_key) {
            let differs = |line_value: f64, record: Option<f64>| record.is_none_or(|record| (record - line_value).abs() > 1e-6);
            if let Some(wt) = line.get("WT").and_then(|wt| wt.trim().parse::<f64>().ok()).filter(|wt| differs(*wt, weight)) {
                out.facts.weight_override_lbs = Some(wt);
            }
            if let Some(gp) = line.get("COST").and_then(|gp| gp.trim().parse::<f64>().ok()).filter(|gp| differs(*gp, cost)) {
                out.facts.cost_override_copper = Some(gp_to_copper(gp));
            }
        }
        out
    }

    /// The imported set and each stack's pieces, from the places the lines were read into. Every
    /// row's `activeState` becomes the set's projection, the state the app's load keeps.
    fn place_copies(&mut self) -> EquipmentSets {
        let row_keys = copy_keys(self.equipment.iter().map(|row| (row.item_id.as_str(), copy_bonus(&row.applied_modifiers))));
        let mut custom_ordinals: BTreeMap<String, u32> = BTreeMap::new();
        let mut keyed: Vec<(&Placement, CopyKey)> = Vec::new();
        for place in self.placements.iter().flatten() {
            let key = match &place.held {
                Some(Held::Row(row)) => row_keys[*row].clone(),
                Some(Held::Custom(id)) => {
                    let ordinal = custom_ordinals.entry(id.clone()).or_insert(0);
                    *ordinal += 1;
                    CopyKey { item_id: format!("custom:{id}"), bonus: 0, ordinal: *ordinal - 1 }
                }
                None => continue,
            };
            keyed.push((place, key));
        }
        let by_entry: BTreeMap<&str, &CopyKey> = keyed.iter().filter_map(|(place, key)| Some((place.entry.as_deref()?, key))).collect();
        // What each entry holds, in lb, over every entry nested below it.
        let mut contents: BTreeMap<&str, f64> = BTreeMap::new();
        for (place, _) in &keyed {
            let mut parent = place.parent.as_deref();
            while let Some(entry) = parent {
                *contents.entry(entry).or_insert(0.0) += f64::from(place.pieces) * place.weight_lbs;
                parent = keyed.iter().find(|(above, _)| above.entry.as_deref() == Some(entry)).and_then(|(above, _)| above.parent.as_deref());
            }
        }
        // The file states no capacity, so a Create item container holds what the file puts in it.
        for (place, _) in &keyed {
            let (Some(Held::Custom(id)), Some(lbs)) = (&place.held, place.entry.as_deref().and_then(|entry| contents.get(entry))) else { continue };
            if let Some(record) = self.custom.equipment.iter_mut().find(|record| record.id == *id) {
                record.container_capacity_lbs = Some(record.container_capacity_lbs.unwrap_or(0.0).max(*lbs));
            }
        }
        let starfinder = self.catalog.starfinder_package().is_some();
        let mut unknown_words: Vec<String> = Vec::new();
        let mut locations = Vec::new();
        let mut copies = BTreeMap::new();
        for (place, key) in &keyed {
            let location = match &place.parent {
                Some(parent) => by_entry.get(parent.as_str()).map_or(Location::Carried, |container| Location::Inside { container: (*container).clone() }),
                // Starfinder has no Pathfinder body slots: a worn line is Equipped (`decisions.md §8`).
                None if starfinder && !matches!(place.word.as_str(), "Carried" | "Not Carried") => Location::Equipped,
                None => location_word(&place.word).unwrap_or_else(|| {
                    if !unknown_words.contains(&place.word) {
                        unknown_words.push(place.word.clone());
                    }
                    Location::Equipped
                }),
            };
            if location != Location::Carried {
                locations.push(SetEntry { copy: key.clone(), location });
            }
            // PCGen's `QUANTITY` counts bundles; a stack counts pieces (`decisions.md §20`, E1.8).
            let pieces = match place.held {
                Some(Held::Row(_)) => place.pieces.saturating_mul(record_base_quantity(&key.item_id)),
                _ => place.pieces,
            };
            copies.insert(key.map_key(), CopyFacts { quantity: Some(pieces), ..place.facts.clone() });
        }
        for (at, word) in unknown_words.into_iter().enumerate() {
            let text = format!("{word}: a PCGen location Codex has no slot for; read as Equipped");
            self.notes.push((NoteKind::Location, at, text.clone()));
            self.report.warnings.push(text);
        }
        let set = EquipmentSet { name: self.set_name.clone(), locations };
        let states = project(&set, &row_keys);
        for (row, state) in self.equipment.iter_mut().zip(states) {
            row.active_state = format!("{state:?}");
        }
        EquipmentSets { copies, ..EquipmentSets::new(set) }
    }

    /// Records a line the import could not map directly, at the report line being written (file order).
    fn unmapped(&mut self, kind: NoteKind, text: String) {
        self.notes.push((kind, self.report.lines.len(), text));
    }

    /// `## Imported from PCGen (<file>, <date>)`, then each kind's lines once, in file order.
    fn notes_block(&mut self) -> String {
        let days = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |since| since.as_secs() / 86_400);
        let mut block = format!("## Imported from PCGen ({}, {})\n", self.report.source_file, iso_date(days));
        self.notes.sort_by_key(|(kind, at, _)| (*kind, *at));
        let mut written: Vec<(NoteKind, &str)> = Vec::new();
        for (kind, _, text) in &self.notes {
            if written.contains(&(*kind, text.as_str())) {
                continue;
            }
            if !written.iter().any(|(seen, _)| seen == kind) {
                block.push_str(&format!("\n### {}\n", kind.heading()));
            }
            block.push_str(&format!("- {text}\n"));
            written.push((*kind, text));
        }
        if written.is_empty() {
            block.push_str("\nEverything in the file was mapped.\n");
        }
        block
    }

    fn read_money(&mut self, line: &Line) {
        match line.value().trim().parse::<f64>() {
            Ok(gold) if gold >= 0.0 => {
                self.money.total_copper = (gold * 100.0).round() as u64;
                let to = format!("money.totalCopper {}", self.money.total_copper);
                self.mapped(line, &to);
            }
            _ => self.custom_feat(line, "A money line Codex could not read as gold pieces."),
        }
    }

    fn bio_field(&mut self, line: &Line, to: &str, field: fn(&mut BioWire) -> &mut String) {
        *field(&mut self.bio) = line.value().trim().to_owned();
        self.mapped(line, to);
    }

    fn mapped(&mut self, line: &Line, to: &str) {
        self.note(line, Outcome::Mapped, to);
    }

    fn note(&mut self, line: &Line, outcome: Outcome, to: &str) {
        self.report.lines.push(ReportLine {
            key: line.key().to_owned(),
            from: short(line.value()),
            outcome,
            to: to.to_owned(),
        });
    }

    fn added(&mut self, to: &str, why: &str) {
        self.report.lines.push(ReportLine { key: String::new(), from: why.to_owned(), outcome: Outcome::Added, to: to.to_owned() });
    }

    fn custom_feat(&mut self, line: &Line, description: &str) {
        let name = if line.value().trim().is_empty() { line.key().to_owned() } else { line.value().trim().to_owned() };
        self.custom_named_feat(line, &name, description);
    }

    fn custom_named_feat(&mut self, line: &Line, name: &str, description: &str) {
        // Q5: every line kept as a custom feat is a Notes line, unless its caller already wrote one.
        if self.notes.last().is_none_or(|(_, at, _)| *at != self.report.lines.len()) {
            self.unmapped(NoteKind::CustomFeat, format!("{name} ({}): kept as a custom feat", line.key()));
        }
        let stats = vec![CustomStat { label: "PCGen key".to_owned(), value: line.key().to_owned() }];
        let record = custom_record("feat", self.custom.feats.len(), name, description, stats);
        self.push_custom(line, record, |custom| &mut custom.feats);
    }

    fn custom_spell(&mut self, line: &Line, name: &str, class: &str) {
        let stats = spell_stats(class, line.get("SPELLLEVEL"), line.get("BOOK"));
        let record = custom_record("spell", self.custom.spells.len(), name, "A spell Codex has no record of.", stats);
        self.push_custom(line, record, |custom| &mut custom.spells);
    }

    /// A line no record maps is a Create item (`decisions.md §7`): weighed and priced per piece from
    /// the line, held at its places by `place_copies`, which also makes it a container when the set
    /// puts anything inside it.
    fn custom_item(&mut self, line: &Line, name: &str, base: &str, data: &BTreeMap<String, String>) {
        let mut stats: Vec<CustomStat> = line
            .get("NOTE")
            .filter(|value| !value.trim().is_empty())
            .map(|value| CustomStat { label: "Note".to_owned(), value: value.to_owned() })
            .into_iter()
            .collect();
        // Q2, Q5: the line's customization stays on the Create item as text, and each field is a Notes line.
        for (key, value) in data.iter().filter(|(key, _)| !matches!(key.as_str(), "KEY" | "NAME")).take(MAX_STATS - stats.len()) {
            stats.push(CustomStat { label: key.clone(), value: value.clone() });
            let kind = if key.ends_with("EQMOD") { NoteKind::Eqmod } else { NoteKind::Property };
            self.unmapped(kind, format!("{name}: {key} {value} kept as text"));
        }
        let reason = format!("no Codex record for {base}");
        let mut record = custom_record("equipment", self.custom.equipment.len(), name, &format!("Imported from PCGen: {reason}."), stats);
        record.weight_lbs = amount(line.get("WT"));
        record.price_gp = amount(line.get("COST"));
        self.report.warnings.push(format!("{name}: {reason}"));
        self.unmapped(NoteKind::Item, format!("{name}: {reason}"));
        self.push_custom(line, record, |custom| &mut custom.equipment);
    }

    fn push_custom(&mut self, line: &Line, record: CustomRecord, list: fn(&mut CustomWire) -> &mut Vec<CustomRecord>) {
        let name = record.name.clone();
        let records = list(&mut self.custom);
        if records.len() < MAX_RECORDS {
            records.push(record);
            self.note(line, Outcome::Custom, &name);
        } else {
            self.report.warnings.push(format!("{name}: past the {MAX_RECORDS} custom records a character holds; only this report lists it"));
            self.note(line, Outcome::Custom, "this report only");
        }
    }

    fn finish(mut self, lines: &[Line]) -> ImportFile {
        let equipment_sets = self.place_copies();
        self.report.game_mode = lines
            .iter()
            .find(|line| line.key() == "GAMEMODE")
            .map(|line| line.value().to_owned())
            .unwrap_or_default();
        let starfinder = self.catalog.starfinder_package();
        if self.report.game_mode != "Pathfinder_RPG" && starfinder.is_none() {
            self.report.warnings.push(format!(
                "This character was made in PCGen's {} game mode. Codex reads it against its Pathfinder 1e records.",
                if self.report.game_mode.is_empty() { "(unnamed)" } else { &self.report.game_mode }
            ));
        }
        let race = self.catalog.race(&self.race_name).cloned();
        let mut ability_scores = self.final_scores(race.as_ref());
        let race_id = race.as_ref().map_or_else(|| format!("race:{}", self.race_name.to_lowercase()), |race| race.id.clone());
        let class_levels = self.class_levels();
        let (source_package_id, game_system) = match starfinder {
            Some(package) => {
                ability_scores = self.starfinder_scores(package, &race_id, &class_levels, ability_scores);
                self.keep_the_picks_the_engine_takes(package, &race_id, &class_levels, &ability_scores);
                (SF_SOURCE_PACKAGE_ID, SF_SOURCE_PACKAGE_ID)
            }
            None => {
                self.add_engine_rows(&class_levels, &race_id);
                self.keep_the_spells_the_engine_takes(&race_id, &class_levels, &ability_scores);
                (SOURCE_PACKAGE_ID, PF_GAME_SYSTEM)
            }
        };
        self.close_biography();
        self.bio.notes = self.notes_block();

        let display_label = if self.label.trim().is_empty() { "Imported character".to_owned() } else { self.label.trim().to_owned() };
        let mut file = ImportFile {
            display_label,
            game_system: game_system.to_owned(),
            character_input: CharacterInputWire {
                source_package_id: source_package_id.to_owned(),
                chosen: ChosenWire {
                    race_id,
                    class_levels,
                    ability_scores,
                    selected_feats: self.feats,
                    skill_allocations: self.skills,
                    equipment_selections: self.equipment,
                    selected_choices: self.choices,
                    selected_traits: Vec::new(),
                    spells_selected: self.spells,
                },
            },
            bio: self.bio,
            money: self.money,
            custom: self.custom,
            equipment_sets,
            report: self.report,
        };
        if let Some(package) = starfinder {
            hold_the_files_credits(package, &mut file);
        }
        file
    }

    /// PCGen saves the score before the race's adjustment; Codex saves it after, as Create does.
    fn final_scores(&mut self, race: Option<&super::catalog::RaceEntry>) -> AbilitiesWire {
        for (_, ability) in ABILITY_NAMES {
            if !self.scores.contains_key(ability) {
                self.report.warnings.push(format!("the file states no {ability}; it reads as 10"));
            }
        }
        let mut floating = self.floating.clone();
        // A Human's first +2 is saved as `choice:human_ability_bonus` (`add_engine_rows`), which the
        // engine adds itself, so the score is saved before it, as Create saves it.
        if race.is_some_and(|race| race.id == "race:human") && !floating.is_empty() {
            floating.remove(0);
        }
        let score = |ability: &str| -> i16 {
            let base = self.scores.get(ability).copied().unwrap_or(10);
            let adjustment = race.and_then(|race| race.adjustments.get(ability).copied()).unwrap_or(0);
            let chosen = floating.iter().filter(|chosen| **chosen == ability).count() as i16 * 2;
            base + adjustment + chosen
        };
        let scores = AbilitiesWire {
            strength: score("strength"),
            dexterity: score("dexterity"),
            constitution: score("constitution"),
            intelligence: score("intelligence"),
            wisdom: score("wisdom"),
            charisma: score("charisma"),
        };
        if race.is_some_and(|race| race.floating_bonus_points > 0) && self.floating.is_empty() {
            self.report.warnings.push(format!("{}'s +2 to one ability is not stated in the file, so it is not applied", self.race_name));
        }
        scores
    }

    /// Starfinder's final scores (`decisions.md §8`): PCGen's `STAT` score is the score before the
    /// race and the theme, so the engine's own race and theme rows (`sf_abilities`, over a level-1
    /// build with no point buy) are added to it, then each increase the file states, as PCGen applied it.
    fn starfinder_scores(&mut self, package: &SheetRulePackage, race_id: &str, class_levels: &[ClassLevelWire], file: AbilitiesWire) -> AbilitiesWire {
        let theme = self.feats.iter().find(|id| package.rule(id).is_some_and(|rule| rule.pool == "theme")).cloned();
        let build = SfBuild {
            chassis: SfChassisBuild {
                classes: class_levels.first().map(|class| (class.class_id.clone(), 1)).into_iter().collect(),
                race: race_id.to_owned(),
                ability_scores: [10; 6],
                key_ability_choice: None,
            },
            theme,
            armor: None,
            picks: Vec::new(),
            skill_ranks: BTreeMap::new(),
            choices: BTreeMap::new(),
        };
        let adjustment = match sf_abilities::compute(package, &build, &SfAbilityBuild::default()) {
            Ok(scores) => scores.finals().map(|score| (score - 10) as i16),
            Err(refusal) => {
                self.report.warnings.push(format!("the race and theme ability adjustments are not applied: {}", refusal.message));
                [0; 6]
            }
        };
        let mut scores = [file.strength, file.dexterity, file.constitution, file.intelligence, file.wisdom, file.charisma];
        for (index, (_, long)) in ABILITY_NAMES.iter().enumerate() {
            scores[index] += adjustment[index] + self.increases.iter().filter(|(ability, _)| ability == long).map(|(_, plus)| plus).sum::<i16>();
        }
        let [strength, dexterity, constitution, intelligence, wisdom, charisma] = scores;
        AbilitiesWire { strength, dexterity, constitution, intelligence, wisdom, charisma }
    }

    /// A class Codex holds is the class; a file with none keeps its first class's name, so the
    /// engine's refusal names what it could not compute rather than an empty character.
    fn class_levels(&mut self) -> Vec<ClassLevelWire> {
        let mut levels: Vec<ClassLevelWire> = Vec::new();
        for (class_id, level) in &self.classes {
            match levels.iter_mut().find(|held| held.class_id == *class_id) {
                Some(held) => held.level = held.level.saturating_add(*level),
                None => levels.push(ClassLevelWire { class_id: class_id.clone(), level: *level }),
            }
        }
        if levels.is_empty() {
            let name = self.report.unmapped_classes.first().cloned().unwrap_or_else(|| "none".to_owned());
            let (class, level) = name.rsplit_once(' ').unwrap_or((name.as_str(), "1"));
            levels.push(ClassLevelWire {
                class_id: format!("class:{}", super::catalog::slug(class)),
                level: level.parse().unwrap_or(1),
            });
        }
        levels
    }

    fn add_engine_rows(&mut self, class_levels: &[ClassLevelWire], race_id: &str) {
        const WHY: &str = "the rules engine computes a character only with this row, as Create gives every character";
        for class in class_levels {
            let slug = class.class_id.strip_prefix("class:").unwrap_or(&class.class_id);
            let (choices, spells) = canonical_seeds_for(slug, class.level);
            for choice in choices {
                let to = format!("{} {} (default pick)", choice.choice_set_id, choice.selection_id);
                // A seed naming a plain feat (the Monk's Dodge) is checked against the feat list.
                let grants_a_feat = choice.selection_id.starts_with("feat:") && !choice.selection_id.contains(":weapon:");
                if grants_a_feat && !feat_identity::holds(&self.feats, &choice.selection_id) {
                    self.feats.push(choice.selection_id.clone());
                }
                self.choices.push(ChoiceWire { choice_set_id: choice.choice_set_id, selection_id: choice.selection_id });
                self.added(&to, WHY);
            }
            for spell in spells {
                let wire = SpellWire { spell_id: spell.spell_id, source_class_id: spell.source_class_id, acquisition_mode: mode_name(spell.acquisition_mode) };
                if !self.spells.contains(&wire) {
                    let to = format!("spellsSelected {} ({})", wire.spell_id, wire.acquisition_mode);
                    self.spells.push(wire);
                    self.added(&to, WHY);
                }
            }
        }
        if class_levels.iter().any(|class| class.class_id == "class:fighter") {
            self.choices.retain(|choice| choice.choice_set_id != FIGHTER_BONUS_CHOICE);
            self.choices.push(ChoiceWire { choice_set_id: FIGHTER_BONUS_CHOICE.to_owned(), selection_id: FIGHTER_BONUS_SELECTION.to_owned() });
            self.added(&format!("{FIGHTER_BONUS_CHOICE} {FIGHTER_BONUS_SELECTION}"), WHY);
        }
        if let (true, Some(ability)) = (race_id == "race:human", self.floating.first().copied()) {
            self.choices.push(ChoiceWire { choice_set_id: "choice:human_ability_bonus".to_owned(), selection_id: format!("ability:{ability}") });
        }
        if !feat_identity::holds(&self.feats, WEAPON_FOCUS_FEAT) {
            self.feats.push(WEAPON_FOCUS_FEAT.to_owned());
            self.added(&format!("selectedFeats {WEAPON_FOCUS_FEAT}"), WHY);
        }
        self.equipment.push(EquipmentWire { item_id: POWER_ATTACK_ROW.to_owned(), active_state: CARRIED.to_owned(), applied_modifiers: Vec::new(), eqmods: Vec::new() });
        self.added(&format!("equipmentSelections {POWER_ATTACK_ROW}"), WHY);
    }

    /// Adds the file's spells one at a time, known before prepared. The engine's class lists are
    /// narrower than PCGen's (a Paladin's holds no Ultimate Combat spell), and one refused spell
    /// refuses the whole character, so a spell that brings a new refusal becomes a custom spell.
    fn keep_the_spells_the_engine_takes(&mut self, race_id: &str, class_levels: &[ClassLevelWire], scores: &AbilitiesWire) {
        let mut file_spells = std::mem::take(&mut self.file_spells);
        file_spells.sort_by_key(|spell| spell.wire.acquisition_mode != "Known");
        let refusals = |this: &Self| -> usize {
            let input = engine_input(&this.chosen_so_far(race_id, class_levels, scores));
            build_pilot_headless_receipt(&input)
                .computation
                .diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.claim_blocking && diagnostic.id.starts_with("class_spell."))
                .count()
        };
        let before = refusals(self);
        for spell in file_spells {
            if self.spells.contains(&spell.wire) {
                continue;
            }
            self.spells.push(spell.wire.clone());
            if refusals(self) <= before {
                continue;
            }
            self.spells.pop();
            let name = spell.wire.spell_id.clone();
            let text = format!("{name}: Codex's {} spell list does not take it, so it is a custom spell", spell.class);
            self.notes.push((NoteKind::Spell, spell.report_line, text.clone()));
            self.report.warnings.push(text);
            let stats = spell_stats(&spell.class, spell.level.as_deref(), spell.book.as_deref());
            let record = custom_record("spell", self.custom.spells.len(), &name, "A spell Codex's class list does not take yet.", stats);
            if self.custom.spells.len() < MAX_RECORDS {
                self.custom.spells.push(record);
            }
            let line = &mut self.report.lines[spell.report_line];
            line.outcome = Outcome::Custom;
            line.to = name;
        }
    }

    fn chosen_so_far(&self, race_id: &str, class_levels: &[ClassLevelWire], scores: &AbilitiesWire) -> ChosenWire {
        ChosenWire {
            race_id: race_id.to_owned(),
            class_levels: class_levels.to_vec(),
            ability_scores: scores.clone(),
            selected_feats: self.feats.clone(),
            skill_allocations: self.skills.clone(),
            equipment_selections: self.equipment.clone(),
            selected_choices: self.choices.clone(),
            selected_traits: Vec::new(),
            spells_selected: self.spells.clone(),
        }
    }

    /// The Languages and Biography custom feats, written last because many lines feed each.
    fn close_biography(&mut self) {
        if !self.languages.is_empty() && self.custom.feats.len() < MAX_RECORDS {
            let description = self.languages.join(", ");
            let record = custom_record("feat", self.custom.feats.len(), "Languages", &description, Vec::new());
            self.custom.feats.push(record);
        }
        if !self.biography.is_empty() && self.custom.feats.len() < MAX_RECORDS {
            let (text, stats): (Vec<_>, Vec<_>) = self.biography.iter().partition(|(key, _)| key == "CHARACTERBIO" || key == "CHARACTERDESC");
            let description = text.iter().map(|(_, value)| value.as_str()).collect::<Vec<_>>().join("\n\n");
            let stats = stats.iter().take(MAX_STATS).map(|(key, value)| CustomStat { label: bio_label(key).to_owned(), value: value.clone() }).collect();
            let record = custom_record("feat", self.custom.feats.len(), "Biography", &description, stats);
            self.custom.feats.push(record);
        }
    }
}

/// The Starfinder build and carried loadout of `input`: the table of the app's Starfinder adapter
/// (`sf_adapter.rs` `build_from_input`, retyped here because the sidecar does not link the app).
pub fn starfinder_build(package: &SheetRulePackage, input: &CharacterInput) -> (SfBuild, SfLoadout) {
    let chosen = &input.chosen;
    let classes: Vec<(String, u8)> = chosen.class_levels.iter().map(|class| (class.class_id.clone(), class.level)).collect();
    let scores = &chosen.ability_scores;
    let ability_scores = [scores.strength, scores.dexterity, scores.constitution, scores.intelligence, scores.wisdom, scores.charisma].map(i64::from);
    let mut key_ability_choice = None;
    let mut choices: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut credits_adjustment = 0_i64;
    for choice in &chosen.selected_choices {
        if choice.choice_set_id == SF_CREDITS_CHOICE {
            credits_adjustment += choice.selection_id.parse::<i64>().unwrap_or(0);
        } else if classes.iter().any(|(class, _)| *class == choice.choice_set_id) {
            key_ability_choice = match choice.selection_id.as_str() {
                "STR" => Some(Ability::Str),
                "DEX" => Some(Ability::Dex),
                "CON" => Some(Ability::Con),
                "INT" => Some(Ability::Int),
                "WIS" => Some(Ability::Wis),
                "CHA" => Some(Ability::Cha),
                _ => key_ability_choice,
            };
        } else {
            choices.entry(choice.choice_set_id.clone()).or_default().push(choice.selection_id.clone());
        }
    }
    let (themes, mut picks): (Vec<String>, Vec<String>) =
        chosen.selected_feats.iter().cloned().partition(|id| package.rule(id).is_some_and(|rule| rule.pool == "theme"));
    let is_armor = |id: &str| package.rule(id).is_some_and(|rule| rule.tags.iter().any(|tag| tag == "ARMOR"));
    let mut armor = None;
    let mut carried: Vec<(String, u32)> = Vec::new();
    let mut applied = Vec::new();
    for selection in chosen.equipment_selections.iter().filter(|selection| selection.active_state != ActiveState::Absent) {
        applied.extend(selection.applied_modifiers.iter().cloned());
        let worn = selection.active_state == ActiveState::EquippedActive;
        if worn && armor.is_none() && is_armor(&selection.item_id) {
            armor = Some(selection.item_id.clone());
            continue;
        }
        if worn && !picks.contains(&selection.item_id) {
            picks.push(selection.item_id.clone());
        }
        match carried.iter_mut().find(|(id, _)| *id == selection.item_id) {
            Some((_, quantity)) => *quantity += 1,
            None => carried.push((selection.item_id.clone(), 1)),
        }
    }
    let build = SfBuild {
        chassis: SfChassisBuild { classes, race: chosen.race_id.clone(), ability_scores, key_ability_choice },
        theme: themes.into_iter().next(),
        armor,
        picks,
        skill_ranks: chosen.skill_allocations.iter().map(|skill| (skill.skill_id.clone(), i64::from(skill.ranks))).collect(),
        choices,
    };
    (build, SfLoadout { starting_credits: None, carried, applied, credits_adjustment })
}

/// The Starfinder engine's verdict on `input`: every total the app's Starfinder sheet computes
/// (`sf_adapter.rs` `compute_sheet`: chassis, defense, skills, spells, carried, attacks), or the
/// first refusal by its id and message. The sheet's printed lines are the app's and not run here.
pub fn starfinder_verdict(package: &SheetRulePackage, input: &CharacterInput) -> Result<(), String> {
    let refused = |refusal: sf_chassis::SfChassisRefusal| format!("{}: {}", refusal.id, refusal.message);
    let (build, loadout) = starfinder_build(package, input);
    let (build, _) = sf_abilities::with_item_bonuses(package, &build);
    let mut chassis = sf_chassis::compute(package, &build.chassis).map_err(refused)?;
    let held = sf_defense::held(package, &build).map_err(refused)?;
    sf_chassis::fold_held(package, &mut chassis, &held);
    sf_defense::compute_with(package, &build, &held).map_err(refused)?;
    sf_skills::compute_with(package, &build, &held).map_err(refused)?;
    sf_spells::compute_with(package, &build, &held).map_err(refused)?;
    sf_loadout::compute(package, &build, &loadout).map_err(refused)?;
    sf_attack::compute_with(package, &held, &chassis.base_attack_bonus, &loadout.carried);
    Ok(())
}

/// A Starfinder character's credits are what its file says it has left (`MONEY`), not the level's
/// wealth less the gear's prices (`sf_loadout`): the difference is the app's credits row, the one
/// the Market writes ([`SF_CREDITS_CHOICE`]).
fn hold_the_files_credits(package: &SheetRulePackage, file: &mut ImportFile) {
    let left = i64::try_from(file.money.total_copper / 100).unwrap_or(i64::MAX);
    let (build, mut loadout) = starfinder_build(package, &character_input_of(file));
    // Any amount that clears the budget check; the row is then what makes the remainder `left`.
    loadout.credits_adjustment = i64::MAX / 4;
    let Ok(carried) = sf_loadout::compute(package, &build, &loadout) else { return };
    let adjustment = left - (carried.starting_credits.total - carried.credits_spent.total);
    if adjustment != 0 {
        file.character_input.chosen.selected_choices.push(ChoiceWire { choice_set_id: SF_CREDITS_CHOICE.to_owned(), selection_id: adjustment.to_string() });
        file.report.lines.push(ReportLine {
            key: String::new(),
            from: format!("the file's {left} credits left"),
            outcome: Outcome::Added,
            to: format!("selectedChoices {SF_CREDITS_CHOICE} {adjustment}"),
        });
    }
}

/// The engine's input for an import, as the app's `character_input_from_dto` reads the wire.
pub fn character_input_of(file: &ImportFile) -> CharacterInput {
    let mut input = engine_input(&file.character_input.chosen);
    input.source_package_id = file.character_input.source_package_id.clone();
    input
}

fn spell_stats(class: &str, level: Option<&str>, book: Option<&str>) -> Vec<CustomStat> {
    [("Class", Some(class)), ("Level", level), ("Book", book)]
        .into_iter()
        .filter_map(|(label, value)| {
            let value = value.filter(|value| !value.trim().is_empty())?;
            Some(CustomStat { label: label.to_owned(), value: value.to_owned() })
        })
        .collect()
}

/// The engine's own input for `chosen`, as `character_input_from_dto` builds it in the app.
fn engine_input(chosen: &ChosenWire) -> CharacterInput {
    CharacterInput {
        case_id: None,
        source_package_id: SOURCE_PACKAGE_ID.to_owned(),
        chosen: ChosenCharacterState {
            race_id: chosen.race_id.clone(),
            class_levels: chosen.class_levels.iter().map(|class| CharacterClassLevel { class_id: class.class_id.clone(), level: class.level }).collect(),
            ability_scores: AbilityScores {
                strength: chosen.ability_scores.strength,
                dexterity: chosen.ability_scores.dexterity,
                constitution: chosen.ability_scores.constitution,
                intelligence: chosen.ability_scores.intelligence,
                wisdom: chosen.ability_scores.wisdom,
                charisma: chosen.ability_scores.charisma,
            },
            selected_feats: chosen.selected_feats.clone(),
            skill_allocations: chosen.skill_allocations.iter().map(|skill| SkillAllocation { skill_id: skill.skill_id.clone(), ranks: skill.ranks }).collect(),
            equipment_selections: chosen
                .equipment_selections
                .iter()
                .map(|row| {
                    let active_state = match row.active_state.as_str() {
                        WORN => ActiveState::EquippedActive,
                        "Absent" => ActiveState::Absent,
                        _ => ActiveState::SelectedInactive,
                    };
                    EquipmentSelection {
                        item_id: row.item_id.clone(),
                        equipped_or_active: active_state == ActiveState::EquippedActive,
                        active_state,
                        applied_modifiers: row.applied_modifiers.clone(),
                    }
                })
                .collect(),
            selected_choices: chosen
                .selected_choices
                .iter()
                .map(|choice| SelectedChoice { choice_set_id: choice.choice_set_id.clone(), selection_id: choice.selection_id.clone() })
                .collect(),
            selected_traits: Vec::new(),
            spells_selected: chosen
                .spells_selected
                .iter()
                .map(|spell| SpellSelection {
                    spell_id: spell.spell_id.clone(),
                    source_class_id: spell.source_class_id.clone(),
                    acquisition_mode: match spell.acquisition_mode.as_str() {
                        "Prepared" => AcquisitionMode::Prepared,
                        "Granted" => AcquisitionMode::Granted,
                        _ => AcquisitionMode::Known,
                    },
                })
                .collect(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

fn custom_record(kind: &str, index: usize, name: &str, description: &str, stats: Vec<CustomStat>) -> CustomRecord {
    CustomRecord {
        id: format!("pcg-{kind}-{}", index + 1),
        name: clipped(name, MAX_NAME),
        description: clipped(description, MAX_DESCRIPTION),
        stats: stats
            .into_iter()
            .map(|stat| CustomStat { label: clipped(&stat.label, MAX_STAT_TEXT), value: clipped(&stat.value, MAX_STAT_TEXT) })
            .collect(),
        weight_lbs: None,
        price_gp: None,
        container_capacity_lbs: None,
    }
}

fn clipped(text: &str, limit: usize) -> String {
    if text.chars().count() <= limit {
        return text.to_owned();
    }
    text.chars().take(limit - 1).chain(std::iter::once('…')).collect()
}

fn short(text: &str) -> String {
    clipped(text.trim(), 200)
}

/// Days since 1970-01-01 as `YYYY-MM-DD` (the proleptic Gregorian calendar, Hinnant's civil_from_days).
fn iso_date(days: u64) -> String {
    let z = days as i64 + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}")
}

/// A `QUANTITY` as whole pieces; `0` is zero (SD-d), an absent or unreadable one is 1.
fn quantity(value: Option<&str>) -> u32 {
    value
        .and_then(|value| value.trim().parse::<f64>().ok())
        .map_or(1, |value| value.round().clamp(0.0, f64::from(u32::MAX)) as u32)
}

/// A line's `WT` or `COST`: a number, 0 or more (`custom.json` refuses any other).
fn amount(value: Option<&str>) -> Option<f64> {
    value.and_then(|value| value.trim().parse::<f64>().ok()).filter(|value| value.is_finite() && *value >= 0.0)
}

/// The record's own weight (lb) and price (gp) for one piece, from the settled record the engine
/// resolves `item_key` to; `None` when no record resolves.
/// The settled equipment corpus the engine weighs a copy by, read once.
pub fn equipment_corpus() -> &'static SourcePackageContent<'static> {
    static CORPUS: OnceLock<SourcePackageContent<'static>> = OnceLock::new();
    CORPUS.get_or_init(|| {
        let dirs = corpus_book_dirs(&runtime_repo_root().join("data/corpus"));
        let ids: Vec<String> = dirs.iter().map(|dir| dir.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default()).collect();
        let roots: Vec<BookCorpusRoot<'_>> = ids.iter().zip(&dirs).map(|(book_id, dir)| BookCorpusRoot { book_id, dir: dir.as_path() }).collect();
        load_equipment_corpus(&roots)
    })
}

/// The pieces one bundle of `item_key` holds, as the engine divides by (`encumbrance::copy_bundles`):
/// the record's `base_quantity`, else its stand-in's, else 1 (also for a key the corpus lacks).
pub fn record_base_quantity(item_key: &str) -> u32 {
    let corpus = equipment_corpus();
    let Some(record) = equipment_converted_resolve(item_key, corpus) else { return 1 };
    record.base_quantity.or_else(|| stand_in_record(record, corpus).and_then(|base| base.base_quantity)).unwrap_or(1).max(1)
}

fn record_weight_and_cost(item_key: &str) -> Option<(Option<f64>, Option<f64>)> {
    let corpus = equipment_corpus();
    let record = equipment_converted_resolve(item_key, corpus)?;
    // A derived record stating neither reads the record it is made from, as the engine weighs it.
    let base = stand_in_record(record, corpus);
    let (weight, cost) = record.weight_and_cost();
    Some((weight.or_else(|| base.and_then(|base| base.weight_lbs)), cost.or_else(|| base.and_then(|base| base.cost_gp))))
}

/// A line's `CUSTOMIZATION`: its `BASEITEM` and its `DATA` fields.
fn customization(line: &Line) -> (Option<String>, BTreeMap<String, String>) {
    let customization = line.tags.iter().find(|tag| tag.key == "CUSTOMIZATION").map(Tag::nested).unwrap_or_default();
    let base_item = customization.iter().find(|tag| tag.key == "BASEITEM").map(|tag| tag.value.clone());
    let data = customization.iter().find(|tag| tag.key == "DATA").map(|tag| data_fields(&tag.value)).unwrap_or_default();
    (base_item, data)
}

/// The `+N` a row's modifiers give its copy key, as the engine reads it (`encumbrance::copy_bonus`).
fn copy_bonus(modifiers: &[String]) -> u8 {
    (1..=10u8)
        .filter(|n| [Enhanceable::Armor, Enhanceable::Shield, Enhanceable::Weapon].iter().any(|kind| modifiers.contains(&kind.modifier_key(*n))))
        .max()
        .unwrap_or(0)
}

/// A top-level location word: Carried, Not Carried, Equipped, or a [`PCGEN_SLOTS`] row.
fn location_word(word: &str) -> Option<Location> {
    match word {
        "Carried" => Some(Location::Carried),
        "Not Carried" => Some(Location::NotCarried),
        "Equipped" => Some(Location::Equipped),
        _ => PCGEN_SLOTS.iter().find(|(name, _)| name.eq_ignore_ascii_case(word)).map(|(_, slot)| match slot {
            BodySlot::None => Location::Equipped,
            slot => Location::Slot { slot: *slot },
        }),
    }
}

fn applied_to(line: &Line) -> Vec<String> {
    line.get("APPLIEDTO")
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|target| !target.is_empty())
        .map(str::to_owned)
        .collect()
}

/// `Weapon Focus (Longsword)` is `Weapon Focus` and `Longsword`. A name with no such tail is itself.
fn split_parenthesised(raw: &str) -> (&str, Option<&str>) {
    match raw.trim().strip_suffix(')').and_then(|body| body.split_once(" (")) {
        Some((name, inner)) => (name, Some(inner)),
        None => (raw.trim(), None),
    }
}

/// `+2 Strength`, PCGen's key for the Human, Half-Elf and Half-Orc floating bonus.
fn floating_bonus_ability(key: &str) -> Option<&'static str> {
    let ability = key.strip_prefix("+2 ")?.trim().to_lowercase();
    ABILITY_NAMES.iter().map(|(_, long)| *long).find(|long| *long == ability)
}

/// `NAME=x$KEY=y$EQMOD=A.B` into its fields.
fn data_fields(data: &str) -> BTreeMap<String, String> {
    data.split('$')
        .filter_map(|field| field.split_once('='))
        .map(|(key, value)| (key.to_owned(), value.to_owned()))
        .collect()
}

/// A `+N` eqmod as the key `attach_equipment_modifier` takes: PCGen's `PLUS1W`, or the key itself
/// as newer files write it (`Special Ability ~ +1 ~ Weapon`).
fn enhancement_key(eqmod: &str) -> Option<String> {
    let is_key = (1..=5u8).any(|n| [Enhanceable::Armor, Enhanceable::Shield, Enhanceable::Weapon].iter().any(|kind| kind.modifier_key(n) == eqmod));
    if is_key {
        return Some(eqmod.to_owned());
    }
    let rest = eqmod.strip_prefix("PLUS")?;
    let mut chars = rest.chars();
    let bonus = chars.next()?.to_digit(10).filter(|bonus| (1..=5).contains(bonus))?;
    let kind = match chars.as_str() {
        "W" => "Weapon",
        "A" => "Armor",
        "S" => "Shield",
        _ => return None,
    };
    Some(format!("Special Ability ~ +{bonus} ~ {kind}"))
}

fn mode_name(mode: AcquisitionMode) -> String {
    match mode {
        AcquisitionMode::Known => "Known",
        AcquisitionMode::Prepared => "Prepared",
        AcquisitionMode::Granted => "Granted",
    }
    .to_owned()
}

fn alignment_name(code: &str) -> String {
    match code.trim() {
        "LG" => "Lawful Good",
        "NG" => "Neutral Good",
        "CG" => "Chaotic Good",
        "LN" => "Lawful Neutral",
        "TN" | "NN" | "N" => "Neutral",
        "CN" => "Chaotic Neutral",
        "LE" => "Lawful Evil",
        "NE" => "Neutral Evil",
        "CE" => "Chaotic Evil",
        other => other,
    }
    .to_owned()
}

fn bio_label(key: &str) -> &'static str {
    match key {
        "CHARACTERCOMP" => "Companions",
        "CHARACTERASSET" => "Assets",
        "CHARACTERMAGIC" => "Magic items",
        "CHARACTERDMNOTES" => "GM notes",
        "TABNAME" => "Tab name",
        "EXPERIENCE" => "Experience",
        "HANDED" => "Handed",
        "SKINCOLOR" => "Skin",
        "HAIRSTYLE" => "Hair style",
        "LOCATION" => "Location",
        "CITY" => "City",
        "BIRTHDAY" => "Birthday",
        "BIRTHPLACE" => "Birthplace",
        "PERSONALITYTRAIT1" | "PERSONALITYTRAIT2" => "Personality",
        "SPEECHPATTERN" => "Speech",
        "PHOBIAS" => "Phobias",
        "INTERESTS" => "Interests",
        "CATCHPHRASE" => "Catch phrase",
        "REGION" => "Region",
        _ => "Note",
    }
}

#[cfg(test)]
mod iso_date_tests {
    #[test]
    fn iso_date_reads_known_days() {
        assert_eq!(super::iso_date(0), "1970-01-01");
        assert_eq!(super::iso_date(11_016), "2000-02-29");
        assert_eq!(super::iso_date(20_774), "2026-11-17");
    }
}
