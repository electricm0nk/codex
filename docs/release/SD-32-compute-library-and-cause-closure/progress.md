---
canonical: true
owner: god-emporer
status: planning-ready (chassis completed 2026-08-22; launch-readiness remediation 2026-08-22; cycle receipts land here)
date: 2026-08-22
---

# SD-32 Progress

Per-cycle receipts land here, in the order they were completed. Each cycle appends one section
under "## Cycles" below. The schema lives in `workflow-instruction.md §7`. Non-self-healable
failures go under "## Open blockers"; out-of-card discoveries under "## DISCOVERED"
(`workflow-instruction.md §8`, `kanban.md` status values).

**At planning-ready time, this file holds only the chassis cycle and the pre-launch receipt.** No
dispatched cycles have run. The file's role is to be the durable audit trail once cycles start.

## Pre-launch state (2026-08-22)

Recorded once, at launch-readiness remediation, so a future reader sees what the package looked
like before the first dispatched cycle landed. Every figure names its command.

- `docs/work-inventory.json` total units: `jq '.total_units' docs/work-inventory.json` → re-derived by
  the first Gate 0 cycle, never transcribed; the SD-31 close baseline was 38,372
  (`epic-breakdown.md`, SD-31 wave 31).
- Doneness: SD-31 close baseline 13,458 / 38,372 = 35.07% (`doneness_verdict()` replay,
  `artifacts/HANDOFF.md`); re-derived by the first Gate 0 cycle.
- PCGen oracle pin SHA: `grep PCGEN_ORACLE_SHA scripts/pcgen-oracle-pin.env` →
  `7f818006e371188e5717fd18d74d18a420747fc6`, present in the repo-local slot
  `artifacts/corpus/operator-supplied/pcgen` (§1 item 9 below).
- Branch: `tranche/12`, cut from `tranche/11`'s tip. SD-31 content on develop via PR #374
  (merged 2026-08-22T19:53Z), verified by content (§1 item 3 below).
- Build counter: `0.12.0` on `tranche/12` (commit `29160889d`, `feat(sd32): version bump 0.12.0 for
  tranche/12`; SD-31 precedent `147f1c2b7`). Derivation:
  `grep -h '"version"' apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json | head -2`.
- Generator population for Epic 5: `ls src/bin/{gen_,ingest_,enrich_}*.rs | wc -l` → 29.

## Pre-launch receipt — `workflow-instruction.md §1` run for real (2026-08-22)

SD-31 precedent: commit `1980d6b95` (S6-prelaunch). Run from the repo root on `tranche/12`; every
item's literal output is also pasted under the command in `workflow-instruction.md §1`.

| # | Item | Command (abridged — full form in `workflow-instruction.md §1`) | Output | Verdict |
|---|---|---|---|---|
| 1 | Local kanban reachable | `test -f kanban.md && wc -l kanban.md progress.md` | `61 kanban.md`, `131 progress.md` | PASS |
| 2 | Branch on origin, ahead of develop | `git ls-remote --heads origin tranche/12`; `git rev-list --count …` | `8d387c39c refs/heads/tranche/12`; ahead=11 behind=0 (pushed again after this receipt) | PASS |
| 3 | SD-31 content on develop (by content) | `gh pr view 374 …`; `git diff --stat origin/develop b1b7f4290 -- src scripts data docs/retro docs/release/SD-31-corpus-closure-grind \| wc -l` | `374 MERGED 2026-08-22T19:53:56Z tranche/11->develop`; `0` | PASS |
| 4 | PAT present | `test -f ~/.config/gh/.claude_gh_token && echo PAT_PRESENT` | `PAT_PRESENT` | PASS |
| 5 | Working tree clean | `git branch --show-current; git status --porcelain \| wc -l` | `tranche/12`; `0` (after commit `d60377a7e`) | PASS |
| 6 | Doctrine docs present | `test -f docs/doctrine-external/identifier-discipline.md && test -f docs/governance/no-stub-mvp-doctrine.md && echo DOCTRINE_PRESENT` | `DOCTRINE_PRESENT` | PASS |
| 7 | Build counter literal | `grep -h '"version"' apps/desktop/package.json apps/desktop/src-tauri/tauri.conf.json \| head -2` | `"version": "0.12.0"` ×2 (commit `29160889d`) | PASS |
| 8 | Artifact dirs exist | `ls -d artifacts/{epic-5-protective-sweep,gate-0-…,gate-1-…,gate-2-…,gate-3-…,corpus/operator-supplied}` | all six listed | PASS |
| 9 | PCGen oracle in the repo-local slot, at pin, git-ignored | `export PCGEN_REPO_DIR=…/artifacts/corpus/operator-supplied/pcgen; export PCGEN_CORPUS_ROOT=$PCGEN_REPO_DIR/data; scripts/verify.sh --only preflight-oracle; git status --porcelain \| grep -c operator-supplied` | `RESULT: PASS` (oracle `7f818006e371188e5717fd18d74d18a420747fc6`); `0` | PASS |

