//! Tool-side readers for the *typed* ingest cache records' PCGen token array.
//!
//! SD-35 `AT-35-E6-002` cycle 3, under `decisions.md` §11 ("no PCGen in live
//! code") and `technical-design.md` §0 (the boundary is **by path**:
//! `src/pcgen_import/**` may read PCGen, `src/rules_core/**` may not).
//!
//! Sibling of [`ingest_record`](crate::pcgen_import::ingest_record), which
//! reads the same array off an untyped `serde_json::Value`. This module reads
//! it off the `*CacheData` structs `shape_b_v1` deserializes a corpus record
//! into, so a live module never has to name the ingest field to get at a fact
//! the row states.
//!
//! Before this module, `race_resolver.rs` (14 production reads),
//! `race_creation.rs` (1) and `trait_pool.rs` (5) each walked the token array
//! themselves, keyed by the PCGen token name, inside a live module. Every one
//! of those readings is here now, one named function per fact, and the live
//! side calls the function.
//!
//! **This does not make the live modules' dependency on the ingest format
//! disappear** — the same caveat [`ingest_record`]'s header states. It makes
//! it a named cross-boundary call rather than an open-coded traversal, which
//! is what "the boundary is by path" means in practice. Re-pointing these
//! readings at converted `SheetRule` fields instead is the remainder
//! `AT-35-E6-002` still owes, and it is a converter-side change.
//!
//! Every function here is a **reading**, not an interpretation
//! (`decisions.md §24`): it transcribes what one row states and returns
//! `None`/empty where the row states nothing, never a guess. The doc comments
//! that justified each reading moved here with it, verbatim.
//!
//! KEPT for Starfinder, like the rest of `src/pcgen_import/`.

use crate::pcgen_import::ingest_payload::{RaceCacheData, RaceTraitCacheData, RawToken};
use crate::rules_core::size::SizeCategory;

/// A record deserialized from a corpus file that still carries the `.lst` row
/// it was ingested from.
///
/// Implemented here rather than beside the structs so the field read stays on
/// the tool side of `technical-design.md` §0's path boundary.
pub trait IngestTokens {
    /// The row's tokens, in file order.
    fn ingest_tokens(&self) -> &[RawToken];
}

impl IngestTokens for RaceCacheData {
    fn ingest_tokens(&self) -> &[RawToken] {
        &self.raw_tokens
    }
}

impl IngestTokens for RaceTraitCacheData {
    fn ingest_tokens(&self) -> &[RawToken] {
        &self.raw_tokens
    }
}

/// Every same-row variable this record declares, as `(name, base)` in file
/// order, `base` being `None` when the declared base is not a bare integer.
///
/// `DEFINE:<Name>|<base>`. A row that declares no variable contributes
/// nothing. Conditional and cross-row contributions are **not** here: they
/// live in the row's bonus chains, which the caller reads itself.
pub fn same_row_defines<T: IngestTokens>(data: &T) -> Vec<(String, Option<i64>)> {
    data.ingest_tokens()
        .iter()
        .filter(|token| token.key == "DEFINE")
        .filter_map(|token| {
            let (name, base) = token.value.split_once('|')?;
            Some((name.trim().to_string(), base.trim().parse::<i64>().ok()))
        })
        .collect()
}

/// This record's description segments, in file order, exactly as the row
/// states them — the input the prose renderer substitutes its `%N` arguments
/// into.
///
/// Empty when the row states none, which is the caller's signal to fall back
/// to the record's own stored `description` field.
pub fn description_segments<T: IngestTokens>(data: &T) -> Vec<&str> {
    data.ingest_tokens()
        .iter()
        .filter(|token| token.key == "DESC")
        .map(|token| token.value.as_str())
        .collect()
}

/// Every ability key this record grants outright through PCGen's
/// `ABILITY:<category>|AUTOMATIC|<key>[|<key>...]` token.
///
/// Returned verbatim and **unfiltered** — most of these name things that are
/// not racial traits at all (`ABILITY:FEAT|AUTOMATIC|Endurance`,
/// `ABILITY:Class Skill|AUTOMATIC|Survival`, `ABILITY:Spell-Like
/// Ability|AUTOMATIC|Racial SLA ~ Invisibility`), and deciding which of them
/// resolve to a loaded race-trait record is the caller's job, not this
/// accessor's. Returning only the resolvable ones would hide the rest, and
/// "we found content we cannot place" is a fact this reading deliberately
/// keeps visible.
pub fn automatic_ability_grants<T: IngestTokens>(data: &T) -> Vec<String> {
    data.ingest_tokens()
        .iter()
        .filter(|token| token.key == "ABILITY")
        .flat_map(|token| automatic_grant_targets(&token.value))
        .collect()
}

