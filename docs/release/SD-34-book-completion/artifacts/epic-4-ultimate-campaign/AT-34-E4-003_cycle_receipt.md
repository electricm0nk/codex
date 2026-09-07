# Cycle — Epic 4 (Ultimate Campaign to zero) / AT-34-E4-003

- **Commit SHA:** `16ab9ce58d`
- **Files touched:**
  - `docs/release/SD-34-book-completion/artifacts/epic-4-ultimate-campaign/step-cost-ledger.json` (new) — the criterion's own evidence artifact.
  - `docs/release/SD-34-book-completion/artifacts/epic-4-ultimate-campaign/step-cost-ledger-raw-commits.json` (new) — the commits the ledger derives from, committed for reproducibility.
  - `docs/release/SD-34-book-completion/artifacts/epic-4-ultimate-campaign/AT-34-E4-003_cycle_receipt.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — checked directly against this cycle's own two
  new files (`grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' step-cost-ledger.json
  step-cost-ledger-raw-commits.json` → no matches). The full epic-scoped diff against
  `merge-base(HEAD, origin/develop)` (`ea2b3396f2`) carries thousands of pre-existing
  `sd34-...`-shaped receipt-quote matches from earlier, already-committed E1/E2/E3/E4 cycles —
  none introduced by this cycle.

- **Wired-integration audit result:** `OK_NO_TOKENS` — checked directly against this cycle's own
  two new files (same grep, no matches). The full epic-scoped diff carries pre-existing
  `placeholder` matches, all verbatim quotes of already-committed PCGen corpus-data prose
  (`docs/work-inventory.json`'s `vacuous_placeholder_row_no_corpus_content_to_render` evidence
  strings and `AT-34-E3-001`'s already-audited "no selection" commit language) — a legitimate
  corpus-data/prior-cycle term, not a stub token, and not introduced by this cycle.

- **Acceptance criterion (verbatim, `epic-breakdown.md` AT-34-E4-003):** "a second, independent
  cost measurement is recorded. The Core Rulebook is deep and many-bucketed; Ultimate Campaign is
  shallow and single-bucketed. Two books of opposite shape give Epic 5 a range rather than one
  blended number. **Evidence:** `artifacts/epic-4-ultimate-campaign/step-cost-ledger.json`, and a
  stated comparison against Epic 3's rates: where they agree, where they diverge, and which
  divergences are explained by book shape rather than by noise. A projection built on a single
  book's rate says so."

## What this cycle did

1. Re-derived every commit touching `docs/work-inventory.json` since the `tranche/14` cut
   (`ea2b3396f2`): `git log --reverse --format='%H|%ct|%s' ea2b3396f2..HEAD -- docs/work-inventory.json`.
   Exactly **one** commit in that log is tagged Epic 4: `4005925ae2` (`AT-34-E4-002`, bucket B).
   `AT-34-E4-001`'s two commits (`72c9f6fec6`, `68e9b353b9`) do **not** touch
   `docs/work-inventory.json` — its own already-committed receipt states plainly "No bucket
   movement (`U:21 X:2` unchanged); resolution is by proof per the criterion's own Evidence
   clause." A cycle moving zero units in the ledger's own denominator correctly contributes no
   rate row, not a silently-folded one.
2. Measured `4005925ae2`'s wall time as the commit-to-commit span from the immediately prior
   `docs/work-inventory.json`-touching commit (`36db23a053`, an Epic 3 cycle — the two epics
   share one linear commit log against this file): `1787977847 - 1787971367 = 6480s = 108.0
   minutes`, same upper-bound caveat as Epic 3's ledger (span includes queue/dispatch overhead,
   here also the intervening AT-34-E4-001 cycle's own real work).
3. Partitioned `docs/work-inventory.json` before/after that commit with the live
   `completion_atlas.partition(units, book='ultimate_campaign')` — the same code the standing
   gate runs — confirming bucket B: 5→0 (3 reached DONE, 1 reclassified to M, 1 to D), matching
   `AT-34-E4-002`'s own already-committed receipt exactly.
4. Built the required comparison against Epic 3's bucket-B rate (`artifacts/epic-3-core-rulebook/step-cost-ledger.json`),
   stating agreement, divergence, and which divergence is shape-explained vs. noise-explained —
   see Figures below and the ledger's own `comparison_against_epic_3_rates` field.

This is a **measurement-only** cycle, same class as `AT-34-E3-004`: it moves no unit's status in
`docs/work-inventory.json` and ships no production code.

## Figures + their re-derive commands

- **Epic 4 bucket B: 1 cycle, 108.0 wall-minutes (1.8 hours), 5→0 units (`ultimate_campaign`),
  3 reached DONE, 2 reclassified (1 M, 1 D). `units_per_hour_reaching_DONE` = 1.667.**
  Denominator: `ultimate_campaign`, 265 units total. Re-derive:
  `git log --reverse --format='%H|%ct|%s' ea2b3396f2..HEAD -- docs/work-inventory.json`
  (commit list, cross-check against `step-cost-ledger-raw-commits.json`'s literal timestamps);
  `python3 scripts/completion_atlas.py --book ultimate_campaign --check` at HEAD confirms current
  `DONE=130 B=0 D=5 M=89 V=18 U=21 X=2` (population=265, unclassified=0, overlap=0) — matches this
  cycle's `atlas_after` field exactly (re-run this cycle, not carried forward, per
  `decisions.md §12` L2).
- **Epic 3 bucket B (comparator, re-quoted with its own source, not re-derived here):** 29
  cycles, 2432.3 wall-minutes, 970→~532 (`core_rulebook`), 235 DONE, blended
  `units_per_hour_reaching_DONE` = 5.8, per-mechanism range 22.2–617.4 minutes/cycle (28x spread).
  Source: `artifacts/epic-3-core-rulebook/step-cost-ledger.json`, unchanged by this cycle.
- **Where they agree:** Epic 4's single mechanism (108.0 min for 5 units, 21.6 min/unit-touched)
  sits squarely inside Epic 3's own per-mechanism spread (22.2–617.4 min/cycle) — not an outlier
  against the population of mechanisms Epic 3 measured. The nearest size- and kind-matched Epic 3
  comparator, `AT-34-E3-001 domain mechanism` (1 unit, 40.3 min, a PI-coordinate-adjacent
  reattribution fix), lands in the same 1–2-units/hour-DONE neighborhood once its own mixed
  DONE/non-DONE destination is normalized the same way.
- **Where they diverge:** Epic 4's blended rate (1.667 units/hour DONE) is ~3.5x slower than
  Epic 3's blended bucket-B rate (5.8).
- **Which divergence is shape-explained vs. noise:** the 3.5x gap is **noise, not book shape** —
  stated explicitly per the criterion's own instruction ("A projection built on a single book's
  rate says so"). Epic 3's 5.8 is a blend across 29 cycles/503 units smoothing over its own 28x
  per-mechanism spread; Epic 4's 1.667 is one cycle, one mechanism, 5 units — exactly the
  small-*n* shape `decisions.md §12` L11 (measure per-unit cost before a population-scoped run)
  warns cannot be extrapolated as a book-wide rate. The one claim this ledger **can** make on
  shape, not noise: Ultimate Campaign's bucket B needed exactly **one** mechanism to reach zero
  where Core Rulebook's needed **29** — consistent with "shallow and single-bucketed" for
  mechanism/bucket count, not (yet, on n=1) for per-unit wall-time cost.
- **Headline finding for Epic 5, stated in the ledger itself:** do not price Ultimate Campaign's
  remaining M/D/V buckets off this single bucket-B data point — M/D/V are compute-path /
  oracle-harness mechanisms, structurally different from bucket B's lookup-wiring mechanism
  measured here. Epic 5 should wait for at least one M/D/V-clearing cycle (either book) before
  pricing those buckets.

## Row-count command output (this cycle's own artifact)

```
$ python3 -c "
import json
d = json.load(open('docs/release/SD-34-book-completion/artifacts/epic-4-ultimate-campaign/step-cost-ledger.json'))
cleared = d['buckets_cleared_so_far']
not_cleared = d['buckets_not_yet_cleared']
print('buckets_cleared_so_far:', sorted(cleared.keys()), 'count=', len(cleared))
print('buckets_not_yet_cleared:', sorted(not_cleared.keys()), 'count=', len(not_cleared))
for k, v in cleared.items():
    assert 'wall_minutes_total' in v and 'cycles' in v and 'dominant_mechanism' in v, k
