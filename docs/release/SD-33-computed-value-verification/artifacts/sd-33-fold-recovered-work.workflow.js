export const meta = {
  name: 'sd-33-fold-recovered-work',
  description: 'SD-33 — fold recovered Skinwalker + Undine race-trait work, regenerate inventory, re-run closure',
  whenToUse: 'Operator ruling 2026-08-26: fold recovered work into SD-33 before PR #377 merges, then recut SD-34.',
  phases: [
    { title: 'Fold recovered work' },
    { title: 'Inventory and gates' },
    { title: 'Re-run closure' },
  ],
}

const REPO = '/home/ubuntu/workspace/repos/codex'
const PKG = 'docs/release/SD-33-computed-value-verification'
const BRANCH = 'tranche/13'

const RECEIPT_SCHEMA = `# Cycle <cycle-id> — <epic-name> / <criterion-id>

- **Commit SHA:** <sha>
- **Files touched:** <list>
- **Identifier audit result:** OK_NO_BUNDLE_TAGS / <violations>
- **Wired-integration audit result:** OK_NO_TOKENS / <violations>
- **Acceptance criterion:** <verbatim>
- **Figures + their re-derive commands:** <every number, with its command and denominator>
- **Row-count command output:** <literal output of the count on this cycle's own artifact>
- **Build scope verified:** <--no-run exit, workspace result, desktop crate result>
- **Status:** complete | blocked-escalated
- **Movement, four buckets:** closure / reclassification / reachability / instrument-correction
- **Notes:** <judgment calls>
- **Next-cycle plan:** <what the next cycle picks up>`

function standingRules(role) {
  return `You are a dispatched SD-33 FOLD agent. Role: ${role}.

## WHY THIS EXISTS — an operator ruling, 2026-08-26

SD-33 closed at 21 of 21 with PR #377 open and **not yet merged**. A sweep of stale local
branches found work that was generated during SD-31, never merged, and is **not superseded**.
The operator ruled: fold it into SD-33 before the PR merges, then recut SD-34 from the
corrected baseline.

**This reopens SD-33.** Adding work after closure means its closure evidence must be
re-derived — the final-acceptance scan runs again, and the release notes are re-derived. That
is the accepted cost of the ruling, not an oversight.

## WHAT WAS RULED IN, AND WHAT WAS RULED OUT — do not re-litigate

**IN — genuinely unique, schema-verified against HEAD:**
- **45 Skinwalker race-trait records** on \`sd31/racetrait4-SD31-E6-F4-005\`. Verified
  schema-identical to a current \`data/corpus/bestiary_5/race_trait/\` record: same top keys
  (\`completeness, data, ingested_at, license, pi_field, pi_marker, population, source,
  wiring_class, wiring_class_signals\`), same \`data\` keys, \`license: OGL\`, \`pi_field: null\`,
  \`raw_tokens\` populated. HEAD's 10 skinwalker files are **different records**
  (\`skinwalker_change_shape\`, \`skinwalker_type\`, ...); the branch's are the
  \`skinwalker_were*_kin\` variants. Zero overlap. 41 of the 50 skinwalker inventory units are
  currently open.
- **103 Undine race-trait fixture entries** on \`worktree-wf_be4660f2-72a-3\`, plus
  \`scripts/derive_race_trait_formula_fixtures.py\` which HEAD does not have. HEAD's
  \`tests/fixtures/rules_core/derived-evaluator-fixtures.json\` contains **zero** Undine entries.

**OUT — do not fold, do not touch these branches:**
- \`worktree-wf_a45ece26-3fc-1\` (1,612 grant files) — **SUPERSEDED.** Its \`class\` field holds
  feature-group names (\`"Deed"\`, \`"Bloodrager Bloodline"\`, \`"Arcanist Exploit"\`) where HEAD's
  240 curated records hold real class names (\`"Fighter"\`), and it lacks
  \`granted_via_archetype\`, which \`class_feature_grant_consumer.rs\` treats as authoritative and
  defaults to \`true\` when missing. Folding it would silently mis-mark every record.
- \`worktree-wf_13156488-c9b-1\` (wave 20) — superseded in approach by SD-32's
  \`untabled_base_class_feature_roster\`.
- \`worktree-wf_c1156061-e3f-5\` (wave 30) — 27 lines of notes on a closed bundle.

## REQUIRED READS, in order
1. ${REPO}/CLAUDE.md
2. ${REPO}/AGENTS.md
3. ${REPO}/${PKG}/workflow-instruction.md §5, §6, §7
Keep context lean.

## ENVIRONMENT
  export RETRO_ACTOR="${role}"
  export CARGO_TARGET_DIR="/tmp/cargo-sd33-${role}"
  export CARGO_INCREMENTAL=0
  mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"

## ONE TURN ONLY
Nothing wakes you. Foreground slow work or poll a background job inside this turn. If
something will not finish, report what you observed and COMMIT AND PUSH ANYWAY.

## GIT DISCIPLINE (§5)
- \`git status --porcelain\` before EVERY git write. Inspect it.
- NEVER \`git add -A\`. NEVER \`git stash\`. NEVER force-push.
- Push: \`git fetch origin ${BRANCH} && git rebase origin/${BRANCH} && git push origin HEAD:${BRANCH}\`
  Retry up to 5 times on non-fast-forward.
- **Do NOT delete, merge, or rebase any of the five stale local branches.** They are the
  operator's to dispose of. Read from them with \`git show <branch>:<path>\` only.

## CORPUS DISCIPLINE — load-bearing for this wave
- **\`data/corpus/**\` is NEVER hand-edited.** Guarded generator path only. \`src/bin/\` carries
  \`ingest_race_traits.rs\`, \`ingest_races.rs\`, \`ingest_apg_race_traits.rs\`.
- **NEVER \`--allow-stamp-loss\`.**
- Regeneration can destroy **license / PI metadata and \`raw_tokens\`** — verify per record that
  they survive. A dropped \`pi_field\` or license stamp is a distribution problem, not a test
  failure.
- **A record-count change compiles clean while leaving other files' hardcoded assertions red.**
  Grep the OLD and NEW counts across \`tests/\`, \`src/\`, \`apps/\`, \`scripts/\`.
- A shallow glob lies here — use recursive search and state the search used.
- Run \`cargo run --locked --bin corpus_literal_sweep\` after any corpus change; it must report
  **0 findings**.

## FIGURES
Every number states its denominator in the same construct and carries its re-derive command.
\`scripts/verify.sh --only denominator-gate\` scans package markdown and has caught six agents.`
}

