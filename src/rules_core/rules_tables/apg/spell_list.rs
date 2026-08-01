//! APG shared spell list — full corpus ingest (SD-24 Epic 6 criteria
//! 6.2/6.5).
//!
//! Source: every real, active (non-`.MOD`, non-comment,
//! non-`SOURCELONG`-header) base spell record in PCGen's `apg_spells.lst`
//! (Advanced Player's Guide), 297 total after deduplication (see the
//! `## DISCOVERED` correction note below). Not hand-authored; generated
//! by a one-off ingestion script against
//! `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_spells.lst`
//! (reachable from this environment) — do not hand-edit, regenerate if
//! the corpus changes.
//!
//! **Correction to the SD-24 criterion 6.1 audit's documented count**:
//! the audit recorded 298 real, active `apg_spells.lst` records. The
//! real corpus has exactly one genuine duplicate base record — two full
//! `Resounding Blow` lines (`apg_spells.lst:10` and `:196`, the second an
//! errata-reprint with an expanded `CLASSES:` token merging in
//! `Resounding Blow.MOD`'s Paladin/Inquisitor grant) — that the audit's
//! `sort -u`-on-name-column methodology should have deduplicated, and did
//! not (the two lines are byte-for-byte distinct due to differing
//! whitespace, so `sort -u` treated them as distinct records). The real
//! deduplicated-by-name count is 297. This module ingests the *last*
//! occurrence of any duplicate name (the later, more complete
//! errata-reprint row), mirroring `crb::equipment_data`'s own SD-17
//! `KEY:`-merge-dedup precedent. See this cycle's `## DISCOVERED` entry
//! (`epic-6-audit-correction`).
//!
//! **Field-sourcing methodology, per record:**
//! - `school`: the corpus `SCHOOL:` token, mapped through
//!   [`Pf1SchoolId::from_corpus_str`]. `None` when the line carries no
//!   `SCHOOL:` token at all (a real corpus gap — see below).
//! - `level`: the minimum spell level across every class=level pair in
//!   every pipe-delimited `CLASSES:` group on the line (e.g.
//!   `CLASSES:Bard=2|Cleric,Inquisitor,Sorcerer,Wizard=3` -> `Some(2)`).
//!   `None` when the line carries no `CLASSES:` token at all.
//! - `description`: the corpus's short `DESC:` summary line (trailing
//!   `|!PRERULE:...`/`|PRERULE:...` formula annotations stripped) when no
//!   richer source is available; the real full SRD/PRD text from a
//!   matching `<Name>.MOD` record's own `DESC:` token when one exists
//!   (`full_text: true` in that case — see criterion 6.5). `None` when
//!   the base record has no `DESC:` token and no matching `.MOD` record
//!   supplies one either.
//! - PCGen `.COPY=` variant records (e.g.
//!   `Planar Binding.COPY=Planar Binding (Demons Only)`, a named variant
//!   that inherits its base record's fields unless it overrides them):
//!   `key` is the variant's own display name (the text after `.COPY=`),
//!   and `school`/`level`/`description` fall back to the referenced base
//!   record's own fields when the variant line itself doesn't carry them.
//!   A handful of these variants' bases (e.g. the `Planar Binding`/
//!   `Planar Ally`/`Summon Monster` family) are themselves core-rulebook
//!   spells living in `cr_spells.lst`, not `apg_spells.lst`.
//!
//!   **That cross-book boundary is now resolved (SD-27, 2026-07-31), and
//!   it used to say "future work".** It cost a player eleven blank rows:
//!   a `.COPY=` row carries no `SCHOOL:`, `CLASSES:` or `DESC:` token of
//!   its own, so eleven variants whose base is a CRB spell arrived at
//!   `list_spell_catalog` as a key and three nulls and rendered as empty
//!   columns. Each now inherits its base record's school, level and
//!   description from [`crb::spell_list`](crate::rules_core::rules_tables::crb::spell_list),
//!   marked `// Inherited from CRB ...` at the entry. The values are
//!   literals — this is a flat `const` table and cannot call a lookup at
//!   compile time — so
//!   `tests/sd27_apg_delta_spell_rows_resolve_against_their_base.rs`
//!   re-reads the CRB record at runtime and requires equality, which is
//!   what keeps a copy from drifting from what it copied.
//!
//!   All 15 cross-book `.COPY=` rows are resolved the same way, including
//!   the four that were only *partly* resolved before (three
//!   `Summon Monster` variants that had a school and text but no level,
//!   and `Call Lightning Storm (Starsoul)`, which had neither).
//!   `Call Lightning Storm (Starsoul)` keeps its own `description`: its
//!   `.MOD` record at `apg_spells.lst:1075` overrides `DESC:` outright,
//!   which is PCGen's layering — `.COPY=` takes the base, then a `.MOD`
//!   replaces individual fields — so inheriting school and level while
//!   keeping the overridden text is the faithful reading, not an
//!   exception to it.
//!
//!   Note on `CLASSES:.CLEARALL`: eleven of these variants carry a
//!   `<variant>.MOD CLASSES:.CLEARALL` line, which strips the copied class
//!   list so the variant is granted by a subdomain rather than by a class
//!   list. Their `level` still reports the base spell's level, matching
//!   both the in-book precedent set by `Elemental Aura (Cold Only)` and
//!   the `DOMAINS:...=N` grant level on their own `.MOD` rows (Planar
//!   Binding's variants are `=6`, and Planar Binding is a 6th-level
//!   spell). A blank level would be a worse answer, not a stricter one.
//!
//! **Real, honest, corpus-level gaps (not fabricatable, not a parsing
//! defect):** 41 of 297 records lack a `SCHOOL:` and/or `CLASSES:` token
//! entirely — mostly Summoner eidolon-only spells (`Evolution Surge`,
//! `Purified Calling`, `Rejuvenate Eidolon`, ...) which the real PF1
//! rules grant automatically rather than placing on a leveled spell list,
//! plus a few `PRESPELL`-linked variant records (`Threefold Aspect
//! (Young Adult)` etc.) and one corpus data-quality issue (`Wall of
//! Thorms` — the corpus's own typo for "Wall of Thorns", confirmed by its
//! `SOURCELINK` pointing at `wallOfThorns`). `school`/`level` are `None`
//! for these by design — inventing a school/level for them would be
//! fabrication, not ingestion.
//!
//! **Summoner's own "Summoner Spells - APG" block
//! (`apg_spells.lst:471` onward) is still entirely `#`-commented out** in
//! the real corpus — this module's ingest correctly excludes it (comment
//! lines are skipped), same finding the prior bootstrap-era doc comment
//! already recorded.
//!
//! **Cavalier casts no spells** (see `class_cavalier.rs`'s doc comment:
//! no `SPELLSTAT:` token on the real `CLASS:Cavalier` record), so it has
//! no distinguished row here by design — same as the prior bootstrap.
//!
//! **SD-25 criterion 7.N ("apg-spell-text") full-text ceiling raise —
//! 261 → 284 of 297 `full_text: true`:** the criterion 6.5 measurement
//! above (261/297) undercounted by 23 real, sourced full-text records for
//! two distinct reasons, both corrected this cycle:
//!
//! 1. **A same-line-concatenation ingest miss (13 records; no web fetch
//!    needed, entirely corpus-native).** `apg_spells.lst` contains three
//!    physical lines where two `.MOD` stanzas were concatenated onto a
//!    single line with no line break (`apg_spells.lst:1945`,
//!    `Fester (Mass).MOD ... Fiery Body.MOD ...`; `apg_spells.lst:2094`,
//!    `Transmogrify.MOD ... Transmute Potion to Poison.MOD ...`). The
//!    prior ingest attributed the line's *last* `DESC:` token to the
//!    line's *first*-named record, silently swapping two unrelated
//!    spells' full text (`Fester (Mass)` got `Fiery Body`'s text;
//!    `Transmogrify` got `Transmute Potion to Poison`'s text) — `Fiery
//!    Body` and `Transmute Potion to Poison` themselves were left with
//!    only their short summaries. Both pairs are now correctly
//!    attributed (`Fester (Mass)` drops from a wrongly-`full_text: true`
//!    state to its own correct short-only `false`; `Fiery Body` and
//!    `Transmute Potion to Poison` gain their real, corpus-native full
//!    text). Separately, `Summon Monster I`'s own base line
//!    (`apg_spells.lst:649`) carries two back-to-back `DESC:` tokens (a
//!    short summary, then the full SRD paragraph) with no separate `.MOD`
//!    involved at all; the full paragraph was already captured as
//!    `description` pre-cycle but `full_text` was left `false` because
//!    the ingest's "sourced from a matching `.MOD` record" rule didn't
//!    recognize a same-line double-`DESC:` pattern as a full-text source.
//!    Corrected to `true` for `Summon Monster I`-`IX` and their `.COPY=`
//!    variants (`Summon Monster III/VII (Reptiles Only)`, `Summon Monster
//!    V (Summons 1d3 Shadows)`) — no content change, just the flag.
//! 2. **A web second-source pass (6 records: `Fester`,
//!    `Heroic Fortune`, `Heroic Fortune (Mass)`, `Malediction`,
//!    `Severed Fate`, `Unravel Destiny`) plus one more corpus-native
//!    `.COPY=`-own-`.MOD` catch (`Call Lightning Storm (Starsoul)`).**
//!    `Fester` was identity-matched (name + School + exact Level/
//!    Components) against `https://www.d20pfsrd.com/magic/all-spells/f/fester/`.
//!    The 5 "hero point" spells (APG's `advancedNewRules.html` variant
//!    subsystem) were identity-matched (name + School + exact Level/
//!    `CLASSES:` set) against
//!    `https://legacy.aonprd.com/advancedPlayersGuide/advancedNewRules.html`
//!    — a first `d20pfsrd.com` search for "Malediction" surfaced only a
//!    same-named, later "Book of the Damned" reprint (Antipaladin/
//!    Cleric/Shaman/Spiritualist/Witch 4, an unrelated damned-soul
//!    mechanic), which was rejected as a cross-book/edition-cousin false
//!    match per this cycle's identity-match rule before falling back to
//!    the correct `legacy.aonprd.com` APG-original page. `Call Lightning
//!    Storm (Starsoul)`'s own `.MOD` record (`apg_spells.lst:1075`)
//!    supplies a full override `DESC:`, corpus-native.
//!
//! Remaining `full_text: false`/`description: None` records (13/12 of
//! 297) are the documented cross-book `.COPY=` variants whose base spell
//! lives outside `apg_spells.lst` (`Planar Binding` ×6, `Planar Ally` ×3,
//! `Beast Shape I (Animals Only)`, `Blindness/Deafness (Only Cause
//! Blindness)`, `Meteor Swarm (Dealing Cold Damage)`) plus the corpus-typo
//! `Wall of Thorms` — all deliberately left `None` per this module's
//! documented cross-book scope boundary (see above), never fabricated.
//! The 3 `Threefold Aspect` `PRESPELL` sub-forms (`Young Adult`/
//! `Adulthood`/`Elderly`) are **not** in this remaining-gap set: their
//! base spell (`Threefold Aspect`) lives in this *same* book, so this
//! cycle extended the module's existing `.COPY=`-fallback convention to
//! this same-book `PRESPELL` case too (a deliberate, documented method
//! choice, not a web claim) — all 3 now carry the base's full text.

use crate::rules_core::rules_tables::RuleSetId;

/// The 8 canonical PF1 arcane/divine spell schools that appear in the
/// real `apg_spells.lst` corpus (widened from the SD-22 bootstrap's
/// 3-school subset to the full set actually present — see
/// `rules_tables::crb::spell_list::Pf1SchoolId` for CRB's own, separately
/// maintained copy of this enum).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Pf1SchoolId {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}