**Verdict: 9/9 PASS — launch-ready.** Launch base for `workflow-instruction.md §6` step 1: the
orchestrator captures `PIN=$(git rev-parse origin/tranche/12)` at dispatch time; at receipt time
`origin/tranche/12` was `8d387c39c` and local HEAD `d60377a7e` (pushed with this receipt).
Non-blocking notes: (a) `tranche/12` ran 1 commit behind develop at audit time (the #374 merge
commit) — merged in as `8d387c39c`, now 0 behind; (b) the chassis commit `5586706b7` had been
unpushed — pushed; (c) `scripts/verify.sh` auto-emits a retro event per run
(`docs/retro/events/sd31-transcribe.jsonl`) — committed alongside, per commit `e28d79a8c`'s precedent.

## Cycles

<!-- Append cycle receipt sections below this line, newest at the bottom. -->

### Cycle 0 — Chassis completion (chassis-only, no agent dispatch)

- **Card ID:** chassis-only
- **Commit SHA:** `89fa78276` (chassis fill-out), `0982f9003` (completeness-review gaps),
  `5586706b7` (re-author from the updated STC templates; rename to `workflow-instruction.md`);
  launch-readiness remediation commits follow in this file's git history.
- **Files touched:** `README.md` (promoted from draft to planning-ready), `scope-draft.md`
  (canonicalised), `epic-breakdown.md` (finalised against wave 31 measurement), `decisions.md`
  (new), `workflow-instruction.md` (new), `technical-requirements.md` (new), `technical-design.md`
  (new), `acceptance-and-verification.md` (new), `risks-and-open-questions.md` (new),
  `forward-scope-register.md` (new), `release-notes.md` (new), `progress.md` (this file),
  `kanban.md` (new), `content-unit-inventory.md` (new), `artifacts/README.md` (new),
  `references/README.md` (new), `artifacts/HANDOFF.md` (carried from SD-31 session),
  `artifacts/UNMERGED-BRANCHES.md` (carried from SD-31 session), and the cleanup of
  `docs/release/SD-32-instrument-coverage-and-consumer-wiring/` (untracked `__pycache__` only).
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (no `sd[0-9]+_` / `SD[0-9]+_` patterns
  introduced; the SD-32 identifier is bundle-level folder naming, not source code).
- **Wired-integration audit result:** OK_NO_TOKENS (planning docs only; no shipping code touched
  in this cycle).
- **Acceptance criterion:** chassis-completion. All 13 template-canonical files plus the house-
  standard extras present (14 root `.md` files + `artifacts/` + `references/`; see `README.md`
  "Files in this folder"); the dead `SD-32-instrument-coverage-and-consumer-wiring/` folder
  cleaned up.
- **Status:** complete
- **Notes:** This is the chassis-only cycle, run by the operator session, not by a dispatched
  agent. From this point forward, all cycles are dispatched via the `Workflow` tool per
  `workflow-instruction.md §2` and follow §6's per-cycle procedure.
- **Discovery forwards:** none.
- **Next-cycle plan:** Pre-G0 phase — card 1 (Epic 5 protective sweep across the 29 Rust
  generators; `scripts/derive_derived_evaluator_fixtures.py` precedent, `artifacts/HANDOFF.md`) in a
  worktree, card 2 (boundary-branch review) in the primary checkout. `workflow-instruction.md §2.4`.

### Cycle 0a — Launch-readiness remediation (planning docs only, no agent dispatch for docs; version bump dispatched)

- **Card ID:** chassis-only (pre-launch)
- **Commit SHA:** recorded in the pre-launch receipt above and in git history
  (`docs(sd32): launch-readiness remediation …`).
