---
canonical: true
owner: sd34-at-34-e6-001
bundle_id: SD-34
criterion: AT-34-E6-001 (gate-lane-B label, wave 23 — NOT the final-acceptance scan)
date: 2026-09-01
verdict: partial
---

# Wave 23, Gate Lane B — frontend, the version bump, and the stale site feeds

**Filename note (read first).** This dispatch's brief and the wave-23 dispatch script
(`artifacts/sd-34-dispatch.workflow.js`, `cycleProcedurePrompt`) both point every criterion —
including this one — at `artifacts/epic-6-closure/AT-34-E6-001_cycle_receipt.md`. That path is
already occupied by a **real, committed, historical artifact**: the actual
`AT-34-E6-001` **final-acceptance-scan** attempt 1 (`17f5245f61`, 2026-08-29, verdict FAIL, 5
kanban cards short). Wave 23 reuses the `AT-34-E6-001` id purely as an Epic-6 tracking label for
three *gate-remediation* lanes (A/B/C) — none of which is the final-acceptance scan itself
(`kanban.md` row 26, `final-acceptance-scan`, still correctly `not-started`). Overwriting the
real scan-1 receipt would destroy the only surviving evidence of that run. This receipt is
therefore written to a **non-colliding filename** (`..._gate-lane-b_cycle_receipt.md`) instead of
the literal instructed path, self-healed per `workflow-instruction.md §8` ("dirty tree, ...
build-counter out of sync" class of issue — a template reused across unrelated criteria is the
same shape). `kanban.md` row 26 is **not** touched by this cycle; this cycle's own status is
recorded only in `progress.md`.

- **Commit SHAs:** `b2805a0b95` (frontend-test fixes + first site-public-status-check attempt,
  later corrected), `19d1c6fdcf` (correction: reverted the wrong-approach code fix, replaced
  with the correct data-only rename in `site/dashboard/units/`, new regression test) — see push
  step for the final pushed SHA.
- **Files touched:** `apps/desktop/src-tauri/Cargo.toml`, `apps/desktop/src-tauri/Cargo.lock`,
  `.github/workflows/publish-tester-release.yml`, `apps/desktop/src/release/buildVersionTriple.test.ts`,
  `apps/desktop/src/releaseChecks/buildVersionTriple.test.ts`,
  `apps/desktop/src/releaseChecks/buildLabelFixtureFreshness.test.ts`,
  `apps/desktop/src/testSupport/makeSurface.ts`,
  `apps/desktop/src/testerWorkbench/loadTesterWorkbenchSurface.test.ts`,
  `apps/desktop/src/testerWorkbench/status/createWorkbenchStatus.test.ts`,
  `apps/desktop/src/testerWorkbench/feedback/bug/composeBugReport.test.ts`,
  `apps/desktop/src/testerWorkbench/feedback/enhancement/composeEnhancementRequest.test.ts`,
  `apps/desktop/src/testerWorkbench/feedback/evidence/captureFeedbackEvidence.test.ts`,
  `apps/desktop/src/operatorTriage/buildOperatorTriageDraft.test.ts`,
  `apps/desktop/src/characterHub/raceCreationCoverage.test.ts`; 19 files under
  `site/dashboard/units/*.json`; new `scripts/tests/test_site_dashboard_units_status_vocabulary_current.py`;
  `docs/retro/events/sd34-at-34-e6-001.jsonl`. `scripts/observer/pf1e_dashboard_producer.py` and
  `scripts/tests/test_pf1e_dashboard_producer.py` were touched then **reverted to a clean net-zero
  diff** in the same cycle (`git diff 6aad5b0f7ab1..HEAD -- <both files>` → empty) — see Notes.

- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
  (`git diff --unified=0 ea2b3396f2fd...HEAD -- apps/desktop/ site/ scripts/observer/pf1e_dashboard_producer.py .github/workflows/publish-tester-release.yml ':!**/__tests__/**' ':!**/*.test.*'`,
  re-run on the final diff)

