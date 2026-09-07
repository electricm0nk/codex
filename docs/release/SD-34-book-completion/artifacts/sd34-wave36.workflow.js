export const meta = {
  name: 'sd34-wave36',
  description: 'SD-34 wave 36 -- matcher bug fix (19 units), zero-magnitude cross-reference (25 units), disposition trace on companion/subdomain shapes (80 units)',
  whenToUse: 'Wave 35 closed 40/40 with zero unit closures but a full reconnaissance of the two big untouched bucket-D shapes (931+179). This wave dispatches lane C wave-35 own cheapest-first items 1-3.',
  phases: [
    { title: 'Fix (3 parallel, isolated worktrees, capped at 3 -- the documented safe ceiling)' },
    { title: 'Sequential merge onto tranche/14' },
    { title: 'Final verify and wave closure' },
  ],
}

const REPO = '/home/ubuntu/workspace/repos/codex'
const PKG = 'docs/release/SD-34-book-completion'

const SAFETY = 'CRITICAL SAFETY RULE: on 2026-09-02 this box crashed (kernel soft-lockup) ' +
  'when a wave dispatched FOUR concurrent cargo-building lanes, one past the three-lane ' +
  'ceiling .cargo/config.toml own jobs=6 fix was calibrated for (see ' +
  '~/.claude/projects/-home-ubuntu-workspace-repos-codex/memory/proxmox-host-stops-vm-on-guest-oom.md). ' +
  'This wave dispatches exactly three lanes in the Fix phase -- do not exceed jobs=6, do not ' +
  'run a second cargo process inside your own agent while the first is still building.'

function standingRules(role) {
  return 'You are a dispatched SD-34 wave-36 agent, role: ' + role + '. Work in your own ' +
    'isolated git worktree (the harness gives you one via isolation:worktree -- your cwd IS ' +
    'that worktree, already on a fresh branch off tranche/14 tip).\n\n' +
    SAFETY + '\n\n' +
    'Required reads first, keep context lean after: ' + REPO + '/CLAUDE.md, ' + REPO + '/AGENTS.md, ' +
    REPO + '/' + PKG + '/workflow-instruction.md (receipt schema, prepend-only progress.md ' +
    'convention), ' + REPO + '/' + PKG + '/artifacts/bucket-d-mining/' +
    'wave35_laneC_reconnaissance_cycle_receipt.md (your full technical brief lives here -- ' +
    'read the section named in your assignment below in full before starting).\n\n' +
    'ENVIRONMENT: export RETRO_ACTOR="sd34-wave36-' + role + '"; export CARGO_BUILD_JOBS=6; ' +
    'export CARGO_TARGET_DIR="/tmp/cargo-sd34-wave36-' + role + '".\n\n' +
    'Do the real work (do not stub, do not fixture-only it -- docs/governance/no-stub-mvp-doctrine.md). ' +
    'Be HONEST about the real outcome, per wave 35 own precedent: if a unit does not actually ' +
    'move to DONE (magnitude genuinely required but this engine has no mechanism for it, or a ' +
    'similar real limit), report that plainly rather than claiming a false closure -- an ' +
    'instrument-correction (more precise evidence) is valuable and welcome, but it is not the ' +
    'same as closure, and your receipt must say which one actually happened, with the real ' +
    'unit-closed count. Run ONLY the tests scoped to what you touched, not the full workspace. ' +
    'Run python3 scripts/completion_atlas.py --check after your change and confirm ' +
    'population=49438 overlap=0 unclassified=0 -- if nonzero, you introduced a real defect, ' +
    'fix it before finishing. Author a full receipt under artifacts/bucket-d-mining/ (same ' +
    'shape as prior waves -- read one for house style) and prepend a progress.md entry. Commit ' +
    'in your own worktree -- do NOT try to merge onto tranche/14 yourself.\n\n' +
    'Return a structured summary: your final local commit SHA, files touched, whether your ' +
    'scoped tests passed, the completion_atlas.py --check output, and how many units ACTUALLY ' +
    'closed to DONE (be honest -- 0 is a valid, reportable answer if that is the truth).'
}

