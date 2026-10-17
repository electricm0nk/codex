---
canonical: true
owner: operator
bundle_id: SD-38
slug: cyberpunk-red
canonical_branch: tranche-18
target_version: 0.18.0
authored_from: docs/governance/workflow-instruction-template.md (2026-07-22), not from a prior bundle's instance
date: 2026-10-13
status: draft — authored under operator-away defaults; operator rulings on licence and scope are on record
---

# SD-38 Workflow Instruction — Cyberpunk RED

> ## OPERATING METHOD — REQUIRED FOR THIS BUNDLE
>
> **This bundle runs as a `Workflow`-tool script, invoked from a live session. It is NOT run with
> `/loop /batch` and it is NOT a one-shot task.** It is one long Workflow run with a recorded run id
> (`decisions.md §16.2`). §2.4 says how to write the script. `scope-draft.md` is the canonical
> *what*; this file is the *how*. The orchestrating session never executes RED→GREEN work itself
> (§2.2).
>
> > ### 🟡 UNATTENDED MODE — the operator is travelling for a few weeks
> >
> > Operator directive on record (2026-08-01, verbatim): *"include instructions to all 3 that
> > indicate they will be running in unnattended mode since i will be out of town while this
> > runs. They may not stop to ask questions - it might be days before i notice."* Operator mode
> > ruling for SD-38 (2026-10-13): autonomous; **report after each epic**; implementation lanes run
> > as Workflow scripts. This callout is one of three mirrors (`decisions.md §16`; `progress.md`
> > Cycle 0).
> >
> > 1. **Default and flag. Do not ask.** A choice inside scope with a safe default in
> >    `decisions.md §16.1` (or a ruling in `§20`/`§21`): take it, log it in `progress.md ## Decisions
> >    taken on safe defaults` with the alternative not taken, continue.
> > 2. **Never call `AskUserQuestion` or `clarify`.** Problems surface only in `progress.md`, the
> >    cycle receipts and the end-of-turn status.
> > 3. **Clear a blocker first.** If only an operator ruling can clear it, mark **only that card**
> >    `blocked-escalated`, write the exact ruling, write scope or precondition under
> >    `progress.md ## Open blockers` with the command and its output, and continue **every
> >    independent card**.
> > 4. **Never open the final PR while any card is open, partial or blocked-escalated.** A run that
> >    ends "every independent card done, blocked cards listed" has stopped correctly.
> > 5. **Report after each epic.** Each `*.MC` card (and E1.V) appends an "Epic report" block to
> >    `progress.md`: cards done, figures with commands, seed deltas, open blockers. Then the run
> >    continues — **dispatch first, report second**.
> > 6. **Quota stop** (`decisions.md §16.3`): an `agent()` result matching
> >    `/usage limit|rate limit|quota|limit reached/i`, or 10 M reported subagent tokens (estimate),
> >    or a shown weekly reading ≥ 85% → dispatch nothing new, write a resume receipt. **Never
> >    downgrade a model.** Resumption is not automatic.
> > 7. **Crash resume** (`decisions.md §16.4`): keep dirty worktrees; start a **new** run with
> >    `START=<first unrun card>`; never resume a script that has parallel lanes in flight.

## 0. Bundle at a glance

- **Branch:** `tranche-18`, cut by C1 from `origin/develop` after PR #398 merges (`decisions.md §2`).
- **Board:** local `./kanban.md` + `./progress.md`.
- **Cadence:** none; one live Workflow run.
- **Epics / cards:** C, E0–E7 (E6 runs in the E2 window) / **70 cards** (68 authored + E4.8, discovered by E4.MC + E7.3a, discovered by E7.3), one criterion each
  (`kanban.md` row check).
- **First concrete build value:** develop is at `0.17.0` (§1 item 7). This bundle's repo value is
  `0.18.0`, stamped by C1. The published triple `0.18.<run>` resolves at the first tester publish after
  the operator merges to `develop`.
- **Source:** `/home/ubuntu/workspace/inbox/cyberpunk-red/` (PDF, `cpr-corebook-v122.txt`, `pages/`,
  459 files, CUI F-19). Never copied into the git tree. **No PCGen oracle** governs any CPR figure
  (`decisions.md §23`).

## 1. Pre-launch checklist

Each command below was run on 2026-10-13 by the package author in `/home/ubuntu/workspace/repos/codex`
(branch `tranche-17-ui` @ `bb6107aee0`), output pasted. Items still open name the card that closes them.

1. **Board readable.** The package is in staging until the lead commits it (C0).
   `test -r docs/release/SD-38-cyberpunk-red/kanban.md && echo KANBAN_READABLE` → **open, closes at C0**.
2. **Bundle branch on origin.**
   ```
   $ git ls-remote --heads origin tranche-18
   (no output, exit 0)
   ```
   **Open → closes at C1.**
3. **Gate PR merged.**
   ```
   $ gh pr view 398 --json state,title
   {"state":"OPEN","title":"Tranche 17 UI"}
   $ git log origin/develop --oneline | head -3
   0b8a5757bf Merge pull request #397 from electricm0nk/tranche/17
   4a18b88eb0 docs(sd37,e7.9a): merge verified — 12/12 verify stages green, floors re-recorded, board 58 of 58
   bb0a26c0c6 merge(sd37,e7.9a): origin/develop fd68740f60 (PR #396) into tranche/17
   ```
   **Open → C1 waits (SD-k).**
