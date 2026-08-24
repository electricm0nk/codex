//! Codegen for `rules_tables::equipment_gap_tables` — the corpus equipment
//! and equipment-modifier records that live in an **already-compiled** book
//! whose hand-authored per-book table does not hold them.
//!
//! # Why this binary exists
//!
//! `docs/work-inventory.json` classifies an `equipment`/`equipment_modifier`
//! unit as `not-ingested` when the book has a compiled rule set but
//! `equipment_resolver::equipment_catalog_rows()` holds no row matching the
//! record's `KEY:` (or, failing that, its display name). Those are real gaps
//! inside started books — not un-started books — and closing them needs no
//! new `RuleSetId`, no new corpus cache, and no new player surface: the
//! existing equipment catalog already renders every row the resolver chains.
//!
//! # What it does, and what it deliberately does not do
//!
//! It re-parses each named `.lst` with the **same record predicate**
//! `v06_work_inventory`'s own enumerator applies (skip comment rows, skip
//! ALL-CAPS directive rows, skip `CATEGORY=Internal|`/`CATEGORY:Internal`
//! bookkeeping rows, skip `.MOD` overlays, take a `.COPY=` row's variant name,
//! identity is `KEY:` when present else the display name) and emits only those
//! records the hand-authored tables do not already hold. It never invents a
//! value: `cost_gp`/`weight_lbs` are `None` whenever the corpus token is
//! absent or carries a PCGen formula this table does not evaluate, exactly as
//! every per-book table in `rules_tables/` already documents.
//!
//! Every generated table is screened through
//! `pi_table_sweep::screen_generated_table` **before** it is written, per the
//! provenance gate this bundle landed ahead of the content lanes; a hit is a
//! hard stop, never a filtered-out row.
//!
//! Run it with a local PCGen corpus checkout:
//!
//! ```text
//! PCGEN_CORPUS_ROOT="$HOME/workspace/repos/pcgen/data" \
//!   cargo run --locked --bin gen_equipment_gap_tables
//! ```

use std::collections::{BTreeSet, HashMap};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use codex::rules_core::codex_neutral_name::{neutral_key, neutral_name};
use codex::rules_core::equipment_resolver::{hand_authored_equipment_rows, EQUIPMENT_BOOK_ACG, EQUIPMENT_BOOK_APG, EQUIPMENT_BOOK_ARG, EQUIPMENT_BOOK_B1, EQUIPMENT_BOOK_CRB, EQUIPMENT_BOOK_UC, EQUIPMENT_BOOK_UE, EQUIPMENT_BOOK_UI, EQUIPMENT_BOOK_UM, EQUIPMENT_BOOK_UPSI, EQUIPMENT_BOOK_UW};
use codex::rules_core::pcgen_desc::{leaked_pcgen_syntax, render_pcgen_desc};
use codex::rules_core::pi_screening::{declared_product_identity, DeclaredProductIdentity, PI_BLACKLIST_TERMS};
use codex::rules_core::pi_table_sweep::screen_generated_table;
use codex::rules_core::shape_b_v1::REDACTED_PI_MARKER;

/// `§53.5`'s declared-PI reader, applied to the real corpus line at
/// `lst_path:line` (1-indexed). A local copy of `cache_gen::equipment_gap`'s
/// `pub(crate)` function of the same name (that visibility is scoped to the
/// library crate; this file is a separate binary crate and cannot reach a
/// `pub(crate)` item there) -- same logic, same public `pi_screening::
/// declared_product_identity` primitive underneath, so the two stay in
/// agreement by construction rather than by convention.
fn declared_pi_at(lst_path: &Path, line: u32) -> DeclaredProductIdentity {
    if line == 0 {
        return DeclaredProductIdentity::default();
    }
    let Ok(content) = std::fs::read_to_string(lst_path) else {
        return DeclaredProductIdentity::default();
    };
    let Some(row) = content.lines().nth((line - 1) as usize) else {
        return DeclaredProductIdentity::default();
    };
    let tokens: Vec<(&str, &str)> =
        row.split('\t').filter_map(|field| field.split_once(':')).collect();
    declared_product_identity(tokens)
}

/// `SD31-E6-F10-004`: a per-RECORD counterpart to `screen_generated_table`'s
/// whole-FILE blacklist hard stop, over the exact same term list (never
/// forked -- `PI_BLACKLIST_TERMS` imported directly, per this module's own
/// "one term list" principle). `declared_pi_at` only catches a row that
/// carries an explicit `NAMEISPI:`/`DESCISPI:` token; a blacklisted deity or
/// place name can appear in perfectly ordinary, undeclared free-text prose
/// (`inner_sea_gods`'s "Cloak Of The Night Sky": "...If Desna is the
/// wearer's patron..." carries no `DESCISPI:` token at all) -- exactly the
/// shape that blocked `inner_sea_gods`/`mythic_adventures`/
/// `inner_sea_combat`/`inner_sea_intrigue`/`book_of_the_damned_volume_2`
/// from this generator's prior batch (`OPEN-ISSUES.md` row 186). Returns the
/// FIRST matching term (for the receipt/log), or `None` for clean text.
/// **Excluding, never weakening**: `screen_generated_table`'s own whole-file
/// hard stop over the FINISHED table still runs unconditionally after this
/// -- a gap in this per-row screen still aborts the entire run rather than
/// shipping a leak, so this is a narrower, additive pre-filter, never a
/// substitute for the backstop.
fn blacklist_hit(text: &str) -> Option<&'static str> {
    PI_BLACKLIST_TERMS.iter().find(|term| text.contains(**term)).copied()
}

/// Outcome of screening one parsed record for Product Identity before it
/// may enter the compiled table. This IS the production screen -- `main()`'s
/// per-record loop calls it directly, and `blacklist_screen_tests` drives it
/// directly too, so a test asserting this function's behavior can never pass
/// while the real screen has been removed or bypassed (wave-12 adversarial
/// review CONFIRMED the prior test defined its own local re-implementation
/// instead, which survived deletion of the whole production screen).
///
/// **`decisions.md §24` (SD-32):** a name-PI row is no longer excluded from
/// the compiled table -- `main()`'s caller renames it (`neutral_name`/
/// `neutral_key`, coordinate-derived) rather than dropping it, because a
/// row this generator drops here can NEVER reach `data/corpus/` at all
/// (`cache_gen::equipment_gap::generate()` only ever iterates rows THIS
/// table already contains). `ScreenOutcome` therefore always keeps the
/// record now; `name_is_pi` tells the caller whether to rename it.
enum ScreenOutcome {
    Kept {
        record: ParsedRecord,
        /// `declared.name` OR a blacklist hit on `key`/`name` -- the
        /// caller renames the record under a Codex-generated neutral
        /// identity when this is `true` (`§24b`-1: never derived from the
        /// original name, only from coordinates the caller already has).
        name_is_pi: bool,
        description_pi_redacted: bool,
        description_blacklist_redacted: bool,
    },
}

fn screen_record(mut record: ParsedRecord, declared: DeclaredProductIdentity) -> ScreenOutcome {
    let mut description_pi_redacted = false;
    if declared.description {
        description_pi_redacted = true;
        record.description = Some(REDACTED_PI_MARKER.to_string());
    }
    let name_is_pi =
        declared.name || blacklist_hit(&record.name).is_some() || blacklist_hit(&record.key).is_some();
    let mut description_blacklist_redacted = false;
    if let Some(desc) = &record.description
        && blacklist_hit(desc).is_some()
    {
        description_blacklist_redacted = true;
        record.description = Some(REDACTED_PI_MARKER.to_string());
    }
    ScreenOutcome::Kept { record, name_is_pi, description_pi_redacted, description_blacklist_redacted }
}

// SD31-E6-F10-003: short codes for 13 further already-compiled books that
// carry `not-ingested` equipment/equipment_modifier residue but have no
// hand-authored `equipment_resolver::EQUIPMENT_BOOK_*` constant of their
// own (none of them has a per-book `equipment_tables` module at all --
// every one of their catalog rows will come from this gap lane, the same
// shape `EQUIPMENT_BOOK_UW`'s own doc comment already describes). Declared
// locally rather than added to `equipment_resolver.rs` -- that file is
// outside this card's file grant and nothing downstream needs these codes
// to be `pub` constants; a plain `&'static str` literal is exactly what
// `BookInput.code`/`EquipmentGapRow.book` already are.
const EQUIPMENT_BOOK_OA: &str = "OA";
const EQUIPMENT_BOOK_HA: &str = "HA";
const EQUIPMENT_BOOK_ISR: &str = "ISR";
const EQUIPMENT_BOOK_ISWG: &str = "ISWG";
const EQUIPMENT_BOOK_MC: &str = "MC";
const EQUIPMENT_BOOK_B2: &str = "B2";
const EQUIPMENT_BOOK_B3: &str = "B3";
const EQUIPMENT_BOOK_B4: &str = "B4";

// SD31-E6-F10-004: the 5 books `SD31-E6-F10-003` found already-compiled but
// deliberately left out of the prior batch (`OPEN-ISSUES.md` row 186) because
// their real corpus text hit `screen_generated_table`'s whole-file blacklist
// hard stop. Reachable now that a per-record `blacklist_hit` pre-filter
// excludes/redacts the individual offending rows instead of aborting the
// whole run -- the hard stop itself is unchanged and still runs over the
// finished table as the backstop.
const EQUIPMENT_BOOK_ISG: &str = "ISG";
const EQUIPMENT_BOOK_MYTHIC: &str = "MYTHIC";
const EQUIPMENT_BOOK_ISC: &str = "ISC";
const EQUIPMENT_BOOK_ISI: &str = "ISI";
const EQUIPMENT_BOOK_BOTD2: &str = "BOTD2";
// SD-32 T9 onboarding (card 11), `decisions.md §19` PI sign-off -- two more
// already-compiled books extended into this gap lane, same shape as the
// arms immediately above. Both are genuine new-content books (not
// previously in `equipment_book_slug_for`'s match at all), so a new
// `equipment_book_slug_for` arm is needed for each -- see that function's
// own edit note in `v06_work_inventory.rs`.
const EQUIPMENT_BOOK_ISTEM: &str = "ISTEM";
const EQUIPMENT_BOOK_ISM: &str = "ISM";
// SD-32 T9 residual (`decisions.md §20`): `adventurers_guide` had no
// `BOOK_INPUT` entry at all -- 115 `not-ingested` equipment units, the
// single largest un-covered `equipment`-kind population, re-derived
// against the pinned oracle. Same shape as the arms immediately above:
// a genuine new-content book, routed in `cache_gen::equipment_gap`'s
// `book_routing` alongside this constant.
const EQUIPMENT_BOOK_AG: &str = "AG";
// SD-32 `sd32-beginner-box-ingest`: `beginner_box` never had a `BOOK_INPUT`
// entry at all -- decisions.md §27b overturns the operator's earlier
// 2026-07-27 "redundant to other tomes, will not be brought in" disposition
// (`v06_work_inventory.rs`'s `out_of_scope` set carried that note; removed
// alongside this arm). §27b: "no 'unregistered book' exemption" and "the
// only admissible reason for a unit not to close is a hard impossibility --
// the source data does not exist, or licensing forbids shipping it". Both
// `bbox_equip_magic_items.lst` and `bbox_equip_arms_armor.lst` are present
// in the pinned oracle and carry ordinary OGC equipment mechanics, so
// neither applies. Same shape as `AG`/`ISTEM`/`ISM` above: a genuine
// new-content book, routed in `cache_gen::equipment_gap`'s `book_routing`
// alongside this constant.
const EQUIPMENT_BOOK_BB: &str = "BB";

