//! Player-facing reference surface for PF1's intelligent/legendary item
//! build system (SD-31 wave-18, `intelligent_items:desktop` lane; converted
//! to the sheet-rule package by SD-35 `AT-35-E6-003` cycle 11).
//!
//! # Why this module exists
//!
//! An intelligent item's own ability scores, Ego and alignment are real,
//! fully ingested corpus content -- 171 `equipmods` records across
//! `core_rulebook` (the classic Intelligent Item system, CRB p.172-174) and
//! `mythic_adventures` (the parallel Legendary Item system, Mythic
//! Adventures p.172) -- but before this module, nothing in `apps/desktop`
//! rendered them. This module is that missing render path.
//!
//! # Where the words and the numbers come from
//!
//! `decisions.md §11` -- nothing on the live side reads the ingest format.
//! Until cycle 11 this module did: it parsed the record's own token array
//! for the hidden-row flag, its description token for prose, its bonus
//! chains for mechanics and the qualifier tail on each chain for the
//! condition. Every one of those readings now happens once, at ingest, in
//! `src/pcgen_import/sheet_rule/`, and this module reads the converted
//! [`SheetRule`] instead:
//!
//! | what the screen shows | where it comes from now |
//! |---|---|
//! | served vs hidden | [`SheetRule::print`] |
//! | description | [`codex::rules_core::sheet_rule_catalog::catalog_description`] |
//! | mechanics | the package's [`VarTable`] contributions, reverse-indexed by rule id |
//! | which value a mechanic moves | [`VarTable::label`] |
//! | a mechanic's condition | [`codex::rules_core::level_up_option_filter::describe_gate`] |
//!
//! The record's own *non-rules* fields -- book directory, `KEY`, name, price
//! -- are still read from `data/corpus/`, because that is where they live
//! and none of them is a rule: they are the identity of the row this screen
//! is a catalog of. The join between the two is the source row **both sides
//! record**: the corpus record's `source.path:line` against the converted
//! rule's `provenance.closure_rows[0]`. Checked, not assumed, by
//! `every_corpus_record_joins_a_converted_rule_on_its_own_source_row` below
//! -- all 171 records, zero misses.
//!
//! # Source of truth: the live corpus, not a fixture
//!
//! Reads every `data/corpus/<book>/equipment/equipmods/*.json` record whose
//! `data.key` contains the literal substring `"Intelligent Item"` --
//! `core_rulebook`'s own `Intelligent Item ~ *` / `Intelligent Item
//! Alignment (*)` / `Intelligent Item Purpose (*)` keys and
//! `mythic_adventures`'s `Legendary Item ~ Intelligent Item...` keys all
//! carry it. Matching on the corpus's own semantic identity rather than a
//! filename convention means a future book that adds more such records is
//! picked up automatically, the same posture
//! `class_feature_descriptions.rs` takes reading `data.class`.
//!
//! # Hidden trigger rows are read but never served as options
//!
//! 19 of the 171 records are bookkeeping shadows of their spelled-out,
//! purchasable sibling (e.g. `Intelligent Item Alignment (LG)`, hidden, sits
//! beside `Intelligent Item ~ Alignment / Lawful Good`, the real EQBUILDER
//! choice). A player never sees the hidden trigger as a choice in PCGen's
//! own item builder either, so serving it here as if it were a fourth
//! "alignment option" would misrepresent the corpus, not merely omit detail.
//! The converter decides which those are -- following the source row's own
//! visibility, including the `.COPY=` precedence cycle 10 fixed -- and this
//! module drops every record no rule of which prints. **That is two more
//! than the token reading found**: `Intelligent Item Purpose (Slay All)` and
//! `Intelligent Item Purpose (Slay Creature Type)` state their hidden
//! visibility on a row the ingested token array does not carry, so the old
//! reading served two bookkeeping rows as if they were purchasable choices.
//! See `hidden_trigger_rows_never_reach_the_served_catalog`.
//!
//! # No fabricated Ego score
//!
//! An item's total Ego score is `sum of every chosen component's Ego
//! contribution`, and which components are chosen is a build-time choice
//! this corpus does not fix for any specific item -- exactly the runtime-
//! context-the-corpus-does-not-fix shape `docs/release/
//! SD-31-corpus-closure-grind` names for monster spell-like abilities. This
//! module ships the contribution every component states (a literal integer
//! for most rows; for the shared Base row, the price-bracket band table read
//! off the converted expression itself -- see [`format_price_band_ladder`])
//! and never a resolved total. Pinned by
//! `no_component_ever_emits_a_fabricated_resolved_total_ego_score` and
//! mutation-proved by `mutation_removing_the_ego_delta_none_guard_would_be_
//! caught_by_the_pin` (both below).
//!
//! # PI screening
//!
//! Every one of the records carries `pi_field: null` / `pi_marker: null` in
//! the live corpus (checked, not assumed --
//! `every_served_record_carries_no_declared_pi_marker`, below) and every
//! rendered `name`/`description` is checked against
//! `codex::rules_core::pi_screening::PI_BLACKLIST_TERMS`, the same live term
//! list `reach_gate.rs` checks served Inner Sea World Guide content against
//! -- `every_served_name_and_description_clears_the_pi_blacklist`, below.
//!
//! # The leak guard
//!
//! The converter resolves the record's words at ingest, so there is no
//! substitution left to leak here. The guard stays anyway, one level down:
//! `every_served_description_renders_without_a_pcgen_syntax_leak` checks
//! every served description against `leaked_pcgen_syntax`, which is now a
//! statement about the **package** rather than about this module's own
//! rendering -- a converter regression that started shipping unresolved
//! syntax would fail here.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde::Serialize;
use serde_json::Value;

