//! SD-35 AT-35-E2-001 -- the converter exists and writes our schema.
//!
//! Three value-form tests on REAL corpus records (the racial spell-like ability's save DC as
//! `Sum([Const(10), Const(1), AbilityMod(Cha)])`, a weapon as `Dice{"1d8", None}`, a choice
//! trait as `Text` with the choice bound), one converter gate PER KIND reading the live
//! `data/sheet_rules/` package (every unit of that kind converts or is in `_refused.json`), the
//! source-format literal scan, and the freshness check the `--check` flag runs. No per-unit
//! fixture carries a hand-derived value (`decisions.md` §4): the expected shapes are the
//! mapping table's own rows.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::OnceLock;

use codex::pcgen_import::sheet_rule::closure::{corpus_root, Closure, PinnedTree};
use codex::pcgen_import::sheet_rule::convert::{convert_record, Converted};
use codex::pcgen_import::sheet_rule::ctx::CorpusIndex;
use codex::pcgen_import::sheet_rule::{build_index, check, load_population, read_output, run, shape_violations};
use codex::rules_core::sheet_rule::*;

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

struct Shared {
    tree: PinnedTree,
    index: CorpusIndex,
    closures: Vec<Closure>,
}

fn shared() -> &'static Shared {
    static S: OnceLock<Shared> = OnceLock::new();
    S.get_or_init(|| {
        let tree = PinnedTree::load(&corpus_root()).expect("pinned corpus checkout present (scripts/fetch-pcgen-oracle.sh)");
        let records = load_population(&repo()).expect("docs/work-inventory.json and data/corpus readable");
        let (index, closures) = build_index(&tree, records);
        Shared { tree, index, closures }
    })
}

fn convert_unit(id: &str) -> Converted {
    let s = shared();
    let pos = s.index.records.iter().position(|r| r.id == id).unwrap_or_else(|| panic!("unit {id} is in docs/work-inventory.json"));
    convert_record(&s.tree, &s.index, &s.index.records[pos], &s.closures[pos])
}

/// The racial SLA DC: table row `FORMULA:corpus variable, SAME-record contributors only`
/// ("DC = Sum([Const(10), Const(1), AbilityMod(Cha)])") on the row's own example record
/// `advanced_players_guide:race_trait_generic:Racial SLA ~ Ill Omen` (`apg_abilities_race.lst:324`,
/// a 1st-level spell), and the same shape with the spell's own level on `Racial SLA ~ Ironskin`
/// (`mc_abilities_race.lst:95`, a 2nd-level spell -- the level is the record's own setter, not
/// a hand-derived value).
#[test]
fn racial_sla_dc_converts_to_ten_plus_spell_level_plus_charisma() {
    for (unit, spell_level, base_row) in [
        ("advanced_players_guide:race_trait:racial_sla_ill_omen", 1, "apg_abilities_race.lst:324"),
        ("monster_codex:race_trait:racial_sla_ironskin", 2, "mc_abilities_race.lst:95"),
    ] {
        let c = convert_unit(unit);
        assert!(c.refusals.is_empty(), "{unit} refusals: {:?}", c.refusals);
        let dc = c
            .rules
            .iter()
            .flat_map(|r| r.also.iter())
            .find_map(|(role, v)| if *role == ValueRole::SaveDc { Some(v.clone()) } else { None })
            .expect("the SPELLS line carries a SaveDc");
        assert_eq!(dc, SheetValue::Number(Expr::Sum(vec![Expr::Const(10), Expr::Const(spell_level), Expr::AbilityMod(Ability::Cha)])), "{unit}");
        let cl = c.rules.iter().flat_map(|r| r.also.iter()).find_map(|(role, v)| if *role == ValueRole::CasterLevel { Some(v.clone()) } else { None }).unwrap();
        assert_eq!(cl, SheetValue::Number(Expr::Level), "{unit}: CASTERLEVEL=<LVL var> folds to Level (row FORMULA:TL)");
        let uses = c.rules.iter().flat_map(|r| r.also.iter()).find_map(|(role, v)| if matches!(role, ValueRole::Uses { .. }) { Some(v.clone()) } else { None }).unwrap();
        assert_eq!(uses, SheetValue::Number(Expr::Const(1)), "{unit}");
        for r in &c.rules {
            assert!(r.provenance.closure_rows.iter().any(|c| c.ends_with(base_row)), "{unit}: provenance cites the base row: {:?}", r.provenance.closure_rows);
        }
    }
}

