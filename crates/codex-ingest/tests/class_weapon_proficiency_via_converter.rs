//! SD-36 Epic F1.2 / F1.5 / F1.8 -- the class weapon-proficiency READER
//! (`codex::rules_core::pilot_compute::class_proficiency_sheet_rules`,
//! `epic-f-class-completion.md` §3.4), pinned against the pinned PCGen oracle.
//!
//! The reader answers from the converted package (`data/sheet_rules/`); these tests hold it to
//! two independent truths:
//!
//! - (a) the 42 hand-typed `CLASS_WEAPON_PROFICIENCIES` rows (tiers / named / groups compared
//!   as sets at class level 1 -- every static row describes the unarchetyped class's starting
//!   proficiencies);
//! - (b) the pinned oracle's own weapon-proficiency rows (`*_profs_weapon.lst`, read here
//!   directly from the pinned tree's raw lines with this file's own tokenizer, never through
//!   the converter's `WeaponMembershipIndex`): every tier, named weapon, weapon group,
//!   conjunctive selector and expanded weapon set the reader returns for a census class must be
//!   re-derivable from a named oracle row (review finding 15).
//!
//! Plus the gate-carriage contract (F1.8, review finding 1) and the unrecognized-tag contract
//! (review finding 15), and the Wizard "no Simple tier" pin.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use codex::rules_core::pilot_compute::class_proficiency_sheet_rules::{
    class_weapon_proficiency_view, class_weapon_proficiency_view_in, ClassWeaponProficiencyView, ProficiencyAnswer,
};
use codex::rules_core::rules_tables::crb::weapon_tables::{WeaponProficiency, CLASS_WEAPON_PROFICIENCIES};
use codex::rules_core::sheet_rule::{
    Applies, Effect, Fact, Grant, Granter, ProfRef, Provenance, SheetRule, SheetRulePackage, SheetValue, Subject,
};
use codex_ingest::pcgen_import::sheet_rule::closure::{corpus_root, PinnedTree};

fn tree() -> &'static PinnedTree {
    static T: OnceLock<PinnedTree> = OnceLock::new();
    T.get_or_init(|| PinnedTree::load(&corpus_root()).expect("pinned corpus checkout present (scripts/fetch-pcgen-oracle.sh)"))
}

/// The oracle's weapon-proficiency rows, re-read independently: weapon identity (its `KEY:`
/// when present, else its NAME stripped of `.MOD`) -> every `TYPE:` dot-segment its base row
/// and `.MOD` rows carry, with the `path:line` of each contributing row. `_pfs/` overlays
/// excluded, like every other closure index.
struct OracleWeapons {
    tags: BTreeMap<String, BTreeSet<String>>,
    rows: BTreeMap<String, Vec<String>>,
}

impl OracleWeapons {
    fn members_with_all(&self, tags: &[&str]) -> BTreeSet<String> {
        let needles: Vec<String> = tags.iter().map(|t| t.to_ascii_lowercase()).collect();
        self.tags
            .iter()
            .filter(|(_, t)| needles.iter().all(|n| t.iter().any(|x| x.to_ascii_lowercase() == *n)))
            .map(|(w, _)| w.clone())
            .collect()
    }

    /// The first oracle row that carries `tag` as a TYPE segment, for the assertion message.
    fn row_carrying(&self, tag: &str) -> Option<String> {
        let members = self.members_with_all(&[tag]);
        let first = members.iter().next()?;
        self.rows.get(first).and_then(|r| r.first()).map(|r| format!("{r} ({first})"))
    }
}

