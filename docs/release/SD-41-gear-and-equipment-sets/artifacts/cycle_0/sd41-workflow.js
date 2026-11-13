export const meta = {
  name: 'sd41-gear-and-equipment-sets',
  description: 'SD-41 — engine (sets, containers, carried weapons), desktop backend, gear screen, print + operator gate, closure',
  phases: [
    { title: 'C — package + cut' }, { title: 'E1 — engine (Phase 1)' },
    { title: 'E2 — desktop backend' }, { title: 'E3 — gear screen' }, { title: 'E4 — print + gate' },
    { title: 'E5 — closure' },
  ],
}

const ROOT = '/home/ubuntu/workspace/repos/codex'
const MAIN = `${ROOT}/.worktrees/sd41-main`
const PKG = 'docs/release/SD-41-gear-and-equipment-sets'
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

const ORDER = ['C1','E1.1','E1.2','E1.3','E1.4','E1.5','E1.6','E1.7','E1.10','E1.8','E1.9','E1.11','E1.MC']
const startAt = Math.max(0, ORDER.indexOf((args && args.start) || 'C1'))
const skip = (id) => ORDER.indexOf(id) >= 0 && ORDER.indexOf(id) < startAt

const prompt = (id, lane, extra) => `You are executing card ${id} of bundle SD-41 (Campaign Codex: gear and equipment sets) in UNATTENDED mode. You get exactly one turn. Never ask a question.

Your tree: ${lane === 'main'
  ? `${MAIN} (branch tranche-21, the serial main lane; you are its only writer now).`
  : `side lane "${lane}": if ${ROOT}/.worktrees/sd41-${lane} exists, reuse it (sync it to origin/tranche-21 first); otherwise create it exactly as ${PKG}/workflow-instruction.md §2.1 says, with CARGO_TARGET_DIR=/home/ubuntu/cargo-target/sd41-${lane}. Push your work to tranche-21 (fetch + rebase + push, retry up to 5 times, never force).${extra && extra.includes('LAST_IN_LANE') ? ' You are the last card of this lane: after your push, remove the worktree, its local branch and its CARGO_TARGET_DIR.' : ' Keep the worktree and CARGO_TARGET_DIR for the next card of this lane.'}`}

Read these before anything else, from your tree:
1. ${PKG}/workflow-instruction.md §2.1 — the environment prefix. Every rule in it binds you, above all: at most 3 cargo lanes across the box (SD-40's run is live — never touch any sd40-* tree), Phase-1 cards never write under apps/ (except the named price test file), never git stash, never git add -A.
2. ${PKG}/workflow-instruction.md §5, §6, §7 — concurrent-write protocol, per-cycle procedure, receipt schema. Follow §6 step by step.
3. ${PKG}/decisions.md, every section from §20 on (later sections win; §30.1 withdraws part of §30).
4. Your card's row in ${PKG}/epic-breakdown.md and in ${PKG}/kanban.md. Its criterion, FILES YOU OWN, acceptance command and "does not cover" are your whole scope.

If any "Depends on" ID in your kanban row is not complete, return status "declined" and name it. The lead has not run your RED command: run it yourself on the base first and quote the failing reason (R21). A test filter matching 0 tests is a failure. Read the existing code your card touches first; reuse before you build. Run cargo clippy --locked --tests on touched workspaces before your first commit of any .rs change. Report only what you actually ran and saw. Commit and push to tranche-21 before you end, even for a partial result, using the commit attribution lines your own system instructions give you.

If you find an in-scope defect outside your card, add a discovered card per the package's discovered-card rule (re-count the card-count constants in the same commit) and list it in "discovered". Status is mechanical: complete only if the whole criterion holds; partial with every remainder named; blocked-escalated only if an operator ruling or a lead action alone can clear it. If the package's own command or wording is wrong, fix the package doc in your commit, log a retro correction with --verified-by, and carry on.${extra ? `\n\n${extra.replace('LAST_IN_LANE', '').trim()}` : ''}`

