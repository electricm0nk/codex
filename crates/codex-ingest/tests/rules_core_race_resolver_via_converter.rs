// -- split from `tests` in src/rules_core/race_resolver.rs (pcgen-touching items only) --
mod tests {
    use codex::rules_core::race_resolver::*;
    use std::fs;
    use std::path::PathBuf;
    use codex::rules_core::corpus_loader::BookCorpusRoot;

    /// The runtime machinery that reports an unmatched swap is still under
    /// test even though the real corpus no longer contains one.
    ///
    /// Driven against a synthetic two-record corpus written to a temp dir —
    /// the same technique
    /// [`a_malformed_record_produces_a_diagnostic_instead_of_taking_down_the_load`]
    /// uses — because the alternative is deleting the test along with the
    /// defect, and then nothing proves the resolver still *says so* the next
    /// time a book arrives with a gate nobody ingested.
    #[test]
    fn a_swap_with_no_counterpart_is_reported_as_an_inert_flag_not_silently_dropped() {
        let dir = std::env::temp_dir().join(format!("codex_inert_flag_{}", std::process::id()));
        fs::remove_dir_all(&dir).ok();
        fs::create_dir_all(dir.join("race")).expect("temp dir");
        fs::create_dir_all(dir.join("race_trait")).expect("temp dir");

        let source = r#""source":{"kind":"lst_token","path":"synthetic.lst","sha256":"0","line":1,"record_key":"x"}"#;
        fs::write(
            dir.join("race/testrace.json"),
            format!(
                r#"{{"population":"in_scope","completeness":"chassis_only","ingested_at":"t","data":{{"key":"Testrace","name":"Testrace","base_size":null,"base_move_walk":30,"race_type":null,"type_tokens":[],"legs":2,"hands":2}},{source},"license":"OGL"}}"#
            ),
        )
        .expect("write");
        // A standard trait with NO gate, and an alternate that fires a flag
        // naming it. This is precisely the Aasimar shape as it was on disk
        // before the globalvar file was ingested.
        for (slug, body) in [
            (
                "standard",
                r#""key":"Testrace ~ Vision","name":"Vision","race_key":"Testrace","type_tokens":["Testrace Racial Default"],"is_racial_default":true,"suppressed_by_flag":null,"sets_replace_flags":[]"#,
            ),
            (
                "alternate",
                r#""key":"Testrace ~ Halo","name":"Halo","race_key":"Testrace","type_tokens":["Testrace Racial Trait"],"is_racial_default":false,"suppressed_by_flag":null,"sets_replace_flags":["Testrace_ReplaceVision"]"#,
            ),
        ] {
            fs::write(
                dir.join(format!("race_trait/{slug}.json")),
                format!(
                    r#"{{"population":"in_scope","completeness":"full","ingested_at":"t","data":{{{body}}},{source},"license":"OGL"}}"#
                ),
            )
            .expect("write");
        }

        // SD-35 `AT-35-E6-003-RULED` cycle 15: the live resolver reads settled
        // records as data, so a synthetic corpus needs its settled bundle
        // produced the same way a real book's is -- by the authoring-time
        // producer, in `#[cfg(test)]` code, which is where `decisions.md` §11
        // allows the converter to be named.
        codex_ingest::pcgen_import::corpus_settled_bundle::write_bundles_for_book(&dir)
            .expect("the synthetic book's settled bundles must be produced");

        let roots = [BookCorpusRoot { book_id: "synthetic", dir: &dir }];
        let corpus = load_race_corpus(&roots);
        assert!(corpus.diagnostics().is_empty(), "{:?}", corpus.diagnostics());

        let halo = corpus.resolve("Testrace", &["Testrace ~ Halo"]).expect("Testrace resolves");
        assert!(halo.traits.iter().any(|t| t.key == "Testrace ~ Halo"), "the alternate applies");
        assert_eq!(halo.fired_flags, vec!["Testrace_ReplaceVision".to_string()]);
        assert_eq!(
            halo.inert_flags,
            vec!["Testrace_ReplaceVision".to_string()],
            "the flag fired but suppressed nothing — reported, not hidden"
        );
        assert!(halo.suppressions.is_empty());
        // The un-suppressed standard trait is still there, which is exactly
        // what `inert_flags` is warning about.
        assert!(halo.traits.iter().any(|t| t.key == "Testrace ~ Vision"));
        fs::remove_dir_all(&dir).ok();

        // Contrast, against the real corpus: a swap with a real counterpart
        // reports no inert flag — including Aasimar's, which is what this
        // cycle changed.
        let corpus = all_books();
        assert!(corpus.resolve("Dwarf", &["Dwarf ~ Ancient Enmity"]).expect("resolves").inert_flags.is_empty());
        assert!(corpus.resolve("Aasimar", &["Aasimar ~ Halo"]).expect("resolves").inert_flags.is_empty());
    }

