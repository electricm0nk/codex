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
//!
//! Feats print as held picks, like every other record (E5.2). **Spells** (E5.2) print one line
//! per spell the character knows (`CharacterInput.spells_selected`), each evaluated for its
//! caster: the source class is the holder, so a formula over the caster level prints its number
//! (*Magic Missile*'s range, 100 ft. + 10 ft. per level, prints 150 ft. at technomancer 5), and
//! the line states the spell's level on that class's list. With no character (a catalog,
//! `sheet_rule_catalog::catalog_prose`) the same formula prints as words. A selection whose
//! spell the package does not hold, whose class the character does not have, or whose class's
//! list does not carry it is refused by name, never dropped.

use codex::rules_core::character_input::{ActiveState, EquipmentSelection, SpellSelection};
use codex::rules_core::pilot_compute::sf_loadout::REFUSED_NOT_HELD;
use codex::rules_core::pilot_compute::sf_chassis::SfChassisRefusal;
use codex::rules_core::pilot_compute::sf_defense::{SfBuild, SfHeld};
use codex::rules_core::sheet_rule::{
    evaluate, render_sheet, split_rule_id, Applies, Cmp, EvalContext, Expr, Granter, HeldSeed, ProseFamily, ProsePiece, RuleId,
    SheetLine, SheetLineValue, SheetRule, SheetRulePackage,
};

pub const REFUSED_UNKNOWN_SPELL: &str = "sf_sheet_print.unknown_spell";
pub const REFUSED_SPELL_CLASS_NOT_HELD: &str = "sf_sheet_print.spell_class_not_held";
pub const REFUSED_SPELL_NOT_ON_CLASS_LIST: &str = "sf_sheet_print.spell_not_on_class_list";
pub const REFUSED_UNKNOWN_MODIFIER: &str = "sf_sheet_print.unknown_modifier";
pub const REFUSED_MODIFIER_DOES_NOT_FIT: &str = "sf_sheet_print.modifier_does_not_fit";
pub const REFUSED_UPGRADE_SLOTS: &str = "sf_sheet_print.upgrade_slots_exceeded";
pub const REFUSED_AUGMENTATION_SYSTEM: &str = "sf_sheet_print.augmentation_system_taken";

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

/// The level `rule` (a spell) has on `class`'s spell list (its `ClassSpellList` grant).
fn spell_level_on(rule: &SheetRule, class: &str) -> Option<u8> {
    rule.granted_by.iter().find_map(|g| match &g.by {
        Granter::ClassSpellList { id, spell_level } if id == class => Some(*spell_level),
        _ => None,
    })
}

/// One printed line per known spell, evaluated with its source class as the holder.
fn spell_lines(
    package: &SheetRulePackage,
    build: &SfBuild,
    held: &SfHeld,
    spells: &[SpellSelection],
) -> Result<Vec<SheetLine>, SfChassisRefusal> {
    let refuse = |id: &'static str, message: String| SfChassisRefusal { id, message };
    let mut lines = Vec::with_capacity(spells.len());
    for selection in spells {
        let rule = package.rule(&selection.spell_id).filter(|r| r.id.split(':').nth(1) == Some("spell")).ok_or_else(|| {
            refuse(REFUSED_UNKNOWN_SPELL, format!("{}: no Starfinder spell record", selection.spell_id))
        })?;
        if !build.chassis.classes.iter().any(|(class, level)| *class == selection.source_class_id && *level >= 1) {
            return Err(refuse(
                REFUSED_SPELL_CLASS_NOT_HELD,
                format!("{}: known through {}, a class the character does not have", rule.id, selection.source_class_id),
            ));
        }
        let class = split_rule_id(&selection.source_class_id).2.to_string();
        let level = spell_level_on(rule, &class).ok_or_else(|| {
            refuse(REFUSED_SPELL_NOT_ON_CLASS_LIST, format!("{}: not on the {class} spell list", rule.id))
        })?;
        let ctx = EvalContext { holder_class: Some(class.clone()), spell_level: i64::from(level), item_tags: Vec::new() };
        let mut line = evaluate(rule, &held.held, package, &held.facts, ctx);
        let class_label = package.rule(&selection.source_class_id).map_or(class.as_str(), |r| r.label.as_str());
        line.also.push((format!("{class_label} spell level {level}"), SheetLineValue::Resolved(i32::from(level))));
        lines.push((level, line));
    }
    lines.sort_by(|(la, a), (lb, b)| la.cmp(lb).then(a.label.cmp(&b.label)).then(a.id.cmp(&b.id)));
    Ok(lines.into_iter().map(|(_, line)| line).collect())
}

/// The tags that make an equipment record an augmentation (the oracle's `TYPE:` heads).
const AUGMENTATION_KINDS: [&str; 5] = ["Cybernetic", "Bio-Tech", "Magitech", "Necrograft", "Personal Upgrade"];