async function fixLane({ role, assignment, schema }) {
  return agent(standingRules(role) + '\n\n## YOUR ASSIGNMENT\n\n' + assignment, {
    phase: 'Fix (3 parallel, isolated worktrees, capped at 3 -- the documented safe ceiling)',
    label: 'wave36-' + role,
    effort: 'high',
    isolation: 'worktree',
    schema: schema || {
      type: 'object',
      properties: {
        commit_sha: { type: 'string' },
        files_touched: { type: 'array', items: { type: 'string' } },
        tests_passed: { type: 'boolean' },
        completion_atlas_check: { type: 'string' },
        units_closed: { type: 'number' },
        notes: { type: 'string' },
      },
      required: ['commit_sha', 'tests_passed', 'completion_atlas_check', 'units_closed'],
    },
  })
}

const laneAAssignment = 'Close wave 35 lane C own named Sub-mechanism 1 (19 units: psychic_warrior ' +
  '18 + rogue 1) from the "class_feature_of_unmodelled_corpus_class" shape -- read that section ' +
  'of the reconnaissance receipt in full (search for "Sub-mechanism 1" in ' +
  'wave35_laneC_reconnaissance_cycle_receipt.md).\n\n' +
  'This is a matcher bug fix, NOT a new-chassis problem -- both classes already have a real, ' +
  'working chassis this engine computes. The brief names the exact fix: (a) in ' +
  'modelled_class_books() (src/bin/v06_work_inventory.rs, around :13910-13990, re-derive the ' +
  'exact current line with grep since prior waves shift it), space-join the untabled-registry ' +
  'loop bare_name the same way the CRB-prestige loop already does (the function own doc comment ' +
  'states the correct convention three lines below the bug); (b) add a facts.class_books ' +
  'membership check to the final corpus_class_names fallback branch before it emits ' +
  'class_feature_of_unmodelled_corpus_class. Write the RED test FIRST (asserting psychic_warrior ' +
  'own multi-word key resolves), confirm it fails for the stated reason, then fix both call ' +
  'sites and confirm GREEN. The brief flags this bug may also affect classes outside this ' +
  '931-unit shape (e.g. Kind::Class-level records) -- check that too and report what you find, ' +
  'even if out of this cycle own scope to fix.'

const laneBAssignment = 'Run wave 35 lane C own named cheapest-first check on Shape 2 ' +
  '("class_feature_no_dedicated_magnitude_id_matched_the_record_slug", 179 units): the 25-unit ' +
  'zero-magnitude sub-split -- read that section in full (search "Shape 2" and "cheap sub-split" ' +
  'in wave35_laneC_reconnaissance_cycle_receipt.md).\n\n' +
  '17 of the 25 already carry wiring_class == "display" (the promotion gate own requirement). ' +
  'Cross-reference those 17 against real corpus DESC: tokens and the universal_sheet_modifier ' +
  'gate -- the EXACT method wave 32 own receipt already proved out on a different, larger ' +
  '1,727-unit shape (search progress.md for "Wave 32" and completion_atlas.py / ' +
  'v06_work_inventory.rs for universal_sheet_modifier / has_real_description / ' +
  'is_display_wiring_class_for_promotion to find that precedent code). This could promote a ' +
  'subset to text-complete with zero magnitude-id work -- or it could find zero promotable, the ' +
  'same as wave 32 own result on the larger shape. Either honest outcome is a valid result; ' +
  'report exactly what you find, with the real re-derived count. Do not touch the remaining 154 ' +
  'magnitude-bearing units in Shape 2 -- those are named as separate, larger Epic 3 scope, out ' +
  'of this cycle.'

