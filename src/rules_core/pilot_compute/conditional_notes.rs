//! The conditional notes the sheet prints under Hit Points, Initiative, the three saves and Armor
//! Class: every bonus a held record states for one of those totals that is **not** added to it,
//! with its condition in the engine's words. Read off the character's rendered sheet lines and
//! the converted package, never a per-record table -- the way
//! [`feat_skill_bonuses`](super::feat_skill_bonus_sheet_rules::feat_skill_bonuses) reads skill
//! bonuses, but for every record kind (feat, class feature, race trait, trait, item, spell, ...).
//!
//! # Mechanism (one rule per source, no per-record cases)
//!
//! Input: the lines `render_sheet` printed for this character
//! ([`PilotBaseChassisComputation::with_sheet_rules`](super::PilotBaseChassisComputation::with_sheet_rules)).
//!
//! 1. **A situational bonus line.** A line whose converted rule targets `Hp`, `Initiative`,
//!    `Save(<save>)` or `Ac` and whose gate left a situational condition (`SheetLine.condition`:
//!    "when active", "while raging") is a note in that list: `value` is the line's printed value
//!    (`"+2"`, or `""` when the evaluator left words), `condition` is the condition.
//! 2. **A held record's prose entry.** A line whose rule carries a prose segment of Aspect family
//!    `SaveBonus`, `AllSaveBonus`, `Save Bonus`, `RefSaveBonus`, `WillSaveBonus` or `ModifyAC`,
//!    and whose rendered prose holds that entry (the evaluator printed it as `"<family>: <words>"`
//!    after its own gate and `pick_last`), is a note: `condition` is the entry's words with every
//!    slot filled (Dwarf Hardy: "+2 racial bonus on saving throws against poison, spells, and
//!    spell-like abilities."), `value` is empty (the words carry the magnitude). The list:
//!    `ModifyAC` -> armor class; `RefSaveBonus` -> reflex; `WillSaveBonus` -> will; the other three
//!    -> the save(s) the words name (`Fortitude`/`Fort`, `Reflex`/`Ref`, `Will`, whole words), or
//!    all three when the words name none.
//!
//! Nothing is summed and nothing is folded into a total: every word is the engine's
//! (`SheetLine.label`, `.printed`, `.condition`, the rendered prose entry). A line that already
//! made a note in a list under rule 1 adds no rule-2 note to that same list, and a list never
//! holds the same `(label, value, condition)` twice.
//!
//! 3. **Combat maneuvers** (SD-40 E2.19). Rule 1 on a `Cmb` / `Cmd` target, and rule 2 on an
//!    Aspect `CombatBonus` entry whose words name CMB, CMD or combat maneuvers: words naming CMD,
//!    or "against" a maneuver, -> combat maneuver defense; the rest -> combat maneuver bonus; a
//!    note lands in one of the two, never both.
//!
//! 4. **Held `print: false` rules** (SD-40 E2.28). A held rule whose value folds into a total and
//!    prints no line (ACG Vermin Focus centipede) is evaluated with the same gate and slot fill
//!    (`sheet_rule::render_held_unprinted`) and its Aspect entries are gathered by rules 2 and 3;
//!    rule 1 reads printed lines only.
//!
//! Not gathered (named remainder): Aspect `CombatBonus` entries naming no combat maneuver (attack
//! and AC against a named foe, e.g. Defensive Training); encumbrance (`BonusTarget` has no
//! carrying-capacity target).

use crate::rules_core::sheet_rule::{BonusTarget, ProseFamily, Save, SheetLine, SheetRulePackage};

/// One note: a bonus the rules state for a total but do not add to it.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionalNote {
    /// The record's line label, as the sheet prints it (`Hardy`, `Gunslinger Initiative`).
    pub label: String,
    /// The line's printed value (`"+2"`), or `""` when the words carry it.
    pub value: String,
    /// When it applies, in the engine's words.
    pub condition: String,
    /// The sheet line's rule id.
    pub rule_id: String,
}

/// The notes per total, in sheet-line order.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionalNotes {
    pub hit_points: Vec<ConditionalNote>,
    pub initiative: Vec<ConditionalNote>,
    pub fortitude: Vec<ConditionalNote>,
    pub reflex: Vec<ConditionalNote>,
    pub will: Vec<ConditionalNote>,
    pub armor_class: Vec<ConditionalNote>,
    pub combat_maneuver_bonus: Vec<ConditionalNote>,
    pub combat_maneuver_defense: Vec<ConditionalNote>,
}