/// An armour record's tag (`TYPE:Armor` items), an armour upgrade's and a weapon fusion's
/// (`equipment_modifier` records, oracle `TYPE:Armor` / `TYPE:Weapon` in `*_equipmods.lst`).
const ARMOR_TAG: &str = "ARMOR";
const UPGRADE_TAG: &str = "Armor";
const FUSION_TAG: &str = "Weapon";

/// Stat-block rows: an armour's upgrade slots, the slots an upgrade uses (oracle
/// `MODIFY:UpgradeSlotTaken|ADD|<n>`), a weapon's damage dice.
const UPGRADE_SLOTS_ROW: &str = "Upgrade slots";
const SLOTS_USED_ROW: &str = "Upgrade slots used";
const DAMAGE_ROW: &str = "Damage";

fn refuse(id: &'static str, message: String) -> SfChassisRefusal {
    SfChassisRefusal { id, message }
}

/// The text of `rule`'s first `StatBlock "<label>"` row.
fn stat_row(rule: &SheetRule, label: &str) -> Option<String> {
    rule.prose.iter().find_map(|seg| match &seg.family {
        ProseFamily::StatBlock(l) if l == label => Some(
            seg.pieces.iter().filter_map(|p| if let ProsePiece::Text(t) = p { Some(t.as_str()) } else { None }).collect::<String>(),
        ),
        _ => None,
    })
}

fn has_tag(rule: &SheetRule, tag: &str) -> bool {
    rule.tags.iter().any(|t| t == tag)
}

/// The augmentation kind of an equipment record and its body systems: every tag that is not
/// the kind, an `ItemLevel_<n>` / `Cybernetic_<system>` index tag, a bare number or a
/// `Personal Upgrade LVL <n>` tag.
fn augmentation(rule: &SheetRule) -> Option<(&'static str, Vec<&str>)> {
    let kind = AUGMENTATION_KINDS.iter().find(|k| has_tag(rule, k))?;
    let systems = rule
        .tags
        .iter()
        .map(String::as_str)
        .filter(|t| {
            !AUGMENTATION_KINDS.contains(t)
                && !t.starts_with("ItemLevel_")
                && !t.starts_with("Cybernetic_")
                && !t.starts_with("Personal Upgrade")
                && !t.chars().all(|c| c.is_ascii_digit())
        })
        .collect();
    Some((kind, systems))
}

/// One carried item: its record, how many are carried, whether one is worn or installed, and
/// the modifiers applied to each of its selections.
struct Carried<'a> {
    id: &'a str,
    quantity: u32,
    installed: bool,
    modifiers: Vec<&'a [String]>,
}

