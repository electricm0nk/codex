#[allow(unused_imports)]
pub(crate) use super::*;

/// `AT-34-E3-002` bridge for `v06_work_inventory`'s completion-atlas classifier
/// (`decisions.md §2` bucket C, "held and computed, never surfaced";
/// `decisions.md §12` L1 -- read the code that writes a verdict field before
/// quoting it). [`push_generic_pool_group_selection_magnitude`] has, since
/// SD-32 T12 Epic 8, already computed and explained real corpus
/// `"<group> ~ <member>"` records for six real pools (Cleric Domain,
/// Sorcerer Bloodline, Bloodrager Bloodline, Oracle Mystery, Warpriest
/// Blessing, Shaman Spirit) -- but nothing in `v06_work_inventory`'s
/// classifier has ever asked it a question, so every one of those records
/// reads `engine-does-not-hold` regardless of whether the engine holds it.
///
/// This returns, from a REAL character's own real `ComputationExplanation`s
/// (produced by a real `compute_pilot_base_chassis`/`build_pilot_headless_receipt`
/// run, never a static reflection of which groups the resolver COULD in
/// principle reach), the exact set of real corpus keys
/// (`"<group> ~ <member>"`) whose generic pool-group magnitude was genuinely
/// emitted this run. Read directly from each matching explanation's own
/// `detail` field, which embeds `` corpus key `<key>` `` verbatim
/// (`push_generic_pool_group_selection_magnitude`'s own format string,
/// literally quoted above) -- never reconstructed by guessing the id's own
/// slug scheme, so a future change to that slug format cannot silently
/// desync this bridge from what the engine actually emitted.
pub fn generic_pool_group_selection_observed_keys(
    explanations: &[ComputationExplanation],
    id_prefix: &str,
) -> std::collections::BTreeSet<String> {
    const MARKER: &str = "corpus key `";
    explanations
        .iter()
        .filter(|e| e.id.starts_with(id_prefix))
        .filter_map(|e| {
            let start = e.detail.find(MARKER)? + MARKER.len();
            let end = e.detail[start..].find('`')?;
            Some(e.detail[start..start + end].to_string())
        })
        .collect()
}

/// The sibling of [`push_generic_pool_group_selection_magnitude`] for the DIFFERENT corpus shape
/// cycle 14's own `§16` finding named and refused to force through the wrong resolver (SD-32 T12
/// Epic 8 row 18 cycle 15): a pool member with an EMPTY `bonus_vars` (so `resolve_pool_member_
/// sole_magnitude` correctly refuses it, per that function's own `record.bonus_vars.is_empty()`
/// guard) but a real `%N`-substituted `DESC:` argument that is itself a raw formula EXPRESSION.
/// Never routes through `resolve_pool_member_sole_magnitude` -- a deliberately separate resolver
/// for a deliberately different shape, wired through `class_feature_grant_consumer::resolved_
/// description_for_formula_only_desc_argument` (`pcgen_desc.rs`'s own documented consumer), not
/// the pool resolver.
///
/// `already_hand_modelled_keys`: member keys a caller ALREADY grounds through a dedicated,
/// activation-gated function (e.g. Warpriest's own `Destruction Blessing ~ Destructive Attacks`
/// and `Strength Blessing ~ Strength Surge`) -- skipped here so this purely-additive generic pass
/// never emits a second, un-gated explanation for a magnitude a hand-modelled function already
/// grounds correctly (with activation-state awareness this generic pass has no way to reproduce).
// Same shape as `push_generic_pool_choice_magnitude` above, wider still for
// this variant's own `already_hand_modelled_keys` skip-list parameter --
// see that function's comment for why this stays unbundled.
#[allow(clippy::too_many_arguments)]
pub(super) fn push_generic_pool_group_selection_description_magnitude(
    input: &CharacterInput,
    level: u8,
    ability_modifiers: &AbilityModifiers,
    choice_set_id: &str,
    class: &str,
    registered_name: &str,
    namespace: &str,
    id_prefix: &str,
    min_level: u8,
    already_hand_modelled_keys: &[&str],
    explanations: &mut Vec<ComputationExplanation>,
) {
    if level < min_level {
        return;
    }
    for selection_id in input
        .chosen
        .selected_choices
        .iter()
        .filter(|c| c.choice_set_id == choice_set_id)
        .map(|c| c.selection_id.as_str())
    {
        let Some(slug) = selection_id.strip_prefix(namespace) else { continue };
        let Some(group) = real_pool_group_for_selection_slug(class, registered_name, slug) else {
            continue;
        };
        let prefix = format!("{group} ~ ");
        let group_slug = class_feature_id_slug(&group);
        // SD-32 T12 Epic 8 row 18 cycle 19: mirror `resolve_pool_member_sole_magnitude`'s own
        // FULL header-chain merge (per-group header + Tracker header + bare base header +
        // owning-class record-level `BONUS:VAR` chain -- `pool_group_header_vars_merged`, `§17`)
        // into THIS resolver too. Cycle 18 named the gap: `Mountain Domain ~ Foothold`'s `%1`
        // needs `DomainMountainTimes`, which chains through the domain-kind header cycle 18 added
        // to `DomainPowerTimes`, which is bound only on Cleric Domain's OWN bare `"Domains"`
        // base-header record (`registered_name = "Domain"`) -- the base-header hop this resolver
        // never had. Computed once per group (not per member), never a new lookup mechanism.
        let description_header_vars =
            pool_group_header_vars_merged(class, &group, Some(registered_name));
        for (key, _record) in class_feature_grant_consumer::class_feature_record_tokens_pre_gate_safe()
            .iter()
        {
            let Some(member_name) = key.strip_prefix(&prefix) else { continue };
            if already_hand_modelled_keys.contains(&key.as_str()) {
                continue;
            }
            let Some((description, value)) =
                class_feature_grant_consumer::resolved_description_for_formula_only_desc_argument(
                    key,
                    level,
                    ability_modifiers,
                    &description_header_vars,
                )
            else {
                continue;
            };
            let Ok(value) = i16::try_from(value) else { continue };
            let member_slug = class_feature_id_slug(member_name);
            explanations.push(ComputationExplanation {
                id: format!("{id_prefix}.{group_slug}.{member_slug}.description"),
                value,
                detail: format!(
                    // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                    //   BONUS:VAR token at all, a shape resolve_pool_member_sole_magnitude never
                    //   reaches
                    "{group} member \"{member_name}\" (corpus key `{key}`, real level {level}): \
                     {description} Resolved generically -- not a hand-picked, per-member function -- \
                     through the real PCGen formula interpreter applied DIRECTLY to this member's \
                     own `%N`-substituted description argument text, after \
                     resolving the recorded {choice_set_id} -> {selection_id} selection to its real \
                     corpus group {group} (SD-32 T12 Epic 8 row 18 cycle 15, decisions.md §17 \
                     generic pool-group-selection description resolver)."
                ),
            });
        }
    }
}

/// Resolves a `push_generic_pool_group_selection_magnitude` selection's
/// bare slug (e.g. `"air"`, `"draconic"`) to the real corpus group name it
/// names (e.g. `"Air Domain"`, `"Draconic Bloodline"`), generically: scans
/// every `class_feature` record's own `" ~ "`-qualified group, tallies
/// each group's OWNER by corpus-read majority `class` across its own
/// member records (never assumed from text -- the same guard
/// `census_class_feature_pool_group_names.py` and
/// `pool_header_record_by_normalized_suffix` both use), keeps only groups
/// majority-owned by the requested `class`, matches the group's own
/// trailing word-boundary suffix against `registered_name` (exact, or
/// after stripping one trailing `s` from each side -- singular/plural
/// insensitive, identical to `pool_header_record_by_normalized_suffix`'s
/// own rule), and returns the one whose remaining leading adjective slugs
/// (via `class_feature_id_slug`) to exactly `slug`. `None` for an invented
/// or unrecognised slug -- never guessed.
///
/// SD-32 T12 Epic 8 row 18 cycle 6: a SECOND real corpus naming shape,
/// confirmed by direct inspection of Bloodrager's own 12 real Bloodline
/// groups (e.g. `"Undead Bloodrager Bloodline"`, `"Aberrant Bloodrager
/// Bloodline"`) -- unlike Sorcerer's plain `"<Adjective> Bloodline"`, the
/// owner CLASS NAME is baked into the group name itself as a middle word,
/// while the real recorded selection id this codebase already establishes
/// (`ARCANE_BLOODRAGER_BLOODLINE_SELECTION = "bloodline:arcane"`, not
/// `"bloodline:arcane_bloodrager"`) still names only the bare adjective.
/// After the registered-name suffix strip above, if what remains ALSO ends
/// with `" <class>"` (a word-boundary suffix, e.g. `"Undead Bloodrager"`
/// ending in `" Bloodrager"`), that trailing class-name word is stripped
/// too before slugging -- safe because it only ever fires when the owner
/// class's own name is literally present as a trailing word, never
/// over-stripping an adjective that merely happens to share a prefix.
pub(super) fn real_pool_group_for_selection_slug(
    class: &str,
    registered_name: &str,
    slug: &str,
) -> Option<String> {
    let table = class_feature_grant_consumer::class_feature_record_tokens_pre_gate_safe();
    let mut owner_tally: std::collections::BTreeMap<String, std::collections::BTreeMap<String, u32>> =
        std::collections::BTreeMap::new();
    for (key, record) in table.iter() {
        if let Some((group, _member)) = key.split_once(" ~ ") {
            *owner_tally.entry(group.to_string()).or_default().entry(record.class.clone()).or_default() +=
                1;
        }
    }
    let normalized_registered = registered_name.trim_end_matches('s');
    owner_tally.into_iter().find_map(|(group, owners)| {
        let majority_class = owners.iter().max_by_key(|(_, count)| **count).map(|(c, _)| c.clone())?;
        // SD-32 T12 Epic 8 row 18 cycle 7: a THIRD real corpus ownership shape, confirmed by
        // direct inspection of Cavalier's own "Order of the X" family --
        // `data/corpus/advanced_class_guide/class_feature/order_of_the_blue_rose/*.json` and
        // three siblings (Green, Seal, Tome) tag every one of their own members' `class` field
        // with the ORDER'S OWN NAME (e.g. `"Order of the Blue Rose"`), never `"Cavalier"` --
        // unlike Beast/Guard/Eastern Star/Shroud, whose members are correctly tagged `"Cavalier"`
        // directly. Never trusted from the self-referential tag alone (that would let ANY
        // same-shaped group from an unrelated class or archetype through): only admitted when a
        // real, independently-verified `"<class> <registered_name> ~ <group>"` CHOOSER header
        // record also exists and is itself tagged `class == class` (confirmed live --
        // `"Cavalier Order ~ Order of the Blue Rose"`, `class: "Cavalier"` -- the same header key
        // shape `push_generic_pool_group_selection_magnitude`'s own doc already cites for the
        // Domain/Bloodline/Mystery family, reused here as ownership PROOF, not merely lookup).
        // `"Order of the Sword"` (already hand-modelled, `ORDER_OF_THE_SWORD_SELECTION`) has no
        // such header record and is correctly NOT picked up by this fallback -- no collision.
        let owned_by_class = majority_class == class
            || table
                .get(&format!("{class} {registered_name} ~ {group}"))
                .is_some_and(|header| header.class == class);
        if !owned_by_class {
            return None;
        }
        let adjective = if let Some(adjective) = group.strip_suffix(&format!(" {registered_name}")) {
            adjective
        } else if let Some(adjective) = strip_prefix_case_insensitive(&group, &format!("{registered_name} of the "))
        {
            // SD-32 T12 Epic 8 row 18 cycle 7: the SAME third shape's naming convention --
            // Cavalier's real corpus groups read `"<RegisteredName> of the <Adjective>"`
            // (`"Order of the Beast"`, `"Order Of The Eastern Star"` -- case varies book to book,
            // hence case-insensitive), the mirror image of the `"<Adjective> <RegisteredName>"`
            // shape the first branch above already handles. Tried only after the exact-suffix
            // branch so it changes nothing for any pool already served by that shape.
            adjective
        } else {
            let (last_word_start, last_word) = group
                .rmatch_indices(' ')
                .next()
                .map(|(idx, _)| (idx + 1, &group[idx + 1..]))
                .unwrap_or((0, group.as_str()));
            if last_word.trim_end_matches('s') != normalized_registered {
                return None;
            }
            group[..last_word_start].trim_end()
        };
        // SD-32 T12 Epic 8 row 18 cycle 6: strip a trailing owner-class-name word too, if present
        // (see this function's own doc, "SECOND real corpus naming shape").
        let adjective = adjective.strip_suffix(&format!(" {class}")).unwrap_or(adjective);
        (class_feature_id_slug(adjective) == slug).then(|| group.clone())
    })
}

/// Case-insensitive `str::strip_prefix`, byte-length-based (ASCII only -- every real corpus
/// group name and `registered_name` this codebase passes here is plain ASCII). `None` when
/// `text` is shorter than `prefix` or the leading bytes do not match case-insensitively.
pub(super) fn strip_prefix_case_insensitive<'a>(text: &'a str, prefix: &str) -> Option<&'a str> {
    if text.len() < prefix.len() {
        return None;
    }
    text[..prefix.len()].eq_ignore_ascii_case(prefix).then(|| &text[prefix.len()..])
}

/// `AT-34-E3-001` (`class_feature_owner_matched_by_name_but_record_not_held_
/// by_engine` mechanism, `engine_effect_token_present` sub-cause, cycle 6):
/// grounds a class's own zero-magnitude "Weapon and Armor Proficiency"
/// class feature as a bounded grant-only identity record, mirroring
/// `class_slayer.rs`'s `ground_slayer_weapon_and_armor_proficiency` -- the
/// SAME real archetype-supersession primitive
/// (`archetype_resolver::archetype_claiming_slot_entry`), now shared by
/// three more callers (Cleric, Assassin, Shadowdancer) instead of
/// re-implemented per class.
///
/// **Cleric is the first BASE class this shape covers with a real
/// registered archetype**, and its own proficiency slot carries FOUR
/// distinct spellings across the corpus's own archetype catalog, not
/// Slayer's uniform three -- confirmed by direct corpus grep across all
/// seven tier-1 archetype tables, not assumed from one book:
/// `ClericWeaponProficiencies`/`ClericArmorProficiencies` (ACG's own
/// Ecclesitheurge, whose own "~ Weapon and Armor Proficiency" sub-feature
/// grant this branch reads directly, the same idiom Slayer's Bounty
/// Hunter/Deliverer/Stygian Slayer branch already established) and
/// `ClericWeaponProficiency`/`ClericArmorProficiency` (Ultimate Magic's
/// Sacred Servant and three Ultimate Combat entries, none of which name a
/// "~ Weapon and Armor Proficiency" sub-feature grant of their own, so
/// those fall through to the "superseded, replacement text not resolved"
/// branch exactly as an un-named Slayer archetype claim would).
///
/// **Assassin and Shadowdancer carry no registered archetype in this
/// engine today** (confirmed: `grep -rn 'subject: "Assassin"\|subject:
/// "Shadowdancer"' src/rules_core/rules_tables/*/archetype_tables.rs` --
/// zero matches), so `proficiency_slot_ids` is passed empty for both and
/// the archetype lookup is always a no-op -- never fabricating a slot
/// check that has nothing real to test, while keeping this one function
/// the single place a future archetype landing in the catalog would need
/// to be wired for all three of this shape's callers.
///
/// **`weapon_half_grounded_elsewhere` is honest per class, not assumed
/// true for all three.** Cleric's own Simple-weapon tier is a real,
/// registered `weapon_tables::class_weapon_proficiency("class:cleric")`
/// entry (`rules_tables/crb/weapon_tables.rs:468`) feeding the real -4
/// nonproficiency-attack-penalty check, matching Sorcerer/Wizard/Slayer's
/// own precedent exactly. Assassin and Shadowdancer have **no** entry in
/// that table at all (confirmed by direct grep -- neither class id
/// appears in `weapon_tables.rs`), so for those two this function states
/// plainly that the weapon half's mechanical consequence is NOT grounded
/// elsewhere in this engine, rather than repeating Cleric's claim for a
/// class it is not true of.
// Every parameter is a real, independently-varying input across this
// shape's three callers (class, archetype-lookup, and per-tier proficiency
// booleans) -- see the doc above for why one shared function stays correct
// here instead of three near-duplicates; bundling into a struct is a
// separate refactor out of this clippy-remediation cycle's scope.
#[allow(clippy::too_many_arguments)]
pub(super) fn ground_class_weapon_and_armor_proficiency(
    input: &CharacterInput,
    class_id: &str,
    archetype_subject: &str,
    proficiency_slot_ids: &[&str],
    explanation_id: &str,
    class_name: &str,
    base_desc: &str,
    weapon_half_grounded_elsewhere: bool,
    explanations: &mut Vec<ComputationExplanation>,
) {
    let has_class =
        input.chosen.class_levels.iter().any(|class_level| class_level.class_id == class_id);
    if !has_class {
        return;
    }

    let claimed = proficiency_slot_ids.iter().find_map(|slot| {
        archetype_resolver::archetype_claiming_slot_entry(input, archetype_subject, slot)
    });

    if let Some(entry) = claimed {
        // Supersession branch -- identical shape to `class_slayer.rs`'s own: the
        // archetype's OWN "~ Weapon and Armor Proficiency" sub-feature text is read
        // directly off its real catalog `grants` entry when present, never re-typed
        // by hand a second time; when the catalog entry names no such sub-feature
        // (Sacred Servant and the three Ultimate Combat entries above), the honest
        // "not resolved in this catalog entry" branch applies instead.
        let own_grant = entry
            .grants
            .iter()
            .find(|g| g.grants_feature_key.ends_with("~ Weapon and Armor Proficiency"));
        let detail = match own_grant.and_then(|g| g.description) {
            Some(text) => format!(
                "{class_name} Weapon and Armor Proficiency: superseded by the selected {} \
                 archetype (corpus KEY:{}), which replaces this base-class slot. {}'s own \
                 text: \"{text}\"",
                entry.archetype_name, entry.key, entry.archetype_name
            ),
            None => format!(
                "{class_name} Weapon and Armor Proficiency: superseded by the selected {} \
                 archetype (corpus KEY:{}), which replaces this base-class slot. The base \
                 progression does not apply; {}'s own replacement proficiency text is not \
                 resolved in this catalog entry",
                entry.archetype_name, entry.key, entry.archetype_name
            ),
        };
        explanations.push(ComputationExplanation { id: explanation_id.to_owned(), value: 0, detail });
        return;
    }

    let weapon_half_note = if weapon_half_grounded_elsewhere {
        "The weapon half's real mechanical consequence -- avoiding the -4 nonproficiency \
         attack penalty -- is already grounded separately by \
         `weapon_tables::class_weapon_proficiency`, which this record does not duplicate."
            .to_owned()
    } else {
        "No `weapon_tables::class_weapon_proficiency` entry exists for this class in this \
         engine, so the weapon half's real mechanical consequence -- avoiding the -4 \
         nonproficiency attack penalty -- is NOT grounded elsewhere; this record is the \
         class-features-tab DISPLAY fact only."
            .to_owned()
    };
    explanations.push(ComputationExplanation {
        id: explanation_id.to_owned(),
        value: 0,
        detail: format!(
            "{class_name} Weapon and Armor Proficiency (corpus KEY:{class_name} ~ Weapon and \
             Armor Proficiency): \"{base_desc}\" This is a bounded grant-only identity record \
             (value 0, non-fabricated): the record grants proficiencies automatically and \
             carries no magnitude anywhere. \
             {weapon_half_note} No armor-nonproficiency-penalty mechanic exists anywhere in \
             this engine (the game system's own miscinfo.lst carries only \
             WEAPONNONPROFPENALTY:-4, no armor equivalent), so the armor half has nothing \
             further to compute here"
        ),
    });
}

/// How many Arcane bloodline bonus spells a sorcerer of `level` has been
/// granted: the count of `ARCANE_BLOODLINE_BONUS_SPELLS` rows whose corpus
/// `PREVARGTEQ:BloodlineCasterLVL,<n>` gate has been met.
///
/// 0 below 3rd level, then one more at every odd level through 19th (9 total).
/// Derived from the table rather than from a closed-form `(level-1)/2`
/// expression on purpose: the gate levels are corpus data, and a formula would
/// silently keep "granting" a tenth spell past 19th where the corpus stops.
pub(super) fn arcane_bloodline_bonus_spells_known(level: u8) -> i16 {
    ARCANE_BLOODLINE_BONUS_SPELLS
        .iter()
        .filter(|(grant_level, _, _)| level >= *grant_level)
        .count() as i16
}

/// The Arcane bloodline spells granted at or below `level`, in grant order,
/// each rendered as `"<name> (bloodline spell level <n>, granted at sorcerer
/// level <m>)"` for an explanation record to quote.
pub(super) fn arcane_bloodline_granted_bonus_spells(level: u8) -> Vec<String> {
    ARCANE_BLOODLINE_BONUS_SPELLS
        .iter()
        .filter(|(grant_level, _, _)| level >= *grant_level)
        .map(|(grant_level, spell_level, spell_id)| {
            format!("{spell_id} (spell level {spell_level}, granted at sorcerer level {grant_level})")
        })
        .collect()
}

