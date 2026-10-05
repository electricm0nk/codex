export const meta = {
  name: 'sd37-starfinder-1e',
  description: 'SD-37 Starfinder 1e: version bump, oracle and licence, game-system partition, schema, SF converter, SF chassis, print path, desktop, parity, rules_tables data package, verify and closure',
  phases: [
    { title: 'C — cycle 0/1' },
    { title: 'E0 — oracle + licence' },
    { title: 'E1 — partition' },
    { title: 'E2 — schema' },
    { title: 'E3 — SF converter' },
    { title: 'E4 — SF chassis' },
    { title: 'E5 — SF print path' },
    { title: 'E6 — desktop' },
    { title: 'E7.1 — SF parity' },
    { title: 'E4a — rules_tables package' },
    { title: 'E7 — verify + closure' },
  ],
}

// SD-37 launch script. Source of truth for order and tier: epic-breakdown.md §0 and kanban.md
// (as revised by C0.2, commit 7c5d63e176). One chain; the only concurrency is E0 beside E1
// (and C0.1 beside them, docs/git only). Launch decisions: progress.md, 2026-10-02 rows.

const TREE = '/home/ubuntu/workspace/worktrees/codex-sd37'
const PKG = 'docs/release/SD-37-starfinder-1e'
const SESSION = 'https://claude.ai/code/session_01E3hoL4vh7JFzaZPaZWBXwH'
const WHO = { opus: 'Claude Opus 5.5', sonnet: 'Claude Sonnet 5.5', haiku: 'Claude Haiku 4.5' }

const RESULT = {
  type: 'object',
  properties: {
    status: { type: 'string', enum: ['complete', 'partial', 'blocked-escalated', 'declined'] },
    sha: { type: 'string', description: 'last commit you pushed to origin/tranche/17, or empty' },
    receipt: { type: 'string', description: 'repo path of the cycle receipt, or empty' },
    summary: { type: 'string', description: 'what you did and what you verified, with commands; seed deltas if any' },
    remainder: { type: 'string', description: 'partial only: every remaining item by sub-cause, with counts that sum' },
    ruling_needed: { type: 'string', description: 'blocked-escalated only: the exact operator ruling, write scope or precondition' },
    owned_by: { type: 'string', description: 'declined only: the card id whose row owns the file or item, e.g. E4.2; empty if none' },
    usage_limit_hit: { type: 'boolean', description: 'true if a usage-limit or quota error stopped you' },
  },
  required: ['status', 'summary'],
}

