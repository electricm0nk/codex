---
canonical: true
owner: operator
bundle_id: SD-39
slug: ui-print-corrections
canonical_branch: tranche-19
target_version: 0.19.0
authored_from: docs/governance/workflow-instruction-template.md, with SD-38's live rendering as the pattern
date: 2026-11-01
status: planning-ready
---

# SD-39 Workflow Instruction — UI and Print Corrections

> ## OPERATING METHOD — REQUIRED FOR THIS BUNDLE
>
> **This bundle runs as a `Workflow`-tool script, invoked from a live session. It is NOT run with
> `/loop /batch` and it is NOT a one-shot task.** One long run with a recorded run id
> (`decisions.md §16.2`). §2.4 says how to write the script. `scope-draft.md` is the *what*; this file
> is the *how*. The orchestrating session never executes RED→GREEN work itself (§2.2).
>
> > ### 🟡 UNATTENDED MODE — the operator launched and walked away
> >
> > Operator, 2026-11-01: "You will be operating in unattended mode". The standing
> > autonomous rules apply (`decisions.md §16`): take the safe default and log it; never call
> > `AskUserQuestion`; clear blockers first and escalate only the card that needs a ruling; report after
> > each epic in `progress.md`; never open the PR while any card is open. **E9.4 is the one planned
> > wait**: the operator prints three sheets and signs the receipt.

## 1. Pre-launch checklist (C1 runs each and pastes the output in its receipt)

`technical-requirements.md §1` P-1…P-9. Items still open at authoring: the branch (P-1, closes at C1),
the notes pointer (P-9, closes at C1), `typst` reachability (P-4, checked at E8.1 start).

## 2. Orchestration mode

- **Dispatch:** the in-harness `Workflow` tool, one long run. `epic-breakdown.md §0` is the
  concurrency and tiering source of truth.
- **Tiering** (`decisions.md §15`): `opus` for engine, UI, verify and every merge check; `sonnet` for
  well-specified work and every long wait; `haiku` for housekeeping. Merge checks never downgraded.
- **Concurrency:** one serial main lane plus three named side lanes (`ui-a`, `sf-cpr`, `ingest`).
  **At most 3 cargo lanes.**

### 2.1 Agent environment — the shared dispatch prefix

Every `agent()` prompt starts with this prefix, **byte-identical** across agents. Write it to
`artifacts/cycle_0/sd39-workflow.js` at launch.

