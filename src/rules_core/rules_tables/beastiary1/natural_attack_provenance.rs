//! Per-attack provenance for the Bestiary 1 natural attacks that are
//! **not** transcribed from a `NATURALATTACKS:` token on the monster's
//! own `b1_races.lst` row.
//!
//! # Why this module exists
//!
//! Read this before "correcting" any `natural_attacks` entry in
//! `monster_subset_0*.rs` back to an empty list. **The absent corpus
//! token is not evidence that the monster has no attack.** It is a
//! documented shape of the PCGen data model, explained below.
//!
//! ## The corpus genuinely cannot supply these damage dice
//!
//! Monsters whose attacks were already present carry their dice inline
//! on their own row, e.g. Ghoul (`b1_races.lst:200`):
//!
//! ```text
//! NATURALATTACKS:Claw,Weapon.Natural...Finesseable.Piercing.Slashing,*2,1d6
//! ```
//!
//! The twelve monsters this module covers carry only a **cross-reference**
//! on their row instead, e.g. Ankheg (`b1_races.lst:18`):
//!
//! ```text
//! ABILITY:Internal|AUTOMATIC|Bite
//! ```
//!
//! That reference **does** resolve — a prior investigation concluded the
//! target row was simply missing from Bestiary 1, but that conclusion was
//! too narrow. The rows exist in
//! `pathfinder/paizo/roleplaying_game/core_essentials/ce_abilities_race.lst`
//! (`Bite` at line 249, `Gore` 250, `Claw` 251, `Slam` 252, `Tail Slap`
//! 258, `Hoof` 259, `Tentacle` 260), not under `bestiary/` — which is why
//! a bestiary-only grep found nothing.
//!
//! **Resolving the reference still does not yield damage dice.** Those
//! `CATEGORY:Internal` rows carry *no* `NATURALATTACKS:` token at all.
//! They are pure mechanical markers — they `DEFINE:`/`BONUS:VAR|` the
//! attack-count and Strength-application variables and grant weapon
//! proficiency:
//!
//! ```text
//! Claw  CATEGORY:Internal  TYPE:NaturalAttack.NaturalAttackPrimary.Primary
//!       DEFINE:ClawAttacks|0  BONUS:VAR|ClawAttacks|2
//!       BONUS:WEAPONPROF=Claw|DAMAGE|MAX(STR/2,0)|...
//! ```
//!
//! The actual dice are supplied at *runtime* by PCGen's size-based
//! natural-attack damage tables, exactly like the AC/HP/save values that
//! `MonsterStatBlock`'s own doc comment already declares out of scope.
//! So no cross-file resolver — however thorough — recovers them. The
//! values below therefore come from the published Bestiary 1 text,
//! corroborated per `SD-26 decisions.md §11.5`.
//!
//! ## What each provenance kind means here
//!
//! - [`AttackSource::LstToken`] — genuinely recovered from a real,
//!   checkable corpus token. Exactly one attack qualifies: Crocodile's
//!   Tail Slap (see [`GROUNDED_NATURAL_ATTACKS`]).
//! - [`AttackSource::WebSecondSource`] — grounded from published values
//!   under `§11.5`'s methodology: allowed domains only
//!   (`aonprd.com`/`legacy.aonprd.com`, `d20pfsrd.com`), identity matched
//!   on more than name alone, and **never** written without at least two
//!   independent agreeing sources.
//!
//! ## The attack *names* are corpus-grounded even where the dice are not
//!
//! Every name below is the literal operand of a real
//! `ABILITY:Internal|AUTOMATIC|<Name>` token on the monster's own row
//! (recorded in [`GroundedAttack::corpus_name_token`]). Only the
//! `damage_dice` needed external grounding. This is why the JSON cache
//! record keeps `source.kind: "lst_token"` at record level — the chassis
//! fields *and* the attack names really do come from that row — while
//! this table supplies the narrower per-field provenance the record-level
//! union cannot express (the structural gap `§11.3` flagged: "the struct
//! doesn't currently store which of the five provenance kinds produced a
//! given value ... flag anything it can't confidently attribute rather
//! than guessing").
//!
//! ## Do not "correct" these back
//!
//! `tests/v06_beastiary1_natural_attack_grounding.rs` pins every value
//! here to its cited sources, and asserts this table and the shipped
//! `monster_subset_0*.rs` tables agree in **both** directions — an
//! attack added to one without the other fails the build. Follow the
//! Monk hit-die precedent in `rules_tables::crb::class_tables` (a
//! deliberate, documented deviation from a literal corpus reading):
//! deviations are recorded, not silently reverted.

