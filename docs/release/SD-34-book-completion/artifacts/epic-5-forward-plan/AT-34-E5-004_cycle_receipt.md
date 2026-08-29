# Cycle AT-34-E5-004 — Epic 5 (Price the remaining 35 books) / AT-34-E5-004

- **Commit SHA:** PENDING (filled in after commit — see final line of this receipt's history)
- **Files touched:**
  - `docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/build_ordered_plan.py` (new)
  - `docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/verify_ordered_plan.py` (new)
  - `docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/ordered-plan.json` (new, generated)
  - `docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/AT-34-E5-004_cycle_receipt.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md` (updated)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** the raw grep over this epic's whole file-touch set
  (`artifacts/epic-5-forward-plan/`) against the base branch returns hits on `placeholder`, but
  every hit is a **pre-existing** line from `AT-34-E5-002`'s already-committed
  `capability-register.json` and its receipt, quoting corpus-shape vocabulary ("menu
  placeholder", "vacuous_placeholder_row", "vacuous-placeholder shapes") describing content
  shapes, not stub code. Confirmed not introduced by this cycle: the same grep restricted to
  only this cycle's three new files (`build_ordered_plan.py`, `verify_ordered_plan.py`,
  `ordered-plan.json`) returns `OK_NO_TOKENS_NEW_FILES`.
- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "the plan is ordered by real cost,
  cheapest-first, and single-bucket books are flagged. **Evidence:** the forward plan sorted by
  projected cost per book, with the ordering's basis stated. **Books whose remaining work is a
  single bucket are identified by name** — those are the genuine low-hanging fruit, and finding
  `ultimate_campaign` this way is what earned it Epic 4."

## What this cycle built

`build_ordered_plan.py` re-derives `ordered-plan.json` at HEAD, every run, from
`artifacts/epic-5-forward-plan/forward-plan.json` (AT-34-E5-001) — read-only against the rest
of the repo (`workflow-instruction.md §3`). It builds nothing; it orders and flags what
AT-34-E5-001..003 already priced.

**Why "real cost" cannot be one number per book, and the ordering basis this cycle actually
uses.** `forward-plan.json`'s own `measured_rates` establishes that only buckets **A**, **B**
and **U** carry a measured rate that reaches **DONE**. Bucket **C**'s only measured rate reaches
**V**, a different endpoint (core_rulebook's one C-clearing cycle moved 42/42 units to V, zero
to DONE). Buckets **D, M, V, X, Z** carry **no rate at all** — zero dedicated clearing cycles ran
in either vehicle book for any of them. Summing a book's full remaining population into one cost
figure would therefore silently blend priced and unpriced units into a fabricated number — the
exact failure `AGENTS.md` rule 9 and this criterion's own Evidence text (and `AT-34-E5-001`'s
three-tier "never blended" summary) warn against.

This cycle's ordering basis, stated verbatim in the artifact's own `ordering_basis` field:
books are ranked **ascending by the midpoint** of each book's **priced-to-DONE** projected-cost
range — the sum of its bucket-A + bucket-B + bucket-U `projected_cost_hours` (bucket B carries a
measured **range**, per Epic 3/4's own finding that a single blended units/hour figure
materially misrepresents any individual book, so the midpoint is used only for sort order, and
the row keeps the full range). Every ranked row states `priced_to_done_units_pct_of_remaining`
so a low rank is never misread as "this book finishes soonest" — most ranked books have a tiny
priced slice (e.g. `advanced_players_guide`: 1 of 2,956 remaining units priced, 0.0%).

**Books with zero priced-to-DONE units have no real cost to sort by** and are listed separately
in `unrankable_zero_priced_to_done_units`, alphabetically, each naming the unpriced buckets that
make up its entire remaining population.

**Population, cross-checked by the RED→GREEN structural check's own independent
re-derivation from `forward-plan.json`:**

- **35 of 35** non-vehicle books partition exactly into the two lists (verified: no book
  missing, none duplicated, neither vehicle book present).
- **19** books carry ≥1 priced-to-DONE unit → ranked.
- **16** books carry 0 priced-to-DONE units → unrankable, alphabetical.
- Cheapest two (tied, both 1 unit / bucket U / 0.025h): `advanced_players_guide`,
  `inner_sea_taverns`.
