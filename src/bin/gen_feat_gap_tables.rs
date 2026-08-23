//! Codegen for `rules_tables::feat_gap_tables` — the corpus `feat` records
//! that live in an **already-compiled** book whose hand-authored per-book feat
//! table does not hold them.
//!
//! # Why this binary exists
//!
//! `docs/work-inventory.json` classifies a `feat` unit as `not-ingested`
//! (`evidence: "feat_key_absent_from_catalog"`) when the book has a compiled
//! rule set but `feats_all::all_feat_tables()` holds no record matching the
//! corpus record's `KEY:` or its display name. Those are real gaps inside
//! started books — not un-started books — and closing them needs no new
//! `RuleSetId`, no new corpus cache and no new player surface: the desktop
//! feat catalog already renders every record `all_feat_tables()` yields.
//!
//! This is the feat sibling of `gen_equipment_gap_tables`, and it follows
//! that binary's contract exactly: the same record predicate as
//! `v06_work_inventory::enumerate_file`, an already-held filter derived from
//! the hand-authored tables themselves (never a hand-maintained exclusion
//! list), and a PI screen run over the generated text **before** it is
//! written.
//!
//! # What it deliberately does not do
//!
//! It never invents a value. `description` is the record's own `DESC:` (joined
//! with `BENEFIT:` when both are present, exactly as
//! `ultimate_wilderness::feat_tables`' own projection does for the same corpus
//! shape) and is `None` when the record carries neither. `prerequisites` is
//! every top-level `PRE`-family token verbatim, and `None` — never `Some(&[])`
//! — when the record carries none.
//!
//! `category` is the corpus `TYPE:` token's **first dot-segment, verbatim**,
//! not a per-book `FeatCategory` enum variant. That is a deliberate
//! difference from the hand-authored tables and it is the honest one: a gap
//! row has no per-book table to take a variant name from, and mapping the
//! corpus facet onto some book's enum would invent a classification the
//! corpus never made. `feats_all::FeatCatalogRecord::category` is already a
//! `&'static str` for precisely this reason (see its doc comment).
//!
//! Run it with a local PCGen corpus checkout:
//!
//! ```text
//! PCGEN_CORPUS_ROOT="$HOME/workspace/repos/pcgen/data" \
//!   cargo run --locked --bin gen_feat_gap_tables
//! ```

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use codex::rules_core::pcgen_desc;
use codex::rules_core::pi_screening::{self, declared_product_identity};
use codex::rules_core::pi_table_sweep::screen_generated_table;
use codex::rules_core::rules_tables::feats_all::hand_authored_feat_tables;
use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::shape_b_v1::{License, REDACTED_PI_MARKER};

/// Where the generated table lands, relative to the crate root.
const OUTPUT_RELATIVE_PATH: &str = "src/rules_core/rules_tables/feat_gap_tables.rs";

/// One book's gap-lane inputs: the `RuleSetId` the joined catalog files its
/// records under, the `RuleSetId` variant name to emit in generated source,
/// and each `.lst` path relative to the corpus root.
struct BookInput {
    rule_set: RuleSetId,
    variant: &'static str,
    slug: &'static str,
    files: &'static [&'static str],
}

