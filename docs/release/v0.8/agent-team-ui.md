---
title: v0.8 — UI Catch-Up Sprint — Agent Team Instructions
status: planning
scope: apps/desktop
artifact_type: agent-team-brief
canonical_branch: tranche/14-ui
date: 2026-09-01
---

# v0.8 — UI Catch-Up Sprint — Agent Team Instructions

## 0. Why this file exists, and why it's thin

This is **not** an SD-N STC bundle (see `../template/template.md` and `.claude/skills/stc-authoring/`).
There is no `epic-breakdown.md`, no `workflow-instruction.md`, no `Workflow`-tool dispatch loop, and
no closure epilogue (architecture-docs truth-up, graphify, PR) attached to this file. That machinery
is built for multi-day corpus bundles with a fully-drafted spec. This sprint has neither: it is a
single time-boxed session (~10 hours, bounded by a Fable 5.1 credit window before quota reset)
against a UI codebase that has drifted behind several tranches of backend engine work, with no
pre-written spec of what drifted.

This file is the **live-session brief** a Claude Code lead session reads before spawning its team.
Everything downstream of "spawn the team" is coordinated by the lead in natural language per
`https://code.claude.com/docs/en/agent-teams`, not by a script. `AGENTS.md`'s Non-Negotiable Rules
still apply in full to every teammate — this file does not relax TDD, the no-stub doctrine, blocker
discipline, or retro logging. It only skips the STC paperwork.

### 0.1 What Codex actually is (read this before touching anything)

Codex is not an online game. It is a **character-sheet builder for an in-person, dice-at-a-table
Pathfinder 1e game**. Pathfinder 1e is an extremely large ruleset (tens of thousands of individual
rules interactions: feats, traits, class features, spells, conditions, equipment, prerequisites).
The product's entire value is correctly interpreting that ruleset and producing a character sheet a
human can actually play from. `PCGen` is the closest prior art — the "archaic grandfather" of this
effort — and it is useful **only** as a completeness bar (what selections a PF1e character sheet
tool needs to offer at all). It is explicitly **not** a UX model: it is dated, dense, and clunky.
This sprint is trying to be the slick, modern version of what PCGen gets right functionally.

The rules engine (repo-root `src/`) is out of scope for this sprint (§4.2) and is being worked by a
separate, concurrent session. This team's job is entirely the **player-facing character-building
UI** at `apps/desktop/src/characterHub/`, which is a partially-wired prototype against a narrower
rule subset than the engine now supports.

## 1. Required minimum handoff (AGENTS.md gate)

| Field | Value |
|---|---|
| Exact objective | Make `apps/desktop`'s character-creation and level-up UI (`characterHub/`) walk a player through every selection a real PF1e character sheet requires, and surface what they qualify for as they go — closing the gap between the narrow rule subset the prototype was wired against and what the backing engine now supports, via small, independently-verifiable, TDD tickets. |
| Exact target repo/workdir | `/home/todd/GITHUB/codex-ui`, working tree checked out to `tranche/14-ui`. |
| Exact allowed write scope | `apps/desktop/**` (both `src/` and `src-tauri/`) plus this folder's own status log. See §4 for per-agent subdivision. **Not** `site/`, **not** repo-root `src/` (the engine crate) — see §4.2. |
| Exact required reads | This file; `AGENTS.md` (root); `CLAUDE.md` (root); each agent's own owned subtree per §4. |
| Explicit non-goals | No SD-N bundle ceremony (§0). No `site/` work. No engine-internals changes (§4.2). No rules-computation logic written in TypeScript — see §3.1. DM Toolkit work only as the stretch queue in §3.5, never ahead of the character-sheet punch list. No merge to `develop`, no PR, no architecture-docs/graphify closure epilogue — this session stops at a clean `tranche/14-ui` with committed, verified tickets; merge-upward is a separate operator decision. |
| Verification commands | `npm run typecheck` and `npm test` (from `apps/desktop/`); `npm run tauri:check` (from `apps/desktop/`, equivalent to `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml`); `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` for Rust-side unit tests. Every ticket names which of these it closes on. |

