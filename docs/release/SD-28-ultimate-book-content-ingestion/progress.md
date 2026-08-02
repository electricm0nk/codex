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
