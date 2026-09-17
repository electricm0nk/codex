// -- split from `tests` in src/rules_core/rules_tables/feats_all.rs (pcgen-touching items only) --
mod tests {
    use codex::rules_core::rules_tables::feats_all::*;
    use codex::rules_core::rules_tables::RuleSetId;
    use codex::rules_core::rules_tables::ultimate_campaign::feat_tables as uca_feats;
    use std::collections::BTreeMap;

    /// UCA's 21 text-complete records surface both the corpus `DESC:`
    /// flavor text and the `.MOD BENEFIT:` mechanical text, joined --
    /// showing only `DESC:` (`"[Not Implemented] ..."`) would be a stub
    /// by `docs/governance/no-stub-mvp-doctrine.md`. Its 2
    /// `deferred-with-reason` records surface the flavor text plus the
    /// engine's own verbatim diagnostic instead of the corrupted upstream
    /// benefit text. (`Stronghold` was deferred in this module's first
    /// pass and is now text-complete -- its own text is genuinely
    /// complete; see `ultimate_campaign::feat_tables`'s own doc comment
    /// for the correction.)
    #[test]
    fn uca_records_join_desc_and_benefit_and_defer_the_two_corrupted_rows() {
        let find = |key: &str| {
            all_feat_tables()
                .iter()
                .filter(|book| book.rule_set == RuleSetId::Uca)
                .flat_map(|book| book.entries.iter())
                .find(|entry| entry.key == key)
                .unwrap_or_else(|| panic!("'{key}' must be in the UCA aggregate"))
        };

        let accursed = find("Accursed");
        assert_eq!(accursed.category, "Story");
        let desc = accursed.description.expect("Accursed must have a joined description");
        assert!(desc.starts_with("[Not Implemented] Your curse weighs down your soul"));
        assert!(
            desc.contains("You gain spell resistance equal to 5 + your character level"),
            "Accursed's joined description must carry the real BENEFIT text, not just DESC:"
        );
        // Accursed's own `PRETEXT:` prerequisite is asserted where it now
        // lives: `pcgen_import::feat_prereq_tokens::tests::
        // the_relocated_tokens_are_the_ones_the_corpus_rows_carry`.

        for key in ["Fearless Zeal", "Magnum Opus"] {
            let entry = find(key);
            let desc = entry.description.unwrap_or_else(|| panic!("{key} must still have a description"));
            assert!(
                desc.contains("DEFERRED-WITH-REASON"),
                "{key}'s joined description must carry the deferral diagnostic, not just flavor text"
            );
            assert!(
                desc.contains("uca_feats.lst:"),
                "{key}'s deferral diagnostic must cite a file:line, not a vague reason"
            );
        }

        let stronghold = find("Stronghold");
        let stronghold_desc = stronghold.description.expect("Stronghold must have a joined description");
        assert!(
            !stronghold_desc.contains("DEFERRED-WITH-REASON"),
            "Stronghold's own text is complete and must not carry the deferral diagnostic"
        );
        assert!(
            stronghold_desc.contains("gains a +2 bonus to AC."),
            "Stronghold's joined description must carry its own real BENEFIT text"
        );
        assert!(
            !stronghold_desc.contains("reroll a failed saving throw"),
            "Stronghold's joined description must not carry Magnum Opus's foreign trailing sentence"
        );

        let complete_count = all_feat_tables()
            .iter()
            .filter(|book| book.rule_set == RuleSetId::Uca)
            .flat_map(|book| book.entries.iter())
            .filter(|entry| !entry.description.unwrap_or_default().contains("DEFERRED-WITH-REASON"))
            .count();
        assert_eq!(complete_count, 21, "21 of 23 UCA records are text-complete, not deferred");
    }

