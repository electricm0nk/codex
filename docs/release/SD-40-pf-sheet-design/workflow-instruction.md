---
canonical: true
owner: operator
bundle_id: SD-40
slug: pf-sheet-design
canonical_branch: tranche-20
target_version: 0.20.0
authored_from: docs/governance/workflow-instruction-template.md, with SD-39's live rendering as the pattern
date: 2026-11-07
status: planning-ready
---

# SD-40 Workflow Instruction — Pathfinder Sheet Design

> ## OPERATING METHOD — REQUIRED FOR THIS BUNDLE
>
> **This bundle runs as a `Workflow`-tool script, invoked from a live session. It is NOT run with
> `/loop /batch` and it is NOT a one-shot task.** One long run with a recorded run id
> (`decisions.md §13.2`). §2.4 says how to write the script. `scope-draft.md` is the *what*; this file
> is the *how*. The orchestrating session never executes RED→GREEN work itself (§2.2).
>
> > ### 🟡 UNATTENDED MODE — after launch, the run stops only at the two operator gates
> >
> > Operator, 2026-11-07: "stop only after step 1, then run the rest unattended". The standing
> > autonomous rules apply (`decisions.md §13`): take the safe default (§13.1) and log it; never call
> > `AskUserQuestion`; clear blockers first and escalate only the card that needs a ruling; report after
> > each epic in `progress.md`; never open the PR while any card is open. **E2.G and E5.2 are the two
> > planned waits.**

## 1. Pre-launch checklist (C1 runs P-1, P-2, P-5, P-8 and pastes the output in its receipt)

`technical-requirements.md §1` P-1…P-9. Items still open at authoring: PCGen headless (P-3, P-4, E0.1),
the golden baseline (P-9, E1.1).

## 2. Orchestration mode

- **Dispatch:** the in-harness `Workflow` tool, one long run. `epic-breakdown.md §0` is the
  concurrency and tiering source of truth.
- **Tiering** (`decisions.md §12`): `opus` for looks, layouts, the model and every merge check; `sonnet`
  for the harness, the two gate cards and every long wait; `haiku` for C1. Merge checks never downgraded.
- **Concurrency:** one serial main lane plus two side lanes (`ref`, E0.1, no cargo; `smoke`, E0.2, the
  app build for ui-smoke, landing before E4.2). **At most 3 cargo lanes** (the main lane; a merge check's
  base build; the `smoke` lane; nothing else).

### 2.1 Agent environment — the shared dispatch prefix

Every `agent()` prompt starts with this prefix, **byte-identical** across agents. Write it to
`artifacts/cycle_0/sd40-workflow.js` at launch.