// The binding core. The full prefix is workflow-instruction.md §2.1; every agent reads it from the file.
const CORE = [
  'BINDING CORE (SD-37). The operator is away. You get exactly ONE turn; nothing re-invokes you.',
  '',
  'READ FIRST, from the main tree ' + TREE + ' (branch tranche/17):',
  '  ' + PKG + '/workflow-instruction.md : the UNATTENDED block at the top, then §2.1 (the dispatch prefix: every line binds you), §5, §6, §7, §8, §12. Read §10 and §11 if your card is a merge check or an E7 card.',
  '  ' + PKG + '/epic-breakdown.md : the note under §0 about `\\|` in table cells, your card row in §0, and your card row in its criterion table (criterion, acceptance command, "Does not cover").',
  '  ' + PKG + '/kanban.md : your row (Depends on). ' + PKG + '/decisions.md : every section those rows cite, and §12.1 (safe defaults).',
  'Then do §6 of workflow-instruction.md, step by step, for your card. The files are the source of truth; this core only repeats the rules that have failed before.',
  '',
  'RULES THAT HAVE FAILED BEFORE:',
  '- The shell working directory resets between commands. Start EVERY command with `cd <your tree> &&`. Never write in /home/ubuntu/workspace/repos/codex: it is another session\'s checkout on another branch.',
  '- First command, in your tree: the wrong-base check from §2.1. It must pass before any write.',
  '- `git status --porcelain` before every write. Stage by explicit path. Never `git add -A`, never `git stash`, never force-push.',
  '- TDD: write the failing test, confirm it fails for the intended reason, then the smallest change.',
  '- One cargo process at a time in your lane; `-j 8`; `-- --test-threads=8`. Use the CARGO_TARGET_DIR line from §2.1 exactly (one directory per source tree).',
  '- Anything that may run more than 10 minutes: nohup + log + pidfile, a not-run skeleton first, then poll inside this turn. Do not end your turn while it runs.',
  '- CONVERTER LANE: you may change crates/codex-ingest when the converter owns the defect; then the structural-diff protocol (§6 step 5) is mandatory.',
  '- Paper-sheet rule: compute only values that feed a sheet total; print the rest. No simulation.',
  '- Every figure you write carries the command that produced it and its denominator. Count with awk or Python, never `grep -o`.',
  '- No AskUserQuestion, no clarify. Take the safe default in decisions.md §12.1 and log it in progress.md. If only an operator ruling can clear a blocker, return blocked-escalated with the exact ruling.',
  '- If your premise is refuted, return declined and name the mechanism. If a file you need belongs to another card\'s row (§3), return declined with owned_by set to that card id. Do not commit a success you did not earn.',
  '- Emit scripts/retro.py events at the moment they happen (correction needs --verified-by).',
  '- Closing commit: the receipt (§7 schema), your kanban.md row, and one progress.md cycle-log row, in one commit. After it, unfiltered `git status --porcelain` prints nothing. Then the dual audit (§6 step 4), then push with the §5 protocol. Commit and push even for a partial result.',
  '- Copy every log your receipt cites into ' + PKG + '/artifacts/ before you return.',
].join('\n')

const slug = (id) => id.toLowerCase().replace(/[^a-z0-9]+/g, '-')

const treeBlock = (c) => c.lane
  ? [
      'YOUR TREE (parallel lane): make your own tree first, from the pushed branch:',
      '  git -C ' + TREE + ' fetch origin tranche/17',
      '  git -C ' + TREE + ' worktree add -b sd37/' + slug(c.id) + ' ' + TREE + '-' + slug(c.id) + ' origin/tranche/17',
      'If that tree already exists from an earlier attempt, reuse it: fetch, then rebase it onto origin/tranche/17.',
      'Work only in ' + TREE + '-' + slug(c.id) + '. Other lanes write to tranche/17 at the same time: land your commits with the §5 protocol (fetch, rebase, push HEAD:tranche/17; SD-o for kanban.md/progress.md row conflicts).',
      'When your work is pushed: remove your tree (git worktree remove), delete the local branch sd37/' + slug(c.id) + ', and delete your CARGO_TARGET_DIR. Never touch ' + TREE + ' itself; another card is writing there.',
    ].join('\n')
  : 'YOUR TREE: ' + TREE + ' (branch tranche/17). You are its only writer right now. Start with `git fetch origin` and the §5 rebase; parallel lanes may have pushed.'