assert 'comparison_against_epic_3_rates' in d
print('SCHEMA_OK: every cleared bucket carries cycles + wall_minutes_total + dominant_mechanism')
print('COMPARISON_PRESENT')
"
buckets_cleared_so_far: ['B'] count= 1
buckets_not_yet_cleared: ['D', 'M', 'U', 'V', 'X'] count= 5
SCHEMA_OK: every cleared bucket carries cycles + wall_minutes_total + dominant_mechanism
COMPARISON_PRESENT
```

1 of Ultimate Campaign's 5 non-DONE buckets (B) carries a real cost entry (the only one closed by
a dedicated clearing cycle so far); the other 4 (D, M, V, X) are named with current population and
reason no entry exists yet — U and X are the distinct "resolved by proof, zero units moved"
class (`AT-34-E4-001`), separately noted rather than forced into a rate.

## Build scope verified

- `cargo test --locked --no-run` — **EXIT=0**, run at this cycle's HEAD `c2805717af` (`tranche/14`
  HEAD at cycle start; this cycle adds only new JSON/doc files, no Rust source). Full workspace
  target list built and linked cleanly.
- `cargo test --locked --lib` / `apps/desktop/src-tauri cargo test --locked`: **not run this
  cycle** — this cycle touches zero Rust source, zero `Cargo.toml`, zero corpus files; it adds two
  new JSON documents and edits `progress.md`/`kanban.md`. No figure this cycle's assertions depend
  on comes from compiled code beyond what `--no-run`'s clean link already establishes (the
  `completion_atlas.py --book ultimate_campaign --check` figures quoted above are re-derived live
  above, independent of any Rust build). Scoped out per §2.5 ("say which sweeps you did not run"),
  same precedent as `AT-34-E3-004`.

## Sweep population

N/A — this cycle adds or regenerates no corpus records. `docs/work-inventory.json` is read-only
this cycle (checked out historically at a prior SHA for measurement, never written).

## Oracle pin

N/A — no figure in this receipt derives from the pinned PCGen corpus.

- **Status:** complete
- **Movement, four buckets:** instrument-correction only. This cycle produces a **measurement**
  artifact; it moves no unit's status in `docs/work-inventory.json`. Within the measured data
  itself (already landed by `AT-34-E4-002`, re-confirmed here): bucket B's 5-unit reduction splits
  into 3 closure (reached DONE) + 2 reclassification (moved to M, D).

## Notes — judgment calls

1. **Only one bucket has a real cost entry.** Epic 4 has run exactly one bucket-clearing cycle
   (`AT-34-E4-002`, bucket B) since the `tranche/14` cut; `AT-34-E4-001` resolved U/X by proof
   without moving units, a legitimate zero-unit-moved deliverable (`decisions.md §9`) reported
   separately, not folded into a rate. This satisfies the criterion's Evidence clause ("a second,
   independent cost measurement is recorded") — it does not require every bucket to be cleared,
   only that a real, independent measurement exists alongside Epic 3's.
2. **The comparison explicitly refuses to over-claim book-shape causation from n=1.** The
   criterion's own text instructs stating which divergences are explained by shape vs. noise; this
   receipt and the ledger both state the 3.5x rate gap is noise (sample size), not shape, while
   still naming the one shape-linked fact the data supports (mechanism count: 1 vs 29).
3. **No TDD RED→GREEN pair applies to this cycle.** Same precedent as `AT-34-E3-004`: this cycle
   ships a measurement/report artifact, not shipping code — inventing a script+test harness to
   validate a static JSON's schema would be scope expansion beyond this criterion or its Epic 3
   precedent. The row-count command above (with its inline `assert`s) is the concrete,
   re-runnable verification in its place.

## Next-cycle plan

- The first M-, D-, or V-clearing cycle (either book) should append its bucket to whichever
  epic's ledger it belongs to; once at least one exists, Epic 5 can price those buckets from a
  real per-mechanism-type rate instead of extrapolating from bucket B alone (this ledger's own
  `headline_finding_for_epic_5`).
- Epic 4's remaining work (`AT-34-E4-002`'s named remainder: `M:89, D:5, V:18`) continues under
  its own criterion; this criterion (`AT-34-E4-003`) does not require that work to be picked up
  here.
