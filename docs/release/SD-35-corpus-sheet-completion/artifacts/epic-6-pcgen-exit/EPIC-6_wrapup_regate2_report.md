# Epic 6 — PCGen exit — wrap-up gate report, **re-gate 2**

> ## SUPERSEDED — do not read this report's verdict as current
>
> Superseded by `EPIC-6_wrapup_gate_report_regate5.md`, the live one. Stamped by the Epic 6
> wrap-up correction cycle `AT-35-E6-WRAPUP-FIX2`, which found this report still unfolded in its
> author's worktree (`.claude/worktrees/wf_291be5c8-5f3-90`) and folded it here so the record is
> complete rather than dying with the worktree.
>
> It describes an earlier tree and an earlier red list. The current verdict at `4b69eb7aab` was
> 48 of 49 stages PASS with one red, `shape-engine-boundary-selftest`, which
> `AT-35-E6-WRAPUP-FIX2` fixed. Kept, not deleted: these are the record of how the red list was
> driven down.


Authored by the **isolated read-only worker** (`workflow-instruction.md §2` worker split,
`decisions.md §3`). This worker **committed nothing and pushed nothing**. Everything below ran in
its own git worktree at
`/home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_291be5c8-5f3-90`, hard-reset to
`origin/tranche/15`, with its own `CARGO_TARGET_DIR=/tmp/cargo-sd35-epic-6-wrapup` and
`CARGO_INCREMENTAL=0`.

This is the **second** re-run of the Epic 6 wrap-up gate.

| Run | Tree | Result |
|---|---|---|
| Gate 1 (`AT-35-E6-WRAPUP`) | `77e8d3919a` | FAIL — 4 of 49 stages red (`site-dashboard-check`, `figure-provenance`, `desktop`, `clippy`) |
| Repair (`AT-35-E6-WRAPUP-FIX` cycle 1) | `952b313bbb` + `c8ce4f7f12` | full gate PASS on the shared checkout |
| Re-gate 1 | `24084e1782` | PASS, 49/49, 2 h 21 m 36 s |
| **Re-gate 2 (this report)** | **`8d0d4acbf2`** | **PASS, 49/49, 2 h 11 m 09 s** |

Re-gate 1 is not a substitute for this run: **four commits landed after it**, two of them touching
Rust and one touching the residue instrument itself —
`git log --oneline 24084e1782..8d0d4acbf2` → `407a83bf2c`, `fdc90243f4`, `2f824171b5`,
`7fadc67843`, `5da55c42e3`, `af4e315b84`, `8d0d4acbf2`, with
`git diff --stat 952b313bbb..8d0d4acbf2` naming `scripts/pcgen_residue_gate.py` (+178/−…),
`scripts/tests/test_pcgen_residue_gate.py` (+134), `src/bin/gen_desktop_fixture_corpus.rs` (+123/−…)
and `src/bin/gen_settled_corpus.rs` (+28/−…). Re-gate 1's figures describe a tree that predates
ruling B17's shipped-data widening.

- **Tree under test:** `origin/tranche/15` at **`8d0d4acbf2`**
  ("docs(sd35): stamp AT-35-E6-004 cycle 2's landed SHA in its receipt, progress and kanban rows")
- **Gate run:** `scripts/verify.sh -j 6` — **every stage, no `--only`**
- **Wall time:** **7,869 s (2 h 11 m 09 s)**, 2026-09-14T13:34:59Z → 2026-09-14T15:46:08Z.
  Re-derive: `python3 -c "print(int(open('/tmp/sd35-e6-gate-end.txt').read())-int(open('/tmp/sd35-e6-gate-start.txt').read()))"`.
  Independently corroborated by `verify.sh`'s own emitted event
  `1789400768911-epic-6-wrapup-ba3af0`, `duration_seconds: 7869`.