/// How many bloodline bonus feats a sorcerer of `level` has been granted.
///
/// Corpus chain (cr_abilities_class.lst), followed end to end rather than
/// recalled: the `Arcane Bloodline` `CATEGORY:Sorcerer Bloodline` record carries
/// `BONUS:ABILITYPOOL|Sorcerer Bloodline Feat|BloodlineFeatCount`, and
/// `BONUS:VAR|BloodlineFeatCount|(BloodlineFeatProgression-1)/6|TYPE=Base`, with
/// `BONUS:VAR|BloodlineFeatProgression|BloodlineProgressionLVL|TYPE=Base` and
/// `BONUS:VAR|BloodlineProgressionLVL|SorcererLVL|TYPE=Base`. So the pool is
/// `(sorcerer level - 1) / 6`: 1 at 7th, 2 at 13th, 3 at 19th.
///
/// The corpus also carries three `-1` deductions against this same pool
/// (`PREVARGTEQ:BloodlineFeatProgression,7|PREVAREQ:Sorcerer_CF_BloodlineFeat7,1`
/// and the 13/19 equivalents). Each is gated on a `Sorcerer_CF_BloodlineFeat<n>`
/// flag, which is the standard corpus mechanism for a sorcerer archetype that
/// replaces a bloodline feat; this repo ingests no sorcerer archetype, so all
/// three are provably vacuous here — the identical situation already documented
/// for Cavalier's and Brawler's own bonus-feat pools, and checked here rather
/// than assumed.
///
/// The `BONUS:VAR|BloodlineFeatCount|(DragonDiscipleLVL+1)/3` line is a
/// different class entirely (Dragon Disciple, gated
/// `PREABILITY:1,CATEGORY=Blood of Dragons Bloodline`) and contributes nothing to
/// a single-class sorcerer.
pub(super) fn arcane_bloodline_bonus_feat_count(level: u8) -> i16 {
    (i16::from(level) - 1) / 6
}

/// Metamagic Adept's uses per day at `level` (Arcane bloodline 3rd-level power).
///
/// Corpus (cr_abilities_class.lst, `KEY:Arcane Bloodline ~ Metamagic Adept`):
/// `BONUS:VAR|Sorcerer_ArcaneMetamagicAdept_Times|floor((Sorcerer_Arcane_BloodlinePower3LVL+1)/4)`,
/// where `Sorcerer_Arcane_BloodlinePower3LVL` resolves to the sorcerer's own
/// bloodline level. 1/day at 3rd, rising by one at 7th, 11th, 15th, and 19th.
///
/// Callers must only invoke this at `level >= ARCANE_BLOODLINE_METAMAGIC_ADEPT_LEVEL`
/// and below `ARCANE_BLOODLINE_ARCANE_APOTHEOSIS_LEVEL`: below the grant level the
/// power does not exist, and at 20th Arcane Apotheosis supersedes the per-day
/// budget entirely (see `ground_sorcerer_arcane_bloodline_progression`).
pub(super) fn arcane_bloodline_metamagic_adept_uses_per_day(level: u8) -> i16 {
    (i16::from(level) + 1) / 4
}

/// How many spells New Arcana adds to the sorcerer's spells known at `level`
/// (Arcane bloodline 9th-level power).
///
/// Corpus (cr_abilities_class.lst, `KEY:Arcane Bloodline ~ New Arcana`):
/// `BONUS:VAR|Sorcerer_NewArcana_Number|floor((Sorcerer_Arcane_BloodlinePower9LVL-5)/4)`,
/// feeding `BONUS:ABILITYPOOL|New Arcana|Sorcerer_NewArcana_Number`. One spell at
/// 9th, a second at 13th, a third at 17th, and no fourth by 20th.
///
/// Callers must only invoke this at `level >= ARCANE_BLOODLINE_NEW_ARCANA_LEVEL`:
/// `level - 5` is computed signed and would floor the wrong way below that gate.
pub(super) fn arcane_bloodline_new_arcana_spell_count(level: u8) -> i16 {
    (i16::from(level) - 5) / 4
}

/// Bonus spells per day from a high Intelligence, PF1 Core Rulebook Table:
/// Ability Modifiers and Bonus Spells (the same formula already grounded for
/// Paladin/Charisma, Ranger/Wisdom, Sorcerer/Charisma, and Bard/Charisma in
/// this file): 0 when the modifier is below the spell level, otherwise
/// `(modifier - spell_level) / 4 + 1`. Never applies to cantrips (spell level
/// 0), per the same rule text every other class's own bonus-spell grounding
/// already restricts to spell level 1+.
pub(super) fn ability_bonus_spells(ability_modifier: i16, spell_level: i16) -> i16 {
    if spell_level < 1 || ability_modifier < spell_level {
        0
    } else {
        (ability_modifier - spell_level) / 4 + 1
    }
}

/// This slice's corpus-free `<school>.<level>.<name>` fixture convention,
/// on its own — no `SPELL_LIST` lookup. Split out of
/// [`parse_wizard_spellbook_spell_id`] so [`resolve_prepared_spell_level`]
/// can consult the synthetic form WITHOUT also inheriting a real record's
/// minimum-across-classes `level`.
pub(super) fn parse_synthetic_spell_id(spell_id: &str) -> Option<(Pf1SchoolId, u8)> {
    let mut parts = spell_id.splitn(3, '.');
    let school_token = parts.next()?;
    let level_token = parts.next()?;
    parts.next()?;
    let level: u8 = level_token.parse().ok()?;
    let mut chars = school_token.chars();
    let first = chars.next()?;
    let capitalized: String = first.to_uppercase().chain(chars).collect();
    let school = Pf1SchoolId::from_corpus_str(&capitalized)?;
    Some((school, level))
}

/// How a prepared spell's level resolved for one specific class.
///
/// The distinction is the whole point: a spell whose level is unknown must
/// be **refused**, not skipped. See [`resolve_prepared_spell_level`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum PreparedSpellLevel {
    /// The corpus states this spell's level for this class.
    Known(u8),
    /// No level can be stated without inventing one. Carries the honest
    /// reason, ready to push onto an `unmet` list verbatim.
    Unknown(String),
}

/// A prepared spell's level **for `class_id` specifically**, across every
/// ingested book.
///
/// **The defect this closes (SD-27).** The three prepared-caster gates
/// (Wizard, Arcanist, Warpriest) previously resolved a prepared spell's
/// level through [`parse_wizard_spellbook_spell_id`], which knows only
/// `crb::spell_list::SPELL_LIST`. An APG, ACG or ARG key is absent from
/// that table and carries no dots, so the synthetic fallback failed too and
/// the resolver returned `None` — and both consuming loops were
/// `filter_map`s, so the spell vanished from the accessibility check AND
/// from the slot-consumption count. Measured on the shipped catalog: a
/// Wizard 1 correctly refused 508 of CRB's 652 records and accepted **all**
/// 297 APG, 144 ACG and 92 ARG records at every caster level. A Dwarf
/// Wizard 1 could add `Tsunami` — a 9th-level Wizard spell — and the sheet
/// saved it. An affordance that succeeds where it must refuse is worse than
/// one that refuses wrongly: the player gets an illegal character silently.
///
/// **It also fixes the level itself, not just the coverage.** The old CRB
/// path used the record's own `level`, which is the MINIMUM across every
/// class in its `CLASSES:` token — so a Wizard 1 could prepare
/// `Hideous Laughter` (`CLASSES:Bard=1|Sorcerer,Wizard=2`, record level 1)
/// although a Wizard learns it at 2. This resolves through
/// `rules_tables::class_spell_levels`, which holds the per-class answer for
/// all four books.
///
/// **Unknown is a refusal, never a default.** A spell with no stated level
/// for this class comes back [`PreparedSpellLevel::Unknown`] carrying why;
/// callers push that onto their unmet list. Falling back to the record's
/// minimum level is precisely the wrong number
/// `rules_tables::class_spell_levels` exists to remove, and inventing one
/// would violate `docs/governance/no-stub-mvp-doctrine.md`.
pub(super) fn resolve_prepared_spell_level(class_id: &str, spell_id: &str) -> PreparedSpellLevel {
    if let Some(level) = class_spell_levels::class_spell_level(class_id, spell_id) {
        return PreparedSpellLevel::Known(level);
    }
    // The bounded synthetic `<school>.<level>.<name>` convention this
    // file's own fixtures are written in states its level outright. Read
    // via the dotted parse specifically, never via a real `SPELL_LIST`
    // record's own `level` — that field is the minimum-across-classes
    // value this function exists to stop trusting.
    if let Some((_, level)) = parse_synthetic_spell_id(spell_id) {
        return PreparedSpellLevel::Known(level);
    }
    if !class_spell_levels::class_has_spell_list(class_id) {
        return PreparedSpellLevel::Unknown(format!(
            "prepared spell '{spell_id}' cannot be checked: no spell list for '{class_id}' has \
             been ingested, so no spell level is known for that class"
        ));
    }
    PreparedSpellLevel::Unknown(format!(
        "prepared spell '{spell_id}' has no '{class_id}' spell level in any ingested book, so \
         the spell level it would occupy is unknown"
    ))
}

/// Why a spell **recorded** under `class_id` (`AcquisitionMode::Known`) is
/// not a spell that class can ever hold — or `None` when it is legitimate.
///
/// **This is the Known rule, and it is deliberately not the Prepared rule.**
/// [`resolve_prepared_spell_level`] answers "which slot would this occupy
/// today", and its callers then check that slot against the caster's own
/// level and budget. Recording has no such budget: PF1 CRB *Spellbooks*
/// caps only the two free spells gained at each new level ("of spell levels
/// he can cast"); *Spells Copied from Another's Spellbook or a Scroll*
/// places **no character-level restriction** on what a wizard may add to
/// her book. A spellbook is a record, not a set of castable options, so a
/// Wizard 1 may legitimately hold a 9th-level wizard spell she cannot yet
/// prepare.
///
/// What *is* capped is membership. A wizard cannot scribe a spell that is
/// on no wizard list — 543 of the desktop spell picker's 1185 records are
/// exactly that (Cleric-only, Druid-only, Bard-only, Alchemist-only), and
/// before this check every one of them was accepted and persisted under
/// `class:wizard`. Sorcerer already refused its own equivalent
/// (`unmet_sorcerer_known_spell_conditions`); wizard did not, and the
/// asymmetry was the defect.
///
/// **Absence is reported, never invented.** A class with no ingested spell
/// list yields `None` — refusing every spell for a class this crate simply
/// has no list for would be a fabricated rule, the mirror image of the
/// fabricated level `class_spell_levels` exists to remove (see
/// `docs/governance/no-stub-mvp-doctrine.md`).
pub(super) fn class_spell_membership_refusal(class_id: &str, spell_id: &str) -> Option<String> {
    if class_spell_levels::class_spell_level(class_id, spell_id).is_some() {
        return None;
    }
    // Same bounded synthetic `<school>.<level>.<name>` fixture convention
    // `resolve_prepared_spell_level` honours — those ids belong to no class
    // list by construction and state their own level.
    if parse_synthetic_spell_id(spell_id).is_some() {
        return None;
    }
    if !class_spell_levels::class_has_spell_list(class_id) {
        return None;
    }
    let class_name = class_id.strip_prefix("class:").unwrap_or(class_id);
    Some(format!(
        "recorded spell '{spell_id}' is not on the {class_name} spell list in any ingested \
         book, so no {class_name} can learn it. Recording a spell is not level-gated (PF1 \
         CRB, Spellbooks: a spell copied from a scroll or another spellbook has no \
         character-level restriction) — but the spell still has to be one of that class's"
    ))
}

/// The school a prepared spell belongs to, across every ingested book.
///
/// Only Wizard needs this (its opposed-school slots cost 2 each); Arcanist
/// and Warpriest have no school mechanic. `None` when no ingested record
/// names a school — APG has 16 such records, and the caller charges the
/// ordinary 1-slot cost rather than inventing an opposed school.
pub(super) fn resolve_prepared_spell_school(spell_id: &str) -> Option<Pf1SchoolId> {
    if let Some(entry) = SPELL_LIST.iter().find(|entry| entry.key == spell_id) {
        return Some(entry.school);
    }
    if let Some(entry) = apg::spell_list::SPELL_LIST
        .iter()
        .find(|entry| entry.key == spell_id)
    {
        return entry
            .school
            .and_then(|school| Pf1SchoolId::from_corpus_str(&format!("{school:?}")));
    }
    if let Some(entry) = acg::spell_list::SPELL_LIST
        .iter()
        .find(|entry| entry.key == spell_id)
    {
        return Pf1SchoolId::from_corpus_str(&format!("{:?}", entry.school));
    }
    if let Some(entry) = advanced_race_guide::spell_list::SPELL_LIST
        .iter()
        .find(|entry| entry.key == spell_id)
    {
        return Pf1SchoolId::from_corpus_str(&format!("{:?}", entry.school));
    }
    parse_wizard_spellbook_spell_id(spell_id).map(|(school, _)| school)
}

/// Whether `input` is a Cleric OR an Inquisitor actively, validly using
/// Touch of Good on herself right now, and if so, the sacred bonus to
/// apply (v0.6 alpha swarm, risks item 8; generalized task #64 from
/// Cleric-only to every real base class whose own domain-choice class
/// feature genuinely grants the chosen domain's powers -- see
/// `TOUCH_OF_GOOD_ABILITY_ID`'s own doc comment for exactly which classes
/// qualify, and why Druid is deliberately excluded despite also having a
/// domain-choice class feature). Class-ownership-gated by construction:
/// only returns `Some` when `class_levels` contains Cleric or Inquisitor
/// AND that class's own domain-choice seam recognizes the Good domain as
/// chosen (mirrors the Barbarian Rage / Bard Inspire Courage spoofed-
/// activation shape exactly) -- a character with neither class, or one of
/// these two classes without Good domain chosen, never has this bonus
/// applied regardless of any stray `class_ability_activations` entry.
/// Every real class here feeds the same DomainLVL variable from its own
/// bare class level with no per-class offset (verified against the corpus
/// this cycle), so `cleric_touch_of_good_bonus` reuses unmodified.
pub(super) fn active_touch_of_good_bonus(input: &CharacterInput) -> Option<i16> {
    let (level, domain_choice_id) = input.chosen.class_levels.iter().find_map(|class_level| {
        if class_level.class_id == CLERIC_CLASS_ID {
            Some((class_level.level, CLERIC_DOMAIN_CHOICE_ID))
        } else if class_level.class_id == INQUISITOR_CLASS_ID {
            Some((class_level.level, INQUISITOR_DOMAIN_CHOICE_ID))
        } else {
            None
        }
    })?;

    let domain_selections: Vec<&str> = input
        .chosen
        .selected_choices
        .iter()
        .filter(|c| c.choice_set_id == domain_choice_id)
        .map(|c| c.selection_id.as_str())
        .collect();
    if !domain_selections.contains(&GOOD_DOMAIN_SELECTION) {
        return None;
    }

    let activation = input
        .chosen
        .class_ability_activations
        .iter()
        .find(|activation| activation.ability_id == TOUCH_OF_GOOD_ABILITY_ID)?;
    if activation.active_state != ActiveState::EquippedActive {
        return None;
    }

    Some(cleric_touch_of_good_bonus(level))
}

/// Compute total saving throws as the grounded Fighter level 1–3 base save plus the
/// relevant ability modifier, or block the claim if a supported Fighter chassis
/// (levels 1–3) is absent.
///
/// Adds the single ability modifier each save uses (Fortitude/CON, Reflex/DEX,
/// Will/WIS), plus (v0.6 alpha swarm item 17 widening, 2026-07-24) any grounded
/// feat-derived save bonus from `feat_effects::save_bonuses_from_feats`
/// (currently Great Fortitude/Iron Will/Lightning Reflexes's real flat +2
/// each) -- no corpus access needed for either, so both stay fully inside
/// this headless, claim-gated computation rather than needing the
/// non-claim-gated DTO-enrichment pattern item 1's AC/attack-bonus/ACP work
/// used (that pattern exists specifically for corpus-blocked math; feats are
/// headless-accessible, like `selected_feats` always has been). Still does
/// not add item- or condition-based save modifiers.
pub(super) fn compute_total_saves(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    base_saves: &BaseSaves,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) -> BaseSaves {
    // SD-21 E6.26: widened from a Fighter-only gate to also accept the newly
    // dispatch-supported Wizard chassis (`has_supported_class_chassis` mirrors
    // `compute_class_chassis`'s own single-class dispatch set) — the base saves this
    // function folds ability modifiers into are only genuinely computed, not
    // fabricated, for those two classes so far.
    if !has_supported_class_chassis(input) {
        diagnostics.push(ComputationDiagnostic {
            id: "defense.total_save.unsupported".to_owned(),
            message: format!(
                "total saving throws are only computed from a grounded base-save chassis this \
                 engine currently supports ({}); chosen class levels {:?} do not provide them, \
                 so no total saves were computed",
                supported_class_chassis_description(),
                input.chosen.class_levels
            ),
            claim_blocking: true,
        });
        return BaseSaves::default();
    }

    let feat_save_bonuses =
        crate::rules_core::feat_effects::save_bonuses_from_feats(&effective_character_feats(input));
    // v0.6 alpha swarm, risks item 8: Barbarian Rage's Will-save morale
    // bonus layers on here, the same shape as `feat_save_bonuses` --
    // class-ownership-gated by `active_barbarian_rage_bonus` construction,
    // 0 for every non-Barbarian or not-currently-raging character.
    let rage_will_bonus = active_barbarian_rage_bonus(input, ability_modifiers)
        .map(|(_, _, _, will_save_bonus, _)| will_save_bonus)
        .unwrap_or(0);
    // v0.6 alpha swarm, risks item 8 (first APG/ACG closure): Skald Inspired
    // Rage's Will-save morale bonus layers on here too, the same shape as
    // Barbarian's -- class-ownership-gated by
    // `active_skald_inspired_rage_bonus` construction, 0 for every non-Skald
    // or not-currently-singing character.
    let inspired_rage_will_bonus = active_skald_inspired_rage_bonus(input, ability_modifiers)
        .map(|(_, _, will_save_bonus)| will_save_bonus)
        .unwrap_or(0);
    // v0.6 alpha swarm, risks item 8 (second APG/ACG closure): Bloodrager
    // Bloodrage's Will-save morale bonus layers on here too, the same
    // shape as Barbarian's/Skald's -- class-ownership-gated by
    // `active_bloodrager_bloodrage_bonus` construction, 0 for every
    // non-Bloodrager or not-currently-bloodraging character.
    let bloodrage_will_bonus = active_bloodrager_bloodrage_bonus(input, ability_modifiers)
        .map(|(_, _, will_save_bonus)| will_save_bonus)
        .unwrap_or(0);
    // v0.6 alpha swarm, risks item 8: Cleric Good domain's Touch of Good
    // sacred bonus (self-application only) applies to all three saves --
    // class-ownership-gated by `active_touch_of_good_bonus`
    // construction, 0 for every non-Cleric, non-Good-domain, or
    // not-currently-active character.
    let touch_of_good_save_bonus = active_touch_of_good_bonus(input).unwrap_or(0);
    // v0.6 alpha swarm, risks item 8 (Inquisitor Judgment closure, widened
    // 2026-07-26): Inquisitor Purity judgment's sacred (or profane) bonus
    // on all saving throws applies here too, the same shape as Cleric's
    // Touch of Good -- class-ownership-gated by
    // `active_inquisitor_purity_judgment_bonus` construction, 0 for every
    // non-Inquisitor, non-Purity-judgment, or not-currently-active
    // character.
    let purity_judgment_save_bonus = active_inquisitor_purity_judgment_bonus(input)
        .map(|(_, bonus)| bonus)
        .unwrap_or(0);
    // v0.6 alpha swarm, risks item 8 (Oracle revelation deepening,
    // 2026-07-26, task #10): Lore Mystery's Sidestep Secret lets Charisma
    // stand in for Dexterity on Reflex saves. Integrated rather than
    // grounded standalone because it is always on with no activation and
    // no per-day budget -- class-ownership-gated AND explicit-revelation-
    // gated by `active_oracle_sidestep_secret_reflex_bonus` construction,
    // 0 for every non-Oracle and for any Oracle who did not take it.
    // Applies to REFLEX ONLY, unlike the all-three-saves bonuses above.
    let sidestep_secret_reflex_bonus =
        active_oracle_sidestep_secret_reflex_bonus(input, ability_modifiers).unwrap_or(0);
    // SD-27 (alternate racial traits reach compute), widened by SD-29's
    // race-trait lane round 3 (`decisions.md §47`). Every alternate racial
    // trait whose corpus chain declares a plain-integer `BONUS:SAVE` on a save
    // this engine totals. Race-gated and selection-gated by
    // `alternate_trait_save_bonuses` construction, all-zero for every character
    // who took none of them. This replaced a single hardcoded Half-Elf
    // constant, which had silently become wrong: two later books' alternates
    // land here too, and were being offered while moving nothing.
    let alternate_trait_saves = alternate_trait_save_bonuses(input);
    // AT-34-E4-002 (fourth slice): `ultimate_campaign`'s two flat
    // `BONUS:SAVE` character traits (Life of Toil/Fortitude, Indomitable
    // Faith/Will) layer on here too, the same shape as `feat_save_bonuses`
    // above -- selection-gated by `trait_effects::save_bonuses_from_traits`
    // construction (never `alternate_trait_saves`'s race-locked mechanism;
    // these are plain `CharacterInput.chosen.selected_traits` entries any
    // race can take), all-zero for every character who selected neither.
    let character_trait_saves =
        crate::rules_core::trait_effects::save_bonuses_from_traits(&input.chosen.selected_traits);
    let total_saves = BaseSaves {
        fortitude: base_saves.fortitude
            + ability_modifiers.constitution
            + feat_save_bonuses.fortitude
            + touch_of_good_save_bonus
            + purity_judgment_save_bonus
            + alternate_trait_saves.fortitude
            + character_trait_saves.fortitude,
        reflex: base_saves.reflex
            + ability_modifiers.dexterity
            + feat_save_bonuses.reflex
            + touch_of_good_save_bonus
            + purity_judgment_save_bonus
            + sidestep_secret_reflex_bonus
            + alternate_trait_saves.reflex
            + character_trait_saves.reflex,
        will: base_saves.will
            + ability_modifiers.wisdom
            + feat_save_bonuses.will
            + rage_will_bonus
            + inspired_rage_will_bonus
            + bloodrage_will_bonus
            + touch_of_good_save_bonus
            + purity_judgment_save_bonus
            + alternate_trait_saves.will
            + character_trait_saves.will,
    };

    let class_label = class_summary_label(input);
    explanations.push(ComputationExplanation {
        id: "defense.total_save.fortitude".to_owned(),
        value: total_saves.fortitude,
        detail: format!(
            "Total Fortitude save: {class_label} base Fortitude save (+{}) + Constitution modifier \
             (+{}) + feat bonus (+{}, Great Fortitude if selected) + Good domain Touch of Good sacred \
             bonus (+{}, self-applied) + Inquisitor Purity judgment sacred/profane bonus (+{}, \
             only while actively, validly judging Purity) + alternate racial trait (+{}, only \
             for a character who took one that declares a Fortitude bonus) + character trait \
             (+{}, Life of Toil if selected) = {}",
            base_saves.fortitude, ability_modifiers.constitution, feat_save_bonuses.fortitude,
            touch_of_good_save_bonus, purity_judgment_save_bonus,
            alternate_trait_saves.fortitude, character_trait_saves.fortitude, total_saves.fortitude
        ),
    });
    explanations.push(ComputationExplanation {
        id: "defense.total_save.reflex".to_owned(),
        value: total_saves.reflex,
        detail: format!(
            "Total Reflex save: {class_label} base Reflex save (+{}) + Dexterity modifier (+{}) + \
             feat bonus (+{}, Lightning Reflexes if selected) + Good domain Touch of Good sacred bonus \
             (+{}, self-applied) + Inquisitor Purity judgment sacred/profane bonus (+{}, only \
             while actively, validly judging Purity) + Oracle Sidestep Secret Charisma-for-\
             Dexterity substitution (+{sidestep_secret_reflex_bonus}, only for a Lore-Mystery \
             Oracle who took that revelation) + alternate racial trait ({:+}, only for a \
             character who took one that declares a Reflex modifier -- this one can be \
             NEGATIVE, e.g. Horror Adventures' Half-Elf Mismatched at -2) + character trait \
             (+{}, no ultimate_campaign trait currently declares a Reflex bonus, so always 0) \
             = {}",
            base_saves.reflex, ability_modifiers.dexterity, feat_save_bonuses.reflex,
            touch_of_good_save_bonus, purity_judgment_save_bonus,
            alternate_trait_saves.reflex, character_trait_saves.reflex, total_saves.reflex
        ),
    });
    explanations.push(ComputationExplanation {
        id: "defense.total_save.will".to_owned(),
        value: total_saves.will,
        detail: format!(
            "Total Will save: {class_label} base Will save (+{}) + Wisdom modifier (+{}) + \
             feat bonus (+{}, Iron Will if selected) + Barbarian Rage morale bonus (+{}, only \
             while actively, validly raging) + Skald Inspired Rage morale bonus (+{}, only \
             while actively, validly singing) + Bloodrager Bloodrage morale bonus (+{}, only \
             while actively, validly bloodraging) + Good domain Touch of Good sacred bonus (+{}, \
             self-applied) + Inquisitor Purity judgment sacred/profane bonus (+{}, only while \
             actively, validly judging Purity) + alternate racial trait (+{}, only for a \
             character who took one that declares a Will bonus -- ARG p.42's Half-Elf Dual \
             Minded is the one such trait in the corpus today) + character trait (+{}, \
             Indomitable Faith if selected) = {}",
            base_saves.will,
            ability_modifiers.wisdom,
            feat_save_bonuses.will,
            rage_will_bonus,
            inspired_rage_will_bonus,
            bloodrage_will_bonus,
            touch_of_good_save_bonus,
            purity_judgment_save_bonus,
            alternate_trait_saves.will,
            character_trait_saves.will,
            total_saves.will
        ),
    });

    total_saves
}

