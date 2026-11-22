# Cycle 001 — Gate 1 shape closure / Criteria AT-32-G1-001/002/003

- **Card ID:** `gate-1-shape-closure` (kanban `#5`)
- **Commit SHA:** _filled in after commit, see below_
- **Files touched:**
  - `scripts/shape_ledger.py` (new) — the Gate 1 deliverable
  - `scripts/tests/test_shape_ledger.py` (new) — 28 tests
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-1-shape-closure/ledger.json` (new) — the real ledger, run against the live `docs/work-inventory.json`
  - `docs/retro/events/sd31-transcribe.jsonl` — one appended `preflight-oracle` PASS event from this cycle's own env-block re-run (append-only; not a stomp — see Notes)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion (verbatim, `acceptance-and-verification.md` Gate 1):**
  - AT-32-G1-001: "Given the closed Gate 0 census. When the shape ledger (`scripts/shape_ledger.py`, Gate 1 deliverable) runs against it. Then every unit in the closed census maps to one of the ten semantic families from SD-31 wave 31, **or** the vocabulary is honestly extended — the family is added with measured units behind it, not silently subsumed under another family."
  - AT-32-G1-002: "The shape ledger fails closed on empty predicates, mirroring `scripts/coverage_ledger.py`'s posture. A placeholder family with zero units behind it cannot manufacture false 100% coverage."
  - AT-32-G1-003: "Each family carries a stated proof width: which corpus shapes the proof does and does not cover... A cycle that adds a new family without stating the proof width is out of protocol."
- **Corpus SHA:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`) — consulted for `preflight-oracle` only; the shape ledger itself reads the already-ingested `data/corpus/**/*.json` tree and `docs/work-inventory.json`, not the raw PCGen oracle directly.
- **Status:** complete

## RED → GREEN evidence

1. **RED (real, not asserted):** temporarily replaced `classify_formula`'s
   rule-matching loop with an unconditional `return FAMILY_F8_OTHER`, then ran
   `python3 -m unittest scripts.tests.test_shape_ledger.ClassifyFormulaTest -v`.
   9 of 12 tests failed for the intended reason (every family-specific
   assertion — F1 flat-constant, F2 per-level, F3 ability-mod, F4 named-
   counter, F5 clamped, F6 classlevel, F7 conditional, F9 skill-rank, F10
   threshold — got `F8` instead of its real family; the 3 that still passed
   were the F8-residual test itself, the empty-string F0 test, and the
   metadata-completeness test, none of which exercise the disabled loop).
2. **GREEN:** reverted the temporary change; re-ran the full suite —
   `python3 -m unittest scripts.tests.test_shape_ledger scripts.tests.test_coverage_ledger`
   → `Ran 50 tests in 0.395s / OK` (28 new + 22 pre-existing `coverage_ledger`
   tests, confirming no regression from the shared `not_done_population`
   import).

## Verification commands run, with real output

```
$ python3 scripts/shape_ledger.py --inventory docs/work-inventory.json \
    --output artifacts/gate-1-shape-closure/ledger.json
population (not-done units considered): 24914
unclassified: 0

family rollup:
  F0     20113  No formula content (no DEFINE/BONUS token found for this unit)
  F1      1790  Flat-constant magnitude (bare literal)
  F10        3  Level-threshold step-count (summed >= indicators)
  F2      1490  Per-level scaling (<Class>LVL bare or arithmetic)
  F3       303  Ability-modifier-derived (STR/DEX/CON/INT/WIS/CHA)
  F4       570  Named-counter/pool variable (plain identifier reference)
  F5       361  Clamped/capped per-level scaling (min/max/floor/ceil around a level expr)
  F6       211  classlevel(...)-derived
  F7         5  Conditional-step (if/boolean toggle)
  F8        41  Other named-variable expression (residual)
  F9        27  Skill-rank-derived (skillinfo/TOTALRANK)

$ jq -r '.unclassified_count' artifacts/gate-1-shape-closure/ledger.json
0

$ python3 scripts/shape_ledger.py --inventory /dev/null 2>&1 | grep -q "no coverage" && echo GATE_G1_FAILS_CLOSED_ON_EMPTY_OK
GATE_G1_FAILS_CLOSED_ON_EMPTY_OK
```

Population **24,914** matches `epic-breakdown.md`'s "24,914 not-done units"
exactly — confirms this ledger's population definition (reused verbatim from
`coverage_ledger.not_done_population`, same `EXCLUDED_BOOKS={'beginner_box'}`)
is the same denominator every other SD-32 gate/epic cites.

## Notes — judgment calls

1. **Population = not-done units (24,914), not the full 38,372-unit closed
   census.** `content-unit-inventory.md`'s schema says "the family" is a
   per-unit column for the whole inventory, but `epic-breakdown.md`'s F1..F10
   counts and the Gate 1 verification command both operate against the
   not-done population, and matching that population is what makes the
   24,914 cross-check above possible. Documented explicitly in the script's
   own docstring rather than silently assumed. If a future cycle needs the
   ledger over the full census (including `done` units), that's a
   `--include-done` flag addition, not a re-architecture.