- **Files touched:** `workflow-instruction.md` (rewritten: H1, §0, §1 run for real with outputs, §2
  model rule, §2.1 repo-local oracle env block, §2.2 "dispatch first, report second" restored, §2.4
  executable script matching `kanban.md`, §3 seven-row phase map, §4 verified claims incl. "no
  coverage stage exists" and the Rust fixture-check CLI, §6 mechanical base-pin check, §9 seven
  standing lessons, §10/§11 placeholder + build counter resolved, §13 DoD trigger + `SD-21 §17`),
  `README.md`, `decisions.md` (§1, §2, §6, §7 B3 note, §8, §9), `acceptance-and-verification.md`
  (repo-local oracle, real fixture-check CLI, F1..F9 wording, blocker-shape list, DoD),
  `technical-requirements.md`, `technical-design.md`, `epic-breakdown.md` (blocker-shape count),
  `risks-and-open-questions.md`, `scope-draft.md` (five epics), `kanban.md` (cards 1/2/11, order,
  section cites), `artifacts/README.md`, `artifacts/UNMERGED-BRANCHES.md` (local-only + unlisted
  origin branches), `references/README.md` (own-artifacts section, §13 cites, path fix),
  `artifacts/corpus/README.md` + `artifacts/corpus/operator-supplied/README.md` (new), five
  `artifacts/<phase>/.gitkeep`, `progress.md` (this file). Shipping code: `apps/desktop/package.json`
  + `apps/desktop/src-tauri/tauri.conf.json` `0.11.0 → 0.12.0` — **dispatched to a Haiku housekeeping
  agent** per `workflow-instruction.md §2.2`, commit `29160889d`.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (docs + version fields only).
- **Wired-integration audit result:** OK_NO_TOKENS.
- **Acceptance criterion:** `workflow-instruction.md` launch-readiness — §1 run end-to-end with
  pasted outputs; no template marker in the bundle (`§10` gate); every pcgen file reference
  repo-local under `artifacts/corpus/` (operator directive 2026-08-22); SD-31 content on develop
  verified by content; closure trigger restated as the Definition of Done (operator ruling
  2026-08-22).
- **Status:** complete
- **Notes:** Findings that drove this cycle are in the session plan
  (`~/.claude/plans/evaluate-workflow-instructions-md-for-la-dynamic-wozniak.md`, operator-local)
  and are summarised in the files-touched list above. `artifacts/HANDOFF.md` was deliberately not
  edited (captured session context); its re-derived figures are corrected in the citing docs
  (`decisions.md §8`).
- **Discovery forwards:** none (card 2's scope was widened in place to cover the site branches and
  the unlisted origin branches).
- **Next-cycle plan:** dispatch `workflow-instruction.md §2.4` — Pre-G0 phase.

### Cycle 1 — Pre-G0 / Card 2 `boundary-branch-review`

- **Card ID:** `boundary-branch-review`
- **Commit SHA:** see this cycle's own commit in git log (git-refs-only scope; no shipping/doc diff
  beyond this receipt, `kanban.md`, this file, and the retro log).
