export const meta = {
  name: 'sd34-wave33-fold-recovered-work',
  description: 'SD-34 wave 33 -- fold crash-recovered lanes A/B/C into tranche/14, one at a time',
  whenToUse: 'A server crash (kernel soft-lockup under 4 concurrent cargo/rust-lld lanes) interrupted wave 33 before lanes A/B/C could commit or merge. Their real, uncommitted work still sits in .claude/worktrees/wf_cbb90b15-7b0-{1,2,3}. Lane D already landed separately (commit d686678427).',
  phases: [
    { title: 'Fold Lane A' },
    { title: 'Fold Lane B' },
    { title: 'Fold Lane C' },
    { title: 'Final verify and landing receipt' },
  ],
}

const REPO = '/home/ubuntu/workspace/repos/codex'
const PKG = 'docs/release/SD-34-book-completion'
const BACKUP = '/tmp/claude-1000/-home-ubuntu-workspace-repos-codex-docs-release-SD-33-computed-value-verification/f34e35e3-6405-4a87-b612-555220a48600/scratchpad/crash-recovery-2026-09-02'

const RECEIPT_EXAMPLE = 'Follow this repo own established receipt shape (see any file under ' +
  'artifacts/bucket-d-mining/*.md or artifacts/epic-*/*.md for a live example). Required ' +
  'sections: Status, Commit SHA, Files touched, Identifier audit result, Wired-integration audit ' +
  'result, Acceptance criterion (verbatim, or state plainly there is no AT-34-E# card and this is ' +
  'a bucket-D mechanism cycle), Figures + re-derive commands (every number on the same line as ' +
  'its command and denominator), Build scope verified, Movement (four buckets: closure / ' +
  'reclassification / reachability / instrument-correction), Notes, Next-cycle plan. Every figure ' +
  'must be re-derived FRESH at your own final commit, never carried forward from the crashed ' +
  'run stale numbers -- a prior lane fold may have shifted the population you are measuring.'

function standingRules(role, worktree, lane) {
  const branchName = 'worktree-' + worktree.split('/').pop()
  return 'You are a dispatched SD-34 wave-33 FOLD agent. Role: ' + role + '. Lane: ' + lane + '.\n\n' +
    '## WHY THIS EXISTS\n\n' +
    'A server crash (2026-09-02, kernel soft-lockup under heavy parallel rust-lld link jobs -- ' +
    'confirmed via journalctl -b -1, NOT disk/RAM exhaustion this time, but the same class of ' +
    'hazard ' + REPO + '/.cargo/config.toml own jobs=6 comment already documents: wave 33 ' +
    'dispatched FOUR concurrent lanes, one more than the three-lane ceiling that comment own ' +
    'math was calibrated for) hit mid-flight, before lanes A/B/C of wave 33 could commit or merge ' +
    'their work. Lane D (the baseline refresh) already landed cleanly as commit d686678427 -- ' +
    'not your concern.\n\n' +
    'Your lane real, uncommitted work still exists on disk at ' + worktree + ', on local ' +
    'branch ' + branchName + ', based on commit aee47d3c5a. A full backup ' +
    'of its diff already exists at ' + BACKUP + '/ in case anything goes wrong -- you do not need to ' +
    'recreate the work, only land it correctly.\n\n' +
    'CRITICAL SAFETY RULE, non-negotiable: you are the ONLY agent permitted to run a cargo ' +
    'build or test right now. Do not run anything that would overlap in time with another lane ' +
    'cargo process -- the orchestrator guarantees this by running lanes strictly one at a time, ' +
    'never in parallel, but you must not spawn any background/detached cargo process either. One ' +
    'compile at a time, full stop. This is the direct, documented cause of today crash.\n\n' +
    '## REQUIRED READS, in order (keep context lean, do not re-read what you already know from this prompt)\n' +
    '1. ' + REPO + '/CLAUDE.md\n' +
    '2. ' + REPO + '/AGENTS.md\n' +
    '3. ' + REPO + '/' + PKG + '/workflow-instruction.md -- especially the receipt schema and the ' +
    'prepend-only progress.md convention\n' +
    '4. ' + REPO + '/' + PKG + '/decisions.md -- read in full; your own lane finding may already be ' +
    'recorded there (lane A is, at a new section 20 the crashed run wrote -- check whether it is ' +
    'already committed to tranche/14 or still only in your worktree uncommitted diff)\n\n' +
    '## ENVIRONMENT\n' +
    '  export RETRO_ACTOR="sd34-wave33-fold-' + role + '"\n' +
    '  export CARGO_TARGET_DIR="/tmp/cargo-sd34-fold-' + role + '"\n' +
    '  export CARGO_BUILD_JOBS=6\n' +
    '  cd ' + worktree + '\n'
}

