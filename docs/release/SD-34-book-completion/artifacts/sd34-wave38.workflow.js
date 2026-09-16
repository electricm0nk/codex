export const meta = {
  name: 'sd34-wave38',
  description: 'SD-34 wave 38 -- widen DomainPowerSpec for a per-spec uses-per-day-formula override (Animate Servant, 3 units), continue sub-mechanism 5 magnitude wiring (largest classes first), further disposition trace on remaining bucket-D shapes',
  whenToUse: 'Wave 37 closed 40/40 with 2 real closures (D: 2662->2661). This wave picks up lane A own named next item (Animate Servant, 3 units) plus starts on sub-mechanism 5 own 634-unit corrected remainder, largest classes first.',
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
  return 'You are a dispatched SD-34 wave-38 agent, role: ' + role + '. Work in your own ' +
    'isolated git worktree (the harness gives you one via isolation:worktree -- your cwd IS ' +
    'that worktree, already on a fresh branch off tranche/14 tip).\n\n' +
    SAFETY + '\n\n' +
    'Required reads first, keep context lean after: ' + REPO + '/CLAUDE.md, ' + REPO + '/AGENTS.md, ' +
    REPO + '/' + PKG + '/workflow-instruction.md (receipt schema, prepend-only progress.md ' +
    'convention), ' + REPO + '/' + PKG + '/artifacts/bucket-d-mining/' +
    'wave37_laneA_domain_granted_power_death_s_kiss_cycle_receipt.md and ' +
    'wave37_laneB_sentinel_correction_and_submech5_rederive_cycle_receipt.md and ' +
    'wave37_laneC_class_field_reliability_disposition_cycle_receipt.md (your full technical ' +
    'briefs -- read the section relevant to your assignment below in full before starting).\n\n' +
    'ENVIRONMENT: export RETRO_ACTOR="sd34-wave38-' + role + '"; export CARGO_BUILD_JOBS=6; ' +
    'export CARGO_TARGET_DIR="/tmp/cargo-sd34-wave38-' + role + '".\n\n' +
    'Do the real work (do not stub, do not fixture-only it -- docs/governance/no-stub-mvp-doctrine.md). ' +
    'Be HONEST about the real outcome: if a unit does not actually move to DONE, report that ' +
    'plainly rather than claiming a false closure. Run ONLY the tests scoped to what you ' +
    'touched, not the full workspace. Run python3 scripts/completion_atlas.py --check after ' +
    'your change and confirm population=49438 overlap=0 unclassified=0. Author a full receipt ' +
    'under artifacts/bucket-d-mining/ (same shape as prior waves) and prepend a progress.md ' +
    'entry. Commit in your own worktree -- do NOT try to merge onto tranche/14 yourself.\n\n' +
    'Return a structured summary: your final local commit SHA, files touched, whether your ' +
    'scoped tests passed, the completion_atlas.py --check output, and how many units ACTUALLY ' +
    'closed to DONE (be honest -- 0 is a valid, reportable answer if that is the truth).'
}

