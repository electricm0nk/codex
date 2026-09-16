export const meta = {
  name: 'sd-33-remediation-2',
  description: 'SD-33 remediation wave 2 — close the last 1,390 Epic 5 units by mechanism, then re-run closure',
  whenToUse: 'After sd-33-remediation halted at AT-33-E6-001 attempt 2 with 1,390 of 8,330 units unexamined.',
  phases: [
    { title: 'Mechanism lanes — the last 1,390' },
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
  return `You are a dispatched SD-33 REMEDIATION WAVE 2 agent. Role: ${role}.

CONTEXT. SD-33 ran, halted at the final-acceptance scan, was remediated once, and
halted at that scan AGAIN. Both halts were correct. Epic 5 has moved 32 -> 6,940
of 8,330 units examined. **1,390 of 8,330 remain with no oracle row anywhere.**
You own one named slice of those 1,390. Nothing else about the bundle is your
concern — Epics 1/2/3/4 are closed, the denominator gate is green, and the two
surviving deferrals are legitimate capability deferrals.

REQUIRED READS, in order:
1. ${REPO}/CLAUDE.md
2. ${REPO}/AGENTS.md
3. ${REPO}/${PKG}/workflow-instruction.md   (dispatch procedure — binding)
4. ${REPO}/${PKG}/epic-breakdown.md          (AT-33-E5-001 / -002, verbatim)
5. ${REPO}/${E5DIR}/AT-33-E5-001_cycle_receipt.md
6. ${REPO}/${E5DIR}/AT-33-E5-002_cycle_receipt.md
   (the two lanes whose remainder you are closing — they named your slice, with
   its real structural reason and a concrete next-cycle plan. START from that
   plan; it is good work, not a dead end.)
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
- Re-read shared files (progress.md, kanban.md, THE-BOX.md) immediately before editing.
- NEVER hand-edit data/corpus/**. NEVER pass --allow-stamp-loss.

RETRO EVENTS (§2.3): emit via \`scripts/retro.py\` as they happen. \`--verified-by\`
required on a \`correction\`.

FIGURES (decisions.md §2): every number states its denominator in the SAME
construct. \`scripts/verify.sh --only denominator-gate\` is GREEN and WILL be run
against your prose — do not turn it red. Beware bare hundred-percent tokens; the
gate catches those specifically.

NO STUBS, NO SAMPLING PRESENTED AS POPULATION, NO DoD-SCOPE DEFERRALS. Per
${REPO}/docs/governance/blocker-closure-doctrine.md, a blocker larger than one
cycle is a SEQUENCING problem, not an exemption. Your slice was already
decomposed for you. Run it.`
}

const THROUGHPUT_BRIEF = `
## THROUGHPUT — the lesson that produced this wave, applied

Attempt 1 hand-authored one PCGen character per unit and reached 32 of 8,330.
The wave-1 lanes built generators and reached 6,940 of 8,330. That worked. Keep
doing it:

- **Measure first.** Run ~30 units, measure the real per-unit cost, multiply by
  YOUR population, and state the projected wall time in your reasoning BEFORE
  launching the full run. If the projection exceeds your turn, make the pipeline
  faster — bigger batches, more units per exported character, parallel JVMs.
  Do NOT sample and defer the rest.
- **One character can carry many units.** A template emitting N computed
  variables verifies N units per JVM start. This remains the biggest lever.
- **Reuse, do not rebuild.** \`scripts/oracle_harness/\` and the wave-1 batch
  generators and probe binaries already exist and are proven. Extend them.

## VERDICT DISCIPLINE — the thing that must not slip

\`unverifiable\` is a FIRST-CLASS verdict (AT-33-E2-003) and a legitimate result
when a unit genuinely has no comparable computed magnitude — e.g. wave 1's
\`no_bonus_chain\` (an empty \`raw_bonus_chains\`, a mundane item computing
nothing) and \`no_probe_surface\` (AT-33-E1-003 says \`probe_exists: false\` for
that kind). Those are real findings.

\`unverifiable\` is NOT a place to put a unit you could not get to. A unit you
never fed to the harness is neither \`agree\` nor \`unverifiable\` — it is
unexamined, and unexamined is exactly what this wave exists to eliminate. The
wave-1 lanes got this right and deliberately wrote NO row for their remainder.
Hold that line.

**Every \`unverifiable\` row MUST carry a populated reason field.** A known defect
in the current combined results: 319 of 6,940 rows carry \`unverifiable\` with no
reason field at all. If any of those fall in your slice's kinds, fix them as you
pass through — a reasonless \`unverifiable\` is indistinguishable from "we did not
look", which AT-33-E4-003's doctrine forbids.
`

function lanePrompt(c) {
  return `${standingRules(c.role)}

## YOUR SLICE: ${c.title}
**Population: ${c.population} units** — ${c.breakdown}

${c.diagnosis}

${THROUGHPUT_BRIEF}

## WHAT "DONE" MEANS FOR YOU
Every one of your ${c.population} units carries a per-unit
\`(ours, oracle, verdict)\` row in a committed results JSON, with a populated
reason on every \`unverifiable\`. Then update BOTH lane receipts' figures and, if
your slice was the last one outstanding for that row, mark the kanban row.

**Coordinate:** two sibling lanes are running RIGHT NOW in parallel on the other
slices of the same 1,390. You share \`${E5DIR}/\`, \`progress.md\`, \`kanban.md\`, and
both AT-33-E5-00{1,2} receipts with them. Write your rows to YOUR OWN results
file (named below) — do NOT rewrite theirs. Re-read before every shared edit,
rebase before every push, never revert a sibling's rows.

**Do NOT mark kanban rows 16/17/18 complete yourself.** A finalize cycle runs
after all three lanes and owns that call. Report your numbers; let it total them.

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
   BULLET, not a \`## Status\` heading (both prior waves drifted on this):

${RECEIPT_SCHEMA}

6. Commit, push via §5.
7. Update progress.md via §5 (re-read first). Kanban Notes are a POINTER only.
8. Report your slice's totals into both AT-33-E5-00{1,2} receipts' figure rows.
9. Report.

## RETURN VALUE — ONLY JSON, no prose:
{"lane":"${c.lane}","status":"complete"|"blocked-escalated",
 "population":${c.population},"examined":0,"agree":0,"disagree":0,"unverifiable":0,
 "unexamined":0,"reasonless_unverifiable":0,
 "denominator_statement":"<X of ${c.population} ${c.lane} units>",
 "results_file":"${E5DIR}/${c.resultsFile}",
 "per_unit_cost_seconds":0,"units_per_character":0,"projected_vs_actual":"...",
 "disagreements":[{"unit_id":"...","ours":"...","oracle":"..."}],
 "contributes_to":{"AT-33-E5-001":0,"AT-33-E5-002":0},
 "commit_shas":[...],"receipt_path":"...","red_green":"...",
 "identifier_audit":"...","wired_audit":"...","next":"..."}`
}

// ---------------------------------------------------------------------------
// The three mechanism lanes. One named defect -> one named owner (SD-32 lesson).
// Grouped BY MECHANISM, not per object kind.
// ---------------------------------------------------------------------------

const LANES = [
  {
    lane: 'spell-casting-ability',
    role: 'sd33-r2-spell',
    title: 'Spells — the missing casting-ability mapping',
    population: 815,
    breakdown: '598 fixture-verified spell units + 217 literal-verified spell units = 815 of the 1,390 remainder',
    resultsFile: 'spell-remainder.oracle-results.json',
    receiptFile: 'AT-33-E5-remainder-spell_cycle_receipt.md',
    diagnosis: `## THE NAMED BLOCKER (from AT-33-E5-001's receipt)
These units were not attempted because there is **no casting-ability mapping** —
the oracle side needs to know which ability score governs a given spell's
computed magnitude (save DC, caster level, bonus spells) for the class casting
it, and that mapping was not available to the wave-1 generator.

**That is a missing mapping, not a missing capability.** Build it:
- PCGen's own game-mode data states the casting ability per class. Look under
  the pinned oracle checkout's \`system/gameModes/Pathfinder\` and the class
  \`.lst\` data for the governing-ability declaration, and DERIVE the mapping by
  reading those files — do not hardcode a table from memory. State the command
  that produced it.
- Cross-check against our own corpus's class records. Where the two disagree
  about a class's casting ability, that is a DISAGREEMENT to root-cause, not a
  mapping bug to paper over.
- A spell reachable by several classes may have several governing abilities.
  Pick the mapping deliberately, and say in the receipt which class you bound
  each spell to and why. A spell whose governing ability is genuinely
  indeterminate is a real \`unverifiable\` WITH THAT REASON — not a skip.

The wave-1 spell probe (\`fixture-spell-probe-output.json\`,
\`fixture-spell.oracle-results.json\`) already works end-to-end for the spells it
could map. You are widening its input, not rebuilding it.`,
    scope: `- \`scripts/oracle_harness/\` — the casting-ability mapping and any generator widening
- \`src/bin/\` — extend the wave-1 spell probe binary; do not fork it
- \`${E5DIR}/\` — files prefixed \`spell-remainder\``,
  },
  {
    lane: 'equipment-other-bonus-shape',
    role: 'sd33-r2-equipment',
    title: 'Equipment — bonus shapes the probe does not yet handle',
    population: 494,
    breakdown: '448 literal-verified equipment units at `other_bonus_shape` + 46 equipment_modifier units = 494 of the 1,390 remainder',
    resultsFile: 'equipment-remainder.oracle-results.json',
    receiptFile: 'AT-33-E5-remainder-equipment_cycle_receipt.md',
    diagnosis: `## THE NAMED BLOCKER (from AT-33-E5-002's receipt)
These units were classified \`other_bonus_shape\` — their bonus chains are shapes
the wave-1 equipment path did not handle. \`equipment_modifier\` is a related but
distinct shape (a modifier applied TO an item rather than an item's own bonus).

**Enumerate the shapes before you write any code.** Group the 494 by their actual
bonus-chain shape, count each group, and put that table in the receipt. Then
handle them by group. This is a GENERIC PASS BY MECHANISM — do not open a lane
per item.

Read the WHOLE corpus record for each shape, not a filtered view: a grep narrowed
to BONUS/PRE hides STACK/MULT and other application-governing fields, and those
fields change the computed value.

Wave 1's equipment path (\`equipment.oracle-results.json\`,
\`equipment-literal.oracle-results.json\`, and its \`.ours.json\` siblings) is proven
for the simple shape. Extend \`compute_equipment_effects\` coverage on our side
and the export template on the oracle side to reach the rest.

A shape that genuinely computes no magnitude is \`unverifiable\` with the
\`no_bonus_chain\`-style reason wave 1 established — reuse that vocabulary rather
than inventing a parallel one.`,
    scope: `- \`scripts/oracle_harness/\` — export-template widening for the new shapes
- \`src/rules_core/\` — only if a bonus shape is genuinely unhandled by our compute path; if you change compute, the change is a real defect fix with its own RED->GREEN
- \`src/bin/\` — extend the wave-1 equipment probe binary
- \`${E5DIR}/\` — files prefixed \`equipment-remainder\``,
  },
  {
    lane: 'full-character-build',
    role: 'sd33-r2-charbuild',
    title: 'Class features, races, race traits — the full-pilot-build shape',
    population: 81,
    breakdown: '15 fixture class_feature + 17 literal class_feature + 36 race + 13 race_trait = 81 of the 1,390 remainder',
    resultsFile: 'charbuild-remainder.oracle-results.json',
    receiptFile: 'AT-33-E5-remainder-charbuild_cycle_receipt.md',
    diagnosis: `## THE NAMED BLOCKER (from AT-33-E5-001's receipt)
Our "ours" value for these comes from \`probe_class_feature_effect_wiring\`'s
consumer-delta mechanism in \`src/bin/v06_work_inventory.rs\`, which needs the FULL
pilot-compute pipeline (\`build_pilot_headless_receipt\`) with a real character
build — race, feats, full class progression — not the narrow library seam the
spell and equipment probes use. Wave 1 judged that too risky to rush. It was
right to stop; it is now your whole job, and 81 units is a small population.

**Your population is small and your per-unit setup is expensive — so amortise the
BUILD, not the unit.** Attempt 1's own next-cycle plan is the concrete route:
one L20 \`.pcg\` per source class, then read many features off that one built
character. Do the same on our side: one \`build_pilot_headless_receipt\` per
class, many features read from the one receipt.

The magnitude shapes differ per feature (DR, sneak-attack dice, channel-energy
dice+uses, trap-sense, fixed numeric bonuses — at least 6 distinct export-token
families per \`outputsheets/base.xml.ftl\`). Enumerate the shapes first, put the
table in the receipt, then handle them by family. Races and race traits attach to
the same built character — fold them into the same builds rather than standing up
a second pipeline.

A feature whose magnitude genuinely has no oracle-side token is \`unverifiable\`
with that named reason. A feature you did not get to is not.`,
    scope: `- \`src/bin/v06_work_inventory.rs\` — probe extension only (Epic 4 is finished; do NOT rewrite docs/work-inventory.json)
- \`scripts/oracle_harness/\` — the L20-per-class build generator and template
- \`${E5DIR}/\` — files prefixed \`charbuild-remainder\``,
  },
]

// ---------------------------------------------------------------------------
// Finalize cycle — owns the kanban call on rows 16/17/18.
// ---------------------------------------------------------------------------

function finalizePrompt(laneResults) {
  return `${standingRules('sd33-r2-e5-finalize')}

## YOUR JOB: total the Epic 5 population and own the kanban call on rows 16, 17, 18

Three mechanism lanes just closed the 1,390-unit remainder. Their reports:
${JSON.stringify(laneResults).slice(0, 8000)}

**Those are reports, not evidence.** Every number below you derive yourself by
COUNTING ROWS in the committed results files. The prior scan caught a lane
marking a row \`complete\` over a partial population; do not repeat it.

## WHAT YOU MUST ESTABLISH
1. **Merge all results files** into the canonical combined artifact
   \`${E5DIR}/AT-33-E5-003.combined-oracle-results.json\`, plus the two
   per-criterion files:
   - \`${E5DIR}/fixture-verified.combined-oracle-results.json\` -> 1,741 rows
   - \`${E5DIR}/literal-verified.oracle-results.json\` -> 6,589 rows
   Merge on \`unit_id\`. **Duplicate \`unit_id\`s across lanes are a real finding** —
   two lanes disagreeing about one unit's verdict must be root-caused, never
   silently de-duplicated by last-writer-wins.
2. **Count, per file:** rows, distinct unit_ids, and the verdict histogram. Every
   population must equal its denominator exactly: 1,741 / 6,589 / 8,330
   (8,330 = 1,741 + 6,589).
3. **Zero reasonless \`unverifiable\` rows.** 319 of 6,940 rows carried
   \`unverifiable\` with no reason field. Re-derive that count. If any survive,
   fix them — a reasonless \`unverifiable\` is a "we did not look" bucket, which
   AT-33-E4-003's doctrine forbids.
4. **Every disagreement resolved.** Wave 1 found 103 and fixed them at
   \`dded72f0b4\`, reaching 0 disagreements across the then-examined population.
   Re-derive the disagreement count across the FULL 8,330. Any NEW disagreement
   from this wave is root-caused: either our computation is wrong (fix it) or the
   harness is wrong (fix it AND re-run everything it already judged). A
   disagreement is NEVER closed by adjusting the expectation to match our output.
   One entry per disagreement in \`progress.md\`. A filed blocker does not satisfy
   AT-33-E5-003.
5. **Re-prove \`disagree\` capability on the batch path.** A zero-disagreement
   result across 8,330 units is a suspicious result, not a happy one. Feed a
   known-disagreeing case through the CURRENT batch path and show it returning
   \`disagree\`. A batch rewrite can silently swallow what the single-unit path
   caught.

## THEN THE KANBAN CALL
Mark rows 16 (AT-33-E5-001), 17 (AT-33-E5-002), and 18 (AT-33-E5-003)
\`complete\` **only if** each population is fully rowed with its denominator
stated. If any is short, leave the row honest and report — an honest
\`in-progress\` is a correct outcome and the two prior waves were right to write it.

Update all three AT-33-E5-00{1,2,3} receipts' figure rows to the final totals.

## PROCEDURE
§6, all nine steps, §7 receipt schema with \`- **Status:** <x>\` as a BULLET.
Receipt: \`${E5DIR}/AT-33-E5-finalize_cycle_receipt.md\`.
Before pushing, run \`scripts/verify.sh --only denominator-gate\` on your own new
prose and confirm it still exits 0.

## RETURN VALUE — ONLY JSON:
{"task":"e5-finalize","status":"complete"|"blocked-escalated",
 "fixture":{"population":1741,"rows":0,"distinct":0,"agree":0,"disagree":0,"unverifiable":0,"unexamined":0},
 "literal":{"population":6589,"rows":0,"distinct":0,"agree":0,"disagree":0,"unverifiable":0,"unexamined":0},
 "combined":{"population":8330,"rows":0,"distinct":0,"agree":0,"disagree":0,"unverifiable":0,"unexamined":0},
 "duplicate_unit_ids":[...],"reasonless_unverifiable":0,
 "new_disagreements":[{"unit_id":"...","rootcause":"...","resolution":"..."}],
 "disagree_capability_reproven_on_batch_path":true|false,
 "kanban":{"row16":"complete"|"in-progress","row17":"...","row18":"..."},
 "denominator_gate":"PASS"|"FAIL",
 "commit_shas":[...],"receipt_path":"..."}`
}

// ---------------------------------------------------------------------------
// Epic 6, attempt 3.
// ---------------------------------------------------------------------------

function finalAcceptanceScanPrompt(summary) {
  return `${standingRules('sd33-r2-acceptance-scan')}

## YOUR CRITERION: AT-33-E6-001 — final-acceptance scan, ATTEMPT 3 (kanban row 19)

Two prior scans FAILED and correctly halted the bundle. Read both receipts:
- \`${PKG}/artifacts/epic-6-closure/AT-33-E6-001_cycle_receipt.md\` (attempt 1)
- \`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt2_cycle_receipt.md\` (attempt 2)

Attempt 2 confirmed CLOSED: the denominator gate (green, detection re-proven live)
and the deferral posture (2 open of 8, both genuine capability deferrals with
named revisit conditions, 0 covering DoD scope). **Re-verify both anyway** — this
wave rewrote prose the gate scans, and emitted new retro events.

Attempt 2's surviving shortfall was one thing only: **1,390 of 8,330 Epic 5 units
unexamined**, splitting rows 16 (in-progress, 1,128 of 1,741) and 17/18
(complete-over-partial). A three-lane mechanism wave plus a finalize cycle has
since run. Reports:
${JSON.stringify(summary).slice(0, 7000)}

That is a REPORT, not evidence. Verify it yourself.

## THE CHECK THAT MATTERS MOST
COUNT THE ROWS. Do not trust a receipt's summary line, a lane's JSON, or a
kanban Notes column:
    python3 -c "import json,collections
for p,pop in [('fixture-verified.combined-oracle-results.json',1741),
              ('literal-verified.oracle-results.json',6589),
              ('AT-33-E5-003.combined-oracle-results.json',8330)]:
  d=json.load(open('${E5DIR}/'+p))
  k='results' if 'results' in d else [x for x in d if isinstance(d[x],list)][0]
  r=d[k]; print(p,'rows',len(r),'distinct',len({x.get('unit_id') for x in r}),'pop',pop,
    dict(collections.Counter(x.get('verdict') or x.get('status') for x in r)))"
Every population must equal its denominator EXACTLY. Also verify:
- 0 rows carry \`unverifiable\` with an empty/missing reason field
- 0 rows carry \`disagree\` unresolved; each historical disagreement traces to a commit
- the batch path has been shown to still return \`disagree\` on a known case
- no duplicate \`unit_id\` across the merged files

## THEN THE FULL SCAN
Every criterion AT-33-E1-001 .. AT-33-E5-003 \`complete\`; every kanban card rows
1-18 \`complete\` (19-21 are Epic 6's own). \`returned-to-backlog\`,
\`in-progress\`, \`blocked-escalated\`, or \`complete\`-with-a-deferred-half BLOCKS.
There is NO "complete OR filed under Open blockers".

Check the WORK, not the reports:
- \`git log\` and the actual target files per criterion
- every receipt exists at its kanban-stated path and matches §7, including the
  figures row (number + denominator + re-derive command) and the four-buckets row
- re-run the headline re-derive commands yourself — recaps quote stale figures
- grep the closure INSTRUMENTS for hardcoded exclusion lists; carve-outs hide in
  code, not prose
- \`python3 scripts/box_ledger.py --check\` exits 0
- \`scripts/verify.sh --only denominator-gate\` exits 0 AND still catches a bare
  percentage with no denominator (re-prove it live, then remove your probe file)
- \`jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json\` is 0
- Epic 3's corpus-wide artifact is at the SD-33 path and SD-32's
  \`docs/release/SD-32-.../artifacts/gate-2-engines/\` file is UNTOUCHED
- enumerate open deferrals; none defers DoD scope; all carry a revisit condition

IF ANYTHING IS SHORT: STOP. No retrospective, no sweep, NO PR. Report what is
short WITH THE COMMAND THAT SHOWS IT. That is a CORRECT outcome — it is what the
two prior scans did, and both were right. You are the scanner, not an executor.

Receipt: \`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt3_cycle_receipt.md\`
Commit and push (§5). Mark kanban row 19 \`complete\` only on PASS.

Return ONLY JSON:
{"criterion":"AT-33-E6-001","attempt":3,"gate":"PASS"|"FAIL",
 "status":"complete"|"blocked-escalated",
 "row_counts":{"fixture":0,"literal":0,"combined":0},
 "prior_shortfalls_closed":[{"shortfall":"...","closed":true|false,"command":"...","output":"..."}],
 "shortfalls":[{"what":"...","command":"...","output":"..."}],
 "commands_rerun":[{"command":"...","output":"..."}],
 "receipt_path":"...","commit_shas":[...]}`
}

function retrospectiveAndSweepPrompt(scanResult) {
  return `${standingRules('sd33-r2-retro-sweep')}

## YOUR CRITERION: AT-33-E6-002 — retrospective written and cited (kanban row 20)
## PLUS §11.3 — full worktree/branch sweep

The final-acceptance scan PASSED on attempt 3:
${JSON.stringify(scanResult).slice(0, 4000)}

1. RETROSPECTIVE -> \`docs/retro/sd33-computed-value-verification-retrospective.md\`,
   grounded in \`python3 scripts/retro.py summary --since 2026-08-24 --json\` — READ
   that output, do not paraphrase from memory. \`deferrals.open\` IS trustworthy
   (SD-32's fix landed: \`grep -n 'len(open_deferrals)' scripts/retro.py\` -> 772).
   Confirm it yourself and say so.

   **The spine of this retrospective is the throughput arc**, and it must be told
   with numbers and denominators:
   - Epic 2 proved the oracle method at n=1 (one hand-authored PCGen character).
   - Epic 5 carried that method into a population of 8,330 without measuring its
     per-unit cost. It reached 32 of 8,330.
   - Scan 1 halted the bundle. Remediation built generators: 6,940 of 8,330.
   - Scan 2 halted it again on the remaining 1,390 of 8,330.
   - Remediation wave 2 decomposed that remainder into three named MECHANISMS
     (spell casting-ability mapping, equipment bonus shapes, full-character
     build) and closed them.
   **Three dispatch waves, two correct halts, one bundle.** The gate worked
   exactly as designed both times, and the honest \`in-progress\` rows the lanes
   wrote are why it could. Say that plainly — the lanes never over-claimed, and
   that is the reason this closed correctly rather than closing falsely.

   Record the throughput lesson WITH ITS ENFORCING MECHANISM — e.g. a required
   dispatch-brief field: measured per-unit cost, population, projected wall time,
   filled in before any population-scoped run starts. A lesson without a
   mechanism is a quote (decisions.md §4). Also record the second-order lesson:
   a remainder named per-mechanism is closable; a remainder named as "the rest"
   is not.

   Close out workflow-instruction.md §12 rows 3 and 8, both UNENFORCED at launch:
   state whether they were closed and how, or why not.

2. CITE IT from \`${PKG}/references/README.md\` IN THIS SAME CYCLE. An uncited
   retrospective does not satisfy the criterion.

3. FULL WORKTREE/BRANCH SWEEP. \`git worktree list\`, \`git branch -a\`, \`df -h /\`.
   Report count FOUND vs count REMOVED. Three dispatch runs left worktrees behind
   (\`.claude/worktrees/wf_*\`). Check each for unmerged commits BEFORE removing.
   NEVER remove a \`locked\` worktree or one carrying unmerged commits — report
   those instead.

NO PR IN THIS CYCLE. Steps 2 and 3 land before the PR opens.

Receipts to \`${PKG}/artifacts/epic-6-closure/\`. Commit, push (§5), mark row 20
complete. Re-run \`scripts/verify.sh --only denominator-gate\` on your own prose
before pushing.

Return ONLY JSON:
{"criterion":"AT-33-E6-002","status":"complete"|"blocked-escalated",
 "retro_path":"...","cited_from":"...","retro_summary_figures":[...],
 "deferrals_field_corrected":true|false,"throughput_lesson_mechanism":"...",
 "sweep":{"worktrees_found":0,"worktrees_removed":0,"branches_found":0,"branches_removed":0,"kept_locked":[...],"kept_unmerged":[...]},
 "unenforced_rows_closed":{"row3":"...","row8":"..."},
 "denominator_gate":"PASS"|"FAIL",
 "receipt_paths":[...],"commit_shas":[...]}`
}

function architectureDocsGraphifyPrPrompt(prior) {
  return `${standingRules('sd33-r2-archdocs-pr')}

## YOUR CRITERION: AT-33-E6-003 (part 1) — architecture docs, graphify, PR (kanban row 21)

Retrospective and sweep are DONE (that order is load-bearing). Prior cycle:
${JSON.stringify(prior).slice(0, 3000)}

Follow \`${REPO}/docs/release/template/template.md §6\` — read it first, it is the
procedure of record.

1. ARCHITECTURE DOCS — \`docs/architecture/\` is CURRENT-STATE TRUTH. Refresh for
   what SD-33 actually changed: the PCGen oracle harness and its batch
   generators, the casting-ability mapping, the widened equipment bonus-shape
   coverage, the L20-per-class pilot-build probe path, \`box_ledger.py\` and THE-BOX
   partition, the denominator gate in \`verify.sh\`, formula-interpreter
   corpus-wide coverage, and the work-inventory \`unknown\` -> zero classification.
2. GRAPHIFY per template §6.
3. OPEN THE PR: ${BRANCH} -> develop. The body cites the retrospective, the
   receipts, and the headline figures WITH THEIR DENOMINATORS. State plainly that
   two final-acceptance scans failed and what each remediation wave closed —
   that history belongs in the PR, not hidden. It is the strongest evidence the
   gate works.
4. Resolve merge conflicts if any. NEVER force-push. DO NOT MERGE — the operator
   merges tranche -> develop.

Commit and push (§5). Do NOT mark row 21 complete; the release-notes cycle owns it.

Return ONLY JSON:
{"criterion":"AT-33-E6-003-part1","status":"complete"|"blocked-escalated",
 "arch_docs_touched":[...],"graphify":"...","pr_url":"...","pr_number":0,
 "conflicts_resolved":"...","commit_shas":[...]}`
}

function releaseNotesVersionBumpPrompt(prPrior) {
  return `${standingRules('sd33-r2-release-notes')}

## YOUR CRITERION: AT-33-E6-003 (part 2) — release notes + version bump (kanban row 21)

Housekeeping. The PR is already open:
${JSON.stringify(prPrior).slice(0, 2000)}

1. Fill in \`${PKG}/release-notes.md\` for build \`0.13.0\` — what shipped, every
   figure stating its denominator in the same construct. No narrative ceremony.
2. Record the PR number in \`release-notes.md\` and in \`${PKG}/receipts.md\`.
3. VERSION BUMP: confirm \`apps/desktop/package.json\` and
   \`apps/desktop/src-tauri/tauri.conf.json\` BOTH read \`0.13.0\`. They were stamped
   at the ${BRANCH} cut. DO NOT bump the tranche digit — it moves only on a NEW
   tranche/N branch cut, never on a bundle's own closure.
4. Placeholder sweep — every match resolves or is a documented deferral:
   grep -rn '<[a-z_-]*>' ${PKG}/*.md
5. Update \`${PKG}/progress.md\` to the closed state; mark kanban row 21 \`complete\`.
6. Re-run \`scripts/verify.sh --only denominator-gate\` on your own new prose and
   confirm it exits 0 before you push. Watch for bare hundred-percent tokens.

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

log('SD-33 REMEDIATION WAVE 2. Epic 5 stands at 6,940 of 8,330 examined. Closing the last 1,390 by mechanism: spell 815 + equipment 494 + charbuild 81.')

phase('Mechanism lanes — the last 1,390')
const laneResults = await parallel(
  LANES.map(c => () => agent(lanePrompt(c), {
    model: 'sonnet',
    label: c.lane,
    phase: 'Mechanism lanes — the last 1,390',
    isolation: 'worktree',
  }))
)
log('Mechanism lanes returned. Check `df -h /` and `git worktree list` (§8).')

phase('Epic 5 finalize')
const finalize = await agent(finalizePrompt(laneResults), {
  model: 'sonnet', label: 'e5-finalize', phase: 'Epic 5 finalize',
})

const summary = { lanes: laneResults, finalize }

phase('Epic 6 — Closure epilogue')
const scan = await agent(finalAcceptanceScanPrompt(summary), {
  model: 'opus', label: 'final-acceptance-scan-attempt3', phase: 'Epic 6 — Closure epilogue',
})

const scanFailed = !scan || /"gate"\s*:\s*"FAIL"/.test(String(scan)) || /blocked-escalated/.test(String(scan))
if (scanFailed) {
  log('AT-33-E6-001 attempt 3 did NOT pass. Per §11 step 1: no retrospective, no sweep, NO PR. Correct outcome, not a failure.')
  return {
    bundle: 'SD-33', wave: 2, closed: false,
    halted_at: 'AT-33-E6-001 final-acceptance scan (attempt 3)',
    scan, summary,
  }
}

const retroSweep = await agent(retrospectiveAndSweepPrompt(scan), { model: 'sonnet', label: 'retrospective-and-sweep', phase: 'Epic 6 — Closure epilogue' })
const archPr = await agent(architectureDocsGraphifyPrPrompt(retroSweep), { model: 'sonnet', label: 'archdocs-graphify-pr', phase: 'Epic 6 — Closure epilogue' })
const notes = await agent(releaseNotesVersionBumpPrompt(archPr), { model: 'haiku', label: 'release-notes-version-bump', phase: 'Epic 6 — Closure epilogue' })

log('SD-33 closure epilogue complete. The operator merges tranche/13 -> develop.')

return {
  bundle: 'SD-33', wave: 2, branch: BRANCH, closed: true,
  summary,
  epic6: { scan, retroSweep, archPr, notes },
}