4. **PAT.** Not applicable (no kanban CLI); `gh` auth works (`gh pr view` above).
5. **Working tree clean.**
   ```
   $ git status --porcelain | wc -l
   0
   ```
6. **Doctrine gates.**
   ```
   $ test -f docs/governance/no-stub-mvp-doctrine.md && test -f docs/doctrine-external/identifier-discipline.md && echo DOCTRINE_PRESENT
   DOCTRINE_PRESENT
   ```
7. **Build counter.**
   ```
   $ grep -m1 '"version"' apps/desktop/package.json; grep -m1 '"version"' apps/desktop/src-tauri/tauri.conf.json
     "version": "0.17.0",
     "version": "0.17.0",
   ```
   This bundle's first concrete value is `0.18.0` (C1).
8. **Artifact directories exist and are empty** (one per epic plus `cycle_0`; `cycle_0/inventory/`
   holds the seven inventories and their scripts by design).
   ```
   $ find artifacts -path '*/cycle_0/inventory' -prune -o -type f ! -name .gitkeep ! -name README.md -print | wc -l
   0
   ```
9. **Cargo load at authoring.** `pgrep -fc '[c]argo'` → `2` (other sessions). Re-check before every
   full sweep.

## 2. Orchestration mode

- **Dispatch:** the in-harness `Workflow` tool, one long run (`decisions.md §16.2`).
  `epic-breakdown.md §0` is the concurrency and tiering source of truth; no `workflow-dispatch.sh`.
- **Tiering** (`decisions.md §15`): Opus 5.5 (`'opus'`) for engine, UI, verify and every merge check;
  Sonnet 5.5 (`'sonnet'`) for transcription, well-specified work and every long wait; Haiku 4.5
  (`'haiku'`) for housekeeping. Merge checks are never downgraded.
- **Concurrency:** fixed in §3. **At most 3 cargo lanes.** Transcription lanes run no cargo.

### 2.1 Agent environment — the shared dispatch prefix

Every `agent()` prompt starts with this prefix, **byte-identical** across agents. Write it to
`artifacts/cycle_0/sd38-workflow.js` at launch so a crash resume can `diff` against it.

```text
ENVIRONMENT (SD-38, binding — every rule here applies to you):
- export PATH="$HOME/.cargo/bin:$PATH"
- export RETRO_ACTOR=sd38-<card-id>          # the ONLY value you bind, e.g. RETRO_ACTOR=sd38-e2.5
- ONE WRITER PER TREE. The serial chain works in /home/ubuntu/workspace/repos/codex/.worktrees/sd38-main
  (branch tranche-18). A parallel lane makes its OWN tree first, from the pushed branch:
      git -C /home/ubuntu/workspace/repos/codex fetch origin tranche-18
      git -C /home/ubuntu/workspace/repos/codex worktree add -b sd38/<lane> \
          /home/ubuntu/workspace/repos/codex/.worktrees/sd38-<lane> origin/tranche-18
  Start EVERY command with `cd <your tree> &&` (the shell's directory resets). Never write in
  /home/ubuntu/workspace/repos/codex itself (the lead's checkout).
- export CARGO_TARGET_DIR=/home/ubuntu/cargo-target/sd38-<lane>    # one per lane per source tree;
  never under /tmp; never shared across trees; the serial chain uses /home/ubuntu/cargo-target/sd38-main.
  Claim it: mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"
  Delete it when your lane's tree is removed (E7.7 deletes sd38-main).
- DISK FIRST: run `df -h /` before any build; a full sweep needs ~24 G free; if less, stop and report.
- AT MOST 3 CARGO LANES at once across the bundle (the script's semaphore holds this; do not start
  extra cargo processes). Before a full sweep: `pgrep -f '[c]argo'` and report what is running.
- MEMORY GUARD: one cargo process at a time in your lane; cargo build/test with -j 8; every
  `cargo test ... -- --test-threads=8`; `free -g` before a run expected > 10 min.
- NEVER `git stash`. NEVER `git add -A` (stage by explicit path). NEVER force-push.
  `git status --porcelain` before EVERY write; if it lists a file you did not modify, stop and report.
- TWO CARGO WORKSPACES: the repo root and apps/desktop/src-tauri are separate; a change that reaches
  both is verified in both. `cargo build --lib` green is not a verified change.
- LONG RUNS: anything that may run > 10 minutes: write a not-run skeleton results file FIRST (every row
  "not-run"), then `nohup <cmd> > <log> 2>&1 & echo $! > <pidfile>` and poll inside this turn.
  A partial results file must never read as complete.
- FRONTEND: in apps/desktop run `npm run typecheck && npm test && npm run ui-smoke:doc` for any
  frontend or spec.json change; `npm run ui-smoke` for a card that adds a ui-smoke row, always with
  `export RUN_DESKTOP_AGENT=sd38-<card-id>` first (run.mjs exits non-zero when it is unset or
  'default'; it must be unique per concurrent lane).
- AUDITS: the identifier audit and the four-check wired-integration audit (workflow-instruction.md §6
  step 4) on every diff, on added lines, after the local commit and before the push.
- TDD: write the failing test, run it, QUOTE the failing reason in the receipt, then make it pass.
- RECEIPTS: every figure carries the command that produced it (and a second implementation when it
  moves a baseline); counts with awk or Python, never `grep -o`.
- RETRO: emit scripts/retro.py events (correction/incident/deferral/rework) at the moment they happen,
  with RETRO_ACTOR set; a correction needs --verified-by.
- SOURCE: P=/home/ubuntu/workspace/inbox/cyberpunk-red/pages (pNNN = printed page + 1);
  PDF=/home/ubuntu/workspace/inbox/cyberpunk-red/cpr-corebook-v122.pdf. Page image:
  `pdftoppm -f N -l N -r 150 -png "$PDF" <scratch>/out`, then view the PNG. Never commit the PDF, page
  text or a PNG.
- GLOSSARY (package = docs/release/SD-38-cyberpunk-red/): "CUI F-n" = content-unit-inventory.md §2;
  "D-n" = decisions.md §20; "A-n" = decisions.md §21; "SD-x" = decisions.md §16.1; "R<n>" =
  workflow-instruction.md §12; "G-n" = acceptance-and-verification.md; "DEF-n" = decisions.md §22.
  In a table cell `\|` is Markdown for `|`: unescape before running.
- PAPER-SHEET RULE: compute only values that land on the sheet as numbers; print everything else; dice
  stay literal on the sheet. Every computed value is an explanations row with an id starting `cpr.`;
  no TSX re-derives a number.
- PF AND SF STAY BYTE-IDENTICAL. A PF render-hash or SF seed-line diff is a defect: fix it, never
  re-baseline (SD-e).
- UNATTENDED: no AskUserQuestion/clarify. If only an operator ruling can clear your blocker, return
  status blocked-escalated with the exact ruling needed.
- If your premise is refuted, return `declined` and name the mechanism. Do not commit a success.
- You get exactly ONE turn. Wait for slow work inside it. Commit and push before ending the turn,
  even for a partial result.
- First command of the cycle (wrong-base control):
  test -d docs && test -d data && test -d scripts && test -f docs/release/SD-38-cyberpunk-red/kanban.md \
    && [ "$(git rev-parse --show-toplevel)" != /home/ubuntu/workspace/repos/codex ] || { echo 'WRONG BASE'; exit 1; }
```