    /// A malformed corpus file becomes a diagnostic, not a panic and not a
    /// silent skip. Written to a temp dir so no real corpus file is touched.
    #[test]
    fn a_malformed_record_produces_a_diagnostic_instead_of_taking_down_the_load() {
        let dir = std::env::temp_dir().join(format!("codex_race_resolver_{}", std::process::id()));
        let race_dir = dir.join("race");
        fs::create_dir_all(&race_dir).expect("temp dir");
        fs::write(race_dir.join("broken.json"), "{ not json").expect("write");
        // ...and a well-formed-JSON-but-wrong-shape record.
        fs::write(race_dir.join("wrong_shape.json"), r#"{"population":"in_scope"}"#).expect("write");
        // The bundle is produced from the same malformed files, so it is
        // present and EMPTY -- which is what makes the two diagnostics below
        // per-record ones rather than one "no bundle" diagnostic for the book.
        codex_ingest::pcgen_import::corpus_settled_bundle::write_bundles_for_book(&dir)
            .expect("the temp book's settled bundle must be produced");
        let roots = [BookCorpusRoot { book_id: "temp", dir: &dir }];
        let corpus = load_race_corpus(&roots);
        assert_eq!(corpus.diagnostics().len(), 2, "{:?}", corpus.diagnostics());
        assert!(corpus.race_keys().is_empty());
        fs::remove_dir_all(&dir).ok();
    }

    /// [`RaceTraitRecord::skinwalker_change_shape_kin`] answers over the LIVE Skinwalker
    /// corpus, not a fixture, and answers exactly what the converter-side grammar answers.
    ///
    /// SD-35 `AT-35-E6-003-RULED` cycle 7. The oracle is the reading
    /// `codex::rules_core::skinwalker_change_shape` performed for itself before this method
    /// existed — strip the ingest pool prefix off each automatic grant — recomputed here
    /// record by record, so the accessor and the grammar it delegates to cannot drift apart
    /// silently. The population is every Skinwalker race-trait row in `bestiary_5`, and the
    /// nine real kins are pinned by name so a corpus change that empties this reading fails
    /// here rather than emptying the picker.
    #[test]
    fn skinwalker_change_shape_kin_names_the_nine_kin_master_rows() {
        let dir = codex_ingest::repo_root().join("data/corpus/bestiary_5");
        let roots = vec![BookCorpusRoot { book_id: "bestiary_5", dir: dir.as_path() }];
        let corpus = load_race_corpus(&roots);
        let rows = corpus.traits_for("Skinwalker");
        assert!(rows.len() > 50, "the live Skinwalker trait population is {} rows", rows.len());

        let mut kins: Vec<String> = Vec::new();
        for record in &rows {
            let oracle: Option<String> = record
                .automatic_trait_grants()
                .into_iter()
                .find_map(|g| codex_ingest::pcgen_import::race_trait_tokens::skinwalker_change_shape_kin(&g).map(str::to_string));
            assert_eq!(
                record.skinwalker_change_shape_kin(),
                oracle,
                "accessor and grammar disagree on {}",
                record.data.key
            );
            if let Some(kin) = oracle {
                kins.push(kin);
            }
        }
        kins.sort();
        kins.dedup();
        assert_eq!(
            kins,
            vec![
                "Default",
                "Werebat-Kin",
                "Werebear-Kin",
                "Wereboar-Kin",
                "Werecrocodile-Kin",
                "Wereraptor-Kin",
                "Wererat-Kin",
                "Wereshark-Kin",
                "Weretiger-Kin",
                "Werewolf-Kin",
            ],
            "the live corpus's kin master rows"
        );
    }

    /// The one test here that deliberately loads a SINGLE book —
    /// `a_race_resolves_from_its_own_book_alone_without_the_alternate_trait_book`
    /// — needs a hand-built root, and that is a real property rather than a
    /// stale scope. Its `arg()`/`b1()` siblings are gone: they existed only to
    /// feed the hardcoded `all_books()` list that
    /// [`app_loaded_books`] replaced.
    fn crb() -> BookCorpusRoot<'static> {
        BookCorpusRoot { book_id: "core_rulebook", dir: Box::leak(Box::new(codex_ingest::repo_root().join("data/corpus/core_rulebook"))).as_path() }
    }

    /// The books the shipped app really loads, read out of its own
    /// `RACE_CORPUS_BOOKS` declaration.
    ///
    /// **This used to be the hardcoded list `[crb(), b1(), arg()]`, and that
    /// is why the defect below shipped.** Every assertion in this module that
    /// says "for every alternate in the corpus" was silently scoped to three
    /// books, so when SD-29's race-trait pilot ingested a fourth
    /// (`monster_codex`), the flag table beneath went on claiming complete
    /// coverage of a corpus it no longer covered — and four alternates
    /// reached the player's picker that `pilot_compute` then refused with a
    /// claim-blocking `race.alternate_trait.unknown`. The pilot had already
    /// found and fixed the identical stale-root bug one file over
    /// (`tests/sd27_duergar_invisibility_sla_is_upstream_blocked.rs`, SD-29
    /// `progress.md`); this instance survived because nothing pointed the
    /// same question at this module.
    fn app_loaded_books() -> Vec<String> {
        let src = std::fs::read_to_string(codex_ingest::repo_root().join("apps/desktop/src-tauri/src/race_catalog.rs"))
            .expect("the desktop race catalog source is readable from the repo root");
        let decl = src
            .split("pub(crate) const RACE_CORPUS_BOOKS: &[&str] =")
            .nth(1)
            .expect("RACE_CORPUS_BOOKS is declared in race_catalog.rs");
        let list = decl.split(';').next().expect("the declaration terminates");
        list.split('"').skip(1).step_by(2).map(str::to_owned).collect()
    }

    fn all_books() -> RaceCorpus {
        let books = app_loaded_books();
        let dirs: Vec<(String, PathBuf)> = books
            .into_iter()
            .map(|book| {
                let dir = codex_ingest::repo_root().join("data/corpus").join(&book);
                (book, dir)
            })
            .collect();
        let roots: Vec<BookCorpusRoot<'_>> = dirs
            .iter()
            .map(|(book, dir)| BookCorpusRoot { book_id: book.as_str(), dir: dir.as_path() })
            .collect();
        let corpus = load_race_corpus(&roots);
        assert!(corpus.diagnostics().is_empty(), "clean load expected: {:?}", corpus.diagnostics());
        corpus
    }


}