async function fixLane({ role, assignment, schema }) {
  return agent(standingRules(role) + '\n\n## YOUR ASSIGNMENT\n\n' + assignment, {
    phase: 'Fix (3 parallel, isolated worktrees, capped at 3 -- the documented safe ceiling)',
    label: 'wave38-' + role,
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

const laneAAssignment = 'Close wave 37 lane A own named next-cycle item: widen DomainPowerSpec ' +
  'with a per-spec uses-per-day-formula override, closing Construct Subdomain Animate Servant ' +
  '(3 units). Read wave37_laneA_domain_granted_power_death_s_kiss_cycle_receipt.md own ' +
  'Next-cycle plan section in full first.\n\n' +
  'Animate Servant own DESC formula slot IS the uses-per-day count ' +
  '(DomainArtificeLVL/4-1), genuinely different from the shared 3+WIS every currently-grounded ' +
  'entry rides on. Extend src/rules_core/pilot_compute/domain_power.rs DomainPowerSpec with a ' +
  'per-spec uses-per-day-formula override field (do not remove or weaken the existing shared ' +
  '3+WIS default -- add an optional override, defaulting to the current behavior for every ' +
  'other entry). Wire Animate Servant own real formula, corpus-verified byte-for-byte against ' +
  'the actual record. Follow the SAME rigor lane A own prior cycle used for Death Kiss ' +
  '(grounds_self_application-style honest gating -- do not fabricate a bonus magnitude this ' +
  'record does not have).'

const laneBAssignment = 'Begin sub-mechanism 5 own corrected 634-unit magnitude-bearing ' +
  'remainder (class_feature_of_unmodelled_corpus_class shape, real new chassis needed per ' +
  'class) -- largest classes first, per its own dispatch table. Read ' +
  'wave37_laneB_sentinel_correction_and_submech5_rederive_cycle_receipt.md in full first for ' +
  'the corrected per-class breakdown.\n\n' +
  'Pick the single largest class group from that table (re-derive the current top group fresh ' +
  '-- do not trust a stale number, the population may have shifted again since wave 37). This ' +
  'is real new-chassis work (Epic 4/5 scope per prior waves own classification) -- read that ' +
  'class own corpus records fully, determine whether a chassis-building precedent already ' +
  'exists for a structurally similar class (grep src/rules_core/rules_tables/ and ' +
  'src/rules_core/pilot_compute/untabled_class_chassis.rs for the closest existing pattern), ' +
  'and build a real, tested chassis for it if the scope is genuinely bounded (BAB/save ' +
  'progression + the specific feature magnitude this shape needs). If the true scope turns out ' +
  'much larger than expected once you look closely, stop and report the real scope honestly ' +
  'rather than rushing an incomplete chassis -- a well-scoped partial closure is better than a ' +
  'fabricated one.'

const laneCAssignment = 'Continue wave 37 lane C own disposition trace: the remaining bucket-D ' +
  'shapes not yet fully dispositioned. Read wave37_laneC_class_field_reliability_disposition_' +
  'cycle_receipt.md in full first for what it already ruled on and what remains open.\n\n' +
  'Re-derive the CURRENT full bucket-D breakdown fresh (python3 scripts/completion_atlas.py ' +
  '--check plus the sub-cause breakdown method prior waves used) and identify the next-cheapest ' +
  'named, not-yet-attempted shape or sub-shape across ALL of bucket D own six original ' +
  'mechanisms (not just the ones already mined this session) -- check whether Shape 2 own ' +
  'remaining 154 magnitude-bearing units (from wave 35 lane C own reconnaissance) have been ' +
  'touched yet, and whether Sub-mechanisms 3/4/5 from the class_feature_of_unmodelled_corpus_' +
  'class shape still have open items beyond what lanes A and B this same wave are handling. ' +
  'Disposition-trace (not necessarily fix) whatever you find cheapest and not yet claimed by ' +
  'lane A or B this wave -- name an exact population and next-step for everything you touch, ' +
  'implement a real fix only if it is small and unambiguous.'

log('Wave 38: dispatching 3 lanes in parallel, isolated worktrees, capped at the documented safe ceiling of 3 concurrent cargo-building lanes')
const [laneA, laneB, laneC] = await parallel([
  () => fixLane({ role: 'lanea', assignment: laneAAssignment }),
  () => fixLane({ role: 'laneb', assignment: laneBAssignment }),
  () => fixLane({ role: 'lanec', assignment: laneCAssignment }),
])

log('Fix phase done. Lane A: ' + (laneA?.commit_sha ?? 'FAILED') + ', Lane B: ' + (laneB?.commit_sha ?? 'FAILED') + ', Lane C: ' + (laneC?.commit_sha ?? 'FAILED') + '. Merging sequentially onto tranche/14.')

const mergePrompt = 'You are the SD-34 wave-38 sequential-merge agent. Three lanes just ' +
  'finished independent work, each in its own isolated worktree, each committed locally but ' +
  'NOT yet merged onto ' + REPO + ' tranche/14:\n' +
  '- Lane A (Animate Servant uses-per-day override, up to 3 units): ' + JSON.stringify(laneA) + '\n' +
  '- Lane B (sub-mechanism 5, largest class group, possible new chassis): ' + JSON.stringify(laneB) + '\n' +
  '- Lane C (disposition trace, possible small fix): ' + JSON.stringify(laneC) + '\n\n' +
  SAFETY + ' You are the only process permitted to run cargo/npm right now -- merge one lane ' +
  'at a time, testing after each before moving to the next.\n\n' +
  'Read ' + REPO + '/CLAUDE.md and ' + REPO + '/AGENTS.md first.\n\n' +
  'For each lane that succeeded (commit_sha present, tests_passed true), in order A then B ' +
  'then C: rebase that lane worktree branch onto the CURRENT tranche/14 tip. Resolve any ' +
  'conflicts by keeping both sides substantive changes where they target genuinely different ' +
  'code paths, and for any shared generated JSON (docs/work-inventory.json, ' +
  'docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json), never ' +
  'hand-merge the numbers -- regenerate/re-run python3 scripts/completion_atlas.py --check ' +
  'and trust its live output. Run that lane own scoped tests one more time post-rebase before ' +
  'merging (git -C ' + REPO + ' merge --ff-only <branch>). If a lane failed, skip it and ' +
  'report why.\n\n' +
  'Return a structured summary: which lanes merged cleanly, which were skipped and why, and ' +
  'the final tranche/14 HEAD sha.'

const merged = await agent(mergePrompt, {
  phase: 'Sequential merge onto tranche/14',
  label: 'wave38-merge',
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

log('Merge done. Merged: ' + JSON.stringify(merged?.lanes_merged) + '. Running final full verify.sh and writing the wave-38 closure receipt.')

const closurePrompt = 'You are the SD-34 wave-38 closure agent. The merge stage just reported: ' +
  JSON.stringify(merged) + '\n\n' +
  'Read ' + REPO + '/CLAUDE.md and ' + REPO + '/AGENTS.md first.\n\n' +
  SAFETY + ' You are the only process permitted to run cargo right now.\n\n' +
  'In the MAIN checkout ' + REPO + ' (confirm with git log --oneline -5 and git status --short ' +
  'first):\n' +
  '1. python3 scripts/completion_atlas.py --check -- confirm population=49438 overlap=0 ' +
  'unclassified=0, and note the fresh D/DONE counts.\n' +
  '2. Launch the FULL scripts/verify.sh -j 6 with nohup, properly detached, then **block on it ' +
  'with an explicit wait loop** (while kill -0 $PID; do sleep 30; done) until it genuinely ' +
  'exits -- do not end your turn or report a result before you have read the completed log ' +
  'yourself. If your own turn is at risk of ending before the ~1.5-2h run finishes, that is ' +
  'fine -- a later cycle will pick up the wait, but launch the process correctly detached ' +
  '(nohup ... & disown) so it survives regardless.\n' +
  '3. If any stage FAILs for a reason clearly caused by this wave own changes (e.g. stale ' +
  'site/dashboard, a stale test-count baseline): fix it and re-run verify.sh until 40/40, ' +
  'blocking the same way each time.\n' +
  '4. Once 40/40: write ONE closing entry to docs/release/SD-34-book-completion/progress.md ' +
  '(prepended) -- what wave 38 actually closed (real unit counts per lane, honestly), the ' +
  'bucket-D movement measured fresh, and the full verify.sh result with duration/log path.\n' +
  '5. Commit the closure entry (and any fixes from step 3) with a clear message.\n\n' +
  'Return a structured summary: whether verify.sh passed 40/40, the final D and DONE bucket ' +
  'counts, final tranche/14 HEAD sha, and whether a closure commit was made.'

const closure = await agent(closurePrompt, {
  phase: 'Final verify and wave closure',
  label: 'wave38-closure',
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
