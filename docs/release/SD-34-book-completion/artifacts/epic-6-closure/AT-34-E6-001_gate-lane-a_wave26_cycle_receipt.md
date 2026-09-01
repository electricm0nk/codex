# Cycle wave-26-gate-lane-a — Epic 6 (gate remediation) / AT-34-E6-001 (Lane A)

**Filename note (same convention wave-23's and wave-24's own header).** This cycle's dispatch
brief reused the label `AT-34-E6-001` for a gate-remediation lane, distinct from `kanban.md` row
26's canonical `AT-34-E6-001` (`final-acceptance-scan`, still `not-started`). Writing to the
literal path the dispatch brief names (`AT-34-E6-001_cycle_receipt.md`) would silently overwrite
the genuine 2026-08-29 final-acceptance-scan FAIL-verdict receipt already on disk there. Filed
here instead, wave-tagged (this session rebased onto a tip whose last commit before mine was
`docs(sd34): wave 26 gate lane B -- fill in this cycle's own commit SHA in the receipt`, so this
cycle is numbered alongside it). `kanban.md` row 26 is left untouched — this cycle's own work
does not satisfy that criterion either.

- **Commit SHA:** `5d7c985bf2` (last figure-moving commit; also this receipt's own build-scope
  SHA — no commits landed on Rust source or corpus after it before this receipt was written)
