---
canonical: true
owner: operator
bundle_id: SD-37
authored_from: docs/governance/workflow-instruction-template.md (2026-07-22), not from a prior bundle's instance
date: 2026-10-02
---

# SD-37 Workflow Instruction — Starfinder 1e

> ## OPERATING METHOD — REQUIRED FOR THIS BUNDLE
>
> **This bundle runs as a `Workflow`-tool script, invoked from a live session. It is NOT run with
> `/loop /batch` and it is NOT a one-shot task.** It is **one long Workflow run** with a recorded
> run id (`decisions.md §12.2`). §2.4 below says how to write the script. `scope-draft.md` is the
> canonical *what*; this file is the *how*. The orchestrating session never executes RED→GREEN
> work itself (§2.2).
>
> > ### UNATTENDED MODE — the operator is away
> >
> > Operator directive on record (2026-08-01, verbatim): *"include instructions to all 3 that
> > indicate they will be running in unnattended mode since i will be out of town while this
> > runs. They may not stop to ask questions - it might be days before i notice."* This bundle runs
> > under it (`decisions.md §12`; `progress.md` Cycle 0).
> >
> > 1. **Default and flag. Do not ask.** If a choice inside the scope has a safe default in
> >    `decisions.md §12.1`, take it. Log it in `progress.md ## Decisions taken on safe defaults`
> >    with the alternative you did not take, and continue.
> > 2. **Never call `AskUserQuestion` or `clarify`.** Problems surface only in `progress.md`, the
> >    cycle receipts and the end-of-turn status.
> > 3. **Clear a blocker first.** If only an operator ruling can clear it, mark **only that card**
> >    `blocked-escalated`. Write the exact ruling, write scope or precondition needed under
> >    `progress.md ## Open blockers`, with the command and its output. Continue **every
> >    independent card**.
> > 4. **Never open the final PR while any card is open, partial or blocked-escalated.** An
> >    unattended run that ends with "every independent card done, blocked cards listed" has
> >    stopped correctly.
> > 5. **Closure is a goal, not a stop signal.** Chain waves without waiting. Dispatch first,
> >    report second.
> > 6. **Quota stop rule** (`decisions.md §12.3`). Stop dispatching new lanes when an `agent()`
> >    result carries a usage-limit error (`/usage limit|rate limit|quota|limit reached/i` — the
> >    binding, script-evaluable trigger), when reported subagent tokens reach 10 M (estimate; only
> >    if the runtime reports usage), or when the harness shows a weekly reading ≥ 85%. No script
> >    on this box can read the weekly quota itself. Write a resume receipt. **Never downgrade a
> >    model** to keep going. Resumption is not automatic (§12.3).
> > 7. **Crash resume** (`decisions.md §12.4`). Keep the dirty worktrees. Patch only the unrun
> >    prompts. The prefix must be byte-identical to `artifacts/cycle_0/sd37-workflow.js`.
> >    Resume with `resumeFromRunId`. A VM stop needs a new session to run this; nothing restarts
> >    it automatically.

## 0. Bundle at a glance

- **Branch:** `tranche/17`, cut from `origin/develop` `20bf84a3b2` (`decisions.md §2`). It is
  pushed to origin by card C1.
- **Board:** local file `./kanban.md`, paired with `./progress.md`.
- **Cadence:** none. Dispatch is one live `Workflow` run, not a timer loop.
- **Epics / criteria:** 11 epic groups (C, E0, E1, E2, E3, E4, E4a, E5, E6, E7, plus E8 as a
  planned deferral and not a card) / **55 cards**, each with one criterion (`kanban.md` row check).
- **First concrete build value:** develop is at `0.16.0` (pasted in §1 item 7). This bundle's
  repo value is `0.17.0`, stamped by C1. The published triple `0.17.<run>` resolves at the first
  tester publish after C1, which happens only after the operator merges to `develop`
  (`publish-tester-release.yml` triggers on `develop`/`main` pushes only; `decisions.md §2`).