const prompt = (c, attempt, prev, handed) => [
  'You are running card ' + c.id + ' of bundle SD-37 (Starfinder 1e). Card title: ' + c.title + '. Attempt ' + attempt + ' of 3.',
  'export RETRO_ACTOR=sd37-' + slug(c.id),
  '',
  CORE,
  '',
  treeBlock(c),
  '',
  c.extra ? 'CARD-SPECIFIC ORDERS:\n' + c.extra + '\n' : '',
  handed && handed.length
    ? 'ITEMS HANDED TO YOU. Earlier cards declined these because your card owns the file or item. You now own each item AND the kanban.md row of the card that handed it over:\n' + handed.map((h) => '- from ' + h.from + ': ' + h.what).join('\n') + '\n'
    : '',
  prev
    ? 'EARLIER ATTEMPT ON THIS CARD ended as "' + prev.status + '". Its report: ' + (prev.summary || '(none)') + (prev.remainder ? '\nRemainder it named: ' + prev.remainder : '') + '\nRead the tree and `git log` first: keep what landed, then finish the remainder. Do not redo finished work.\n'
    : '',
  'COMMIT TRAILER: end every commit message with these two lines:',
  'Co-Authored-By: ' + WHO[c.tier] + ' <noreply@anthropic.com>',
  'Claude-Session: ' + SESSION,
  '',
  'RETURN (structured): status is `complete` only if the acceptance command of your card passed on the pushed tree and your kanban row says complete. `partial` = part closed, with every remaining item named by sub-cause and counts that sum. `blocked-escalated` = only an operator ruling can clear it. `declined` = premise refuted or the file belongs to another card (set owned_by). Set usage_limit_hit=true if a usage or quota limit stopped you. In summary, give the commands you ran and their real results; if you could not verify something, say so plainly.',
].join('\n')

const MC = 'You are an ADVERSARIAL merge check, not an implementer. Work on the state of origin/tranche/17. Re-run the epic\'s acceptance commands yourself; do not trust the receipts. Plant the mutations your criterion names and confirm each one turns the gate red, then restore. Render the seeds your epic can reach and report per-seed deltas. Fold `scripts/retro.py summary --since <epic start>` into your receipt (§10). Fix a defect you find only if it is small and inside the epic\'s file set; otherwise return partial and name it. If you are not sure the epic holds, it is not complete.'

const card = (id, tier, phase, title, extra, lane) => ({ id: id, tier: tier, phase: phase, title: title, extra: extra || '', lane: !!lane })

const P = {
  C: 'C — cycle 0/1', E0: 'E0 — oracle + licence', E1: 'E1 — partition', E2: 'E2 — schema', E3: 'E3 — SF converter',
  E4: 'E4 — SF chassis', E5: 'E5 — SF print path', E6: 'E6 — desktop', E71: 'E7.1 — SF parity',
  E4a: 'E4a — rules_tables package', E7: 'E7 — verify + closure',
}

const C1 = card('C1', 'haiku', P.C, 'Version bump 0.17.0 (14 surfaces, one commit) + push tranche/17', [
  'tranche/17 is NOT on origin yet. You push it for the first time. So for this card only, the §5 "fetch origin tranche/17" step does not apply before your push.',
  'Step A: `git fetch origin develop`. If origin/develop is no longer 20bf84a3b2, rebase tranche/17 onto origin/develop first (the branch holds only docs commits under ' + PKG + '/).',
  'Step B: the bump. The 14 surfaces are listed in decisions.md §2 and content-unit-inventory.md row F-17; the model commit is 9a650cfd41 (`git show --name-only --format= 9a650cfd41`). Change 0.16.0 to 0.17.0 in exactly those surfaces, in ONE commit. Root Cargo.toml stays 0.1.0.',
  'Step C: run the acceptance command of card C1 (epic-breakdown.md, Epic C table).',
  'Step D: a second commit with your receipt (artifacts/cycle_0/C1_cycle_receipt.md), the kanban row, the progress row, and the "Pinned tranche/17 SHA" field in progress.md (the bump commit SHA).',
  'Step E: `git push -u origin tranche/17`. Confirm with `git ls-remote --heads origin tranche/17`.',
].join('\n'))