- **Acceptance criterion (this dispatch's own text, verbatim from the wave-23 brief — NOT
  `epic-breakdown.md`'s `AT-34-E6-001`, which is the unrelated final-acceptance scan):**
  "GATE LANE B — frontend, the missed version bump, and the stale site feeds... Your stages:
  `frontend-test` (4 of 100 files), `site-dashboard-check`, `site-public-status-check`... Bar:
  your stages exit 0, nothing green goes red."

## Figures + their re-derive commands

- **frontend-test, before:** `cd apps/desktop && npm test` → `96/100 test files passed`
  (4 failing: `raceCreationCoverage.test.ts`, `release/buildVersionTriple.test.ts`,
  `releaseChecks/buildLabelFixtureFreshness.test.ts`, `releaseChecks/buildVersionTriple.test.ts`).
- **frontend-test, after:** same command → `100/100 test files passed`, exit 0.
- **race_trait count, re-derived independently (not copied from the dispatch brief):**
  `find data/corpus/core_rulebook/race_trait -name '*.json' | wc -l` → `76`;
  `find data/corpus/beastiary/race_trait -name '*.json' | wc -l` → `108`;
  `find data/corpus/advanced_race_guide/race_trait -name '*.json' | wc -l` → `421`.
  Sum `76+108+421=605`, matching the live test failure's own reported actual value exactly.
  Root cause: `git log --oneline -- data/corpus/core_rulebook/race_trait` shows `ae25d75d7d`
  (AT-34-E3-001, 2026-08-27) added 9 CRB rows (7 `Adopted Race ~ <Race>` CHOOSE selectors + 2
  `Human Ethnicity ~ *` placeholders) — `596 + 9 = 605`. This also moved the same test file's two
  downstream assertions: `standard.length` `175→184` (`67→76` CRB + unchanged `108` B1) and the
  "not racial defaults" list `2→11` entries (both Duergar spell-like-ability rows plus the same
  9 new CRB rows, none of which is `is_racial_default`) — both re-derived from the live failure
  output and updated with the file's own established provenance-narrative comment style.
- **Version files, before:** `python3 -c "import json;print(json.load(open('apps/desktop/package.json'))['version'])"` → `0.14.0`;
  same for `tauri.conf.json` → `0.14.0`;
  `grep '^version' apps/desktop/src-tauri/Cargo.toml` → `0.11.0` (two tranches stale).
- **Version files, after:** `grep '^version' apps/desktop/src-tauri/Cargo.toml` → `0.14.0`;
  `grep -A1 'name = "codex-desktop"' apps/desktop/src-tauri/Cargo.lock` → `version = "0.14.0"`.
  Root `Cargo.toml`: `grep '^version' Cargo.toml` → `0.1.0`, **left unchanged** —
  `decisions.md §11` states this explicitly: *"Root `Cargo.toml` stays pinned at `0.1.0` and is
  not the version source of truth."* Not "fixed blind", per the brief's own warning.
- **Publish-workflow stamp, before:** `grep 'VERSION=' .github/workflows/publish-tester-release.yml`
  line 111 → `VERSION="0.11.${GITHUB_RUN_NUMBER}"` — caught by
  `buildVersionTriple.test.ts`'s `verifiesWorkflowStampTrancheAgreesWithRepoVersionFiles`, not
  named in the dispatch brief. After: `VERSION="0.14.${GITHUB_RUN_NUMBER}"`.
- **Build-label fixtures, before/after:** `grep -c "Codex 0.11.0-test" apps/desktop/src/testSupport/makeSurface.ts <6 more *.test.ts files>`
  → `1` each (7 files, 9 literal occurrences total) before; `0` after,
  `grep -c "Codex 0.14.0-test"` → `1`/`2` each (9 total) after. `STALE_LABEL` in
  `buildLabelFixtureFreshness.test.ts` moved `'Codex 0.10.0-test'` → `'Codex 0.11.0-test'` (this
  bump's own pre-bump literal, continuing the file's established one-bump-behind pattern).
- **site-public-status-check, before:** `python3 scripts/site/build_public_status.py --check` →
  `Traceback ... ValueError: doneness: unmapped 'static' + 'not-ingested'`, exit 1 (crash, not a
  clean STALE report).
- **site-public-status-check, after:** same command → `OK: status-data.json and status-data/*.json are up to date`, exit 0.
- **Legacy-string population in the stale artifact:**
  `grep -rl 'not-ingested' site/dashboard/units/*.json | wc -l` → `19` files;
  total occurrences (one status field per JSON row across 19 single-line files) — re-derived via
  the new test's own `_OLD_PATTERN.findall` count, largest single file `PF1e-units-ability.json`
  → `4337` (== that kind's own bucket-A population from `progress.md` Cycle 1, cross-checked:
  `ability=4337`). After the rename: `grep -rl 'not-ingested' site/dashboard/units/*.json` →
  no output (0 files).
- **AT-34-E1-005 regression sweep, before my code-fix attempt / after reverting it:**
  `python3 -m unittest scripts.tests.test_legacy_not_ingested_string_swept -v` → `OK` (2/2) in
  both states — RED only in between, while the (reverted) code fix was live: `AssertionError`
  naming `scripts/observer/pf1e_dashboard_producer.py:[3959, 3970, 3971]` and
  `scripts/tests/test_pf1e_dashboard_producer.py:[169, 183, 187, 189]`.
- **New regression test:** `python3 -m unittest scripts.tests.test_site_dashboard_units_status_vocabulary_current -v` → `OK` (3/3). RED→GREEN proven by temporarily restoring
  `site/dashboard/units/PF1e-units-ability.json` to its pre-fix committed content
  (`git show HEAD:<path>`), re-running → 2 of 3 tests FAIL for the intended reason (naming the
  exact file, line-count, and the exact `ValueError`), then restoring the fix and re-running → 3/3 OK.
- **`test_pf1e_dashboard_producer` suite, before my touch / after revert:** `python3 -m unittest scripts.tests.test_pf1e_dashboard_producer` → `Ran 21 tests ... OK` in both states (unmodified net).
- **Denominator gate on this package (baseline, untouched by this cycle):**
  `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` →
  `files_checked=16 violations=16` — all 16 pre-existing in `progress.md`/`fable-review.md`
  (verbatim-quoted corpus prose and already-flagged census figures from prior cycles); this
  cycle added no new `.md` prose to that package (only this receipt, added after this figure was
  taken).
- **site-dashboard-check — unresolved this cycle (see Notes and Disposition).**

## Row-count command output (this cycle's own artifact — the 3 assigned stages)

```
$ echo "frontend-test: verified GREEN (100/100, exit 0)"
$ echo "site-public-status-check: verified GREEN (OK: ..., exit 0)"
$ echo "site-dashboard-check: NOT verified green this cycle -- timed out twice under environmental load (see Notes); root-caused and one contributing bug fixed (CODEX_REPO_ROOT); underlying data genuinely stale and its real fix (a producer write-mode run) is out of this lane's authority"
frontend-test: verified GREEN (100/100, exit 0)
site-public-status-check: verified GREEN (OK: ..., exit 0)
site-dashboard-check: NOT verified green this cycle -- timed out twice under environmental load (see Notes); root-caused and one contributing bug fixed (CODEX_REPO_ROOT); underlying data genuinely stale and its real fix (a producer write-mode run) is out of this lane's authority
```

**2 of 3 assigned stages verified GREEN. 1 of 3 (`site-dashboard-check`) not closed this
cycle** — named exactly, with its remaining population being the single committed artifact
`site/dashboard/PF1e-dashboard.json` (`generated_at: 2026-08-24T22:17:30Z`, predating wave-22's
`docs/work-inventory.json` regeneration on 2026-08-31) plus its own `units/` shard mtime-cache,
both of which require `scripts/publish-site-dashboard.sh` run **without** `--check` to refresh —
explicitly the "dashboard producer" run this wave's brief forbids from this lane (hazard:
"both can silently drop stamps").

