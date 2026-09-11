//! Catalog-mode prose: a converted rule's words with **no character in hand**.
//!
//! # Why this module exists
//!
//! SD-35 `decisions.md §11` — "there should be nothing left of pcgen" on the live side. The
//! desktop catalog screens (Feat Catalog, Spell Catalog, Monster Catalog, Equipment Catalog,
//! the companion and intelligent-item catalogs, the reference library) each served a record's
//! description by parsing the ingest format's own description token at run time. The
//! substitution those call sites performed already happens at ingest, in
//! `src/pcgen_import/sheet_rule/`: the converted [`SheetRule::prose`] carries the record's
//! words with **typed** holes ([`ProsePiece::Slot`] over an [`Expr`]) rather than the source
//! format's positional placeholders and argument tail.
//!
//! # Why the sheet evaluator is not enough
//!
//! [`crate::rules_core::sheet_rule::evaluate`] renders a rule **for a character**: a
//! [`ProsePiece::Slot`] evaluates to a number. A catalog screen has no character, and the
//! evaluator's empty-facts answer for `CasterLevel`/`Level`/`AbilityMod` is `0` — so
//! Advanced Race Guide's *Absorbing Inhalation* would read *"for up to 0 rounds"*. A wrong
//! computed number looks right, which is the one failure mode a presence gate never catches
//! (`docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003_cycle1_receipt.md`).
//!
//! `decisions.md §1` already rules what to print instead: **a final number, dice in final
//! form, or the rule's words.** A slot the catalog cannot settle is not a number — it is the
//! rule's words. So this module renders the same prose the sheet renders, and substitutes
//! each unsettled slot with the words for the term it stands on: *"for up to your caster
//! level rounds"*. A slot whose expression is a constant still prints its number, because
//! that number **is** settled.
//!
//! # What is NOT in here
//!
//! No token, no formula string, no argument tail, no ingest-format vocabulary. The input is a
//! converted [`SheetRule`]; the output is English.

use crate::rules_core::level_up_option_filter::{
    ability_word, describe_gate, describe_prof, expr_words, label_of, save_word, words_of_id,
};
use crate::rules_core::sheet_rule::{
    fold_dice_modifier, slug, Applies, BonusTarget, Choice, CountsAs, Effect, Expr, Fact, Grant,
    Granter, OptionSet, ProseFamily, ProsePiece, ProseSegment, Scope, SheetRule, SheetRulePackage,
    SheetValue, ValueRole, WeaponRef,
};
use std::collections::BTreeMap;

/// The families a catalog screen's `description` field is allowed to carry: the record's own
/// descriptive text, not its stat block. `Desc`, `Benefit` and `Special` are the three the
/// converter writes from a record's authored prose; `Aspect`/`StatBlock`/`WhenActive` are the
/// structured lines the sheet prints beside the description (a spell's casting time, a
/// companion's advancement aspect), and a catalog row carries those in its own typed fields.
const DESCRIPTION_FAMILIES: [&str; 3] = ["Desc", "Benefit", "Special"];

/// `rule.prose` rendered with no character: every family, in the sheet's own order.
pub fn catalog_prose(package: &SheetRulePackage, rule: &SheetRule) -> String {
    render(package, &rule.prose, None)
}

