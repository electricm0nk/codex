//! Hand-modelled evaluation of PCGen `PRE`-family prerequisite tokens.
//!
//! # What this is, and deliberately is not
//!
//! `decisions.md` §24 rules that formula-token content is hand-modelled
//! **per kind**, never run through a general interpreter, because "a
//! hand-modelled formula that is wrong is a failing test; a misinterpreted
//! token is a plausible number nobody checks". This module follows that
//! ruling exactly: [`evaluate_prerequisite_token`] is a `match` over the
//! token kinds that actually occur in the ingested corpus, each arm a small
//! pure function over already-known character facts. There is no expression
//! parser and no variable environment.
//!
//! The kinds were enumerated by command over all 690 catalog records'
//! gathered tokens rather than guessed -- see
//! `tests/sd27_feat_prerequisite_enforcement.rs`, which re-derives the
//! census from the live tables and fails if a kind appears that no arm
//! below names.
//!
//! # Three outcomes, not two
//!
//! The critical design property is that [`ClauseOutcome`] has a third
//! variant. A checker with only met/unmet has to decide what to do with a
//! prerequisite it cannot evaluate, and both answers are lies: silently
//! passing it claims the character qualifies when nobody checked, and
//! failing it greys out a feat the character may well be entitled to. This
//! module says [`ClauseOutcome::Unmodelled`] and names the token, so the
//! UI can offer the feat while telling the player what was not verified.
//!
//! Only a definitive [`ClauseOutcome::Unmet`] ever blocks.
//!
//! # Two PCGen variables are modelled; the other 44 are not
//!
//! `PREVARGTEQ:` and friends reference PCGen runtime variables. 46 distinct
//! ones appear across the catalog. Two are modelled, each because the
//! corpus itself pins its value for every character this product can build:
//!
//! * **`PreStatScore_<ABBR>`** (65 references) is defined by PF1's own
//!   `core_rulebook/cr__stats.lst`, one row per ability:
//!   `DEFINE:PreStatScore_DEX|0` plus
//!   `BONUS:VAR|PreStatScore_DEX|max(DEXSCORE,AltDEXSCORE)|TYPE=Base`, with
//!   `DEFINE:AltDEXSCORE|0` on the same row. So its base value *is* the
//!   ability score. It is modelled as exactly that.
//! * **`FeatDexRequirement`** (51 references) is defined `|0` on the same
//!   corpus rows and is raised by exactly three records in the entire PCGen
//!   data tree, none of which is in any of the six books this repo has
//!   ingested (they are in Adventurer's Guide, Ultimate Intrigue and a
//!   Player Companion volume). For a character built here it is 0, and
//!   `feat_dex_requirement_is_never_raised_by_any_ingested_book` re-checks
//!   that against the real corpus when `PCGEN_CORPUS_ROOT` is set.
//!
//! `CombatFeatIntRequirement` is deliberately **not** modelled even though
//! it looks identical, because ACG's Brawler (`Brawler's Cunning`),
//! Swashbuckler (`Swashbuckler Finesse`) and Daring Champion
//! (`Champion's Finesse`) really do raise it, and those are ingested,
//! selectable classes. A character with one of them genuinely qualifies for
//! Combat Expertise at Int 7, so treating the variable as 0 would produce
//! exactly the kind of confidently-wrong denial this module exists to
//! avoid.

use crate::rules_core::character_input::CharacterInput;
use crate::rules_core::feat_identity;
use crate::rules_core::size::SizeCategory;

/// The facts a prerequisite clause can be evaluated against.
///
/// Deliberately a plain snapshot rather than a `&CharacterInput`: the
/// character's base attack bonus is a *computed* value that only the pilot
/// compute path can produce (it spans CRB/APG/ACG/PU class chassis), so it
/// is handed in by the caller that already computed it rather than
/// re-derived here from a partial copy of the class tables. Everything else
/// is read straight off chosen input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterPrereqFacts {
    /// The `race:<slug>` token, verbatim.
    pub race_id: String,
    /// `(class_id, level)` for every class the character has levels in.
    pub class_levels: Vec<(String, u8)>,
    /// Sum of `class_levels`' levels. For a PC race (all 18 ingested ones)
    /// this is also the character's hit-dice count -- none of them grants
    /// racial hit dice.
    pub total_character_level: u8,
    /// The computed base attack bonus, handed in by the caller.
    pub base_attack_bonus: i16,
    /// Final ability scores as the sheet shows them. The desktop create
    /// flow already folds racial adjustments into the submitted scores
    /// (`composeCreateCharacterRequest.ts`: "every non-Human race's
    /// submitted score already includes its racial adjustment"), so this is
    /// the total PCGen's `DEXSCORE` means, not a pre-racial base.
    pub ability_scores: AbilityScoreSnapshot,
    /// `chosen.selected_feats`, verbatim -- compared through
    /// `feat_identity`, which folds the two real id shapes.
    pub selected_feats: Vec<String>,
    /// `(skill_id, ranks)` for every allocated skill.
    pub skill_ranks: Vec<(String, u8)>,
    /// The character's size, or `None` for a race this repo has not
    /// ingested a size for.
    pub size: Option<SizeCategory>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AbilityScoreSnapshot {
    pub strength: i16,
    pub dexterity: i16,
    pub constitution: i16,
    pub intelligence: i16,
    pub wisdom: i16,
    pub charisma: i16,
}

impl AbilityScoreSnapshot {
    /// PCGen's three-letter ability abbreviation, as `PRESTAT:` and
    /// `PreStatScore_<ABBR>` both spell it. `None` for anything else --
    /// there are exactly six abilities and no arm guesses.
    pub fn by_abbreviation(&self, abbreviation: &str) -> Option<i16> {
        match abbreviation.trim().to_ascii_uppercase().as_str() {
            "STR" => Some(self.strength),
            "DEX" => Some(self.dexterity),
            "CON" => Some(self.constitution),
            "INT" => Some(self.intelligence),
            "WIS" => Some(self.wisdom),
            "CHA" => Some(self.charisma),
            _ => None,
        }
    }
}

