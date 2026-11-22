---
canonical: true
owner: operator
bundle_id: SD-43
slug: market-inventory-and-abilities
canonical_branch: tranche-43
target_version: 0.43.0
authored_from: docs/governance/workflow-instruction-template.md (procedure not forked); SD-42's rendering as the shape reference
date: 2026-11-22
status: planning — back half; §1's commands were run on 2026-11-22 and their output is pasted
---

# SD-43 Workflow Instruction — Market, Inventory and the Abilities Tab

> ## OPERATING METHOD — REQUIRED FOR THIS BUNDLE
>
> **This bundle runs as a `Workflow`-tool script, invoked from a live session. It is NOT run with `/loop /batch`
> and it is NOT a one-shot task.** §2.4 says how to write the script. `scope-draft.md` is the *what*; this file
> is the *how*. The orchestrating session never executes RED→GREEN work itself (§2.2).
>
> **Unattended after launch.** The run stops for the operator at exactly one planned wait, **E4.G** (approval of
> the proposed item names, `decisions.md §22`). Every other open question takes its default from
> `decisions.md §30` and logs it. One **precondition** only the operator can clear: the CI secret
> `CODEX_REPORT_KEY` (P-8, Q-13.2) — created 2026-11-22 (`decisions.md §32`); closure (E7.2) still re-checks it is present; before it existed, closure (E7.2) could not pass without it
> (`risks-and-open-questions.md` R-1). Q-22.3 is settled by the operator (2026-11-22): the Feats tab is renamed
> **Abilities** and Feats becomes one of its seven sub-tabs (E3.1).

## 0. Bundle at a glance

- **Branch:** `tranche-43`, cut from `origin/develop` @ `fb94a85049`; not on origin until C1 pushes it.
- **Board:** local file `./kanban.md` (56 rows; `awk -F'|' '$2 ~ /^ (C|E)[0-9]/' kanban.md | awk 'END{print NR}'`).
- **Cadence:** N/A — dispatch is a live `Workflow` session, not a timer loop.
- **Epics / cards:** 9 (C, E0–E7) / 56, of which C0 is the lead's and E4.G is the operator's (`epic-breakdown.md §0`).
- **First concrete build value:** §1 item 7.

## 1. Pre-launch checklist (run 2026-11-22 by the package author; C1 re-runs items 2, 5 and 7 and pastes them in its receipt)

1. **Board readable.** The Hermes board is retired (template §1 note); the board is `./kanban.md`.
   `test -s docs/release/SD-43-market-inventory-and-abilities/kanban.md && echo KANBAN_OK` → `KANBAN_OK` once this
   back half is committed.
2. **Branch on origin.** `git ls-remote --heads origin tranche-43 | awk 'END{print NR}'` → `0` on 2026-11-22. C1
   pushes it; until then no lane can be cut (§2.1 prefix creates lanes from `origin/tranche-43`).
3. **Predecessor merged.** SD-42 merged as PR #413; `git merge-base --is-ancestor fb94a85049 HEAD && echo P1_ok` →
   `P1_ok`. `git log origin/develop --oneline -1` → `fb94a85049 Merge pull request #424 from electricm0nk/chore/heal-main-into-develop`.
4. **PAT.** Not used (no Hermes board). `gh auth status` must succeed for C1's push and E7.4's PR.
5. **Tree clean.** `git status --porcelain | awk 'END{print NR}'` → `0` before this back half's first write.
6. **Doctrine gates.** `test -f docs/governance/no-stub-mvp-doctrine.md && test -f docs/governance/blocker-closure-doctrine.md && echo ok`
   → `ok`. The `~/.hermes/` skill-path check is moot (template §1 note); §6's greps are the gate.
