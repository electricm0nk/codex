# Cycle AT-33-E6-001 (build-green lane) — epic-6-closure / AT-33-E6-001 Shortfall 1

- **Commit SHA:** this cycle's own landing commit (see `progress.md` entry `sd33-r8-build-green`).
- **Worktree:** `/tmp/sd33-r8-build-green-wt`, a clean `git worktree add` off `origin/tranche/13` @
  `3fc992a727` (attempt 8's own scanned HEAD) — the shared checkout at
  `/home/ubuntu/workspace/repos/codex` was 14 commits behind with 157 foreign `git status` entries
  (141 uncommitted `data/corpus/**` modifies from a concurrent lane, 7 deletions, 9 untracked) this
  agent did not create; per `AGENTS.md`'s "One writer per tree" nothing was written there.
- **Files touched:** `tests/sd20_equipment_equipmods.rs`, `tests/sd20_tabletop_readiness_integration.rs`
  (sibling found by the full build, not named in the dispatch), `tests/sd25_monk_level_up_explanation_filter_audit.rs`,
  `tests/v06_work_inventory.rs`; this receipt; `progress.md`; `kanban.md` (rows 17/18 Notes pointer
  only); `docs/retro/events/sd33-r8-build-green.jsonl` (auto-appended by `verify.sh`).

## Timeline re-verified live (not trusted from the scan)

```
$ for c in f652db7ac7 66984fe7bc 2f1d52f22d 7d439876b7; do
    git show $c:src/rules_core/equipment_effects/equipmods.rs \
      | sed -n '/pub struct WeaponEnhancementBonus/,/^}/p' | grep -E '^\s*pub [a-z_]+:'; done
f652db7ac7 / 66984fe7bc: pub affects: String, pub bonus: i16, ...
2f1d52f22d / 7d439876b7: pub tohit_bonus: Option<i16>, pub damage_bonus: Option<i16>, ...

$ git log --oneline f652db7ac7..HEAD -- tests/sd20_equipment_equipmods.rs
(empty)
```
Confirmed: the struct carried `affects`/`bonus` through `66984fe7bc` and was split by `2f1d52f22d`
(`AT-33-E5-finalize-wave5`); no SD-33 commit ever updated `tests/sd20_equipment_equipmods.rs`.
**Attributed to `2f1d52f22d`, neither pre-existing nor wave 6's**, matching attempt 8's scan.

## Shortfall 1 fix — `tests/sd20_equipment_equipmods.rs:94-111`

Both assertion pairs read the corpus token verbatim, so the intended magnitude was cross-checked
against the fixture's own `BONUS:` token before rewriting:

| Case | Token | Old assertion | New assertion |
|---|---|---|---|
| `+1 (Enhancement to Weapon)` | `BONUS:WEAPON\|DAMAGE,TOHIT\|1\|TYPE=Enhancement` | `affects == "DAMAGE,TOHIT"`, `bonus == 1` | `tohit_bonus == Some(1)`, `damage_bonus == Some(1)` |
| `Adamantine` | `BONUS:WEAPON\|TOHIT\|1\|TYPE=Enhancement` | `affects == "TOHIT"`, `bonus == 1` | `tohit_bonus == Some(1)`, `damage_bonus == None` |

Matches the scan's inference exactly — **no divergence**. No assertion weakened; both new pairs are
strictly more specific than the old single-field pair (two typed fields vs. one string).

## Sibling search (the instruction the dispatch named)

```
$ grep -rn --include='*.rs' -E '\b(affects|bonus)\b' tests/ | grep -i weaponenhancement
(nothing — misses any local var not literally named "weaponenhancement")
$ grep -rn --include='*.rs' 'WeaponEnhancementBonus' tests/ src/ apps/
(21 hits, all in src/rules_core/equipment_effects/equipmods.rs (definition + its own
already-updated unit tests), src/rules_core/equipment_effects.rs, src/rules_core/damage_total.rs
(doc-comment prose only, not a field access — `WeaponEnhancementBonus::affects` inside a code-span,
never compiled), src/bin/e5_last67_weapon_ours.rs (already uses tohit_bonus/damage_bonus))
```
The name-grep found nothing because `tests/sd20_tabletop_readiness_integration.rs:1528-1529` reads
the field through a local variable named `enhancement`, not `weaponenhancement`. **Only
`cargo test --locked --no-run` on the FULL workspace surfaced it** — a real instance of the
dispatch's own warning that a shallow/name-based search lies in this repo. Root-caused before
fixing: `masterwork` weapon quality's real token is `BONUS:WEAPON|TOHIT|1|TYPE=Enhancement`
(TOHIT-only) — fixed to `tohit_bonus == Some(1)`, `damage_bonus == None`, RED confirmed first
(`error[E0609]`), then GREEN (target compiles, both tests in the file pass).
No other reference to the old `affects`/`bonus` fields exists anywhere in `tests/`, `src/`, `apps/`
(no `benches/` directory exists in this repo).

## Two further genuine SD-33 failures, found only because the build now runs to completion

`cargo test --locked --no-fail-fast` at HEAD (3fc992a727 + this cycle's 4 file edits) was compared,
target-by-target, against the SAME command run in a clean worktree at the `tranche/13` cut
(`f652db7ac7`, restricted to the 33 targets HEAD failed, for turn-budget reasons — root-lib and
desktop were already independently confirmed green at both points). **31 of 33 failed identically
at both points (byte-identical `N passed; M failed` pairs, same order)** — genuinely pre-existing,
not fixed, see "Pre-existing failures" below. **2 did not: they passed at the cut and failed at
HEAD** — SD-33's own debt, fixed here, RED confirmed before the fix, GREEN after:

### `sd25_monk_level_up_explanation_filter_audit.rs` — `the_flat_ac_bonus_id_...`

`git diff f652db7ac7..HEAD -- src/rules_core/pilot_compute/mod.rs` shows `AT-33-E5-remainder-charbuild`
grounded Monk AC Bonus's real level-4+ dodge progression (`monk_ac_bonus_dodge_progression`,
oracle-confirmed), which this audit's own exclusion #1a ("AC Bonus is flat, never surfaces") had
documented as a KNOWN GAP before that fix landed. The exclusion's premise is now false — the id
legitimately changes at levels 3->4, 7->8, 11->12, 15->16, 19->20 and the level-up filter correctly
surfaces every one of those five transitions (verified, not assumed: per-transition proof added).
Rewrote the test to assert the NEW, correct reality (`the_ac_bonus_id_now_legitimately_surfaces_the_level_4_plus_dodge_progression`)
rather than weakening or deleting it — this is a real strengthening: it now positively proves the
filter does not silently drop the newly-grounded delta, the exact SD-24 Wizard bug shape this whole
audit file exists to catch. Header doc comment and the `AC_BONUS_ID` const doc comment updated to
match (no longer "structurally-never-diffing"). 6/6 tests in the file pass.

### `v06_work_inventory.rs` — `zero_magnitude_option_pool_class_features_are_not_ingested_not_unknown`

Root-caused, not assumed: `src/rules_core/class_feature_pool_catalog.rs` is **byte-identical** to
the `tranche/13` cut (`git log`/`git diff` both empty) — the code deciding this did not change.
The fixture's two `Discovery ~ *` ids flipped `not-ingested` -> `text-complete` because the
COMMITTED `docs/work-inventory.json` this test reads was simply stale relative to that
already-shipped SD-32 T12 code (which widened `is_registered_pool_group` to accept every
`" ~ "`-qualified group, gated on render-and-refuse safety checks alone, per that module's own doc
comment) — SD-33's `AT-33-E4-002` regeneration cycle disclosed this in its own commit message as
"3,985 units of unrelated drift from SD-32 engine work landed since the file's last regen", and
this pair is part of that disclosed bucket, not new classifier logic. Confirmed both ids now
genuinely render clean, real prose (hand-read from
`data/corpus/advanced_players_guide/class_feature/discovery/*.json`) — `text-complete` is the
CORRECT verdict, not a bug to revert. Per the test's own documented self-healing design ("If a
future wave registers 'Discovery' too, THIS fixture will need re-picking the same way"), re-picked
to `advanced_class_guide:class_feature:{aberrant,abyssal}_bloodrager_bloodline_bonus_spells` — an
internal chassis record whose corpus `description` is `null` (no DESC token at all, not merely
missing prose), which structurally can never pass `class_feature_pool_catalog`'s
`has_real_description` gate, so this pick cannot go stale the same way twice. Owner-resolution
mechanism (the `CLASS_FEATURE_POOLS` third fallback, "Bloodrager Bloodline" -> `bloodrager`)
confirmed live, matching the evidence string the test already asserts. 16/16 tests in the file pass
(1 ignored, same as the cut).

## Pre-existing failures — NOT fixed, evidence per item

All 31 reproduce byte-for-byte at `f652db7ac7` (same `N passed; M failed`, same target order,
verified live in a clean worktree, not inferred):

- **`ingest_races` (bin unit test)**: `assertion failed: record.data.key.starts_with(&record.data.race_key)`
  at `src/bin/ingest_races.rs:2160`. Re-ran the SAME test at a `f652db7ac7` worktree: **identical
  panic, identical location.** `git log f652db7ac7..HEAD -- src/bin/ingest_races.rs` is empty; 0
  `data/corpus/*/race/*` or `*/race_trait/*` files changed since the cut. A corpus-wide scan found
  87 real records where `key` does not start with `race_key` (e.g. `"Adopted Race ~ Suli"` vs.
  `"Suli"`, `"Deep Jungle Halfling ~ Poison Use"` vs. `"Halfling"`) — a pre-existing naming-shape
  assumption in the test, unrelated to any SD-33 change.
- **29 more** (`sd13_sorcerer_*` x3, `sd18_cleric_level{11..20}_widening` x10,
  `sd24_{identifier_discipline,wired_integration}_audit`, `sd26_{cache_acg,cache_apg,identifier_discipline_audit}`,
  `sd27_{ability_automatic_granted_race_traits,advanced_race_guide_cache_shape,alternate_racial_trait_reachability,book_license_record_counts,equipment_modifier_price_matches_corpus_cost_token,known_spells_must_be_on_the_class_spell_list}`,
  `sd30_declared_product_identity_in_shipped_class_features`, `sd31_class_feature_corpus_key_uniqueness`,
  `duergar_invisibility_sla_reaches_a_player_via_monster_codex`, `formula_interpreter_family_fixture_check`,
  `no_foreign_home_paths`, `v06_corpus_trap_report`): none of these `tests/*.rs` files carry any
  commit since the cut (`git log f652db7ac7..HEAD -- <each path>` empty for all 30). Spot-checked
  `sd27_equipment_modifier_price_matches_corpus_cost_token` in depth (closest in shape to this
  bundle's own equipment work): imports only `equipment_resolver::equipment_catalog_row_by_key`,
  whose only SD-33 diff (`d0035040ae`) is a pinned-count doc/assertion edit with **zero** functional
  change (`git diff` shows only a comment + `8_100`->`8_119` literal); the failing book
  (`pathfinder_unchained`) has 0 changed corpus files since the cut. Full per-target
  before/after pass-fail pairs in `/tmp/head_31.txt` / `/tmp/cut_31.txt` (session-local; the
  identical-counts diff is quoted in this cycle's chat transcript).
- **Root-lib, desktop crate**: unaffected by any of the above — already green both before and after.

## FINISH LINE

1. `cargo test --locked --no-run` → **exit 0**, all **543** `tests/*.rs` targets built
   (`ls tests/*.rs | wc -l` = 543; `grep -c '^  Executable tests/' <build log>` = 543).
2. `cargo test --locked --no-fail-fast` (the real full-workspace run — first time in this bundle
   all 543 integration targets execute):
   **passed=7974, failed=49 (31 targets, all pre-existing per above), ignored=67, 599 suites run**
   (2,836 lib + 543 integration + bin unit tests + 20 doc-tests etc.). Plain `cargo test --locked`
   (fail-fast, the literal instructed command) → **exit 101** at the pre-existing `ingest_races`
   failure — identical exit and failure point to a plain run at the cut.
3. `cd apps/desktop/src-tauri && cargo test --locked` → **548 passed, 0 failed, exit 0.** Confirmed
   unchanged: `git diff --stat f652db7ac7..HEAD -- apps/desktop/` is **empty** — this whole crate is
   byte-identical to the cut.
4. `scripts/verify.sh` full — **32 passed, 5 FAILED**: `site-dashboard-check`, `root-full`
   (subsumed by item 2 above — same 31 pre-existing target failures), `corpus-sweep`, `frontend-test`,
   `clippy`. `--only denominator-gate` → **PASS, files_checked=56 violations=0, exit 0.**
   Per-stage attribution, none fixed this cycle (see "New finding" and "Pre-existing, verify.sh-only"
   below — both outside this dispatch's named defect and write scope):
   - **`clippy` root** (`invisible character detected`, `src/rules_core/rules_tables/monster_codex/spell_list.rs:83`,
     a `\u{AD}` soft hyphen baked into generated spell-description data) and **`clippy` desktop**
     (20 warnings vs. a recorded ceiling of 7) and **`frontend-test`** (3 of 100 files — a
     `Cargo.toml` (0.11.0) vs. `package.json`/`tauri.conf.json` (0.13.0) version-triple drift) are
     all **pre-existing**: `apps/desktop/` (whole tree) and `src/rules_core/rules_tables/monster_codex/spell_list.rs`
     are both byte-identical to the cut, so clippy/jest's verdict on them cannot have changed.
   - **`corpus-sweep`** (105 findings across 10 records, e.g. `data/corpus/ultimate_equipment/equipment/blade_of_the_sword_saint.json`:
     every one of its 13 `raw_tokens` reported "not byte-present in the corpus token closure" the
     independent PCGen-oracle re-parse computes) is a **NEW finding, genuinely SD-33-caused, NOT
     fixed this cycle** — out of this dispatch's named defect and write scope. `git diff
     f652db7ac7..HEAD` on this record shows `raw_tokens`/`raw_bonus_chains` went from `[]` (empty,
     vacuously passing the sweep) to fully populated by SD-33's own wave-6 corpus regeneration
     (`enrich_equipment_raw_tokens.rs`, +243 lines this bundle) — and the populated tokens do not
     byte-match what `corpus_literal_sweep`'s independent `.MOD`-chain closure-builder derives from
     the pinned oracle `.lst`. All 5 records visible before the log's own truncation are confirmed
     inside SD-33's 137-file corpus diff. **Root cause not investigated further** (a different
     subsystem — `.MOD` identity/fold logic — outside this cycle's scope and turn budget); flagged
     here and in `progress.md`'s Open blockers for a dedicated follow-up cycle, per `AGENTS.md`
     Blocker Discipline ("raise your hand" — not cleared, not silently dropped).
   - **`site-dashboard-check`**: `v06_work_inventory --summary` timed out after 600s. Not
     root-caused (plausibly environmental — this machine had been running back-to-back ~40-minute
     full workspace builds for several hours by this point in the cycle); reported, not attributed
     either way.
5. `box_ledger.py --check` → **`oracle_disagreement=0`, exit 0**, re-confirmed after all of the
   above. Epic 5 undisturbed.

## Movement, four buckets

Closure 0 / reclassification 0 / reachability 0 / instrument-correction 0 — this cycle repairs two
test files' stale expectations and one struct-rename gap; no `docs/work-inventory.json` `status`
field, unit, or instrument changed.

## Identifier / wired-integration audits (final diff, this cycle's 4 files only)

```
OK_NO_BUNDLE_TAGS
OK_NO_TOKENS
```

- **Status:** complete
- **Notes:** The corpus-sweep finding is real, evidenced, and NOT a disposition — it is an
  escalation per `AGENTS.md` Blocker Discipline, filed under `progress.md`'s `## Open blockers`
  this same cycle. `AT-33-E6-001` (row 19) itself is the final-acceptance scan's own card and is
  intentionally left untouched here — re-scanning and re-dispositioning it is that lane's act, not
  this build-green lane's.
- **Next-cycle plan:** (1) a dedicated Epic-5/6 cycle to root-cause `enrich_equipment_raw_tokens.rs`'s
  `.MOD`-chain fold against `corpus_literal_sweep`'s independent closure for the 10 affected weapon
  records; (2) re-run `AT-33-E6-001`'s final-acceptance scan now that the workspace build is green.
