---
canonical: true
owner: operator
bundle_id: SD-41
slug: gear-and-equipment-sets
canonical_branch: tranche-21
target_version: 0.21.0
authored_from: docs/governance/workflow-instruction-template.md, with SD-40's live rendering as the pattern
date: 2026-11-12
status: planning-ready
---

# SD-41 Workflow Instruction — Gear and Equipment Sets

> ## OPERATING METHOD — REQUIRED FOR THIS BUNDLE
>
> **This bundle runs as a `Workflow`-tool script, invoked from a live session. It is NOT run with
> `/loop /batch` and it is NOT a one-shot task.** One long run per phase with a recorded run id
> (`decisions.md §12.2`). §2.4 says how to write the script. `scope-draft.md` is the *what*; this file
> is the *how*. The orchestrating session never executes RED→GREEN work itself (§2.2).
>
> > ### 🟡 UNATTENDED MODE — after launch, the run stops only at E4.G and between the two phases
> >
> > The standing autonomous rules apply (`decisions.md §12`): take the safe default (§12.1) and log it;
> > never call `AskUserQuestion`; clear blockers first and escalate only the card that needs a ruling;
> > report after each epic in `progress.md`; never open the PR while any card is open. **E4.G is the one
> > planned wait; the Phase-1 run ends at E1.MC by design** (`decisions.md §4`).

## 1. Pre-launch checklist (C1 runs P-1, P-2, P-4, P-7 and pastes the output in its receipt)

`technical-requirements.md §1` P-1…P-8. Items still open at authoring: the SD-40 merge (P-3, the Phase-2
precondition).

## 2. Orchestration mode

- **Dispatch:** the in-harness `Workflow` tool, one long run per phase. `epic-breakdown.md §0` is the
  concurrency, phase and tiering source of truth.
- **Tiering** (`~/.claude/CLAUDE.md` 2026-09-29): `opus` for the engine, the commands, the screen, the
  print card and every merge check; `sonnet` for the gate card, E5.1, E5.3's notes and E5.4; `haiku` for
  C1. Merge checks never downgraded.
- **Concurrency:** one serial main lane; two side lanes: `convert` (E1.8 → E1.9, tool side, Phase 1, beside
  E1.1–E1.7, landing before E1.MC) and `smoke` (E3.3, beside E3.MC and E4.1, landing before E4.MC).
  **At most 3 cargo lanes across the box, SD-40's included while it runs** (R-Q2): in Phase 1, SD-41
  builds only when `pgrep -x cargo | awk 'END{print NR}'` < 3 (process name, not -f: -f matches wrappers and /home/ubuntu/cargo-target/ paths).

### 2.1 Agent environment — the shared dispatch prefix

Every `agent()` prompt starts with this prefix, **byte-identical** across agents. Write it to
`artifacts/cycle_0/sd41-workflow.js` at launch.