// ---------------------------------------------------------------------------
// Lane A — Skinwalker records
// ---------------------------------------------------------------------------

function skinwalkerPrompt() {
  return `${standingRules('sd33-fold-skinwalker')}

## YOUR TASK — fold the 45 recovered Skinwalker race-trait records

Source branch: \`sd31/racetrait4-SD31-E6-F4-005\` (read-only; \`git show <branch>:<path>\`).
Its two commits say what it is: *"PRESERVE 48 generated Skinwalker race_trait records from the
same lost lane"* and *"PRESERVE uncommitted race-chassis work from the wave-11 lane lost to an
API error"*. It is a rescue branch. **The operator's standing note is that it must not be
merged on trust** — hence this lane rather than a \`git merge\`.

### STEP 1 — enumerate, do not assume
List every file the branch adds that HEAD lacks:

    git diff --name-status origin/develop..sd31/racetrait4-SD31-E6-F4-005 | grep -v '^D'

The count I measured was 45 skinwalker \`*_kin\` records, and the commit message says 48. **Both
numbers may be wrong.** Re-derive the real set and state it with its command. If the branch
carries non-skinwalker files too, enumerate those separately and report them — do not silently
fold or silently skip them.

### STEP 2 — regenerate, do not copy
**The records must come from the guarded generator path, not from \`git show\`.** Establish
whether \`src/bin/ingest_race_traits.rs\` (or a sibling) can produce them from the pinned
oracle source at \`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6\`.

- **If the generator produces them:** regenerate, then diff your output against the branch's
  records. Differences are findings — report them per field. The generator's output wins.
- **If the generator cannot produce them**, say exactly why (missing source, unmodelled race,
  a gap in the ingest binary) and **fix the generator** with RED→GREEN so it can. That fix is
  the real deliverable; the records fall out of it.
- **Only if fixing the generator is genuinely out of reach** may you fall back to committing
  the branch's records directly — and then you must state that plainly, verify every record's
  \`license\`, \`pi_field\`, \`pi_marker\` and \`raw_tokens\` field-by-field against a current HEAD
  record, and record it as a named exception in the receipt. **Do not take this route to save
  time.**

### STEP 3 — verify the records are real
For a sample of at least 5, trace the record back to the pinned oracle \`.lst\` source **by
hand**, token by token, and put that derivation in the receipt. These records were generated by
a lane that then died; nothing has ever verified them.

### STEP 4 — count sweep
Corpus record counts move. Grep OLD and NEW counts across \`tests/\`, \`src/\`, \`apps/\`,
\`scripts/\` and fix every stale assertion. Report what you found.

### STEP 5 — gates
    cargo run --locked --bin corpus_literal_sweep      # 0 findings, exit 0
    cargo test --locked --no-run                        # exit 0
    cargo test --locked --lib
    cd apps/desktop/src-tauri && cargo test --locked
State each result. **Do NOT regenerate \`docs/work-inventory.json\`** — a later lane owns it.

### COORDINATION
One sibling lane is folding the Undine fixtures right now. Your files are
\`data/corpus/bestiary_5/race_trait/**\` and the ingest binaries; theirs are the fixture JSON
and \`scripts/derive_race_trait_formula_fixtures.py\`. Re-read before every shared edit.

### PROCEDURE
§6, all nine steps, §7 schema with \`- **Status:** <x>\` as a BULLET.
Receipt: \`${PKG}/artifacts/epic-6-closure/fold-skinwalker_cycle_receipt.md\`

### RETURN VALUE — ONLY JSON:
{"lane":"fold-skinwalker","status":"complete"|"blocked-escalated",
 "branch_files_enumerated":0,"skinwalker_records":0,"other_files_on_branch":[...],
 "route":"generator-produced"|"generator-fixed"|"direct-commit-exception",
 "generator_fix":"...","records_committed":0,
 "hand_traced_sample":[{"record":"...","lst_source":"...","tokens_verified":0}],
 "license_pi_verified":true|false,"raw_tokens_verified":true|false,
 "counts_moved":true|false,"count_sweep_result":"...",
 "corpus_sweep":"...","no_run_exit":0,"lib_suite":"...","desktop_suite":"...",
 "row_count_command_output":"...",
 "commit_shas":[...],"receipt_path":"...","red_green":"...","next":"..."}`
}

