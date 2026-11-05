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

use codex::rules_core::class_seeds::canonical_seeds_for;
use codex::rules_core::character_input::{
    AbilityScores, AcquisitionMode, ActiveState, CharacterClassLevel, CharacterInput, ChosenCharacterState, EquipmentSelection,
    SelectedChoice, SkillAllocation, SpellSelection,
};
use codex::rules_core::feat_effects::chooser_contract_for_feat;
use codex::rules_core::feat_identity;
use codex::rules_core::pilot_compute::build_pilot_headless_receipt;

use super::catalog::{Catalog, fold};
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
    ("EQUIPNAME", KeyRule::Field("equipmentSelections, BASEITEM first, PLUSnW/A/S to an applied +n modifier")),
    ("EQUIPSET", KeyRule::Field("activeState: a location at the set's top level is worn; Carried, Not Carried and anything inside a container are carried")),
    ("CALCEQUIPSET", KeyRule::Field("which EQUIPSET is the worn one")),
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
/// One selection row per copy, so a stack of ammunition stays a bounded list.
const MAX_COPIES: u32 = 99;

/// Converts the text of one `.pcg`. Refuses only a file that is not a PCGen character.
pub fn convert(text: &str, source_file: &str, catalog: &Catalog) -> Result<ImportFile, String> {
    let lines = parse(text);
    if !looks_like_a_character(&lines) {
        return Err(format!(
            "{source_file} is not a PCGen 6 character: it has no PCGVERSION or CHARACTERNAME line (PCGen 5 and older wrote another format)"
        ));
    }
    let mut importer = Importer::new(source_file, catalog);
    importer.read_equipment_sets(&lines);
    for line in &lines {
        importer.read(line);
    }
    Ok(importer.finish(&lines))
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
    /// Worn and carried counts left to hand out, by the folded EQUIPSET value.
    worn: BTreeMap<String, u32>,
    carried: BTreeMap<String, u32>,
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
            worn: BTreeMap::new(),
            carried: BTreeMap::new(),
        }
    }

    /// EQUIPSET lines may sit before or after what they name, so they are read first. A location at
    /// the top level of the worn set is worn; `Carried` and `Not Carried` are not, and an item in a
    /// container is in the container.
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
        let top_level = worn_set.split('.').count() + 1;
        for line in lines.iter().filter(|line| line.key() == "EQUIPSET") {
            let (Some(id), Some(value)) = (line.get("ID"), line.get("VALUE")) else { continue };
            if !id.starts_with(&format!("{worn_set}.")) {
                continue;
            }
            let copies = quantity(line.get("QUANTITY"));
            let is_worn = id.split('.').count() == top_level && !matches!(line.value(), "Carried" | "Not Carried");
            let target = if is_worn { &mut self.worn } else { &mut self.carried };
            *target.entry(fold(value)).or_default() += copies;
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
        let name = line.value().to_owned();
        let customization = line.tags.iter().find(|tag| tag.key == "CUSTOMIZATION").map(Tag::nested).unwrap_or_default();
        let base_item = customization.iter().find(|tag| tag.key == "BASEITEM").map(|tag| tag.value.clone());
        let data = customization.iter().find(|tag| tag.key == "DATA").map(|tag| data_fields(&tag.value)).unwrap_or_default();
        let data_key = data.get("KEY").cloned();
        let copies = quantity(line.get("QUANTITY"));
        let names_it = [Some(name.clone()), data_key, base_item.clone()];

        let (worn, carried) = self.split_copies(&names_it, copies);
        let item_key = [base_item.as_deref(), data.get("KEY").map(String::as_str), Some(name.as_str())]
            .into_iter()
            .flatten()
            .find_map(|candidate| self.catalog.equipment_key(candidate))
            .map(str::to_owned);
        let Some(item_key) = item_key else {
            self.custom_item(line, &name, worn, carried);
            return;
        };

        let (modifiers, other_mods) = equipment_modifiers(data.get("EQMOD").map_or("", String::as_str));
        if !other_mods.is_empty() {
            self.report.warnings.push(format!("{name}: its modifiers {} are not applied", other_mods.join(", ")));
        }
        if worn + carried > MAX_COPIES {
            self.report.warnings.push(format!("{name}: {} copies listed as {MAX_COPIES}", worn + carried));
        }
        let worn = worn.min(MAX_COPIES);
        let carried = carried.min(MAX_COPIES - worn);
        let row = |active_state: &str| EquipmentWire {
            item_id: item_key.clone(),
            active_state: active_state.to_owned(),
            applied_modifiers: modifiers.clone(),
        };
        self.equipment.extend((0..worn).map(|_| row(WORN)));
        self.equipment.extend((0..carried).map(|_| row(CARRIED)));
        self.mapped(line, &format!("equipmentSelections {item_key}: {worn} worn, {carried} carried"));
    }

    /// Hands out this line's copies from what the sets say. Whatever no set names is carried.
    fn split_copies(&mut self, names: &[Option<String>], copies: u32) -> (u32, u32) {
        let keys: Vec<String> = names.iter().flatten().map(|name| fold(name)).collect();
        let mut worn = 0;
        for key in &keys {
            if let Some(left) = self.worn.get_mut(key) {
                let taken = (*left).min(copies - worn);
                *left -= taken;
                worn += taken;
            }
        }
        (worn, copies - worn)
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
        let stats = vec![CustomStat { label: "PCGen key".to_owned(), value: line.key().to_owned() }];
        let record = custom_record("feat", self.custom.feats.len(), name, description, stats);
        self.push_custom(line, record, |custom| &mut custom.feats);
    }

    fn custom_spell(&mut self, line: &Line, name: &str, class: &str) {
        let stats = spell_stats(class, line.get("SPELLLEVEL"), line.get("BOOK"));
        let record = custom_record("spell", self.custom.spells.len(), name, "A spell Codex has no record of.", stats);
        self.push_custom(line, record, |custom| &mut custom.spells);
    }

    fn custom_item(&mut self, line: &Line, name: &str, worn: u32, carried: u32) {
        let mut stats = vec![
            CustomStat { label: "Worn".to_owned(), value: worn.to_string() },
            CustomStat { label: "Carried".to_owned(), value: carried.to_string() },
        ];
        for (label, key) in [("Cost (gp)", "COST"), ("Weight (lb.)", "WT"), ("Note", "NOTE")] {
            if let Some(value) = line.get(key).filter(|value| !value.trim().is_empty()) {
                stats.push(CustomStat { label: label.to_owned(), value: value.to_owned() });
            }
        }
        let record = custom_record("equipment", self.custom.equipment.len(), name, "An item Codex has no record of.", stats);
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
        self.report.game_mode = lines
            .iter()
            .find(|line| line.key() == "GAMEMODE")
            .map(|line| line.value().to_owned())
            .unwrap_or_default();
        if self.report.game_mode != "Pathfinder_RPG" {
            self.report.warnings.push(format!(
                "This character was made in PCGen's {} game mode. Codex reads it against its Pathfinder 1e records.",
                if self.report.game_mode.is_empty() { "(unnamed)" } else { &self.report.game_mode }
            ));
        }
        let race = self.catalog.race(&self.race_name).cloned();
        let ability_scores = self.final_scores(race.as_ref());
        let race_id = race.as_ref().map_or_else(|| format!("race:{}", self.race_name.to_lowercase()), |race| race.id.clone());
        let class_levels = self.class_levels();
        self.add_engine_rows(&class_levels, &race_id);
        self.keep_the_spells_the_engine_takes(&race_id, &class_levels, &ability_scores);
        self.close_biography();

        let display_label = if self.label.trim().is_empty() { "Imported character".to_owned() } else { self.label.trim().to_owned() };
        ImportFile {
            display_label,
            character_input: CharacterInputWire {
                source_package_id: SOURCE_PACKAGE_ID.to_owned(),
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
            report: self.report,
        }
    }

    /// PCGen saves the score before the race's adjustment; Codex saves it after, as Create does.
    fn final_scores(&mut self, race: Option<&super::catalog::RaceEntry>) -> AbilitiesWire {
        for (_, ability) in ABILITY_NAMES {
            if !self.scores.contains_key(ability) {
                self.report.warnings.push(format!("the file states no {ability}; it reads as 10"));
            }
        }
        let floating = self.floating.clone();
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
        if race.is_some_and(|race| race.floating_bonus_points > 0) && floating.is_empty() {
            self.report.warnings.push(format!("{}'s +2 to one ability is not stated in the file, so it is not applied", self.race_name));
        }
        scores
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
        self.equipment.push(EquipmentWire { item_id: POWER_ATTACK_ROW.to_owned(), active_state: CARRIED.to_owned(), applied_modifiers: Vec::new() });
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
            self.report.warnings.push(format!("{name}: Codex's {} spell list does not take it, so it is a custom spell", spell.class));
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
                    let active_state = if row.active_state == WORN { ActiveState::EquippedActive } else { ActiveState::SelectedInactive };
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

fn quantity(value: Option<&str>) -> u32 {
    value
        .and_then(|value| value.trim().parse::<f64>().ok())
        .map_or(1, |value| value.round().max(1.0).min(f64::from(u32::MAX)) as u32)
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

/// The `+N` eqmods as the keys `attach_equipment_modifier` takes, and every other eqmod apart.
/// Materials and special abilities are later work (`decisions.md` R14), so they are reported.
fn equipment_modifiers(eqmods: &str) -> (Vec<String>, Vec<String>) {
    let mut modifiers = Vec::new();
    let mut others = Vec::new();
    for eqmod in eqmods.split('.').map(str::trim).filter(|eqmod| !eqmod.is_empty()) {
        match enhancement_key(eqmod) {
            Some(key) => modifiers.push(key),
            None => others.push(eqmod.to_owned()),
        }
    }
    (modifiers, others)
}

fn enhancement_key(eqmod: &str) -> Option<String> {
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
