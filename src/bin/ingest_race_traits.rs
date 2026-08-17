//! Ingests a book's **alternate racial traits** from its
//! `*_abilities_race.lst` into Shape B v1 `RaceTraitCacheData` records under
//! `data/corpus/<book>/race_trait/<race>/<trait>.json`.
//!
//! # Books
//!
//! [`BOOK_SOURCES`] is the whole per-book surface: a source path, a corpus
//! directory, and nothing else. The Advanced Race Guide came first (SD-27);
//! Monster Codex is SD-29's race-trait lane pilot. Everything between those two
//! entries — the row parser, the `DESC:` renderer, the replace-flag protocol,
//! the in-scope-race filter and the PI screen — is shared, because each is a
//! property of PCGen's `.lst` format rather than of a book. The alternative
//! this replaced was a per-book copy of the same 1,100 lines, of which the repo
//! already carries one (`src/bin/ingest_apg_race_traits.rs`).
//!
//! **Why this is ARG's real contribution.** `decisions.md §25.2` records that
//! ARG declares *zero* races of its own -- all 37 races in `arg_races.lst` are
//! `.MOD` reprints whose chassis lives in PCGen's shared `core_essentials/`
//! storage. ARG's genuine own content is the alternate-racial-trait corpus in
//! this one file (`decisions.md §25.4`), and that is what this binary ingests.
//!
//! **Scope filter (`decisions.md §25.3`).** Only the **18** races whose true
//! source book is already ingested are emitted: Core Rulebook's 7 (Dwarf, Elf,
//! Gnome, Half-Elf, Half-Orc, Halfling, Human) and Bestiary 1's 11 (Aasimar,
//! Drow, Duergar, Goblin, Hobgoblin, Kobold, Merfolk, Orc, Svirfneblin, Tengu,
//! Tiefling). Traits belonging to B2/B3/B4/ISWG races are counted and reported,
//! never written -- emitting them would manufacture content for a book nobody
//! has audited. `core_essentials` is never used as a book attribution; these
//! records are ARG's own, so they file under `advanced_race_guide`.
//!
//! **The replace-flag protocol (`decisions.md §26`), read off the corpus, not
//! guessed.** A standard racial trait in `core_essentials/races/<race>/
//! <race>_abilities_race.lst` is gated on a negated fact check naming its own
//! flag, e.g. `!PREFACT:1,ABILITIES,Dwarf_ReplaceGreed=True`. An ARG alternate
//! *sets* that flag with a trailing token of the exact form
//!
//! ```text
//! FACT:Dwarf_ReplaceGreed|True
//! ```
//!
//! That `FACT:<Race>_Replace<Trait>|True` token -- **not** the
//! `!PREFACT:...=true` occurrences, which only *read* flags -- is what
//! [`RaceTraitCacheData::sets_replace_flags`] captures.
//!
//! Alternates additionally carry a `PREMULT:1,[PREABILITY:...this ability...],
//! [!PREFACT:1,ABILITIES,<flag>=true]` clause. That is a **self-exclusion
//! guard** ("you may not take a second trait replacing something you already
//! replaced"), not a suppression by some other trait: verified over the corpus,
//! every flag named inside such a `PREMULT` is a flag the same row itself sets.
//! It is therefore preserved verbatim in `raw_tokens` rather than being
//! laundered into `suppressed_by_flag`, which is reserved for a *standalone*
//! `!PREFACT` gate (the shape standard traits use).
//!
//! **`DESC:` rendering.** PCGen's description syntax is not prose: a segment
//! carries a `|`-delimited tail of substitution arguments and prerequisite
//! gates, `%N` references argument N, and `%%` is a literal-percent escape.
//! This binary originally stored only the leading segment, which shipped
//! *"Three %1 times per day ... a +%1 luck bonus"* and *"reduced by 20%%"* to
//! the Race Traits panel. It now renders descriptions the same way
//! `src/bin/ingest_races.rs` already renders the Core Rulebook and Bestiary
//! traits: gates are evaluated against the row's own `DEFINE:`/`BONUS:VAR`
//! literals, `%N` is substituted from those literals, and `%%` collapses to
//! one sign. An argument that is not a same-row literal — ARG has exactly one,
//! `Halfling_AdaptableLuck_Bonus-1`, which is an *expression* and so
//! unreadable without the formula interpreter `decisions.md §24` forbids — is
//! **dropped and reported, never guessed**. [`leaked_pcgen_syntax`] is a
//! production guard: any description still carrying PCGen syntax fails the run
//! instead of reaching a screen.
//!
//! Run with `cargo run --bin ingest_race_traits` for every declared book, or
//! `cargo run --bin ingest_race_traits -- monster_codex` for one.
//! `PCGEN_CORPUS_ROOT` may point at a local PCGen `data/` checkout; it defaults
//! to `$HOME/workspace/repos/pcgen/data`.
//!
//! # A book's own race scope is not this binary's to widen
//!
//! [`IN_SCOPE_RACES`] is the 18 races whose chassis this project has ingested.
//! A row belonging to any other race is **counted and reported, never
//! written** — Monster Codex's six Ratfolk alternates are the live instance.
//! Writing them would create the only Ratfolk content in the repo, for a race
//! with no chassis, no default traits and no picker entry: content invented to
//! make a count look better, which is the failure this filter exists to
//! prevent.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use codex::rules_core::cache_gen::WiringClassIndex;
use codex::rules_core::pi_screening;
use codex::rules_core::shape_b_v1::{
    Completeness, CorpusRecordV1, CorpusSource, Population, RaceTraitCacheData, RawBonusChain, RawToken,
};

/// One book's alternate-racial-trait source file.
///
/// Every field is a *location*, never a behaviour: the parsing, the `DESC:`
/// rendering, the replace-flag protocol and the in-scope-race filter are
/// identical across books because they are properties of PCGen's `.lst` format,
/// not of any one book. That is why adding a book here is the whole cost of
/// adding a book — see this binary's module doc.
struct BookSource {
    /// The corpus book directory the records file under, and the book id the
    /// wiring-class index is built for. These are the same string for every
    /// book this binary writes.
    corpus_book: &'static str,
    /// The source `.lst` files, relative to the PCGen `data/` root. The one a
    /// row came from is written verbatim into that record's `source.path`.
    ///
    /// **A list rather than a single path since SD-29's race-trait lane round
    /// 4.** A book may declare its racial traits across more than one file --
    /// `core_essentials` states Aasimar's in
    /// `races/aasimar/aasimar_abilities_race_subrace.lst` and Tiefling's in
    /// `races/tiefling/tiefling_abilities_race_subrace.lst` -- and the output
    /// tree is rebuilt per *book*, so two `BookSource` rows sharing one
    /// `corpus_book` would have the second silently erase the first's records.
    lst_relatives: &'static [&'static str],
    /// The `<race>_abilities_globalvar_subrace.lst` files that state, per
    /// heritage selector, which replacement traits it grants and which
    /// standard trait each one displaces. Empty for every book whose racial
    /// traits declare their own gate on their own row. See [`subrace_grants`].
    subrace_globalvar_relatives: &'static [&'static str],
    /// The book's own directory under the PCGen `data/` root, which the
    /// wiring-class index scans for the row's magnitude signals.
    pcgen_book_relative: &'static str,
}

/// Every book whose alternate racial traits this binary ingests.
///
/// `advanced_race_guide` is the original (`decisions.md §25.4`);
/// `monster_codex` is SD-29's race-trait lane pilot, re-pinned from
/// `inner_sea_intrigue` because that book carries zero player-race racial
/// traits (SD-29 `decisions.md §43`). Monster Codex is the only re-pin
/// candidate whose `race_trait`-kinded rows are genuine *player-race* alternate
/// racial traits rather than monster or eidolon abilities filed under a
/// `_abilities_race` filename.
const BOOK_SOURCES: &[BookSource] = &[
    BookSource {
        corpus_book: "advanced_race_guide",
        lst_relatives: &["pathfinder/paizo/roleplaying_game/advanced_race_guide/arg_abilities_race.lst"],
        subrace_globalvar_relatives: &[],
        pcgen_book_relative: "pathfinder/paizo/roleplaying_game/advanced_race_guide",
    },
    // Advanced Player's Guide -- INVESTIGATED and deliberately NOT added as a
    // `BookSource`, SD-31 Epic 6-F4 (2026-08-15). `docs/work-inventory.json`
    // carried 49 of its rows as `evidence ==
    // "race_trait_absent_from_race_traits"`, which reads as a genuine
    // not-yet-ingested gap. It is not: `advanced_race_guide` is Paizo's own
    // compilation reprint of APG's alternate-racial-trait system, and a
    // corpus-wide KEY scan (`python3 -c` joining every committed
    // `race_trait` record's `data.key` across books) found **49 of APG's 50
    // in-scope rows already ingested, byte-mechanically-identical (same
    // `sets_replace_flags`, cosmetic wording only), under
    // `advanced_race_guide`** -- e.g. `Dwarf ~ Ancient Enmity`
    // (`arg_abilities_race.lst:33` reprints `apg_abilities_race.lst:16`
    // near-verbatim). The 50th, `Half-Orc ~ Plagueborn`, is APG-exclusive
    // and was ALREADY ingested outside this binary (`SD29-E7-F2-010`,
    // `data/corpus/advanced_players_guide/race_trait/half_orc/
    // half_orc_plagueborn.json`) -- already `computed`/`grounded` before
    // this cycle touched anything. So APG's true, non-duplicate,
    // not-yet-ingested contribution is **zero**: adding this `BookSource`
    // was tried and reverted after `race_resolver`'s own test suite proved
    // the hazard directly -- `RaceCorpus` builds ONE global map keyed by
    // trait KEY across every book (not book+key), so ingesting APG's 49
    // reprints alongside ARG's already-committed originals produced
    // `panicked ... Dwarf: duplicate resolved trait Dwarf ~ Ancient Enmity`
    // and a picker-population count regression (`the_whole_corpus_
    // classifies_into_the_four_roles_with_no_leftovers`, 379 -> 330, the
    // -49 exactly accounting for the duplicates). This is a MEASUREMENT
    // finding, not a missing-mechanism one: the raw-corpus enumeration
    // counts each book's row independently, so two books reprinting the
    // SAME real trait are counted as two distinct not-done units when at
    // most one can ever ground. Logged to `OPEN-ISSUES.md` for an operator
    // ruling on how the denominator should treat this shape (a
    // resolver-side de-dup layer, or a Structural Exclusion Register entry
    // for the 49 phantom duplicates) rather than silently ingesting a
    // record set that would either crash the resolver or add zero real
    // coverage.

    BookSource {
        corpus_book: "monster_codex",
        lst_relatives: &["pathfinder/paizo/roleplaying_game/monster_codex/mc_abilities_race.lst"],
        subrace_globalvar_relatives: &[],
        pcgen_book_relative: "pathfinder/paizo/roleplaying_game/monster_codex",
    },
    // Inner Sea Races, SD-29's race-trait lane round 2. The single largest
    // alternate-racial-trait contribution after ARG's own: 68 of its 72
    // in-scope rows set a `<Race>_Replace<Trait>` flag, so they are
    // `TraitRole::Alternate` and reach a player through the picker that
    // already serves ARG, APG and Monster Codex. Nothing here is a new
    // mechanism -- which is the correction this round records against
    // `decisions.md §44.4`, whose successor queue put this book behind two
    // that DO need one.
    BookSource {
        corpus_book: "inner_sea_races",
        lst_relatives: &["pathfinder/paizo/campaign_setting/inner_sea_races/isr_abilities_race.lst"],
        subrace_globalvar_relatives: &[],
        pcgen_book_relative: "pathfinder/paizo/campaign_setting/inner_sea_races",
    },
    // Horror Adventures, SD-29's race-trait lane round 3. 41 of its 43
    // in-scope rows in the book's main `_abilities_race.lst` set a
    // `<Race>_Replace<Trait>` flag, so they are `TraitRole::Alternate` and
    // reach a player through the same picker that already serves ARG, APG,
    // Monster Codex and Inner Sea Races. Classified before the round
    // committed to the book, per SD-29 `decisions.md §45.1`:
    // `python3 scripts/classify_race_trait_rows.py ha_abilities_race.lst`
    // -> `in-scope rows 43 | default 0 | alternate 41 | flag_granted 0 |
    // unclassified 2`.
    //
    // **The book's second racial-ability file is deliberately NOT listed.**
    // `support/ha_abilities_race_oa.lst` carries one further in-scope row
    // (`Tiefling ~ Fiendish Heritage`-shaped, one alternate), but the pcc
    // loads it conditionally --
    // `ABILITY:support/ha_abilities_race_oa.lst|PRECAMPAIGN:1,INCLUDES=Occult
    // Adventures` (`_horror_adventures.pcc:91`) -- so its content exists only
    // for a game that also owns Occult Adventures, a book this repo has not
    // ingested. Listing it here would ingest it unconditionally and
    // mis-attach a conditional record to the base book, which is the hazard
    // `loop-instruction.md`'s "Conditional cross-book support files" note
    // names. The gate is on the pcc load line, not inside the `.lst`:
    // `grep PRECAMPAIGN support/ha_abilities_race_oa.lst` returns 0, so a
    // lane that checks the file for its own gate concludes wrongly that it is
    // ungated. Recorded as a scope finding for a successor round, not as gap.
    BookSource {
        corpus_book: "horror_adventures",
        lst_relatives: &["pathfinder/paizo/roleplaying_game/horror_adventures/ha_abilities_race.lst"],
        subrace_globalvar_relatives: &[],
        pcgen_book_relative: "pathfinder/paizo/roleplaying_game/horror_adventures",
    },
    // Core Essentials' Aasimar and Tiefling *heritage* traits, SD-29's
    // race-trait lane round 4 -- the last of the 553-unit ceiling that is
    // ordinary content rather than a chassis problem (`decisions.md` 47.8).
    //
    // 48 rows across two files, and they are the only rows in this lane whose
    // swap is not declared on the row itself. Each is gated
    // `PREABILITY:1,CATEGORY=Special Ability,<Race> ~ <Heritage>` on a
    // *selector* row typed `TYPE:<Race> Subrace`, and which standard trait it
    // replaces is stated in a third file. Both halves are read here: the
    // selectors become the records a player picks, and
    // [`subrace_grants`] supplies the replace-flags and the
    // `ABILITY:...|AUTOMATIC|...` grant links that make the 48 apply.
    //
    // Classified before the round committed to the book, per `decisions.md`
    // 45.1:
    // `python3 scripts/classify_race_trait_rows.py aasimar_abilities_race_subrace.lst tiefling_abilities_race_subrace.lst`
    // -> aasimar: `in-scope rows 18 | alternate 0 | flag_granted 18`;
    //    tiefling: `in-scope rows 30 | alternate 0 | flag_granted 30`.
    // Zero of the 48 are self-gating alternates, which is exactly why the
    // third file is not optional.
    //
    // **`races/skinwalker/` is deliberately not listed.** It carries the same
    // subrace shape, but Skinwalker is not one of the 18 races this project
    // models, so `IN_SCOPE_RACES` would drop every row and
    // `RaceCorpus::resolve` would return `None` for the chassis regardless.
    BookSource {
        corpus_book: "core_essentials",
        lst_relatives: &[
            "pathfinder/paizo/roleplaying_game/core_essentials/races/aasimar/aasimar_abilities_race_subrace.lst",
            "pathfinder/paizo/roleplaying_game/core_essentials/races/tiefling/tiefling_abilities_race_subrace.lst",
        ],
        subrace_globalvar_relatives: &[
            "pathfinder/paizo/roleplaying_game/core_essentials/races/aasimar/aasimar_abilities_globalvar_subrace.lst",
            "pathfinder/paizo/roleplaying_game/core_essentials/races/tiefling/tiefling_abilities_globalvar_subrace.lst",
        ],
        pcgen_book_relative: "pathfinder/paizo/roleplaying_game/core_essentials",
    },
];