/// Where a single natural attack's `damage_dice` value actually came
/// from. Mirrors the two `SD-26 decisions.md §11.2` discriminated-union
/// kinds that apply to this data; the other three kinds
/// (`lst_inherited_copy`, `lst_corrected_ingest`, `same_book_fallback`)
/// have no instance here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackSource {
    /// A real, checkable `NATURALATTACKS:` token in the live PCGen
    /// corpus — just not on the monster's own `b1_races.lst` row.
    LstToken {
        /// Corpus-root-relative path of the file carrying the token.
        path: &'static str,
        /// 1-based line number of the record within `path`.
        line: u32,
        /// The record's `KEY:` (or first-column name) at `line`.
        record_key: &'static str,
    },
    /// Grounded from the published Bestiary 1 text via `§11.5`'s
    /// allowed-domain web methodology.
    WebSecondSource {
        /// Independent agreeing sources. `§11.5` permits only
        /// `aonprd.com` / `legacy.aonprd.com` / `d20pfsrd.com`; the
        /// grounding bar requires **at least two**, both asserted by
        /// `tests/v06_beastiary1_natural_attack_grounding.rs`.
        urls: &'static [&'static str],
        /// ISO-8601 date the sources were read.
        fetched_at: &'static str,
        /// Which fields beyond the name confirmed this is the right
        /// creature — `§11.5`'s "reject same-named cross-book /
        /// edition-cousin false matches" discipline.
        identity_match_basis: &'static str,
    },
}

/// One natural attack whose `damage_dice` is not transcribed from the
/// monster's own `b1_races.lst` row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroundedAttack {
    /// Canonical `beastiary1:monster:<slug>` key, matching
    /// `super::monster_key_resolve`'s key shape.
    pub monster_key: &'static str,
    /// Must equal the shipped `NaturalAttack::name`.
    pub attack_name: &'static str,
    /// Must equal the shipped `NaturalAttack::damage_dice`.
    pub damage_dice: &'static str,
    /// The real token on the monster's own row that *names* this attack
    /// (the dice are what the corpus lacks, not the name).
    pub corpus_name_token: &'static str,
    /// Verbatim published "Melee" text the dice were read from, kept so
    /// a future reader can re-check the transcription without refetching.
    pub published_melee_text: &'static str,
    /// Where `damage_dice` came from.
    pub source: AttackSource,
}

