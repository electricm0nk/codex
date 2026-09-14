# Epic 4 — Resolve and verify — wrap-up **re-gate**, 2026-09-10

`workflow-instruction.md §10` steps 0–3, run on the **isolated read-only worker**
(`§2` worker split, `decisions.md §3`). The worker pushed nothing and committed nothing; this
file is handed to the next cycle agent to commit, together with the retro shard named at the end.

- **Status: COMPLETE — full gate GREEN, 49 of 49 stages PASS, 0 FAIL.**
- **Tree under test:** `a4efb1a91b` — `docs(sd35): pin AT-35-E4-003 cycle 2's commit SHA in its
  own receipt`, the tip of `origin/tranche/15` at gate start.
- **Command:** `scripts/verify.sh -j 3` — **every stage, no `--only`**. `VERIFY_EXIT=0`,
  `RESULT: PASS`.
- **Wall time: 4,862 s = 1 h 21 m 02 s** (started `2026-09-10T11:50:11-04:00`, ended
  `2026-09-10T13:11:13-04:00`).
- **Worker worktree:** `.claude/worktrees/wf_291be5c8-5f3-27` (`locked`).
- **`CARGO_TARGET_DIR`:** `/tmp/cargo-sd35-epic4-wrapup-regate-0910` (own directory, 34 G at peak;
  `.reclaim-claim` written).
- **Run log:** `/tmp/sd35-e4-regate-verify.log`; per-stage logs `/tmp/codex-verify-vfZYzD/`.
  Both are on this worker's `/tmp` and do **not** survive teardown, so every stage's own
  result line is transcribed verbatim below.

## Read this before reading the stage table — what tree this actually gates

This dispatch is a **re-dispatch**. Epic 4's wrap-up gate already ran on 2026-09-09
(`EPIC-4_wrapup_gate_report.md`, 47 PASS / 1 FAIL `figure-provenance`), its red stage was already
fixed by the wrap-up correction cycle (`1f4c0ad9fe` / `20812af147`,
`EPIC-4_wrapup_correction_cycle_receipt.md`), and that cycle already re-ran the gate **green at
48 of 48**. Both artifacts are committed on `origin/tranche/15`.

Re-running at Epic 4's own tip (`5e2c0c8c5b`) would therefore have re-proved a settled,
59-commits-stale result. This run gates **`origin/tranche/15`'s current tip instead**, which
carries all of Epic 5 and Epic 6 on top of Epic 4 — including `AT-35-E6-001`'s `src/pcgen_import`
module move, the Epic 5 wrap-up correction, and the Epic 2 re-gate. Read the stage table as
"the bundle is green at HEAD today", not as "Epic 4's tip is green" — that second claim was
already established, and this run does not weaken it.

The stage count moved 48 → 49 between the two runs: `site-dashboard-pin` is new (added by the
Epic 5 wrap-up correction `6e4b1f7b4e` as the named control for
`site-dashboard-json-stale-after-inventory-move`). It PASSED.

---

## Step 0 — the full gate, once

### Stage table (49 stages, in run order) — every line as `verify.sh` printed it