/// Refuses to ship a description whose rendering the player would see as
/// broken PCGen syntax -- an unsubstituted `%N`/`%<KEYWORD>` reference or a
/// raw `|` argument tail `render_pcgen_desc`/`split_prose_and_args` could
/// not resolve for this row's shape (confirmed real: `IntItemBase`'s
/// `SPROP:` states 4 BARE (unnumbered) `%` placeholders followed by a
/// 4-argument pipe tail naming the `BONUS:VAR` keys each one substitutes,
/// a shape `max_arg_reference`'s numbered-reference detection does not
/// recognize -- caught live by `apps/desktop`'s own
/// `no_catalog_serves_a_description_carrying_raw_pcgen_syntax` test).
/// Never fabricates a fix; the SAME judgment call `v06_work_inventory.rs`'s
/// `corpus_json_description_leaks_pcgen_syntax` already makes for the
/// identical shape, applied here at the SOURCE so a broken description
/// never ships at all rather than being caught downstream.
///
/// **Checks the RENDERED text's own leak, never `dropped_args` alone**
/// (empirically confirmed, not assumed: `%CHOICE` with no `|` tail drops
/// cleanly to a readable sentence with `dropped_args: ["CHOICE"]` but
/// `leaked_pcgen_syntax: None` — this is the SAME shape the real desktop
/// equipment catalog already ships today; refusing on `dropped_args` alone
/// would have wrongly discarded 68 of the 69 `%`/`|`-carrying recovered
/// descriptions this cycle recovers, keeping only the 1 that genuinely
/// leaks). Matches `apps/desktop`'s own `no_catalog_serves_a_description_
/// carrying_raw_pcgen_syntax` check exactly (`leaked_pcgen_syntax` on the
/// rendered text, nothing else), so this refuses precisely what that test
/// would otherwise catch downstream — never more, never less.
fn safe_description(description: Option<String>) -> Option<String> {
    let description = description?;
    let rendered = render_pcgen_desc(&description);
    if leaked_pcgen_syntax(&rendered.text).is_some() {
        return None;
    }
    Some(description)
}

/// Where the generated table lands, relative to the crate root.
const OUTPUT_RELATIVE_PATH: &str = "src/rules_core/rules_tables/equipment_gap_tables.rs";

/// One book's gap-lane inputs: the `EQUIPMENT_BOOK_*` code the resolver files
/// its rows under, the `docs/work-inventory.json` book slug the classifier
/// keys on, and each `.lst` path relative to the corpus root.
struct BookInput {
    code: &'static str,
    slug: &'static str,
    files: &'static [&'static str],
}

