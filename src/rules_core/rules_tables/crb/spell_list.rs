//! PF1 CRB strict-school spell list.
//!
//! Bootstrap coverage: one representative spell per school, copied
//! verbatim (`KEY:`/name and `DESC:` text) from the real PCGen corpus
//! (`core_rulebook/cr_spells.lst`) — not synthesized. Exhaustive
//! per-school coverage (~652 spells total) is the loop's job, one school
//! per cycle, per `scope-draft.md` §2.4 ("prove the school's spells are
//! reachable... landing all spells in the school in one round").

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pf1SchoolId {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
    Universal,
}

impl Pf1SchoolId {
    pub const ALL: &'static [Pf1SchoolId] = &[
        Pf1SchoolId::Abjuration,
        Pf1SchoolId::Conjuration,
        Pf1SchoolId::Divination,
        Pf1SchoolId::Enchantment,
        Pf1SchoolId::Evocation,
        Pf1SchoolId::Illusion,
        Pf1SchoolId::Necromancy,
        Pf1SchoolId::Transmutation,
        Pf1SchoolId::Universal,
    ];

    /// Maps the corpus's raw `SCHOOL:` string to the strict-school enum.
    /// Returns `None` for an unrecognized string (SD-19's resolvers
    /// route that case to `Open Blockers` rather than guessing).
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
            "Universal" => Some(Pf1SchoolId::Universal),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpellListEntry {
    /// The spell's corpus identity. `cr_spells.lst` has no `KEY:` token
    /// for spells (unlike equipment records) — the record's `name` field
    /// is its identity, matching `LstSpellRecord.name`.
    pub key: &'static str,
    pub school: Pf1SchoolId,
    /// The minimum spell level across the corpus's `CLASSES:` tag for
    /// this record (e.g. `CLASSES:Bard,Ranger,Sorcerer,Wizard=1` -> 1).
    pub level: u8,
    pub description: &'static str,
}

/// Source: `pathfinder/paizo/roleplaying_game/core_rulebook/cr_spells.lst`.
pub const SPELL_LIST: &[SpellListEntry] = &[
    SpellListEntry {
        key: "Alarm",
        school: Pf1SchoolId::Abjuration,
        level: 1,
        description: "Alarm creates a subtle ward on an area you select.",
    },
    SpellListEntry {
        key: "Acid Arrow",
        school: Pf1SchoolId::Conjuration,
        level: 2,
        description: "An arrow of acid springs from your hand and speeds to its target dealing 2d4 points of acid damage.",
    },
    SpellListEntry {
        key: "Analyze Dweomer",
        school: Pf1SchoolId::Divination,
        level: 6,
        description: "You can observe magical auras.",
    },
    SpellListEntry {
        key: "Aid",
        school: Pf1SchoolId::Enchantment,
        level: 2,
        description: "Aid grants +1 morale bonus on attack rolls and saves vs fear effects, plus 1d8 + (min(CASTERLEVEL,10)) temporary hit points.",
    },
    SpellListEntry {
        key: "Blade Barrier",
        school: Pf1SchoolId::Evocation,
        level: 6,
        description: "An immobile, vertical curtain of whirling blades shaped of pure force springs into existence dealing damage to any creature passing through.",
    },
    SpellListEntry {
        key: "Blur",
        school: Pf1SchoolId::Illusion,
        level: 2,
        description: "The subject's outline appears blurred, shifting, and wavering granting the subject concealment (20% miss chance).",
    },
    SpellListEntry {
        key: "Animate Dead",
        school: Pf1SchoolId::Necromancy,
        level: 3,
        description: "Turns corpses into undead skeletons or zombies that obey your spoken commands.",
    },
    SpellListEntry {
        key: "Air Walk",
        school: Pf1SchoolId::Transmutation,
        level: 4,
        description: "The subject can tread on air as if walking on solid ground.",
    },
    SpellListEntry {
        key: "Arcane Mark",
        school: Pf1SchoolId::Universal,
        level: 0,
        description: "This spell allows you to inscribe your personal rune or mark.",
    },
];