| # | stage | result |
|---|---|---|
| 1 | preflight-disk | PASS — disk budget OK |
| 2 | preflight-oracle | PASS — oracle at pin `7f818006e371188e5717fd18d74d18a420747fc6` |
| 3 | oracle-pin-selftest | PASS — 11 passed, 0 failed |
| 4 | producer-selftest | PASS — 30 cases passed |
| 5 | pi-redaction-selftest | PASS — 49 cases passed |
| 6 | provenance-selftest | PASS — 32 cases passed |
| 7 | site-dashboard-selftest | PASS — 13 passed, 0 failed |
| 8 | **site-dashboard-pin** *(new stage)* | PASS — `docs/work-inventory.json` matches the pin the feed was published from |
| 9 | **site-dashboard-check** | **PASS** — `site/dashboard/PF1e-dashboard.json` is current |
| 10 | site-dashboard-pi-gate | PASS — 22 files scanned against 1,612 declared-PI names, zero leaked |
| 11 | build-public-status-selftest | PASS — 37 cases passed |
| 12 | site-public-status-check | PASS — `site/status-data.json` and `site/status-data/*.json` are current |
| 13 | site-public-status-pi-gate | PASS — 31 files, zero leaked |
| 14 | site-asset-stamp-check | PASS — `site/*.html` cache-busting stamps match `site/styles.css` |
| 15 | reachability-audit-selftest | PASS — 11 cases passed |
| 16 | reachability-audit | PASS — reachable ceiling 100.00% (49,438 of 49,438 units; `/tmp/codex-verify-vfZYzD/reachability-audit.log` line 1 `reachability_audit: 49438 units`) |
| 17 | groundtruth-guard-selftest | PASS — 17 cases passed |
| 18 | supersession-gate-selftest | PASS — 16 cases passed |
| 19 | shape-coverage-standing-gate-selftest | PASS — 20 cases passed |
| 20 | shape-coverage-standing-gate | PASS — population=2485 unclassified=0 no_record=0 corpus_sha=`7f818006e3…` |
| 21 | cycle-scope-gate-selftest | PASS — 51 cases passed |
| 22 | shape-engine-boundary-selftest | PASS — 15 cases passed |
| 23 | shape-engine-boundary | PASS — magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True |
| 24 | missing-engine-tables | PASS — population=0 kinds=0 citation_failures=0 |
| 25 | denominator-gate | PASS — files_checked=267 violations=0 |
| 26 | **figure-provenance** | **PASS** — files_checked=197 figures_examined=363 violations=0 |
| 27 | pcgen-residue-gate | PASS — live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS |
| 28 | token-coverage-selftest | PASS — 14 cases passed |
| 29 | token-coverage | PASS — non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=231 shapes=1 verdict=PASS |
| 30 | pi-sweep | PASS — 11 hits over `src/rules_core/rules_tables`, 11 baseline rows |
| 31 | declared-pi-audit | PASS — clean |
| 32 | audit-selftest | PASS — 28 passed, 0 failed |
| 33 | reclaim-selftest | PASS — 13 passed, 0 failed |
| 34 | driver-selftest | PASS — 7 passed, 0 failed |
| 35 | corpus-sweep-selftest | PASS — 15 passed, 0 failed |
| 36 | corpus-trap-audit-selftest | PASS — 14 passed, 0 failed |
| 37 | root-lib | PASS — 3,261 passed |
| 38 | root-full | PASS — **8,772 passed across 413 suites, all 361 `tests/*.rs` suites executed** |
| 39 | desktop | PASS — 576 passed |
| 40 | reach | PASS — 32 passed |
| 41 | corpus-sweep | PASS — 48,706 records examined of 51,476 read, 413,314 tokens compared (9 synthesized), 51,463 digests checked, **0 findings** |
| 42 | sheet-rules-check | PASS — records=49438 converted=49296 refused=142 rules=69344 var_tables=5277 verdict=PASS (115.8 s) |
| 43 | corpus-trap-audit | PASS — records_examined=27634, defects[wiring-class-mismatch=0 disabled-line=165 key-differs-from-name=650 mod-record=2117 shared-name-distinct-records=249], traps=407 — all defect kinds at their registered counts |
| 44 | supersession-gate | PASS — 116 objects, all clean |
| 45 | frontend-install | PASS — `node_modules` present |
| 46 | frontend-test | PASS — 101/101 files |
| 47 | frontend-typecheck | PASS — `tsc --noEmit` clean |
| 48 | clippy | PASS — root:0 desktop:0 warnings, 0 errors |
| 49 | class-dump | PASS — 31/31 computing |

`verify.sh` summary line: `passed: 49` / `RESULT: PASS`. **No red stage. No stage was skipped,
and no `--only` was used.** No wrap-up correction cycle is owed.

### The two stages that had been the bundle's standing red, both green here

- **`figure-provenance`** — red at the Epic 2, Epic 3 **and** Epic 4 wrap-ups
  (`figure-provenance-command-on-next-line`, 3 firings; 13 of the window's 23 failed verification
  runs). The control named at `1788964900910-at-35-e4-wrapup-5a211d` — adding
  `python3 scripts/denominator_gate.py --check-provenance` to `workflow-instruction.md §6` step 3's
  per-cycle gate block, which previously ran only the *different* flag `--check` — landed in
  `1f4c0ad9fe`. It has now held through **two consecutive** wrap-up gates (Epic 5's at
  `c3500e7984`, and this one) while the examined population grew from 228 figures to **363**.
  **The control works.**
- **`site-dashboard-check`** — red 10 times in-window (`site-dashboard-json-stale-after-inventory-move`,
  4 incident firings). Its named control, the new `site-dashboard-pin` stage, landed in
  `6e4b1f7b4e`. Both the pin stage and the check stage passed here. **First clean wrap-up gate
  for this stage in the SD-35 window.**