/// Every carried item's line (`equipment` lines; the worn armour's and an equipped item's
/// already print as held records and gain their quantity here), and every applied upgrade's
/// and fusion's line. A weapon's line prints its damage dice; its per-weapon attack and damage
/// rows stay off the sheet (E5.1): their numbers are attack totals (E7.1), and they read +0
/// until the weapon focus/specialization ability that sets them is held.
fn equipment_lines(
    package: &SheetRulePackage,
    held: &SfHeld,
    equipment: &[EquipmentSelection],
    lines: &mut Vec<SheetLine>,
) -> Result<(), SfChassisRefusal> {
    let mut carried: Vec<Carried> = Vec::new();
    for selection in equipment.iter().filter(|s| s.active_state != ActiveState::Absent) {
        let installed = selection.active_state == ActiveState::EquippedActive;
        match carried.iter_mut().find(|c| c.id == selection.item_id) {
            Some(c) => {
                c.quantity += 1;
                c.installed |= installed;
                c.modifiers.push(&selection.applied_modifiers);
            }
            None => carried.push(Carried {
                id: &selection.item_id,
                quantity: 1,
                installed,
                modifiers: vec![&selection.applied_modifiers],
            }),
        }
    }
    let mut systems_taken: Vec<(String, String)> = Vec::new();
    let mut modifier_lines = Vec::new();
    for item in &carried {
        let rule = package
            .rule(item.id)
            .filter(|r| split_rule_id(&r.id).1 == "equipment" && !r.id.contains('#'))
            .ok_or_else(|| refuse(REFUSED_NOT_HELD, format!("{}: no such equipment record in the Starfinder package", item.id)))?;
        let at = match lines.iter().position(|l| l.id == rule.id) {
            Some(at) => at,
            None => {
                let ctx = EvalContext { holder_class: None, spell_level: 0, item_tags: rule.tags.clone() };
                lines.push(evaluate(rule, &held.held, package, &held.facts, ctx));
                lines.len() - 1
            }
        };
        let mut also = vec![(format!("quantity {}", item.quantity), SheetLineValue::Resolved(item.quantity as i32))];
        if let Some((kind, systems)) = augmentation(rule) {
            also.push((format!("{kind} augmentation, system: {}", systems.join(", ")), SheetLineValue::Words));
            if item.installed {
                for system in systems {
                    if let Some((other, _)) = systems_taken.iter().find(|(_, s)| s.eq_ignore_ascii_case(system)) {
                        return Err(refuse(
                            REFUSED_AUGMENTATION_SYSTEM,
                            format!("{} and {other} are both installed in the {system} system (one augmentation per system)", rule.id),
                        ));
                    }
                    systems_taken.push((rule.id.clone(), system.to_owned()));
                }
            }
        }
        let slots = stat_row(rule, UPGRADE_SLOTS_ROW);
        let mut slots_used = 0i64;
        for modifiers in &item.modifiers {
            let mut used_here = 0i64;
            for id in modifiers.iter() {
                let modifier = package
                    .rule(id)
                    .filter(|r| split_rule_id(&r.id).1 == "equipment_modifier" && !r.id.contains('#'))
                    .ok_or_else(|| refuse(REFUSED_UNKNOWN_MODIFIER, format!("{id}: no such equipment modifier in the Starfinder package")))?;
                let place = if has_tag(modifier, UPGRADE_TAG) {
                    if !has_tag(rule, ARMOR_TAG) {
                        return Err(refuse(REFUSED_MODIFIER_DOES_NOT_FIT, format!("{id}: an armour modifier applied to {}, which is not armour", rule.id)));
                    }
                    match stat_row(modifier, SLOTS_USED_ROW) {
                        Some(used) => {
                            used_here += used.trim().trim_start_matches('+').parse::<i64>().map_err(|_| {
                                refuse(REFUSED_UPGRADE_SLOTS, format!("{id}: upgrade slots used {used:?} is not a number"))
                            })?;
                            "upgrade installed in"
                        }
                        // A special material (adamantine, noqual, ...) is armour-tagged and uses no slot.
                        None => "applied to",
                    }
                } else if has_tag(modifier, FUSION_TAG) {
                    if stat_row(rule, DAMAGE_ROW).is_none() {
                        return Err(refuse(REFUSED_MODIFIER_DOES_NOT_FIT, format!("{id}: a weapon fusion applied to {}, which is not a weapon", rule.id)));
                    }
                    "fusion on"
                } else {
                    "applied to"
                };
                let ctx = EvalContext { holder_class: None, spell_level: 0, item_tags: rule.tags.clone() };
                let mut line = evaluate(modifier, &held.held, package, &held.facts, ctx);
                line.also.push((format!("{place} {}", rule.label), SheetLineValue::Words));
                modifier_lines.push(line);
            }
            if used_here > 0 {
                let available = slots.as_deref().and_then(|s| s.trim().parse::<i64>().ok()).unwrap_or(0);
                if used_here > available {
                    return Err(refuse(
                        REFUSED_UPGRADE_SLOTS,
                        format!("{}: its upgrades use {used_here} upgrade slots; it has {available}", rule.id),
                    ));
                }
            }
            slots_used = slots_used.max(used_here);
        }
        if let Some(slots) = slots {
            also.push((format!("upgrade slots used {slots_used} of {}", slots.trim()), SheetLineValue::Resolved(slots_used as i32)));
        }
        lines[at].also.extend(also);
    }
    lines.extend(modifier_lines);
    Ok(())
}

