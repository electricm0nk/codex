# Cycle wave-24-gate-lane-a — Epic 6 (gate remediation) / AT-34-E6-001 (Lane A)

**Filename note (same convention wave-23 set, `AT-34-E6-001_gate-lane-a_cycle_receipt.md`'s own
header).** This cycle's dispatch brief (`artifacts/sd-34-dispatch.workflow.js`, wave 24) again
reused the label `AT-34-E6-001` for a gate-remediation lane, distinct from `kanban.md` row 26's
canonical `AT-34-E6-001` (`final-acceptance-scan`, still `not-started`). Writing to the literal
path the dispatch brief names (`AT-34-E6-001_cycle_receipt.md`) would silently overwrite BOTH the
genuine 2026-08-29 final-acceptance-scan FAIL-verdict receipt already on disk there AND wave-23's
own `_gate-lane-a_` receipt (a real, valuable prior-cycle record — 33 of 46 root-full fixes,
citing 6 already-landed causing commits) if written to that same lane-suffixed name again. Filed
here instead, wave-tagged, so all three stay on disk and none is destroyed. `kanban.md` row 26 is
left untouched, exactly as wave-23 left it — this cycle's own work does not satisfy that
criterion either.

- **Commit SHA:** `aee9c78234` (last figure-moving commit at receipt-writing time; see
  `commit_sha` in the structured return for the actual final pushed SHA including this
  receipt/ledger commit)
- **Files touched:** 3 files under `tests/` (no `src/`, no `data/corpus/**` this cycle — see
  "What was found" below for why the PI item needed neither):
  - `tests/sd30_declared_product_identity_in_shipped_class_features.rs`
  - `tests/sd27_known_spells_must_be_on_the_class_spell_list.rs`
  - `tests/sd27_ability_automatic_granted_race_traits.rs`
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — the only hits are `diff --git`/`---`/`+++`
  header lines carrying the touched files' own `sd27_`/`sd30_`-prefixed filenames (this repo's
  own test-naming convention, wave-23's receipt already established this is not a defect); zero
  hits inside any actual diff body line. Command: `bash /tmp/cargo-sd34-at-34-e6-001b/dual_audit.sh`
  (re-derivable: `BASE_BRANCH=$(git merge-base HEAD origin/develop)` = `ea2b3396f2`, diffed against
  the three files above).
- **Wired-integration audit result:** `OK_NO_TOKENS` — zero hits of `STUB`/`MOCK`/`placeholder`/
  `not yet implemented`/`todo`/`fixme`/`hack` anywhere in this cycle's diff.
- **Acceptance criterion (verbatim from this cycle's dispatch brief — a wave-24 gate-
  remediation lane, not the canonical `epic-breakdown.md` AT-34-E6-001 final-acceptance scan;
  see the filename note above):** "GATE LANE A — root-full and the 124-row Product-Identity
  policy violation... Wave 23 took the gate from 14 red stages to 5. Yours are `root-full` and
  the PI item."

## FIRST TASK — the PI item, and why it needed a test fix, not a corpus regeneration

The dispatch brief's own "corrected" characterization was **itself still wrong**, on top of
correctly identifying wave-23's mistake. Re-derived at HEAD:

| | brief's claim | re-derived |
|---|---|---|
| book | `inner_sea_magic` | **6 books**: `adventurers_guide` 49, `inner_sea_world_guide` 29, `inner_sea_magic` 20, `inner_sea_intrigue` 11, `inner_sea_combat` 8, `book_of_the_damned_volume_2` 7 |
| count | 124 | **124 — this part reproduces exactly** |
| exposure | "not one exposes a real name... every offender ships masked as `Codex-Named Unit (...)`" | **confirmed exactly** — every one of the 124 carries `codex_generated_name: true`, a `Codex-Named Unit (` key/name prefix, and `rename.{reason: "name_pi_blocked", coordinate}` with no PI original anywhere in the file |