fn oracle_weapons() -> &'static OracleWeapons {
    static W: OnceLock<OracleWeapons> = OnceLock::new();
    W.get_or_init(|| {
        let mut tags: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        let mut rows: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for file in &tree().files {
            if file.is_pfs {
                continue;
            }
            let base = file.rel_path.rsplit('/').next().unwrap_or(&file.rel_path).to_ascii_lowercase();
            if !base.ends_with("profs_weapon.lst") {
                continue;
            }
            for (n, line) in file.lines.iter().enumerate() {
                let line = line.trim_end_matches('\r');
                let first = line.trim_start();
                if first.is_empty() || first.starts_with('#') {
                    continue;
                }
                let mut fields = line.split('\t').map(str::trim).filter(|f| !f.is_empty());
                let Some(name) = fields.next() else { continue };
                let fields: Vec<&str> = fields.collect();
                let name = name.strip_suffix(".MOD").unwrap_or(name).trim();
                let key = fields.iter().find_map(|f| f.strip_prefix("KEY:")).map(str::trim).filter(|k| !k.is_empty());
                let weapon = key.unwrap_or(name).to_string();
                if weapon.is_empty() {
                    continue;
                }
                let entry = tags.entry(weapon.clone()).or_default();
                for f in &fields {
                    if let Some(v) = f.strip_prefix("TYPE:") {
                        entry.extend(v.split('.').map(str::trim).filter(|s| !s.is_empty()).map(str::to_string));
                    }
                }
                rows.entry(weapon).or_default().push(format!("{}:{}", file.rel_path, n + 1));
            }
        }
        OracleWeapons { tags, rows }
    })
}

fn tier_name(t: WeaponProficiency) -> &'static str {
    match t {
        WeaponProficiency::Simple => "Simple",
        WeaponProficiency::Martial => "Martial",
        WeaponProficiency::Exotic => "Exotic",
    }
}

/// Every problem re-deriving `view` from the oracle, or empty.
fn oracle_problems(view: &ClassWeaponProficiencyView) -> Vec<String> {
    let oracle = oracle_weapons();
    let mut problems = Vec::new();
    for tier in &view.tiers {
        if oracle.row_carrying(tier_name(*tier)).is_none() {
            problems.push(format!("tier {} carried by no oracle weapon row", tier_name(*tier)));
        }
    }
    for name in &view.named {
        // PCGen keys a weapon-proficiency name case-insensitively (`pure_legion_enforcer`'s own
        // row spells `falchion`), so the re-derivation does too.
        if !oracle.rows.keys().any(|k| k.eq_ignore_ascii_case(name)) {
            problems.push(format!("named weapon `{name}` is no oracle weapon-proficiency row"));
        }
    }
    for group in &view.groups {
        let tag = format!("Weapon Group {group}");
        if oracle.row_carrying(&tag).is_none() {
            problems.push(format!("weapon group `{tag}` carried by no oracle weapon row"));
        }
    }
    for conj in &view.all_of {
        let tags: Vec<&str> = conj.iter().map(String::as_str).collect();
        if oracle.members_with_all(&tags).is_empty() {
            problems.push(format!("conjunctive selector {conj:?} selects no oracle weapon row"));
        }
    }
    for set in &view.sets {
        let tags: Vec<&str> = set.label.split('.').collect();
        let expected = oracle.members_with_all(&tags);
        let actual: BTreeSet<String> = set.members.iter().cloned().collect();
        if expected != actual {
            let missing: Vec<_> = expected.difference(&actual).collect();
            let extra: Vec<_> = actual.difference(&expected).collect();
            problems.push(format!("weapon set `{}`: members differ from the oracle rows (oracle-only {missing:?}, reader-only {extra:?})", set.label));
        }
    }
    problems
}

fn slug_of(class_id: &str) -> &str {
    class_id.strip_prefix("class:").unwrap_or(class_id)
}

/// How a static row and the reader disagree, and why -- each entry traced to its mechanism by
/// reading the converted records and the pinned oracle rows (never assumed).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Disagreement {
    /// The reader answers Unknown (never "proficient with nothing").
    ReaderUnknown,
    /// Both answer; the sets differ.
    Differs,
}

