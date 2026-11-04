export const meta = {
  name: 'sd39-ui-print-corrections',
  description: 'SD-39 — guards, layout, Market, slots, ledger, overrides, .pcg import, print, closure (unattended)',
  phases: [
    { title: 'C — package + cut' }, { title: 'E1 — guards' }, { title: 'E2 — layout' },
    { title: 'E3 — Market' }, { title: 'E4 — slots' }, { title: 'E5 — ledger' },
    { title: 'E6 — overrides' }, { title: 'E7 — import' }, { title: 'E8 — print' }, { title: 'E9 — closure' },
  ],
}

const MAIN = '/home/ubuntu/workspace/repos/codex/.worktrees/sd39-main'
const PKG = 'docs/release/SD-39-ui-print-corrections'
const RESULT = {
  type: 'object',
  properties: {
    status: { type: 'string', enum: ['complete', 'partial', 'blocked-escalated', 'declined'] },
    sha: { type: 'string' },
    receipt: { type: 'string' },
    summary: { type: 'string', description: 'What changed, the commands run and their real results, and what was not covered.' },
    remainder: { type: 'string', description: 'For partial: every remaining item by sub-cause. For blocked-escalated or declined: the exact ruling, scope or mechanism needed.' },
    discovered: {
      type: 'array',
      description: 'IDs of cards you added to kanban.md and epic-breakdown.md for in-scope defects you found. Empty if none.',
      items: { type: 'object', properties: { id: { type: 'string' }, tier: { type: 'string', enum: ['opus', 'sonnet', 'haiku'] } }, required: ['id', 'tier'] },
    },
  },
  required: ['status', 'summary', 'discovered'],
}

const prompt = (id, lane, extra) => `You are executing card ${id} of bundle SD-39 (Campaign Codex: UI and print corrections) in UNATTENDED mode. You get exactly one turn. Never ask a question.

Your tree: ${lane === 'main'
  ? `${MAIN} (branch tranche-19, the serial main lane; you are its only writer now).`
  : `side lane "${lane}": create your own tree first, exactly as ${PKG}/workflow-instruction.md §2.1 says, at /home/ubuntu/workspace/repos/codex/.worktrees/sd39-${lane}. When your work is pushed to tranche-19, remove that worktree, its local sd39/${lane} branch and its CARGO_TARGET_DIR.`}

Read these before anything else, from your tree:
1. ${PKG}/workflow-instruction.md §2.1 — the environment prefix. Every rule in it binds you. Export RETRO_ACTOR=sd39-${id.toLowerCase()}.
2. ${PKG}/workflow-instruction.md §5, §6, §7 — the concurrent-write protocol, the per-cycle procedure, the receipt schema. Follow §6 step by step.
3. ${PKG}/decisions.md §3 and §25 — operator rulings and pre-launch corrections. §25 wins over older text anywhere in the package.
4. Your card's row in ${PKG}/epic-breakdown.md and in ${PKG}/kanban.md. Its criterion, FILES YOU OWN, acceptance command and "does not cover" are your whole scope.

If any "Depends on" ID in your kanban row is not complete, return status "declined" and name it. Read the existing code your card touches first; reuse before you build. Write the failing test first and quote the failing reason. A test filter matching 0 tests is a failure. Report only what you actually ran and saw. Commit and push to tranche-19 before you end, even for a partial result, using the commit attribution lines your own system instructions give you.

If you find an in-scope defect outside your card, add a discovered card per decisions.md §21 (re-count the card-count constant in the same commit) and list it in "discovered". Status is mechanical: complete only if the whole criterion holds; partial with every remainder named; blocked-escalated only if an operator ruling alone can clear it. If the package's own command or wording is what is wrong, that is not a ruling: fix the package doc in your commit, log a retro correction with --verified-by, and carry on.${extra ? `\n\n${extra}` : ''}`

const MC_EXTRA = `This is a merge check, so be adversarial: re-derive the claims of this epic's cards on origin/tranche-19 from the code and commands, not from their receipts. Try to prove them wrong. Fix what you can inside this card; add a discovered card for what you cannot. Run scripts/retro.py summary for the epic. Write the "Epic report" block in progress.md (workflow-instruction.md §10): cards done, figures with their commands, seed deltas, open blockers. Then sweep this epic's merged, unlocked, clean sd39-* side worktrees and their CARGO_TARGET_DIRs. Never remove sd39-main.`

const ORDER = ['C1','E1.0','E1.1','E1.2','E1.3','E1.4','E1.MC','E2.1','E2.2','E2.3','E2.4','E2.MC','E3.1','E3.2','E3.3','E3.4','E3.5','E3.MC','E4.1','E4.2','E4.3','E4.4','E4.MC','E5.1','E5.2','E5.3','E5.MC','E6.1','E6.2','E6.MC','E7.1','E7.2','E7.3','E7.MC','E8.1','E8.2','E8.3','E8.4','E8.5','E8.MC','E9.1','E9.2','E9.3','E9.4','E9.5','E9.6','E9.7','E9.8','E9.9','E9.10','E9.11']
const startAt = ORDER.indexOf((args && args.start) || 'C1')
const skip = (id) => ORDER.indexOf(id) >= 0 && ORDER.indexOf(id) < startAt

