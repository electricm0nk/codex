export const meta = {
  name: 'sd-33-remediation-6',
  description: 'SD-33 remediation wave 6 — clear the corpus-extraction blocker, verify the AC method re-run, close the last 39',
  whenToUse: 'After sd-33-remediation-5 halted at AT-33-E6-001 attempt 6 with 39 unexamined, 1 escalated disagreement, and an unverified method re-run.',
  phases: [
    { title: 'Blocker, method, and the last 39' },
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
  return `You are a dispatched SD-33 REMEDIATION WAVE 6 agent. Role: ${role}.

CONTEXT. SD-33 has run six dispatch waves and been halted six times by its own
final-acceptance scan. Every halt was correct. Epic 5 has moved
32 -> 6,940 -> 7,939 -> 8,255 -> 8,263 -> 8,291 of 8,330 units examined, and its
disagreement count 26 -> 4 -> 1.

Wave 5 landed real work: a genuine METHOD replacement at \`a68fbeea3d\` (a
per-type AC isolator replacing a whole-character \`AC.TOTAL\` diff that conflated
a MAXDEX-cap Dex loss), a real engine fix at \`2f1d52f22d\`
(\`compute_equipmods_effect\` multi-chain summing), a harness fix
(campaign-KEY-vs-display-name), and an engine fix for OUTPUTNAME-divergent
identity in \`equipment_id_resolve\`. The scan spot-checked six units and
confirmed every resolved one was a REAL fix with no edited expectations.

Three things still block closure:
- **39 of 8,330 units** unexamined (23 weapon-shape + 9 skill-combat-shape + 7 eqm-shape).
- **1 disagreement**, \`rending_claw_blades\`, currently filed under
  \`## Open blockers\` — which PAUSES the bundle and is not a closure path.
- **The wave-5 method change was NOT verified to have been re-run** across the
  rows it already judged (\`method_change_rerun_verified: false\`).

Everything else is closed and must NOT be re-litigated: Epics 1/2/3/4; row 16 at
1,741 of 1,741 with 0 disagree; the denominator gate (scope widened, detection
re-proven, not blinded); 0 reasonless \`unverifiable\`; 0 duplicate unit_ids;
work-inventory \`unknown\` at 0; no hardcoded exclusion lists.

REQUIRED READS, in order:
1. ${REPO}/CLAUDE.md
2. ${REPO}/AGENTS.md
3. ${REPO}/${PKG}/workflow-instruction.md   (dispatch procedure — binding)
4. ${REPO}/${PKG}/epic-breakdown.md          (AT-33-E5-002 / AT-33-E5-003, verbatim)
5. ${REPO}/${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt6_cycle_receipt.md
   (the scan that halted the bundle — your shortfall is named there)
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
- NEVER pass --allow-stamp-loss.

RETRO EVENTS (§2.3): emit via \`scripts/retro.py\` as they happen. \`--verified-by\`
required on a \`correction\`.

FIGURES (decisions.md §2): every number states its denominator in the SAME
construct. Run \`scripts/verify.sh --only denominator-gate\` before you push —
its scope covers the package's markdown documents and it has already caught
three agents' own receipts. Bare hundred-percent tokens are caught specifically.

NO STUBS. NO SAMPLING PRESENTED AS POPULATION. NO DoD-SCOPE DEFERRALS.

## YOUR STATUS MUST MATCH YOUR ROW COUNT
Run the count on your own artifact before writing your return JSON, and put its
literal output in the JSON. If it is short of your population, your status is
\`blocked-escalated\`. Waves 4 and 5 produced honest short reports with complete
shape tables, and those reports are exactly why each later wave could split the
work cleanly. That is the model.`
}

const SHARED_METHOD = `
## METHOD — what works, after six waves

Reuse, do NOT rebuild:
- \`scripts/oracle_harness/\` — the proven batch harness, export and compare path,
  now carrying wave 5's per-type AC isolator and campaign-KEY fixes
- the equipment probe binaries under \`src/bin/\`
- the row format in \`${E5DIR}/*.oracle-results.json\`

**Amortise the JVM, not the unit.** One exported character carrying N computed
variables verifies N units per BatchExporter start. Measure on ~10 units, project
your wall time, then run.

**Read the WHOLE corpus record.** A grep narrowed to BONUS/PRE hides STACK/MULT
and other application-governing fields, and those govern the computed value.

**A method carried past its limit is this bundle's recurring failure shape** —
one character per unit at n=8,330; a whole-character AC diff that could not
isolate an armor value. If your method stalls, CHANGE THE METHOD, and re-run
everything it already judged.

## VERDICT DISCIPLINE

\`unverifiable\` is a FIRST-CLASS verdict and correct when a unit genuinely has no
comparable computed magnitude. Reuse the established vocabulary
(\`no_bonus_chain\`, \`no_probe_surface\`, \`no_comparable_export_token\`,
\`no_resolver\`); do not invent a parallel one.

**\`unverifiable\` is not a parking space for a unit you could not reach.** Every
\`unverifiable\` row MUST carry a populated reason field — the files currently
have 0 reasonless rows and must stay that way.

**Record a real \`disagree\` honestly.** Do not suppress one to keep a count tidy.
Waves 4 and 5 surfaced real product bugs this way and all but one are fixed.
`

// ---------------------------------------------------------------------------
// Lane 1 — the Open blocker. Highest priority: it pauses the bundle.
// ---------------------------------------------------------------------------

function corpusExtractionPrompt() {
  return `${standingRules('sd33-r6-corpus-extraction')}

## YOUR TASK — clear the \`## Open blockers\` entry by FIXING it

An entry under \`## Open blockers\` in \`${PKG}/progress.md\` **pauses the bundle**.
Per \`${REPO}/docs/governance/blocker-closure-doctrine.md\` there are exactly two
dispositions: **clear it** (decompose and run the cycles — a large blocker is a
SEQUENCING problem, not an exemption) or raise a hand and stop. It is never a
closure path.

**Your job is to clear it.**

### THE BLOCKER, as filed by \`sd33-r5-e5-finalize\`
\`advanced_race_guide:equipment:rending_claw_blades\` is a genuine examined
\`disagree\` (\`ours=0\`, \`oracle=1\`, DAMAGE dimension). Root cause, already
established and verified live:

> The pinned PCGen source defines this record via a \`.MOD\`-attached line in
> \`arg_equip_arms_armor.lst\` that references two additional \`Special Ability\`
> EQMODs (\`+1 ~ Weapon\`, \`Keen ~ Weapon\`). This repo's corpus extraction
> pipeline never captured those \`.MOD\`-attached EQMOD references into
> \`data/corpus/advanced_race_guide/equipment/rending_claw_blades.json\`'s
> \`raw_tokens\`. The record's own JSON genuinely has no DAMAGE-affecting token
> to read.

That diagnosis is correct and is NOT what you re-investigate. The filer was also
right that no \`src/rules_core/\` resolver change can fix it —
\`compute_equipmods_effect\` would have to invent a value absent from its input,
which its no-fabrication discipline rightly forbids.

**The filer's conclusion — "escalate, because the real fix is in the corpus
extraction pipeline" — is the part that does not hold.** A fix that lives in the
ingest pipeline is still a fix. It is more work, not different authority.

### WHAT TO DO
1. **Find the extraction gap.** Locate the corpus extraction/generator code that
   parses \`.lst\` sources into \`data/corpus/**\`, and find where \`.MOD\`-attached
   lines are handled. Establish by execution why the attached EQMOD references
   are dropped.
2. **ENUMERATE THE BLAST RADIUS FIRST, before fixing anything.** \`rending_claw_blades\`
   is one symptom; the gap is systemic. Count how many corpus records across ALL
   books are defined or amended via a \`.MOD\`-attached line carrying EQMOD or
   BONUS references that the pipeline dropped. Report that number with its
   command and denominator. **This number is a bundle-level finding either way**
   — if it is large, say so loudly; if it is genuinely one record, prove that.
3. **Fix the extraction**, with RED->GREEN. The RED case is a test asserting the
   dropped tokens are captured for a known \`.MOD\`-attached record.
4. **Regenerate the affected corpus records through the GUARDED GENERATOR PATH
   ONLY.** \`data/corpus/**\` is NEVER hand-edited. Three hazards, all real and
   all previously observed in this repo:
   - regenerating can **destroy license/PI metadata and \`raw_tokens\`** — verify
     they survive, per record, and never pass \`--allow-stamp-loss\`;
   - a **record-count change compiles clean but leaves other files' hardcoded
     assertions red** — if any count moves, grep the old AND new numbers across
     \`tests/\`, \`src/\`, and \`apps/\` before committing;
   - a shallow glob lies here — use recursive search when you enumerate.
5. **Re-run the affected units through the oracle harness** and confirm
   \`rending_claw_blades\` now agrees, or produces a different, honestly-recorded
   verdict. If newly-captured tokens change values for OTHER units already
   judged, re-run those too and say how many rows moved.
6. **Remove the \`## Open blockers\` entry** from \`progress.md\` and replace it with
   the resolution, citing your commit. Leave the section's standing preamble
   ("This section is not a parking lot...") intact — you are removing the entry,
   not the section.

### IF YOU GENUINELY CANNOT
If the extraction fix turns out to be structurally impossible rather than merely
large, say exactly why with the code that shows it, leave the blocker filed, and
return \`blocked-escalated\`. But "this is a bigger job than one cycle" is NOT
that — it is a sequencing statement, and six waves of this bundle have shown
that decomposing beats deferring.

### COORDINATION
Four sibling lanes are running RIGHT NOW. You are the only lane touching the
corpus extraction pipeline and \`data/corpus/**\`. Write your rows to
\`${E5DIR}/corpus-extraction-fix.oracle-results.json\`. Do NOT mark kanban rows
16/17/18 — finalize owns that call.

### PROCEDURE
§6, all nine steps, §7 schema with \`- **Status:** <x>\` as a BULLET.
Receipt: \`${E5DIR}/AT-33-E5-003-corpus-extraction-fix_cycle_receipt.md\`

### RETURN VALUE — ONLY JSON:
{"lane":"corpus-extraction","status":"complete"|"blocked-escalated",
 "gap_located_at":"<file:line>","why_dropped":"...",
 "blast_radius":{"records_affected":0,"books_affected":0,"command":"...","denominator":"..."},
 "extraction_fixed":true|false,"red_green":"...",
 "records_regenerated":0,"license_pi_preserved":true|false,"raw_tokens_preserved":true|false,
 "record_counts_changed":true|false,"count_sweep_result":"...",
 "rending_claw_blades":{"was":"ours=0 oracle=1","now":"...","verdict":"..."},
 "other_units_rerun":0,"rows_that_moved":0,
 "open_blocker_removed":true|false,
 "commit_shas":[...],"receipt_path":"...","next":"..."}`
}

// ---------------------------------------------------------------------------
// Lane 2 — the unverified method re-run. Correctness risk across many rows.
// ---------------------------------------------------------------------------

function methodRerunPrompt() {
  return `${standingRules('sd33-r6-method-rerun')}

## YOUR TASK — the wave-5 METHOD change was never re-run across what it already judged

The attempt-6 scan returned \`method_change_rerun_verified: false\`. This is the
highest-severity open item in the bundle, because it is invisible: every affected
row still SAYS \`agree\`, and some of those agreements may be stale.

### WHAT CHANGED
Wave 5 replaced the AC comparison METHOD at \`a68fbeea3d\`: a per-type isolator
replaced a whole-character \`AC.TOTAL\` diff that conflated a MAXDEX-cap Dex loss
and, for \`snakeskin_tunic\`, absorbed the record's own co-located
\`STAT|DEX|2|TYPE=Enhancement\` chain through the Dex-to-AC path. Oracle values
legitimately MOVED for the four units it was applied to
(\`full_plate_of_the_corpse\` 10->11, \`field_plate\` 6->7, \`stoneplate\` 8->9,
\`snakeskin_tunic\` 2->1) — the number was not edited, the measurement was
corrected.

Two other corrections landed the same wave and have the same property: a harness
fix for campaign-KEY-vs-display-name, and an engine fix for OUTPUTNAME-divergent
identity in \`equipment_id_resolve\`.

### THE RULE (verbatim from epic-breakdown.md AT-33-E5-003)
> either our computation is wrong (fix it) or the oracle comparison is wrong
> (fix the harness, **and re-run everything it already judged**).

The second limb was not satisfied. Satisfy it.

### WHAT TO DO
1. **Determine the true blast radius of each of the three corrections.** For the
   AC isolator: every row whose oracle value came from the old whole-character
   \`AC.TOTAL\` diff — likely every AC-dimension unit, not only armor. For the
   campaign-KEY fix and the identity fix: every row whose oracle lookup or unit
   identity went through the corrected path. Derive each set BY EXECUTION with a
   command, state its size and denominator, and do NOT assume the four
   hand-checked units are the whole set.
2. **Re-run every affected row through the CURRENT harness.** Write the full
   re-run to \`${E5DIR}/method-rerun-wave6.oracle-results.json\`.
3. **Report what moved.** How many rows changed value, and how many changed
   VERDICT — specifically how many former \`agree\` rows are now \`disagree\`.
   **A newly-surfaced disagreement here is the single most valuable finding
   available in this bundle**, because it is a wrong answer that was hiding
   behind a correct-looking one. Record every one honestly with its
   \`(ours, oracle)\` pair. Do not fix them yourself unless the fix is small and
   clearly in our compute with RED->GREEN — otherwise report them cleanly so the
   finalize cycle and the scan can see them.
4. **Prove the re-run actually covered the set.** Show the count of rows re-run
   against the count of rows in the affected set, both with their denominators.
   A re-run that silently covered a subset is the same defect one level up.
5. If re-running everything affected is genuinely too large for one turn,
   re-run the largest coherent portion, report the exact remainder with its
   command, and return \`blocked-escalated\`. Do not claim completion over a subset.

### COORDINATION
Four sibling lanes are running RIGHT NOW; one is fixing the corpus extraction
pipeline, three are closing the last 39 units. Their new rows are NOT your
concern — you re-run rows that already exist. Write only to your own file. Do NOT
overwrite merged files or a sibling's file. Do NOT mark kanban rows.

### PROCEDURE
§6, all nine steps, §7 schema with \`- **Status:** <x>\` as a BULLET.
Receipt: \`${E5DIR}/AT-33-E5-003-method-rerun_cycle_receipt.md\`
One \`progress.md\` entry per newly-surfaced disagreement.

### RETURN VALUE — ONLY JSON:
{"lane":"method-rerun","status":"complete"|"blocked-escalated",
 "corrections":[{"correction":"ac-isolator"|"campaign-key"|"identity-resolve","commit":"...","affected_rows":0,"derive_command":"...","denominator":"..."}],
 "rows_rerun":0,"rows_in_affected_set":0,"coverage_statement":"<X of Y affected rows>",
 "rows_value_changed":0,"rows_verdict_changed":0,
 "new_disagreements":[{"unit_id":"...","ours":"...","oracle":"...","was_verdict":"agree"}],
 "rerun_file":"${E5DIR}/method-rerun-wave6.oracle-results.json",
 "commit_shas":[...],"receipt_path":"...","next":"..."}`
}

// ---------------------------------------------------------------------------
// Lanes 3-5 — the last 39.
// ---------------------------------------------------------------------------

function lanePrompt(c) {
  return `${standingRules(c.role)}

## YOUR SLICE: ${c.title}
**Population: ${c.population} of the 39 remaining**, which are 39 of the 8,330
Epic 5 population (8,330 = 1,741 fixture-verified + 6,589 literal-verified).

${c.diagnosis}

${SHARED_METHOD}

## FIRST ACTION — re-derive your set
    python3 -c "import json
wi=json.load(open('docs/work-inventory.json'))['units']
pop={u['id'] for u in wi if u.get('status') in ('literal-verified','fixture-verified')}
d=json.load(open('${E5DIR}/AT-33-E5-003.combined-oracle-results.json'))['results']
miss=sorted(pop-{r['unit_id'] for r in d})
print(len(miss)); [print(m) for m in miss]"
Select YOUR shapes from that list by reading each unit's bonus chain. If the
three lanes' shapes would not cover all 39, say so loudly with the command — a
unit that falls in no lane's list is how a remainder survives another wave.

## WHAT "DONE" MEANS FOR YOU
Every one of your ${c.population} units carries a per-unit
\`(ours, oracle, verdict)\` row in your committed results JSON, with a populated
reason on every \`unverifiable\`.

## COORDINATION
Four sibling lanes are running RIGHT NOW — two on the corpus-extraction blocker
and the method re-run, two on the other token families. Write rows ONLY to your
own results file. Never write a sibling's file or the merged
\`AT-33-E5-003.combined-oracle-results.json\`. Re-read before every shared edit,
rebase before every push. Do NOT mark kanban rows 16/17/18 — finalize owns that.

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
 "decisions_recorded":[{"shape":"...","comparable_magnitude":"...","why":"..."}],
 "unclaimed_units_seen":[...],
 "disagreements":[{"unit_id":"...","ours":"...","oracle":"..."}],
 "commit_shas":[...],"receipt_path":"...","red_green":"...",
 "identifier_audit":"...","wired_audit":"...","next":"..."}`
}

const LANES = [
  {
    lane: 'weapon-shape-final',
    role: 'sd33-r6-weapon',
    title: 'Weapon-shape remainder',
    population: 23,
    resultsFile: 'last39-weapon.oracle-results.json',
    receiptFile: 'AT-33-E5-last39-weapon_cycle_receipt.md',
    diagnosis: `## WHAT WAVE 5 ESTABLISHED, AND WHAT IT LEFT
Wave 5's weapon lane made real progress and recorded two decisions you INHERIT
and must apply consistently — do not re-decide them:
- **bare \`WEAPON|TOHIT,DAMAGE,ATTACKS\` with no \`TYPE=\` qualifier: TOHIT is the
  single comparable magnitude** (its receipt, line 260). It proved this on 2 of 6
  units; the other 4 are yours. Apply the same rule.
- **\`WEAPON|DAMAGEMULT\` fractional values are recorded \`unverifiable\` with reason
  \`no_comparable_export_token\`, NOT truncated to an integer** — a truncation
  would manufacture a false \`agree\`. It proved this on \`sword_cane\` (1 of 4);
  the other 3 are yours. Apply the same rule, and confirm in your receipt that no
  truncation occurred.

Your 23 units include the amulet-of-mighty-fists family (natural-attack
bonuses), the flurry units (extra-attack counts rather than to-hit values), rods
and staves, the wield-size no-penalty variants (whose \`WIELDCATEGORY\` half is
non-scalar but whose bare \`WEAPON|TOHIT\` half may well be comparable — compare
the comparable half and say exactly what you compared), \`horseshoes_of_crushing_blows_1..5\`,
and assorted cursed/odd items.

Group by shape, count each group, put the table in the receipt, handle by group.
These need a character wielding or wearing the item for the oracle to compute
anything — amortise: one character, many items, many tokens per export.

For the flurry units, if the comparable quantity is an ATTACK COUNT rather than a
to-hit value, compare attack counts and say so explicitly.`,
    scope: `- \`scripts/oracle_harness/\` — weapon export-template widening and the build generator
- \`src/rules_core/\` — only if a shape is genuinely unhandled; RED->GREEN required
- \`src/bin/\` — extend the existing equipment probe binaries; do not fork them`,
  },
  {
    lane: 'skill-combat-final',
    role: 'sd33-r6-skillcombat',
    title: 'Skill- and combat-shape remainder',
    population: 9,
    resultsFile: 'last39-skill-combat.oracle-results.json',
    receiptFile: 'AT-33-E5-last39-skill-combat_cycle_receipt.md',
    diagnosis: `## WHAT WAVE 5 ESTABLISHED
Wave 5's skill/combat lane succeeded on the bulk of its slice: all 14
\`ultimate_psionics\` SKILL-shape units came back \`agree\` with real matched
magnitudes on both sides. It also landed two real fixes you build on — a harness
fix for campaign-KEY-vs-display-name, and an engine fix in
\`equipment_id_resolve\` for OUTPUTNAME-divergent identity (RED->GREEN).

**"Ultimate Psionics is not in the oracle" is NOT available to you as a reason.**
Wave 4 disproved it (\`data/pathfinder/dreamscarred_press/ultimate_psionics/\`
exists, including \`up_equipment.lst\`) and wave 5 produced 14 real psionic
comparisons. If a psionic unit ends \`unverifiable\`, the reason must be about
that unit's bonus shape.

## YOUR 9
They include \`companion_stone_far_sight\`, the two \`flurry_of_*\` psionic units,
the two \`dissonance\` \`VAR\`+\`WEAPON\`-formula modifier pairs, and the remaining
non-psionics \`COMBAT\`-shape units (INITIATIVE / TOHIT.Ranged / formula-valued
AC and SAVE).

Two shapes need an explicitly recorded decision:
- **\`COMBAT\` subtokens.** The subtoken determines which oracle export token
  carries the comparable value, so the mapping is per subtoken, not per family.
  A FORMULA-valued bonus is not a number until evaluated against a concrete
  character — build the character, evaluate, compare, and name the character.
- **The dissonance \`VAR\`+\`WEAPON\`-formula pairs.** Two coupled halves. Decide
  what the comparable magnitude is, state it, apply it to both consistently.

Reuse wave 4's established \`unverifiable\` reasons where the shape recurs: a
\`Magic.Wondrous.Implant\` slot with no matching PCGen \`EQSLOT\` (cannot be
equipped), and multi-skill comma-joined chains with no single PCGen token.`,
    scope: `- \`scripts/oracle_harness/\` — SKILL/COMBAT export-template widening and the build generator
- \`src/rules_core/\` — only for a genuinely unhandled shape; RED->GREEN required
- \`src/bin/\` — extend the existing probe binaries`,
  },
  {
    lane: 'eqm-modifier-final',
    role: 'sd33-r6-eqm',
    title: 'EQM* equipment-modifier remainder — needs a NEW mechanism',
    population: 7,
    resultsFile: 'last39-eqm.oracle-results.json',
    receiptFile: 'AT-33-E5-last39-eqm_cycle_receipt.md',
    diagnosis: `## WAVE 5'S EQM LANE RETURNED 0 OF 7, AND ITS FINDING IS YOUR STARTING POINT
It reported honestly that the live-oracle **equipmod-attachment mechanism
"never produced a trustworthy oracle value"**, confirmed across two independent
shapes, hosts, and export tokens. Read its receipt
(\`${E5DIR}/AT-33-E5-last67-eqm_cycle_receipt.md\`) before planning.

**Do not retry the same mechanism.** A method carried past its limit is this
bundle's recurring failure shape, and wave 5 already established this one's
limit by execution. Your first task is to find a DIFFERENT way to get a
trustworthy oracle value for an equipment modifier.

Routes worth evaluating, in rough order of promise:
1. **Pre-attached host items.** Instead of attaching the modifier at export time,
   author a \`.pcg\` whose item already carries the EQMOD in its item string —
   the same form PCGen writes when a user applies a modifier in the UI. Look at
   how the pinned oracle's own sample characters encode modified items, and copy
   that encoding rather than inventing one.
2. **A \`.MOD\`/EQMOD-bearing custom item definition** fed to the oracle as data,
   so the modifier is part of the item's definition and its computed value falls
   out of a normal item export.
3. **Differential against a host that is otherwise inert** — a host chosen so
   that no other term can contaminate the delta. Wave 5's delta approach failed;
   whether it failed because of the host or because of the attachment mechanism
   is a question you can answer by execution.

Your 7: \`draco\`, \`dragonhide\`, \`material_dragonhide\` (\`EQMARMOR\` materials),
\`material_darkleaf_cloth_clothing\`, \`spike_sb\` and
\`special_quality_spikes_shieldbash\`, plus the remaining \`EQM*\` shapes
(\`DAMAGESIZE\`, \`RANGEADD\`, \`WEIGHTDIV\` per wave 4's table).

Whatever route you take: **name the host item per modifier, and use the SAME
host on our side and the oracle's.** A different host legitimately gives a
different delta, so the choice must be recorded.

Shape-specific notes:
- \`EQMARMOR\` materials — build on wave 5's corrected per-type AC isolator, not
  on the old whole-character diff.
- \`EQMWEAPON|DAMAGESIZE\` — changes a damage DIE SIZE, not a scalar. If the
  comparable quantity is a die step or die string, say so. If there is genuinely
  no comparable scalar, that is a real \`unverifiable\` WITH THAT REASON.
- \`EQM|WEIGHTDIV\` — weight IS a real computed magnitude; compare weights, and do
  not silently truncate a fractional result.

**If, after trying a genuinely different mechanism, some of these have no
comparable oracle value at all, that is a legitimate \`unverifiable\` finding —
but it must be a statement about the SHAPE, proven by execution, not about the
attempt.** Seven units and a named mechanism problem is a sequencing problem,
not an exemption.`,
    scope: `- \`scripts/oracle_harness/\` — the new modifier-application mechanism and host selection
- \`src/rules_core/\` — only for a genuinely unhandled shape; RED->GREEN required
- \`src/bin/\` — extend the existing equipment probe binaries`,
  },
]

// ---------------------------------------------------------------------------
// Finalize.
// ---------------------------------------------------------------------------

function finalizePrompt(laneResults) {
  return `${standingRules('sd33-r6-e5-finalize')}

## YOUR JOB: total Epic 5 and own the kanban call on rows 16, 17, 18

Wave 6's five lanes just ran — one clearing the corpus-extraction blocker, one
re-running the wave-5 method change, three closing the last 39 units. Reports:
${JSON.stringify(laneResults).slice(0, 9000)}

**Reports are not evidence.** Derive every number yourself.

## WHAT YOU MUST ESTABLISH
1. **Merge every lane results file** into the canonical artifacts:
   - \`${E5DIR}/fixture-verified.combined-oracle-results.json\` -> exactly 1,741 rows
   - \`${E5DIR}/literal-verified.oracle-results.json\` -> exactly 6,589 rows
   - \`${E5DIR}/AT-33-E5-003.combined-oracle-results.json\` -> exactly 8,330 rows
   Merge on \`unit_id\`. **Precedence matters this wave** — the method re-run file
   (\`method-rerun-wave6.oracle-results.json\`) and the corpus-extraction file
   SUPERSEDE stale rows for the unit_ids they contain, because both corrected a
   MEASUREMENT rather than editing a number. State which ids you superseded, from
   which file, and how many rows moved. Any OTHER duplicate \`unit_id\` is a real
   finding to root-cause, never last-writer-wins.
2. **Derive the unexamined SET, not its size:**
       python3 -c "import json
   wi=json.load(open('docs/work-inventory.json'))['units']
   pop={u['id'] for u in wi if u.get('status') in ('literal-verified','fixture-verified')}
   d=json.load(open('${E5DIR}/AT-33-E5-003.combined-oracle-results.json'))['results']
   miss=sorted(pop-{r['unit_id'] for r in d}); print(len(miss)); [print(m) for m in miss]"
   It must be EMPTY.
3. **Zero unresolved \`disagree\`:**
   \`python3 scripts/box_ledger.py --check --oracle-results ${E5DIR}/AT-33-E5-003.combined-oracle-results.json\`
   exits 0 with \`oracle_disagreement=0\`. **The method re-run may have surfaced
   NEW disagreements that were hiding behind stale agreements — those are the
   most valuable findings in the bundle and they are REAL WORK, not noise.**
   Root-cause each: our compute wrong (fix it) or the comparison wrong (fix it
   AND re-run what it judged). Never suppressed, never closed by moving an
   expected value. One \`progress.md\` entry each.
4. **The \`## Open blockers\` section in \`progress.md\` must be EMPTY of entries.**
   If the corpus-extraction lane cleared it, verify the entry is gone and the
   resolution is recorded with its commit. If that lane returned
   \`blocked-escalated\`, the bundle stays paused and your status is
   \`blocked-escalated\` — say so plainly.
5. **Zero reasonless \`unverifiable\`** across all three files. Re-derive.
6. **Re-prove \`disagree\` capability on the current batch path.** Feed a
   known-disagreeing case through, show \`disagree\`, remove the probe.
7. **Keep the gate green.** Run \`scripts/verify.sh --only denominator-gate\`
   before pushing; fix your own prose if it trips. Do not narrow scope to pass.
8. **If the corpus extraction changed record counts**, confirm the count sweep
   was done — a record-count change compiles clean but leaves other files'
   hardcoded assertions red.

## THEN THE KANBAN CALL
Mark rows 16, 17, 18 \`complete\` **only if** each population is fully rowed with
its denominator stated, zero disagreements remain, and \`## Open blockers\` holds
no entry. If any is short, leave the row honest and report.

## PROCEDURE
§6, all nine steps, §7 schema with \`- **Status:** <x>\` as a BULLET.
Receipt: \`${E5DIR}/AT-33-E5-finalize-wave6_cycle_receipt.md\`

## RETURN VALUE — ONLY JSON:
{"task":"e5-finalize-wave6","status":"complete"|"blocked-escalated",
 "fixture":{"population":1741,"rows":0,"distinct":0,"agree":0,"disagree":0,"unverifiable":0},
 "literal":{"population":6589,"rows":0,"distinct":0,"agree":0,"disagree":0,"unverifiable":0},
 "combined":{"population":8330,"rows":0,"distinct":0,"agree":0,"disagree":0,"unverifiable":0},
 "missing_unit_ids":[...],"superseded":[{"from_file":"...","unit_ids":0,"rows_moved":0}],
 "unexpected_duplicates":[...],"reasonless_unverifiable":0,
 "box_ledger_oracle_disagreement":0,
 "new_disagreements_from_rerun":[{"unit_id":"...","rootcause":"...","resolution":"..."}],
 "open_blockers_empty":true|false,
 "disagree_capability_reproven_on_batch_path":true|false,
 "record_count_sweep_confirmed":true|false,
 "kanban":{"row16":"complete"|"in-progress","row17":"...","row18":"..."},
 "denominator_gate":"PASS"|"FAIL",
 "commit_shas":[...],"receipt_path":"..."}`
}

// ---------------------------------------------------------------------------
// Epic 6, attempt 7.
// ---------------------------------------------------------------------------

function finalAcceptanceScanPrompt(summary) {
  return `${standingRules('sd33-r6-acceptance-scan')}

## YOUR CRITERION: AT-33-E6-001 — final-acceptance scan, ATTEMPT 7 (kanban row 19)

Six prior scans FAILED and correctly halted the bundle. Read attempt 6's receipt:
\`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt6_cycle_receipt.md\`

Attempt 6 confirmed CLOSED — re-verify, do not re-investigate: row 16 at 1,741 of
1,741 with 0 disagree; the denominator gate (widened not blinded, detection
re-proven); the deferral posture; 0 reasonless \`unverifiable\`; 0 duplicate
unit_ids; work-inventory \`unknown\` at 0; no hardcoded exclusion lists; Epic 3's
artifact at the SD-33 path with SD-32's untouched.

Attempt 6's three surviving shortfalls: **39 of 8,330 unexamined**, **1
disagreement filed under \`## Open blockers\`**, and **\`method_change_rerun_verified:
false\`**. Five lanes and a finalize cycle have since run. Reports:
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

## CHECK 2 — THE \`## Open blockers\` SECTION HOLDS NO ENTRY
    sed -n '/## Open blockers/,$p' ${PKG}/progress.md
An entry there PAUSES the bundle and is never a closure path. If the
corpus-extraction lane cleared it, verify by execution that the fix is real:
- the extraction code actually changed (\`git log -p\`, read the diff)
- \`rending_claw_blades\` now carries a non-\`disagree\` verdict traced to that fix
- the regenerated corpus records kept their license/PI metadata and \`raw_tokens\`
- if record counts moved, the count sweep across \`tests/\`, \`src/\`, \`apps/\` was done
- the BLAST RADIUS was enumerated, not assumed to be one record — if the lane
  reported the gap affects more records than it fixed, that remainder is itself
  a shortfall
**An entry still present, or removed without a real fix behind it, is BLOCKING.**

## CHECK 3 — THE METHOD RE-RUN ACTUALLY HAPPENED AND COVERED ITS SET
This is the check attempt 6 could not satisfy. Wave 5 replaced the AC comparison
method at \`a68fbeea3d\` plus two other corrections (campaign-KEY,
\`equipment_id_resolve\` identity). Verify:
- the affected set was DERIVED by execution, not assumed — read the command
- the re-run covered that whole set: rows re-run vs rows in the affected set,
  both with denominators
- what MOVED is reported: rows whose value changed, rows whose VERDICT changed
- **every former \`agree\` that became \`disagree\` was root-caused and resolved.**
  These are wrong answers that were hiding behind correct-looking ones; a re-run
  that surfaced none at all across a large affected set deserves suspicion, not
  relief — spot-check a few rows against a fresh harness call yourself.
A method change whose re-run silently covered a subset is the same defect one
level up, and is BLOCKING.

## CHECK 4 — DISAGREEMENTS RESOLVED, NOT HIDDEN
    python3 scripts/box_ledger.py --check --oracle-results ${E5DIR}/AT-33-E5-003.combined-oracle-results.json; echo $?
must print \`oracle_disagreement=0\` and exit 0. Blocking if you find:
(a) an oracle/expected value edited to match our output;
(b) a disagreeing unit reclassified to \`unverifiable\` or dropped;
(c) a harness/method change without its re-run.
Note the legitimate case explicitly: an oracle value that MOVED because the
MEASUREMENT METHOD was corrected is a real fix, not an edit — wave 5's four
armor units are the precedent. Distinguish by reading the commit diff.
**Re-prove \`disagree\` capability on the current batch path yourself.**

## CHECK 5 — THE NEWLY-COVERED SHAPES ARE REAL COMPARISONS
Audit wave 6's judgment calls; each is a place a false \`agree\` could be made:
- \`WEAPON|DAMAGEMULT\` — no silent truncation of a fractional value;
- bare \`WEAPON|TOHIT,DAMAGE,ATTACKS\` — the TOHIT-is-comparable rule inherited
  from wave 5 applied consistently across all 6;
- flurry units — if attack COUNT was the comparable quantity, that is stated;
- \`EQM*\` modifiers — a host item named per modifier, the SAME host on both
  sides, and the mechanism genuinely different from the one wave 5 proved
  untrustworthy;
- psionics — no unit called \`unverifiable\` for "book not in oracle";
- \`EQM|WEIGHTDIV\` — fractional weights not truncated.
Sample rows of each and confirm the receipt states the decision.

## THEN THE FULL SCAN
Every criterion AT-33-E1-001 .. AT-33-E5-003 \`complete\`; every kanban card rows
1-18 \`complete\` (19-21 are Epic 6's own). \`returned-to-backlog\`,
\`in-progress\`, \`blocked-escalated\`, or \`complete\`-with-a-deferred-half BLOCKS.
There is NO "complete OR filed under Open blockers".

Check the WORK, not the reports:
- \`git log\` and the actual target files per criterion
- every receipt at its kanban-stated path, matching §7 including the figures row
  and the four-buckets row
- re-run the headline re-derive commands yourself
- grep the closure INSTRUMENTS for hardcoded exclusion lists
- \`scripts/verify.sh --only denominator-gate\` exits 0, scope still wide, matcher
  not relaxed — re-prove detection live, then remove your probe
- \`jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json\` is 0
- Epic 3's artifact at the SD-33 path; SD-32's gate-2-engines file UNTOUCHED
- enumerate open deferrals; none defers DoD scope; all carry a revisit condition
- the Rust suite is green for whatever \`src/rules_core/\` changed this wave
- if \`data/corpus/**\` changed, license/PI and \`raw_tokens\` survived

IF ANYTHING IS SHORT: STOP. No retrospective, no sweep, NO PR. Report what is
short WITH THE COMMAND THAT SHOWS IT. Six prior scans did exactly that and all
six were right.

Receipt: \`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt7_cycle_receipt.md\`
Commit and push (§5). Mark kanban row 19 \`complete\` only on PASS.

Return ONLY JSON:
{"criterion":"AT-33-E6-001","attempt":7,"gate":"PASS"|"FAIL",
 "status":"complete"|"blocked-escalated",
 "row_counts":{"fixture":0,"literal":0,"combined":0},
 "unexamined_set_empty":true|false,"missing_unit_ids":[...],
 "open_blockers_empty":true|false,"blocker_fix_verified":true|false,
 "method_change_rerun_verified":true|false,"rerun_coverage":"...",
 "oracle_disagreement":0,"disagreements_resolved_not_hidden":true|false,
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
  return `${standingRules('sd33-r6-retro-sweep')}

## YOUR CRITERION: AT-33-E6-002 — retrospective written and cited (kanban row 20)
## PLUS §11.3 — full worktree/branch sweep

The final-acceptance scan PASSED on attempt 7:
${JSON.stringify(scanResult).slice(0, 4000)}

1. RETROSPECTIVE -> \`docs/retro/sd33-computed-value-verification-retrospective.md\`,
   grounded in \`python3 scripts/retro.py summary --since 2026-08-24 --json\` — READ
   that output. \`deferrals.open\` IS trustworthy (SD-32's fix landed). Confirm it.

   **THE HEADLINE IS THE DEFECTS.** SD-33 existed to answer one question: is a
   computed value that looks right actually right? It found real wrong answers
   and fixed them — the spell-DC fixture defect (103 units), the armor compute
   defect (22 units at \`abc72f75ec\`), the AC measurement method that conflated a
   MAXDEX cap (\`a68fbeea3d\`), \`compute_equipmods_effect\` multi-chain summing
   (\`2f1d52f22d\`), the OUTPUTNAME-divergent identity resolve, the
   campaign-KEY-vs-display-name harness defect, and the corpus-extraction gap
   that dropped \`.MOD\`-attached EQMOD references. Lead with these, each with its
   mechanism and its commit. Process comes second.

   **The second story is the throughput arc**, with denominators:
   32 -> 6,940 -> 7,939 -> 8,255 -> 8,263 -> 8,291 -> 8,330 of 8,330 examined;
   disagreements 26 -> 4 -> 1 -> 0.
   **Seven dispatch waves, six correct halts, one bundle.**

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
      shape** — one character per unit at n=8,330; a whole-character AC diff that
      could not isolate an armor value; an equipmod-attachment mechanism that
      never produced a trustworthy value. Mechanism: when a method stalls, change
      the method and RE-RUN EVERYTHING IT ALREADY JUDGED — and the scan verifies
      that re-run, because a corrected method leaves stale agreements that look
      exactly like real ones.
   f. **A blocker whose fix lives in another subsystem is still a fix.** The
      \`rending_claw_blades\` blocker was filed for escalation because the repair
      belonged to the corpus extraction pipeline; decomposing it closed it.
      Mechanism: \`blocker-closure-doctrine.md\`'s two-dispositions rule, enforced
      by the scan reading the \`## Open blockers\` section.
   Record also that the gate's own scan SCOPE was a defect, found by the scanner
   and closed by an instrument lane.

   Say plainly: six scans failed and each failure was the system working. The
   lanes wrote honest short rows rather than false greens — with one exception,
   wave 2's equipment lane, which row-counting caught.

   Close out workflow-instruction.md §12 rows 3 and 8, both UNENFORCED at launch.

2. CITE IT from \`${PKG}/references/README.md\` IN THIS SAME CYCLE.

3. FULL WORKTREE/BRANCH SWEEP. \`git worktree list\`, \`git branch -a\`, \`df -h /\`.
   Report count FOUND vs REMOVED. Seven dispatch runs left worktrees behind
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
  return `${standingRules('sd33-r6-archdocs-pr')}

## YOUR CRITERION: AT-33-E6-003 (part 1) — architecture docs, graphify, PR (kanban row 21)

Retrospective and sweep are DONE (that order is load-bearing). Prior cycle:
${JSON.stringify(prior).slice(0, 3000)}

Follow \`${REPO}/docs/release/template/template.md §6\` — read it first.

1. ARCHITECTURE DOCS — \`docs/architecture/\` is CURRENT-STATE TRUTH. Refresh for
   what SD-33 actually changed: the PCGen oracle harness and its batch
   generators, the per-type AC isolator that replaced the whole-character diff,
   the spell casting-ability mapping, the widened equipment bonus-shape coverage
   (VAR / COMBAT / WEAPON / WEAPONPROF / STAT / SAVE / SITUATION / SKILL /
   natural-attack / special-material / EQM* modifiers), the armor and
   \`compute_equipmods_effect\` fixes, the \`equipment_id_resolve\` identity fix,
   **the corpus-extraction fix for \`.MOD\`-attached EQMOD references**, the
   L20-per-class pilot-build probe path, \`box_ledger.py\` and THE-BOX partition,
   the denominator gate and its widened scan scope, formula-interpreter
   corpus-wide coverage, and the work-inventory \`unknown\` -> zero classification.
2. GRAPHIFY per template §6.
3. OPEN THE PR: ${BRANCH} -> develop. Lead with the DEFECTS FOUND AND FIXED, cite
   the retrospective and receipts, state headline figures WITH DENOMINATORS, and
   say plainly that six final-acceptance scans failed and what each wave closed.
   That history is the strongest evidence the gate works.
4. Resolve merge conflicts if any. NEVER force-push. DO NOT MERGE — the operator
   merges tranche -> develop.

Commit and push (§5). Do NOT mark row 21 complete; release-notes owns it.

Return ONLY JSON:
{"criterion":"AT-33-E6-003-part1","status":"complete"|"blocked-escalated",
 "arch_docs_touched":[...],"graphify":"...","pr_url":"...","pr_number":0,
 "conflicts_resolved":"...","commit_shas":[...]}`
}

function releaseNotesVersionBumpPrompt(prPrior) {
  return `${standingRules('sd33-r6-release-notes')}

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
   Watch for bare hundred-percent tokens — the gate has caught three agents.

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

log('SD-33 REMEDIATION WAVE 6. Epic 5 at 8,291 of 8,330. Five lanes: clear the corpus-extraction Open blocker, verify+run the wave-5 method re-run, and close the last 39 (weapon 23, skill/combat 9, EQM 7 with a new mechanism).')

phase('Blocker, method, and the last 39')
const [corpusFix, methodRerun, weapon, skillCombat, eqm] = await parallel([
  () => agent(corpusExtractionPrompt(), { model: 'sonnet', label: 'corpus-extraction-blocker', phase: 'Blocker, method, and the last 39', isolation: 'worktree' }),
  () => agent(methodRerunPrompt(), { model: 'sonnet', label: 'method-rerun', phase: 'Blocker, method, and the last 39', isolation: 'worktree' }),
  () => agent(lanePrompt(LANES[0]), { model: 'sonnet', label: LANES[0].lane, phase: 'Blocker, method, and the last 39', isolation: 'worktree' }),
  () => agent(lanePrompt(LANES[1]), { model: 'sonnet', label: LANES[1].lane, phase: 'Blocker, method, and the last 39', isolation: 'worktree' }),
  () => agent(lanePrompt(LANES[2]), { model: 'sonnet', label: LANES[2].lane, phase: 'Blocker, method, and the last 39', isolation: 'worktree' }),
])
log('Wave-6 lanes returned. Check `df -h /` and `git worktree list` (§8).')

phase('Epic 5 finalize')
const finalize = await agent(finalizePrompt({ corpusFix, methodRerun, weapon, skillCombat, eqm }), {
  model: 'sonnet', label: 'e5-finalize-wave6', phase: 'Epic 5 finalize',
})

const summary = { corpusFix, methodRerun, weapon, skillCombat, eqm, finalize }

phase('Epic 6 — Closure epilogue')
const scan = await agent(finalAcceptanceScanPrompt(summary), {
  model: 'opus', label: 'final-acceptance-scan-attempt7', phase: 'Epic 6 — Closure epilogue',
})

const scanFailed = !scan || /"gate"\s*:\s*"FAIL"/.test(String(scan)) || /blocked-escalated/.test(String(scan))
if (scanFailed) {
  log('AT-33-E6-001 attempt 7 did NOT pass. Per §11 step 1: no retrospective, no sweep, NO PR. Correct outcome, not a failure.')
  return {
    bundle: 'SD-33', wave: 6, closed: false,
    halted_at: 'AT-33-E6-001 final-acceptance scan (attempt 7)',
    scan, summary,
  }
}

const retroSweep = await agent(retrospectiveAndSweepPrompt(scan), { model: 'sonnet', label: 'retrospective-and-sweep', phase: 'Epic 6 — Closure epilogue' })
const archPr = await agent(architectureDocsGraphifyPrPrompt(retroSweep), { model: 'sonnet', label: 'archdocs-graphify-pr', phase: 'Epic 6 — Closure epilogue' })
const notes = await agent(releaseNotesVersionBumpPrompt(archPr), { model: 'haiku', label: 'release-notes-version-bump', phase: 'Epic 6 — Closure epilogue' })

log('SD-33 closure epilogue complete. The operator merges tranche/13 -> develop.')

return {
  bundle: 'SD-33', wave: 6, branch: BRANCH, closed: true,
  summary,
  epic6: { scan, retroSweep, archPr, notes },
}