const MC_EXTRA = `This is a merge check, so be adversarial: re-derive the claims of this epic's cards on origin/tranche-21 from the code and commands, not from their receipts. Try to prove them wrong. Fix what you can inside this card; add a discovered card for what you cannot. Run the partition check and the corpus field check your row names. Run scripts/retro.py summary for the epic. Write the "Epic report" block in progress.md. Sweep merged, clean sd41-* side worktrees and their CARGO_TARGET_DIRs (never sd41-main, never any sd40-* tree). End the Phase-1 run with a resume receipt in progress.md: next is E2.1, after SD-40 merges and tranche-21 is rebased onto develop.`

const E111_EXTRA = `Network: try d20pfsrd yourself with a browser user agent (curl -A 'Mozilla/5.0 ...'). If the site refuses you, do NOT stop the whole card: write every batch file artifacts/epic_1/E1.11/batch-<n>.json (key, name, category, candidate URL) and push them, then return status "blocked-escalated" with remainder "LEAD_WEB_FETCH: <absolute path of each batch file>" — the lead dispatches a web-fetch agent per batch and restarts you with the results.`

class Stop extends Error { constructor(id, r) { super(id); this.id = id; this.r = r } }
const results = {}
const QUOTA = /usage limit|rate limit|quota|limit reached/i

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

const BOARD = { type: 'object', properties: { rows: { type: 'array', items: { type: 'object', properties: { id: { type: 'string' }, tier: { type: 'string', enum: ['opus', 'sonnet', 'haiku'] } }, required: ['id', 'tier'] } } }, required: ['rows'] }
async function drainBoard(prefix, phaseName) {
  for (let pass = 0; pass < 10; pass++) {
    const b = await agent(`Run: git -C ${MAIN} fetch origin tranche-21 && git -C ${MAIN} show FETCH_HEAD:${PKG}/kanban.md. Change nothing. Return, in dependency order, every row whose ID starts with "${prefix}." and is not a merge-check row (its ID contains ".MC"), not an operator-gate row (ID ends in ".G") and not E1.11, whose status is ready, waiting, in-progress or partial, and whose "Depends on" IDs are all complete. Use its Tier value: opus, sonnet or haiku. A tier naming two models counts as opus. Return an empty list if there are none.`, { label: `board ${prefix}`, phase: phaseName, model: 'haiku', schema: BOARD })
    if (!b) throw new Stop(prefix, { status: 'partial', remainder: 'board reader died' })
    if (!b.rows.length) return
    for (const r of b.rows) await go(r.id, r.tier, phaseName)
  }
  throw new Stop(prefix, { status: 'partial', remainder: 'board still had ready rows after 10 passes' })
}

try {
  const P1 = 'E1 — engine (Phase 1)'
  phase('C — package + cut')
  await go('C1', 'haiku', 'C — package + cut', 'main', 'C0 is complete and tranche-21 is cut and pushed. Your job is the one version commit: 0.21.0 on the 14 surfaces your row names, plus docs/release/current-release.json and the notes pointer.')

  phase(P1)
  // Side lane convert (E1.8 -> E1.9 -> E1.11) beside the main lane E1.1 ... E1.10.
  const wrap = (p) => p.then(r => ({ r }), e => ({ e }))
  const conv = wrap((async () => {
    await go('E1.8', 'opus', P1, 'convert')
    await go('E1.9', 'opus', P1, 'convert')
    await go('E1.11', 'opus', P1, 'convert', 'LAST_IN_LANE ' + E111_EXTRA)
  })())
  for (const id of ['E1.1','E1.2','E1.3','E1.4','E1.5','E1.6','E1.7','E1.10']) await go(id, 'opus', P1)
  const c = await conv
  if (c.e) throw c.e
  if (!skip('E1.MC')) await drainBoard('E1', P1)
  await go('E1.MC', 'opus', P1, 'main', MC_EXTRA)
  return { outcome: 'PHASE_1_DONE', resume: 'Phase 2: new run with args {"start":"E2.1"} after SD-40 merges and tranche-21 is rebased onto develop (script needs its Phase-2 part added)', results }
} catch (e) {
  if (e instanceof Stop) return { outcome: 'STOPPED', card: e.id, status: e.r.status, needs: e.r.remainder, summary: e.r.summary, resume: `new run with args {"start":"${e.id}"}`, results }
  throw e
}
