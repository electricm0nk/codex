export const meta = {
  name: 'sd-33-fold-fix',
  description: 'SD-33 — re-pin F1 after the fold, re-run lib LAST, then re-scan and refresh closure docs',
  whenToUse: 'After sd-33-fold-recovered-work halted at AT-33-E6-001 attempt 11 on one stale count assertion.',
  phases: [{ title: 'Re-pin and re-green' }, { title: 'Re-run closure' }],
}

const REPO = '/home/ubuntu/workspace/repos/codex'
const PKG = 'docs/release/SD-33-computed-value-verification'
const BRANCH = 'tranche/13'

function standingRules(role) {
  return `You are a dispatched SD-33 FOLD-FIX agent. Role: ${role}.

## SITUATION
The fold of recovered SD-31 work landed and was verified. Attempt 11 of the
final-acceptance scan then FAILED on **one** blocking item, and it is an **ordering
bug, not a data problem**.

**Verified good by attempt 11 — do NOT re-litigate:**
- 75 Skinwalker race-trait corpus records present (was 10, **+65**); the scan
  independently hand-traced 2 back to the pinned oracle \`.lst\`, byte-identical
- \`corpus_literal_sweep\`: **48,699 records examined, grown from 48,634 by exactly +65**,
  **0 findings** — the new records are genuinely inside the examined population, not
  passing vacuously. Mutation probe on a *folded* record caught it correctly.
- Epic 5 untouched: rows 1,741 / 6,589 / 8,330, unexamined set empty **both directions**,
  0 disagree, 0 dups, 0 reasonless \`unverifiable\`. \`fixture-verified\` and
  \`literal-verified\` counts unchanged, so no oracle re-run was owed.
- license / PI intact on the new records; \`box_ledger.py --check\` exit 0;
  work-inventory \`unknown\` 0 of 49,438; \`## Open blockers\` empty; denominator gate
  0 violations of 67 files; desktop 548 of 548; \`--no-run\` exit 0, 543 of 543 targets.
- The fold **fixed** two previously-failing suites (\`ingest_races\`,
  \`sd27_alternate_racial_trait_reachability\`).

## REQUIRED READS
1. ${REPO}/CLAUDE.md
2. ${REPO}/AGENTS.md
3. ${REPO}/${PKG}/workflow-instruction.md §5, §6, §7
4. ${REPO}/${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt11_cycle_receipt.md

## ENVIRONMENT
  export RETRO_ACTOR="${role}"
  export CARGO_TARGET_DIR="/tmp/cargo-sd33-${role}"
  export CARGO_INCREMENTAL=0
  mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"

## ONE TURN ONLY. Foreground slow work. Commit and push before ending the turn.

## GIT DISCIPLINE (§5)
- \`git status --porcelain\` before EVERY git write. Inspect it.
- NEVER \`git add -A\`. NEVER \`git stash\`. NEVER force-push.
- Push: \`git fetch origin ${BRANCH} && git rebase origin/${BRANCH} && git push origin HEAD:${BRANCH}\`
- **Do not delete, merge or rebase the five stale local branches.** Operator's to dispose of.
- **Do not hand-edit \`data/corpus/**\`.** Guarded generator path only.

## PROBE HYGIENE — a near-miss the last scan recorded
A \`corpus_literal_sweep\` mutation probe used \`cp -al\` (hardlinks), so writing the mutant
through the scratch path **truncated the shared inode and dirtied the real
\`data/corpus\` file**. It was caught only by the \`git status --porcelain\` that follows
every git write. If you plant a probe: \`rm\` the scratch file before writing, or copy
**without** \`-l\`. Then verify both trees clean.`
}

