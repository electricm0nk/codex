# v0.6 Alpha Release Swarm

> One-off, experimental release-closure lane sitting between SD-26 and SD-27. Intentional separation from the SD-N *chassis ceremony only*: **no** per-bundle kanban board, **no** `/loop /batch /goal` loop-instruction, **no** closure-epilogue doctrine, **no** per-cycle receipts. The **wired-integration doctrine itself still applies** — its four-check audit is load-bearing in §1.5, §4.4, and §7.1 and is `scope: universal` per `docs/governance/no-stub-mvp-doctrine.md`; only the per-cycle receipt ceremony is waived, recorded as an operator override in `docs/release/v0.6/risks-and-open-questions.md` per the doctrine's own waiver mechanism. The swarm stands alone. It will end when the alpha bar is met, when the lead reports it is blocked, or when the operator ends it at a go/no-go checkpoint (§6), whichever comes first.

**Operating entry point.** `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` is set to `1` in the `env` block of `~/.claude/settings.json` and is inherited by Claude Code sessions (it is *not* exported in the interactive shell — do not diagnose from `printenv`). The session that opens this document is the lead. Spawn teammates with explicit role names: `orchestrator`, `frontend`, `backend`, `qa`. In-process mode only.

**Pre-launch checklist (operator, before opening this doc as the lead):**

1. Run `/model sonnet` — the lead session cannot change its own model after launch, and swarm coordination runs on Sonnet per the standing model-tiering policy.
2. Cut `tranche/6` from `develop` HEAD and make committing `docs/release/v0.6/` (currently untracked) the first commit on it.
3. Confirm the version-bump task (§4.3) is on the task list: `0.5.99 → 0.6.0` across `apps/desktop/package.json`, `apps/desktop/src-tauri/tauri.conf.json`, `apps/desktop/src-tauri/Cargo.toml`, **plus** updating the `0.5.` anchor in `buildVersionTriple.test.ts:44-47` — that test documents a prior erroneous 0.6 bump that was reverted and will fail the bump until updated.
4. Either bring the observer lane live (§8.3.1 — start a listener and a tick loop) or accept that the observation lane is dark for this run.
5. Note: no tranche-specific CI workflow exists past `tranche-3-ci.yml`. Decide whether `tranche/6` needs one or whether `publish-tester-release.yml` + the promotion gates suffice.

---

## 1. Alpha bar (acceptance gate)

The swarm is finished only when **all** of the following hold, verified by the QA engineer, then attested by the lead:

1. A beta tester can install Codex (Tauri shell + Windows installer) without intervention. CI already builds the Windows MSI + NSIS installers (`publish-tester-release.yml`, `windows-latest` job); they are intentionally unsigned, so "without intervention" means *past* the expected SmartScreen "More info → Run anyway" prompt, which does not count as a failure.
2. A beta tester can create a character of any class or race from the four primary books — **CRB**, **Bestiary 1**, **APG**, **ACG** — and the character can be loaded from disk.
3. The tester can advance that character through at least 6 levels (multiclass required), select spells at each spell-gaining level, select feats, buy equipment (armor, weapons, gear), record bio notes, and manage money.
4. Every rules-engine calculation the tester can reach — ability scores, attack rolls, BAB/save progression, skill allocation, spell slot allocation, AC, durability, carry capacity, encumbrance, money conversion, level-up hit points, multiclass stacking — **matches the PCGen reference implementation** within the tolerances PCGen itself uses.
5. No `Coming soon` / `Not implemented` / placeholder UI surface survives in the v0.6 build. Every stubbed affordance in the desktop app is either fully wired against SD-26 JSON content or is recorded as a defect owned by the QA engineer with a reproduction.
6. **Red-green** Rust unit tests exist for every shipped calculation. The implementer (backend) authors the failing test first per AGENTS.md TDD rules; the QA engineer reviews, adopts, and owns the resulting catalogue and signs it off. QA ownership is of the *catalogue and sign-off*, not a gate in front of every test authorship.
7. The `tranche/6 → develop` PR lands green on CI with the wired-integration four-check audit re-run by the QA engineer and recorded in a swarm report artifact at `docs/release/v0.6/SWARM_REPORT.md`.

