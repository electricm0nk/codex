//! SD-36 Epic F1-2 -- conditional grants keep their gate (`epic-f-class-completion.md`
//! §0.1a/§3.1 item 4/§3.2, review finding 1). Today's `AUTO` arm in
//! `crates/codex-ingest/src/pcgen_import/sheet_rule/convert.rs` computes the row's own PRE-gate
//! (`when = gates_of(ctx, &gates, level_gate)?`), then discards it (`let _ = when;`) and emits a
//! plain `Effect::FactGrant` regardless -- a PRE-gated proficiency reads as unconditionally
//! granted, the exact hazard `docs/governance/no-stub-mvp-doctrine.md` forbids. These are the
//! live-corpus, real-oracle tests (`convert_record` over the pinned tree, exactly what
//! `sheet_rule_convert --one <id>` prints for n=1 -- never writing `data/sheet_rules/` to disk,
//! matching `sheet_rule_link_repair.rs`'s own `run()`-in-memory pattern from F1-1).
//!
//! **n=1 units and why (verified against the pinned oracle, not assumed from the spec's own
//! citation):**
//! - `ultimate_psionics:class:marksman` -- `pathfinder/dreamscarred_press/ultimate_psionics/
//!   up_classes.lst:171`: `AUTO:SHIELDPROF|SHIELDTYPE=Buckler|!PREABILITY:1,CATEGORY=Archetype,
//!   TYPE.MarksmanBucklerProficiency` and `AUTO:WEAPONPROF|TYPE=Light.Martial|!PREABILITY:1,
//!   CATEGORY=Archetype,TYPE.MarksmanLightMartialProficiency` -- the doc's own "conjunction"
//!   example: a `TYPE=` selector (`ProfRef::WeaponGroup("Light.Martial")`), PRE-gated on NOT
//!   holding a specific archetype ability (an "archetype-only" shape: granted unless an
//!   archetype replaces it).
//! - `core_rulebook:class_feature:bard_weapon_proficiencies` -- `core_rulebook/cr_abilities_
//!   class.lst:2788`: `AUTO:WEAPONPROF|Longsword|Rapier|Sap|Sword (Short)|Shortbow|Whip|
//!   !PREABILITY:1,CATEGORY=Archetype,TYPE.BardWeaponProficiencies` -- six literal NAMED
//!   weapons, same archetype-off gate shape as Marksman's.
//!
//!   The spec's own §0.1a/F1.8 text names "Kensai's PRE-gated grant" as the second unit. Checked
//!   directly against the pinned oracle (`uc_abilities_class_um.lst:16`, `KEY:Kensai ~ Weapon
//!   and Armor Proficiency`): that record's only weapon-facing row is `ABILITY:Internal|
//!   AUTOMATIC|TYPE=WeaponProfSimple` -- a grant-BY-TYPE ABILITY row (mechanism unrelated to
//!   this gate-drop; today it is a `grant-by-type` defect, `convert.rs:1376-1378`), not an
//!   `AUTO:WEAPONPROF` row with its own PRE-gate at all; §1's own count places "Kensai" in
//!   mechanism F (unfindable references), explicitly out of Epic F1's scope
//!   (`epic-f-class-completion.md` line 206). Kensai does not exhibit the shape this test needs.
//!   Bard is the real record substituted in its place -- verified present in the pinned oracle
//!   with the literal-named-weapon PRE-gated shape the doc describes, rather than fabricating
//!   coverage against a citation that does not hold up under a real-corpus check
//!   (`every-figure-states-its-denominator`: ground the claim in the corpus, not the prose).

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

