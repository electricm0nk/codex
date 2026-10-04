//! The Starfinder 1e printed sheet lines (SD-37 E5.1): race, theme, class-feature and every
//! other held record's line, as `sheet_rule::render_sheet` prints it over the converted package.
//!
//! The held set is the one the E4 readers total from (`sf_defense::held`: the race, the first
//! class's level-1 BaseClass template, the theme, the worn armour and the picks, with the class
//! skills its gates read), so a line prints for exactly the records whose numbers the `sf.*`
//! totals add. Two print rules are Starfinder's own:
//!
//! - **Per-weapon rows print on the weapon, not as features.** A `CATEGORY:Weapon` ability
//!   (package `pool` `weapon`, e.g. `X-gen gun (tactical) attack`) is one weapon's attack and
//!   damage term, granted to every proficient character for every weapon of the group. Its
//!   number belongs on that weapon's line (carried weapons: E5.3; attack totals: E7.1), so it is
//!   not a sheet line of its own (the E5.1 receipt counts how many a seed holds).
//! - **No hit die.** A Starfinder class has no hit die (SRD: Hit Points = racial HP + class HP
//!   per level). The class record's `Hit die: d1` row is PCGen's `HD:1` device, which E4.1 reads
//!   as the per-level term of the HP total; it is not printed (the Pathfinder precedent:
//!   `hit_die_source::print_hit_die_lines` prints no `Hit die:` line for a class with no die).

use codex::rules_core::pilot_compute::sf_defense::{SfBuild, SfHeld};
use codex::rules_core::sheet_rule::{
    render_sheet, split_rule_id, Applies, Cmp, Expr, HeldSeed, RuleId, SheetLine, SheetRulePackage,
};

/// The package `pool` of a `CATEGORY:Weapon` ability (a per-weapon attack/damage row).
const WEAPON_POOL: &str = "weapon";

/// The stat-block row of PCGen's `HD:1` device on a Starfinder class line.
const HIT_DIE_ROW: &str = "Hit die: ";

/// The BaseClass template gated on `class_slug` level 1 -- the record the first class level
/// holds (`sf_defense::held` seeds the same one; a package with none or several is refused
/// there before any line prints).
fn first_class_template(package: &SheetRulePackage, class_slug: &str) -> Option<RuleId> {
    let found: Vec<&RuleId> = package
        .rules_of_kind("template")
        .filter(|r| !r.id.contains('#') && r.tags.iter().any(|t| t == "BaseClass"))
        .filter(|r| {
            matches!(&r.applies, Applies::Compare { lhs: Expr::ClassLevel(c), op: Cmp::Gte, rhs: Expr::Const(1) } if c == class_slug)
        })
        .map(|r| &r.id)
        .collect();
    match found.as_slice() {
        [one] => Some((*one).clone()),
        _ => None,
    }
}