const laneCAssignment = 'DISPOSITION TRACE (investigation, real code change only if the trace ' +
  'clearly resolves without ambiguity) on wave 35 lane C own named Sub-mechanisms 2-4 (80 units: ' +
  '63 creature-type collisions + 16 eidolon + 1 sentinel single-unit collision) from the ' +
  '"class_feature_of_unmodelled_corpus_class" shape -- read that section in full (search ' +
  '"Sub-mechanism 2", "Sub-mechanism 3", "Sub-mechanism 4" in ' +
  'wave35_laneC_reconnaissance_cycle_receipt.md).\n\n' +
  'The brief flags these as "likely a decisions.md-shaped ruling... not a chassis question at ' +
  'all" but does NOT yet trace each named group to ground truth. For each of animal/undead/' +
  'dragon/construct/plant/ooze (63 units) and eidolon (16 units) and the sentinel single-unit ' +
  'collision (1 unit): determine whether the feature is (a) already computed elsewhere under a ' +
  'DIFFERENT unit id this shape double-counts (a real matcher bug you should fix, same rigor as ' +
  'lane A own sub-mechanism 1), (b) a genuinely-needed new companion/subdomain-table mechanism ' +
  '(name it precisely, do not build it this cycle -- that is real new-mechanism scope needing ' +
  'its own dispatch), or (c) something else you find that the brief did not anticipate. Fix ONLY ' +
  'case (a) findings (matcher bugs) if you find any -- do not attempt (b) work this cycle. ' +
  'Author your findings as a proper disposition table (same rigor as the reconnaissance receipt ' +
  'you are extending), naming an exact population and next-step for every one of the 80 units, ' +
  'none left as "the rest".'

log('Wave 36: dispatching 3 lanes in parallel, isolated worktrees, capped at the documented safe ceiling of 3 concurrent cargo-building lanes')
const [laneA, laneB, laneC] = await parallel([
  () => fixLane({ role: 'lanea', assignment: laneAAssignment }),
  () => fixLane({ role: 'laneb', assignment: laneBAssignment }),
  () => fixLane({ role: 'lanec', assignment: laneCAssignment }),
])

log('Fix phase done. Lane A: ' + (laneA?.commit_sha ?? 'FAILED') + ', Lane B: ' + (laneB?.commit_sha ?? 'FAILED') + ', Lane C: ' + (laneC?.commit_sha ?? 'FAILED') + '. Merging sequentially onto tranche/14.')

const mergePrompt = 'You are the SD-34 wave-36 sequential-merge agent. Three lanes just ' +
  'finished independent work, each in its own isolated worktree, each committed locally but ' +
  'NOT yet merged onto ' + REPO + ' tranche/14:\n' +
  '- Lane A (matcher bug fix, up to 19 units): ' + JSON.stringify(laneA) + '\n' +
  '- Lane B (zero-magnitude cross-reference, up to 25 units): ' + JSON.stringify(laneB) + '\n' +
  '- Lane C (disposition trace on 80 units, possible bug fixes): ' + JSON.stringify(laneC) + '\n\n' +
  SAFETY + ' You are the only process permitted to run cargo/npm right now -- merge one lane ' +
  'at a time, testing after each before moving to the next.\n\n' +
  'Read ' + REPO + '/CLAUDE.md and ' + REPO + '/AGENTS.md first.\n\n' +
  'For each lane that succeeded (commit_sha present, tests_passed true), in order A then B ' +
  'then C: rebase that lane worktree branch onto the CURRENT tranche/14 tip (which moves ' +
  'after each successful merge -- lane A and lane C both may touch v06_work_inventory.rs own ' +
  'matcher functions, so a real rebase conflict there is plausible, not just a generated-JSON ' +
  'shift). Resolve any conflicts by keeping both sides substantive changes where they target ' +
  'genuinely different code paths, and for any shared generated JSON (docs/work-inventory.json, ' +
  'docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json), never ' +
  'hand-merge the numbers -- regenerate/re-run python3 scripts/completion_atlas.py --check ' +
  'and trust its live output. If lane A and lane C both touched the SAME function in a way that ' +
  'does not cleanly compose, stop and report the conflict clearly rather than guessing at a ' +
  'resolution. Run that lane own scoped tests one more time post-rebase before merging ' +
  '(git -C ' + REPO + ' merge --ff-only <branch>). If a lane failed, skip it and report why.\n\n' +
  'Return a structured summary: which lanes merged cleanly, which were skipped and why, and ' +
  'the final tranche/14 HEAD sha.'

