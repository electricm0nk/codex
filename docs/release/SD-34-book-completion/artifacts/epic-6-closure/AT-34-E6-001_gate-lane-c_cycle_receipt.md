# Cycle — Epic 6 Closure epilogue / AT-34-E6-001 — Gate Lane C (`denominator-gate` + `figure-provenance`)

**Not the final-acceptance scan** (that is `AT-34-E6-001_cycle_receipt.md`). This is one of the
three territory-disjoint gate-remediation lanes fable-review.md §7 dispatched under the
`AT-34-E6-001` tracking label — the same reuse-of-the-id pattern wave 23's gate-lane-B receipt
already recorded. Scope: the two failing `verify.sh` stages `denominator-gate` and
`figure-provenance`, both attributed to "SD-34's own documents — `progress.md` and the epic-3/5
receipts" (fable-review.md §7).

- **Commit SHA:** see bottom (pushed after this receipt)
- **Files touched:**
  - `scripts/denominator_gate.py` (two new baseline mechanisms for verbatim-quoted corpus prose;
    no detection weakened)
  - `scripts/tests/test_denominator_gate.py` (RED→GREEN tests for both new mechanisms)
  - `docs/release/SD-34-book-completion/progress.md` (7 genuine same-line-denominator fixes + 2
    cosmetic line-rewraps of pre-existing quote text, content unchanged)
  - `docs/release/SD-34-book-completion/fable-review.md` (3 genuine same-line-denominator fixes)
  - `docs/release/SD-34-book-completion/artifacts/bucket-v-widen/AT-34-E3-005_bucket_v_widen_cycle_receipt.md`
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-002_cycle_receipt.md`
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-003_m_bucket_equipment_cycle_receipt.md`
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-003_m_bucket_skill_cycle_receipt.md`
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-005_bucket_v_consolidation_cycle_receipt.md`
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-005_cycle_receipt.md`
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/bucket-v/AT-34-E3-005_bucket_v_apply_cycle_receipt.md`
  - `docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/AT-34-E5-001_cycle_receipt.md`
  - `docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/AT-34-E5-002_cycle_receipt.md`
  - `docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/AT-34-E5-003_cycle_receipt.md`
  - `docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/AT-34-E5-004_cycle_receipt.md`
  - `docs/release/SD-34-book-completion/artifacts/epic-6-closure/AT-34-E6-001_gate-lane-c_cycle_receipt.md` (new, this file)
  - `docs/release/SD-34-book-completion/progress.md` (prepended cycle entry, same commit)
  - `docs/release/SD-34-book-completion/kanban.md` (gate-remediation label row, same commit)
  - `docs/retro/events/sd34-at-34-e6-001-gate-lane-c.jsonl` (new, this cycle's own retro shard)

- **Identifier audit result:** OK_NO_BUNDLE_TAGS — own uncommitted diff (`git diff --unified=0 --
  docs/release/SD-34-book-completion/ scripts/denominator_gate.py
  scripts/tests/test_denominator_gate.py`) carries zero added `sd[0-9]+_`/`t_[0-9a-f]{8,}`-shaped
  lines. (The full `${BASE_BRANCH}...HEAD` diff over the whole package carries many pre-existing
  `sd32_*`/`sd13_*`-shaped hits from earlier, already-merged cycles — not from this cycle's own
  diff, which is what was checked.)
- **Wired-integration audit result:** OK_NO_TOKENS — same own-diff scoping, zero
  `STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack` tokens added.
- **Acceptance criterion (verbatim, dispatch brief):** "AT-34-E6-001 — GATE LANE C —
  denominator-gate (26 violations) and figure-provenance (32). ... Bar: both stages exit 0."
  (Note: the brief's "26" is stale — re-derived at HEAD the true narrow-scope figure was 16, and
  the true full-default-scope figure, since `AT-34-E1-006` already widened `DEFAULT_GLOBS` before
  this cycle, was 21. See Discoveries.)

## Figures + their re-derive commands

| Figure | Value | Command | Denominator |
|---|---:|---|---|
| `denominator-gate`, narrow SD-34-root scope, BEFORE | 16 of 16 files | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` | of 16 files checked |
| `denominator-gate`, full default scope, BEFORE (re-derived, see Discoveries) | 21 of 132 files | reconstructed: 16 (root) + 5 (artifacts-only: `AT-34-E3-003_u_bucket_render_bug` ×1, `AT-34-E3-005_bucket_v_consolidation` ×3, `AT-34-E5-004` ×1) | of 132 files, per `scripts/verify.sh --only denominator-gate` |
| `denominator-gate`, full default scope, AFTER | **0** | `scripts/verify.sh --only denominator-gate` → `PASS (files_checked=132 violations=0)` | of 132 files |
| `denominator-gate` genuine (fixed, denominator/command restated same-line) | **11** | 7 root (`fable-review.md` ×3, `progress.md` ×4) + 4 artifacts (`AT-34-E3-005_bucket_v_consolidation` ×3, `AT-34-E5-004` ×1) | of 21 total |
| `denominator-gate` false-positive (verbatim-quoted corpus prose, recorded via new baseline mechanisms) | **10** | 9 `progress.md` ("75% chance..." `FRT_HVY` quote, `QUOTED_PROSE_CHANCE_IDIOM_RE`) + 1 artifact ("Carrying capacity increased by 50%" `burdenless` quote, `KNOWN_QUOTED_CORPUS_PHRASES`) | of 21 total |
| `figure-provenance`, default scope, BEFORE | 32 of 62 files | `python3 scripts/denominator_gate.py --check-provenance` (default `PROVENANCE_DEFAULT_GLOBS`) | of 62 files, `figures_examined=119` |
| `figure-provenance`, default scope, AFTER | **0** | `scripts/verify.sh --only figure-provenance` → `PASS (files_checked=62 figures_examined=112 violations=0)` | of 62 files |
| `figure-provenance` genuine (fixed, command restated on the figure's own line) | **31** | one file per row below | of 32 total |
| `figure-provenance` false-positive (same `burdenless` quote as above, one shared baseline mechanism) | **1** | `AT-34-E3-003_u_bucket_render_bug_cycle_receipt.md:74` | of 32 total |
| Distinct real corpus phrases behind every false positive in both stages | **2** | `FRT_HVY`'s "75% chance to negate critical hits and sneak attack damage"; `burdenless`'s "Carrying capacity increased by 50%" | 11 line-hits total across both stages trace to these 2 phrases |
| `cargo test --locked --no-run`, workspace | exit 0, 60 test binaries built | `CARGO_TARGET_DIR=/tmp/cargo-sd34-gate-lane-c cargo test --locked --no-run`, at HEAD `9ac2170b3d82f70a3f6076294d22c62a351d5c23` | N/A (build check) |
| `cargo test --locked --no-run`, `apps/desktop/src-tauri` (separate workspace) | exit 0 | `CARGO_TARGET_DIR=/tmp/cargo-sd34-gate-lane-c-desktop cargo test --locked --no-run`, run from `apps/desktop/src-tauri`, same HEAD | N/A (build check) |

**Figure-provenance genuine fixes, one row per file (all restated the actual re-derive command
on the same physical line as its figure, in place of "same command"/"same artifact"/a broken
mid-command line-wrap):**

| File | Lines fixed | Count |
|---|---|---:|
| `artifacts/bucket-v-widen/AT-34-E3-005_bucket_v_widen_cycle_receipt.md` | 114, 115, 117, 120 | 4 |
| `artifacts/epic-3-core-rulebook/AT-34-E3-002_cycle_receipt.md` | 254, 255, 258 | 3 |
| `artifacts/epic-3-core-rulebook/AT-34-E3-003_m_bucket_equipment_cycle_receipt.md` | 107, 113, 127 | 3 |
| `artifacts/epic-3-core-rulebook/AT-34-E3-003_m_bucket_skill_cycle_receipt.md` | 84, 86, 89, 90, 94, 113 | 6 |
| `artifacts/epic-3-core-rulebook/AT-34-E3-005_bucket_v_consolidation_cycle_receipt.md` | 110, 112, 115 | 3 |
| `artifacts/epic-3-core-rulebook/AT-34-E3-005_cycle_receipt.md` | 25, 28 | 2 |
| `artifacts/epic-3-core-rulebook/bucket-v/AT-34-E3-005_bucket_v_apply_cycle_receipt.md` | 114, 117, 118, 120, 132 | 5 |
| `artifacts/epic-5-forward-plan/AT-34-E5-001_cycle_receipt.md` | 23, 28 | 2 |
| `artifacts/epic-5-forward-plan/AT-34-E5-002_cycle_receipt.md` | 84 | 1 |
| `artifacts/epic-5-forward-plan/AT-34-E5-003_cycle_receipt.md` | 130 | 1 |
| `artifacts/epic-5-forward-plan/AT-34-E5-004_cycle_receipt.md` | 112 | 1 |
| **Total** | | **31** |

(Original line numbers, as reported by the pre-fix `--check-provenance` run; several files also
carried a `progress.md`-owned fix pushed earlier in this same cycle.)

## Row-count command output (this cycle's own artifact)

```
$ python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'
files_checked=16
violations=0

$ scripts/verify.sh --only denominator-gate
    PASS  denominator-gate  (files_checked=132 violations=0)

$ scripts/verify.sh --only figure-provenance
    PASS  figure-provenance  (files_checked=62 figures_examined=112 violations=0)
```

Both target stages report `violations=0`. Status set from this count, per `decisions.md §4`.

## Build scope verified

- `cargo test --locked --no-run` (workspace): **exit 0**, 60 test binaries linked, run at HEAD
  `9ac2170b3d82f70a3f6076294d22c62a351d5c23` (this cycle's changes are `.md`/`.py` only — no Rust
  source touched, so no later commit in this cycle can move this result).
- `apps/desktop/src-tauri` (separate cargo workspace): **exit 0**, `cargo test --locked --no-run`
  run explicitly from that directory, same HEAD.

## Sweep population

N/A — this cycle touched no `data/corpus/**` file; `corpus_literal_sweep` not re-run (nothing in
the diff can move its examined-population).

## Oracle pin

N/A — no figure in this cycle's own fixes was drawn from the pinned oracle corpus (the
figure-provenance fixes restate commands that were already correct in the receipts; none is a
newly-derived oracle-sourced number).

## RED → GREEN evidence (TDD)

**RED (before this cycle's code change):** every one of the 9 root-scope `progress.md`
"75% chance..." lines and the 1 artifact "Carrying capacity increased by 50%" line failed
`denominator-gate`/`figure-provenance` for the correct reason — a bare percentage with no
denominator marker (or no reachable command) on its own line, per the gate's own stated rule.
Confirmed by the very first `--check`/`--check-provenance` runs this cycle (see progress.md
entry, quoting the literal pre-fix output).

**Mechanism added, not a text reword:** `QUOTED_PROSE_CHANCE_IDIOM_RE` (a percentile-game-mechanic
idiom, `\d[\d,]*(?:\.\d+)?\s?%\s*chance\b`) and `KNOWN_QUOTED_CORPUS_PHRASES` (an explicit,
named, exact-substring allowlist — one entry so far) in `scripts/denominator_gate.py`, blanked
out of the scan line *before* the percent/figure/denominator checks run, identical discipline to
the pre-existing `FALSE_100_IDIOM_RE`. New unit tests: `test_chance_idiom_not_flagged`,
`test_chance_idiom_bare_form_not_flagged`,
`test_chance_idiom_does_not_shadow_a_real_percentage_on_the_same_line`,
`test_chance_idiom_with_its_own_denominator_still_passes`,
`test_known_quoted_corpus_phrase_not_flagged`,
`test_known_quoted_corpus_phrase_does_not_shadow_a_real_percentage` — each proves the mechanism
exempts only its own quoted token and still catches a genuine, separate percentage placed on the
same line (the "shadow" tests are the RED→GREEN proof: a synthetic line combining the quote and a
real unsourced figure still reports exactly one violation, on the real figure).

**GREEN:** `python3 -m unittest scripts.tests.test_denominator_gate` → `Ran 46 tests ... OK`
(41 pre-existing + 6 new, all passing; two pre-existing mutation-proof tests
(`test_default_globs_currently_clean`, `test_provenance_default_run_is_clean`) that exercise the
*entire live package* now pass for the first time, having failed at the start of this cycle).

**No baseline mechanism weakened detection:** every existing test in
`scripts/tests/test_denominator_gate.py` (percentage-with-denominator, false-100 idiom, fenced
blocks, mutation RED/GREEN proofs) still passes unchanged; a synthetic 4th-digit percentage with
no denominator and no quote-marker is still caught (`test_bare_percentage_flagged`, unmodified).

## Discoveries

1. **The brief's "26 violations" figure was stale.** `decisions.md §3` documents the narrow-scope
   command (`--check 'docs/.../*.md'`, root-level only) as the standing gate "until AT-34-E1-006
   widens the default." `AT-34-E1-006` is `complete` (`kanban.md` row 6) and its landed code
   (`scripts/denominator_gate.py`'s `SD34_BUNDLE_DIR` entries in `DEFAULT_GLOBS`) already widens
   the default scope to include this package's `artifacts/**/*_cycle_receipt.md` on top of the
   root `*.md` files — so the *real* standing gate is now `scripts/verify.sh --only
   denominator-gate` (full default scope, `files_checked=132`), not the narrow command. Re-derived
   the true original count at that real scope: **21**, not 26 and not 16. Filed as a `correction`
   retro event (`sd34-at-34-e6-001-gate-lane-c-23afdb`).
2. **The figure-provenance count (32) matched the brief exactly** — that stage's own default
   scope (`PROVENANCE_DEFAULT_GLOBS`, SD-34-only) was already correctly scoped at authoring time.
3. **Two of the "false positive" progress.md lines (2901, 3529) needed a cosmetic line-rewrap**,
   not a code change alone: the corpus quote's own two words ("75%" / "chance...") had been
   manually word-wrapped across two physical source lines by an earlier cycle's prose, which
   defeats any same-line idiom match by construction (the gate is deliberately line-scoped, per
   its own docstring). Rewrapped so both words land on one physical line — the quoted text itself
   is byte-for-byte unchanged, only where the line break falls moved.
4. **One own-introduced regression, caught and fixed inside this same cycle.** Writing the first
   pass of the `bucket_v_consolidation` receipt's "92.4% of 2,793" fix wrapped the new "of 2,793"
   marker onto the *next* physical line, re-triggering the exact defect this cycle exists to fix,
   on my own new text. Caught by re-running the full test suite before moving on (`decisions.md
   §12` L7's own discipline, applied to my own diff, not just the corpus's), fixed by keeping the
   whole clause on one line.
5. **Every figure-provenance fix restates a command that was already correct** (already given in
   full on an earlier line in the same table/paragraph, or a script already committed) — none of
   the 31 genuine fixes invented a new command; each is a "same command"/"same artifact"/broken
   mid-line-wrap reference resolved by literally repeating the existing command inline. Two rows
   (`AT-34-E3-005_bucket_v_widen`'s two "via ..." breakdown rows) needed a real, previously-unwritten
   command — verified against the live artifact before use: `python3 -c "import json,collections;
   print(collections.Counter(r['_source'] for r in
   json.load(open('.../bucket-v-corpus-wide-consolidated.oracle-results.json'))['results']))"` →
   `{'.../AT-33-E5-003.combined-oracle-results.json': 5748, '.../probe-surface-census.json': 842}`,
   matching the receipt's own stated 5,748/842 exactly.
6. **No new baseline mechanism was proposed but left unimplemented** — the brief asked to "propose
   one and say so" if the gate had no mechanism for quoted prose; it had none, so both a
   content-shaped idiom (`% chance`, generalizable — matches any future PF1e percentile-mechanic
   quote) and a narrow, explicit, auditable allowlist (`KNOWN_QUOTED_CORPUS_PHRASES`, for the one
   quote that isn't chance-shaped) were built and wired into *both* `denominator-gate` and
   `figure-provenance`, not just proposed.

## Status: complete

Both assigned stages (`denominator-gate`, `figure-provenance`) exit 0 at HEAD, re-verified via
`scripts/verify.sh --only <stage>` (not just the narrower `denominator_gate.py` CLI), with the
full unit test suite green (46/46) and the widest build scope (workspace + separate desktop
crate) both exit 0.

## Movement, four buckets

- **Closure:** 0 (this criterion moves no `docs/work-inventory.json` bucket; it is a
  gate-remediation lane, not a content-completion cycle).
- **Reclassification:** 0.
- **Reachability:** 0.
- **Instrument-correction:** 2 — (a) `scripts/denominator_gate.py` gained two additive baseline
  mechanisms so 11 real quoted-corpus-prose lines are no longer misclassified as ungrounded
  figures; (b) the true standing-gate scope was corrected from the stale narrow command to the
  real, already-widened default (`scripts/verify.sh --only denominator-gate`), a 5-violation
  undercount in the brief's own accounting.

## Notes

- **Judgment call:** the "Carrying capacity increased by 50%" quote is handled by an explicit,
  named, single-entry allowlist rather than a broader "percentage inside quotation marks"
  heuristic. A broad quote-scoped exemption would also swallow a genuine completion percentage
  someone quoted for emphasis — the narrower, auditable list trades a little future convenience
  for never weakening what the gate detects (the brief's own constraint on editing this file).
- **Judgment call:** did not touch `docs/release/SD-33-computed-value-verification/` even though
  `DEFAULT_GLOBS` also scans it (116 of the 132 files checked belong to SD-33's folder) — that
  folder is outside this bundle's write scope per `workflow-instruction.md`, and the full-scope
  run already reports 0 violations there at HEAD, so no SD-33 fix was needed.
- Followed `workflow-instruction.md §5`'s concurrent-write protocol for the shared files
  (`progress.md`, `kanban.md`); re-read both immediately before editing.

## Next-cycle plan

None named — both assigned stages are green and this lane's scope is exhausted. The
final-acceptance scan (a separate criterion, `AT-34-E6-001_cycle_receipt.md`) should re-verify
`scripts/verify.sh --only denominator-gate` and `--only figure-provenance` both report
`violations=0` as part of its own re-derivation, per `acceptance-and-verification.md §3`
obligation 2 ("re-run every headline command yourself").
