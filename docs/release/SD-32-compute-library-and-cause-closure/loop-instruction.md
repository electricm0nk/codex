---
canonical: true
owner: god-emporer
status: planning-ready (chassis completed 2026-08-22 from SD-31 session)
date: 2026-08-22
template_source: ../../governance/loop-instruction-template.md
---

# SD-32 Loop Instruction

**This file is authored from `../../governance/loop-instruction-template.md` (Workflow-orchestrated
dispatch) with this package's specific overrides.** Read the template once for the dispatch
mechanism and dual-audit gate; read this file for everything bundle-specific. SD-31's own
`loop-instruction.md` is a useful worked example of "governed by template + per-bundle overrides";
this bundle follows the same shape.

## 0. Bundle at a glance

- **Branch:** `tranche/12` (cut from `tranche/11`'s tip after SD-31 closes; per `decisions.md §1`)
- **Board:** **local-file** `kanban.md` (no Hermes board; per SD-30 decisions §14a, retired
  2026-08-01)
- **Cadence:** N/A — dispatch is a live `Workflow` session, not a timer loop (see §2)
- **Epics / criteria:** 5 epics / 4 gates (G0 census, G1 shape, G2 engines, G3 closure invariant)
- **First concrete build value:** captured in §1.7, not left as a template placeholder

## 1. Pre-launch checklist

Every command in this section must be run for real during drafting, with its actual output pasted
below the command, before the bundle is marked planning-ready. A command written from memory or
assumption is not a verified precondition.

1. **Local kanban file reachable.** `test -f kanban.md && wc -l kanban.md progress.md`. Paste
   output. The Hermes kanban board was retired 2026-08-01; the local-file equivalent is the live
   dispatch input.
2. **Bundle branch is on origin, pushed and ahead of develop.**
   `git ls-remote --heads origin tranche/12` + `git log origin/develop --oneline | head -5`. Paste
   output.
3. **Predecessor bundle's closure PR is merged to develop.** `git log origin/develop --oneline |
   head -5`, confirm the SD-31 closure commit is HEAD or in HEAD's ancestry. Paste output.
4. **PAT present** at whatever path the operator's runbook expects. Per the standing doctrine,
   the classic PAT lives at `~/.config/gh/.claude_gh_token` (ghp_ prefix, broad-scope, can write
   rulesets); fine-grained tokens live in profile `.env` files and cannot. Paste
   `test -f ~/.config/gh/.claude_gh_token && echo PAT_PRESENT` output.
5. **Working tree clean** on the bundle branch. `git status --porcelain | wc -l` — expect `0`.
   Paste output.
6. **Doctrine gates.** `test -f ../../doctrine-external/identifier-discipline.md && test -f
   ../../governance/no-stub-mvp-doctrine.md && echo DOCTRINE_PRESENT`. Paste output.
7. **Build counter captured.** Read the live version source of truth:
   `apps/desktop/package.json` and `apps/desktop/src-tauri/tauri.conf.json` (the two files that
   actually carry the version; **`Cargo.toml` stays pinned at `0.1.0` and is not authoritative**).
   Write the literal next value into §1.7 below — e.g. "develop is at `0.11.x`; this bundle's first
   concrete build is `0.12.<next>`." A bundle is not planning-ready with `0.12.<build_at_launch>`
   still in the text.
8. **Artifact directories exist and are empty**, one per gate:
   `artifacts/gate-0-census-closure/`, `artifacts/gate-1-shape-closure/`,
   `artifacts/gate-2-engines/`, `artifacts/gate-3-closure-invariant/`, plus the existing
   `artifacts/HANDOFF.md` and `artifacts/UNMERGED-BRANCHES.md`.

## 2. Orchestration mode

Standing policy (pulled from the operator's global model-selection tiering — state it here once per
bundle rather than re-deriving it mid-launch):

- **Dispatch mechanism:** the in-harness `Workflow` tool, invoked from a live session — NOT
  `/loop /batch` (requires a human to type it per invocation, cannot run unattended) and NOT a
  standalone background process. Verify any per-bundle `scripts/workflow-dispatch.sh` actually
  contains a runnable subagent-invocation form before trusting it as the dispatch primitive; if
  it doesn't, dispatch via `Workflow`/`agent()` calls from the live session, reading the script's
  concurrency/tiering maps as the source of truth.
- **Default subagent model:** Sonnet (inherits session model unless overridden).
- **Tiering exceptions:**
  - Housekeeping (release notes, changelog, version bump, lint fixes) → Haiku.
  - Adversarial verification / final completeness scan / judge-panel steps → Opus (or Fable while
    it remains on-subscription for planning-tier work).
  - Everything else (real implementation, TDD cycles, audits, remediation) → Sonnet.
- **Concurrency shape:** decided explicitly per gate in §3 below, at authoring time — not derived
  live by whichever model launches the bundle.

### 2.1 Agent environment setup

Every dispatched agent should have `RETRO_ACTOR` set to its role name (e.g. `RETRO_ACTOR=gate-0-census`).
Without this, the retrospective log's by-actor breakdown falls back to opaque worktree directory
names, which become meaningless after the run ends. The `scripts/retro.py` tool resolves actor
identity in this order: `--actor` flag → `$RETRO_ACTOR` → worktree name → git config.

Every dispatched agent must also claim its per-cycle `CARGO_TARGET_DIR` immediately after
establishing it: `mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"`
(see `scripts/reclaim.sh`). Between builds, no passive liveness signal protects a live agent's
directory; without the claim file, a 27G target directory is protected only by an age heuristic,
and a sibling's reclaim sweep can silently delete work in progress.

### 2.2 Execution boundary — the launching session is always the orchestrator, never the executor

The session that plans, scopes, or launches a bundle is the orchestrator. §6's per-cycle procedure
— steps 1 through 9, especially step 3 ("implement the criterion TDD-style") — describes what
happens **inside a dispatched `agent()`/`Workflow` call**, never what the orchestrating session
does with its own `Edit`/`Write`/`Bash` tool calls. This holds with no exceptions: not for a "quick"
one-file fix, not mid-investigation when the context is already loaded, not because Plan Mode
approval already authorized the underlying change (approval authorizes the *work*, not a shortcut
around the *mechanism*).

Discovering that a cycle's real scope differs from what the cycle doc assumed is common and
expected (see §4) — it is a reason to **pause, record the corrected scope, and dispatch (or
re-dispatch) an `agent()` call with that scope**, never a license to keep executing inline because
the investigation already surfaced the fix.

**Self-check before any `Edit`/`Write`/`Bash`-that-mutates-a-file call while driving a bundle:** is
the target path under the bundle's implementation trees (e.g. `apps/desktop/`,
`apps/desktop/src-tauri/`, `src/`, `scripts/`) or otherwise part of a criterion's RED→GREEN work?
If yes, stop — that call belongs inside a dispatched `agent()`, not here. The orchestrating
session's own direct tool calls are reserved for: read-only investigation/scoping, authoring or
correcting this bundle's own planning docs (`loop-instruction.md`, `epic-breakdown.md`,
`decisions.md`, `cycles/*.md`), and git plumbing on those planning-doc commits — never on the
shipped-code diff itself.

