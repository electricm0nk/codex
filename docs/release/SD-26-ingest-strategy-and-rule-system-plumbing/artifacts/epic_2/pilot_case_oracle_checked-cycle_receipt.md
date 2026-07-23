# Cycle epic2-2.5-pilot_case_verification — Epic 2 Oracle-Harness Comparator / Criterion 2.5

- **Card ID:** t_b7bb32bf (receipt only, minted post-hoc as a done-receipt on board `codex-tranche-5`, assignee `operator`, completed — not a live claim)
- **Commit SHA:** 0a8ba631a2b7a549aa017c341f18641bce5332e0
- **Files touched:**
  - `tests/sd26_pilot_case_verification.rs` (new)
  - `artifacts/oracle_validation/parity_report_pf1-crb-human-fighter-level1.md` (new — real generated parity report)
  - `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/artifacts/epic_2/pilot_case_oracle_checked-cycle_receipt.md` (this file)
  - `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/progress.md`
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (diff-scoped dual-audit gate, `BASE_BRANCH...HEAD`)
- **Wired-integration audit result:** `OK_NO_TOKENS` (diff-scoped dual-audit gate, `BASE_BRANCH...HEAD`)
- **Acceptance criterion:** "Criterion 2.5 — Verification cycle for the pilot case. Runs the comparator against the pilot case; upgrades `current_claim_status` from `not_yet_grounded` to `oracle_checked`. Concurrency: serial." (`epic-breakdown.md` Epic 2). Verification row: "Pilot case fixture's `current_claim_status=oracle_checked`" (`acceptance-and-verification.md` row 2.5) — **explicitly conditioned by this cycle's own task brief on the comparator run genuinely confirming parity; not to be forced.**
- **Status:** complete (verification cycle executed for real; upgrade correctly withheld — see Notes)
- **Notes:**
  - **Pre-check confirmed all four Epic 2 dependencies are on `tranche/5-4`**: `comparator.rs`, `normalization.rs`, `parity_report.rs`, `pcgen_runner.rs` all present before this cycle started, per commits `744cd71`/`a87bc0d`/`7566d06`/`20ab8c9`.
  - **Read prior Epic 2 receipts first**, per the cycle brief. `pcgen_runner_rust-cycle_receipt.md` (2.4) and `pcgen_runner_smoke.rs`'s (SD-25) own doc comments already established a load-bearing fact for this cycle: **no real PCGen-native `.pcg` character file exists anywhere in either this repo or the checked-out PCGen repo for the exact pilot build** (PF1 Core Rulebook Human Fighter level 1, `pf1-crb-human-fighter-level1`). The only real, non-synthetic PCGen engine invocation available is against PCGen's own bundled `code/testsuite/PCGfiles/pf_Paladin.pcg` fixture — a materially different character build (different race, class, level).
  - **Ran the full pipeline for real** in `tests/sd26_pilot_case_verification.rs`: (1) loaded the real deterministic input fixture via `rules_core::character_input::load_character_input_fixture`, built the real `PilotHeadlessReceipt` via `rules_core::pilot_compute::build_pilot_headless_receipt`, and projected it to real `SelectedParityDimensions` via `oracle_validation::selected_parity_dimensions::SelectedParityDimensions::from_receipt` — genuine Codex-computed values, not synthetic; (2) ran the real PCGen engine end to end via `oracle_validation::pcgen_runner::run_pcgen_character` against the substitute `pf_Paladin.pcg` (real Gradle invocation, ~35s wall clock, no mocking); (3) compared the two via the real `oracle_validation::comparator::compare`; (4) rendered and wrote a real parity report via `oracle_validation::parity_report::write_parity_report` to the real default output path (`artifacts/oracle_validation/parity_report_pf1-crb-human-fighter-level1.md`, committed as a durable artifact — not gitignored, and named by `scope-draft.md §1.2`).
  - **Genuine finding, not a fabricated pass: the comparison shows 0/9 dimensions matching.** Every dimension mismatches — including `character.identity` itself (PCGen: "Florian Syrkov" [the bundled Paladin fixture's name], Codex: "pf1-crb-human-fighter-level1"). This is concrete, structural proof that the run is not a genuine same-character parity check: the substitute PCGen build and the pilot's own Codex-computed build are, definitionally, different characters, so their numeric disagreement carries no parity signal one way or the other. A coincidental match would have been a false-positive parity signal; this cycle does not treat the real mismatches as a meaningful "fail" of the pilot case's own correctness either — both would be equally dishonest reads of this data.
  - **Per this cycle's own explicit instruction ("upgrade ... but ONLY if the comparator run actually confirms parity; if it finds a real discrepancy, do NOT force the upgrade, report the discrepancy as a real finding instead"), the golden fixture's `current_claim_status` is NOT changed.** `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt` is untouched by this cycle; the test asserts both the loaded, typed value (`ClaimTier::NotYetGrounded`) and the raw on-disk text (`current_claim_status=not_yet_grounded`) remain exactly as they were, proving no forced edit occurred.
  - **Judgment call — this is reported as a real, structural blocker on `CG-03` ("Pilot case upgraded to `oracle_checked`"), not a self-healable gap.** `CG-01` ("All 17 declarative + 21 dynamic criteria `complete` or have a real blocker") explicitly allows this outcome. The blocker is *not* a comparator/normalization/report defect — all four upstream modules work correctly and are proven wired end-to-end by this cycle — it is the absence of a genuine same-character oracle source. Resolving it requires either (a) hand-authoring a real `.pcg` matching the pilot's exact deterministic input in the PCGen checkout (`~/workspace/repos/pcgen`), which is real production-data authorship in a *different* repository, outside this cycle's (and this epic's) `src/oracle_validation/` file-touch grant, or (b) sourcing another legitimate same-character oracle capture. Neither is a same-cycle self-heal.
  - Considered and rejected: silently declaring the substitute run's mismatches "the pilot case's real discrepancies" and reporting *those* as the CG-03 finding. Rejected because that would misrepresent a Paladin-vs-Fighter identity mismatch as if it were evidence about the Fighter pilot build's own correctness — a fabricated-meaning failure, which is exactly the kind of success/failure-shaped fake `no-stub-mvp-doctrine.md` prohibits in either direction.
  - Ran `cargo test --locked --lib` (157/157), `cargo test --locked --test sd26_comparator` (4/4), `cargo test --locked --test sd26_normalization` (7/7), `cargo test --locked --test sd26_parity_report` (10/10), `cargo test --locked --test sd26_pcgen_runner` (6/6), and `cargo test --locked --test sd26_identifier_discipline_audit` (1/1) to confirm no regressions to the four upstream Epic 2 modules or their consumers.
  - Hermes kanban CLI was available this cycle (`hermes kanban --board codex-tranche-5 create ... --assignee operator --initial-status blocked`, then `hermes kanban complete t_b7bb32bf`) — minted and completed as a receipt-only card per step 8, not a live claim.
- **Discovery forwards:**
  - `## DISCOVERED` (forwarded to `progress.md`): no real PCGen-native `.pcg` character file exists anywhere in either repo for the exact pilot build (`pf1-crb-human-fighter-level1`, PF1 CRB Human Fighter level 1). This blocks a genuine `oracle_checked` upgrade for the pilot case (`CG-03`) until either a real matching `.pcg` is hand-authored in the PCGen checkout (out of this epic's file-touch scope — a different repo, production character-data authorship) or another legitimate same-character oracle source is identified. Not self-healable inline.
- **Next-cycle plan:** Epic 2 is otherwise closed (2.1–2.5 all landed). The forwarded blocker above should be picked up either as a new dynamic criterion under a later epic (e.g. alongside Epic 4's book-stub research, or as an Epic 6 closure-scan note) that explicitly scopes hand-authoring a real pilot `.pcg` in `~/workspace/repos/pcgen`, or logged as an accepted, durable known-gap in the golden fixture's own `known_gap_ref` fields for the closure epilogue (6.1) to surface. Epic 3 (JSON Cache Build) can proceed independently; it does not depend on the pilot case's claim-tier upgrade.

## Verification transcript

```text
$ cargo test --locked --test sd26_pilot_case_verification
running 2 tests
test golden_fixture_starts_this_cycle_at_not_yet_grounded ... ok
test full_pipeline_runs_end_to_end_and_the_pilot_case_stays_not_yet_grounded ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 34.80s

$ cargo test --locked --lib
test result: ok. 157 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

$ cargo test --locked --test sd26_comparator
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo test --locked --test sd26_normalization
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo test --locked --test sd26_parity_report
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo test --locked --test sd26_pcgen_runner
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 35.32s

$ cargo test --locked --test sd26_identifier_discipline_audit
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s
```

## Real generated parity report (this cycle's actual output)

```markdown
# Oracle parity report: pf1-crb-human-fighter-level1

## Summary

- Matches: 0
- Mismatches: 9
- Result: FAIL

## Per-Dimension Comparison

| Dimension | PCGen | Codex | Match | Notes |
|---|---|---|---|---|
| character.identity | Florian Syrkov | pf1-crb-human-fighter-level1 | no | — |
| combat.baseline_melee_attack_bonus | 10 | 5 | no | — |
| defense.baseline_armor_class | 22 | 17 | no | — |
| defense.total_save.fortitude | 9 | 4 | no | — |
| defense.total_save.reflex | 5 | 2 | no | — |
| defense.total_save.will | 8 | 1 | no | — |
| skill.selected_modifier.climb | -1 | 5 | no | — |
| skill.selected_modifier.intimidate | 2 | 3 | no | — |
| skill.selected_modifier.swim | -1 | 5 | no | — |

## Normalization Rules Used

- trailing-whitespace-strip (per `normalization.rs`)
- integer-coercion (per `normalization.rs`)

## Discovered Deltas

- `character.identity` — PCGen: Florian Syrkov, Codex: pf1-crb-human-fighter-level1 (value mismatch)
- `combat.baseline_melee_attack_bonus` — PCGen: 10, Codex: 5 (value mismatch)
- `defense.baseline_armor_class` — PCGen: 22, Codex: 17 (value mismatch)
- `defense.total_save.fortitude` — PCGen: 9, Codex: 4 (value mismatch)
- `defense.total_save.reflex` — PCGen: 5, Codex: 2 (value mismatch)
- `defense.total_save.will` — PCGen: 8, Codex: 1 (value mismatch)
- `skill.selected_modifier.climb` — PCGen: -1, Codex: 5 (value mismatch)
- `skill.selected_modifier.intimidate` — PCGen: 2, Codex: 3 (value mismatch)
- `skill.selected_modifier.swim` — PCGen: -1, Codex: 5 (value mismatch)
```

**This report's 9 mismatches are the expected, structural consequence of comparing two different character builds (substitute PCGen Paladin vs. Codex's own Human Fighter pilot chassis) — not a claim about the pilot case's real-world correctness in either direction.**