## 2. Operating mechanism: Claude Code Agent Teams (native, experimental)

Use Claude Code's built-in agent-teams feature, not a `Workflow`-tool script and not manually
launched separate sessions/panes.

**Before spawning anything**, confirm `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` is set (shell env or
`settings.json`). If it is not set, stop and ask the operator to set it — do not fall back to plain
subagents and call it a team; the whole point of this file is the shared task list and direct
teammate-to-teammate messaging that only the real feature provides.

**Display mode**: `~/.claude/settings.json` sets `"teammateMode": "tmux"`, which requires `tmux` to
be installed on the host (confirmed present at time of writing). Each teammate renders in its own
split tmux pane rather than the single-terminal in-process default — the operator can watch all
four work live and click into any pane to intervene directly. This is a display choice only; the
coordination mechanism underneath (shared task list, mailbox, direct teammate messaging) is
identical either way. Note this is a real tmux session nested in whatever terminal this Claude Code
session is already running in (which may itself be a Herdr-managed pane) — it is not a Herdr pane
itself; Herdr is not one of the display-mode backends Claude Code's Agent Teams feature supports
(only `tmux` and iTerm2's `it2` CLI are).

The **lead session is the orchestrator**. It does not need to be separately spawned — the human
operator's live Claude Code session, once it has read this file, *is* the orchestrator.

### 2.1 How to invoke

There is no separate launch command — Agent Teams spawns from natural language inside an ordinary
interactive session, once the env var and `teammateMode` above are in effect (a new session, or a
settings reload, picks them up). Concretely, in the orchestrator's own session:

1. Set the orchestrator's own model/effort first (§2's table): `/model opus`, then `/effort low`.
2. Tell the lead to read this file and start: e.g. *"Read `docs/release/v0.8/agent-team-ui.md` and
   spawn `scout` per §3.1."* The lead reads the brief, confirms the env var and file are in place,
   and calls the `Agent` tool with `name: "scout"` — that call is what actually creates the
   teammate; naming it while agent teams are enabled is what makes it a teammate instead of an
   ordinary subagent.
3. Do not ask it to spawn `frontend`/`backend`/`qa` yet — §3.1 is explicit that those wait for
   `scout`'s punch list.
4. Once tmux split panes appear, each teammate's pane is independently addressable — click into one
   to talk to that teammate directly, or stay in the lead's pane and address teammates by name.

The lead spawns four named teammates:

| Agent | Role | Model | Effort | Why |
|---|---|---|---|---|
| orchestrator (lead, the operator's own session) | Triage, ticket-cutting, blocker calls, commits | Opus 5 (`/model opus`) | Low (`/effort low`) | Lowest volume, highest leverage — a scoping mistake here compounds across every downstream teammate. Opus's reasoning ceiling helps catch bad ticket boundaries and scope creep the fastest teammate model might wave through; low effort keeps the many small, frequent triage decisions (one per scout finding, one per QA verdict) snappy rather than over-thinking each one. Not part of the Fable-spend goal — the operator drives this session directly, so it barely touches that credit pool either way. |
| `scout` | Player-journey gap-audit and ongoing discovery | Fable 5.1 (`claude-fable-5`) | inherits lead's effort | High-leverage (a sloppy audit poisons the whole punch list) but bounded by the orchestrator explicitly *not* treating its output as pre-approved scope (§3.1) — a real review gate sits between scout's output and any committed work, which makes this a reasonable role to spend Fable credits on. |
| `frontend` | React/TypeScript implementation | Fable 5.1 (`claude-fable-5`) | inherits lead's effort | This is where the volume actually lives — many small, independently-verifiable tickets, each gated by a failing test and then QA before anything is committed. The intended target for spending down the credit window. |
| `backend` | Tauri/Rust bridge implementation | Fable 5.1 (`claude-fable-5`) | inherits lead's effort | Same reasoning as `frontend` — narrow, mechanical wiring tickets, each with a hard verification command. |
| `qa` | Verification, not implementation | Sonnet 5 (`claude-sonnet-5`), deliberately **not** Fable | inherits lead's effort | QA's whole job is catching what `frontend`/`backend` get wrong. If implementers and their reviewer are the same model, any systematic blind spot that model has shows up in the mistake *and* in the check meant to catch it — correlated failure. A different model buys a genuinely independent second opinion, which matters more here than the credits saved by keeping QA on Fable too. |

Name the model explicitly in each spawn prompt (per the docs, an explicit name in the spawn prompt
wins over every other model-selection rule) — for `scout`/`frontend`/`backend` this guarantees Fable
5.1 actually gets used instead of silently falling back to the lead's model; for `qa` it guarantees
the *opposite* substitution doesn't happen (a default that would otherwise inherit the lead's model
is fine here, but naming it explicitly removes any ambiguity). Per the docs, teammates always
inherit the lead's effort level — effort cannot be set per-teammate at spawn time, only the model
can be chosen per-teammate.

