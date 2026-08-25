# Cycle cross-generator-self-erasure-fix — Epic 5 protective sweep (correction) / Criterion: no generator deletes a record it does not own

- **Card ID:** `epic-5-protective-sweep` (card 1) — correction to a `complete` card, per `AGENTS.md` Blocker Discipline: cleared, not deferred.
- **Commit SHA:** (this cycle's own commit, see push receipt)
- **Files touched:**
  - `src/rules_core/cache_gen/ultimate_equipment.rs` — `remove_stale_owned_files` signature widened to take an `owns_citation: &dyn Fn(&str, u32) -> bool` predicate; new tests.
  - `src/rules_core/cache_gen/spell_lane_dump.rs` — call site now passes a citation-line-scoped ownership predicate.
  - `src/bin/gen_book_cache.rs` (5 call sites) — updated to the new signature (unscoped `|_p,_l| true`, verified single-writer per directory).
  - `src/bin/gen_core_rulebook_cache.rs` (1 call site) — same.

## The defect

`gen_cache_spell_lane_dump` (`src/rules_core/cache_gen/spell_lane_dump.rs`) and
`gen_cache_spell_mod_access` (`src/rules_core/cache_gen/spell_mod_access.rs`) both write
`data/corpus/{occult_adventures,ultimate_magic}/spell/*.json`, both parsing the **same literal LST
file** (`oa_spells.lst`, `um_spells.lst`). `spell_mod_access` writes one JSON record per `.MOD`
class-access row; a `.MOD` row's stripped key is frequently **identical** to its base spell's own
key (`Occultist Spell ~ Accelerate Poison.MOD` → key `Accelerate Poison`, widening the
already-published spell of that name). `spell_lane_dump`'s own `remove_stale_owned_files` call used
Card 1's key-only ownership predicate (`data.key` absent from this run's `current_keys` ⇒ delete).
Because `spell_mod_access`'s `.MOD`-row keys are absent from `spell_lane_dump`'s own `current_keys`
set (it never iterates `.MOD` rows), every `spell_mod_access` record in the shared directory was
misread as "stale output of mine" and deleted.

**Card 1's own sweep (`cycle-1_cycle_receipt.md`) audited the *within-generator* self-erasure shape
(a generator wiping its own prior output) across all 29 generators, including this exact function.
It did not check the *cross-generator* shape** (two generators sharing one output directory) because
at sweep time `spell_mod_access` either did not yet write to this directory or the sweep's own
per-generator framing did not compare directory writers against each other. This cycle is that
missed check, done retroactively.

## RED — reproduced live against the real pinned corpus, not asserted

