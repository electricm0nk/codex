//! SD-36 Epic F1c-5 (defect D8): a pick into an ability category whose pool a variable sizes.
//!
//! `apg_abilitycategories.lst:267` declares `ABILITYCATEGORY:Summoner Class Selection` with
//! `POOL:Pool_Summoner_Class_Selection CATEGORY:Class TYPE:Summoner Class Selection`; the Summoner
//! ability (`apg_abilities_class.lst:739`) fills it with `BONUS:VAR|Pool_Summoner_Class_Selection|1`.
//! Before D8 the converter wrote neither a choice on the Summoner ability nor an edge onto the
//! pool's members, so no walk reached `Summoner ~ Standard Class` (`:741`), the record that grants
//! the Summoner's weapon and armor proficiency (`:747`).

use std::sync::OnceLock;

use codex::rules_core::sheet_rule::{var_id, Applies, Choice, Expr, Grant, Granter, OptionSet};
use codex_ingest::pcgen_import::sheet_rule::{self, closure::corpus_root, closure::PinnedTree, pool_pick};

const SUMMONER: &str = "advanced_players_guide:class_feature:summoner";
const STANDARD: &str = "advanced_players_guide:class_feature:summoner_standard_class";
const UNCHAINED: &str = "pathfinder_unchained:class_feature:summoner_unchained_class";

fn tree() -> &'static PinnedTree {
    static T: OnceLock<PinnedTree> = OnceLock::new();
    T.get_or_init(|| PinnedTree::load(&corpus_root()).expect("pinned corpus checkout present (scripts/fetch-pcgen-oracle.sh)"))
}

/// n=1 through `sheet_rule_convert --one`'s own path (`sheet_rule::convert_one`): the Summoner
/// ability offers one pick among the category's members, counted by the pool variable, and each
/// member is granted by that choice.
#[test]
fn summoner_class_selection_is_a_converted_choice_with_members() {
    let (converted, _rows) = sheet_rule::convert_one(&codex_ingest::repo_root(), SUMMONER).expect("the Summoner ability converts");
    let principal = converted.rules.first().expect("a principal rule");
    assert_eq!(
        principal.offers,
        Some(Choice {
            id: SUMMONER.to_string(),
            count: Expr::Var(var_id("Pool_Summoner_Class_Selection")),
            from: OptionSet::Rules { pool: "class".into(), tags: vec!["Summoner Class Selection".into()], requires: Applies::Always },
        }),
        "the Summoner ability's pick (apg_abilities_class.lst:739 BONUS:VAR|Pool_Summoner_Class_Selection|1)"
    );
    let edge = Grant { by: Granter::Choice(SUMMONER.to_string()), when: Applies::Always };
    let members: Vec<&String> = converted.grants_out.iter().filter(|(_, g)| g == &edge).map(|(t, _)| t).collect();
    assert_eq!(members, vec![STANDARD, UNCHAINED], "the pool's members, in oracle order (apg :741, pu_abilities_class.lst:117)");
}

/// The category the Summoner pick names is one variable pool whose members all resolve; a pool
/// whose oracle member has no converted record names it by row (`pool-member-unconverted`), never
/// a guess. `Level 4 Primal Choice` (`acg_abilitycategories.lst:159`, `POOL:Pool_BarbarianPrimalistChoice4`,
/// filled by the Primalist's `Primal Choice 4`) has two oracle members; `Primal Choice ~ Bloodline
/// Power 4` (`acg_abilities_class.lst:2801`) is carried by no corpus record.
#[test]
fn a_pool_member_that_does_not_resolve_is_a_named_defect() {
    let (pools, _) = pool_pick::variable_pools(tree());
    let summoner = pools.get("POOL_SUMMONER_CLASS_SELECTION").expect("Summoner Class Selection is a variable pool");
    assert_eq!(summoner.decl, "pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_abilitycategories.lst:267");
    assert_eq!((summoner.parent.as_str(), summoner.tags.clone()), ("CLASS", vec!["Summoner Class Selection".to_string()]));

    let package_defects: Vec<String> = serde_json::from_str(
        &std::fs::read_to_string(codex_ingest::repo_root().join("data/sheet_rules/_defects/pool-member-unconverted.json")).expect("the defect file is written"),
    )
    .expect("defect file parses");
    let primal: Vec<&String> = package_defects.iter().filter(|l| l.starts_with("pool:level_4_primal_choice ")).collect();
    assert_eq!(
        primal,
        vec!["pool:level_4_primal_choice (pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_abilitycategories.lst:159): member PRIMAL CHOICE ~ BLOODLINE POWER 4 (pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_abilities_class.lst:2801): no converted record stands for this member"],
        "the unconverted member is a named defect row citing its oracle row"
    );
    let (filled_member, _) = sheet_rule::convert_one(&codex_ingest::repo_root(), "advanced_class_guide:class_feature:primalist_primal_choice_4").expect("the filler converts");
    let offered = filled_member.grants_out.iter().filter(|(_, g)| g.by == Granter::Choice("advanced_class_guide:class_feature:primalist_primal_choice_4".into())).count();
    assert_eq!(offered, 1, "only the member a converted record stands for gets an edge: {:?}", filled_member.grants_out);
    assert!(
        !package_defects.iter().any(|l| l.starts_with("pool:summoner_class_selection ")),
        "the Summoner Class Selection members all resolve: {package_defects:#?}"
    );
}
