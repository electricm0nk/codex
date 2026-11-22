#!/usr/bin/env python3
r"""Demote the LAST unframed PCGen citations out of rendered prose, by hand --
SD-35 Epic 6, AT-35-E6-003-SWEEP cycle 10.

Why this is a hand table and not another rule
---------------------------------------------
Cycles 4, 5 and 6 built three automatic citation frames for this mechanism and
**removed two of them** after measuring the prose they produced against the real
file: a bare connective run and a backtick-quoted token both compiled, both
passed every safety gate, and both emitted ungrammatical sheet lines
(*"the record carries no -- and cross-checked against"*). Cycle 6's receipt
named the surviving residue and said so plainly: the citations that remain wear
no shared frame, so no fourth rule reaches them. Cycle 9's *Next-cycle scope*
row costed them as **47 hits of hand work, record by record**.

This file is that hand work, written as an explicit table so it is
re-derivable, reversible and checkable rather than a diff to be read. Each row
is one rendered string -- a `ComputationExplanation.detail`, a
`ComputationDiagnostic.message`, a `SupportStateRow.next_required_uplift` or an
assertion-failure message -- in which a PCGen ingest token was being printed
mid-sentence. The token leaves the string; where it carried provenance a reader
would want, it is re-emitted verbatim as a `//` comment beside the value, which
is where `AGENTS.md` rule 9 wants it and where operator ruling B14
(`decisions.md` §17) says the residue gate does not count it.

**No information is destroyed.** Every row either (a) keeps the same fact in the
rule's own words -- `BONUS:VAR|X|` becomes *"a bonus-variable magnitude named
X"*, `DESC: argument` becomes *"description argument"* -- or (b) moves the token
verbatim into the adjacent provenance comment. A diagnostic that told a
developer *which* variable was missing still tells them which variable is
missing.

What is deliberately NOT in this table
--------------------------------------
`PU_RESOLVABLE_DESCRIPTIONS` and the `PU_*_DESC_TOKEN` constants
(`pilot_compute/mod.rs`) are **verbatim corpus transcriptions**, pinned
byte-for-byte against the `.lst` files on disk by
`tests/sd27_pu_class_feature_descriptions_carry_the_characters_numbers.rs`.
They are not prose this codebase wrote; they are the corpus, held so
`render_pcgen_desc_tokens` can substitute `%N` into them. Editing them would
break that corpus pin and would be a lie about what the book says. They leave
the live side when `pcgen_desc.rs` is deleted -- which is `AT-35-E6-003`'s own
scope, blocked on `class_feature_pool_catalog.rs` reading the sheet-rule
package. They are counted in this cycle's refused remainder under that
mechanism, not exempted.

Usage
-----
    python3 <this> --apply        rewrite in place
    python3 <this> --check        every row already applied? (exit 0/1)

`--check` is the re-derive command for the receipt's row count. It prints
`rows=<n> rows_changed=<n>`; `rows_changed=0` means every row is landed.
"""

import argparse
import os
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", ".."))

PROV = "Provenance (ingest tokens, demoted out of the rendered sheet line -- SD-35 AT-35-E6-003-SWEEP):"