- **Files touched:** `artifacts/epic-5-protective-sweep/boundary-branch-review_cycle_receipt.md`
  (new), `kanban.md` (card 2 → complete), `progress.md` (this entry),
  `docs/retro/events/boundary-branch-review.jsonl` (new, correction + deferral events).
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (docs only).
- **Wired-integration audit result:** OK_NO_TOKENS — 2 raw grep hits against the full
  `BASE_BRANCH...HEAD` diff (which spans the whole bundle branch, not just this cycle), both false
  positives on the literal directory name `todo/sweeps.md` (`kanban.md`'s pre-existing card-2 note
  text and `workflow-instruction.md`'s pre-existing §remediation note), neither introduced by this
  cycle and neither a stub/placeholder token. No real violation.
- **Acceptance criterion:** kanban.md #2 — review and disposition the 3 orphaned-but-real branches,
  confirm the closed sweep landed, disposition the site branches and the 9 unlisted origin branches,
  leave the GAMED + rescue branches untouched.
- **Status:** complete
- **Notes:** Full disposition table, evidence, and commands in the cycle receipt (path above).
  Deleted 4 local branches (`site-deploy`, `fix/site-deploy-page-workflow`, `review-merge-test`,
  `worktree-wf_cb84ba1e-439-2`) and 8 origin branches (7 `worktree-wf_*` at 0 unique commits +
  `test`, superseded). Kept `worktree-wf_c1156061-e3f-3`/`-5` (real unlanded work, forwarded below).
  **Correction:** `UNMERGED-BRANCHES.md` §3 mis-categorized `worktree-wf_be4660f2-72a-3` as
  mergeable late-landing work — it is a **third GAMED branch** (`OPEN-ISSUES.md` row 365,
  RULING-NEEDED, referenced directly in `src/rules_core/pilot_compute/mod.rs:9645`), left untouched
  alongside the other two named GAMED branches. Logged via `scripts/retro.py correction`.
- **Discovery forwards:** DISCOVERED-1, DISCOVERED-2 (below).
- **Next-cycle plan:** Gate 0 (card 3) is gated on Pre-G0 (cards 1+2) both closing — card 1 (Epic 5
  protective sweep) is confirmed still live this cycle (`worktree-wf_efd6f5fc-a9c-1`, locked
  worktree).

### Cycle 1 — Pre-G0 / Card 1 `epic-5-protective-sweep`

- **Card ID:** `epic-5-protective-sweep`
- **Commit SHA:** `3b470c56f` (rebased onto `8d1e1dd78` after card 2 landed concurrently).
- **Files touched:** `src/bin/gen_book_cache.rs` (`gen_advanced_race_guide`, `gen_companion_book`,
  `gen_pathfinder_unchained`), `src/bin/gen_core_rulebook_cache.rs` (`main`),
  `src/rules_core/cache_gen/{acg,apg,beastiary1,spell_lane_dump,ultimate_equipment}.rs`, plus
  `kanban.md`, this file, and `docs/retro/events/epic-5-protective-sweep.jsonl` (new).
- **Identifier audit result:** OK_NO_BUNDLE_TAGS.
- **Wired-integration audit result:** OK_NO_TOKENS.
- **Acceptance criterion:** `acceptance-and-verification.md` AT-32-E5-001 — the protective
  self-erasure sweep across all 29 Rust generators runs before Gate 0.
- **Status:** complete
- **Notes:** Re-verified population = 29. Fixed the 2 SD-31 D9 binaries (4 vulnerable functions
  total) PLUS 5 more genuinely vulnerable generators discovered in the "17 never checked" bucket
  (`gen_cache_acg`, `gen_cache_apg`, `gen_cache_beastiary`, `gen_cache_spell_lane_dump`,
  `gen_cache_ultimate_equipment`) — SD-31's own "12 checked (3 vulnerable)" framing undercounted
  the residual; see the two `correction` retro events and Discovery entries below. Live RED→GREEN
  reproduction for the 4 binary functions (isolated worktree, `git checkout --`/`git clean -fd`
  reverted between runs); permanent unit-test RED→GREEN for the 5 library modules. Full detail,
  every command, and the per-generator SAFE/VULNERABLE table: this card's own receipt,
  `artifacts/epic-5-protective-sweep/cycle-1_cycle_receipt.md`.
- **Discovery forwards:** resolves card 2's DISCOVERED-1 below (superseded, see its own note) — no
  new forwards opened by this cycle.
- **Next-cycle plan:** Gate 0 (card 3) is now gated only on Pre-G0 being fully closed (cards 1+2 both
  `complete` as of this commit) — card 3 is ready to dispatch.

## Open blockers

<!-- Non-self-healable failures (workflow-instruction.md §8): one entry per blocker — cycle id,
     card id, what failed, the command that shows it, named owner. Empty at launch. -->

(none)

## DISCOVERED

<!-- Work found mid-cycle that does not fit the claimed card (kanban.md `DISCOVERED-forked`).
     One line each: date, discovering cycle, what, proposed card or forward-scope-register id.
     Queue > 10 entries is non-self-healable (§8). Empty at launch. -->

- 2026-08-22, Cycle 1 (`boundary-branch-review`): `worktree-wf_c1156061-e3f-3`'s `gen_book_cache.rs`
  self-erasure fix (`index_existing_records_by_key`, extends the `gen_monster_book` exists-guard to
  `gen_pathfinder_unchained`/`gen_advanced_race_guide`/`gen_companion_book`) is real, unmerged, and
  the vulnerability is confirmed still live at HEAD (`grep -n remove_dir_all src/bin/gen_book_cache.rs`
  still shows 3 unguarded call sites). Same defect class as Epic 5's own sweep (card 1) — proposed
  target: card 1's dispatch or a dedicated follow-up cycle, fresh TDD (RED→GREEN), not a raw
  cherry-pick of the pre-SD-32 branch.
  **RESOLVED, Cycle 1 (`epic-5-protective-sweep`), same day:** fixed independently with a fresh
  TDD cycle exactly as this note's own target proposed, not a cherry-pick of
  `worktree-wf_c1156061-e3f-3`. `worktree-wf_c1156061-e3f-3` itself remains unmerged and can be
  deleted as superseded (its content is now landed under this cycle's own commit `3b470c56f`,
  independently re-derived and RED→GREEN proven).
- 2026-08-22, Cycle 1 (`boundary-branch-review`): `worktree-wf_c1156061-e3f-5`'s two doc corrections
  (`todo/levers.md` L3 → DEAD; `todo/defects.md` new finding re `MonsterAbilityRecord` missing a
  `DEFINE:`/`SPELLS:` field, needs a fresh ID — current `D9` is taken by a different, already-landed
  finding) never landed under any wording. Doc-only, low risk — proposed target: next cycle touching
  `docs/release/SD-31-corpus-closure-grind/todo/`.