```text
ENVIRONMENT (SD-41, binding — every rule here applies to you):
- export PATH="$HOME/.cargo/bin:$PATH"
- export RETRO_ACTOR=sd41-<card-id>          # the ONLY value you bind, e.g. RETRO_ACTOR=sd41-e1.2
- ONE WRITER PER TREE. The main lane works in /home/ubuntu/workspace/repos/codex/.worktrees/sd41-main
  (branch tranche-21). Each side lane (`convert`, `smoke`) makes its OWN tree first, from the pushed branch:
      git -C /home/ubuntu/workspace/repos/codex fetch origin tranche-21
      git -C /home/ubuntu/workspace/repos/codex worktree add -b sd41/<lane> \
          /home/ubuntu/workspace/repos/codex/.worktrees/sd41-<lane> origin/tranche-21
  Start EVERY command with `cd <your tree> &&` (the shell's directory resets). Never write in
  /home/ubuntu/workspace/repos/codex itself (the lead's checkout) and NEVER in
  /home/ubuntu/workspace/repos/codex/.worktrees/sd40-main or any sd40-* tree (another run's trees).
- export CARGO_TARGET_DIR=/home/ubuntu/cargo-target/sd41-<lane>    # one per lane per source tree;
  never under /tmp; never shared across trees; the main lane uses /home/ubuntu/cargo-target/sd41-main.
  Claim it: mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"
- DISK FIRST: `df -h /` before any build; a full sweep needs ~24 G free; if less, stop and report.
- AT MOST 3 CARGO LANES at once across the box, SD-40's lanes included. Before any cargo command:
  `pgrep -x cargo | awk 'END{print NR}'` < 3 (process name, not -f: -f matches wrappers and /home/ubuntu/cargo-target/ paths), else wait inside the turn (poll every 60 s, up to 40 min).
- MEMORY GUARD: one cargo process at a time in your lane; -j 8; `-- --test-threads=8`; `free -g`
  before a run expected > 10 min.
- NEVER `git stash`. NEVER `git add -A` (stage by explicit path). NEVER force-push.
  `git status --porcelain` before EVERY write; if it lists a file you did not modify, stop and report.
- TWO CARGO WORKSPACES: the repo root (src/, crates/) and apps/desktop/src-tauri are separate; the desktop
  crate is a binary crate with no lib target (its tests are #[cfg(test)] modules or tests/*.rs, run with
  `cargo test --locked -j 8 <filter> -- --test-threads=8` from that directory).
- PHASE 1 CARDS (E1.x) NEVER WRITE UNDER apps/ (C1 is the named exception: its FILES YOU OWN are the 14 F-1 version surfaces, and its commit is the base of the E1.MC partition check; the other exception: E1.8/E1.9's price test file under
  apps/desktop/src-tauri/tests/ if the catalog can only be called from that crate): `git diff --name-only <base>..HEAD -- apps | grep -v 'tests/market_price' | awk 'END{print NR}'`
  must print 0 before you commit; any other Phase-1 need for an apps/ file is a `declined` with ownedBy:<E2 card>.
- THE CONVERT LANE (E1.8, E1.9) re-converts equipment records only: after the re-convert run the field-diff
  check (only cost_gp / weight / name / raw_tokens may change; license, pi_field, pi_marker, population,
  source never) and `git diff --name-only -- data | grep -v '/equipment/' | grep -v '_settled/equipment.json' | grep -v price_overrides | grep -v '^data/rules_tables/.*equipment' | awk 'END{print NR}'` → 0 (the Market's prices live in `data/rules_tables/**/equipment*` and `equipment_gap_tables`, sd41-e1.9);
  `_settled/equipment.json` (30 bundles) is regenerated by `gen_settled_corpus` — it is where the live app reads
  prices and the settled container / ammunition / quantity fields (decisions.md §31).
- SETTLED FIELDS (E1.1, E1.6, E1.7): a corpus fact the live record needs is settled by ir_converter.rs into
  _settled/equipment.json, never read from tags or raw_tokens in live code; after every regen check
  `sha256sum apps/desktop/src-tauri/resources/corpus_fixtures/_settled/equipment.json` is unchanged (E1.9 may
  change it only for a named wrong fixture price).
- NETWORK (E1.9, E1.11): fetch d20pfsrd pages read-only with a browser user agent; record the URL per price;
  if the site refuses, stop fetching and say so in the receipt (the lead's web-fetch agent fills the list).
- CLIPPY BEFORE THE FIRST COMMIT of any card that touches a .rs file:
  `cargo clippy --locked --tests -j 8` in each touched workspace (R16).
- LONG RUNS: anything that may run > 10 minutes: write a not-run skeleton results file FIRST, then
  `nohup <cmd> > <log> 2>&1 & echo $! > <pidfile>` and poll inside this turn.
- FRONTEND: in apps/desktop run `npm run typecheck && npm test && npm run ui-smoke:doc` for any
  frontend or spec.json change; `npm run ui-smoke` for a card that adds or rewrites a ui-smoke row, always with
  `export RUN_DESKTOP_AGENT=sd41-<card-id>` first; in a fresh tree run `node scripts/gen-corpus-bundle.mjs` once before.
- `decisions.md §20` WINS over older text in this package. Reuse before you build: read the existing code
  your card names first (equipment_sets.rs once it exists, character_hub.rs's held list, body_slots.rs).
- A test filter that matches 0 tests is a FAIL. Run each filter in the cargo workspace that holds the test.
- WORKTREES: each side lane removes its own worktree, its local sd41/<lane> branch and its CARGO_TARGET_DIR
  when its work is pushed.
- SEEDS AND GOLDENS (G-1, G-2): PF seed render JSON is UNCHANGED except the NEW_KEYS the gate names; SF1e
  and CPR golden TEXT and PAGE COUNTS equal the base's; PF golden page counts may move at E4.1 and every
  move is listed before → after. Never re-baseline silently.
- THE ENGINE OWNS THE NUMBERS (NR-2): no TS and no .typ sums a weight or an AC; the projection
  (decisions.md §6) is the only bridge from a set to the engine; containers.rs is the only new sum.
- NO PCGEN IN LIVE CODE (decisions.md §17): nothing from $PCGEN_REPO_DIR enters apps/, src/, crates/,
  data/, resources/ or scripts/; the functional reference is described in technical-design.md §1 only.
- AUDITS: the identifier audit and the four-check wired-integration audit (§6 step 5) on every diff,
  on added lines, after the local commit and before the push.
- TDD: write the failing test, run it, QUOTE the failing reason in the receipt, then make it pass.
- RECEIPTS: every figure carries the command that produced it and its denominator; counts with awk or
  Python, never `grep -o`; two implementations when a figure moves a baseline.
- RETRO: emit scripts/retro.py events (correction/incident/deferral/rework) at the moment they happen,
  with RETRO_ACTOR set; a correction needs --verified-by.
- FILES YOU OWN IS CLOSED (R15, R22): before your first write, `git grep` every producer your row names
  and list every second call site; a file you need outside the list is a fence exit named in the
  receipt (decisions.md §19), never a silent edit.
- GLOSSARY (package = docs/release/SD-41-gear-and-equipment-sets/): "A<n>" in decisions.md §3 = operator
  answer; "R<n>" in this file §12 = standing rule; "SD-x" = decisions.md §12.1; "G-n" =
  acceptance-and-verification.md; "F-n" = content-unit-inventory.md; "DEF-n" / "FSR-Cn" =
  forward-scope-register.md; "location" = decisions.md §5.1; "projection" = §6; "set" = one named
  EquipmentSet in equipment_sets.json. In a table cell `\|` is Markdown for `|`.
- PAPER-SHEET RULE: compute only values that land on the sheet as numbers; print everything else; dice
  stay literal; write-in space is a ruled blank, never a value.
- AUTONOMOUS: no AskUserQuestion/clarify. If only an operator ruling can clear your blocker, return
  status blocked-escalated with the exact ruling needed. A taste question takes decisions.md §12.1's default.
- If your premise is refuted, return `declined` and name the mechanism. Do not commit a success.
- You get exactly ONE turn. Wait for slow work inside it. Commit and push before ending the turn,
  even for a partial result.
- First command of the cycle (wrong-base control):
  test -d docs && test -d data && test -d scripts && test -f docs/release/SD-41-gear-and-equipment-sets/kanban.md \
    && [ "$(git rev-parse --show-toplevel)" != /home/ubuntu/workspace/repos/codex ] \
    && [ "$(git rev-parse --show-toplevel)" != /home/ubuntu/workspace/repos/codex/.worktrees/sd40-main ] \
    && [ "$(git rev-parse --abbrev-ref HEAD)" != tranche-20 ] \
    || { echo 'WRONG BASE'; exit 1; }
```

