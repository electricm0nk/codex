// -- split from `tests` in src/rules_core/class_feature_pool_catalog.rs (pcgen-touching items only) --
mod tests {
    use codex_ingest::pcgen_import::ingest_record;
    use codex_ingest::pcgen_import::pool_member_tokens;
    use codex::rules_core::class_feature_pool_catalog::*;
    use serde_json::Value;
    use std::collections::BTreeMap;
    use std::path::PathBuf;
    use codex::rules_core::converted_prose;

    /// Proves `VACUOUS_PLACEHOLDER_CLASS_FEATURES`' own claim against the
    /// REAL, committed corpus — not merely asserted in a doc comment. RED
    /// if the corpus ever gains real content for one of these keys (a
    /// genuine PCGen data update), which is exactly when this table must
    /// be revisited (`decisions.md §2`'s "cleared by revisiting the stated
    /// condition"). Also RED if a fourth `empty_selection/*.json` file
    /// ever appears uncovered by the table.
    #[test]
    fn vacuous_placeholder_rows_are_genuinely_empty_in_the_committed_corpus() {
        let dir = repo_root().join("data/corpus/core_rulebook/class_feature/empty_selection");
        let mut found = std::collections::BTreeSet::new();
        for entry in std::fs::read_dir(&dir).expect("empty_selection/ dir exists") {
            let entry = entry.expect("readable dir entry");
            let text = std::fs::read_to_string(entry.path()).expect("readable corpus json");
            let json: Value = serde_json::from_str(&text).expect("valid corpus json");
            let key = json["data"]["key"].as_str().expect("data.key present").to_string();
            assert!(
                VACUOUS_PLACEHOLDER_CLASS_FEATURES.iter().any(|(k, _)| *k == key),
                "unexpected key under empty_selection/, not covered by the closed list: {key}"
            );
            assert!(
                json["data"]["description"].is_null(),
                "{key} now carries a real description -- revisit this table, decisions.md §2"
            );
            let token_keys: std::collections::BTreeSet<&str> =
                ingest_record::token_keys(&json).into_iter().collect();
            assert_eq!(
                token_keys,
                std::collections::BTreeSet::from(["KEY", "CATEGORY", "TYPE"]),
                "{key} carries a token beyond the placeholder's structural KEY/CATEGORY/TYPE -- \
                 revisit this table, decisions.md §2"
            );
            found.insert(key);
        }
        assert_eq!(
            found.len(),
            VACUOUS_PLACEHOLDER_CLASS_FEATURES.len(),
            "every key in VACUOUS_PLACEHOLDER_CLASS_FEATURES must have exactly one corpus file, \
             and vice versa"
        );
    }