function repinPrompt() {
  return `${standingRules('sd33-fold-fix-repin')}

## YOUR TASK — close attempt 11's one blocking shortfall

### THE DEFECT
\`rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::f1_population_matches_the_current_true_formula_bearing_count_not_the_stale_sd32_census\`
is RED at \`src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs:616\`:

    assertion \`left == right\` failed: left: 6257, right: 6260

**It is SD-33's own, not inherited debt** — \`git log --oneline f652db7ac7..HEAD\` on that
file returns 3 commits since the cut, against 0 of 29 for the inherited failing set.

**Root cause, already derived — an ORDERING bug:**
1. \`6e2f2f076b\` (the Skinwalker fold) correctly re-pinned F1 **6,278 → 6,260** against the
   \`docs/work-inventory.json\` committed *at that moment*.
2. \`cef0ca1b39\` (fold-inventory) then **regenerated** that file — 89 units moved status —
   and **did not re-run the lib suite afterwards**.
3. Three units left F1's \`not_done_population()\` gate:
   \`bestiary_5:race_trait:skinwalker_speed\`,
   \`ultimate_psionics:equipment_modifier:plusn_svs\`,
   \`ultimate_psionics:equipment_modifier:special_quality_severis_enhancement_bonus\`.

fold-inventory's receipt reports "lib 2845 passed, 0 failed". That was a **true measurement
of the tree before its own work-inventory write** — not of the tree it landed.

### STEP 1 — re-derive F1 yourself
Do **not** take 6,257 on trust; it is a number from a report.

    python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus

State the live F1 population with the command that produced it. Confirm the three named
units are the movers, by id.

### STEP 2 — re-pin, and fix the doc comment with it
Update the assertion to the live value. **The assertion's doc comment carries the old
figure and its re-derive date — update both**, or the next reader inherits a stale number
from prose, which is how this bundle has been bitten twice already.

### STEP 3 — the ordering rule, made mechanical in the receipt
The lesson is not "re-pin the number". It is **run the suite AFTER the last write that can
move it.** In your receipt, state the order you ran things in, explicitly:
work-inventory write → shape ledger re-derive → re-pin → **lib suite last**.

### STEP 4 — the stale prose attempt 11 found (non-blocking, close it anyway)
Neither is a live assertion, but both state numbers that are now wrong:
- \`src/bin/v06_work_inventory.rs:4308\` — comment reads "every one of the 831
  currently-ingested race_trait records"; the live count is **910**.
- \`src/rules_core/race_resolver.rs:3105\` — an assertion *message* says "same as the other
  370"; live is **415**. (The assertions themselves at \`:2680\` and \`:3378\` were correctly
  re-pinned; only the message text is stale.)

Re-derive both numbers yourself before writing them. Then sweep for other stale figures the
fold moved:

    for n in 831 910 370 415 6278 6260 6257 48634 48699; do grep -rn "\\b$n\\b" src/ tests/ apps/ scripts/ | grep -v '^Binary'; done

Report what you found and what you changed. A stale figure in shipped prose drifts
roster-wide — that is a recorded lesson in this repo, not a style nit.

### STEP 5 — record the attribution correction
Attempt 11 filed an instrument-correction that is **not** yours to fix but **is** yours to
record: \`fold-inventory_cycle_receipt.md\`'s fold-attribution split of the 89 moved units
says 14 fold-attributable / 75 drift. The real split is **50 / 39** — its
\`'skinwalker' in id\` substring test misses the 36 \`were*_kin_*\` ids, all of which map to a
corpus file the fold created. One of the three units that left F1 sits inside those 36.

Emit a \`correction\` retro event with \`--verified-by\`, and note it in your receipt. Epic 5's
population is unaffected — that was independently re-derived.

### FINISH LINE — order matters
    python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus
    cargo test --locked --lib          # LAST, after every write above. Must be 0 failed.
    cargo test --locked --no-run       # exit 0
    cd apps/desktop/src-tauri && cargo test --locked
    cargo run --locked --bin corpus_literal_sweep      # 48,699 examined, 0 findings
    scripts/verify.sh --only denominator-gate

**Do NOT regenerate \`docs/work-inventory.json\`.** It is correct as landed; regenerating it
would restart exactly the ordering problem you are closing.

### PROCEDURE
§6, all nine steps, §7 schema with \`- **Status:** <x>\` as a BULLET.
Receipt: \`${PKG}/artifacts/epic-6-closure/fold-fix-repin_cycle_receipt.md\`

### RETURN VALUE — ONLY JSON:
{"lane":"fold-fix-repin","status":"complete"|"blocked-escalated",
 "f1_live":0,"f1_rederive_command":"...","three_movers_confirmed":true|false,
 "repinned_from":6260,"repinned_to":0,"doc_comment_updated":true|false,
 "stale_prose_fixed":[{"file":"...","line":0,"was":0,"now":0}],
 "figure_sweep_result":"...",
 "attribution_correction_logged":true|false,
 "run_order_stated":"...",
 "lib_suite":{"passed":0,"failed":0},"no_run_exit":0,"desktop_suite":"...",
 "corpus_sweep":{"examined":0,"findings":0},"denominator_gate":"PASS"|"FAIL",
 "work_inventory_untouched":true|false,
 "commit_shas":[...],"receipt_path":"...","red_green":"..."}`
}

