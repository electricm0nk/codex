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
}

impl CorpusEquipmentRecord {
    /// Weight in pounds and price in gold, the pair
    /// [`crate::rules_core::encumbrance`] reads for one carried item.
    pub fn weight_and_cost(&self) -> (Option<f64>, Option<f64>) {
        (self.weight_lbs, self.cost_gp)
    }
}

