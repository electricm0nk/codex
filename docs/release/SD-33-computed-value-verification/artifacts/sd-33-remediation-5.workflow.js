export const meta = {
  name: 'sd-33-remediation-5',
  description: 'SD-33 remediation wave 5 — close the last 67 units by token family, resolve 4 armor disagreements, re-run closure',
  whenToUse: 'After sd-33-remediation-4 halted at AT-33-E6-001 attempt 5 with 4 disagreements and 67 of 8,330 unexamined.',
  phases: [
    { title: 'Token-family lanes — the last 67' },
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
  return `You are a dispatched SD-33 REMEDIATION WAVE 5 agent. Role: ${role}.

CONTEXT. SD-33 has run five dispatch waves and been halted five times by its own
final-acceptance scan. Every halt was correct. Epic 5 has moved
32 -> 6,940 -> 7,939 -> 8,255 -> 8,263 of 8,330 units examined, and its
disagreement count has moved 26 -> 4.

Wave 4 landed a REAL PRODUCT FIX at \`abc72f75ec\`: 22 of 26 armor disagreements
were genuine defects in our compute, now corrected. The scan spot-checked six by
name and confirmed the oracle values were untouched — our numbers moved to match
reality, not the reverse. That is the bundle doing its job.

Two things still block closure:
- **67 of 8,330 units** unexamined, every one named by token family below.
- **4 disagreements** that a baseline-diff method could not isolate.

Everything else is closed and must NOT be re-litigated: Epics 1/2/3/4; row 16 at
1,741 of 1,741 with 0 disagree; the denominator gate (scope widened to
\`artifacts/**/*_cycle_receipt.md\` plus 8 root package documents, detection
re-proven live twice, not blinded); the deferral posture; 0 reasonless
\`unverifiable\` of 7,501; 0 duplicate unit_ids of 8,263; work-inventory
\`unknown\` at 0; no hardcoded exclusion lists.

REQUIRED READS, in order:
1. ${REPO}/CLAUDE.md
2. ${REPO}/AGENTS.md
3. ${REPO}/${PKG}/workflow-instruction.md   (dispatch procedure — binding)
4. ${REPO}/${PKG}/epic-breakdown.md          (AT-33-E5-002 / AT-33-E5-003, verbatim)
5. ${REPO}/${E5DIR}/AT-33-E5-last75_cycle_receipt.md
   (wave 4's remainder lane. It returned \`blocked-escalated\` honestly after 8 of
   75 and left a COMPLETE per-shape table with populations and the concrete
   blocker for each. That table is your map and it is good work.)
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
construct. The denominator gate scans the package's markdown documents now — run
\`scripts/verify.sh --only denominator-gate\` before you push. Bare
hundred-percent tokens are caught specifically; it has already flagged two
agents' own receipts.

NO STUBS. NO SAMPLING PRESENTED AS POPULATION. NO DoD-SCOPE DEFERRALS.

## YOUR STATUS MUST MATCH YOUR ROW COUNT
Run the count on your own artifact before writing your return JSON, and put its
literal output in the JSON. If it is short of your population, your status is
\`blocked-escalated\`. Wave 4's remainder lane did exactly this — 8 of 75,
reported honestly, with a shape table that made THIS wave possible. That is the
model. A \`complete\` over a partial population is the only unrecoverable move.`
}

const SHARED_METHOD = `
## METHOD — what has and has not worked across five waves

Working and reusable, do NOT rebuild:
- \`scripts/oracle_harness/\` — the proven batch harness, export and compare path
- the equipment probe binaries under \`src/bin/\`
- the row format in \`${E5DIR}/*.oracle-results.json\`

**Amortise the JVM, not the unit.** One exported character carrying N computed
variables verifies N units per BatchExporter start. Your populations are small,
so setup cost dominates — measure on ~10 units, project your wall time, then run.

**Read the WHOLE corpus record.** A grep narrowed to BONUS/PRE hides STACK/MULT
and other application-governing fields, and those govern the computed value.

**Group by mechanism, fix by mechanism.** Put your shape table in the receipt
with the count per group. Do not open a sub-lane per item.

## VERDICT DISCIPLINE

\`unverifiable\` is a FIRST-CLASS verdict (AT-33-E2-003) and correct when a unit
genuinely has no comparable computed magnitude — a non-scalar effect, or a
chain with no single PCGen token to compare against. Reuse the established
\`no_bonus_chain\` / \`no_probe_surface\` vocabulary; do not invent a parallel one.

**\`unverifiable\` is not a parking space for a unit you could not reach.** Every
\`unverifiable\` row MUST carry a populated reason field — the files currently
have 0 reasonless rows of 7,501 and must stay that way.

**If a unit yields a comparable value and it disagrees, record the \`disagree\`
honestly.** Do not suppress one to keep a count tidy. A real disagreement is a
find, not a failure — wave 4 found 26 and 22 were genuine product bugs.
`

function lanePrompt(c) {
  return `${standingRules(c.role)}

## YOUR SLICE: ${c.title}
**Population: ${c.population} of the 67 remaining**, which are 67 of the 8,330
Epic 5 population (8,330 = 1,741 fixture-verified + 6,589 literal-verified).

Shapes, verbatim from wave 4's remainder receipt (population per shape):
${c.shapes}

${c.diagnosis}

${SHARED_METHOD}

## FIRST ACTION — re-derive your set
    python3 -c "import json
wi=json.load(open('docs/work-inventory.json'))['units']
pop={u['id'] for u in wi if u.get('status') in ('literal-verified','fixture-verified')}
d=json.load(open('${E5DIR}/AT-33-E5-003.combined-oracle-results.json'))['results']
miss=sorted(pop-{r['unit_id'] for r in d})
print(len(miss)); [print(m) for m in miss]"
Then select YOUR shapes from that list by reading each unit's bonus chain. If the
total across the three sibling lanes would not cover all 67, say so loudly with
the command — an unexamined unit that falls in no lane's shape list is exactly
how a remainder survives another wave.

## WHAT "DONE" MEANS FOR YOU
Every one of your ${c.population} units carries a per-unit
\`(ours, oracle, verdict)\` row in your committed results JSON, with a populated
reason on every \`unverifiable\`.

## COORDINATION
Sibling lanes are running RIGHT NOW on the other token families and on the 4
remaining disagreements. You share \`${E5DIR}/\`, \`progress.md\`, and \`kanban.md\`.
Write rows ONLY to your own results file (named below) — never to a sibling's
file or to the merged \`AT-33-E5-003.combined-oracle-results.json\`. Re-read before
every shared edit, rebase before every push, never revert a sibling's rows.
Do NOT mark kanban rows 16/17/18 — the finalize cycle owns that call.

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
5. Receipt, EXACTLY this schema — \`- **Status:** <x>\` is a BULLET:

${RECEIPT_SCHEMA}

6. Commit, push via §5.
7. Update progress.md via §5 (re-read first). Kanban Notes are a POINTER only.
8. Count your rows mechanically; set status from that count.
9. Report.

## RETURN VALUE — ONLY JSON, no prose:
{"lane":"${c.lane}","status":"complete"|"blocked-escalated",
 "population":${c.population},"rows_written":0,
 "agree":0,"disagree":0,"unverifiable":0,"unexamined":0,"reasonless_unverifiable":0,
 "row_count_command_output":"<literal output of your own len() count>",
 "denominator_statement":"<X of ${c.population} ${c.lane} units>",
 "shape_table":[{"shape":"...","population":0,"examined":0,"verdicts":{}}],
 "unclaimed_units_seen":[...],
 "per_unit_cost_seconds":0,"units_per_character":0,
 "disagreements":[{"unit_id":"...","ours":"...","oracle":"..."}],
 "commit_shas":[...],"receipt_path":"...","red_green":"...",
 "identifier_audit":"...","wired_audit":"...","next":"..."}`
}

// ---------------------------------------------------------------------------
// Three token-family lanes covering 37 + 23 + 7 = 67.
// ---------------------------------------------------------------------------

const LANES = [
  {
    lane: 'weapon-token-family',
    role: 'sd33-r5-weapon',
    title: 'WEAPON / WEAPONPROF token family',
    population: 37,
    shapes: `- \`WEAPONPROF=<x>\` / \`WEAPON\` enhancement family — 24 (wave 4 notes \`compute_equipmods_effect\` ALREADY covers this on our side)
- bare \`WEAPON|TOHIT,DAMAGE,ATTACKS\` with no \`TYPE=\` qualifier — 6 (3 flurry-extra-attacks + 3 plain offset)
- \`WEAPON|DAMAGEMULT\` fractional crit-multiplier — 4
- wield-size \`WIELDCATEGORY\` + bare \`WEAPON|TOHIT\` (the no-penalty variants) — 3`,
    resultsFile: 'last67-weapon.oracle-results.json',
    receiptFile: 'AT-33-E5-last67-weapon_cycle_receipt.md',
    diagnosis: `## THE BLOCKER, AND WHY THIS IS THE MOST TRACTABLE LANE
Wave 4 explicitly recorded that our side ALREADY handles the 24-unit
\`WEAPONPROF=<x>\`/\`WEAPON\` enhancement family via \`compute_equipmods_effect\`.
So for those 24 the missing half is the ORACLE side: an export template emitting
the weapon attack/damage token for a character wielding the item. Start there —
it is 24 of your 37 and it needs no compute change.

The other three shapes each need a decision you must state explicitly:
- **bare \`WEAPON|TOHIT,DAMAGE,ATTACKS\` (6).** No \`TYPE=\` qualifier means the
  bonus is unscoped. Decide what the single comparable magnitude is, say so, and
  be consistent across all 6. The 3 flurry-extra-attacks units bonus the NUMBER
  of attacks rather than a to-hit value — if the comparable quantity for those is
  an attack count, compare attack counts and say so.
- **\`WEAPON|DAMAGEMULT\` (4).** A fractional crit multiplier. Our row format
  carries integers; decide how a fractional value is represented and compared,
  state it, and do not silently truncate — a truncation that turns 1.5 into 1
  would manufacture a false \`agree\`.
- **wield-size no-penalty variants (3).** Wave 4 ruled the plain
  \`WIELDCATEGORY\`-only units \`unverifiable\` (non-scalar — they change a size
  category, not a number). These three ALSO carry a bare \`WEAPON|TOHIT\`, so the
  TOHIT half may well be comparable even though the wield-size half is not.
  Compare the comparable half and say exactly what you compared.

These need a character wielding the item for the oracle to compute anything.
Amortise: one character, many weapons, many tokens per export.`,
    scope: `- \`scripts/oracle_harness/\` — weapon export-template widening and the build generator
- \`src/rules_core/\` — only if a shape is genuinely unhandled by our compute path; a compute change is a real defect fix with its own RED->GREEN
- \`src/bin/\` — extend the existing equipment probe binaries; do not fork them`,
  },
  {
    lane: 'skill-combat-token-family',
    role: 'sd33-r5-skillcombat',
    title: 'SKILL and COMBAT token families, psionics included',
    population: 23,
    shapes: `- \`SKILL\` single-skill, \`ultimate_psionics\` book — 14
- \`COMBAT\`-shape non-psionics (INITIATIVE / TOHIT.Ranged / formula-valued AC, SAVE) — 6
- \`COMBAT\`-shape, \`ultimate_psionics\` book — 1
- \`ultimate_psionics\` dissonance \`VAR\`+\`WEAPON\`-formula pair — 2`,
    resultsFile: 'last67-skill-combat.oracle-results.json',
    receiptFile: 'AT-33-E5-last67-skill-combat_cycle_receipt.md',
    diagnosis: `## THE BLOCKER — AND A MYTH ALREADY KILLED
**Ultimate Psionics data DOES exist in the pinned oracle checkout.** Wave 4
verified it at \`data/pathfinder/dreamscarred_press/ultimate_psionics/\`
(including \`up_equipment.lst\`), and 25 \`ultimate_psionics\` units already carry
real live-PCGen oracle values in the combined file. So "psionics is not in the
oracle" is NOT available to you as a reason. If a psionic unit ends
\`unverifiable\`, the reason must be about that unit's bonus shape, not about book
availability.

Your 17 psionic units are mostly plain single-skill \`SKILL\` bonuses — the same
shape wave 4 successfully compared for a non-psionics unit (1 of 1 \`agree\`). The
route is proven; you are widening its input to the psionics book. Start there.

Two shapes need explicit decisions:
- **\`COMBAT\` non-psionics (6)** — INITIATIVE, TOHIT.Ranged, and formula-valued
  AC/SAVE. The subtoken determines which oracle export token carries the
  comparable value, so the mapping is per subtoken. A FORMULA-valued bonus needs
  the formula evaluated against a concrete character before it is a number —
  build the character, evaluate, compare, and say which character you used.
- **dissonance \`VAR\`+\`WEAPON\`-formula pair (2)** — two coupled halves. Decide
  what the comparable magnitude is, state it, and apply it to both consistently.

Wave 4 also found two hazards worth avoiding: a \`Magic.Wondrous.Implant\` slot
with no matching PCGen \`EQSLOT\` (correctly \`unverifiable\`, slot cannot be
equipped), and multi-skill comma-joined chains with no single PCGen token
(correctly \`unverifiable\`). If you meet those shapes again, reuse those exact
reasons rather than inventing new ones.`,
    scope: `- \`scripts/oracle_harness/\` — SKILL/COMBAT export-template widening and the build generator
- \`src/rules_core/\` — only for a genuinely unhandled shape; RED->GREEN required
- \`src/bin/\` — extend the existing probe binaries`,
  },
  {
    lane: 'eqm-modifier-family',
    role: 'sd33-r5-eqm',
    title: 'EQM* equipment-modifier token family',
    population: 7,
    shapes: `- \`EQMARMOR\` material family (\`draco\`, \`dragonhide\`, \`material_dragonhide\`) — 3
- \`EQMWEAPON|DAMAGESIZE\` — 2
- \`EQMWEAPON|RANGEADD\` — 1
- \`EQM|WEIGHTDIV\` — 1`,
    resultsFile: 'last67-eqm.oracle-results.json',
    receiptFile: 'AT-33-E5-last67-eqm_cycle_receipt.md',
    diagnosis: `## THE BLOCKER
These are \`equipment_modifier\` records, not items: they modify a host item
rather than carrying a bonus of their own. PCGen computes nothing for a modifier
in isolation — it must be APPLIED to a host item on a character before any number
exists. That is the whole difficulty and it is the same on both sides.

Route: pick a concrete host item per modifier, apply the modifier, and compare
the host's computed value with and without it — the DELTA is the modifier's
magnitude. State which host you chose for each modifier and why; a different host
can legitimately give a different delta, so the choice must be recorded, and it
must be the same host on our side and the oracle's.

Per shape:
- **\`EQMARMOR\` materials (3)** — apply to a concrete armor host. Note the wave-4
  finding that armor compute was genuinely buggy and was fixed at \`abc72f75ec\`;
  build on the corrected behaviour, and if you find a further armor defect, that
  is a real \`disagree\` to record honestly.
- **\`EQMWEAPON|DAMAGESIZE\` (2)** — changes a damage DIE SIZE, not a scalar
  bonus. If the comparable quantity is a die step or a die string rather than a
  number, say so explicitly. If there is genuinely no comparable scalar, that is
  a real \`unverifiable\` WITH THAT REASON — a legitimate finding for this shape.
- **\`EQMWEAPON|RANGEADD\` (1)** — a range increment addition; compare ranges.
- **\`EQM|WEIGHTDIV\` (1)** — divides weight. Weight is a real computed magnitude;
  compare weights, and note that a division may produce a fractional value —
  do not truncate silently.

Only 7 units, but 4 distinct mechanisms. Enumerate, decide, record.`,
    scope: `- \`scripts/oracle_harness/\` — modifier-application export path and host selection
- \`src/rules_core/\` — only for a genuinely unhandled shape; RED->GREEN required
- \`src/bin/\` — extend the existing equipment probe binaries`,
  },
]

// ---------------------------------------------------------------------------
// Lane 4 — the 4 remaining disagreements.
// ---------------------------------------------------------------------------

function disagreementPrompt() {
  return `${standingRules('sd33-r5-disagreements')}

## YOUR CRITERION: AT-33-E5-003 — the last 4 disagreements

Wave 4 fixed 22 of 26 disagreements with a genuine compute correction at
\`abc72f75ec\`, verified by spot-check with the oracle values untouched. **4 of
8,263 examined units still disagree**, and wave 4 recorded the reason as
\`baseline_diff_harness_limitation\`:

| unit_id | ours | oracle |
|---|---:|---:|
| advanced_class_guide:equipment:full_plate_of_the_corpse | 9 | 10 |
| inner_sea_world_guide:equipment:field_plate | 7 | 6 |
| inner_sea_world_guide:equipment:stoneplate | 9 | 8 |
| ultimate_equipment:equipment:snakeskin_tunic | 1 | 2 |

## THE DIAGNOSIS TO TEST
All four are armor. Two (\`field_plate\`, \`stoneplate\`) have ours HIGH by 1; two
have ours LOW by 1. Wave 4's stated blocker is that its BASELINE-DIFF method —
computing a character's value with and without the item and taking the delta —
cannot isolate these. A plausible cause is that the item's contribution
interacts with the baseline character's own armor state, so the delta is not the
item's magnitude; another is a masterwork or base-armor-bonus term counted on one
side only. **Establish which, by execution, before changing anything.**

**The method itself may be the defect.** If the baseline-diff approach is
structurally unable to isolate an armor value, the fix is an ABSOLUTE method —
export the character's total with a controlled, known baseline (an unarmoured
character, or one whose armor state you set explicitly) rather than differencing
two unknowns. Consider that route seriously; four waves have shown that a method
carried past its limit is this bundle's recurring failure shape.

## THE BAR (verbatim from epic-breakdown.md)
> A disagreement is **never** closed by adjusting the expectation to match our
> output. Each is root-caused: either our computation is wrong (fix it) or the
> oracle comparison is wrong (fix the harness, and re-run everything it already
> judged).
> **Evidence:** one entry per disagreement in \`progress.md\`, each resolved to a
> commit or an operator escalation. **A filed blocker does not satisfy this
> criterion.**

**Note the second limb and budget for it.** If you fix the HARNESS, you MUST
re-run everything it already judged — all 8,263 rows, not just these 4. Decide
your route deliberately, say which you took and why, and if it is the harness
route, do the re-run inside this turn.

Illegitimate closures, each of which the scan checks for by name:
(a) moving the expected value to match our output;
(b) reclassifying a disagreeing unit to \`unverifiable\` or dropping it;
(c) fixing the harness without re-running its prior verdicts.

\`snakeskin_tunic\` is currently marked \`unverifiable\`-adjacent by wave 4 as
"escalated, not hidden". An escalation is for a genuine operator ruling, not a
way out of the work — with 4 units and a named method limitation, this is a
SEQUENCING problem, not an exemption. Clear it.

## FINISH LINE
\`python3 scripts/box_ledger.py --check --oracle-results ${E5DIR}/AT-33-E5-003.combined-oracle-results.json\`
exits 0 with \`oracle_disagreement=0\`, over a population where all four were
genuinely resolved. Show before and after.

**Re-prove the batch path can still return \`disagree\`.** After fixing the last
disagreements, a broken comparison and a clean result look identical. Feed a
known-disagreeing case through the CURRENT path, show \`disagree\`, remove the probe.

**If your method change alters values for units already judged, re-run them.**
Say how many rows you re-ran and what moved.

## COORDINATION
Three sibling lanes are running RIGHT NOW on the 67 unexamined units, appending
rows to their own files. Write your corrected rows to
\`${E5DIR}/disagreement-fixes-wave5.oracle-results.json\`; the finalize cycle
merges. If you re-run the whole population after a harness fix, write that full
set to \`${E5DIR}/full-rerun-wave5.oracle-results.json\` and say so — do NOT
overwrite the merged file directly, and do not clobber a sibling's rows.
Do NOT mark kanban rows 16/17/18 — finalize owns that call.

## PROCEDURE
§6, all nine steps, §7 schema with \`- **Status:** <x>\` as a BULLET.
Receipt: \`${E5DIR}/AT-33-E5-003-disagreement-fixes-wave5_cycle_receipt.md\`
One \`progress.md\` entry per disagreement, each resolved to a commit.

## WRITE SCOPE
- \`src/rules_core/\` — the compute fix, if that is the route; RED->GREEN required
- \`scripts/oracle_harness/\` — the method/harness fix, if that is the route
- \`src/bin/\` — the equipment probe binaries
- your two results files and your receipt

## RETURN VALUE — ONLY JSON:
{"lane":"disagreements-wave5","status":"complete"|"blocked-escalated",
 "disagreements_in":4,"disagreements_out":0,
 "rootcause":"...","method_was_the_defect":true|false,
 "route_taken":"our-compute"|"harness"|"both",
 "rerun_everything_after_harness_fix":true|false,"rows_rerun":0,"rows_that_moved":0,
 "per_unit":[{"unit_id":"...","was_ours":0,"was_oracle":0,"now_ours":0,"now_oracle":0,"rootcause":"...","commit":"..."}],
 "box_ledger_before":"...","box_ledger_after":"...",
 "disagree_capability_reproven_on_batch_path":true|false,
 "progress_entries_written":0,
 "commit_shas":[...],"receipt_path":"...","red_green":"...",
 "identifier_audit":"...","wired_audit":"...","next":"..."}`
}

// ---------------------------------------------------------------------------
// Finalize.
// ---------------------------------------------------------------------------

function finalizePrompt(laneResults) {
  return `${standingRules('sd33-r5-e5-finalize')}

## YOUR JOB: total Epic 5 and own the kanban call on rows 16, 17, 18

Wave 5's four lanes just ran — three closing the 67 unexamined units by token
family, one resolving the last 4 disagreements. Reports:
${JSON.stringify(laneResults).slice(0, 9000)}

**Reports are not evidence.** Derive every number yourself by counting rows and
deriving sets.

## WHAT YOU MUST ESTABLISH
1. **Merge every lane results file** into the canonical artifacts:
   - \`${E5DIR}/fixture-verified.combined-oracle-results.json\` -> exactly 1,741 rows
   - \`${E5DIR}/literal-verified.oracle-results.json\` -> exactly 6,589 rows
   - \`${E5DIR}/AT-33-E5-003.combined-oracle-results.json\` -> exactly 8,330 rows
   Merge on \`unit_id\`. The disagreement lane's corrected rows SUPERSEDE the stale
   rows for those unit_ids — the one legitimate overwrite; state which ids you
   superseded and why. If that lane produced a FULL RE-RUN file
   (\`full-rerun-wave5.oracle-results.json\`), it supersedes every row it contains
   and you must say how many rows moved. Any OTHER duplicate \`unit_id\` is a real
   finding to root-cause, never last-writer-wins.
2. **Derive the unexamined SET, not its size:**
       python3 -c "import json
   wi=json.load(open('docs/work-inventory.json'))['units']
   pop={u['id'] for u in wi if u.get('status') in ('literal-verified','fixture-verified')}
   d=json.load(open('${E5DIR}/AT-33-E5-003.combined-oracle-results.json'))['results']
   miss=sorted(pop-{r['unit_id'] for r in d}); print(len(miss)); [print(m) for m in miss]"
   It must be EMPTY. If not, print the missing ids and their shapes; your status
   is \`blocked-escalated\`.
3. **Zero unresolved \`disagree\`:**
   \`python3 scripts/box_ledger.py --check --oracle-results ${E5DIR}/AT-33-E5-003.combined-oracle-results.json\`
   exits 0 with \`oracle_disagreement=0\`. A NEW disagreement from a token-family
   lane is root-caused here the same way — never suppressed, never closed by
   moving the expected value.
4. **Zero reasonless \`unverifiable\`** across all three files. Re-derive.
5. **Re-prove \`disagree\` capability on the current batch path.** Feed a
   known-disagreeing case through, show \`disagree\`, remove the probe.
6. **Keep the gate green.** Run \`scripts/verify.sh --only denominator-gate\`
   before pushing; fix your own prose if it trips. Do not narrow scope to pass.

## THEN THE KANBAN CALL
Mark rows 16, 17, 18 \`complete\` **only if** each population is fully rowed with
its denominator stated and zero disagreements remain. If any is short, leave the
row honest and report. Update the AT-33-E5-00{1,2,3} receipts' figure rows.

## PROCEDURE
§6, all nine steps, §7 schema with \`- **Status:** <x>\` as a BULLET.
Receipt: \`${E5DIR}/AT-33-E5-finalize-wave5_cycle_receipt.md\`

## RETURN VALUE — ONLY JSON:
{"task":"e5-finalize-wave5","status":"complete"|"blocked-escalated",
 "fixture":{"population":1741,"rows":0,"distinct":0,"agree":0,"disagree":0,"unverifiable":0},
 "literal":{"population":6589,"rows":0,"distinct":0,"agree":0,"disagree":0,"unverifiable":0},
 "combined":{"population":8330,"rows":0,"distinct":0,"agree":0,"disagree":0,"unverifiable":0},
 "missing_unit_ids":[...],"superseded_unit_ids":[...],"rows_moved_by_rerun":0,
 "unexpected_duplicates":[...],"reasonless_unverifiable":0,
 "box_ledger_oracle_disagreement":0,
 "disagree_capability_reproven_on_batch_path":true|false,
 "kanban":{"row16":"complete"|"in-progress","row17":"...","row18":"..."},
 "denominator_gate":"PASS"|"FAIL",
 "commit_shas":[...],"receipt_path":"..."}`
}

// ---------------------------------------------------------------------------
// Epic 6, attempt 6.
// ---------------------------------------------------------------------------

function finalAcceptanceScanPrompt(summary) {
  return `${standingRules('sd33-r5-acceptance-scan')}

## YOUR CRITERION: AT-33-E6-001 — final-acceptance scan, ATTEMPT 6 (kanban row 19)

Five prior scans FAILED and correctly halted the bundle. Read attempt 5's
receipt: \`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt5_cycle_receipt.md\`

Attempt 5 confirmed CLOSED — re-verify, do not re-investigate: row 16 at 1,741 of
1,741 with 0 disagree; the denominator gate (scope = \`artifacts/**/*_cycle_receipt.md\`
plus 8 root package documents, widened not blinded, detection re-proven twice
including against the scanner's own draft); the deferral posture; 0 reasonless
\`unverifiable\` of 7,501; 0 duplicate unit_ids; work-inventory \`unknown\` at 0;
no hardcoded exclusion lists; Epic 3's artifact at the SD-33 path with SD-32's
untouched.

Attempt 5's two surviving shortfalls: **4 disagreements** and **67 of 8,330
unexamined**. Four lanes and a finalize cycle have since run. Reports:
${JSON.stringify(summary).slice(0, 7000)}

That is a REPORT, not evidence.

## CHECK 1 — COUNT THE ROWS AND DERIVE THE SET
    python3 -c "import json,collections
wi=json.load(open('docs/work-inventory.json'))['units']
pop={u['id'] for u in wi if u.get('status') in ('literal-verified','fixture-verified')}
for p,n in [('fixture-verified.combined-oracle-results.json',1741),
            ('literal-verified.oracle-results.json',6589),
            ('AT-33-E5-003.combined-oracle-results.json',8330)]:
  d=json.load(open('${E5DIR}/'+p))
  k='results' if 'results' in d else [x for x in d if isinstance(d[x],list)][0]
  r=d[k]; ids=[x.get('unit_id') for x in r]
  print(p,'rows',len(r),'distinct',len(set(ids)),'pop',n,dict(collections.Counter(x.get('verdict') for x in r)))
c=json.load(open('${E5DIR}/AT-33-E5-003.combined-oracle-results.json'))['results']
miss=sorted(pop-{x['unit_id'] for x in c}); print('MISSING',len(miss)); [print(' ',m) for m in miss[:80]]"
Every population must equal its denominator EXACTLY, and MISSING must be 0.

## CHECK 2 — THE DISAGREEMENTS WERE RESOLVED, NOT HIDDEN
    python3 scripts/box_ledger.py --check --oracle-results ${E5DIR}/AT-33-E5-003.combined-oracle-results.json; echo $?
must print \`oracle_disagreement=0\` and exit 0. Then verify HOW the zero was
reached. Spot-check the four by name — \`full_plate_of_the_corpse\` (was 9 vs 10),
\`field_plate\` (7 vs 6), \`stoneplate\` (9 vs 8), \`snakeskin_tunic\` (1 vs 2) — and
for each confirm the row is still PRESENT, now \`agree\`, and traced to a commit
in \`git log\` whose diff you READ.

Blocking shortfalls if you find them:
(a) an expected/oracle value edited to match our output;
(b) a disagreeing unit reclassified to \`unverifiable\` or dropped;
(c) the harness or method changed WITHOUT re-running its prior verdicts —
    this wave's disagreement lane was told the method itself may be the defect,
    so if it changed the method, verify the re-run actually happened and check
    how many rows moved. A method change that silently leaves 8,000 stale rows
    is the worst available outcome.

Also verify one \`progress.md\` entry per disagreement resolved to a commit.
**Re-prove \`disagree\` capability on the current batch path yourself** — feed a
known-disagreeing case through, show \`disagree\`, remove the probe.

## CHECK 3 — THE NEWLY-COVERED SHAPES ARE REAL COMPARISONS
Wave 5's lanes made judgment calls this scan must audit, because each is a place
a false \`agree\` could be manufactured:
- \`WEAPON|DAMAGEMULT\` fractional values — confirm no silent truncation turned a
  1.5 into a 1;
- bare \`WEAPON|TOHIT,DAMAGE,ATTACKS\` — confirm the chosen comparable magnitude
  is stated and applied consistently across all 6;
- \`EQM*\` modifiers — confirm the host item is named per modifier and is the SAME
  host on our side and the oracle's;
- psionics — confirm no unit was called \`unverifiable\` for "book not in oracle",
  which wave 4 disproved.
Sample a few rows of each and confirm the receipt states the decision.

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
- Epic 3's artifact at the SD-33 path; SD-32's gate-2-engines file UNTOUCHED
- enumerate open deferrals; none defers DoD scope; all carry a revisit condition
- the Rust suite is green for whatever \`src/rules_core/\` changed this wave

IF ANYTHING IS SHORT: STOP. No retrospective, no sweep, NO PR. Report what is
short WITH THE COMMAND THAT SHOWS IT. Five prior scans did exactly that and all
five were right.

Receipt: \`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt6_cycle_receipt.md\`
Commit and push (§5). Mark kanban row 19 \`complete\` only on PASS.

Return ONLY JSON:
{"criterion":"AT-33-E6-001","attempt":6,"gate":"PASS"|"FAIL",
 "status":"complete"|"blocked-escalated",
 "row_counts":{"fixture":0,"literal":0,"combined":0},
 "unexamined_set_empty":true|false,"missing_unit_ids":[...],
 "oracle_disagreement":0,"disagreements_resolved_not_hidden":true|false,
 "method_change_rerun_verified":true|false,
 "spot_checks":[{"unit_id":"...","was":"...","now":"...","commit":"...","verdict":"real-fix"|"hidden"}],
 "new_shape_audits":[{"shape":"...","finding":"..."}],
 "disagree_capability_reproven":true|false,
 "gate_scope_widened_not_blinded":true|false,
 "prior_shortfalls_closed":[{"shortfall":"...","closed":true|false,"command":"...","output":"..."}],
 "shortfalls":[{"what":"...","command":"...","output":"..."}],
 "commands_rerun":[{"command":"...","output":"..."}],
 "receipt_path":"...","commit_shas":[...]}`
}

function retrospectiveAndSweepPrompt(scanResult) {
  return `${standingRules('sd33-r5-retro-sweep')}

## YOUR CRITERION: AT-33-E6-002 — retrospective written and cited (kanban row 20)
## PLUS §11.3 — full worktree/branch sweep

The final-acceptance scan PASSED on attempt 6:
${JSON.stringify(scanResult).slice(0, 4000)}

1. RETROSPECTIVE -> \`docs/retro/sd33-computed-value-verification-retrospective.md\`,
   grounded in \`python3 scripts/retro.py summary --since 2026-08-24 --json\` — READ
   that output. \`deferrals.open\` IS trustworthy (SD-32's fix landed:
   \`grep -n 'len(open_deferrals)' scripts/retro.py\`). Confirm it and say so.

   **THE HEADLINE IS THE DEFECTS.** SD-33 existed to answer one question: is a
   computed value that looks right actually right? It found real wrong answers —
   the armor disagreements, 22 of them fixed at \`abc72f75ec\` alone — and fixed
   them. Lead with the defects and the mechanism behind each group, not process.

   **The second story is the throughput arc**, with denominators:
   - Epic 2 proved the oracle at n=1 — one hand-authored PCGen character.
   - Epic 5 carried that method to 8,330 without measuring per-unit cost first,
     and reached 32 of 8,330.
   - Scan 1 halted. Generators -> 6,940 of 8,330.
   - Scan 2 halted. The 1,390 remainder split into three MECHANISMS -> 7,939.
   - Scan 3 halted. The 391 remainder split by bonus-shape family -> 8,255.
   - Scan 4 halted on 26 disagreements + 75 unexamined -> 8,263, 26 -> 4.
   - Scan 5 halted on 4 disagreements + 67 unexamined, split by TOKEN family
     -> 8,330 of 8,330, 0 disagreements.
   **Six dispatch waves, five correct halts, one bundle.**

   Lessons, each with its ENFORCING MECHANISM (a lesson without a mechanism is a
   quote — decisions.md §4):
   a. **Measure per-unit cost before a population-scoped run.** Mechanism: a
      required dispatch-brief field — measured cost, population, projected wall
      time — filled before the run starts.
   b. **A remainder named per-MECHANISM is closable; "the rest" is not.** Every
      wave shrank because the prior wave named its remainder specifically, and
      wave 4's honest 8-of-75 report with a full shape table is what made wave 5
      a clean token-family split. Mechanism: a required per-shape enumeration of
      anything unexamined, in the receipt.
   c. **A lane's status must be a mechanical function of its row count.** Wave 2's
      equipment lane returned \`complete\` over 103 of 494; only row-counting
      caught it. Mechanism: the scan counts rows and derives the unexamined SET.
   d. **Coverage growth surfaces defects late.** The disagreements appeared only
      in newly-covered shapes — a bundle that stopped at 95% would have shipped
      them. Mechanism: the scan's set-difference check.
   e. **A method carried past its limit is this bundle's recurring failure
      shape** — one character per unit at n=8,330; a baseline-diff that could not
      isolate an armor value. Mechanism: when a method stalls, change the method,
      and re-run everything it already judged.
   Record also that the gate's own scan SCOPE was a defect (\`DEFAULT_GLOBS\` could
   not see the bundle's headline documents), found by the scanner and closed by
   an instrument lane.

   Say plainly: five scans failed and each failure was the system working. The
   lanes wrote honest short rows rather than false greens — with one exception,
   wave 2's equipment lane, which row-counting caught. That is why this closed
   truthfully.

   Close out workflow-instruction.md §12 rows 3 and 8, both UNENFORCED at launch.

2. CITE IT from \`${PKG}/references/README.md\` IN THIS SAME CYCLE.

3. FULL WORKTREE/BRANCH SWEEP. \`git worktree list\`, \`git branch -a\`, \`df -h /\`.
   Report count FOUND vs REMOVED. Six dispatch runs left worktrees behind
   (\`.claude/worktrees/wf_*\`). Check each for unmerged commits BEFORE removing.
   NEVER remove a \`locked\` worktree or one carrying unmerged commits — report those.

NO PR IN THIS CYCLE. Steps 2 and 3 land before the PR opens.

Receipts to \`${PKG}/artifacts/epic-6-closure/\`. Commit, push (§5), mark row 20
complete. Run \`scripts/verify.sh --only denominator-gate\` on your own prose
before pushing.

Return ONLY JSON:
{"criterion":"AT-33-E6-002","status":"complete"|"blocked-escalated",
 "retro_path":"...","cited_from":"...","retro_summary_figures":[...],
 "defects_found_and_fixed":0,"deferrals_field_corrected":true|false,
 "lessons_with_mechanisms":[...],
 "sweep":{"worktrees_found":0,"worktrees_removed":0,"branches_found":0,"branches_removed":0,"kept_locked":[...],"kept_unmerged":[...]},
 "unenforced_rows_closed":{"row3":"...","row8":"..."},
 "denominator_gate":"PASS"|"FAIL",
 "receipt_paths":[...],"commit_shas":[...]}`
}

function architectureDocsGraphifyPrPrompt(prior) {
  return `${standingRules('sd33-r5-archdocs-pr')}

## YOUR CRITERION: AT-33-E6-003 (part 1) — architecture docs, graphify, PR (kanban row 21)

Retrospective and sweep are DONE (that order is load-bearing). Prior cycle:
${JSON.stringify(prior).slice(0, 3000)}

Follow \`${REPO}/docs/release/template/template.md §6\` — read it first.

1. ARCHITECTURE DOCS — \`docs/architecture/\` is CURRENT-STATE TRUTH. Refresh for
   what SD-33 actually changed: the PCGen oracle harness and its batch
   generators, the spell casting-ability mapping, the widened equipment
   bonus-shape coverage (VAR / COMBAT / WEAPON / WEAPONPROF / STAT / SAVE /
   SITUATION / SKILL / natural-attack / special-material / EQM* modifiers),
   **the armor-bonus compute fixes behind the disagreements**, the
   L20-per-class pilot-build probe path, \`box_ledger.py\` and THE-BOX partition,
   the denominator gate in \`verify.sh\` and its widened scan scope,
   formula-interpreter corpus-wide coverage, and the work-inventory
   \`unknown\` -> zero classification.
2. GRAPHIFY per template §6.
3. OPEN THE PR: ${BRANCH} -> develop. The body leads with the DEFECTS FOUND AND
   FIXED, cites the retrospective and receipts, and states headline figures WITH
   THEIR DENOMINATORS. State plainly that five final-acceptance scans failed and
   what each wave closed — that history is the strongest evidence the gate works.
4. Resolve merge conflicts if any. NEVER force-push. DO NOT MERGE — the operator
   merges tranche -> develop.

Commit and push (§5). Do NOT mark row 21 complete; release-notes owns it.

Return ONLY JSON:
{"criterion":"AT-33-E6-003-part1","status":"complete"|"blocked-escalated",
 "arch_docs_touched":[...],"graphify":"...","pr_url":"...","pr_number":0,
 "conflicts_resolved":"...","commit_shas":[...]}`
}

function releaseNotesVersionBumpPrompt(prPrior) {
  return `${standingRules('sd33-r5-release-notes')}

## YOUR CRITERION: AT-33-E6-003 (part 2) — release notes + version bump (kanban row 21)

Housekeeping. The PR is open:
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
   Watch for bare hundred-percent tokens — the gate has already caught two agents.

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

log('SD-33 REMEDIATION WAVE 5. Epic 5 at 8,263 of 8,330 with 4 disagreements. Four lanes: weapon family 37, skill/combat family 23, EQM family 7, plus the last 4 disagreements.')

phase('Token-family lanes — the last 67')
const [weapon, skillCombat, eqm, disagreements] = await parallel([
  () => agent(lanePrompt(LANES[0]), { model: 'sonnet', label: LANES[0].lane, phase: 'Token-family lanes — the last 67', isolation: 'worktree' }),
  () => agent(lanePrompt(LANES[1]), { model: 'sonnet', label: LANES[1].lane, phase: 'Token-family lanes — the last 67', isolation: 'worktree' }),
  () => agent(lanePrompt(LANES[2]), { model: 'sonnet', label: LANES[2].lane, phase: 'Token-family lanes — the last 67', isolation: 'worktree' }),
  () => agent(disagreementPrompt(), { model: 'sonnet', label: 'disagreements-last-4', phase: 'Token-family lanes — the last 67', isolation: 'worktree' }),
])
log('Wave-5 lanes returned. Check `df -h /` and `git worktree list` (§8).')

phase('Epic 5 finalize')
const finalize = await agent(finalizePrompt({ weapon, skillCombat, eqm, disagreements }), {
  model: 'sonnet', label: 'e5-finalize-wave5', phase: 'Epic 5 finalize',
})

const summary = { weapon, skillCombat, eqm, disagreements, finalize }

phase('Epic 6 — Closure epilogue')
const scan = await agent(finalAcceptanceScanPrompt(summary), {
  model: 'opus', label: 'final-acceptance-scan-attempt6', phase: 'Epic 6 — Closure epilogue',
})

const scanFailed = !scan || /"gate"\s*:\s*"FAIL"/.test(String(scan)) || /blocked-escalated/.test(String(scan))
if (scanFailed) {
  log('AT-33-E6-001 attempt 6 did NOT pass. Per §11 step 1: no retrospective, no sweep, NO PR. Correct outcome, not a failure.')
  return {
    bundle: 'SD-33', wave: 5, closed: false,
    halted_at: 'AT-33-E6-001 final-acceptance scan (attempt 6)',
    scan, summary,
  }
}

const retroSweep = await agent(retrospectiveAndSweepPrompt(scan), { model: 'sonnet', label: 'retrospective-and-sweep', phase: 'Epic 6 — Closure epilogue' })
const archPr = await agent(architectureDocsGraphifyPrPrompt(retroSweep), { model: 'sonnet', label: 'archdocs-graphify-pr', phase: 'Epic 6 — Closure epilogue' })
const notes = await agent(releaseNotesVersionBumpPrompt(archPr), { model: 'haiku', label: 'release-notes-version-bump', phase: 'Epic 6 — Closure epilogue' })

log('SD-33 closure epilogue complete. The operator merges tranche/13 -> develop.')

return {
  bundle: 'SD-33', wave: 5, branch: BRANCH, closed: true,
  summary,
  epic6: { scan, retroSweep, archPr, notes },
}