### 2.2 Execution boundary

The session that launches SD-38 is the orchestrator. §6 runs **inside** dispatched `agent()` calls,
never as the orchestrator's own `Edit`/`Write`/`Bash` on `apps/`, `src/`, `crates/`, `scripts/`,
`tests/` or `data/`. The orchestrator may make read-only investigation calls, edit this package's
planning docs, and run git plumbing for them. A cycle whose real scope differs from its card is
re-dispatched with the corrected scope. **Dispatch first, report second.** Never accept a lane's
message as evidence: check `git log` and the target files. Before every dispatch run
`git reflog --date=iso -3` and `git worktree list`.

### 2.3 Retrospective event logging

`scripts/retro.py correction --subject … --claimed … --actual … --verified-by …`; `incident`,
`deferral`, `rework` per `python3 scripts/retro.py help <type>`. Emitted when they happen.
`RETRO_ACTOR` is set by the prefix.

### 2.4 Creating the Workflow script

Load the `workflow-authoring` skill first. Plain JavaScript:

1. `export const meta = { name: 'sd38-cyberpunk-red', description, phases }` with **one phase per §3
   row, same titles** (below).
2. **The script reads `kanban.md`** at each step: it dispatches a card only when every `Depends on` ID
   is `complete`, and it picks up discovered cards from the board, never from a static list (R8).
3. `pipeline()` within a lane; `parallel()` only where §3 says `yes`. Parallel lanes make their own
   worktree per the prefix (`git worktree add .worktrees/sd38-<lane>`), not the harness's isolation
   (its base is the session checkout, which is not `tranche-18`).
4. A **3-slot cargo semaphore**: a card whose `Cargo?` column (`epic-breakdown.md §0`) is `yes` takes
   a slot for its whole run.
5. **Every `agent()` sets `model`** from the Tier column, written on the same line as `agent(`.
   Check: `awk '/agent\(/ && !/model:/' <script>` prints nothing. The Tier value must be exactly
   `opus`, `sonnet` or `haiku`; C0 (`lead session`) is never dispatched, and E7.9 (`sonnet + opus`) is two
   calls: the truth-up with `model: 'sonnet'`, then the claims critic with `model: 'opus'`. Check:
   the script refuses any card whose tier is not one of the three words, except E7.9's explicit pair.
6. Every prompt = §2.1 prefix + §6 verbatim + the card's `epic-breakdown.md` row (criterion, FILES YOU
   OWN, command, does not cover) + its `kanban.md` row.
7. **Quota gate** between steps (`decisions.md §16.3`).
8. **Refuted premise stops the step:** an `ok:false` or `declined` result never reads as success; the
   gate checks the implementer's status as well as verify-green.
9. Write the run id and script path into `progress.md` the moment the run starts.
10. **Never resume** a run with parallel lanes; start a new run with `START=<card>` (R9).