// ---------------------------------------------------------------------------
// Lane B — Undine fixtures
// ---------------------------------------------------------------------------

function undinePrompt() {
  return `${standingRules('sd33-fold-undine')}

## YOUR TASK — fold the 103 recovered Undine race-trait fixture entries

Source branch: \`worktree-wf_be4660f2-72a-3\` (read-only). Its commit:
*"W26-racetrait — Undine formula race-trait seam through the interpreter (+14 board)"*, plus a
retro correction commit noting that the wave's own \`modelled_race_of_race_trait\` claim was
stale.

**Measured:** \`tests/fixtures/rules_core/derived-evaluator-fixtures.json\` has **0** Undine
entries on HEAD and **103** on the branch. \`scripts/derive_race_trait_formula_fixtures.py\`
exists on the branch and **not** on HEAD. \`src/rules_core/derived_evaluator_fixture_check.rs\`
exists on both.

### STEP 1 — re-derive the numbers
Confirm 0-on-HEAD and 103-on-branch yourself, with the command. Then diff
\`derived_evaluator_fixture_check.rs\` between HEAD and the branch — HEAD's version may have
moved on since, and the branch's changes may already be present or may conflict.

### STEP 2 — the generator is the deliverable, not the fixtures
\`scripts/derive_race_trait_formula_fixtures.py\` is what *produces* the 103 entries. Fold the
**script**, run it, and let the fixtures fall out of it. A fixture file committed without the
generator that made it is exactly the shape that rots.

**Fixture discipline (binding):** a fixture's expected value must be transcribed from bytes the
engine's own read path does **not** touch. A fixture built from the same file the engine reads
is a mirror, not a check — it will happily validate a fabricated value. If the recovered
generator violates this, **fix it**, and say so.

### STEP 3 — the fixtures must actually run
Run the fixture check and show the Undine entries being exercised:

    cargo test --locked --lib derived_evaluator_fixture

A fixture that is committed but not executed by any test is decoration. Confirm the count of
Undine cases the suite actually runs, with its denominator.

### STEP 4 — the stale claim
The branch's own second commit is a retro correction: the wave's
\`modelled_race_of_race_trait\` claim was stale. **Read that correction and confirm whether the
claim is still stale against today's code.** If it is, that is a finding for the receipt, not
something to carry forward silently.

### STEP 5 — gates
    cargo test --locked --no-run        # exit 0
    cargo test --locked --lib
    cd apps/desktop/src-tauri && cargo test --locked
State each. **Do NOT regenerate \`docs/work-inventory.json\`** — a later lane owns it.
**Do NOT touch \`data/corpus/**\`** — the sibling lane owns the corpus this wave.

### COORDINATION
One sibling lane is folding the Skinwalker corpus records right now. Your files are
\`tests/fixtures/rules_core/derived-evaluator-fixtures.json\`,
\`scripts/derive_race_trait_formula_fixtures.py\`, and
\`src/rules_core/derived_evaluator_fixture_check.rs\`. Re-read before every shared edit.

### PROCEDURE
§6, all nine steps, §7 schema with \`- **Status:** <x>\` as a BULLET.
Receipt: \`${PKG}/artifacts/epic-6-closure/fold-undine_cycle_receipt.md\`

### RETURN VALUE — ONLY JSON:
{"lane":"fold-undine","status":"complete"|"blocked-escalated",
 "undine_on_head_before":0,"undine_on_branch":0,"undine_committed":0,
 "generator_folded":true|false,"generator_path":"...",
 "fixtures_regenerated_from_generator":true|false,
 "fixture_discipline_ok":true|false,"fixture_discipline_fix":"...",
 "undine_cases_actually_run":0,"denominator":"...",
 "stale_claim_still_stale":true|false,"stale_claim_finding":"...",
 "fixture_check_diff_vs_head":"...",
 "no_run_exit":0,"lib_suite":"...","desktop_suite":"...",
 "row_count_command_output":"...",
 "commit_shas":[...],"receipt_path":"...","red_green":"...","next":"..."}`
}

