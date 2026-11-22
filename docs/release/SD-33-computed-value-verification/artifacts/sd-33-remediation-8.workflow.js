export const meta = {
  name: 'sd-33-remediation-8',
  description: 'SD-33 remediation wave 8 — fix the integration-test build, then close the bundle',
  whenToUse: 'After sd-33-remediation-7 halted at AT-33-E6-001 attempt 8 with everything green except the root workspace test build.',
  phases: [
    { title: 'Workspace build green' },
    { title: 'Epic 6 — Closure epilogue' },
  ],
}

const REPO = '/home/ubuntu/workspace/repos/codex'
const PKG = 'docs/release/SD-33-computed-value-verification'
const E5DIR = `${PKG}/artifacts/epic-5-reverification`
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

function standingRules(role) {
  return `You are a dispatched SD-33 REMEDIATION WAVE 8 agent. Role: ${role}.

CONTEXT. SD-33 has run eight dispatch waves and been halted eight times by its
own final-acceptance scan. Every halt was correct. **The bundle is one small fix
from closing.**

Verified GREEN by attempt 8's scan, by execution — do NOT re-litigate any of it:
- \`cargo test --locked --lib\` -> 2,836 passed, 0 failed, EXIT=0
- \`apps/desktop/src-tauri\` -> 548 passed, 0 failed, EXIT=0
- Epic 5: 1,741 / 6,589 / 8,330 rows, unexamined SET empty, 0 disagree,
  0 reasonless \`unverifiable\` of 7,519, \`box_ledger.py --check\` EXIT=0
- \`## Open blockers\` empty; work-inventory \`unknown\` = 0 of 49,438
- the denominator gate green (56 files, 0 violations), matcher untouched,
  detection re-proven live
- the doneness mapping fixed without blinding the fail-closed check; the 8,119
  catalog count DERIVED not fitted; the count sweep found 0 stale live assertions

**Exactly one shortfall remains and it is yours.**

REQUIRED READS, in order:
1. ${REPO}/CLAUDE.md
2. ${REPO}/AGENTS.md
3. ${REPO}/${PKG}/workflow-instruction.md
4. ${REPO}/${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt8_cycle_receipt.md
   (the scan that halted the bundle — it diagnosed your shortfall to the line)
Keep context lean beyond that.

ENVIRONMENT (§2.1) — run first:
  export RETRO_ACTOR="${role}"
  export CARGO_TARGET_DIR="/tmp/cargo-sd33-${role}"
  export CARGO_INCREMENTAL=0
  mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"

BRANCH: all work lands on ${BRANCH}.

ONE TURN ONLY (§2.5). Nothing wakes you. Never end your turn waiting. Foreground
slow work or poll a background job in a loop inside this turn. If something will
not finish, report what you observed and COMMIT AND PUSH ANYWAY. Always commit
and push before ending the turn.

GIT DISCIPLINE (§5):
- \`git status --porcelain\` before EVERY git write. Inspect it.
- NEVER \`git add -A\`. NEVER \`git stash\`. NEVER force-push.
- Push with: \`git fetch origin ${BRANCH} && git rebase origin/${BRANCH} && git push origin HEAD:${BRANCH}\`
  Retry up to 5 times on non-fast-forward.
- Re-read shared files (progress.md, kanban.md) immediately before editing.
- NEVER hand-edit \`data/corpus/**\`. NEVER pass \`--allow-stamp-loss\`.

RETRO EVENTS (§2.3): emit via \`scripts/retro.py\` as they happen. \`--verified-by\`
required on a \`correction\`.

FIGURES (decisions.md §2): every number states its denominator in the SAME
construct. Run \`scripts/verify.sh --only denominator-gate\` before pushing — it
scans the package's markdown documents and has caught five agents' own receipts.
Bare hundred-percent tokens are caught specifically.

NO STUBS. NO DoD-SCOPE DEFERRALS.`
}

// ---------------------------------------------------------------------------
// Lane — fix the integration test build.
// ---------------------------------------------------------------------------

