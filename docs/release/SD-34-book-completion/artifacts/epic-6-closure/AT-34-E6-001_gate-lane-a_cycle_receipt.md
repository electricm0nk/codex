# Cycle wave-23-gate-lane-a — Epic 6 (gate remediation) / AT-34-E6-001 (Lane A)

- **Commit SHA:** 654015b8d9 (last figure-moving commit in this cycle at time of writing;
  see `commit_sha` in the structured return for the actual final pushed SHA including any
  trailing docs-only commit)
- **Files touched:** 44 files under `src/`, `tests/`, `apps/desktop/src-tauri/src/` — full
  list: `git diff --stat a3d38746b8...654015b8d9 -- src/ tests/ apps/desktop/src-tauri/src/`
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (scoped to this cycle's own 44 changed files;
  the literal `BASE_BRANCH=merge-base-with-origin/develop` full-package diff is dominated by
  22 prior waves' legitimate `sd13_*`/`sd25_*`-prefixed test-file content, this repo's own
  naming convention, not a defect — see the first cycle commit's note)
- **Wired-integration audit result:** OK_NO_TOKENS (every `placeholder`/`hack`/`todo` hit in
  this cycle's own diff is either a deletion, or new prose *about* an already-reviewed
  non-stub shape — none is a new stub marker; see the receipt body below for the full
  `git diff` grep)
- **Acceptance criterion (verbatim from this cycle's dispatch brief — a wave-23 gate-
  remediation lane, not the canonical `epic-breakdown.md` AT-34-E6-001 final-acceptance
  scan, which stays a separate, later criterion; see `## Note on the AT-34-E6-001 label`
  below):** "GATE LANE A — the Rust test suites: root-lib, root-full, desktop, reach... Bar:
  every stage you own exits 0, with ZERO stages that were green going red."

## Note on the AT-34-E6-001 label

This cycle's dispatch brief reused the label `AT-34-E6-001` for wave 23's three gate-
remediation lanes (A/B/C), distinct from `kanban.md` row 26's canonical `AT-34-E6-001`
("final-acceptance-scan", still `not-started`). This receipt is filed alongside Lane B's
(`AT-34-E6-001_gate-lane-b_cycle_receipt.md`) under a lane-suffixed filename so as not to
collide with the earlier, genuine final-acceptance-scan attempt already on disk at the plain
`AT-34-E6-001_cycle_receipt.md` path (dated 2026-08-29, FAIL verdict, unrelated to this wave).
`kanban.md` row 26 is left untouched — this cycle's own work does not satisfy that criterion.

## What was found (fable-review.md §7's root-cause table, re-derived)

Wave 22's oracle-verdict restamp (`decisions.md §19`) introduced two status words,
`oracle-agree` and `oracle-unverifiable`, that the producer's doneness table had no mapping
for (fixed separately, `58b4f837cc`, already landed before this cycle). That fix cleared the
*dominant* cause but not the whole gate: root-lib and root-full also carried real code gaps
(a status-match list that needed the same two-status widening) and — the larger share of
root-full's population — **stale hardcoded census/exclusion assertions across many
independent SD-34 closure cycles that landed real, tested, corpus-verified content between
2026-08-27 and 2026-08-29 without a following full-suite run.** Every fix below traces to a
named, dated, already-landed commit; none is a guess.

## Figures + their re-derive commands (root-lib, fully closed)

| Test | Old (broken) | New (live) | Command |
|---|---:|---:|---|
| `companion_chassis::grant_token_only_rows_dispatch_to_already_held_content` | `HELD_STATUSES` 3 entries | 5 entries (+`oracle-agree`/`oracle-unverifiable`) | code fix, not a count — same reading as `58b4f837cc` |
| `class_feature_pool_catalog::…_non_excluded_remainder_is_24…` | excluded=215 | excluded=213 | `python3 /tmp/cargo-sd34-at-34-e6-001/derive_excluded.py` |
| `formula_interpreter_corpus_wide::f1_population_matches…` | F1=5,400 | F1=5,231 | `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus` |

`cargo test --locked --lib` (isolated `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001`, this cycle's tree): **3,022 passed; 0 failed; 14 ignored** (was 3,019 passed / 3 failed at cycle start; 2 of the original 5 root-lib failures were already cleared by `58b4f837cc` alone).
Floor `BASELINE_ROOT_LIB_TESTS=2336` — well clear.

## Figures + their re-derive commands (root-full — every touched file individually verified)

| File | Old | New | Command / re-derive |
|---|---:|---:|---|
| `src/bin/ingest_races.rs` (2 tests) | 175 records, book skip-list missing `core_rulebook` | 184 records; `core_rulebook` added to skip-list | `python3 /tmp/cargo-sd34-at-34-e6-001/derive_race_trait_census.py`, `check_crb_nondefault.py` |
| `tests/fixtures/rules_core/formula-interpreter-family-fixtures.json` (F6) | missing `CLASSLEVEL::Shaman` binding | binding added, `expected` unchanged | evaluator's own documented `CLASSLEVEL::<name>` requirement, `formula_interpreter.rs` |
| `tests/duergar_invisibility_sla_reaches_a_player_via_monster_codex.rs` | `LOADED_BOOKS` missing `bestiary_3` | added, matching `race_catalog.rs::RACE_CORPUS_BOOKS` | direct file read, both lists |
| 10× `tests/sd18_cleric_level*_widening.rs` | 2-id exclusion gap | +`weapon_and_armor_proficiency` / +`domain.generic…rebukedeathtimes` | `bash /tmp/cargo-sd34-at-34-e6-001/extract_cleric_ids.sh`, `extract_cleric21_ids.sh` |
| 5× `tests/sd18_wizard_level*_widening.rs` | exclusion gap | +`corpus_record.` prefix, +`weapon_and_armor_proficiency` | `bash /tmp/cargo-sd34-at-34-e6-001/extract_wizard_ids.sh` |
| 8× `tests/sd18_sorcerer_level*_widening.rs` | exclusion gap | +`bloodline_feat_pool.slot_count` | `bash /tmp/cargo-sd34-at-34-e6-001/extract_sorcerer_ids.sh` |
| `tests/sd18_bard_level18_widening.rs` | exclusion gap | +`suggestion_dc`/`mass_suggestion_dc` | direct source read, `mod.rs:49070-49110` |
| `tests/sd20_skill_allocation_class_skill.rs` (3 tests) + `sd20_skill_allocation_max_rank_cap.rs` + `sd20_skill_allocation_untrained.rs` + `sd20_tabletop_readiness_integration.rs` + `tests/fixtures/wire/sd20/skill_allocation_parity.json` | 3-skill Fighter list; `skill:perception` unmapped | 62-skill Fighter list; `skill:not_a_real_pf1_skill` substitute | live list extracted verbatim from the fable-review's own captured cargo output (`extract_left_list.py`), `skill_allocation.rs` unchanged since capture (`git log 03ba5fcdb2..HEAD -- src/rules_core/skill_allocation.rs` → empty) |
| `tests/sd24_identifier_discipline_audit.rs` + `sd26_identifier_discipline_audit.rs` (shared cause) + `src/rules_core/cache_gen/equipment_gap.rs` | citation missing `tests/` prefix | prefix added | exemption's own documented convention |
| `tests/sd24_wired_integration_audit.rs` (2 tests) + `class_feature_grant_consumer.rs` ×2 + `apps/desktop/src-tauri/src/reach_gate.rs` ×1 | "todo" substring in real path citations; Tophet "hack" unhandled; 6 placeholder-shape hits unhandled | citations reworded; 2nd named hack exception; bucket F added | direct source read each hit |
| `tests/sd26_cache_acg.rs` (2 tests) + `sd26_cache_apg.rs` (2 tests) | `class_id.unwrap()` panics on `VISIBLE:NO` helper records | filtered on `class_id.is_some()` | `python3 /tmp/cargo-sd34-at-34-e6-001/check_apg_classes.py`, `check_apg_spells.py` |
| `tests/sd27_advanced_race_guide_cache_shape.rs` (2 tests) | feat count 187 unfiltered | filtered to 3 real PF1 categories | `python3 /tmp/cargo-sd34-at-34-e6-001/count_arg_feats.py` |
| `tests/sd27_alternate_racial_trait_reachability.rs` (2 tests) | 910 | 919 | same 9-record delta as `ingest_races.rs` above |

`cargo test --locked --no-fail-fast` targeted at every one of the above (24 sd18 binaries +
9 sd20/sd24/sd26/sd27/duergar/formula-fixture/pi-sweep/ingest_races binaries, isolated
`CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001`, run AFTER the last commit touching each):
**33 of 33 binaries green, 0 failed** (`verify-batch1.log`, `verify-batch3.log`). No stage
that was green went red — every touched binary's OWN prior-red state is the only state
changed.

## Row-count command output

```
$ grep -c "^test result: ok" /tmp/cargo-sd34-at-34-e6-001/verify-batch1.log /tmp/cargo-sd34-at-34-e6-001/verify-batch3.log
verify-batch1.log:24
verify-batch3.log:9
```
33 of 33 targeted binaries green (24 sd18 cleric/wizard/sorcerer/bard + 9 sd20/sd24/sd26/sd27/
duergar/formula-fixture/pi-table-sweep/ingest_races), plus root-lib's own live run
(3,022 passed / 0 failed).

## Build scope verified

- `cargo test --locked --no-run` (whole workspace, isolated target dir, at `3199f8e4c6`,
  AFTER the last commit that could move a compile result — this cycle's own brace-escaping
  fix): **exit 0.**
- `cargo test --locked --lib`: **3,022 passed / 0 failed** (see above).
- `cargo test --locked` in `apps/desktop/src-tauri` (separate cargo workspace, isolated
  `CARGO_TARGET_DIR=/tmp/cargo-sd34-desktop-verify`, run AFTER this cycle's `companion_catalog.rs`/
  `reach_gate.rs` fixes): **565 passed; 7 failed** (was 544 passed / 28 failed at cycle start
  — 21 of 28 closed). The `PREHD` render fix alone cleared 15+ cascading companion_catalog/
  reach_gate failures; the `ClassArmorProficiency`/`ClassSkillList` registry additions cleared
  `every_ingested_record_type_is_classified`.
- Full untargeted `cargo test --locked --no-fail-fast` over the whole ~600-suite root
  workspace was **not completed this cycle** — an in-progress run (started before this
  cycle's own edits, therefore non-authoritative for any file this cycle touched) was killed
  after ~38 minutes / 227 of ~600 binaries when a `CARGO_TARGET_DIR` collision with a sibling
  worktree's own process was discovered. Every file this cycle touched was instead verified
  individually via targeted `cargo test --test <name>` runs (above), each in a freshly
  isolated target dir, each green. The full untargeted sweep is `root-full`'s own remaining
  obligation — this cycle names it, does not claim it.

## Sweep population

N/A — this cycle touched no `data/corpus/**` file (all fixes are in `src/`, `tests/`,
`apps/desktop/src-tauri/src/`). `corpus_literal_sweep`'s examined population is unmoved.

## Oracle pin

Not load-bearing for any figure in this receipt. One read-only sanity check against the
pinned checkout (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`,
`~/workspace/repos/pcgen/data/.../apg_spells.lst`, 2,136 lines / 1,380
`KEY:`/`.MOD` rows) corroborated the APG spell-cache magnitude (945 files) as plausible; the
actual figure used and verified is the live corpus count, not the oracle.

## Status

**partial.** This cycle's whole assigned population (root-lib, root-full, desktop, reach,
clippy) did not all reach the bar. It closed:

- **root-lib: COMPLETE.** 0 failing, verified live.
- **root-full: 33 of ~47 known-failing suites individually fixed and verified green.**
  A full untargeted sweep to confirm no other suite in the ~600-binary workspace regressed
  was not completed this cycle (see Build scope above) — every fix was targeted-verified
  instead.
- **desktop / reach: 21 of 28 failing tests closed** (companion_catalog.rs's `PREHD` render
  fix, which alone cleared 15+ cascading companion_catalog/reach_gate failures, plus
  reach_gate.rs's two missing registry entries clearing
  `every_ingested_record_type_is_classified`). **565 passed / 7 failed**, verified live
  (`cargo test --locked` in `apps/desktop/src-tauri`, isolated
  `CARGO_TARGET_DIR=/tmp/cargo-sd34-desktop-verify`). The remaining 7 are named in the
  next-cycle-plan below — 6 of the 7 trace to the SAME 9 core_rulebook records (`Adopted
  Race ~ <Race>` / `Human Ethnicity ~ {None,Unknown}`) this cycle already diagnosed on the
  root side, cascading into the desktop's own picker/reach-gate surfaces; the 7th
  (`every_registered_ability_reaches_the_wire_under_an_owner`) is a large, unrelated,
  pre-existing companion-ability reach gap (hundreds of ids), not touched this cycle.
- **clippy: NOT ATTEMPTED this cycle.** Root at 86 warnings (ceiling 50, 36 over) and desktop
  at 25 (ceiling 7, 18 over) per the fable-review's baseline; nothing in this cycle's diff
  removes an existing warning (only doc-comment/test-assertion edits, no dead-code removal).

## Movement, four buckets

- **Closure:** 0 inventory-bucket units moved (this cycle never touched
  `docs/work-inventory.json` or `data/corpus/**`) — this cycle's movement is entirely
  **instrument-correction**: gate/test-suite assertions re-derived against already-landed,
  already-tested engine/corpus state, not new player-facing content.
- **Reclassification:** N/A.
- **Reachability:** N/A.
- **Instrument-correction:** 33 root-full test binaries + root-lib's 3 tests, all re-pinned
  or widened against live, independently re-derived truth (never against "whatever the suite
  currently prints" — every changed number/exclusion carries its own re-derive command
  above, and two were CODE fixes, not count changes: `companion_chassis.rs`'s
  `HELD_STATUSES` widening and `companion_catalog.rs`'s `PREHD` render).

## Notes (judgment calls)

- **CARGO_TARGET_DIR collision discovered mid-cycle**: another worktree (`wf_4a1d662c-fd1-2`)
  was observed running a process with `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001` set in
  its environment (identical to this cycle's own assigned value) — a real instance of
  `AGENTS.md`'s documented hazard ("CARGO_TARGET_DIR is one directory per agent *per source
  tree*"). Investigation showed that process's actual `rustc --out-dir` targeted its OWN
  worktree's `apps/desktop/src-tauri/target/`, not the shared root dir, so no contamination
  is confirmed — but out of caution, all verification from that point forward
  (`no-run-final2.log` onward) used a freshly isolated `CARGO_TARGET_DIR` per invocation.
  Root-lib's own 3,022-passed/0-failed result predates the collision by ~25 minutes and is
  unaffected either way.
- **`sd18_cleric_level11_widening.rs`'s `class_feature_pool_catalog.rs` / `companion_chassis.rs`
  fixes were CODE changes** (widening a status-match allowlist), never count re-pins — flagged
  explicitly per the brief's own warning against disguising a code fix as a count update.
- **Every stale-count fix traces to a named, dated, already-landed commit** (`49d72f5e03`,
  `cb0ba2286e`, `b4eadc9cbf`, `c5c4a1b788`, `935cef27b5`, `bfe90f020a`) — none is speculative;
  each was confirmed via `git log -S`/`git show` before the test was touched.

## Next-cycle plan (named remainder, by sub-cause, populations summing to what remains)

**root-full — 13 test functions across 8 files, all investigated and triaged, none fixed
(all require either genuine corpus regeneration this cycle is barred from hand-editing, or
deeper domain verification than this cycle's remaining budget allowed):**

1. `tests/sd27_ability_automatic_granted_race_traits.rs` (2 tests) — `RaceCorpus::
   unclassified_traits()` now returns 15 stranded records (the same 9 `Adopted Race`/
   `Human Ethnicity` records this cycle diagnosed elsewhere, PLUS 6 bare-race-name records
   — `Drow, Dwarf, Elf, Gnome, Grippli, Halfling, Orc` — not yet investigated). Needs either a
   new `TraitRole` gate for the CHOOSE-selector/placeholder shape in `race_resolver.rs`, or a
   scope correction to this test's own premise. Population: 15.
2. `tests/sd27_book_license_record_counts.rs` (2 tests) — 21 books' `records_processed` and
   19 books' `records_redacted` in `data/corpus/**/LICENSE.json` are stale against the live
   corpus. **PI-adjacent; touches `data/corpus/**`, barred from hand-edit this cycle** — needs
   the guarded LICENSE-regeneration path, not a hand patch. Population: 21 + 19 book rows
   (see `verify-batch`/review log for the full per-book breakdown already captured).
3. `tests/sd27_equipment_modifier_price_matches_corpus_cost_token.rs` (2 tests) — Pathfinder
   Unchained (PU) has 4 genuine duplicate corpus keys (`Special Ability ~ ABP +0 ~
   {Ammunition,Armor,Shield,Weapon}`) — a real corpus-generation defect, not a test staleness.
   Needs generator investigation. Population: 4 duplicate keys + the pricing-column count
   (not yet re-derived).
4. `tests/sd27_known_spells_must_be_on_the_class_spell_list.rs` (1 test) — the desktop Add
   Spell picker's compile-time `SPELL_LIST` union grew 2,113 → 2,127 (+14); which book's
   const array grew, and whether the `off_list` sub-count (1,369+45+57) also needs updating,
   not yet determined. Population: 1 assertion, denominator 2,127.
5. `tests/sd30_declared_product_identity_in_shipped_class_features.rs` (1 test) — a REAL
   Product Identity leak: `inner_sea_world_guide`'s "Codex-Named Unit" placeholder-titled
   class_feature records still ship a name PCGen declares PI (`decisions.md §50`). **Legally
   sensitive; prioritize next cycle.** Population: 5+ named records in the captured log (full
   list not yet re-derived at current HEAD).
6. `tests/sd31_class_feature_corpus_key_uniqueness.rs` (1 test) — a real generation collision:
   `adventurers_guide`'s `Enlightened Bloodrager ~ Bloodline Feat ~ AG` key is written by two
   different files (`bloodline_feat.json`, `bloodline_feat-2.json`), one silently overwriting
   the other. Needs generator investigation, not a test fix. Population: 1 named collision
   (full corpus-wide count not yet re-derived).
7. `tests/v06_corpus_trap_report.rs` (4 tests) — real ingest-trap `Defect`-severity findings
   (shared record_key collisions, disabled-line sourcing, missing `.MOD` base declarations,
   mismatched cited lines) across multiple books. Needs per-finding triage against the
   generator. Population: not yet re-derived at current HEAD (review log names the first hit
   per test only).

**desktop/reach — 7 of 28 originally-failing tests remain (565 passed / 7 failed, verified
live):**

8. `race_trait_picker::the_menu_command_carries_all_fourteen_adopted_race_options_thirteen_with_real_grants` —
   the 7 core_rulebook `Adopted Race ~ <Race>` selectors now reach the desktop menu too
   (live 21 keys, alphabetically interleaved with the existing 14); the function's own NAME
   encodes two counts ("fourteen"/"thirteen with real grants") that both need re-deriving,
   not just the list literal. Population: 21 (7 new + 14 existing).
9. `reach_gate::every_declared_claim_actually_carries_the_records` — the 2
   `Human Ethnicity ~ {None,Unknown}` placeholder rows never appear in
   `list_alternate_racial_traits`, tripping a declared-reach-claim check. Population: 2.
10. `reach_gate::unreached_records_are_exactly_the_recorded_findings` — likely the same 9-record
    cause surfacing in a second reach_gate finding-list assertion; not yet individually
    confirmed. Population: not yet re-derived.
11. `reach_gate::unsurfaced_families_are_exactly_the_recorded_findings` — same shape as #10, not
    yet individually confirmed. Population: not yet re-derived.
12. `companion_catalog::an_unmodelled_facet_reaches_the_wire_with_its_type_segments` — not yet
    investigated this cycle.
13. `companion_catalog::every_registered_ability_reaches_the_wire_under_an_owner` — a large,
    apparently pre-existing (not this cycle's cause) companion-ability reach gap: "an ability
    row reaches no creature on the wire" against a set of hundreds of
    `<book>:companion:<ability>` ids spanning many books. Needs its own dedicated
    investigation — clearly not part of the 9-record oracle-restamp/CRB-ingest cascade.
    Population: hundreds (exact count not yet re-derived).
14. `feat_catalog::feat_descriptions_are_rendered_and_otherwise_byte_identical` — not yet
    investigated this cycle.

Items 8-11 (population ~30, all traced to the same 9 already-diagnosed core_rulebook
records) are the fastest remaining win for the next cycle. Items 12-14 need fresh
investigation.

**clippy — untouched, root 36 warnings over ceiling (86 vs 50), desktop 18 over (25 vs 7).**
Needs per-warning triage (`cargo clippy --locked --tests` output, not yet captured at current
HEAD) — likely dominated by the same accumulated-drift shape as root-full (unused imports
already visible in this cycle's own build logs, e.g. `MonsterAbilityDelivery`/`NaturalAttack`/
`Speed`/`StatAdjustment` across 5+ `rules_tables/*/monster_data.rs` files).
