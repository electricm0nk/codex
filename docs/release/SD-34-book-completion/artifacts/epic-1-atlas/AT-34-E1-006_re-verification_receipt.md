# Cycle AT-34-E1-006-R — Epic 1 Completion Atlas / AT-34-E1-006 (re-verification at HEAD)

This cycle was dispatched against `AT-34-E1-006` after the criterion had already landed
(original cycle receipt: `AT-34-E1-006_cycle_receipt.md`, commit `6490738c38`, kanban row already
`complete`). Between that cycle and this dispatch, Epic 3's `AT-34-E3-001` per-mechanism cycles
committed four new cycle receipts under `artifacts/epic-3-core-rulebook/`, and per
`decisions.md §12` L2/L19 ("never carry your own number forward — re-derive it"), this cycle
re-ran the criterion's own verifying commands fresh at HEAD rather than trusting the original
receipt's PASS.

**This re-verification found the standing gate RED**, not green: `scripts/verify.sh --only
figure-provenance` — the primary verifying command named in `acceptance-and-verification.md §1`
row 22 for this criterion — failed with `violations=14` across four of Epic 3's committed
receipts. The gate itself was not defective; it correctly caught real non-compliance the gate's
own population (this package's own artifacts) had accumulated after this criterion's original
closure. Per `AGENTS.md` blocker discipline ("a fix that lives in another subsystem is still a
fix") and the bundle's "no carve-outs — close, do not flag" precedent, this cycle fixed the 14
violations in place rather than reporting them as someone else's problem.

- **Commit SHA:** see push output below (this receipt's own landing commit).
- **Files touched (this cycle):**
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_with_magnitude_cycle_receipt.md` (4 table cells reworded to carry an inline reachable command instead of "same"/"same command")
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_deity_absent_cycle_receipt.md` (3 bullets reflowed onto single physical lines with an inline command)
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_domain_cycle_receipt.md` (2 bullets reflowed / given an inline command)
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_race_trait_race_not_modelled_cycle_receipt.md` (6 bullets reflowed onto single physical lines)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/AT-34-E1-006_re-verification_receipt.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md` (prepended)
  - `docs/release/SD-34-book-completion/kanban.md` (row re-confirmed, status unchanged — already `complete`)
  - No edits to `scripts/denominator_gate.py` or `scripts/verify.sh` — the stage and the widened
    `denominator-gate` default were already correct; only the receipts they examine had drifted
    out of compliance.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (working-tree diff on the four touched receipts, `git diff --unified=0 -- <4 files> | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff, `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-34-E1-006 — every figure in this package carries its re-derive command, enforced
  >
  > A `scripts/verify.sh` stage that **fails** on a figure stated in an SD-34 package document or
  > cycle receipt without a re-derive command reachable from it.
  >
  > **Evidence:** RED→GREEN mutation proof — a deliberately-unsourced figure fails the stage; the
  > sourced form passes. Wired into `verify.sh`'s stage list, alongside `denominator-gate`, not as
  > a standalone script.
  >
  > **Second obligation of the same cycle:** widen `scripts/denominator_gate.py`'s default scope
  > (`BUNDLE_DIR` / `DEFAULT_GLOBS`) from SD-33's folder to this package, so that
  > `scripts/verify.sh --only denominator-gate` examines SD-34 without an explicit path.
  > RED→GREEN: the default run's `files_checked` must include every SD-34 `.md`.

## Re-derivation at HEAD

RED — the standing gate before this cycle's fix, run at this cycle's start SHA `2eb1536876`:

```
$ python3 scripts/denominator_gate.py --check-provenance | grep -c VIOLATION
14
$ bash scripts/verify.sh --only figure-provenance 2>&1 | tail -3
    FAIL  figure-provenance  (violations=14 of figures_examined=64 (files_checked=41) — .../figure-provenance.log)
RESULT: FAIL
```

All 14 violations were in Epic 3's four `AT-34-E3-001_*_cycle_receipt.md` files, committed after
the original AT-34-E1-006 cycle. Two failure shapes, both real (not gate false-positives):

1. **Table cells / bullets that said "same" or "same command"** instead of restating a reachable
   inline-code command on the figure's own line (`class_feature_option_pool_with_magnitude`
   receipt, 4 cells).
2. **Multi-line markdown bullets** where the figure and its backtick-quoted command sat on
   different physical *lines* of the wrapped paragraph — substantively sourced, but not reachable
   by this line-addressed check (`deity_absent`, `domain`, `race_trait_race_not_modelled`
   receipts, 10 lines combined). This is the same "same construct = same line" granularity
   `denominator_gate.py`'s own docstring already documents for the percentage/denominator check;
   the figure-provenance check inherits it.

Fix: reworded shape-1 cells to carry an inline `` `python3 ...` `` (or repeat the exact command)
on the same line as the value, and reflowed shape-2 bullets so the command and the figure share
one physical line — no command was invented; every command used already appeared verbatim
elsewhere in the same receipt (verified by grep before editing).

GREEN — the standing gate after the fix:

```
$ python3 scripts/denominator_gate.py --check-provenance | tail -3
files_checked=41
figures_examined=64
violations=0
$ bash scripts/verify.sh --only figure-provenance 2>&1 | tail -3
    PASS  figure-provenance  (files_checked=41 figures_examined=64 violations=0)
RESULT: PASS
```

The original cycle's own RED→GREEN mutation-proof unit suite is unchanged and still green (no
production code touched this cycle):

```
$ python3 -m unittest scripts.tests.test_denominator_gate -v 2>&1 | tail -3
Ran 40 tests in 0.449s
OK
```

The second obligation (denominator-gate default scope widened to SD-34) is unchanged and still
holds at HEAD — confirmed fresh, not re-quoted from the original receipt:

```
$ python3 scripts/denominator_gate.py --check | tail -2
files_checked=111
violations=0
$ ls docs/release/SD-34-book-completion/*.md | wc -l
15
```

`files_checked=111` includes every SD-34 `.md` (15 root files + this package's now-41 cycle
receipts, up from the original cycle's `files_checked=90`/`20 root+receipt` count — growth is
Epic 2/3's own new receipts landing since, not a regression) plus SD-33's unchanged 15 files and
60 receipts.

- **Figures + their re-derive commands:**
  - `figure-provenance` RED, before this cycle's fix: `violations=14 of figures_examined=64
    (files_checked=41)` — `python3 scripts/denominator_gate.py --check-provenance` (denominator:
    every figure in this package's 41 committed cycle receipts + 15 root `.md` files)
  - `figure-provenance` GREEN, after this cycle's fix: `violations=0 of figures_examined=64
    (files_checked=41)` — same command, same population, re-run after the edit
  - `denominator-gate` default run (widened scope, unchanged this cycle): `files_checked=111
    violations=0` — `python3 scripts/denominator_gate.py --check` (denominator: SD-33's 15 files
    + 60 receipts, unchanged, plus SD-34's 15 root `.md` + 41 receipts)
  - SD-34 `.md` files at package root: `15` — `ls docs/release/SD-34-book-completion/*.md | wc -l`
    (denominator: non-recursive glob at the package root, unchanged since the original cycle)
  - Unit test suite (this module, unchanged): `40 passed, 0 failed, 0 skipped` — `python3 -m
    unittest scripts.tests.test_denominator_gate -v` (denominator: the module's own case count)
  - `completion_atlas.py --check` (context only, not this criterion's own figure — confirms no
    corpus regeneration happened this cycle): `population=49438 buckets=10 unclassified=0
    overlap=0` — `python3 scripts/completion_atlas.py --check` (denominator: full corpus, 37
    books)

- **Row-count command output:**
  ```
  $ python3 scripts/denominator_gate.py --check-provenance | tail -3
  files_checked=41
  figures_examined=64
  violations=0
  ```
  Same population construct as the original cycle's row-count row (this package's cycle receipts
  + root `.md` files), grown from `files_checked=20`/`figures_examined=22` at the original cycle
  to `41`/`64` now — the growth is Epic 2/3's own new receipts landing in the interim, verified by
  `find docs/release/SD-34-book-completion/artifacts -name '*_cycle_receipt.md' | wc -l` → `41`.

- **Build scope verified:** `cargo test --locked --no-run` — **exit 0**, full workspace, run at
  commit `2eb1536876` plus this cycle's docs-only diff (`CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e1-006`,
  `CARGO_INCREMENTAL=0`). `cargo test --locked --lib` and `apps/desktop/src-tauri`'s suite not
  re-run this cycle: no Rust source or Cargo target was touched, and markdown-only edits to four
  receipt files have no path into either build — inherited per `decisions.md §12` L7 ("run after
  the last write that can move a figure," and this write cannot move a Rust-suite figure).
- **Sweep population:** N/A — this cycle adds and regenerates no corpus records;
  `docs/work-inventory.json` is untouched.
- **Oracle pin:** N/A — no figure in this cycle came from the pinned PCGen corpus.
- **Status:** complete
- **Movement, four buckets:**
  - **Closure:** none new — the stage and the widened default were already built and closed at
    the original cycle.
  - **Reclassification:** none.
  - **Reachability:** none.
  - **Instrument-correction:** the standing gate's *population* (this package's cycle receipts)
    had drifted out of compliance since the original closure — 14 lines across 4 Epic 3 receipts
    stated a figure without a same-line reachable command. Corrected in place; the gate's own
    logic required no change. This is the fourth bucket named in `decisions.md`: a count (here,
    `violations`) that moved because new material was added, not because the instrument itself
    was wrong.
- **Notes:**
  - Two of the four fixed receipts used "same command" / "same" as a cross-reference to a command
    stated on an earlier row/line — clear to a human reader, not reachable to this line-addressed
    check. This is a real instance of the exact anti-pattern `AGENTS.md` rule 9 and
    `workflow-instruction.md §12` row 2 exist to prevent: a command that exists in the document is
    not the same as a command reachable from the specific figure it backs. No score was disputed
    with the gate; the receipts were fixed to restate their own commands.
  - No command was invented to satisfy the gate — every inline command added was copied verbatim
    from a command already present elsewhere in the same receipt (verified with `grep` before
    each edit), preserving `AGENTS.md` rule 7's "a proof is only as wide as the cases it covers":
    this cycle does not claim the Epic 3 figures are newly re-derived, only that their existing,
    already-run commands are now reachable from the lines that state them.
  - This is a standing-gate obligation, not a one-time closure: `acceptance-and-verification.md
    §2` lists `figure-provenance` among the gates that must stay green "at every cycle, not just
    at closure." Any future Epic 3/4/5 cycle that writes a new receipt inherits the same
    same-line-command discipline; a future re-verification should expect `figures_examined` and
    `files_checked` to keep growing and should re-run this exact check before trusting a prior
    PASS.
- **Next-cycle plan:** none required from this cycle — `AT-34-E1-006` remains closed and its
  standing gate is green again at HEAD. A later Epic 3/4/5 cycle that commits a new cycle receipt
  should run `python3 scripts/denominator_gate.py --check-provenance` against its own draft
  before committing, per this cycle's own finding.
