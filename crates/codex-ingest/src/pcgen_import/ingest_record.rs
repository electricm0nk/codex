//! Tool-side accessors for an ingest record's PCGen token array.
//!
//! SD-35 `AT-35-E6-002` cycle 2, under `decisions.md` §11 ("no PCGen in live
//! code"). A corpus record on disk carries the `.lst` row it was ingested
//! from as a `data.raw_tokens` array of `{"key": …, "value": …}` objects.
//! That array is **ingest format**: knowing its field name, its element
//! shape, and how to search it is converter business, and the ruling puts
//! converter business on the tool side.
//!
//! Before this module, every ground-truth corpus assertion under
//! `src/rules_core/` spelled the traversal out by hand -- `json["data"]
//! ["raw_tokens"].as_array().expect(…).iter().find(|t| t["key"].as_str() ==
//! Some("CSKILL"))` -- which put the ingest field name, and a private
//! re-implementation of the search, inside a live module. These functions are
//! the one place that reads it.
//!
//! **This does not make a live module's dependency on the ingest format
//! disappear**; it makes it a named cross-boundary call instead of an
//! open-coded one. A live *rules* path that still needs a token value is a
//! reader `AT-35-E6-002`'s remainder still owes; a *ground-truth test* that
//! asserts a hand-written table against the row it was transcribed from is
//! reading the oracle, which `decisions.md` §11 keeps.
//!
//! Both record shapes are accepted: the full document (`{"data": {…}}`) and
//! the `data` object on its own. Callers hold one or the other depending on
//! whether they parsed a whole file or already indexed into it.
//!
//! KEPT for Starfinder, like the rest of `src/pcgen_import/`.

use serde_json::Value;

/// The token array of `doc`, whichever of the two record shapes `doc` is.
///
/// Returns an empty slice for a record that carries no token array at all --
/// a "thin" record. Absent is not an error here: the callers that require a
/// token to exist assert on the value they were looking for, which names the
/// token in the failure message instead of naming the container.
fn tokens(doc: &Value) -> &[Value] {
    let data = if doc.get("data").is_some_and(Value::is_object) { &doc["data"] } else { doc };
    data.get("raw_tokens").and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[])
}

/// Every `(key, value)` pair on `doc`, in file order.
///
/// Entries whose `key` or `value` is not a string are skipped rather than
/// panicking -- the ingest writes both as strings, and a caller that wants to
/// prove that should assert on [`token_count`] against the length of this.
pub fn token_pairs(doc: &Value) -> Vec<(&str, &str)> {
    tokens(doc)
        .iter()
        .filter_map(|t| Some((t.get("key")?.as_str()?, t.get("value")?.as_str()?)))
        .collect()
}

/// Every `BONUS:` chain on `doc`, as its pipe-delimited qualifier list, in file
/// order.
///
/// The sibling of [`token_pairs`] for the record's other ingest array. A record
/// carrying no chains -- a "thin" record, or one whose LST row had no `BONUS:`
/// clause -- yields an empty `Vec`, and a malformed entry is skipped rather
/// than panicking, exactly as [`token_pairs`] treats a malformed token.
///
/// SD-35 `AT-35-E6-002` cycle 4: `rules_core::corpus_loader` open-coded this
/// traversal, naming the ingest field and re-implementing the search inside a
/// live module -- the same defect cycle 2 built this module to remove for the
/// token array, missed then only because the residue gate's pattern names
/// `raw_tokens` and not its sibling.
pub fn bonus_chain_qualifiers(doc: &Value) -> Vec<Vec<&str>> {
    let data = if doc.get("data").is_some_and(Value::is_object) { &doc["data"] } else { doc };
    let Some(chains) = data.get("raw_bonus_chains").and_then(Value::as_array) else {
        return Vec::new();
    };
    chains
        .iter()
        .filter_map(|entry| {
            let qualifiers = entry.get("qualifiers")?.as_array()?;
            Some(qualifiers.iter().filter_map(Value::as_str).collect())
        })
        .collect()
}

/// Rebuild the ingest-format `BONUS:` line a qualifier chain came off.
///
/// SD-35 `AT-35-E6-003-SWEEP` cycle 11. [`bonus_chain_qualifiers`] splits an
/// ingest record's bonus chain into its `|`-separated qualifiers; this puts the
/// chain back together in the format it was read from, and it belongs here —
/// beside the split — rather than on the live side. `decisions.md` §11: the
/// ingest format is the converter's vocabulary, and the token name `BONUS:` is
/// part of that format, not part of anything a character sheet computes.
///
/// The one caller is `rules_core::corpus_loader`, which fills
/// `BonusToken::raw_bonus` for the corpus-provenance readers (`wiring_class`,
/// `corpus_literal_sweep`). Same bytes as before: `"BONUS:"` + the qualifiers
/// re-joined on `|`, which is the inverse of the split that produced them.
pub fn rebuild_bonus_token<S: AsRef<str>>(qualifiers: &[S]) -> String {
    let mut out = String::from("BONUS:");
    for (i, qualifier) in qualifiers.iter().enumerate() {
        if i > 0 {
            out.push('|');
        }
        out.push_str(qualifier.as_ref());
    }
    out
}

/// Every token key on `doc`, in file order, with duplicates kept.
pub fn token_keys(doc: &Value) -> Vec<&str> {
    tokens(doc).iter().filter_map(|t| t.get("key")?.as_str()).collect()
}