/// The eight lists by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum List {
    HitPoints,
    Initiative,
    Fortitude,
    Reflex,
    Will,
    ArmorClass,
    CombatManeuverBonus,
    CombatManeuverDefense,
}

const ALL_SAVES: [List; 3] = [List::Fortitude, List::Reflex, List::Will];

impl ConditionalNotes {
    fn list_mut(&mut self, list: List) -> &mut Vec<ConditionalNote> {
        match list {
            List::HitPoints => &mut self.hit_points,
            List::Initiative => &mut self.initiative,
            List::Fortitude => &mut self.fortitude,
            List::Reflex => &mut self.reflex,
            List::Will => &mut self.will,
            List::ArmorClass => &mut self.armor_class,
            List::CombatManeuverBonus => &mut self.combat_maneuver_bonus,
            List::CombatManeuverDefense => &mut self.combat_maneuver_defense,
        }
    }

    fn push(&mut self, list: List, note: ConditionalNote) {
        let notes = self.list_mut(list);
        let same = |n: &ConditionalNote| n.label == note.label && n.value == note.value && n.condition == note.condition;
        if !notes.iter().any(same) {
            notes.push(note);
        }
    }
}

/// The list a bonus target feeds, if it is one of the eight.
fn target_list(target: Option<&BonusTarget>) -> Option<List> {
    match target? {
        BonusTarget::Hp => Some(List::HitPoints),
        BonusTarget::Initiative => Some(List::Initiative),
        BonusTarget::Save(Save::Fortitude) => Some(List::Fortitude),
        BonusTarget::Save(Save::Reflex) => Some(List::Reflex),
        BonusTarget::Save(Save::Will) => Some(List::Will),
        BonusTarget::Ac => Some(List::ArmorClass),
        BonusTarget::Cmb => Some(List::CombatManeuverBonus),
        BonusTarget::Cmd => Some(List::CombatManeuverDefense),
        _ => None,
    }
}

/// The Aspect families gathered (rule 2).
const SAVE_ASPECTS: [&str; 3] = ["SaveBonus", "AllSaveBonus", "Save Bonus"];
const REFLEX_ASPECT: &str = "RefSaveBonus";
const WILL_ASPECT: &str = "WillSaveBonus";
const AC_ASPECT: &str = "ModifyAC";
const COMBAT_ASPECT: &str = "CombatBonus";

/// The combat maneuvers (CRB pp.198-201, APG p.320-322) an "against" names.
const MANEUVERS: [&str; 13] =
    ["bull rush", "dirty trick", "disarm", "drag", "grapple", "overrun", "pull", "push", "reposition", "steal", "sunder", "trip", "combat maneuver"];

/// The combat-maneuver list `words` feed (rule 3), or none when they name no combat maneuver.
fn combat_maneuver_list(words: &str) -> Option<List> {
    let lower = words.to_lowercase();
    let names_cmd = names_word(&lower, "cmd") || lower.contains("combat maneuver defense");
    if !(names_cmd || names_word(&lower, "cmb") || lower.contains("combat maneuver")) {
        return None;
    }
    // "against" then a maneuver as whole words ("against dragons" is not "drag").
    let against_maneuver = lower.match_indices("against ").any(|(at, word)| {
        let rest = lower[at + word.len()..].trim_start_matches("a ").trim_start_matches("an ");
        MANEUVERS.iter().any(|m| rest.starts_with(m) && !rest[m.len()..].starts_with(|c: char| c.is_ascii_alphanumeric()))
    });
    Some(if names_cmd || against_maneuver { List::CombatManeuverDefense } else { List::CombatManeuverBonus })
}

/// Whether `words` holds `name` as a whole word.
fn names_word(words: &str, name: &str) -> bool {
    words.split(|c: char| !c.is_ascii_alphanumeric()).any(|w| w == name)
}

/// The lists an Aspect entry of `family` with `words` feeds (rule 2).
fn aspect_lists(family: &str, words: &str) -> Vec<List> {
    match family {
        AC_ASPECT => vec![List::ArmorClass],
        REFLEX_ASPECT => vec![List::Reflex],
        WILL_ASPECT => vec![List::Will],
        COMBAT_ASPECT => combat_maneuver_list(words).into_iter().collect(),
        f if SAVE_ASPECTS.contains(&f) => {
            let named: Vec<List> = [(List::Fortitude, ["Fortitude", "Fort"]), (List::Reflex, ["Reflex", "Ref"]), (List::Will, ["Will", "Will"])]
                .into_iter()
                .filter(|(_, names)| names.iter().any(|n| names_word(words, n)))
                .map(|(list, _)| list)
                .collect();
            if named.is_empty() { ALL_SAVES.to_vec() } else { named }
        }
        _ => Vec::new(),
    }
}