/// The description a catalog row serves: the `Desc`/`Benefit`/`Special` families only.
///
/// `None` when the record states no descriptive prose at all — the honest answer for the
/// template and bookkeeping rows that carry a stat block and nothing else. Never a fabricated
/// or partial sentence.
pub fn catalog_description(package: &SheetRulePackage, rule: &SheetRule) -> Option<String> {
    let text = render(package, &rule.prose, Some(&DESCRIPTION_FAMILIES));
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

/// [`catalog_description`] for the rule of `kind` whose name slugs to `name`'s slug.
///
/// The lookup is [`SheetRulePackage::find`]'s — `core_rulebook` first, then the
/// lexicographically first book id — so a name several printings share resolves to one
/// record, deterministically, the same way the sheet resolves it.
pub fn catalog_description_by_name(
    package: &SheetRulePackage,
    kind: &str,
    name: &str,
) -> Option<String> {
    let id = package.find(kind, &slug(name))?;
    catalog_description(package, package.rule(id)?)
}

/// [`catalog_prose`] for the rule of `kind` whose name slugs to `name`'s slug.
pub fn catalog_prose_by_name(
    package: &SheetRulePackage,
    kind: &str,
    name: &str,
) -> Option<String> {
    let id = package.find(kind, &slug(name))?;
    Some(catalog_prose(package, package.rule(id)?))
}

/// A rule with **no prose at all**, rendered from its typed fields as words.
///
/// # The gap this closes
///
/// `catalog_description` serves the `Desc`/`Benefit`/`Special` families and nothing else, which
/// is the right answer for a record whose author wrote a description. A large part of this
/// corpus has none: the source ships `skill`, `language`, `domain` and most of `template` as a
/// bare mechanical row. The desktop reference library served those rows by printing the ingest
/// format's own token lines — key head, colon and value — straight onto the screen, a live-side
/// read of the ingest format, which `decisions.md §11` rules out, and ingest-format vocabulary
/// on a player's screen, which `decisions.md §1` rules out.
///
/// The converted record already holds those same facts, typed: the value, the second numbers
/// beside it, what sheet total it feeds and under which stacking type, its tags, the condition
/// it applies under, the choice it offers, what holding it does to the fact set, and who hands
/// it out. This renders exactly those, in that fixed order, through the **same vocabulary**
/// [`describe_gate`] and [`expr_words`] give a prerequisite line — no second describer, no
/// token, no formula string.
///
/// `None` when the record states nothing beyond its own identity. That is the honest answer,
/// and it is a smaller population than the token dump suggested: a record whose only remaining
/// content was an ingest bookkeeping row -- a visibility flag, a rule-ordering directive --
/// was never describing a rule to a player.
pub fn catalog_field_summary(package: &SheetRulePackage, rule: &SheetRule) -> Option<String> {
    let mut parts: Vec<String> = Vec::new();

    if let Some(words) = value_words(package, &rule.value) {
        parts.push(format!("Value: {words}"));
    }
    for (role, value) in &rule.also {
        if let Some(words) = value_words(package, value) {
            parts.push(format!("{}: {words}", role_label(role)));
        }
    }
    if let Some(target) = &rule.target {
        let mut line = format!("Adds to {}", target_words(package, target));
        if let Some(bonus_type) = rule.bonus_type.as_ref().filter(|b| !b.name.trim().is_empty()) {
            line = format!("{line} as a {} bonus", bonus_type.name);
        }
        parts.push(line);
    }
    if !rule.tags.is_empty() {
        parts.push(format!("Tags: {}", rule.tags.join(", ")));
    }
    if rule.applies != Applies::Always {
        let condition = describe_gate(package, &rule.applies);
        if !condition.is_empty() && condition != "no prerequisite" {
            parts.push(format!("Applies if {condition}"));
        }
    }
    if let Some(choice) = &rule.offers {
        parts.push(choice_words(package, choice));
    }
    for effect in &rule.grants {
        parts.push(effect_words(package, effect));
    }
    if !rule.granted_by.is_empty() {
        parts.push(granted_by_words(package, &rule.granted_by));
    }
    // Read backwards: a record that states only "apply this other thing" carries that fact on
    // the far end of the edge, never in its own fields.
    let hands_out = package.granted_from(&rule.id);
    if !hands_out.is_empty() {
        let mut names: Vec<String> = Vec::new();
        for id in hands_out {
            let words = label_of(package, id);
            if !names.contains(&words) {
                names.push(words);
            }
        }
        if names.len() > MAX_NAMED_OPTIONS {
            parts.push(format!("Grants {} rules", names.len()));
        } else {
            parts.push(format!("Grants {}", names.join(", ")));
        }
    }

    // Last resort, and only when the record's own fields said nothing: a record whose entire
    // content is a variable contribution (a bonus to a named rules variable, or the
    // declaration of one) carries that fact in the package's variable tables, keyed the other
    // way round. Scanning them is linear, so it happens for the few hundred rules that would
    // otherwise be blank rather than for every rule a catalog loads.
    if parts.is_empty() {
        parts.extend(var_contribution_words(package, &rule.id));
    }

    if parts.is_empty() {
        None
    } else {
        Some(parts.join("; "))
    }
}

/// At most this many variable clauses print; beyond that the record is a bookkeeping row and
/// the count is the honest sentence.
const MAX_VAR_CLAUSES: usize = 6;

/// What a rule does to the package's named variables, as words: the value it adds and the
/// condition it adds it under, then the variables it declares.
///
/// A variable with no label is skipped rather than named generically — an id is a content
/// hash, and "a rules variable" is a phrase, not a fact.
fn var_contribution_words(package: &SheetRulePackage, rule_id: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut declares: Vec<String> = Vec::new();
    for table in package.vars.values() {
        if table.label.trim().is_empty() {
            continue;
        }
        for contribution in &table.contributions {
            if contribution.rule_id != rule_id {
                continue;
            }
            let value = match const_value(&contribution.expr) {
                Some(n) => n.to_string(),
                None => expr_words(package, &contribution.expr),
            };
            let mut line = format!("Adds {value} to {}", table.label);
            if contribution.when != Applies::Always {
                let condition = describe_gate(package, &contribution.when);
                if !condition.is_empty() && condition != "no prerequisite" {
                    line = format!("{line} if {condition}");
                }
            }
            if !out.contains(&line) {
                out.push(line);
            }
        }
        if table.declared_by.iter().any(|id| id == rule_id) {
            let line = format!("Declares {}", table.label);
            if !declares.contains(&line) {
                declares.push(line);
            }
        }
    }
    if out.len() + declares.len() > MAX_VAR_CLAUSES {
        return vec![format!("Contributes to {} rules variables", out.len() + declares.len())];
    }
    out.extend(declares);
    out
}

/// The description a catalog row serves, prose first and the typed fields when there is no
/// prose: [`catalog_description`], then [`catalog_prose`]'s stat-block families, then
/// [`catalog_field_summary`].
///
/// The three tiers are distinguishable by the caller — a stat block and a field summary are
/// real content but they are not the record's authored words — via [`CatalogDescription::tier`].
pub fn catalog_description_or_fields(
    package: &SheetRulePackage,
    rule: &SheetRule,
) -> Option<CatalogDescription> {
    if let Some(text) = catalog_description(package, rule) {
        return Some(CatalogDescription { text, tier: DescriptionTier::Prose });
    }
    let stat_block = catalog_prose(package, rule);
    if !stat_block.is_empty() {
        return Some(CatalogDescription { text: stat_block, tier: DescriptionTier::StatBlock });
    }
    catalog_field_summary(package, rule)
        .map(|text| CatalogDescription { text, tier: DescriptionTier::Fields })
}

/// A resolved catalog description and which tier produced it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogDescription {
    pub text: String,
    pub tier: DescriptionTier,
}

/// Which of the three tiers [`catalog_description_or_fields`] answered from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescriptionTier {
    /// The record's own authored words (`Desc`/`Benefit`/`Special`).
    Prose,
    /// The record's structured lines (a casting time, an advancement aspect) — real content,
    /// authored by the source, but not a description.
    StatBlock,
    /// [`catalog_field_summary`]: the typed fields as words, for a record with no prose at all.
    Fields,
}

