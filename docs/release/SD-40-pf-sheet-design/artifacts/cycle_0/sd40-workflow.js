export const meta = {
  name: 'sd40-pf-sheet-design',
  description: 'SD-40 — reference picture, layout model + grid, page-1 looks (operator gate), remaining looks, print sections, closure (final look)',
  phases: [
    { title: 'C — package + cut' }, { title: 'E0 — reference' }, { title: 'E1 — model + grid' },
    { title: 'E2 — page-1 looks + gate' }, { title: 'E3 — remaining looks' }, { title: 'E4 — print sections' },
    { title: 'E5 — closure + final look' },
  ],
}

const ROOT = '/home/ubuntu/workspace/repos/codex'
const MAIN = `${ROOT}/.worktrees/sd40-main`
const PKG = 'docs/release/SD-40-pf-sheet-design'
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

const prompt = (id, lane, extra) => `You are executing card ${id} of bundle SD-40 (Campaign Codex: Pathfinder sheet design) in UNATTENDED mode. You get exactly one turn. Never ask a question.

Your tree: ${lane === 'main'
  ? `${MAIN} (branch tranche-20, the serial main lane; you are its only writer now).`
  : `side lane "${lane}": create your own tree first, exactly as ${PKG}/workflow-instruction.md §2.1 says for a side lane, at ${ROOT}/.worktrees/sd40-${lane} on branch sd40/${lane}, with its own CARGO_TARGET_DIR=/home/ubuntu/cargo-target/sd40-${lane} if you build. When your work is pushed to tranche-20, remove that worktree, its local branch and its CARGO_TARGET_DIR.`}

Read these before anything else, from your tree:
1. ${PKG}/workflow-instruction.md §2.1 — the environment prefix. Every rule in it binds you. Export RETRO_ACTOR=sd40-${id.toLowerCase()}. (The sd39-main tree it mentions no longer exists.)
2. ${PKG}/workflow-instruction.md §5, §6, §7 — the concurrent-write protocol, the per-cycle procedure, the receipt schema. Follow §6 step by step.
3. ${PKG}/decisions.md §3, §13.1 and every later "Operator answers" or "Pre-launch corrections" section. Those later sections win over older text anywhere in the package.
4. Your card's row in ${PKG}/epic-breakdown.md and in ${PKG}/kanban.md. Its criterion, FILES YOU OWN, acceptance command and "does not cover" are your whole scope.

If any "Depends on" ID in your kanban row is not complete, return status "declined" and name it. Read the existing code your card touches first; reuse before you build. Write the failing test first and quote the failing reason. A test filter matching 0 tests is a failure. Run cargo clippy --locked --tests on touched crates before your first commit of any .rs change. Report only what you actually ran and saw. Commit and push to tranche-20 before you end, even for a partial result, using the commit attribution lines your own system instructions give you.

If you find an in-scope defect outside your card, add a discovered card per decisions.md §14 (re-count the card-count constants in the same commit) and list it in "discovered". Status is mechanical: complete only if the whole criterion holds; partial with every remainder named; blocked-escalated only if an operator ruling alone can clear it. If the package's own command or wording is what is wrong, that is not a ruling: fix the package doc in your commit, log a retro correction with --verified-by, and carry on.${extra ? `\n\n${extra}` : ''}`

const MC_EXTRA = `This is a merge check, so be adversarial: re-derive the claims of this epic's cards on origin/tranche-20 from the code and commands, not from their receipts. Try to prove them wrong. Fix what you can inside this card; add a discovered card for what you cannot. Run scripts/retro.py summary for the epic. Write the "Epic report" block in progress.md (workflow-instruction.md §10): cards done, figures with their commands, page counts per golden case, open blockers. Then sweep merged, unlocked, clean sd40-* side worktrees and their CARGO_TARGET_DIRs. Never remove sd40-main.`

const GATE_EXTRA = (n) => `Operator gate. Render the PDFs and PNGs your criterion names. Write the receipt with one line per sheet: "usable: " for the operator to fill (${n} lines). If those lines already all read "usable: yes", return complete. If any reads something else, turn each named defect into a discovered card in this epic and return partial naming them. Otherwise leave the row partial, push, and return partial with the absolute file paths. Never sign for the operator.`