function buildGreenPrompt() {
  return `${standingRules('sd33-r8-build-green')}

## YOUR TASK — the root workspace test build must compile and run

    cargo test --locked --no-run

currently exits 101. **0 of 543 integration test targets execute.** One broken
target blocks the entire build, so every integration suite in the repo is
silently not running — the exact hazard \`AGENTS.md\` names:

> Verify at the widest build scope the repo has ... one broken bin meant
> 0 of 502 suites ran while the phase reported COMPLETE.

### THE DEFECT, diagnosed to the line by attempt 8's scan
\`tests/sd20_equipment_equipmods.rs:94-111\` reads fields \`affects\` and \`bonus\`
on \`WeaponEnhancementBonus\`:

    error[E0609]: no field \`affects\` on type \`&WeaponEnhancementBonus\` (x2)
    error[E0609]: no field \`bonus\` (x2)
    error: could not compile \`codex\` (test "sd20_equipment_equipmods")

**This is SD-33's own debt, and the attribution matters.** Commit \`2f1d52f22d\`
(\`AT-33-E5-finalize-wave5\` — rows 17/18's own commit) split
\`WeaponEnhancementBonus::{affects, bonus}\` into \`{tohit_bonus, damage_bonus}\`
and never updated the test. The scan proved the timeline:

    f652db7ac7 (tranche/13 cut) -> affects, bonus
    66984fe7bc                  -> affects, bonus
    2f1d52f22d                  -> tohit_bonus, damage_bonus
    7d439876b7                  -> tohit_bonus, damage_bonus
    git log f652db7ac7..HEAD -- tests/sd20_equipment_equipmods.rs  -> EMPTY

So it compiled at the cut. It is neither pre-existing nor wave 6's. Wave 7's
suite-green lane reported this item but mis-attributed it to \`7d439876b7\` and to
an "unrelated pre-existing gap", and declined to fix it on that basis — both
claims are false, and the scan filed a \`correction\` for them. Do not repeat
either claim. **Verify the timeline yourself with the commands above** before you
act on it; a scan is a good witness, not a substitute for running the thing.

### THE FIX, named by the scan
Update \`tests/sd20_equipment_equipmods.rs:94-111\` to the new field names, both
\`Option<i16>\`, **preserving each assertion's original intent**:
- the \`+1 Weapon\` case: \`tohit_bonus == Some(1) && damage_bonus == Some(1)\`
- the \`Adamantine\` case: \`tohit_bonus == Some(1) && damage_bonus == None\`

Read the surrounding test and the current struct before applying that — if the
intent of an assertion is not what the scan inferred, follow the INTENT and say
in the receipt where and why you diverged. **Do not weaken an assertion to make
it pass**, and do not delete or \`#[ignore]\` the test. The whole bundle refuses
that move everywhere else.

### THEN SEARCH FOR SIBLINGS
A struct rename that broke one test file may have broken others that simply are
not reached yet, because the build stops at the first failing target. After your
fix compiles, look for every other reference to the old field names across
\`tests/\`, \`src/\`, \`apps/\`, and \`benches/\` if present:

    grep -rn --include='*.rs' -E '\\b(affects|bonus)\\b' tests/ | grep -i weaponenhancement
    grep -rn --include='*.rs' 'WeaponEnhancementBonus' tests/ src/ apps/

Report what you found and whether it needed changing. Use recursive search — a
shallow glob lies in this repo.

### FINISH LINE — the widest build scope
1. \`cargo test --locked --no-run\` exits 0. State how many targets built.
2. \`cargo test --locked\` — the FULL workspace suite, integration targets
   included. Report passed / failed / ignored and the exit code. **This is the
   first time in this bundle that all 543 integration targets will actually
   run.** Expect the possibility of genuine failures that have been invisible
   behind the broken build. If any appear:
   - a failure caused by SD-33's own changes is YOURS to fix, with RED->GREEN;
   - a failure genuinely pre-existing at the \`tranche/13\` cut (\`f652db7ac7\`) is
     reported with the \`git\` evidence that shows it, not fixed;
   - decide by checking out the behaviour at the cut, not by guessing.
3. \`cd apps/desktop/src-tauri && cargo test --locked\` — SEPARATE cargo
   workspace, run it explicitly. It was 548 of 548 green; confirm it still is.
4. \`scripts/verify.sh\` in full — report every stage. \`--only denominator-gate\`
   must exit 0.
5. Re-confirm Epic 5 is undisturbed:
       python3 scripts/box_ledger.py --check --oracle-results ${E5DIR}/AT-33-E5-003.combined-oracle-results.json; echo EXIT=$?
   must still print \`oracle_disagreement=0\` and EXIT=0.

### PROCEDURE
§6, all nine steps, §7 schema with \`- **Status:** <x>\` as a BULLET.
Receipt: \`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-build-green_cycle_receipt.md\`
Update \`progress.md\` with the resolution and its commit. Rows 17 and 18 stay
\`complete\`; append a pointer to their Notes (pointer only, never a story).

### RETURN VALUE — ONLY JSON, no prose:
{"lane":"build-green","status":"complete"|"blocked-escalated",
 "no_run_exit":0,"targets_built":0,
 "timeline_verified":true|false,"attributed_to":"2f1d52f22d",
 "assertions_preserved":true|false,"divergence_from_scan_inference":"...",
 "sibling_references_found":[...],"sibling_references_fixed":[...],
 "workspace_suite":{"passed":0,"failed":0,"ignored":0,"exit":0},
 "new_failures":[{"test":"...","cause":"sd33"|"pre-existing-at-cut","evidence":"...","action":"fixed"|"reported"}],
 "desktop_tauri_suite":{"passed":0,"failed":0,"exit":0},
 "verify_sh_stages":"...","box_ledger_still_zero":true|false,
 "red_green":"...","identifier_audit":"...","wired_audit":"...",
 "commit_shas":[...],"receipt_path":"...","next":"..."}`
}

