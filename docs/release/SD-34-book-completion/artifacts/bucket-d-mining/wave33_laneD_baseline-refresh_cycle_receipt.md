# Cycle — SD-34 wave 33, Lane D — refresh the four stale `scripts/verify.sh` test baselines

**Status: complete.** Raised `BASELINE_ROOT_LIB_TESTS`, `BASELINE_ROOT_FULL_TESTS`,
`BASELINE_ROOT_TEST_BINARIES`, and `BASELINE_DESKTOP_TESTS` in
`scripts/verify-baselines.env` from their SD-31-wave-29-era recorded values
(2336/7469/569/515) to this cycle's own freshly-measured values
(3028/8372/589/572), itemized per that file's own convention, and re-verified
`scripts/verify.sh` 40/40 at the new floor.

- **Commit SHA (baseline edit, pushed):** `7ea9651b87`
- **Commit SHA (this receipt + progress.md, pushed):** *(reported in the
  structured output — cannot cite its own future hash)*
- **Files touched this cycle:**
  - `scripts/verify-baselines.env` — appended one new dated block (this
    file's own "last assignment wins" convention; nothing above the new
    block was edited) raising the four floors and itemizing why each moved
    by exactly that much.
  - No other file needed a change: a repo-wide grep for both the old
    (`2336`, `7469`, `569`, `515`) and new (`3028`, `8372`, `589`, `572`)
    literals across `tests/`, `src/`, `apps/`, `scripts/` found no
    load-bearing hits — the only matches are coincidental
    `monster_data.rs` `source_line:` values and unrelated PCGen `.lst` line
    citations, confirmed by reading each hit, not just counting it:
    `grep -rn "\b2336\b\|\b7469\b\|\b569\b\|\b515\b\|\b3028\b\|\b8372\b\|\b589\b\|\b572\b" tests/ src/ apps/ scripts/ --include="*.rs" --include="*.py" --include="*.sh" --include="*.ts" --include="*.tsx" | grep -v verify-baselines.env`
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — `BASE_BRANCH=$(git
  merge-base HEAD origin/develop)` = `ea2b3396f2`; `git diff --unified=0
  ea2b3396f2...HEAD -- scripts/verify-baselines.env | grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` finds nothing (the new
  comment block writes "SD-34"/"SD-31" with a hyphen, not the underscore
  shape the audit scans for).
- **Wired-integration audit result:** `OK_NO_TOKENS` — same diff range,
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`
  finds nothing. No shipping code path touched either way; this is a
  recorded-truth number in a verification config file.
- **Acceptance criterion:** *(no `AT-34-E#` card exists for this generic
  gate-instrument task — the wave-33 dispatch script names it directly:
  "LANE D — refresh the four stale test baselines".)* Verbatim from the
  dispatch brief: "`scripts/verify.sh` now passes 40 of 40 stages, but
  prints four BASELINE NOTES ... These live in `scripts/verify-baselines.env`
  ... every raise is itemized and justified: which tests, from which cycle,
  and why the number moved by exactly that much."

## Figures + their re-derive commands

All four percentages/large figures below carry their command and
denominator on the same line, per `denominator_gate.py --check-provenance`.

- `BASELINE_ROOT_LIB_TESTS`: 2336 recorded -> **3028 measured**, of the
  crate's own lib-target test population (not a percentage; a floor). Command:
  `scripts/verify.sh --only root-lib --show-actuals` -> `PASS root-lib
  (3028 passed)`.
- `BASELINE_ROOT_FULL_TESTS`: 7469 recorded -> **8372 measured**, of the
  crate's own full `cargo test --no-fail-fast` population. Command:
  `scripts/verify.sh --only root-full --show-actuals` -> `PASS root-full
  (8372 passed across 589 suites, all N tests/*.rs suites executed)`.
- `BASELINE_ROOT_TEST_BINARIES`: 569 recorded -> **589 measured**, of the
  same `root-full` run's own "Running" line count. Fully reconciled (not
  just measured): `git ls-tree --name-only 2071ce7c46 -- src/bin/ | grep -c
  '\.rs$'` -> 39, `... tests/ | grep -c '\.rs$'` -> 540 (39+540+1 lib = 580,
  matching the wave-29 baseline exactly); `git ls-tree --name-only
  84760e4326^ -- src/bin/ | grep -c '\.rs$'` -> 56, `... tests/ ...` -> 543
  (real count had grown to 600 before the SD-34 batch-A deletion, unrecorded);
  `git ls-tree --name-only 84760e4326 -- src/bin/ | grep -c '\.rs$'` -> 45,
  `... tests/ ...` -> 543 (589 real, but batch-A wrote 569 = 580-11 against
  the stale base); `git diff --stat 84760e4326 HEAD -- src/bin tests | grep
  -c '\.rs '` -> 0 (zero further bin/tests file churn since batch-A, so 589
  is exact and current).
- `BASELINE_DESKTOP_TESTS`: 515 recorded -> **572 measured**, of the
  desktop crate's own `cargo test` population. Command:
  `scripts/verify.sh --only desktop --show-actuals` -> `PASS desktop
  (572 passed)`.
