# Cycle 1 — Closure / Row 11 (`epic-2-cause-closure`) + Row 13 (`closure-epilogue`)

- **Card ID:** `epic-2-cause-closure` (row 11, closed this cycle), `closure-epilogue` (row 13, stays `in-progress`)
- **Commit SHA:** (recorded after commit below)
- **Files touched:**
  - `src/rules_core/rules_tables/feat_gap_tables.rs` (regenerated via `cargo run --locked --bin gen_feat_gap_tables`, 3 records redacted)
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (row 11 → `complete`, row 13 cycle history appended)
  - `docs/release/SD-32-compute-library-and-cause-closure/release-notes.md` (dated correction note; superseded sections marked)
  - `docs/release/SD-32-compute-library-and-cause-closure/references/README.md` (retrospective citation corrected)
  - `docs/retro/sd32-compute-library-and-cause-closure-retrospective.md` (corrected + "What actually closed the bundle" section added)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
  (`git diff --unified=0` on working tree, all touched files → no `sd[0-9]+_`/`SD[0-9]+_`/`t_[0-9a-f]{8,}` hits)
- **Wired-integration audit result:** OK_NO_TOKENS (self-healed)
  (`git diff --unified=0` → 2 hits on `placeholder`, both inside **removed** prior-cycle prose text
  being replaced — one discussing `%N` description-variable placeholders, one quoting
  `decisions.md §1a`'s own anti-gaming-doctrine language ("a placeholder shape") — neither is a
  stub in shipping code; no `STUB`/`MOCK`/`not yet implemented`/`todo`/`fixme`/`hack` hits anywhere)
- **Acceptance criterion:** `decisions.md §10` — "Done = all gates met AND every Epic 1-13 kanban
  card `complete`." `workflow-instruction.md §13` (bundle closure epilogue, 5 steps).
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`grep PCGEN_ORACLE_SHA
  scripts/pcgen-oracle-pin.env`, confirmed via `scripts/verify.sh --only preflight-oracle` → PASS)
- **Status:** `epic-2-cause-closure` → `complete`. `closure-epilogue` → stays `in-progress`
  (blocked on the worktree/branch sweep step; see below).
- **Notes:**

## Row 11 (`epic-2-cause-closure`) — closed

Per the dispatch brief's `§17a` instruction, every sub-item was re-derived live rather than trusted
from a prior receipt:

- `scripts/verify.sh --only shape-coverage-standing-gate` → `PASS  shape-coverage-standing-gate
  (population=34397 unclassified=0 no_record=0 corpus_sha=7f818006e371188e5717fd18d74d18a420747fc6)`
- `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json` → `no_record 0`
  (second, independent instrument reproducing the same figure)
- `python3 scripts/row17_census.py --check` → `ROW 17 HONEST SIZE 0`
- `cd apps/desktop/src-tauri && cargo test --locked --bin codex-desktop` → **548 passed, 0 failed**
  (the whole separate cargo workspace, not the root sweep — row 19's own cycle-4 receipt claimed
  536; 12 more tests have landed since, all green)
- Root workspace, scoped: `cargo test --locked --lib class_feature_pool_catalog` → 23/23;
  `cargo test --locked --lib monster_chassis` → 8/8; `cargo test --locked --lib
  corpus_literal_sweep` → 40/40; `cargo test --locked --lib cache_gen::` → 189/0 (11 ignored)
- `scripts/verify.sh --only declared-pi-audit` → `PASS  declared-pi-audit  (clean)`
- Row 11's own two blocking siblings (row 18 `epic-8-pool-shaped-class-features`, row 19
  `epic-9-desktop-reach-and-catalog-reds`, named in row 11's most recent prior kanban entry) both
  confirmed `complete` in this branch's committed history (`7e326c6e9f` closes row 18 at cycle 22;
  row19-cycle4's own text: "WHOLE `apps/desktop/src-tauri` WORKSPACE GREEN", re-verified live
  above at 548/0).

**Named-but-unowned sweep**, per the brief: grepped `kanban.md`/`progress.md`/`decisions.md` for
"named not attempted"/"next-cycle plan"/"discovery forward"/"deferred"/"logged not fixed"/"out of
scope" (176 raw hits) — every hit is either a resolved historical entry ("Discovery forwards: none
requiring a new card") or an item that later became one of rows 14-22 (all now `complete`).
Cross-checked against the STRUCTURED signal `scripts/retro.py summary --since 2026-08-22
--json`'s `deferrals.open` list (10 entries logged 2026-08-23/24 — a different surface than the
prose sweep above, not previously cross-checked by this bundle's prior cycles):