impl CharacterPrereqFacts {
    /// Builds the snapshot from chosen input plus the already-computed base
    /// attack bonus.
    pub fn from_character(input: &CharacterInput, base_attack_bonus: i16) -> Self {
        let scores = &input.chosen.ability_scores;
        let total_character_level = input
            .chosen
            .class_levels
            .iter()
            .map(|entry| u16::from(entry.level))
            .sum::<u16>()
            .min(u16::from(u8::MAX)) as u8;

        CharacterPrereqFacts {
            race_id: input.chosen.race_id.clone(),
            class_levels: input
                .chosen
                .class_levels
                .iter()
                .map(|entry| (entry.class_id.clone(), entry.level))
                .collect(),
            total_character_level,
            base_attack_bonus,
            ability_scores: AbilityScoreSnapshot {
                strength: scores.strength,
                dexterity: scores.dexterity,
                constitution: scores.constitution,
                intelligence: scores.intelligence,
                wisdom: scores.wisdom,
                charisma: scores.charisma,
            },
            selected_feats: input.chosen.selected_feats.clone(),
            skill_ranks: input
                .chosen
                .skill_allocations
                .iter()
                .map(|allocation| (allocation.skill_id.clone(), allocation.ranks))
                .collect(),
            size: crate::rules_core::race_resolver::race_size_for_race_token(
                &input.chosen.race_id,
            ),
        }
    }

    fn skill_ranks_in(&self, skill_name: &str) -> u8 {
        let needle = loose_fold(skill_name);
        self.skill_ranks
            .iter()
            .filter(|(skill_id, _)| loose_fold(strip_prefix_before_colon(skill_id)) == needle)
            .map(|(_, ranks)| *ranks)
            .max()
            .unwrap_or(0)
    }

    fn level_in_class(&self, class_name: &str) -> Option<u8> {
        let needle = loose_fold(class_name);
        self.class_levels
            .iter()
            .find(|(class_id, _)| loose_fold(strip_prefix_before_colon(class_id)) == needle)
            .map(|(_, level)| *level)
    }

    /// The `RACESUBTYPE:` values this character's race carries. See
    /// [`RACE_SUBTYPES`].
    fn race_subtypes(&self) -> &'static [&'static str] {
        let needle = loose_fold(strip_prefix_before_colon(&self.race_id));
        RACE_SUBTYPES
            .iter()
            .find(|(race, _)| loose_fold(race) == needle)
            .map(|(_, subtypes)| *subtypes)
            .unwrap_or(&[])
    }

    fn is_race(&self, race_name: &str) -> bool {
        loose_fold(strip_prefix_before_colon(&self.race_id)) == loose_fold(race_name)
    }
}

/// Every ingested race's `RACESUBTYPE:` values.
///
/// Hand-modelled per `decisions.md` §24, from the corpus's own race
/// templates (`core_essentials/races/<race>/<race>_templates.lst`, each
/// carrying the `RACESUBTYPE:` token verbatim; Half-Elf and Half-Orc carry
/// no template of their own and instead take `TEMPLATE:Elf|Human` and
/// `TEMPLATE:Orc|Human` off their race rows, which is why they have two).
/// Values are the corpus's, not PF1 prose:
///
/// * Aasimar and Tiefling are `Native`, not `Outsider` -- the corpus's own
///   `RACESUBTYPE:Native`.
/// * Duergar is `Dwarf`, Drow is `Elf`, Svirfneblin is `Gnome` -- each a
///   sub-race whose subtype is its parent's, which is exactly why
///   `PRERACE:1,RACESUBTYPE=Dwarf` on ARG's dwarven feats must reach a
///   Duergar.
/// * Goblin and Hobgoblin are `Goblinoid`, Kobold is `Reptilian`, Merfolk
///   is `Aquatic`.
///
/// Pinned against the real corpus by
/// `race_subtypes_match_the_corpus_race_templates` when
/// `PCGEN_CORPUS_ROOT` is set.
const RACE_SUBTYPES: &[(&str, &[&str])] = &[
    // Core Rulebook's 7.
    ("Dwarf", &["Dwarf"]),
    ("Elf", &["Elf"]),
    ("Gnome", &["Gnome"]),
    ("Half-Elf", &["Elf", "Human"]),
    ("Half-Orc", &["Orc", "Human"]),
    ("Halfling", &["Halfling"]),
    ("Human", &["Human"]),
    // Bestiary 1's 11.
    ("Aasimar", &["Native"]),
    ("Drow", &["Elf"]),
    ("Duergar", &["Dwarf"]),
    ("Goblin", &["Goblinoid"]),
    ("Hobgoblin", &["Goblinoid"]),
    ("Kobold", &["Reptilian"]),
    ("Merfolk", &["Aquatic"]),
    ("Orc", &["Orc"]),
    ("Svirfneblin", &["Gnome"]),
    ("Tengu", &["Tengu"]),
    ("Tiefling", &["Native"]),
];

/// What evaluating one prerequisite clause concluded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClauseOutcome {
    /// The character satisfies it. Carries the player-facing statement of
    /// what was satisfied, so a UI can show met prerequisites too.
    Met { requirement: String },
    /// The character definitively does not satisfy it. **Only this blocks.**
    /// `reason` names both the requirement and the character's actual value.
    Unmet { requirement: String, reason: String },
    /// This engine cannot evaluate the clause. Never blocks; always
    /// reported. `note` names the token verbatim so the gap is auditable
    /// rather than a shrug.
    Unmodelled { token: String, note: String },
    /// PCGen display-only prose (`PRETEXT:`). Carries no machine-checkable
    /// condition at all -- every `PRETEXT:` in the catalog sits alongside
    /// the real tokens for the same requirement. Neither blocks nor counts
    /// as a gap.
    Informational { text: String },
}

impl ClauseOutcome {
    pub fn blocks(&self) -> bool {
        matches!(self, ClauseOutcome::Unmet { .. })
    }
}

/// The token kind (`PREABILITY`, `!PREABILITY`, `PREMULT`, ...) of `token`,
/// or `None` if it is not a `PRE`-family token at all.
pub fn token_kind(token: &str) -> Option<&str> {
    let (kind, _) = token.split_once(':')?;
    if kind.trim_start_matches('!').starts_with("PRE") {
        Some(kind)
    } else {
        None
    }
}