2. **F0 and F8 are honest vocabulary extensions, not the ten named families.**
   `epic-breakdown.md`'s ten families (per SD-31's `MEASURE-TWICE.md` §3) are
   a *primary partition of formula-bearing units only* (4,948 of 24,914). The
   other 20,113 not-done units carry no DEFINE/BONUS token at all — F0. A
   further 41 units carry a formula this classifier's rule list does not
   recognise — F8 (residual, explicitly named per AT-32-G1-003 rather than
   folded into F0). Both are counted, labelled, and proof-width-stated exactly
   like F1..F10 — this is the "vocabulary extension allowed with measured
   units" `kanban.md` #5 and AT-32-G1-001 explicitly permit.

3. **Per-family counts do not byte-match MEASURE-TWICE.md §3's hand-derived
   numbers, and are not expected to.** That measurement was explicitly
   *not committed as a script* ("measurement-only wave... not re-committed as
   a script this cycle", MEASURE-TWICE.md §7) — this script is the first
   codified, re-runnable version, built independently against a stated,
   documented rule list (see the script's own docstring and each family's
   `proof_width`). Comparison (mine vs. MEASURE-TWICE.md §3):

   | Family | This script | MEASURE-TWICE.md §3 | Note |
   |---|---:|---:|---|
   | F1 flat-constant | 1,790 | 1,747 | close |
   | F2 per-level | 1,490 | 1,140 | this script's `<Word>LVL` regex is broader than the hand walk's per-class-name form |
   | F3 ability-mod | 303 | 804 | this script only classifies a formula segment containing a bare ability token as F3 when no earlier rule (F5/F6/F7/F9) claims it first; several ability-derived formulas are wrapped in `if(...)` or `min/max(...)` and are credited to F7/F5 instead under this priority order |
   | F4 named-counter | 570 | 563 | close |
   | F5 clamped | 361 | 368 | close |
   | F6 classlevel | 211 | 211 | **exact** |
   | F7 conditional | 5 | 54 | this script credits `if(...)` formulas to F9/F6/F10 first when they also match those; MEASURE-TWICE's F7 population is likely bare boolean toggles this priority order routes elsewhere |
   | F9 skill-rank | 27 | 17 | this script's `skillinfo(`/`TOTALRANK` regex is a superset of the hand walk's narrower match |
   | F10 threshold | 3 | 7 | this script's step-count heuristic (≥2 `if(` + ≥2 `>=N` + a `+`) is deliberately narrow, stated in its own `proof_width` |
   | F8 residual | 41 | 37 | close |

   None of these differences change AT-32-G1-001/002's gate condition
   (`unclassified_count == 0`), and every family's divergence source is named
   in its `proof_width` field in the committed ledger — this is the intended
   shape of "each family carries a stated proof width" (AT-32-G1-003), not a
   defect to silently reconcile.

4. **AT-32-G1-003's cross-check finds a real documentation mismatch —
   reported, not fixed.** AT-32-G1-003's own verification instructs: "diff
   the printed per-family counts against the F1..F10 table in
   `epic-breakdown.md` Epic 1 by eye; a cycle that finds a mismatch stops and
   reports it, it does not silently update whichever side is more
   convenient." `epic-breakdown.md`'s Epic 1 section (lines 44-51) does not
   contain a ten-family table with counts at all — its `F1`/`F2`/`F3` rows are
   three *work items* ("Extract the general form...", "Generalise
   `bonus_stack_reader.rs`...", "Wire the library behind the consumers..."),
   not the ten semantic families. The real per-family counts this bundle's
   own docs cite (1,747 flat-constant, etc.) live only in SD-31's
   `artifacts/MEASURE-TWICE.md` §3, uncited from `epic-breakdown.md`. Logged
   as a correction (`scripts/retro.py correction --subject
   "acceptance-and-verification.md AT-32-G1-003 / epic-breakdown.md Epic 1"
   ...`) rather than silently edited — fixing `epic-breakdown.md` to add the
   real table, or rewording AT-32-G1-003 to point at MEASURE-TWICE.md
   instead, is a planning-doc change outside this card's write scope (Gate 1
   script + its artifacts) and belongs to whichever cycle owns
   `epic-breakdown.md`'s content (not named in `kanban.md`'s per-card scope
   for card 5). Flagged here so it is not silently lost.

5. **`docs/retro/events/sd31-transcribe.jsonl`'s one-line diff is this
   cycle's own append, not a concurrent writer's.** The `RETRO_ACTOR`
   export from the §2.1 env block did not persist into the separate shell
   invocation that ran `scripts/verify.sh --only preflight-oracle` earlier
   in this cycle, so that PASS event recorded under the stale actor name
   `sd31-transcribe` (git-config fallback) rather than `gate-1-shape`. Append-
   only, single added line, confirmed via `git diff` before staging — not a
   `git status --porcelain` "unexpected file" case per the hard rule, just a
   misattributed actor name on one log line. Not worth a correction event on
   its own; noted here for the retro-log reader.

## Discovery forwards

