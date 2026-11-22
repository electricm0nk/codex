---
canonical: true
owner: god-emporer
status: planning-ready (chassis completed 2026-08-22 from SD-31 session; launch-readiness remediation 2026-08-22, §1 run for real — see §1 and progress.md "Pre-launch receipt")
date: 2026-08-22
template_source: ../../governance/workflow-instruction-template.md
---

# SD-32 Workflow Instruction

**This file is authored from `../../governance/workflow-instruction-template.md` (Workflow-orchestrated
dispatch) with this package's specific overrides.** Read the template once for the dispatch
mechanism and dual-audit gate; read this file for everything bundle-specific. SD-31's own
`../SD-31-corpus-closure-grind/loop-instruction.md` is a useful worked example of "governed by template
+ per-bundle overrides" (SD-31 predates the `workflow-instruction.md` naming convention); this bundle
follows the same shape.

## 0. Bundle at a glance

- **Branch:** `tranche/12` (cut from `tranche/11`'s tip; SD-31's content reached `develop` via PR #374,
  merged 2026-08-22 — verified by content in §1 item 3; per `decisions.md §1`)
- **Board:** **local-file** `kanban.md` (no Hermes board; per SD-30 decisions §14a, retired
  2026-08-01) — 13 cards in claim-priority order
- **Cadence:** N/A — dispatch is a live `Workflow` session, not a timer loop (see §2)
- **Epics / criteria:** 5 epics / 4 gates (G0 census, G1 shape, G2 engines, G3 closure invariant).
  **Closure is the Definition of Done — all four gates' AT-32-* criteria met — never a wave or cycle
  budget** (operator ruling 2026-08-22, `decisions.md §2`).
- **First concrete build value:** `0.12.0` — bumped on `tranche/12` at launch-readiness remediation
  (SD-31 precedent `147f1c2b7`); published builds stamp `0.12.<build>` (§11, `decisions.md §9`).
- **PCGen oracle:** repo-local, at `artifacts/corpus/operator-supplied/pcgen` (git-ignored slot, SD-22
  precedent); never `~/workspace/repos/pcgen`. See §1 item 9 and §2.1.

## 1. Pre-launch checklist

Every command in this section must be run for real during drafting, with its actual output pasted
below the command, before the bundle is marked planning-ready. A command written from memory or
assumption is not a verified precondition. **Run from the repo root.** The same run is recorded as
the "Pre-launch receipt" in `progress.md` (SD-31 precedent: commit `1980d6b95`).

1. **Local kanban file reachable.** The Hermes kanban board was retired 2026-08-01; the local-file
   equivalent is the live dispatch input.
   ```bash
   B=docs/release/SD-32-compute-library-and-cause-closure
   test -f $B/kanban.md && wc -l $B/kanban.md $B/progress.md
   ```
   Output:
   ```
   # 2026-08-22, HEAD d60377a7e
      61 docs/release/SD-32-compute-library-and-cause-closure/kanban.md
     131 docs/release/SD-32-compute-library-and-cause-closure/progress.md
     192 total
   ```
2. **Bundle branch is on origin, pushed and ahead of develop.**
   ```bash
   git ls-remote --heads origin tranche/12
   git rev-list --count origin/develop..origin/tranche/12   # ahead
   git rev-list --count origin/tranche/12..origin/develop   # behind — expect 0
   ```
   Output:
   ```
   # 2026-08-22 (origin tip before the receipt push; pushed again after this commit)
   8d387c39ce82a68a944796c24850cfb97456c60f	refs/heads/tranche/12
   ahead=11 behind=0
   ```
3. **Predecessor bundle's content is merged to develop.** Verify **by content, not by commit count**
   — squash/merge-commit topology makes `--is-ancestor` lie (SD-31's tip is not an ancestor of
   develop even though every byte of it is there).
   ```bash
   gh pr view 374 --json number,state,mergedAt,headRefName,baseRefName --jq '"\(.number) \(.state) \(.mergedAt) \(.headRefName)->\(.baseRefName)"'
   git diff --stat origin/develop b1b7f4290 -- src scripts data docs/retro docs/release/SD-31-corpus-closure-grind | tail -1   # expect empty
   ```
   Output:
   ```
   # 2026-08-22
   374 MERGED 2026-08-22T19:53:56Z tranche/11->develop
   content-diff-lines=0      # git diff --stat ... | wc -l → nothing differs
   ```
4. **PAT present** at whatever path the operator's runbook expects. Per the standing doctrine,
   the classic PAT lives at `~/.config/gh/.claude_gh_token` (ghp_ prefix, broad-scope, can write
   rulesets); fine-grained tokens live in profile `.env` files and cannot.
   ```bash
   test -f ~/.config/gh/.claude_gh_token && echo PAT_PRESENT
   ```
   Output:
   ```
   PAT_PRESENT
   ```
