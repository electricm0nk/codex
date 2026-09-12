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

/// The literal `CHOOSE:` payload prefix an Adopted-Race selector row's pool
/// token carries; the pool's `<X> Race Trait` suffix follows it verbatim.
///
/// SD-35 `AT-35-E6-003-SWEEP` cycle 14: moved here from
/// `rules_core::race_resolver`, which held it as a live `pub const` and passed
/// it back into [`choice_pool_suffix`] — a live module carrying the ingest
/// format's vocabulary, which `decisions.md` §11 forbids and
/// `technical-design.md` §0 places on this side of the path boundary. Still
/// public because `src/bin/ingest_race_traits.rs` matches on it too, and one
/// literal read by both is the point; `src/bin/**` is tool side.
pub const ADOPTED_RACE_SELECTOR_CHOOSE_PREFIX: &str = "ABILITYSELECTION|Special Ability|TYPE=";

/// The pool suffix of this row's Adopted-Race selector (`decisions.md` §25) —
/// e.g. `"Oread Race Trait"` — or `None` when the row carries no such selector.
///
/// [`choice_pool_suffix`] specialized to the one prefix the live side used to
/// supply itself. The caller now asks *which trait pool does this selector
/// adopt from?* and never names a `CHOOSE:` payload shape.
pub fn adopted_race_pool_suffix<T: IngestTokens>(data: &T) -> Option<String> {
    choice_pool_suffix(data, ADOPTED_RACE_SELECTOR_CHOOSE_PREFIX)
}

/// The `TYPE=` pool prefix every Skinwalker kin master record's own `ABILITY:`
/// grant carries ahead of the kin name.
///
/// SD-35 `AT-35-E6-003-SWEEP` cycle 14: moved here from
/// `rules_core::skinwalker_change_shape`, for the same reason and under the
/// same rules as [`ADOPTED_RACE_SELECTOR_CHOOSE_PREFIX`].
const SKINWALKER_CHANGE_SHAPE_POOL_PREFIX: &str = "TYPE=Skinwalker Change Shape ";

/// The Skinwalker kin this automatic grant names its Change Shape pool for —
/// `"Werebear-Kin"`, `"Default"` — or `None` when the grant is not a Change
/// Shape pool grant at all.
///
/// Takes one grant string as [`automatic_ability_grants`] returns it, so the
/// caller keeps its own choice of which grant on the row to ask about. A
/// **reading**, not an interpretation (`decisions.md` §24): the kin name is
/// handed back exactly as the row spells it, with no mapping, normalization or
/// validation against any kin list — the caller owns that, and does own it
/// (`skinwalker_change_shape::KIN_OPTION_KEYS`, whose `Default` row is
/// deliberately absent).
pub fn skinwalker_change_shape_kin(grant: &str) -> Option<&str> {
    grant.strip_prefix(SKINWALKER_CHANGE_SHAPE_POOL_PREFIX)
}

/// The ability flag a *positive* `PREFACT:1,ABILITIES,<Flag>=True` gate on
/// this row names, if it carries one.
pub fn positive_prefact_flag<T: IngestTokens>(data: &T) -> Option<String> {
    data.ingest_tokens()
        .iter()
        .filter(|t| t.key == "PREFACT")
        .find_map(|t| first_ability_flag(&t.value))
}