The lead does **not** declare alpha-ready on its own judgment; the QA engineer attests first, the lead reports the attestation to the operator, and the operator decides whether to ship.

---

## 2. Branch policy

- Single shared branch: **`tranche/6`**. This is the v0.6.x release line and a fresh cut from `develop`.
- All three implementation teammates (`frontend`, `backend`, `qa`) work on this branch directly. **No worktrees.** Per-teamate worktrees have produced a documented silent-failure mode in this environment; in-process teammates writing into the same checkout is the safer shape for this swarm.
- **Commit serialization.** The §5 file partition prevents content conflicts but not repo-state races (git index contention on concurrent commits, `cargo` target-dir contention during parallel test runs). Rule: teammates edit freely inside their lanes, but only **one teammate commits at a time** — a teammate announces "committing" to the lead, commits, announces "done"; the lead arbitrates if two ask at once. Long `cargo test` runs are likewise announced so the frontend build and backend test loop don't thrash the same target dir.
- The lead (this session) does not edit code. It writes the SWARM_REPORT.md at the end and opens the `tranche/6 → develop` PR (the **operator** merges it — see §7).
- If a teammate hits a worktree-style failure (writes that seem to land but do not appear on the branch), it stops and messages the lead with the exact repo-root and `git status --porcelain` output.

---

## 3. Spawn instructions (natural language)

Use this exact text to spawn the team from the lead session:

```
CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS is set. Stand up the v0.6 alpha
release-swarm on branch tranche/6 of /home/ubuntu/workspace/repos/codex.
Treat the doc at /home/ubuntu/workspace/repos/codex/docs/release/v0.6/release-swarm.md
as the source of truth for scope, file ownership, acceptance bar, and
handshake.

Spawn exactly four teammates, with these fixed names:
  - orchestrator  (this lead — already running)
  - frontend      (TypeScript / React (.tsx) / Tauri shell)
  - backend       (Rust / src/rules_core/)
  - qa            (PCGen parity + red-green catalogue)

Use Sonnet for every teammate. (The lead is already on Sonnet — the
operator ran /model sonnet before delivering this prompt, per the
pre-launch checklist; the lead cannot switch its own model.) Default
display mode is in-process; do not request split panes.

The lead does not implement; it spawns, partitions the task list, watches
for blockers, and writes SWARM_REPORT.md at the end. All three
implementation teammates start as soon as this prompt is delivered; they
do not wait for the lead.
```

---

## 4. Roles

### 4.1 Orchestrator (lead)

This is the session that opened this document. Responsibilities, in order:

