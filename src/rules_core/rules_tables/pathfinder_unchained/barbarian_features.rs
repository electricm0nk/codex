//! Pathfinder Unchained — **Unchained Barbarian** class features, one
//! hand-modelled pure function per magnitude.
//!
//! # Why hand-modelled and not interpreted
//!
//! `decisions.md §24.1` forbids a general `BONUS:`/`DEFINE:`/`PREREQ:`
//! formula interpreter and mandates the shape the 27 already-shipped
//! classes use: a small pure function whose formula is verified
//! byte-exact against the corpus row, plus a test. Every function below
//! names the exact source token it transcribes and the exact
//! `pu_abilities_class.lst` line it came from. Nothing here parses a
//! formula at runtime.
//!
//! # What "Unchained Barbarian" actually is in PCGen
//!
//! Pathfinder Unchained declares **zero `CLASS:` objects**. `Barbarian ~
//! Unchained Class` is a `CATEGORY:CLASS` *selection ability* layered
//! over Core Rulebook's real `CLASS:Barbarian` (`cr_classes.lst:7`,
//! `MAXLEVEL:20` — the origin of [`MAX_SUPPORTED_LEVEL`]). The variant
//! record overrides **no** chassis field: its `hit_die`, `bab` and all
//! three save columns are `null` in
//! `data/corpus/pathfinder_unchained/class/barbarian_unchained_class.json`,
//! so the Unchained Barbarian keeps CRB's d12 / full BAB / good-Fort
//! chassis (`rules_tables::crb::class_tables`) unchanged. **This module
//! therefore carries no chassis table** — inventing one would be a second,
//! competing statement of a fact CRB already owns. Only the *features*
//! differ, and only those are modelled here.
//!
//! # Coexistence with the Core Rulebook Barbarian
//!
//! Nothing in `rules_tables::crb` or `pilot_compute.rs` is touched. The
//! CRB Barbarian's own Rage functions (`barbarian_rage_rounds_per_day`,
//! the `+4 Str / +4 Con / +2 Will` magnitude tier) stay exactly where they
//! are; this module is an additive sibling under a distinct namespace with
//! distinct function names. The two are genuinely different rules and the
//! numbers say so:
//!
//! | | CRB Barbarian | Unchained Barbarian |
//! |---|---|---|
//! | rage benefit | `+4` Str, `+4` Con, `+2` Will (temporary ability scores) | flat `+2` morale to melee attack, melee/thrown damage **and** Will ([`rage_morale_bonus`]) |
//! | rage temp HP | none (Con score raise gives HP indirectly) | `2 × character level` ([`rage_temporary_hit_points`]) |
//! | rounds/day | `4 + Con + 2*(level-1)` | `2 + Con + 2*level` ([`rage_rounds_per_day`]) — algebraically the same ladder, transcribed from PU's own token rather than reused |
//! | AC penalty | `-2` | `-2` ([`RAGE_ARMOR_CLASS_PENALTY`]) |
//!
//! A campaign picks one; both remain resolvable.
//!
//! # Where the Rage magnitudes physically live
//!
//! `Unchained Barbarian ~ Rage` (line 290) carries only
//! `BONUS:VAR|RageLVL|BarbarianLVL` and a `DESC:` whose `%1`–`%4` args are
//! `RageDuration|RageBonus|RageACPenalty|RageBonusHP`. Those four variables
//! are filled by a **separate row**, `KEY:Standard Unchained Rage`
//! (line 306, `TYPE:UnchainedRageSelection`), which is the default option
//! inside line 303's `BONUS:ABILITYPOOL|Unchained Raging Selection|1`.
//!
//! That row is a *selectable option*, so the ingestion cycle correctly did
//! **not** write it under `class_feature/` — it is one of the 140 option
//! rows that cycle reported as deliberately unwritten. Consequently the
//! four rage magnitudes below cannot be pinned against
//! `data/corpus/**.json`; they are pinned against the raw `.lst` instead,
//! by `rage_magnitudes_are_byte_exact_against_the_real_lst_row`
//! (`#[ignore]`d, opt-in via `PCGEN_CORPUS_ROOT`, exactly as
//! `tests/sd17_b1_martial_class.rs` does). Everything else in this module
//! is pinned against the in-repo corpus JSON on every plain `cargo test`.
//!
//! # Grant levels
//!
//! Read off `PREVARGTEQ:Barbarian_CFP_Level,N` on the progression rows
//! (`pu_abilities_class.lst:131-141`, the `Barbarian ~ Unchained Class
//! Full.MOD` block). A second, shorter block (`Barbarian ~ Unchained
//! Ex-Class.MOD`, lines 145-150) grants a 6-feature subset and **disagrees
//! on one row**: Ex-Class line 145 states `PREVARGTEQ:Barbarian_CFP_Level,1`
//! for Weapon and Armor Proficiency where Full line 131 states no level at
//! all. This module takes the Full (primary) progression, so that feature's
//! [`UnchainedBarbarianFeature::min_level`] is `None`; the disagreement is
//! recorded here rather than silently resolved.