/// Every `<X>_Replace<Y>` flag whose being already set blocks a *new*
/// selection of this alternate racial trait, in source order, deduplicated.
///
/// SD-35 `AT-35-E6-003-SWEEP` cycle 15: moved here verbatim from
/// `apps/desktop/src-tauri/src/race_trait_picker.rs`, which was the last
/// `apps/` file the residue gate listed. The picker asked *which flags exclude
/// this trait?*; to answer it, it had to walk `raw_tokens` keyed on four PCGen
/// token names and strip a `PREVAREQ:` prefix by hand. The rules question is
/// the picker's; the grammar that answers it is this side's
/// (`decisions.md` §11, `technical-design.md` §0).
///
/// **This corpus states one relation in four spellings.** Each is read, and
/// none is inferred:
///
/// 1. **`PREMULT` with a negated branch** — the ARG shape, from
///    `arg_abilities_race.lst:38`:
///
///    ```text
///    PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Magic Resistant],
///              [!PREFACT:1,ABILITIES,Dwarf_ReplaceHardy=true]
///    ```
///
///    Read: satisfied if you already have this ability **or**
///    `Dwarf_ReplaceHardy` is not set. The first branch is PCGen's way of
///    letting an ability satisfy its own prerequisite once granted; the
///    operative constraint for a *new* selection is the second. Only bracket
///    groups beginning `!` are read, and within them only clauses whose
///    left-hand side contains `_Replace` — so `CATEGORY=Special Ability` and
///    the ability key in the positive branch contribute nothing.
/// 2. **`!PREABILITY` instead of `!PREFACT` in that negated branch** — three
///    ARG rows (`Half-Elf ~ Wary`, `~ Drow-Blooded`, `~ Drow Magic`) write it
///    that way. An upstream token slip, since the operand is unmistakably a
///    fact flag and is the very flag each row sets. Matching on the negation
///    plus the `_Replace` operand rather than on the token name reads all
///    three correctly without inventing anything;
///    [`declares_preability_negated_guard`] is how the caller reports the slip.
/// 3. **`PREVAREQ:<flag>,0` on the record's own `ABILITY:...|AUTOMATIC|<key>`
///    grant** — `core_essentials`' heritage selectors (SD-29 race-trait lane
///    round 4, `decisions.md §49`) carry no `PREMULT` at all: upstream, only
///    one heritage can apply because a heritage is a PCGen SUBRACE and a
///    character has one. Read through the `PREMULT` branch alone all 16 would
///    come back unguarded, and a player could tick `Aasimar ~ Angel-Blooded`
///    and `Aasimar ~ Archon-Blooded` together and collect both ability-score
///    bonuses. The corpus does state the constraint, on the grant itself:
///    `ABILITY:Aasimar Racial Trait|AUTOMATIC|Angel-Blooded ~ Ability Scores|PREVAREQ:Aasimar_ReplaceAbilityScores,0`
///    reads *grant this while that standard trait has not already been
///    replaced*, which is the same "already set by someone else blocks me"
///    relation. Only `,0` is read, for `ingest_races::globalvar_gates`' stated
///    reason: `,1` is the opposite statement.
/// 4. **A positive `PREABILITY` parent dependency plus the row's own
///    `sets_replace_flags`** — SD-33 Epic 6's Skinwalker fold (2026-08-26). A
///    record with a positive `PREABILITY:1,CATEGORY=Special Ability,<parent
///    key>` dependency (not a negated `!PREABILITY` bracket; that shape is
///    already read above) **and** a non-empty `sets_replace_flags` is a
///    heritage REPLACEMENT row this corpus never gives a `PREMULT`/`PREVAREQ`
///    guard of its own to (Skinwalker's 36 `<Kin> ~ <Trait>` rows: PCGen gates
///    them on their PARENT selector's `PREABILITY`/`PREMULT` alone, on the
///    assumption a player reaches them only by picking that one selector
///    first). Without this branch none of the 36 carried ANY exclusion guard,
///    and a player could tick `Werebat-Kin ~ Ability Scores` AND
///    `Werebear-Kin ~ Ability Scores` together — both fire
///    `Skinwalker_ReplaceAbilityScores` — and collect both incompatible
///    ability-score swaps, since nothing suppressed the second. The guard used
///    is the record's OWN already-honest `sets_replace_flags`, read off its
///    real `FACT:<flag>|True` token, not a fabricated one. Monster Codex's
///    `Oversized Goblin ~ Ability Scores` / `~ Size` are this branch's negative
///    control: they carry no `PREABILITY` at all, never reach it, and stay
///    unguarded.
///
/// Branch 4 fires only when the first three found nothing, which is the
/// ordering the picker applied and is load-bearing: it is a fallback for rows
/// the corpus guards nowhere else, never an addition to a row already guarded.
pub fn exclusion_guard_flags(data: &RaceTraitCacheData) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for token in data.raw_tokens.iter().filter(|token| token.key == "ABILITY") {
        let parts: Vec<&str> = token.value.split('|').collect();
        if parts.len() < 2 || !parts[1].trim().eq_ignore_ascii_case("AUTOMATIC") {
            continue;
        }
        for clause in &parts[2..] {
            let Some(rest) = clause.trim().strip_prefix("PREVAREQ:") else { continue };
            let Some((flag, want)) = rest.rsplit_once(',') else { continue };
            let flag = flag.trim();
            if want.trim() != "0" || !flag.contains("_Replace") {
                continue;
            }
            if !out.iter().any(|existing| existing == flag) {
                out.push(flag.to_string());
            }
        }
    }
    for token in data.raw_tokens.iter().filter(|token| token.key == "PREMULT") {
        for group in negated_bracket_groups(&token.value) {
            for clause in group.split(',') {
                let Some((name, value)) = clause.split_once('=') else { continue };
                let name = name.trim();
                if !name.contains("_Replace") || !value.trim().eq_ignore_ascii_case("true") {
                    continue;
                }
                if !out.iter().any(|existing| existing == name) {
                    out.push(name.to_string());
                }
            }
        }
    }
    if out.is_empty()
        && !data.sets_replace_flags.is_empty()
        && data.raw_tokens.iter().any(|token| token.key == "PREABILITY")
    {
        for flag in &data.sets_replace_flags {
            if !out.iter().any(|existing| existing == flag) {
                out.push(flag.clone());
            }
        }
    }
    out
}