```javascript
export const meta = {
  name: 'sd38-cyberpunk-red',
  description: 'SD-38 Cyberpunk RED — seam, transcription, engine, create, sheet, improve, catalogs, closure',
  phases: [
    { title: 'C — package + cut' }, { title: 'E0 — seam' }, { title: 'E1 — transcription' },
    { title: 'E1.V — verify' }, { title: 'E2 — engine lanes' }, { title: 'E6 — catalogs' },
    { title: 'E2 — assembly' }, { title: 'E3 — create' }, { title: 'E4 — sheet' },
    { title: 'E5 — improve' }, { title: 'E7 — closure' },
  ],
}
// tier and cargo come from epic-breakdown.md §0; deps from kanban.md (R8). model on the agent( line.
const run = (card) => agent(PREFIX + PROCEDURE + criterion(card), { model: card.tier, phase: card.phase })
const slots = semaphore(3)                       // cargo lanes
const go = async (card) => { await depsComplete(card); quotaGate(); return card.cargo ? slots.with(() => run(card)) : run(card) }

phase('C — package + cut');   await go(C1)                                   // C0 is the lead's commit
phase('E0 — seam');           for (const c of E0) await go(c)                // serial, then E0.MC
phase('E1 — transcription');  await go(E1_0); await parallel(E1_LANES.map(c => () => go(c)))  // 8 lanes, no cargo
phase('E1.V — verify');       await go(E1V)
phase('E2 — engine lanes')
const laneA = (async () => { for (const c of [E2_1, E2_2, E2_3, E2_4]) await go(c) })()
const laneB = (async () => { for (const c of [E2_5, E2_6, E2_7, E2_8]) await go(c) })()
const laneC = (async () => { await go(E2_9); phase('E6 — catalogs'); for (const c of E6) await go(c) })()
const e3head = (async () => { await laneA; await go(E3_1) })()               // E3.1 after lane A and E6.1
await Promise.all([laneA, laneB, laneC, e3head])
phase('E2 — assembly');       await go(E2_10); await go(E2_MC)
phase('E3 — create')
const e3 = (async () => { for (const c of E3_REST) await go(c) })()          // E3.2 … E3.MC
phase('E4 — sheet')
const e4 = (async () => { for (const c of E4) await go(c) })()               // starts when E2.MC + E3.2 are complete
await Promise.all([e3, e4])
phase('E5 — improve');        for (const c of E5) await go(c)
phase('E7 — closure');        for (const c of E7) await go(c)                // E7.5 stops the run if short
```

### 2.5 A dispatched agent is never resumed

A dispatched `agent()` gets exactly one turn. It waits for slow work inside that turn (`nohup` + a
poll loop), scopes test runs to the binaries its change touches plus the suites §6 names, reports what
it observed if something will not finish, and commits and pushes anyway. The orchestrator verifies by
`git log` and the target files.

## 3. Per-epic parallel/sequential map

Phase titles above match these rows. Files marked *(new)* do not exist yet; the owning card creates
them and names the real path in its receipt. Shared-file writer order: `epic-breakdown.md §0`.

| Phase / epic | Cards | Parallel? | File-touch set | Gated on |
|---|---|---|---|---|
| C — package + cut | C0, C1 | no | this package; the 14 version surfaces + `docs/release/current-release.json` | PR #398 merged (C1) |
| E0 — seam | E0.1–E0.6, E0.MC | no | `src/rules_core/game_system.rs` and every `match`/`PerSystem` site (`decisions.md §8`); `scripts/verify.sh` (tauri-resources-tracked block); `data/cyberpunk-red/corpus/LICENSE.json` *(new)*; `docs/governance/license-matrix.md`; `apps/desktop/src-tauri/{tauri.conf.json,tests/packaged_resources.rs}`; `src/rules_core/cpr/{mod,input,sheet}.rs` *(new)*; `src/saved_character/mod.rs`; `apps/desktop/src-tauri/src/{cpr_adapter.rs (new),rule_system_adapter.rs,main.rs}`; `docs/governance/wired-integration-stubs-registry.md`; `LandingScreen.tsx`, `characterHubRuntime.ts`, `spec.json`, `docs/testing/ui-smoke-inventory.md` | C1 |
| E1 — transcription | E1.0, E1.1–E1.8 | lanes yes (after E1.0), each a disjoint set of `data/cyberpunk-red/corpus/<kind>.json` files (CUI §3); no cargo | `data/cyberpunk-red/corpus/**` *(new)*; `scripts/cpr_manifest_check.py` *(new, E1.0)*; CUI §2.8 (E1.8 only) | E0.MC |
| E1.V — verify | E1.V | no | corpus fixes; `MANIFEST.json`; `tests/cpr_corpus_manifest.rs` *(new)* | E1.1–E1.8 |
| E2 — engine lanes | lane A E2.1–E2.4; lane B E2.5–E2.8; lane C E2.9 | lanes yes, serial within a lane | A: `cpr/{stats,skills,armor,critical}.rs`; B: `cpr/{cyberware,roles,improvement,reputation}.rs`; C: `cpr/netrunning.rs`; each its own `mod.rs` anchor (A-10); fixtures under `tests/fixtures/cpr/` per card | E1 lane cards per `kanban.md` |
| E6 — catalogs | E6.1–E6.4, E6.MC | serial, in lane C's slot | `cpr_catalog.rs` *(new)*, `main.rs`, `apps/desktop/src/boundary/{loadCyberpunkCatalog,cprDeck}.ts` *(new)*, `apps/desktop/src/cyberpunkCatalog/**` *(new)*, `LandingScreen.tsx`, `CharacterHubPage.tsx`, `spec.json` | E1.V, E2.9 |
| E2 — assembly | E2.10, E2.MC | no | `cpr/sheet.rs`, `cpr_adapter.rs`, `tests/fixtures/cpr/sample_sheet_p121.json` *(new)* | all E2 lanes, E1.V |
| E3 — create | E3.1–E3.7, E3.MC | serial; E3.1 runs beside E6.2–E6.4 (disjoint files) | `cpr_creation.rs` *(new)*, `main.rs` (E3.1), `apps/desktop/src/boundary/cprCreation.ts` *(new)*, `CyberpunkCreateForm.tsx`, `cyberpunkCreationModel.ts`, `cyberpunkCreateLayout.ts` *(new)*, `characterHubRuntime.ts`, `CharacterHubPage.tsx` (E3.2), `spec.json` (E3.2, E3.MC) | E2.4 + E6.1 (E3.1); E6.MC (E3.2); E2.MC (E3.MC) |
| E4 — sheet | E4.1–E4.7, E4.MC | serial; beside E3.3–E3.MC | `CyberpunkCharacterSheet.tsx`, `cyberpunkSheetModel.ts`, `cyberpunkSheetLayout.ts` *(new)*, `CharacterHubPage.tsx` (E4.1), `cpr_adapter.rs` (E4.5, E4.6), `apps/desktop/src/boundary/cprSheet.ts` *(new, E4.5)*, `cpr_sheet_print.rs` *(new)*, `main.rs` (E4.5, E4.6), `spec.json` (E4.7) | E2.MC, E3.2; E3.MC (E4.7) |
| E5 — improve | E5.1–E5.5, E5.MC | no | `cpr_improve.rs` *(new)*, `main.rs`, `apps/desktop/src/boundary/cprImprove.ts` *(new)*, `CyberpunkImproveDialog.tsx` *(new)*, `CyberpunkCharacterSheet.tsx`, `CharacterHubPage.tsx`, `spec.json` + `docs/testing/ui-smoke-inventory.md` (E5.1, E5.MC) | E3.MC, E4.MC |
| E7 — closure | E7.1–E7.11 | no | `character_hub.rs` (starters), `packaged_resources.rs`, `scripts/verify-baselines.env`, `docs/retro/sd38-retrospective.md`, `docs/architecture/**`, graph outputs, this folder | E5.MC |