/// Every book that carries at least one `not-ingested` equipment or
/// equipment-modifier unit, with the exact files those units come from.
/// Derived from `docs/work-inventory.json`'s own `source_file` field over the
/// `status == "not-ingested"` population — not guessed from a directory glob,
/// so a file with no gap is not re-parsed and cannot introduce a row nobody
/// asked for.
const BOOK_INPUTS: &[BookInput] = &[
    BookInput {
        code: EQUIPMENT_BOOK_CRB,
        slug: "core_rulebook",
        files: &["pathfinder/paizo/roleplaying_game/core_rulebook/cr_equipmods.lst"],
    },
    // `decisions.md §9` (`core_essentials` re-attribution, "re-attribute
    // first, drop the label second"): these 2 files physically live under
    // the shared `core_essentials` library `core_rulebook.pcc` includes
    // unconditionally, and an earlier draft of this table routed their 3
    // records to CRB on that basis ("CRB is that host" -- now corrected).
    // Both files' own uncommented `SOURCELONG:Bestiary`/`SOURCESHORT:B1`
    // header (verified 2026-08-17, not assumed) says otherwise: 100% of
    // each file's content is Bestiary, none is genuinely Core Rulebook, so
    // Decision 9's "re-attribute by the file's own SOURCELONG" rule routes
    // them to B1/bestiary instead. Confirmed harmless to CRB: neither file
    // ever supplied a genuinely-CRB record (`grep SOURCELONG` on both
    // files finds exactly one value, `Bestiary`, each).
    BookInput {
        code: EQUIPMENT_BOOK_B1,
        slug: "bestiary",
        files: &[
            "pathfinder/paizo/roleplaying_game/core_essentials/ce_equip_arms_armor.lst",
            "pathfinder/paizo/roleplaying_game/core_essentials/ce_equip_general.lst",
        ],
    },
    BookInput {
        code: EQUIPMENT_BOOK_APG,
        slug: "advanced_players_guide",
        files: &["pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_equipmods.lst"],
    },
    BookInput {
        code: EQUIPMENT_BOOK_ACG,
        slug: "advanced_class_guide",
        files: &[
            "pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_equipmods.lst",
            "pathfinder/paizo/roleplaying_game/advanced_class_guide/_pfs/pfs_acg_equip.lst",
        ],
    },
    BookInput {
        code: EQUIPMENT_BOOK_ARG,
        slug: "advanced_race_guide",
        files: &[
            "pathfinder/paizo/roleplaying_game/advanced_race_guide/arg_equipmods.lst",
            "pathfinder/paizo/roleplaying_game/advanced_race_guide/arg_equip_arms_armor.lst",
        ],
    },
    BookInput {
        code: EQUIPMENT_BOOK_UC,
        slug: "ultimate_combat",
        files: &["pathfinder/paizo/roleplaying_game/ultimate_combat/uc_equipmods.lst"],
    },
    BookInput {
        code: EQUIPMENT_BOOK_UI,
        slug: "ultimate_intrigue",
        files: &["pathfinder/paizo/roleplaying_game/ultimate_intrigue/ui_equipmods.lst"],
    },
    BookInput {
        code: EQUIPMENT_BOOK_UE,
        slug: "ultimate_equipment",
        files: &[
            "pathfinder/paizo/roleplaying_game/ultimate_equipment/ue_equip_general.lst",
            "pathfinder/paizo/roleplaying_game/ultimate_equipment/ue_equip_arms_armor.lst",
            "pathfinder/paizo/roleplaying_game/ultimate_equipment/ue_equip_magic_items.lst",
            "pathfinder/paizo/roleplaying_game/ultimate_equipment/ue_equipmods.lst",
        ],
    },
    BookInput {
        code: EQUIPMENT_BOOK_UPSI,
        slug: "ultimate_psionics",
        files: &["pathfinder/dreamscarred_press/ultimate_psionics/up_equipmods.lst"],
    },
    BookInput {
        code: EQUIPMENT_BOOK_UW,
        slug: "ultimate_wilderness",
        files: &[
            "pathfinder/paizo/roleplaying_game/ultimate_wilderness/uw_equip_general.lst",
            "pathfinder/paizo/roleplaying_game/ultimate_wilderness/uw_equip_arms_armor.lst",
            "pathfinder/paizo/roleplaying_game/ultimate_wilderness/uw_equip_magic_items.lst",
            "pathfinder/paizo/roleplaying_game/ultimate_wilderness/uw_equipmods.lst",
        ],
    },
    // --- SD31-E6-F10-003: 8 further already-compiled books, none of which
    // has a hand-authored `equipment_tables` module -- exactly the same
    // "compiled rule set, no per-book equipment table" shape as `UW` above,
    // confirmed against `v06_work_inventory.rs`'s own `COMPILED_RULE_SETS`
    // (each of these book slugs already has a `RuleSetId` compiled for its
    // `monster`/`race_trait`/`class_feature` kinds) and against
    // `docs/work-inventory.json`'s own `source_file` field per book (never
    // guessed from a directory glob). No new `RuleSetId`, no new corpus
    // cache, no new player surface -- the existing equipment catalog already
    // renders every row `equipment_gap_rows()` chains in.
    //
    // **5 further eligible books (`inner_sea_gods`, `mythic_adventures`,
    // `inner_sea_combat`, `inner_sea_intrigue`, `book_of_the_damned_volume_2`
    // -- ~394 more units, `OPEN-ISSUES.md` row 177) were tried and DELIBERATELY
    // EXCLUDED from this batch**, not silently dropped: the real, unmodified
    // corpus text hits `pi_table_sweep::screen_generated_table`'s blacklist
    // (deity/place proper nouns -- "Desna", "Erastil", "Lastwall", "Numeria",
    // "Kyonin", "Calistria", ... -- inside item names like "Altar of Desna"),
    // and that screen's own doc contract is a HARD STOP on the whole
    // generation run, never a per-row silent drop. Per the mandate's own PI
    // discipline this is the check working as designed, not a defect to route
    // around -- weakening it to a per-row skip so this batch could include
    // those 5 books would be exactly the kind of gate-loosening the mandate
    // forbids. Left for a dedicated PI-redaction pass.
    BookInput {
        code: EQUIPMENT_BOOK_OA,
        slug: "occult_adventures",
        files: &["pathfinder/paizo/roleplaying_game/occult_adventures/oa_equip.lst"],
    },
    BookInput {
        code: EQUIPMENT_BOOK_HA,
        slug: "horror_adventures",
        files: &[
            "pathfinder/paizo/roleplaying_game/horror_adventures/ha_equip_arms_armor.lst",
            "pathfinder/paizo/roleplaying_game/horror_adventures/ha_equip_general.lst",
            "pathfinder/paizo/roleplaying_game/horror_adventures/ha_equip_magic_items.lst",
            "pathfinder/paizo/roleplaying_game/horror_adventures/ha_equipmods.lst",
        ],
    },
    BookInput {
        code: EQUIPMENT_BOOK_ISR,
        slug: "inner_sea_races",
        files: &[
            "pathfinder/paizo/campaign_setting/inner_sea_races/isr_equip_arms_armor.lst",
            "pathfinder/paizo/campaign_setting/inner_sea_races/isr_equip_general.lst",
        ],
    },
    BookInput {
        code: EQUIPMENT_BOOK_ISWG,
        slug: "inner_sea_world_guide",
        files: &[
            "pathfinder/paizo/campaign_setting/inner_sea_world_guide/iswg_equip_arms_armor.lst",
            "pathfinder/paizo/campaign_setting/inner_sea_world_guide/iswg_equip_general.lst",
            "pathfinder/paizo/campaign_setting/inner_sea_world_guide/iswg_equip_magic_items.lst",
            "pathfinder/paizo/campaign_setting/inner_sea_world_guide/iswg_equipmods.lst",
        ],
    },
    BookInput {
        code: EQUIPMENT_BOOK_MC,
        slug: "monster_codex",
        files: &[
            "pathfinder/paizo/roleplaying_game/monster_codex/mc_equip_arms_armor.lst",
            "pathfinder/paizo/roleplaying_game/monster_codex/mc_equip_general.lst",
            "pathfinder/paizo/roleplaying_game/monster_codex/mc_equip_magic_items.lst",
            "pathfinder/paizo/roleplaying_game/monster_codex/mc_equipmods.lst",
        ],
    },
    BookInput {
        code: EQUIPMENT_BOOK_B2,
        slug: "bestiary_2",
        files: &[
            "pathfinder/paizo/roleplaying_game/bestiary_2/b2_equip_arms_armor.lst",
            "pathfinder/paizo/roleplaying_game/bestiary_2/b2_equip_general.lst",
            "pathfinder/paizo/roleplaying_game/bestiary_2/_pfs/pfs_b2_equip_arms_armor.lst",
            "pathfinder/paizo/roleplaying_game/bestiary_2/_pfs/pfs_b2_equip_general.lst",
        ],
    },
    BookInput {
        code: EQUIPMENT_BOOK_B3,
        slug: "bestiary_3",
        files: &[
            "pathfinder/paizo/roleplaying_game/bestiary_3/b3_equip_arms_armor.lst",
            "pathfinder/paizo/roleplaying_game/bestiary_3/b3_equipmods.lst",
            "pathfinder/paizo/roleplaying_game/bestiary_3/_pfs/pfs_b3_equip_arms_armor.lst",
            "pathfinder/paizo/roleplaying_game/bestiary_3/_pfs/pfs_b3_equipmods.lst",
        ],
    },
    BookInput {
        code: EQUIPMENT_BOOK_B4,
        slug: "bestiary_4",
        files: &[
            "pathfinder/paizo/roleplaying_game/bestiary_4/b4_equip_arms_armor.lst",
            "pathfinder/paizo/roleplaying_game/bestiary_4/b4_equip_magic_items.lst",
            "pathfinder/paizo/roleplaying_game/bestiary_4/b4_equipmods.lst",
        ],
    },
    // --- SD31-E6-F10-004: the 5 books `SD31-E6-F10-003` deliberately left
    // out (`OPEN-ISSUES.md` row 186) because their real, unmodified corpus
    // text hit `screen_generated_table`'s whole-file blacklist hard stop
    // (deity/place proper nouns, e.g. "Desna" inside "Cloak Of The Night
    // Sky"'s undeclared `DESC:` prose). Reachable now that a per-record
    // `blacklist_hit` pre-filter (below) excludes/redacts only the
    // individual offending rows -- the whole-file hard stop is unchanged
    // and still runs over the finished table as the backstop; each of these
    // 5 already has a compiled `RuleSetId` (`v06_work_inventory.rs`'s
    // `COMPILED_RULE_SETS`: `Isg`, `Mythic`, `Isc`, `Isi`, `Botd2`),
    // confirmed before routing, same discipline as the prior 13.
    BookInput {
        code: EQUIPMENT_BOOK_ISG,
        slug: "inner_sea_gods",
        files: &["pathfinder/paizo/campaign_setting/inner_sea_gods/isg_equip.lst"],
    },
    BookInput {
        code: EQUIPMENT_BOOK_MYTHIC,
        slug: "mythic_adventures",
        files: &[
            "pathfinder/paizo/roleplaying_game/mythic_adventures/ma_equip.lst",
            "pathfinder/paizo/roleplaying_game/mythic_adventures/ma_equipmods.lst",
        ],
    },
    BookInput {
        code: EQUIPMENT_BOOK_ISC,
        slug: "inner_sea_combat",
        files: &[
            "pathfinder/paizo/campaign_setting/inner_sea_combat/isc_equip_arms_armor.lst",
            "pathfinder/paizo/campaign_setting/inner_sea_combat/isc_equip_magic.lst",
            "pathfinder/paizo/campaign_setting/inner_sea_combat/_pfs/pfs_isc_equip_arms_armor.lst",
            "pathfinder/paizo/campaign_setting/inner_sea_combat/_pfs/pfs_isc_equip_magic.lst",
        ],
    },
    BookInput {
        code: EQUIPMENT_BOOK_ISI,
        slug: "inner_sea_intrigue",
        files: &[
            "pathfinder/paizo/campaign_setting/inner_sea_intrigue/isi_equip_general.lst",
            "pathfinder/paizo/campaign_setting/inner_sea_intrigue/isi_equip_magic_items.lst",
            "pathfinder/paizo/campaign_setting/inner_sea_intrigue/isi_equipmods.lst",
        ],
    },
    BookInput {
        code: EQUIPMENT_BOOK_BOTD2,
        slug: "book_of_the_damned_volume_2",
        files: &["pathfinder/paizo/campaign_setting/book_of_the_damned_volume_2/botd2_equip.lst"],
    },
    // SD-32 T9 onboarding (card 11), `decisions.md §19` PI sign-off. Three
    // files per `docs/work-inventory.json`'s own `source_file` field over
    // this book's `not-ingested` equipment population -- re-derived by
    // direct read of `fresh_inventory.json`, not guessed from a directory
    // glob.
    BookInput {
        code: EQUIPMENT_BOOK_ISTEM,
        slug: "inner_sea_temples",
        files: &[
            "pathfinder/paizo/campaign_setting/inner_sea_temples/istem_equip_magic_items.lst",
            "pathfinder/paizo/campaign_setting/inner_sea_temples/istem_equip_arm_armor.lst",
            "pathfinder/paizo/campaign_setting/inner_sea_temples/istem_equip_general.lst",
        ],
    },
    // SD-32 T9 residual: the doc comment this replaced ("`ism_equipmods.lst`
    // is deliberately NOT named here ... zero not-ingested equipment units
    // for that file") went stale -- re-derived against the pinned oracle,
    // `docs/work-inventory.json` now carries 62 `not-ingested`
    // `equipment_modifier` units citing `ism_equipmods.lst` (kind, not the
    // `equipment` kind the retired comment checked). Added back in.
    BookInput {
        code: EQUIPMENT_BOOK_ISM,
        slug: "inner_sea_magic",
        files: &[
            "pathfinder/paizo/campaign_setting/inner_sea_magic/ism_equip.lst",
            "pathfinder/paizo/campaign_setting/inner_sea_magic/ism_equipmods.lst",
        ],
    },
    // SD-32 T9 residual: `adventurers_guide` had no `BOOK_INPUT` entry at
    // all -- see `EQUIPMENT_BOOK_AG`'s doc comment above.
    //
    // Cause C (`t9-onboarding-equipment-modifier-ability-rootcause` receipt,
    // group C): `ag_equipmods.lst` was simply absent from this `files` list,
    // so the book's one `equipment_modifier` object ("Medium Grey Maiden
    // Plate ~ Agile Maiden ~ Armor" et al) was never read at all -- a
    // genuine ingest gap, not a citation defect. Added back in; the
    // `_equipmods` basename check at `equipment_book_slug_for`/the category
    // classifier below routes it to `equipment_modifier` automatically.
    BookInput {
        code: EQUIPMENT_BOOK_AG,
        slug: "adventurers_guide",
        files: &[
            "pathfinder/paizo/roleplaying_game/adventurers_guide/ag_equip_arms_armor.lst",
            "pathfinder/paizo/roleplaying_game/adventurers_guide/ag_equip_general.lst",
            "pathfinder/paizo/roleplaying_game/adventurers_guide/ag_equip_magic_items.lst",
            "pathfinder/paizo/roleplaying_game/adventurers_guide/ag_equipmods.lst",
        ],
    },
    // SD-32 T9 residual: `ultimate_magic` (`EQUIPMENT_BOOK_UM`, already
    // routed in `equipment_resolver.rs`'s compiled catalog) had no
    // `BOOK_INPUT` entry -- 19 `not-ingested` equipment units, re-derived
    // against the pinned oracle. `pfs_um_equip_general.lst` is a real
    // Pathfinder Society legality-overlay file cited by some of those
    // units, same shape as `_pfs/pfs_acg_equip.lst` elsewhere in this list.
    BookInput {
        code: EQUIPMENT_BOOK_UM,
        slug: "ultimate_magic",
        files: &[
            "pathfinder/paizo/roleplaying_game/ultimate_magic/um_equip_general.lst",
            "pathfinder/paizo/roleplaying_game/ultimate_magic/_pfs/pfs_um_equip_general.lst",
        ],
    },
    // SD-32 `sd32-beginner-box-ingest` (`decisions.md §27b`): `beginner_box`
    // had no `BOOK_INPUT` entry at all -- 19 `equipment`-kind units (14
    // `no_record`, 5 cross-book-name-matched `no_formula_tokens`), the whole
    // of `docs/work-inventory.json`'s `beginner_box` population, re-derived
    // against the pinned oracle. Both files live directly under the book's
    // own corpus directory (no shared-library host-discovery hazard).
    BookInput {
        code: EQUIPMENT_BOOK_BB,
        slug: "beginner_box",
        files: &[
            "pathfinder/paizo/roleplaying_game/beginner_box/bbox_equip_magic_items.lst",
            "pathfinder/paizo/roleplaying_game/beginner_box/bbox_equip_arms_armor.lst",
        ],
    },
];