```bash
export PCGEN_CORPUS_ROOT=<worktree>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data
cargo build --locked --bin gen_cache_spell_lane_dump
"$CARGO_TARGET_DIR/debug/gen_cache_spell_lane_dump"
git status --porcelain | grep -c '^ D '
```
→ **1580** deletions, before this fix:
```
git status --porcelain | grep '^ D ' | sed -E 's#^ D data/corpus/([^/]+)/.*#\1#' | sort | uniq -c
   1417 occult_adventures
    163 ultimate_magic
```
matching the brief's escalated figure exactly. Confirmed the deleted files were genuine
`spell_mod_access` `.MOD` records (e.g. `data/corpus/occult_adventures/spell/ablative_barrier.json`
carries `data.name`/`data.raw_tokens`, the `SpellModAccessData` shape, `source.path` = `oa_spells.lst`
line 4021-class — the `.MOD` row's own citation, never a `spell_lane_dump`-cited line). Reverted
immediately (`git checkout -- data/corpus/occult_adventures data/corpus/ultimate_magic`) — **not
committed.**

Unit-level mutation proof, `ultimate_equipment::tests::an_unscoped_key_only_predicate_reproduces_the_incident`:
re-runs the identical two-record scenario with the pre-fix unscoped predicate (`|_p,_l| true`) and
asserts the sibling record IS deleted — proving the surviving-record assertion in the sibling test is
load-bearing, not vacuous.

## The fix (by class, not by book-name exclusion)

`remove_stale_owned_files(dir, current_keys, owns_citation: &dyn Fn(&str, u32) -> bool)`: a file is
now removed only when **both** (a) its `data.key` is absent from `current_keys` **and** (b) the
caller's `owns_citation` predicate accepts its `(source.path, source.line)`. A record whose
`source.path`/`source.line` cannot be read is never deleted (fails closed, `decisions.md §1a`).

`spell_lane_dump`'s own ownership boundary is **the citation line**, not the file or the key: its own
`base_declaration_lines()` already excludes every `.MOD`/`.COPY=` row by construction (pre-existing,
proven by `base_declaration_lines_excludes_mod_and_copy_rows`), so the full value-set of that map is
exactly "every line this generator's own parse of this file would ever cite." A `.MOD` row's citation
line can never be a member, so the predicate structurally cannot match a `spell_mod_access` record —
even though the file and the key both collide. No book-name exclusion list; the same predicate shape
applies to any future book either generator adds.

`ultimate_equipment`'s own call site (the only other production caller) keeps a coarser
citation-**path**-prefix predicate (`UE_DIR`), because it is verified the sole writer of
`data/corpus/ultimate_equipment/equipment/` — a single-writer directory does not need line-level
scoping, but the predicate shape (citation-based, not key-based) is the same discipline.

## GREEN — re-proven live post-fix

```bash
cargo build --locked --bin gen_cache_spell_lane_dump --bin gen_book_cache --bin gen_core_rulebook_cache --lib
"$CARGO_TARGET_DIR/debug/gen_cache_spell_lane_dump"
git status --porcelain | grep -c '^ D '
```
→ **0**. `data/corpus/occult_adventures/spell` file count unchanged (1561), `ultimate_magic/spell`
unchanged (432); `git diff --stat data/corpus` empty. The `.MOD` record
(`data/corpus/occult_adventures/spell/ablative_barrier.json`) is byte-identical to `HEAD` after the
run (`diff` clean).

`cargo test --locked --lib rules_core::cache_gen` — all tests GREEN (see push receipt for the exact
count), including the two new direct-guard tests
(`a_sibling_generators_record_sharing_the_same_key_and_file_survives`,
`this_generators_own_stale_record_is_still_removed`) and the mutation proof
(`an_unscoped_key_only_predicate_reproduces_the_incident`).

## Sweep for the same shape elsewhere (task item 3)

Every `remove_stale_owned_files` call site (the only self-erasure-guard function in the codebase) and
every directory written by more than one generator:

| Directory (book/kind) | Writers | Any writer calls `remove_stale_owned_files`? | Verdict |
|---|---|---|---|
| `{occult_adventures,ultimate_magic}/spell/` | `spell_lane_dump`, `spell_mod_access` | `spell_lane_dump` only | **FIXED this cycle** (was the live landmine) |
| `advanced_players_guide/spell/` | `spell_mod_access`, `apg.rs` (its own `spell_dir`) | neither | Dormant — `apg.rs` never calls the guard against its own dir either, so no deletion risk exists today; a future guard added to either would need this same citation-line treatment. Reported, not fixed (out of scope: no deletion logic present to fix). |
| `{core_rulebook,ultimate_psionics}/feat/` | `feat_gap.rs`, `hand_authored_feat_dump.rs` | neither | Dormant, same reasoning — both use no-clobber `write_json` only, no stale-removal call exists. Reported. |
| `pathfinder_unchained/feat/`, `advanced_race_guide/{spell,equipment}/`, `<monster-book>/{monster,monster_ability}/`, `<companion-book>/companion/`, `core_rulebook/spell/`, `ultimate_equipment/equipment/` | single writer each (verified against every other `cache_gen` module's own book-list constants) | yes (all 6 `gen_book_cache.rs`/`gen_core_rulebook_cache.rs` sites + `ultimate_equipment`'s own) | Safe — updated to the new signature with an unscoped predicate (`|_p,_l| true`), each with a comment naming the verification that no sibling writes there. |
| `data/class_feature_grants/<book>/` | `class_feature_grants.rs` only (`class_feature.rs` only *reads* this tree) | N/A — this generator does a full `remove_dir_all` + full regenerate every run, not an incremental stale-file scan | Safe by construction: single owner, always-complete regeneration, no partial-run staleness window. |
| `advanced_race_guide/race_trait/` | `ingest_races.rs`, `ingest_race_traits.rs` | N/A (pre-`remove_stale_owned_files`, older per-binary "scoped clear" idiom) | **Already fixed in a prior bundle** (`SD-31-E6-F4-003`) — both binaries' own doc comments describe this exact cross-generator mutual-destruction hazard and clear only records whose on-disk shape (`is_racial_default` field) matches their own known output. Confirms the defect class, not a new instance. |

No other `remove_dir_all`/`remove_file` call in `src/rules_core/cache_gen/*.rs` or `src/bin/*.rs` is a
production write-path stale-cleanup call; every remaining hit is `#[cfg(test)]` scratch-directory
teardown (verified by locating each hit relative to its file's `#[cfg(test)]` marker).

## Acceptance criterion

A generator sharing an output directory with a sibling generator must never delete a record it does
not own, proved live against the real pinned corpus (not a synthetic fixture) with the exact
population the escalation named (1,580 / 1,417 + 163), and proved not to regress by a mutation test.

## Corpus SHA

`7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`) — unchanged; this cycle
writes no new corpus content, only prevents wrongful deletion.

- **Identifier audit result:** OK_NO_BUNDLE_TAGS (see push receipt)
- **Wired-integration audit result:** OK_NO_TOKENS (see push receipt)
- **Status:** complete
- **Notes:** No corpus data was harmed — every reproduction step reverted with `git checkout --` before commit; `git status --porcelain` confirmed zero pending corpus changes at commit time.
- **Discovery forwards:** the two "dormant" shared-directory rows above (`advanced_players_guide/spell/`, `{core_rulebook,ultimate_psionics}/feat/`) — no fix needed today (no deletion logic exists to misfire), but any future stale-cleanup added to either generator must use a citation-based predicate, not a key-only one.
- **Next-cycle plan:** none required for this defect; card 1 stays `complete` with this correction appended.