// ---------------------------------------------------------------------------
// Lane C — inventory regeneration + gates
// ---------------------------------------------------------------------------

function inventoryPrompt(fold) {
  return `${standingRules('sd33-fold-inventory')}

## YOUR TASK — regenerate the work inventory and re-green every gate

Two lanes just folded recovered work. Their reports (**reports, not evidence** — verify
against the repo):
${JSON.stringify(fold).slice(0, 5000)}

### STEP 1 — verify what actually landed
\`git log\` and the target files. Count the new corpus records and fixture entries yourself.
If a lane reported \`complete\` over a partial result, say so.

### STEP 2 — regenerate \`docs/work-inventory.json\`
Through its own binary (\`src/bin/v06_work_inventory.rs\`), never by hand.

**Report the movement precisely**, in four buckets — closure / reclassification / reachability
/ instrument-correction — with before and after counts and their denominators:
- total units (was **49,438**)
- \`not-ingested\` (was **26,047**)
- skinwalker \`race_trait\` units and their statuses (was 50 units: 41 not-ingested, 5
  ingested-magnitude, 3 text-complete, 1 grounded)

**A unit count that changes is not automatically closure.** If units moved because the
inventory now sees records it could not see before, that is real. If a count moved because a
classifier changed, that is instrument-correction. Say which.

### STEP 3 — the count sweep
New corpus records change counts. Grep OLD and NEW numbers across \`tests/\`, \`src/\`, \`apps/\`,
\`scripts/\` and fix every stale assertion. This is the failure that compiles clean.

### STEP 4 — every gate green
    python3 scripts/box_ledger.py --check
    python3 scripts/box_ledger.py --check --oracle-results ${PKG}/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json
    scripts/verify.sh --only denominator-gate
    scripts/verify.sh --only corpus-sweep
    cargo run --locked --bin corpus_literal_sweep
    cargo test --locked --no-run ; cargo test --locked --lib
    cd apps/desktop/src-tauri && cargo test --locked

**The oracle-results check is the one to watch.** SD-33 closed at
\`oracle_disagreement=0\` over an 8,330-unit population and \`unexamined_set_empty: true\`. If
the inventory now holds units that were not in that population, **the set is no longer empty**
and SD-33's Epic 5 evidence no longer covers the book. Derive the unexamined SET, not its size:

    python3 -c "import json
wi=json.load(open('docs/work-inventory.json'))['units']
pop={u['id'] for u in wi if u.get('status') in ('literal-verified','fixture-verified')}
d=json.load(open('${PKG}/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json'))['results']
miss=sorted(pop-{r['unit_id'] for r in d}); print(len(miss)); [print(m) for m in miss[:50]]"

**If that set is non-empty, run the new units through the oracle harness**
(\`scripts/oracle_harness/\`) and merge their rows, so SD-33's own closure evidence still holds
over its stated population. Amortise the JVM: one character carrying many computed variables.
Measure per-unit cost on a sample and project before the full run.

If the set is non-empty and you genuinely cannot close it in this turn, report the exact
remainder with its command and return \`blocked-escalated\`. **Do not narrow the population to
make the check pass.**

### PROCEDURE
§6, all nine steps, §7 schema.
Receipt: \`${PKG}/artifacts/epic-6-closure/fold-inventory_cycle_receipt.md\`

### RETURN VALUE — ONLY JSON:
{"lane":"fold-inventory","status":"complete"|"blocked-escalated",
 "landed_verified":{"skinwalker_records":0,"undine_entries":0,"discrepancies_vs_lane_reports":[...]},
 "inventory":{"units_before":49438,"units_after":0,"not_ingested_before":26047,"not_ingested_after":0},
 "skinwalker_units":{"before":50,"after":0,"statuses_after":{}},
 "movement_four_buckets":{"closure":0,"reclassification":0,"reachability":0,"instrument_correction":0},
 "counts_moved":true|false,"count_sweep_result":"...",
 "unexamined_set_size":0,"unexamined_sample":[...],
 "new_units_run_through_oracle":0,"oracle_disagreement_after":0,
 "gates":{"box_ledger":"...","denominator_gate":"...","corpus_sweep":"...","no_run_exit":0,"lib":"...","desktop":"..."},
 "commit_shas":[...],"receipt_path":"...","next":"..."}`
}

