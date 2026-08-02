# SD-28 — Per-cycle Receipts

This file carries the per-cycle receipt for SD-28. Each cycle appends a
new section with the cycle-id and the operator-readable per-cycle facts.

The supervisor reads this file to verify completion before the next cycle
claim (per `decisions.md §15a` local-file dispatch + `loop-instruction.md`
§"Step 6").

## Cycle 0.0 — Chassis Land (planning-ready)

**Date:** 2026-08-01
**Cycle ID:** `SD28-LAND-1`
**Operator:** Todd Hintzmann (directive 2026-08-01)
**Surface:** this directory (`programs/codex/requirements/SD-28-ultimate-book-content-ingestion/`)

### What landed

- 12-file canonical chassis per the spec-domain-bundle-authoring skill.
- Per-doctrine amendments per operator directive 2026-08-01:
  - **Decision §13** — seven books confirmed (six Paizo + Dreamscarred Press).
  - **Decision §14** — `tranche/8` branch, no Hermes board.
  - **Decision §15** — `0.8.<build>` build version.
  - **Decision §15a** — Hermes board retired, local-file dispatch.
  - **Decision §16** — cross-book conflict rule (newer = doctrine).
  - **Decision §17** — Dreamscarred Press license gate.
  - **Decision §17a** — bulk modifications deferred.
  - **Decision §18** — reach gate is the definition of done; engines only when strictly necessary; rules-as-data with pre-computed values (supersedes §12).
  - **Decision §19** — operator ack-chain recorded.

### Pre-launch state

| Check | Status |
|-------|--------|
| `kanban.md` exists | DONE 2026-08-01 — kanban.md present with 12 cards (epics 1–12), dispatch-ordered |
| Branch `tranche/8` pushed to origin | DONE 2026-08-01 — `git branch -r --list 'origin/tranche/8'` → `origin/tranche/8` |
| OAuth credentials valid | PENDING (operator action at cycle launch) |
| Working tree clean | ASSUMED (pre-launch verification) |
| Dreamscarred Press license precheck | PENDING (Epic 9 cycle 0) |

### Next cycle

The next cycle is Epic 2's pre-flight: launch the cycle on `kanban.md`; verify branch + OAuth + tree state. This is
the local-file counterpart to the prior Hermes-board readiness check.

---

**(c) Per-cycle receipts append below this line as cycles fire.**

**2026-08-01 pre-launch readiness pass (operator-side):** branch tip at launch prep: 4d75856c on `tranche/8`. Launch-readiness audit applied fixes to loop-instruction.md (claim step, merged append steps, corpus shape notes, unattended item 4, receipt path), kanban.md (dispatch ordering + Depends-on), scope-draft.md (seven-book reconciliation), and v06_work_inventory (ultimate_psionics roster entry). Eligibility rule 'progress.md corresponds to the operator-pinned branch tip' is satisfied by this entry.

---

## Cycle 0.1 — Workspace cleanup (cross-bundle, applied during SD-28 land)

**Date:** 2026-08-01
**Cycle ID:** `SD28-CLEANUP-1`
**Surface:** `programs/codex/requirements/SD-27-future-state-book-content-ingestion/`

### What landed

The workspace SD-27 directory was deleted on operator directive 2026-08-01
(the move-not-copy doctrine was honored by removing the workspace copy that
had been retained past the prior publish). The canonical SD-27 chassis
remains at `docs/release/SD-27-future-state-book-content-ingestion/`. SD-27
published docs were updated to reflect the workspace removal (Decisions §6
on the publish mechanic + technical-requirements.md line 5 + the
cross-bundle-findings-2026-07-30.md artifact).

Per SD-27's `decisions.md §18`, the underlying conflict (§19.1 "content-only
scope vs. the reach gate") is resolved by SD-28's `decisions.md §18` (reach
gate is the definition of done; engines permitted only when strictly
necessary; no dice-rolling).

## Cycle 0.0+1 — Unattended-mode acknowledgment (operator directive 2026-08-01)

**Date:** 2026-08-01
**Cycle ID:** `SD28-LAND-2` (unattended-mode directive landing)
**Operator:** Todd Hintzmann (out of town per directive)
**Surface:** this directory (`docs/release/SD-28-ultimate-book-content-ingestion/`)

### What landed

The operator is out of town and may not see the harness's output for days. Per
operator directive 2026-08-01, this bundle operates in **unattended mode**.

Cycles MUST NOT pause to ask the operator questions. The operator's verbatim:

> "include instructions to all 3 that indicate they will be running in unnattended
> mode since i will be out of town while this runs. They may not stop to ask
> questions - it might be days before i notice."

The doctrine is mirrored across three files:

- `loop-instruction.md` §"OPERATING METHOD" sub-callout (cycle supervisor reads it first).
- `decisions.md` Decision §21 (load-bearing doctrine entry).
- `progress.md` Cycle 0.0+1 (this entry — per-cycle receipt confirms the operator-on-record).

The receipt chain is the operator's after-return review surface. When the
operator returns, the cycle receipts in this file carry the per-cycle decisions
that the harness made on its behalf.

### Operating protocol summary (mirror of `decisions.md §21`)

1. Default-and-flag, not ask.
2. No `clarify` tool calls.
3. Blockers are recorded, not raised.
4. `decision-blocked` IS allowed.
5. Closure is a goal, not a stop signal.

### Bundle-specific unattended-mode notes

The Dreamscarred Press tier (Epic 9) is the most likely place where the cycle
will want an operator decision. Per the unattended-mode protocol:

- **Dreamscarred Press license audit (Epic 9 cycle 0)** — if the trap-report
  surfaces records not matching open-content tier, record the drops in
  `artifacts/upsi-license-drops.md` and proceed. Do not pause to ask.
- **Epic 9 dispatch decision** — if cycle 0 finds license-conformance gaps
  that would require Dreamscarred Press-specific trap patterns, record
  `decision-blocked` in this file and proceed with the safe default (drop
  non-conforming records; carry the gap into the next cycle-batch).
- **Epic 1 Identifier Cleanup finding** — if the audit surfaces new forbidden
  patterns specific to the third-party tier, record the finding and proceed.

---

(c) Per-cycle receipts append below this line as cycles fire.

## Cycle SD28-E1-F1-001 — Epic 1, Identifier-disclosure audit pass

**Date:** 2026-08-02
**Cycle ID:** `SD28-E1-F1-001`
**Actor:** `sd28-epic1`
**Card:** `epic-1-identifier` (kanban.md row 1) — claimed IN-FLIGHT at
2026-08-02T02:25:56Z, closed COMPLETE this cycle.

### What landed

Ran the Epic 1 identifier-disclosure audit pass (`epic-breakdown.md`
SD28-E1-F1/F2, `decisions.md §6`) against the seven-book surface code and the
repo's audit tooling. No renames were required — the audit returned 0
findings, and the only known instance of the `sd28_` pattern issue
(write-scope text in `technical-requirements.md` /
`acceptance-and-verification.md` permitting `src/bin/sd28_*` /
`tests/sd28_*`) was already corrected in commit `222611be` (pre-existing
on this branch before this cycle). This cycle re-verifies that fix holds
against live code, per SD28-E1-F1's acceptance criteria.

### Commands run (every figure re-derived, per Cycle mechanics 1b)

- Preflight disk: `./scripts/verify.sh --only preflight-disk` → PASS, exit 0
  (repo fs 20% used, 389G available).
- Repo audit script:
  `bash scripts/identifier-discipline-audit.sh` → `OK_NO_BUNDLE_TAGS`, exit 0.
- Direct re-derivation, seven-book surface tree (SD28-E1-F1's acceptance
  path): `grep -rniE 'sd28_|SD28_|Sd28[A-Z]|sd28-' src/rules_core/rules_tables/ultimate_* src/rules_core/rules_tables/dreamscarred_press` →
  0 matches (exit 1 / no-match).
- Direct re-derivation, bins/tests: `grep -rniE 'sd28_|SD28_' src/bin tests` →
  0 matches (exit 1 / no-match).
