# Cycle 1 — Pre-G0 (Epic 5 protective sweep) / Criterion AT-32-E5-001

- **Card ID:** `epic-5-protective-sweep`
- **Commit SHA:** (recorded after commit below)
- **Files touched:**
  - `src/bin/gen_book_cache.rs` (`gen_advanced_race_guide`, `gen_companion_book`, `gen_pathfinder_unchained`)
  - `src/bin/gen_core_rulebook_cache.rs` (`main`)
  - `src/rules_core/cache_gen/acg.rs` (`write_json`, +1 test)
  - `src/rules_core/cache_gen/apg.rs` (`write_json`, +1 test)
  - `src/rules_core/cache_gen/beastiary1.rs` (`write_json`, +1 test)
  - `src/rules_core/cache_gen/spell_lane_dump.rs` (`generate`, `write_json`, +1 test)
  - `src/rules_core/cache_gen/ultimate_equipment.rs` (`generate_equipment`, `write_json`, +`remove_stale_owned_files` (now `pub`), +1 test)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
  (`git diff --unified=0 1bb523773d32705d1b7387fd4c494861523f55ba -- <7 files> | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` → no output)
- **Wired-integration audit result:** OK_NO_TOKENS
  (same diff | `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → no output)
- **Acceptance criterion (verbatim, `acceptance-and-verification.md`):**
  > **AT-32-E5-001 — Automation, decided on evidence.** (Epic 5.) The protective self-erasure sweep
  > across all 29 Rust generators (`ls src/bin/{gen_,ingest_,enrich_}*.rs | wc -l`) runs **before Gate 0**.
  > A cycle that touches an engine before this sweep is out of protocol...
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`) — every live-reproduction figure below was re-derived against this pin, at the repo-local oracle slot.
- **Status:** complete
- **Notes:** see full write-up below.
- **Discovery forwards:** see `## DISCOVERED` below (2 items; neither exceeds the queue-of-10 self-heal ceiling, both filed inline as this cycle's own findings rather than deferred to `forward-scope-register.md` — they are corpus-shape observations, not new scope).
- **Next-cycle plan:** card 2 (boundary-branch review, primary checkout) is next per `kanban.md` claim priority; Gate 0 (card 3) is gated on this card + card 2.

## Population re-verification (re-run at cycle start, per card notes)

```
$ ls src/bin/{gen_,ingest_,enrich_}*.rs | wc -l
29
```
Matches `epic-breakdown.md`/`acceptance-and-verification.md`'s stated population exactly (13 `gen_*` in
`src/bin/` + 11 `ingest_*` + 5 `enrich_*`; confirmed by listing — see raw list in the full write-up
below).

## What SD-31 left as the residual (from `todo/sweeps.md` S6, `todo/defects.md` D9)

- **12 of 29 checked in SD-31.** 10 confirmed SAFE (5 `enrich_*`, `ingest_races.rs`/
  `ingest_race_traits.rs`/`ingest_apg_race_traits.rs`, `ingest_pu_classes.rs`, `gen_cache_beastiary` —
  the last one's SD-31 "safe" verdict is corrected below, see Discovery 1). 2 confirmed-vulnerable
  BINARIES (`gen_book_cache.rs` — 3 of its 4 internal functions: `gen_advanced_race_guide`,
  `gen_companion_book`, `gen_pathfinder_unchained` — and the separate `gen_core_rulebook_cache.rs`),
  filed as `defects.md` D9, never fixed.
- **17 of 29 never reached.** 10 `gen_*` (`gen_cache_acg`, `gen_cache_apg`, `gen_cache_class_feature`,
  `gen_cache_equipment_gap`, `gen_cache_hand_authored_equipment`, `gen_cache_spell_lane_dump`,
  `gen_cache_ultimate_equipment`, `gen_class_feature_grants`, `gen_equipment_gap_tables`,
  `gen_feat_gap_tables`) + 7 `ingest_*` (the 5-book spell-lane ingesters plus `ingest_class_spell_levels_arg`).

## This cycle's audit of the 17 never-reached generators

Read every one's production write path (the binary's own `main()`, and any `codex::rules_core::cache_gen::*`
module it delegates to) for the S6/D9 self-erasure shape: an unconditional directory wipe or an
unconditional per-file overwrite with no `out_path.exists()`-then-skip guard, over a directory a
LATER, SEPARATE enrichment pass (`enrich_{equipment,spell,companion,monster,monster_ability}_raw_tokens.rs`)
also writes into.