/// Every kind [`evaluate_prerequisite_token`] evaluates to a real
/// met/unmet verdict. A kind absent from here always yields
/// [`ClauseOutcome::Unmodelled`] and can never block.
///
/// `!PRE...` negated forms are handled for the same set; the leading `!` is
/// stripped before dispatch.
pub const MODELLED_KINDS: &[&str] = &[
    "PREABILITY",
    "PRECLASS",
    "PREFACT",
    "PREHD",
    "PRELEVEL",
    "PREMULT",
    "PREPCLEVEL",
    "PRERACE",
    "PRESIZELTEQ",
    "PRESKILL",
    "PRESTAT",
    "PRETOTALAB",
    "PREVARGT",
    "PREVARGTEQ",
];

/// Kinds present in the catalog that are deliberately reported as
/// unmodelled rather than guessed at, each with the reason. Kept as data so
/// the report a caller renders and the census test read the same list.
pub const UNMODELLED_KINDS: &[(&str, &str)] = &[
    ("PREALIGN", "the character record carries no alignment"),
    ("PRECHECKBASE", "base saving-throw prerequisites are not modelled"),
    ("PREDEITYALIGN", "the character record carries no deity"),
    ("PREDOMAIN", "cleric domains are not modelled"),
    ("PREPROFWITHARMOR", "armor proficiency is not modelled"),
    ("PREPROFWITHSHIELD", "shield proficiency is not modelled"),
    ("PRESPELL", "known/prepared spell prerequisites are not modelled"),
    ("PRESPELLCAST", "spellcasting-style prerequisites are not modelled"),
    ("PRESPELLDESCRIPTOR", "spell-descriptor prerequisites are not modelled"),
    (
        "PRESPELLSCHOOLSUB",
        "spell-subschool prerequisites are not modelled",
    ),
    ("PRESPELLTYPE", "spell-type prerequisites are not modelled"),
    ("PREVAREQ", "the referenced PCGen variable is not modelled"),
    ("PREVARLT", "the referenced PCGen variable is not modelled"),
    ("PREVISION", "racial vision modes are not modelled"),
    ("PREWEAPONPROF", "weapon proficiency is not modelled"),
    ("PREMOVE", "movement-type prerequisites are not modelled"),
];

/// Evaluates one top-level `PRE`-family token against `facts`.
///
/// One `match` over the kinds the corpus actually contains, per
/// `decisions.md` §24. Anything unrecognised -- a kind not listed in
/// [`MODELLED_KINDS`], a body shape an arm does not recognise, or a PCGen
/// variable outside the two modelled families -- returns
/// [`ClauseOutcome::Unmodelled`] rather than a verdict.
pub fn evaluate_prerequisite_token(token: &str, facts: &CharacterPrereqFacts) -> ClauseOutcome {
    let token = token.trim();
    let Some((kind, body)) = token.split_once(':') else {
        return unmodelled(token, "not a PRE-family token");
    };
    let negated = kind.starts_with('!');
    let bare_kind = kind.trim_start_matches('!');

    let outcome = match bare_kind {
        "PREABILITY" => evaluate_ability(token, body, facts),
        "PRECLASS" => evaluate_class(token, body, facts),
        "PREFACT" => evaluate_fact(token, body, facts),
        "PREHD" => evaluate_min_max(token, body, facts, "hit dice"),
        "PRELEVEL" | "PREPCLEVEL" => evaluate_min_max(token, body, facts, "character level"),
        "PREMULT" => return evaluate_mult(token, body, facts, negated),
        "PRERACE" => evaluate_race(token, body, facts),
        "PRESIZELTEQ" => evaluate_size_lteq(token, body, facts),
        "PRESKILL" => evaluate_skill(token, body, facts),
        "PRESTAT" => evaluate_stat(token, body, facts),
        "PRETOTALAB" => evaluate_total_ab(token, body, facts),
        "PREVARGT" | "PREVARGTEQ" => evaluate_var(token, body, facts, bare_kind == "PREVARGTEQ"),
        "PRETEXT" => return ClauseOutcome::Informational { text: body.to_owned() },
        _ => {
            let note = UNMODELLED_KINDS
                .iter()
                .find(|(name, _)| *name == bare_kind)
                .map(|(_, reason)| *reason)
                .unwrap_or("this prerequisite kind has no landed evaluation path");
            return unmodelled(token, note);
        }
    };

    if negated {
        negate(outcome)
    } else {
        outcome
    }
}

/// `!PRE...` inverts a met/unmet verdict and leaves the other two alone --
/// negating "could not evaluate" would fabricate a verdict out of a gap.
fn negate(outcome: ClauseOutcome) -> ClauseOutcome {
    match outcome {
        ClauseOutcome::Met { requirement } => ClauseOutcome::Unmet {
            reason: format!("you must NOT satisfy: {requirement}"),
            requirement: format!("must not: {requirement}"),
        },
        ClauseOutcome::Unmet { requirement, .. } => {
            ClauseOutcome::Met { requirement: format!("must not: {requirement}") }
        }
        other => other,
    }
}

fn unmodelled(token: &str, note: &str) -> ClauseOutcome {
    ClauseOutcome::Unmodelled { token: token.to_owned(), note: note.to_owned() }
}

// ---------------------------------------------------------------------------
// Per-kind arms
// ---------------------------------------------------------------------------