### 2.2 Execution boundary

The session that launches SD-41 is the orchestrator. §6 runs **inside** dispatched `agent()` calls,
never as the orchestrator's own `Edit`/`Write`/`Bash` on `apps/`, `src/`, `crates/`, `scripts/`,
`tests/`, `data/` or `resources/`. The orchestrator may make read-only investigation calls, edit this
package's planning docs, and run git plumbing for them (including the SD-m catch-up rebase of
`tranche-21` onto `develop` between the phases). **Dispatch first, report second.** Never accept a lane's
message as evidence: check `git log` and the target files. Before every dispatch run
`git reflog --date=iso -3` and `git worktree list` (another session holds `sd40-main`).

**Before dispatching any card (R21, R22):** run its acceptance command on the pre-change tree and
require a failure or a non-zero exit (paste it into the dispatch prompt as the expected RED); `git grep`
each producer named in its FILES YOU OWN and add any second call site to the row before dispatch.

**Before dispatching E2.1 (R25):** `git merge-base --is-ancestor <SD-40 merge sha> HEAD` must succeed in
`sd41-main`, and `git log origin/develop -1 --format=%s` is pasted into `progress.md`; the `SD-40 merged`
token in `kanban.md` is replaced by that SHA in the same docs commit.

### 2.3 Retrospective event logging

`scripts/retro.py correction --subject … --claimed … --actual … --verified-by …`; `incident`,
`deferral`, `rework` per `python3 scripts/retro.py help <type>`. `RETRO_ACTOR` is set by the prefix.

### 2.4 Creating the Workflow script

Load the `workflow-authoring` skill first. Plain JavaScript:

1. `export const meta = { name: 'sd41-gear-and-equipment-sets', description, phases }` with one phase per
   §3 row, same titles.
2. **A script has no file access**, so the card list is in the script and each dispatched agent reads
   `kanban.md`: it returns `declined` if a `Depends on` is not `complete`. Each agent returns
   `{status, sha, receipt, discovered}`; `discovered` card IDs run before that epic's merge check (R8).
3. `pipeline()` in the main lane; `parallel()` only for the side lanes `convert` (E1.8 → E1.9 beside
   E1.1–E1.7) and `smoke` (E3.3 beside E3.MC and E4.1), each making its own worktree per the prefix (never
   the harness's isolation: its base is the session checkout).
4. Cargo lanes stay ≤ 3 by construction **including SD-40's** in Phase 1 (the prefix's `pgrep` wait).
5. **Every `agent()` sets `model`** from the Tier column, on the same line as `agent(`. Check:
   `awk '/agent\(/ && !/model:/' <script>` prints nothing. Tier is exactly `opus`, `sonnet` or `haiku`;
   C0 (`lead`) is never dispatched; E5.3 is two calls (`sonnet` notes + truth-up, then `opus` critic);
   E4.G is dispatched on `sonnet` to assemble the receipt and return `partial` until the operator's
   lines exist.
6. Every prompt = §2.1 prefix + §6 verbatim + the card's `epic-breakdown.md` row + its `kanban.md`
   row + the lead's pre-dispatch RED output (R21).
7. **Quota gate** between steps (`decisions.md §12.3`).
8. **Refuted premise stops the step:** an `ok:false` or `declined` result never reads as success.
9. Write the run id and script path into `progress.md ## Run handle` the moment the run starts.
10. **Never resume** a run with parallel lanes; start a new run with `START=<card>` (R9).
11. **Phase and gate waits:** the Phase-1 run returns after E1.MC with a resume receipt; the Phase-2 run
    starts with `START=E2.1` after the SD-40 merge and the SD-m rebase; when E4.G returns `partial`, the
    script ends the run cleanly with a resume receipt; the lead restarts with `START=E5.1` once the
    operator has signed, or with `START=<discovered card>` on defects.