// Compute the selected deterministic Climb / Intimidate / Swim skill modifiers,
// or block the claim if the selected-skill or Chain Shirt posture is absent or
// widened beyond this slice.
//
// This is intentionally not a skill engine. It computes only the three
// selected deterministic-posture skills (Climb/Intimidate/Swim -- Fighter
// and Rogue class skills, but NOT Wizard's; see
// `selected_skill_class_skill_bonus_applies`'s own doc comment for the
// v0.6 alpha swarm class-skill-bonus fix) from the accepted deterministic
// rank allocations, applying the already-grounded Chain Shirt armor-check
// penalty to the armor-check skills (Climb, Swim) only. It does not handle
// other skills, arbitrary classes,
// feat/racial/item skill bonuses, encumbrance, or speed-dependent adjustments.
// Any deviation from the exact supported posture is refused with a claim-blocking
// diagnostic and withheld selected-skill explanations rather than fabricated
// totals.
//
// # Class skills (SD-36 Epic F3b3)
//
// Whether Climb / Intimidate / Swim is a class skill is read from each class's CONVERTED
// record ([`class_skill_sheet_rules::class_skill_view`]); see [`selected_skill_class_skill`].

/// Whether one of the three selected skills is a class skill for the character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SelectedClassSkill {
    /// At least one of the character's classes grants it: the +3 applies.
    Yes,
    /// Every class answered, and none grants it.
    No,
    /// No class that answered grants it, and at least one class could not answer (named).
    Unknown(Vec<String>),
}

/// The classes whose converted record the class-skill reader cannot answer, with the verified
/// oracle row for the three selected skills (`[climb, intimidate, swim]`). Each is a named
/// converter remainder: the class line's `ABILITY:Class|AUTOMATIC|<Class>` grant is an
/// unresolved reference (`_defects/unresolved-references.json`, `<book>:class:<slug>: Class|<Class>`),
/// so the walk never reaches the class's own `<Class> ~ Class Skills` record, which the package
/// does carry. A row is consulted ONLY when the reader answers Unknown for the class;
/// `every_fallback_row_is_a_class_the_reader_cannot_answer` fails the day the converter closes
/// the edge, and the row must then be deleted.
const UNREAD_RECORD_SELECTED_CLASS_SKILLS: [(&str, [bool; 3], &str); 9] = [
    // CSKILL:Appraise|TYPE=Craft|Fly|TYPE=Knowledge|Linguistics|TYPE=Profession|Spellcraft|Use Magic Device
    ("class:arcanist", [false, false, false], "acg_abilities_class.lst:65"),
    // CSKILL:Acrobatics|Climb|TYPE=Craft|Escape Artist|Handle Animal|Intimidate|...|Swim
    ("class:brawler", [true, true, true], "acg_abilities_class.lst:891"),
    // CSKILL:Climb|TYPE=Craft|Handle Animal|Heal|Intimidate|...|Survival|Swim
    ("class:hunter", [true, true, true], "acg_abilities_class.lst:1169"),
    // CSKILL:Acrobatics|Appraise|Bluff|Climb|...|Intimidate|...|Use Magic Device (no Swim)
    ("class:investigator", [true, true, false], "acg_abilities_class.lst:1241"),
    // CSKILL:TYPE=Craft|Diplomacy|Fly|Handle Animal|Heal|Knowledge (Nature)|...|Survival
    ("class:shaman", [false, false, false], "acg_abilities_class.lst:1384"),
    // CSKILL:Acrobatics|Appraise|Bluff|Climb|...|Intimidate|...|Swim|Use Magic Device
    ("class:skald", [true, true, true], "acg_abilities_class.lst:1720"),
    // CSKILL:Acrobatics|Bluff|Climb|...|Intimidate|...|Survival|Swim
    ("class:slayer", [true, true, true], "acg_abilities_class.lst:1786"),
    // CSKILL:Acrobatics|Bluff|Climb|...|Intimidate|...|Sleight of Hand|Swim
    ("class:swashbuckler", [true, true, true], "acg_abilities_class.lst:1951"),
    // CSKILL:Climb|TYPE=Craft|Diplomacy|Handle Animal|Heal|Intimidate|...|Survival|Swim
    ("class:warpriest", [true, true, true], "acg_abilities_class.lst:2133"),
];

const SELECTED_SKILLS: [&str; 3] = ["climb", "intimidate", "swim"];

/// The oracle fallback row for `class_id`, when it has one ([`UNREAD_RECORD_SELECTED_CLASS_SKILLS`]).
pub(crate) fn unread_record_selected_class_skills(class_id: &str) -> Option<[bool; 3]> {
    UNREAD_RECORD_SELECTED_CLASS_SKILLS.iter().find(|(id, ..)| *id == class_id).map(|(_, row, _)| *row)
}

/// The class ids the fallback table covers (for the test that retires a row).
#[cfg(test)]
pub(crate) fn unread_record_selected_class_skill_ids() -> impl Iterator<Item = &'static str> {
    UNREAD_RECORD_SELECTED_CLASS_SKILLS.iter().map(|(id, ..)| *id)
}

/// Whether `skill` (`"climb"`, `"intimidate"` or `"swim"`) is a class skill for the character:
/// PF1's union rule (CRB p.87 -- a skill is a class skill when ANY of the character's classes
/// lists it), each class answered by its converted record at its own level
/// ([`class_skill_sheet_rules::class_skill_view`]). One rule for every class. A class the
/// reader cannot answer falls back to its verified oracle row
/// ([`UNREAD_RECORD_SELECTED_CLASS_SKILLS`]) when it has one; otherwise it is Unknown, named, and
/// the +3 is never silently withheld (nor granted).
pub(crate) fn selected_skill_class_skill(input: &CharacterInput, skill: &str) -> SelectedClassSkill {
    let index = SELECTED_SKILLS.iter().position(|s| *s == skill);
    let mut unknown = Vec::new();
    for class_level in &input.chosen.class_levels {
        let slug = class_level.class_id.strip_prefix("class:").unwrap_or(&class_level.class_id);
        let grants = match class_skill_sheet_rules::class_skill_view(slug, class_level.level) {
            class_skill_sheet_rules::ClassSkillAnswer::Known(view) => view.contains(skill),
            class_skill_sheet_rules::ClassSkillAnswer::Unknown { reason } => {
                match (unread_record_selected_class_skills(&class_level.class_id), index) {
                    (Some(row), Some(i)) => row[i],
                    _ => {
                        unknown.push(format!("{} {}: {reason}", class_level.class_id, class_level.level));
                        continue;
                    }
                }
            }
        };
        if grants {
            return SelectedClassSkill::Yes;
        }
    }
    if unknown.is_empty() { SelectedClassSkill::No } else { SelectedClassSkill::Unknown(unknown) }
}

/// The claim-blocking diagnostic id for a selected skill whose class-skill status no class
/// answers.
pub(crate) const SELECTED_SKILL_CLASS_SKILL_UNKNOWN: &str = "skill.selected_modifier.class_skill_unknown";

/// Whether Climb is a class skill for the character (`Yes` only; see
/// [`selected_skill_class_skill`] for the Unknown case callers must name).
pub(crate) fn selected_skill_climb_is_class_skill(input: &CharacterInput) -> bool {
    selected_skill_class_skill(input, "climb") == SelectedClassSkill::Yes
}

/// Whether Intimidate is a class skill for the character (see [`selected_skill_class_skill`]).
pub(crate) fn selected_skill_intimidate_is_class_skill(input: &CharacterInput) -> bool {
    selected_skill_class_skill(input, "intimidate") == SelectedClassSkill::Yes
}

/// Whether Swim is a class skill for the character (see [`selected_skill_class_skill`]).
pub(crate) fn selected_skill_swim_is_class_skill(input: &CharacterInput) -> bool {
    selected_skill_class_skill(input, "swim") == SelectedClassSkill::Yes
}

/// Every selected skill whose class-skill status is Unknown, as `"<skill>: <reasons>"` -- the
/// claim-blocking words both the headless and the corpus path raise.
pub(crate) fn selected_skill_class_skill_unknowns(input: &CharacterInput) -> Vec<String> {
    SELECTED_SKILLS
        .iter()
        .filter_map(|skill| match selected_skill_class_skill(input, skill) {
            SelectedClassSkill::Unknown(reasons) => Some(format!("{skill}: {}", reasons.join("; "))),
            _ => None,
        })
        .collect()
}

/// The `RagePowersLVL` magnitude Raging Climber/Raging Swimmer contribute
/// to `compute_selected_skill_modifiers`'s Climb and Swim totals (task
/// #54), or 0 when the character is not currently, validly raging/singing.
/// Reuses `active_barbarian_rage_bonus`/`active_skald_inspired_rage_bonus`
/// directly rather than re-deriving the rage/Inspired-Rage activation and
/// rounds-budget check a third time, so this can never disagree with
/// `ground_raging_climber_and_swimmer`'s own records (see that function's
/// doc comment for the full corpus citation and the canonical-narrowing
/// scope note). Barbarian is checked first, then Skald -- a character who
/// somehow satisfies both (a Barbarian/Skald multiclass actively raging
/// AND singing at once) is not summed; this mirrors every other
/// class-exclusive rage-shaped bonus in this engine, none of which stacks
/// two sources of the "same" rage magnitude.
pub(super) fn active_rage_powers_level(input: &CharacterInput, ability_modifiers: &AbilityModifiers) -> i16 {
    if let Some((barbarian_level, ..)) = active_barbarian_rage_bonus(input, ability_modifiers) {
        return i16::from(barbarian_level);
    }
    if let Some((skald_level, ..)) = active_skald_inspired_rage_bonus(input, ability_modifiers) {
        return i16::from(skald_level);
    }
    0
}

/// Every feat-derived contribution to a pillar that BOTH compute paths
/// produce independently -- the single place a feat is wired into a number
/// the player reads.
///
/// **Why this type exists.** This engine has two compute twins.
/// `compute_combat_baseline` / `compute_selected_skill_modifiers` in this file
/// compute Armor Class, touch AC and Climb/Intimidate/Swim from hardcoded
/// Chain-Shirt constants; `pilot_compute_corpus`'s
/// `compute_combat_baseline_from_corpus` /
/// `compute_selected_skill_modifiers_from_corpus` compute the same pillars
/// from real corpus-resolved equipment, and *that* pair is what
/// `pf1_adapter::resolve_unified_pilot_snapshot` gates on -- so it is the pair
/// whose numbers reach the sheet.
///
/// Before this seam, each twin read `feat_effects` on its own: this file
/// referenced 33 producers, `pilot_compute_corpus.rs` referenced zero and
/// hand-inlined Dodge. Five feats were therefore wired, tested and green while
/// changing nothing a player could see -- CRB's **Athletic** (+2 Climb/Swim),
/// **Persuasive** (+2 Intimidate) and **Intimidating Prowess** (Strength to
/// Intimidate), and ARG's **Armor of the Pit** (+2 natural armor) and **Sure
/// and Fleet** (+2 Climb). Proven on screen: a live Tiefling Fighter 1 taking
/// Armor of the Pit saw an unchanged Armor Class across a full app restart.
///
/// **The invariant.** Both twins now consume this one struct and neither reads
/// `feat_effects` for these pillars at all, so a feat wired here reaches both
/// paths by construction and cannot reach only one.
/// `pilot_compute_corpus`'s `every_catalog_feat_moves_both_compute_paths_identically`
/// pins that behaviourally over the live 690-record feat catalog, and
/// `the_two_compute_twins_read_feat_effects_only_through_the_shared_seam`
/// pins it structurally, so re-introducing a direct per-path `feat_effects`
/// read fails the build rather than shipping silently.
///
/// Adding the next feat: give it a field here, fold it into the accessor for
/// the pillar it belongs to, and both paths move together. Nothing else needs
/// touching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FeatDerivedPillarContributions {
    /// Natural armor from feats (`armor_of_the_pit_natural_armor_bonus_from_feats`).
    /// Kept as its own field, not merged into the Armor Class total, because a
    /// touch attack ignores natural armor and both paths must subtract exactly
    /// this much and no more.
    pub natural_armor_bonus: i16,
    /// Dodge's dodge bonus. A conditional CONTRIBUTION, never a precondition
    /// -- see `compute_combat_baseline`'s own note on why requiring it forced
    /// character creation to claim a feat no slot had granted.
    pub dodge_armor_class_bonus: i16,
    /// `feat_effects::skill_bonuses_from_feats` -- Athletic's Climb/Swim,
    /// Persuasive's and Intimidating Prowess's Intimidate.
    pub skill_bonuses: crate::rules_core::feat_effects::SkillBonusesFromFeats,
    /// `feat_effects::arg_computed_climb_bonus_from_feats` -- Sure and Fleet's
    /// racial +2 Climb. Separate from `skill_bonuses` because the explanation
    /// strings on the hardcoded path name the two sources independently.
    pub arg_climb_bonus: i16,
}

impl FeatDerivedPillarContributions {
    /// Everything feats add to Armor Class, natural armor included.
    pub fn armor_class_bonus(self) -> i16 {
        self.natural_armor_bonus + self.dodge_armor_class_bonus
    }

    /// The part of [`Self::armor_class_bonus`] a touch attack ignores. PF1
    /// touch AC drops natural armor and keeps dodge bonuses.
    pub fn excluded_from_touch_armor_class(self) -> i16 {
        self.natural_armor_bonus
    }

    /// The part of [`Self::armor_class_bonus`] a **flat-footed** character is
    /// denied — the exact complement of [`Self::excluded_from_touch_armor_class`]
    /// on the two fields this struct carries today, and deliberately written as
    /// its own accessor rather than as "whatever touch keeps".
    ///
    /// PF1, *Bonus Types*: "Any situation that denies you your Dexterity bonus
    /// to Armor Class also denies you dodge bonuses." Natural armor is not
    /// denied (it is part of the creature's hide, not its reflexes), so a feat
    /// added to [`Self::natural_armor_bonus`] must move flat-footed AC exactly
    /// as much as it moves Armor Class, and one added to
    /// [`Self::dodge_armor_class_bonus`] must move it not at all. Reading this
    /// accessor from both twins is what makes that true on both at once.
    pub fn denied_to_flat_footed_armor_class(self) -> i16 {
        self.dodge_armor_class_bonus
    }

    /// Everything feats add to the Climb total.
    pub fn climb_skill_bonus(self) -> i16 {
        self.skill_bonuses.climb + self.arg_climb_bonus
    }

    /// Everything feats add to the Intimidate total.
    pub fn intimidate_skill_bonus(self) -> i16 {
        self.skill_bonuses.intimidate
    }

    /// Everything feats add to the Swim total.
    pub fn swim_skill_bonus(self) -> i16 {
        self.skill_bonuses.swim
    }
}

/// Resolves [`FeatDerivedPillarContributions`] for one character. The sole
/// `feat_effects` reader for every pillar the two compute twins each derive
/// independently.
///
/// `strength_modifier` is threaded in rather than re-derived because
/// Intimidating Prowess adds the character's real Strength modifier to
/// Intimidate, and both callers already hold the resolved
/// `AbilityModifiers`.
pub fn feat_derived_pillar_contributions(
    input: &CharacterInput,
    strength_modifier: i16,
) -> FeatDerivedPillarContributions {
    let feats = effective_character_feats(input);
    FeatDerivedPillarContributions {
        // SD-27 (`decisions.md` §24/§28, 2026-07-31): ARG's Armor of the Pit is
        // the only one of the Advanced Race Guide's 187 feats whose
        // unconditional corpus magnitude lands on a total this engine computes
        // -- a `+2` natural armor bonus, and therefore a real, visible change to
        // a tiefling's Armor Class and (by exclusion) to their touch AC.
        //
        // Its corpus token carries `!PREABILITY:1,CATEGORY=Special Ability,
        // Scaled Skin C ~ Tiefling,...`, which a mechanical "an inline PRE means
        // situational" reading would classify as conditional. It is not a
        // situation: it asks whether this character took the Scaled Skin
        // alternate racial trait, a persisted creation-time decision this engine
        // already reads through `selected_alternate_trait_keys`. So the branch is
        // fully decided here rather than deferred, matching the `BENEFIT:` prose
        // exactly ("+2 natural armor bonus. If you have the scaled skin racial
        // trait, you *instead* gain resistance 5 to two of ...").
        //
        // Deliberately NOT race-gated on `PREFACT:1,TEMPLATES,IsTiefling=true`:
        // asserting a feat's selection prerequisites is `feat_prereqs`' job, the
        // same split `master_craftsman_facts_from_choices` already documents.
        natural_armor_bonus:
            crate::rules_core::feat_effects::armor_of_the_pit_natural_armor_bonus_from_feats(
                &feats,
                character_has_tiefling_scaled_skin(input),
            ),
        // The identity fold is slot-blind: a Dodge granted through any slot
        // counts exactly as one picked from the catalog.
        dodge_armor_class_bonus: if feat_identity::holds(&input.chosen.selected_feats, DODGE_FEAT_ID)
        {
            DODGE_AC_BONUS
        } else {
            0
        },
        // Athletic's real +2 Climb/Swim, Persuasive's real +2 Intimidate, and
        // Intimidating Prowess's real Strength-modifier-to-Intimidate, each
        // stacking independently.
        skill_bonuses: crate::rules_core::feat_effects::skill_bonuses_from_feats(
            &feats,
            strength_modifier,
        ),
        // SD-27 (`decisions.md` §24/§28, 2026-07-31): ARG's Sure and Fleet is
        // the only one of the Advanced Race Guide's 187 feats whose
        // unconditional `BONUS:SKILL` token names a skill this engine computes a
        // total for. Its `+2` is `TYPE=Racial` and does NOT stack with the
        // halfling Sure-Footed racial bonus -- but it cannot collide with it
        // either: the feat requires `Halfling ~ Fleet Of Foot`, which is
        // precisely the alternate racial trait that replaces Sure-Footed. See
        // `feat_effects::ARG_SKILL_FEAT_FACTS`' own doc comment.
        arg_climb_bonus: crate::rules_core::feat_effects::arg_computed_climb_bonus_from_feats(
            &feats,
        ),
    }
}