    /// `AT-34-E4-001`: the atlas classifier (`v06_work_inventory::classify`,
    /// via `feat_desc_leaks_pi_or_upstream_marker`) demotes all 21 of this
    /// module's own "text-complete" UCA Story feats to `unmeasurable`,
    /// because their SERVED (joined `DESC:`+`BENEFIT:`) description still
    /// opens with PCGen's own `[Not Implemented]` editorial admission. This
    /// re-runs the SAME detector the classifier calls
    /// (`wiring_class::carries_editorial_not_implemented_marker`) against
    /// the real joined catalog output -- not a synthesised string -- to
    /// prove that verdict is a deliberate, corpus-wide-consistent product
    /// decision (`SD31-E2-F3-002`, whose own test names these exact 21
    /// records while fixing a case-sensitivity gap that let them
    /// previously read `done`), never an instrument gap this book's own
    /// work is free to reopen on its own authority. `desc.len() > 150`
    /// mechanically re-confirms `twenty_one_are_text_complete_with_real_benefit_text`'s
    /// claim from the served side: the marker's presence is never because
    /// there is "nothing else to show" -- see this module's cousin
    /// `ultimate_campaign::feat_tables`'s own doc comment, corrected
    /// alongside this test, for why its prior "21 text-complete" target
    /// was stale against the live atlas.
    #[test]
    fn uca_u_bucket_records_still_carry_the_editorial_marker_in_served_form() {
        let uca_keys: Vec<&str> = uca_feats::feat_tables()
            .iter()
            .filter(|e| e.benefit.is_some())
            .map(|e| e.key)
            .collect();
        assert_eq!(uca_keys.len(), 21, "the U-bucket population this proof covers must stay 21");

        for key in uca_keys {
            let record = all_feat_tables()
                .iter()
                .filter(|book| book.rule_set == RuleSetId::Uca)
                .flat_map(|book| book.entries.iter())
                .find(|entry| entry.key == key)
                .unwrap_or_else(|| panic!("'{key}' must be in the UCA aggregate"));
            let desc = record
                .description
                .unwrap_or_else(|| panic!("'{key}' must have a served description"));
            assert!(
                codex_ingest::pcgen_import::wiring_class::carries_editorial_not_implemented_marker(desc),
                "'{key}'s served description no longer carries the editorial marker -- \
                 the unmeasurable verdict may now be stale and worth re-litigating"
            );
            assert!(
                desc.len() > 150,
                "'{key}'s served description should carry substantial real BENEFIT content \
                 beyond the marker, not just the flavor line"
            );
        }
    }

    /// UI's 104 records all carry both `DESC:` and `BENEFIT:` (see
    /// `ultimate_intrigue::feat_tables`'s own module doc comment -- no
    /// upstream splice/truncation defect found), so every joined
    /// description carries both, unlike UCA's two deferred rows.
    #[test]
    fn ui_records_join_desc_and_benefit_with_no_deferrals() {
        let find = |key: &str| {
            all_feat_tables()
                .iter()
                .filter(|book| book.rule_set == RuleSetId::Ui)
                .flat_map(|book| book.entries.iter())
                .find(|entry| entry.key == key)
                .unwrap_or_else(|| panic!("'{key}' must be in the UI aggregate"))
        };

        let acrobatic = find("Acrobatic Spellcaster");
        assert_eq!(acrobatic.category, "Combat");
        let desc = acrobatic.description.expect("Acrobatic Spellcaster must have a joined description");
        assert!(desc.starts_with("Your skillful movements prevent foes from disrupting your spells."));
        assert!(
            desc.contains("creatures denied attacks of opportunity by your Acrobatics check"),
            "Acrobatic Spellcaster's joined description must carry the real BENEFIT text, not just DESC:"
        );
        // Acrobatic Spellcaster's own prerequisite token is asserted where it
        // now lives: `pcgen_import::feat_prereq_tokens::tests::
        // the_relocated_tokens_are_the_ones_the_corpus_rows_carry`.

        let no_deferrals = all_feat_tables()
            .iter()
            .filter(|book| book.rule_set == RuleSetId::Ui)
            .flat_map(|book| book.entries.iter())
            .filter(|entry| entry.description.unwrap_or_default().contains("DEFERRED-WITH-REASON"))
            .count();
        assert_eq!(no_deferrals, 0, "no UI feat record is deferred-with-reason");
    }

