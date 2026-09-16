export const meta = {
  name: 'sd34-wave35',
  description: 'SD-34 wave 35 -- Skinwalker Change Shape picker (20), Human Tribalistic Languages (2), plus a reconnaissance pass on the two big untouched bucket-D shapes',
  whenToUse: 'Wave 34 closed 40/40 (D 2924->2891). The next named item (Samurai + 17 absent classes) needs an operator policy ruling, so this wave skips it and picks the next ready, unblocked items from lane B wave-33 own next-cycle plan.',
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
  return 'You are a dispatched SD-34 wave-35 agent, role: ' + role + '. Work in your own ' +
    'isolated git worktree (the harness gives you one via isolation:worktree -- your cwd IS ' +
    'that worktree, already on a fresh branch off tranche/14 tip).\n\n' +
    SAFETY + '\n\n' +
    'Required reads first, keep context lean after: ' + REPO + '/CLAUDE.md, ' + REPO + '/AGENTS.md, ' +
    REPO + '/' + PKG + '/workflow-instruction.md (receipt schema, prepend-only progress.md ' +
    'convention), ' + REPO + '/' + PKG + '/decisions.md (skim for standing rulings relevant to ' +
    'your lane).\n\n' +
    'ENVIRONMENT: export RETRO_ACTOR="sd34-wave35-' + role + '"; export CARGO_BUILD_JOBS=6; ' +
    'export CARGO_TARGET_DIR="/tmp/cargo-sd34-wave35-' + role + '".\n\n' +
    'Do the real work (do not stub, do not fixture-only it -- docs/governance/no-stub-mvp-doctrine.md). ' +
    'Run ONLY the tests scoped to what you touched, not the full workspace -- a separate agent ' +
    'runs the full scripts/verify.sh once at wave-end. Run python3 scripts/completion_atlas.py ' +
    '--check after your change and confirm population=49438 overlap=0 unclassified=0 -- if ' +
    'nonzero, you introduced a real defect, fix it before finishing. Author a full receipt ' +
    'under artifacts/bucket-d-mining/ (same shape as prior waves -- read one for house style) ' +
    'and prepend a progress.md entry. Commit in your own worktree with a clear message -- do ' +
    'NOT try to merge onto tranche/14 yourself, a later sequential stage does that.\n\n' +
    'Return a structured summary: your final local commit SHA, files touched, whether your ' +
    'scoped tests passed, the completion_atlas.py --check output, and how many units you ' +
    'closed (be honest if the true number differs from the plan -- name what you could not ' +
    'close and why, per this repo own convention).'
}