/// A weapon: table row `DAMAGE / ALTDAMAGE` -> `Dice{dice:'1d8', modifier:None}` on the
/// core longsword (`cr_equip_arms_armor.lst:223`, a `.COPY=` row resolved to its base). The
/// `+2` of a "1d8+2" line is a held damage bonus the evaluator folds into `Dice.modifier`; no
/// corpus record carries a `NdM+K` damage literal, so the modifier form is proven on the die
/// literal reader alone (see `dice_literal_reads_a_modifier`).
#[test]
fn weapon_converts_to_dice() {
    let c = convert_unit("core_rulebook:equipment:longsword");
    assert!(c.refusals.is_empty(), "refusals: {:?}", c.refusals);
    let principal = &c.rules[0];
    assert_eq!(principal.value, SheetValue::Dice { dice: "1d8".into(), modifier: None, size_steps: None });
    assert!(principal.prose.iter().any(|s| matches!(&s.family, ProseFamily::StatBlock(l) if l == "Critical threat")), "the crit line is a stat-block segment");
}

/// A choice trait: synthesis row `%CHOICE / %LIST (all positions)` -- `Text` with the choice
/// bound: `offers` carries the CHOOSE and the prose slot is `ChoiceName(choice)`
/// (`advanced_players_guide:trait_generic:Trait ~ Magical Knack`: `CHOOSE:NUMCHOICES=1|CLASS|SPELLCASTER`,
/// a DESC with `%CHOICE`, no bonus token).
#[test]
fn choice_trait_converts_to_text_with_the_choice_bound() {
    let c = convert_unit("advanced_players_guide:trait:trait_magical_knack");
    assert!(c.refusals.is_empty(), "refusals: {:?}", c.refusals);
    let r = &c.rules[0];
    assert_eq!(r.value, SheetValue::Text);
    let offers = r.offers.as_ref().expect("CHOOSE:NUMCHOICES=1|CLASS|... makes the record choice-bearing");
    assert_eq!(offers.id, r.id);
    assert!(matches!(offers.from, OptionSet::Classes(_)), "a class to pick: {:?}", offers.from);
    assert!(r.prose.iter().any(|seg| seg.pieces.iter().any(|p| *p == ProsePiece::ChoiceName(r.id.clone()))), "the DESC %CHOICE slot is the chosen name: {:?}", r.prose);
    assert!(!serde_json::to_string(r).unwrap().contains('%'), "no marker survives into the rule");
}

#[test]
fn dice_literal_reads_a_modifier() {
    // The `Dice.modifier` form the design names ("1d8" + Const(2) -> "1d8+2"), on the reader
    // every DAMAGE / NATURALATTACKS die goes through.
    use codex::pcgen_import::sheet_rule::convert::dice_literal;
    assert_eq!(dice_literal("1d8+2"), Some(("1d8".to_string(), Some(Expr::Const(2)))));
    assert_eq!(dice_literal("1d8"), Some(("1d8".to_string(), None)));
    assert_eq!(dice_literal("2d6-1"), Some(("2d6".to_string(), Some(Expr::Const(-1)))));
    assert_eq!(dice_literal("Special"), None);
    assert_eq!(dice_literal("0"), None);
}

fn package_files() -> BTreeMap<String, Vec<u8>> {
    let out = read_output(&repo().join("data/sheet_rules"));
    assert!(!out.is_empty(), "data/sheet_rules/ is generated (cargo run --locked --bin sheet_rule_convert)");
    out
}