- `t_<hex>` kanban-token check, scoped to SD28's surface + bins/tests:
  `grep -rnE '\bt_[0-9a-f]{6,}\b' src/rules_core/rules_tables/ultimate_* src/rules_core/rules_tables/dreamscarred_press src/bin tests` →
  8 matches, all in `tests/sd13_*` files (pre-existing SD-13 test slice
  identifiers, outside this bundle's introduced code) — not a finding
  against SD28-E1-F1's scope. No `sd28_*`-tagged `t_<hex>` tokens exist.
- `./scripts/verify.sh` (full, not `--quick`), exit code captured directly:
  **exit 0**. `SUMMARY: passed: 10  preflight-disk root-lib root-full desktop
  reach frontend-install frontend-test frontend-typecheck clippy class-dump`.
  Baseline note (informational, not a failure, not touched this cycle):
  `BASELINE_ROOT_FULL_TESTS` stale — 5930 recorded vs. 5933 measured; this
  cycle did not add or remove tests, so left for whichever cycle owns that
  baseline file next.

### Definition of done — Epic 1 scope

- SD28-E1-F1 acceptance: no `sd28_*` / `SD28_*` / `Sd28*` / `sd28-*` patterns
  in the seven books' surface code — confirmed, 0 findings.
- SD28-E1-F1 acceptance: no `t_<hex>` kanban tokens in source files
  introduced by this bundle — confirmed (only pre-existing SD-13 test
  tokens exist, out of scope).
- SD28-E1-F1 acceptance: identifier-discipline audit script returns 0
  findings — confirmed (`OK_NO_BUNDLE_TAGS`).
- SD28-E1-F2 acceptance: four-grep dual-audit runs cleanly post-Epic-1
  commit — `scripts/identifier-discipline-audit.sh` and
  `scripts/wired-integration-audit.sh` both present and exercised via
  `./scripts/verify.sh`'s green run this cycle.

### Decision-blocked entries

None. No hard blocks encountered.

### Retro events

None emitted this cycle — no new correction, incident, deferral, or rework
occurred (the write-scope text defect this audit re-verifies was already
corrected and logged in the prior `222611be` cycle, not this one).

### Kanban

`epic-1-identifier` → `COMPLETE`. Per `loop-instruction.md` "Epic ordering,"
Epics 2-9, 11 (`epic-11-version` also depends on `epic-1-identifier`) are now
unblocked with respect to this dependency.

## Cycle `SD28-E2-F1-001` — Card `epic-2-prelaunch` (Operator Pre-Launch)

Ran the loop-instruction.md "Pre-launch checklist" items 1-5.

1. **`kanban.md` exists with a ready queue** — confirmed. `epic-1-identifier`
   `COMPLETE`, `epic-2-prelaunch` claimed this cycle, `epic-3..9`,
   `epic-11-version`, `epic-12-code-review`, `epic-10-closure` all `READY`
   and correctly gated by `Depends-on`.
2. **`tranche/8` pushed to origin** — confirmed.
   `git rev-parse HEAD` → `b8fb7d61f105aef93ed6c6ecaf48f541e70c9ff5`;
   `git rev-parse origin/tranche/8` → same hash. Exit 0/0.
3. **GitHub OAuth credentials valid for push** — confirmed.
   `gh auth status` → "Logged in to github.com account electricm0nk",
   "Active account: true", token scopes `project, repo, workflow,
   write:packages`. Exit 0. Note: token is missing `read:org`, which is not
   required for `git push`; no action taken (safer-default: proceed, this
   scope is unrelated to push authority).
4. **Working tree clean** — confirmed clean at cycle start.
   `git status` → "nothing to commit, working tree clean" (repo root, this
   worktree). Note per handoff: this bundle's own doc edits (this cycle's
   `kanban.md` claim edit and this `progress.md` append) are the only
   changes made after that clean check; no other session's in-flight work
   was clobbered — confirmed via `git status` before this write (only
   `kanban.md` modified + new `docs/retro/events/sd28-epic2.jsonl`, both
   this cycle's own).
5. **Dreamscarred Press licensing pre-cycle verification** — confirmed
   open-content per `decisions.md` Decision 17.
   `cargo run --locked --bin v06_corpus_trap_report -- ~/workspace/repos/pcgen/data/pathfinder/dreamscarred_press/ultimate_psionics/`
   → exit 0, ran clean over the full corpus dir (name-binding-collision,
   token-dense-record, governing-token-hidden-by-filter categories
   reported as informational, not defects — trap-report's own framing:
   "Everything above is legitimate upstream data... Nothing here is a
   reason to fail a build.").
   License evidence checked directly (not transcribed from decisions.md):
   `~/workspace/repos/pcgen/data/pathfinder/dreamscarred_press/ultimate_psionics/OGL.txt`
   present, 12 occurrences of "Open Game Content" phrase
   (`grep -c "Open Game Content" .../OGL.txt` → 12); `ultimate_psionics.pcc`
   `COPYRIGHT:` block leads with
   "Open Game License v 1.0a Copyright 2000, Wizards of the Coast, Inc."
   and "System Reference Document" — consistent with the PF-OGL-compatible
   open-content tier per Decision 17. No record failed the licensing audit
   this cycle; nothing dropped from scope.

**Local-file dispatch readiness:** confirmed — `kanban.md` claim/complete
protocol exercised this cycle (Status → IN-FLIGHT → COMPLETE,
Claimed-by/Claimed-at/Cycle-id fields populated), `progress.md` append-only
receipt pattern followed, no Hermes board dependency invoked.

**`./scripts/verify.sh` (full, not `--quick`) — exit code: 0.**
Run backgrounded (foreground run exceeded a 2-minute tool timeout on
`root-full`'s ~490-binary build); log tail confirms
`SUMMARY  passed: 10  preflight-disk root-lib root-full desktop reach
frontend-install frontend-test frontend-typecheck clippy class-dump` and
`RESULT: PASS`. `scripts/verify.sh` line 798 is `exit 0` on its only exit
path after a PASS summary, confirming exit code 0 for this run.

`scripts/reclaim.sh --apply` run at cycle end — exit 0, 0 items reclaimed
(all verify-log dirs from this and concurrent sessions' cycles were
younger than the reclaim age threshold; `tranche/10` worktree skipped,
not merged / upstream present — correctly left alone per shared-checkout
git discipline).

### Pre-launch checklist result: **ALL 5 ITEMS GREEN.** No gaps recorded.

### Decision-blocked entries

None. No hard blocks encountered.

### Retro events

`note` event emitted (no `decision` type exists in `scripts/retro.py`'s
vocabulary — checked via `python3 scripts/retro.py help decision`, which
lists the real types: `correction, incident, near-miss, deferral, rework,
verification, note`) recording the confirmed-open-content ruling for
`ultimate_psionics` this cycle in `docs/retro/events/sd28-epic2.jsonl`,
since Decision 17's pre-cycle verification step is itself a decision point
re-run each launch, not a one-time historical fact. Two `verification`
events were also auto-emitted by `verify.sh` into the same shard (the
`--only preflight-disk` run and the full run), per "Retrospective log"
§2 — nothing manual required for those.

### Kanban

`epic-2-prelaunch` → `COMPLETE`. Per `loop-instruction.md` "Epic ordering,"
Epics 3-9 and `epic-11-version` (already unblocked by `epic-1-identifier`)
remain ready to dispatch; the pre-launch gate itself is now satisfied for
the bundle as a whole.

## Cycle SD28-E11-F1-001 — Epic 11, Build Version Numbering

**Date:** 2026-08-02
**Cycle ID:** `SD28-E11-F1-001`
**Actor:** `sd28-epic11`
**Card:** `epic-11-version` (kanban.md row 10) — claimed IN-FLIGHT at
2026-08-02T03:00:00Z, closed COMPLETE this cycle.

### What landed

Updated the build version numbering from 0.6.1 to 0.8.0 per `decisions.md`
Decision 15, operator-pinned 2026-08-01. The version scheme is
`<major>.<tranche-base>.<build>` where major=0 (no main-publish yet),
tranche-base=8 (base digit of active working tranche `tranche/8`), and build
is the monotonic counter (placeholder 0 in repo files, stamped at CI time by
GITHUB_RUN_NUMBER).

### Files changed

Version triple files (synchronized):
- `apps/desktop/package.json`: 0.6.1 → 0.8.0
- `apps/desktop/src-tauri/Cargo.toml`: 0.6.1 → 0.8.0
- `apps/desktop/src-tauri/tauri.conf.json`: 0.6.1 → 0.8.0
- `apps/desktop/src-tauri/Cargo.lock`: updated via `cargo update` to match new Cargo.toml version

Version check tests (updated anchors from tranche/6 to tranche/8):
- `apps/desktop/src/sd21/buildVersionTriple.test.ts`: updated anchor comment and assertion
- `apps/desktop/src/releaseChecks/buildVersionTriple.test.ts`: updated anchor comment and assertion

Test data (synchronized with new version):
- `apps/desktop/src/testSupport/makeSurface.ts`: 0.6.1-test → 0.8.0-test (2 occurrences)
- `apps/desktop/src/operatorTriage/buildOperatorTriageDraft.test.ts`: 0.6.1-test → 0.8.0-test (2 occurrences)
- `apps/desktop/src/testerWorkbench/loadTesterWorkbenchSurface.test.ts`: 0.6.1-test → 0.8.0-test (2 occurrences)
- `apps/desktop/src/testerWorkbench/status/createWorkbenchStatus.test.ts`: 0.6.1-test → 0.8.0-test (2 occurrences)
- `apps/desktop/src/testerWorkbench/feedback/bug/composeBugReport.test.ts`: 0.6.1-test → 0.8.0-test
- `apps/desktop/src/testerWorkbench/feedback/enhancement/composeEnhancementRequest.test.ts`: 0.6.1-test → 0.8.0-test
- `apps/desktop/src/testerWorkbench/feedback/evidence/captureFeedbackEvidence.test.ts`: 0.6.1-test → 0.8.0-test

Kanban:
- `docs/release/SD-28-ultimate-book-content-ingestion/kanban.md`: card `epic-11-version` claimed IN-FLIGHT

### Commands run (every figure re-derived, per Cycle mechanics 1b)

Re-derive current version from three source files:
```sh
grep -E '"version":|version = ' apps/desktop/{package.json,src-tauri/{Cargo.toml,tauri.conf.json}}
```
Result: all three files had `0.6.1` (verified 2026-08-02).

Re-derive tranche-base from current branch:
```sh
git rev-parse --abbrev-ref HEAD
```
Result: `tranche/8` (verified 2026-08-02 — matches Decision 15's requirement that
tranche-base is the base digit of the active working tranche).

Determine new version per Decision 15:
- major = 0 (per Decision 15: "no main-publish yet")
- tranche-base = 8 (base digit of tranche/8)
- build = 0 (repo placeholder; CI stamps to ${GITHUB_RUN_NUMBER})
→ new version = 0.8.0

Update Cargo.lock to match Cargo.toml:
```sh
cd apps/desktop/src-tauri && cargo update
```
(no --locked flag; needed to regenerate lock file after version change)

Verify all changes with full test suite:
```sh
export RETRO_ACTOR=sd28-epic11 && ./scripts/verify.sh
```
Exit code: **0** (PASS).

### Definition of done — Epic 11 scope

- All three version files (package.json, Cargo.toml, tauri.conf.json) have version 0.8.0 — confirmed
- Cargo.lock regenerated and matches new Cargo.toml version — confirmed via `cargo update`
- buildVersionTriple.test.ts checks updated to expect 0.8.X on tranche/8 — confirmed (2 files updated)
- Test data updated to use 0.8.0-test instead of 0.6.1-test — confirmed (7 files, ~15 occurrences)
- ./scripts/verify.sh (full, not --quick) exits 0 — confirmed, exit code 0

### Decision-blocked entries

None. No hard blocks encountered.

### Retro events

`verification` event auto-emitted by `./scripts/verify.sh`, per "Retrospective log"
§2 (`loop-instruction.md`). One verification run (the full run) with exit code 0.

No corrections, incidents, deferrals, or reworks occurred this cycle.

### Kanban

`epic-11-version` → `COMPLETE`. Per `loop-instruction.md` "Epic ordering,"
Epics 3-9, `epic-12-code-review`, and `epic-10-closure` remain on schedule;
Epic 12 now unblocked with respect to this dependency.

## Cycle SD28-E3-F1-001 — Epic 3, Ultimate Combat

**Date:** 2026-08-01
**Cycle ID:** `SD28-E3-F1-001`
**Actor:** `epic-3-uc`
**Card:** `epic-3-uc` (kanban.md row 3) — claimed IN-FLIGHT at
2026-08-01T00:00:00Z. **Not closed this cycle** — see "What did not land"
below.

### What landed (cycle-open steps only)

Steps 0, 0b, 1, 1b, 1c, and 2 of the cycle mechanics were executed for
real against the current corpus/repo state; no ingest code was written this
cycle.

- **Step 0 (shape):** `cargo run --locked --bin v06_work_inventory`
  regenerated `docs/work-inventory.json` (exit 0). `ultimate_combat`'s
  entries carry `"status": "not-started"`, `"evidence":
  "no_compiled_rule_set_for_book"` — confirms no prior cycle has landed UC
  ingest.
- **Step 0b (trap report):**
  `cargo run --locked --bin v06_corpus_trap_report -- ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_combat`
  (exit 0). Findings of note: 190 `governing-token-hidden-by-filter` hits,
  1500+ `bare-leaf-key-collision`-class hits under namespaced `KEY:`
  patterns (e.g. `KEY:Monk Bonus Feat ~ <leaf>`, 273 records; `KEY:Master of
  Many Styles ~ <leaf>`, 238 records — 139 distinct namespaces total), and
  `token-dense-record` hits (e.g. Tracker's Terrain Mastery: 11
  `BONUS:VAR` tokens on one record — a token count is not a record count).
  None of this is a defect in the corpus; it is real ingest-design surface
  that a naive per-line or bare-leaf-grep ingest would get wrong.
- **Step 1/1b (re-derive):** feat count re-derived directly, matching the
  loop-instruction worked example exactly:
  `grep -c 'CATEGORY:FEAT' ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_combat/uc_feats.lst`
  → **263**.
  `.lst` file count re-derived:
  `find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_combat -iname '*.lst' | wc -l`
  → **46** total, of which
  `find ... -iname '*.lst' -path '*_pfs*' | wc -l` → **4** sit under
  `_pfs/`. Per the book-shape note and `_pfs/`-exclusion rule: `_pfs/`
  (Pathfinder Society legality overrides) is excluded from this cycle's
  ingest scope; this exclusion is stated here explicitly, not dropped
  silently.
- **Step 1c (disk preflight):** `./scripts/verify.sh --only
  preflight-disk` → PASS, exit 0 (repo fs 21% used, 385G available).
- **Step 2 (claim):** `kanban.md` row 3 edited to `IN-FLIGHT`,
  `Claimed-by: epic-3-uc`, `Cycle-id: SD28-E3-F1-001`.

### What did not land, and why (decision-blocked)

`decision-blocked`: **Full Ultimate Combat ingest (TDD implementation,
`reach_gate.rs` claim, wired-integration audit, on-screen verification) was
not attempted this cycle.**

Reason, recorded per the UNATTENDED MODE protocol rather than fabricating a
partial or unverifiable pass:

1. Re-running Definition-of-done item 3 as a pre-check
   (`cargo run --locked --bin v06_corpus_trap_report -- --audit`, exit
   code captured directly) returned **exit 2**, not 0 — a real, pre-existing
   `[key-differs-from-name]` finding set against
   `advanced_class_guide/acg_spells.lst` (e.g. `Summon Nature's Ally II`
   ingested under a different KEY than the corpus declares —
   `Naturalist Summon Nature's Ally II`). This is SD-22 (closed,
   doctrinal-read-only per `loop-instruction.md` "Cross-bundle references")
   content, not Ultimate Combat, and is out of `epic-3-uc`'s write scope.
   Per "Stop vs. press on": *"A gate fails for a reason that is a real
   finding about content or scope... Never weaken, skip, `#[ignore]`, or
   exclude a gate to get green"* — this cycle does not touch ACG data to
   force the audit green, and does not claim item 3 of the Definition of
   Done for this book while the repo-wide audit is red for an unrelated
   reason. This blocks `epic-12-code-review`'s and `epic-10-closure`'s
   preconditions bundle-wide, not just this card, and should be triaged as
   its own fix (likely against SD-22's identifier-discipline follow-up, not
   SD-28) before any book epic can honestly claim DoD item 3.
2. The UC namespace shape surfaced by the trap report (139 distinct
   namespaced `KEY:` prefixes, the largest three carrying 273/238/96
   records respectively, `ASPECT`-bearing class-ability records across
   `support/uc_abilities_class_{acg,apg,um}.lst`, and 22 referenced
   sourcebooks for cross-book prereqs) is materially larger and more
   structurally varied than a single bounded cycle can responsibly design,
   implement (TDD), wire into `reach_gate.rs` with a real claim, pass the
   four-check wired-integration audit, and on-screen-verify — all within
   one cycle-batch — without risking exactly the stub/fixture-only,
   fabricated-pass failure mode `AGENTS.md` §6 and this program's own
   retrospective forbid. No ingest code, `RuleSetId` variant, or
   `reach_gate.rs` claim was written this cycle; none is claimed as done.

Per "Stop vs. press on," size alone is never a stop reason and is not the
reason recorded here — the *combination* of (a) a real, pre-existing
DoD-item-3 gate failure outside this card's write scope, and (b) a
first-pass trap-report scope wide enough that any implementation attempted
in the remainder of this cycle could not clear TDD + verify.sh + the
four-check audit + on-screen verification honestly, is.

### Commands run (every figure re-derived)

```sh
cargo run --locked --bin v06_work_inventory                                        # exit 0
cargo run --locked --bin v06_corpus_trap_report -- ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_combat   # exit 0
cargo run --locked --bin v06_corpus_trap_report -- --audit                          # exit 2 (pre-existing ACG finding, out of scope)
grep -c 'CATEGORY:FEAT' ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_combat/uc_feats.lst   # 263
find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_combat -iname '*.lst' | wc -l            # 46
find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_combat -iname '*.lst' -path '*_pfs*' | wc -l   # 4
./scripts/verify.sh --only preflight-disk                                           # exit 0
```

### Retro events

`scripts/retro.py deferral --actor epic-3-uc --what "Ultimate Combat TDD
ingest, reach_gate claim, wired-integration audit, on-screen verification"
--reason "repo-wide v06_corpus_trap_report --audit exits 2 on a
pre-existing, out-of-scope ACG identifier finding (DoD item 3 precondition
red bundle-wide); UC's own trap-report surface (139 namespaces, 190
governing-token-hidden-by-filter hits) is too structurally varied to
implement+TDD+wire+verify honestly in the remainder of this cycle" --scope
"one book epic (epic-3-uc)" --blocked-by "ACG key-differs-from-name fix
(out of SD-28 write scope, likely SD-22 follow-up)" --tracked-at
"docs/release/SD-28-ultimate-book-content-ingestion/progress.md
SD28-E3-F1-001"` — emitted this cycle to
`docs/retro/events/epic-3-uc.jsonl`.

### Kanban

`epic-3-uc` remains `IN-FLIGHT`, `Claimed-by: epic-3-uc`,
`Cycle-id: SD28-E3-F1-001`. Not moved to `COMPLETE`. Next cycle against
this card should: (1) confirm whether the ACG audit finding has been fixed
by another epic/session, (2) if not, decide (as a fresh `decision-blocked`
or an accepted scope note) whether epic-3-uc proceeds with DoD item 3 red
for a reason outside its own scope, and (3) if proceeding, design the
`RuleSetId` addition and per-namespace ingest shape against the trap-report
findings recorded above before writing ingest code.

## SD28-E4-F1-001 — epic-4-um (Ultimate Magic) — 2026-08-02T03:40:27Z

### Steps completed

- **Step 0 (shape):** `cargo run --locked --bin v06_work_inventory` → exit 0.
  `docs/work-inventory.json` `books[]` entry for `ultimate_magic`:
  `files_enumerated: 16`, `files_not_enumerated: 14` (support/data-control
  files with no `Kind` mapping yet — e.g. `um_abilities.lst`,
  `um_domains.lst`, `um_kits.lst`, `um_templates.lst`). `kinds`: class 1,
  class_feature 1783, race_trait 17, feat 155, spell 291, equipment 26,
  companion 173 — all `not-started`. `trap_hits` dominated by
  `mod_record: 750` and `comment_or_disabled: 826`.
- **Step 0b (trap report, book dir, before any ingest code):**
  `cargo run --locked --bin v06_corpus_trap_report -- ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_magic`
  → **exit 0**. Findings are informational per the tool's own footer ("Everything
  above is legitimate upstream data... Nothing here is a reason to fail a
  build."): 2007+ `namespace-collapsed-by-bare-key` hits (largest namespaces:
  `Magus Spellblend` 553, `Evolution` 101, `Ki Power` 78, `Discovery` 44,
  `Pack Lord` 42, `Forbidden Rites Domain` 33, `Magus Arcana` 33), 61
  `token-dense-record` hits (e.g. `Aberrant Bloodline` — 12 `BONUS:VAR`
  tokens on one record), 318 `governing-token-hidden-by-filter` hits (e.g.
  `Magus` carries `MULT`/`STACK` alongside BONUS/PRE; `Arcane Pool` carries
  `ASPECT`).
- **Step 1 / 1b (read + re-derive):** `scope-draft.md`, `decisions.md`,
  `kanban.md`, and this file read. Figures re-derived directly, not
  transcribed:
  - `.lst` file count: `find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_magic -iname '*.lst' | wc -l`
    → **30**.
  - `_pfs/` subset: `find ... -iname '*.lst' -path '*_pfs*' | wc -l` →
    **5** (16.7% of 30, not "34%" — that figure in `loop-instruction.md`'s
    corpus-shape notes is Ultimate Combat's, not Ultimate Magic's; UM has
    no `support/` subdirectory per those same notes, only `_pfs/`). `_pfs/`
    (Pathfinder Society legality overrides) is excluded from this cycle's
    ingest scope — exclusion stated here explicitly, not dropped silently.
  - `.pcc` count: `find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_magic -iname '*.pcc' | wc -l`
    → **2**.
- **Step 1c (disk preflight):** `./scripts/verify.sh --only preflight-disk`
  → PASS, exit 0 (repo fs 21% used, 385G available).
- **Step 2 (claim):** `kanban.md` row 4 edited to `IN-FLIGHT`,
  `Claimed-by: epic-4-um`, `Claimed-at: 2026-08-02T03:40:27Z`,
  `Cycle-id: SD28-E4-F1-001`.

### What did not land, and why (decision-blocked)

`decision-blocked`: **Full Ultimate Magic ingest (TDD implementation,
`reach_gate.rs` claim, wired-integration audit, on-screen verification) was
not attempted this cycle.**

This cycle re-ran Definition-of-done item 3 as a pre-check before writing
any ingest code, per this bundle's own "Stop vs. press on" rule ("A gate
fails for a reason that is a real finding about content or scope... Never
weaken, skip, `#[ignore]`, or exclude a gate to get green"):

`cargo run --locked --bin v06_corpus_trap_report -- --audit` → **exit 2**
(re-derived fresh this cycle, not carried forward from `epic-3-uc`'s
receipt). The 9 defects are unchanged from the ones `epic-3-uc` recorded
(`SD28-E3-F1-001`, this file, above): `[key-differs-from-name]` on
`spell/summon_nature_s_ally_{i..ix}.json`, all sourced to
`pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_spells.lst`
(e.g. `Naturalist Summon Nature's Ally I` ingested under the bare
`Summon Nature's Ally I` identity). This is SD-22 (closed,
doctrinal-read-only per `loop-instruction.md` "Cross-bundle references")
ACG content — not Ultimate Magic, and out of `epic-4-um`'s write scope.

Confirming this is the *same* repo-wide precondition `epic-3-uc` hit (not a
new instance): the audit output's defect list, file paths, and count (9
defects, 0 `mod-record` trap defects, 268 `TRAP` count) match `epic-3-uc`'s
recorded receipt exactly. This is one bundle-wide blocker, not a
per-book-recurring one — DoD item 3 is red for every book epic in this
bundle until the ACG fix lands, and this cycle does not claim item 3 for
Ultimate Magic while that fix is outstanding, per the same reasoning
`epic-3-uc` recorded.

Unlike `epic-3-uc`, this cycle's own book-dir trap report (`Step 0b` above)
came back **exit 0** — Ultimate Magic's own corpus shape is not, by itself,
a second reason to block. The sole recorded blocker for `epic-4-um` is the
repo-wide DoD-item-3 precondition. No ingest code, `RuleSetId` variant, or
`reach_gate.rs` claim was written this cycle; none is claimed as done.

### Commands run (every figure re-derived)

```sh
cargo run --locked --bin v06_work_inventory                                          # exit 0
cargo run --locked --bin v06_corpus_trap_report -- ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_magic   # exit 0
cargo run --locked --bin v06_corpus_trap_report -- --audit                            # exit 2 (pre-existing ACG finding, out of scope)
find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_magic -iname '*.lst' | wc -l                # 30
find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_magic -iname '*.lst' -path '*_pfs*' | wc -l  # 5
find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_magic -iname '*.pcc' | wc -l                 # 2
./scripts/verify.sh --only preflight-disk                                             # exit 0
```

### Retro events

`scripts/retro.py deferral --actor epic-4-um --what "Ultimate Magic TDD
ingest, reach_gate claim, wired-integration audit, on-screen verification"
--reason "repo-wide v06_corpus_trap_report --audit exits 2 on the same
pre-existing, out-of-scope ACG identifier finding epic-3-uc recorded
(SD28-E3-F1-001); UM's own book-dir trap report is clean (exit 0), so this
is a bundle-wide DoD-item-3 precondition failure, not a UM-specific
scope problem" --scope "one book epic (epic-4-um)" --blocked-by "ACG
key-differs-from-name fix (out of SD-28 write scope, likely SD-22
follow-up)" --tracked-at
"docs/release/SD-28-ultimate-book-content-ingestion/progress.md
SD28-E4-F1-001"` — emitted this cycle to `docs/retro/events/epic-4-um.jsonl`.

### Kanban

`epic-4-um` remains `IN-FLIGHT`, `Claimed-by: epic-4-um`,
`Cycle-id: SD28-E4-F1-001`. Not moved to `COMPLETE`. Next cycle against
this card should: (1) confirm whether the ACG audit finding has been fixed
by another epic/session (check `epic-3-uc`'s card and this file for a
newer receipt first — the fix, once it lands, unblocks every book epic at
once, not just one), (2) if fixed, proceed straight to Step 3 (TDD
implementation) using this cycle's Step 0/0b/1b findings above as the
starting shape, and (3) if not fixed, re-run the `--audit` pre-check before
doing anything else — do not re-derive the ACG defect list from memory.

## `epic-5-ue` — Ultimate Equipment (`SD28-E5-F1-001`, 2026-08-01)

- **Step 0 (shape):** `cargo run --locked --bin v06_work_inventory` → exit
  0. `docs/work-inventory.json` `ultimate_equipment` entry: `files_enumerated:
  9`, 7 `files_not_enumerated` (`ue_abilities.lst`, `ue_abilitycategories.lst`,
  `ue_kits.lst`, `ue_profs_armor.lst`, `ue_profs_weapon.lst`, `ue_skills.lst`,
  `ue_templates.lst`). `kinds`: `equipment` 1424 units (all `not-started`),
  `equipment_modifier` 190 units (all `not-started`), `spell` 1 unit
  (`not-started`). `trap_hits`: `comment_or_disabled` 264, `copy_record` 92,
  `directive_line` 9, `duplicate_identity` 3, `invisible_record` 5,
  `mod_record` 1682.
- **Step 0b (trap-report, book dir, before any ingest code):**
  `cargo run --locked --bin v06_corpus_trap_report --
  ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_equipment`
  → **exit 0**, clean. Findings are informational, not defects: 1
  `define-zero-value-elsewhere` (`MerformBeltMerfolkForm` in
  `ue_abilities.lst:13`), 185 `namespaced-key` (dominant namespaces:
  `Special Ability` 120, `Material` 49, `Page of Spell Knowledge` 9,
  `Special Quality` 7), 15 `governing-token-hidden-by-filter` (e.g.
  `Headband of Intellect Knowledge Skill Selection` carries
  `MULT`/`STACK`/`CHOOSE`; `Otherworldly Kimono` carries `TEMPBONUS`).
- **Step 1 / 1b (read + re-derive):** `scope-draft.md`, `decisions.md`
  (Decision 10, §18), `forward-scope-register.md` (C3.1), `kanban.md`, this
  file, and `epic-4-um`'s receipt (above) read. Figures re-derived directly
  against the corpus, not transcribed:
  - `.pcc` discovery (glob `*.pcc`, no leading underscore per
    `loop-instruction.md` corpus-shape notes): `find
    ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_equipment
    -maxdepth 1 -iname '*.pcc'` → **1** (`ultimate_equipment.pcc`).
  - `.lst` count, recursive: `find
    ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_equipment
    -iname '*.lst' | wc -l` → **16**.
  - `_pfs/` subset: `find ... -iname '*.lst' -path '*_pfs*' | wc -l` →
    **4** (25% of 16). `_pfs/` (Pathfinder Society legality overrides) is
    excluded from this cycle's ingest scope — exclusion stated here
    explicitly, not dropped silently.
  - `support/` subset: `find ... -iname '*.lst' -path '*support*' | wc -l`
    → **0** — UE has only `_pfs/`, no `support/`, matching
    `loop-instruction.md`'s corpus-shape notes.
  - Feats file: `find ... -iname '*feat*'` → **no matches** — UE has no
    feats file, confirmed (matches the brief's shape note).
  - `CATEGORY:` distribution (`ue_abilities.lst` and
    `ue_equip_magic_items.lst`): `grep -rho 'CATEGORY:[A-Za-z ]*'
    <book-dir>/*.lst | sort | uniq -c | sort -rn` → `41 CATEGORY:Special
    Ability`, `7 CATEGORY:Internal`, `1 CATEGORY:Headband Knowledge Skill`,
    `1 CATEGORY:Equipment`.
  - Repo-wide DoD item 3 precondition, re-checked fresh (not carried
    forward from `epic-3-uc`/`epic-4-um`'s receipts): `cargo run --locked
    --bin v06_corpus_trap_report -- --audit` → **exit 2**, same 9
    `key-differs-from-name` defects on
    `pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_spells.lst`
    (`Naturalist Summon Nature's Ally I..IX` filed under the bare `Summon
    Nature's Ally I..IX` identity) that `epic-3-uc` (`SD28-E3-F1-001`) and
    `epic-4-um` (`SD28-E4-F1-001`) both recorded. This is SD-22 (closed,
    doctrinal-read-only) ACG content, out of `epic-5-ue`'s write scope —
    same repo-wide blocker, not a new instance.
- **Step 1c (disk preflight):** `./scripts/verify.sh --only
  preflight-disk` → PASS, exit 0 (repo fs 21% used, 385G available).
- **Step 2 (claim):** `kanban.md` row 5 edited to `IN-FLIGHT`,
  `Claimed-by: epic-5-ue`, `Claimed-at: 2026-08-01T00:00:00Z`,
  `Cycle-id: SD28-E5-F1-001`.

### What did not land, and why (decision-blocked — two independent reasons)

`decision-blocked`: **Full Ultimate Equipment TDD ingest, `reach_gate.rs`
claim, wired-integration audit, and on-screen verification were not
attempted this cycle.** Two independent blockers, either one sufficient on
its own:

1. **Repo-wide DoD-item-3 precondition (shared with `epic-3-uc` and
   `epic-4-um`):** `cargo run --locked --bin v06_corpus_trap_report --
   --audit` exits 2 on the pre-existing, out-of-scope ACG
   `key-differs-from-name` finding (re-derived fresh this cycle, above).
   UE's own book-dir trap report is clean (exit 0), so this is not a
   UE-specific scope problem — it is the same bundle-wide blocker recorded
   twice already.
2. **The known C3.1 equipment-catalog widening
   (`forward-scope-register.md` C3.1, `decisions.md` §10/§18):**
   `apps/desktop/src-tauri/src/equipment_catalog.rs` reads CRB alone;
   APG/ACG-ingested equipment already reaches no surface today per
   `reach_gate.rs OPEN_FINDINGS`. UE is the largest equipment book in the
   corpus, and per `decisions.md` §18 ("the reach gate is the definition
   of done; engine or widening where strictly necessary; UE's cycling
   pauses on `decision-blocked` if the surface remains absent") and the
   "Hard stops" rule in `loop-instruction.md` ("A record family cannot be
   surfaced without work outside this bundle's epic structure... The cycle
   reports the gap; it does not add an epic and it does not ingest without
   a reach claim"), this cycle takes the safe default: it does **not** add
   a surface-building epic, and it does **not** ingest UE's 1424
   `equipment` + 190 `equipment_modifier` units without a reach claim for
   them. The operator's open question (precycle prerequisite outside SD-28
   vs. SD-28-owned retrofit) is left unanswered per `forward-scope-register.md`
   C3.1's own framing — this cycle does not force that answer.

No ingest code, `RuleSetId` variant, or `reach_gate.rs` claim was written
this cycle; none is claimed as done. `./scripts/verify.sh` (full) was not
run against new production code because none was written; the disk-preflight
subset above stands as this cycle's only `verify.sh` invocation, consistent
with `epic-4-um`'s precedent for a decision-blocked cycle.

### Commands run (every figure re-derived)

```sh
cargo run --locked --bin v06_work_inventory                                                                                  # exit 0
cargo run --locked --bin v06_corpus_trap_report -- ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_equipment   # exit 0
cargo run --locked --bin v06_corpus_trap_report -- --audit                                                                    # exit 2 (pre-existing ACG finding, out of scope)
find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_equipment -maxdepth 1 -iname '*.pcc'             # 1
find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_equipment -iname '*.lst' | wc -l                 # 16
find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_equipment -iname '*.lst' -path '*_pfs*' | wc -l  # 4
find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_equipment -iname '*.lst' -path '*support*' | wc -l  # 0
find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_equipment -iname '*feat*'                       # (no matches)
grep -rho 'CATEGORY:[A-Za-z ]*' ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_equipment/*.lst | sort | uniq -c | sort -rn   # see above
./scripts/verify.sh --only preflight-disk                                                                                     # exit 0
```

### Retro events

`scripts/retro.py deferral --actor epic-5-ue --what "Ultimate Equipment TDD
ingest, reach_gate claim, wired-integration audit, on-screen verification"
--reason "two independent decision-blocked reasons: (1) repo-wide
v06_corpus_trap_report --audit exits 2 on the same pre-existing,
out-of-scope ACG key-differs-from-name finding epic-3-uc/epic-4-um
recorded; (2) the known C3.1 equipment-catalog-widening gap
(forward-scope-register.md C3.1, decisions.md §10/§18) — equipment_catalog.rs
reads CRB only, UE's 1424 equipment + 190 equipment_modifier units have no
reach claim to make without widening it, and the hard-stop rule bars
ingesting without a reach claim or adding an out-of-epic surface-building
epic" --scope "one book epic (epic-5-ue)" --blocked-by "ACG
key-differs-from-name fix (out of SD-28 write scope); equipment_catalog.rs
widening (operator-pending, forward-scope-register.md C3.1)"
--tracked-at
"docs/release/SD-28-ultimate-book-content-ingestion/progress.md
SD28-E5-F1-001"` — emitted this cycle to `docs/retro/events/epic-5-ue.jsonl`.

### Kanban

`epic-5-ue` remains `IN-FLIGHT`, `Claimed-by: epic-5-ue`, `Cycle-id:
SD28-E5-F1-001`. Not moved to `COMPLETE`. Next cycle against this card
should: (1) confirm whether the ACG audit finding has been fixed by another
epic/session (check `epic-3-uc`/`epic-4-um`'s cards and this file for a
newer receipt first), (2) confirm whether the equipment-catalog widening
question has been operator-answered or otherwise resolved (check
`forward-scope-register.md` C3.1 and `decisions.md` §10/§18 for an updated
status), and (3) only once both are clear, proceed to Step 3 (TDD
implementation) using this cycle's Step 0/0b/1b findings above as the
starting shape.

## Cycle `SD28-E6-F1-001` — Card `epic-6-ui` (Ultimate Intrigue)

**Date:** 2026-08-01
**Actor:** `epic-6-ui`
**Card:** `epic-6-ui` (kanban.md row 6) — claimed `IN-FLIGHT` at
2026-08-01T00:00:00Z. **Not closed this cycle** — see "What did not land" below.

### What landed (cycle-open steps only)

- **Step 0 (shape):** `cargo run --locked --bin v06_work_inventory` → exit 0.
  `docs/work-inventory.json` `books[]` entry for `ultimate_intrigue`:
  `scope: "future_state"`, `engine_rule_set: null` (no compiled `RuleSetId`
  variant exists yet — re-confirmed with
  `grep -rn "UltimateIntrigue\|ultimate_intrigue" src/ apps/desktop/src-tauri/src/ --include=*.rs`
  → zero matches). `kinds`: class 3, class_feature 931, race_trait 17,
  feat 107, spell 101, equipment 91, equipment_modifier 14, companion 1 —
  all `"status": "not-started"`. `trap_hits` dominated by `mod_record: 262`,
  `comment_or_disabled: 341`, `internal_namespace: 48`.
- **Step 0b (trap report, book dir, before any ingest code):**
  `cargo run --locked --bin v06_corpus_trap_report -- ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_intrigue`
  → **exit 0**. 950 `namespaced-key` findings (largest namespaces: `Refined
  Education` 94, `Social Grace` 85, `Vigilante Talent` 74, `Skinshaper` 32,
  `Refined Education Unlock` 25), 45 `governing-token-hidden-by-filter`
  findings (e.g. `Combat Skill` carries `MULT`/`STACK`/`CHOOSE` alongside
  BONUS/PRE), and several `DEFINE`d-to-0-here findings whose real value is
  granted elsewhere (e.g. `Vigilante Specialization` — `VigilanteIsAvenger`
  / `VigilanteIsStalker`). All informational per the tool's own footer;
  none is a defect.
- **Step 1/1b (re-derive; corrects two book-shape notes carried in this
  cycle's dispatch brief):**
  - `.pcc` discovery: `find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_intrigue -maxdepth 1 -iname '*.pcc'`
    → `_ultimate_intrigue.pcc` (**leading underscore**). The dispatch
    brief for this cycle stated "glob `*.pcc` (not `_*.pcc`)" for this
    book; that is wrong — Ultimate Intrigue is one of the five books
    `loop-instruction.md`'s own corpus-shape notes already correctly flag
    as underscore-prefixed (only `ultimate_equipment.pcc` and
    `ultimate_psionics.pcc` lack the underscore). **Correction recorded**
    via `scripts/retro.py correction` below.
  - `.lst` file count (recursive): `find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_intrigue -iname '*.lst' | wc -l`
    → **21**. Subdirectory breakdown: `find ... -iname '*.lst' -path '*support*' | wc -l`
    → **5** (24% of 21); `find ... -iname '*.lst' -path '*_pfs*' | wc -l`
    → **0** — Ultimate Intrigue has **no `_pfs/` directory at all**, matching
    `loop-instruction.md`'s note "UI and UW have only `support/`". The
    dispatch brief's "34% of them sit in support/ and `_pfs/`" figure is
    the whole-bundle aggregate, not UI-specific; there is nothing to
    exclude for `_pfs/` in this book (0 files), so the exclusion is
    recorded here as a no-op, not dropped silently.
  - Feat count: `grep -c 'CATEGORY:FEAT' ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_intrigue/ui_feats.lst`
    → **104** at the top level; recursive total including
    `support/ui_feats_oa.lst` via
    `find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_intrigue -iname '*.lst' -exec grep -c 'CATEGORY:FEAT' {} \; | awk '{s+=$1} END{print s}'`
    → **108**, matching `work-inventory.json`'s `feat: 107` closely enough
    to be within its own dedup/`.MOD` handling (not re-derived further this
    cycle — feat ingest itself is blocked, see below).
  - License: `SOURCESHORT:UI` (confirmed against `loop-instruction.md`'s
    note). `OGL.txt` exists on disk at the book root *and* `_ultimate_intrigue.pcc`
    carries `COPYRIGHT:` blocks — both mechanisms present for this book (UC/UPsi's
    asymmetry per `loop-instruction.md` does not apply to UI).
  - Class overlap note (re-confirmed, not re-derived numerically — this is
    a scope-boundary check, not a count): Occultist/Spiritualist/Medium/
    Mesmerist class content in this corpus is canonical-to-SD-30 per
    `decisions.md`/`loop-instruction.md`'s Cross-bundle-references section;
    any future ingest cycle for this book references the canonical class
    id only, never re-derives or forks it here.
- **Step 1c (disk preflight):** `./scripts/verify.sh --only preflight-disk`
  → PASS, exit 0 (repo fs 21% used, 385G available).
- **Step 2 (claim):** `kanban.md` row 6 edited to `IN-FLIGHT`,
  `Claimed-by: epic-6-ui`, `Claimed-at: 2026-08-01T00:00:00Z`,
  `Cycle-id: SD28-E6-F1-001`.

### What did not land, and why (decision-blocked — repo-wide precondition, same as `epic-3-uc`/`epic-4-um`/`epic-5-ue`)

`decision-blocked`: **Full Ultimate Intrigue TDD ingest (new `RuleSetId`
variant, ingest code, `reach_gate.rs` claim, wired-integration audit,
on-screen verification) was not attempted this cycle.**

Repo-wide DoD item 3 precondition, re-checked fresh this cycle (not carried
forward from a sibling receipt): `cargo run --locked --bin
v06_corpus_trap_report -- --audit` (piped to a file, exit code read
directly, never through a pipe) → **exit 2**, the same 9
`key-differs-from-name` defects on
`pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_spells.lst`
(`Naturalist Summon Nature's Ally I..IX` filed under the bare `Summon
Nature's Ally I..IX` identity) already recorded by `epic-3-uc`
(`SD28-E3-F1-001`), `epic-4-um` (`SD28-E4-F1-001`), and `epic-5-ue`
(`SD28-E5-F1-001`). This is SD-22 (closed, doctrinal-read-only) ACG
content, out of `epic-6-ui`'s write scope — the same repo-wide blocker,
recorded a fourth time, not a new instance. Per "Hard stops" in
`loop-instruction.md` ("A figure derived this cycle disagrees with a
figure recorded in this package... Investigate which is wrong and report")
this cycle investigated: the figure agrees exactly with the three prior
receipts, so it is not disagreement — it is confirmation the blocker is
still live and untouched by any intervening cycle.

Separately, `ultimate_intrigue` carries `scope: "future_state"` and
`engine_rule_set: null` in `docs/work-inventory.json` — no compiled
`RuleSetId` variant exists for this book yet (re-confirmed above). Even
absent the repo-wide blocker, a from-scratch `RuleSetId` variant plus
parser hookup plus a reach-claim surface is exactly the class of
cross-cutting engine work the "Hard stops" rule reserves for explicit
scope decision, not a single per-book cycle's silent invention. This cycle
does not add that surface unilaterally.

No ingest code, `RuleSetId` variant, or `reach_gate.rs` claim was written
this cycle; none is claimed as done. `./scripts/verify.sh` (full) was not
run against new production code because none was written; the
disk-preflight subset above stands as this cycle's only `verify.sh`
invocation, consistent with `epic-4-um`/`epic-5-ue`'s precedent for a
decision-blocked cycle.

### Commands run (every figure re-derived)

```sh
cargo run --locked --bin v06_work_inventory                                                                                             # exit 0
cargo run --locked --bin v06_corpus_trap_report -- ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_intrigue      # exit 0
cargo run --locked --bin v06_corpus_trap_report -- --audit                                                                                # exit 2 (pre-existing ACG finding, out of scope)
grep -rn "UltimateIntrigue\|ultimate_intrigue" src/ apps/desktop/src-tauri/src/ --include=*.rs                                            # (no matches)
find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_intrigue -maxdepth 1 -iname '*.pcc'                          # _ultimate_intrigue.pcc
find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_intrigue -iname '*.lst' | wc -l                              # 21
find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_intrigue -iname '*.lst' -path '*support*' | wc -l            # 5
find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_intrigue -iname '*.lst' -path '*_pfs*' | wc -l               # 0
grep -c 'CATEGORY:FEAT' ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_intrigue/ui_feats.lst                     # 104
find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_intrigue -iname '*.lst' -exec grep -c 'CATEGORY:FEAT' {} \; | awk '{s+=$1} END{print s}'  # 108
find ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_intrigue -iname 'OGL*'                                       # OGL.txt
grep -i 'SOURCESHORT' ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_intrigue/_ultimate_intrigue.pcc             # SOURCESHORT:UI
./scripts/verify.sh --only preflight-disk                                                                                                 # exit 0
```

### Retro events

`scripts/retro.py correction --subject "epic-6-ui dispatch brief" --claimed
"glob *.pcc (not _*.pcc) for ultimate_intrigue" --actual
"_ultimate_intrigue.pcc — leading underscore, matching
loop-instruction.md's own corpus-shape note that only ultimate_equipment
and ultimate_psionics lack the underscore" --verified-by "find
~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_intrigue
-maxdepth 1 -iname '*.pcc'" — emitted this cycle to
`docs/retro/events/epic-6-ui.jsonl`.

`scripts/retro.py deferral --actor epic-6-ui --what "Ultimate Intrigue TDD
ingest, new RuleSetId variant, reach_gate claim, wired-integration audit,
on-screen verification" --reason "repo-wide v06_corpus_trap_report
--audit exits 2 on the same pre-existing, out-of-scope ACG
key-differs-from-name finding epic-3-uc/epic-4-um/epic-5-ue already
recorded (fourth confirmation, not a new instance); separately,
ultimate_intrigue is scope:future_state / engine_rule_set:null in
work-inventory.json — no RuleSetId variant exists, and inventing one
unilaterally in a single per-book cycle is exactly the cross-cutting
engine work the hard-stops rule reserves for explicit scope decision"
--scope "one book epic (epic-6-ui)" --blocked-by "ACG
key-differs-from-name fix (out of SD-28 write scope); RuleSetId variant +
parser hookup for ultimate_intrigue (not yet scoped to a single-book
cycle)" --tracked-at
"docs/release/SD-28-ultimate-book-content-ingestion/progress.md
SD28-E6-F1-001"` — emitted this cycle to `docs/retro/events/epic-6-ui.jsonl`.

### Kanban

`epic-6-ui` remains `IN-FLIGHT`, `Claimed-by: epic-6-ui`, `Cycle-id:
SD28-E6-F1-001`. Not moved to `COMPLETE`. Next cycle against this card
should: (1) confirm whether the ACG audit finding has been fixed by
another epic/session (check `epic-3-uc`/`epic-4-um`/`epic-5-ue`'s cards
and this file for a newer receipt first), (2) confirm whether a
`RuleSetId` variant for `ultimate_intrigue` has been added by an
intervening cycle, and (3) only once both are clear, proceed to Step 3
(TDD implementation) using this cycle's Step 0/0b/1b findings above as the
starting shape.

## Cycle SD28-E7-F1-001 — `epic-7-ucam` (Ultimate Campaign)

**Date:** 2026-08-01
**Cycle ID:** `SD28-E7-F1-001`
**Claim:** kanban.md row 7 → `Status: IN-FLIGHT`,
`Claimed-by: epic-7-ucam`, `Claimed-at: 2026-08-01T00:00:00Z`,
`Cycle-id: SD28-E7-F1-001`.

### Step 0 — shape (`v06_work_inventory`)

`cargo run --locked --bin v06_work_inventory` → exit 0. `ultimate_campaign`
entry in `docs/work-inventory.json`: `scope: "future_state"`,
`engine_rule_set: null`, `files_enumerated: 1` (only the `.pcc`),
`kinds: {"feat": {"units": 23, "by_status": {"not-started": 23}}}`. The
book-specific brief's "~23 inventory units is the correct shape" is
confirmed exactly, not approximately — 23, re-derived, matches.

Note the work-inventory's `kinds` map shows only `feat` — its `Kind`
mapping folds traits/drawbacks/retraining records under the `feat` kind
for this book (a work-inventory taxonomy fact, not a corpus fact); the
corpus itself has separate trait/drawback/retraining files as enumerated
below.

### Step 0b — trap report (before any ingest code)

`cargo run --locked --bin v06_corpus_trap_report --
~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_campaign`
→ exit 0. Findings, all upstream-legitimate per the tool's own footer ("Everything
above is legitimate upstream data... Nothing here is a reason to fail a
build"):

- `comment-or-disabled`: ~81 (78 shown + header)
- `key-differs-from-name`: 271 — every trait/drawback/retraining record is
  namespaced (`Drawback ~ Attached`, `Trait ~ ...`, `Retrain ~ ...`,
  `Retraining ~ ...`, `Deathtouched ~ ...`); joining on display name would
  merge distinct records.
- `namespaced-key`: 271 (same records, flagged from the opposite side)
- `define-zero-value-elsewhere`: 1 (`RetrainingDaysSpent` DEFINEd to 0 in
  `uca_abilities_retraining.lst:6`, granted via `BONUS:VAR` elsewhere in
  the same file)
- `governing-token-hidden-by-filter`: 81 (`MULT`/`STACK`/`CHOOSE` alongside
  BONUS/PRE on retraining records)
- KEY namespaces in this book: `Trait` (154), `Retrain` (50), `Retraining`
  (48), `Drawback` (17), `Deathtouched` (2).

**`_pfs/` exclusion (stated per instruction, not dropped silently):** this
cycle's inventory and trap-report both exclude
`ultimate_campaign/_pfs/pfs_uca_abilities_drawbacks.lst` and
`ultimate_campaign/_pfs/pfs_uca_abilities_traits.lst` (2 files, confirmed
below) — Pathfinder Society legality overrides, deliberately out of scope
per `loop-instruction.md` "Corpus shape notes."

### Step 1b — re-derivation (every figure, own command)

```sh
BOOK=~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_campaign
find "$BOOK" -maxdepth 1 -iname '*.pcc'                                   # _ultimate_campaign.pcc (leading underscore)
find "$BOOK" -iname '*.lst' | wc -l                                       # 11 (recursive)
find "$BOOK" -maxdepth 1 -iname '*.lst' | wc -l                           # 9 (top-level)
find "$BOOK" -iname '*.lst' -path '*_pfs*' | wc -l                        # 2
find "$BOOK" -iname '*.lst' -path '*support*' | wc -l                     # 0 (no support/ dir in this book)
grep -i SOURCESHORT "$BOOK"/*.pcc                                          # SOURCESHORT:UCA
grep -c 'KEY:' "$BOOK"/uca_abilities_drawbacks.lst                        # 17
grep -c 'KEY:' "$BOOK"/uca_abilities_retraining.lst                       # 83
grep -c 'KEY:' "$BOOK"/uca_abilities_traits.lst                           # 233
grep -c '"book": "ultimate_campaign"' docs/work-inventory.json            # 23 (inventory units)
grep -rn "UltimateCampaign\|ultimate_campaign" src/ apps/desktop/src-tauri/src/ --include=*.rs  # (no matches)
cargo run --locked --bin v06_corpus_trap_report -- --audit                # exit 2 (pre-existing, out-of-scope ACG finding)
./scripts/verify.sh --only preflight-disk                                 # exit 0
```

**Correction to the dispatch brief:** the brief said "glob `*.pcc` (not
`_*.pcc`)" for this book. `find "$BOOK" -maxdepth 1 -iname '*.pcc'`
returns `_ultimate_campaign.pcc` — a leading underscore, the opposite of
what the brief stated, matching `loop-instruction.md`'s own corpus-shape
note that only `ultimate_equipment` and `ultimate_psionics` lack the
underscore. `epic-6-ui`'s receipt (`SD28-E6-F1-001`) recorded the
identical brief error for `ultimate_intrigue`; this is a second, book-2
confirmation the same brief-authoring mistake repeats per book, not a
new class of error.

**Book-directory naming note (not a defect):** the brief's "9 top-level
`.lst` files" is confirmed exactly (9). "34% of them sit in support/ and
_pfs/" (a repo-wide loop-instruction figure) does not apply file-for-file
to this specific book, which has `_pfs/` only (2 files) and no `support/`
dir at all — consistent with `loop-instruction.md`'s own per-book
breakdown ("UM, UE and UCam have only `_pfs/`").

### Step 0 / Step 1b cross-check — engine RuleSetId

`docs/work-inventory.json`'s `ultimate_campaign` entry carries
`"scope": "future_state"` and `"engine_rule_set": null`, and
`grep -rn "UltimateCampaign\|ultimate_campaign" src/
apps/desktop/src-tauri/src/ --include=*.rs` returns no matches — no
compiled `RuleSetId` variant exists for this book, the same shape
`epic-6-ui` found for `ultimate_intrigue`.

### What did not land, and why (decision-blocked — repo-wide precondition, same as `epic-3-uc`/`epic-4-um`/`epic-5-ue`/`epic-6-ui`)

`decision-blocked`: **Full Ultimate Campaign TDD ingest (new `RuleSetId`
variant, ingest code, `reach_gate.rs` claim, wired-integration audit,
on-screen verification) was not attempted this cycle.**

Repo-wide DoD item 3 precondition, re-checked fresh this cycle:
`cargo run --locked --bin v06_corpus_trap_report -- --audit` → **exit 2**,
the same 9 `key-differs-from-name` defects on
`pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_spells.lst`
(`Naturalist Summon Nature's Ally I..IX` filed under the bare `Summon
Nature's Ally I..IX` identity) already recorded by `epic-3-uc`
(`SD28-E3-F1-001`), `epic-4-um` (`SD28-E4-F1-001`), `epic-5-ue`
(`SD28-E5-F1-001`), and `epic-6-ui` (`SD28-E6-F1-001`) — a fifth
confirmation the blocker is still live, not a new instance and not
disagreement with any prior figure (Hard stops rule re-checked: agrees
exactly).

Separately, `ultimate_campaign` carries `scope: "future_state"` and
`engine_rule_set: null` — no compiled `RuleSetId` variant exists for this
book (re-confirmed above). Per the same reasoning `epic-6-ui` applied to
`ultimate_intrigue`: inventing a `RuleSetId` variant, parser hookup, and
reach-claim surface unilaterally inside a single per-book cycle is exactly
the cross-cutting engine work the "Hard stops" rule reserves for explicit
scope decision, not a silent per-book invention.

No ingest code, `RuleSetId` variant, or `reach_gate.rs` claim was written
this cycle; none is claimed as done. `./scripts/verify.sh` (full) was not
run against new production code because none was written; the
disk-preflight subset above stands as this cycle's only `verify.sh`
invocation, consistent with `epic-4-um`/`epic-5-ue`/`epic-6-ui`'s
precedent for a decision-blocked cycle.

### Retro events

`scripts/retro.py correction --subject "epic-7-ucam dispatch brief"
--claimed "glob *.pcc (not _*.pcc) for ultimate_campaign" --actual
"_ultimate_campaign.pcc — leading underscore, matching
loop-instruction.md's own corpus-shape note that only ultimate_equipment
and ultimate_psionics lack the underscore" --verified-by "find
~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_campaign
-maxdepth 1 -iname '*.pcc'"` — emitted this cycle to
`docs/retro/events/epic-7-ucam.jsonl`.

`scripts/retro.py deferral --actor epic-7-ucam --what "Ultimate Campaign
TDD ingest, new RuleSetId variant, reach_gate claim, wired-integration
audit, on-screen verification" --reason "repo-wide v06_corpus_trap_report
--audit exits 2 on the same pre-existing, out-of-scope ACG
key-differs-from-name finding epic-3-uc/epic-4-um/epic-5-ue/epic-6-ui
already recorded (fifth confirmation, not a new instance); separately,
ultimate_campaign is scope:future_state / engine_rule_set:null in
work-inventory.json — no RuleSetId variant exists, and inventing one
unilaterally in a single per-book cycle is exactly the cross-cutting
engine work the hard-stops rule reserves for explicit scope decision"
--scope "one book epic (epic-7-ucam)" --blocked-by "ACG
key-differs-from-name fix (out of SD-28 write scope); RuleSetId variant +
parser hookup for ultimate_campaign (not yet scoped to a single-book
cycle)" --tracked-at
"docs/release/SD-28-ultimate-book-content-ingestion/progress.md
SD28-E7-F1-001"` — emitted this cycle to `docs/retro/events/epic-7-ucam.jsonl`.

### Kanban

`epic-7-ucam` remains `IN-FLIGHT`, `Claimed-by: epic-7-ucam`, `Cycle-id:
SD28-E7-F1-001`. Not moved to `COMPLETE`. Next cycle against this card
should: (1) confirm whether the ACG audit finding has been fixed by
another epic/session, (2) confirm whether a `RuleSetId` variant for
`ultimate_campaign` has been added by an intervening cycle, and (3) only
once both are clear, proceed to Step 3 (TDD implementation) using this
cycle's Step 0/0b/1b findings above as the starting shape.

## Cycle `SD28-E8-F1-001` — `epic-8-uw` (Ultimate Wilderness)

**Claimed:** `epic-8-uw`, 2026-08-02T00:00:00Z. `RETRO_ACTOR=epic-8-uw`.

### Step 0 — shape (`v06_work_inventory`)

`cargo run --locked --bin v06_work_inventory` regenerated
`docs/work-inventory.json`. `ultimate_wilderness` entries carry
`"evidence": "no_compiled_rule_set_for_book"` and `"engine_book": null`
throughout (`python3 -c "import json; d=json.load(open('docs/work-inventory.json'));
print([b for b in d['books'] if 'wilderness' in str(b)])"` — the book's
units are keyed per-record inline, not a single `books[]` summary entry
in this snapshot; per-record `evidence`/`status` fields were read
directly from the `ultimate_wilderness:*` record entries).

### Step 0b — trap report

`cargo run --locked --bin v06_corpus_trap_report -- ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_wilderness`
→ exit **0**. Findings: 1479 `namespaced-key` (upstream `KEY:<namespace> ~
<leaf>` shape, e.g. `Witch Archetype ~ Flood Walker`), 2
`token-dense-record` (Shifter Claws: 10 `BONUS:VAR` tokens; Verdant
Bloodline: 11), 176 `governing-token-hidden-by-filter` (records carrying
`MULT`/`STACK`/`CHOOSE`/`SERVESAS`/`ASPECT` alongside `BONUS`/`PRE` that a
filtered grep would drop). None are defects — report states this
explicitly ("Everything above is legitimate upstream data... Nothing
here is a reason to fail a build."). 271+ `KEY` namespaces present; top
by count: Favored Class Bonus (68), Companion Advancement (52), Favored
Class Bonus Output (50), Shifter Aspect (45), Animal Trick (39), Wild
Talent (34), Animal Companion Feat (33).

### Step 1 / 1b — read + re-derive

Read `scope-draft.md`, `decisions.md`, `progress.md` (this file, tail).
Re-derived figures against source data, none transcribed:

```
find "$BOOK" -maxdepth 1 -iname '*.pcc'                    # -> _ultimate_wilderness.pcc (leading underscore — brief said
                                                             #    "glob *.pcc (not _*.pcc)"; wrong for this book, matching
                                                             #    loop-instruction.md's own note and the epic-6-ui/epic-7-ucam
                                                             #    precedent that only ultimate_equipment/ultimate_psionics lack it)
find "$BOOK" -maxdepth 1 -iname '*.lst' | wc -l              # -> 24
find "$BOOK" -iname '*.lst' | wc -l                          # -> 35  (recursive)
find "$BOOK/support" -iname '*.lst' | wc -l                  # -> 11
find "$BOOK" -path '*_pfs*' -iname '*.lst' | wc -l            # -> 0  (no _pfs/ dir in this book; brief's "34%"
                                                             #    repo-wide figure does not apply file-for-file — matches
                                                             #    loop-instruction.md's own breakdown: "UI and UW have only support/")
ls -d "$BOOK"/*/                                             # -> support/ only
grep -c 'CATEGORY:FEAT' "$BOOK"/uw_feats.lst                 # -> 136 (uw_feats.lst is 165 lines total)
grep -rn "UltimateWilderness\|ultimate_wilderness" src/ apps/desktop/src-tauri/src/ --include=*.rs
                                                             # -> only doc-comment references in pilot_compute.rs
                                                             #    (lines 14976, 19366); no compiled RuleSetId variant
```

(`$BOOK` = `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_wilderness`)

**Confirms brief error (repeat of epic-6-ui/epic-7-ucam finding, not a new
class):** the dispatch brief said "glob `*.pcc` (not `_*.pcc`)" for this
book; the file on disk is `_ultimate_wilderness.pcc` — leading
underscore. This is the third book-specific confirmation of the same
brief-authoring mistake (`epic-6-ui`/`SD28-E6-F1-001`,
`epic-7-ucam`/`SD28-E7-F1-001`, now `epic-8-uw`).

**Book-directory shape (not a defect):** 24 top-level `.lst` files, 11
more in `support/` (35 total, 31% in `support/`), 0 in `_pfs/` (this book
has no `_pfs/` directory) — consistent with `loop-instruction.md`'s own
per-book note ("UI and UW have only `support/`").

### Repo-wide precondition — re-checked fresh this cycle

`cargo run --locked --bin v06_corpus_trap_report -- --audit` → **exit 2**
(re-run live 2026-08-02, not transcribed from a prior cycle). Same 9
`key-differs-from-name` defects, unchanged from `epic-3-uc` through
`epic-7-ucam`: `advanced_class_guide/acg_spells.lst`'s "Naturalist Summon
Nature's Ally I..IX" filed under the bare "Summon Nature's Ally I..IX"
identity. This is a sixth confirmation the repo-wide precondition
blocking DoD item 3 (`v06_corpus_trap_report -- --audit` exits 0) is
still live — not a new instance, not a disagreement with any prior
figure.

`./scripts/verify.sh --only preflight-disk` → exit **0** (21% used, 385G
available on both the repo filesystem and the scratch-log filesystem).
No reclaim needed; `scripts/reclaim.sh --apply` still run at cycle-end
per step 8.

### Step 1b cross-check — engine RuleSetId

`docs/work-inventory.json`'s `ultimate_wilderness` record entries all
carry `"evidence": "no_compiled_rule_set_for_book"` and `"engine_book":
null`, and `grep -rn "UltimateWilderness\|ultimate_wilderness" src/
apps/desktop/src-tauri/src/ --include=*.rs` returns only doc-comment
prose references (`pilot_compute.rs:14976`, `:19366`) — no compiled
`RuleSetId` variant exists for this book. Identical shape to what
`epic-6-ui` found for `ultimate_intrigue` and `epic-7-ucam` found for
`ultimate_campaign`.

### What did not land, and why (decision-blocked — repo-wide precondition, same as `epic-3-uc`/`epic-4-um`/`epic-5-ue`/`epic-6-ui`/`epic-7-ucam`)

`decision-blocked`: **Full Ultimate Wilderness TDD ingest (new
`RuleSetId` variant, ingest code, `reach_gate.rs` claim, wired-integration
audit, on-screen verification) was not attempted this cycle.**

Two independent, previously-recorded blockers, both re-checked fresh:

1. `cargo run --locked --bin v06_corpus_trap_report -- --audit` exits
   `2` on the same pre-existing, out-of-scope ACG `key-differs-from-name`
   defects already recorded by `epic-3-uc` (`SD28-E3-F1-001`),
   `epic-4-um` (`SD28-E4-F1-001`), `epic-5-ue` (`SD28-E5-F1-001`),
   `epic-6-ui` (`SD28-E6-F1-001`), and `epic-7-ucam` (`SD28-E7-F1-001`) —
   DoD item 3 precondition, still failing repo-wide, out of this cycle's
   write scope (the defect is in `advanced_class_guide/acg_spells.lst`
   ingest, not in `ultimate_wilderness`).
2. `ultimate_wilderness` has no compiled `RuleSetId` variant
   (`engine_book: null`, `evidence: no_compiled_rule_set_for_book`
   throughout its work-inventory entries, re-confirmed above). Per the
   same reasoning `epic-6-ui` and `epic-7-ucam` applied to their books:
   inventing a `RuleSetId` variant, parser hookup, and reach-claim
   surface unilaterally inside a single per-book cycle is exactly the
   cross-cutting engine work the "Hard stops" rule reserves for explicit
   scope decision, not a silent per-book invention.

No ingest code, `RuleSetId` variant, or `reach_gate.rs` claim was written
this cycle; none is claimed as done. `./scripts/verify.sh` (full) was not
run against new production code because none was written; the
disk-preflight subset above stands as this cycle's only `verify.sh`
invocation, consistent with `epic-4-um`/`epic-5-ue`/`epic-6-ui`/`epic-7-ucam`'s
precedent for a decision-blocked cycle.

### Retro events

`scripts/retro.py correction --subject "epic-8-uw dispatch brief"
--claimed "glob *.pcc (not _*.pcc) for ultimate_wilderness" --actual
"_ultimate_wilderness.pcc — leading underscore, matching
loop-instruction.md's own corpus-shape note that only ultimate_equipment
and ultimate_psionics lack the underscore; third book-specific
confirmation of this brief error after epic-6-ui and epic-7-ucam"
--verified-by "find
~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_wilderness
-maxdepth 1 -iname '*.pcc'"` — emitted this cycle to
`docs/retro/events/epic-8-uw.jsonl`.

`scripts/retro.py deferral --actor epic-8-uw --what "Ultimate Wilderness
TDD ingest, new RuleSetId variant, reach_gate claim, wired-integration
audit, on-screen verification" --reason "repo-wide
v06_corpus_trap_report --audit exits 2 on the same pre-existing,
out-of-scope ACG key-differs-from-name finding
epic-3-uc/epic-4-um/epic-5-ue/epic-6-ui/epic-7-ucam already recorded
(sixth confirmation, not a new instance); separately, ultimate_wilderness
is engine_book:null / evidence:no_compiled_rule_set_for_book throughout
work-inventory.json — no RuleSetId variant exists, and inventing one
unilaterally in a single per-book cycle is exactly the cross-cutting
engine work the hard-stops rule reserves for explicit scope decision"
--scope "one book epic (epic-8-uw)" --blocked-by "ACG
key-differs-from-name fix (out of SD-28 write scope); RuleSetId variant +
parser hookup for ultimate_wilderness (not yet scoped to a single-book
cycle)" --tracked-at
"docs/release/SD-28-ultimate-book-content-ingestion/progress.md
SD28-E8-F1-001"` — emitted this cycle to `docs/retro/events/epic-8-uw.jsonl`.

### Kanban

`epic-8-uw` remains `IN-FLIGHT`, `Claimed-by: epic-8-uw`, `Cycle-id:
SD28-E8-F1-001`. Not moved to `COMPLETE` (the repo-wide precondition and
missing `RuleSetId` are not this cycle's to fix; moving to `COMPLETE`
would misrepresent DoD item 3 as satisfied). Next cycle against this
card should: (1) confirm whether the ACG audit finding has been fixed by
another epic/session, (2) confirm whether a `RuleSetId` variant for
`ultimate_wilderness` has been added by an intervening cycle, and (3)
only once both are clear, proceed to Step 3 (TDD implementation) using
this cycle's Step 0/0b/1b findings above as the starting shape.