/// One parsed corpus record, before the already-held filter runs.
struct ParsedRecord {
    key: String,
    name: String,
    category: &'static str,
    cost_gp: Option<f64>,
    weight_lbs: Option<f64>,
    description: Option<String>,
    /// 1-indexed line within the file this record was parsed from — the
    /// `§53.5` declared-PI reader (`declared_pi_at`) needs a real citation,
    /// not just the parsed fields. Added `SD31-E6-F10-003`: before this
    /// field existed, this generator's ONLY PI defense was `screen_
    /// generated_table`'s blacklist substring scan (`§52.3`) over the
    /// FINISHED table — real, but not the SAME check `gen_cache_equipment_
    /// gap`'s separate JSON-write path already runs (`declared_pi_at`,
    /// reading the corpus's own `NAMEISPI:`/`DESCISPI:` declaration), and
    /// this generator's output (the compiled Rust table) is what the
    /// desktop catalog actually reads (`equipment_catalog_rows()` chains
    /// `equipment_gap_tables::equipment_gap_rows()` directly, never through
    /// the JSON files at all) — so a name the JSON-write path correctly
    /// excluded could still ship, uncaught, through THIS path. Confirmed
    /// live: `inner_sea_races:Belkzen Battle Standard` (`NAMEISPI:YES`
    /// declared) was excluded from `data/corpus/inner_sea_races/equipment/`
    /// by `gen_cache_equipment_gap` but was STILL compiled into this file's
    /// own `INNER_SEA_RACES_GAP_ROWS` static before this fix.
    line: u32,
    /// `decisions.md §24`: `Some((file, line))` when `key`/`name` above
    /// have ALREADY been overwritten with the Codex-generated neutral
    /// identity -- carries the real citation forward so `cache_gen::
    /// equipment_gap::generate` can resolve the record's true corpus
    /// location directly, without text-searching the LST for a `key`/
    /// `name` that no longer appears there (it now reads
    /// "Codex-Named Unit (...)"). `file` is relative to the BOOK's own
    /// directory (matching `find_citation`'s own return shape), not to
    /// `PCGEN_CORPUS_ROOT`. `None` for an ordinary (non-renamed) row.
    name_pi_citation: Option<(String, u32)>,
}

/// The catalog category a `.lst` basename declares. `_equipmods` is tested
/// before `_equip` for the same reason `file_kind` tests it first: every
/// equipmods basename also contains `_equip`.
/// `decisions.md §24`: the path a renamed row's citation must carry --
/// relative to the BOOK's own directory, matching `cache_gen::
/// equipment_gap::find_citation`'s own return shape (which
/// `PathBuf::strip_prefix`s the book directory), not `PCGEN_CORPUS_ROOT`.
/// `rel` is one of `BookInput.files`'s entries (always
/// `.../<slug>/...`); everything after the LAST `/<slug>/` marker is the
/// book-relative path, preserving any `_pfs/`-style subdirectory. Falls
/// back to the bare filename if the marker is somehow absent (never
/// observed -- every `BookInput.files` entry is rooted under its own
/// `slug` directory by construction).
fn book_relative_path(rel: &str, slug: &str) -> String {
    let marker = format!("/{slug}/");
    match rel.find(&marker) {
        Some(idx) => rel[idx + marker.len()..].to_string(),
        None => Path::new(rel).file_name().and_then(|f| f.to_str()).unwrap_or(rel).to_string(),
    }
}

fn category_for(basename: &str) -> &'static str {
    if basename.contains("_equipmods") {
        "Equipmods"
    } else if basename.contains("_magic_items") || basename.contains("_equip_magic") {
        "MagicItems"
    } else if basename.contains("_arms_armor") || basename.contains("_arm_armor") {
        "ArmsArmor"
    } else {
        "General"
    }
}

fn tab_fields(line: &str) -> Vec<&str> {
    line.split('\t').collect()
}

fn token_value<'a>(fields: &[&'a str], token: &str) -> Option<&'a str> {
    fields.iter().find_map(|f| f.trim().strip_prefix(token))
}

/// A `DESC:` token's real value, found by `SD31-E6-F10-003` while extending
/// this generator to `horror_adventures`: PCGen lets a `.COPY=`/variant row
/// state `DESC:.CLEAR` (discard the base record's inherited description)
/// immediately followed by a SECOND `DESC:` field carrying the row's own
/// real replacement text -- confirmed against the real corpus,
/// `horror_adventures/ha_equip_arms_armor.lst`'s `Lupine Rageskin` row:
/// `DESC:.CLEAR\tDESC:This +1 leather armor consists of wolf skins sewn
/// together...` (9 rows total share this shape, all in this one file).
/// `token_value`'s own `find_map` returns the FIRST match, which for all 9
/// rows is the literal string `".CLEAR"` -- not fabricated data, but a real,
/// richer description silently REPLACED by a bare directive token. This
/// walks every `DESC:` field in a row's declared order and returns the
/// first one that is not the bare clear-directive, so a
/// `.CLEAR`-then-real-value row recovers its real value and a row with only
/// `DESC:.CLEAR` and nothing after it (not observed in this generator's own
/// input set, but not assumed impossible) correctly yields `None` rather
/// than shipping the directive token as if it were prose.
fn description_token_value<'a>(fields: &[&'a str]) -> Option<&'a str> {
    fields.iter().filter_map(|f| f.trim().strip_prefix("DESC:")).find(|v| *v != ".CLEAR")
}

/// A PCGen numeric token, or `None` when the token is absent or carries a
/// formula (`WT*375`, `1+2`, …) this table deliberately does not evaluate.
fn numeric(fields: &[&str], token: &str) -> Option<f64> {
    token_value(fields, token).and_then(|v| v.trim().parse::<f64>().ok())
}

/// True when a raw `.lst` line is not a real record declaration at all
/// (blank, comment, ALL-CAPS directive other than `CLASS:`, an internal
/// bookkeeping `CATEGORY:`/`CATEGORY=Internal|` row, or a `.MOD` overlay) —
/// shared by [`parse_lst`] and [`collect_base_fields`] so the two scans of
/// the same corpus text can never silently disagree on what counts as a
/// record (the exact shape of `OPEN-ISSUES.md` row 90's citation defect:
/// two similar-but-drifted predicates over the same file).
fn is_non_record_line(first: &str, fields: &[&str]) -> bool {
    if first.is_empty() || first.starts_with('#') {
        return true;
    }
    let is_directive = first
        .split_once(':')
        .map(|(head, _)| {
            !head.is_empty() && head.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
        })
        .unwrap_or(false);
    if is_directive && !first.starts_with("CLASS:") {
        return true;
    }
    if first.starts_with("CATEGORY=Internal|") || fields.iter().any(|f| f.trim() == "CATEGORY:Internal")
    {
        return true;
    }
    if first.contains(".MOD") {
        return true;
    }
    // `SD31-E6-F10-003`: a PFS organized-play legality OVERLAY row -- e.g.
    // `bestiary_2/_pfs/pfs_b2_equip_arms_armor.lst`'s entire content is
    // shaped `<bare item name>\tTYPE:PFSNotLegal\t!PRECHARACTERTYPE:1,PC`,
    // flagging an item ALREADY declared elsewhere as ineligible for PFS
    // play, never a new item declaration. Confirmed corpus-wide before
    // narrowing this predicate: 146 files carry `PFSNotLegal` and only 11
    // of those occurrences also carry `COST:`/`DESC:`/`SPROP:` (a real
    // record that additionally states a legality flag), so this checks for
    // the ABSENCE of every real-content token too, not the presence of
    // `PFSNotLegal` alone -- a row combining both stays a real record.
    // Caught live: `pfs_b2_equip_arms_armor.lst`'s bare `Maul of the
    // Titans` row (no `KEY:` of its own) collided with the SAME book's real
    // `Maul of the Titans` row (whose own `KEY:` is the archetype-qualified
    // `Elysian Maul of the Titans`) and was shipped as if it were a second,
    // distinct catalog entry citing the real row's line -- a genuine
    // `record_key`/cited-line mismatch `tests/v06_corpus_trap_report.rs`'s
    // `ingested_record_keys_match_their_cited_line` caught. `bestiary_3`'s
    // `pfs_b3_equip_arms_armor.lst` had the identical shape for `Ranged
    // Cannon` (real row's own `KEY:` is `Ranged Cannon ~ Clockwork
    // Goliath`).
    if fields.iter().any(|f| f.trim() == "TYPE:PFSNotLegal")
        && !fields.iter().any(|f| {
            let f = f.trim();
            f.starts_with("COST:")
                || f.starts_with("WT:")
                || f.starts_with("DESC:")
                || f.starts_with("SPROP:")
                || f.starts_with("EQMOD:")
        })
    {
        return true;
    }
    false
}

/// A `.COPY=`-declaring record's base-record fields, keyed by the identity a
/// `.COPY=<identity>` reference resolves against — the base row's own `KEY:`
/// token when present, else its bare declared name. This is PCGen's own
/// resolution rule (confirmed against the real corpus: `Special Ability ~
/// Answering ~ Weapon.COPY=Answering` resolves against the `KEY:Special
/// Ability ~ Answering ~ Weapon` row, not any row literally named
/// "Special Ability ~ Answering ~ Weapon").
#[derive(Debug, Clone, Default, PartialEq)]
struct BaseFields {
    description: Option<String>,
    cost_gp: Option<f64>,
    weight_lbs: Option<f64>,
}

/// Builds the base-record lookup used by [`parse_lst`]'s `.COPY=`
/// inheritance, from every PLAIN (non-`.COPY=`) row across a book's own
/// input files — never from another `.COPY=` row, so inheritance is at most
/// one hop deep and cannot chain through an already-inherited value. Corpus-
/// wide: 0 `.COPY=` rows in this generator's 19 input files carry their own
/// `DESC:`/`SPROP:`/`COST:`/`WT:` token (re-derived at generation time, not
/// assumed), so this restriction has never actually excluded a real base.
/// "First wins" per book, matching every other first-match convention this
/// generator and `equipment_catalog_row_by_key` already use.
fn collect_base_fields(texts: &[String]) -> HashMap<String, BaseFields> {
    let mut map: HashMap<String, BaseFields> = HashMap::new();
    for text in texts {
        for line in text.lines() {
            let fields = tab_fields(line);
            let Some(first) = fields.first() else { continue };
            let first = first.trim();
            if is_non_record_line(first, &fields) || first.contains(".COPY=") {
                continue;
            }
            let key = token_value(&fields, "KEY:").map(str::to_string).unwrap_or_else(|| first.to_string());
            let desc = description_token_value(&fields).map(str::trim).filter(|d| !d.is_empty());
            let sprop = token_value(&fields, "SPROP:").map(str::trim).filter(|d| !d.is_empty());
            let description = match (desc, sprop) {
                (Some(d), Some(s)) if d != s => Some(format!("{d} {s}")),
                (Some(d), _) => Some(d.to_string()),
                (None, Some(s)) => Some(s.to_string()),
                (None, None) => None,
            };
            map.entry(key).or_insert(BaseFields {
                description,
                cost_gp: numeric(&fields, "COST:"),
                weight_lbs: numeric(&fields, "WT:"),
            });
        }
    }
    map
}