/// Every static row the reader does not reproduce at level 1, with the mechanism. A reader bug
/// would be a row here with no converter/oracle mechanism; there is none. Two families:
///
/// - **Converter defects** (the package says less than the oracle; the reader is faithful to
///   the package and, where the walk is empty or a class-line grant is shut, says Unknown):
///   (`grant-by-type` -- `ABILITY:Internal|AUTOMATIC|TYPE=WeaponProf{Simple,Martial}` -- converts
///   since F1c-1 and is no longer a mechanism here; nor is `pre-hoist` -- one line's PRE
///   hoisted onto the class-feature record's own `applies` -- since F1c-2, which un-pinned
///   fighter, medium, mesmerist, psychic and spiritualist: all five now reproduce their static
///   row exactly; nor is `no-class-record` since F1c-3, which gave the four Pathfinder
///   Unchained classes their class principal: three reproduce their static row exactly, and the
///   Unchained Monk's is stale the same way the Monk's is -- see its pin).
/// - **Class-selection gate** (summoner): the proficiency record is reached only through a
///   class-selection pick whose `applies` reads a variable no record in the class closure
///   contributes.
/// - **Stale static rows** (the reader is right per the pinned oracle; the hand-typed row is
///   not; this step does not edit the static rows).
const KNOWN_DISAGREEMENTS: &[(&str, Disagreement, &str)] = &[
    ("monk", Disagreement::Differs, "stale static row: omits Flurry of Blows (cr_abilities_class.lst:2794,2817) and Sword (Temple) (apg_abilities_class.lst:41 .MOD); its Unarmed Strike is granted via the Auto/Monk sets"),
    ("unchained_monk", Disagreement::Differs, "stale static row (F1c-3 gave the class its record): the Unchained Monk is taken on the Monk's class line, whose `Monk` class ability grants Weapon and Armor Proficiency ~ Monk (Flurry of Blows) under PREVAREQ:Monk_CF_Proficiencies,0 (cr_abilities_globalvar.lst:581, cr_abilities_class.lst:2817) -- a variable the Unchained selection does not set -- and apg_abilities_class.lst:41 .MOD adds Sword (Temple); the static row omits both, as the Monk's does"),
    ("summoner", Disagreement::ReaderUnknown, "class-selection gate: summoner_weapon_and_armor_proficiency (Weapon Prof ~ Simple since F1c-1) is reached only via summoner_standard_class, whose applies reads Standard Summoner Allowed == 1, contributed only by advanced_players_guide:class_feature:default (apg_abilities_class.lst:715,717), outside the class closure; empty walk"),
    ("occultist", Disagreement::Differs, "stale static row: omits Simple; the oracle's TYPE=WeaponProfMartial grant (occultist_weapon_and_armor_proficiency) reaches Weapon Prof ~ Simple, tagged WeaponProfSimple.WeaponProfMartial (cr_abilities_class.lst:2800)"),
    ("vigilante", Disagreement::Differs, "stale static row: omits Simple; the oracle's TYPE=WeaponProfMartial grant (vigilante_weapon_and_armor_proficiencies) reaches Weapon Prof ~ Simple, tagged WeaponProfSimple.WeaponProfMartial (cr_abilities_class.lst:2800)"),
    ("psion", Disagreement::Differs, "stale static row: omits All Automatic Proficiencies (Unarmed Strike, Spells (Ray), Spells (Touch), Splash Weapon), up_classes.lst:258"),
    ("ninja", Disagreement::Differs, "stale static row: omits ABILITY:FEAT|AUTOMATIC|Simple Weapon Proficiency and All Automatic Proficiencies, uc_abilities_globalvar.lst:178"),
];