const ORDER = ['C1','E0.1','E0.2','E1.1','E1.2','E1.MC','E2.1','E2.2','E2.3','E2.MC','E2.5','E2.6','E2.7','E2.8','E2.14','E2.9','E2.10','E2.11','E2.15','E2.12','E2.13','E2.4','E2.MC2','E2.G','E3.1','E3.2','E3.3','E3.6','E3.4','E3.5','E3.MC','E4.1','E4.2','E4.MC','E5.1','E5.2','E5.3','E5.4','E5.5']
const startAt = ORDER.indexOf((args && args.start) || 'C1')
const skip = (id) => ORDER.indexOf(id) >= 0 && ORDER.indexOf(id) < startAt

class Stop extends Error { constructor(id, r) { super(id); this.id = id; this.r = r } }
const results = {}
const QUOTA = /usage limit|rate limit|quota|limit reached/i

// A partial or a dead agent is re-dispatched; only a ruling, a refuted premise or the quota stops the run.
async function go(id, tier, phaseName, lane = 'main', extra = '') {
  if (skip(id)) return { status: 'complete', skipped: true, discovered: [] }
  let note = (args && args.note && id === args.start) ? `\n\n${args.note}` : ''
  for (let attempt = 1; attempt <= 4; attempt++) {
    const r = await agent(prompt(id, lane, extra) + note, { label: id + (attempt > 1 ? ` #${attempt}` : ''), phase: phaseName, model: tier, schema: RESULT })
    if (r) results[id] = { status: r.status, sha: r.sha, receipt: r.receipt, attempt }
    if (r && QUOTA.test(`${r.summary} ${r.remainder || ''}`)) throw new Stop(id, { status: 'quota', remainder: r.remainder || r.summary })
    if (r && r.status === 'complete') { log(`${id} complete ${r.sha || ''}`); return r }
    if (r && (r.status === 'blocked-escalated' || r.status === 'declined')) throw new Stop(id, r)
    note = r ? `\n\nA previous attempt ended partial. It said: ${r.remainder || r.summary}\nFirst read git log and the receipt, then finish the remainder. Do not redo finished work.`
             : `\n\nA previous attempt died without a result. First read git log and git status in your tree, keep any good work, and continue.`
    log(`${id} attempt ${attempt}: ${r ? r.status : 'agent died'}`)
  }
  throw new Stop(id, { status: 'partial', remainder: 'still not complete after 4 attempts' })
}

// The board is the source of truth for discovered cards: before a merge check, run every ready unfinished row of that epic.
const BOARD = { type: 'object', properties: { rows: { type: 'array', items: { type: 'object', properties: { id: { type: 'string' }, tier: { type: 'string', enum: ['opus', 'sonnet', 'haiku'] } }, required: ['id', 'tier'] } } }, required: ['rows'] }
async function drainBoard(prefix, phaseName) {
  for (let pass = 0; pass < 10; pass++) {
    const b = await agent(`Run: git -C ${MAIN} fetch origin tranche-20 && git -C ${MAIN} show FETCH_HEAD:${PKG}/kanban.md. Change nothing. Return, in dependency order, every row whose ID starts with "${prefix}." and is not a merge-check row (its ID contains ".MC") and not an operator-gate row (${prefix}.G, E5.2), whose status is ready, waiting, in-progress or partial, and whose "Depends on" IDs are all complete. Use its Tier value: opus, sonnet or haiku. A tier naming two models counts as opus. Return an empty list if there are none.`, { label: `board ${prefix}`, phase: phaseName, model: 'haiku', schema: BOARD })
    if (!b) throw new Stop(prefix, { status: 'partial', remainder: 'board reader died' })
    if (!b.rows.length) return
    for (const r of b.rows) await go(r.id, r.tier, phaseName)
  }
  throw new Stop(prefix, { status: 'partial', remainder: 'board still had ready rows after 10 passes' })
}
async function drainEarlier(prefixes, phaseName) { for (const p of prefixes) await drainBoard(p, phaseName) }

async function epic(phaseName, ids) {
  for (const [id, tier] of ids) {
    const isMc = id.endsWith('.MC')
    if (isMc && !skip(id)) await drainBoard(id.split('.')[0], phaseName)
    await go(id, tier, phaseName, 'main', isMc ? MC_EXTRA : '')
  }
}