/// The 18 in-scope races (`decisions.md §25.3`), spelled exactly as the corpus
/// spells them in its `TYPE:<Race> Racial Trait` component.
// Widened 18 -> 24 by SD-31 Epic 1-F2 (2026-08-15): Bestiary 2's 6
// non-heritage races (Dhampir excluded -- see `ingest_races.rs`'s
// `IN_SCOPE_RACES` doc comment for why). Kept in sync by hand with that
// binary's own race table; `race_resolver::RaceCorpus::resolve` only
// resolves a race whose chassis this OTHER binary (`ingest_races`) wrote,
// so a race added only here and not there would collect alternate-trait
// rows into a `race_trait/<race>/` directory `RaceCorpus::chassis()` never
// populates -- loaded but permanently unreachable, not merely incomplete.
//
// Widened 24 -> 30 by SD-31-E6-F4-003 (2026-08-16): `ingest_races.rs`'s own
// SD-31-E6-F4-002 batch gave 6 ARG-native races (Catfolk, Kitsune, Ratfolk,
// Strix, Suli, Wayang) a real chassis, but this binary's roster was never
// widened to match, so `arg_abilities_race.lst`'s real
// `###Block: Alternate Racial Traits` rows for those 6 (confirmed non-`.MOD`
// content by direct inspection of the pinned oracle -- Catfolk 6, Kitsune 2,
// Ratfolk 4, Strix 6, Suli 5, Wayang 1 -- Kitsune corrected 2026-08-17,
// SD31-W9-INTEGRATE-001: the prior "7" summed in Kitsune's 5 `Favored Class
// Bonus ~ <Class> ~ Kitsune` rows, a DIFFERENT unit kind every other race's
// own figure in this list already excludes) sat un-ingested. `race_dir` is now
// SHARED between the two binaries for these 6 races (both write into
// `advanced_race_guide/race_trait/<race>/`); see
// `clear_own_alternate_trait_files`'s doc comment for how that clear no
// longer destroys the sibling binary's files.
const IN_SCOPE_RACES: [&str; 34] = [
    // Core Rulebook (7)
    "Dwarf",
    "Elf",
    "Gnome",
    "Half-Elf",
    "Half-Orc",
    "Halfling",
    "Human",
    // Bestiary 1 (11)
    "Aasimar",
    "Drow",
    "Duergar",
    "Goblin",
    "Hobgoblin",
    "Kobold",
    "Merfolk",
    "Orc",
    "Svirfneblin",
    "Tengu",
    "Tiefling",
    // Bestiary 2 (6), SD-31 Epic 1-F2
    "Fetchling",
    "Grippli",
    "Ifrit",
    "Oread",
    "Sylph",
    "Undine",
    // Advanced Race Guide native chassis (6), SD-31-E6-F4-002/003
    "Catfolk",
    "Kitsune",
    "Ratfolk",
    "Strix",
    "Suli",
    "Wayang",
    // Advanced Race Guide follow-on chassis (4), SD31-E6-F4-006 -- standard
    // tier already ingested by `ingest_races.rs`'s SD31-E6-F4-004 batch;
    // this widens THIS binary's roster so their real alternate racial
    // trait rows in `arg_abilities_race.lst` pass the in-scope filter too.
    "Gillman",
    "Nagaji",
    "Vanara",
    "Vishkanya",
];

const RACIAL_TRAIT_TYPE_SUFFIX: &str = " Racial Trait";
const RACIAL_DEFAULT_TYPE_SUFFIX: &str = " Racial Default";
/// The TYPE component that marks a *heritage selector* row -- `TYPE:Aasimar
/// Subrace`, `TYPE:Tiefling Subrace`. See [`subrace_grants`].
const SUBRACE_TYPE_SUFFIX: &str = " Subrace";
/// PCGen's suffix marking a row an *update* to a record declared elsewhere.
const MOD_MARKER: &str = ".MOD";

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

/// `true` when `path` is a record THIS binary could itself have written --
/// i.e. its stored `data.is_racial_default` is `false`. Shared by the
/// scoped clear and the scoped on-disk count below, both of which need the
/// identical ownership test. See [`clear_own_alternate_trait_files`]'s doc
/// comment for why this partition is exact, not a guess.
fn is_own_alternate_trait_record(path: &Path) -> bool {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("failed to read {path:?} to decide ownership: {e}"));
    let parsed: serde_json::Value =
        serde_json::from_str(&text).unwrap_or_else(|e| panic!("{path:?} is not valid JSON, cannot decide ownership safely: {e}"));
    let is_racial_default = parsed
        .get("data")
        .and_then(|d| d.get("is_racial_default"))
        .and_then(|v| v.as_bool())
        .unwrap_or_else(|| {
            panic!(
                "{path:?} has no boolean data.is_racial_default -- cannot tell whether this \
                 binary or `ingest_races.rs` wrote it, so ownership refuses to guess"
            )
        });
    !is_racial_default
}

/// Clears exactly the `.json` files in `race_dir` that THIS binary could
/// itself have written on a prior run, and leaves every other file alone --
/// the fix for the mutual-destruction hazard `ingest_book`'s clear-loop doc
/// comment describes (`SD-31-E6-F4-003`, `advanced_race_guide`'s shared
/// Catfolk/Kitsune/Ratfolk/Strix/Suli/Wayang directories).
///
/// This binary never writes an `is_racial_default: true` record (verified
/// corpus-wide: zero counter-examples across every book it has ever
/// ingested); `ingest_races.rs` never writes an `is_racial_default: false`
/// one for these 6 races (see `ingest_races.rs`'s own
/// `clear_own_standard_trait_files` doc comment). A `.json` file that does
/// not parse, or is missing that field, belongs to neither binary's known
/// shape -- refused rather than guessed at, per this repo's no-stub
/// discipline: a silent guess here is exactly how a sibling binary's real
/// content gets deleted.
fn clear_own_alternate_trait_files(race_dir: &Path) {
    let entries = fs::read_dir(race_dir)
        .unwrap_or_else(|e| panic!("failed to list {race_dir:?} for a scoped clear: {e}"));
    for entry in entries {
        let entry = entry.unwrap_or_else(|e| panic!("failed to read a directory entry under {race_dir:?}: {e}"));
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        if is_own_alternate_trait_record(&path) {
            fs::remove_file(&path).unwrap_or_else(|e| panic!("failed to remove {path:?} during a scoped clear: {e}"));
        }
    }
}

/// A recursive `.json`-file count, filtered to records this binary could
/// itself have written (`is_own_alternate_trait_record`). Needed wherever
/// `advanced_race_guide/race_trait/<race>/` is shared with
/// `ingest_races.rs` (Catfolk/Kitsune/Ratfolk/Strix/Suli/Wayang,
/// `SD-31-E6-F4-003`): an unfiltered count there would count that sibling
/// binary's preserved files too, turning a correct run into a false
/// self-check mismatch.
fn count_own_json(dir: &Path) -> usize {
    let mut n = 0;
    for entry in fs::read_dir(dir).unwrap_or_else(|e| panic!("failed to read {dir:?}: {e}")) {
        let entry = entry.expect("readable dir entry");
        let path = entry.path();
        if path.is_dir() {
            n += count_own_json(&path);
        } else if path.extension().is_some_and(|e| e == "json") && is_own_alternate_trait_record(&path) {
            n += 1;
        }
    }
    n
}

fn ingested_at_now() -> String {
    let output = std::process::Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .expect("`date -u` must be available to stamp ingested_at");
    String::from_utf8(output.stdout).expect("date output is valid UTF-8").trim().to_string()
}

/// Same slug rule `gen_book_cache.rs` already uses for every other
/// content kind, so `race_trait/` paths read like their `feat/`/`spell/`
/// siblings.
fn slugify(raw: &str) -> String {
    let mut out = String::new();
    let mut last_was_sep = false;
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            last_was_sep = false;
        } else if !last_was_sep {
            out.push('_');
            last_was_sep = true;
        }
    }
    let trimmed = out.trim_matches('_').to_string();
    if trimmed.is_empty() { "record".to_string() } else { trimmed }
}

fn pcgen_data_root() -> PathBuf {
    if let Ok(v) = std::env::var("PCGEN_CORPUS_ROOT") {
        return PathBuf::from(v);
    }
    let home = std::env::var("HOME").expect("HOME must be set to locate the default PCGen corpus checkout");
    PathBuf::from(home).join("workspace/repos/pcgen/data")
}

/// One tab-delimited `KEY:VALUE` field off a corpus row. The row's single
/// bare (colon-less) leading field is the ability's display name and is not
/// represented here -- it becomes `RaceTraitCacheData::name`.
struct Field {
    key: String,
    value: String,
}

/// Splits a raw LST row into its non-empty tab-delimited fields. PrettyLST
/// pads columns with runs of tabs, so empty fields are structural padding and
/// carry no content.
fn split_fields(line: &str) -> Vec<&str> {
    line.split('\t').map(str::trim).filter(|f| !f.is_empty()).collect()
}

