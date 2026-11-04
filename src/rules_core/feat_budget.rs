//! How many feats a Pathfinder 1e character may hold, pool by pool, and which pool each held feat
//! is charged to.
//!
//! The pools come from the rules the character holds, never a per-class table: every held rule
//! whose `target` is `Pool(<feat pool>)` adds its evaluated value, so a Fighter 3's
//! `fighter_bonus_feat` reads 2 off `fighter_bonus_feats` and an alternate racial trait that replaces
//! `human_bonus_feat` takes the racial pool away. The one pool with no record is `level`: a feat at
//! every odd character level is a rule of the game, not of any class or race.
//!
//! `selected_feats` is a flat list that does not say which pool an entry came from. Each entry is
//! charged by a best-case assignment: restricted pools fill before open ones, and an entry moves to
//! another pool when that lets a later entry fit. So a character is over budget only when no
//! assignment fits every feat, and a pool the book restricts stays open to what only it can take.

use std::collections::BTreeMap;

use crate::rules_core::character_input::CharacterInput;
use crate::rules_core::level_up_option_filter::FEAT_POOL;
use crate::rules_core::pilot_compute::{ComputationExplanation, PilotBaseChassisComputation};
use crate::rules_core::sheet_rule::{
    evaluate_applies, evaluate_expr, held_set, id_slug, sibling_line_gate, split_rule_id, BonusTarget, CharacterFacts, EvalContext,
    HeldSeed, OptionSet, SheetRule, SheetRulePackage, SheetValue,
};

/// A feat at every odd character level.
pub const LEVEL_POOL: &str = "level";
/// `Pool(feat)` handed out by a racial trait (a human's bonus feat).
pub const RACIAL_POOL: &str = "racial";
/// `Pool(feat)` handed out by anything else: an open feat with no list of its own.
pub const OPEN_BONUS_POOL: &str = "bonus";

const EXPLANATION_PREFIX: &str = "feats.budget";
const FIGHTER_BONUS_FEAT_POOL: &str = "fighter_bonus_feat";
/// The combat baseline requires Weapon Focus of every class, and creation seeds it in this token
/// shape (`feat_pillar_and_pool_aggregation.rs`, `unmet_combat_posture_conditions`). A character
/// with no Fighter pool holds it by that rule, not by spending a pick.
const POSTURE_FEAT_TOKEN: &str = "feat:weapon_focus";

/// Whether this character level grants a feat.
pub fn level_grants_feat(character_level: u8) -> bool {
    character_level % 2 == 1
}

/// Every character level up to `cap` that grants a feat.
pub fn levels_granting_a_feat(cap: u8) -> Vec<u8> {
    (1..=cap).filter(|level| level_grants_feat(*level)).collect()
}

/// One pool of feat picks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatPool {
    pub id: String,
    pub granted: i64,
    pub spent: i64,
    /// `None`: any feat. `Some`: a feat carrying every tag of one of these sets (the rule's `offers`).
    admits: Option<Vec<Vec<String>>>,
    /// The labels of the rules that grant it.
    sources: Vec<String>,
}

impl FeatPool {
    pub fn remaining(&self) -> i64 {
        self.granted - self.spent
    }

    /// The name a player reads: `level`, `racial`, `fighter bonus feat`.
    pub fn display_name(&self) -> String {
        self.id.replace('_', " ")
    }

    fn admits(&self, feat: &HeldFeat) -> bool {
        match &self.admits {
            None => true,
            Some(sets) => sets.iter().any(|tags| {
                feat.tag_sets.iter().any(|held| tags.iter().all(|t| held.iter().any(|h| h.eq_ignore_ascii_case(t))))
            }),
        }
    }
}

/// One entry of `selected_feats` and the tags of the records it can name.
#[derive(Debug, Clone)]
struct HeldFeat {
    tag_sets: Vec<Vec<String>>,
}

/// A character's feat pools with every held feat charged.
#[derive(Debug, Clone)]
pub struct FeatBudget {
    pools: Vec<FeatPool>,
    held: Vec<HeldFeat>,
    over_by: i64,
}