pub(super) fn compute_selected_skill_modifiers(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) -> SelectedSkillModifiers {
    let unmet = unmet_selected_skill_posture_conditions(input);

    if !unmet.is_empty() {
        diagnostics.push(ComputationDiagnostic {
            id: "skill.selected_modifier.unsupported".to_owned(),
            message: format!(
                "selected skill modifiers are only computed for the exact GE-06 deterministic \
                 level-1 Climb/Intimidate/Swim rank-1 posture (on a grounded {}) with the \
                 grounded Chain Shirt armor-check penalty; unmet conditions: {}",
                supported_class_chassis_description(),
                unmet.join("; ")
            ),
            claim_blocking: true,
        });
        return SelectedSkillModifiers::default();
    }

    // SD-36 Epic F3b3: a selected skill whose class-skill status no class answers is refused
    // by name -- never printed without (or with) a +3 the record cannot support.
    let class_skill_unknowns = selected_skill_class_skill_unknowns(input);
    if !class_skill_unknowns.is_empty() {
        diagnostics.push(ComputationDiagnostic {
            id: SELECTED_SKILL_CLASS_SKILL_UNKNOWN.to_owned(),
            message: format!(
                "the class-skill bonus on the selected skills cannot be decided: no class the \
                 character has grants the skill, and a class's converted record does not answer \
                 its class skills ({})",
                class_skill_unknowns.join(" | ")
            ),
            claim_blocking: true,
        });
        return SelectedSkillModifiers::default();
    }

    let rank = i16::from(SELECTED_SKILL_RANK);

    // The Chain Shirt armor-check penalty applied to Climb/Swim is reduced by Fighter
    // armor training from level 3, so the armor-check skills rise at that milestone.
    // The posture check above guarantees a supported Fighter level here.
    let level = supported_fighter_level(input).unwrap_or(1);
    let armor_check_penalty = effective_chain_shirt_armor_check_penalty(level);
    let armor_check_detail = if fighter_armor_training(level).armor_check_reduction > 0 {
        format!(
            "Chain Shirt armor-check penalty ({armor_check_penalty:+}, reduced from \
             {CHAIN_SHIRT_ARMOR_CHECK_PENALTY:+} by Fighter armor training)"
        )
    } else {
        format!("Chain Shirt armor-check penalty ({armor_check_penalty:+})")
    };

    // v0.6 alpha swarm: the class-skill bonus only applies for a class whose
    // real PF1 class-skill list actually includes the skill in question --
    // see selected_skill_climb_is_class_skill's own doc comment for why
    // this is three independent per-skill checks, not one shared scalar,
    // since Investigator's own real class-skill list is a genuine partial
    // match (Climb/Intimidate yes, Swim no).
    let climb_class_skill_bonus_applies = selected_skill_climb_is_class_skill(input);
    let climb_class_skill_bonus = if climb_class_skill_bonus_applies { CLASS_SKILL_BONUS } else { 0 };
    let climb_class_skill_bonus_detail = if climb_class_skill_bonus_applies {
        format!("class-skill bonus ({climb_class_skill_bonus:+})")
    } else {
        "no class-skill bonus (Climb is not a class skill for this character's class)".to_owned()
    };

    let intimidate_class_skill_bonus_applies = selected_skill_intimidate_is_class_skill(input);
    let intimidate_class_skill_bonus =
        if intimidate_class_skill_bonus_applies { CLASS_SKILL_BONUS } else { 0 };
    let intimidate_class_skill_bonus_detail = if intimidate_class_skill_bonus_applies {
        format!("class-skill bonus ({intimidate_class_skill_bonus:+})")
    } else {
        "no class-skill bonus (Intimidate is not a class skill for this character's class)"
            .to_owned()
    };

    let swim_class_skill_bonus_applies = selected_skill_swim_is_class_skill(input);
    let swim_class_skill_bonus = if swim_class_skill_bonus_applies { CLASS_SKILL_BONUS } else { 0 };
    let swim_class_skill_bonus_detail = if swim_class_skill_bonus_applies {
        format!("class-skill bonus ({swim_class_skill_bonus:+})")
    } else {
        "no class-skill bonus (Swim is not a class skill for this character's class)".to_owned()
    };

    // v0.6 alpha swarm, risks item 8: Cleric Good domain's Touch of Good
    // sacred bonus (self-application only) applies to all three selected
    // skills -- class-ownership-gated by `active_touch_of_good_bonus`
    // construction, 0 for every non-Cleric, non-Good-domain, or
    // not-currently-active character.
    let touch_of_good_skill_bonus = active_touch_of_good_bonus(input).unwrap_or(0);
    let touch_of_good_skill_detail = if touch_of_good_skill_bonus > 0 {
        format!(" + Good domain Touch of Good sacred bonus ({touch_of_good_skill_bonus:+}, self-applied)")
    } else {
        String::new()
    };

    // v0.6 alpha swarm: the real, grounded feat-derived skill bonus -- the same
    // "headless-accessible, no corpus needed" layering `compute_total_saves`'s
    // own `feat_save_bonuses` already established for Great Fortitude/Iron
    // Will/Lightning Reflexes.
    //
    // SD-27 (`decisions.md` §28, feat-seam defect, 2026-07-31): read through
    // `feat_derived_pillar_contributions` rather than calling
    // `feat_effects::skill_bonuses_from_feats` here, so the corpus twin
    // (`compute_selected_skill_modifiers_from_corpus`, the path the shipped
    // sheet actually reads) consumes the identical value. Athletic, Persuasive
    // and Intimidating Prowess all moved this total and moved the sheet by
    // nothing until it did.
    let feat_contributions =
        feat_derived_pillar_contributions(input, ability_modifiers.strength);
    let feat_skill_bonuses = feat_contributions.skill_bonuses;

    // v0.6 alpha swarm, risks item 8 (Inquisitor Judgment closure, widened
    // 2026-07-26): Inquisitor Stern Gaze's morale bonus applies to
    // Intimidate only (Sense Motive is the other half of its real DESC
    // text, but this codebase computes no Sense Motive total anywhere --
    // see `active_inquisitor_stern_gaze_bonus`'s own doc comment) --
    // class-ownership-gated, 0 for every non-Inquisitor character, and
    // unconditional (no choice/activation gate) for every Inquisitor.
    let stern_gaze_intimidate_bonus = active_inquisitor_stern_gaze_bonus(input).unwrap_or(0);
    let stern_gaze_detail = if stern_gaze_intimidate_bonus > 0 {
        format!(" + Inquisitor Stern Gaze morale bonus ({stern_gaze_intimidate_bonus:+})")
    } else {
        String::new()
    };

    // v0.6 alpha swarm task #54: Raging Climber's / Raging Swimmer's shared
    // RagePowersLVL magnitude (see active_rage_powers_level's own doc
    // comment) -- class-ownership- and Raging-state-gated by construction,
    // 0 for every non-Barbarian, non-Skald, or not-currently-raging
    // character.
    let raging_climber_swimmer_bonus = active_rage_powers_level(input, ability_modifiers);
    // SD-27 (alternate racial traits reach compute): the racial skill bonus a
    // chosen ARG alternate declares, for the ten of 153 whose `BONUS:SKILL`
    // names Climb, Intimidate or Swim with a plain integer. Race-gated and
    // selection-gated by `alternate_trait_selected_skill_bonuses`
    // construction, all zeroes for every character who took none — and the
    // highest of two same-typed racial bonuses rather than their sum; see that
    // function's own doc comment.
    let alternate_trait_skill_bonuses = alternate_trait_selected_skill_bonuses(input);
    let alternate_trait_skill_detail = |bonus: i16| {
        if bonus > 0 {
            format!(" + alternate racial trait bonus ({bonus:+}, racial)")
        } else {
            String::new()
        }
    };
    let raging_climber_swimmer_detail = if raging_climber_swimmer_bonus > 0 {
        format!(" + Raging Climber/Raging Swimmer enhancement bonus ({raging_climber_swimmer_bonus:+}, while raging)")
    } else {
        String::new()
    };

    // SD-27 (`decisions.md` §24/§28, 2026-07-31): ARG's Sure and Fleet is the
    // only one of the Advanced Race Guide's 187 feats whose unconditional
    // `BONUS:SKILL` token names a skill this engine computes a total for.
    // Its `+2` is `TYPE=Racial` and does NOT stack with the halfling
    // Sure-Footed racial bonus — but it cannot collide with it either: the
    // feat requires `Halfling ~ Fleet Of Foot`, which is precisely the
    // alternate racial trait that replaces Sure-Footed. See
    // `feat_effects::ARG_SKILL_FEAT_FACTS`' own doc comment.
    //
    // Read through `feat_derived_pillar_contributions` for the same reason
    // `feat_skill_bonuses` above is: the corpus twin must see it too.
    let arg_feat_climb_bonus = feat_contributions.arg_climb_bonus;
    let arg_feat_climb_detail = if arg_feat_climb_bonus > 0 {
        format!(" + ARG Sure and Fleet racial bonus ({arg_feat_climb_bonus:+})")
    } else {
        String::new()
    };

    // Climb (STR, armor-check skill): rank + STR + class-skill + Chain Shirt ACP.
    let climb = rank
        + ability_modifiers.strength
        + climb_class_skill_bonus
        + armor_check_penalty
        + touch_of_good_skill_bonus
        + feat_skill_bonuses.climb
        + arg_feat_climb_bonus
        + raging_climber_swimmer_bonus
        + alternate_trait_skill_bonuses.climb;
    explanations.push(ComputationExplanation {
        id: "skill.selected_modifier.climb".to_owned(),
        value: climb,
        detail: format!(
            "Selected Climb modifier: rank {rank} + Strength modifier ({:+}) + \
             {climb_class_skill_bonus_detail} + {armor_check_detail}{touch_of_good_skill_detail} \
             + feat bonus (+{}, Athletic if selected){arg_feat_climb_detail}\
             {raging_climber_swimmer_detail}{} = {climb}",
            ability_modifiers.strength, feat_skill_bonuses.climb,
            alternate_trait_skill_detail(alternate_trait_skill_bonuses.climb)
        ),
    });

    // Intimidate (CHA, not an armor-check skill): rank + CHA + class-skill.
    let intimidate = rank
        + ability_modifiers.charisma
        + intimidate_class_skill_bonus
        + touch_of_good_skill_bonus
        + feat_skill_bonuses.intimidate
        + stern_gaze_intimidate_bonus
        + alternate_trait_skill_bonuses.intimidate;
    explanations.push(ComputationExplanation {
        id: "skill.selected_modifier.intimidate".to_owned(),
        value: intimidate,
        detail: format!(
            "Selected Intimidate modifier: rank {rank} + Charisma modifier ({:+}) + \
             {intimidate_class_skill_bonus_detail}{touch_of_good_skill_detail} + feat bonus \
             (+{}, Persuasive and/or Intimidating Prowess if selected){stern_gaze_detail}{} = \
             {intimidate}",
            ability_modifiers.charisma, feat_skill_bonuses.intimidate,
            alternate_trait_skill_detail(alternate_trait_skill_bonuses.intimidate)
        ),
    });

    // Swim (STR, armor-check skill): rank + STR + class-skill + Chain Shirt ACP.
    // v0.6 alpha swarm task #11 (2026-07-27): the Witch's Flight hex adds
    // a real +4 here -- the only one of the 53 hex records whose magnitude
    // lands on a total this engine computes. Class-ownership-gated and
    // explicit-choice-gated by construction, 0 for everyone else.
    let flight_hex_swim_bonus = witch_flight_hex_swim_bonus(input);
    let swim = rank
        + ability_modifiers.strength
        + swim_class_skill_bonus
        + armor_check_penalty
        + touch_of_good_skill_bonus
        + feat_skill_bonuses.swim
        + flight_hex_swim_bonus
        + raging_climber_swimmer_bonus
        + alternate_trait_skill_bonuses.swim;
    explanations.push(ComputationExplanation {
        id: "skill.selected_modifier.swim".to_owned(),
        value: swim,
        detail: format!(
            "Selected Swim modifier: rank {rank} + Strength modifier ({:+}) + \
             {swim_class_skill_bonus_detail} + {armor_check_detail}{touch_of_good_skill_detail} \
             + feat bonus (+{}, Athletic if selected) + Witch Flight hex \
             (+{flight_hex_swim_bonus}){raging_climber_swimmer_detail}{} = {swim}",
            ability_modifiers.strength, feat_skill_bonuses.swim,
            alternate_trait_skill_detail(alternate_trait_skill_bonuses.swim)
        ),
    });

    SelectedSkillModifiers {
        climb,
        intimidate,
        swim,
    }
}

/// Grounds every standalone feat-derived skill fact this engine's
/// `feat_effects` module already computes but has no live total to layer
/// onto (turnkey consumer wiring for `feat_effects::
/// standalone_skill_facts_from_feats`/`skill_focus_facts_from_choices`,
/// both already built and independently tested by the feat-effects agent).
/// Unlike `skill_bonuses_from_feats` (wired directly into
/// `compute_selected_skill_modifiers`'s Climb/Intimidate/Swim totals),
/// these facts target skills this codebase computes no total for at all
/// (Acrobatics, Fly, Perception, Sense Motive, Handle Animal, Ride,
/// Bluff, Disguise, Disable Device, Sleight of Hand, Spellcraft, Use
/// Magic Device, Diplomacy, Heal, Survival, Escape Artist, Stealth, or
/// whichever skill Skill Focus's own player choice names) -- so each
/// grounds as its own standalone explanation record, explicitly labeled
/// as not wired into any skill total, mirroring the established
/// standalone-fact idiom (Bard's Bardic Knowledge, Slayer's Track,
/// Barbarian's Damage Reduction, Inquisitor's Smiting). Unconditional on
/// class ownership or posture -- these are general feat effects, not
/// class-specific, so every character's `selected_feats`/
/// Grounds the `feat_effects` producers that target dimensions this
/// engine computes NOWHERE (task #22, 2026-07-27): initiative,
/// non-skill checks, combat maneuvers, unarmed strikes, and movement.
///
/// These eight producers were built, tested, and consumed by nothing --
/// correct code that never ran. Seven of them ground standalone here
/// under the corrected bar (a genuine, verifiable magnitude, honestly
/// labelled as not-integrated); the eighth, Toughness, has a real total
/// and is wired into the Fighter hit-point record instead.
///
/// Reads EFFECTIVE feats, so a Monk's automatically-granted Stunning
/// Fist and a Ranger's granted Endurance reach these producers even
/// though neither ever appears in `selected_feats`.
pub(super) fn ground_orphan_feat_facts(
    input: &CharacterInput,
    base_attack_bonus: i16,
    chassis_supported: bool,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    use crate::rules_core::feat_effects;
    let feats = effective_character_feats(input);

    let initiative = feat_effects::initiative_bonus_from_feats(&feats);
    if initiative != 0 {
        explanations.push(ComputationExplanation {
            id: "feat.standalone.initiative_bonus".to_owned(),
            value: initiative,
            detail: format!(
                "Improved Initiative grants a +{initiative} bonus on initiative checks. This \
                 engine computes no initiative total anywhere, so this grounds as a standalone \
                 flat record -- the same shape as Inquisitor's own Cunning Initiative and \
                 Oracle's Deaf-curse initiative penalty"
            ),
        });
    }

    let endurance = feat_effects::endurance_check_bonus_from_feats(&feats);
    if endurance != 0 {
        explanations.push(ComputationExplanation {
            id: "feat.standalone.endurance_check_bonus".to_owned(),
            value: endurance,
            detail: format!(
                "Endurance grants a +{endurance} bonus on checks to resist nonlethal damage, \
                 hunger, thirst, exhaustion, and suffocation. None of those checks is computed \
                 anywhere here, so this grounds standalone. A Ranger receives this feat \
                 automatically at 3rd level and never selects it, which is why it is read from \
                 the effective feat set rather than the chosen one"
            ),
        });
    }

    let speed = feat_effects::base_speed_bonus_from_feats(&feats);
    if speed != 0 {
        explanations.push(ComputationExplanation {
            id: "feat.standalone.base_speed_bonus".to_owned(),
            value: speed,
            detail: format!(
                "Fleet increases base land speed by {speed} feet. Fleet is `STACK:YES MULT:YES`, \
                 so repeated picks genuinely stack and the producer counts occurrences rather \
                 than testing presence. This engine computes no movement total, so it grounds \
                 standalone"
            ),
        });
    }

    let terrain = feat_effects::difficult_terrain_feet_from_feats(&feats);
    if terrain != 0 {
        explanations.push(ComputationExplanation {
            id: "feat.standalone.difficult_terrain_feet".to_owned(),
            value: terrain,
            detail: format!(
                "Nimble Moves lets the character move {terrain} feet through difficult terrain \
                 each round as though it were normal terrain. No movement or terrain state \
                 exists in this engine, so this grounds standalone"
            ),
        });
    }

    // v0.6 alpha swarm (APG/ACG passive-bonus widening, 2026-07-29): the four
    // APG/ACG feats whose flat bonus lands on a dimension this engine computes
    // no total for. Each is a real corpus magnitude with a live consumer here,
    // not an inert producer.
    let sharp_senses = feat_effects::sharp_senses_perception_bonus_from_feats(&feats);
    if sharp_senses != 0 {
        explanations.push(ComputationExplanation {
            id: "feat.standalone.sharp_senses_perception_bonus".to_owned(),
            value: sharp_senses,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   Its corpus token is only BONUS:VAR|KeenSensesBonus|2, an INCREMENT to the same
                //   variable the keen-senses racial trait already sets to 2 -- the feat's BENEFIT
                //   prose states the resulting total, +4, and says it REPLACES the racial bonus
                //   rather than stacking with it.
                "Sharp Senses (APG) grants a +{sharp_senses} racial bonus on Perception checks. Its \
                 own PREABILITY TYPE.KeenSenses prerequisite guarantees that racial base, so +4 is \
                 exact. Perception is not among the three skills compute_selected_skill_modifiers \
                 tracks (Climb/Intimidate/Swim), so this grounds standalone; where a separate racial \
                 keen-senses +2 record also appears, this +4 supersedes it and the two must not be \
                 added"
            ),
        });
    }

    let steel_soul = feat_effects::steel_soul_save_vs_spells_bonus_from_feats(&feats);
    if steel_soul != 0 {
        explanations.push(ComputationExplanation {
            id: "feat.standalone.steel_soul_save_vs_spells".to_owned(),
            value: steel_soul,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   Same increment-not-total shape as Sharp Senses: the token is
                //   BONUS:VAR|SaveBonus_vs_Spells|2, incrementing the variable the dwarf Hardy
                //   racial trait already sets to 2, and the BENEFIT prose states the +4 result and
                //   that it REPLACES Hardy's bonus rather than stacking.
                "Steel Soul (APG) grants a +{steel_soul} racial bonus on saving throws against \
                 spells and spell-like abilities. Deliberately NOT added to the \
                 Fortitude/Reflex/Will totals: it applies only against spells and spell-like \
                 abilities, while those totals are the general unconditional saves, so folding it in \
                 would overstate every save against a non-magical effect. Grounds standalone, \
                 exactly as this engine already grounds the dwarf Hardy bonus this feat supersedes"
            ),
        });
    }

    let deepsight = feat_effects::deepsight_darkvision_bonus_from_feats(&feats);
    if deepsight != 0 {
        explanations.push(ComputationExplanation {
            id: "feat.standalone.deepsight_darkvision_feet".to_owned(),
            value: deepsight,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:VISION|Darkvision|60
                //   Its PREVISION:1,Darkvision=60 prerequisite fixes the holder's base at 60 feet,
                //   so the BENEFIT prose's resulting range of 120 feet is exact.
                "Deepsight (APG) extends darkvision by {deepsight} feet. This engine models no \
                 vision numerically anywhere -- racial darkvision exists only as prose inside a race \
                 trait's detail string, never as a number -- so the increment grounds standalone and \
                 the 120-foot total cannot be computed from data here"
            ),
        });
    }

    let charisma = ability_modifier(input.chosen.ability_scores.charisma);
    let wisdom_for_steadfast = ability_modifier(input.chosen.ability_scores.wisdom);
    if let Some(steadfast) = feat_effects::steadfast_personality_will_bonus_from_feats(
        &feats,
        charisma,
        wisdom_for_steadfast,
    ) {
        explanations.push(ComputationExplanation {
            id: "feat.standalone.steadfast_personality_will_vs_mind_affecting".to_owned(),
            value: steadfast,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   Both corpus tokens are needed: BONUS:SAVE|Will|CHA-WIS swaps Charisma in for
                //   Wisdom, and BONUS:SAVE|Will|WIS|PREVARLT:WIS,0 adds Wisdom back only when it is
                //   a penalty, matching the BENEFIT prose that a Wisdom penalty applies alongside
                //   the Charisma modifier rather than being replaced by it.
                "Steadfast Personality (ACG) changes Will saves against mind-affecting effects by \
                 {steadfast:+} for this character: Charisma ({charisma:+}) replaces Wisdom \
                 ({wisdom_for_steadfast:+}), i.e. CHA - max(WIS, 0). Deliberately NOT added to the \
                 Will total: this applies ONLY against mind-affecting effects, whereas the computed \
                 Will total is the general save -- unlike Oracle's Sidestep Secret, which is \
                 unconditional and therefore is integrated. A zero here is a real result (equal \
                 Charisma and Wisdom), not an absent one"
            ),
        });
    }

    for bonus in feat_effects::combat_maneuver_bonuses_from_feats(&feats) {
        let slug = bonus.maneuver.to_lowercase().replace(' ', "_");
        explanations.push(ComputationExplanation {
            id: format!("feat.standalone.combat_maneuver.{slug}.cmb"),
            value: bonus.cmb_bonus,
            detail: format!(
                "{} grants a +{} bonus on {} combat maneuver checks. This engine computes no \
                 combat maneuver bonus total, so this grounds standalone",
                bonus.feat_key, bonus.cmb_bonus, bonus.maneuver
            ),
        });
        if bonus.cmd_bonus != 0 {
            explanations.push(ComputationExplanation {
                id: format!("feat.standalone.combat_maneuver.{slug}.cmd"),
                value: bonus.cmd_bonus,
                detail: format!(
                    "{} also grants a +{} bonus to Combat Maneuver Defense against {}. The six \
                     Greater variants carry no CMD term at all in the corpus, so only the \
                     Improved feats produce this record",
                    bonus.feat_key, bonus.cmd_bonus, bonus.maneuver
                ),
            });
        }
    }

    let total_level: u8 = input.chosen.class_levels.iter().map(|c| c.level).sum();
    let monk_level = input
        .chosen
        .class_levels
        .iter()
        .find(|c| c.class_id == MONK_CLASS_ID)
        .map(|c| c.level)
        .unwrap_or(0);
    let wisdom = ability_modifier(input.chosen.ability_scores.wisdom);
    if let Some(facts) =
        feat_effects::stunning_fist_facts_from_feats(&feats, total_level, monk_level, wisdom)
    {
        explanations.push(ComputationExplanation {
            id: "feat.standalone.stunning_fist.save_dc".to_owned(),
            value: facts.save_dc,
            detail: format!(
                "Stunning Fist save DC: {} (10 + half total level + Wisdom modifier \
                 ({wisdom:+})). A Monk is GRANTED this feat at 1st level and never selects it, \
                 so it is read from the effective feat set. No unarmed-strike or \
                 condition-resolution engine exists here, so this grounds standalone",
                facts.save_dc
            ),
        });
        explanations.push(ComputationExplanation {
            id: "feat.standalone.stunning_fist.uses_per_day".to_owned(),
            value: facts.uses_per_day,
            detail: format!(
                "Stunning Fist uses per day: {} (monk level, plus one per four non-monk levels). \
                 A flat daily pool -- no per-use consumption is tracked",
                facts.uses_per_day
            ),
        });
    }

    for fact in
        feat_effects::weapon_focus_facts_from_choices(&feats, &input.chosen.selected_choices)
    {
        let slug = fact.weapon_name.to_lowercase().replace(' ', "_");
        explanations.push(ComputationExplanation {
            id: format!("feat.standalone.weapon_focus.{slug}"),
            value: fact.attack_bonus,
            detail: format!(
                "Weapon Focus ({}) grants a +{} bonus on attack rolls with that weapon (+2 when \
                 Greater Weapon Focus names it too). This bonus is also carried inside \
                 combat.weapon_attack_bonus.<weapon> whenever that weapon is equipped; this record \
                 is the component fact, that one is the total, and the two agreeing is expected \
                 rather than a double-count. It still grounds standalone, because the feat's bonus \
                 is real whether or not the chosen weapon happens to be equipped. Requires an \
                 explicit recorded weapon choice -- nothing is seeded, per the same \
                 no-silent-seeding design as Skill Focus",
                fact.weapon_name, fact.attack_bonus
            ),
        });
    }

    for fact in feat_effects::spell_focus_facts_from_choices(&feats, &input.chosen.selected_choices)
    {
        let slug = fact.school_name.to_lowercase().replace(' ', "_");
        explanations.push(ComputationExplanation {
            id: format!("feat.standalone.spell_focus.{slug}"),
            value: fact.dc_bonus,
            detail: format!(
                "Spell Focus ({}) raises the save DC of that school's spells by +{}. \
                 Deliberately NOT folded into this codebase's spell-save-DC records: those are \
                 keyed by spell LEVEL while Spell Focus is keyed by SCHOOL, so integrating would \
                 require inventing a level-to-school mapping that the records do not carry. \
                 Grounds standalone instead",
                fact.school_name, fact.dc_bonus
            ),
        });
    }

    ground_arg_and_pu_feat_facts(
        input,
        &feats,
        base_attack_bonus,
        chassis_supported,
        ability_modifiers,
        explanations,
    );
}