/// (a) F1.2: for each of the 42 static rows, the reader's level-1 answer equals the row (tiers,
/// named, groups as sets) -- except the rows [`KNOWN_DISAGREEMENTS`] names with their mechanism.
/// A NEW disagreement fails (a reader bug or a new data change to trace), and so does a pinned
/// one that has gone away (the converter was fixed: un-pin it).
#[test]
fn every_static_row_equals_the_reader_answer() {
    assert_eq!(CLASS_WEAPON_PROFICIENCIES.len(), 42, "the static table is 42 rows");
    let mut actual: BTreeMap<String, (Disagreement, String)> = BTreeMap::new();
    for row in CLASS_WEAPON_PROFICIENCIES {
        let class = slug_of(row.class_id);
        let view = match class_weapon_proficiency_view(class, 1) {
            ProficiencyAnswer::Known(view) => view,
            ProficiencyAnswer::Unknown { reason } => {
                actual.insert(class.to_string(), (Disagreement::ReaderUnknown, format!("reader Unknown ({reason})")));
                continue;
            }
        };
        let static_tiers: BTreeSet<&str> = row.tiers.iter().map(|t| tier_name(*t)).collect();
        let reader_tiers: BTreeSet<&str> = view.tiers.iter().map(|t| tier_name(*t)).collect();
        let static_named: BTreeSet<String> = row.named.iter().map(|s| s.to_string()).collect();
        let static_groups: BTreeSet<String> = row.weapon_groups.iter().map(|s| s.to_string()).collect();
        let mut diffs = Vec::new();
        if static_tiers != reader_tiers {
            diffs.push(format!("tiers static {static_tiers:?} reader {reader_tiers:?}"));
        }
        // A static named weapon the reader grants through an expanded weapon set (the universal
        // `Auto` set carries Unarmed Strike) is granted either way; it is not a disagreement.
        let set_members: BTreeSet<&String> = view.sets.iter().flat_map(|s| s.members.iter()).collect();
        let static_only: Vec<&String> =
            static_named.difference(&view.named).filter(|n| !set_members.contains(n)).collect();
        let reader_only: Vec<&String> = view.named.difference(&static_named).collect();
        if !static_only.is_empty() || !reader_only.is_empty() {
            diffs.push(format!("named static-only {static_only:?} reader-only {reader_only:?}"));
        }
        if static_groups != view.groups {
            diffs.push(format!("groups static {static_groups:?} reader {:?}", view.groups));
        }
        if !diffs.is_empty() {
            actual.insert(class.to_string(), (Disagreement::Differs, diffs.join("; ")));
        }
    }
    let agree = 42 - actual.len();
    println!("static rows reproduced by the reader at level 1: {agree} of 42; disagreements: {}", actual.len());
    let mut failures = Vec::new();
    for (class, (kind, detail)) in &actual {
        match KNOWN_DISAGREEMENTS.iter().find(|(c, _, _)| c == class) {
            Some((_, pinned, mechanism)) if pinned == kind => println!("  {class}: {detail}\n    mechanism: {mechanism}"),
            Some((_, pinned, _)) => failures.push(format!("{class}: pinned {pinned:?} but now {kind:?}: {detail}")),
            None => failures.push(format!("{class}: NEW disagreement (trace it: stale static row or reader bug): {detail}")),
        }
    }
    for (class, _, mechanism) in KNOWN_DISAGREEMENTS {
        if !actual.contains_key(*class) {
            failures.push(format!("{class}: pinned disagreement ({mechanism}) no longer occurs -- un-pin it"));
        }
    }
    assert!(failures.is_empty(), "{} static-row disagreements not as pinned:\n{}", failures.len(), failures.join("\n"));
}

fn census_ids() -> Vec<String> {
    let path = codex_ingest::repo_root().join("docs/release/SD-36-consolidation/artifacts/epic-f/census-f0.json");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let json: serde_json::Value = serde_json::from_str(&text).expect("census json parses");
    let expected = json["ids"].as_u64().expect("census carries an `ids` count") as usize;
    let ids: Vec<String> = ["classes", "prestige"]
        .iter()
        .flat_map(|list| json[*list].as_array().unwrap_or_else(|| panic!("census carries `{list}`")).iter())
        .map(|row| row["class_id"].as_str().expect("class_id string").to_string())
        .collect();
    assert_eq!(ids.len(), expected, "census `classes` + `prestige` rows must total its own `ids` count");
    ids
}

/// (b) F1.2 extended (review finding 15): for every census class id, every weapon `ProfRef` the
/// reader returns at level 1 and at level 20 is re-derivable from a named oracle row. A class
/// the reader answers Unknown is listed with its reason (printed; not a re-derivation failure).
#[test]
fn every_census_class_answer_is_rederivable_from_the_oracle() {
    let ids = census_ids();
    assert_eq!(ids.len(), 135, "census-f0.json carries 135 class ids");
    let mut problems = Vec::new();
    let mut unknown = Vec::new();
    let mut known = 0usize;
    for id in &ids {
        let class = slug_of(id);
        for level in [1u8, 20] {
            match class_weapon_proficiency_view(class, level) {
                ProficiencyAnswer::Known(view) => {
                    known += 1;
                    for p in oracle_problems(&view) {
                        problems.push(format!("{class} L{level}: {p}"));
                    }
                }
                ProficiencyAnswer::Unknown { reason } => unknown.push(format!("{class} L{level}: {reason}")),
            }
        }
    }
    println!("census reader answers: {known} Known of {} (135 classes x levels 1,20); {} Unknown:", ids.len() * 2, unknown.len());
    for u in &unknown {
        println!("  UNKNOWN {u}");
    }
    assert!(problems.is_empty(), "{} reader rows are not re-derivable from the oracle:\n{}", problems.len(), problems.join("\n"));
}

