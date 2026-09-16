export const meta = {
  name: 'sd-33-remediation-4',
  description: 'SD-33 remediation wave 4 — root-cause the 26 armor disagreements, close the last 75 units, re-run closure',
  whenToUse: 'After sd-33-remediation-3 halted at AT-33-E6-001 attempt 4 with 26 disagreements and 75 of 8,330 unexamined.',
  phases: [
    { title: 'Defects and remainder' },
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
  return `You are a dispatched SD-33 REMEDIATION WAVE 4 agent. Role: ${role}.

CONTEXT. SD-33 has run four dispatch waves and been halted four times by its own
final-acceptance scan. Every halt was correct. Epic 5 has moved
32 -> 6,940 -> 7,939 -> 8,255 of 8,330 units examined.

Two things now block closure, and they are DIFFERENT IN KIND:
- **26 real disagreements**, newly surfaced. These are the bundle WORKING — its
  entire purpose is to find computed values that are wrong, and it found some.
- **75 of 8,330 units** still unexamined.

Everything else is closed and must not be re-litigated: Epics 1/2/3/4; the
denominator gate (widened scope, detection re-proven live, not blinded);
the deferral posture; 0 reasonless \`unverifiable\`; 0 duplicate unit_ids;
\`box_ledger.py\` structurally green (\`uncovered=0 overlap=0 population=49438\`);
work-inventory \`unknown\` at 0; row 16 at 1,741 of 1,741.

REQUIRED READS, in order:
1. ${REPO}/CLAUDE.md
2. ${REPO}/AGENTS.md
3. ${REPO}/${PKG}/workflow-instruction.md   (dispatch procedure — binding)
4. ${REPO}/${PKG}/epic-breakdown.md          (AT-33-E5-002 / AT-33-E5-003, verbatim)
5. ${REPO}/${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt4_cycle_receipt.md
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
- NEVER hand-edit data/corpus/**. NEVER pass --allow-stamp-loss.

RETRO EVENTS (§2.3): emit via \`scripts/retro.py\` as they happen. \`--verified-by\`
required on a \`correction\` — and this wave produces real corrections.

FIGURES (decisions.md §2): every number states its denominator in the SAME
construct. The denominator gate's scope is now WIDE — it scans the package's
markdown documents, not just receipts. Run
\`scripts/verify.sh --only denominator-gate\` before you push. Bare
hundred-percent tokens are caught specifically.

NO STUBS. NO SAMPLING PRESENTED AS POPULATION. NO DoD-SCOPE DEFERRALS.

## YOUR STATUS MUST MATCH YOUR ROW COUNT
Wave 2's equipment lane returned \`"status":"complete"\` having written rows for
103 of its own 494-unit population; only row-counting caught it. Your \`status\`
is a mechanical function of your artifact, not a judgment about your effort.
Run the count yourself before writing your return JSON, and put its literal
output in the JSON. If the number is short of your population, your status is
\`blocked-escalated\` — an honest short report is a CORRECT outcome and is why
this bundle can still close truthfully.`
}

// ---------------------------------------------------------------------------
// Lane 1 — the 26 disagreements. The most important work in this wave.
// ---------------------------------------------------------------------------

function disagreementPrompt() {
  return `${standingRules('sd33-r4-disagreements')}

## YOUR CRITERION: AT-33-E5-003 — every disagreement is a named defect, fixed or escalated

**26 of 8,255 examined units disagree.** This is the bundle doing the job it was
built for. Treat these as real defects until proven otherwise, and root-cause
them properly.

### The 26, verbatim from \`${E5DIR}/AT-33-E5-003.combined-oracle-results.json\`

| unit_id | ours | oracle | delta |
|---|---:|---:|---:|
| inner_sea_races:equipment:armor_of_grim_triumph | 6 | 7 | -1 |
| inner_sea_races:equipment:coat_of_shells | 5 | 7 | -2 |
| inner_sea_races:equipment:gnome_scrap_armor | 3 | 5 | -2 |
| inner_sea_races:equipment:hallowed_chain | 6 | 8 | -2 |
| inner_sea_races:equipment:hallowed_chain_greater | 6 | 9 | -3 |
| inner_sea_races:equipment:hide_of_grim_triumph | 4 | 5 | -1 |
| inner_sea_races:equipment:mail_of_sly_steps | 4 | 6 | -2 |
| inner_sea_races:equipment:panoply_of_the_fierani_knight | 6 | 3 | **+3** |
| ultimate_intrigue:equipment:diviner_s_blight | 2 | 6 | -4 |
| advanced_class_guide:equipment:full_plate_of_the_corpse | 9 | 10 | -1 |
| advanced_class_guide:equipment:hero_s_hauberk | 4 | 5 | -1 |
| advanced_class_guide:equipment:stalking_armor_cold | 3 | 5 | -2 |
| advanced_class_guide:equipment:stalking_armor_desert | 3 | 5 | -2 |
| advanced_class_guide:equipment:stalking_armor_forest | 3 | 5 | -2 |
| advanced_class_guide:equipment:stalking_armor_jungle | 3 | 5 | -2 |
| advanced_class_guide:equipment:stalking_armor_mountain | 3 | 5 | -2 |
| advanced_class_guide:equipment:stalking_armor_plains | 3 | 5 | -2 |
| advanced_class_guide:equipment:stalking_armor_swamp | 3 | 5 | -2 |
| advanced_class_guide:equipment:stalking_armor_underground | 3 | 5 | -2 |
| advanced_class_guide:equipment:stalking_armor_urban | 3 | 5 | -2 |
| advanced_class_guide:equipment:stalking_armor_water | 3 | 5 | -2 |
| advanced_class_guide:equipment:tireless_tracking_hide | 4 | 5 | -1 |
| advanced_race_guide:equipment:sea_knife | -2 | 0 | -2 |
| inner_sea_world_guide:equipment:field_plate | 7 | 6 | **+1** |
| inner_sea_world_guide:equipment:stoneplate | 9 | 8 | **+1** |
| ultimate_equipment:equipment:snakeskin_tunic | 1 | 2 | -1 |

### What the shape suggests — a HYPOTHESIS to test, not a conclusion
Almost all are ARMOR, and most have \`ours\` LOW by 1-3. The eleven
\`stalking_armor_*\` variants are identical at 3 vs 5, which points at ONE
mechanism repeated eleven times, not eleven bugs. Three go the OTHER way
(\`panoply_of_the_fierani_knight\` +3, \`field_plate\` +1, \`stoneplate\` +1), which
suggests a SECOND, distinct mechanism. \`sea_knife\` at -2 vs 0 is a third shape
(negative on our side, zero on the oracle's) and may be a different thing again.

**Group by mechanism, then fix by mechanism.** Do not open 26 investigations.
Put the grouping table in the receipt with the count per group.

Read the WHOLE corpus record for each — a grep narrowed to BONUS/PRE hides
STACK/MULT and other application-governing fields, and those fields are exactly
what governs stacking behaviour. Likely suspects worth checking concretely:
armor bonus vs enhancement bonus stacking, masterwork, a special-material or
special-quality contribution not applied, and same-type bonus overwrite rules.

### THE BAR (verbatim from epic-breakdown.md)
> A disagreement is **never** closed by adjusting the expectation to match our
> output. Each is root-caused: either our computation is wrong (fix it) or the
> oracle comparison is wrong (fix the harness, and re-run everything it already
> judged).
> **Evidence:** one entry per disagreement in \`progress.md\`, each resolved to a
> commit or an operator escalation. **A filed blocker does not satisfy this
> criterion.**

Note the second limb. **If you fix the HARNESS, you must RE-RUN EVERYTHING IT
ALREADY JUDGED** — all 8,255 examined rows, not just these 26. Budget for that
before you choose that route, and say in the receipt which route you took and
why. If you fix OUR COMPUTE, that is a real product defect fix and needs its own
RED->GREEN plus the scoped Rust suite.

Both routes are legitimate. What is NOT legitimate is moving the expected value
to match our output, or reclassifying a disagreement to \`unverifiable\` to make
it go away. A disagreement you cannot resolve is escalated in \`progress.md\` with
its root-cause analysis — but note that "escalate" is for a genuine operator
ruling, not a way out of the work; four waves of this bundle have shown that a
blocker larger than one cycle is a SEQUENCING problem, not an exemption.

### FINISH LINE
\`python3 scripts/box_ledger.py --check --oracle-results ${E5DIR}/AT-33-E5-003.combined-oracle-results.json\`
exits 0 with \`oracle_disagreement=0\`, over a population where the 26 were
genuinely fixed rather than made invisible. Show the before and after.

Also: **re-prove the batch path can still return \`disagree\`.** After fixing 26
real defects, a zero-disagreement result is exactly when a silently-broken
comparison would look like success. Feed a known-disagreeing case through the
CURRENT path, show it returning \`disagree\`, then remove the probe.

### COORDINATION
One sibling lane is running RIGHT NOW on the 75 unexamined units, and it will be
APPENDING rows to the same merged files you are re-running. Write your corrected
rows to \`${E5DIR}/disagreement-fixes.oracle-results.json\` and let the finalize
cycle merge. Re-read before every shared edit; never revert a sibling's rows.
Do NOT mark kanban rows 16/17/18 — the finalize cycle owns that call.

### PROCEDURE
§6, all nine steps, §7 schema with \`- **Status:** <x>\` as a BULLET.
Receipt: \`${E5DIR}/AT-33-E5-003-disagreement-fixes_cycle_receipt.md\`
Write one \`progress.md\` entry per disagreement, each resolved to a commit.

### WRITE SCOPE
- \`src/rules_core/\` — the real compute fix, if that is the route; RED->GREEN required
- \`scripts/oracle_harness/\` — the harness fix, if that is the route
- \`src/bin/\` — the equipment probe binaries
- \`${E5DIR}/disagreement-fixes.oracle-results.json\` and your receipt

### RETURN VALUE — ONLY JSON:
{"lane":"disagreements","status":"complete"|"blocked-escalated",
 "disagreements_in":26,"disagreements_out":0,
 "mechanism_groups":[{"mechanism":"...","units":0,"route":"our-compute"|"harness","commit":"..."}],
 "route_taken":"our-compute"|"harness"|"both",
 "rerun_everything_after_harness_fix":true|false,"rows_rerun":0,
 "box_ledger_before":"...","box_ledger_after":"...",
 "disagree_capability_reproven_on_batch_path":true|false,
 "escalated":[{"unit_id":"...","why":"...","rootcause":"..."}],
 "progress_entries_written":0,
 "commit_shas":[...],"receipt_path":"...","red_green":"...",
 "identifier_audit":"...","wired_audit":"...","next":"..."}`
}

// ---------------------------------------------------------------------------
// Lane 2 — the last 75.
// ---------------------------------------------------------------------------

function remainderPrompt() {
  return `${standingRules('sd33-r4-last75')}

## YOUR SLICE: the last 75 unexamined units

**Population: 75 of 8,330** — 61 \`equipment\` + 14 \`equipment_modifier\`,
all \`literal-verified\`. The attempt-4 scan derived the SET, not just the count,
and named every id. Re-derive it yourself as your first action:

    python3 -c "import json,collections
wi=json.load(open('docs/work-inventory.json'))['units']
pop={u['id'] for u in wi if u.get('status') in ('literal-verified','fixture-verified')}
d=json.load(open('${E5DIR}/AT-33-E5-003.combined-oracle-results.json'))['results']
miss=sorted(pop-{r['unit_id'] for r in d})
print(len(miss)); [print(m) for m in miss]"

**If that yields a different set than 75, YOUR population is what it actually
yields** — say so with the command and cover all of it. A sibling lane is
re-running rows right now, so re-derive rather than inherit.

### THE SHAPES — grouped from the named ids
- **Natural-attack bonuses** — \`amulet_of_mighty_fists_1..5\`, \`talons_of_leng\`,
  \`claw_blades_catfolk\`, \`rending_claw_blades\`, \`flurry_of_fists\`,
  \`flurry_of_strikes\`, \`brawler_s_flurry\`. These bonus an attack form the
  wave-3 combat path did not model.
- **Rods and staves** — \`rod_alertness\`, \`rod_flailing\`, \`rod_python\`,
  \`rod_thunder_and_lightning\`, \`rod_viper\`, \`rod_withering\`,
  \`staff_of_the_hierophant\`, \`mattock_of_the_titans\`.
- **Wield-size / shield-bash modifiers** — the six
  \`special_quality_wield_size_*\` ids plus \`spike_sb\`,
  \`special_quality_spikes_shieldbash\`. These change how an item is wielded
  rather than adding a scalar bonus.
- **Special materials** — \`dragonhide\`, \`material_dragonhide\`, \`draco\`,
  \`material_darkleaf_cloth_clothing\`.
- **Psionic items** — the \`ultimate_psionics\` block (crystal masks, psychoactive
  skins, meld stones, companion stones, third eye, dissonance modifiers).
  **Check first whether the pinned PCGen oracle even carries Ultimate Psionics
  data.** If it does not, every psionic unit is a legitimate \`unverifiable\` with
  that exact named reason — prove it by looking at the oracle checkout's data
  paths, and say so.
- **Cursed and odd items** — \`berserking_sword\`, \`cursed_backbiter_spear\`,
  \`cursed_sword_2\`, \`ornery_pistol\`, \`robe_of_vermin\`,
  \`horseshoes_of_crushing_blows_1..5\`, \`belt_of_teeth\`, \`heavy_hammer\`,
  \`gunfighter_s_poncho\`, \`scattershot_bracers\`, and the rest.

Group by mechanism, count each group, put the table in the receipt, then handle
them by group. Do not open a sub-lane per item.

### THROUGHPUT
75 units is small; setup cost dominates. Reuse \`scripts/oracle_harness/\` and the
existing equipment probe binaries under \`src/bin/\` — do not rebuild. One
exported character carrying N computed variables verifies N units per JVM start.
Measure on ~10 units, project, then run.

### VERDICT DISCIPLINE
\`unverifiable\` is a FIRST-CLASS verdict and correct when a unit genuinely has no
comparable computed magnitude — reuse the established \`no_bonus_chain\` /
\`no_probe_surface\` vocabulary rather than inventing a parallel one. A
wield-size modifier that changes a size category rather than a number, or a
psionic item absent from the oracle's data, are real \`unverifiable\` results WITH
THEIR REASON.

**\`unverifiable\` is not a parking space for a unit you could not reach.** Every
\`unverifiable\` row MUST carry a populated reason field — the current files have
zero reasonless rows and must stay that way.

**If a unit DOES yield a comparable value and it disagrees, that is a real
finding — record the \`disagree\` honestly.** A sibling lane is fixing 26
disagreements right now; do not suppress a 27th to keep the count tidy.

### COORDINATION
Write your rows ONLY to \`${E5DIR}/equipment-last75.oracle-results.json\`. Never
rewrite the merged files or a sibling's file. The sibling disagreement lane is
re-running rows concurrently. Re-read before every shared edit; rebase before
every push. Do NOT mark kanban rows 16/17/18 — finalize owns that call.

### PROCEDURE
§6, all nine steps, §7 schema with \`- **Status:** <x>\` as a BULLET.
Receipt: \`${E5DIR}/AT-33-E5-last75_cycle_receipt.md\`

### WRITE SCOPE
- \`scripts/oracle_harness/\` — export-template widening for these shapes
- \`src/rules_core/\` — only for a genuinely unhandled shape; RED->GREEN required
- \`src/bin/\` — extend the existing equipment probe binaries
- \`${E5DIR}/equipment-last75.oracle-results.json\` and your receipt

### RETURN VALUE — ONLY JSON:
{"lane":"last75","status":"complete"|"blocked-escalated",
 "population_rederived":0,"rows_written":0,
 "agree":0,"disagree":0,"unverifiable":0,"unexamined":0,"reasonless_unverifiable":0,
 "row_count_command_output":"<literal output of your own len() count>",
 "denominator_statement":"<X of 75 remaining units>",
 "shape_table":[{"shape":"...","population":0,"examined":0,"verdicts":{}}],
 "psionics_in_oracle_data":true|false,"psionics_finding":"...",
 "disagreements":[{"unit_id":"...","ours":"...","oracle":"..."}],
 "commit_shas":[...],"receipt_path":"...","red_green":"...",
 "identifier_audit":"...","wired_audit":"...","next":"..."}`
}

// ---------------------------------------------------------------------------
// Finalize.
// ---------------------------------------------------------------------------

function finalizePrompt(laneResults) {
  return `${standingRules('sd33-r4-e5-finalize')}

## YOUR JOB: total Epic 5 and own the kanban call on rows 16, 17, 18

Wave 4's two lanes just ran — one fixing 26 disagreements, one closing the last
75 unexamined units. Reports:
${JSON.stringify(laneResults).slice(0, 8000)}

**Reports are not evidence.** Wave 2's equipment lane returned \`complete\` over
103 of 494. Derive every number yourself by counting rows and deriving sets.

## WHAT YOU MUST ESTABLISH
1. **Merge every lane results file** into the canonical artifacts:
   - \`${E5DIR}/fixture-verified.combined-oracle-results.json\` -> exactly 1,741 rows
   - \`${E5DIR}/literal-verified.oracle-results.json\` -> exactly 6,589 rows
   - \`${E5DIR}/AT-33-E5-003.combined-oracle-results.json\` -> exactly 8,330 rows
   Merge on \`unit_id\`. The disagreement lane's corrected rows SUPERSEDE the stale
   rows for those same unit_ids — that is the one legitimate overwrite, and you
   must state which unit_ids you superseded and why. Any OTHER duplicate
   \`unit_id\` across lanes is a real finding to root-cause, never last-writer-wins.
2. **Derive the unexamined SET, not its size.** Take the fixture+literal unit_id
   set from \`docs/work-inventory.json\`, subtract the merged file's unit_ids,
   print the difference. It must be empty. If not, print the missing ids and
   their shapes, and your status is \`blocked-escalated\`.
3. **Zero unresolved \`disagree\`.** Re-derive across all 8,330:
   \`python3 scripts/box_ledger.py --check --oracle-results ${E5DIR}/AT-33-E5-003.combined-oracle-results.json\`
   must exit 0 with \`oracle_disagreement=0\`. If the last-75 lane surfaced a NEW
   disagreement, it is root-caused here the same way — never suppressed, never
   closed by moving the expected value.
4. **Zero reasonless \`unverifiable\`** across all three files. Re-derive.
5. **Re-prove \`disagree\` capability on the current batch path.** After 26 real
   fixes, a zero-disagreement result is exactly when a silently-broken comparison
   looks like success. Feed a known-disagreeing case through the CURRENT path,
   show it returning \`disagree\`, then remove the probe.
6. **Keep the gate green.** \`scripts/verify.sh --only denominator-gate\` scans the
   package's markdown documents now. Run it before you push; fix your own prose
   if it trips. Do not narrow the scope to pass.

## THEN THE KANBAN CALL
Mark rows 16, 17, 18 \`complete\` **only if** each population is fully rowed with
its denominator stated and zero disagreements remain. If any is short, leave the
row honest and report. Update the AT-33-E5-00{1,2,3} receipts' figure rows to
final totals.

## PROCEDURE
§6, all nine steps, §7 schema with \`- **Status:** <x>\` as a BULLET.
Receipt: \`${E5DIR}/AT-33-E5-finalize-wave4_cycle_receipt.md\`

## RETURN VALUE — ONLY JSON:
{"task":"e5-finalize-wave4","status":"complete"|"blocked-escalated",
 "fixture":{"population":1741,"rows":0,"distinct":0,"agree":0,"disagree":0,"unverifiable":0},
 "literal":{"population":6589,"rows":0,"distinct":0,"agree":0,"disagree":0,"unverifiable":0},
 "combined":{"population":8330,"rows":0,"distinct":0,"agree":0,"disagree":0,"unverifiable":0},
 "missing_unit_ids":[...],"superseded_unit_ids":[...],"unexpected_duplicates":[...],
 "reasonless_unverifiable":0,
 "box_ledger_oracle_disagreement":0,
 "disagree_capability_reproven_on_batch_path":true|false,
 "kanban":{"row16":"complete"|"in-progress","row17":"...","row18":"..."},
 "denominator_gate":"PASS"|"FAIL",
 "commit_shas":[...],"receipt_path":"..."}`
}

// ---------------------------------------------------------------------------
// Epic 6, attempt 5.
// ---------------------------------------------------------------------------

function finalAcceptanceScanPrompt(summary) {
  return `${standingRules('sd33-r4-acceptance-scan')}

## YOUR CRITERION: AT-33-E6-001 — final-acceptance scan, ATTEMPT 5 (kanban row 19)

Four prior scans FAILED and correctly halted the bundle. Read attempt 4's
receipt: \`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt4_cycle_receipt.md\`

Attempt 4 confirmed CLOSED — re-verify rather than re-investigate: row 16 at
1,741 of 1,741; the denominator gate widened (globs now cover the package's
markdown documents) and NOT blinded, detection re-proven live; the deferral
posture; 0 reasonless \`unverifiable\`; 0 duplicate unit_ids; \`box_ledger.py\`
structurally green; work-inventory \`unknown\` at 0; Epic 3's artifact at the SD-33
path with SD-32's untouched.

Attempt 4's two surviving shortfalls: **26 disagreements** and **75 of 8,330
unexamined**. Two lanes and a finalize cycle have since run. Reports:
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
miss=sorted(pop-{x['unit_id'] for x in c}); print('MISSING',len(miss)); [print(' ',m) for m in miss[:50]]"
Every population must equal its denominator EXACTLY, and MISSING must be 0. A
count can match while membership does not — the SET is the check.

## CHECK 2 — THE DISAGREEMENTS WERE FIXED, NOT HIDDEN
This is the check that matters most this attempt. 26 units disagreed. Verify:
    python3 scripts/box_ledger.py --check --oracle-results ${E5DIR}/AT-33-E5-003.combined-oracle-results.json; echo $?
must print \`oracle_disagreement=0\` and exit 0. Then go further — a zero can be
produced three legitimate ways and three illegitimate ones:
- LEGITIMATE: our compute was fixed; or the harness was fixed AND everything it
  already judged was re-run; or a genuine operator escalation is recorded.
- ILLEGITIMATE, and each is a BLOCKING shortfall if you find it:
  (a) the expected value was moved to match our output;
  (b) a disagreeing unit was reclassified to \`unverifiable\` or dropped entirely;
  (c) the harness was fixed but its prior verdicts were NOT re-run.
Spot-check several of the 26 by name — \`hallowed_chain_greater\` (was 6 vs 9),
\`diviner_s_blight\` (2 vs 6), \`panoply_of_the_fierani_knight\` (6 vs 3, the
opposite direction), \`sea_knife\` (-2 vs 0), and two \`stalking_armor_*\` — and
confirm each now carries a real \`agree\` with a value, still present in the file,
traced to a compute or harness commit in \`git log\`. Read that commit's diff.
If the harness route was taken, verify the re-run actually happened.

**Re-prove \`disagree\` capability on the current batch path yourself.** After 26
fixes, a broken comparison and a clean result look identical. Feed a
known-disagreeing case through, show \`disagree\`, remove the probe.

Also verify one \`progress.md\` entry per disagreement, each resolved to a commit
or a genuine escalation. A filed blocker does not satisfy AT-33-E5-003.

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
- grep the closure INSTRUMENTS for hardcoded exclusion lists
- \`scripts/verify.sh --only denominator-gate\` exits 0, scope still wide, matcher
  not relaxed — re-prove detection live, then remove your probe
- \`jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json\` is 0
- Epic 3's corpus-wide artifact at the SD-33 path; SD-32's
  \`docs/release/SD-32-.../artifacts/gate-2-engines/\` file UNTOUCHED
- enumerate open deferrals; none defers DoD scope; all carry a revisit condition
- the Rust suite is green for whatever \`src/rules_core/\` changed this wave

IF ANYTHING IS SHORT: STOP. No retrospective, no sweep, NO PR. Report what is
short WITH THE COMMAND THAT SHOWS IT. Four prior scans did exactly that and all
four were right. You are the scanner, not an executor.

Receipt: \`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt5_cycle_receipt.md\`
Commit and push (§5). Mark kanban row 19 \`complete\` only on PASS.

Return ONLY JSON:
{"criterion":"AT-33-E6-001","attempt":5,"gate":"PASS"|"FAIL",
 "status":"complete"|"blocked-escalated",
 "row_counts":{"fixture":0,"literal":0,"combined":0},
 "unexamined_set_empty":true|false,"missing_unit_ids":[...],
 "oracle_disagreement":0,"disagreements_fixed_not_hidden":true|false,
 "spot_checks":[{"unit_id":"...","was":"...","now":"...","commit":"...","verdict":"real-fix"|"hidden"}],
 "disagree_capability_reproven":true|false,
 "gate_scope_widened_not_blinded":true|false,
 "prior_shortfalls_closed":[{"shortfall":"...","closed":true|false,"command":"...","output":"..."}],
 "shortfalls":[{"what":"...","command":"...","output":"..."}],
 "commands_rerun":[{"command":"...","output":"..."}],
 "receipt_path":"...","commit_shas":[...]}`
}

function retrospectiveAndSweepPrompt(scanResult) {
  return `${standingRules('sd33-r4-retro-sweep')}

## YOUR CRITERION: AT-33-E6-002 — retrospective written and cited (kanban row 20)
## PLUS §11.3 — full worktree/branch sweep

The final-acceptance scan PASSED on attempt 5:
${JSON.stringify(scanResult).slice(0, 4000)}

1. RETROSPECTIVE -> \`docs/retro/sd33-computed-value-verification-retrospective.md\`,
   grounded in \`python3 scripts/retro.py summary --since 2026-08-24 --json\` — READ
   that output. \`deferrals.open\` IS trustworthy (SD-32's fix landed:
   \`grep -n 'len(open_deferrals)' scripts/retro.py\`). Confirm it and say so.

   **THE HEADLINE IS THE DEFECTS.** SD-33 was built to answer one question: is a
   computed value that looks right actually right? It found real wrong answers —
   the 26 armor disagreements of wave 4, plus the earlier 103 — and fixed them.
   Lead with that, with the mechanism behind each group, not with process.

   **The second story is the throughput arc**, told with denominators:
   - Epic 2 proved the oracle at n=1 — one hand-authored PCGen character.
   - Epic 5 carried that method to a population of 8,330 without measuring its
     per-unit cost, and reached 32 of 8,330.
   - Scan 1 halted. Generators -> 6,940 of 8,330.
   - Scan 2 halted. The 1,390 remainder split into three MECHANISMS -> 7,939.
   - Scan 3 halted. The 391 remainder split by bonus-shape family -> 8,255.
   - Scan 4 halted on 26 disagreements and 75 of 8,330 -> 8,330 of 8,330.
   **Five dispatch waves, four correct halts, one bundle.**

   Lessons, each with its ENFORCING MECHANISM (a lesson without a mechanism is a
   quote — decisions.md §4):
   a. **Measure per-unit cost before a population-scoped run.** Mechanism: a
      required dispatch-brief field — measured cost, population, projected wall
      time — filled before the run starts.
   b. **A remainder named per-MECHANISM is closable; "the rest" is not.** Every
      wave shrank because the prior wave named its remainder specifically.
      Mechanism: a required per-shape enumeration of anything unexamined.
   c. **A lane's status must be a mechanical function of its row count.** Wave 2's
      equipment lane returned \`complete\` over 103 of 494; only row-counting
      caught it. Mechanism: the scan counts rows and derives the unexamined SET.
   d. **Coverage growth surfaces defects late.** The 26 disagreements appeared
      only in wave 3's newly-covered shapes — a bundle that had stopped at 95%
      would have shipped them. Mechanism: the scan's set-difference check, and
      the rule that \`unverifiable\` needs a populated reason.
   Record also that the gate's own scan SCOPE was a defect (\`DEFAULT_GLOBS\` could
   not see the bundle's headline documents), found by the scanner and closed by
   an instrument lane.

   Say plainly: four scans failed and each failure was the system working. The
   lanes wrote honest short rows rather than false greens — with one exception,
   wave 2's equipment lane, which row-counting caught. That is why this closed
   truthfully.

   Close out workflow-instruction.md §12 rows 3 and 8, both UNENFORCED at launch.

2. CITE IT from \`${PKG}/references/README.md\` IN THIS SAME CYCLE.

3. FULL WORKTREE/BRANCH SWEEP. \`git worktree list\`, \`git branch -a\`, \`df -h /\`.
   Report count FOUND vs REMOVED. Five dispatch runs left worktrees behind
   (\`.claude/worktrees/wf_*\`). Check each for unmerged commits BEFORE removing.
   NEVER remove a \`locked\` worktree or one carrying unmerged commits — report those.

NO PR IN THIS CYCLE. Steps 2 and 3 land before the PR opens.

Receipts to \`${PKG}/artifacts/epic-6-closure/\`. Commit, push (§5), mark row 20
complete. Run \`scripts/verify.sh --only denominator-gate\` on your own prose
before pushing — its scope covers the package's markdown documents.

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
  return `${standingRules('sd33-r4-archdocs-pr')}

## YOUR CRITERION: AT-33-E6-003 (part 1) — architecture docs, graphify, PR (kanban row 21)

Retrospective and sweep are DONE (that order is load-bearing). Prior cycle:
${JSON.stringify(prior).slice(0, 3000)}

Follow \`${REPO}/docs/release/template/template.md §6\` — read it first.

1. ARCHITECTURE DOCS — \`docs/architecture/\` is CURRENT-STATE TRUTH. Refresh for
   what SD-33 actually changed: the PCGen oracle harness and its batch
   generators, the spell casting-ability mapping, the widened equipment
   bonus-shape coverage (VAR / COMBAT / WEAPON / STAT / SAVE / SITUATION /
   natural-attack / special-material), **the armor-bonus compute fixes behind the
   26 disagreements**, the L20-per-class pilot-build probe path,
   \`box_ledger.py\` and THE-BOX partition, the denominator gate in \`verify.sh\`
   and its widened scan scope, formula-interpreter corpus-wide coverage, and the
   work-inventory \`unknown\` -> zero classification.
2. GRAPHIFY per template §6.
3. OPEN THE PR: ${BRANCH} -> develop. The body leads with the DEFECTS FOUND AND
   FIXED, cites the retrospective and receipts, and states headline figures WITH
   THEIR DENOMINATORS. State plainly that four final-acceptance scans failed and
   what each wave closed — that history is the strongest evidence the gate works,
   and it belongs in the PR.
4. Resolve merge conflicts if any. NEVER force-push. DO NOT MERGE — the operator
   merges tranche -> develop.

Commit and push (§5). Do NOT mark row 21 complete; release-notes owns it.

Return ONLY JSON:
{"criterion":"AT-33-E6-003-part1","status":"complete"|"blocked-escalated",
 "arch_docs_touched":[...],"graphify":"...","pr_url":"...","pr_number":0,
 "conflicts_resolved":"...","commit_shas":[...]}`
}

function releaseNotesVersionBumpPrompt(prPrior) {
  return `${standingRules('sd33-r4-release-notes')}

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
6. Re-run \`scripts/verify.sh --only denominator-gate\` on your own new prose and
   confirm it exits 0 before pushing. Its scope includes \`release-notes.md\`, so
   your own prose IS scanned. Watch for bare hundred-percent tokens.

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

log('SD-33 REMEDIATION WAVE 4. Epic 5 at 8,255 of 8,330 examined, with 26 real disagreements surfaced — the bundle working as designed. Two lanes: fix the 26 defects; close the last 75.')

phase('Defects and remainder')
const [disagreements, last75] = await parallel([
  () => agent(disagreementPrompt(), { model: 'sonnet', label: 'disagreement-fixes-26', phase: 'Defects and remainder', isolation: 'worktree' }),
  () => agent(remainderPrompt(), { model: 'sonnet', label: 'last-75-units', phase: 'Defects and remainder', isolation: 'worktree' }),
])
log('Wave-4 lanes returned. Check `df -h /` and `git worktree list` (§8).')

phase('Epic 5 finalize')
const finalize = await agent(finalizePrompt({ disagreements, last75 }), {
  model: 'sonnet', label: 'e5-finalize-wave4', phase: 'Epic 5 finalize',
})

const summary = { disagreements, last75, finalize }

phase('Epic 6 — Closure epilogue')
const scan = await agent(finalAcceptanceScanPrompt(summary), {
  model: 'opus', label: 'final-acceptance-scan-attempt5', phase: 'Epic 6 — Closure epilogue',
})

const scanFailed = !scan || /"gate"\s*:\s*"FAIL"/.test(String(scan)) || /blocked-escalated/.test(String(scan))
if (scanFailed) {
  log('AT-33-E6-001 attempt 5 did NOT pass. Per §11 step 1: no retrospective, no sweep, NO PR. Correct outcome, not a failure.')
  return {
    bundle: 'SD-33', wave: 4, closed: false,
    halted_at: 'AT-33-E6-001 final-acceptance scan (attempt 5)',
    scan, summary,
  }
}

const retroSweep = await agent(retrospectiveAndSweepPrompt(scan), { model: 'sonnet', label: 'retrospective-and-sweep', phase: 'Epic 6 — Closure epilogue' })
const archPr = await agent(architectureDocsGraphifyPrPrompt(retroSweep), { model: 'sonnet', label: 'archdocs-graphify-pr', phase: 'Epic 6 — Closure epilogue' })
const notes = await agent(releaseNotesVersionBumpPrompt(archPr), { model: 'haiku', label: 'release-notes-version-bump', phase: 'Epic 6 — Closure epilogue' })

log('SD-33 closure epilogue complete. The operator merges tranche/13 -> develop.')

return {
  bundle: 'SD-33', wave: 4, branch: BRANCH, closed: true,
  summary,
  epic6: { scan, retroSweep, archPr, notes },
}