/// This record's *negated fact* suppression gates, one entry per gate, each
/// entry the flags that gate names in the order the row writes them.
///
/// A standard racial trait declares the flag whose presence suppresses it. The
/// typed cache field that carries it,
/// [`RaceTraitCacheData::suppressed_by_flag`], is single-valued, so a *single
/// gate* naming more than one flag has a trailing flag the resolver never acts
/// on. The caller reports that as a distinct upstream finding; returning the
/// flags grouped **by gate** rather than flattened is what lets it stay exact —
/// two separate one-flag gates are not the same statement as one two-flag gate,
/// and flattening would make them indistinguishable.
///
/// A gate whose leading count is not `1`, or whose subject is not `ABILITIES`,
/// yields an empty entry: those are different statements, and this transcribes
/// rather than generalizes.
pub fn negated_fact_gates<T: IngestTokens>(data: &T) -> Vec<Vec<String>> {
    data.ingest_tokens()
        .iter()
        .filter(|token| token.key == "!PREFACT")
        .map(|token| negated_prefact_flags(&token.value))
        .collect()
}

/// Whether this record writes its self-exclusion guard's negated branch as
/// `!PREABILITY` rather than `!PREFACT`.
///
/// The upstream token slip described in [`exclusion_guard_flags`]' spelling 2.
/// [`exclusion_guard_flags`] reads such a row correctly regardless; this is the
/// separate question *did we have to?*, which the picker surfaces to the
/// player as a findings row so a corpus defect is reported rather than
/// silently absorbed.
pub fn declares_preability_negated_guard<T: IngestTokens>(data: &T) -> bool {
    data.ingest_tokens()
        .iter()
        .filter(|token| token.key == "PREMULT")
        .any(|token| negated_bracket_groups(&token.value).iter().any(|g| g.starts_with("PREABILITY:")))
}

/// The contents of every `[...]` group in a `PREMULT` value whose first
/// character is `!`, with that `!` stripped. Nesting does not occur in this
/// token family, so a flat scan is exact rather than approximate.
fn negated_bracket_groups(value: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let bytes = value.as_bytes();
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] != b'[' {
            index += 1;
            continue;
        }
        let start = index + 1;
        let Some(offset) = value[start..].find(']') else { break };
        let group = &value[start..start + offset];
        if let Some(rest) = group.strip_prefix('!') {
            out.push(rest);
        }
        index = start + offset + 1;
    }
    out
}