class Stop extends Error { constructor(id, r) { super(id); this.id = id; this.r = r } }
const results = {}

// A partial or a dead agent is re-dispatched: needing more cycles never halts the bundle. Only a ruling or a refuted premise does.
async function go(id, tier, phaseName, lane = 'main', extra = '') {
  if (skip(id)) return { status: 'complete', skipped: true, discovered: [] }
  let note = ''
  for (let attempt = 1; attempt <= 4; attempt++) {
    const r = await agent(prompt(id, lane, extra) + note, { label: id + (attempt > 1 ? ` #${attempt}` : ''), phase: phaseName, model: tier, schema: RESULT })
    if (r) results[id] = { status: r.status, sha: r.sha, receipt: r.receipt, attempt }
    if (r && r.status === 'complete') { log(`${id} complete ${r.sha || ''}`); return r }
    if (r && (r.status === 'blocked-escalated' || r.status === 'declined')) throw new Stop(id, r)
    note = r ? `\n\nA previous attempt ended partial. It said: ${r.remainder || r.summary}\nFirst read git log and the receipt, then finish the remainder. Do not redo finished work.`
             : `\n\nA previous attempt died without a result. First read git log and git status in your tree, keep any good work, and continue.`
    log(`${id} attempt ${attempt}: ${r ? r.status : 'agent died'}`)
  }
  throw new Stop(id, { status: 'partial', remainder: `still not complete after 4 attempts` })
}

// The board is the source of truth for discovered cards: before a merge check, run every ready unfinished row of that epic.
const BOARD = { type: 'object', properties: { rows: { type: 'array', items: { type: 'object', properties: { id: { type: 'string' }, tier: { type: 'string', enum: ['opus', 'sonnet', 'haiku'] } }, required: ['id', 'tier'] } } }, required: ['rows'] }
async function drainBoard(prefix, phaseName) {
  for (let pass = 0; pass < 10; pass++) {
    const b = await agent(`Run: git -C ${MAIN} fetch origin tranche-19 && git -C ${MAIN} show FETCH_HEAD:${PKG}/kanban.md. Change nothing. Return, in dependency order, every row whose ID starts with "${prefix}." and is not the ${prefix}.MC row, whose status is ready, waiting, in-progress or partial, and whose "Depends on" IDs are all complete. Use its Tier value: opus, sonnet or haiku. A tier naming two models counts as opus. Return an empty list if there are none.`, { label: `board ${prefix}`, phase: phaseName, model: 'haiku', schema: BOARD })
    if (!b) throw new Stop(prefix, { status: 'partial', remainder: 'board reader died' })
    if (!b.rows.length) return
    for (const r of b.rows) await go(r.id, r.tier, phaseName)
  }
  throw new Stop(prefix, { status: 'partial', remainder: 'board still had ready rows after 10 passes' })
}

// A merge check can discover a card after its own drain has passed, so earlier epics are drained again at each later gate.
async function drainEarlier(prefixes, phaseName) {
  for (const p of prefixes) await drainBoard(p, phaseName)
}

async function epic(phaseName, ids) {
  for (const [id, tier] of ids) {
    const isMc = id.endsWith('.MC')
    if (isMc && !skip(id)) await drainBoard(id.split('.')[0], phaseName)
    await go(id, tier, phaseName, 'main', isMc ? MC_EXTRA : '')
  }
}

