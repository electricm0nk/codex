//! The live side's own converted spell record.
//!
//! SD-35 `AT-35-E6-003-RULED` cycle 8, `decisions.md` §11 ("not one line of
//! PCGen in our live code") and §19 (operator ruling B16: naming
//! `pcgen_import` in shipping code under a live root is a hit).
//!
//! ## What this closes
//!
//! Every Epic 6 census from cycle 1 on named the same blocker for the
//! `lst_parser_types`, `ir_converter` and `source_content_payload` groups:
//! *the live side has no converted record shape of its own, so it uses the
//! ingest-format parser struct as its data type and runs the converter at
//! run time to produce it.* This module is that shape for the spell kind --
//! the first one -- and it is deliberately the whole field set the parser
//! row carries, not a two-field narrowing, so nothing downstream of the
//! canonical envelope loses data.
//!
//! ## Why a spell record is a *converted* record, not an ingest one
//!
//! Nothing on this struct is PCGen grammar. Every field is a settled value:
//! `school` is `"Transmutation"`, `casting_time` is `"1 standard action"`,
//! `duration` is `"Instantaneous"`. The PCGen row's `SCHOOL:`/`CASTTIME:`
//! column tags -- the grammar -- are stripped by the parser and never
//! appear here. That is exactly the sheet rule: the sheet prints the value
//! or the rule's words, and the token spelling stays on the converter side.
//!
//! ## Who builds one
//!
//! Two producers, and only two:
//!
//! - **the converter**, [`crate::pcgen_import::ir_converter::spell_record_to_corpus`],
//!   for a record that came from a raw `.lst` row. It is the only code that
//!   reads [`crate::pcgen_import::lst_parser::spell::LstSpellRecord`].
//! - **the live corpus loader**,
//!   [`crate::rules_core::corpus_loader::load_spell_corpus`], for a record
//!   read from `data/corpus/<book>/spell/*.json`, which is already-converted
//!   corpus data and needs no converter at all.
//!
//! Consumers ([`crate::rules_core::spell_resolver::spell_id_resolve`] and
//! everything downstream of it) see only this type.

/// One spell as the live side holds it: the converted, settled field set of
/// a corpus spell record.
///
/// Field-for-field the content of a parsed spell row, minus the ingest
/// format's own vocabulary. `line_number`/`source_path` are provenance, the
/// same provenance the canonical
/// [`crate::rules_core::source_content::SourceRef`] is built from.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CorpusSpellRecord {
    /// One-based source line number this record came from. `1` for a
    /// record read from an already-converted corpus JSON file, which is
    /// one record per file.
    pub line_number: usize,
    /// Where the record came from: the `.lst` path for a converted row,
    /// the corpus `.json` path for a corpus-loaded record.
    pub source_path: String,
    /// The spell's identity. PF1 spell rows carry no separate key token,
    /// so the name *is* the identity (see
    /// [`crate::rules_core::spell_resolver`]).
    pub name: String,
    /// Display-name override when the source declared one.
    pub output_name: Option<String>,
    /// `Arcane`, `Divine`, `Arcane.Divine`, ...
    pub spell_type: Option<String>,
    /// Class/level availability as the source states it, e.g.
    /// `Wizard=0|Sorcerer=0`.
    pub classes: Option<String>,
    /// `Conjuration`, `Transmutation`, ...
    pub school: Option<String>,
    /// `Acid`, `Fire|Mind-Affecting`, ...
    pub descriptor: Option<String>,
    /// `Teleportation`, ...
    pub sub_school: Option<String>,
    /// `V, S, M`, ...
    pub components: Option<String>,
    /// `1 standard action`, ...
    pub casting_time: Option<String>,
    /// `Personal`, `Close`, ...
    pub range: Option<String>,
    /// `Potion`, `Scroll`, ...
    pub item: Option<String>,
    /// `One creature`, ...
    pub target_area: Option<String>,
    /// `Instantaneous`, ...
    pub duration: Option<String>,
    /// `Fortitude negates`, ...
    pub save_info: Option<String>,
    /// `Yes (harmless)`, ...
    pub spell_resistance: Option<String>,
    /// `p.241`, ...
    pub source_page: Option<String>,
    /// Source link when the source declared one.
    pub source_link: Option<String>,
    /// Visible description text.
    pub description: Option<String>,
    /// Description text as the source carried it, before the visible-text
    /// trim.
    pub description_raw: Option<String>,
}

impl CorpusSpellRecord {
    /// The minimal record an already-converted corpus JSON file yields:
    /// identity, school and provenance. Every other field stays `None` --
    /// this constructor never fabricates mechanical data the JSON does not
    /// carry (the same honest degradation the equipment loader documents).
    pub fn from_corpus_json_fields(
        source_path: String,
        name: String,
        school: Option<String>,
    ) -> Self {
        Self {
            line_number: 1,
            source_path,
            name,
            school,
            ..Self::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_corpus_json_record_carries_identity_school_and_provenance_and_nothing_invented() {
        let record = CorpusSpellRecord::from_corpus_json_fields(
            "data/corpus/core_rulebook/spell/animate_plants.json".to_string(),
            "Animate Plants".to_string(),
            Some("Transmutation".to_string()),
        );
        assert_eq!(record.name, "Animate Plants");
        assert_eq!(record.school.as_deref(), Some("Transmutation"));
        assert_eq!(record.line_number, 1);
        assert!(record.source_path.ends_with("animate_plants.json"));
        // Nothing invented: every mechanical field the JSON does not carry
        // stays absent rather than acquiring a plausible-looking default.
        assert!(record.casting_time.is_none());
        assert!(record.duration.is_none());
        assert!(record.description.is_none());
        assert!(record.descriptor.is_none());
        assert!(record.classes.is_none());
    }
}
