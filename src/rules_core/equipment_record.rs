//! The live side's own converted equipment record.
//!
//! SD-35 `AT-35-E6-003-RULED` cycle 10, `decisions.md` §11 ("not one line of
//! PCGen in our live code") and §19 (operator ruling B16: naming
//! `pcgen_import` in shipping code under a live root is a hit).
//!
//! ## What this closes
//!
//! Cycle 8 gave the spell kind a converted shape of its own
//! ([`crate::rules_core::spell_record::CorpusSpellRecord`]) and every census
//! since has named the equipment kind as the piece still open: nine live
//! consumers reading `EquipmentRecord.tokens` / `EquipmentRecord.bonus_chains`
//! -- the ingest format's own token arrays -- as their data type.
//!
//! Cycles 8 and 9 both refused a `CorpusEquipmentRecord` that would *carry
//! those arrays*, on the correct ground that moving PCGen token structures
//! under a live root is worse than the hit it clears. **This record carries no
//! array.** Every field on it is a settled value: a weight in pounds, a price
//! in gold, a resolved `+2 enhancement bonus to Str`. The token spelling, the
//! `BONUS:` chain grammar and the qualifier traversal stay on the converter
//! side, in [`crate::pcgen_import::ir_converter::equipment_record_to_corpus`],
//! which is where `decisions.md` §11 rules that rule conversion happens.
//!
//! That is the sheet rule (`decisions.md` §1) applied to the equipment kind:
//! the sheet prints one final number, and the number is settled at ingest.
//!
//! ## Who builds one
//!
//! One producer: [`crate::pcgen_import::ir_converter::equipment_record_to_corpus`],
//! called from [`crate::pcgen_import::ir_converter::convert_equipment_record`],
//! which is the single construction point of the canonical envelope's
//! `Equipment` payload. A record read from `data/corpus/**/equipment/*.json`
//! reaches it through [`crate::rules_core::corpus_loader::load_equipment_corpus`]
//! on exactly the same path, so there is one conversion, not two.
//!
//! ## What is deliberately NOT on it yet
//!
//! Nothing a live consumer still reads off the parser row. Cycle 12 settled
//! the last of them -- the weapon half and the two `EQMOD:`-referenced steps
//! (see below) -- so every remaining `pcgen_import` hit under
//! `src/rules_core/` belongs to a different group than this one.
//!
//! What is still absent is absent because no consumer asks for it, never
//! because a value resisted settling: a field absent from this struct is a
//! question the live side does not ask, not a value we failed to settle.
//!
//! ## What cycle 12 added
//!
//! The weapon half -- what `damage_total` reads (base damage dice and the
//! `BASEITEM:` stand-in identity, wield category, critical threat range and
//! multiplier) -- plus the two values a record contributes **as a referenced
//! modifier** (`damage_size_steps`, `weight_divisor`) and the two weapon
//! predicates `equipment_effects` asked of a parser row (`is_natural_attack`,
//! `is_shield`, with `states_base_damage` carrying the presence half of the
//! "is this an actively-wielded weapon" test). Each is a settled value; the
//! `DAMAGE:`, `BASEITEM:`, `WIELD:`, `CRITRANGE:`, `CRITMULT:`, `TYPE:`,
//! `BONUS:EQMWEAPON|DAMAGESIZE` and `BONUS:EQM|WEIGHTDIV` grammar stayed on
//! the converter.
//!
//! ## What cycle 11 added
//!
//! The armour, skill, named-variable and weapon-enhancement halves -- what
//! `equipment_effects::arms_armor`, `::general` and `::equipmods` read. Each
//! of those three modules held the ingest format's own vocabulary
//! (`ACCHECK:`, `MAXDEX:`, `SPELLFAILURE:`, `BONUS:COMBAT|AC`,
//! `BONUS:SKILL`, `BONUS:VAR`, `BONUS:WEAPON|…|TYPE=Enhancement`, `SR:`,
//! `TEMPBONUS:`, `EQMOD:`) to answer a question whose answer is one settled
//! number. The grammar moved to
//! [`crate::pcgen_import::ir_converter::equipment_record_to_corpus`]; the
//! numbers live here.