impl Pf1SchoolId {
    /// Maps the corpus's raw `SCHOOL:` string to this enum. `None` for
    /// any string outside the 8 canonical schools (e.g. a `SUBSCHOOL:`
    /// value like `Charm`/`Glamer`/`Polymorph` is never passed here — the
    /// ingester only reads the `SCHOOL:` token, never `SUBSCHOOL:`).
    pub fn from_corpus_str(raw: &str) -> Option<Self> {
        match raw {
            "Abjuration" => Some(Pf1SchoolId::Abjuration),
            "Conjuration" => Some(Pf1SchoolId::Conjuration),
            "Divination" => Some(Pf1SchoolId::Divination),
            "Enchantment" => Some(Pf1SchoolId::Enchantment),
            "Evocation" => Some(Pf1SchoolId::Evocation),
            "Illusion" => Some(Pf1SchoolId::Illusion),
            "Necromancy" => Some(Pf1SchoolId::Necromancy),
            "Transmutation" => Some(Pf1SchoolId::Transmutation),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpellListEntry {
    /// The record's identity in `apg_spells.lst`: its `KEY:` token when
    /// the row carries one, else its display name (most rows carry no
    /// `KEY:`), or — for a `.COPY=` variant record — the variant's own
    /// display name (see this module's doc comment).
    ///
    /// Exactly 9 rows in this file carry a `KEY:` token, all of them the
    /// Summoner-archetype summon spells (`apg_spells.lst:649`+), whose
    /// KEY is archetype-qualified — `KEY:Summoner Summon Monster I` for
    /// the row displayed as `Summon Monster I`. Those are genuinely
    /// different records from CRB's own `Summon Monster I` (they carry
    /// their own `DURATION:` formula keyed to
    /// `ConjurationSummonersCharmBonus`), so storing the display name
    /// here — as this table originally did — collided all 9 with the CRB
    /// record and made a selection carrying that name unresolvable
    /// between the two. The `KEY:` token is stored instead; see
    /// `tests/spell_cross_book_identity.rs`.
    ///
    /// This field is the record's *identity*, not its lookup surface:
    /// those 9 rows are still reachable by their display name through
    /// [`spell_resolve`], which consults [`ARCHETYPE_QUALIFIED_KEYS`].
    /// Do not read a qualified `key` as "this record can only be found
    /// under its archetype name".
    pub key: &'static str,
    /// `None` when the corpus record has no `SCHOOL:` token (a real,
    /// documented gap — see this module's doc comment).
    pub school: Option<Pf1SchoolId>,
    /// Minimum spell level across every `CLASSES:` group on the real
    /// record. `None` when the corpus record has no `CLASSES:` token (a
    /// real, documented gap).
    pub level: Option<u8>,
    /// `None` only when neither the base record nor a matching `.MOD`
    /// record supplies any `DESC:` text at all (a small, real,
    /// documented gap — see this module's doc comment).
    pub description: Option<&'static str>,
    /// `true` when `description` is sourced from the corpus's
    /// `<Name>.MOD` record's own full SRD/PRD `DESC:` text (criterion
    /// 6.5); `false` when it is only the base record's short summary, or
    /// when `description` is `None`.
    pub full_text: bool,
}

pub const SPELL_LIST: &[SpellListEntry] = &[
    SpellListEntry {
        key: "Corruption Resistance",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(2),
        description: Some("You grant the touched creature limited protection from magical effects that inflict damage based on the target creature's alignment, such as holy smite, order's wrath, a paladin's smite evil attack, or an unholy weapon. Choose one alignment type: chaos, evil, good, or law. The subject takes 5 less points of damage from effects that specifically harm creatures of that alignment. The value of the protection increases to 10 points at 7th level and 15 points at 11th level. The spell protects the recipient's equipment as well.  Corruption resistance protects against spells, spell-like abilities, and special abilities, not physical attacks. Corruption resistance only protects against damage. The subject can still suffer side effects from such attacks. When you use this spell to protect an alignment, it gains the descriptor of that alignment."),
        full_text: true,
    },
    SpellListEntry {
        key: "Defile Armor",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(3),
        description: Some("As sanctify armor, except you gain DR 5/good when using your judgment or smite ability."),
        full_text: true,
    },
    SpellListEntry {
        key: "Resounding Blow",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(4),
        description: Some("You must have a melee weapon in hand to cast this spell.  On a successful melee attack, your weapon resounds with a thunderous clash. The target takes 1d6 points of sonic damage. Your weapon is not harmed by this attack.  If you are using your judgment ability or smite ability against the target, it is staggered for 1 round on a successful attack. A successful Fortitude save negates the staggering effect.  On a successful critical hit, the target is stunned for 1 round and deafened for 1d6 rounds. A successful Fortitude save negates the stunning and deafening effects. If you are also using judgment or smite, your foe makes only a single save against all effects.  This spell immediately ends if your weapon leaves your hand.  Resounding blow stacks with the thundering weapon property."),
        full_text: true,
    },
    SpellListEntry {
        key: "Absorbing Touch",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(3),
        description: Some("You absorb the next object your hand touches into your body. If you are already holding an object, you can attempt to absorb it; otherwise, you may touch an object as a standard action separate from casting the spell. You can absorb any nonmagical, nonliving object weighing no more than 1 pound per caster level. A container and its contents count as a single object. Objects in the possession of another creature receive a Fortitude saving throw to prevent absorption. You may continue to attempt to absorb objects until one is absorbed.  An absorbed object melds with your hand and cannot be targeted, but the object's weight still counts against your carrying capacity. If you dismiss the spell, the item appears in the hand that absorbed it, or falls to the ground if that hand is full. Multiple castings let you absorb additional items, but you can only store a single item per hand."),
        full_text: true,
    },
    SpellListEntry {
        key: "Accelerate Poison",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(2),
        description: Some("You hasten the onset of poison in the target. If the poison normally has an onset time, its effects begin immediately. If the poison has no onset time, its frequency is doubled, requiring two saving throws and inflicting damage twice per round or minute, though its duration is halved. Accelerate poison does not change the cure condition for the poison. If the target is affected by more than one poison, you may choose which is affected if you administered the poison; otherwise, randomly determine which poison is affected."),
        full_text: true,
    },
    SpellListEntry {
        key: "Acid Pit",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(4),
        description: Some("This spell functions as create pit, except that it places a 5-footdeep pool of acid at the bottom of the pit. The pit has a maximum depth of 100 feet. Creatures who fall into the pit take falling damage as normal (the acid counts as a yielding surface), plus 2d6 points of acid damage per round spent in contact with the acid. In addition, exposed items carried by a creature in the pit may be harmed. Refer to Table 9-2: Items Affected by Magical Attacks (Core Rulebook 216). Items are affected one at a time in the order listed on the table, and must make Fortitude saves after 3 consecutive rounds in the acid or gain the broken condition. Objects with the broken condition (regardless of how they became broken) must make a Fortitude saving throw each round spent within the acid or be destroyed. The walls of the pit are quite slippery and have a Climb DC of 30."),
        full_text: true,
    },
    SpellListEntry {
        key: "Alchemical Allocation",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(2),
        description: Some("This extract causes a pale aura to emanate from your mouth.  If you consume a potion or elixir on the round following the consumption of this extract, you can spit it back into its container as a free action. You gain all the benefits of the potion or elixir, but it is not consumed. You can only gain the benefits of one potion or elixir in this way per use of this extract."),
        full_text: true,
    },
    SpellListEntry {
        key: "Allfood",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(2),
        description: Some("You can transform one object, weighing up to 5 lbs. per caster level, into an edible substance that any living creature can chew, swallow, and safely digest. This allfood always has a bland taste and slightly gooey consistency when chewed regardless of its original nature.  One pound of allfood provides enough sustenance to support a Medium creature for an entire day.  Objects of similar nature in close proximity, such as a pile of rocks, count as a single object for the purpose of this spell. You cannot use this spell to transform objects with magical or other exceptional qualities. An object's hardness is considered to be 0 for the purposes of eating it, but it retains its normal hardness for all other situations, and becomes no more vulnerable to sunder attacks, break attempts, or any other action typically directed against objects. Objects such as hot coals or a burning torch continue to produce energy even after you transform them into allfood and if a creature attempts to eat such an object, it takes damage accordingly."),
        full_text: true,
    },
    SpellListEntry {
        key: "Alter Winds",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("You subtly enhance or diminish the effects of natural winds within the spell's area, which is an immobile emanation around a point touched by you as the spell is cast. Within the area, natural (but not magical) wind effects are either increased or decreased by one step in intensity (Core Rulebook 439). The maximum wind force you can affect with this spell is based on your caster level, as shown on the table below. Alter winds has no effect on magical wind effects."),
        full_text: true,
    },
    SpellListEntry {
        key: "Amplify Elixir",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(3),
        description: Some("This extract greatly enhances the effects of any potion or elixir you consume. For the duration of this extract, any potion or elixir you consume is treated as if it were empowered. Increase all variable numeric effects of the potion or elixir by half. If the potion or elixir does not have any variable numeric effects, it is instead treated as if it were extended (double the duration of the potion or elixir). If the potion or elixir does not have any variable numeric effects, or has an instantaneous duration, amplify elixir has no effect. Amplify elixir affects oils that target you, but it has no effect on oils that target your equipment."),
        full_text: true,
    },
    SpellListEntry {
        key: "Ant Haul",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("The target's carrying capacity triples (see Table 7-4: Carrying Capacity on page 171 of the Core Rulebook). This does not affect the creature's actual Strength in any way, merely the amount of material it can carry while benefiting from this spell. It also has no effect on encumbrance due to armor. If the creature wears armor it still takes the normal penalties for doing so regardless of how much weight the spell allows it to carry."),
        full_text: true,
    },
    SpellListEntry {
        key: "Aqueous Orb",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(3),
        description: Some("You create a rolling sphere of churning water that can engulf those it strikes. The aqueous orb can move up to 30 feet per round, rolling over barriers less than 10 feet tall. It automatically quenches any nonmagical fires and functions as dispel magic against magical fires as long as those fires are size Large or less.  Any creature in the path of the aqueous orb takes 2d6 points of nonlethal damage. A successful Reflex save negates this damage, but a Large or smaller creature that fails its save must make a second save or be engulfed by the aqueous orb and carried along with it. Engulfed creatures are immersed in water and must hold their breath unless capable of breathing water. They gain cover against attacks from outside the aqueous orb but are considered entangled by its churning currents, takes 2d6 points of nonlethal damage at the beginning of their turn each round they remain trapped. Creatures within the orb may attempt a new Reflex save each round to escape into a random square adjacent to the aqueous orb. The orb may hold one Large creature, 4 Medium, or 16 Small or smaller creatures within it.  The sphere moves as long as you actively direct it (a move action for you); otherwise, it merely stays at rest and churns in place. An aqueous orb stops if it moves outside the spell's range."),
        full_text: true,
    },
    SpellListEntry {
        key: "Arcane Concordance",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(3),
        description: Some("A shimmering, blue and gold radiance surrounds you, enhancing arcane spells cast by your allies within its area. Any arcane spell cast by a creature within the area gains a +1 enhancement bonus to the DC of any saving throws against the spell, and can be cast as if one of the following metamagic feats was applied to it (without increasing the spell level or casting time): Enlarge Spell, Extend Spell, Silent Spell, or Still Spell (you choose the metamagic feat when you cast arcane concordance)."),
        full_text: true,
    },
    SpellListEntry {
        key: "Arrow Eruption",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(2),
        description: Some("You create exact duplicates of the arrow or crossbow bolt you used to kill a creature in the previous round and launch one at enemy creatures within a 30-foot radius of the corpse. You can target one creature per caster level (maximum 15) within range of the burst and must make a single attack roll and apply it to each arrow. These duplicate arrows possess all the intrinsic magical properties of the arrow that killed the original creature as well as those passed on to it by your bow. They also enjoy the full benefit of any bonuses or modifiers you applied to the attack from other magical items, feats, and class or racial features. However, this spell cannot reproduce any spells or other limited-use magical effects that you used to enhance that particular attack. This includes such effects as the true strike spell, as well as any area spell you might have placed on the arrow by means of the arcane archer's imbue arrow class feature."),
        full_text: true,
    },
    SpellListEntry {
        key: "Aspect of the Bear",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(2),
        description: Some("You take on an aspect of a bear. You gain a +2 enhancement bonus to natural armor and a +2 enhancement bonus on CMB rolls. You can also perform bull rush, grapple, and overrun combat maneuvers without provoking attacks of opportunity."),
        full_text: true,
    },
    SpellListEntry {
        key: "Aspect of the Falcon",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("You take on an aspect of a falcon. Your eyes become wide and raptor-like, and you grow feathers on the sides of your head. You gain a +3 competence bonus on Perception checks, a +1 competence bonus on ranged attacks, and the critical multiplier for your bows and crossbows becomes 19-20/x3.  This effect does not stack with any other effect that expands the threat range of a weapon, such as the Improved Critical feat or a keen weapon."),
        full_text: true,
    },
    SpellListEntry {
        key: "Aspect of the Stag",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(3),
        description: Some("When you cast this spell, you take on an aspect of a stag, including some of its physical characteristics. Your features become elongated and sinewy, and you grow a set of antlers you can use for defense. You gain a +2 dodge bonus to AC against attacks of opportunity, your base speed increases by 20 ft., you can move through any undergrowth (including magically manipulated undergrowth) at your normal speed, and can even make a 5-foot step within such terrain.  Furthermore, when you are hit with an attack of opportunity, you can make a single attack with your antlers against the opponent that hit you as an immediate action. This attack uses your highest base attack bonus plus your Strength or Dexterity bonus (your choice) and deals 1d8 points of piercing damage (if you are Medium; 1d6 points of damage if Small) plus your Strength modifier on a successful hit. The antlers have a critical multiplier of 19-20/x2."),
        full_text: true,
    },
    SpellListEntry {
        key: "Aspect of the Wolf",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(4),
        description: Some("When you cast this spell, you take on an aspect of a wolf, including some of its physical characteristics. You become more rugged, your ears become elongated, and you sprout sharp fangs and fur.  You gain a +4 enhancement bonus to Strength and Dexterity, the scent ability, a +2 enhancement bonus on trip attacks, and can make a trip combat maneuver as a swift action. This trip attack does not provoke attacks of opportunity."),
        full_text: true,
    },
    SpellListEntry {
        key: "Aura of Greater Courage",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(2),
        description: Some("When you cast this spell you strengthen your paladin's aura of courage. Until the end of its duration, all allies within that aura are immune to fear (magical or otherwise). If you do not have the aura of courage class feature, aura of greater courage has no effect."),
        full_text: true,
    },
    SpellListEntry {
        key: "Ball Lightning",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(4),
        description: Some("You create two globes of lightning that fly in whichever direction you indicate. For every 4 caster levels above 7th, you create an additional globe of lightning (3 globes at 11th, 4 globes at 15th, to the maximum of 5 globes at 19th). These globes fly at a rate of 20 feet per round and have perfect maneuverability. Wind does not affect a flying sphere's course.  If a globe enters a space with a creature, it stops moving for the round and deals 3d6 points of electricity damage to that creature, though a successful Reflex save negates the damage. Creatures wearing metal armor take a -4 penalty on this saving throw.  Each globe moves as long as you actively direct it (it's a move action for you to direct all the spheres created by a single casting of this spell); otherwise they stay at rest. These globes have no mass and cannot push aside unwilling creatures or move solid objects. A ball lightning globe winks out if it exceeds the spell's range."),
        full_text: true,
    },
    SpellListEntry {
        key: "Banish Seeming",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(3),
        description: Some("With a melee touch attack you can dispel an illusion or return a creature to its natural form. This functions as dispel magic directed at the effect in question, except you receive a +2 enhancement bonus on your dispel check and you can only dispel illusions or changes in form created by supernatural effects or spells. If multiple effects are changing the creature's appearance, you can dispel one such effect for every four caster levels you possess, starting with the highest caster level spells and proceeding to spells with lower caster levels.  The caster level for supernatural abilities such as change shape is equal to the target creature's Hit Dice. A creature returned to its natural form by banish seeming is prevented from changing its form again for a number of rounds equal to your caster level.  Banish seeming has no effect on nonmagical means of changing appearance, such as that provided by a disguise or extraordinary ability."),
        full_text: true,
    },
    SpellListEntry {
        key: "Bard's Escape",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(5),
        description: Some("You whisk yourself and willing allies out of a tight jam, or instantly transfer yourselves to another location to achieve greater strategic positioning. You can move any allies within range to any other space of your choosing within range. Those spaces need not be in line of sight or line of effect from your original position, but must be open locations on surfaces able to support the creatures teleported. Except as noted above, this spell otherwise functions as dimension door."),
        full_text: true,
    },
    SpellListEntry {
        key: "Beguiling Gift",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(1),
        description: Some("You offer an object to an adjacent creature, and entice it into using or consuming the proffered item. If the target fails its Will save, it immediately takes the offered object, dropping an already held object if necessary. On its next turn, it consumes or dons the object, as appropriate for the item in question. For example, an apple would be eaten, a potion consumed, a ring put on a finger, and a sword wielded in a free hand. If the target is physically unable to accept the object, the spell fails. The subject is under no obligation to continue consuming or using the item once the spell's duration has expired, although it may find a cursed item difficult to be rid of."),
        full_text: true,
    },
    SpellListEntry {
        key: "Bestow Grace",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(2),
        description: Some("With this spell you can bestow your divine grace on another good creature for a short amount of time, infusing that creature with a portion of your holy virtue. When you touch the subject, you grant that creature a sacred bonus to its saving throws equal to its Charisma bonus (if any) on all saving throws."),
        full_text: true,
    },
    SpellListEntry {
        key: "Blaze of Glory",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(4),
        description: Some("You fall unconscious but also unleash a spectacular wave of holy energy that heartens and heals your allies while leaving your opponents daunted and damaged. You can cast this spell as either a standard action on your turn, or as an immediate action when brought to below 0 hit points. If cast as a standard action, you are immediately reduced to -1 hit points, but stable, after casting the spell.  When you cast this spell, any good creature within range is healed for 1d6 points of damage per two caster levels. All evil creatures within range take the same amount as damage instead (a successful Will save halves this damage). In addition, all allies and enemies within range are affected as if by the prayer spell for 1 round per caster level."),
        full_text: true,
    },
    SpellListEntry {
        key: "Blessing of Courage and Life",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(2),
        description: Some("With this prayer you provide long-lasting succor to a wounded creature. For as long as the effect lasts, the target receives a +2 morale bonus on saving throws against fear and death effects.  At any time while the spell is in effect, the target can choose to end the spell as a swift action for a burst of healing energy. The target loses the saving throw bonus, but is healed of 1d8 points of damage +1 point per caster level (maximum +10)."),
        full_text: true,
    },
    SpellListEntry {
        key: "Blessing of Fervor",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(4),
        description: Some("With this blessing, you call your allies to move forth and empower them to conquer and become victorious. Each round for the duration of this spell, each of your allies can choose one of the following bonuses for that round at the beginning of its turn (their choice): Increase its speed by 30 feet, Stand up as a swift action without provoking an attack of opportunity, Make one extra attack as part of a full attack action, using its highest base attack bonus, Gain a +2 bonus on attack rolls and a +2 dodge bonus to AC and Reflex saves, Cast a single spell of 2nd level or lower as if it were an enlarged, extended, silent, or still spell.  These effects are not cumulative with similar effects, such as those provided by haste or a speed weapon, nor do they actually grant an extra action, so you can't use it to cast a second spell or otherwise take an extra action in the round. Blessing of fervor does not stack with haste."),
        full_text: true,
    },
    SpellListEntry {
        key: "Blessing of the Salamander",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(4),
        description: Some("When you cast this on a creature, its skin turns slick and clammy and it is able to regenerate damage each round. While under the effects of the spell, the creature gains fast healing 5, fire resistance 20 and a +2 competence bonus to its Combat Maneuver Defense."),
        full_text: true,
    },
    SpellListEntry {
        key: "Blood Biography",
        school: Some(Pf1SchoolId::Divination),
        level: Some(2),
        description: Some("You learn the answers to a specific set of questions about a creature so long as you have access to at least one drop of its blood. You can cast this spell on the blood of the living or the dead, but living or undead creatures are entitled to a saving throw to resist the spell. You can cast the spell on dried or fresh blood. Once you cast the spell the answers to the following four questions appear on any flat surface you designate (a wall, a piece of paper, and so on): Who are you? (The name by which the creature is most commonly known), What are you? (Gender, race, profession/role), How was your blood shed? (Brief outline of the events that caused its wound, to the best of the victim's knowledge), When was your blood shed?  These answers always appear in a language you can read even if the creature cannot speak that or any language."),
        full_text: true,
    },
    SpellListEntry {
        key: "Bloodhound",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(2),
        description: Some("You gain the scent special quality, including the ability to track by scent. You receive a +8 competence bonus on Perception checks involving smell and a +4 competence bonus on Survival checks to track using scent. You take a -4 penalty on saving throws against odor-related effects such as the stench ability and stinking cloud. A creature under the effects of bloodhound can detect poison by scent with a DC 20 Perception check."),
        full_text: true,
    },
    SpellListEntry {
        key: "Bloody Claws",
        school: Some(Pf1SchoolId::Necromancy),
        level: Some(3),
        description: Some("You give a creature the ability to deal bleed damage when making natural attacks so long as the attack deals slashing or piercing damage. This bleed damage for each attack is equal to one-half your caster level (limited to the creature's maximum damage with that attack), though bleed damage does not stack. When two or more attacks deal bleed damage, take the worse effect."),
        full_text: true,
    },
    SpellListEntry {
        key: "Bomber's Eye",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("This extract allows you to throw weapons farther and more accurately. While this extract is in effect, increase the range of any thrown weapon by 10 feet. In addition, you receive a +1 insight bonus on attack rolls made with thrown weapons."),
        full_text: true,
    },
    SpellListEntry {
        key: "Borrow Skill",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("You borrow one of the target's known skills. After touching the subject, you can make a single check with the chosen skill using the subject's skill ranks, but modified by your own key ability.  If you take longer than the spell's duration to make the check or the check requires more time, then it fails and you must use your own skill ranks (if any). If the borrowed skill is a class skill for you, you gain the +3 bonus on checks using that skill."),
        full_text: true,
    },
    SpellListEntry {
        key: "Bow Spirit",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(4),
        description: Some("A bow spirit is a shapeless force that hovers about you, taking ammunition from your quiver and firing it. For as long as the bow spirit lasts, you can spend a swift action to direct the bow spirit to fire an arrow or a bolt at a target of your choice, as if the bow spirit were firing the necessary ranged weapon. The bow spirit uses your base attack bonus plus your Dexterity modifier, as well as any bonuses and effects from feats you have that affect ranged attacks, or bonuses from the ammunition it uses.  A bow spirit's attacks do not provoke attacks of opportunity.  There must be ammunition available for the bow spirit to use, and it expends ammunition as if used by you.  A bow spirit occupies your space, and moves with you.  It cannot be attacked or harmed by physical attacks, but dispel magic, disintegrate, a sphere of annihilation, or a rod of cancellation affects it. A bow spirit's AC against touch attacks is 10 + your Dexterity modifier."),
        full_text: true,
    },
    SpellListEntry {
        key: "Brand",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(0),
        description: Some("Brand etches an indelible rune or mark of no more than 6 characters onto a creature, inflicting 1 point of damage. The mark may be placed on any exposed portion of the creature, typically the head or forearm. A brand can be hidden by clothing or removed by scraping it away (causing 1d6 points of damage), though the brand returns if the damage is healed."),
        full_text: true,
    },
    SpellListEntry {
        key: "Brand (Greater)",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(4),
        description: Some("This spell functions like brand, except that it inflicts 1d6 points of damage when applied and cannot be removed, even temporarily. In addition, a greater brand blazes as bright as a torch when brought within 30 feet of a visible symbol of your faith. While the symbol is blazing, the recipient is sickened.  Like the effects of bestow curse, a greater brand cannot be dispelled, but it can be removed by any means that removes a mark of justice."),
        full_text: true,
    },
    SpellListEntry {
        key: "Break",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("You can attempt to break or at least damage any one Medium or smaller object within range. If the target fails its Fortitude saving throw, it gains the broken condition. If cast on a broken item, that item is destroyed on a failed save."),
        full_text: true,
    },
    SpellListEntry {
        key: "Brilliant Inspiration",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(6),
        description: Some("You open a link between your mind and the subject's mind, giving advice and encouragement for as long as the spell is in effect. Each time the subject of the spell makes an attack roll, ability check, or skill check, it rolls two d20s and takes the better result. If any roll is a natural 20, the spell's effect ends-your brilliant advice is spent."),
        full_text: true,
    },
    SpellListEntry {
        key: "Bristle",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("You give a creature the ability to redirect a portion of its innate toughness away from its own defense and toward the amount of damage it deals with natural attacks. Each round, as a swift action at the start of its turn, the creature can choose to reduce some or all of its natural armor bonus to AC and gain an enhancement bonus on all damage rolls for natural attacks equal to that amount. The reduction to natural armor, and thus the enhancement bonus on damage rolls, cannot exceed 1 point per 3 caster levels, to a maximum penalty/bonus of -5/+5 at 15th level. A creature cannot reduce its natural armor bonus to less than 0 with this spell.  All attacks directed against the creature use its adjusted AC until the start of its next turn, at which time it can choose to modify its AC again or keep it at its current level. Creatures make this decision without any need for conscious thought or reflection; even creatures with no Intelligence score can benefit from this spell, although they always opt for the maximum possible reduction and bonus, regardless of any tactical advantage they might lose."),
        full_text: true,
    },
    SpellListEntry {
        key: "Burning Gaze",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(2),
        description: Some("Your eyes burn like hot coals, allowing you to set objects or foes alight with a glance.  As a standard action as long as this spell's effects persist, you may direct your burning gaze against a single creature or object within 30 feet of your location. Targeted creatures must succeed at a Fortitude save or take 1d6 points of fire damage. Unattended objects do not get a save. Creatures damaged by the spell must make a Reflex save or catch fire.  Each round, burning creatures may attempt a Reflex save to quench the flames; failure results in another 1d6 points of fire damage. Flammable items worn by a creature must also save or take the same damage as the creature. If a creature or object is already on fire, it suffers no additional effects from burning gaze.  Note that this spell does not grant an actual gaze attack- foes and allies are not in danger of catching on fire simply by meeting your gaze."),
        full_text: true,
    },
    SpellListEntry {
        key: "Burst Bonds",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(1),
        description: Some("You blast your restraints with divine wrath, dealing 1d6 points of damage per caster level to the target object (maximum 5d6), ignoring hardness of 10 or less. A Fortitude save halves the damage, although only magical restraints receive a saving throw.  You can also target a creature that is grappling or otherwise restraining you (such as by adhesive or the swallow whole ability) with the spell. You don't need to make a touch attack to use burst bonds against a creature, and you can cast this spell without making a concentration check while grappled or pinned. The creature does not take damage as above. If you are grappled and your target fails its save, you may make an immediate attempt to break the grapple (but not reverse it) as a free action, with a +1 luck bonus to your CMB per caster level (maximum +5). If the target creature fails its save and you are restrained by an ability that originally allowed a saving throw, burst bonds allows a new saving throw with a +1 luck bonus per two caster levels (maximum +5)."),
        full_text: true,
    },
    SpellListEntry {
        key: "Cacophonous Call",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(2),
        description: Some("You fill your target's mind with a blaring cacophony of discordant sounds, making it hard for the target to act and concentrate.  The creature gains the nauseated condition for the duration of the spell if it fails its Will save."),
        full_text: true,
    },
    SpellListEntry {
        key: "Cacophonous Call (Mass)",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(5),
        description: Some("This spell functions as cacophonous call, except that it affects multiple creatures."),
        full_text: true,
    },
    SpellListEntry {
        key: "Calcific Touch",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(4),
        description: Some("Your touch progressively transmutes the substance of creatures you touch into stone. Once per round, you may deliver a touch attack that inflicts 1d4 points of Dexterity damage and slows the target (as the spell) for 1 round. A successful Fortitude save negates the slow effect but not the ability damage. A target reduced to 0 Dexterity is petrified permanently. Break enchantment, restoration, or stone to flesh can reverse the effects of calcific touch."),
        full_text: true,
    },
    SpellListEntry {
        key: "Call Animal",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(1),
        description: Some("This spell calls the nearest wild animal of a particular type you designate (provided the animal's CR is equal to or less than your caster level) to seek you out. The animal moves toward you under its own power, so the time it takes to arrive depends on how close an animal of the desired type is when you cast the spell. If there is no animal of that type capable of reaching you within the spell's duration, you are aware of this fact, but the spell is wasted. Knowledge of the local fauna makes this spell more effective, and the GM may permit a Knowledge (nature) skill check (DC 15) to know what animals can be found in an area.  When the called animal arrives, it approaches to within 5 feet of you and remains nearby for the duration of the spell.  Its starting attitude is indifferent, modified by circumstances and interaction. Other than starting attitude, this spell gives you no special influence or ability to communicate with the called animal, although you may use other spells or abilities to do so.  Once the spell's duration expires, the animal acts in accordance with its attitude. Most animals will likely wander off, but a hostile predatory animal may attack, especially if it is hungry or provoked.  Domesticated animals or animals trained by someone else, including such creatures as familiars or animal companions, are unaffected by call animals."),
        full_text: true,
    },
    SpellListEntry {
        key: "Campfire Wall",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(2),
        description: Some("You can create a barrier around a fire of at least campfire size that shelters everyone inside so long as the fire continues to burn. The barrier appears as a crackling sphere of light and fire that is clearly visible, providing as much illumination as a torch.  The barrier bocks line of sight, granting creatures on either side of the barrier total concealment from creatures on the other side. Any object or creature passing through the barrier from outside takes 1d6 points of fire damage and is also outlined with light equivalent to that of a torch, for 1d6 minutes. Creatures outlined in this way are plainly visible regardless of the light conditions and do not benefit from any sort of concealment, magical or otherwise. The light is not bright enough to have any special effect on undead or creatures vulnerable to light.  Creatures inside the barrier can leave without penalty, but if they try to return they suffer the same consequences as anyone else. If the fire source at the barrier's center is extinguished or moved, the spell ends."),
        full_text: true,
    },
    SpellListEntry {
        key: "Cast Out",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(3),
        description: Some("With a melee touch attack you blast a single creature and disrupt any magic controlling it. The target takes 2d8 points of damage + 1 point per caster level (maximum +15). In addition, you make a dispel check against any magic jar effect (including a ghost's malevolence ability) or enchantment (compulsion) spells on the target creature, starting with the highest level spells and proceeding to lower level spells. Treat this as a targeted dispel magic spell, except cast out can dispel one such spell for every four caster levels you possess. A successful Will save halves the damage and limits the dispelling to a single spell or effect."),
        full_text: true,
    },
    SpellListEntry {
        key: "Castigate",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(2),
        description: Some("You compel the target to beg for forgiveness. On a failed save, the target cowers with fear. On a successful save, it is shaken for 1 round. Each round on its turn, a cowering subject may attempt a new save to end the effect. A creature who worships the same god as you takes a -2 penalty on its saving throw."),
        full_text: true,
    },
    SpellListEntry {
        key: "Castigate (Mass)",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(5),
        description: Some("This spell functions as castigate, except it affects many creatures."),
        full_text: true,
    },
    SpellListEntry {
        key: "Challenge Evil",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(1),
        description: Some("You challenge an evil creature to bring the fight to you, or suffer the consequences. You gain a +2 sacred bonus on all melee attacks against the subject of the spell. At the end of its turn, if the target has not made at least one attack on you, it becomes sickened. If you move away from the target, the spell ends."),
        full_text: true,
    },
    SpellListEntry {
        key: "Chameleon Stride",
        school: Some(Pf1SchoolId::Illusion),
        level: Some(2),
        description: Some("You fade into the background, and while you are not truly invisible, you are hard to pinpoint due to your translucent state.  While under the effects of this spell, you gain a +4 bonus on Stealth checks and have concealment from creatures more than 5 feet away (attacks have a 20%% miss chance)."),
        full_text: true,
    },
    SpellListEntry {
        key: "Clashing Rocks",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(9),
        description: Some("You create two Colossal-sized masses of rock, dirt, and stone and slam them together against a single creature between them. The clashing rocks appear up to 30 feet away from the target on opposite sides and rush toward it with a mighty grinding crash. You must make a ranged touch attack to hit the target with the rocks. The clashing rocks ignore concealment and cover, and if there is a solid barrier between the target and either of the clashing rocks, the spell has a +28 bonus on the Strength check to burst through the barrier and continue unimpeded to the target. A creature struck by the clashing rocks takes 20d6 points of bludgeoning damage and is knocked prone. If the target fails a Reflex saving throw, it is also buried under the resulting rubble as if by a cave-in (see Pathfinder RPG Core Rulebook 415).  If the clashing rocks miss the target, the target still takes 10d6 points of bludgeoning damage from falling rocks and is knocked prone. A successful Reflex save reduces this damage to half and the target remains standing. Creatures other than the target that occupy the spaces where the clashing rocks appear or within their path (30 feet wide, 30 feet high, and up to 60 feet long) must also make Reflex saves or take 10d6 points of bludgeoning damage and be knocked prone (save for half and remain standing). A creature can only take damage once from the clashing rocks, no matter how many times the clashing rocks pass over a target creature."),
        full_text: true,
    },
    SpellListEntry {
        key: "Cleanse",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(5),
        description: Some("Positive energy infuses and cleanses your body. This spell cures 4d8 points of damage + 1 point per caster level (maximum +25) and ends any and all of the following adverse conditions affecting you: ability damage, blinded, confused, dazzled, deafened, diseased, exhausted, fatigued, nauseated, poisoned, and sickened. In addition, cleanse functions as break enchantment upon a single additional effect of your choice that is affecting you and that can be legally affected by this effect.  If used by undead or other creatures healed by negative energy, the spell cleanses with negative energy rather than positive."),
        full_text: true,
    },
    SpellListEntry {
        key: "Cloak of Dreams",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(5),
        description: Some("You are surrounded by a soporific aroma that causes living creatures that begin their turn or end their movement within 5 feet of you to fall asleep for 1 minute. Creatures must save each time they begin their turn or end their movement within the cloak of dreams, even if they have previously saved against the effect. Sleeping creatures are helpless but can be awakened with a standard action or after being wounded.  Creatures with the scent special quality have a -4 penalty on their saves."),
        full_text: true,
    },
    SpellListEntry {
        key: "Cloak of Shade",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(1),
        description: Some("This spell provides the subject with some degree of protection from the harmful effects of the sun. The cloaked subject treats environmental heat due to sun exposure as one level less: severe heat is considered very hot conditions, while very hot is considered average temperature (see page 444 of the Core Rulebook for heat dangers). The cloak of shade also reduces any penalties from sunlight by 1. The spell does not, however, eliminate the effects of direct sunlight on creatures vulnerable to sunlight. Cloak of shade has no effect on environmental heat from sources other than the sun."),
        full_text: true,
    },
    SpellListEntry {
        key: "Cloak of Winds",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(3),
        description: Some("You shroud a creature in a whirling screen of strong, howling wind. The subject is never checked or blown away by strong winds of windstorm or lesser strength (whether natural or magically created), and ranged attack rolls against the subject take a -4 penalty. Tiny or smaller creatures must succeed at a Fortitude save to successfully touch or attack the subject in melee. Failure knocks the attacker prone and pushes it 5 feet away from the subject per level of the caster. This movement can pass through the squares of other creatures without affecting them and does not provoke attacks of opportunity, but the creature takes 3d6 points of nonlethal damage, plus 1d6 if the creature strikes a solid object that blocks its movement."),
        full_text: true,
    },
    SpellListEntry {
        key: "Confess",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(2),
        description: Some("You ask the target creature a single question. On the subject's next action, it must answer truthfully in the same language as the question or take 1d6 points of damage per two caster levels (maximum 5d6) and be sickened for 2d4 rounds. A successful Will save negates the sickening effect and halves the damage. A creature that is unable to answer still takes damage."),
        full_text: true,
    },
    SpellListEntry {
        key: "Contagious Flame",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(6),
        description: Some("You blast several enemies with beams of fire. You may fire three rays, plus one additional ray for every four levels beyond 11th (to a maximum of five rays at 19th level). Each ray requires a ranged touch attack to hit and deals 4d6 points of fire damage.  The rays may be fired at the same or different targets, but all rays must be aimed at targets within 30 feet of each other and fired simultaneously.  Every round on your turn, a new ray of fire launches from each creature who took damage from the spell in the previous round-these new rays attack as if you fired them, but their point of origin is the previous creature damaged. You can choose the new targets as a free action, all of whom must be within close range (25 ft. + 5 ft./2 levels) of their new starting point. This contagion of flame continues for a total of three rounds-a creature can be struck more than once by this spell over the course of these three rounds, although never by a ray of fire that launches from itself."),
        full_text: true,
    },
    SpellListEntry {
        key: "Coordinated Effort",
        school: Some(Pf1SchoolId::Divination),
        level: Some(3),
        description: Some("Choose a single teamwork feat that you possess. You forge a link with your allies, allowing them to gain the benefits of the chosen feat even if they do not have it themselves. You must be a part of the group qualifying for a given bonus for your allies to gain the benefits. For example, two of your allies flanking an orc would gain no benefit from your Outflank feat, but an ally flanking the orc with you would. Your allies' positioning and actions must still meet the prerequisites listed in the teamwork feat for either of you to receive the listed bonus."),
        full_text: true,
    },
    SpellListEntry {
        key: "Coward's Lament",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(4),
        description: Some("You compel an opponent to face you in combat, or suffer for its cowardice. Each round that the target fails to attack you in melee, it receives a cumulative -1 penalty to its Armor Class, attack rolls, and saving throws (maximum penalty -5). Each round at the end of its turn, the target may attempt a Will saving throw to prevent the penalties from increasing for that round. All penalties reset to zero when the target attacks you in melee, but increase again if it stops attacking. If the target is prevented from attacking you by physical restraint, magic, or impassable terrain, the penalties do not increase. If you move away from the target, the spell ends."),
        full_text: true,
    },
    SpellListEntry {
        key: "Crafter's Curse",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("The target of crafter's curse takes a -5 penalty on all Craft skill checks while the spell lasts."),
        full_text: true,
    },
    SpellListEntry {
        key: "Crafter's Fortune",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("The target is struck by inspiration and gains a +5 luck bonus on its next Craft skill check."),
        full_text: true,
    },
    SpellListEntry {
        key: "Create Pit",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(2),
        description: Some("You create a 10-foot-by-10-foot extradimensional hole with a depth of 10 feet per two caster levels (maximum 30 feet).  You must create the pit on a horizontal surface of sufficient size. Since it extends into another dimension, the pit has no weight and does not otherwise displace the original underlying material. You can create the pit in the deck of a ship as easily as in a dungeon floor or the ground of a forest. Any creature standing in the area where you first conjured the pit must make a Reflex saving throw to jump to safety in the nearest open space. In addition, the edges of the pit are sloped, and any creature ending its turn on a square adjacent to the pit must make a Reflex saving throw with a +2 bonus to avoid falling into it. Creatures subjected to an effect intended to push them into the pit (such as bull rush) do not get a saving throw to avoid falling in if they are affected by the pushing effect.  Creatures who fall into the pit take falling damage as normal.  The pit's coarse stone walls have a Climb DC of 25. When the duration of the spell ends, creatures within the hole rise up with the bottom of the pit until they are standing on the surface over the course of a single round."),
        full_text: true,
    },
    SpellListEntry {
        key: "Create Treasure Map",
        school: Some(Pf1SchoolId::Divination),
        level: Some(2),
        description: Some("You can take a piece from a dead creature's body and use it to create a map that reveals the locations of any valuables that creature knew about while still alive. You must cut this piece off yourself and the creature cannot have died more than 24 hours prior to the casting of this spell. You cannot cast this spell on a creature without a physical body, and the portion you remove must have a fairly level surface such as a piece of hide, carapace, or skin. The map you create is accurate to the best knowledge the creature had at the time of its death. It reveals the location of one source of treasure for every three caster levels you possess.  Any inaccuracies or faulty information held by the creature are represented on the map as well. Similarly, the map cannot account for any changes that occur after the creature's death.  You must choose the scale of the map when creating it, opting between nearby (e.g., one or two dungeon levels), local (e.g., a valley or community), or broad (e.g., a country or larger). The map reveals the locations of what the creature deemed most valuable within its area. Depending on the creature, the map might reveal a source of tasty food, suitable mates, or even your own treasure vault."),
        full_text: true,
    },
    SpellListEntry {
        key: "Cup of Dust",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(3),
        description: Some("You curse the target with a thirst no drink can quench. On a failed saving throw, the target begins to dehydrate (see page 444 of the Core Rulebook for starvation and thirst). The effects of this spell cannot inflict more nonlethal damage than the target has hit points. No amount of drinking can counter this effect, although the target still experiences the other normal effects of any liquid imbibed (such as poisons or potions). Cup of dust can be removed with a break enchantment, limited wish, miracle, remove curse, or wish spell, but effects suffered as a result of dehydration must be healed by normal means."),
        full_text: true,
    },
    SpellListEntry {
        key: "Dancing Lantern",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("You can animate a lantern and order it to follow you. The lantern floats at shoulder height and remains within 5 feet of you, no matter how fast you move. The lantern cannot support any additional weight. The lantern illuminates its normal area, even if it does not have any oil in it. For the purposes of spells or effects targeting it the lantern always acts as if in your possession even when not directly on your person. A dancing lantern can be made permanent with a permanency spell (CL 9th, 2,500 gp)."),
        full_text: true,
    },
    SpellListEntry {
        key: "Deadly Finale",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(6),
        description: Some("You must have a bardic performance in effect to cast this spell.  With a flourish, you immediately end your performance, dealing 2d8 points of sonic damage to each target. In addition, each target takes 3d6 points of bleed damage for 1d6 rounds. A save negates the bleed damage but not the sonic damage."),
        full_text: true,
    },
    SpellListEntry {
        key: "Deafening Song Bolt",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(5),
        description: Some("Three notes you sing or perform become tangible bolts of arcane energy that shriek across the battlefield. Each bolt requires a ranged touch attack to hit and deals 3d10 points of sonic damage, deafening the target for 1d6 rounds. The bolts may be fired at the same or different targets, but all must be fired simultaneously."),
        full_text: true,
    },
    SpellListEntry {
        key: "Deflection",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(7),
        description: Some("You surround yourself in a whirling barrier of force that sends any attack that misses you hurling back toward its source.  This applies to any melee or ranged attack directed against you so long as it uses an attack roll to determine whether or not it strikes you. If an attack misses you, the attacker must make a second attack roll against its own Armor Class, using all of the applicable modifiers of the original attack and if it hits, the attacker takes the attack's damage and suffers all the other consequences of getting struck with that attack. You cannot deflect attacks that miss you for any reason besides a failed attack roll (such as concealment). Similarly, you cannot deflect attacks that actually do strike you but simply fail to do any harm."),
        full_text: true,
    },
    SpellListEntry {
        key: "Delayed Consumption",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(5),
        description: Some("When you consume this extract, you quickly consume another extract of your choice-this second extract's effects do not come into effect until a later point. You must consume this second, companion extract on the round following delayed consumption or waste the extract. The companion extract can be no higher than 4th level, and you must pay any costs associated with the companion extract when you consume it. At any point during the duration of this extract, you can cause the companion extract to take effect as an immediate action. You can only have one delayed consumption in effect at one time. If a second is consumed, the first is dispelled without any effect."),
        full_text: true,
    },
    SpellListEntry {
        key: "Denounce",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(4),
        description: Some("You speak out against a single creature within line of sight and turn possible allies against it by drawing upon the power of your voice and conviction. Creatures in the area who can see the denounced creature must make a Will saving throw or have their starting attitude toward the denounced creature worsened by two levels (see page 94 of the Core Rulebook). For example, creatures previously indifferent to the subject turn unfriendly.  Diplomacy checks made to reverse the effects of denounce are made at a -10 penalty."),
        full_text: true,
    },
    SpellListEntry {
        key: "Detect Aberration",
        school: Some(Pf1SchoolId::Divination),
        level: Some(1),
        description: Some("This spell functions like detect animals or plants, except it detects creatures of the aberration type."),
        full_text: true,
    },
    SpellListEntry {
        key: "Detonate",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(4),
        description: Some("You flood yourself with a potent surge of elemental energy.  One round after completing the casting of the spell, the energy explodes from your body.  When this spell creates the explosion of energy, choose one of the following four energy types: acid, cold, electricity, or fire. The explosion inflicts 1d8 points of damage of that energy type per caster level (maximum 10d8) to all creatures and unattended objects within 15 feet, and half that amount to targets past 15 feet but within 30 feet. You automatically take half damage from the explosion, without a saving throw, but any other energy resistance or energy immunity effects you may have in place can prevent or lessen this overflow damage caused by the explosion."),
        full_text: true,
    },
    SpellListEntry {
        key: "Devolution",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(3),
        description: Some("This spell causes an eidolon to lose one evolution plus one additional evolution for every five caster levels. Evolutions with the highest total cost are lost first. If there is a tie, randomly determine which is lost. If this spell causes an eidolon to fail to meet the prerequisites for other evolutions or abilities, those evolutions or abilities are lost as long as this spell persists."),
        full_text: true,
    },
    SpellListEntry {
        key: "Discordant Blast",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(4),
        description: Some("You create a wave of thunder and force, either in a 10-footradius burst centered on you or in 30-foot cone-shaped burst.  Creatures in the area take 3d6 points of sonic damage and are pushed away as if bull rushed. Make a combat maneuver check and apply its results to each creature in the area. Your CMB for this bull rush is equal to your caster level plus your Charisma modifier. This bull rush does not provoke an attack of opportunity. A discordant blast cannot penetrate a silence spell (or any similar magical silence effect)."),
        full_text: true,
    },
    SpellListEntry {
        key: "Divine Transfer",
        school: Some(Pf1SchoolId::Necromancy),
        level: Some(3),
        description: Some("With a single touch, you transfer some of your life essence to the subject, transferring your hit points and your resolve. When you touch the subject you can transfer up to a number of hit points equal to your Constitution score to the target. These hit points heal the subject, but cannot raise the subject's hit points higher than its normal hit point total. In addition, the subject gains DR/evil equal to your Charisma bonus (if any) for the duration of the spell."),
        full_text: true,
    },
    SpellListEntry {
        key: "Draconic Reservoir",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(3),
        description: Some("Draconic reservoir functions as protection from energy, absorbing 6 points of one type of energy damage per caster level (acid, cold, electricity, or fire, maximum 60 points). Each round, as a swift action, the subject can release 1d6 points of the absorbed energy and apply it to any melee attack, as if using an acidic, flaming, frost, or shock weapon. The first creature the subject strikes with this attack takes the energy damage in addition to any other consequences of the attack.  Releasing energy in this way does not \"free up\" space to absorb still more energy; the maximum amount of energy the spell can absorb remains fixed. The subject cannot release more energy than he currently has absorbed. Once the subject has absorbed all the energy allowed by the spell, he takes damage as normal from that energy type. Once the energy has been released, the spell is discharged.  Draconic reservoir does not stack with protection from energy.  Draconic reservoir overlaps (and does not stack with) resist energy. If a character is warded by draconic reservoir and resist energy, draconic reservoir absorbs damage until it reaches its maximum limit."),
        full_text: true,
    },
    SpellListEntry {
        key: "Dragon's Breath",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(4),
        description: Some("You breathe out a blast of energy. Creatures in the affected area take 1d6 points of energy damage per caster level (maximum of 12d6). A successful Reflex save results in half damage. The spell's effect and energy type depend on the type of dragon scale used: Black dragon: 60-foot line of acid.  Blue or bronze dragon: 60-foot line of electricity.  Green dragon: 30-foot cone of acid.  Gold or red dragon: 30-foot cone of fire.  Silver or white dragon: 30-foot cone of cold.  Brass dragon: 60-foot line of fire.  Copper dragon: 60-foot line of acid."),
        full_text: true,
    },
    SpellListEntry {
        key: "Dust of Twilight",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(2),
        description: Some("A shower of iridescent black particles clings to and extinguishes torches, lanterns, sunrods, and similar mundane light sources and dispels any spell of 2nd level or lower with the light descriptor (as dispel magic). Creatures in the area must make a Fortitude save or become fatigued."),
        full_text: true,
    },
    SpellListEntry {
        key: "Eagle Eye",
        school: Some(Pf1SchoolId::Divination),
        level: Some(2),
        description: Some("Eagle eye creates a magical sensor directly above you. The sensor can appear anywhere above you, to a maximum height equal to the spell's range. You can see from this vantage as if you were actually there, rotating your viewpoint 360 degrees.  You perceive with your normal visual senses. A caster using eagle eye can easily see for a considerable distance. The spell does not penetrate any solid surface, although it is unaffected by foliage and the like."),
        full_text: true,
    },
    SpellListEntry {
        key: "Elemental Aura",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(3),
        description: Some("This spell forms an aura of energy around you, damaging all those that come near you. Choose an energy type: acid, cold, electricity, or fire. Creatures adjacent to you when this spell is cast and at the start of your turn take 2d6 points of energy damage of the selected type. This aura has an additional effect, depending upon the type of energy chosen.  Acid: Creatures affected by your aura take 1 point of ongoing acid damage per round for 1 round per three caster levels, and are sickened for the duration of the ongoing acid damage.  Cold: Creatures affected by your aura are fatigued. A creature that is already fatigued suffers no additional effect.  Electricity: Creatures affected by your aura are staggered for 1 round.  Fire: Creatures affected by your aura catch on fire (Core Rulebook 444).  Creatures adjacent to you are allowed a Reflex save to halve the damage and negate the additional effect. The aura's additional effects do not stack if a creature takes damage from your aura multiple times. You may only have one elemental aura in effect at one time. When you cast this spell to deal acid, cold, electricity, or fire damage, it is a spell of that type."),
        full_text: true,
    },
    SpellListEntry {
        key: "Elemental Speech",
        school: Some(Pf1SchoolId::Divination),
        level: Some(2),
        description: Some("This spell enables you to converse with creatures associated with a chosen element, including but not limited to true elemental creatures. This spell gains the elemental subtype based on the version of the spell you cast. Elemental speech does not guarantee a friendly reaction; it merely enables communication. You may converse with all creatures of the selected type with an Intelligence score of 1 or greater, even if they do not understand one another.  When cast as an air spell, you can converse in Auran and with any creature that has the air subtype or a fly speed.  When cast as an earth spell, you can converse in Terran and with any creature that has the earth subtype or a burrow speed.  When cast as a fire spell, you can converse in Ignan and with any creature that has the fire subtype.  When cast as a water spell, you can converse in Aquan and with any creature that has the water subtype or a swim speed."),
        full_text: true,
    },
    SpellListEntry {
        key: "Elemental Touch",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(2),
        description: Some("Upon completing the casting of this spell, elemental energy infuses your hands. Choose an energy type: acid, cold, electricity, or fire. You gain a melee touch attack causing 1d6 points of damage of that energy type, along with a special effect described below. You also deal energy damage and the related special effect when you attack with your hands using an unarmed strike, a single claw, or a single slam attack. This bonus damage can never apply to multiple weapons.  Acid: Your touch attack causes 1 point of ongoing acid damage per round for 1 round per three caster levels. The target must make a Fortitude save or be sickened for the duration of the ongoing acid damage.  Cold: The target must make a Fortitude save or be fatigued.  A creature that is already fatigued suffers no additional effect.  Electricity: The target must make a Fortitude save or be staggered for 1 round.  Fire: Your hands ignite and shed light as a torch. Your touch may cause targets to catch on fire (Core Rulebook 444).  Subsequent attacks inflict the normal damage, but the additional effects do not stack. This spell grants no special protection to anything held in or worn on your hands. When you cast this spell to deal acid, cold, electricity, or fire damage, it is a spell of that type."),
        full_text: true,
    },
    SpellListEntry {
        key: "Elude Time",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(5),
        description: Some("You place yourself in a state of suspended animation, similar to temporal stasis. At the time of casting, you choose when the stasis will expire, up to the maximum duration of the spell.  Until the duration ends, time ceases to flow for you, and all bodily functions cease. No force or effect can harm you until the spell expires or is removed, such as by a successful dispel magic spell."),
        full_text: true,
    },
    SpellListEntry {
        key: "Enemy Hammer",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(6),
        description: Some("You grab a creature with telekinesis and use it to batter nearby opponents or objects. You must target a specific creature when casting this spell and once you select that creature you cannot switch to another. Each round, as a standard action, you can attempt to hurl the target at any creature or object within 30 feet of it. You must make an attack roll whenever you use the target as a weapon. The attack bonus for this attack is equal to your caster level plus either your Intelligence or Charisma modifier (whichever is higher). If you successfully hit the new target with the creature both it and the creature take damage based on the creature's size: Fine 1d4, Diminutive 1d6, Tiny 1d8, Small 1d10, Medium 2d6, Large 2d8, Huge 2d10, Gargantuan 3d6, Colossal 3d8.  The target creature can make a Fortitude saving throw each time you attempt to use it as a weapon. If it makes its saving throw it can act normally, but if it fails its save it loses all actions for the round and ends its turn prone in a square adjacent to the target of your attack. However, if the creature chooses to resist your efforts to move it, taking no other actions for the round, it gets a +4 circumstance bonus on its saving throw. The spell ends immediately if the target creature dies or is destroyed."),
        full_text: true,
    },
    SpellListEntry {
        key: "Enter Image",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(2),
        description: Some("You gain a dim impression of the activities around any object bearing your face or form and can also project your consciousness into one such object at a time, allowing you to observe or even interact with nearby creatures. This spell allows you to leave your body and subconsciously monitor the immediate area around any depiction of your image whether one, hundreds, or even thousands. Each such image has the full range of your normal senses (sight, hearing, smell, etc.) and you can make a Perception check to notice anything occurring nearby. The DC of this check is always 10 greater than what it would be if you were actually present at that location, so even if you would normally notice something automatically (such as talking, normally a DC of 0), you would need to make a DC 10 Perception check to notice it.  You may, at any time, choose to fill one specific image within range with your consciousness. While inside an image you lose the ability to monitor any other images within range but remain fully aware of your own body's surroundings. Your body is defenseless and helpless (always failing any saving throw) while your consciousness is filling an image, but you can return to it at any time as an immediate action.  When you fill an image with your consciousness it gains a limited form of animation. Statues and similar three-dimensional representations can move their heads, make gestures, and talk, but cannot attack or leave their square without assistance.  Portraits or similar two-dimensional representations bearing your likeness can also talk and your image can move anywhere within the picture or even temporarily alter it (such as by picking a flower in the painting). You cannot use any spells or other abilities while within an image.  You may leave the image at any time, returning you to your body. Once your consciousness departs from an object bearing your likeness, it immediately reverts to the appearance it had before you entered it. If someone destroys or damages the image you return to your body unharmed. The spell lasts until you cease concentrating, or until it is terminated by some outside means, such as a dispel magic cast upon either the image or your body, or if your body is killed."),
        full_text: true,
    },
    SpellListEntry {
        key: "Euphoric Tranquility",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(6),
        description: Some("A creature under the effect of this enchantment enters a state of euphoria. The target treats all creatures as dear friends and abhors violence, but may rise up to protect itself if violence is perpetrated against it. Until the end of the spell's duration, the creature's speed is halved, and it cannot make attacks or cast spells. If the creature is attacked, it gets a Will saving throw. If the saving throw succeeds, the creature can make act normally for 1 round. If the saving throw fails, the creature moves half its speed away from the attacker as its next action.  Furthermore, if the creature is interacted with, or questioned while under the effects of this spell, it is considered to have an attitude of Helpful (see the Diplomacy skill, Core Rulebook 94), though any advice or answers it gives may be disjointed or stream of consciousness due to its euphoric state."),
        full_text: true,
    },
    SpellListEntry {
        key: "Evolution Surge",
        school: Some(Pf1SchoolId::Transmutation),
        level: None,
        description: Some("This spell functions as lesser evolution surge, except that you can grant any evolution whose total cost does not exceed 4 evolution points."),
        full_text: true,
    },
    SpellListEntry {
        key: "Evolution Surge (Greater)",
        school: Some(Pf1SchoolId::Transmutation),
        level: None,
        description: Some("This spell functions as lesser evolution surge, except that you can grant any two evolutions whose total cost does not exceed 6 evolution points."),
        full_text: true,
    },
    SpellListEntry {
        key: "Evolution Surge (Lesser)",
        school: Some(Pf1SchoolId::Transmutation),
        level: None,
        description: Some("This spell causes your eidolon to take on new characteristics.  You can grant the eidolon any evolution whose total cost does not exceed 2 evolution points. You may only grant one evolution with this spell, even if that evolution can be taken multiple times.  You can grant an evolution that allows you to spend additional evolution points to upgrade that evolution. This spell cannot be used to grant an upgrade to an evolution that the eidolon already possesses. The eidolon must meet any prerequisites of the selected evolution. This spell does not allow an eidolon to exceed its maximum number of natural attacks."),
        full_text: true,
    },
    SpellListEntry {
        key: "Expeditious Excavation",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("You can excavate and move earth, dust, and sand up to the size of a 5-foot cube. If you are buried, you may open a 5-foot cube around yourself, but the spell cannot be used for tunneling.  Besides its mundane applications, you can open a 5-foot-deep pit at a creature's feet. A Medium or smaller creature falls prone in the pit unless it succeeds on a Reflex save. With a successful save, it can choose to land harmlessly on its feet in the pit or hop to an adjacent square; this movement does not provoke attacks of opportunity. A creature can escape a 5-foot-deep pit with a DC 5 Climb check. Larger creatures may ignore pits smaller than their size.  The earth excavated by this spell is ordinarily distributed harmlessly across the spell's range, but you may choose to throw up a burst of grit and debris when you dig a pit. This cloud of debris provides concealment to any creatures in the square affected and all adjacent squares for 1 round. Expeditious excavation has no effect on solid rock or earth creatures."),
        full_text: true,
    },
    SpellListEntry {
        key: "Expend",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(7),
        description: Some("You cause all creatures in the affected area to use up one or more of their limited-use magical abilities without any actual effect. A limited-use magical ability is any supernatural or spelllike ability which a creature can only activate a certain number of times during a set interval (3/day, 1/hour, etc.). The spell does not affect abilities which the creature can use at will or which are constant. It also does not apply to magical items or anything else external to a creature. A creature in the spell's area must make a Will saving throw or expend a single use of one of its abilities. Abilities with the greatest number of uses per day are affected first; if more than one ability has the same number of uses, randomly determine which one is affected first. Creatures must continue making Will saves, expending an additional use of an ability for each failed save, until they succeed, at which point the spell has no further effect."),
        full_text: true,
    },
    SpellListEntry {
        key: "Feast of Ashes",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(2),
        description: Some("You curse the target with a hunger no food can assuage. On a failed saving throw, the target begins to starve (see page 444 of the Core Rulebook). The effects of this spell cannot inflict more nonlethal damage than the target has hit points. No amount of eating can counter the effect and, further, even attempting to eat causes the target to become nauseated for 1 round unless they make a DC 12 Fortitude save. The target still experiences the other normal effects of anything consumed (such as poisons or potions).  Feast of ashes can be removed with a break enchantment, limited wish, miracle, remove curse, or wish spell, but effects suffered as a result of starvation must be healed by normal means."),
        full_text: true,
    },
    SpellListEntry {
        key: "Feather Step",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("For the duration of this spell, the subject ignores the adverse movement effects of difficult terrain, and can even take 5-foot steps in difficult terrain."),
        full_text: true,
    },
    SpellListEntry {
        key: "Feather Step (Mass)",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(3),
        description: Some("As feather step, except this spell affects multiple creatures. For the duration of this spell, the subject ignores the adverse movement effects of difficult terrain, and can even take 5-foot steps in difficult terrain."),
        full_text: true,
    },
    SpellListEntry {
        key: "Fester",
        school: Some(Pf1SchoolId::Necromancy),
        level: Some(2),
        // Web second-source pass (SD-25 criterion 7.N, "apg-spell-text"):
        // no `.MOD` full-text record exists anywhere in `apg_spells.lst`
        // for this base record (verified: the only other `Fester.MOD`
        // token in the corpus is a corrupted/mismatched stanza describing
        // Feather Step's terrain mechanic, not Fester's SR effect -- see
        // this module's method-notes doc comment below). Identity-matched
        // (name + School Necromancy + Level Inquisitor 3/Witch 2 +
        // Components V,S,M[rotted meat], all exact) against
        // https://www.d20pfsrd.com/magic/all-spells/f/fester/.
        description: Some("Necrotic energy permeates the target, blocking healing abilities. The subject gains spell resistance equal to 12 + your caster level against effects that restore hit points or grant temporary hit points. In addition, any healing provided by effects that ignore spell resistance (such as fast healing, regeneration, and some spells) is halved. If the target succeeds on a Fortitude saving throw, fester lasts only 1 round."),
        full_text: true,
    },
    SpellListEntry {
        key: "Fester (Mass)",
        school: Some(Pf1SchoolId::Necromancy),
        level: Some(6),
        // Corpus-parsing correction (SD-25 criterion 7.N, "apg-spell-text"):
        // `apg_spells.lst:1945` concatenates two `.MOD` stanzas onto one
        // physical line with no line break (`Fester (Mass).MOD ... Fiery
        // Body.MOD ...`); the prior ingest took the line's *last* `DESC:`
        // token as this record's full text, which actually belongs to the
        // unrelated `Fiery Body` record further down the same line. This
        // record's own, correctly-attributed short `DESC:` (from
        // `apg_spells.lst:111`) is restored here; no genuine full-text
        // record exists for `Fester (Mass)` itself anywhere in the corpus.
        description: Some("This spell functions as fester, except that it affects multiple foes."),
        full_text: false,
    },
    SpellListEntry {
        key: "Fiery Body",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(9),
        // Corpus-parsing correction (SD-25 criterion 7.N, "apg-spell-text"):
        // this spell's real full-text `.MOD` record was present all along
        // at `apg_spells.lst:1945`, concatenated onto the same physical
        // line as `Fester (Mass).MOD` (see that record's note above) --
        // the prior ingest attributed it to `Fester (Mass)` instead.
        // Recovered directly from the corpus, no web fetch needed.
        description: Some("This spell transforms your body into living flame. You and your equipment are immune to fire damage. In fact, every time you would normally take damage from fire, you are instead healed of damage at a rate of 1 point per 3 points of damage the fire attack would have normally inflicted. You are immune to blindness, critical hits, ability score damage, deafness, disease, drowning, electricity, poison, stunning, and all spells that affect your physiology or respiration. You take only half damage from acid or electricity. You take 150%% as much damage from cold than normal.  You gain a +6 enhancement bonus to your Dexterity score and a fly speed of 40 ft. (perfect maneuverability). Your unarmed attack deals an additional 3d6 points of fire damage, and you are considered armed when making unarmed attacks. Your body burns so brightly that creatures who do not avert their gaze from you are dazzled. Fire spells you cast have their save DCs increased by +1. If you enter water, you are surrounded by a 5-foot radius of steam and bubbles that grant you concealment (50%% miss chance) but you take 2d6 points of damage each round you remain in water."),
        full_text: true,
    },
    SpellListEntry {
        key: "Fire Breath",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(2),
        description: Some("Up to thrice during this spell's duration, you can belch forth a cone of fire as a standard action. The first cone deals 4d6 points of fire damage to every creature in the area. The second cone of flame deals 2d6 points of fire damage to every creature in the area. The third cone of flame deals 1d6 points of fire damage to every creature in the area. A successful Reflex save halves this damage. After the third cone of flame, the spell ends."),
        full_text: true,
    },
    SpellListEntry {
        key: "Fire of Entanglement",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(2),
        description: Some("The next creature you attack using your smite evil class ability is wreathed in flames that impede its movement.  Until the end of the spell's duration, the target is entangled.  If the target starts its turn in a square adjacent to you, it is considered to be entangled to an immobile object (you) and cannot move. A target that saves against this spell is affected for only 1 round."),
        full_text: true,
    },
    SpellListEntry {
        key: "Fire of Judgment",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(3),
        description: Some("After casting this spell, the next creature you attack using your smite evil class ability is engulfed in flames of positive energy. At the start of its turn, the target takes 1d6 points of damage, and takes an additional 1d6 points of damage each time it attacks a creature other than you. If the creature is an outsider with the evil subtype, an evil-aligned dragon, or an undead creature this damage increases to 1d10. With a successful saving throw, a creature is affected by this spell for only 1 round. This damage is divine in nature and bypasses any DR the creature possesses."),
        full_text: true,
    },
    SpellListEntry {
        key: "Fire of Vengeance",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(4),
        description: Some("After casting this spell, the next creature you attack using your smite evil class ability is engulfed in holy flames that flare up when the target attacks someone other than you. If at any time during the duration of your smite evil effect the target makes an attack that does not include you, it takes 3d8 points of fire damage. Once triggered in this manner, or when your smite evil ability expires, the spell ends."),
        full_text: true,
    },
    SpellListEntry {
        key: "Fire Snake",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(5),
        description: Some("You create a sinuous line of flames that you may shape as desired. The fire snake affects one 5-foot square per caster level, and each square must be adjacent to the previous square, starting with you. The fire snake may not extend beyond its maximum range. Creatures in the path of the fire snake take 1d6 points of fire damage per caster level (maximum 15d6)."),
        full_text: true,
    },
    SpellListEntry {
        key: "Firebrand",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(7),
        description: Some("You mark several allies with a flaming rune. This rune does not cause damage, and sheds light as if it were a torch. While the firebrand burns, any creature it marks is immune to damage from any fire spell you cast. All of the target's weapons (both natural and manufactured) inflict +1d6 points of fire damage on a hit (this bonus fire damage stacks with any amount of fire damage a creature's weapons might already inflict). At any point during the spell's duration, a creature bearing a firebrand can launch a beam of fire at any target within 30 feet as a swift action. This ray requires a ranged touch attack to hit and deals 6d6 points of fire damage. Once a creature uses its firebrand to fire a ray in this manner, the effects of the spell end for that creature."),
        full_text: true,
    },
    SpellListEntry {
        key: "Firefall",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(4),
        description: Some("Firefall causes a fire to erupt into a geyser of dazzlingly bright liquid flame. The spell uses one fire source, which is immediately extinguished. A fire larger than a 20-foot cube is only partly extinguished. Magical fires are not extinguished, but a creature of the fire subtype used as the source takes 1 point of damage per caster level (no saving throw).  The coruscating rain of fire fills a hemispherical burst with a radius of 60 feet. All creatures and objects in the area take 5d6 points of fire damage and catch on fire (Core Rulebook 444).  Creatures who make successful Reflex saves take half damage and don't catch on fire. Creatures within 120 feet of the original fire source are blinded for 1d4+1 rounds (Will negates)."),
        full_text: true,
    },
    SpellListEntry {
        key: "Flames of the Faithful",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(2),
        description: Some("With a touch, you cause a glowing rune to appear on a single weapon, granting that weapon the flaming property (and allowing it to cause an extra 1d6 points of fire damage on a successful hit). If you are using the judgment class feature, your weapon gains the flaming burst property instead. The spell functions only for weapons that you wield. If the weapon leaves your hand for any reason, the spell effect ends. The effects of this spell do not stack with any existing flaming or flaming burst weapon property that the target weapon may already possess."),
        full_text: true,
    },
    SpellListEntry {
        key: "Flare Burst",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(1),
        description: Some("This spell functions as flare, except it affects all creatures in a 10-foot-radius burst from the target point."),
        full_text: true,
    },
    SpellListEntry {
        key: "Fluid Form",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(4),
        description: Some("When you cast this spell, your body takes on a slick, oily appearance. For the duration of this spell, your form can stretch and shift with ease and becomes slightly transparent, as if you were composed of liquid. This transparency is not enough to grant concealment. You gain DR 10/slashing and your reach increases by 10 feet. In addition, you can pass through small holes or narrow openings, even mere cracks, with anything you were carrying at the time the spell was cast (except other creatures). Finally, you can move through water with a swim speed of 60 feet and can breathe both water and air for the duration of this effect. You are treated as if you had the water subtype while this spell is in effect."),
        full_text: true,
    },
    SpellListEntry {
        key: "Fly (Mass)",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(7),
        description: Some("This spell functions as fly (Core Rulebook 284), save that it can target numerous creatures and lasts longer."),
        full_text: true,
    },
    SpellListEntry {
        key: "Foe to Friend",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(5),
        description: Some("Cast this spell when an enemy is about to make an attack against an ally. The creature makes the attack against a valid target of your choice instead or the attack is negated.  In addition, the target is considered an ally for determining flanking for 1 round."),
        full_text: true,
    },
    SpellListEntry {
        key: "Follow Aura",
        school: Some(Pf1SchoolId::Divination),
        level: Some(2),
        description: Some("Choose one alignment type: chaotic, evil, good, or lawful. You gain the ability to follow the trail of a strong or overwhelming aura of that alignment (see detect evil, Core Rulebook 266). This is treated as tracking using the scent special quality, except you are following the traces of their aura upon the ground rather than their scent, and you receive no bonus on Perception checks. At 10th level, you can track the trail of creatures with a moderate alignment aura as well.  When you use this spell to track a given alignment, it gains the descriptor of the opposite alignment. For example, when follow aura is used to track evil, it gains the good descriptor."),
        full_text: true,
    },
    SpellListEntry {
        key: "Fool's Forbiddance",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(6),
        description: Some("Through your antics and performance, you create an area of warding that adversely affects all enemies that dare enter it. When an enemy creature enters the area it must make an immediate Will saving throw. If it fails, the creature is confused as long as it is in the area and for 1 round after it leaves. If it succeeds on the saving throw, the creature is staggered as long as it is in the area and for 1 round after it leaves."),
        full_text: true,
    },
    SpellListEntry {
        key: "Forced Repentance",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(4),
        description: Some("You force an evil creature that does not have the evil subtype to momentarily reflect on its past actions and be overcome by grief and conscience. The target immediately drops prone and begins to loudly confess all of its sins and transgressions to the caster for the duration of the spell. The spell immediately ends if you move out of line of sight or if the target is attacked."),
        full_text: true,
    },
    SpellListEntry {
        key: "Frozen Note",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(5),
        description: Some("You produce and sustain a single perfect musical note that holds nearby creatures, friend as well as foe, utterly spellbound until you stop singing. Affected creatures are both paralyzed and oblivious to their surroundings so long as you maintain the note.  Maintaining this note requires your absolute attention; if you take damage or take any other action, including moving from your current square, the spell instantly ends. Creatures that succeed in their initial saving throw must make a new one for each round they spend within the area of the spell until they fail and become captivated. Creatures with 4 or more Hit Dice greater than your caster level are not affected by this spell. However, those with 4 or more Hit Dice less than your caster level do not receive a saving throw to resist it. The note is so clear and flawless that less than extraordinary attempts to avoid hearing it merely grant targets a bonus on their saving throws. Targets get a +2 circumstance bonus on their saving throw for each barrier between your voice and their ears. This includes such things as a creature stopping up its ears with wax, closing all windows and doors in a nearby building, or even crawling into a barrel and pulling down the top.  If a target that had previously made its saving throw loses the benefit of one or more of its barriers it must immediately make a new saving throw. Similarly, any target that had previously failed its saving throw gets a new save each time it gains the protection of a barrier."),
        full_text: true,
    },
    SpellListEntry {
        key: "Gallant Inspiration",
        school: Some(Pf1SchoolId::Divination),
        level: Some(2),
        description: Some("This word of arcane-empowered inspiration often ensures success of a crucial endeavor. Cast this spell when a creature fails an attack roll or skill check. The creature gains a +2d4 competence bonus to the attack roll or skill check retroactively.  If the bonus is enough to turn the failure into a success, the roll succeeds."),
        full_text: true,
    },
    SpellListEntry {
        key: "Getaway",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(6),
        description: Some("Getaway teleports you and allies you designate at the time of casting to a predetermined location. You must initially cast the spell at that location-all creatures affected by the spell must be present at that time and within 30 feet of you. At any time before the spell's duration expires, you may trigger the spell as a swift action. At this time, all affected creatures (or their remains and gear) within 30 feet of you are teleported to the location where you first cast the spell. Creatures more than 30 feet away from you are left behind. Selected creatures within 30 feet of you who do not wish to travel with you at that time can simply choose not to go. You can be transported any distance within a plane but cannot travel between planes."),
        full_text: true,
    },
    SpellListEntry {
        key: "Geyser",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(4),
        description: Some("You cause a column of boiling water to spring forth from any horizontal surface, knocking over creatures directly over it and exposing nearby creatures to searing droplets as its spray falls back to the ground.  Any creature entering the geyser, or occupying the square it appears in, must make a Reflex saving throw to avoid being hurled into the air and then tossed to the ground. If the creature fails its saving throw, it takes 3d6 points of fire damage from the boiling water and also takes falling damage based upon the height of the geyser (e.g., if the geyser is 50 feet tall, the creature takes 5d6 falling damage), landing prone in a random square adjacent to the geyser. A successful saving throw halves the damage and negates the falling damage, and the creature is moved to the closest square adjacent to the geyser (Large-sized or larger creatures are moved enough so that they are not on top of the geyser but still adjacent to it).  This movement does not provoke attacks of opportunity and does not count toward the creature's normal movement.  In addition, the geyser sprays boiling water in a hemispherical emanation around its square. The radius of this emanation is equal to one-half the geyser's height (e.g., a 50-foot geyser has a 25-foot-radius emanation). Any creature within this area, including yourself, takes 1d6 points of fire damage each round as droplets of boiling water cascade on them.  You can choose to make a smaller geyser than your level permits if an obstruction prevents it from reaching its full height, or if you simply want to create a spread of boiling rain that's smaller than what would be created by a full-height geyser spell."),
        full_text: true,
    },
    SpellListEntry {
        key: "Ghostbane Dirge",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("The target coalesces into a semi-physical form for a short period of time. While subject to the spell, the incorporeal creature takes half damage (50%%) from nonmagical attack forms, and full damage from magic weapons, spells, spell-like effects, and supernatural effects."),
        full_text: true,
    },
    SpellListEntry {
        key: "Ghostbane Dirge (Mass)",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(3),
        description: Some("This spell functions as ghostbane dirge, except that it affects multiple targets."),
        full_text: true,
    },
    SpellListEntry {
        key: "Glide",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("You take no damage from falls (as if from feather fall). In addition, you can move up to 5 feet in any horizontal direction for every 1 foot you fall, at a speed of 60 feet per round. You cannot use this spell to actually gain height, merely coast in other directions as you fall. If subjected to a strong wind or any other effect that causes you to rise you can take advantage of it in order to increase the distance you can glide. The spell ends as soon as your feet touch the ground regardless of its remaining duration. If the spell expires while you are still in the air you fall the remaining distance as normal."),
        full_text: true,
    },
    SpellListEntry {
        key: "Grace",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(1),
        description: Some("Until the end of your turn, your movement does not provoke attacks of opportunity."),
        full_text: true,
    },
    SpellListEntry {
        key: "Gravity Bow",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("Gravity bow significantly increases the weight and density of arrows or bolts fired from your bow or crossbow the instant before they strike their target and then return them to normal a few moments later. Any arrow fired from a bow or crossbow you are carrying when the spell is cast deals damage as if one size larger than it actually is. For instance, an arrow fired from a Medium longbow normally deals 1d8 points of damage, but it would instead deal 2d6 points of damage if fired from a gravity bow (see page 145 of the Core Rulebook for more information). Only you can benefit from this spell. If anyone else uses your bow to make an attack the arrows deal damage as normal for their size."),
        full_text: true,
    },
    SpellListEntry {
        key: "Grove of Respite",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(4),
        description: Some("You conjure a sheltered grove of trees surrounding a shallow spring. The grove appears from nowhere even in the most barren, rocky soil regardless of season, but the spell must be cast outside on open ground.  The area within the grove is temperate and comfortable, like that of a tiny hut, although the grove provides no illumination and provides no protection from the elements. The water in the grove's spring is clean and drinkable. Additionally, the trees provide a variety of ripe fruit, regardless of season, which acts as a goodberry spell for up to eight people. The grove is warded with an alarm spell that alerts you whenever any creature enters the area. Once the spell expires, the grove vanishes, including all fruit, water, and other materials from it, although any nourishment or healing gained from its effects remains."),
        full_text: true,
    },
    SpellListEntry {
        key: "Guiding Star",
        school: Some(Pf1SchoolId::Divination),
        level: Some(2),
        description: Some("You form a bond with your surroundings when you cast this spell. For the remaining duration of the spell you can always, as a standard action, determine your approximate distance from that area as well as the direction you must travel in order to reach it. You cannot determine the location of the area if you are on a different plane. The area counts as \"very familiar\" for the purposes of teleport or similar spells. You can only attune yourself to one location at a time. If you cast the spell at another spot you lose the ability to locate your original area."),
        full_text: true,
    },
    SpellListEntry {
        key: "Heroic Finale",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(4),
        description: Some("You must have a bardic performance in effect to cast this spell. With a flourish, you immediately end the performance, and one creature within range affected by your bardic performance can make a move action or a standard action of their choice."),
        full_text: true,
    },
    SpellListEntry {
        key: "Hero's Defiance",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(1),
        description: Some("The instant before you are reduced to 0 or fewer hit points, you can expend a use of your lay on hands ability to heal yourself as if you had used lay on hands, plus an additional 1d6 hit points.  If this healing brings your hit point total above 0 hit points, you do not fall, and may continue to act. If you have no more uses of lay on hands this spell has no effect."),
        full_text: true,
    },
    SpellListEntry {
        key: "Hidden Speech",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(2),
        description: Some("You greatly enhance the ability of the subjects to communicate hidden messages to each other. Each target gains a +10 competence bonus on Bluff checks to send secret message to other recipients. Others using opposed Sense Motive checks to decipher these messages receive a -5 penalty. All subjects affected by the spell must share a language to be able to pass messages."),
        full_text: true,
    },
    SpellListEntry {
        key: "Hide Campsite",
        school: Some(Pf1SchoolId::Illusion),
        level: Some(2),
        description: Some("You make the area around a campsite appear to be a thicket of untouched and forbidding foliage, or some other unwelcoming feature matching the surrounding terrain. Creatures outside the area cannot sense any activity going on inside the area- they cannot smell campfires or cooking food, they cannot hear conversation, loud noises, or spells being cast, and they cannot even feel heat or a rush of wind coming from the area. Those inside the area can see out normally. Once a creature steps into the area of the spell, it can see everything in and around the area normally."),
        full_text: true,
    },
    SpellListEntry {
        key: "Holy Whisper",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(3),
        description: Some("You whisper a single word in the primordial language of good that is anathema to the minions of evil and strengthens the resolve of good creatures. Evil creatures within the burst must make a Fortitude saving throw or become sickened for 1 round/ level. Evil outsiders with the evil subtype, evil-aligned dragons, and undead in the burst also take 2d8 points of damage if they fail their saves. Good-aligned creatures in the burst gain a +2 sacred bonus on attack and damage rolls for 1 round."),
        full_text: true,
    },
    SpellListEntry {
        key: "Honeyed Tongue",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("This spell augments your diplomacies. While under the effects of spell, you roll two dice each time you make a Diplomacy check to change a creature's attitude, taking the highest roll.  If this results in a roll low enough to reduce the creature's attitude by a step, that creature gets some clue that you are manipulating it with a spell. Alternatively you can cast this spell before making a Diplomacy check to gather information, gaining a +5 competence bonus on the check."),
        full_text: true,
    },
    SpellListEntry {
        key: "Hungry Pit",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(5),
        description: Some("This spell functions as create pit, except that the pit has the ability to squeeze and crush any creature trapped within it and has a maximum depth of 100 feet. Creatures who fall into the hole take falling damage as normal. In addition, anyone within the pit, not just those on the bottom, takes 4d6 points of bludgeoning damage each round as the pit contracts and then returns to its normal size (a successful Reflex save halves this damage). The ever-shifting walls of the pit are quite difficult to scale and have a Climb DC of 35."),
        full_text: true,
    },
    SpellListEntry {
        key: "Hunter's Eye",
        school: Some(Pf1SchoolId::Divination),
        level: Some(2),
        description: Some("Sharpening your perceptive abilities and tuning them against obfuscating effects, hunter's eye greatly enhances your senses against the target creature. You gain the ability to perceive the target when it is invisible or ethereal as though using the see invisibility spell, and receive a +20 competence bonus on Perception checks to locate the target. You ignore concealment provided by fog or mist, blur, displacement, invisibility, and similar effects affecting the target, but not concealment provided by darkness. This spell provides no benefits to your allies, and no benefits against creatures other than the target."),
        full_text: true,
    },
    SpellListEntry {
        key: "Hunter's Howl",
        school: Some(Pf1SchoolId::Necromancy),
        level: Some(1),
        description: Some("With a single primal howl, you strike fear in your opponents.  Until the end of the spell's duration, you treat those affected by it as if they were your favored enemy, gaining a +2 bonus on weapon attack and damage rolls against them, and a +2 bonus on Bluff, Knowledge, Perception, Sense Motive, and Survival checks against them. If an affected creature is already one of your favored enemies, it is shaken instead. This is a mindaffecting fear effect."),
        full_text: true,
    },
    SpellListEntry {
        key: "Hydraulic Push",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(1),
        description: Some("You call forth a quick blast of water that knocks over and soaks one creature or square. You can use this blast of water to make a bull rush against any one creature or object. Your CMB for this bull rush is equal to your caster level plus your Intelligence, Wisdom, or Charisma modifier, whichever is highest. This bull rush does not provoke an attack of opportunity. Hydraulic push extinguishes any normal fires on a creature, object, or in a single 5-foot square which it is targeted against. Magical fires are unaffected."),
        full_text: true,
    },
    SpellListEntry {
        key: "Hydraulic Torrent",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(3),
        description: Some("You call forth a powerful stream of water that batters all creatures and obstacles in its path until it strikes something it cannot push past. Against creatures and moveable objects this stream acts as a bull rush. You can bull rush creatures of any size, not just those one size larger than your own. Make a combat maneuver check and apply its results to each creature within the area. Your CMB for this bull rush is equal to your caster level plus your Intelligence, Wisdom, or Charisma modifier, whichever is highest. This bull rush does not provoke an attack of opportunity.  Against immovable objects this stream instead allows you to make a Strength check to destroy the target. When attempting to break an object, the stream has an effective Strength equal to your caster level plus the ability score modifier as above. The Break DC depends on the object you're trying to break-page 175 of the Pathfinder RPG Core Rulebook lists several sample Break DCs for various objects.  Hydraulic torrent extinguishes any normal fires it encounters along its path. Magical fires are unaffected."),
        full_text: true,
    },
    SpellListEntry {
        key: "Ill Omen",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(1),
        description: Some("You afflict the target with bad luck. On the next d20 roll the target makes, it must roll twice and take the less favorable result. For every five caster levels you have, the target must roll twice on an additional d20 roll (to a maximum of five rolls at 20th level).  A target who can speak and has at least one free hand and who is aware of the spell and its effects (such as from a Spellcraft check to identify the spell as it is cast) can negate one reroll by spending a move action to utter a brief prayer or good luck charm to appease the spirits of ill fortune."),
        full_text: true,
    },
    SpellListEntry {
        key: "Innocence",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("You surround yourself with an aura of innocence and trustworthiness. You gain a +10 competence bonus on Bluff skill checks to convince others of your innocence. They find it difficult to believe you capable of any wrongdoing. This bonus does not apply to other uses of the Bluff skill, such as feinting in combat, creating a diversion to hide, or communicating secret messages via innuendo, nor does it apply to any use of the skill to convince anyone of anything other than your complete innocence and blamelessness."),
        full_text: true,
    },
    SpellListEntry {
        key: "Instant Armor",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(2),
        description: Some("You instantly wrap your body in a suit of armor made from opaque force. At your option, the armor can be decorated with your religion's holy symbol in a prominent place, such as upon the breastplate or helm. While it exists, this armor replaces any garments or other sort of armor worn, magical as well as mundane. You lose access to, and all benefits from, armor replaced by this spell until the spell ends and the instant armor disappears.  Instant armor acts in all ways as armor typical of its type (armor bonus, maximum Dexterity bonus, arcane spell failure chance, and so on). Since instant armor is made of force, incorporeal creatures can't bypass it the way they do normal armor. The sort of armor you can create with this spell depends on your caster level: 5th or lower chainmail, 6th-8th banded mail, 9th-11th half-plate, 12th or higher full plate.  If you choose to create lesser armor than your level allows you gain a +1 magical enhancement bonus to the armor you do create for every type of armor you pass over. For instance, if you have the capacity to create full plate instant armor you could instead choose to create +1 half-plate, +2 banded mail, or +3 chainmail. You cannot replace these bonuses with armor special properties."),
        full_text: true,
    },
    SpellListEntry {
        key: "Instant Enemy",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(3),
        description: Some("With this spell you designate the target as your favored enemy for the remainder of its duration. Select one of your favored enemy types. For the duration of the spell, you treat the target as if it were that type of favored enemy for all purposes."),
        full_text: true,
    },
    SpellListEntry {
        key: "Invigorate",
        school: Some(Pf1SchoolId::Illusion),
        level: Some(1),
        description: Some("This spell banishes feelings of weariness. For the duration, the subject takes no penalties from the fatigued or exhausted conditions. The effect of invigorate is merely an illusion, however, not a substitute for actual rest or respite. When the spell ends, the subject takes 1d6 points of nonlethal damage, along with the return of the original condition(s). A creature can be under the effects of only one invigorate spell at a time; if it is cast a second time on that creature, the first effect immediately ends."),
        full_text: true,
    },
    SpellListEntry {
        key: "Invigorate (Mass)",
        school: Some(Pf1SchoolId::Illusion),
        level: Some(3),
        description: Some("This spell work as invigorate, except it affects multiple creatures."),
        full_text: true,
    },
    SpellListEntry {
        key: "Jester's Jaunt",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(3),
        description: Some("You teleport the target to a space you can see within 30 feet of the target. The destination must be on solid ground, and the teleportation cannot end in a space that is by nature hazardous to the creature you are teleporting."),
        full_text: true,
    },
    SpellListEntry {
        key: "Keen Senses",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("The subject gains a +2 competence bonus on Perception checks and gains low-light vision. Subjects that have low-light vision double the distance they can see under the effects of this spell."),
        full_text: true,
    },
    SpellListEntry {
        key: "King's Castle",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(4),
        description: Some("This spell allows you to switch places with an ally. When you cast this spell, choose a single ally within range. You teleport to your ally's space while your ally teleports to your former space."),
        full_text: true,
    },
    SpellListEntry {
        key: "Knight's Calling",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(1),
        description: Some("You compel an enemy to come and fight you. On its turn, the target moves its speed toward you, avoiding any other dangers along its path (including any movement that would provoke attacks of opportunity). The target may do nothing but move on its turn. If the target ends its move adjacent to you, you can make an attack of opportunity against the target."),
        full_text: true,
    },
    SpellListEntry {
        key: "Lead Blades",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("Lead blades increases the momentum and density of your melee weapons just as they strike a foe. All melee weapons you are carrying when the spell is cast deal damage as if one size category larger than they actually are. For instance, a Medium longsword normally deals 1d8 points of damage, but it would instead deal 2d6 points of damage if benefiting from lead blades (see page 145 of the Core Rulebook for more information). Only you can benefit from this spell. If anyone else uses one of your weapons to make an attack it deals damage as normal for its size."),
        full_text: true,
    },
    SpellListEntry {
        key: "Life Bubble",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(3),
        description: Some("You surround the touched creatures with a constant and moveable 1-inch shell of tolerable living conditions. This shell enables the subjects to breathe freely, even underwater or in a vacuum, as well as making them immune to harmful gases and vapors, including inhaled diseases and poisons and spells like cloudkill and stinking cloud. In addition, the shell protects subjects from extremes of temperature (per endure elements) as well as extremes of pressure.  Life bubble does not provide protection from negative or positive energy (such as found on the Negative and Positive Energy planes), the ability to see in conditions of poor visibility (such as in smoke or fog), nor the ability to move or act normally in conditions that impede movement (such as underwater).  When you cast this spell it has a total duration of 2 hours per caster level. You can divide this duration up in any manner you wish, not necessarily equally, between up to 1 creature per caster level."),
        full_text: true,
    },
    SpellListEntry {
        key: "Light Lance",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(2),
        description: Some("You create a glorious beam of brilliant radiance that acts as a +1 holy lance suitable for your size. You must have a free hand when casting the spell and, once you call the lance into being, you cannot switch it to another hand or put it down without prematurely ending the spell. As a standard action while holding a light lance, you can choose not to make an attack and instead hold the lance aloft as a beacon of light and truth, creating an area of bright light with a radius of 90 feet. If you choose to do this for one or more additional rounds the lance also casts a spear of brilliant light up into the sky that grows progressively more visible as it continues to climb toward the heavens. Under cover of night or other darkness, if nothing obstructs the beam, it becomes visible for an additional 2 miles during each round in which you continue to aim your lance skyward (2 miles in the second round, 4 miles in the third round, and so on)."),
        full_text: true,
    },
    SpellListEntry {
        key: "Lily Pad Stride",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(3),
        description: Some("Your every footstep creates aquatic plants capable of supporting your weight and that of any creature of your size or smaller that is following you. These lily pads only appear when you cross water or other liquids which do not immediately destroy plants. You can walk across these liquid surfaces without any fear of stumbling (though if knocked prone, dragged under, or otherwise pulled off your feet you still sink in the liquid). For the duration of the spell, any creature of your size or smaller can attempt to follow you by making a DC 10 Acrobatics check every round. Each such check allows it to move at half its normal land speed. If the creature takes a -5 penalty on its check it can move at its normal speed.  If a creature fails an Acrobatics check, or if a creature that's larger than you attempts to follow, it falls through, damaging the plants. Each time a creature falls through, all subsequent creatures take a cumulative -5 penalty on their Acrobatics checks when traveling across that particular stretch of plants.  As you move, the lily pads disappear behind you once you pass beyond the spell's range. Otherwise, they remain until the spell's duration expires."),
        full_text: true,
    },
    SpellListEntry {
        key: "Lockjaw",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(2),
        description: Some("You give a creature the ability to use one of its natural attacks to firmly attach itself to an opponent. Choose one of the creature's natural attacks (usually a claw or bite attack).  The creature gains the grab ability with that natural attack, including the +4 bonus on combat maneuver checks to start or maintain a grapple. A creature with multiple natural attacks can strike at its grappled opponent with its other natural attacks, but cannot attack any other creature."),
        full_text: true,
    },
    SpellListEntry {
        key: "Marks of Forbiddance",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(3),
        description: Some("Marks of Forbiddance magically brand two creatures, preventing them from committing violence against one another. When you cast this spell choose one enemy and one ally within range. Both targets become branded with the sigil of forbiddance. Until the end of the spell's duration, in order for the two targets to attack one another, even with a targeted spell, each attacker must attempt a Will save. If the save succeeds, that target can attack the other normally.  If the save fails, the target can't follow through with the attack and the action is lost. This spell does not prevent the targets from attacking each other with area of effect spells or abilities."),
        full_text: true,
    },
    SpellListEntry {
        key: "Mask Dweomer",
        school: Some(Pf1SchoolId::Illusion),
        level: Some(1),
        description: Some("You mask and manipulate a spell's magic aura to make it harder to detect. Select one spell effect on the target creature or object. You must have either cast this spell yourself or have perceived it by means such as detect magic or arcane sight.  Both the desired spell effect and mask dweomer are hidden from detect magic, although more powerful spells (such as arcane sight) pierce the deception if the caster succeeds on a Will save. Analyze dweomer automatically detects both mask dweomer and any masked spell effects."),
        full_text: true,
    },
    SpellListEntry {
        key: "Memory Lapse",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(1),
        description: Some("You cause the target to forget what happened from the casting of the spell back to the beginning of its last turn. This may allow a retry on a Diplomacy, Intimidate, or opposed skill check, though only with respect to the target, not other creatures that may be present."),
        full_text: true,
    },
    SpellListEntry {
        key: "Moonstruck",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(4),
        description: Some("You invoke the mystical power of the moon to drive the target into a mad, bestial frenzy. If the target fails its save, it is dazed for 1 round, dropping held items as its nails and teeth elongate and sharpen. The target gains a bite attack and two claw attacks that deal damage appropriate for the creature's size, and for the remainder of the spell's duration the target behaves as if under simultaneous rage and confusion spells, attacking with its natural weapons in preference to other actions. During the final round of the spell's duration, the target is again dazed as it returns to its normal state."),
        full_text: true,
    },
    SpellListEntry {
        key: "Nap Stack",
        school: Some(Pf1SchoolId::Necromancy),
        level: Some(3),
        description: Some("You reduce the amount of uninterrupted sleep or rest creatures within the spell's area need in order to recover from injuries, regain spells, or other special abilities to 2 hours instead of the normal eight. In addition, if creatures continue to sleep or rest beyond the initial 2 hours, every additional 2 hours counts as a day of rest for the purpose of recovering hit points, ability damage, as well as for enduring diseases, poisons, or other afflictions. This means 8 total hours of sleep counts as 4 days for natural healing and for saving throws as diseases or similar afflictions run their course. When suffering from diseases, poison, or other afflictions, sleepers experience vivid dreams that help them track their recovery. If things go poorly they can, at any time, wake themselves up in order to seek a better alternative. If awoken or otherwise disturbed during this 8-hour period, creatures may return to sleep but they no longer enjoy the benefits of the accelerated recovery time. Creatures can only enjoy the benefits of this spell once in any 1-week period."),
        full_text: true,
    },
    SpellListEntry {
        key: "Natural Rhythm",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(2),
        description: Some("You enhance a creature's ability to harm an opponent based on the number of times it has already injured that opponent with a natural attack. Each time the creature successfully strikes an opponent with a natural attack, the subject gains a cumulative +1 bonus on damage rolls made against that opponent when making natural attacks (maximum +5 bonus). If an attack misses, the bonus to damage for all attacks resets to +0."),
        full_text: true,
    },
    SpellListEntry {
        key: "Nature's Exile",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(3),
        description: Some("This spell curses the creature touched, making it inimical to the natural world. All animals have an initial attitude of hostile toward the target. Familiars, being magical beasts, are not affected by this spell, but animal companions are. If you have an animal companion, it does not become hostile, but as long as you remain cursed, your animal companion takes a -2 penalty on all attack rolls, skill checks, and saving throws. The target also takes a -10 penalty on Survival skill checks as the weather and environment themselves seem to conspire to cause trouble.  Nature's exile can be removed with break enchantment, limited wish, miracle, remove curse, or wish."),
        full_text: true,
    },
    SpellListEntry {
        key: "Negate Aroma",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("With a gesture, this spell allows you to dismiss even the foulest or most distinctive scents. When cast, the targeted creatures or objects lose all natural and unnatural odors. A creature under the effect of negate aroma cannot be tracked, located, or pinpointed by the scent special quality. In addition, this spell prevents the target creature from using the stench special ability and similar odor-based abilities (such as those possessed by troglodytes).  Negate aroma does not prevent the target from acquiring outside smells or odors. Dowsing the creature with a pungent substance effectively negates the benefits of the spell until the substance is neutralized or washed away."),
        full_text: true,
    },
    SpellListEntry {
        key: "Oath of Peace",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(4),
        description: Some("Entreating your deity for aid, you make a temporary oath of peace, granting you superior defenses but means you can't attack for the duration of the oath. For as long as you are subject to this spell, you gain a +5 sacred bonus to AC and on saving throws, as well as DR 10/evil. If you make a direct or indirect attack or any show any hostility toward any creature, the spell immediately ends."),
        full_text: true,
    },
    SpellListEntry {
        key: "Pain Strike",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(3),
        description: Some("Pain strike racks the targeted creature with agony, inflicting 1d6 points of nonlethal damage per round for 1 round per level (maximum 10 rounds). Additionally, the affected creature is sickened for the spell's duration, and the caster gains a +4 circumstance bonus on Intimidate checks against the target."),
        full_text: true,
    },
    SpellListEntry {
        key: "Pain Strike (Mass)",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(5),
        description: Some("This spell works like pain strike, except as noted above."),
        full_text: true,
    },
    SpellListEntry {
        key: "Paladin's Sacrifice",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(2),
        description: Some("You open up a brief but powerful divine conduit between you and another creature, taking on the damage and any other effects that creature suffers. When a creature in range is hit by an attack or fails a saving throw, you can cast this spell and the wounds and/or effects are magically transmitted to you instead of the target. You are affected as if you were hit by the attack or failed the saving throw, taking all the damage and suffering all of the adverse effects. Any resistances or immunities you have are applied normally, but you cannot otherwise reduce or negate the damage or effects in any way.  If you use this spell against an effect that also targets you or includes you in its area, you suffer the effects for both yourself and the target you spared, potentially taking damage or suffering other consequences twice."),
        full_text: true,
    },
    SpellListEntry {
        key: "Perceive Cues",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(2),
        description: Some("Your senses become preternaturally keen, and you gain insight into subtle behavioral cues. For the duration of the spell, you receive a +5 competence bonus on Perception and Sense Motive checks."),
        full_text: true,
    },
    SpellListEntry {
        key: "Phantasmal Revenge",
        school: Some(Pf1SchoolId::Illusion),
        level: Some(7),
        description: Some("When you cast this eerie spell upon a recently slain creature, you cause a ghastly, spectral image of it to rise up from its corpse, shrieking for vengeance before it vanishes in a burst of unnatural light. This phantom then unerringly seeks out the creature that killed it, as long as that creature is on the same plane, and tries to slay its killer in turn.  Only the corpse's killer can see the phantasmal image created by this spell. You and any others who witness the spell's casting or its ultimate effects see only a vague shape. The target first gets a Will save to disbelieve the illusion. If that save fails, the target must succeed on Fortitude saving throw or take 10 points of damage per caster level from the phantasmal image. Even if the Fortitude save is successful, the target takes 5d6 points of damage + 1 point of damage per caster level. You know immediately if the spell succeeded or failed to kill its target but gain no other information, including the identity of the target of the spell.  The creature providing the corpse must have died no more than 1 day per caster level prior to the casting of this spell. If the creature that provided the corpse was not, in fact, killed by another, or if the killer is dead or no longer on the same plane, the spell fails. Phantasmal revenge makes no moral judgments; it targets winners of fair fights just as easily as murderers. Any given body can only be targeted by a phantasmal revenge spell once."),
        full_text: true,
    },
    SpellListEntry {
        key: "Phantasmal Web",
        school: Some(Pf1SchoolId::Illusion),
        level: Some(5),
        description: Some("You implant within the minds of your targets the illusion that they are engulfed in tangled webs teeming with swarms of tiny spiders. Those who fail to disbelieve the phantasmal web are treated as if in a web spell, but must also make a Fortitude save at the beginning of each turn or become nauseated for that round by the phantasmal spiders.  As the phantasmal web exists only in the minds of the targets, it cannot be burned or destroyed, and it provides no cover (though it does provide concealment) against attacks made by the targets. Targets cannot escape the phantasmal web by moving, even by teleportation. Freedom of movement allows unobstructed movement but does not negate the concealment or nausea effects.  Targets of the spell perceive everyone else around them to be engulfed in webs and swarming spiders, but the spell has no visible effect to other creatures (who may assist allies to disbelieve the effect)."),
        full_text: true,
    },
    SpellListEntry {
        key: "Pied Piping",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(6),
        description: Some("You call forth a melody so compelling and attractive that creatures belonging to a group with a specific, identifiable physical trait (such as type, subtype, age, gender, or hair color) find it almost impossible not to approach and follow you. You choose the nature of the creatures you seek to attract when you cast the spell and, once you make this choice, cannot alter it. You can only use physical traits to determine your audience. You can never use social and other more voluntary criteria (such as alignment, religion, nationality, or class) to select targets for this spell.  Eligible creatures must make a saving throw each round spent within the area of the spell and, once they fail, approach you at their normal movement speed and stand as close to you as possible without actually entering an occupied square.  If you move, they do their best to maintain their proximity to you and follow. Affected creatures can take no actions other than listening to your music and following you if you move.  Those entering the area while the spell is in effect must also successfully save or be compelled to come to you.  The attractive power of the spell does not cause affected creatures to put themselves in jeopardy and they have enough presence of mind to avoid or negotiate around obstacles and other perilous hazards. If unable to approach or follow you without endangering themselves, they simply wait, swaying to the music, until you pass out of range, at which point they regain their senses after 1 round/caster level. If circumstances change once you move out of range, making it possible for affected creatures to resume their attempts to get near you they do so, and, if they manage to return to within the area of the spell before it wears off, the spell continues as normal.  If attacked, affected creatures can take defensive measures, even going so far as to avoid existing threats by moving out of the area of the spell, but cannot make attacks themselves or take any other actions until the effects of the spell wear off. The effects of the spell persist for 1 round/caster level even after you stop concentrating. Once the spell ceases, the affected creatures continue to stay near you until the effects of the spell wear off."),
        full_text: true,
    },
    SpellListEntry {
        key: "Pillar of Life",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(5),
        description: Some("You conjure a pillar of positive energy in a single 5-foot square within range that radiates light as if it were a sunrod. Living creatures adjacent to the pillar can spend a standard action to touch the pillar and heal 2d8 points of damage + 1 point per caster level (maximum +20). Creatures can move into the square containing the pillar, but if an undead creature moves into the pillar it takes 1d6 points of damage per caster level (maximum 10d6). Undead creatures vulnerable to bright light take 1d8 points if damage per caster level (maximum 10d8). A creature cannot benefit or suffer more than once from a single casting of this spell."),
        full_text: true,
    },
    SpellListEntry {
        key: "Planar Adaptation",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(4),
        description: Some("Planar adaptation grants you immunity to the harmful environmental effects of a particular plane of existence, including such hazards as toxicity, extreme temperatures, and lack of air. Additionally, you gain energy resistance 20 to a single energy type prevalent on that plane (choose one if more than one type is equally prevalent). Planar adaptation has no effect when cast upon your native plane."),
        full_text: true,
    },
    SpellListEntry {
        key: "Planar Adaptation (Mass)",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(6),
        description: Some("This spell functions like planar adaptation, except as noted above."),
        full_text: true,
    },
    SpellListEntry {
        key: "Pox Pustules",
        school: Some(Pf1SchoolId::Necromancy),
        level: Some(2),
        description: Some("You inflict a painful, itching rash on the target creature. The target is sickened and takes a -4 penalty to Dexterity (this penalty cannot lower Dexterity below 0). The target can spend a move action scratching furiously at the rash to remove the sickened condition (but not the Dexterity penalty) until the start of its next turn."),
        full_text: true,
    },
    SpellListEntry {
        key: "Protective Spirit",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(2),
        description: Some("A protective spirit is an invisible, mindless, shapeless force that hovers about you, defending you against unexpected attacks. When a creature makes an attack of opportunity against you, the protective spirit makes an immediate attack using your base attack bonus plus your Dexterity modifier. On a successful hit, the spirit does no damage, but it causes the attack of opportunity to automatically miss. A protective spirit can defend against a number of attacks of opportunity equal to your Dexterity bonus (minimum 1) per round.  A protective spirit occupies your space, and moves with you. It cannot be attacked or harmed by physical attacks, but dispel magic, disintegrate, a sphere of annihilation, or a rod of cancellation affects it. A protective spirit's AC against touch attacks is 10 + your Dexterity modifier."),
        full_text: true,
    },
    SpellListEntry {
        key: "Purging Finale",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(3),
        description: Some("You must have a bardic performance in effect to cast this spell. With a flourish, you immediately end your bardic performance, removing one of the following conditions on a creature within range affected by your bardic performance: cowering, dazzled, exhausted, paralyzed, shaken, or stunned."),
        full_text: true,
    },
    SpellListEntry {
        key: "Purified Calling",
        school: Some(Pf1SchoolId::Conjuration),
        level: None,
        description: Some("Your eidolon is restored to full health when you summon it. Upon casting this spell, you must immediately begin the ritual to summon your eidolon. Upon completion of the ritual, your eidolon appears at full hit points and without any damage or penalties to its ability scores, regardless of its previous condition. In addition, any temporary negative conditions affecting your eidolon are immediately removed.  Permanent conditions and ability drain are not affected by this spell."),
        full_text: true,
    },
    SpellListEntry {
        key: "Putrefy Food and Drink",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(0),
        description: Some("This spell causes otherwise edible food to rot and spoil instantly, and water and other liquids to become brackish and undrinkable. Holy water and similar food and drink of significance are spoiled by putrefy food and drink, but the spell has no effect on creatures of any type, nor upon unholy water. Water weighs about 8 pounds per gallon. One cubic foot of water contains roughly 8 gallons and weighs about 60 pounds.  Although potions and elixirs are unaffected by the normal use of the spell, you can instead choose to target a single such object with this spell, destroying it if it fails a saving throw."),
        full_text: true,
    },
    SpellListEntry {
        key: "Rally Point",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(1),
        description: Some("You create a spot that has the power to briefly hearten any good creature who comes into contact with it. A good creature who enters this square (even if simply as part of its normal move) gains a +2 morale bonus on attacks, saving throws, and 2 temporary hit points per caster level for 1 round. Nongood creatures gain no benefit from this spell.  A creature cannot benefit more than once from the same casting of this spell."),
        full_text: true,
    },
    SpellListEntry {
        key: "Rampart",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(7),
        description: Some("You create a massive rampart of hard-packed earth and stone 5 feet thick. The rampart cannot be conjured so that it occupies the same space as another creature or object. Each 5-foot-wide section of the rampart has hardness 0 and 180 hit points. A section of the rampart whose hit points drop to 0 is breached. If a creature tries to break through the rampart with a single attack, the DC for the Strength check is 60. A creature can climb over the rampart with a DC 20 Climb check."),
        full_text: true,
    },
    SpellListEntry {
        key: "Rebuke",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(4),
        description: Some("Your wrathful words cause physical harm to your enemies.  Your enemies take 1d8 points of damage per two caster levels (maximum 5d8) and are staggered for 1 round. Half of this damage is sonic damage, but the other half results directly from divine power and is therefore not subject to being reduced by resistance to sonic-based attacks. Rebuke is especially devastating to foes who worship your god, inflicting 1d6 points of damage per caster level (maximum 10d6) and stunning them for 1d4 rounds. A successful Fortitude save halves the damage and negates the staggering or stunning effect.  Enemies do not need to hear a rebuke to be harmed by it, but it is negated by magical silence."),
        full_text: true,
    },
    SpellListEntry {
        key: "Rejuvenate Eidolon",
        school: Some(Pf1SchoolId::Conjuration),
        level: None,
        description: Some("This spell functions as lesser rejuvenate eidolon, except that it cures 3d10 points of damage +1 point per caster level (maximum +10)."),
        full_text: true,
    },
    SpellListEntry {
        key: "Rejuvenate Eidolon (Greater)",
        school: Some(Pf1SchoolId::Conjuration),
        level: None,
        description: Some("This spell functions as lesser rejuvenate eidolon, except that it cures 5d10 points of damage +1 point per caster level (maximum +20)."),
        full_text: true,
    },
    SpellListEntry {
        key: "Rejuvenate Eidolon (Lesser)",
        school: Some(Pf1SchoolId::Conjuration),
        level: None,
        description: Some("By laying your hand upon an eidolon, you cause its wounds to close and its form to solidify. This spell cures 1d10 points of damage +1 point per caster level (maximum +5)."),
        full_text: true,
    },
    SpellListEntry {
        key: "Residual Tracking",
        school: Some(Pf1SchoolId::Divination),
        level: Some(1),
        description: Some("By touching a footprint, you receive a clear mental image of the person or creature that made it. The image has the exact same appearance as the creature that made the footprint at the moment it made the imprint, including any telling features or expressions, any gear or equipment, or anything else the creature was carrying at the time. For example, the footprint of a horse would reveal both the horse and any creature riding it when it made the footprint."),
        full_text: true,
    },
    SpellListEntry {
        key: "Rest Eternal",
        school: Some(Pf1SchoolId::Necromancy),
        level: Some(4),
        description: Some("You place a curse upon a dead creature that bars its spirit from returning. Anyone casting a spell that would communicate with the dead creature, return it to life, or turn it into an undead creature must succeed on a caster level check with a DC of 11 plus your caster level. Rest eternal cannot be dispelled, but it can be negated with remove curse or break enchantment."),
        full_text: true,
    },
    SpellListEntry {
        key: "Restful Sleep",
        school: Some(Pf1SchoolId::Necromancy),
        level: Some(1),
        description: Some("You cast this spell immediately prior to resting. While under the effects of this spell, you and your allies enjoy a restful night's sleep. If a subject completes a full night's rest, it regains hit points as if it had undergone a full day of bed rest (regaining twice its character level in hit points). If a subject completes a full day's rest, it regains three times its character level in hit points. Any significant interruption during the rest (such as being awoken) prevents any healing that night and ends the effect of this spell on the awakened subject. Any healing acquired while under the effects of restful sleep is considered natural healing, and has no affect on effects requiring magical healing to cure."),
        full_text: true,
    },
    SpellListEntry {
        key: "Resurgent Transformation",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(5),
        description: Some("Resurgent transformation grants you the ability to recover from deadly wounds with restored vitality and a will to fight, but at a price. Once cast, resurgent transformation lies dormant for up to 1 hour per level until you are reduced to one-quarter hit points or less. Once triggered, you immediately gain a +4 enhancement bonus to Constitution and Strength, damage reduction 5/-, and the benefits of a haste spell. In addition, you heal 4d8 points of damage + 1 point per caster level (maximum +25). Resurgent transformation can even save you from death by healing the damage from an otherwise mortal wound, though it does not prevent death from massive damage or from causes other than hit point loss. Your mental faculties are impaired by this magical transformation, however, causing 1d4 points of Intelligence and Wisdom damage.  The advantages from your transformation persist for 1 round per caster level after the spell is triggered. Once the spell ends, it takes a heavy toll, leaving you exhausted and inflicting 1d4 points of Constitution damage. Utilizing more than one resurgent transformation in a day is particularly risky.  The expiration of a second resurgent transformation in a 24- hour period inflicts an additional 1d4 points of Constitution damage and results in death from system shock unless you succeed at a DC 15 Fortitude save, made after the Constitution damage has been applied.  If the spell expires or is dispelled before the transformation has been triggered, no adverse effects result."),
        full_text: true,
    },
    SpellListEntry {
        key: "Retribution",
        school: Some(Pf1SchoolId::Necromancy),
        level: Some(3),
        description: Some("Retribution blasts those who have the temerity to assault your person. You may target anyone who has just struck you with an attack within the previous round, whether with a melee, ranged, or natural weapon, or a spell that requires an attack roll. The target is wracked with shooting pains that impose a -4 penalty on attack rolls, skills checks, and ability checks. On a successful Fortitude save, the penalty only lasts for 1 round.  A target who worships the same god as you takes a -2 penalty on its saving throw."),
        full_text: true,
    },
    SpellListEntry {
        key: "Reviving Finale",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(3),
        description: Some("You must have a bardic performance in effect to cast this spell.  With a flourish, you immediately end your bardic performance, and allies within the area of the spell regain 2d6 hit points. This spell has no effect on undead creatures."),
        full_text: true,
    },
    SpellListEntry {
        key: "Righteous Vigor",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(2),
        description: Some("Infusing the target with a surge of furious divine energy, you enhance a creature's ability to hit an opponent based on the number of times it has already hit that opponent with a successful attack. Each time the subject successfully strikes an opponent with a successful melee attack, the subject gains a cumulative +1 morale bonus on attack rolls (maximum +4 bonus) and gains 1d8 temporary hit points (to a maximum of 20 temporary hit points). If an attack misses, the attack bonus resets to +0 but any accumulated temporary hit points remain. The temporary hit points disappear at the end of the spell's duration."),
        full_text: true,
    },
    SpellListEntry {
        key: "River of Wind",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(4),
        description: Some("Summoning up the power of the tempest, you direct a current of forceful winds where you please. This spell creates a 5-foot-diameter line of wind-the direction of the wind is away from your location when you cast the spell, and remains constant in that direction for the spell duration. Creatures caught in a river of wind take 4d6 nonlethal damage and are knocked prone. A successful Fortitude save halves the damage and prevents being knocked prone.  A creature that begins its turn wholly or partially within a river of wind must make a Fortitude save or be pushed 20 feet in the wind's direction of flow, take 2d6 nonlethal damage, and be knocked prone-a successful Fortitude save means the creature merely takes 1d6 nonlethal damage. Creatures under the effect of freedom of movement and creatures with the air subtype are unaffected by a river of wind."),
        full_text: true,
    },
    SpellListEntry {
        key: "Sacred Bond",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(2),
        description: Some("To use this spell, you first touch the intended recipient, creating a sympathetic field of healing energies between you.  Once the spell has been cast, you and the target may cast conjuration (healing) spells with a range of touch upon each other so long as you are within close range (25 ft. + 5 ft./2 levels). Should either you or the target remove your bracelet, the spell immediately ends."),
        full_text: true,
    },
    SpellListEntry {
        key: "Sacrificial Oath",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(4),
        description: Some("You create a powerful bond between yourself and a single creature. Until the end of the spell's duration, each time the target is hit with an attack or fails a saving throw, you can take the full damage of that attack and any other effects that creature suffers. If you choose not to take on the damage and effects, you instead take a number of points of damage equal to your Constitution score as backlash. Any resistances or immunities you have are applied normally, but you cannot otherwise reduce or negate the damage or effects of either the transfer or the backlash. If you or the subject of the spell move out of line of sight, the spell ends."),
        full_text: true,
    },
    SpellListEntry {
        key: "Saddle Surge",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(2),
        description: Some("You and your mount form a perfect synergy that endows both of you with advantages based upon how far you travel each round. For every 5 feet your mount moves in a given round, you gain a +1 competence bonus on Ride checks and both you and your mount gain a +1 morale bonus on damage rolls made with weapons or natural attacks for 1 round. For instance, if your mount traveled 40 feet in a round, you would gain a +8 bonus on Ride checks and you and your mount would both gain a +8 bonus on damage rolls for 1 round, to a maximum bonus equal to your caster level. You must be mounted to enjoy the benefits of this spell. If you dismount, get knocked off, or take any other action that separates you from your mount, the spell immediately ends."),
        full_text: true,
    },
    SpellListEntry {
        key: "Sanctify Armor",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(3),
        description: Some("You imbue your armor with a righteous aura. It gains a +1 enhancement bonus per four caster levels (maximum +5 at 20th level). When using your judgment or smite ability, you gain DR 5/evil.  An outfit of regular clothing counts as armor that grants no AC bonus for the purpose of this spell."),
        full_text: true,
    },
    SpellListEntry {
        key: "Saving Finale",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(1),
        description: Some("You must have a bardic performance in effect to cast this spell. With a flourish, you can immediately end your bardic performance when a creature within range affected by your bardic performance fails a saving throw, allowing the subject to immediately reroll the failed saving throw."),
        full_text: true,
    },
    SpellListEntry {
        key: "Scent Trail",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(2),
        description: Some("You, or a willing creature you touch, lay down a trail of scents that only the creatures you designate at the time of casting can detect. These creatures find it very easy to follow this trail.  They gain a +20 competence bonus on any Survival checks made for the purpose of tracking the creature laying this trail.  Subject creatures with the scent special ability gain a +10 competence bonus on Wisdom or Survival checks to follow the trail.  In addition, you can leave scent messages along the trail.  Each word or emotion in this message requires spending a move action in the same spot. Subject creatures seeking to understand this message must make another DC 20 Survival check, with a -1 penalty for every word or emotion in the message, in order to understand it. Creatures cannot benefit from this smell if they cannot use, or do not have, a sense of smell. Creatures can use this spell to backtrack through a dungeon, maze, or similar areas even if the creature laying down the trail crossed over the spot multiple times."),
        full_text: true,
    },
    SpellListEntry {
        key: "Screech",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(3),
        description: Some("You emit a shrill, piercing shriek, startling those around you into dropping their guard. Enemies in the area must make a successful saving throw or immediately provoke attacks of opportunity from foes that threaten them. You and your allies are unaffected by your own screech."),
        full_text: true,
    },
    SpellListEntry {
        key: "Sculpt Corpse",
        school: Some(Pf1SchoolId::Necromancy),
        level: Some(1),
        description: Some("With a clammy caress and a word to the powers lurking beyond death, you can reshape a dead body look like another creature or even a specific person so long as you have firsthand knowledge of how that creature or person actually looks.  You can make the corpse one size larger or smaller and change anything else about it including its apparent type, gender, or age. Creatures with a reason to suspect deception may make a Will saving throw to realize that the corpse was altered. If you chose to make the corpse look like a specific individual anyone who knows that individual can make a Will save to realize that the corpse is not actually that person. However, if a creature fails its Will save by 5 or less they believe the corpse is that of someone who closely resembled the person they knew rather than a deliberate fake. This spell merely changes the appearance of the corpse. Any spell or effect that targets the corpse (such as speak with dead or raise dead) treats it as if it still had its original appearance."),
        full_text: true,
    },
    SpellListEntry {
        key: "Seamantle",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(8),
        description: Some("You sheathe yourself within a churning column of pure elemental water up to 30 feet high that fills your space. You gain a swim speed equal to your land speed and can see, hear, and breathe normally within the seamantle, but attacks against you are treated as if you were under the surface of the water. You gain improved cover (+8 cover bonus to AC, +4 bonus on Reflex saves) against foes that do not have freedom of movement effects. The cover granted by the seamantle does not enable you to make Stealth checks or prevent attacks of opportunity. Magical attacks against you are unaffected unless they require attack rolls or state that they do not function underwater (such as cloudkill).  The seamantle blocks line of effect for any fire spell or supernatural fire effect, but enemies can attempt to use fire spells within the seamantle; this requires a caster level check (DC 20 + spell level), and if successful the fire spell takes effect as a bubble of steam contained within the seamantle rather than its usual effect.  The seamantle allows you to make a slam attack by forming a pseudopod of water, inflicting damage appropriate for your size. This slam attack has a reach of 30 feet. In addition, as a standard action, you can attempt to extinguish fires by touch.  You automatically extinguish up to a 10-foot cube of normal fire. Against magical fire effects, your touch acts as dispel magic; this also applies to any non-instantaneous fire affect that comes into contact with you (such as flame blade, flaming sphere, or incendiary cloud). Even if you fail to extinguish a fire, you are not harmed by it. A flaming or flaming burst weapon that strikes you has its power suppressed for 1d4 rounds if the wielder fails a Fortitude save."),
        full_text: true,
    },
    SpellListEntry {
        key: "Seek Thoughts",
        school: Some(Pf1SchoolId::Divination),
        level: Some(3),
        description: Some("Similar to detect thoughts, seek thoughts allows you to sift through the surface thoughts of those around you. You may scan for either the answer to a simple question (such as \"Where is the hidden lair of the wererats?\") or for information on a general topic (such as the beliefs of an evil cult). You detect the number of creatures who are thinking about this question or topic within range, as well as their location if they are visible to you. Seek thoughts does not let you read actual surface thoughts, only if a given creature is thinking about the topic you are concentrating on. A successful Will save prevents you from sensing a creature's thoughts for the duration of the spell.  You can maintain concentration on seek thoughts while you engage in normal conversation, allowing you to ask leading questions about topics of interest. A creature conversing with you while you concentrate can notice that you are distracted with a successful DC 25 Sense Motive check."),
        full_text: true,
    },
    SpellListEntry {
        key: "Shadow Projection",
        school: Some(Pf1SchoolId::Necromancy),
        level: Some(4),
        description: Some("With this spell, you infuse your life force and psyche into your shadow, giving it independent life and movement as if it were an undead shadow (see Pathfinder RPG Bestiary 245). Your physical body lies comatose while you are projecting your shadow, and your body has no shadow or reflection while the spell is in effect.  While projecting your shadow, you gain a shadow's darkvision, defensive abilities, fly speed, racial stealth modifier, and strength damage attack. You do not gain the creature's create spawn ability, nor its skill ranks or Hit Dice.  Your shadow has Hit Dice and hit points equal to your own.  Your shadow projection has the undead type and may be turned or affected as undead.  If your shadow projection is slain, you return to your physical body and are immediately reduced to -1 hit points.  Your condition becomes dying, and you must begin making Constitution checks to stabilize."),
        full_text: true,
    },
    SpellListEntry {
        key: "Share Language",
        school: Some(Pf1SchoolId::Divination),
        level: Some(1),
        description: Some("You can share your facility for one particular language with another creature. For 24 hours the target can read, understand, and communicate to the best of its ability in any one language which you already know. For every 5 levels you possess, you can grant the use of another language you know, to a maximum of 5 languages at 20th level. The target must have the physical capacity to articulate sounds, make gestures, or engage in whatever other method speakers of the language use to communicate with each other in order to actually converse. If the target lacks the mental capacity to grasp an actual language it still gains enough knowledge to respond to and carry out even extremely complex commands or suggestions coached in the language (whether written or spoken). However, since this spell does not endow the target with greater reasoning capacity, merely a temporarily enhanced vocabulary, the person offering up instructions to non-sentient creatures must take care to remove any ambiguity or guesswork.  Similarly, this spell does not affect the basic nature of the target, or its disposition toward you or anyone else, so convincing it to actually carry out these instructions could require negotiation, threats, or outright bribery."),
        full_text: true,
    },
    SpellListEntry {
        key: "Share Senses",
        school: Some(Pf1SchoolId::Divination),
        level: Some(3),
        description: Some("Spending a moment in meditation and conjuring an image of the creature to mind, you reshape your link with your familiar, causing it to functions like a scrying sensor. Upon casting this spell you can hear, see, or smell (any one, your choice) what your familiar is experiencing. You gain the benefits of any nonmagical special abilities your familiar has tied to the chosen sense (such as low-light vision or scent), but use your own Perception skill. You don't need line of sight or line of effect to your familiar, but you must have an active empathic link. You may change the sense you are sharing as a standard action. Unlike other scrying spells, share senses does not allow magically or supernaturally enhanced senses to work through it, and you are unable to see in magical or natural darkness unless your familiar possesses the ability to see in such conditions.  The sensory link with your familiar can be detected as though it were a scrying sensor."),
        full_text: true,
    },
    SpellListEntry {
        key: "Shifting Sand",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(3),
        description: Some("You cause an earthen or sandy surface to shift and churn within the area. The shifting sand obliterates tracks and is considered difficult terrain. Acrobatics checks in the area take a penalty equal to your caster level (maximum +10). Creatures entering or beginning their turn in the shifting sand must make a Reflex save each round or become entangled until the beginning of their next turn. If they attempt to move while entangled, they must make a second Reflex save or fall prone. Creatures with the stability racial trait (like dwarves) may apply it as a bonus on their saving throws.  As a move action, you may move the area of shifting sand up to 10 feet in any direction. Creatures that are entangled or prone in the spell's area are carried along with the shifting sand in the same direction if possible. This movement does not provoke attacks of opportunity. Unattended Medium or smaller objects may also be carried along or shallowly buried by the shifting sand."),
        full_text: true,
    },
    SpellListEntry {
        key: "Shared Wrath",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(4),
        description: Some("This spell functions as wrath, except that you grant multiple creatures the bonuses against your designated foe. All subjects of shared wrath receive their bonuses against the same creature."),
        full_text: true,
    },
    SpellListEntry {
        key: "Sift",
        school: Some(Pf1SchoolId::Divination),
        level: Some(0),
        description: Some("You examine an area at range as if you were searching for fine details with the Perception skill. Make a Perception check with a -5 penalty, modified as normal for conditions. No penalty is applied for distance. Apply the result against the DC for any hidden features, such as secret doors, traps, or hidden treasure. You must be able to see the area you are attempting to search, and you only find details that can be perceived with sight or touch. Sift detects only objects and features, not actual creatures."),
        full_text: true,
    },
    SpellListEntry {
        key: "Sirocco",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(6),
        description: Some("A blast of furnace-hot wind blasts downward, inflicting 4d6 fire damage +1 point per caster level to all creatures in the area and knocking them prone. A successful Fortitude save halves the fire damage and negates being knocked prone.  Flying creatures forced into the ground by the powerful downdraft take damage as if they fell unless they make a DC 15 Fly check, in which case they remain at their original altitude.  Any creature that takes damage from a sirocco becomes fatigued (or exhausted, if already fatigued, such as from a previous round of exposure to a sirocco spell). Creatures with the water subtype take a -4 penalty on all saving throws against this spell and take double normal damage."),
        full_text: true,
    },
    SpellListEntry {
        key: "Sleepwalk",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(4),
        description: Some("You compel an unconscious or sleeping creature to rise and move in a half-awake state. The target creature staggers about if led or guided, but remains helpless for all other purposes.  The subject moves at half speed and is limited to a single move action each round. It is not capable of moving at a higher rate of speed or taking actions other than movement except by magical assistance, and automatically fails any Dexterity- or Strength-based skill checks. If the creature takes any damage while sleepwalking it must make a new saving throw or the spell ends and the creature awakes (if it has more than 0 hit points). When the spell ends or is dismissed, the target remains unconscious and must be awoken normally. While sleepwalk allows an unconscious creature to move, it does not awaken the creature, nor does it stabilize or otherwise heal them. A disabled creature that moves about while under the effects of this spell does not start dying again as a result of this movement."),
        full_text: true,
    },
    SpellListEntry {
        key: "Slipstream",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(2),
        description: Some("You create a low-cresting wave of water that carries the target along the surface of water or the ground. When moving across level ground, the target's speed increases by 10 feet.  If going downhill, speed increases by 20 feet instead, but slipstream provides no movement bonus when going uphill.  While swimming, the slipstream increases the target's swim speed by 20 feet-if the target does not have a swim speed, this spell grants a swim speed of 20 ft."),
        full_text: true,
    },
    SpellListEntry {
        key: "Snake Staff",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(5),
        description: Some("With a long hissing whisper, you transform ordinary pieces of wood into various sorts of snakes that immediately attack your foes.  As long as the snakes remain within sight, you can direct their actions telepathically as a free action. You can only apply this spell to wooden objects not in a creature's possession or not part of a larger structure or plant. Each time you cast this spell you can create a number of snakes equal to your caster level. More powerful snakes take up more than one of your available total, as noted below. Snake statistics can be found on page 255 of the Pathfinder Bestiary; details on the advanced and giant simple templates can be found on pages 294-295 of that book.  Venomous Snake: A stick or piece of firewood.  Counts as 1 snake.  Constrictor Snake: A staff or tree branch. Counts as 2 snakes.  Advanced Venomous Snake: A stick or piece of firewood. Counts as 2 snakes.  Advanced Constrictor Snake: A staff or tree branch.  Counts as 3 snakes.  Advanced Giant Venomous Snake: A log or pile of debris. Counts as 4 snakes.  Advanced Giant Constrictor Snake: A fallen tree or a large pile of debris. Counts as 5 snakes."),
        full_text: true,
    },
    SpellListEntry {
        key: "Solid Note",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(1),
        description: Some("You give temporary physical form to a single musical note and can hang it, suspended, wherever you wish within range, allowing you to use it as hook, pulley, door blocker, or anything else your imagination desires. The exact appearance of the solid note depends on your melody. You can make it spiked or smooth, simple or convoluted, and with any color pattern you wish, however, it always has a size approximately equal to that of your outstretched hand. Once created, the solid note resists all attempts to move it but instantly disappears if enough force or weight is brought to bear against it. The note has an effective Strength equal to 10 + your caster level. It cannot hold more weight than the equivalent of a heavy load for its Strength without disappearing. Any creature that wins an opposed Strength check against the note (by pushing open a door which the note is holding shut for example) also causes it to disappear. The note can never deal actual harm to a creature and disappears if successfully attacked (AC 10 + your Charisma modifier) or overcome with a combat maneuver such as bull rush (CMD 2 + your base attack bonus + your Charisma modifier). Any creature obstructed by the solid note simply fails to budge it and loses that action for the round."),
        full_text: true,
    },
    SpellListEntry {
        key: "Spark",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(0),
        description: Some("You can make an unattended Fine flammable object catch on fire. This works as if you were using flint and steel except that you can use spark in any sort of weather and it takes much less time to actually ignite an object."),
        full_text: true,
    },
    SpellListEntry {
        key: "Spiked Pit",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(3),
        description: Some("This spell functions as create pit, except that the pit is lined with wickedly sharp spikes along its bottom and walls and has a maximum depth of 50 feet. Creatures who fall into the pit take falling damage as normal, plus 2d6 points of piercing damage from the spikes. Any creature or object coming into contact with the spikes along the walls, such as a creature trying to climb out, or rope or other typical aids to climbing, takes 1d6 points of piercing damage each round they are in contact with the walls. For those willing to accept the damage incurred while climbing, the pit's walls have a Climb DC of 20."),
        full_text: true,
    },
    SpellListEntry {
        key: "Spiritual Ally",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(4),
        description: Some("An ally made of pure force appears in a single 5-foot square within range. The ally takes the form of a servant of your god.  The spiritual ally occupies its space, though you and your allies can move through it, since it is your ally. The spiritual ally carries a single weapon, one favored by your deity (as for spiritual weapon), which has the same threat range and critical modifiers as a real weapon of its form. Each round on your turn, starting with the turn that you cast this spell, your spiritual ally can make an attack against a foe within its reach that you designate. The spiritual ally threatens adjacent squares and can flank and make attacks of opportunity as if it were a normal creature. The spiritual ally uses your base attack bonus (gaining extra attacks if your base attack bonus is high enough) plus your Wisdom bonus when it makes a melee attack. When the spiritual ally hits, it deals 1d10 points of force damage + 1 point of damage per 3 caster levels (maximum +5 at 15th level). It strikes as a spell, not a weapon, so it bypasses DR and can affect incorporeal creatures.  Each round after the first, you can move the spiritual ally as a swift action. It has a speed of 30 feet, and a fly speed of 30 feet (perfect maneuverability). Being a construct of force, the spiritual ally cannot be harmed by any physical attacks, but dispel magic, disintegrate, a sphere of annihilation, or a rod of cancellation affects it. A spiritual ally's AC against touch attacks is 10.  If an attacked creature has spell resistance, you make a caster level check (1d20 + caster level) against that spell resistance the first time the spiritual ally strikes it. If the ally is successfully resisted, the spell is dispelled. If not, the weapon has its normal full effect on that creature for the duration of the spell."),
        full_text: true,
    },
    SpellListEntry {
        key: "Spite",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(4),
        description: Some("Choose a single touch range spell of 4th level or lower with a casting time of 1 standard action or less. As part of the action of casting spite, you cast the associated spell and bind it into a defensive ward in the form of a tattoo, birthmark, or wart somewhere upon your body. The next time you are hit by a melee attack or a combat maneuver is used successfully against you, the stored spell is triggered against your foe. You do not need to succeed on a touch attack to affect the target, but in all other respects the spell is treated as though you had cast it normally. If the attacking creature is not a valid target for the spell, the stored spell is lost with no effect.  You can have only one spite spell in effect at a time; if you cast this spell a second time, the previous spell effect ends."),
        full_text: true,
    },
    SpellListEntry {
        key: "Stay the Hand",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(4),
        description: Some("You cause a creature's arm to waver and prevent it from striking another creature. You can cast this spell when the target is about to make a melee attack against another creature. On a failed saving throw, the target does not follow through with its attack, and its entire action is wasted for the round. On a successful saving throw, the target can make its attack, forcing its strike though the compulsion but losing both accuracy and power, taking a -5 penalty on its attack and damage rolls. Whether or not the target makes its initial save or not, it is subject to a -2 penalty on attack and damage rolls against the creature it originally targeted for the duration of the spell."),
        full_text: true,
    },
    SpellListEntry {
        key: "Stone Call",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(2),
        description: Some("A rain of dirt, gravel, and small pebbles fills the area, dealing 2d6 points of bludgeoning damage to every creature in the area. This damage only occurs once, when the spell is cast.  For the remaining duration of the spell, this debris covers the ground, making the entire area difficult terrain. At the end of the duration, the rocks disappear, leaving no aftereffects (other than the damage dealt)."),
        full_text: true,
    },
    SpellListEntry {
        key: "Stone Fist",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("This spell transforms your hands into living stone. While this spell is in effect, your unarmed strikes do not provoke attacks of opportunity and deal 1d6 points of lethal bludgeoning damage (1d4 if you are Small). In addition, your unarmed strikes ignore the hardness of any object with a hardness less than 8.  Stone to flesh immediately dispels stone fist. Should you be the target of transmute rock to mud, this spell immediately ends and you take 4d6 points of damage."),
        full_text: true,
    },
    SpellListEntry {
        key: "Stormbolts",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(8),
        description: Some("When you cast this spell, lightning spills forth from your body in all directions. The bolts do not harm natural vegetation or creatures in the area you wish to exclude from damage. Any other creatures within the area take 1d8 points of electricity damage per caster level (maximum 20d8) and are stunned for 1 round. A successful saving throw halves the damage and negates the stun effect."),
        full_text: true,
    },
    SpellListEntry {
        key: "Strong Jaw",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(3),
        description: Some("Laying a hand upon an allied creature's jaw, claws, tentacles, or other natural weapons, you enhance the power of that creature's natural attacks. Each natural attack that creature makes deals damage as if the creature were two sizes larger than it actually is (see page 302 of the Pathfinder Bestiary for more information). If the creature is already Gargantuan or Colossal-sized, double the amount of damage dealt by each of its natural attacks instead. This spell does not actually change the creature's size; all of its statistics except the amount of damage dealt by its natural attacks remain unchanged."),
        full_text: true,
    },
    SpellListEntry {
        key: "Stumble Gap",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(1),
        description: Some("You create a shallow extradimensional hole perfectly sized to trip anyone who steps within it. This hole occupies a single 5-foot square with a depth of six inches. Any creature occupying the square when you first create the hole, or who later steps into the square containing the hole, must make a Reflex saving throw to avoid falling prone in an adjacent square and taking 1d6 points of damage. A creature that makes its saving throw still stumbles ever so slightly and takes a -1 penalty on all rolls and checks for 1 round. The spell has no effect on creatures adjacent to the square containing the hole."),
        full_text: true,
    },
    SpellListEntry {
        key: "Stunning Finale",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(5),
        description: Some("You must have a bardic performance in effect to cast this spell.  With a flourish, you immediately end your bardic performance, assaulting the senses of the targets with your finale. Each target is stunned for 1 round. On a successful saving throw, a target is staggered for 1 round."),
        full_text: true,
    },
    SpellListEntry {
        key: "Suffocation",
        school: Some(Pf1SchoolId::Necromancy),
        level: Some(5),
        description: Some("This spell extracts the air from the target's lungs, causing swift suffocation. The target can attempt to resist this spell's effects with a Fortitude save-if he succeeds, he is merely staggered for 1 round as he gasps for breath. If the target fails, he immediately begins to suffocate. On the target's next turn, he falls unconscious and is reduced to 0 hit points. One round later, the target drops to -1 hit points and is dying. One round after that, the target dies. Each round, the target can delay that round's effects from occurring by making a successful Fortitude save, but the spell effect continues for 3 rounds, and each time a target fails his Fortitude save, he moves one step further along the track to suffocation. This spell only affects living creatures that must breathe. It is impossible to defeat the effects of this spell by simply holding one's breath-if the victim fails the initial saving throw, the air in his lungs is extracted."),
        full_text: true,
    },
    SpellListEntry {
        key: "Suffocation (Mass)",
        school: Some(Pf1SchoolId::Necromancy),
        level: Some(9),
        description: Some("This spell functions as suffocation except as noted above. Note that the duration of this spell is much longer, forcing those suffering from the effect to make far more Fortitude saves to stave off eventual suffocation."),
        full_text: true,
    },
    SpellListEntry {
        key: "Summon Eidolon",
        school: Some(Pf1SchoolId::Conjuration),
        level: None,
        description: Some("You open a rift between dimensions that summons your eidolon.  Treat this as if you had summoned your eidolon normally, except that it only remains with you for the duration of this spell. While summoned in this way, your eidolon cannot touch any creature warded by protection from evil or a similar effect and your eidolon can be sent back to its home plane by dispel magic.  If you cast this spell while your eidolon is already on your plane, this spell has no effect. This spell allows you to summon your eidolon even if it has been returned to its home plane due to damage."),
        full_text: true,
    },
    SpellListEntry {
        key: "Swarm Skin",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(6),
        description: Some("You convert all of your soft tissue into swarms of insects or other creatures and send them off to do your bidding, leaving nothing but your bones and gear behind. These swarms have all their normal qualities and abilities but also bear a tiny portion of your consciousness that allows you to perceive what they do and to control their actions. While in this form, you cannot use any of your own abilities or take any actions other than controlling the swarms.  Each time you cast this spell you get a total allotment of levels equal to your caster level and any swarm you choose to create costs one or more of those levels. You can \"spend\" your allotment of levels to create any combination of swarms so long as their total does not exceed your caster level: 2 caster levels Spider swarm (Pathfinder Bestiary page 258), 4 caster levels Rat swarm (Bestiary 232), 6 caster levels Crab swarm, wasp swarm (Bestiary 50, 275), 8 caster levels Centipede swarm, leech swarm (Bestiary 43, 187), 10 caster levels Army ant swarm (Bestiary 16).  Once you create these swarms they remain in existence until destroyed or you order them to return to your body. When all swarms have returned to your body or are destroyed, your flesh wraps itself back around your bones and you regain the ability to act normally. If your bones are not where you left them you must first locate them in order to regain access to your body. You always know if your bones are destroyed and your consciousness remains in control of the swarms until they too are destroyed (and your die). If you can use the swarms to get help and arrange for someone to repair your bones (using anything that would normally restore you to life), at which point you can return the swarms to your body."),
        full_text: true,
    },
    SpellListEntry {
        key: "Thorn Body",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(3),
        description: Some("This spell causes spines to grow from your exposed skin, damaging creatures that strike you. Any creature striking you with a melee weapon, an unarmed strike, or a natural weapon takes 1d6 points of piercing damage +1 point per caster level (maximum +15). Creatures using melee weapons with reach are unaffected by this spell. Creatures that successfully grapple you take 2d6 points of piercing damage +1 point per caster level (maximum +15). In addition, your natural attacks and unarmed strikes deal an additional 1d6 points of piercing damage.  Thorns created by this spell persist through any new physical shape or form you assume, such as via wildshape or any polymorph effect."),
        full_text: true,
    },
    SpellListEntry {
        key: "Threefold Aspect",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(4),
        description: Some("Threefold aspect allows you to shift your appearance between your natural age and three idealized age categories: young adult (youth/maiden), adulthood (father/mother), or elderly (elder/crone). In each case, your appearance is your own at the appropriate age, rather than that of a new individual.  You may change between these three aspects or your actual age as a standard action. As the young adult, you gain a +2 enhancement bonus to Dexterity and Constitution, but suffer a -2 penalty to Wisdom. In the adult aspect, you gain a +2 enhancement bonus to Wisdom and Intelligence, but take a -2 penalty to Dexterity. As the elderly aspect, you gain a +4 enhancement bonus to Wisdom and Intelligence, but take a -2 penalty to Strength and Dexterity. As enhancement bonuses, these stack with any bonuses or penalties you may have from your actual age (which are untyped bonuses)-the bonuses granted by this spell represent your idealized form in this threefold aspect rather than simply duplicating your ability scores at any one particular age.  True seeing reveals your natural appearance overlaid with that your aspect, recognizing both as part of your true self.  Individuals who study you closely and have interacted with you at another apparent age recognize a resemblance (as though family) with a successful DC 20 Perception check.  Threefold aspect does not alter your clothing or equipment, and does not heal any deformity or injury unrelated to age."),
        full_text: true,
    },
    SpellListEntry {
        key: "Thundering Drums",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(3),
        description: Some("You strike the ground in front of you, filling the area in front of you with the thunder of pounding drums. Any creature in the area takes 1d8 points of sonic damage per caster level (maximum 5d8) and is knocked prone. A successful Fortitude save halves the damage and negates being knocked prone."),
        full_text: true,
    },
    SpellListEntry {
        key: "Timely Inspiration",
        school: Some(Pf1SchoolId::Divination),
        level: Some(1),
        description: Some("A word of arcane-empowered inspiration can snatch victory from seeming defeat. Cast this spell when a creature fails an attack roll or skill check. The target gains a +1 competence bonus per five caster levels (maximum +3 bonus) on the attack roll or skill check retroactively. If the bonus is enough to make the failure a success, the roll succeeds."),
        full_text: true,
    },
    SpellListEntry {
        key: "Tireless Pursuers",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(3),
        description: Some("This spell functions as tireless pursuit, except as noted above."),
        full_text: true,
    },
    SpellListEntry {
        key: "Tireless Pursuit",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("You harden your body against the stresses of long travel. You halve the damage caused by hustling and forced marching (see Pathfinder RPG Core Rulebook 171). In addition, for the duration of the spell you ignore any fatigue caused by such travel. Once the spell ends, if you still have any nonlethal damage caused by the hustling or forced march, you become fatigued, or exhausted if already fatigued."),
        full_text: true,
    },
    SpellListEntry {
        key: "Touch of Gracelessness",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("With a single touch, you reduce a creature to a fumbling clown.  The target takes a penalty to its Dexterity equal to 1d6+1 per two caster levels (maximum 1d6+5). This penalty cannot drop the target's Dexterity score below 1.  In addition, if the subject moves more than half its speed, it falls prone. If the subject flies, its maneuverability is reduced by one step (perfect maneuverability becomes good, good becomes average, and so on).  A successful Fortitude save halves the penalty to Dexterity and negates the possibility of falling prone or the reduction to fly maneuverabilities."),
        full_text: true,
    },
    SpellListEntry {
        key: "Touch of the Sea",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(1),
        description: Some("You cause webs to grow between the target's fingers and its feet to transform into flippers, granting a swim speed of 30 feet along with the standard +8 bonus on Swim checks and the ability to take 10 even if distracted or endangered. You can also use the run action while swimming, provided you swim in a straight line. This transformation causes any boots or gloves the target is wearing to meld into its form (although magic items with a continuous effect continue to function). This spell does not grant the target any ability to breathe water."),
        full_text: true,
    },
    SpellListEntry {
        key: "Transmogrify",
        school: Some(Pf1SchoolId::Transmutation),
        level: None,
        // Corpus-parsing correction (SD-25 criterion 7.N, "apg-spell-text"):
        // `apg_spells.lst:2094` concatenates two `.MOD` stanzas onto one
        // physical line with no line break (`Transmogrify.MOD ...
        // Transmute Potion to Poison.MOD ...`); the prior ingest took the
        // line's *last* `DESC:` token as this record's full text, which
        // actually belongs to the unrelated `Transmute Potion to Poison`
        // record further down the same line. This record's own correctly
        // co-located first `DESC:` token is restored here. `level` stays
        // `None`: the only `CLASSES:`-granting `.MOD` for Transmogrify
        // (`apg_spells.lst:492`) is itself `#`-commented out in the real
        // corpus, a genuine gap, not part of this correction.
        description: Some("Your eidolon's form shifts and transforms. This spell allows you to change any of the eidolon's evolutions by allocating its evolution pool on new evolutions. If you have the aspect or greater aspect ability, this spell also allows you to change the evolution points spent to modify you, including removing or adding points as allowed by those abilities.  Your eidolon cannot benefit from this spell more than once per day. This spell does not allow you to change your eidolon's base form."),
        full_text: true,
    },
    SpellListEntry {
        key: "Transmute Potion to Poison",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(2),
        // Corpus-parsing correction (SD-25 criterion 7.N, "apg-spell-text"):
        // this spell's real full-text `.MOD` record was present all along
        // at `apg_spells.lst:2094`, concatenated onto the same physical
        // line as `Transmogrify.MOD` (see that record's note above) -- the
        // prior ingest attributed it to `Transmogrify` instead. Recovered
        // directly from the corpus, no web fetch needed.
        description: Some("This extract causes you to take on a pale, sickly pallor for 1 round. During this time, if you consume a potion, it has no effect. Instead, your mouth fills with a vile poison that you can spit onto a weapon as a free action. If you do not spit out the poison in 1 round, it affects you instead. The effect of the poison depends on the level of spell contained in the potion consumed (see sidebar). If the poison is not used within 1 minute per caster level it becomes inert. You can only create one dose of poison in this way per casting of this spell.  The following poisons are created through the transmute potion to poison spell. The power of the poison depends on the level of the spell contained in the transmuted potion.  0-Level Potion Poison: injury, Fortitude DC 10, 1/round for 2 rounds, 1 Dex damage, Cure 1 save.  1st-Level Potion Poison: injury, Fortitude DC 11, 1/round for 4 rounds, 1d2 Dex damage, Cure 1 save.  2nd-Level Potion Poison: injury, Fortitude DC 13, 1/round for 6 rounds, 1d4 Str damage, Cure 1 save.  3rd-Level Potion Poison: injury, Fortitude DC 14, 1/round for 6 rounds, 1d4 Con damage, Cure 2 consecutive saves."),
        full_text: true,
    },
    SpellListEntry {
        key: "Treasure Stitching",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(4),
        description: Some("You can transform all objects placed on a cloth into an embroidered, two-dimensional representation, making it easy for you to store and transport the items. You can pile any amount or weight of objects on the cloth so long as the pile stays within the dimensions of the cloth (up to a 10-foot cube).  When you cast the spell the entire pile disappears into the cloth, replaced by a highly accurate, sewn picture of the pile from whatever angle you wish. The cloth retains its normal weight and dimensions. You can restore the pile of objects at any time as a full-round action by unfurling the cloth and ordering the spell to end, consuming the cloth. Anyone else wishing to restore the objects must successfully dispel your spell. If the cloth is destroyed or significantly damaged, all objects held within the cloth are lost. This spell cannot affect artifacts or other sorts of similarly unique objects."),
        full_text: true,
    },
    SpellListEntry {
        key: "True Form",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(4),
        description: Some("True form removes any polymorph effect from a target creature, returning it to its true form, even against its will. Against creatures affected by polymorph spell effects (such as baleful polymorph or giant form), you must make a caster level check (1d20 + caster level, maximum +15) against a DC of 11 + caster level of the effect. Success means that the polymorph effect immediately ends.  A creature with the supernatural ability to change shape (such as a lycanthrope) must make a Will saving throw or immediately revert to its true form. If it fails this first saving throw, the creature can attempt another Will saving throw to overcome the spell as a full-round action that does not provoke attacks of opportunity. If this save succeeds, the spell ends and the creature is capable of changing shape once again. If the second save fails, the creature is locked in its true form for the duration of the spell, preventing any further polymorph effects from changing its shape."),
        full_text: true,
    },
    SpellListEntry {
        key: "Tsunami",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(9),
        description: Some("You create a massive wave of water that then moves in a straight line across water or land-you select the direction the wave travels (which must be perpendicular to its width), but once set in motion, the wave cannot change course. Over the surface of open water, the wave travels at a speed of 60 feet per round-on land or underwater, the wave travels at a speed of 30 feet per round.  Creatures struck by a tsunami take 8d6 points of bludgeoning damage (a Fortitude save halves this damage).  In addition, the tsunami makes a special CMB check against any creature it strikes-the wave's CMB is equal to your caster level + your relevant spellcasting ability score modifier (whichever is highest) + 8 (for the wave's size). If this CMB check defeats a creature's CMD, the creature is knocked prone and carried along by the wave. A creature carried along by a tsunami can attempt to escape the wave on its turn as a standard action by making a CMB or Swim check opposed by the wave's CMB check-if a creature fails to escape, it takes another 6d6 points of bludgeoning damage (Fortitude save for half ) and continues being carried along by the wave.  Objects struck by a tsunami are swept up if they are Huge or smaller and are carried along by the wave, deposited in a pile at the end of the wave's journey. Gargantuan or larger objects, as well as structures or objects firmly attached to the ground, take 8d6 points of bludgeoning damage when a tsunami passes through its space-if this is enough to destroy the object or structure, the remains are carried along by the wave. Hardness does not reduce this damage, nor is it halved as damage dealt to objects normally is. Freedom of movement prevents a creature from being carried along by a tsunami but does not prevent damage caused by it hitting a creature.  A solid barrier that is taller than the tsunami that is not destroyed by the wave stops that portion of the wave from continuing onward, leaving a gap in the wave as the rest of it continues forward."),
        full_text: true,
    },
    SpellListEntry {
        key: "Twilight Knife",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(3),
        description: Some("You create a darkly sinister floating knife that attacks the same creature as you each round. If you choose not to attack a creature, or you make an attack that affects multiple opponents, the knife makes no attack during that round.  The knife uses your base attack bonus modified by either your Intelligence or Charisma bonus (whichever is higher) when making this attack. Unless you specifically will it to do otherwise, or it proves impossible to do so, the knife always maneuvers itself so that it can flank your opponent before making the attack. The knife deals 1d4 points of force damage on a successful hit and has the same threat range and critical multipliers as a normal dagger. In addition, if the target is denied a Dexterity bonus to AC or the knife flanks the target, the knife can make sneak attacks as a rogue, inflicting an extra 1d6 points of force damage per four caster levels on a successful attack.  A twilight knife cannot be attacked or harmed by physical attacks, but dispel magic, disintegrate, a sphere of annihilation, or a rod of cancellation affects it. A twilight knife's AC against touch attacks is 12 (10 + size bonus for Tiny object) plus your Dexterity modifier.  If an attacked creature has spell resistance, you make a caster level check (1d20 + caster level) against that spell resistance the first time the twilight knife strikes it. If the knife is successfully resisted, the spell is dispelled. If not, the weapon has its normal full effect on that creature for the duration of the spell."),
        full_text: true,
    },
    SpellListEntry {
        key: "Twin Form",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(6),
        description: Some("This extract splits a perfect double of yourself from your body, dressed and equipped exactly as you are. You are able to shift your consciousness from one body to the other once each round as a free action. This shift takes place either immediately before your turn or immediately after it, but not during the round. You may act normally in the body you inhabit. Your other self is treated as though dazed, except it may take a single move action each round during your turn. Your twin cannot speak while you are in your other body, and cannot flank, make attacks of opportunity, or otherwise threaten enemies.  Both you and your twin have the same statistics and start with the number of hit points you had when you ingested the extract. Once you have split, these hit points are tracked separately. Any spells, extracts, or magical effects (such as from potions) that were active when you ingested the extract are active for both you and your twin. If any such effects expire, are dispelled, dismissed, or otherwise used or ended, they end for both of you. Extracts or spells cast after you split affect you and your twin as though you were two separate targets. Your equipment is linked between your two selves, and if an item on one is consumed or destroyed, its duplicate is used up or destroyed as well.  The body you do not inhabit crumbles into dust when the extract's duration expires or is dismissed. If the body you inhabit is destroyed, you immediately shift to your surviving self and the extract immediately ends. The body you left behind crumbles into dust, and you are stunned until the start of your next turn. If the body you do not inhabit is destroyed, the extract also ends immediately, but you suffer no ill effects.  You have no special ability to sense what your second body is experiencing, though you immediately know if it has been destroyed. You may switch between bodies at any distance on the same plane. If your bodies cross into separate planes (including through the use of teleport or blink), the body you inhabit survives, while your other body is destroyed."),
        full_text: true,
    },
    SpellListEntry {
        key: "Unfetter",
        school: Some(Pf1SchoolId::Transmutation),
        level: None,
        description: Some("This spell breaks the life link between you and your eidolon. This spell allows your eidolon to venture more than 100 feet away from you without penalty. It can travel any distance while this spell is in effect without penalty, but if the spell expires while the eidolon is farther than 100 feet away, it immediately loses hit points as normal for distance and is possibly sent back to its home plane. While this spell is in effect, you cannot sacrifice hit points to prevent damage to your eidolon. Damage that would be transferred due to the life bond ability is not transferred. If you attempt to use the transposition ability while this spell is in effect, you must roll on the teleport mishap table, using the \"studied carefully\" row."),
        full_text: true,
    },
    SpellListEntry {
        key: "Universal Formula",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(4),
        description: Some("As you ingest this extract, it transforms into the appropriate extract for any formula you know of 3rd level or lower and takes effect immediately. If the chosen formula has an expensive material component, it must be provided along with the component for this formula. You may not create an infused extract with this formula."),
        full_text: true,
    },
    SpellListEntry {
        key: "Unwilling Shield",
        school: Some(Pf1SchoolId::Necromancy),
        level: Some(5),
        description: Some("Like shield other, unwilling shield creates a mystic connection between you and the target, but unlike shield other, the target shares the wounds that you receive. In addition, the link draws upon the target's life force to supplement your own defenses. You gain a +1 luck bonus to AC and on saving throws. You take only half damage from all wounds and attacks (including those dealt by special abilities) that deal hit point damage. The amount of damage not taken by you is taken by the target. Forms of harm that do not involve hit points, such as charm effects, ability damage, level draining, and death effects are not affected. If you take a reduction in hit points from a lowered Constitution score, the reduction is not split with the target because it is not hit point damage.  When the spell ends, subsequent damage is no longer divided between you and the subject, but damage already split is not assigned to you.  If you and the target of the spell move out of range of each other, the spell remains active, but damage is no longer shared until you are once again within range of each other."),
        full_text: true,
    },
    SpellListEntry {
        key: "Unwitting Ally",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(0),
        description: Some("You befuddle the target's mind. The target has difficulty telling friend from foe for a short period of time. The subject is considered your ally and not your enemies' ally while determining flanking. The subject takes no other hostile action against your enemies due to this spell's effect."),
        full_text: true,
    },
    SpellListEntry {
        key: "Vanish",
        school: Some(Pf1SchoolId::Illusion),
        level: Some(1),
        description: Some("This spell functions like invisibility, except the effect only lasts for 1 round per caster level (maximum of 5 rounds). Like invisibility, the spell immediately ends if the subject attacks any creature."),
        full_text: true,
    },
    SpellListEntry {
        key: "Veil of Positive Energy",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(1),
        description: Some("You surround yourself with a veil of positive energy, making it harder for undead creatures to harm you. When under the effect of this spell, you gain a +2 sacred bonus to AC and a+2 sacred bonus on saves. Both of these bonuses apply only against attacks or effects created by undead creatures. You can dismiss this spell before its normal duration as a swift action on your turn to deal a number of points of positive energy damage equal to your level to all undead creatures within 5 feet of you."),
        full_text: true,
    },
    SpellListEntry {
        key: "Venomous Bolt",
        school: Some(Pf1SchoolId::Necromancy),
        level: Some(3),
        description: Some("You infuse a single arrow or crossbow bolt with natural venom as you fire it. In addition to its normal damage, anyone struck by this arrow or bolt is affected as if by the poison spell. If the arrow is not fired immediately, the spell ends with no effect."),
        full_text: true,
    },
    SpellListEntry {
        key: "Versatile Weapon",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(2),
        description: Some("You transform the physical makeup of a weapon as you desire.  This spell functions like greater magic weapon, except that it subtly alters the physical properties of a weapon, enabling it to bypass damage reduction of one the following types: bludgeoning, cold iron, piercing, silver, or slashing. The affected weapon still inflicts damage of its normal type and its hardness and hit points are unchanged. This spell can be cast on a natural weapon or unarmed strike."),
        full_text: true,
    },
    SpellListEntry {
        key: "Vomit Swarm",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(2),
        description: Some("You vomit forth a swarm of spiders (Bestiary, page 258) that attacks all other creatures within its area. The swarm begins adjacent to you, but if no living creatures are within its area, it moves in one direction of your choosing at its normal speed.  You can move the swarm or change the swarm's direction by spending a standard action to concentrate on the swarm, otherwise it continues moving in its current direction. If your caster level is at least 7th, you can vomit forth a swarm of wasps instead (Bestiary 275). Finally, if your caster level is at least 13th, you can vomit forth a swarm of army ants (Bestiary 16)."),
        full_text: true,
    },
    SpellListEntry {
        key: "Vortex",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(7),
        description: Some("You create a powerful and immobile whirlpool in any body of liquid large enough to contain the spell's effect. Any Large or smaller creature that comes in contact with the spell effect must succeed on a Reflex save or take 3d6 points of bludgeoning damage. A Medium or smaller creature that fails its first save must succeed on a second one or be pulled into the whirlpool and held suspended in its powerful currents, taking 1d8 points of damage each round on your turn with no save allowed.  You may direct the whirlpool to eject any carried creatures whenever you wish. A boat that is equal in length or shorter than the vortex's width that passes through a vortex takes 6d6 points of damage and is caught up by the current. If the boat's captain makes a DC 25 Profession (sailor) check (or if the boat is longer than the vortex's width), the boat takes only half damage and is not caught up by the vortex."),
        full_text: true,
    },
    SpellListEntry {
        key: "Wake of Light",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(2),
        description: Some("You and your mount lay down a trail of glowing, heavenly mist behind you as you move that makes passage easier for good creatures but more difficult for evil creatures. This glowing trail of mist appears behind your mount in a straight line starting where you cast the spell and ending where your mount ends its movement. The mist takes the form of a path 10 feet wide and up to 120 feet long and always floats just above the ground, up to a height of 1 foot. Thereafter, the mist persists for 1 round per level.  Good creatures may walk along the top of the mist as if it were solid, treating squares of difficult terrain containing the mist as normal terrain. Evil creatures find the mist thick and cloying, and treat squares of normal terrain containing the mist as difficult terrain instead. Neutral creatures pass through the mist with no effect. The mist has no effect on obstacles or otherwise impassable terrain, and does not block sight or provide concealment.  Except for very special circumstances (such as a celestial or fiendish mount), mounts use the alignment of their rider when determining how this spell affects them. You must be mounted to enjoy the benefits of this spell. If you dismount, get knocked off, or take any other action that separates you from your mount, the spell immediately ends, although the mist remains for the spell's normal duration.  A wake of light cannot follow across water, underwater, or through the air in the case of a mount capable of walking on water, swimming, or flight."),
        full_text: true,
    },
    SpellListEntry {
        key: "Wall of Lava",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(8),
        description: Some("This spell creates a vertical wall of lava that is 1 inch thick for every 4 caster levels and composed of up to one 5-foot square per level. A wall of lava's maximum height cannot exceed half of its width (with a minimum height of 5 feet). The wall cannot be conjured so that it occupies the same space as a creature or object. A section of a wall of lava can be destroyed by damage (hardness 4, hp 90), but if a section is destroyed, the remaining lava in the wall immediately fills in any such hole created, reducing the wall's overall size by one 5-foot square but remaining a contiguous barrier. Each time a weapon strikes a wall of lava, it takes 2d6 points of fire damage (or the creature who strikes the wall takes 2d6 fire damage if the attack was via an unarmed strike or natural attack).  A creature can move through a wall of lava as a full-round action by making a DC 25 Strength check-failure indicates that the creature is pushed back out of the wall to the point he just attempted to leave. A creature with a burrow speed can move through the wall using its burrow speed. An attempt to move through a wall of lava inflicts 20d6 fire damage. A wall of lava also radiates heat as if it were a wall of fire, although the heat from a wall of lava radiates from both sides.  Once per round as a move action, you can direct the wall of lava to erupt. This causes a plume of lava to fire at any target within 60 feet of either side of the wall, but reduces the wall's overall size by 1d4 5-foot square sections. You must make a ranged touch attack to hit the target, which takes 10d6 points of fire damage on a hit. Holes created in a wall of lava from this effect instantly reseal, reducing the overall size of the wall.  All damage inflicted by physical contact with a wall of lava continues for 1d3 rounds after exposure ceases, but this additional damage is only half that dealt during actual contact (that is, 1d6 or 5d6 or 10d6 points per round)."),
        full_text: true,
    },
    SpellListEntry {
        key: "Wall of Suppression",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(9),
        description: Some("You create a wall of glittering motes that suppresses or even negates any magical effect passing through it. The wall appears to have no actual substance and does not physically obstruct or impede anything attempting to move through it. However, the wall exerts a powerful anti-magical effect.  Any magic item or magical spell or effect of your caster level or lower that passes through the wall is suppressed for 1 round per level. Spells or effects with durations expire normally, even while thus suppressed. A spell or effect with a duration greater than that of the suppression effect resumes functioning normally when the suppression ends.  The wall affects all magical effects, including spells, spelllike abilities, magical items, and any effects stemming from them that pass through the wall. The wall does not suppress a creature's ability to cast spells, use spell-like abilities, or any other sort of limited-use abilities even if the wall suppresses a particular application of those abilities. However, if a creature with magical abilities that are constant or otherwise always active passes through the wall, those abilities are suppressed for the normal duration.  The wall blocks line of effect, so no spell or effect can pass through the wall, but it does not block line of sight. Magic items or spell effects with a higher caster level than yours are unaffected by the wall of suppression. The wall does not affect artifacts, anything stemming from the direct action of a deity, or similarly powerful sorts of magic."),
        full_text: true,
    },
    SpellListEntry {
        key: "Wandering Star Motes",
        school: Some(Pf1SchoolId::Illusion),
        level: Some(4),
        description: Some("You create sparkling motes of bright light that shoot toward the target and swirl around it in a complex pattern. The pattern clearly outlines the target and radiates light as if it were a sunrod, negating any concealment for the target. The target must make a successful Will save. If the target fails its save, it is dazed for 1 round and must make another save on its next turn or be dazed again for 1 round. The target must continue making Will saves each round. If a target makes its Will save, the wandering star motes jump to the nearest enemy within 30 feet, who must now make Will saves every round or be dazed. Any time a target makes its Will save, the wandering star motes jump to the next nearest enemy within 30 feet.  A given creature can only be affected by the wandering star motes once; once a target has successfully saved against the spell, it cannot be affected again. If there are no new targets within 30 feet of a target that has successfully made its save, the spell immediately ends. The spell only affects enemy creatures; your allies are not affected."),
        full_text: true,
    },
    SpellListEntry {
        key: "Ward the Faithful",
        school: Some(Pf1SchoolId::Abjuration),
        level: Some(3),
        description: Some("All creatures within the area who worship the same god as you gain a +2 deflection bonus to AC and a +2 resistance bonus on all saves. At 12th level, these bonuses increase to +3. At 18th level, the bonuses increase to +4.  If you do not worship a specific deity, the bonuses apply to those who share your faith. If you do not have a specific faith or religion, the spell provides benefits only to yourself. Enemies gain the benefits of this spell if they worship the same god or share the same faith as you, even if you view them as heretical."),
        full_text: true,
    },
    SpellListEntry {
        key: "Weapon of Awe",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(2),
        description: Some("You transform a single weapon into an awe-inspiring instrument. The weapon gains a +2 sacred bonus on damage rolls, and if the weapon scores a critical hit, the target of that critical hit becomes shaken for 1 round with no saving throw.  This is a mind-affecting fear effect. A ranged weapon affected by this spell applies these effects to its ammunition.  You can't cast this spell on a natural weapon, but you can cast it on an unarmed strike."),
        full_text: true,
    },
    SpellListEntry {
        key: "Winds of Vengeance",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(9),
        description: Some("You surround yourself with a buffeting shroud of supernatural, tornado-force winds. These winds grant you a fly speed of 60 feet with perfect maneuverability. Neither your armor nor your load affects this fly speed. The winds shield you from any other wind effects, and form a shell of breathable air around you, allowing you to fly and breathe underwater or in outer space.  Ranged weapons (including giant-thrown boulders, siege weapon projectiles, and other massive ranged weapons) passing through the winds are deflected by the winds and automatically miss you. Gases and most gaseous breath weapons cannot pass though the winds.  In addition, when a creature hits you with a melee attack, you can shape your winds so they lash out at that creature as an immediate action. The creature must make a Fortitude saving throw or take 5d8 points of bludgeoning damage and be knocked prone (if on the ground). On a failed save, Huge flying creatures are checked and Large-sized or smaller flying creatures are blown away instead of knocked prone (see page 439 of the Pathfinder RPG Core Rulebook for more information).  On a successful save, the damage is halved and the creature is not knocked prone (or checked or blown away)."),
        full_text: true,
    },
    SpellListEntry {
        key: "World Wave",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(9),
        description: Some("You cause any sort of natural terrain (including water, forest, desert, tundra, and so on) to surge beneath your feet and safely propel you with devastating force over great distances. This wave of terrain undulates as it passes over the world, harmlessly lifting or stretching objects, creatures, and phenomena with a connection to nature but tearing through and damaging anything else it encounters. When you cast the spell you must choose the wave's type, either a tsunami or a swell. If you choose a tsunami, you create a 30-foot-high tidal wave of earth or water hurtling across the landscape that lasts for 1 round per level. If you choose a swell, you create a more controlled and gentle 5-foot-high wave that lasts for 1 hour per level. Regardless of its form or composition, the crest of the wave extends 10 feet in front of and behind you, and 5 feet per level to both your left and right. You can stand or sit on the crest of this wave without any fear of falling off it and can even lie down and sleep (or take any other actions you could take if standing on solid ground) as it travels. You can grant up to one additional creature per level the ability to safely accompany you on this wave.  When you first create the wave, you must choose its path by facing the direction you wish it to travel. Once you make this decision, you cannot change it. The wave, in either version, moves at eight times your base land speed. Any object, creature, or phenomena strongly connected to, or a part of, the natural world simply rises up and down with the wave as it passes, taking no damage or injury. However, anything else coming into contact with the wave takes either 6d6 points of bludgeoning damage (if the tsunami) or 1d6 points of bludgeoning damage (if the swell).  The wave damages any manufactured object or structure. On the Material Plane, it also damages any creature of the aberration, construct, outsider, or undead type, or with the extraplanar subtype. Creatures with the construct or undead type take double damage from the wave. Other creatures, or creatures with even one druid class level (regardless of their type), are considered a part of the natural world and are unaffected by the wave. Note that on other planes of existence, what is considered to be a part of the natural world may vary at the GM's discretion.  The wave can travel up or down the sides of natural features so long as it does not exceed a 45-degree angle. You cannot alter the dimensions of the wave as it travels. If the wave encounters terrain that it cannot incorporate into itself, the wave simply flows over or around the obstacle (creatures riding the wave are harmlessly displaced to the side to avoid the obstacle) unless the terrain occupies one-half or more of the area of the wave's crest. In such cases the spell ends in 1d6 rounds as the wave falters and collapses, unless its duration would normally cause it to expire prior to that.  The momentum of the wave carries you forward through this new terrain without any injury until it collapses, at which point you suffer the normal effects of the terrain in which you are deposited. Terrain the wave cannot incorporate includes anything primarily made from fire (such as lava), air (such as an open cliff face), or something man-made (such as a city).  You can only create the wave if standing on the ground. You cannot create it when underground or on terrain that it cannot incorporate."),
        full_text: true,
    },
    SpellListEntry {
        key: "Wrath",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(1),
        description: Some("You focus your anger against an enemy. Choose one enemy creature that you can see. You gain a +1 morale bonus on attack rolls and weapon damage rolls against that designated creature for every three caster levels you have (at least +1, maximum +3). You also receive this bonus on caster level checks made to overcome the creature's spell resistance, if any. At 12th level, you gain the benefits of the Improved Critical feat on attack rolls made against the designated creature. This effect doesn't stack with any other effect that expands the threat range of a weapon."),
        full_text: true,
    },
    SpellListEntry {
        key: "Wrathful Mantle",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(3),
        description: Some("A shimmering mantle of light shrouds the subject, casting light like a torch. The subject of the spell gains a +1 resistance bonus on all saving throws per four caster levels (maximum +5 at 20th level). The subject can end the wrathful mantle at any time as a swift action to deal 2d8 points of force damage to all creatures within 5 feet."),
        full_text: true,
    },
    SpellListEntry {
        key: "Borrow Fortune",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(3),
        description: Some("When you make a d20 roll, you may choose to immediately cast this spell to reroll that die before success or failure is known, keeping the more favorable result. For the next two rounds following your casting of the spell, you must roll two dice each time a d20 roll is called for, keeping the less favorable result."),
        full_text: true,
    },
    SpellListEntry {
        key: "Divine Vessel",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(8),
        description: Some("You accept otherworldly energies into your body and transform.  Your size increases by one category, your height doubles, and your weight increases by a factor of eight. Your features shift into those of a cold and alien being of logic, a creature of primal chaos, an angelic presence, or a fiendish monster, as chosen by you. You gain a +6 size bonus to Strength and Constitution, a +3 natural armor bonus, darkvision 60 ft., and SR of 12 + your caster level. These modifiers replace the normal modifiers for increasing your size. The size modifier for AC, attacks, CMB, and CMD changes as appropriate for your new size category.  This spell doesn't change your base speed. Determine space and reach as appropriate for your new size.  If insufficient room is available for the desired growth, you attain the maximum possible size and may make a Strength check (using your increased Strength) to burst any enclosures in the process (see page 175 of the Core Rulebook for rules on breaking objects). If you fail, you are constrained without harm by the materials enclosing you-the spell cannot crush you by increasing your size.  All equipment you wear or carry is similarly enlarged by this spell. Melee weapons deal more damage. Other magical properties are not affected by this spell. Any enlarged item that leaves your possession (including a projectile or thrown weapon) instantly returns to its normal size. This means that thrown and projectile weapons deal their normal damage.  Magical effects that increase size do not stack.  You gain additional abilities as noted below.  Anarchic aspect: You gain the following abilities: an additional +2 bonus to Constitution, DR 10/lawful, resist acid 10, electricity 10, and sonic 10, a +4 bonus on saves against poison, blindsense 30 feet, and a fly speed of 60 feet (good maneuverability). You gain a bite attack dealing 2d6 points of damage. Your natural weapons and any weapons you wield are considered chaotic-aligned for the purpose of overcoming damage resistance.  Axiomatic aspect: You gain the following abilities: an additional +2 bonus to Strength, DR 10/chaotic, resist cold 10, electricity 10, and fire 10, a +4 bonus on saves against poison, low-light vision, and a fly speed of 60 feet (good maneuverability). You gain 2 slam attacks dealing 1d6 points of damage each. Your natural weapons and any weapons you wield are considered lawful-aligned for the purpose of overcoming damage resistance.  Celestial aspect: You gain the following abilities: a +2 bonus to Dexterity, DR 10/evil, resist acid 10, cold 10, and electricity 10, a +4 bonus on saves against poison, low-light vision, and a fly speed of 60 feet (good maneuverability). You gain 2 slam attacks dealing 1d6 points of damage each. Your natural weapons and any weapons you wield are considered goodaligned for the purpose of overcoming damage resistance.  Fiendish aspect: You gain the following abilities: an additional +2 bonus to Strength, DR 10/good, resist acid 10, cold 10, and fire 10, a +4 bonus on saves against poison, see in darkness, and a fly speed of 60 feet (good maneuverability).  You gain 2 claw attacks dealing 1d6 points of damage each. Your natural weapons and any weapons you wield are considered evil-aligned for the purpose of overcoming damage resistance.  When used to assume an anarchic, axiomatic, celestial, or fiendish aspect, this spell is a chaotic, lawful, good, or evil spell respectively."),
        full_text: true,
    },
    SpellListEntry {
        key: "Oracle's Burden",
        school: Some(Pf1SchoolId::Necromancy),
        level: Some(2),
        description: Some("You entreat the forces of fate to bestow your oracle's curse upon another creature. The target creature suffers all the hindrances and none of the benefits of your oracle's curse class feature. You still suffer all effects of your oracle's curse.  If you do not have the oracle's curse class feature, this spell has no effect."),
        full_text: true,
    },
    SpellListEntry {
        // PRESPELL fallback (SD-25 criterion 7.N, "apg-spell-text",
        // register A8-style deliberate method decision): this record is a
        // `PRESPELL:1,Threefold Aspect`-linked sub-form with no own
        // `SCHOOL:`/`CLASSES:`/`DESC:` token in the corpus -- unlike the
        // documented `.COPY=` cross-book scope boundary elsewhere in this
        // module, the base spell here (`Threefold Aspect`, above) lives in
        // this *same* book/file, and its own full text already narrates
        // this exact sub-form ("As the young adult, you gain a +2
        // enhancement bonus..."). Falling back to the base's corpus-native
        // full text is therefore a same-book, exact-identity, non-fabricated
        // extension of the module's existing `.COPY=` fallback convention
        // (not a web second-source claim -- no URL to cite).
        key: "Threefold Aspect (Young Adult)",
        school: None,
        level: None,
        description: Some("Threefold aspect allows you to shift your appearance between your natural age and three idealized age categories: young adult (youth/maiden), adulthood (father/mother), or elderly (elder/crone). In each case, your appearance is your own at the appropriate age, rather than that of a new individual.  You may change between these three aspects or your actual age as a standard action. As the young adult, you gain a +2 enhancement bonus to Dexterity and Constitution, but suffer a -2 penalty to Wisdom. In the adult aspect, you gain a +2 enhancement bonus to Wisdom and Intelligence, but take a -2 penalty to Dexterity. As the elderly aspect, you gain a +4 enhancement bonus to Wisdom and Intelligence, but take a -2 penalty to Strength and Dexterity. As enhancement bonuses, these stack with any bonuses or penalties you may have from your actual age (which are untyped bonuses)-the bonuses granted by this spell represent your idealized form in this threefold aspect rather than simply duplicating your ability scores at any one particular age.  True seeing reveals your natural appearance overlaid with that your aspect, recognizing both as part of your true self.  Individuals who study you closely and have interacted with you at another apparent age recognize a resemblance (as though family) with a successful DC 20 Perception check.  Threefold aspect does not alter your clothing or equipment, and does not heal any deformity or injury unrelated to age."),
        full_text: true,
    },
    SpellListEntry {
        key: "Threefold Aspect (Adulthood)",
        school: None,
        level: None,
        description: Some("Threefold aspect allows you to shift your appearance between your natural age and three idealized age categories: young adult (youth/maiden), adulthood (father/mother), or elderly (elder/crone). In each case, your appearance is your own at the appropriate age, rather than that of a new individual.  You may change between these three aspects or your actual age as a standard action. As the young adult, you gain a +2 enhancement bonus to Dexterity and Constitution, but suffer a -2 penalty to Wisdom. In the adult aspect, you gain a +2 enhancement bonus to Wisdom and Intelligence, but take a -2 penalty to Dexterity. As the elderly aspect, you gain a +4 enhancement bonus to Wisdom and Intelligence, but take a -2 penalty to Strength and Dexterity. As enhancement bonuses, these stack with any bonuses or penalties you may have from your actual age (which are untyped bonuses)-the bonuses granted by this spell represent your idealized form in this threefold aspect rather than simply duplicating your ability scores at any one particular age.  True seeing reveals your natural appearance overlaid with that your aspect, recognizing both as part of your true self.  Individuals who study you closely and have interacted with you at another apparent age recognize a resemblance (as though family) with a successful DC 20 Perception check.  Threefold aspect does not alter your clothing or equipment, and does not heal any deformity or injury unrelated to age."),
        full_text: true,
    },
    SpellListEntry {
        key: "Threefold Aspect (Elderly)",
        school: None,
        level: None,
        description: Some("Threefold aspect allows you to shift your appearance between your natural age and three idealized age categories: young adult (youth/maiden), adulthood (father/mother), or elderly (elder/crone). In each case, your appearance is your own at the appropriate age, rather than that of a new individual.  You may change between these three aspects or your actual age as a standard action. As the young adult, you gain a +2 enhancement bonus to Dexterity and Constitution, but suffer a -2 penalty to Wisdom. In the adult aspect, you gain a +2 enhancement bonus to Wisdom and Intelligence, but take a -2 penalty to Dexterity. As the elderly aspect, you gain a +4 enhancement bonus to Wisdom and Intelligence, but take a -2 penalty to Strength and Dexterity. As enhancement bonuses, these stack with any bonuses or penalties you may have from your actual age (which are untyped bonuses)-the bonuses granted by this spell represent your idealized form in this threefold aspect rather than simply duplicating your ability scores at any one particular age.  True seeing reveals your natural appearance overlaid with that your aspect, recognizing both as part of your true self.  Individuals who study you closely and have interacted with you at another apparent age recognize a resemblance (as though family) with a successful DC 20 Perception check.  Threefold aspect does not alter your clothing or equipment, and does not heal any deformity or injury unrelated to age."),
        full_text: true,
    },
    SpellListEntry {
        // Corpus-parsing correction (SD-25 criterion 7.N, "apg-spell-text"):
        // this base record's own line (`apg_spells.lst:649`) carries *two*
        // `DESC:` tokens back-to-back (a short `!PRERULE:1,DisplayFullSpell`
        // summary, then a full `PRERULE:1,DisplayFullSpell` SRD paragraph
        // matching `crb::spell_list`'s own "Summon Monster I" text
        // verbatim) -- both self-contained on this record's own line, not
        // sourced from a separate `.MOD` record. The prior ingest already
        // captured the full paragraph as `description` but under-flagged
        // `full_text` as `false` (its "sourced from a matching `.MOD`
        // record" rule didn't recognize the same-line double-`DESC:`
        // pattern as a full-text source). Corrected here to `true`; no
        // content change, no web fetch needed. Same finding applies to
        // Summon Monster II-IX below (`apg_spells.lst:650-657`) and their
        // `.COPY=` variants (Reptiles Only / Summons 1d3 Shadows), which
        // inherit this now-corrected `full_text: true` via the base's
        // fields per this module's own `.COPY=` fallback rule.
        key: "Summoner Summon Monster I",
        school: Some(Pf1SchoolId::Conjuration),
        level: None,
        description: Some("This spell summons an extraplanar creature [typically an outsider, elemental, or magical beast native to another plane]. It appears where you designate and acts immediately, on your turn. It attacks your opponents to the best of its ability. If you can communicate with the creature, you can direct it not to attack, to attack particular enemies, or to perform other actions. The spell conjures one of the creatures from the 1st Level list on Table 10-5. You choose which kind of creature to summon, and you can choose a different one each time you cast the spell. A summoned monster cannot summon or otherwise conjure another creature, nor can it use any teleportation or planar travel abilities. Creatures cannot be summoned into an environment that cannot support them. Creatures summoned using this spell cannot use spells or spell-like abilities that duplicate spells with expensive material components [such as wish]. When you use a summoning spell to summon a creature with an alignment or elemental subtype, it is a spell of that type. Creatures on Table 10-5 marked with an \"*\" are summoned with the celestial template, if you are good, and the fiendish template, if you are evil. If you are neutral, you may choose which template to apply to the creature. Creatures marked with an \"*\" always have an alignment that matches yours, regardless of their usual alignment. Summoning these creatures makes the summoning spell's type match your alignment. [Table Not Included]"),
        full_text: true,
    },
    SpellListEntry {
        key: "Summoner Summon Monster II",
        school: Some(Pf1SchoolId::Conjuration),
        level: None,
        description: Some("This spell functions like summon monster I, except that you can summon one creature from the 2nd-level list or 1d3 creatures of the same kind from the 1st-level list."),
        full_text: true,
    },
    SpellListEntry {
        key: "Summoner Summon Monster III",
        school: Some(Pf1SchoolId::Conjuration),
        level: None,
        description: Some("This spell functions like summon monster I, except that you can summon one creature from the 3rd-level list, 1d3 creatures of the same kind from the 2nd-level list, or 1d4+1 creatures of the same kind from the 1st-level list."),
        full_text: true,
    },
    SpellListEntry {
        key: "Summoner Summon Monster IV",
        school: Some(Pf1SchoolId::Conjuration),
        level: None,
        description: Some("This spell functions like summon monster I, except that you can summon one creature from the 4th-level list, 1d3 creatures of the same kind from the 3rd-level list, or 1d4+1 creatures of the same kind from a lower-level list."),
        full_text: true,
    },
    SpellListEntry {
        key: "Summoner Summon Monster V",
        school: Some(Pf1SchoolId::Conjuration),
        level: None,
        description: Some("This spell functions like summon monster I, except that you can summon one creature from the 5th-level list, 1d3 creatures of the same kind from the 4th-level list, or 1d4+1 creatures of the same kind from a lower-level list."),
        full_text: true,
    },
    SpellListEntry {
        key: "Summoner Summon Monster VI",
        school: Some(Pf1SchoolId::Conjuration),
        level: None,
        description: Some("This spell functions like summon monster I, except you can summon one creature from the 6th-level list, 1d3 creatures of the same kind from the 5th-level list, or 1d4+1 creatures of the same kind from a lower-level list."),
        full_text: true,
    },
    SpellListEntry {
        key: "Summoner Summon Monster VII",
        school: Some(Pf1SchoolId::Conjuration),
        level: None,
        description: Some("This spell functions like summon monster I, except that you can summon one creature from the 7th-level list, 1d3 creatures of the same kind from the 6th-level list, or 1d4+1 creatures of the same kind from a lower-level list."),
        full_text: true,
    },
    SpellListEntry {
        key: "Summoner Summon Monster VIII",
        school: Some(Pf1SchoolId::Conjuration),
        level: None,
        description: Some("This spell functions like summon monster I, except that you can summon one creature from the 8th-level list, 1d3 creatures of the same kind from the 7th-level list, or 1d4+1 creatures of the same kind from a lower-level list."),
        full_text: true,
    },
    SpellListEntry {
        key: "Summoner Summon Monster IX",
        school: Some(Pf1SchoolId::Conjuration),
        level: None,
        description: Some("This spell functions like summon monster I, except that you can summon one creature from the 9th-level list, 1d3 creatures of the same kind from the 8th-level list, or 1d4+1 creatures of the same kind from a lower-level list."),
        full_text: true,
    },
    SpellListEntry {
        // Inherited from CRB `Blindness/Deafness` (see this module's `.COPY=` note).
        key: "Blindness/Deafness (Only Cause Blindness)",
        school: Some(Pf1SchoolId::Necromancy),
        level: Some(2),
        description: Some("You call upon the powers of unlife to render the subject blinded or deafened, as you choose."),
        full_text: true,
    },
    SpellListEntry {
        key: "Summon Monster V (Summons 1d3 Shadows)",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(5), // inherited from CRB `Summon Monster V`
        description: Some("This spell functions like summon monster I, except that you can summon one creature from the 5th-level list, 1d3 creatures of the same kind from the 4th-level list, or 1d4+1 creatures of the same kind from a lower-level list."),
        full_text: true,
    },
    SpellListEntry {
        // Inherited from CRB `Planar Binding` (see this module's `.COPY=` note).
        key: "Planar Binding (Demons Only)",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(6),
        description: Some("This spell functions like lesser planar binding, except that you may call a single creature of 12 HD or less, or up to three creatures of the same kind whose Hit Dice total no more than 12. Each creature gets a saving throw, makes an independent attempt to escape, and must be individually persuaded to aid you."),
        full_text: true,
    },
    SpellListEntry {
        // Inherited from CRB `Planar Binding` (see this module's `.COPY=` note).
        key: "Planar Binding (Devils Only)",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(6),
        description: Some("This spell functions like lesser planar binding, except that you may call a single creature of 12 HD or less, or up to three creatures of the same kind whose Hit Dice total no more than 12. Each creature gets a saving throw, makes an independent attempt to escape, and must be individually persuaded to aid you."),
        full_text: true,
    },
    SpellListEntry {
        // Inherited from CRB `Planar Ally` (see this module's `.COPY=` note).
        key: "Planar Ally (Agathions Only)",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(6),
        description: Some("This spell functions like lesser planar ally, except you may call a single creature of 12 HD or less, or two creatures of the same kind whose HD total no more than 12. The creatures agree to help you and request your return payment together."),
        full_text: true,
    },
    SpellListEntry {
        // Inherited from CRB `Planar Ally` (see this module's `.COPY=` note).
        key: "Planar Ally (Archon Only)",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(6),
        description: Some("This spell functions like lesser planar ally, except you may call a single creature of 12 HD or less, or two creatures of the same kind whose HD total no more than 12. The creatures agree to help you and request your return payment together."),
        full_text: true,
    },
    SpellListEntry {
        // Inherited from CRB `Planar Ally` (see this module's `.COPY=` note).
        key: "Planar Ally (Azata Only)",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(6),
        description: Some("This spell functions like lesser planar ally, except you may call a single creature of 12 HD or less, or two creatures of the same kind whose HD total no more than 12. The creatures agree to help you and request your return payment together."),
        full_text: true,
    },
    SpellListEntry {
        // Inherited from CRB `Beast Shape I` (see this module's `.COPY=` note).
        key: "Beast Shape I (Animals Only)",
        school: Some(Pf1SchoolId::Transmutation),
        level: Some(3),
        description: Some("When you cast this spell, you can assume the form of any Small or Medium creature of the animal type. If the form you assume has any of the following abilities, you gain the listed ability: climb 30 feet, fly 30 feet [average maneuverability], swim 30 feet, darkvision 60 feet, low-light vision, and scent. Small animal: If the form you take is that of a Small animal, you gain a +2 size bonus to your Dexterity and a +1 natural armor bonus. Medium animal: If the form you take is that of a Medium animal, you gain a +2 size bonus to your Strength and a +2 natural armor bonus."),
        full_text: true,
    },
    SpellListEntry {
        // Inherited from CRB `Planar Binding` (see this module's `.COPY=` note).
        key: "Planar Binding (Daemons Only)",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(6),
        description: Some("This spell functions like lesser planar binding, except that you may call a single creature of 12 HD or less, or up to three creatures of the same kind whose Hit Dice total no more than 12. Each creature gets a saving throw, makes an independent attempt to escape, and must be individually persuaded to aid you."),
        full_text: true,
    },
    SpellListEntry {
        // Inherited from CRB `Planar Binding` (see this module's `.COPY=` note).
        key: "Planar Binding (Inevitables Only)",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(6),
        description: Some("This spell functions like lesser planar binding, except that you may call a single creature of 12 HD or less, or up to three creatures of the same kind whose Hit Dice total no more than 12. Each creature gets a saving throw, makes an independent attempt to escape, and must be individually persuaded to aid you."),
        full_text: true,
    },
    SpellListEntry {
        // Inherited from CRB `Planar Binding` (see this module's `.COPY=` note).
        key: "Planar Binding (Proteans Only)",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(6),
        description: Some("This spell functions like lesser planar binding, except that you may call a single creature of 12 HD or less, or up to three creatures of the same kind whose Hit Dice total no more than 12. Each creature gets a saving throw, makes an independent attempt to escape, and must be individually persuaded to aid you."),
        full_text: true,
    },
    SpellListEntry {
        key: "Elemental Aura (Cold Only)",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(3),
        description: Some("This spell forms an aura of energy around you, damaging all those that come near you. Choose an energy type: acid, cold, electricity, or fire. Creatures adjacent to you when this spell is cast and at the start of your turn take 2d6 points of energy damage of the selected type. This aura has an additional effect, depending upon the type of energy chosen.  Acid: Creatures affected by your aura take 1 point of ongoing acid damage per round for 1 round per three caster levels, and are sickened for the duration of the ongoing acid damage.  Cold: Creatures affected by your aura are fatigued. A creature that is already fatigued suffers no additional effect.  Electricity: Creatures affected by your aura are staggered for 1 round.  Fire: Creatures affected by your aura catch on fire (Core Rulebook 444).  Creatures adjacent to you are allowed a Reflex save to halve the damage and negate the additional effect. The aura's additional effects do not stack if a creature takes damage from your aura multiple times. You may only have one elemental aura in effect at one time. When you cast this spell to deal acid, cold, electricity, or fire damage, it is a spell of that type."),
        full_text: true,
    },
    SpellListEntry {
        // Inherited from CRB `Meteor Swarm` (see this module's `.COPY=` note).
        key: "Meteor Swarm (Dealing Cold Damage)",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(9),
        description: Some("Meteor swarm is a very powerful and spectacular spell that is similar to fireball in many aspects. When you cast it, four 2- foot-diameter spheres spring from your outstretched hand and streak in straight lines to the spots you select. The meteor spheres leave a fiery trail of sparks. If you aim a sphere at a specific creature, you may make a ranged touch attack to strike the target with the meteor. Any creature struck by a sphere takes 2d6 points of bludgeoning damage [no save] and takes a -4 penalty on the saving throw against the sphere's fire damage [see below]. If a targeted sphere misses its target, it simply explodes at the nearest corner of the target's space. You may aim more than one sphere at the same target. Once a sphere reaches its destination, it explodes in a 40- foot-radius spread, dealing 6d6 points of fire damage to each creature in the area. If a creature is within the area of more than one sphere, it must save separately against each. Despite stemming from separate spheres, all of the fire damage is added together after the saves have been made, and fire resistance is applied only once."),
        full_text: true,
    },
    SpellListEntry {
        key: "Summon Monster III (Reptiles Only)",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(3), // inherited from CRB `Summon Monster III`
        description: Some("This spell functions like summon monster I, except that you can summon one creature from the 3rd-level list, 1d3 creatures of the same kind from the 2nd-level list, or 1d4+1 creatures of the same kind from the 1st-level list."),
        full_text: true,
    },
    SpellListEntry {
        key: "Summon Monster VII (Reptiles Only)",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(7), // inherited from CRB `Summon Monster VII`
        description: Some("This spell functions like summon monster I, except that you can summon one creature from the 7th-level list, 1d3 creatures of the same kind from the 6th-level list, or 1d4+1 creatures of the same kind from a lower-level list."),
        full_text: true,
    },
    SpellListEntry {
        // Corpus-parsing correction (SD-25 criterion 7.N, "apg-spell-text"):
        // this `.COPY=Call Lightning Storm` variant has its own `.MOD`
        // record (`apg_spells.lst:1075`) that overrides `DESC:` with a
        // full, self-contained `PRERULE:1,DisplayFullSpell` paragraph --
        // corpus-native, no web fetch needed. `school`/`level` stay `None`:
        // the base `Call Lightning Storm` spell itself is not present in
        // this book's corpus (cross-book base, per this module's
        // documented `.COPY=` scope boundary), so there is no in-book
        // `SCHOOL:`/`CLASSES:` token to inherit.
        //
        // **The cross-book boundary is resolved (2026-07-31)**, so
        // `school`/`level` now come from CRB `Call Lightning Storm`
        // (Evocation, 5) like every other cross-book `.COPY=` row. The
        // `description` above is unaffected and stays the variant's own
        // `.MOD` override — which is exactly PCGen's layering: `.COPY=`
        // inherits the base, then the variant's `.MOD DESC:` replaces one
        // field. It is deliberately NOT equal to the base's text.
        key: "Call Lightning Storm (Starsoul)",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(5),
        description: Some("This spell functions like call lightning, except that each bolt deals 5d6 points of Fire damage [or 5d10 if created outdoors at night], and you may call a maximum of 15 bolts."),
        full_text: true,
    },
    SpellListEntry {
        // **Upstream typo, preserved verbatim.** `apg_spells.lst:1555` reads
        // `Wall of Thorms<TAB>DOMAINS:Blood Subdomain=5<TAB>SOURCELINK:...
        // /spells/wallOfThorns.html#_wall-of-thorns`. It is meant to be
        // `Wall of Thorns.MOD`, and the same file proves it: `:1431` is
        // `Wall of Thorns.MOD  DOMAINS:Decay Subdomain=5` — the identical
        // construct, correctly spelled and correctly suffixed — every other
        // row in the `:1550`-`:1560` subdomain block is `<CRB spell>.MOD
        // DOMAINS:...`, the row's own SOURCELINK names the real spell, and
        // `Thorms` appears exactly once in the whole PCGen checkout. Dropping
        // the `.MOD` is what makes PCGen mint a junk spell object rather than
        // modify the real one.
        //
        // The KEY is NOT corrected here: a record in this repo cites a
        // `source.line` a reader is expected to be able to open, and renaming
        // it would make this table disagree with the file it cites. The
        // fields are resolved against CRB `Wall of Thorns` exactly as the
        // eleven `.COPY=` rows above are resolved against their bases, so a
        // player sees the real spell under a visibly odd name instead of a
        // blank row. Pinned by
        // `tests/sd27_apg_delta_spell_rows_resolve_against_their_base.rs`.
        key: "Wall of Thorms",
        school: Some(Pf1SchoolId::Conjuration),
        level: Some(5),
        description: Some("A wall of thorns spell creates a barrier of very tough, pliable, tangled brush bearing needle-sharp thorns as long as a human's finger. Any creature forced into or attempting to move through a wall of thorns takes piercing damage per round of movement equal to 25 minus the creature's AC. Dexterity and dodge bonuses to AC do not count for this calculation. [Creatures with an AC of 25 or higher, without considering Dexterity and dodge bonuses, take no damage from contact with the wall.] You can make the wall as thin as 5 feet thick, which allows you to shape the wall as a number of 10-by-10-by-5-foot blocks equal to twice your caster level. This has no effect on the damage dealt by the thorns, but any creature attempting to break through takes that much less time to force its way through the barrier. Creatures can force their way slowly through the wall by making a Strength check as a full-round action. For every 5 points by which the check exceeds 20, a creature moves 5 feet [up to a maximum distance equal to its normal land speed]. Of course, moving or attempting to move through the thorns incurs damage as described above. A creature trapped in the thorns can choose to remain motionless in order to avoid taking any more damage. Any creature within the area of the spell when it is cast takes damage as if it had moved into the wall and is caught inside. In order to escape, it must attempt to push its way free, or it can wait until the spell ends. Creatures with the ability to pass through overgrown areas unhindered can pass through a wall of thorns at normal speed without taking damage. A wall of thorns can be breached by slow work with edged weapons. Chopping away at the wall creates a safe passage 1 foot deep for every 10 minutes of work. Normal fire cannot harm the barrier, but magical fire burns it away in 10 minutes."),
        full_text: true,
    },
    SpellListEntry {
        // Web second-source pass (SD-25 criterion 7.N, "apg-spell-text"):
        // no `.MOD` full-text record exists in `apg_spells.lst` for any of
        // this and the following 4 "hero point" spells (their only
        // `.MOD` stanzas are `#`-commented-out `ITEM:Potion` lines).
        // Identity-matched (name + School + exact Level/CLASSES set, all
        // exact) against
        // https://legacy.aonprd.com/advancedPlayersGuide/advancedNewRules.html
        // -- the original APG hero-points-chapter page, not the
        // later/edition-cousin "Book of the Damned" reprint of the
        // same-named "Malediction" spell (Antipaladin/Cleric/Shaman/
        // Spiritualist/Witch 4, unrelated hero-point mechanic) that a
        // first d20pfsrd.com search surfaced and was rejected as a false
        // match per this cycle's identity-match rule.
        key: "Heroic Fortune",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(2),
        description: Some("This spell grants 1 hero point to the target. This hero point must be spent before the duration expires, or it is lost. The bonus hero point is spent before any other hero points the target might possess."),
        full_text: true,
    },
    SpellListEntry {
        key: "Heroic Fortune (Mass)",
        school: Some(Pf1SchoolId::Evocation),
        level: Some(4),
        description: Some("This spell functions like heroic fortune, except that it affects one or more creatures, no two of which can be more than 30 ft. apart, at close range (25 ft. + 5 ft./2 levels)."),
        full_text: true,
    },
    SpellListEntry {
        key: "Malediction",
        school: Some(Pf1SchoolId::Necromancy),
        level: None,
        description: Some("You utter a dire curse over the body of a dying creature, allowing you to consume its waning life force. Upon casting this spell, you touch a living creature that has -1 or fewer hit points. If the target fails its saving throw, it dies and you gain 1 hero point for every 5 Hit Dice possessed by the target (minimum 1, maximum 3). These hero points last for a number of minutes equal to the target's Hit Dice. Any hero points remaining when this spell ends are lost."),
        full_text: true,
    },
    SpellListEntry {
        key: "Severed Fate",
        school: Some(Pf1SchoolId::Enchantment),
        level: Some(2),
        description: Some("You curse the target, preventing it from drawing upon the powers of destiny. The target is shaken and cannot use hero points for the duration of the spell. This effect can be removed by dispel magic, remove curse, or other similar effects."),
        full_text: true,
    },
    SpellListEntry {
        key: "Unravel Destiny",
        school: Some(Pf1SchoolId::Divination),
        level: None,
        description: Some("You utter a dire portent, causing destiny and fate to unravel around the target. This profoundly disturbing effect causes the target to suffer a cumulative -2 penalty on all ability checks, attack rolls, saving throws, and skill checks for every hero point it possesses. The target can reduce this penalty by spending hero points normally, but it takes 2d6 points of damage for each hero point spent while this spell is in effect."),
        full_text: true,
    },
    SpellListEntry {
        key: "Corruption Resistance (Evil)",
        school: Some(Pf1SchoolId::Abjuration),
        level: None,
        description: Some("You grant the touched creature limited protection from magical effects that inflict damage based on the target creature's alignment, such as holy smite, order's wrath, a paladin's smite evil attack, or an unholy weapon. Choose one alignment type: chaos, evil, good, or law. The subject takes 5 less points of damage from effects that specifically harm creatures of that alignment. The value of the protection increases to 10 points at 7th level and 15 points at 11th level. The spell protects the recipient's equipment as well.  Corruption resistance protects against spells, spell-like abilities, and special abilities, not physical attacks. Corruption resistance only protects against damage. The subject can still suffer side effects from such attacks. When you use this spell to protect an alignment, it gains the descriptor of that alignment."),
        full_text: true,
    },
];

/// SD-24 Epic 6 criterion 6.1 — spell field-coverage audit row. Mirrors
/// `rules_tables::crb::spell_list::SpellFieldCoverage`'s shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpellFieldCoverage {
    /// Records currently in `SPELL_LIST`.
    pub total_records: u32,
    /// Real, active (non-`.MOD`, non-comment), deduplicated-by-name
    /// record count in `apg_spells.lst` (297 — see this module's doc
    /// comment for the 298 -> 297 audit correction).
    pub records_expected: u32,
    /// Records with `description.is_some()` -- a real per-row count
    /// (this field is `Option`, not always-populated, unlike the SD-22
    /// bootstrap's non-optional field of the same name).
    pub has_description: u32,
    /// Records whose ingested `description` is the full SRD/PRD spell
    /// text (`full_text: true`), sourced from a matching `<Name>.MOD`
    /// record's own `DESC:` token, not merely the short base-record
    /// summary.
    pub full_text_verified: u32,
}

/// Computes this book's spell field-coverage audit row.
pub fn spell_coverage_report() -> SpellFieldCoverage {
    SpellFieldCoverage {
        total_records: SPELL_LIST.len() as u32,
        records_expected: 297,
        has_description: SPELL_LIST.iter().filter(|entry| entry.description.is_some()).count() as u32,
        full_text_verified: SPELL_LIST.iter().filter(|entry| entry.full_text).count() as u32,
    }
}

/// The `(KEY:` token, display name`)` pair for each of the 9
/// `apg_spells.lst` rows whose two name columns differ — read verbatim
/// from the rows themselves (`apg_spells.lst:649`-`657`), e.g. `:651` is
/// `Summon Monster III<TAB>KEY:Summoner Summon Monster III`. Every other
/// ingested APG row carries no `KEY:` token at all, so its display name
/// *is* its key and it needs no entry here.
///
/// Sibling of `acg::spell_list::ARCHETYPE_QUALIFIED_KEYS`; see that
/// constant's doc comment for why both name columns are real. Note that
/// the bare `Summon Monster <roman>` display name is unambiguous within
/// APG: the book's other ingested `Summon Monster` records are the
/// `.COPY=`-derived variants, which all carry a distinguishing
/// parenthetical (`Summon Monster V (Summons 1d3 Shadows)`,
/// `Summon Monster III (Reptiles Only)`, `Summon Monster VII (Reptiles
/// Only)`), and the many bare `Summon Monster <roman>.MOD` rows are
/// modifiers, not ingested records.
pub const ARCHETYPE_QUALIFIED_KEYS: &[(&str, &str)] = &[
    ("Summoner Summon Monster I", "Summon Monster I"),
    ("Summoner Summon Monster II", "Summon Monster II"),
    ("Summoner Summon Monster III", "Summon Monster III"),
    ("Summoner Summon Monster IV", "Summon Monster IV"),
    ("Summoner Summon Monster V", "Summon Monster V"),
    ("Summoner Summon Monster VI", "Summon Monster VI"),
    ("Summoner Summon Monster VII", "Summon Monster VII"),
    ("Summoner Summon Monster VIII", "Summon Monster VIII"),
    ("Summoner Summon Monster IX", "Summon Monster IX"),
];

/// Resolves an APG spell by either of its two corpus name columns, scoped
/// to `RuleSetId::Apg`. Returns `None` for any other rule set (cross-book
/// invariant, mirrors `apg::class_chassis_resolve`), and `None` when
/// `name_or_key` names no APG record.
///
/// A record's `KEY:` token always resolves. So does its display name,
/// when the two differ — the 9 rows in [`ARCHETYPE_QUALIFIED_KEYS`] —
/// because a caller may hold only the name a selection carries, with no
/// archetype context. The KEY is tried first, so an exact record key can
/// never be shadowed by another record's display name; within APG the
/// question is moot, since no display name in that table is also a record
/// key (asserted by `tests/spell_cross_book_identity.rs`).
pub fn spell_resolve(name_or_key: &str, rule_set: RuleSetId) -> Option<&'static SpellListEntry> {
    if rule_set != RuleSetId::Apg {
        return None;
    }
    if let Some(entry) = SPELL_LIST.iter().find(|entry| entry.key == name_or_key) {
        return Some(entry);
    }
    let key = ARCHETYPE_QUALIFIED_KEYS
        .iter()
        .find(|(_, display)| *display == name_or_key)
        .map(|(key, _)| *key)?;
    SPELL_LIST.iter().find(|entry| entry.key == key)
}