/// The pool suffix of the first `CHOOSE:` token whose payload starts with
/// `prefix`, trimmed of leading whitespace — `None` when the row carries no
/// such selector.
pub fn choice_pool_suffix<T: IngestTokens>(data: &T, prefix: &str) -> Option<String> {
    data.ingest_tokens()
        .iter()
        .find(|t| t.key == "CHOOSE" && t.value.trim_start().starts_with(prefix))
        .map(|t| t.value.trim_start()[prefix.len()..].to_string())
}

/// The ability flag a *positive* `PREFACT:1,ABILITIES,<Flag>=True` gate on
/// this row names, if it carries one.
pub fn positive_prefact_flag<T: IngestTokens>(data: &T) -> Option<String> {
    data.ingest_tokens()
        .iter()
        .filter(|t| t.key == "PREFACT")
        .find_map(|t| first_ability_flag(&t.value))
}

/// This record's `MOVE:Walk,N` in feet, if it declares one.
pub fn declared_walk_speed_ft<T: IngestTokens>(data: &T) -> Option<i32> {
    data.ingest_tokens()
        .iter()
        .filter(|t| t.key == "MOVE")
        .find_map(|t| walk_speed_from_move(&t.value))
}

/// The creature size this record's `TEMPLATE:SIZE_<code>` assigns, if it
/// carries one.
///
/// This is transcription, not interpretation (`decisions.md §24`): PCGen's
/// `SIZE_*` templates are defined in
/// `core_essentials/ce_templates.lst:924-933` and each one's entire body
/// *is* a size assignment — `SIZE_S  SIZE:S  VISIBLE:NO`,
/// `SIZE_M  SIZE:M  VISIBLE:NO`, and so on for `F D T S M L H G C`, using the
/// same single-letter code set as `FACT:BaseSize`. Reading `SIZE_M` off the
/// row that declares it is reading a constant off the row that defines it.
///
/// `SIZE_C+` (which maps to the non-`SizeCategory` code `P`) and any other
/// unrecognized suffix yield `None` rather than a guess.
pub fn declared_size<T: IngestTokens>(data: &T) -> Option<SizeCategory> {
    data.ingest_tokens()
        .iter()
        .filter(|t| t.key == "TEMPLATE")
        .find_map(|t| size_from_size_template(&t.value))
}

/// Every sense this record's `VISION:` tokens declare, one entry per segment,
/// trimmed, in file order.
///
/// PCGen states more than one sense two different ways: as separate
/// `VISION:`-keyed fields on the same row (Svirfneblin's
/// `VISION:Darkvision (120) VISION:Low-Light Vision`) or as one field with a
/// `|`-joined tail (Dhampir's `VISION:Darkvision (60)|Low-Light Vision`,
/// SD-32 card-11 T2b lane, 2026-08-23). Both are the same fact stated two
/// ways, so both are flattened to the same segment list here rather than only
/// the first shape being read. The segments are returned **verbatim** —
/// rendering `Darkvision (60)` as `Darkvision 60 ft.` is the sheet's job, not
/// this reading's.
pub fn declared_vision_segments<T: IngestTokens>(data: &T) -> Vec<String> {
    data.ingest_tokens()
        .iter()
        .filter(|t| t.key == "VISION")
        .flat_map(|t| t.value.split('|'))
        .map(|segment| segment.trim().to_string())
        .filter(|segment| !segment.is_empty())
        .collect()
}