/// (c) F1.8 (review finding 1): a PRE-gated weapon grant the class-level facts cannot decide is
/// printed with its condition, never counted. Marksman's `Sling Staff (Halfling)`
/// (`marksman_ranged_weapon_proficiency`) is gated on holding Weapon Familiarity (Halfling), a
/// racial fact. Kensai: checked against the converted records, its grants sit on
/// `kensai_weapon_and_armor_proficiency` / `kensai_weapon_proficiency`, reached only through the
/// `magus_archetype_kensai` rule (a `Granter::Rule` edge from the archetype, not a gated fact
/// inside the Magus closure) -- so the contract here is that a plain Magus's class-level walk
/// never holds them, at any level. Bard's archetype-OFF named weapons are the converse: the gate
/// is decidable (no archetype held) and true, so they count.
#[test]
fn a_pre_gated_weapon_proficiency_is_not_granted_unconditionally() {
    use codex::rules_core::sheet_rule::{held_set, CharacterFacts, HeldSeed};

    let marksman = class_weapon_proficiency_view("marksman", 1);
    let view = marksman.known().unwrap_or_else(|| panic!("marksman must be Known: {marksman:?}"));
    assert!(!view.named.contains("Sling Staff (Halfling)"), "a racial-gated grant must not be counted: {view:?}");
    assert!(
        view.printed_conditions.iter().any(|c| c.contains("Sling Staff") && c.contains("Weapon Familiarity ~ Halfling")),
        "the gated grant must print with its condition: {:?}",
        view.printed_conditions
    );

    let package = codex::rules_core::sheet_rule_package::package().as_ref().expect("package loads");
    let kensai = ["ultimate_combat:class_feature:kensai_weapon_and_armor_proficiency", "ultimate_combat:class_feature:kensai_weapon_proficiency"];
    for id in kensai {
        assert!(package.rule(id).is_some(), "{id} is a converted record");
    }
    for level in 1..=20i64 {
        let seed = HeldSeed { classes: vec![("magus".to_string(), level)], ..HeldSeed::default() };
        let facts = CharacterFacts { level, class_levels: vec![("magus".to_string(), level)], ..CharacterFacts::default() };
        let held = held_set(package, &seed, &facts);
        for id in kensai {
            assert!(!held.rules.contains_key(id), "a plain Magus L{level} must not hold the Kensai-only {id}");
        }
        if let ProficiencyAnswer::Known(view) = class_weapon_proficiency_view("magus", level as u8) {
            assert!(
                !view.printed_conditions.iter().chain(view.named.iter()).any(|c| c.to_ascii_lowercase().contains("kensai")),
                "a Kensai-only grant surfaced on a plain Magus L{level}: {view:?}"
            );
        }
    }

    let bard = class_weapon_proficiency_view("bard", 1);
    let view = bard.known().unwrap_or_else(|| panic!("bard must be Known: {bard:?}"));
    for weapon in ["Longsword", "Rapier", "Sap", "Sword (Short)", "Shortbow", "Whip"] {
        assert!(view.named.contains(weapon), "the unarchetyped Bard's archetype-off grant of {weapon} is decidable and true: {view:?}");
    }
}

fn fixture_rule(id: &str, granted_by: Vec<Grant>, grants: Vec<Effect>) -> SheetRule {
    SheetRule {
        id: id.to_string(),
        label: id.rsplit(':').next().unwrap_or(id).to_string(),
        value: SheetValue::Text,
        also: Vec::new(),
        prose: Vec::new(),
        applies: Applies::Always,
        target: None,
        bonus_type: None,
        print: true,
        pool: String::new(),
        tags: Vec::new(),
        subject: Subject::Character,
        repeatable: false,
        granted_by,
        offers: None,
        grants,
        closure_complete: false,
        provenance: Provenance::default(),
    }
}

fn fixture_package(tag: &str) -> SheetRulePackage {
    let mut package = SheetRulePackage::new();
    package.insert_rule(fixture_rule("fixture_book:class:fixture_class", Vec::new(), Vec::new()));
    package.insert_rule(fixture_rule(
        "fixture_book:class_feature:fixture_class_proficiencies",
        vec![Grant { by: Granter::Class { id: "fixture_class".into(), at_level: 1 }, when: Applies::Always }],
        vec![
            Effect::FactGrant(Fact::Proficiency(ProfRef::Weapon("Dagger".into()))),
            Effect::FactGrant(Fact::Proficiency(ProfRef::WeaponGroup(tag.into()))),
        ],
    ));
    package.finish();
    package
}

