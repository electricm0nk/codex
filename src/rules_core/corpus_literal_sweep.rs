//! Corpus-literal byte-equality sweep for the shipped JSON corpus records
//! under `data/corpus/**/*.json`.
//!
//! **The bar this closes.** A record whose `wiring_class` is `static` carries
//! nothing but literal magnitudes (`wiring_class::classify`'s
//! `literal_magnitudes_only` fallback). Its bar is therefore already known and
//! needs no consumer-delta probe: the shipped record must byte-match the
//! corpus literal it was transcribed from. Until this module existed there was
//! no check that performed that comparison, so every such unit sat at `held` —
//! "as done as the current instruments can prove" — for want of the one
//! instrument that could prove it. This is that instrument. It does **not**
//! decide any unit's doneness verdict; it supplies the evidence a verdict
//! would need, and it is deliberately capable of reporting that the evidence
//! is absent.
//!
//! **What byte-equality means here, precisely.** A shipped record's
//! `data.raw_tokens` is a list of `{key, value}` pairs transcribed off a
//! PCGen `.lst` row. Reconstituted as `KEY:VALUE` each one must appear, byte
//! for byte, as a tab-separated field of that record's **token closure** —
//! its base corpus row plus every `.MOD` row in the same book targeting its
//! name or key. The closure, not the base row alone, is the correct
//! comparand: `wiring_class` itself is derived from the closure
//! (`wiring_class::token_closure_rows`), and a record whose gate token lives
//! on a `.MOD` row in a sibling file is transcription-correct, not drifted.
//! Comparing against the base row alone reports 25 such records as mismatches
//! that are in fact faithful — measured 2026-08-13, and the reason this
//! module reuses `wiring_class`'s own `build_mod_index` rather than forking a
//! narrower rule.
//!
//! **Synthesized tokens.** `ingest_races::parse_trait` appends one token the
//! trait row did not state, under the key `GLOBALVAR:ABILITY`, when a race's
//! suppression gate is declared in the book's `*globalvar*` file instead of
//! on the trait row. That key deliberately contains a `:` so it cannot be
//! mistaken for an LST token name. It is still held to a corpus literal, just
//! a different one: its value must appear verbatim as an `ABILITY:` field
//! somewhere in the same book's corpus. Every key carrying a `:` that is not
//! on [`SYNTHESIZED_TOKEN_KEYS`] is reported as
//! [`Finding::UnknownSynthesizedKey`] rather than waved through — a new
//! synthesized key must be added here with its own corpus rule, not inherit
//! an exemption.
//!
//! **Provenance.** Independently of the tokens, every record claiming a
//! `source.sha256` is checked against the real digest of the corpus file it
//! names. That is what catches the corpus moving underneath a shipped record
//! whose tokens happen to still be found on some line.
//!
//! Pure comparison only — every function here takes the corpus material it
//! needs as an argument. The `corpus_literal_sweep` binary does the walking
//! and the I/O.

use std::collections::{BTreeMap, BTreeSet};

/// Token keys this repo synthesizes rather than transcribes, each with the
/// LST token name its value must be found under in the book's corpus.
///
/// Kept as an explicit two-column table, never as "any key containing a
/// colon": a blanket rule would let a future synthesized key inherit an
/// exemption nobody reviewed, which is the shape of gate defect this repo has
/// already shipped twice (an audit implementing 3 of its 4 patterns, and an
/// open-handle check dead behind a SIGPIPE).
pub const SYNTHESIZED_TOKEN_KEYS: &[(&str, &str)] = &[("GLOBALVAR:ABILITY", "ABILITY")];

/// One `{key, value}` pair as the shipped record carries it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShippedToken {
    pub key: String,
    pub value: String,
}

impl ShippedToken {
    /// The token as it would appear on a `.lst` row: `KEY:VALUE`.
    pub fn joined(&self) -> String {
        format!("{}:{}", self.key, self.value)
    }
}

/// One shipped JSON corpus record, reduced to what the sweep compares.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShippedRecord {
    /// Repo-relative path of the JSON file, for the report.
    pub record_path: String,
    /// `source.path`: the corpus-relative `.lst` file this was read from.
    pub source_path: String,
    /// `source.line`, 1-indexed.
    pub source_line: usize,
    /// `source.sha256`, when the record claims one.
    pub source_sha256: Option<String>,
    /// Every name this record answers to (`data.key`, `data.name`,
    /// `source.record_key`) — the identities a `.MOD` row can target.
    pub identities: BTreeSet<String>,
    /// `data.raw_tokens`, in order.
    pub tokens: Vec<ShippedToken>,
}

