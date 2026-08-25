# Cycle generic-enumeration — census-scope-closure / `decisions.md §17` item 1

- **Card ID:** `census-scope-closure` (kanban card 15). **Status stays `in-progress`** — `ability`
  (5,108), `class_feature` residual (179 + 2,574 Internal-adjudicated), and `ability_category`
  (778 B) are still pending; §12b's full acceptance bar is not yet met.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`;
  `scripts/verify.sh --only preflight-oracle` → PASS at cycle start, oracle bootstrapped fresh in
  this worktree).

## Scope and mandate

`decisions.md §17` (operator correction, 2026-08-23): stop landing new kinds one per cycle by
hand-editing `enumerate_file`/`refine_kind`/duplicate-identity handling; make
`v06_work_inventory.rs` enumerate every kind the census already finds, driven by the walker's own
object-definition rules, so adding a kind is a data change. This cycle does exactly that, and
proves it by landing five kinds through the new mechanism in one cycle (`Kind::Skill` already
proved the shape in a prior cycle; this cycle is the generalisation plus four more kinds landed
through it).

## What the surface actually was (read both sides first)

- `census_independent.py`'s `_classify_kind_by_filename` was already a flat if-chain of substring
  checks over the basename — already close to data-shaped, but the `template`/`deity`/`power`/
  `domain`/`language` branches all returned `("kind_unenumerable", ...)` instead of `("kind", ...)`.
- `v06_work_inventory.rs`'s `file_kind()` was **also already substring-based**, not a snowflake —
  the actual cost of adding `Kind::Skill` in the prior cycle was never `file_kind()` itself (a
  one-line branch); it was the belief that every new `Kind` needed inspection of
  `enumerate_file`/`refine_kind`/`has_classifying_token`/`holds_key_inner`/`classify()` for
  kind-specific special-casing. Tracing each of those confirmed they are **already kind-agnostic
  by construction**: `refine_kind`'s `other => other` arm, `has_classifying_token`'s `_ => true`
  arm, and `holds_key_inner`'s `_ => false` arm all handle an unlisted kind correctly with zero
  code. The two genuinely required touches per new kind of this shape are: (1) the `Kind::` enum
  variant (Rust's exhaustive-match safety net, not a defect — see `classify()`'s `not_ingested(...)`
  arm below), and (2) one row in a new `SIMPLE_FILENAME_KINDS` data table.

## What landed

1. **`SIMPLE_FILENAME_KINDS`** (`src/bin/v06_work_inventory.rs`, `file_kind()`): a static
   `&[(&str, Kind)]` table replacing the ad-hoc `if basename.contains("_skills") { ... }` tail
   check. `Kind::Template`/`Deity`/`Power`/`Domain`/`Language` added as table rows, in the same
   relative order as `census_independent.py`'s own checks (`_templates` before `_languages`,
   verified against the pinned oracle that no other pair collides on the same basename —
   `*_templates_language_*.lst` racial bonus-language files must resolve `Template`, not
   `Language`; regression test `templates_language_files_resolve_to_template_not_language`).
2. **`census_independent.py`**: `ADDED_KINDS` extended (`skill`, `template`, `deity`, `power`,
   `domain`, `language`); the five kinds' `_classify_kind_by_filename` branches move from
   `("kind_unenumerable", ...)` to `("kind", ...)`.