/// (d) review finding 15: a proficiency tag matching none of {tier, a weapon group carried by
/// an equipment rule, an expanded weapon set} makes the class Unknown, never a fabricated
/// membership. Synthetic fixture: the live corpus carries no such tag.
#[test]
fn an_unrecognized_proficiency_tag_makes_the_class_unknown() {
    let control = class_weapon_proficiency_view_in(&fixture_package("Simple"), "fixture_class", 1);
    let view = control.known().unwrap_or_else(|| panic!("the control fixture must be Known: {control:?}"));
    assert_eq!(view.tiers, vec![WeaponProficiency::Simple]);
    assert!(view.named.contains("Dagger"));

    for tag in ["FabricatedMemberlessTag", "Auto", "Weapon Group Nonexistent"] {
        let answer = class_weapon_proficiency_view_in(&fixture_package(tag), "fixture_class", 1);
        match answer {
            ProficiencyAnswer::Unknown { reason } => assert!(reason.contains(tag), "the reason must name the tag `{tag}`: {reason}"),
            ProficiencyAnswer::Known(view) => panic!("tag `{tag}` must make the class Unknown, got {view:?}"),
        }
    }
}

/// (e) Wizard's record grants five named weapons and NO blanket Simple tier.
#[test]
fn wizard_is_not_granted_the_simple_tier() {
    let answer = class_weapon_proficiency_view("wizard", 1);
    let view = answer.known().unwrap_or_else(|| panic!("wizard must be Known: {answer:?}"));
    assert!(!view.tiers.contains(&WeaponProficiency::Simple), "wizard tiers: {:?}", view.tiers);
    let expected: BTreeSet<String> =
        ["Club", "Dagger", "Crossbow (Heavy)", "Crossbow (Light)", "Quarterstaff"].iter().map(|s| s.to_string()).collect();
    assert_eq!(view.named, expected);
}


/// The converter's own `_defects/` entries (`"<rule id>: <text>"`) whose source rule the class's
/// level-`level` held closure holds -- test-side only (live code never reads `_defects/`, §3.4).
/// `weapon_grant_by_type`: a dropped `ABILITY:Internal|AUTOMATIC|TYPE=WeaponProf*` grant, a
/// definite hole in the weapon closure. `unresolved`: any reference out of a held rule the
/// converter could not resolve, a possible one.
struct ClosureDefects {
    weapon_grant_by_type: Vec<String>,
    unresolved: Vec<String>,
}

fn defect_lines(name: &str) -> Vec<String> {
    let path = codex_ingest::repo_root().join(format!("data/sheet_rules/_defects/{name}.json"));
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn closure_defects(class: &str, level: u8) -> ClosureDefects {
    use codex::rules_core::sheet_rule::{held_set, CharacterFacts, HeldSeed};
    static DEFECTS: OnceLock<(Vec<String>, Vec<String>)> = OnceLock::new();
    let (by_type, unresolved) = DEFECTS.get_or_init(|| (defect_lines("grant-by-type"), defect_lines("unresolved-references")));
    let package = codex::rules_core::sheet_rule_package::package().as_ref().expect("package loads");
    let level = i64::from(level);
    let seed = HeldSeed { classes: vec![(class.to_string(), level)], ..HeldSeed::default() };
    let facts = CharacterFacts { level, class_levels: vec![(class.to_string(), level)], ..CharacterFacts::default() };
    let held = held_set(package, &seed, &facts);
    let in_closure = |line: &&String| line.split_once(": ").is_some_and(|(id, _)| held.rules.contains_key(id));
    ClosureDefects {
        weapon_grant_by_type: by_type.iter().filter(in_closure).filter(|l| l.contains("WeaponProf")).cloned().collect(),
        unresolved: unresolved.iter().filter(in_closure).cloned().collect(),
    }
}

/// The classes the F0 census reports Blocked ONLY on `combat.baseline_weapon_proficiency_unknown`
/// -- derived from the census artifact, never hand-typed.
fn census_blocked_only_on_weapon_proficiency() -> Vec<String> {
    let path = codex_ingest::repo_root().join("docs/release/SD-36-consolidation/artifacts/epic-f/census-f0.json");
    let json: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&path).expect("census readable")).expect("census parses");
    json["classes"]
        .as_array()
        .expect("census `classes`")
        .iter()
        .filter(|row| row["status"] == "Blocked")
        .filter(|row| {
            let ids: BTreeSet<&str> =
                row["blocking_diagnostics"].as_array().into_iter().flatten().filter_map(|d| d["id"].as_str()).collect();
            ids.len() == 1 && ids.contains("combat.baseline_weapon_proficiency_unknown")
        })
        .map(|row| slug_of(row["class_id"].as_str().expect("class_id")).to_string())
        .collect()
}

