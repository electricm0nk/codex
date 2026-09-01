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
pub mod class_feature;
pub mod class_feature_grants;
pub mod enrich_raw_tokens_shared;
pub mod equipment_copy_citation_repair;
pub mod equipment_gap;
pub mod feat_gap;
pub mod hand_authored_equipment;
pub mod hand_authored_feat_dump;
pub mod lst_provenance_repair;
pub mod spell_lane_dump;
pub mod spell_mod_access;
pub mod ultimate_equipment;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Serialize;

use crate::rules_core::pi_screening;
use crate::rules_core::wiring_class::{
    ClosureIndexes, CorpusLines, build_copy_base_index, build_mod_index, determine_closure,
    token_closure_rows,
};

// ---------------------------------------------------------------------
// R14-04: helper-function family shared by the per-book generators below.
// Each was previously hand-copied, byte-identical, into up to 13 of the
// 14 non-`mod.rs` files in this directory. Hoisted here once; every
// per-book file that used a given helper now imports it under its
// original local name (`use super::x [as y];`) so every existing call
// site is untouched. A generator whose own copy diverges in real
// behavior (different signature, an extra `is_disabled_line` filter, a
// different return shape) keeps that copy local -- it is a distinct
// helper that happens to share a name, not a duplicate of this family.
// ---------------------------------------------------------------------