/// A row's own `TEMPLATE:Bonus Language ~ <Lang>|...` chain, transcribed
/// verbatim into the language name(s) it names — **reading, not applying**,
/// the same claim [`declared_size`] makes for `SIZE_<code>`.
///
/// Verified directly against the real corpus, not assumed: every
/// `Bonus Language ~ <Lang>` row PCGen defines is a single-purpose template
/// whose entire body is one `LANGBONUS:<Lang>` token —
/// `data/corpus/core_rulebook/template/bonus_language_common.json`'s own
/// ingest tokens are exactly `[VISIBLE:NO, LANGBONUS:Common]`, and the same
/// shape holds for every sibling this function is used against
/// (`bonus_language_giant.json`, `_goblin.json`, `_halfling.json`). Reading
/// `"Bonus Language ~ Common"` off a `TEMPLATE:` chain and returning
/// `"Common"` is therefore transcription of a real, verified 1:1 name
/// mapping, not an interpretation.
///
/// A `TEMPLATE:` value that does not carry the `"Bonus Language ~ "` prefix
/// contributes nothing. One real corpus row's own chain, `Human ~ Languages`'
/// `TEMPLATE:Bonus Language ~ Any Spoken`, DOES match the prefix and yields
/// the literal marker `"Any Spoken"` — that is PCGen's own "no restriction"
/// template, not a real language, and callers that care about the difference
/// must check for it; this function only transcribes, it does not classify
/// what it reads.
pub fn declared_template_bonus_languages(tokens: &[RawToken]) -> Vec<String> {
    tokens
        .iter()
        .filter(|t| t.key == "TEMPLATE")
        .flat_map(|t| t.value.split('|'))
        .filter_map(|part| part.trim().strip_prefix("Bonus Language ~ ").map(str::to_string))
        .collect()
}

/// `ABILITY:<category>|AUTOMATIC|<key>[|<key>...]` -> the keys it grants.
///
/// Anything after the first `PRE`/`!PRE` qualifier is a gate, not a grant, so
/// the walk stops there. `%LIST` is skipped: it is PCGen's "whatever the
/// player chose" placeholder and names no concrete record.
fn automatic_grant_targets(value: &str) -> Vec<String> {
    let mut parts = value.split('|');
    let _category = parts.next();
    if !parts.next().is_some_and(|nature| nature.eq_ignore_ascii_case("AUTOMATIC")) {
        return Vec::new();
    }
    let mut out = Vec::new();
    for part in parts {
        let part = part.trim();
        if part.starts_with("PRE") || part.starts_with("!PRE") {
            break;
        }
        if part.is_empty() || part == "%LIST" {
            continue;
        }
        out.push(part.to_string());
    }
    out
}

/// `1,ABILITIES,Dwarf_ReplaceGreed=True` -> `Dwarf_ReplaceGreed`.
fn first_ability_flag(value: &str) -> Option<String> {
    let mut parts = value.split(',');
    if parts.next()? != "1" {
        return None;
    }
    if !parts.next()?.eq_ignore_ascii_case("ABILITIES") {
        return None;
    }
    let clause = parts.next()?;
    let (flag, _) = clause.split_once('=')?;
    Some(flag.to_string())
}

/// `SIZE_M` -> [`SizeCategory::Medium`]. Any `TEMPLATE:` payload that is not
/// one of `ce_templates.lst`'s nine `SIZE_<code>` rows yields `None` — a race
/// trait carries plenty of other templates, and `SIZE_C+` (whose body is
/// `SIZE:P`, a code `SizeCategory` does not model) must not be mistaken for
/// Colossal.
fn size_from_size_template(value: &str) -> Option<SizeCategory> {
    let code = value.trim().strip_prefix("SIZE_")?;
    if code.len() != 1 {
        return None;
    }
    SizeCategory::from_base_size_code(code)
}

/// `Walk,20` / `Walk,15,Swim,30` -> `20` / `15`. `None` when the token names
/// no walk movement at all.
fn walk_speed_from_move(value: &str) -> Option<i32> {
    let parts: Vec<&str> = value.split(',').collect();
    parts
        .windows(2)
        .find(|pair| pair[0].trim().eq_ignore_ascii_case("Walk"))
        .and_then(|pair| pair[1].trim().parse::<i32>().ok())
}

#[cfg(test)]
mod declared_template_bonus_languages_tests {
    use super::*;

    fn token(key: &str, value: &str) -> RawToken {
        RawToken { key: key.to_string(), value: value.to_string() }
    }

