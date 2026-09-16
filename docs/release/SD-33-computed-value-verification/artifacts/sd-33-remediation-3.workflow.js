export const meta = {
  name: 'sd-33-remediation-3',
  description: 'SD-33 remediation wave 3 — close the last 391 equipment bonus shapes, widen the gate scope, re-run closure',
  whenToUse: 'After sd-33-remediation-2 halted at AT-33-E6-001 attempt 3 with 391 of 8,330 units unexamined.',
  phases: [
    { title: 'Bonus-shape lanes — the last 391' },
    { title: 'Epic 5 finalize' },
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
  return `You are a dispatched SD-33 REMEDIATION WAVE 3 agent. Role: ${role}.

CONTEXT. SD-33 has run three dispatch waves and been halted three times by its
own final-acceptance scan. Every halt was correct. Epic 5 has moved
32 -> 6,940 -> 7,939 of 8,330 units examined. **391 of 8,330 remain unexamined.**
They are ALL equipment bonus shapes, and they are ALL named. That is your target.

Everything else in the bundle is closed: Epics 1/2/3/4, the denominator gate
(green, detection re-proven live), the deferral posture (2 open of 8, both
genuine capability deferrals with named revisit conditions, 0 covering DoD
scope), zero reasonless \`unverifiable\` rows, zero unresolved disagreements, zero
duplicate unit_ids. Do not re-litigate any of it.

REQUIRED READS, in order:
1. ${REPO}/CLAUDE.md
2. ${REPO}/AGENTS.md
3. ${REPO}/${PKG}/workflow-instruction.md   (dispatch procedure — binding)
4. ${REPO}/${PKG}/epic-breakdown.md          (AT-33-E5-002, verbatim)
5. ${REPO}/${E5DIR}/AT-33-E5-remainder-equipment_cycle_receipt.md
   (wave-2's equipment lane — it enumerated your shapes with counts. That
   enumeration is good work and is your starting map.)
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
- NEVER hand-edit data/corpus/**. NEVER pass --allow-stamp-loss.

RETRO EVENTS (§2.3): emit via \`scripts/retro.py\` as they happen. \`--verified-by\`
required on a \`correction\`.

FIGURES (decisions.md §2): every number states its denominator in the SAME
construct. \`scripts/verify.sh --only denominator-gate\` is GREEN and WILL run
against your prose. Bare hundred-percent tokens are caught specifically.

NO STUBS. NO SAMPLING PRESENTED AS POPULATION. NO DoD-SCOPE DEFERRALS.

## THE ONE NEW RULE THIS WAVE — YOUR STATUS MUST MATCH YOUR ROW COUNT

Wave 2's equipment lane returned \`"status":"complete"\` having examined 103 of its
own 494-unit population. The scan caught it by COUNTING ROWS. That is the first
over-claim in this bundle, and it is the single thing that must not happen again.

**Your \`status\` field is a mechanical function of your row count, not a judgment
about your effort.** Before you write your return JSON, run the count yourself:

    python3 -c "import json;d=json.load(open('<your results file>'));print(len(d['results']))"

If that number is less than your population, your status is
\`blocked-escalated\` and your \`examined\` field says exactly what you got. An
honest short report is a CORRECT outcome — the wave-1 lanes wrote honest
\`in-progress\` rows and that is precisely why this bundle can still close
truthfully. A \`complete\` over a partial population is the only unrecoverable
move here.`
}

const SHARED_BRIEF = `
## THROUGHPUT — measure before you run

Attempt 1 hand-authored one PCGen character per unit and reached 32 of 8,330.
Generators got it to 7,939 of 8,330. Your populations are small (about 100-130
units each), so setup cost dominates. Reuse, do not rebuild:
- \`scripts/oracle_harness/\` — the proven batch harness and export path
- the wave-1/wave-2 equipment probe binaries under \`src/bin/\`
- \`${E5DIR}/equipment*.oracle-results.json\` — the working row format

Measure the real per-unit cost on ~20 units, multiply by YOUR population, and
state the projected wall time in your reasoning BEFORE the full run. One
exported character carrying N computed variables verifies N units per JVM start
— that lever still applies.

## VERDICT DISCIPLINE

\`unverifiable\` is a FIRST-CLASS verdict (AT-33-E2-003) and correct when a unit
genuinely has no comparable computed magnitude — e.g. \`no_bonus_chain\` (empty
\`raw_bonus_chains\`, nothing computed) or \`no_probe_surface\`
(AT-33-E1-003 says \`probe_exists: false\` for the kind). Reuse that established
vocabulary; do not invent a parallel one.

**\`unverifiable\` is not a parking space for a unit you could not reach.** A unit
never fed to the harness is neither \`agree\` nor \`unverifiable\` — it is
unexamined, and writing it under a verdict would be exactly the "we did not look"
bucket AT-33-E4-003 forbids. Every \`unverifiable\` row MUST carry a populated
reason field; the current files have zero reasonless rows and must stay that way.

**A bonus shape that is genuinely not comparable is a real finding, not a
failure.** Some of these shapes may be situational or conditional in a way that
has no single scalar to compare — say so per unit, with the reason, and that
unit is legitimately done.

## READ THE WHOLE CORPUS RECORD

A grep narrowed to BONUS/PRE hides STACK/MULT and other application-governing
fields, and those fields change the computed value. Read the whole record for
every shape you handle.

## GENERIC PASS, BY MECHANISM

Group your units by their actual bonus-chain shape, count each group, put that
table in your receipt, then handle them by group. Do not open a sub-lane per
item.
`

function lanePrompt(c) {
  return `${standingRules(c.role)}

## YOUR SLICE: ${c.title}
**Population: ${c.population} units**, all \`literal-verified\` equipment, by shape:
${c.shapes}

These are ${c.population} of the 391 unexamined units, which are 391 of the
6,589 \`literal-verified\` population (itself 6,589 of Epic 5's 8,330 =
1,741 fixture-verified + 6,589 literal-verified).

${c.diagnosis}

${SHARED_BRIEF}

## WHAT "DONE" MEANS FOR YOU
Every one of your ${c.population} units carries a per-unit
\`(ours, oracle, verdict)\` row in your committed results JSON, with a populated
reason on every \`unverifiable\`. Nothing less is \`complete\`.

**Coordinate:** sibling lanes are running RIGHT NOW in parallel on the other
shapes of the same 391, plus one instrument lane. You share \`${E5DIR}/\`,
\`progress.md\`, and \`kanban.md\`. Write your rows ONLY to your own results file
(named below) — never rewrite a sibling's file or the merged
\`literal-verified.oracle-results.json\`. Re-read before every shared edit, rebase
before every push, never revert a sibling's rows.

**Do NOT mark kanban rows 16/17/18.** A finalize cycle runs after all lanes and
owns that call. Report your numbers; let it total them.

## WRITE SCOPE
${c.scope}
- your results file: \`${E5DIR}/${c.resultsFile}\`
- your receipt: \`${E5DIR}/${c.receiptFile}\`

## PER-CYCLE PROCEDURE (§6) — all nine steps
1. Rebase onto ${BRANCH} (§5), then verify the base is real:
     test -d docs && test -d data && test -d scripts \\
       || { echo 'WRONG BASE — reset before continuing'; exit 1; }
2. Define the audit base once, then run both greps:
     BASE_BRANCH=$(git merge-base HEAD origin/develop)
     git diff --unified=0 "\${BASE_BRANCH}...HEAD" -- <your scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \\
       | grep -nE '\\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
     git diff --unified=0 "\${BASE_BRANCH}...HEAD" -- <your scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \\
       | grep -nE '\\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\\b' || echo 'OK_NO_TOKENS'
   The trailing \\b is deliberately omitted from the first pattern. Do not add it.
3. TDD: RED -> confirm it fails FOR THE INTENDED REASON -> GREEN -> scoped suite.
4. Re-run both audits on the final diff.
5. Write your receipt using EXACTLY this schema — \`- **Status:** <x>\` is a
   BULLET, not a \`## Status\` heading:

${RECEIPT_SCHEMA}

6. Commit, push via §5.
7. Update progress.md via §5 (re-read first). Kanban Notes are a POINTER only.
8. Count your rows mechanically and set your status from that count (see the
   rule at the top of this brief).
9. Report.

## RETURN VALUE — ONLY JSON, no prose:
{"lane":"${c.lane}","status":"complete"|"blocked-escalated",
 "population":${c.population},"rows_written":0,"examined":0,
 "agree":0,"disagree":0,"unverifiable":0,"unexamined":0,
 "reasonless_unverifiable":0,
 "row_count_command_output":"<the literal output of your own len() count>",
 "denominator_statement":"<X of ${c.population} ${c.lane} units>",
 "shape_table":[{"shape":"...","population":0,"examined":0,"verdicts":{}}],
 "results_file":"${E5DIR}/${c.resultsFile}",
 "per_unit_cost_seconds":0,"units_per_character":0,"projected_vs_actual":"...",
 "disagreements":[{"unit_id":"...","ours":"...","oracle":"..."}],
 "commit_shas":[...],"receipt_path":"...","red_green":"...",
 "identifier_audit":"...","wired_audit":"...","next":"..."}`
}

// ---------------------------------------------------------------------------
// Lanes. The 391 split by bonus-shape family; plus one disjoint instrument lane.
// ---------------------------------------------------------------------------

const LANES = [
  {
    lane: 'var-bonus-shape',
    role: 'sd33-r3-var',
    title: 'VAR-driven bonus shapes',
    population: 108,
    shapes: '- `VAR` — 108 of 391',
    resultsFile: 'equipment-shape-var.oracle-results.json',
    receiptFile: 'AT-33-E5-shape-var_cycle_receipt.md',
    diagnosis: `## THE SHAPE
\`BONUS:VAR|...\` chains compute against a named variable rather than a fixed
stat slot. The value depends on what that variable resolves to at evaluation
time, which is why the wave-2 equipment path did not handle them.

Establish, by reading the corpus records and PCGen's own variable resolution:
- what each \`VAR\` name resolves to, and whether our engine resolves it the same way
- whether the variable is character-dependent (needs a built character to have a
  value) or item-local
Where PCGen and our engine disagree about what a variable resolves to, that is a
DISAGREEMENT to root-cause — not a mapping quirk to smooth over.

A \`VAR\` whose value is genuinely indeterminate without a character context is
\`unverifiable\` WITH THAT NAMED REASON, and that is a legitimate finding.`,
    scope: `- \`scripts/oracle_harness/\` — export-template widening for VAR tokens
- \`src/rules_core/\` — only if a VAR shape is genuinely unhandled by our compute path; a compute change is a real defect fix with its own RED->GREEN
- \`src/bin/\` — extend the existing equipment probe binary; do not fork it`,
  },
  {
    lane: 'combat-weapon-shape',
    role: 'sd33-r3-combat',
    title: 'Combat and weapon bonus shapes',
    population: 125,
    shapes: '- `COMBAT` — 92 of 391\n- `WEAPON` — 18 of 391\n- `WEAPONPROF=*` — 15 of 391',
    resultsFile: 'equipment-shape-combat.oracle-results.json',
    receiptFile: 'AT-33-E5-shape-combat_cycle_receipt.md',
    diagnosis: `## THE SHAPE
Three related families, all attaching to the combat path rather than an item's
own stat slot: \`BONUS:COMBAT|...\` (attack/damage/AC/initiative subtokens),
\`BONUS:WEAPON|...\`, and \`BONUS:WEAPONPROF=*|...\` (scoped to a proficiency).

Enumerate the SUBTOKENS first — \`COMBAT|TOHIT\`, \`COMBAT|DAMAGE\`, \`COMBAT|AC\`,
\`COMBAT|INITIATIVE\` and whatever else is actually present — count each, and put
that table in the receipt. The subtoken determines which oracle export token
carries the comparable value, so the mapping is per subtoken, not per family.

\`WEAPONPROF=*\` is proficiency-scoped: the bonus applies only for weapons in that
proficiency group. Decide deliberately what the comparable magnitude is, say so
in the receipt, and be consistent across all 15.

These need a character with the item equipped for the oracle to compute a combat
number at all. Amortise the build: one character, many items, many tokens per
export.`,
    scope: `- \`scripts/oracle_harness/\` — combat/weapon export-template widening and the build generator
- \`src/rules_core/\` — only for a genuinely unhandled shape; RED->GREEN required
- \`src/bin/\` — extend the existing equipment probe binary`,
  },
  {
    lane: 'stat-save-situation-shape',
    role: 'sd33-r3-statsave',
    title: 'Stat-slot, save, situational and the long tail',
    population: 158,
    shapes: '- `STAT_multi_or_other_slot` — 43 of 391\n- `SITUATION` — 34 of 391\n- `SAVE` — 24 of 391\n- 11 smaller shapes — 20 of 391 combined\n- plus any shape the sibling lanes do not claim (you own the tail; see below)',
    resultsFile: 'equipment-shape-stat-save-tail.oracle-results.json',
    receiptFile: 'AT-33-E5-shape-stat-save-tail_cycle_receipt.md',
    diagnosis: `## THE SHAPES
Four groups, plus the tail:
- \`STAT_multi_or_other_slot\` (43) — items bonusing several stats at once, or a
  slot the wave-2 single-stat path did not model. The multi-stat case may need
  several comparable values per unit; decide what one row means for such a unit,
  say so, and be consistent.
- \`SITUATION\` (34) — conditional bonuses. Many will have no unconditional
  scalar to compare. Where that is genuinely true it is \`unverifiable\` with that
  named reason, per unit — a real finding, not a skip. Where a condition CAN be
  set up in the oracle character, set it up and compare.
- \`SAVE\` (24) — Fortitude/Reflex/Will. Straightforward once a character exists.
- The 11 smaller shapes (20 combined) — handle each, however few units it has.

## YOU OWN THE TAIL
Your first action is to re-derive the FULL list of unexamined literal-verified
unit_ids, subtract the shapes your two sibling lanes own
(\`VAR\`; \`COMBAT\`/\`WEAPON\`/\`WEAPONPROF=*\`), and confirm the remainder equals your
${'158'} figure. **If it does not, YOUR population is whatever that subtraction
actually yields** — say so with the command, and cover all of it. The 391 total
and its shape split come from wave 2's enumeration; re-derive rather than
inherit. An unexamined unit that falls in no lane's shape list is exactly how a
remainder survives another wave.`,
    scope: `- \`scripts/oracle_harness/\` — stat/save/situation export-template widening
- \`src/rules_core/\` — only for a genuinely unhandled shape; RED->GREEN required
- \`src/bin/\` — extend the existing equipment probe binary`,
  },
]

// ---------------------------------------------------------------------------
// Instrument lane — disjoint files, runs alongside.
// ---------------------------------------------------------------------------

function gateScopePrompt() {
  return `${standingRules('sd33-r3-gate-scope')}

## YOUR TASK — close the denominator gate's scan-scope gap (instrument correction)

The attempt-3 scan found and recorded this, as an instrument-correction retro
event:

> a first probe at the bundle root was NOT scanned (files_checked stayed 23) —
> \`DEFAULT_GLOBS\` covers only \`artifacts/**/*_cycle_receipt.md\` + \`progress.md\`

So a percentage stated without its denominator in \`README.md\`, \`decisions.md\`,
\`epic-breakdown.md\`, \`release-notes.md\`, \`scope-draft.md\`, \`kanban.md\`, or
\`THE-BOX.md\` is invisible to the gate. AT-33-E1-004's whole point is that the
gate catches the defect; a gate that cannot see the bundle's headline documents
is catching it in the least likely place.

\`DEFAULT_GLOBS\` is at \`scripts/denominator_gate.py:97\`, and the module docstring
around line 80 already anticipates this: *"A later bundle extends DEFAULT_GLOBS
(or passes its own paths / sets DENOMINATOR_GATE_PATHS, the env var
scripts/verify.sh's stage reads)."* You are that later bundle.

## WHAT TO DO
1. Widen the scanned scope to cover the SD-33 package's markdown documents, not
   just receipts and progress.md. Follow the extension mechanism the docstring
   names — do not invent a second one.
2. Run it. **Expect new violations** in documents never scanned before. Fix each
   one the honest way: if the construct genuinely states its denominator, that
   is a matcher false positive and the MATCHER is wrong; if it does not, the
   PROSE is wrong. Per violation, say in the receipt which you concluded and why.
3. **Do not widen the scope and then relax the matcher until it passes.** That
   is the failure mode this bundle exists to prevent. If widening surfaces more
   real violations than you can fix in this turn, fix what you can, report the
   count honestly, and return \`blocked-escalated\` — do NOT narrow the scope back.
4. RE-PROVE DETECTION after your change, live, inside the real scope:
   - a receipt with a bare percentage and no denominator -> FAILS
   - a bare hundred-percent token -> FAILS
   - the corrected form -> passes
   Remove your probe files afterwards and show the baseline back at 0 violations.
   A gate relaxed until nothing trips it has stopped being a gate (AT-33-E1-002).
5. \`scripts/verify.sh --only denominator-gate\` must exit 0 at the end. Also run
   \`scripts/verify.sh\` in full and REPORT any other red stage — do not fix those.

## COORDINATION
Three sibling lanes are writing to \`${E5DIR}/\` right now. Your files are
\`scripts/denominator_gate.py\`, its tests, and your own receipt — disjoint from
theirs. You WILL be scanning files they are actively rewriting; scan and fix
against what is committed at your rebase point, and note in the receipt that
later prose is the finalize cycle's responsibility to keep green.

## PROCEDURE
§6, all nine steps, §7 schema with \`- **Status:** <x>\` as a BULLET.
Receipt: \`${PKG}/artifacts/epic-1-instruments/AT-33-E1-004-scope-widening_cycle_receipt.md\`
Kanban row 4 stays \`complete\`; append a pointer to its Notes (pointer only).

## RETURN VALUE — ONLY JSON:
{"lane":"gate-scope","status":"complete"|"blocked-escalated",
 "globs_before":[...],"globs_after":[...],
 "files_checked_before":0,"files_checked_after":0,
 "new_violations_found":0,"violations_remaining":0,
 "dispositions":[{"file":"...","line":0,"verdict":"false-positive"|"real-violation","fix":"matcher"|"prose"}],
 "detection_reproven":true|false,"red_green":"...",
 "verify_sh_full_result":"...","other_red_stages":[...],
 "commit_shas":[...],"receipt_path":"..."}`
}

// ---------------------------------------------------------------------------
// Finalize.
// ---------------------------------------------------------------------------

function finalizePrompt(laneResults) {
  return `${standingRules('sd33-r3-e5-finalize')}

## YOUR JOB: total Epic 5 and own the kanban call on rows 16, 17, 18

Wave 3's lanes just ran against the last 391 unexamined units. Reports:
${JSON.stringify(laneResults).slice(0, 8000)}

**Reports are not evidence.** Wave 2's equipment lane returned
\`"status":"complete"\` over 103 of 494 units. Derive every number yourself by
COUNTING ROWS in the committed files. Trust nothing above.

## WHAT YOU MUST ESTABLISH
1. **Merge every lane results file** into the canonical artifacts:
   - \`${E5DIR}/fixture-verified.combined-oracle-results.json\` -> exactly 1,741 rows
   - \`${E5DIR}/literal-verified.oracle-results.json\` -> exactly 6,589 rows
   - \`${E5DIR}/AT-33-E5-003.combined-oracle-results.json\` -> exactly 8,330 rows
   Merge on \`unit_id\`. **A duplicate \`unit_id\` across lanes is a real finding** —
   two lanes disagreeing about one unit must be root-caused, never resolved by
   last-writer-wins. The fixture file is already at 1,741 of 1,741 and must stay
   there; do not disturb it.
2. **Re-derive the unexamined set explicitly**, do not infer it from a count:
   take the full \`literal-verified\` unit_id set from \`docs/work-inventory.json\`,
   subtract the unit_ids present in the merged file, and print the difference.
   It must be empty. If it is not, print the missing ids and their shapes — that
   list is the honest next-cycle target and your status is \`blocked-escalated\`.
3. **Zero reasonless \`unverifiable\` rows** across all three files. Re-derive.
4. **Zero unresolved \`disagree\` rows.** Any NEW disagreement from this wave is
   root-caused: either our computation is wrong (fix it) or the harness is wrong
   (fix it AND re-run everything it already judged). A disagreement is NEVER
   closed by adjusting the expectation to match our output. One entry per
   disagreement in \`progress.md\`. A filed blocker does not satisfy AT-33-E5-003.
5. **Re-prove \`disagree\` capability on the current batch path.** A
   zero-disagreement result across 8,330 is suspicious, not happy. Feed a
   known-disagreeing case through the CURRENT path, show it returning
   \`disagree\`, then remove the probe.
6. **Keep the gate green.** The gate-scope lane widened what
   \`scripts/denominator_gate.py\` scans, so documents never checked before are now
   in scope — including ones the lanes and you are rewriting. Run
   \`scripts/verify.sh --only denominator-gate\` before you push and fix your own
   prose if it trips. Do not narrow the scope to pass.

## THEN THE KANBAN CALL
Mark rows 16, 17, 18 \`complete\` **only if** each population is fully rowed with
its denominator stated. If any is short, leave the row honest and report — three
prior waves wrote honest short rows and that is why this bundle can still close
truthfully. Update the AT-33-E5-00{1,2,3} receipts' figure rows to final totals.

## PROCEDURE
§6, all nine steps, §7 schema with \`- **Status:** <x>\` as a BULLET (the wave-2
finalize receipt drifted to \`## Status:\` — do not repeat that).
Receipt: \`${E5DIR}/AT-33-E5-finalize-wave3_cycle_receipt.md\`

## RETURN VALUE — ONLY JSON:
{"task":"e5-finalize-wave3","status":"complete"|"blocked-escalated",
 "fixture":{"population":1741,"rows":0,"distinct":0,"agree":0,"disagree":0,"unverifiable":0,"unexamined":0},
 "literal":{"population":6589,"rows":0,"distinct":0,"agree":0,"disagree":0,"unverifiable":0,"unexamined":0},
 "combined":{"population":8330,"rows":0,"distinct":0,"agree":0,"disagree":0,"unverifiable":0,"unexamined":0},
 "missing_unit_ids":[...],"duplicate_unit_ids":[...],"reasonless_unverifiable":0,
 "new_disagreements":[{"unit_id":"...","rootcause":"...","resolution":"..."}],
 "disagree_capability_reproven_on_batch_path":true|false,
 "kanban":{"row16":"complete"|"in-progress","row17":"...","row18":"..."},
 "denominator_gate":"PASS"|"FAIL",
 "commit_shas":[...],"receipt_path":"..."}`
}

// ---------------------------------------------------------------------------
// Epic 6, attempt 4.
// ---------------------------------------------------------------------------

function finalAcceptanceScanPrompt(summary) {
  return `${standingRules('sd33-r3-acceptance-scan')}

## YOUR CRITERION: AT-33-E6-001 — final-acceptance scan, ATTEMPT 4 (kanban row 19)

Three prior scans FAILED and correctly halted the bundle. Read attempt 3's
receipt: \`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt3_cycle_receipt.md\`

Attempt 3 confirmed CLOSED and you should re-verify rather than re-investigate:
row 16 at 1,741 of 1,741; the denominator gate green with detection re-proven;
the deferral posture (2 open of 8, both genuine capability deferrals with named
revisit conditions, 0 covering DoD scope); 0 reasonless \`unverifiable\`;
0 \`disagree\`; 0 duplicate unit_ids; \`box_ledger.py --check\` exit 0; work-inventory
\`unknown\` at 0; Epic 3's artifact at the SD-33 path with SD-32's untouched.

Attempt 3's surviving shortfall was one thing: **391 of 6,589 literal-verified
units unexamined**, leaving rows 17 and 18 at \`in-progress\`. Three shape lanes,
one instrument lane, and a finalize cycle have since run. Reports:
${JSON.stringify(summary).slice(0, 7000)}

That is a REPORT, not evidence.

## THE CHECK THAT MATTERS MOST — COUNT THE ROWS
Wave 2's equipment lane returned \`complete\` over 103 of 494. You caught it by
counting. Count again:
    python3 -c "import json,collections
for p,pop in [('fixture-verified.combined-oracle-results.json',1741),
              ('literal-verified.oracle-results.json',6589),
              ('AT-33-E5-003.combined-oracle-results.json',8330)]:
  d=json.load(open('${E5DIR}/'+p))
  k='results' if 'results' in d else [x for x in d if isinstance(d[x],list)][0]
  r=d[k]; print(p,'rows',len(r),'distinct',len({x.get('unit_id') for x in r}),'pop',pop,
    dict(collections.Counter(x.get('verdict') or x.get('status') for x in r)))"

Every population must equal its denominator EXACTLY. Then go further than a
count: **derive the unexamined SET, not just its size** — take the
literal-verified unit_id set from \`docs/work-inventory.json\`, subtract the merged
file's unit_ids, and confirm the difference is empty. A count can match while the
membership does not.

Also verify: 0 reasonless \`unverifiable\`; 0 unresolved \`disagree\`; no duplicate
\`unit_id\`; the batch path still demonstrably returns \`disagree\` on a known case.

## THE INSTRUMENT LANE'S CHANGE — SCRUTINISE IT
A lane widened \`scripts/denominator_gate.py\`'s \`DEFAULT_GLOBS\`. Verify it was
widened and NOT quietly narrowed back, and that the matcher was not relaxed to
absorb the new violations. Re-prove detection live inside the real scope (a bare
percentage with no denominator FAILS; a bare hundred-percent token FAILS; the
corrected form passes), then remove your probe and show the baseline at 0.
A gate that grew its scope while losing its teeth is a WORSE outcome than the
narrow gate it replaced — treat that as a blocking shortfall if you find it.

## THEN THE FULL SCAN
Every criterion AT-33-E1-001 .. AT-33-E5-003 \`complete\`; every kanban card rows
1-18 \`complete\` (19-21 are Epic 6's own). \`returned-to-backlog\`,
\`in-progress\`, \`blocked-escalated\`, or \`complete\`-with-a-deferred-half BLOCKS.
There is NO "complete OR filed under Open blockers".

Check the WORK, not the reports:
- \`git log\` and the actual target files per criterion
- every receipt exists at its kanban-stated path and matches §7, including the
  figures row (number + denominator + re-derive command) and the four-buckets row
- re-run the headline re-derive commands yourself
- grep the closure INSTRUMENTS for hardcoded exclusion lists — carve-outs hide in
  code, not prose
- \`python3 scripts/box_ledger.py --check\` exits 0
- \`jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json\` is 0
- Epic 3's corpus-wide artifact at the SD-33 path; SD-32's
  \`docs/release/SD-32-.../artifacts/gate-2-engines/\` file UNTOUCHED
- enumerate open deferrals; none defers DoD scope; all carry a revisit condition

IF ANYTHING IS SHORT: STOP. No retrospective, no sweep, NO PR. Report what is
short WITH THE COMMAND THAT SHOWS IT. That is a CORRECT outcome — three prior
scans did exactly that and all three were right. You are the scanner, not an
executor.

Receipt: \`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt4_cycle_receipt.md\`
Commit and push (§5). Mark kanban row 19 \`complete\` only on PASS.

Return ONLY JSON:
{"criterion":"AT-33-E6-001","attempt":4,"gate":"PASS"|"FAIL",
 "status":"complete"|"blocked-escalated",
 "row_counts":{"fixture":0,"literal":0,"combined":0},
 "unexamined_set_empty":true|false,"missing_unit_ids":[...],
 "gate_scope_widened_not_blinded":true|false,
 "prior_shortfalls_closed":[{"shortfall":"...","closed":true|false,"command":"...","output":"..."}],
 "shortfalls":[{"what":"...","command":"...","output":"..."}],
 "commands_rerun":[{"command":"...","output":"..."}],
 "receipt_path":"...","commit_shas":[...]}`
}

function retrospectiveAndSweepPrompt(scanResult) {
  return `${standingRules('sd33-r3-retro-sweep')}

## YOUR CRITERION: AT-33-E6-002 — retrospective written and cited (kanban row 20)
## PLUS §11.3 — full worktree/branch sweep

The final-acceptance scan PASSED on attempt 4:
${JSON.stringify(scanResult).slice(0, 4000)}

1. RETROSPECTIVE -> \`docs/retro/sd33-computed-value-verification-retrospective.md\`,
   grounded in \`python3 scripts/retro.py summary --since 2026-08-24 --json\` — READ
   that output. \`deferrals.open\` IS trustworthy (SD-32's fix landed:
   \`grep -n 'len(open_deferrals)' scripts/retro.py\`). Confirm it and say so.

   **The spine is the throughput arc**, told with denominators:
   - Epic 2 proved the oracle at n=1 — one hand-authored PCGen character.
   - Epic 5 carried that method to a population of 8,330 without measuring its
     per-unit cost first, and reached 32 of 8,330.
   - Scan 1 halted the bundle. Generators -> 6,940 of 8,330.
   - Scan 2 halted it. The 1,390 remainder was decomposed into three MECHANISMS
     (spell casting-ability mapping, equipment bonus shapes, full-character
     build) -> 7,939 of 8,330.
   - Scan 3 halted it. The last 391 were all equipment bonus shapes, split by
     shape family -> 8,330 of 8,330.
   **Four dispatch waves, three correct halts, one bundle.**

   Three lessons, each with its ENFORCING MECHANISM (a lesson without a
   mechanism is a quote — decisions.md §4):
   a. **Measure per-unit cost before a population-scoped run.** Mechanism: a
      required dispatch-brief field — measured cost, population, projected wall
      time — filled before the run starts.
   b. **A remainder named per-MECHANISM is closable; a remainder named as "the
      rest" is not.** Each wave shrank because the prior wave named its
      remainder honestly and specifically. Mechanism: the receipt's four-buckets
      row plus a required per-shape enumeration of anything unexamined.
   c. **A lane's status must be a mechanical function of its row count.** Wave
      2's equipment lane returned \`complete\` over 103 of 494; only row-counting
      caught it. Mechanism: the scan counts rows and derives the unexamined SET,
      never trusting a status field.
   Record also that the gate's scan SCOPE was itself a defect (\`DEFAULT_GLOBS\`
   could not see the bundle's headline documents) — found by the scanner, closed
   by an instrument lane.

   Say plainly: three scans failed and each failure was the system working. The
   lanes wrote honest short rows rather than false greens, and that is the only
   reason this closed truthfully.

   Close out workflow-instruction.md §12 rows 3 and 8, both UNENFORCED at launch:
   state whether they were closed and how, or why not.

2. CITE IT from \`${PKG}/references/README.md\` IN THIS SAME CYCLE.

3. FULL WORKTREE/BRANCH SWEEP. \`git worktree list\`, \`git branch -a\`, \`df -h /\`.
   Report count FOUND vs REMOVED. Four dispatch runs left worktrees behind
   (\`.claude/worktrees/wf_*\`). Check each for unmerged commits BEFORE removing.
   NEVER remove a \`locked\` worktree or one carrying unmerged commits — report those.

NO PR IN THIS CYCLE. Steps 2 and 3 land before the PR opens.

Receipts to \`${PKG}/artifacts/epic-6-closure/\`. Commit, push (§5), mark row 20
complete. Run \`scripts/verify.sh --only denominator-gate\` on your own prose
before pushing — its scope is now WIDER than in earlier waves.

Return ONLY JSON:
{"criterion":"AT-33-E6-002","status":"complete"|"blocked-escalated",
 "retro_path":"...","cited_from":"...","retro_summary_figures":[...],
 "deferrals_field_corrected":true|false,"lessons_with_mechanisms":[...],
 "sweep":{"worktrees_found":0,"worktrees_removed":0,"branches_found":0,"branches_removed":0,"kept_locked":[...],"kept_unmerged":[...]},
 "unenforced_rows_closed":{"row3":"...","row8":"..."},
 "denominator_gate":"PASS"|"FAIL",
 "receipt_paths":[...],"commit_shas":[...]}`
}

function architectureDocsGraphifyPrPrompt(prior) {
  return `${standingRules('sd33-r3-archdocs-pr')}

## YOUR CRITERION: AT-33-E6-003 (part 1) — architecture docs, graphify, PR (kanban row 21)

Retrospective and sweep are DONE (that order is load-bearing). Prior cycle:
${JSON.stringify(prior).slice(0, 3000)}

Follow \`${REPO}/docs/release/template/template.md §6\` — read it first.

1. ARCHITECTURE DOCS — \`docs/architecture/\` is CURRENT-STATE TRUTH. Refresh for
   what SD-33 actually changed: the PCGen oracle harness and its batch
   generators, the spell casting-ability mapping, the widened equipment
   bonus-shape coverage (VAR / COMBAT / WEAPON / STAT / SAVE / SITUATION), the
   L20-per-class pilot-build probe path, \`box_ledger.py\` and THE-BOX partition,
   the denominator gate in \`verify.sh\` and its widened scan scope,
   formula-interpreter corpus-wide coverage, and the work-inventory
   \`unknown\` -> zero classification.
2. GRAPHIFY per template §6.
3. OPEN THE PR: ${BRANCH} -> develop. The body cites the retrospective, the
   receipts, and the headline figures WITH THEIR DENOMINATORS. State plainly that
   three final-acceptance scans failed and what each remediation wave closed —
   that history belongs in the PR. It is the strongest evidence the gate works.
4. Resolve merge conflicts if any. NEVER force-push. DO NOT MERGE — the operator
   merges tranche -> develop.

Commit and push (§5). Do NOT mark row 21 complete; release-notes owns it.

Return ONLY JSON:
{"criterion":"AT-33-E6-003-part1","status":"complete"|"blocked-escalated",
 "arch_docs_touched":[...],"graphify":"...","pr_url":"...","pr_number":0,
 "conflicts_resolved":"...","commit_shas":[...]}`
}

function releaseNotesVersionBumpPrompt(prPrior) {
  return `${standingRules('sd33-r3-release-notes')}

## YOUR CRITERION: AT-33-E6-003 (part 2) — release notes + version bump (kanban row 21)

Housekeeping. The PR is open:
${JSON.stringify(prPrior).slice(0, 2000)}

1. Fill in \`${PKG}/release-notes.md\` for build \`0.13.0\` — what shipped, every
   figure stating its denominator in the same construct. No narrative ceremony.
2. Record the PR number in \`release-notes.md\` and in \`${PKG}/receipts.md\`.
3. VERSION BUMP: confirm \`apps/desktop/package.json\` and
   \`apps/desktop/src-tauri/tauri.conf.json\` BOTH read \`0.13.0\`. Stamped at the
   ${BRANCH} cut. DO NOT bump the tranche digit — it moves only on a NEW
   tranche/N branch cut, never on a bundle's own closure.
4. Placeholder sweep — every match resolves or is a documented deferral:
   grep -rn '<[a-z_-]*>' ${PKG}/*.md
5. Update \`${PKG}/progress.md\` to the closed state; mark kanban row 21 \`complete\`.
6. Re-run \`scripts/verify.sh --only denominator-gate\` on your own new prose and
   confirm it exits 0 before pushing. Its scope now includes the package's
   markdown documents, \`release-notes.md\` among them — so your own new prose IS
   scanned. Watch for bare hundred-percent tokens.

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

log('SD-33 REMEDIATION WAVE 3. Epic 5 stands at 7,939 of 8,330 examined. Closing the last 391 equipment bonus shapes: VAR 108 + COMBAT/WEAPON 125 + STAT/SAVE/SITUATION/tail 158. Plus one instrument lane widening the denominator gate scan scope.')

phase('Bonus-shape lanes — the last 391')
const allLanes = await parallel([
  ...LANES.map(c => () => agent(lanePrompt(c), {
    model: 'sonnet', label: c.lane, phase: 'Bonus-shape lanes — the last 391', isolation: 'worktree',
  })),
  () => agent(gateScopePrompt(), {
    model: 'sonnet', label: 'gate-scope-widening', phase: 'Bonus-shape lanes — the last 391', isolation: 'worktree',
  }),
])
log('Wave-3 lanes returned. Check `df -h /` and `git worktree list` (§8).')

phase('Epic 5 finalize')
const finalize = await agent(finalizePrompt(allLanes), {
  model: 'sonnet', label: 'e5-finalize-wave3', phase: 'Epic 5 finalize',
})

const summary = { lanes: allLanes, finalize }

phase('Epic 6 — Closure epilogue')
const scan = await agent(finalAcceptanceScanPrompt(summary), {
  model: 'opus', label: 'final-acceptance-scan-attempt4', phase: 'Epic 6 — Closure epilogue',
})

const scanFailed = !scan || /"gate"\s*:\s*"FAIL"/.test(String(scan)) || /blocked-escalated/.test(String(scan))
if (scanFailed) {
  log('AT-33-E6-001 attempt 4 did NOT pass. Per §11 step 1: no retrospective, no sweep, NO PR. Correct outcome, not a failure.')
  return {
    bundle: 'SD-33', wave: 3, closed: false,
    halted_at: 'AT-33-E6-001 final-acceptance scan (attempt 4)',
    scan, summary,
  }
}

const retroSweep = await agent(retrospectiveAndSweepPrompt(scan), { model: 'sonnet', label: 'retrospective-and-sweep', phase: 'Epic 6 — Closure epilogue' })
const archPr = await agent(architectureDocsGraphifyPrPrompt(retroSweep), { model: 'sonnet', label: 'archdocs-graphify-pr', phase: 'Epic 6 — Closure epilogue' })
const notes = await agent(releaseNotesVersionBumpPrompt(archPr), { model: 'haiku', label: 'release-notes-version-bump', phase: 'Epic 6 — Closure epilogue' })

log('SD-33 closure epilogue complete. The operator merges tranche/13 -> develop.')

return {
  bundle: 'SD-33', wave: 3, branch: BRANCH, closed: true,
  summary,
  epic6: { scan, retroSweep, archPr, notes },
}