/// `PREABILITY:N,[CHECKMULT,]CATEGORY=<cat>,<item>,<item>...`
///
/// Only `CATEGORY=FEAT` is evaluated. `CATEGORY=Special Ability`,
/// `CATEGORY=Archetype` and `CATEGORY=CLASS` name class features,
/// archetypes and class abilities the character record does not carry a
/// roster of, so they are reported rather than guessed.
///
/// Items are feat names, or `TYPE.<facet>` / `TYPE=<facet>`. A `TYPE.`
/// facet resolves only when it names one of the catalog's own category
/// strings (`Alignment`, `ItemCreation`, ...); the corpus's finer `TYPE:`
/// subtypes are not ingested anywhere, so those stay unmodelled.
fn evaluate_ability(token: &str, body: &str, facts: &CharacterPrereqFacts) -> ClauseOutcome {
    let mut parts = body.split(',');
    let Some(required) = parts.next().and_then(|n| n.trim().parse::<usize>().ok()) else {
        return unmodelled(token, "PREABILITY count is not a number");
    };
    let rest: Vec<&str> = parts.map(str::trim).collect();
    let check_multiple = rest.contains(&"CHECKMULT");
    let category = rest
        .iter()
        .find_map(|part| part.strip_prefix("CATEGORY="))
        .unwrap_or("");
    if !category.eq_ignore_ascii_case("FEAT") {
        return unmodelled(
            token,
            "only CATEGORY=FEAT prerequisites are modelled; this one names a class \
             feature, archetype or class ability the character record has no roster of",
        );
    }

    let items: Vec<&str> = rest
        .into_iter()
        .filter(|part| *part != "CHECKMULT" && !part.starts_with("CATEGORY="))
        .collect();
    if items.is_empty() {
        return unmodelled(token, "PREABILITY names no ability");
    }

    let mut held = 0usize;
    for item in &items {
        if let Some(facet) = item.strip_prefix("TYPE.").or_else(|| item.strip_prefix("TYPE=")) {
            match feats_held_in_category(facts, facet) {
                Some(count) => held += count,
                None => {
                    return unmodelled(
                        token,
                        "names a corpus TYPE: subtype finer than the ingested feat categories",
                    )
                }
            }
        } else if check_multiple {
            held += feat_identity::count(&facts.selected_feats, item);
        } else if feat_identity::holds(&facts.selected_feats, item) {
            held += 1;
        }
    }

    let requirement = if items.len() == 1 {
        format!("the {} feat", items[0])
    } else {
        format!("{required} of these feats: {}", items.join(", "))
    };
    if held >= required {
        ClauseOutcome::Met { requirement }
    } else {
        ClauseOutcome::Unmet {
            reason: format!(
                "requires {requirement} (you have {held} of the {required} needed)"
            ),
            requirement,
        }
    }
}

/// How many selected feats fall in the catalog category `facet` names, or
/// `None` when `facet` is not one of the catalog's category strings.
fn feats_held_in_category(facts: &CharacterPrereqFacts, facet: &str) -> Option<usize> {
    use crate::rules_core::rules_tables::feats_all::all_feat_tables;

    let mut is_known_category = false;
    let mut held = 0usize;
    for book in all_feat_tables() {
        for entry in book.entries {
            if !entry.category.eq_ignore_ascii_case(facet) {
                continue;
            }
            is_known_category = true;
            if feat_identity::holds(&facts.selected_feats, entry.key) {
                held += 1;
            }
        }
    }
    is_known_category.then_some(held)
}

/// `PRECLASS:N,<Class>=<level>[,<Class>=<level>...]`.
///
/// `SPELLCASTER`, `SPELLCASTER.Arcane` and `SPELLCASTER.Divine` are PCGen
/// class *types*, not classes, and no ingested table classifies the 27
/// classes that way -- reported, not guessed.
fn evaluate_class(token: &str, body: &str, facts: &CharacterPrereqFacts) -> ClauseOutcome {
    let mut parts = body.split(',');
    let Some(required) = parts.next().and_then(|n| n.trim().parse::<usize>().ok()) else {
        return unmodelled(token, "PRECLASS count is not a number");
    };

    let mut satisfied = 0usize;
    let mut labels = Vec::new();
    for part in parts {
        let Some((class_name, level)) = part.trim().split_once('=') else {
            return unmodelled(token, "PRECLASS entry is not <Class>=<level>");
        };
        let Ok(level) = level.trim().parse::<u8>() else {
            return unmodelled(token, "PRECLASS level is not a number");
        };
        if class_name.eq_ignore_ascii_case("SPELLCASTER")
            || class_name.to_ascii_uppercase().starts_with("SPELLCASTER.")
        {
            return unmodelled(
                token,
                "names the PCGen SPELLCASTER class type, which no ingested class table \
                 classifies",
            );
        }
        labels.push(format!("{class_name} {level}"));
        if facts.level_in_class(class_name).is_some_and(|held| held >= level) {
            satisfied += 1;
        }
    }

    let requirement = format!("{required} of: {}", labels.join(", "));
    if satisfied >= required {
        ClauseOutcome::Met { requirement }
    } else {
        ClauseOutcome::Unmet {
            reason: format!("requires {requirement}; your classes are {}", describe_classes(facts)),
            requirement,
        }
    }
}

fn describe_classes(facts: &CharacterPrereqFacts) -> String {
    if facts.class_levels.is_empty() {
        return "none".to_owned();
    }
    facts
        .class_levels
        .iter()
        .map(|(class_id, level)| format!("{} {level}", strip_prefix_before_colon(class_id)))
        .collect::<Vec<_>>()
        .join(", ")
}

/// `PREFACT:1,TEMPLATES,Is<Race>=true[,Is<Race>=true...]`
///
/// Every one of the 185 `PREFACT:` tokens in the catalog is this exact
/// shape: an `Is<Race>` template flag, which PCGen sets from the character's
/// race row (`TEMPLATE:IsTiefling` and siblings in each race's
/// `<race>_templates.lst`). It is therefore a race gate, and a race the
/// player did not pick definitively does not have the flag -- including the
/// 18 ARG races this repo has not ingested, which no character here can be.
fn evaluate_fact(token: &str, body: &str, facts: &CharacterPrereqFacts) -> ClauseOutcome {
    let mut parts = body.split(',');
    let Some(required) = parts.next().and_then(|n| n.trim().parse::<usize>().ok()) else {
        return unmodelled(token, "PREFACT count is not a number");
    };
    let Some(fact_set) = parts.next().map(str::trim) else {
        return unmodelled(token, "PREFACT names no fact set");
    };
    if !fact_set.eq_ignore_ascii_case("TEMPLATES") {
        return unmodelled(token, "only PREFACT over TEMPLATES is modelled");
    }

    let mut satisfied = 0usize;
    let mut labels = Vec::new();
    for part in parts {
        let Some((flag, value)) = part.trim().split_once('=') else {
            return unmodelled(token, "PREFACT entry is not <flag>=<value>");
        };
        let Some(race) = flag.trim().strip_prefix("Is") else {
            return unmodelled(token, "only Is<Race> template flags are modelled");
        };
        if !value.trim().eq_ignore_ascii_case("true") {
            return unmodelled(token, "only Is<Race>=true is modelled");
        }
        labels.push(race.to_owned());
        // `Is<Race>` is set by the race row itself, so it holds exactly
        // when the character IS that race -- including via a subtype-parent
        // race row (a Duergar carries the Dwarf template).
        if facts.is_race(race) || facts.race_subtypes().iter().any(|s| loose_fold(s) == loose_fold(race)) {
            satisfied += 1;
        }
    }

    let requirement = format!("race: {}", labels.join(" or "));
    if satisfied >= required {
        ClauseOutcome::Met { requirement }
    } else {
        ClauseOutcome::Unmet {
            reason: format!(
                "requires {requirement}; you are {}",
                strip_prefix_before_colon(&facts.race_id)
            ),
            requirement,
        }
    }
}