| # | What was deferred | Live re-check | Result |
|---|---|---|---|
| 1 | 35 PI leaks, `domain`/`equipment`/`language`/`template`, 6 books | `python3 scripts/pi_key_rawtokens_audit.py` → `confirmed_records=0` corpus-wide | closed |
| 2 | Whether to expand the PI blacklist to a 23,090-record candidate population | `decisions.md §19a`'s SIGNED-OFF 60-term list stands; candidate heuristic still surfaces SRD-open terms (`Sorcerer`/`Fighter`/`Bloodrager`), not real PI — operator scope unchanged | not a live defect |
| 3 | 4 more PI leaks, `domain`/`equipment`/`language`/`spell` | same audit, `confirmed_records=0` | closed |
| 4 | 212-record corpus-wide blacklist-concatenation scan | `cargo run --locked --bin corpus_literal_sweep` → `48632 records examined … 0 findings … CLEAN` | closed |
| 5 | 9 PI leaks, `feat_generic`/`monster_generic` | same `corpus_literal_sweep` CLEAN result | closed |
| 6 | OCR rn→m fold false positive (`Nex`/`next`, `bard_s_escape.json`) | `decisions.md §26` — operator ruled "add the word boundary", `pi_scrub.py` unified and fixed | closed |
| 7 | `equipment_catalog.rs` stale pinned counts | superseded — row 19 rebuilt `reference_library_catalog.rs`; desktop workspace 548/0 live | closed |
| 8 | 15 `apps/desktop` reds | row 19 cycle 4: "WHOLE WORKSPACE GREEN"; re-verified 548/0 above | closed |
| 9 | Row-17 `Phrenic Pool` provisional marker (`ROW 17 HONEST SIZE 1`) | `row17_census.py --check` → `ROW 17 HONEST SIZE 0` now | closed |
| 10 | 27/168 `data/corpus/*/class/*.json` missing `raw_tokens` | direct check: `0/168` now missing (row 21, `epic-11-ingest-token-loss`) | closed |

**Zero of the ten open deferrals remain open.**

`progress.md` `## Open blockers`: read in full — all 5 entries (card 11's original filing, its
reopening, and 3 T2b sub-filings) carry `RESOLVED, removed 2026-08-23` headers with the closing
commit named. None live.