Spawn `scout` alone first (§3.1). Do not spawn `frontend`/`backend`/`qa` until the scout's punch
list exists — there is nothing bounded to hand them before that.

## 3. Sequence

### 3.1 Scout: play through character creation as a player, not an audit of code (time-box ~60 min)

This is a **user-journey walkthrough**, not a command-coverage diff. The scout puts itself in a
player's seat: *I am rolling a new PF1e character. What do I need to select, in what order, and
what should the sheet tell me I now qualify for as a result of each choice?*

Spawn prompt for `scout` should include, verbatim or close to it:

> You are read-only — do not edit any file. Walk the character-creation and level-up flow in
> `apps/desktop/src/characterHub/` the way a player actually would, step by step: race → class →
> ability scores → skills → feats → traits → languages → equipment → spells (if a caster) → review.
> At each step, ask two questions: (1) does the UI actually offer every selection a PF1e character
> sheet requires at this step (use `PCGen`'s PF1e character-sheet output purely as a checklist of
> *what selections exist* — do not use PCGen's interface as a model, it is dated and not the bar
> we're aiming for), and (2) once a choice is made, does the sheet surface what it now qualifies the
> character for (e.g. picking a race should immediately show its bonus feat/skill eligibility;
> picking a class level should show which feats/talents/spells just became available; ability score
> increases should ripple into every dependent field visibly)? Read `characterHubModel.ts`,
> `CharacterHubPage.tsx`, `CreateCharacterForm.tsx`, `LevelUpDialog.tsx`, and the per-tab models
> (`featsTabModel.ts`, `skillsModel.ts`, `spellsTabModel.ts`, `classFeaturesModel.ts`,
> `racialTraitsModel.ts`, `weaponsTabModel.ts`, `encumbranceTabModel.ts`, `petsTabModel.ts`) to see
> what's actually wired versus stubbed or missing outright. Do not speculate about whether the
> engine can support a given selection — that is out of scope for you; assume the engine question
> gets answered separately and just record what the UI is missing. Where a gap is genuinely
> ambiguous in shape, list it as an open question rather than inventing a design — that's the
> orchestrator's job once tickets are cut. Write your findings to
> `docs/release/v0.8/scout-gap-audit.md` as a flat, numbered punch list ordered by where in the
> player's journey the gap occurs, each tagged `[frontend]` or `[backend]` (backend only ever means
> "wire an existing engine capability through Tauri," never "extend the engine" — see the file's
> §4.2), with the file(s) it concerns and a one-line description of what a player would be stuck on.
> Message the lead when done.

The orchestrator reads `scout-gap-audit.md`, does **not** treat it as pre-approved scope, and
triages it into tickets per §3.2 before handing anything to `frontend`/`backend`.

### 3.2 Orchestrator: triage into tickets

Each ticket must independently satisfy AGENTS.md §Required Minimum Handoff (a punch-list line is
not a ticket). A ticket is right-sized when it:

- touches files owned by exactly one of `frontend` or `backend` (§4.1) — a ticket that needs both
  is two tickets, sequenced, not one shared one;
- has a named verification command from §1;
- does not require touching repo-root `src/` (§4.2) — if it would, that is a blocker, not an
  extension of the ticket: file it as `## Open blockers` in `scout-gap-audit.md` and escalate to
  the operator (AGENTS.md Blocker Discipline) rather than expanding scope into the engine;