/// `PRERACE:N,<Race>` or `PRERACE:N,RACESUBTYPE=<subtype>[,...]`.
fn evaluate_race(token: &str, body: &str, facts: &CharacterPrereqFacts) -> ClauseOutcome {
    let mut parts = body.split(',');
    let Some(required) = parts.next().and_then(|n| n.trim().parse::<usize>().ok()) else {
        return unmodelled(token, "PRERACE count is not a number");
    };

    let mut satisfied = 0usize;
    let mut labels = Vec::new();
    for part in parts {
        let part = part.trim();
        if let Some(subtype) = part.strip_prefix("RACESUBTYPE=") {
            labels.push(format!("{subtype} subtype"));
            if facts.race_subtypes().iter().any(|held| loose_fold(held) == loose_fold(subtype)) {
                satisfied += 1;
            }
        } else if part.contains('=') {
            return unmodelled(token, "only plain race names and RACESUBTYPE= are modelled");
        } else {
            labels.push(part.to_owned());
            if facts.is_race(part) {
                satisfied += 1;
            }
        }
    }

    let requirement = format!("race: {}", labels.join(" or "));
    if satisfied >= required {
        ClauseOutcome::Met { requirement }
    } else {
        ClauseOutcome::Unmet {
            reason: format!(
                "requires {requirement}; you are {}",
                strip_prefix_before_colon(&facts.race_id)
            ),
            requirement,
        }
    }
}

/// `PRESTAT:N,<ABBR>=<score>[,<ABBR>=<score>...]` -- N of the listed
/// ability scores must be at least the given value.
fn evaluate_stat(token: &str, body: &str, facts: &CharacterPrereqFacts) -> ClauseOutcome {
    let mut parts = body.split(',');
    let Some(required) = parts.next().and_then(|n| n.trim().parse::<usize>().ok()) else {
        return unmodelled(token, "PRESTAT count is not a number");
    };

    let mut satisfied = 0usize;
    let mut labels = Vec::new();
    let mut held = Vec::new();
    for part in parts {
        let Some((abbreviation, score)) = part.trim().split_once('=') else {
            return unmodelled(token, "PRESTAT entry is not <ABBR>=<score>");
        };
        let (Some(actual), Ok(score)) = (
            facts.ability_scores.by_abbreviation(abbreviation),
            score.trim().parse::<i16>(),
        ) else {
            return unmodelled(token, "PRESTAT names an ability or score this arm cannot read");
        };
        labels.push(format!("{} {score}", abbreviation.trim()));
        held.push(format!("{} {actual}", abbreviation.trim()));
        if actual >= score {
            satisfied += 1;
        }
    }

    let requirement = labels.join(" and ");
    if satisfied >= required {
        ClauseOutcome::Met { requirement }
    } else {
        ClauseOutcome::Unmet {
            reason: format!("requires {requirement} (you have {})", held.join(", ")),
            requirement,
        }
    }
}

/// `PRESKILL:N,<Skill>=<ranks>[,...]` -- N of the listed skills must have
/// at least that many ranks. `TYPE.<facet>` skill selectors are not
/// modelled (the character record carries skill ids, not skill types).
fn evaluate_skill(token: &str, body: &str, facts: &CharacterPrereqFacts) -> ClauseOutcome {
    let mut parts = body.split(',');
    let Some(required) = parts.next().and_then(|n| n.trim().parse::<usize>().ok()) else {
        return unmodelled(token, "PRESKILL count is not a number");
    };

    let mut satisfied = 0usize;
    let mut labels = Vec::new();
    let mut held = Vec::new();
    for part in parts {
        let Some((skill, ranks)) = part.trim().rsplit_once('=') else {
            return unmodelled(token, "PRESKILL entry is not <Skill>=<ranks>");
        };
        if skill.starts_with("TYPE.") || skill.starts_with("TYPE=") {
            return unmodelled(
                token,
                "names a skill TYPE facet; the character record carries skill ids, not types",
            );
        }
        let Ok(ranks) = ranks.trim().parse::<u8>() else {
            return unmodelled(token, "PRESKILL rank count is not a number");
        };
        let actual = facts.skill_ranks_in(skill);
        labels.push(format!("{ranks} rank(s) in {skill}"));
        held.push(format!("{skill} {actual}"));
        if actual >= ranks {
            satisfied += 1;
        }
    }

    let requirement = labels.join(" and ");
    if satisfied >= required {
        ClauseOutcome::Met { requirement }
    } else {
        ClauseOutcome::Unmet {
            reason: format!("requires {requirement} (you have {})", held.join(", ")),
            requirement,
        }
    }
}

/// `PRETOTALAB:N` -- base attack bonus of at least N.
fn evaluate_total_ab(token: &str, body: &str, facts: &CharacterPrereqFacts) -> ClauseOutcome {
    let Ok(required) = body.trim().parse::<i16>() else {
        return unmodelled(token, "PRETOTALAB value is not a number");
    };
    let requirement = format!("base attack bonus +{required}");
    if facts.base_attack_bonus >= required {
        ClauseOutcome::Met { requirement }
    } else {
        ClauseOutcome::Unmet {
            reason: format!(
                "requires {requirement} (you have +{})",
                facts.base_attack_bonus
            ),
            requirement,
        }
    }
}