// SD-o: once tranche-19 is in develop, bring develop into tranche-20 between epics. Merge, never rebase a pushed branch.
async function catchUp(phaseName) {
  const r = await agent(`SD-40 lead housekeeping, between epics, in ${MAIN} (branch tranche-20; no card is running). Run: cd ${MAIN} && git status --porcelain && git fetch origin develop tranche-20. If git status lists anything, change nothing and return "dirty: <list>". If HEAD is not origin/tranche-20, change nothing and return "head mismatch". If origin/develop is already an ancestor of HEAD (git merge-base --is-ancestor origin/develop HEAD), return "up to date". Otherwise run git merge --no-edit origin/develop. On a conflict: git merge --abort and return "conflict: <files>". On success push to origin tranche-20 (never force), then append one line to ${PKG}/progress.md under its run log: "<date> develop merged into tranche-20 at <sha> (SD-o)", commit only that file with the attribution lines your own system instructions give you, push, and return "merged <sha>". Never git stash; never git add -A.`, { label: 'catch up to develop', phase: phaseName, model: 'sonnet' })
  if (!r || /dirty|mismatch|conflict/i.test(r)) throw new Stop('catch-up', { status: 'blocked-escalated', remainder: r || 'catch-up agent died' })
  log(`catch-up: ${r}`)
}

async function gate(id, n, phaseName, more = '') {
  if (skip(id)) return null
  const r = await agent(prompt(id, 'main', GATE_EXTRA(n) + more), { label: id, phase: phaseName, model: 'sonnet', schema: RESULT })
  results[id] = r ? { status: r.status, sha: r.sha, receipt: r.receipt } : { status: 'died' }
  if (r && r.status === 'complete') return null
  return { outcome: `WAITING_FOR_OPERATOR_AT_${id}`, resume: `new run with args {"start":"${id}"} after the operator signs (or after defects are carded)`, gate: r, results }
}

