//! SD-36 Epic F1-3 -- weapon selectors resolved at ingest (`epic-f-class-completion.md`
//! §3.2/§3.3, review finding 15). Today's `AUTO:WEAPONPROF|TYPE=` arm in `convert.rs` joins a
//! conjunctive selector into one lossy word (`"Light.Martial"`) and treats every other `TYPE=`
//! value as a bare `WeaponGroup`, including tags that name no real tier, weapon group, or
//! membership at all (`Auto`, `KoboldTailAttachment`) -- these read as a fabricated granted
//! proficiency. Live-corpus, real-oracle tests (`convert_record` over the pinned tree, exactly
//! what `sheet_rule_convert --one <id>` prints for n=1 -- never writing `data/sheet_rules/`,
//! matching `sheet_rule_link_repair.rs`'s and `sheet_rule_gated_fact_grants.rs`'s own
//! in-memory `run()`/`convert_record` pattern).
//!
//! **n=1 units:**
//! - `ultimate_combat:class_feature:samurai_proficiencies` (`uc_abilities_class.lst:186`):
//!   `AUTO:WEAPONPROF|TYPE=Samurai` -- a lone membership tag whose members exist ONLY as an
//!   ADDED tag on the weapons that carry it (`uc_profs_weapon.lst:100-102`'s `Katana.MOD`/
//!   `Naginata.MOD`/`Wakizashi.MOD` rows), never as their own record.
//! - `ultimate_psionics:class:marksman` (`up_classes.lst:171`, the same PRE-gated row F1-2's own
//!   n=1 already covers for the gate-carriage half): `AUTO:WEAPONPROF|TYPE=Light.Martial` -- the
//!   doc's own "conjunction" example, and one whose "Light" segment the live weapon table cannot
//!   answer directly (`WeaponTableEntry` has no weight-class field), so it must ALSO expand to a
//!   set, not a two-element `WeaponAllOf`.
//! - `core_rulebook:class_feature:weapon_prof_auto` (`cr_abilities_class.lst:2799`, `KEY:Weapon
//!   Prof ~ Auto`): `AUTO:WEAPONPROF|TYPE=Auto` -- review finding 15's own first named junk tag.
//!   Granted internally by 11 different class/archetype records (the finding's own "11
//!   occurrences" figure), all sharing this one record's conversion.
//! - `core_rulebook:class_feature:weapon_and_armor_proficiency_commoner`
//!   (`cr_abilities_class.lst:2825`): Commoner's "proficient with one simple weapon" is
//!   `BONUS:ABILITYPOOL|Simple Weapon Proficiency Choice|1`, never an `AUTO:WEAPONPROF` row at
//!   all -- a regression pin that this step's change to the `WEAPONPROF` arm leaves the
//!   pool-based choice lane untouched.

use std::sync::OnceLock;

use codex::rules_core::sheet_rule::{Applies, Effect, Fact, ProfRef};
use codex_ingest::pcgen_import::sheet_rule::closure::{corpus_root, Closure, PinnedTree};
use codex_ingest::pcgen_import::sheet_rule::convert::{convert_record, Converted};
use codex_ingest::pcgen_import::sheet_rule::ctx::CorpusIndex;
use codex_ingest::pcgen_import::sheet_rule::{build_index, load_population};

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

fn convert_unit(id: &str) -> Converted {
    let s = shared();
    let pos = s.index.records.iter().position(|r| r.id == id).unwrap_or_else(|| panic!("unit {id} is in docs/work-inventory.json"));
    convert_record(&s.tree, &s.index, &s.index.records[pos], &s.closures[pos])
}

/// The fact an effect carries, gated or not -- so a test can check "was this fact granted at
/// all" without caring whether F1-2's `GatedFactGrant` wraps it.
fn effect_fact(e: &Effect) -> Option<&Fact> {
    match e {
        Effect::FactGrant(f) => Some(f),
        Effect::GatedFactGrant { fact, .. } => Some(fact),
        _ => None,
    }
}