# (relative path, old exact text, new exact text)
ROWS = [
    # ---- src/rules_core/pilot_compute/mod.rs -------------------------------
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''        detail: format!(
            "Alternate racial traits chosen for {}: {}. Firing replace-flag(s) {}, each of which \
             suppresses the standard racial trait whose \
             !PREFACT:1,ABILITIES,<flag>=True gate names it (decisions.md §26 — the swap is a \
''',
        r'''        detail: format!(
            // ''' + PROV + r'''
            //   !PREFACT:1,ABILITIES,<flag>=True
            "Alternate racial traits chosen for {}: {}. Firing replace-flag(s) {}, each of which \
             suppresses the standard racial trait its own exclusion gate names \
             (decisions.md §26 — the swap is a \
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''                 shape as Bard's Bardic Knowledge. DESC-sourced -- the record carries no \
                 `BONUS:SKILL` token -- and cross-checked against the published rule text, which \
                 agrees"
''',
        r'''                 shape as Bard's Bardic Knowledge. Taken from the record's own \
                 description rather than from a skill-bonus magnitude, which it does not carry, \
                 and cross-checked against the published rule text, which agrees"
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''             (`advanced_players_guide/class_feature/inquisitor/inquisitor_domains.json` carries \
             `DEFINE:InquisitorDomain{define_token_domain}|0`). So a {domain}-domain \
''',
        r'''             (`advanced_players_guide/class_feature/inquisitor/inquisitor_domains.json` \
             declares an `InquisitorDomain{define_token_domain}` variable of its own). So a \
             {domain}-domain \
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''            detail: format!(
                "Oracle level {oracle_level} Life Mystery Channel save DC: 10 + level/2 + \
                 Charisma modifier ({charisma:+}) + Improved Channel feat ({:+}) = {dc}. That \
                 feat's own Core Rulebook record names only Cleric/Paladin channel DC variables \
                 this engine computes no total for; its reach here comes from apg_feats.lst's \
                 own `CATEGORY=FEAT|Improved Channel.MOD`, which adds BONUS:VAR|OracleChannelDC|2",
''',
        r'''            detail: format!(
                // ''' + PROV + r'''
                //   apg_feats.lst's own `CATEGORY=FEAT|Improved Channel.MOD`, which adds
                //   `BONUS:VAR|OracleChannelDC|2`
                "Oracle level {oracle_level} Life Mystery Channel save DC: 10 + level/2 + \
                 Charisma modifier ({charisma:+}) + Improved Channel feat ({:+}) = {dc}. That \
                 feat's own Core Rulebook record names only Cleric/Paladin channel DC variables \
                 this engine computes no total for; its reach here comes from the Advanced \
                 Player's Guide's own modification of that feat, which adds +2 to the Oracle's \
                 channel DC",
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''                 tokens do. The revelation's parallel `BONUS:VAR|CMD` half is not integrated -- \
                 this engine computes no Combat Maneuver Defense total"
''',
        r'''                 tokens do. The revelation's parallel Combat Maneuver Defense half is not \
                 integrated -- this engine computes no Combat Maneuver Defense total"
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''                 so it grounds standalone. The corpus record carries `!PREABILITY:1,CATEGORY=Special \
                 Ability,Oracle ~ Lame`, making this revelation mutually exclusive with the Lame \
                 Curse; the bonus feats it also grants at Oracle 5 and 10 (Nimble Moves, Acrobatic \
''',
        r'''                 so it grounds standalone. The corpus record excludes the Oracle's Lame curse \
                 outright, making this revelation mutually exclusive with the Lame \
                 Curse; the bonus feats it also grants at Oracle 5 and 10 (Nimble Moves, Acrobatic \
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''        "level {level}: {instances_taken} instances of Improved Natural Armor exceeds the \
         corpus PREVARLTEQ:EvoImpNatArmCount,MasterLevel/5 cap"
''',
        r'''        // ''' + PROV + r'''
        //   PREVARLTEQ:EvoImpNatArmCount,MasterLevel/5
        "level {level}: {instances_taken} instances of Improved Natural Armor exceeds the \
         corpus cap of one per five master levels"
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''                 Combat is unaffected either way. Gated at investigator level \
                 {INVESTIGATOR_STUDIED_DEFENSE_LEVEL} by the talent's own \
                 `PREVARGTEQ:InvestigatorTalentLVL,9`"
''',
        r'''                 Combat is unaffected either way. Gated at investigator level \
                 {INVESTIGATOR_STUDIED_DEFENSE_LEVEL} by the talent's own prerequisite, which \
                 requires nine investigator talent levels"
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''                 later-book Spirits this codebase does not recognize -- Mammoth \
                 (`adventurers_guide/ag_abilities_class.lst`, Powerful Smash) and Wood \
                 (`ultimate_wilderness/uw_abilities_class.lst`, Tree Limb), neither of which \
                 carries any `BONUS:` token at all"
''',
        r'''                 later-book Spirits this codebase does not recognize -- Mammoth \
                 (`adventurers_guide/ag_abilities_class.lst`, Powerful Smash) and Wood \
                 (`ultimate_wilderness/uw_abilities_class.lst`, Tree Limb), neither of which \
                 carries any magnitude at all"
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''                 checks (corpus BONUS:VAR|{bonus_var}|RagePowersLVL, no arithmetic; RagePowersLVL = \
                 {class_label}LVL directly). Gated on the same active-Raging state as the \
                 rage-execution engine above (corpus BONUS:SKILL|{skill}|{bonus_var}|), integrated \
''',
        r'''                 checks (the corpus states the magnitude as the rage-power level {bonus_var} \
                 with no arithmetic, and that level tracks {class_label} level directly). Gated on \
                 the same active-Raging state as the \
                 rage-execution engine above, and integrated \
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''                "{class_label} level {level} {power_name}: not currently raging, so no enhancement \
                 bonus to {skill} is claimed (corpus BONUS:SKILL|{skill}|{bonus_var}|). Mirrors the \
''',
        r'''                "{class_label} level {level} {power_name}: not currently raging, so no enhancement \
                 bonus to {skill} is claimed (the corpus grants it only while raging). Mirrors the \
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''             clause is a resolution, not a magnitude. The corpus's own BONUS:VAR token for this \
             record reads 3+CHA+(2*SkaldLVL), two higher than its own rule text at every level; that \
''',
        r'''             clause is a resolution, not a magnitude. The corpus's own magnitude for this \
             record reads 3+CHA+(2*SkaldLVL), two higher than its own rule text at every level; that \
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''             magnitude precedent for another. Like Monk's, the corpus BONUS:MOVEADD is \
             armor/encumbrance-conditional: this engine models no encumbrance state, so the \
''',
        r'''             magnitude precedent for another. Like Monk's, the corpus speed increase is \
             armor/encumbrance-conditional: this engine models no encumbrance state, so the \
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''                 class block carries no `CAST:`/`KNOWN:` row at all below level \
                 {BLOODRAGER_FIRST_CASTING_LEVEL}, and its caster-level token is itself gated \
                 `PRECLASS:1,Bloodrager={BLOODRAGER_FIRST_CASTING_LEVEL}`. Correctly absent by \
''',
        r'''                 class block carries no spells-per-day or spells-known row at all below level \
                 {BLOODRAGER_FIRST_CASTING_LEVEL}, and its caster level is itself gated on \
                 reaching bloodrager level {BLOODRAGER_FIRST_CASTING_LEVEL}. Correctly absent by \
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''                         consumed today), granting evasion{}. A boolean posture fact (real corpus) \
                         -- no `BONUS:VAR` magnitude exists for this option at all, and this \
                         codebase has no integrated evasion/improved-evasion total to apply it to",
''',
        r'''                         consumed today), granting evasion{}. A boolean posture fact (real corpus) \
                         -- no magnitude exists for this option at all, and this \
                         codebase has no integrated evasion/improved-evasion total to apply it to",
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''                         minutes; {minutes_consumed_today} consumed today): a genuinely valid, \
                         text-only posture carrying no `BONUS:VAR` at all -- no focus bonus is \
                         claimed"
''',
        r'''                         minutes; {minutes_consumed_today} consumed today): a genuinely valid, \
                         text-only posture carrying no magnitude at all -- no focus bonus is \
                         claimed"
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''        detail: format!(
            "Hunter Bonus Tricks at hunter level {level}: {} bonus trick(s) known by her animal \
             companion, from the corpus's own `BONUS:VAR|HunterBonusTricks|\
             floor((HunterLVL-1)/6)`. The formula and the DESC agree exactly -- first trick at \
''',
        r'''        detail: format!(
            // ''' + PROV + r'''
            //   `BONUS:VAR|HunterBonusTricks|floor((HunterLVL-1)/6)`
            "Hunter Bonus Tricks at hunter level {level}: {} bonus trick(s) known by her animal \
             companion, from the corpus's own count of one per six hunter levels after the \
             first. The formula and the printed description agree exactly -- first trick at \
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''            detail: format!(
                "Monk level {level} level-1 bonus feat is Combat Reflexes: the corpus formula \
                 token BONUS:VAR|CombatReflexesAttacks|DEX resolves to \
                 max(Dexterity modifier, 0) = {additional_attacks_of_opportunity} additional \
''',
        r'''            detail: format!(
                // ''' + PROV + r'''
                //   BONUS:VAR|CombatReflexesAttacks|DEX
                "Monk level {level} level-1 bonus feat is Combat Reflexes: the corpus formula \
                 for its extra attacks of opportunity resolves to \
                 max(Dexterity modifier, 0) = {additional_attacks_of_opportunity} additional \
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''                     own `%N`-substituted DESC: argument text (this record carries no), after \
''',
        r'''                     own `%N`-substituted description argument text, after \
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''             Armor Proficiency): \"{base_desc}\" This is a bounded grant-only identity record \
             (value 0, non-fabricated): the record's only proficiency tokens are \
             ABILITY:...AUTOMATIC/AUTO:WEAPONPROF grants, no BONUS: magnitude anywhere. \
''',
        r'''             Armor Proficiency): \"{base_desc}\" This is a bounded grant-only identity record \
             (value 0, non-fabricated): the record grants proficiencies automatically and \
             carries no magnitude anywhere. \
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''        format!(
            "Sorcerer Arcane bloodline bonus feats at sorcerer level {sorcerer_level}: none yet, \
             correctly absent by PF1 Core Rulebook level gate (the first is granted at 7th \
             level). Corpus: BONUS:ABILITYPOOL|Sorcerer Bloodline Feat|BloodlineFeatCount with \
             BONUS:VAR|BloodlineFeatCount|(BloodlineFeatProgression-1)/6, which is 0 below 7th"
        )
''',
        r'''        format!(
            // ''' + PROV + r'''
            //   BONUS:ABILITYPOOL|Sorcerer Bloodline Feat|BloodlineFeatCount
            //   BONUS:VAR|BloodlineFeatCount|(BloodlineFeatProgression-1)/6
            "Sorcerer Arcane bloodline bonus feats at sorcerer level {sorcerer_level}: none yet, \
             correctly absent by PF1 Core Rulebook level gate (the first is granted at 7th \
             level). The corpus sizes the bloodline feat pool as one per six bloodline levels \
             after the first, which is 0 below 7th"
        )
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''            detail: format!(
                "{} grants a {:+} bonus on {} checks {} \
                 (BONUS:SITUATION|{}=...|{}), transcribed from its corpus token. This is a \
                 SITUATIONAL bonus and is deliberately NOT added to any skill total: it applies \
                 only in the circumstance named here, and folding it into a general modifier \
                 would report a specific, checkable, wrong number on every ordinary {} check. \
                 Same treatment the ARG feat situational records and the Dwarf Stonecunning/\
                 Greed situational records already receive",
                fact.trait_name,
                fact.bonus,
                fact.skill_name,
                fact.circumstance,
                fact.skill_name,
                fact.bonus,
                fact.skill_name
            ),
''',
        r'''            detail: format!(
                // ''' + PROV + r'''
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
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''            detail: format!(
                "{} lets you treat your caster level as {:+} for spells of the {} subschool \
                 (BONUS:CASTERLEVEL|SUBSCHOOL.{}|{}), transcribed from its corpus token. This \
                 engine computes no integrated per-subschool caster level total anywhere, so \
                 this grounds as a standalone flat record rather than folding into a total that \
                 would misstate every other subschool's spells",
                fact.trait_name, fact.bonus, fact.subschool, fact.subschool, fact.bonus
''',
        r'''            detail: format!(
                // ''' + PROV + r'''
                //   BONUS:CASTERLEVEL|SUBSCHOOL.<subschool>|<bonus>
                "{} lets you treat your caster level as {:+} for spells of the {} subschool, \
                 transcribed from its corpus caster-level entry. This \
                 engine computes no integrated per-subschool caster level total anywhere, so \
                 this grounds as a standalone flat record rather than folding into a total that \
                 would misstate every other subschool's spells",
                fact.trait_name, fact.bonus, fact.subschool
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''            detail: format!(
                "{} (ARG) grants a {:+} bonus on {} checks {} \
                 (BONUS:SITUATION|{}=...|{}), transcribed from its corpus token and confirmed \
                 against its own BENEFIT prose. This is a SITUATIONAL bonus and is deliberately \
                 NOT added to any skill total: it applies only in the circumstance named here, \
                 and folding it into a general modifier would report a specific, checkable, \
                 wrong number on every ordinary {} check. Same treatment the Dwarf Stonecunning \
                 and Greed situational records already receive",
                fact.feat_key,
                fact.bonus,
                fact.skill_name,
                fact.circumstance,
                fact.skill_name,
                fact.bonus,
                fact.skill_name
            ),
''',
        r'''            detail: format!(
                // ''' + PROV + r'''
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
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''                 Improvisation's BONUS:VAR|UseUntrainedSkills|1 (use trained-only skills untrained) \
                 is a capability with no magnitude, and Improved Improvisation's \
                 BONUS:VAR|ACCHECK|ArmorCheckPenalty/2 halves a NONPROFICIENCY penalty this engine \
                 models no proficiency state to incur"
''',
        r'''                 Improvisation's own use-trained-only-skills-untrained clause \
                 is a capability with no magnitude, and Improved Improvisation \
                 halves an armor check penalty for nonproficiency that this engine \
                 models no proficiency state to incur"
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''                 as Aquatic Ancestry's swim speed does. The feat's companion \
                 BONUS:VAR|Maneuverability|1 is NOT grounded: it moves a manoeuvrability tier on \
''',
        r'''                 as Aquatic Ancestry's swim speed does. The feat's companion \
                 manoeuvrability step is NOT grounded: it moves a manoeuvrability tier on \
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''                 grounds standalone. The feat's BONUS:VAR|FiendSightTier|1 is a pick counter \
''',
        r'''                 grounds standalone. The feat's own tier counter is a pick counter \
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''                 attack bonus (+{base_attack_bonus}) + Constitution modifier ({constitution:+}), \
                 transcribed from BONUS:VAR|StaminaPool|BAB+CON, plus {extra} Extra Stamina pick(s) \
''',
        r'''                 attack bonus (+{base_attack_bonus}) + Constitution modifier ({constitution:+}), \
                 transcribed from the corpus's own pool formula, plus {extra} Extra Stamina pick(s) \
''',
    ),
    (
        "src/rules_core/pilot_compute/mod.rs",
        r'''            detail: format!(
                "{} (ARG) grants a {:+} bonus on {} checks, transcribed from its corpus \
                 BONUS:SKILL token and confirmed against its own BENEFIT prose. {}",
''',
        r'''            detail: format!(
                // ''' + PROV + r'''
                //   BONUS:SKILL|<skill>|<bonus>
                "{} (ARG) grants a {:+} bonus on {} checks, transcribed from its corpus \
                 skill-bonus entry and confirmed against its own printed benefit. {}",
''',
    ),
    # ---- src/rules_core/pilot_compute/class_slayer.rs ----------------------
    (
        "src/rules_core/pilot_compute/class_slayer.rs",
        r'''                 This is a bounded grant-only identity record (value 0, non-fabricated): the \
                 record's only tokens are ABILITY:...AUTOMATIC proficiency grants, no BONUS: \
                 magnitude anywhere. The weapon half's real mechanical consequence -- avoiding \
''',
        r'''                 This is a bounded grant-only identity record (value 0, non-fabricated): the \
                 record grants proficiencies automatically and carries no \
                 magnitude anywhere. The weapon half's real mechanical consequence -- avoiding \
''',
    ),
    # ---- src/rules_core/pilot_compute/class_ultimate_combat.rs -------------
    (
        "src/rules_core/pilot_compute/class_ultimate_combat.rs",
        r'''                 the base row carries no BONUS: magnitude of its own beyond the shared \
''',
        r'''                 the base row carries no magnitude of its own beyond the shared \
''',
    ),
    # ---- src/rules_core/derived_evaluator_fixture_check.rs -----------------
    (
        "src/rules_core/derived_evaluator_fixture_check.rs",
        r'''                        "corpus row states {} but carries no BONUS:VAR|{}| token at all",
''',
        r'''                        "corpus row states {} but carries no bonus-variable magnitude named {} at all",
''',
    ),
    (
        "src/rules_core/derived_evaluator_fixture_check.rs",
        r'''                            "no record in {book}'s class_feature corpus defines BONUS:VAR|{level_var}|, \
                             so the fixture's expected alias {:?} cannot be confirmed",
''',
        r'''                            "no record in {book}'s class_feature corpus defines a bonus-variable \
                             magnitude named {level_var}, \
                             so the fixture's expected alias {:?} cannot be confirmed",
''',
    ),
    (
        "src/rules_core/derived_evaluator_fixture_check.rs",
        r'''                    "corpus row states {} but the shipped record carries no matching \
                     BONUS:SKILL|{}|… token at all",
''',
        r'''                    "corpus row states {} but the shipped record carries no matching \
                     skill-bonus magnitude for {} at all",
''',
    ),
    (
        "src/rules_core/derived_evaluator_fixture_check.rs",
        r'''                        "corpus row states {} but the shipped record carries no DESC: argument \
                         the evaluator can parse a save-DC shape from (candidates: {candidates:?})",
''',
        r'''                        "corpus row states {} but the shipped record carries no description argument \
                         the evaluator can parse a save-DC shape from (candidates: {candidates:?})",
''',
    ),
    (
        "src/rules_core/derived_evaluator_fixture_check.rs",
        r'''                        "corpus row states {} but the shipped record carries {n} DISTINCT \
                         parseable save-DC shapes across its DESC: arguments — ambiguous",
''',
        r'''                        "corpus row states {} but the shipped record carries {n} DISTINCT \
                         parseable save-DC shapes across its description arguments — ambiguous",
''',
    ),
    (
        "src/rules_core/derived_evaluator_fixture_check.rs",
        r'''                    "corpus row states {} but the shipped record carries no \
                     BONUS:WEAPONPROF={}|DAMAGE| token at all",
''',
        r'''                    "corpus row states {} but the shipped record carries no \
                     weapon-proficiency damage magnitude for {} at all",
''',
    ),
    # ---- src/rules_core/support_state_matrix.rs ----------------------------
    (
        "src/rules_core/support_state_matrix.rs",
        r'''                    widening of this row — Hatred currently lacks a machine-readable BONUS: tag \
                    in the LST corpus (only DESC/ASPECT prose), so it is not eligible \
''',
        r'''                    widening of this row — Hatred currently lacks a machine-readable bonus tag \
                    in the LST corpus (its own entry is description prose only), so it is not eligible \
''',
    ),
]

