# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 — wave-10 regeneration and attribution: measuring the bucket-B (cycle 10) engine lane

- **Commit SHA:** this receipt is committed in the same commit as the `docs/work-inventory.json`
  regeneration and `completion-atlas.json` it describes; `git log -1` at the time of reading
  resolves it. Base rebased onto `origin/tranche/14` at `935cef27b5` before any pass ran
  (fast-forward, no conflicts).
- **Files touched:** `docs/work-inventory.json` (regenerated), `docs/release/SD-34-book-completion/
  artifacts/epic-1-atlas/completion-atlas.json` (regenerated output of `completion_atlas.py
  --check`), this receipt, `docs/release/SD-34-book-completion/progress.md`, `docs/release/
  SD-34-book-completion/kanban.md`. No `src/**` file was touched this cycle — the bucket-B engine
  lane (`935cef27b5`) had already landed its own code changes; this cycle only measures them.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` —
  `git diff --unified=0 -- docs/work-inventory.json | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  → zero matches.
- **Wired-integration audit result:** `OK_NO_TOKENS` — same diff,
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → zero matches.
- **Acceptance criterion:** this cycle implements no `epic-breakdown.md` criterion directly — it is
  the regeneration-and-attribution cycle that measures cycle 10's bucket-B batch (`935cef27b5`),
  which landed its own code change and left `docs/work-inventory.json` unregenerated. Evidence:
  the whole-corpus before/after diff below, checked unit-by-unit against cycle 10's own stated
  expectations in `progress.md` and `kanban.md`.

## Why this cycle exists

Cycle 10's bucket-B batch (`fix(sd34): AT-34-E3-001 cycle 10 -- bucket-B batch: widen Druid/Monk
citation gate + fix 8 pre-existing stale anti-fabrication tests`, `935cef27b5`) widened the
`class_feature_grant_consumer.rs` citation gate and removed a stale class-wide Druid/Monk
exclusion, then stated an expectation in its own `progress.md`/`kanban.md` entries: 2 closures
(`Monk ~ Flurry of Blows`, `Monk ~ Unarmed Strike`) plus "6 bonus closures in a different
mechanism" (5 Monk + 1 Druid, all in `no_explanation_id_and_no_diagnostic_names_this_feature`).
It did not regenerate `docs/work-inventory.json`. This cycle runs the three-pass pipeline and
diffs the result against that stated expectation, unit by unit.

A sibling bucket-V lane (`e7b87138d1`) also landed on this branch but wrote artifacts only
(`artifacts/epic-3-core-rulebook/bucket-v/`) and changed no unit statuses — explicitly out of
this cycle's scope, not applied, and checked below for accidental movement.

## Procedure and per-pass wall time

Environment: `RETRO_ACTOR=sd34-regen-b`, `CARGO_TARGET_DIR=/tmp/cargo-sd34-regenb`,
`CARGO_INCREMENTAL=0`, fresh target directory.

```bash
git fetch origin tranche/14 && git rebase origin/tranche/14   # fast-forward, no conflicts, landed at 935cef27b5
git show HEAD:docs/work-inventory.json > /tmp/wi-before-regenb.json   # snapshot before any pass
```

| Pass | Command | Wall time | Result |
|---|---|---|---|
| 1 | `cargo run --locked --bin corpus_literal_sweep -- --json-out /tmp/sweep.json` | **191s (3m11s)** | `48708 records examined of 51482 read, 413336 tokens compared (9 synthesized), 51469 digests checked, 0 findings`, `CLEAN` |
| 2 | `cargo run --locked --bin derived_evaluator_fixture_check -- --json-out /tmp/fixture.json` | **12s** | `1839 unit(s) cleared over 2580 fixture row(s); 0 failed; 0 not ingested` |
| 3 | `CORPUS_LITERAL_SWEEP_REPORT=/tmp/sweep.json DERIVED_FIXTURE_CHECK_REPORT=/tmp/fixture.json cargo run --locked --bin v06_work_inventory` (no `--allow-stamp-loss`) | **666s (11m6s)** | exit 0, ran guarded (both reports present, no stamp loss refused) |

Total pipeline: **~14m29s**. The guard was never tested into refusing (both prior reports were
present from the start), consistent with `--allow-stamp-loss` never being passed, per instruction.

## Whole-corpus before/after diff, by unit id

```python
import json
before = json.load(open('/tmp/wi-before-regenb.json'))
after = json.load(open('docs/work-inventory.json'))
bi = {u['id']: u for u in before['units']}
ai = {u['id']: u for u in after['units']}
print('before', len(bi), 'after', len(ai))
print('added', len(set(ai)-set(bi)), 'removed', len(set(bi)-set(ai)))
changed = [uid for uid in set(bi)&set(ai)
           if bi[uid]['status'] != ai[uid]['status'] or bi[uid]['evidence'] != ai[uid]['evidence']]