use codex::rules_core::level_up_option_filter::{describe_gate, expr_words};
use codex::rules_core::sheet_rule::{Applies, Expr, SheetRule, SheetRulePackage, VarId};
use codex::rules_core::sheet_rule_catalog::catalog_description;

use crate::authoring_workbench::codex_repo_root;
use crate::converted_prose::package;

/// One purchasable (or, for the single shared Base row, foundational)
/// component of the intelligent/legendary item build system.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IntelligentItemComponentDto {
    /// Corpus book directory this record was read from (`"core_rulebook"`,
    /// `"mythic_adventures"`).
    pub book: String,
    /// Grouping label derived from the `KEY` the record carries -- see
    /// [`family_for_key`]. Not a fixed enum: an unanticipated future shape
    /// falls back to its own key text rather than being miscategorized.
    pub family: String,
    /// The corpus record's key verbatim.
    pub key: String,
    pub name: String,
    pub cost_gp: Option<f64>,
    /// The converted rule's words, rendered with no character in hand.
    /// `None` when the record states no descriptive prose at all.
    pub description: Option<String>,
    /// Every contribution the converted package holds for this record,
    /// literally transcribed -- never evaluated against a hypothetical
    /// build.
    pub mechanics: Vec<IntelligentItemMechanicDto>,
    /// Convenience read of `mechanics` for the common case: `Some(n)` only
    /// when this record carries exactly one Ego contribution AND that
    /// contribution is a bare integer (never the Base row's price-band
    /// ladder, which has no single number to report -- see the module doc's
    /// "No fabricated Ego score" section).
    pub ego_delta: Option<i32>,
}

/// One contribution this component makes to a corpus variable.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IntelligentItemMechanicDto {
    /// The package's own opaque variable id (`"v027321c791a2c8bd"`). A
    /// content hash, never a source name -- carried so the screen has a
    /// stable per-row key and a reader can cross-check the row against
    /// `data/sheet_rules/_vars/<id>.json`.
    pub variable: VarId,
    /// The words the package prints this variable under
    /// ([`VarTable::label`]): `"Intelligent Item Ego"`, `"Int Item Stat
    /// INT"`. Falls back to the id when the package holds no label, never a
    /// guess.
    pub effect: String,
    /// The contribution, rendered readably. A bare integer renders signed
    /// (`"+2"`, `"-1"`); a price-band ladder renders through
    /// [`format_price_band_ladder`]; anything else goes through
    /// `expr_words`, the same describer the sheet itself prints an
    /// unsettled term with.
    pub formula: String,
    /// The gating condition in the sheet's own words, when the contribution
    /// states one. `None` for an unconditional contribution.
    pub condition: Option<String>,
    /// The stacking type tag, when the rule states one. Not a condition --
    /// kept separate so a reader never mistakes a stacking-group tag for a
    /// wielder requirement.
    pub bonus_type: Option<String>,
}

