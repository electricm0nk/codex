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
    fn pairs_preserve_file_order() {
        assert_eq!(
            token_pairs(&doc()),
            vec![("DESC", "first"), ("CSKILL", "Perception|Stealth"), ("DESC", "second")]
        );
    }
}
