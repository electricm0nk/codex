//! SD-36 Epic F1-1 -- resolver repair (`docs/release/SD-36-consolidation/epic-f-class-completion.md`
//! §0.2/§0.3/§3.1 items 1-2). These are the live-corpus, whole-tree-in-memory tests the fast
//! synthetic-tree unit tests in `ctx.rs`/`closure.rs` can't stand in for: they prove the fix
//! against the REAL pinned oracle and the REAL `docs/work-inventory.json` population, never
//! writing `data/sheet_rules/` to disk (`run()` builds everything in memory, exactly like
//! `sheet_rule_convert_gate.rs`'s own `package_on_disk_is_fresh_and_clean` already does).

use std::sync::OnceLock;

use codex_ingest::pcgen_import::sheet_rule::closure::{corpus_root, Closure, PinnedTree};
use codex_ingest::pcgen_import::sheet_rule::ctx::{resolve_rule_in, resolve_rule_in_checked, CorpusIndex, RuleLookup};
use codex_ingest::pcgen_import::sheet_rule::{build_index, load_population, run, Run};
use codex::rules_core::sheet_rule::Granter;

fn repo() -> std::path::PathBuf {
    codex_ingest::repo_root()
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
        let records = load_population(&repo(), &tree).expect("docs/work-inventory.json and data/corpus readable");
        let (index, closures) = build_index(&tree, records);
        Shared { tree, index, closures }
    })
}

fn shared_run() -> &'static Run {
    static R: OnceLock<Run> = OnceLock::new();
    R.get_or_init(|| {
        let s = shared();
        run(&s.tree, &s.index, &s.closures)
    })
}

/// §3.1 item 1, mechanism A -- Wizard's OWN defect from §0.2: the reference
/// `Wizard Class Feature|Wizard ~ Weapon and Armor Proficiency` names a CHILD ability category
/// (`Wizard Class Feature`, parent `Special Ability` per the oracle's own
/// `ABILITYCATEGORY:Wizard Class Feature ... CATEGORY:Special Ability` row), and the target rule
/// is indexed under the PARENT. The per-edge pin (review finding 12a /F1.3): a count match alone
/// is not enough (slug `wizard` collides across records), so this asserts the target rule's own
/// `provenance.closure_rows` cites the oracle line the reference names.
#[test]
fn a_child_category_reference_resolves_through_its_parent_on_the_real_corpus() {
    let r = shared_run();
    let target_id = "core_rulebook:class_feature:wizard_weapon_and_armor_proficiency";
    let rules = r
        .files
        .values()
        .flatten()
        .find(|rule| rule.id == target_id)
        .unwrap_or_else(|| panic!("{target_id} must be a converted rule"));
    assert!(
        !rules.granted_by.is_empty(),
        "{target_id} must carry a granted_by edge now that the parent-category retry resolves \
         'Wizard Class Feature' to its parent 'Special Ability': {:?}",
        rules.granted_by
    );
    // Per-edge correctness: the grant's SOURCE must be the Wizard class record's own class
    // ability (never a same-slug collision -- 0.3's own named hazard).
    let by_class = rules.granted_by.iter().any(|g| matches!(&g.by, Granter::Rule(id) if id == "core_rulebook:class_feature:wizard_class__d9affb050e701218"));
    assert!(by_class, "the grant must come from Wizard's own CATEGORY:Class ability record, not a colliding same-slug record: {:?}", rules.granted_by);
    // The TARGET rule's own provenance must cite ITS OWN base row in the oracle
    // (`cr_abilities_class.lst:2566`, the "Wizard ~ Weapon and Armor Proficiency" record's own
    // declaration -- distinct from line 108, which is the SOURCE Wizard ability that grants it)
    // -- proves the parent-category retry landed on the exact record the oracle tool would
    // itself resolve to, never a same-slug collision (0.3's own named hazard: a count match
    // alone is satisfied equally by an edge written to a colliding record).
    assert!(
        rules.provenance.closure_rows.iter().any(|c| c.ends_with("cr_abilities_class.lst:2566")),
        "the target rule's own provenance must cite its own base oracle row (2566), not merely exist: {:?}",
        rules.provenance.closure_rows
    );
}

/// §3.1 item 2 / §0.3 -- the SILENT class -> class-ability miss, diagnosed on Wizard: before the
/// closure-indexer fix, `cr_classes.lst:301` (Wizard's own level-1 self-grant,
/// `ABILITY:Class|AUTOMATIC|Wizard`) never entered the Wizard class record's closure at all, so
/// `resolve_holdable_rule` was never even called for it -- no grant, no defect, nothing. This
/// proves the row now enters the closure (RED before the closure.rs fix: the assertion on
/// `closure_rows` below fails because line 301 is absent).
#[test]
fn wizards_own_level_one_self_grant_now_enters_its_closure() {
    let s = shared();
    let pos = s.index.records.iter().position(|r| r.id == "core_rulebook:class:wizard").expect("core_rulebook:class:wizard is in the population");
    let closure = &s.closures[pos];
    let has_line_301 = closure.rows.iter().any(|row| row.cite.ends_with("cr_classes.lst:301"));
    assert!(has_line_301, "Wizard's own level-1 self-grant (cr_classes.lst:301) must be in the class record's own closure: {:?}", closure.rows.iter().map(|r| &r.cite).collect::<Vec<_>>());
}