- Most expensive of the 19 ranked: `ultimate_equipment` — 1.068h across 44 priced units of
  1,477 remaining (97.0% of that book's remaining population is unpriced, dominated by bucket V).

**Single-bucket books, flagged by name:** exactly **1** of 35 — `beginner_box` (19 units, all
bucket `Z`; zero measured Z-clearing rate exists anywhere in this bundle, so it also lands in
the unrankable list). `single_bucket_books` is deliberately **independent of pricing** — a
single remaining bucket is a book-**shape** property (one mechanism clears the whole book),
not a pricing property, which is exactly how `ultimate_campaign` earned Epic 4 before any rate
existed to price it. Live cross-check in `verify_ordered_plan.py` confirms the stated set
equals the live set of books whose `forward-plan.json` `buckets` dict has exactly one key, in
both directions (no false positive, no false negative).

## RED → GREEN (TDD, `workflow-instruction.md §6` step 3)

**RED #1 — missing artifact, confirmed for the intended reason:**
```
$ python3 verify_ordered_plan.py
FAIL: .../ordered-plan.json does not exist
```
(exit 1, before `build_ordered_plan.py` existed to write it.)

**RED #2 — mutation on the generated artifact, confirmed for the intended reason:** after
building the real artifact, hand-mutated it to set `advanced_players_guide`'s
`priced_to_done_hours_midpoint` to `99999` (an illegal value that both disagrees with the live
re-derivation from `forward-plan.json` AND breaks the ascending-sort invariant against the next
row) and emptied `single_bucket_books` (dropping the one true flag, `beginner_box`):

```
FAIL: 3 violation(s)
 - advanced_players_guide: priced_to_done_hours_midpoint=99999 but live re-derivation says 0.025
 - inner_sea_taverns: not sorted ascending by priced_to_done_hours_midpoint (0.025 follows 99999)
 - single_bucket_books mismatch: stated=[] live=['beginner_box']
```
All three planted defects fired, no crash, no other row spuriously flagged.

**GREEN**, after re-running `build_ordered_plan.py` to discard the mutation (re-derives every
field from `forward-plan.json` at HEAD, discarding the plant):
```
$ python3 verify_ordered_plan.py
PASS: 19 ranked + 16 unrankable = 35 books, sorted ascending by priced_to_done_hours_midpoint, 1 single-bucket book(s) flagged and confirmed live
```

## Figures + their re-derive commands

- `population=49438 buckets=10 unclassified=0 overlap=0` — `python3 scripts/completion_atlas.py --check` (whole-corpus denominator, unaffected by this read-only cycle).
- 35 books / 19 ranked / 16 unrankable — `python3 docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/build_ordered_plan.py`, then read `ordered-plan.json`'s `population` object.
- 1 single-bucket book (`beginner_box`, bucket `Z`, 19 of 19 of that book's remaining units) — same artifact, `.single_bucket_books`, of the 35-book population.
- Cheapest ranked (tied): `advanced_players_guide` and `inner_sea_taverns`, both 1 priced unit / 0.025h midpoint — same artifact, `.ranked_by_priced_to_done_cost[0:2]`.
- Priciest ranked: `ultimate_equipment`, 44 priced units of 1,477 remaining (2.98% priced), 1.068h midpoint — same artifact, `.ranked_by_priced_to_done_cost[-1]`.
- Denominator gate against this package: `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` → `files_checked=15 violations=4` — all 4 pre-existing in `progress.md` (verbatim-quoted corpus prose, "75% chance..."), already flagged and attributed by the already-merged `AT-34-E3-004` cycle (confirmed by line numbers 260, 303, 360, 366, unchanged from that cycle's own receipt); this cycle's new files are `.py`/`.json` only and its `progress.md`/`kanban.md` edits contain no bare percentage.

## Row-count command output

```
$ python3 docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/verify_ordered_plan.py
PASS: 19 ranked + 16 unrankable = 35 books, sorted ascending by priced_to_done_hours_midpoint, 1 single-bucket book(s) flagged and confirmed live
```

## Build scope verified

`cargo test --locked --no-run` — workspace: **exit 0**, all targets (lib, bins, integration
test binaries) compiled clean, run at this cycle's commit SHA (see top of this receipt).
`apps/desktop/src-tauri` — **separate cargo workspace**, run explicitly: `cargo test --locked
--no-run` — **exit 0**. No Rust source touched by this cycle; both runs verify the tree is
still clean after this cycle's Python/JSON-only change, with no later commit in this cycle
regenerating anything the build depends on.

## Sweep population

N/A — this cycle added no corpus records and regenerated none. `corpus_literal_sweep`'s
examined-population is unaffected (`git status --porcelain -- data/corpus/` shows no diff for
this cycle).

## Oracle pin

N/A — no figure in this cycle's artifact came from the pinned PCGen oracle corpus; every figure
is drawn from `forward-plan.json`, itself drawn from `docs/work-inventory.json` and prior
SD-34 artifacts.

- **Status:** complete

## Movement, four buckets

**None.** This cycle moves no unit on any bucket board — `docs/work-inventory.json` is
untouched (confirmed by `git status --porcelain -- docs/work-inventory.json` showing no diff).
It is a pure **ordering/flagging** artifact over an already-priced plan (AT-34-E5-001..003),
matching the criterion's own bar exactly: no closure, no reclassification, no reachability
change, no instrument-correction.

## Notes

- The 16 "unrankable" books are not a failure of this cycle's method — they are an honest
  report of where SD-34's own measured rates stop. A future bundle that runs a dedicated
  D-, M-, V-, X- or Z-clearing cycle in any book would immediately promote some fraction of
  those 16 books' units into the priced-to-DONE ranking; this artifact's `re_derive_command`
  re-runs cleanly against `forward-plan.json` at that point with zero code change needed here.
- `beginner_box` appearing in **both** `single_bucket_books` and
  `unrankable_zero_priced_to_done_units` is intentional, not a contradiction: shape (single
  bucket) and price (a rate exists) are independent properties, and the criterion's own Evidence
  text ties "low-hanging fruit" to shape (via the `ultimate_campaign` precedent), not to an
  existing rate.
- The B-bucket rate range (1.667–5.8 units/hour) is `AT-34-E5-001`'s own explicitly
  thin-sample figure (2 measurements, one of them n=1 cycle/5 units); this cycle's ranking
  inherits that caveat unmodified rather than collapsing it, per `decisions.md §12` L2 (never
  carry a number forward without its own caveat).

## Next-cycle plan

Epic 5 is now complete — `AT-34-E5-001`, `AT-34-E5-002`, `AT-34-E5-003`, `AT-34-E5-004` are all
`complete` in `kanban.md`. The next gated work is Epic 6's closure epilogue
(`AT-34-E6-001`, final-acceptance scan), which is gated on Epics 1–5 all `complete`. That scan
should re-derive this cycle's `single_bucket_books` and `ranked_by_priced_to_done_cost` counts
independently rather than trust this receipt, per its own obligation #2 ("re-run every headline
command yourself").