/// Every book that carries at least one `not-ingested` feat unit, with the
/// exact files those units come from. Derived from
/// `docs/work-inventory.json`'s own `source_file` field over the
/// `status == "not-ingested"` feat population — not guessed from a directory
/// glob, so a file with no gap is never re-parsed and cannot introduce a row
/// nobody asked for.
const BOOK_INPUTS: &[BookInput] = &[
    BookInput {
        rule_set: RuleSetId::Crb,
        variant: "Crb",
        slug: "core_rulebook",
        files: &["pathfinder/paizo/roleplaying_game/core_rulebook/cr_feats.lst"],
    },
    BookInput {
        rule_set: RuleSetId::Ce,
        variant: "Ce",
        slug: "core_essentials",
        // **Corrected `SD31-E6-F8-001`** — this was previously filed under
        // `RuleSetId::Crb` on the theory that `core_rulebook.pcc`'s
        // unconditional include of `core_essentials` made CRB the "observed
        // host". That predates `RuleSetId::Ce` existing as its own compiled
        // rule set (added later for companion/familiar content); now that it
        // does, `classify()`'s feat arm resolves a `core_essentials`-directory
        // record's `engine_book` straight from its own `source_book` field
        // ("core_essentials" -> `RuleSetId::Ce`, `own_engine_book`, never the
        // CRB shared-library-host fallback) — so filing these 15 records
        // under Crb left them permanently unreachable through the ONLY path
        // `classify()` actually checks, regardless of which real-world book
        // Decision 9's separate content re-attribution says they belong to
        // (`bestiary`, per that decision's SOURCELONG join — a different
        // question from which RULE SET serves them at chargen).
        files: &["pathfinder/paizo/roleplaying_game/core_essentials/ce_feats.lst"],
    },
    BookInput {
        rule_set: RuleSetId::Arg,
        variant: "Arg",
        slug: "advanced_race_guide",
        files: &["pathfinder/paizo/roleplaying_game/advanced_race_guide/arg_feats.lst"],
    },
    BookInput {
        rule_set: RuleSetId::Uc,
        variant: "Uc",
        slug: "ultimate_combat",
        files: &["pathfinder/paizo/roleplaying_game/ultimate_combat/uc_feats.lst"],
    },
    BookInput {
        rule_set: RuleSetId::Ui,
        variant: "Ui",
        slug: "ultimate_intrigue",
        // `support/ui_feats_oa.lst` is loaded by `ultimate_intrigue.pcc`
        // behind `PRECAMPAIGN:1,Occult Adventures` — the gate is on the pcc
        // LOAD LINE, not inside the `.lst` (a `grep PRECAMPAIGN` over the
        // file itself returns 0). It is included here anyway, and the reason
        // is that the gate governs which CAMPAIGN activates the file, not
        // which BOOK ships it: these three records are Ultimate Intrigue's
        // own, `v06_work_inventory` enumerates them as `ultimate_intrigue`
        // units, and this engine models no campaign activation to gate on.
        // Filing them under `Ui` is the accurate book attribution; excluding
        // them would leave three units `not-ingested` forever with no lane
        // that could ever close them.
        files: &["pathfinder/paizo/roleplaying_game/ultimate_intrigue/support/ui_feats_oa.lst"],
    },
    BookInput {
        rule_set: RuleSetId::Um,
        variant: "Um",
        slug: "ultimate_magic",
        files: &[
            "pathfinder/paizo/roleplaying_game/ultimate_magic/um_feats.lst",
            "pathfinder/paizo/roleplaying_game/ultimate_magic/um_feats_wordsofpower.lst",
        ],
    },
    BookInput {
        rule_set: RuleSetId::Upsi,
        variant: "Upsi",
        slug: "ultimate_psionics",
        files: &["pathfinder/dreamscarred_press/ultimate_psionics/up_feats.lst"],
    },
    BookInput {
        rule_set: RuleSetId::Uw,
        variant: "Uw",
        slug: "ultimate_wilderness",
        files: &["pathfinder/paizo/roleplaying_game/ultimate_wilderness/uw_feats.lst"],
    },
    // `SD31-E6-F8-002` -- five books already compiled into `COMPILED_RULE_SETS`
    // for another kind (race_trait and/or monster) that never had a feat
    // table at all. `docs/work-inventory.json`'s `feat` units for these books
    // carry `evidence: "feat_key_absent_from_catalog"` (a started book with no
    // feat table), never `no_compiled_rule_set_for_book` -- re-derived,
    // `python3` over `docs/work-inventory.json`, 2026-08-16 -- so, unlike
    // `mythic_adventures`/`adventurers_guide`, closing this population needs
    // no new `RuleSetId` and no `v06_work_inventory.rs` edit, only a gap-row
    // book here plus the matching empty `hand_authored_feat_tables()` slice
    // (`feats_all.rs`) so `all_feat_tables()` joins them.
    BookInput {
        rule_set: RuleSetId::Ha,
        variant: "Ha",
        slug: "horror_adventures",
        files: &["pathfinder/paizo/roleplaying_game/horror_adventures/ha_feats.lst"],
    },
    BookInput {
        rule_set: RuleSetId::Isr,
        variant: "Isr",
        slug: "inner_sea_races",
        files: &["pathfinder/paizo/campaign_setting/inner_sea_races/isr_feats.lst"],
    },
    BookInput {
        rule_set: RuleSetId::Oa,
        variant: "Oa",
        slug: "occult_adventures",
        files: &["pathfinder/paizo/roleplaying_game/occult_adventures/oa_feats.lst"],
    },
    BookInput {
        rule_set: RuleSetId::Iswg,
        variant: "Iswg",
        slug: "inner_sea_world_guide",
        files: &["pathfinder/paizo/campaign_setting/inner_sea_world_guide/iswg_feats.lst"],
    },
    BookInput {
        rule_set: RuleSetId::MonsterCodex,
        variant: "MonsterCodex",
        slug: "monster_codex",
        files: &["pathfinder/paizo/roleplaying_game/monster_codex/mc_feats.lst"],
    },
    // `SD31-E6-F2-007` -- Mythic Adventures' first compiled rule set of any
    // kind (named in this generator's own doc comment above as an example
    // needing a new `RuleSetId` and a `v06_work_inventory.rs` edit; both
    // now done). Only `ma_feats.lst`'s 358 non-`.MOD` declarations are
    // parsed here -- `parse_lst`'s existing `.MOD` skip already excludes
    // the file's 208 `.MOD` rows, which target `race_trait`-kind base
    // records elsewhere in the corpus (e.g. `Android ~ Vision`), not a
    // feat this table could honestly hold (`RuleSetId::Mythic`'s own doc
    // comment; reported, not ingested, `OPEN-ISSUES.md`).
    BookInput {
        rule_set: RuleSetId::Mythic,
        variant: "Mythic",
        slug: "mythic_adventures",
        files: &["pathfinder/paizo/roleplaying_game/mythic_adventures/ma_feats.lst"],
    },
    // `SD31-E6-F8-003` -- two more books already compiled into
    // `COMPILED_RULE_SETS` for another kind (`Isi`: familiars + abilities,
    // `Botd2`: monsters) that never had a feat table of their own, same
    // shape as the five-book lane above. `docs/work-inventory.json`'s
    // remaining `feat` `not-ingested` population for these two books is
    // `origin: "declared"` (real standalone records, not a `.MOD`-only
    // artifact) and neither file's records carry `NAMEISPI:YES`, re-derived
    // by direct read of both files.
    BookInput {
        rule_set: RuleSetId::Isi,
        variant: "Isi",
        slug: "inner_sea_intrigue",
        files: &["pathfinder/paizo/campaign_setting/inner_sea_intrigue/isi_feats.lst"],
    },
    BookInput {
        rule_set: RuleSetId::Botd2,
        variant: "Botd2",
        slug: "book_of_the_damned_volume_2",
        files: &[
            "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_2/botd2_feats.lst",
        ],
    },
    // SD-32 Gate 0 book-onboarding precondition (`gate-0-book-onboarding-
    // precondition`, AT-32-G0-003) -- Inner Sea Taverns' first compiled
    // rule set of any kind, the same shape `RuleSetId::Mythic` above
    // established (new `RuleSetId`, no hand-authored table, first record
    // family delivered entirely by this generator). Only `istav_feats.lst`'s
    // 9 non-`.MOD` declarations are parsed here -- this file has no `.MOD`
    // rows at all (re-derived: `grep -c '\.MOD' istav_feats.lst` -> 0).
    BookInput {
        rule_set: RuleSetId::InnerSeaTaverns,
        variant: "InnerSeaTaverns",
        slug: "inner_sea_taverns",
        files: &["pathfinder/paizo/campaign_setting/inner_sea_taverns/istav_feats.lst"],
    },
    // SD-32 T9 onboarding (card 11) -- `decisions.md §19` PI sign-off cleared
    // this book's feat population. `RuleSetId::Isc` already exists and is
    // already wired into `COMPILED_RULE_SETS`/`engine_book_for` (added for
    // this book's equipment/monster content, `v06_work_inventory.rs`), so
    // this is a pure config addition: no new `RuleSetId`, no `v06_work_
    // inventory.rs` edit. Re-derived directly against `docs/work-
    // inventory.json`'s own `feat_key_absent_from_catalog` evidence for this
    // book: every one of its 24 not-ingested feat units carries `CATEGORY:
    // FEAT` in the raw row (checked by direct read, not assumed) -- genuine
    // player feats, not the `.MOD`/`VISIBLE:EXPORT` continuation shape found
    // blocking `horror_adventures`/`mythic_adventures` (see this cycle's own
    // receipt for that finding).
    BookInput {
        rule_set: RuleSetId::Isc,
        variant: "Isc",
        slug: "inner_sea_combat",
        files: &["pathfinder/paizo/campaign_setting/inner_sea_combat/isc_abilities_feat.lst"],
    },
    // Same shape as `Isc` immediately above: `RuleSetId::Isg` already exists
    // and is already wired (added for this book's equipment/monster
    // content). Re-derived directly: every one of this book's not-ingested
    // feat units carries `CATEGORY:FEAT` in the raw row.
    BookInput {
        rule_set: RuleSetId::Isg,
        variant: "Isg",
        slug: "inner_sea_gods",
        files: &["pathfinder/paizo/campaign_setting/inner_sea_gods/isg_abilities_feat.lst"],
    },
];