/// One way a shipped record failed to byte-match its corpus literal.
///
/// Every variant carries the record path, because a finding a reader cannot
/// locate is a finding nobody acts on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Finding {
    /// The corpus file named by `source.path` does not exist under the
    /// corpus root.
    CorpusFileMissing { record: String, source_path: String },
    /// `source.line` is past the end of the corpus file.
    CorpusLineOutOfRange { record: String, source_path: String, line: usize, file_lines: usize },
    /// `source.sha256` does not match the corpus file's real digest.
    DigestDrift { record: String, source_path: String, claimed: String, actual: String },
    /// A transcribed token is not a byte-identical field of the record's
    /// token closure.
    TokenNotInClosure { record: String, token: String },
    /// A synthesized token's value is not a byte-identical field of the
    /// book's corpus under its declared LST token name.
    SynthesizedTokenNotInCorpus { record: String, token: String },
    /// A token key carries a `:` but is not on [`SYNTHESIZED_TOKEN_KEYS`].
    UnknownSynthesizedKey { record: String, key: String },
}

impl Finding {
    /// The record whose comparison produced this finding.
    pub fn record(&self) -> &str {
        match self {
            Finding::CorpusFileMissing { record, .. }
            | Finding::CorpusLineOutOfRange { record, .. }
            | Finding::DigestDrift { record, .. }
            | Finding::TokenNotInClosure { record, .. }
            | Finding::SynthesizedTokenNotInCorpus { record, .. }
            | Finding::UnknownSynthesizedKey { record, .. } => record,
        }
    }

    /// A single-line report form, stable enough to grep for in a gate log.
    pub fn describe(&self) -> String {
        match self {
            Finding::CorpusFileMissing { record, source_path } => {
                format!("{record}: corpus file missing: {source_path}")
            }
            Finding::CorpusLineOutOfRange { record, source_path, line, file_lines } => {
                format!("{record}: {source_path} has {file_lines} lines, record claims line {line}")
            }
            Finding::DigestDrift { record, source_path, claimed, actual } => {
                format!("{record}: {source_path} digest drift: record claims {claimed}, file is {actual}")
            }
            Finding::TokenNotInClosure { record, token } => {
                format!("{record}: token not byte-present in corpus token closure: {token}")
            }
            Finding::SynthesizedTokenNotInCorpus { record, token } => {
                format!("{record}: synthesized token not byte-present in book corpus: {token}")
            }
            Finding::UnknownSynthesizedKey { record, key } => {
                format!("{record}: token key is not an LST token name and is not a declared synthesized key: {key}")
            }
        }
    }
}

/// How much a sweep actually examined. Reported alongside the findings so a
/// run that matched nothing cannot be read as a run that found nothing —
/// the zero-cases-ran failure mode `verify.sh`'s `reach` and `audit-selftest`
/// stages each guard against.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SweepTally {
    /// JSON files read under `data/corpus`.
    pub records_seen: usize,
    /// Records in the sweep's population: `source.kind == "lst_token"` with a
    /// `data.raw_tokens` array.
    pub records_examined: usize,
    /// Individual `raw_tokens` entries compared byte-for-byte.
    pub tokens_compared: usize,
    /// Of those, the ones held to the synthesized-token rule.
    pub synthesized_tokens_compared: usize,
    /// Records whose `source.sha256` claim was checked against the real file.
    pub digests_checked: usize,
}

/// The tab-separated fields of a `.lst` row that can carry tokens: field 0 is
/// the record's own name/`.MOD` head, never a `KEY:VALUE` token, and PCGen
/// rows are padded with long runs of empty fields.
pub fn tab_tokens(line: &str) -> Vec<&str> {
    line.trim_end_matches(['\r']).split('\t').skip(1).filter(|f| !f.is_empty()).collect()
}