use crate::rules_core::damage_total::{DiceExpression, WieldCategory};
use crate::rules_core::equipment_effects::general::{SkillCheckBonus, VarBonus};
use crate::rules_core::equipment_effects::equipmods::WeaponEnhancementBonus;
use crate::rules_core::equipment_effects::intelligent_item::IntelligentItemContribution;
use crate::rules_core::equipment_effects::magic_items::AbilityScoreBonus;
use crate::rules_core::equipment_effects::EquipmentStatEffect;

/// One standing bonus an item states. `target` is the sheet's own word: `ability:strength`,
/// `save:will`, `skill:acrobatics`, `attack`, `initiative`, `speed`, `cmb`, `cmd`; or `skill_set:all`
/// and the like, a word `equipment_effects::skill_set_rows` reads as rows; or `speed_percent`, which
/// is a multiplier in whole percent and not an addend.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StatedBonus {
    pub target: String,
    /// The engine's word for the type. `None` adds to any other.
    pub bonus_type: Option<String>,
    pub value: i16,
}

/// One equipment or equipment-modifier item as the live side holds it: the
/// converted, settled values of a corpus equipment record.
///
/// `None` on an optional field is honest absence -- the source record does not
/// state that value -- never a fabricated zero. This is the same discipline
/// the consumer modules stated before the values moved here.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CorpusEquipmentRecord {
    /// The item's corpus identity: its own key when the source record carried
    /// one, else its name. The same rule
    /// [`crate::rules_core::equipment_resolver::equipment_id_resolve`] uses to
    /// decide what a needle resolves against.
    pub identity: String,
    /// The item's display name.
    pub name: String,
    /// Item weight in pounds.
    pub weight_lbs: Option<f64>,
    /// Item price in gold pieces.
    pub cost_gp: Option<f64>,
    /// The item's settled ability-score enhancement, e.g. a Belt of Giant
    /// Strength +2's `+2` to Str, or a Potion of Bull's Strength's `+4`.
    pub ability_score_bonus: Option<AbilityScoreBonus>,
    /// The item's settled contribution to an intelligent item's stat block.
    /// See [`IntelligentItemContribution`]'s own doc comment for why
    /// `ego_bonus` is a partial sum.
    pub intelligent_item: Option<IntelligentItemContribution>,
    /// The item's own settled armour/shield stat contribution: AC bonus, max
    /// Dex, arcane spell-failure chance and armour check penalty. Every field
    /// `None` for an item that states none of them (a longsword, a trade
    /// good) -- honest absence, never a fabricated zero.
    pub stat_effect: EquipmentStatEffect,
    /// The item's AC contribution **as a referenced modifier**: the same
    /// number [`Self::stat_effect`]'s `armor_class_bonus` carries, minus the
    /// consumable-triggered fallback, which is a character-side temporary
    /// effect and is never summed into a host item's armour total. Kept as
    /// its own field because a modifier record is read only for this value.
    pub armor_class_chain_bonus: Option<i16>,
    /// The bonus type of the chain [`Self::stat_effect`]'s `armor_class_bonus` was read from, in the
    /// engine's word (`Deflection`, `Armor`, `NaturalArmor`). `None` when the chain states none, or
    /// says it stacks with itself, so it adds to any other.
    #[serde(default)]
    pub armor_class_bonus_type: Option<String>,
    /// What the item states standing on the wearer's saves, ability scores and named skills, one
    /// row per target. A condition (a class, a race, a deity) or a formula keeps a chain out: the
    /// item's words print it.
    #[serde(default)]
    pub stated_bonuses: Vec<StatedBonus>,
    /// The item's settled circumstance bonus to one named skill.
    pub skill_check_bonus: Option<SkillCheckBonus>,
    /// The item's settled flat bonuses to named rules variables, one row per
    /// name. Empty when the item states none.
    pub var_bonuses: Vec<VarBonus>,
    /// The item's settled weapon to-hit / damage enhancement.
    pub weapon_enhancement: Option<WeaponEnhancementBonus>,
    /// The item's settled flat Spell Resistance grant.
    pub spell_resistance_bonus: Option<i16>,
    /// The corpus identities of the modifier items attached to this one, in
    /// the order the source record names them. A **list of item identities**,
    /// not a token: the attachment grammar the identities were read out of
    /// stays on the converter side. Some entries resolve to no corpus record
    /// at all -- that is the same resolve-or-skip discipline the live side
    /// applied to the token's own segments before the list was settled.
    pub eqmod_references: Vec<String>,
    /// The item's settled base damage die, e.g. a longsword's `1d8`. `None`
    /// for an item that states no base damage at all, **and** for one whose
    /// stated value is not PF1's canonical `<count>d<size>` shape -- the same
    /// honest absence [`crate::rules_core::damage_total::DiceExpression::parse`]
    /// gave before the value was settled, never a fabricated roll.
    pub base_damage_dice: Option<DiceExpression>,
    /// Whether the source record states a base damage value **at all**,
    /// including one [`Self::base_damage_dice`] declines to parse. This is the
    /// "is this an actively-wielded weapon" signal, which has always been
    /// presence, not parseability: an item that states damage it does not
    /// spell as `<count>d<size>` is still a weapon.
    pub states_base_damage: bool,
    /// The corpus identity of the item whose own stats stand in for this
    /// one's, when the source record states none of its own -- the real corpus
    /// convention behind `Crossbow (Light)` naming `Light Crossbow (Base)`. An
    /// **item identity**, not a token: the caller chases it one hop through the
    /// same resolver, exactly as it did before the identity was settled.
    pub base_item: Option<String>,
    /// The item's settled wield category, which governs how much of the
    /// wielder's Strength modifier reaches its damage roll (CRB p.187). `None`
    /// for an item that states none, and for one stating a category outside
    /// PF1's three.
    pub wield_category: Option<WieldCategory>,
    /// The item's settled critical threat range as inclusive natural-roll
    /// bounds, e.g. `(19, 20)` for a longsword. `None` for an item that states
    /// none, and for a stated width outside `1..=20`.
    pub critical_threat_range: Option<(u8, u8)>,
    /// The item's settled critical-hit damage multiplier, e.g. `2` for a
    /// longsword and `4` for a scythe. `None` for an item that states none,
    /// and for a stated multiplier below `2`.
    pub critical_multiplier: Option<u8>,
    /// The number of steps this item, **as a referenced modifier**, moves its
    /// host weapon's single-die damage progression -- Shield Spikes' `+1`
    /// step. `0` for an item that states none, which is the same "no step to
    /// apply" the caller distinguished before the value was settled.
    pub damage_size_steps: i32,
    /// The divisor this item, **as a referenced modifier**, applies to its
    /// host item's weight -- Darkleaf Cloth's `2`. `None` for an item that
    /// states none.
    pub weight_divisor: Option<f32>,
    /// Whether this item is one of PF1's natural attacks. The scope signal a
    /// `natural_attack_only` weapon enhancement (the Amulet of Mighty Fists
    /// family) is checked against before it may reach an ordinary weapon's
    /// roll.
    pub is_natural_attack: bool,
    /// Whether this item is a shield. A real corpus shield states a genuine
    /// shield-bash damage die, and a worn-but-not-bashing shield must not
    /// count as a second wielded weapon -- see
    /// [`crate::rules_core::equipment_effects`]'s own doc comment for the
    /// correction that forced this distinction.
    pub is_shield: bool,
    /// Whether the source record is an equipment **modifier** rather than an
    /// item in its own right.
    ///
    /// SD-35 `AT-35-E6-003-RULED` cycle 13. The canonical envelope answers
    /// `SourceContentPayload::kind_token()` with `"EQUIP"` or `"EQUIPMOD"`, and
    /// it used to read that straight off the parser row's own
    /// `EquipmentRecordKind`. The row left the envelope this cycle, so the
    /// distinction is settled here instead -- the same two-valued answer, from
    /// the same source record, on the side that ships.
    pub is_modifier: bool,
    /// Whether the item is a container a character can put other copies in: its type holds
    /// `Container` and it is not a weapon (SD-41 `decisions.md §31` R2). A weapon's own `CONTAINS`
    /// names its loaded ammunition, not a carried bag. Omitted from the settled bundle when `false`.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub container: bool,
    /// What a container holds, settled only when [`Self::container`] is true; `Unstated` otherwise.
    #[serde(default, skip_serializing_if = "ContainerCapacity::is_unstated")]
    pub container_capacity: ContainerCapacity,
}