impl FeatBudget {
    fn settled(mut pools: Vec<FeatPool>, held: Vec<HeldFeat>) -> FeatBudget {
        pools.sort_by_key(fill_rank);
        let assignment = assign(&pools, &held);
        for pool in &mut pools {
            pool.spent = 0;
        }
        let mut over_by = 0;
        for slot in &assignment {
            match slot {
                Some(pool) => pools[*pool].spent += 1,
                None => over_by += 1,
            }
        }
        // A feat no assignment fits is charged to the last open pool, so `remaining` goes negative
        // on one row and the rows alone say by how much.
        if over_by > 0
            && let Some(open) = pools.iter_mut().rev().find(|pool| pool.admits.is_none())
        {
            open.spent += over_by;
        }
        pools.sort_by_key(|pool| (display_rank(pool), pool.id.clone()));
        FeatBudget { pools, held, over_by }
    }

    pub fn pools(&self) -> &[FeatPool] {
        &self.pools
    }

    pub fn pool(&self, id: &str) -> Option<&FeatPool> {
        self.pools.iter().find(|pool| pool.id == id)
    }

    /// How many held feats no assignment of the pools can fit.
    pub fn over_by(&self) -> i64 {
        self.over_by
    }

    /// `feats.budget.<pool>.{granted,spent,remaining}` for every pool.
    pub fn explanation_rows(&self) -> Vec<ComputationExplanation> {
        let row = |pool: &FeatPool, field: &str, value: i64, detail: String| ComputationExplanation {
            id: format!("{EXPLANATION_PREFIX}.{}.{field}", pool.id),
            value: i16::try_from(value).unwrap_or(i16::MAX),
            detail,
        };
        self.pools
            .iter()
            .flat_map(|pool| {
                [
                    row(pool, "granted", pool.granted, format!("Feats granted by {}", pool.sources.join(", "))),
                    row(pool, "spent", pool.spent, format!("Held feats charged to the {} pool", pool.display_name())),
                    row(pool, "remaining", pool.remaining(), format!("Granted minus spent in the {} pool", pool.display_name())),
                ]
            })
            .collect()
    }

    /// Why this character may not take `feat_id` now, naming the pool; `None` when a pool has room.
    pub fn refusal(&self, package: &SheetRulePackage, feat_id: &str) -> Option<String> {
        if self.over_by > 0 {
            let over: Vec<String> = self
                .pools
                .iter()
                .filter(|pool| pool.remaining() < 0)
                .map(|pool| format!("the {} pool has {} of {} spent", pool.display_name(), pool.spent, pool.granted))
                .collect();
            return Some(format!(
                "This character is over its feat budget by {}: {}. Remove a feat before adding another.",
                self.over_by,
                over.join(", ")
            ));
        }
        let wanted = held_feat(package, feat_id);
        let mut held = self.held.clone();
        held.push(wanted.clone());
        if FeatBudget::settled(self.pools.clone(), held).over_by == 0 {
            return None;
        }
        let full: Vec<String> = self
            .pools
            .iter()
            .filter(|pool| pool.admits(&wanted))
            .map(|pool| format!("the {} pool has {} of {} left", pool.display_name(), pool.remaining().max(0), pool.granted))
            .collect();
        Some(format!("'{feat_id}' cannot be taken: no feat pool has room for it ({}).", full.join(", ")))
    }
}

/// The order pools fill in: a restricted pool first, the open ones last, `level` very last.
fn fill_rank(pool: &FeatPool) -> u8 {
    match (pool.admits.is_some(), pool.id.as_str()) {
        (true, _) => 0,
        (false, LEVEL_POOL) => 4,
        (false, RACIAL_POOL) => 3,
        (false, OPEN_BONUS_POOL) => 2,
        (false, _) => 1,
    }
}

/// The order rows read in: `level`, `racial`, `bonus`, then the class pools by name.
fn display_rank(pool: &FeatPool) -> u8 {
    match pool.id.as_str() {
        LEVEL_POOL => 0,
        RACIAL_POOL => 1,
        OPEN_BONUS_POOL => 2,
        _ => 3,
    }
}