/// F1.8: a PRE-gated `AUTO:WEAPONPROF`/`AUTO:SHIELDPROF` row must convert to
/// `Effect::GatedFactGrant { fact, when }` with `when != Applies::Always`, never a plain
/// `Effect::FactGrant` -- the fabricated-fact hazard review finding 1 named. Marksman's TYPE=
/// selector (conjunction) and Bard's literal named weapons cover both `ProfRef` shapes the AUTO
/// arm emits.
#[test]
fn a_pre_gated_weapon_proficiency_is_not_granted_unconditionally() {
    let marksman = convert_unit("ultimate_psionics:class:marksman");
    assert!(marksman.refusals.is_empty(), "marksman refusals: {:?}", marksman.refusals);
    let marksman_grants: Vec<&Effect> = marksman.rules.iter().flat_map(|r| r.grants.iter()).collect();

    {
        let (fact, label) = (Fact::Proficiency(ProfRef::ShieldGroup("Buckler".into())), "the Buckler shield proficiency");
        assert!(
            !marksman_grants.iter().any(|g| matches!(g, Effect::FactGrant(f) if *f == fact)),
            "Marksman: {label} must not convert to an unconditional FactGrant (its PRE-gate \
             `!PREABILITY:1,CATEGORY=Archetype,TYPE.Marksman...Proficiency` was dropped): {:?}",
            marksman_grants
        );
        let gated = marksman_grants.iter().find_map(|g| match g {
            Effect::GatedFactGrant { fact: f, when } if *f == fact => Some(when),
            _ => None,
        });
        let when = gated.unwrap_or_else(|| panic!("Marksman: {label} must convert to a GatedFactGrant carrying its PRE-gate: {marksman_grants:?}"));
        assert_ne!(*when, Applies::Always, "Marksman: {label}'s gate must not collapse to Applies::Always -- it has a real PRE token");
    }

    // The `TYPE=Light.Martial` conjunction (SD-36 Epic F1-3, `sheet_rule_weapon_selectors.rs`
    // pins its own member list against the oracle rows): it must still keep this record's own
    // PRE-gate, whatever shape F1-3 converts the selector itself to -- not the plain unconditional
    // FactGrant this test's whole point rules out.
    assert!(
        !marksman_grants.iter().any(|g| matches!(g, Effect::FactGrant(Fact::Proficiency(ProfRef::WeaponSet { label, .. })) if label == "Light.Martial")),
        "Marksman: the Light.Martial conjunction must not convert to an unconditional FactGrant (its PRE-gate was dropped): {:?}",
        marksman_grants
    );
    let light_martial_when = marksman_grants.iter().find_map(|g| match g {
        Effect::GatedFactGrant { fact: Fact::Proficiency(ProfRef::WeaponSet { label, .. }), when } if label == "Light.Martial" => Some(when),
        _ => None,
    });
    let when = light_martial_when.unwrap_or_else(|| panic!("Marksman: the Light.Martial conjunction must convert to a GatedFactGrant carrying its PRE-gate: {marksman_grants:?}"));
    assert_ne!(*when, Applies::Always, "Marksman: the Light.Martial conjunction's gate must not collapse to Applies::Always -- it has a real PRE token");

    let bard = convert_unit("core_rulebook:class_feature:bard_weapon_proficiencies");
    assert!(bard.refusals.is_empty(), "bard refusals: {:?}", bard.refusals);
    let bard_grants: Vec<&Effect> = bard.rules.iter().flat_map(|r| r.grants.iter()).collect();
    for weapon in ["Longsword", "Rapier", "Sap", "Sword (Short)", "Shortbow", "Whip"] {
        let fact = Fact::Proficiency(ProfRef::Weapon(weapon.into()));
        assert!(
            !bard_grants.iter().any(|g| matches!(g, Effect::FactGrant(f) if *f == fact)),
            "Bard: {weapon} must not convert to an unconditional FactGrant (its archetype-off \
             PRE-gate was dropped): {:?}",
            bard_grants
        );
        let gated = bard_grants.iter().find_map(|g| match g {
            Effect::GatedFactGrant { fact: f, when } if *f == fact => Some(when),
            _ => None,
        });
        let when = gated.unwrap_or_else(|| panic!("Bard: {weapon} must convert to a GatedFactGrant carrying its PRE-gate: {bard_grants:?}"));
        assert_ne!(*when, Applies::Always, "Bard: {weapon}'s gate must not collapse to Applies::Always -- it has a real PRE token");
    }
}

/// The converse: an AUTO row the oracle leaves genuinely ungated (no PRE token at all) must
/// keep converting to a plain `FactGrant`, never a `GatedFactGrant` with a trivial
/// `Applies::Always` -- the smallest-schema-change contract (existing converted JSON, e.g.
/// Druid's own weapon list, deserializes unchanged; `ungated_fact_grants_deserialize_unchanged`
/// pins the on-disk bytes, this pins the CONVERTER's own live behavior against the real
/// oracle).
#[test]
fn an_ungated_weapon_proficiency_still_converts_to_a_plain_fact_grant() {
    let druid = convert_unit("core_rulebook:class_feature:weapon_and_armor_proficiency_druid");
    assert!(druid.refusals.is_empty(), "druid refusals: {:?}", druid.refusals);
    let druid_grants: Vec<&Effect> = druid.rules.iter().flat_map(|r| r.grants.iter()).collect();
    assert!(
        !druid_grants.iter().any(|g| matches!(g, Effect::GatedFactGrant { .. })),
        "Druid's weapon list carries no PRE token in the oracle; it must stay a plain FactGrant: {:?}",
        druid_grants
    );
    let expected = Fact::Proficiency(ProfRef::Weapon("Club".into()));
    assert!(druid_grants.iter().any(|g| matches!(g, Effect::FactGrant(f) if *f == expected)), "{:?}", druid_grants);
}