1. Spawn `frontend`, `backend`, `qa` from the prompt above.
2. Build the shared task list. Walk the repo, the SD-26 JSON output, and the existing test files. For each unwired surface or missing rule path you find, mint a task with explicit file paths, acceptance criterion, and which teammate owns it. Use the dependency graph to mark items `blocked` until their prereqs land.
3. File-ownership partition is yours to enforce. If two teammates claim the same file, you make the call.
4. Do not implement. If you start typing edits, you are doing the wrong job.
5. Watch for blockers. When a teammate stops on an out-of-scope question, answer it or escalate to the operator.
6. Token-budget throttling. Default every teammate to **Sonnet** at spawn. The lead runs on **Sonnet** because the operator set it before launch (pre-launch checklist step 1) — the lead has no tool to change its own model, so if it finds itself on Opus or Fable it reports that to the operator rather than pretending to switch. Each teammate is pre-authorized for **up to 5 Opus-task turns per rolling 24h**, drawn from the swarm's weekly usage budget — the lead approves these on teammate request without bouncing to the operator. Beyond the 5/24h ceiling, escalate to the operator. Never auto-promote without an owned task. Compact task sizing: a single task is one bounded PR or one rule-path red-green test, not a multi-paragraph investigation. No speculative exploration — gates expensive investigations (full-graph SWE sweeps, deep multi-class parity sweeps beyond a real failing seed) behind an actual PCGen-mismatch reproduction filed by the QA engineer.
7. State visibility. After any of: a teammate going idle, a task completing, a blocker being filed, an Opus promotion being approved, the model/throttle tier changing — rewrite `docs/release/v0.6/SWARM_STATUS.md` from scratch. Flat ASCII, three sections, no big tables: **(a) Happening now** (one line per teammate: current task, model, last activity), **(b) Happened** (chronological tail of completions + decisions, last 30 entries), **(c) On deck** (queued + blocked-by with the owner of the prereq). Keep it readable in plain `cat` and `watch -n 5 cat` inside zellij — no color codes, no wide tables, no embedded HTML. The file is the swarm's single source of truth for operator-side dashboards; do not let teammates drift to alternate status surfaces.
8. At the end, gate-merge: run the four-check wired-integration audit, write `docs/release/v0.6/SWARM_REPORT.md` with the QA engineer's attestation attached, open the `tranche/6 → develop` PR, and report back to the operator.

### 4.2 Frontend teammate

- **Owns.** `apps/desktop/src/**/*.{ts,tsx,css}` — the desktop UI is **React (.tsx)**, not Svelte — Tauri shell config, the React component tree, UI test IDs, all in-process renderer code paths. Specifically: the Level Up dialog (`LevelUpDialog.tsx` — its `onAccept` is an empty closure today), the Equipment/Spell pickers (`ItemPickerModal.tsx` + catalog screens), the **Feat picker and Money panel (which do not exist yet and must be built)**, the Bio editor (`DetailsPanel` is fully written but never rendered — wire it in), and the 7 of 10 CharacterSheet tabs that currently render "coming soon."
- **Does not touch.** `src/rules_core/**` calculation logic. May add Tauri commands but must request the backend to add the Rust handler, not the Rust handler itself.
- **Tools.** Defaults plus the project's existing `wireit`-style task runners.
- **Done.** Every UI affordance is fully wired against the SD-26 JSON content (Wired Integration doctrine — no empty handlers, no mock data, no `would-have-done` return strings). The Tauri shell launches, the desktop installer builds, and a tester can drive the happy path in the dev build without console errors.

### 4.3 Backend teammate

- **Owns.** `src/rules_core/**` (PF1 calculation engine), the SD-26 JSON content surfaces at `data/**` (corpus + stubs), `schemas/**`, `src/oracle_validation/**` and `src/pcgen_import/**` (production side — QA owns the parity *tests*), Tauri command handlers in `apps/desktop/src-tauri/src/**` calculation-side, and all production `*.rs` files. (Note: there is no `src/data/` — the JSON content lives at top-level `data/` and `schemas/`.)
- **Does not touch.** Frontend React/TS files. May add a Tauri command signature, but the renderer-side glue is the frontend teammate's job.
- **Tools.** Defaults. Use `cargo test` for the local test loop; do not push until CI is green.
- **First task (fixed).** The `0.5.99 → 0.6.0` version bump: `apps/desktop/package.json`, `apps/desktop/src-tauri/tauri.conf.json`, `apps/desktop/src-tauri/Cargo.toml`, plus the `0.5.` anchor in `buildVersionTriple.test.ts:44-47` (which memorializes a prior erroneous 0.6 bump — this time it is intentional, on a real `tranche/6` cut, per `docs/architecture/release-pipeline.md`).
- **Done.** Every calculation the QA engineer flags as PCGen-divergent is either fixed or has a documented PCGen-mismatch defect ticket with a reproduction. Skill points, multiclass BAB/save stacking, spell-slot allocation, hit-die HP on level-up, and equipment AC bonus are the priority surfaces. Per AGENTS.md TDD rules, backend authors the failing test first and hands it to QA for catalogue adoption.