// ---------------------------------------------------------------------------
// Epic 6, attempt 9.
// ---------------------------------------------------------------------------

function finalAcceptanceScanPrompt(buildResult) {
  return `${standingRules('sd33-r9-acceptance-scan')}

## YOUR CRITERION: AT-33-E6-001 — final-acceptance scan, ATTEMPT 9 (kanban row 19)

Eight prior scans FAILED and correctly halted the bundle. Read attempt 8's
receipt: \`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt8_cycle_receipt.md\`

Attempt 8 verified and CLOSED everything except the root workspace test build.
Re-verify those closures rather than re-investigating them:
- \`cargo test --locked --lib\` 2,836 passed / 0 failed
- \`apps/desktop/src-tauri\` 548 passed / 0 failed
- Epic 5: 1,741 / 6,589 / 8,330, unexamined SET empty, 0 disagree, 0 reasonless
  \`unverifiable\` of 7,519, \`box_ledger.py --check\` EXIT=0
- \`## Open blockers\` empty; work-inventory \`unknown\` = 0 of 49,438
- the denominator gate green, matcher untouched, detection re-proven
- the doneness fail-closed behaviour preserved; the 8,119 count derived; the
  count sweep clean

Attempt 8's single surviving shortfall: **\`cargo test --locked --no-run\` exits
101; 0 of 543 integration targets execute**, caused by SD-33's own \`2f1d52f22d\`
renaming \`WeaponEnhancementBonus\` fields without updating
\`tests/sd20_equipment_equipmods.rs\`. One lane has since run. Report:
${JSON.stringify(buildResult).slice(0, 6000)}

That is a REPORT, not evidence.

## CHECK 1 — THE WIDEST BUILD SCOPE ACTUALLY RUNS
    cargo test --locked --no-run ; echo EXIT=$?
    cargo test --locked ; echo EXIT=$?
Both must exit 0. **Verify the integration targets genuinely EXECUTED** — count
the test binaries that reported results, not just the exit code. A build that
compiles but runs nothing is the same defect wearing a green hat.

Then verify HOW it was fixed:
- the assertions' INTENT was preserved, not weakened. Read the diff of
  \`tests/sd20_equipment_equipmods.rs\`. \`+1 Weapon\` should assert
  \`tohit_bonus == Some(1) && damage_bonus == Some(1)\`; \`Adamantine\` should
  assert \`tohit_bonus == Some(1) && damage_bonus == None\`. An assertion
  loosened, deleted, or \`#[ignore]\`d is BLOCKING.
- sibling references to the old field names were searched for across \`tests/\`,
  \`src/\`, \`apps/\` — confirm the search ran and re-run it yourself.

## CHECK 2 — FAILURES THAT WERE INVISIBLE BEHIND THE BROKEN BUILD
This is the first run in the bundle where all 543 integration targets execute.
If the lane reported new failures, verify each disposition:
- one caused by SD-33's own changes must be FIXED, not reported — check the
  commit;
- one genuinely pre-existing at the \`tranche/13\` cut (\`f652db7ac7\`) may be
  reported, but only with \`git\` evidence proving it predates the cut. Verify
  that evidence yourself; do not accept "pre-existing" as an assertion. Wave 7's
  lane made exactly this mis-attribution and the scan had to correct it.
If the lane reported ZERO new failures across 543 previously-unrun targets, that
deserves a spot-check rather than relief — confirm a sample of those targets
really ran and really passed.

## CHECK 3 — NOTHING ELSE MOVED
- \`cd apps/desktop/src-tauri && cargo test --locked\` still 548 of 548
- \`python3 scripts/box_ledger.py --check --oracle-results ${E5DIR}/AT-33-E5-003.combined-oracle-results.json\`
  still \`oracle_disagreement=0\`, EXIT=0
- row counts still 1,741 / 6,589 / 8,330 with an empty unexamined set — re-derive
- \`scripts/verify.sh --only denominator-gate\` exits 0; re-prove detection live
  with a probe, then remove it and show the baseline back at 0 violations

## THEN THE FULL SCAN
Every criterion AT-33-E1-001 .. AT-33-E5-003 \`complete\`; every kanban card rows
1-18 \`complete\` (19-21 are Epic 6's own). \`returned-to-backlog\`,
\`in-progress\`, \`blocked-escalated\`, or \`complete\`-with-a-deferred-half BLOCKS.
There is NO "complete OR filed under Open blockers".

Check the WORK, not the reports:
- \`git log\` and the actual target files per criterion
- every receipt at its kanban-stated path, matching §7 including the figures row
  (number + denominator + re-derive command) and the four-buckets row
- re-run the headline re-derive commands yourself
- grep the closure INSTRUMENTS for hardcoded exclusion lists
- \`jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json\` is 0
- \`sed -n '/## Open blockers/,$p' ${PKG}/progress.md\` holds no entry
- enumerate open deferrals; none defers DoD scope; all carry a revisit condition
- Epic 3's artifact at the SD-33 path; SD-32's gate-2-engines file UNTOUCHED
- if \`data/corpus/**\` changed, license/PI metadata and \`raw_tokens\` survived

IF ANYTHING IS SHORT: STOP. No retrospective, no sweep, NO PR. Report what is
short WITH THE COMMAND THAT SHOWS IT. Eight prior scans did exactly that and all
eight were right — that record is the reason this bundle's numbers can be
believed. Do not soften on the ninth because the bundle is close.

Receipt: \`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt9_cycle_receipt.md\`
Commit and push (§5). Mark kanban row 19 \`complete\` only on PASS.

Return ONLY JSON:
{"criterion":"AT-33-E6-001","attempt":9,"gate":"PASS"|"FAIL",
 "status":"complete"|"blocked-escalated",
 "no_run_exit":0,"workspace_suite":{"passed":0,"failed":0,"exit":0,"targets_executed":0},
 "assertions_intent_preserved":true|false,"siblings_searched":true|false,
 "new_failures_dispositions":[{"test":"...","claimed":"...","verified":true|false}],
 "lib_suite":{"passed":0,"failed":0},"desktop_tauri_suite":{"passed":0,"failed":0},
 "row_counts":{"fixture":0,"literal":0,"combined":0},
 "unexamined_set_empty":true|false,"oracle_disagreement":0,
 "open_blockers_empty":true|false,"denominator_gate":"PASS"|"FAIL",
 "prior_shortfalls_closed":[{"shortfall":"...","closed":true|false,"command":"...","output":"..."}],
 "shortfalls":[{"what":"...","command":"...","output":"..."}],
 "commands_rerun":[{"command":"...","output":"..."}],
 "receipt_path":"...","commit_shas":[...]}`
}