/// Real sha256 of `path`'s current on-disk content, via the system
/// `sha256sum` tool (no `sha2` crate dependency exists in this
/// workspace). Byte-identical across every per-book generator that had
/// its own copy.
pub fn sha256_file(path: &Path) -> std::io::Result<String> {
    let output = Command::new("sha256sum").arg(path).output()?;
    if !output.status.success() {
        return Err(std::io::Error::other(format!("sha256sum failed for {}", path.display())));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    Ok(text.split_whitespace().next().unwrap_or_default().to_string())
}

/// Finds `record_name` as an exact match on a line's first tab-delimited
/// column in `lst_path`. Real corpus lookup, not a value parse -- only
/// the line number is used. `feat_gap`/`equipment_gap` keep their own
/// divergent (disabled-line-aware, `Option`-returning) variants local,
/// since those are a genuinely different helper under the same name.
pub(crate) fn find_exact_first_column(lst_path: &Path, record_name: &str) -> std::io::Result<Option<u32>> {
    let content = std::fs::read_to_string(lst_path)?;
    for (idx, line) in content.lines().enumerate() {
        let first_col = line.split('\t').next().unwrap_or("");
        if first_col == record_name {
            return Ok(Some((idx + 1) as u32));
        }
    }
    Ok(None)
}

/// Finds a line carrying the exact tab-delimited field `KEY:<record_key>`
/// in `lst_path` -- for a book whose display name repeats across
/// distinct `KEY:`-tagged targets. `feat_gap`/`equipment_gap` keep their
/// own divergent variants local (see [`find_exact_first_column`]).
pub(crate) fn find_by_key_field(lst_path: &Path, record_key: &str) -> std::io::Result<Option<u32>> {
    let content = std::fs::read_to_string(lst_path)?;
    let needle = format!("KEY:{record_key}");
    for (idx, line) in content.lines().enumerate() {
        if line.split('\t').any(|field| field == needle) {
            return Ok(Some((idx + 1) as u32));
        }
    }
    Ok(None)
}

/// PI blacklist screen over a record's own candidate identity strings
/// (typically `[name, key]`). `ultimate_equipment` keeps its own
/// differently-shaped `(declared_name, name)` variant local -- a
/// distinct helper under the same name, not a duplicate of this one.
pub(crate) fn name_or_key_is_pi(values: &[&str]) -> bool {
    values.iter().any(|v| pi_screening::blacklist_term_hit_including_concatenated(v).is_some())
}

/// Lowercases `name`, collapses every non-alphanumeric run to a single
/// `_`, trims leading/trailing `_`, falls back to `"unnamed"` when
/// empty, then disambiguates against `used` with a `-2`, `-3`, ... suffix
/// so two records that slugify identically still get distinct file
/// names.
pub(crate) fn slugify_dedup(name: &str, used: &mut BTreeSet<String>) -> String {
    let mut slug: String =
        name.to_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
    while slug.contains("__") {
        slug = slug.replace("__", "_");
    }
    let slug = slug.trim_matches('_').to_string();
    let slug = if slug.is_empty() { "unnamed".to_string() } else { slug };

    if !used.contains(&slug) {
        used.insert(slug.clone());
        return slug;
    }
    let mut n = 2;
    loop {
        let candidate = format!("{slug}-{n}");
        if !used.contains(&candidate) {
            used.insert(candidate.clone());
            return candidate;
        }
        n += 1;
    }
}

/// Same slug formula as [`slugify_dedup`], without the collision-dedup
/// step -- for a generator whose record population is already known not
/// to collide.
pub(crate) fn slugify_or_unnamed(name: &str) -> String {
    let mut slug: String =
        name.to_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
    while slug.contains("__") {
        slug = slug.replace("__", "_");
    }
    let slug = slug.trim_matches('_').to_string();
    if slug.is_empty() { "unnamed".to_string() } else { slug }
}

/// Serializes `record` to `<out_dir>/<slug>.json`, creating `out_dir` if
/// needed, WITHOUT overwriting an existing file (SD-32 Epic 5 protective
/// sweep -- an already-shipped file's later-pass-only fields, e.g.
/// `raw_tokens`, must never be silently clobbered by a re-run). Silently
/// no-ops when the file already exists; callers that need to know
/// whether a write actually happened use [`write_json_bool`] instead.
pub(crate) fn write_json_unit<R: Serialize>(out_dir: &Path, slug: &str, record: &R) -> std::io::Result<()> {
    std::fs::create_dir_all(out_dir)?;
    let path = out_dir.join(format!("{slug}.json"));
    if path.exists() {
        return Ok(());
    }
    let json = serde_json::to_string_pretty(record)
        .expect("record is a plain-data shape; serialization cannot fail");
    std::fs::write(path, json)
}

/// Same no-clobber contract as [`write_json_unit`], but reports whether a
/// write actually happened (`Ok(true)`) or an existing file blocked it
/// (`Ok(false)`).
pub(crate) fn write_json_bool<R: Serialize>(out_dir: &Path, slug: &str, record: &R) -> std::io::Result<bool> {
    std::fs::create_dir_all(out_dir)?;
    let path = out_dir.join(format!("{slug}.json"));
    if path.exists() {
        return Ok(false);
    }
    let json = serde_json::to_string_pretty(record)
        .expect("record is a plain-data shape; serialization cannot fail");
    std::fs::write(path, json)?;
    Ok(true)
}

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
    copy_base_index: BTreeMap<(String, String), String>,
}

impl WiringClassIndex {
    /// `book_id` is the id `wiring_class`'s corpus-wide tables key on
    /// (e.g. `"advanced_class_guide"`); `book_dir` is that book's real
    /// directory (`cache_gen::acg::book_dir(corpus_root)` and siblings).
    pub fn build(book_id: &str, book_dir: &Path) -> Self {
        let mut book_paths = BTreeMap::new();
        book_paths.insert(book_id.to_string(), book_dir.to_path_buf());
        let mod_index = build_mod_index(&book_paths);
        let copy_base_index = build_copy_base_index(&book_paths);
        WiringClassIndex { book_id: book_id.to_string(), book_paths, mod_index, copy_base_index }
    }