5. **Working tree clean** on the bundle branch.
   ```bash
   git branch --show-current; git status --porcelain | wc -l   # expect tranche/12, 0
   ```
   Output:
   ```
   # 2026-08-22, immediately after commit d60377a7e
   tranche/12
   0
   ```
6. **Doctrine gates.**
   ```bash
   test -f docs/doctrine-external/identifier-discipline.md && test -f docs/governance/no-stub-mvp-doctrine.md && echo DOCTRINE_PRESENT
   ```
   Output:
   ```
   DOCTRINE_PRESENT
   ```
7. **Build counter captured.** The live version source of truth is `apps/desktop/package.json` and
   `apps/desktop/src-tauri/tauri.conf.json` (**`Cargo.toml` stays pinned at `0.1.0` and is not
   authoritative**). The tranche digit bumps once, at the tranche cut (SD-31 precedent `147f1c2b7`
   bumped `0.11.0` for `tranche/11`); published builds stamp `0.12.<build>` at publish time.
   ```bash
   grep -h '"version"' apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json | head -2
   ```
   Output (literal first concrete value for SD-32: **`0.12.0`**):
   ```
   # 2026-08-22, after commit 29160889d "feat(sd32): version bump 0.12.0 for tranche/12"
     "version": "0.12.0",
     "version": "0.12.0",
   ```
8. **Artifact directories exist**, one per gate plus the Epic 5 sweep (matches `artifacts/README.md`):
   ```bash
   ls -d docs/release/SD-32-compute-library-and-cause-closure/artifacts/{epic-5-protective-sweep,gate-0-census-closure,gate-1-shape-closure,gate-2-engines,gate-3-closure-invariant,corpus/operator-supplied}
   ```
   Output:
   ```
   docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied
   docs/release/SD-32-compute-library-and-cause-closure/artifacts/epic-5-protective-sweep
   docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure
   docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-1-shape-closure
   docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-2-engines
   docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-3-closure-invariant
   ```
9. **PCGen oracle present in the repo-local slot, at the pin.** Bootstrap with
   `scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR"` if the check fails. The slot is
   git-ignored (`.gitignore`: `docs/release/SD-*/artifacts/corpus/operator-supplied/**`); only its
   README ships.
   ```bash
   export PCGEN_REPO_DIR="$(git rev-parse --show-toplevel)/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen"
   export PCGEN_CORPUS_ROOT="$PCGEN_REPO_DIR/data"
   scripts/verify.sh --only preflight-oracle 2>&1 | tail -2
   git status --porcelain | grep -c operator-supplied   # expect 0 (slot is ignored)
   ```
   Output:
   ```
   # 2026-08-22; oracle at 7f818006e371188e5717fd18d74d18a420747fc6 (scripts/pcgen-oracle-pin.env)
   RESULT: PASS
   logs in /tmp/codex-verify-aCOe8n
   0                         # git status --porcelain | grep -c operator-supplied
   ```

## 2. Orchestration mode

Standing policy (pulled from the operator's global model-selection tiering — state it here once per
bundle rather than re-deriving it mid-launch):

- **Dispatch mechanism:** the in-harness `Workflow` tool, invoked from a live session — NOT
  `/loop /batch` (requires a human to type it per invocation, cannot run unattended) and NOT a
  standalone background process. This bundle has no `scripts/workflow-dispatch.sh`; §3 below is the
  concurrency/tiering source of truth and §2.4 is the script.
- **Subagent model: no default — `model` is set explicitly on every `agent()` call.** An omitted
  `model` inherits the orchestrating session's model (footgun 3, §9), it does not fall back to Sonnet.
- **Tiering:**
  - Housekeeping (release notes, changelog, version bump, lint fixes, branch sweeps) → Haiku.
  - Adversarial verification / final completeness scan / judge-panel steps → Opus (or Fable while
    it remains on-subscription for planning-tier work).
  - Everything else (real implementation, TDD cycles, audits, remediation) → Sonnet.
- **Concurrency shape:** decided explicitly per phase in §3 below, at authoring time — not derived
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

**PCGen oracle — repo-local, always.** Every dispatched agent exports, before any corpus command:

```bash
export PCGEN_REPO_DIR="$(git rev-parse --show-toplevel)/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen"
export PCGEN_CORPUS_ROOT="$PCGEN_REPO_DIR/data"
scripts/verify.sh --only preflight-oracle >/dev/null || scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR"
```

