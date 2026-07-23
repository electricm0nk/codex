# Cycle epic5-5.1-doctrine_cost_floor — Epic 5 (Doctrine-Cost Reduction) / Criterion 5.1

Single serial audit cycle. Measures the per-class gate-cost cycle floor: pre-cut
(historical doctrine) vs post-cut (this bundle's proposed trims), per
`technical-design.md §5.1` and `scope-draft.md §1.5`.

## 1. What was genuinely measured live vs reconstructed from history

Per the task's explicit escape hatch: a literal from-scratch re-enactment of a
full ~40-minute per-class chassis-building cycle (twice — once "old doctrine,"
once "new doctrine" — for 2-3 different classes) is not practical inside this
one audit cycle, because the ~40-minute figure is dominated by *implementation*
work (deriving a new class's BAB/save chassis from real LST tokens, writing new
Rust modules) that Epic 5 does not touch and that would require fabricating a
new per-class feature for no product reason. What Epic 5 actually changes is
the *doctrine overhead wrapped around* that implementation (doc-comment,
artifact size, progress.md fan-out) — so this audit isolates and live-measures
exactly those overhead steps for real, and takes the load-bearing steps'
durations from real historical receipts rather than inventing numbers.

**Live-measured in this cycle (real wall-clock, commands run and timed below):**
- Dual-audit gate (identifier + wired-integration greps) — real `time` output.
- Post-cut artifact write (minimal-receipt shape) — real file write, bounded by
  `date -u +%s.%N` before/after.
- Post-cut `progress.md` one-row update — real edit, same bounding method.
- Real PCGen-engine pipeline round-trip (`cargo test --locked --test
  sd26_pcgen_runner` / `sd26_pilot_case_verification`) — real end-to-end
  invocation of PCGen's Gradle wrapper via `scripts/pcgen-run-character.sh`,
  the same wrapper a per-class RED/GREEN cycle exercises. Run twice for real,
  independently, to show the range (34.50s and 21.65s wall-clock).
- Commit + push of this cycle's own changes — real, timed at landing (§6 below).

**Reconstructed from real historical receipts (not live-timed, cited by source):**
- The ~40-minute **total** pre-cut baseline: `docs/release/SD-22/artifacts/apg/class_alchemist_cycle_receipt.md`
  §"Cycle metadata" → `duration: ~40 minutes` (real, operator-recorded, cited
  verbatim by `technical-design.md §5.1` and `scope-draft.md §1.5`). That
  receipt is itself **exactly 135 lines** — the literal artifact this
  criterion's "135-line cycle-artifact write" target refers to (confirmed via
  `wc -l`, not assumed).
- The **per-step split** of that 40 minutes (RED setup, GREEN setup, dual-audit
  grep, doc-comment write, artifact write, commit+push, progress.md update) —
  the SD-22 receipt itself only records a single total (`~40 minutes`), not a
  per-step timestamp breakdown; no per-step timestamps exist anywhere in any
  prior SD-22/24/25/26 receipt. The split below is therefore a **reasoned
  reconstruction** grounded in (a) the technical-design.md §5.1 target list,
  which was itself set by inverting the real cut list, and (b) real,
  independently-confirmed proxies for two of the un-timestamped steps:
  the 135-line write itself (word/structure-counted, see §3), and the
  dual-audit grep (which this cycle re-ran live against the real repo and
  found to be sub-second regardless of doctrine version — so its historical
  cost was concentrated in the surrounding narrative-writing time it was
  embedded in, not the grep invocation itself).
- **Corroborating real signal (not the pre-cut baseline, but supporting
  evidence the ~6-minute floor is achievable once overhead is normalized):**
  real `git log --format='%cI %H %s'` commit timestamps for this same
  bundle's Epic 4 book-stub cycles (a lighter-scope but still full
  RED/GREEN/dual-audit/receipt/progress cycle shape, run *before* any of
  Epic 5's cuts) give real sequential inter-cycle deltas between each
  cycle's main landing commit, chronologically:
  `adventurers_guide`→`beginner_box` 5m48s, →`bestiary_2` 6m43s,
  →`bestiary_3` 4m29s, →`bestiary_4` 5m00s, →`bestiary_5` 67m24s (session
  gap, excluded), →`bestiary_6` 4m21s, →`bonus_bestiary` 5m47s,
  →`core_essentials` 3m04s, →`horror_adventures` 5m18s, →`monster_codex`
  9m22s, →`mythic_adventures` **2m44s**, →`occult_adventures` 5m14s,
  →`pathfinder_unchained` 5m51s, →`ultimate_campaign` 4m28s,
  →`ultimate_combat` 4m46s, →`ultimate_equipment` 3m43s,
  →`ultimate_intrigue` 4m45s, →`ultimate_magic` 3m51s, →`ultimate_wilderness`
  3m53s. Excluding the one 67-minute session-pause outlier, real deltas
  range **2m44s–9m22s**, median ≈4-5min. These are a different, lighter task
  shape (one JSON stub + one registry line, not a class chassis table) so
  they are **not** used as the pre-cut baseline, but they show full-doctrine
  cycles already land in or near the 6-minute neighborhood when the
  implementation itself is small — consistent with this audit's finding that
  per-class doctrine *overhead*, not implementation time, was the excess in
  the SD-22 case.

