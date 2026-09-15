# AT-35-E7-001 — final-acceptance scan, cycle 3 — **verdict: FAIL**

**Head scanned:** `486c7f0cf04756d3b1fba3900a8daac48ce6cc18`
**Tranche cut compared against:** `4c6c57eb9f`
**Date:** 2026-09-15
**Bar:** `acceptance-and-verification.md §3` + `§3a`; `epic-breakdown.md` "AT-35-E7-001".

`§3a`: *"If anything is short: STOP. No retrospective, no sweep, no PR."* **Eight** items are short.
This cycle therefore closes **zero** units, writes **no** retrospective, runs **no** sweep and
opens **no** PR. It is a measurement cycle only.

`§3a` also says *"do not manufacture a shortfall."* Every shortfall below is stated with the
command that produces it, and the green items are listed just as explicitly.

---

## What is GREEN at HEAD (re-derived, not quoted)

| check | command | result |
|---|---|---|
| Completion atlas | `python3 scripts/completion_atlas.py --check` | `population=49450 unclassified=0 overlap=0`, `DONE: 49450`, every other bucket `0`, `done_evidence_violations=0`, `missing_clearing_mechanisms=0`, `stale_derived_at=False`, `citation_failures=0`, **exit 0** |
| PCGen residue closure gate | `python3 scripts/pcgen_residue_gate.py --check --closure` | `live_files=0 live_hits=0 verdict=PASS`, **exit 0**; all 12 patterns 0, all 5 live roots 0, `shipped_data_hits=0` over `shipped_scanned=11` |
| `data/sheet_rules/` token grep | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` | `0` |
| Token-coverage partition | `python3 scripts/token_coverage.py --check` | `verdict=PASS`, exit 0 — `units=49450 non_done=0 census_records=49450`, all six sub-checks `ok=True` (but see **S3**) |
| Closure instruments carry no carve-out list | `grep -rniE 'EXCLUDED_BOOKS\|EXCLUDE_\|SKIP_BOOKS\|ALLOWLIST\|ALLOW_LIST\|WHITELIST\|IGNORE_PATHS' scripts/completion_atlas.py scripts/token_coverage.py scripts/pcgen_residue_gate.py scripts/cycle_scope_gate.py` | **no output** — no hardcoded exclusion list in any of the four |
| Tool side intact (`decisions.md §11`) | `ls scripts/pcgen-oracle-pin.env scripts/fetch-pcgen-oracle.sh`; `python3 scripts/oracle_harness/run.py --help`; `ls src/pcgen_import/` | both pin files present; harness `--help` exits 0; `src/pcgen_import/` holds the relocated parser, generators and `cache_gen`. **No converter/oracle file deleted.** |
| Manifest evidence pointers | independent stratified sample, seed 7, **209 units across all 19 kinds** (11 per kind) | `no_evidence_field=0` — every sampled unit carries `evidence`, `source_file`, `source_line`, `sheet_rule_content` |
| Cycle-receipt scope-gate coverage | `grep -qiE 'SCOPE_GATE\|cycle_scope_gate'` over all 125 receipts | 123 of 125 carry one (see **S5**) |
| Independent live-code grep (`§3a`) | `grep -rn 'raw_tokens\|PcgenFormulaEvaluator\|render_pcgen_desc' src/rules_core src/saved_character src/campaign src/homebrew_authoring apps/desktop` | hits exist but **every one** is a doc comment (ruling B14, `decisions.md §17`) or inside a `#[cfg(test)]` region (ruling B15, `§18`) — e.g. `trait_pool.rs:479` sits under the `#[cfg(test)]` at line 293. **No live read.** |

---

## SHORTFALLS

### S1 — 57 of 104 `kanban.md` rows are not `complete`

*(Denominator: measured at scan time, before this cycle appended its own row 107. After that
append the board reads **58 of 107**. Both figures are the same 54 substantive rows.)*

```
awk -F'|' '/^\| *[0-9]+ *\|/{st=$6; gsub(/^ +| +$/,"",st); print st}' \
  docs/release/SD-35-corpus-sheet-completion/kanban.md | sort | uniq -c
```
→ `complete 47`, `in-progress 51`, `partial 3`, `blocked 3`, `blocked-escalated 1`, `not-started 1`.

The bar is verbatim: *"Every criterion `AT-35-E1-001` … `AT-35-E6-004` is `complete` and every
`kanban.md` card is `complete`. There is no 'complete or filed under Open blockers'."*

Breakdown of the 57:

| rows | card | state |
|---|---|---|
| 26, 48–58 (12) | `desktop-and-prose-leave-pcgen` / AT-35-E6-003 | `in-progress` |
| 43–46 (4) | `generators-leave-rules-core` / AT-35-E6-002 cycles 2–5 | `in-progress` |
| 59 | AT-35-E6-003-SWEEP cycle 1 | **`blocked-escalated`** |
| 60–75 (16) | AT-35-E6-003-SWEEP cycles 2–17 | `in-progress` |
| 76 | AT-35-E6-003-FINISH cycle 1 | **`blocked`** |
| 77, 78 | AT-35-E6-003-FINISH cycles 2, 3 | `in-progress` |
| 79–95 (17) | AT-35-E6-003-RULED cycles 1–17 | `in-progress` |
| 101, 102, 103 | AT-35-E7-000-POPULATION-CENSUS cycles 1–3 | `partial` |
| 28, 100 | `final-acceptance-scan` / AT-35-E7-001 | `blocked` (this scan's own rows) |
| 29 | `retro-sweep-archdocs-pr` / AT-35-E7-002 + E7-003 | `not-started` |

Rows 28, 100 and 29 are the closure rows themselves and are expected to be open while the scan
runs. **The other 54 are not.** AT-35-E6-002 and AT-35-E6-003 are both certified `complete` at
the criterion level while 53 of their own cycle rows read `in-progress`, `blocked` or
`blocked-escalated`. Under `§5` (*"A lane's `status: complete` unsupported by the mechanical
receipt rows"* does not satisfy a criterion) this is exactly the gap the scan exists to catch.

**This is the same S1 cycle 2 reported (it said 55 rows).** It has not moved; the count changed
only because three census rows were added since.

### S2 — `data/sheet_rules/_refused.json` is not empty

`§3a` requires: *"`sheet_rule_convert --check` at HEAD → ids agree with the corpus;
`_refused.json` empty."*

```
python3 -c "import json;d=json.load(open('data/sheet_rules/_refused.json'));print(d['records'],d['converted'],d['refused'],d['by_token_type'])"
```
→ `49450 49308 142 {'no_corpus_record': 142}` — 142 entries, one token type, e.g.
`advanced_race_guide:race:dhampir`, `advanced_race_guide:race:drow`.

Mitigating and stated plainly: every one of the 142 is `DONE` in the atlas by another route
(`refused_non_done=0`), so **no unit is un-rendered**. But `§3a` names the *file*, and the file
is not empty. This needs an operator ruling amending `§3a` to "zero non-`DONE` refusals", or the
142 converted. **Cycle 2 asked for exactly this ruling and did not get one.**

### S3 — `token_coverage.py --check` reports `refused=142`

`§3a` requires *"`token_coverage.py --check` at HEAD → zero refused units."* It reports
`refused=142 refused_non_done=0`. The gate's own `verdict=PASS` because its partition balances;
**the acceptance bar is stricter than the gate**. Same population as S2 — one ruling disposes of
both.

### S4 — 88 open deferrals

```
python3 scripts/retro.py summary --since 2026-09-07 --json | \
  python3 -c "import json,sys;d=json.load(sys.stdin)['deferrals'];print(d['open'],d['resolved'],d['total'])"
```
→ `open=88 resolved=1 total=89`.

`§3` step 8: *"Enumerate open deferrals. None may defer DoD scope."* 88 stand open, up from the
83 cycle 2 recorded. `AGENTS.md` Blocker Discipline: a deferral of DoD scope is a blocker, and a
blocker is cleared or escalated, never deferred.

### S5 — two cycle receipts carry no scope-gate line

`§3` step 10 and `§5` (*"A cycle receipt with no `cycle_scope_gate.py` line"* does not satisfy a
criterion). Of 125 receipts under `artifacts/`, two have neither a `SCOPE_GATE` nor a
`cycle_scope_gate` string:

- `artifacts/epic-4-resolve-and-verify/AT-35-E4-001_cycle1_receipt.md`
- `artifacts/epic-4-resolve-and-verify/AT-35-E4-002_cycle1_receipt.md`

`progress.md`'s own preamble calls this out: *"An entry without the scope-gate line is a process
defect."*

### S6 — the three `§3a` oracle-parity artifacts do not exist at their named paths

`§3a`: *"The oracle parity artifacts (`oracle-parity-epic2.json`, `-before.json`,
`-after.json`) each name `PCGEN_ORACLE_SHA` and show `disagree=0`."*

```
ls docs/release/SD-35-corpus-sheet-completion/artifacts/epic-2-sheet-rule/oracle-parity-epic2.json \
   docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/oracle-parity-before.json \
   docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/oracle-parity-after.json
```
→ all three **No such file or directory**.

Per-cycle parity artifacts *do* exist in quantity (`AT-35-E6-001_cycle1_sheet-parity-before.json`,
`AT-35-E6-004_cycle3_sheet-parity-after.json`, an `epic-2-sheet-rule/oracle-parity/` directory
holding `ours.json` / `sheet-parity.json` / `exports/` / `roster/`, and ~17 more). The evidence
is plausibly all present under other names — but `§3a` names three files by path and the scan
does not get to decide that a differently-named file satisfies a named deliverable. Either the
three consolidated artifacts are produced, or `§3a` is amended by ruling to name what exists.

### S7 — `completion-manifest.json` is stale: 49,438 units, generated at a non-HEAD SHA

```
python3 -c "import json;d=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-5-residues/completion-manifest.json'));print(d['generated_at_head'],len(d['units']))"
```
→ `39dfd59b7e4763acc09ac374441ed4b3647c01f7 49438`

The atlas at HEAD reads `population=49450`. Ruling B18 (`decisions.md §21`) admitted 12 more
records and **the manifest was never regenerated**; `49,438` is the figure `epic-breakdown.md`
says *"is superseded everywhere and is never the bar."* `§3` step 11 requires every build-scope
row to name the SHA it ran at with no later commit regenerating the inventory — here a later
commit did exactly that and the manifest did not follow. The 209-unit sample above was therefore
drawn from a population 12 units short of the real one.

---

## Steps NOT RUN, and why

`§3a`'s stop rule fires on the first shortfall. These were not run because the verdict was
already FAIL and running them cannot change it:

- **`§3` step 9 — re-prove each gate still fails** (plant a violation in `cycle_scope_gate`,
  `pcgen_residue_gate`, `token_coverage`, `sheet_rule_convert --check`, a moved anchor; confirm
  the catch; remove the probe; confirm zero residue). Planting probes in a shared checkout that
  is already FAIL adds risk with no decision value.
- **Per-sampled-unit `SheetRule` evaluation against the probe character** — the sample's
  population is wrong (S7); re-sampling after the manifest is regenerated is the only sound run.
- **`§3` step 1 launch-vs-HEAD id-set subtraction** and **step 4 failure attribution against
  `4c6c57eb9f`** — both are attribution work on a closure that is not being certified.

---

### S8 — the full `verify.sh` at HEAD is **RED**: 48 passed, 1 FAILED

`scripts/verify.sh` (all stages), run at HEAD in a private `CARGO_TARGET_DIR`
(`/tmp/cargo-sd35-AT-35-E7-001`, `CARGO_INCREMENTAL=0`). Logs: `/tmp/codex-verify-0vc2D9`.

```
SUMMARY
  passed:  48  preflight-disk preflight-oracle oracle-pin-selftest producer-selftest
               pi-redaction-selftest provenance-selftest site-dashboard-selftest
               site-dashboard-pin site-dashboard-check site-dashboard-pi-gate
               build-public-status-selftest site-public-status-check site-public-status-pi-gate
               site-asset-stamp-check reachability-audit-selftest reachability-audit
               groundtruth-guard-selftest supersession-gate-selftest
               shape-coverage-standing-gate-selftest shape-coverage-standing-gate
               cycle-scope-gate-selftest shape-engine-boundary-selftest shape-engine-boundary
               missing-engine-tables denominator-gate figure-provenance pcgen-residue-gate
               token-coverage-selftest token-coverage pi-sweep declared-pi-audit audit-selftest
               reclaim-selftest driver-selftest corpus-sweep-selftest corpus-trap-audit-selftest
               root-lib desktop reach corpus-sweep sheet-rules-check corpus-trap-audit
               supersession-gate frontend-install frontend-test frontend-typecheck clippy
               class-dump
  FAILED:  1  root-full

RESULT: FAIL — logs in /tmp/codex-verify-0vc2D9
```

Notable greens inside it, quoted from the stage lines:

- `sheet-rules-check` — `records=49450 converted=49308 refused=142 rules=70147 var_tables=5293
  verdict=PASS` — **this is the HEAD run of `sheet_rule_convert --check` `§3a` asks for, and it
  independently confirms S2 and S3's `refused=142`.**
- `pcgen-residue-gate`, `token-coverage`, `cycle-scope-gate-selftest`,
  `shape-engine-boundary`, `missing-engine-tables`, `denominator-gate`, `figure-provenance`,
  `pi-sweep`, `declared-pi-audit` — all PASS.
- `corpus-sweep` — `48706 records examined of 51523 read, 413314 tokens compared, 0 findings`.
- `desktop` 570 passed · `reach` 32 passed · `frontend-test` **101/101 files** · `clippy`
  `root:0 desktop:0 warnings`.

**Failure attribution, done per `AGENTS.md` — every `test result: FAILED` line traced to its
`Running` line, not bucketed:**

- `grep -c "test result: FAILED" /tmp/codex-verify-0vc2D9/root-full.log` → **1**, out of
  **420** `Running` / `Doc-tests` lines. `8925 passed across 419 suites`.
- The one suite: **`tests/sd26_pilot_case_verification.rs`** — `2 passed; 1 failed`, 968.67s.
- The one test: `full_pipeline_runs_end_to_end_and_finds_one_genuine_attack_bonus_mismatch`,
  panicking at `tests/sd26_pilot_case_verification.rs:377:31`.
- The panic message: `real PCGen engine run should succeed: pcgen-run-character.sh failed (exit
  status Some(2))` → PCGen's **own** gradle build, at
  `/home/ubuntu/workspace/repos/pcgen/build.gradle:79`:
  `Could not create task ':extractJavaFXLocal'. > Could not create task ':downloadJavaFXLocal'.
  > Unable to process url: https://api.adoptium.net/v3/assets/feature_releases/25/ga?...`
  `BUILD FAILED in 16m`.

**It is NOT excused as environmental, and here is why not.** The obvious reading is "the box
could not reach adoptium." That reading does not survive a check: re-queried from this same box
right after the run, that exact URL returns **HTTP 200** (`curl -o /dev/null -w '%{http_code}'`),
and `api.adoptium.net` pings at 10 ms. So the network is up now and the cause is not established
— it is either a transient outage during the 16-minute gradle window or something in gradle's own
resolution, and this scan does not know which.

Compounding it: `tests/sd26_pilot_case_verification.rs` **was touched inside this bundle** —
`git log 4c6c57eb9f..HEAD -- tests/sd26_pilot_case_verification.rs` → `b91d16a66b`
(`AT-35-E6-002 cycle 1`). The panic occurs at the PCGen invocation, before any of our comparison
logic runs, so the bundle's edit is not the visible cause; but "not the visible cause" is not
"attributed", and the scan does not get to assume.

`AGENTS.md`: *"A verification stage red for more than one run is a blocker, not a background
condition… 'The N known environmental failures' is a bucket, not an attribution."* One suite is
named here, and it needs one clean re-run to classify. **Either way the gate's verdict at HEAD is
`RESULT: FAIL`, and `§3` step 6 requires the full `verify.sh` to be green here.**

---

## Scope-gate row

`SCOPE_GATE: EXEMPT (Epic 7 acceptance-scan cycle — closes zero units by design, decisions.md §2)`
· `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=1 pcgen_live_files=0`

Retro events: `incident 1789462522400-at-35-e7-001-b6657f` (the scan's FAIL),
`incident 1789470212670-at-35-e7-001-ce9b36` (the `verify.sh` red),
`correction 1789462522557-at-35-e7-001-75f938` (criterion-level `complete` vs the board),
`deferral 1789462522692-at-35-e7-001-49fa3b` (steps 1, 4, 9 and the per-unit evaluation).

`pcgen_live_files=0` is non-increasing against every prior cycle. `builds_recorded=1` — the one
build is `verify.sh`'s.

---

## What must happen before AT-35-E7-001 can be re-run

1. **Close the 54 substantive non-`complete` kanban rows** (S1) — row 26 and its 53 descendants
   first. Relabelling them to make the scan pass is `§5`'s forbidden move run in reverse.
2. **Obtain the operator ruling on `_refused.json`** (S2, and it disposes of S3): either convert
   the 142 `no_corpus_record` units, or amend `§3a`'s "empty" to "zero non-`DONE` refusals".
   **Asked twice now — cycle 2 and this cycle.**
3. **Dispose of the 88 open deferrals** (S4), naming which, if any, defer DoD scope.
4. **Add the scope-gate line to the two Epic 4 receipts** (S5).
5. **Produce the three named oracle-parity artifacts, or amend `§3a` by ruling** (S6).
6. **Regenerate `completion-manifest.json` at HEAD to 49,450 units** (S7), then re-draw the
   200+ sample and evaluate each sampled rule.
7. **Get `verify.sh` green** (S8) — re-run `root-full` and classify
   `sd26_pilot_case_verification`'s PCGen-gradle failure, rather than bucketing it.

Then run `AT-35-E7-001` in full, **including `§3` steps 1, 4, 9 and the per-unit rule
evaluation**.