    /// Proves `WEAPON_PROFICIENCY_GRANT_CLASS_TABLE_MATCHES`'s own claim
    /// against BOTH the live corpus AND the live
    /// `weapon_tables::CLASS_WEAPON_PROFICIENCIES` table — not merely
    /// asserted in a doc comment. RED if either side ever changes such
    /// that the sets stop matching exactly (which is exactly when this
    /// table must be revisited, `decisions.md §2`'s "cleared by revisiting
    /// the stated condition").
    #[test]
    fn weapon_proficiency_grant_class_table_matches_are_exact() {
        use codex::rules_core::rules_tables::crb::weapon_tables;
        let dir = repo_root().join("data/corpus/core_rulebook/class_feature/weapon_proficiencies");
        for (key, class_id) in WEAPON_PROFICIENCY_GRANT_CLASS_TABLE_MATCHES {
            let file_stub = key
                .rsplit(' ')
                .next()
                .expect("key has a class-name suffix")
                .to_lowercase();
            let path = dir.join(format!("weapon_proficiencies_{file_stub}.json"));
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("readable corpus json at {path:?}: {e}"));
            let json: Value = serde_json::from_str(&text).expect("valid corpus json");
            assert_eq!(json["data"]["key"].as_str(), Some(*key), "corpus file's own key must match");
            assert!(
                json["data"]["description"].is_null(),
                "{key} now carries a real description -- this record may now qualify for a \
                 different, display-bearing rung; revisit this table"
            );
            let auto_token = ingest_record::first_token_value(&json, "AUTO")
                .unwrap_or_else(|| panic!("{key} carries no AUTO token"))
                .to_string();
            let corpus_weapons: std::collections::BTreeSet<String> = auto_token
                .strip_prefix("WEAPONPROF|")
                .unwrap_or_else(|| panic!("{key}'s AUTO token is not a WEAPONPROF grant: {auto_token}"))
                .split('|')
                .filter(|w| *w != "TYPE=Auto")
                .map(|w| w.to_string())
                .collect();
            let table_row = weapon_tables::class_weapon_proficiency(class_id)
                .unwrap_or_else(|| panic!("{class_id} must be a real row in CLASS_WEAPON_PROFICIENCIES"));
            let table_weapons: std::collections::BTreeSet<String> =
                table_row.named.iter().map(|w| w.to_string()).collect();
            assert_eq!(
                corpus_weapons, table_weapons,
                "{key}'s corpus AUTO:WEAPONPROF list must be an EXACT set match for \
                 {class_id}'s named list in CLASS_WEAPON_PROFICIENCIES -- a near-match must stay \
                 unclosed (Monk's own \"Flurry of Blows\"/\"Unarmed Strike\" mismatch is exactly \
                 why this table only names Bard/Druid/Rogue)"
            );
        }
    }

    /// Proves `WEAPON_AND_ARMOR_PROFICIENCY_GRANT_CLASS_TABLE_MATCHES`'s
    /// own claim against BOTH the live corpus AND both live tables — not
    /// merely asserted in a doc comment. RED if either table, or the
    /// corpus record, ever changes such that a listed class's weapon OR
    /// armor content stops being an exact set match (`decisions.md §2`'s
    /// "cleared by revisiting the stated condition").
    #[test]
    fn weapon_and_armor_proficiency_grant_class_table_matches_are_exact() {
        use codex::rules_core::rules_tables::crb::weapon_tables;
        let dir = repo_root().join("data/corpus/core_rulebook/class_feature/weapon_and_armor_proficiency");
        for (key, class_id) in WEAPON_AND_ARMOR_PROFICIENCY_GRANT_CLASS_TABLE_MATCHES {
            let class_name = key.rsplit(' ').next().expect("key has a class-name suffix");
            let mut matched_file = None;
            for entry in std::fs::read_dir(&dir).expect("dir exists") {
                let entry = entry.expect("readable dir entry");
                let text = std::fs::read_to_string(entry.path()).expect("readable corpus json");
                let json: Value = serde_json::from_str(&text).expect("valid corpus json");
                if json["data"]["key"].as_str() == Some(*key) {
                    matched_file = Some(json);
                    break;
                }
            }
            let json = matched_file.unwrap_or_else(|| panic!("no corpus file found for key {key}"));
            assert!(
                !json["data"]["description"].is_null(),
                "{key} must carry a real description -- this table is only for the DISPLAY-\
                 bearing combined records, not the internal weapon-only chassis rows"
            );
            // Weapon-side: named list (if any) must be an exact set match.
            let named_weapons: std::collections::BTreeSet<String> =
                ingest_record::first_token_value(&json, "AUTO")
                .and_then(|v| v.strip_prefix("WEAPONPROF|"))
                .map(|list| {
                    list.split('|')
                        .filter(|w| !w.starts_with("TYPE=") && !w.starts_with('!'))
                        .map(|w| w.to_string())
                        .collect()
                })
                .unwrap_or_default();
            let table_row = weapon_tables::class_weapon_proficiency(class_id)
                .unwrap_or_else(|| panic!("{class_id} must be a real row in CLASS_WEAPON_PROFICIENCIES"));
            let table_named: std::collections::BTreeSet<String> =
                table_row.named.iter().map(|w| w.to_string()).collect();
            assert_eq!(named_weapons, table_named, "{key} named weapon list must match exactly");

            // Armor-side: verified independently by
            // `class_armor_proficiency_tests` in `weapon_tables.rs`
            // against this SAME corpus file. Re-derive it here too so a
            // caller reading only this test still sees the full claim.
            let armor_row = weapon_tables::class_armor_proficiency(class_id)
                .unwrap_or_else(|| panic!("{class_id} must be a real row in CLASS_ARMOR_PROFICIENCIES"));
            let ability_tokens: Vec<String> = ingest_record::token_values(&json, "ABILITY")
                .into_iter()
                .map(str::to_string)
                .collect();
            let has = |needle: &str| ability_tokens.iter().any(|v| v.contains(needle));
            assert_eq!(has("Armor Prof ~ Light"), armor_row.light, "{key} light armor");
            assert_eq!(has("Armor Prof ~ Medium"), armor_row.medium, "{key} medium armor");
            assert_eq!(has("Armor Prof ~ Heavy"), armor_row.heavy, "{key} heavy armor");
            assert_eq!(has("Shield Prof ~ Tower"), armor_row.tower_shield, "{key} tower shield");
            let has_plain_shield_prof =
                ability_tokens.iter().any(|v| v.split('|').any(|part| part == "Shield Prof"));
            assert_eq!(has_plain_shield_prof, armor_row.shield, "{key} shield (non-tower)");
            let _ = class_name;
        }
    }

    /// `AT-34-E3-001`'s `class_feature_owner_matched_by_name_but_record_
    /// not_held_by_engine` mechanism (`decisions.md §14`, 346 of 1,006
    /// `core_rulebook` bucket-B units at this cycle's start): re-derives,
    /// from the live `docs/work-inventory.json` and the live corpus this
    /// module already reads, WHY each unit in this mechanism's population
    /// is not served by [`load_pool_catalog`] -- the exact gate this
    /// module's own filter (`load_class_feature_catalog`) refuses it at,
    /// walked in the SAME order that function checks them, so the count is
    /// never a re-narration.
    ///
    /// **Every gate below is load-bearing, not this cycle's own
    /// invention** -- each was hand-verified against a real corpus finding
    /// by an earlier cycle (this file's own doc comments cite them). This
    /// test proves the negative the receipt reports: none of the 346 is a
    /// narrow catalog-widening bug this cycle can close without either (a)
    /// new engine wiring for a genuinely mechanical/computed record, or (b)
    /// new ingest work for a record with no player-facing description at
    /// all. The seven buckets below are that population's exact partition
    /// (`decisions.md §15`: a named remainder, not "the rest").
    #[test]
    fn class_feature_owner_matched_but_not_held_346_sub_causes_are_named_and_sum_exactly() {
        let repo_root = repo_root();
        let inventory_text = std::fs::read_to_string(repo_root.join("docs/work-inventory.json"))
            .expect("docs/work-inventory.json is readable");
        let inventory: Value =
            serde_json::from_str(&inventory_text).expect("docs/work-inventory.json is valid JSON");
        let units = inventory["units"].as_array().expect("units is an array");
        let mechanism_units: Vec<(String, String)> = units
            .iter()
            .filter(|u| {
                u["book"].as_str() == Some("core_rulebook")
                    && u["status"].as_str() == Some("engine-does-not-hold")
                    && u["evidence"].as_str()
                        == Some("class_feature_owner_matched_by_name_but_record_not_held_by_engine")
            })
            .map(|u| {
                (
                    u["book"].as_str().unwrap_or_default().to_string(),
                    u["corpus_key"].as_str().unwrap_or_default().to_string(),
                )
            })
            .collect();
        let population = mechanism_units.len();

        let corpus_root = repo_root.join("data/corpus");
        let mut reasons: BTreeMap<&'static str, u32> = BTreeMap::new();
        for (book, key) in &mechanism_units {
            let cf_dir = corpus_root.join(book).join("class_feature");
            let mut files = Vec::new();
            walk_json_files(&cf_dir, &mut files);
            let mut found = None;
            for file in &files {
                let Ok(text) = std::fs::read_to_string(file) else { continue };
                let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
                if doc["data"]["key"].as_str() == Some(key.as_str()) {
                    found = Some(doc);
                    break;
                }
            }
            let Some(doc) = found else {
                *reasons.entry("no_corpus_record_found").or_default() += 1;
                continue;
            };
            let data = &doc["data"];
            let raw_desc = data["description"].as_str();
            let Some(raw_desc) = raw_desc else {
                // No `DESC:` at all -- a genuinely internal, never
                // player-facing bookkeeping row (`ADD:SPELLCASTER`,
                // `SPELLKNOWN`, `SPELLLEVEL`, ...). Real ingest work
                // (writing a description that does not exist upstream) or
                // a reclassification, not a catalog fix.
                *reasons.entry("description_is_null_internal_bookkeeping").or_default() += 1;
                continue;
            };
            if !is_real_description_value(raw_desc) {
                *reasons.entry("description_not_real_value").or_default() += 1;
                continue;
            }
            let owning_class = data["class"].as_str().unwrap_or("");
            if carries_class_specific_level_phrase(raw_desc, owning_class) {
                // Prose states a value that scales with the OWNING class's
                // level (e.g. "200 gp per wizard level") -- Decision 7
                // condition 2 ("nothing to compute") genuinely fails; this
                // needs a real per-character computation, not a serve.
                *reasons.entry("class_specific_level_phrase").or_default() += 1;
                continue;
            }
            if !pool_member_tokens::has_no_engine_effect_token(data) {
                // Carries a real mechanical token (`ADD`, `ABILITY`,
                // `AUTO`, `BONUS`, `DEFINE`, `SPELLS`, ...) alongside its
                // description -- a genuine mechanic, not prose-only.
                *reasons.entry("engine_effect_token_present").or_default() += 1;
                continue;
            }
            if pool_member_tokens::is_archetype_locked(data) {
                *reasons.entry("archetype_locked").or_default() += 1;
                continue;
            }
            if pool_member_tokens::carries_more_than_one_desc_segment(data)
                && !pool_member_tokens::shipped_description_is_the_already_regenerated_safe_multi_desc_join(
                    data,
                    raw_desc,
                )
            {
                // Every one of these, hand-checked this cycle, carries a
                // genuine `PRE*`-gated alternative-branch shape (mutually
                // exclusive choices or level bands), not the `class_
                // feature_option_pool` cycle's safe sequential-continuation
                // shape -- joining them would show every branch at once,
                // the exact silent-truncation-turned-over-disclosure defect
                // that gate exists to prevent.
                *reasons.entry("multi_desc_segment_not_regenerated").or_default() += 1;
                continue;
            }
            if raw_desc_has_a_bare_percent_reference_no_pipe_tail_can_resolve(raw_desc) {
                *reasons.entry("bare_percent_reference").or_default() += 1;
                continue;
            }
            // Cycle 17: the render-and-refuse pair became the converted-prose join. The two
            // old buckets (`dropped_pcgen_args`, `leaked_pcgen_syntax`) were two shapes of one
            // fact — the ingest row states words this engine cannot finish — and the converter
            // now decides that once, at ingest, by stating no prose for such a record.
            let converted = converted_prose::description_for(book, "class_feature", key);
            let Some(converted) = converted else {
                *reasons.entry("converter_states_no_prose").or_default() += 1;
                continue;
            };
            if carries_unimplemented_marker(&converted) {
                *reasons.entry("carries_unimplemented_marker").or_default() += 1;
                continue;
            }
            // Passes every gate this catalog runs -- genuinely already
            // SERVED by `load_pool_catalog`/`pool_catalog_index`. Every one
            // hand-sampled this cycle (`Sorcerer Bonus Spell L4 ~ Elemental
            // Body I`, `Sorcerer Bonus Spell L1 ~ Bless`, ...) is still
            // blocked at `classify()`'s own promotion gate: either its
            // `wiring_class` is not `"display"` (`computed`/`ambiguous`/
            // `static`/`derived` -- a real magnitude/scaling signal the
            // catalog's render-and-refuse gate alone cannot see), or its
            // prose trips `closure_states_universal_sheet_modifier`'s
            // `"size bonus"` cue (a per-character numeric effect, not
            // static flavor text). Both gates are `classify()`'s, deliberate
            // and correct per Decision 7 -- a text-complete promotion for
            // either shape would misreport a record that still needs a
            // real computation as merely displayed.
            *reasons.entry("catalog_serves_it_but_classify_wiring_class_gate_blocks_promotion")
                .or_default() += 1;
        }

        let total: u32 = reasons.values().sum();
        assert_eq!(
            total as usize, population,
            "the seven named sub-causes must partition the WHOLE mechanism population \
             exactly, decisions.md §15 -- got {reasons:?} summing to {total} against a \
             population of {population}"
        );
        for (k, v) in &reasons {
            eprintln!("AT-34-E3-001 class_feature_owner_matched sub-cause: {v} | {k}");
        }
    }

    /// Proves `WIZARD_SCHOOL_SPELL_LIST_KEY_OWNER`'s own claim against BOTH
    /// the live corpus AND the live
    /// `wizard_spell_list::wizard_school_zero_level_spells` join — not
    /// merely asserted in a doc comment. RED if either side ever changes
    /// such that the sets stop matching exactly (`decisions.md §2`'s
    /// "cleared by revisiting the stated condition").
    #[test]
    fn wizard_school_spell_list_key_owner_matches_are_exact() {
        use codex::rules_core::rules_tables::crb::spell_list::Pf1SchoolId;
        use codex::rules_core::rules_tables::crb::wizard_spell_list::wizard_school_zero_level_spells;
        let dir = repo_root().join("data/corpus/core_rulebook/class_feature");
        let schools: &[(&str, &str, Pf1SchoolId)] = &[
            ("Abjuration Wizard Spells", "abjuration_wizard_spells", Pf1SchoolId::Abjuration),
            ("Conjuration Wizard Spells", "conjuration_wizard_spells", Pf1SchoolId::Conjuration),
            ("Divination Wizard Spells", "divination_wizard_spells", Pf1SchoolId::Divination),
            ("Enchantment Wizard Spells", "enchantment_wizard_spells", Pf1SchoolId::Enchantment),
            ("Evocation Wizard Spells", "evocation_wizard_spells", Pf1SchoolId::Evocation),
            ("Illusion Wizard Spells", "illusion_wizard_spells", Pf1SchoolId::Illusion),
            ("Necromancy Wizard Spells", "necromancy_wizard_spells", Pf1SchoolId::Necromancy),
            ("Transmutation Wizard Spells", "transmutation_wizard_spells", Pf1SchoolId::Transmutation),
            ("Universal Wizard Spells", "universal_wizard_spells", Pf1SchoolId::Universal),
        ];
        assert_eq!(
            schools.len(),
            WIZARD_SCHOOL_SPELL_LIST_KEY_OWNER.len(),
            "every WIZARD_SCHOOL_SPELL_LIST_KEY_OWNER entry must be checked here, and vice versa"
        );
        for (key, dir_stub, school) in schools {
            assert_eq!(wizard_school_spell_list_key_owner(key), Some("class:wizard"));
            let path = dir.join(dir_stub).join(format!("{dir_stub}.json"));
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("readable corpus json at {path:?}: {e}"));
            let json: Value = serde_json::from_str(&text).expect("valid corpus json");
            assert_eq!(json["data"]["key"].as_str(), Some(*key), "corpus file's own key must match");
            assert!(
                json["data"]["description"].is_null(),
                "{key} now carries a real description -- this record may now qualify for a \
                 different, display-bearing rung; revisit this table"
            );
            let spellknown = ingest_record::first_token_value(&json, "SPELLKNOWN")
                .unwrap_or_else(|| panic!("{key} carries no SPELLKNOWN token"))
                .to_string();
            let corpus_spells: std::collections::BTreeSet<String> = spellknown
                .split('|')
                .nth(2)
                .unwrap_or_else(|| panic!("{key}'s SPELLKNOWN token has a spell-list segment"))
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            let table_spells: std::collections::BTreeSet<String> =
                wizard_school_zero_level_spells(*school).into_iter().map(|s| s.to_string()).collect();
            assert_eq!(
                table_spells, corpus_spells,
                "{key}: the wizard_spell_list/spell_list join disagrees with the real corpus \
                 SPELLKNOWN token"
            );
        }
    }

    /// `AT-34-E3-001` cycle 9's own re-derivation: the dispatch's own
    /// inherited claim of a 161/81 excluded/non-excluded split against this
    /// mechanism's 242-unit `core_rulebook` population did NOT match a
    /// direct query against the live corpus (218 excluded / 24
    /// non-excluded, re-derived by `docs/release/SD-34-book-completion/
    /// artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_owner_matched_cycle_receipt_9.md`).
    /// This test proves that split mechanically, reusing this file's own
    /// sibling test's exact gate-walk (so it can never independently drift
    /// from what `load_pool_catalog` actually refuses each unit for).
    ///
    /// **The excluded-class literal below is a frozen snapshot, not a live
    /// import.** `class_feature_grant_consumer`'s own
    /// `ANTI_FABRICATION_GATE_EXCLUDED_CLASSES` (the gate this cycle's
    /// dispatch brief and `decisions.md §18` both name) was renamed and
    /// repurposed to `LEVEL_UP_PILLAR_FILTERED_CLASSES` (Druid/Monk only)
    /// by a sibling lane's SAME-wave, concurrently-landed `§18` fix
    /// (anti-fabrication is now enforced by corpus-citation, not a class
    /// allowlist), then REMOVED entirely by a later bucket-B batch cycle
    /// (Druid/Monk widened the same way) -- importing either constant is
    /// no longer possible, so this test keeps its own frozen, hand-written
    /// copy of the ORIGINAL seven-class definition and characterizes THIS
    /// mechanism's population split by that definition regardless of what
    /// production code currently excludes.
    ///
    /// **242 -> 239 (SD-34 wave-9 shared regeneration, prior cycle):**
    /// `docs/work-inventory.json` was regenerated once, after this
    /// characterization was first written; 3 excluded-class units left
    /// this mechanism's population (moved status by the SAME-wave
    /// citation-gate widening for Wizard/Bard/Paladin/Cleric/Sorcerer, not
    /// by this test's own lane) -- confirmed live, not merely inferred: a
    /// fresh per-class re-group against the CURRENT `docs/work-
    /// inventory.json` gives Sorcerer 137, Cleric 38 (was 39), Monk 25,
    /// Wizard 5 (was 7), Paladin 5, Bard 4, Druid 1 -- **215**, not 218
    /// (Monk and Druid are BYTE-FOR-BYTE unchanged, matching wave-9's own
    /// "0 Druid/Monk movement confirmed" finding; only Cleric and Wizard
    /// moved). The non-excluded 24 (below) are confirmed UNCHANGED by the
    /// same re-group. This is an instrument-correction to a STALE pinned
    /// count this test's own prior author never re-checked after the
    /// regeneration landed -- re-derive again with the query in this
    /// test's own body before trusting either number further.
    ///
    /// **This lane owns only the non-excluded remainder** (a sibling lane
    /// owns the 215 excluded-class units, gated on an operator ruling on
    /// `OPEN-ISSUES.md` rows 330/338 this test does not decide). Of the 24
    /// non-excluded units: 18 carry no corpus description at all (the
    /// zero-description internal-bookkeeping sub-cause `atlas-defects.md`
    /// already names as the OPEN definitional question -- left in bucket B,
    /// never reclassified into X or U by this test or this cycle); the
    /// remaining 6 carry a REAL description but are correctly refused by
    /// one of this catalog's own pre-existing, independently-tested safety
    /// gates (an unresolvable `%N` argument, a class-level-scaled phrase,
    /// or a genuine mechanical token such as `ABILITY`/`SELECT`) -- each of
    /// those 6 already has its own dedicated live-corpus regression test in
    /// this module (`bleeding_attack_is_refused_for_an_unresolvable_
    /// percent_argument`, the Knockback/Finesse-Rogue/Skill-Mastery/
    /// Improved-Evasion cases this file's doc comments cite). None of the
    /// 24 is a narrow catalog-widening bug this cycle can close: every one
    /// needs either real per-character grant/formula wiring (a talent pick
    /// actually consumed by `pilot_compute`, a sneak-attack-dice-scaled
    /// damage formula) or new ingest work no engine change can supply.
    #[test]
    fn class_feature_owner_matched_non_excluded_remainder_is_24_and_named_by_subcause() {
        // Frozen snapshot of `class_feature_grant_consumer::ANTI_FABRICATION_GATE_EXCLUDED_
        // CLASSES` as it stood for the whole of this wave's `docs/work-inventory.json` (see
        // this fn's own doc comment for why a live import is no longer possible).
        const ANTI_FABRICATION_GATE_EXCLUDED_CLASSES: [&str; 7] =
            ["wizard", "bard", "paladin", "cleric", "sorcerer", "druid", "monk"];

        let repo_root = repo_root();
        let inventory_text = std::fs::read_to_string(repo_root.join("docs/work-inventory.json"))
            .expect("docs/work-inventory.json is readable");
        let inventory: Value =
            serde_json::from_str(&inventory_text).expect("docs/work-inventory.json is valid JSON");
        let units = inventory["units"].as_array().expect("units is an array");
        let mechanism_units: Vec<String> = units
            .iter()
            .filter(|u| {
                u["book"].as_str() == Some("core_rulebook")
                    && u["status"].as_str() == Some("engine-does-not-hold")
                    && u["evidence"].as_str()
                        == Some("class_feature_owner_matched_by_name_but_record_not_held_by_engine")
            })
            .map(|u| u["corpus_key"].as_str().unwrap_or_default().to_string())
            .collect();

        let corpus_root = repo_root.join("data/corpus/core_rulebook/class_feature");
        let mut files = Vec::new();
        walk_json_files(&corpus_root, &mut files);
        let mut by_key: BTreeMap<String, Value> = BTreeMap::new();
        for file in &files {
            let Ok(text) = std::fs::read_to_string(file) else { continue };
            let Ok(doc) = serde_json::from_str::<Value>(&text) else { continue };
            if let Some(k) = doc["data"]["key"].as_str() {
                by_key.insert(k.to_string(), doc);
            }
        }

        let mut excluded = 0u32;
        let mut null_desc = 0u32;
        let mut real_desc_refused = 0u32;
        let mut real_desc_unrefused_unexpected: Vec<String> = Vec::new();

        for key in &mechanism_units {
            let doc = by_key.get(key).unwrap_or_else(|| panic!("no corpus record for {key}"));
            let data = &doc["data"];
            let owner = data["class"].as_str().unwrap_or_default().to_ascii_lowercase();
            if ANTI_FABRICATION_GATE_EXCLUDED_CLASSES.contains(&owner.as_str()) {
                excluded += 1;
                continue;
            }
            let Some(raw_desc) = data["description"].as_str() else {
                null_desc += 1;
                continue;
            };
            // Non-excluded, real-description unit: it must be refused by
            // one of the catalog's own gates, never silently unaccounted
            // for -- the same gate walk the sibling 346-population test
            // above runs, restricted to just this record.
            let owning_class = data["class"].as_str().unwrap_or("");
            let refused = !is_real_description_value(raw_desc)
                || carries_unimplemented_marker(raw_desc)
                || carries_class_specific_level_phrase(raw_desc, owning_class)
                || !pool_member_tokens::has_no_engine_effect_token(data)
                || pool_member_tokens::is_archetype_locked(data)
                || (pool_member_tokens::carries_more_than_one_desc_segment(data)
                    && !pool_member_tokens::shipped_description_is_the_already_regenerated_safe_multi_desc_join(
                        data,
                        raw_desc,
                    ))
                || raw_desc_has_a_bare_percent_reference_no_pipe_tail_can_resolve(raw_desc)
                // The converted-prose join, which replaced the render-and-refuse pair in cycle
                // 17 — a record the package states no prose for, or states marked-up prose
                // for, is refused exactly as the two render gates used to refuse it.
                || converted_prose::description_for("core_rulebook", "class_feature", key)
                    .is_none_or(|text| carries_unimplemented_marker(&text));
            if refused {
                real_desc_refused += 1;
            } else {
                real_desc_unrefused_unexpected.push(key.clone());
            }
        }

        assert!(
            real_desc_unrefused_unexpected.is_empty(),
            "found a non-excluded, real-description unit this cycle's gate walk does NOT \
             refuse -- this WOULD be a narrow catalog-widening closure, re-investigate: \
             {real_desc_unrefused_unexpected:?}"
        );
        // Re-derived 2026-09-01 against `docs/work-inventory.json` at this cycle's HEAD (this
        // test's own live query above, re-run standalone): 215 -> 213. `mechanism_units.len()`
        // itself moved 239 -> 237 (two units no longer carry `status ==
        // "engine-does-not-hold"` with this evidence string; the EXCLUDED-CLASS ROSTER, the 7
        // names above, is unchanged and not the cause -- only membership in `mechanism_units`
        // shrank). `null_desc`/`real_desc_refused` below are unaffected (18/6 still hold live).
        // Wave 50 re-derivation: 213 -> 138. `classify()`'s `Kind::ClassFeature` owner-matched
        // arm gained two new rungs this wave (Core Domain/Sorcerer Domain, Sorcerer Bonus Spell
        // L1-L9 -- both genuinely proseless, set-shaped internal chassis grants, `decisions.md
        // §22` wave-50 update) that promote 75 `core_rulebook` units straight to `grounded`
        // rather than the `class_feature_owner_matched_by_name_but_record_not_held_by_engine`
        // evidence this test's own `mechanism_units` filter reads -- all 75 have `class: "Cleric"`
        // or `class: "Sorcerer"`, both already in the excluded-class roster above (31 Core
        // Domain + 22 Sorcerer Domain + 22 Sorcerer Bonus Spell = 75, 213 - 75 = 138, confirmed
        // by re-running this test's own live query standalone post-regen). This lane's OWN
        // owned population (`null_desc`/`real_desc_refused` below, neither Cleric nor Sorcerer
        // was ever counted there) is unaffected -- still 18/6, still summing to 24.
        // SD-35 AT-35-E2-005 re-derivation (2026-09-08): 138 -> 1, 18 -> 0, 6 -> 0. The first
        // corpus-wide sheet-rule pass stamped `sheet-complete` (AT-35-E2-003's rung) on every
        // `engine-does-not-hold` unit whose converted `SheetRule` renders for the probe
        // character, so `mechanism_units` (this test's own live query, re-run standalone
        // post-regen: `python3 -c` over `docs/work-inventory.json` filtering book/status/
        // evidence exactly as above) shrank 162 -> 1 -- the one survivor is excluded-class
        // (its record is in `data/sheet_rules/_refused.json`). This lane's own owned
        // population (`null_desc` + `real_desc_refused`) is 0: every one of the 24 rendered
        // as a sheet line and left the mechanism.
        // SD-35 AT-35-E3-001 re-derivation (2026-09-08): 1 -> 0. Term-level refusal (the
        // converter no longer deletes a whole record because one of its tokens will not
        // lower) converted the last survivor, so it rendered as a sheet line and left the
        // mechanism. `mechanism_units` is now empty: this test's own live query over
        // `docs/work-inventory.json`, re-run standalone post-regen, returns 0 rows.
        assert_eq!(excluded, 0, "excluded-class population (sibling lane's, do not touch)");
        assert_eq!(null_desc, 0, "non-excluded, zero-description internal-bookkeeping (bucket B, OPEN question, left untouched)");
        assert_eq!(real_desc_refused, 0, "non-excluded, real-description, correctly refused by an existing safety gate (needs real engine wiring, not this cycle's scope)");
        assert_eq!(excluded + null_desc + real_desc_refused, mechanism_units.len() as u32);
        assert_eq!(null_desc + real_desc_refused, 0, "this lane's own owned population (24 before SD-35 AT-35-E2-005's pass)");
    }

    fn repo_root() -> PathBuf {
        codex_ingest::repo_root()
    }


}