**Fences.** A lane that needs a file outside its row returns `declined` with `ownedBy:<card>`
**unless** that file's epic merge check is already `complete`: then it edits the file under its own
criterion and re-runs that epic's gate (`decisions.md §7` fence exit).

## 4. File-touch verification

Each existing path named in §3, `technical-design.md` and `content-unit-inventory.md` was checked on
2026-10-13 (`ls`/`grep -n`, line numbers quoted in `decisions.md` and `technical-design.md`). Paths
marked *(new)* are design choices the owning card confirms. Measured corrections folded in at
authoring: the brief's version grep misses `Cargo.lock` and catches `test_resolve_release_notes.py`
(`decisions.md §2`); `sheet_rules_relative` returns `&'static str` today and the
`tauri-resources-tracked` stage parses its arms (`decisions.md §8`); `StarterSeed` holds a Pathfinder
`CreateCharacterRequest` (`character_hub.rs:2103`).

## 5. Concurrent-write protocol

```bash
git fetch origin tranche-18 && git rebase origin/tranche-18 && git push origin HEAD:tranche-18
```

On a non-fast-forward rejection, retry up to 5 times; then `CLAIM-EXISTS`, card `blocked-escalated`.
Never force-push. Re-fetch and re-read `progress.md`/`kanban.md` immediately before every edit. A
rebase **conflict**: SD-l (`kanban.md`: take upstream and re-apply only your own row, then the
duplicate-ID check prints nothing — **never** keep both sides, which duplicates rows; `progress.md`
append-only sections: keep both sides) and SD-p (keep both sides for the `cpr/mod.rs` lane anchors); any other conflict aborts the rebase and the card is
`blocked-escalated` with the paths. Every card, lanes included, lands by pushing its own commits to
`tranche-18`; merge-check cards re-verify on `origin/tranche-18` and merge nothing.

## 6. Per-cycle procedure (runs inside the dispatched agent)

**Environment rules, verbatim from the brief (binding in every step):** one writer per tree
(`git worktree add .worktrees/<lane>`), own `CARGO_TARGET_DIR=/home/ubuntu/cargo-target/<lane>`
(delete at end; never under /tmp; check `df -h /` first; ≤3 cargo lanes at once), never `git stash`,
never `git add -A`, `pgrep -f '[c]argo'` before a full sweep, root and `apps/desktop/src-tauri` are
separate cargo workspaces (both must pass), `--test-threads=8`, nohup for runs > 10 min with a not-run
skeleton, `npm run typecheck && npm test && npm run ui-smoke:doc` in `apps/desktop`, the four-check
no-stub audit + the identifier audit on every diff, TDD red→green with the failing reason quoted,
receipts with every figure's command, retro events via `scripts/retro.py` with `RETRO_ACTOR`.

1. **Base check** (a control): run the prefix's wrong-base test. On failure,
   `git reset --hard <pinned tranche-18 SHA from progress.md>` and re-check. Then §5's fetch+rebase;
   record `CARD_BASE=$(git rev-parse HEAD)`.
2. **Disk and load:** `df -h /`; `pgrep -f '[c]argo'`; claim `CARGO_TARGET_DIR`.
3. **Baseline:** run the card's acceptance command on the pre-change tree and keep its output (RED).
4. **TDD:** write the failing test; run it; quote the failing reason; make the smallest change; run the
   targeted suites.