    /// Feat keys were globally unique across CRB/APG/ACG and are not
    /// once PU is in. `Endurance` is the only one, and it is a re-listing
    /// rather than two different feats -- see this module's own "Key
    /// collisions" section for the corpus evidence.
    ///
    /// What that costs today: a consumer that flattens the catalog and
    /// looks up by key resolves CRB's row, because CRB is first in book
    /// order. For this collision that is harmless -- the two rows'
    /// ingested fields other than `category` are identical, so
    /// `description_completion` returns the same text either way -- and
    /// the desktop picker shows two rows whose `source` and `category`
    /// tell them apart. The assertion is exact so that a *different*
    /// second feat arriving under an existing key fails here instead of
    /// silently shadowing one book's record with another's.
    #[test]
    fn cross_book_key_collisions_are_exactly_the_known_set() {
        let collide = |tables: &'static [BookFeatTable]| {
            let mut seen: BTreeMap<&'static str, RuleSetId> = BTreeMap::new();
            let mut collisions: Vec<(&'static str, RuleSetId, RuleSetId)> = Vec::new();
            for book in tables {
                for entry in book.entries {
                    match seen.insert(entry.key, book.rule_set) {
                        Some(previous) if previous != book.rule_set => {
                            collisions.push((entry.key, previous, book.rule_set));
                        }
                        _ => {}
                    }
                }
            }
            collisions
        };

        // The original review, kept intact: across the HAND-AUTHORED tables
        // `Endurance` is still the only collision, so a *new* clash between
        // two books' own ingests fails here exactly as it always did.
        assert_eq!(
            collide(hand_authored_feat_tables()),
            vec![("Endurance", RuleSetId::Crb, RuleSetId::Pu)]
        );

        // The joined catalog carries two more, and both are correct rather
        // than defects: a feat one book reprints out of another is a record
        // in *both* books, and this lane's predicate is "a record this book's
        // own table does not hold". Each was checked against its owning
        // corpus record, not inferred from the shared name:
        //
        // * `Feral Combat Training` — `up_feats.lst` carries the comment
        //   "Feral Combat Training copied from Ultimate Combat - consider
        //   INCLUDEing (and .MODding) it" immediately above the record. The
        //   corpus states the reprint itself.
        // * `Extended Animal Focus` — one record in `uw_feats.lst`, the same
        //   Hunter animal-focus feat ACG prints; Ultimate Wilderness reprints
        //   it because it is the book that expands animal focus.
        //
        // `SD31-E6-F8-002` adds three more, and NONE of them is a reprint —
        // each is two genuinely DIFFERENT feats that happen to share a
        // display name, verified against both corpus records' own `DESC:`/
        // `BENEFIT:` text (Decision 10's "a shared NAME is not a duplicate"
        // guard, checked here even though this lane is not the Supersession
        // Register):
        //
        // * `Returning Throw` — `up_feats.lst` (Ultimate Psionics, TYPE
        //   `Psionic.MarksmanBonus`): "Thrown weapons return to your hand."
        //   `isr_feats.lst` (Inner Sea Races, TYPE `Combat.Teamwork`,
        //   `PRERACE:1,RACESUBTYPE=Goblinoid`): a goblinoid-only teamwork
        //   feat about catching an ally's missed thrown weapon. Different
        //   mechanics, different prerequisites, different books.
        // * `Desert Dweller` — `uw_feats.lst` (Ultimate Wilderness,
        //   `PREABILITY:...Favored Terrain ~ Desert`) vs `iswg_feats.lst`
        //   (Inner Sea World Guide, `PRESKILL:Survival=1`+`PRESTAT:CON=13`,
        //   no Favored Terrain requirement at all). Different prerequisite
        //   structure, different `BENEFIT:` text.
        // * `Strangler` — `uc_feats.lst` (Ultimate Combat, grapple/sneak-
        //   attack feat: "spend a swift action to deal your sneak attack
        //   damage") vs `mc_feats.lst` (Monster Codex, lasso feat: "choke
        //   foes with a lasso"). Unrelated combat maneuvers.
        //
        // Pinned exactly, so a further collision — reprint or coincidence —
        // still fails here until it too is checked against its own corpus
        // text.
        let all_collisions = collide(all_feat_tables());
        let (mythic_collisions, other_collisions): (Vec<_>, Vec<_>) =
            all_collisions.into_iter().partition(|(_, _, second)| *second == RuleSetId::Mythic);
        assert_eq!(
            other_collisions,
            vec![
                ("Endurance", RuleSetId::Crb, RuleSetId::Pu),
                ("Extended Animal Focus", RuleSetId::Acg, RuleSetId::Uw),
                ("Feral Combat Training", RuleSetId::Uc, RuleSetId::Upsi),
                ("Returning Throw", RuleSetId::Upsi, RuleSetId::Isr),
                ("Desert Dweller", RuleSetId::Uw, RuleSetId::Iswg),
                ("Strangler", RuleSetId::Uc, RuleSetId::MonsterCodex),
            ]
        );

        // `SD31-E6-F2-007` -- `RuleSetId::Mythic`'s 142 collisions are not
        // hand-enumerated the way the six above are: `decisions.md §10`'s
        // AMENDMENT already establishes, as standing doctrine, that a
        // Mythic feat sharing a key with the base feat it upgrades is the
        // paradigm VARIANT case, not a reprint -- re-litigating each of 142
        // records by hand would restate the operator's own ruling, not
        // verify anything new. What this loop checks INSTEAD is the
        // mechanical, per-record fact that makes the doctrine apply here:
        // every colliding Mythic row's own `PREABILITY:` prerequisite names
        // that exact key under `CATEGORY=FEAT`, i.e. the corpus itself
        // states "you must already hold the base feat to take its mythic
        // form" -- proof of variant-hood a coincidental name clash could
        // never carry. A future collision that is NOT a real mythic-upgrade
        // (a corpus edit, or a new book whose feat happens to share a name)
        // fails this loop rather than sliding in silently.
        assert_eq!(mythic_collisions.len(), 142, "re-derive if a book's feat gap rows change");
        // The per-record `PREABILITY:` proof this comment describes now runs
        // on the side that owns the ingest format, over the same 142 keys:
        // `pcgen_import::feat_prereq_tokens::tests::
        // every_mythic_collision_names_the_base_feat_it_upgrades`. It moved
        // with the tokens (SD-35 `AT-35-E6-003-SWEEP` cycle 3,
        // `decisions.md` §11); it was not weakened or dropped.

        // ... and it really is the same feat re-listed, not a name clash
        // between two different ones: the CRB and PU rows carry the
        // corpus's own identical `DESC:` text, and only the block-derived
        // category differs.
        let rows: Vec<&FeatCatalogRecord> = all_feat_tables()
            .iter()
            .flat_map(|book| book.entries.iter())
            .filter(|entry| entry.key == "Endurance")
            .collect();
        // `SD31-E6-F2-007` -- a third "Endurance" row now exists, Mythic
        // Adventures' own mythic upgrade of the feat (its `PREABILITY:
        // ...,CATEGORY=FEAT,Endurance` prerequisite is checked in the loop
        // above, alongside every other Mythic collision). CRB and PU stay
        // the first two, in the same table order `all_feat_tables()` always
        // yields.
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].description, rows[1].description);
        assert_eq!(rows[0].category, "General");
        assert_eq!(rows[1].category, "WoundThreshold");
        assert_eq!(rows[2].category, "Mythic");
    }


}