### 4.4 QA engineer

- **Owns.** `tests/**` Rust test catalogue (curation and sign-off), parity test tooling, `docs/release/v0.6/SWARM_REPORT.md` (final attestation only — the lead collates the rest). The QA engineer is the **owner of the test catalogue**: backend authors failing tests first (TDD, per AGENTS.md and §1.6) and QA reviews/adopts them into the catalogue; QA also writes parity and audit tests directly. Frontend does not add rules-engine tests.
- **Does not touch.** Production code. May write parity tooling and tests only. If a code defect is found, file a task and assign to the right teammate.
- **Reference data source.** The SD-26 JSON output at `data/corpus/` — 4,437 files across `core_rulebook/`, `advanced_players_guide/`, `advanced_class_guide/`, `beastiary/`. The PCGen parity machinery **already exists**: `src/oracle_validation/pcgen_runner.rs`, `src/pcgen_import/`, `scripts/pcgen-run-character.sh`, `scripts/pcgen-normalize-output.py`, and tests `tests/sd26_pcgen_runner.rs` / `tests/pcgen_runner_smoke.rs`. Do **not** mint acquisition tasks for a corpus — extend the existing runner and corpus. (There is no `tools/parity/`; that path was an error in an earlier draft of this doc.)
- **Done.** A signed attestation in `SWARM_REPORT.md` confirming (a) every shipped calculation has a red-green test, (b) every QA-found PCGen delta has been fixed or filed as an owned defect, (c) the four-check wired-integration audit (no forbidden tokens, no empty handlers, no mock-library leaks, no `Would …` strings) passed against the swarm's combined diff, (d) the operator's alpha bar in §1 holds.

---

## 5. File-ownership partition (enforced by the lead)

| Surface                                                          | Owner       |
| :--------------------------------------------------------------- | :---------- |
| `apps/desktop/src/**` (TS / React `.tsx` / UI assets)            | frontend    |
| `apps/desktop/src-tauri/src/**` (Rust handlers, lib)             | backend     |
| `src/rules_core/**` (PF1 calculations, JSON loaders)             | backend     |
| `data/**`, `schemas/**` (SD-26 JSON corpus, stubs, schemas)      | backend     |
| `src/oracle_validation/**`, `src/pcgen_import/**` (production)   | backend     |
| `tests/**` (Rust test catalogue, parity tests)                   | qa          |
| `scripts/pcgen-*` (PCGen runner scripts)                         | qa          |
| `docs/release/v0.6/**` (this doc + SWARM_REPORT.md)              | lead (orch) |
| `.github/workflows/**` (CI gate owners)                          | lead (orch) |

Two teammates touching the same file is a partition failure and the lead's job to settle. Cross-cutting changes (e.g. a new Tauri command paired with a React handler) get split: backend writes the Rust, frontend writes the React/TS glue, both reference the same agreed command name.

---

## 6. Operating norms

- **Wait for teammates, do not race the lead.** If the lead starts implementing, you are seeing a bug in the lead — say so out loud and tell it to delegate.
- **Rolling waves of five-to-six tasks per teammate, kept sized.** A "task" is a self-contained unit (a function, a test file, a wiring fix). Anything larger than one bounded PR is multiple tasks. The gap between current state (single-class Fighter 1–3 is the only path to a `Computed` receipt; 7 of 10 sheet tabs stubbed; Feat picker and Money panel absent — see `docs/architecture/status.md`) and the §1 bar is **far larger than one wave** — the lead refills each teammate's queue in waves of 5–6 and does not pretend one wave reaches the bar.
- **Go/no-go checkpoints.** At the end of every wave (or every ~4 hours of wall clock, whichever first), the lead posts a checkpoint to the operator: tasks landed, bar-distance estimate, token spend so far. The operator may narrow scope, pause, or end the swarm at any checkpoint; the swarm never silently rolls into another wave past a checkpoint the operator hasn't seen.
- **Avoid file conflicts.** Honour §5. If a teammate wants to touch a file outside its lane, it messages the lead.
- **SendMessage over polling.** Teammates communicate by name; the lead is auto-delivered on idle.
- **No session resumption.** This is a one-shot run; do not rely on `/resume` or `/rewind`. If the lead session dies, the swarm dies.
- **Hooks allowed, but minimal.** A `TeammateIdle` hook that emits "QA — please stop and report when you finish" is fine. Do not bolt on hooks that try to substitute for the wired-integration audit; that audit is the QA engineer's job.