In a fresh worktree, `git rev-parse --show-toplevel` is the worktree root; the slot is git-ignored
so it will be empty there — the fallback fetch above repopulates it at the pin (86 MB, sparse cone).
No bundle document, prompt, or receipt references `~/workspace/repos/pcgen`; the operator-supplied
slot under `artifacts/corpus/` is the only oracle location this bundle knows
(`artifacts/corpus/README.md`).

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
correcting this bundle's own planning docs (`workflow-instruction.md`, `epic-breakdown.md`,
`decisions.md`, `kanban.md`, `progress.md`, `artifacts/<gate>/*.md` indexes), and git plumbing on
those planning-doc commits — never on the shipped-code diff itself.

**Corollary:** mint kanban done-receipts inside the dispatched agent, not from the orchestrating
session's own `Bash` calls. Kanban card completion is one more §6 per-cycle step; it happens
inside the dispatched agent's scoped task, never as a bare orchestrating-session Bash call.

**Dispatch first, report second.** SD-31 lost four full stalls this way — twice the operator had
to say *"you look idle"* / *"you stopped working again"* before work resumed. A wave finished, the
orchestrating session wrote a summary, and the turn ended without dispatching the next phase. The
summary **feels** like the deliverable. It is not. Before ending any turn while the bundle has
ready, undispatched work, dispatch it first — the summary then describes something that already
exists, rather than substituting for it. (`docs/retro/sd31-retrospective.md` "Changes for SD-32" #3.)

### 2.3 Retrospective event logging (every cycle)

Every dispatched cycle emits retro events during the work itself, not just a written summary at
the end — per `AGENTS.md`'s "Retrospective Logging" discipline. Git records what landed; it says
nothing about what nearly landed wrong or who caught it, and nothing survives the run except this
log. SD-31's own retrospective (`docs/retro/sd31-retrospective.md`) is grounded in its 1,940-event
log for exactly this reason.

- **When you catch an error, hit an incident, defer work, or redo something**, emit the event via
  `scripts/retro.py` at the moment it happens — not batched at cycle end.
- **Correction:** `scripts/retro.py correction --subject <who-was-wrong> --claimed <value> --actual <value> --verified-by <command-or-check>`. `--verified-by` is required.
- **Incident / deferral / rework:** `scripts/retro.py <type> ...` — `python3 scripts/retro.py help <type>` for that type's required fields.
- **`RETRO_ACTOR` must already be set** (§2.1) so the by-actor breakdown resolves to a real role.
- Full vocabulary: `docs/retro/schema.json`.

### 2.4 Creating the Workflow script

Dispatch is a script passed to the `Workflow` tool from the live orchestrating session — plain
JavaScript, not a shell script, not `/loop`. This bundle's phases are **gates**, not epics (§3
below), bracketed by a pre-G0 phase and the closure phase. **One `phases` entry per §3 row, same
titles; `phase()` calls fire in §3's gated order.** Every `agent()` call sets `model` explicitly;
every agent that mutates files inside a `parallel()`/`pipeline()` fan-out gets `isolation: 'worktree'`.
The card numbers are `kanban.md`'s `#` column.