/// The parsed shape of one alternate-racial-trait row.
struct TraitRow {
    line_number: u32,
    name: String,
    key: String,
    race_key: String,
    category: Option<String>,
    type_tokens: Vec<String>,
    is_racial_default: bool,
    /// True when the row's race key came from a `TYPE:<Race> Subrace`
    /// component rather than `TYPE:<Race> Racial Trait` -- PCGen's *heritage
    /// selector*. See [`subrace_grants`] for the whole shape.
    is_subrace_selector: bool,
    suppressed_by_flag: Option<String>,
    sets_replace_flags: Vec<String>,
    description: Option<String>,
    /// `DESC:` arguments that are not same-row literals. Dropped from the
    /// player-facing prose and reported by the run, never guessed.
    unresolved_desc_args: Vec<String>,
    source_page: Option<String>,
    raw_tokens: Vec<RawToken>,
    raw_bonus_chains: Vec<RawBonusChain>,
}

/// Extracts the flag name a `!PREFACT:1,ABILITIES,<flag>=true` clause reads.
/// Used only for *standalone* `!PREFACT` fields; `PREMULT`-wrapped ones are a
/// different construct (see this module's doc comment).
fn prefact_flag(clause_value: &str) -> Option<String> {
    // Value shape: `1,ABILITIES,<flag>=true`
    let mut parts = clause_value.split(',');
    let _count = parts.next()?;
    let scope = parts.next()?;
    if scope != "ABILITIES" {
        return None;
    }
    let assignment = parts.next()?;
    let (flag, truth) = assignment.split_once('=')?;
    if !truth.eq_ignore_ascii_case("true") {
        return None;
    }
    Some(flag.trim().to_string())
}

// ---------------------------------------------------------------------
// `DESC:` rendering
//
// Ported from `src/bin/ingest_races.rs`, which already gives the 175
// Core Rulebook / Bestiary standard traits their player-facing prose.
// This binary shipped without it and put raw PCGen syntax on screen.
// Behaviour is deliberately identical, with one addition this book needs
// and that book never hit: the `%%` literal-percent escape.
// ---------------------------------------------------------------------

/// Every variable this row defines *and finishes* on its own, with its
/// resolved integer value — or `None` where the row names the variable
/// but its value depends on something the row does not itself state.
///
/// PCGen seeds a row-local variable with `DEFINE:<Var>|<base>` and adds
/// to it with `BONUS:VAR|<Var>|<value>`. Where both are integer literals
/// on the same row the variable is a constant written across two tokens:
/// `Halfling ~ Adaptable Luck` carries `DEFINE:Halfling_AdaptableLuck_Bonus|0`
/// and `BONUS:VAR|Halfling_AdaptableLuck_Bonus|2`, so the value is 2 and
/// reading it is transcription, not evaluation. `decisions.md §24`'s ban
/// on a formula interpreter is therefore not engaged.
///
/// The instant any contribution stops being a same-row literal — a
/// formula (`BONUS:VAR|X|OtherVar`), a conditional bonus (a trailing
/// `PRE...` qualifier), or a base declared elsewhere in the corpus — the
/// variable is marked unresolvable and **no value is guessed**.
fn same_row_vars(parsed: &[Field]) -> BTreeMap<String, Option<i64>> {
    let mut vars: BTreeMap<String, Option<i64>> = BTreeMap::new();

    for f in parsed.iter().filter(|f| f.key == "DEFINE") {
        let Some((name, base)) = f.value.split_once('|') else { continue };
        vars.insert(name.trim().to_string(), base.trim().parse::<i64>().ok());
    }

    for f in parsed.iter().filter(|f| f.key == "BONUS") {
        let quals: Vec<&str> = f.value.split('|').collect();
        if !quals.first().map(|q| q.eq_ignore_ascii_case("VAR")).unwrap_or(false) {
            continue;
        }
        let (Some(names), Some(amount)) = (quals.get(1), quals.get(2)) else { continue };
        let conditional = quals[3..].iter().any(|q| q.starts_with("PRE") || q.starts_with("!PRE"));
        let amount = if conditional { None } else { amount.trim().parse::<i64>().ok() };
        for name in names.split(',') {
            let name = name.trim().to_string();
            match vars.get_mut(&name) {
                // Never `DEFINE`d here: the base lives in another file,
                // so this row cannot resolve the variable by itself.
                None => {
                    vars.insert(name, None);
                }
                Some(slot) => {
                    *slot = match (*slot, amount) {
                        (Some(current), Some(add)) => Some(current + add),
                        _ => None,
                    };
                }
            }
        }
    }

    vars
}

/// True when a `DESC:` argument is a prerequisite gate rather than a
/// substitution argument. PCGen prerequisites are upper-case and always
/// carry a colon (`PREVARGTEQ:Halfling_AdaptableLuck_Times,4`); variable
/// names never contain one.
fn is_prerequisite_arg(arg: &str) -> bool {
    arg.contains(':') && (arg.starts_with("PRE") || arg.starts_with("!PRE"))
}

/// Evaluates one `PREVAR<CMP>:<lhs>,<rhs>[,<lhs>,<rhs>...]` gate against
/// the row's own variable table, honouring a leading `!` as negation and
/// requiring every pair to hold.
///
/// This compares two same-row constants; it is not formula evaluation.
/// Anything undecidable — an unknown comparison, a prerequisite kind this
/// does not model, an operand defined elsewhere — is an `Err`, never a
/// coin flip: a gate decides what the rules text *says* ("Three" vs a
/// variable count), so guessing it would ship a false statement rather
/// than merely an incomplete one.
fn eval_prevar_gate(token: &str, vars: &BTreeMap<String, Option<i64>>) -> Result<bool, String> {
    let (negated, body) = match token.strip_prefix('!') {
        Some(rest) => (true, rest),
        None => (false, token),
    };
    let (head, args) = body.split_once(':').ok_or_else(|| format!("malformed DESC gate {token:?}"))?;
    let cmp = head.strip_prefix("PREVAR").ok_or_else(|| format!("unmodelled DESC gate kind {token:?}"))?;

    let operand = |raw: &str| -> Result<i64, String> {
        let raw = raw.trim();
        if let Ok(n) = raw.parse::<i64>() {
            return Ok(n);
        }
        vars.get(raw)
            .copied()
            .flatten()
            .ok_or_else(|| format!("DESC gate {token:?}: {raw:?} is not a same-row literal"))
    };

    let parts: Vec<&str> = args.split(',').collect();
    if parts.is_empty() || !parts.len().is_multiple_of(2) {
        return Err(format!("DESC gate {token:?} is not a list of <operand>,<value> pairs"));
    }

    let mut all = true;
    for pair in parts.chunks(2) {
        let (lhs, rhs) = (operand(pair[0])?, operand(pair[1])?);
        all &= match cmp {
            "EQ" => lhs == rhs,
            "NEQ" => lhs != rhs,
            "LT" => lhs < rhs,
            "LTEQ" => lhs <= rhs,
            "GT" => lhs > rhs,
            "GTEQ" => lhs >= rhs,
            other => return Err(format!("DESC gate {token:?}: unmodelled comparison {other:?}")),
        };
    }
    Ok(negated != all)
}

/// Collapses every whitespace run to a single space and trims the ends.
/// Applied only where a placeholder was removed, so prose that needed no
/// edit stays byte-identical to the source.
fn collapse_whitespace(text: &str) -> String {
    let mut out = String::new();
    let mut last_was_space = false;
    for ch in text.chars() {
        if ch.is_whitespace() {
            if !last_was_space {
                out.push(' ');
            }
            last_was_space = true;
        } else {
            out.push(ch);
            last_was_space = false;
        }
    }
    out.trim().to_string()
}

/// Renders one `DESC:` segment's prose: `%%` becomes a literal `%`, and
/// every `%N` becomes argument N's resolved literal. Returns the rendered
/// text and the names of any arguments that would not resolve.
///
/// Two different PCGen constructs share the `%` sign and must not be
/// conflated:
///
/// * `%%` is an **escape**. `arg_abilities_race.lst` writes
///   `reduced by 20%%` and `(50%% or fewer hit points)`; the player must
///   see one sign. Nothing is looked up and nothing can be lost.
/// * `%N` is an **argument reference** into the segment's `|`-delimited
///   tail.
///
/// An unresolvable argument is **dropped, never guessed**: the
/// placeholder goes, the `+`/`-` sign that introduced it goes with it,
/// and the whitespace is closed up so the sentence still reads. The raw
/// argument tail is not emitted under any branch — that is the whole
/// point of this function.
fn substitute_placeholders(prose: &str, args: &[&str], vars: &BTreeMap<String, Option<i64>>) -> (String, Vec<String>) {
    let chars: Vec<char> = prose.chars().collect();
    let mut out = String::new();
    let mut unresolved: Vec<String> = Vec::new();
    let mut dropped_any = false;
    let mut i = 0;

    while i < chars.len() {
        // The escape is checked first: `%%` is never an argument
        // reference, and `%%1` would otherwise be misread as one.
        if chars[i] == '%' && chars.get(i + 1) == Some(&'%') {
            out.push('%');
            i += 2;
            continue;
        }
        if chars[i] == '%'
            && let Some(digit) = chars.get(i + 1).and_then(|c| c.to_digit(10))
            && digit >= 1
        {
            let arg = args.get(digit as usize - 1).copied();
            let value = arg.and_then(|name| {
                let name = name.trim();
                name.parse::<i64>().ok().or_else(|| vars.get(name).copied().flatten())
            });
            match value {
                Some(v) => out.push_str(&v.to_string()),
                None => {
                    if let Some(name) = arg {
                        unresolved.push(name.to_string());
                    }
                    while out.ends_with('+') || out.ends_with('-') {
                        out.pop();
                    }
                    dropped_any = true;
                }
            }
            i += 2;
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }

    let text = if dropped_any { collapse_whitespace(&out) } else { out };
    (text, unresolved)
}

/// One row's rendered player-facing description, plus the arguments that
/// could not be resolved (reported by the binary rather than swallowed).
#[derive(Debug, Clone, PartialEq, Eq)]
struct RenderedDescription {
    text: Option<String>,
    unresolved_args: Vec<String>,
}

/// Turns a row's `DESC:` tokens into the prose the player actually sees.
///
/// PCGen's format is `DESC:<prose>|<arg1>|<arg2>...`, where `%1` in the
/// prose stands for arg 1's resolved value and where an argument that
/// looks like a prerequisite gates the whole segment instead. A row may
/// carry several `DESC:` tokens (`Halfling ~ Adaptable Luck` carries
/// five, two of them mutually exclusive gates); the surviving segments
/// concatenate, in source order, into one description.
///
/// Storing the leading segment instead — which is what this binary used
/// to do — put PCGen substitution syntax on screen verbatim, e.g.
/// *"Three %1 times per day ... a +%1 luck bonus"* and *"reduced by 20%%"*.
fn render_description(parsed: &[Field]) -> Result<RenderedDescription, String> {
    let vars = same_row_vars(parsed);
    let mut segments: Vec<String> = Vec::new();
    let mut unresolved_args: Vec<String> = Vec::new();
    let mut saw_desc = false;

    for f in parsed.iter().filter(|f| f.key == "DESC") {
        saw_desc = true;
        let mut parts = f.value.split('|');
        let prose = parts.next().unwrap_or_default();
        let (gates, args): (Vec<&str>, Vec<&str>) = parts.partition(|p| is_prerequisite_arg(p));

        let mut applies = true;
        for gate in &gates {
            // `!PREABILITY` guards are the "you already have this" shape
            // and are not variable comparisons; they never suppress the
            // segment for ingest purposes, and are preserved verbatim in
            // `raw_tokens`. Only `PREVAR` gates are evaluated.
            if !gate.trim_start_matches('!').starts_with("PREVAR") {
                continue;
            }
            // Every gate is evaluated even once one has failed, so an
            // undecidable gate is surfaced rather than masked by a
            // neighbour that happened to be decided first.
            applies &= eval_prevar_gate(gate, &vars)?;
        }
        if !applies {
            continue;
        }

        let (text, mut unresolved) = substitute_placeholders(prose.trim(), &args, &vars);
        unresolved_args.append(&mut unresolved);
        if !text.is_empty() {
            segments.push(text);
        }
    }

    let joined = segments.join(" ");
    let text = if !saw_desc || joined.is_empty() { None } else { Some(joined) };
    Ok(RenderedDescription { text, unresolved_args })
}

/// The PCGen syntax that must never reach a player: an unsubstituted
/// `%<digit>` argument reference, an unresolved `%%` literal-percent
/// escape, or a raw `|` argument tail. Used as a production guard on
/// every description this binary writes.
fn leaked_pcgen_syntax(text: &str) -> Option<&'static str> {
    if text.contains('|') {
        return Some("raw '|' argument tail");
    }
    if text.contains("%%") {
        return Some("unescaped '%%' literal-percent escape");
    }
    let chars: Vec<char> = text.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        if *c == '%' && chars.get(i + 1).is_some_and(char::is_ascii_digit) {
            return Some("unsubstituted '%N' argument reference");
        }
    }
    None
}