/// Places `feat`, moving a feat already placed when that makes room (one augmenting path).
fn place(feat: usize, pools: &[FeatPool], held: &[HeldFeat], seen: &mut [bool], slots: &mut [Option<usize>]) -> bool {
    for (index, pool) in pools.iter().enumerate() {
        if seen[index] || !pool.admits(&held[feat]) {
            continue;
        }
        seen[index] = true;
        let charged = slots.iter().filter(|slot| **slot == Some(index)).count() as i64;
        if charged < pool.granted {
            slots[feat] = Some(index);
            return true;
        }
        for other in 0..held.len() {
            if slots[other] == Some(index) && place(other, pools, held, seen, slots) {
                slots[feat] = Some(index);
                return true;
            }
        }
    }
    false
}

/// A maximum assignment of feats to pool capacity (augmenting paths): `None` for a feat left out.
/// Pools are tried in slice order, so earlier pools fill first.
fn assign(pools: &[FeatPool], held: &[HeldFeat]) -> Vec<Option<usize>> {
    let mut slots = vec![None; held.len()];
    for feat in 0..held.len() {
        let mut seen = vec![false; pools.len()];
        place(feat, pools, held, &mut seen, &mut slots);
    }
    slots
}

/// The rule's feat pool, when its `target` hands out picks the character itself spends on feats.
/// `Pool(feat)` itself, a pool whose offer lists feats, or a pool named for feats by a whole word
/// (`mythic_path_feature_*` is not one). A pick on the companion is the companion's.
fn feat_pool_of(rule: &SheetRule) -> Option<&str> {
    let Some(BonusTarget::Pool(pool)) = &rule.target else { return None };
    let offers_feats = matches!(rule.offers.as_ref().map(|o| &o.from), Some(OptionSet::Rules { pool, .. }) if pool == FEAT_POOL);
    (pool == FEAT_POOL || offers_feats || named_for_the_characters_feats(pool)).then_some(pool.as_str())
}

fn named_for_the_characters_feats(pool: &str) -> bool {
    let words = || pool.split('_');
    words().any(|w| w == "feat" || w == "feats") && !words().any(|w| w == "companion")
}

/// The tag sets a held feat answers to: the tags of every converted record of that name.
fn held_feat(package: &SheetRulePackage, feat_id: &str) -> HeldFeat {
    // `Weapon Focus (Longbow)` names the `weapon_focus` record.
    let base = feat_id.split_once('(').map_or(feat_id, |(base, _)| base).trim();
    let tag_sets = package
        .find_all(FEAT_POOL, &id_slug(base))
        .iter()
        .filter_map(|id| package.rule(id))
        .map(|rule| rule.tags.clone())
        .collect();
    HeldFeat { tag_sets }
}