7. **Build counter.** Repo files read `0.43.0` (`node -p "require('./apps/desktop/package.json').version"`;
   `python3 -c "import json;print(json.load(open('apps/desktop/src-tauri/tauri.conf.json'))['version'])"` →
   `0.43.0`). The publish stamps `0.43.${GITHUB_RUN_NUMBER}` (`.github/workflows/publish-tester-release.yml:129`).
   The newest publish run is **168** (`gh run list -w publish-tester-release.yml -L 1 --json number` →
   `168`, on `main`; develop's newest alpha is `alpha v0.22.166`, `gh release list -L 3`). SD-43's first concrete
   build is therefore **`0.43.<run>` with `<run>` ≥ 169**, the first publish after E7.4 merges; the exact value is
   recorded in E7.4's receipt.
8. **Artifact directories.** `artifacts/cycle_0/` … `artifacts/epic_7/` are listed in `artifacts/README.md`; each
   card creates its epic directory on first write (git does not keep empty directories).
9. **Disk.** `df -h /` → `1.5T size, 101G used, 1.4T avail` on 2026-11-22; a full sweep needs ~24 G.
10. **Worktrees.** `git worktree list` → one line (the main checkout) on 2026-11-22.

## 2. Orchestration mode

- **Dispatch:** the in-harness `Workflow` tool, invoked from a live session. `epic-breakdown.md §0` is the source
  of truth for lane, dependency, tier, cargo and `hub`; `kanban.md` carries the same values per row.
- **Tiering** (`~/.claude/CLAUDE.md`, 2026-09-29): **Opus 5.5 (`opus`)** is the default for implementation,
  debugging and every merge check; **Sonnet 5.5 (`sonnet`)** for the well-specified mechanical cards the Tier
  column names; **Haiku 4.5 (`haiku`)** for housekeeping (C1 only). The model is **never omitted** (§2.4 check).
  Every `*.MC` is `opus` and is never downgraded. No Haiku on a card that may wait more than 20 minutes (memory:
  Haiku cannot hold a long-run return contract), so E7.4, which watches the publish, is `sonnet`.
- **Concurrency:** seven lanes (`main`, `census`, `market`, `gear`, `sheet`, `settings`, `campaign`), each serial
  inside, each in its own worktree with its own `CARGO_TARGET_DIR`. Two script-held controls, not warnings:
  - **cargo semaphore = 3.** A card whose `Cargo?` column is not `no` acquires one of three slots before dispatch
    (memory: the Proxmox host stops the VM on guest OOM). The agent also waits on
    `pgrep -x cargo | awk 'END{print NR}'` < 3 inside its turn, because another session may build on the box.
  - **`hub` token = 1.** A card marked `hub` acquires the single token for `character_hub.rs`, `main.rs`,
    `CharacterSheet.tsx` before dispatch and releases it when its commit is pushed.
- **Unique `RUN_DESKTOP_AGENT` per dispatch** (SD-41 lesson): `sd43-<card-id>-<run id>`.

### 2.1 Agent environment — the shared dispatch prefix

Every `agent()` prompt starts with this block, byte-identical across agents except the bound values `<card-id>`,
`<lane>`, `<run id>`. The lead writes the script with this block to `artifacts/cycle_0/sd43-workflow.js` at launch
(SD-42 retrospective change 1).

```text
ENVIRONMENT (SD-43, binding — every rule here applies to you):
- export PATH="$HOME/.cargo/bin:$PATH"
- export RETRO_ACTOR=sd43-<card-id>                     # e.g. RETRO_ACTOR=sd43-e3.4
- export RUN_DESKTOP_AGENT=sd43-<card-id>-<run id>      # unique per dispatch
- PACKAGE=docs/release/SD-43-market-inventory-and-abilities
- ONE WRITER PER TREE. Your lane's tree is /home/ubuntu/workspace/repos/codex/.worktrees/sd43-<lane>
  (lanes: main, census, market, gear, sheet, settings, campaign). If it does not exist, make it from the PUSHED
  branch (never from the session checkout):
      git -C /home/ubuntu/workspace/repos/codex fetch origin tranche-43
      git -C /home/ubuntu/workspace/repos/codex worktree add -b sd43/<lane> \
          /home/ubuntu/workspace/repos/codex/.worktrees/sd43-<lane> origin/tranche-43
  Start EVERY command with `cd <your tree> &&` (the shell's directory resets). Never write in
  /home/ubuntu/workspace/repos/codex itself (the lead's checkout) or in another lane's tree.
- export CARGO_TARGET_DIR=/home/ubuntu/cargo-target/sd43-<lane>   # one per lane per source tree; NEVER under /tmp;
  never shared across trees. Claim it: mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"
- DISK FIRST: `df -h /` before any build; a full sweep needs ~24 G free; if less, stop and report.
- AT MOST 3 CARGO PROCESSES on the box: before any cargo command,
  `pgrep -x cargo | awk 'END{print NR}'` must print < 3 (process NAME, never `pgrep -f`/`pkill -f` on a pattern
  your own shell line holds); else poll every 60 s inside this turn, up to 40 min. One cargo process at a time in
  your lane; `-j 8`; `-- --test-threads=8`.
- NEVER `git stash`. NEVER `git add -A` (stage explicit paths). NEVER force-push.
  `git status --porcelain` before EVERY write; if it lists a file you did not modify, stop and report.
- TWO CARGO WORKSPACES: the repo root (src/, crates/) and apps/desktop/src-tauri are separate. The desktop crate
  is a binary crate (tests are #[cfg(test)] modules or tests/*.rs, run from apps/desktop/src-tauri). A test
  filter that matches 0 tests is a FAIL; quote the `N passed` line.
- THE HUB FILES: apps/desktop/src-tauri/src/character_hub.rs, apps/desktop/src-tauri/src/main.rs,
  apps/desktop/src/characterHub/CharacterSheet.tsx. You may write one only if your kanban row says `hub`
  (the script holds the token for you). ui-smoke spec.json is written by E7.1 only.
- FILES YOU OWN IS CLOSED: before your first write, `git grep` every producer your card names and list every
  second call site, command registration, DTO producer and boundary file; a file outside the list is a fence
  exit you name in the receipt and log as `scripts/retro.py incident`. A new runtime data read adds a
  `bundle.resources` entry in apps/desktop/src-tauri/tauri.conf.json AND an assertion in
  apps/desktop/src-tauri/tests/packaged_resources.rs, read through `repo_root()`, never `env!("CARGO_MANIFEST_DIR")`.
- DEPENDENCIES: read $PACKAGE/kanban.md; if any `Depends on` of your row is not `complete`, return `declined`
  naming it. Do no work.
- TDD: run the card's acceptance command on the pre-change tree first; it must fail (or exit non-zero) for the
  stated reason and match the lead's RED in this prompt. Write the failing test, run it, QUOTE the failing line in
  the receipt, make the smallest change, run the targeted suites. If the RED does not reproduce, return
  `declined` with the mechanism — never commit a success on a test that could not fail.
- CLIPPY before the first commit of any .rs change: `cargo clippy --locked --tests -j 8` in each touched workspace.
- PCGEN RESIDUE: any card that changes a desktop #[cfg(test)] module or a .rs file runs
  `bash scripts/verify.sh --only pcgen-residue-gate -j 8` → PASS (SD-42 retrospective change 4).
- THE ORACLE: resolve it only through `source <(bash scripts/fetch-pcgen-oracle.sh --print-env)` →
  `$PCGEN_REPO_DIR` / `$PCGEN_CORPUS_ROOT`; `git -C "$PCGEN_REPO_DIR" rev-parse HEAD` must equal PCGEN_ORACLE_SHA in
  scripts/pcgen-oracle-pin.env; never write a literal checkout path; nothing from the oracle enters live code.
- THE ENGINE OWNS THE NUMBERS: no TS sums a weight, price, grant or prerequisite; the UI renders engine rows
  (NR-1). Paper-sheet rule: compute only values that land on the sheet as numbers; print every rule as text.
- NO STUBS: a tab, button, menu or sub-tab that ships does what it says. A sub-tab whose engine card has not
  landed shows a notice naming that card; at E3.MC none may remain. A data gap is a sentence naming what the
  corpus lacks, equal to E0.1's list — never a hidden or faked picker.
- SECRETS: the report key never enters the repo, a log, a receipt or JS. Never print it.
- AUDITS: the identifier audit and the four-check wired-integration audit (§6 step 5) on added lines of your
  diff, after the local commit and before the push.
- LONG RUNS: anything that may take > 10 min: write a not-run skeleton results file first, then
  `nohup <cmd> > <log> 2>&1 & echo $! > <pidfile>` and poll inside this turn.
- FRONTEND: in apps/desktop run `npm run typecheck && npm test` for any frontend change; in a fresh tree run
  `node scripts/gen-corpus-bundle.mjs` from the repo root once first. No new npm package unless your card names it;
  then quote `npm view <pkg> version license` in the receipt.
- APP LAUNCHES: the run-desktop harness with an isolated XDG_DATA_HOME under your lane tree's scratch dir and
  your RUN_DESKTOP_AGENT; never the operator's data.
- RECEIPTS: every figure carries the command that produced it and its denominator; count with awk or Python,
  never `grep -o`; two implementations when a figure moves a baseline. State what your proof does NOT cover.
- RETRO: emit `scripts/retro.py correction|incident|deferral|rework` the moment it happens, with RETRO_ACTOR set;
  a correction needs --verified-by.
- AUTONOMOUS: no AskUserQuestion. A taste question takes decisions.md §30's default and is logged. If only an
  operator ruling can clear your blocker, return `blocked-escalated` naming the exact ruling, write scope or
  precondition needed. Never defer DoD scope to SD-44.
- You get exactly ONE turn. Wait for slow work inside it. Commit and push before ending the turn, even for a
  partial result.
- FIRST COMMAND (wrong-base control):
  cd /home/ubuntu/workspace/repos/codex/.worktrees/sd43-<lane> && test -d docs && test -d data && test -d scripts \
    && test -f docs/release/SD-43-market-inventory-and-abilities/kanban.md \
    && [ "$(git rev-parse --show-toplevel)" != /home/ubuntu/workspace/repos/codex ] \
    && git merge-base --is-ancestor fb94a85049 HEAD \
    || { echo 'WRONG BASE'; exit 1; }
```

### 2.2 Execution boundary

The session that launches SD-43 is the orchestrator. §6 runs **inside** dispatched `agent()` calls, never as the
orchestrator's own `Edit`/`Write`/`Bash` on `apps/`, `src/`, `crates/`, `scripts/`, `tests/`, `data/`,
`resources/` or `.github/`. The orchestrator may make read-only investigation calls, edit this package's planning
docs, run git plumbing for them, and record operator replies (E4.G, any §30 overrule) in `progress.md`. **Dispatch first,
report second.** Never accept a lane's message as evidence: check `git log origin/tranche-43` and the target
files. Before every dispatch: `git reflog --date=iso -3`, `git worktree list`, `git status --porcelain`
(memory: a peer session may grab the shared checkout).

**Before dispatching any card (RED before dispatch):** run its acceptance command on the pre-change tree, require a
failure or a non-zero exit, and paste it into both the dispatch prompt and `progress.md ## Dispatch log` (SD-42
retrospective change 1: the script refuses to dispatch a card whose recorded RED exited 0). Derive its second
call sites with `git grep` over the producers its row names and add them to the prompt.

**E3.1 is the rename (operator ruling Q-22.3, 2026-11-22):** the `Feats` entry in `SHEET_TABS` becomes
`Abilities`; inside it, seven sub-tabs in PCGen's order with today's `FeatsTab` as the Feats sub-tab, unchanged in
behaviour. No second feat picker and no tab beside Feats. The lead records the ruling line in `progress.md ##
Operator replies` before E3.1 dispatches.

**After each card returns:** run the receipt-row greps (`grep -c 'closure check\|Does not cover\|RED' <receipt>`
→ 3 rows present; `Looked at` for screen cards and every MC) on that receipt before the next dependent card
dispatches (SD-42 retrospective change 3).

### 2.3 Retrospective event logging

`RETRO_ACTOR` is set by the prefix. Corrections:
`scripts/retro.py correction --subject <who-was-wrong> --claimed <value> --actual <value> --verified-by <command>`;
`incident`, `deferral`, `rework` per `python3 scripts/retro.py help <type>`. Events are written as they happen,
never batched. Field contract: `docs/retro/schema.json`.

### 2.4 Creating the Workflow script

Load the `workflow-authoring` skill first. Plain JavaScript. A script has no file access, so the card table
(id, lane, deps, tier, cargo, hub, phase) is written into the script from `kanban.md`, and every agent re-checks
its dependencies in `kanban.md` (the prefix).

1. `export const meta = { name: 'sd43-market-inventory-and-abilities', description, phases }`, one phase per §3
   row, same titles.
2. Each card's dispatch awaits its dependencies' promises, then the cargo semaphore (if `cargo != 'no'`), then the
   `hub` token (if `hub`). Lanes are serial chains; lanes run concurrently (`parallel()`). **No `isolation: 'worktree'`:** run 1 (`wf_5a8c1aee-909`) showed the
   isolated agent's sandbox refuses git commands on its named lane tree, so C1 worked in a throwaway tree instead.
   The lane tree per the prefix is the isolation (one writer per tree, own `CARGO_TARGET_DIR`).
3. **Every `agent()` sets `model`** from the Tier column, on the same line as `agent(`. Check:
   `awk '/agent\(/ && !/model:/' artifacts/cycle_0/sd43-workflow.js` prints nothing. Tier is exactly `opus`,
   `sonnet` or `haiku`; C0 (`lead`) and E4.G (`operator`) are never dispatched as work.
4. Every prompt = §2.1 prefix (bound) + §6 verbatim + the card's `epic-breakdown.md` section + its `kanban.md` row
   + the lead's pre-dispatch RED output.
5. **A refuted premise stops the chain:** `declined`, `partial` or `blocked-escalated` never reads as success; the
   lane stops at that card; independent lanes continue; dependants are not dispatched.
6. **E4.G:** the script dispatches a `sonnet` agent that writes the question block into `progress.md` (the
   proposed-names path and the "1" confirmation) and returns `partial` until an operator reply line exists. The
   run then lets every lane finish whatever does not depend on E4.G (the sheet lane runs E6.2–E6.4 and the
   campaign lane E6.MC) and returns a resume receipt `START=E4.2`. The lead restarts with `START=E4.2` once the
   operator's reply is recorded.
7. Write the run id and script path into `progress.md ## Run handle` the moment the run starts. **Never resume a
   run with parallel lanes;** start a new run with `START=<card>`.
8. The quota and disk check between dispatches: `df -h /` ≥ 24 G free, else stop with a resume receipt.

```javascript
export const meta = {
  name: 'sd43-market-inventory-and-abilities',
  description: 'SD-43 — Market and inventory, the Abilities tab, and the rest of intake items 0-23',
  phases: [
    { title: 'C — package, pointer, graph' },
    { title: 'E0 — census and engine-gap measurement' },
    { title: 'E1 — Market and inventory' },
    { title: 'E2 — sheet layout, campaign button, rest text' },
    { title: 'E3 — the Abilities tab' },
    { title: 'E4 — item names (E4.G operator wait)' },
    { title: 'E5 — reports, Update menu, Developer tab' },
    { title: 'E6 — campaign folder, House Rules, custom items' },
    { title: 'E7 — closure' },
  ],
}
// CARDS: [{id, lane, deps, tier, cargo, hub, phase}] transcribed from kanban.md; START from args.
const done = {}                                   // id -> promise of the card's result
const sem = makeSemaphore(3), hub = makeSemaphore(1)
const run = async (c) => {
  const deps = await Promise.all(c.deps.map(d => done[d]))
  if (deps.some(r => r.status !== 'complete')) return { status: 'not-dispatched', id: c.id }
  const release = [c.cargo !== 'no' ? await sem.acquire() : null, c.hub ? await hub.acquire() : null]
  try { return await agent(PREFIX(c) + PROCEDURE + criterion(c) + red(c), { model: c.tier, phase: c.phase }) }
  finally { release.forEach(r => r && r()) }
}
const lane = async (cards) => { for (const c of cards) { done[c.id] = run(c); const r = await done[c.id]; if (r.status !== 'complete') break } }
await parallel(LANES.map(cards => () => lane(cards)))   // done[] is filled lane by lane; cross-lane deps await promises
```

The skeleton shows shape only: `done[id]` must exist before a cross-lane dependant awaits it (create a deferred per
card up front, resolved by its lane). The lead adapts it after loading `workflow-authoring`.

### 2.5 A dispatched agent is never resumed

One turn; wait for slow work inside it; scope test runs to the binaries the change touches plus the MC's widest
sweep; commit and push before ending, even for a partial result. The orchestrator checks `git log` and the target
files, never the summary.

## 3. Per-epic parallel/sequential map

Phase titles above match these rows. Lanes cross epics (the `sheet` lane carries E1.7, E2, E3, E4.1–E4.2,
E6.2–E6.4); a card's phase is its epic. Every `parallel: yes` lane works in its own lane tree (no `isolation: 'worktree'`, §2.4 item 2) with its own
`CARGO_TARGET_DIR` (`/home/ubuntu/cargo-target/sd43-<lane>`, never under `/tmp`, deleted when the lane ends); at
most 3 cargo cards at once (semaphore); at most one `hub` card at once (token).

| Phase / epic | Cards | Parallel? | File-touch set (verified 2026-11-22; *new* = created by the card) | Gated on |
|---|---|---|---|---|
| C — package, pointer, graph | C0, C1, C2 | no (`main` lane; C0 is the lead) | this package; `docs/release/current-release.json`; `release-notes.md` *new*; `graphify-out/**`; `artifacts/cycle_0/**` | — |
| E0 — census and engine-gap measurement | E0.1–E0.5 | yes: lanes `census` (E0.1, E0.3, E0.5, no cargo), `market` (E0.2), `gear` (E0.4) | `artifacts/epic_0/**` (scripts *new*, outputs); `apps/desktop/src-tauri/tests/market_counts.rs` *new* (or a `#[cfg(test)]` module in `equipment_catalog.rs`) | C1 |
| E1 — Market and inventory | E1.1–E1.11, E1.MC | yes: lanes `gear` (E1.1–E1.6, E1.MC), `market` (E1.8–E1.11), `sheet` (E1.7) | `ui/characterHub/{gearScreenModel.ts,gearScreenModel.test.ts,GearScreen.tsx,GearDetail.tsx,MarketDialog.tsx,MarketDialog.test.ts,itemPickerFilter.ts,equipmentCategories.ts}`; `SellDialog.tsx`, `sellDialogModel.ts`, `MarketButton.tsx`, `marketFilters.ts` + tests *new*; `CharacterSheet.tsx`, `StarfinderCharacterSheet.tsx`, `CyberpunkCharacterSheet.tsx` (E1.7); `desktop/{character_hub.rs,equipment_catalog.rs,main.rs}` (`hub`); `ui/boundary/` describe file *new if a command is added* | E0.2, E0.4 (data); E1.7 after E1.6 |
| E2 — sheet layout, campaign button, rest text | E2.1–E2.5, E2.MC | no (`sheet` lane) | `SheetLayout.tsx` + test *new*; the three sheets; `CharacterHubPage.tsx`; `ui/campaign/CampaignManagerScreen.tsx`; `data/` rest table *new* + `tauri.conf.json` `bundle.resources` + `tests/packaged_resources.rs`; `src/rules_core/rest_text.rs` *new*; `desktop/print/blocks/rest.rs` *new*; `desktop/print/layout/mod.rs`; `desktop/character_custom.rs`; `desktop/main.rs` (`hub`) | E1.7; E2.4 on E0.5 |
| E3 — the Abilities tab | E3.1–E3.11, E3.MC | no (`sheet` lane; every card `hub` except E3.11, E3.MC) | `AbilitiesTab.tsx`, `AbilitySubTab.tsx`, `abilitySubTabModel.ts` + tests *new*; `CharacterSheet.tsx`; `src/rules_core/{ability_budget.rs,ability_prereqs.rs}` *new*, `mod.rs`, `character_input.rs`, `class_feature_pool_catalog.rs` (read API), `archetype_resolver.rs` (read API); `desktop/abilities.rs` *new*; `desktop/{main.rs,character_hub.rs,race_trait_picker.rs,character_custom.rs,character_ledger.rs}`; `ui/boundary/abilities.ts` *new*; `ui/characterHub/customModel.ts`; `desktop/print/blocks/abilities.rs` *new*; `tests/old_save_loads_unchanged.rs` | E0.1, E2.5 |
| E4 — item names (E4.G operator wait) | E4.1, E4.G, E4.2 | no (`sheet` lane; E4.G is the operator) | the display-name function's file (named by E4.1), its all-kinds test, `artifacts/epic_4/proposed-names.md`; a name table + `bundle.resources` + `packaged_resources.rs` if read at runtime | E0.3, E3.MC; E4.2 on E4.G |
| E5 — reports, Update menu, Developer tab | E5.1–E5.4, E5.MC | yes (`settings` lane, beside the others) | `desktop/report_api.rs` *new*; `desktop/main.rs` (`hub`); the build-time key stamp; `.github/workflows/publish-tester-release.yml` (the secret into the build env only); `ui/boundary/sendReport.ts` *new*; `ui/feedback/**`; `ui/testerWorkbench/feedback/**`; `ui/update/{ChannelSelector,CheckPanel,installedPanel,lastCheckPanel,Ui}.tsx`, `Ui.test.ts`; `ui/App.tsx`; `ui/settings/SettingsModal.tsx`; `ui/characterHub/{LandingScreen,CharacterHubPage,CorpusIngestDiagnosticPanel}.tsx`; `desktop/authoring_workbench.rs` | C1; E5.4 also on E2.3 (`CharacterHubPage.tsx`) |
| E6 — campaign folder, House Rules, custom items | E6.1–E6.5, E6.MC | yes: lanes `campaign` (E6.1, E6.5, E6.MC), `sheet` (E6.2–E6.4) | `desktop/campaign_folder.rs`; `ui/campaign/{campaignFolderModel,campaignMigrationModel}.ts`, `EditCampaignScreen.tsx`; `HouseRulesDialog.tsx` + model + test *new*; `desktop/character_custom.rs`; `desktop/character_hub.rs` HP path (`hub`); the rules_core two-weapon path (named by E6.4); `desktop/main.rs` (`hub`) | C1 (campaign); E3.MC (sheet) |
| E7 — closure | E7.1–E7.4 | no (`main` lane) | `apps/desktop/scripts/ui-smoke/spec.json` (sole writer E7.1), `docs/testing/ui-smoke-inventory.md`; `docs/retro/sd43-market-inventory-and-abilities-retrospective.md` *new*; `release-notes.md`; `current-release.json` (check); `docs/architecture/**`; `receipts.md`; `graphify-out/**`; `references/README.md` | E1.MC, E2.MC, E3.MC, E4.2, E5.MC, E6.MC |

**Shared-file order.** `CharacterSheet.tsx`: E1.7 → E2.1 → E2.3 → E2.5 → E3.1 … E3.10 → E6.2 (all `sheet`, serial).
`CharacterHubPage.tsx`: E2.3 before E5.4 (kanban dependency). `main.rs`: any `hub` card, one at a time.
`character_custom.rs`: E2.4, E3.10, E6.2–E6.5 — E6.5 (campaign lane) holds `hub` and runs only after E6.1;
it and the sheet lane's E6.2–E6.4 are serialised by the token because each also registers in `main.rs`.

## 4. File-touch verification

Every existing path in §3, `technical-design.md §1` and `content-unit-inventory.md` was checked on 2026-11-22 on
`tranche-43` @ `75e95b30c2` with `ls` (line numbers by the front half with `grep -n`/`sed -n` on `e451225461`;
nothing under `apps/` or `src/` changed between the two: `git diff --name-only e451225461 75e95b30c2 -- apps src | awk 'END{print NR}'` → 0).
Paths marked *new* are design choices the owning card confirms in its receipt. Paths a card must still locate
("named by E4.1", "named by E6.4") carry the locating `grep` in `epic-breakdown.md`.

## 5. Concurrent-write protocol

```bash
git fetch origin tranche-43 && git rebase origin/tranche-43 && git push origin HEAD:tranche-43
```

Retry up to 5 times on a non-fast-forward; then `CLAIM-EXISTS`, card `blocked-escalated`. Never force-push.
Re-fetch and re-read `kanban.md`/`progress.md` immediately before editing them; conflicts in those two files are
expected with seven lanes and are resolved row by row (keep both rows). Merge-check cards re-verify on
`origin/tranche-43` and merge nothing.

## 6. Per-cycle procedure (runs inside the dispatched agent — §2.2)

**Environment rules, binding in every step:** the §2.1 prefix verbatim.

1. **Base check (a control):** the prefix's first command; then §5's fetch + rebase; `CARD_BASE=$(git rev-parse HEAD)`.
2. **Disk and load:** `df -h /`; the cargo wait; claim `CARGO_TARGET_DIR`.
3. **RED:** run the card's acceptance command on the pre-change tree; it must fail for the stated reason and match
   the lead's RED in the prompt. Then write the failing test; run it; quote the failing line.
4. **GREEN:** the smallest change; run the targeted suites; clippy before the first commit of any `.rs` change;
   `pcgen-residue-gate` for any `.rs` change.
5. **Commit locally** (explicit paths), then the dual audit on **added lines**:
   ```bash
   BASE_BRANCH=$(git merge-base HEAD origin/develop)
   # Identifier audit — no trailing \b (it would stop catching sd19_class_catalog-shaped identifiers)
   git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src crates apps scripts tests data ':!**/__tests__/**' ':!**/*.test.*' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
   # Wired-integration four-check audit (docs/governance/no-stub-mvp-doctrine.md §"Per-cycle audit")
   git diff --unified=0 "${BASE_BRANCH}...HEAD" -- 'apps/desktop/**/*.ts*' 'apps/desktop/src-tauri/**/*.rs' 'src/**/*.rs' 'crates/**/*.rs' ':!**/__tests__/**' ':!**/*.test.ts' ':!**/*.test.rs' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo OK_NO_TOKENS
   git diff --unified=0 "${BASE_BRANCH}...HEAD" -- 'apps/desktop/**/*.tsx' 'apps/desktop/**/*.jsx' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE 'onClick=\{\s*\(\)\s*=>\s*\{\s*\}\s*\}|onClick=\{undefined|onDoubleClick=\{\s*\(\)\s*=>\s*\{\s*\}\s*\}|onDrop=\{\s*\(\)\s*=>\s*\{\s*\}\s*\}' || echo OK_NO_NOOP_HANDLERS
   git diff --unified=0 "${BASE_BRANCH}...HEAD" -- 'apps/desktop/**/*.ts' 'apps/desktop/**/*.tsx' 'apps/desktop/**/*.jsx' 'apps/desktop/**/*.rs' ':!**/__tests__/**' ':!**/*.test.*' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE 'mockResolvedValue|mockReturnValue\(|vi\.mock\(|__mocks__' || echo OK_NO_MOCK_LEAKS
   git diff --unified=0 "${BASE_BRANCH}...HEAD" -- 'apps/desktop/**/*.ts' 'apps/desktop/**/*.tsx' 'src/**/*.rs' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE '"Would [^"]*"' || echo OK_NO_WOULD_STRINGS
   ```
   The audits read the whole branch's added lines since `origin/develop`, so an earlier card's violation also
   shows; the receipt attributes each hit to its commit (`git log -S`). A single-token violation is fixed by a new
   commit and the audit re-run. A real stub, mock or "Would …" string in shipping code is non-self-healable (§8).
   The word "placeholder" is expected in E0.3/E4.1 **test** files (excluded) and in their artifacts (not scoped);
   in shipping code it is a hit like any other.
6. **Look at your own output** (every screen card and every MC): the `run-desktop` skill, isolated XDG_DATA_HOME,
   your `RUN_DESKTOP_AGENT`; `pdftotext` / rasterize for print cards. **Read** every PNG; write what you saw;
   fix and re-look, up to three passes; keep the final images under `artifacts/epic_<n>/<card>/` (≤ 2 MB per card,
   `du -sk`).
7. **Verify once at the widest scope the change reaches.** A root change: root
   `cargo test --locked -j 8 --lib -- --test-threads=8` **and** the desktop workspace (it consumes the root crate);
   a desktop change: `cd apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8`;
   a frontend change: `cd apps/desktop && npm run typecheck && npm test`. Every MC runs all three plus root
   `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` (bins and integration tests: `cargo build --lib`
   green is not a completed phase). **Attribute every `test result: FAILED` to its `Running` line** by name; "known
   environmental failures" is not an attribution. Do not repeat a clean gate.
8. **Receipt** to `artifacts/epic_<n>/<card-id>_cycle_receipt.md` (§7). Update the card's `kanban.md` row
   (`Status` → `complete` only with the receipt path and SHA) and append a `progress.md` cycle-log row **in the same
   commit**. A discovered card is added to `kanban.md`, `epic-breakdown.md §0` and the count line in
   `acceptance-and-verification.md` G-CL in that same commit. Unfiltered `git status --porcelain` prints nothing
   afterwards. Push via §5.
9. **Report:** status (`complete | partial | blocked-escalated | declined`), SHA, receipt path, audit results,
   RED → GREEN lines, what you looked at, discoveries, next card.

## 7. Per-cycle receipt schema

Defined once in `receipts.md §2` (path convention, fields, the four-check capture). Every receipt follows it.

## 8. Self-heal posture

- **Self-healable (fix inline, continue):** a dirty tree from your own lane; a single-token audit hit; unrelated
  test-setup breakage you can attribute; a `kanban.md`/`progress.md` rebase conflict; a missing output dir; a
  cargo-slot wait; a count two implementations re-derive.
- **Not self-healable (`blocked-escalated` on THIS card; independent lanes continue; dependants wait):** two live
  writers on one file; RED → GREEN not preserved; a stub, mock or "Would …" string in shipping code; an engine-gap
  notice still rendered at E3.MC; a data-gap count unequal to E0.1's list; the CI secret absent at E7.2; a red
  publish at E7.4; `CLAIM-EXISTS`; a red `test result: FAILED` the card cannot attribute and clear.
- **A `## Open blockers` entry in `progress.md` is a request for an operator ruling — not a disposition and never
  a closure path.** Filing one **pauses** the blocked card and everything that depends on it. A blocker between
  the bundle and 100% of its Definition of Done has two dispositions: **clear it** (decompose and run the cycles)
  or **raise your hand** (escalate naming what blocks you, what you tried, and the ruling, write scope or
  precondition needed; then stop). "Forwarded to SD-44", "deferred with reason", "out of scope for this cycle" are
  ways of writing down that the work is not done. The test against a planned capability deferral
  (`forward-scope-register.md`): was the scope in the Definition of Done at launch? If yes, it is a blocker.
- **Disk:** after every wave of parallel cards, `df -h /` and `git worktree list`; prune a merged, unlocked, clean
  lane tree and its `CARGO_TARGET_DIR` when its lane has no card left. Never remove a `locked` tree or one with
  unpushed commits (`git -C <tree> log origin/tranche-43..HEAD --oneline | awk 'END{print NR}'` → 0 first).

## 9. Placeholder-resolution checklist

```bash
grep -rn '<[a-z_-]*>' docs/release/SD-43-market-inventory-and-abilities/ --include=*.md
```

Every match must be one of:

- **(a) schema or command tokens** a running card binds: `<card-id>`, `<lane>`, `<run id>` (not matched by the
  pattern: it has a space), `<sha>`, `<n>`, `<epic>`, `<value>`, `<cmd>`, `<log>`, `<pidfile>`, `<pkg>`, `<type>`,
  `<who-was-wrong>`, `<command>`, `<command-or-check>`, `<receipt>`, `<tree>`, `<card>`, `<path>`, `<list>`,
  `<text>`, `<result>`, `<epic-start>`, `<reason>`, `<file>`, `<name>`, `<id>`, `<date>`, `<subtab>`, `<pool>`,
  `<violations>`, `<output>`, `<cycle_id>` (`<pool>`/`<subtab>` name a family of explanation-row ids and never
  resolve);
- **(b) a deferred value with a named resolution point:** `<run>` in `0.43.<run>` → the GitHub run number of the
  first publish after E7.4 merges (≥ 169, §1 item 7);
- **(c)** anything else is a defect fixed before planning-ready.

The author's run of the gate is recorded in `progress.md` Cycle 0.

## 10. Epic wrap-up (after every epic; the epic's MC card does it)

1. `scripts/retro.py summary --since <epic-start> --json`; fold the counts and any key that fired more than once
   into the MC receipt; a key firing more than twice gets a non-zero-exit check in the next card that touches the
   area (AGENTS.md rule 8).
2. The MC writes the **Epic report** block in `progress.md` (what shipped, screenshots looked at, retro counts) and
   updates `kanban.md` in its closing commit.
3. Sweep this epic's lane tree only if the lane has no card left: worktree, `sd43/<lane>` branch, `CARGO_TARGET_DIR`;
   found vs removed counts in the receipt. Never a `locked` tree or one with unpushed commits.
4. No PR.

## 11. Bundle closure epilogue (fires once, as E7; the order is binding)

1. **Final-acceptance scan (E7.2).** Every acceptance criterion in `acceptance-and-verification.md` holds and
   every `kanban.md` row other than E7.2–E7.4 is `complete` (G-CL's commands). **A filed blocker does not satisfy
   the scan**; nor does `partial`, `in-progress`, `todo`, `blocked-escalated`, or a `complete` row with half its
   criterion deferred. The scan also runs the widest verify, packaged resources, the release-tool tests, the
   fresh-install checks (.deb and AppImage) and P-8 (the CI secret exists). **If anything is short, the cycle
   stops here:** no retrospective, no sweep, **no PR**; report what is short with the command that shows it.
2. **Retrospective (E7.2).** `scripts/retro.py summary --since 2026-11-22 --json` → write
   `docs/retro/sd43-market-inventory-and-abilities-retrospective.md` in the shape of `docs/retro/sd42-retrospective.md`
   (raw tally, what the data says, what worked, what did not, changes for the next bundle, not covered); **cite it
   from `references/README.md` in the same commit**.
3. **Sweep (E7.2).** Every `sd43-*` worktree and `sd43/*` branch except `sd43-main` (E7.3–E7.4 still need it);
   delete `/home/ubuntu/cargo-target/sd43-*` except `sd43-main`'s; found vs removed counts; never `test`,
   `update-index`, a `locked` tree or another bundle's tree.
4. **Release notes + pointer (E7.3).** Final `release-notes.md` (the `SD-43 INTERIM NOTES` sentinel gone); every
   figure with its command; `current-release.json` = `{"tranche": 43, "notes_path": …SD-43…/release-notes.md}`;
   `python3 tools/release/resolve_release_notes.py --repo-root .` prints this package's notes path;
   `python3 scripts/tranche/validate-tranche-notes.py --release-notes-path docs/release/SD-43-market-inventory-and-abilities/release-notes.md` passes.
5. **Architecture docs (E7.3).** `~/.hermes/profiles/god-emporer/skills/devops/architecture-truth-up/scripts/architecture_truth_up.py --integration-target develop --receipts-md docs/release/SD-43-market-inventory-and-abilities/receipts.md --bundle SD-43`;
   if it maps 0 docs (SD-39…SD-42 precedent), hand-map from `git diff --name-status origin/develop...HEAD` and
   append a second receipt; then an Opus claims critic over every added line.
6. **Graphify LAST (E7.4).** Precondition: unfiltered `git status --porcelain` prints nothing and
   `git rev-parse HEAD` = `git rev-parse origin/tranche-43`. Run
   `~/.hermes/profiles/god-emporer/skills/devops/graphify-update/scripts/update_graphify.py --integration-target develop --receipts-md docs/release/SD-43-market-inventory-and-abilities/receipts.md --bundle SD-43`.
   **Never `--force`, never `graphify update`.** Node-count guard exit 1 → file the receipt and stop (memory:
   `--force` once replaced a 648k-node graph with a 52k AST build).
7. **PR LAST (E7.4).** Re-run step 1's scan with the exemption narrowed to E7.4; commit E7.4's row; `gh pr create
   --base develop --head tranche-43`; wait inside the turn on `gh pr checks <n> --watch`.
8. **Merge and watch (E7.4; operator ruling 2026-11-20).** When every check is green, the lead merges
   `tranche-43 → develop` (`gh pr merge <n> --merge`), then watches the develop push run of
   `publish-tester-release.yml` to green on all three OSes. A red publish is a blocker on the bundle
   (`blocked-escalated` with the failing job named), not a follow-up. No operator gate before `develop` other than
   E4.G; the operator gates `develop → test` and `test → main`, which the run never opens or merges.
9. Stop.

## 12. Standing rules

### 12.1 Carried rules (SD-36…SD-42; `../SD-42-pcgen-import-and-campaign-folder/workflow-instruction.md §12`)

| Rule | Statement | Checked by |
|---|---|---|
| Merge checks stay on Opus | every `*.MC` row's Tier is `opus` | `awk -F'\|' '$2 ~ /MC/ && $4 !~ /opus/' kanban.md` prints nothing |
| No hand-kept desktop tables | the screen prints what the engine holds | every MC |
| Isolated app-data root | any app launch uses a per-run XDG_DATA_HOME | screen-card and MC receipts |
| Long waits on Sonnet+ | no Haiku on a card that may wait > 20 min | `awk -F'\|' '$2 ~ /E7\.4/ && $4 ~ /haiku/' kanban.md` prints nothing |
| The script reads the board | dispatch only when every `Depends on` is `complete` | the prefix; the script's dependency promises |
| Never resume a parallel script | new run with `START=<card>` | `progress.md ## Run handle` |
| Zero tests is a fail | receipts quote the `N passed` line | MC review |
| RED before dispatch | the lead runs every acceptance command on the pre-change tree; exit 0 refuses dispatch | `progress.md ## Dispatch log` holds one RED per dispatched card |
| Second call sites by `git grep` | FILES YOU OWN is closed before the first write | receipt row `closure check` |
| Name the population | every "Does not cover" names the population and its denominator | MC review |
| One writer per JSON array | `spec.json`: E7.1 only | §3 file-touch sets |
| Re-count at discovery | a discovered card updates the three count lines in one commit | G-CL's row count |

### 12.2 Changes carried from SD-42's retrospective (`docs/retro/sd42-retrospective.md` "Changes for the next bundle")

| # | SD-42 change | Where SD-43 builds it |
|---|---|---|
| 1 | Commit the workflow script under `artifacts/cycle_0/` and write each card's RED into the dispatch log; refuse on exit 0 | §2.1, §2.2, §2.4; the script itself |
| 2 | The pgrep rule as a command: `pgrep -x cargo`, never `-f` on a pattern the shell holds | §2.1 prefix |
| 3 | Run the receipt-row greps on each receipt when it returns | §2.2 "After each card returns" |
| 4 | `verify.sh --only pcgen-residue-gate` on any card changing a desktop `#[cfg(test)]` module | §2.1 prefix |

### 12.3 Per-cycle and measurement rules

TDD with the failing line quoted; batch homogeneous remainders in one dispatch; one verify per wave; clean tree at
every commit; every figure with its command and denominator (awk or Python, never `grep -o`); every proof states
what it does not cover; a recurring incident key gets a control, not a warning; widest build scope (both cargo
workspaces when a root file changed); look before you claim (§6 step 6).

## Cross-references

- `../../governance/workflow-instruction-template.md` (source), `../template/template.md`.
- `../../governance/blocker-closure-doctrine.md`, `../../governance/deferral-revisit-doctrine.md`,
  `../../governance/no-stub-mvp-doctrine.md`, `../../doctrine-external/identifier-discipline.md`.
- `../SD-42-pcgen-import-and-campaign-folder/workflow-instruction.md` (shape), `../../retro/sd42-retrospective.md`.