```javascript
export const meta = {
  name: 'sd32-dispatch',
  description: 'SD-32 — compute library and cause closure, gate-sequenced dispatch',
  phases: [
    { title: 'Pre-G0 — Epic 5 protective sweep + boundary-branch review' },
    { title: 'Gate 0 — census closure' },
    { title: 'Gate 1 — shape closure' },
    { title: 'Gate 2 — engines' },
    { title: 'Gate 3 — closure invariant' },
    { title: 'Epics 1-3 — library, cause closure, class reachability' },
    { title: 'Closure — bundle epilogue' },
  ],
}
// cycleProcedurePrompt(card) embeds §6 verbatim + the card's row from kanban.md + the pinned base
// SHA (§6 step 1) + the §2.1 env block. A dispatched agent starts with zero context of this bundle.

phase('Pre-G0 — Epic 5 protective sweep + boundary-branch review')
await parallel([
  // card 1 mutates src/bin/*.rs (per-generator fixes) -> worktree isolation
  () => agent(cycleProcedurePrompt(card(1)), { model: 'sonnet', isolation: 'worktree',
          phase: 'Pre-G0 — Epic 5 protective sweep + boundary-branch review' }),
  // card 2 is branch housekeeping over LOCAL-ONLY branches (artifacts/UNMERGED-BRANCHES.md):
  // it must run in the primary checkout, never in a fresh worktree cut from origin.
  () => agent(cycleProcedurePrompt(card(2)), { model: 'sonnet',
          phase: 'Pre-G0 — Epic 5 protective sweep + boundary-branch review' }),
])

phase('Gate 0 — census closure')
// serial: card 4 (book onboarding) is sequenced behind card 3's census walk (kanban.md #4)
await agent(cycleProcedurePrompt(card(3)), { model: 'sonnet', phase: 'Gate 0 — census closure' })
await agent(cycleProcedurePrompt(card(4)), { model: 'sonnet', phase: 'Gate 0 — census closure' })

phase('Gate 1 — shape closure')
await agent(cycleProcedurePrompt(card(5)), { model: 'sonnet', phase: 'Gate 1 — shape closure' })

phase('Gate 2 — engines')
// One chain per engine: build/prove the engine (card 6 or 7), THEN its own corpus-wide run
// (card 8, "one cycle per engine", AT-32-G2-004). The two chains are disjoint files -> worktree.
await pipeline(
  [card(6), card(7)],
  c => agent(cycleProcedurePrompt(c), { model: 'sonnet', isolation: 'worktree', phase: 'Gate 2 — engines' }),
  (_, c) => agent(cycleProcedurePrompt(card(8), { engineFrom: c }), { model: 'sonnet', isolation: 'worktree', phase: 'Gate 2 — engines' }),
)

phase('Gate 3 — closure invariant')
await agent(cycleProcedurePrompt(card(9)), { model: 'sonnet', phase: 'Gate 3 — closure invariant' })

phase('Epics 1-3 — library, cause closure, class reachability')
// cards 10/11 gated on G1+G2, card 12 on G0 (decisions.md §6); disjoint files -> worktree
await parallel([10, 11, 12].map(n => () =>
  agent(cycleProcedurePrompt(card(n)), { model: 'sonnet', isolation: 'worktree',
        phase: 'Epics 1-3 — library, cause closure, class reachability' })))

phase('Closure — bundle epilogue')
// card 13 = §13 steps 1-4 (final-acceptance scan, retrospective, sweep, arch-docs + PR)
await agent(cycleProcedurePrompt(card(13)), { model: 'sonnet', phase: 'Closure — bundle epilogue' })
// §13 step 5 (release notes + version stamp) is housekeeping
await agent(releaseNotesPrompt(), { model: 'haiku', phase: 'Closure — bundle epilogue' })
```

A real SD-31 incident burned 97% of a week's Opus quota by omitting `model` on six build lanes —
never omit it. Adversarial verifiers, when a gate's closing cycle needs one, are the only
`model: 'opus'` calls in this bundle.

## 3. Per-phase parallel/sequential map

This bundle's structure is **gates, not epics**, in the per-cycle dispatch sense. The five epics
in `epic-breakdown.md` are content-facing; the four gates are the actual sequencing constraint.
A gate is closed by its own set of cycles before the next gate opens. One row per §2.4 phase.

| Phase | Cards | Touches | Parallel? | File-touch set (verified §4) | Gated on |
|---|---|---|---|---|---|
| Pre-G0 — Epic 5 protective sweep + boundary-branch review | 1, 2 | card 1: self-erasure assertion over the 29 Rust generators (`src/bin/{gen_,ingest_,enrich_}*.rs`), per-generator fix where needed; card 2: branch review in the primary checkout | **yes** (2 agents; card 1 in a worktree, card 2 in the primary checkout) | card 1: `src/bin/*.rs`, `tests/`, `data/corpus` reverted clean between runs; card 2: git refs only | §1 all green |
| Gate 0 — census closure | 3 → 4 | `scripts/census_independent.py` (new), `data/corpus/*/...lst` enumeration; then Epic 4 book onboarding | **serial** (card 4 behind card 3) | new scripts under `scripts/`; read-only on corpus; card 4: the ~7 count-pinning files per book | Pre-G0 met |
| Gate 1 — shape closure | 5 | `scripts/shape_ledger.py` (new), mirroring `scripts/coverage_ledger.py`'s fail-closed posture | **serial** (single card) | new scripts under `scripts/`; read-only on corpus | G0 met |
| Gate 2 — engines | 6 → 8, 7 → 8 | `src/rules_core/pilot_compute/formula_interpreter.rs`, `src/rules_core/pilot_compute/bonus_stack_reader.rs` and generalisation; per-engine corpus-wide run | **yes** (two engine chains, `isolation: 'worktree'`) | `src/rules_core/pilot_compute/*.rs`; new test files; `artifacts/gate-2-engines/` | G1 met |
| Gate 3 — closure invariant | 9 | new `scripts/shape_coverage_standing_gate.py` + a **new** `scripts/verify.sh` stage `shape-coverage-standing-gate` | **serial** (single, well-named gate) | new scripts under `scripts/`; `scripts/verify.sh` | G2 met |
| Epics 1-3 — library, cause closure, class reachability | 10, 11, 12 | library extraction (`src/rules_core/pilot_compute/`), cause closure by class, prestige gating at the `compute_class_chassis` call site | **yes** (`isolation: 'worktree'`) | per-card, disjoint; each card's first cycle states its file set in its receipt | 10/11: G1+G2; 12: G0 (`decisions.md §6`); dispatched after G3 for a single gated order |
| Closure — bundle epilogue | 13 | §13 | **serial** | planning docs, `docs/retro/`, `docs/architecture/`, `release-notes.md`, version files | all four gates met (Definition of Done) |