fn inventory_ids_by_kind() -> BTreeMap<String, BTreeSet<String>> {
    let text = std::fs::read_to_string(repo().join("docs/work-inventory.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&text).unwrap();
    let mut out: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for u in json["units"].as_array().unwrap() {
        out.entry(u["kind"].as_str().unwrap().to_string()).or_default().insert(u["id"].as_str().unwrap().to_string());
    }
    out
}

fn refused_ids() -> BTreeSet<String> {
    let text = std::fs::read_to_string(repo().join("data/sheet_rules/_refused.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&text).unwrap();
    json["entries"].as_array().unwrap().iter().map(|e| e["id"].as_str().unwrap().to_string()).collect()
}

/// One gate per kind: every inventory unit of the kind has a rule file whose principal rule id
/// is the unit id, or an entry in `_refused.json` -- never neither, never both.
fn kind_gate(kind: &str) {
    let inventory = inventory_ids_by_kind();
    let units = inventory.get(kind).cloned().unwrap_or_default();
    assert!(!units.is_empty(), "kind {kind} exists in docs/work-inventory.json");
    let files = package_files();
    let mut converted: BTreeSet<String> = BTreeSet::new();
    for (rel, bytes) in &files {
        let parts: Vec<&str> = rel.split('/').collect();
        if parts.len() != 3 || parts[1] != kind {
            continue;
        }
        let rules: Vec<SheetRule> = serde_json::from_slice(bytes).unwrap_or_else(|e| panic!("{rel}: {e}"));
        assert!(!rules.is_empty(), "{rel}: at least one rule");
        converted.insert(rules[0].id.clone());
        for r in &rules {
            assert_eq!(r.provenance.kind, kind, "{rel}: provenance kind");
        }
    }
    let refused: BTreeSet<String> = refused_ids().into_iter().filter(|id| id.split(':').nth(1) == Some(kind)).collect();
    let both: Vec<_> = converted.intersection(&refused).take(5).collect();
    assert!(both.is_empty(), "{kind}: converted AND refused: {both:?}");
    let covered: BTreeSet<String> = converted.union(&refused).cloned().collect();
    let missing: Vec<_> = units.difference(&covered).take(5).collect();
    let extra: Vec<_> = covered.difference(&units).take(5).collect();
    assert!(missing.is_empty() && extra.is_empty(), "{kind}: units={} converted={} refused={} missing={missing:?} extra={extra:?}", units.len(), converted.len(), refused.len());
}

macro_rules! kind_gates {
    ($($name:ident => $kind:literal),* $(,)?) => {
        $(#[test] fn $name() { kind_gate($kind); })*
    };
}

kind_gates! {
    gate_ability => "ability",
    gate_class => "class",
    gate_class_feature => "class_feature",
    gate_companion => "companion",
    gate_deity => "deity",
    gate_domain => "domain",
    gate_equipment => "equipment",
    gate_equipment_modifier => "equipment_modifier",
    gate_feat => "feat",
    gate_language => "language",
    gate_monster => "monster",
    gate_monster_ability => "monster_ability",
    gate_power => "power",
    gate_race => "race",
    gate_race_trait => "race_trait",
    gate_skill => "skill",
    gate_spell => "spell",
    gate_template => "template",
    gate_trait => "trait",
}

/// `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` is 0, plus
/// the `TYPE=` / `%<digit>` / `DESC:` / `SAB:` literals the residue gate names.
#[test]
fn package_carries_no_source_format_literal() {
    let files = package_files();
    let hits = shape_violations(&files);
    assert!(hits.is_empty(), "{} files carry a source-format literal, e.g. {:?}", hits.len(), hits.iter().take(5).collect::<Vec<_>>());
}

/// The `--check` gate: the package on disk equals a fresh conversion byte for byte, every
/// referenced variable has a table, and converted + refused sums to the population.
#[test]
fn package_on_disk_is_fresh_and_clean() {
    let s = shared();
    let r = run(&s.tree, &s.index, &s.closures);
    assert_eq!(r.report.records, 49_438, "records = every docs/work-inventory.json unit");
    assert_eq!(r.report.converted + r.report.refused, r.report.records);
    if let Err(problems) = check(&repo().join("data/sheet_rules"), &r) {
        panic!("{} problem(s), e.g. {:?}", problems.len(), problems.iter().take(8).collect::<Vec<_>>());
    }
}

/// Two consecutive conversions of the unchanged tree are identical (the `--check` premise).
#[test]
fn conversion_is_deterministic() {
    let s = shared();
    let a = codex::pcgen_import::sheet_rule::render(&run(&s.tree, &s.index, &s.closures));
    let b = codex::pcgen_import::sheet_rule::render(&run(&s.tree, &s.index, &s.closures));
    assert_eq!(a.len(), b.len());
    for (k, v) in &a {
        assert_eq!(Some(v), b.get(k), "{k} differs between two runs");
    }
}

/// SD-35 AT-35-E2-004 -- the token census `scripts/token_coverage.py` reads. Every token the
/// converter processed names the mapping-table row it resolved to (or `unmapped:<HEAD>` /
/// `BONUS:<SUB>` when the table has none), and every refusal names the token type it arose
/// under, so "units carrying it" and "units refused because of this token" are counted from
/// the closure the converter actually read -- never from a second reading of the corpus.
#[test]
fn token_census_names_the_row_for_every_token_and_the_head_under_each_refusal() {
    // A converted record carries the rows its tokens resolve to and no refusal.
    let c = convert_unit("core_rulebook:equipment:longsword");
    assert!(c.tokens.contains("DAMAGE / ALTDAMAGE"), "the longsword's DAMAGE token names its row: {:?}", c.tokens);
    assert!(c.refusals.is_empty() && c.refusal_under.is_empty());
    // A refused record: every refusal shape is recorded under the token type it arose under,
    // and that token type is itself in the census.
    let c = convert_unit("advanced_class_guide:class:arcanist");
    for shape in ["unmapped:STARTSKILLPTS", "unmapped:MEMORIZE"] {
        assert!(c.refusals.contains(shape), "{shape} refuses the Arcanist: {:?}", c.refusals);
        let under: Vec<&String> = c.refusal_under.get(shape).map(|s| s.iter().collect()).unwrap_or_default();
        assert_eq!(under, vec![shape], "{shape} arose under itself");
        assert!(c.tokens.contains(shape), "an unmapped head is still a token the record carries");
    }
    // A formula-shaped refusal names the TOKEN it arose under, not the formula family.
    let c = convert_unit("advanced_class_guide:class_feature:eldritch_scion_spells");
    let under = c.refusal_under.get("BONUS:STAT (target BASESPELLSTAT;Class)").expect("the refusal is recorded");
    assert_eq!(under.iter().collect::<Vec<_>>(), vec!["BONUS:STAT"], "refused under the BONUS:STAT row");
    // The whole run's census: one entry per record, ids unique, and the refused id set equals
    // `_refused.json`'s -- the sum `token_coverage.py --check` re-checks.
    let s = shared();
    let r = run(&s.tree, &s.index, &s.closures);
    assert_eq!(r.tokens.entries.len(), r.report.records, "one census entry per record");
    let ids: BTreeSet<&String> = r.tokens.entries.iter().map(|e| &e.id).collect();
    assert_eq!(ids.len(), r.report.records, "no record appears twice in the census");
    let census_refused: BTreeSet<&String> = r.tokens.entries.iter().filter(|e| !e.refusals.is_empty()).map(|e| &e.id).collect();
    let refused: BTreeSet<&String> = r.refused.entries.iter().map(|e| &e.id).collect();
    assert_eq!(census_refused, refused, "the census refuses exactly the records _refused.json refuses");
    let rendered = codex::pcgen_import::sheet_rule::render(&r);
    assert!(rendered.contains_key("_tokens.json"), "the census is written into the package");
}