/// A [`SheetValue`] as words; `None` for [`SheetValue::Text`], whose value *is* the prose.
fn value_words(package: &SheetRulePackage, value: &SheetValue) -> Option<String> {
    match value {
        SheetValue::Text => None,
        SheetValue::Number(expr) => Some(match const_value(expr) {
            Some(n) => n.to_string(),
            None => expr_words(package, expr),
        }),
        SheetValue::Dice { dice, modifier, .. } => Some(match modifier.as_ref().map(const_value) {
            None => dice.clone(),
            Some(Some(n)) => fold_dice_modifier(dice, n),
            Some(None) => {
                let words = expr_words(package, modifier.as_ref().expect("matched Some"));
                format!("{dice} plus {words}")
            }
        }),
        SheetValue::DiceBySize(by_size) => Some(format!("{} (by size)", by_size.join(" / "))),
    }
}

/// The label a second number on the line prints under.
fn role_label(role: &ValueRole) -> String {
    match role {
        ValueRole::Uses { period } => format!("Uses per {}", words_of_id(period)),
        ValueRole::CasterLevel => "Caster level".to_owned(),
        ValueRole::SaveDc => "Save DC".to_owned(),
    }
}

/// The sheet total a rule feeds, in the sheet's own words.
fn target_words(package: &SheetRulePackage, target: &BonusTarget) -> String {
    match target {
        BonusTarget::Ability(a) => ability_word(*a).to_owned(),
        BonusTarget::Skill(s) => words_of_id(s),
        BonusTarget::SkillSituation { skill, situation } => {
            format!("{} ({situation})", words_of_id(skill))
        }
        BonusTarget::Save(s) => format!("{} saves", save_word(*s)),
        BonusTarget::BaseSave(s) => format!("the base {} save", save_word(*s)),
        BonusTarget::Ac => "Armor Class".to_owned(),
        BonusTarget::Attack => "attack rolls".to_owned(),
        BonusTarget::BaseAttack => "the base attack bonus".to_owned(),
        BonusTarget::Damage(w) => format!("{} damage", weapon_words(w)),
        BonusTarget::Hp => "hit points".to_owned(),
        BonusTarget::Initiative => "initiative".to_owned(),
        BonusTarget::Cmb => "the combat maneuver bonus".to_owned(),
        BonusTarget::Cmd => "the combat maneuver defense".to_owned(),
        BonusTarget::Speed(mode) => format!("{} speed", words_of_id(mode)),
        BonusTarget::Vision(tag) => words_of_id(tag),
        BonusTarget::Dr => "damage reduction".to_owned(),
        BonusTarget::SpellDc(scope) => format!("spell save DCs {}", scope_words(package, scope)),
        BonusTarget::CasterLevel(scope) => format!("caster level {}", scope_words(package, scope)),
        BonusTarget::SpellcastingLevels(class) => {
            format!("spellcasting levels as a {}", words_of_id(class))
        }
        BonusTarget::SpellCell { class, level } => {
            format!("level {level} {} spells per day", words_of_id(class))
        }
        BonusTarget::SpellsKnown { class, level } => {
            format!("level {level} {} spells known", words_of_id(class))
        }
        BonusTarget::Pool(pool) => format!("the {} pool", words_of_id(pool)),
        BonusTarget::DamageSize(w) => format!("{} damage die size", weapon_words(w)),
        BonusTarget::WeaponAttack(w) => format!("{} attack rolls", weapon_words(w)),
        BonusTarget::SkillGroup(tag) => format!("every {} skill", words_of_id(tag)),
        BonusTarget::Chosen(choice) => format!("the choice made for {}", words_of_id(choice)),
        BonusTarget::Other(other) => words_of_id(other),
    }
}

fn weapon_words(weapon: &WeaponRef) -> String {
    match weapon {
        WeaponRef::Any => "weapon".to_owned(),
        WeaponRef::Melee => "melee weapon".to_owned(),
        WeaponRef::Ranged => "ranged weapon".to_owned(),
        WeaponRef::Named(name) => words_of_id(name),
        WeaponRef::Group(tag) => format!("{} weapon", words_of_id(tag)),
        WeaponRef::Chosen(choice) => {
            format!("the weapon chosen for {}", words_of_id(choice))
        }
    }
}

fn scope_words(package: &SheetRulePackage, scope: &Scope) -> String {
    match scope {
        Scope::All => "for every class".to_owned(),
        Scope::Class(class) => format!("as a {}", words_of_id(class)),
        Scope::School(school) => format!("for the {school} school"),
        Scope::Subschool(sub) => format!("for the {sub} subschool"),
        Scope::Descriptor(d) => format!("for {d} spells"),
        Scope::Spell(id) => format!("for {}", label_of(package, id)),
        Scope::Chosen(choice) => format!("for the choice made for {}", words_of_id(choice)),
    }
}

/// The choice a record offers, as words.
fn choice_words(package: &SheetRulePackage, choice: &Choice) -> String {
    let count = match const_value(&choice.count) {
        Some(n) => n.to_string(),
        None => expr_words(package, &choice.count),
    };
    format!("Offers a choice of {count} from {}", option_set_words(package, &choice.from))
}

/// At most this many named options print before the list is summarised by its size: a catalog
/// line is a sentence, not the option list itself.
const MAX_NAMED_OPTIONS: usize = 6;

fn named_list(kind: &str, names: &[String]) -> String {
    if names.is_empty() {
        return kind.to_owned();
    }
    if names.len() > MAX_NAMED_OPTIONS {
        return format!("{} {kind}", names.len());
    }
    format!(
        "{kind}: {}",
        names.iter().map(|n| words_of_id(n)).collect::<Vec<_>>().join(", ")
    )
}