/// Independent re-derivation of Samurai weapon membership, reading `uc_profs_weapon.lst`
/// directly (never calling `weapon_membership::WeaponMembershipIndex`, so this pins the
/// CONVERTER against the oracle text itself, not against its own index's own idea of the
/// answer): every weapon name whose base row or `.MOD` row TYPE facet carries the exact
/// `Samurai` dot-segment.
fn samurai_members_from_the_oracle_file() -> Vec<String> {
    let path = corpus_root().join("pathfinder/paizo/roleplaying_game/ultimate_combat/uc_profs_weapon.lst");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut members = std::collections::BTreeSet::new();
    for line in text.lines() {
        let line = line.trim_end_matches('\r');
        let trimmed = line.trim_start();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let mut fields = line.split('\t').filter(|f| !f.trim().is_empty());
        let Some(name) = fields.next() else { continue };
        let carries_samurai = fields.filter_map(|f| f.strip_prefix("TYPE:")).any(|v| v.split('.').any(|seg| seg == "Samurai"));
        if !carries_samurai {
            continue;
        }
        let weapon = name.trim().strip_suffix(".MOD").unwrap_or(name.trim()).trim().to_string();
        if !weapon.is_empty() {
            members.insert(weapon);
        }
    }
    members.into_iter().collect()
}

/// F1.2/§3.3: membership for a lone tag the live weapon table cannot answer generically
/// (`Samurai` is neither a tier nor a `Weapon Group <x>` tag) comes from the oracle's own
/// weapon-proficiency rows at ingest, never a hand-typed list -- re-derived here directly from
/// the pinned `.lst` text, independent of the converter's own membership index.
#[test]
fn samurai_set_members_come_from_the_oracle_rows() {
    let expected = samurai_members_from_the_oracle_file();
    assert_eq!(expected, vec!["Katana".to_string(), "Naginata".to_string(), "Wakizashi".to_string()], "sanity: the re-derivation itself must find the three named members (uc_profs_weapon.lst:82,100-102,62,80), not zero and not a different set");

    let c = convert_unit("ultimate_combat:class_feature:samurai_proficiencies");
    assert!(c.refusals.is_empty(), "samurai refusals: {:?}", c.refusals);
    assert!(c.defects.is_empty(), "Samurai's TYPE=Samurai selector resolves to real members; it must carry no defect: {:?}", c.defects);

    let grants: Vec<&Effect> = c.rules.iter().flat_map(|r| r.grants.iter()).collect();
    let expected_fact = Fact::Proficiency(ProfRef::WeaponSet { label: "Samurai".into(), members: expected });
    assert!(
        grants.iter().filter_map(|g| effect_fact(g)).any(|f| *f == expected_fact),
        "Samurai: expected {expected_fact:?} among {:?}",
        grants
    );
    // Never the old lossy bare tag either.
    let old_shape = Fact::Proficiency(ProfRef::WeaponGroup("Samurai".into()));
    assert!(!grants.iter().filter_map(|g| effect_fact(g)).any(|f| *f == old_shape), "Samurai must not stay the un-expanded WeaponGroup(\"Samurai\"): {:?}", grants);
}