/// Every Bestiary 1 natural attack whose damage dice required grounding
/// outside the monster's own corpus row.
///
/// Ordering within a monster follows that monster's own
/// `ABILITY:Internal|AUTOMATIC|` operand order where the row lists more
/// than one (Wolverine's row reads `...|Bite|Claw`), preserving the
/// existing "attack order comes from the real row's token order"
/// convention that `tests/sd26_cache_beastiary.rs` documents. Cave
/// Fisher is the one partial case: its real `NATURALATTACKS:Filament...`
/// token stays first and untouched, with the recovered Claw appended.
pub const GROUNDED_NATURAL_ATTACKS: &[GroundedAttack] = &[
    GroundedAttack {
        monster_key: "beastiary1:monster:ankheg",
        attack_name: "Bite",
        damage_dice: "2d6",
        corpus_name_token: "ABILITY:Internal|AUTOMATIC|Bite",
        published_melee_text: "bite +5 (2d6+4 plus 1d4 acid and grab)",
        source: AttackSource::WebSecondSource {
            urls: &[
                "https://www.aonprd.com/MonsterDisplay.aspx?ItemName=Ankheg",
                "https://www.d20pfsrd.com/bestiary/monster-listings/magical-beasts/ankheg/",
            ],
            fetched_at: "2026-07-29",
            identity_match_basis: "name + CR 3 + Large magical beast + Bestiary pg. 15 (matches the cached chassis's own source_page)",
        },
    },
    GroundedAttack {
        monster_key: "beastiary1:monster:assassin_vine",
        attack_name: "Slam",
        damage_dice: "1d8",
        corpus_name_token: "ABILITY:Internal|AUTOMATIC|Slam",
        published_melee_text: "slam +7 (1d8+7 plus grab)",
        source: AttackSource::WebSecondSource {
            urls: &[
                "https://www.aonprd.com/MonsterDisplay.aspx?ItemName=Assassin+Vine",
                "https://www.d20pfsrd.com/bestiary/monster-listings/plants/assassin-vine/",
            ],
            fetched_at: "2026-07-29",
            identity_match_basis: "name + CR 3 + Large plant + Bestiary pg. 22 (matches the cached chassis's own source_page)",
        },
    },
    GroundedAttack {
        monster_key: "beastiary1:monster:boar",
        attack_name: "Gore",
        damage_dice: "1d8",
        corpus_name_token: "ABILITY:Internal|AUTOMATIC|Gore",
        published_melee_text: "gore +4 (1d8+4)",
        source: AttackSource::WebSecondSource {
            urls: &[
                "https://legacy.aonprd.com/bestiary/boar.html",
                "https://www.d20pfsrd.com/bestiary/monster-listings/animals/boar/boar/",
            ],
            fetched_at: "2026-07-29",
            identity_match_basis: "name + CR 2 + Medium animal + Bestiary pg. 36 (matches the cached chassis's own source_page)",
        },
    },
    GroundedAttack {
        monster_key: "beastiary1:monster:cave_fisher",
        attack_name: "Claw",
        damage_dice: "1d4",
        corpus_name_token: "ABILITY:Internal|AUTOMATIC|Claw",
        published_melee_text: "2 claws +5 (1d4+3)",
        source: AttackSource::WebSecondSource {
            urls: &[
                "https://legacy.aonprd.com/bestiary/caveFisher.html",
                "https://www.d20pfsrd.com/bestiary/monster-listings/vermin/cave-fisher/",
            ],
            fetched_at: "2026-07-29",
            identity_match_basis: "name + CR 2 + Medium vermin + Bestiary pg. 41 (matches the cached chassis's own source_page); all sources place Filament on the Ranged line, matching the real row's own NATURALATTACKS:Filament,...Ranged... token",
        },
    },
    GroundedAttack {
        monster_key: "beastiary1:monster:centaur",
        attack_name: "Hoof",
        damage_dice: "1d6",
        corpus_name_token: "ABILITY:Internal|AUTOMATIC|Hoof",
        published_melee_text: "longsword +5 (1d8+2/19-20), 2 hooves +0 (1d6+1)",
        source: AttackSource::WebSecondSource {
            urls: &[
                "https://www.aonprd.com/MonsterDisplay.aspx?ItemName=Centaur",
                "https://www.d20pfsrd.com/bestiary/monster-listings/monstrous-humanoids/centaur/",
            ],
            fetched_at: "2026-07-29",
            identity_match_basis: "name + CR 3 + Large monstrous humanoid + Bestiary pg. 42 (matches the cached chassis's own source_page); the longsword is a manufactured weapon and is deliberately excluded, matching how Gnoll/Derro/Bugbear are handled",
        },
    },
    GroundedAttack {
        monster_key: "beastiary1:monster:choker",
        attack_name: "Tentacle",
        damage_dice: "1d4",
        corpus_name_token: "ABILITY:Internal|AUTOMATIC|Tentacle",
        published_melee_text: "2 tentacles +6 (1d4+3 plus grab)",
        source: AttackSource::WebSecondSource {
            urls: &[
                "https://www.aonprd.com/MonsterDisplay.aspx?ItemName=Choker",
                "https://www.d20pfsrd.com/bestiary/monster-listings/aberrations/choker/",
            ],
            fetched_at: "2026-07-29",
            identity_match_basis: "name + CR 2 + Small aberration + Bestiary pg. 45 (matches the cached chassis's own source_page); the published block has tentacles only -- no claw attack exists, so none is recorded",
        },
    },
    GroundedAttack {
        monster_key: "beastiary1:monster:cockatrice",
        attack_name: "Bite",
        damage_dice: "1d4",
        corpus_name_token: "ABILITY:Internal|AUTOMATIC|Bite",
        published_melee_text: "bite +9 (1d4-2 plus petrification)",
        source: AttackSource::WebSecondSource {
            urls: &[
                "https://www.aonprd.com/MonsterDisplay.aspx?ItemName=Cockatrice",
                "https://www.d20pfsrd.com/bestiary/monster-listings/magical-beasts/cockatrice/",
            ],
            fetched_at: "2026-07-29",
            identity_match_basis: "name + CR 3 + Small magical beast + Bestiary pg. 48 (matches the cached chassis's own source_page); the '-2' is the Strength penalty, not a die-size change",
        },
    },
    GroundedAttack {
        monster_key: "beastiary1:monster:crocodile",
        attack_name: "Bite",
        damage_dice: "1d8",
        corpus_name_token: "ABILITY:Internal|AUTOMATIC|Racial Traits ~ Crocodile -> ABILITY:Internal|AUTOMATIC|Bite (b1_abilities_race.lst:244)",
        published_melee_text: "bite +5 (1d8+4 plus grab) and tail slap +0 (1d12+2)",
        source: AttackSource::WebSecondSource {
            urls: &[
                "https://www.aonprd.com/MonsterDisplay.aspx?ItemName=Crocodile",
                "https://www.d20pfsrd.com/bestiary/monster-listings/animals/crocodilians/crocodile/",
            ],
            fetched_at: "2026-07-29",
            identity_match_basis: "name + CR 2 + Large animal + Bestiary pg. 51 (matches the cached chassis's own source_page)",
        },
    },
    // The one genuine corpus recovery in this table. Crocodile's row
    // reaches its attacks through `Racial Traits ~ Crocodile`
    // (`b1_abilities_race.lst:244`), whose own
    // `ABILITY:Internal|AUTOMATIC|Bite|Crocodile ~ Tail Slap` names both.
    // Unlike the generic `Bite` marker, `Crocodile ~ Tail Slap` carries a
    // real inline `NATURALATTACKS:` token WITH dice:
    //
    //   NATURALATTACKS:Tail Slap,Weapon.Natural.NaturalSecondary...
    //                  .Bludgeoning,*1,1d12
    //
    // so this value is transcribed from the corpus, not web-grounded.
    // The published text independently agrees ("tail slap +0 (1d12+2)"),
    // which is a three-way corroboration rather than the two-source
    // minimum.
    GroundedAttack {
        monster_key: "beastiary1:monster:crocodile",
        attack_name: "Tail Slap",
        damage_dice: "1d12",
        corpus_name_token: "ABILITY:Internal|AUTOMATIC|Racial Traits ~ Crocodile -> ABILITY:Internal|AUTOMATIC|Crocodile ~ Tail Slap",
        published_melee_text: "bite +5 (1d8+4 plus grab) and tail slap +0 (1d12+2)",
        source: AttackSource::LstToken {
            path: "pathfinder/paizo/roleplaying_game/bestiary/b1_abilities_race.lst",
            line: 248,
            record_key: "Crocodile ~ Tail Slap",
        },
    },
    GroundedAttack {
        monster_key: "beastiary1:monster:vargouille",
        attack_name: "Bite",
        damage_dice: "1d4",
        corpus_name_token: "ABILITY:Internal|AUTOMATIC|Bite",
        published_melee_text: "bite +5 (1d4 plus poison)",
        source: AttackSource::WebSecondSource {
            urls: &[
                "https://www.aonprd.com/MonsterDisplay.aspx?ItemName=Vargouille",
                "https://www.d20pfsrd.com/bestiary/monster-listings/outsiders/vargouille/",
            ],
            fetched_at: "2026-07-29",
            identity_match_basis: "name + CR 2 + Small outsider (evil, extraplanar) + Bestiary pg. 272 (matches the cached chassis's own source_page and RACESUBTYPE); 'kiss' and 'shriek' appear only under Special Attacks with no damage dice in any source, so neither is recorded as a natural attack",
        },
    },
    GroundedAttack {
        monster_key: "beastiary1:monster:wolf",
        attack_name: "Bite",
        damage_dice: "1d6",
        corpus_name_token: "ABILITY:Internal|AUTOMATIC|Bite",
        published_melee_text: "bite +2 (1d6+1 plus trip)",
        source: AttackSource::WebSecondSource {
            urls: &[
                "https://www.aonprd.com/MonsterDisplay.aspx?ItemName=Wolf",
                "https://www.d20pfsrd.com/bestiary/monster-listings/animals/canines/wolf/",
            ],
            fetched_at: "2026-07-29",
            identity_match_basis: "name + CR 1 + Medium animal + Bestiary pg. 278 (matches the cached chassis's own source_page); explicitly the plain Bestiary 1 wolf, NOT the dire wolf and NOT the wolf animal companion",
        },
    },
    GroundedAttack {
        monster_key: "beastiary1:monster:wolverine",
        attack_name: "Bite",
        damage_dice: "1d4",
        corpus_name_token: "ABILITY:Internal|AUTOMATIC|Bite|Claw",
        published_melee_text: "2 claws +4 (1d6+2), bite +4 (1d4+2)",
        source: AttackSource::WebSecondSource {
            urls: &[
                "https://www.aonprd.com/MonsterDisplay.aspx?ItemName=Wolverine",
                "https://www.d20pfsrd.com/bestiary/monster-listings/animals/wolverine/",
            ],
            fetched_at: "2026-07-29",
            identity_match_basis: "name + CR 2 + Medium animal + Bestiary pg. 279 (matches the cached chassis's own source_page)",
        },
    },
    GroundedAttack {
        monster_key: "beastiary1:monster:wolverine",
        attack_name: "Claw",
        damage_dice: "1d6",
        corpus_name_token: "ABILITY:Internal|AUTOMATIC|Bite|Claw",
        published_melee_text: "2 claws +4 (1d6+2), bite +4 (1d4+2)",
        source: AttackSource::WebSecondSource {
            urls: &[
                "https://www.aonprd.com/MonsterDisplay.aspx?ItemName=Wolverine",
                "https://www.d20pfsrd.com/bestiary/monster-listings/animals/wolverine/",
            ],
            fetched_at: "2026-07-29",
            identity_match_basis: "name + CR 2 + Medium animal + Bestiary pg. 279 (matches the cached chassis's own source_page)",
        },
    },
    GroundedAttack {
        monster_key: "beastiary1:monster:worg",
        attack_name: "Bite",
        damage_dice: "1d6",
        corpus_name_token: "ABILITY:Internal|AUTOMATIC|Bite",
        published_melee_text: "bite +7 (1d6+4 plus trip)",
        source: AttackSource::WebSecondSource {
            urls: &[
                "https://www.aonprd.com/MonsterDisplay.aspx?ItemName=Worg",
                "https://www.d20pfsrd.com/bestiary/monster-listings/magical-beasts/worg/",
            ],
            fetched_at: "2026-07-29",
            identity_match_basis: "name + CR 2 + Medium magical beast + Bestiary pg. 280 (matches the cached chassis's own source_page)",
        },
    },
];

/// Looks up the provenance rows for one monster, in table order.
pub fn provenance_for(monster_key: &str) -> Vec<&'static GroundedAttack> {
    GROUNDED_NATURAL_ATTACKS.iter().filter(|g| g.monster_key == monster_key).collect()
}