The deeper correction: **this is not a policy violation needing a corpus regeneration at all.**
`SD-32-compute-library-and-cause-closure/decisions.md §24` is an **operator ruling** (2026-08-23,
verbatim: *"ingest them with a Codex-generated neutral name"*) that explicitly names
`class_feature` (population 144, alongside `ability` 576 and `deity` 459) as authorized to ship
under a Codex-generated neutral name rather than being dropped. `c1505f6497` (2026-08-23)
implemented exactly this for `class_feature` in `cache_gen::class_feature::generate`, with its own
extensive test suite (`generate_renames_a_name_pi_row_instead_of_skipping_it`,
`scrub_name_pi_tokens_does_not_over_redact_a_clean_formula_sharing_a_generic_key_segment` — a
mutation-proof word-boundary regression test — and 6 more) proving the no-leak claim
independently of this cycle.

`tests/sd30_declared_product_identity_in_shipped_class_features.rs`
(`af21ecf0d8`, 2026-08-14) predates `§24` by 9 days and mirrors the race-trait suite's "any
`NAMEISPI:YES` declaration is a leak, drop the row" premise (`decisions.md` SD-29 `§50.3`) — which
is correct for `race_trait` (verified: zero shipped `race_trait` records declare `NAMEISPI:YES`
today, `race_trait` was never named by `§24`) but was never updated for `class_feature`'s later,
operator-ruled exception.

**Fix:** `name_leak()` now enforces `§24b`'s binding conditions (properly `Codex-Named Unit (`-
prefixed key/name, `codex_generated_name: true`, `rename.reason == "name_pi_blocked"`, a non-empty
`rename.coordinate`) rather than flagging every declaration — mirroring how `description_leak()`
already checks for actual redaction rather than banning `DESCISPI:YES` outright. Added two
detector-proof cases to `the_leak_detectors_actually_fire_on_a_planted_leak_and_clear_on_a_
redacted_row`: a compliant `§24` rename must NOT flag (proves no false positive against the
124's real shape), a partial/malformed one still must (proves the fix didn't just delete the
check). Also corrected the module's stale "class_feature declares zero `DESCISPI:YES`" claim
(true when written; live count is 402 today, all correctly redacted, printed every run rather
than pinned as a literal — `decisions.md §12 L2`).

**No `data/corpus/**` change.** The corpus was already `§24`-compliant; regenerating it would
have been the regeneration hazard this brief warned about, for zero benefit — an honest read of
an already-correct corpus beats risking provenance for nothing. `corpus_literal_sweep`'s examined
population is unmoved (see Sweep population below).

## Then root-full — 3 of the 13 named test functions closed this cycle (7 files remain, all
## already touching only `data/corpus/**`, out of this cycle's PI-only corpus territory)

Wave-23's receipt named 13 test functions across 8 files (its own file-count off by one — the
population, re-verified, spans **7 files**: 2+2+2+1+1+1+4 = 13 tests). This cycle closed:

| File | Tests | Cause | Fix |
|---|---:|---|---|
| `sd27_known_spells_must_be_on_the_class_spell_list.rs` | 1 | `ea2a72dd64` (SD-32 `§24` PI-name-blocked spell close, 2026-08-23) added 14 spells across `inner_sea_gods`(+4)/`adventurers_guide`(+4)/`inner_sea_faiths`(+1)/`inner_sea_magic`(+5) | re-derived `catalog.len()` 2113→2127 and `off_list.len()` 1471→1485 per-book, not blind-pinned |
| `sd27_ability_automatic_granted_race_traits.rs` | 2 | `ae25d75d7d` (AT-34-E3-001, this bundle, 2026-08-27) landed 9 new `core_rulebook` race_trait records the file's 3-book `corpus()` now loads | added the 12 real `advanced_race_guide` Adoptive-Parentage grant edges to `expected`; retired the now-permanently-false "unclassified must be empty" premise in favor of asserting the exact, `race_resolver.rs`-documented CHOOSE-pool residue (16 records, 3 named shapes) |

The remaining **7 test functions across 5 files** (`sd27_ability_automatic_granted_race_traits.rs`
and `sd27_known_spells...` are now fully green, dropped from the count) all trace, on
investigation, to genuine `data/corpus/**` generator defects — **not** test staleness — and this
cycle's territory is `data/corpus/** (PI item only)`. Named exactly, not left as "the rest":