/// Every `key` token's value on `doc`, in file order.
pub fn token_values<'a>(doc: &'a Value, key: &str) -> Vec<&'a str> {
    token_pairs(doc).into_iter().filter(|(k, _)| *k == key).map(|(_, v)| v).collect()
}

/// The first `key` token's value on `doc`, or `None` when the record carries
/// none.
pub fn first_token_value<'a>(doc: &'a Value, key: &str) -> Option<&'a str> {
    token_pairs(doc).into_iter().find(|(k, _)| *k == key).map(|(_, v)| v)
}

/// The tail of the first `TYPE:` token on `doc` whose value starts with
/// `prefix` — `None` when the record carries no such token.
///
/// `TYPE:Trait.RaceTrait.Oread Race Trait` with prefix `"Trait.RaceTrait."`
/// yields `"Oread Race Trait"`. A `TYPE:` token that does not carry the
/// prefix contributes nothing, which is how a non-race-scoped row honestly
/// reports that it belongs to no race pool.
pub fn type_token_suffix<'a>(doc: &'a Value, prefix: &str) -> Option<&'a str> {
    token_values(doc, "TYPE").into_iter().find_map(|v| v.strip_prefix(prefix))
}

/// How many tokens `doc` carries. `0` for a record with no token array.
pub fn token_count(doc: &Value) -> usize {
    tokens(doc).len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn doc() -> Value {
        json!({"data": {"key": "Probe", "raw_tokens": [
            {"key": "DESC", "value": "first"},
            {"key": "CSKILL", "value": "Perception|Stealth"},
            {"key": "DESC", "value": "second"}
        ]}})
    }

    #[test]
    fn reads_the_full_document_shape() {
        assert_eq!(token_values(&doc(), "DESC"), vec!["first", "second"]);
        assert_eq!(first_token_value(&doc(), "CSKILL"), Some("Perception|Stealth"));
        assert_eq!(token_count(&doc()), 3);
        assert_eq!(token_keys(&doc()), vec!["DESC", "CSKILL", "DESC"]);
    }

    #[test]
    fn reads_the_bare_data_object_shape() {
        let full = doc();
        let data = &full["data"];
        assert_eq!(token_values(data, "DESC"), vec!["first", "second"]);
        assert_eq!(token_count(data), 3);
    }

    /// SD-35 `AT-35-E6-003-SWEEP` cycle 11. The rebuild is the exact inverse
    /// of [`bonus_chain_qualifiers`]' split, checked against a real shipped
    /// chain shape rather than against a hand-typed string: split it, rebuild
    /// it, and the bytes are the ones the record states.
    #[test]
    fn rebuilding_a_bonus_chain_is_the_inverse_of_splitting_it() {
        let doc = json!({"data": {"key": "Probe", "raw_bonus_chains": [
            {"qualifiers": ["COMBAT", "AC", "1", "TYPE=Enhancement"]},
            {"qualifiers": ["WEAPONPROF=TYPE.Natural", "TOHIT,DAMAGE", "1", "TYPE=Enhancement"]},
            {"qualifiers": ["FEAT"]}
        ]}});
        let chains = bonus_chain_qualifiers(&doc);
        let rebuilt: Vec<String> = chains.iter().map(|q| rebuild_bonus_token(q)).collect();
        assert_eq!(
            rebuilt,
            vec![
                "BONUS:COMBAT|AC|1|TYPE=Enhancement".to_string(),
                "BONUS:WEAPONPROF=TYPE.Natural|TOHIT,DAMAGE|1|TYPE=Enhancement".to_string(),
                "BONUS:FEAT".to_string(),
            ]
        );
        // A chain with no qualifiers at all is the bare token name, never a
        // trailing separator.
        let empty: [&str; 0] = [];
        assert_eq!(rebuild_bonus_token(&empty), "BONUS:");
    }

    #[test]
    fn a_thin_record_with_no_token_array_is_empty_not_a_panic() {
        let thin = json!({"data": {"key": "Thin", "weight_lbs": 0.1}});
        assert_eq!(token_count(&thin), 0);
        assert!(token_values(&thin, "WT").is_empty());
        assert_eq!(first_token_value(&thin, "WT"), None);
    }

    #[test]
    fn an_absent_token_is_none_not_a_fabricated_empty_string() {
        assert_eq!(first_token_value(&doc(), "SPELLKNOWN"), None);
        assert!(token_values(&doc(), "SPELLKNOWN").is_empty());
    }

    #[test]
    fn a_type_token_yields_only_the_tail_behind_its_prefix() {
        let row = json!({"data": {"raw_tokens": [
            {"key": "TYPE", "value": "Trait.RaceTrait.Oread Race Trait"}
        ]}});
        assert_eq!(type_token_suffix(&row, "Trait.RaceTrait."), Some("Oread Race Trait"));
        let bare = json!({"data": {"raw_tokens": [{"key": "TYPE", "value": "Trait"}]}});
        assert_eq!(type_token_suffix(&bare, "Trait.RaceTrait."), None);
        assert_eq!(type_token_suffix(&doc(), "Trait.RaceTrait."), None);
    }

    #[test]
    fn pairs_preserve_file_order() {
        assert_eq!(
            token_pairs(&doc()),
            vec![("DESC", "first"), ("CSKILL", "Perception|Stealth"), ("DESC", "second")]
        );
    }
}