When `parallel: yes`, every parallel agent that mutates files must get `isolation: 'worktree'` —
agents are mutating a shared checkout and will otherwise step on each other's working-directory
state even on disjoint files. Card 2 is the one deliberate exception: its subject is local-only
branches that do not exist on origin, so it runs in the primary checkout and touches no tracked file.

## 4. File-touch verification (required before §3 is filled in)

Run `ls` / `find` on every path named in this document, `content-unit-inventory.md`, and
`technical-design.md`. Confirm it exists as written. Verified 2026-08-22 (launch-readiness
remediation), all present:

- `src/rules_core/pilot_compute/formula_interpreter.rs` — the existing 9-of-10 engine.
- `src/rules_core/pilot_compute/bonus_stack_reader.rs` — the binding-layer precedent (wave 26,
  329 lines).
- `scripts/coverage_ledger.py` — the failing-closed pattern to mirror in Gate 1's ledger and Gate 3's
  standing gate. **It is not wired into `scripts/verify.sh`** and `scripts/verify.sh --list` has
  **no `coverage`-named stage today** (34 stages, checked 2026-08-22). Gate 3 therefore **adds the
  first** such stage (`shape-coverage-standing-gate`); nothing is being "extended".
- `src/bin/derived_evaluator_fixture_check.rs` — the fixture-check gate CLI
  (`cargo run --locked --bin derived_evaluator_fixture_check`), library module
  `src/rules_core/derived_evaluator_fixture_check.rs`, fixture
  `tests/fixtures/rules_core/derived-evaluator-fixtures.json`. There is **no**
  `scripts/derived_evaluator_fixture_check.py`.
- `docs/work-inventory.json` — the live board; the `doneness_verdict()` table is the source of
  truth for any cycle that quotes a number.
- `artifacts/corpus/operator-supplied/pcgen/data` — the oracle at `PCGEN_ORACLE_SHA`
  (`scripts/pcgen-oracle-pin.env`), §1 item 9.
- Generator population for Epic 5: `ls src/bin/{gen_,ingest_,enrich_}*.rs | wc -l` → **29**
  (13 `gen_*` + 11 `ingest_*` + 5 `enrich_*`; `repair_*`, `v06_*`, `pi_*`, `corpus_*`,
  `declared_*` are not generators). Every document in this bundle that quotes a generator count
  quotes 29 and this command.

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
this program three times. Use `git show HEAD:<file> > "$SCRATCH/<file>"` to read a baseline.

## 6. Per-cycle procedure

**This procedure runs inside a dispatched `agent()`/`Workflow` call — see §2.2.** The orchestrating
session never performs steps 1–9 itself with its own tool calls.