    /// `isr_abilities_race.lst:216`'s own `TEMPLATE:` value, copied verbatim
    /// from `data/corpus/inner_sea_races/race_trait/human/
    /// human_tribalistic_languages.json` — real corpus content, not a
    /// fabricated example.
    #[test]
    fn tribalistic_languages_template_chain_transcribes_to_four_real_languages() {
        let raw = vec![token(
            "TEMPLATE",
            "Bonus Language ~ Common|Bonus Language ~ Giant|Bonus Language ~ Goblin|Bonus Language ~ Halfling",
        )];
        assert_eq!(
            declared_template_bonus_languages(&raw),
            vec![
                "Common".to_string(),
                "Giant".to_string(),
                "Goblin".to_string(),
                "Halfling".to_string(),
            ]
        );
    }

    /// `core_rulebook`'s standard `Human ~ Languages` row (`suppressed_by_flag:
    /// Human_ReplaceLanguages`) carries the "no restriction" marker template,
    /// not a real language — transcribed, not filtered, so a caller sees the
    /// literal name and can decide what it means.
    #[test]
    fn any_spoken_marker_transcribes_literally_not_as_a_language_name() {
        let raw = vec![token("TEMPLATE", "Bonus Language ~ Any Spoken")];
        assert_eq!(declared_template_bonus_languages(&raw), vec!["Any Spoken".to_string()]);
    }

    /// A `TEMPLATE:` chain naming something other than a `Bonus Language ~`
    /// row (e.g. a `SIZE_<code>` row) contributes nothing here — the two
    /// readers are deliberately independent.
    #[test]
    fn non_bonus_language_template_yields_nothing() {
        let raw = vec![token("TEMPLATE", "SIZE_M")];
        assert!(declared_template_bonus_languages(&raw).is_empty());
    }

    /// No `TEMPLATE:` token at all yields nothing, not a guess.
    #[test]
    fn no_template_token_yields_nothing() {
        let raw = vec![token("DESC", "irrelevant")];
        assert!(declared_template_bonus_languages(&raw).is_empty());
    }
}

#[cfg(test)]
mod reader_tests {
    use super::*;

    fn token(key: &str, value: &str) -> RawToken {
        RawToken { key: key.to_string(), value: value.to_string() }
    }

    /// A bare token carrier, so the readers are provable without constructing
    /// a whole `RaceTraitCacheData`.
    struct Row(Vec<RawToken>);
    impl IngestTokens for Row {
        fn ingest_tokens(&self) -> &[RawToken] {
            &self.0
        }
    }

    #[test]
    fn defines_read_name_and_integer_base_and_report_an_unresolvable_base_as_none() {
        let row = Row(vec![
            token("DEFINE", "Dwarf_StoneCunning_SkillBonus|2"),
            token("DEFINE", "Elf_Something|OtherVar"),
            token("DESC", "not a define"),
        ]);
        assert_eq!(
            same_row_defines(&row),
            vec![
                ("Dwarf_StoneCunning_SkillBonus".to_string(), Some(2)),
                ("Elf_Something".to_string(), None),
            ]
        );
    }

    #[test]
    fn both_vision_shapes_flatten_to_the_same_segment_list() {
        let separate =
            Row(vec![token("VISION", "Darkvision (120)"), token("VISION", "Low-Light Vision")]);
        let joined = Row(vec![token("VISION", "Darkvision (120)|Low-Light Vision")]);
        let expected = vec!["Darkvision (120)".to_string(), "Low-Light Vision".to_string()];
        assert_eq!(declared_vision_segments(&separate), expected);
        assert_eq!(declared_vision_segments(&joined), expected);
    }

    #[test]
    fn a_row_with_no_vision_token_declares_no_sense() {
        assert!(declared_vision_segments(&Row(vec![token("DESC", "x")])).is_empty());
    }

    #[test]
    fn walk_speed_and_size_read_their_own_token_and_nothing_else() {
        let row = Row(vec![token("MOVE", "Walk,15,Swim,30"), token("TEMPLATE", "SIZE_S")]);
        assert_eq!(declared_walk_speed_ft(&row), Some(15));
        assert_eq!(declared_size(&row), Some(SizeCategory::Small));
        let neither = Row(vec![token("MOVE", "Swim,30"), token("TEMPLATE", "SIZE_C+")]);
        assert_eq!(declared_walk_speed_ft(&neither), None, "no walk movement is None, not 0");
        assert_eq!(declared_size(&neither), None, "an unmodelled size code is None, not a guess");
    }