try {
  phase('C — package + cut')
  await go('C1', 'haiku', 'C — package + cut', 'main', 'C0 is complete and the lead already cut and pushed tranche-19. Your job is the one version commit: 0.19.0 on the 14 F-1 surfaces, plus docs/release/current-release.json. Also write the pinned SHA into progress.md Summary.')

  phase('E1 — guards')
  await go('E1.0', 'opus', 'E1 — guards')
  const uiA = go('E1.4', 'sonnet', 'E1 — guards', 'ui-a', 'Do not edit spec.json: state the ui-smoke row you want in your summary. E1.MC adds it.')
  await go('E1.1', 'opus', 'E1 — guards')
  await go('E1.2', 'opus', 'E1 — guards')
  await go('E1.3', 'opus', 'E1 — guards')
  await uiA
  await epic('E1 — guards', [['E1.MC', 'opus']])

  phase('E2 — layout')
  await epic('E2 — layout', [['E2.1','opus'],['E2.2','opus'],['E2.3','opus'],['E2.4','sonnet'],['E2.MC','opus']])

  phase('E3 — Market')
  await epic('E3 — Market', [['E3.1','opus'],['E3.2','opus'],['E3.3','opus']])
  await Promise.all([go('E3.4', 'opus', 'E3 — Market'), go('E3.5', 'opus', 'E3 — Market', 'sf-cpr')])
  await epic('E3 — Market', [['E3.MC', 'opus']])

  // ingest touches only crates/codex-ingest, so it runs beside E4-E6. A stop there is held until the main lane stops or E7 needs it.
  const ingest = go('E7.1', 'opus', 'E7 — import', 'ingest').then(r => ({ r }), e => ({ e }))

  phase('E4 — slots')
  await epic('E4 — slots', [['E4.1','opus'],['E4.2','opus'],['E4.3','opus'],['E4.4','opus'],['E4.MC','opus']])
  phase('E5 — ledger')
  await epic('E5 — ledger', [['E5.1','opus'],['E5.2','opus'],['E5.3','opus'],['E5.MC','opus']])
  phase('E6 — overrides')
  await epic('E6 — overrides', [['E6.1','opus'],['E6.2','opus'],['E6.MC','opus']])

  phase('E7 — import')
  const ing = await ingest
  if (ing.e) throw ing.e
  await drainEarlier(['E1','E2','E3','E4','E5','E6'], 'E7 — import')
  await epic('E7 — import', [['E7.2','opus'],['E7.3','opus'],['E7.MC','opus']])

  phase('E8 — print')
  await epic('E8 — print', [['E8.1','opus'],['E8.2','opus'],['E8.3','opus'],['E8.4','opus'],['E8.5','opus'],['E8.MC','opus']])

  phase('E9 — closure')
  await drainEarlier(['E1','E2','E3','E4','E5','E6','E7','E8'], 'E9 — closure')
  await epic('E9 — closure', [['E9.1','opus'],['E9.2','sonnet'],['E9.3','sonnet']])
  if (!skip('E9.4')) {
    const r = await agent(prompt('E9.4', 'main', 'Operator gate. Render the three PDFs named in your criterion to artifacts/epic_9/E9.4/. Write the receipt with one empty line per sheet: "usable: " for the operator to fill. Check whether those lines are already filled: if all three read "usable: yes", return complete. Otherwise leave the row at partial, push, and return partial with the three absolute file paths. Do not sign for the operator.'), { label: 'E9.4', phase: 'E9 — closure', model: 'sonnet', schema: RESULT })
    results['E9.4'] = r ? { status: r.status, sha: r.sha, receipt: r.receipt } : { status: 'died' }
    if (!r || r.status !== 'complete') return { outcome: 'WAITING_FOR_OPERATOR_AT_E9.4', resume: 'START=E9.5 after the receipt holds usable: yes three times; START=E9.4 to re-check', e94: r, results }
  }
  await go('E9.5', 'opus', 'E9 — closure')
  await go('E9.6', 'sonnet', 'E9 — closure')
  await go('E9.7', 'haiku', 'E9 — closure')
  await go('E9.8', 'sonnet', 'E9 — closure')
  await go('E9.9', 'sonnet', 'E9 — closure', 'main', 'This is the truth-up half only. An Opus claims critic follows you.')
  const critic = await agent(prompt('E9.9', 'main', 'This is the claims-critic half of E9.9. The truth-up has landed. Check every capability claim, path and number the truth-up and release notes made against the code. Fix what is wrong, then mark E9.9 complete.'), { label: 'E9.9 critic', phase: 'E9 — closure', model: 'opus', schema: RESULT })
  if (!critic || critic.status !== 'complete') throw new Stop('E9.9', critic || { status: 'partial', remainder: 'critic died' })
  await go('E9.10', 'sonnet', 'E9 — closure')
  const pr = await go('E9.11', 'sonnet', 'E9 — closure', 'main', 'Do not merge. The operator merges. Put the PR URL in your summary. End the PR body with the attribution lines:\n\n🤖 Generated with [Claude Code](https://claude.com/claude-code)\n\nhttps://claude.ai/code/session_01C6T1xzwL1R5c3Ri4UmkThY')
  const cleanup = await agent(`SD-39 is closed and its PR is open. Clean up the disk. From /home/ubuntu/workspace/repos/codex run: git worktree list; then check ${MAIN} is clean with git status --porcelain and that HEAD equals origin/tranche-19. If both hold: git worktree remove ${MAIN}, delete /home/ubuntu/cargo-target/sd39-main and any other /home/ubuntu/cargo-target/sd39-*, delete any local sd39/* branches, git worktree prune. If either fails, remove nothing and report. Do not delete tranche-19. Return df -h / and the final worktree list.`, { label: 'final cleanup', phase: 'E9 — closure', model: 'haiku' })
  return { outcome: 'PR_OPEN', pr: pr.summary, cleanup, results }
} catch (e) {
  if (e instanceof Stop) return { outcome: 'STOPPED', card: e.id, status: e.r.status, needs: e.r.remainder, summary: e.r.summary, resume: `new run with args {"start":"${e.id}"}`, results }
  throw e
}