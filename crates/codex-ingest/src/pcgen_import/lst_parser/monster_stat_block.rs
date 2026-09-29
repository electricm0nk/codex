//! LST bare-row monster stat-block parser (SD-22 Epic 5 — Bestiary 1).
//!
//! ## Why this parser exists
//!
//! [`super::race_ability`]'s `parse_lst_entry` recognizes exactly two
//! directive shapes: `RACE:`/`RACES:` pointer lines (used in PCC campaign
//! files) and `ABILITY:` declarations. Neither shape matches how the real
//! Bestiary 1 monster stat blocks are written in
//! `pathfinder/paizo/roleplaying_game/bestiary/b1_races.lst`: a monster
//! record there is a **bare tab-delimited row** with the monster's name as
//! the unprefixed first field, followed by `KEY:VALUE` tab-separated
//! tokens — there is no `RACE:` (or any other) key prefixing the record
//! itself. `race_ability.rs`'s parser extracts zero records from this file
//! shape (confirmed: `grep -c "RACE:" b1_races.lst` → 0), which blocked
//! Epic 5's first cycle (`docs/release/SD-22/progress.md`, the
//! `E3.6-9 / E4.10-13 / E5.14-17` "new parsing code required" blocker
//! entry). This module is the new, bounded parser that gap calls for —
//! kept as a sibling module to `race_ability.rs` rather than folded into
//! it, so `race_ability.rs`'s own documented scope boundary ("does not
//! parse or semantically interpret other LST object kinds") stays true.
//!
//! ## Recognition surface
//!
//! A line is a monster stat-block record when, after trimming and
//! skipping blank/`#`-comment lines, its first tab-delimited field is:
//! - non-empty,
//! - not one of the directive prefixes owned by sibling parsers
//!   (`RACE:`, `RACES:`, `ABILITY:`),
//! - not itself a `KEY:VALUE` token (i.e. does not contain `:`),
//! - not a `.MOD` or `.COPY=<target>` suffixed name — those rows only
//!   carry a partial override layered onto a base record defined
//!   elsewhere (e.g. `Goblin.MOD` in `b1_races_pc.lst` layers onto the
//!   *playable-race* `Goblin` record in
//!   `core_essentials/races/goblin/goblin_races.lst`, not a Bestiary 1
//!   monster stat block) — parsing one as a fresh, standalone record
//!   would fabricate data this parser never actually read.
//!
//! Scope is further bounded to rows that carry a `CR:` token — a monster
//! *stat block* always publishes a Challenge Rating; rows without one
//! (per-level template-application shims like the bare `Skeleton` /
//! `Zombie` rows at `b1_races.lst:439-440`, which exist only to attach a
//! template onto a base creature and carry no combat data of their own)
//! are out of scope for this parser and are skipped without a
//! diagnostic, mirroring `race_ability.rs`'s own "ignored, not
//! diagnosed" treatment of out-of-scope lines.
//!
//! ## Field coverage (deliberately bounded)
//!
//! Mirrors `rules_tables::crb::class_tables`'s and every SD-22 Epic 3/4
//! class chassis module's scope boundary: only the fields *literally
//! present as tokens on the row* are transcribed here (`SIZE:`, `MOVE:`
//! walk speed, `CR:`, `RACETYPE:`, `RACESUBTYPE:`, `SOURCEPAGE:`,
//! `NATURALATTACKS:`). Fields that PCGen computes at runtime from other
//! sources (AC, HP, Fort/Ref/Will saves, skill totals) are **not** literal
//! tokens on a bare monster row — deriving them would require walking the
//! monster's `MONSTERCLASS:` hit-dice table and ability-score modifiers,
//! which is a distinct, larger ingest slice. Transcribing invented values
//! for those fields would be exactly the fabricated-data risk `AGENTS.md`
//! and the CRB precedent both rule out; they are left for a future
//! ingest slice instead.

/// A single real-world natural-weapon attack parsed from a
/// `NATURALATTACKS:` token, e.g. `Claw,Weapon.Natural...,*2,1d6` yields
/// `NaturalAttack { name: "Claw", damage_dice: "1d6" }`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NaturalAttack {
    pub name: String,
    pub damage_dice: String,
}

/// A monster's Challenge Rating, preserving the real corpus's whole and
/// fractional (`N/D`) forms (e.g. `CR:1` vs. `CR:1/3`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChallengeRating {
    pub numerator: u32,
    pub denominator: u32,
}

impl ChallengeRating {
    pub fn as_f32(&self) -> f32 {
        self.numerator as f32 / self.denominator as f32
    }
}