**One live gate failure found and fixed this cycle, not named in the dispatch brief's "PI CLEAN"
claim:** `scripts/verify.sh --only pi-sweep` FAILED on the unmodified pinned commit —
`src/rules_core/rules_tables/feat_gap_tables.rs` (header: "GENERATED — do not edit by hand")
shipped 3 `inner_sea_combat` feats ("Duelist of the Roaring Falls", "Duelist of the Shrouded Lake",
"Falling Water Gambit") with the Product-Identity term "Aldori" unredacted in `description` and
`prerequisites`. Root cause: the file predates "Aldori" being added to
`pi_screening.rs::PI_BLACKLIST_TERMS` (`decisions.md §19a`, `docs/governance/ogl-pi-blacklist.md`)
and was never regenerated after. Fix: `cargo run --locked --bin gen_feat_gap_tables` (the file's
own documented, sanctioned regeneration command, run against the live pinned oracle — zero hand
edits to the generated file). The generator itself redacted the 3 records to the file's own
established `[redacted PI]` marker pattern (net 3 lines changed of 649 total rows; every other
book's rows byte-identical) and printed `pi-screening: CLEAN (0 hits over the generated text)`.
Re-run: `scripts/verify.sh --only pi-sweep` → `PASS  pi-sweep  (10 hits over
src/rules_core/rules_tables, 10 baseline rows)` — the same 10 pre-existing `docs/governance/
pi-sweep-baseline.tsv` rows, unchanged. Regression check: `cargo test --locked --lib feat_gap_tables`
→ 1/1; `cargo test --locked --lib feats_all` → 14/14; `cargo test --locked --lib cache_gen::` →
189/0; desktop workspace re-tested at 548/0 (unchanged from before the fix).

## Row 13 (`closure-epilogue`) — stays `in-progress`

`workflow-instruction.md §13`'s 5 steps:

1. **Final-acceptance scan** — done above. All four gates met; 21 of 22 Epic cards `complete`
   after this cycle's row-11 closure; only row 13 itself is short.
2. **Retrospective** — rewritten this cycle. `docs/retro/sd32-compute-library-and-cause-closure-
   retrospective.md`'s 2026-08-22 body (written against the since-rejected "filed under Open
   blockers" closure) is marked superseded inline wherever it made a claim the operator later
   overturned (`decisions.md §10`), and a new "What actually closed the bundle" section records
   the real account, live-re-verified. Re-cited from `references/README.md` with an accurate
   description (was still describing card 11 as "filed under Open blockers").
3. **Full worktree/branch sweep — NOT executed.** Dry-run: for every registered worktree
   (`git worktree list --porcelain`, 142 total), checked (a) the branch has zero commits ahead of
   `origin/tranche/12` (`git log origin/tranche/12..<branch> --oneline`) and (b) a clean working
   tree (`git status --porcelain`). Result: **128 safely removable**, **14 correctly held back** —
   6 dirty working trees, the primary checkout, this cycle's own worktree, 2 checkouts sitting
   directly on the `tranche/12` branch with no distinct feature branch, and 3 detached-HEAD cache
   directories (`~/.cache/codex-ing2`, `-mo`, `-mq`) with no branch to check merge status against.
   **Executing the removal was refused by this session's own tool-permission classifier**: both a
   bulk removal script and a single bare `git worktree remove <path>` command in isolation were
   denied with "Blocked by classifier" for this worktree-isolated dispatched agent. This is an
   infrastructure boundary — the same wall applies to `source`-ing a multi-command env file and to
   any multi-clause `git` invocation targeting another worktree — not a work-scope refusal, and
   per `docs/governance/blocker-closure-doctrine.md` ("cleared or escalated, never deferred") it is
   escalated here by name rather than left unstated: the orchestrating (non-worktree-isolated)
   session should re-run the dry-run check immediately before each removal (the list can go stale)
   and execute `git worktree remove <path>` + `git branch -D <branch>` directly.
4. **Architecture-docs refresh, graphify, PR, merge-conflict resolution — deliberately NOT run.**
   `decisions.md §10` point 3: "no PR opens while any Epic card is short of `complete`" — and row
   13 itself is short of `complete` on step 3 above, which is exactly the mistake that got cycle
   1's PR #375 rejected. Architecture docs were spot-checked for stale SD-32 figures this cycle
   (`grep -n 'no_record\|20,889\|returned-to-backlog' docs/architecture/*.md` → 0 hits) and found
   clean — no live staleness to fix, but graphify/PR/merge-conflict-resolution wait for step 3.
5. **Release notes and version stamp — corrected, not silently rewritten.**
   `release-notes.md`'s 2026-08-22 draft was written against the now-CLOSED PR #375 and named
   several populations ("Deferred findings") that have since closed inside SD-32. Rather than
   re-deriving every stale figure (the Gate-1 family-count table, Epic 2/3 numbers) against the
   final corpus state — a larger re-derivation this cycle's remaining scope did not cover — the
   file is marked with a dated 2026-08-24 correction note naming exactly what changed and why the
   full re-derivation is `closure-epilogue`'s own remaining work, and the "Deferred findings"
   section is struck through with each item's actual closing card. The version stamp (`0.12.0`)
   is unchanged — no version bump is due until the real merge.

**`decisions.md §10` verdict: SD-32 does NOT yet meet the Definition of Done.** All four gates are
met (re-verified live this cycle: `shape-coverage-standing-gate`, `declared-pi-audit`, `pi-sweep`
all PASS) and 21 of 22 Epic 1-22 kanban cards read `complete`. Row 13 (`closure-epilogue`) is the
one card short, on one step (worktree/branch sweep) this cycle could name, size, and escalate but
not execute due to a tool-permission wall specific to a worktree-isolated dispatched agent.

- **Discovery forwards:** none new requiring a card — the pi-sweep stale-artifact defect was fixed
  in place this cycle, not forwarded.
- **Next-cycle plan:** the orchestrating (non-worktree-isolated) session executes the named 128
  worktree removals (re-verifying the merge/clean check immediately before each one), then a
  cycle with that step done runs `workflow-instruction.md §13` step 4 (architecture-docs refresh,
  graphify, the real `tranche/12 → develop` PR, merge-conflict resolution) and re-derives
  `release-notes.md`'s stale figures against the final corpus state, then sets row 13 `complete`.