const C01 = card('C0.1', 'haiku', P.C, 'SD-36 loose ends + pending retro corrections', [
  'Not a gate. Do only what is safe; log the rest (safe default SD-j: a tree, branch or file that another session may own is not touched).',
  'Docs half (in your lane tree, lands on tranche/17): SD-36 README status line and kanban D2–D6 row; then run the queued retro commands under progress.md "Pending retro events" and commit docs/retro/events/ changes.',
  'Git half, each with a guard that must pass before the action:',
  '- Delete a remote branch origin/sd36/<x> ONLY if `git rev-list --count origin/develop..origin/sd36/<x>` prints 0. Never delete `test`, `update-index`, `develop`, `main`, any `tranche/*`, or any `fix/*` branch.',
  '- Remove a worktree ONLY if all hold: its path matches /home/ubuntu/workspace/worktrees/codex-epic-f*; `git -C <tree> status --porcelain` prints nothing; it is not locked; its branch has 0 commits ahead of origin/develop. Otherwise leave it and log why.',
  '- NEVER touch: /home/ubuntu/workspace/repos/codex (the main checkout, including its uncommitted docs/retro/events/root.jsonl and its .worktrees/ directory), anything under .worktrees/, /home/ubuntu/workspace/worktrees/codex-ci-oracle, ' + TREE + ' and any ' + TREE + '-* lane tree.',
  'Receipt: `git worktree list` and `git branch -r` before and after, with counts.',
].join('\n'), true)

const E01 = card('E0.1', 'sonnet', P.E0, 'Oracle sparse paths + SF completeness probe + fresh-clone proof', 'No cargo. Touch only the E0 row\'s files (workflow-instruction.md §3). The fresh-clone proof uses a new directory outside every repo tree, and you delete it after.', true)
const E02 = card('E0.2', 'opus', P.E0, 'Licence matrix SF rows + SF PI term set', 'Uses cargo for the term-set test (you are one of at most 3 cargo lanes). Never touch crates/codex-ingest. The registry test is NOT yours (moved to E3.1). `operator_sign_off` stays false: only the operator signs.', true)
const E04 = card('E0.4', 'opus', P.E0, 'Seed builds + hand values from the SRD (transcriber)', 'You are the TRANSCRIBER. Write artifacts/epic_0/seed-builds.md first (4 SF seeds fully specified, under the constraints in decisions.md §8 and §18, e.g. a non-zero Con modifier on every seed), then artifacts/epic_0/seed-hand-values.md. Every value carries its source URL and section, fetched now, never from recall. An independent reviewer runs after you: leave the E0.4 kanban row as `in-progress` and say so; return complete when your two files are pushed. If the SRD is unreachable: blocked-escalated (SD-c).', true)
const E04R = card('E0.4', 'opus', P.E0, 'Seed builds + hand values from the SRD (independent reviewer)', 'You are the INDEPENDENT REVIEWER of artifacts/epic_0/seed-builds.md and seed-hand-values.md, which another agent just wrote. Re-fetch every cited URL and re-derive every value yourself before you read the transcriber\'s number. Fix what is wrong, and list each correction in the receipt. Then run the E0.4 acceptance command, set the E0.4 kanban row to complete, and write the receipt artifacts/epic_0/E0.4_cycle_receipt.md.', true)
const E03 = card('E0.3', 'sonnet', P.E0, 'SF work inventory (denominator), fail-closed sum', 'E0.1 and E0.2 are pushed; fetch first. Two independent implementations must agree on the unit count.', true)

const E1B = card('E1.1-E1.3', 'opus', P.E1, 'Game-system partition: package roots, per-system book registries, converter system parameter', 'This ONE dispatch covers cards E1.1, E1.2 and E1.3 (batch big). Read all three criterion rows. Write one receipt per card (artifacts/epic_1/E1.1_…, E1.2_…, E1.3_…) and update all three kanban rows. Before any change, record the pre-change PF baseline that E1.4 needs: the render hashes of Aldric (Fighter 3) and Elowen (Wizard 5) and the structural-diff baseline, built from the untouched tree. No Pathfinder behaviour may change.')
const E14 = card('E1.4', 'opus', P.E1, 'PF byte-identical gate (Aldric, Elowen) + structural diff')
const E1MC = card('E1.MC', 'opus', P.E1, 'E1 adversarial merge check', MC)