const merged = await agent(mergePrompt, {
  phase: 'Sequential merge onto tranche/14',
  label: 'wave36-merge',
  effort: 'high',
  schema: {
    type: 'object',
    properties: {
      lanes_merged: { type: 'array', items: { type: 'string' } },
      lanes_skipped: { type: 'array', items: { type: 'string' } },
      final_head_sha: { type: 'string' },
      notes: { type: 'string' },
    },
    required: ['lanes_merged', 'lanes_skipped', 'final_head_sha'],
  },
})

log('Merge done. Merged: ' + JSON.stringify(merged?.lanes_merged) + '. Running final full verify.sh and writing the wave-36 closure receipt.')

const closurePrompt = 'You are the SD-34 wave-36 closure agent. The merge stage just reported: ' +
  JSON.stringify(merged) + '\n\n' +
  'Read ' + REPO + '/CLAUDE.md and ' + REPO + '/AGENTS.md first.\n\n' +
  SAFETY + ' You are the only process permitted to run cargo right now.\n\n' +
  'In the MAIN checkout ' + REPO + ' (confirm with git log --oneline -5 and git status --short ' +
  'first):\n' +
  '1. python3 scripts/completion_atlas.py --check -- confirm population=49438 overlap=0 ' +
  'unclassified=0, and note the fresh D/DONE counts (this wave may genuinely close 0 units and ' +
  'that is fine if honestly reported by the lanes -- do not treat an unchanged D/DONE as a ' +
  'problem to paper over).\n' +
  '2. Launch the FULL scripts/verify.sh -j 6 with nohup, properly detached, then **block on it ' +
  'with an explicit wait loop** (while kill -0 $PID; do sleep 30; done) until it genuinely ' +
  'exits -- do not end your turn or report a result before you have read the completed log ' +
  'yourself. Prior waves repeatedly had a closure agent end its turn before the ~1.5-2h run ' +
  'finished -- do not repeat that.\n' +
  '3. If any stage FAILs for a reason clearly caused by this wave own changes (e.g. stale ' +
  'site/dashboard, a stale test-count baseline): fix it and re-run verify.sh until 40/40, ' +
  'blocking the same way each time.\n' +
  '4. Once 40/40: write ONE closing entry to docs/release/SD-34-book-completion/progress.md ' +
  '(prepended) -- what wave 36 actually closed (real unit counts per lane, honestly -- if it is ' +
  'zero, say so and say why), lane C own disposition-trace findings summary, the bucket-D ' +
  'movement measured fresh, and the full verify.sh result with duration/log path.\n' +
  '5. Commit the closure entry (and any fixes from step 3) with a clear message.\n\n' +
  'Return a structured summary: whether verify.sh passed 40/40, the final D and DONE bucket ' +
  'counts, final tranche/14 HEAD sha, and whether a closure commit was made.'

const closure = await agent(closurePrompt, {
  phase: 'Final verify and wave closure',
  label: 'wave36-closure',
  effort: 'high',
  schema: {
    type: 'object',
    properties: {
      verify_passed: { type: 'boolean' },
      bucket_d_count: { type: 'number' },
      bucket_done_count: { type: 'number' },
      final_head_sha: { type: 'string' },
      closure_commit_made: { type: 'boolean' },
      notes: { type: 'string' },
    },
    required: ['verify_passed', 'final_head_sha', 'closure_commit_made'],
  },
})

return { laneA, laneB, laneC, merged, closure }
