export const meta = {
  name: 'sd34-wave34',
  description: 'SD-34 wave 34 -- bucket D cheapest-first: lane A weapon/armor grounding (5), lane B picker UI wiring (27), lane C weapon-proficiency rows (19)',
  whenToUse: 'Autonomous next-wave dispatch after wave 33 closed (D: 2955->2924). Picks each lane own named next-cycle plan from its wave-33 receipt, cheapest-first, per this project standing convention.',
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
  return 'You are a dispatched SD-34 wave-34 agent, role: ' + role + '. Work in your own ' +
    'isolated git worktree (the harness gives you one via isolation:worktree -- your cwd IS ' +
    'that worktree, already on a fresh branch off tranche/14 tip).\n\n' +
    SAFETY + '\n\n' +
    'Required reads first, keep context lean after: ' + REPO + '/CLAUDE.md, ' + REPO + '/AGENTS.md, ' +
    REPO + '/' + PKG + '/workflow-instruction.md (receipt schema, prepend-only progress.md ' +
    'convention), ' + REPO + '/' + PKG + '/decisions.md (skim for standing rulings relevant to ' +
    'your lane).\n\n' +
    'ENVIRONMENT: export RETRO_ACTOR="sd34-wave34-' + role + '"; export CARGO_BUILD_JOBS=6; ' +
    'export CARGO_TARGET_DIR="/tmp/cargo-sd34-wave34-' + role + '".\n\n' +
    'Do the real work (do not stub, do not fixture-only it -- docs/governance/no-stub-mvp-doctrine.md). ' +
    'Run ONLY the tests scoped to what you touched, not the full workspace -- a separate agent ' +
    'runs the full scripts/verify.sh once at wave-end. Run python3 scripts/completion_atlas.py ' +
    '--check after your change and confirm population=49438 overlap=0 unclassified=0 -- if ' +
    'nonzero, you introduced a real defect, fix it before finishing. Author a full receipt ' +
    'under artifacts/bucket-d-mining/ (same shape as wave 33 lanes -- read one for house ' +
    'style) and prepend a progress.md entry. Commit in your own worktree with a clear message ' +
    '-- do NOT try to merge onto tranche/14 yourself, a later sequential stage does that.\n\n' +
    'Return a structured summary: your final local commit SHA, files touched, whether your ' +
    'scoped tests passed, the completion_atlas.py --check output, and how many units you ' +
    'closed (be honest if the true number differs from the plan -- name what you could not ' +
    'close and why, per this repo own convention).'
}