- is TDD-shaped: a failing test can be written before the implementation.

If a gap requires touching repo-root `src/` (the engine), do not cut it as a ticket — see §4.2.

Cut tickets into the **shared task list** (`TaskCreate`), not into chat messages — this is what
lets `frontend`/`backend` self-claim without the orchestrator relaying each one by hand. Batch-cut
as much of the triaged punch list as is ticket-shaped in one pass (aim for the full 5-6-per-teammate
buffer up front, not one at a time), so a teammate never goes idle waiting on the orchestrator to
notice it finished. Use `TaskCreate`'s dependency field for any ticket that must land after another
(e.g. a `backend` command before the `frontend` ticket that calls it) instead of manually sequencing
by chat — the task list unblocks the dependent ticket automatically once its dependency completes.

### 3.3 Frontend / backend: implement

Standard AGENTS.md discipline applies without exception: red test first, confirm it fails for the
right reason, smallest implementation to green, no stubs (`wired-integration-discipline` skill),
no scope creep beyond the claimed ticket. On completion, message `qa` directly by name (not the
lead) with: files changed, the verification command run, and its actual output — not "should work"
— then immediately self-claim the next available unblocked task from the shared list without
waiting to be told. This is the whole point of the shared task list: the orchestrator is not in
the loop between one ticket finishing and the next one starting. If the list is ever empty, idle
and wait rather than inventing scope — an empty list means the orchestrator needs to cut more
tickets, not that the teammate should improvise one.

`frontend` never reimplements PF1e rules logic (bonus math, prerequisite checks, DC calculation,
eligibility) in TypeScript, even for something that looks trivial. Every rule-bearing value the UI
displays comes from a Tauri command backed by the engine. If a ticket's UI gap turns out to need a
value or check the engine doesn't expose yet, that is a `backend` wiring ticket (or, if the engine
itself doesn't compute it at all, an escalated blocker per §4.2) — not something `frontend` works
around locally. `frontend` may do purely presentational derivations (sorting a list, formatting a
number for display) that carry no rules judgment of their own.

Neither `frontend` nor `backend` runs `git commit` or `git push`. This sprint runs all teammates
against one shared working tree on `tranche/14-ui` (no per-agent worktrees), so a mid-flight commit
by a teammate risks capturing another teammate's in-progress edit. Report done to `qa` directly
(above); the orchestrator commits only after `qa` confirms (§3.4).

### 3.4 QA: verify, then orchestrator commits

`qa` does not implement. `frontend`/`backend` message `qa` directly when a ticket is done — `qa`
does not wait for the lead to relay it. For each ticket:

1. runs the ticket's named verification command itself (does not trust the dev's self-report);
2. checks the diff against the no-stub doctrine's four-check audit (`wired-integration-discipline`
   skill) — no empty handlers on user-facing affordances, no fixture-only data in a production
   path, no `success: true` masking unfinished work;
3. reports pass/fail with the actual command output to the lead.

Only after `qa` confirms does the orchestrator `git add`/`git commit` that ticket's diff to
`tranche/14-ui`, one commit per ticket, and check the line off in `scout-gap-audit.md`.

`qa` may add or extend test files anywhere under `apps/desktop/**` in service of verification; it
does not touch production code.

### 3.5 Stretch queue: DM Toolkit (only after the character-sheet punch list is empty)

Character-sheet UX is the entire mandate for this sprint (§0.1) and will very likely fill the whole
window on its own. DM Toolkit tickets are never dispatched while a character-sheet ticket is
available to claim — this is a strict priority order, not a parallel track.

If and only if `scout-gap-audit.md`'s character-sheet punch list is fully checked off with real time
still left before the Fable 5.1 window resets, the orchestrator may ask `scout` to spend a bounded
pass (~20 min) noting the gap between today's DM Toolkit and the vision at
`docs/release/v0.8/NoDA_Campaign_Console.html` (a single-file prototype console the operator built
and used live at the table — world/timeline/places/people/scenes/rules tabs, feeding an eventual
export-to-standalone-HTML capability a DM could open on any phone/tablet/laptop). Two things worth
knowing going in, so `scout` isn't rediscovering them from scratch:

