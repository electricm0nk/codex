export const meta = {
  name: 'sd-34-wave32',
  description: 'SD-34 wave 32: close the last 2 verify.sh FAILs, repair the completion-atlas citation gate, and mine bucket D',
  phases: [{ title: 'Wave 32' }],
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
  '## LANE A -- close the last two `scripts/verify.sh` FAILs',
  '',
  'The gate stands at **38 PASS / 2 FAIL** of 40 stages, down from 14 red. Both remaining FAILs are named, small, and fully specified. Your bar is **40/40**, proven by a full `scripts/verify.sh` run whose printed `SUMMARY` you paste literally.',
  '',
  '**FAIL 1 -- `site-dashboard-check`.** The stage now fails loudly and correctly (lane B\'s `a893bfcb39` landed, so it no longer silently serves a stale cache). Its message: `site/dashboard/PF1e-dashboard.json is STALE -- run ./scripts/publish-site-dashboard.sh`. The published artifact is genuinely stale. Regenerate it through `./scripts/publish-site-dashboard.sh`, confirm the stage goes green, and commit the regenerated artifact. **Before you commit it, diff it** and say in your receipt what actually changed and why -- a regeneration here has destroyed license/PI fields and `raw_tokens` before. If the regenerator would drop data, stop and report rather than shipping the loss.',
  '',
  '**FAIL 2 -- `denominator-gate`, `violations=3` of `files_checked=149`.** Three bare figures with no same-line re-derive command, all in prose, all already located for you:',
  '  1. `docs/release/SD-34-book-completion/artifacts/epic-6-closure/AT-34-E6-001_gate-lane-b_wave26_cycle_receipt.md:138` -- a `99% CPU` figure',
  '  2. the same file, `:153` -- the second `99% CPU` figure',
  '  3. `docs/release/SD-34-book-completion/progress.md:33` -- `"950s = 757s measured + ~25% margin"`, a bare `~25%`',
  'Fix each by adding the denominator and a same-line command that re-derives it. Do NOT satisfy the gate with the words `same run` -- that was tried and it is not a re-derive command. Do NOT delete the figures to make the gate pass. `progress.md` is prepend-only as a rule, but repairing an existing line\'s provenance in place is a correction, not a rewrite -- note it in your receipt.',
  '',
  'Then run the full sweep. **Diff your stage table against the 38/2 baseline stage by stage** and name any PASS that is now FAIL as a regression this wave caused. Paste each FAIL\'s literal command and its last output line. If a stage is outside your territory, say so rather than reaching for it.',
].join('\n') + RULES

const laneB = () => BASE + [
  '## LANE B -- the closure instrument\'s own fail-closed gate is tripping',
  '',
  '`python3 scripts/completion_atlas.py --check` reports, alongside a clean `population=49438 buckets=10 unclassified=0 overlap=0` and `done_evidence_violations=0`:',
  '',
  '    citation_failure: DONE: src/bin/v06_work_inventory.rs:10172 no longer contains \'grounded\'',
  '',
  'This is condition 6 of `AT-34-E1-002` doing exactly its job: every `BUCKET_DEFINITIONS` entry must cite the real `file:line` that emits the marker string the bucket keys on, and the check verifies the cited **line\'s content**, not just its path and number. Someone\'s edit to `v06_work_inventory.rs` shifted the line, so the DONE bucket\'s citation no longer resolves.',
  '',
  'Your job:',
  '  1. Find where the `grounded` marker for the DONE bucket is really emitted now. `git log -L 10172,10172:src/bin/v06_work_inventory.rs` will show you what moved and when.',
  '  2. Re-point the citation to the live line. **Verify the new citation against the live file, not against what you expect.**',
  '  3. Audit the OTHER nine buckets\' citations the same way -- the check reports the first failure, so a green run after one fix does not prove the other nine resolve. Confirm each one individually and put the ten results in a table.',
  '  4. `--check` must exit clean, with its literal output pasted in your receipt.',
  '',
  '**The trap here is the fix that makes the gate quiet instead of correct.** Do not loosen the check to a path-only match, do not drop the citation field, and do not repoint a citation to a line that merely contains the word. A citation must name the line that actually emits the evidence string the bucket keys on. If a bucket\'s marker no longer exists anywhere, that is a real finding -- report it, do not invent a citation for it.',
  '',
  'Consider whether the citation should be anchored to something more durable than a line number, since this will break again on the next edit. If you propose that, implement it only if it keeps the content check honest; otherwise write it up as a recommendation and leave the mechanism alone.',
].join('\n') + RULES

const laneC = () => BASE + [
  '## LANE C -- mine bucket D',
  '',
  'The atlas reports `population=49438`, `DONE: 24963` -- **50.5% (24963/49438)**. Bucket D is the recommended next content target: one shape, zero magnitude.',
  '',
  '**Derive your own population at the branch tip before you plan anything.** The figures in this brief were measured at dispatch. Run `python3 scripts/completion_atlas.py --check` and `--by-book` yourself and report bucket D\'s real current size and its per-book split. If bucket D is smaller or differently shaped than this brief implies, follow the repo and say so.',
  '',
  'Then close as much of bucket D as one cycle holds, to DONE, with evidence that supports DONE -- `done_evidence_violations` must stay 0 and `--check` must stay clean. Prefer finishing one book\'s bucket D completely over touching several partially: a complete book is a closable unit, "some of each" is not.',
  '',
  '**Report movement in four separate buckets** -- closure, reclassification, reachability, instrument-correction -- and bucket-diff the inventory across your cycle to derive them mechanically. Do not report a bucket-to-bucket move as a closure. If your cycle is `partial`, name the remainder **by mechanism**, not as "the rest": per-shape enumeration is what makes the next wave closable.',
  '',
  'A text-only feature -- zero magnitude, with its description shown to the player -- is COMPLETE, not blocked. That ruling is what unstuck the class matrix; apply it.',
].join('\n') + RULES

const title = 'Wave 32'

const [a, b, c] = await parallel([
  () => agent(laneA(), { model: 'sonnet', phase: title, label: 'A: last 2 gate FAILs -> 40/40', schema: CYCLE_SCHEMA, isolation: 'worktree' }),
  () => agent(laneB(), { model: 'sonnet', phase: title, label: 'B: atlas citation gate', schema: CYCLE_SCHEMA, isolation: 'worktree' }),
  () => agent(laneC(), { model: 'sonnet', phase: title, label: 'C: bucket D', schema: CYCLE_SCHEMA, isolation: 'worktree' }),
])

return { laneA: a, laneB: b, laneC: c }