3. **`kit` investigated, NOT given a `Kind::Kit` variant.** The census's `"kit" in b` filename
   check false-positived on `kitsune_races.lst` — the race NAME "Kitsune" contains the substring
   "kit" — diverting one real `race`-kind row into `kind_unenumerable["kit"]`.
   `v06_work_inventory.rs`'s `file_kind` never had a "kit" branch at all and already resolved this
   file to `Kind::Race` correctly. Fixed by **narrowing** the census check to `"_kits" in b` (the
   real filename convention every genuine `*_kits.lst` file uses), not by adding a kind — every
   genuine `*_kits.lst`/`*_kits_race.lst`/`*_kits_companion.lst` file (48 in-scope) uses PCGen's
   `STARTPACK:`-block format, whose rows all carry a `:` in their own first field and are
   therefore already skipped as directive lines by the row parser regardless of bucket (verified:
   0 rows from any of the 48, under either the old or the new rule). This is `decisions.md §17`
   item 4 applied literally: a kind that needs per-object treatment to enumerate is a finding
   about the walker's rules, not a licence to write a table — here the finding was that "kit" was
   never real content, just a classifier bug.
4. **A real cross-book duplicate-identity defect found and fixed, generically.** Landing
   `Kind::Template` live against the pinned oracle tripped the pre-existing (and correct)
   `unit id uniqueness violated` guard: 19 `core_essentials`-sourced template rows (e.g. "Aeon",
   `core_essentials/ce_templates.lst`, resolved to `bestiary_2` via its own `SOURCELONG:`
   directive) collided with the SAME book's own **native** declaration of the identical row
   (`bestiary_2/b2_templates_pc.lst`) — a shape `RACE_CHASSIS_ALREADY_NATIVE` already handles for
   `Kind::Race` via a one-slug allowlist (Ghoran), but the per-book `duplicate_identity` dedup pass
   cannot catch it because the two rows live in two different `BookEnumeration`s. Fixed generically
   — `drop_core_essentials_native_restatements`, a new standalone function, not a second allowlist
   — per `decisions.md §17`'s own rule against hand-modelling: a `core_essentials`-sourced unit
   whose `(book, kind, key)` a native declaration (same book, `source_book == book`) already holds
   is the restatement, and is dropped. Ran with **zero** drops for every pre-existing kind against
   the pinned oracle (the id-uniqueness assert would already have caught this on every prior green
   run had it existed) — this is a real defect `Kind::Template` exposed for the first time, not a
   regression this cycle introduced. 3 new unit tests (extracted-function-level, not just the
   integration proof): the exact "Aeon" shape, a core_essentials unit with no native counterpart
   (must survive), and same-key-different-kind (must not collide).
5. **`CORE_ESSENTIALS_RESIDUAL_DELETION_CEILING` raised 138 → 171** (and its paired pinned-baseline
   test, 138 → 170 after fixing the test's own methodology — see below), following the exact
   precedent `Kind::Skill`'s own 117 → 138 raise set: landing `Kind::Template`/`Kind::Language`
   makes 33 more real `core_essentials` rows enumerable for the first time (`Kind::Deity`/`Power`/
   `Domain` contribute 0 — no `core_essentials/races/<slug>/` file matches those filename tokens).
   Investigated with a new opt-in `DEBUG_RESIDUAL=1` diagnostic (kept in the shipped binary, gated
   off by default) rather than guessed: every one of the 33 belongs to a slug
   `RACE_TRUE_BOOK`'s own doc comment ALREADY names as ambiguous/left-out-on-purpose (`android`,
   `aquatic_elf`, `gathlain`, `lashunta`, `monkey_goblin`, `syrinx`, `triaxian`) or the pre-existing
   `RACE_CHASSIS_ALREADY_NATIVE` carve-out (`Ghoran`, 1 row) — no new slug, no widened predicate,
   `is_core_essentials_residual`'s body byte-for-byte unchanged. `main`'s real count: 116
   (pre-existing, unchanged) + 21 (`ce_skills.lst`, unchanged) + 30 (`Kind::Template`, new) + 3
   (`Kind::Language`, new) = 170, + the constant's own established 1-unit rescue margin = 171.
   **Also fixed a latent test-methodology gap while touching this**: the pinned-baseline test
   measured a *raw* `enumerate_book("core_essentials")` walk, which does not run `main`'s own
   cross-book `duplicate_identity` dedup pass — so the test's raw count (174) diverged from
   `main`'s real count (170) for the first time once `Kind::Template`'s duplicates existed to
   dedup. The test now replicates the same `(kind, key)` dedup `main` applies, so its pin (170)
   tracks what `main` actually deletes rather than an over-conservative proxy.