print('changed', len(changed))
```

Result: **49,438 ids before, 49,438 after, 0 added, 0 removed, 10 changed** (status or evidence),
**all 10 in `core_rulebook`** — consistent with `935cef27b5` touching only
`class_feature_grant_consumer.rs`.

## Bucket-V scope check (explicitly out of scope)

Cross-checked all 10 changed ids against `artifacts/epic-3-core-rulebook/bucket-v/
bucket-v-consolidated.oracle-results.json`'s 2,712 dispositioned `unit_id`s: **zero overlap.**
Bucket-V's proposal remains fully inert with respect to this regeneration — no unit this cycle
moved is one bucket-V claims to disposition. Bucket-V was not applied, consulted, or allowed to
influence any figure below.

## Attribution — expected vs actual, per mechanism

### `class_feature_owner_matched_by_name_but_record_not_held_by_engine`

**Expected:** 239 → 237 (2 closures: `Monk ~ Flurry of Blows`, `Monk ~ Unarmed Strike`).

**Actual population: 239 → 237 — the count matches exactly.** Membership does not:

| Unit | Before | After | Outcome |
|---|---|---|---|
| `monk_unarmed_strike` | `engine-does-not-hold` / `class_feature_owner_matched_by_name_but_record_not_held_by_engine` | `text-complete` / `explanation_id_observed_and_corpus_record_carries_real_description` | **real closure (DONE)** |
| `monk_flurry_of_blows` | `engine-does-not-hold` / `class_feature_owner_matched_by_name_but_record_not_held_by_engine` | `engine-does-not-hold` / `class_feature_no_dedicated_magnitude_id_matched_the_record_slug` | **reclassification only (B → D), NOT closed** |

**Verdict: count confirmed, membership refuted.** Only 1 of the 2 predicted closures actually
reached DONE. `Flurry of Blows` left this mechanism (correctly accounting for the 239→237 drop)
but landed in a new, un-named `engine-does-not-hold` bucket, not in `grounded`/`text-complete`.
The lane's own receipt reported this as "2 closures"; it was 1 closure + 1 reclassification.

### `no_explanation_id_and_no_diagnostic_names_this_feature` ("6 bonus closures")

**Expected (per `progress.md` Cycle 10 and `kanban.md` row 13):** "6 bonus closures in a
different mechanism's own bucket (5 Monk + 1 Druid)" — `Monk ~ Abundant Step`, `Diamond Soul`,
`Maneuver Training`, `Perfect Self`, `Stunning Fist`, and `Druid ~ Nature Bond`.

**Actual: refuted, and in the opposite direction.** This mechanism's `core_rulebook` population
went **363 → 357, a decrease of 6** — not a gain. All 6 named units were **already** classified
under this mechanism (evidence contains `explanation_id` as a substring, so `completion_atlas.py`
buckets it `C`) in the pre-regen snapshot, and this cycle's regeneration moved every one of them
**out**, to the same new evidence string:

| Unit | Before | After |
|---|---|---|
| `monk_abundant_step` | `engine-does-not-hold` / `no_explanation_id_and_no_diagnostic_names_this_feature` | `engine-does-not-hold` / `class_feature_no_dedicated_magnitude_id_matched_the_record_slug` |
| `monk_diamond_soul` | same | same new evidence |
| `monk_maneuver_training` | same | same new evidence |
| `monk_perfect_self` | same | same new evidence |
| `monk_stunning_fist` | same | same new evidence |
| `druid_nature_bond` | same | same new evidence |

None reached `grounded` or `text-complete`. **Zero of the predicted 6 bonus closures
materialized** — all 6 reclassified `C → D`, remaining `engine-does-not-hold`.

**Verdict: refuted entirely.** The lane's own "6 bonus closures" framing was backwards on two
counts: these units did not newly arrive at this mechanism as a side effect (they were already
there), and none of the 6 closed to DONE — they left this mechanism for an un-named D-bucket
evidence string instead.

### The un-predicted destination: `class_feature_no_dedicated_magnitude_id_matched_the_record_slug`

Neither `progress.md` nor `kanban.md`'s cycle-10 entries name this evidence string anywhere.
`core_rulebook` population under it: **22 before → 29 after, +7** — exactly the 7 reclassified
units above (`monk_flurry_of_blows` + the "bonus 6"). This is the citation-gate widening's real,
un-credited effect: it correctly moved these 7 records out of their prior evidence paths, but the
record itself still carries no dedicated magnitude id the engine can match to a slug, so
`classify()` falls through to this existing `D`-bucket fallthrough rather than to `grounded`/
`text-complete`. This is a real code behavior, not a measurement artifact — confirmed by the
`completion_atlas.py` bucket move (`C`/`B` → `D`) below.

### `class_feature_option_pool_record_with_magnitude_not_held_by_engine`

**Expected:** 208, unchanged. **Actual:** 208, unchanged — byte-identical membership, verified
by direct id-set comparison (`before_ids == after_ids`). **Confirmed exactly.**

### `class_feature_option_pool_record_not_held_by_engine`

**Expected:** 25, unchanged. **Actual:** 25, unchanged — byte-identical membership, verified the
same way. **Confirmed exactly.**

### Evidence-string churn, no bucket crossed (2 units, not attributed to closure or reclassification)

`druid_orisons` and `druid_spontaneous_casting` — both already `text-complete` (DONE) before and
after — changed `evidence` only:
`class_feature_pool_catalog_serves_a_rendered_description` →
`explanation_id_observed_and_corpus_record_carries_real_description`. Same pattern the wave-9
regeneration documented for the gate-widening lane's rewrite reaching already-admitted units;
reported separately per `decisions.md §9`, not folded into closure or reclassification.

## Movement, four buckets (`decisions.md §9`)

- **Closure (reached DONE):** **1** — `monk_unarmed_strike` (B → `text-complete`).
- **Reclassification (between non-DONE buckets):** **7** — `monk_flurry_of_blows` (B → D),
  `monk_abundant_step`, `monk_diamond_soul`, `monk_maneuver_training`, `monk_perfect_self`,
  `monk_stunning_fist`, `druid_nature_bond` (all C → D). **A B→D or C→D move is reclassification,
  never closure**, per this cycle's own instruction.
- **Evidence-string churn, no bucket crossed:** **2** — `druid_orisons`, `druid_spontaneous_casting`
  (see above), reported separately.
- **Instrument-correction:** **0.** No wrong count was found in the inventory itself; the gap
  between the lane's stated expectation and the actual result is a real code-classification
  outcome (the citation-gate widening genuinely produces `D`-bucket records here, not `DONE`
  ones), not a measurement error.

**1 + 7 + 2 = 10**, the full changed-unit count.

**Net effect: cycle 10's own receipt claimed 8 total closures (2 + 6); the actual, measured
result is 1.** This is the single most load-bearing finding of this cycle.

## `completion_atlas.py` — corpus-wide and per-book

```
$ python3 scripts/completion_atlas.py --check
population=49438 buckets=10 unclassified=0 overlap=0
  DONE: 14741  A: 449  B: 11769  C: 4338  D: 3078  M: 5114  V: 9558  U: 202  X: 170  Z: 19