/// Parse one `.lst` under exactly `v06_work_inventory::enumerate_file`'s
/// record predicate. Kept as a standalone function so its agreement with
/// that enumerator is testable rather than asserted.
///
/// `base_fields` recovers a `.COPY=` row's inherited `description`/
/// `cost_gp`/`weight_lbs` when the row's OWN line states none of them —
/// `OPEN-ISSUES.md` rows 70/103's named recovery, generalized past
/// `description` alone once the same base-lookup mechanism proved it also
/// explains the pre-existing 8-row ACG `cost_gp` hand-correction
/// (`equipment_gap_tables.rs`'s former doc comment): both defects have the
/// identical root cause, a `.COPY=` row parsed as if it stated nothing
/// beyond its own line.
fn parse_lst(text: &str, category: &'static str, base_fields: &HashMap<String, BaseFields>) -> Vec<ParsedRecord> {
    let mut out = Vec::new();
    for (line_idx, line) in text.lines().enumerate() {
        let line_number = (line_idx + 1) as u32;
        let fields = tab_fields(line);
        let Some(first) = fields.first() else { continue };
        let first = first.trim();
        if is_non_record_line(first, &fields) {
            continue;
        }
        let copy_split = first.split_once(".COPY=");
        let copy_base = copy_split.map(|(base, _)| base.to_string());
        let name = if let Some((_, variant)) = copy_split {
            variant.to_string()
        } else if let Some(rest) =
            first.strip_prefix("CATEGORY=").and_then(|r| r.split_once('|')).map(|(_, r)| r)
        {
            rest.to_string()
        } else {
            first.to_string()
        };
        let key = token_value(&fields, "KEY:").map(|k| k.to_string()).unwrap_or_else(|| name.clone());

        // `DESC:` is the record's own prose; `SPROP:` is its special-property
        // line. A record may carry either, both, or neither — joined when
        // both are present, exactly as `ultimate_equipment::equipment_tables`
        // documents for the same corpus shape. Never a fabricated placeholder.
        let desc = description_token_value(&fields).map(str::trim).filter(|d| !d.is_empty());
        let sprop = token_value(&fields, "SPROP:").map(str::trim).filter(|d| !d.is_empty());
        let mut description = match (desc, sprop) {
            (Some(d), Some(s)) if d != s => Some(format!("{d} {s}")),
            (Some(d), _) => Some(d.to_string()),
            (None, Some(s)) => Some(s.to_string()),
            (None, None) => None,
        };
        let mut cost_gp = numeric(&fields, "COST:");
        let mut weight_lbs = numeric(&fields, "WT:");

        // `.COPY=` inheritance: a field this row's own line leaves unstated
        // is inherited from the base record it declares itself a copy of —
        // never overriding a field the row DOES state.
        if let Some(base) = &copy_base
            && let Some(inherited) = base_fields.get(base)
        {
            if description.is_none() {
                description = inherited.description.clone();
            }
            if cost_gp.is_none() {
                cost_gp = inherited.cost_gp;
            }
            if weight_lbs.is_none() {
                weight_lbs = inherited.weight_lbs;
            }
        }

        out.push(ParsedRecord {
            key,
            name,
            category,
            cost_gp,
            weight_lbs,
            description: safe_description(description),
            line: line_number,
            name_pi_citation: None,
        });
    }
    out
}

