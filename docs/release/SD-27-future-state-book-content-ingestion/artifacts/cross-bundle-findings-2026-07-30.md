---
title: Cross-bundle findings for SD-27 from the v0.6 CRB retrospective
date: 2026-07-30
source: v0.6 CRB completion run (tranche/6, closed via PR #343 and #346)
status: advisory — findings only, no scope changed and no epic added
---

# Cross-bundle findings for SD-27

Recorded on `tranche/8` as a **new file** so it cannot conflict with the live SD-27
edits on `tranche/7`. Nothing here changes SD-27's scope; items 1 and 2 are decisions
for the SD-27 owner.

Every figure below was re-derived from code or corpus on 2026-07-30. Where this file
and a tool disagree, **the tool is right and this file is stale**.

---

## 1. Two contradictions inside SD-27's own process docs

Both were found while updating the planning-tree copies of these files. They are
recorded in full at the planning tree's `decisions.md §19`
(`~/workspace/programs/codex/requirements/SD-27-future-state-book-content-ingestion/`),
which as of this date is **ahead of the repo copy**.

### 1.1 `decisions.md §8` forbids what `loop-instruction.md:243` requires

The partition's "must not touch" list bans `src/rules_core/rules_tables/<book>/`
**for any book**. The per-book cycle at `loop-instruction.md:243` writes exactly that
tree, `epic-breakdown.md:53` repeats it, and the partition audit command at
`loop-instruction.md:399` explicitly allows the path.

**A cycle reading §8 literally will refuse required work.** This is a doc fix, not a
scope question — but it must be made before it burns a cycle.

*(Line numbers are against the planning-tree copy on 2026-07-30; re-locate by content,
not by number, since `tranche/7` has since edited these files.)*

### 1.2 Content-only scope vs. the content-reach gate

`README.md §1` and `technical-design.md:156` record "no new engine work". Nothing in
SD-27's six process files mentions a player surface, IPC, or the desktop app. But the
per-book cycle generates `src/rules_core/rules_tables/<book>/` — the exact tree that
`apps/desktop/src-tauri/src/reach_gate.rs` scans.

**Once `develop` is merged, the reach gate will fail an ingest that reaches no player
surface.** That is deliberate, not a regression. Decide whether surfacing belongs in
SD-27 or is a named prerequisite bundle.

### 1.3 Two smaller internal inconsistencies

- **ACG's ceiling is stated two ways** — `content-unit-inventory.md §1.4` says "not touched, verify independently"; `technical-design.md:41` states a measured 98.1%.
- **"23 books" vs "25 books"** — criterion 2.0.10's heading says 25; its body and `acceptance-and-verification.md` say 23.

---

## 2. Why the reach gate exists

The single most repeated defect of the CRB run was **content ingested, computed,
corpus-cited, and never reaching the player**. It was diagnosed and one-off-fixed
**six times** — feats rendering as raw ids, 441 invisible APG/ACG spells, equipment
computed then discarded at the IPC boundary, AC-by-source, the Pets tab, the Weapons
tab — before anyone treated it as structural.

The gate was then built, and immediately found **four more**: APG equipment (338),
ACG equipment (269), Bestiary equipment (4), and all 41 monsters.

**Ingest and surfacing are one unit of work.** A book that ingests cleanly and shows
the player nothing is not done.

---

## 3. Tooling now available (merge `develop` to get it)

As of 2026-07-30, `origin/tranche/7` has **none** of the following and is 107 commits
behind `develop`.

| tool | use |
|---|---|
| `./scripts/verify.sh` | the single gate — 9 stages, one exit code |
| `cargo run --bin v06_work_inventory` → `docs/work-inventory.json` | generated per-book counts, all 25 books |
| `cargo run --bin v06_corpus_trap_report` | run **before** ingesting a book |
| `apps/desktop/src-tauri/src/reach_gate.rs` | fails when ingested content reaches no surface |
| `scripts/retro.py` | event log; `verify.sh` auto-emits a verification event |
| `docs/governance/book-ingestion-playbook.md` | the per-book cycle |

**The merge is lower-risk than the diffstat suggests.** Both branches changed
`src/rules_core/encumbrance.rs` and `src/rules_core/equipment_effects.rs`, which reads
as a collision — but the blobs are **byte-identical** (`git rev-parse` returns the same
SHA on both sides). That equipment work is already in `develop`.

### 3.1 `verify.sh` exists because four false-green paths shipped

- `cargo test` **fail-fasts** — one run reported green having executed 124 of 488 suites. `--no-fail-fast` is mandatory.
- **Piping to `grep`/`tail` returns the pipe's exit status, not cargo's.** Never pipe the gate.
- `apps/desktop/src-tauri` is a **separate cargo workspace with no lib target** — a root-only sweep misses it, and `--lib` fails there.
- A clean root sweep passed on a tree whose **desktop binary would not compile**.

---

## 4. Counting discipline

During the CRB run, **orchestrator-supplied figures were corrected by agents six
times**: 396→301 missing feats, 207→166 bonus-bearing, 180→86 `BONUS:VAR` records (one
feat carried 66 tokens), 186→185 CRB feats, twenty→nineteen books remaining.

Every correction came from an agent checking the corpus against a number carried from
memory or a prior brief. Three briefs were also wrong on substance: a "sole blocker"
that wasn't one, a feature list whose names existed nowhere in the repo, and a "broken"
feat that worked correctly.

**Cite `v06_work_inventory`; do not quote a remembered figure.** If a brief hands you a
number, checking it is expected.

---

## 5. Verified corpus facts affecting downstream bundles

Checked 2026-07-30 against
`/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game`.

- **`bestiary_2`** has a real `b2_races.lst` with **323** monster declarations.
- **`bestiary_5` and `bestiary_6` have no base `races.lst` at all** — only `_pc` and `_companion` files plus a `_for_players.pcc`. They are **player-options datasets, not monster books**. A per-monster-block epic over them yields **zero cycles**. This invalidates SD-29's "~250-300 monsters each, total ~1,000-1,200".
- **`occult_origins` and `haunted_heroes` (SD-30) are absent** from all 25 corpus directories.
- **`core_essentials` is `shared_library`, not out-of-scope** — its `included_by` names nine books, including the Core Rulebook, Bestiary 1, and SD-27's own in-scope Advanced Race Guide.

Derive book shape from the corpus before writing any epic.

---

## 6. Shared-checkout hazards

- **`git status --porcelain` before every git write. Never `git add -A`** — an indiscriminate add swept another agent's uncommitted work into a commit whose message described something else entirely.
- **Never `git stash` in this repo.** The bare form stashes the whole repo even from a subdirectory; it has burned this checkout three times.
- **Use a scratch `CARGO_TARGET_DIR` per agent** — a shared one cross-fed build artifacts between agents and produced plausible-but-wrong results.
- **Check `git symbolic-ref refs/remotes/origin/HEAD`** — new worktrees branch from it; it silently pointed at the wrong branch for days and misrouted 4+ agents.
- **Export `RETRO_ACTOR=<role>` per dispatched agent** — no harness variable identifies an agent, and the fallback names a checkout, not a role.

---

## 7. A tranche PR is the close-out, not a checkpoint

PR #343 was opened and then accrued hours of further agent work, making it describe a
snapshot rather than the branch's final state. Roughly 46,000 lines looked merged and
were not; it cost a second close-out PR (#346) and a full re-verification pass.

As of 2026-07-30, **PR #342 (`tranche/7 → develop`) is open with 10+ substantive
commits added after it was opened** — the corpus loader, both equipment fixes, the
`RawToken`/`RawBonusChain` work, and four docs commits. Same shape. Either land it and
branch fresh for continuing work, or state plainly in the PR body that it is still
accruing.

**Verify merged-ness by content, never by commit count.** Heartbeat cron commits inflate
counts and squash merges hide real work; `git cat-file -e <branch>:<path>` is what
settles it.