function retrospectiveAndSweepPrompt(scanResult) {
  return `${standingRules('sd33-r9-retro-sweep')}

## YOUR CRITERION: AT-33-E6-002 — retrospective written and cited (kanban row 20)
## PLUS §11.3 — full worktree/branch sweep

The final-acceptance scan PASSED on attempt 9:
${JSON.stringify(scanResult).slice(0, 4000)}

1. RETROSPECTIVE -> \`docs/retro/sd33-computed-value-verification-retrospective.md\`,
   grounded in \`python3 scripts/retro.py summary --since 2026-08-24 --json\` — READ
   that output. \`deferrals.open\` IS trustworthy (SD-32's fix landed:
   \`grep -n 'len(open_deferrals)' scripts/retro.py\`). Confirm it and say so.

   **THE HEADLINE IS THE DEFECTS.** SD-33 existed to answer one question: is a
   computed value that looks right actually right? It found real wrong answers
   and fixed them. Lead with these, each with its mechanism and its commit:
   - the spell-DC fixture defect — a \`.pcg\` template pinned \`STAT:WIS|SCORE:10\`
     instead of 18, correct by accident for INT/CHA casters and wrong for every
     WIS caster (103 units)
   - the armor compute defect (22 units, \`abc72f75ec\`)
   - the AC MEASUREMENT method that conflated a MAXDEX cap and absorbed a
     co-located Dex chain (\`a68fbeea3d\`) — a wrong ruler, not a wrong engine
   - \`compute_equipmods_effect\` multi-chain summing (\`2f1d52f22d\`)
   - the \`equipment_id_resolve\` OUTPUTNAME-divergent identity fix
   - the campaign-KEY-vs-display-name harness defect
   - the corpus-extraction gap that dropped \`.MOD\`-attached EQMOD references
     (\`7d439876b7\` engine + \`fbc945f198\` corpus)
   - two previously-unwired resolvers landed RED->GREEN (\`EQMWEAPON|DAMAGESIZE\`,
     \`EQM|WEIGHTDIV\`)
   - Epic 4's reclassification turning the lib suite red on an unmapped doneness
     pair — a DoD defect the bundle closed rather than handed on
   - a struct rename in Epic 5's own commit that broke the integration-test
     build, hiding 543 of 543 targets behind a compile error
   Process comes second.

   **The second story is the throughput arc**, with denominators:
   32 -> 6,940 -> 7,939 -> 8,255 -> 8,263 -> 8,291 -> 8,330 of 8,330 examined;
   disagreements 26 -> 4 -> 1 -> 0.
   **Nine dispatch waves, eight correct halts, one bundle.**

   Lessons, each with its ENFORCING MECHANISM (a lesson without a mechanism is a
   quote — decisions.md §4):
   a. **Measure per-unit cost before a population-scoped run.** Mechanism: a
      required dispatch-brief field — measured cost, population, projected wall
      time — filled before the run starts.
   b. **A remainder named per-MECHANISM is closable; "the rest" is not.**
      Mechanism: a required per-shape enumeration of anything unexamined.
   c. **A lane's status must be a mechanical function of its row count.**
      Mechanism: the scan counts rows and derives the unexamined SET.
   d. **Coverage growth surfaces defects late.** The disagreements appeared only
      in newly-covered shapes; a bundle stopping at 95% would have shipped them.
   e. **A method carried past its limit is this bundle's recurring failure
      shape.** Mechanism: when a method stalls, change it and RE-RUN EVERYTHING
      IT ALREADY JUDGED — the scan verifies that re-run, because a corrected
      method leaves stale agreements indistinguishable from real ones.
   f. **A blocker whose fix lives in another subsystem is still a fix.**
      Mechanism: \`blocker-closure-doctrine.md\`'s two-dispositions rule, enforced
      by the scan reading \`## Open blockers\`.
   g. **A count change compiles clean and leaves other assertions red.**
      Mechanism: a required count sweep across \`tests\`/\`src\`/\`apps\`/\`scripts\`
      whenever a population figure moves.
   h. **Verify at the widest build scope the repo has.** One broken test target
      hid 543 of 543 integration suites behind a compile error while the lanes
      that broke it reported \`complete\`. Mechanism: \`cargo test --locked --no-run\`
      plus the full workspace run in the scan, and \`apps/desktop/src-tauri\`
      tested explicitly as the separate workspace it is.
   i. **A lane's attribution of a failure is a claim, not evidence.** Wave 7's
      lane mis-attributed the broken build to another commit and to a
      "pre-existing gap", and declined to fix it on that basis; the scan proved
      the timeline with \`git show\` and filed a correction. Mechanism: the scan
      re-derives attribution from \`git\`, never from the lane's account.
   Record also that the gate's own scan SCOPE was a defect, found by the scanner
   and closed by an instrument lane.

   Say plainly: eight scans failed and each failure was the system working. The
   lanes wrote honest short rows rather than false greens — with two exceptions,
   both caught mechanically: wave 2's equipment lane claiming \`complete\` over
   103 of 494 (caught by row-counting), and wave 7's mis-attribution (caught by
   re-deriving from \`git\`).

   Close out workflow-instruction.md §12 rows 3 and 8, both UNENFORCED at launch.

2. CITE IT from \`${PKG}/references/README.md\` IN THIS SAME CYCLE.

3. FULL WORKTREE/BRANCH SWEEP. \`git worktree list\`, \`git branch -a\`, \`df -h /\`.
   Report count FOUND vs REMOVED. Nine dispatch runs left worktrees behind
   (\`.claude/worktrees/wf_*\`). Check each for unmerged commits BEFORE removing.
   NEVER remove a \`locked\` worktree or one carrying unmerged commits.

NO PR IN THIS CYCLE. Steps 2 and 3 land before the PR opens.

Receipts to \`${PKG}/artifacts/epic-6-closure/\`. Commit, push (§5), mark row 20
complete. Run \`scripts/verify.sh --only denominator-gate\` before pushing.

Return ONLY JSON:
{"criterion":"AT-33-E6-002","status":"complete"|"blocked-escalated",
 "retro_path":"...","cited_from":"...","retro_summary_figures":[...],
 "defects_found_and_fixed":[{"defect":"...","units":0,"commit":"..."}],
 "deferrals_field_corrected":true|false,"lessons_with_mechanisms":[...],
 "sweep":{"worktrees_found":0,"worktrees_removed":0,"branches_found":0,"branches_removed":0,"kept_locked":[...],"kept_unmerged":[...]},
 "unenforced_rows_closed":{"row3":"...","row8":"..."},
 "denominator_gate":"PASS"|"FAIL",
 "receipt_paths":[...],"commit_shas":[...]}`
}