fn option_set_words(package: &SheetRulePackage, from: &OptionSet) -> String {
    match from {
        OptionSet::Rules { pool, tags, .. } => {
            if tags.is_empty() {
                format!("the {} list", words_of_id(pool))
            } else {
                format!("the {} list tagged {}", words_of_id(pool), tags.join(", "))
            }
        }
        OptionSet::Skills(list) => named_list("skills", list),
        OptionSet::Weapons(list) => named_list("weapons", list),
        OptionSet::Spells { class, levels } => {
            format!("{} spells of levels {} to {}", words_of_id(class), levels.0, levels.1)
        }
        OptionSet::Languages(list) => named_list("languages", list),
        OptionSet::Templates(list) => {
            let names: Vec<String> = list.iter().map(|id| label_of(package, id)).collect();
            named_list("templates", &names)
        }
        OptionSet::Classes(list) => named_list("classes", list),
        OptionSet::Races(list) => named_list("races", list),
        OptionSet::Schools => "the schools of magic".to_owned(),
        OptionSet::Deities => "the deities".to_owned(),
        OptionSet::Domains => "the domains".to_owned(),
        OptionSet::Equipment => "the equipment list".to_owned(),
        OptionSet::FreeText => "any text the player writes in".to_owned(),
        OptionSet::Number { min, max } => format!(
            "the numbers {} to {}",
            const_value(min).map_or_else(|| expr_words(package, min), |n| n.to_string()),
            const_value(max).map_or_else(|| expr_words(package, max), |n| n.to_string()),
        ),
    }
}

/// What holding the rule does to the fact set, as words.
fn effect_words(package: &SheetRulePackage, effect: &Effect) -> String {
    match effect {
        Effect::FactGrant(fact) => format!("Grants {}", fact_words(fact)),
        Effect::FactRevoke(fact) => format!("Removes {}", fact_words(fact)),
        Effect::CountsAs(counts) => format!("Counts as {}", counts_as_words(package, counts)),
        Effect::Waives(id) => format!("Waives the prerequisites of {}", label_of(package, id)),
        Effect::Revokes(id) => format!("Revokes {}", label_of(package, id)),
        Effect::FactDeclare { name, value } => {
            format!("Declares {} as {value}", words_of_id(name))
        }
    }
}

fn counts_as_words(package: &SheetRulePackage, counts: &CountsAs) -> String {
    match counts {
        CountsAs::Rule(id) => label_of(package, id),
        CountsAs::Class(class) => format!("the {} class", words_of_id(class)),
        CountsAs::Race(race) => format!("the {} race", words_of_id(race)),
    }
}

fn fact_words(fact: &Fact) -> String {
    match fact {
        Fact::ClassSkill(skill) => format!("{} as a class skill", words_of_id(skill)),
        Fact::ClassSkillGroup(tag) => format!("every {} skill as a class skill", words_of_id(tag)),
        Fact::ClassSkillChosen(choice) => {
            format!("the skill chosen for {} as a class skill", words_of_id(choice))
        }
        Fact::CrossClassSkill(skill) => {
            format!("{} as a cross-class skill", words_of_id(skill))
        }
        Fact::Language(language) => format!("the {language} language"),
        Fact::Proficiency(prof) => format!("proficiency with {}", describe_prof(prof)),
        Fact::Equipment(item) => format!("the item {item}"),
        Fact::CompanionSlots { role, count } => match const_value(count) {
            Some(n) => format!("{n} {} companion slot(s)", words_of_id(role)),
            None => format!("{} companion slot(s)", words_of_id(role)),
        },
        Fact::Chosen(choice) => format!("the option chosen for {}", words_of_id(choice)),
    }
}

/// Who hands the rule out. Capped: a record every class grants is a sentence about the count,
/// not a list of forty class names.
fn granted_by_words(package: &SheetRulePackage, granted_by: &[Grant]) -> String {
    let mut names: Vec<String> = Vec::new();
    for grant in granted_by {
        let words = granter_words(package, &grant.by);
        if !names.contains(&words) {
            names.push(words);
        }
    }
    if names.len() > MAX_NAMED_OPTIONS {
        return format!("Granted by {} sources", names.len());
    }
    format!("Granted by {}", names.join(", "))
}

fn granter_words(package: &SheetRulePackage, by: &Granter) -> String {
    match by {
        Granter::Rule(id) | Granter::Deity(id) => label_of(package, id),
        Granter::Class { id, at_level } => {
            format!("{} at level {at_level}", words_of_id(id))
        }
        Granter::ClassSpellList { id, spell_level } => {
            format!("the {} spell list at spell level {spell_level}", words_of_id(id))
        }
        Granter::Race(race) => format!("the {} race", words_of_id(race)),
        Granter::Choice(choice) => format!("a choice made for {}", words_of_id(choice)),
    }
}

fn family_key(family: &ProseFamily) -> String {
    format!("{family:?}")
}

fn family_name(family: &ProseFamily) -> &'static str {
    match family {
        ProseFamily::Desc => "Desc",
        ProseFamily::Benefit => "Benefit",
        ProseFamily::Special => "Special",
        ProseFamily::Aspect(_) => "Aspect",
        ProseFamily::StatBlock(_) => "StatBlock",
        ProseFamily::WhenActive => "WhenActive",
    }
}

fn rank(family: &ProseFamily) -> i32 {
    match family {
        ProseFamily::Desc => 0,
        ProseFamily::Benefit => 1,
        ProseFamily::Special => 2,
        ProseFamily::Aspect(_) => 3,
        ProseFamily::StatBlock(_) => 4,
        ProseFamily::WhenActive => 5,
    }
}

