export const meta = {
  name: 'sd34-wave37',
  description: 'SD-34 wave 37 -- domain dual-representation mechanism (7 units), Sentinel ingest re-tag (1 unit) + sub-mechanism-5 figure re-derive, Undead Savant/Plant Master/Dragon Shaman feasibility trace',
  whenToUse: 'Wave 36 closed 40/40 with 219 real closures (D: 2891->2662). This wave dispatches wave 36 lane C own named items 5, 4 (Sentinel half), and 1 from its next-cycle plan.',
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
  return 'You are a dispatched SD-34 wave-37 agent, role: ' + role + '. Work in your own ' +
    'isolated git worktree (the harness gives you one via isolation:worktree -- your cwd IS ' +
    'that worktree, already on a fresh branch off tranche/14 tip).\n\n' +
    SAFETY + '\n\n' +
    'Required reads first, keep context lean after: ' + REPO + '/CLAUDE.md, ' + REPO + '/AGENTS.md, ' +
    REPO + '/' + PKG + '/workflow-instruction.md (receipt schema, prepend-only progress.md ' +
    'convention), ' + REPO + '/' + PKG + '/artifacts/bucket-d-mining/' +
    'wave36_laneC_creature_type_collision_disposition_cycle_receipt.md (your full technical ' +
    'brief -- read the "Next-cycle plan" section and the relevant sub-mechanism section named ' +
    'in your assignment below in full before starting).\n\n' +
    'ENVIRONMENT: export RETRO_ACTOR="sd34-wave37-' + role + '"; export CARGO_BUILD_JOBS=6; ' +
    'export CARGO_TARGET_DIR="/tmp/cargo-sd34-wave37-' + role + '".\n\n' +
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
    label: 'wave37-' + role,
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

const laneAAssignment = 'Close wave 36 lane C own named Next-cycle-plan item 5 (7 units): the ' +
  '"domain-vs-class_feature dual-representation" shape -- Dragon Subdomain ~ Dragonbreath, ' +
  'Undead Subdomain ~ Death Kiss, Construct Subdomain ~ Animate Servant, and their kin. Read ' +
  'item 5 in wave36_laneC_creature_type_collision_disposition_cycle_receipt.md own Next-cycle ' +
  'plan section in full first.\n\n' +
  'This is a real, small, well-scoped mechanism gap: a domain granted-power grounding path this ' +
  'engine does not yet have. Find each of the 7 units own corpus record (search data/corpus for ' +
  'the exact record names above and their siblings), read what real PF1 mechanic each one ' +
  'encodes (a domain granted power granting a specific ability), and check whether an existing ' +
  'domain-granted-power grounding precedent exists anywhere in src/rules_core/ (grep for how ' +
  'other domain granted powers, e.g. any already-grounded Cleric domain power, are wired) before ' +
  'building a new one from scratch -- reuse a precedented idiom if one fits the same shape. ' +
  'Ground each with real corpus-quoted text, no fabricated magnitude.'

const laneBAssignment = 'Two small, independent tasks from wave 36 lane C own Next-cycle plan:\n\n' +
  '1. **Item 4 (Sentinel, 1 unit)** -- the corpus record for "Sentinel Style" or similarly named ' +
  'needs an ingest-time Kind::feat re-tag (it is a combat-style feat chain misclassified as a ' +
  'class feature by a short-word matcher collision, per wave 35/36 lane C own findings). Read ' +
  'item 4 in wave36_laneC_creature_type_collision_disposition_cycle_receipt.md in full, find the ' +
  'exact record, and fix the ingest-time classification (search scripts/ or the ingest pipeline ' +
  'for how Kind is assigned at ingest time, not at classify() time -- this is a different code ' +
  'surface than the matcher fixes prior lanes made, confirm you are editing the right one before ' +
  'touching anything).\n\n' +
  '2. **Item 2 (figure re-derivation, no code change)** -- wave 36 lane C closed 202 units of ' +
  'sub-mechanism 5 own 832-unit/60-class population as text-only. Re-derive the corrected ' +
  'remaining figure post-regen: same Counter method wave 35 lane C used originally (grep the ' +
  'reconnaissance receipt for the exact command), run it fresh against the current committed ' +
  'docs/work-inventory.json, and report the corrected population + per-class breakdown in your ' +
  'own receipt so the next wave can dispatch sub-mechanism 5 directly without its own ' +
  'reconnaissance cycle. This part is read-only, no commit needed for it beyond the receipt ' +
  'itself.'

const laneCAssignment = 'DISPOSITION TRACE (investigation, no schema/mechanism build this cycle) ' +
  'on wave 36 lane C own Next-cycle-plan item 1: Undead Savant Subschool, Plant Master, and ' +
  'Dragon Shaman own remaining magnitude-bearing sub-features (owner already traced to ' +
  'Arcanist/Hunter/Druid respectively, but not resolvable via any existing matcher signal). Read ' +
  'item 1 in wave36_laneC_creature_type_collision_disposition_cycle_receipt.md in full.\n\n' +
  'Determine which of the brief own two named paths is actually viable: (a) audit whether ' +
  '`/data/class` is reliable enough corpus-wide to trust as a new owner-resolution signal (check ' +
  'a meaningful sample of records across multiple books, not just these three, for consistency ' +
  'and correctness), or (b) the population is small enough that a narrow, per-group hardcoded ' +
  'owner override (matching the existing CLASS_FEATURE_POOLS table shape) is safer and cheaper. ' +
  'Report your exact population count for this shape (re-derive fresh, do not trust the brief own ' +
  'estimate), your recommendation with reasoning, and if option (b) is clearly the right call and ' +
  'the population is small (roughly under 20), you may implement it this cycle -- otherwise name ' +
  'it precisely for a dedicated future wave rather than rushing a fix. Do not touch other lanes ' +
  'own scope (domain mechanism, Sentinel, sub-mechanism 5 figure).'

log('Wave 37: dispatching 3 lanes in parallel, isolated worktrees, capped at the documented safe ceiling of 3 concurrent cargo-building lanes')
const [laneA, laneB, laneC] = await parallel([
  () => fixLane({ role: 'lanea', assignment: laneAAssignment }),
  () => fixLane({ role: 'laneb', assignment: laneBAssignment }),
  () => fixLane({ role: 'lanec', assignment: laneCAssignment }),
])

log('Fix phase done. Lane A: ' + (laneA?.commit_sha ?? 'FAILED') + ', Lane B: ' + (laneB?.commit_sha ?? 'FAILED') + ', Lane C: ' + (laneC?.commit_sha ?? 'FAILED') + '. Merging sequentially onto tranche/14.')

const mergePrompt = 'You are the SD-34 wave-37 sequential-merge agent. Three lanes just ' +
  'finished independent work, each in its own isolated worktree, each committed locally but ' +
  'NOT yet merged onto ' + REPO + ' tranche/14:\n' +
  '- Lane A (domain dual-representation mechanism, up to 7 units): ' + JSON.stringify(laneA) + '\n' +
  '- Lane B (Sentinel ingest re-tag + figure re-derive, up to 1 unit): ' + JSON.stringify(laneB) + '\n' +
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
  label: 'wave37-merge',
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

log('Merge done. Merged: ' + JSON.stringify(merged?.lanes_merged) + '. Running final full verify.sh and writing the wave-37 closure receipt.')

const closurePrompt = 'You are the SD-34 wave-37 closure agent. The merge stage just reported: ' +
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
  'yourself.\n' +
  '3. If any stage FAILs for a reason clearly caused by this wave own changes (e.g. stale ' +
  'site/dashboard, a stale test-count baseline): fix it and re-run verify.sh until 40/40, ' +
  'blocking the same way each time.\n' +
  '4. Once 40/40: write ONE closing entry to docs/release/SD-34-book-completion/progress.md ' +
  '(prepended) -- what wave 37 actually closed (real unit counts per lane, honestly), lane C ' +
  'own disposition findings summary, the bucket-D movement measured fresh, and the full ' +
  'verify.sh result with duration/log path.\n' +
  '5. Commit the closure entry (and any fixes from step 3) with a clear message.\n\n' +
  'Return a structured summary: whether verify.sh passed 40/40, the final D and DONE bucket ' +
  'counts, final tranche/14 HEAD sha, and whether a closure commit was made.'

const closure = await agent(closurePrompt, {
  phase: 'Final verify and wave closure',
  label: 'wave37-closure',
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