function architectureDocsGraphifyPrPrompt(prior) {
  return `${standingRules('sd33-r9-archdocs-pr')}

## YOUR CRITERION: AT-33-E6-003 (part 1) — architecture docs, graphify, PR (kanban row 21)

Retrospective and sweep are DONE (that order is load-bearing). Prior cycle:
${JSON.stringify(prior).slice(0, 3000)}

Follow \`${REPO}/docs/release/template/template.md §6\` — read it first.

1. ARCHITECTURE DOCS — \`docs/architecture/\` is CURRENT-STATE TRUTH. Refresh for
   what SD-33 actually changed:
   - the PCGen oracle harness, its batch generators, and the homebrew-LST EQMOD
     mechanism that replaced the untrustworthy \`.pcg\`-time attachment
   - the per-type AC isolator that replaced the whole-character \`AC.TOTAL\` diff
   - the spell casting-ability mapping and the corrected \`.pcg\` fixture template
   - the widened equipment bonus-shape coverage (VAR / COMBAT / WEAPON /
     WEAPONPROF / STAT / SAVE / SITUATION / SKILL / natural-attack /
     special-material / EQM* modifiers)
   - the armor and \`compute_equipmods_effect\` fixes; \`equipment_id_resolve\`
     identity; the new \`EQMWEAPON|DAMAGESIZE\` and \`EQM|WEIGHTDIV\` resolvers
   - the corpus-extraction fix for \`.MOD\`-attached EQMOD references
   - the L20-per-class pilot-build probe path
   - \`box_ledger.py\` and the THE-BOX partition; the denominator gate in
     \`verify.sh\` and its widened scan scope
   - formula-interpreter corpus-wide coverage; work-inventory \`unknown\` -> zero
     and the doneness mapping that reclassification required
2. GRAPHIFY per template §6.
3. OPEN THE PR: ${BRANCH} -> develop. Lead with the DEFECTS FOUND AND FIXED, cite
   the retrospective and receipts, state headline figures WITH DENOMINATORS, and
   say plainly that eight final-acceptance scans failed and what each wave
   closed. That history is the strongest evidence the gate works.
4. Resolve merge conflicts if any. NEVER force-push. DO NOT MERGE — the operator
   merges tranche -> develop.

Commit and push (§5). Do NOT mark row 21 complete; release-notes owns it.

Return ONLY JSON:
{"criterion":"AT-33-E6-003-part1","status":"complete"|"blocked-escalated",
 "arch_docs_touched":[...],"graphify":"...","pr_url":"...","pr_number":0,
 "conflicts_resolved":"...","commit_shas":[...]}`
}