```text
ENVIRONMENT (SD-40, binding — every rule here applies to you):
- export PATH="$HOME/.cargo/bin:$PATH"
- export RETRO_ACTOR=sd40-<card-id>          # the ONLY value you bind, e.g. RETRO_ACTOR=sd40-e2.1
- ONE WRITER PER TREE. The main lane works in /home/ubuntu/workspace/repos/codex/.worktrees/sd40-main
  (branch tranche-20). Each side lane (`ref`, `smoke`) makes its OWN tree first, from the pushed branch:
      git -C /home/ubuntu/workspace/repos/codex fetch origin tranche-20
      git -C /home/ubuntu/workspace/repos/codex worktree add -b sd40/<lane> \
          /home/ubuntu/workspace/repos/codex/.worktrees/sd40-<lane> origin/tranche-20
  Start EVERY command with `cd <your tree> &&` (the shell's directory resets). Never write in
  /home/ubuntu/workspace/repos/codex itself (the lead's checkout) and NEVER in
  /home/ubuntu/workspace/repos/codex/.worktrees/sd39-main (another session's tree, decisions.md §20).
- export CARGO_TARGET_DIR=/home/ubuntu/cargo-target/sd40-<lane>    # one per lane per source tree;
  never under /tmp; never shared across trees; the main lane uses /home/ubuntu/cargo-target/sd40-main.
  Claim it: mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"
- DISK FIRST: `df -h /` before any build; a full sweep needs ~24 G free; if less, stop and report.
- AT MOST 3 CARGO LANES at once across the bundle. Before a full sweep: `pgrep -f '[c]argo'`.
- MEMORY GUARD: one cargo process at a time in your lane; -j 8; `-- --test-threads=8`; `free -g`
  before a run expected > 10 min.
- NEVER `git stash`. NEVER `git add -A` (stage by explicit path). NEVER force-push.
  `git status --porcelain` before EVERY write; if it lists a file you did not modify, stop and report.
- TWO CARGO WORKSPACES: the repo root and apps/desktop/src-tauri are separate; the print module is in
  apps/desktop/src-tauri (a binary crate with no lib target: every print test is a #[cfg(test)] module
  of the binary, run with `cargo test --locked -j 8 <filter> -- --test-threads=8` from that directory).
- CLIPPY BEFORE THE FIRST COMMIT of any card that touches a .rs file:
  `cargo clippy --locked --tests -j 8` in each touched workspace (R16).
- LONG RUNS: anything that may run > 10 minutes: write a not-run skeleton results file FIRST, then
  `nohup <cmd> > <log> 2>&1 & echo $! > <pidfile>` and poll inside this turn.
- FRONTEND: in apps/desktop run `npm run typecheck && npm test && npm run ui-smoke:doc` for any
  frontend or spec.json change; `npm run ui-smoke` for a card that adds or repairs a ui-smoke row, always with
  `export RUN_DESKTOP_AGENT=sd40-<card-id>` first.
- `decisions.md §21` WINS over older text in this package. Reuse before you build: read the existing
  print code your card names first (print/model.rs, print/typst.rs, resources/print/*.typ).
- A test filter that matches 0 tests is a FAIL. Run each filter in the cargo workspace that holds the test.
- WORKTREES: each side lane removes its own worktree, its local sd40/<lane> branch and its CARGO_TARGET_DIR
  when its work is pushed.
- SEEDS AND GOLDENS (G-1, G-2): PF seed render hashes are UNCHANGED by every card (diff empty); SF1e and
  CPR golden TEXT and PAGE COUNTS equal C1's (diff empty) except the E4.1 renaming decisions.md §9 names;
  PF golden page counts may move and every move is listed before → after. Never re-baseline silently.
- LOOK AT YOUR OWN OUTPUT (R7, §6 step 8): a look or layout card renders (print_proof), rasterizes
  (scripts/print/rasterize.sh), READS the PNGs with the Read tool beside the reference PNGs under
  artifacts/epic_0/E0.1/, iterates up to three passes, and commits the final PNGs under
  artifacts/epic_<n>/<card>/ (60 dpi, ≤ 12 pages, ≤ 2 MB). A receipt without a "Looked at" row is partial.
- NO COMPUTED NUMBERS IN A LOOK (NR-2): a .typ file formats strings it is given; it never adds, sums or
  derives. No TSX re-derives a number either.
- NO PCGEN IN LIVE CODE (R3): nothing from $PCGEN_REPO_DIR/outputsheets enters apps/, src/, crates/,
  data/, resources/ or scripts/; the reference render lives under this package's artifacts/ only.
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
- GLOSSARY (package = docs/release/SD-40-pf-sheet-design/): "R<n>" in decisions.md §3 = operator
  ruling; "R<n>" in this file §12 = standing rule; "SD-x" = decisions.md §13.1; "G-n" =
  acceptance-and-verification.md; "F-n" = content-unit-inventory.md; "DEF-n" / "FSR-Cn" =
  forward-scope-register.md; "look" = one resources/print/looks/<look>.typ; "preset" = one &[Placed]
  list in print/layout/pf1e.rs. In a table cell `\|` is Markdown for `|`.
- PAPER-SHEET RULE: compute only values that land on the sheet as numbers; print everything else; dice
  stay literal; write-in space is a ruled blank, never a value.
- AUTONOMOUS: no AskUserQuestion/clarify. If only an operator ruling can clear your blocker, return
  status blocked-escalated with the exact ruling needed. A taste question takes decisions.md §13.1's default.
- If your premise is refuted, return `declined` and name the mechanism. Do not commit a success.
- You get exactly ONE turn. Wait for slow work inside it. Commit and push before ending the turn,
  even for a partial result.
- First command of the cycle (wrong-base control):
  test -d docs && test -d data && test -d scripts && test -f docs/release/SD-40-pf-sheet-design/kanban.md \
    && [ "$(git rev-parse --show-toplevel)" != /home/ubuntu/workspace/repos/codex ] \
    && [ "$(git rev-parse --show-toplevel)" != /home/ubuntu/workspace/repos/codex/.worktrees/sd39-main ] \
    || { echo 'WRONG BASE'; exit 1; }
```