/// True when a row's leading field marks it a PCGen `.MOD` — an *update* to a
/// record declared elsewhere, not a declaration.
///
/// Only field 0 decides. `.MOD` occurring inside a token *value* (PCGen writes
/// `var("STAT.3.MOD...")`) is not a mod row, which is why this reads the
/// leading field rather than searching the line. That distinction is
/// `v06_corpus_trap_report`'s own `mod-record` trap, whose stated risk is
/// exactly this: "counting these as declarations inflates a record estimate".
fn is_mod_row(fields: &[&str]) -> bool {
    fields.first().is_some_and(|f| f.contains(".MOD"))
}

/// One `<Race> Racial Trait|AUTOMATIC|<key>|PREVAREQ:<flag>,0` grant a
/// heritage selector's `.MOD` block declares.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SubraceGrant {
    /// The replacement trait this selector hands the character.
    granted_trait_key: String,
    /// The `<Race>_Replace<Trait>` flag guarding the standard trait the
    /// replacement stands in for.
    replaces_flag: String,
}

/// Reads a race's `<race>_abilities_globalvar_subrace.lst` and returns, per
/// heritage-selector key, the replacement traits that selector grants and the
/// standard-trait flag each one displaces.
///
/// # Why this file has to be read at all
///
/// `core_essentials`' two subrace files are the last books in this lane
/// (`decisions.md` Section 47.8) and they state the swap in a shape no other
/// book this binary ingests uses. The selector row is typed `TYPE:Aasimar
/// Subrace` and the replacement rows are typed `TYPE:...Aasimar Racial
/// Trait...` and gated `PREABILITY:1,CATEGORY=Special Ability,Aasimar ~
/// Agathion-Blooded` -- so on the evidence of that file alone the replacements
/// have a *positive* gate naming an ability, and nothing says which standard
/// traits they replace. Ingesting them on that evidence produces records that
/// load and never apply, or worse, apply *alongside* the standard trait and
/// double its bonus.
///
/// The missing half is stated, per selector, in the race's
/// `_abilities_globalvar_subrace.lst`:
///
/// ```text
/// CATEGORY=Special Ability|Aasimar ~ Agathion-Blooded.MOD
///     ABILITY:Aasimar Racial Trait|AUTOMATIC|Agathion-Blooded ~ Ability Scores|PREVAREQ:Aasimar_ReplaceAbilityScores,0
///     ABILITY:Aasimar Racial Trait|AUTOMATIC|Aasimar ~ Type|PREVAREQ:Aasimar_ReplaceType,0
/// ```
///
/// Read: *taking this heritage grants `Agathion-Blooded ~ Ability Scores`
/// while `Aasimar_ReplaceAbilityScores` is 0*. The block names the selector's
/// **whole** effective trait set -- the race's own standard rows where the
/// heritage keeps them, and a subrace-specific row where it does not. Only the
/// latter are returned: a grant whose target is the race's own
/// `<Race> ~ <Trait>` key replaces nothing.
///
/// This is exactly the second gate source `ingest_races::globalvar_gates`
/// already reads for a race's *default* trait set
/// (`CATEGORY=Special Ability|Aasimar ~ Default.MOD`); the subrace file is the
/// same protocol addressed to a heritage instead of to `~ Default`.
///
/// # Why reading it is transcription, not invention
///
/// It is checkable against a first source and **is** checked, by the caller:
/// Aasimar's six selector rows carry their own
/// `FACT:Aasimar_Replace<Trait>|True` tokens, and the flags derived here must
/// equal them exactly. Tiefling's ten carry no `FACT:` token at all, which is
/// the whole reason this file must be read -- and it is the same file, the
/// same token and the same rule that Aasimar proves.
///
/// Only `PREVAREQ:<flag>,0` is read, for `globalvar_gates`' stated reason: a
/// `PREVAREQ:<flag>,1` is the opposite statement and treating it as a
/// suppressor would invert the rule.
fn subrace_grants(text: &str, race_key: &str) -> BTreeMap<String, Vec<SubraceGrant>> {
    let category_prefix = format!("{race_key} Racial Trait");
    let standard_key_prefix = format!("{race_key} ~ ");
    let mut out: BTreeMap<String, Vec<SubraceGrant>> = BTreeMap::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let fields = split_fields(line);
        let Some(head) = fields.first() else { continue };
        // `CATEGORY=Special Ability|Aasimar ~ Agathion-Blooded` + the mod marker
        let Some(selector) = head.rsplit('|').next().and_then(|k| k.strip_suffix(MOD_MARKER)) else { continue };
        let selector = selector.trim();
        if selector.is_empty() {
            continue;
        }
        for field in &fields[1..] {
            let Some((key, value)) = field.split_once(':') else { continue };
            if key != "ABILITY" {
                continue;
            }
            let parts: Vec<&str> = value.split('|').collect();
            if parts.len() < 3 || parts[0].trim() != category_prefix || parts[1].trim() != "AUTOMATIC" {
                continue;
            }
            let target = parts[2].trim();
            // A grant naming the race's own standard trait replaces nothing:
            // it is this heritage keeping the trait it did not swap out.
            if target.starts_with(&standard_key_prefix) {
                continue;
            }
            let Some(flag) = parts[3..].iter().find_map(|clause| {
                let rest = clause.trim().strip_prefix("PREVAREQ:")?;
                let (flag, want) = rest.rsplit_once(',')?;
                (want.trim() == "0").then(|| flag.trim().to_string())
            }) else {
                continue;
            };
            let entry = out.entry(selector.to_string()).or_default();
            let grant = SubraceGrant { granted_trait_key: target.to_string(), replaces_flag: flag };
            if !entry.contains(&grant) {
                entry.push(grant);
            }
        }
    }
    out
}

/// True for a `SOURCEPAGE` value that is an upstream placeholder rather than a
/// page. Deliberately an exact-match list of the two spellings the corpus
/// actually uses, not a pattern: a page cite is free text and a heuristic here
/// would start discarding real ones.
fn is_placeholder_source_page(value: &str) -> bool {
    matches!(value.trim().to_ascii_lowercase().as_str(), "xx" | "p.xx" | "p. xx" | "pxx")
}

fn parse_row(line_number: u32, line: &str) -> Option<TraitRow> {
    let fields = split_fields(line);
    if fields.is_empty() {
        return None;
    }

    // A `.MOD` row declares nothing, so it can never become a record. ARG and
    // Monster Codex never exercised this: their `.MOD` rows carry no `TYPE:`
    // at all, so the race-key check below already rejected them and the guard
    // looked unnecessary. **Inner Sea Races is the counter-example** —
    // `isr_abilities_race.lst` carries 618 `.MOD` rows, 5 of which DO carry a
    // `<Race> Racial Trait` TYPE and so reach this far. All 5 name races this
    // project does not model, so they were filtered one step later by
    // `IN_SCOPE_RACES` and nothing wrong shipped; that is luck, not a rule, and
    // the next book's `.MOD` row for a modelled race would have been written
    // out as though it were a new alternate racial trait.
    if is_mod_row(&fields) {
        return None;
    }

    let mut name: Option<String> = None;
    let mut parsed: Vec<Field> = Vec::new();
    for field in &fields {
        match field.split_once(':') {
            Some((key, value)) => parsed.push(Field { key: key.to_string(), value: value.to_string() }),
            None => {
                if name.is_none() {
                    name = Some((*field).to_string());
                } else {
                    panic!("line {line_number}: more than one bare (colon-less) field: {field:?}");
                }
            }
        }
    }

    // TYPE components decide whether this row is a racial trait at all, and
    // which race owns it. `TYPE:RacialTraits.Dwarf Racial Trait.SpecialQuality`
    // -> race "Dwarf". Rows may carry more than one TYPE field.
    let mut type_tokens: Vec<String> = Vec::new();
    for f in parsed.iter().filter(|f| f.key == "TYPE") {
        type_tokens.extend(f.value.split('.').map(|t| t.trim().to_string()).filter(|t| !t.is_empty()));
    }
    //
    // A row may instead be a *heritage selector*, typed `TYPE:<Race> Subrace`
    // (`Aasimar ~ Agathion-Blooded`, `Tiefling ~ Asura-Spawn`). It is the row
    // a player actually chooses, and the 48 `<Race> Racial Trait` replacement
    // rows in the same file are gated on it -- so reading the replacements
    // without it ships records that load and never apply. The `Racial Trait`
    // component is tried first and wins: nothing in the corpus carries both,
    // and the more specific reading is the right one if anything ever does.
    let racial_trait_race =
        type_tokens.iter().find_map(|t| t.strip_suffix(RACIAL_TRAIT_TYPE_SUFFIX)).map(|r| r.trim().to_string());
    let is_subrace_selector = racial_trait_race.is_none();
    let race_key = match racial_trait_race {
        Some(race) => race,
        None => type_tokens.iter().find_map(|t| t.strip_suffix(SUBRACE_TYPE_SUFFIX)).map(|r| r.trim().to_string())?,
    };

    let name = name.unwrap_or_else(|| panic!("line {line_number}: racial-trait row has no display-name field"));

    // `KEY:` is optional in PCGen: a row that omits it is keyed by its own
    // display name. ARG states one on every racial-trait row, so this binary
    // used to panic on a missing one — correct for ARG, wrong as a general
    // rule, and Monster Codex is the counter-example (`mc_abilities_race.lst:30`
    // `Standard Goblin` and `:31` `Oversized Goblin` carry no `KEY:` at all).
    //
    // The default is not invented here: `src/bin/v06_work_inventory.rs` already
    // enumerates those same two rows under the keys `Standard Goblin` and
    // `Oversized Goblin`, so this makes the ingest agree with the inventory
    // rather than introducing a second convention. ARG's output is unaffected —
    // every one of its 156 rows still takes its explicit `KEY:`.
    let key = parsed
        .iter()
        .find(|f| f.key == "KEY")
        .map(|f| f.value.clone())
        .unwrap_or_else(|| name.clone());

    // Read the default marker off the corpus rather than assuming alternates
    // are never defaults -- `decisions.md §26` notes the standard set is
    // self-identifying via `TYPE:...<Race> Racial Default...`.
    let default_marker = format!("{race_key}{RACIAL_DEFAULT_TYPE_SUFFIX}");
    let is_racial_default = type_tokens.iter().any(|t| t == &default_marker);

    // `FACT:<flag>|True` is the *setting* form. `!PREFACT:...` occurrences read
    // flags and are deliberately not counted here.
    let mut sets_replace_flags: Vec<String> = Vec::new();
    for f in parsed.iter().filter(|f| f.key == "FACT") {
        let Some((flag, truth)) = f.value.split_once('|') else { continue };
        if !flag.contains("_Replace") || !truth.trim().eq_ignore_ascii_case("true") {
            continue;
        }
        let flag = flag.trim().to_string();
        if !sets_replace_flags.contains(&flag) {
            sets_replace_flags.push(flag);
        }
    }

    // Standalone `!PREFACT` only. A `PREMULT`-wrapped one is the self-exclusion
    // guard, not a suppression gate.
    let suppressed_by_flag = parsed.iter().filter(|f| f.key == "!PREFACT").find_map(|f| prefact_flag(&f.value));

    let rendered = render_description(&parsed).unwrap_or_else(|e| panic!("line {line_number}: {e}"));

    // A `SOURCEPAGE` that is an upstream placeholder is not a citation, and
    // shipping it renders "p.xx" next to the trait on the Race Traits panel as
    // though it were a real page. `core_essentials`' two subrace files carry
    // one on every row -- `SOURCEPAGE:p.xx` on all 40 Tiefling rows and
    // `SOURCEPAGE:xx` on all 24 Aasimar ones -- and none of the four books
    // this binary ingested before them carries any placeholder at all
    // (`grep -oh 'SOURCEPAGE:[^\t]*' <their four .lst files> | sort -u | grep -i x`
    // -> no output). Recorded as no page rather than as a wrong one; the row
    // still ships, with its name, prose and bonuses intact.
    let source_page = parsed
        .iter()
        .find(|f| f.key == "SOURCEPAGE")
        .map(|f| f.value.trim().to_string())
        .filter(|page| !is_placeholder_source_page(page));

    let raw_bonus_chains: Vec<RawBonusChain> = parsed
        .iter()
        .filter(|f| f.key == "BONUS")
        .map(|f| RawBonusChain {
            qualifiers: f.value.split('|').map(|q| q.trim().to_string()).filter(|q| !q.is_empty()).collect(),
        })
        .collect();

    // Everything except the BONUS chains (which have their own field), kept in
    // source order and verbatim -- this is what preserves PREMULT, ASPECT,
    // ABILITY, DEFINE, VISION and the rest for downstream resolvers.
    let raw_tokens: Vec<RawToken> = parsed
        .iter()
        .filter(|f| f.key != "BONUS")
        .map(|f| RawToken { key: f.key.clone(), value: f.value.clone() })
        .collect();

    Some(TraitRow {
        line_number,
        name,
        key,
        race_key,
        category: parsed.iter().find(|f| f.key == "CATEGORY").map(|f| f.value.clone()),
        type_tokens,
        is_racial_default,
        is_subrace_selector,
        suppressed_by_flag,
        sets_replace_flags,
        description: rendered.text,
        unresolved_desc_args: rendered.unresolved_args,
        source_page,
        raw_tokens,
        raw_bonus_chains,
    })
}

