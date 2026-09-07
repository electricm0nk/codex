export const meta = {
  name: 'sd-34-wave33',
  description: 'SD-34 wave 33: close bucket D\'s three smallest named mechanisms and refresh the four stale test baselines',
  phases: [{ title: 'Wave 33' }],
}

const RULES = [
  '\n\n## Standing rules (all lanes)',
  '',
  '**CHECKPOINT ON A CLOCK.** The Proxmox host STOPS this VM when the guest runs out of memory, with no warning and no shutdown. `.cargo/config.toml` pins `jobs = 6` -- do NOT override with `-j` and do NOT raise `CARGO_BUILD_JOBS`. Never run two cargo commands at once; run them one after the other. Commit AND push at least every 20 minutes whatever state you are in, using a `wip:` subject that names what is unverified. A commit that lives only in a worktree on this box dies with the box. Wave 13 lost three lanes ~38 minutes in, all uncommitted.',
  '',
  '**Git hygiene.** `git status --porcelain` before EVERY git write. Stage your own paths explicitly. Never `git add -A`. Never `git stash` (the bare form stashes the whole repo even from a subdirectory). Run `git diff --cached --numstat` and READ IT before committing -- a commit whose subject says one thing and whose contents do another has shipped a revert here before.',
  '',
  '**Fresh base FIRST.** Before reading anything: `git fetch origin && git log --oneline -1 origin/tranche/14`. If your HEAD is not that commit, rebase or reset onto it. A lane once opened at the tranche CUT commit and lost half a cycle re-doing landed work. The same staleness applies to THIS brief: where the repo and this brief disagree, **the repo wins** -- say so in your receipt rather than quietly following either one.',
  '',
  '**Generated files are not yours to hand-edit.** `docs/work-inventory.json` and `completion-atlas.json` are generated. Regenerate them through their producer or leave them alone.',
  '',
  '**A reclassification is not a closure.** Moving units between two non-DONE buckets closes nothing. Only `-> DONE` is closure. Report movement in four separate buckets: closure / reclassification / reachability / instrument-correction. A lane once moved 6 units C->V and its commit said it "closed 6".',
  '',
  '**Every figure states its denominator** and carries a same-line command that re-derives it. `scripts/denominator_gate.py --check` and `--check-provenance` enforce this on your own receipt -- run them on what you wrote before you commit it.',
].join('\n')

const CYCLE_SCHEMA = {
  type: 'object',
  required: ['criterion', 'status', 'commit_sha', 'row_count_command_output', 'receipt_path'],
  properties: {
    criterion: { type: 'string' },
    status: { type: 'string', enum: ['complete', 'partial', 'blocked-escalated'],
      description: 'complete = your whole assigned population reached the bar. partial = you closed part and NAMED every remaining unit by sub-cause with populations that sum exactly. blocked-escalated = you need an OPERATOR RULING; this PAUSES the bundle. Needing more cycles is never blocked-escalated -- it is partial.' },
    remainder: { type: 'string' },
    commit_sha: { type: 'string', description: 'the pushed commit for this cycle, or NONE' },
    row_count_command_output: { type: 'string' },
    receipt_path: { type: 'string' },
    figures: { type: 'string', description: 'every number with its re-derive command and denominator' },
    build_scope: { type: 'string', description: 'no-run exit, workspace result, desktop crate result, and the SHA it ran at' },
    movement: { type: 'string', description: 'four buckets: closure / reclassification / reachability / instrument-correction' },
    discoveries: { type: 'string' },
    next_cycle_plan: { type: 'string' },
  },
}

const BASE = 'You are a lane in SD-34 wave 32, on branch `tranche/14` of the codex repo. Read `CLAUDE.md` then `AGENTS.md` first. Work in your own worktree; commit and push to `origin/tranche/14`. Write a cycle receipt under `docs/release/SD-34-book-completion/artifacts/` and prepend a dated entry to that package\'s `progress.md`.\n\n'


const laneA = () => BASE + [
  '## LANE A -- the 27 `class_feature_*_held_by_*_table` records',
  '',
  'Wave 32 lane C enumerated bucket D exactly: **2,955 units of 49,438**, six mechanisms, populations summing exactly. Its receipt is `docs/release/SD-34-book-completion/artifacts/bucket-d-mining/wave32_laneC_reconnaissance_cycle_receipt.md` -- read it first.',
  '',
  'Yours is the smallest and cleanest: **27 units** across four rungs -- class-skill-list, wizard-school-spell-list, weapon-and-armor-proficiency, weapon-proficiency. All have `description: null`, verified against the corpus rather than assumed. They are set- or list-shaped records with no player-facing prose.',
  '',
  'Lane C recommended asking the operator to choose a disposition. **Do not escalate.** Determine the truth and act on it. The question is answerable from the data:',
  '  1. Does real DESC prose exist upstream and was simply never ingested? The PCGen source data is at `~/workspace/repos/pcgen/data/pathfinder/`. Go look. If prose exists, ingest it -- that is a content fix and these become ordinary text-complete units.',
  '  2. If no prose exists anywhere upstream, then these records genuinely have none, and a set/list-shaped record with no prose is not incomplete -- it is a different shape. The standing ruling is that a zero-magnitude feature whose description is shown to the player is COMPLETE. Extend it honestly: give these a distinct evidence string (NOT `text-complete`, which would be a lie about prose that does not exist) that the atlas counts as DONE, and register the reasoning in `decisions.md`.',
  '',
  'Whichever holds, **prove it before you apply it** -- name the upstream files you searched and what you found. `done_evidence_violations` must stay 0 and `completion_atlas.py --check` must stay clean, including all ten citations. If you add an evidence string, add its `BUCKET_DEFINITIONS` citation in the same cycle or the fail-closed gate will correctly trip on you.',
  '',
  'Only `-> DONE` counts as closure. Report all four movement buckets separately.',
].join('\n') + RULES