### 2.2 Execution boundary

The session that launches SD-40 is the orchestrator. §6 runs **inside** dispatched `agent()` calls,
never as the orchestrator's own `Edit`/`Write`/`Bash` on `apps/`, `src/`, `crates/`, `scripts/`,
`tests/`, `data/` or `resources/`. The orchestrator may make read-only investigation calls, edit this
package's planning docs, and run git plumbing for them (including the SD-o catch-up rebase of
`tranche-20` onto `develop` between epics). **Dispatch first, report second.** Never accept a lane's
message as evidence: check `git log` and the target files. Before every dispatch run
`git reflog --date=iso -3` and `git worktree list` (a peer session holds `sd39-main`).

**Before dispatching any card (R21, R22):** run its acceptance command on the pre-change tree and
require a failure or a non-zero exit (paste it into the dispatch prompt as the expected RED); `git grep`
each producer named in its FILES YOU OWN and add any second call site to the row before dispatch.

### 2.3 Retrospective event logging

`scripts/retro.py correction --subject … --claimed … --actual … --verified-by …`; `incident`,
`deferral`, `rework` per `python3 scripts/retro.py help <type>`. `RETRO_ACTOR` is set by the prefix.

### 2.4 Creating the Workflow script

Load the `workflow-authoring` skill first. Plain JavaScript:

1. `export const meta = { name: 'sd40-pf-sheet-design', description, phases }` with one phase per
   §3 row, same titles.
2. **A script has no file access**, so the card list is in the script and each dispatched agent reads
   `kanban.md`: it returns `declined` if a `Depends on` is not `complete`. Each agent returns
   `{status, sha, receipt, discovered}`; `discovered` card IDs run before that epic's merge check (R8).