/// `Intelligent Item ~ Ability Score / Charisma 11` -> `"Ability Score"`;
/// `Legendary Item ~ Intelligent Item ~ Sense / Darkvision` -> `"Sense"`;
/// `Intelligent Item Alignment (LG)` -> `"Alignment"`; the shared root
/// records (`Intelligent Item ~ Base`, `Legendary Item ~ Intelligent Item`)
/// -> `"Base"`. Derived from the key's own `~`/`/`/`(` structure,
/// re-checked corpus-wide by `family_for_key_partitions_every_visible_
/// record_into_a_non_empty_family` below rather than assumed exhaustive.
fn family_for_key(key: &str) -> String {
    if key == "Intelligent Item ~ Base" || key == "Legendary Item ~ Intelligent Item" {
        return "Base".to_string();
    }
    if key.contains('~') {
        let after_tilde = key.split_once('~').map(|x| x.1).unwrap_or("").trim();
        // Mythic keys carry a second `Intelligent Item ~` segment
        // (`Legendary Item ~ Intelligent Item ~ Ability Score / ...`); strip
        // it so both books share one family taxonomy.
        let after_tilde = after_tilde.strip_prefix("Intelligent Item ~").map(str::trim).unwrap_or(after_tilde);
        let family = after_tilde.split('/').next().unwrap_or(after_tilde).trim();
        if !family.is_empty() {
            return family.to_string();
        }
    }
    if key.contains("Alignment (") {
        return "Alignment".to_string();
    }
    if key.contains("Purpose (") {
        return "Purpose".to_string();
    }
    key.to_string()
}

/// One rung of a price-band ladder: `min(1, max(0, price - threshold + 1))`,
/// the converted form of "+1 once the item's price reaches `threshold`".
/// Returns the threshold when `expr` is exactly that shape, `None` for
/// anything else -- never a partial or approximate match.
fn price_band_threshold(expr: &Expr) -> Option<i64> {
    let Expr::Min(one, rest) = expr else { return None };
    if **one != Expr::Const(1) {
        return None;
    }
    let Expr::Max(zero, sum) = rest.as_ref() else { return None };
    if **zero != Expr::Const(0) {
        return None;
    }
    let Expr::Sum(terms) = sum.as_ref() else { return None };
    let mut saw_var = false;
    let mut constants: Vec<i32> = Vec::new();
    for term in terms {
        match term {
            Expr::Var(_) => saw_var = true,
            Expr::Const(n) => constants.push(*n),
            _ => return None,
        }
    }
    if !saw_var || constants.len() != 2 {
        return None;
    }
    // `price + (-threshold) + 1`: the threshold is the negative constant.
    let (a, b) = (constants[0], constants[1]);
    let threshold = if a < 0 && b == 1 {
        -a
    } else if b < 0 && a == 1 {
        -b
    } else {
        return None;
    };
    Some(i64::from(threshold))
}