- **Files touched:**
  - `src/bin/gen_book_cache.rs` (new `existing_equipment_record_path()` helper + write-site fix,
    + 3 unit tests)
  - `data/corpus/adventurers_guide/class_feature/enlightened_bloodrager/bloodline_feat-2.json`
    (deleted)
  - `data/corpus/core_rulebook/class_feature/draconic_bloodline/draconic_bloodline-2.json`
    (deleted)
  - `data/corpus/pathfinder_unchained/equipment/0_abp_enhancement_to_{ammunition,armor,shield,
    weapon}.json` (4 files, deleted)
  - `data/corpus/<book>/LICENSE.json` — 23 books (see Figures)
  - `docs/retro/events/sd34-at-34-e6-001.jsonl` (1 correction event, this cycle's own)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` on the code diff
  (`src/bin/gen_book_cache.rs` + the 8 deleted/modified corpus JSON files): all 3 hits are
  doc-comment citations of this repo's own permanent test filename
  (`sd27_equipment_modifier_price_matches_corpus_cost_token.rs`) — the same "repo's own
  test-naming convention" shape wave-24's receipt already established is not a defect. The
  23 LICENSE.json edits DO carry many `sd27_`/`SD3[0-9]-`-shaped hits, but every one is inside
  the file's own established `screening_method_note` audit-trail convention — 3 books already
  shipped `RECONCILED SD31-W4-INTEGRATE-001`/`PASS -- SD31-E5-F1-001`/`PASS -- SD29-E7-F2-009`
  tags in this exact field before this cycle touched anything, and a `unified=0` diff of a
  single-string JSON field necessarily re-quotes that entire prior history as part of showing
  the new value. This cycle's own added tag (`RECONCILED SD34-AT-34-E6-001, 2026-09-01: ...`)
  follows the identical, already-established convention. Command:
  `bash /tmp/claude-1000/-home-ubuntu-workspace-repos-codex/96c6fb81-ac0c-4826-af55-ff6066936971/scratchpad/dual_audit.sh`
  (re-derivable: `BASE_BRANCH=$(git merge-base HEAD origin/develop)` = `ea2b3396f2`, diffed
  against this cycle's own file list, both `base...HEAD` and the working-tree form).
- **Wired-integration audit result:** `OK_NO_TOKENS` — zero hits of
  `STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack` anywhere in either
  diff form.
- **Acceptance criterion (verbatim from this cycle's dispatch brief):** "GATE LANE A — the four
  data/corpus mechanisms that are the last of root-full... all four are fully diagnosed, none
  needs further investigation" — `sd27_book_license_record_counts.rs` (2 tests, guarded
  LICENSE-only regen), `sd27_equipment_modifier_price_matches_corpus_cost_token.rs` (2 tests,
  generator defect), `sd31_class_feature_corpus_key_uniqueness.rs` (1 test, stale-file
  deletion), `v06_corpus_trap_report.rs` (4 tests, route to the existing trap epic).

## What this cycle did, mechanism by mechanism

### 1. `sd27_book_license_record_counts.rs` — guarded LICENSE-only regen, 23 books

Wave-24's diagnosis reproduced exactly at HEAD (21 `records_processed` mismatches, 19
`records_redacted` mismatches, 17-book overlap = 23 books touched). Fixed by restating both
fields from the live corpus per the test's own directive ("restate the number... to match the
corpus, rather than adjusting this test") — **no `data/corpus/**` content record touched**, only
each book's `LICENSE.json`. Each book's `screening_method_note` gets a `RECONCILED
SD34-AT-34-E6-001, 2026-09-01: ...` prefix (matching the file's own 3-book precedent) that
quotes the new number and its `find`-command derivation, with the prior note preserved unedited
underneath so `the_screening_note_quotes_the_same_count_the_field_states` still passes.

**Cross-item interaction caught and fixed the same cycle.** Item 2 (below) deleted 4
`pathfinder_unchained` equipment records AFTER item 1's regen already ran, which silently
un-fixed PU's `records_processed` (1271 stated vs 1267 real once the 4 files were gone). Caught
by re-running the 4-target-file suite together after item 2 landed; the same guarded regen
script was re-run and only PU needed a second restatement (`records_redacted` unaffected, stayed
1). Logged as a `retro.py correction` event (`docs/retro/events/sd34-at-34-e6-001.jsonl`).

### 2. `sd27_equipment_modifier_price_matches_corpus_cost_token.rs` — generator defect fixed at the source

Root cause traced, not just symptom-patched. `b34bf2b4f0` ("git mv, kind-misclassified by
directory, 4 units") relocated `pathfinder_unchained`'s 4 `ABP +0` equipmods records
(`{Ammunition,Armor,Shield,Weapon}`) from the flat `equipment/<slug>.json` layout to the
categorized `equipment/equipmods/<slug>.json` layout `advanced_race_guide`'s own equipment block
already uses. `gen_book_cache.rs`'s PU equipmods write guard (`if !path.exists()`) checked ONLY
the flat path, so it could not see the relocated files — the very next regen in that same
2026-08-23 cycle silently re-wrote 4 flat duplicates beside the already-correctly-relocated
originals, each carrying the corpus's own duplicate key.

**Fix:** new `existing_equipment_record_path(out_root, category_slug, slug)` helper checks BOTH
the flat and category-nested layouts before writing; the write site now skips creating a flat
file when either already exists. TDD: 3 new unit tests (temp-dir based, matching this file's own
`write_json_never_overwrites_an_existing_file` precedent in `cache_gen::apg`) written RED first
(the function did not exist — confirmed failing with `E0425: cannot find function` for the
intended reason), then GREEN after the fix. Full `gen_book_cache` unit suite: 8 passed / 0
failed (no regression on the 5 pre-existing tests).

Corpus: deleted the 4 stale flat duplicates, each independently verified (same `data.key` AND
same `source` citation as its `equipmods/`-nested sibling) before deletion. This also resolved
the sibling price-classification assertion wave-24 had flagged as "plausibly the same 4,
unconfirmed" — confirmed exactly: `every_newly_reachable_record_is_priced_by_its_own_corpus_
cost_token` is green now too, with no further corpus edit.

### 3. `sd31_class_feature_corpus_key_uniqueness.rs` — 2 stale files, not 1

Wave-24 diagnosed and fully verified ONE stale leftover
(`adventurers_guide/.../bloodline_feat-2.json`, superseded at the same source line/sha256 by
`bloodline_feat.json` after `a08973ae35`'s class-field fix). Deleting it and re-running the test
surfaced a SECOND, previously-undiscovered duplicate the test's own `BTreeMap`-insert-panics-on-
first-collision design had been hiding: `core_rulebook`'s `draconic_bloodline/draconic_bloodline
-2.json` vs `draconic_bloodline.json` — the exact same defect shape (same source line 2976, same
sha256, `-2` file is the pre-`a08973ae35` copy with the stale `class: "Draconic Bloodline"`
value, the non-suffixed file is the post-fix copy with the correct `class: "Sorcerer"`). Verified
via full-record inspection of both pairs (not sampling) plus a standalone python scan of every
`(book, data.key)` pair under `data/corpus/**/class_feature/` confirming **zero** remaining
duplicates after both deletions — so this cycle does not leave a third hidden collision for a
future cycle to trip over.

### 4. `v06_corpus_trap_report.rs` — routed, not touched

Re-derived live at HEAD, reproducing wave-24's counts exactly: `no_two_ingested_records_share_a_
record_key` 249, `ingested_record_keys_match_their_cited_line` 650, `every_mod_sourced_ingest_
has_a_live_base_declaration` 2,117, `no_ingested_record_is_sourced_from_a_disabled_line` 165 (sum
3,181). Cross-checked against `decisions.md §13`, which pre-dates this cycle: these are the
EXACT SAME 4 numbers (`mod-record` 2,117, `key-differs-from-name` 650, `shared-name-distinct-
records` 249, `disabled-line` 165) that decision explicitly rules "SD-33's already-verified,
already-out-of-DoD inherited debt (`forward-scope-register.md D1.1`'s `v06_corpus_trap_report`
target)... stay registered, not absorbed: AT-34-E1-008's bar is `wiring-class-mismatch = 0`,
with the other four trap kinds reported at their unchanged counts." `kanban.md` rows 7/8
(AT-34-E1-007/AT-34-E1-008) are both already `complete` — correctly so, since neither was ever
scoped to these 4 trap kinds. **No code or corpus change made; no test weakened.** This item was
never this cycle's to close — it is named here, with its live population, so no future cycle
re-discovers it as new.

## Figures + their re-derive commands

| Figure | Old | New | Command / denominator |
|---|---:|---:|---|
| LICENSE `records_processed` mismatches | 21 books (wave-24) | 21 books, reproduced exactly, then 23 fixed (incl. the +1 cross-item PU regression) | `cargo test --locked --test sd27_book_license_record_counts -- --nocapture` before/after |
| LICENSE `records_redacted` mismatches | 19 books (wave-24) | 19 books, reproduced exactly, all fixed | same run |
| `sd27_book_license_record_counts` | 4 passed / 2 failed | 6 passed / 0 failed | `cargo test --locked --test sd27_book_license_record_counts` |
| PU duplicate equipmods keys | 4 (wave-24, re-confirmed) | 4 found and deleted, 0 remain | `cargo test --locked --test sd27_equipment_modifier_price_matches_corpus_cost_token -- --nocapture` before/after |
| PU price-classification tuple | (447,1,130) live vs (447,1,126) pinned | (447,1,130) == (447,1,130), test green | same run |
| `sd27_equipment_modifier_price_matches_corpus_cost_token` | 1 passed / 2 failed | 3 passed / 0 failed | `cargo test --locked --test sd27_equipment_modifier_price_matches_corpus_cost_token` |
| `gen_book_cache` unit tests | 5 passed (pre-existing) | 8 passed / 0 failed (+3 new) | `cargo test --locked --bin gen_book_cache` |
| class_feature duplicate `(book,key)` pairs | 1 known (wave-24), 1 more found this cycle | 0 remain, corpus-wide | standalone python full-corpus scan (this receipt's own script) + `cargo test --locked --test sd31_class_feature_corpus_key_uniqueness` |
| `sd31_class_feature_corpus_key_uniqueness` | 1 passed / 1 failed | 2 passed / 0 failed | `cargo test --locked --test sd31_class_feature_corpus_key_uniqueness` |
| `v06_corpus_trap_report` finding counts | 249/650/2117/165 (wave-24) | 249/650/2117/165, reproduced exactly, routed not fixed | `cargo test --locked --test v06_corpus_trap_report <test_name> -- --nocapture 2>&1 \| grep -c "Finding {"`, one command per failing test |
| `corpus_literal_sweep` examined population | 48,708 (measured this cycle, before item 3's 2 deletions) | 48,706 after items 3+2 (item 1/LICENSE-only never moves it — see Sweep population) | `cargo run --locked --bin corpus_literal_sweep -- --json-out <path> --quiet`, `records_examined` field |
| `cargo test --locked --lib` | 3,022 passed (wave-24) | 3,019 passed / 0 failed / 14 ignored (delta from other lanes' commits between wave-24 and this cycle, not this cycle's own `src/bin/*` change — `--lib` excludes binary-crate tests) | `cargo test --locked --lib` |

## Row-count command output

```
$ cargo test --locked --test sd27_book_license_record_counts --test sd27_equipment_modifier_price_matches_corpus_cost_token --test sd31_class_feature_corpus_key_uniqueness --no-fail-fast
```
- `sd27_book_license_record_counts`: `test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `sd27_equipment_modifier_price_matches_corpus_cost_token`: `test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `sd31_class_feature_corpus_key_uniqueness`: `test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`

Combined: **11 passed / 0 failed** across this cycle's three closed mechanisms.

`v06_corpus_trap_report` deliberately left red (4 failed / 21 passed, routed to
AT-34-E1-007/AT-34-E1-008, see item 4 above) — included in the same combined run to prove it was
not silently weakened:
```
$ cargo test --locked --test sd27_book_license_record_counts --test sd27_equipment_modifier_price_matches_corpus_cost_token --test sd31_class_feature_corpus_key_uniqueness --test v06_corpus_trap_report --no-fail-fast
```
exits 101 (non-zero) — v06_corpus_trap_report: `21 passed; 4 failed` (identical population to
wave-24 and to this cycle's own live re-derivation above; unchanged, deliberately).

## Build scope verified

- `cargo test --locked --no-run` (whole workspace, isolated `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001`, run AFTER this cycle's last commit `5d7c985bf2`): **exit 0.**
- `cargo test --locked --lib`: **3,019 passed / 0 failed / 14 ignored** — 0 failures; the -3 vs
  wave-24's 3,022 is other lanes' commits between wave-24 and this cycle (`--lib` never runs
  `src/bin/*` unit tests, so this cycle's own 3 new `gen_book_cache` tests are not in this
  count), not a regression this cycle caused.
- `apps/desktop/src-tauri` (separate cargo workspace): **not touched, not run** this cycle —
  every file this cycle changed is under `src/bin/gen_book_cache.rs`, `data/corpus/**`, or
  `docs/retro/`, none desktop-adjacent.
- Full untargeted `cargo test --locked --no-fail-fast` over the ~600-suite root workspace was
  **not run** this cycle — the memory/wall-time hazard this bundle's brief warns against for a
  population-scoped run of this size, same gap wave-23/24's own receipts name. Every file this
  cycle touched was verified individually instead (all green except the deliberately-unfixed
  `v06_corpus_trap_report`, see Row-count above), and this receipt names the trap-report
  remainder by exact mechanism and population so no future cycle re-discovers it.

## Sweep population

`corpus_literal_sweep --json-out ... --quiet`, `records_examined` field:

- **48,708** — measured this cycle with items 3's 2 deleted `class_feature` files temporarily
  restored (`git show 45c25e1bc8~1:<path>` into place, not staged/committed), i.e. the
  population immediately BEFORE this cycle's own corpus deletions.
- **48,706** — measured after item 3's 2 deletions (delta **exactly -2**, matching the 2 records
  removed).
- **48,706** — measured again after item 2's 4 equipment-file deletions (delta **0** — these 4
  records carry `wiring_class: "computed"` / `computed:pre_guard` with a null `cost_gp` and no
  literal `COST:` token to byte-compare, so they were never in the sweep's counted population to
  begin with; confirmed empirically, not assumed).
- Item 1 (LICENSE.json only) never moves this count: `LICENSE.json` is explicitly excluded from
  `corpus_literal_sweep`'s walk (`src/bin/corpus_literal_sweep.rs` lines 630, 1023).
- **Final, at this cycle's last commit `5d7c985bf2`: 48,706, `clean: true`.**

## Oracle pin

`$PCGEN_CORPUS_ROOT` resolved via the standing `~/workspace/repos/pcgen` checkout (unpinned
literal path used only interactively for this receipt's own sweep runs, never written into
source/docs) — load-bearing for the sweep-population figures above, not for any other figure in
this receipt.

## Status

**partial.**

- **Item 1 (LICENSE.json, 23 books): CLOSED.** `sd27_book_license_record_counts.rs`: 6/6 green,
  including the cross-item PU regression caught and fixed the same cycle.
- **Item 2 (equipment generator defect): CLOSED.** `sd27_equipment_modifier_price_matches_
  corpus_cost_token.rs`: 3/3 green. Generator fixed at the root cause (not just the corpus),
  with a regression-proof unit test.
- **Item 3 (class_feature dup keys): CLOSED**, and widened beyond the brief's single named file
  — a full-corpus scan found and closed a second, previously-undiscovered duplicate of the same
  shape. `sd31_class_feature_corpus_key_uniqueness.rs`: 2/2 green.
- **Item 4 (trap report, 3,181 findings): ROUTED, not fixed** — confirmed out of this criterion's
  scope by `decisions.md §13`, an existing ruling this cycle did not need to make. Left exactly
  as diagnosed, deliberately red, per the brief's own instruction not to weaken it or silently
  absorb it.

Of this cycle's four named mechanisms, **3 of 4 are fully closed this cycle**: item 1 (2 tests),
item 2 (2 tests), item 3 (1 test) — **5 tests fixed this cycle**, on top of wave-24's own 3
(`sd27_known_spells...` 1, `sd27_ability_automatic...` 2), for **8 tests green** across the whole
gate-lane-a effort to date. The remaining 4 (`v06_corpus_trap_report`) were never this
criterion's to close and are correctly routed to `AT-34-E1-007`/`AT-34-E1-008` (already
`complete` in `kanban.md`, correctly so per `decisions.md §13`).

## Movement, four buckets

- **Closure:** 0 inventory-bucket units moved (no `docs/work-inventory.json` touch — this is a
  gate-remediation lane, not a content-completion cycle).
- **Reclassification:** 0.
- **Reachability:** N/A.
- **Instrument-correction:** 3 mechanisms, all re-pinned against live, independently re-derived
  truth: (a) 24 `LICENSE.json` restatements (23 this-cycle + the cross-item PU correction) with
  every number carrying its own `find`-command derivation; (b) a real code-shape fix in
  `gen_book_cache.rs` (`existing_equipment_record_path`), not a count change, that prevents the
  duplicate-key defect from recurring on a future regen; (c) 2 stale pre-`a08973ae35` corpus
  files deleted after independent per-record verification, closing the whole `(book, key)`
  duplicate population to exactly 0 rather than the 1 the brief named.

## Notes (judgment calls)

- **This cycle's territory was the full `data/corpus/**` grant the wave-24 diagnosis lacked** —
  used it for a guarded, targeted regen (LICENSE.json fields + 6 stale/duplicate record
  deletions), never a full corpus regen, per the brief's explicit warning that a full regen has
  previously destroyed license metadata and `raw_tokens`.
- **The equipment generator fix was scoped to the root cause, not just the 4 duplicate files.**
  Deleting the 4 duplicates alone would have left the same defect ready to recur on the next
  `gen_book_cache` regen of `pathfinder_unchained`. The `existing_equipment_record_path` fix is
  the minimal change that makes the write guard category-aware without restructuring the other
  38 flat PU equipment records that have no nested counterpart.
- **The class_feature item was widened from the brief's named single file after a full-corpus
  scan, not assumed closed at 1.** The test's own `BTreeMap`-insert-panic design only ever
  surfaces its FIRST collision per run, so wave-24's single-file diagnosis was genuinely correct
  but genuinely incomplete — a second cargo-test-and-fix-one-at-a-time cycle would have found the
  second duplicate eventually, but the standalone full scan found and closed it in one pass and
  proved zero remain, rather than leaving that discovery to chance ordering on a future run.
- **The cross-item LICENSE.json regression (PU, item 1 vs item 2) was caught by re-running the
  full 4-target-file suite after every commit, not assumed stable from an earlier green run.**
  This is exactly the shape `AGENTS.md`'s "no fake completion" rule and this bundle's
  `decisions.md §12` L2 ("never carry your own number forward") exist to catch; logged as a
  `retro.py correction` event rather than silently amended.
- **Item 4 was NOT re-scoped into this cycle's DoD.** `decisions.md §13` already rules on this
  population by name and by exact figure; this receipt confirms the live numbers still match
  that ruling rather than re-litigating it.

## Next-cycle plan

**root-full, this lane's four named mechanisms: 0 remain requiring further `data/corpus/**`
work.** Items 1-3 are closed. Item 4 (`v06_corpus_trap_report`, 3,181 findings) is
`AT-34-E1-007`/`AT-34-E1-008`'s population by an existing ruling, not a remainder of this
criterion — no future gate-lane-a cycle should re-diagnose it.

**Whatever cycle runs `AT-34-E6-001`'s canonical final-acceptance scan next** should re-verify:
`sd27_book_license_record_counts`, `sd27_equipment_modifier_price_matches_corpus_cost_token`,
and `sd31_class_feature_corpus_key_uniqueness` all green at HEAD (this receipt's own commit
`5d7c985bf2` or later), and that `v06_corpus_trap_report`'s 4 known-red tests are still
attributed to `AT-34-E1-007`/`AT-34-E1-008` (both `complete`) rather than counted as new
`root-full` reds against this criterion.

**clippy — untouched this cycle (Lane C's territory, already closed per wave-25's receipt).**

**desktop / reach — untouched this cycle (Lane B's territory, wave-26's own concurrent cycle).**