/// One ingested `class_feature` record for the Unchained Barbarian.
///
/// The roster is exactly the 14 records under
/// `data/corpus/pathfinder_unchained/class_feature/barbarian_unchained_class/`
/// — no more (nothing invented) and no fewer (nothing quietly dropped).
/// `corpus_line` is the 1-based line in `pu_abilities_class.lst`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnchainedBarbarianFeature {
    /// PCGen `KEY:` token.
    pub key: &'static str,
    /// PCGen display name (the row's first column).
    pub name: &'static str,
    /// Grant level from `PREVARGTEQ:Barbarian_CFP_Level,N` on the
    /// `Barbarian ~ Unchained Class Full.MOD` progression row. `None` means
    /// the progression row states no level (see the module doc comment).
    pub min_level: Option<u8>,
    /// Whether any progression row grants this record at all. Three records
    /// are declared but never granted — a real corpus fact, kept rather
    /// than dropped.
    pub is_granted: bool,
    /// 1-based line in `pu_abilities_class.lst`.
    pub corpus_line: u32,
}

/// `MAXLEVEL:20` on the base `CLASS:Barbarian` record
/// (`core_rulebook/cr_classes.lst:7`). PU adds no levels.
pub const MAX_SUPPORTED_LEVEL: u8 = 20;

/// `pu_abilities_class.lst:132` — `PREVARGTEQ:Barbarian_CFP_Level,1`.
pub const RAGE_LEVEL: u8 = 1;
/// `pu_abilities_class.lst:133` — `PREVARGTEQ:Barbarian_CFP_Level,1`.
pub const FAST_MOVEMENT_LEVEL: u8 = 1;
/// `pu_abilities_class.lst:134` — `PREVARGTEQ:Barbarian_CFP_Level,2`.
pub const RAGE_POWERS_LEVEL: u8 = 2;
/// `pu_abilities_class.lst:301` — `PREVARGTEQ:BarbarianLVL,2` on the
/// Uncanny Dodge Tracker's first `BONUS:VAR|UncannyDodgeLVL|1` chain.
pub const UNCANNY_DODGE_LEVEL: u8 = 2;
/// `pu_abilities_class.lst:136` — `PREVARGTEQ:Barbarian_CFP_Level,3`.
pub const DANGER_SENSE_LEVEL: u8 = 3;
/// `pu_abilities_class.lst:301` — `PREVARGTEQ:BarbarianLVL,5` on the
/// Uncanny Dodge Tracker's second `BONUS:VAR|UncannyDodgeLVL|1` chain.
pub const IMPROVED_UNCANNY_DODGE_LEVEL: u8 = 5;
/// `pu_abilities_class.lst:137` — `PREVARGTEQ:Barbarian_CFP_Level,7`.
pub const DAMAGE_REDUCTION_LEVEL: u8 = 7;
/// `pu_abilities_class.lst:138` — `PREVARGTEQ:Barbarian_CFP_Level,11`.
pub const GREATER_RAGE_LEVEL: u8 = 11;
/// `pu_abilities_class.lst:139` — `PREVARGTEQ:Barbarian_CFP_Level,14`.
pub const INDOMITABLE_WILL_LEVEL: u8 = 14;
/// `pu_abilities_class.lst:140` — `PREVARGTEQ:Barbarian_CFP_Level,17`.
pub const TIRELESS_RAGE_LEVEL: u8 = 17;
/// `pu_abilities_class.lst:141` — `PREVARGTEQ:Barbarian_CFP_Level,20`.
pub const MIGHTY_RAGE_LEVEL: u8 = 20;

/// `BONUS:VAR|RageACPenalty|-2` (`pu_abilities_class.lst:306`). Flat at
/// every tier — Greater Rage and Mighty Rage add to `RageBonus` and
/// `RageBonusHP` only, never to this, so there is no level argument to
/// take. Matches CRB Barbarian's own unchanging `-2`.
pub const RAGE_ARMOR_CLASS_PENALTY: i16 = -2;

/// `BONUS:MOVEADD|TYPE=Walk|10|PREVARLT:ENCUMBERANCE,2,...`
/// (`pu_abilities_class.lst:289`). A flat literal, not a progression —
/// unlike the Unchained Monk's own Fast Movement, which scales
/// (`super::monk_features::fast_movement_bonus_feet`).
pub const FAST_MOVEMENT_BONUS_FEET: i16 = 10;

/// `ASPECT:SaveBonus|+4 bonus to Will saves vs. Enchantment spells while
/// raging` (`pu_abilities_class.lst:297`). PCGen carries no `BONUS:SAVE`
/// token for this — the condition ("while raging", "vs. enchantment") is
/// not expressible as an unconditional save bonus — so the magnitude is
/// read off the same row's own structured `ASPECT:` token, which is
/// transcription under `decisions.md §24`.
pub const INDOMITABLE_WILL_SAVE_BONUS: i16 = 4;