function rescanPrompt(fix) {
  return `${standingRules('sd33-fold-fix-rescan')}

## YOUR TASK — AT-33-E6-001, attempt 12

Attempt 11 FAILED on one blocking item: a stale F1 count assertion, caused by the lib suite
being run **before** the final work-inventory write rather than after. One lane has since
closed it. Report (**a report, not evidence**):
${JSON.stringify(fix).slice(0, 5000)}

Read attempt 11's receipt first —
\`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt11_cycle_receipt.md\` — and re-verify
its confirmed-good list rather than re-investigating it (summarised in your standing rules).

### CHECK 1 — the blocking item is genuinely closed, and closed the right way
    python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus
    cargo test --locked --lib ; echo EXIT=$?

- \`cargo test --locked --lib\` must be **0 failed**.
- The re-pinned constant must equal the **live** F1 value you derive yourself.
- **Read the diff.** The legitimate fix is re-pinning the assertion to a re-derived value.
  Weakening the assertion, deleting it, \`#[ignore]\`-ing it, or loosening it to a range is
  BLOCKING.
- The assertion's **doc comment** must carry the new figure too, not the old one.
- Confirm \`docs/work-inventory.json\` was **not** regenerated again
  (\`git log --oneline -- docs/work-inventory.json\`) — doing so would restart the ordering
  problem and invalidate the re-pin.

### CHECK 2 — nothing the fold established has moved
- \`corpus_literal_sweep\`: **48,699 examined, 0 findings**. If the examined count dropped,
  the new records left the population — BLOCKING.
- Epic 5: rows 1,741 / 6,589 / 8,330, unexamined set empty **derived as a set**, 0 disagree,
  0 reasonless \`unverifiable\`, 0 dups.
- \`box_ledger.py --check\` exit 0; work-inventory \`unknown\` 0 of 49,438.
- 75 Skinwalker records still present with \`license\` / \`pi_field\` intact.
- \`--no-run\` exit 0 with 543 of 543 targets; desktop 548 of 548.

### CHECK 3 — the inherited failing set did not grow
Attempt 11 measured **30 failing suites / 47 failures of 8,034 executed**, of which 29 are
inherited (0 of 29 carry a commit since \`f652db7ac7\`) and 1 was SD-33's own — the F1 test.
With that closed, the set should be back to **29 / 46**.

Re-derive it. **A failure outside the inherited 29 is SD-33's and is BLOCKING**, and
"pre-existing" must be proven against the cut SHA with \`git\`, never asserted.

### CHECK 4 — the fold's own quality holds
- Spot-check 2 Skinwalker records back to the pinned oracle \`.lst\` yourself.
- Confirm the Undine fixtures are executed by a real test and their expected values are not
  transcribed from the file the engine reads (a mirror validates fabricated values).
- Confirm the stale-prose figures are now right, by re-deriving them.

**Re-prove the gates still fail.** Plant a violation, confirm the catch, remove it, confirm
the baseline returns to zero. **Use \`rm\` before writing a scratch file, or copy without
\`-l\`** — attempt 11's hardlink probe dirtied the real corpus file. Verify both trees clean
after.

### THEN THE FULL SCAN
Every criterion and every \`kanban.md\` card at \`complete\`. No "complete *or* filed under
\`## Open blockers\`". Check the WORK, never the reports.

IF ANYTHING IS SHORT: **STOP.** No docs refresh, no PR change. Report what is short with the
command that shows it. Ten correct halts have come from exactly this discipline — do not
soften on the twelfth.

Receipt: \`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt12_cycle_receipt.md\`
Commit and push (§5).

### RETURN VALUE — ONLY JSON:
{"criterion":"AT-33-E6-001","attempt":12,"gate":"PASS"|"FAIL",
 "status":"complete"|"blocked-escalated",
 "f1_live":0,"f1_pinned":0,"assertion_repinned_not_weakened":true|false,
 "doc_comment_current":true|false,"work_inventory_not_regenerated":true|false,
 "lib_suite":{"passed":0,"failed":0},"no_run_exit":0,"desktop_suite":{"passed":0,"failed":0},
 "corpus_sweep":{"examined":0,"findings":0},
 "row_counts":{"fixture":0,"literal":0,"combined":0},
 "unexamined_set_empty":true|false,"oracle_disagreement":0,
 "inherited_failing":{"suites":0,"failures":0,"grew":true|false,"new_sd33_failures":[...]},
 "skinwalker_spot_checks":[{"record":"...","verified":true|false}],
 "undine_executed":true|false,"fixture_discipline_ok":true|false,
 "stale_prose_verified":true|false,
 "open_blockers_empty":true|false,"denominator_gate":"PASS"|"FAIL",
 "probe_left_no_residue":true|false,
 "shortfalls":[{"what":"...","command":"...","output":"..."}],
 "commands_rerun":[{"command":"...","output":"..."}],
 "receipt_path":"...","commit_shas":[...]}`
}