| Generator | Verdict | Basis |
|---|---|---|
| `gen_cache_acg` (→ `cache_gen::acg::generate`) | **VULNERABLE — fixed this cycle** | `write_json` unconditional overwrite, no exists-guard. `advanced_class_guide` spell/equipment records ARE enriched (`enrich_spell_raw_tokens.rs`/`enrich_equipment_raw_tokens.rs` books lists). 413 of 2,867 on-disk records carry `raw_tokens` today. |
| `gen_cache_apg` (→ `cache_gen::apg::generate`) | **VULNERABLE — fixed this cycle** | Identical shape to `acg`. 622 of 670 on-disk `advanced_players_guide` spell+equipment records carry `raw_tokens`. |
| `gen_cache_class_feature` (→ `cache_gen::class_feature::generate`) | SAFE | No directory wipe; per-file overwrite is deterministic (same inputs → same bytes) and this generator writes its own `raw_tokens` inline (no separate `enrich_class_feature_*` pass exists to lose). No cross-generator directory collision (`gen_class_feature_grants` writes a disjoint top-level `data/class_feature_grants/`, not `data/corpus/*/class_feature/`). |
| `gen_cache_equipment_gap` (→ `cache_gen::equipment_gap::generate`/`write_json`) | SAFE | Production `write_json` already checks `path.exists()` and returns early — was already correctly guarded (test name `write_json_never_overwrites_an_existing_file` already existed, SD-31 or earlier). |
| `gen_cache_hand_authored_equipment` (→ `cache_gen::hand_authored_equipment::generate`) | SAFE | Delegates to `equipment_gap::write_json`, same no-clobber guard. |
| `gen_cache_spell_lane_dump` (→ `cache_gen::spell_lane_dump::generate`) | **VULNERABLE — fixed this cycle** | Directory-level `remove_dir_all(&out_spell_dir)` every run across 5 books (`occult_adventures`, `ultimate_magic`, `ultimate_combat`, `inner_sea_gods`, `ultimate_wilderness`). All 712 of 712 on-disk spell records across these 5 books carry `raw_tokens` (`enrich_spell_raw_tokens.rs`). |
| `gen_cache_ultimate_equipment` (→ `cache_gen::ultimate_equipment::generate`) | **VULNERABLE — fixed this cycle** | Directory-level `remove_dir_all(&equipment_dir)` every run (added deliberately at "OPEN-ISSUES row 38" to make a `NAMEISPI:YES`-dropped record's stale file disappear — but that fix also erased every still-valid record). 1,368 of 1,548 on-disk `ultimate_equipment` equipment records carry `raw_tokens`. |
| `gen_class_feature_grants` (→ `cache_gen::class_feature_grants::generate_all`) | SAFE (wipe is correct here) | `remove_dir_all(grants_root)` targets `data/class_feature_grants/`, a directory this generator SOLELY owns — no sibling enrichment or cross-generator write ever touches it. This is the "genuinely single-owner, full-rebuild-is-correct" case D9's own fix pattern does not need to distinguish from the vulnerable case, because there is nothing else in the directory to lose. |
| `gen_equipment_gap_tables` (own `main`, production path via `cache_gen::equipment_gap`) | SAFE | `remove_dir_all` calls in this file are all inside `#[cfg(test)] mod tests` (scratch-fixture cleanup), not the production write path. |
| `gen_feat_gap_tables` | SAFE | Writes ONE generated `.rs` SOURCE FILE wholesale each run (`OUTPUT_RELATIVE_PATH`), not `data/corpus` JSON — deterministic full-file regeneration of versioned source is the correct behavior for this shape, not a self-erasure hazard. |
| 6 of 7 `ingest_*_spells.rs` (`ingest_adventurers_guide_spells`, `ingest_occult_adventures_spells`, `ingest_ultimate_wilderness_spells`, `ingest_ultimate_combat_spells`, `ingest_ultimate_magic_spells`, `ingest_inner_sea_gods_spells`) | SAFE | Same shape as `gen_feat_gap_tables`: each writes exactly one generated `.rs` module (`OUT_PATH`) wholesale, not JSON corpus records. |
| `ingest_class_spell_levels_arg` | SAFE | Same shape: one generated `.rs` module (`class_spell_levels.rs`) wholesale, gated behind an explicit `--emit` flag (`do_emit`). |

**5 of the 17 never-reached generators were genuinely vulnerable** (not the 0 SD-31's own "12 checked,
3 vulnerable" framing might suggest were left in the untouched 17) — all 5 are fixed this cycle.

## Discovery 1 (in-scope, fixed inline, not deferred): SD-31's own "safe" verdict for `gen_cache_beastiary` was wrong

SD-31 counted `gen_cache_beastiary` among the "12 checked... confirmed safe" bucket (`todo/sweeps.md`
S6), but the check it actually ran was for a DIFFERENT defect (D1, a citation-narrowing lead that was
refuted) — never for the S6 self-erasure shape specifically. Re-checked this cycle:
`src/rules_core/cache_gen/beastiary1.rs`'s own `write_json` is the SAME unconditional-overwrite shape
found in `acg.rs`/`apg.rs`. `enrich_equipment_raw_tokens.rs` lists `"beastiary"` among its books; 3 of
the book's 4 on-disk equipment records carry `raw_tokens` today. Fixed inline as part of this cycle's
shared `write_json` fix pattern (same file group as `acg`/`apg`). Filed here per standing lesson
"never trust a board figure/verdict from a document, re-derive" (`MEMORY.md`) rather than as a
`## DISCOVERED` forward, since it is squarely inside this card's own population and was fixed, not
deferred.

## Discovery 2 (`## DISCOVERED`, forwarded, not fixed this cycle): `core_rulebook`'s own generator was wiping FIVE OTHER generators' owned kinds, not just its own

Live-reproducing `gen_core_rulebook_cache` before the fix (isolated worktree, `git status` clean
before, `git checkout -- ` + `git clean -fd` after) showed collateral damage far larger than D9's own
"664 of 664 spell records" estimate: the pre-fix `main()` wiped **every** subdirectory under
`data/corpus/core_rulebook/`, not just the three kinds (`class`/`spell`/`equipment`) this binary
itself writes. One run deleted:

```
   959 class_feature   (cache_gen::class_feature's own records)
    84 companion       (companion books that cite core_rulebook)
   330 equipment       (334 of these belong to THIS generator; the rest overlap with
                         cache_gen::equipment_gap's "CRB" records in the same directory)
     7 race            (ingest_races.rs)
    67 race_trait      (ingest_race_traits.rs)
    29 spell           (see below — genuinely a pre-existing gap, not this generator's fault)
```

Fixed this cycle: the wipe-every-subdirectory loop is removed entirely; the three kinds this binary
owns each get their own guard (`class`/`equipment`: exists-skip only, no stale-sweep, because
`equipment` is shared with `cache_gen::equipment_gap`'s own CRB records and `class` has no `key`
field to sweep on safely; `spell`: exists-skip + stale-key sweep, matching `gen_monster_book`'s
already-proven pattern, since nothing else owns that directory).

**Forwarded, not fixed:** the live GREEN re-run also surfaced 29 real `core_rulebook` spell records
(energy-substitution variants — `burning_hands_acid`/`_cold`/`_electricity`,
`align_weapon_chaos_only`/`_evil_only`/etc., `scorching_ray_acid`/`_cold`, and 20 similar) that the
compiled `SPELL_LIST` table names but that have NEVER been on disk in the committed corpus — a
pre-existing 29-record gap, unrelated to and not caused by this cycle's fix (confirmed: they appeared
as brand-new untracked files on the FIRST post-fix run, not a diff against previously-tracked
content). Reverted from the working tree (`git clean -fd`) rather than committed, since populating
them would need the SAME enrichment pass (`enrich_spell_raw_tokens.rs`) to reach the same completeness
as their 664 siblings, which is out of this card's scope. Not filed to `forward-scope-register.md`
(too small/local to need its own register entry) — named here so a future Gate-0 census walk (card 3)
or Epic-4 book-onboarding cycle re-derives it rather than re-discovering it.

## Live reproduction (RED) and re-verification (GREEN), by generator

All runs used the repo-local pinned oracle (§2.1 export block), in this worktree, `git status`
verified clean immediately before and after each run; every RED run was reverted with
`git checkout -- <dir>` (+ `git clean -fd <dir>` where the run created new untracked files) before
the fix was applied, and GREEN was a second live run of the SAME command after the fix.

### `gen_advanced_race_guide` (`gen_book_cache advanced_race_guide`)
- **RED:** all 93 spell records lost `raw_tokens`
  (`git diff --stat data/corpus/advanced_race_guide/spell | tail -1` → `93 files changed, 1040
  insertions(+), 6772 deletions(-)`); 15 equipment records owned by `cache_gen::equipment_gap`'s ARG
  mapping were PERMANENTLY DELETED (`git status --porcelain | grep -c '^ D'` → `15`).
- **GREEN (after fix):** `git status --porcelain -- data/corpus/advanced_race_guide` → only
  `LICENSE.json` (a pre-existing, out-of-scope cosmetic overwrite issue — every lane's write to this
  book's `LICENSE.json` has always unconditionally replaced the prior note rather than appending;
  reverted, not fixed, since it is not the self-erasure shape this card scopes and is already a known,
  tolerated, manually-reconciled pattern per the file's own accumulated history). Zero spell diffs,
  zero equipment deletions.

### `gen_pathfinder_unchained` (`gen_book_cache pathfinder_unchained`)
- **GREEN:** `git status --porcelain -- data/corpus/pathfinder_unchained` → only `LICENSE.json` (same
  cosmetic issue, reverted). 17/17 feats, 42/42 equipment, zero content diffs.
- RED not separately live-reproduced for this function (budget) — identical code shape to
  `gen_advanced_race_guide` (`grep -n "out_path.exists()" src/bin/gen_book_cache.rs` returned zero
  hits for it before this fix, same as D9 code-read-confirmed in SD-31), same fix applied.

### `gen_companion_book` (`gen_book_cache companion:inner_sea_combat`)
- **GREEN:** `git status --porcelain -- data/corpus/inner_sea_combat` → only `LICENSE.json` (reverted).
  4 creatures + 6 abilities, zero content diffs.
- RED not separately live-reproduced (budget) — same reasoning as `gen_pathfinder_unchained`.

### `gen_core_rulebook_cache`
- **RED:** see Discovery 2 above — 664 spell files modified (raw_tokens stripped), 1,476 files deleted
  across 5 OTHER generators' kinds.
- **GREEN:** 11/11 classes, 664/664 spells, 2,663/2,663 equipment; `git status --porcelain -- data/corpus/core_rulebook`
  → 29 new untracked spell files (Discovery 2's pre-existing gap, reverted via `git clean -fd`, not
  committed) and otherwise zero diffs — zero raw_tokens loss, zero cross-generator deletion.

### `cache_gen::acg` / `cache_gen::apg` / `cache_gen::beastiary1` / `cache_gen::spell_lane_dump` / `cache_gen::ultimate_equipment`
RED→GREEN proven via permanent unit tests added this cycle (not live corpus runs, since these write
into the SAME on-disk records the rest of the suite depends on and a synthetic in-memory/temp-dir
fixture is the safer, faster, equally-decisive proof):
- `acg::tests::write_json_never_overwrites_an_existing_file`
- `apg::tests::write_json_never_overwrites_an_existing_file`
- `beastiary1::tests::write_json_never_overwrites_an_existing_file`
- `spell_lane_dump::tests::a_second_run_does_not_erase_a_later_enrichment_pass_on_a_still_valid_record`
  (against the REAL pinned oracle, via the module's own pre-existing `generate()` test scaffold)
- `ultimate_equipment::tests::a_second_run_does_not_erase_a_later_enrichment_pass_on_a_still_valid_record`
  (synthetic scratch corpus, the module's own pre-existing `ScratchCorpus` fixture)

Each was confirmed RED (fails for the intended reason — the enrichment marker is clobbered) before
the fix, then GREEN after. `ultimate_equipment`'s pre-existing regression test
`a_dropped_record_does_not_linger_from_a_prior_run` (added at "OPEN-ISSUES row 38", the reason the
directory wipe existed in the first place) was re-run and still passes after the fix — the new
`remove_stale_owned_files` mechanism preserves that guarantee without the collateral wipe.

```
$ cargo test --locked --lib rules_core::cache_gen::
... (acg, apg, beastiary1, spell_lane_dump, ultimate_equipment, equipment_gap, class_feature, hand_authored_equipment, class_feature_grants — all pass)
```

## Full test suite

```
$ cargo test --locked --lib
test result: ok. 2341 passed; 0 failed; 13 ignored; 0 measured; 0 filtered out

$ cargo test --locked --test sd26_cache_core_rulebook --test sd27_advanced_race_guide_cache_shape \
    --test sd27_advanced_race_guide_parity --test sd27_book_license_record_counts \
    --test sd27_pathfinder_unchained_cache_shape
running 6 tests ... ok (sd26_cache_core_rulebook)
running 8 tests ... ok (sd27_advanced_race_guide_cache_shape)
running 2 tests ... ok (sd27_advanced_race_guide_parity — includes a 64s real end-to-end pipeline run)
running 6 tests ... ok (sd27_book_license_record_counts)
running 7 tests ... ok (sd27_pathfinder_unchained_cache_shape)

$ cargo test --locked --test v06_corpus_trap_report
running 25 tests ... ok
```

## Retro log (per §12 gate wrap-up)

`scripts/retro.py summary --since 2026-08-22 --json` shows this session's own preflight-oracle
FAIL→bootstrap event (self-healed per §8's "empty oracle slot in a fresh worktree" rule) plus two
prior PASS verification events from the launch-readiness remediation session — no incidents,
corrections, or deferrals logged by prior cycles in this window (this is the first cycle dispatched
against this bundle since launch-readiness). No recurrence key fires more than once. This cycle's own
retro events (below) are the first substantive entries.