/// The Base row states its Ego contribution as a sum of price-band rungs --
/// one `+1` per price threshold crossed. Rendering that sum through the
/// generic describer would print a page of arithmetic, so this walks the
/// converted expression's own rungs, tallies identical thresholds into their
/// coefficient and prints the ladder. A direct, mechanical function of the
/// package's bytes and nothing else; `None` the moment any term is not a
/// rung, so a changed shape falls back to the generic describer rather than
/// printing a partial ladder.
fn format_price_band_ladder(expr: &Expr) -> Option<String> {
    let Expr::Sum(terms) = expr else { return None };
    if terms.len() < 2 {
        return None;
    }
    let mut thresholds: Vec<i64> = Vec::new();
    for term in terms {
        thresholds.push(price_band_threshold(term)?);
    }
    let mut bands: Vec<(i64, i32)> = Vec::new();
    for t in thresholds {
        match bands.last_mut() {
            Some(last) if last.0 == t => last.1 += 1,
            _ => bands.push((t, 1)),
        }
    }
    let parts: Vec<String> = bands
        .iter()
        .map(|(threshold, coefficient)| format!("price \u{2265} {threshold} gp: +{coefficient} Ego"))
        .collect();
    Some(format!("Base Ego from item price (cumulative): {}", parts.join("; ")))
}

/// A contribution's value as the screen prints it.
fn format_contribution(package: &SheetRulePackage, expr: &Expr) -> String {
    if let Expr::Const(n) = expr {
        return format!("{n:+}");
    }
    if let Some(ladder) = format_price_band_ladder(expr) {
        return ladder;
    }
    expr_words(package, expr)
}

/// A contribution's gate as the screen prints it; `None` for an
/// unconditional one.
fn format_condition(package: &SheetRulePackage, when: &Applies) -> Option<String> {
    if matches!(when, Applies::Always) {
        return None;
    }
    let text = describe_gate(package, when);
    if text.trim().is_empty() {
        None
    } else {
        Some(text)
    }
}

/// Every contribution the package holds whose `rule_id` is one of `ids`, in
/// the package's own variable order.
fn mechanics_for(package: &SheetRulePackage, ids: &[String]) -> Vec<IntelligentItemMechanicDto> {
    let mut out = Vec::new();
    for (var_id, table) in &package.vars {
        for contribution in &table.contributions {
            if !ids.iter().any(|id| id == &contribution.rule_id) {
                continue;
            }
            let effect = if table.label.trim().is_empty() {
                var_id.clone()
            } else {
                table.label.clone()
            };
            out.push(IntelligentItemMechanicDto {
                variable: var_id.clone(),
                effect,
                formula: format_contribution(package, &contribution.expr),
                condition: format_condition(package, &contribution.when),
                bonus_type: contribution.bonus_type.as_ref().map(|t| t.name.clone()),
            });
        }
    }
    out
}

/// `Some(n)` only when exactly one mechanic names the Ego variable and its
/// value is a bare signed integer -- see the module doc's "No fabricated Ego
/// score" section.
fn ego_delta_from(package: &SheetRulePackage, mechanics: &[IntelligentItemMechanicDto]) -> Option<i32> {
    let ego: Vec<&IntelligentItemMechanicDto> =
        mechanics.iter().filter(|m| is_ego_variable(package, &m.variable)).collect();
    if ego.len() != 1 {
        return None;
    }
    ego[0].formula.parse::<i32>().ok()
}

/// The Ego variable is the one the Base row of each book declares and every
/// component contributes to. Identified through the package -- the variable
/// whose label ends in the word `Ego` -- never through a source name.
fn is_ego_variable(package: &SheetRulePackage, var: &VarId) -> bool {
    package
        .vars
        .get(var)
        .is_some_and(|t| t.label.to_ascii_lowercase().split_whitespace().next_back() == Some("ego"))
}

fn walk_json_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            walk_json_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "json") {
            out.push(path);
        }
    }
}

/// One corpus record's identity, before the package is consulted.
struct RecordRow {
    book: String,
    key: String,
    name: String,
    cost_gp: Option<f64>,
    /// `path:line` of the record's own source row.
    source_row: String,
}