fn render(
    package: &SheetRulePackage,
    segments: &[ProseSegment],
    families: Option<&[&str]>,
) -> String {
    let included: Vec<bool> = segments
        .iter()
        .map(|s| families.is_none_or(|f| f.contains(&family_name(&s.family))))
        .collect();
    // `pick_last` means "print only the LAST segment of this family whose gate passes". With no
    // character no gate is evaluated, so every gate is treated as passing and the last segment
    // of the family is the one that prints -- the same segment the evaluator picks for a
    // character who meets every condition.
    let mut last_pick: BTreeMap<String, usize> = BTreeMap::new();
    for (i, s) in segments.iter().enumerate() {
        if s.pick_last && included[i] {
            last_pick.insert(family_key(&s.family), i);
        }
    }

    let mut lines: Vec<(i32, usize, String)> = Vec::new();
    for (i, s) in segments.iter().enumerate() {
        if !included[i] {
            continue;
        }
        if s.pick_last && last_pick.get(&family_key(&s.family)) != Some(&i) {
            continue;
        }
        let mut text = String::new();
        let mut slots = 0usize;
        let mut nonzero = false;
        for p in &s.pieces {
            match p {
                ProsePiece::Text(t) => text.push_str(t),
                ProsePiece::Slot(e) => {
                    slots += 1;
                    match const_value(e) {
                        Some(n) => {
                            nonzero |= n != 0;
                            text.push_str(&n.to_string());
                        }
                        None => {
                            // Not settled without a character: the rule's words, never `0`.
                            nonzero = true;
                            text.push_str(&expr_words(package, e));
                        }
                    }
                }
                ProsePiece::ChoiceName(id) => {
                    text.push_str(&format!("your choice of {}", words_of_id(id)));
                }
                ProsePiece::Dice { dice, modifier } => match modifier.as_ref().map(const_value) {
                    None => text.push_str(dice),
                    Some(Some(n)) => text.push_str(&fold_dice_modifier(dice, n)),
                    Some(None) => {
                        let words = expr_words(package, modifier.as_ref().expect("matched Some"));
                        text.push_str(&format!("{dice} plus {words}"));
                    }
                },
            }
        }
        // The evaluator drops a line whose every slot came out zero. Here a slot is only known
        // to be zero when it is a constant zero; a slot standing on a character term is words,
        // and words are never suppressed.
        if s.suppress_when_all_zero && slots > 0 && !nonzero {
            continue;
        }
        let text = text.trim().to_string();
        if text.is_empty() {
            continue;
        }
        let text = match &s.applies {
            Some(gate) => {
                let condition = describe_gate(package, gate);
                if condition.is_empty() || condition == "no prerequisite" {
                    text
                } else {
                    format!("If {condition}: {text}")
                }
            }
            None => text,
        };
        let line = match &s.family {
            ProseFamily::Aspect(label) | ProseFamily::StatBlock(label) => format!("{label}: {text}"),
            ProseFamily::WhenActive => format!("When active: {text}"),
            _ => text,
        };
        if !lines.iter().any(|(_, _, l)| *l == line) {
            lines.push((rank(&s.family), i, line));
        }
    }
    lines.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    lines.into_iter().map(|(_, _, l)| l).collect::<Vec<_>>().join("\n")
}