/// `PRELEVEL:MIN=N` / `PRELEVEL:MAX=N` / `PREPCLEVEL:...` / `PREHD:MIN=N`.
///
/// All four resolve to the character's total class levels: none of the 18
/// ingested races grants racial hit dice, so hit dice and character level
/// are the same number for every character this product can build.
fn evaluate_min_max(
    token: &str,
    body: &str,
    facts: &CharacterPrereqFacts,
    label: &str,
) -> ClauseOutcome {
    let Some((bound, value)) = body.trim().split_once('=') else {
        return unmodelled(token, "expected MIN=<n> or MAX=<n>");
    };
    let Ok(value) = value.trim().parse::<u8>() else {
        return unmodelled(token, "bound is not a number");
    };
    let actual = facts.total_character_level;
    match bound.trim().to_ascii_uppercase().as_str() {
        "MIN" => {
            let requirement = format!("{label} {value} or higher");
            if actual >= value {
                ClauseOutcome::Met { requirement }
            } else {
                ClauseOutcome::Unmet {
                    reason: format!("requires {requirement} (you are {actual})"),
                    requirement,
                }
            }
        }
        "MAX" => {
            let requirement = format!("{label} {value} or lower");
            if actual <= value {
                ClauseOutcome::Met { requirement }
            } else {
                ClauseOutcome::Unmet {
                    reason: format!("requires {requirement} (you are {actual})"),
                    requirement,
                }
            }
        }
        _ => unmodelled(token, "expected MIN or MAX"),
    }
}

/// `PRESIZELTEQ:<size code>` -- the character's size must be at most the
/// named one.
fn evaluate_size_lteq(token: &str, body: &str, facts: &CharacterPrereqFacts) -> ClauseOutcome {
    let Some(limit) = size_from_code(body.trim()) else {
        return unmodelled(token, "unrecognised size code");
    };
    let requirement = format!("size {} or smaller", size_label(limit));
    match facts.size {
        Some(size) if size_rank(size) <= size_rank(limit) => ClauseOutcome::Met { requirement },
        Some(size) => ClauseOutcome::Unmet {
            reason: format!("requires {requirement} (you are {})", size_label(size)),
            requirement,
        },
        // An un-ingested race's size is genuinely unknown here; saying so
        // beats defaulting to Medium and denying on an assumption.
        None => unmodelled(token, "this race's size is not ingested"),
    }
}

fn size_from_code(code: &str) -> Option<SizeCategory> {
    match code.to_ascii_uppercase().as_str() {
        "F" => Some(SizeCategory::Fine),
        "D" => Some(SizeCategory::Diminutive),
        "T" => Some(SizeCategory::Tiny),
        "S" => Some(SizeCategory::Small),
        "M" => Some(SizeCategory::Medium),
        "L" => Some(SizeCategory::Large),
        "H" => Some(SizeCategory::Huge),
        "G" => Some(SizeCategory::Gargantuan),
        "C" => Some(SizeCategory::Colossal),
        _ => None,
    }
}

fn size_rank(size: SizeCategory) -> i8 {
    match size {
        SizeCategory::Fine => 0,
        SizeCategory::Diminutive => 1,
        SizeCategory::Tiny => 2,
        SizeCategory::Small => 3,
        SizeCategory::Medium => 4,
        SizeCategory::Large => 5,
        SizeCategory::Huge => 6,
        SizeCategory::Gargantuan => 7,
        SizeCategory::Colossal => 8,
    }
}

fn size_label(size: SizeCategory) -> &'static str {
    match size {
        SizeCategory::Fine => "Fine",
        SizeCategory::Diminutive => "Diminutive",
        SizeCategory::Tiny => "Tiny",
        SizeCategory::Small => "Small",
        SizeCategory::Medium => "Medium",
        SizeCategory::Large => "Large",
        SizeCategory::Huge => "Huge",
        SizeCategory::Gargantuan => "Gargantuan",
        SizeCategory::Colossal => "Colossal",
    }
}

/// `PREVARGTEQ:<var>,<n>` / `PREVARGT:<var>,<n>`.
///
/// Only the two variable families this module's doc comment justifies are
/// modelled. Everything else is reported by name.
fn evaluate_var(
    token: &str,
    body: &str,
    facts: &CharacterPrereqFacts,
    or_equal: bool,
) -> ClauseOutcome {
    let Some((variable, value)) = body.trim().rsplit_once(',') else {
        return unmodelled(token, "expected <variable>,<value>");
    };
    let Ok(threshold) = value.trim().parse::<i16>() else {
        return unmodelled(token, "variable threshold is not a number");
    };
    let variable = variable.trim();

    let (actual, requirement) = if let Some(abbreviation) = variable.strip_prefix("PreStatScore_") {
        let Some(score) = facts.ability_scores.by_abbreviation(abbreviation) else {
            return unmodelled(token, "PreStatScore_ names no known ability");
        };
        (score, format!("{abbreviation} {threshold}"))
    } else if variable == "FeatDexRequirement" {
        // 0 for every character this product can build -- see the module
        // doc comment, and the corpus-gated test that re-checks it.
        (0, format!("a feature waiving the Dex {threshold} requirement"))
    } else {
        return unmodelled(
            token,
            "references a PCGen runtime variable this engine does not model",
        );
    };

    let met = if or_equal { actual >= threshold } else { actual > threshold };
    if met {
        ClauseOutcome::Met { requirement }
    } else {
        ClauseOutcome::Unmet {
            reason: format!("requires {requirement} (you have {actual})"),
            requirement,
        }
    }
}