/// The 14 ingested Unchained Barbarian `class_feature` records, in
/// `pu_abilities_class.lst` line order.
///
/// `Uncanny Dodge` (298), `Improved Uncanny Dodge` (299) and `Unchained
/// Rage` (303) carry `is_granted: false`: no progression row names them.
/// The first two are reached indirectly through the Uncanny Dodge Tracker
/// (301), which is what the progression actually grants; the third is
/// reached through line 290's
/// `ABILITY:Special Ability|AUTOMATIC|Unchained Rage`.
const FEATURES: &[UnchainedBarbarianFeature] = &[
    UnchainedBarbarianFeature {
        key: "Unchained Barbarian ~ Weapon and Armor Proficiency",
        name: "Weapon and Armor Proficiency",
        min_level: None,
        is_granted: true,
        corpus_line: 288,
    },
    UnchainedBarbarianFeature {
        key: "Unchained Barbarian ~ Fast Movement",
        name: "Fast Movement",
        min_level: Some(FAST_MOVEMENT_LEVEL),
        is_granted: true,
        corpus_line: 289,
    },
    UnchainedBarbarianFeature {
        key: "Unchained Barbarian ~ Rage",
        name: "Rage",
        min_level: Some(RAGE_LEVEL),
        is_granted: true,
        corpus_line: 290,
    },
    UnchainedBarbarianFeature {
        key: "Unchained Barbarian ~ Rage Powers",
        name: "Rage Powers",
        min_level: Some(RAGE_POWERS_LEVEL),
        is_granted: true,
        corpus_line: 291,
    },
    UnchainedBarbarianFeature {
        key: "Unchained Barbarian ~ Danger Sense",
        name: "Danger Sense",
        min_level: Some(DANGER_SENSE_LEVEL),
        is_granted: true,
        corpus_line: 292,
    },
    UnchainedBarbarianFeature {
        key: "Unchained Barbarian ~ Damage Reduction",
        name: "Damage Reduction",
        min_level: Some(DAMAGE_REDUCTION_LEVEL),
        is_granted: true,
        corpus_line: 293,
    },
    UnchainedBarbarianFeature {
        key: "Unchained Barbarian ~ Greater Rage",
        name: "Greater Rage",
        min_level: Some(GREATER_RAGE_LEVEL),
        is_granted: true,
        corpus_line: 294,
    },
    UnchainedBarbarianFeature {
        key: "Unchained Barbarian ~ Tireless Rage",
        name: "Tireless Rage",
        min_level: Some(TIRELESS_RAGE_LEVEL),
        is_granted: true,
        corpus_line: 295,
    },
    UnchainedBarbarianFeature {
        key: "Unchained Barbarian ~ Mighty Rage",
        name: "Mighty Rage",
        min_level: Some(MIGHTY_RAGE_LEVEL),
        is_granted: true,
        corpus_line: 296,
    },
    UnchainedBarbarianFeature {
        key: "Unchained Barbarian ~ Indomitable Will",
        name: "Indomitable Will",
        min_level: Some(INDOMITABLE_WILL_LEVEL),
        is_granted: true,
        corpus_line: 297,
    },
    UnchainedBarbarianFeature {
        key: "Unchained Barbarian ~ Uncanny Dodge",
        name: "Uncanny Dodge",
        min_level: None,
        is_granted: false,
        corpus_line: 298,
    },
    UnchainedBarbarianFeature {
        key: "Unchained Barbarian ~ Improved Uncanny Dodge",
        name: "Improved Uncanny Dodge",
        min_level: None,
        is_granted: false,
        corpus_line: 299,
    },
    UnchainedBarbarianFeature {
        key: "Unchained Barbarian ~ Uncanny Dodge Tracker",
        name: "Unchained Barbarian ~ Uncanny Dodge Tracker",
        min_level: Some(UNCANNY_DODGE_LEVEL),
        is_granted: true,
        corpus_line: 301,
    },
    UnchainedBarbarianFeature {
        key: "Unchained Rage",
        name: "Rage",
        min_level: None,
        is_granted: false,
        corpus_line: 303,
    },
];

/// The full ingested feature roster.
pub fn features() -> &'static [UnchainedBarbarianFeature] {
    FEATURES
}

/// Looks a feature up by its PCGen `KEY:`.
pub fn feature(key: &str) -> Option<&'static UnchainedBarbarianFeature> {
    FEATURES.iter().find(|f| f.key == key)
}

/// Rage rounds per day: `2 + Constitution modifier + 2 × barbarian level`.
///
/// Two tokens, both transcribed byte-exact:
/// - `pu_abilities_class.lst:306` (`KEY:Standard Unchained Rage`) —
///   `BONUS:VAR|RageDuration|2+var("STAT.2.MOD.NOTEMP")+(2*RageLVL)`
/// - `pu_abilities_class.lst:290` (`KEY:Unchained Barbarian ~ Rage`) —
///   `BONUS:VAR|RageLVL|BarbarianLVL`
///
/// `STAT.2` is Constitution: `core_rulebook/cr__stats.lst` lists the six
/// stats with `SORTKEY:1`–`SORTKEY:6` in the order Str, Dex, **Con**, Int,
/// Wis, Cha, and PCGen's `STAT.<n>` index is 0-based. That is the one
/// substitution this function makes and it is checked, not assumed —
/// `constitution_is_pcgen_stat_index_2` re-reads the stats file.
///
/// `None` below [`RAGE_LEVEL`]. Note this reads `2 + 2*level`, **not**
/// CRB's `4 + 2*(level-1)`; the two are algebraically identical but this
/// one is PU's own token, transcribed rather than borrowed.
pub fn rage_rounds_per_day(level: u8, constitution_modifier: i16) -> Option<i16> {
    if level < RAGE_LEVEL {
        return None;
    }
    Some(2 + constitution_modifier + (2 * i16::from(level)))
}