function refreshPrompt(scan) {
  return `${standingRules('sd33-fold-fix-docs')}

## YOUR TASK — re-derive SD-33's closure documents after the fold

The re-run scan PASSED on attempt 12:
${JSON.stringify(scan).slice(0, 4000)}

**Re-derive every headline figure rather than editing the old ones.** A number carried
forward from an earlier document is a recollection, not a measurement — and this bundle has
now been bitten by that exact thing twice.

1. **\`${PKG}/release-notes.md\`** — add what the fold delivered: **65 recovered Skinwalker
   race-trait corpus records** (10 → 75) and the Undine race-trait fixtures with their
   generator, each with counts and denominators. Say plainly that this work was **recovered
   from an SD-31 lane lost to an API error**, not newly built — that provenance is the
   interesting part. Note that the fold also **fixed two previously-failing suites**
   (\`ingest_races\`, \`sd27_alternate_racial_trait_reachability\`), taking the inherited
   failing set from 31/49 to 29/46. Re-derive every other figure in the file.

2. **\`${PKG}/progress.md\`** — a cycle entry for the fold and the re-closure, and the
   closed-state summary updated.

3. **\`${PKG}/forward-scope-register.md\`** — record the three branches ruled OUT, with
   reasons, so nobody rediscovers them as a surprise:
   - \`worktree-wf_a45ece26-3fc-1\` — 1,612 grant files, **superseded**: its \`class\` field
     holds feature-group names (\`"Deed"\`, \`"Bloodrager Bloodline"\`) where HEAD's 240 curated
     records hold real class names (\`"Fighter"\`), and it lacks \`granted_via_archetype\`,
     which \`class_feature_grant_consumer.rs\` treats as authoritative and defaults to \`true\`
     when absent — every folded record would have been silently mis-marked.
   - \`worktree-wf_13156488-c9b-1\` — wave 20, superseded by SD-32's
     \`untabled_base_class_feature_roster\`.
   - \`worktree-wf_c1156061-e3f-5\` — wave 30, 27 lines of notes on a closed bundle.
   **These are the operator's branches to delete. SD-34 must not re-litigate them.**

4. **\`docs/retro/sd33-computed-value-verification-retrospective.md\`** — append a fold
   section, and record these three lessons, each with what makes it fail:
   - **A stale branch's file count is not its value.** The 1,612-file grant branch looked
     like the biggest win available and was superseded; the 45-record rescue branch looked
     minor and was real. Only reading the records' schema against HEAD, and checking whether
     the live consumer required a field the branch lacked, told them apart.
   - **Run the suite after the last write that can move it.** The F1 assertion went red
     because a lane re-pinned a count correctly, then a later lane regenerated the inventory
     and never re-ran lib. Its receipt's "0 failed" was a true measurement of a tree it did
     not land.
   - **A gate's examined-population must grow when records are added.** \`corpus_literal_sweep\`
     going 48,634 → 48,699 (exactly +65) is what proved the new records were really inside the
     population rather than passing vacuously.
   Cite the retrospective from \`${PKG}/references/README.md\` if not already cited.

5. **PR #377** — update its body to reflect the fold. \`gh pr edit 377\`. **Do NOT merge.**

6. Re-run \`scripts/verify.sh --only denominator-gate\` on your own prose before pushing.

### PROCEDURE
§6, receipt at \`${PKG}/artifacts/epic-6-closure/fold-docs_cycle_receipt.md\`. Commit, push (§5).

### RETURN VALUE — ONLY JSON:
{"lane":"fold-docs","status":"complete",
 "release_notes_updated":true|false,
 "figures_rederived":[{"figure":"...","command":"...","output":"..."}],
 "progress_updated":true|false,"forward_scope_records_3_branches":true|false,
 "retro_appended":true|false,"retro_lessons":3,"retro_cited":true|false,
 "pr_body_updated":true|false,"pr_number":377,
 "denominator_gate":"PASS"|"FAIL",
 "commit_shas":[...],"receipt_path":"..."}`
}

