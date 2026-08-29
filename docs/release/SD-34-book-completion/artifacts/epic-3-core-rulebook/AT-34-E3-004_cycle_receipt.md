# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-004

- **Commit SHA:** PENDING (filled after commit, before push — see below)
- **Files touched:**
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/step-cost-ledger.json` (new) — the criterion's own evidence artifact.
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/step-cost-ledger-raw-commits.json` (new) — the full per-commit derivation the ledger aggregates, committed for reproducibility.
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-004_cycle_receipt.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`
  - `docs/retro/events/sd34-at-34-e3-004.jsonl` (one `incident` event, the pre-existing denominator-gate regression discovered below)

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — this cycle's own two new files carry no
  `sd[0-9]+_`/`t_[0-9a-f]{8,}` identifiers
  (`grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b' step-cost-ledger.json
  step-cost-ledger-raw-commits.json` → no matches). The epic-scoped diff against
  `merge-base(HEAD, origin/develop)` (`ea2b3396f2`) carries thousands of pre-existing
  `sd32_class_ingest`/`sd32_simple_filename_kind_ingest` matches from earlier, already-committed
  cycles' own receipts and `docs/work-inventory.json` data-field values — not introduced by this
  cycle, same shape every prior E3 receipt in this epic already documents.

- **Wired-integration audit result:** `OK_NO_TOKENS` — this cycle introduces exactly one
  "placeholder" token, inside a **quoted, already-committed commit subject**
  (`step-cost-ledger.json`'s `mechanism_breakdown` key `"AT-34-E3-001 vacuous-placeholder
  sub-cause"`, and the matching literal commit subject in `step-cost-ledger-raw-commits.json`).
  Both are verbatim quotes of `6040c33306`'s own already-audited commit message (PCGen's own
  "no selection" CHOOSE-menu rows, not a stub) — no new stub/mock/placeholder marker in shipping
  code, and this cycle touches no shipping code at all.

- **Acceptance criterion (verbatim, `epic-breakdown.md` AT-34-E3-004):** "For each bucket
  cleared: units cleared, wall time, and what dominated. **Evidence:**
  `artifacts/epic-3-core-rulebook/step-cost-ledger.json`. **This is Epic 5's input and the
  reason this epic exists at all.** A bucket cleared without its cost recorded has delivered
  half its value."

## What this cycle did