- **Oracle:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`. Every receipt that quotes
  a corpus-derived figure quotes this SHA.

## 1. Pre-launch checklist

Each command was run for real on 2026-10-02 in `/home/ubuntu/workspace/worktrees/codex-sd37`, and
its output is pasted below it. Items still open name the card that closes them.

1. **Board readable** (the local-`kanban.md` equivalent; Hermes retired 2026-08-01).
   ```
   $ test -r docs/release/SD-37-starfinder-1e/kanban.md && echo KANBAN_READABLE
   KANBAN_READABLE
   ```
2. **Bundle branch on origin.** Not yet: this package is committed locally only.
   ```
   $ git ls-remote --heads origin tranche/17
   (no output, exit 0)
   ```
   **Open → closes at C1**, which pushes `tranche/17` with the bump commit.
3. **Predecessor merged.**
   ```
   $ gh pr view 393 --json state,mergedAt
   {"mergedAt":"2026-09-29T16:09:30Z","state":"MERGED"}
   $ git log origin/develop --oneline | head -5
   20bf84a3b2 Merge pull request #394 from electricm0nk/fix/windows-nsis-only
   48ba0856dd fix(release): build Windows NSIS only; WiX light.exe fails on 107k-file bundle
   613257c4a8 Merge pull request #393 from electricm0nk/tranche/16
   5a5e3a5b19 docs(architecture): F6/F7 truth-up — …
   165cc205e7 docs(retro): SD-36 Epic F closure addendum — …
   ```
4. **PAT.** Not applicable. There is no kanban CLI, and `gh` auth is used for the PR only
   (`gh pr view` above succeeded).
5. **Working tree clean.** `git status --porcelain` printed nothing before authoring began. This
   package's own commit restores that state. C0.2 re-ran it on 2026-10-02 at `8b048211ca`:
   ```
   $ git status --porcelain
   (no output)
   ```
6. **Doctrine gates.**
   ```
   $ test -f docs/governance/no-stub-mvp-doctrine.md && test -f docs/doctrine-external/identifier-discipline.md && echo DOCTRINE_PRESENT
   DOCTRINE_PRESENT
   ```
7. **Build counter.**
   ```
   $ grep -m1 '"version"' apps/desktop/package.json; grep -m1 '"version"' apps/desktop/src-tauri/tauri.conf.json
     "version": "0.16.0",
     "version": "0.16.0",
   ```
   develop is at `0.16.0`. This bundle's first concrete value is `0.17.0` (C1).
8. **Artifact directories exist and are empty**, one per epic plus `cycle_0`. Each holds only a
   `.gitkeep`.
   ```
   $ find docs/release/SD-37-starfinder-1e/artifacts -type f ! -name .gitkeep ! -name README.md | wc -l
   0
   ```

## 2. Orchestration mode

- **Dispatch mechanism:** the in-harness `Workflow` tool, invoked from a live session, as one long
  run (`decisions.md §12.2`). There is no `scripts/workflow-dispatch.sh` for this bundle.
  `epic-breakdown.md §0` is the concurrency and tiering source of truth.
- **Tiering** (`decisions.md §11`, from `~/.claude/CLAUDE.md` 2026-09-29; it supersedes the
  template's "Default subagent model: Sonnet"):
  - Opus 5.5 (`'opus'`): build, debugging, review, **every merge check**.
  - Sonnet 5.5 (`'sonnet'`): well-specified mechanical work and every long-run wait.
  - Haiku 4.5 (`'haiku'`): housekeeping (version bump, sweep, PR).
  - Merge checks are never downgraded. Under quota pressure, narrow their scope instead.
- **Concurrency shape:** fixed in §3 at authoring time. At most 3 concurrent cargo lanes.

### 2.1 Agent environment setup — the shared dispatch prefix

Every `agent()` prompt starts with this prefix, **byte-identical** across all agents. Write it to
`artifacts/cycle_0/sd37-workflow.js` at launch, so a crash resume can `diff` against it.

```text
ENVIRONMENT (SD-37, binding):
- export PATH="$HOME/.cargo/bin:$PATH"
- export RETRO_ACTOR=sd37-<card-id>       # the ONLY value you bind, e.g. RETRO_ACTOR=sd37-e3.2
- TREE: the main chain works in /home/ubuntu/workspace/worktrees/codex-sd37 (branch tranche/17).
    A parallel lane (E0.x, C0.1) makes its OWN tree first, from the pushed branch, never with the
    harness's worktree isolation (its base is the session checkout, which is NOT tranche/17):
      git -C /home/ubuntu/workspace/worktrees/codex-sd37 fetch origin tranche/17
      git -C /home/ubuntu/workspace/worktrees/codex-sd37 worktree add -b sd37/<card-id> \
          /home/ubuntu/workspace/worktrees/codex-sd37-<card-id> origin/tranche/17
    The shell's working directory resets between commands: start EVERY command with `cd <your tree> &&`.
    Never write in /home/ubuntu/workspace/repos/codex (another session's checkout).
- export CARGO_TARGET_DIR="/home/ubuntu/workspace/worktrees/cargo-target/sd37-$(basename "$(git rev-parse --show-toplevel)")"
    # one per SOURCE TREE. Serial cards on the main tree share it (one writer at a time; a fresh
    # dir per card would rebuild the whole workspace 45 times). A lane's own tree gets its own dir
    # by basename. Never under /tmp; never shared across trees. A lane deletes its dir with its
    # tree; the main tree's dir is deleted at E7.5. (Launch change, progress.md 2026-10-02.)
- mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"
- GLOSSARY (read the cited row before you start; the package is docs/release/SD-37-starfinder-1e/):
    "CUI F-n" = content-unit-inventory.md §1 row F-n (figure + its command);
    "SD-x" = decisions.md §12.1 safe default x; "§n" with no file = decisions.md §n;
    "R1".."R7", "R-x" = workflow-instruction.md §12; "G-n" = acceptance-and-verification.md;
    "FSR …"/"DEF-1" = forward-scope-register.md; "M1".."M4" = decisions.md §8 mutations;
    "memory <name>" = ~/.claude/projects/-home-ubuntu-workspace-repos-codex/memory/<name>.md;
    seeds = content-unit-inventory.md §4 + artifacts/epic_0/seed-builds.md (E0.4).
    In a table cell, `\|` is Markdown for `|`: unescape before running (epic-breakdown.md §0 note).
- MEMORY GUARD: one cargo process at a time in this lane; cargo build/test with -j 8;
  every `cargo test ... -- --test-threads=8`; run `free -g` before any run expected > 10 min.
- Before a full sweep: `df -h /` — a full sweep needs ~24 G free; if less, stop and report.
- Anything that may run > 10 minutes: `nohup <cmd> > <log> 2>&1 & echo $! > <pidfile>`, write a
  not-run skeleton results file FIRST (every row "not-run"), then poll in a loop inside this turn.
  A partial results file must never read as complete.
- Copy every log or artifact your receipt cites from tmpfs/scratch into the repo
  (artifacts/epic_<n>/) BEFORE you return.
- Wait for slow work INSIDE your turn. You get exactly one turn; nothing re-invokes you.
- Commit and push before ending the turn, even for a partial result (§5 protocol).
- git: `git status --porcelain` before EVERY write; stage by explicit path; never `git add -A`;
  never `git stash`; never force-push.
- First command of the cycle (wrong-base control):
  test -d docs && test -d data && test -d scripts && test -f docs/release/SD-37-starfinder-1e/kanban.md \
    && [ "$(git rev-parse --show-toplevel)" != /home/ubuntu/workspace/repos/codex ] || { echo 'WRONG BASE'; exit 1; }
  # kanban.md exists only on tranche/17, so this fails on a tree cut from any other branch.
- Oracle: resolve via $PCGEN_REPO_DIR / $PCGEN_CORPUS_ROOT (scripts/fetch-pcgen-oracle.sh); never
  write a literal ~/workspace/repos/pcgen path into code or docs. Quote PCGEN_ORACLE_SHA in any
  receipt that quotes a corpus figure.
- CONVERTER LANE: you may change the converter (crates/codex-ingest) if the owning layer of the
  defect is the converter. Every converter change runs the structural-diff protocol (§6 step 5).
  "No converter change" is never the invariant.
- Paper-sheet rule: compute only values that feed a sheet total; print everything else, with every
  term that resolves for this character added into one number.
- UNATTENDED: no AskUserQuestion/clarify. Safe defaults are in decisions.md §12.1. If only an operator
  ruling can clear your blocker, return status blocked-escalated with the exact ruling needed.
- If your premise is refuted, return `declined` and name the mechanism. Do not commit a success.
- Emit scripts/retro.py events (correction/incident/deferral/rework) at the moment they happen.
```

### 2.2 Execution boundary

The session that launches SD-37 is the orchestrator. §6 runs **inside** dispatched `agent()`
calls, never as the orchestrator's own `Edit`/`Write`/`Bash` on `apps/`, `src/`, `crates/`,
`scripts/`, `schemas/` or `data/`. The orchestrator may make only these direct calls: read-only
investigation, edits to this bundle's planning docs, and git plumbing for those docs. When a
cycle's real scope turns out different from the brief, record the corrected scope and re-dispatch;
do not fix it inline.

**Dispatch first, report second.** Never end a turn while ready work is undispatched. Never accept
a lane's message as evidence that its work landed: check `git log` and the target files. Before
every dispatch, run `git reflog --date=iso -3` and `git worktree list`, because a peer session may
hold the checkout (memory `peer-session-may-grab-the-shared-checkout`).

### 2.3 Retrospective event logging

`scripts/retro.py correction --subject … --claimed … --actual … --verified-by …` (`--verified-by`
is required); `incident`, `deferral` and `rework` per `python3 scripts/retro.py help <type>`.
Emit each event at the moment it happens, never batched at the end of the cycle. `RETRO_ACTOR` is
set by the prefix.

### 2.4 Creating the Workflow script

Load the `workflow-authoring` skill before writing the script. The script is plain JavaScript and
has this shape:

1. `export const meta = { name: 'sd37-starfinder-1e', description, phases }`, with one phase per
   §3 row title.
2. `phase()` calls run in §3's gated order. There is one chain. The only concurrency is E0 ∥ E1
   (and C0.1 ∥ C1). E4 and E5 are serial, and E4a runs as a serial block after E7.1 (C0.2
   re-sequencing, `decisions.md §3`).
3. `pipeline()` by default. `parallel()` only where §3 says `yes`, and then every mutating agent
   gets `isolation: 'worktree'`.
4. **Every `agent()` sets `model`** from `epic-breakdown.md §0`'s Tier column. Check:
   `awk '/agent\(/ && !/model:/' <script>` must print nothing.
5. Every prompt = the §2.1 prefix + §6 verbatim + the card's criterion row from
   `epic-breakdown.md` + its "Does not cover" cell + the `kanban.md` row (for "Depends on").
6a. **Quota gate between steps** (`decisions.md §12.3`): before each dispatch, the script checks the
   previous results for a usage-limit error and, if the runtime exposes token usage, the running
   sum. On a hit it dispatches nothing new and returns the resume data.
6. **Step lists carry `ownedBy`** (R4). A `declined` item that names a later step's key does not
   stop the run. That later step owns the item.
7. **Refuted premise stops the step.** If an implementer returns `ok:false` or `declined`, no commit
   reads as a success. The step's gate checks the implementer's status as well as verify-green.
8. Write the run id and script path into `progress.md` the moment the run starts.

```javascript
export const meta = {
  name: 'sd37-starfinder-1e',
  description: 'SD-37 Starfinder 1e — partition, converter, SF chassis, desktop; rules_tables data package in parallel',
  phases: [
    { title: 'C — cycle 0/1' }, { title: 'E0 — oracle + licence' }, { title: 'E1 — partition' },
    { title: 'E2 — schema' }, { title: 'E3 — SF converter' }, { title: 'E4 — SF chassis' },
    { title: 'E5 — SF print path' }, { title: 'E6 — desktop' }, { title: 'E7.1 — SF parity' },
    { title: 'E4a — rules_tables package' }, { title: 'E7 — verify + closure' },
  ],
}
// TIER is read from epic-breakdown.md §0 — never omit model. Write `model: <x>` on the same line
// as `agent(` so the §2.4 item-4 awk check can see it (shorthand `{ model }` fails that check).
const run = (card, model, opts = {}) =>
  agent(PREFIX + PROCEDURE + criterion(card), { model: model, phase: card.phase, ...opts })

// C0.2 ran before launch (complete); it is NOT dispatched here.
phase('C — cycle 0/1')
await parallel([
  () => run(C01, 'haiku', { isolation: 'worktree' }),  // docs/git only; own tree (one writer per tree)
  () => run(C1,  'haiku'),                             // the bump; pushes tranche/17
])

phase('E0 — oracle + licence')        // E0 ∥ E1: disjoint files (§3). Cargo lanes: E0.2 + E1 = 2
const e0 = parallel([
  () => run(E01, 'sonnet', { isolation: 'worktree' }),
  () => run(E02, 'opus',   { isolation: 'worktree' }),
  () => run(E04, 'opus',   { isolation: 'worktree' }).then(() => run(E04_REVIEW, 'opus', { isolation: 'worktree' })),
])
phase('E1 — partition')
await run(E1_batch, 'opus')           // E1.1–E1.3 in one dispatch (batch big)
await run(E14, 'opus'); await run(E1MC, 'opus')
await e0; await run(E03, 'sonnet')

// One serial chain from here (C0.2 re-sequencing): E2 → E3 → E4 → E5 → E6 → E7.1 → E4a → E7.2 … E7.9
// for each epic: for (const c of cards) { quotaGate(); await run(c, c.tier) }
phase('E4a — rules_tables package')   // after E7.1; no other code lane in flight
for (const c of [E4a1, E4a2, E4a3, E4a4, E4aMC]) { quotaGate(); await run(c, c.tier) }
```

### 2.5 A dispatched agent is never resumed — never end a turn waiting

A dispatched `agent()` gets exactly one turn. It waits for slow work inside that turn, with
`nohup` and a poll loop, and scopes its test runs to the binaries its change touches, plus the
workspace suites §6 names. If something will not finish, it reports what it observed, commits
anyway, and pushes. The orchestrator verifies by `git log` and by reading the target files, never
by reading the lane's summary.

## 3. Per-epic parallel/sequential map

The file-touch sets below were checked with `test -e` on 2026-10-02 (§4). Files marked
*(new, proposed)* do not exist yet. The owning card creates each one and names the real path in
its receipt.

| Epic | Criteria | Parallel? | File-touch set (verified) | Gated on |
|---|---|---|---|---|
| C | C0.1, C0.2, C1 | C0.2 complete before launch; C0.1 ∥ C1 (C0.1 in its own worktree) | C0.1: SD-36 docs + git refs only. C1: the 14 surfaces in `decisions.md §2` | C1 after C0.2 |
| E0 | E0.1–E0.4 | yes ∥ E1; E0.3 after E0.1/E0.2 | `scripts/pcgen-oracle-pin.env`, `scripts/fetch-pcgen-oracle.sh`, `scripts/verify.sh` (preflight-oracle stage only), `docs/governance/license-matrix.md`, `docs/governance/ogl-pi-blacklist.md`, `src/rules_core/pi_screening.rs` (SF term set; **E0.2 only**), `artifacts/epic_0/**`, `docs/work-inventory.starfinder-1e.json` *(new, proposed)* | C1 |
| E1 | E1.1–E1.4, E1.MC | no (one batch) | `src/rules_core/corpus_loader.rs`, `apps/desktop/src-tauri/src/character_hub.rs`, `apps/desktop/src-tauri/src/authoring_workbench.rs`, the 18 files holding the 19 BOOKS consts (CUI F-13), `crates/codex-ingest/src/pcgen_import/sheet_rule/closure.rs`, `…/sheet_rule/reprint*`, `crates/codex-ingest/src/bin/sheet_rule_convert.rs`, `src/rules_core/game_system.rs` *(new, proposed)*, the PF render-hash harness *(new, E1.4)* | C1 |
| E2 | E2.1, E2.2, E2.MC | no | `src/rules_core/sheet_rule.rs`, `schemas/rules/*.schema.json` *(new)*, `scripts/verify.sh` (new stage) | E1.MC |
| E3 | E3.1–E3.5, E3.MC | no | `crates/codex-ingest/**` (converter), `scripts/token_coverage.py`, `scripts/pcgen_residue_gate.py`, `scripts/verify.sh` (SF stages), `data/starfinder-1e/**` *(new)*, `tests/sf_license_registry.rs` *(new, E3.1)*, `artifacts/epic_3/**` | E2.MC, E0.1, E0.2 (E3.1), E0.4 (E3.3), E0.3 (E3.4) |
| E4 | E4.1–E4.6, E4.MC | no | `src/rules_core/pilot_compute/sf_*.rs` *(new)*, `src/rules_core/pilot_compute/mod.rs`, `src/rules_core/{money.rs,encumbrance.rs}`, `apps/desktop/src-tauri/src/{rule_system_adapter.rs,stub_adapter.rs}`, `apps/desktop/src-tauri/src/sf_adapter.rs` *(new)*, `docs/governance/wired-integration-stubs-registry.md`, the converter (CONVERTER LANE) | E3.MC |
| E5 | E5.1–E5.4, E5.MC | no (after E4.MC — C0.2) | `data/starfinder-1e/**` (regenerated by converter), `crates/codex-ingest/**` (prose), SF print-path files *(new)*. **Never** `pilot_compute/**`, except `pilot_compute/sf_loadout.rs` for E5.3 only (`decisions.md §20`, after E4 completed) | E4.MC |
| E6 | E6.1–E6.6, E6.MC | no | `apps/desktop/src/characterHub/**`, `apps/desktop/src/**/CharacterSheet*.tsx`, `apps/desktop/src/characterHub/abilityScoreMethods.ts`, `apps/desktop/src-tauri/src/*catalog*.rs` (SF catalogs as **new files**), `apps/desktop/src-tauri/tauri.conf.json`, `apps/desktop/scripts/ui-smoke/spec.json` | E4.MC, E5.MC |
| E7.1 | E7.1 | no | `scripts/oracle_harness/**`, `scripts/pcgen-run-character.sh`, and any SF engine file a parity fix needs | E6.MC |
| E4a | E4a.1–E4a.4, E4a.MC | **no — serial, after E7.1, nothing else in flight** (C0.2) | `src/rules_core/rules_tables/**`, `src/rules_core/rules_data_package.rs` *(new, proposed)*, `schemas/rules/rules_tables.schema.json` *(new)*, the 252 importers (CUI F-12), `data/rules_tables/**` *(new, proposed)*, `apps/desktop/src-tauri/tauri.conf.json` (resources), `scripts/verify.sh` if a stage needs the new path | E7.1 |
| E7.2–E7.9 | E7.2–E7.9 | no | `scripts/verify-baselines.env`, `docs/retro/sd37-retrospective.md`, `docs/architecture/**`, graph outputs, this folder | E4a.MC |

**File-level fences (R-F) — C0.2 rewrite.** The authoring fences let E4a run beside E2–E6 and
E4 beside E5. C0.2 measured the overlap (`decisions.md §3`: E4a.2's 252 importers include
`sheet_rule.rs`, `corpus_loader.rs`, `character_hub.rs`, `encumbrance.rs`, `pilot_compute/mod.rs`,
23 `pcgen_import/` files, 6 BOOKS-const files and 9 desktop `*catalog*.rs` files; E4 ∥ E5 shared the converter
and `data/starfinder-1e/**`) and removed the concurrency instead of adding fences. The only
concurrent windows left are:
- **E0 ∥ E1.** E0 touches only the E0 row's files; E1 touches only the E1 row's. Disjoint — verified
  by C0.2: no E0 path appears in the E1 row and vice versa. E0 lanes never touch
  `crates/codex-ingest/**` (the CONVERTER LANE does not apply to E0).
- **C0.1 ∥ C1.** C0.1 touches SD-36 docs and git refs in its own worktree; C1 touches the 14
  version surfaces.

A lane that needs a file outside its row returns `declined` with `ownedBy:<card>` naming the card
whose row holds the file.

The command C0.2 used for the importer overlap (re-run it if the epic order changes again):

```bash
# list the 252 importers, then intersect with each epic's file set by hand or script
for r in src crates apps/desktop/src-tauri tests; do grep -rlE 'rules_tables::' $r --include='*.rs' | awk '!/src\/rules_core\/rules_tables\//'; done | sort > "$SCRATCH/importers.txt"   # SCRATCH = any scratch dir outside the repo
awk '/corpus_loader|sheet_rule\.rs|character_hub|encumbrance|money\.rs|pilot_compute\/mod\.rs|pcgen_import\/|catalog/' "$SCRATCH/importers.txt"
```

## 4. File-touch verification

Each existing path named in §3, `technical-design.md` and `content-unit-inventory.md` was checked
with `test -e` on 2026-10-02. All exist except one: `src/money.rs` does not exist, and the real
file is `src/rules_core/money.rs` (corrected everywhere). The doc paths
`docs/release/SD-34-corpus-sheet-completion` (wrong in SD-36 FS-2) → the real path is
`docs/release/SD-34-book-completion`. Paths marked *(new, proposed)* are design choices that their
owning card confirms. They are not claims about the repo.

## 5. Concurrent-write protocol

```bash
git fetch origin tranche/17 && git rebase origin/tranche/17 && git push origin HEAD:tranche/17
```

On a non-fast-forward rejection, retry up to 5 times. If it still fails, report `CLAIM-EXISTS` and
mark the card `blocked-escalated`. Never force-push. `progress.md` and `kanban.md` are re-fetched
and re-read immediately before every edit. A rebase **conflict** (not a rejection) follows SD-o:
conflicts confined to `kanban.md`/`progress.md` rows are resolved by keeping both sides; any other
conflict aborts the rebase and the card is `blocked-escalated` with the paths.

**Every card, including worktree lanes, lands by pushing its own commits to `tranche/17` with this
protocol** (C0.2 correction: the authoring text also said worktree lanes merge "through their
merge-check card, never directly from the lane", which contradicted the prefix's "commit and push
before ending the turn" and named no mechanism). Merge-check cards re-verify on
`origin/tranche/17`; they do not merge anything.

## 6. Per-cycle procedure (runs inside the dispatched agent)

1. **Base check** (a control, not a warning): run the wrong-base test from the prefix. On failure,
   `git reset --hard <pinned tranche/17 SHA from progress.md>` and re-check. Then run the §5
   fetch+rebase, and record `CARD_BASE=$(git rev-parse HEAD)` (the receipt's "Base SHA").
2. **Baseline.** Record the card's acceptance command output on the **pre-change** tree (expected
   RED). That output goes in the receipt.
3. **TDD.** Write the failing test and confirm it fails **for the intended reason**. Make the
   smallest change, then run the targeted suites.
4. **Dual audit on the final diff** (every check must print its `OK_*`). Run it **after** the local
   commit of step 9 and **before** the push: `...HEAD` sees committed changes only, so an audit run
   on uncommitted work audits nothing (C0.2 correction). Only **added** lines are audited: the
   `awk` filter keeps `+` lines, so removing a stub (E4.6 deletes `"Would render …"` strings from
   `stub_adapter.rs`) does not trip the audit it satisfies (C0.2 correction). A violation is fixed
   with a new commit and the audit re-run; push only when all print `OK_*`.
   ```bash
   git diff --unified=0 "${CARD_BASE}...HEAD" -- <scoped paths> ':!**/__tests__/**' ':!**/*.test.*' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
   # Four-check wired-integration audit — docs/governance/no-stub-mvp-doctrine.md §"Per-cycle audit"
   git diff --unified=0 "${CARD_BASE}...HEAD" -- 'apps/desktop/**/*.ts*' 'apps/desktop/src-tauri/**/*.rs' 'src/**/*.rs' 'crates/**/*.rs' ':!**/__tests__/**' ':!**/*.test.ts' ':!**/*.test.rs' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo OK_NO_TOKENS
   git diff --unified=0 "${CARD_BASE}...HEAD" -- 'apps/desktop/**/*.tsx' 'apps/desktop/**/*.jsx' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE 'onClick=\{\s*\(\)\s*=>\s*\{\s*\}\s*\}|onClick=\{undefined' || echo OK_NO_NOOP_HANDLERS
   git diff --unified=0 "${CARD_BASE}...HEAD" -- 'apps/desktop/**/*.ts' 'apps/desktop/**/*.tsx' 'apps/desktop/**/*.jsx' 'apps/desktop/**/*.rs' ':!**/__tests__/**' ':!**/*.test.*' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE 'mockResolvedValue|mockReturnValue\(|vi\.mock\(|__mocks__' || echo OK_NO_MOCK_LEAKS
   git diff --unified=0 "${CARD_BASE}...HEAD" -- 'apps/desktop/**/*.ts' 'apps/desktop/**/*.tsx' 'src/**/*.rs' \
     | awk '/^\+/ && !/^\+\+\+/' | grep -nE '"Would [^"]*"' || echo OK_NO_WOULD_STRINGS
   ```
   (`crates/**` is added to check 1 relative to the doctrine's list, because E3/E4a ship code
   there.)
5. **Converter cycles: structural-diff protocol** (`docs/release/SD-36-consolidation/decisions.md
   §11.1`; script `docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py`):
   pinned delta classes; planted mutations that **must fail**; records unmoved where unmoved is the
   claim (PF 49,450); `cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --check`
   exit 0; `python3 scripts/pcgen_residue_gate.py --check --closure` exit 0.
6. **Seed deltas** (R7): render every seed your change can reach, and record each seed's changed
   lines (old → new) in the receipt.
7. **Verify once at the widest scope your change reaches.** Root `cargo test --locked -j 8
   --no-fail-fast -- --test-threads=8` and/or `apps/desktop/src-tauri` `cargo test`, plus the
   frontend if touched. Attribute every `test result: FAILED` to its `Running` line. Do not repeat
   a clean gate (R-V).
8. **Receipt** to `artifacts/epic_<n>/<card>_cycle_receipt.md` (§7).
9. **Commit** (explicit paths). Update the card's `kanban.md` row and append a `progress.md` row in
   the same commit. **Clean-tree check:** unfiltered `git status --porcelain` must print nothing
   after the commit. Push via §5.
10. **Report:** status (`complete | partial | blocked-escalated`), SHA, receipt path, seed deltas,
    and discoveries.

## 7. Per-cycle receipt schema

The template's `complete | returned-to-backlog | DISCOVERED-forked` statuses are replaced by the
chassis three-status vocabulary `complete | partial | blocked-escalated` (`decisions.md §14`). A
discovery is a new `kanban.md` card, not a status.

```markdown
# Cycle <card-id> — <epic> / <criterion title>

- **Card ID:** <card-id>   **Model:** <opus|sonnet|haiku>   **RETRO_ACTOR:** <value>
- **Commit SHA:** <sha>    **Base SHA:** <tranche/17 sha at start>   **Oracle SHA:** 7f818006e3…
- **Files touched:** <list>
- **Acceptance criterion (verbatim from epic-breakdown.md):** <text>
- **RED (pre-change command + output):** <…>
- **GREEN (same command + output):** <…>
- **Figures:** <value> — <predicate/denominator> — <command A> → <output>; <command B> → <output>
- **Raw row-count output:** <pasted, not summarised>
- **Build scope verified:** <root lib | root full | codex-ingest | apps/desktop/src-tauri | frontend> + log path in repo
- **Identifier audit:** OK_NO_BUNDLE_TAGS | <violations>
- **Wired-integration audit:** OK_NO_TOKENS / OK_NO_NOOP_HANDLERS / OK_NO_MOCK_LEAKS / OK_NO_WOULD_STRINGS | <violations>
- **Structural diff (converter cycles):** verdict, delta classes, planted mutations → FAIL count, records moved
- **Seed deltas:** Aldric / Elowen / SF-Soldier-3 / SF-Mystic-5 / SF-Technomancer-5 / SF-Envoy-3: <line: old → new> or "unchanged (rendered, hash <h>)"
- **Does not cover:** <the shapes this proof does not exercise>
- **Status:** complete | partial | blocked-escalated
- **Safe defaults taken:** <SD-x + alternative not taken>
- **Retro events emitted:** <types + counts>
- **Next-cycle plan:** <…>
```

## 8. Self-heal posture

- **Self-healable (fix inline, then continue):** dirty tree from your own lane; a single-token
  audit violation; unrelated test-setup breakage; a stale count floor that has a re-derive command;
  build-counter drift.
- **Not self-healable (`blocked-escalated` on THIS card only; continue the independent cards):** a
  launch-gate dependency that has not merged; two live lanes on one fenced file; RED→GREEN not
  preserved; `success: true` from a fake operation, an inline mock or a "Would …" string in
  shipping code; a licence question E0.2 cannot settle; an unreachable SRD (SD-c);
  `CLAIM-EXISTS`.
- **Blocker doctrine** (`docs/governance/blocker-closure-doctrine.md`): a blocker on the Definition
  of Done is **cleared** or **escalated**. It is never deferred and never handed to a successor
  bundle. "Filed with a named owner", "forwarded", "deferred with reason" and "out of scope for this
  cycle" are not dispositions. The deferral test: *was this scope in the Definition of Done at
  scoping?* If yes, it is a blocker.
- **Disk:** after every parallel wave, run `df -h /` and `git worktree list`. Prune merged,
  unlocked, clean worktrees and delete their `CARGO_TARGET_DIR`s. Never remove a `locked` tree.

## 9. Placeholder-resolution checklist

```bash
grep -rn '<[a-z_-]*>' docs/release/SD-37-starfinder-1e/*.md
```

Every match must be one of three things:

- (a) a schema placeholder inside a fenced receipt/prefix template in this file (§2.1, §7) or in
  `progress.md`'s row template; or a command argument that the running card binds at run time
  (`<fresh>`, `<sha>`, `<bump-sha>`, `<render>`, `<script>`, `<file>`, `<pcc>`, `<tree>`,
  `<checkout>`, `<branch>`, `<p>`, `<role>`, `<n>`, `<cycle>`, `<Class>`, `<card>`, `<card-id>`,
  `<type>`, `<epic-start>`, `<cmd>`, `<log>`, `<pidfile>`, `<seed>`, `<field>`, `<value>`,
  `<name>`, `<scratch>`, `<x>`), or a naming pattern (`SD-NN-<slug>`, `stc-<##>-<description>`);
- (b) a deferred value with a named resolution point:
  - `0.17.<run>` → first tester publish after the operator merges to `develop` (not a card);
  - the SF denominator → E0.3;
  - hand values → E0.4;
  - the data-package path → E4a.1;
  - the SF mapping-table path → E3.3;
  - the Workflow run id → launch;
- (c) a bug to fix before launch.

C0.2 ran the command on 2026-10-02 and classified every match in
`artifacts/cycle_0/C0.2_cycle_receipt.md` (§"Placeholder gate"). `<repo-parent>` and `<tree-name>`
were class (c) — undefined in the prefix — and were replaced by a computed path (§2.1).

## 10. Epic wrap-up (after every epic)

1. `scripts/retro.py summary --since <epic-start> --json`. Read it, and fold the counts and any key
   that fires more than once into the epic's merge-check receipt.
2. The merge-check card updates `kanban.md` and `progress.md` **in its own closing commit**
   (SD-36 "Changes" row 1). A forgotten row is a closure-scan failure.
3. Sweep this epic's own merged, unlocked, clean worktrees and delete their `CARGO_TARGET_DIR`s.
4. No PR.

## 11. Bundle closure epilogue (corrected order — `decisions.md §13`)

This order **differs from the template's §11**. The template puts release notes after graphify,
and that is a known defect (FSR-C9).

1. **E7.3 final-acceptance scan.** Every card except the closure chain E7.3–E7.9 is `complete`:
   ```bash
   cd docs/release/SD-37-starfinder-1e
   test -s kanban.md || { echo NO_KANBAN; exit 2; }
   awk -F'|' '$2 ~ /^ (C|E)[0-9]/ { n++ } END { if (n != 55) print "ROW_COUNT " n }' kanban.md
   awk -F'|' '$2 ~ /^ (C|E)[0-9]/ && $2 !~ /^ E7\.[3-9] / && $5 !~ /^ complete *$/ { print $2 "|" $5 }' kanban.md
   ```
   Pass = no output. Every FSR revisit condition is checked, including DEF-1 (`decisions.md §17`'s
   fenced command; exit 2 = could not check = short). **If anything is short, stop here:** no
   retrospective, no sweep, **no PR**. Report what is short with the command that shows it. This is
   a correct outcome.
2. **E7.4 retrospective.** Run `scripts/retro.py summary --since 2026-10-02 --json`, then write
   `docs/retro/sd37-retrospective.md` in the shape of `docs/retro/sd31-retrospective.md`, and
   **cite it from `references/README.md`** in the same commit.
3. **E7.5 full worktree/branch sweep.** Report counts found vs removed. Never remove `test` or
   `update-index`, a locked tree, or another session's tree.
4. **E7.6 release notes** (`release-notes.md`), with every figure re-derived and its command
   attached.
5. **E7.7 architecture truth-up** (`~/.hermes/profiles/god-emporer/skills/devops/architecture-truth-up/scripts/architecture_truth_up.py --integration-target develop --receipts-md docs/release/SD-37-starfinder-1e/receipts.md --bundle SD-37`),
   then an Opus claims critic. The critic checks capability claims as well as paths and numbers.
6. **E7.8 graphify LAST.** Precondition: unfiltered `git status --porcelain` prints nothing, and
   `git rev-parse HEAD` = `git rev-parse origin/tranche/17`. Run
   `~/.hermes/profiles/god-emporer/skills/devops/graphify-update/scripts/update_graphify.py --integration-target develop --receipts-md docs/release/SD-37-starfinder-1e/receipts.md --bundle SD-37`.
   Record the indexed SHA. If the node-count guard exits 1, file the receipt and stop. **Never**
   pass a force flag (memory `graphify-force-update-replaces-semantic-graph`). After graphify,
   commit **only** the graph outputs and `receipts.md`, push, and confirm the tree is clean again.
7. **E7.9 PR** `tranche/17 → develop`. This is the final action. Before opening it: re-run the
   step-1 scan with the exemption narrowed to `E7\.9`; commit E7.9's own `complete` row and
   progress row (docs only) and push; read any named, pre-scoped merge-conflict deferral
   (expected: `publish-tester-release.yml` vs PR #395). Then `gh pr create`, and wait inside the
   turn for the PR's `pr-tests` run (`gh pr checks <n> --watch`) — the only CI that runs for this
   branch (SD-n). Red or unopenable → one follow-up docs commit setting E7.9 `blocked-escalated`
   with the failing job named. The operator merges.
8. Stop.

## 12. Standing rules

### 12.1 SD-36 Epic F lessons as checkable rules (R1–R7, `decisions.md §15`)

| Rule | Statement | Checked by |
|---|---|---|
| **R1 Converter lane in every prefix** | Every prefix carries the CONVERTER LANE paragraph (§2.1). Any converter change runs the structural-diff protocol (§6 step 5). | `grep -c 'CONVERTER LANE' artifacts/cycle_0/sd37-workflow.js` = 1; converter receipts carry `verdict=PASS` + planted-mutation FAIL counts |
| **R2 No hand-kept desktop tables** | The engine is the single source. A fallback appears only behind a visible notice. Merge checks render real builds on both trees and open the seeds in the real app. `Computed` is a floor, never the claim. | E6.3's test (every SF sheet number traces to an engine explanation row); E6.MC receipt |
| **R3 Merge checks stay on Opus** | Every `*.MC` card is `opus`. Under quota pressure narrow the scope, never the model. | `awk -F'\|' '$2 ~ /MC/ && $4 !~ /opus/' kanban.md` prints nothing (unescape `\|` first; C0.2 ran it: no output) |
| **R4 `ownedBy` declines** | A declined item that names a later step's key goes to that step and does not stop the run. | Script review at launch (the script does not exist at C0.2): every step list item has `ownedBy` |
| **R5 Isolated app-data root** | Any harness that launches the app uses a per-run `XDG_DATA_HOME`, refuses the real root, and records the real store's entry count + sha256 before and after. | E6.6/E6.MC receipts |
| **R6 Long waits on Sonnet+; `green:false` + empty failing = not finished** | No Haiku on runs that may exceed 20 min. Cited logs copied into the repo before return. Crash recovery per `decisions.md §12.4`. | `kanban.md` Tier column for E6.6/E7.2/E7.8/E7.9 ≠ haiku (E7.9 waits for `pr-tests`); receipts cite repo paths |
| **R7 Per-character status** | Every status message and every receipt reports deltas for the 6 seeds, not census totals alone. | Receipt field "Seed deltas" present: `grep -L 'Seed deltas' artifacts/epic_*/*_cycle_receipt.md` prints nothing |

### 12.2 Per-cycle and measurement rules

- **R-T TDD** (AGENTS.md rule 1). RED is confirmed for the intended reason.
- **R-B Batch big.** All of a homogeneous remainder goes in one dispatch (E1.1–E1.3; E3.5's seven
  books; E4a.2's 252 importers).
- **R-V One verify per wave.** All changes first, then ONE full verify. Re-run only if something
  changed or a spot-check contradicts the result.
- **R-C Clean tree = unfiltered `git status --porcelain` empty** at every wave commit. Fold dispatch
  scripts and retro logs into the commit.
- **R-N Every figure carries its command and its denominator.** A ratio carries its predicate.
  Counts come from `awk` or Python, never `grep -o`. A number that moves a baseline needs two
  independent implementations that agree.
- **R-P Every proof states what it does not cover** (AGENTS.md rule 7): the "Does not cover" field
  in every receipt.
- **R-W A warning is not a control.** Any incident key that fires more than twice in
  `retro.py summary` gets a non-zero-exit check, in the next card that touches that area.
- **R-S Sum the piles.** The SF denominator (E0.3) and every partition sum through a fail-closed
  command.
- **R-F File-level fences** (§3). Never directory globs.
- **R-G Widest build scope.** Root workspace **and** `apps/desktop/src-tauri` (a separate
  workspace). `cargo build --lib` green is not a completed phase.
- **R-X Cross-tree renders.** The sheet-rule path is baked at compile time
  (`corpus_loader.rs:360`), so a `cd` into another worktree renders the wrong data. Build per tree,
  or swap at the baked path.
- **R-Q Quota.** `decisions.md §12.3`.

### 12.3 Template standing lessons (carried)

- A blocker on the Definition of Done is cleared or escalated, never deferred.
- A deferral names a revisit condition that is **checked** (DEF-1's command runs at E7.3).
- A headline figure written before the wave that establishes it is provisional, and the text says
  "estimated" (for example the SF denominator until E0.3).
- Recurring incidents get a mechanical control.
- Measurement work that banks zero units is still a deliverable.

## Cross-references

- `docs/governance/workflow-instruction-template.md` (source template, whose §11 order is defective
  → FSR-C9), `docs/release/template/template.md`.
- `docs/governance/blocker-closure-doctrine.md`, `docs/governance/deferral-revisit-doctrine.md`,
  `docs/governance/no-stub-mvp-doctrine.md`, `docs/governance/wired-integration-stubs-registry.md`.
- `~/.claude/skills/stc-bundle-authoring/references/unattended-mode-doctrine.md` (reconciled in
  `decisions.md §12`).
- `docs/retro/sd36-retrospective.md` lessons 14–20 and "Changes for the next bundle".