// ---------------------------------------------------------------------------
// Lane D — re-run the closure scan
// ---------------------------------------------------------------------------

function rescanPrompt(summary) {
  return `${standingRules('sd33-fold-rescan')}

## YOUR TASK — AT-33-E6-001, attempt 11: re-run the final-acceptance scan after the fold

SD-33 passed its scan on attempt 10 and closed at 21 of 21. **Work has since been folded in
under an operator ruling**, so that PASS no longer covers the current tree. Read attempt 10's
receipt first:
\`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt10_cycle_receipt.md\`

Fold reports (**reports, not evidence**):
${JSON.stringify(summary).slice(0, 6000)}

### WHAT ATTEMPT 10 ESTABLISHED — re-verify, do not re-investigate
- rows 1,741 / 6,589 / 8,330; unexamined set empty both directions; **0 disagree of 8,330**
- 0 reasonless \`unverifiable\` of 7,519; 0 duplicate unit_ids
- \`box_ledger.py --check\` exit 0; work-inventory \`unknown\` 0 of 49,438
- \`## Open blockers\` empty; open deferrals 3, none deferring live DoD scope
- denominator gate 0 violations; corpus sweep 0 findings of 48,634 records
- lib 2,837 of 2,837; desktop 548 of 548; \`--no-run\` 543 of 543 targets
- inherited debt: 31 of 599 suites / 49 of 8,026 tests, 0 of 31 with a commit since the cut

### WHAT THE FOLD PUTS AT RISK — check each specifically
1. **The Epic 5 population changed.** New corpus records mean new inventory units. Derive the
   unexamined SET (not its size) against
   \`AT-33-E5-003.combined-oracle-results.json\`. **It must be empty.** If the inventory lane
   ran new units through the oracle, verify those rows exist, carry real
   \`(ours, oracle, verdict)\` values, and that no \`disagree\` was left unresolved.
2. **Counts moved.** Verify the sweep across \`tests/\`, \`src/\`, \`apps/\`, \`scripts/\` actually
   ran and left zero stale live assertions.
3. **Corpus integrity.** \`corpus_literal_sweep\` must still report **0 findings**, and the
   population it examined must have GROWN (it was 48,634 records). A sweep that stayed at
   exactly 48,634 while records were added means the new records are not being examined —
   that is a vacuous pass and is BLOCKING.
4. **License / PI survived** on every new corpus record. Spot-check by field.
5. **The Skinwalker records are real.** The lane was asked to hand-trace at least 5 back to the
   pinned oracle \`.lst\`. Verify that derivation yourself against the source for at least 2.
   These records were generated by a lane that died; nothing had ever verified them.
6. **The Undine fixtures actually execute.** A committed fixture no test runs is decoration.
   Confirm the count of Undine cases the suite runs, with its denominator.
7. **Fixture discipline.** Confirm the Undine fixtures' expected values are not transcribed
   from the same file the engine reads. A mirror validates fabricated values.

### THEN THE FULL SCAN
Every criterion and every \`kanban.md\` card at \`complete\`. No "complete *or* filed under
\`## Open blockers\`". Check the WORK, never the reports: count rows, derive sets, re-run
headline commands, read commit diffs, re-derive attribution from \`git\`, grep instruments for
hardcoded exclusion lists, verify at the widest build scope with targets **executed** counted.

**Re-prove the gates still fail.** Plant a genuine violation, confirm the catch, remove the
probe, confirm the baseline returns to zero. Leave no residue.

IF ANYTHING IS SHORT: **STOP.** No release-notes update, no PR change. Report what is short
with the command that shows it. Ten scans have run on this bundle and nine correct halts came
from exactly this discipline — **do not soften on the eleventh because the bundle was already
closed once.**

Receipt: \`${PKG}/artifacts/epic-6-closure/AT-33-E6-001-attempt11_cycle_receipt.md\`
Commit and push (§5).

### RETURN VALUE — ONLY JSON:
{"criterion":"AT-33-E6-001","attempt":11,"gate":"PASS"|"FAIL",
 "status":"complete"|"blocked-escalated",
 "row_counts":{"fixture":0,"literal":0,"combined":0},
 "unexamined_set_empty":true|false,"missing_unit_ids":[...],
 "oracle_disagreement":0,
 "corpus_sweep":{"records_examined":0,"grew_from_48634":true|false,"findings":0},
 "license_pi_intact":true|false,
 "skinwalker_hand_traced_by_scan":[{"record":"...","verified":true|false}],
 "undine_cases_executed":0,"fixture_discipline_ok":true|false,
 "counts_swept":true|false,
 "lib_suite":{"passed":0,"failed":0},"desktop_suite":{"passed":0,"failed":0},"no_run_exit":0,
 "preexisting_failing_suites":{"count":0,"failures":0,"grew":true|false},
 "open_blockers_empty":true|false,"denominator_gate":"PASS"|"FAIL",
 "shortfalls":[{"what":"...","command":"...","output":"..."}],
 "commands_rerun":[{"command":"...","output":"..."}],
 "receipt_path":"...","commit_shas":[...]}`
}