    #[test]
    fn automatic_grants_stop_at_the_first_gate_and_skip_the_choice_placeholder() {
        let row = Row(vec![
            token("ABILITY", "Orc Racial Trait|AUTOMATIC|Feral ~ Languages|PREFACT:1,ABILITIES,X=True"),
            token("ABILITY", "FEAT|AUTOMATIC|%LIST"),
            token("ABILITY", "FEAT|VIRTUAL|Endurance"),
        ]);
        assert_eq!(automatic_ability_grants(&row), vec!["Feral ~ Languages".to_string()]);
    }

    #[test]
    fn a_positive_gate_names_its_flag_and_a_negated_one_is_not_a_token_this_reads() {
        let row = Row(vec![token("PREFACT", "1,ABILITIES,Dwarf_ReplaceGreed=True")]);
        assert_eq!(positive_prefact_flag(&row), Some("Dwarf_ReplaceGreed".to_string()));
        assert_eq!(positive_prefact_flag(&Row(vec![token("PREFACT", "1,SOMETHINGELSE,X=True")])), None);
    }

    #[test]
    fn a_choice_selector_yields_the_pool_suffix_after_its_prefix() {
        let row = Row(vec![token("CHOOSE", "ABILITYSELECTION|Trait|TYPE=Dwarf Race Trait")]);
        assert_eq!(
            choice_pool_suffix(&row, "ABILITYSELECTION|Trait|TYPE="),
            Some("Dwarf Race Trait".to_string())
        );
        assert_eq!(choice_pool_suffix(&row, "SOMETHING ELSE|"), None);
    }

    #[test]
    fn description_segments_preserve_file_order_and_are_empty_when_the_row_states_none() {
        let row = Row(vec![token("DESC", "first"), token("MOVE", "Walk,20"), token("DESC", "second")]);
        assert_eq!(description_segments(&row), vec!["first", "second"]);
        assert!(description_segments(&Row(vec![token("MOVE", "Walk,20")])).is_empty());
    }
}

#[cfg(test)]
mod moved_from_race_resolver_tests {
    //! The unit tests of the four private token parsers, moved here verbatim
    //! with the functions they cover (SD-35 `AT-35-E6-002` cycle 3). Same
    //! assertions, same real token forms; only their home changed.
    use super::*;

    /// `TEMPLATE:` is a busy token; only the nine `SIZE_<code>` rows of
    /// `ce_templates.lst` may be read as a size, and `SIZE_C+` (body
    /// `SIZE:P`) is not one of them.
    #[test]
    fn only_a_real_size_template_token_is_read_as_a_size() {
        assert_eq!(size_from_size_template("SIZE_M"), Some(SizeCategory::Medium));
        assert_eq!(size_from_size_template("SIZE_S"), Some(SizeCategory::Small));
        assert_eq!(size_from_size_template("SIZE_C"), Some(SizeCategory::Colossal));
        assert_eq!(size_from_size_template("SIZE_C+"), None, "its body is SIZE:P, not a modelled code");
        assert_eq!(size_from_size_template("Dragon Size Tracker"), None);
        assert_eq!(size_from_size_template("SIZE_"), None);
        assert_eq!(size_from_size_template("Half-Orc Language Template"), None);
        assert_eq!(size_from_size_template(""), None);
    }

    #[test]
    fn move_and_prefact_token_parsing_reads_the_real_token_forms() {
        assert_eq!(walk_speed_from_move("Walk,20"), Some(20));
        assert_eq!(walk_speed_from_move("Walk,15,Swim,30"), Some(15));
        assert_eq!(walk_speed_from_move("Walk,0"), Some(0));
        assert_eq!(walk_speed_from_move("Swim,50"), None, "no walk component");
        assert_eq!(first_ability_flag("1,ABILITIES,Dwarf_ReplaceGreed=True"), Some("Dwarf_ReplaceGreed".into()));
        assert_eq!(first_ability_flag("1,ABILITIES,Dwarf_ReplaceGreed=true"), Some("Dwarf_ReplaceGreed".into()));
        assert_eq!(first_ability_flag("1,SOMETHINGELSE,X=True"), None);
        assert_eq!(first_ability_flag("garbage"), None);
    }
}