/// Every token in one record's closure: its base row plus each `.MOD` row
/// targeting any of its identities.
///
/// `mod_index` is `wiring_class::build_mod_index`'s output narrowed to one
/// book — this module does not fork the `.MOD` discovery rule.
pub fn token_closure(
    base_row: &str,
    identities: &BTreeSet<String>,
    mod_index: &BTreeMap<String, Vec<String>>,
) -> BTreeSet<String> {
    let mut closure: BTreeSet<String> =
        tab_tokens(base_row).into_iter().map(str::to_string).collect();
    for identity in identities {
        for row in mod_index.get(identity).into_iter().flatten() {
            closure.extend(tab_tokens(row).into_iter().map(str::to_string));
        }
    }
    closure
}

/// Compare one record's transcribed tokens against its corpus closure.
///
/// `book_corpus_tokens` is every tab field of every `.lst` row in the record's
/// book — the wider surface a synthesized token is checked against, since by
/// construction it was read from a file other than the record's own row.
pub fn compare_tokens(
    record: &ShippedRecord,
    closure: &BTreeSet<String>,
    book_corpus_tokens: &BTreeSet<String>,
    tally: &mut SweepTally,
) -> Vec<Finding> {
    let mut findings = Vec::new();
    for token in &record.tokens {
        tally.tokens_compared += 1;
        let joined = token.joined();
        if let Some((_, lst_name)) =
            SYNTHESIZED_TOKEN_KEYS.iter().find(|(key, _)| *key == token.key)
        {
            tally.synthesized_tokens_compared += 1;
            let as_corpus_token = format!("{lst_name}:{}", token.value);
            if !book_corpus_tokens.contains(&as_corpus_token) {
                findings.push(Finding::SynthesizedTokenNotInCorpus {
                    record: record.record_path.clone(),
                    token: joined,
                });
            }
            continue;
        }
        if token.key.contains(':') {
            findings.push(Finding::UnknownSynthesizedKey {
                record: record.record_path.clone(),
                key: token.key.clone(),
            });
            continue;
        }
        if !closure.contains(&joined) {
            findings.push(Finding::TokenNotInClosure {
                record: record.record_path.clone(),
                token: joined,
            });
        }
    }
    findings
}

/// One record's claim that a named corpus file had a named digest when the
/// record was written.
///
/// Deliberately a SEPARATE population from [`ShippedRecord`]. Only records
/// carrying `raw_tokens` have transcribed tokens to compare, but every record
/// citing a `source.sha256` — spells, monsters, companions and feats included
/// — is making a falsifiable claim about the corpus, and there is no reason a
/// record without `raw_tokens` should get to make it unchecked. Scoping the
/// digest check to the token population would have verified 3,516 of the
/// 8,903 claims on the tree and reported that as full coverage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceClaim {
    pub record_path: String,
    pub source_path: String,
    pub sha256: String,
}

/// Compare one provenance claim against the corpus file's real digest.
pub fn compare_digest(claim: &ProvenanceClaim, actual: &str) -> Option<Finding> {
    if claim.sha256 == actual {
        return None;
    }
    Some(Finding::DigestDrift {
        record: claim.record_path.clone(),
        source_path: claim.source_path.clone(),
        claimed: claim.sha256.clone(),
        actual: actual.to_string(),
    })
}

/// Read one shipped JSON record into the sweep's shape.
///
/// `Ok(None)` means the record is outside the sweep's population — not an
/// `lst_token` citation, or carrying no `raw_tokens` array to compare. `Err`
/// means the file is malformed, which is a finding for the caller to report,
/// never something to skip.
pub fn parse_record(record_path: &str, text: &str) -> Result<Option<ShippedRecord>, String> {
    Ok(parse_document(record_path, text)?.record)
}

/// Both populations one shipped JSON file can belong to, from a single parse.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ParsedDocument {
    /// The provenance claim, when the record cites a file and a digest —
    /// checked for every record regardless of `source.kind`.
    pub provenance: Option<ProvenanceClaim>,
    /// The transcribed record, when there are tokens to compare.
    pub record: Option<ShippedRecord>,
}

/// Read one shipped JSON record into both of the sweep's populations.
pub fn parse_document(record_path: &str, text: &str) -> Result<ParsedDocument, String> {
    let value: serde_json::Value =
        serde_json::from_str(text).map_err(|e| format!("{record_path}: invalid JSON: {e}"))?;
    let Some(source) = value.get("source").and_then(serde_json::Value::as_object) else {
        return Ok(ParsedDocument::default());
    };
    let provenance = match (
        source.get("path").and_then(serde_json::Value::as_str),
        source.get("sha256").and_then(serde_json::Value::as_str),
    ) {
        (Some(path), Some(sha256)) => Some(ProvenanceClaim {
            record_path: record_path.to_string(),
            source_path: path.to_string(),
            sha256: sha256.to_string(),
        }),
        _ => None,
    };
    let record = parse_transcription(record_path, source, &value)?;
    Ok(ParsedDocument { provenance, record })
}

