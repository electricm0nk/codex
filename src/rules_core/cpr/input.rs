//! `CprCharacterInput`: a Cyberpunk RED character's own choices, and the text grammar of
//! `cpr_character_input.txt` (one `key = value` line each).
//!
//! E0.4 holds the ten STATs (Cyberpunk RED core rulebook p. 72: "Ten Primary Statistics");
//! the engine and creation cards add the rest of the input (method, Roles, skills, kit,
//! cyberware) as new keys.

use std::fmt::Write as _;

/// The ten STATs, in the order the book's STAT templates list them (p. 74-77 column order).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CprStat {
    Int,
    Ref,
    Dex,
    Tech,
    Cool,
    Will,
    Luck,
    Move,
    Body,
    Emp,
}

impl CprStat {
    pub const ALL: [CprStat; 10] = [
        CprStat::Int,
        CprStat::Ref,
        CprStat::Dex,
        CprStat::Tech,
        CprStat::Cool,
        CprStat::Will,
        CprStat::Luck,
        CprStat::Move,
        CprStat::Body,
        CprStat::Emp,
    ];

    /// The lower-case id used in `cpr_character_input.txt` keys and `cpr.stat.<id>` rows.
    pub const fn id(self) -> &'static str {
        match self {
            CprStat::Int => "int",
            CprStat::Ref => "ref",
            CprStat::Dex => "dex",
            CprStat::Tech => "tech",
            CprStat::Cool => "cool",
            CprStat::Will => "will",
            CprStat::Luck => "luck",
            CprStat::Move => "move",
            CprStat::Body => "body",
            CprStat::Emp => "emp",
        }
    }

    /// The book's abbreviation.
    pub const fn label(self) -> &'static str {
        match self {
            CprStat::Int => "INT",
            CprStat::Ref => "REF",
            CprStat::Dex => "DEX",
            CprStat::Tech => "TECH",
            CprStat::Cool => "COOL",
            CprStat::Will => "WILL",
            CprStat::Luck => "LUCK",
            CprStat::Move => "MOVE",
            CprStat::Body => "BODY",
            CprStat::Emp => "EMP",
        }
    }

    /// The book's full name.
    pub const fn name(self) -> &'static str {
        match self {
            CprStat::Int => "Intelligence",
            CprStat::Ref => "Reflexes",
            CprStat::Dex => "Dexterity",
            CprStat::Tech => "Technique",
            CprStat::Cool => "Cool",
            CprStat::Will => "Willpower",
            CprStat::Luck => "Luck",
            CprStat::Move => "Movement",
            CprStat::Body => "Body",
            CprStat::Emp => "Empathy",
        }
    }

    const fn index(self) -> usize {
        self as usize
    }
}

/// One value per STAT, indexed by [`CprStat`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CprStats([u8; 10]);

impl CprStats {
    pub fn get(&self, stat: CprStat) -> u8 {
        self.0[stat.index()]
    }

    pub fn set(&mut self, stat: CprStat, value: u8) {
        self.0[stat.index()] = value;
    }
}

/// A Cyberpunk RED character's choices: what `cpr_character_input.txt` holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CprCharacterInput {
    pub stats: CprStats,
}

const FILE: &str = "cpr_character_input.txt";

/// The `cpr_character_input.txt` text: `stat.<id> = <value>`, one line per STAT, book order.
pub fn render_cpr_character_input(input: &CprCharacterInput) -> String {
    let mut out = String::new();
    for stat in CprStat::ALL {
        let _ = writeln!(out, "stat.{} = {}", stat.id(), input.stats.get(stat));
    }
    out
}

/// Parses `cpr_character_input.txt`. Every STAT must appear exactly once; an unknown key, a
/// line without `=`, or a value that is not a whole number 0-255 is refused with its reason.
pub fn parse_cpr_character_input(text: &str) -> Result<CprCharacterInput, String> {
    let mut seen: [Option<u8>; 10] = [None; 10];
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(format!("{FILE} line missing '=': {raw_line}"));
        };
        let (key, value) = (key.trim(), value.trim());
        let Some(stat) = key
            .strip_prefix("stat.")
            .and_then(|id| CprStat::ALL.into_iter().find(|s| s.id() == id))
        else {
            return Err(format!("{FILE} has an unknown key '{key}'"));
        };
        let parsed = value
            .parse::<u8>()
            .map_err(|_| format!("{FILE} {key} '{value}' is not a whole number from 0 to 255"))?;
        if seen[stat.index()].replace(parsed).is_some() {
            return Err(format!("{FILE} names {key} twice"));
        }
    }
    let mut stats = CprStats::default();
    for stat in CprStat::ALL {
        let value = seen[stat.index()].ok_or_else(|| format!("{FILE} is missing stat.{}", stat.id()))?;
        stats.set(stat, value);
    }
    Ok(CprCharacterInput { stats })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> CprCharacterInput {
        let mut stats = CprStats::default();
        for (i, stat) in CprStat::ALL.into_iter().enumerate() {
            stats.set(stat, 2 + i as u8 % 7);
        }
        CprCharacterInput { stats }
    }

    #[test]
    fn the_ten_stats_are_the_books_in_its_order() {
        let labels: Vec<&str> = CprStat::ALL.iter().map(|s| s.label()).collect();
        assert_eq!(labels, ["INT", "REF", "DEX", "TECH", "COOL", "WILL", "LUCK", "MOVE", "BODY", "EMP"]);
        let ids: Vec<&str> = CprStat::ALL.iter().map(|s| s.id()).collect();
        assert_eq!(ids, ["int", "ref", "dex", "tech", "cool", "will", "luck", "move", "body", "emp"]);
    }

    #[test]
    fn render_then_parse_round_trips() {
        let input = sample();
        let text = render_cpr_character_input(&input);
        assert!(text.starts_with("stat.int = 2\nstat.ref = 3\n"), "{text}");
        assert_eq!(parse_cpr_character_input(&text), Ok(input));
    }

    #[test]
    fn parse_refuses_a_missing_stat_an_unknown_key_and_a_non_number() {
        let text = render_cpr_character_input(&sample());
        let without_emp: String = text.lines().filter(|l| !l.starts_with("stat.emp")).map(|l| format!("{l}\n")).collect();
        assert_eq!(parse_cpr_character_input(&without_emp), Err("cpr_character_input.txt is missing stat.emp".to_owned()));
        assert_eq!(
            parse_cpr_character_input(&format!("{text}stat.str = 5\n")),
            Err("cpr_character_input.txt has an unknown key 'stat.str'".to_owned())
        );
        assert_eq!(
            parse_cpr_character_input(&text.replace("stat.int = 2", "stat.int = -1")),
            Err("cpr_character_input.txt stat.int '-1' is not a whole number from 0 to 255".to_owned())
        );
        assert_eq!(
            parse_cpr_character_input(&format!("{text}stat.int = 4\n")),
            Err("cpr_character_input.txt names stat.int twice".to_owned())
        );
    }
}