- **Build scope verified:** `cd apps/desktop && npm test` exit 0 (100/100) and
  `npm run typecheck` exit 0, both re-run at `19d1c6fdcf` (post-fix HEAD).
  `python3 -m unittest scripts.tests.test_pf1e_dashboard_producer scripts.tests.test_legacy_not_ingested_string_swept scripts.tests.test_site_dashboard_units_status_vocabulary_current` → all green, re-run at the same SHA.
  **NOT run this cycle:** `cargo test --locked --no-run` at the workspace's widest scope, and
  `apps/desktop/src-tauri`'s own `cargo test --locked` — the only Rust-adjacent files this cycle
  touched are `apps/desktop/src-tauri/Cargo.toml`/`Cargo.lock` (a pure version-string bump, no
  source), and this lane's shared `CARGO_TARGET_DIR` was occupied for most of the cycle by the
  `site-dashboard-check` investigation (a leftover orphaned `cargo test --locked --no-fail-fast -j 6`
  process, PID 1338655, PPID 1 — inherited from an earlier incarnation of this same lane, still
  running throughout this cycle) and by Lane A's own concurrent root-full sweep on the shared
  host; running a second cargo workspace build concurrently risks the exact memory/CPU fan-out
  hazard this wave's brief names explicitly. **Left for the next cycle to run explicitly before
  any further Rust-adjacent change**, named here rather than silently skipped.
