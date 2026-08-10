# SD-29 — Loop Instruction

> ⚠️ **OPERATING METHOD — REQUIRED FOR THIS BUNDLE**
> This file is the operational loop-instruction for SD-29. The bundle is operated via:
>
> **Dispatch mechanism: the in-harness `Workflow` tool, driven from a live session** — not a headless `/loop` script and not a cron driver. Deterministic control flow (per-epic ordering, fan-out, `decision-blocked` handling) lives in this document and in `kanban.md`'s claim/complete state; model judgment lives inside the dispatched `agent()`/`Workflow` calls. Per `decisions.md §23` (adopted from SD-27 `decisions.md §19`, itself adopted from SD-26 `decisions.md §13`) and `docs/governance/loop-instruction-template.md §2`.
>
> `/batch` is **not** the default concurrency primitive for this bundle. `/batch` fans out into parallel isolated worktrees by default; SD-29's cycles mutate shared state on nearly every cycle (`progress.md`, `kanban.md`, `reach_gate.rs`'s `OPEN_FINDINGS`), so parallel dispatch is the exception, called out explicitly per-epic (see "Epic ordering" below), not the default. Any parallel wave dispatches each agent with `isolation: 'worktree'` (`loop-instruction-template.md §3`); a shared-checkout wave with more than one mutating agent and no worktree isolation is not a valid dispatch. Where cycles touch shared state, dispatch an explicit single-cycle procedure instead of reaching for `/batch`.
>
> Every dispatched agent gets `RETRO_ACTOR=<role-name>` set in its environment (`loop-instruction-template.md §2.1`) — no harness variable identifies an agent's role, and the fallback (worktree directory name) names a checkout, not a role, which makes the retrospective log's by-actor breakdown meaningless.
>
> The orchestrating session never implements directly — it dispatches, verifies, and rules (`loop-instruction-template.md §2.2`). Do NOT engage this bundle via ad-hoc single-task invocations; one Workflow-tool launch runs to closure.
>
> **Orchestrator model: Opus, low reasoning effort** (operator directive 2026-08-01, `decisions.md §26`) — Opus at low reasoning effort produced materially better orchestration results than Sonnet at high reasoning effort, and is the new normal for the *orchestrating session* on this program. This supersedes any prior "orchestration runs on Sonnet" guidance (none existed in this package before this pass). Dispatched sub-agents are unaffected — they keep task-matched tiers (Haiku for housekeeping, Sonnet for real implementation/debugging/review, Opus for adversarial verification/judge-panel steps only) per `loop-instruction-template.md §2`. A session cannot change its own model mid-run: setting Opus-low is a **pre-launch operator step**, done before this cycle session starts.
>
> **🟡 UNATTENDED MODE (operator directive 2026-08-01).** The operator is out of town. Cycles MUST NOT pause to ask the operator questions; the operator may not see the harness's output for days. The operating protocol for the duration of the bundle is:
>
> 1. **Default-and-flag, not ask.** When the cycle needs a decision, pick the safer default, capture it in the cycle's `progress.md` receipt, and continue. The operator reviews the receipts after return.
> 2. **No `clarify` tool calls.** Cycles must not invoke the operator clarification tool under any circumstance; this is a hard ban during unattended mode.
> 3. **Blockers are recorded, not raised.** If a cycle hits a true hard-block (auth failed, branch can't be created, identity conflict on disk), record the blocker in `progress.md` with the command and exit code, then attempt the next ready card per `kanban.md`. Do not halt the bundle.
> 4. **`decision-blocked` IS allowed.** Where the playbook calls for an operator decision (Epic 8 DM Toolkit extension in-scope-vs-separate), record `decision-blocked` in `progress.md` with the recorded reason and proceed on the safe default per `successor-forward-scope-register.md C3.1` retrofit. Do not wait. See also "Stop vs. press on" below for the general rule this instance follows.
> 5. **Closure is a goal, not a stop signal.** The bundle runs to closure under the Workflow tool's own dispatch loop, not a human re-invoking a slash command per cycle. The operator's review happens after return; cycles do not pause for operator review.
> 6. **Operator's verbatim:** "include instructions to all 3 that indicate they will be running in unnattended mode since i will be out of town while this runs. They may not stop to ask questions - it might be days before i notice."

## Pre-launch checklist (must be true before any cycle fires)

1. **`kanban.md` exists and lists a ready queue.** (Operator-pinned 2026-08-01: Hermes board retired; work-queue artifact is `kanban.md` paired with `progress.md` inside this directory.)
2. **Branch pushed:** `tranche/9` is pushed to origin (`git push -u origin tranche/9`). (Operator-pinned 2026-08-01; cut from the post-SD-28-closure tip per decisions.md §34; the SD-29 launch session performs the cut+push.)
3. **OAuth credentials valid:** the active harness has fresh GitHub OAuth credentials for `git push` operations to origin.
4. **Working tree clean:** no uncommitted work-in-progress from a prior bundle. Run `git status` from the repo root.
5. **Sequential launch order:** SD-28 has reached closure on tranche/8 (its `progress.md` closure receipt exists). SD-29 does not launch concurrently with SD-28 on a shared checkout.

If any of these is false, the cycle refuses to launch and reports the gap.

## Cycle mechanics

Each cycle follows the SD-22 cycle shape, with the repo's ingestion tooling
inserted at the two points where this project has repeatedly shipped defects —
before any ingest code is written, and at verification. The full procedure is
`docs/governance/book-ingestion-playbook.md`; read it before the first cycle of
each book.

0. **Shape** the book. `cargo run --locked --bin v06_work_inventory`, then read
   the book's `books[]` entry in `docs/work-inventory.json` — `kinds`,
   `files_not_enumerated`, `trap_hits`, `reconciliation`. The shape decides the
   cycle; do not assume a template, and do not assume a bestiary contains
   monsters (Bestiary 5's and Bestiary 6's do not; Monster Codex carries only
   2 — its weight is class features, feats, spells, and equipment). Done once
   per book, not once per cycle.
0b. **Trap-report** the book, before writing a line of ingest code:
   `cargo run --locked --bin v06_corpus_trap_report -- <book_dir>`. Record the
   output in the cycle receipt. See `decisions.md` Decision 9.
1. **Read** the doctrine-of-record (`scope-draft.md`, `decisions.md`, current `progress.md`).
1b. **Re-derive.** Before accepting any figure carried in a brief, a doc, or a
   prior cycle's `progress.md` entry — including this package's own
   `scope-draft.md` and `decisions.md` — re-derive it yourself with a
   one-line `grep`/`awk`/`python3` command over the actual source data (the
   PCGen `.lst` tree under `~/workspace/repos/pcgen/data/` for anything not
   yet ingested, `data/corpus/<book>/` for anything that is), and record the
   exact command in the cycle receipt. Do not transcribe a count from a doc,
   a summary tool, or memory of a prior cycle. Worked example, this bundle's
   own data: `awk '!/^#/ && !/^SOURCELONG/ && NF>0' ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/bestiary_2/b2_races.lst | wc -l`
   → **322** (re-derived 2026-08-01), the number to cite for "Bestiary 2
   monster/race count," not a remembered or copied-forward estimate. This is
   the tranche/7 retrospective's rank-1 finding, re-run against the current
   log rather than transcribed: applying the retrospective's own classifier
   (`docs/retro/tranche-7-retrospective.md` §3's regex over each
   correction's `verified_by` field) to the live log shows ad-hoc commands
   over source data catching **50%** of all logged corrections (69 of 138
   correction events currently in `docs/retro/events/*.jsonl`, re-derived
   2026-08-01 with:
   `python3 -c "import json,glob,re; p=re.compile(r'\b(grep|rg|awk|sed|wc -l|python3|find |sort -u|uniq|Counter|jq)\b'); evs=[json.loads(l) for f in glob.glob('docs/retro/events/*.jsonl') for l in open(f) if l.strip()]; c=[e for e in evs if e.get('type')=='correction']; print(sum(1 for e in c if p.search(e.get('verified_by',''))), len(c))"`
   → `69 138`; the retrospective's own published snapshot was 46% of 115 —
   the log has kept growing across every bundle that inherited the practice
   since tranche/7, not just from tranche/7 itself, which is itself the
   lesson: re-derive at the point of use rather than transcribe) and remain
   the single strongest detector by a wide margin — more than
   `./scripts/verify.sh` (8%, Cycle mechanics step 4 below), on-screen
   driving (14%, Definition of done item 8 below), and every repo test
   combined. See `docs/retro/tranche-7-retrospective.md` §3 and §0 for the
   original reproduction command.
1c. **Preflight** the disk. `./scripts/verify.sh --only preflight-disk` (fast —
   no build). Refuse to start the bounded work below if it fails; run
   `scripts/reclaim.sh` (no flags — dry run) to see what it would reclaim,
   then `scripts/reclaim.sh --apply` and re-check. Disk exhaustion is this
   program's second-largest recorded orchestration failure mode
   (`docs/retro/tranche-7-retrospective.md` §4.1, 5 of 34 incidents,
   including `/home` at 100% used, 0 bytes available) and a ~490-binary
   `root-full` build (cycle mechanics step 4 below) is exactly what tips a
   box over. See `decisions.md` Decision 33.
2. **Claim** the highest-priority ready card on `kanban.md` per its claim/complete protocol.
3. **Do** the bounded work (TDD per the repo's `AGENTS.md`: failing test → smallest change → green → refactor). **The player surface is part of the bounded work, not a follow-on** — see `decisions.md` Decision 10.
4. **Verify** with `./scripts/verify.sh` (full, not `--quick`), exit code captured
   directly and never through a pipe. Do not compose a substitute command set;
   `cargo test --workspace --locked` from the repo root does not reach
   `apps/desktop/src-tauri` at all. See `decisions.md` Decision 8.
5. **Commit** with a `feat(sd29): ...` or `fix(sd29): ...` prefix.
6. **Append** the cycle record directly to `progress.md` (no Hermes release —
   the board is retired). The cycle record carries the PR-id, branch-tip,
   per-cycle test result, and the command behind every figure it publishes.
   The supervisor reads `kanban.md` at top of the next cycle to find the next
   ready card.
7. **Emit** a retro event for anything this cycle corrected, deferred, reworked,
   or narrowly avoided. See "Retrospective log" below — this step is part of
   the cycle, not an optional courtesy.
8. **Reclaim.** `scripts/reclaim.sh --apply` at the end of every cycle — not
   only when disk pressure is already visible. The script is dry-run-safe by
   default and its safety guards (never touches a target dir a live build is
   using, never removes a worktree with uncommitted or unpushed work, never
   touches this repo's own checkout or the `pcgen` oracle) make `--apply`
   the correct default for a routine cycle-end, not a special case. See
   `decisions.md` Decision 33 — this is the executable counterpart to the
   `CARGO_TARGET_DIR` cleanup rule that this program has, until now, had only
   as a written instruction nobody automated.

## Corpus shape notes (re-derived 2026-08-02)

Operational guidance for Epics 4-7 (kind lanes, re-cut 2026-08-10 per `decisions.md §37`; the
prior "Epics 3-6 and 11-13" per-book numbering this section referenced is retired), re-derived directly against
the corpus rather than transcribed. Re-check before relying on any of these
if the corpus tree has moved since 2026-08-02.

- **Quoting hazard:** `bestiary_6/_bestiary_6 _for_players.pcc` contains a
  SPACE in the filename — all path handling must quote; an unquoted glob
  silently drops it.
- **`.pcc` naming split:** B1/B2/B3 main pccs have no leading underscore
  (`bestiary_2.pcc`); B4/B5/B6 do (`_bestiary_4.pcc`) — glob `*.pcc`, never
  `bestiary_*.pcc` or `_*.pcc` alone.
- **`SOURCESHORT` is not unique per book:** B1 alone has three pccs carrying
  `SOURCESHORT:B1` (main, `_for_players`, `_pfs`) — key ingest on pcc path or
  `CAMPAIGN` name, not `SOURCESHORT`.
- **`*_races_pc.lst` files are `.MOD` overlays** onto races defined in
  Core/ARG — updates to existing records, not new monsters (e.g. B2's entire
  `b2_races_pc.lst` is 7 `.MOD` lines).
- **Worked count anatomy:** B2's 322 = 314 first-class `RACE` records + 8
  `.COPY=` derived variants (Chupacabra (Flying), Gug Savant, etc.); B3=261,
  B4=220, no `.MOD`/`.COPY` inflation there. State which convention a cycle
  counts under in its receipt.
- **Conditional cross-book support files:** `bestiary_4/support/*_ma.lst`
  load only under Mythic Adventures, `bestiary_5/support/*_oa.lst` only
  under Occult Adventures (`PRECAMPAIGN`-gated) — file-by-file ingest pulls
  them unconditionally and mis-attaches content.
- **B3's pcc `INCLUDE` lines reach into** `../ultimate_combat/` and
  `campaign_setting/inner_sea_gods/` — naive pcc-following drags other books
  in.
- **Seven zero-byte `.lst` files exist across B1-B4** (datacontrols/globalvar)
  — legitimately empty, referenced by pccs; not an error.
- **Upstream quality:** B3/B4/B5/B6 pccs are `STATUS:BETA`; 16 files carry
  `TODO` markers (heaviest b1/b2 races). Coverage claims cite corpus-as-shipped.
- **Book shapes:** B5 (188 units) and B6 (63 units) have ZERO monsters —
  player-options datasets (B5's pcc `CAMPAIGN` literally says "Only Player
  Options Implemented"); bonus_bestiary is a tiny true monster book (14
  monsters, 4 `.lst`); monster_codex is per-record-family (72
  class_features, 32 feats, 24 spells, 45 equipment, 15 companions, 2
  monsters; 18 `.lst` + `support/`).
- **Out-of-scope adjacents:** `inner_sea_bestiary/` (pcc+jpg stub) and
  `inner_sea_world_guide`'s `iswg_races_bestiary.lst` are NOT in this
  bundle — do not pull by accident.

## Retrospective log

Every cycle emits at least one event to the running retrospective log
(`scripts/retro.py`, schema `docs/retro/schema.json`, shards
`docs/retro/events/<actor>.jsonl` — do not re-derive the flag syntax from
memory, run `python3 scripts/retro.py help <type>` for the real fields per
type). This package's own `forward-scope-register.md` already reads the log
as data (`ls docs/retro/events/*.jsonl`); this section is the write side.

- `RETRO_ACTOR` is exported for the dispatched agent per the OPERATING METHOD
  callout above; emissions from inside a cycle inherit it automatically.
- `./scripts/verify.sh` (Cycle mechanics step 4) auto-emits its own
  `verification` event on every run, pass or fail — the denominator is honest
  without anyone deciding to record it. Nothing to do here beyond not skipping
  `verify.sh`.
- A correction to a figure this package (or a prior cycle) stated is not
  folded silently into the next edit: emit
  `scripts/retro.py correction --subject <who/what stated it> --claimed <...> --actual <...> --verified-by <the command/file/count that established the true value>`
  alongside the doc fix. A correction without `--verified-by` is just a
  competing assertion.
- A deferral, a rework, or a near-miss verification caught is emitted at the
  point it happens, never narrated afterward from memory.

See `decisions.md §24`.

## Definition of done (per book-ingest cycle)

All of the following, each checkable by someone who was not present:

1. `./scripts/verify.sh` exits `0`. Exit code captured directly.
2. The `reach` stage passes **with a claim for this book's families**, not by
   the families being absent from the gate's inventory. `reach_gate` reporting
   0 matched tests is a hard failure — a gate running zero tests asserts nothing.
3. `cargo run --locked --bin v06_corpus_trap_report -- --audit` exits `0`.
4. `cargo run --locked --bin v06_work_inventory` regenerates
   `docs/work-inventory.json`, the book's units leave `not-started`, and a
   second run changes only `generated_at`.
5. The four-check wired-integration audit
   (`docs/governance/no-stub-mvp-doctrine.md` §"Per-cycle audit") is clean.
6. Any family that could not be surfaced has an `OPEN_FINDINGS` entry in
   `reach_gate.rs` naming its remedy — recorded as a cycle shortfall, not a
   pass. Each newly ingested book's families must land their own reach
   claims; a bestiary passing "by absence" (its families simply missing from
   the gate's inventory) is the DoD item 2 failure, not this item.
   **`beastiary1/monsters` is no longer an `OPEN_FINDINGS` entry — it is a
   live reach claim (`reach_gate.rs:840`), backed by the shipped monster
   catalog.** The surviving `beastiary1/race_traits` entry (the Duergar
   Spell-Like Ability ~ Invisibility record, upstream-blocked on
   `monster_codex/mc_abilities_race.lst`) is expected to be **retired by
   Epic 5's Monster Codex cycle-batch** (Race-Trait Lane; was Epic 13 under
   the retired per-book numbering) now that Monster Codex is in scope — a
   closure receipt that leaves it standing must say why.
7. Baseline movements in `scripts/verify-baselines.env`, if any, are a separate
   reviewable commit carrying `--show-actuals` output.
8. **On-screen verification for any record family whose reach claim is
   player-visible.** `reach_gate.rs` passing proves a code path exists; it
   does not prove a player sees the value, and three separate compute twins
   have each independently passed that gate while showing nothing on the
   sheet (`decisions.md §29`'s inherited traps). Drive the running desktop
   app via `apps/desktop/.claude/skills/run-desktop/driver.sh` (launch,
   navigate to the record, `screenshot`) and confirm the value on the
   captured image for a sample of this cycle's newly-surfaced families —
   not by reading the gate's exit code. **Set `RUN_DESKTOP_AGENT` to a value
   unique to this cycle before the first `driver.sh` call** — see
   `apps/desktop/.claude/skills/run-desktop/SKILL.md` §"Concurrent agents";
   its unset default collides with any sibling dispatch that also left it
   unset. This is the tranche/7 retrospective's rank-3 finding: on-screen
   driving was the *sole* mechanism that caught 14% of that tranche's
   corrections, and it is the only mechanism that reaches the "wired into a
   twin the sheet doesn't read" class of defect — a passing test cannot, by
   construction (`docs/retro/tranche-7-retrospective.md` §3, §6.1 rule A7).

## Epic ordering

**Re-cut 2026-08-10 (`decisions.md §37`).** SD-29 is partitioned by kind lane, not by book epic.
The prior "Epics 3-6 and 11-13, per-book" structure is retired; see `epic-breakdown.md` for the
full 11-epic structure this section now reflects.

- **Epic 1 (Identifier Cleanup)** fires FIRST. No other epic may start until Epic 1 is closed.
- **Epic 2 (Operator Pre-Launch)** is the pre-launch gate, corpus-wide (all 7 books' shape derived
  in one pass, not per-book). Pre-launch checklist verifies before any other epic starts.
- **Epic 3 (Provenance Gate — PI-Screening for Kind-Lane Ingestion)** fires after Epic 2, before any
  content lane. Wires a PI-blacklist sweep into each lane's extraction step; cites
  `docs/governance/license-matrix.md` for OGL/attribution (already satisfied for all 7 books).
  Blocking per `../corpus-work-channels.md §6` and `decisions.md §37.3`.
- **Epics 4-7 (kind lanes)** may run in any order after Epic 3, file-disjoint by kind:
  - **Epic 4 (Monster / Monster-Ability Chassis Lane)** — merged per
    `../corpus-work-channels.md §9.2`; 2,159 units. **Pilot-then-extend:** Bonus Bestiary (34 units)
    runs end-to-end first; the remaining six books' cycle-batches (Bestiary 2, 3, 4, 5, 6, Monster
    Codex) dispatch only after the pilot lands and its per-unit cost is recorded.
  - **Epic 5 (Race-Trait Lane)** — 1,124 units; fixes the `classify()` name-coincidence defect
    alongside the per-book ingest, per `../corpus-work-channels.md §9.3`.
  - **Epic 6 (Companion Lane)** — 275 units; new mechanism, no corpus-wide precedent.
  - **Epic 7 (Residual Proven-Path Content Lane)** — 203 units (spell/equipment/feat/race/
    equipment_modifier/class) using the settled per-book method. `class_feature` (90 units) is
    excluded — see `decisions.md §37.4`.
- **Epic 8 (DM Toolkit extension, consuming Epic 4's monster records)** is optional-but-proposed.
  Per reach-gate doctrine of 2026-08-01, the toolkit extension either lands inside SD-29 (if cycles
  need the consumer surface to satisfy reach) or surfaces as a Class 3 retrofit (C3.1) in
  `successor-forward-scope-register.md`. Operator-pinned per-cycle at the closure of Epic 4's pilot
  cycle-batch (not all seven books — the toolkit can consume monster records incrementally as
  Epic 4's cycle-batches land).
- **Epic 9 (Build Version Numbering)** fires after Epic 1, before Epic 11. First concrete value
  `0.9.<build>` per the 2026-08-01 amendment.
- **Epic 10 (Bundle Code Review)** fires after Epic 9 and every lane epic (4-7, plus Epic 8 if in
  scope), before Epic 11. Reviews the whole bundle's diff against its branch point, not the closing
  cycle alone; `./scripts/verify.sh` passing is a precondition, not the review itself. Per
  `decisions.md §27`.
- **Epic 11 (Closure Epilogue)** fires LAST. Tranche promotion PR fires only after all other epics
  are closed.

## Hard stops

- The cycle records `decision-blocked` (or the blocker) in `progress.md` and moves to the next ready card per `kanban.md`, rather than pausing (see "Stop vs. press on" below), when:
  - A single monster block's ingest cycle fails to converge after 3 attempts.
  - The build crashes in a way that requires a non-book-list fix.
  - A cross-bundle reference yields a missing monster id that the source bundle's progress file shows as not yet landed.
  - The operator-pinned branch / board diverges from the in-flight branch / board.
  - **A record family cannot be surfaced without work outside this bundle's epic structure** (Decision 10's open question). The monster catalog/browser SHIPPED (`reach_gate.rs:840`); the open instance of this class is any record family with no existing surface analog — e.g. a Monster Codex family the sheet and catalog have no screen for. The cycle reports the gap; it does not add an epic and it does not ingest without a reach claim.
  - **A figure derived this cycle disagrees with a figure recorded in this package.** Investigate which is wrong and report; do not overwrite either on the assumption that the newer one wins.
  - **A book's derived shape contradicts its recorded ingest subtype** — e.g. a per-monster-block epic against a book the generator reports as carrying zero monsters. The cycle reports; the operator re-pins the book list.

## Stop vs. press on

The bullets above are this bundle's concrete STOP instances. This section
states the general rule they follow, so a case not on that list is still
classified correctly. Getting this backwards is expensive in both directions:
stopping on the wrong things stalls the bundle for days under unattended mode;
pressing on through the wrong things ships a defect or clobbers someone else's
work.

**STOP — record `decision-blocked` per the UNATTENDED MODE protocol above; do
not fabricate a pass:**

- A gate fails for a reason that is a real finding about content or scope —
  the reach gate flagging genuinely unsurfaced content is the paradigm case,
  not an exception to route around. Never weaken, skip, `#[ignore]`, or
  exclude a gate to get green, and never invent a surface or a number to
  satisfy one.
- Two authorities disagree on scope (e.g. this package and a sibling bundle's
  docs naming different in-scope books/records).
- The work would revert or clobber another session's live work on the shared
  branch.
- Proceeding would require inventing data not present in the corpus.

None of these mean "ask the operator and wait" — under unattended mode,
`decision-blocked` in `progress.md` with the reason **is** the stop; the
supervisor moves to the next ready card in `kanban.md` rather than idling the
bundle.

**PRESS ON — without asking, without recording `decision-blocked`:**

- This package's own stated figure or premise turns out wrong — correct it in
  place and continue. Correcting the brief/decisions/scope docs is expected,
  not insubordination (emit the correction event per "Retrospective log"
  above).
- The scope turns out larger than expected — no scope is too big to just do.
  Size alone is never a stop reason.
- A mechanical defect (duplicate module after a merge, stale fixture label,
  lint fix) — fix it and continue; this is what "Self-heal" below already
  covers for the cases it names.
- A routine judgment call with a conventional default — pick it, state it in
  the cycle receipt/`progress.md`, and move on.

See `decisions.md §25`.

## Eligibility

A cycle is eligible to fire when:

- The pre-launch checklist is fully green.
- All parent cards the cycle depends on are `complete`.
- The current `progress.md` corresponds to the operator-pinned branch tip.

## Self-heal

- A flaky test that fails once but passes on a clean re-run is annotated in the cycle record and not re-fired.
- A code-side identifier that leaks the `sd29_` pattern is renamed in-cycle (per the identifier-discipline doctrine).
- A cross-bundle reference that yields a missing-monster error is filed as a blocker against the source bundle per UNATTENDED MODE item 3 (blocker + command + exit code in progress.md) and the cycle moves to the next ready card in kanban.md.

## Cross-bundle references

SD-29 references the following bundles:

- **SD-22 (closed):** Bestiary 1 + DM toolkit. Reference is doctrinal read-only. Do not pull from `~/workspace/SD-22-...-*.md` files; pull from SD-22's repo canonical (`~/workspace/repos/codex/docs/release/SD-22/`).
- **SD-28 (launched 2026-08-01, runs FIRST on tranche/8):** Ultimate book content-source ingest. SD-29 launches only after SD-28's closure (sequential order operator-pinned 2026-08-02, decisions.md §34). Read-only reference to its landed state is allowed once closed; its `source_record` work (decisions.md §31) lands there first and SD-29 consumes it.
- **SD-30 (planned):** Occult Adventures + companions. No live cross-reference until SD-30 is launched.

## Decision record

See `decisions.md` for the running decision record. Each decision is dated, named, and stable.

## Per-bundle progress file

`docs/release/SD-29-bestiary-line-book-ingestion/progress.md` (this
directory) carries the per-cycle receipt — it is the sole receipt file for
this bundle. The prior workspace-lane path
(`~/workspace/programs/codex/requirements/SD-29-bestiary-2-3-4-5-content-ingestion/progress.md`)
no longer exists; the source-of-record moved, not copied, on publish per the
`release-package-promotion` skill. Do not use a shared chassis-lane progress
file; each bundle's progress is its own.