Epic 3's buckets are **not yet fully cleared** (see the standing gate below) — this criterion
does not require full closure, only that clearing work already done carries a measured cost.
Three buckets have had dedicated clearing cycles so far: **B** (`AT-34-E3-001`, 29 cycles), **C**
(`AT-34-E3-002`, 1 cycle), **U** (`AT-34-E3-003`, 2 cycles). Four have not yet: D, M, V, X (their
counts moved only as side effects of B/C work, never from a dedicated clearing cycle — recorded
in the ledger's `buckets_not_yet_cleared` section, not silently omitted).

A prior orchestrator pass already produced
`artifacts/epic-3-core-rulebook/step-cost-ledger.derived.json`, and its own embedded note states
plainly that it does **not** satisfy this criterion: it measures cost **corpus-wide, by commit
shape** (generic vs. narrow), not **per atlas bucket, for `core_rulebook`** — the shape the
criterion and Epic 5 actually need. This cycle builds the real artifact:

1. Enumerated every commit touching `docs/work-inventory.json` since the `tranche/14` cut
   (`ea2b3396f2`) with its real commit timestamp — `git log --reverse --format='%H|%ct|%s'
   ea2b3396f2..HEAD -- docs/work-inventory.json` (32 commits).
2. For each commit, checked out `docs/work-inventory.json` at that SHA
   (`git show <sha>:docs/work-inventory.json`) and partitioned it with the **live**
   `completion_atlas.partition(units, book='core_rulebook')` function — the same code the
   standing gate runs, never a re-implementation — to get real bucket counts at that point in
   history.
3. Diffed consecutive bucket-count snapshots to get real per-commit bucket deltas, and classified
   each commit to a bucket by its own commit-message tag (`AT-34-E3-001` → B, `AT-34-E3-002` → C,
   `AT-34-E3-003` "bucket U" → U).
4. Aggregated per bucket: cycle count, total wall minutes (sum of commit-to-commit timestamp
   spans), net bucket-count reduction, units that actually reached `DONE` (real closure) vs.
   units that moved to another unfinished bucket (reclassification — `decisions.md §12` L10),
   and the single costliest mechanism by wall time.

This is **measured**, not estimated: every wall-time figure is a real timestamp delta, every
unit-count figure is a real re-partition of a real historical `docs/work-inventory.json`, not a
model-in-your-head guess.

## Figures + their re-derive commands

- **Bucket B, starting population 970, current remaining 532 (core_rulebook), 29 cycles, 2432.3
  wall-minutes (40.54 hours) total.** Denominator: core_rulebook units only (6,701). Re-derive:
  `git log --reverse --format='%H|%ct|%s' ea2b3396f2..HEAD -- docs/work-inventory.json` (commit
  list), cross-checked against `python3 scripts/completion_atlas.py --book core_rulebook --check`
  run at each SHA (script in this receipt's own commit, `step-cost-ledger-raw-commits.json` is
  its literal output). 503 units left bucket B net; only 235 reached `DONE` (real closure), 268
  moved to another unfinished bucket (D/V/X/M) — reclassification, not closure, per
  `decisions.md §12` L10. `units_per_hour_reaching_DONE` = 5.8. Dominant mechanism by wall time:
  `AT-34-E3-001 option-pool-with-magnitude cycle 8` (617.4 minutes, 2 units) — a live
  domain-power probe requiring per-unit verification, not a batch-groundable pattern.
- **Bucket C, 1 cycle, 61.0 wall-minutes.** 42 units left bucket C; **0 reached DONE** — all 42
  moved to bucket V (verified-by-proxy, still awaiting the oracle harness), a forward
  reclassification, not closure.
- **Bucket U, 2 cycles, 71.5 wall-minutes (1.19 hours).** 48 units left bucket U; **all 48
  reached DONE** — real closure by instrument correction (a rendering defect fix and a ruling),
  not reclassification. `units_per_hour_reaching_DONE` = 40.28 — 7x bucket B's rate, because both
  U cycles were class-wide instrument corrections, not per-record grounding.
- **Buckets D (382), M (1048), V (2793), X (116) remaining, core_rulebook**: no dedicated
  clearing cycle has run for any of them yet. Re-derive current counts:
  `python3 scripts/completion_atlas.py --book core_rulebook --check` (run this cycle at HEAD,
  see Build scope row's SHA).
- **All population and bucket figures above are re-derived at this cycle's own HEAD**, not
  carried forward from `decisions.md §14`'s table or the epic-breakdown's original snapshot
  (`decisions.md §12` L2) — the live atlas run in the previous bullet reports the same 532/372/
  382/1048/2793/10/116 the ledger's `counts_after` for the final commit records.

## Row-count command output (this cycle's own artifact)

```
$ python3 -c "
import json
d = json.load(open('docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/step-cost-ledger.json'))
cleared = d['buckets_cleared_so_far']
not_cleared = d['buckets_not_yet_cleared']
print('buckets_cleared_so_far:', sorted(cleared.keys()), 'count=', len(cleared))
print('buckets_not_yet_cleared:', sorted(not_cleared.keys()), 'count=', len(not_cleared))
for k, v in cleared.items():
    assert 'wall_minutes_total' in v and 'cycles' in v and 'dominant_mechanism' in v, k
print('SCHEMA_OK: every cleared bucket carries cycles + wall_minutes_total + dominant_mechanism')
"
buckets_cleared_so_far: ['B', 'C', 'U'] count= 3
buckets_not_yet_cleared: ['D', 'M', 'V', 'X'] count= 4
SCHEMA_OK: every cleared bucket carries cycles + wall_minutes_total + dominant_mechanism
```

3 of the 7 non-DONE core_rulebook buckets (B, C, U) have a real cost entry; the other 4 (D, M, V,
X) are named, not silently omitted, in `buckets_not_yet_cleared` with their current counts and
why no entry exists yet. **This satisfies AT-34-E3-004 for the buckets actually cleared so far**
— the criterion is "for each bucket cleared", not "for every bucket", and Epic 3 itself has not
closed yet (`AT-34-E3-001`/`002`/`003` remain open per `kanban.md`).

## Build scope verified

- `cargo test --locked --no-run` — **EXIT=0**, run at this cycle's start SHA `4e8d639f36`
  (`tranche/14` HEAD at cycle start; this cycle added no commits before this run). Full target
  list built cleanly (workspace crate); log tail confirms all `tests/*.rs` binaries linked with
  no compile errors.
- `cargo test --locked --lib` / `apps/desktop/src-tauri cargo test --locked`: **not run this
  cycle** — this cycle touches zero Rust source, zero `Cargo.toml`, and zero corpus files; it
  adds two new JSON documents and edits three docs (`progress.md`, `kanban.md`, this receipt).
  No figure this cycle's assertions depend on comes from compiled code, so there is nothing a
  multi-hour full-lib/desktop-crate run could invalidate or confirm that `--no-run`'s clean link
  doesn't already establish. Scoped out per §2.5 ("say which sweeps you did not run").

## Sweep population

N/A — this cycle adds or regenerates no corpus records. `docs/work-inventory.json` is read-only
this cycle (checked out historically at prior SHAs for measurement, never written).

## Oracle pin

N/A — no figure in this receipt derives from the pinned PCGen corpus.

- **Status:** complete
- **Movement, four buckets:** instrument-correction only. This cycle produces a **measurement**
  artifact; it moves no unit's status in `docs/work-inventory.json`. Within the measured data
  itself: bucket B's 503-unit net reduction splits into 235 closure (reached DONE) + 268
  reclassification (moved to D/V/X/M); bucket C's 42-unit reduction is 0 closure + 42
  reclassification (moved to V); bucket U's 48-unit reduction is 48 closure + 0
  reclassification.

## Notes — judgment calls

1. **`step-cost-ledger.derived.json` is superseded as this criterion's evidence, not deleted.**
   It remains in the artifact directory as a secondary corpus-wide cross-check (its own
   `by_shape` generic-vs-narrow finding — 345 vs 20 units/hour — is a genuinely useful, distinct
   observation from this ledger's per-bucket-for-`core_rulebook` view). This receipt's
   `step-cost-ledger.json` is the file `acceptance-and-verification.md` and `epic-breakdown.md`
   both name as the Evidence artifact.
2. **Wall time is an upper bound**, inherited caveat from the derived ledger: commit-to-commit
   timestamp spans include queue/dispatch overhead between cycles, not pure engineering time.
   Stated in `step-cost-ledger.json`'s own `measurement_method.wall_time` field, not hidden.
3. **No TDD RED→GREEN pair applies to this cycle.** This cycle ships a measurement/report
   artifact (same class as `table-build-rate.json` in Epic 2, which also shipped with no
   accompanying script test) — it is not shipping code under the no-stub-MVP doctrine, and
   inventing a script + test harness to validate a static JSON's own schema would be scope
   expansion beyond what this criterion or its Epic 2 precedent required. The row-count command
   above (with its inline `assert`) is the concrete, re-runnable verification in place of a
   committed test.
4. **Pre-existing denominator-gate regression found, not caused by this cycle.**
   `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'`
   reports `files_checked=15 violations=2` at this cycle's start HEAD (both violations in
   `progress.md` lines 22/28, quoting `36db23a053`'s own already-committed receipt prose — a
   literal corpus percent-sign phrase, `FRT_HVY`'s digit-preceded "chance to negate" string).
   Confirmed pre-existing via `git show
   HEAD:docs/release/SD-34-book-completion/progress.md` before this cycle's own edits. Filed as
   `docs/retro/events/sd34-at-34-e3-004.jsonl` (`incident`,
   `denominator-gate-percent-literal-false-positive`), **not self-healed here** — it is another
   already-closed cycle's (`AT-34-E3-003`) committed receipt text, outside this criterion's
   file-touch set (`artifacts/epic-3-core-rulebook/`, not `progress.md`'s historical entries),
   and `progress.md` is a shared prepend-only log where rewriting another lane's already-landed
   entry risks a collision. Reported here for the operator/closure scan, per `decisions.md §12`
   L22 (a repeated workaround means clear the obstacle) — this is a first observation of this
   recurrence key, not yet a 3+ recurrence requiring a mechanical control.

## Next-cycle plan

- Epic 3's remaining criteria (`AT-34-E3-001`/`002`/`003`) continue clearing buckets B/C/D/M/V/X.
  Each subsequent clearing cycle should append its own bucket to `step-cost-ledger.json`'s
  `buckets_cleared_so_far` (same derivation method: re-run the per-commit atlas re-partition from
  this cycle's last recorded SHA forward) rather than starting a fresh ledger.
- The pre-existing `progress.md` denominator-gate violation (Note 4) should be corrected by
  whichever cycle next touches `progress.md`'s lines 22/28, or by `AT-34-E6-001`'s closure scan —
  named here so it is not lost.