async function fixLane({ role, assignment, schema }) {
  return agent(standingRules(role) + '\n\n## YOUR ASSIGNMENT\n\n' + assignment, {
    phase: 'Fix (3 parallel, isolated worktrees, capped at 3 -- the documented safe ceiling)',
    label: 'wave35-' + role,
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

const laneAAssignment = 'Close wave 33 lane B own named 20-unit remainder: the Skinwalker ' +
  '"Change Shape" race_trait population (docs/release/SD-34-book-completion/artifacts/' +
  'bucket-d-mining/wave33_laneB_race_trait_never_applies_cycle_receipt.md, its own Next-cycle ' +
  'plan item 2, is your starting brief -- read it in full).\n\n' +
  'This shape needs a TYPE-pool option-picker resolver mechanism, not a data lookup: ' +
  'src/rules_core/trait_pool.rs already has the precedent pattern (resolve_adopted_race_options, ' +
  'wired in wave 34 lane B) -- follow the SAME idiom for a new resolve_skinwalker_change_shape_options ' +
  'or similarly-named function. The corpus records live under BOTH ' +
  'data/corpus/bestiary_5/race_trait_generic/skinwalker_change_shape_*.json (per-attribute grant ' +
  'records, e.g. claw/bite/darkvision/ability-score bonuses) AND ' +
  'data/corpus/bestiary_5/race_trait/skinwalker/*_kin_change_shape.json (the were-creature-kin ' +
  'variant selector records, e.g. werebear_kin_change_shape.json, wererat_kin_change_shape.json) -- ' +
  'read enough of both shapes to understand whether this is ONE flat option list or a two-level ' +
  'selection (choose a kin, which then grants its own bundle of attribute records). Wire the ' +
  'result into the desktop TypeScript boundary and picker UI the same way wave 34 lane B did for ' +
  'adoptedRaceOptions -- read apps/desktop/src/boundary/loadAlternateRacialTraits.ts and ' +
  'apps/desktop/src/raceCatalog/AlternateTraitPicker.tsx for the established pattern to extend, ' +
  'not replace. Verify end to end (a selected option renders its real description), not just ' +
  'that data reaches the frontend.'

const laneBAssignment = 'Close wave 33 lane B own named 2-unit remainder: `Human ~ Tribalistic ' +
  'Languages` (data/corpus/inner_sea_races/race_trait/human/human_tribalistic_languages.json) ' +
  'and its sibling `Human ~ Tribalistic` record (docs/release/SD-34-book-completion/artifacts/' +
  'bucket-d-mining/wave33_laneB_race_trait_never_applies_cycle_receipt.md, its own Next-cycle ' +
  'plan item 4, names this pair and states it needs a new TEMPLATE:-reading mechanism -- read ' +
  'that section and the two corpus records in full before starting).\n\n' +
  'Read the record own TEMPLATE: token(s) directly and determine what real PF1 mechanic they ' +
  'encode (likely a bonus language grant keyed off the character selected region/ethnicity ' +
  'template). Check whether an existing template-reading precedent already exists anywhere in ' +
  'src/rules_core/ (grep for "TEMPLATE:" handling) before building a new one from scratch -- ' +
  'reuse a precedented idiom if one fits. Ground it the same rigor this bundle already applies ' +
  'elsewhere: real corpus-quoted text, no fabricated magnitude, verified against the actual token.'

const laneCAssignment = 'RECONNAISSANCE ONLY -- do not attempt closures, name the path for a ' +
  'future wave, matching the wave-32-lane-C precedent (docs/release/SD-34-book-completion/' +
  'progress.md, search for "Wave 32, Lane C" for that precedent entry own shape and rigor to ' +
  'match).\n\n' +
  'Bucket D still holds two large, entirely untouched sub-shapes from the original six-mechanism ' +
  'enumeration: **931 units** of `class_feature_of_unmodelled_corpus_class` (75 classes, real ' +
  'non-zero magnitude, needs new chassis per Epic 4/5) and **179 units** of ' +
  '`class_feature_no_dedicated_magnitude_id_matched_the_record_slug` (also real magnitude, also ' +
  'not yet mined). Run `python3 scripts/completion_atlas.py --check` to confirm both counts are ' +
  'still current, then decompose EACH of these two shapes into named, population-counted ' +
  'sub-mechanisms (the same table format wave 32 lane C used for the original 2,955-unit bucket ' +
  'D breakdown) -- cheapest-first, so the next wave can dispatch directly from your table without ' +
  'its own reconnaissance cycle. For the 75-class `class_feature_of_unmodelled_corpus_class` ' +
  'population specifically: group by WHICH class chassis is missing (not yet in ' +
  'has_supported_class_chassis / the untabled_base_class_chassis registry wave 34 lane C ' +
  'touched) versus which classes already have a chassis but a specific feature within them is ' +
  'unmodelled -- these are structurally different fixes. Do NOT write any Rust/corpus changes ' +
  'this cycle -- this is a read-only mining cycle. Author a reconnaissance receipt (same ' +
  'artifacts/bucket-d-mining/ location, same schema, Status: partial, Movement: 0 closure/0 ' +
  'reclassification/0 reachability/report as instrument-correction only if you fix a genuine ' +
  'stale figure you find along the way) and a progress.md entry. Commit only docs/receipt ' +
  'changes -- no source or corpus files.'

log('Wave 35: dispatching 3 lanes in parallel, isolated worktrees, capped at the documented safe ceiling of 3 concurrent cargo-building lanes')
const [laneA, laneB, laneC] = await parallel([
  () => fixLane({ role: 'lanea', assignment: laneAAssignment }),
  () => fixLane({ role: 'laneb', assignment: laneBAssignment }),
  () => fixLane({ role: 'lanec', assignment: laneCAssignment }),
])

log('Fix phase done. Lane A: ' + (laneA?.commit_sha ?? 'FAILED') + ', Lane B: ' + (laneB?.commit_sha ?? 'FAILED') + ', Lane C: ' + (laneC?.commit_sha ?? 'FAILED') + '. Merging sequentially onto tranche/14.')

const mergePrompt = 'You are the SD-34 wave-35 sequential-merge agent. Three lanes just ' +
  'finished independent work, each in its own isolated worktree, each committed locally but ' +
  'NOT yet merged onto ' + REPO + ' tranche/14:\n' +
  '- Lane A (Skinwalker Change Shape picker, 20 units): ' + JSON.stringify(laneA) + '\n' +
  '- Lane B (Human Tribalistic Languages, 2 units): ' + JSON.stringify(laneB) + '\n' +
  '- Lane C (reconnaissance only, docs-only): ' + JSON.stringify(laneC) + '\n\n' +
  SAFETY + ' You are the only process permitted to run cargo/npm right now -- merge one lane ' +
  'at a time, testing after each before moving to the next. Lane C is docs-only, so it should ' +
  'merge trivially with no test re-run needed -- confirm this by checking its files_touched ' +
  'list before assuming.\n\n' +
  'Read ' + REPO + '/CLAUDE.md and ' + REPO + '/AGENTS.md first.\n\n' +
  'For each lane that succeeded (commit_sha present, tests_passed true), in order A then B ' +
  'then C: rebase that lane worktree branch onto the CURRENT tranche/14 tip (which moves ' +
  'after each successful merge). Resolve any conflicts by keeping both sides substantive ' +
  'changes and, for any shared generated JSON (docs/work-inventory.json, ' +
  'docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json), never ' +
  'hand-merge the numbers -- regenerate/re-run python3 scripts/completion_atlas.py --check ' +
  'and trust its live output. Run that lane own scoped tests one more time post-rebase before ' +
  'merging (git -C ' + REPO + ' merge --ff-only <branch>). If a lane failed, skip it and ' +
  'report clearly what was skipped and why.\n\n' +
  'Return a structured summary: which lanes merged cleanly, which were skipped and why, and ' +
  'the final tranche/14 HEAD sha.'

const merged = await agent(mergePrompt, {
  phase: 'Sequential merge onto tranche/14',
  label: 'wave35-merge',
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

log('Merge done. Merged: ' + JSON.stringify(merged?.lanes_merged) + '. Running final full verify.sh and writing the wave-35 closure receipt.')

const closurePrompt = 'You are the SD-34 wave-35 closure agent. The merge stage just reported: ' +
  JSON.stringify(merged) + '\n\n' +
  'Read ' + REPO + '/CLAUDE.md and ' + REPO + '/AGENTS.md first.\n\n' +
  SAFETY + ' You are the only process permitted to run cargo right now.\n\n' +
  'In the MAIN checkout ' + REPO + ' (confirm with git log --oneline -5 and git status --short ' +
  'first):\n' +
  '1. python3 scripts/completion_atlas.py --check -- confirm population=49438 overlap=0 ' +
  'unclassified=0, and note the fresh D/DONE counts.\n' +
  '2. Launch the FULL scripts/verify.sh -j 6 with nohup, properly detached (not a foregrounded ' +
  'command that could get cut off), then **block on it with an explicit wait loop** ' +
  '(while kill -0 $PID; do sleep 30; done) until it genuinely exits -- do not end your turn or ' +
  'report a result before you have read the completed log yourself. This is critical: two ' +
  'prior waves both had a closure agent report INCOMPLETE because it ended its turn before ' +
  'the ~1.5-2h run finished -- do not repeat that.\n' +
  '3. If any stage FAILs for a reason clearly caused by this wave own changes: fix it (a stale ' +
  'generated artifact like site/dashboard needs regenerating; a stale baseline in ' +
  'scripts/verify-baselines.env needs raising with a real itemized diff) and re-run verify.sh ' +
  'until 40/40, blocking the same way each time. If a failure looks pre-existing/unrelated, ' +
  'check the inherited-baseline list (workflow-instruction.md item 10) before calling it that.\n' +
  '4. Once 40/40: write ONE closing entry to docs/release/SD-34-book-completion/progress.md ' +
  '(prepended) -- what wave 35 closed (real unit counts per lane), lane C reconnaissance table ' +
  'summary, the bucket-D movement measured fresh, and the full verify.sh result with ' +
  'duration/log path.\n' +
  '5. Commit the closure entry (and any fixes from step 3) with a clear message.\n\n' +
  'Return a structured summary: whether verify.sh passed 40/40, the final D and DONE bucket ' +
  'counts, final tranche/14 HEAD sha, and whether a closure commit was made.'

const closure = await agent(closurePrompt, {
  phase: 'Final verify and wave closure',
  label: 'wave35-closure',
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
