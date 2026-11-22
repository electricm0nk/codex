export const meta = {
  name: 'sd34-wave39',
  description: 'SD-34 wave 39 -- Shape 2 remaining-54 synonym gap (word-choice audit, Unchained classes first) and the 9 CRB-base/prestige classes first-check (does a per-feature compute function exist at all)',
  whenToUse: 'Wave 38 closed 40/40 with 88+26 real closures (D: 2661->2555). Shape 2 has a named 54-unit magnitude-bearing remainder with a DIFFERENT failure shape (word-choice synonym, not the dot-segment gap wave 38 closed) -- confirmed on 3 of 19 classes. Sub-mechanism 5 (634/60 classes) was investigated twice (wave 37/38 lane B) and both times found genuinely Epic 4/5 scope, not wave-sized -- not re-dispatched this wave.',
  phases: [
    { title: 'Fix (2 parallel, isolated worktrees)' },
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
  'On 2026-09-03 a SEPARATE incident hit: a full verify.sh run on this SAME shared checkout ' +
  'got SIGKILLed (exit 137) mid-root-full from colliding with ANOTHER session concurrently ' +
  'building on the shared target/ dir (this box has other peer Claude sessions that can run ' +
  'cargo at any time without warning). This wave dispatches exactly two lanes -- do not ' +
  'exceed jobs=6, do not run a second cargo process inside your own agent while the first is ' +
  'still building, and use your OWN isolated CARGO_TARGET_DIR (set below) for every cargo ' +
  'invocation, never the shared repo target/.'

function standingRules(role) {
  return 'You are a dispatched SD-34 wave-39 agent, role: ' + role + '. Work in your own ' +
    'isolated git worktree (the harness gives you one via isolation:worktree -- your cwd IS ' +
    'that worktree, already on a fresh branch off tranche/14 tip).\n\n' +
    SAFETY + '\n\n' +
    'Required reads first, keep context lean after: ' + REPO + '/CLAUDE.md, ' + REPO + '/AGENTS.md, ' +
    REPO + '/' + PKG + '/workflow-instruction.md (receipt schema, prepend-only progress.md ' +
    'convention), ' + REPO + '/' + PKG + '/artifacts/bucket-d-mining/' +
    'wave38_laneC_shape2_dot_segment_magnitude_id_matcher_cycle_receipt.md (your full ' +
    'technical brief -- read the "Shape 2 remaining 54-unit magnitude-bearing population, ' +
    'named precisely" and "Next-cycle plan" sections in full before starting; the per-class ' +
    'unit table and the confirmed synonym-gap examples for Unchained Monk/Summoner/Wizard ' +
    'live there).\n\n' +
    'ENVIRONMENT: export RETRO_ACTOR="sd34-wave39-' + role + '"; export CARGO_BUILD_JOBS=6; ' +
    'export CARGO_TARGET_DIR="/tmp/cargo-sd34-wave39-' + role + '".\n\n' +
    'Do the real work (do not stub, do not fixture-only it -- docs/governance/no-stub-mvp-doctrine.md). ' +
    'Be HONEST about the real outcome: if a unit does not actually move to DONE, report that ' +
    'plainly rather than claiming a false closure. Run ONLY the tests scoped to what you ' +
    'touched, not the full workspace. Run python3 scripts/completion_atlas.py --check after ' +
    'your change and confirm population=49438 overlap=0 unclassified=0. Author a full receipt ' +
    'under artifacts/bucket-d-mining/ (same shape as prior waves receipts) and prepend a ' +
    'progress.md entry. Commit in your own worktree -- do NOT try to merge onto tranche/14 ' +
    'yourself.\n\n' +
    'Return a structured summary: your final local commit SHA, files touched, whether your ' +
    'scoped tests passed, the completion_atlas.py --check output, and how many units ACTUALLY ' +
    'closed to DONE (be honest -- 0 is a valid, reportable answer if that is the truth).'
}

async function fixLane({ role, assignment, schema }) {
  return agent(standingRules(role) + '\n\n## YOUR ASSIGNMENT\n\n' + assignment, {
    phase: 'Fix (2 parallel, isolated worktrees)',
    label: 'wave39-' + role,
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

const laneAAssignment = 'Close Shape 2 own remaining 54-unit magnitude-bearing population, ' +
  'starting with the four classes that ALREADY have a per-feature compute function but a ' +
  'DIFFERENT word choice than the corpus feature own slug (confirmed shape, spot-checked on ' +
  'Unchained Monk/Summoner/Wizard by wave 38 lane C, e.g. `AC Bonus` / `ac_bonus` computed as ' +
  '`armor_class_bonus`, `Ki Pool` / `ki_pool` computed as `ki_points`): Unchained Monk (7), ' +
  'Unchained Barbarian (6), Unchained Rogue (4), Unchained Summoner (3) -- 20 units total. Read ' +
  'wave38_laneC_shape2_dot_segment_magnitude_id_matcher_cycle_receipt.md own "What Shape 2 ' +
  'actually is" and "Shape 2 remaining 54-unit" sections in full first.\n\n' +
  'For EACH of the four classes, confirm the exact synonym pairs by reading ' +
  'pilot_compute/mod.rs own explanation-id literals directly (do not assume from the two ' +
  'examples already named -- there may be more than one synonym pair per class). Fix by ' +
  'widening the classifier own matcher (v06_work_inventory.rs, likely ' +
  'class_feature_exact_suffix_grounded or a sibling), NOT by renaming the engine own ' +
  'explanation ids (those are already shipped, tested, and correct as compute -- the gap is ' +
  'purely in what the classifier recognizes as a match). A per-feature alias/synonym table ' +
  '(mapping corpus feature_slug -> the engine own descriptor word it actually uses) is the ' +
  'most likely correct shape, following the SAME "read the real engine source before ' +
  'assuming this needs new computation" discipline wave 38 lane C used. Add safety guards ' +
  'the same way wave 38 lane C did (a temporary explanation-id dump test to catch false ' +
  'positives BEFORE committing, not assumed from reasoning alone) -- this codebase has a ' +
  'demonstrated history of collisions in this exact area (bare class-name / class_chassis ' +
  'facts, `.unsupported`/`.not_modelled` diagnostic mirrors). If you find the four classes ' +
  'yield fewer real closures than 20 (e.g. some units turn out to need something else once ' +
  'you read the real source), report the true number honestly and name the remaining classes ' +
  '(Duelist, Shadowdancer, Assassin, Loremaster -- prestige; Monk, Bard, Cleric, Druid, ' +
  'Paladin, Ranger, Sorcerer -- CRB base) as lane B own scope (do not touch them yourself).'

const laneBAssignment = 'First-check whether the 9 CRB-base/prestige classes from Shape 2 own ' +
  '54-unit remainder (NOT the 4 Unchained classes -- those are lane A own scope this wave) ' +
  'have ANY per-feature compute function at all, before assuming the same word-choice-synonym ' +
  'shape lane A is fixing: Monk (5), Duelist (4), Shadowdancer (4), Assassin (2), Fighter (2), ' +
  'Loremaster (2), Wizard (2), Bard/Cleric/Druid/Paladin/Ranger/Sorcerer/Psychic (1 each) -- 27 ' +
  'units total. Read wave38_laneC_shape2_dot_segment_magnitude_id_matcher_cycle_receipt.md own ' +
  '"Shape 2 remaining 54-unit" section and Next-cycle-plan item 1 in full first (it names this ' +
  'exact check as the open question).\n\n' +
  'For EACH class, grep pilot_compute/mod.rs for a `ground_<class>_class_features`-style ' +
  'dispatch function (or equivalent). If one exists and the gap is the same word-choice-' +
  'synonym shape lane A is fixing, coordinate via a synonym table entry for that class too ' +
  '(read lane A own final commit if it lands first -- if your worktree started before lane A ' +
  'own fix merged, note the collision risk and prefer disposition-tracing over touching the ' +
  'SAME matcher file lane A is editing, to avoid a merge conflict; a real code fix for a class ' +
  'confirmed to share the synonym shape can be added to the SAME synonym table lane A builds, ' +
  'but only after checking lane A own final state, not blind). If NO per-feature compute ' +
  'function exists for a class, that class is genuinely different scope (a new-chassis gap, ' +
  'not a synonym gap) -- name it precisely and do not attempt to build one this cycle (real ' +
  'Epic 4/5-shaped work, same disposition wave 37/38 lane B already established for sub-' +
  'mechanism 5). Report per-class findings honestly: how many of the 9 classes have an ' +
  'existing function, how many of the 27 units are the synonym shape vs. a genuinely ' +
  'different gap, and fix ONLY the ones that are unambiguously the synonym shape AND do not ' +
  'risk a merge collision with lane A own matcher-file edit.'

async function fixPhase() {
  return {
    laneA: await fixLane({ role: 'laneA', assignment: laneAAssignment }),
    laneB: await fixLane({ role: 'laneB', assignment: laneBAssignment }),
  }
}

async function mergeSequential(results) {
  phase('Sequential merge onto tranche/14')
  const mergePrompt = 'You are the SD-34 wave-39 sequential-merge agent. Two lanes each ' +
    'committed real work in their own isolated worktrees/branches (given below). Your job: ' +
    'fold each lane branch onto tranche/14 ONE AT A TIME (git rebase or cherry-pick, whichever ' +
    'is clean -- never merge both at once, never run two cargo builds concurrently). After ' +
    'each fold, run `python3 scripts/completion_atlas.py --check` to confirm ' +
    'population=49438 overlap=0 unclassified=0 citation_failures=0 -- if a lane own commit ' +
    'needs its own completion_atlas.py citation pins re-derived after the other lane own ' +
    'edits shifted them, do that BEFORE moving to the next lane, guarded (never ' +
    '--allow-stamp-loss against a real committed prior state). Regenerate ' +
    'docs/work-inventory.json and completion-atlas.json ONCE at the end, guarded, after both ' +
    'lanes are folded. Report the final tranche/14 HEAD sha and the completion_atlas.py ' +
    '--check output.\n\nLane results:\n' + JSON.stringify(results, null, 2)
  return agent(mergePrompt, {
    label: 'wave39-merge',
    effort: 'high',
    schema: {
      type: 'object',
      properties: {
        final_head_sha: { type: 'string' },
        completion_atlas_check: { type: 'string' },
        notes: { type: 'string' },
      },
      required: ['final_head_sha', 'completion_atlas_check'],
    },
  })
}

phase('Fix (2 parallel, isolated worktrees)')
const results = await fixPhase()
const mergeResult = await mergeSequential(results)

phase('Final verify and wave closure')
log('Wave 39 fold complete at ' + (mergeResult ? mergeResult.final_head_sha : 'unknown') +
  '. Orchestrator will run the wave-end verify.sh gate and write the roll-up receipt.')

return { results, mergeResult }