1. Ensure the working tree/worktree is based on the latest bundle branch (§5's fetch+rebase), **and
   verify the base is real rather than assuming it** — footgun 1 below fired 27 times against a
   prose warning alone. The orchestrator captures `PIN=$(git rev-parse origin/tranche/12)`
   immediately before dispatching each wave and embeds the literal SHA in every prompt (the
   launch-time value is recorded in `progress.md` "Pre-launch receipt"). Run this and stop if it
   fails, before doing anything else:
   ```bash
   test -d docs && test -d data && test -d scripts && git merge-base --is-ancestor "$PIN" HEAD \
     || { echo "WRONG BASE — expected descendant of $PIN; reset before continuing"; exit 1; }
   ```
   On failure, `git reset --hard "$PIN"` and re-verify. Then export the §2.1 env block.
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
   package's local `kanban.md` (`Status` → `complete`) and appending the receipt to `progress.md` —
   there is no live Hermes board to mint a card on.
9. Report: criterion, files touched, commit SHA(s), dual-audit results, RED→GREEN evidence, receipt
   path, kanban card ID, discoveries, next-cycle plan.

### 6a. The §27 provisional-shape-default contract (row 17, `epic-7-shape-categorization-100`)

Any cycle that applies `decisions.md §27`'s provisional `SpecialQuality` default — or any other
placeholder/defaulted/provisional shape assignment `§27a` widens the scope to cover — **must** call
`scripts/shape_provisional_marker.py::stamp_provisional_default(record, reason)` on the corpus record
at the moment it applies the default, never write the marker fields (`data.shape_provisional_default`,
`data.shape_provisional_reason`) by hand. That is the only sanctioned way to set it, and it is what
makes the marker impossible to set silently: the marker and the value it accompanies land in the same
call, so a record carrying the default without going through this function is a detectable contract
violation (`scripts/row17_census.py --check` fails on a marker present with no reason; a marker
entirely absent from a record that should carry it is what `scripts/row17_census.py`'s `fallthrough`
count and `scripts/shape_ledger.py`'s `f0_reached_by=="fallthrough"` field exist to surface — see
below).

**Where cycles read this before dispatching row 17's own work:**

- `scripts/shape_ledger.py`'s per-row `f0_reached_by` field (`"not_ingested"` / `"measured_empty"` /
  `"fallthrough"`) distinguishes a genuinely-measured F0 (a real corpus record with zero DEFINE/BONUS
  tokens) from F0 reached by "nothing else matched" — `build_ledger`'s `f0_breakdown` aggregates this,
  and `pi_redacted_formula`/`f0_fallthrough_pi_redacted` flag the specific case where a token's VALUE
  is itself the PI-redaction marker (`decisions.md §24b`) rather than real formula content.
- `scripts/row17_census.py` is the single command that answers row 17's own question ("which units
  have a real shape, and which have a placeholder?"), per kind and per book:
  `python3 scripts/row17_census.py --output artifacts/gate-1-shape-closure/row17-census.json`. Its
  `totals.row17_honest_size` field is `fallthrough` + `provisional_default` (within the not-done
  population) — the actionable count row 17 must re-categorize once `no_record` reaches zero. Run
  `--check` in any cycle that touches the provisional-default marker, to fail closed on a malformed
  (reason-less) marker before it ships.
- Re-derive `row17_honest_size` fresh at dispatch time — do not carry forward a prior cycle's printed
  number (`decisions.md §17a`: validate an instrument before trusting a confident claim it produces).

## 7. Per-cycle receipt schema

```markdown
# Cycle <cycle-id> — <gate-name> / Criterion <n>

- **Card ID:** <kanban-card-id>
- **Commit SHA:** <sha>
- **Files touched:** <list>
- **Identifier audit result:** OK_NO_BUNDLE_TAGS / <violation list>
- **Wired-integration audit result:** OK_NO_TOKENS / OK_NO_NOOP_HANDLERS / OK_NO_MOCK_LEAKS / OK_NO_WOULD_STRINGS / <violation list>
- **Acceptance criterion:** <verbatim from acceptance-and-verification.md or epic-breakdown.md>
- **Corpus SHA:** <PCGEN_ORACLE_SHA from scripts/pcgen-oracle-pin.env, when any figure was re-derived>
- **Status:** complete | returned-to-backlog | DISCOVERED-forked
- **Notes:** <judgment calls, deferred items, audit-exclusion requests>
- **Discovery forwards:** <list of ## DISCOVERED entries added>
- **Next-cycle plan:** <what the next cycle picks up>
```

## 8. Self-heal posture

- **Self-healable (resolve inline, exit GREEN):** dirty tree, single-token audit violation,
  unrelated test-setup breakage, build-counter out of sync, `## DISCOVERED` duplicates, empty
  oracle slot in a fresh worktree (re-fetch per §2.1).
- **Non-self-healable (write `## Open blockers` in `progress.md`, exit FAIL):** working tree
  diverged from the bundle branch needing manual rebase; two live cycles on conflicting files; a
  launch-gate dependency not actually merged; `## DISCOVERED` queue > 10 entries; RED → GREEN not
  preserved in the cycle receipt; a cycle finds `success: true` from a fake operation, an inline
  mock in a shipping module, or a "Would …" string in shipping code.