/// A parsed bare tab-delimited monster stat-block row.
#[derive(Debug, Clone, PartialEq)]
pub struct MonsterStatBlockRecord {
    /// Identity of the LST source file this record originates from.
    pub source_path: String,
    /// One-based line number of the record in the parsed input.
    pub line_number: usize,
    /// The full raw record line, preserved verbatim as evidence.
    pub raw_directive: String,
    /// The monster's name (the unprefixed first tab field).
    pub name: String,
    /// `SIZE:` token value, e.g. `"M"`.
    pub size: Option<String>,
    /// Walk speed in feet/round, from the first `Walk,<N>` pair in the
    /// `MOVE:` token (a monster may have multiple movement modes; only
    /// the walk speed is extracted here).
    pub speed_ft: Option<u32>,
    /// `CR:` token value.
    pub challenge_rating: ChallengeRating,
    /// `RACETYPE:` token value.
    pub race_type: Option<String>,
    /// `RACESUBTYPE:` token value.
    pub race_subtype: Option<String>,
    /// `SOURCEPAGE:` token value.
    pub source_page: Option<String>,
    /// Natural-weapon attacks, from one or more `NATURALATTACKS:` tokens
    /// (a row may carry several natural-attack tokens across separate
    /// tab fields, and/or several `|`-separated attacks within one
    /// token).
    pub natural_attacks: Vec<NaturalAttack>,
}

/// Parse `input_text` (the contents of a bestiary `<book>_races.lst` file)
/// into the monster stat-block records it contains.
///
/// Walks the input once, line-by-line. Lines outside this parser's scope
/// (blank lines, `#`-comments, `RACE:`/`RACES:`/`ABILITY:`-prefixed
/// directive lines owned by [`super::race_ability`], `.MOD`/`.COPY=`
/// override rows, and rows with no `CR:` token) are skipped without a
/// diagnostic — mirroring `race_ability.rs`'s own treatment of
/// out-of-scope lines.
pub fn parse_monster_stat_block_entries(
    source_path: &str,
    input_text: &str,
) -> Vec<MonsterStatBlockRecord> {
    let mut records = Vec::new();

    for (index, raw_line) in input_text.lines().enumerate() {
        let line_number = index + 1;
        let content = raw_line.trim_start();

        if content.is_empty() || content.starts_with('#') {
            continue;
        }

        // Directive-prefixed lines are owned by sibling parsers.
        if content.starts_with("RACE:")
            || content.starts_with("RACES:")
            || content.starts_with("ABILITY:")
        {
            continue;
        }

        let mut fields = raw_line.split('\t');
        let Some(name_field) = fields.next() else {
            continue;
        };
        let name_field = name_field.trim();
        if name_field.is_empty() || name_field.contains(':') {
            continue;
        }
        if name_field.ends_with(".MOD") || name_field.contains(".COPY=") {
            continue;
        }

        let mut size = None;
        let mut speed_ft = None;
        let mut challenge_rating = None;
        let mut race_type = None;
        let mut race_subtype = None;
        let mut source_page = None;
        let mut natural_attacks = Vec::new();

        for field in fields {
            let field = field.trim();
            if field.is_empty() {
                continue;
            }
            let Some((key, value)) = field.split_once(':') else {
                continue;
            };
            match key {
                "SIZE" => size = Some(value.to_string()),
                "MOVE" => speed_ft = speed_ft.or_else(|| parse_walk_speed(value)),
                "CR" => challenge_rating = parse_challenge_rating(value),
                "RACETYPE" => race_type = Some(value.to_string()),
                "RACESUBTYPE" => race_subtype = Some(value.to_string()),
                "SOURCEPAGE" => source_page = Some(value.to_string()),
                "NATURALATTACKS" => natural_attacks.extend(parse_natural_attacks(value)),
                _ => {}
            }
        }

        // A monster stat block always publishes a CR; rows without one
        // are template-application shims or other out-of-scope shapes.
        let Some(challenge_rating) = challenge_rating else {
            continue;
        };

        records.push(MonsterStatBlockRecord {
            source_path: source_path.to_string(),
            line_number,
            raw_directive: raw_line.to_string(),
            name: name_field.to_string(),
            size,
            speed_ft,
            challenge_rating,
            race_type,
            race_subtype,
            source_page,
            natural_attacks,
        });
    }

    records
}

/// Extracts the walk speed from a `MOVE:` token value, e.g.
/// `"Walk,30"` or `"Walk,30,Swim,15"` both yield `Some(30)`.
fn parse_walk_speed(value: &str) -> Option<u32> {
    let parts: Vec<&str> = value.split(',').collect();
    let mut pair = parts.chunks_exact(2);
    pair.find_map(|chunk| {
        if chunk[0].trim().eq_ignore_ascii_case("walk") {
            chunk[1].trim().parse::<u32>().ok()
        } else {
            None
        }
    })
}