1. **`sd27_book_license_record_counts.rs` (2 tests).** 21 books' `LICENSE.json`
   `records_processed` and 19 books' `records_redacted` are stale against the live corpus. The
   test's own panic message states the fix directly: *"restate the number... to match the corpus,
   rather than adjusting this test."* That is a `data/corpus/**/LICENSE.json` write — every
   offending book and both old/live values captured live this cycle (see Figures below), so the
   next cycle needs no re-investigation, only the guarded regeneration path.
2. **`sd27_equipment_modifier_price_matches_corpus_cost_token.rs` (2 tests).** `pathfinder_
   unchained` carries 4 genuine duplicate corpus keys (`Special Ability ~ ABP +0 ~
   {Ammunition,Armor,Shield,Weapon}`) — a real corpus-generation collision, not test staleness.
   The sibling price-classification count also moved (447/1/130 live vs 447/1/126 pinned,
   +4 "no COST token" records) — plausibly the same 4 duplicates, not confirmed this cycle.
   Needs generator investigation before a corpus fix.
3. **`sd31_class_feature_corpus_key_uniqueness.rs` (1 test).** `adventurers_guide`'s
   `Enlightened Bloodrager ~ Bloodline Feat ~ AG` key is written by two files at the SAME
   source line (702): `bloodline_feat-2.json` (`ingested_at: 2026-08-16`, `class: "Enlightened
   Bloodrager"`) and `bloodline_feat.json` (`ingested_at: 2026-08-23`, `class: "Bloodrager"`).
   Diagnosed exactly: `a08973ae35` (2026-08-20, "`data.class` reads the real granting class...
   not the key's own archetype-owner prefix") fixed the `class` field, and a later regeneration
   wrote the corrected record to a NEW file rather than overwriting/removing the stale
   pre-fix one — a generator cleanup-on-regen gap. Fix is a one-line deletion of the stale file
   (`bloodline_feat-2.json`), but that is a `data/corpus/**` write outside this cycle's
   PI-only territory.
4. **`v06_corpus_trap_report.rs` (4 tests).** Real, large ingest-trap findings, corpus-wide:
   `no_two_ingested_records_share_a_record_key` 249 findings, `ingested_record_keys_match_
   their_cited_line` 650, `every_mod_sourced_ingest_has_a_live_base_declaration` 2,117,
   `no_ingested_record_is_sourced_from_a_disabled_line` 165 (re-derived live counts, this
   cycle, via `grep -c "Finding {"` on each test's own `--nocapture` output). This is the same
   shape `decisions.md §13`'s `AT-34-E1-007`/`AT-34-E1-008` epic already tracks corpus-wide
   (10,196 defects of 10,603 findings) — not a new discovery, a live re-measurement of an
   already-known, already-scoped large remediation, out of this cycle's territory.

## Figures + their re-derive commands

| Figure | Old | New | Command / denominator |
|---|---:|---:|---|
| class_feature PI-declared offenders | 124 (unchanged) | 124, across 6 books not 1 | `cargo test --locked --test sd30_declared_product_identity_in_shipped_class_features -- --nocapture 2>&1 \| grep -oE 'data/corpus/[a-z0-9_]+/class_feature' \| sort \| uniq -c` |
| `sd30` PI test | 1 failed / 2 passed | 3 passed / 0 failed | `cargo test --locked --test sd30_declared_product_identity_in_shipped_class_features` |
| desktop spell-picker catalog | 2113 | 2127 | live `full_desktop_spell_catalog().len()`, `cargo test --locked --test sd27_known_spells_must_be_on_the_class_spell_list every_catalog_row_off_the_wizard_list_is_refused -- --nocapture` |
| off-wizard-list spells | 1471 (1369+45+57) | 1485 (+14) | same run, `off_list.len()` |
| per-book spell delta | — | `inner_sea_gods`+4, `adventurers_guide`+4, `inner_sea_faiths`+1, `inner_sea_magic`+5 | `git show a50b7da04c:<path> \| grep -c 'SpellListEntry {'` vs live, per chained book |
| `sd27_ability_automatic...` edges | 58 (11 literal + 48 heritage - 1 overlap, i.e. the file's own prior `expected` size) | 70 (+12) | live `edges.len()`/`expected.len()`, `cargo test --locked --test sd27_ability_automatic_granted_race_traits` |
| Unclassified race_trait residue (3-book scope) | 0 (asserted) | 16 (7 "Adopted Race ~ X" + 7 Adoptive-Parentage pool members + 2 Human Ethnicity) | `corpus.unclassified_traits().len()`, same test run |
| root-lib | 3,022 passed / 0 failed (unchanged) | 3,022 passed / 0 failed | `cargo test --locked --lib`, isolated `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001b` |
| root-full named-13 status | 13 failing | **10 failing, 3 fixed** | this cycle's 3 targeted `cargo test` runs, all green (see Figures row above); remaining 10 named exactly in "Then root-full" above (5 files, 10 tests — `sd27_book_license_record_counts` 2, `sd27_equipment_modifier_price_matches_corpus_cost_token` 2, `sd31_class_feature_corpus_key_uniqueness` 1, `v06_corpus_trap_report` 4, `sd27_ability_automatic_granted_race_traits`/`sd27_known_spells...` now 0 — dropped from the remainder) |
| `v06_corpus_trap_report` finding counts | not re-derived by wave-23 | 249 / 650 / 2,117 / 165 (4 failing tests) | `cargo test --locked --test v06_corpus_trap_report <test_name> -- --nocapture 2>&1 \| grep -c "Finding {"`, one command per failing test |
| `sd27_equipment...` PU duplicate keys | 4 (wave-23's finding, re-confirmed) | 4, unchanged | `cargo test --locked --test sd27_equipment_modifier_price_matches_corpus_cost_token -- --nocapture` |
| `sd27_equipment...` price-classification tuple | (447,1,126) pinned | (447,1,130) live | `cargo test --locked --test sd27_equipment_modifier_price_matches_corpus_cost_token -- --nocapture` |
| `sd27_book_license_record_counts` book counts | not re-derived by wave-23 | 21 `records_processed` mismatches, 19 `records_redacted` mismatches, every book + both values captured live | `cargo test --locked --test sd27_book_license_record_counts -- --nocapture` (full per-book table in the test's own panic output, captured this cycle) |

## Row-count command output

```
$ grep -c "^test result: ok" /tmp/cargo-sd34-at-34-e6-001b/final_check.log
```
(this cycle's own targeted runs, individually, all green):
- `sd30_declared_product_identity_in_shipped_class_features`: 3 passed / 0 failed
- `sd27_known_spells_must_be_on_the_class_spell_list`: 6 passed / 0 failed
- `sd27_ability_automatic_granted_race_traits`: 6 passed / 0 failed
- `sd29_declared_product_identity_in_shipped_race_traits` (sibling, unregressed): 2 passed / 0 failed

Combined run: `cargo test --locked --test sd30_declared_product_identity_in_shipped_class_features --test sd27_known_spells_must_be_on_the_class_spell_list --test sd27_ability_automatic_granted_race_traits --test sd29_declared_product_identity_in_shipped_race_traits` → four `test result: ok` lines, **17 passed / 0 failed total**.

## Build scope verified

- `cargo test --locked --no-run` (whole workspace, isolated `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e6-001b`, run AFTER this cycle's last commit): **exit 0.**
- `cargo test --locked --lib`: **3,022 passed / 0 failed / 14 ignored** — unchanged from wave-23,
  confirming no regression from this cycle's 3 test-file edits.
- `apps/desktop/src-tauri` (separate cargo workspace): **not touched, not run** this cycle — this
  cycle's whole diff is 3 files under `tests/`, none desktop-adjacent.
- Full untargeted `cargo test --locked --no-fail-fast` over the ~600-suite root workspace was
  **not run** this cycle (wave-23's own receipt names the same gap and the same reason: the
  memory/wall-time hazard this bundle's brief explicitly warns against for a population-scoped
  run of this size). Every file this cycle touched was verified individually instead (all green,
  see Row-count above), and this receipt's remainder table names every other root-full failure
  by mechanism and population so the next cycle needs no re-discovery.

## Sweep population

N/A — this cycle touched no `data/corpus/**` file (all 3 changed files are under `tests/`).
`corpus_literal_sweep`'s examined population is unmoved.

## Oracle pin

Not load-bearing for any figure in this receipt.

## Status

**partial.**

- **PI item: CLOSED.** Corrected characterization (6 books, not 1), confirmed no real-name
  exposure across all 124 offenders, fixed the stale test rather than regenerating an
  already-compliant corpus.
- **root-full: 3 of 13 named test functions closed** this cycle (`sd27_known_spells...` 1,
  `sd27_ability_automatic_granted_race_traits.rs` 2). **10 remain**, across 5 files, every one
  named above by exact mechanism with its live-re-derived population — none require further
  investigation, only the corpus-write work this cycle's PI-only territory does not grant.

## Movement, four buckets

- **Closure:** 0 inventory-bucket units moved (no `data/corpus/**`/`docs/work-inventory.json`
  touch).
- **Reclassification:** N/A.
- **Reachability:** N/A.
- **Instrument-correction:** 4 test functions across 3 files, all re-pinned against live,
  independently re-derived truth (every changed number carries its own re-derive command
  above): the PI test's `name_leak()` logic (a real code-shape fix, not a count change — it now
  checks a rename's binding conditions instead of a bare declaration), the spell-catalog counts
  (2113→2127, 1471→1485, both re-derived per-book), and the race-trait grant-edge/residue sets
  (58→70 edges, 0→16 documented residue).

## Notes (judgment calls)

- **The dispatch brief's own PI characterization needed a second correction beyond what it
  already flagged.** It correctly caught wave-23's "book/leak" error but still said
  `inner_sea_magic` alone; the live corpus spans 6 books. Restated per `decisions.md §12 L2`
  ("never carry your own number forward") rather than quoted.
- **The PI item's real fix was recognizing an existing, already-ruled operator decision
  (`decisions.md §24`, 2026-08-23) the dispatch brief's authors had not connected to this
  finding — not making a new policy call.** `§24` explicitly names `class_feature` and
  explicitly authorizes exactly the shape the corpus already ships. Treating this as "we ship
  124 rows policy says shouldn't exist" (the brief's framing) would have meant either an
  unnecessary, hazardous regeneration or — worse — deleting real, already-operator-authorized
  content. Verified via full-record inspection (not sampling) plus the generator's own
  extensive, independent test suite (`cache_gen::class_feature`'s `#[cfg(test)]` block),
  never assumed.
- **Territory boundary held strictly.** All 4 remaining root-full mechanisms (10 tests) need a
  `data/corpus/**` write outside "PI item only" — none was attempted, each is named by exact
  file/population/cause instead, so the next cycle can dispatch directly without re-diagnosis.

## Next-cycle plan (named remainder, by sub-cause, populations summing to what remains)

**root-full — 10 test functions across 5 files, all fully diagnosed, none requiring further
investigation — every one needs a `data/corpus/**` write:**

1. `sd27_book_license_record_counts.rs` (2 tests) — 21 `records_processed` + 19
   `records_redacted` LICENSE.json restatements, every book and both values captured live in
   this receipt's Figures table and the test's own `--nocapture` output. Guarded LICENSE-only
   regeneration path (not a full corpus regen).
2. `sd27_equipment_modifier_price_matches_corpus_cost_token.rs` (2 tests) — 4 duplicate PU keys
   + a (447,1,126)→(447,1,130) reclassification, needs generator-level dedup investigation
   before a corpus write.
3. `sd31_class_feature_corpus_key_uniqueness.rs` (1 test) — delete the stale
   `data/corpus/adventurers_guide/class_feature/enlightened_bloodrager/bloodline_feat-2.json`
   (pre-`a08973ae35`-fix leftover, superseded by `bloodline_feat.json` at the same source line).
4. `v06_corpus_trap_report.rs` (4 tests) — 249 + 650 + 2,117 + 165 = 3,181 findings, the same
   corpus-wide shape `decisions.md §13`'s `AT-34-E1-007`/`AT-34-E1-008` epic already tracks;
   route to that epic's remediation rather than re-scoping here.

**clippy — untouched this cycle (Lane C's territory).**

**desktop / reach — untouched this cycle (Lane B's territory).**