### Baselines

No stale-baseline warning was printed by any stage (`grep -niE 'stale|WARN' ` over the run log
returns only the `clippy` line, which contains the word "warnings" in its PASS text). The suite
floors `verify-baselines.env` carries are satisfied at the measured 3,261 root-lib / 8,772
root-full / 576 desktop / 413 root test binaries.

---

## Step 1 — retrospective summary

`python3 scripts/retro.py summary --since 2026-09-07 --json`. **`--since` takes a time, not a
SHA** — the dispatch named `53296d80f0`, whose commit date is `2026-09-07 21:24:13 -0400`
(`git log -1 --format='%ci' 53296d80f0`), so `2026-09-07` is the window that contains it and
`--since 53296d80f0` exits 1 with `retro: cannot parse time`.

- **Window:** `2026-09-07T00:00:00+00:00` → open. **346 events**, 45 shards, **0 invalid lines,
  0 problems**. 152 agent-origin / 194 derived. Git join: 177 commits, 1 author,
  1.95 events per commit.
- **By type:** verification **173** · correction **82** · incident **38** · deferral **21** ·
  resolution 15 · note 13 · rework 4.
- **Verification runs:** 173 runs, **23 failed — a 13.29% fail rate, 23 of 173**, by failing stage —
  `figure-provenance` 13, `site-dashboard-check` 10, `reachability-audit-selftest` 2,
  `shape-engine-boundary-selftest` 2, `denominator-gate` 1. **This run adds a 174th, passing.**
- **Deferrals: 20 open, 0 resolved in-window.** This gate closes no units and refuses no tokens,
  so it emits **no `deferral` of its own**. The open 20 are Epic 1–3 dispositions; the largest,
  `AT-35-E2-005`'s, names its refused token types: `FORMULA:var(COUNT)`=210,
  `unmapped:STARTSKILLPTS`=119, PI-redacted `SPELLS`=66, `FORMULA:malformed`=62,
  `BONUS:[redacted PI]`=62, of 659 refused. They are **Epic 7's closure business** —
  `workflow-instruction.md §11` step 1 forbids closing over them.
- **Rework: 4**, all Epic 1–2 and already dispositioned in their own receipts.
- **Near misses: 0 recorded, 0 escaped.**

### Incident keys at 3+ firings — each with its named control or escalation

| recurrence key | firings | disposition |
|---|---|---|
| `disk-full` | **14** | Control exists and is running: `preflight-disk` is stage 1 of `verify.sh` and blocks before any cargo cost; `reclaim-selftest` (stage 33) covers the reclaim daemon. Both PASSED here, and this 34 G run never approached the limit (`/` went 79% → 81% of its 1.5 T, 279 G still free at root-full peak). **Not escalated** — the mechanism is built and held. |
| `epic-wrapup-gate-red` | **4** | **Closed by this run.** Both of the two stages behind all four firings (`figure-provenance`, `site-dashboard-check`) now have landed mechanical controls and both are green here. |
| `site-dashboard-json-stale-after-inventory-move` | **4** | Control landed: the `site-dashboard-pin` stage (`6e4b1f7b4e`). PASSED here, alongside `site-dashboard-check`. The 4th firing (`1789034786341-at-35-e2-regate2-26a448`) named a **new sub-mechanism** — the feed went stale with its input pin intact — which the pin stage does not cover. It did not recur here; **flagged to Epic 7, not escalated.** |
| `figure-provenance-command-on-next-line` | **3** | Control landed (`1f4c0ad9fe`, `workflow-instruction.md §6` step 3 now runs `--check-provenance`). Green at two consecutive wrap-ups. |
| `wrong-base-worktree` | **3 → 5** | **ESCALATION — the only one owed.** This run raised it to 5 (see below). Its control was named twice (`1788995389793-at-35-e5-wrapup-156d6c`, and again by this worker) and **has not been built**. |

**The escalation, stated plainly.** `wrong-base-worktree` and its twin
`stale-wip-in-reused-worktree` are the one class in this window with a named control that nobody
has built, and they fired again on this very dispatch. **`scripts/verify.sh` needs a
`preflight-base` stage**, run beside `preflight-disk` and `preflight-oracle`, failing nonzero when
either

1. `git rev-list --count HEAD..origin/<tranche branch>` is non-zero — the tree is behind what it
   claims to gate; or