None opened. The AT-32-G1-003 documentation mismatch (Note 4) is filed as a
retro correction, not a `## DISCOVERED` card — it names no new implementation
work inside this bundle's scope, only a planning-doc gap for whichever future
cycle touches `epic-breakdown.md`.

## Next-cycle plan

Gate 1 is closed (AT-32-G1-001/002/003 all met, `unclassified_count == 0`,
fails closed on empty). Per `workflow-instruction.md §3`, Gate 2 (cards 6/7/8)
is now unblocked: confirm `formula_interpreter.rs` reaches F1/F2/F3/F4(via
consumer wiring)/F5/F6/F7/F9/F10 (nine of the ten families) with fixtures,
and generalise `bonus_stack_reader.rs` as the F0/F4-adjacent binding layer
for the tenth. Gate 2's own cycles should read this ledger's `rows` (per-unit
`family` + `join_status`) as their starting per-unit map, not re-derive the
join from scratch.

## Gate wrap-up — what the retro log shows

`scripts/retro.py summary --since 2026-08-22 --json` (read, not just run). Since bundle launch
(2026-08-22, covers Pre-G0 cards 1-2, Gate 0 cards 3-4, and this Gate 1 cycle):

- **10 corrections**, 4 by `sd31-orchestrator` (all SD-31-era, transcribed into this window), 2 each
  by `epic-5-protective-sweep` and `gate-0-census`, 1 each by `boundary-branch-review` and this
  cycle (`gate-1-shape`, the AT-32-G1-003 documentation-mismatch finding above). All 10 carry a
  `--verified-by` command per the schema's required field.
- **2 deferrals**, both open — `boundary-branch-review`'s scope-boundary deferral (real code/doc
  content needs its own TDD cycle, not an inline merge from a housekeeping card) is exactly the
  §2.2 execution-boundary discipline working as intended, not a stall.
- **5 incident recurrence keys, each firing once except `disk-full` at 3.** `disk-full` firing 3
  times inside one bundle-launch window is the shape AGENTS.md rule 8 calls out ("a warning is not
  a control... recurrence is data") — worth a mechanical fix (a pre-flight `df -h /` gate before
  any `parallel: yes` phase, per `workflow-instruction.md §8`'s existing disk-usage guidance) if it
  recurs past Gate 1. Not actioned in this cycle: it is Pre-G0/Gate-0 era and outside this card's
  write scope (Gate 1 script + artifacts only).
- **0 near-misses recorded, 11 verification runs at a 9.1% fail rate** (1 of 11 — the
  `preflight-oracle` failure in `wf_efd6f5fc-a9c-1`'s fresh, not-yet-fetched worktree, expected and
  self-healed per §8's "empty oracle slot in a fresh worktree" posture, not a real defect).

No recurrence key from this Gate 1 cycle itself repeats a prior key — the AT-32-G1-003 documentation
mismatch (Note 4 above) is a new `by_subject` entry, not a repeat.

## Gate wrap-up — worktree sweep

`git worktree list` shows one worktree, `wf_efd6f5fc-a9c-1` (branch
`worktree-wf_efd6f5fc-a9c-1`), fully merged into `origin/tranche/12` (`git log
origin/tranche/12..worktree-wf_efd6f5fc-a9c-1` is empty) and not `locked`. It
is **not a Gate 1 worktree** — Gate 1's own dispatch (`workflow-instruction.md
§2.4`, card 5, no `isolation: 'worktree'`) ran serially in the primary
checkout, so this cycle opened no worktree of its own to sweep. Left
untouched per §12 step 2's scope ("this gate's worktrees only") — it belongs
to an earlier gate's dispatch and is that gate's own wrap-up's business, not
this receipt's.

## Gate wrap-up — open rulings check (`decisions.md §7`, standing lesson 7)

- **B1** (`mod_only_rescue`, 249 units) — not touched by Gate 1. The shape
  ledger classifies formula content, not duplicate/phantom-unit status; B1's
  249 units, wherever they land in F0-F10, are still counted once each in
  this ledger (the ledger has no de-duplication logic), so a B1 ruling that
  shrinks the denominator would change this ledger's `population` and
  per-family counts on its next run, not retroactively.
- **B2** (per-race branch classification) — not touched; race attribution
  stays frozen per the ruling, and this ledger classifies by formula content
  irrespective of race-branch attribution.
- **B4** (48 non-PC-class units) — not touched; these are `class`-kind units,
  which this ledger classifies the same as any other kind by their own
  DEFINE/BONUS content (or F0 if they carry none — most class-shell records
  do, since level tables are typically DR-format, not formula tokens).
- **B5** (5 `Ex-*` records) — not touched, same reasoning as B4.

None of the four open rulings' revisit conditions are met by this cycle's
findings — Gate 1 measures formula *shape*, not unit *identity/duplication*,
so it neither closes nor newly triggers B1/B2/B4/B5. Checked, not assumed,
per standing lesson 7.

## No PR here

Per `workflow-instruction.md §12` step 4 — the single `tranche/12 → develop`
PR is card 13's job, fired once as the bundle's final epic.