```javascript
export const meta = {
  name: 'sd41-gear-and-equipment-sets',
  description: 'SD-41 — engine (sets, containers, carried weapons), desktop backend, gear screen, print + operator gate, closure',
  phases: [
    { title: 'C — package + cut' }, { title: 'E1 — engine (Phase 1)' },
    { title: 'E2 — desktop backend' }, { title: 'E3 — gear screen' }, { title: 'E4 — print + gate' },
    { title: 'E5 — closure' },
  ],
}
// tier and cargo come from epic-breakdown.md §0; deps from kanban.md (R8). model on the agent( line.
const run = (card) => agent(PREFIX + PROCEDURE + criterion(card) + red(card), { model: card.tier, phase: card.phase })
const go = async (card) => { quotaGate(); return run(card) }

if (START <= 'E1.MC') {                       // Phase 1 — may run beside SD-40
phase('C — package + cut');           await go(C1)
phase('E1 — engine (Phase 1)');       const convert = (async () => { await go(E1_8); await go(E1_9); await go(E1_11); await go(E1_12) })()   // side lane convert; E1.11 batches via the lead's web-fetch agent
                                      for (const c of [E1_1, E1_2, E1_3, E1_4, E1_5, E1_6, E1_7, E1_14, E1_10]) await go(c)   // E1.13 (discovered by E1.6) and E1.14 by the board drain
                                      await convert; await go(E1_MC)
                                      return resumeReceipt('E2.1 after SD-40 merges + SD-m rebase')
}
phase('E2 — desktop backend');        for (const c of [E2_1, E2_2, E2_3, E2_4, E2_5, E2_6, E2_7, E2_MC]) await go(c)
phase('E3 — gear screen');            await go(E3_1); await go(E3_2); await go(E3_4); await go(E3_5)
                                      const smoke = go(E3_3)              // side lane smoke, beside E3.MC + E4.1
                                      await go(E3_MC)
phase('E4 — print + gate');           for (const c of [E4_1, E4_2, E4_3, E4_4]) await go(c); await smoke; await go(E4_MC)
                                      { const r = await go(E4_G); if (r.status !== 'complete') return resumeReceipt('E4.G') }
phase('E5 — closure');                for (const c of [E5_1, E5_2, E5_3a, E5_3b, E5_4]) await go(c)
```

### 2.5 A dispatched agent is never resumed

One turn; wait for slow work inside it; commit and push before ending even for a partial result.

## 3. Per-epic parallel/sequential map

Phase titles above match these rows. Shared-file writer order: `epic-breakdown.md §0`.