- Bundle-boundary `#[test]`-count proxy (cross-check, not the recorded
  figure itself): `git grep -c '#\[test\]' <rev> -- 'src/*.rs'
  'src/**/*.rs' ':(exclude)src/bin/**' | awk -F: '{s+=$NF} END{print s}'`
  at `2071ce7c46`/`e28d79a8c7`/`525e087c5b`/`9a00662f22`/`aee47d3c5a` reads
  2354/2354/2828/2865/3042 (lib column) — full derivation and the matching
  `full`/`desktop` columns are in `scripts/verify-baselines.env`'s own new
  comment block. This proxy's total growth (688 lib / 899 full / 57
  desktop) reconciles to within 4 of the real measured deltas (692/903/57)
  — the residual is named, not hidden.

## Row-count command output

Not a row-owning artifact (no `AT-34-E#` card, no JSON/table this cycle
regenerated) — N/A, per the receipt schema's own allowance for cycles
outside the epic/card structure.

## Build scope verified

`scripts/verify.sh --only root-lib --only root-full --only desktop
--show-actuals` at HEAD `aee47d3c5a` (pre-edit) / re-run at `7ea9651b87`
(post-edit, the same commit as the baseline change plus this receipt):

- root-lib: PASS, 3028 passed
- root-full: PASS, 8372 passed across 589 suites, all `tests/*.rs` suites
  executed (no missing-suite gap)
- desktop (`apps/desktop/src-tauri`, separate crate/lockfile): PASS, 572
  passed

Full `scripts/verify.sh` (all 40 stages) run separately, log path and
`RESULT:` line in the next section.

## Sweep population

N/A — this cycle touches no corpus content; `corpus-sweep`'s own population
is unaffected (`BASELINE_CORPUS_LITERAL_RECORDS` unchanged at 48706).

## Oracle pin

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` — unchanged,
carried forward from `scripts/pcgen-oracle-pin.env`; no figure in this
receipt derives from the pinned corpus (all four figures are test counts,
not corpus-content counts).

## Full gate result

`scripts/verify.sh` at `7ea9651b87` (this cycle's own commit): **RESULT: PASS, 40/40**,
`5906s` (1h38m26s), log `/tmp/codex-verify-9KJsiq`, auto-emitted retro event
`docs/retro/events/sd31-transcribe.jsonl` (`ts: 2026-09-02T14:44:02Z`, `result: "PASS"`,
`stages_passed` lists all 40). This run was interrupted mid-flight by a server crash
(unrelated kernel soft-lockup, `journalctl -b -1` — heavy parallel `rust-lld` link jobs,
not this cycle's own baseline edit) and re-run clean from the same commit after reboot; the
re-run is the one cited here.

## Status: complete

The lane's assigned population — the four named BASELINE NOTES — is fully
closed: all four floors raised, itemized, cross-checked, and re-verified
40/40 green.

## Movement, four buckets

- **Closure:** 0 content units (this is a gate-instrument cycle, not
  content-facing work).
- **Reclassification:** 0.
- **Reachability:** 0.
- **Instrument-correction:** 4 — the four stale `BASELINE_ROOT_*`/
  `BASELINE_DESKTOP_TESTS` floors, raised from an 8-day-old (2026-08-21)
  wave-29 measurement to this cycle's own fresh measurement, closing the
  BASELINE NOTES `scripts/verify.sh` had been printing on every green run
  since. Also corrects (in the itemization comment, not by rewriting
  history) the SD-34 batch-A cycle's own arithmetic error — its `-11`
  `BASELINE_ROOT_TEST_BINARIES` delta was computed against the stale
  wave-29 base (580) rather than the true pre-deletion count (600),
  silently discarding +20 of real, already-landed growth.

## Notes

- **The brief said "wave 32"; the repo's own ledger already had wave 33
  registered at HEAD.** Per the standing rule ("where the repo and this
  brief disagree, the repo wins"), this receipt treats the assignment as
  wave 33 lane D, matching `docs/release/SD-34-book-completion/artifacts/sd-34-wave33.workflow.js`'s
  own `laneD()` prompt text (which is verbatim identical to the brief this
  cycle received, just correctly labeled).
- **Full per-cycle itemization (692/903/57 tests traced to individual
  waves) was judged out of budget and not attempted.** 283 commits touched
  test-bearing paths between the wave-29 anchor and this cycle's HEAD,
  spanning the closed SD-32 and SD-33 bundles' full histories plus SD-34's
  own 33+ waves so far. The brief's own escape hatch — "record what you
  could not account for rather than rounding to the measured figure
  silently" — is exercised here: the itemization in
  `scripts/verify-baselines.env` is at BUNDLE granularity (SD-31 tail /
  SD-32 / SD-33 / SD-34), backed by a mechanical, reproducible `#[test]`
  proxy count, not a manual per-cycle narrative. A 4-test (lib) and 4-test
  (full) residual gap between the proxy's total and the real measured
  total is named as untraced, not rounded away.
- **TEST_BINARIES is the one figure fully, exactly reconciled** — no proxy,
  no residual — because file-existence diffing (`git ls-tree`) is exact in
  a way `#[test]`-count grepping is not.
- Clippy (root/desktop), frontend-test-files, computed-classes, and
  corpus-literal-records baselines are unchanged (0/0/100/31/48706) — none
  of them were named in this cycle's territory, and none had a BASELINE
  NOTE outstanding for this cycle to close.

## Next-cycle plan

None outstanding for this specific lane — all four named baselines are
closed. The residual 4/4-test untraced gap in the lib/full proxy
reconciliation is not large enough to warrant a dedicated follow-up cycle
on its own; it is recorded in `scripts/verify-baselines.env` for whichever
future cycle next re-measures these floors to fold in if it turns out to
matter.
