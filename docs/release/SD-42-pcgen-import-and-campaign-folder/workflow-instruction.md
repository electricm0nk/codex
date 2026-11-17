---
canonical: true
owner: operator
bundle_id: SD-42
slug: pcgen-import-and-campaign-folder
canonical_branch: tranche-22
target_version: 0.22.0
authored_from: docs/governance/workflow-instruction-template.md, with SD-41's live rendering as the pattern
date: 2026-11-16
status: planning-ready
---

# SD-42 Workflow Instruction — PCGen Import Accuracy and the Campaign Folder

> ## OPERATING METHOD — REQUIRED FOR THIS BUNDLE
>
> **This bundle runs as a `Workflow`-tool script, invoked from a live session. It is NOT run with
> `/loop /batch` and it is NOT a one-shot task.** One long run per phase with a recorded run id
> (`decisions.md §12.2`). §2.4 says how to write the script. `scope-draft.md` is the *what*; this file is the
> *how*. The orchestrating session never executes RED→GREEN work itself (§2.2).
>
> > ### 🟡 UNATTENDED MODE — after launch, the run stops only at E5.G and between the two phases
> >
> > The standing autonomous rules apply (`decisions.md §12`): take the safe default (§12.1) and log it; never
> > call `AskUserQuestion`; clear blockers first and escalate only the card that needs a ruling; report after
> > each epic in `progress.md`; never open the PR while any card is open. **E5.G is the one planned wait; the
> > Phase-1 run ends when E1.MC, E2.MC and E3.MC are `complete`** (`decisions.md §4`).

## 1. Pre-launch checklist (C1 runs P-1, P-2, P-4, P-7, P-8 and pastes the output in its receipt)

`technical-requirements.md §1` P-1…P-12. Items still open at authoring: the SD-41 merge (P-3, the Phase-2
precondition); the PCGen engine run (P-9, checked at E1.7).

## 2. Orchestration mode

- **Dispatch:** the in-harness `Workflow` tool, one long run per phase. `epic-breakdown.md §0` is the
  concurrency, phase and tiering source of truth.
- **Tiering** (`~/.claude/CLAUDE.md` 2026-09-29): `opus` for every implementation card and every merge check;
  `sonnet` for the gate card, E5.1, E5.3's notes and E5.4; `haiku` for C1. Merge checks never downgraded.
- **Concurrency:** three Phase-1 lanes in their own worktrees — `import` (E1), `campaign` (E2), `editor` (E3,
  no cargo) — each serial inside; Phase 2 serial in `sd42-main`. **At most 4 cargo lanes across the box,
  SD-41's included while it runs** (R-Q2): a card builds only when `pgrep -x cargo | awk 'END{print NR}'` < 4.