5. **Commit locally** (explicit paths), then run the **dual audit on added lines** (every check must
   print its `OK_*`; a violation is fixed by a new commit and the audit re-run):
   ```bash
   # Identifier audit (no trailing \b — it would miss `sd19_` style identifiers)
   git diff --unified=0 "${CARD_BASE}...HEAD" -- src crates apps scripts tests ':!**/__tests__/**' ':!**/*.test.*' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
   # Four-check wired-integration audit — docs/governance/no-stub-mvp-doctrine.md §"Per-cycle audit"
   # 1. tokens (crates/** added: SD-38 ships engine code under src/ and may touch crates/ match arms)
   git diff --unified=0 "${CARD_BASE}...HEAD" -- 'apps/desktop/**/*.ts*' 'apps/desktop/src-tauri/**/*.rs' 'src/**/*.rs' 'crates/**/*.rs' ':!**/__tests__/**' ':!**/*.test.ts' ':!**/*.test.rs' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo OK_NO_TOKENS
   # 2. no-op handlers
   git diff --unified=0 "${CARD_BASE}...HEAD" -- 'apps/desktop/**/*.tsx' 'apps/desktop/**/*.jsx' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE 'onClick=\{\s*\(\)\s*=>\s*\{\s*\}\s*\}|onClick=\{undefined' || echo OK_NO_NOOP_HANDLERS
   # 3. mock leaks
   git diff --unified=0 "${CARD_BASE}...HEAD" -- 'apps/desktop/**/*.ts' 'apps/desktop/**/*.tsx' 'apps/desktop/**/*.jsx' 'apps/desktop/**/*.rs' ':!**/__tests__/**' ':!**/*.test.*' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE 'mockResolvedValue|mockReturnValue\(|vi\.mock\(|__mocks__' || echo OK_NO_MOCK_LEAKS
   # 4. "Would …" strings
   git diff --unified=0 "${CARD_BASE}...HEAD" -- 'apps/desktop/**/*.ts' 'apps/desktop/**/*.tsx' 'src/**/*.rs' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE '"Would [^"]*"' || echo OK_NO_WOULD_STRINGS
   ```
   The doctrine's own four commands (verbatim, `develop...HEAD` form) are in
   `docs/governance/no-stub-mvp-doctrine.md` §"Per-cycle audit"; the form above differs only in its
   base (`CARD_BASE`), its added-lines filter and `crates/**` in check 1.
6. **Clippy and the audit test** in every card whose diff touches a `.rs` file (R11; a TS/TSX-only
   card such as E4.2–E4.4 or E6.3 runs no cargo, matching its `Cargo?` value): `bash scripts/verify.sh --only clippy -j 8` (the
   stage runs `cargo clippy --locked --tests` over the three crates, `scripts/verify.sh:2166`) and root
   `cargo test --locked -j 8 --test sd24_wired_integration_audit -- --test-threads=8`.
7. **PF/SF deltas** (R7): any card that touches `src/`, `crates/`, `apps/desktop/src-tauri/` or a shared
   frontend file renders the PF pair and the SF seed lines (`acceptance-and-verification.md` G-1) and
   records each seed's hash/lines unchanged, or the card is not done.
8. **Verify once at the widest scope the change reaches:** root `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8`
   and/or `cd apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8`,
   plus `cd apps/desktop && npm run typecheck && npm test && npm run ui-smoke:doc` when frontend or
   `spec.json` changed. Attribute every `test result: FAILED` to its `Running` line. Do not repeat a
   clean gate (R-V).
9. **Receipt** to `artifacts/epic_<n>/<card>_cycle_receipt.md` (§7). Update the card's `kanban.md`
   row and append a `progress.md` row **in the same commit**. Unfiltered `git status --porcelain`
   prints nothing after it. Push via §5.
10. **Report:** status (`complete | partial | blocked-escalated`), SHA, receipt path, seed deltas,
    discoveries.

## 7. Per-cycle receipt schema

```markdown
# Cycle <card-id> — <epic> / <criterion title>

- **Card ID:** <card-id>   **Model:** <opus|sonnet|haiku>   **RETRO_ACTOR:** <value>
- **Commit SHA:** <sha>    **Base SHA:** <tranche-18 sha at start>
- **Files touched:** <list>   (every one inside the card's FILES YOU OWN, or a named fence exit)
- **Acceptance criterion (verbatim from epic-breakdown.md):** <text>
- **RED (pre-change command + output, failing reason quoted):** <…>
- **GREEN (same command + output):** <…>
- **Figures:** <value> — <predicate/denominator> — <command A> → <output>; <command B> → <output>
- **Source pages read / image checks:** <pNNN list; crop boxes>
- **Build scope verified:** <root | apps/desktop/src-tauri | frontend> + log path in the package
- **Identifier audit:** OK_NO_BUNDLE_TAGS | <violations>
- **Wired-integration audit:** OK_NO_TOKENS / OK_NO_NOOP_HANDLERS / OK_NO_MOCK_LEAKS / OK_NO_WOULD_STRINGS | <violations>
- **Clippy + sd24 audit test:** <result>
- **Seed deltas:** Aldric / Elowen sha256; SF-Soldier-3 / SF-Mystic-5 / SF-Technomancer-5 / SF-Envoy-3 lines; CPR seeds (after E7.1): <unchanged | old → new>
- **Rulings applied:** <D-n / A-n / SD-x + alternative not taken>
- **Does not cover:** <the shapes this proof does not exercise>
- **Status:** complete | partial | blocked-escalated
- **Retro events emitted:** <types + counts>
- **Next-cycle plan:** <…>
```