const MAIN = [
  card('E2.1', 'opus', P.E2, 'Additive schema variants'),
  card('E2.2', 'sonnet', P.E2, 'Published schemas/rules/*.schema.json + rules-schema-check stage'),
  card('E2.MC', 'opus', P.E2, 'E2 adversarial merge check', MC),
  card('E3.1', 'opus', P.E3, 'SF .pcc include structure + game mode (SD-35 C2.1 adopted) + SF-registry licence test'),
  card('E3.2', 'opus', P.E3, 'Formula-system reader (MODIFY / MODIFYOTHER / CHANNEL / DATATABLE)'),
  card('E3.3', 'opus', P.E3, 'SF mapping table, oracle rows, planted mutations M1–M4', 'decisions.md §8 states the HP / Stamina reading as a HYPOTHESIS from the oracle data. Your card confirms or refutes it against the SRD hand values (artifacts/epic_0/seed-hand-values.md). If it is refuted, correct decisions.md §8 and say so in the receipt; do not bend the mapping to fit the text.'),
  card('E3.4', 'opus', P.E3, 'Core Rulebook proof generation'),
  card('E3.5', 'opus', P.E3, 'Go wide: the 7 other in-scope books, one batch'),
  card('E3.MC', 'opus', P.E3, 'E3 adversarial merge check', MC),
  card('E4.1', 'opus', P.E4, 'Generic SF chassis: BAB, saves, HP, Stamina, Resolve, key ability', 'No per-class modules for Starfinder.'),
  card('E4.2', 'opus', P.E4, 'EAC/KAC, initiative, skills, ACP'),
  card('E4.3', 'opus', P.E4, 'Themes, point buy, ability increases'),
  card('E4.4', 'opus', P.E4, 'Spellcasting 0–6'),
  card('E4.5', 'opus', P.E4, 'Credits, bulk, encumbrance', 'You generalise Pathfinder files (money.rs, encumbrance.rs). Re-run the PF hash pair (Aldric, Elowen): it must stay byte-identical.'),
  card('E4.6', 'opus', P.E4, 'StarfinderAdapter; retire stub 0002 for SF'),
  card('E4.MC', 'opus', P.E4, 'E4 adversarial merge check', MC),
  card('E5.1', 'opus', P.E5, 'Races, themes, class features print', 'Print path only. Never edit pilot_compute/**.'),
  card('E5.2', 'opus', P.E5, 'Feats, spells print'),
  card('E5.3', 'opus', P.E5, 'Equipment, augmentations, upgrades, fusions print + total feeds', 'A first attempt landed most of this card and returned blocked-escalated (artifacts/epic_5/E5.3_cycle_receipt.md; progress.md "Open blockers"). Both rulings are now given in decisions.md §20: (a) you MAY edit src/rules_core/pilot_compute/sf_loadout.rs, and only that pilot_compute file, to apply artifacts/epic_5/E5.3-proposed-sf_loadout.patch test-first (update every struct-literal site; re-run E4.5\'s loadout fixtures and the PF hash pair); (b) an installed augmentation adds no bulk: sf_loadout::bulk treats it as 0 and prints its price instead of refusing the character. Read §20, then finish the criterion, update the receipt, and set the E5.3 kanban row to complete.'),
  card('E5.4', 'opus', P.E5, 'Drone print'),
  card('E5.MC', 'opus', P.E5, 'E5 adversarial merge check', MC),
  card('E6.1', 'opus', P.E6, 'System picker routes starfinder-1e to the real adapter; SF data bundled'),
  card('E6.2', 'opus', P.E6, 'SF creation flow'),
  card('E6.3', 'opus', P.E6, 'SF sheet layout, engine-single-source', 'No hand-kept tables in the desktop (R2). Every SF number on the sheet comes from an engine explanation row.'),
  card('E6.4', 'opus', P.E6, 'SF catalogs from data/starfinder-1e/sheet_rules'),
  card('E6.5', 'opus', P.E6, 'SF level-up'),
  card('E6.6', 'sonnet', P.E6, 'Six seeds open in the real app, isolated XDG_DATA_HOME', 'R5: a per-run XDG_DATA_HOME, a guard that refuses the real root, and the real store\'s entry count + sha256 before and after. Long run: wait inside your turn.'),
  card('E6.MC', 'opus', P.E6, 'E6 adversarial merge check', MC + ' Render real builds and open all six seeds in the real app under an isolated XDG_DATA_HOME (R5).'),
  card('E7.1', 'opus', P.E71, 'SF oracle parity roster + "what the oracle does not contain" list'),
  card('E4a.1', 'opus', P.E4a, 'rules_tables data-package format, loader, schema, bundle path, licence/PI stamping'),
  card('E4a.2', 'sonnet', P.E4a, 'Re-point all 252 importers (one dispatch)'),
  card('E4a.3', 'sonnet', P.E4a, '.lst citation burn-down (re-derive SD-36 D6 status first; target 0)'),
  card('E4a.4', 'opus', P.E4a, 'PF parity + Rust table removal'),
  card('E4a.MC', 'opus', P.E4a, 'E4a adversarial merge check', MC),
  card('E7.2', 'sonnet', P.E7, 'Widest-scope verify (root workspace + apps/desktop/src-tauri), baselines', 'ONE full pass. Check `df -h /` first (about 24 G needed). Attribute every `test result: FAILED` line to its `Running` line. A result with green=false and an empty failing list means NOT FINISHED: keep waiting. If a suite is red, return partial and name each failing suite; do not excuse any as environmental.'),
  card('E7.3', 'opus', P.E7, 'Final-acceptance scan (stop if short)', 'Run the fenced scan in workflow-instruction.md §11 step 1 and every FSR revisit check, including DEF-1. If ANY card is short, return partial and name each one with the command output. Then the run stops with no retrospective, no sweep and no PR. That is a correct outcome; do not close a short card yourself.'),
  card('E7.4', 'sonnet', P.E7, 'Retrospective written + cited', 'Also copy the Workflow run id from ~/.claude/projects/-home-ubuntu-workspace-repos-codex/memory/sd37-launch-state.md into the progress.md "Run handle" table.'),
  card('E7.5', 'haiku', P.E7, 'Worktree/branch sweep for this bundle', 'Remove only this bundle\'s own lane trees (' + TREE + '-*) and local sd37/* branches, each only if clean and pushed. Delete /home/ubuntu/workspace/worktrees/cargo-target/sd37-* EXCEPT the main tree\'s directory if a later card still needs it (E7.7/E7.8 do not build; delete it). Never remove ' + TREE + ' itself, /home/ubuntu/workspace/repos/codex, anything under .worktrees/, a locked tree, or the branches test / update-index.'),
  card('E7.6', 'sonnet', P.E7, 'Release notes, every figure re-derived with its command'),
  card('E7.7', 'sonnet', P.E7, 'Architecture truth-up (first half)', 'You run the truth-up (workflow-instruction.md §11 step 5). An Opus claims critic runs after you: leave the E7.7 kanban row as `in-progress` and say so; return complete when the truth-up is pushed.'),
  card('E7.7', 'opus', P.E7, 'Architecture truth-up: claims critic (second half)', 'You are the CLAIMS CRITIC for the architecture docs and release notes that the truth-up just touched. Check capability claims against the live code and instruments, as well as paths and numbers (a floor test is not a ceiling). Fix what is wrong. 0 blockers left = set the E7.7 kanban row to complete and write the receipt.'),
  card('E7.8', 'sonnet', P.E7, 'Graphify LAST over the final tree', 'Follow workflow-instruction.md §11 step 6 exactly. Never pass a force flag. If the node-count guard exits 1: file the receipt and return blocked-escalated.'),
  card('E7.9', 'sonnet', P.E7, 'PR tranche/17 → develop (final action) + wait for pr-tests', 'Open the PR only if every kanban row is complete (re-run the §11 step 1 scan first; if short, return partial and open nothing). End the PR body with these two lines: "🤖 Generated with [Claude Code](https://claude.com/claude-code)" and "' + SESSION + '". Do NOT merge: the operator merges. Wait for the pr-tests run inside your turn; a red run = blocked-escalated (SD-n).'),
]

