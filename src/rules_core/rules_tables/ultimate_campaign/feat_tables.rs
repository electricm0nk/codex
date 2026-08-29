//! Ultimate Campaign (UCA) Story-trait feat catalog. SD-28 Epic 13
//! (`docs/release/SD-28-ultimate-book-content-ingestion/epic-breakdown.md`
//! §"Epic 13 (SD28-E13) -- Cost calibration") per-book pre-build, mirroring
//! `pathfinder_unchained::feat_tables`'s established shape for a book whose
//! own feat table does not reuse `crb::feats::FeatTableEntry`.
//!
//! **Full corpus coverage, honestly bounded.** `uca_feats.lst` has exactly
//! 23 top-level `CATEGORY:FEAT` records -- every one is a "Story Feat"
//! (`TYPE:Story`), Ultimate Campaign's signature feat family: each ties a
//! narrow mechanical benefit to a long-term roleplaying goal, and every
//! single one carries an upstream `DESC:[Not Implemented] ...` token, with
//! the real mechanical text living on a separate `.MOD BENEFIT:` row
//! instead. Displaying both `description` (the flavor-text `DESC:`) and
//! `benefit` (the `.MOD BENEFIT:` row) is what keeps these records text-
//! complete rather than stubs -- an ingest that surfaced only the
//! `[Not Implemented]` `DESC:` would be shipping a stub by the letter of
//! `docs/governance/no-stub-mvp-doctrine.md`, since the actual rule text a
//! player needs lives on the row this catalog also carries.
//!
//! **Two of the 23 are `deferred-with-reason`, not text-complete --
//! upstream corpus splices, re-derived and confirmed against the live
//! `.lst`, not the brief's own transcription.** `Fearless Zeal` was
//! flagged going into this cycle; `Magnum Opus` was found independently
//! while re-deriving every field (`decisions.md`, dated entries for
//! SD28-E13, record the corrections against the cycle brief, which named
//! only `Fearless Zeal`, and against this module's own first pass, which
//! over-deferred a third record, `Stronghold` -- see below):
//!
//! * **`Fearless Zeal`** (`uca_feats.lst:66`) -- the `.MOD BENEFIT:` row
//!   reads correctly through "...you can add a +2 bonus on any single
//!   attack roll, caster level check, saving throw, or skill check. You
//!   must choose to add this bonus after the die has been rolled and
//!   success or failure determined, but" and then splices verbatim into
//!   `Damned`'s own `BENEFIT:` row (`uca_feats.lst:37`) starting at "...
//!   before the DC of spells and spell-like abilities you use against such
//!   creatures." -- confirmed byte-for-byte against `Damned`'s row, not
//!   merely similar phrasing. `Damned`'s own row is otherwise real and
//!   distinct, not itself a splice -- two feats legitimately sharing a
//!   "+2 enhancement bonus to an ability score" completion tier is not,
//!   on its own, evidence of corruption (see the `Stronghold` correction
//!   below for why that distinction matters); this one is confirmed
//!   corrupted because `Fearless Zeal`'s own sentence changes subject
//!   mid-clause into `Damned`'s unrelated topic, not merely because the
//!   text repeats.
//! * **`Magnum Opus`** (`uca_feats.lst:74`) -- the row's own sentence is
//!   grammatically truncated in its own right, independent of any
//!   cross-row comparison: "...or win the artistic Completion
//!   Benefit:..." has no object after "artistic" -- a clause cut off
//!   mid-phrase. The corpus does not say what `Magnum Opus`'s own
//!   artistic-triumph goal actually was, so neither the `Goal:` clause nor
//!   whatever `Completion Benefit:` genuinely follows it can be displayed
//!   honestly.
//!
//! **`Stronghold` (`uca_feats.lst:76`) is NOT deferred -- corrected after
//! independent review found the first pass over-deferred it.** This
//! module's first version deferred `Stronghold` because its row carries a
//! second "Completion Benefit:" sentence, byte-for-byte identical to the
//! one appended to `Magnum Opus` above ("You gain the ability to reroll a
//! failed saving throw once per day..."). On its own, a *repeated*
//! sentence is not proof of corruption in this corpus -- `Damned` and
//! `Fearless Zeal` (before the fix above) demonstrate the file can carry
//! the same completion-tier text on two records, and one of those two
//! (`Damned`) is genuinely undamaged. What distinguishes a real splice
//! (`Fearless Zeal`, `Magnum Opus`) from a merely-repeated tier is whether
//! the record's OWN sentence is grammatically broken. `Stronghold`'s own
//! text terminates cleanly and completely with its own "Completion
//! Benefit:" clause -- "...you could grant your archers +2 on attack
//! rolls while your front line gains a +2 bonus to AC." -- a complete
//! sentence needing nothing after it. The second "Completion Benefit:"
//! sentence that follows is a verified-foreign fragment (byte-identical to
//! `Magnum Opus`'s row, and `Magnum Opus`'s own row independently proves
//! it belongs there, not here) rather than evidence that `Stronghold`'s
//! own content is damaged. `benefit` below carries `Stronghold`'s own
//! complete text ONLY, trimmed at the point its own sentence ends -- this
//! is not inventing or repairing text (no word is added or guessed), it
//! is declining to attribute a different record's sentence to this one,
//! the same species of judgment the ingest pipeline already exercises
//! when it decides which `.MOD` block's tokens belong to which feat.
//!
//! Re-derivation method: a sliding 10-word shingle comparison across all
//! 23 `BENEFIT:` rows (`python3` script, cycle receipt) flags any 10-word
//! run repeated verbatim across two different feats' rows as a splice
//! candidate; every hit traces to `Fearless Zeal`/`Damned` or `Magnum
//! Opus`/`Stronghold` plus one benign false positive (`Champion`/`Town
//! Tamer` sharing the generic "+1 dodge bonus to AC" phrase, which is not
//! a splice -- ordinary repeated game-mechanics language, not a
//! duplicated sentence boundary). The shingle scan finds *candidates*; it
//! is not, by itself, proof a given record's own text is damaged -- see
//! the `Stronghold` correction above for why each candidate still needs
//! its own record checked for internal grammatical completeness before
//! being deferred.
//!
//! Per `docs/governance/no-stub-mvp-doctrine.md` and the operator ruling
//! carried into this cycle, corrupted upstream text is never displayed to
//! a player and never repaired by inventing replacement prose -- and,
//! symmetrically, a record whose own text is genuinely intact is not
//! withheld from the player merely because it shares wording with another
//! record. Both deferred records ingest as real records
//! (`key`/`name`/`description`/`pretext`/`source_page` populated, all
//! independently correct) with `benefit: None` -- the engine's own
//! diagnostic (`corpus_ingest_diagnostic`, `wiring_class`) names the
//! defect with a file:line citation rather than silently omitting the
//! row. Honest target for this MODULE: **21 of 23 join real `BENEFIT:`
//! text (`benefit: Some(..)`), 2 are `deferred-with-reason`** -- not 23
//! joined, and not the first pass's 20+3.
//!
//! **Correction, `AT-34-E4-001` (SD-34): this is not the same claim as the
//! live atlas's `status`.** This module's own earlier wording called the
//! 21 "text-complete", which reads as the atlas bucket of that name. It
//! is not: `v06_work_inventory::classify` (via
//! `feat_desc_leaks_pi_or_upstream_marker`) demotes all 21 to
//! `unmeasurable`, because their SERVED (joined) description still opens
//! with the upstream `[Not Implemented]` editorial marker this module
//! documents above -- and that demotion is deliberate and
//! corpus-wide-consistent (`SD31-E2-F3-002` fixed a case-sensitivity gap
//! that let these exact 21 records previously reach `done` while a
//! byte-identical uppercase shape elsewhere did not; it chose uniform
//! demotion, not uniform promotion). Whether "marker + substantial real
//! mechanical prose" should ever read as complete is a genuine,
//! corpus-wide policy question (~392 marker occurrences project-wide,
//! `wiring_class::editorial_not_implemented_marker_is_detected_in_every_shipped_form`'s
//! own citation), not one this book's module doc gets to settle by
//! restating its own target as though it were the classifier's verdict.
//! See `feats_all::tests::uca_u_bucket_records_still_carry_the_editorial_marker_in_served_form`
//! for the mechanical proof, and
//! `artifacts/epic-4-ultimate-campaign/AT-34-E4-001_cycle_receipt.md` for
//! the full disposition of all 23 of this book's `U`/`X` units.
//!
//! **`PRETEXT:`, not a formal `PRE`-family token, on all 23.** Every
//! record's prerequisite is prose ("Prerequisite:You must have..."),
//! carried as the corpus's own `PRETEXT:` token rather than a structured
//! `PREABILITY:`/`PRESTAT:`/etc. Per the dated `decisions.md` entry for
//! this cycle, `PRETEXT:` is carried through verbatim as display
//! prerequisite text and never synthesised into a formal `PRE` token from
//! prose -- and this is established precedent, not a fresh ruling: ARG's
//! own `channel energy 2d6`/`channel energy 4d6`/`channel energy 6d6` rows
//! in `feats_all::ARG_FEAT_PREREQUISITES` already carry raw `PRETEXT:`
//! strings as prerequisite tokens for exactly this reason.
//!
//! Every field below is copied verbatim from the real corpus row (source:
//! `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/
//! ultimate_campaign/uca_feats.lst`), including the corpus's own kerning
//! artifacts (e.g. "Benef it", "conf lict", "Ref lex") -- not hand-
//! corrected, mirroring `pathfinder_unchained::feat_tables`'s own
//! "verbatim from the real corpus row" discipline and `crb`'s documented
//! `%%`-stays-escaped convention for literal `%` in corpus prose.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoryFeatEntry {
    /// The corpus record identity. No record in this catalog carries a
    /// distinct `KEY:` token of its own, so `key == name` for every entry,
    /// mirroring `pathfinder_unchained::feat_tables::FeatTableEntry.key`'s
    /// own documented fallback.
    pub key: &'static str,
    pub name: &'static str,
    /// The corpus `DESC:` token, verbatim -- always the upstream
    /// `[Not Implemented] ...` flavor line. `description` alone is never
    /// sufficient to avoid the stub doctrine; see this module's own doc
    /// comment for why `benefit` is what actually carries the mechanics.
    pub description: Option<&'static str>,
    /// The corpus `PRETEXT:` token, verbatim -- display prerequisite
    /// prose, not a formal `PRE`-family token. See this module's own doc
    /// comment ("PRETEXT:, not a formal PRE-family token").
    pub pretext: Option<&'static str>,
    pub source_page: Option<&'static str>,
    /// The corpus `.MOD BENEFIT:` row, verbatim -- the actual mechanical
    /// text a player needs. `None` for the three `deferred-with-reason`
    /// records (`Fearless Zeal`, `Magnum Opus`, `Stronghold`) whose own
    /// corpus row is corrupted; see this module's own doc comment for the
    /// file:line citation and re-derivation method for each. Never `Some`
    /// with corrupted or invented text.
    pub benefit: Option<&'static str>,
}