try {
  phase('C — package + cut')
  const ref = go('E0.1', 'opus', 'E0 — reference', 'ref').then(r => ({ r }), e => ({ e }))
  await go('C1', 'haiku', 'C — package + cut', 'main', 'C0 is complete and the lead already cut and pushed tranche-20. Your job is the one version commit: 0.20.0 on the F-1 surfaces, plus docs/release/current-release.json. Also write the pinned SHA into progress.md Summary.')

  // E0.2 depends on C1; it runs beside E1 and must land before E4.2 (both write spec.json).
  const ui = go('E0.2', 'opus', 'E0 — reference', 'smoke').then(r => ({ r }), e => ({ e }))

  phase('E1 — model + grid')
  await go('E1.1', 'opus', 'E1 — model + grid')
  await go('E1.2', 'sonnet', 'E1 — model + grid')
  const rr = await ref; if (rr.e) throw rr.e
  const uu = await ui; if (uu.e) throw uu.e
  await drainEarlier(['E0'], 'E1 — model + grid')
  await epic('E1 — model + grid', [['E1.MC', 'opus']])

  phase('E2 — page-1 looks + gate')
  await epic('E2 — page-1 looks + gate', [['E2.1','opus'],['E2.2','opus'],['E2.3','opus'],['E2.MC','opus']])

  // Round 2 (decisions.md §23): engine cards run in side lane "engine", one after another, beside the main-lane page cards.
  const P2 = 'E2 — page-1 looks + gate'
  const ENGINE_EXTRA = (last) => last
    ? 'You are the last engine-lane card: when your work is on tranche-20, remove the sd40-engine worktree, its local branch and its CARGO_TARGET_DIR.'
    : 'Engine lane, more engine cards follow you: if .worktrees/sd40-engine already exists, reuse it (sync it to origin/tranche-20 first); when your work is pushed to tranche-20, KEEP the worktree and its CARGO_TARGET_DIR for the next engine card.'
  const wrap = (p) => p.then(r => ({ r }), e => ({ e }))
  const need = async (w) => { const x = await w; if (x.e) throw x.e; return x.r }
  const p6 = go('E2.6', 'opus', P2, 'engine', ENGINE_EXTRA(false))
  const p7 = p6.then(() => go('E2.7', 'opus', P2, 'engine', ENGINE_EXTRA(false)))
  const p8 = p7.then(() => go('E2.8', 'opus', P2, 'engine', ENGINE_EXTRA(false)))
  const p14 = p8.then(() => go('E2.14', 'opus', P2, 'engine', ENGINE_EXTRA(true)))
  // E2.15 discovered by E2.7; E2.14 already removed the engine tree, so E2.15 makes it again and removes it at the end.
  const p15 = p14.then(() => go('E2.15', 'opus', P2, 'engine', ENGINE_EXTRA(true)))
  const w6 = wrap(p6), w7 = wrap(p7), w8 = wrap(p8), w14 = wrap(p14), w15 = wrap(p15)
  await go('E2.5', 'opus', P2)
  await need(w6); await need(w8)
  await go('E2.9', 'opus', P2)
  await need(w14)
  await go('E2.10', 'opus', P2)
  await go('E2.11', 'opus', P2)
  await need(w7); await need(w15)
  await go('E2.12', 'opus', P2)
  await go('E2.13', 'opus', P2)
  await go('E2.4', 'opus', P2)
  if (!skip('E2.MC2')) await drainBoard('E2', P2)
  await go('E2.MC2', 'opus', P2, 'main', MC_EXTRA)
  const g1 = await gate('E2.G', 2, P2, ' This is round 2 (decisions.md §23). Only the round-2 "usable:" lines count; the round-1 verdict above them is already carded, never card it again.'); if (g1) return g1

  phase('E3 — remaining looks')
  await catchUp('E3 — remaining looks')
  await drainEarlier(['E0','E1','E2'], 'E3 — remaining looks')
  await epic('E3 — remaining looks', [['E3.1','opus'],['E3.2','opus'],['E3.3','opus'],['E3.6','opus'],['E3.4','opus'],['E3.5','opus'],['E3.MC','opus']])

  phase('E4 — print sections')
  await epic('E4 — print sections', [['E4.1','opus'],['E4.2','opus'],['E4.MC','opus']])

  phase('E5 — closure + final look')
  await catchUp('E5 — closure + final look')
  await drainEarlier(['E0','E1','E2','E3','E4'], 'E5 — closure + final look')
  await go('E5.1', 'sonnet', 'E5 — closure + final look')
  const g2 = await gate('E5.2', 5, 'E5 — closure + final look'); if (g2) return g2
  await drainEarlier(['E2','E3','E4','E5'], 'E5 — closure + final look')
  await go('E5.3', 'opus', 'E5 — closure + final look')
  // E5.4 is two calls; the notes + truth-up half ends partial by design.
  if (!skip('E5.4') && !(args && args.truthUpDone)) {
    const t = await agent(prompt('E5.4', 'main', 'This is the release-notes + truth-up half only. An Opus claims critic follows you and closes the card, so end with status partial and name the critic as the remainder.'), { label: 'E5.4 notes + truth-up', phase: 'E5 — closure + final look', model: 'sonnet', schema: RESULT })
    if (!t || (t.status !== 'partial' && t.status !== 'complete')) throw new Stop('E5.4', t || { status: 'partial', remainder: 'truth-up died' })
  }
  if (!skip('E5.4')) {
    const critic = await agent(prompt('E5.4', 'main', 'This is the claims-critic half of E5.4. The release notes and truth-up have landed. Check every capability claim, path and number they made against the code. Fix what is wrong, then mark E5.4 complete.'), { label: 'E5.4 critic', phase: 'E5 — closure + final look', model: 'opus', schema: RESULT })
    if (!critic || critic.status !== 'complete') throw new Stop('E5.4', critic || { status: 'partial', remainder: 'critic died' })
  }
  const pr = await go('E5.5', 'sonnet', 'E5 — closure + final look', 'main', 'Do not merge. The operator merges. Put the PR URL in your summary. Wait for the PR checks inside your turn (gh pr checks --watch). A check that fails on runner loss or disk (no test result: FAILED line) gets one rerun; a real test failure is a discovered card. End the PR body with the attribution lines:\n\n🤖 Generated with [Claude Code](https://claude.com/claude-code)\n\nhttps://claude.ai/code/session_01C6T1xzwL1R5c3Ri4UmkThY')
  const cleanup = await agent(`SD-40 is closed and its PR is open. Clean up the disk. From ${ROOT} run: git worktree list; then check ${MAIN} is clean with git status --porcelain and that HEAD equals origin/tranche-20. If both hold: git worktree remove ${MAIN}, delete /home/ubuntu/cargo-target/sd40-* , delete any local sd40/* branches, git worktree prune. If either fails, remove nothing and report. Do not delete tranche-20. Return df -h / and the final worktree list.`, { label: 'final cleanup', phase: 'E5 — closure + final look', model: 'haiku' })
  return { outcome: 'PR_OPEN', pr: pr.summary, cleanup, results }
} catch (e) {
  if (e instanceof Stop) return { outcome: 'STOPPED', card: e.id, status: e.r.status, needs: e.r.remainder, summary: e.r.summary, resume: `new run with args {"start":"${e.id}"}`, results }
  throw e
}