2. `git status --porcelain` lists any **tracked** modification — the gate would run over
   uncommitted work its report cannot attribute.

Both checks are git-only and sub-second, and both run before any build cost. Per `AGENTS.md`
rule 8 the dispatch-prompt caution that exists today is not a control. Recorded as
`1789055528818-epic4-wrapup-regate-0910-80352e`.

### Cycles whose `rust_lines_changed / units_closed` exceeded 3.0

**None — the ratio is `null` for every Epic 4 cycle, and `null` is not a number above 3.0.**
Source: `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/rate-ledger.json`,
7 rows, verified at `5341630c0e` by `AT-35-E4-003` cycle 2 against the six committed receipts.

| cycle | units_closed | rust_lines_changed | ratio | builds |
|---|---|---|---|---|
| `AT-35-E4-001_cycle1` | 0 | 131 | null | 0 |
| `AT-35-E4-002_cycle1` | 0 | 171 | null | 1 |
| `AT-35-E4-003_cycle1` | 0 | 0 | null | 0 |
| `EPIC-4_wrapup_correction_cycle` | 0 | 0 | null | 1 |
| `AT-35-E4-001_cycle2` | 0 | 0 | null | 0 |
| `AT-35-E4-002_cycle2` | 0 | 0 | null | 0 |
| `AT-35-E4-003_cycle2` | 0 | 0 | null | 0 |
| **totals** | **0** | **302** | **null** | **2** |

Re-derive:
`python3 -c "import json;c=json.load(open('docs/release/SD-35-corpus-sheet-completion/artifacts/epic-4-resolve-and-verify/rate-ledger.json'))['cycles'];print(len(c),sum(x['units_closed'] for x in c),sum(x['rust_lines_changed'] for x in c),sum(x['builds_recorded'] for x in c))"`
→ `7 0 302 2`.

`null` is a **division by zero, never a 0.0**: Epic 4's authoring-time population — bucket M 4,334
plus bucket V 392 = 4,726 of the then-23,315 non-DONE — had already been closed by Epic 3's
`AT-35-E3-001_cycle2` (618 units) and `AT-35-E3-002_cycle1` (786, the whole remainder), so every
Epic 4 cycle opened on a 0-unit scoped population.

**So what did the 302 Rust lines buy, since they closed no unit?** All 302 are in the two cycle-1
rows, and both bought *instrument* movement, which is the epic's actual job ("Resolve and
verify"):

- **`AT-35-E4-001_cycle1`, 131 lines** — 24 converter mapping rows plus one head alias
  (`GLOBALVAR:ABILITY` → the existing `ABILITY` row). Took `unmapped_token_types` **25 → 0** and
  `degraded_records` **974 → 603**. The bought figure is visible in this gate's stage 29:
  `token-coverage … token_types=231 … verdict=PASS`.
- **`AT-35-E4-002_cycle1`, 171 lines** — the corpus-wide oracle harness that produced bucket V's
  owed verdicts: **392 compared, 184 agree, 10 disagree, 198 unverifiable**
  (`AT-35-E4-002_cycle1_bucket-v-parity.json`).

`builds_recorded` totals **2 across 7 cycles** — inside `decisions.md §3`'s one-build-per-cycle
rule, not an overrun. The five zero-Rust rows are docs/verification cycles.

`pcgen_live_files` **fell** 260 → 253 across the epic's ledger window. **A fall is never a rise**
(`decisions.md §11`); the reduction is Epic 6 `AT-35-E6-001`'s, not any Epic 4 cycle's.

---

## Step 2 — worktree sweep

- `df -h /`: **79% of the 1.5 T volume used, 311 G available** at gate start; **81% of 1.5 T, 279 G** at root-full peak.
  This worker's `CARGO_TARGET_DIR` peaked at **34 G**.
- `git worktree list`: **18 entries** — the shared checkout at `a4efb1a91b`, 16
  `.claude/worktrees/wf_291be5c8-5f3-*` workers, and `.worktrees/ci-trait-choice`
  (`fix/trait-choice-set-id-roundtrip`, an unrelated long-lived branch).

**Epic 4's worktrees: 3 found, 0 removable, 0 removed.**

| worktree | HEAD | Epic 4 role | disposition |
|---|---|---|---|
| `wf_291be5c8-5f3-21` | `07e29075b4` | Epic 4's first-cycle base SHA | merged; **not removed** — read-only worker |
| `wf_291be5c8-5f3-30` | `00d0611e87` | Epic 4 wrap-up correction receipt commit | merged; **not removed** — read-only worker |
| `wf_291be5c8-5f3-27` | `a4efb1a91b` | **this worker** | **`locked` — never removed** |