/// Every printed sheet line of `build`, over `held` (`sf_defense::held(package, build)`), one
/// line per spell in `spells` (the character's spells known), and one line per carried item and
/// per upgrade or fusion applied to one (`equipment`, the save's selections; E5.3).
pub fn sheet_lines(
    package: &SheetRulePackage,
    build: &SfBuild,
    held: &SfHeld,
    spells: &[SpellSelection],
    equipment: &[EquipmentSelection],
) -> Result<Vec<SheetLine>, SfChassisRefusal> {
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
    lines.extend(spell_lines(package, build, held, spells)?);
    equipment_lines(package, held, equipment, &mut lines)?;
    Ok(lines)
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
        const LITERALS: [&str; 16] = [
            "BONUS:", "DEFINE:", "SAB:", "DESC:", "%CHOICE", "%LIST", "TYPE=", "var(", "Display ~",
            "Codex-Named Unit", "Hit die: d1", "Walk 0 ft.", "MODIFY",
            // UTF-8 read as Windows-1252 (`â€œ`, `â€"`, `Â `, `Ã—`): the oracle's own bytes.
            "\u{e2}\u{20ac}", "\u{c2}", "\u{c3}",
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
                    .map(|l| {
                        let also: Vec<&str> = l.also.iter().map(|(t, _)| t.as_str()).collect();
                        format!("{} | {} | {} | {} | {} | {}\n", l.kind, l.id, l.label, l.printed, l.prose.replace('\n', " / "), also.join("; "))
                    })
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

    /// The seed builds (E0.4): every seed's feats and spells known, read from its `**Feats**` and
    /// `**Spells known**` tables. Feats: the first cell, its parenthetical dropped, of every row
    /// whose "When" cell is not a class feature. Spells: `(spell level, name)` for every name in
    /// the spells and connection-spell cells, parentheticals dropped.
    const SEED_BUILDS: &str = "docs/release/SD-37-starfinder-1e/artifacts/epic_0/seed-builds.md";

    type SeedPicks = BTreeMap<String, (BTreeSet<String>, BTreeSet<(i64, String)>)>;

    fn seed_builds() -> SeedPicks {
        let text = std::fs::read_to_string(repo_root().join(SEED_BUILDS)).expect("seed-builds.md reads");
        let bare = |name: &str| name.split(" (").next().unwrap_or(name).trim().to_lowercase();
        let mut out = SeedPicks::new();
        let (mut seed, mut table) = (None::<String>, "");
        for line in text.lines() {
            if let Some(rest) = line.strip_prefix("## ") {
                seed = rest.split_whitespace().nth(1).filter(|s| s.starts_with("SF-")).map(str::to_owned);
                table = "";
            } else if line.starts_with("**") {
                table = if line.starts_with("**Feats**") {
                    "feats"
                } else if line.starts_with("**Spells known**") {
                    "spells"
                } else {
                    ""
                };
            } else if let (Some(seed), true) = (&seed, line.starts_with("| ") && !line.starts_with("|---")) {
                let cells: Vec<&str> = line.trim_matches('|').split(" | ").map(str::trim).collect();
                let entry = out.entry(seed.clone()).or_default();
                match table {
                    "feats" if cells[0] != "Feat" && !cells[1].contains("class feature") => {
                        entry.0.insert(bare(cells[0]));
                    }
                    "spells" if cells[0] != "Spell level" => {
                        let level: i64 = match cells[0].split_whitespace().next() {
                            Some("0") => 0,
                            Some(ordinal) => ordinal.trim_end_matches(char::is_alphabetic).parse().expect("spell level"),
                            None => panic!("spell row {line:?}"),
                        };
                        for name in cells[1..].iter().flat_map(|c| c.split(", ")).filter(|n| *n != "\u{2014}") {
                            entry.1.insert((level, bare(name)));
                        }
                    }
                    _ => {}
                }
            }
        }
        out
    }

    /// The spell level a printed spell line states (`also`: "<Class> spell level <n>").
    fn printed_spell_level(line: &codex::rules_core::sheet_rule::SheetLine) -> i64 {
        line.also
            .iter()
            .find_map(|(text, _)| text.rsplit_once("spell level ").and_then(|(_, n)| n.parse().ok()))
            .unwrap_or_else(|| panic!("{}: no spell level printed ({:?})", line.id, line.also))
    }

    /// E5.2's criterion, feats: each seed's sheet prints one feat line for every feat the seed
    /// build takes (E0.4), and no other.
    #[test]
    fn sf_seed_feats_print_the_seed_builds_feats() {
        let builds = seed_builds();
        let seeds = crate::sf_adapter::tests::seeds();
        assert_eq!(builds.len(), seeds.len(), "one seed-builds section per seed: {:?}", builds.keys().collect::<Vec<_>>());
        for (seed, _, input) in seeds {
            let want = &builds[seed].0;
            assert!(!want.is_empty(), "{seed}: seed-builds.md lists no feats");
            let chassis = StarfinderAdapter.chassis_resolve(&input);
            let printed: BTreeSet<String> = chassis
                .sheet_lines
                .iter()
                .filter(|l| l.kind == "feat" && !l.id.contains('#'))
                .map(|l| l.label.to_lowercase())
                .collect();
            assert_eq!(&printed, want, "{seed}: printed feats vs seed-builds.md");
        }
    }

    /// E5.2's criterion, spells: each seed's printed spell list = E0.4's spells known, each at
    /// its spell level; per level, the count printed = the `sf.spells.<class>.<n>.known` total.
    #[test]
    fn sf_seed_spell_lists_equal_the_seed_builds_spells_known() {
        let builds = seed_builds();
        let mut compared = 0;
        for (seed, class, input) in crate::sf_adapter::tests::seeds() {
            let want = &builds[seed].1;
            let chassis = StarfinderAdapter.chassis_resolve(&input);
            let spells: Vec<_> = chassis.sheet_lines.iter().filter(|l| l.kind == "spell").collect();
            let printed: BTreeSet<(i64, String)> =
                spells.iter().map(|l| (printed_spell_level(l), l.label.to_lowercase())).collect();
            assert_eq!(printed.len(), spells.len(), "{seed}: a spell printed twice");
            assert_eq!(&printed, want, "{seed}: printed spells vs seed-builds.md");
            for level in printed.iter().map(|(n, _)| *n).collect::<BTreeSet<_>>() {
                let id = format!("sf.spells.{class}.{level}.known");
                let known = chassis.explanations.iter().find(|e| e.id == id).unwrap_or_else(|| panic!("{seed}: no {id}"));
                let count = printed.iter().filter(|(n, _)| *n == level).count();
                assert_eq!(i64::from(known.value), count as i64, "{seed}: {id} vs spells printed at level {level}");
            }
            compared += want.len();
        }
        assert_eq!(compared, 28, "Mystic 15 + Technomancer 13 spells known (seed-builds.md §2, §3)");
    }

    /// A printed spell's prose is evaluated for its caster: a formula over the caster level
    /// prints the number (SRD *Magic Missile*: range "medium (100 ft. + 10 ft./level)" -> 150 ft.
    /// at technomancer 5), the save DC and every other term with nothing left unsubstituted.
    #[test]
    fn sf_seed_spell_prose_resolves_the_casters_terms() {
        let (_, _, input) = crate::sf_adapter::tests::seeds()
            .into_iter()
            .find(|(seed, _, _)| *seed == "SF-Technomancer-5")
            .expect("the technomancer seed");
        let chassis = StarfinderAdapter.chassis_resolve(&input);
        let missile = chassis.sheet_lines.iter().find(|l| l.id == "core:spell:magic_missile").expect("magic missile prints");
        assert!(missile.prose.contains("Range: 150 ft."), "{:?}", missile.prose);
        assert!(!missile.prose.to_lowercase().contains("caster level"), "{:?}", missile.prose);
    }

    /// "Spell prose formulas as words": with no character (a catalog), every Starfinder spell's
    /// prose prints each formula as words, never a raw token, and carries no mis-decoded text.
    #[test]
    fn every_sf_spell_prints_its_formulas_as_words_with_no_character() {
        use codex::rules_core::corpus_loader::live_sheet_rules_for;
        use codex::rules_core::game_system::GameSystem;
        use codex::rules_core::sheet_rule_catalog::catalog_prose;
        let package = live_sheet_rules_for(GameSystem::Starfinder1e).expect("the Starfinder package loads");
        let spells: Vec<_> = package.rules_of_kind("spell").filter(|r| !r.id.contains('#')).collect();
        assert!(spells.len() >= 370, "{} spells", spells.len());
        for rule in &spells {
            let prose = catalog_prose(package, rule);
            assert_eq!(residue(&prose), None, "{} prints {prose:?}", rule.id);
            // No converted term printed as its data shape (`Slot`, `CasterLevel`, `Holder`, ...).
            for debris in ["Slot", "CasterLevel", "\"Holder\"", "Const(", "{\"", "\"Text\""] {
                assert!(!prose.contains(debris), "{} prints {debris:?}: {prose:?}", rule.id);
            }
        }
        let missile = spells.iter().find(|r| r.id == "core:spell:magic_missile").expect("magic missile");
        let prose = catalog_prose(package, missile);
        assert!(prose.contains("caster level"), "{prose:?}");
    }

    /// The seed builds' gear (E0.4): per seed, `(item name, quantity)` for every row of the
    /// `**Armour and weapons**` table (quantity 1) and the `**Other gear**` table (its `Qty`
    /// cell). The name is the first cell, its parenthetical dropped, lowercased.
    fn seed_gear() -> BTreeMap<String, Vec<(String, u32)>> {
        let text = std::fs::read_to_string(repo_root().join(SEED_BUILDS)).expect("seed-builds.md reads");
        let bare = |name: &str| name.split(" (").next().unwrap_or(name).trim().to_lowercase();
        let mut out: BTreeMap<String, Vec<(String, u32)>> = BTreeMap::new();
        let (mut seed, mut table) = (None::<String>, "");
        for line in text.lines() {
            if let Some(rest) = line.strip_prefix("## ") {
                seed = rest.split_whitespace().nth(1).filter(|s| s.starts_with("SF-")).map(str::to_owned);
                table = "";
            } else if line.starts_with("**") {
                table = if line.starts_with("**Armour and weapons") {
                    "worn"
                } else if line.starts_with("**Other gear") {
                    "gear"
                } else {
                    ""
                };
            } else if let (Some(seed), true) = (&seed, line.starts_with("| ") && !line.starts_with("|---")) {
                let cells: Vec<&str> = line.trim_matches('|').split(" | ").map(str::trim).collect();
                match table {
                    "worn" if cells[0] != "Item" => out.entry(seed.clone()).or_default().push((bare(cells[0]), 1)),
                    "gear" if cells[0] != "Item" => {
                        let qty = cells[1].parse().unwrap_or_else(|_| panic!("gear row {line:?}"));
                        out.entry(seed.clone()).or_default().push((bare(cells[0]), qty));
                    }
                    _ => {}
                }
            }
        }
        out
    }

    /// The quantity a printed item line states (`also`: "quantity <n>").
    fn printed_quantity(line: &codex::rules_core::sheet_rule::SheetLine) -> u32 {
        line.also
            .iter()
            .find_map(|(text, _)| text.strip_prefix("quantity ").and_then(|n| n.parse().ok()))
            .unwrap_or_else(|| panic!("{}: no quantity printed ({:?})", line.id, line.also))
    }

    /// The same item name in the seed builds and the package: equal ignoring case and
    /// punctuation, or the package names the standard model by its family alone ("Battery,
    /// Standard" is the package's "Battery").
    fn same_item(seed_name: &str, label: &str) -> bool {
        let norm = |s: &str| s.to_lowercase().chars().filter(|c| c.is_alphanumeric() || *c == ' ').collect::<String>();
        let (seed_name, label) = (norm(seed_name), norm(label));
        seed_name.split_whitespace().eq(label.split_whitespace()) || seed_name == format!("{label} standard")
    }

    /// E5.3's criterion, "seed loadouts render": each seed's sheet prints one equipment line per
    /// item the seed carries -- the worn armour and every weapon and piece of gear in
    /// `seed-builds.md`'s tables -- each with its quantity, and no other equipment line.
    #[test]
    fn sf_seed_loadouts_print_every_carried_item_with_its_quantity() {
        let gear = seed_gear();
        let seeds = crate::sf_adapter::tests::seeds();
        assert_eq!(gear.len(), seeds.len(), "one gear block per seed: {:?}", gear.keys().collect::<Vec<_>>());
        let mut compared = 0;
        for (seed, _, input) in seeds {
            let want = &gear[seed];
            let chassis = StarfinderAdapter.chassis_resolve(&input);
            let items: Vec<_> = chassis.sheet_lines.iter().filter(|l| l.kind == "equipment" && !l.id.contains('#')).collect();
            assert_eq!(items.len(), want.len(), "{seed}: printed {:?}", items.iter().map(|l| &l.label).collect::<Vec<_>>());
            for (name, qty) in want {
                let line = items
                    .iter()
                    .find(|l| same_item(name, &l.label))
                    .unwrap_or_else(|| panic!("{seed}: {name:?} not printed ({:?})", items.iter().map(|l| &l.label).collect::<Vec<_>>()));
                assert_eq!(printed_quantity(line), *qty, "{seed}: {name} quantity");
                assert!(line.prose.contains("Price: "), "{seed}: {name} prints no price: {:?}", line.prose);
                compared += 1;
            }
            // The worn armour's line states its upgrade slots and how many are used.
            let armor = items.iter().find(|l| l.also.iter().any(|(t, _)| t.starts_with("upgrade slots used"))).expect("armour line");
            assert!(armor.also.iter().any(|(t, _)| t == "upgrade slots used 0 of 1"), "{seed}: {:?}", armor.also);
        }
        assert_eq!(compared, 5 + 6 + 5 + 4, "seed-builds.md item rows: Soldier 5, Mystic 6, Technomancer 5, Envoy 4");
    }

    /// A carried weapon's line prints its damage dice (its `Damage` stat row) as its value, and
    /// none of the weapon's per-weapon attack/damage rows (attack totals, E7.1).
    #[test]
    fn sf_seed_weapon_lines_print_their_damage_dice() {
        let mut weapons = 0;
        for (seed, _, input) in crate::sf_adapter::tests::seeds() {
            let chassis = StarfinderAdapter.chassis_resolve(&input);
            for line in chassis.sheet_lines.iter().filter(|l| l.kind == "equipment" && !l.id.contains('#')) {
                let Some(dice) = line.prose.split(" / ").chain(line.prose.split('\n')).find_map(|row| row.strip_prefix("Damage: ")) else {
                    continue;
                };
                assert_eq!(line.printed, dice.trim(), "{seed}: {}", line.id);
                assert!(!line.also.iter().any(|(t, _)| t.contains(" attack ") || t.contains(" damage ")), "{seed}: {:?}", line.also);
                weapons += 1;
            }
        }
        assert_eq!(weapons, 2 + 2 + 1 + 2, "seed weapons: Soldier rifle + baton, Mystic pistol + baton, Technomancer pistol, Envoy pistol + baton");
    }

    fn technomancer() -> codex::rules_core::character_input::CharacterInput {
        crate::sf_adapter::tests::seeds()
            .into_iter()
            .find(|(seed, _, _)| *seed == "SF-Technomancer-5")
            .expect("the technomancer seed")
            .2
    }

    fn selection(input: &mut codex::rules_core::character_input::CharacterInput, item: &str) -> usize {
        input
            .chosen
            .equipment_selections
            .iter()
            .position(|s| s.item_id == format!("core:equipment:{item}"))
            .unwrap_or_else(|| panic!("{item} selected"))
    }

    /// An armour upgrade (an `equipment_modifier` tagged `Armor`) applied to the worn armour
    /// prints on its own line, installed in that armour, and the armour line counts the slots
    /// it uses; a fusion (tagged `Weapon`) applied to a weapon prints fused to it. Neither
    /// moves EAC or KAC.
    #[test]
    fn installed_upgrades_and_fusions_print_on_their_item() {
        let base = StarfinderAdapter.chassis_resolve(&technomancer());
        let mut input = technomancer();
        let armor = selection(&mut input, "d_suit_i");
        input.chosen.equipment_selections[armor].applied_modifiers.push("core:equipment_modifier:armor_infrared_sensors".into());
        let pistol = selection(&mut input, "laser_pistol_azimuth");
        input.chosen.equipment_selections[pistol].applied_modifiers.push("core:equipment_modifier:weapon_flaming".into());
        let chassis = StarfinderAdapter.chassis_resolve(&input);
        assert!(chassis.diagnostics.iter().all(|d| !d.claim_blocking), "{:?}", chassis.diagnostics);
        let line = |id: &str| chassis.sheet_lines.iter().find(|l| l.id == id).unwrap_or_else(|| panic!("{id} prints"));
        let upgrade = line("core:equipment_modifier:armor_infrared_sensors");
        assert!(upgrade.also.iter().any(|(t, _)| t == "upgrade installed in D-suit I"), "{:?}", upgrade.also);
        assert!(upgrade.prose.contains("darkvision"), "{:?}", upgrade.prose);
        // Its price, item level and bulk print as stat rows (oracle `COST:200`,
        // `SPROP:ItemLevel=1`, `SPROP:Bulk=L`), never as `ItemLevel=1` debris.
        for row in ["Price: 200", "Item level: 1", "Quality: Bulk: L"] {
            assert!(upgrade.prose.split('\n').any(|r| r == row), "{row}: {:?}", upgrade.prose);
        }
        assert!(!upgrade.prose.contains("ItemLevel=") && !upgrade.prose.contains("Bulk="), "{:?}", upgrade.prose);
        let suit = line("core:equipment:d_suit_i");
        assert!(suit.also.iter().any(|(t, _)| t == "upgrade slots used 1 of 1"), "{:?}", suit.also);
        let fusion = line("core:equipment_modifier:weapon_flaming");
        assert!(fusion.also.iter().any(|(t, _)| t == "fusion on Laser pistol, azimuth"), "{:?}", fusion.also);
        for id in ["sf.eac", "sf.kac"] {
            let value = |c: &codex::rules_core::pilot_compute::PilotBaseChassisComputation| {
                c.explanations.iter().find(|e| e.id == id).map(|e| e.value)
            };
            assert_eq!(value(&chassis), value(&base), "{id} moved");
        }
    }

    /// An upgrade or fusion the sheet cannot place is refused by name: more upgrade slots used
    /// than the armour has, an armour upgrade on a weapon, a fusion on armour, a record that is
    /// not an equipment modifier.
    #[test]
    fn an_unplaceable_upgrade_or_fusion_is_a_named_refusal() {
        use super::{REFUSED_MODIFIER_DOES_NOT_FIT, REFUSED_UNKNOWN_MODIFIER, REFUSED_UPGRADE_SLOTS};
        for (item, modifiers, refusal) in [
            ("d_suit_i", vec!["armor_infrared_sensors", "armor_jump_jets"], REFUSED_UPGRADE_SLOTS),
            ("laser_pistol_azimuth", vec!["armor_infrared_sensors"], REFUSED_MODIFIER_DOES_NOT_FIT),
            ("d_suit_i", vec!["weapon_flaming"], REFUSED_MODIFIER_DOES_NOT_FIT),
            ("d_suit_i", vec!["no_such_upgrade"], REFUSED_UNKNOWN_MODIFIER),
        ] {
            let mut input = technomancer();
            let at = selection(&mut input, item);
            input.chosen.equipment_selections[at].applied_modifiers =
                modifiers.iter().map(|m| format!("core:equipment_modifier:{m}")).collect();
            let chassis = StarfinderAdapter.chassis_resolve(&input);
            assert_eq!(chassis.diagnostics.len(), 1, "{item} {modifiers:?}: {:?}", chassis.diagnostics);
            assert_eq!(chassis.diagnostics[0].id, refusal, "{item} {modifiers:?}");
            assert!(chassis.diagnostics[0].claim_blocking);
        }
    }

    /// An equipped item's bonus to a sheet total reaches that total (E4's reader totals it): an
    /// equipped *Aeon Stone (Dark Blue Rhomboid)* adds its +2 insight bonus to Perception and
    /// Sense Motive; the same stone carried but not equipped adds nothing, and its line prints
    /// either way.
    #[test]
    fn an_equipped_items_bonus_reaches_the_skill_totals() {
        use codex::rules_core::character_input::{ActiveState, EquipmentSelection};
        // A soldier of 10th level can afford the stone (18,000 credits; Table 11-5).
        let mut input = crate::rule_system_adapter::tests::sf_soldier_3_input();
        input.chosen.class_levels[0].level = 10;
        let stone = |active_state| EquipmentSelection {
            item_id: "core:equipment:aeon_stone_dark_blue_rhomboid".into(),
            equipped_or_active: active_state == ActiveState::EquippedActive,
            active_state,
            applied_modifiers: Vec::new(),
        };
        let total = |input: &codex::rules_core::character_input::CharacterInput, id: &str| {
            let chassis = StarfinderAdapter.chassis_resolve(input);
            assert!(chassis.diagnostics.iter().all(|d| !d.claim_blocking), "{:?}", chassis.diagnostics);
            assert!(chassis.sheet_lines.iter().any(|l| l.id == "core:equipment:aeon_stone_dark_blue_rhomboid"), "the stone prints");
            chassis.explanations.iter().find(|e| e.id == id).map(|e| e.value).unwrap_or_else(|| panic!("no {id}"))
        };
        let mut carried = input.clone();
        carried.chosen.equipment_selections.push(stone(ActiveState::SelectedInactive));
        input.chosen.equipment_selections.push(stone(ActiveState::EquippedActive));
        for skill in ["perception", "sense_motive"] {
            let id = format!("sf.skill.{skill}");
            assert_eq!(total(&input, &id), total(&carried, &id) + 2, "{id}");
        }
    }

    /// An installed augmentation (an equipment record tagged `Cybernetic`, `Bio-Tech`,
    /// `Magitech`, `Necrograft` or `Personal Upgrade`) prints its kind and body system; two
    /// installed in one system are refused by name (SRD: one augmentation per system).
    #[test]
    fn an_installed_augmentation_prints_its_system_and_one_per_system() {
        use super::REFUSED_AUGMENTATION_SYSTEM;
        use codex::rules_core::character_input::{ActiveState, EquipmentSelection};
        use codex::rules_core::pilot_compute::sf_defense::held;
        use codex::rules_core::corpus_loader::live_sheet_rules_for;
        use codex::rules_core::game_system::GameSystem;
        let package = live_sheet_rules_for(GameSystem::Starfinder1e).expect("the Starfinder package loads");
        let installed = |id: &str| EquipmentSelection {
            item_id: format!("core:equipment:{id}"),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
        };
        let input = technomancer();
        let (build, _) = crate::sf_adapter::build_from_input(package, &input).expect("builds");
        let held_set = held(package, &build).expect("holds");
        let mut equipment = input.chosen.equipment_selections.clone();
        equipment.push(installed("cybernetic_vocal_modulator"));
        let lines = super::sheet_lines(package, &build, &held_set, &input.chosen.spells_selected, &equipment).expect("prints");
        let line = lines.iter().find(|l| l.id == "core:equipment:cybernetic_vocal_modulator").expect("the augmentation prints");
        assert!(line.also.iter().any(|(t, _)| t == "Cybernetic augmentation, system: Throat"), "{:?}", line.also);
        assert!(line.prose.contains("vocal modulator"), "{:?}", line.prose);
        equipment.push(installed("dragon_gland_wyrmling"));
        let refused = super::sheet_lines(package, &build, &held_set, &input.chosen.spells_selected, &equipment).expect_err("two in the throat");
        assert_eq!(refused.id, REFUSED_AUGMENTATION_SYSTEM);
        assert!(refused.message.contains("Throat"), "{}", refused.message);
    }

    /// A spell selection the sheet cannot print is refused by name -- a claim-blocking
    /// diagnostic, never a silently missing line: a spell the package does not hold, a class
    /// the character does not have, a spell not on that class's list.
    #[test]
    fn an_unprintable_spell_selection_is_a_named_refusal() {
        use super::{REFUSED_SPELL_CLASS_NOT_HELD, REFUSED_SPELL_NOT_ON_CLASS_LIST, REFUSED_UNKNOWN_SPELL};
        let (_, _, input) = crate::sf_adapter::tests::seeds()
            .into_iter()
            .find(|(seed, _, _)| *seed == "SF-Technomancer-5")
            .expect("the technomancer seed");
        for (spell, class, refusal) in [
            ("core:spell:no_such_spell", "core:class:technomancer", REFUSED_UNKNOWN_SPELL),
            ("core:feat:mobility", "core:class:technomancer", REFUSED_UNKNOWN_SPELL),
            ("core:spell:magic_missile", "core:class:mystic", REFUSED_SPELL_CLASS_NOT_HELD),
            ("core:spell:mystic_cure_level_1", "core:class:technomancer", REFUSED_SPELL_NOT_ON_CLASS_LIST),
        ] {
            let mut bad = input.clone();
            bad.chosen.spells_selected[0].spell_id = spell.to_owned();
            bad.chosen.spells_selected[0].source_class_id = class.to_owned();
            let chassis = StarfinderAdapter.chassis_resolve(&bad);
            assert_eq!(chassis.diagnostics.len(), 1, "{spell}: {:?}", chassis.diagnostics);
            assert_eq!(chassis.diagnostics[0].id, refusal, "{spell}");
            assert!(chassis.diagnostics[0].claim_blocking && chassis.diagnostics[0].message.contains(spell), "{:?}", chassis.diagnostics);
        }
    }
}