3. `pipeline()` in the main lane; `parallel()` only for the `ref` and `smoke` side lanes, each of which makes its own
   worktree per the prefix (never the harness's isolation: its base is the session checkout).
4. Cargo lanes stay ≤ 3 by construction: the main lane, a merge check's base build and the `smoke` lane.
5. **Every `agent()` sets `model`** from the Tier column, on the same line as `agent(`. Check:
   `awk '/agent\(/ && !/model:/' <script>` prints nothing. Tier is exactly `opus`, `sonnet` or `haiku`;
   C0 (`lead`) is never dispatched; E5.4 is two calls (`sonnet` notes + truth-up, then `opus` critic);
   E2.G and E5.2 are dispatched on `sonnet` to prepare the PDFs and return `partial` until the
   operator's lines exist.
6. Every prompt = §2.1 prefix + §6 verbatim + the card's `epic-breakdown.md` row + its `kanban.md`
   row + the lead's pre-dispatch RED output (R21).
7. **Quota gate** between steps (`decisions.md §13.3`).
8. **Refuted premise stops the step:** an `ok:false` or `declined` result never reads as success.
9. Write the run id and script path into `progress.md ## Run handle` the moment the run starts.
10. **Never resume** a run with parallel lanes; start a new run with `START=<card>` (R9).
11. **Gate waits:** when E2.G or E5.2 returns `partial`, the script ends the run cleanly with a resume
    receipt; the lead restarts with `START=E3.1` (after E2.G) or `START=E5.3` (after E5.2) once the
    operator has signed, or with `START=<discovered card>` on defects.

```javascript
export const meta = {
  name: 'sd40-pf-sheet-design',
  description: 'SD-40 — reference picture, layout model + grid, page-1 looks (operator gate), remaining looks, presets, closure (final look)',
  phases: [
    { title: 'C — package + cut' }, { title: 'E0 — reference' }, { title: 'E1 — model + grid' },
    { title: 'E2 — page-1 looks + gate' }, { title: 'E3 — remaining looks' }, { title: 'E4 — presets' },
    { title: 'E5 — closure + final look' },
  ],
}
// tier and cargo come from epic-breakdown.md §0; deps from kanban.md (R8). model on the agent( line.
const run = (card) => agent(PREFIX + PROCEDURE + criterion(card) + red(card), { model: card.tier, phase: card.phase })
const go = async (card) => { quotaGate(); return run(card) }

phase('C — package + cut');          const ref = go(E0_1)                 // side lane ref, no cargo
                                      await go(C1)
                                      const smoke = go(E0_2)               // side lane smoke, beside E1–E4.1
phase('E1 — model + grid');          await go(E1_1); await go(E1_2); await ref; await go(E1_MC)
phase('E2 — page-1 looks + gate');   for (const c of [E2_1, E2_2, E2_3, E2_MC]) await go(c)
                                      { const r = await go(E2_G); if (r.status !== 'complete') return resumeReceipt('E2.G') }
phase('E3 — remaining looks');       for (const c of [E3_1, E3_2, E3_3, E3_MC]) await go(c)
phase('E4 — presets');               await go(E4_1); await smoke; await go(E4_2); await go(E4_MC)   // E0.2 → E4.2 (spec.json)
phase('E5 — closure + final look');  await go(E5_1)
                                      { const r = await go(E5_2); if (r.status !== 'complete') return resumeReceipt('E5.2') }
                                      for (const c of [E5_3, E5_4a, E5_4b, E5_5]) await go(c)
```

### 2.5 A dispatched agent is never resumed

One turn; wait for slow work inside it; commit and push before ending even for a partial result.

## 3. Per-epic parallel/sequential map

Phase titles above match these rows. Shared-file writer order: `epic-breakdown.md §0`.

| Phase / epic | Cards | Parallel? | File-touch set | Gated on |
|---|---|---|---|---|
| C — package + cut | C0, C1 | no | this package; the 14 F-1 surfaces + `current-release.json` | — |
| E0 — reference | E0.1, E0.2 | **yes** (`ref` beside C1 and E1.1; `smoke` beside E1.1–E4.1) | E0.1: `artifacts/epic_0/E0.1/**`, its F-12–F-14 and `decisions.md §10` figure rows; E0.2: `apps/desktop/scripts/ui-smoke/spec.json` (the 14 rows), `docs/testing/ui-smoke-inventory.md`, `artifacts/epic_0/E0.2/**` and the harness or `data-testid` files its FILES YOU OWN allows; each its own `kanban.md`/`progress.md` rows | C0 (E0.1); C1 (E0.2); **E4.2 waits on E0.2** |
| E1 — model + grid | E1.1, E1.2, E1.MC | no | `print/{model,typst,mod}.rs`, `print/blocks/{mod,core}.rs`, `print/layout/{mod,pf1e}.rs`, `resources/print/*.typ`, `looks/table.typ` *(new)*, `print/golden/*.json`, the print test modules, `sheetLayout.test.ts`, `print/print_proof.rs` *(new)*, `scripts/print/rasterize.sh` *(new)* | C1 (E1.1); E0.1 (E1.MC) |
| E2 — page-1 looks + gate | E2.1–E2.3, E2.MC, E2.G | no | `print/blocks/{core,skills,gear}.rs`, `print/layout/pf1e.rs`, `print/model.rs`, `print/typst.rs`, `looks/*.typ` *(new)*, `print/golden/*.json`, the print test modules, `artifacts/epic_2/**` | E1.MC |
| E3 — remaining looks | E3.1–E3.3, E3.MC | no | `print/blocks/{feats,gear,spells,bio_notes,math,extras,core}.rs`, `print/layout/pf1e.rs`, `print/model.rs`, `print/typst.rs`, `looks/*.typ`, `print/golden/*.json`, the print test modules, `artifacts/epic_3/**` | E2.G |
| E4 — presets | E4.1, E4.2, E4.MC | no | `print/mod.rs`, `print/layout/{mod,pf1e}.rs`, `print/print_golden.rs`, `print/golden/print_golden.json`, `print/print_proof.rs`, `PrintDialog.tsx` + test, `boundary/saveSheetPdf.ts` + test, `spec.json`, `docs/testing/ui-smoke-inventory.md` | E3.MC; E0.2 (before E4.2) |
| E5 — closure + final look | E5.1–E5.5 | no | `tests/packaged_resources.rs`, `scripts/verify-baselines.env`, `docs/retro/sd40-retrospective.md`, `docs/architecture/**`, graph outputs, this folder, `artifacts/epic_5/**` | E4.MC |

**Fences.** A lane that needs a file outside its row returns `declined` with `ownedBy:<card>` unless
that file's epic merge check is `complete` (`decisions.md §19`).

## 4. File-touch verification

Each existing path named in §3, `technical-design.md` and `content-unit-inventory.md` was checked on
2026-11-07 with `ls`/`grep -n` on `tranche-19` @ `d4489b3557`; line counts are quoted in
`technical-design.md §0`. Paths marked *(new)* are design choices the owning card confirms and names in
its receipt.

## 5. Concurrent-write protocol

```bash
git fetch origin tranche-20 && git rebase origin/tranche-20 && git push origin HEAD:tranche-20
```

Retry up to 5 times on a non-fast-forward; then `CLAIM-EXISTS`, card `blocked-escalated`. Never
force-push. Re-fetch and re-read `progress.md`/`kanban.md` immediately before every edit. Rebase
conflict: SD-k. Every card lands by pushing its own commits to `tranche-20`; merge-check cards re-verify
on `origin/tranche-20` and merge nothing.

## 6. Per-cycle procedure (runs inside the dispatched agent)

**Environment rules, binding in every step:** the §2.1 prefix verbatim.

1. **Base check** (a control): the prefix's wrong-base test. Then §5's fetch+rebase; record
   `CARD_BASE=$(git rev-parse HEAD)`.
2. **Disk and load:** `df -h /`; `pgrep -f '[c]argo'`; claim `CARGO_TARGET_DIR`.
3. **Baseline:** run the card's acceptance command on the pre-change tree and keep its output (RED; it
   must match the lead's R21 output in the prompt); render the PF seeds (G-1) and the SF/CPR goldens'
   text (G-2) into `<dir>/base` when the card touches `print/**` or `resources/print/**`.
4. **TDD:** write the failing test; run it; quote the failing reason; make the smallest change; run the
   targeted suites. **Clippy before the first commit** for any `.rs` change (R16).
5. **Commit locally** (explicit paths), then run the **dual audit on added lines**:
   ```bash
   git diff --unified=0 "${CARD_BASE}...HEAD" -- src crates apps scripts tests ':!**/__tests__/**' ':!**/*.test.*' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
   git diff --unified=0 "${CARD_BASE}...HEAD" -- 'apps/desktop/**/*.ts*' 'apps/desktop/src-tauri/**/*.rs' 'apps/desktop/src-tauri/resources/print/**' 'src/**/*.rs' 'crates/**/*.rs' ':!**/__tests__/**' ':!**/*.test.ts' ':!**/*.test.rs' \
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
7. **Seeds and goldens** (G-1, G-2, R7): PF seed hashes `diff` empty; SF/CPR golden text and page
   counts `diff` empty (E4.1: the named renaming only); PF golden page counts listed before → after.
8. **Look at your own output** (R7; every look or layout card, and every merge check): from
   `apps/desktop/src-tauri`, `mkdir -p "$CARGO_TARGET_DIR/proof" && PRINT_PROOF_OUT="$CARGO_TARGET_DIR/proof" cargo test --locked -j 8 print_proof -- --test-threads=8`, then
   `bash scripts/print/rasterize.sh "$CARGO_TARGET_DIR/proof" "$CARGO_TARGET_DIR/proof/png"`; **Read** every
   PNG your card changed and the reference PNGs under `artifacts/epic_0/E0.1/`; write what you saw
   (alignment, crowding, blank space, wrong cell, clipped text, missing write-in space) and fix it; up to
   three passes; copy the final PNGs (and the PDFs for a gate card) to
   `artifacts/epic_<n>/<card>/`; `du -sk` that directory ≤ 2048. The receipt's "Looked at" row names
   the files, the passes and the differences left against the reference, each with a reason.
9. **Verify once at the widest scope the change reaches:** desktop `cargo test --locked -j 8
   --no-fail-fast -- --test-threads=8` (and root when a root file changed), plus `cd apps/desktop && npm
   run typecheck && npm test && npm run ui-smoke:doc` when frontend or `spec.json` changed. Attribute
   every `test result: FAILED` to its `Running` line. Do not repeat a clean gate (R-V).
10. **Receipt** to `artifacts/epic_<n>/<card>_cycle_receipt.md` (§7). Update the card's `kanban.md` row
    and append a `progress.md` row **in the same commit**. If you added a discovered card, re-count the
    card-count constant in `acceptance-and-verification.md` G-8 and `kanban.md` in that commit (R18).
    Unfiltered `git status --porcelain` prints nothing after it. Push via §5.
11. **Report:** status (`complete | partial | blocked-escalated`), SHA, receipt path, golden deltas,
    what you looked at, discoveries.

## 7. Per-cycle receipt schema

```markdown
# Cycle <card-id> — <epic> / <criterion title>

- **Card ID:** <card-id>   **Model:** <opus|sonnet|haiku>   **RETRO_ACTOR:** <value>
- **Commit SHA:** <sha>    **Base SHA:** <tranche-20 sha at start>
- **Files touched:** <list>   (every one inside FILES YOU OWN, or a named fence exit)
- **FILES YOU OWN closure check:** <producers git-grepped; second call sites listed>
- **Acceptance criterion (verbatim from epic-breakdown.md):** <text>
- **RED (pre-change command + output, failing reason quoted; equals the lead's R21 output):** <…>
- **GREEN (same command + output):** <…>
- **Figures:** <value> — <predicate/denominator> — <command A> → <output>; <command B> → <output>
- **Build scope verified:** <apps/desktop/src-tauri | root | frontend> + log path
- **Clippy before first commit:** <result>   **Clippy stage + sd24 audit test:** <result>
- **Identifier audit:** OK_NO_BUNDLE_TAGS | <violations>
- **Wired-integration audit:** OK_NO_TOKENS / OK_NO_NOOP_HANDLERS / OK_NO_MOCK_LEAKS / OK_NO_WOULD_STRINGS | <violations>
- **Seed deltas (G-1):** PF Aldric / Elowen render hashes: <none | a defect>
- **Golden deltas (G-2):** SF1e ×8, CPR ×8 text + pages: <none | E4.1's named renaming>; PF ×8 pages: <case: before → after, reason …>; Typst warnings: 0
- **Looked at (R7; look/layout cards and merge checks):** <PNG paths; reference compared; passes; differences left and why>
- **Rulings applied:** <R<n> / SD-x + alternative not taken>
- **Does not cover:** <the shapes this proof does not exercise, with the population and its denominator named (R23)>
- **Status:** complete | partial | blocked-escalated
- **Remainder (partial only):** <every remaining item by sub-cause; populations sum to the total>
- **Retro events emitted:** <types + counts>
- **Next-cycle plan:** <…>
```

## 8. Self-heal posture

- **Self-healable:** dirty tree from your own lane; a single-token audit violation; unrelated test-setup
  breakage; a count two implementations re-derive; a missing out dir (`mkdir -p`); a Typst warning
  (SD-e); a PF golden page count that moved (SD-f, listed).
- **Not self-healable (`blocked-escalated` on THIS card; continue independent cards):** two live lanes
  on one file; RED→GREEN not preserved; a stub, mock or "Would …" string in shipping code; an SF/CPR
  golden text delta the card cannot remove (SD-g); PCGen refusing to run at all (SD-h); `CLAIM-EXISTS`.
- **Blocker doctrine:** cleared or escalated, never deferred. The test: was this scope in the Definition
  of Done at scoping? If yes, it is a blocker.
- **Disk:** after each side lane ends, `df -h /` and `git worktree list`; prune the merged, unlocked,
  clean `sd40-ref` / `sd40-smoke` worktree and its `CARGO_TARGET_DIR`. Never remove a `locked` tree or `sd39-main`.

## 9. Placeholder-resolution checklist

```bash
grep -rn '<[a-z_-]*>' docs/release/SD-40-pf-sheet-design/*.md
```

Every match must be (a) a schema placeholder inside a fenced template or a command argument the running
card binds (`<sha>`, `<base>`, `<head>`, `<dir>`, `<lane>`, `<card>`, `<card-id>`, `<n>`, `<epic>`,
`<list>`, `<text>`, `<output>`, `<violations>`, `<result>`, `<value>`, `<cmd>`, `<log>`, `<pidfile>`,
`<script>`, `<type>`, `<look>`, `<preset>`, `<pdf>`, `<pdf-dir>`, `<png-dir>`, `<pcg>`, `<sheet>`,
`<out>`, `<settings>`, `<binary>`, `<clean-worktree>`, `<system>`, `<paper>`, `<seed>`, `<the two new ids>`,
`<epic-start>`, `<date>`, `<filter>`, `<description>`, `<run>`), (b) a deferred value with a named resolution point (`0.20.<run>` → first tester
publish after the operator merges; `(E0.1)` figures → that card; the run id → launch), or (c) a bug to fix
before launch. The author's run is recorded in `progress.md`.

## 10. Epic wrap-up (after every epic)

1. `scripts/retro.py summary --since <epic-start> --json`; fold the counts and any key that fired more
   than once into the merge-check receipt; a key firing more than twice gets a non-zero-exit check in
   the next card that touches that area (R-W).
2. The merge-check card writes the **Epic report** block in `progress.md` and updates `kanban.md` in its
   own closing commit.
3. Sweep this epic's merged, unlocked, clean `sd40-*` worktrees and delete their `CARGO_TARGET_DIR`s.
4. No PR.

## 11. Bundle closure epilogue (corrected order — `decisions.md §15`)

1. **E5.3 final-acceptance scan** — the fenced commands in `acceptance-and-verification.md` G-8. Pass =
   no output from the two `awk`s, `usable: yes` 1 (E2.G) and 5 (E5.2), every DEF/FSR revisit command
   pasted. **If anything is short, stop here:** no retrospective, no sweep, **no PR**. That is a correct
   outcome.
2. **E5.3 retrospective:** `scripts/retro.py summary --since 2026-11-07 --json`; write
   `docs/retro/sd40-retrospective.md` in the shape of `docs/retro/sd39-retrospective.md`; cite it from
   `references/README.md` in the same commit.
3. **E5.3 sweep:** every `sd40-*` worktree and `sd40/*` branch except `sd40-main`, which E5.4–E5.5 still
   need (the script's last step removes it after the PR); found vs removed counts; never `test`,
   `update-index`, `sd39-*` or a locked tree; delete `/home/ubuntu/cargo-target/sd40-*`.
4. **E5.4 release notes:** replace the interim body; `grep -c 'SD-40 INTERIM NOTES'` → 0; every figure with
   its command; `python3 tools/release/resolve_release_notes.py --repo-root .` passes.
5. **E5.4 architecture truth-up**
   (`~/.hermes/profiles/god-emporer/skills/devops/architecture-truth-up/scripts/architecture_truth_up.py --integration-target develop --receipts-md docs/release/SD-40-pf-sheet-design/receipts.md --bundle SD-40`),
   then an Opus claims critic.
6. **E5.5 graphify LAST.** Precondition: unfiltered `git status --porcelain` prints nothing and
   `git rev-parse HEAD` = `git rev-parse origin/tranche-20`. Run
   `~/.hermes/profiles/god-emporer/skills/devops/graphify-update/scripts/update_graphify.py --integration-target develop --receipts-md docs/release/SD-40-pf-sheet-design/receipts.md --bundle SD-40`.
   On the node-count guard (exit 1): file the receipt and stop. **Never** a force flag.
7. **E5.5 PR** `tranche-20 → develop`, the final action: re-run step 1 with the exemption narrowed to
   `E5\.5`; the sentinel grep → 0; commit E5.5's own `complete` row; `gh pr create`; wait inside the
   turn for `gh pr checks <n> --watch`. Red or unopenable → one docs commit setting E5.5
   `blocked-escalated` with the failing job named. **The operator merges.**
8. Stop.

## 12. Standing rules

### 12.1 Carried from SD-36–SD-39 (R1–R20, `../SD-39-ui-print-corrections/workflow-instruction.md §12`)

| Rule | Statement | Checked by |
|---|---|---|
| R2 No hand-kept desktop tables | a look prints what the block holds; no table of values in Typst or TSX | E2.MC, E3.MC |
| R3 Merge checks stay on Opus | every `*.MC` is `opus` | `awk -F'\|' '$2 ~ /MC/ && $4 !~ /opus/' kanban.md` prints nothing |
| R4 `ownedBy` declines | a declined item naming a later card goes to that card | script review |
| R5 Isolated app-data root | any app-launching harness uses a per-run `XDG_DATA_HOME` | E0.2, E4.2, E5.1 receipts |
| R6 Long waits on Sonnet+ | no Haiku on runs that may exceed 20 min | Tier of E5.1, E5.5 ≠ haiku |
| R7 Per-system status | every receipt reports PF seed / SF / CPR / PF golden deltas | `grep -L 'Golden deltas' artifacts/epic_*/*_cycle_receipt.md` prints nothing |
| R8 The script reads the board | dispatch only when every `Depends on` is `complete` | script review |
| R9 Never resume a parallel script | new run with `START=<card>` | `progress.md` run handle |
| R10 Fences have an exit | `decisions.md §19` | receipts |
| R11 Clippy + audit test every code card | §6 step 6 | receipts |
| R12 Generated artifacts re-run-and-diff | `ui-smoke:doc` regenerated and committed by every `spec.json` writer | `git status --porcelain docs/testing/ui-smoke-inventory.md` empty |
| R13 Re-derive copied figures | every figure with its command | receipts |
| R15 FILES YOU OWN is closed | each card greps producers and second call sites before its first write; the receipt has the closure-check row | `grep -L 'closure check' artifacts/epic_*/*_cycle_receipt.md` prints nothing |
| R16 Clippy before the first commit | `cargo clippy --locked --tests` before any `.rs` card's first local commit | receipt row |
| R17 Full ui-smoke has an owner | the full suite is **not** a gate in SD-40; the 14 stale red rows are owned by card E0.2 (FSR-C1 → COM-9, operator answer "1: B", `decisions.md §22`), whose acceptance runs those 14 by name | E0.2 receipt; G-9 |
| R18 Re-count at discovery | the card that adds a discovered row edits the card-count constant in G-8 and `kanban.md` in the same commit | E5.3's `ROW_COUNT` check never fires |
| R19 Observed concurrency | `git status --porcelain` and `git worktree list` before dispatching either side lane | the lead's dispatch log |
| R20 Zero tests is a fail | a filter matching 0 tests proves nothing; receipts quote the `N passed` line | receipts |

### 12.2 SD-39 retrospective changes (R21–R24, `docs/retro/sd39-retrospective.md` "Changes for SD-40")

| Rule | Statement | Checked by |
|---|---|---|
| **R21 RED before dispatch** | the lead runs every card's acceptance command on the pre-change tree and requires a failure or non-zero exit; a command that passes on the base is a defect in the brief, fixed before dispatch (change 1) | the dispatch prompt carries the RED output; the receipt's RED row equals it |
| **R22 Second call sites by `git grep`** | the lead derives each card's second-call-site list over the producers named in its row before dispatch (change 2) | the dispatch log in `progress.md` names the grep per card |
| **R23 Name the population** | every non-MC card's "Does not cover" row names the population it proved and its denominator (change 4) | `grep -L 'denominator\|of [0-9]' artifacts/epic_*/*_cycle_receipt.md` read by each MC |
| **R24 One writer per JSON array** | one writer at a time on `spec.json`: E0.2 (side lane `smoke`) edits the 14 stale rows in place and appends or removes none, then E4.2 appends two; the serial edge E0.2 → E4.2 is a `Depends on` in `kanban.md`; no other card writes it (change 3, a note: it fired once) | §3 file-touch sets; `kanban.md` E4.2 row |

Change 5 (an owner for the stale ui-smoke rows) is FSR-C1, owned by card E0.2 (operator answer "1: B",
2026-11-07, `decisions.md §22`).

### 12.3 Per-cycle and measurement rules

**R-T TDD**, failing reason quoted. **R-B Batch big:** homogeneous remainders in one dispatch (one
look family per card; one golden re-record per card). **R-V One verify per wave.** **R-C Clean tree** at
every commit. **R-N** every figure with its command and denominator; awk or Python, never `grep -o`.
**R-P** every proof states what it does not cover. **R-W** a recurring incident key gets a control, not
a warning. **R-G Widest build scope:** both cargo workspaces when a root file changed. **R-X Cross-tree
renders:** base renders build in their own worktree with their own `CARGO_TARGET_DIR`. **R-Q Quota:**
`decisions.md §13.3`. **R-L Look before you claim:** §6 step 8 on every look or layout card.

## Cross-references

- `docs/governance/workflow-instruction-template.md` (source; its §11 order is defective),
  `docs/release/template/template.md`.
- `docs/governance/blocker-closure-doctrine.md`, `docs/governance/deferral-revisit-doctrine.md`,
  `docs/governance/no-stub-mvp-doctrine.md`.
- `docs/retro/sd39-retrospective.md` (R21–R24), `../SD-39-ui-print-corrections/workflow-instruction.md` (R1–R20).