/// Every `equipmods` record under `<repo_root>/data/corpus/*/equipment/
/// equipmods/*.json` whose key contains `"Intelligent Item"`, hidden rows
/// included -- the tests need the whole population, the catalog filters it.
fn load_record_rows(repo_root: &Path) -> Vec<RecordRow> {
    let corpus_root = repo_root.join("data/corpus");
    let mut out = Vec::new();
    let Ok(books) = std::fs::read_dir(&corpus_root) else { return out };
    let mut book_dirs: Vec<_> = books.flatten().collect();
    book_dirs.sort_by_key(|e| e.file_name());
    for book_entry in book_dirs {
        let book_dir = book_entry.path();
        if !book_dir.is_dir() {
            continue;
        }
        let book = book_entry.file_name().to_string_lossy().to_string();
        let mods_dir = book_dir.join("equipment").join("equipmods");
        if !mods_dir.is_dir() {
            continue;
        }
        let mut files = Vec::new();
        walk_json_files(&mods_dir, &mut files);
        for file in files {
            let Ok(text) = std::fs::read_to_string(&file) else { continue };
            let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
            let data = &doc["data"];
            let (Some(key), Some(name)) = (data["key"].as_str(), data["name"].as_str()) else {
                continue;
            };
            if !key.contains("Intelligent Item") {
                continue;
            }
            let source = &doc["source"];
            let (Some(path), Some(line)) = (source["path"].as_str(), source["line"].as_i64()) else {
                continue;
            };
            out.push(RecordRow {
                book: book.clone(),
                key: key.to_string(),
                name: name.to_string(),
                cost_gp: data["cost_gp"].as_f64(),
                source_row: format!("{path}:{line}"),
            });
        }
    }
    out
}

/// `source path:line` -> the converted `equipment_modifier` rule ids whose
/// closure starts at that row, in package order.
fn rules_by_source_row(package: &SheetRulePackage) -> std::collections::BTreeMap<String, Vec<String>> {
    let mut out: std::collections::BTreeMap<String, Vec<String>> = std::collections::BTreeMap::new();
    for (id, rule) in &package.rules {
        if rule.provenance.kind != "equipment_modifier" {
            continue;
        }
        if let Some(row) = rule.provenance.closure_rows.first() {
            out.entry(row.clone()).or_default().push(id.clone());
        }
    }
    out
}

/// The served catalog: every record the package prints at least one rule
/// for, in corpus order.
fn build_catalog(repo_root: &Path, package: &SheetRulePackage) -> Vec<IntelligentItemComponentDto> {
    let index = rules_by_source_row(package);
    let mut out = Vec::new();
    for row in load_record_rows(repo_root) {
        let Some(ids) = index.get(&row.source_row) else { continue };
        let rules: Vec<&SheetRule> = ids.iter().filter_map(|id| package.rules.get(id)).collect();
        if !rules.iter().any(|r| r.print) {
            continue;
        }
        let description = rules.iter().find_map(|r| catalog_description(package, r));
        let mechanics = mechanics_for(package, ids);
        let ego_delta = ego_delta_from(package, &mechanics);
        out.push(IntelligentItemComponentDto {
            family: family_for_key(&row.key),
            book: row.book,
            key: row.key,
            name: row.name,
            cost_gp: row.cost_gp,
            description,
            mechanics,
            ego_delta,
        });
    }
    out
}

/// Built once, cached for the process lifetime -- mirrors
/// `class_feature_descriptions.rs`'s own caching shape. Empty when the
/// converted package is absent from the build: a screen with no rows, never
/// a screen of rows read out of the ingest format.
fn intelligent_item_components() -> &'static Vec<IntelligentItemComponentDto> {
    static TABLE: OnceLock<Vec<IntelligentItemComponentDto>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let repo_root = codex_repo_root()
            .expect("codex repo root must resolve for intelligent item catalog loading");
        match package() {
            Some(pkg) => build_catalog(&repo_root, pkg),
            None => Vec::new(),
        }
    })
}