/// `PREMULT:N,[clause],[clause],...` -- at least N of the bracketed
/// sub-clauses must hold.
///
/// The three-valued fold is where this earns its keep. If N sub-clauses are
/// met, the whole clause is met. Otherwise, if the met ones plus the ones
/// this engine could not evaluate could still reach N, the result is
/// *unmodelled*, not unmet -- because a character might satisfy the
/// alternative nobody checked. Only when even counting every unmodelled
/// sub-clause as satisfied still falls short is the clause a real block.
///
/// That is exactly the CRB Combat Expertise case:
/// `PREMULT:1,[PREVARGTEQ:PreStatScore_INT,13],[PREVARGTEQ:CombatFeatIntRequirement,13]`
/// on an Int 10 character is *not* a denial, because an ACG Brawler really
/// does qualify through the second clause.
fn evaluate_mult(
    token: &str,
    body: &str,
    facts: &CharacterPrereqFacts,
    negated: bool,
) -> ClauseOutcome {
    let Some((count, rest)) = body.split_once(',') else {
        return unmodelled(token, "PREMULT has no sub-clauses");
    };
    let Ok(required) = count.trim().parse::<usize>() else {
        return unmodelled(token, "PREMULT count is not a number");
    };
    let clauses = split_bracketed_clauses(rest);
    if clauses.is_empty() {
        return unmodelled(token, "PREMULT sub-clauses are not bracketed as expected");
    }

    let mut met = 0usize;
    let mut unknown = 0usize;
    let mut met_labels = Vec::new();
    let mut unmet_reasons = Vec::new();
    let mut unknown_notes = Vec::new();
    for clause in &clauses {
        match evaluate_prerequisite_token(clause, facts) {
            ClauseOutcome::Met { requirement } => {
                met += 1;
                met_labels.push(requirement);
            }
            ClauseOutcome::Unmet { reason, .. } => unmet_reasons.push(reason),
            ClauseOutcome::Unmodelled { note, .. } => {
                unknown += 1;
                unknown_notes.push(note);
            }
            ClauseOutcome::Informational { .. } => {}
        }
    }

    let requirement = format!("{required} of {} alternatives", clauses.len());
    let outcome = if met >= required {
        ClauseOutcome::Met { requirement: met_labels.join(" and ") }
    } else if met + unknown >= required {
        ClauseOutcome::Unmodelled {
            token: token.to_owned(),
            note: format!(
                "one of its alternatives could not be evaluated ({}), so this is not treated \
                 as a denial",
                unknown_notes.join("; ")
            ),
        }
    } else {
        ClauseOutcome::Unmet { reason: unmet_reasons.join("; "), requirement }
    };

    if negated {
        negate(outcome)
    } else {
        outcome
    }
}

/// Splits `[a,b],[c]` into `["a,b", "c"]`, honouring nested brackets so a
/// `PREMULT:` inside a `PREMULT:` stays one clause.
fn split_bracketed_clauses(rest: &str) -> Vec<String> {
    let mut clauses = Vec::new();
    let mut depth = 0usize;
    let mut current = String::new();
    for character in rest.chars() {
        match character {
            '[' => {
                depth += 1;
                if depth > 1 {
                    current.push(character);
                }
            }
            ']' => {
                if depth == 0 {
                    return Vec::new();
                }
                depth -= 1;
                if depth == 0 {
                    clauses.push(std::mem::take(&mut current));
                } else {
                    current.push(character);
                }
            }
            _ if depth > 0 => current.push(character),
            _ => {}
        }
    }
    if depth != 0 {
        return Vec::new();
    }
    clauses
}

// ---------------------------------------------------------------------------
// Shared identity folding
// ---------------------------------------------------------------------------

/// Drops a `race:` / `class:` / `skill:` style prefix. Matches the
/// `<kind>:<slug>` idiom `CharacterInput` uses for every identity field.
fn strip_prefix_before_colon(id: &str) -> &str {
    id.split_once(':').map(|(_, rest)| rest).unwrap_or(id)
}