/// Whether `family` is one of the gathered Aspect families.
fn gathered_aspect(family: &str) -> bool {
    SAVE_ASPECTS.contains(&family) || [REFLEX_ASPECT, WILL_ASPECT, AC_ASPECT, COMBAT_ASPECT].contains(&family)
}

/// The conditional notes over a character's rendered `lines` and the lines of its held
/// `print: false` rules, `unprinted` (`sheet_rule::render_held_unprinted`; see the module doc).
pub fn conditional_notes(package: &SheetRulePackage, lines: &[SheetLine], unprinted: &[SheetLine]) -> ConditionalNotes {
    let mut out = ConditionalNotes::default();
    let printed = lines.iter().map(|line| (line, true));
    for (line, is_printed) in printed.chain(unprinted.iter().map(|line| (line, false))) {
        let Some(rule) = package.rule(&line.id) else { continue };
        // Rule 1: a situational bonus line on one of the eight totals (printed lines only).
        let situational_list = match (&line.condition, target_list(rule.target.as_ref())) {
            (Some(condition), Some(list)) if is_printed => {
                out.push(
                    list,
                    ConditionalNote {
                        label: line.label.clone(),
                        value: line.printed.clone(),
                        condition: condition.clone(),
                        rule_id: line.id.clone(),
                    },
                );
                Some(list)
            }
            _ => None,
        };
        // Rule 2: the gathered Aspect entries the evaluator printed in this line's prose.
        let mut families: Vec<&str> = rule
            .prose
            .iter()
            .filter_map(|segment| match &segment.family {
                ProseFamily::Aspect(name) if gathered_aspect(name) => Some(name.as_str()),
                _ => None,
            })
            .collect();
        families.sort_unstable();
        families.dedup();
        for family in families {
            let prefix = format!("{family}: ");
            for words in line.prose.lines().filter_map(|l| l.strip_prefix(prefix.as_str())) {
                for list in aspect_lists(family, words) {
                    if Some(list) == situational_list {
                        continue;
                    }
                    out.push(
                        list,
                        ConditionalNote {
                            label: line.label.clone(),
                            value: String::new(),
                            condition: words.to_owned(),
                            rule_id: line.id.clone(),
                        },
                    );
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::character_input::CharacterInput;
    use crate::rules_core::class_census::load_sweep_fixture;
    use crate::rules_core::sheet_rule::ProsePiece;
    use crate::rules_core::sheet_rule_package;

    fn package() -> &'static SheetRulePackage {
        sheet_rule_package::package().as_ref().expect("the converted package loads")
    }

    fn census(class_name: &str, level: u8) -> CharacterInput {
        let fixture = load_sweep_fixture().expect("census fixture");
        crate::rules_core::class_seeds::input_for(&fixture, class_name, level)
    }

    fn notes_for(input: &CharacterInput, race_traits: &[&str]) -> ConditionalNotes {
        let race_traits: Vec<String> = race_traits.iter().map(|t| (*t).to_owned()).collect();
        let computed =
            crate::rules_core::pilot_compute::compute_pilot_base_chassis(input).with_sheet_rules(input, package(), &race_traits);
        let unprinted = computed.held_unprinted_lines(input, package(), &race_traits);
        conditional_notes(package(), &computed.sheet_lines, &unprinted)
    }

    const HARDY_WORDS: &str = "+2 racial bonus on saving throws against poison, spells, and spell-like abilities.";

    /// CRB p.21, Dwarf Hardy: "+2 racial bonus on saving throws against poison, spells, and
    /// spell-like abilities." The record's `SaveBonus` entry names no single save, so the words
    /// land in all three lists, once each, and in no other list.
    #[test]
    fn conditional_notes_put_dwarf_hardy_under_every_save() {
        let mut dwarf = census("fighter", 1);
        dwarf.chosen.race_id = "race:dwarf".to_owned();
        let notes = notes_for(&dwarf, &["Dwarf ~ Hardy"]);
        for (save, list) in [("fortitude", &notes.fortitude), ("reflex", &notes.reflex), ("will", &notes.will)] {
            let hardy: Vec<&ConditionalNote> = list.iter().filter(|n| n.rule_id == "core_rulebook:race_trait:dwarf_hardy").collect();
            assert_eq!(hardy.len(), 1, "{save}: Hardy once: {notes:#?}");
            assert_eq!(hardy[0].label, "Hardy");
            assert_eq!(hardy[0].condition, HARDY_WORDS, "{save}");
            assert_eq!(hardy[0].value, "", "{save}: the words carry the +2");
        }
        let elsewhere = [&notes.hit_points, &notes.initiative, &notes.armor_class];
        assert!(elsewhere.iter().all(|l| l.iter().all(|n| !n.rule_id.contains("dwarf_hardy"))), "{notes:#?}");
    }

    /// APG p.333, Veteran of Battle (combat trait): "+1 trait bonus on initiative checks". The
    /// converted record targets `Initiative` with a situational gate, so the line is printed with
    /// its condition and never added to the total: it lands in `initiative` with the line's own
    /// printed value and condition.
    #[test]
    fn conditional_notes_put_a_situational_initiative_line_under_initiative() {
        let mut input = census("fighter", 1);
        input.chosen.selected_traits.push("trait:trait_veteran_of_battle".to_owned());
        let notes = notes_for(&input, &[]);
        let veteran: Vec<&ConditionalNote> =
            notes.initiative.iter().filter(|n| n.rule_id == "advanced_players_guide:trait:trait_veteran_of_battle").collect();
        assert_eq!(veteran.len(), 1, "{notes:#?}");
        assert_eq!(veteran[0].value, "+1");
        assert_eq!(veteran[0].condition, "requirement withheld", "the engine's own condition words");
    }

    /// A Human Fighter 3 (Aldric's build on the census fixture): CRB p.55, Bravery "Starting at
    /// 2nd level, a fighter gains a +1 bonus on Will saves against fear." The record's `SaveBonus`
    /// entry names Will, so it lands under `will` alone; nothing else on HP, initiative, saves
    /// or AC is conditional for this build.
    #[test]
    fn conditional_notes_hold_only_bravery_for_a_human_fighter_three() {
        let notes = notes_for(&census("fighter", 3), &[]);
        let bravery = ConditionalNote {
            label: "Bravery".to_owned(),
            value: String::new(),
            condition: "+1 Will vs. fear".to_owned(),
            rule_id: "core_rulebook:class_feature:fighter_bravery".to_owned(),
        };
        assert_eq!(notes, ConditionalNotes { will: vec![bravery], ..ConditionalNotes::default() }, "{notes:#?}");
    }

    /// Negative control: a Human Fighter 1 (no Bravery yet) holds no conditional note at all.
    #[test]
    fn conditional_notes_are_empty_for_a_human_fighter_one() {
        let notes = notes_for(&census("fighter", 1), &[]);
        assert_eq!(notes, ConditionalNotes::default(), "{notes:#?}");
    }

    /// SD-40 E2.19, the two combat-maneuver lists. CRB p.21, Dwarf Stability: "+4 bonus to their
    /// Combat Maneuver Defense when resisting a bull rush or trip attempt while standing on the
    /// ground" -> `combat_maneuver_defense`. ARG p.11, Relentless (replaces Stability): "+2 bonus
    /// on Combat Maneuver checks made to bull rush or overrun an opponent" -> `combat_maneuver_bonus`.
    /// Each lands once, in its one list. The routing over the words: naming CMD, or "against" a
    /// maneuver, is CMD (ACG Verminous Hunter, Vermin Focus centipede: "+2 on CMD against pull,
    /// push, reposition, and trip combat maneuvers"); the rest is CMB; words naming neither feed
    /// no list.
    #[test]
    fn conditional_notes_gather_combat_maneuver_bonuses() {
        let mut dwarf = census("fighter", 1);
        dwarf.chosen.race_id = "race:dwarf".to_owned();
        let stable = notes_for(&dwarf, &["Dwarf ~ Stability"]);
        let stability = ConditionalNote {
            label: "Stability".to_owned(),
            value: String::new(),
            condition: "+4 bonus to their Combat Maneuver Defense when resisting a bull rush or trip attempt while standing on the ground.".to_owned(),
            rule_id: "core_rulebook:race_trait:dwarf_stability".to_owned(),
        };
        assert_eq!(stable.combat_maneuver_defense, vec![stability], "{stable:#?}");
        assert!(stable.combat_maneuver_bonus.is_empty(), "{stable:#?}");

        let relentless = notes_for(&dwarf, &["Dwarf ~ Relentless"]);
        let cmb: Vec<(&str, &str)> =
            relentless.combat_maneuver_bonus.iter().map(|n| (n.label.as_str(), n.condition.as_str())).collect();
        assert_eq!(
            cmb,
            [("Relentless", "+2 bonus on Combat Maneuver checks made to bull rush or overrun an opponent. This bonus only applies while both the dwarf and his opponent are standing on the ground.")],
            "{relentless:#?}"
        );
        assert!(relentless.combat_maneuver_defense.iter().all(|n| !n.rule_id.contains("relentless")), "{relentless:#?}");

        assert_eq!(combat_maneuver_list("+2 on CMD against pull, push, reposition, and trip combat maneuvers"), Some(List::CombatManeuverDefense));
        assert_eq!(combat_maneuver_list("+3 alchemical bonus on Strength checks and to CMB and CMD."), Some(List::CombatManeuverDefense), "once, in CMD");
        assert_eq!(combat_maneuver_list("+4 racial bonus against bull rush or trip combat maneuvers"), Some(List::CombatManeuverDefense), "against a maneuver");
        assert_eq!(
            combat_maneuver_list("+2 dodge bonus to AC against monsters of the aberration type and a +2 racial bonus to their CMB on attempts made to grapple such creatures"),
            Some(List::CombatManeuverBonus),
            "'against monsters' is not against a maneuver"
        );
        assert_eq!(combat_maneuver_list("+2 racial bonus on combat maneuver checks to escape a grapple against any creature"), Some(List::CombatManeuverBonus));
        assert_eq!(combat_maneuver_list("+1 on attack rolls against goblinoids"), None);
        assert_eq!(combat_maneuver_list("+2 bonus against bull rush attempts"), None, "names no combat maneuver: not gathered");
        assert_eq!(combat_maneuver_list("+1 on combat maneuver checks against dragons"), Some(List::CombatManeuverBonus), "'dragons' is not 'drag'");
    }

    const CENTIPEDE: &str = "advanced_class_guide:class_feature:verminous_hunter_vermin_focus_centipede";

    /// SD-40 E2.28: a held `print: false` rule's gathered Aspect entries. ACG p.130, Verminous
    /// Hunter, Vermin Focus (centipede): "+2 bonus to CMD against pull, push, reposition, and trip
    /// combat maneuvers". The converted record is `print: false` (its Climb bonus folds into the
    /// skill total), so it renders no sheet line; its `CombatBonus` entry still lands in
    /// `combat_maneuver_defense`, once, with the slot filled at hunter level 1.
    #[test]
    fn conditional_notes_gather_held_unprinted_rules() {
        use crate::rules_core::character_input::SelectedChoice;
        use crate::rules_core::sheet_rule::{render_held_unprinted, render_sheet, CharacterFacts, HeldSeed};
        let mut hunter = census("hunter", 1);
        hunter.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: "choice:verminous_hunter_vermin_focus".to_owned(),
            selection_id: "class_feature:verminous_hunter_vermin_focus_centipede".to_owned(),
        });
        let computed = crate::rules_core::pilot_compute::compute_pilot_base_chassis(&hunter);
        let mut seed = HeldSeed::from_character(&hunter, &computed);
        // No character-record pick holds an archetype's sheet rule today (`choice:archetype` feeds
        // the archetype resolver, not the held set): the test holds Verminous Hunter by id.
        seed.rule_ids.push("advanced_class_guide:class_feature:hunter_archetype_verminous_hunter".to_owned());
        let facts = CharacterFacts::from_character(&hunter, &computed).with_linked_picks(package(), &seed);
        let printed = render_sheet(package(), &seed, &facts);
        assert!(printed.iter().all(|l| l.id != CENTIPEDE), "print: false renders no sheet line");
        let unprinted = render_held_unprinted(package(), &seed, &facts);
        let notes = conditional_notes(package(), &printed, &unprinted);
        let centipede = ConditionalNote {
            label: "Centipede".to_owned(),
            value: String::new(),
            condition: "+2 on CMD against pull, push, reposition, and trip combat maneuvers".to_owned(),
            rule_id: CENTIPEDE.to_owned(),
        };
        assert_eq!(notes.combat_maneuver_defense, vec![centipede], "{notes:#?}");
        assert!(notes.combat_maneuver_bonus.iter().all(|n| n.rule_id != CENTIPEDE), "{notes:#?}");
    }

    /// The save an Aspect entry's words name picks its list; no name means all three.
    #[test]
    fn conditional_notes_route_save_aspects_by_the_words() {
        assert_eq!(aspect_lists("SaveBonus", "+2 Will vs. scrying"), vec![List::Will]);
        assert_eq!(aspect_lists("SaveBonus", "+1 on Reflex saving throws against traps"), vec![List::Reflex]);
        assert_eq!(aspect_lists("SaveBonus", "+4 Fortitude (profane) vs fatigue"), vec![List::Fortitude]);
        assert_eq!(aspect_lists("SaveBonus", "+2 vs. mind-affecting effects"), ALL_SAVES.to_vec());
        assert_eq!(aspect_lists("SaveBonus", "+2 vs. effects that will you"), ALL_SAVES.to_vec(), "lower-case 'will' is not the save");
        assert_eq!(aspect_lists("WillSaveBonus", "+4 vs. sanity damage"), vec![List::Will]);
        assert_eq!(aspect_lists("RefSaveBonus", "+1 vs. trample attacks"), vec![List::Reflex]);
        assert_eq!(aspect_lists("ModifyAC", "+4 deflection vs. evil"), vec![List::ArmorClass]);
        assert_eq!(aspect_lists("CombatBonus", "+1 on attack rolls"), Vec::<List>::new(), "CombatBonus is not gathered");
    }

    /// The population the two rules read, counted over the whole converted package: rules with a
    /// situational gate per target list (rule 1), and gathered Aspect prose entries (rule 2).
    /// The same figures as the replan census, a second implementation
    /// (`python3 -I docs/release/SD-40-pf-sheet-design/artifacts/epic_2/replan-r1/census.py`:
    /// Hp 15, Initiative 10, Fortitude 52, Reflex 58, Will 78, Ac 178; SaveBonus 407,
    /// AllSaveBonus 3, Save Bonus 2, WillSaveBonus 3, RefSaveBonus 2, ModifyAC 6). A converter
    /// change that moves the population shows here. SD-40 E2.19: + the 22 CombatBonus entries
    /// naming a combat maneuver (`replan-r2/census.py` (c)).
    #[test]
    fn conditional_notes_census_matches_the_package() {
        let mut by_target = std::collections::BTreeMap::<&str, usize>::new();
        for rule in package().rules.values() {
            let situational = format!("{:?}", rule.applies).contains("Situational");
            if let (true, Some(list)) = (situational, target_list(rule.target.as_ref())) {
                *by_target.entry(match list {
                    List::HitPoints => "Hp",
                    List::Initiative => "Initiative",
                    List::Fortitude => "Fortitude",
                    List::Reflex => "Reflex",
                    List::Will => "Will",
                    List::ArmorClass => "Ac",
                    List::CombatManeuverBonus => "Cmb",
                    List::CombatManeuverDefense => "Cmd",
                }).or_default() += 1;
            }
            for segment in &rule.prose {
                match &segment.family {
                    ProseFamily::Aspect(name) if name == COMBAT_ASPECT => {
                        let words: String =
                            segment.pieces.iter().filter_map(|p| if let ProsePiece::Text(t) = p { Some(t.as_str()) } else { None }).collect();
                        if combat_maneuver_list(&words).is_some() {
                            *by_target.entry("combat maneuver entries").or_default() += 1;
                            if !rule.print {
                                *by_target.entry("combat maneuver entries on print: false rules").or_default() += 1;
                            }
                        }
                    }
                    ProseFamily::Aspect(name) if gathered_aspect(name) => {
                        *by_target.entry("aspect entries").or_default() += 1;
                        if !rule.print {
                            *by_target.entry("aspect entries on print: false rules").or_default() += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
        eprintln!("gatherable rules by list: {by_target:?}");
        let expected: std::collections::BTreeMap<&str, usize> = [
            ("Hp", 15),
            ("Initiative", 10),
            ("Fortitude", 52),
            ("Reflex", 58),
            ("Will", 78),
            ("Ac", 178),
            ("aspect entries", 407 + 3 + 2 + 3 + 2 + 6),
            // replan-r2/census.py (c): "of the 314 CombatBonus aspects 22 name CMB, CMD or combat
            // maneuver"; situational Cmb 0, Cmd 0 (no entry).
            ("combat maneuver entries", 22),
            // SD-40 E2.28: the entries only `render_held_unprinted` reaches (the five ACG Vermin
            // Focus CombatBonus records; eight SaveBonus records), E2.19's receipt "Figures".
            ("combat maneuver entries on print: false rules", 5),
            ("aspect entries on print: false rules", 8),
        ]
        .into_iter()
        .collect();
        assert_eq!(by_target, expected, "the replan census (census.py) and this count agree");
    }
}