```text
ENVIRONMENT (SD-39, binding — every rule here applies to you):
- export PATH="$HOME/.cargo/bin:$PATH"
- export RETRO_ACTOR=sd39-<card-id>          # the ONLY value you bind, e.g. RETRO_ACTOR=sd39-e3.2
- ONE WRITER PER TREE. The main lane works in /home/ubuntu/workspace/repos/codex/.worktrees/sd39-main
  (branch tranche-19). A side lane makes its OWN tree first, from the pushed branch:
      git -C /home/ubuntu/workspace/repos/codex fetch origin tranche-19
      git -C /home/ubuntu/workspace/repos/codex worktree add -b sd39/<lane> \
          /home/ubuntu/workspace/repos/codex/.worktrees/sd39-<lane> origin/tranche-19
  Start EVERY command with `cd <your tree> &&` (the shell's directory resets). Never write in
  /home/ubuntu/workspace/repos/codex itself (the lead's checkout).
- export CARGO_TARGET_DIR=/home/ubuntu/cargo-target/sd39-<lane>    # one per lane per source tree;
  never under /tmp; never shared across trees; the main lane uses /home/ubuntu/cargo-target/sd39-main.
  Claim it: mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"
- DISK FIRST: `df -h /` before any build; a full sweep needs ~24 G free; if less, stop and report.
- AT MOST 3 CARGO LANES at once across the bundle. Before a full sweep: `pgrep -f '[c]argo'`.
- MEMORY GUARD: one cargo process at a time in your lane; -j 8; `-- --test-threads=8`; `free -g`
  before a run expected > 10 min.
- NEVER `git stash`. NEVER `git add -A` (stage by explicit path). NEVER force-push.
  `git status --porcelain` before EVERY write; if it lists a file you did not modify, stop and report.
- TWO CARGO WORKSPACES: the repo root and apps/desktop/src-tauri are separate; a change that reaches
  both is verified in both.
- CLIPPY BEFORE THE FIRST COMMIT of any card that touches a .rs file:
  `cargo clippy --locked --tests -j 8` in each touched workspace (R16).
- LONG RUNS: anything that may run > 10 minutes: write a not-run skeleton results file FIRST, then
  `nohup <cmd> > <log> 2>&1 & echo $! > <pidfile>` and poll inside this turn.
- FRONTEND: in apps/desktop run `npm run typecheck && npm test && npm run ui-smoke:doc` for any
  frontend or spec.json change; `npm run ui-smoke` for a card that adds a ui-smoke row, always with
  `export RUN_DESKTOP_AGENT=sd39-<card-id>` first.
- `decisions.md §25` WINS over older text in this package. Reuse before you build: read the existing code your card names first.
- A test filter that matches 0 tests is a FAIL. Run each filter in the cargo workspace that holds the test.
- WORKTREES: a side lane removes its own worktree, its local sd39/<lane> branch and its CARGO_TARGET_DIR when its work is pushed.
- SEED OUTPUTS BEFORE AND AFTER (G-1): mkdir -p every *_OUT dir first. PF deltas are listed path by
  path and classed additive-field | intended-value-change; SF and CPR deltas are defects outside
  E3.5 / E8.3. Never re-baseline silently.
- AUDITS: the identifier audit and the four-check wired-integration audit (§6 step 5) on every diff,
  on added lines, after the local commit and before the push.
- TDD: write the failing test, run it, QUOTE the failing reason in the receipt, then make it pass.
- RECEIPTS: every figure carries the command that produced it and its denominator; counts with awk or
  Python, never `grep -o`; two implementations when a figure moves a baseline.
- RETRO: emit scripts/retro.py events (correction/incident/deferral/rework) at the moment they happen,
  with RETRO_ACTOR set; a correction needs --verified-by.
- FILES YOU OWN IS CLOSED (R15): before your first write, grep boundary/*.ts wire() for every request
  field you add and list every second call site; a file you need outside the list is a fence exit named
  in the receipt (decisions.md §23), never a silent edit.
- GLOSSARY (package = docs/release/SD-39-ui-print-corrections/): "D<n>" = scope-draft.md §1 defect;
  "R<n>" in decisions.md §3 = grill ruling; "R<n>" in this file §12 = standing rule; "SD-x" =
  decisions.md §16.1; "G-n" = acceptance-and-verification.md; "F-n" = content-unit-inventory.md;
  "DEF-n" / "FSR-Cn" = forward-scope-register.md. In a table cell `\|` is Markdown for `|`.
- PAPER-SHEET RULE: compute only values that land on the sheet as numbers; print everything else; dice
  stay literal. No TSX and no Typst template re-derives a number.
- NO PCGEN IN LIVE CODE: src/ and apps/desktop/** parse no .lst/.pcg; the .pcg reader is in
  crates/codex-ingest and ships as a sidecar binary.
- AUTONOMOUS: no AskUserQuestion/clarify. If only an operator ruling can clear your blocker, return
  status blocked-escalated with the exact ruling needed.
- If your premise is refuted, return `declined` and name the mechanism. Do not commit a success.
- You get exactly ONE turn. Wait for slow work inside it. Commit and push before ending the turn,
  even for a partial result.
- First command of the cycle (wrong-base control):
  test -d docs && test -d data && test -d scripts && test -f docs/release/SD-39-ui-print-corrections/kanban.md \
    && [ "$(git rev-parse --show-toplevel)" != /home/ubuntu/workspace/repos/codex ] || { echo 'WRONG BASE'; exit 1; }
```

### 2.2 Execution boundary

The session that launches SD-39 is the orchestrator. §6 runs **inside** dispatched `agent()` calls,
never as the orchestrator's own `Edit`/`Write`/`Bash` on `apps/`, `src/`, `crates/`, `scripts/`,
`tests/` or `data/`. The orchestrator may make read-only investigation calls, edit this package's
planning docs, and run git plumbing for them. **Dispatch first, report second.** Never accept a lane's
message as evidence: check `git log` and the target files. Before every dispatch run
`git reflog --date=iso -3` and `git worktree list` (a peer session may hold the checkout).