function releaseNotesVersionBumpPrompt(prPrior) {
  return `${standingRules('sd33-r9-release-notes')}

## YOUR CRITERION: AT-33-E6-003 (part 2) — release notes + version bump (kanban row 21)

Housekeeping, and the last cycle of the bundle. The PR is open:
${JSON.stringify(prPrior).slice(0, 2000)}

1. Fill in \`${PKG}/release-notes.md\` for build \`0.13.0\` — what shipped, every
   figure stating its denominator in the same construct. Lead with the computed
   values that were WRONG and are now right. No narrative ceremony.
2. Record the PR number in \`release-notes.md\` and in \`${PKG}/receipts.md\`.
3. VERSION BUMP: confirm \`apps/desktop/package.json\` and
   \`apps/desktop/src-tauri/tauri.conf.json\` BOTH read \`0.13.0\`. Stamped at the
   ${BRANCH} cut. DO NOT bump the tranche digit — it moves only on a NEW
   tranche/N branch cut, never on a bundle's own closure.
4. Placeholder sweep — every match resolves or is a documented deferral:
   grep -rn '<[a-z_-]*>' ${PKG}/*.md
5. Update \`${PKG}/progress.md\` to the closed state; mark kanban row 21 \`complete\`.
6. Re-run \`scripts/verify.sh --only denominator-gate\` and confirm exit 0 before
   pushing. Its scope includes \`release-notes.md\`, so your own prose IS scanned.
   Watch for bare hundred-percent tokens — the gate has caught five agents.

Commit and push (§5).

Return ONLY JSON:
{"criterion":"AT-33-E6-003-part2","status":"complete",
 "release_notes_path":"...","pr_number":0,
 "versions":{"package_json":"...","tauri_conf":"..."},
 "placeholders_remaining":[...],"denominator_gate":"PASS"|"FAIL",
 "commit_shas":[...]}`
}