/// The morale bonus Unchained Rage grants to melee attack rolls, melee and
/// thrown damage rolls, and Will saves — `+2`, rising to `+3` at
/// [`GREATER_RAGE_LEVEL`] and `+4` at [`MIGHTY_RAGE_LEVEL`].
///
/// Three tokens stacking on one variable:
/// - `:306` `BONUS:VAR|RageBonus|2` (base)
/// - `:294` `BONUS:VAR|RageBonus|1` (Greater Rage, granted at 11)
/// - `:296` `BONUS:VAR|RageBonus|1` (Mighty Rage, granted at 20)
///
/// The four places it lands are line 309's benefit row
/// (`Grant Standard Unchained Rage Benefits`):
/// `BONUS:COMBAT|TOHIT.Melee|RageBonus|TYPE=Rage`,
/// `BONUS:COMBAT|DAMAGE.Melee,DAMAGE.Thrown|RageBonus|TYPE=Rage` and
/// `BONUS:SAVE|Will|RageBonus|TYPE=Rage`.
///
/// This is the single sharpest difference from the CRB Barbarian, which
/// instead raises Strength and Constitution by 4 and Will by 2.
pub fn rage_morale_bonus(level: u8) -> Option<i16> {
    if level < RAGE_LEVEL {
        return None;
    }
    let mut bonus = 2;
    if level >= GREATER_RAGE_LEVEL {
        bonus += 1;
    }
    if level >= MIGHTY_RAGE_LEVEL {
        bonus += 1;
    }
    Some(bonus)
}

/// [`RAGE_ARMOR_CLASS_PENALTY`], or `None` below [`RAGE_LEVEL`]. Exposed as
/// a function as well as a constant so callers that already hold a level
/// get the same "does this feature exist yet" gate as every other
/// magnitude here.
pub fn rage_armor_class_penalty(level: u8) -> Option<i16> {
    if level < RAGE_LEVEL {
        return None;
    }
    Some(RAGE_ARMOR_CLASS_PENALTY)
}

/// Temporary hit points granted by Unchained Rage:
/// `character level × (2, or 3 from level 11, or 4 from level 20)`.
///
/// - `:306` `BONUS:VAR|RageBonusHP|TL*2`
/// - `:294` `BONUS:VAR|RageBonusHP|TL` (Greater Rage)
/// - `:296` `BONUS:VAR|RageBonusHP|TL` (Mighty Rage)
///
/// **Two different levels are involved and the corpus says so.** `TL` is
/// *total* character level, so the multiplicand is the character's level;
/// but the two extra `TL` contributions are gated on
/// `PREVARGTEQ:Barbarian_CFP_Level,11` / `,20` on the progression rows, so
/// the *multiplier* keys off barbarian class level. A Barbarian 11 /
/// Fighter 4 therefore gets `15 × 3 = 45`, not `11 × 3`. Collapsing the two
/// arguments into one would be wrong for every multiclass character, which
/// is exactly the kind of plausible-looking error `decisions.md §24`
/// exists to make loud.
///
/// `None` below [`RAGE_LEVEL`].
pub fn rage_temporary_hit_points(barbarian_level: u8, character_level: u8) -> Option<i16> {
    if barbarian_level < RAGE_LEVEL {
        return None;
    }
    let mut multiplier = 2;
    if barbarian_level >= GREATER_RAGE_LEVEL {
        multiplier += 1;
    }
    if barbarian_level >= MIGHTY_RAGE_LEVEL {
        multiplier += 1;
    }
    Some(i16::from(character_level) * multiplier)
}

/// Rage powers known: `barbarian level / 2` (integer division).
///
/// `pu_abilities_class.lst:291` —
/// `BONUS:ABILITYPOOL|Unchained Rage Power|RagePowersLVL/2` with the same
/// row's `BONUS:VAR|RagePowersLVL|BarbarianLVL`. `None` below
/// [`RAGE_POWERS_LEVEL`].
///
/// **The 54 Unchained Rage Powers this pool is spent on are not modelled
/// anywhere in this repo.** The ingestion cycle deliberately did not write
/// them (they are options a feature offers, not features the class has, and
/// need their own content-kind directory). This function returns the size of
/// the pool, which is a real number; it does not imply a catalogue exists.
pub fn rage_powers_known(level: u8) -> Option<i16> {
    if level < RAGE_POWERS_LEVEL {
        return None;
    }
    Some(i16::from(level) / 2)
}

/// Danger Sense's bonus on Reflex saves against traps and on Perception
/// checks to notice them: `barbarian level / 3`.
///
/// `pu_abilities_class.lst:292` —
/// `BONUS:VAR|TrapSenseBonus|BarbarianTrapSenseLVL/3` with the same row's
/// `BONUS:VAR|BarbarianTrapSenseLVL|BarbarianLVL`. `None` below
/// [`DANGER_SENSE_LEVEL`].
///
/// Note the absence of the `max(1,...)` wrapper that ACG's Slayer Trap
/// Sense carries (`BONUS:VAR|TrapSenseBonus|max(1,SlayerTrapSenseLVL/3)`).
/// PU's row genuinely does not have it — transcribed per-row rather than
/// carried across by assumption. It makes no difference in practice because
/// the grant level is 3, but the difference is real and is not smoothed
/// over.
pub fn danger_sense_bonus(level: u8) -> Option<i16> {
    if level < DANGER_SENSE_LEVEL {
        return None;
    }
    Some(i16::from(level) / 3)
}

