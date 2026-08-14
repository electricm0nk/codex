# SD-30 — Loop Instruction

> ⚠️ **OPERATING METHOD — REQUIRED FOR THIS BUNDLE**
> This file is the operational loop-instruction for SD-30. The bundle is operated via:
>
> **Dispatch mechanism: the in-harness `Workflow` tool, driven from a live session** — not a headless `/loop` script and not a cron driver. Deterministic control flow (per-epic ordering, fan-out, `decision-blocked` handling) lives in this document and in `kanban.md`'s claim/complete state; model judgment lives inside the dispatched `agent()`/`Workflow` calls. Per `decisions.md §22` (adopted from SD-27 `decisions.md §19`, itself adopted from SD-26 `decisions.md §13`) and `docs/governance/loop-instruction-template.md §2`.
>
> `/batch` is **not** the default concurrency primitive for this bundle. `/batch` fans out into parallel isolated worktrees by default; SD-30's cycles mutate shared state on nearly every cycle (`progress.md`, `kanban.md`, `reach_gate.rs`'s `OPEN_FINDINGS`), so parallel dispatch is the exception, called out explicitly per-epic (see "Epic ordering" below), not the default. Any parallel wave dispatches each agent with `isolation: 'worktree'` (`loop-instruction-template.md §3`); a shared-checkout wave with more than one mutating agent and no worktree isolation is not a valid dispatch. Where cycles touch shared state, dispatch an explicit single-cycle procedure instead of reaching for `/batch`.
>
> Every dispatched agent gets `RETRO_ACTOR=<role-name>` set in its environment (`loop-instruction-template.md §2.1`) — no harness variable identifies an agent's role, and the fallback (worktree directory name) names a checkout, not a role, which makes the retrospective log's by-actor breakdown meaningless.
>
> **REQUIRED READ BEFORE THE FIRST CYCLE: `./state-goals-and-lessons.md`.** It carries the state
> this bundle inherits, its goals and honest ceiling, the live hazards (the regenerator silently
> drops 2,371 verification stamps; the dashboard producer crashes rather than degrades on an
> unknown status), and the orchestration lessons that cost the handoff session real time — the
> 2-agent concurrency cap, the background-and-yield stall, and commit-and-push-as-you-go.
>
> The orchestrating session never implements directly — it dispatches, verifies, and rules (`loop-instruction-template.md §2.2`). Do NOT engage this bundle via ad-hoc single-task invocations; one Workflow-tool launch runs to closure.
>
> **Orchestrator model: Opus, low reasoning effort** (operator directive 2026-08-01, `decisions.md §25`) — Opus at low reasoning effort produced materially better orchestration results than Sonnet at high reasoning effort, and is the new normal for the *orchestrating session* on this program. This supersedes any prior "orchestration runs on Sonnet" guidance (none existed in this package before this pass). Dispatched sub-agents are unaffected — they keep task-matched tiers (Haiku for housekeeping, Sonnet for real implementation/debugging/review, Opus for adversarial verification/judge-panel steps only) per `loop-instruction-template.md §2`. A session cannot change its own model mid-run: setting Opus-low is a **pre-launch operator step**, done before this cycle session starts.
>
> **🟡 UNATTENDED MODE (operator directive 2026-08-01).** The operator is out of town. Cycles MUST NOT pause to ask the operator questions; the operator may not see the harness's output for days. The operating protocol for the duration of the bundle is:
>
> 1. **Default-and-flag, not ask.** When the cycle needs a decision, pick the safer default, capture it in the cycle's `progress.md` receipt, and continue. The operator reviews the receipts after return.
> 2. **No `clarify` tool calls.** Cycles must not invoke the operator clarification tool under any circumstance; this is a hard ban during unattended mode.
> 3. **Blockers are recorded, not raised.** If a cycle hits a true hard-block (auth failed, branch can't be created, identity conflict on disk), record the blocker in `progress.md` with the command and exit code, then attempt the next ready card per `kanban.md`. Do not halt the bundle.
> 4. **`decision-blocked` IS allowed.** Where the playbook calls for an operator decision (Mythic Adventures consumer surface in-scope-vs-separate; psychic-discipline consumer; Inner Sea campaign-tool surface), record `decision-blocked` in `progress.md` with the recorded reason and proceed on the safe default per `forward-scope-register.md C3.x` retrofit. Do not wait. See also "Stop vs. press on" below for the general rule this instance follows.
> 5. **Closure is a goal, not a stop signal.** The bundle runs to closure under the Workflow tool's own dispatch loop, not a human re-invoking a slash command per cycle. The operator's review happens after return; cycles do not pause for operator review.
> 6. **Operator's verbatim:** "include instructions to all 3 that indicate they will be running in unnattended mode since i will be out of town while this runs. They may not stop to ask questions - it might be days before i notice."

## Pre-launch checklist (must be true before any cycle fires)

1. **`kanban.md` exists and lists a ready queue.** (Operator-pinned 2026-08-01: Hermes board retired; work-queue artifact is `kanban.md` paired with `progress.md` inside this directory.)
2. **Branch pushed:** `tranche/10` is pushed to origin (`git push -u origin tranche/10`). (Operator-pinned 2026-08-01.)
3. **OAuth credentials valid:** the active harness has fresh GitHub OAuth credentials for `git push` operations to origin.
4. **Working tree clean:** no uncommitted work-in-progress from a prior bundle. Run `git status` from the repo root.
5. **Wave disk budget computed and recorded** — see "Concurrency and resource budget" below. The
   number of agents the first wave dispatches is written into `progress.md` **with the `df` output
   that justified it**, before the wave fires. A wave dispatched without this is out of protocol.
   (New 2026-08-11 from the tranche/9 retrospective: SD-29 dispatched six concurrent worktree agents
   with no disk budget and it cost an entire kind lane.)
6. **Pilot/scope validation performed for every book-or-class a first cycle will claim** — see
   "Pilot and scope validation" below. Applied to SD-30's own pinned scope 2026-08-11; findings are
   recorded in `decisions.md §39` and must be read before Epic 6 pins a first book.
7. **`epic-0-instrument-apply` card status known.** `epic-0-instrument-apply` (`kanban.md` Order 0,
   `decisions.md §43`) runs independently of the `class_feature` chain (`epic-1`..`epic-9`) and does
   not gate or get gated by it — no player-visible lane cycle waits on its completion. This item is
   satisfied by confirming that fact against `kanban.md`'s own claim-priority note before the first
   cycle fires; no interim disposition is needed because no gate exists to disposition. (Corrected
   2026-08-14: the prior text named a nonexistent `epic-0-desktop-driver` card and a DoD-item-8
   interim rule that does not exist in this file; see `retro.py correction`.)

If any of these is false, the cycle refuses to launch and reports the gap.

## Concurrency and resource budget (new 2026-08-11 — derived, not inherited)

**This section exists because it is the single largest recorded cost of the predecessor bundle.**
SD-29 placed six concurrent worktree agents plus two shared-checkout agents on this box. Disk
exhaustion from that arrangement spans **26 of 44 incidents (59 %)** in the tranche/9 window across
three recurrence keys; `preflight-disk` was the **single largest failing stage in the whole log — 9
failures, more than every other stage combined**; `reclaim.sh --apply` ran 14 times and at peak
reclaimed **0.0 B**, because every candidate was correctly refused as a live target dir or an
unpushed worktree. The bill was an entire kind lane: `epic-7-companion-lane-pilot` refused twice at
`preflight-disk` (91 % used / 47 G free), correctly left its card unclaimed, and **1,696 companion
units were never started.** (`docs/retro/tranche-9-retrospective.md` §4.1, §9.1.)

**The numbers below are measured on this box, not carried from that document.** Re-derive them at
pre-launch and before every wave; they are a snapshot, not a constant.

```bash
df -B1G /                                   # 484 total / 387 used / 98 avail / 80%   (2026-08-11)
grep -n 'PREFLIGHT_DISK_MAX_PERCENT=\|PREFLIGHT_DISK_MIN_FREE_GB=' scripts/verify.sh
#   verify.sh:244  PREFLIGHT_DISK_MAX_PERCENT=${PREFLIGHT_DISK_MAX_PERCENT:-90}
#   verify.sh:243  PREFLIGHT_DISK_MIN_FREE_GB=${PREFLIGHT_DISK_MIN_FREE_GB:-20}
du -sh target /home/ubuntu/cargo-targets/* /tmp/codex-target-* 2>/dev/null
#   60G  target                                          (the primary checkout's accumulated tree)
#   27G  /home/ubuntu/cargo-targets/sd29-e2-prelaunch     (one SD-29 cycle's, orphaned)
#   11G  /tmp/codex-target-sd29-e6-racetrait-extend       (orphaned; AGENTS.md bans /tmp for these)
nproc                                       # 4      (NOT 2 — see below)
grep -n 'cargo build parallelism' scripts/verify.sh   # :47  default 2
```

**The budget, derived from those four commands:**

| quantity | value | how |
|---|---:|---|
| filesystem | 484 G | `df -B1G /` |
| currently used | 387 G (80 %) | same |
| `preflight-disk` refuses at | **90 % used** or **< 20 G free** | `verify.sh:243-244` |
| headroom to the 90 % floor | **48 G** | `0.90 × 484 − 387` |
| headroom to the 20 G-free floor | 78 G | `98 − 20` |
| **binding headroom** | **48 G** | the smaller of the two |
| a full-gate `CARGO_TARGET_DIR`, measured | **27 G – 60 G** | `du -sh` above; 27 G is one SD-29 cycle's, 60 G the accumulated primary |
| **concurrent full-gate agents this box can carry today** | **1** | `48 G ÷ 60 G = 0` additional cold target dirs beyond the primary; reclaiming the two orphans (38 G) raises headroom to ~86 G, which affords exactly **one** |

**Rules, binding on the dispatching session:**

1. **The cap is ONE concurrent full-gate agent** until a wave's own `df` shows otherwise. A
   "full-gate agent" is any agent that will run `./scripts/verify.sh` without `--only`. Agents doing
   measurement, doc, or `--only <stage>` work do not count against the cap and may fan out.
2. **The budget is checked before the fan-out, not by each agent afterwards.** `N` concurrent
   full-gate agents need `N × 60 G` **plus** headroom above the 90 % floor. If the budget does not
   fit, **dispatch fewer.** An agent refusing at `preflight-disk` is the gate working correctly and
   is not a substitute for admission control — SD-29 proved that costs a lane.
3. **Every dispatched agent gets its own `CARGO_TARGET_DIR`, named for its role, never under `/tmp`**
   (`AGENTS.md` §Concurrency; `/tmp` is banned there and two orphans are sitting in it today).
   Export it in the dispatch, do not leave it to the agent to remember. Delete it at cycle end
   (Cycle-mechanics step 8 already runs `reclaim.sh --apply`).
4. **Reclaim before the wave, not after the failure.** `scripts/reclaim.sh` (dry run) then `--apply`,
   and record the reclaimed bytes. `reclaim.sh` correctly refuses live target dirs and unpushed
   worktrees, so **`0.0 B` reclaimed means the box is structurally full, not that it is clean** —
   that is the condition to dispatch fewer agents on, and SD-29 read it as noise 14 times.
5. **CPU: `nproc` is 4 and `verify.sh` defaults to `-j 2`.** Two concurrent full sweeps starve each
   other for ~15 minutes and the symptom is *a sweep that looks hung*. Before concluding a build has
   stalled, run `pgrep -fa 'verify.sh|cargo test'` — frozen log timestamps and a frozen `deps/*.d`
   count under live `rustc` mean **starved, not hung** (`AGENTS.md` rule A12).
   (The predecessor retrospective repeatedly called this "a two-core box"; `nproc` says 4. The 2 is
   `verify.sh`'s flag default, not the hardware — corrected 2026-08-11.)
6. **Never `pkill -f` a pattern naming a shared tool.** `pgrep -af`, read the listing, `kill` by PID.
   On a shared checkout every agent's gate has the same command line by construction (`AGENTS.md`
   rule A11; one near-miss in SD-29 would have killed a sibling's 45-minute gate).
7. **One writer per checkout, including the primary one.** If two cards must run at once, both get
   worktrees, or one waits. SD-29 granted isolation to six lane agents and denied it to two others,
   which produced a gate result that *"certifies the mixture, not either card"* and **five green
   commits that could not be pushed at all.**
8. **Cycle ids are allocated by the dispatcher, not minted by the agent.** Two SD-29 lanes
   concurrently minted `SD29-E4-F1-001` because neither could see the other's claim before pushing.
   Suffix per-cycle ids at claim time (`SD30-E6-F1-001-<class>`); a colliding id destroys the receipt
   trail every later audit reads.

## Pilot and scope validation (new 2026-08-11 — REQUIRED before a first cycle pins a book or class)

**Evidence:** SD-29 pinned `inner_sea_intrigue` as its Race-Trait pilot on the strength of 9
`race_trait` units. The count was right and **the kind was wrong** — all 9 come from
`isi_abilities_race_companion.lst` and are Clockwork Familiar / Clockwork Spy *construct-companion*
abilities. The book carries **zero genuine race traits**. Root cause: `file_kind()` types a file by
its **basename**, and `_abilities_race` was tested before the `companion`/`familiar` markers. That
lane's pilot half was `decision-blocked` on the spot. This is the third consecutive bundle in which
filename-typing produced a wrong scoping figure, and the precedent is on record: *"do not assume a
bestiary contains monsters"* — Bestiary 5 and Bestiary 6 carry **zero** monsters, and Monster Codex
carries 2 (SD-29 `loop-instruction.md` §Corpus shape notes).

**The rule: a unit COUNT and a unit KIND are two different claims, and the inventory only asserts
the first.** A scoping figure that selects a pilot is verified **at source, one record deep**, never
at the inventory.

**Required before any cycle claims a book or class for the first time.** Run all three, record all
three verbatim in the cycle receipt:

```bash
# 1. What the inventory says, and WHICH FILES those units come from.
python3 - <<'PY'
import json, collections
B='<book>'; K='class_feature'
U=json.load(open('docs/work-inventory.json'))['units']
us=[u for u in U if u['book']==B and u['kind']==K]
print(len(us),'units'); print(collections.Counter(u['source_file'] for u in us))
print(json.dumps(us[0], indent=1))          # one whole record, not a filtered field
PY

# 2. The source file itself, read — not counted.
awk '!/^#/ && !/^SOURCE/ && NF>0' ~/workspace/repos/pcgen/data/pathfinder/paizo/*/<book>/<file>.lst | head -8

# 3. The shape assertion the LANE depends on, stated and tested.
#    For SD-30 that is: does this book carry the archetype content Epic 5's
#    supersession mechanism acts on, or only base-class / prestige / domain-power content?
python3 - <<'PY'
import json, collections
B='<book>'
U=json.load(open('docs/work-inventory.json'))['units']
us=[u for u in U if u['book']==B and u['kind']=='class_feature']
print('archetype declarations:', sum(1 for u in us if (u.get('type_facet') or '').startswith('Archetype')))
print('key prefixes:', collections.Counter((u.get('corpus_key') or '').split(' ~ ')[0] for u in us).most_common(8))
PY
```

**If step 3 returns a shape the lane's mechanism cannot act on, the book is not a pilot** — record
the finding, pick another, and do not weaken the lane to fit the book. **A prior revision of this
file claimed this step was applied to SD-30's own pinned scope on 2026-08-11 and found four books
that fail it, citing `decisions.md §39` — that section is about declared-PI reading and contains no
such finding, and no four-failing-books finding was located anywhere else in `decisions.md` on a
2026-08-14 search. Treat that finding's record as not located: it must be re-derived (re-run this
step's process for every book Epic 6 is about to pin) before Epic 6 pins its first book, not assumed
still valid.** (`retro.py correction` emitted for this citation, 2026-08-14.)

## Cycle mechanics

Each cycle follows the SD-22 cycle shape, with the repo's ingestion tooling
inserted at the two points where this project has repeatedly shipped defects —
before any ingest code is written, and at verification. The full procedure is
`docs/governance/book-ingestion-playbook.md`; read it before the first cycle of
each book.

0−. **Assert the checkout before reading anything.** MANDATORY FIRST ACTION for every dispatched
   cycle, before step 0. Dispatch worktrees in the predecessor bundle were cut at `7d9f1c4f` —
   **`origin/main`'s tip, a PR-#23 merge from 2026-06-28 with no `docs/` tree at all** — so the
   card's own package directory did not exist and **none of its required reads were present.** Six
   cycles recorded the recovery in their receipts; the event log carries only two such incidents, so
   the log under-records this defect by 3×.

   ```bash
   git rev-parse HEAD && git log --oneline -1
   ls docs/release/SD-30-class-feature-archetype-bundle/loop-instruction.md   # required reads present?
   # If the package directory is absent, on a clean tree ONLY:
   git status --porcelain      # must be empty; if not, STOP and report — you are not alone here
   git fetch origin && git reset --hard origin/tranche/10
   ```

   Record the HEAD you started from in the receipt whether or not you had to recover. A cycle that
   silently recovered and did not say so is why the log under-counts this.
0. **Shape** the book. `cargo run --locked --bin v06_work_inventory`, then read
   the book's `books[]` entry in `docs/work-inventory.json` — `kinds`,
   `files_not_enumerated`, `trap_hits`, `reconciliation`. The shape decides the
   cycle; do not assume a template. **Confirm the book has a corpus directory
   at all** — the current 23-book `class_feature` roster (`decisions.md §33`,
   corrected 2026-08-14 from the old sixteen-pinned-books figure, which was
   retired 2026-08-10) all have verified corpus directories, so a missing
   directory now means corpus drift, not expected absence; treat it as the
   hard stop below. Done once per book, not once per cycle.
0b. **Trap-report** the book, before writing a line of ingest code:
   `cargo run --locked --bin v06_corpus_trap_report -- <book_dir>`. A bare
   book name resolves across all known corpus subtrees (`roleplaying_game`,
   `campaign_setting`, `player_companion`, `dreamscarred_press`); an absolute
   path also works. Record the output in the cycle receipt. See
   `decisions.md` Decision 10.
1. **Read** the doctrine-of-record (`scope-draft.md`, `decisions.md`, current `progress.md`).
1b. **Re-derive.** Before accepting any figure carried in a brief, a doc, or a
   prior cycle's `progress.md` entry — including this package's own
   `scope-draft.md` and `decisions.md` — re-derive it yourself with a
   one-line `grep`/`awk`/`python3` command over the actual source data (the
   PCGen `.lst` tree under `~/workspace/repos/pcgen/data/` for anything not
   yet ingested, `data/corpus/<book>/` for anything that is), and record the
   exact command in the cycle receipt. Do not transcribe a count from a doc,
   a summary tool, or memory of a prior cycle. Worked example, this bundle's
   own data: `awk '!/^#/ && !/^SOURCELONG/ && NF>0' ~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/occult_adventures/oa_spells.lst | wc -l`
   → **2040** (re-derived 2026-08-01), the number to cite for "Occult
   Adventures spell-row count," not a remembered or copied-forward estimate.
   This is the rank-1 finding of **two consecutive retrospectives**, re-run
   here against the live log on 2026-08-11 rather than transcribed:

   ```bash
   python3 - <<'PY'
   import json,glob,re
   ev=[json.loads(l) for f in glob.glob('docs/retro/events/*.jsonl') for l in open(f) if l.strip()]
   cor=[e for e in ev if e.get('type')=='correction']
   verb =re.compile(r'\b(grep|rg|awk|sed|wc -l|python3|find |sort -u|uniq|Counter|jq)\b')
   amend=re.compile(r'(\b(grep|rg|awk|sed|wc -l|python3|sort -u|uniq|Counter|jq)\b|\bfind |\bls )')
   scr  =re.compile(r'driver\.sh|screenshot|Xvfb|DISPLAY|on-screen|desktop app|rendered surface')
   print(len(cor), sum(1 for e in cor if verb.search(e.get('verified_by',''))),
                   sum(1 for e in cor if amend.search(e.get('verified_by',''))),
                   sum(1 for e in cor if scr.search(e.get('verified_by',''))),
                   sum(1 for e in cor if 'verify.sh' in e.get('verified_by','')))
   PY
   # -> 226 corrections all-time: 114 ad-hoc verbatim (50.4%), 129 amended (57.1%),
   #    16 on-screen (7.1%), 13 verify.sh (5.8%)   [re-derived 2026-08-11]
   ```

   Ad-hoc commands over source data are the single strongest detector by a
   wide margin — more than `./scripts/verify.sh`, more than on-screen
   driving, more than every repo test combined. **Two independent tranches
   agree to within one point** on the in-window figure (tranche/7 46 %,
   tranche/9 47.1 %, sole-sufficient 15 of 16), which makes it the most
   robust number this program has.
   **Note the classifier's own defect, because it is the lesson in miniature:**
   the published tranche-7 regex writes `find ` followed by `\b`, which cannot
   match `find ~/workspace/...` (`~` is not a word character), and omits `ls`
   entirely. The amended form above fixes both. *The classifier that measures
   how errors are caught had an error of exactly the class it measures.* See
   `docs/retro/tranche-9-retrospective.md` §3, §3.1 and rule **B16**.
1c. **Preflight** the disk. `./scripts/verify.sh --only preflight-disk` (fast —
   no build). Refuse to start the bounded work below if it fails; run
   `scripts/reclaim.sh` (no flags — dry run) to see what it would reclaim,
   then `scripts/reclaim.sh --apply` and re-check. Disk exhaustion is now this
   program's **largest** recorded orchestration failure mode, not its second —
   tranche/7 recorded 5 of 34 incidents (including `/home` at 100 % used, 0
   bytes available); tranche/9 recorded **26 of 44 (59 %)** and
   `preflight-disk` was its single largest failing stage at **9 failures,
   more than every other stage combined**
   (`docs/retro/tranche-9-retrospective.md` §4.1). A ~490-binary `root-full`
   build (cycle mechanics step 4 below) is exactly what tips a box over.
   See `decisions.md` Decision 31 and **"Concurrency and resource budget"
   above — the disk budget is the dispatcher's job before the fan-out, and
   this step is the agent's last-line check, not the control.**
   **If this step fails twice with `reclaim.sh --apply` in between, do NOT
   set `PREFLIGHT_DISK_MAX_PERCENT`, do NOT fabricate a pass, and do NOT
   park the card `IN-FLIGHT` under an agent that did no bounded work** —
   leave the card claimable, record the refusal in `progress.md` with both
   `df` readings, and **emit an `incident` naming the card** so the
   dispatcher can re-queue it when the condition clears. SD-29's companion
   lane was lost precisely here: the agent did all of the above correctly
   except the last clause, and **nothing re-queued the card** after the disk
   cleared from 91 % to 80 %.
2. **Claim** the highest-priority ready card on `kanban.md` (per `decisions.md §13` + §14a).
3. **Do** the bounded work (TDD per the repo's `AGENTS.md`: failing test → smallest change → green → refactor). **The player surface is part of the bounded work, not a follow-on** — see `decisions.md` Decision 11.
4. **Verify** with `./scripts/verify.sh` (full, not `--quick`), exit code captured
   directly and never through a pipe. Do not compose a substitute command set;
   `cargo test --workspace --locked` from the repo root does not reach
   `apps/desktop/src-tauri` at all. See `decisions.md` Decision 9.

   **4a. GATE SEQUENCING — launch the full gate EARLY, in the background, and do
   bounded work while it runs.** New 2026-08-11. **Three** SD-29 cycles lost
   their gate to turn-budget expiry: Epic 2 run 1 (45 min, card left
   `IN-FLIGHT` with no receipt), `epic-4-proven-feat-race-class` (**never
   obtained an exit code at all** — *"a gate that has not returned is not a
   gate that passed"*), and `epic-6-race-trait-lane-extend` (`VERIFY_EXIT=1`,
   `root-full` *"did not complete — CPU/lock-starved, not hung"*, DoD item 1
   **NO** — and its deliverable shipped anyway). A full sweep on this box takes
   longer than the tail of a turn. Therefore:
   - Start the gate as soon as the code change is complete, **not** as the last
     act of the cycle. Redirect to a log file and capture the exit code in the
     same shell statement that ran it (`./scripts/verify.sh > "$LOG" 2>&1;
     echo "VERIFY_EXIT=$?" >> "$LOG"`), never through a pipe.
   - Do the receipt, the retro events, and the doc edits **while it runs**.
   - **Always land the commit and the receipt before returning**, even if the
     gate has not finished. A receipt that says *"gate launched at HH:MM, log at
     `<path>`, exit code not yet obtained"* is honest and resumable; a card left
     `IN-FLIGHT` with nothing written is not.
   - **"Ran out of turn" is not "blocked."** A resumed cycle inherits a warm
     build cache and is cheap. The predecessor's run-1 halt condition
     (`if (!ok(...)) return`) stopped the entire workflow at Epic 2 on an agent
     that had *finished its derivation* and was waiting on `root-full`; the
     resumed cycle re-derived and **every figure reproduced.** Halt conditions
     in this bundle must distinguish the two.

   **4b. READING THE EXIT CODE.** A non-zero exit is not automatically a gate
   failure. **Read the number:** `143 = 128 + 15 = SIGTERM` — a harness timeout
   killing the wrapper, not a red gate. Corroborate against the log's own
   `SUMMARY` block, which is the gate's verdict. **The harness's task status
   reports the wrapper, never the gate** — in SD-29 the two simultaneously
   reported "completed (exit code 0)" and exit 143 for the same run, and
   neither was the gate's answer. If no exit code was obtained, say so; do not
   infer one.
5. **Commit** with a `feat(sd30): ...` or `fix(sd30): ...` prefix.
   **5a. MERGED-NESS IS VERIFIED BY CONTENT, NOT BY STATUS.** A card is
   `COMPLETE` when its work is **on `tranche/10`**, not when its receipt says
   so. If the cycle ran in a worktree, its branch must be merged or
   fast-forwarded onto the named branch before the card flips, and the
   dispatcher — not the agent — owns that merge. **A successor cycle verifies
   its parent's artifacts by content before depending on them:**
   `grep -rn '<the-new-symbol>' --include=*.rs -l .`, not by reading the card's
   status. SD-29's `epic-5-monster-lane-pilot` was marked `COMPLETE` with its
   **entire chassis** (`RuleSetId::BonusBestiary`, the rules-table module, the
   generator arm, the wire DTO, the `CORPUS_KIND_NAMES` entry, two reach
   claims, the frontend path and 31 corpus records) sitting only on
   `origin/worktree-wf_3516060a-756-9`. The extend cycle found it by exactly
   that `grep`, merged it, and **spent its whole budget on the integration.**
6. **Append** the cycle record directly to `progress.md` (no Hermes release —
   the board is retired), with the command behind every figure it publishes.
   The cycle record carries the PR-id, branch-tip, and per-cycle test result.
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
   `decisions.md` Decision 31 — this is the executable counterpart to the
   `CARGO_TARGET_DIR` cleanup rule that this program has, until now, had only
   as a written instruction nobody automated.

## Retrospective log

Every cycle emits at least one event to the running retrospective log
(`scripts/retro.py`, schema `docs/retro/schema.json`, shards
`docs/retro/events/<actor>.jsonl` — do not re-derive the flag syntax from
memory, run `python3 scripts/retro.py help <type>` for the real fields per
type).

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

See `decisions.md §23`.

## Definition of done (per book-ingest cycle)

All of the following, each checkable by someone who was not present:

1. `./scripts/verify.sh` exits `0`. Exit code captured directly.
2. The `reach` stage passes **with a claim for this book's families**, not by
   the families being absent from the gate's inventory. `reach_gate` reporting
   0 matched tests is a hard failure — a gate running zero tests asserts nothing.
   A record kind the gate does not recognise (haunts, corruptions, psychic
   disciplines) fails until it is classified in `RECORD_TYPE_KINDS` with the
   surface that renders it, or in `SUPPORTING_RECORD_TYPES` with why it is a
   facet of an existing family.
3. `cargo run --locked --bin v06_corpus_trap_report -- --audit` exits `0`.
4. **Guarded regen, not a plain run.** A bare `cargo run --locked --bin
   v06_work_inventory` silently drops every `literal-verified`/
   `fixture-verified` stamp the committed file carries (P0.1 fixed this from
   a live hazard into a hard-refusing guard, `state-goals-and-lessons.md`
   §1.3 hazard 1) — the sanctioned procedure is:
   ```
   cargo run --locked --bin corpus_literal_sweep -- --json-out /tmp/sweep.json
   cargo run --locked --bin derived_evaluator_fixture_check -- --json-out /tmp/fixture.json
   CORPUS_LITERAL_SWEEP_REPORT=/tmp/sweep.json \
   DERIVED_FIXTURE_CHECK_REPORT=/tmp/fixture.json \
     cargo run --locked --bin v06_work_inventory
   ```
   This regenerates `docs/work-inventory.json`, the book's units leave
   `not-started`, a second run changes only `generated_at`, AND the run's own
   guard reports zero stamp loss (it exits 1 naming the dropped count
   otherwise). `--allow-stamp-loss` is the explicit, logged escape hatch when
   a stamp loss is intended — never the default path.
5. The four-check wired-integration audit
   (`docs/governance/no-stub-mvp-doctrine.md` §"Per-cycle audit") is clean.
6. Any family that could not be surfaced has an `OPEN_FINDINGS` entry in
   `reach_gate.rs` naming its remedy — recorded as a cycle shortfall, not a pass.
7. Baseline movements in `scripts/verify-baselines.env`, if any, are a separate
   reviewable commit carrying `--show-actuals` output.
8. **On-screen verification for any record family whose reach claim is
   player-visible.** `reach_gate.rs` passing proves a code path exists; it
   does not prove a player sees the value, and three separate compute twins
   have each independently passed that gate while showing nothing on the
   sheet (`decisions.md §28`'s inherited traps). Drive the running desktop
   app via `apps/desktop/.claude/skills/run-desktop/driver.sh` (launch,
   navigate to the record, `screenshot`) and confirm the value on the
   captured image for a sample of this cycle's newly-surfaced families —
   not by reading the gate's exit code. **Set `RUN_DESKTOP_AGENT` to a value
   unique to this cycle before the first `driver.sh` call** — see
   `apps/desktop/.claude/skills/run-desktop/SKILL.md` §"Concurrent agents";
   its unset default collides with any sibling dispatch that also left it
   unset. This is the tranche/7 retrospective's rank-3 finding, re-derived
   2026-08-01: on-screen driving appears in ~10% of that tranche's logged
   corrections (12 of 122 by the retrospective's own filter) and was the
   *sole* catching mechanism for 8% of them (9 corrections; an earlier
   revision here quoted 14% and conflated the mentions share with the
   sole-mechanism count — the retrospective's §3 itself ranks only the
   rank-1 finding as robust). The percentage is not the point: on-screen
   driving is the only mechanism that reaches the "wired into a twin the
   sheet doesn't read" class of defect — a passing test cannot, by
   construction (`docs/retro/tranche-7-retrospective.md` §3, §6.1 rule A7).

## Epic ordering

**Re-cut 2026-08-10** (`decisions.md §33-38`, `epic-breakdown.md`). The sixteen-per-book-epic
ordering this section previously described is retired; the current 9-epic dependency chain is:

- **Epic 1 (Identifier Cleanup)** fires FIRST. No other epic may start until Epic 1 is closed.
- **Epic 2 (Operator Pre-Launch)** is the pre-launch gate. Pre-launch checklist verifies before any other epic starts.
- **Epic 3 (PI-Screening Provenance Gate)** fires after Epic 2; stands as a standing gate re-invoked by every Epic 6 cycle after that, not a one-time epic.
- **Epic 4 (Per-Class Archetype Measurement)** runs continuously from after Epic 2 onward, clearing classes one at a time; never fully "completes" in the sense of blocking dispatch.
- **Epic 5 (Archetype Mechanism)** and **Epic 6 (Per-Class Chassis Sweep)** dispatch per class, each gated on that class's Epic 4 clearance (Epic 6 additionally gated on Epic 5 for that class). Different classes' Epic 5/6 cycles may run in any order per the file-touch partition, file-disjoint by class and by `rules_tables/<book>/` path.
- **Closure Epilogue (Epic 9)** fires LAST. Tranche promotion PR fires only after all other epics close.
- **Build Version Numbering (Epic 7)** fires after Epic 1, before Closure. First concrete value `0.10.<build>` per `decisions.md §15`.
- **Bundle Code Review (Epic 8)** fires after Build Version Numbering and Epics 5/6's cycles for that pass, before Closure. Reviews the whole bundle's diff against its branch point, not the closing cycle alone; `./scripts/verify.sh` passing is a precondition, not the review itself. Per `decisions.md §26`.

## Hard stops

- Stops and reports the blocker (per the repo's `AGENTS.md` hard-stop doctrine) when:
  - A single class / monster / discipline's ingest cycle fails to converge after 3 attempts.
  - The build crashes in a way that requires a non-book-list fix.
  - A cross-bundle reference yields a missing class / monster id that the source bundle's progress file shows as not yet landed.
  - The operator-pinned branch / board diverges from the in-flight branch / board.
  - **A book on the recorded list has no corpus directory to ingest from.** The cycle reports; the operator re-pins the book list. No known instances as of 2026-08-14 — all 23 `class_feature`-bearing corpus dirs (`decisions.md §33`) have verified corpus directories. (An earlier revision named Occult Origins and Haunted Heroes Handbook here; that finding was a bad check — wrong search root and wrong identifier — and both books exist under `player_companion/`. They are deferred by operator choice, not absence; see `scope-draft.md` and `decisions.md` Decision 1. That earlier revision also referred to "sixteen pinned books," language retired 2026-08-10 by the widened `class_feature` re-scope.)
  - **A record family cannot be surfaced without work outside this bundle's epic structure** (Decision 11's open question). The cycle reports the gap; it does not add an epic and it does not ingest without a reach claim.
  - **A figure derived this cycle disagrees with a figure recorded in this package.** Investigate which is wrong and report; do not overwrite either on the assumption that the newer one wins.

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

See `decisions.md §24`.

## Eligibility

A cycle is eligible to fire when:

- The pre-launch checklist is fully green.
- All parent cards the cycle depends on are `complete`.
- The current `progress.md` corresponds to the operator-pinned branch tip.

## Self-heal

- A flaky test that fails once but passes on a clean re-run is annotated in the cycle record and not re-fired.
- A code-side identifier that leaks the `sd30_` pattern is renamed in-cycle (per the identifier-discipline doctrine).
- A cross-bundle reference that yields a missing-class / missing-monster error is filed as a blocker against the source bundle and the cycle pauses.

## Cross-bundle references

SD-30 references the following bundles:

- **SD-22 (closed):** APG + ACG + Bestiary 1 + DM toolkit. Reference is doctrinal read-only. Do not pull from `~/workspace/SD-22-...-*.md` files; pull from SD-22's repo canonical (`~/workspace/repos/codex/docs/release/SD-22/`).
- **SD-28 (planned):** Ultimate book content-source ingest. Class overlap (Occultist, Spiritualist, Medium, Mesmerist in Ultimate Intrigue) is canonical-to-SD-30; SD-28 references the canonical class id only.
- **SD-29 (planned):** Bestiary 2-3-4-5 content-source ingest. Monster overlap (occult monsters in later Bestiary books) is canonical-to-SD-30; SD-29 references the canonical monster id only.

## Decision record

See `decisions.md` for the running decision record. Each decision is dated, named, and stable.

## Per-bundle progress file

`docs/release/SD-30-class-feature-archetype-bundle/progress.md` — this package's own directory, where the move-not-copy publish landed it — carries the per-cycle receipt. (Corrected 2026-08-01: an earlier revision pointed at `~/workspace/programs/codex/requirements/SD-30-.../progress.md`, a directory that does not exist.) Do not use a shared chassis-lane progress file; each bundle's progress is its own.