- **Console log (this worker's):** `/tmp/sd35-e6-verify-console.log`
- **Per-stage logs (verify.sh's own scratch dir):** `/tmp/codex-verify-tPQ50k/`
- **Result:** **`RESULT: PASS`**, `VERIFY_EXIT=0`. **49 of 49 stages passed, 0 failed.**

---

## 0. The full gate, once — stage table

Every row is the literal `PASS`/`FAIL` line `scripts/verify.sh` printed. Re-derive with
`grep -nE '^ +(PASS|FAIL) ' /tmp/sd35-e6-verify-console.log`.

| # | Stage | Result | Detail |
|---|---|---|---|
| 1 | preflight-disk | PASS | disk budget OK |
| 2 | preflight-oracle | PASS | oracle at pin `7f818006e371188e5717fd18d74d18a420747fc6` |
| 3 | oracle-pin-selftest | PASS | 11 passed, 0 failed |
| 4 | producer-selftest | PASS | 30 cases passed |
| 5 | pi-redaction-selftest | PASS | 49 cases passed |
| 6 | provenance-selftest | PASS | 32 cases passed |
| 7 | site-dashboard-selftest | PASS | 13 passed, 0 failed |
| 8 | site-dashboard-pin | PASS | `docs/work-inventory.json` matches the pin the feed was published from |
| 9 | site-dashboard-check | PASS | `site/dashboard/PF1e-dashboard.json` is current |
| 10 | site-dashboard-pi-gate | PASS | 22 files scanned against 1,612 declared-PI names, zero leaked |
| 11 | build-public-status-selftest | PASS | 37 cases passed |
| 12 | site-public-status-check | PASS | `site/status-data.json` and `site/status-data/*.json` are current |
| 13 | site-public-status-pi-gate | PASS | 31 files scanned against 1,612 declared-PI names, zero leaked |
| 14 | site-asset-stamp-check | PASS | `site/*.html` cache-busting stamps match `site/styles.css` |
| 15 | reachability-audit-selftest | PASS | 11 cases passed |
| 16 | reachability-audit | PASS | reachable ceiling 100.00%, i.e. 49,438 of the 49,438-unit inventory |
| 17 | groundtruth-guard-selftest | PASS | 17 cases passed |
| 18 | supersession-gate-selftest | PASS | 16 cases passed |
| 19 | shape-coverage-standing-gate-selftest | PASS | 20 cases passed |
| 20 | shape-coverage-standing-gate | PASS | population=2485 unclassified=0 no_record=0 corpus_sha=`7f818006e3…` |
| 21 | cycle-scope-gate-selftest | PASS | 51 cases passed |
| 22 | shape-engine-boundary-selftest | PASS | 15 cases passed |
| 23 | shape-engine-boundary | PASS | magnitude_bearing=26396 **not_held_by_engine=0** citation_ok=True |
| 24 | missing-engine-tables | PASS | population=0 kinds=0 citation_failures=0 |
| 25 | denominator-gate | PASS | files_checked=**347** violations=0 (re-gate 1: 344) |
| 26 | figure-provenance | PASS | files_checked=**277** figures_examined=**615** violations=0 (re-gate 1: 274 / 614) |
| 27 | **pcgen-residue-gate** | **PASS** | **live_files=0 live_hits=0 verdict=PASS** |
| 28 | token-coverage-selftest | PASS | 14 cases passed |
| 29 | token-coverage | PASS | non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=233 shapes=1 |
| 30 | pi-sweep | PASS | 11 hits over `src/rules_core/rules_tables`, 11 baseline rows |
| 31 | declared-pi-audit | PASS | clean |
| 32 | audit-selftest | PASS | 28 passed, 0 failed |
| 33 | reclaim-selftest | PASS | 13 passed, 0 failed |
| 34 | driver-selftest | PASS | 7 passed, 0 failed |
| 35 | corpus-sweep-selftest | PASS | 15 passed, 0 failed |
| 36 | corpus-trap-audit-selftest | PASS | 14 passed, 0 failed |
| 37 | root-lib | PASS | 3,390 passed |
| 38 | root-full | PASS | **8,919 passed across 419 suites, all 365 `tests/*.rs` suites executed** |
| 39 | desktop | PASS | **570 passed** against the corrected floor of 570 |
| 40 | reach | PASS | 32 passed |
| 41 | corpus-sweep | PASS | 48,706 records examined of 51,523 read, 413,314 tokens compared (9 synthesized), 51,463 digests checked, **0 findings** |
| 42 | sheet-rules-check | PASS | records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 (116.9 s) |
| 43 | corpus-trap-audit | PASS | records_examined=27681; `wiring-class-mismatch=0 disabled-line=165 key-differs-from-name=650 mod-record=2117 shared-name-distinct-records=249`; traps=407 — all at registered counts |
| 44 | supersession-gate | PASS | 116 objects, all clean |
| 45 | frontend-install | PASS | `npm ci` (`node_modules` absent in this fresh worktree) |
| 46 | frontend-test | PASS | 101/101 files |
| 47 | frontend-typecheck | PASS | `tsc --noEmit` clean |
| 48 | clippy | PASS | **root:0 desktop:0** warnings, 0 errors |
| 49 | class-dump | PASS | 31/31 computing |

**Red stages: none.** Every stage that was red at gate 1 is green here for the second consecutive
independent run, now on a tree four commits newer. **No wrap-up correction cycle is owed for
Epic 6.**

### 0a. This worker's own base was wrong before the gate ran — 10th firing

Before any measurement, this worktree was found to be at **`da307a1b74`** (a merge of PR #389 on
the `develop` line), **not an ancestor of `origin/tranche/15`**:
`git merge-base --is-ancestor HEAD origin/tranche/15` → non-zero;
`git merge-base --is-ancestor da307a1b74 origin/tranche/15` → `NOT_CONTAINED`.

On that base `scripts/pcgen_residue_gate.py` exists but **predates ruling B17's shipped-data
widening** (`2f824171b5`), so the mandated residue line would have been measured by the older
instrument on the older tree and every figure in this report would have described a tree nobody is
shipping.

Cleared by `git fetch origin tranche/15 && git reset --hard origin/tranche/15`, tree clean
(`git status --porcelain` → 0 lines) before the gate started. Logged as incident
`1789393170272-at-35-e6-wrapup-regate-2-ccbf58`, recurrence key `wrong-base-worktree`, marked
`silent` — **the 10th recurrence in the SD-35 window**, and the first *since* re-gate 1 named the
control. Re-gate 1's escalation is therefore not merely unadopted; the very next worker of the same
role hit the same defect. See §2.

---

## 1. Retrospective read

Command as dispatched: `python3 scripts/retro.py summary --since 53296d80f0 --json`.

**Correction, verified.** `retro.py summary --since` does **not** accept a git SHA — it returned
`retro: cannot parse time '53296d80f0' (try 7d, or 2026-07-01)` and produced no summary. The SHA
was resolved to its commit timestamp and the window re-run:

```
git log -1 --format='%H %ad' --date=iso 53296d80f0
  -> 53296d80f05fe8c32e867f90dd5064a125187f32 2026-09-07 21:24:13 -0400
python3 scripts/retro.py summary --since 2026-09-07T21:24:13-04:00 --json
```

Window: `2026-09-07T21:24:13-04:00` → open. Log: `docs/retro/events`, **66 shards, 0 invalid
lines, 0 problems**.

### 1.1 Counts

| Metric | Bundle window (since `53296d80f0`) | Epic 6 actors only |
|---|---|---|
| Events, total | **585** | **218** |
| — correction | 203 | 113 |
| — deferral | 84 | 60 |
| — incident | 60 (**578 min lost**, 4 silent) | 26 |
| — verification | 184 | 14 |
| — note / resolution / rework | 32 / 16 / 6 | 2 / 0 / 3 |
| Deferrals open / resolved | **83 / 1** | 60 / 0 |
| Near-misses | 0 recorded, 0 escaped | — |
| `verify.sh` runs / failed | **184 / 22** — fail rate **11.96%** | — |

Epic 6 spans **2026-09-09T23:17:16Z → 2026-09-14T13:27:40Z** across **13 actor ids**
(`AT-35-E6-001`, `-001-c3`, `-002`, `-003`, `-003-FINISH`, `-003-RULED`, `-003-SWEEP`
(+ lowercase variant), `-004`, `-005-SHIPPED-DATA`, `-WRAPUP`, `-WRAPUP-FIX`, and this
`-WRAPUP-REGATE-2`). Epic 6 accounts for **113 of the window's 203 corrections (55.7%)** and
**60 of its 84 deferrals (71.4%)**.

`caught_before` for corrections: implementation 26, merge 14, release 5, brief 3, **"nothing
(already shipped)" 1**. That single already-shipped row remains the window's most expensive
correction and the only one with no gate in front of it.

`by_failing_stage` across the 22 failed runs: **figure-provenance 14**, site-dashboard-check 8,
reachability-audit-selftest 2, shape-engine-boundary-selftest 2, clippy 1, denominator-gate 1,
desktop 1, pcgen-residue-gate 1.

### 1.2 Incident keys firing 3+ times — each gets a control or a named escalation

`§10 step 1` is binding: 3+ firings is a missing mechanism, not bad luck (AGENTS.md rule 8).
Counts are from `incidents.by_recurrence_key` in the window JSON.

| Key | Firings (window) | Disposition |
|---|---|---|
| `wrong-base-worktree` | **8**, **10 counting re-gate 1's and this run's** | **ESCALATION, named, and now overdue.** Two halves. (a) `.worktrees/` in `.gitignore` — the prepared one-line fix, refused by every cycle because it is outside the epic's granted file-touch set (`workflow-instruction.md §3`, AGENTS.md rule 4); `AT-35-E6-WRAPUP-FIX` corrected the escalation itself, finding the real gap is that the exclusion currently lives machine-locally in `.git/info/exclude`. (b) The base-freshness half, which `.gitignore` does not touch: **make ancestry a precondition, not a habit** — every dispatched cycle and gate worker runs `git fetch origin <branch> && git merge-base --is-ancestor HEAD origin/<branch>` as its first command and **refuses to measure** on non-zero. Two lines, no new write scope beyond the dispatch template. **This run is the evidence it is needed: it was named at re-gate 1 and the next worker of the same role hit it anyway.** |
| `site-dashboard-json-stale-after-inventory-move` | **6** | **Controlled.** `site-dashboard-pin` (stage 8, <1 s input-pin gate now in every cycle's push gate, per `AT-35-E5-WRAPUP-FIX`'s resolution event) and `site-dashboard-check` (stage 9) are both in the full gate and both green here. The 6 firings are pre-control. No further action. |
| `disk-full` | **5** in this window (169 lifetime) | Partly controlled. `preflight-disk` is stage 1 and passed: 802 G used of the 1.5 T `/dev/sda1` (56%), 651 G free, rising to 838 G used of the same 1.5 T (58%), 615 G free, at the gate's peak. The residual gap is that the preflight guards *verify.sh*, not the per-agent `CARGO_TARGET_DIR` allocations that actually fill the disk — this worker's own was 36 G. `.reclaim-claim` files exist (this worker wrote one); **the missing half is a reaper that deletes a claim dir whose claiming PID is gone.** Named, not built. |
| `epic-wrapup-gate-red` | **5** | **Controlled by design, and this report is the evidence.** `§10 step 0`'s "red is fixed in a wrap-up correction cycle before the next epic's second cycle" *is* the mechanism. For Epic 6 it ran once (`AT-35-E6-WRAPUP-FIX`) and has now been independently confirmed **twice** — 49/49 at `24084e1782` and 49/49 at `8d0d4acbf2`. Not a defect class; the protocol working. |
| `untracked-worktrees-dir-on-shared-checkout` | **5** | Folded into the `wrong-base-worktree` escalation above — same blocked `.gitignore` line, plus `AT-35-E6-WRAPUP-FIX`'s correction that the live exclusion is machine-local in `.git/info/exclude` and therefore invisible to every fresh checkout. |
| `figure-provenance-command-on-next-line` | **4** | **Control exists and is enforced.** `figure-provenance` is stage 26 and is the single most frequent red (14 of 22 failed runs) — which is what an effective gate looks like: it catches authoring drift before merge. Green here at files_checked=277, figures_examined=615, violations=0. **Improvement named, not built:** the checker names the violating file and figure but not the accepted row shape; a `--explain` example in the failure text would remove most of the 4. |
| `retro-actor-lost-between-bash-calls` | **3** | **Mechanical control, adoptable today.** `RETRO_ACTOR` does not survive into a `nohup`'d subshell, which is how gate workers run `verify.sh`; three of this window's `verify.sh` events misfiled to `sd31-transcribe.jsonl`. This worker avoided it by exporting `RETRO_ACTOR` **inside the wrapper script** (`/tmp/sd35-e6-run-gate.sh`) rather than in the calling shell — verify.sh's own event landed correctly at `docs/retro/events/epic-6-wrapup.jsonl`. **Durable fix:** `retro.py` should fall back to a `.retro-actor` file written once at cycle start when `$RETRO_ACTOR` is unset, instead of silently attributing to whoever wrote the shard last. Named for the closure epic. |

Two structural classes below the threshold, both of which fired again during this run:

- **`unfolded-gate-artifacts-die-with-the-worktree` — 2nd firing** (this run,
  `1789393316017-at-35-e6-wrapup-regate-2-244ddf`). Re-gate 1's evidence **is still not in
  `tranche/15`**: `git log --oneline -1 -- …/EPIC-6_wrapup_regate1_report.md` returns nothing and
  `docs/retro/events/at-35-e6-wrapup-regate-1.jsonl` is absent from the branch. Both live only in
  sibling worktree `…-91` and in `/home/ubuntu/workspace/sd35-regate-logs/handoff/`. The
  isolated-worker split hands outputs to the orchestrator and **nothing mechanical checks the
  hand-off landed**. Control named: the wrap-up correction cycle should assert the previous gate
  worker's shard is present in `docs/retro/events/` before it runs.
- **`epic-wrapup-worktree-sweep-unreachable-from-isolated-worker` — 2nd firing** (this run,
  `1789393180832-at-35-e6-wrapup-regate-2-c73577`). See §2.

### 1.3 Cycles whose `rust_lines_changed / units_closed` exceeded 3.0

**None — and the reason is structural, not evasive.**

Re-derived over all **65** Epic 6 cycle receipts (re-gate 1 saw 63; two landed since) by parsing
the mechanical row each carries:

```
cd docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit
python3 - <<'EOF'
import re,glob
rows=[]
for f in sorted(glob.glob('*_receipt.md')):
    m=re.search(r'closed=(\d+) relabeled=(\d+) rust_lines_changed=(\d+) ratio=(\S+) '
                r'builds_recorded=(\d+) pcgen_live_files=(\d+)', open(f).read())
    if m: rows.append((f,)+m.groups())
print(len(rows), len(glob.glob('*_receipt.md')),
      sum(1 for r in rows if int(r[1])>0), sorted({r[4] for r in rows}),
      sum(int(r[3]) for r in rows), sum(int(r[5]) for r in rows))
EOF
```

- receipts carrying a mechanical row: **65 of 65**
- cycles with `closed > 0`: **0**
- distinct `ratio` values across all 65: **`{n/a}`** — every one a division by zero
- total `rust_lines_changed` across Epic 6: **46,449**
- total `builds_recorded`: **87**
- `pcgen_live_files`: **254 → 0**

**What the 46,449 lines bought.** `decisions.md §4`'s ratio test assumes a *content* epic, where
Rust lines are spent to close corpus units. Epic 6 closes **zero** units by design — it is the
PCGen exit, and its unit of progress is `pcgen_live_files`, not `units_closed`. Reporting
"ratio = n/a" and stopping would be a carve-out, so the honest accounting is the substitute
denominator:

- **254 → 0 live PCGen-reading files** across `src/rules_core` (minus `cache_gen`),
  `src/saved_character`, `src/campaign`, `src/homebrew_authoring` and `apps/desktop`. At 46,449
  Rust lines that is **≈183 lines per file evicted** — the cost of *replacing* a token read with a
  converted, engine-held value rather than deleting the read and leaving a hole.
- **The tool side is intact**, as `decisions.md §11` requires: `src/pcgen_import/` holds **70**
  `.rs` files (`find src/pcgen_import -name '*.rs' | wc -l`), and
  `scripts/fetch-pcgen-oracle.sh`, `scripts/pcgen-run-character.sh`,
  `scripts/pcgen-normalize-output.py` are all present. The residue gate's two-population design —
  `live_*` must be 0, `baseline_*` (260 files / 12,736 hits, unchanged) must **not** be driven to
  0 — is what makes "we removed PCGen from the live side" and "we deleted the converter"
  distinguishable.
- **Nothing on the sheet moved:** `sheet-rules-check` reads `records=49438 converted=49296
  refused=142`, `corpus-sweep` reports **0 findings** over 413,314 compared tokens, and
  `shape-engine-boundary` reports `not_held_by_engine=0`. 46,449 lines of live-side rewrite with a
  byte-identical rendered result is the correct outcome for an exit epic.

Largest single cycles by Rust volume, each a mass token-eviction pass carrying its own
`pcgen_live_files` reading: `AT-35-E6-003-SWEEP` cycle 3 (**4,020**, live_files 75),
cycle 4 (**2,920**, 73), `AT-35-E6-001` cycle 4 (**2,520**, 253) and cycle 2 (**2,173**, 253),
`AT-35-E6-003` cycle 12 (**1,978**, 197).

### 1.4 The 60 open Epic 6 deferrals are remainder snapshots, not open scope

`deferrals.open` reads **83** window-wide, **60** of them Epic 6's. Read one at a time they look
alarming; read in sequence they are a **single running remainder**, re-stated each cycle and
superseded by the next: 251 hits → … → 19 → 14 → 12 → 10 → 5 → 4 → **0**. **54 of the 60** are
residue-remainder shaped (`pcgen_import` / `raw_tokens` / live-hit counts). The terminal
measurement settles all of them: `pcgen_residue_gate.py --check --closure` at `8d0d4acbf2` reads
`live_files=0 live_hits=0`, and stage 27 of this gate agrees.

**The gap is bookkeeping, and it is real:** nobody emitted the closing `resolution` events, so the
log will tell SD-35's retrospective that 60 Epic 6 deferrals stand open when the instrument says
zero. **Named for `AT-35-E7-002`:** emit `resolution` events against the superseded remainder
deferrals, citing `pcgen_residue_gate.py --check --closure` as the verification.

The **six non-residue** Epic 6 deferrals are the substantive ones. Five are catalog-swap refusals
(`spell_catalog`/`feat_catalog` revert, two spells losing a served description,
`reference_library_catalog` losing 1,150 of 9,679 descriptions, 26 `class_feature` records, and
the 251-hit desktop remainder) — all overtaken by `live_files=0`. **One stands genuinely open and
is not Epic 6's to close:** `at-35-e6-003-sweep`, 2026-09-12 — `v06_work_inventory` classifies a
unit *text-complete* **before** it reaches the *sheet-complete* rung, so the `class_feature`
catalog widening (4,463 → 5,256 served records) shadows 230 units. That is a rung-ordering defect
in the inventory instrument. **Carry to Epic 7.**

---

## 2. Worktree sweep — **attempted, refused, escalated (2nd firing)**

`df -h /`: **802 G used of the 1.5 T `/dev/sda1`, 651 G available (56%)** before the gate, rising
to **838 G used of the same 1.5 T, 615 G available (58%)** at its peak. No disk pressure.

`git worktree list` — 15 entries. **Epic 6's** are those created after Epic 6's first event
(2026-09-09T23:17Z = 2026-09-09 19:17 EDT), by directory mtime:

| Worktree | HEAD | mtime | Size | Merged into `tranche/15`? | Disposition |
|---|---|---|---|---|---|
| `…-19` | `b78b076b2e` | 2026-09-10 07:49 | 888 M | **yes** | **RETAINED** |
| `…-89` | `77e8d3919a` | 2026-09-13 18:28 | 911 M | **yes** | **RETAINED** — Epic 6's first gate worker |
| `…-91` | `24084e1782` | 2026-09-13 23:33 | 911 M | **yes** | **RETAINED** — holds re-gate 1's **unfolded** report |
| `…-90` | `8d0d4acbf2` | 2026-09-14 09:32 | 767 M | **yes** | **RETAINED** — this worker, `locked` |

The remaining 10 (`…-2 -3 -4 -5 -7 -14 -15 -21 -24 -30`) all predate Epic 6 and are out of this
sweep's scope per `§10 step 2` ("this epic's worktrees only"). Note `…-30` separately:
`AT-35-E6-WRAPUP-FIX` recorded it as carrying **uncommitted live Rust WIP across 13 `src/` files,
2 of them absent in the repo** — it must not be removed by anyone.

**This run got further than re-gate 1 did, and still could not sweep.**

1. **Merged-ness established, contra re-gate 1.** `git branch --merged origin/tranche/15 --list
   'worktree-*'` runs fine from inside an isolated worktree and proves **all 13 sibling worktree
   branches carry zero unmerged commits**. The "unmerged commits" half of the safety rule is
   satisfied.
2. **Removal actually attempted** on all three Epic 6 worktrees. All three refused identically:
   `fatal: '…' contains modified or untracked files, use --force to delete it`.
3. **`--force` refused, deliberately.** Forcing is precisely the move
   `unfolded-gate-artifacts-die-with-the-worktree` forbids, and the risk is not hypothetical here:
   a `find`-diff of each sibling's `artifacts/` tree against this one surfaced
   `…-91/docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/EPIC-6_wrapup_regate1_report.md`
   — **331 lines, the entire 49/49 evidence for re-gate 1, never committed to any branch.**
   One `--force` would have destroyed it.

**Escalation (unchanged from re-gate 1, now with a second firing behind it,
`1789393180832-at-35-e6-wrapup-regate-2-c73577`):** `workflow-instruction.md §10 step 2` assigns a
destructive, fold-first step to the one role structurally unable to fold. **Asked of the operator:
move §10 step 2 off the isolated read-only worker and onto the LOCAL wrap-up correction cycle,**
which runs on the shared checkout, already folds worktree artifacts, and can therefore
fold-then-remove. The isolated worker keeps only the *reporting* half — `df -h` plus the inventory
and merged-ness table above.

**Carried to that cycle, in order:** fold `…-91`'s report and retro shard **and** this worker's
(§5), then remove `…-89` and `…-91`; `…-19` after checking what it holds; never `…-30`.

---

## 3. Pull request

**None.** `workflow-instruction.md §10 step 3`: no PR at an epic wrap-up.

---

## 4. PCGen residue gate — literal output

`python3 scripts/pcgen_residue_gate.py --check` at `8d0d4acbf2`, exit 0:

```
pattern raw_tokens files=0 hits=0
pattern raw_bonus_chains files=0 hits=0
pattern PcgenFormulaEvaluator files=0 hits=0
pattern render_pcgen_desc files=0 hits=0
pattern bonus_stack_reader files=0 hits=0
pattern pre_tokens files=0 hits=0
pattern BONUS: files=0 hits=0
pattern DEFINE: files=0 hits=0
pattern PRE[A-Z]+: files=0 hits=0
pattern SAB: files=0 hits=0
pattern DESC: files=0 hits=0
pattern %CHOICE files=0 hits=0
pattern %LIST files=0 hits=0
pattern TYPE= files=0 hits=0
pattern pcgen_import files=0 hits=0
root src/rules_core files=0 hits=0
root src/saved_character files=0 hits=0
root src/campaign files=0 hits=0
root src/homebrew_authoring files=0 hits=0
root apps/desktop files=0 hits=0
identifier_files=0 identifier_hits=0
shipped_data_files=0 shipped_data_hits=0 shipped_scanned=11
live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS
```

**Residue line:**
`live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS`

Closure mode agrees: `python3 scripts/pcgen_residue_gate.py --check --closure` → exit 0,
`shipped_data_files=0 shipped_data_hits=0 shipped_scanned=11`, `live_files=0 live_hits=0
verdict=PASS`.

**Independent census of the shipped-data class (AT-35-E6-005-SHIPPED-DATA's evidence bar).**
Derived from `bundle.resources` in `apps/desktop/src-tauri/tauri.conf.json` — 5 entries, walked
recursively and **de-duplicated** (the entries overlap: `resources/corpus_fixtures/` and three of
its own subdirectories are listed separately, so a naive walk double-counts to 16) — yielding
**11 unique shipped files**, matching the gate's `shipped_scanned=11` exactly. Grepped whole for
`"raw_tokens"`, `"raw_bonus_chains"`, `BONUS:`, `DEFINE:`, `PRE[A-Z]+:`, `SAB:`, `DESC:`,
`%CHOICE`, `%LIST`: **0 files with hits, 0 total hits.**

The live side is at zero, the shipped data is at zero, and the baseline side is **unmoved** at
260 files / 12,736 hits — the converter, parser, generators and oracle harness were kept, as
`decisions.md §11` requires.

---

## 5. Verdict

**`complete`.** 49 of 49 `scripts/verify.sh` stages passed at `origin/tranche/15` @ `8d0d4acbf2`
in 2 h 11 m 09 s. No red stage. **No wrap-up correction cycle is owed for Epic 6**, and Epic 7 is
unblocked on this axis.

Three items leave this gate as **named escalations requiring an operator ruling**; none blocks
Epic 7:

1. **`wrong-base-worktree` (10 firings).** Grant the `.gitignore` `.worktrees/` line (the live
   exclusion is currently machine-local in `.git/info/exclude`) **and** adopt a base-ancestry
   precondition in the dispatch template: `git merge-base --is-ancestor HEAD origin/<branch>` must
   pass before any measurement. Re-gate 1 named this and the next worker of the same role hit it
   anyway — it is now a demonstrated, not a predicted, failure.
2. **Reassign `§10 step 2`** (worktree sweep) from the isolated read-only worker, which the harness
   forbids from touching sibling worktrees, to the local wrap-up correction cycle.
3. **Hand-off has no receipt.** Re-gate 1's report and retro shard never reached `tranche/15`.
   The wrap-up correction cycle should assert the previous gate worker's shard is present in
   `docs/retro/events/` before it runs.

One **content** item is carried forward, and it is not Epic 6's to close: the `v06_work_inventory`
rung-ordering defect (`at-35-e6-003-sweep`, 2026-09-12) that classifies a unit text-complete before
the sheet-complete rung, shadowing 230 units behind the `class_feature` catalog widening. **Carry
to Epic 7.** Also for `AT-35-E7-002`: emit `resolution` events against the 54 superseded Epic 6
remainder deferrals, whose scope the closure gate measures at zero.

This worker committed nothing and pushed nothing.

### To fold (exactly two untracked files; `git status --porcelain` in this worktree)

| File | Contents |
|---|---|
| `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/EPIC-6_wrapup_regate2_report.md` | this report |
| `docs/retro/events/at-35-e6-wrapup-regate-2.jsonl` | **5** events — 3 incidents: `1789393170272-…-ccbf58` (`wrong-base-worktree`, silent), `1789393180832-…-c73577` (`epic-wrapup-worktree-sweep-unreachable-from-isolated-worker`), `1789393316017-…-244ddf` (`unfolded-gate-artifacts-die-with-the-worktree`, silent); and 2 corrections: `1789400980366-…-173e90` (this report's own first draft, 3 denominator-gate violations) and `1789400989018-…-5e5d45` (the dispatch prompt's `--since 53296d80f0`, §1) |

Plus one file `verify.sh` wrote itself, which is also untracked and must be folded with them:
`docs/retro/events/epic-6-wrapup.jsonl` — 1 `verification` event
(`1789400768911-epic-6-wrapup-ba3af0`, `mode: full`, `result: PASS`, `duration_seconds: 7869`).

**Still unfolded from re-gate 1** and not this worker's to commit, but it dies with worktree
`…-91`: `EPIC-6_wrapup_regate1_report.md` and `at-35-e6-wrapup-regate-1.jsonl`, both also copied to
`/home/ubuntu/workspace/sd35-regate-logs/handoff/`.

All three of this run's files are copied, outside the worktree so they survive its cleanup, to
`/home/ubuntu/workspace/sd35-regate-logs/handoff-regate2/`.