fn rust_string(value: &str) -> String {
    let mut out = String::from("\"");
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

fn rust_f64(value: Option<f64>) -> String {
    match value {
        Some(v) => format!("Some({v:?})"),
        None => "None".to_string(),
    }
}

/// The corpus checkout, from the environment only.
///
/// No default and no tilde expansion: `tests/no_foreign_home_paths.rs` treats
/// both an absolute `/home/<someone>` literal and an unexpanded `~` default in
/// Rust source as failures, and it is right to — a baked-in path is one
/// machine's truth shipped as everyone's. `PCGEN_CORPUS_ROOT` is the same
/// variable `pathfinder_unchained::monk_features`'s corpus-gated test already
/// requires, so there is no second convention to learn.
fn corpus_root() -> PathBuf {
    PathBuf::from(
        std::env::var("PCGEN_CORPUS_ROOT")
            .expect("PCGEN_CORPUS_ROOT must point at a local pcgen/data checkout"),
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = corpus_root();
    if !root.is_dir() {
        return Err(format!(
            "PCGEN_CORPUS_ROOT does not point at a directory: {}",
            root.display()
        )
        .into());
    }

    // What the hand-authored tables already hold, per book code — **row keys
    // only**, then tested against a corpus record's key OR its display name.
    // That asymmetry is not an oversight: it is exactly what
    // `v06_work_inventory`'s `equipment_keys` map does (it inserts
    // `row.key` and nothing else, then asks
    // `s.contains(unit.key) || s.contains(unit.name)`), so this generator's
    // output set is precisely that classifier's `not-ingested` set. A first
    // pass here inserted row NAMES into the set as well and emitted 741 rows
    // against the inventory's 769 — 28 records suppressed because some other
    // record's display name happened to equal theirs. Caught by differencing
    // the two counts, not by a test.
    let mut held: std::collections::BTreeMap<&'static str, BTreeSet<String>> = Default::default();
    for row in hand_authored_equipment_rows() {
        held.entry(row.book).or_default().insert(row.key.to_string());
    }

    let mut body = String::new();
    let mut totals: Vec<(&str, usize)> = Vec::new();
    let mut name_pi_excluded: u32 = 0;
    let mut description_pi_redacted: u32 = 0;
    let mut blacklist_name_excluded: u32 = 0;
    let mut blacklist_description_redacted: u32 = 0;

    for input in BOOK_INPUTS {
        let mut rows: Vec<ParsedRecord> = Vec::new();
        // Deduped on the record IDENTITY (`key`) alone, not on `(key, name)`:
        // once a key is in the book's catalog set, every unit carrying it is
        // classified ingested regardless of display name, so a second row
        // would add nothing but a duplicate catalog entry.
        let mut seen: BTreeSet<String> = BTreeSet::new();

        // Read every one of this book's files up front, once — needed twice
        // per file (base-field collection spans the whole book, then the
        // real per-record parse), and re-reading from disk a second time
        // risks a race against nothing (files are static) but is simply
        // wasted I/O; read once, use twice.
        let mut file_texts: Vec<(PathBuf, String, String, String)> = Vec::new();
        for rel in input.files {
            let path = root.join(rel);
            let text = std::fs::read_to_string(&path)
                .map_err(|e| format!("{}: {e}", path.display()))?;
            let basename = Path::new(rel).file_name().unwrap().to_string_lossy().into_owned();
            // `decisions.md §24`: the citation `cache_gen::equipment_gap::
            // generate` needs for a renamed row is relative to the BOOK's
            // own directory (matching `find_citation`'s own return shape),
            // not just the bare filename -- several books cite a `_pfs/`
            // subdirectory file, and `basename` alone would lose that.
            let book_rel = book_relative_path(rel, input.slug);
            file_texts.push((path, basename, book_rel, text));
        }
        let base_fields = collect_base_fields(
            &file_texts.iter().map(|(_, _, _, t)| t.clone()).collect::<Vec<_>>(),
        );

        for (path, basename, book_rel, text) in &file_texts {
            for record in parse_lst(text, category_for(basename), &base_fields) {
                let already = held
                    .get(input.code)
                    .map(|s| s.contains(&record.key) || s.contains(&record.name))
                    .unwrap_or(false);
                if already {
                    continue;
                }
                // A key repeated across two of a book's own files is one
                // record for the catalog's purposes; the first wins, matching
                // `equipment_catalog_row_by_key`'s own first-match rule.
                if !seen.insert(record.key.clone()) {
                    continue;
                }
                // `§53.5` declared-PI reader, applied to the COMPILED table
                // this time (`SD31-E6-F10-003`) -- see `ParsedRecord::line`'s
                // own doc comment for why this generator needed its own
                // copy of this check rather than relying on `gen_cache_
                // equipment_gap`'s separate JSON-write-path screen. A
                // declared-PI name USED to be a hard exclude; `decisions.md
                // §24` supersedes that -- see `screen_record`'s own doc
                // comment for why a row excluded HERE could never reach
                // `data/corpus/` at all. A declared-PI description is still
                // redacted to the marker, same as every other PI-screened
                // description in this program. `screen_record` (below) is
                // the real production screen; `blacklist_screen_tests`
                // drives THIS function directly rather than a hand-rolled
                // restatement that cannot detect it being removed (wave-12
                // adversarial review CONFIRMED the prior in-test `fn
                // screen` was a gate that could not fail).
                let declared = declared_pi_at(path, record.line);
                match screen_record(record, declared) {
                    ScreenOutcome::Kept { mut record, name_is_pi, description_pi_redacted: d1, description_blacklist_redacted: d2 } => {
                        if d1 {
                            description_pi_redacted += 1;
                        }
                        if d2 {
                            blacklist_description_redacted += 1;
                        }
                        if name_is_pi {
                            // `decisions.md §24` -- ingest under a
                            // Codex-generated neutral name derived ONLY
                            // from (kind, book, source_file, source_line),
                            // never from the original PI name (which this
                            // branch does not even read to compute it).
                            if declared.name {
                                name_pi_excluded += 1;
                            } else {
                                blacklist_name_excluded += 1;
                            }
                            let kind = if record.category == "Equipmods" { "equipment_modifier" } else { "equipment" };
                            record.name_pi_citation = Some((book_rel.clone(), record.line));
                            record.key = neutral_key(kind, input.slug, book_rel, record.line);
                            record.name = neutral_name(kind, input.slug, book_rel, record.line);
                        }
                        rows.push(record);
                    }
                }
            }
        }

        totals.push((input.slug, rows.len()));
        writeln!(
            body,
            "\n/// {} — {} record(s) the hand-authored `{}` table does not hold.\npub static {}_GAP_ROWS: &[EquipmentGapRow] = &[",
            input.slug,
            rows.len(),
            input.slug,
            input.slug.to_uppercase()
        )?;
        for row in &rows {
            writeln!(
                body,
                "    EquipmentGapRow {{ book: {}, key: {}, name: {}, category: {}, cost_gp: {}, weight_lbs: {}, description: {}, name_pi_citation: {} }},",
                rust_string(input.code),
                rust_string(&row.key),
                rust_string(&row.name),
                rust_string(row.category),
                rust_f64(row.cost_gp),
                rust_f64(row.weight_lbs),
                match &row.description {
                    Some(d) => format!("Some({})", rust_string(d)),
                    None => "None".to_string(),
                },
                match &row.name_pi_citation {
                    Some((file, line)) => format!("Some(({}, {line}))", rust_string(file)),
                    None => "None".to_string(),
                }
            )?;
        }
        writeln!(body, "];")?;
    }

    let total: usize = totals.iter().map(|(_, n)| *n).sum();
    let mut header = String::new();
    writeln!(
        header,
        "//! Corpus equipment and equipment-modifier records that belong to an\n\
         //! ALREADY-COMPILED book whose hand-authored per-book table does not hold\n\
         //! them — the `not-ingested` population of `docs/work-inventory.json`'s\n\
         //! `equipment`/`equipment_modifier` kinds, closed corpus-wide.\n\
         //!\n\
         //! **GENERATED — do not edit by hand.** Regenerate with\n\
         //! `PCGEN_CORPUS_ROOT=<pcgen>/data cargo run --locked --bin gen_equipment_gap_tables`.\n\
         //! The generator applies `v06_work_inventory`'s own record predicate, so a\n\
         //! row here is exactly a row that inventory reported `not-ingested`.\n\
         //!\n\
         //! `cost_gp`/`weight_lbs` are `None` when the corpus record carries no such\n\
         //! token, or carries a PCGen formula this table deliberately does not\n\
         //! evaluate — never a fabricated flat number. `description` joins the\n\
         //! record's `DESC:` and `SPROP:` tokens when both are present. A `.COPY=`\n\
         //! row that states none of `description`/`cost_gp`/`weight_lbs` on its own\n\
         //! line inherits them from the base record it declares itself a copy of\n\
         //! (`SD31-E6-F6-001`, `OPEN-ISSUES.md` rows 70/103) — never fabricated,\n\
         //! never inherited past one hop.\n\
         //!\n\
         //! `decisions.md §24`: a row whose real `key`/`name` is Product Identity\n\
         //! (declared `NAMEISPI:YES` or a blacklist hit) is no longer excluded\n\
         //! whole — it is INCLUDED under a Codex-generated neutral `key`/`name`\n\
         //! (`name_pi_citation` is `Some` for exactly these rows).\n\
         //!\n\
         //! Total: {total} rows.\n"
    )?;
    writeln!(
        header,
        "/// One recovered corpus equipment row. Deliberately one flat shape for\n\
         /// every book: unlike the hand-authored per-book tables (each with its own\n\
         /// `EquipmentCategory` enum and field set), these rows exist to be chained\n\
         /// into `equipment_resolver::equipment_catalog_rows()` and rendered by the\n\
         /// desktop equipment catalog, both of which read exactly these fields.\n\
         #[derive(Debug, Clone, Copy, PartialEq)]\n\
         pub struct EquipmentGapRow {{\n\
         \x20   /// One of `equipment_resolver`'s `EQUIPMENT_BOOK_*` codes.\n\
         \x20   pub book: &'static str,\n\
         \x20   /// The record's `KEY:` token when it carries one, else its display name.\n\
         \x20   pub key: &'static str,\n\
         \x20   pub name: &'static str,\n\
         \x20   /// The catalog category, matching the `EquipmentCategory` variant names\n\
         \x20   /// the per-book tables project onto `EquipmentCatalogEntryDto::category`.\n\
         \x20   pub category: &'static str,\n\
         \x20   pub cost_gp: Option<f64>,\n\
         \x20   pub weight_lbs: Option<f64>,\n\
         \x20   pub description: Option<&'static str>,\n\
         \x20   /// `decisions.md §24`: `Some((source_file, source_line))`\n\
         \x20   /// ONLY when `key`/`name` above are a Codex-generated neutral\n\
         \x20   /// identity (the row's real name/key is Product Identity) --\n\
         \x20   /// carries the real citation so `cache_gen::equipment_gap::\n\
         \x20   /// generate` can resolve it directly instead of text-searching\n\
         \x20   /// for a `key`/`name` the real corpus no longer contains.\n\
         \x20   /// `source_file` is relative to the book's own directory.\n\
         \x20   /// `None` for an ordinary row.\n\
         \x20   pub name_pi_citation: Option<(&'static str, u32)>,\n\
         }}\n"
    )?;
    writeln!(
        header,
        "/// Every recovered row, in book order. The order is load-bearing the same\n\
         /// way `equipment_catalog_rows()`'s is: first match wins for key lookup.\n\
         pub fn equipment_gap_rows() -> impl Iterator<Item = &'static EquipmentGapRow> {{\n\
         \x20   [{}]\n\
         \x20       .into_iter()\n\
         \x20       .flat_map(|rows| rows.iter())\n\
         }}",
        BOOK_INPUTS
            .iter()
            .map(|b| format!("{}_GAP_ROWS", b.slug.to_uppercase()))
            .collect::<Vec<_>>()
            .join(", ")
    )?;

    let generated = format!("{header}{body}");

    // Provenance gate (`epic-3-provenance`): screen the text BEFORE writing it.
    let hits = screen_generated_table(OUTPUT_RELATIVE_PATH, &generated);
    if !hits.is_empty() {
        eprintln!("PI screening HARD STOP — {} hit(s), nothing written:", hits.len());
        for hit in &hits {
            eprintln!("  {hit:?}");
        }
        std::process::exit(1);
    }

    std::fs::write(Path::new(OUTPUT_RELATIVE_PATH), &generated)?;
    println!("wrote {OUTPUT_RELATIVE_PATH}: {total} rows");
    for (slug, n) in &totals {
        println!("  {slug:28} {n:5}");
    }
    println!("pi-screening: CLEAN (0 hits over the generated text)");
    println!(
        "declared-pi (§53.5): {name_pi_excluded} name(s) renamed under a Codex-generated \
         neutral identity (decisions.md §24), {description_pi_redacted} description(s) \
         redacted to {REDACTED_PI_MARKER:?}"
    );
    println!(
        "blacklist-screen (per-record, §52.3-equivalent): {blacklist_name_excluded} name/key \
         hit(s) renamed under a Codex-generated neutral identity (decisions.md §24), \
         {blacklist_description_redacted} description(s) redacted to {REDACTED_PI_MARKER:?}"
    );
    Ok(())
}

#[cfg(test)]
mod safe_description_tests {
    use super::*;

    /// The real reproduction (`IntItemBase`): a bare (unnumbered) `%`
    /// placeholder run followed by a multi-argument `|` tail render_pcgen_
    /// desc's numbered-reference detection does not recognize -- the tail
    /// survives verbatim, and `no_catalog_serves_a_description_carrying_
    /// raw_pcgen_syntax` (apps/desktop) correctly refuses to serve it.
    /// `safe_description` must refuse it at the source instead.
    #[test]
    fn a_description_whose_render_still_leaks_pcgen_syntax_is_refused() {
        let raw = "Intelligence %, Wisdom %, Charisma %, Ego Score %|IntItemStatINT|IntItemStatWIS|IntItemStatCHA|IntelligentItemEgo".to_string();
        assert_eq!(safe_description(Some(raw)), None);
    }

    /// A description with no PCGen substitution syntax at all is untouched.
    #[test]
    fn a_clean_description_passes_through_unchanged() {
        let raw = "Enhancement bonus increases by 4 (to a max of 5)".to_string();
        assert_eq!(safe_description(Some(raw.clone())), Some(raw));
    }

    #[test]
    fn none_stays_none() {
        assert_eq!(safe_description(None), None);
    }

    /// Empirical check, not assumed: a bare `%CHOICE` keyword reference
    /// with NO trailing `|` argument tail renders clean today (confirmed by
    /// this cycle's own guarded regen -- only 1 of 69 `%`/`|`-carrying
    /// recovered descriptions actually leaked in the real desktop catalog
    /// render). Prints the rendered result so a future reader can see
    /// exactly what `safe_description` decided, rather than trusting a
    /// bare pass/fail.
    #[test]
    fn a_bare_choice_keyword_with_no_pipe_tail_survives() {
        let raw = "Enhancement bonus to ability %CHOICE".to_string();
        let result = safe_description(Some(raw));
        assert_eq!(
            result.as_deref(),
            Some("Enhancement bonus to ability %CHOICE"),
            "a dropped %CHOICE that renders to clean, leak-free text must still ship -- \
             matches production's own equipment catalog behavior"
        );
    }
}

#[cfg(test)]
mod copy_inheritance_tests {
    use super::*;

    /// The proof case, reproduced from the real corpus (`SD31-E6-F6-001`):
    /// `acg_equipmods.lst`'s "Answering" `.COPY=` row states only
    /// `VISIBLE:NO`; the base it copies (`KEY:Special Ability ~ Answering ~
    /// Weapon`) carries a real `SPROP:`. Before this cycle's fix, the `.COPY=`
    /// row shipped `description: None` despite the base's real prose existing
    /// two lines away in the same file — `OPEN-ISSUES.md` rows 70/103's own
    /// named recovery, generalized to this generator.
    #[test]
    fn a_copy_row_inherits_the_base_records_description_when_it_states_none_of_its_own() {
        let text = "Answering\t\tKEY:Special Ability ~ Answering ~ Weapon\t\tSPROP:Enhancement bonus increases by 4\n\
                     Special Ability ~ Answering ~ Weapon.COPY=Answering\t\tVISIBLE:NO\n";
        let base_fields = collect_base_fields(&[text.to_string()]);
        let records = parse_lst(text, "Equipmods", &base_fields);
        let copy_record = records.iter().find(|r| r.key == "Answering").expect("copy row parsed");
        assert_eq!(
            copy_record.description.as_deref(),
            Some("Enhancement bonus increases by 4"),
            "the .COPY= row must inherit the base row's SPROP text, not ship None"
        );
    }

    /// The base's identity for a `.COPY=` reference is its own `KEY:` token,
    /// not its bare first-column name — resolving against the bare name
    /// alone would silently miss every real case in the corpus (this exact
    /// shape: the base's first column is "Answering" too, coincidentally
    /// equal to the KEY the `.COPY=` reference actually names).
    #[test]
    fn resolution_is_by_key_not_by_bare_first_column_name() {
        let text = "Answering\t\tKEY:Special Ability ~ Answering ~ Weapon\t\tSPROP:Real base text\n\
                     Special Ability ~ Answering ~ Weapon.COPY=Answering\t\tVISIBLE:NO\n";
        let base_fields = collect_base_fields(&[text.to_string()]);
        assert!(
            base_fields.contains_key("Special Ability ~ Answering ~ Weapon"),
            "must be keyed by the KEY: token, since that is what the .COPY= reference names"
        );
        assert!(
            !base_fields.contains_key("Answering"),
            "must NOT also be keyed by the bare first-column name — that name belongs to a \
             DIFFERENT identity (the variant), not the base"
        );
    }

    /// A `.COPY=` row that DOES state its own field on its own line keeps
    /// that value — inheritance only fills a genuine gap, never overrides.
    #[test]
    fn a_copy_row_stating_its_own_field_is_never_overridden_by_the_base() {
        let text = "Widget\t\tKEY:Special Ability ~ Widget ~ Weapon\t\tSPROP:Base text\t\tCOST:100\n\
                     Special Ability ~ Widget ~ Weapon.COPY=Widget Variant\t\tDESC:Own real text\t\tCOST:250\n";
        let base_fields = collect_base_fields(&[text.to_string()]);
        let records = parse_lst(text, "Equipmods", &base_fields);
        let copy_record =
            records.iter().find(|r| r.name == "Widget Variant").expect("copy row parsed");
        assert_eq!(copy_record.description.as_deref(), Some("Own real text"));
        assert_eq!(copy_record.cost_gp, Some(250.0));
    }

    /// `cost_gp`/`weight_lbs` inherit the identical way `description` does —
    /// the same defect shape as the pre-existing 8-row ACG hand-correction
    /// this cycle's fix generalizes and makes automatic.
    #[test]
    fn a_copy_row_inherits_cost_and_weight_when_it_states_neither() {
        let text = "Amorphous\t\tKEY:Special Ability ~ Amorphous ~ Armor\t\tCOST:4500\t\tSPROP:1/day take form\n\
                     Special Ability ~ Amorphous ~ Armor.COPY=Amorphous\t\tVISIBLE:NO\n";
        let base_fields = collect_base_fields(&[text.to_string()]);
        let records = parse_lst(text, "Equipmods", &base_fields);
        let copy_record = records.iter().find(|r| r.key == "Amorphous").expect("copy row parsed");
        assert_eq!(copy_record.cost_gp, Some(4500.0));
        assert_eq!(copy_record.description.as_deref(), Some("1/day take form"));
    }

    /// No base found at all (the true no-fabrication case): a `.COPY=` row
    /// whose base is genuinely absent from this book's files stays `None` —
    /// never invents a value.
    #[test]
    fn a_copy_row_with_no_resolvable_base_stays_none_rather_than_fabricating() {
        let text = "Some Base.COPY=Orphan Variant\t\tVISIBLE:NO\n";
        let base_fields = collect_base_fields(&[text.to_string()]);
        let records = parse_lst(text, "Equipmods", &base_fields);
        let copy_record = records.iter().find(|r| r.name == "Orphan Variant").expect("parsed");
        assert_eq!(copy_record.description, None);
        assert_eq!(copy_record.cost_gp, None);
        assert_eq!(copy_record.weight_lbs, None);
    }

    /// Base-field collection spans multiple files of the same book (the real
    /// shape: `ACG` reads both `acg_equipmods.lst` and
    /// `_pfs/pfs_acg_equip.lst`) — a base declared in one file must be found
    /// by a `.COPY=` row parsed from a different file's text.
    #[test]
    fn base_lookup_spans_multiple_files_of_the_same_book() {
        let file_a = "Foo\t\tKEY:Special Ability ~ Foo ~ Weapon\t\tSPROP:Cross-file base text\n".to_string();
        let file_b = "Special Ability ~ Foo ~ Weapon.COPY=Foo\t\tVISIBLE:NO\n".to_string();
        let base_fields = collect_base_fields(&[file_a, file_b.clone()]);
        let records = parse_lst(&file_b, "Equipmods", &base_fields);
        let copy_record = records.iter().find(|r| r.key == "Foo").expect("copy row parsed");
        assert_eq!(copy_record.description.as_deref(), Some("Cross-file base text"));
    }

    /// A `.COPY=` row can never itself serve as another row's base — proves
    /// inheritance is at most one hop and cannot chain through an
    /// already-inherited value.
    #[test]
    fn a_copy_row_is_never_used_as_a_base_for_another_copy_row() {
        let text = "Base\t\tKEY:X\t\tSPROP:Real\n\
                     X.COPY=Mid\t\tVISIBLE:NO\n\
                     Mid.COPY=Leaf\t\tVISIBLE:NO\n";
        let base_fields = collect_base_fields(&[text.to_string()]);
        assert!(
            !base_fields.contains_key("Mid"),
            "a .COPY= row (Mid, whose own declared name is X.COPY=Mid) must never be \
             registered as a base — only plain (non-.COPY=) declarations are bases"
        );
        let records = parse_lst(text, "Equipmods", &base_fields);
        let leaf = records.iter().find(|r| r.name == "Leaf").expect("leaf parsed");
        // Leaf's base identity is "Mid" (bare, no KEY: token on that COPY
        // line), which base_fields correctly does NOT hold — so leaf stays
        // unresolved rather than silently chaining through X's real text.
        assert_eq!(leaf.description, None);
    }
}

#[cfg(test)]
mod desc_clear_directive_tests {
    use super::*;

    /// The proof case, reproduced byte-for-byte from the real corpus
    /// (`SD31-E6-F10-003`, found extending this generator to
    /// `horror_adventures`): `ha_equip_arms_armor.lst`'s `Lupine Rageskin`
    /// row states TWO `DESC:` fields on one line — `DESC:.CLEAR` (discard
    /// the base's inherited description) followed by the row's own real
    /// replacement prose. Before this cycle's fix, `token_value`'s
    /// `find_map` returned the FIRST `DESC:` match — the literal string
    /// `".CLEAR"` — so the record shipped that directive token joined with
    /// its `SPROP:` as if it were real description text, silently dropping
    /// 200+ words of genuine corpus prose. 9 rows in this one file share
    /// this exact shape.
    #[test]
    fn a_desc_clear_directive_is_skipped_in_favor_of_the_real_desc_that_follows_it() {
        let line = "Leather Armor (Base).COPY=Lupine Rageskin\t\tSORTKEY:Lupine Rageskin\tVISIBLE:YES\t\
                     SPROP:when wearer rages, he turns into a Medium wolf with +1 bonus to natural armor\t\t\
                     DESC:.CLEAR\tDESC:This +1 leather armor consists of wolf skins sewn together with sinew.";
        let base_fields = collect_base_fields(&[]);
        let records = parse_lst(line, "ArmsArmor", &base_fields);
        let record = &records[0];
        assert_eq!(
            record.description.as_deref(),
            Some(
                "This +1 leather armor consists of wolf skins sewn together with sinew. \
                 when wearer rages, he turns into a Medium wolf with +1 bonus to natural armor"
            ),
            "must ship the row's real DESC: prose (joined with SPROP:), never the bare \
             `.CLEAR` directive token"
        );
    }

    /// A row with only a single, ordinary `DESC:` field is unaffected — the
    /// fix must not touch the overwhelmingly common case.
    #[test]
    fn an_ordinary_single_desc_field_is_unaffected() {
        let fields = ["Foo", "DESC:Ordinary prose"];
        assert_eq!(description_token_value(&fields), Some("Ordinary prose"));
    }

    /// A row stating `DESC:.CLEAR` with no real `DESC:` after it yields
    /// `None`, never the directive token itself — this exact shape was not
    /// observed in this generator's own input set, but the function must
    /// not assume it is impossible.
    #[test]
    fn a_bare_clear_with_no_following_desc_yields_none() {
        let fields = ["Foo", "DESC:.CLEAR"];
        assert_eq!(description_token_value(&fields), None);
    }
}

#[cfg(test)]
mod pfs_not_legal_overlay_tests {
    use super::*;

    /// The proof case, reproduced byte-for-byte from the real corpus
    /// (`SD31-E6-F10-003`): `bestiary_2/_pfs/pfs_b2_equip_arms_armor.lst`'s
    /// bare `Maul of the Titans` row carries only a PFS legality flag, no
    /// `KEY:` of its own and no real equipment content — the SAME book's
    /// real declaration (`b2_equip_arms_armor.lst`) states this exact item
    /// under `KEY:Elysian Maul of the Titans`, so before this fix the two
    /// rows' bare-name-vs-KEY mismatch let the overlay row slip past the
    /// `seen` dedup as if it were a second, distinct catalog entry.
    #[test]
    fn a_bare_pfs_not_legal_overlay_row_is_not_a_record() {
        let base_fields = collect_base_fields(&[]);
        let text = "Maul of the Titans\tTYPE:PFSNotLegal\t!PRECHARACTERTYPE:1,PC\n";
        assert_eq!(
            parse_lst(text, "ArmsArmor", &base_fields).len(),
            0,
            "a bare PFSNotLegal overlay row with no real content must not parse as a record"
        );
    }

    /// The corpus-wide guard this predicate must not become too broad for:
    /// 11 of 146 `PFSNotLegal`-carrying rows corpus-wide ALSO carry real
    /// content (a genuine record that happens to also be PFS-illegal), and
    /// those must still parse.
    #[test]
    fn a_pfs_not_legal_row_that_also_carries_real_content_still_parses() {
        let base_fields = collect_base_fields(&[]);
        let text = "Real Weapon\tTYPE:PFSNotLegal\tCOST:50\tWT:2\tDESC:A real item that happens to be PFS-illegal.\n";
        let records = parse_lst(text, "ArmsArmor", &base_fields);
        assert_eq!(records.len(), 1, "a PFSNotLegal row carrying real content must still parse");
        assert_eq!(records[0].cost_gp, Some(50.0));
    }
}

#[cfg(test)]
mod declared_pi_tests {
    use super::*;

    /// The proof case, reproduced from the real corpus (`SD31-E6-F10-003`):
    /// `inner_sea_races:Belkzen Battle Standard` declares `NAMEISPI:YES` and
    /// was compiled into this generator's output BEFORE this fix, even
    /// though `gen_cache_equipment_gap`'s separate JSON-write path already
    /// excluded it. `line` is 1-indexed, matching every other citation this
    /// program stores.
    #[test]
    fn a_nameispi_row_is_flagged_and_a_descispi_row_is_flagged_separately() {
        let dir = std::env::temp_dir().join(format!("gegt_pi_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("book_equip.lst");
        std::fs::write(
            &file,
            "Belkzen Battle Standard\tCOST:34000\tNAMEISPI:YES\n\
             Ordinary Banner\tCOST:100\tDESC:A plain banner.\tDESCISPI:YES\n\
             Plain Rope\tCOST:1\n",
        )
        .unwrap();
        let name_pi = declared_pi_at(&file, 1);
        assert!(name_pi.name, "line 1's NAMEISPI:YES must be read");
        assert!(!name_pi.description);
        let desc_pi = declared_pi_at(&file, 2);
        assert!(desc_pi.description, "line 2's DESCISPI:YES must be read");
        assert!(!desc_pi.name);
        let clean = declared_pi_at(&file, 3);
        assert!(!clean.name && !clean.description, "an undeclared row must read as clean");
        std::fs::remove_dir_all(&dir).ok();
    }

    /// Line 0 (no citation at all) is never a declaration -- mirrors
    /// `cache_gen::equipment_gap`'s own `declared_pi_at_line_zero_is_no_
    /// declaration`, since this is a local copy of that exact function.
    #[test]
    fn line_zero_is_no_declaration() {
        assert!(!declared_pi_at(Path::new("/nonexistent"), 0).any());
    }
}

#[cfg(test)]
mod blacklist_screen_tests {
    use super::*;

    /// The gap `declared_pi_at` cannot close: a record whose CORPUS row
    /// declares no `NAMEISPI:`/`DESCISPI:` token at all, but whose free-text
    /// `DESC:` names a blacklisted deity, mid-sentence -- reproduced
    /// verbatim from the real, undeclared corpus row this cycle's own
    /// 5-book extension would otherwise ship (`inner_sea_gods/isg_equip.lst`,
    /// `Cloak Of The Night Sky`: "...If Desna is the wearer's patron..."
    /// carries no `DESCISPI:` token on that line). `declared_pi_at` reads
    /// this line as clean (proven first, so the two screens are shown not
    /// to overlap); `blacklist_hit` -- the SAME `PI_BLACKLIST_TERMS` list
    /// `screen_generated_table`'s own whole-file hard stop already uses,
    /// never forked -- is the only thing that catches it.
    #[test]
    fn a_deity_name_inside_undeclared_description_prose_is_a_blacklist_hit_but_not_a_declared_pi_hit(
    ) {
        let dir = std::env::temp_dir().join(format!("gegt_blacklist_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("isg_equip.lst");
        std::fs::write(
            &file,
            "Cloak Of The Night Sky\t\t\t\t\tTYPE:Magic.Wondrous Item.SLOT_Shoulders.Cloak\t\t\t\tCOST:2500\tWT:1\t\tSOURCELONG:Inner Sea Gods\tSOURCESHORT:isg\t\tSOURCEPAGE:p.262\tDESC:This dark hooded cloak is decorated with embroidered comets. If Desna is the wearer's patron: as a standard action the wearer can cause additional celestial bodies to appear.\n",
        )
        .unwrap();
        let declared = declared_pi_at(&file, 1);
        assert!(
            !declared.any(),
            "the real corpus row carries no NAMEISPI:/DESCISPI: token -- declared_pi_at must read it clean"
        );
        assert_eq!(
            blacklist_hit("Cloak Of The Night Sky"),
            None,
            "the NAME carries no blacklisted term"
        );
        assert_eq!(
            blacklist_hit(
                "This dark hooded cloak is decorated with embroidered comets. If Desna is the wearer's patron: as a standard action the wearer can cause additional celestial bodies to appear."
            ),
            Some("Desna"),
            "the free-text DESCRIPTION carries an undeclared blacklisted deity name"
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    /// A name-field hit (`"Altar of Desna"`) and a clean record both resolve
    /// as their own English word would suggest -- no false positive on an
    /// ordinary item, no false negative on the exact real corpus name this
    /// cycle's own 5-book extension names as its proof case.
    #[test]
    fn a_name_field_hit_is_found_and_a_clean_record_is_not() {
        assert_eq!(blacklist_hit("Altar of Desna"), Some("Desna"));
        assert_eq!(blacklist_hit("Masterwork Backpack"), None);
    }

    /// Mutation proof: this gate must be ABLE to fail. Simulating the
    /// production call site's own exclude-on-name / redact-on-description
    /// logic directly (not a fixture the gate cannot see) confirms a
    /// blacklisted name is excluded outright and a blacklisted description
    /// is redacted to the marker rather than shipped -- the same two
    /// outcomes `declared_pi_at`'s own name/description split already
    /// produces, over the SAME term list `screen_generated_table`'s
    /// whole-file backstop uses, so a per-row miss here still cannot reach
    /// a player: the backstop still runs over the finished table.
    #[test]
    fn production_logic_excludes_a_blacklisted_name_and_redacts_a_blacklisted_description() {
        // **Wave-12 fix**: drives the REAL production function
        // (`screen_record`, called by `main()`'s own per-record loop) rather
        // than a hand-rolled restatement — adversarial review CONFIRMED the
        // prior version of this test defined its own local `fn screen`,
        // which could not detect the production screen being removed
        // entirely (verified by deleting `main()`'s screening block: all
        // three tests in this module still passed). This version cannot
        // make that mistake because there is only one `screen_record` to
        // call.
        fn record(name: &str, description: Option<&str>) -> ParsedRecord {
            ParsedRecord {
                key: name.to_string(),
                name: name.to_string(),
                category: "General",
                cost_gp: None,
                weight_lbs: None,
                description: description.map(|d| d.to_string()),
                line: 1,
                name_pi_citation: None,
            }
        }
        let clean = DeclaredProductIdentity::default();

        match screen_record(record("Altar of Desna", None), clean) {
            ScreenOutcome::Kept { name_is_pi, .. } => {
                assert!(name_is_pi, "a blacklisted NAME must be flagged for rename (decisions.md §24), never dropped")
            }
        }

        match screen_record(
            record("Cloak Of The Night Sky", Some("...If Desna is the wearer's patron...")),
            clean,
        ) {
            ScreenOutcome::Kept { record, name_is_pi, description_pi_redacted, description_blacklist_redacted } => {
                assert!(!name_is_pi, "the NAME itself carries no blacklisted term");
                assert_eq!(record.description.as_deref(), Some(REDACTED_PI_MARKER));
                assert!(!description_pi_redacted, "not a DECLARED hit -- must count as the blacklist counter, not the declared one");
                assert!(description_blacklist_redacted);
            }
        }

        match screen_record(record("Masterwork Backpack", Some("A sturdy pack.")), clean) {
            ScreenOutcome::Kept { record, name_is_pi, description_pi_redacted, description_blacklist_redacted } => {
                assert!(!name_is_pi);
                assert_eq!(record.description.as_deref(), Some("A sturdy pack."));
                assert!(!description_pi_redacted);
                assert!(!description_blacklist_redacted);
            }
        }
    }

    /// A DECLARED `NAMEISPI:YES` flags the record for rename even when its
    /// name carries no blacklist term at all -- the two `name_is_pi`
    /// triggers (`declared.name` / blacklist hit) are genuinely distinct
    /// conditions, not one path double-counted as the other.
    #[test]
    fn a_declared_name_hit_is_flagged_for_rename_even_with_no_blacklist_term() {
        let record = ParsedRecord {
            key: "Belkzen Battle Standard".to_string(),
            name: "Belkzen Battle Standard".to_string(),
            category: "General",
            cost_gp: None,
            weight_lbs: None,
            description: None,
            name_pi_citation: None,
            line: 1,
        };
        let declared = DeclaredProductIdentity { name: true, description: false };
        match screen_record(record, declared) {
            ScreenOutcome::Kept { name_is_pi, .. } => {
                assert!(name_is_pi, "a declared NAMEISPI:YES record must be flagged for rename")
            }
        }
    }

    /// End-to-end `decisions.md §24` proof against `main()`'s real caller
    /// logic (mirrored here rather than driving the whole binary): a
    /// name-PI row must be KEPT under a Codex-generated neutral
    /// `key`/`name`, carrying its real citation on `name_pi_citation` so
    /// the corpus-dump step can still find it, and the returned identity
    /// must never contain the original PI-shaped string.
    #[test]
    fn a_renamed_row_carries_a_neutral_identity_and_its_real_citation() {
        let record = ParsedRecord {
            key: "Belkzen Battle Standard".to_string(),
            name: "Belkzen Battle Standard".to_string(),
            category: "General",
            cost_gp: None,
            weight_lbs: None,
            description: None,
            name_pi_citation: None,
            line: 7,
        };
        let declared = DeclaredProductIdentity { name: true, description: false };
        let ScreenOutcome::Kept { mut record, name_is_pi, .. } = screen_record(record, declared);
        assert!(name_is_pi);
        // Mirrors `main()`'s own post-`screen_record` rename block.
        let book_rel = "isr_equip.lst";
        record.name_pi_citation = Some((book_rel.to_string(), record.line));
        record.key = neutral_key("equipment", "inner_sea_races", book_rel, record.line);
        record.name = neutral_name("equipment", "inner_sea_races", book_rel, record.line);

        assert!(record.name.starts_with("Codex-Named Unit ("));
        assert!(record.key.starts_with("Codex-Named Unit ("));
        assert!(!record.name.contains("Belkzen"));
        assert!(!record.key.contains("Belkzen"));
        assert_eq!(record.name_pi_citation, Some(("isr_equip.lst".to_string(), 7)));
    }

    #[test]
    fn book_relative_path_strips_everything_before_the_book_slug_directory() {
        assert_eq!(
            book_relative_path(
                "pathfinder/paizo/roleplaying_game/ultimate_magic/_pfs/pfs_um_equip_general.lst",
                "ultimate_magic"
            ),
            "_pfs/pfs_um_equip_general.lst"
        );
        assert_eq!(
            book_relative_path("pathfinder/paizo/campaign_setting/inner_sea_gods/isg_equip.lst", "inner_sea_gods"),
            "isg_equip.lst"
        );
    }
}