// ---------------------------------------------------------------------------
// Lane E — refresh closure documents
// ---------------------------------------------------------------------------

function refreshPrompt(scan) {
  return `${standingRules('sd33-fold-docs')}

## YOUR TASK — re-derive SD-33's closure documents after the fold

The re-run scan PASSED:
${JSON.stringify(scan).slice(0, 4000)}

SD-33's closure documents were written before the fold and now state figures that have moved.
**Re-derive every headline figure rather than editing the old ones** — a number carried
forward from an earlier document is a recollection, not a measurement.

1. **\`${PKG}/release-notes.md\`** — update for what the fold added: the recovered Skinwalker
   records and Undine fixtures, with their counts and denominators, and any inventory movement
   in four buckets. Say plainly that the work was **recovered from an SD-31 lane lost to an API
   error**, not newly built — that provenance is the interesting part and a reviewer should not
   have to infer it. Re-derive every other figure in the file; do not assume the pre-fold ones
   still hold.

2. **\`${PKG}/progress.md\`** — a cycle entry for the fold, and the closed-state summary updated.

3. **\`${PKG}/forward-scope-register.md\`** — record the three branches ruled OUT, each with the
   reason, so nobody re-discovers them as a surprise:
   - \`worktree-wf_a45ece26-3fc-1\` — 1,612 grant files, **superseded**: \`class\` holds
     feature-group names where HEAD holds real class names, and \`granted_via_archetype\` is
     absent where the consumer treats it as authoritative and defaults it to \`true\`.
   - \`worktree-wf_13156488-c9b-1\` — wave 20, superseded by SD-32's
     \`untabled_base_class_feature_roster\`.
   - \`worktree-wf_c1156061-e3f-5\` — wave 30, 27 lines of notes on a closed bundle.
   **These are the operator's branches to delete; SD-34 must not re-litigate them.**

4. **\`docs/retro/sd33-computed-value-verification-retrospective.md\`** — append a section on the
   fold. The lesson worth recording: **a stale branch's file count is not its value.** The
   1,612-file grant branch looked like the biggest win available and was superseded; the
   45-record rescue branch looked minor and was real. The difference was only visible by
   reading the records' schema against HEAD, not by counting them.
   Cite it from \`${PKG}/references/README.md\` if the citation is not already there.

5. **PR #377** — update its body to reflect the fold. \`gh pr edit 377\`. **Do NOT merge.**

6. Re-run \`scripts/verify.sh --only denominator-gate\` on your own prose before pushing; it
   scans these files and has caught six agents.

### PROCEDURE
§6, receipt at \`${PKG}/artifacts/epic-6-closure/fold-docs_cycle_receipt.md\`. Commit, push (§5).

### RETURN VALUE — ONLY JSON:
{"lane":"fold-docs","status":"complete",
 "release_notes_updated":true|false,"figures_rederived":[{"figure":"...","command":"...","output":"..."}],
 "progress_updated":true|false,"forward_scope_records_3_branches":true|false,
 "retro_appended":true|false,"retro_cited":true|false,
 "pr_body_updated":true|false,"pr_number":377,
 "denominator_gate":"PASS"|"FAIL",
 "commit_shas":[...],"receipt_path":"..."}`
}