/// Every printed sheet line of `build`, over `held` (`sf_defense::held(package, build)`).
pub fn sheet_lines(package: &SheetRulePackage, build: &SfBuild, held: &SfHeld) -> Vec<SheetLine> {
    let chassis = &build.chassis;
    let classes: Vec<(String, i64)> =
        chassis.classes.iter().map(|(id, l)| (split_rule_id(id).2.to_string(), i64::from(*l))).collect();
    let mut rule_ids: Vec<RuleId> = vec![chassis.race.clone()];
    rule_ids.extend(classes.first().and_then(|(first, _)| first_class_template(package, first)));
    rule_ids.extend(build.theme.iter().cloned());
    rule_ids.extend(build.armor.iter().cloned());
    rule_ids.extend(build.picks.iter().cloned());
    let seed = HeldSeed { classes, rule_ids, ..HeldSeed::default() };
    let mut lines = render_sheet(package, &seed, &held.facts);
    lines.retain(|l| package.rule(&l.id).is_none_or(|r| r.pool != WEAPON_POOL));
    for line in lines.iter_mut().filter(|l| l.kind == "class" && !l.id.contains('#')) {
        line.prose = line.prose.split('\n').filter(|row| !row.starts_with(HIT_DIE_ROW)).collect::<Vec<_>>().join("\n");
    }
    lines
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};
    use std::path::PathBuf;

    use crate::rule_system_adapter::RuleSystemAdapter;
    use crate::sf_adapter::StarfinderAdapter;

    /// The features the converted records grant each seed at its level, derived by
    /// `E5.1_seed_features.py` (an independent walk over the package JSON).
    const FEATURE_FIXTURE: &str = "docs/release/SD-37-starfinder-1e/artifacts/epic_5/E5.1-seed-features.md";

    /// The record kinds a feature line has (`E5.1_seed_features.py`'s population).
    const FEATURE_KINDS: [&str; 6] = ["race", "class", "ability", "feat", "pool_option", "template"];

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
    }

    /// seed id -> (rule id -> label), from the fixture's `| SF-… |` rows.
    fn fixture() -> BTreeMap<String, BTreeMap<String, String>> {
        let text = std::fs::read_to_string(repo_root().join(FEATURE_FIXTURE)).expect("the E5.1 fixture reads");
        let mut out: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
        for line in text.lines().filter(|l| l.starts_with("| SF-")) {
            let cells: Vec<&str> = line.trim_matches('|').split(" | ").map(str::trim).collect();
            assert_eq!(cells.len(), 4, "fixture row {line:?}");
            out.entry(cells[0].to_owned())
                .or_default()
                .insert(cells[1].trim_matches('`').to_owned(), cells[2].to_owned());
        }
        out
    }

    fn root_id(id: &str) -> &str {
        id.split('#').next().unwrap_or(id)
    }

    /// The first raw PCGen token or key in `text`: the residue gate's token shapes
    /// (`BONUS:`, `PRE<X>:`, `%LIST`, ...), a `Display ~` record key, a product-identity
    /// stand-in name, the `HD:1` device printed as a hit die, or an unresolved race speed.
    fn residue(text: &str) -> Option<&'static str> {
        const LITERALS: [&str; 13] = [
            "BONUS:", "DEFINE:", "SAB:", "DESC:", "%CHOICE", "%LIST", "TYPE=", "var(", "Display ~",
            "Codex-Named Unit", "Hit die: d1", "Walk 0 ft.", "MODIFY",
        ];
        if let Some(hit) = LITERALS.iter().find(|l| text.contains(**l)) {
            return Some(hit);
        }
        // `PRE<UPPER>+:` / `!PRE<UPPER>+:`
        text.match_indices("PRE")
            .any(|(i, _)| {
                let rest = &text[i + 3..];
                let caps = rest.bytes().take_while(u8::is_ascii_uppercase).count();
                caps > 0 && rest.as_bytes().get(caps) == Some(&b':')
            })
            .then_some("PRE<X>:")
    }

    /// E5.1's criterion: for each seed, the sheet's printed feature list (every printed line of a
    /// feature kind, by its record) = the set of features the converted records grant at that
    /// level, and each feature's own line prints the record's label.
    #[test]
    fn sf_seed_printed_features_equal_the_features_the_converted_records_grant() {
        let fixture = fixture();
        let seeds = crate::sf_adapter::tests::seeds();
        assert_eq!(fixture.len(), seeds.len(), "one fixture block per seed: {:?}", fixture.keys().collect::<Vec<_>>());
        let mut compared = 0;
        for (seed, _, input) in seeds {
            let want = fixture.get(seed).unwrap_or_else(|| panic!("{seed}: no fixture rows"));
            let chassis = StarfinderAdapter.chassis_resolve(&input);
            let printed: BTreeSet<&str> = chassis
                .sheet_lines
                .iter()
                .filter(|l| FEATURE_KINDS.contains(&l.kind.as_str()))
                .map(|l| root_id(&l.id))
                .collect();
            let wanted: BTreeSet<&str> = want.keys().map(String::as_str).collect();
            let missing: Vec<_> = wanted.difference(&printed).collect();
            let extra: Vec<_> = printed.difference(&wanted).take(20).collect();
            assert!(
                missing.is_empty() && extra.is_empty(),
                "{seed}: printed {} feature records, granted {}; not printed {missing:?}; printed but not granted (first 20) {extra:?}",
                printed.len(),
                wanted.len()
            );
            for (id, label) in want {
                if let Some(line) = chassis.sheet_lines.iter().find(|l| l.id == *id) {
                    assert_eq!(&line.label, label, "{seed}: {id} label");
                }
            }
            compared += wanted.len();
            // The printed sheet of each seed, for the receipt's seed deltas (set the variable to
            // a directory outside the repo).
            if let Some(dir) = std::env::var_os("SF_SEED_LINES_OUT") {
                let text: String = chassis
                    .sheet_lines
                    .iter()
                    .map(|l| format!("{} | {} | {} | {} | {}\n", l.kind, l.id, l.label, l.printed, l.prose.replace('\n', " / ")))
                    .collect();
                std::fs::write(PathBuf::from(dir).join(format!("{seed}.txt")), text).expect("writes the seed's lines");
            }
        }
        assert_eq!(compared, fixture.values().map(BTreeMap::len).sum::<usize>());
    }

    /// Every resolvable term on the four seeds' printed lines is resolved, and no line carries a
    /// raw PCGen token or key: the race speed prints the race's speed (SRD: 30 ft. for the
    /// human, lashunta, android and ysoki), and no label names a `Display ~` record, a
    /// product-identity stand-in, or the PCGen `HD:1` device as a hit die.
    #[test]
    fn sf_seed_lines_print_every_resolved_term_and_no_pcgen_residue() {
        for (seed, _, input) in crate::sf_adapter::tests::seeds() {
            let chassis = StarfinderAdapter.chassis_resolve(&input);
            assert!(!chassis.sheet_lines.is_empty(), "{seed}: no printed lines");
            let race: Vec<_> = chassis.sheet_lines.iter().filter(|l| l.kind == "race" && !l.id.contains('#')).collect();
            assert_eq!(race.len(), 1, "{seed}: one race line");
            assert!(race[0].prose.contains("Speed: Walk 30 ft."), "{seed}: race prose {:?}", race[0].prose);
            for line in &chassis.sheet_lines {
                for text in [&line.label, &line.printed, &line.prose] {
                    assert_eq!(residue(text), None, "{seed}: {} prints {text:?}", line.id);
                }
            }
        }
    }
}