// ---------------------------------------------------------------------------
// Dispatch
// ---------------------------------------------------------------------------

log('SD-33 REMEDIATION WAVE 8. Everything green except the root workspace test build: 0 of 543 integration targets execute behind one compile error in tests/sd20_equipment_equipmods.rs, caused by SD-33 commit 2f1d52f22d.')

phase('Workspace build green')
const build = await agent(buildGreenPrompt(), { model: 'sonnet', label: 'build-green', phase: 'Workspace build green' })

phase('Epic 6 — Closure epilogue')
const scan = await agent(finalAcceptanceScanPrompt(build), {
  model: 'opus', label: 'final-acceptance-scan-attempt9', phase: 'Epic 6 — Closure epilogue',
})

const scanFailed = !scan || /"gate"\s*:\s*"FAIL"/.test(String(scan)) || /blocked-escalated/.test(String(scan))
if (scanFailed) {
  log('AT-33-E6-001 attempt 9 did NOT pass. Per §11 step 1: no retrospective, no sweep, NO PR. Correct outcome, not a failure.')
  return {
    bundle: 'SD-33', wave: 8, closed: false,
    halted_at: 'AT-33-E6-001 final-acceptance scan (attempt 9)',
    scan, build,
  }
}

const retroSweep = await agent(retrospectiveAndSweepPrompt(scan), { model: 'sonnet', label: 'retrospective-and-sweep', phase: 'Epic 6 — Closure epilogue' })
const archPr = await agent(architectureDocsGraphifyPrPrompt(retroSweep), { model: 'sonnet', label: 'archdocs-graphify-pr', phase: 'Epic 6 — Closure epilogue' })
const notes = await agent(releaseNotesVersionBumpPrompt(archPr), { model: 'haiku', label: 'release-notes-version-bump', phase: 'Epic 6 — Closure epilogue' })

log('SD-33 closure epilogue complete. The operator merges tranche/13 -> develop.')

return {
  bundle: 'SD-33', wave: 8, branch: BRANCH, closed: true,
  build, epic6: { scan, retroSweep, archPr, notes },
}
