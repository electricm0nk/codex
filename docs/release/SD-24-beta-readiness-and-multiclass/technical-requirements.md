# SD-24 — Technical Requirements

> **Operating method:** see `./scope-draft.md`. Pre-loop prerequisites and normative requirements.

## 1. Pre-loop prerequisites

These are the hard preconditions the operator must verify before `/loop 1m /batch /goal ./loop-instruction.md` fires. Per `./loop-instruction.md §1` pre-launch checklist, the loop refuses to dispatch if any item fails.

### 1.1 Environment

- **`codex-tranche-5` kanban board exists.** Run `hermes kanban list-boards`; confirm `codex-tranche-5` is in the list.
- **`tranche/5-2` branch exists on origin.** Run `git ls-remote origin tranche/5-2`. The branch must resolve to a SHA.
- **Working tree clean on `tranche/5-2`.** Run `git status --porcelain`; output must be empty.
- **Skills loaded.** Skill list must include `wired-integration-discipline`, `identifier-discipline`, `kanban-claude-code-execution-receipt`.

### 1.2 Tier-1 launch-gate dependency

- **SD-23 closure PR merged to develop.** Per duracon 2026-07-21 09:24:59, SD-23's Tier-1 launch-gate for SD-24. Run `git log origin/develop --oneline | head -5` and confirm the SD-23 closure commit is HEAD of develop. If not, the loop does NOT start.

### 1.3 Credentials

- **Classic PAT present at `~/.config/gh/.claude_gh_token`.** Per memory `~/workspace/.hermes/profiles/god-emporer/memory.md §Classic PAT credential location`. Required for the kanban dispatcher's `active_pr` rule emissions.

### 1.4 Doctrine files

- **Repo-local canonical doctrine exists.** The repo-local copies at `~/workspace/repos/codex/governance/no-stub-mvp-doctrine.md`, `identifier-discipline.md`, `wired-integration-stubs-registry.md` are present. The harness reads the repo copy on wake-up, per the dual-canonical-doctrine.

### 1.5 Build counter

- **Capture develop's `Cargo.toml` workspace version into `./decisions.md §3`.** This is the build counter inheritance for SD-24's first concrete value.

## 2. Normative requirements (per cycle)

Per the operator-pinned doctrine and cycle mechanics:

### 2.1 RED → GREEN → re-audit

Per repo `AGENTS.md` §"Non-Negotiable Rules" — TDD is mandatory. Every code-bearing cycle captures RED → GREEN in the cycle artifact.

### 2.2 Dual-audit gate

Per identifier-discipline v1.5.0 + wired-integration-discipline v1.1.0: every code-bearing cycle runs both audits before marking `complete`. The dual-audit gate is the per-cycle discipline-of-record.

### 2.3 Identifier discipline

Per `identifier-discipline SKILL.md` "Per-cycle review checklist":

- No `sd<N>_*`, `SD<N>_*`, `Sd<N>` in any identifier added or modified.
- No `t_<hex>` kanban tokens in source code, comments, or test fixtures.
- No `SD-N-Ex...`, `AV-PAY-N`, "Tranche N chassis lane" strings in source-code comments.
- Functions, methods, constants, properties, Tauri commands are PascalCase.
- Variables and function arguments are lowercase camelCase.

### 2.4 Wired-integration discipline

Per `wired-integration-discipline SKILL.md` "Fully wired" checklist:

- The handler executes a real call to a real backend.
- The handler's `await` chain returns a result object that reflects the actual outcome.
- The UI updates to reflect the actual outcome.
- The state layer re-fetches or re-derives from the source of truth.

If any item fails, the cycle is not done. Fix and re-run the four-check audit.

### 2.5 Cycle artifact schema

Per `./loop-instruction.md §3`: every cycle writes `./artifacts/<epic>/<cycle-id>_cycle_receipt.md` with the standard schema.

### 2.6 Kanban card mint after done receipt

Per `./loop-instruction.md §2.3 step 10`: the kanban card is minted *after* the cycle artifact is written. Kanban does not dispatch.

### 2.7 Operator identity in commits

Per memory `~/workspace/.hermes/profiles/god-emporer/memory.md §Git identity`: every commit uses `Todd Hintzmann <todd@hintzmann.net>` as author.

## 3. Hard requirements (bundle-level)

### 3.1 Multiclass scope

Per `decisions.md §4` and Epic 5: Fighter + Wizard only. APG/ACG-class multiclass is out of scope.

### 3.2 Equipment corpus scope

Per `decisions.md §5` and Epic 6: strict 100% field coverage. Full PF1 core rules + APG + ACG + Bestiary 1 corpus.

### 3.3 Tauri command-surface scope

Per Epic 7: `appendToCharacter` + `recomputeCharacter` + `reSaveCharacter`. Other Tauri command work (e.g. `deleteCharacter`, `importCharacter`) is operator-pinned.

### 3.4 Build version

Per `decisions.md §3`: SD-24's first concrete value is `0.5.<build>` (no tranche-position increment; `tranche/5-2` carries tranche-base=5).

### 3.5 Publish mode

Per `decisions.md §6`: publish by move-not-copy. Workspace copy deleted on the publish commit.

## 4. Out-of-scope (deferred to follow-on bundles)

Per `risks-and-open-questions.md §5`:

- APG/ACG-class multiclass → deferred to follow-on (default: SD-25).
- Storage-tier structural convergence (e.g. SQLite-based character store) → deferred to follow-on.
- Inline mock libraries or "Would …" strings outside SD-24's file-touch partition → deferred to follow-on Wired Integration Cleanup epic.
- Identifier-discipline directory renames (`apps/desktop/src/sd<N>/` → `apps/desktop/src/<descriptive>/`) → deferred to follow-on.
- Equipment corpus extension beyond PF1 core rules + APG + ACG + Bestiary 1 → operator-pinned only.

## 5. Cross-reference

- `./scope-draft.md` — bundle intent
- `./decisions.md` — bundle-specific ADRs
- `./loop-instruction.md` — cycle mechanics
- `./acceptance-and-verification.md` — closure gates
- `./risks-and-open-questions.md` — risks + override flags