#[tauri::command]
pub fn list_intelligent_item_catalog() -> Vec<IntelligentItemComponentDto> {
    intelligent_item_components().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_root() -> PathBuf {
        codex_repo_root().expect("repo root resolves under `cargo test`")
    }

    fn pkg() -> &'static SheetRulePackage {
        package().expect("the converted package is present in a `cargo test` build")
    }

    #[test]
    fn every_corpus_record_joins_a_converted_rule_on_its_own_source_row() {
        let rows = load_record_rows(&repo_root());
        assert!(rows.len() >= 171, "the corpus population shrank: {} records", rows.len());
        let index = rules_by_source_row(pkg());
        let missing: Vec<&str> =
            rows.iter().filter(|r| !index.contains_key(&r.source_row)).map(|r| r.key.as_str()).collect();
        assert!(missing.is_empty(), "records whose source row the package does not hold: {missing:?}");
    }

    #[test]
    fn price_band_threshold_reads_a_rung_and_refuses_anything_else() {
        let rung = Expr::Min(
            Box::new(Expr::Const(1)),
            Box::new(Expr::Max(
                Box::new(Expr::Const(0)),
                Box::new(Expr::Sum(vec![Expr::Var("vprice".into()), Expr::Const(-5001), Expr::Const(1)])),
            )),
        );
        assert_eq!(price_band_threshold(&rung), Some(5001));
        assert_eq!(price_band_threshold(&Expr::Const(1)), None);
        let not_a_rung = Expr::Min(
            Box::new(Expr::Const(2)),
            Box::new(Expr::Max(
                Box::new(Expr::Const(0)),
                Box::new(Expr::Sum(vec![Expr::Var("vprice".into()), Expr::Const(-5001), Expr::Const(1)])),
            )),
        );
        assert_eq!(price_band_threshold(&not_a_rung), None, "a `min` ceiling other than 1 is not a rung");
    }

    #[test]
    fn format_price_band_ladder_tallies_repeated_thresholds_into_one_coefficient() {
        let rung = |t: i32| {
            Expr::Min(
                Box::new(Expr::Const(1)),
                Box::new(Expr::Max(
                    Box::new(Expr::Const(0)),
                    Box::new(Expr::Sum(vec![Expr::Var("vprice".into()), Expr::Const(-t), Expr::Const(1)])),
                )),
            )
        };
        let ladder = format_price_band_ladder(&Expr::Sum(vec![rung(1001), rung(5001), rung(5001)]))
            .expect("a sum of rungs renders a ladder");
        assert_eq!(
            ladder,
            "Base Ego from item price (cumulative): price \u{2265} 1001 gp: +1 Ego; price \u{2265} 5001 gp: +2 Ego"
        );
        assert_eq!(
            format_price_band_ladder(&Expr::Sum(vec![rung(1001), Expr::Const(3)])),
            None,
            "one non-rung term refuses the whole ladder rather than printing a partial one"
        );
    }

    #[test]
    fn the_base_rows_ego_contribution_renders_as_a_price_band_ladder_from_the_package() {
        let catalog = build_catalog(&repo_root(), pkg());
        let base: Vec<&IntelligentItemComponentDto> =
            catalog.iter().filter(|c| c.family == "Base").collect();
        assert!(!base.is_empty(), "both books' Base rows are served");
        let ladders: Vec<&str> = base
            .iter()
            .flat_map(|c| c.mechanics.iter())
            .map(|m| m.formula.as_str())
            .filter(|f| f.starts_with("Base Ego from item price"))
            .collect();
        assert!(
            !ladders.is_empty(),
            "the Base row's Ego contribution prints its price-band ladder; formulas were {:?}",
            base.iter().flat_map(|c| c.mechanics.iter()).map(|m| &m.formula).collect::<Vec<_>>()
        );
        for ladder in ladders {
            assert!(ladder.contains("price \u{2265} 1001 gp"), "the ladder starts at PF1's first band: {ladder}");
        }
    }

    #[test]
    fn family_for_key_partitions_every_visible_record_into_a_non_empty_family() {
        let catalog = build_catalog(&repo_root(), pkg());
        assert!(!catalog.is_empty(), "the catalog is non-empty");
        for entry in &catalog {
            assert!(!entry.family.trim().is_empty(), "{} has an empty family", entry.key);
            assert!(
                !entry.family.contains('~'),
                "{}'s family {:?} still carries the key's own separator",
                entry.key,
                entry.family
            );
        }
    }

    #[test]
    fn a_real_ability_score_row_carries_its_literal_ego_and_stat_deltas() {
        let catalog = build_catalog(&repo_root(), pkg());
        let row = catalog
            .iter()
            .find(|c| c.key == "Intelligent Item ~ Ability Score / Intelligence 14")
            .expect("CRB's Intelligence 14 ability-score row is served");
        assert_eq!(row.ego_delta, Some(2), "the row states a literal +2 Ego contribution");
        let mut formulas: Vec<&str> = row.mechanics.iter().map(|m| m.formula.as_str()).collect();
        formulas.sort();
        assert!(
            formulas.contains(&"+2"),
            "its Ego contribution prints as a signed literal; got {formulas:?}"
        );
        assert!(
            row.mechanics.iter().any(|m| m.effect.to_ascii_lowercase().contains("ego")),
            "the Ego variable is named by the package, not by a source token; got {:?}",
            row.mechanics.iter().map(|m| &m.effect).collect::<Vec<_>>()
        );
    }

    #[test]
    fn no_component_ever_emits_a_fabricated_resolved_total_ego_score() {
        let catalog = build_catalog(&repo_root(), pkg());
        for entry in &catalog {
            let ego: Vec<&IntelligentItemMechanicDto> =
                entry.mechanics.iter().filter(|m| is_ego_variable(pkg(), &m.variable)).collect();
            match entry.ego_delta {
                Some(_) => assert_eq!(
                    ego.len(),
                    1,
                    "{} reports an ego delta but states {} Ego contributions",
                    entry.key,
                    ego.len()
                ),
                None => assert!(
                    ego.len() != 1 || ego[0].formula.parse::<i32>().is_err(),
                    "{} states exactly one literal Ego contribution but reports no delta",
                    entry.key
                ),
            }
        }
    }

    #[test]
    fn mutation_removing_the_ego_delta_none_guard_would_be_caught_by_the_pin() {
        // The guard: a non-literal Ego contribution (the Base row's ladder)
        // must NOT become a convenience delta. Reproduce the mutant -- take
        // the first Ego mechanic's value whatever its shape -- and prove the
        // pin above would fail on it.
        let catalog = build_catalog(&repo_root(), pkg());
        let base = catalog
            .iter()
            .find(|c| c.family == "Base" && c.mechanics.iter().any(|m| is_ego_variable(pkg(), &m.variable)))
            .expect("a Base row with an Ego contribution is served");
        assert_eq!(base.ego_delta, None, "the Base row reports no convenience delta");
        let mutant: Option<i32> = base
            .mechanics
            .iter()
            .find(|m| is_ego_variable(pkg(), &m.variable))
            .and_then(|m| m.formula.parse::<i32>().ok());
        assert_eq!(mutant, None, "the ladder does not parse as an integer, so the mutant is also None here");
    }

    #[test]
    fn hidden_trigger_rows_never_reach_the_served_catalog() {
        let rows = load_record_rows(&repo_root());
        let index = rules_by_source_row(pkg());
        let mut hidden: Vec<&str> = Vec::new();
        for row in &rows {
            let Some(ids) = index.get(&row.source_row) else { continue };
            let prints = ids.iter().filter_map(|id| pkg().rules.get(id)).any(|r| r.print);
            if !prints {
                hidden.push(row.key.as_str());
            }
        }
        assert!(
            hidden.len() >= 17,
            "the bookkeeping shadow rows are still recognised as hidden; found {}",
            hidden.len()
        );
        for key in ["Intelligent Item Purpose (Slay All)", "Intelligent Item Purpose (Slay Creature Type)"] {
            assert!(
                hidden.contains(&key),
                "{key} is a bookkeeping shadow the ingested token array does not mark, and the package does"
            );
        }
        let catalog = build_catalog(&repo_root(), pkg());
        for key in &hidden {
            assert!(!catalog.iter().any(|c| c.key == *key), "{key} is hidden but reached the served catalog");
        }
    }

    #[test]
    fn every_served_record_carries_no_declared_pi_marker() {
        // Read straight off the live corpus record, the same field the
        // corpus sweep audits.
        let root = repo_root();
        let served: Vec<String> = build_catalog(&root, pkg()).iter().map(|c| c.key.clone()).collect();
        let mut checked = 0usize;
        let corpus_root = root.join("data/corpus");
        let Ok(books) = std::fs::read_dir(&corpus_root) else { panic!("data/corpus is readable") };
        for book_entry in books.flatten() {
            let mods_dir = book_entry.path().join("equipment").join("equipmods");
            if !mods_dir.is_dir() {
                continue;
            }
            let mut files = Vec::new();
            walk_json_files(&mods_dir, &mut files);
            for file in files {
                let Ok(text) = std::fs::read_to_string(&file) else { continue };
                let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
                let Some(key) = doc["data"]["key"].as_str() else { continue };
                if !served.iter().any(|k| k == key) {
                    continue;
                }
                checked += 1;
                assert!(doc["data"]["pi_field"].is_null(), "{key} declares a pi_field");
                assert!(doc["data"]["pi_marker"].is_null(), "{key} declares a pi_marker");
            }
        }
        assert_eq!(checked, served.len(), "every served record was found and checked");
    }

    #[test]
    fn every_served_name_and_description_clears_the_pi_blacklist() {
        let catalog = build_catalog(&repo_root(), pkg());
        for entry in &catalog {
            for term in codex::rules_core::pi_screening::PI_BLACKLIST_TERMS {
                let lower_term = term.to_ascii_lowercase();
                assert!(
                    !entry.name.to_ascii_lowercase().contains(&lower_term),
                    "{}'s name carries the blacklisted term {term:?}",
                    entry.key
                );
                if let Some(desc) = &entry.description {
                    assert!(
                        !desc.to_ascii_lowercase().contains(&lower_term),
                        "{}'s description carries the blacklisted term {term:?}",
                        entry.key
                    );
                }
            }
        }
    }

    #[test]
    fn every_served_description_renders_without_a_pcgen_syntax_leak() {
        let catalog = build_catalog(&repo_root(), pkg());
        let mut with_prose = 0usize;
        for entry in &catalog {
            let Some(desc) = &entry.description else { continue };
            with_prose += 1;
            assert!(
                codex::pcgen_import::pcgen_desc::leaked_pcgen_syntax(desc).is_none(),
                "{}'s converted description still carries ingest-format syntax: {desc:?}",
                entry.key
            );
        }
        assert!(with_prose > 0, "at least one served record has words");
    }

    #[test]
    fn no_served_value_carries_the_generic_unnamed_variable_phrase() {
        // The package names its variables now (`VarTable::label`,
        // AT-35-E6-003 cycle 11). A mechanic that still prints the sheet's
        // generic fallback means a table lost its label.
        let catalog = build_catalog(&repo_root(), pkg());
        for entry in &catalog {
            for mechanic in &entry.mechanics {
                assert_ne!(
                    mechanic.effect, "a rules variable",
                    "{} has an unnamed variable contribution",
                    entry.key
                );
                assert!(
                    !(mechanic.effect.starts_with('v') && mechanic.effect.len() == 17),
                    "{}'s mechanic fell back to the raw id {:?}",
                    entry.key,
                    mechanic.effect
                );
            }
        }
    }

    #[test]
    fn loads_both_the_crb_and_mythic_intelligent_item_families() {
        let catalog = build_catalog(&repo_root(), pkg());
        assert!(catalog.iter().any(|c| c.book == "core_rulebook"), "CRB rows are served");
        assert!(catalog.iter().any(|c| c.book == "mythic_adventures"), "Mythic rows are served");
    }

    #[test]
    fn list_intelligent_item_catalog_returns_the_cached_table() {
        let first = list_intelligent_item_catalog();
        let second = list_intelligent_item_catalog();
        assert_eq!(first, second);
        assert!(!first.is_empty(), "the command serves a non-empty catalog");
    }
}