/// §3.1 item 2's own acceptance shape ("every `ABILITY:*|AUTOMATIC|` grant on a converted record
/// yields either a `granted_by` row on some rule or a `_defects` row -- count in = count out"),
/// pinned on Wizard specifically (the diagnosed n=1): every `ABILITY:<category>|AUTOMATIC|<target>`
/// token this class record's own closure carries resolves to EITHER a `granted_by` edge on the
/// named target somewhere in the run, OR an `unresolved-references` defect line naming this
/// record and that exact `category|target` pair. None may be silently absent from both.
#[test]
fn automatic_grants_are_never_dropped_silently() {
    let s = shared();
    let r = shared_run();
    let pos = s.index.records.iter().position(|r| r.id == "core_rulebook:class:wizard").expect("core_rulebook:class:wizard is in the population");
    let closure = &s.closures[pos];

    // Re-derive the exact target list from the closure's own tokens, mirroring convert.rs's own
    // ABILITY-arm skip rules (empty / `.CLEAR` / `%LIST` / `%CHOICE` / `TYPE=` selectors never
    // reach `resolve_holdable_rule`, by design -- they are not grants to a single named rule).
    let mut targets: Vec<(String, String)> = Vec::new();
    for row in &closure.rows {
        for (k, v) in &row.tokens {
            if k != "ABILITY" {
                continue;
            }
            let fields: Vec<&str> = v.split('|').collect();
            if fields.len() < 3 || !fields[1].eq_ignore_ascii_case("AUTOMATIC") {
                continue;
            }
            let category = fields[0].trim().to_string();
            for raw_target in &fields[2..] {
                let t = raw_target.trim();
                // Trailing PRE gates ride the LAST field; strip them the same way `split_gates`
                // would (a `PRE...:` segment is a gate, not a target name).
                let t = t.split('|').next().unwrap_or(t).trim();
                if t.is_empty() || t == ".CLEAR" || t.contains("%LIST") || t.contains("%CHOICE") || t.starts_with("TYPE=") || t.starts_with("TYPE.") {
                    continue;
                }
                targets.push((category.clone(), t.to_string()));
            }
        }
    }
    assert!(!targets.is_empty(), "Wizard's own closure must carry at least one ABILITY|AUTOMATIC| grant token to test against");

    let defect_lines: Vec<&String> = r.defects.get("unresolved-references").map(|v| v.iter().collect()).unwrap_or_default();
    let mut dropped: Vec<String> = Vec::new();
    for (category, target) in &targets {
        // `resolve_rule_in` is the exact function `resolve_holdable_rule` calls at conversion
        // time (via `RecordCtx::resolve_rule`); a `Some` here means the grant reaches a real
        // target rule id and can never fall through to the defect path for this token.
        let resolves = resolve_rule_in(&s.tree, &s.index, category, target).is_some();
        let defect_line = format!("core_rulebook:class:wizard: {category}|{target}");
        let has_defect = defect_lines.iter().any(|l| l.as_str() == defect_line);
        if !resolves && !has_defect {
            dropped.push(format!("{category}|{target}"));
        }
    }
    assert!(dropped.is_empty(), "{} automatic grant(s) of Wizard's neither resolve nor carry a defect row -- silently dropped: {dropped:?}", dropped.len());
}

/// F1 adversarial finding 4: the parent-category retry's TARGET lookup (`by_cat_key`/
/// `by_cat_name`, built with `.or_insert`) is not de-ambiguated the way the child->parent MAP
/// already is -- a `(parent category, key)` pair claimed by more than one DIFFERENT converted
/// record silently resolved to whichever one loaded first. Proved against the REAL pinned oracle,
/// not a synthetic fixture: `Combat Feat` is a real CHILD ability category whose declared parent
/// is `Feat` (`PinnedTree::ability_category_parent`, built from the tree's own `ABILITYCATEGORY:`
/// rows), and `Alertness` is a real `KEY` more than one distinct converted FEAT record shares
/// under the parent category `Feat` (the SRD feat "Alertness" is declared as its own record in
/// more than one sourcebook). The retry must never guess between them; it must miss, and the
/// checked variant must report `Ambiguous`, not `Missing`, so the caller
/// (`resolve_holdable_rule`, pinned separately in `prereq.rs`'s own unit tests) can name it as a
/// distinct defect kind.
#[test]
fn an_ambiguous_parent_retry_target_never_resolves_to_a_first_loaded_guess() {
    let s = shared();
    assert_eq!(
        s.tree.ability_category_parent.get("COMBAT FEAT").map(|p| p.as_str()),
        Some("FEAT"),
        "sanity: Combat Feat's declared parent in the pinned oracle must still be Feat"
    );
    assert!(
        s.index.ambiguous_cat_key.contains(&("FEAT".to_string(), "ALERTNESS".to_string())),
        "sanity: the pinned corpus really does carry more than one FEAT record keyed ALERTNESS \
         (a reprint across sourcebooks) -- if this ever becomes false the test needs a new n=1, \
         not a weaker assertion"
    );

    assert!(
        resolve_rule_in(&s.tree, &s.index, "Combat Feat", "Alertness").is_none(),
        "an ambiguous parent-retry target must never resolve to whichever candidate loaded first"
    );
    assert_eq!(
        resolve_rule_in_checked(&s.tree, &s.index, "Combat Feat", "Alertness"),
        RuleLookup::Ambiguous,
        "resolve_rule_in_checked must report Ambiguous, not Missing, so the caller can record a \
         distinct, named defect instead of a plain unresolved reference"
    );
}