    /// Same as [`build`], but ALSO indexes a second book-keyed directory.
    ///
    /// `decisions.md §9`: a `decisions.md §9`-re-attributed unit's physical
    /// file can live under `core_essentials`'s own directory while its
    /// REPORTING `book_id` is this index's primary one -- confirmed live
    /// (`SD31-E6-F9-005`): `beastiary`/`bestiary_2`'s 168 newly-transcribed
    /// `ce_abilities_race.lst`-origin `monster_ability` records all stamped
    /// `wiring_class: "ambiguous"` (`no_corpus_line`) under plain [`build`],
    /// because `book_paths` held only `beastiary`'s own directory and
    /// `ce_abilities_race.lst` is not under it. [`wiring_class_for_book`] is
    /// the matching lookup half: pass `extra_book_id` there (never the
    /// primary `book_id`) for a citation whose `source.path` names the extra
    /// directory.
    pub fn build_with_extra(
        book_id: &str,
        book_dir: &Path,
        extra_book_id: &str,
        extra_dir: &Path,
    ) -> Self {
        let mut book_paths = BTreeMap::new();
        book_paths.insert(book_id.to_string(), book_dir.to_path_buf());
        book_paths.insert(extra_book_id.to_string(), extra_dir.to_path_buf());
        let mod_index = build_mod_index(&book_paths);
        let copy_base_index = build_copy_base_index(&book_paths);
        WiringClassIndex { book_id: book_id.to_string(), book_paths, mod_index, copy_base_index }
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
        self.wiring_class_for_book(lines, &self.book_id.clone(), file, line, name, key)
    }

    /// Same as [`wiring_class_for`], but resolves the citation against
    /// `book` (which must be a key [`build`]/[`build_with_extra`]
    /// registered) rather than always this index's own `book_id` -- the
    /// matching lookup half of [`build_with_extra`]'s second directory.
    pub fn wiring_class_for_book(
        &self,
        lines: &mut CorpusLines,
        book: &str,
        file: &str,
        line: u32,
        name: &str,
        key: &str,
    ) -> (String, Vec<String>) {
        let rows = token_closure_rows(
            lines,
            ClosureIndexes { mod_index: &self.mod_index, copy_base_index: &self.copy_base_index },
            book,
            file,
            line as usize,
            name,
            key,
        );
        let row_refs: Vec<Option<&str>> = rows.iter().map(|r| r.as_deref()).collect();
        let (class, _reason, sigs) = determine_closure(&row_refs);
        let mut signals: Vec<String> = sigs.into_iter().collect();
        signals.sort();
        (class.id().to_string(), signals)
    }

    /// The full raw-row token closure for one record -- its own base corpus
    /// row, plus every `.MOD` row targeting its `name`/`key` within this
    /// book, plus (when applicable) the plain base row a `.COPY=` row
    /// inherits from. The SAME [`token_closure_rows`] resolution
    /// [`wiring_class_for_book`] already uses to CLASSIFY a record, exposed
    /// here so a generator's shipped `raw_tokens` field can be built from
    /// the identical closure instead of the base row alone.
    ///
    /// Row 21 (`decisions.md`): a real `.MOD`-appended `BONUS:VAR` line
    /// lives on a SEPARATE corpus row from the base record it targets --
    /// reading only the base row's own line (as every `raw_tokens` builder
    /// did before this method existed) silently drops it. PCGen legally
    /// emits SEVERAL `.MOD` lines against the same base name (three, for
    /// `core_rulebook`'s own `"Bloodline Tracker"`); this closure carries
    /// all of them, not just the first or last one found.
    pub fn closure_rows_for_book(
        &self,
        lines: &mut CorpusLines,
        book: &str,
        file: &str,
        line: u32,
        name: &str,
        key: &str,
    ) -> Vec<Option<String>> {
        token_closure_rows(
            lines,
            ClosureIndexes { mod_index: &self.mod_index, copy_base_index: &self.copy_base_index },
            book,
            file,
            line as usize,
            name,
            key,
        )
    }