/// `1,ABILITIES,A=True,B=True` → `["A", "B"]`.
fn negated_prefact_flags(value: &str) -> Vec<String> {
    let mut parts = value.split(',');
    if parts.next() != Some("1") {
        return Vec::new();
    }
    match parts.next() {
        Some(word) if word.eq_ignore_ascii_case("ABILITIES") => {}
        _ => return Vec::new(),
    }
    parts.filter_map(|clause| clause.split_once('=').map(|(flag, _)| flag.trim().to_string())).collect()
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

#[cfg(test)]
mod moved_from_race_trait_picker_tests {
    //! The corpus-wide round trip for the exclusion-guard reading moved here
    //! from `apps/desktop/src-tauri/src/race_trait_picker.rs` in SD-35
    //! `AT-35-E6-003-SWEEP` cycle 15.
    //!
    //! The "before" side of every comparison below is the picker's code as it
    //! stood at `94b4db5306`, transcribed inline rather than referenced. That
    //! is deliberate and is the same bar `equipment_bonus_reader` set in cycle
    //! 14: a round trip proved against a *paraphrase* of the old predicate
    //! proves nothing about the move.
    //!
    //! It reads `data/corpus/**` — the live corpus directory — not a fixture
    //! with a hand-derived value, so the populations below are the shipped
    //! records and a new book changes them without anyone editing this file.
    use super::*;
    use crate::pcgen_import::ingest_payload::RaceTraitCacheData;
    use std::path::{Path, PathBuf};

    fn repo() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    /// Every `race_trait` record in the shipped corpus, as `(relative path,
    /// payload)`.
    fn corpus_race_traits() -> Vec<(String, RaceTraitCacheData)> {
        fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
            let Ok(entries) = std::fs::read_dir(dir) else { return };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk(&path, out);
                } else if path.extension().is_some_and(|e| e == "json") {
                    out.push(path);
                }
            }
        }

        let corpus = repo().join("data/corpus");
        let mut books: Vec<PathBuf> = std::fs::read_dir(&corpus)
            .expect("data/corpus must exist")
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .collect();
        books.sort();

        let mut out = Vec::new();
        for book in books {
            let kind = book.join("race_trait");
            if !kind.is_dir() {
                continue;
            }
            let mut files = Vec::new();
            walk(&kind, &mut files);
            files.sort();
            for file in files {
                let Ok(text) = std::fs::read_to_string(&file) else { continue };
                let Ok(doc) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
                let Some(data) = doc.get("data") else { continue };
                let Ok(payload) = serde_json::from_value::<RaceTraitCacheData>(data.clone()) else { continue };
                let rel = file.strip_prefix(repo()).unwrap_or(&file).display().to_string();
                out.push((rel, payload));
            }
        }
        out
    }

    /// The picker's `exclusion_guard_flags` body, verbatim at `94b4db5306`,
    /// with `record.data` rewritten as `data` — the only edit, and a
    /// mechanical one.
    fn before_exclusion_guard_flags(data: &RaceTraitCacheData) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for token in data.raw_tokens.iter().filter(|token| token.key == "ABILITY") {
            let parts: Vec<&str> = token.value.split('|').collect();
            if parts.len() < 2 || !parts[1].trim().eq_ignore_ascii_case("AUTOMATIC") {
                continue;
            }
            for clause in &parts[2..] {
                let Some(rest) = clause.trim().strip_prefix("PREVAREQ:") else { continue };
                let Some((flag, want)) = rest.rsplit_once(',') else { continue };
                let flag = flag.trim();
                if want.trim() != "0" || !flag.contains("_Replace") {
                    continue;
                }
                if !out.iter().any(|existing| existing == flag) {
                    out.push(flag.to_string());
                }
            }
        }
        for token in data.raw_tokens.iter().filter(|token| token.key == "PREMULT") {
            for group in before_negated_bracket_groups(&token.value) {
                for clause in group.split(',') {
                    let Some((name, value)) = clause.split_once('=') else { continue };
                    let name = name.trim();
                    if !name.contains("_Replace") || !value.trim().eq_ignore_ascii_case("true") {
                        continue;
                    }
                    if !out.iter().any(|existing| existing == name) {
                        out.push(name.to_string());
                    }
                }
            }
        }
        if out.is_empty()
            && !data.sets_replace_flags.is_empty()
            && data.raw_tokens.iter().any(|token| token.key == "PREABILITY")
        {
            for flag in &data.sets_replace_flags {
                if !out.iter().any(|existing| existing == flag) {
                    out.push(flag.clone());
                }
            }
        }
        out
    }

    /// The picker's `negated_bracket_groups`, verbatim at `94b4db5306`.
    fn before_negated_bracket_groups(value: &str) -> Vec<&str> {
        let mut out = Vec::new();
        let bytes = value.as_bytes();
        let mut index = 0usize;
        while index < bytes.len() {
            if bytes[index] != b'[' {
                index += 1;
                continue;
            }
            let start = index + 1;
            let Some(offset) = value[start..].find(']') else { break };
            let group = &value[start..start + offset];
            if let Some(rest) = group.strip_prefix('!') {
                out.push(rest);
            }
            index = start + offset + 1;
        }
        out
    }

    /// The picker's `negated_prefact_flags`, verbatim at `94b4db5306`.
    fn before_negated_prefact_flags(value: &str) -> Vec<String> {
        let mut parts = value.split(',');
        if parts.next() != Some("1") {
            return Vec::new();
        }
        match parts.next() {
            Some(word) if word.eq_ignore_ascii_case("ABILITIES") => {}
            _ => return Vec::new(),
        }
        parts.filter_map(|clause| clause.split_once('=').map(|(flag, _)| flag.trim().to_string())).collect()
    }

    /// The whole point of the move: every one of the three readings must be
    /// **identical** to the code it replaced, on every real record, not merely
    /// on the ones somebody thought to write down.
    #[test]
    fn the_exclusion_guard_readings_are_unchanged_by_this_module() {
        let mut disagreements: Vec<String> = Vec::new();
        let mut records = 0usize;
        let mut guarded = 0usize;
        let mut preability_spelled = 0usize;
        let mut multi_flag_gates = 0usize;

        for (rel, data) in corpus_race_traits() {
            records += 1;

            let before = before_exclusion_guard_flags(&data);
            let after = exclusion_guard_flags(&data);
            if before != after {
                disagreements.push(format!("{rel}: guard flags {before:?} -> {after:?}"));
            }
            if !after.is_empty() {
                guarded += 1;
            }

            // Verbatim `preability_guard_findings`' inner predicate.
            let before_preability = data
                .raw_tokens
                .iter()
                .filter(|token| token.key == "PREMULT")
                .any(|token| before_negated_bracket_groups(&token.value).iter().any(|g| g.starts_with("PREABILITY:")));
            let after_preability = declares_preability_negated_guard(&data);
            if before_preability != after_preability {
                disagreements.push(format!("{rel}: !PREABILITY spelling {before_preability} -> {after_preability}"));
            }
            if after_preability {
                preability_spelled += 1;
            }

            // Verbatim `multi_flag_gate_findings`' inner loop, which is
            // per-gate and must stay per-gate.
            let before_gates: Vec<Vec<String>> = data
                .raw_tokens
                .iter()
                .filter(|token| token.key == "!PREFACT")
                .map(|token| before_negated_prefact_flags(&token.value))
                .collect();
            let after_gates = negated_fact_gates(&data);
            if before_gates != after_gates {
                disagreements.push(format!("{rel}: !PREFACT gates {before_gates:?} -> {after_gates:?}"));
            }
            multi_flag_gates += after_gates.iter().filter(|flags| flags.len() > 1).count();
        }

        assert!(
            disagreements.is_empty(),
            "{} corpus race-trait record(s) read differently after the move:\n{}",
            disagreements.len(),
            disagreements.join("\n")
        );

        // A walk that silently stopped finding records would agree with itself
        // about nothing, so every population this gate rests on must be
        // non-empty. The figures are printed, never asserted as constants:
        // ingesting a book moves them and this file must not have to change.
        assert!(records > 0, "the corpus walk found no race_trait records at all");
        assert!(guarded > 0, "no record came back with an exclusion guard -- the reading is inert");
        assert!(preability_spelled > 0, "the !PREABILITY spelling branch was never exercised");
        assert!(multi_flag_gates > 0, "the multi-flag gate branch was never exercised");
        println!(
            "records={records} guarded={guarded} preability_spelled={preability_spelled} \
             multi_flag_gates={multi_flag_gates}"
        );
    }

    /// The three deliberate narrownesses, each of which a tidy-up would widen
    /// and each of which changes which corpus records match.
    #[test]
    fn the_readings_keep_their_deliberate_narrowness() {
        let row = |tokens: Vec<(&str, &str)>, sets: Vec<&str>| RaceTraitCacheData {
            key: "X ~ Y".into(),
            name: "Y".into(),
            race_key: "X".into(),
            category: None,
            type_tokens: Vec::new(),
            is_racial_default: false,
            suppressed_by_flag: None,
            sets_replace_flags: sets.into_iter().map(str::to_owned).collect(),
            description: None,
            source_page: None,
            raw_tokens: tokens
                .into_iter()
                .map(|(k, v)| RawToken { key: k.into(), value: v.into() })
                .collect(),
            raw_bonus_chains: Vec::new(),
        };

        // `PREVAREQ:<flag>,1` is the opposite statement to `,0` and is not a
        // guard.
        let zero = row(vec![("ABILITY", "X Racial Trait|AUTOMATIC|Y|PREVAREQ:X_ReplaceY,0")], vec![]);
        let one = row(vec![("ABILITY", "X Racial Trait|AUTOMATIC|Y|PREVAREQ:X_ReplaceY,1")], vec![]);
        assert_eq!(exclusion_guard_flags(&zero), vec!["X_ReplaceY"]);
        assert!(exclusion_guard_flags(&one).is_empty(), "`,1` is the opposite statement");

        // Branch 4 is a fallback, never an addition: a row the first branches
        // already guarded does not also absorb its own `sets_replace_flags`.
        let both = row(
            vec![
                ("ABILITY", "X Racial Trait|AUTOMATIC|Y|PREVAREQ:X_ReplaceY,0"),
                ("PREABILITY", "1,CATEGORY=Special Ability,X ~ Parent"),
            ],
            vec!["X_ReplaceSomethingElse"],
        );
        assert_eq!(exclusion_guard_flags(&both), vec!["X_ReplaceY"], "branch 4 must stay a fallback");

        // A positive `PREMULT` branch contributes nothing; only negated ones
        // are read, and only clauses naming a `_Replace` flag.
        let positive = row(
            vec![("PREMULT", "1,[PREABILITY:1,CATEGORY=Special Ability,X ~ Z],[!PREFACT:1,ABILITIES,X_ReplaceZ=true]")],
            vec![],
        );
        assert_eq!(exclusion_guard_flags(&positive), vec!["X_ReplaceZ"]);

        // Two one-flag gates are not one two-flag gate.
        let two_gates = row(
            vec![("!PREFACT", "1,ABILITIES,A=True"), ("!PREFACT", "1,ABILITIES,B=True")],
            vec![],
        );
        let one_gate = row(vec![("!PREFACT", "1,ABILITIES,A=True,B=True")], vec![]);
        assert_eq!(negated_fact_gates(&two_gates), vec![vec!["A".to_string()], vec!["B".to_string()]]);
        assert_eq!(negated_fact_gates(&one_gate), vec![vec!["A".to_string(), "B".to_string()]]);
    }
}