/// Parses a `CR:` token value, e.g. `"1"` -> `1/1`, `"1/3"` -> `1/3`.
fn parse_challenge_rating(value: &str) -> Option<ChallengeRating> {
    let value = value.trim();
    if let Some((n, d)) = value.split_once('/') {
        let numerator: u32 = n.trim().parse().ok()?;
        let denominator: u32 = d.trim().parse().ok()?;
        if denominator == 0 {
            return None;
        }
        Some(ChallengeRating { numerator, denominator })
    } else {
        let numerator: u32 = value.parse().ok()?;
        Some(ChallengeRating { numerator, denominator: 1 })
    }
}

/// Parses a `NATURALATTACKS:` token value into its `NaturalAttack`
/// entries. A single token may carry several `|`-separated attacks, e.g.
/// `"Claw,Weapon.Natural...,*1,1d4|Bite,Weapon.Natural...,*1,1d4"`.
fn parse_natural_attacks(value: &str) -> Vec<NaturalAttack> {
    value
        .split('|')
        .filter_map(|entry| {
            let parts: Vec<&str> = entry.split(',').collect();
            let name = parts.first()?.trim();
            let damage_dice = parts.last()?.trim();
            if name.is_empty() || damage_dice.is_empty() {
                return None;
            }
            Some(NaturalAttack {
                name: name.to_string(),
                damage_dice: damage_dice.to_string(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    //! Internal smoke checks against small synthetic rows. The
    //! authoritative real-corpus acceptance surface lives in
    //! `tests/sd17_b_monster_stat_block.rs`.
    use super::*;

    #[test]
    fn parses_a_bare_row_with_single_natural_attack() {
        let input = "Goblin Dog\tSTARTFEATS:1\tSIZE:M\tMOVE:Walk,50\tNATURALATTACKS:Bite,Weapon.Natural,*1,1d6\tRACETYPE:Animal\tCR:1\tSOURCEPAGE:p.157\n";
        let records = parse_monster_stat_block_entries("b1_races.lst", input);
        assert_eq!(records.len(), 1);
        let record = &records[0];
        assert_eq!(record.name, "Goblin Dog");
        assert_eq!(record.size.as_deref(), Some("M"));
        assert_eq!(record.speed_ft, Some(50));
        assert_eq!(record.challenge_rating.as_f32(), 1.0);
        assert_eq!(record.race_type.as_deref(), Some("Animal"));
        assert_eq!(record.source_page.as_deref(), Some("p.157"));
        assert_eq!(
            record.natural_attacks,
            vec![NaturalAttack { name: "Bite".to_string(), damage_dice: "1d6".to_string() }]
        );
    }

    #[test]
    fn parses_pipe_separated_natural_attacks_in_one_token() {
        let input = "Lizardfolk\tSIZE:M\tNATURALATTACKS:Claw,Weapon.Natural,*1,1d4|Bite,Weapon.Natural,*1,1d4\tRACETYPE:Humanoid\tCR:1\n";
        let records = parse_monster_stat_block_entries("b1_races.lst", input);
        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0].natural_attacks,
            vec![
                NaturalAttack { name: "Claw".to_string(), damage_dice: "1d4".to_string() },
                NaturalAttack { name: "Bite".to_string(), damage_dice: "1d4".to_string() },
            ]
        );
    }

    #[test]
    fn ignores_race_pointer_and_ability_and_comment_and_blank_lines() {
        let input = "RACE:cr_races.lst\nABILITY:Foo|AUTOMATIC|Bar\n# a comment\n\n";
        let records = parse_monster_stat_block_entries("b1_races.lst", input);
        assert!(records.is_empty());
    }

    #[test]
    fn ignores_mod_and_copy_override_rows() {
        let input = "Goblin.MOD\tSOURCEPAGE:p.156\tCR:1\nHydra.COPY=Hydra (Cryohydra)\tCR:1\n";
        let records = parse_monster_stat_block_entries("b1_races.lst", input);
        assert!(records.is_empty());
    }

    #[test]
    fn ignores_rows_with_no_cr_token_as_out_of_scope() {
        // Mirrors the real b1_races.lst `Skeleton` / `Zombie` template
        // shim rows (no CR of their own).
        let input = "Skeleton\tMOVE:Walk,0\tRACETYPE:Undead\n";
        let records = parse_monster_stat_block_entries("b1_races.lst", input);
        assert!(records.is_empty());
    }

    #[test]
    fn parses_fractional_challenge_rating() {
        let input = "Skeleton (Human)\tSIZE:M\tCR:1/3\n";
        let records = parse_monster_stat_block_entries("b1_races.lst", input);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].challenge_rating.numerator, 1);
        assert_eq!(records[0].challenge_rating.denominator, 3);
        assert!((records[0].challenge_rating.as_f32() - (1.0 / 3.0)).abs() < 1e-6);
    }
}