/// AT-34-E4-002's fifth trait-capability slice: standalone orphan facts
/// derived from a character's SELECTED traits (unlike feats, traits carry
/// no automatic-grant concept, so this reads `input.chosen.selected_traits`
/// directly rather than an "effective" set). Mirrors
/// `ground_orphan_feat_facts` exactly -- this engine computes no
/// integrated initiative total and no concentration-check total anywhere,
/// so both pillars ground as standalone facts rather than being folded
/// into any total. Unconditional on class ownership/posture, the same as
/// every other orphan-fact producer: these are general trait effects, not
/// class-specific.
pub(super) fn ground_orphan_trait_facts(input: &CharacterInput, explanations: &mut Vec<ComputationExplanation>) {
    use crate::rules_core::trait_effects;
    let selected_traits = &input.chosen.selected_traits;

    let initiative = trait_effects::initiative_bonus_from_traits(selected_traits);
    if initiative != 0 {
        explanations.push(ComputationExplanation {
            id: "trait.standalone.initiative_bonus".to_owned(),
            value: initiative,
            detail: format!(
                "A selected trait (Tactician and/or Arcane Temper) grants a +{initiative} bonus \
                 on initiative checks. This engine computes no initiative total anywhere, so \
                 this grounds as a standalone flat record -- the same shape as Improved \
                 Initiative's own already-grounded standalone record"
            ),
        });
    }

    let concentration = trait_effects::concentration_bonus_from_traits(selected_traits);
    if concentration != 0 {
        explanations.push(ComputationExplanation {
            id: "trait.standalone.concentration_bonus".to_owned(),
            value: concentration,
            detail: format!(
                "A selected trait (Arcane Temper and/or Desperate Resolve) grants a +{concentration} \
                 bonus on concentration checks. This engine computes no concentration-check total \
                 anywhere, so this grounds as a standalone flat record"
            ),
        });
    }

    // AT-34-E4-002's seventh trait-capability slice: `BONUS:SITUATION`
    // traits. Mirrors `feat_effects::arg_situational_skill_facts_from_feats`
    // exactly -- the circumstance is carried in the record's own text
    // rather than folded into a skill total, which would report a
    // specific, checkable, wrong number on the ordinary check.
    for fact in trait_effects::situational_skill_facts_from_traits(selected_traits) {
        explanations.push(ComputationExplanation {
            id: trait_effects::situational_skill_fact_explanation_id(
                fact.trait_id,
                fact.skill_name,
            ),
            value: fact.bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:SITUATION|<skill>=<circumstance>|<bonus>
                "{} grants a {:+} bonus on {} checks {}, transcribed from its corpus \
                 situational-bonus entry. This is a \
                 SITUATIONAL bonus and is deliberately NOT added to any skill total: it applies \
                 only in the circumstance named here, and folding it into a general modifier \
                 would report a specific, checkable, wrong number on every ordinary {} check. \
                 Same treatment the ARG feat situational records and the Dwarf Stonecunning/\
                 Greed situational records already receive",
                fact.trait_name,
                fact.bonus,
                fact.skill_name,
                fact.circumstance,
                fact.skill_name
            ),
        });
    }

    // AT-34-E4-002's eighth trait-capability slice: a `BONUS:CASTERLEVEL|
    // SUBSCHOOL` token, deliberately NOT folded into any integrated
    // caster-level total (this crate has none, and one keyed by class
    // level would misreport every OTHER subschool's spells) -- mirrors
    // `feat_effects::spell_focus_facts_from_choices`'s own per-school
    // spell-save-DC standalone treatment.
    for fact in trait_effects::caster_level_subschool_facts_from_traits(selected_traits) {
        explanations.push(ComputationExplanation {
            id: trait_effects::caster_level_subschool_fact_explanation_id(fact.trait_id),
            value: fact.bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:CASTERLEVEL|SUBSCHOOL.<subschool>|<bonus>
                "{} lets you treat your caster level as {:+} for spells of the {} subschool, \
                 transcribed from its corpus caster-level entry. This \
                 engine computes no integrated per-subschool caster level total anywhere, so \
                 this grounds as a standalone flat record rather than folding into a total that \
                 would misstate every other subschool's spells",
                fact.trait_name, fact.bonus, fact.subschool
            ),
        });
    }
}

/// SD-27 (`decisions.md` §24/§28, 2026-07-31): the Advanced Race Guide's and
/// Pathfinder Unchained's feats whose unconditional corpus `BONUS:` token
/// carries a real standing magnitude on a dimension this engine has no total
/// for.
///
/// Split out of [`ground_orphan_feat_facts`] rather than appended to it because
/// these are two whole books' worth of records with their own classification
/// story (see `feat_effects`' own SD-27 section header for the derivation that
/// chose this set), and because two of them -- Armor of the Pit's natural armor
/// and Sure and Fleet's Climb -- deliberately do NOT ground here: they reach
/// real computed totals instead, in `compute_combat_baseline` and
/// `compute_selected_skill_modifiers`.
///
/// Every producer is keyed on the EFFECTIVE feat set, so a class-granted copy
/// of any of these reaches its record exactly as a chosen one does.
pub(super) fn ground_arg_and_pu_feat_facts(
    input: &CharacterInput,
    feats: &[String],
    base_attack_bonus: i16,
    chassis_supported: bool,
    ability_modifiers: &AbilityModifiers,
    explanations: &mut Vec<ComputationExplanation>,
) {
    use crate::rules_core::feat_effects;

    for fact in feat_effects::arg_maneuver_defense_facts_from_feats(feats) {
        let feat_slug = slugify_id_segment(fact.feat_key);
        let maneuver_slug = slugify_id_segment(fact.maneuver);
        explanations.push(ComputationExplanation {
            id: format!("feat.arg_maneuver_defense.{feat_slug}.{maneuver_slug}"),
            value: fact.cmd_bonus,
            detail: format!(
                "{} (ARG) grants a +{} bonus to Combat Maneuver Defense against {}. \
                 Deliberately NOT added to defense.combat_maneuver_defense: that total is the \
                 general CMD, applying to every maneuver, and folding in a bonus that covers \
                 only some of them would report a specific, checkable, wrong number against a \
                 disarm or a sunder. The condition is the opponent's ACTION TYPE, a static \
                 defensive property of the character, which is why it grounds at all",
                fact.feat_key, fact.cmd_bonus, fact.maneuver
            ),
        });
    }

    for fact in feat_effects::arg_energy_resistance_facts_from_feats(feats) {
        let feat_slug = slugify_id_segment(fact.feat_key);
        explanations.push(ComputationExplanation {
            id: format!("feat.arg_energy_resistance.{feat_slug}.{}", fact.energy_type),
            value: fact.amount,
            detail: format!(
                "{} (ARG) grants resistance {} to {}. No energy-resistance total exists anywhere \
                 in this codebase -- the same absence the Inquisitor Resistance judgment and the \
                 Sorcerer Draconic Dragon Resistances records already name -- so this grounds \
                 standalone. Two feats naming one energy type ground two records rather than a \
                 sum: PF1 energy resistance from two sources does not add, the larger applies, \
                 and both of these are 5",
                fact.feat_key, fact.amount, fact.energy_type
            ),
        });
    }

    let flame_heart_caster_level = feat_effects::flame_heart_fire_caster_level_bonus_from_feats(feats);
    if flame_heart_caster_level != 0 {
        explanations.push(ComputationExplanation {
            id: "feat.arg_standalone.flame_heart_fire_caster_level".to_owned(),
            value: flame_heart_caster_level,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:CASTERLEVEL|DESCRIPTOR.Fire|1
                "Flame Heart (ARG) treats the caster level of fire-descriptor spells (and the \
                 alchemist level of fire bombs) as {flame_heart_caster_level} higher. Deliberately \
                 NOT added to any effective_caster_level record: those are the character's general \
                 caster level for every spell, while this applies only to spells carrying the fire \
                 descriptor, and this engine's spell records carry no descriptor to test"
            ),
        });
    }

    let emotion_save = feat_effects::emotion_save_bonus_from_feats(feats);
    if emotion_save != 0 {
        explanations.push(ComputationExplanation {
            id: "feat.arg_standalone.emotion_descriptor_save_bonus".to_owned(),
            value: emotion_save,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:VAR|FearlessCuriosityBonus|1 each
                "Saving throws against effects with the EMOTION descriptor are +{emotion_save} for \
                 this character. Fearless Curiosity and Intimidating Confidence both write the same \
                 corpus variable, PCGen's own way of saying they add -- and neither record states a \
                 literal magnitude at all, printing the running total through a %1 substitution \
                 token instead, so +1 alone and +2 together are read from the pair rather than off \
                 one row. Deliberately NOT added to defense.total_save.will: that total is the \
                 general Will save, and this applies only against emotion-descriptor effects"
            ),
        });
    }

    let swim_speed = feat_effects::aquatic_ancestry_swim_speed_bonus_from_feats(feats);
    if swim_speed != 0 {
        explanations.push(ComputationExplanation {
            id: "feat.arg_standalone.aquatic_ancestry_swim_speed".to_owned(),
            value: swim_speed,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:MOVEADD|TYPE.Swim|10
                "Aquatic Ancestry (ARG) increases swim speed by {swim_speed} feet. This engine \
                 computes no movement total of any kind, so this grounds standalone exactly as \
                 Fleet's base-speed bonus does. The feat's amphibious special quality is a \
                 capability with no magnitude and is not grounded as a number"
            ),
        });
    }

    // 2026-08-01, second pass over SD-27's deferral list. ARG's three
    // `BONUS:SITUATION` tokens (2 feats) were deferred as "the corpus itself
    // classifies these as situational". True -- and not a reason to withhold
    // them: this very file already grounds `BONUS:SITUATION|Perception=to
    // notice unusual stonework|2|TYPE=Racial` and
    // `BONUS:SITUATION|Appraise=to assess nonmagical metals or gemstones|2`
    // for the Core Rulebook dwarf, as flat situational-bonus-magnitude records
    // carrying their circumstance in their own text. Being situational was
    // never the bar; having nowhere to land was, and these land where the
    // dwarf's do.
    for fact in feat_effects::arg_situational_skill_facts_from_feats(feats) {
        let feat_slug = slugify_id_segment(fact.feat_key);
        let skill_slug = slugify_id_segment(fact.skill_name);
        explanations.push(ComputationExplanation {
            id: format!("feat.arg_situational_skill_bonus.{feat_slug}.{skill_slug}"),
            value: fact.bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:SITUATION|<skill>=<circumstance>|<bonus>
                "{} (ARG) grants a {:+} bonus on {} checks {}, transcribed from its corpus \
                 situational-bonus entry and confirmed \
                 against its own printed benefit. This is a SITUATIONAL bonus and is deliberately \
                 NOT added to any skill total: it applies only in the circumstance named here, \
                 and folding it into a general modifier would report a specific, checkable, \
                 wrong number on every ordinary {} check. Same treatment the Dwarf Stonecunning \
                 and Greed situational records already receive",
                fact.feat_key,
                fact.bonus,
                fact.skill_name,
                fact.circumstance,
                fact.skill_name
            ),
        });
    }

    // Improvisation / Improved Improvisation. Deferred in the first pass under
    // "BONUS:VAR increments a bookkeeping variable with a base this engine does
    // not model" -- which is false here: `DEFINE:ImprovisationBonus|0` is on the
    // Improvisation feat record itself, so the running total is wholly
    // feat-determined.
    let improvisation = feat_effects::improvisation_untrained_skill_bonus_from_feats(feats);
    if improvisation != 0 {
        explanations.push(ComputationExplanation {
            id: "feat.arg_standalone.improvisation_untrained_skill_bonus".to_owned(),
            value: improvisation,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:VAR|ImprovisationBonus|2 each
                //   The scope is the corpus's own: Improvisation carries 26 companion
                //   BONUS:SKILL|<skill>|ImprovisationBonus|!PRESKILL:1,<skill>=1 tokens, one per
                //   skill, each gated on having no rank in it.
                "Skill checks for skills this character has NO ranks in are +{improvisation}. \
                 Improvisation and Improved Improvisation both write the one corpus variable \
                 ImprovisationBonus, PCGen's own way of saying they add, and neither record states a \
                 literal -- both print the running total through a %1 substitution token -- so +2 \
                 alone and +4 together are read from the pair. Deliberately NOT added to \
                 skill.selected_modifier.climb/intimidate/swim: this engine's deterministic posture \
                 pins all three at rank 1, and this bonus applies only where there are no ranks. \
                 Improvisation's own use-trained-only-skills-untrained clause \
                 is a capability with no magnitude, and Improved Improvisation \
                 halves an armor check penalty for nonproficiency that this engine \
                 models no proficiency state to incur"
            ),
        });
    }

    // Stretched Wings. Deferred in the first pass as a fly-manoeuvrability feat
    // -- but that is only its `BONUS:VAR|Maneuverability|1` half. Its other
    // token is a plain `BONUS:MOVEADD|TYPE.Fly|40`, the identical movement shape
    // Aquatic Ancestry is already grounded on.
    let fly_speed = feat_effects::stretched_wings_fly_speed_bonus_from_feats(feats);
    if fly_speed != 0 {
        explanations.push(ComputationExplanation {
            id: "feat.arg_standalone.stretched_wings_fly_speed".to_owned(),
            value: fly_speed,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:MOVEADD|TYPE.Fly|40
                //   BONUS:VAR|Maneuverability|2|TYPE=Base for \"poor\"
                //   Its BENEFIT prose states the resulting total rather than the increment (\"Your
                //   strix racial fly speed increases to 60 feet (average)\"), and the two reconcile
                //   exactly because the feat's own PREABILITY:1,CATEGORY=Special Ability,Strix ~
                //   Wing-Clipped fixes the base: arg_abilities_race.lst's Wing-Clipped ~ Strix ~
                //   Flight carries MOVE:Fly,20.
                "Stretched Wings (ARG) increases fly speed by {fly_speed} feet, to 60 feet. This \
                 engine computes no movement total of any kind, so this grounds standalone exactly \
                 as Aquatic Ancestry's swim speed does. The feat's companion \
                 manoeuvrability step is NOT grounded: it moves a manoeuvrability tier on \
                 PCGen's own integer scale (the wing-clipped flight record sets), and this engine \
                 has no manoeuvrability dimension for a tier index to mean anything in"
            ),
        });
    }

    // Defiant Luck / Bestow Luck. The one ARG per-day budget whose DEFINE: sits
    // on the feat record itself, so unlike the halfling, suli and fetchling
    // budgets there is no unmodelled racial base underneath it.
    if let Some(uses) = feat_effects::defiant_luck_uses_per_day_from_feats(feats) {
        explanations.push(ComputationExplanation {
            id: "feat.arg_standalone.defiant_luck_uses_per_day".to_owned(),
            value: uses,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:VAR|DefiantLuckTimes|1 each
                //   Unlike ARG's other per-day variables, DEFINE:DefiantLuckTimes|0 sits on the
                //   Defiant Luck record itself, so this budget is wholly feat-determined rather
                //   than an increment to a racial pool this engine does not model.
                "Defiant Luck (ARG) is usable {uses} time(s) per day -- after a natural 1 on a \
                 saving throw, or a confirmed critical hit against this character, that roll may be \
                 rerolled. Defiant Luck and Bestow Luck both write the one corpus variable \
                 DefiantLuckTimes and neither states a literal, printing the total through a %1 \
                 substitution token instead. No per-use consumption is tracked -- this is the size \
                 of a daily pool, the same way Stunning Fist's uses per day grounds"
            ),
        });
    }

    // Fiend Sight. Its `BONUS:VAR|FiendSightTier|1` is a pick counter, not the
    // magnitude; the magnitude is on the record's own VISION token and prose.
    let fiend_sight_darkvision = feat_effects::fiend_sight_darkvision_feet_from_feats(feats);
    if fiend_sight_darkvision != 0 {
        explanations.push(ComputationExplanation {
            id: "feat.arg_standalone.fiend_sight_darkvision_feet".to_owned(),
            value: fiend_sight_darkvision,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   by PREVARLT:FiendSightTier,2
                //   The record states the range absolutely and twice -- VISION:Darkvision (120')
                //   and its BENEFIT prose -- over the 60-foot base its own
                //   PREVISION:1,Darkvision=60 prerequisite fixes, so this is a doubling of a known
                //   base rather than a free-floating claim; the same reconciliation the Deepsight
                //   record performs, which reports its own 60-to-120 step as the increment its
                //   BONUS:VISION token actually is.
                "Fiend Sight (ARG) improves darkvision to {fiend_sight_darkvision} feet and grants \
                 low-light vision. This engine models no vision numerically anywhere, so the range \
                 grounds standalone. The feat's own tier counter is a pick counter \
                 (STACK:YES MULT:YES, capped at two) whose only consumer is the record's own second \
                 pick, granting the see-in-darkness universal monster ability -- a capability with \
                 no magnitude, and no further range"
            ),
        });
    }

    let gnome_weapon_attack = feat_effects::gnome_weapon_focus_attack_bonus_from_feats(feats);
    if gnome_weapon_attack != 0 {
        explanations.push(ComputationExplanation {
            id: "feat.arg_standalone.gnome_weapon_focus_attack_bonus".to_owned(),
            value: gnome_weapon_attack,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:WEAPONPROF=TYPE.Gnome|TOHIT|1
                "Gnome Weapon Focus (ARG) grants a +{gnome_weapon_attack} bonus on attack rolls with \
                 gnome weapons -- weapons with \"gnome\" in the title. Deliberately NOT added to any \
                 attack total: the bonus is scoped to a weapon TYPE and this engine's per-weapon \
                 attack totals carry no gnome facet to test, unlike Weapon Focus whose target is a \
                 specific weapon the player records"
            ),
        });
    }

    // Pathfinder Unchained's stamina pools. Gated on a real computed chassis:
    // Combat Stamina's pool is BAB + CON, and the `0` the chassis fallback
    // substitutes for an unsupported posture is not a base attack bonus.
    let constitution = ability_modifiers.constitution;
    if let Some(facts) = chassis_supported
        .then(|| feat_effects::stamina_pool_facts_from_feats(feats, base_attack_bonus, constitution))
        .flatten()
    {
        let extra = facts.extra_stamina_picks;
        explanations.push(ComputationExplanation {
            id: "feat.pu_standalone.stamina_pool".to_owned(),
            value: facts.primary,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   !PREABILITY:3 token
                "Combat Stamina (Pathfinder Unchained) grants a stamina pool of {} points: base \
                 attack bonus (+{base_attack_bonus}) + Constitution modifier ({constitution:+}), \
                 transcribed from the corpus's own pool formula, plus {extra} Extra Stamina pick(s) \
                 at +3 each (STACK:YES MULT:YES, capped at three by the feat's own). No stamina \
                 expenditure, combat trick or per-round state is modelled here, so the pool grounds \
                 standalone as a size rather than as a running resource",
                facts.primary
            ),
        });
        if let Some(secondary) = facts.secondary {
            explanations.push(ComputationExplanation {
                id: "feat.pu_standalone.secondary_stamina_pool".to_owned(),
                value: secondary,
                detail: format!(
                    // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                    //   BONUS:VAR|SecondaryStaminaPool|CON
                    "Push the Limits (Pathfinder Unchained) grants a SECOND stamina pool of \
                     {secondary} points, equal to the Constitution modifier. Reported separately \
                     rather than added into the primary pool above: its points are spendable only at \
                     0 primary stamina or while fatigued, conditions this engine does not model, so \
                     summing the two would overstate what the character can spend"
                ),
            });
        }
    }

    // Armor of the Pit's OTHER half, named rather than silently dropped.
    // Its `BONUS:VAR|Cold/Electricity/FireResistanceBonus|5|TYPE=Resistance`
    // token names three energy types unconditionally; the rule grants two of
    // three, chosen by the player from the ones they do not already resist.
    // Grounding three would overstate it and grounding a guessed two would
    // fabricate the choice, so this states the gap instead of inventing a
    // number -- and only for the character the branch actually applies to.
    if feat_identity::holds(feats, "Armor of the Pit") && character_has_tiefling_scaled_skin(input) {
        explanations.push(ComputationExplanation {
            id: "feat.arg_standalone.armor_of_the_pit_scaled_skin_branch".to_owned(),
            value: 0,
            detail: "Armor of the Pit (ARG) on a character who took the Scaled Skin alternate \
                     racial trait grants resistance 5 to TWO of cold, electricity and fire -- \
                     the two they do not already resist -- INSTEAD of its +2 natural armor \
                     bonus, which is therefore withheld from this character's Armor Class. \
                     Which two is a player choice this engine records nowhere, so no resistance \
                     value is claimed here (+0) rather than guessing a pair or asserting all \
                     three, which is what the feat's corpus token literally says"
                .to_owned(),
        });
    }
}