done_evidence_violations=0
missing_clearing_mechanisms=0
stale_derived_at=False
citation_failures=0
```
Exit 0. All 10 `BUCKET_DEFINITIONS` `file:line` citations were re-checked directly against
`src/bin/v06_work_inventory.rs` at HEAD **before** any pass ran (`awk 'NR==<line>'` against each
of the 10 cited lines) — all 10 still resolve and still contain their claimed marker, because
this cycle's own scope touched no `src/**` file. No re-derivation was needed.

```
$ python3 scripts/completion_atlas.py --book core_rulebook --check
book=core_rulebook population=6701 unclassified=0 overlap=0
  DONE: 1503  A: 0  B: 470  C: 357  D: 405  M: 1048  V: 2793  U: 10  X: 115  Z: 0
```
(exit 1 — book not yet closed, expected. `DONE` 1502→1503 (+1, the one real closure); `B`
472→470 (-2, the two units leaving `owner_matched`); `C` 363→357 (-6, the six units leaving
`no_explanation_id`); `D` 398→405 (+7, the seven units landing in the new evidence string) — all
four deltas reconcile exactly against the attribution table above.)

```
$ python3 scripts/completion_atlas.py --book ultimate_campaign --check
book=ultimate_campaign population=265 unclassified=0 overlap=0
  DONE: 130  A: 0  B: 0  C: 0  D: 5  M: 89  V: 18  U: 21  X: 2  Z: 0
```
(exit 1, unchanged from the wave-9 baseline — 0 of this book's units appear in the 10-unit
changed set; cycle 10's own change is `core_rulebook`-scoped only.)

## Denominator gate

```
$ python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'
files_checked=15
violations=6
```

**Not 5, as the wave-9 receipt (`progress.md:975`) reported for its own run.** Verified directly
rather than inherited: read all 6 flagged lines (`progress.md:205, 472, 515, 572, 578, 975`) at
their exact line numbers. Every one genuinely quotes the same pre-existing, already-merged
`AT-34-E3-004` cycle's corpus-prose fragment (`FRT_HVY`'s percent-chance description), in six
different historical narration contexts spanning several past cycles' own progress-log entries.
None was introduced by this cycle (this cycle added no new `.md` prose containing a bare
percentage — its only `.md` edits are this receipt plus the `progress.md`/`kanban.md` entries
below, neither of which repeats the flagged substring).

**The mechanism, named rather than just counted:** the violation count has climbed one-by-one
across cycles (2 → 3 → 4 → 5 → 6, visible at `progress.md:515/472/205/975` respectively) because
each cycle's own `denominator_gate.py` report, once committed to `progress.md`, itself quotes the
flagged substring for context — and that quoted line becomes a **new** instance of the same
violation for the *next* cycle's scan to find. This is a self-perpetuating recursive citation,
not a growing population of real defects; nothing here is "ours" to fix (the 6 flagged lines all
predate this cycle and are outside this cycle's own file-touch set). This cycle's own new prose
in `progress.md`/`kanban.md` deliberately avoids re-quoting the flagged substring, so as not to
mint a 7th instance for the next cycle.

## Build scope verified

`cargo test --locked --no-run` (workspace): exit 0, **130s**, run at this cycle's final HEAD
(after the regeneration commit). `cargo test --locked --no-run --manifest-path
apps/desktop/src-tauri/Cargo.toml`: exit 0, **211s (3m31s)**, explicit desktop-crate run
(`decisions.md §10`). Neither build was re-run against a code change — this cycle touched no
`src/**` file — but both are re-verified at the widest scope per instruction, since the
regenerated `docs/work-inventory.json` and `completion-atlas.json` are the cycle's own committed
output.

## Sweep population

Pass 1 (`corpus_literal_sweep`) examined 48,708 of 51,482 records read, 0 findings, `CLEAN`.
This cycle added or edited no `data/corpus/**` record — the examined-count is unaffected by
`935cef27b5`'s `src/**`-only change.

## Oracle pin

N/A — no figure in this receipt came from the pinned PCGen oracle corpus.

## Row-count command output

`docs/work-inventory.json`'s own row count is `population=49438` (unchanged, 0 added/removed).
The figure that matters here is the **changed-unit** count (10), derived by the before/after
diff script quoted above, and its four-way movement split (1 / 7 / 2 / 0), which sums to 10.

## Status

**complete** — the three-pass pipeline ran to completion with both prior reports present (never
`--allow-stamp-loss`), the whole-corpus diff was taken and attributed unit-by-unit against cycle
10's own stated expectations, and every mismatch is reported explicitly rather than smoothed
over. Per this cycle's own instruction, an expectation that turns out wrong is the valuable
finding here, not a failure to report.

## Notes

- **The lane's own receipt significantly overclaimed its own result.** Stated: 2 + 6 = 8 total
  closures. Measured: 1. This is not an instrument problem (the instrument agrees with itself —
  `completion_atlas.py`'s bucket counts and the raw before/after diff reconcile exactly) — it is
  the lane's own commit reasoning about what the citation-gate widening would produce, checked
  against what the widened code actually does, and found short by 7 of 8 predicted closures.
- **The "count right, membership wrong" pattern recurred exactly as this wave's brief warned it
  might** (`owner_matched` 239→237 population-count-correct, closure-membership-wrong) —
  independently of, and in addition to, the "6 bonus" claim being wrong on both count-direction
  and membership at once.
- **Bucket-V's inertness is now cross-checked, not merely assumed:** zero overlap between this
  cycle's 10 changed units and bucket-V's 2,712-unit disposition list.
- **The denominator-gate violation count's growth mechanism is now named**, not just
  re-measured — future cycles reporting this gate's output should expect the count to keep
  climbing by exactly the number of times their own report quotes the flagged substring, and
  should write around it as this cycle's own new prose does.

## Next-cycle plan

1. **The `class_feature_no_dedicated_magnitude_id_matched_the_record_slug` D-bucket sub-cause**
   (29 units in `core_rulebook` after this cycle, up from 22) is real, un-named engine-gap work —
   a future AT-34-E3-001 cycle should investigate why these 7 records (and the pre-existing 22)
   carry no dedicated magnitude id the engine can match, and whether a real fix would move them
   to `grounded`/`text-complete` rather than leaving them in `D`.
2. **`owner_matched`'s remaining 237 and `no_explanation_id`'s remaining 357** are unchanged in
   scope from prior cycles' own next-cycle plans beyond the 10-unit movement measured here.
3. **AT-34-E6-001 (final-acceptance scan)** should re-derive this cycle's own figures at HEAD
   rather than trusting this receipt, per its own standing instruction — in particular the 1
   real closure vs. the lane's own claimed 8.