/// [`pi_screening::declared_product_identity`] over one parsed row's preserved
/// tokens.
///
/// Reads `raw_tokens` rather than re-parsing the line, because `raw_tokens` is
/// what actually ships: if a token were ever dropped on the way into a record,
/// screening the line would report a declaration the shipped file does not
/// carry, and the corpus-level gate
/// (`tests/sd29_declared_product_identity_in_shipped_race_traits.rs`) reads the
/// shipped file. Both ends therefore read the same bytes.
fn declared_product_identity_of(row: &TraitRow) -> pi_screening::DeclaredProductIdentity {
    pi_screening::declared_product_identity(
        row.raw_tokens.iter().map(|token| (token.key.as_str(), token.value.as_str())),
    )
}

fn main() {
    // A book name selects one book; no argument ingests every declared book.
    // Both forms are deterministic and both rebuild whatever they write, so a
    // narrowed run can never leave a stale record from a wider one behind.
    let requested: Option<String> = std::env::args().nth(1);
    let selected: Vec<&BookSource> = match requested.as_deref() {
        None => BOOK_SOURCES.iter().collect(),
        Some(name) => {
            let matched: Vec<&BookSource> =
                BOOK_SOURCES.iter().filter(|b| b.corpus_book == name).collect();
            if matched.is_empty() {
                let known: Vec<&str> = BOOK_SOURCES.iter().map(|b| b.corpus_book).collect();
                panic!("unknown book {name:?}; this binary ingests {known:?}");
            }
            matched
        }
    };

    for book in selected {
        ingest_book(book);
    }
}

/// One parsed row plus the file it came from -- a book may declare its racial
/// traits across several `.lst` files and each record cites its own.
struct SourcedRow {
    row: TraitRow,
    lst_relative: &'static str,
    sha256: String,
}