const results = {}
const handoffs = {}
const KNOWN = new Set(MAIN.map((c) => c.id).concat(['C0.1', 'C1', 'E0.1', 'E0.2', 'E0.3', 'E0.4', 'E1.1', 'E1.2', 'E1.3', 'E1.4', 'E1.MC']))
const passes = (r) => r && (r.status === 'complete' || r.status === 'handed-off')

async function runCard(c) {
  let prev = null
  let dead = 0
  for (let attempt = 1; attempt <= 3; attempt++) {
    const r = await agent(prompt(c, attempt, prev, handoffs[c.id]), { model: c.tier, label: c.id + ' ' + c.tier + (attempt > 1 ? ' try ' + attempt : ''), phase: c.phase, schema: RESULT })
    if (!r) {
      dead++
      if (dead >= 2) return { status: 'dead', summary: 'the agent returned no result twice (usage limit, crash or skip)' }
      prev = { status: 'dead', summary: 'the earlier agent ended with no result; inspect the tree and git log for what it left' }
      continue
    }
    if (r.usage_limit_hit) return { ...r, status: 'quota' }
    if (r.status === 'complete' || r.status === 'blocked-escalated') return r
    if (r.status === 'declined') {
      const owner = (r.owned_by || '').trim()
      if (owner && owner !== c.id && KNOWN.has(owner)) {
        handoffs[owner] = (handoffs[owner] || []).concat([{ from: c.id, what: r.summary }])
        return { ...r, status: 'handed-off' }
      }
      return r
    }
    prev = r // partial: one more agent finishes the remainder
  }
  return prev
}

