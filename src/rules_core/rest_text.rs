//! Rest and recovery text (SD-43 E2.4, `decisions.md §8`): the committed table at
//! `data/rest_and_recovery.json`, one row per rule, each the book's words with its book and page,
//! and the one query both surfaces read (the print block after the Bio page and the Biography tab).
//! Nothing here computes a number: the rows are printed as text (the paper-sheet rule).

use serde::{Deserialize, Serialize};

use crate::support::paths::repo_root;

/// Path of the table under the data root (bundled as a resource in a packaged app).
pub const REST_TABLE_PATH: &str = "data/rest_and_recovery.json";

/// One rule: who it is for, its words and where the book prints them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RestRow {
    pub id: String,
    /// The saved character's `game_system` the row is for (`pf1`).
    pub system: String,
    /// `all`, `class:<slug>`, `race:<slug>` or `trait:<record key>`.
    pub applies: Vec<String>,
    pub rule: String,
    pub text: String,
    pub book: String,
    pub page: String,
    /// The corpus record the text is quoted from; `None` for the book's chapter prose.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record: Option<String>,
}

#[derive(Deserialize)]
struct RestTable {
    rows: Vec<RestRow>,
}

/// The table's rows. Malformed input, an uncited row, or an `applies` entry of no known shape is an
/// error naming the row, never a silently shorter table.
pub fn parse_rest_table(json: &str) -> Result<Vec<RestRow>, String> {
    let table: RestTable = serde_json::from_str(json).map_err(|e| format!("{REST_TABLE_PATH} is not a rest table: {e}"))?;
    for row in &table.rows {
        if [&row.id, &row.system, &row.rule, &row.text, &row.book, &row.page].iter().any(|field| field.trim().is_empty()) {
            return Err(format!("{REST_TABLE_PATH}: row {:?} lacks an id, system, rule, text, book or page", row.id));
        }
        let known = |entry: &str| entry == "all" || ["class:", "race:", "trait:"].iter().any(|prefix| entry.strip_prefix(prefix).is_some_and(|rest| !rest.is_empty()));
        if row.applies.is_empty() || !row.applies.iter().all(|entry| known(entry)) {
            return Err(format!("{REST_TABLE_PATH}: row {:?} has an `applies` entry of no known shape: {:?}", row.id, row.applies));
        }
    }
    Ok(table.rows)
}

/// The table, loaded once per process from [`repo_root`] (the packaged resource root in an
/// installed app).
pub fn rest_table() -> &'static Result<Vec<RestRow>, String> {
    static LOADED: std::sync::OnceLock<Result<Vec<RestRow>, String>> = std::sync::OnceLock::new();
    LOADED.get_or_init(|| {
        let path = repo_root().join(REST_TABLE_PATH);
        let text = std::fs::read_to_string(&path).map_err(|e| format!("could not read {}: {e}", path.display()))?;
        parse_rest_table(&text)
    })
}

/// The rows for one character, in table order: its system's `all` rows, then the rows for a held
/// class (`classes`: slugs, `wizard`), its race (`race`: slug, `elf`) or a held alternate racial
/// trait (`traits`: record keys, `Gnome ~ Vivacious`).
pub fn rest_rows_for<'a>(rows: &'a [RestRow], system: &str, classes: &[&str], race: &str, traits: &[String]) -> Vec<&'a RestRow> {
    let applies = |entry: &str| match entry.split_once(':') {
        None => entry == "all",
        Some(("class", slug)) => classes.contains(&slug),
        Some(("race", slug)) => slug == race,
        Some(("trait", key)) => traits.iter().any(|held| held == key),
        Some(_) => false,
    };
    rows.iter().filter(|row| row.system == system && row.applies.iter().any(|entry| applies(entry))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table() -> &'static [RestRow] {
        rest_table().as_ref().expect("data/rest_and_recovery.json loads")
    }

    #[test]
    fn rest_text_every_row_is_cited_and_a_quoted_row_is_its_records_words_on_its_page() {
        let mut quoted = 0;
        for row in table() {
            assert!(row.page.starts_with("p."), "{}: page {:?}", row.id, row.page);
            let Some(record) = &row.record else { continue };
            let path = repo_root().join(record);
            let json: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {record}: {e}", row.id))).unwrap();
            let data = &json["data"];
            let description = data["description"].as_str().unwrap_or_default().replace("&nl;", " ");
            assert!(description.contains(&row.text), "{}: the text is not {record}'s words", row.id);
            let page = data["raw_tokens"].as_array().into_iter().flatten().find(|token| token["key"] == "SOURCEPAGE").and_then(|token| token["value"].as_str());
            assert_eq!(page, Some(row.page.as_str()), "{}: {record}'s page", row.id);
            quoted += 1;
        }
        assert!(quoted >= 1, "at least one row is checked against its record");
        let mut ids: Vec<&str> = table().iter().map(|row| row.id.as_str()).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), table().len(), "row ids are unique");
    }

    #[test]
    fn rest_text_an_uncited_row_or_an_unknown_applies_is_an_error() {
        let row = |book: &str, applies: &str| format!(r#"{{"rows":[{{"id":"x","system":"pf1","applies":["{applies}"],"rule":"r","text":"t","book":"{book}","page":"p.1"}}]}}"#);
        assert!(parse_rest_table(&row("Core Rulebook", "all")).is_ok());
        assert!(parse_rest_table(&row("", "all")).is_err(), "no book");
        assert!(parse_rest_table(&row("Core Rulebook", "deity:iomedae")).is_err(), "unknown shape");
        assert!(parse_rest_table(&row("Core Rulebook", "class:")).is_err(), "empty slug");
        assert!(parse_rest_table("not json").is_err());
    }

    #[test]
    fn rest_text_a_wizard_elf_gets_the_all_rows_and_the_wizard_rows_only() {
        let ids = |classes: &[&str], race: &str, traits: &[String]| -> Vec<String> {
            rest_rows_for(table(), "pf1", classes, race, traits).into_iter().map(|row| row.id.clone()).collect()
        };
        assert_eq!(
            ids(&["wizard"], "elf", &[]),
            ["pf1.all.natural_healing", "pf1.all.ability_damage", "pf1.class.wizard.rest", "pf1.class.wizard.preparation_time"]
        );
        assert!(ids(&["paladin"], "human", &[]).contains(&"pf1.class.divine.time_of_day".to_owned()));
        assert!(ids(&["fighter"], "gnome", &["Gnome ~ Vivacious".to_owned()]).contains(&"pf1.trait.gnome_vivacious".to_owned()));
        assert!(!ids(&["fighter"], "gnome", &[]).contains(&"pf1.trait.gnome_vivacious".to_owned()), "only a held trait's row");
        assert!(rest_rows_for(table(), "starfinder-1e", &["mystic"], "human", &[]).is_empty(), "no Starfinder rows");
    }
}