/// The pools `input` holds and what it has spent. `race_traits` are the racial traits the race
/// resolver applied, as [`crate::rules_core::pilot_compute`]'s `with_sheet_rules` takes them.
pub fn feat_budget(
    package: &SheetRulePackage,
    input: &CharacterInput,
    base: &PilotBaseChassisComputation,
    race_traits: &[String],
) -> FeatBudget {
    let mut seed = HeldSeed::from_character(input, base);
    seed.race_traits.extend(race_traits.iter().cloned());
    let facts = CharacterFacts::from_character(input, base).with_linked_picks(package, &seed);
    let held = held_set(package, &seed, &facts);

    let mut pools: BTreeMap<String, FeatPool> = BTreeMap::new();
    let character_level: i64 = input.chosen.class_levels.iter().map(|c| i64::from(c.level)).sum();
    let level_feats = (1..=character_level).filter(|level| u8::try_from(*level).is_ok_and(level_grants_feat)).count() as i64;
    pools.insert(
        LEVEL_POOL.to_owned(),
        FeatPool {
            id: LEVEL_POOL.to_owned(),
            granted: level_feats,
            spent: 0,
            admits: None,
            sources: vec!["a feat at every odd character level".to_owned()],
        },
    );

    for (id, entry) in &held.rules {
        if held.removed.contains(id) {
            continue;
        }
        let Some(rule) = package.rule(id) else { continue };
        let (Some(pool), SheetValue::Number(value)) = (feat_pool_of(rule), &rule.value) else { continue };
        let ctx = EvalContext { holder_class: entry.holder_class.clone(), spell_level: entry.spell_level.unwrap_or(0), item_tags: Vec::new() };
        let gate = if id.contains('#') {
            sibling_line_gate(package, &held, &facts, rule, ctx.clone())
        } else {
            evaluate_applies(&rule.applies, &held, package, &facts, ctx.clone())
        };
        if !gate.includes() {
            continue;
        }
        let amount = evaluate_expr(value, &held, package, &facts, ctx);
        let amount = if amount.den == 0 { 0 } else { amount.num / amount.den };
        let name = if pool != FEAT_POOL {
            pool
        } else if split_rule_id(id).1 == "race_trait" {
            RACIAL_POOL
        } else {
            OPEN_BONUS_POOL
        };
        let row = pools.entry(name.to_owned()).or_insert_with(|| FeatPool {
            id: name.to_owned(),
            granted: 0,
            spent: 0,
            admits: None,
            sources: Vec::new(),
        });
        row.granted += amount;
        if !row.sources.contains(&rule.label) {
            row.sources.push(rule.label.clone());
        }
        if let Some(OptionSet::Rules { pool: offered, tags, .. }) = rule.offers.as_ref().map(|o| &o.from)
            && offered == FEAT_POOL
        {
            row.admits.get_or_insert_with(Vec::new).push(tags.clone());
        }
    }
    pools.retain(|_, pool| pool.granted > 0 || pool.id == LEVEL_POOL);

    let has_fighter_pool = pools.contains_key(FIGHTER_BONUS_FEAT_POOL);
    let mut posture_exempted = false;
    let held_feats: Vec<HeldFeat> = input
        .chosen
        .selected_feats
        .iter()
        .filter(|feat| {
            let exempt = !has_fighter_pool && !posture_exempted && feat.as_str() == POSTURE_FEAT_TOKEN;
            posture_exempted |= exempt;
            !exempt
        })
        .map(|feat| held_feat(package, feat))
        .collect();
    FeatBudget::settled(pools.into_values().collect(), held_feats)
}

