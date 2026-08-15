//! JSON cache generation (SD-26 Epic 3). Per-book generator modules live
//! here (`apg`, and siblings added by sibling per-book cycles). Each
//! generator DUMPS the current runtime state of its book's already-landed
//! `rules_tables` module to `data/corpus/<book>/**/*.json` -- it never
//! re-derives field values by re-parsing raw PCGen LST (`decisions.md
//! §11.3`). The only reason any of this code touches the real LST corpus
//! at all is to recover a real, checkable line-number *citation* for a
//! value that is already known (from the compiled Rust module) to be
//! correct -- never to compute the value itself.

pub mod acg;
pub mod apg;
pub mod beastiary1;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::rules_core::wiring_class::{
    CorpusLines, build_mod_index, determine_closure, token_closure_rows,
};

/// GE-01 `wiring_class` support shared by every per-book generator: builds
/// the book's `.MOD`-row closure index once, then answers per-record
/// `wiring_class` + `wiring_class_signals` from it. A generator DUMPS
/// already-computed Rust values for every OTHER field (module doc comment
/// above); `wiring_class` is the one field this module computes directly
/// from the corpus, because it describes the corpus record's own shape,
/// not anything the compiled Rust module could carry.
pub struct WiringClassIndex {
    book_id: String,
    book_paths: BTreeMap<String, PathBuf>,
    mod_index: BTreeMap<(String, String), Vec<String>>,
}

impl WiringClassIndex {
    /// `book_id` is the id `wiring_class`'s corpus-wide tables key on
    /// (e.g. `"advanced_class_guide"`); `book_dir` is that book's real
    /// directory (`cache_gen::acg::book_dir(corpus_root)` and siblings).
    pub fn build(book_id: &str, book_dir: &Path) -> Self {
        let mut book_paths = BTreeMap::new();
        book_paths.insert(book_id.to_string(), book_dir.to_path_buf());
        let mod_index = build_mod_index(&book_paths);
        WiringClassIndex { book_id: book_id.to_string(), book_paths, mod_index }
    }

    /// A fresh raw-line reader borrowing this index's book-path table.
    /// Callers keep one alive across a `generate_*` function's whole loop
    /// so repeated citations into the same `.lst` file are cached.
    pub fn lines(&self) -> CorpusLines<'_> {
        CorpusLines::new(&self.book_paths)
    }

    /// The `book_id` this index was built with -- lets a caller holding
    /// only the index (not the original `book_id` string) derive a
    /// citation's path correctly relative to this index's own `book_dir`,
    /// e.g. `gen_book_cache.rs`'s `wiring_class_file_arg`.
    pub fn book_id(&self) -> &str {
        &self.book_id
    }

    /// `(wiring_class id, sorted wiring_class_signals)` for one record,
    /// given the SAME `file`/`line`/`name`/`key` the generator already
    /// resolved for its `source.path`/`source.line` citation. `file` is
    /// the bare `.lst` basename (e.g. `"acg_classes.lst"`), not the
    /// book-prefixed `source.path`.
    pub fn wiring_class_for(
        &self,
        lines: &mut CorpusLines,
        file: &str,
        line: u32,
        name: &str,
        key: &str,
    ) -> (String, Vec<String>) {
        let rows =
            token_closure_rows(lines, &self.mod_index, &self.book_id, file, line as usize, name, key);
        let row_refs: Vec<Option<&str>> = rows.iter().map(|r| r.as_deref()).collect();
        let (class, _reason, sigs) = determine_closure(&row_refs);
        let mut signals: Vec<String> = sigs.into_iter().collect();
        signals.sort();
        (class.id().to_string(), signals)
    }
}