/// The expression's value when **nothing about a character** is needed to know it.
///
/// Structural, not an evaluation against empty facts: `Expr::Level` against empty facts is `0`,
/// which is a wrong number wearing a right number's clothes. Only the arithmetic whose leaves
/// are all `Const` folds; every other leaf returns `None` and renders as words.
///
/// Division and the rounding wrappers follow the evaluator's own rule -- exact rational
/// arithmetic with ONE truncation at the boundary -- by folding in `i64` and refusing a
/// division that is not exact, so a folded constant can never disagree with the number the
/// sheet would print.
fn const_value(e: &Expr) -> Option<i64> {
    match e {
        Expr::Const(n) => Some(i64::from(*n)),
        Expr::Sum(terms) => terms.iter().try_fold(0i64, |acc, t| acc.checked_add(const_value(t)?)),
        Expr::Mul(a, b) => const_value(a)?.checked_mul(const_value(b)?),
        Expr::Div(a, b) => {
            let (a, b) = (const_value(a)?, const_value(b)?);
            if b != 0 && a % b == 0 {
                Some(a / b)
            } else {
                None
            }
        }
        Expr::Min(a, b) => Some(const_value(a)?.min(const_value(b)?)),
        Expr::Max(a, b) => Some(const_value(a)?.max(const_value(b)?)),
        Expr::Floor(inner) | Expr::Ceil(inner) => const_value(inner),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::sheet_rule::{
        evaluate, Ability, Applies, CharacterFacts, ClassRef, EvalContext, HeldSet, Provenance,
        SheetValue, Subject,
    };
    use crate::rules_core::sheet_rule::{BonusTarget, Choice, Effect, Fact, OptionSet};

    fn rule_with(prose: Vec<ProseSegment>) -> SheetRule {
        SheetRule {
            id: "core_rulebook:spell:test_rule".to_string(),
            label: "Test Rule".to_string(),
            value: SheetValue::Text,
            also: Vec::new(),
            prose,
            applies: Applies::Always,
            target: None,
            bonus_type: None,
            print: true,
            pool: "spell".to_string(),
            tags: Vec::new(),
            subject: Subject::Character,
            repeatable: false,
            granted_by: Vec::new(),
            offers: None,
            grants: Vec::new(),
            provenance: Provenance::default(),
        }
    }

    fn segment(family: ProseFamily, pieces: Vec<ProsePiece>) -> ProseSegment {
        ProseSegment { family, pieces, applies: None, pick_last: false, suppress_when_all_zero: false }
    }

    fn package_with(rule: SheetRule) -> SheetRulePackage {
        let mut p = SheetRulePackage::new();
        p.insert_rule(rule);
        p.finish();
        p
    }

    /// The defect this module exists to fix, stated as a test: the sheet evaluator with no
    /// character prints the ARG shape as `0`, and a catalog screen has no character.
    #[test]
    fn an_unsettled_slot_prints_the_rules_words_where_the_evaluator_prints_zero() {
        let rule = rule_with(vec![segment(
            ProseFamily::Desc,
            vec![
                ProsePiece::Text("contained within you for up to ".into()),
                ProsePiece::Slot(Expr::CasterLevel(ClassRef::Holder)),
                ProsePiece::Text(" rounds".into()),
            ],
        )]);
        let package = package_with(rule.clone());

        let line = evaluate(
            &rule,
            &HeldSet::default(),
            &package,
            &CharacterFacts::default(),
            EvalContext::default(),
        );
        assert_eq!(
            line.prose, "contained within you for up to 0 rounds",
            "the evaluator's characterless answer is the wrong number this module replaces"
        );

        assert_eq!(
            catalog_prose(&package, &rule),
            "contained within you for up to caster level rounds"
        );
    }

    #[test]
    fn a_constant_slot_still_prints_its_number() {
        let rule = rule_with(vec![segment(
            ProseFamily::Desc,
            vec![
                ProsePiece::Text("a DC ".into()),
                ProsePiece::Slot(Expr::Sum(vec![Expr::Const(10), Expr::Const(5)])),
                ProsePiece::Text(" Fortitude save".into()),
            ],
        )]);
        let package = package_with(rule.clone());
        assert_eq!(catalog_prose(&package, &rule), "a DC 15 Fortitude save");
    }

    #[test]
    fn a_slot_mixing_a_constant_and_a_character_term_is_words() {
        let rule = rule_with(vec![segment(
            ProseFamily::Desc,
            vec![
                ProsePiece::Text("DC ".into()),
                ProsePiece::Slot(Expr::Sum(vec![
                    Expr::Const(10),
                    Expr::AbilityMod(Ability::Cha),
                ])),
            ],
        )]);
        let package = package_with(rule.clone());
        assert_eq!(catalog_prose(&package, &rule), "DC 10 plus Charisma modifier");
    }

    #[test]
    fn dice_stay_dice_and_a_constant_modifier_folds() {
        let rule = rule_with(vec![segment(
            ProseFamily::Desc,
            vec![ProsePiece::Dice { dice: "1d8".into(), modifier: Some(Expr::Const(2)) }],
        )]);
        let package = package_with(rule.clone());
        assert_eq!(catalog_prose(&package, &rule), "1d8+2");
    }

    #[test]
    fn a_dice_modifier_on_a_character_term_reads_as_words_beside_the_dice() {
        let rule = rule_with(vec![segment(
            ProseFamily::Desc,
            vec![ProsePiece::Dice {
                dice: "1d6".into(),
                modifier: Some(Expr::AbilityMod(Ability::Str)),
            }],
        )]);
        let package = package_with(rule.clone());
        assert_eq!(catalog_prose(&package, &rule), "1d6 plus Strength modifier");
    }

    #[test]
    fn a_gated_segment_prints_its_condition_in_words() {
        let mut seg = segment(ProseFamily::Desc, vec![ProsePiece::Text("you fly".into())]);
        seg.applies = Some(Applies::Compare {
            lhs: Expr::Level,
            op: crate::rules_core::sheet_rule::Cmp::Gte,
            rhs: Expr::Const(5),
        });
        let rule = rule_with(vec![seg]);
        let package = package_with(rule.clone());
        assert_eq!(
            catalog_prose(&package, &rule),
            "If character level at least 5: you fly"
        );
    }

    #[test]
    fn the_description_families_exclude_the_stat_block() {
        let rule = rule_with(vec![
            segment(ProseFamily::Desc, vec![ProsePiece::Text("the words".into())]),
            segment(
                ProseFamily::StatBlock("Casting time".into()),
                vec![ProsePiece::Text("1 standard action".into())],
            ),
        ]);
        let package = package_with(rule.clone());
        assert_eq!(catalog_description(&package, &rule).as_deref(), Some("the words"));
        assert_eq!(
            catalog_prose(&package, &rule),
            "the words\nCasting time: 1 standard action"
        );
    }

    #[test]
    fn a_record_with_no_descriptive_prose_serves_no_description() {
        let rule = rule_with(vec![segment(
            ProseFamily::StatBlock("Casting time".into()),
            vec![ProsePiece::Text("1 standard action".into())],
        )]);
        let package = package_with(rule.clone());
        assert_eq!(catalog_description(&package, &rule), None);
    }

    #[test]
    fn lookup_is_by_kind_and_the_names_slug() {
        let rule = rule_with(vec![segment(
            ProseFamily::Desc,
            vec![ProsePiece::Text("the words".into())],
        )]);
        let package = package_with(rule);
        assert_eq!(
            catalog_description_by_name(&package, "spell", "Test Rule").as_deref(),
            Some("the words")
        );
        assert_eq!(catalog_description_by_name(&package, "spell", "No Such Rule"), None);
        assert_eq!(catalog_description_by_name(&package, "feat", "Test Rule"), None);
    }

    /// The corpus-wide gate, over the live `data/sheet_rules/` directory rather than a fixture
    /// (`decisions.md §4`: a per-kind gate that reads the live corpus, never a per-unit fixture
    /// with a hand-derived value).
    ///
    /// Every rule whose prose carries a slot that **no character settles** is rendered both
    /// ways: through the sheet evaluator with empty facts, and through this module. The
    /// evaluator's answer for such a slot is a number it has no character to compute -- `0` --
    /// and this module's is the term's own words. The two must differ for every one of them,
    /// which is the property a catalog screen depends on and the one a presence check could
    /// never see (`presence-gates-vs-correctness-gates`: a wrong computed number looks right).
    ///
    /// The population is reported, so a cycle that changes the converter sees the figure move.
    #[test]
    fn every_unsettled_slot_in_the_live_package_renders_as_words_not_as_the_characterless_zero() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data/sheet_rules");
        if !dir.is_dir() {
            panic!("{} is not a directory -- regenerate with `cargo run --locked --bin sheet_rule_convert`", dir.display());
        }
        let load = crate::rules_core::corpus_loader::load_sheet_rules(&dir);
        let package = load.package;
        assert!(!package.rules.is_empty(), "the converted package carries no rules");

        let held = HeldSet::default();
        let facts = CharacterFacts::default();
        let mut with_unsettled_slot = 0usize;
        let mut disagreed = 0usize;
        let mut agreed: Vec<String> = Vec::new();
        for rule in package.rules.values() {
            let unsettled = rule.prose.iter().any(|s| {
                s.pieces.iter().any(|p| match p {
                    ProsePiece::Slot(e) => const_value(e).is_none(),
                    ProsePiece::Dice { modifier: Some(e), .. } => const_value(e).is_none(),
                    _ => false,
                })
            });
            if !unsettled {
                continue;
            }
            with_unsettled_slot += 1;
            let sheet = evaluate(rule, &held, &package, &facts, EvalContext::default()).prose;
            let catalog = catalog_prose(&package, rule);
            if sheet == catalog {
                if agreed.len() < 10 {
                    agreed.push(rule.id.clone());
                }
            } else {
                disagreed += 1;
            }
        }
        println!(
            "sheet_rule_catalog live gate: rules={} with_unsettled_slot={with_unsettled_slot} \
             rendered_differently={disagreed}",
            package.rules.len()
        );
        assert!(
            with_unsettled_slot > 0,
            "no converted rule carries an unsettled slot -- this gate is measuring nothing"
        );
        assert!(
            agreed.is_empty(),
            "{} rule(s) render identically with and without a character, so their unsettled \
             slot still reaches a catalog screen as the characterless number; first few: {agreed:?}",
            with_unsettled_slot - disagreed
        );
    }

    /// The defect the field summary exists to fix, stated as a test: a record with no prose at
    /// all had nothing for a catalog screen to print, which is why the desktop reference
    /// library was reading the ingest format's token rows instead.
    #[test]
    fn a_rule_with_no_prose_renders_its_typed_fields_as_words() {
        let mut rule = rule_with(Vec::new());
        rule.tags = vec!["Dexterity".into(), "Base".into()];
        rule.applies = Applies::Compare {
            lhs: Expr::Level,
            op: crate::rules_core::sheet_rule::Cmp::Gte,
            rhs: Expr::Const(3),
        };
        let package = package_with(rule.clone());

        assert_eq!(
            catalog_description(&package, &rule),
            None,
            "the description tier has nothing to serve -- that is the gap"
        );
        assert_eq!(
            catalog_field_summary(&package, &rule).as_deref(),
            Some("Tags: Dexterity, Base; Applies if character level at least 3")
        );
    }

    #[test]
    fn a_field_summary_names_the_sheet_total_and_the_stacking_type() {
        let mut rule = rule_with(Vec::new());
        rule.value = SheetValue::Number(Expr::Const(2));
        rule.target = Some(BonusTarget::Save(crate::rules_core::sheet_rule::Save::Will));
        rule.bonus_type = Some(crate::rules_core::sheet_rule::BonusType {
            name: "Racial".into(),
            mode: crate::rules_core::sheet_rule::StackMode::Stack,
        });
        let package = package_with(rule.clone());
        assert_eq!(
            catalog_field_summary(&package, &rule).as_deref(),
            Some("Value: 2; Adds to Will saves as a Racial bonus")
        );
    }

    /// An unsettled value is words, never the characterless zero -- the same rule the prose
    /// renderer follows, applied to the typed fields.
    #[test]
    fn an_unsettled_value_in_a_field_summary_is_words_not_zero() {
        let mut rule = rule_with(Vec::new());
        rule.value = SheetValue::Number(Expr::CasterLevel(ClassRef::Holder));
        let package = package_with(rule.clone());
        assert_eq!(
            catalog_field_summary(&package, &rule).as_deref(),
            Some("Value: caster level")
        );
    }

    #[test]
    fn a_field_summary_names_the_choice_offered_and_the_facts_granted() {
        let mut rule = rule_with(Vec::new());
        rule.offers = Some(Choice {
            id: "pick_a_language".into(),
            count: Expr::Const(1),
            from: OptionSet::Languages(vec!["Draconic".into(), "Goblin".into()]),
        });
        rule.grants = vec![Effect::FactGrant(Fact::ClassSkill("perception".into()))];
        let package = package_with(rule.clone());
        assert_eq!(
            catalog_field_summary(&package, &rule).as_deref(),
            Some(
                "Offers a choice of 1 from languages: Draconic, Goblin; \
                 Grants perception as a class skill"
            )
        );
    }

    /// A record stating nothing beyond its identity gets `None`, never a fabricated sentence
    /// and never an ingest bookkeeping row dressed up as content.
    #[test]
    fn a_rule_stating_nothing_beyond_its_identity_has_no_field_summary() {
        let rule = rule_with(Vec::new());
        let package = package_with(rule.clone());
        assert_eq!(catalog_field_summary(&package, &rule), None);
    }

    #[test]
    fn the_three_description_tiers_are_tried_in_order_and_are_distinguishable() {
        let prose = rule_with(vec![segment(
            ProseFamily::Desc,
            vec![ProsePiece::Text("the words".into())],
        )]);
        let package = package_with(prose.clone());
        assert_eq!(
            catalog_description_or_fields(&package, &prose),
            Some(CatalogDescription {
                text: "the words".into(),
                tier: DescriptionTier::Prose
            })
        );

        let stat = rule_with(vec![segment(
            ProseFamily::StatBlock("Casting time".into()),
            vec![ProsePiece::Text("1 standard action".into())],
        )]);
        let package = package_with(stat.clone());
        assert_eq!(
            catalog_description_or_fields(&package, &stat),
            Some(CatalogDescription {
                text: "Casting time: 1 standard action".into(),
                tier: DescriptionTier::StatBlock
            })
        );

        let mut fields = rule_with(Vec::new());
        fields.tags = vec!["Spoken".into()];
        let package = package_with(fields.clone());
        assert_eq!(
            catalog_description_or_fields(&package, &fields),
            Some(CatalogDescription {
                text: "Tags: Spoken".into(),
                tier: DescriptionTier::Fields
            })
        );

        let bare = rule_with(Vec::new());
        let package = package_with(bare.clone());
        assert_eq!(catalog_description_or_fields(&package, &bare), None);
    }

    /// The ingest format's own vocabulary, detected by **shape** rather than by a list of
    /// token heads: an all-capitals run of four or more letters immediately followed by `:` or
    /// `=` is a token head, and a `%` followed by a digit or a capital is a substitution
    /// marker. Written this way deliberately — a list of literal token heads in a live-side
    /// file is itself a live-side occurrence of the ingest format, which
    /// `scripts/pcgen_residue_gate.py` counts and `decisions.md §11` forbids. Returns the
    /// offending fragment so a failure names what leaked.
    fn ingest_vocabulary(text: &str) -> Option<String> {
        let bytes: Vec<char> = text.chars().collect();
        let mut i = 0usize;
        while i < bytes.len() {
            if bytes[i] == '%'
                && bytes.get(i + 1).is_some_and(|c| c.is_ascii_digit() || c.is_ascii_uppercase())
            {
                return Some(bytes[i..(i + 2).min(bytes.len())].iter().collect());
            }
            if bytes[i].is_ascii_uppercase() {
                let start = i;
                while i < bytes.len() && bytes[i].is_ascii_uppercase() {
                    i += 1;
                }
                if i - start >= 4 && matches!(bytes.get(i), Some(':') | Some('=')) {
                    return Some(bytes[start..=i].iter().collect());
                }
                continue;
            }
            i += 1;
        }
        None
    }

    /// The corpus-wide gate (`decisions.md §4`: a per-kind gate over the live
    /// `data/sheet_rules/` directory, never a per-unit fixture with a hand-derived value).
    ///
    /// Over the twelve reference-library kinds, every rule the package holds is resolved
    /// through all three tiers, and **no rendered description may carry ingest-format
    /// vocabulary** — the exact property the token dump it replaces could not have. The
    /// population of each tier is printed so a converter change moves a visible figure.
    #[test]
    fn no_reference_library_description_carries_ingest_format_vocabulary() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data/sheet_rules");
        if !dir.is_dir() {
            panic!(
                "{} is not a directory -- regenerate with `cargo run --locked --bin sheet_rule_convert`",
                dir.display()
            );
        }
        let package = crate::rules_core::corpus_loader::load_sheet_rules(&dir).package;
        assert!(!package.rules.is_empty(), "the converted package carries no rules");

        const KINDS: [&str; 12] = [
            "ability",
            "class_generic",
            "deity",
            "domain",
            "feat_generic",
            "language",
            "monster_generic",
            "power",
            "race_generic",
            "skill",
            "template",
            "trait_generic",
        ];

        let (mut prose, mut stat_block, mut fields, mut bare) = (0usize, 0usize, 0usize, 0usize);
        let mut leaks: Vec<String> = Vec::new();
        for rule in package.rules.values() {
            let kind = rule.provenance.kind.as_str();
            // The package's `kind` is the singular corpus directory name for these twelve.
            if !KINDS.contains(&kind) {
                continue;
            }
            match catalog_description_or_fields(&package, rule) {
                None => bare += 1,
                Some(resolved) => {
                    match resolved.tier {
                        DescriptionTier::Prose => prose += 1,
                        DescriptionTier::StatBlock => stat_block += 1,
                        DescriptionTier::Fields => fields += 1,
                    }
                    if let Some(marker) =
                        ingest_vocabulary(&resolved.text).filter(|_| leaks.len() < 10)
                    {
                        leaks.push(format!("{}: `{marker}` in {}", rule.id, resolved.text));
                    }
                }
            }
        }
        println!(
            "reference-library description tiers: prose={prose} stat_block={stat_block} \
             fields={fields} identity_only={bare}"
        );
        assert!(
            fields > 0,
            "no reference-library rule resolved through the field summary -- this gate is \
             measuring nothing"
        );
        assert!(leaks.is_empty(), "ingest-format vocabulary reached a description: {leaks:?}");
    }

    #[test]
    fn const_value_refuses_an_inexact_division_rather_than_rounding_it() {
        assert_eq!(const_value(&Expr::Div(Box::new(Expr::Const(7)), Box::new(Expr::Const(2)))), None);
        assert_eq!(const_value(&Expr::Div(Box::new(Expr::Const(6)), Box::new(Expr::Const(2)))), Some(3));
        assert_eq!(const_value(&Expr::Level), None);
    }
}