/// Damage reduction `X/—`: `(barbarian level - 4) / 3`.
///
/// `pu_abilities_class.lst:293` — `BONUS:VAR|BarbarianDR|(BarbarianDRLVL-4)/3`
/// with the same row's `BONUS:VAR|BarbarianDRLVL|BarbarianLVL`, applied by
/// the same row's `DR:BarbarianDR/-`. `None` below
/// [`DAMAGE_REDUCTION_LEVEL`], which is also what keeps the subtraction off
/// the negative branch.
pub fn damage_reduction(level: u8) -> Option<i16> {
    if level < DAMAGE_REDUCTION_LEVEL {
        return None;
    }
    Some((i16::from(level) - 4) / 3)
}

/// [`FAST_MOVEMENT_BONUS_FEET`], or `None` below [`FAST_MOVEMENT_LEVEL`].
///
/// The corpus token is conditioned
/// (`PREVARLT:ENCUMBERANCE,2,var("COUNT[EQTYPE.ARMOR.EQUIPPED.IS.HEAVY]"),1`
/// — light/medium load and no heavy armor). This function returns the
/// magnitude only; the load/armor condition is a caller concern and is
/// **not** silently applied here.
pub fn fast_movement_bonus_feet(level: u8) -> Option<i16> {
    if level < FAST_MOVEMENT_LEVEL {
        return None;
    }
    Some(FAST_MOVEMENT_BONUS_FEET)
}

/// [`INDOMITABLE_WILL_SAVE_BONUS`], or `None` below
/// [`INDOMITABLE_WILL_LEVEL`]. Applies only to Will saves against
/// enchantment spells **while raging** — see the constant's own note on why
/// PCGen carries no `BONUS:SAVE` token for it.
pub fn indomitable_will_save_bonus(level: u8) -> Option<i16> {
    if level < INDOMITABLE_WILL_LEVEL {
        return None;
    }
    Some(INDOMITABLE_WILL_SAVE_BONUS)
}

/// The class level Uncanny Dodge counts as for the "can a rogue of level
/// N flank me" comparison: the barbarian's own level.
///
/// `pu_abilities_class.lst:301` (Uncanny Dodge Tracker) —
/// `BONUS:VAR|UncannyDodgeFlankingLevel|BarbarianLVL|TYPE=EachClass.REPLACE|
/// PREVARGTEQ:BarbarianLVL,2|PREVAREQ:Barbarian_CF_UncannyDodge,0`. The
/// same variable is set identically by line 298. `None` below
/// [`UNCANNY_DODGE_LEVEL`].
pub fn uncanny_dodge_flanking_level(level: u8) -> Option<i16> {
    if level < UNCANNY_DODGE_LEVEL {
        return None;
    }
    Some(i16::from(level))
}