fn parse_transcription(
    record_path: &str,
    source: &serde_json::Map<String, serde_json::Value>,
    value: &serde_json::Value,
) -> Result<Option<ShippedRecord>, String> {
    if source.get("kind").and_then(serde_json::Value::as_str) != Some("lst_token") {
        return Ok(None);
    }
    let Some(data) = value.get("data").and_then(serde_json::Value::as_object) else {
        return Ok(None);
    };
    let Some(raw_tokens) = data.get("raw_tokens").and_then(serde_json::Value::as_array) else {
        return Ok(None);
    };
    let source_path = source
        .get("path")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| format!("{record_path}: source.kind is lst_token but source.path is absent"))?
        .to_string();
    let source_line = source
        .get("line")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| format!("{record_path}: source.kind is lst_token but source.line is absent"))?
        as usize;
    let mut tokens = Vec::with_capacity(raw_tokens.len());
    for entry in raw_tokens {
        let key = entry
            .get("key")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| format!("{record_path}: a raw_tokens entry has no string key"))?;
        let value = entry
            .get("value")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| format!("{record_path}: raw_tokens entry {key} has no string value"))?;
        tokens.push(ShippedToken { key: key.to_string(), value: value.to_string() });
    }
    let mut identities = BTreeSet::new();
    for candidate in [data.get("key"), data.get("name"), source.get("record_key")] {
        if let Some(name) = candidate.and_then(serde_json::Value::as_str) {
            identities.insert(name.to_string());
        }
    }
    Ok(Some(ShippedRecord {
        record_path: record_path.to_string(),
        source_path,
        source_line,
        source_sha256: source
            .get("sha256")
            .and_then(serde_json::Value::as_str)
            .map(str::to_string),
        identities,
        tokens,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(tokens: &[(&str, &str)]) -> ShippedRecord {
        ShippedRecord {
            record_path: "data/corpus/x/equipment/general/thing.json".to_string(),
            source_path: "pathfinder/paizo/roleplaying_game/x/x_equip.lst".to_string(),
            source_line: 4,
            source_sha256: Some("aa".repeat(32)),
            identities: ["Thing".to_string()].into_iter().collect(),
            tokens: tokens
                .iter()
                .map(|(k, v)| ShippedToken { key: k.to_string(), value: v.to_string() })
                .collect(),
        }
    }

    fn closure_of(rows: &[&str], identities: &BTreeSet<String>) -> BTreeSet<String> {
        let mut index: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for row in &rows[1..] {
            let head = row.split('\t').next().unwrap_or("");
            if let Some(at) = head.find(".MOD") {
                index
                    .entry(crate::rules_core::wiring_class::mod_base_name(&head[..at]))
                    .or_default()
                    .push((*row).to_string());
            }
        }
        token_closure(rows[0], identities, &index)
    }

    // ---- the detection cases: each of these MUST go red on a corrupted record

    #[test]
    fn a_value_that_drifted_by_one_byte_is_a_finding() {
        let rec = record(&[("COST", "50"), ("WT", "3")]);
        let rows = ["Thing\tCOST:5\tWT:3"];
        let mut tally = SweepTally::default();
        let findings = compare_tokens(
            &rec,
            &closure_of(&rows, &rec.identities),
            &BTreeSet::new(),
            &mut tally,
        );
        assert_eq!(
            findings,
            vec![Finding::TokenNotInClosure {
                record: rec.record_path.clone(),
                token: "COST:50".to_string(),
            }],
            "a shipped COST of 50 over a corpus COST of 5 must be reported"
        );
        assert_eq!(tally.tokens_compared, 2);
    }

    #[test]
    fn a_token_the_corpus_row_never_carried_is_a_finding() {
        let rec = record(&[("COST", "50"), ("SPELLFAILURE", "35")]);
        let rows = ["Thing\tCOST:50"];
        let mut tally = SweepTally::default();
        let findings = compare_tokens(
            &rec,
            &closure_of(&rows, &rec.identities),
            &BTreeSet::new(),
            &mut tally,
        );
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert!(matches!(&findings[0], Finding::TokenNotInClosure { token, .. } if token == "SPELLFAILURE:35"));
    }

    #[test]
    fn whitespace_is_not_normalised_away() {
        // Byte-equality means byte-equality: a trailing space the corpus does
        // not have is a mismatch, not a formatting difference to forgive.
        let rec = record(&[("DAMAGE", "1d8 ")]);
        let rows = ["Thing\tDAMAGE:1d8"];
        let mut tally = SweepTally::default();
        let findings = compare_tokens(
            &rec,
            &closure_of(&rows, &rec.identities),
            &BTreeSet::new(),
            &mut tally,
        );
        assert_eq!(findings.len(), 1, "{findings:?}");
    }

    fn claim() -> ProvenanceClaim {
        ProvenanceClaim {
            record_path: "data/corpus/x/spell/thing.json".to_string(),
            source_path: "pathfinder/paizo/roleplaying_game/x/x.lst".to_string(),
            sha256: "aa".repeat(32),
        }
    }

    #[test]
    fn a_digest_that_no_longer_matches_the_corpus_file_is_a_finding() {
        let finding = compare_digest(&claim(), &"bb".repeat(32));
        assert!(
            matches!(finding, Some(Finding::DigestDrift { .. })),
            "a corpus file that changed under a shipped record must be reported, got {finding:?}"
        );
    }

    #[test]
    fn an_undeclared_namespaced_key_is_a_finding_not_an_exemption() {
        let rec = record(&[("INVENTED:ABILITY", "whatever")]);
        let rows = ["Thing"];
        let mut tally = SweepTally::default();
        let findings = compare_tokens(
            &rec,
            &closure_of(&rows, &rec.identities),
            &BTreeSet::new(),
            &mut tally,
        );
        assert_eq!(
            findings,
            vec![Finding::UnknownSynthesizedKey {
                record: rec.record_path.clone(),
                key: "INVENTED:ABILITY".to_string(),
            }],
            "a new synthesized key must not inherit GLOBALVAR:ABILITY's exemption"
        );
    }

    #[test]
    fn a_synthesized_token_absent_from_the_book_corpus_is_a_finding() {
        let rec = record(&[("GLOBALVAR:ABILITY", "Aasimar Racial Trait|AUTOMATIC|Aasimar ~ Size|PREVAREQ:Aasimar_ReplaceSize,0")]);
        let rows = ["Thing"];
        let mut tally = SweepTally::default();
        let findings = compare_tokens(
            &rec,
            &closure_of(&rows, &rec.identities),
            &BTreeSet::new(),
            &mut tally,
        );
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert!(matches!(&findings[0], Finding::SynthesizedTokenNotInCorpus { .. }));
        assert_eq!(tally.synthesized_tokens_compared, 1);
    }

    // ---- the acceptance cases: what a faithful record looks like

    #[test]
    fn a_faithful_record_produces_no_findings() {
        let rec = record(&[("COST", "50"), ("WT", "3"), ("DAMAGE", "1d8")]);
        let rows = ["Thing\t\t\tCOST:50\t\t\tWT:3\t\t\t\t\tDAMAGE:1d8"];
        let mut tally = SweepTally::default();
        let findings = compare_tokens(
            &rec,
            &closure_of(&rows, &rec.identities),
            &BTreeSet::new(),
            &mut tally,
        );
        assert_eq!(findings, vec![], "padded empty tab fields are PCGen's own formatting");
        assert_eq!(tally.tokens_compared, 3);
    }

    #[test]
    fn a_token_carried_by_a_mod_row_in_the_same_book_is_faithful() {
        // The 25-record correction: comparing against the base row alone
        // reports a `.MOD`-carried token as drift when it is transcribed
        // exactly right.
        let rec = record(&[("COST", "50"), ("SR", "13")]);
        let rows = ["Thing\tCOST:50", "CATEGORY=Special Ability|Thing.MOD\tSR:13"];
        let mut tally = SweepTally::default();
        let findings = compare_tokens(
            &rec,
            &closure_of(&rows, &rec.identities),
            &BTreeSet::new(),
            &mut tally,
        );
        assert_eq!(findings, vec![]);
    }

    #[test]
    fn a_synthesized_token_present_in_the_book_corpus_is_faithful() {
        let value = "Aasimar Racial Trait|AUTOMATIC|Aasimar ~ Size|PREVAREQ:Aasimar_ReplaceSize,0";
        let rec = record(&[("GLOBALVAR:ABILITY", value)]);
        let rows = ["Thing"];
        let book: BTreeSet<String> = [format!("ABILITY:{value}")].into_iter().collect();
        let mut tally = SweepTally::default();
        let findings =
            compare_tokens(&rec, &closure_of(&rows, &rec.identities), &book, &mut tally);
        assert_eq!(findings, vec![]);
    }

    #[test]
    fn a_matching_digest_produces_no_finding() {
        assert_eq!(compare_digest(&claim(), &"aa".repeat(32)), None);
    }

    #[test]
    fn a_record_outside_the_token_population_still_makes_a_checkable_digest_claim() {
        // The coverage gap this population split closes: a spell record cites
        // a corpus file and a digest but carries no `raw_tokens`. Scoping the
        // digest check to the token population would leave 5,387 of the
        // tree's 8,903 claims unverified while reporting full coverage.
        let text = r#"{"source":{"kind":"web_second_source","path":"a.lst","sha256":"ff"},
                      "data":{"key":"Fireball"}}"#;
        let parsed = parse_document("spell.json", text).unwrap();
        assert_eq!(parsed.record, None, "no tokens to compare");
        assert_eq!(
            parsed.provenance,
            Some(ProvenanceClaim {
                record_path: "spell.json".to_string(),
                source_path: "a.lst".to_string(),
                sha256: "ff".to_string(),
            }),
            "but its provenance claim is still falsifiable and must be checked"
        );
    }

    #[test]
    fn a_record_claiming_no_digest_makes_no_claim_to_check() {
        let text = r#"{"source":{"kind":"web_second_source","path":"a.lst"},"data":{"key":"K"}}"#;
        assert_eq!(parse_document("r.json", text).unwrap().provenance, None);
    }

    // ---- parsing

    #[test]
    fn a_record_with_no_raw_tokens_is_out_of_population_not_a_pass() {
        let text = r#"{"source":{"kind":"lst_token","path":"a.lst","line":1},"data":{"key":"K"}}"#;
        assert_eq!(parse_record("r.json", text).unwrap(), None);
    }

    #[test]
    fn a_corrected_ingest_record_is_out_of_population() {
        let text = r#"{"source":{"kind":"lst_corrected_ingest","path":"a.lst","line":1},
                      "data":{"key":"K","raw_tokens":[{"key":"COST","value":"1"}]}}"#;
        assert_eq!(parse_record("r.json", text).unwrap(), None);
    }

    #[test]
    fn parsing_collects_every_identity_a_mod_row_could_target() {
        let text = r#"{"source":{"kind":"lst_token","path":"a.lst","line":7,"sha256":"ff","record_key":"RK"},
                      "data":{"key":"K","name":"N","raw_tokens":[{"key":"COST","value":"1"}]}}"#;
        let rec = parse_record("r.json", text).unwrap().expect("in population");
        assert_eq!(rec.source_line, 7);
        assert_eq!(rec.source_sha256.as_deref(), Some("ff"));
        assert_eq!(
            rec.identities,
            ["K".to_string(), "N".to_string(), "RK".to_string()].into_iter().collect()
        );
    }

    #[test]
    fn malformed_json_is_an_error_never_a_silent_skip() {
        assert!(parse_record("r.json", "{not json").is_err());
    }

    #[test]
    fn an_lst_token_record_missing_its_line_is_an_error_never_a_silent_skip() {
        let text = r#"{"source":{"kind":"lst_token","path":"a.lst"},
                      "data":{"key":"K","raw_tokens":[{"key":"COST","value":"1"}]}}"#;
        assert!(parse_record("r.json", text).is_err());
    }

    #[test]
    fn field_zero_is_never_read_as_a_token() {
        // `Heavy Pick (Base).COPY=Pick, Heavy` is a record head, not a
        // `COPY=` token, and must not enter the closure.
        let tokens = tab_tokens("Heavy Pick (Base).COPY=Pick, Heavy\tKEY:Pick (Heavy)");
        assert_eq!(tokens, vec!["KEY:Pick (Heavy)"]);
    }
}