**Corollary:** mint kanban done-receipts inside the dispatched agent, not from the orchestrating
session's own `Bash` calls. Kanban card completion is one more §6 per-cycle step; it happens
inside the dispatched agent's scoped task, never as a bare orchestrating-session Bash call.

## 3. Per-gate parallel/sequential map

This bundle's structure is **gates, not epics**, in the per-cycle dispatch sense. The five epics
in `epic-breakdown.md` are content-facing; the four gates are the actual sequencing constraint.
A gate is closed by its own set of cycles before the next gate opens.

| Gate | Touches | Parallel? | File-touch set (verified) | Gated on |
|---|---|---|---|---|
| G0 (census closure) | `scripts/census_*.py` (new), `data/corpus/*/...lst` enumeration | **yes** within gate, against the new walker | new scripts under `scripts/`; read-only on corpus | SD-31 closure PR merged to develop |
| G1 (shape closure) | extension of `scripts/coverage_ledger.py` to shapes (or new `scripts/shape_ledger.py`) | **yes** within gate | new scripts under `scripts/`; read-only on corpus | G0 met |
| G2 (engines) | `src/rules_core/pilot_compute/formula_interpreter.rs`, `src/rules_core/pilot_compute/bonus_stack_reader.rs` and generalisation | **yes** within gate, with `isolation: 'worktree'` | `src/rules_core/pilot_compute/*.rs`; new test files | G1 met |
| G3 (closure invariant) | new `scripts/shape_coverage_standing_gate.py` (mirror of `coverage_ledger.py`'s shape) + a `verify.sh` stage wiring | **serial** (single, well-named gate) | new scripts under `scripts/`; `scripts/verify.sh` | G2 met |

The Epic 5 protective sweep (self-erasure check across all 30 Rust generators, per
`epic-breakdown.md Epic 5`) fires **before** Gate 0 — it is not gated on any other phase. Scaling
engines over a generator that silently empties its own fixtures is the failure class Gate 2
depends on not existing.

When `parallel: yes`, every parallel agent that mutates files must get `isolation: 'worktree'` —
agents are mutating a shared checkout and will otherwise step on each other's working-directory
state even on disjoint files.

## 4. File-touch verification (required before §3 is filled in)

Run `ls` / `find` on every path named in this document, `content-unit-inventory.md`, and
`technical-design.md`. Confirm it exists as written. The key paths to verify before cycle-0:

- `src/rules_core/pilot_compute/formula_interpreter.rs` — the existing 9-of-10 engine.
- `src/rules_core/pilot_compute/bonus_stack_reader.rs` — the binding-layer precedent (wave 26,
  329 lines).
- `scripts/coverage_ledger.py` — the failing-closed pattern to mirror in Gate 3's standing gate.
- `scripts/verify.sh --list` — confirm the existing `coverage`-named stages this bundle will
  extend.
- `docs/work-inventory.json` — the live board; the `doneness_verdict()` table is the source of
  truth for any cycle that quotes a number.

If any path does not exist, find the real analogous file, correct the path in this document before
publish, or note it explicitly if it reflects a genuine design decision still open.

## 5. Concurrent-write protocol

Every cycle that commits and pushes to the shared bundle branch must use this exact retry
protocol — do not invent a per-bundle variant:

```bash
git fetch origin tranche/12 && git rebase origin/tranche/12 && git push origin HEAD:tranche/12
```

On non-fast-forward rejection, repeat up to 5 times. If it still fails after 5 attempts, stop and
report a `CLAIM-EXISTS` blocker — do not force-push. This applies to both the code commit and any
shared-state file every cycle touches (e.g. `progress.md`): re-fetch and re-read the file's
current content immediately before editing it, so a concurrent cycle's append isn't clobbered.

**Hard rule (added 2026-08-22, from SD-31 `artifacts/HANDOFF.md`):** **never `git stash` in this
repo.** The bare form stashes the whole shared checkout even from a subdirectory. It has bitten
this program three times.

## 6. Per-cycle procedure

**This procedure runs inside a dispatched `agent()`/`Workflow` call — see §2.2.** The orchestrating
session never performs steps 1–9 itself with its own tool calls.

1. Ensure the working tree/worktree is based on the latest bundle branch (§5's fetch+rebase).
2. `BASE_BRANCH=$(git merge-base HEAD origin/develop)` — define this before either grep block
   below, not between them.
   ```bash
   BASE_BRANCH=$(git merge-base HEAD origin/develop)

   # Identifier audit — bundle-tag leaks in diff
   # Trailing \b is deliberately omitted -- \b never matches between `_` and a
   # following word character, so a trailing \b silently fails to catch real
   # identifiers like `sd19_class_catalog`. Do not add it back.
   git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \
     | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'

   # Wired-integration four-check audit — forbidden patterns in shipping code
   git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \
     | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
   ```
3. Implement the criterion TDD-style: RED → confirm it fails for the intended reason → GREEN →
   run the relevant test suite.
4. Re-run the dual-audit gate on the final diff; both must show `OK_*`. A single-token violation is
   self-healable inline; re-audit and continue.
5. Write the cycle receipt to `artifacts/<gate>/<cycle-id>_cycle_receipt.md` (schema in §7).
6. Commit, then push via §5's retry protocol.
7. Update the shared progress doc in place via §5's protocol.
8. Mint the kanban card as a done-receipt, from inside this dispatched agent (per §2.2's
   corollary). Post-2026-08-01, "mint the kanban card" means editing the card's row in this
   package's local `kanban.md` (`Status` → `COMPLETE`) and appending the receipt to `progress.md` —
   there is no live Hermes board to mint a card on.
9. Report: criterion, files touched, commit SHA(s), dual-audit results, RED→GREEN evidence, receipt
   path, kanban card ID, discoveries, next-cycle plan.

## 7. Per-cycle receipt schema

```markdown
# Cycle <cycle-id> — <gate-name> / Criterion <n>

- **Card ID:** <kanban-card-id>
- **Commit SHA:** <sha>
- **Files touched:** <list>
- **Identifier audit result:** OK_NO_BUNDLE_TAGS / <violation list>
- **Wired-integration audit result:** OK_NO_TOKENS / OK_NO_NOOP_HANDLERS / OK_NO_MOCK_LEAKS / OK_NO_WOULD_STRINGS / <violation list>
- **Acceptance criterion:** <verbatim from acceptance-and-verification.md or epic-breakdown.md>
- **Status:** complete | returned-to-backlog | DISCOVERED-forked
- **Notes:** <judgment calls, deferred items, audit-exclusion requests>
- **Discovery forwards:** <list of ## DISCOVERED entries added>
- **Next-cycle plan:** <what the next cycle picks up>
```

## 8. Self-heal posture

- **Self-healable (resolve inline, exit GREEN):** dirty tree, single-token audit violation,
  unrelated test-setup breakage, build-counter out of sync, `## DISCOVERED` duplicates.
- **Non-self-healable (write `## Open blockers`, exit FAIL):** working tree diverged from the
  bundle branch needing manual rebase; two live cycles on conflicting files; a launch-gate
  dependency not actually merged; `## DISCOVERED` queue > 10 entries; RED → GREEN not preserved
  in the cycle receipt; a cycle finds `success: true` from a fake operation, an inline mock in a
  shipping module, or a "Would …" string in shipping code.

**Disk usage — check proactively, not reactively.** After every wave of `parallel: yes` cycles
completes (not just when something breaks), run `df -h /` and `git worktree list`; if usage is
climbing toward the disk's ceiling, prune merged worktrees and their `target`/build-cache
directories immediately — don't wait for a build to fail with `ENOSPC` first. A full sweep needs
~24 G; never place one under `/tmp`. Never remove a worktree that's still `locked` (an agent is
actively using it); confirm via `git worktree list`'s lock annotation and via `git status
--porcelain`/`git log <branch>..origin/<branch>` showing no unmerged, uncommitted work before
removing anything.

## 9. The five footguns from the SD-31 session

Mirrored from `artifacts/HANDOFF.md` so they sit alongside the cycle procedure rather than buried
in an artifact. Every cycle should treat these as load-bearing context.

1. **Worktrees are cut from the wrong base.** Every SD-31 wave since 15 had lanes land on a
   site-publish commit with no `docs/`, `data/`, `scripts/` or `schemas/` tree. Pin the base SHA
   in every dispatch and tell lanes to verify and `git reset --hard` it if wrong. Delete spent
   `site-publish/*` branches — they are what poisons the base.
2. **`find -newermt` lies on this box.** Agent-file mtimes run slightly ahead of system time, so
   it reports zero for a file written seconds ago. Use a Python mtime comparison.
3. **Omitting `model` on an `agent()` call inherits the orchestrator's model.** It does not
   default to Sonnet. One wave ran six Opus build lanes this way at 97% weekly quota. Set it
   explicitly every time: Sonnet for build and integration, Opus only for adversarial verifiers.
4. **Never `git stash` in this repo.** See §5. The bare form stashes the whole shared checkout
   even from a subdirectory.
5. **A ruling is not in force until it is committed.** A wave was dispatched telling lanes to read
   an operator ruling that existed only in the orchestrator's working tree. The lane checked
   every ref, found nothing, and correctly refused to reverse a pinned safety rule on a prompt's
   authority.

## 10. Placeholder-resolution checklist (final gate before "planning-ready")

Grep the whole bundle directory for `<...>`-style placeholders and template markers (e.g.
`0.12.<build_at_launch>`) before publish:

```bash
grep -rn '<[a-z_-]*>' docs/release/SD-32-compute-library-and-cause-closure/*.md
```

Every match must be resolved to a real value, or explicitly justified as intentionally deferred
(e.g., "filled in by Gate 0's first cycle, not at authoring time"). The `0.12.<build_at_launch>`
form in this file (§1.7), the README, and `decisions.md §1` are the three remaining template
markers; the first cycle replaces all three with the literal value.

## 11. Build counter resolution at first cycle (mirrors decisions.md §9)

**At the start of the first cycle** that produces a build, run:

```bash
CURRENT=$(grep '"version"' apps/desktop/package.json | head -1 | sed 's/.*"version": "\(.*\)".*/\1/')
echo "develop is at $CURRENT; SD-32 first concrete build will be 0.12.<next>"
```

And write the resolved value into:

- `README.md` "Bundle at a glance" — replacing `0.12.<build_at_launch>`.
- `decisions.md §1` — same.
- `loop-instruction.md §1.7` — capturing the literal `<next>` value with the command that
  produced it.

A bundle is not planning-ready until §1's checklist has been run end-to-end with this step
producing a literal value rather than a template placeholder.

## Cross-references

- `../../governance/loop-instruction-template.md` — the per-cycle dispatch procedure this file
  is authored from. Distinct scope: this template covers the per-cycle dispatch procedure;
  `docs/release/template/template.md` covers the release-folder's file index and bundle-snapshot
  table. Both must agree on the dispatch mechanism (`Workflow` tool, not `/loop /batch`) — if one
  changes, check the other.
- `../../governance/no-stub-mvp-doctrine.md`, `../../doctrine-external/identifier-discipline.md` —
  the two doctrine docs §6's dual-audit gate enforces inline.
- `../../governance/wired-integration-stubs-registry.md` — operator-granted stub exceptions.
- `artifacts/HANDOFF.md` — the SD-31 → SD-32 session handoff. Five footguns, two theses refuted,
  the anti-gaming apparatus, what is immediately actionable. Read first.
- `artifacts/UNMERGED-BRANCHES.md` — ten branches at the `tranche/11 → tranche/12` boundary and
  their recommended disposition order (genuine work first, GAMED branches alone, rescue branch
  untouched).