async function foldLane({ role, lane, worktree, mechanism, hasReceipt, priorLaneSummary }) {
  const branchName = 'worktree-' + worktree.split('/').pop()
  const conflictGuidance = priorLaneSummary
    ? ('Lanes landed before you, in order: ' + priorLaneSummary + '. Your worktree is still based ' +
       'on the OLD commit aee47d3c5a -- tranche/14 has moved since. Expect real rebase conflicts in ' +
       'files every lane touches (docs/work-inventory.json, scripts/completion_atlas.py, ' +
       'docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json, ' +
       'src/bin/v06_work_inventory.rs). Resolve them by KEEPING BOTH lanes substantive changes ' +
       '(they target different records/mechanisms, not the same ones) -- and for every JSON ' +
       'count/derived field, do NOT hand-merge the numbers: after resolving, RE-RUN the actual ' +
       'instrument (python3 scripts/completion_atlas.py --check) and let its live output be the ' +
       'number you commit, never an arithmetic guess.')
    : ('You are the first lane to land. Rebase should be close to a clean fast-forward ' +
       '(tranche/14 only moved by lane D baseline-only commits since your worktree base).')

  const receiptGuidance = hasReceipt
    ? ('A receipt draft already exists at ' + BACKUP + '/lane' + role.slice(-1) + '_untracked/ -- ' +
       'read it, verify every one of its figures still holds after your rebase (a prior lane fold ' +
       'may have shifted the population), correct anything stale, and use it (copied into ' + worktree +
       ', same path convention as the other lanes receipts under artifacts/bucket-d-mining/).')
    : ('No receipt was drafted before the crash for this lane. Author one now, fresh, following the ' +
       'schema below. Your worktree diff (git diff aee47d3c5a) and decisions.md relevant section are ' +
       'your source material for what was actually done and why.')

  const prompt = standingRules(role, worktree, lane) + '\n' +
    '## YOUR ASSIGNMENT: land lane ' + lane + ' (' + mechanism + ') onto tranche/14\n\n' +
    conflictGuidance + '\n\n' +
    '**Steps:**\n' +
    '1. cd ' + worktree + '. Confirm git status --short still shows the expected uncommitted ' +
    'work (compare against ' + BACKUP + '/lane' + role.slice(-1) + '_status.txt if you want a sanity ' +
    'check -- that file is the crash-time snapshot, not a source of truth to copy from).\n' +
    '2. git fetch ' + REPO + ' tranche/14:refs/heads/_tranche14_sync (ignore error if ref exists, ' +
    'delete and retry) then git rebase _tranche14_sync (or git rebase tranche/14 if that local ref ' +
    'already resolves -- same repo, either works). Resolve any conflicts per the guidance above.\n' +
    '3. Run ONLY the tests scoped to files you touched (e.g. cargo test --locked --lib -j 6 -- ' +
    '<module_path_filter>), not the full workspace -- the full scripts/verify.sh runs once at the ' +
    'very end, by a different agent, after all three lanes are in.\n' +
    '4. python3 scripts/completion_atlas.py --check -- confirm population=49438, overlap=0, ' +
    'unclassified=0, done_evidence_violations=0, citation_failures=0. If any of those is nonzero, ' +
    'you have a real defect from the merge -- fix it before proceeding, do not paper over it.\n' +
    '5. ' + receiptGuidance + ' ' + RECEIPT_EXAMPLE + '\n' +
    '6. Prepend a ' + PKG + '/progress.md entry for this lane, same format/rigor as the existing ' +
    'entries already in that file (read a couple for the house style before writing yours).\n' +
    '7. Commit (in ' + worktree + ') with a clear message. Then land it on tranche/14: ' +
    'git -C ' + REPO + ' merge --ff-only ' + branchName + ' (run from ANY directory, -C targets the ' +
    'main checkout explicitly). If it is not a clean fast-forward, something upstream of you moved ' +
    'unexpectedly -- stop and report rather than force anything.\n' +
    '8. Confirm git -C ' + REPO + ' log --oneline -1 shows your commit as the new tranche/14 tip.\n\n' +
    'Return a structured summary: your final commit SHA, whether the scoped tests passed, the ' +
    'completion_atlas.py --check output (population/D/DONE counts), and whether the merge onto ' +
    'tranche/14 was clean.'

  return agent(prompt, {
    phase: 'Fold Lane ' + lane,
    label: 'fold-lane-' + lane,
    effort: 'high',
    schema: {
      type: 'object',
      properties: {
        commit_sha: { type: 'string' },
        tests_passed: { type: 'boolean' },
        completion_atlas_check: { type: 'string' },
        merge_clean: { type: 'boolean' },
        bucket_d_count: { type: 'number' },
        bucket_done_count: { type: 'number' },
        notes: { type: 'string' },
      },
      required: ['commit_sha', 'tests_passed', 'completion_atlas_check', 'merge_clean'],
    },
  })
}

log('Folding lane A (held-by-table records, 22 of 27 closed) -- first, worktree wf_cbb90b15-7b0-1')
const laneA = await foldLane({
  role: 'lanea',
  lane: 'A',
  worktree: REPO + '/.claude/worktrees/wf_cbb90b15-7b0-1',
  mechanism: 'class_feature held_by table -- 22 of 27 closed as genuinely no-upstream-description, 5 deferred (decisions.md section 20)',
  hasReceipt: false,
  priorLaneSummary: null,
})