**Nothing was pruned, and that is the correct outcome for this step**, not a skipped chore:
`§2`'s worker split makes this an isolated read-only worker, its isolation guard refuses
cross-worktree git operations by design, and `§10` step 2 scopes the sweep to *this epic's*
worktrees. The bundle-wide prune is `§11` step 3's business at Epic 7.

**Every one of the 15 sibling worker worktrees is an ancestor of `origin/tranche/15`** — verified
individually with `git merge-base --is-ancestor <head> origin/tranche/15`, exit 0 on all **14
distinct** HEAD SHAs the 15 worktrees hold (`wf-…-2, -3, -4, -5, -7, -14, -15, -17, -19, -21,
-23, -24, -30, -35`; `wf-…-41` shares `f1f547a41e` with `wf-…-17`). **None carries an
unmerged commit**, so none is at risk from Epic 7's prune. Whether any carries *uncommitted* work
was **not** determined — this worker cannot read another worktree's `git status` — and Epic 7 must
check that before removing any of them.

## Step 3 — no PR

None opened. Correct for an epic wrap-up (`§10` step 3).

---

## PCGen residue gate

`python3 scripts/pcgen_residue_gate.py --check`, literal final line:

```
live_files=253 live_hits=12256 baseline_files=260 baseline_hits=12736 verdict=PASS
```

Live-side roots at HEAD: `src/campaign` 0/0, `src/homebrew_authoring` 0/0, `apps/desktop`
51 files / 477 hits; identifier scan 54 files / 343 hits. The same gate ran independently as
`verify.sh` stage 27 and returned the identical line.

The live-side count **fell** 260 → 253 files and 12,736 → 12,256 hits against the
`AT-35-E1-005` baseline. `decisions.md §11` forbids a **rise**; this is Epic 6
`AT-35-E6-001`'s `src/pcgen_import` module move taking PCGen reads off the live side, and the
gate reports it as `PASS`. **The converter, parser, generators and oracle harness are intact** —
they are kept for Starfinder; the stage-2 `preflight-oracle` PASS at pin
`7f818006e371188e5717fd18d74d18a420747fc6` and the 11-case `oracle-pin-selftest` PASS are the
evidence that the oracle side still runs.

---

## What this worker did to its own tree, and why

The worktree was handed back **already dirty and 59 commits stale** — the same incident class the
window has now recorded five times. Both conditions were caught by hand before the run started:

- `git status --porcelain` listed **15 tracked `.rs` files this agent did not write**
  (`AGENTS.md` one-writer-per-tree), mtime `2026-09-09 19:12`, an uncommitted partial codemod
  repointing `rules_core::pilot_compute::formula_interpreter` →`crate::pcgen_import::…`. At
  `5e2c0c8c5b` that module did not exist and the tree would not have compiled.
- `git rev-list --count 5e2c0c8c5b..origin/tranche/15` → **59**.

The WIP was **verified superseded, not lost**, before it was discarded: `origin/tranche/15`
carries **zero** `pcgen_import` references in `trait_effects.rs`, `racial_sla.rs` and
`feat_catalog.rs` — Epic 6 removed the dependency outright (148 deletions) rather than
repointing it. It was backed up to `/tmp/sd35-e4-worktree-stale-edits-backup.patch` (284 lines)
and the worktree was `git reset --hard FETCH_HEAD` to `a4efb1a91b` from there. The tree was
byte-clean at gate start and is byte-clean now apart from this report and the retro shard below.

## Retrospective events emitted by this gate

Appended to `docs/retro/events/epic4-wrapup-regate-0910.jsonl` **in this worker's worktree**,
which is never pushed. **The next cycle agent must fold this shard** along with committing this
report.

| id | type | subject |
|---|---|---|
| `1789055528690-epic4-wrapup-regate-0910-eb24be` | incident | re-dispatch into a stale (59 behind) and dirty preserved worktree; WIP verified superseded before discard |
| `1789055528818-epic4-wrapup-regate-0910-80352e` | incident | `wrong-base-worktree` at 5 firings + `stale-wip-in-reused-worktree` at 2; the owed `preflight-base` control, both halves |
| `1789060273721-epic4-wrapup-regate-0910-ccc9e5` | verification | `verify.sh` full: PASS (emitted by `verify.sh` itself) |
