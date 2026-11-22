# Cycle sd32-five-unverified-deferrals — verify-or-close the 5 deferrals the closure lane never checked

- **Card ID:** `epic-5-protective-sweep` — correction pass, per `docs/governance/blocker-closure-doctrine.md`
  (a DoD blocker is cleared or escalated, never deferred).
- **Territory:** `src/bin/gen_book_cache.rs`, `scripts/census_independent.py`, `src/rules_core/**`
  (class_tables / archetype-traversal areas). Did NOT touch `scripts/retro.py` (lane A) or
  `scripts/observer/pf1e_dashboard_producer.py` (lane C) — used the former's CLI only, per the
  standing rule ("emit retro events as they happen"), never edited it.
- **Commit SHA:** see push receipt (this cycle's own commit)
- **Files touched:** `src/bin/gen_book_cache.rs` (fix + a mislabeled-comment correction).
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`). The
  `operator-supplied/pcgen` slot was EMPTY at cycle start (only its README, git-ignored per
  `artifacts/corpus/README.md`) — fetched fresh this cycle via `scripts/fetch-pcgen-oracle.sh`
  before any live reproduction; `pcgen-oracle: OK 7f818006e371188e5717fd18d74d18a420747fc6`.

## Environment note (own mistake, corrected before any figure below)

First live-reproduction attempt against `gen_book_cache advanced_race_guide` set only
`PCGEN_CORPUS_ROOT`, not `PCGEN_CORPUS_ROOT_ARG` — `arg_corpus_root()` (the function
`gen_advanced_race_guide`'s own spell/equipment/feat loop uses) reads the latter only, and without
it silently fell back to `$HOME/workspace/repos/pcgen/data/...` — the FORBIDDEN oracle path this
brief explicitly rules out. This was caught before drawing any conclusion (the corpus tree at the
pinned repo-local slot didn't even exist yet at that point, so the fallback firing was visible
immediately) and every figure reported below was re-run with `PCGEN_CORPUS_ROOT_ARG` correctly set
to the pinned, repo-local, fetched oracle. `data/corpus` was reverted (`git checkout --
data/corpus`) after every single run in this cycle, never left dirty, never committed.

---

## 1. `gen_book_cache.rs` self-erasure fix (`index_existing_records_by_key`)

**ALREADY RESOLVED at `origin/tranche/12` HEAD (`6e8b4bf9b4`) — plus one genuinely NEW, still-live
sibling defect found and fixed this cycle.**

The literal function name `index_existing_records_by_key` from the abandoned
`worktree-wf_c1156061-e3f-3` branch does not exist anywhere in this repo (`grep` returns nothing) —
but the underlying vulnerability that branch fixed (unconditional `remove_dir_all` wiping
`gen_pathfinder_unchained`/`gen_advanced_race_guide`/`gen_monster_book`/`gen_companion_book`'s own
output every run) was independently closed by commit `3b470c56f9` ("close 7 self-erasing Rust
generators, Epic 5 protective sweep"), which landed BEFORE `boundary-branch-review` (`8d1e1dd786`)
ran and wrongly concluded the branch's fix was "genuinely unmerged, still-needed work" — that
review's own check (`grep -c index_existing_records_by_key`) tested for a specific function NAME
from the abandoned branch, not for the actual defect (unconditional `remove_dir_all`). **Instrument
correction**: `boundary-branch-review`'s disposition of `worktree-wf_c1156061-e3f-3` was itself
wrong — the defect it flagged as unmerged was already closed, under different code, before that
review ran.

Confirmed by content, not commit count: `grep -n "remove_dir_all" src/bin/gen_book_cache.rs` today
returns 3 hits, all inside `//` doc comments describing PAST behaviour, zero live calls. All 4
previously-vulnerable functions now use an `out_path.exists()` guard (never rewrite an existing
record) plus `ultimate_equipment::remove_stale_owned_files` (citation-scoped stale-key removal) —
5 call sites across the file, `grep -c remove_stale_owned_files` = 5.

**The NEW defect (found by live reproduction, in scope, fixed this cycle):**
`gen_advanced_race_guide()`'s own `feat/` stale-sweep call carried a comment claiming "Single writer
of `pathfinder_unchained/feat/`" (copy-pasted from a DIFFERENT function's identical call, never
re-verified for `advanced_race_guide`) and used an unscoped `|_p,_l| true` predicate.
`cache_gen::feat_gap::FEAT_GAP_BOOKS` DOES register `advanced_race_guide`, writing gap-filled feat
records into the SAME `data/corpus/advanced_race_guide/feat/` directory, parsed from the SAME
`arg_feats.lst`.