# The four equipment `next_required_uplift` rows are byte-identical to each
# other, so they are applied by count rather than by position.
REPEATED = [
    (
        "src/rules_core/support_state_matrix.rs",
        r'''                    populating derived_stats from corpus BONUS:/ACCHECK:/MAXDEX: tokens \
                    is a future cycle's or a future SD-N's scope",
''',
        r'''                    populating derived_stats from the corpus's own armor bonus, armor \
                    check penalty and maximum-Dexterity fields \
                    is a future cycle's or a future SD-N's scope",
''',
        4,
    ),
]


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__)
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--apply", action="store_true")
    g.add_argument("--check", action="store_true")
    args = ap.parse_args(argv)

    texts = {}

    def load(rel):
        if rel not in texts:
            with open(os.path.join(REPO, rel), encoding="utf-8") as fh:
                texts[rel] = fh.read()
        return texts[rel]

    rows = 0
    changed = 0
    problems = []

    for rel, old, new in ROWS:
        rows += 1
        text = load(rel)
        n_old = text.count(old)
        n_new = text.count(new)
        if n_old == 1:
            changed += 1
            texts[rel] = text.replace(old, new, 1)
        elif n_old == 0 and n_new >= 1:
            pass  # already applied
        else:
            problems.append(f"{rel}: old x{n_old} new x{n_new} :: {old.strip()[:70]}")

    for rel, old, new, count in REPEATED:
        rows += count
        text = load(rel)
        n_old = text.count(old)
        n_new = text.count(new)
        if n_old == count:
            changed += count
            texts[rel] = text.replace(old, new)
        elif n_old == 0 and n_new == count:
            pass
        else:
            problems.append(f"{rel}: repeated old x{n_old} new x{n_new} (want {count})")

    if problems:
        for p in problems:
            print("AMBIGUOUS " + p, file=sys.stderr)
        print(f"rows={rows} rows_changed={changed} problems={len(problems)}")
        return 2

    if args.apply and changed:
        for rel, text in texts.items():
            with open(os.path.join(REPO, rel), "w", encoding="utf-8") as fh:
                fh.write(text)

    print(f"rows={rows} rows_changed={changed}")
    if args.check:
        return 0 if changed == 0 else 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