## 8. Self-heal posture

- **Self-healable:** dirty tree from your own lane; a single-token audit violation; unrelated
  test-setup breakage; a count that two implementations re-derive (SD-c); build-counter drift.
- **Not self-healable (`blocked-escalated` on THIS card; continue independent cards):** a gate PR not
  merged (SD-k); two live lanes on one file; RED→GREEN not preserved; `success: true` from a fake
  operation, an inline mock or a "Would …" string in shipping code; a rule the book does not state
  that blocks a DoD item (SD-d); `CLAIM-EXISTS`; a fresh-install check the box cannot run (SD-m).
- **Blocker doctrine** (`docs/governance/blocker-closure-doctrine.md`): a blocker on the Definition of
  Done is cleared or escalated, never deferred. "Filed with a named owner", "forwarded", "deferred with
  reason" and "out of scope for this cycle" are not dispositions. The test: was this scope in the
  Definition of Done at scoping? If yes, it is a blocker.
- **Disk:** after every parallel wave, `df -h /` and `git worktree list`; prune merged, unlocked, clean
  worktrees and delete their `CARGO_TARGET_DIR`s. Never remove a `locked` tree.

## 9. Placeholder-resolution checklist

```bash
grep -rn '<[a-z_-]*>' docs/release/SD-38-cyberpunk-red/*.md
```

Every match must be (a) a schema placeholder inside a fenced template (every `<…>` in the §2.1 prefix and
the §7 receipt schema, e.g. `<epic>`, `<list>`, `<text>`, `<output>`, `<violations>`, `<result>`), a command
argument the running card binds (`<sha>`, `<bump-sha>`, `<dir>`, `<base>`, `<head>`, `<lane>`, `<branch>`,
`<card>`, `<card-id>`, `<n>`, `<k>`, `<type>`, `<cmd>`, `<log>`, `<pidfile>`, `<scratch>`, `<script>`,
`<epic-start>`, `<value>`, `<name>` with `<STAT>` in CUI F-38's per-skill loop), or a name pattern
(`<kind>` in `data/cyberpunk-red/corpus/<kind>.json`; `cpr.<area>.<field>` explanation ids), or a naming pattern
(`SD-NN-<slug>`, `stc-<##>-<description>`); (b) a deferred value with a named resolution point
(`0.18.<run>` → first tester publish after the operator merges; Mooks/Encounters/Beat Charts counts →
E1.8 then E1.V; the run id → launch); or (c) a bug to fix before launch. The author's run of this gate
is recorded in `progress.md` Cycle 0.

## 10. Epic wrap-up (after every epic)

1. `scripts/retro.py summary --since <epic-start> --json`; read it; fold the counts and any key that
   fired more than once into the merge-check receipt.
2. The merge-check card writes the **Epic report** block in `progress.md` (operator mode ruling) and
   updates `kanban.md` in its own closing commit.
3. Sweep this epic's merged, unlocked, clean worktrees (`git worktree list`; `git log <branch>..origin/<branch>`
   empty) and delete their `CARGO_TARGET_DIR`s.
4. No PR.

## 11. Bundle closure epilogue (corrected order — `decisions.md §17`)

This order differs from the template's §11 (release notes after graphify), a known template defect.

1. **E7.5 final-acceptance scan:**
   ```bash
   cd docs/release/SD-38-cyberpunk-red
   test -s kanban.md || { echo NO_KANBAN; exit 2; }
   awk -F'|' '$2 ~ /^ (C|E)[0-9]/ { n++ } END { if (n != 70) print "ROW_COUNT " n }' kanban.md
   awk -F'|' '$2 ~ /^ (C|E)[0-9]/ && $2 !~ /^ E7\.([5-9]|1[01]) / && $5 !~ /^ complete *$/ { print $2 "|" $5 }' kanban.md
   ```
   Pass = no output. Every DEF revisit condition (`decisions.md §22`) is checked and its output pasted.
   **If anything is short, stop here:** no retrospective, no sweep, **no PR**. Report what is short
   with the command that shows it. That is a correct outcome.
2. **Seeds and packaged resources are already done** (E7.1, E7.2, E7.3) and the widest verify ran
   once on the final code (E7.4).
3. **E7.6 retrospective:** `scripts/retro.py summary --since 2026-10-13 --json`; write
   `docs/retro/sd38-retrospective.md` in the shape of `docs/retro/sd31-retrospective.md`; **cite it
   from `references/README.md` in the same commit**.
4. **E7.7 sweep:** every SD-38 worktree and local/remote `sd38/*` branch; found vs removed counts;
   never `test` or `update-index`, a locked tree, or another session's tree; delete
   `/home/ubuntu/cargo-target/sd38-*`.
5. **E7.8 release notes** (`release-notes.md`): replace the whole interim body, including the
   `SD-38 INTERIM NOTES` sentinel (`grep -c` → 0); every figure with its command;
   `python3 tools/release/resolve_release_notes.py --repo-root .` passes.
6. **E7.9 architecture truth-up**
   (`~/.hermes/profiles/god-emporer/skills/devops/architecture-truth-up/scripts/architecture_truth_up.py --integration-target develop --receipts-md docs/release/SD-38-cyberpunk-red/receipts.md --bundle SD-38`),
   then an Opus claims critic (capability claims, paths, numbers).
