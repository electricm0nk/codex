export const meta = {
  name: 'sd-33-dispatch',
  description: 'SD-33 — computed-value verification: oracle harness, engine coverage, classification',
  whenToUse: 'Run the SD-33 bundle to closure on tranche/13, per docs/release/SD-33-computed-value-verification/workflow-instruction.md',
  phases: [
    { title: 'Epic 1 — Instruments' },
    { title: 'Epic 2 — Oracle harness' },
    { title: 'Epic 3 — Engine coverage' },
    { title: 'Epic 4 — Unknown classification' },
    { title: 'Epic 5 — Re-verification' },
    { title: 'Epic 6 — Closure epilogue' },
  ],
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const REPO = '/home/ubuntu/workspace/repos/codex'
const PKG = 'docs/release/SD-33-computed-value-verification'
const BRANCH = 'tranche/13'

const RECEIPT_SCHEMA = `# Cycle <cycle-id> — <epic-name> / <criterion-id>

- **Commit SHA:** <sha>
- **Files touched:** <list>
- **Identifier audit result:** OK_NO_BUNDLE_TAGS / <violations>
- **Wired-integration audit result:** OK_NO_TOKENS / <violations>
- **Acceptance criterion:** <verbatim from epic-breakdown.md>
- **Figures + their re-derive commands:** <every number, with its command and denominator>
- **Status:** complete | blocked-escalated
- **Movement, four buckets:** closure / reclassification / reachability / instrument-correction
- **Notes:** <judgment calls>
- **Next-cycle plan:** <what the next cycle picks up>`

// Standing preamble injected into EVERY dispatched agent (§2.1, §2.2, §2.5, §5, §6).
function standingRules(role) {
  return `You are a dispatched SD-33 execution agent. Role: ${role}.

REQUIRED READS, in order, before any other work:
1. ${REPO}/CLAUDE.md
2. ${REPO}/AGENTS.md
3. ${REPO}/${PKG}/workflow-instruction.md   (the dispatch procedure — binding)
4. ${REPO}/${PKG}/epic-breakdown.md          (your criterion, verbatim)
5. ${REPO}/${PKG}/decisions.md
Read nothing else unless your criterion requires it. Keep context lean.

ENVIRONMENT (§2.1) — run these first:
  export RETRO_ACTOR="${role}"
  export CARGO_TARGET_DIR="/tmp/cargo-sd33-${role}"
  export CARGO_INCREMENTAL=0
  mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"

BRANCH: all work lands on ${BRANCH}.

ONE TURN ONLY (§2.5). Nothing will wake you. Never end your turn waiting on
anything. Foreground slow work, or poll a background job in a loop inside this
turn. If something will not finish, report what you observed and COMMIT AND PUSH
THE WORK ANYWAY. Always commit and push before ending the turn.

GIT DISCIPLINE (§5):
- \`git status --porcelain\` before EVERY git write. Inspect it.
- NEVER \`git add -A\`. NEVER \`git stash\` — the bare form stashes the whole
  shared checkout even from a subdirectory.
- NEVER force-push.
- Push with: \`git fetch origin ${BRANCH} && git rebase origin/${BRANCH} && git push origin HEAD:${BRANCH}\`
  Retry up to 5 times on non-fast-forward.
- Re-read any shared file (progress.md, kanban.md, THE-BOX.md) immediately
  before editing it. THE-BOX.md is append-only for Epics 2-5.
- NEVER hand-edit data/corpus/** — guarded generator path only.
- NEVER pass --allow-stamp-loss.

TEST SCOPING (§2.5): name the targeted binaries/modules plus the workspace
suites you run, and say which sweeps you did NOT run. \`apps/desktop/src-tauri\`
is a SEPARATE cargo workspace — test it explicitly or not at all, never assume a
root sweep covered it.

RETRO EVENTS (§2.3): emit via \`scripts/retro.py\` at the moment they happen, not
batched at the end. \`--verified-by\` is required on a \`correction\`.

FIGURES (decisions.md §2): every number you report states its denominator in the
same construct, and its re-derive command. A bare percentage is not evidence.
Do NOT quote \`retro.py\`'s \`deferrals.open\` as a closure figure unless you have
confirmed SD-32's fix landed (it should read \`len(open_deferrals)\`); say which.

NO STUBS: no stub, inline mock, placeholder, or "Would ..." string in shipping
code. Code paths that ship must do what they claim.`
}

// The §6 per-cycle procedure, given a criterion.
function cycleProcedurePrompt(c) {
  return `${standingRules(c.role)}

## YOUR CRITERION: ${c.id} — ${c.card}
Epic: ${c.epic}
Kanban row: ${c.row}
Receipt path: ${PKG}/artifacts/${c.artifactDir}/${c.id}_cycle_receipt.md

Read ${c.id} verbatim from epic-breakdown.md. Its "Evidence" line is the bar —
satisfy the evidence obligation, not a description of the work.

${c.notes ? `SCOPE NOTES:\n${c.notes}\n` : ''}
## PER-CYCLE PROCEDURE (§6) — follow all nine steps

1. Rebase onto ${BRANCH} (§5), then verify the base is real:
     test -d docs && test -d data && test -d scripts \\
       || { echo 'WRONG BASE — reset before continuing'; exit 1; }
   A nonzero exit here means STOP and reset.

2. Define the audit base once, then run both greps:
     BASE_BRANCH=$(git merge-base HEAD origin/develop)
     git diff --unified=0 "\${BASE_BRANCH}...HEAD" -- <your scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \\
       | grep -nE '\\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
     git diff --unified=0 "\${BASE_BRANCH}...HEAD" -- <your scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \\
       | grep -nE '\\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\\b' || echo 'OK_NO_TOKENS'
   The trailing \\b is deliberately omitted from the first pattern. Do not add it.

3. Implement TDD-style: RED -> confirm it fails FOR THE INTENDED REASON -> GREEN
   -> run the scoped suite. Preserve the RED->GREEN evidence for the receipt.

4. Re-run both audits on the final diff.

5. Write the receipt to the path above, using EXACTLY this schema:

${RECEIPT_SCHEMA}

6. Commit, push via §5.

7. Update ${PKG}/progress.md and ${PKG}/kanban.md in place via §5 (re-read first).

8. Mark kanban row ${c.row} (${c.card}) \`complete\`, and append the receipt
   pointer to progress.md. Keep kanban Notes a POINTER, never a story.

9. Report back: criterion, files touched, SHAs, audit results, RED->GREEN
   evidence, receipt path, discoveries, next-cycle plan.

## SELF-HEAL POSTURE (§8)
Self-healable: dirty tree, single-token audit violation, unrelated test-setup
breakage, build-counter out of sync — fix and continue.
NOT self-healable: diverged tree needing manual rebase; a launch gate not met;
RED->GREEN not preserved; a stub in shipping code.

A \`## Open blockers\` entry is a request for an operator ruling, NOT a
disposition and never a closure path. Filing one PAUSES the bundle. Two
dispositions only: clear it (decompose it and run the cycles — a large blocker is
a sequencing problem, not an exemption), or raise your hand and stop.

## RETURN VALUE
Your final message IS the return value and is parsed by the orchestrator. Return
ONLY a JSON object, no prose around it:
{"criterion":"${c.id}","status":"complete"|"blocked-escalated","commit_shas":[...],
 "files_touched":[...],"identifier_audit":"...","wired_audit":"...",
 "red_green":"<one line of evidence>","receipt_path":"...",
 "figures":[{"value":"...","denominator":"...","command":"..."}],
 "buckets":{"closure":0,"reclassification":0,"reachability":0,"instrument_correction":0},
 "discoveries":"...","next":"..."}`
}

// ---------------------------------------------------------------------------
// Criteria (from epic-breakdown.md / kanban.md)
// ---------------------------------------------------------------------------

const E1 = [
  { id: 'AT-33-E1-001', card: 'box-partition', row: 1, epic: '1 — Instruments', role: 'sd33-e1-box-partition', artifactDir: 'epic-1-instruments',
    notes: `Creates scripts/box_ledger.py (new) and ${PKG}/THE-BOX.md (new). THE-BOX.md partitions ALL 49,438 inventory units, not the not-done subset. Each group carries a count, a disposition, and a re-derive command. Target: \`python3 scripts/box_ledger.py --check\` exits 0 printing \`uncovered=0 overlap=0 population=49438\`. Confirm the 49,438 population by execution against docs/work-inventory.json before trusting it; if it differs, state the real number with its command and say so loudly.` },
  { id: 'AT-33-E1-002', card: 'box-fail-closed', row: 2, epic: '1 — Instruments', role: 'sd33-e1-fail-closed', artifactDir: 'epic-1-instruments',
    notes: `box_ledger.py must exit non-zero on all FIVE conditions: uncovered != 0; overlap != 0; oracle disagreement; an \`unverifiable\` unit dispositioned \`done\`; a \`derived_at\` SHA that is not an ancestor of HEAD (staleness gate). Evidence is FIVE separate RED->GREEN mutation proofs, one per condition, transcribed into the receipt. A tool never observed to fail is not a gate.` },
  { id: 'AT-33-E1-003', card: 'probe-surface-census', row: 3, epic: '1 — Instruments', role: 'sd33-e1-probe-census', artifactDir: 'epic-1-instruments',
    notes: `Enumerate EVERY corpus kind BY EXECUTION — never from memory or prior prose (decisions.md §7). For each kind state whether a probe exists that can verify a computed magnitude, and name the probe. Output artifacts/epic-1-instruments/probe-surface-census.json plus the generating command. The count of kinds with NO probe is a bundle-level figure and goes in progress.md, not a footnote. Hazard: a shallow glob lies here — use recursive find.` },
  { id: 'AT-33-E1-004', card: 'denominator-gate', row: 4, epic: '1 — Instruments', role: 'sd33-e1-denominator-gate', artifactDir: 'epic-1-instruments',
    notes: `\`scripts/verify.sh --only denominator-gate\` must run and FAIL on a percentage stated without its denominator in the same construct. Wire it into verify.sh's stage list — NOT a standalone script. Evidence: RED->GREEN mutation proof using a deliberately-malformed receipt, then the corrected form passing.` },
]

const E5 = [
  { id: 'AT-33-E5-001', card: 'reverify-fixture-verified', row: 16, epic: '5 — Re-verification', role: 'sd33-e5-fixture', artifactDir: 'epic-5-reverification',
    notes: `Population: the 1,741 units blessed by fixture, never by the oracle. Re-examine each against the Epic 2 harness. Commit per-unit (ours, oracle, verdict) rows. State agreement AND disagreement counts, each with its denominator. Read Epic 2's closing receipt first — the Path A/Path B ruling governs how the harness is invoked.` },
  { id: 'AT-33-E5-002', card: 'reverify-literal-verified', row: 17, epic: '5 — Re-verification', role: 'sd33-e5-literal', artifactDir: 'epic-5-reverification',
    notes: `Population: the 6,589 units blessed by literal check. Same evidence obligation as AT-33-E5-001.` },
  { id: 'AT-33-E5-003', card: 'disagreement-resolution', row: 18, epic: '5 — Re-verification', role: 'sd33-e5-disagreements', artifactDir: 'epic-5-reverification',
    notes: `Every disagreement from E5-001/E5-002 is a NAMED DEFECT, fixed or escalated. A disagreement is NEVER closed by adjusting the expectation to match our output. Root-cause each: either our computation is wrong (fix it) or the oracle comparison is wrong (fix the harness AND re-run everything it already judged). One entry per disagreement in progress.md, each resolved to a commit or an operator escalation. A filed blocker does NOT satisfy this criterion.` },
]

// Epics 2/3/4 dispatch as one agent per epic (§2.4), worktree-isolated (§3).
const E2 = {
  id: 'AT-33-E2-001..004', card: 'oracle harness (rows 5-8)', row: '5, 6, 7, 8', epic: '2 — Oracle harness', role: 'sd33-e2-oracle', artifactDir: 'epic-2-oracle-harness',
  notes: `You own ALL FOUR of AT-33-E2-001, -002, -003, -004. Run them in order; write a
separate receipt per criterion; mark each kanban row complete as you finish it.

TIMEBOXED (decisions.md §5). The deliverable is a RULING, not unbounded effort.

Write scope: scripts/oracle_harness/ (new), artifacts/epic-2-oracle-harness/,
THE-BOX.md (APPEND-ONLY, re-read before every edit).

ORACLE PIN — read scripts/pcgen-oracle-pin.env. \`~/workspace/repos/pcgen\` is
FORBIDDEN as an oracle path: fetch-pcgen-oracle.sh's default --dest resolves
there and a preflight-oracle PASS against it fails SILENTLY. Use the repo-local
slot and pass --dest explicitly.

E2-001: prove by RUNNING THE BUILD whether pinned PCGen builds headless on this
box. Resolve all three named risks to facts: Gradle-vs-Java-25,
pcgen.gui2.UIPropertyContext coupling, .pcg input authoring. Commit the build
transcript.
E2-002: one authored .pcg exports through BatchExporter via a template emitting
computed variables, machine-readable output. Path A is NOT "established" until a
real value comes out. Commit the .pcg, the template, the output, the command.
E2-003: harness returns (ours, oracle, agree|disagree|unverifiable) per unit.
\`unverifiable\` is a FIRST-CLASS return, never an error swallowed into \`agree\`.
Commit a fixture set exercising all three outcomes including a KNOWN-DISAGREEING
case. Fixture discipline: a fixture's expected value is transcribed from bytes
the harness's own read path does NOT touch — a fixture built from the file the
harness reads is a mirror, not a check.
E2-004: closing receipt states Path A or Path B EXPLICITLY, and names the
consequence for Epic 5's throughput. If Path B, escalate that consequence to the
operator as a decision point in progress.md — never absorb it silently, and
never let the bundle drift down to "coverage only".

Add to your JSON return value: "path_ruling":"A"|"B", "path_ruling_rationale":"...",
"escalation":"<null or the escalation text>".`,
}

const E3 = {
  id: 'AT-33-E3-001..004', card: 'engine coverage (rows 9-12)', row: '9, 10, 11, 12', epic: '3 — Engine coverage', role: 'sd33-e3-coverage', artifactDir: 'epic-3-engine-coverage',
  notes: `You own ALL FOUR of AT-33-E3-001, -002, -003, -004, in order. Separate receipt
per criterion; mark each kanban row complete as you finish it.

Population: the 6,854 formula-bearing units never run through an engine (E - F,
README.md §4). Verify that number by execution before leaning on it.

Write scope: src/rules_core/pilot_compute/formula_interpreter.rs,
src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs,
src/bin/formula_interpreter.rs,
artifacts/epic-3-engine-coverage/formula_interpreter.corpus-wide.json,
THE-BOX.md (APPEND-ONLY). You READ docs/work-inventory.json pinned at wave start
and NEVER write it — Epic 4 is its sole writer.

E3-001: diagnose the CAUSE before running anything. 41% is a symptom. Explain
per family why 6,854 units were never run — the gap is uneven (F1 28%, F8 21%,
F2 64%). Trace sampled units to concrete coordinates. Output
artifacts/epic-3-engine-coverage/coverage-gap-rootcause.md. GENERIC PASS, NOT
PER-OBJECT LANES — analyse by mechanism.
E3-002: F1's gap closes (6,308 formula-bearing, 1,790 run). Evidence: a fresh
corpus-wide run where F1 population == F1 formula-bearing count, both stated.
E3-003: F2-F9 each reach full population coverage. Per-family table in the
receipt, each row stating run-population and true-population.
E3-004: corpus-wide run reports 11,652 of 11,652, both numbers in the receipt.
MUST use an explicit --output:
  cargo run --locked --bin formula_interpreter -- --corpus-wide --output ${PKG}/artifacts/epic-3-engine-coverage/formula_interpreter.corpus-wide.json
The binary's DEFAULT output artifacts/gate-2-engines/formula_interpreter.corpus-wide.json
is SD-32's closed Gate 2 evidence — NEVER overwrite it.
Also run README.md §4 row G's comparison command and show it returning 0.

NOTE: recognition rate is a SEPARATE number from coverage. A refused unit is a
named finding with its refusal reason — not a coverage failure, and not a silent
exclusion either.`,
}

const E4 = {
  id: 'AT-33-E4-001..003', card: 'unknown classification (rows 13-15)', row: '13, 14, 15', epic: '4 — Unknown classification', role: 'sd33-e4-unknown', artifactDir: 'epic-4-unknown-classification',
  notes: `You own ALL THREE of AT-33-E4-001, -002, -003, in order. Separate receipt per
criterion; mark each kanban row complete as you finish it.

Population: the 4,224 units at \`status: unknown\`, verdict \`unmeasurable\`.
Verify that number by execution first.

Write scope: src/bin/v06_work_inventory.rs, docs/work-inventory.json (you are
its SOLE writer), artifacts/epic-4-unknown-classification/, THE-BOX.md
(APPEND-ONLY).

E4-001: establish the CAUSE before any count moves. Are these units genuinely
unmeasured, or measured by an instrument that could not express the result?
Answer it first. Output artifacts/epic-4-unknown-classification/unknown-rootcause.md.
A count that drops because measurement changed is NOT closure.
E4-002: the 4,224 reach zero. Evidence:
  jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json  -> 0
Report movement in FOUR BUCKETS — closure / reclassification / reachability /
instrument correction — never as a single number.
E4-003: nothing lands in a bucket meaning "we did not look". Every reclassified
unit carries a disposition that is a statement about THE UNIT, not about our
effort. Evidence: box_ledger.py --check passes on the reclassified population,
and no group is named for an absence of work.

NO CARVE-OUTS. not-reachable is a reachability number, never an ingest
exemption. Grep code for hardcoded exclusion lists, not just prose.`,
}

// ---------------------------------------------------------------------------
// Epic 6 prompts (§11, sequential — order is load-bearing)
// ---------------------------------------------------------------------------

function finalAcceptanceScanPrompt() {
  return `${standingRules('sd33-e6-acceptance-scan')}

## YOUR CRITERION: AT-33-E6-001 — final-acceptance scan (kanban row 19)

This is a GATE, not a formality. You are the adversarial check on the whole bundle.

Verify BY EXECUTION that every criterion AT-33-E1-001 .. AT-33-E5-003 is
\`complete\` and every kanban.md card (rows 1-21, excluding your own downstream
rows 20-21) is \`complete\`.

A card at \`returned-to-backlog\`, \`in-progress\`, \`blocked-escalated\`, or
\`complete\`-with-a-deferred-half BLOCKS closure. There is NO
"complete OR filed under Open blockers" — that phrasing let SD-32 close over an
open card and the operator rejected it.

Check the WORK, not the reports. A lane's final message is never evidence:
- \`git log\` and the actual target files for each criterion
- each receipt exists at its stated path and matches the §7 schema, including
  the figures row (every number with denominator + re-derive command) and the
  four-buckets row
- re-run the headline re-derive commands yourself; subagent recaps quote stale
  intermediate figures
- grep the closure instruments themselves for hardcoded exclusion lists —
  carve-outs hide in CODE, not prose (EXCLUDED_BOOKS survived SD-32 this way)
- \`python3 scripts/box_ledger.py --check\` exits 0
- \`scripts/verify.sh --only denominator-gate\` passes
- \`jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json\` is 0
- the Epic 3 corpus-wide artifact is at the SD-33 path and SD-32's
  artifacts/gate-2-engines/ file is UNTOUCHED (\`git log\` it)
- scan every deferral for a named, CHECKED revisit condition

IF ANYTHING IS SHORT: STOP HERE. No retrospective, no sweep, NO PR. Report what
is short WITH THE COMMAND THAT SHOWS IT. That is a CORRECT outcome, not a failure.
Do not fix the shortfall yourself — you are the scanner, not an executor.

Write the receipt to ${PKG}/artifacts/epic-6-closure/AT-33-E6-001_cycle_receipt.md,
commit and push (§5), and mark kanban row 19 complete only if the scan PASSED.

Return ONLY JSON:
{"criterion":"AT-33-E6-001","gate":"PASS"|"FAIL","status":"complete"|"blocked-escalated",
 "shortfalls":[{"what":"...","command":"...","output":"..."}],
 "commands_rerun":[{"command":"...","output":"..."}],
 "receipt_path":"...","commit_shas":[...]}`
}

function retrospectiveAndSweepPrompt(scanResult) {
  return `${standingRules('sd33-e6-retro-sweep')}

## YOUR CRITERION: AT-33-E6-002 — retrospective written and cited (kanban row 20)
## PLUS §11.3 — full worktree/branch sweep

The final-acceptance scan (AT-33-E6-001) returned PASS. Its result:
${JSON.stringify(scanResult).slice(0, 4000)}

1. RETROSPECTIVE. Write docs/retro/sd33-computed-value-verification-retrospective.md,
   grounded in \`python3 scripts/retro.py summary --since 2026-08-25 --json\` — read
   that output, do not paraphrase from memory.
   BINDING CORRECTION (decisions.md §2): retro.py's \`deferrals.open\` was
   \`deferrals[-limit:]\` — the last N, not the open ones. SD-32's fix should have
   landed (\`grep -n "open.*len(open_deferrals)" scripts/retro.py\`). CHECK IT. If
   it landed, use the corrected field and SAY SO in the doc. If not, enumerate
   deferrals directly and state the total.
   Every lesson you record must name its ENFORCING COMMAND or be marked
   UNENFORCED — a lesson without a mechanism is a quote (decisions.md §4).
   Close out workflow-instruction.md §12 rows 3 and 8, both marked UNENFORCED at
   launch: state whether they were closed and how, or why not.

2. CITE IT from ${PKG}/references/README.md IN THIS SAME CYCLE. An uncited
   retrospective does not satisfy the criterion.

3. FULL WORKTREE/BRANCH SWEEP. \`git worktree list\`, \`git branch -a\`, \`df -h /\`.
   Report count FOUND vs count REMOVED. NEVER remove a \`locked\` worktree or one
   carrying unmerged commits — report those instead.

NO PR IN THIS CYCLE. Steps 2 and 3 must land before the PR opens.

Write receipts to ${PKG}/artifacts/epic-6-closure/, commit and push (§5), mark
kanban row 20 complete.

Return ONLY JSON:
{"criterion":"AT-33-E6-002","status":"complete"|"blocked-escalated",
 "retro_path":"...","cited_from":"...","retro_summary_figures":[...],
 "deferrals_field_corrected":true|false,
 "sweep":{"worktrees_found":0,"worktrees_removed":0,"branches_found":0,"branches_removed":0,"kept_locked":[...]},
 "unenforced_rows_closed":{"row3":"...","row8":"..."},
 "receipt_paths":[...],"commit_shas":[...]}`
}

function architectureDocsGraphifyPrPrompt(prior) {
  return `${standingRules('sd33-e6-archdocs-pr')}

## YOUR CRITERION: AT-33-E6-003 (part 1) — architecture docs, graphify, PR (kanban row 21)

Retrospective and sweep are DONE (that order is load-bearing). Prior cycle:
${JSON.stringify(prior).slice(0, 3000)}

Follow ${REPO}/docs/release/template/template.md §6. Read it first — it is the
procedure of record for this step.

1. ARCHITECTURE DOCS. docs/architecture/ is CURRENT-STATE TRUTH. Refresh it for
   what SD-33 actually changed: the oracle harness, box_ledger, the denominator
   gate, formula-interpreter coverage, the work-inventory classification.
2. GRAPHIFY per template §6.
3. OPEN THE PR: ${BRANCH} -> develop. Body cites the retrospective, the receipts,
   and the headline figures WITH THEIR DENOMINATORS.
4. Resolve merge conflicts if any. NEVER force-push. Do NOT merge — the operator
   merges tranche -> develop.

Commit and push (§5). Do not mark kanban row 21 complete yet — the release-notes
and version-bump cycle closes it.

Return ONLY JSON:
{"criterion":"AT-33-E6-003-part1","status":"complete"|"blocked-escalated",
 "arch_docs_touched":[...],"graphify":"...","pr_url":"...","pr_number":0,
 "conflicts_resolved":"...","commit_shas":[...]}`
}

function releaseNotesVersionBumpPrompt(prPrior) {
  return `${standingRules('sd33-e6-release-notes')}

## YOUR CRITERION: AT-33-E6-003 (part 2) — release notes + version bump (kanban row 21)

Housekeeping cycle. The PR is already open:
${JSON.stringify(prPrior).slice(0, 2000)}

1. Fill in ${PKG}/release-notes.md for build 0.13.0 — what shipped, with figures
   that state their denominators. No narrative ceremony.
2. Record the PR number in release-notes.md and in ${PKG}/receipts.md.
3. VERSION BUMP: confirm apps/desktop/package.json and
   apps/desktop/src-tauri/tauri.conf.json both read 0.13.0. They were stamped at
   the ${BRANCH} cut. DO NOT bump the tranche digit — that only moves on a NEW
   tranche/N branch cut, never on a bundle's own closure.
4. Placeholder sweep — every match must resolve or be a documented deferral:
   grep -rn '<[a-z_-]*>' ${PKG}/*.md
5. Update ${PKG}/progress.md to the closed state and mark kanban row 21 complete.

Commit and push (§5).

Return ONLY JSON:
{"criterion":"AT-33-E6-003-part2","status":"complete",
 "release_notes_path":"...","pr_number":0,"versions":{"package_json":"...","tauri_conf":"..."},
 "placeholders_remaining":[...],"commit_shas":[...]}`
}

// ---------------------------------------------------------------------------
// Dispatch
// ---------------------------------------------------------------------------

log('SD-33 dispatch starting on ' + BRANCH + '. Orchestration per workflow-instruction.md §2.4.')

// --- Epic 1 — sequential (§3: Parallel? no). Gates every other epic. ---------
phase('Epic 1 — Instruments')
const e1Results = []
for (const c of E1) {
  const r = await agent(cycleProcedurePrompt(c), {
    model: 'sonnet',
    label: c.card,
    phase: 'Epic 1 — Instruments',
  })
  e1Results.push({ criterion: c.id, result: r })
  log(`Epic 1: ${c.id} (${c.card}) returned.`)
}

const e1Blocked = e1Results.filter(r => typeof r.result === 'string' && /blocked-escalated/.test(r.result))
if (e1Blocked.length) {
  log(`HALT — Epic 1 has ${e1Blocked.length} blocked-escalated criteria. Epic 1 gates every other epic (§3). Bundle paused for an operator ruling.`)
  return { halted_at: 'Epic 1', reason: 'blocked-escalated criteria in the gating epic', e1: e1Results }
}

// --- Epics 2/3/4 — parallel, worktree-isolated (§3, mandatory). -------------
log('Epic 1 complete. Dispatching Epics 2/3/4 in parallel, worktree-isolated (§3).')
const [e2, e3, e4] = await parallel([
  () => agent(cycleProcedurePrompt(E2), { model: 'sonnet', label: 'oracle-harness', phase: 'Epic 2 — Oracle harness', isolation: 'worktree' }),
  () => agent(cycleProcedurePrompt(E3), { model: 'sonnet', label: 'engine-coverage', phase: 'Epic 3 — Engine coverage', isolation: 'worktree' }),
  () => agent(cycleProcedurePrompt(E4), { model: 'sonnet', label: 'unknown-classification', phase: 'Epic 4 — Unknown classification', isolation: 'worktree' }),
])
log('Epics 2/3/4 returned. Run `df -h /` and `git worktree list` after this wave (§8).')

// --- Epic 5 — gated on Epic 2's ruling (AT-33-E2-004), sequential. ----------
phase('Epic 5 — Re-verification')
const e5Results = []
if (!e2) {
  log('Epic 2 returned null — the oracle harness is absent. Epic 5 cannot run: it needs the harness. Escalating rather than silently reducing the bundle to "coverage only" (decisions.md §5).')
} else {
  for (const c of E5) {
    const prompt = cycleProcedurePrompt(c) +
      `\n\n## EPIC 2'S CLOSING RESULT (your gating input — AT-33-E2-004)\n` +
      `${JSON.stringify(e2).slice(0, 6000)}\n\n` +
      `Read Epic 2's committed receipts in ${PKG}/artifacts/epic-2-oracle-harness/ for the\n` +
      `authoritative Path A / Path B ruling — the text above is a report, not evidence.\n` +
      `If the ruling is Path B, Epic 5's throughput assumption changes. That is an\n` +
      `OPERATOR DECISION POINT (decisions.md §5), escalated in progress.md — never a\n` +
      `silent scope reduction to "coverage only".`
    const r = await agent(prompt, { model: 'sonnet', label: c.card, phase: 'Epic 5 — Re-verification' })
    e5Results.push({ criterion: c.id, result: r })
    log(`Epic 5: ${c.id} (${c.card}) returned.`)
  }
}

// --- Epic 6 — closure epilogue, sequential. Order is load-bearing (§11). ----
phase('Epic 6 — Closure epilogue')

// §11.1 — the gate. Opus: adversarial verification tier (§2).
const scan = await agent(finalAcceptanceScanPrompt(), { model: 'opus', label: 'final-acceptance-scan', phase: 'Epic 6 — Closure epilogue' })

const scanFailed = !scan || /"gate"\s*:\s*"FAIL"/.test(String(scan)) || /blocked-escalated/.test(String(scan))
if (scanFailed) {
  log('AT-33-E6-001 did NOT pass. Per §11 step 1: no retrospective, no sweep, NO PR. This is a correct outcome, not a failure.')
  return {
    bundle: 'SD-33',
    closed: false,
    halted_at: 'AT-33-E6-001 final-acceptance scan',
    scan,
    e1: e1Results, e2, e3, e4, e5: e5Results,
  }
}

// §11.2 + §11.3 — retrospective and sweep, BEFORE the PR.
const retroSweep = await agent(retrospectiveAndSweepPrompt(scan), { model: 'sonnet', label: 'retrospective-and-sweep', phase: 'Epic 6 — Closure epilogue' })

// §11.4 — architecture docs, graphify, PR.
const archPr = await agent(architectureDocsGraphifyPrPrompt(retroSweep), { model: 'sonnet', label: 'archdocs-graphify-pr', phase: 'Epic 6 — Closure epilogue' })

// §11.5 — release notes and version bump. Housekeeping tier -> Haiku (§2).
const notes = await agent(releaseNotesVersionBumpPrompt(archPr), { model: 'haiku', label: 'release-notes-version-bump', phase: 'Epic 6 — Closure epilogue' })

log('SD-33 closure epilogue complete. The operator merges tranche/13 -> develop.')

return {
  bundle: 'SD-33',
  branch: BRANCH,
  closed: true,
  epic1: e1Results,
  epic2: e2,
  epic3: e3,
  epic4: e4,
  epic5: e5Results,
  epic6: { scan, retroSweep, archPr, notes },
}