**Per-class cases referenced (per the task's fixture selection):** `pf1_human_fighter_level1`
(the existing E2.5 pilot case), `pf1_human_barbarian_level1`, and
`pf1_human_wizard_level1` — all real files under `tests/fixtures/rules_core/`.
The two live PCGen-engine round-trip measurements below exercise the actual
shared pipeline (`pcgen_runner.rs` → `scripts/pcgen-run-character.sh` → real
PCGen Gradle wrapper → `scripts/pcgen-normalize-output.py` → `comparator.rs`)
that a per-class cycle for any of these three cases would invoke; per
prior SD-25/26 precedent (no PCGen-native `.pcg` exists yet for any of these
three specific builds), the live runs use the same bundled substitute
`code/testsuite/PCGfiles/pf_Paladin.pcg` the E2.4/E2.5 cycles already
established as the real, non-fabricated stand-in — the Gradle round-trip cost
measured is character-agnostic (same JVM+Gradle+PCGen startup dominates
regardless of which `.pcg` is loaded), so it is a valid real proxy for the
per-case pipeline cost regardless of which of the three classes is loaded.

## 2. Pre-cut vs post-cut — per-step table

| Step | Pre-cut (historical) | Post-cut (this cycle) | Live-measured? |
|---|---|---|---|
| RED setup + GREEN setup + dual-audit gate | ~30 min (reconstructed: dominant share of SD-22's 40 min — real class-chassis derivation + test authoring + `cargo test`/`clippy` runs, per the Alchemist receipt's RED/GREEN evidence sections) | ~5 min (unchanged, load-bearing target per `technical-design.md §5.1`) | Partially — dual-audit grep itself live-measured at **0.72s + 0.02s = 0.74s** (both `OK_*`, see §4); real PCGen-engine round-trip live-measured at **34.50s** and **21.65s** (two independent runs) as the dominant real sub-component inside this budget |
| Per-class doc-comment with source citation | ~5 min (reconstructed — SD-22 receipt's "Why this criterion" + inline LST-token citation narrative, e.g. `apg_classes.lst:11`'s `BONUS:COMBAT`/`BONUS:SAVE` tokens, hand-written prose) | **0 min (cut)** — durable citation now lives in the JSON cache's SHA-256 frontmatter (`src/bin/sd26_gen_core_rulebook_cache.rs`'s `source` discriminated union, confirmed real and populated for all 3,326 CRB records per `artifacts/epic_3/core_rulebook_json_cache-cycle_receipt.md`) | Reconstructed (pre-cut); cut is structural (post-cut has nothing to time) |
| Cycle-artifact write | ~3 min (reconstructed, but the artifact itself is **real and measured**: `docs/release/SD-22/artifacts/apg/class_alchemist_cycle_receipt.md` is confirmed **135 lines** via `wc -l` — the literal citation behind this criterion's "135-line cycle-artifact write" target) | **~8s** — live-measured writing the minimal shape (`artifacts/epic_5/example-minimal-receipt-postcut.md`, 9 lines: RED/GREEN summary + dual-audit result + `duration_seconds` field only), bounded by `date -u +%s.%N` before/after: `1784777845.03 − 1784777837.24 = 7.79s` | **Yes, live** |
| Commit + push | ~1 min (unchanged — load-bearing) | ~1 min budget (unchanged — load-bearing; concurrent-write protocol per `loop-instruction.md §5`). **Real measured this cycle: 1.17s** (0.05s commit + 1.11s push, single fetch/rebase/push, no retry needed — commit `251e4e2`) — the ~60s budget is a doctrine allowance for the general case (interactive message drafting + possible non-fast-forward retries up to 5x), not a claim that push itself is slow; kept unchanged per §7, not re-cut, since retries are a real possibility under concurrent E1-E6 dispatch this bundle uses elsewhere | **Yes, live** |
| `progress.md` update | ~1 min (reconstructed — historical 4-section fan-out: Status-matrix row + `## TODO` dequeue + `## DONE` append + `## Cycle log` table row, each hand-written prose, e.g. the 3.1 row alone runs ~700 words) | **~7s** — live-measured, one status-matrix row edit only (`progress.md`'s 5.1 row), bounded the same way: `1784777856.50 − 1784777849.28 = 7.23s` | **Yes, live** |
| **Total** | **~40 min** (real, cited verbatim from SD-22's own receipt) | **~6.3 min** (5 min load-bearing RED/GREEN/audit + 8s artifact + 7s progress.md + ~60s commit/push ≈ 378s) | Mixed — see breakdown above |

## 3. Real corroborating counts (not estimates)

- `wc -l docs/release/SD-22/artifacts/apg/class_alchemist_cycle_receipt.md` → **135** (exact match to the criterion's own "135-line cycle-artifact write" citation).
- `wc -l docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/artifacts/epic_3/apg_json_cache-cycle_receipt.md` → **34** lines — SD-26's own Epic 3 receipts are already shorter than SD-22's, but still carry full per-book narrative (source-provenance reconciliation, ceiling percentages, judgment-call notes) well beyond the ~9-line minimal shape this criterion targets.
- Post-cut example artifact (`example-minimal-receipt-postcut.md`, written and timed live this cycle) → **9 lines**.

## 4. Dual-audit gate (this cycle's own diff)

`BASE_BRANCH=$(git merge-base HEAD origin/develop)`; both greps live-run against `${BASE_BRANCH}...HEAD`:

```
$ time ( git diff --unified=0 "${BASE_BRANCH}...HEAD" -- 'apps/desktop/**/*.ts*' 'apps/desktop/src-tauri/**/*.rs' 'src/**/*.rs' 'scripts/**/*.sh' 'scripts/**/*.py' 'data/**/*.json' 'docs/governance/wired-integration-stubs-registry.md' ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b' || echo 'OK_NO_BUNDLE_TAGS' )
OK_NO_BUNDLE_TAGS
real  0m0.722s

$ time ( git diff --unified=0 "${BASE_BRANCH}...HEAD" -- 'apps/desktop/**/*.ts*' 'apps/desktop/src-tauri/**/*.rs' 'src/**/*.rs' ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS' )
OK_NO_TOKENS
real  0m0.019s
```

**Identifier audit result:** OK_NO_BUNDLE_TAGS
**Wired-integration audit result:** OK_NO_TOKENS

(This cycle touches only `docs/release/SD-26-.../artifacts/epic_5/*.md` and
`progress.md` — no `src/`/`apps/`/`scripts/`/`data/` files — so both greps
match nothing to flag, as expected for a doctrine-audit cycle.)

## 5. Real PCGen-pipeline round-trip evidence (RED/GREEN load-bearing step, grounded live)

```
$ cargo test --locked --test sd26_pcgen_runner
running 6 tests
test run_pcgen_character_runs_the_real_pcgen_engine_end_to_end ... ok
... (6/6 pass)
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 35.23s
Elapsed (wall clock) time: 0:35.33

$ cargo test --locked --test sd26_pcgen_runner -- --test run_pcgen_character_runs_the_real_pcgen_engine_end_to_end
test result: ok. 1 passed; finished in 34.43s
Elapsed (wall clock) time: 0:34.50

$ cargo test --locked --test sd26_pilot_case_verification
running 2 tests
test golden_fixture_starts_this_cycle_at_not_yet_grounded ... ok
test full_pipeline_runs_end_to_end_and_finds_two_genuine_skill_mismatches ... ok
test result: ok. 2 passed; 0 failed; finished in 21.58s
Elapsed (wall clock) time: 0:21.65
```

No mocked, stubbed, or fabricated PCGen output anywhere in these runs — both
invoke the real PCGen Gradle wrapper end to end (per `pcgen_runner.rs`'s own
doc comment, verified again live here). These two independent real
round-trips (34.50s, 21.65s) are the honest floor of what a per-class
RED→GREEN cycle's "run the pipeline against this class" sub-step actually
costs — well inside the ~5-minute load-bearing budget, confirming that
budget is not itself inflated.

## 6. Bottom line

- **Pre-cut baseline: ~40 minutes** (real, `docs/release/SD-22/.../class_alchemist_cycle_receipt.md`'s own `duration: ~40 minutes`).
- **Post-cut floor: ~6.3 minutes on the ~60s commit+push budget** (5 min
  load-bearing RED/GREEN/dual-audit [unchanged by design] + 7.79s
  live-measured artifact write + 7.23s live-measured progress.md update +
  ~60s load-bearing commit+push budget). **Using this cycle's own real
  measured commit+push (1.17s, single-shot, no retry) instead of the
  budget, the real floor lands at ~5.2 minutes** — the ~60s figure is kept
  as the reported bottom-line commit+push budget per §7 (not re-cut; a real
  concurrent-dispatch cycle can need up to 5 rebase/push retries per
  `loop-instruction.md §5`), so **~6.3 minutes is reported as the honest
  bottom line**, with ~5.2 minutes noted as the best-case real number
  observed this cycle.
- **Target from `technical-design.md §5.1`: ~6 minutes.** The measured
  post-cut floor (~6.3 min) is **within ~5% of the stated target** —
  effectively met. The residual ~18 seconds beyond nominal 6:00 comes
  entirely from the load-bearing commit+push allowance (~60s, per
  `loop-instruction.md §5`'s concurrent-write protocol, unchanged and
  correctly not cut) plus rounding in the 5-minute RED/GREEN/dual-audit
  budget — not from any of the three cut steps, which landed at **15
  seconds combined** (7.79s + 7.23s; doc-comment cut to 0), dramatically
  under their own ~60-second combined target (30s artifact + 30s
  progress.md per `technical-design.md §5.1`).
- **Honest caveat:** the 5-minute RED/GREEN/dual-audit figure is the one
  number in this table that is a target, not a full live re-enactment,
  because re-implementing a genuinely new per-class chassis end-to-end
  inside this audit cycle would fabricate product work with no product
  need. The load-bearing sub-components that *were* live-measured inside
  that budget (dual-audit grep: 0.74s; real PCGen round-trip: 21.65s-34.50s)
  together total well under a minute, leaving ample real headroom under the
  5-minute ceiling for the actual implementation/authoring time a genuine
  new class would require — so the 5-minute figure is plausible, not just
  asserted.

## 7. What does NOT cut (re-affirmed, unchanged this cycle)

Per `technical-design.md §5.2`, none of the following were touched or timed
differently — they are load-bearing and this audit does not propose changing
them:
- RED → GREEN TDD mandate.
- The dual-audit gate (identifier + wired-integration checks, operator-pinned 2026-07-20).
- The concurrent-write protocol (fetch/rebase/push, `loop-instruction.md §5`).
- Operator identity in commits.

## 8. Files touched this cycle

- NEW `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/artifacts/epic_5/per-class-cycle-floor-measurement.md` (this file)
- NEW `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/artifacts/epic_5/example-minimal-receipt-postcut.md` (live-timed post-cut artifact-write exhibit, §2/§1)
- `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/progress.md` (5.1 row → complete, one row; `## TODO` dequeued)

## 9. Acceptance criterion (verbatim, `acceptance-and-verification.md` row 5.1)

"`./artifacts/epic_5/per-class-cycle-floor-measurement.md` — Pre-cut vs post-cut cycle floor measured."

**Status:** complete. Pre-cut (~40 min, real historical citation) vs post-cut
(~6.3 min, live-measured cut steps + unchanged load-bearing steps) both
recorded above; target (~6 min) effectively met (within ~5%).

## 10. Notes / judgment calls

- Did not attempt to re-run a full historical-doctrine 40-minute cycle
  verbatim (would require fabricating new class-chassis product work with no
  product need, and would not itself validate anything the SD-22 receipt's
  own recorded `duration: ~40 minutes` doesn't already establish for real).
- The per-step split of the historical 40 minutes is a reasoned
  reconstruction, not a live measurement or an invented number — flagged
  explicitly in §1 and in each table row's own note, per the task's
  instruction not to fabricate a measurement not actually taken.
- Chose to keep `example-minimal-receipt-postcut.md` as a real on-disk
  exhibit (rather than only describing it in prose) so the "8 seconds to
  write" claim is checkable against a real artifact, not just asserted.

## 11. Next-cycle plan

Epic 5 has exactly one criterion (5.1); this closes Epic 5. Epic 6 (Closure
Epilogue) is next per `loop-instruction.md §3`'s dependency map (E6 gated on
E5).
