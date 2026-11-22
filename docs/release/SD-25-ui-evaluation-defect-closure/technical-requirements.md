# SD-25 — Technical Requirements

> **Operating method:** see `./scope-draft.md`. Pre-loop prerequisites + normative requirements + out-of-scope deferrals.

## 1. Pre-loop prerequisites

These are the hard preconditions the operator must verify before launching `scripts/workflow-dispatch.sh`. Per `/governance/loop-instruction-template.md §1`:

### 1.1 Environment

- `codex-tranche-5` kanban board exists. Run `hermes kanban boards`; confirm.
- `tranche/5-3` branch exists on origin. Run `git ls-remote --heads origin tranche/5-3`.
- Working tree clean on `tranche/5-3`. Run `git status --porcelain | wc -l` (expect `0`).
- Skill `workflow-orchestrated-dispatch` is loaded. Run `hermes skills --profile god-emporer --list | grep orchestration`.
- Doctrines at `governance/identifier-discipline.md` + `governance/no-stub-mvp-doctrine.md` exist as files (NOT hermes-skill-loaded; per template §1 §6).

### 1.2 Tier-1 launch-gate dependency

- SD-24 closure PR merged to develop. Run `git log origin/develop --oneline | head -5`; confirm SD-24 closure is HEAD or in HEAD's ancestry.

### 1.3 Credentials

- Classic PAT present at `~/.config/gh/.claude_gh_token`. Run `test -f ~/.config/gh/.claude_gh_token && echo PAT_PRESENT`.

### 1.4 Build counter

- Read develop's `apps/desktop/package.json` + `apps/desktop/src-tauri/tauri.conf.json` (NOT root `Cargo.toml`); confirm `0.5.97` is the develop value; write `0.5.98` as SD-25's first concrete value into `decisions.md §4`.

### 1.5 Artifact directories

- `artifacts/{epic_1,epic_2,epic_3,epic_4,epic_5,epic_6,epic_7,epic_8}/` exist and are empty.

## 2. Normative requirements (per cycle)

### 2.1 TDD is mandatory

RED → confirm fails for the intended reason → GREEN → run the relevant test suite.

### 2.2 Dual-audit gate

Per `loop-instruction.md §6`: identifier audit + wired-integration four-check audit. Both must show `OK_*` to mark `complete`. Single-token violations are self-healable inline; re-audit and continue.

### 2.3 Identifier discipline

No `sd<N>_*`, `SD<N>_*`, `Sd<N>*`, `t_<hex>` in source code; `scripts/**/*.sh` and `scripts/**/*.py` are scoped into the audit per `loop-instruction.md §6` (SD-25 deviation from SD-24 to cover the new `scripts/` directory). PascalCase for functions / constants / Tauri commands; camelCase for variables.

### 2.4 Wired-integration discipline

Per `wired-integration-discipline SKILL.md`: real calls, real results, real UI updates, real state re-fetch. Cycle rejected if any item fails. **"Would …" strings ARE forbidden in shipping code** *unless* registered in `governance/wired-integration-stubs-registry.md` (which is how the StubAdapter gets an exception).

### 2.5 Cycle artifact schema

Per `loop-instruction.md §7`. Write to `artifacts/<epic>/<cycle-id>_cycle_receipt.md`.

### 2.6 Kanban card mint AFTER done receipt

Mint post-cycle; cycle artifact is the durable receipt. Kanban does not dispatch.

### 2.7 Operator identity in commits

`Todd Hintzmann <todd@hintzmann.net>` per memory.

### 2.8 Concurrent-write protocol

Per `loop-instruction.md §5`: `git fetch && git rebase origin/<branch> && git push origin HEAD:<branch>`. Retry up to 5 times; then `CLAIM-EXISTS`.

## 3. Hard requirements (bundle-level)

### 3.1 Workflow orchestrator, not /loop /batch

Per `/governance/loop-instruction-template.md §2` + skill `workflow-orchestrated-dispatch` + `AGENTS.md §7`.

### 3.2 Discovery-dominant

Epics 6 + 7 carry dynamic criteria. The cycle picker reads `## DISCOVERED` for both.

### 3.3 Build version

First concrete value: `0.5.98`. Per `decisions.md §4`.

### 3.4 Publish mode

Move-not-copy from workspace-side to `docs/release/SD-25-ui-evaluation-defect-closure/` on `tranche/5-3`. Workspace-side copy deleted on the publish commit.

### 3.5 Stubs Registry entry for StubAdapter

Criterion 3.3 lands an entry in `governance/wired-integration-stubs-registry.md` per the wired-integration doctrine.

## 4. Out-of-scope (deferred to follow-on bundles)

- **PCGen library build** — SD-26.
- **JSON cache build for 26 books** — SD-26.
- **Book-stub-manifest entries (Stubs Registry `book_stub` kind)** — SD-26.
- **Equipment corpus extension** beyond PF1 core rules + APG + ACG + Bestiary 1.
- **Storage-tier structural convergence** (deferred per SD-24's storage-tier deferral).
- **Hub-of-Hubs multi-system implementations** beyond the trait + StubAdapter.
- **Inline mocks / "Would …" strings outside bundle's file-touch partition.**
- **Identifier-discipline directory renames** (`apps/desktop/src/sd<N>/` → descriptive).

## 5. Cross-reference

- `./scope-draft.md` — bundle intent
- `./decisions.md` — bundle-specific ADRs
- `./loop-instruction.md` — cycle mechanics
- `./acceptance-and-verification.md` — closure gates
- `./risks-and-open-questions.md` — risks + override flags
- `/governance/loop-instruction-template.md` — canonical template
- `~/.hermes/profiles/god-emporer/skills/orchestration/workflow-orchestrated-dispatch/` — orchestrator skill