/// Records whose `.MOD BENEFIT:` row is a confirmed upstream splice --
/// `benefit: None` above, plus the engine's own verbatim diagnostic
/// naming the defect for anything that surfaces these records (reach
/// gate, work inventory, wiring-class determinator). Keyed by `key`.
pub const DEFERRED_WITH_REASON: &[(&str, &str)] = &[
    (
        "Fearless Zeal",
        "uca_feats.lst:66 -- .MOD BENEFIT: row reads correctly through \"...but\" then splices verbatim into Damned's own BENEFIT: row (uca_feats.lst:37) starting at \"before the DC of spells and spell-like abilities...\"; upstream corpus defect, not repaired by inventing text.",
    ),
    (
        "Magnum Opus",
        "uca_feats.lst:74 -- .MOD BENEFIT: row's own sentence is grammatically truncated (\"...or win the artistic Completion Benefit:...\", no object after \"artistic\"); the Goal: clause's real ending is not recoverable from the corpus; upstream corpus defect, not repaired by inventing text.",
    ),
];

/// Full UCA Story Feat catalog: all 23 real, distinct corpus records, in
/// source order. Built once and cached for the process lifetime.
pub fn feat_tables() -> &'static [StoryFeatEntry] {
    static TABLE: std::sync::OnceLock<Vec<StoryFeatEntry>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        vec![
            StoryFeatEntry {
                key: "Accursed",
                name: "Accursed",
                description: Some("[Not Implemented] Your curse weighs down your soul like a millstone around your neck."),
                pretext: Some("Prerequisite:You must carry a curse that can be lifted only by a quest or similar great undertaking, or have the Cursed Birth background."),
                source_page: Some("p.67"),
                benefit: Some("You gain spell resistance equal to 5 + your character level, as the curse interferes with all magic. Unlike most spell resistance, it can't voluntarily be lowered, though your own spells and magic items still automatically affect you. Goal:Your curse is lifted or you are able to purge the corruption of your fiendish blood (the circumstances of either vary widely based on the nature of the curse, and are up to the GM). Completion Benefit:You lose the spell resistance described above. You gain spell resistance equal to 11 + your character level, but only against harmful enchantment, necromancy, and transmutation spells and spell-like abilities-your aura resists further attempts to curse you. Harmless spells automatically bypass this spell resistance whether you desire it or not. This spell resistance can be voluntarily lowered."),
            },
            StoryFeatEntry {
                key: "Arisen",
                name: "Arisen",
                description: Some("[Not Implemented] Escaping death strengthened your bond to life, but fills you with a need for answers."),
                pretext: Some("Prerequisite:You must have been slain and brought back from the dead, or have the Left to Die or Cursed Birth background."),
                source_page: Some("p.67"),
                benefit: Some("You don't die until your negative hit point total is equal to or greater than 4 + your Constitution score. Once per day as a standard action, you can force yourself to carry on by strength of will alone, gaining 1 temporary hit point per hit die. These temporary hit points last for 10 minutes. Normal:You die when your negative hit point total is equal to or greater than your Constitution score. Goal:You meet in person and hear the words of your deity or your deity's chosen herald. If you worship a pantheon of deities, you must meet and hear a member of that pantheon-a herald does not suffice in this case. If you worship no specific deity, you must hear the words of an appropriate entity of the GM's choice. Completion Benefit:You gain a +2 bonus on saving throws against death effects and fear effects. In addition, the caster level of any conjuration (healing) spell that is cast on you increases by 1 for the purposes of its effects on you alone."),
            },
            StoryFeatEntry {
                key: "Battlefield Healer",
                name: "Battlefield Healer",
                description: Some("[Not Implemented] In even the fiercest battles, you risk life and limb to save your allies."),
                pretext: Some("Prerequisite:You must successfully cast a conjuration (healing) spell on an ally after being hit by an attack of opportunity, or have the Battle, Chaplain, or Healed background."),
                source_page: Some("p.67"),
                benefit: Some("When attempting a concentration check caused by receiving damage (including ongoing damage), you reduce the damage taken by 50%% for the purposes of determining the concentration check DC. Goal:Over time, provoke at least 20 attacks of opportunity for casting conjuration (healing) spells on allies. These spells don't have to succeed to count. Completion Benef it:You automatically succeed at concentration checks for conjuration (healing) spells caused by taking damage."),
            },
            StoryFeatEntry {
                key: "Champion",
                name: "Champion",
                description: Some("[Not Implemented] You must prove yourself through single combat."),
                pretext: Some("Prerequisite:You must have defeated a single challenging foe without any aid from another, or have the Champion of a God, Champion of the People, Competition Champion, or Gladiator background."),
                source_page: Some("p.67"),
                benefit: Some("As a swift action, you can declare a single combat challenge to one foe within 50 feet and in line of sight. Upon doing so, you gain a +1 bonus on attack rolls and a +1 dodge bonus to AC against that foe as long as no one else threatens that opponent or until the single combat challenge ends. If another combatant attacks you or your foe, the challenge ends and you take a -2 penalty on attack rolls and to AC for 1 round. Though you can declare a single combat challenge at will, once you declare it on a foe you can't declare it on the same foe for another 24 hours. Goal:Defeat an appropriate number of challenging foes in single combat. These combats must not be interrupted by other creatures, and the foes must not have already been substantially injured or impaired prior to combat with you. Completion Benefit:Your bonuses for single combat increase to +2. In addition, any confirmed critical hits you make against such a foe deal an additional 1d6 points of damage."),
            },
            StoryFeatEntry {
                key: "Damned",
                name: "Damned",
                description: Some("[Not Implemented] From your earliest days, you were destined to sacrif ice everything in your quest for power."),
                pretext: Some("Prerequisite:You must have had friendly contact with an evil-aligned outsider that would qualify as a challenging foe, have a fiend-related sorcerous bloodline such as abyssal or infernal, have direct fiendish ancestry (such as being a tiefling or half-fiend), or have the Fiend Raised or The Fiend background."),
                source_page: Some("p.67"),
                benefit: Some("You gain a +2 bonus on Charisma-based checks involving evil-aligned outsiders and +1 bonus to the DC of spells and spell-like abilities you use against such creatures. You take a -2 penalty on Charisma-based checks involving good-aligned outsiders. Goal:Successfully trade your soul to an evil outsider. Completion Benefit:You gain a +2 enhancement bonus to an ability score of your choice. This enhancement bonus can't be dispelled or removed save by the direct intervention of a deity, and counts as a supernatural ability. In addition, you gain a +2 bonus on caster level checks (including dispel checks and checks to bypass spell resistance) against goodaligned outsiders. If you die while under the effects of this agreement, you can't be brought back from the dead unless the evil outsider permits it. You lose your completion benefits immediately and permanently if you renege on the arrangement by which you traded your soul, though you keep the feat's basic benefits."),
            },
            StoryFeatEntry {
                key: "Deny the Reaper",
                name: "Deny the Reaper",
                description: Some("[Not Implemented] The lives you could not save stay with you to your final breath."),
                pretext: Some("Prerequisite:You must have witnessed the death of a close companion in battle-a death that could have been prevented, such as from bleeding, failure to stabilize, or ongoing poison damage-or have the Death in the Family or The War background."),
                source_page: Some("p.68"),
                benefit: Some("You gain a +2 bonus on Heal checks. If you have 10 or more ranks in Heal, this bonus increases to +4. You can apply first aid as a move action and don't take a penalty when treating deadly wounds without a healing kit. Goal:Bring an ally back from the dead, including by using breath of life or reincarnate. Completion Benef it:You and each ally within 10 feet of you gain a +2 bonus on saves against death effects. In addition, once per day you can spontaneously convert any 5th-level or higher conjuration (healing) spell into breath of life."),
            },
            StoryFeatEntry {
                key: "Eldritch Researcher",
                name: "Eldritch Researcher",
                description: Some("[Not Implemented] You seek new applications for magical energy."),
                pretext: Some("Prerequisite:You must have created a new spell, or have The Way Things Work background."),
                source_page: Some("p.68"),
                benefit: Some("When casting a spell you've created, add 1 to your caster level. In addition, you gain a +2 bonus on Spellcraft checks. If you have 10 or more ranks in Spellcraft, this bonus increases to +4. Goal:Create a new spell of at least 6th level. Completion Benefit:The save DCs for any spells you create increase by 1 when you cast them. In addition, when applying metamagic feats to self-created spells, reduce the total level adjustment by 1. You can't reduce metamagic costs to lower than the spell's original level in this manner. Special:For a self-created spell to benef it from this feat, it must be a truly novel spell. Spells slightly altered from the original (for example, delayed blast fireball as compared to fireball) gain no benef it. Alchemists can benef it from this feat as though their formulae and extracts were spells."),
            },
            StoryFeatEntry {
                key: "Fearless Zeal",
                name: "Fearless Zeal",
                description: Some("[Not Implemented] You're willing to lay down your life for your faith."),
                pretext: Some("Prerequisite:You must be ordained as a sacred (or profane) champion of your faith by a high-ranking member of its clergy, or have the Devoted, Faith-Bringer, or Moral Debt background. Such an honor goes above and beyond the normal oaths required of a cleric or paladin."),
                source_page: Some("p.68"),
                benefit: None,
            },
            StoryFeatEntry {
                key: "Feral Heart",
                name: "Feral Heart",
                description: Some("[Not Implemented] In your chest beasts the heart of a wild beast."),
                pretext: Some("Prerequisite:You must have reverted to savage behavior through a traumatic event or extended period in the wilderness, or have the Raised by Beasts background."),
                source_page: Some("p.69"),
                benefit: Some("Whenever you receive a morale bonus on Strength or attack rolls (such as from heroism or the barbarian rage class feature), you receive a +2 bonus on Dexterity- and Strength-based ability and skill checks and a +1 bonus on Ref lex saves until the morale bonus effect ends. Goal:Woo and then marry or otherwise enter into a committed relationship with a person from a civilized culture. This relationship must be forged in love, not bought or coerced. Completion Benefit:Delay the penalties for the exhausted, fatigued, shaken, and sickened conditions for 1 round after first receiving them. If you already have the condition in question and it is applied again, this feat provides no benefit. Despite ignoring the penalties you do still have the condition. For example, if you become shaken again while benefiting from this feat, you become frightened as normal."),
            },
            StoryFeatEntry {
                key: "Foeslayer",
                name: "Foeslayer",
                description: Some("[Not Implemented] Your bitter feud with your enemies can be quenched only with blood."),
                pretext: Some("Prerequisite:You must have been defeated and robbed of at least half your possessions by a particular group of humanoids or monstrous humanoids, or have the An Eye for an Eye, Hated Foe, Raiders, or Vengeance background. You may choose a specific race, such as duergar, or a broader group, such as goblinoids. At the GM's option, you may instead choose residents of a particular country, settlement, or tribe."),
                source_page: Some("p.69"),
                benefit: Some("The save DCs for your spells or abilities increase by 1 when you use them against the chosen group, and you gain a +1 dodge bonus to AC against their attacks. Goal:Slay an appropriate number of challenging foes. Completion Benef it:You gain the benefits of the Improved Critical feat on attacks made against members of your chosen race. Your dodge bonus to AC against such foes increases to +2."),
            },
            StoryFeatEntry {
                key: "Forgotten Past",
                name: "Forgotten Past",
                description: Some("[Not Implemented] A pivotal event from your past eludes your memory."),
                pretext: Some("Prerequisite:You must have suffered permanent memory loss or have the Reincarnated background."),
                source_page: Some("p.69"),
                benefit: Some("The duration of mind-affecting spells (even beneficial ones) is halved for you, to a minimum of 1 round. Your inquisitive nature gives you a +2 bonus on Perception checks. If you have 10 or more ranks in Perception, this bonus increases to +4. Goal:Regain a major portion of your lost memories. The exact means varies, possibly requiring a wish, assistance from a divine being, reliving a past life, or confronting the situation that led to your memory loss. This process must involve encountering a challenging foe, though possibly in ways other than direct confrontation. Completion Benefit:You roll twice whenever you attempt a saving throw against a mind-affecting effect, keeping the better result. Special:Restoration of memories by means less significant than miracle or wish does not qualify for the prerequisite."),
            },
            StoryFeatEntry {
                key: "Glimpse Beyond",
                name: "Glimpse Beyond",
                description: Some("[Not Implemented] You have glimpsed the madness at the edges of reality."),
                pretext: Some("Prerequisite:You must have faced an undead, evil outsider, or aberration with a CR greater than your level +4, or have the Raised Among the Dead or The Dead One background."),
                source_page: Some("p.69"),
                benefit: Some("You gain a +2 bonus on Knowledge (dungeoneering) checks to identify the vulnerabilities and powers of aberrations, Knowledge (planes) checks to identify the vulnerabilities and powers of evil outsiders, and Knowledge (religion) checks to identify the vulnerabilities and powers of undead, and you can make such checks untrained. If you have 10 or more ranks in any of these Knowledge skills, the bonus increases to +4 for the appropriate skill. In addition, you gain a +2 bonus on saves against fear effects. Goal:Be killed or driven insane (as determined by the GM) by an aberration, evil outsider, or undead. This leaves your mind permanently marked. Completion Benefit:Any sane creature that attempts to read your thoughts takes 1d6 points of Wisdom damage (Will DC 10 + 1/2 your level + your Charisma modified negates). In addition, the effect of any ability damage, ability drain, or penalty to your Intelligence, Wisdom, or Charisma is halved (minimum 1). You take a -2 penalty on Will saving throws. Whenever you roll a save against a mind-affecting effect, roll twice and keep the better result."),
            },
            StoryFeatEntry {
                key: "Innocent Blood",
                name: "Innocent Blood",
                description: Some("[Not Implemented] With their deaths, the pitiful wretches that inhabit this world open your path to greatness."),
                pretext: Some("Prerequisite:You must slay at least 50 intelligent noncombatants for either your own personal gain or for no cause at all, or have the Bloodthirsty, First Kill, or The Kill background."),
                source_page: Some("p.69"),
                benefit: Some("You gain a +2 bonus on Intimidate checks. If you have 10 or more ranks in Intimidate, this bonus increases to +4. Each time you slay an intelligent creature, you gain a +1 bonus on attack rolls and caster level checks for 1 minute (this bonus does not stack with itself ). Goal:Slay at least 200 more intelligent noncombatants, then slay a challenging foe that seeks to either bring you to justice for your crimes or usurp your position. Completion Benefit:Any shaken creature takes double the normal penalties when attacking you, making saves against your abilities, or resolving skill checks with you as a target."),
            },
            StoryFeatEntry {
                key: "Liberator",
                name: "Liberator",
                description: Some("[Not Implemented] Your time in shackles has forever marked your soul."),
                pretext: Some("Prerequisite:You must have been enslaved for at least 6 months, or have the Imprisoned or Kidnapped background."),
                source_page: Some("p.70"),
                benefit: Some("You gain a +1 bonus on attack rolls, weapon damage rolls, and skill checks when your actions would directly lead to freeing prisoners or slaves. Goal:Free at least 200 slaves through perilous rescues (not merely buying them at market). Completion Benefit:You gain the ability to inspire others through your dedication to your cause. Allies within 20 feet receive your Liberator feat bonuses when working with you to free prisoners or slaves. In addition, as a standard action you can inspire slaves and former slaves within 120 feet, giving them temporary hit points equal to 1/2 your character level and a +1 bonus on saving throws. These benefits last for 1 hour, and a given creature can receive this benefit only once per day. These are mind-affecting effects, and the inspiring bonus is language-dependent."),
            },
            StoryFeatEntry {
                key: "Lost Legacy",
                name: "Lost Legacy",
                description: Some("[Not Implemented] What once belonged to your family shall be yours again."),
                pretext: Some("Prerequisite:Your family must have claim to an inherited title or position that no longer belongs to them, or have the Dishonored Family background. You can take this feat even if you have no knowledge of this lost family title."),
                source_page: Some("p.70"),
                benefit: Some("You gain a +1 bonus on Charisma-based ability checks and skill checks. Goal:Regain your family's lost claim, either for yourself or another in your family. In the process of completing this claim, you must decisively defeat a challenging foe that seeks to deny your birthright. Completion Benefit:You gain a +1 bonus on Wisdom ability checks, Wisdom-based skill checks, and Will saving throws. Special:If you manage to regain your position without defeating a challenging foe, you may still complete this story feat at a later date if a suitable challenging foe attempts to steal your birthright again."),
            },
            StoryFeatEntry {
                key: "Magnum Opus",
                name: "Magnum Opus",
                description: Some("[Not Implemented] You seek to create a true masterpiece."),
                pretext: Some("Prerequisite:You must either have sold five or more self-created works of art worth a total of at least 5,000 gp, have performed at least five performances for audiences of 50 or more while achieving a great performance result or better on your Perform check, or have the Virtuoso background."),
                source_page: Some("p.70"),
                benefit: None,
            },
            StoryFeatEntry {
                key: "Shamed",
                name: "Shamed",
                description: Some("[Not Implemented] A past humiliation haunts you to this day."),
                pretext: Some("Prerequisite:You must have been publicly embarrassed, or must have the Bastard Born background. If the embarrassment didn't cause significant harm to your personal honor or social standing, it does not qualify for the feat prerequisites. The humiliation doesn't need to have been unjustified."),
                source_page: Some("p.71"),
                benefit: Some("Being observed drives you to excel. When you're in a conf lict that is being observed by others not involved in the conf lict, you gain a +1 bonus on attack rolls and skill checks. Goal:You can complete this goal in one of two fashions. First, thwart a chosen foe in a fashion that clearly establishes your superiority to the general public. Second, prove your worth another way, like gaining a title or becoming the chosen hero of a region. Completion Benefit:Your newfound confidence gives you temporary hit points equal to your character level. These temporary hit points last until lost and refresh any time you rest long enough for natural healing to occur (Core Rulebook 191), whether or not any healing actually occurs."),
            },
            StoryFeatEntry {
                key: "Stronghold",
                name: "Stronghold",
                description: Some("[Not Implemented] You seek to build a bastion against which your enemies shall break like water against the rocks."),
                pretext: Some("Prerequisites:You must have the Leadership feat and must lead at least 10 combat-capable followers (such as fighters or rangers)."),
                source_page: Some("p.71"),
                // Trimmed at the point this record's own sentence ends --
                // see this module's own doc comment ("Stronghold is NOT
                // deferred") for why the corpus row's second, unrelated
                // "Completion Benefit:" sentence (byte-identical to Magnum
                // Opus's row) is excluded rather than attributed here.
                // Nothing after this point is added, guessed, or
                // paraphrased -- every word up to the trim point is
                // verbatim uca_feats.lst:76.
                benefit: Some("You can spend a move action to give battle orders to your troops, granting creatures under your command within 60 feet your choice of a +1 morale bonus on attack rolls, a +1 dodge bonus to AC, or a +1 bonus on a single type of saving throw. All creatures must receive the same benefit. You can't use this benefit on allies not under your command. This is a language-dependent, mind-affecting effect. Goal:Build or capture a stronghold capable of housing a force of at least 200 troops, and staff it with at least 100 combat-capable soldiers (or the equivalent) under your command. You must also provide food and water sufficient to survive at least a 6-month siege and a gold reserve sufficient for at least 6 months of wages if your troops require pay. Completion Benefit:Your battle order bonuses improve to +2, and the range of your orders increases to 120 feet. In addition, you can give two different orders to your troops. For example, you could grant your archers +2 on attack rolls while your front line gains a +2 bonus to AC."),
            },
            StoryFeatEntry {
                key: "Thief of Legend",
                name: "Thief of Legend",
                description: Some("[Not Implemented] More than just a burglar, you aspire to commit crimes of legendary stature."),
                pretext: Some("Prerequisites:You must have stolen at least 1,000 gp worth of treasure without being caught and kept mementos of these thefts worth at least 500 gp, or have the Greed background."),
                source_page: Some("p.72"),
                benefit: Some("Once per day when you attempt a Disable Device check to open a lock or Sleight of Hand check to pick a pocket, you can roll twice and take the better result. Goal:Steal a famous and well-guarded treasure worth at least 50,000 gp while leaving no evidence of your involvement behind. The treasure must be protected by a mix of at least 8 traps or challenging foes. Any guardians need not be defeated, merely bypassed. You don't need to keep the treasure, and you can boast of the theft afterward. Completion Benefit:You gain the ability to reactively disarm a trap. When you trigger a trap, you can attempt a Disable Device check with a -5 penalty to interrupt the trap's function, leaving it still armed but effectively preventing it from activating. You can use this ability a number of times per day equal to 3 + your Intelligence modifier. Since the trap is still armed, it might activate again if you don't immediately back away or otherwise avoid repeating the act that set it off the first time."),
            },
            StoryFeatEntry {
                key: "Town Tamer",
                name: "Town Tamer",
                description: Some("[Not Implemented] This town needs cleaning up, and you're just the one to do it."),
                pretext: Some("Prerequisites:You must have 5 ranks in Intimidate and a personal motivation to clean up a particular town (such as an old friend calling in a favor, or seeking a place to settle down), or you must have the Bounty Hunter or Champion of the People background."),
                source_page: Some("p.72"),
                benefit: Some("Choose a particular settlement. When you're in your chosen settlement, the DC of Intimidate checks made against you increases by 10. You gain a +1 bonus on attack rolls and a +1 dodge bonus to AC against undesirable elements like criminals or ruffians in your chosen settlement. This bonus increases to +2 for combat maneuver checks. Goal:Defeat 10 more troublemakers in your chosen settlement with a CR equal to your character level or higher, and in the process eliminate a serious criminal threat or otherwise clean up the streets. Completion Benefit:You gain the ability to designate a new chosen settlement as often as you wish. You must first live in a settlement for 1 week to make it your chosen settlement. In addition, you gain a +2 bonus on initiative checks and a +1 bonus on saving throws when in your chosen settlement."),
            },
            StoryFeatEntry {
                key: "True Love",
                name: "True Love",
                description: Some("[Not Implemented] You found love, only to have it denied by the cruelty of fate."),
                pretext: Some("Prerequisite:You must have found love with a person you can't be with, have a current lover, or have the Current Lover, For Love, or The Lover background. Possible complications include distance, your love being with another, your feelings being unrequited, or your relationship being forbidden."),
                source_page: Some("p.72"),
                benefit: Some("You add 1 to the save DC and caster level of your spells and spell-like abilities with the emotion descriptor. In addition, you gain a +2 bonus on Sense Motive checks. If you have 10 or more ranks in Sense Motive, this bonus increases to +4. Goal:Find a way to be with your true love (even if you can't formally wed). Completion Benefit:The inspiration of knowing your love waits for your return gives you a +2 bonus on attack rolls, saving throws, and skill checks whenever you are below a quarter of your total hit points (not counting any temporary hit points). You lose this completion benefit if your relationship with your true love comes to an end for any reason, including death. Special:At the GM's discretion, you can find true love with a person other than the one you designated when you chose this feat. In this case, the love you initially chose was wrong for you, but this became obvious only when you found the one truly meant for you."),
            },
            StoryFeatEntry {
                key: "Unforgotten",
                name: "Unforgotten",
                description: Some("[Not Implemented] You search for a person dear to you-lost, but you pray not dead."),
                pretext: Some("Prerequisite:You must have a close relative, spouse, or other person dear to your heart who never returned from a journey, was captured, or otherwise vanished with little trace, or you have the Major Disaster background."),
                source_page: Some("p.72"),
                benefit: Some("Your dogged determination reduces any nonlethal damage you take by 1 point, to a minimum of 1 point of nonlethal damage. You also gain a +1 bonus on Will saves. Goal:Find your lost loved one alive, and in the process, decisively defeat a challenging foe who kept you apart. Completion Benefit:You gain a +1 bonus on all saving throws, replacing the +1 bonus on Will saves. If you find your loved one dead, you lose all benefits from this feat until you put the body and possibly soul to proper rest. Putting the body to rest restores the completion benefit, but you don't regain the ability to reduce nonlethal damage."),
            },
            StoryFeatEntry {
                key: "Vengeance",
                name: "Vengeance",
                description: Some("[Not Implemented] The need to avenge those you loved drives you to great deeds."),
                pretext: Some("Prerequisite:You must have a close family member or other loved one slain by a specific challenging foe or that foe's minions, or have the Raiders or Vengeance background."),
                source_page: Some("p.72"),
                benefit: Some("You gain a +1 bonus on saving throws, attack rolls, and weapon damage rolls against your chosen foe and known minions of that foe. Goal:Thwart your chosen foe. Completion Benef it:You gain a +1 bonus on all saving throws. This bonus stacks with this feats's bonus against your foe and its minions, should they survive."),
            },
        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_has_23_records() {
        assert_eq!(feat_tables().len(), 23);
    }

    #[test]
    fn every_record_carries_desc_and_pretext() {
        for e in feat_tables() {
            assert!(e.description.is_some(), "{} has no DESC:", e.key);
            assert!(e.pretext.is_some(), "{} has no PRETEXT:", e.key);
        }
    }

    #[test]
    fn exactly_two_are_deferred_with_reason() {
        let deferred: Vec<&str> = feat_tables()
            .iter()
            .filter(|e| e.benefit.is_none())
            .map(|e| e.key)
            .collect();
        assert_eq!(deferred, vec!["Fearless Zeal", "Magnum Opus"]);
        assert_eq!(DEFERRED_WITH_REASON.len(), 2);
    }

    #[test]
    fn twenty_one_are_text_complete_with_real_benefit_text() {
        let complete = feat_tables().iter().filter(|e| e.benefit.is_some()).count();
        assert_eq!(complete, 21);
        for e in feat_tables() {
            if let Some(b) = e.benefit {
                assert!(!b.is_empty());
                assert!(b.len() > 20, "{} benefit text looks too short to be real", e.key);
            }
        }
    }

    /// `Stronghold`'s own text is genuinely complete -- it must not be
    /// truncated at the point the excluded, foreign sentence used to
    /// start, and it must not silently regain that foreign sentence
    /// either.
    #[test]
    fn strongholds_benefit_is_its_own_complete_text_and_excludes_the_foreign_tail() {
        let stronghold = feat_tables().iter().find(|e| e.key == "Stronghold").unwrap();
        let benefit = stronghold.benefit.expect("Stronghold is text-complete, not deferred");
        assert!(benefit.ends_with("gains a +2 bonus to AC."), "must end on Stronghold's own sentence");
        assert!(
            !benefit.contains("reroll a failed saving throw"),
            "must not carry Magnum Opus's foreign trailing sentence"
        );
    }

    #[test]
    fn deferred_with_reason_keys_match_catalog() {
        let catalog_keys: std::collections::BTreeSet<&str> = feat_tables().iter().map(|e| e.key).collect();
        for (key, reason) in DEFERRED_WITH_REASON {
            assert!(catalog_keys.contains(key), "{key} not in catalog");
            assert!(reason.contains("uca_feats.lst:"), "{key} reason has no file:line citation");
        }
    }
}
