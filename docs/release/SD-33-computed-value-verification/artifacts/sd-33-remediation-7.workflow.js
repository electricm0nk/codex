export const meta = {
  name: 'sd-33-remediation-7',
  description: 'SD-33 remediation wave 7 — green the Rust lib suite, then close the bundle',
  whenToUse: 'After sd-33-remediation-6 halted at AT-33-E6-001 attempt 7 with Epic 5 fully closed and 4 of 2,836 lib tests red.',
  phases: [
    { title: 'Suite green' },
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
  return `You are a dispatched SD-33 REMEDIATION WAVE 7 agent. Role: ${role}.

CONTEXT. SD-33 has run seven dispatch waves and been halted seven times by its
own final-acceptance scan. Every halt was correct.

**Epic 5 is now genuinely closed.** Attempt 7's scan verified by execution:
- 8,330 of 8,330 units rowed; the unexamined SET is EMPTY, not merely counted
- \`oracle_disagreement=0\`, \`box_ledger.py --check\` EXIT=0
- \`## Open blockers\` holds no entry — the \`rending_claw_blades\`
  corpus-extraction gap was FIXED (\`7d439876b7\` engine + \`fbc945f198\` corpus),
  not escalated
- \`method_change_rerun_verified: true\` — all three wave-5 corrections re-run
  over their derived affected sets, 0 of 8,291 \`agree\` -> \`disagree\` transitions,
  corroborated three independent ways
- every new-shape audit passed: no silent truncation, no fabricated \`agree\`, no
  "book not in oracle" excuse, EQM host items named and identical on both sides

**Exactly one shortfall remains, and it is yours.** Do not re-litigate anything
above; do not touch \`${E5DIR}/\` results files.

REQUIRED READS, in order:
1. ${REPO}/CLAUDE.md
2. ${REPO}/AGENTS.md
3. ${REPO}/${PKG}/workflow-instruction.md
4. ${REPO}/${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt7_cycle_receipt.md
   (the scan that halted the bundle — it diagnosed your shortfall precisely)
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
scans the package's markdown documents and has caught four agents' own receipts.
Bare hundred-percent tokens are caught specifically.

NO STUBS. NO DoD-SCOPE DEFERRALS.`
}

// ---------------------------------------------------------------------------
// Lane — green the suite.
// ---------------------------------------------------------------------------

function suiteGreenPrompt() {
  return `${standingRules('sd33-r7-suite-green')}

## YOUR TASK — \`cargo test --locked --lib\` must be GREEN

Current state: **2,832 of 2,836 executed lib tests pass; 4 of 2,836 fail.**

    cargo test --locked --lib

Failing:
1. \`rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::a_subset_run_trips_the_population_mismatch_check\`
2. \`rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::corpus_wide_scan_population_matches_the_closed_gate1_census\`
3. \`rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::f1_population_matches_the_current_true_formula_bearing_count_not_the_stale_sd32_census\`
4. \`rules_core::equipment_resolver::tests::catalog_rows_span_every_ingested_book_with_their_real_counts\`

The attempt-7 scan already root-caused both groups. Verify each diagnosis by
execution before acting on it — a scan is a good witness, not a substitute for
running the thing.

### GROUP A — failures 1, 2, 3: an unmapped doneness pair
All three raise \`ValueError: doneness: unmapped 'ambiguous' + 'unmeasurable'\`
through a real shell-out from the Rust test into
\`scripts/shape_ledger.py\` -> \`scripts/coverage_ledger.py:202\` ->
\`scripts/observer/pf1e_dashboard_producer.py:4031\`.

The pair \`(wiring_class='ambiguous', status='unmeasurable')\` exists on **11 of
49,438** work-inventory units. Re-derive that:

    python3 -c "import json,collections
u=json.load(open('docs/work-inventory.json'))['units']
c=collections.Counter((x.get('wiring_class'),x.get('status')) for x in u)
print('ambiguous+unmeasurable:', c[('ambiguous','unmeasurable')], 'of', len(u), 'work-inventory units')"

**This is SD-33's own defect.** \`docs/work-inventory.json\` has exactly one commit
on this branch — \`00ca087775\`, "AT-33-E4-002 — 4,224 unknown units reclassified
to zero", Epic 4's own deliverable. Kanban row 14 was marked \`complete\` over a
suite its own commit turned red. That is the
count-change-needs-a-sweep-not-just-a-build shape, and it is squarely inside the
Definition of Done — naming a future owner for it, as \`progress.md\` currently
does, is not closing it.

**The fix is a real mapping decision, not a silencer.** \`ambiguous\` +
\`unmeasurable\` is a legitimate combination that Epic 4's reclassification
produced; the producer's doneness map simply has no entry for it. Decide what
doneness that pair means, add the entry, and JUSTIFY the choice in the receipt.

Two things you must NOT do:
- do not make the producer swallow unmapped pairs (a silent default defeats the
  fail-closed design that caught this);
- do not edit \`docs/work-inventory.json\` to make the pair disappear — that would
  be reclassifying data to fit an instrument, the exact inversion this bundle
  exists to prevent.

If, on inspection, those 11 units are genuinely mis-classified rather than
legitimately ambiguous, fixing the classification IS allowed — but only via the
Epic 4 code path that produced it, with RED->GREEN, and you must state clearly
which of the two you concluded and why.

### GROUP B — failure 4: a stale hardcoded count
\`equipment_resolver.rs:863\` asserts a catalog count: \`left: 8119, right: 8100\`.
The scan re-confirmed this is INHERITED from the \`tranche/13\` cut and NOT caused
by wave 6's corpus regeneration — verify that yourself
(\`git log\`/\`git show\` the assertion and the catalog source).

Reconcile the two numbers by establishing which is true, and say how you know.
If the corpus genuinely holds 8,119 rows, the assertion is stale and moves;
if 8,100 is right, something is over-counting and that is a real defect.
**Do not simply edit the expected number to match the actual** — that is the
same move this bundle refuses everywhere else. Derive the true count from the
data with a command, put it in the receipt with its denominator, and then make
the assertion state it.

### THE SWEEP THAT MUST FOLLOW
A record-count change compiles clean while leaving OTHER files' hardcoded
assertions red. If any count moves in this cycle, grep the OLD and NEW numbers
across \`tests/\`, \`src/\`, \`apps/\`, and \`scripts/\` before committing, and report
what you found. Use recursive search — a shallow glob lies in this repo.

### FINISH LINE
- \`cargo test --locked --lib\` exits 0. State passed/failed/ignored counts.
- Run the workspace suite too, and \`apps/desktop/src-tauri\` **explicitly** — it
  is a SEPARATE cargo workspace and a root sweep does NOT cover it. Report each
  result. If a suite outside your change is red for an unrelated reason, report
  it with evidence rather than fixing it.
- \`scripts/verify.sh\` in full: report every stage's result. \`--only
  denominator-gate\` must exit 0.
- Re-confirm Epic 5 is undisturbed by your change:
      python3 scripts/box_ledger.py --check --oracle-results ${E5DIR}/AT-33-E5-003.combined-oracle-results.json; echo EXIT=$?
  must still print \`oracle_disagreement=0\` and EXIT=0.

### PROCEDURE
§6, all nine steps, §7 schema with \`- **Status:** <x>\` as a BULLET.
Receipt: \`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-suite-green_cycle_receipt.md\`
Update \`progress.md\` — remove the "remains for its own owner" concession and
record the resolution with its commit. Kanban row 14 stays \`complete\`; append a
pointer to its Notes (pointer only, never a story).

### RETURN VALUE — ONLY JSON, no prose:
{"lane":"suite-green","status":"complete"|"blocked-escalated",
 "lib_before":{"passed":2832,"failed":4},"lib_after":{"passed":0,"failed":0,"ignored":0},
 "group_a":{"pair":"ambiguous+unmeasurable","units":0,"denominator":49438,
            "conclusion":"legitimate-pair-needs-mapping"|"misclassified-units",
            "fix":"...","justification":"...","commit":"..."},
 "group_b":{"asserted":8100,"actual":8119,"true_count":0,"how_derived":"...",
            "conclusion":"stale-assertion"|"real-overcount","commit":"..."},
 "counts_moved":true|false,"count_sweep_result":"...",
 "workspace_suite":"...","desktop_tauri_suite":"...","verify_sh_stages":"...",
 "box_ledger_still_zero":true|false,
 "red_green":"...","identifier_audit":"...","wired_audit":"...",
 "commit_shas":[...],"receipt_path":"...","next":"..."}`
}

// ---------------------------------------------------------------------------
// Epic 6, attempt 8.
// ---------------------------------------------------------------------------

function finalAcceptanceScanPrompt(suiteResult) {
  return `${standingRules('sd33-r8-acceptance-scan')}

## YOUR CRITERION: AT-33-E6-001 — final-acceptance scan, ATTEMPT 8 (kanban row 19)

Seven prior scans FAILED and correctly halted the bundle. Read attempt 7's
receipt: \`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt7_cycle_receipt.md\`

Attempt 7 verified and CLOSED everything except the Rust suite. Re-verify those
closures rather than re-investigating them:
- 8,330 of 8,330 rowed, unexamined SET empty
- \`oracle_disagreement=0\`, \`box_ledger.py --check\` EXIT=0
- \`## Open blockers\` empty, the corpus-extraction gap genuinely fixed
- \`method_change_rerun_verified: true\`, coverage proven over derived sets
- every new-shape audit passed
- the denominator gate widened not blinded; 0 reasonless \`unverifiable\`;
  0 duplicate unit_ids; work-inventory \`unknown\` at 0; no hardcoded exclusion
  lists; Epic 3's artifact at the SD-33 path with SD-32's untouched

Attempt 7's single surviving shortfall: **4 of 2,836 lib tests red.** One lane
has since run. Report:
${JSON.stringify(suiteResult).slice(0, 6000)}

That is a REPORT, not evidence.

## CHECK 1 — THE SUITE IS GREEN, AND GREEN HONESTLY
    cargo test --locked --lib
must exit 0. Then verify HOW:
- **Group A** (\`ambiguous\` + \`unmeasurable\` unmapped doneness). Confirm the
  producer's fail-closed behaviour SURVIVES — feed it a genuinely unmapped pair
  and confirm it still raises, then remove your probe. A mapping gap closed by
  making the producer swallow unknown pairs is a WORSE outcome than the red
  suite, and is BLOCKING. Confirm \`docs/work-inventory.json\` was not edited to
  make the pair disappear (\`git log -p\` it; its only prior commit on this branch
  is \`00ca087775\`). If the lane instead reclassified the 11 units, confirm it
  went through the Epic 4 code path with RED->GREEN and that
  \`jq '[.units[]|select(.status=="unknown")]|length'\` is still 0.
- **Group B** (catalog count 8,119 vs 8,100). Confirm the true count was DERIVED
  from the data with a command, not that the expected number was edited to match
  the actual. Read the diff. Re-derive the count yourself.
- **The sweep.** If any count moved, confirm the old AND new numbers were grepped
  across \`tests/\`, \`src/\`, \`apps/\`, \`scripts/\`. A count change compiles clean
  while leaving other assertions red.

## CHECK 2 — THE OTHER SUITES
Run the workspace suite, and \`apps/desktop/src-tauri\` EXPLICITLY (separate cargo
workspace; a root sweep does not cover it). Report each. A suite red for a reason
unrelated to SD-33 is reported with evidence, not silently accepted.

## CHECK 3 — EPIC 5 IS UNDISTURBED
    python3 scripts/box_ledger.py --check --oracle-results ${E5DIR}/AT-33-E5-003.combined-oracle-results.json; echo EXIT=$?
must still print \`oracle_disagreement=0\` and EXIT=0, and the row counts must
still be 1,741 / 6,589 / 8,330 with an empty unexamined set. Re-derive both.

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
- \`scripts/verify.sh --only denominator-gate\` exits 0, scope still wide, matcher
  not relaxed — re-prove detection live, then remove your probe
- \`jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json\` is 0
- \`sed -n '/## Open blockers/,$p' ${PKG}/progress.md\` holds no entry
- enumerate open deferrals; none defers DoD scope; all carry a revisit condition
- if \`data/corpus/**\` changed, license/PI metadata and \`raw_tokens\` survived

IF ANYTHING IS SHORT: STOP. No retrospective, no sweep, NO PR. Report what is
short WITH THE COMMAND THAT SHOWS IT. Seven prior scans did exactly that and all
seven were right — that record is the reason this bundle's numbers can be
believed.

Receipt: \`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt8_cycle_receipt.md\`
Commit and push (§5). Mark kanban row 19 \`complete\` only on PASS.

Return ONLY JSON:
{"criterion":"AT-33-E6-001","attempt":8,"gate":"PASS"|"FAIL",
 "status":"complete"|"blocked-escalated",
 "lib_suite":{"passed":0,"failed":0,"exit":0},
 "fail_closed_preserved":true|false,"work_inventory_unedited":true|false,
 "count_derived_not_edited":true|false,"count_sweep_verified":true|false,
 "workspace_suite":"...","desktop_tauri_suite":"...",
 "row_counts":{"fixture":0,"literal":0,"combined":0},
 "unexamined_set_empty":true|false,"oracle_disagreement":0,
 "open_blockers_empty":true|false,
 "prior_shortfalls_closed":[{"shortfall":"...","closed":true|false,"command":"...","output":"..."}],
 "shortfalls":[{"what":"...","command":"...","output":"..."}],
 "commands_rerun":[{"command":"...","output":"..."}],
 "receipt_path":"...","commit_shas":[...]}`
}

function retrospectiveAndSweepPrompt(scanResult) {
  return `${standingRules('sd33-r8-retro-sweep')}

## YOUR CRITERION: AT-33-E6-002 — retrospective written and cited (kanban row 20)
## PLUS §11.3 — full worktree/branch sweep

The final-acceptance scan PASSED on attempt 8:
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
   - Epic 4's own reclassification turning the lib suite red on an unmapped
     doneness pair — a DoD defect the bundle closed rather than handed on
   Process comes second.

   **The second story is the throughput arc**, with denominators:
   32 -> 6,940 -> 7,939 -> 8,255 -> 8,263 -> 8,291 -> 8,330 of 8,330 examined;
   disagreements 26 -> 4 -> 1 -> 0.
   **Eight dispatch waves, seven correct halts, one bundle.**

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
   g. **A count change compiles clean and leaves other assertions red.** Epic 4's
      reclassification passed its own criterion and turned four lib tests red.
      Mechanism: a required count sweep across \`tests/\`/\`src/\`/\`apps/\`/\`scripts/\`
      whenever a population figure moves.
   Record also that the gate's own scan SCOPE was a defect, found by the scanner
   and closed by an instrument lane.

   Say plainly: seven scans failed and each failure was the system working. The
   lanes wrote honest short rows rather than false greens — with one exception,
   wave 2's equipment lane, which row-counting caught. That is why this closed
   truthfully.

   Close out workflow-instruction.md §12 rows 3 and 8, both UNENFORCED at launch.

2. CITE IT from \`${PKG}/references/README.md\` IN THIS SAME CYCLE.

3. FULL WORKTREE/BRANCH SWEEP. \`git worktree list\`, \`git branch -a\`, \`df -h /\`.
   Report count FOUND vs REMOVED. Eight dispatch runs left worktrees behind
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
  return `${standingRules('sd33-r8-archdocs-pr')}

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
   say plainly that seven final-acceptance scans failed and what each wave
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
  return `${standingRules('sd33-r8-release-notes')}

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
   Watch for bare hundred-percent tokens — the gate has caught four agents.

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

log('SD-33 REMEDIATION WAVE 7. Epic 5 is closed: 8,330 of 8,330 rowed, 0 disagreements, Open blockers empty, method re-run verified. One shortfall left: 4 of 2,836 lib tests red.')

phase('Suite green')
const suite = await agent(suiteGreenPrompt(), { model: 'sonnet', label: 'suite-green', phase: 'Suite green' })

phase('Epic 6 — Closure epilogue')
const scan = await agent(finalAcceptanceScanPrompt(suite), {
  model: 'opus', label: 'final-acceptance-scan-attempt8', phase: 'Epic 6 — Closure epilogue',
})

const scanFailed = !scan || /"gate"\s*:\s*"FAIL"/.test(String(scan)) || /blocked-escalated/.test(String(scan))
if (scanFailed) {
  log('AT-33-E6-001 attempt 8 did NOT pass. Per §11 step 1: no retrospective, no sweep, NO PR. Correct outcome, not a failure.')
  return {
    bundle: 'SD-33', wave: 7, closed: false,
    halted_at: 'AT-33-E6-001 final-acceptance scan (attempt 8)',
    scan, suite,
  }
}

const retroSweep = await agent(retrospectiveAndSweepPrompt(scan), { model: 'sonnet', label: 'retrospective-and-sweep', phase: 'Epic 6 — Closure epilogue' })
const archPr = await agent(architectureDocsGraphifyPrPrompt(retroSweep), { model: 'sonnet', label: 'archdocs-graphify-pr', phase: 'Epic 6 — Closure epilogue' })
const notes = await agent(releaseNotesVersionBumpPrompt(archPr), { model: 'haiku', label: 'release-notes-version-bump', phase: 'Epic 6 — Closure epilogue' })

log('SD-33 closure epilogue complete. The operator merges tranche/13 -> develop.')

return {
  bundle: 'SD-33', wave: 7, branch: BRANCH, closed: true,
  suite, epic6: { scan, retroSweep, archPr, notes },
}
