export const meta = {
  name: 'sd-33-remediation-9',
  description: 'SD-33 remediation wave 9 — reconcile the two .MOD-chain token derivations, then close the bundle',
  whenToUse: 'After sd-33-remediation-8 halted at AT-33-E6-001 attempt 9 with corpus_literal_sweep red on SD-33 own wave-6 regeneration.',
  phases: [
    { title: 'Corpus sweep green' },
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
  return `You are a dispatched SD-33 REMEDIATION WAVE 9 agent. Role: ${role}.

CONTEXT. SD-33 has run nine dispatch waves and been halted nine times by its own
final-acceptance scan. Every halt was correct.

Verified GREEN by attempt 9's scan, by execution — do NOT re-litigate any of it:
- \`cargo test --locked --no-run\` EXIT=0; **543 of 543 integration targets build
  and 599 of 599 executables execute** (the previous wave's compile break, in
  SD-33's own \`2f1d52f22d\`, is fixed and its assertions were STRENGTHENED)
- \`cargo test --locked --lib\` 2,836 passed / 0 failed
- \`apps/desktop/src-tauri\` 548 passed / 0 failed
- Epic 5: 1,741 / 6,589 / 8,330 rows all distinct, unexamined SET empty both
  directions, 0 disagree, 0 agree-with-mismatch of 811, 0 reasonless
  \`unverifiable\` of 7,519, \`box_ledger.py --check\` EXIT=0
- work-inventory \`unknown\` = 0 of 49,438
- the denominator gate green (58 files, 0 violations), matcher untouched,
  detection re-proven live
- 31 of 599 suites carrying 49 of 8,023 test failures are **PROVEN pre-existing
  at the \`tranche/13\` cut** — normalized-identical results at \`f652db7ac7\` and
  HEAD, 0 commits since the cut for all 31. Inherited, not SD-33's. Not yours.

**Exactly one shortfall remains and it is yours.**

REQUIRED READS, in order:
1. ${REPO}/CLAUDE.md
2. ${REPO}/AGENTS.md
3. ${REPO}/${PKG}/workflow-instruction.md
4. ${REPO}/${PKG}/artifacts/epic-6-closure/AT-33-E6-001-build-green_cycle_receipt.md
   (the lane that FOUND your defect and filed it — full evidence, exact command,
   the confirmed record paths. Good diagnostic work; it simply filed instead of
   fixing.)
5. ${REPO}/${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt9_cycle_receipt.md
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
- **NEVER hand-edit \`data/corpus/**\`** — guarded generator path only.
- **NEVER pass \`--allow-stamp-loss\`.**

RETRO EVENTS (§2.3): emit via \`scripts/retro.py\` as they happen. \`--verified-by\`
required on a \`correction\`.

FIGURES (decisions.md §2): every number states its denominator in the SAME
construct. Run \`scripts/verify.sh --only denominator-gate\` before pushing — it
scans the package's markdown documents and has caught six agents' own receipts.
Bare hundred-percent tokens are caught specifically.

NO STUBS. NO DoD-SCOPE DEFERRALS.`
}

function corpusSweepPrompt() {
  return `${standingRules('sd33-r9-corpus-sweep')}

## YOUR TASK — two independent \`.MOD\`-chain derivations disagree. Reconcile them.

    cargo run --locked --bin corpus_literal_sweep ; echo SWEEP_EXIT=$?

currently reports **105 findings across 10 of the 137 \`data/corpus/**\` records
this bundle changed**, and exits 1. \`scripts/verify.sh\`'s \`corpus-sweep\` stage
is therefore RED.

### THE DEFECT, root-caused by the lane that filed it
Two pieces of code independently derive the token closure for a corpus record by
walking the pinned PCGen oracle's \`.lst\` \`.MOD\` chains, and **they disagree**:

- \`src/bin/enrich_equipment_raw_tokens.rs\` — **new this bundle, +243 lines in
  wave 6.** It WRITES \`data.raw_tokens\` / \`data.raw_bonus_chains\`.
- \`src/rules_core/corpus_literal_sweep.rs::token_closure\` — **unchanged since
  the \`tranche/13\` cut.** It independently RE-DERIVES the closure and checks
  that every token a record claims is byte-present in it.

Before wave 6 those fields were \`[]\` on these records, so the sweep passed
**vacuously** — its population is "every token the record itself claims", and a
record claiming nothing cannot mismatch. Wave 6 populated them and the
disagreement became visible. **That is the gate doing its job on new data, not a
new gate.**

Confirmed affected records (5 visible before the log's own 40-item cap):
\`ultimate_equipment/equipment/{blade_of_the_sword_saint,blade_of_the_rising_sun,hammer_polarity,hellscourge}.json\`
and \`inner_sea_gods/equipment/fugitive_finder.json\`. **Enumerate all 10 yourself**
— raise or remove the log cap, or query the sweep directly. Do not work from a
truncated list.

### THE QUESTION YOU MUST ANSWER FIRST
**Which of the two is wrong?** Do not assume. Both outcomes are real and each
implies different work:

- **If the ENRICHER is wrong** — it wrote tokens the oracle source does not
  actually contain, or folded a \`.MOD\` chain incorrectly. Fix
  \`enrich_equipment_raw_tokens.rs\`, then REGENERATE the affected records through
  the guarded generator path.
  **THIS CASE HAS AN EPIC 5 CONSEQUENCE YOU MUST FOLLOW.** Those \`raw_tokens\`
  feed our compute path. If they were wrong, the \`ours\` values Epic 5 recorded
  for the affected units may be wrong too — and Epic 5 currently reports 0
  disagreements. **Identify every Epic 5 unit whose \`ours\` derives from an
  affected record, re-run them through the oracle harness, and report what
  moved.** A corrected input that leaves stale agreements is the same failure
  shape this bundle has now hit three times.
- **If \`corpus_literal_sweep.rs\` is wrong** — its independent closure-builder
  mishandles a \`.MOD\` form the enricher handles correctly. Fix the sweep. Note
  that it is UNCHANGED since the cut and has been green on other data, so this
  case needs stronger evidence: show the specific \`.MOD\` construct it gets wrong,
  read from the pinned oracle \`.lst\` bytes.
- **If BOTH are partly wrong**, say so and fix both.

**Decide by reading the pinned oracle \`.lst\` source bytes for a specific
affected record and working out by hand what the true closure is.** That is the
tie-breaker — a third, independent reading, not a preference between two
programs. Put that hand-derivation in the receipt for at least one record, token
by token. Related context: wave 6 already found and fixed a genuine
\`.MOD\`-attached EQMOD extraction gap (\`7d439876b7\` + \`fbc945f198\`) for
\`rending_claw_blades\`, so \`.MOD\` handling in this repo is known to have been
incomplete. That makes "the enricher is wrong" plausible — but plausible is not
proven, and the same fix may equally have been incomplete in the sweep.

### WHAT YOU MUST NOT DO
- Do NOT silence, cap, or narrow \`corpus_literal_sweep\`'s population to make it
  pass. A gate weakened to pass is worse than the red it replaced (AT-33-E1-002).
- Do NOT hand-edit \`data/corpus/**\`. Regeneration is through the guarded
  generator path only.
- Do NOT revert wave 6's enrichment wholesale to restore the vacuous pass. Empty
  \`raw_tokens\` was never correct; it was merely unmeasurable. Reverting to
  unmeasurable is the "we did not look" bucket AT-33-E4-003 forbids.
- Do NOT add the affected records to any exclusion list. Carve-outs hide in code,
  not prose.

### REGENERATION HAZARDS — all three previously observed in this repo
- regenerating can **destroy license/PI metadata and \`raw_tokens\`** — verify per
  record that they survive, and never pass \`--allow-stamp-loss\`;
- a **record-count change compiles clean but leaves other files' hardcoded
  assertions red** — if any count moves, grep the OLD and NEW numbers across
  \`tests/\`, \`src/\`, \`apps/\`, \`scripts/\` before committing;
- a shallow glob lies here — use recursive search.

### CLEAR THE BLOCKER
\`${PKG}/progress.md\` carries 1 active entry under \`## Open blockers\` (real
heading at line ~304), filed by \`sd33-r8-build-green\`, plus a \`deferral\` retro
event in \`docs/retro/events/sd33-r8-build-green.jsonl\`. Per
\`${REPO}/docs/governance/blocker-closure-doctrine.md\` there are two dispositions
only: clear it, or raise a hand and stop. Filing is not a disposition.
**Clear it**: remove the entry, record the resolution with its commit, and
resolve the deferral event via \`scripts/retro.py\`. Leave the section's standing
preamble intact.

### FINISH LINE
1. \`cargo run --locked --bin corpus_literal_sweep\` reports **0 findings**, EXIT=0.
   State findings-before and findings-after with the record denominator.
2. \`scripts/verify.sh\` in full — every stage's result, \`corpus-sweep\` included.
   \`--only denominator-gate\` exits 0.
3. \`cargo test --locked --lib\` still 2,836 / 0. \`cargo test --locked --no-run\`
   still EXIT=0. \`cd apps/desktop/src-tauri && cargo test --locked\` still 548 / 0.
4. Epic 5 undisturbed OR correctly updated:
       python3 scripts/box_ledger.py --check --oracle-results ${E5DIR}/AT-33-E5-003.combined-oracle-results.json; echo EXIT=$?
   \`oracle_disagreement=0\`, EXIT=0, rows still 1,741 / 6,589 / 8,330 with an
   empty unexamined set. If you re-ran units, say how many and what moved — and
   if a re-run surfaced a real \`disagree\`, root-cause and fix it rather than
   leaving it.
5. \`## Open blockers\` holds no active entry.

### PROCEDURE
§6, all nine steps, §7 schema with \`- **Status:** <x>\` as a BULLET.
Receipt: \`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-corpus-sweep_cycle_receipt.md\`
Update \`progress.md\`. Rows 16-18 stay \`complete\`; append a pointer to their
Notes (pointer only, never a story).

### RETURN VALUE — ONLY JSON, no prose:
{"lane":"corpus-sweep","status":"complete"|"blocked-escalated",
 "findings_before":105,"findings_after":0,"records_affected":10,"records_denominator":137,
 "all_ten_enumerated":[...],
 "hand_derivation_record":"...","hand_derivation_conclusion":"enricher-wrong"|"sweep-wrong"|"both-wrong",
 "fix":"...","fix_commit":"...","red_green":"...",
 "records_regenerated":0,"license_pi_preserved":true|false,"raw_tokens_preserved":true|false,
 "counts_moved":true|false,"count_sweep_result":"...",
 "epic5_units_affected":0,"epic5_units_rerun":0,"epic5_rows_moved":0,
 "epic5_new_disagreements":[{"unit_id":"...","rootcause":"...","resolution":"..."}],
 "open_blocker_removed":true|false,"deferral_resolved":true|false,
 "sweep_exit":0,"verify_sh_stages":"...",
 "lib_suite":"...","no_run_exit":0,"desktop_tauri_suite":"...",
 "box_ledger_still_zero":true|false,
 "identifier_audit":"...","wired_audit":"...",
 "commit_shas":[...],"receipt_path":"...","next":"..."}`
}

// ---------------------------------------------------------------------------
// Epic 6, attempt 10.
// ---------------------------------------------------------------------------

function finalAcceptanceScanPrompt(sweepResult) {
  return `${standingRules('sd33-r10-acceptance-scan')}

## YOUR CRITERION: AT-33-E6-001 — final-acceptance scan, ATTEMPT 10 (kanban row 19)

Nine prior scans FAILED and correctly halted the bundle. Read attempt 9's
receipt: \`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt9_cycle_receipt.md\`

Attempt 9 verified and CLOSED everything except the corpus sweep. Re-verify those
closures rather than re-investigating them (the list is in your standing rules
above). Its single surviving shortfall: **\`corpus_literal_sweep\` RED, 105
findings across 10 of 137 changed records, filed under \`## Open blockers\` rather
than cleared.** One lane has since run. Report:
${JSON.stringify(sweepResult).slice(0, 6000)}

That is a REPORT, not evidence.

## CHECK 1 — THE SWEEP IS GREEN, AND GREEN HONESTLY
    cargo run --locked --bin corpus_literal_sweep ; echo SWEEP_EXIT=$?
must report 0 findings and exit 0. Then verify HOW, because there are four
illegitimate routes to that zero and each is BLOCKING:
(a) the sweep's population narrowed, capped, or filtered so it checks less —
    \`git log -p -- src/rules_core/corpus_literal_sweep.rs\` and read the diff;
(b) the affected records' \`raw_tokens\` reverted to \`[]\`, restoring the VACUOUS
    pass — check the 10 records' fields are still populated;
(c) records added to an exclusion list — grep for exclusion/skip/allowlist
    constructs in the sweep and the enricher;
(d) \`data/corpus/**\` hand-edited rather than regenerated through the guarded
    path — check the commit and the generator invocation.
Confirm the fix rests on a **hand-derivation from the pinned oracle \`.lst\`
bytes** for at least one record, token by token, and check that derivation
yourself against the source. Two programs disagreeing is settled by the source,
not by preferring one program.

**Re-prove the sweep still fails on a genuine mismatch**: plant a bogus token in
one record's claimed set, confirm the sweep FAILS, restore, confirm 0. Use the
generator path or a scratch copy — do not leave residue in \`data/corpus/**\`.

## CHECK 2 — THE EPIC 5 CONSEQUENCE WAS FOLLOWED
If the ENRICHER was wrong, the \`raw_tokens\` it wrote feed our compute path, so
Epic 5's \`ours\` values for affected units may have been wrong while still
reporting \`agree\`. Verify:
- the set of Epic 5 units deriving from the 10 affected records was DERIVED by
  execution, with its command and denominator — not assumed empty;
- those units were re-run through the harness;
- what moved is reported, and any surfaced \`disagree\` was root-caused and fixed,
  not left.
If the lane concluded the affected records touch ZERO Epic 5 units, verify that
claim yourself — it is exactly the kind of convenient emptiness worth checking.

## CHECK 3 — NOTHING ELSE MOVED
- \`cargo test --locked --no-run\` EXIT=0; \`cargo test --locked --lib\` 2,836 / 0
- \`cd apps/desktop/src-tauri && cargo test --locked\` 548 / 0
- the 31 pre-existing failing suites are still exactly 31 with 49 failures —
  if that set GREW, the growth is SD-33's and is BLOCKING; re-derive it
- \`box_ledger.py --check --oracle-results\` -> \`oracle_disagreement=0\`, EXIT=0
- rows still 1,741 / 6,589 / 8,330, unexamined SET empty both directions
- if \`data/corpus/**\` changed, license/PI metadata and \`raw_tokens\` survived, and
  any count change was swept across \`tests\`/\`src\`/\`apps\`/\`scripts\`
- \`scripts/verify.sh\` in full: report EVERY stage. \`--only denominator-gate\`
  exits 0; re-prove detection live with a probe, remove it, show baseline 0

## CHECK 4 — \`## Open blockers\` HOLDS NO ACTIVE ENTRY
    sed -n '/## Open blockers/,$p' ${PKG}/progress.md
Find the REAL heading (earlier attempts hit archived \`<details>\` copies) and
bound the section at the next \`## \`. No active \`###\` entry may remain, and the
\`deferral\` retro event filed with it must be resolved:
    python3 scripts/retro.py summary --since 2026-08-24 --json
Enumerate open deferrals; none may defer DoD scope; all carry a revisit condition.

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
- Epic 3's artifact at the SD-33 path; SD-32's gate-2-engines file UNTOUCHED

IF ANYTHING IS SHORT: STOP. No retrospective, no sweep, NO PR. Report what is
short WITH THE COMMAND THAT SHOWS IT. Nine prior scans did exactly that and all
nine were right — that record is the reason this bundle's numbers can be
believed. **Do not soften on the tenth because the bundle is close.** Equally, do
not manufacture a shortfall: if the work is genuinely done, PASS it.

Receipt: \`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt10_cycle_receipt.md\`
Commit and push (§5). Mark kanban row 19 \`complete\` only on PASS.

Return ONLY JSON:
{"criterion":"AT-33-E6-001","attempt":10,"gate":"PASS"|"FAIL",
 "status":"complete"|"blocked-escalated",
 "sweep":{"findings":0,"exit":0},
 "sweep_green_honestly":true|false,"illegitimate_route_found":"none"|"...",
 "hand_derivation_checked":true|false,"sweep_still_fails_on_mismatch":true|false,
 "epic5_consequence_followed":true|false,"epic5_units_affected":0,"epic5_rows_moved":0,
 "preexisting_failing_suites":{"count":0,"failures":0,"grew":true|false},
 "lib_suite":{"passed":0,"failed":0},"no_run_exit":0,
 "desktop_tauri_suite":{"passed":0,"failed":0},
 "row_counts":{"fixture":0,"literal":0,"combined":0},
 "unexamined_set_empty":true|false,"oracle_disagreement":0,
 "open_blockers_empty":true|false,"deferrals_open":0,
 "denominator_gate":"PASS"|"FAIL","verify_sh_stages":"...",
 "prior_shortfalls_closed":[{"shortfall":"...","closed":true|false,"command":"...","output":"..."}],
 "shortfalls":[{"what":"...","command":"...","output":"..."}],
 "commands_rerun":[{"command":"...","output":"..."}],
 "receipt_path":"...","commit_shas":[...]}`
}

function retrospectiveAndSweepPrompt(scanResult) {
  return `${standingRules('sd33-r10-retro-sweep')}

## YOUR CRITERION: AT-33-E6-002 — retrospective written and cited (kanban row 20)
## PLUS §11.3 — full worktree/branch sweep

The final-acceptance scan PASSED on attempt 10:
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
     (\`7d439876b7\` + \`fbc945f198\`)
   - two previously-unwired resolvers landed RED->GREEN (\`EQMWEAPON|DAMAGESIZE\`,
     \`EQM|WEIGHTDIV\`)
   - Epic 4's reclassification turning the lib suite red on an unmapped doneness
     pair — a DoD defect the bundle closed rather than handed on
   - a struct rename in Epic 5's own commit that hid 543 of 543 integration
     targets behind a compile error
   - the two disagreeing \`.MOD\`-chain token derivations the corpus sweep caught
     once enrichment made the records measurable
   Process comes second.

   **The second story is the throughput arc**, with denominators:
   32 -> 6,940 -> 7,939 -> 8,255 -> 8,263 -> 8,291 -> 8,330 of 8,330 examined;
   disagreements 26 -> 4 -> 1 -> 0.
   **Ten dispatch waves, nine correct halts, one bundle.**

   Lessons, each with its ENFORCING MECHANISM (a lesson without a mechanism is a
   quote — decisions.md §4):
   a. **Measure per-unit cost before a population-scoped run.** Mechanism: a
      required dispatch-brief field — measured cost, population, projected wall
      time — filled before the run starts.
   b. **A remainder named per-MECHANISM is closable; "the rest" is not.**
      Mechanism: a required per-shape enumeration of anything unexamined.
   c. **A lane's status must be a mechanical function of its row count.**
      Mechanism: the scan counts rows and derives the unexamined SET.
   d. **Coverage growth surfaces defects late.** Disagreements appeared only in
      newly-covered shapes; a bundle stopping at 95% would have shipped them.
   e. **A method carried past its limit is this bundle's recurring failure
      shape.** Mechanism: change the method and RE-RUN EVERYTHING IT ALREADY
      JUDGED; the scan verifies that re-run.
   f. **A blocker whose fix lives in another subsystem is still a fix.**
      Mechanism: \`blocker-closure-doctrine.md\`'s two dispositions, enforced by
      the scan reading \`## Open blockers\`. This bundle filed three blockers and
      cleared all three by decomposing them.
   g. **A count change compiles clean and leaves other assertions red.**
      Mechanism: a required count sweep across \`tests\`/\`src\`/\`apps\`/\`scripts\`.
   h. **Verify at the widest build scope the repo has.** Mechanism:
      \`cargo test --no-run\` plus the full workspace run in the scan, and
      \`apps/desktop/src-tauri\` tested explicitly as the separate workspace it is.
   i. **A lane's attribution of a failure is a claim, not evidence.** Mechanism:
      the scan re-derives attribution from \`git\` against the cut SHA. It caught
      one false "pre-existing" claim and confirmed a true one covering 31 suites.
   j. **A vacuous pass is not a pass.** The corpus sweep was green on records
      whose \`raw_tokens\` were \`[]\` — its population was "tokens the record
      claims", and a record claiming nothing cannot mismatch. Enrichment made
      them measurable and the disagreement appeared instantly. Mechanism: state
      the population of every gate, and treat an empty population as unmeasured
      rather than passing.
   Record also that the gate's own scan SCOPE was a defect, found by the scanner
   and closed by an instrument lane.

   Say plainly: nine scans failed and each failure was the system working. The
   lanes wrote honest short rows rather than false greens — with two exceptions,
   both caught mechanically: wave 2's equipment lane claiming \`complete\` over
   103 of 494 (caught by row-counting), and wave 7's mis-attribution (caught by
   re-deriving from \`git\`).

   Close out workflow-instruction.md §12 rows 3 and 8, both UNENFORCED at launch.

2. CITE IT from \`${PKG}/references/README.md\` IN THIS SAME CYCLE.

3. **REGISTER THE INHERITED DEBT.** 31 of 599 test suites carrying 49 of 8,023
   failures were PROVEN pre-existing at the \`tranche/13\` cut and are genuinely
   outside this bundle's DoD. They must not simply vanish from the record:
   add them to \`${PKG}/forward-scope-register.md\` with the proof command, the
   counts and their denominators, and the fact that this bundle verified their
   inheritance rather than assuming it. Reporting inherited red with proof is a
   legitimate disposition; forgetting it is not.

4. FULL WORKTREE/BRANCH SWEEP. \`git worktree list\`, \`git branch -a\`, \`df -h /\`.
   Report count FOUND vs REMOVED. Ten dispatch runs left worktrees behind
   (\`.claude/worktrees/wf_*\`). Check each for unmerged commits BEFORE removing.
   NEVER remove a \`locked\` worktree or one carrying unmerged commits.

NO PR IN THIS CYCLE. Steps 2-4 land before the PR opens.

Receipts to \`${PKG}/artifacts/epic-6-closure/\`. Commit, push (§5), mark row 20
complete. Run \`scripts/verify.sh --only denominator-gate\` before pushing.

Return ONLY JSON:
{"criterion":"AT-33-E6-002","status":"complete"|"blocked-escalated",
 "retro_path":"...","cited_from":"...","retro_summary_figures":[...],
 "defects_found_and_fixed":[{"defect":"...","units":0,"commit":"..."}],
 "deferrals_field_corrected":true|false,"lessons_with_mechanisms":[...],
 "inherited_debt_registered":true|false,"forward_scope_entry":"...",
 "sweep":{"worktrees_found":0,"worktrees_removed":0,"branches_found":0,"branches_removed":0,"kept_locked":[...],"kept_unmerged":[...]},
 "unenforced_rows_closed":{"row3":"...","row8":"..."},
 "denominator_gate":"PASS"|"FAIL",
 "receipt_paths":[...],"commit_shas":[...]}`
}

function architectureDocsGraphifyPrPrompt(prior) {
  return `${standingRules('sd33-r10-archdocs-pr')}

## YOUR CRITERION: AT-33-E6-003 (part 1) — architecture docs, graphify, PR (kanban row 21)

Retrospective, inherited-debt registration, and sweep are DONE (that order is
load-bearing). Prior cycle:
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
   - the corpus-extraction fix for \`.MOD\`-attached EQMOD references, the
     \`enrich_equipment_raw_tokens\` path, and its reconciliation with
     \`corpus_literal_sweep\`'s independent closure-builder
   - the L20-per-class pilot-build probe path
   - \`box_ledger.py\` and the THE-BOX partition; the denominator gate in
     \`verify.sh\` and its widened scan scope
   - formula-interpreter corpus-wide coverage; work-inventory \`unknown\` -> zero
     and the doneness mapping that reclassification required
2. GRAPHIFY per template §6.
3. OPEN THE PR: ${BRANCH} -> develop. Lead with the DEFECTS FOUND AND FIXED, cite
   the retrospective and receipts, state headline figures WITH DENOMINATORS, and
   say plainly that nine final-acceptance scans failed and what each wave closed.
   Note the 31 inherited failing suites with their proof, so a reviewer is not
   surprised by a non-green workspace run. That history is the strongest
   evidence the gate works.
4. Resolve merge conflicts if any. NEVER force-push. DO NOT MERGE — the operator
   merges tranche -> develop.

Commit and push (§5). Do NOT mark row 21 complete; release-notes owns it.

Return ONLY JSON:
{"criterion":"AT-33-E6-003-part1","status":"complete"|"blocked-escalated",
 "arch_docs_touched":[...],"graphify":"...","pr_url":"...","pr_number":0,
 "inherited_suites_noted_in_pr":true|false,
 "conflicts_resolved":"...","commit_shas":[...]}`
}

function releaseNotesVersionBumpPrompt(prPrior) {
  return `${standingRules('sd33-r10-release-notes')}

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
   Watch for bare hundred-percent tokens — the gate has caught six agents.

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

log('SD-33 REMEDIATION WAVE 9. Everything green except corpus_literal_sweep: 105 findings across 10 of 137 changed records. Two independent .MOD-chain token derivations disagree — enrich_equipment_raw_tokens.rs (new this bundle) vs corpus_literal_sweep.rs (unchanged since the cut). Settle it against the pinned oracle source.')

phase('Corpus sweep green')
const sweep = await agent(corpusSweepPrompt(), { model: 'sonnet', label: 'corpus-sweep-green', phase: 'Corpus sweep green' })

phase('Epic 6 — Closure epilogue')
const scan = await agent(finalAcceptanceScanPrompt(sweep), {
  model: 'opus', label: 'final-acceptance-scan-attempt10', phase: 'Epic 6 — Closure epilogue',
})

// Match the scan's own gate/status FIELDS, not a loose mention of the words.
// The first run of this wave halted spuriously: the scan returned gate PASS with
// zero shortfalls, but the old bare /blocked-escalated/ test matched that string
// inside a sentence stating rows are NOT blocked-escalated.
const scanText = String(scan)
const scanFailed = !scan
  || /"gate"\s*:\s*"FAIL"/.test(scanText)
  || /"status"\s*:\s*"blocked-escalated"/.test(scanText)
  || !/"gate"\s*:\s*"PASS"/.test(scanText)
if (scanFailed) {
  log('AT-33-E6-001 attempt 10 did NOT pass. Per §11 step 1: no retrospective, no sweep, NO PR. Correct outcome, not a failure.')
  return {
    bundle: 'SD-33', wave: 9, closed: false,
    halted_at: 'AT-33-E6-001 final-acceptance scan (attempt 10)',
    scan, sweep,
  }
}

const retroSweep = await agent(retrospectiveAndSweepPrompt(scan), { model: 'sonnet', label: 'retrospective-and-sweep', phase: 'Epic 6 — Closure epilogue' })
const archPr = await agent(architectureDocsGraphifyPrPrompt(retroSweep), { model: 'sonnet', label: 'archdocs-graphify-pr', phase: 'Epic 6 — Closure epilogue' })
const notes = await agent(releaseNotesVersionBumpPrompt(archPr), { model: 'haiku', label: 'release-notes-version-bump', phase: 'Epic 6 — Closure epilogue' })

log('SD-33 closure epilogue complete. The operator merges tranche/13 -> develop.')

return {
  bundle: 'SD-33', wave: 9, branch: BRANCH, closed: true,
  sweep, epic6: { scan, retroSweep, archPr, notes },
}