fn describe_view(view: &ClassWeaponProficiencyView) -> String {
    let mut parts = Vec::new();
    if !view.tiers.is_empty() {
        parts.push(format!("tiers {}", view.tiers.iter().map(|t| tier_name(*t)).collect::<Vec<_>>().join(", ")));
    }
    if !view.named.is_empty() {
        parts.push(format!("named {}", view.named.iter().cloned().collect::<Vec<_>>().join(", ")));
    }
    if !view.groups.is_empty() {
        parts.push(format!("groups {}", view.groups.iter().cloned().collect::<Vec<_>>().join(", ")));
    }
    for conj in &view.all_of {
        parts.push(format!("all of {}", conj.join(" + ")));
    }
    for set in &view.sets {
        parts.push(format!("set `{}` ({} members)", set.label, set.members.len()));
    }
    for c in &view.printed_conditions {
        parts.push(format!("printed: {c}"));
    }
    parts.join("; ")
}

/// The reader's answer for the census classes blocked only on the weapon-proficiency
/// diagnostic, as markdown on stdout (the number step 3 depends on). Run:
/// `cargo test --locked -j 8 -p codex-ingest --test class_weapon_proficiency_via_converter \
///  print_reader_answers_for_the_weapon_proficiency_blocked_classes -- --ignored --nocapture`
#[test]
#[ignore = "report generator; run with --ignored --nocapture"]
fn print_reader_answers_for_the_weapon_proficiency_blocked_classes() {
    let classes = census_blocked_only_on_weapon_proficiency();
    let mut known = 0usize;
    let mut trusted = 0usize;
    let mut out = String::new();
    for class in &classes {
        for level in [1u8, 20] {
            let answer = class_weapon_proficiency_view(class, level);
            let defects = closure_defects(class, level);
            let verdict = match &answer {
                ProficiencyAnswer::Known(view) => {
                    known += 1;
                    if defects.weapon_grant_by_type.is_empty() {
                        trusted += 1;
                    }
                    format!("Known -- {}", describe_view(view))
                }
                ProficiencyAnswer::Unknown { reason } => format!("Unknown -- {reason}"),
            };
            out.push_str(&format!("| {class} | {level} | {verdict} | {} | {} |\n",
                if defects.weapon_grant_by_type.is_empty() { "none".to_string() } else { defects.weapon_grant_by_type.join("<br>") },
                defects.unresolved.len()));
        }
    }
    let mut sweep = String::new();
    let mut known_all_levels = 0usize;
    for class in &classes {
        let known_levels: Vec<u8> = (1..=20u8).filter(|l| class_weapon_proficiency_view(class, *l).known().is_some()).collect();
        if known_levels.len() == 20 {
            known_all_levels += 1;
        }
        sweep.push_str(&format!("| {class} | {} of 20 |\n", known_levels.len()));
    }
    println!("READER-19-BEGIN");
    println!("classes Known at every level 1..=20: {known_all_levels} of {}", classes.len());
    println!();
    println!("| class | levels Known (1..=20) |");
    println!("|---|---|");
    print!("{sweep}");
    println!();
    println!("classes: {} (census-f0.json `classes` rows Blocked only on combat.baseline_weapon_proficiency_unknown)", classes.len());
    println!("answers: {known} Known of {} (class x level 1,20); Known with no dropped weapon grant-by-type in the held closure: {trusted}", classes.len() * 2);
    println!();
    println!("| class | level | reader answer | dropped weapon grant-by-type in held closure (_defects/grant-by-type.json) | unresolved refs out of held rules (_defects/unresolved-references.json) |");
    println!("|---|---|---|---|---|");
    print!("{out}");
    println!("READER-19-END");
}