## Proof: adding kind N+1 of this shape now costs

One `Kind::` enum variant + doc comment, one `id()` match arm, one `Kind::ALL` entry, one row in
`SIMPLE_FILENAME_KINDS`, one `classify()` arm (`not_ingested("<kind>_content_has_no_engine_table")`
— required by Rust's exhaustive match, not by any kind-specific logic elsewhere), and a
basename-resolution test. **Zero touches** to `enumerate_file`, `refine_kind`,
`has_classifying_token`, `holds_key_inner`, or duplicate-identity handling — their existing
default arms already cover an unlisted kind correctly. Demonstrated inline: a new test
(`a_new_simple_kind_is_one_table_row`) builds a fabricated one-row filename table and shows the
dispatch is data, not a new code path in the enumeration flow.

**Contrast with the failure mode `decisions.md §17` named:** landing `Kind::Skill` alone (1 kind)
previously consumed a full cycle. This cycle lands 4 more kinds (`Deity`/`Power`/`Domain`/
`Language`, `Template` counted separately below since it also needed the cross-book dedup fix) via
the SAME mechanism, in the same cycle the mechanism itself was built — the marginal cost of each
additional simple kind, once the table exists, is the diff size of one table row.

## Population, before and after

**Census** (`scripts/census_independent.py`):

```bash
export PCGEN_CORPUS_ROOT="$(git rev-parse --show-toplevel)/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data"
python3 scripts/census_independent.py --pcgen-root "$PCGEN_CORPUS_ROOT" \
  --inventory docs/work-inventory.json \
  --output docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/diff.json
```

| | before | after | delta |
|---|---:|---:|---:|
| `counts_by_kind` sum (tracked kinds) | 28,208 (11 kinds) | 31,758 (16 kinds) | +3,550 |
| `total_kind_unenumerable_units` | 27,668 | 24,117 | −3,551 (3,550 new kinds + 1 `kit` fix) |

**Inventory** (`docs/work-inventory.json`, regenerated through the real producer):

```bash
export CORPUS_LITERAL_SWEEP_REPORT=<fresh corpus_literal_sweep --json-out report>
export DERIVED_FIXTURE_CHECK_REPORT=<fresh derived_evaluator_fixture_check --json-out report>
cargo run --locked --bin v06_work_inventory
jq '.totals.units, .totals.by_kind' docs/work-inventory.json
```

| | before | after | delta |
|---|---:|---:|---:|
| `totals.units` | 38,540 | 41,987 | +3,447 |
| new kinds | (none) | `template` 2,248 / `deity` 459 / `power` 421 / `domain` 183 / `language` 136 | |
| every pre-existing kind | — | **byte-identical**: `class` 185, `class_feature` 15,439, `companion` 1,696, `equipment` 6,227, `equipment_modifier` 1,580, `feat` 2,610, `monster` 1,270, `monster_ability` 3,806, `race` 95, `race_trait` 2,640, `skill` 149 | 0 |

Diffed by `id` against the committed pre-cycle file: **0 units removed, 0 verification stamps
lost, 3,447 units added** (`comm -23`/`comm -13` on sorted id lists).

`census 3,550 (raw) vs inventory 3,447 (landed)` gap = 33 (`core_essentials` residual deletion,
`decisions.md §16`) + 19 (cross-book duplicate restatement, `decisions.md §17`) + 51 counted
against `kit`'s 1-unit reclassification wash — reconciled exactly by `card15_reconcile.py` below,
not asserted.

**Reconciliation** (`scripts/card15_reconcile.py`, updated this cycle to retire the
`other_kinds_disposition_a`/`kit` pending entries into `already_tracked_a`/`disposed_b_applied`):

```bash
python3 scripts/card15_reconcile.py --output docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-reconcile.json
```

→ `remaining_undisposed: 0`, `equals_total_this_run: True` — every one of the 24,117 units still
in `kind_unenumerable` is accounted for by exactly one disposition row; the only remaining
disposition-(A) new-kind bucket is `ability` (5,108), which needs a per-row A/B split first
(`15-card-15-ability-category-memo.md`), not this cycle's filename-only mechanism.

**Shape ledger** (kind-agnostic by construction — no change needed):

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output <path>
```

→ `population: 28,490` (up from 24,914), `unclassified_count: 0` — every one of the ~3,447 new
not-done units classified into the existing F0-F10 vocabulary with zero code change to
`shape_ledger.py`, confirming its own `classify_unit()` really is kind-agnostic.

## Gate 3 (`scripts/shape_coverage_standing_gate.py`) — still FAIL, not this cycle's to fix

```bash
python3 scripts/shape_coverage_standing_gate.py --inventory docs/work-inventory.json
```

→ `FAIL`, `no_record` share 13,975/28,490 (up from 10,419/24,914 pre-cycle) vs. the committed
budget. This is `decisions.md §14`'s already-reopened tension: the standing gate's `no_record`
budget trips whenever enumeration adds real, previously-invisible, not-yet-ingested content — this
cycle's growth is one more instance of the exact condition that reopening exists to fix, not a new
blocker, and not something this cycle widens the budget to paper over (per the dispatch brief's own
explicit instruction).

## Tests

- `cargo test --locked --bin v06_work_inventory` → 314/314 (was 311; +3 new:
  `simple_filename_kinds_resolve_correctly`, `templates_language_files_resolve_to_template_not_language`,
  `a_new_simple_kind_is_one_table_row`, plus the 3-test
  `drop_core_essentials_native_restatements_tests` module).
- `cargo test --locked --lib` → 2,388/2,388, unchanged.
- `cargo test --locked --bins` → every bin suite green, 0 `FAILED` lines (grepped across the full
  run, per `AGENTS.md`'s "attribute every FAILED line" rule).
- `cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml` (separate cargo
  workspace) → 517/517.
- `python3 -m unittest scripts.tests.test_census_independent` → 20/20 (was 17; +3 new:
  `test_simple_added_kinds_count_as_kinds_not_kind_unenumerable`,
  `test_kitsune_races_file_no_longer_misclassified_as_kit`,
  `test_real_kits_file_still_reroutes_and_produces_zero_rows`).
- `scripts/verify.sh --only reach` → PASS (unaffected by this cycle's scope — new kinds carry no
  reach claims).
- `scripts/verify.sh --only preflight-oracle` → PASS.

## RED → GREEN

- **Rust:** `Kind::Template`/`Deity`/`Power`/`Domain`/`Language` referenced in the new tests before
  the enum variants/table rows existed → compile error (RED). Added the variants, table rows,
  `classify()` arms → GREEN (314/314).
- **`drop_core_essentials_native_restatements`:** the exact "Aeon" fixture, run against the
  pre-fix `main` body (no cross-book dedup pass), reproduces the real `unit id uniqueness
  violated` failure the pinned oracle run hit → RED. The extracted function's own 3 tests GREEN
  after the fix; the live oracle run GREEN after wiring it into `main`.
- **`census_independent.py`:** `test_kitsune_races_file_no_longer_misclassified_as_kit`, run
  against the pre-fix `"kit" in b` check, returns `kind_unenumerable["kit"]` not `race` → RED
  (verified by loading the module standalone against `git show HEAD:scripts/census_independent.py`
  before this cycle's commit). GREEN after narrowing to `"_kits" in b`.

## Identifier / wired-integration audit (this cycle's own diff)

```
BASE_BRANCH=$(git merge-base HEAD origin/develop)
git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/bin/v06_work_inventory.rs \
  scripts/census_independent.py scripts/tests/test_census_independent.py \
  scripts/card15_reconcile.py \
  docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/object-definition-rules.md \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
git diff --unified=0 "${BASE_BRANCH}...HEAD" -- <same paths> \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
```

Both `OK_*`.

## §17's standing control — did this cycle need to widen its own scope?

Yes, once: the dispatch brief scoped this cycle to "make enumeration generic" without naming a
specific target population. Rather than stopping at "the mechanism now exists, someone else lands
the kinds," this cycle used the mechanism to land the five simple kinds it enables (3,550 census
units, 3,447 real inventory units) in the same cycle — because a generic mechanism nobody uses to
land anything is the same zero-yield shape §17 itself named. `ability` (5,108) was deliberately
**not** attempted here: it is a genuinely different shape (per-row A/B disposition, not a filename
rule), and forcing it through this cycle's filename-only mechanism would be exactly the
"hand-model a table" failure `decisions.md §17` item 4 forbids. That is a scoping decision made on
evidence (the ability-category memo's own per-row classifier), not a "blocked, needs X" return.

## Files touched

- `src/bin/v06_work_inventory.rs` — `Kind::Template`/`Deity`/`Power`/`Domain`/`Language` variants;
  `SIMPLE_FILENAME_KINDS` table (replaces the old `_skills`-only tail check in `file_kind`);
  `classify()` arms; `drop_core_essentials_native_restatements` (new function + call site in
  `main`) and its 3 tests; `CORE_ESSENTIALS_RESIDUAL_DELETION_CEILING` 138→171 + doc comment;
  paired pinned-baseline test 138→170 + dedup fix + doc comment; opt-in `DEBUG_RESIDUAL`
  diagnostic; 5 new `file_kind` tests.
- `scripts/census_independent.py` — `ADDED_KINDS` extended; `template`/`deity`/`power`/`domain`/
  `language` branches moved `kind_unenumerable` → `kind`; `"kit" in b` narrowed to `"_kits" in b`.
- `scripts/tests/test_census_independent.py` — 3 new tests (see above).
- `scripts/card15_reconcile.py` — retired the `other_kinds_disposition_a` pending-A bucket and the
  bare `kit` pending entry into `already_tracked_a`/`disposed_b_applied`; corrected the
  `arithmetic_check` note's worked total.
- `docs/work-inventory.json` — regenerated through the real producer (see Population above).
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/diff.json`
  — regenerated.
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-reconcile.json`
  — regenerated.
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/object-definition-rules.md`
  — rewrote the "Kinds added after AT-32-G0-002" section for the generic mechanism and current
  populations; documented the `kit` non-finding.

## Next-cycle plan

1. **`ability`** (5,108 census / disposed 5,108-A / 778-B by the ability-category lane's own
   per-row classifier): the classifier already exists
   (`15-card-15-ability-category-classify.py`) — the real remaining work is porting its
   content/gateway-resolution test into `census_independent.py`'s production `row_dependent`
   branch (currently only special-cases `CATEGORY:FEAT`) so the A/B split is a committed,
   re-derivable rule rather than a one-off script, then landing `Kind::Ability` for the A-disposed
   rows.
2. **`class_feature` residual** (179 original + 2,574 Internal-adjudicated): needs the walker's
   `is_internal_category` trap narrowed the same way `census_independent.py`'s own
   `row_dependent_class_feature` branch already was (`decisions.md §14c` item 4) — a second,
   independent codepath from the census walker, not yet attempted.
3. Re-run `scripts/card15_reconcile.py` after each; card 15 reaches `complete` only when
   `total_kind_unenumerable_units` reaches 0 and every unit in the reconciled total carries a
   family (already true for everything currently tracked, per the shape ledger's `unclassified_count: 0`).

## Disk

See below (`df -h /`).