/// Lowercase ASCII-alphanumeric fold, the same rule `feat_identity` uses
/// for feats -- so `"skill:disable_device"` and `"Disable Device"` compare
/// equal, and `"class:half-elf"` and `"Half-Elf"` do too.
fn loose_fold(raw: &str) -> String {
    raw.chars().filter(char::is_ascii_alphanumeric).map(|c| c.to_ascii_lowercase()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fighter(level: u8, dexterity: i16, feats: &[&str]) -> CharacterPrereqFacts {
        CharacterPrereqFacts {
            race_id: "race:human".to_owned(),
            class_levels: vec![("class:fighter".to_owned(), level)],
            total_character_level: level,
            base_attack_bonus: i16::from(level),
            ability_scores: AbilityScoreSnapshot {
                strength: 14,
                dexterity,
                constitution: 12,
                intelligence: 10,
                wisdom: 10,
                charisma: 8,
            },
            selected_feats: feats.iter().map(|f| (*f).to_owned()).collect(),
            skill_ranks: Vec::new(),
            size: Some(SizeCategory::Medium),
        }
    }

    /// The exact defect this work exists to close, at the clause level.
    #[test]
    fn a_fighter_1_fails_every_improved_two_weapon_fighting_clause() {
        let facts = fighter(1, 13, &[]);
        assert!(matches!(
            evaluate_prerequisite_token("PRETOTALAB:6", &facts),
            ClauseOutcome::Unmet { .. }
        ));
        assert!(matches!(
            evaluate_prerequisite_token(
                "PREABILITY:1,CATEGORY=FEAT,Two-Weapon Fighting",
                &facts
            ),
            ClauseOutcome::Unmet { .. }
        ));
        assert!(matches!(
            evaluate_prerequisite_token(
                "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,17],[PREVARGTEQ:FeatDexRequirement,17]",
                &facts
            ),
            ClauseOutcome::Unmet { .. }
        ));
    }

    #[test]
    fn a_fighter_6_with_dex_17_and_twf_meets_every_clause() {
        let facts = fighter(6, 17, &["Two-Weapon Fighting"]);
        for token in [
            "PRETOTALAB:6",
            "PREABILITY:1,CATEGORY=FEAT,Two-Weapon Fighting",
            "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,17],[PREVARGTEQ:FeatDexRequirement,17]",
        ] {
            assert!(
                matches!(evaluate_prerequisite_token(token, &facts), ClauseOutcome::Met { .. }),
                "{token} should be met, got {:?}",
                evaluate_prerequisite_token(token, &facts)
            );
        }
    }

    /// The engine-token feat id shape must satisfy a catalog-key
    /// prerequisite -- both shapes really occur in `selected_feats`.
    #[test]
    fn the_engine_feat_token_shape_satisfies_a_catalog_key_prerequisite() {
        let facts = fighter(6, 17, &["feat:two_weapon_fighting"]);
        assert!(matches!(
            evaluate_prerequisite_token("PREABILITY:1,CATEGORY=FEAT,Two-Weapon Fighting", &facts),
            ClauseOutcome::Met { .. }
        ));
    }

    /// A PREMULT whose only unsatisfied alternative is unmodelled must not
    /// deny. This is CRB Combat Expertise on an Int 10 character: an ACG
    /// Brawler qualifies through `CombatFeatIntRequirement`.
    #[test]
    fn a_premult_with_an_unmodelled_alternative_reports_rather_than_denies() {
        let facts = fighter(1, 13, &[]);
        let outcome = evaluate_prerequisite_token(
            "PREMULT:1,[PREVARGTEQ:PreStatScore_INT,13],[PREVARGTEQ:CombatFeatIntRequirement,13]",
            &facts,
        );
        assert!(matches!(outcome, ClauseOutcome::Unmodelled { .. }), "{outcome:?}");
        assert!(!outcome.blocks());
    }

    #[test]
    fn a_special_ability_prerequisite_is_reported_not_guessed() {
        let facts = fighter(1, 13, &[]);
        let outcome = evaluate_prerequisite_token(
            "PREABILITY:1,CATEGORY=Special Ability,TYPE.Bardic Performance",
            &facts,
        );
        assert!(matches!(outcome, ClauseOutcome::Unmodelled { .. }), "{outcome:?}");
        assert!(!outcome.blocks());
    }

    #[test]
    fn pretext_is_informational_and_never_a_gap_or_a_block() {
        let facts = fighter(1, 13, &[]);
        let outcome =
            evaluate_prerequisite_token("PRETEXT:Prerequisite: Base attack bonus +1.", &facts);
        assert!(matches!(outcome, ClauseOutcome::Informational { .. }));
        assert!(!outcome.blocks());
    }

    /// ARG's race gates are the largest single prerequisite family in the
    /// catalog (185 `PREFACT:` tokens) and had no data at all before this
    /// cycle.
    #[test]
    fn a_race_gate_denies_the_wrong_race_and_admits_the_right_one() {
        let human = fighter(1, 13, &[]);
        let denial = evaluate_prerequisite_token("PREFACT:1,TEMPLATES,IsTiefling=true", &human);
        match &denial {
            ClauseOutcome::Unmet { reason, .. } => {
                assert!(reason.contains("Tiefling"), "{reason}");
                assert!(reason.contains("human"), "{reason}");
            }
            other => panic!("expected a denial, got {other:?}"),
        }

        let mut tiefling = fighter(1, 13, &[]);
        tiefling.race_id = "race:tiefling".to_owned();
        assert!(matches!(
            evaluate_prerequisite_token("PREFACT:1,TEMPLATES,IsTiefling=true", &tiefling),
            ClauseOutcome::Met { .. }
        ));
    }

    /// A Duergar carries the Dwarf template and the Dwarf subtype, so
    /// dwarven-heritage feats must reach it. Checking a real corpus
    /// consequence, not the table's literal contents.
    #[test]
    fn a_duergar_satisfies_a_dwarf_subtype_prerequisite() {
        let mut duergar = fighter(1, 13, &[]);
        duergar.race_id = "race:duergar".to_owned();
        assert!(matches!(
            evaluate_prerequisite_token("PRERACE:1,RACESUBTYPE=Dwarf", &duergar),
            ClauseOutcome::Met { .. }
        ));
        assert!(matches!(
            evaluate_prerequisite_token("PRERACE:1,RACESUBTYPE=Elf", &duergar),
            ClauseOutcome::Unmet { .. }
        ));
    }

    #[test]
    fn skill_rank_prerequisites_read_the_characters_allocations() {
        let mut facts = fighter(3, 13, &[]);
        facts.skill_ranks = vec![("skill:intimidate".to_owned(), 3)];
        assert!(matches!(
            evaluate_prerequisite_token("PRESKILL:1,Intimidate=3", &facts),
            ClauseOutcome::Met { .. }
        ));
        match evaluate_prerequisite_token("PRESKILL:1,Intimidate=5", &facts) {
            ClauseOutcome::Unmet { reason, .. } => assert!(reason.contains("Intimidate 3"), "{reason}"),
            other => panic!("expected a denial, got {other:?}"),
        }
    }

    #[test]
    fn a_negated_feat_prerequisite_inverts_the_verdict() {
        let without = fighter(1, 13, &[]);
        assert!(matches!(
            evaluate_prerequisite_token("!PREABILITY:1,CATEGORY=FEAT,Extra Panache", &without),
            ClauseOutcome::Met { .. }
        ));
        let with = fighter(1, 13, &["Extra Panache"]);
        assert!(matches!(
            evaluate_prerequisite_token("!PREABILITY:1,CATEGORY=FEAT,Extra Panache", &with),
            ClauseOutcome::Unmet { .. }
        ));
    }

    /// Negating something that could not be evaluated must stay
    /// unevaluated, not flip into a fabricated verdict.
    #[test]
    fn negating_an_unmodelled_clause_stays_unmodelled() {
        let facts = fighter(1, 13, &[]);
        let outcome = evaluate_prerequisite_token(
            "!PREABILITY:1,CATEGORY=Special Ability,TYPE.Panache",
            &facts,
        );
        assert!(matches!(outcome, ClauseOutcome::Unmodelled { .. }), "{outcome:?}");
    }

    #[test]
    fn nested_bracket_clauses_split_without_losing_the_inner_ones() {
        let clauses = split_bracketed_clauses(
            "[PREABILITY:1,CATEGORY=FEAT,A],[PREMULT:1,[PRESTAT:1,INT=13],[PRECLASS:1,Fighter=1]]",
        );
        assert_eq!(clauses.len(), 2);
        assert_eq!(clauses[0], "PREABILITY:1,CATEGORY=FEAT,A");
        assert_eq!(
            clauses[1],
            "PREMULT:1,[PRESTAT:1,INT=13],[PRECLASS:1,Fighter=1]"
        );
    }

    #[test]
    fn an_unrecognised_kind_never_blocks() {
        let facts = fighter(1, 13, &[]);
        for token in ["PREALIGN:LG", "PREWEAPONPROF:1,TYPE.Martial", "PREVISION:1,Darkvision=60"] {
            let outcome = evaluate_prerequisite_token(token, &facts);
            assert!(!outcome.blocks(), "{token} must not block: {outcome:?}");
            assert!(matches!(outcome, ClauseOutcome::Unmodelled { .. }));
        }
    }
}