/// A container's stated capacity (SD-41 `decisions.md §7`). `Unstated` covers a container that
/// states none and the weight-reduction kinds, whose contents weigh in full (DEF-1).
#[derive(Debug, Clone, Copy, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ContainerCapacity {
    Pounds(f64),
    Unlimited,
    #[default]
    Unstated,
}

impl ContainerCapacity {
    pub fn is_unstated(&self) -> bool {
        matches!(self, Self::Unstated)
    }
}

impl CorpusEquipmentRecord {
    /// Whether other copies can be put inside this one.
    pub fn is_container(&self) -> bool {
        self.container
    }

    /// The container's capacity; `Unstated` for a non-container.
    pub fn capacity(&self) -> ContainerCapacity {
        self.container_capacity
    }

    /// Weight in pounds and price in gold, the pair
    /// [`crate::rules_core::encumbrance`] reads for one carried item.
    pub fn weight_and_cost(&self) -> (Option<f64>, Option<f64>) {
        (self.weight_lbs, self.cost_gp)
    }
}


#[cfg(test)]
mod container_tests {
    //! SD-41 E1.1 (`decisions.md §7, §31`): which settled records are containers and what they hold.
    //! The raw-corpus walks below are this test's second implementation of each figure; the live
    //! side reads only the settled `container` / `container_capacity` fields.
    use super::{ContainerCapacity, CorpusEquipmentRecord};
    use crate::rules_core::settled_corpus::{corpus_book_dirs, read_equipment_bundle, SettledBundleError};
    use std::path::{Path, PathBuf};