fn ingest_book(book: &BookSource) {
    let BookSource { corpus_book, lst_relatives, subrace_globalvar_relatives, pcgen_book_relative } = *book;
    let data_root = pcgen_data_root();

    let out_root =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/corpus").join(corpus_book).join("race_trait");
    let ingested_at = ingested_at_now();
    let pcgen_book_dir = data_root.join(pcgen_book_relative);
    let wiring_index = WiringClassIndex::build(corpus_book, &pcgen_book_dir);
    let mut wiring_lines = wiring_index.lines();

    let in_scope: BTreeSet<&str> = IN_SCOPE_RACES.into_iter().collect();

    let mut rows: Vec<SourcedRow> = Vec::new();
    let mut skipped: BTreeMap<String, usize> = BTreeMap::new();
    // Rows refused outright because the corpus declares their NAME to be
    // Product Identity. Reported, never silent: a row that vanishes without a
    // line in the receipt is indistinguishable from an ingest bug.
    let mut pi_dropped: Vec<String> = Vec::new();
    let mut real_lines = 0usize;
    let mut source_shas: Vec<(&'static str, String)> = Vec::new();

    for lst_relative in lst_relatives {
        let lst_path = data_root.join(lst_relative);
        let bytes = fs::read(&lst_path)
            .unwrap_or_else(|e| panic!("failed to read the {corpus_book} racial-ability corpus {lst_path:?}: {e}"));
        let sha256 = sha256_hex(&bytes);
        source_shas.push((lst_relative, sha256.clone()));
        let text = String::from_utf8_lossy(&bytes).to_string();
        for (idx, line) in text.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            real_lines += 1;
            let Some(row) = parse_row((idx + 1) as u32, line) else { continue };
            // PCGen's own per-record Product Identity declaration, read before
            // the scope filter so a dropped row is reported even for a race
            // this book does not model. A NAME cannot be redacted -- it is the
            // record's identity on every screen and half of its key -- so a row
            // declaring `NAMEISPI:YES` is DROPPED, never screened. Same ruling
            // the monster lane reached for Inner Sea World Guide's five
            // `NAMEISPI:YES` monster rows (`decisions.md §50`), applied to the
            // kind that lane reported it against.
            if declared_product_identity_of(&row).name {
                pi_dropped.push(format!("{lst_relative}:{} {}", row.line_number, row.key));
                continue;
            }
            if in_scope.contains(row.race_key.as_str()) {
                rows.push(SourcedRow { row, lst_relative, sha256: sha256.clone() });
            } else {
                *skipped.entry(row.race_key.clone()).or_default() += 1;
            }
        }
    }

    // The heritage-selector half of the swap, read from the book's
    // `_abilities_globalvar_subrace.lst` files. See `subrace_grants`.
    let mut grants: BTreeMap<String, Vec<SubraceGrant>> = BTreeMap::new();
    for globalvar_relative in subrace_globalvar_relatives {
        let path = data_root.join(globalvar_relative);
        let bytes = fs::read(&path)
            .unwrap_or_else(|e| panic!("failed to read the {corpus_book} subrace globalvar file {path:?}: {e}"));
        let text = String::from_utf8_lossy(&bytes).to_string();
        for race in IN_SCOPE_RACES {
            for (selector, found) in subrace_grants(&text, race) {
                grants.entry(selector).or_default().extend(found);
            }
        }
    }

    let mut heritage_report: Vec<String> = Vec::new();
    for sourced in rows.iter_mut() {
        if !sourced.row.is_subrace_selector {
            continue;
        }
        let found = grants.get(&sourced.row.key).cloned().unwrap_or_default();
        // A selector nothing grants through is a browse-only stub by
        // construction: the player would pick a heritage and get none of its
        // content. Refuse to ship it rather than write a record that does
        // nothing (`decisions.md` 44.2).
        assert!(
            !found.is_empty(),
            "{}: heritage selector {:?} is granted no replacement trait by any \
             `_abilities_globalvar_subrace.lst` row; ingesting it would ship a \
             selectable record that changes nothing",
            sourced.lst_relative,
            sourced.row.key
        );
        let derived: Vec<String> = {
            let mut v: Vec<String> = found.iter().map(|g| g.replaces_flag.clone()).collect();
            v.sort();
            v.dedup();
            v
        };
        // Aasimar's six selectors state the same flags on their own row.
        // Where both sources speak they must agree exactly; that agreement is
        // what licenses reading the globalvar file for Tiefling's ten, which
        // carry no `FACT:` token at all.
        if !sourced.row.sets_replace_flags.is_empty() {
            let mut declared = sourced.row.sets_replace_flags.clone();
            declared.sort();
            declared.dedup();
            assert_eq!(
                declared, derived,
                "{}: heritage selector {:?} declares replace-flags {declared:?} on its own row but its \
                 `_abilities_globalvar_subrace.lst` block derives {derived:?}; the two sources contradict",
                sourced.lst_relative, sourced.row.key
            );
        }
        heritage_report.push(format!(
            "{} -> replaces {:?}, grants {:?}",
            sourced.row.key,
            derived,
            found.iter().map(|g| g.granted_trait_key.as_str()).collect::<Vec<_>>()
        ));
        sourced.row.sets_replace_flags = derived;
        // The grant link itself, in the shape `race_resolver` already reads
        // (`RaceTraitRecord::automatic_trait_grants` ->
        // `link_automatic_grants`): the selector names its replacement rows
        // outright, so selecting it brings them in and nothing new is needed
        // on the engine side.
        for grant in &found {
            // The `PREVAREQ:<flag>,0` qualifier is carried through verbatim,
            // not dropped as noise. It is the corpus's own statement of the
            // heritage's mutual exclusion -- this heritage grants its
            // replacement only while the standard trait it displaces has not
            // already been displaced by a different heritage -- and
            // `race_trait_picker::exclusion_guard_flags` reads exactly that.
            // Without it two heritages of one race could both be ticked and
            // the character would collect both sets of ability-score bonuses.
            // `race_resolver::automatic_grant_targets` stops at the first
            // `PRE` part, so the grant link itself is unaffected.
            let value = format!(
                "{} Racial Trait|AUTOMATIC|{}|PREVAREQ:{},0",
                sourced.row.race_key, grant.granted_trait_key, grant.replaces_flag
            );
            if !sourced.row.raw_tokens.iter().any(|t| t.key == "ABILITY" && t.value == value) {
                sourced.row.raw_tokens.push(RawToken { key: "ABILITY".to_string(), value });
            }
        }
    }

    // A stale record from a previous run with different scope would be
    // indistinguishable from a fresh one, so this binary's own output is
    // rebuilt -- scoped to exactly the race slugs in `IN_SCOPE_RACES`,
    // not the whole `out_root` directory.
    //
    // **Why scoped, not whole-directory, as of SD-31-E6-F4-002 (2026-08-16):**
    // `advanced_race_guide` is now ALSO written by `ingest_races.rs`, which
    // filed 6 new races' (Catfolk/Kitsune/Ratfolk/Strix/Suli/Wayang)
    // chassis + standard-tier traits into this same `out_root` for the
    // first time.
    //
    // **Widened again, SD-31-E6-F4-003 (2026-08-16): those same 6 races now
    // carry real ARG alternate-trait content this binary DOES ingest**
    // (`arg_abilities_race.lst`'s `###Block: Alternate Racial Traits` rows
    // for Catfolk/Ratfolk/Kitsune/Strix/Suli/Wayang -- confirmed real,
    // not `.MOD` bookkeeping, by direct inspection of the pinned oracle),
    // so their race slugs are now IN `IN_SCOPE_RACES` too and a per-slug
    // `remove_dir_all` on `catfolk/` etc. would delete `ingest_races.rs`'s
    // already-shipped standard-tier files in the SAME directory every time
    // this binary runs -- the mutual-destruction hazard the SD-31-E6-F4-002
    // comment above only avoided by having disjoint race sets, which is no
    // longer true. The two binaries' records are still disjoint by
    // *content*, though: `ingest_races.rs` writes only
    // `is_racial_default: true` (chassis/standard) records and this binary
    // writes only `is_racial_default: false` ones -- confirmed by scanning
    // every file either binary has ever shipped (zero counter-examples in
    // either direction). [`clear_own_alternate_trait_files`] clears by that
    // real, already-shipped field instead of by directory, so each binary's
    // rebuild only ever removes files it could itself have written.
    if out_root.exists() {
        for race_name in IN_SCOPE_RACES {
            let race_dir = out_root.join(slugify(race_name));
            if race_dir.exists() {
                clear_own_alternate_trait_files(&race_dir);
            }
        }
    }

    let mut written = 0usize;
    let mut flags_total = 0usize;
    let mut per_race: BTreeMap<String, usize> = BTreeMap::new();
    let mut per_race_flags: BTreeMap<String, usize> = BTreeMap::new();
    let mut defaults_seen: Vec<String> = Vec::new();
    let mut gated_alternates: Vec<(String, String)> = Vec::new();
    let mut written_paths: BTreeSet<PathBuf> = BTreeSet::new();
    let mut unresolved_desc_args: Vec<String> = Vec::new();
    let mut leaks: Vec<String> = Vec::new();
    let mut pi_declared_descriptions = 0usize;

    for sourced in &rows {
        let row = &sourced.row;
        let lst_relative = sourced.lst_relative;
        let sha256 = &sourced.sha256;
        let lst_basename = lst_relative.rsplit('/').next().unwrap_or(lst_relative);
        // Production guard: a description carrying PCGen syntax fails the
        // run loudly rather than reaching a screen. `%1` on the Race
        // Traits panel is the defect this exists to make impossible.
        if let Some(desc) = row.description.as_deref()
            && let Some(leak) = leaked_pcgen_syntax(desc)
        {
            leaks.push(format!("{lst_relative}:{}: {} would ship a {leak}: {desc}", row.line_number, row.key));
        }
        for arg in &row.unresolved_desc_args {
            unresolved_desc_args
                .push(format!("{} -> DESC arg {arg:?} is not a same-row literal (dropped, not guessed)", row.key));
        }

        if row.is_racial_default {
            defaults_seen.push(row.key.clone());
        }
        if let Some(flag) = &row.suppressed_by_flag {
            gated_alternates.push((row.key.clone(), flag.clone()));
        }
        flags_total += row.sets_replace_flags.len();
        *per_race.entry(row.race_key.clone()).or_default() += 1;
        *per_race_flags.entry(row.race_key.clone()).or_default() += row.sets_replace_flags.len();

        let (wiring_class, wiring_class_signals) = wiring_index.wiring_class_for(
            &mut wiring_lines,
            lst_basename,
            row.line_number,
            &row.key,
            &row.key,
        );
        // Closes a documented open finding (`data/corpus/advanced_race_guide/
        // LICENSE.json`'s own note): this binary previously classified every
        // record `OGL` WITHOUT running the term scan -- correct by luck
        // (re-verified externally on 2026-07-31 with 0 hits across all 156
        // records) but not by construction. It now runs the same screen
        // every other unscreened writer in this cycle gained.
        // ...and, since 2026-08-12, the row's own declaration takes precedence
        // over that term scan. `DESCISPI:YES` is PCGen stating that this
        // description is Product Identity; 8 of the 26 shipped race_trait rows
        // carrying it named nothing the 55-term list knows (`Kodar Mountains`,
        // `Earthfall`, `Ekujae`, `Gogpodda`, `Omesta`, `Droskar`, `Abaddon`,
        // `Inner Sea`) and were published verbatim. The two screens are a
        // union: an undeclared row is still scanned.
        let declared = declared_product_identity_of(row);
        let (license, pi_field, pi_marker, stored_desc) = pi_screening::classify_optional_field_declared(
            "description",
            row.description.as_deref(),
            declared.description,
        );
        if declared.description {
            pi_declared_descriptions += 1;
        }
        let record = CorpusRecordV1 {
            population: Population::InScope,
            completeness: Completeness::Full,
            ingested_at: ingested_at.clone(),
            data: RaceTraitCacheData {
                key: row.key.clone(),
                name: row.name.clone(),
                race_key: row.race_key.clone(),
                category: row.category.clone(),
                type_tokens: row.type_tokens.clone(),
                is_racial_default: row.is_racial_default,
                suppressed_by_flag: row.suppressed_by_flag.clone(),
                sets_replace_flags: row.sets_replace_flags.clone(),
                description: stored_desc,
                source_page: row.source_page.clone(),
                raw_tokens: row.raw_tokens.clone(),
                raw_bonus_chains: row.raw_bonus_chains.clone(),
            },
            source: CorpusSource::LstToken {
                path: lst_relative.to_string(),
                sha256: sha256.clone(),
                line: row.line_number,
                record_key: row.key.clone(),
            },
            license: Some(license),
            pi_field,
            pi_marker,
            wiring_class,
            wiring_class_signals,
        };

        let path = out_root.join(slugify(&row.race_key)).join(format!("{}.json", slugify(&row.key)));
        if !written_paths.insert(path.clone()) {
            panic!("slug collision: two {corpus_book} racial traits both resolve to {path:?}");
        }
        fs::create_dir_all(path.parent().expect("record path has a parent")).expect("failed to create output dir");
        let json = serde_json::to_string_pretty(&record).expect("record must serialize");
        fs::write(&path, json + "\n").unwrap_or_else(|e| panic!("failed to write {path:?}: {e}"));
        written += 1;
    }

    let skipped_total: usize = skipped.values().sum();
    println!("{corpus_book} alternate racial traits");
    for (relative, sha) in &source_shas {
        println!("  source                        : {relative}");
        println!("    sha256                      : {sha}");
    }
    println!("  real (non-comment) lines      : {real_lines}");
    println!("  records emitted               : {written}");
    println!("  distinct races covered        : {}", per_race.len());
    println!("  replace-flags captured        : {flags_total}");
    println!("  skipped, out-of-scope races   : {skipped_total} across {} races", skipped.len());
    println!("  dropped, NAMEISPI:YES         : {}", pi_dropped.len());
    for line in &pi_dropped {
        println!("    {line}");
    }
    println!("  descriptions redacted by DESCISPI:YES : {pi_declared_descriptions}");
    println!("  ingested_at                   : {ingested_at}");
    println!("\n  per in-scope race (records / replace-flags):");
    for (race, n) in &per_race {
        println!("    {race:<12} {n:>4} / {:>4}", per_race_flags.get(race).copied().unwrap_or(0));
    }
    println!("\n  skipped per out-of-scope race:");
    for (race, n) in &skipped {
        println!("    {race:<12} {n:>4}");
    }
    println!("\n  rows carrying a \"<Race> Racial Default\" TYPE marker : {}", defaults_seen.len());
    for k in &defaults_seen {
        println!("    {k}");
    }
    println!("  in-scope alternates gated by a standalone !PREFACT   : {}", gated_alternates.len());
    for (k, flag) in &gated_alternates {
        println!("    {k} <- {flag}");
    }

    println!("\n  heritage selectors, with the swap read from the subrace globalvar file : {}", heritage_report.len());
    for line in &heritage_report {
        println!("    {line}");
    }

    println!("\n  DESC args that are not same-row literals (dropped, never guessed) : {}", unresolved_desc_args.len());
    for line in &unresolved_desc_args {
        println!("    {line}");
    }

    assert_eq!(written, rows.len(), "every in-scope row must produce exactly one record");
    // Scoped to this binary's own `IN_SCOPE_RACES` race slugs, not a whole-
    // directory walk -- for the same reason the clearing step above is
    // scoped (SD-31-E6-F4-002, 2026-08-16). `advanced_race_guide/
    // race_trait/` now also holds `ingest_races.rs`'s 6-race batch
    // (Catfolk/Kitsune/Ratfolk/Strix/Suli/Wayang), which a whole-directory
    // `count_json` would count as though this run had written them too --
    // `count_own_json` (SD-31-E6-F4-003) filters those out by the same
    // ownership test the scoped clear above uses, so the two figures being
    // compared are both "this binary's own records" on both sides.
    // A given book only ever touches a subset of the 30 in-scope races
    // (e.g. `monster_codex` writes for a handful, not all 30), so a race
    // this book's rows never mentioned has no subdirectory here at all --
    // that is 0 records, not a missing-directory error.
    let on_disk: usize = IN_SCOPE_RACES
        .iter()
        .map(|race_name| {
            let race_dir = out_root.join(slugify(race_name));
            if race_dir.exists() { count_own_json(&race_dir) } else { 0 }
        })
        .sum();
    assert_eq!(on_disk, written, "records written to disk must match records emitted");

    // Last, and fatal: nothing PCGen-shaped may survive into a served
    // description.
    if !leaks.is_empty() {
        for line in &leaks {
            eprintln!("LEAK  {line}");
        }
        panic!("{} description(s) carry PCGen syntax; refusing to ship them", leaks.len());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /// `arg_abilities_race.lst:38` verbatim except for a shortened first
    /// `DESC:` (tokens joined with single tabs; the corpus pads with tab
    /// runs, which `split_fields` discards). Chosen
    /// because it exercises every branch at once: two `TYPE:` fields, a
    /// `PREMULT`-wrapped `!PREFACT` self-exclusion guard, two `DESC:` fields
    /// (one carrying a `|!PREABILITY` condition), and a `FACT:...|True`
    /// replace-flag setting.
    const MAGIC_RESISTANT: &str = concat!(
        "Magic Resistant\t",
        "KEY:Dwarf ~ Magic Resistant\t",
        "CATEGORY:Special Ability\t",
        "TYPE:RacialTraits.Dwarf Racial Trait.SpecialQuality.Special Quality\t",
        "TYPE:Replaces Dwarf Hardy\t",
        "PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Magic Resistant],",
        "[!PREFACT:1,ABILITIES,Dwarf_ReplaceHardy=true]\t",
        "DESC:Some of the older dwarven clans are particularly resistant to magic.\t",
        "DESC:This racial trait replaces hardy.|!PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Magic Resistant\t",
        "SR:5+(TL-HD)\t",
        "COST:0\t",
        "SOURCEPAGE:p.12\t",
        "FACT:Dwarf_ReplaceHardy|True",
    );

    /// `arg_abilities_race.lst:43` (`Dwarf ~ Saltbeard`), verbatim except for
    /// a shortened first `DESC:` and its 3 `ASPECT:` fields elided. It sets
    /// **4** replace flags while its `PREMULT` guard names only 3 of them —
    /// the precise case that proves `sets_replace_flags` is read off the
    /// `FACT:` fields and not off the `PREMULT` clause. It also carries 3
    /// `BONUS:` chains.
    const SALTBEARD: &str = concat!(
        "Saltbeard\t",
        "KEY:Dwarf ~ Saltbeard\t",
        "CATEGORY:Special Ability\t",
        "TYPE:RacialTraits.Dwarf Racial Trait.SpecialAttack.Special Attack.Defensive\t",
        "PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Saltbeard],",
        "[!PREFACT:1,ABILITIES,Dwarf_ReplaceDefensiveTraining=true,",
        "Dwarf_ReplaceHatred=true,Dwarf_ReplaceStonecunning=true]\t",
        "DEFINE:RacialDefensiveTrainingBonus|0\t",
        "DESC:Dwarves occasionally found iron cities along rugged seacoasts.\t",
        "DESC:This racial trait replaces defensive training, hatred, and stonecunning.",
        "|!PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Saltbeard\t",
        "ABILITY:Dwarf Racial Trait|AUTOMATIC|Saltbeard ~ Dwarf ~ Greed\t",
        "BONUS:SITUATION|Survival=while at sea|2|TYPE=Racial\t",
        "BONUS:SKILL|Profession (Sailor)|2|TYPE=Racial\t",
        "BONUS:VAR|RacialDefensiveTrainingBonus|2\t",
        "COST:0\t",
        "SOURCEPAGE:p.12\t",
        "FACT:Dwarf_ReplaceDefensiveTraining|True\t",
        "FACT:Dwarf_ReplaceHatred|True\t",
        "FACT:Dwarf_ReplaceStonecunning|True\t",
        "FACT:Dwarf_ReplaceGreed|True",
    );

    /// `core_essentials/races/dwarf/dwarf_abilities_race.lst:23` verbatim
    /// (shortened `DESC`) — a *standard* trait, i.e. the other end of the
    /// protocol. It is the shape that legitimately populates
    /// `suppressed_by_flag`, and it self-identifies as a racial default.
    const STANDARD_GREED: &str = concat!(
        "Greed\t",
        "KEY:Dwarf ~ Greed\t",
        "CATEGORY:Special Ability\t",
        "TYPE:RacialTraits.Dwarf Racial Trait.Dwarf Racial Default.SpecialQuality\t",
        "!PREFACT:1,ABILITIES,Dwarf_ReplaceGreed=True\t",
        "DESC:Dwarves receive a +2 racial bonus on Appraise skill checks.",
    );

    #[test]
    fn alternate_row_sets_its_replace_flag_and_is_not_suppressed_by_its_own_guard() {
        let row = parse_row(38, MAGIC_RESISTANT).expect("row is a racial trait");
        assert_eq!(row.key, "Dwarf ~ Magic Resistant");
        assert_eq!(row.name, "Magic Resistant");
        assert_eq!(row.race_key, "Dwarf");
        assert_eq!(row.category.as_deref(), Some("Special Ability"));
        assert_eq!(row.source_page.as_deref(), Some("p.12"));
        assert_eq!(row.sets_replace_flags, vec!["Dwarf_ReplaceHardy"]);
        // The `PREMULT`-wrapped `!PREFACT` is a self-exclusion guard, not a
        // suppression gate, so it must NOT be laundered into this field.
        assert_eq!(row.suppressed_by_flag, None);
        assert!(!row.is_racial_default);
        // Both TYPE fields contribute, split on `.`.
        assert!(row.type_tokens.contains(&"Dwarf Racial Trait".to_string()));
        assert!(row.type_tokens.contains(&"Replaces Dwarf Hardy".to_string()));
        // ...but the guard is still preserved verbatim, so nothing is lost.
        let premult = row.raw_tokens.iter().find(|t| t.key == "PREMULT").expect("PREMULT preserved");
        assert!(premult.value.contains("!PREFACT:1,ABILITIES,Dwarf_ReplaceHardy=true"));
        // DESC prose is joined; the `|!PREABILITY` condition is stripped from
        // the prose but kept whole in raw_tokens.
        let desc = row.description.expect("description");
        assert!(desc.ends_with("This racial trait replaces hardy."), "got {desc:?}");
        assert!(!desc.contains("PREABILITY"));
        assert!(row.raw_tokens.iter().any(|t| t.key == "DESC" && t.value.contains("!PREABILITY")));
    }

    #[test]
    fn alternate_row_captures_every_flag_it_sets_not_just_the_guarded_ones() {
        let row = parse_row(43, SALTBEARD).expect("row is a racial trait");
        assert_eq!(row.race_key, "Dwarf");
        // 4 set; the PREMULT guard names only the first 3.
        assert_eq!(
            row.sets_replace_flags,
            vec![
                "Dwarf_ReplaceDefensiveTraining",
                "Dwarf_ReplaceHatred",
                "Dwarf_ReplaceStonecunning",
                "Dwarf_ReplaceGreed",
            ]
        );
        assert_eq!(row.suppressed_by_flag, None);
        assert_eq!(row.raw_bonus_chains.len(), 3);
        assert_eq!(row.raw_bonus_chains[0].qualifiers, vec!["SITUATION", "Survival=while at sea", "2", "TYPE=Racial"]);
        assert_eq!(row.raw_bonus_chains[2].qualifiers, vec!["VAR", "RacialDefensiveTrainingBonus", "2"]);
        // BONUS lives in its own field and is not duplicated into raw_tokens,
        // but every other field is preserved.
        assert!(row.raw_tokens.iter().all(|t| t.key != "BONUS"));
        assert!(row.raw_tokens.iter().any(|t| t.key == "DEFINE"));
        assert!(row.raw_tokens.iter().any(|t| t.key == "ABILITY"));
    }

    #[test]
    fn standard_row_populates_suppressed_by_flag_and_the_racial_default_marker() {
        let row = parse_row(23, STANDARD_GREED).expect("row is a racial trait");
        assert_eq!(row.race_key, "Dwarf");
        assert_eq!(row.suppressed_by_flag.as_deref(), Some("Dwarf_ReplaceGreed"));
        assert!(row.sets_replace_flags.is_empty());
        // Read off the corpus, never forced: this row really is a default.
        assert!(row.is_racial_default);
    }

    #[test]
    fn rows_without_a_racial_trait_type_are_not_racial_traits() {
        // `arg_abilities_race.lst:23` verbatim — a `.MOD` row from one of the
        // file's 37 `Racial Traits` blocks. It only re-stamps a SOURCEPAGE on
        // a trait `core_essentials` declares; it carries no TYPE at all.
        assert!(parse_row(23, "CATEGORY=Special Ability|Dwarf ~ Greed.MOD\t\t\tSOURCEPAGE:p.10").is_none());
        // `arg_abilities_race.lst:1323` (truncated) — a favored-class-bonus
        // row. TYPE is present, but names no `<Race> Racial Trait`.
        assert!(
            parse_row(
                1323,
                concat!(
                    "Bonus Acid and Earth Spell Damage\t\t\t",
                    "KEY:Favored Class Bonus ~ Acid and Earth Spell Damage\t\t\t",
                    "CATEGORY:Special Ability\t",
                    "TYPE:SpecialQuality.FavoredClassBonus.FavoredClassSorcerer",
                )
            )
            .is_none()
        );
    }

    /// A `.MOD` row updates a record declared elsewhere; it declares nothing
    /// and must never become one.
    ///
    /// **This is not hypothetical and it is not covered by the TYPE check.**
    /// ARG's and Monster Codex's `.MOD` rows carry no `TYPE:` at all, so
    /// `rows_without_a_racial_trait_type_are_not_racial_traits` rejected them
    /// for a different reason and this property was never exercised. Inner Sea
    /// Races is the counter-example: `isr_abilities_race.lst` has 618 `.MOD`
    /// rows, and 5 of them DO carry a `<Race> Racial Trait` TYPE. Those 5 name
    /// races this project does not model, so `IN_SCOPE_RACES` filtered them one
    /// step later and nothing wrong shipped — but that is luck, and the next
    /// book's `.MOD` row for a modelled race would have been written out as a
    /// new alternate racial trait.
    ///
    /// The 5 are `isr_abilities_race.lst:650-654`, all one record
    /// (`Geneiekin ~ Mostly Human.MOD`) re-typed for Ifrit, Oread, Sylph,
    /// Undine and Suli. Re-derived by splitting each row on tabs, requiring
    /// `.MOD` in field 0, and requiring a `TYPE:` component that *ends in*
    /// `" Racial Trait"` — a substring grep for `Racial Trait` over the same
    /// rows answers **6**, because it also matches a `.MOD` row whose own name
    /// is `Changeling ~ Hag Racial Trait`. Different predicate, different
    /// number; the one that matters here is the TYPE component, because that
    /// is what `parse_row` reads.
    #[test]
    fn a_mod_row_declares_nothing_even_when_it_carries_a_racial_trait_type() {
        let modded = concat!(
            "Dwarf ~ Greed.MOD\t",
            "CATEGORY:Special Ability\t",
            "TYPE:RacialTraits.Dwarf Racial Trait.SpecialQuality\t",
            "SOURCEPAGE:p.10",
        );
        // The TYPE check alone would have accepted this row: it names a
        // modelled race and it is well-formed. Only the `.MOD` guard rejects
        // it, which is the whole point of asserting it separately.
        assert!(is_mod_row(&split_fields(modded)));
        assert!(parse_row(23, modded).is_none(), "a .MOD row must never produce a record");

        // ...and `.MOD` inside a token VALUE is not a mod row. PCGen writes
        // `var("STAT.3.MOD")` in formulas; only field 0 decides.
        let value_mod = concat!(
            "Sharp Senses\tKEY:Dwarf ~ Sharp Senses\tTYPE:Dwarf Racial Trait\t",
            "BONUS:SKILL|Perception|var(\"STAT.3.MOD\")",
        );
        assert!(!is_mod_row(&split_fields(value_mod)));
        assert!(parse_row(1, value_mod).is_some(), "a formula containing .MOD is not a mod row");
    }

    #[test]
    fn only_true_valued_replace_facts_count_as_settings() {
        // `FACT:` fields that are not replace flags, and replace flags set to
        // something other than True, are both excluded.
        let line = concat!(
            "X\tKEY:Dwarf ~ X\tCATEGORY:Special Ability\tTYPE:Dwarf Racial Trait\t",
            "FACT:Dwarf_ReplaceGreed|True\tFACT:BaseSize|M\tFACT:Dwarf_ReplaceHardy|False"
        );
        let row = parse_row(1, line).expect("row is a racial trait");
        assert_eq!(row.sets_replace_flags, vec!["Dwarf_ReplaceGreed"]);
    }

    #[test]
    fn prefact_flag_reads_only_abilities_scoped_true_assertions() {
        assert_eq!(prefact_flag("1,ABILITIES,Dwarf_ReplaceGreed=True").as_deref(), Some("Dwarf_ReplaceGreed"));
        assert_eq!(prefact_flag("1,ABILITIES,Dwarf_ReplaceGreed=false"), None);
        assert_eq!(prefact_flag("1,VAR,Something=True"), None);
        assert_eq!(prefact_flag("garbage"), None);
    }

    #[test]
    fn description_drops_pcgen_argument_and_prerequisite_segments() {
        let row = parse_row(1, "X\tKEY:Dwarf ~ X\tTYPE:Dwarf Racial Trait\tDESC:Plain text.")
            .expect("row is a racial trait");
        assert_eq!(row.description.as_deref(), Some("Plain text."));

        let row = parse_row(
            1,
            concat!(
                "X\tKEY:Dwarf ~ X\tTYPE:Dwarf Racial Trait\t",
                "DESC:Replaces greed.|!PREABILITY:1,CATEGORY=Special Ability,X"
            ),
        )
        .expect("row is a racial trait");
        assert_eq!(row.description.as_deref(), Some("Replaces greed."));
    }

    /// `arg_abilities_race.lst:716` (`Tengu ~ Carrion Sense`), verbatim
    /// except for the elided tab padding. The only PCGen syntax it carries
    /// is `%%`, which is the *literal-percent escape* — the player must see
    /// `50%`, one sign, and never `50%%`.
    const CARRION_SENSE: &str = concat!(
        "Carrion Sense\t",
        "KEY:Tengu ~ Carrion Sense\t",
        "CATEGORY:Special Ability\t",
        "TYPE:RacialTraits.Tengu Racial Trait.SpecialQuality.Special Quality.Sense\t",
        "PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Tengu ~ Carrion Sense],",
        "[!PREFACT:1,ABILITIES,Tengu_ReplaceGiftedLinguist=True]\t",
        "DESC:Tengus with this racial trait have a limited scent ability, which only ",
        "functions for corpses and badly wounded creatures (50%% or fewer hit points).\t",
        "DESC:This racial trait replaces gifted linguist.",
        "|!PREABILITY:1,CATEGORY=Special Ability,Tengu ~ Carrion Sense\t",
        "COST:0\t",
        "SOURCEPAGE:p.163\t",
        "FACT:Tengu_ReplaceGiftedLinguist|True",
    );

    /// `arg_abilities_race.lst:227` (`Halfling ~ Adaptable Luck`), verbatim
    /// except for a shortened first `DESC:` and the elided tab padding. It is
    /// the hardest row in the file: five `DESC:` segments, two of them gated
    /// on `PREVAR` comparisons over row-local variables, one `%N` argument
    /// that resolves off same-row literals (`Halfling_AdaptableLuck_Bonus`
    /// = `DEFINE 0` + `BONUS:VAR 2` = 2) and one that does **not**
    /// (`Halfling_AdaptableLuck_Bonus-1` is an expression, not a literal).
    const ADAPTABLE_LUCK: &str = concat!(
        "Adaptable Luck\t",
        "KEY:Halfling ~ Adaptable Luck\t",
        "CATEGORY:Special Ability\t",
        "TYPE:RacialTraits.Halfling Racial Trait.SpecialQuality.Special Quality\t",
        "PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Halfling ~ Adaptable Luck],",
        "[!PREFACT:1,ABILITIES,Halfling_ReplaceHalflingLuck=true]\t",
        "DEFINE:Halfling_AdaptableLuck_Times|0\t",
        "DEFINE:Halfling_AdaptableLuck_Bonus|0\t",
        "DESC:Some halflings have greater control over their innate luck.\t",
        "DESC:Three|PREVARLTEQ:Halfling_AdaptableLuck_Times,3\t",
        "DESC:%1|Halfling_AdaptableLuck_Times|PREVARGTEQ:Halfling_AdaptableLuck_Times,4\t",
        "DESC:times per day, a halfling can gain a +%1 luck bonus on an ability check, ",
        "attack roll, saving throw, or skill check. If halflings choose to use the ability ",
        "before they make the roll or check, they gain the full +%1 bonus; if they choose ",
        "to do so afterward, they only gain a +%2 bonus. Using adaptive luck in this way is ",
        "not an action.|Halfling_AdaptableLuck_Bonus|Halfling_AdaptableLuck_Bonus-1\t",
        "DESC:This racial trait replaces halfling luck.",
        "|!PREABILITY:1,CATEGORY=Special Ability,Halfling ~ Adaptable Luck\t",
        "BONUS:VAR|Halfling_AdaptableLuck_Times|3\t",
        "BONUS:VAR|Halfling_AdaptableLuck_Bonus|2\t",
        "COST:0\t",
        "SOURCEPAGE:p.21\t",
        "FACT:Halfling_ReplaceHalflingLuck|True",
    );

    #[test]
    fn literal_percent_escape_renders_as_a_single_percent_sign() {
        let row = parse_row(716, CARRION_SENSE).expect("row is a racial trait");
        let desc = row.description.expect("description");
        assert!(desc.contains("(50% or fewer hit points)"), "got {desc:?}");
        assert!(!desc.contains("%%"), "the escape must not survive: {desc:?}");
        assert_eq!(leaked_pcgen_syntax(&desc), None);
    }

    #[test]
    fn adaptable_luck_resolves_what_the_row_states_and_drops_only_what_it_does_not() {
        let row = parse_row(227, ADAPTABLE_LUCK).expect("row is a racial trait");
        let desc = row.description.expect("description");
        assert_eq!(
            desc,
            concat!(
                "Some halflings have greater control over their innate luck. ",
                "Three times per day, a halfling can gain a +2 luck bonus on an ability check, ",
                "attack roll, saving throw, or skill check. If halflings choose to use the ability ",
                "before they make the roll or check, they gain the full +2 bonus; if they choose ",
                "to do so afterward, they only gain a bonus. Using adaptive luck in this way is ",
                "not an action. This racial trait replaces halfling luck.",
            )
        );
        // The `%1`-only segment is gated on `Times >= 4`; the row's own
        // literals make Times 3, so that segment is dropped whole and the
        // "Three" segment (`Times <= 3`) is what survives.
        assert!(!desc.contains("Three 3 times"), "gates must not both fire: {desc:?}");
        assert_eq!(leaked_pcgen_syntax(&desc), None);
        // The one argument that is an expression rather than a literal is
        // reported, never guessed.
        assert_eq!(row.unresolved_desc_args, vec!["Halfling_AdaptableLuck_Bonus-1"]);
    }

    #[test]
    fn leaked_pcgen_syntax_names_every_shape_that_must_never_reach_a_player() {
        assert_eq!(leaked_pcgen_syntax("Clean prose with 50% of something."), None);
        assert_eq!(leaked_pcgen_syntax("A +%1 bonus."), Some("unsubstituted '%N' argument reference"));
        assert_eq!(leaked_pcgen_syntax("reduced by 20%%."), Some("unescaped '%%' literal-percent escape"));
        assert_eq!(leaked_pcgen_syntax("Text.|Some_Var"), Some("raw '|' argument tail"));
    }

    /// The property the player actually experiences: nothing PCGen-shaped
    /// survives into a served description.
    ///
    /// **This used to load one hardcoded book root** — `advanced_race_guide` —
    /// while [`BOOK_SOURCES`] had grown to three. That is the identical
    /// stale-hardcoded-roots defect SD-29 `decisions.md §44.2` and `§44.5`
    /// found in `race_resolver`'s test module and in two integration tests: a
    /// test whose stated job is "no committed record leaks" that cannot see
    /// two thirds of the committed records, and that stays green through any
    /// number of new books precisely because it cannot see them. It now
    /// derives its roots from `BOOK_SOURCES`, so adding a book to that table
    /// automatically widens this guard instead of silently escaping it.
    #[test]
    fn no_committed_trait_description_leaks_pcgen_syntax_in_any_declared_book() {
        use codex::rules_core::shape_b_v1::CorpusRecordV1;

        // Per-book expected counts, re-derived on disk by this test itself.
        // Stated as a table so a book whose ingest silently stops writing
        // fails here by name rather than by a total that still adds up.
        // ARG 156->201 and Inner Sea Races 71->82: SD-31 Epic 1-F2
        // (2026-08-15) widened `IN_SCOPE_RACES` from 18 to 24 (Bestiary 2's
        // 6 non-heritage races), so both books' alternate-trait rows for
        // those 6 races now pass the in-scope filter for the first time.
        // ARG 201->259: SD-31-E6-F4-002 (2026-08-16) added `ingest_races.rs`'s
        // own 6-race chassis batch (Catfolk, Kitsune, Ratfolk, Strix, Suli,
        // Wayang) into this SAME book directory for the first time -- this
        // test deliberately walks the whole `advanced_race_guide/race_trait/`
        // tree (a real corpus-wide leak check, not an ownership-scoped
        // count), so it is supposed to see both binaries' output.
        // ARG 259->283: SD-31-E6-F4-003 (2026-08-16) widened `IN_SCOPE_RACES`
        // 24->30 (Catfolk/Kitsune/Ratfolk/Strix/Suli/Wayang), so THIS
        // binary's own real `arg_abilities_race.lst` alternate-trait rows
        // for those 6 races (Catfolk 6, Kitsune 2, Ratfolk 4, Strix 6,
        // Suli 5, Wayang 1 = 24) now pass the in-scope filter for the first
        // time too, alongside `ingest_races.rs`'s already-shipped 58
        // standard-tier records in the same directories. Re-derived on
        // disk, not transcribed: `find data/corpus/advanced_race_guide/
        // race_trait -name '*.json' | wc -l` -> 283; same for
        // `inner_sea_races` -> 82.
        // ARG 321->332: SD31-E6-F4-006 (2026-08-17) widened `IN_SCOPE_RACES`
        // 30->34 (Gillman/Nagaji/Vanara/Vishkanya) -- their real
        // `arg_abilities_race.lst` alternate-trait rows (Gillman 5:
        // Riverfolk/Slime Hunter/Throwback + 2 flag-granted replacement
        // rows, Nagaji 1: Hypnotic Gaze, Vanara 3: Tree Stranger/Whitecape +
        // 1 flag-granted replacement row, Vishkanya 2: Sensual/Subtle
        // Appearance = 11) now pass the in-scope filter for the first time.
        // Re-derived on disk: `find data/corpus/advanced_race_guide/
        // race_trait -name '*.json' | wc -l` -> 332.
        let expected: BTreeMap<&str, usize> =
            [
                ("advanced_race_guide", 332usize),
                ("monster_codex", 5),
                ("inner_sea_races", 82),
                ("horror_adventures", 43),
                // Core Essentials' Aasimar and Tiefling heritage traits
                // (race-trait lane round 4): 16 heritage selectors + the 48
                // replacement rows they grant, across the book's two subrace
                // files. Re-derived on disk rather than transcribed:
                // `find data/corpus/core_essentials/race_trait -name '*.json'
                // | wc -l` -> 64.
                ("core_essentials", 64),
            ]
                .into_iter()
                .collect();
        assert_eq!(
            expected.len(),
            BOOK_SOURCES.len(),
            "every book this binary writes must be counted here"
        );

        let mut total = 0usize;
        for book in BOOK_SOURCES {
            let trait_root =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/corpus").join(book.corpus_book).join("race_trait");
            let mut race_dirs: Vec<PathBuf> = fs::read_dir(&trait_root)
                .unwrap_or_else(|e| panic!("{trait_root:?} must exist for {}: {e}", book.corpus_book))
                .filter_map(Result::ok)
                .map(|e| e.path())
                .collect();
            race_dirs.sort();

            let mut checked = 0usize;
            let mut with_description = 0usize;
            for race_dir in race_dirs {
                let mut files: Vec<PathBuf> =
                    fs::read_dir(&race_dir).unwrap().filter_map(Result::ok).map(|e| e.path()).collect();
                files.sort();
                for path in files {
                    let record: CorpusRecordV1<RaceTraitCacheData> =
                        serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
                    checked += 1;
                    let Some(desc) = record.data.description.as_deref() else { continue };
                    with_description += 1;
                    assert_eq!(
                        leaked_pcgen_syntax(desc),
                        None,
                        "{path:?}: served description carries PCGen syntax: {desc}"
                    );
                    assert_eq!(desc.trim(), desc, "{path:?}: served description has stray edge whitespace");
                }
            }
            assert_eq!(
                checked,
                expected[book.corpus_book],
                "{} record count on disk",
                book.corpus_book
            );
            // Every record carries prose. A redacted one carries the PI marker
            // rather than nothing, so this holds for Inner Sea Races' 12 and
            // Core Essentials' 8 redactions too — which is the point of a
            // schema-preserving redaction and is worth asserting rather than
            // assuming.
            assert_eq!(
                with_description,
                checked,
                "{}: every record must carry a description",
                book.corpus_book
            );
            total += checked;
        }
        assert_eq!(
            total,
            526,
            "332 ARG (of which 96 are `ingest_races.rs`'s own standard-tier batches: \
             58 from Catfolk/Kitsune/Ratfolk/Strix/Suli/Wayang, SD-31-E6-F4-002, plus 38 \
             from Gillman/Nagaji/Vanara/Vishkanya, SD31-E6-F4-004; the remaining 236 are \
             this binary's own alternate-tier batches: 201 pre-existing + 24 for the first \
             6-race batch, SD-31-E6-F4-003, 2026-08-16, plus 11 for the second 4-race \
             follow-on batch, SD31-E6-F4-006, 2026-08-17) + \
             5 Monster Codex + 82 Inner Sea Races + \
             43 Horror Adventures + 64 Core \
             Essentials heritage records (ARG/ISR moved from 156/71 by SD-31 Epic 1-F2, \
             2026-08-15). Advanced Player's Guide was investigated (SD-31 Epic 6-F4,
             2026-08-15) and deliberately NOT added as a `BookSource` -- see this file's \
             `BOOK_SOURCES` doc comment: 49 of its 50 in-scope rows are already ingested, \
             byte-mechanically-identical, via `advanced_race_guide`'s own reprint of them. \
             This total sits alongside the per-book map above and must move with it; round \
             3 moved the map first and this pin caught the omission, round 4 did the same, \
             the companion lane hit it a third time in one cycle, batch four a fourth, and \
             SD31-E6-F4-004 a fifth, SD31-E6-F4-006 a sixth -- fixing one assertion reveals \
             the next one below it, which is the whole reason the test states both"
        );
    }

    #[test]
    fn race_directory_slugs_match_the_corpus_directory_convention() {
        assert_eq!(slugify("Half-Elf"), "half_elf");
        assert_eq!(slugify("Half-Orc"), "half_orc");
        assert_eq!(slugify("Svirfneblin"), "svirfneblin");
        assert_eq!(slugify("Dwarf ~ Ancient Enmity"), "dwarf_ancient_enmity");
    }

    #[test]
    fn in_scope_roster_is_exactly_the_34_races_sd31_e6_f4_006_names() {
        // Widened 18 -> 24 by SD-31 Epic 1-F2 (2026-08-15), 24 -> 30 by
        // SD-31-E6-F4-003 (2026-08-16), then 30 -> 34 by SD31-E6-F4-006
        // (2026-08-17: `ingest_races.rs`'s SD31-E6-F4-004 standard-tier
        // chassis batch already ingested Gillman/Nagaji/Vanara/Vishkanya's
        // `~ Ability Scores`/`~ Speed`/`~ Vision`/etc rows; this cycle widens
        // THIS binary's roster so their real ALTERNATE racial trait rows
        // (`arg_abilities_race.lst`'s `Riverfolk`/`Slime Hunter`/`Throwback`
        // for Gillman, `Hypnotic Gaze` for Nagaji, `Tree Stranger`/
        // `Whitecape` for Vanara, `Sensual`/`Subtle Appearance` for
        // Vishkanya) pass the in-scope filter for the first time too); see
        // this constant's own doc comment and `ingest_races.rs`'s matching
        // `IN_SCOPE_RACES` table.
        assert_eq!(IN_SCOPE_RACES.len(), 34);
        let unique: BTreeSet<&str> = IN_SCOPE_RACES.into_iter().collect();
        assert_eq!(unique.len(), 34, "roster must not repeat a race");
        // The 6 Bestiary 2 races Epic 1-F2 added must actually be present.
        for added in ["Fetchling", "Grippli", "Ifrit", "Oread", "Sylph", "Undine"] {
            assert!(unique.contains(added), "{added} is SD-31 Epic 1-F2's batch and must be in scope");
        }
        // The 6 ARG-native races SD-31-E6-F4-003 added must actually be present.
        for added in ["Catfolk", "Kitsune", "Ratfolk", "Strix", "Suli", "Wayang"] {
            assert!(unique.contains(added), "{added} is SD-31-E6-F4-003's batch and must be in scope");
        }
        // The 4 ARG-follow-on races this cycle added must actually be present.
        for added in ["Gillman", "Nagaji", "Vanara", "Vishkanya"] {
            assert!(unique.contains(added), "{added} is SD31-E6-F4-006's batch and must be in scope");
        }
        // Still-out-of-scope races (`decisions.md §25.3`'s original deferral,
        // minus the 16 these three batches moved into scope) must not have
        // crept in.
        for deferred in ["Dhampir", "Changeling", "Samsaran"] {
            assert!(!unique.contains(deferred), "{deferred} is still deferred and must not be in scope");
        }
    }
}