log('SD-33 FOLD-FIX. Attempt 11 failed on ONE item: a stale F1 count assertion, caused by lib running before the final work-inventory write. Everything the fold delivered verified good — 65 new records, sweep grown to 48,699 with 0 findings, Epic 5 untouched.')

phase('Re-pin and re-green')
const fix = await agent(repinPrompt(), { model: 'sonnet', label: 'fold-fix-repin', phase: 'Re-pin and re-green' })

phase('Re-run closure')
const scan = await agent(rescanPrompt(fix), { model: 'opus', label: 'final-acceptance-scan-attempt12', phase: 'Re-run closure' })

const t = String(scan)
const failed = !scan || /"gate"\s*:\s*"FAIL"/.test(t) || /"status"\s*:\s*"blocked-escalated"/.test(t) || !/"gate"\s*:\s*"PASS"/.test(t)
if (failed) {
  log('AT-33-E6-001 attempt 12 did NOT pass. No docs refresh, no PR change. Correct outcome.')
  return { bundle: 'SD-33', fold: true, closed: false, halted_at: 'AT-33-E6-001 attempt 12', scan, fix }
}

const docs = await agent(refreshPrompt(scan), { model: 'sonnet', label: 'fold-docs', phase: 'Re-run closure' })
log('SD-33 fold complete and re-closed. PR #377 updated, NOT merged. SD-34 recut is next.')
return { bundle: 'SD-33', fold: true, closed: true, fix, scan, docs }