- Today's "DM Toolkit" entry point in the desktop UI is `StubScreen.tsx` — a placeholder with no
  behavior behind it at all.
- The engine-side code that already exists under the "DM Toolkit" name (SD-22, `src/rules_core/encounters.rs`)
  is narrowly encounter-difficulty math (`Encounter`, `party_challenge_rating`) — it has no concept
  of people, places, or worldbuilding notes. The NoDA console's actual scope (entity authoring for
  world/people/places + HTML export) has no engine backing yet at all, which almost certainly makes
  it a `## Open blockers` engine-gap item (§4.2), not a ticket this team can cut and close solo.

Any DM Toolkit findings go in a clearly separated `## DM Toolkit (stretch, phase 2)` section of
`scout-gap-audit.md`, not mixed into the character-sheet punch list, and are not expected to reach
a shippable ticket this session — the realistic outcome is a short, well-scoped note for a future
brief, not committed code.

## 4. File ownership (avoid same-file collisions — no worktree isolation this sprint)

### 4.1 Within `apps/desktop`

| Owner | Path |
|---|---|
| `frontend` | `apps/desktop/src/**` (React/TypeScript UI). Primary target this sprint is `characterHub/` (character creation and level-up); the picker/catalog subtrees it draws from — `classCatalog/`, `equipmentCatalog/`, `spellCatalog/`, `monsterCatalog/`, `raceCatalog/`, `companionCatalog/` — are in scope only as needed to close a `characterHub/` gap, not for their own sake. `campaign/`, `settings/`, and DM-Toolkit-adjacent code are out of scope except per §3.5. |
| `backend` | `apps/desktop/src-tauri/src/**` (Tauri command bridge, Rust). Job is to give `frontend` something real to call against the engine as it exists today — wrapping, wiring, and shaping existing `codex` engine output for the UI. Not a lane for growing the engine itself. |
| `qa` | test files only, either side (`*.test.ts` under `src/`, `#[cfg(test)]` modules under `src-tauri/src/`) |
| `scout` | `docs/release/v0.8/scout-gap-audit.md` only; read-only everywhere else |
| orchestrator (lead) | `docs/release/v0.8/agent-team-ui.md` status updates, `git commit` |

If a ticket's shape genuinely needs both `frontend` and `backend` touching related code (a new
Tauri command plus the UI that calls it), sequence it as two tickets — backend command lands and is
committed first, then a second ticket hands the frontend teammate the now-real command to call.
Never let both teammates hold an open edit against the same file at once.

### 4.2 Repo-root `src/` (the engine) is out of scope for editing

The engine crate has multiple concurrent SD-3x corpus bundles landing against it outside this
session's awareness. `backend` may **read** it freely to understand a command's real return shape,
but does not edit it. If a gap audit finding requires a genuinely new engine capability (not just a
new Tauri command wrapping an existing one), that is a blocker: file it in
`scout-gap-audit.md` under `## Open blockers` with the specific engine surface needed, and escalate
to the operator rather than implementing it inline.

## 5. Retro logging

Any teammate that catches an error, hits an incident, defers work, or redoes something emits a
one-line event via `scripts/retro.py` per `AGENTS.md`'s Retrospective Logging section — this
applies to teammates exactly as it would to a solo session.

## 6. Stopping condition

Stop when either: the character-sheet punch list in `scout-gap-audit.md` (plus, only if reached,
the §3.5 DM Toolkit stretch queue) is fully checked off, or the Fable 5.1 window is close enough to
reset that a new ticket could not realistically finish and get QA-verified before it does. Either
way, the orchestrator leaves `tranche/14-ui` in a state where every commit on it individually
passes its own verification command — never a half-applied ticket
committed to make a deadline. A ticket that doesn't make it in time is left uncommitted (working
tree) or unclaimed (still on the punch list), not partially merged.

No PR, no merge to `develop`, no architecture-docs refresh at the end of this session — that's a
separate, explicit next step the operator triggers.