/// F1-3/§3.2: a conjunctive `TYPE=` selector converts to a real list, never the joined word
/// `"Light.Martial"` -- and because `Light` is a tag the live `WeaponTableEntry` cannot answer
/// (it has no weight-class field, only tier/group/melee/ranged), the conjunction expands to a
/// `WeaponSet`, not a two-element `WeaponAllOf`. The row's own PRE-gate (F1-2) must still carry.
#[test]
fn marksman_conjunction_is_a_list_not_a_joined_word() {
    let c = convert_unit("ultimate_psionics:class:marksman");
    assert!(c.refusals.is_empty(), "marksman refusals: {:?}", c.refusals);
    let grants: Vec<&Effect> = c.rules.iter().flat_map(|r| r.grants.iter()).collect();

    let old_shape = Fact::Proficiency(ProfRef::WeaponGroup("Light.Martial".into()));
    assert!(!grants.iter().filter_map(|g| effect_fact(g)).any(|f| *f == old_shape), "Light.Martial must never convert to the old joined-word WeaponGroup: {:?}", grants);

    let members = grants
        .iter()
        .find_map(|g| match g {
            Effect::GatedFactGrant { fact: Fact::Proficiency(ProfRef::WeaponSet { label, members }), when } if label == "Light.Martial" => {
                assert_ne!(*when, Applies::Always, "the conjunction keeps its own PRE-gate (F1-2, `!PREABILITY:1,CATEGORY=Archetype,TYPE.MarksmanLightMartialProficiency`)");
                Some(members.clone())
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("expected a GatedFactGrant carrying a WeaponSet labeled \"Light.Martial\": {:?}", grants));

    assert!(!members.is_empty(), "a real list, never an empty set");
    // Independently checked against core_rulebook/cr_profs_weapon.lst:44-53: every one of these
    // carries BOTH a `Martial` and a `Light` TYPE dot-segment on its own base row.
    for w in ["Handaxe", "Kukri", "Short Sword", "Starknife", "Sap", "Throwing Axe", "Light Hammer", "Light Pick", "Spiked Armor"] {
        assert!(members.contains(&w.to_string()), "{w} is Martial+Light in the pinned oracle and must be a member: {:?}", members);
    }
    // "Light Flail"'s own TYPE facet (`cr_profs_weapon.lst:56`) carries no `Light` dot-segment
    // at all -- only its NAME does -- so a substring-style match would wrongly include it.
    assert!(!members.contains(&"Light Flail".to_string()), "Light Flail must not be a false-positive member (its own TYPE facet has no Light tag): {:?}", members);
}

/// Review finding 15: `TYPE=Auto` is a PCGen bookkeeping marker (PF1's own baseline weapons --
/// Unarmed Strike, Grapple, Splash Weapon -- carry `Auto` as an incidental TYPE tag, but no class
/// ever intentionally grants "the Auto weapons" as a feature), never a real weapon group or
/// membership set. It must become a named defect, not a fabricated proficiency, and the record
/// must still convert (no-carve-outs: an unfindable membership is a number to report, not a
/// reason to drop the whole record).
#[test]
fn an_unrecognized_proficiency_tag_becomes_a_defect_not_a_grant() {
    let c = convert_unit("core_rulebook:class_feature:weapon_prof_auto");
    assert!(c.refusals.is_empty(), "the record must still convert: {:?}", c.refusals);

    let grants: Vec<&Effect> = c.rules.iter().flat_map(|r| r.grants.iter()).collect();
    assert!(
        !grants.iter().filter_map(|g| effect_fact(g)).any(|f| matches!(f, Fact::Proficiency(_))),
        "TYPE=Auto must never become a granted proficiency of any shape: {:?}",
        grants
    );

    let lines = c.defects.get("unrecognized-proficiency-tag").cloned().unwrap_or_default();
    assert!(
        lines.iter().any(|l| l.contains("weapon_prof_auto") && l.contains("TYPE=Auto")),
        "expected an unrecognized-proficiency-tag defect naming the TYPE=Auto selector: {:?}",
        c.defects
    );
}

/// Commoner's "proficient with one simple weapon" is `BONUS:ABILITYPOOL|Simple Weapon
/// Proficiency Choice|1` on its own class-feature row -- never an `AUTO:WEAPONPROF` row at all.
/// This step changes only the `WEAPONPROF` arm; Commoner's own choice-pool conversion must stay
/// exactly as it was, carrying no `Proficiency` fact and no new defect.
#[test]
fn single_simple_weapon_choice_stays_a_choice() {
    let c = convert_unit("core_rulebook:class_feature:weapon_and_armor_proficiency_commoner");
    assert!(c.refusals.is_empty(), "commoner refusals: {:?}", c.refusals);
    assert!(c.defects.is_empty(), "Commoner's own row never reaches the AUTO:WEAPONPROF arm this step changed: {:?}", c.defects);

    let grants: Vec<&Effect> = c.rules.iter().flat_map(|r| r.grants.iter()).collect();
    assert!(
        !grants.iter().filter_map(|g| effect_fact(g)).any(|f| matches!(f, Fact::Proficiency(_))),
        "Commoner's row carries no AUTO:WEAPONPROF token at all; it must carry no Proficiency fact before or after this step: {:?}",
        grants
    );
}