/// The character level of the pool slot each entry of `selected_feats` fills, given the character's
/// budget at each of its levels from 1 up (`by_level`). `None` is a feat no assignment fits: as many
/// as [`FeatBudget::over_by`], the latest in saved order.
///
/// A slot opens where its pool's grant rises. Slots are tried earliest first, so saved order fills
/// them in order, and a feat moves to a later slot when that lets a later feat fit. The feat the
/// combat baseline requires takes no slot; it was held from the character's first level.
pub fn feat_slot_levels(selected_feats: &[String], by_level: &[FeatBudget]) -> Vec<Option<u8>> {
    let Some(now) = by_level.last() else { return vec![None; selected_feats.len()] };
    let mut slots: Vec<(usize, FeatPool)> = Vec::new();
    for (index, budget) in by_level.iter().enumerate() {
        for pool in &budget.pools {
            let earlier = index.checked_sub(1).and_then(|previous| by_level[previous].pool(&pool.id)).map_or(0, |p| p.granted);
            slots.extend((earlier..pool.granted).map(|_| (index, FeatPool { granted: 1, spent: 0, ..pool.clone() })));
        }
    }
    slots.sort_by_key(|(index, pool)| (*index, fill_rank(pool)));
    let pools: Vec<FeatPool> = slots.iter().map(|(_, pool)| pool.clone()).collect();
    let mut filled: Vec<Option<usize>> = vec![None; now.held.len()];
    for feat in 0..now.held.len() {
        // The earliest free slot, so an earlier feat keeps its place until a later one cannot fit without moving it.
        let free = (0..pools.len()).find(|slot| !filled.contains(&Some(*slot)) && pools[*slot].admits(&now.held[feat]));
        if free.is_some() {
            filled[feat] = free;
        } else {
            place(feat, &pools, &now.held, &mut vec![false; pools.len()], &mut filled);
        }
    }
    let mut charged = filled.into_iter().map(|slot| slot.and_then(|slot| u8::try_from(slots[slot].0 + 1).ok()));

    let exempt = if now.pool(FIGHTER_BONUS_FEAT_POOL).is_none() {
        selected_feats.iter().position(|feat| feat == POSTURE_FEAT_TOKEN)
    } else {
        None
    };
    (0..selected_feats.len()).map(|index| if exempt == Some(index) { Some(1) } else { charged.next().flatten() }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::class_census::load_sweep_fixture;
    use crate::rules_core::class_seeds::input_for;
    use crate::rules_core::corpus_loader::load_sheet_rules;
    use crate::rules_core::pilot_compute::compute_pilot_base_chassis;
    use std::sync::OnceLock;

    fn package() -> &'static SheetRulePackage {
        static PACKAGE: OnceLock<SheetRulePackage> = OnceLock::new();
        PACKAGE.get_or_init(|| load_sheet_rules(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data/sheet_rules")).package)
    }

    fn budget_of(class: &str, level: u8, race_traits: &[&str], feats: &[&str]) -> FeatBudget {
        let fixture = load_sweep_fixture().expect("census fixture");
        let mut input = input_for(&fixture, class, level);
        input.chosen.selected_feats = feats.iter().map(|f| (*f).to_owned()).collect();
        let base = compute_pilot_base_chassis(&input);
        let traits: Vec<String> = race_traits.iter().map(|t| (*t).to_owned()).collect();
        feat_budget(package(), &input, &base, &traits)
    }

    fn numbers(budget: &FeatBudget, id: &str) -> (i64, i64, i64) {
        let pool = budget.pool(id).unwrap_or_else(|| panic!("no pool {id} in {:?}", budget.pools()));
        (pool.granted, pool.spent, pool.remaining())
    }

    const HUMAN: &[&str] = &["Human ~ Bonus Feat"];

    fn slot_levels(class: &str, level: u8, race_traits: &[&str], feats: &[&str]) -> Vec<Option<u8>> {
        let saved: Vec<String> = feats.iter().map(|feat| (*feat).to_owned()).collect();
        let by_level: Vec<FeatBudget> = (1..=level).map(|at| budget_of(class, at, race_traits, feats)).collect();
        let levels = feat_slot_levels(&saved, &by_level);
        let unplaced = levels.iter().filter(|level| level.is_none()).count() as i64;
        assert_eq!(unplaced, by_level.last().unwrap().over_by(), "{levels:?}");
        levels
    }

    /// CRB p.55, p.112: a level slot and a Fighter slot at 1, a Fighter slot at 2. First-come would give
    /// Weapon Focus the open slot and leave Toughness, not a combat feat, nowhere to go. Which combat
    /// feat moves up is the assignment's choice; that all three fit is the rule.
    #[test]
    fn ledger_migration_a_feat_moves_to_a_later_slot_when_that_lets_a_later_feat_fit() {
        let feats = ["feat:power_attack", "feat:weapon_focus", "feat:toughness"];
        assert_eq!(slot_levels("fighter", 2, &[], &feats), [Some(2), Some(1), Some(1)]);
    }

    #[test]
    fn ledger_migration_slots_open_at_their_grant_levels_and_the_rest_have_no_level() {
        // Human Fighter 3: racial, level and Fighter at 1, Fighter at 2, level at 3.
        let feats = ["feat:power_attack", "feat:cleave", "feat:dodge", "feat:mobility", "feat:weapon_focus", "feat:toughness"];
        assert_eq!(slot_levels("fighter", 3, HUMAN, &feats), [Some(1), Some(1), Some(1), Some(2), Some(3), None]);
        // A Rogue holds Weapon Focus by the baseline, not on a slot, so the next feat still has level 1's.
        let saved = ["feat:weapon_focus", "feat:toughness", "feat:endurance"];
        assert_eq!(slot_levels("rogue", 3, &[], &saved), [Some(1), Some(1), Some(3)]);
        assert_eq!(feat_slot_levels(&["feat:toughness".to_owned()], &[]), [None], "a character with no level has no slot");
    }

    /// CRB p.55 (Fighter), p.21 (Human), p.112 (a feat at every odd level).
    #[test]
    fn feat_budget_of_a_human_fighter_3_is_two_level_one_racial_two_fighter() {
        let budget = budget_of("fighter", 3, HUMAN, &["feat:power_attack", "feat:dodge", "feat:weapon_focus"]);
        assert_eq!(budget.pool("level").unwrap().granted, 2);
        assert_eq!(budget.pool("racial").unwrap().granted, 1);
        assert_eq!(budget.pool("fighter_bonus_feat").unwrap().granted, 2);
        assert_eq!(budget.over_by(), 0);
        let spent: i64 = budget.pools().iter().map(|p| p.spent).sum();
        assert_eq!(spent, 3, "every held feat is charged once");
        assert_eq!(numbers(&budget, "fighter_bonus_feat"), (2, 2, 0), "three combat feats fill the restricted pool first");
    }

    #[test]
    fn feat_budget_reads_each_class_pool_from_the_rules_the_class_holds() {
        for (class, level, pool, want) in [
            ("fighter", 1, "fighter_bonus_feat", 1),
            ("fighter", 20, "fighter_bonus_feat", 11),
            ("wizard", 5, "wizard_bonus_feat", 1),
            ("wizard", 20, "wizard_bonus_feat", 4),
            ("monk", 1, "monk_bonus_feat", 1),
            ("monk", 20, "monk_bonus_feat", 6),
            ("brawler", 20, "brawler_bonus_feat", 7),
            ("magus", 20, "magus_bonus_feat", 3),
            ("cavalier", 20, "cavalier_feat", 3),
        ] {
            let budget = budget_of(class, level, &[], &[]);
            assert_eq!(budget.pool(pool).map(|p| p.granted), Some(want), "{class} {level}: {:?}", budget.pools());
        }
        assert!(budget_of("wizard", 4, &[], &[]).pool("wizard_bonus_feat").is_none(), "no pool before the first grant");
        assert_eq!(budget_of("rogue", 20, &[], &[]).pools().len(), 1, "a Rogue has only the level pool");
    }

    #[test]
    fn feat_budget_level_pool_is_a_feat_at_every_odd_character_level() {
        assert_eq!(budget_of("rogue", 1, &[], &[]).pool("level").unwrap().granted, 1);
        assert_eq!(budget_of("rogue", 2, &[], &[]).pool("level").unwrap().granted, 1);
        assert_eq!(budget_of("rogue", 3, &[], &[]).pool("level").unwrap().granted, 2);
        assert_eq!(budget_of("rogue", 20, &[], &[]).pool("level").unwrap().granted, 10);
        assert_eq!(levels_granting_a_feat(5), vec![1, 3, 5]);
    }

    #[test]
    fn feat_budget_a_restricted_pool_only_takes_what_its_offer_lists() {
        // Fighter 1 with no racial trait: one level feat, one combat feat. Toughness is not a combat
        // feat, so two Toughness-shaped picks do not fit even though two slots exist.
        let two_general = budget_of("fighter", 1, &[], &["feat:toughness", "feat:alertness"]);
        assert_eq!(two_general.over_by(), 1);
        let one_of_each = budget_of("fighter", 1, &[], &["feat:toughness", "feat:power_attack"]);
        assert_eq!(one_of_each.over_by(), 0);
        assert_eq!(numbers(&one_of_each, "fighter_bonus_feat"), (1, 1, 0));
        assert_eq!(numbers(&one_of_each, "level"), (1, 1, 0));
    }

    /// The order of `selected_feats` must not change the verdict: a combat feat taken first moves
    /// to the level pool's place when a later feat can only use the other one.
    #[test]
    fn feat_budget_assignment_moves_an_earlier_feat_when_a_later_one_needs_its_pool() {
        for feats in [["feat:power_attack", "feat:toughness"], ["feat:toughness", "feat:power_attack"]] {
            assert_eq!(budget_of("fighter", 1, &[], &feats).over_by(), 0, "{feats:?}");
        }
        // Two restricted pools: the first feat fits either, the second only the first pool.
        let pool = |id: &str, tags: Option<&[&str]>| FeatPool {
            id: id.to_owned(),
            granted: 1,
            spent: 0,
            admits: tags.map(|t| vec![t.iter().map(|s| (*s).to_owned()).collect()]),
            sources: vec![id.to_owned()],
        };
        let feat = |tags: &[&str]| HeldFeat { tag_sets: vec![tags.iter().map(|s| (*s).to_owned()).collect()] };
        let settled = FeatBudget::settled(
            vec![pool("x", Some(&["Combat"])), pool("y", Some(&["Teamwork"]))],
            vec![feat(&["Combat", "Teamwork"]), feat(&["Combat"])],
        );
        assert_eq!(settled.over_by(), 0, "the first feat moves to y so the second can have x");
    }

    #[test]
    fn feat_budget_over_budget_is_a_negative_remaining_and_the_rows_say_so() {
        let budget = budget_of("rogue", 1, &[], &["feat:toughness", "feat:alertness", "feat:dodge"]);
        assert_eq!(budget.over_by(), 2);
        assert_eq!(numbers(&budget, "level"), (1, 3, -2));
        let rows = budget.explanation_rows();
        let row = |id: &str| rows.iter().find(|r| r.id == id).map(|r| r.value);
        assert_eq!(row("feats.budget.level.granted"), Some(1));
        assert_eq!(row("feats.budget.level.spent"), Some(3));
        assert_eq!(row("feats.budget.level.remaining"), Some(-2));
    }

    #[test]
    fn feat_budget_refusal_names_the_pool_and_a_room_left_answer_is_none() {
        let budget = budget_of("rogue", 3, &[], &["feat:toughness"]);
        assert_eq!(budget.refusal(package(), "feat:alertness"), None, "one level feat is left");

        let full = budget_of("rogue", 1, &[], &["feat:toughness"]);
        let refused = full.refusal(package(), "feat:alertness").expect("no room");
        assert!(refused.contains("level pool") && refused.contains("0 of 1 left"), "{refused}");

        let over = budget_of("rogue", 1, &[], &["feat:toughness", "feat:alertness"]);
        let refused = over.refusal(package(), "feat:dodge").expect("over budget");
        assert!(refused.contains("over its feat budget by 1") && refused.contains("level pool has 2 of 1 spent"), "{refused}");
    }

    /// Every class holds Weapon Focus for the combat baseline; only a Fighter spends a pick on it.
    #[test]
    fn feat_budget_does_not_charge_a_non_fighter_for_the_baseline_weapon_focus_token() {
        let wizard = budget_of("wizard", 1, &[], &["feat:power_attack", "feat:weapon_focus"]);
        assert_eq!(wizard.over_by(), 0);
        assert_eq!(numbers(&wizard, "level"), (1, 1, 0));

        let picked_too = budget_of("wizard", 1, &[], &["feat:weapon_focus", "Weapon Focus"]);
        assert_eq!(numbers(&picked_too, "level"), (1, 1, 0), "a second Weapon Focus was the player's");

        let fighter = budget_of("fighter", 1, &[], &["feat:weapon_focus"]);
        assert_eq!(numbers(&fighter, "fighter_bonus_feat"), (1, 1, 0));
    }

    #[test]
    fn feat_budget_recognises_both_id_shapes_and_a_target_in_parentheses() {
        for feat in ["feat:power_attack", "Power Attack"] {
            assert_eq!(numbers(&budget_of("fighter", 1, &[], &[feat]), "fighter_bonus_feat"), (1, 1, 0), "{feat}");
        }
        assert_eq!(numbers(&budget_of("fighter", 1, &[], &["Weapon Focus (Longbow)"]), "fighter_bonus_feat"), (1, 1, 0));
    }

    #[test]
    fn feat_budget_only_counts_pools_named_for_the_characters_feats() {
        assert!(!named_for_the_characters_feats("mythic_path_feature_archmage_arcana"), "`feature` is not the word `feat`");
        assert!(named_for_the_characters_feats("wizard_bonus_feat"));
        assert!(named_for_the_characters_feats("hunter_teamwork_feats"));
        assert!(!named_for_the_characters_feats("hunter_precise_companion_feat"), "a companion's pick is not the character's");
    }
}