const laneB = () => BASE + [
  '## LANE B -- the 53 `race_trait_record_loaded_but_never_applies` units',
  '',
  'Read `docs/release/SD-34-book-completion/artifacts/bucket-d-mining/wave32_laneC_reconnaissance_cycle_receipt.md` first for the full bucket-D shape.',
  '',
  '**53 units**, zero magnitude, 0 of 53 carry a real description. They sit in a terminal `TraitRole::Unclassified` state in `RaceCorpus`: the record loads, and then never applies to anything.',
  '',
  'A prior cycle already named the cause -- its `AT-34-E3-001` race_trait_absent receipt calls for a **cross-book ownership shape (Shape 8)**. Find that receipt, read what it concluded, and pick the investigation up rather than restarting it.',
  '',
  'Your bar: either these 53 reach DONE with evidence that genuinely supports it, or you name precisely which of the 53 cannot and why, by sub-cause, with populations that sum to 53. A unit that loads but never applies is not automatically incomplete -- establish whether it SHOULD apply and does not (a real defect, fix it), or whether it is correctly inert (then it needs an honest evidence string, not a pretended one).',
  '',
  'Do not invent a `TraitRole` classification to make a number move. A shared name never implies a shared thing -- check the owning record.',
].join('\n') + RULES

const laneC = () => BASE + [
  '## LANE C -- the 38 `class_modelled_but_no_observed_delta_on_the_rendered_snapshot` units',
  '',
  'Read `docs/release/SD-34-book-completion/artifacts/bucket-d-mining/wave32_laneC_reconnaissance_cycle_receipt.md` first.',
  '',
  '**38 units**, `Kind::Class` (not `class_feature`), magnitude 1-2. The class is modelled, but no delta is observed on the rendered snapshot -- so the engine holds it and the player never sees it change anything.',
  '',
  'Lane C\'s own hint: this likely shares plumbing with the existing `--class-probe` instrument already in `v06_work_inventory`\'s `main()`. Start there.',
  '',
  'Scope the class-level snapshot-delta wiring and close what one cycle holds. **The failure mode to avoid is a probe that reports success without executing anything** -- the doneness instrument hierarchy puts `reach_gate` (which executes IPC) above the dumps for exactly this reason. If you build or extend a probe, prove it fires on a planted regression.',
  '',
  'A magnitude of 1-2 is not zero, so the text-only ruling does NOT apply here. These need real observed deltas.',
].join('\n') + RULES

const laneD = () => BASE + [
  '## LANE D -- refresh the four stale test baselines',
  '',
  '`scripts/verify.sh` now passes **40 of 40 stages**, but prints four BASELINE NOTES (not failures -- they must be updated deliberately):',
  '',
  '    BASELINE_ROOT_LIB_TESTS       2336 recorded, 3028 measured',
  '    BASELINE_ROOT_FULL_TESTS      7469 recorded, 8372 measured',
  '    BASELINE_ROOT_TEST_BINARIES    569 recorded,  589 measured',
  '    BASELINE_DESKTOP_TESTS         515 recorded,  572 measured',
  '',
  'These live in `scripts/verify-baselines.env`, at the LAST assignment of each name -- that file carries a long history of superseded values in comments, so edit the live assignment and confirm with `grep -n` which one actually wins.',
  '',
  '**The gaps are large because a prior cycle deliberately declined to raise them**, not because nobody noticed. That file\'s own convention is that every raise is itemized and justified: which tests, from which cycle, and why the number moved by exactly that much. Follow it. A raise you cannot itemize is a raise you should not make -- record what you could not account for rather than rounding to the measured figure silently.',
  '',
  '**A count change is not a one-file edit.** Changing a recorded count compiles clean and still leaves OTHER files red -- hardcoded assertions elsewhere pin the old numbers. Grep for both the old AND new values across `tests/`, `src/`, `apps/` and `scripts/` before committing, and run the full `scripts/verify.sh` afterwards to prove 40/40 still holds.',
  '',
  'Territory: `scripts/verify-baselines.env` plus whatever the grep proves must move with it. Do not touch lanes A/B/C\'s files.',
].join('\n') + RULES

const title = 'Wave 33'

const [a, b, c, d] = await parallel([
  () => agent(laneA(), { model: 'sonnet', phase: title, label: 'A: 27 held-by-table -> DONE', schema: CYCLE_SCHEMA, isolation: 'worktree' }),
  () => agent(laneB(), { model: 'sonnet', phase: title, label: 'B: 53 race_trait never-applies', schema: CYCLE_SCHEMA, isolation: 'worktree' }),
  () => agent(laneC(), { model: 'sonnet', phase: title, label: 'C: 38 class snapshot-delta', schema: CYCLE_SCHEMA, isolation: 'worktree' }),
  () => agent(laneD(), { model: 'sonnet', phase: title, label: 'D: 4 stale baselines', schema: CYCLE_SCHEMA, isolation: 'worktree' }),
])

return { laneA: a, laneB: b, laneC: c, laneD: d }