/// Turns a human-readable name into a stable explanation-id segment:
/// lowercase, ASCII alphanumerics kept, every other run of characters collapsed
/// to a single `_`, with no leading or trailing separator.
///
/// The existing records here build their slugs with
/// `.to_lowercase().replace(' ', "_")`, which is fine for the plain skill names
/// they carry but produces `craft_(alchemy)` and `scavenger's_eye` for ARG's,
/// putting punctuation into an id other code matches on. This collapses those
/// to `craft_alchemy` and `scavengers_eye`. Deliberately a new helper rather
/// than a rewrite of the shipped call sites: changing an id already asserted by
/// tests and read by the desktop layer is a separate, larger change than this
/// one is scoped for.
///
/// **Corrected 2026-08-01.** The `scavengers_eye` half of that paragraph was
/// false when it was written: an apostrophe took the `else` arm, so
/// `Scavenger's Eye` produced `scavenger_s_eye` and the sheet read
/// "Scavenger S Eye". [`is_intraword_punctuation`] now swallows it, and
/// `tests/sd27_apostrophes_do_not_split_id_slugs.rs` pins the corrected id
/// against the live pipeline so the doc comment and the behaviour cannot
/// disagree again.
pub(super) fn slugify_id_segment(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    let mut pending_separator = false;
    for character in name.chars() {
        if character.is_ascii_alphanumeric() {
            if pending_separator && !out.is_empty() {
                out.push('_');
            }
            pending_separator = false;
            out.push(character.to_ascii_lowercase());
        } else if is_intraword_punctuation(character) {
            // Swallowed, not separated. See `is_intraword_punctuation`.
        } else {
            pending_separator = true;
        }
    }
    out
}

/// `selected_choices` is scanned regardless of chassis support.
pub(super) fn ground_standalone_feat_skill_facts(
    input: &CharacterInput,
    explanations: &mut Vec<ComputationExplanation>,
) {
    for fact in
        crate::rules_core::feat_effects::standalone_skill_facts_from_feats(&effective_character_feats(input))
    {
        let skill_slug = fact.skill_name.to_lowercase().replace(' ', "_");
        explanations.push(ComputationExplanation {
            id: format!("feat.standalone_skill_bonus.{skill_slug}"),
            value: fact.bonus,
            detail: format!(
                "{} grants a +{} bonus on {} checks. {} is not among the three skills \
                 compute_selected_skill_modifiers tracks (Climb/Intimidate/Swim), so this \
                 grounds as a standalone flat record, not wired into any skill total this \
                 codebase computes",
                fact.feat_key, fact.bonus, fact.skill_name, fact.skill_name
            ),
        });
    }

    // SD-27 (`decisions.md` §24/§28, 2026-07-31): the Advanced Race Guide's
    // five `BONUS:SKILL` feats. Their ids carry the FEAT as well as the skill,
    // unlike the CRB/APG records above, because ARG genuinely puts three
    // different feats on one skill -- Seen and Unseen's +2 Stealth, Angelic
    // Flesh's -2 Stealth, and CRB Stealthy's +2, which already owns
    // `feat.standalone_skill_bonus.stealth`. A skill-only id would collapse
    // them.
    for fact in crate::rules_core::feat_effects::arg_skill_facts_from_feats(
        &effective_character_feats(input),
    ) {
        let feat_slug = slugify_id_segment(fact.feat_key);
        let skill_slug = slugify_id_segment(fact.skill_name);
        let integrated = fact.skill_name == "Climb";
        explanations.push(ComputationExplanation {
            id: format!("feat.arg_skill_bonus.{feat_slug}.{skill_slug}"),
            value: fact.bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   BONUS:SKILL|<skill>|<bonus>
                "{} (ARG) grants a {:+} bonus on {} checks, transcribed from its corpus \
                 skill-bonus entry and confirmed against its own printed benefit. {}",
                fact.feat_key,
                fact.bonus,
                fact.skill_name,
                if integrated {
                    "Climb IS one of the three skills compute_selected_skill_modifiers computes, \
                     so this value is already summed into skill.selected_modifier.climb -- this \
                     record names the contributor, it is not a second, separate bonus"
                } else {
                    "This skill is not among the three compute_selected_skill_modifiers tracks \
                     (Climb/Intimidate/Swim), so it grounds as a standalone flat record and is \
                     not wired into any skill total this codebase computes"
                }
            ),
        });
    }

    for fact in crate::rules_core::feat_effects::skill_focus_facts_from_choices(
        &effective_character_feats(input),
        &input.chosen.selected_choices,
    ) {
        let skill_slug = fact.skill_name.to_lowercase().replace(' ', "_");
        explanations.push(ComputationExplanation {
            id: format!("feat.skill_focus_bonus.{skill_slug}"),
            value: fact.bonus,
            detail: format!(
                "Skill Focus grants a +{} bonus on {} checks (the player-chosen target, \
                 recorded via choice:skill_focus_target). This grounds as a standalone flat \
                 record; it is not wired into any skill total this codebase computes",
                fact.bonus, fact.skill_name
            ),
        });
    }

    for fact in crate::rules_core::feat_effects::master_craftsman_facts_from_choices(
        &effective_character_feats(input),
        &input.chosen.selected_choices,
    ) {
        // SD-36 Epic E PC8-1: `slugify_id_segment`, not the naive
        // `.to_lowercase().replace(' ', "_")` every sibling record here still uses -- a
        // Craft/Profession skill name carries its own parenthetical subtype
        // ("Craft (Armor)", "Profession (Siege Engineer)"), and the naive slug put the literal
        // `(`/`)` characters into a wire-format explanation id.
        let skill_slug = slugify_id_segment(&fact.skill_name);
        explanations.push(ComputationExplanation {
            id: format!("feat.master_craftsman_bonus.{skill_slug}"),
            value: fact.bonus,
            detail: format!(
                // Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):
                //   The feat's own PRESKILL:1,TYPE.Craft=5,TYPE.Profession=5 prerequisite is NOT
                //   asserted here -- prerequisite validation is feat_prereqs' job, and this
                //   engine's deterministic skill posture pins ranks at 1, so no character it
                //   currently composes could legally hold the feat.
                "Master Craftsman grants a +{} bonus on {} checks (the player-chosen Craft or \
                 Profession target, recorded via choice:master_craftsman_target). This grounds as a \
                 standalone flat record; Craft and Profession are not among the three skills \
                 compute_selected_skill_modifiers tracks (Climb/Intimidate/Swim), so there is no \
                 total to layer it onto. That is a gap in what the engine represents, not a reason \
                 to withhold a verified magnitude. Its second token (MasterCraftsmanRanks, \
                 substituting skill ranks for caster level when crafting magic items) stays \
                 ungrounded: this codebase models no item creation to substitute into",
                fact.bonus, fact.skill_name
            ),
        });
    }
}

/// Return the list of unmet conditions for the exact deterministic selected-skill
/// posture. An empty list means the posture is fully supported.
///
/// The bounded posture requires a Fighter level 1–3 chassis, exactly the three
/// selected class skills (Climb, Intimidate, Swim) each at rank 1 with no other
/// skill allocations, and the grounded Chain Shirt armor-check posture that the
/// Climb/Swim totals depend on.
pub(super) fn unmet_selected_skill_posture_conditions(input: &CharacterInput) -> Vec<String> {
    let allocations = &input.chosen.skill_allocations;
    let mut unmet = Vec::new();

    // SD-21 E6b.1: widened from a Fighter-only gate to the same dispatch-supported
    // class set `compute_class_chassis` / `compute_total_saves` already recognize
    // (`has_supported_class_chassis`), mirroring `unmet_combat_posture_conditions`.
    if !has_supported_class_chassis(input) {
        unmet.push(format!(
            "missing a supported chassis (needs one of: {})",
            supported_class_chassis_description()
        ));
    }

    let expected = [CLIMB_SKILL_ID, INTIMIDATE_SKILL_ID, SWIM_SKILL_ID];
    for skill_id in expected {
        require_selected_skill_rank(allocations, skill_id, &mut unmet);
    }

    // Refuse any widening beyond exactly the three selected skills.
    for allocation in allocations {
        if !expected.contains(&allocation.skill_id.as_str()) {
            unmet.push(format!(
                "skill allocation {} is outside the selected Climb/Intimidate/Swim slice",
                allocation.skill_id
            ));
        }
    }

    // Climb and Swim totals depend on the grounded Chain Shirt armor-check posture.
    require_active_state(
        input,
        CHAIN_SHIRT_ITEM_ID,
        ActiveState::EquippedActive,
        &mut unmet,
    );

    unmet
}

/// Record an unmet condition unless the named skill is allocated exactly the
/// supported deterministic rank.
pub(crate) fn require_selected_skill_rank(
    allocations: &[SkillAllocation],
    skill_id: &str,
    unmet: &mut Vec<String>,
) {
    let actual = allocations
        .iter()
        .find(|a| a.skill_id == skill_id)
        .map(|a| a.ranks);
    if actual != Some(SELECTED_SKILL_RANK) {
        unmet.push(format!(
            "{skill_id} must be allocated rank {SELECTED_SKILL_RANK} for the selected-skill slice, got {actual:?}"
        ));
    }
}

/// Folds an equipment `item_id` (`"item:longsword"`) or a
/// `weapon_tables` key (`"Longsword"`) down to one comparable identity:
/// lowercase, alphanumeric only, with any `item:` prefix dropped.
///
/// **This is deliberately a SECOND instance of the same idea as the
/// desktop app's `normalizeFeatIdentity`, not a shared helper**, and the
/// reason is structural rather than an oversight: that one is TypeScript
/// in `apps/desktop/src/characterHub/featsTabModel.ts`, folding feat
/// identities for the sheet; this is Rust in `rules_core`, folding weapon
/// identities for the engine. Different languages, different processes,
/// no seam between them to share across. If a third instance ever appears
/// *within* Rust, that one should be shared with this rather than added.
pub(super) fn normalize_weapon_identity(raw: &str) -> String {
    let without_prefix = raw.strip_prefix("item:").unwrap_or(raw);
    without_prefix
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

/// Resolves an equipment `item_id` to its real corpus weapon stat block,
/// or `None` when the item is not a weapon this engine has ingested
/// (armor, gear, or a weapon outside the CRB table).
pub(crate) fn equipped_weapon_stat_block(
    item_id: &str,
) -> Option<&'static weapon_tables::WeaponTableEntry> {
    let wanted = normalize_weapon_identity(item_id);
    weapon_tables::WEAPON_TABLE
        .iter()
        .find(|entry| normalize_weapon_identity(entry.key) == wanted)
}

/// PF1's attack-roll penalty for using a weapon you are not proficient
/// with, from the game system's own machine-readable constant:
/// `WEAPONNONPROFPENALTY:-4` in
/// `system/gameModes/Pathfinder/miscinfo.lst:193`. Not transcribed from
/// rulebook prose or memory.
pub(crate) const WEAPON_NONPROFICIENCY_ATTACK_PENALTY: i16 = -4;

/// Whether this character is proficient with `weapon`, or `None` when at
/// least one class in the mix has no answer -- neither a static
/// `CLASS_WEAPON_PROFICIENCIES` row nor a Known answer from the converted
/// record -- and the question therefore cannot be answered honestly.
/// [`character_weapon_proficiency`] is the same verdict with its printed
/// conditions and its Unknown reason; this is its boolean projection.
///
/// **`None` means "unknown", never "not proficient"** --
/// `class_weapon_proficiency`'s own contract. A caller that collapsed
/// `None` into `false` would silently invent a -4 penalty for a class
/// whose grants were merely un-ingested, which is the same
/// fabricated-number failure mode this whole fix exists to remove, just
/// pointing the other way.
///
/// v0.6 alpha swarm, risks item #89 / tasks #80+#86 (2026-07-29); feat
/// grants added by the combat feat-effects slice (2026-07-29); the
/// converted-record fallback by SD-36 Epic F step 2.
pub(crate) fn character_is_proficient_with(
    input: &CharacterInput,
    weapon: &weapon_tables::WeaponTableEntry,
) -> Option<bool> {
    character_weapon_proficiency(input, weapon).proficient()
}

/// One weapon's proficiency verdict for a whole character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum WeaponProficiencyVerdict {
    /// Decided. `printed` carries every converted grant the class-level
    /// facts could not decide (a gated grant, a player's pick, a deity's
    /// weapon), each with its condition: printed on the sheet, never
    /// counted (paper-sheet doctrine).
    Known { proficient: bool, printed: Vec<String> },
    /// Not decidable from what the engine holds; `reason` names why, per
    /// class, and reaches the claim-blocking diagnostic verbatim.
    Unknown { reason: String },
}

impl WeaponProficiencyVerdict {
    pub(crate) fn proficient(&self) -> Option<bool> {
        match self {
            WeaponProficiencyVerdict::Known { proficient, .. } => Some(*proficient),
            WeaponProficiencyVerdict::Unknown { .. } => None,
        }
    }

    /// `" Printed, not counted: ..."` for the explanation text, or empty.
    pub(crate) fn printed_detail(&self) -> String {
        match self {
            WeaponProficiencyVerdict::Known { printed, .. } if !printed.is_empty() => {
                format!(" Printed, not counted: {}.", printed.join("; "))
            }
            _ => String::new(),
        }
    }
}