async function fixLane({ role, assignment, schema }) {
  return agent(standingRules(role) + '\n\n## YOUR ASSIGNMENT\n\n' + assignment, {
    phase: 'Fix (3 parallel, isolated worktrees, capped at 3 -- the documented safe ceiling)',
    label: 'wave34-' + role,
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

const laneAAssignment = 'Close wave 33 lane A own named 5-unit remainder: ' +
  'weapon-and-armor-proficiency class_feature records for Bard, Fighter, Paladin, Ranger, ' +
  'and Rogue (docs/release/SD-34-book-completion/artifacts/bucket-d-mining/' +
  'wave33_laneA_class_feature_held_by_no_prose_cycle_receipt.md, its own Next-cycle plan ' +
  'section, is your full brief -- read it in full before starting).\n\n' +
  'Ground pilot_compute::explain_base_class_weapon_and_armor_proficiency / ' +
  'ground_class_weapon_and_armor_proficiency for these five classes, the SAME idiom already ' +
  'grounding Sorcerer, Wizard, Cleric, Assassin, and Shadowdancer own version of this record ' +
  'shape (src/rules_core/pilot_compute/mod.rs -- grep those five class names to find the ' +
  'precedent code). Match Cleric own cycle 6 rigor exactly: read EVERY one of these five ' +
  'classes own registered archetypes in src/rules_core/rules_tables/*/archetype_tables.rs ' +
  'that supersede a weapon/armor proficiency slot before trusting any replacement text -- ' +
  'this is a real fabrication-risk hazard the receipt itself names, do not skip it or you ' +
  'will ship stale archetype text to a player who selected one.'

const laneBAssignment = 'Close wave 33 lane B own named highest-value 27-unit remainder: ' +
  'wire adoptedRaceOptions / adoptiveParentageOptions into the desktop TypeScript boundary ' +
  'and a real picker UI section (docs/release/SD-34-book-completion/artifacts/bucket-d-mining/' +
  'wave33_laneB_race_trait_never_applies_cycle_receipt.md, its own Next-cycle plan section ' +
  'item 1, is your full brief -- read it in full before starting).\n\n' +
  'The Rust backend is already done and tested: src/rules_core/race_resolver.rs own ' +
  'adoptive_parentage_options() and src/rules_core/trait_pool.rs own ' +
  'resolve_adopted_race_options() are the functions to expose. Both are text_only/' +
  'zero-magnitude records, so a real rendered description in the desktop UI is the whole bar ' +
  'for DONE -- this should be pure frontend wiring, not new backend mechanism. ' +
  'apps/desktop/src/raceCatalog/AlternateTraitPicker.tsx + alternateTraitPickerModel.ts is ' +
  'the closest existing precedent pattern for a race-trait-selector-style picker UI section -- ' +
  'read it before designing yours. Wire the Tauri IPC boundary, the picker component, and ' +
  'confirm the description actually renders end to end, not just that data reaches the ' +
  'frontend.'

const laneCAssignment = 'Close wave 33 lane C own named highest-value 19-unit remainder: ' +
  'add real CLASS_WEAPON_PROFICIENCIES rows for the 19 already-gate-eligible classes with no ' +
  'proficiency row yet found (docs/release/SD-34-book-completion/artifacts/bucket-d-mining/' +
  'wave33_laneC_class_snapshot_delta_cycle_receipt.md, its own Next-cycle plan section item ' +
  '1, names the exact class list -- read it in full before starting).\n\n' +
  'For each of the 19 classes (10 untabled base classes + 7 CRB NPC/Ex-* classes + ' +
  'Ninja/Samurai), search that class own data/corpus/<book>/class_feature/<class>/' +
  'weapon_and_armor_proficiency*.json (or sibling naming) individually -- the SAME per-key ' +
  'discipline wave 33 lane A and this same lane C already used, do not batch-assume. Add a ' +
  'real row to src/rules_core/rules_tables/crb/weapon_tables.rs (or the correct book-specific ' +
  'table file if the class is not CRB) only for classes where a record genuinely exists -- ' +
  'name, do not silently drop, any class where you search and find nothing. The chassis gate ' +
  'is already open for all 19, so a found row alone should close each unit -- verify that ' +
  'claim is actually true for a few before assuming it for all nineteen.'

log('Wave 34: dispatching 3 lanes in parallel, isolated worktrees, capped at the documented safe ceiling of 3 concurrent cargo-building lanes')
const [laneA, laneB, laneC] = await parallel([
  () => fixLane({ role: 'lanea', assignment: laneAAssignment }),
  () => fixLane({ role: 'laneb', assignment: laneBAssignment }),
  () => fixLane({ role: 'lanec', assignment: laneCAssignment }),
])

log('Fix phase done. Lane A: ' + (laneA?.commit_sha ?? 'FAILED') + ', Lane B: ' + (laneB?.commit_sha ?? 'FAILED') + ', Lane C: ' + (laneC?.commit_sha ?? 'FAILED') + '. Merging sequentially onto tranche/14.')

const mergePrompt = 'You are the SD-34 wave-34 sequential-merge agent. Three lanes just ' +
  'finished independent work, each in its own isolated worktree, each committed locally but ' +
  'NOT yet merged onto ' + REPO + ' tranche/14:\n' +
  '- Lane A (weapon/armor grounding, 5 units): ' + JSON.stringify(laneA) + '\n' +
  '- Lane B (picker UI wiring, 27 units): ' + JSON.stringify(laneB) + '\n' +
  '- Lane C (weapon-proficiency rows, 19 units): ' + JSON.stringify(laneC) + '\n\n' +
  SAFETY + ' You are the only process permitted to run cargo/npm right now -- merge one lane ' +
  'at a time, testing after each before moving to the next.\n\n' +
  'Read ' + REPO + '/CLAUDE.md and ' + REPO + '/AGENTS.md first.\n\n' +
  'For each lane that succeeded (commit_sha present, tests_passed true), in order A then B ' +
  'then C: rebase that lane worktree branch onto the CURRENT tranche/14 tip (which moves ' +
  'after each successful merge -- lane B must rebase onto a tip that already includes lane A, ' +
  'lane C onto one that includes both). Resolve any conflicts by keeping both sides ' +
  'substantive changes (different files/records per lane, not the same ones) and, for any ' +
  'shared generated JSON (docs/work-inventory.json, ' +
  'docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json), never ' +
  'hand-merge the numbers -- regenerate/re-run python3 scripts/completion_atlas.py --check ' +
  'and trust its live output. Run that lane own scoped tests one more time post-rebase before ' +
  'merging (git -C ' + REPO + ' merge --ff-only <branch>). If a lane failed (no commit_sha, or ' +
  'tests_passed false), skip it and report clearly what was skipped and why -- do not attempt ' +
  'to fix a failed lane yourself, a future cycle picks it up with fresh context.\n\n' +
  'Return a structured summary: which lanes merged cleanly, which were skipped and why, and ' +
  'the final tranche/14 HEAD sha.'

const merged = await agent(mergePrompt, {
  phase: 'Sequential merge onto tranche/14',
  label: 'wave34-merge',
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

log('Merge done. Merged: ' + JSON.stringify(merged?.lanes_merged) + '. Running final full verify.sh and writing the wave-34 closure receipt.')

const closurePrompt = 'You are the SD-34 wave-34 closure agent. The merge stage just reported: ' +
  JSON.stringify(merged) + '\n\n' +
  'Read ' + REPO + '/CLAUDE.md and ' + REPO + '/AGENTS.md first.\n\n' +
  SAFETY + ' You are the only process permitted to run cargo right now.\n\n' +
  'In the MAIN checkout ' + REPO + ' (confirm with git log --oneline -5 and git status --short ' +
  'first):\n' +
  '1. python3 scripts/completion_atlas.py --check -- confirm population=49438 overlap=0 ' +
  'unclassified=0, and note the fresh D/DONE counts.\n' +
  '2. Run the FULL scripts/verify.sh -j 6 (all 40 stages, do not reduce scope, ~1.5-2 hours). ' +
  'Wait for it to actually finish -- do not report a result until the process has genuinely ' +
  'exited.\n' +
  '3. If any stage FAILs for a reason clearly caused by this wave own changes: fix it (a ' +
  'stale generated artifact like site/dashboard needs regenerating; a stale baseline in ' +
  'scripts/verify-baselines.env needs raising with a real itemized diff, same convention as ' +
  'every prior wave) and re-run verify.sh until 40/40, same discipline as wave 33 own ' +
  'closure. If a failure looks pre-existing/unrelated, check the inherited-baseline list ' +
  '(workflow-instruction.md item 10) before calling it that -- do not guess.\n' +
  '4. Once 40/40: write ONE closing entry to docs/release/SD-34-book-completion/progress.md ' +
  '(prepended, per that file own convention) -- what wave 34 closed (real unit counts per ' +
  'lane, not the plan estimate if it differed), the bucket-D movement measured fresh, and the ' +
  'full verify.sh result with duration/log path. The three lanes already wrote their own ' +
  'detailed receipts in the Fix phase -- this is the roll-up, not a repeat.\n' +
  '5. Commit the closure entry (and any fixes from step 3) with a clear message.\n\n' +
  'Return a structured summary: whether verify.sh passed 40/40, the final D and DONE bucket ' +
  'counts, final tranche/14 HEAD sha, and whether a closure commit was made.'

const closure = await agent(closurePrompt, {
  phase: 'Final verify and wave closure',
  label: 'wave34-closure',
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