---

## 7. Closure handshake

When the QA engineer signs off on the alpha bar in §1:

1. The lead runs the four-check wired-integration audit against the swarm's combined diff and pastes the raw grep output into `docs/release/v0.6/SWARM_REPORT.md`.
2. The lead **opens** the `tranche/6` → `develop` PR titled `Release Swarm v0.6-alpha — fully wired PCGen parity on CRB/B1/APG/ACG`. The lead does **not** merge it — the **operator** approves and merges tranche→develop PRs, per standing convention.
3. The lead posts a short report to the operator covering: tasks completed, defects filed (if any), the QA attestation, the PR URL, verification that the version triple reads `0.6.x` (including the updated `buildVersionTriple.test.ts` anchor), and any unresolved questions.
4. The lead does **not** merge to `develop` or `main`. Promotion beyond the open PR is the operator's call after the SWARM_REPORT review.

If at any point the lead reports the swarm cannot meet the alpha bar (out-of-scope blockers, missing reference data, conflicting requirements between SD-26 outputs and the desktop app, etc.), the lead stops, posts a blocker report, and waits for operator direction. The session stays alive until the operator decides whether to widen scope, narrow scope, or end the experiment.

---

## 8. Observation lane (operator-side, off-budget)

The operator runs a **second observer agent outside the swarm** under a separate subscription (minimax in the operator's current setup). The observer is not a teammate of the swarm, has no spawn relationship to the lead, is invisible to the shared task list, and consumes **operator-pool tokens, not swarm-pool tokens** — this is how visibility into the swarm stays cheap.

### 8.1 What the observer is allowed to read

- `docs/release/v0.6/SWARM_STATUS.md` — the canonical state. Sole writer is the lead (§4.1 step 7). The observer's job is to render it, not produce it.
- `~/.claude/teams/{team-name}/inboxes/*.json` — the lead's mailbox JSON for raw event stream between status-file refreshes (entries carry `from`, `to`, `content`, `timestamp`).
- The team's task list at `~/.claude/tasks/{team-name}/` — read-only.
- Public PR/CI status for the `tranche/6` branch.

### 8.2 What the observer is NOT allowed to do

- Write to `SWARM_STATUS.md`. The lead is its only writer; observer-side writes race the lead and corrupt the canonical view.
- SendMessage any teammate. The observer has no spawn relationship and no bus connection.
- Mint tasks, complete tasks, or modify the task list.
- Spawn its own teammates. It is a leaf.
- Read production code freely to "investigate." It may scan the file surface, but any code-level finding is filed back to the operator to be routed to the swarm's QA engineer; the observer does not produce defects directly.

### 8.3 Observer output

The observer renders one of:

- A flat ASCII view at `docs/release/v0.6/SWARM_LIVE.txt` (auto-refresh on a 30–60s cadence, designed for `tail -f` in a zellij pane).
- An HTML dashboard at `docs/release/v0.6/dashboard.html` (auto-refresh via `<meta http-equiv="refresh" content="5">` for browser viewing) plus a copied view at `~/swarm-observer/dashboard.html` for direct serving.

The observer picks one shape; the operator may switch by message. Both views are derived from `SWARM_STATUS.md` plus raw mailbox activity, so the canonical source never forks.

#### 8.3.1 v0.6-alpha intended address (operator-side — NOT currently live)

> **Status check 2026-07-23:** nothing is listening on port 9876, nginx is not running (the `location /swarm/` block exists on disk in `/etc/nginx/sites-available/default` but is not being served), and no cron/systemd tick drives `observer.py` — `~/swarm-observer/dashboard.html` and `SWARM_LIVE.txt` are stale manual snapshots. Before relying on this lane the operator must start a listener **and** a tick loop (`bash ~/.hermes/profiles/god-emporer/skills/release-swarm-observer/scripts/tick.sh` on a 30–60s cadence); otherwise the observation lane is dark for this run (pre-launch checklist step 4).

The intended address for the v0.6-alpha run is:

- **`http://10.0.0.134:9876/dashboard.html`** — second listener (Python `http.server --bind 0.0.0.0 --directory /home/ubuntu/swarm-observer` on this box). Pure HTTP, plain bytes, no TLS. Reachable from any device on the same LAN / Tailscale network as the box. Title in the response is `Release-Swarm Dashboard — v0.6 alpha`.
- **`http://10.0.0.134:9876/SWARM_LIVE.txt`** — companion ASCII view, designed for `tail -f` in a zellij side pane.

Diagnosing the address from a stale-cache symptom: a request to the URL must return the bytes whose `<title>` is `Release-Swarm Dashboard — v0.6 alpha`. A 200 with any other title (e.g. `Hermes Agent - Dashboard`) means the request is being served by the catch-all hermes vhost, NOT the observer — fix routing before continuing.

#### 8.3.2 Long-term address (operator-side, optional)

The intended long-term shape is a `location /swarm/` block on the existing hermes vhost pointing at `~/swarm-observer/`. Recommended operator-side patch into `/etc/nginx/sites-available/default`, placed BEFORE the catch-all `location /`:

```
    location /swarm/ {
        alias /home/ubuntu/swarm-observer/;
        autoindex off;
        default_type text/html;
        add_header Cache-Control "no-store, no-cache, must-revalidate" always;
    }
```

Apply with `sudo nginx -t && sudo nginx -s reload` after backing up the vhost. Note: the `location /swarm/` block must come **before** the catch-all `location /` block in the vhost — nginx uses longest-prefix match and the catch-all would otherwise win.

The second listener (8.3.1) and the nginx block (8.3.2) are not redundant — they coexist. The second listener is the working address; the nginx block is the production-shape address once the operator has time to land it.

### 8.4 Observer scaffold (built)

A minimal observer skill lives at `~/.hermes/profiles/god-emporer/skills/release-swarm-observer/` and consists of:

- `SKILL.md` — skill frontmatter + loadable body (description, when-to-use, hard guardrails, file inventory).
- `references/canonical-sources.md` — paths and formats for `SWARM_STATUS.md`, mailbox JSON, task list, with the explicit read-only rule.
- `references/serve-and-refresh.md` — nginx + zellij wiring; existing vhost gets a `location /swarm/` block, `tail -f` from any pane works against `SWARM_LIVE.txt`.
- `scripts/observer.py` — stdlib-only Python renderer; parses the lead's three-section `SWARM_STATUS.md`, reads the lead's mailbox JSON, reads task counts, writes `~/swarm-observer/dashboard.html` (single file, `<meta http-equiv="refresh" content="5">`) and `~/swarm-observer/SWARM_LIVE.txt` (flat ASCII for `tail -f`).
- `scripts/tick.sh` — wrapper for cron / systemd / manual loops.

Invocation:

```
bash $HOME/.hermes/profiles/god-emporer/skills/release-swarm-observer/scripts/tick.sh
```

The scaffold is operator-side and intentionally not under `docs/release/v0.6/` — that path is swarm-owned. If the operator wants the scaffold under repo control, file a follow-up task to move it; for v0.6 alpha it's a personal-side skill.