/// One parsed corpus feat record, before the already-held filter runs.
struct ParsedRecord {
    key: String,
    name: String,
    category: String,
    description: Option<String>,
    prerequisites: Vec<String>,
    /// SD-30 contract 53.5, read off this row's own `NAMEISPI:YES` token. A
    /// record whose NAME is declared Product Identity cannot be published
    /// at all -- its identity cannot be redacted -- so a caller must drop
    /// the whole record rather than emit it.
    name_is_pi: bool,
}

fn tab_fields(line: &str) -> Vec<&str> {
    line.split('\t').collect()
}

fn token_value<'a>(fields: &[&'a str], token: &str) -> Option<&'a str> {
    fields.iter().find_map(|f| f.trim().strip_prefix(token))
}

fn has_token(fields: &[&str], token: &str) -> bool {
    fields.iter().any(|f| f.trim().starts_with(token))
}

/// Parse one feat `.lst` under exactly `v06_work_inventory::enumerate_file`'s
/// record predicate for `Kind::Feat`, including its
/// `has_classifying_token(Kind::Feat, ..) == has_token("TYPE:")` requirement.
/// Kept as a standalone function so its agreement with that enumerator is
/// testable rather than asserted.
fn parse_lst(text: &str) -> Vec<ParsedRecord> {
    let mut out = Vec::new();
    for line in text.lines() {
        let fields = tab_fields(line);
        let Some(first) = fields.first() else { continue };
        let first = first.trim();
        if first.is_empty() || first.starts_with('#') {
            continue;
        }
        // An ALL-CAPS `TOKEN:` first field is file metadata, not a record.
        // (`CLASS:` is the enumerator's one exception and cannot occur in a
        // feat file.)
        let is_directive = first
            .split_once(':')
            .map(|(head, _)| {
                !head.is_empty()
                    && head.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
            })
            .unwrap_or(false);
        if is_directive && !first.starts_with("CLASS:") {
            continue;
        }
        if first.starts_with("CATEGORY=Internal|")
            || fields.iter().any(|f| f.trim() == "CATEGORY:Internal")
        {
            continue;
        }
        // `.MOD` rows are overlays onto an existing record, never a new one.
        if first.contains(".MOD") {
            continue;
        }
        // SD31-W10-INTEGRATE-001 fix for a CONFIRMED review finding: a
        // `VISIBLE:EXPORT` row is PCGen's own export-plumbing twin of a real,
        // player-selectable record -- it exists so a character sheet PDF can
        // print the auto-granted feat's name/benefit text, never so a player
        // can select it independently. `mythic_adventures/ma_feats.lst`
        // carries 159 such twins (e.g. `Dual Path` at CATEGORY:Mythic Feat
        // with `PREVARGTEQ:MythicTierLevel,1`, granted automatically, sits
        // alongside `KEY:Mythic Feat Output ~ Dual Path` at CATEGORY:FEAT
        // VISIBLE:EXPORT, identical DESC/BENEFIT, ZERO `PRE*` token).
        // Shipping the twin into this generator's output puts it in the
        // player-facing Add Feat picker as an independently selectable,
        // ungated duplicate -- bypassing the mythic-tier gate the real row
        // enforces (proven live: a level-1 non-mythic search for "Accursed
        // Hex" surfaced both the correctly-gated real feat AND a second,
        // fully-selectable "Accursed Hex (Mythic)"). Skipping it here means
        // its unit correctly lands in the `not-ingested` gap population
        // instead of a bogus `done` credit -- it is real PCGen content, not
        // retracted, just not independently selectable, matching every
        // other `VISIBLE:EXPORT`/`VISIBLE:DISPLAY`-shaped exclusion this
        // codebase already makes for non-player-facing rows.
        if fields.iter().any(|f| f.trim() == "VISIBLE:EXPORT") {
            continue;
        }
        let name = if let Some((_, variant)) = first.split_once(".COPY=") {
            variant.to_string()
        } else if let Some(rest) =
            first.strip_prefix("CATEGORY=").and_then(|r| r.split_once('|')).map(|(_, r)| r)
        {
            rest.to_string()
        } else {
            first.to_string()
        };
        // `has_classifying_token(Kind::Feat, ..)`: a feat row without a
        // `TYPE:` token is a sub-choice helper, not an independent record.
        if !has_token(&fields, "TYPE:") {
            continue;
        }
        let key = token_value(&fields, "KEY:")
            .map(|k| k.to_string())
            .unwrap_or_else(|| name.clone());

        let category = token_value(&fields, "TYPE:")
            .and_then(|t| t.split('.').next())
            .map(str::trim)
            .filter(|c| !c.is_empty())
            .unwrap_or("General")
            .to_string();

        let desc = token_value(&fields, "DESC:").map(str::trim).filter(|d| !d.is_empty());
        let benefit = token_value(&fields, "BENEFIT:").map(str::trim).filter(|d| !d.is_empty());
        let mut description = match (desc, benefit) {
            // Both fields present and different: each is rendered through
            // `pcgen_desc::render_pcgen_desc` INDEPENDENTLY before the join,
            // not after. `inner_sea_world_guide`'s `Godless Healing` carries
            // a real PCGen `%1/day...|GodlessHealingTimes` argument tail on
            // BOTH its `DESC:` and its `BENEFIT:` token; joining the two RAW
            // strings first (as the single-field arms below still do) would
            // strand the first field's tail in the MIDDLE of the joined
            // text, where `apps/desktop`'s own serve-time `render_pcgen_desc`
            // call -- which can only strip a *trailing* tag -- ships it
            // verbatim (`SD31-E6-F8-002`, caught live by `feat_catalog::
            // feat_descriptions_are_rendered_and_otherwise_byte_identical`).
            // A single field carries no such risk (nothing else to strand
            // it against) and is left exactly as raw as before, so the
            // single-field population's existing "raw, rendered once at
            // serve time" contract (`feat_catalog.rs`'s own pinned counts)
            // is unchanged by this fix.
            (Some(d), Some(b)) if d != b => Some(format!(
                "{} {}",
                pcgen_desc::render_pcgen_desc(d).text,
                pcgen_desc::render_pcgen_desc(b).text
            )),
            (Some(d), _) => Some(d.to_string()),
            (None, Some(b)) => Some(b.to_string()),
            (None, None) => None,
        };

        // SD-30 PI screen, both invocation contracts, unioned -- this
        // generator previously ran ONLY contract 52.3 (the blacklist scan,
        // over the whole generated file, below in `main`) and never
        // contract 53.5 (the row's own `NAMEISPI:YES`/`DESCISPI:YES`
        // declaration). Read it here, off the same `fields` already parsed,
        // so a caller can drop a name-PI record before it ever reaches the
        // generated table.
        let token_pairs: Vec<(&str, &str)> = fields
            .iter()
            .filter_map(|f| f.trim().split_once(':'))
            .collect();
        let declared = declared_product_identity(token_pairs);
        let name_is_pi = declared.name;
        if let Some(d) = &description {
            let (blacklist_license, ..) = pi_screening::classify_field("description", d);
            let blacklisted = blacklist_license != License::Ogl;
            if declared.description || blacklisted {
                description = Some(REDACTED_PI_MARKER.to_string());
            }
        }

        // Every top-level `PRE`-family token, verbatim and in source order,
        // including the negated `!PRE...` form — the same set
        // `feats_all::FeatCatalogRecord::prerequisites` documents.
        //
        // Contract 52.3's blacklist scan, applied per-token: a `PRETEXT:`
        // (free narrative prose, unlike the other structured `PRE*` tokens)
        // can name a Golarion setting term (`inner_sea_races`'s `Varisian`/
        // `Garundi`/`Mwangi`/`Vudrani` ethnicities, found live this cycle,
        // `SD31-E6-F8-002`) the whole-generated-file sweep below in `main`
        // would otherwise HARD STOP on with no per-record remedy. Redacted in
        // place, the same treatment `description` already gets a few lines
        // up — a prerequisite is mechanics text, not the record's identity,
        // so this is the `description` shape, never the `NAMEISPI` drop-the-
        // record shape.
        let prerequisites: Vec<String> = fields
            .iter()
            .map(|f| f.trim())
            .filter(|f| f.starts_with("PRE") || f.starts_with("!PRE"))
            .map(|f| {
                let (blacklist_license, ..) = pi_screening::classify_field("prerequisite", f);
                if blacklist_license != License::Ogl {
                    match f.split_once(':') {
                        Some((token, _)) => format!("{token}:{REDACTED_PI_MARKER}"),
                        None => REDACTED_PI_MARKER.to_string(),
                    }
                } else {
                    f.to_string()
                }
            })
            .collect();

        out.push(ParsedRecord { key, name, category, description, prerequisites, name_is_pi });
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

/// The corpus checkout, from the environment only.
///
/// No default and no tilde expansion: `tests/no_foreign_home_paths.rs` treats
/// both an absolute `/home/<someone>` literal and an unexpanded `~` default in
/// Rust source as failures, and it is right to — a baked-in path is one
/// machine's truth shipped as everyone's. Same variable
/// `gen_equipment_gap_tables` and `pathfinder_unchained::monk_features` use.
fn corpus_root() -> PathBuf {
    PathBuf::from(
        std::env::var("PCGEN_CORPUS_ROOT")
            .expect("PCGEN_CORPUS_ROOT must point at a local pcgen/data checkout"),
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = corpus_root();
    if !root.is_dir() {
        return Err(
            format!("PCGEN_CORPUS_ROOT does not point at a directory: {}", root.display()).into()
        );
    }

    // What the hand-authored tables already hold, per rule set — both the
    // record KEY and its display NAME, because `v06_work_inventory`'s own
    // check is `set.contains(unit.key) || set.contains(unit.name)` against a
    // set built from `entry.key` alone, then tested against both. Mirroring
    // that asymmetry exactly is what makes this generator's output set
    // precisely the classifier's `not-ingested` set; the equipment lane
    // recorded a 28-row divergence from getting it wrong in the other
    // direction, so it is checked here by count, not by assertion.
    //
    // Keyed on the `RuleSetId`'s `Debug` name rather than the value itself:
    // `RuleSetId` is deliberately not `Ord`, and inventing an ordering for it
    // here just to key a map would be a change to a shared type made for one
    // binary's convenience.
    let mut held: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for book in hand_authored_feat_tables() {
        let set = held.entry(format!("{:?}", book.rule_set)).or_default();
        for entry in book.entries {
            set.insert(entry.key.to_string());
        }
    }

    let mut body = String::new();
    let mut totals: Vec<(&str, usize)> = Vec::new();
    let mut name_pi_dropped: usize = 0;

    for input in BOOK_INPUTS {
        let mut rows: Vec<ParsedRecord> = Vec::new();
        // Deduped on the record IDENTITY (`key`) alone: once a key is in the
        // book's catalog set every unit carrying it classifies as ingested
        // regardless of display name, so a second row would add nothing but a
        // duplicate catalog entry.
        let mut seen: BTreeSet<String> = BTreeSet::new();
        for rel in input.files {
            let path = root.join(rel);
            let text =
                std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            for record in parse_lst(&text) {
                let already = held
                    .get(&format!("{:?}", input.rule_set))
                    .map(|s| s.contains(&record.key) || s.contains(&record.name))
                    .unwrap_or(false);
                if already {
                    continue;
                }
                if !seen.insert(record.key.clone()) {
                    continue;
                }
                // SD-30 contract 53.5 hard stop: a NAMEISPI:YES record's
                // identity cannot be redacted, so it is never emitted.
                if record.name_is_pi {
                    name_pi_dropped += 1;
                    continue;
                }
                rows.push(record);
            }
        }

        totals.push((input.slug, rows.len()));
        writeln!(
            body,
            "\n/// {} — {} record(s) the hand-authored `{}` feat table does not hold.\npub static {}_FEAT_GAP_ROWS: &[FeatCatalogRecord] = &[",
            input.slug,
            rows.len(),
            input.slug,
            input.slug.to_uppercase()
        )?;
        for row in &rows {
            let prerequisites = if row.prerequisites.is_empty() {
                "None".to_string()
            } else {
                format!(
                    "Some(&[{}])",
                    row.prerequisites.iter().map(|p| rust_string(p)).collect::<Vec<_>>().join(", ")
                )
            };
            writeln!(
                body,
                "    FeatCatalogRecord {{ key: {}, category: {}, name: {}, description: {}, prerequisites: {} }},",
                rust_string(&row.key),
                rust_string(&row.category),
                rust_string(&row.name),
                match &row.description {
                    Some(d) => format!("Some({})", rust_string(d)),
                    None => "None".to_string(),
                },
                prerequisites
            )?;
        }
        writeln!(body, "];")?;
    }

    let total: usize = totals.iter().map(|(_, n)| *n).sum();
    if name_pi_dropped > 0 {
        eprintln!("gen_feat_gap_tables: {name_pi_dropped} record(s) dropped -- NAMEISPI:YES declared");
    }
    let mut header = String::new();
    writeln!(
        header,
        "//! Corpus `feat` records that belong to an ALREADY-COMPILED book whose\n\
         //! hand-authored per-book feat table does not hold them — the\n\
         //! `not-ingested` population of `docs/work-inventory.json`'s `feat` kind,\n\
         //! closed corpus-wide.\n\
         //!\n\
         //! **GENERATED — do not edit by hand.** Regenerate with\n\
         //! `PCGEN_CORPUS_ROOT=<pcgen>/data cargo run --locked --bin gen_feat_gap_tables`.\n\
         //! The generator applies `v06_work_inventory`'s own record predicate for\n\
         //! `Kind::Feat`, so a row here is exactly a row that inventory reported\n\
         //! `not-ingested`.\n\
         //!\n\
         //! `description` is the record's `DESC:` joined with its `BENEFIT:` when\n\
         //! both are present, and `None` when it carries neither — never a\n\
         //! fabricated placeholder. `prerequisites` is every top-level `PRE`-family\n\
         //! token verbatim, and `None` (never `Some(&[])`) when there are none.\n\
         //! `category` is the corpus `TYPE:` token's first dot-segment verbatim,\n\
         //! NOT a per-book `FeatCategory` variant name: a gap row has no per-book\n\
         //! table to take a variant from, and mapping the corpus facet onto some\n\
         //! book's enum would invent a classification the corpus never made.\n\
         //!\n\
         //! Total: {total} rows.\n"
    )?;
    writeln!(header, "use super::feats_all::FeatCatalogRecord;\nuse super::RuleSetId;\n")?;
    writeln!(
        header,
        "/// The gap rows for one rule set, or an empty slice when that book has\n\
         /// none. Chained AFTER the book's hand-authored records by\n\
         /// `feats_all::all_feat_tables`, so a first-match key lookup keeps\n\
         /// resolving to the hand-authored record.\n\
         pub fn feat_gap_rows_for(rule_set: RuleSetId) -> &'static [FeatCatalogRecord] {{\n\
         \x20   match rule_set {{\n\
         {}\
         \x20       _ => &[],\n\
         \x20   }}\n\
         }}",
        BOOK_INPUTS
            .iter()
            .map(|b| format!(
                "        RuleSetId::{} => {}_FEAT_GAP_ROWS,\n",
                b.variant,
                b.slug.to_uppercase()
            ))
            .collect::<String>()
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
    Ok(())
}

#[cfg(test)]
mod pi_screen_tests {
    use super::*;

    /// SD-30 contract 53.5, mutation-proven live against `parse_lst`: a row
    /// declaring `NAMEISPI:YES` must be flagged `name_is_pi` so the caller
    /// can drop it, never silently emitted.
    #[test]
    fn parse_lst_flags_a_nameispi_declared_row() {
        let text = "Secret Trick\tTYPE:General\tNAMEISPI:YES\tDESC:A trick.\n";
        let records = parse_lst(text);
        assert_eq!(records.len(), 1);
        assert!(records[0].name_is_pi, "a NAMEISPI:YES row must be flagged for the caller to drop");
    }

    #[test]
    fn parse_lst_does_not_flag_an_undeclared_row() {
        let text = "Ordinary Trick\tTYPE:General\tDESC:Nothing special.\n";
        let records = parse_lst(text);
        assert_eq!(records.len(), 1);
        assert!(!records[0].name_is_pi);
    }

    /// SD31-W10-INTEGRATE-001 (CONFIRMED review finding): a `VISIBLE:EXPORT`
    /// row is PCGen's own display-plumbing twin of an auto-granted feat, not
    /// a second, independently selectable one. Skipping it here is what
    /// keeps it out of the player-facing Add Feat picker without inventing
    /// a `PRE` token it never carried.
    #[test]
    fn parse_lst_skips_a_visible_export_row() {
        let text = "Dual Path\tKEY:Mythic Feat Output ~ Dual Path\tCATEGORY:FEAT\tTYPE:Mythic\tVISIBLE:EXPORT\tDESC:You follow two mythic paths.\n";
        let records = parse_lst(text);
        assert!(records.is_empty(), "a VISIBLE:EXPORT row must never reach the catalog, got {} record(s)", records.len());
    }

    /// The real, gated row (no `VISIBLE:EXPORT`) is untouched by the same
    /// fix -- this is a targeted skip, not a blanket Mythic-feat filter.
    #[test]
    fn parse_lst_still_parses_the_real_gated_sibling_row() {
        let text = "Dual Path\tCATEGORY:Mythic Feat\tTYPE:Mythic\tPREVARGTEQ:MythicTierLevel,1\tDESC:You follow two mythic paths.\n";
        let records = parse_lst(text);
        assert_eq!(records.len(), 1, "the real, ungated-by-this-fix sibling row must still parse");
    }

    /// Contract 53.5's other half: `DESCISPI:YES` redacts the description in
    /// place rather than shipping the declared-PI prose verbatim.
    #[test]
    fn parse_lst_redacts_a_descispi_declared_description() {
        let text = "Hidden Trick\tTYPE:General\tDESCISPI:YES\tDESC:A trick of great secrecy.\n";
        let records = parse_lst(text);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].description.as_deref(), Some(REDACTED_PI_MARKER));
    }

    /// Contract 52.3, applied per-record rather than only over the whole
    /// generated file: a description that hits the shared blacklist term
    /// list is redacted even with no `NAMEISPI`/`DESCISPI` declaration.
    #[test]
    fn parse_lst_redacts_a_description_that_hits_the_blacklist() {
        let text = "Blessed Trick\tTYPE:General\tDESC:Granted by Iomedae herself.\n";
        let records = parse_lst(text);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].description.as_deref(), Some(REDACTED_PI_MARKER));
    }

    /// Contract 52.3's blanket blacklist scan applied to a THIRD field this
    /// generator carries: `prerequisites`. `SD31-E6-F8-002` found this gap
    /// live — `inner_sea_races/isr_feats.lst`'s `PRETEXT:` prerequisites name
    /// Golarion setting terms (`Varisian`, `Garundi`, `Mwangi`, `Vudrani`) the
    /// generator's own whole-file `screen_generated_table` sweep correctly
    /// caught (HARD STOP, nothing written) but `parse_lst` had no per-field
    /// redaction for, unlike `description`. A blacklisted PRE-family token is
    /// redacted in place, mirroring `description`'s own treatment — it is
    /// prerequisite mechanics text, not the record's identity, so this is the
    /// `description` shape (redact), never the `NAMEISPI` shape (drop the
    /// whole record).
    #[test]
    fn parse_lst_redacts_a_prerequisite_that_hits_the_blacklist() {
        let text = "Deadly Troupe\tTYPE:Teamwork\tPRETEXT:human (Varisian).\tDESC:Ok.\n";
        let records = parse_lst(text);
        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0].prerequisites,
            vec![format!("PRETEXT:{REDACTED_PI_MARKER}")],
            "a blacklisted PRETEXT token must be redacted, not shipped verbatim"
        );
    }

    /// The sibling case: an ordinary prerequisite with no blacklist hit must
    /// pass through byte-for-byte, so the redaction cannot become a blanket
    /// PRETEXT-stripping shortcut.
    #[test]
    fn parse_lst_does_not_redact_an_ordinary_prerequisite() {
        let text = "Ordinary Feat\tTYPE:General\tPRESTAT:1,STR=13\tDESC:Ok.\n";
        let records = parse_lst(text);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].prerequisites, vec!["PRESTAT:1,STR=13".to_string()]);
    }

    /// The real defect `SD31-E6-F8-002` found: `inner_sea_world_guide`'s
    /// `Godless Healing` carries BOTH a `DESC:` and a `BENEFIT:` token, each
    /// independently ending in a real PCGen `%1/day...|GodlessHealingTimes`
    /// argument tail (`BONUS:VAR|GodlessHealingTimes|1` backs the `%1`).
    /// Naively joining the two RAW strings with `format!("{d} {b}")` leaves
    /// the FIRST field's tail sitting in the MIDDLE of the joined text —
    /// `apps/desktop`'s serve-time `render_pcgen_desc` call can only strip a
    /// *trailing* tag, so the middle one shipped verbatim
    /// (`feat_catalog::feat_descriptions_are_rendered_and_otherwise_byte_
    /// identical` caught it live: "served feat \"Godless Healing\" still
    /// leaks"). Each field must be rendered (and its own tail resolved/
    /// dropped, per `pcgen_desc`'s no-fabrication rule — this generator has
    /// no character to compute a real value against) BEFORE the join, not
    /// after.
    #[test]
    fn parse_lst_never_leaves_a_pipe_tail_stranded_mid_string_when_desc_and_benefit_both_carry_one()
    {
        let text = "Godless Healing\tTYPE:General\t\
                     DESC:Heal yourself 1d8+(TL) hp %1/day.|GodlessHealingTimes\t\
                     BENEFIT:%1/day you may heal yourself.|GodlessHealingTimes\n";
        let records = parse_lst(text);
        assert_eq!(records.len(), 1);
        let description = records[0].description.as_deref().expect("has a description");
        assert_eq!(
            codex::rules_core::pcgen_desc::leaked_pcgen_syntax(description),
            None,
            "joined description must not leak raw PCGen syntax: {description:?}"
        );
        assert_eq!(
            description, "Heal yourself 1d8+(TL) hp /day. /day you may heal yourself.",
            "each field's own %1 is dropped in place (no character to resolve it against) and its tail stripped, independently, before the join"
        );
    }
}