/// `UncannyDodgeLVL`: `0` before level 2, `1` at levels 2-4 (Uncanny
/// Dodge), `2` from level 5 (Improved Uncanny Dodge).
///
/// `pu_abilities_class.lst:301` carries two independent `+1` chains on one
/// variable —
/// `BONUS:VAR|UncannyDodgeLVL|1|PREVARGTEQ:BarbarianLVL,2|PREVAREQ:Barbarian_CF_UncannyDodge,0`
/// and
/// `BONUS:VAR|UncannyDodgeLVL|1|PREVARGTEQ:BarbarianLVL,5|PREVAREQ:Barbarian_CF_ImprovedUncannyDodge,0`
/// — so the tier is a count of satisfied gates, not a formula. Modelled as
/// the count.
///
/// The `PREVAREQ:Barbarian_CF_*,0` halves of those gates are archetype
/// suppression flags (an archetype that trades the feature away sets them
/// to 1). This function models the unsuppressed progression; archetype
/// swapping is not implemented anywhere in this repo and is not faked here.
pub fn uncanny_dodge_tier(level: u8) -> u8 {
    let mut tier = 0;
    if level >= UNCANNY_DODGE_LEVEL {
        tier += 1;
    }
    if level >= IMPROVED_UNCANNY_DODGE_LEVEL {
        tier += 1;
    }
    tier
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn corpus_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("data/corpus/pathfinder_unchained/class_feature/barbarian_unchained_class")
    }

    /// Every `data` block under the ingested Barbarian feature directory.
    fn ingested_records() -> Vec<serde_json::Value> {
        let mut out = Vec::new();
        let dir = corpus_dir();
        let entries = std::fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("ingested Barbarian corpus dir {dir:?} must exist: {e}"));
        for entry in entries {
            let path = entry.expect("readable dir entry").path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("readable corpus record");
            let value: serde_json::Value =
                serde_json::from_str(&text).expect("corpus record is valid JSON");
            out.push(value["data"].clone());
        }
        out
    }

    /// The `BONUS:` chain qualifiers of one record, rejoined with `|` so
    /// they can be compared against the literal source token.
    fn bonus_tokens(record: &serde_json::Value) -> Vec<String> {
        record["raw_bonus_chains"]
            .as_array()
            .expect("raw_bonus_chains is an array")
            .iter()
            .map(|chain| {
                let parts: Vec<String> = chain["qualifiers"]
                    .as_array()
                    .expect("qualifiers is an array")
                    .iter()
                    .map(|q| q.as_str().expect("qualifier is a string").to_owned())
                    .collect();
                format!("BONUS:{}", parts.join("|"))
            })
            .collect()
    }

    fn record_for(key: &str) -> serde_json::Value {
        ingested_records()
            .into_iter()
            .find(|r| r["key"] == key)
            .unwrap_or_else(|| panic!("no ingested record with KEY:{key}"))
    }

    fn assert_bonus_token(key: &str, token: &str) {
        let record = record_for(key);
        let tokens = bonus_tokens(&record);
        assert!(
            tokens.iter().any(|t| t == token),
            "{key} must carry the exact token {token:?}; it carries {tokens:?}"
        );
    }

    /// The roster must be exactly the ingested set — same size, same keys,
    /// same grant levels, same `is_granted` flags. This is the guard against
    /// this table drifting away from the corpus it claims to transcribe.
    #[test]
    fn feature_roster_is_the_fourteen_ingested_records() {
        let ingested = ingested_records();
        assert_eq!(
            ingested.len(),
            14,
            "the ingested Barbarian feature directory must hold 14 records"
        );
        assert_eq!(features().len(), ingested.len());

        for feature in features() {
            let record = record_for(feature.key);
            assert_eq!(
                record["name"].as_str(),
                Some(feature.name),
                "{}: name disagrees with the corpus",
                feature.key
            );
            assert_eq!(
                record["min_level"].as_u64().map(|v| v as u8),
                feature.min_level,
                "{}: min_level disagrees with the corpus",
                feature.key
            );
            assert_eq!(
                record["is_granted"].as_bool(),
                Some(feature.is_granted),
                "{}: is_granted disagrees with the corpus",
                feature.key
            );
            assert_eq!(
                record["source"]["line"].as_u64(),
                None,
                "source.line lives on the envelope, not the data block"
            );
        }
    }

    /// Exactly three declared-but-never-granted records, named explicitly
    /// so that a future ingestion change that starts or stops granting one
    /// fails here instead of silently changing a character sheet.
    #[test]
    fn exactly_three_features_are_declared_but_never_granted() {
        let ungranted: Vec<&str> =
            features().iter().filter(|f| !f.is_granted).map(|f| f.key).collect();
        assert_eq!(
            ungranted,
            vec![
                "Unchained Barbarian ~ Uncanny Dodge",
                "Unchained Barbarian ~ Improved Uncanny Dodge",
                "Unchained Rage",
            ]
        );
    }

    /// `2 + Con + 2*level`, and the CRB Barbarian's published
    /// `4 + Con + 2*(level-1)` must agree with it at every level — two
    /// independent statements of one ladder, pinned together so neither can
    /// drift alone.
    #[test]
    fn rage_rounds_per_day_matches_the_corpus_formula() {
        assert_eq!(rage_rounds_per_day(0, 3), None);
        assert_eq!(rage_rounds_per_day(1, 2), Some(6));
        assert_eq!(rage_rounds_per_day(1, 0), Some(4));
        assert_eq!(rage_rounds_per_day(20, 0), Some(42));
        assert_eq!(rage_rounds_per_day(5, -1), Some(11));

        for level in RAGE_LEVEL..=MAX_SUPPORTED_LEVEL {
            for con in -2..=6 {
                let crb_shape = 4 + con + 2 * (i16::from(level) - 1);
                assert_eq!(
                    rage_rounds_per_day(level, con),
                    Some(crb_shape),
                    "level {level}, Con {con}"
                );
            }
        }
    }

    /// PCGen's `STAT.2` in the rage-duration token is Constitution. Derived
    /// from the corpus rather than asserted, because getting it wrong would
    /// silently hand the Barbarian its Dexterity modifier in rage rounds.
    #[test]
    #[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
    fn constitution_is_pcgen_stat_index_2() {
        let root = PathBuf::from(
            std::env::var("PCGEN_CORPUS_ROOT")
                .expect("PCGEN_CORPUS_ROOT must point at a local pcgen/data checkout"),
        );
        let text = std::fs::read_to_string(
            root.join("pathfinder/paizo/roleplaying_game/core_rulebook/cr__stats.lst"),
        )
        .expect("cr__stats.lst must be readable");
        let names: Vec<&str> = text
            .lines()
            .filter(|l| !l.trim_start().starts_with('#') && l.contains("STATMOD:"))
            .map(|l| l.split('\t').next().expect("row has a first column").trim())
            .collect();
        assert_eq!(names.len(), 6, "PF1 declares six ability scores; got {names:?}");
        assert_eq!(names[2], "Constitution", "STAT.2 must be Constitution; got {names:?}");
    }

    /// The four Rage magnitudes live on `KEY:Standard Unchained Rage`, a
    /// selectable-option row the ingestion cycle deliberately did not write
    /// under `class_feature/`. They can only be pinned against the raw
    /// `.lst`, so this test is opt-in — but the tokens ARE checked, not
    /// assumed, whenever the corpus is available.
    #[test]
    #[ignore = "requires a local PCGen corpus checkout; set PCGEN_CORPUS_ROOT=/path/to/pcgen/data"]
    fn rage_magnitudes_are_byte_exact_against_the_real_lst_row() {
        let root = PathBuf::from(
            std::env::var("PCGEN_CORPUS_ROOT")
                .expect("PCGEN_CORPUS_ROOT must point at a local pcgen/data checkout"),
        );
        let text = std::fs::read_to_string(root.join(
            "pathfinder/paizo/roleplaying_game/pathfinder_unchained/pu_abilities_class.lst",
        ))
        .expect("pu_abilities_class.lst must be readable");
        let lines: Vec<&str> = text.lines().collect();

        let line_306 = lines[305];
        assert!(line_306.contains("KEY:Standard Unchained Rage"), "line 306 moved: {line_306}");
        for token in [
            "BONUS:VAR|RageDuration|2+var(\"STAT.2.MOD.NOTEMP\")+(2*RageLVL)",
            "BONUS:VAR|RageBonus|2",
            "BONUS:VAR|RageACPenalty|-2",
            "BONUS:VAR|RageBonusHP|TL*2",
        ] {
            assert!(line_306.contains(token), "line 306 must carry {token:?}");
        }

        let line_290 = lines[289];
        assert!(line_290.contains("KEY:Unchained Barbarian ~ Rage"), "line 290 moved: {line_290}");
        assert!(line_290.contains("BONUS:VAR|RageLVL|BarbarianLVL"));

        for (index, feature) in [(293_usize, "Greater Rage"), (295_usize, "Mighty Rage")] {
            let line = lines[index];
            assert!(line.contains(feature), "line {} moved: {line}", index + 1);
            assert!(line.contains("BONUS:VAR|RageBonus|1"), "{feature} must add +1 RageBonus");
            assert!(line.contains("BONUS:VAR|RageBonusHP|TL"), "{feature} must add TL temp HP");
        }
    }

    #[test]
    fn rage_morale_bonus_rises_only_at_greater_and_mighty_rage() {
        assert_eq!(rage_morale_bonus(0), None);
        for level in 1..GREATER_RAGE_LEVEL {
            assert_eq!(rage_morale_bonus(level), Some(2), "level {level}");
        }
        for level in GREATER_RAGE_LEVEL..MIGHTY_RAGE_LEVEL {
            assert_eq!(rage_morale_bonus(level), Some(3), "level {level}");
        }
        assert_eq!(rage_morale_bonus(MIGHTY_RAGE_LEVEL), Some(4));
    }

    /// The AC penalty never moves — the guard against someone "improving"
    /// it alongside the morale bonus.
    #[test]
    fn rage_armor_class_penalty_is_flat_at_every_tier() {
        assert_eq!(rage_armor_class_penalty(0), None);
        for level in RAGE_LEVEL..=MAX_SUPPORTED_LEVEL {
            assert_eq!(rage_armor_class_penalty(level), Some(-2), "level {level}");
        }
    }

    /// Multiclass characters are the whole reason this takes two levels.
    #[test]
    fn rage_temporary_hit_points_scale_on_character_level_and_tier_on_class_level() {
        assert_eq!(rage_temporary_hit_points(0, 5), None);
        assert_eq!(rage_temporary_hit_points(1, 1), Some(2));
        assert_eq!(rage_temporary_hit_points(10, 10), Some(20));
        assert_eq!(rage_temporary_hit_points(11, 11), Some(33));
        assert_eq!(rage_temporary_hit_points(20, 20), Some(80));
        // Barbarian 11 / Fighter 4: tier from 11 barbarian levels, magnitude
        // from 15 character levels.
        assert_eq!(rage_temporary_hit_points(11, 15), Some(45));
        // Barbarian 10 / Fighter 5: one barbarian level short of Greater Rage.
        assert_eq!(rage_temporary_hit_points(10, 15), Some(30));
    }

    #[test]
    fn rage_powers_known_is_half_level_from_level_two() {
        assert_eq!(rage_powers_known(1), None);
        assert_eq!(rage_powers_known(2), Some(1));
        assert_eq!(rage_powers_known(3), Some(1));
        assert_eq!(rage_powers_known(4), Some(2));
        assert_eq!(rage_powers_known(20), Some(10));
    }

    #[test]
    fn danger_sense_is_a_third_of_level_from_level_three() {
        assert_eq!(danger_sense_bonus(2), None);
        assert_eq!(danger_sense_bonus(3), Some(1));
        assert_eq!(danger_sense_bonus(5), Some(1));
        assert_eq!(danger_sense_bonus(6), Some(2));
        assert_eq!(danger_sense_bonus(18), Some(6));
        assert_eq!(danger_sense_bonus(20), Some(6));
    }

    /// The published Unchained Barbarian ladder is DR 1/— at 7th, rising by
    /// 1 every three levels to DR 5/— at 19th. Asserted as the whole ladder
    /// rather than a spot check, because `(level-4)/3` is exactly the shape
    /// that looks right and is off by one.
    #[test]
    fn damage_reduction_ladder_is_one_at_seven_rising_every_three_levels() {
        for level in 1..DAMAGE_REDUCTION_LEVEL {
            assert_eq!(damage_reduction(level), None, "level {level}");
        }
        let expected = [
            (7, 1),
            (8, 1),
            (9, 1),
            (10, 2),
            (12, 2),
            (13, 3),
            (15, 3),
            (16, 4),
            (18, 4),
            (19, 5),
            (20, 5),
        ];
        for (level, dr) in expected {
            assert_eq!(damage_reduction(level), Some(dr), "level {level}");
        }
    }

    #[test]
    fn fast_movement_is_a_flat_ten_feet_at_every_level() {
        assert_eq!(fast_movement_bonus_feet(0), None);
        for level in FAST_MOVEMENT_LEVEL..=MAX_SUPPORTED_LEVEL {
            assert_eq!(fast_movement_bonus_feet(level), Some(10), "level {level}");
        }
    }

    #[test]
    fn indomitable_will_is_plus_four_from_level_fourteen() {
        assert_eq!(indomitable_will_save_bonus(13), None);
        assert_eq!(indomitable_will_save_bonus(14), Some(4));
        assert_eq!(indomitable_will_save_bonus(20), Some(4));
    }

    #[test]
    fn uncanny_dodge_tiers_at_two_and_five() {
        assert_eq!(uncanny_dodge_tier(1), 0);
        assert_eq!(uncanny_dodge_tier(2), 1);
        assert_eq!(uncanny_dodge_tier(4), 1);
        assert_eq!(uncanny_dodge_tier(5), 2);
        assert_eq!(uncanny_dodge_tier(20), 2);

        assert_eq!(uncanny_dodge_flanking_level(1), None);
        assert_eq!(uncanny_dodge_flanking_level(2), Some(2));
        assert_eq!(uncanny_dodge_flanking_level(20), Some(20));
    }

    /// Every level-scaling magnitude modelled here must be traceable to a
    /// `BONUS:` token that the ingested corpus record actually carries.
    /// This is the §24 compliance check: transcription, verified.
    #[test]
    fn every_modelled_formula_is_byte_exact_against_the_ingested_corpus_record() {
        assert_bonus_token(
            "Unchained Barbarian ~ Rage Powers",
            "BONUS:ABILITYPOOL|Unchained Rage Power|RagePowersLVL/2",
        );
        assert_bonus_token(
            "Unchained Barbarian ~ Rage Powers",
            "BONUS:VAR|RagePowersLVL|BarbarianLVL",
        );
        assert_bonus_token(
            "Unchained Barbarian ~ Danger Sense",
            "BONUS:VAR|TrapSenseBonus|BarbarianTrapSenseLVL/3",
        );
        assert_bonus_token(
            "Unchained Barbarian ~ Danger Sense",
            "BONUS:VAR|BarbarianTrapSenseLVL|BarbarianLVL",
        );
        assert_bonus_token(
            "Unchained Barbarian ~ Damage Reduction",
            "BONUS:VAR|BarbarianDR|(BarbarianDRLVL-4)/3",
        );
        assert_bonus_token(
            "Unchained Barbarian ~ Damage Reduction",
            "BONUS:VAR|BarbarianDRLVL|BarbarianLVL",
        );
        assert_bonus_token(
            "Unchained Barbarian ~ Fast Movement",
            "BONUS:MOVEADD|TYPE=Walk|10|PREVARLT:ENCUMBERANCE,2,var(\"COUNT[EQTYPE.ARMOR.EQUIPPED.IS.HEAVY]\"),1",
        );
        assert_bonus_token(
            "Unchained Barbarian ~ Rage",
            "BONUS:VAR|RageLVL|BarbarianLVL",
        );
        assert_bonus_token(
            "Unchained Barbarian ~ Uncanny Dodge Tracker",
            "BONUS:VAR|UncannyDodgeLVL|1|PREVARGTEQ:BarbarianLVL,2|PREVAREQ:Barbarian_CF_UncannyDodge,0",
        );
        assert_bonus_token(
            "Unchained Barbarian ~ Uncanny Dodge Tracker",
            "BONUS:VAR|UncannyDodgeLVL|1|PREVARGTEQ:BarbarianLVL,5|PREVAREQ:Barbarian_CF_ImprovedUncannyDodge,0",
        );
        assert_bonus_token(
            "Unchained Barbarian ~ Uncanny Dodge Tracker",
            "BONUS:VAR|UncannyDodgeFlankingLevel|BarbarianLVL|TYPE=EachClass.REPLACE|PREVARGTEQ:BarbarianLVL,2|PREVAREQ:Barbarian_CF_UncannyDodge,0",
        );

        // Indomitable Will has no BONUS token at all -- its magnitude is on
        // the row's ASPECT. Pinned separately, so that "no BONUS token" is a
        // checked fact rather than an omission.
        let indomitable = record_for("Unchained Barbarian ~ Indomitable Will");
        assert!(
            bonus_tokens(&indomitable).is_empty(),
            "Indomitable Will carries no BONUS chain in the corpus"
        );
        let aspect = indomitable["raw_tokens"]
            .as_array()
            .expect("raw_tokens is an array")
            .iter()
            .find(|t| t["key"] == "ASPECT")
            .expect("Indomitable Will must carry an ASPECT token");
        assert_eq!(
            aspect["value"].as_str(),
            Some("SaveBonus|+4 bonus to Will saves vs. Enchantment spells while raging")
        );
    }

    /// The Unchained Barbarian inherits CRB's chassis untouched. Stated as a
    /// test so that adding a chassis table here later is a deliberate act
    /// with a failing test attached, not a quiet second source of truth.
    #[test]
    fn the_variant_overrides_no_chassis_field() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("data/corpus/pathfinder_unchained/class/barbarian_unchained_class.json");
        let text = std::fs::read_to_string(&path).expect("class record must be readable");
        let value: serde_json::Value = serde_json::from_str(&text).expect("valid JSON");
        let data = &value["data"];
        assert_eq!(data["base_class_key"].as_str(), Some("Barbarian"));
        assert_eq!(data["base_class_book"].as_str(), Some("core_rulebook"));
        for field in ["hit_die", "bab", "save_fort", "save_ref", "save_will"] {
            assert!(
                data[field].is_null(),
                "{field} must stay null -- PU overrides no Barbarian chassis field"
            );
        }
    }
}