### 2.3 Retrospective event logging

`scripts/retro.py correction --subject … --claimed … --actual … --verified-by …`; `incident`,
`deferral`, `rework` per `python3 scripts/retro.py help <type>`. `RETRO_ACTOR` is set by the prefix.

### 2.4 Creating the Workflow script

Load the `workflow-authoring` skill first. Plain JavaScript:

1. `export const meta = { name: 'sd39-ui-print-corrections', description, phases }` with one phase per
   §3 row, same titles.
2. **A script has no file access**, so the card list is in the script and each dispatched agent reads `kanban.md`: it returns `declined` if a `Depends on` is not `complete`. Each agent returns `{status, sha, receipt, discovered}`; `discovered` card IDs run before that epic's merge check (R8).
3. `pipeline()` in the main lane; `parallel()` only for the three side lanes, each making its own
   worktree per the prefix (never the harness's isolation: its base is the session checkout).
4. Cargo lanes stay ≤ 3 by construction: the main lane, `ingest`, and at most one other side lane.
5. **Every `agent()` sets `model`** from the Tier column, on the same line as `agent(`. Check:
   `awk '/agent\(/ && !/model:/' <script>` prints nothing. Tier is exactly `opus`, `sonnet` or `haiku`;
   C0 (`lead`) is never dispatched; E9.9 is two calls (`sonnet` truth-up, then `opus` critic); E9.4 is
   dispatched on `sonnet` to prepare the PDFs and returns `partial` until the operator's line exists.
6. Every prompt = §2.1 prefix + §6 verbatim + the card's `epic-breakdown.md` row + its `kanban.md` row.
7. **Quota gate** between steps (`decisions.md §16.3`).
8. **Refuted premise stops the step:** an `ok:false` or `declined` result never reads as success.
9. Write the run id and script path into `progress.md ## Run handle` the moment the run starts.
10. **Never resume** a run with parallel lanes; start a new run with `START=<card>` (R9).
11. **E9.4 wait:** when E9.4 returns `partial`, the script ends the run cleanly with a resume receipt;
    the lead restarts with `START=E9.4` after the operator signs.

```javascript
export const meta = {
  name: 'sd39-ui-print-corrections',
  description: 'SD-39 — guards, layout, Market, slots, ledger, overrides, .pcg import, print, closure',
  phases: [
    { title: 'C — package + cut' }, { title: 'E1 — guards' }, { title: 'E2 — layout' },
    { title: 'E3 — Market' }, { title: 'E4 — slots' }, { title: 'E5 — ledger' },
    { title: 'E6 — overrides' }, { title: 'E7 — import' }, { title: 'E8 — print' }, { title: 'E9 — closure' },
  ],
}
// tier and cargo come from epic-breakdown.md §0; deps from kanban.md (R8). model on the agent( line.
const run = (card) => agent(PREFIX + PROCEDURE + criterion(card), { model: card.tier, phase: card.phase })
const go = async (card) => { quotaGate(); return run(card) }

phase('C — package + cut');  await go(C1)
phase('E1 — guards');        await go(E1_0)
const uiA = go(E1_4)                                            // side lane ui-a (CustomDialog.tsx + customChip.ts)
for (const c of [E1_1, E1_2, E1_3]) await go(c); await uiA; await go(E1_MC)
phase('E2 — layout');        for (const c of E2) await go(c)    // serial, then E2.MC
phase('E3 — Market');        for (const c of [E3_1, E3_2, E3_3]) await go(c)
await Promise.all([go(E3_4), go(E3_5)]); await go(E3_MC)         // sf-cpr side lane beside E3.4
const ingest = (async () => { await go(E7_1) })()               // side lane ingest, from E1.0 on
phase('E4 — slots');         for (const c of E4) await go(c)
phase('E5 — ledger');        for (const c of E5) await go(c)
phase('E6 — overrides');     for (const c of E6) await go(c)
phase('E7 — import');        await ingest; await go(E7_2); await go(E7_MC)
phase('E8 — print');         for (const c of E8) await go(c)
phase('E9 — closure');       for (const c of E9) { const r = await go(c); if (c.id === 'E9.4' && r.status !== 'complete') return resumeReceipt('E9.4') }
```

### 2.5 A dispatched agent is never resumed

One turn; wait for slow work inside it; commit and push before ending even for a partial result.

## 3. Per-epic parallel/sequential map

Phase titles above match these rows. Shared-file writer order: `epic-breakdown.md §0`.

| Phase / epic | Cards | Parallel? | File-touch set | Gated on |
|---|---|---|---|---|
| C — package + cut | C0, C1 | no | this package; the 14 F-1 surfaces + `current-release.json` | — |
| E1 — guards | E1.0–E1.6, E1.MC | E1.4 ∥ E1.1–E1.3 (`ui-a`) | `crates/codex-ingest/**`, `data/sheet_rules/**`, `data/equipment_types.json`; `src/rules_core/feat_budget.rs` *(new)*; `character_hub.rs`; `CharacterSheet.tsx`; `CustomDialog.tsx`, `customChip.ts` (`ui-a` only); `settings/**` | C1 |
| E2 — layout | E2.1–E2.4, E2.MC | no | `sheetLayout.ts`, `CharacterSheet.tsx`, `ProgressionDrawer.tsx` *(new)*, `spec.json`, inventory | E1.MC |
| E3 — Market | E3.1–E3.5, E3.MC | E3.5 ∥ E3.4 (`sf-cpr`) | `equipment_catalog.rs`, `src/rules_core/enhancement.rs` *(new)*, `character_hub.rs`, `main.rs`, `MarketDialog.tsx` *(new)*, `CharacterSheet.tsx`, `boundary/*.ts`, `spec.json`; `sf-cpr`: the SF/CPR sheet files and adapters only | E2.MC |
| E4 — slots | E4.1–E4.4, E4.MC | no | `src/rules_core/body_slots.rs` *(new)*, `character_input.rs`, the F-11 consumers, `character_hub.rs`, `main.rs`, `local_store.rs`, `src/saved_character/mod.rs`, `EquipmentPanel.tsx` *(new)*, `CharacterSheet.tsx`, `character_custom.rs`, `CustomTab.tsx`, `spec.json` | E3.MC |
| E5 — ledger | E5.1–E5.3, E5.MC | no | `src/saved_character/{mod,migrate_ledger}.rs`, `local_store.rs`, `character_hub.rs`, `main.rs`, `ProgressionDrawer.tsx`, `characterProgression.ts`, `boundary/*.ts` | E4.MC |
| E6 — overrides | E6.1, E6.2, E6.MC | no | `character_custom.rs`, `CustomTab.tsx`, `character_hub.rs`, `equipment_resolver.rs`, spell add path, `OverrideTag.tsx` *(new)*, selection DTOs, `spec.json` | E5.MC |
| E7 — import | E7.1 (`ingest` lane from E1.0), E7.2, E7.MC | E7.1 ∥ E4–E6 | `crates/codex-ingest/src/pcg_import/**` *(new)*, `bin/pcg_import.rs` *(new)*; E7.2: `tauri.conf.json`, `Cargo.toml`, `character_hub.rs`, `main.rs`, Import UI, `packaged_resources.rs`, `boundary/importPcg.ts` *(new)* | E1.0 (E7.1); E6.MC + E7.1 (E7.2) |
| E8 — print | E8.1–E8.5, E8.MC | no | `apps/desktop/src-tauri/src/print/**` *(new)*, `resources/{fonts,print}/**` *(new)*, `Cargo.toml`, `main.rs`, `settings/**`, `PrintDialog.tsx` *(new)*, the three sheet files, delete `printLayout.ts`+test, `boundary/*.ts`, `spec.json`, `tests/print_golden.rs` *(new)* | E7.MC |
| E9 — closure | E9.1–E9.11 | no | `tauri.conf.json`, `packaged_resources.rs`, `scripts/verify.sh`, `scripts/verify-baselines.env`, `docs/retro/sd39-retrospective.md`, `docs/architecture/**`, graph outputs, this folder | E8.MC |

**Fences.** A lane that needs a file outside its row returns `declined` with `ownedBy:<card>` unless
that file's epic merge check is `complete` (`decisions.md §23`).

## 4. File-touch verification

Each existing path named in §3, `technical-design.md` and `content-unit-inventory.md` was checked on
2026-11-01 with `ls`/`grep -n` on `develop` @ `5ef2f17b67`; line numbers are quoted in
`technical-design.md`. Paths marked *(new)* are design choices the owning card confirms and names in its
receipt.

## 5. Concurrent-write protocol

```bash
git fetch origin tranche-19 && git rebase origin/tranche-19 && git push origin HEAD:tranche-19
```

Retry up to 5 times on a non-fast-forward; then `CLAIM-EXISTS`, card `blocked-escalated`. Never
force-push. Re-fetch and re-read `progress.md`/`kanban.md` immediately before every edit. Rebase
conflict: SD-k. Every card lands by pushing its own commits to `tranche-19`; merge-check cards re-verify
on `origin/tranche-19` and merge nothing.

## 6. Per-cycle procedure (runs inside the dispatched agent)

**Environment rules, binding in every step:** the §2.1 prefix verbatim.

1. **Base check** (a control): the prefix's wrong-base test. Then §5's fetch+rebase; record
   `CARD_BASE=$(git rev-parse HEAD)`.
2. **Disk and load:** `df -h /`; `pgrep -f '[c]argo'`; claim `CARGO_TARGET_DIR`.
3. **Baseline:** run the card's acceptance command on the pre-change tree and keep its output (RED);
   render the seeds (G-1) into `<dir>/base`.
4. **TDD:** write the failing test; run it; quote the failing reason; make the smallest change; run the
   targeted suites. **Clippy before the first commit** for any `.rs` change (R16).
5. **Commit locally** (explicit paths), then run the **dual audit on added lines**:
   ```bash
   git diff --unified=0 "${CARD_BASE}...HEAD" -- src crates apps scripts tests ':!**/__tests__/**' ':!**/*.test.*' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
   git diff --unified=0 "${CARD_BASE}...HEAD" -- 'apps/desktop/**/*.ts*' 'apps/desktop/src-tauri/**/*.rs' 'src/**/*.rs' 'crates/**/*.rs' ':!**/__tests__/**' ':!**/*.test.ts' ':!**/*.test.rs' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo OK_NO_TOKENS
   git diff --unified=0 "${CARD_BASE}...HEAD" -- 'apps/desktop/**/*.tsx' 'apps/desktop/**/*.jsx' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE 'onClick=\{\s*\(\)\s*=>\s*\{\s*\}\s*\}|onClick=\{undefined' || echo OK_NO_NOOP_HANDLERS
   git diff --unified=0 "${CARD_BASE}...HEAD" -- 'apps/desktop/**/*.ts' 'apps/desktop/**/*.tsx' 'apps/desktop/**/*.jsx' 'apps/desktop/**/*.rs' ':!**/__tests__/**' ':!**/*.test.*' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE 'mockResolvedValue|mockReturnValue\(|vi\.mock\(|__mocks__' || echo OK_NO_MOCK_LEAKS
   git diff --unified=0 "${CARD_BASE}...HEAD" -- 'apps/desktop/**/*.ts' 'apps/desktop/**/*.tsx' 'src/**/*.rs' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE '"Would [^"]*"' || echo OK_NO_WOULD_STRINGS
   ```
   A violation is fixed by a new commit and the audit re-run. A real stub is non-self-healable.
6. **Clippy stage and the audit test** in every card whose diff touches a `.rs` file (R11):
   `bash scripts/verify.sh --only clippy -j 8` and root
   `cargo test --locked -j 8 --test sd24_wired_integration_audit -- --test-threads=8`.
7. **Seed deltas** (G-1, R7): render into `<dir>/head`; diff against `base`; list every PF path
   changed with its class; SF/CPR must be empty outside E3.5/E8.3.
8. **Verify once at the widest scope the change reaches:** root and/or desktop `cargo test --locked -j 8
   --no-fail-fast -- --test-threads=8`, plus `cd apps/desktop && npm run typecheck && npm test && npm run
   ui-smoke:doc` when frontend or `spec.json` changed. Attribute every `test result: FAILED` to its
   `Running` line. Do not repeat a clean gate (R-V).
9. **Receipt** to `artifacts/epic_<n>/<card>_cycle_receipt.md` (§7). Update the card's `kanban.md` row
   and append a `progress.md` row **in the same commit**. If you added a discovered card, re-count the
   card-count constant in `acceptance-and-verification.md` G-12 and `kanban.md` in that commit (R18).
   Unfiltered `git status --porcelain` prints nothing after it. Push via §5.
10. **Report:** status (`complete | partial | blocked-escalated`), SHA, receipt path, seed deltas,
    discoveries.

## 7. Per-cycle receipt schema

```markdown
# Cycle <card-id> — <epic> / <criterion title>

- **Card ID:** <card-id>   **Model:** <opus|sonnet|haiku>   **RETRO_ACTOR:** <value>
- **Commit SHA:** <sha>    **Base SHA:** <tranche-19 sha at start>
- **Files touched:** <list>   (every one inside FILES YOU OWN, or a named fence exit)
- **FILES YOU OWN closure check:** <boundary wire() fields grepped; second call sites listed>
- **Acceptance criterion (verbatim from epic-breakdown.md):** <text>
- **RED (pre-change command + output, failing reason quoted):** <…>
- **GREEN (same command + output):** <…>
- **Figures:** <value> — <predicate/denominator> — <command A> → <output>; <command B> → <output>
- **Build scope verified:** <root | apps/desktop/src-tauri | frontend> + log path
- **Clippy before first commit:** <result>   **Clippy stage + sd24 audit test:** <result>
- **Identifier audit:** OK_NO_BUNDLE_TAGS | <violations>
- **Wired-integration audit:** OK_NO_TOKENS / OK_NO_NOOP_HANDLERS / OK_NO_MOCK_LEAKS / OK_NO_WOULD_STRINGS | <violations>
- **Seed deltas:** PF Aldric / Elowen: <none | path: old → new (additive-field | intended-value-change) …>; SF ×4: <none | listed (E3.5/E8.3 only)>; CPR ×3: <same>
- **Rulings applied:** <R<n> / SD-x + alternative not taken>
- **Does not cover:** <the shapes this proof does not exercise>
- **Status:** complete | partial | blocked-escalated
- **Remainder (partial only):** <every remaining item by sub-cause; populations sum to the total>
- **Retro events emitted:** <types + counts>
- **Next-cycle plan:** <…>
```

## 8. Self-heal posture

- **Self-healable:** dirty tree from your own lane; a single-token audit violation; unrelated test-setup
  breakage; a count two implementations re-derive (SD-c); a missing `*_OUT` dir (`mkdir -p`).
- **Not self-healable (`blocked-escalated` on THIS card; continue independent cards):** two live lanes
  on one file; RED→GREEN not preserved; a stub, mock or "Would …" string in shipping code; a converter
  diff wider than the three keys (G-2); `typst` unbuildable (SD-h); a fresh-install check the box cannot
  run (SD-l); `CLAIM-EXISTS`.
- **Blocker doctrine:** cleared or escalated, never deferred. The test: was this scope in the Definition
  of Done at scoping? If yes, it is a blocker.
- **Disk:** after every side lane ends, `df -h /` and `git worktree list`; prune merged, unlocked, clean
  `sd39-*` worktrees and delete their `CARGO_TARGET_DIR`s. Never remove a `locked` tree.

## 9. Placeholder-resolution checklist

```bash
grep -rn '<[a-z_-]*>' docs/release/SD-39-ui-print-corrections/*.md
```

Every match must be (a) a schema placeholder inside a fenced template or a command argument the running
card binds (`<sha>`, `<c1-sha>`, `<base>`, `<head>`, `<dir>`, `<lane>`, `<card>`, `<card-id>`, `<n>`,
`<epic>`, `<list>`, `<text>`, `<output>`, `<violations>`, `<result>`, `<value>`, `<cmd>`, `<log>`,
`<pidfile>`, `<script>`, `<type>`, `<new>`, `<item>`, `<slot>`, `<flag>`, `<base_id>`, `<name>`,
`<system>`, `<file>`, `<json>`, `<fixture>`, `<E2.4 ids>`, `<new ids>`, `<F-1 globs>`, `<BodySlot>`,
`<kind>`, `<price>`, and the pattern/message placeholders `<pool>`, `<price>`, `<gold>`, `<same>`, `<book>`, `<description>`, `<epic-start>`), (b) a deferred value with a named resolution point (`0.19.<run>` → first tester publish after
the operator merges; `(E1.0)`, `(E2.4)`, `(E7.1)` figures → those cards; the run id → launch), or (c) a
bug to fix before launch. The author's run is recorded in `progress.md`.

## 10. Epic wrap-up (after every epic)

1. `scripts/retro.py summary --since <epic-start> --json`; fold the counts and any key that fired more
   than once into the merge-check receipt; a key firing more than twice gets a non-zero-exit check in
   the next card that touches that area (R-W).
2. The merge-check card writes the **Epic report** block in `progress.md` and updates `kanban.md` in its
   own closing commit.
3. Sweep this epic's merged, unlocked, clean `sd39-*` worktrees and delete their `CARGO_TARGET_DIR`s.
4. No PR.

## 11. Bundle closure epilogue (corrected order — `decisions.md §17`)

1. **E9.5 final-acceptance scan** — the fenced commands in `acceptance-and-verification.md` G-12. Pass =
   no output from the two `awk`s, `usable: yes` ×3, every DEF/FSR revisit command pasted. **If anything
   is short, stop here:** no retrospective, no sweep, **no PR**. That is a correct outcome.
2. **E9.6 retrospective:** `scripts/retro.py summary --since 2026-11-01 --json`; write
   `docs/retro/sd39-retrospective.md` in the shape of `docs/retro/sd38-retrospective.md`; cite it from
   `references/README.md` in the same commit.
3. **E9.7 sweep:** every `sd39-*` worktree and `sd39/*` branch except `sd39-main`, which E9.8–E9.11 still need (the script's last step removes it after the PR); found vs removed counts; never `test`,
   `update-index` or a locked tree; delete `/home/ubuntu/cargo-target/sd39-*`.
4. **E9.8 release notes:** replace the interim body; `grep -c 'SD-39 INTERIM NOTES'` → 0; every figure with
   its command; `python3 tools/release/resolve_release_notes.py --repo-root .` passes.
5. **E9.9 architecture truth-up**
   (`~/.hermes/profiles/god-emporer/skills/devops/architecture-truth-up/scripts/architecture_truth_up.py --integration-target develop --receipts-md docs/release/SD-39-ui-print-corrections/receipts.md --bundle SD-39`),
   then an Opus claims critic.
6. **E9.10 graphify LAST.** Precondition: unfiltered `git status --porcelain` prints nothing and
   `git rev-parse HEAD` = `git rev-parse origin/tranche-19`. Run
   `~/.hermes/profiles/god-emporer/skills/devops/graphify-update/scripts/update_graphify.py --integration-target develop --receipts-md docs/release/SD-39-ui-print-corrections/receipts.md --bundle SD-39`.
   On the node-count guard (exit 1): file the receipt and stop. **Never** a force flag.
7. **E9.11 PR** `tranche-19 → develop`, the final action: re-run step 1 with the exemption narrowed to
   `E9\.11`; the sentinel grep → 0; commit E9.11's own `complete` row; `gh pr create`; wait inside the
   turn for `gh pr checks <n> --watch`. Red or unopenable → one docs commit setting E9.11
   `blocked-escalated` with the failing job named. **The operator merges.**
8. Stop.

## 12. Standing rules

### 12.1 Carried from SD-36/SD-37 (R1–R13, `../SD-38-cyberpunk-red/workflow-instruction.md §12`)

| Rule | Statement | Checked by |
|---|---|---|
| R1 Converter lane in the prefix | Only if E1.0's proof fails (`decisions.md §25`); `--check` in G-1 | E1.0 receipt; G-1 |
| R2 No hand-kept desktop tables | Budget pools, prices and slots come from the catalog/data, never a TSX table | E1.MC, E3.MC, E4.MC |
| R3 Merge checks stay on Opus | every `*.MC` is `opus` | `awk -F'\|' '$2 ~ /MC/ && $4 !~ /opus/' kanban.md` prints nothing |
| R4 `ownedBy` declines | a declined item naming a later card goes to that card | script review |
| R5 Isolated app-data root | any app-launching harness uses a per-run `XDG_DATA_HOME` | E2.4, E3.MC, E4.MC, E9.2 receipts |
| R6 Long waits on Sonnet+ | no Haiku on runs that may exceed 20 min | Tier of E8.5, E9.2, E9.3, E9.10, E9.11 ≠ haiku |
| R7 Per-character status | every receipt reports PF/SF/CPR seed deltas | `grep -L 'Seed deltas' artifacts/epic_*/*_cycle_receipt.md` prints nothing |
| R8 The script reads the board | dispatch only when every `Depends on` is `complete` | script review |
| R9 Never resume a parallel script | new run with `START=<card>` | `progress.md` run handle |
| R10 Fences have an exit | `decisions.md §23` | receipts |
| R11 Clippy + audit test every code card | §6 step 6 | receipts |
| R12 Generated artifacts re-run-and-diff | `ui-smoke:doc` regenerated and committed by every `spec.json` writer | `git status --porcelain docs/testing/ui-smoke-inventory.md` empty |
| R13 Re-derive copied figures | every figure with its command | receipts |

### 12.2 SD-38 retrospective changes (R14–R19, `docs/retro/sd38-retrospective.md` "Changes for SD-39")

| Rule | Statement | Checked by |
|---|---|---|
| **R14 Out-dir is a control** | every G-1 command carries `mkdir -p`; if `sf-seed-lines-out-dir-missing` fires once in SD-39, E8.3 makes the writer tests create their dir | `grep -rl 'sf-seed-lines-out-dir-missing' docs/retro/events` at E8.3 |
| **R15 FILES YOU OWN is closed** | each card greps `boundary/*.ts wire()` and second call sites before its first write; the receipt has the closure-check row | `grep -L 'closure check' artifacts/epic_*/*_cycle_receipt.md` prints nothing |
| **R16 Clippy before the first commit** | `cargo clippy --locked --tests` on touched workspaces before any `.rs` card's first local commit | receipt row "Clippy before first commit" |
| **R17 Full ui-smoke has an owner** | E2.4 runs the full suite once and records the red-row list as FSR-C2 evidence; the full suite is **not** a gate in SD-39 (stated here, per the retro's wording) | E2.4 receipt |
| **R18 Re-count at discovery** | the card that adds a discovered row edits the card-count constant in G-12 and this file in the same commit | E9.5's `ROW_COUNT` check never fires |
| **R20 Zero tests is a fail** | a filter matching 0 tests proves nothing; receipts quote the `N passed` line | receipts |
| **R19 Observed concurrency, not stated** | before dispatching a side lane, `git status --porcelain` and `git worktree list` are read; no two cards share a tree on "depends only on" reasoning | the lead's dispatch log in `progress.md` |

### 12.3 Per-cycle and measurement rules

**R-T TDD**, failing reason quoted. **R-B Batch big:** homogeneous remainders in one dispatch (one
converter run; one golden suite). **R-V One verify per wave.** **R-C Clean tree** at every commit.
**R-N** every figure with its command and denominator; awk or Python, never `grep -o`. **R-P** every
proof states what it does not cover. **R-W** a recurring incident key gets a control, not a warning.
**R-G Widest build scope:** both cargo workspaces. **R-X Cross-tree renders:** base renders build in
their own worktree with their own `CARGO_TARGET_DIR`. **R-Q Quota:** `decisions.md §16.3`.

## Cross-references

- `docs/governance/workflow-instruction-template.md` (source; its §11 order is defective),
  `docs/release/template/template.md`.
- `docs/governance/blocker-closure-doctrine.md`, `docs/governance/deferral-revisit-doctrine.md`,
  `docs/governance/no-stub-mvp-doctrine.md`.
- `docs/retro/sd38-retrospective.md` (R14–R19), `../SD-38-cyberpunk-red/workflow-instruction.md` (R1–R13).