**Disk usage — check proactively, not reactively.** After every wave of `parallel: yes` cycles
completes (not just when something breaks), run `df -h /` and `git worktree list`; if usage is
climbing toward the disk's ceiling, prune merged worktrees and their `target`/build-cache
directories immediately — don't wait for a build to fail with `ENOSPC` first. A full sweep needs
~24 G; never place one under `/tmp`. Never remove a worktree that's still `locked` (an agent is
actively using it); confirm via `git worktree list`'s lock annotation and via `git status
--porcelain`/`git log <branch>..origin/<branch>` showing no unmerged, uncommitted work before
removing anything.

## 9. Standing lessons and the five footguns from the SD-31 session

**Seven standing lessons** — the six from `docs/retro/sd31-retrospective.md` "Changes for SD-32"
(lines 126-153; this bundle's own predecessor's retrospective), plus the deferral-revisit lesson
from the same retrospective's body:

1. **Recurring incidents get a mechanical control, not a better-worded warning.** §6 step 1 above is
   the applied form — a command with a nonzero exit code, not prose asking an agent to be careful,
   because SD-31's own prose warning for the identical wrong-base-worktree failure fired 27 times.
2. **Documents get tests or expiry; every figure carries its re-derive command.** Package prose is
   the most-corrected artifact in the program and the only one with no gate. Any figure in a brief,
   card, or receipt **must carry the command that produces it** (and the corpus SHA when it came
   from the oracle), so a reader re-derives rather than trusts; anything that cannot be expressed as
   a command is marked as an estimate, in the text. This bundle's own scope README already retracted
   one headline figure ("1,049 formula shapes") under this rule.
3. **Dispatch first, report second.** §2.2's last paragraph. Applied from SD-31 wave 30 and it held.
4. **Gate 0 before engines.** §3's order is forced by the operator's requirement never to run a shape
   engine twice; book onboarding is a precondition (card 4), not an epic.
5. **Sum the piles, always.** THE-BOX caught 1,212 double-counted units and 298 in no lane at all
   purely by insisting the parts add up; `scripts/coverage_ledger.py` enforces it mechanically. **SD-32's
   epic arithmetic is not summable yet** (`scope-draft.md` "What SD-32 does NOT promise") — no cycle
   adds epic ceilings into a headline percentage until the overlaps are measured.
6. **Measurement waves are legitimate deliverables.** A cycle that banks zero units but changes the
   plan or finds a regeneration hazard is a closed cycle, judged on its receipt, not on the board.
7. **A ruling that defers a capability must name the condition under which it is revisited, and
   that condition must be checked, not remembered.** SD-31's no-formula-interpreter ruling sat
   unexamined for ~18 waves after its own stated precondition had already landed. `decisions.md §7`'s
   four open rulings (B1/B2/B4/B5; there is no B3 — the numbering is SD-31's `todo/blocked.md`'s)
   are exactly this shape — check them at every gate closure, not only when a card touches them.

**The five footguns**, mirrored from `artifacts/HANDOFF.md` so they sit alongside the cycle
procedure rather than buried in an artifact. Every cycle should treat these as load-bearing
context.

1. **Worktrees are cut from the wrong base.** Every SD-31 wave since 15 had lanes land on a
   site-publish commit with no `docs/`, `data/`, `scripts/` or `schemas/` tree. Pin the base SHA
   in every dispatch (§6 step 1 is the mechanical check). The poison was spent `site-publish/*`
   commits; the stale local branches that still exist at the tranche/12 boundary are `site-deploy`
   and `fix/site-deploy-page-workflow` (`artifacts/UNMERGED-BRANCHES.md §4`) — card 2 dispositions
   them.
2. **`find -newermt` lies on this box.** Agent-file mtimes run slightly ahead of system time, so
   it reports zero for a file written seconds ago. Use a Python mtime comparison or `git status --porcelain`.
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

Grep the whole bundle directory for `<...>`-style placeholders and template markers before publish:

```bash
grep -rn '<[a-z_-]*>' docs/release/SD-32-compute-library-and-cause-closure/*.md
```

Every match must be resolved to a real value, or explicitly justified as intentionally deferred.
As of the 2026-08-22 remediation the only permitted matches are **schema/receipt placeholders** —
this file's §7 receipt schema and §2.3 `retro.py` argument shapes, `content-unit-inventory.md`'s
field table (`<publisher>/<book>`, `<gate>/<cycle-id>`), `acceptance-and-verification.md`'s
`<engine>` in a command template, §6/§8/§12's `<branch>`/`<gate>`/`<cycle-id>`/`<gate-start-date>`,
and `decisions.md §1`'s version-scheme notation `<major>.<tranche-base>.<build>`. The old
build-at-launch template marker is **resolved** to `0.12.0` (§11) and must not reappear anywhere in
the bundle; `0.12.<build>` is the published-stamp notation, not a placeholder.

## 11. Build counter resolution (mirrors decisions.md §9)

The tranche digit bumps **once**, when the tranche branch is cut — SD-31 precedent commit
`147f1c2b7` (`feat(sd31): version bump 0.11.0 for tranche/11`). SD-32's bump to **`0.12.0`** landed on
`tranche/12` at launch-readiness remediation (2026-08-22; dispatched to a housekeeping agent per
§2.2 because `apps/desktop/` is shipping code). Published builds stamp `0.12.<build>` at publish
time. One derivation command, quoted wherever the value is quoted:

```bash
grep -h '"version"' apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json | head -2
```

The literal value is written in: `README.md` frontmatter `build_version_target` and the "Bundle at
a glance" table, `decisions.md §1`, this file's §0 and §1 item 7, `progress.md` "Pre-launch
receipt", and `risks-and-open-questions.md` risk 7 (now resolved). A bundle is not planning-ready
until §1's checklist has been run end-to-end with item 7 showing `0.12.0`, not a template marker.

## 12. Gate wrap-up (fires after every gate)

Lightweight, and distinct from §13's bundle-final closure — this runs at the end of **every**
phase (Pre-G0, G0, G1, G2, G3, Epics 1-3), not just at the very end:

1. **Retro summary for the gate's work window.** `scripts/retro.py summary --since <gate-start-date> --json`. Read it — don't just run it. Append a short "what the retro log shows" note to the gate's own closing cycle receipt: incident/correction/deferral counts, any recurrence key firing more than once.
2. **Worktree sweep for this gate's worktrees only.** `git worktree list`; remove any worktree used by this gate's now-merged, no-longer-live cycles. Never remove one still `locked` or carrying unmerged commits — confirm via `git worktree list`'s lock annotation and `git log <branch>..origin/<branch>` showing nothing unmerged, per §8.
3. **Check the open rulings** (`decisions.md §7` B1/B2/B4/B5) against the gate's findings — standing lesson 7.
4. **No PR here.** The bundle's single `tranche/12 → develop` PR is §13's job, fired once, as the bundle's own final epic.

## 13. Bundle closure epilogue (fires once, as the bundle's final epic — kanban card 13)

The pattern every bundle since SD-21 has used (`../SD-21/decisions.md §17`, "Closure Epilogue is a
standard part of every spec-domain handoff"), with the retro write-up folded in. **The trigger is
the Definition of Done — all four gates' AT-32-* criteria met (`acceptance-and-verification.md`) —
not a wave count, a date, or a token budget** (operator ruling 2026-08-22; `decisions.md §2`).

1. **Final-acceptance scan.** All four gates met **and every Epic 1-5 card at `complete`**
   (`decisions.md §10`, operator ruling 2026-08-22 — supersedes this step's earlier "complete or
   filed under `## Open blockers` with a named owner" wording). A card at `returned-to-backlog`,
   `in-progress`, or `DISCOVERED-forked` blocks closure, as does a card marked `complete` with a
   half of its criterion explicitly deferred. An `## Open blockers` filing pauses the bundle and
   requests an operator ruling; it never authorises closure past the card, and **no PR opens**
   while any Epic card is short of `complete`.
2. **Write the bundle's retrospective:**
   ```bash
   scripts/retro.py summary --since <SD-32 launch date> --json
   ```
   Write `docs/retro/sd32-compute-library-and-cause-closure-retrospective.md` in the shape
   `docs/retro/sd31-retrospective.md` uses, then **cite it from `references/README.md` in the same
   closure cycle** — not as a follow-up (this bundle's own chassis review had to fix exactly that
   gap for SD-31's retrospective; don't repeat it for this bundle's own).
3. **Full worktree/branch sweep** for the whole bundle, with a real count in `progress.md`. Also
   close out `artifacts/UNMERGED-BRANCHES.md`'s still-open dispositions if card 2 hasn't already
   resolved them — including the nine origin-side `worktree-wf_*`/`test`/`update-index` branches
   it lists as unlisted-at-capture.
4. **Architecture-docs refresh, graphify, PR, merge-conflict resolution** — `docs/release/template/template.md §6`.
5. **Release notes and version stamp** — `release-notes.md`'s `[Populated at closure]` section
   (Haiku housekeeping agent).

## Cross-references

- `../../governance/workflow-instruction-template.md` — the per-cycle dispatch procedure this file
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
  untouched). Nine of the ten are local-only.
- `artifacts/corpus/README.md` — the repo-local PCGen oracle slot (§1 item 9, §2.1).
- `AGENTS.md §Retrospective Logging`, `scripts/retro.py`, `docs/retro/schema.json` — the
  event-logging discipline §2.3 points at.
- `docs/retro/sd31-retrospective.md` — this bundle's predecessor's retrospective; the source of
  §9's standing lessons and the worked example §13 step 2 follows.
- `.claude/skills/stc-authoring/SKILL.md` — the Claude-Code-native rendering of both canonical
  templates, for a session auditing or authoring this bundle directly in this repo.