/// Whether this character is proficient with `weapon`, with the words the
/// sheet prints and, when it cannot be decided, the reason.
///
/// Multiclass follows PF1's actual rule: proficiency is the UNION across
/// classes, so a Fighter/Wizard is proficient with everything Fighter is.
/// A single non-proficient class in the mix must not remove a
/// proficiency another class genuinely grants. A class with no answer
/// makes the verdict Unknown only when no other class grants the weapon
/// (SD-36 F3b: before, it made the whole verdict Unknown even beside a
/// class that grants it, which a union cannot be).
///
/// **Feats are checked before classes, and before the unknown-class
/// bail-out.** The three CRB proficiency-granting feats (Simple/Martial/
/// Exotic Weapon Proficiency) grant proficiency outright, so a character
/// holding one has a KNOWN answer for that weapon even if some class in
/// the mix has no ingested record. Checking classes first and returning
/// Unknown would throw away a fact the input states explicitly.
///
/// **Per class, one mechanical rule** (SD-36 Epic F §3.4): the static
/// `weapon_tables::class_weapon_proficiency` row first (the 42 rows answer
/// exactly as before); otherwise the converted record
/// (`class_proficiency_sheet_rules::class_weapon_proficiency_view`), matched
/// per weapon by [`converted_view_covers_weapon`]; the reader's `Unknown`
/// stays Unknown and carries its reason.
pub(crate) fn character_weapon_proficiency(
    input: &CharacterInput,
    weapon: &weapon_tables::WeaponTableEntry,
) -> WeaponProficiencyVerdict {
    use crate::rules_core::pilot_compute::class_proficiency_sheet_rules::{
        class_weapon_proficiency_view, ProficiencyAnswer,
    };
    let grants = crate::rules_core::feat_effects::weapon_proficiency_grants_from_feats(
        &effective_character_feats(input),
        &input.chosen.selected_choices,
    );
    let granted = || WeaponProficiencyVerdict::Known { proficient: true, printed: Vec::new() };
    // Simple Weapon Proficiency grants the whole Simple tier, exactly as a
    // class's own `AUTO:WEAPONPROF|TYPE=Simple` does.
    if grants.grants_simple_tier
        && weapon.proficiency == Some(weapon_tables::WeaponProficiency::Simple)
    {
        return granted();
    }
    // Martial/Exotic Weapon Proficiency name one weapon each. The recorded
    // target is a display name (`weapon:Longsword`), so it joins on the
    // same normalized identity the equipment ids use -- NOT on
    // `proficiency_name`, which is a different namespace and would fail for
    // more than half the table.
    if grants
        .named_weapons
        .iter()
        .any(|name| normalize_weapon_identity(name) == normalize_weapon_identity(weapon.key))
    {
        return granted();
    }

    let mut any_proficient = false;
    let mut printed: Vec<String> = Vec::new();
    let mut unknown: Vec<String> = Vec::new();
    for class_level in &input.chosen.class_levels {
        if let Some(proficiency) = weapon_tables::class_weapon_proficiency(&class_level.class_id) {
            if weapon_tables::class_is_proficient_with(proficiency, weapon) {
                any_proficient = true;
            }
            continue;
        }
        let slug = crate::rules_core::sheet_rule::id_slug(&class_level.class_id);
        match class_weapon_proficiency_view(&slug, class_level.level) {
            ProficiencyAnswer::Unknown { reason } => {
                unknown.push(format!("{} level {}: {reason}", class_level.class_id, class_level.level));
            }
            ProficiencyAnswer::Known(view) => {
                // SD-36 F1c-3 (D6): a pick of one weapon from a named list is decided by the
                // character's own recorded choice, and the sheet prints the choice.
                let mut pick_undecided: Vec<String> = Vec::new();
                for pick in &view.weapon_picks {
                    let chosen: Vec<&str> = input
                        .chosen
                        .selected_choices
                        .iter()
                        .filter(|c| c.choice_set_id == pick.choice)
                        .map(|c| c.selection_id.strip_prefix(crate::rules_core::feat_effects::WEAPON_SELECTION_PREFIX).unwrap_or(&c.selection_id))
                        .collect();
                    if chosen.is_empty() {
                        printed.push(format!(
                            "{} (converted record): {}: one weapon of the player's choice ({} options), not recorded",
                            class_level.class_id,
                            pick.label,
                            pick.options.len()
                        ));
                        if pick_offers_weapon(pick, weapon) {
                            pick_undecided.push(format!(
                                "{} level {}: {} picks one weapon of {} options (this weapon among them) and the character records no choice under {}",
                                class_level.class_id,
                                class_level.level,
                                pick.label,
                                pick.options.len(),
                                pick.choice
                            ));
                        }
                        continue;
                    }
                    printed.push(format!(
                        "{} (converted record): {}: proficient with the chosen weapon ({})",
                        class_level.class_id,
                        pick.label,
                        chosen.join(", ")
                    ));
                    if pick_offers_weapon(pick, weapon)
                        && chosen.iter().any(|c| normalize_weapon_identity(c) == weapon_identity(weapon))
                    {
                        any_proficient = true;
                    }
                }
                if converted_view_covers_weapon(&view, weapon) {
                    any_proficient = true;
                } else if !pick_undecided.is_empty() {
                    unknown.extend(pick_undecided);
                } else if !view.unresolved_picks.is_empty() {
                    // The closure is incomplete: an unseen pick could cover this weapon.
                    unknown.push(format!(
                        "{} level {}: {}",
                        class_level.class_id,
                        class_level.level,
                        view.unresolved_picks.join("; ")
                    ));
                }
                printed.extend(
                    view.printed_conditions.iter().map(|c| format!("{} (converted record): {c}", class_level.class_id)),
                );
            }
        }
    }
    // SD-36 F3b: a union is decided by any one member that grants it -- a class
    // with no answer can only ADD a proficiency, never take one away, so it
    // leaves the verdict Unknown only when no class grants this weapon.
    if !unknown.is_empty() && !any_proficient {
        return WeaponProficiencyVerdict::Unknown { reason: unknown.join("; ") };
    }
    WeaponProficiencyVerdict::Known { proficient: any_proficient, printed }
}

/// The identity a weapon is compared on against a converted proficiency name: its
/// `proficiency_name` when it has one, else its record key -- normalized.
fn weapon_identity(weapon: &weapon_tables::WeaponTableEntry) -> String {
    normalize_weapon_identity(weapon.proficiency_name.unwrap_or(weapon.key))
}

/// Whether a converted weapon pick's options include `weapon`.
fn pick_offers_weapon(
    pick: &crate::rules_core::pilot_compute::class_proficiency_sheet_rules::WeaponPickView,
    weapon: &weapon_tables::WeaponTableEntry,
) -> bool {
    let wanted = weapon_identity(weapon);
    pick.options.iter().any(|o| normalize_weapon_identity(o) == wanted)
}

/// Whether a class's CONVERTED proficiency answer covers `weapon`. Only
/// counted grants decide it; `printed_conditions` (a gated grant, a
/// player's pick, a deity's favored weapon) never do -- the engine holds no
/// deity fact, and a pick is printed with its choice, not assumed.
///
/// - tier: the weapon's `proficiency` facet;
/// - named: the weapon's `proficiency_name` (the `PROFICIENCY:WEAPON`
///   namespace a converted `ProfRef::Weapon` carries), case-insensitively
///   as PCGen matches (`pure_legion_enforcer` names `falchion`);
/// - group: the weapon's `Weapon Group <x>` facet;
/// - all-of: every conjunct holds -- a tier, `Melee` or `Ranged`. The reader
///   already refuses any other conjunct as Unknown, and every converted
///   conjunction is pinned answerable
///   (`every_converted_weapon_all_of_conjunct_is_answerable_by_the_weapon_table`);
///   an unanswerable conjunct here is therefore never counted;
/// - set: membership by the weapon's `proficiency_name`, falling back to its
///   record `key` -- the converted set members are PCGen proficiency names
///   (`Sword (Short)`, `Pick (Light)`), not record keys (`Short Sword`);
///   compared on the normalized identity, as the named branch compares
///   case-insensitively.
pub(crate) fn converted_view_covers_weapon(
    view: &crate::rules_core::pilot_compute::class_proficiency_sheet_rules::ClassWeaponProficiencyView,
    weapon: &weapon_tables::WeaponTableEntry,
) -> bool {
    use weapon_tables::WeaponProficiency;
    let tier_of = |tag: &str| match tag {
        "Simple" => Some(WeaponProficiency::Simple),
        "Martial" => Some(WeaponProficiency::Martial),
        "Exotic" => Some(WeaponProficiency::Exotic),
        _ => None,
    };
    if weapon.proficiency.is_some_and(|tier| view.tiers.contains(&tier)) {
        return true;
    }
    if weapon
        .proficiency_name
        .is_some_and(|name| view.named.iter().any(|n| n.eq_ignore_ascii_case(name)))
    {
        return true;
    }
    if weapon
        .weapon_group
        .is_some_and(|group| view.groups.iter().any(|g| g.eq_ignore_ascii_case(group)))
    {
        return true;
    }
    let conjunct_holds = |tag: &String| match tag.as_str() {
        "Melee" => weapon.is_melee,
        "Ranged" => weapon.is_ranged,
        other => tier_of(other).is_some_and(|tier| weapon.proficiency == Some(tier)),
    };
    if view.all_of.iter().any(|tags| !tags.is_empty() && tags.iter().all(conjunct_holds)) {
        return true;
    }
    let wanted = normalize_weapon_identity(weapon.proficiency_name.unwrap_or(weapon.key));
    view.sets
        .iter()
        .any(|set| set.members.iter().any(|member| normalize_weapon_identity(member) == wanted))
}

/// Grounds the per-weapon combat surfaces for every equipped weapon that
/// resolves against the ingested CRB weapon table (task #72, stages 2-3):
/// an attack total, the feat-granted damage bonus, and the threat range.
///
/// **Additive by design.** This does NOT touch
/// `combat.baseline_melee_attack_bonus`, which is a single-fixture GE-06
/// special case hardcoded to the Longsword (its own detail string says
/// so) and asserted against by a large number of existing suites.
/// Generalising that one -- widening its posture gate and driving its
/// hardcoded `WEAPON_FOCUS_TO_HIT_BONUS` off the real
/// `choice:weapon_focus_target` chooser -- is genuinely separate surgery,
/// deferred as its own task (#80). Until then the two coexist.
///
/// **The five weapon feats do not all land on the same surface**, which
/// their shared naming actively disguises. Corpus payload slots decide it:
/// Weapon Focus and Greater Weapon Focus write `TOHIT` (attack), Weapon
/// Specialization and Greater Weapon Specialization write `DAMAGE`, and
/// Improved Critical writes `CRITRANGEDOUBLE` (a multiplier on the
/// weapon's own threat range, not a flat bonus). Folding all five into the
/// attack total -- the shape stage 3 was originally briefed as -- would
/// have inflated every attack total by up to `+4` of damage bonus.
///
/// **The attack total is the MELEE-use total.** Its governing ability is
/// Strength (or Dexterity via Weapon Finesse), which is correct for melee
/// and wrong for a ranged attack, where PF1 uses Dexterity. Eleven of this
/// table's rows are both melee and ranged (Dagger, Trident, Club...), so
/// there is no clean per-weapon answer without a wield/mode state this
/// engine does not record -- the same gap the damage record below names.
/// Deliberately left as-is rather than silently switching on `is_ranged`,
/// which would change the meaning of an existing shipped record for the
/// thrown weapons without being able to say which mode it now describes.
/// That also keeps Point-Blank Shot deferred rather than half-grounded --
/// see this slice's report.
///
/// **Damage is grounded as the feat contribution only, not a damage
/// total.** A real total would need the Strength-to-damage multiplier,
/// which is 1x one-handed, 1.5x two-handed and 0.5x off-hand -- and how a
/// weapon is being wielded is not recorded anywhere in
/// `equipment_selections`. Emitting a "damage total" would mean inventing
/// that wield state, so this grounds the flat feat bonus, which is real
/// and complete on its own, and says plainly what it excludes.
pub(super) fn ground_per_weapon_combat_totals(
    input: &CharacterInput,
    ability_modifiers: &AbilityModifiers,
    base_attack_bonus: i16,
    explanations: &mut Vec<ComputationExplanation>,
    diagnostics: &mut Vec<ComputationDiagnostic>,
) {
    use crate::rules_core::feat_effects;

    // SD-36 Epic E PC8-2: PF1 Table 8-1's size modifier applies to attack rolls the same way
    // `compute_combat_baseline` already applies it to its GE-06 fixture posture (`decisions.md`
    // §28 defect 1) -- every Small/Large race got a wrong per-weapon attack total here before,
    // silently missing the size term that same sibling function has carried since SD-27.
    let size_attack_modifier = combat_size_modifiers(input, diagnostics).armor_class_and_attack;

    let strength_modifier = ability_modifiers.strength;
    let dexterity_modifier = ability_modifiers.dexterity;
    let feats = effective_character_feats(input);
    let has_weapon_finesse = feat_effects::holds_weapon_finesse(&feats);
    let focus_facts =
        feat_effects::weapon_focus_facts_from_choices(&feats, &input.chosen.selected_choices);
    let specialization_facts = feat_effects::weapon_specialization_facts_from_choices(
        &feats,
        &input.chosen.selected_choices,
    );
    let improved_critical_targets = feat_effects::improved_critical_targets_from_choices(
        &feats,
        &input.chosen.selected_choices,
    );

    let mut grounded: Vec<&str> = Vec::new();

    for selection in &input.chosen.equipment_selections {
        if selection.active_state != ActiveState::EquippedActive {
            continue;
        }
        let Some(weapon) = equipped_weapon_stat_block(&selection.item_id) else {
            continue;
        };
        if grounded.contains(&weapon.key) {
            continue;
        }
        grounded.push(weapon.key);

        let slug = normalize_weapon_identity(weapon.key);
        let names_this_weapon =
            |name: &str| normalize_weapon_identity(name) == normalize_weapon_identity(weapon.key);

        // Attack: base attack bonus + the governing ability + the two Focus
        // feats, each counted exactly once. The producer has already summed
        // Greater onto base, so this reads one resolved bonus per weapon
        // rather than adding two facts together here.
        let focus_bonus = focus_facts
            .iter()
            .find(|fact| names_this_weapon(&fact.weapon_name))
            .map_or(0, |fact| fact.attack_bonus);

        // Weapon Finesse (combat feat-effects slice, 2026-07-29). Corpus
        // token: `BONUS:COMBAT|TOHIT.Finesseable|
        // ((max(STR,DEX)-STR)+SHIELDACCHECK)|TYPE=NotRanged`. Applied as
        // written -- as a swap to `max(STR, DEX)`, not a flat bonus -- and
        // only for a weapon actually carrying the corpus's own
        // `Finesseable` facet.
        //
        // The token's `SHIELDACCHECK` term (a worn shield's armor check
        // penalty, which the feat's own BENEFIT text calls out: "If you
        // carry a shield, its armor check penalty applies to your attack
        // rolls") is NOT applied: no equipped shield contributes an armor
        // check penalty anywhere in this engine's equipment model, so
        // there is no verified number to subtract. Naming it rather than
        // inventing one.
        let finesse_applies = has_weapon_finesse && weapon_tables::weapon_is_finesseable(weapon);
        let attack_ability_modifier =
            if finesse_applies { strength_modifier.max(dexterity_modifier) } else { strength_modifier };
        let finesse_detail = if !has_weapon_finesse {
            String::new()
        } else if !finesse_applies {
            format!(
                " Weapon Finesse is held but the {} carries no Finesseable facet in the corpus, so \
                 Strength still governs.",
                weapon.key
            )
        } else if attack_ability_modifier == strength_modifier {
            format!(
                " Weapon Finesse applies to this weapon but changes nothing here: Dexterity \
                 ({dexterity_modifier:+}) does not exceed Strength ({strength_modifier:+}), and \
                 the corpus token is max(STR,DEX)-STR, not a flat bonus."
            )
        } else {
            format!(
                " Weapon Finesse applies: Dexterity ({dexterity_modifier:+}) replaces Strength \
                 ({strength_modifier:+}) on attack rolls with this finesseable weapon. The \
                 token's shield armor-check-penalty term is not applied -- no equipped shield \
                 contributes one in this engine."
            )
        };

        // The -4 nonproficiency penalty, the same one
        // `compute_combat_baseline` applies to its hardcoded Longsword.
        // Without this a Wizard who equipped a Greatsword read a full
        // attack bonus for it. Feat grants are already folded in by
        // `character_is_proficient_with`, so Martial/Exotic Weapon
        // Proficiency naming this weapon removes the penalty here.
        let full_verdict = character_weapon_proficiency(input, weapon);
        let proficiency_verdict = full_verdict.proficient();
        let nonproficiency_penalty = match proficiency_verdict {
            Some(false) => WEAPON_NONPROFICIENCY_ATTACK_PENALTY,
            // `None` is "unknown", never "not proficient" -- refuse to
            // invent a penalty for a class whose grants are merely
            // un-ingested, and say so instead.
            Some(true) | None => 0,
        };
        let proficiency_detail = match proficiency_verdict {
            Some(false) => format!(
                " {WEAPON_NONPROFICIENCY_ATTACK_PENALTY} nonproficiency penalty: nothing this \
                 character holds grants proficiency with this weapon -- neither a class grant nor \
                 Simple/Martial/Exotic Weapon Proficiency naming it."
            ),
            Some(true) => " The character is proficient with this weapon (class grant or a \
                 Simple/Martial/Exotic Weapon Proficiency feat naming it), so no nonproficiency \
                 penalty applies."
                .to_owned(),
            None => format!(
                " Proficiency is UNRESOLVED ({}), so no nonproficiency penalty is applied in \
                 either direction and this total is not claimed to account for one.",
                match &full_verdict {
                    WeaponProficiencyVerdict::Unknown { reason } => reason.as_str(),
                    WeaponProficiencyVerdict::Known { .. } => "",
                }
            ),
        };
        let proficiency_detail = format!("{proficiency_detail}{}", full_verdict.printed_detail());

        let attack_total = base_attack_bonus
            + attack_ability_modifier
            + focus_bonus
            + nonproficiency_penalty
            + size_attack_modifier;
        let focus_detail = if focus_bonus == 0 {
            " No Weapon Focus or Greater Weapon Focus names this weapon, so no feat bonus applies to \
             the attack roll."
                .to_owned()
        } else {
            format!(
                " Weapon Focus / Greater Weapon Focus name this weapon, adding {focus_bonus:+} to \
                 the attack roll."
            )
        };
        explanations.push(ComputationExplanation {
            id: format!("combat.weapon_attack_bonus.{slug}"),
            value: attack_total,
            detail: format!(
                "Attack bonus with the equipped {}: base attack bonus (+{base_attack_bonus}) + \
                 governing ability modifier ({attack_ability_modifier:+}) + weapon feats \
                 ({focus_bonus:+}) + nonproficiency penalty ({nonproficiency_penalty}) + size \
                 modifier ({size_attack_modifier:+}) = \
                 {attack_total}.{focus_detail}{finesse_detail}{proficiency_detail} Only the Focus \
                 feats reach this total -- the Specialization feats are damage and Improved \
                 Critical is threat range, both grounded separately. Separate from \
                 combat.baseline_melee_attack_bonus, which stays a Longsword-specific GE-06 \
                 fixture total until task #80 generalises it",
                weapon.key
            ),
        });

        // Damage: the flat feat bonus only. Grounded even at zero, so a
        // weapon with no Specialization reads as an explicit "correctly
        // absent" rather than a missing record.
        let damage_bonus = specialization_facts
            .iter()
            .find(|fact| names_this_weapon(&fact.weapon_name))
            .map_or(0, |fact| fact.damage_bonus);
        let damage_detail = if damage_bonus == 0 {
            "No Weapon Specialization or Greater Weapon Specialization names this weapon, so no \
             feat bonus applies to its damage rolls."
                .to_owned()
        } else {
            format!(
                "Weapon Specialization / Greater Weapon Specialization name this weapon, adding \
                 {damage_bonus:+} to every damage roll with it."
            )
        };
        explanations.push(ComputationExplanation {
            id: format!("combat.weapon_damage_bonus.{slug}"),
            value: damage_bonus,
            detail: format!(
                "Feat damage bonus with the equipped {}: {damage_bonus:+}. {damage_detail} This is \
                 the feat contribution alone, NOT a damage total: the weapon's own {} damage die \
                 and the Strength-to-damage contribution are excluded, because Strength applies at \
                 1x one-handed, 1.5x two-handed and 0.5x off-hand and this engine does not record \
                 how a weapon is being wielded",
                weapon.key, weapon.damage_die
            ),
        });

        // Threat range: doubled width when Improved Critical names the weapon.
        let base_width = weapon.critical_threat_range_width;
        let improved = improved_critical_targets.iter().any(|name| names_this_weapon(name));
        let effective_width = if improved { base_width * 2 } else { base_width };
        let threat_low = 21_u8.saturating_sub(effective_width);
        let improved_detail = if improved {
            format!(
                "Improved Critical names this weapon, doubling the threat range width from \
                 {base_width} to {effective_width} (a doubled range widens the low end; the high \
                 end is always 20)."
            )
        } else {
            "Improved Critical does not name this weapon, so its threat range is the weapon's \
             printed one."
                .to_owned()
        };
        explanations.push(ComputationExplanation {
            id: format!("combat.weapon_threat_range_low.{slug}"),
            value: i16::from(threat_low),
            detail: format!(
                "Critical threat range with the equipped {}: {threat_low}-20/x{}. {improved_detail} \
                 The value grounded here is the LOW end of the range; the corpus stores CRITRANGE \
                 as a width, so the range is 21-width..=20",
                weapon.key, weapon.critical_multiplier
            ),
        });
    }
}