- **Unique `RUN_DESKTOP_AGENT` per dispatch** (SD-41's lesson): every prompt exports
  `RUN_DESKTOP_AGENT=sd42-<card-id>-<run id>` so two app launches never share a profile or a screenshot dir.

### 2.1 Agent environment — the shared dispatch prefix

Every `agent()` prompt starts with this prefix, **byte-identical** across agents except the three bound values
(`<card-id>`, `<lane>`, `<run id>`). Write it to `artifacts/cycle_0/sd42-workflow.js` at launch.

```text
ENVIRONMENT (SD-42, binding — every rule here applies to you):
- export PATH="$HOME/.cargo/bin:$PATH"
- export RETRO_ACTOR=sd42-<card-id>              # e.g. RETRO_ACTOR=sd42-e1.2
- export RUN_DESKTOP_AGENT=sd42-<card-id>-<run id> # unique per dispatch; never reuse another card's
- ONE WRITER PER TREE. Your lane's tree is /home/ubuntu/workspace/repos/codex/.worktrees/sd42-<lane>
  (lanes: import, campaign, editor in Phase 1; main in Phase 2), branch tranche-22. A side lane makes its OWN
  tree first, from the pushed branch:
      git -C /home/ubuntu/workspace/repos/codex fetch origin tranche-22
      git -C /home/ubuntu/workspace/repos/codex worktree add -b sd42/<lane> \
          /home/ubuntu/workspace/repos/codex/.worktrees/sd42-<lane> origin/tranche-22
  Start EVERY command with `cd <your tree> &&` (the shell's directory resets). Never write in
  /home/ubuntu/workspace/repos/codex itself (the lead's checkout), NEVER in
  /home/ubuntu/workspace/repos/codex/.worktrees/sd41-main or any sd41-* tree (another run's trees), and never
  in another sd42-* lane's tree.
- export CARGO_TARGET_DIR=/home/ubuntu/cargo-target/sd42-<lane>    # one per lane per source tree; never under
  /tmp; never shared across trees. Claim it: mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"
- DISK FIRST: `df -h /` before any build; a full sweep needs ~24 G free; if less, stop and report.
- AT MOST 4 CARGO LANES at once across the box, SD-41's lanes included. Before any cargo command:
  `pgrep -x cargo | awk 'END{print NR}'` < 4 (process name, not -f), else wait inside the turn (poll every
  60 s, up to 40 min).
- MEMORY GUARD: one cargo process at a time in your lane; -j 8; `-- --test-threads=8`; `free -g` before a run
  expected > 10 min.
- NEVER `git stash`. NEVER `git add -A` (stage by explicit path). NEVER force-push.
  `git status --porcelain` before EVERY write; if it lists a file you did not modify, stop and report.
- TWO CARGO WORKSPACES: the repo root (src/, crates/) and apps/desktop/src-tauri are separate; the desktop
  crate is a binary crate with no lib target (its tests are #[cfg(test)] modules or tests/*.rs, run with
  `cargo test --locked -j 8 <filter> -- --test-threads=8` from that directory). codex-ingest tests run from
  crates/codex-ingest.
- PHASE 1 PARTITION (decisions.md §4): no card writes apps/desktop/src-tauri/src/character_hub.rs,
  apps/desktop/src/characterHub/BuyCustomDialog.tsx, docs/architecture/** or scripts/verify-baselines.env before
  SD-41 merges; the import lane writes nothing under apps/ at all. Before you commit:
  `git diff --name-only <base>..HEAD -- apps/desktop/src-tauri/src/character_hub.rs apps/desktop/src/characterHub/BuyCustomDialog.tsx docs/architecture scripts/verify-baselines.env | awk 'END{print NR}'`
  must print 0; a Phase-1 need for one of them is a `declined` with ownedBy:<E4 card>.
- LANE FENCES: the import lane owns crates/codex-ingest/src/pcg_import/**, crates/codex-ingest/tests/pcg_import_*,
  scripts/oracle_harness/pcg_import_parity/**, and only the named lines of src/saved_character/equipment_sets.rs,
  src/rules_core/damage_total.rs; the campaign lane owns src/campaign/**, src/saved_character/campaign_mirror.rs,
  apps/desktop/src-tauri/src/{campaign_folder,campaign_drive,encounter_rating}.rs, main.rs registration lines,
  capabilities/default.json, apps/desktop/src/campaign/**, settings/{GoogleDrivePanel.tsx,googleDrive.ts,SettingsModal.tsx},
  boundary/campaignFolder/**, boundary/{listDriveCampaigns,writeCampaignLocalFolderArtifacts}.ts, characterHub/
  {CharacterHubPage.tsx,CampaignMembershipDialog.tsx,campaignMembershipModel.ts}, dmToolkit/**; the editor lane
  owns apps/desktop/src/editor/** and apps/desktop/package.json + package-lock.json (E2.5 adds ONE line to
  package.json for the opener: a named fence exit, applied after E3.1 is complete or declined ownedBy:E3.1).
- THE OPERATOR'S FILE (decisions.md §10; NR-4): read it only from "$CODEX_SD42_PCG"
  (default /home/ubuntu/sd42-input/Aonghus Macleod.pcg); write its outputs only under /home/ubuntu/sd42-input/;
  never copy, quote a line of, or screenshot it into the repo; receipts carry COUNTS. Before every commit:
  `git diff --cached -S"$SD42_CHAR" --stat -- . ':!docs/release/SD-42-pcgen-import-and-campaign-folder'` and
  `git diff --cached -S"$SD42_PII" --stat` must print nothing (the lead sets SD42_CHAR and SD42_PII in the
  dispatch environment; if they are unset, run `export SD42_CHAR=$(awk -F: '/^CHARACTERNAME:/{print $2}' "$CODEX_SD42_PCG")`
  and the PLAYERNAME twin first). A test needing the file prints `SKIPPED: operator .pcg absent at <path>
  (decisions.md §10)` and passes when it is absent.
- THE ORACLE (decisions.md §22): resolve it via `$PCGEN_REPO_DIR` (default $HOME/workspace/repos/pcgen) and
  `$PCGEN_CORPUS_ROOT`; never write a literal path; `git -C "$PCGEN_REPO_DIR" rev-parse HEAD` must equal
  PCGEN_ORACLE_SHA in scripts/pcgen-oracle-pin.env before any oracle read; nothing from the oracle enters
  live code; the parity exports under scripts/oracle_harness/pcg_import_parity/exports/ are the one committed
  oracle output (plain totals of PCGen's own sample characters).
- NO PCGEN IN LIVE CODE: `bash scripts/verify.sh --only pcgen-residue-gate -j 8` PASS on every import-lane card.
- CLIPPY BEFORE THE FIRST COMMIT of any card that touches a .rs file: `cargo clippy --locked --tests -j 8`
  in each touched workspace (R16).
- LONG RUNS: anything that may run > 10 minutes: write a not-run skeleton results file FIRST, then
  `nohup <cmd> > <log> 2>&1 & echo $! > <pidfile>` and poll inside this turn.
- FRONTEND: in apps/desktop run `npm run typecheck && npm test && npm run ui-smoke:doc` for any frontend or
  spec.json change; `npm run ui-smoke -- --only <row>` for a card that adds or rewrites a ui-smoke row; in a
  fresh tree run `node scripts/gen-corpus-bundle.mjs` once before. New npm packages: only those decisions.md
  §18 and §13.3 name; quote `npm view <pkg> version license` in the receipt.
- CAMPAIGN FOLDERS IN TESTS: every test and smoke row makes its campaign folder under a temp dir or
  `{XDG_DATA_HOME}`; never under the repo, never under the operator's home outside those.
- `decisions.md §20` WINS over older text in this package. Reuse before you build: read the existing code your
  card names first (map.rs, equipment_sets.rs, campaign_mirror.rs, local_store.rs, sf_adapter.rs's loaders).
- A test filter that matches 0 tests is a FAIL. Run each filter in the cargo workspace that holds the test.
- WORKTREES: each side lane removes its own worktree, its local sd42/<lane> branch and its CARGO_TARGET_DIR
  when its MC is complete and pushed.
- SEEDS AND GOLDENS (G-1, G-2, Phase 2): PF seed render JSON is UNCHANGED except the NEW_KEYS the gate names;
  SF1e and CPR golden TEXT and PAGE COUNTS equal the base's. Never re-baseline silently.
- THE ENGINE OWNS THE NUMBERS (NR-2): no TS and no .typ sums a weight or a price or rolls a die; the three
  copy overrides are read in one engine place each.
- NO STUBS (NR-15): a tab, a button, a pin, a menu that ships does what it says; a tab whose feature card has
  not landed renders a read-only file list, never an empty handler.
- AUDITS: the identifier audit and the four-check wired-integration audit (§6 step 5) on every diff, on added
  lines, after the local commit and before the push.
- TDD: write the failing test, run it, QUOTE the failing reason in the receipt, then make it pass.
- RECEIPTS: every figure carries the command that produced it and its denominator; counts with awk or Python,
  never `grep -o`; two implementations when a figure moves a baseline.
- RETRO: emit scripts/retro.py events (correction/incident/deferral/rework) at the moment they happen, with
  RETRO_ACTOR set; a correction needs --verified-by.
- FILES YOU OWN IS CLOSED (R15, R22): before your first write, `git grep` every producer your row names and
  list every second call site; a file you need outside the list is a fence exit named in the receipt
  (decisions.md §19), never a silent edit.
- GLOSSARY (package = docs/release/SD-42-pcgen-import-and-campaign-folder/): "Q<n>" in decisions.md §3 = operator
  ruling; "Q-<letter>" in §21 = a question with its default; "R<n>" in this file §12 = standing rule; "SD-x" =
  decisions.md §12.1; "G-n" = acceptance-and-verification.md; "F-n" = content-unit-inventory.md; "DEF-n" /
  "FSR-Cn" = forward-scope-register.md; "H<n>" = technical-design.md §1 hypotheses. In a table cell `\|` is
  Markdown for `|`.
- PAPER-SHEET RULE: compute only values that land on the sheet as numbers; print everything else; dice stay
  literal; a property with no number is printed text.
- AUTONOMOUS: no AskUserQuestion/clarify. If only an operator ruling can clear your blocker, return status
  blocked-escalated with the exact ruling needed. A taste question takes decisions.md §12.1's default.
- If your premise is refuted (an H the diagnosis disproved, a test that passes on the base), return `declined`
  and name the mechanism. Do not commit a success.
- You get exactly ONE turn. Wait for slow work inside it. Commit and push before ending the turn, even for a
  partial result.
- First command of the cycle (wrong-base control):
  test -d docs && test -d data && test -d scripts && test -f docs/release/SD-42-pcgen-import-and-campaign-folder/kanban.md \
    && [ "$(git rev-parse --show-toplevel)" != /home/ubuntu/workspace/repos/codex ] \
    && case "$(git rev-parse --show-toplevel)" in */.worktrees/sd41-*) echo 'WRONG BASE'; exit 1;; esac \
    && [ "$(git rev-parse --abbrev-ref HEAD)" != tranche-21 ] \
    || { echo 'WRONG BASE'; exit 1; }
```

### 2.2 Execution boundary

The session that launches SD-42 is the orchestrator. §6 runs **inside** dispatched `agent()` calls, never as
the orchestrator's own `Edit`/`Write`/`Bash` on `apps/`, `src/`, `crates/`, `scripts/`, `tests/`, `data/` or
`resources/`. The orchestrator may make read-only investigation calls, edit this package's planning docs, and
run git plumbing for them (including the SD-m catch-up rebase of `tranche-22` onto `develop` between the
phases). **Dispatch first, report second.** Never accept a lane's message as evidence: check `git log` and the
target files. Before every dispatch run `git reflog --date=iso -3` and `git worktree list` (another session
holds `sd41-main`).

**Before dispatching any card (R21, R22):** run its acceptance command on the pre-change tree and require a
failure or a non-zero exit (paste it into the dispatch prompt as the expected RED); `git grep` each producer
named in its FILES YOU OWN and add any second call site to the row before dispatch.

**Before dispatching E1.2–E1.7:** read E1.1's receipt; a refuted hypothesis re-scopes the card by a
`decisions.md §20` row before dispatch (R-2).

**Before dispatching E4.1 (R25):** `git merge-base --is-ancestor <SD-41 merge sha> HEAD` must succeed in
`sd42-main`, and `git log origin/develop -1 --format=%s` is pasted into `progress.md`; the `SD-41 merged` token
in `kanban.md` is replaced by that SHA in the same docs commit; SD-41's retrospective is read (§12.2).

### 2.3 Retrospective event logging

`scripts/retro.py correction --subject … --claimed … --actual … --verified-by …`; `incident`, `deferral`,
`rework` per `python3 scripts/retro.py help <type>`. `RETRO_ACTOR` is set by the prefix.

### 2.4 Creating the Workflow script

Load the `workflow-authoring` skill first. Plain JavaScript:

1. `export const meta = { name: 'sd42-pcgen-import-and-campaign-folder', description, phases }` with one phase
   per §3 row, same titles.
2. **A script has no file access**, so the card list is in the script and each dispatched agent reads
   `kanban.md`: it returns `declined` if a `Depends on` is not `complete`. Each agent returns
   `{status, sha, receipt, discovered}`; `discovered` card IDs run before that epic's merge check (R8).
3. `parallel()` for the three Phase-1 lanes, each a `pipeline()` of its cards; every lane makes its own
   worktree per the prefix (never the harness's isolation: its base is the session checkout). Phase 2 is one
   `pipeline()` in `sd42-main`.
4. Cargo lanes stay ≤ 4 by construction **including SD-41's** (two cargo lanes + the editor's none + SD-41's).
5. **Every `agent()` sets `model`** from the Tier column, on the same line as `agent(`. Check:
   `awk '/agent\(/ && !/model:/' <script>` prints nothing. Tier is exactly `opus`, `sonnet` or `haiku`; C0
   (`lead`) is never dispatched; E5.3 is two calls (`sonnet` notes + truth-up, then `opus` critic); E5.G is
   dispatched on `sonnet` to assemble the receipt and return `partial` until the operator's lines exist.
6. Every prompt = §2.1 prefix (with `<card-id>`, `<lane>`, `<run id>` bound) + §6 verbatim + the card's
   `epic-breakdown.md` row + its `kanban.md` row + the lead's pre-dispatch RED output (R21).
7. **Quota gate** between steps (`decisions.md §12.3`).
8. **Refuted premise stops the step:** an `ok:false` or `declined` result never reads as success; a `declined`
   naming `ownedBy:<later card>` lets the lane continue (R4).
9. Write the run id and script path into `progress.md ## Run handle` the moment the run starts.
10. **Never resume** a run with parallel lanes; start a new run with `START=<card>` (R9).
11. **Phase and gate waits:** the Phase-1 run returns after the three MCs with a resume receipt; the Phase-2
    run starts with `START=E4.1` after the SD-41 merge and the SD-m rebase; when E5.G returns `partial`, the
    script ends the run cleanly with a resume receipt; the lead restarts with `START=E5.1` once the operator
    has signed, or with `START=<discovered card>` on defects.

```javascript
export const meta = {
  name: 'sd42-pcgen-import-and-campaign-folder',
  description: 'SD-42 — PCGen import accuracy (PF + SF) and the campaign folder (Manager, tabs, Characters/, editor), two phases',
  phases: [
    { title: 'C — package + cut' }, { title: 'E1 — import converter (lane import)' },
    { title: 'E2 — campaign folder (lane campaign)' }, { title: 'E3 — editor (lane editor)' },
    { title: 'E4 — desktop wiring (Phase 2)' }, { title: 'E5 — gate + closure' },
  ],
}
// tier and lane come from epic-breakdown.md §0; deps from kanban.md (R8). model on the agent( line.
const run = (card) => agent(PREFIX(card) + PROCEDURE + criterion(card) + red(card), { model: card.tier, phase: card.phase })
const go = async (card) => { quotaGate(); return run(card) }
const lane = async (cards) => { for (const c of cards) await go(c) }

if (START <= 'E3.MC') {                       // Phase 1 — three lanes beside SD-41
phase('C — package + cut');                 await go(C1)
await parallel([
  lane([E1_1, E1_2, E1_3, E1_4, E1_5, E1_6, E1_7, E1_MC]),                       // phase E1
  lane([E2_1, E2_2, E2_3, E2_4, E2_5, E2_6, E2_7, E2_8, E2_9, E2_10, E2_11, E2_MC]), // phase E2; E2.6 waits on E3.1 via kanban deps (declined → retried after E3.1)
  lane([E3_1, E3_2, E3_3, E3_MC]),                                               // phase E3
])
return resumeReceipt('E4.1 after SD-41 merges + SD-m rebase')
}
phase('E4 — desktop wiring (Phase 2)');     await lane([E4_1, E4_2, E4_3, E4_4, E4_5, E4_MC])
phase('E5 — gate + closure');               { const r = await go(E5_G); if (r.status !== 'complete') return resumeReceipt('E5.G') }
                                            await lane([E5_1, E5_2, E5_3a, E5_3b, E5_4])
```

A `declined` with `ownedBy:E3.1` from E2.6 (the editor not yet landed) makes the campaign lane wait: the lane
helper re-dispatches E2.6 after a 10-minute poll of `kanban.md` shows E3.1 `complete` (the agent itself checks
the dependency; the helper only waits).

### 2.5 A dispatched agent is never resumed

One turn; wait for slow work inside it; commit and push before ending even for a partial result.

## 3. Per-epic parallel/sequential map

Phase titles above match these rows. Shared-file writer order: `epic-breakdown.md §0`.

| Phase / epic | Cards | Parallel? | File-touch set | Gated on |
|---|---|---|---|---|
| C — package + cut | C0, C1 | no | this package; the 14 version surfaces + `current-release.json` | — |
| E1 — import converter (lane `import`, Phase 1) | E1.1–E1.7, E1.MC | ∥ E2, E3 | `crates/codex-ingest/src/pcg_import/**`, `crates/codex-ingest/tests/pcg_import_*.rs` *(new)*, `crates/codex-ingest/tests/fixtures/pcg_stacks.pcg` *(new)*, `crates/codex-ingest/tests/common/mod.rs` (a helper), `src/saved_character/equipment_sets.rs` (four `CopyFacts` fields + `eqmod_option_key`), `scripts/oracle_harness/pcg_import_parity/**` *(new)*; **nothing under `apps/`** | C1 |
| E2 — campaign folder (lane `campaign`, Phase 1) | E2.1–E2.11, E2.MC | ∥ E1, E3 | `src/campaign/{mod,local_store,folder,frontmatter,adventure_log,maps,wiki,characters,migrate}.rs`, `src/saved_character/campaign_mirror.rs`, `desktop/campaign_folder.rs` *(new)*, `desktop/campaign_drive.rs` *(deleted)*, `desktop/main.rs` (registration), `desktop/encounter_rating.rs`, `apps/desktop/src-tauri/capabilities/default.json`, `ui/campaign/**`, `ui/settings/{GoogleDrivePanel.tsx,googleDrive.ts,SettingsModal.tsx}`, `ui/boundary/campaignFolder/**` *(new)*, `ui/boundary/{listDriveCampaigns,writeCampaignLocalFolderArtifacts}.ts` *(deleted)*, `ui/boundary/{rateEncounter,openExternal}.ts`, `ui/characterHub/{CharacterHubPage.tsx,CampaignMembershipDialog.tsx,campaignMembershipModel.ts}`, `ui/dmToolkit/**`, `ui/App.tsx` (if it names the gate), one `package.json` line (E2.5, fence exit) | C1; E2.6 on E3.1 |
| E3 — editor (lane `editor`, Phase 1, no cargo) | E3.1–E3.3, E3.MC | ∥ E1, E2 | `ui/editor/**` *(new)*, `apps/desktop/package.json`, `package-lock.json`; E3.3's `save_attachment` in `desktop/campaign_folder.rs` + `main.rs` as a fence exit after E2.1 | C1 |
| E4 — desktop wiring (Phase 2) | E4.1–E4.5, E4.MC | no | `desktop/character_hub.rs`, `desktop/tests/{import_writes_equipment_sets,mirror_two_master,import_operator_file}.rs` *(new)*, `src/rules_core/{damage_total,encumbrance,containers}.rs`, `ui/boundary/listHeldEquipment.ts`, `ui/characterHub/{GearScreen.tsx,gearScreenModel.ts,CharacterSheet.tsx,CampaignMembershipDialog.tsx,campaignMembershipModel.ts}`, `ui/campaign/CampaignSheet.tsx`, `desktop/print/blocks/gear.rs`, `resources/print/looks/{gear,weapons}.typ`, `print/golden/print_golden.json`, `spec.json`, `docs/testing/ui-smoke-inventory.md`, `apps/desktop/scripts/ui-smoke/**` (the substitution), `docs/architecture/pcg-import.md`, `crates/codex-ingest/tests/pcg_import_operator_file.rs` *(new)* | E1.MC, E2.MC, E3.MC; **SD-41 merged + SD-m rebase (R25)** |
| E5 — gate + closure | E5.G, E5.1–E5.4 | no | `scripts/verify-baselines.env`, `docs/retro/sd42-retrospective.md`, `docs/architecture/**`, graph outputs, this folder, `artifacts/epic_5/**` | E4.MC |

**Fences.** A lane that needs a file outside its row returns `declined` with `ownedBy:<card>` unless that
file's epic merge check is `complete` (`decisions.md §19`).

## 4. File-touch verification

Each existing path named in §3, `technical-design.md` and `content-unit-inventory.md` was checked on
2026-11-16 with `ls`/`grep -n`/`sed -n` on `tranche-22` @ `1046daefc0`; line counts are in
`artifacts/epic_0/census.json`. Paths marked *(new)* are design choices the owning card confirms and names in its
receipt. Phase-2 paths are re-checked by the lead after the SD-m rebase (SD-41's E3.12 may have moved lines in
`character_hub.rs`).

## 5. Concurrent-write protocol

```bash
git fetch origin tranche-22 && git rebase origin/tranche-22 && git push origin HEAD:tranche-22
```

Retry up to 5 times on a non-fast-forward; then `CLAIM-EXISTS`, card `blocked-escalated`. Never force-push.
Re-fetch and re-read `progress.md`/`kanban.md` immediately before every edit. Rebase conflict: SD-k. Every
card lands by pushing its own commits to `tranche-22`; merge-check cards re-verify on `origin/tranche-22` and
merge nothing. Three lanes push to one branch: `kanban.md` and `progress.md` conflicts are expected and SD-k
resolves them row by row.

## 6. Per-cycle procedure (runs inside the dispatched agent)

**Environment rules, binding in every step:** the §2.1 prefix verbatim.

1. **Base check** (a control): the prefix's wrong-base test. Then §5's fetch+rebase; record
   `CARD_BASE=$(git rev-parse HEAD)`.
2. **Disk and load:** `df -h /`; the cargo-lane wait; claim `CARGO_TARGET_DIR`.
3. **Baseline:** run the card's acceptance command on the pre-change tree and keep its output (RED; it must
   match the lead's R21 output in the prompt); in Phase 2 render the PF seeds (G-1) into `<dir>/base` when the
   card touches the load response or `damage_total.rs`; the SF/CPR goldens' text (G-2) when it touches
   `print/**` or `resources/print/**`.
4. **TDD:** write the failing test; run it; quote the failing reason; make the smallest change; run the
   targeted suites. **Clippy before the first commit** for any `.rs` change (R16).
5. **Commit locally** (explicit paths), then run the **dual audit on added lines** and the **PII check**:
   ```bash
   git diff --unified=0 "${CARD_BASE}...HEAD" -- src crates apps scripts tests ':!**/__tests__/**' ':!**/*.test.*' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
   git diff --unified=0 "${CARD_BASE}...HEAD" -- 'apps/desktop/**/*.ts*' 'apps/desktop/src-tauri/**/*.rs' 'apps/desktop/src-tauri/resources/print/**' 'src/**/*.rs' 'crates/**/*.rs' ':!**/__tests__/**' ':!**/*.test.ts' ':!**/*.test.rs' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo OK_NO_TOKENS
   git diff --unified=0 "${CARD_BASE}...HEAD" -- 'apps/desktop/**/*.tsx' 'apps/desktop/**/*.jsx' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE 'onClick=\{\s*\(\)\s*=>\s*\{\s*\}\s*\}|onClick=\{undefined|onDrop=\{\s*\(\)\s*=>\s*\{\s*\}\s*\}' || echo OK_NO_NOOP_HANDLERS
   git diff --unified=0 "${CARD_BASE}...HEAD" -- 'apps/desktop/**/*.ts' 'apps/desktop/**/*.tsx' 'apps/desktop/**/*.jsx' 'apps/desktop/**/*.rs' ':!**/__tests__/**' ':!**/*.test.*' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE 'mockResolvedValue|mockReturnValue\(|vi\.mock\(|__mocks__' || echo OK_NO_MOCK_LEAKS
   git diff --unified=0 "${CARD_BASE}...HEAD" -- 'apps/desktop/**/*.ts' 'apps/desktop/**/*.tsx' 'src/**/*.rs' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE '"Would [^"]*"' || echo OK_NO_WOULD_STRINGS
   git log "${CARD_BASE}..HEAD" -p -S"$SD42_CHAR" -- . ':!docs/release/SD-42-pcgen-import-and-campaign-folder' | awk 'END{print NR}'   # 0
   git log "${CARD_BASE}..HEAD" -p -S"$SD42_PII" -- . | awk 'END{print NR}'                                                      # 0 → OK_NO_PII
   ```
   A violation is fixed by a new commit and the audit re-run. A real stub is non-self-healable. A PII hit is
   an incident: rewrite the local commits before any push (`git reset --soft "${CARD_BASE}"`, remove the file,
   recommit) and emit `scripts/retro.py incident`.
6. **Clippy stage and the audit test** in every card whose diff touches a `.rs` file (R11):
   `bash scripts/verify.sh --only clippy -j 8` and root
   `cargo test --locked -j 8 --test sd24_wired_integration_audit -- --test-threads=8`.
7. **Seeds and goldens** (Phase 2; G-1, G-2): PF seed JSON `SAME_EXCEPT_NAMED` per seed; SF/CPR golden text
   and page counts `diff` empty.
8. **Look at your own output** (every screen card's merge check, E2.6, E2.8, E2.9, E4.2, E4.MC): the
   `run-desktop` skill with an isolated `XDG_DATA_HOME` (R5) and your unique `RUN_DESKTOP_AGENT` for
   screenshots; `print_proof` + `scripts/print/rasterize.sh` for PDFs; **Read** every PNG; write what you saw
   and fix it; up to three passes; copy the final images under `artifacts/epic_<n>/<card>/`; `du -sk` ≤ 2048
   (SD-n). The receipt's "Looked at" row names the files and the passes.
9. **Verify once at the widest scope the change reaches:** root `cargo test --locked -j 8 --lib`
   (+ `crates/codex-ingest`) for a root change; desktop `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8`
   for a desktop change; `cd apps/desktop && npm run typecheck && npm test && npm run ui-smoke:doc` when
   frontend or `spec.json` changed. Attribute every `test result: FAILED` to its `Running` line. Do not repeat
   a clean gate (R-V).
10. **Receipt** to `artifacts/epic_<n>/<card>_cycle_receipt.md` (§7). Update the card's `kanban.md` row and
    append a `progress.md` row **in the same commit**. If you added a discovered card, re-count the card-count
    constant in `acceptance-and-verification.md` G-9 and `kanban.md` in that commit (R18). Unfiltered
    `git status --porcelain` prints nothing after it. Push via §5.
11. **Report:** status (`complete | partial | blocked-escalated`), SHA, receipt path, seed/golden deltas, what
    you looked at, discoveries.

## 7. Per-cycle receipt schema

```markdown
# Cycle <card-id> — <epic> / <criterion title>

- **Card ID:** <card-id>   **Model:** <opus|sonnet|haiku>   **RETRO_ACTOR:** <value>   **RUN_DESKTOP_AGENT:** <value>
- **Commit SHA:** <sha>    **Base SHA:** <tranche-22 sha at start>    **Lane / tree:** <lane> / <path>
- **Files touched:** <list>   (every one inside FILES YOU OWN, or a named fence exit)
- **FILES YOU OWN closure check:** <producers git-grepped; second call sites listed>
- **Acceptance criterion (verbatim from epic-breakdown.md):** <text>
- **RED (pre-change command + output, failing reason quoted; equals the lead's R21 output):** <…>
- **GREEN (same command + output):** <…>
- **Figures:** <value> — <predicate/denominator> — <command A> → <output>; <command B> → <output>
- **Build scope verified:** <root | codex-ingest | apps/desktop/src-tauri | frontend> + log path
- **Clippy before first commit:** <result>   **Clippy stage + sd24 audit test:** <result>
- **Identifier audit:** OK_NO_BUNDLE_TAGS | <violations>
- **Wired-integration audit:** OK_NO_TOKENS / OK_NO_NOOP_HANDLERS / OK_NO_MOCK_LEAKS / OK_NO_WOULD_STRINGS | <violations>
- **PII check:** OK_NO_PII (both greps → 0)
- **Partition (Phase 1 only):** the G-3 command → 0
- **Seed deltas (G-1, Phase 2):** PF Aldric / Elowen: SAME_EXCEPT_NAMED <keys> | VALUE_MOVED (a defect)
- **Golden deltas (G-2, print cards):** SF1e ×8, CPR ×8 text + pages: none; PF: <case: before → after>
- **Looked at (screen and print cards, every MC):** <PNG paths; passes; differences left and why>
- **Rulings applied:** <Q<n> / Q-<letter> / SD-x + alternative not taken>
- **Does not cover:** <the shapes this proof does not exercise, with the population and its denominator named (R23)>
- **Status:** complete | partial | blocked-escalated
- **Remainder (partial only):** <every remaining item by sub-cause; populations sum to the total>
- **Retro events emitted:** <types + counts>
- **Next-cycle plan:** <…>
```

## 8. Self-heal posture

- **Self-healable:** dirty tree from your own lane; a single-token audit violation; unrelated test-setup
  breakage; a count two implementations re-derive; a missing out dir (`mkdir -p`); a `kanban.md`/`progress.md`
  rebase conflict (SD-k); a cargo-lane wait; a `declined ownedBy:` from a lane whose dependency lands later.
- **Not self-healable (`blocked-escalated` on THIS card; continue independent cards):** two live lanes on one
  file; RED→GREEN not preserved; a stub, mock or "Would …" string in shipping code; a `VALUE_MOVED` seed the
  card cannot explain by a named key; an SF/CPR golden text delta; a Phase-1 need for an SD-41-owned file; the
  PCGen engine not runnable after P-9's missing piece is named; `CLAIM-EXISTS`. **A PII hit is an incident, not
  a card state:** fix before any push.
- **Blocker doctrine:** cleared or escalated, never deferred. The test: was this scope in the Definition of
  Done at scoping? If yes, it is a blocker.
- **Disk:** after a lane ends, `df -h /` and `git worktree list`; prune the merged, unlocked, clean
  `sd42-import` / `sd42-campaign` / `sd42-editor` worktree and its `CARGO_TARGET_DIR`. Never remove a `locked`
  tree or any `sd41-*` tree.

## 9. Placeholder-resolution checklist

```bash
grep -rn '<[a-z_-]*>' docs/release/SD-42-pcgen-import-and-campaign-folder/*.md
```

Every match must be (a) a schema placeholder inside a fenced template or a command argument the running card
binds (`<sha>`, `<base>`, `<head>`, `<dir>`, `<lane>`, `<card>`, `<card-id>`, `<n>`, `<epic>`, `<list>`, `<text>`,
`<output>`, `<violations>`, `<result>`, `<value>`, `<cmd>`, `<log>`, `<pidfile>`, `<script>`, `<type>`, `<keys>`,
`<file>`, `<id>`, `<path>`, `<name>`, `<pkg>`, `<row>`, `<filter>`, `<date>`, `<out dir>`, `<sample>`, `<item>`,
`<field>`, `<base>`, `<die>`, `<m>`, `<ability>`, `<reason>`, `<device>`, `<slug>`, `<characterId>`, `<campaign folder>`,
`<campaign dir>`, `<campaign>`, `<campaign name>`, `<parent>`, `<picked parent>`, `<app data>`, `<ISO date>`,
`<source file>`, `<set name>`, `<your name>`, `<author>`, `<title>`, `<user>`, `<old updatedAt date>`,
`<session start YYYY-MM-DD>`, `<YYYY-MM-DD>`, `<folder name singular>`, `<your tree>`, `<run id>`, `<E4 card>`,
`<C1 sha>`, `<SD-41 merge sha>`, `<later card>`, `<Location JSON as equipment_sets.rs serialises it>`,
`<parent entry's key>`, `<longsword key>`, `<XDG campaign dir>`, `<each new row>`, `<the user's name>`, `<run>`, `<letter>`, `<next>`, `<folder>`, `<campaign_dir>`, `<epic-start>`),
(b) a deferred value with a named resolution point (`0.22.<run>` → first tester publish after the operator merges;
the run id → launch; the SD-41 merge SHA → the lead at R25), or (c) a bug to fix before launch. The author's run is
recorded in `progress.md`.

## 10. Epic wrap-up (after every epic)

1. `scripts/retro.py summary --since <epic-start> --json`; fold the counts and any key that fired more than
   once into the merge-check receipt; a key firing more than twice gets a non-zero-exit check in the next card
   that touches that area (R-W).
2. The merge-check card writes the **Epic report** block in `progress.md` and updates `kanban.md` in its own
   closing commit.
3. Sweep this epic's merged, unlocked, clean `sd42-*` worktree and delete its `CARGO_TARGET_DIR`.
4. No PR.

## 11. Bundle closure epilogue (corrected order — `decisions.md §23`)

1. **E5.2 final-acceptance scan** — the fenced commands in `acceptance-and-verification.md` G-9. Pass = no
   output from the two `awk`s, `usable: yes` 3 (E5.G), the E4.5 receipt holding the local run's counts with no
   `SKIPPED` line, every DEF/FSR revisit command pasted. **If anything is short, stop here:** no
   retrospective, no sweep, **no PR**. That is a correct outcome.
2. **E5.2 retrospective:** `scripts/retro.py summary --since 2026-11-16 --json`; write
   `docs/retro/sd42-retrospective.md` in the shape of the latest sibling retrospective; cite it from
   `references/README.md` in the same commit.
3. **E5.2 sweep:** every `sd42-*` worktree and `sd42/*` branch except `sd42-main`, which E5.3–E5.4 still need
   (the script's last step removes it after the PR); found vs removed counts; never `test`, `update-index`,
   `sd41-*` or a locked tree; delete `/home/ubuntu/cargo-target/sd42-*`.
4. **E5.3 release notes:** replace the interim body; `grep -c 'SD-42 INTERIM NOTES'` → 0; every figure with
   its command; `python3 tools/release/resolve_release_notes.py --repo-root .` passes.
5. **E5.3 architecture truth-up**
   (`~/.hermes/profiles/god-emporer/skills/devops/architecture-truth-up/scripts/architecture_truth_up.py --integration-target develop --receipts-md docs/release/SD-42-pcgen-import-and-campaign-folder/receipts.md --bundle SD-42`),
   then an Opus claims critic.
6. **E5.4 graphify LAST.** Precondition: unfiltered `git status --porcelain` prints nothing and
   `git rev-parse HEAD` = `git rev-parse origin/tranche-22`. Run
   `~/.hermes/profiles/god-emporer/skills/devops/graphify-update/scripts/update_graphify.py --integration-target develop --receipts-md docs/release/SD-42-pcgen-import-and-campaign-folder/receipts.md --bundle SD-42`.
   On the node-count guard (exit 1): file the receipt and stop. **Never** a force flag.
7. **E5.4 PR** `tranche-22 → develop`, the final action: re-run step 1 with the exemption narrowed to `E5\.4`;
   the sentinel grep → 0; commit E5.4's own `complete` row; `gh pr create`; wait inside the turn for
   `gh pr checks <n> --watch`. Red or unopenable → one docs commit setting E5.4 `blocked-escalated` with the
   failing job named. **The operator merges.**
8. Stop.

## 12. Standing rules

### 12.1 Carried from SD-36–SD-41 (R1–R25, `../SD-41-gear-and-equipment-sets/workflow-instruction.md §12`)

| Rule | Statement | Checked by |
|---|---|---|
| R2 No hand-kept desktop tables | the screen prints what the engine holds; no table of values in TSX or Typst | every MC |
| R3 Merge checks stay on Opus | every `*.MC` is `opus` | `awk -F'\|' '$2 ~ /MC/ && $4 !~ /opus/' kanban.md` prints nothing |
| R4 `ownedBy` declines | a declined item naming a later card goes to that card and never stops the lane | script review |
| R5 Isolated app-data root | any app-launching harness uses a per-run `XDG_DATA_HOME` | E2.MC, E4.4, E4.MC, E5.1 receipts |
| R6 Long waits on Sonnet+ | no Haiku on runs that may exceed 20 min | Tier of E5.1, E5.4 ≠ haiku |
| R7 Per-system status | every Phase-2 receipt reports PF seed / SF / CPR / PF golden deltas where it renders | `grep -L 'Seed deltas' artifacts/epic_[45]/*_cycle_receipt.md` prints nothing |
| R8 The script reads the board | dispatch only when every `Depends on` is `complete` | script review |
| R9 Never resume a parallel script | new run with `START=<card>` | `progress.md` run handle |
| R10 Fences have an exit | `decisions.md §19` | receipts |
| R11 Clippy + audit test every code card | §6 step 6 | receipts |
| R12 Generated artifacts re-run-and-diff | `ui-smoke:doc` regenerated and committed by every `spec.json` writer | `git status --porcelain docs/testing/ui-smoke-inventory.md` empty |
| R13 Re-derive copied figures | every figure with its command | receipts |
| R15 FILES YOU OWN is closed | each card greps producers and second call sites before its first write; the receipt has the closure-check row | `grep -L 'closure check' artifacts/epic_*/*_cycle_receipt.md` prints nothing |
| R16 Clippy before the first commit | `cargo clippy --locked --tests` before any `.rs` card's first local commit | receipt row |
| R17 Full ui-smoke has an owner | the full suite is **not** a gate in SD-42; E4.4 runs its rows by name | E4.4 receipt |
| R18 Re-count at discovery | the card that adds a discovered row edits the card-count constant in G-9 and `kanban.md` in the same commit | E5.2's `ROW_COUNT` check never fires |
| R19 Observed concurrency | `git status --porcelain` and `git worktree list` before dispatching any lane | the lead's dispatch log |
| R20 Zero tests is a fail | a filter matching 0 tests proves nothing; receipts quote the `N passed` line | receipts |
| R21 RED before dispatch | the lead runs every card's acceptance command on the pre-change tree and requires a failure or non-zero exit | the dispatch prompt carries the RED output; the receipt's RED row equals it |
| R22 Second call sites by `git grep` | the lead derives each card's second-call-site list over the producers named in its row before dispatch | the dispatch log in `progress.md` names the grep per card |
| R23 Name the population | every non-MC card's "Does not cover" row names the population it proved and its denominator | `grep -L 'denominator\|of [0-9]' artifacts/epic_*/*_cycle_receipt.md` read by each MC |
| R24 One writer per JSON array | one writer at a time on `spec.json`: E4.4 only; one writer on `package.json`: E3.1, then E2.5's one line after E3.1 is `complete` | §3 file-touch sets |
| R25 Phase 2 starts on the merged base | E4.1 is dispatched only after `git merge-base --is-ancestor <SD-41 merge sha> HEAD` succeeds in `sd42-main`; the `SD-41 merged` token in `kanban.md` is replaced by that SHA; an SD-42 card never writes an SD-41-owned file before then | §2.2; `kanban.md` E4.1 row; the prefix's `tranche-21` branch check |

### 12.2 SD-42's own rules (R26–R28) and SD-41's retrospective

| Rule | Statement | Checked by |
|---|---|---|
| **R26 Unique `RUN_DESKTOP_AGENT` per dispatch** | every prompt binds `RUN_DESKTOP_AGENT=sd42-<card-id>-<run id>`; two dispatches never share one (SD-41: concurrent app launches collided on one profile) | the prefix; `grep -c RUN_DESKTOP_AGENT artifacts/cycle_0/sd42-workflow.js` ≥ 1; receipts carry the value |
| **R27 The operator's file never enters the repo** | the PII check in §6 step 5 on every commit; counts only in receipts; the local path only as `$CODEX_SD42_PCG` | `OK_NO_PII` in every receipt; G-8 at every MC |
| **R28 Diagnose before you fix** | E1.2–E1.7 are dispatched only after E1.1's receipt marks each hypothesis they rest on; a refuted one re-scopes the card by a `§20` row first | §2.2; the dispatch log names the H rows per card |

SD-41's retrospective (`docs/retro/sd41-retrospective.md`) is written at SD-41's E5.2, after this package was
authored. **The lead reads its "Changes for the next bundle" before dispatching E4.1** and records each as a
rule row here or a `decisions.md §20` correction (the stc-authoring skill's audit item 5).

### 12.3 Per-cycle and measurement rules

**R-T TDD**, failing reason quoted. **R-B Batch big:** homogeneous remainders in one dispatch. **R-V One verify
per wave.** **R-C Clean tree** at every commit. **R-N** every figure with its command and denominator; awk or
Python, never `grep -o`. **R-P** every proof states what it does not cover. **R-W** a recurring incident key
gets a control, not a warning. **R-G Widest build scope:** both cargo workspaces when a root file changed.
**R-X Cross-tree renders:** base renders build in their own worktree with their own `CARGO_TARGET_DIR`.
**R-Q Quota:** `decisions.md §12.3`. **R-Q2 Cargo lanes count the box:** SD-41's lanes count toward the 4 while
it runs. **R-L Look before you claim:** §6 step 8.

## Cross-references

- `docs/governance/workflow-instruction-template.md` (source; its §11 order is defective),
  `docs/release/template/template.md`.
- `docs/governance/blocker-closure-doctrine.md`, `docs/governance/deferral-revisit-doctrine.md`,
  `docs/governance/no-stub-mvp-doctrine.md`.
- `../SD-41-gear-and-equipment-sets/workflow-instruction.md` (R1–R25), `docs/retro/sd40-retrospective.md`.