// ---------------------------------------------------------------------------
// Dispatch
// ---------------------------------------------------------------------------

log('SD-33 FOLD. Operator ruling: fold recovered Skinwalker (45 records) + Undine (103 fixtures) work into SD-33 before PR #377 merges. Grants/wave-20/wave-30 branches ruled OUT as superseded. This reopens SD-33 — its closure evidence is re-derived.')

phase('Fold recovered work')
const [skinwalker, undine] = await parallel([
  () => agent(skinwalkerPrompt(), { model: 'sonnet', label: 'fold-skinwalker', phase: 'Fold recovered work', isolation: 'worktree' }),
  () => agent(undinePrompt(), { model: 'sonnet', label: 'fold-undine', phase: 'Fold recovered work', isolation: 'worktree' }),
])
log('Fold lanes returned. Check `df -h /` and `git worktree list`.')

phase('Inventory and gates')
const inventory = await agent(inventoryPrompt({ skinwalker, undine }), {
  model: 'sonnet', label: 'fold-inventory', phase: 'Inventory and gates',
})

const summary = { skinwalker, undine, inventory }

phase('Re-run closure')
const scan = await agent(rescanPrompt(summary), {
  model: 'opus', label: 'final-acceptance-scan-attempt11', phase: 'Re-run closure',
})

const scanText = String(scan)
const scanFailed = !scan
  || /"gate"\s*:\s*"FAIL"/.test(scanText)
  || /"status"\s*:\s*"blocked-escalated"/.test(scanText)
  || !/"gate"\s*:\s*"PASS"/.test(scanText)

if (scanFailed) {
  log('AT-33-E6-001 attempt 11 did NOT pass. No release-notes update, no PR change. Correct outcome, not a failure.')
  return { bundle: 'SD-33', fold: true, closed: false, halted_at: 'AT-33-E6-001 attempt 11', scan, summary }
}

const docs = await agent(refreshPrompt(scan), { model: 'sonnet', label: 'fold-docs', phase: 'Re-run closure' })

log('SD-33 fold complete and re-closed. PR #377 updated, NOT merged. SD-34 is recut next from the new inventory.')

return { bundle: 'SD-33', fold: true, closed: true, summary, scan, docs }