    /// Same as [`closure_rows_for_book`], but against this index's own
    /// `book_id` -- the closure-rows counterpart to [`wiring_class_for`].
    pub fn closure_rows(
        &self,
        lines: &mut CorpusLines,
        file: &str,
        line: u32,
        name: &str,
        key: &str,
    ) -> Vec<Option<String>> {
        self.closure_rows_for_book(lines, &self.book_id.clone(), file, line, name, key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /// A throwaway pair of directories -- `primary` and `extra` -- cleaned up
    /// on drop, mirroring `gen_book_cache.rs`'s own scratch-fixture pattern.
    struct Scratch {
        primary: PathBuf,
        extra: PathBuf,
    }

    impl Scratch {
        fn new(name: &str) -> Self {
            let base = std::env::temp_dir()
                .join(format!("codex_wiring_class_index_{name}_{}", std::process::id()));
            let _ = fs::remove_dir_all(&base);
            let primary = base.join("primary");
            let extra = base.join("extra");
            fs::create_dir_all(&primary).unwrap();
            fs::create_dir_all(&extra).unwrap();
            Scratch { primary, extra }
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(self.primary.parent().unwrap());
        }
    }

    /// `SD31-E6-F9-005`: proves `build_with_extra`/`wiring_class_for_book`
    /// actually reaches a citation that lives ONLY under the extra directory
    /// -- confirmed live against the real pinned oracle (168
    /// `ce_abilities_race.lst`-origin `monster_ability` records all stamped
    /// `wiring_class: "ambiguous"` before this fix existed), reproduced here
    /// hermetically so a regression fails fast without the live oracle.
    #[test]
    fn wiring_class_for_book_resolves_a_citation_only_present_in_the_extra_directory() {
        let scratch = Scratch::new("extra_dir");
        fs::write(scratch.extra.join("x.lst"), "Row\tKEY:Row\tBONUS:VAR|Foo|1+HD\n").unwrap();
        let index =
            WiringClassIndex::build_with_extra("primary_book", &scratch.primary, "extra_book", &scratch.extra);
        let mut lines = index.lines();
        let (class, signals) = index.wiring_class_for_book(&mut lines, "extra_book", "x.lst", 1, "Row", "Row");
        assert_ne!(class, "ambiguous", "the extra directory's row must resolve, not fall through to no_corpus_line");
        assert!(!signals.contains(&"no_corpus_line".to_string()));
    }

    /// The mutation-proof half: plain `build` (no extra directory registered
    /// at all) canNOT see the same file, so the SAME citation correctly
    /// stamps `ambiguous`/`no_corpus_line` -- proving the fix in the test
    /// above is really exercising the fallback path, not something that
    /// would have passed anyway.
    #[test]
    fn build_alone_cannot_see_a_file_outside_its_own_directory() {
        let scratch = Scratch::new("no_extra");
        fs::write(scratch.extra.join("x.lst"), "Row\tKEY:Row\tBONUS:VAR|Foo|1+HD\n").unwrap();
        let index = WiringClassIndex::build("primary_book", &scratch.primary);
        let mut lines = index.lines();
        // `line: 2`, not `1` -- `OPEN-ISSUES.md` row 10's documented,
        // pre-existing, deliberately-left-unfixed `CorpusLines::line()`
        // quirk: an unresolved `(book, file)` caches as `"".split('\n')`,
        // which yields a 1-element `[""]` buffer, so `line == 1` against ANY
        // unresolved file returns `Some("")` rather than `None`; only
        // `line > 1` correctly reports "genuinely absent." Using `2` here is
        // the same workaround that row's own remedy names, not a new
        // discovery -- unrelated to this test's actual subject
        // (`build`/`build_with_extra`'s directory scoping).
        let (class, signals) = index.wiring_class_for(&mut lines, "x.lst", 2, "Row", "Row");
        assert_eq!(class, "ambiguous");
        assert_eq!(signals, vec!["no_corpus_line".to_string()]);
    }
}