7. **E7.10 graphify LAST.** Precondition: unfiltered `git status --porcelain` prints nothing and
   `git rev-parse HEAD` = `git rev-parse origin/tranche-18`. Run
   `~/.hermes/profiles/god-emporer/skills/devops/graphify-update/scripts/update_graphify.py --integration-target develop --receipts-md docs/release/SD-38-cyberpunk-red/receipts.md --bundle SD-38`.
   Record the indexed SHA. On the node-count guard (exit 1): file the receipt and stop. **Never** a
   force flag. Commit only the graph outputs and `receipts.md`; push; confirm the tree is clean.
8. **E7.11 PR** `tranche-18 → develop`, the final action: re-run step 1 with the exemption narrowed
   to `E7\.11`, and `grep -c 'SD-38 INTERIM NOTES' docs/release/SD-38-cyberpunk-red/release-notes.md`
   → 0 (anything else: stop, no PR); commit E7.11's own `complete` row (docs only) and push; `gh pr create`; wait inside the
   turn for `gh pr checks <n> --watch`. Red or unopenable → one follow-up docs commit setting E7.11
   `blocked-escalated` with the failing job named. **The operator merges.**
9. Stop.

## 12. Standing rules

### 12.1 SD-36 lessons (R1–R7, carried from SD-37 where they still apply)

| Rule | Statement | Checked by |
|---|---|---|
| R1 *(retired for SD-38)* | Converter lane in every prefix | Not applicable: SD-38 uses no PCGen converter (`decisions.md §23`) |
| **R2 No hand-kept desktop tables** | The engine is the single source; every CPR sheet number traces to a `cpr.` row | E4.1's row-mapping test; E4.MC |
| **R3 Merge checks stay on Opus** | Every `*.MC` and E1.V is `opus` | `awk -F'\|' '$2 ~ /MC\|E1\.V/ && $4 !~ /opus/' kanban.md` prints nothing (unescape `\|`) |
| **R4 `ownedBy` declines** | A declined item naming a later card goes to that card | script review at launch |
| **R5 Isolated app-data root** | Any app-launching harness uses a per-run `XDG_DATA_HOME`, refuses the real root, records the real store's entry count + sha256 before and after | E3.MC, E4.7, E5.MC, E7.1 receipts |
| **R6 Long waits on Sonnet+** | No Haiku on runs that may exceed 20 min; `green:false` + empty failing list = not finished | Tier column of E7.3, E7.4, E7.10, E7.11 ≠ haiku |
| **R7 Per-character status** | Every receipt reports PF/SF (and later CPR) seed deltas | `grep -L 'Seed deltas' artifacts/epic_*/*_cycle_receipt.md` prints nothing |

### 12.2 SD-37 retrospective changes (R8–R13, `docs/retro/sd37-retrospective.md:273`)

| Rule | Statement | Checked by |
|---|---|---|
| **R8 The script reads the board** | A card is dispatched only when every `Depends on` is `complete`; discovered cards come from `kanban.md` | script review; `kanban.md` dependency check (its own fenced command) |
| **R9 Never resume a parallel script** | Start a new run with `START=<card>` | `progress.md` run handle lists each run id with its `START` |
| **R10 Fences have an exit** | A completed epic's files open to later serial cards, gate re-run (`decisions.md §7`) | receipts name the file, owning epic and re-run gate |
| **R11 Clippy + audit test every code card** | Not only merge checks (9 of 19 SD-37 verification failures were clippy) | §6 step 6 line in each code receipt |
| **R12 Generated artifacts have a re-run-and-diff check** | `docs/testing/ui-smoke-inventory.md` is regenerated and committed by every card that edits `spec.json`; `MANIFEST.json` is checked by two implementations | E4.7: `npm run ui-smoke:doc && git status --porcelain docs/testing/ui-smoke-inventory.md` empty |
| **R13 Re-derive copied figures** | A figure from an earlier doc is re-derived before use (SD-37's "8 open P1s" was 0) | receipts give the command for every figure |

### 12.3 Per-cycle and measurement rules

- **R-T TDD**, failing reason quoted. **R-B Batch big:** a homogeneous remainder goes in one dispatch
  (each E1 lane takes all its kinds). **R-V One verify per wave.** **R-C Clean tree** = unfiltered
  `git status --porcelain` empty at every commit. **R-N** every figure with its command and
  denominator; awk or Python, never `grep -o`; two implementations for a baseline. **R-P** every proof
  states what it does not cover. **R-W** an incident key that fires more than twice gets a non-zero-exit
  check in the next card that touches that area. **R-S Sum the piles:** the MANIFEST sum check fails
  closed. **R-G Widest build scope:** both cargo workspaces. **R-X Cross-tree renders:** the sheet-rule
  path is baked at compile time, so the base-tree render builds in its own worktree with its own
  `CARGO_TARGET_DIR`. **R-Q Quota:** `decisions.md §16.3`.

## Cross-references

- `docs/governance/workflow-instruction-template.md` (source; its §11 order is defective),
  `docs/release/template/template.md`.
- `docs/governance/blocker-closure-doctrine.md`, `docs/governance/deferral-revisit-doctrine.md`,
  `docs/governance/no-stub-mvp-doctrine.md`, `docs/governance/wired-integration-stubs-registry.md`.
- `docs/retro/sd37-retrospective.md` (R8–R13), `docs/retro/sd36-retrospective.md` (R1–R7).