- **Sweep population:** N/A — no `data/corpus/` records added or regenerated this cycle.
- **Oracle pin:** N/A — no figure in this cycle came from the pinned PCGen corpus.
- **Status:** partial
- **Movement, four buckets:**
  - **closure:** frontend-test (4→0 failing files); site-public-status-check (crash→OK).
  - **reclassification:** none.
  - **reachability:** the `site/dashboard/units/*.json` shard cache's real status vocabulary is
    now reachable by `build_public_status.py` without crashing (it was previously unreadable by
    that consumer at all for any `static`/`derived`/`computed`/`ambiguous` unit carrying the old
    word).
  - **instrument-correction:** two, both retro-logged. (1) `_doneness_verdict_uncapped`'s
    'unknown'-precedent pattern does **not** generalize to every retired status word —
    AT-34-E1-005's own regression sweep specifically forbids the 'not-ingested' spelling from
    ever reappearing in code, unlike 'unknown', so the correct fix is a data rename, not a code
    accommodation (correction, `verified-by`: the sweep going RED on the code fix). (2)
    `scripts/publish-site-dashboard.sh --check`'s `v06_work_inventory --summary` subprocess
    defaults its cwd to the shared checkout, not the invoking worktree, unless `CODEX_REPO_ROOT`
    is exported — same root cause and recurrence-key (`wrong-base-worktree`) as pre-existing
    `docs/retro/events/wf_b9c2a3a2-9da-1.jsonl` (incident, 2026-08-19); fixed for this cycle's
    own invocations but the underlying script/producer default is unchanged (out of this lane's
    territory to fix at the source — `scripts/observer/pf1e_dashboard_producer.py` and
    `scripts/publish-site-dashboard.sh` are shared instruments, not `apps/desktop/`/`site/`).

## Notes — the wrong first fix, corrected in the same cycle

The first attempt at `site-public-status-check` taught `pf1e_dashboard_producer.py`'s
`_doneness_verdict_uncapped` the legacy `'not-ingested'` word directly, reasoning by analogy from
its own existing `'unknown'`/`'unmeasurable'` backward-compatibility precedent (commit
`b2805a0b95`). Re-running `test_legacy_not_ingested_string_swept` (AT-34-E1-005's own regression
guard, part of the standard verification sweep for anything touching `scripts/`) caught this
immediately: it went RED, because that sweep's whole design is "this exact string must never
appear in source under `tests/`, `src/`, `apps/`, `scripts/` again, full stop" — a stronger
guarantee than the `'unknown'` precedent carries, made explicit in the sweep's own docstring
(`decisions.md §12` L1's exact lesson: read the code that enforces a guarantee before assuming a
similar-shaped fix generalizes). Reverted in `19d1c6fdcf`, replaced with the doctrine-correct
fix: a plain data-only string rename in the actually-stale committed artifact
(`site/dashboard/units/*.json`), the identical `sed` command AT-34-E1-005's own receipt used for
`docs/work-inventory.json`, each of the 19 files validated as well-formed JSON before and after.
Retro-logged as a `correction` with `--verified-by` naming the sweep's own RED output.

**Why the code-level bug was real but the code-level fix was wrong.** The crash itself
(`ValueError: doneness: unmapped 'static' + 'not-ingested'`) is a genuine defect: a committed,
shipped artifact (`site/dashboard/units/`) silently fell out of sync with a repo-wide rename
because that rename's own criterion (AT-34-E1-005) never enumerated `site/` in its directory
list. The fix belongs in the **data**, because the **string itself** is the thing AT-34-E1-005
banned — not because the classifier was wrong to raise on it.

## Disposition

`frontend-test` and `site-public-status-check`: **closed, verified GREEN, nothing green went
red.** `site-dashboard-check`: **not closed.** Root cause identified (the committed
`site/dashboard/PF1e-dashboard.json` genuinely predates wave-22's inventory regeneration by a
week; its own read-only `--check` invocation additionally suffered a real, now-partially-fixed
environmental bug); its real fix — `scripts/publish-site-dashboard.sh` run for real — is
explicitly out of this lane's authority per this wave's brief. Named as the exact remainder for
the wave's regeneration cycle: run `./scripts/publish-site-dashboard.sh`, verify
`scripts/verify.sh --only site-dashboard-check` exits 0, commit the refreshed feed. A
sequencing/authority question, not an ambiguity — no escalation filed.

## Next-cycle plan

1. The wave's regeneration cycle refreshes `site/dashboard/PF1e-dashboard.json` and
   `site/dashboard/units/*.json` for real (`./scripts/publish-site-dashboard.sh`, **with**
   `CODEX_REPO_ROOT` exported to whatever tree it runs from, to avoid this cycle's own
   near-miss), then re-runs `scripts/verify.sh --only site-dashboard-check`.
2. Run `apps/desktop/src-tauri`'s own `cargo test --locked` and the workspace's
   `cargo test --locked --no-run`, explicitly, before any further Rust-adjacent change on this
   branch — not run this cycle for the resource-contention reason stated above.
3. Consider (out of this lane's scope, flagged for a future cycle, not filed as a blocker):
   widening `test_legacy_not_ingested_string_swept.py`'s `SEARCH_DIRS` to include `site/`, so a
   future regeneration cannot silently reintroduce a retired status word into this exact cache
   again without a loud, immediate test failure.