/// Return the list of unmet conditions for the exact deterministic combat
/// posture. An empty list means the posture is fully supported.
pub(super) fn unmet_combat_posture_conditions(input: &CharacterInput) -> Vec<String> {
    let chosen = &input.chosen;
    let mut unmet = Vec::new();

    // SD-21 E6b.1: widened from a Fighter-only gate to the same dispatch-supported
    // class set `compute_class_chassis` / `compute_total_saves` already recognize
    // (`has_supported_class_chassis`) -- the combat baseline math itself (BAB +
    // STR + Weapon Focus, with Fighter-only Weapon/Armor Training folded in via
    // the `supported_fighter_level(input).unwrap_or(1)` fallback below, which is
    // 0 for any non-Fighter class) is not Fighter-specific, only the class-level
    // recognition gate was.
    if !has_supported_class_chassis(input) {
        unmet.push(format!(
            "missing supported {FIGHTER_CLASS_ID} levels 1-{MAX_SUPPORTED_FIGHTER_LEVEL} or \
             {WIZARD_CLASS_ID} levels 1-{MAX_SUPPORTED_WIZARD_LEVEL} chassis"
        ));
    }

    require_active_state(
        input,
        LONGSWORD_ITEM_ID,
        ActiveState::EquippedActive,
        &mut unmet,
    );
    require_active_state(
        input,
        CHAIN_SHIRT_ITEM_ID,
        ActiveState::EquippedActive,
        &mut unmet,
    );
    require_active_state(input, SHIELD_ITEM_ID, ActiveState::Absent, &mut unmet);
    require_active_state(
        input,
        POWER_ATTACK_ITEM_ID,
        ActiveState::SelectedInactive,
        &mut unmet,
    );

    // v0.6 alpha swarm (creation-seed honesty fix): **Dodge is deliberately
    // NOT required here.** It used to be, and that requirement is what
    // forced `compose_character_input` to seed `feat:dodge` onto every
    // freshly created character regardless of race or class -- a feat no
    // choice slot had granted, shown to the player verbatim on the sheet's
    // Feats tab. Dodge is now a conditional contribution in
    // `compute_combat_baseline` instead (`dodge_armor_class_bonus`): the
    // baseline computes either way, one point of armor class lower without
    // it. Nothing is weakened -- no number that used to be claimed is now
    // claimed on thinner evidence; a claim that used to be *fabricated at
    // creation time* simply is not made any more.
    //
    // Weapon Focus IS still required, and that is not an oversight: this
    // baseline's whole attack formula is Longsword-specific (Fighter
    // Weapon Training is hardcoded to the Longsword's "Heavy Blades"
    // group), and for the one class whose slot actually grants it
    // (`choice:fighter_bonus_feat`, a real Fighter class feature) the
    // claim is backed. Seeding it for non-Fighters is the same class of
    // unbacked claim Dodge was, and closing that one needs the weapon
    // loadout widened first -- a separate increment, not this fix.
    //
    // Matched through the shared identity fold, not by string equality: the
    // feat picker sends the catalog key ("Weapon Focus"), character creation
    // seeds the engine token ("feat:weapon_focus"), and both name the same
    // feat. Comparing verbatim here meant a player who picked it from the
    // catalog left this posture unmet, so the whole combat baseline came back
    // unsupported -- no armor class, no melee attack bonus.
    if !feat_identity::holds(&chosen.selected_feats, WEAPON_FOCUS_FEAT_ID) {
        unmet.push(format!("missing selected feat {WEAPON_FOCUS_FEAT_ID}"));
    }

    // The Fighter bonus-feat choice mechanism (`choice:fighter_bonus_feat`) is a
    // Fighter-only class feature (how a Fighter's own 1st-level bonus feat was
    // granted); it is only required when Fighter is actually the dispatch-supported
    // class here. A Wizard has no such class feature at all, so it must not be
    // asked to satisfy a choice mechanism it can never have -- Weapon Focus itself
    // is still required for every class via the unconditional `selected_feats`
    // check immediately above.
    //
    // v0.6 alpha swarm (systematic sweep after finding the same brittleness twice
    // in `validate_fighter_feat_choice_legality`): this used `supported_fighter_level`
    // (single-class-only), so it was also blind to a Fighter+X multiclass mix with
    // a wrong bonus-feat choice -- confirmed empirically (this function's own
    // `unmet` list came back empty for that exact scenario) before fixing. Not
    // currently exploitable at the system level (`validate_fighter_feat_choice_legality`,
    // already fixed for multiclass, independently claim-blocks the same
    // `FIGHTER_BONUS_FEAT_CHOICE_ID` slot), but fixing anyway for correctness and
    // so the two checks don't silently diverge if either is ever touched again.
    if fighter_level_in_mix(input).is_some() {
        let fighter_bonus_selection = choice_selection(input, FIGHTER_BONUS_FEAT_CHOICE_ID);
        if fighter_bonus_selection != Some(WEAPON_FOCUS_LONGSWORD_SELECTION) {
            unmet.push(format!(
                "{FIGHTER_BONUS_FEAT_CHOICE_ID} selection must be {WEAPON_FOCUS_LONGSWORD_SELECTION}, got {fighter_bonus_selection:?}"
            ));
        }
    }

    unmet
}

/// Record an unmet condition unless the named item has exactly `expected` state.
pub(crate) fn require_active_state(
    input: &CharacterInput,
    item_id: &str,
    expected: ActiveState,
    unmet: &mut Vec<String>,
) {
    let actual = input
        .chosen
        .equipment_selections
        .iter()
        .find(|e| e.item_id == item_id)
        .map(|e| e.active_state);
    if actual != Some(expected) {
        unmet.push(format!(
            "{item_id} must be {expected:?} for the deterministic baseline, got {actual:?}"
        ));
    }
}

/// v0.6 alpha swarm, risks item 8 (Slayer full-build closure, seventh
/// ACG/APG class-specific closure): tests the four flat class-feature
/// formulas directly, mirroring the established dispatch-widening test
/// module shape.
#[cfg(test)]
mod slayer_dispatch_widening_safety_tests {
    use super::{
        build_pilot_headless_receipt, CharacterClassLevel, CharacterInput, HeadlessReceiptStatus,
        FIGHTER_CLASS_ID, SLAYER_CLASS_ID,
    };
    use crate::rules_core::character_input::load_character_input_fixture;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );

    fn human_slayer_input(level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: SLAYER_CLASS_ID.to_owned(), level }];
        input
    }

    /// A single-class Human Slayer stays `Blocked` on the new, narrower
    /// `other_features_deferred` diagnostic alone (never the retired
    /// generic one), with all four flat sub-feature formulas grounded
    /// unconditionally -- Slayer has no choice or activation gate for
    /// any of them.
    ///
    /// Level 1: Sneak Attack 1/3=0d6, Trap Sense max(1,1/3)=1,
    /// Trapfinding 1/2=0, Track max(1/2,1)=1.
    ///
    /// **Task #91 flips the status assertion** for the same reason as its
    /// sibling in `acg_class_chassis_dispatch_tests`: the seven features
    /// its blocking claim named are now grounded. The four flat formulas
    /// this test exists to pin are unaffected and still checked below.
    #[test]
    fn single_class_slayer_computes_with_all_four_flat_formulas_grounded() {
        let input = human_slayer_input(1);
        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Slayer grounds every named corpus feature and must now compute: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.slayer.unsupported"),
            "the retired generic diagnostic must never appear for Slayer: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            receipt
                .computation
                .diagnostics
                .iter()
                .any(|d| d.id == "class_feature.acg.slayer.other_features_deferred.unsupported"
                    && !d.claim_blocking),
            "the remainder record must survive as a NON-blocking diagnostic: {:?}",
            receipt.computation.diagnostics
        );

        let sneak_attack = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.slayer.sneak_attack_dice")
            .expect("Sneak Attack dice must be grounded");
        assert_eq!(sneak_attack.value, 0, "Slayer level 1 Sneak Attack dice: 1/3=0: {:?}", sneak_attack);

        let trap_sense = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.slayer.trap_sense_bonus")
            .expect("Trap Sense bonus must be grounded");
        assert_eq!(trap_sense.value, 1, "Slayer level 1 Trap Sense: max(1,1/3)=1: {:?}", trap_sense);

        let trapfinding = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.slayer.trapfinding_bonus")
            .expect("Trapfinding bonus must be grounded");
        assert_eq!(trapfinding.value, 0, "Slayer level 1 Trapfinding: 1/2=0: {:?}", trapfinding);

        let track = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "class_feature.acg.slayer.track_bonus")
            .expect("Track bonus must be grounded");
        assert_eq!(track.value, 1, "Slayer level 1 Track: max(1/2,1)=1: {:?}", track);
    }

    /// Sneak Attack dice progression at higher levels, verified against
    /// the raw corpus `BONUS:VAR` formula directly.
    #[test]
    fn slayer_sneak_attack_dice_progression_matches_the_corpus_formula_at_higher_levels() {
        for (level, expected_dice) in [(3, 1), (6, 2), (9, 3), (12, 4)] {
            let input = human_slayer_input(level);
            let receipt = build_pilot_headless_receipt(&input);

            let sneak_attack = receipt
                .computation
                .explanations
                .iter()
                .find(|e| e.id == "class_feature.acg.slayer.sneak_attack_dice")
                .expect("Sneak Attack dice must be grounded");
            assert_eq!(
                sneak_attack.value, expected_dice,
                "level {level} Sneak Attack dice: {:?}",
                sneak_attack
            );
        }
    }

    /// A non-Slayer character must never ground any Slayer explanation.
    /// Also proves Fighter's own golden path is unaffected.
    #[test]
    fn non_slayer_characters_never_ground_slayer_explanations() {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty());
        let input = result.character_input.expect("valid fixture");
        assert_eq!(input.chosen.class_levels[0].class_id, FIGHTER_CLASS_ID);

        let receipt = build_pilot_headless_receipt(&input);

        assert_eq!(
            receipt.status,
            HeadlessReceiptStatus::Computed,
            "Fighter's own golden path must be unaffected: {:?}",
            receipt.computation.diagnostics
        );
        assert!(
            !receipt
                .computation
                .explanations
                .iter()
                .any(|e| e.id.starts_with("class_feature.acg.slayer.")),
            "a non-Slayer character must never ground any Slayer explanation: {:?}",
            receipt.computation.explanations
        );
    }
}


/// SD-36 Epic F step 2 (`epic-f-class-completion.md` §3.4): a class with no static
/// `CLASS_WEAPON_PROFICIENCIES` row falls back to the converted record
/// (`class_proficiency_sheet_rules::class_weapon_proficiency_view`). The 42 static rows keep
/// first precedence; the reader's `Unknown` keeps the claim-blocking diagnostic and now names
/// its reason.
#[cfg(test)]
mod converted_record_proficiency_fallback_tests {
    use super::{
        build_pilot_headless_receipt, character_is_proficient_with, character_weapon_proficiency,
        CharacterClassLevel, CharacterInput, WeaponProficiencyVerdict,
    };
    use crate::rules_core::character_input::load_character_input_fixture;
    use crate::rules_core::rules_tables::crb::weapon_tables::{
        self, WeaponProficiency, WeaponTableEntry,
    };
    use crate::rules_core::sheet_rule::{Effect, Fact, ProfRef};
    use crate::rules_core::sheet_rule_package;

    const FIGHTER_LEVEL_1_FIXTURE: &str = include_str!(
        "../../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
    );
    const PROFICIENCY_UNKNOWN: &str = "combat.baseline_weapon_proficiency_unknown";

    fn single_class(class_id: &str, level: u8) -> CharacterInput {
        let result = load_character_input_fixture(FIGHTER_LEVEL_1_FIXTURE);
        assert!(result.diagnostics.is_empty(), "fixture should load cleanly");
        let mut input = result.character_input.expect("valid fixture");
        input.chosen.class_levels =
            vec![CharacterClassLevel { class_id: class_id.to_owned(), level }];
        input
    }

    fn crb_weapon(key: &str) -> &'static WeaponTableEntry {
        weapon_tables::WEAPON_TABLE.iter().find(|w| w.key == key).expect("CRB weapon table row")
    }

    /// The Katana is an Ultimate Combat weapon the CRB-only `WEAPON_TABLE` has no row for. Its
    /// stat facts here are read from the CONVERTED equipment record's own tags
    /// (`ultimate_combat:equipment:katana`), never typed from memory.
    fn converted_weapon(slug: &str) -> WeaponTableEntry {
        let package = sheet_rule_package::package().as_ref().expect("converted package loads");
        let id = package.find("equipment", slug).expect("converted equipment record");
        let rule = package.rule(id).expect("rule");
        let has = |tag: &str| rule.tags.iter().any(|t| t == tag);
        let proficiency = [
            ("Simple", WeaponProficiency::Simple),
            ("Martial", WeaponProficiency::Martial),
            ("Exotic", WeaponProficiency::Exotic),
        ]
        .into_iter()
        .find(|(tag, _)| has(tag))
        .map(|(_, tier)| tier);
        let label: &'static str = Box::leak(rule.label.clone().into_boxed_str());
        let group: Option<&'static str> = rule
            .tags
            .iter()
            .filter_map(|t| t.strip_prefix("Weapon Group "))
            .find(|g| !g.starts_with("Melee") && !g.starts_with("Ranged"))
            .map(|g| &*Box::leak(g.to_owned().into_boxed_str()));
        WeaponTableEntry {
            key: label,
            damage_die: "1d8",
            critical_threat_range_width: 1,
            critical_multiplier: 2,
            proficiency_name: Some(label),
            proficiency,
            weapon_group: group,
            is_melee: has("Melee"),
            is_ranged: has("Ranged"),
        }
    }

    fn blocking_ids(input: &CharacterInput) -> Vec<(String, String)> {
        build_pilot_headless_receipt(input)
            .computation
            .diagnostics
            .into_iter()
            .filter(|d| d.claim_blocking)
            .map(|d| (d.id, d.message))
            .collect()
    }

    /// Samurai has no static row. Its converted `samurai_proficiencies` record grants the
    /// `Samurai` weapon set (Katana, Naginata, Wakizashi) expanded at ingest; the Katana is
    /// Exotic, so only set membership can make a Samurai proficient with it.
    #[test]
    fn samurai_at_level_5_is_proficient_with_the_katana_via_its_weapon_set() {
        assert!(weapon_tables::class_weapon_proficiency("class:samurai").is_none(), "no static row");
        let input = single_class("class:samurai", 5);
        let katana = converted_weapon("katana");
        assert_eq!(katana.proficiency, Some(WeaponProficiency::Exotic), "the converted Katana is Exotic");
        assert_eq!(character_is_proficient_with(&input, &katana), Some(true));
        // The Longsword (the combat baseline's one question) is Martial: a Samurai tier grant.
        assert_eq!(character_is_proficient_with(&input, crb_weapon("Longsword")), Some(true));
        // An Exotic CRB weapon outside the set stays non-proficient -- a known, not an unknown.
        assert_eq!(crb_weapon("Dire Flail").proficiency, Some(WeaponProficiency::Exotic));
        assert_eq!(character_is_proficient_with(&input, crb_weapon("Dire Flail")), Some(false));
        let blocking = blocking_ids(&input);
        assert!(!blocking.iter().any(|(id, _)| id == PROFICIENCY_UNKNOWN), "{blocking:?}");
    }

    /// Commoner (SD-36 F1c-3, D6): its converted proficiency record
    /// (`weapon_and_armor_proficiency_commoner`) picks one Single Simple Weapon Proficiency, the
    /// one member of pool `simple_weapon_proficiency_choice`, linked at ingest (`pool_link.rs`)
    /// to the Simple-tier weapon list that member offers. The reader carries it as a weapon pick;
    /// the character's recorded choice decides it, and the verdict prints the choice.
    ///
    /// - No choice recorded: a Simple weapon (Club) is Unknown -- the pick could cover it --
    ///   while a weapon outside the options (Longsword, Martial) is Known(false): no pick of a
    ///   Simple weapon can cover it.
    /// - The canonical seed (`class_seeds::COMMONER_CANONICAL_WEAPON`, Club): Club is Known(true),
    ///   Dagger Known(false), and the printed words carry the chosen weapon.
    #[test]
    fn commoner_weapon_pick_is_decided_by_the_recorded_choice() {
        use crate::rules_core::class_seeds::{COMMONER_CANONICAL_WEAPON, COMMONER_WEAPON_CHOICE_ID};
        use crate::rules_core::pilot_compute::class_proficiency_sheet_rules::{
            class_weapon_proficiency_view, ProficiencyAnswer,
        };
        let ProficiencyAnswer::Known(view) = class_weapon_proficiency_view("commoner", 1) else {
            panic!("commoner reads Known (with its weapon pick) from the converted record");
        };
        assert!(view.unresolved_picks.is_empty(), "the pick is linked: {view:?}");
        assert_eq!(view.weapon_picks.len(), 1, "{view:?}");
        let pick = &view.weapon_picks[0];
        assert_eq!(pick.choice, COMMONER_WEAPON_CHOICE_ID);
        for simple in ["Club", "Dagger", "Quarterstaff", "Crossbow (Light)"] {
            assert!(pick.options.iter().any(|o| o == simple), "{simple} must be an option: {:?}", pick.options);
        }
        assert!(!pick.options.iter().any(|o| o == "Longsword"), "a Martial weapon is no option: {:?}", pick.options);

        let unseeded = single_class("class:commoner", 1);
        assert!(!unseeded.chosen.selected_choices.iter().any(|c| c.choice_set_id == COMMONER_WEAPON_CHOICE_ID));
        match character_weapon_proficiency(&unseeded, crb_weapon("Club")) {
            WeaponProficiencyVerdict::Unknown { reason } => {
                assert!(reason.contains("class:commoner level 1") && reason.contains("records no choice"), "{reason}")
            }
            known => panic!("an unrecorded pick that could cover the Club must leave it Unknown, got {known:?}"),
        }
        assert_eq!(character_is_proficient_with(&unseeded, crb_weapon("Longsword")), Some(false));
        let blocking = blocking_ids(&unseeded);
        assert!(!blocking.iter().any(|(id, _)| id == PROFICIENCY_UNKNOWN), "Longsword is decided: {blocking:?}");

        let mut seeded = unseeded.clone();
        seeded.chosen.selected_choices.push(crate::rules_core::character_input::SelectedChoice {
            choice_set_id: COMMONER_WEAPON_CHOICE_ID.to_owned(),
            selection_id: COMMONER_CANONICAL_WEAPON.to_owned(),
        });
        match character_weapon_proficiency(&seeded, crb_weapon("Club")) {
            WeaponProficiencyVerdict::Known { proficient, printed } => {
                assert!(proficient, "the chosen Club is proficient");
                assert!(printed.iter().any(|p| p.contains("proficient with the chosen weapon (Club)")), "{printed:?}");
            }
            unknown => panic!("a recorded pick decides the Club, got {unknown:?}"),
        }
        assert_eq!(character_is_proficient_with(&seeded, crb_weapon("Dagger")), Some(false));
        assert_eq!(character_is_proficient_with(&seeded, crb_weapon("Longsword")), Some(false));
        // The per-weapon total for an equipped Club carries no -4.
        let mut club = seeded.clone();
        for selection in &mut club.chosen.equipment_selections {
            if selection.item_id == "item:longsword" {
                selection.item_id = "item:club".to_owned();
            }
        }
        let receipt = build_pilot_headless_receipt(&club);
        let club_total = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "combat.weapon_attack_bonus.club")
            .expect("club attack total");
        assert!(club_total.detail.contains("nonproficiency penalty (0)"), "{}", club_total.detail);
    }

    /// Reader batch blocker 1: a converted `WeaponSet` lists its members by PROFICIENCY name
    /// (`Sword (Short)`, `Pick (Light)`), so set membership must join on `proficiency_name`, not
    /// on the weapon's record key (`Short Sword`). Marksman holds `Light.Martial`.
    #[test]
    fn marksman_light_martial_set_covers_weapons_whose_key_differs_from_their_proficiency_name() {
        assert!(weapon_tables::class_weapon_proficiency("class:marksman").is_none(), "no static row");
        let input = single_class("class:marksman", 1);
        for weapon in ["Short Sword", "Light Pick", "Kukri"] {
            assert_eq!(character_is_proficient_with(&input, crb_weapon(weapon)), Some(true), "{weapon}");
        }
    }

    /// A class with no static row whose converted answer is Unknown keeps the claim-blocking
    /// diagnostic, and the answer now carries the reader's reason. Synthetic: a class id with
    /// no converted record at all (it has no chassis either, so the combat baseline never runs
    /// for it -- the verdict itself is asserted). Magus was the real case until SD-36 Epic F1c-1
    /// converted its `TYPE=WeaponProfMartial` grant-by-type, and Commoner until F1c-3 linked its
    /// one-simple-weapon pick (`commoner_weapon_pick_is_decided_by_the_recorded_choice`); Magus is
    /// asserted as the flip: it reads Longsword from the converted record and carries no
    /// proficiency diagnostic.
    #[test]
    fn a_class_with_no_row_and_incomplete_closure_keeps_the_diagnostic() {
        let synthetic = single_class("class:fixture_class_with_no_record", 1);
        match character_weapon_proficiency(&synthetic, crb_weapon("Longsword")) {
            WeaponProficiencyVerdict::Unknown { reason } => assert!(
                reason.contains("no converted class record for `fixture_class_with_no_record`"),
                "{reason}"
            ),
            known => panic!("a class with no row and no record must be Unknown, got {known:?}"),
        }

        assert!(weapon_tables::class_weapon_proficiency("class:magus").is_none());
        let magus = single_class("class:magus", 1);
        assert_eq!(character_is_proficient_with(&magus, crb_weapon("Longsword")), Some(true));
        let blocking = blocking_ids(&magus);
        assert!(
            !blocking.iter().any(|(id, _)| id == PROFICIENCY_UNKNOWN),
            "magus reads its tiers from the converted record since F1c-1: {blocking:?}"
        );
    }

    /// Marksman's racial-gated Sling Staff is printed with its condition, never counted
    /// (paper-sheet doctrine): the condition reaches the baseline attack explanation.
    #[test]
    fn a_printed_condition_reaches_the_receipt_as_explanation_text() {
        let receipt = build_pilot_headless_receipt(&single_class("class:marksman", 1));
        let baseline = receipt
            .computation
            .explanations
            .iter()
            .find(|e| e.id == "combat.baseline_melee_attack_bonus")
            .expect("baseline melee explanation");
        assert!(
            baseline.detail.contains("Sling Staff") && baseline.detail.contains("only when"),
            "{}",
            baseline.detail
        );
    }

    /// Review finding 15 / the step-2 WeaponAllOf contract: every conjunctive selector the
    /// converter wrote is answerable by the weapon table (a tier, `Melee`, `Ranged`); a
    /// conjunct the table cannot answer was expanded to a `WeaponSet` at ingest instead.
    #[test]
    fn every_converted_weapon_all_of_conjunct_is_answerable_by_the_weapon_table() {
        let package = sheet_rule_package::package().as_ref().expect("converted package loads");
        let answerable = ["Simple", "Martial", "Exotic", "Melee", "Ranged"];
        let mut seen = 0usize;
        for rule in package.rules.values() {
            for effect in &rule.grants {
                let fact = match effect {
                    Effect::FactGrant(fact) | Effect::GatedFactGrant { fact, .. } => fact,
                    _ => continue,
                };
                if let Fact::Proficiency(ProfRef::WeaponAllOf(tags)) = fact {
                    seen += 1;
                    assert!(!tags.is_empty(), "{}: empty conjunction", rule.id);
                    for tag in tags {
                        assert!(answerable.contains(&tag.as_str()), "{}: conjunct `{tag}` in {tags:?}", rule.id);
                    }
                }
            }
        }
        assert!(seen > 0, "the package carries WeaponAllOf selectors (Marksman's Martial + Ranged)");
    }
}