log('Lane A landed: ' + (laneA?.commit_sha ?? 'FAILED') + '. Folding lane B (race_trait never applies, 53 units) -- worktree wf_cbb90b15-7b0-2')
const laneB = await foldLane({
  role: 'laneb',
  lane: 'B',
  worktree: REPO + '/.claude/worktrees/wf_cbb90b15-7b0-2',
  mechanism: 'race_trait_record_loaded_but_never_applies -- 53-unit cross-book-ownership Shape 8',
  hasReceipt: true,
  priorLaneSummary: 'Lane A (commit ' + (laneA?.commit_sha ?? 'unknown -- check tranche/14 log') + ')',
})

log('Lane B landed: ' + (laneB?.commit_sha ?? 'FAILED') + '. Folding lane C (class-level snapshot delta, 38 units) -- worktree wf_cbb90b15-7b0-3')
const laneC = await foldLane({
  role: 'lanec',
  lane: 'C',
  worktree: REPO + '/.claude/worktrees/wf_cbb90b15-7b0-3',
  mechanism: 'class_modelled_but_no_observed_delta_on_the_rendered_snapshot -- 38-unit class-level snapshot-delta shape',
  hasReceipt: false,
  priorLaneSummary: 'Lane A (' + (laneA?.commit_sha ?? 'unknown') + '), Lane B (' + (laneB?.commit_sha ?? 'unknown') + ')',
})

log('Lane C landed: ' + (laneC?.commit_sha ?? 'FAILED') + '. Running final full verify.sh and writing the wave-33 fold-closure receipt.')

const closurePrompt = 'You are the final SD-34 wave-33 fold-closure agent. Lanes A, B, and C of wave 33 were ' +
  'crash-recovered and just landed sequentially onto ' + REPO + ' tranche/14 branch:\n' +
  '- Lane A: ' + JSON.stringify(laneA) + '\n' +
  '- Lane B: ' + JSON.stringify(laneB) + '\n' +
  '- Lane C: ' + JSON.stringify(laneC) + '\n' +
  '(Lane D already landed earlier as commit d686678427, confirmed 40/40, not your concern except ' +
  'as context.)\n\n' +
  'Read ' + REPO + '/CLAUDE.md and ' + REPO + '/AGENTS.md first, keep context lean otherwise.\n\n' +
  'CRITICAL SAFETY RULE: you are the only process permitted to run cargo right now. Do not ' +
  'run anything concurrently with your own full verify.sh -- that overlap is what crashed the ' +
  'server earlier today (4 concurrent cargo/rust-lld lanes; see .cargo/config.toml own comment).\n\n' +
  'Your job, in the MAIN checkout ' + REPO + ' (already on tranche/14, already contains all three ' +
  'lanes work if the merges above were clean -- confirm with git log --oneline -5 and ' +
  'git status --short first; if anything looks off, stop and report rather than guessing):\n\n' +
  '1. Run python3 scripts/completion_atlas.py --check one more time at the final tip -- this ' +
  'is the authoritative population/bucket figure for wave 33 total real movement, confirm ' +
  'population=49438 overlap=0 unclassified=0.\n' +
  '2. Run the full scripts/verify.sh (all 40 stages, roughly 2 hours). Do not skip it and do not ' +
  'run it with reduced scope -- this must be the real, complete gate, since three lanes of ' +
  'content changed since the last full green run.\n' +
  '3. If it passes 40/40: write one closing entry to ' +
  'docs/release/SD-34-book-completion/progress.md (prepended, per that file own ' +
  'convention) summarizing the crash-recovery fold as a whole -- what was recovered, the ' +
  'bucket-D movement across all three lanes measured fresh (population before wave 33 vs. ' +
  'after), and the full verify.sh result with its duration and log path. Keep it factual and ' +
  'concise; the individual lanes already have their own detailed receipts, this entry is the ' +
  'integration summary, not a repeat of them.\n' +
  '4. If it does NOT pass 40/40: do NOT write a closing entry. Report exactly which stage(s) ' +
  'failed and whether the failure traces to one of the three folded lanes or is pre-existing ' +
  '-- do not guess, check the SD-34 launch package inherited-baseline failure list first ' +
  '(workflow-instruction.md item 10) before calling anything pre-existing.\n' +
  '5. Commit the closure entry (if written) with a clear message.\n\n' +
  'Return a structured summary: whether verify.sh passed, the RESULT line, duration, log path, ' +
  'final tranche/14 HEAD sha, and whether a closure commit was made.'

const closure = await agent(closurePrompt, {
  phase: 'Final verify and landing receipt',
  label: 'wave33-fold-closure',
  effort: 'high',
  schema: {
    type: 'object',
    properties: {
      verify_passed: { type: 'boolean' },
      result_line: { type: 'string' },
      duration_seconds: { type: 'number' },
      log_path: { type: 'string' },
      final_head_sha: { type: 'string' },
      closure_commit_made: { type: 'boolean' },
      notes: { type: 'string' },
    },
    required: ['verify_passed', 'final_head_sha', 'closure_commit_made'],
  },
})

return { laneA, laneB, laneC, closure }