| Phase / epic | Cards | Parallel? | File-touch set | Gated on |
|---|---|---|---|---|
| C — package + cut | C0, C1 | no | this package; the 14 F-1 surfaces + `current-release.json` | — |
| E1 — engine (Phase 1) | E1.1–E1.7, E1.10, E1.13, E1.14, E1.MC; `convert`: E1.8, E1.9, E1.11, E1.12 | `convert` ∥ E1.1–E1.7 | main: `src/rules_core/{equipment_record,sheet_rule,containers,damage_total,encumbrance,mod}.rs`, `src/rules_core/pilot_compute/{combat,class_monk}.rs` (named lines), every `compute_encumbrance` caller (`&[]`), `src/saved_character/{equipment_sets,campaign_mirror,mod,local_store}.rs`, `crates/codex-ingest/tests/equipment_sets_pcg_population.rs` *(new)*; `convert`: `crates/codex-ingest/src/pcgen_import/**` (named), `data/corpus/**/equipment/**`, `src/rules_core/rules_catalog/**` (regenerated), `data/price_overrides/*.json` *(new)*, the price test file; **nothing else under `apps/`** | C1; SD-40 lanes ≤ 2 for cargo |
| E2 — desktop backend | E2.1–E2.7, E2.MC | no | `desktop/character_hub.rs`, `desktop/main.rs`, `desktop/character_custom.rs`, `desktop/equipment_catalog.rs`, `tests/every_character_root_writer_mirrors.rs` *(new)*, `ui/boundary/setCharacterCampaigns.ts` *(new)*, `desktop/pf1_adapter.rs` (named), `desktop/pf_seed_render_hash.rs` (if pinned), `src/saved_character/equipment_sets.rs` (`copyNames`), `src/rules_core/body_slots.rs` (named, E2.6), `tests/old_save_loads_unchanged.rs` + fixture *(new)*, `ui/boundary/{loadSavedCharacterDetail,loadCreateCharacter,listHeldEquipment,setEquipmentActiveState,loadCharacterCustom,purchaseEquipment,listMarket}.ts`, `ui/boundary/{moveEquipmentCopy,equipmentSets,createCustomItem,sellEquipment,discardEquipmentCopy,renameEquipmentCopy,purchaseCustomEquipment,listCustomEquipmentOptions,quoteCustomEquipment}.ts` *(new or extended)*, `ui/characterHub/CharacterSheet.tsx` (line 3295; the Weapons tab rows), `ui/characterHub/weaponsTabModel.ts` + test *(new)*, `ui/characterHub/{CustomDialog,CustomTab}.tsx`, `customModel.ts` + test, `MarketDialog.test.ts` | E1.MC; **SD-40 merged + SD-m rebase (R25)** |
| E3 — gear screen | E3.1, E3.2, E3.4, E3.5, E3.3, E3.MC | `smoke` (E3.3) ∥ E3.MC, E4.1 | `ui/characterHub/GearScreen.tsx` *(new)*, `gearScreenModel.ts` + test *(new)*, `CharacterSheet.tsx` (the Gear tab line + handlers; the Market handlers; the Campaign field and the Skills footer), `CharacterHubPage.tsx` (the `campaign-sheet` mode), `CampaignMembershipDialog.tsx` + `campaignMembershipModel.ts` + test *(new)*, `EquipmentPanel.tsx` + test (deleted), `MarketDialog.tsx` + test, `marketModel.ts` + test *(new)*, `{BuyCustomDialog,CreateItemDialog}.tsx` *(new)*; `smoke`: `spec.json`, `docs/testing/ui-smoke-inventory.md`, `apps/desktop/scripts/ui-smoke/**` (only a missing step kind), `artifacts/epic_3/E3.3/**` | E2.MC |
| E4 — print + gate | E4.1, E4.2, E4.3, E4.4, E4.MC, E4.G | no | `desktop/print/blocks/gear.rs`, `print/layout/pf1e.rs` (if the rows' shape changes), `resources/print/looks/{gear,weapons,skills,speed,portrait,conditional-bonuses,…}.typ` (the 11 page-1 looks, E4.2), `resources/print/frame.typ`, `resources/print/logos/*.png` *(new, operator-supplied)*, `tauri.conf.json`, `tests/packaged_resources.rs` (E4.4), `print/blocks/core.rs`, `print/golden/print_golden.json`, `print/blocks/print_pf_blocks.rs`, `print/print_golden.rs`, `artifacts/epic_4/**` | E3.MC; E4.MC also on E3.3 |
| E5 — closure | E5.1–E5.4 | no | `scripts/verify-baselines.env`, `docs/retro/sd41-retrospective.md`, `docs/architecture/**`, graph outputs, this folder, `artifacts/epic_5/**` | E4.G |

**Fences.** A lane that needs a file outside its row returns `declined` with `ownedBy:<card>` unless
that file's epic merge check is `complete` (`decisions.md §19`).

## 4. File-touch verification

Each existing path named in §3, `technical-design.md` and `content-unit-inventory.md` was checked on
2026-11-12 with `ls`/`grep -n`/`sed -n` on `tranche-20` @ `f7f85f3d24`; line counts are quoted in
`technical-design.md §0` and `census.txt`. Paths marked *(new)* are design choices the owning card
confirms and names in its receipt. Phase-2 paths are re-checked by the lead after the SD-m rebase
(SD-40's E3.2 may have renamed `gear.rs`'s block; E4.1's row says so).

## 5. Concurrent-write protocol

```bash
git fetch origin tranche-21 && git rebase origin/tranche-21 && git push origin HEAD:tranche-21
```

Retry up to 5 times on a non-fast-forward; then `CLAIM-EXISTS`, card `blocked-escalated`. Never
force-push. Re-fetch and re-read `progress.md`/`kanban.md` immediately before every edit. Rebase
conflict: SD-k. Every card lands by pushing its own commits to `tranche-21`; merge-check cards re-verify
on `origin/tranche-21` and merge nothing.

## 6. Per-cycle procedure (runs inside the dispatched agent)

**Environment rules, binding in every step:** the §2.1 prefix verbatim.

1. **Base check** (a control): the prefix's wrong-base test. Then §5's fetch+rebase; record
   `CARD_BASE=$(git rev-parse HEAD)`.
2. **Disk and load:** `df -h /`; the cargo-lane wait; claim `CARGO_TARGET_DIR`.
3. **Baseline:** run the card's acceptance command on the pre-change tree and keep its output (RED; it
   must match the lead's R21 output in the prompt); render the PF seeds (G-1) into `<dir>/base` when the
   card touches the load response or `damage_total.rs`; the SF/CPR goldens' text (G-2) when it touches
   `print/**` or `resources/print/**`.
4. **TDD:** write the failing test; run it; quote the failing reason; make the smallest change; run the
   targeted suites. **Clippy before the first commit** for any `.rs` change (R16).
5. **Commit locally** (explicit paths), then run the **dual audit on added lines**:
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
   ```
   A violation is fixed by a new commit and the audit re-run. A real stub is non-self-healable.
6. **Clippy stage and the audit test** in every card whose diff touches a `.rs` file (R11):
   `bash scripts/verify.sh --only clippy -j 8` and root
   `cargo test --locked -j 8 --test sd24_wired_integration_audit -- --test-threads=8`.
7. **Seeds and goldens** (G-1, G-2): PF seed JSON `SAME_EXCEPT_NAMED` per seed; SF/CPR golden text and
   page counts `diff` empty; PF golden page counts listed before → after (E4.1).
8. **Look at your own output** (every screen card's merge check, E4.1, E4.MC): the `run-desktop` skill
   with an isolated `XDG_DATA_HOME` (R5) for screenshots; `print_proof` + `scripts/print/rasterize.sh`
   for PDFs; **Read** every PNG; write what you saw and fix it; up to three passes; copy the final
   images under `artifacts/epic_<n>/<card>/`; `du -sk` ≤ 2048 (SD-n). The receipt's "Looked at" row
   names the files and the passes.
9. **Verify once at the widest scope the change reaches:** root `cargo test --locked -j 8 --lib`
   (+ `crates/codex-ingest`) for a root change; desktop `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8`
   for a desktop change; `cd apps/desktop && npm run typecheck && npm test && npm run ui-smoke:doc` when
   frontend or `spec.json` changed. Attribute every `test result: FAILED` to its `Running` line. Do not
   repeat a clean gate (R-V).
10. **Receipt** to `artifacts/epic_<n>/<card>_cycle_receipt.md` (§7). Update the card's `kanban.md` row
    and append a `progress.md` row **in the same commit**. If you added a discovered card, re-count the
    card-count constant in `acceptance-and-verification.md` G-8 and `kanban.md` in that commit (R18).
    Unfiltered `git status --porcelain` prints nothing after it. Push via §5.
11. **Report:** status (`complete | partial | blocked-escalated`), SHA, receipt path, seed/golden deltas,
    what you looked at, discoveries.

## 7. Per-cycle receipt schema

```markdown
# Cycle <card-id> — <epic> / <criterion title>

- **Card ID:** <card-id>   **Model:** <opus|sonnet|haiku>   **RETRO_ACTOR:** <value>
- **Commit SHA:** <sha>    **Base SHA:** <tranche-21 sha at start>
- **Files touched:** <list>   (every one inside FILES YOU OWN, or a named fence exit)
- **FILES YOU OWN closure check:** <producers git-grepped; second call sites listed>
- **Acceptance criterion (verbatim from epic-breakdown.md):** <text>
- **RED (pre-change command + output, failing reason quoted; equals the lead's R21 output):** <…>
- **GREEN (same command + output):** <…>
- **Figures:** <value> — <predicate/denominator> — <command A> → <output>; <command B> → <output>
- **Build scope verified:** <root | apps/desktop/src-tauri | frontend> + log path
- **Clippy before first commit:** <result>   **Clippy stage + sd24 audit test:** <result>
- **Identifier audit:** OK_NO_BUNDLE_TAGS | <violations>
- **Wired-integration audit:** OK_NO_TOKENS / OK_NO_NOOP_HANDLERS / OK_NO_MOCK_LEAKS / OK_NO_WOULD_STRINGS | <violations>
- **Partition (Phase 1 only):** `git diff --name-only <base>..HEAD -- apps | awk 'END{print NR}'` → 0
- **Seed deltas (G-1):** PF Aldric / Elowen: SAME_EXCEPT_NAMED <keys> | VALUE_MOVED (a defect)
- **Golden deltas (G-2, print cards):** SF1e ×8, CPR ×8 text + pages: none; PF ×8 pages: <case: before → after>
- **Looked at (screen and print cards, every MC):** <PNG paths; passes; differences left and why>
- **Rulings applied:** <A<n> / SD-x + alternative not taken>
- **Does not cover:** <the shapes this proof does not exercise, with the population and its denominator named (R23)>
- **Status:** complete | partial | blocked-escalated
- **Remainder (partial only):** <every remaining item by sub-cause; populations sum to the total>
- **Retro events emitted:** <types + counts>
- **Next-cycle plan:** <…>
```

## 8. Self-heal posture

- **Self-healable:** dirty tree from your own lane; a single-token audit violation; unrelated test-setup
  breakage; a count two implementations re-derive; a missing out dir (`mkdir -p`); a PF golden page
  count that moved (listed); a cargo-lane wait.
- **Not self-healable (`blocked-escalated` on THIS card; continue independent cards):** two live lanes
  on one file; RED→GREEN not preserved; a stub, mock or "Would …" string in shipping code; a `VALUE_MOVED`
  seed the card cannot explain by a named key; an SF/CPR golden text delta; a Phase-1 need for an `apps/`
  file; `CLAIM-EXISTS`.
- **Blocker doctrine:** cleared or escalated, never deferred. The test: was this scope in the Definition
  of Done at scoping? If yes, it is a blocker.
- **Disk:** after a side lane ends, `df -h /` and `git worktree list`; prune the merged, unlocked,
  clean `sd41-convert` / `sd41-smoke` worktree and its `CARGO_TARGET_DIR`. Never remove a `locked` tree or any `sd40-*` tree.

## 9. Placeholder-resolution checklist

```bash
grep -rn '<[a-z_-]*>' docs/release/SD-41-gear-and-equipment-sets/*.md
```

Every match must be (a) a schema placeholder inside a fenced template or a command argument the running
card binds (`<sha>`, `<base>`, `<head>`, `<dir>`, `<lane>`, `<card>`, `<card-id>`, `<n>`, `<epic>`,
`<list>`, `<text>`, `<output>`, `<violations>`, `<result>`, `<value>`, `<cmd>`, `<log>`, `<pidfile>`,
`<script>`, `<type>`, `<keys>`, `<file>`, `<id>`, `<path>`, `<defect>`, `<itemId>`, `<bonus>`, `<ordinal>`, `<kind>`, `<slot>`, `<lb>`,
`<name>`, `<state>`, `<select>`, `<out>`, `<clean-worktree>`, `<epic-start>`, `<date>`, `<filter>`, `<description>`, `<run>`,
`<your tree>`, `<E2 card>`, `<E1.MC sha>`, `<E3.1 sha>`, `<C1 sha>`, `<SD-40 merge sha>`, `<the SD-40 closing count>`, `<character root>`,
`<total>`, `<column>`, `<eqmod keys>`, `<eqmod key>`, `<lb>`, `<target>`, `<reason>`, `<campaignId>`, `<campaign dir>`, `<campaign_dir>`, `<character_id>`, `<pid>`, `<root>`, `<slug>` (the mirror's path schema, `decisions.md §29`), `<url>`, `<book>`, `<subdir>` (the settled-bundle path, `decisions.md §31`), `<system>` (the logo asset path, `decisions.md §34`), `<ft>` (the `MOVE` token, `decisions.md §35`)),
(b) a deferred value with a named resolution point (`0.21.<run>` → first tester publish after the
operator merges; the run id → launch; the SD-40 merge SHA → the lead at R25), or (c) a bug to fix before
launch. The author's run is recorded in `progress.md`.

## 10. Epic wrap-up (after every epic)

1. `scripts/retro.py summary --since <epic-start> --json`; fold the counts and any key that fired more
   than once into the merge-check receipt; a key firing more than twice gets a non-zero-exit check in
   the next card that touches that area (R-W).
2. The merge-check card writes the **Epic report** block in `progress.md` and updates `kanban.md` in its
   own closing commit.
3. Sweep this epic's merged, unlocked, clean `sd41-*` worktrees and delete their `CARGO_TARGET_DIR`s.
4. No PR.

## 11. Bundle closure epilogue (corrected order — `decisions.md §18`)

1. **E5.2 final-acceptance scan** — the fenced commands in `acceptance-and-verification.md` G-8. Pass =
   no output from the two `awk`s, `usable: yes` 3 (E4.G), the price gate passed with every
   Market key in `price-unknown.json` carrying a `tried` list (`decisions.md §30.1`), every DEF/FSR revisit command pasted. **If
   anything is short, stop here:** no retrospective, no sweep, **no PR**. That is a correct outcome.
2. **E5.2 retrospective:** `scripts/retro.py summary --since 2026-11-12 --json`; write
   `docs/retro/sd41-retrospective.md` in the shape of the latest sibling retrospective; cite it from
   `references/README.md` in the same commit.
3. **E5.2 sweep:** every `sd41-*` worktree and `sd41/*` branch except `sd41-main`, which E5.3–E5.4 still
   need (the script's last step removes it after the PR); found vs removed counts; never `test`,
   `update-index`, `sd40-*` or a locked tree; delete `/home/ubuntu/cargo-target/sd41-*`.
4. **E5.3 release notes:** replace the interim body; `grep -c 'SD-41 INTERIM NOTES'` → 0; every figure with
   its command; `python3 tools/release/resolve_release_notes.py --repo-root .` passes.
5. **E5.3 architecture truth-up**
   (`~/.hermes/profiles/god-emporer/skills/devops/architecture-truth-up/scripts/architecture_truth_up.py --integration-target develop --receipts-md docs/release/SD-41-gear-and-equipment-sets/receipts.md --bundle SD-41`),
   then an Opus claims critic.
6. **E5.4 graphify LAST.** Precondition: unfiltered `git status --porcelain` prints nothing and
   `git rev-parse HEAD` = `git rev-parse origin/tranche-21`. Run
   `~/.hermes/profiles/god-emporer/skills/devops/graphify-update/scripts/update_graphify.py --integration-target develop --receipts-md docs/release/SD-41-gear-and-equipment-sets/receipts.md --bundle SD-41`.
   On the node-count guard (exit 1): file the receipt and stop. **Never** a force flag.
7. **E5.4 PR** `tranche-21 → develop`, the final action: re-run step 1 with the exemption narrowed to
   `E5\.4`; the sentinel grep → 0; commit E5.4's own `complete` row; `gh pr create`; wait inside the
   turn for `gh pr checks <n> --watch`. Red or unopenable → one docs commit setting E5.4
   `blocked-escalated` with the failing job named. **The operator merges.**
8. Stop.

## 12. Standing rules

### 12.1 Carried from SD-36–SD-40 (R1–R24, `../SD-40-pf-sheet-design/workflow-instruction.md §12`)

| Rule | Statement | Checked by |
|---|---|---|
| R2 No hand-kept desktop tables | the screen prints what the engine holds; no table of values in TSX or Typst | E3.MC, E4.MC |
| R3 Merge checks stay on Opus | every `*.MC` is `opus` | `awk -F'\|' '$2 ~ /MC/ && $4 !~ /opus/' kanban.md` prints nothing |
| R4 `ownedBy` declines | a declined item naming a later card goes to that card | script review |
| R5 Isolated app-data root | any app-launching harness uses a per-run `XDG_DATA_HOME` | E3.3, E3.MC, E4.MC, E5.1 receipts |
| R6 Long waits on Sonnet+ | no Haiku on runs that may exceed 20 min | Tier of E5.1, E5.4 ≠ haiku |
| R7 Per-system status | every receipt reports PF seed / SF / CPR / PF golden deltas where it renders | `grep -L 'Seed deltas' artifacts/epic_[2-5]/*_cycle_receipt.md` prints nothing |
| R8 The script reads the board | dispatch only when every `Depends on` is `complete` | script review |
| R9 Never resume a parallel script | new run with `START=<card>` | `progress.md` run handle |
| R10 Fences have an exit | `decisions.md §19` | receipts |
| R11 Clippy + audit test every code card | §6 step 6 | receipts |
| R12 Generated artifacts re-run-and-diff | `ui-smoke:doc` regenerated and committed by every `spec.json` writer | `git status --porcelain docs/testing/ui-smoke-inventory.md` empty |
| R13 Re-derive copied figures | every figure with its command | receipts |
| R15 FILES YOU OWN is closed | each card greps producers and second call sites before its first write; the receipt has the closure-check row | `grep -L 'closure check' artifacts/epic_*/*_cycle_receipt.md` prints nothing |
| R16 Clippy before the first commit | `cargo clippy --locked --tests` before any `.rs` card's first local commit | receipt row |
| R17 Full ui-smoke has an owner | the full suite is **not** a gate in SD-41; E3.3 runs its 13 rows by name | E3.3 receipt; G-6 |
| R18 Re-count at discovery | the card that adds a discovered row edits the card-count constant in G-8 and `kanban.md` in the same commit | E5.2's `ROW_COUNT` check never fires |
| R19 Observed concurrency | `git status --porcelain` and `git worktree list` before dispatching the side lane or any Phase-1 card | the lead's dispatch log |
| R20 Zero tests is a fail | a filter matching 0 tests proves nothing; receipts quote the `N passed` line | receipts |
| R21 RED before dispatch | the lead runs every card's acceptance command on the pre-change tree and requires a failure or non-zero exit | the dispatch prompt carries the RED output; the receipt's RED row equals it |
| R22 Second call sites by `git grep` | the lead derives each card's second-call-site list over the producers named in its row before dispatch | the dispatch log in `progress.md` names the grep per card |
| R23 Name the population | every non-MC card's "Does not cover" row names the population it proved and its denominator | `grep -L 'denominator\|of [0-9]' artifacts/epic_*/*_cycle_receipt.md` read by each MC |
| R24 One writer per JSON array | one writer at a time on `spec.json`: E3.3 only, after SD-40's E4.2 has merged; one writer on `data/price_overrides/*.json`: E1.9 | §3 file-touch sets; phasing |

### 12.2 SD-41's own rule (R25) and SD-40's retrospective

| Rule | Statement | Checked by |
|---|---|---|
| **R25 Phase 2 starts on the merged base** | E2.1 is dispatched only after `git merge-base --is-ancestor <SD-40 merge sha> HEAD` succeeds in `sd41-main`; the `SD-40 merged` token in `kanban.md` is replaced by that SHA; an SD-41 card never writes an SD-40-owned file before then | §2.2; `kanban.md` E2.1 row; the prefix's `tranche-20` branch check |

SD-40's retrospective (`docs/retro/sd40-retrospective.md`) is written at SD-40's E5.3, after this package
was authored. **The lead reads its "Changes for the next bundle" before dispatching E2.1** and records
each as a rule row here or a `decisions.md §20` correction (the stc-authoring skill's audit item 5).

### 12.3 Per-cycle and measurement rules

**R-T TDD**, failing reason quoted. **R-B Batch big:** homogeneous remainders in one dispatch. **R-V One
verify per wave.** **R-C Clean tree** at every commit. **R-N** every figure with its command and
denominator; awk or Python, never `grep -o`. **R-P** every proof states what it does not cover. **R-W** a
recurring incident key gets a control, not a warning. **R-G Widest build scope:** both cargo workspaces
when a root file changed. **R-X Cross-tree renders:** base renders build in their own worktree with their
own `CARGO_TARGET_DIR`. **R-Q Quota:** `decisions.md §12.3`. **R-Q2 Cargo lanes count the box:** SD-40's
lanes count toward the 3 while it runs. **R-L Look before you claim:** §6 step 8.

## Cross-references

- `docs/governance/workflow-instruction-template.md` (source; its §11 order is defective),
  `docs/release/template/template.md`.
- `docs/governance/blocker-closure-doctrine.md`, `docs/governance/deferral-revisit-doctrine.md`,
  `docs/governance/no-stub-mvp-doctrine.md`.
- `../SD-40-pf-sheet-design/workflow-instruction.md` (R1–R24), `docs/retro/sd39-retrospective.md`.