    fn repo() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn json_files(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                json_files(&path, out);
            } else if path.extension().is_some_and(|ext| ext == "json") {
                out.push(path);
            }
        }
    }

    /// Every settled equipment record across the 30 corpus books, keyed `<book>/<path>`.
    fn settled() -> Vec<(String, CorpusEquipmentRecord)> {
        let mut rows = Vec::new();
        for book in corpus_book_dirs(&repo().join("data/corpus")) {
            let name = book.file_name().unwrap().to_string_lossy().to_string();
            let bundle = match read_equipment_bundle(&book) {
                Ok(bundle) => bundle,
                Err(SettledBundleError::Missing) if !book.join("equipment").is_dir() => continue,
                Err(err) => panic!("{}: {err}", book.display()),
            };
            rows.extend(bundle.records.into_iter().map(|(key, entry)| (format!("{name}/{key}"), entry.record)));
        }
        rows
    }

    fn settled_record(key: &str) -> CorpusEquipmentRecord {
        settled().into_iter().find(|(k, _)| k == key).unwrap_or_else(|| panic!("no settled record {key}")).1
    }

    /// The source corpus records' first value per token key, from `data/corpus/<book>/equipment/**`.
    fn corpus_token_maps() -> Vec<std::collections::BTreeMap<String, String>> {
        let mut files = Vec::new();
        for book in corpus_book_dirs(&repo().join("data/corpus")) {
            json_files(&book.join("equipment"), &mut files);
        }
        files
            .iter()
            .filter_map(|path| {
                let value: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).ok()?).ok()?;
                let tokens = value.get("data")?.get("raw_tokens")?.as_array()?;
                let mut map = std::collections::BTreeMap::new();
                for token in tokens {
                    if let (Some(k), Some(v)) = (token["key"].as_str(), token["value"].as_str()) {
                        map.entry(k.to_string()).or_insert_with(|| v.to_string());
                    }
                }
                Some(map)
            })
            .collect()
    }

    fn kind_of(capacity: &ContainerCapacity) -> &'static str {
        match capacity {
            ContainerCapacity::Pounds(_) => "lb",
            ContainerCapacity::Unlimited => "unlim",
            ContainerCapacity::Unstated => "unstated",
        }
    }

    #[test]
    fn container_tag_population_is_232_files_265_rows() {
        let mut files = Vec::new();
        for book in std::fs::read_dir(repo().join("data/sheet_rules")).unwrap().flatten() {
            json_files(&book.path().join("equipment"), &mut files);
        }
        let (mut tagged_files, mut tagged_rows, mut weapon_rows) = (0, 0, 0);
        for path in &files {
            let rows: Vec<serde_json::Value> = match serde_json::from_str(&std::fs::read_to_string(path).unwrap()) {
                Ok(serde_json::Value::Array(rows)) => rows,
                _ => continue,
            };
            let has = |row: &serde_json::Value, tag: &str| {
                row["tags"].as_array().is_some_and(|tags| tags.iter().any(|t| t == tag))
            };
            let container_rows: Vec<_> = rows.iter().filter(|row| has(row, "Container")).collect();
            tagged_files += usize::from(!container_rows.is_empty());
            tagged_rows += container_rows.len();
            weapon_rows += container_rows.iter().filter(|row| has(row, "Weapon")).count();
        }
        assert_eq!((tagged_files, tagged_rows, weapon_rows), (232, 265, 123), "sheet-rule Container tag population");
    }

    #[test]
    fn container_capacity_population_is_210_88_73_49() {
        // The source population: every corpus record with a CONTAINS token, by its first field.
        let maps = corpus_token_maps();
        let first_kind = |value: &str| {
            let first = value.split('|').next().unwrap_or("");
            if first.starts_with("UNLIM") {
                "unlim"
            } else if first.starts_with('*') {
                "unstated"
            } else if first.parse::<f64>().is_ok() {
                "lb"
            } else {
                "other"
            }
        };
        let contains: Vec<&str> = maps.iter().filter_map(|m| m.get("CONTAINS").map(|v| first_kind(v))).collect();
        let count = |rows: &[&str], kind: &str| rows.iter().filter(|k| **k == kind).count();
        assert_eq!(
            (contains.len(), count(&contains, "lb"), count(&contains, "unlim"), count(&contains, "unstated"), count(&contains, "other")),
            (210, 88, 73, 49, 0),
            "CONTAINS records by first field"
        );

        // The settled containers: Container-typed, not a weapon (no DAMAGE, no Weapon type word).
        // Implementation A: the settled bundles. Implementation B: the source records.
        let settled = settled();
        let a: Vec<&str> =
            settled.iter().filter(|(_, r)| r.is_container()).map(|(_, r)| kind_of(&r.capacity())).collect();
        let b: Vec<&str> = maps
            .iter()
            .filter(|m| {
                let words: Vec<&str> = m.get("TYPE").map_or(vec![], |t| t.split('.').collect());
                words.contains(&"Container") && !words.contains(&"Weapon") && !m.contains_key("DAMAGE")
            })
            .map(|m| m.get("CONTAINS").map_or("unstated", |v| match first_kind(v) {
                "other" => "unstated",
                kind => kind,
            }))
            .collect();
        let tally = |rows: &[&str]| (rows.len(), count(rows, "lb"), count(rows, "unlim"), count(rows, "unstated"));
        assert_eq!(tally(&a), tally(&b), "settled containers agree with the source predicate");
        assert_eq!(tally(&a), (130, 32, 50, 48), "settled containers: total, lb, unlimited, unstated");
        assert!(
            settled.iter().all(|(_, r)| r.is_container() || r.capacity() == ContainerCapacity::Unstated),
            "capacity is settled only on containers"
        );
    }

    #[test]
    fn container_capacity_reads_backpack_unlimited() {
        for key in [
            "core_rulebook/general/backpack.json",
            "advanced_players_guide/backpack_masterwork.json",
            "ultimate_equipment/powder_horn.json",
        ] {
            let record = settled_record(key);
            assert!(record.is_container(), "{key} is a container");
            assert_eq!(record.capacity(), ContainerCapacity::Unlimited, "{key}: UNLIM|…");
        }
        assert_eq!(settled_record("core_rulebook/general/sled.json").capacity(), ContainerCapacity::Pounds(2000.0));
        assert_eq!(settled_record("core_rulebook/general/waterskin.json").capacity(), ContainerCapacity::Pounds(4.0));
    }

    #[test]
    fn container_weapon_with_contains_is_not_a_container() {
        // Pistol states DAMAGE 1d8; Wrist Launcher states no DAMAGE but is Weapon-typed. Both name
        // their loaded ammunition in CONTAINS, which is not a carried bag (operator, §31 R2).
        for key in ["ultimate_equipment/pistol.json", "ultimate_intrigue/wrist_launcher.json"] {
            let record = settled_record(key);
            assert!(!record.is_container(), "{key} is a weapon, not a container");
            assert_eq!(record.capacity(), ContainerCapacity::Unstated, "{key}");
        }
    }
}