// Runs cards in order; stops this chain at the first card that does not pass.
async function chain(cards) {
  for (const c of cards) {
    const r = await runCard(c)
    const key = results[c.id] ? c.id + ' (2)' : c.id
    results[key] = r
    log(c.id + ' → ' + r.status + (r.sha ? ' @' + r.sha.slice(0, 10) : ''))
    if (!passes(r)) return { ok: false, card: c.id, status: r.status, detail: r.ruling_needed || r.remainder || r.summary }
  }
  return { ok: true }
}

const report = (stopped) => ({ stopped: stopped, results: results, handoffs: handoffs })

// C1 first, alone: it puts tranche/17 on origin, which every lane tree is cut from.
const c1 = await chain([C1])
if (!c1.ok) return report(c1)

// E0 lanes (own trees) beside E1 (main tree). C0.1 is docs/git only and is not a gate.
const c01 = chain([C01])
const e0 = (async () => {
  const lanes = await parallel([() => chain([E01]), () => chain([E02]), () => chain([E04, E04R])])
  const bad = lanes.find((x) => !x || !x.ok)
  if (lanes[0] && lanes[0].ok && lanes[1] && lanes[1].ok) {
    const e03 = await chain([E03])
    if (!e03.ok) return e03
  }
  return bad === undefined ? { ok: true } : (bad || { ok: false, card: 'E0', status: 'dead', detail: 'an E0 lane threw' })
})()
const e1 = await chain([E1B, E14, E1MC])
const e0r = await e0
const c01r = await c01
if (!c01r.ok) log('C0.1 did not complete (' + c01r.status + '). It is not a gate; the E7.3 scan will hold closure until its row is complete.')
if (!e1.ok) return report(e1)
if (!e0r.ok) return report(e0r)

// One serial chain: E2 → E3 → E4 → E5 → E6 → E7.1 → E4a → E7.2 … E7.9.
const main = await chain(MAIN)
return report(main.ok ? null : main)