**RED, live, against the correctly-set pinned oracle:**
```bash
PCGEN_CORPUS_ROOT=<pinned>/data PCGEN_CORPUS_ROOT_ARG=<pinned>/data/pathfinder/paizo/roleplaying_game/advanced_race_guide \
  "$CARGO_TARGET_DIR/debug/gen_book_cache" advanced_race_guide
git status --porcelain -- data/corpus | grep -c '^ D '
```
→ **48** deletions, e.g. `data/corpus/advanced_race_guide/feat/angelic_flesh_brazen.json`
(`source.path` = `arg_feats.lst`, a flat top-level file — never under this generator's own
`feat/<category_slug>/` nesting, confirming it's a `feat_gap` record). Reverted immediately
(`git checkout -- data/corpus`), never committed.

**The fix:** removed the stale-key sweep for `advanced_race_guide/feat/` entirely, keeping only the
`exists()`-guard — the same carve-out this exact function already uses for its OWN `equipment/`
directory (to avoid deleting `cache_gen::equipment_gap`'s records), for the identical reason: this
generator can never regenerate a sibling generator's gap-filled records, so it must never delete
one either. Also corrected a second, cosmetic-only mislabeling of the same copy-pasted comment on
`gen_pathfinder_unchained()`'s own `equipment/` call site (said `advanced_race_guide`, is actually
inside `gen_pathfinder_unchained`; the underlying verification was already correct, only the book
name in the comment was wrong).

**GREEN, live, same oracle, same command, post-fix:** `git status --porcelain -- data/corpus | grep
-c '^ D '` → **0**. Only `LICENSE.json` timestamp changed (`git checkout -- data/corpus` after every
run). Re-ran `gen_pathfinder_unchained` too (the sibling call site whose comment I also corrected) —
0 deletions, confirms no regression.

**Full RED→GREEN pair also reproduced** by temporarily restoring the pre-fix file
(`git show HEAD:src/bin/gen_book_cache.rs`), rebuilding, re-running (48 deletions again, reverted),
then restoring the fix and rebuilding again (0 deletions) — the fix, not an unrelated corpus
change, is what moves the number.

`cargo test --locked --bin gen_book_cache` — 5/5 pass (this binary's own `#[cfg(test)]` module,
unrelated to this change's call sites but confirms no compile/logic regression).

**Sweep for the same shape elsewhere:** cross-checked every `feat_gap.rs`-registered `book_id`
(19 books) against `gen_book_cache.rs`'s own feat-writing functions (`gen_pathfinder_unchained`,
`gen_advanced_race_guide`) — `advanced_race_guide` is the only overlap; `monster_codex` (also in
`feat_gap`'s list) is handled by `gen_monster_book`, which never writes `feat/`. No further
collision found in this file.

## 2. `class_feature` kind classification in `scripts/census_independent.py`

**ALREADY RESOLVED — deliberate design, verified zero-gap, not a live defect.**

The deferral's shape: `census_independent.py`'s own `DISCOVERY` doc comment (lines 86–93) states
AT-32-G0-002's ten-kind list omits `class_feature` (the single largest kind), and the walker
deliberately does NOT guess which of the ten kinds a `class_feature` row belongs to — every
`*_abilities_class*` file is filed under `kind_unenumerable["class_feature"]`, named and counted,
never force-fit. This is the correct behaviour per AT-32-G0-002's own explicit requirement
("named and counted — not pretended to be zero"), not a defect to fix.

Kanban row 15 (`census-scope-closure`, status `complete`) closed the 27,847 kind-unenumerable
population (class_feature's share included) to zero unexplained gap.

**Re-derived live, this cycle**, against the freshly-fetched pinned oracle:
```bash
python3 scripts/census_independent.py --pcgen-root "$PCGEN_CORPUS_ROOT" \
  --inventory docs/work-inventory.json --output /tmp/census_diff.json
```
→ `discovered=186 in_scope=38 excluded=148 unexplained=0`. `kind_unenumerable.class_feature` =
**18,191** (named and counted, exactly as designed), `total_counted_units=36,883`. Zero-gap
confirmed live, not requoted from a prior receipt.

(Cross-check note, not a defect: `docs/work-inventory.json`'s own
`totals.by_kind.class_feature=18,043` differs from the census walker's 18,191 by 148 — a
denominator-shape difference between "LST rows counted by the independent walker" and "distinct
units the inventory producer ingested," not something this deferral asked me to reconcile and not
evidence the classification itself is wrong.)

## 3. T7 — shallow single-hop archetype-grant traversal, 4 units

**ALREADY RESOLVED — landed via `epic-2-t7-t8` (`caaef7762`) / `epic-2-cause-closure` cycle 1,
in-territory (`src/rules_core/pilot_compute/class_feature_grant_consumer.rs`).**

D12/T7's fix (`RawGrantFact.gate` field, `resolvable_grants`'s single-hop archetype-grant guard —
"an uncorroborated bare-`PRECLASS:`-only pair must never resolve") is present at current HEAD:
`grep -n "T7/D12" src/rules_core/pilot_compute/class_feature_grant_consumer.rs` → 6 hits, including
the dedicated regression test
`a_bare_preclass_only_pair_with_no_mod_row_corroboration_is_refused` and doc comment
`"an uncorroborated bare-PRECLASS: pair must never resolve -- T7/D12 regression"`.

**Re-ran the module's full test suite live, this cycle:**
```bash
cargo test --locked --lib rules_core::pilot_compute::class_feature_grant_consumer
```
→ **31 passed; 0 failed**, including the T7 regression test and
`the_live_scale_of_this_waves_widening_is_measured_and_pinned`.

## 4. T8 — `wiring_class`-vs-status classifier blind spot, 12 CRB `class_feature` units

**ALREADY RESOLVED — out of my territory (`scripts/observer/pf1e_dashboard_producer.py`, lane C),
verified present, not touched.**

Landed via `epic-2-cause-closure` cycle 2 (`e3f3559dd`): `compute_wiring_class_summary()`
reclassifies exactly the 12-unit predicate (`kind=='class_feature' and wiring_class=='display' and
status=='grounded' and evidence=='explanation_id_observed_in_a_real_computation'`) from `display`
to `computed`, generalized from a hardcoded 12-id set to a class predicate (proved set-equal to the
hardcoded list on today's corpus). Confirmed present at HEAD:
`grep -c classifier_reclassified_units scripts/observer/pf1e_dashboard_producer.py` → 5 hits
(field definition, population, threading). Per this cycle's territory boundary, this file is lane
C's — I verified by reading, did not re-run or edit it.

## 5. 18 real base classes without a `class_tables()` row

**ALREADY RESOLVED — net-new chassis construction landed and wired, population corrected 18→20
(already logged as a `retro.py correction` by the landing cycle, re-confirmed live this cycle).**

`src/rules_core/pilot_compute/untabled_base_class_chassis.rs`'s own doc comment: "Population: 20,
not 18 — corrected, not silently substituted" — `scripts/census_untabled_base_classes.py` (re-run
by that cycle against the pinned oracle) found 20 real (`TYPE:Base.PC`) base classes with no
existing `compute_class_chassis` dispatch arm: Aegis, Antipaladin, Cryptic, Dread, Kineticist,
Magus, Marksman, Medium, Mesmerist, Occultist, Psion, Psychic, Psychic Warrior, Shifter, Soulknife,
Spiritualist, Tactician, Vigilante, Vitalist, Wilder.

**Re-derived live, this cycle**, against the freshly-fetched pinned oracle:
```bash
PCGEN_CORPUS_ROOT=<pinned>/data python3 scripts/census_untabled_base_classes.py
```
→ `population=20`, rewrote `tests/fixtures/rules_core/untabled-base-class-chassis.json` —
`git status --porcelain` on that path: **empty** (byte-identical to committed, deterministic
re-derivation, no drift).

**Wired into production, not just a fixture:** `compute_class_chassis`'s dispatch chain in
`pilot_compute/mod.rs` (~line 26498) falls through to
`untabled_base_class_chassis::resolve(&class_level.class_id, class_level.level)` as a real arm,
reusing the CRB table's own `base_attack_bonus`/`save_bonus` formulas (SD-18-verified) rather than
re-deriving a second copy.

**Re-ran the module's tests live, this cycle:**
```bash
cargo test --locked --lib untabled_base_class_chassis
```
→ **11 passed; 0 failed**, including `registry_loads_all_20_corpus_derived_entries` and
`registry_has_no_overlap_with_any_already_dispatched_class_id` (the mutual-exclusion proof against
every other already-dispatched chassis).

This is genuine net-new capability that was already built, not a defect needing escalation — no
"if this is net-new, escalate" branch is needed since it is already complete and wired.

---

## Summary

| # | Item | Disposition |
|---|---|---|
| 1 | `gen_book_cache.rs` self-erasure (`index_existing_records_by_key`) | ALREADY RESOLVED (instrument correction on `boundary-branch-review`'s own disposition) + **1 new sibling defect found live, RESOLVED THIS CYCLE** (`advanced_race_guide/feat/` vs `cache_gen::feat_gap`) |
| 2 | `class_feature` kind classification, `census_independent.py` | ALREADY RESOLVED (deliberate design, zero-gap re-confirmed live) |
| 3 | T7 shallow single-hop archetype-grant traversal (4 units) | ALREADY RESOLVED (31/31 tests GREEN, live) |
| 4 | T8 `wiring_class`-vs-status blind spot (12 CRB units) | ALREADY RESOLVED (verified present, lane C's file, not touched) |
| 5 | 18 base classes without `class_tables()` | ALREADY RESOLVED (net-new, 20 not 18, wired, 11/11 tests GREEN, live) |

Four of five were already closed by prior cycles this closure lane never checked; the fifth
(item 1) surfaced one genuinely new, still-live self-erasure defect while verifying — found, RED-
reproduced against the correctly-configured pinned oracle, fixed, GREEN-reproduced, reverted-and-
clean at every intermediate step, never touched `data/corpus` in the final committed state.

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Retro event:** `docs/retro/events/sd31-transcribe.jsonl` (actor slot picked up by the shared
  `RETRO_ACTOR` default in this checkout) — 1 `correction`, subject `sd32-five-unverified-deferrals`.
- **Status:** complete
- **Discovery forwards:** none requiring a new card — all 5 items dispositioned above; the new
  `advanced_race_guide/feat/` defect is fixed inline, not forwarded.
- **Next-cycle plan:** none required.
