# Cycle card-15-ability — census-scope-closure / `decisions.md §12b`, `Kind::Ability`

- **Card ID:** `census-scope-closure` (kanban card 15). **Status stays `in-progress`** per dispatch
  instruction — the `class_feature` residual (179 + 2,574) is the only remaining disposition-(A)
  population not yet integrated; §12b's full acceptance bar is not yet met.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`;
  `scripts/verify.sh --only preflight-oracle` → PASS, oracle bootstrapped fresh into the
  repo-local `artifacts/corpus/operator-supplied/pcgen` slot per the dispatch brief, never
  `~/workspace/repos/pcgen`).

## §17a re-derivation — the brief's own figures did not hold up

The dispatch brief cited "5,886 units... 2,371 (A) / 243 (B) for the contested `CATEGORY:Internal`
rows." Re-derived before building anything, per `decisions.md §17a`:

1. **The population is 5,926, not 5,886.** `ability_category:Internal` grew 839→879 between the
   memo's pin and this cycle's start — an already-landed, unrelated reroute
   (`decisions.md §14c` item 4, `category-internal-adjudication` cycle), not a regression.
2. **The `2,371 (A) / 243 (B)` figure the brief cited is NOT part of this card's population at
   all.** It belongs to a *different, already-landed* population: the `row_dependent_class_feature`
   branch's `_abilities_class.lst` `CATEGORY:Internal` reroute (2,614 units, `decisions.md §14c`),
   already wired into `scripts/census_independent.py` before this cycle started
   (`_row_is_bare_internal_marker`, committed). This card's actual population is the
   `row_dependent` branch — bare (non-`_race`/`_class`/`_companion`/`_familiar`) `*abilities*.lst`
   files — whose own adjudicated split is `15-card-15-ability-category-memo.md`'s **5,108 (A) /
   778 (B) of 5,886**, a disjoint population from the one the brief quoted. The brief conflated two
   different adjudications; corrected before porting anything.
3. **Neither the memo's 5,108/778 nor the corrected 5,926 total survived re-derivation either** —
   see "A defect found and fixed" below. The real, live figure at this cycle's pin is **5,028 (A) /
   801 (B) of 5,829** (5,926 minus 97 rows this cycle found were never part of this population at
   all).

## A census/inventory disagreement found and fixed before landing anything

6 in-scope `*_abilities_familiar*.lst` files (`b2_abilities_familiar.lst`,
`pfs_b2_abilities_familiar.lst`, `b3_abilities_familiar.lst`, `ce_abilities_familiar_cr.lst`,
`ce_abilities_familiar_um.lst`, `ce_abilities_familiar_apg.lst`, 97 rows) were falling into
`census_independent.py`'s `row_dependent` branch even though `src/bin/v06_work_inventory.rs`'s
`file_kind` **already** routes them to the tracked `companion` kind (`_abilities_companion`/
`_abilities_familiar` checked before any bare-abilities fallback). This is exactly the
`decisions.md §12b` "the two walkers must agree" defect class — porting the ability classifier
onto the memo's stale population would have double-counted 97 rows against `companion`. Fixed by
routing `_companion`/`_familiar` abilities files to `kind:companion` in
`census_independent.py`'s `abilit` branch, matching Rust's own order (`scripts/census_independent.py`
diff). Proven by a dedicated test
(`test_abilities_familiar_file_routes_to_companion_not_ability_category`).

## What landed

1. **`scripts/census_independent.py`** — the memo's per-row A/B classifier
   (`15-card-15-ability-category-classify.py`) ported into the production `row_dependent` branch,
   unchanged in substance: `_ABILITY_CONTENT_RE` (the memo's own narrower field list, deliberately
   NOT the wider `_ROW_CONTENT_FIELD_RE` used for the disjoint class_feature-Internal population —
   the memo's per-bucket rulings were reviewed against this exact list), `_collect_tracked_keys` +
   KEY:-field-only B-duplicate join (ported from the classify script's own Pass 1, unchanged), the
   companion/familiar routing fix above. `ability` added to `ADDED_KINDS`.
2. **`src/bin/v06_work_inventory.rs`** — `Kind::Ability` landed as a genuinely new kind, not a
   `SIMPLE_FILENAME_KINDS` row (this population needs per-row disposition, not a filename rule,
   exactly as `decisions.md §17`'s own `generic-enumeration` receipt predicted):
   - `file_kind`: bare `abilit` fallback, positioned after the existing `_abilities_class`/
     `_abilities_race`/`_abilities_companion`/`_abilities_familiar` carve-outs (same order as
     Python's `abilit` branch).
   - `refine_kind`: `Kind::Ability` + `CATEGORY:FEAT` → `Kind::Feat` (one real corpus row at the
     memo's own population, `apg_abilities.lst`'s "Magical Lineage ~ Metamagic" — matches Python's
     row_dependent FEAT special-case).
   - `has_classifying_token`: `Kind::Ability` → `ability_row_has_content` (ported `_ABILITY_CONTENT_RE`
     as `ABILITY_CONTENT_PREFIXES` + `is_bonus_token`, a hand-matcher for `BONUS[A-Z]*:` — no
     `regex` crate dependency exists in this binary). Gateway/picklist rows both correctly fall
     through to `false` (excluded), same as the census side — B-gateway is a facet, not content.
   - **`is_internal_category`'s file-wide trap narrowed for `Kind::Ability` only**
     (`kind != Kind::Ability` guard added to its existing condition): without this, EVERY
     `CATEGORY:Internal` ability row — including the ~inline content-bearing ones — would have been
     silently dropped before `has_classifying_token` ever ran, swallowing real (A) content. Every
     other kind's behaviour (including `class_feature`'s own Internal rows) is byte-for-byte
     unchanged — proven by `class_feature_internal_row_is_still_dropped_by_the_blanket_trap`.
   - `classify()`'s exhaustive-match arm: `not_ingested("ability_content_has_no_engine_table")`.
   - **B-duplicate NOT implemented in Rust** (deliberate scope decision, reported not hidden): the
     memo's KEY:-field-only join needs cross-object visibility `CorpusUnit` does not currently carry
     (whether a row's `key` came from an explicit `KEY:` token vs. a bare-identity fallback).
     Approximating it with `unit.key` alone would reintroduce the exact "shared name is not proof"
     hazard the memo's own classifier exists to avoid, and risk **over-excluding** real (A) rows —
     the opposite failure this cycle's own mandate (#4, "do not swallow real objects") forbids.
     Population affected: single digits (8 in the memo's original count). Under-excluding here (not
     deduping a handful of true content-reuse rows) is the `decisions.md §1a`-consistent default,
     matching this file's own precedent elsewhere ("under-exclude, not over-exclude").

## A pre-existing Rust/census disagreement the `abilit` fallback incidentally fixed

3 in-scope files — `isg_abilities_feat.lst` (inner_sea_gods), `isc_abilities_feat.lst`
(inner_sea_combat), `isf_abilities_feat.lst` (inner_sea_faiths) — contain **both** "feat" and
"abilit" as substrings. `census_independent.py`'s `"feat" in b` check (checked before `"abilit" in
b`) has always routed these whole files to `kind:feat` unconditionally, no per-row filtering.
`src/bin/v06_work_inventory.rs`'s old `file_kind` had NO `"_abilities_feat"` branch at all — these
114 rows were entirely unenumerated by Rust (silently invisible in `files_not_enumerated`) despite
census already counting them. Landing the `abilit` fallback + `CATEGORY:FEAT` redirect makes Rust
enumerate these files for the first time, correctly splitting 111 real feat rows / 3 real ability
rows via the SAME per-row test used everywhere else. `docs/work-inventory.json`'s `feat` kind grew
2,610 → 2,722 (+112 = 111 + the 1 `apg_abilities.lst` row the memo already named) as a direct,
real, beneficial side effect — verified by id-diff: 0 pre-existing `feat` units removed, 0 stamps
lost.

**Found, not silently fixed further:** census's own `"feat" in b` check still counts all 114 rows
in these 3 files as `feat` unconditionally (no per-row `CATEGORY:` test at that branch) — so census
now disagrees with the inventory by 3 units for these files (the ability-disposed ones). Small,
real, and reported here rather than expanded into this cycle's scope (fixing it risks touching
every genuine `_feats.lst` file's ordering).

## Population, before and after

**Census** (`scripts/census_independent.py`):

```bash
export PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data
python3 scripts/census_independent.py --pcgen-root "$PCGEN_CORPUS_ROOT" \
  --inventory docs/work-inventory.json \
  --output docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/diff.json
```

| | before | after | delta |
|---|---:|---:|---:|
| `total_kind_unenumerable_units` | 24,117 | 18,992 | −5,125 |
| `counts_by_kind['ability']` | 0 | 5,028 | +5,028 |
| `counts_by_kind['companion']` | 1,676 | 1,773 | +97 (routing fix, not new content) |
| `ability_category:*` sum (remaining, disposition B) | 5,926 | 801 | −5,125 |

**Inventory** (`docs/work-inventory.json`, regenerated through the real producer, `--allow-stamp-loss`
NOT needed — no stamp lost):

```bash
export PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data
export CORPUS_LITERAL_SWEEP_REPORT=<fresh corpus_literal_sweep --json-out report>
export DERIVED_FIXTURE_CHECK_REPORT=<fresh derived_evaluator_fixture_check --json-out report>
cargo run --locked --bin v06_work_inventory
jq '.totals.units, .totals.by_kind' docs/work-inventory.json
```

| | before | after | delta |
|---|---:|---:|---:|
| `totals.units` | 41,987 | 46,923 | +4,936 |
| `ability` (new kind) | — | 4,824 | +4,824 |
| `feat` | 2,610 | 2,722 | +112 (see "pre-existing disagreement" above) |
| every other kind | — | byte-identical | 0 |

Diffed by `id` against the committed pre-cycle file: **0 units removed, 0 verification stamps
lost, 4,936 units added.**

`census 5,028 (raw, live) vs inventory 4,824 (landed)` gap = 289 (real, new `core_essentials`
residual deletion, `decisions.md §16` — see `CORE_ESSENTIALS_RESIDUAL_DELETION_CEILING`'s doc
comment, raised 171→460, and the pinned baseline test's own raised pin 171→448) minus 85
(the pre-existing feat/census disagreement's beneficial side effect, additional real `feat` units
Rust now enumerates that census already counted). Reconciled exactly by `card15_reconcile.py`
below, not asserted.

**Reconciliation** (`scripts/card15_reconcile.py`, updated this cycle: `ability_category_disposition_a`
retired from `pending_a` into `already_tracked_a.ability_landed_this_cycle`;
`ability_category_gateway_picklist_duplicate` retired from `pending_b_unapplied` — now empty — into
`disposed_b_applied.ability_category_b_disposed`, live-derived; the companion-routing fix logged as
its own `disposed_b_applied` entry):

```bash
python3 scripts/card15_reconcile.py --output docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-reconcile.json
```

→ `equals_total_this_run: True`, `remaining_undisposed: 0` — every one of the 18,992 units still in
`kind_unenumerable` is accounted for by exactly one disposition row. **The piles sum.** Card 15's
remaining scope, unchanged in shape by this cycle: the `class_feature` residual (179 + 2,574),
already flagged by the prior `generic-enumeration` cycle as needing a second, independent
`is_internal_category`-style narrowing in `v06_work_inventory.rs` for the `class_feature` kind
specifically (out of this cycle's scope — that population's own `refine_kind`/trap logic is
untouched here).

**Shape ledger** (kind-agnostic by construction — no change needed):

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output <path>
```

→ `population: 33,426` (up from 28,490), `unclassified_count: 0` — every one of the ~4,936 new
not-done units classified into the existing F0-F10 vocabulary with zero code change to
`shape_ledger.py` (F0 28,624 / F1 1,791 / F2 1,490 / F4 570 / F5 361 / F3 303 / F6 211 / F8 41 /
F9 27 / F7 5 / F10 3).

## Gate 3 (`scripts/shape_coverage_standing_gate.py`) — still FAIL, budget NOT touched

```bash
python3 scripts/shape_coverage_standing_gate.py --inventory docs/work-inventory.json
```

→ `FAIL`. `population` 28,490 → **33,426**. `no_record` share **18,904/33,426** (up from
13,975/28,490 pre-cycle) vs. the committed budget (13,968/28,490 baseline cited by the gate's own
output — unmodified). This is `decisions.md §14`'s already-reopened tension: the standing gate's
`no_record` budget trips whenever enumeration adds real, previously-invisible, not-yet-ingested
content — this cycle's growth (4,824 new `ability` units, 112 new `feat` units, none yet ingested
into `data/corpus`) is one more instance of the exact condition the reopening exists to fix, not a
new blocker. **The budget constants in `scripts/shape_coverage_standing_gate.py` are NOT modified
by this cycle** — the repin needs its own evidence-gated cycle per the dispatch brief's explicit
instruction.

## Tests

- `cargo test --locked --bin v06_work_inventory` → 323/323 (was 314; +9 new `kind_ability_tests`).
- `cargo test --locked --lib` → 2,397/2,397 (was 2,388), 13 ignored, 0 failed.
- `cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml` (separate cargo
  workspace) → 518/518 (was 517).
- `python3 -m unittest scripts.tests.test_census_independent` → 26/26 (was 20; +6 new ability
  tests, including the mutation/red-proof test).
- `python3 scripts/card15_reconcile.py` → `equals_total_this_run: True`, `remaining_undisposed: 0`.

## RED → GREEN

- **Rust:** `Kind::Ability` referenced in `kind_ability_tests` before the enum variant/`file_kind`
  row/`refine_kind` arm/`has_classifying_token` arm existed → compile error (RED). Landed →
  GREEN (9/9). `core_essentials_real_corpus_residual_never_grows_past_its_pinned_baseline` went
  RED (panic, "GREW to 448/459") the moment `Kind::Ability` made `ce_abilities.lst`'s residual
  population visible for the first time — investigated per its own instruction (not silenced), real
  new content confirmed via `DEBUG_RESIDUAL=1`, both pins raised on the SAME evidence-only terms
  the file's own precedent already established (Template/Language, 138→171) → GREEN.
- **Python — item 4's mutation/red-proof test**
  (`test_exclusion_rule_mutation_proof_widening_it_swallows_a_real_object`): widens
  `_ABILITY_CONTENT_RE` in-place to treat a bare `TYPE:` field as "content" (a plausible-looking but
  wrong widening — every picklist row also carries `TYPE:`) and asserts the widened rule wrongly
  swallows a genuine (B)-picklist fixture (`Ability Focus`'s own shape) as `ability` — RED,
  reproduced live inside the test, not asserted from memory. Reverts the regex and re-asserts the
  correct exclusion — GREEN. Proves the exclusion rule is NOT vacuously permissive: if it is ever
  widened to eat (A) rows, this test catches it.
- **`content_bearing_internal_ability_row_is_enumerated_not_dropped_by_internal_trap`**: without
  the `kind != Kind::Ability` guard on `is_internal_category`, this fixture (a real
  `CATEGORY:Internal` row with `ASPECT:` content) produces 0 units (RED — the file-wide trap fires
  first). With the guard, 1 unit lands (GREEN), and `class_feature_internal_row_is_still_dropped_by_the_blanket_trap`
  proves the guard is scoped to `Kind::Ability` alone, not a blanket loosening.

## Identifier / wired-integration audit (this cycle's own diff, scoped to touched files only)

```bash
git diff --unified=0 4d1354fef69fcf5fddbde923bc46e6f4342d511c -- src/bin/v06_work_inventory.rs \
  scripts/census_independent.py scripts/tests/test_census_independent.py \
  scripts/card15_reconcile.py \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
git diff --unified=0 4d1354fef69fcf5fddbde923bc46e6f4342d511c -- <same paths> \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
```

Both `OK_*`.

## §15 — Product Identity

No record disposed this cycle was transcribed, ingested, or scored against `ogl-pi-blacklist.md` —
enumeration only (identity/key/name/file/line, per the existing inventory schema every other kind
already uses). No PI-screening question arises at this layer; T9's paused-onboarding PI audit is
unaffected and untouched.

## §16 — a unit moved out of a shape is not a unit closed

`ability`'s 4,824 landed units are `status: not-ingested` — `classify()`'s
`not_ingested("ability_content_has_no_engine_table")` arm, matching every other recently-landed
kind (`template`/`deity`/`power`/`domain`/`language`/`skill`). They are enumerated, not engineered.
No claim of closure is made for this population beyond "it is now a named, tracked kind" — Gate 2
(engines) has not run against it.

## Files touched

- `src/bin/v06_work_inventory.rs` — `Kind::Ability` variant + doc comment; `file_kind`'s bare
  `abilit` fallback; `refine_kind`'s `Kind::Ability`/`CATEGORY:FEAT` arm; `ABILITY_CONTENT_PREFIXES`
  + `is_bonus_token` + `ability_row_has_content`; `has_classifying_token`'s `Kind::Ability` arm;
  `is_internal_category`'s `kind != Kind::Ability` guard; `classify()`'s exhaustive-match arm;
  `CORE_ESSENTIALS_RESIDUAL_DELETION_CEILING` 171→460 + doc comment; paired pinned-baseline test
  171→448 + doc comment; 9 new `kind_ability_tests`.
- `scripts/census_independent.py` — `ADDED_KINDS` extended (`ability`); `abilit` branch's
  companion/familiar routing fix; `_key_field`/`_ABILITY_CONTENT_RE`/`_ABILITY_GATEWAY_RE`/
  `_ABILITY_DUPLICATE_CHECK_KINDS`/`_collect_tracked_keys`; `count_objects`'s `row_dependent`
  branch ported disposition logic.
- `scripts/tests/test_census_independent.py` — 6 new tests (content/picklist/gateway/duplicate/
  companion-routing/mutation-proof).
- `scripts/card15_reconcile.py` — retired the `ability_category_disposition_a` pending-A entry and
  the `ability_category_gateway_picklist_duplicate` pending-B entry (now empty) into live-derived
  `already_tracked_a`/`disposed_b_applied` entries; added the companion-routing-fix entry; updated
  narrative notes.
- `docs/work-inventory.json` — regenerated through the real producer (see Population above).
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/diff.json`
  — regenerated.
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-reconcile.json`
  — regenerated.

## Next-cycle plan

1. **`class_feature` residual** (179 original + 2,574 Internal-adjudicated): needs
   `v06_work_inventory.rs`'s OWN `is_internal_category` trap narrowed for the `class_feature` kind
   specifically (a second, independent codepath from this cycle's `Kind::Ability` carve-out — the
   class_feature population's adjudicated rule is the wider `_ROW_CONTENT_FIELD_RE`, not the
   narrower `_ABILITY_CONTENT_RE` this cycle ported), not yet attempted.
2. The small (3-unit) `_abilities_feat.lst` census/inventory disagreement this cycle surfaced
   (census's `"feat" in b` check doesn't discriminate row content the way Rust's `refine_kind` now
   does) is flagged, not fixed — low priority, tiny population.
3. Card 15 reaches `complete` only when `total_kind_unenumerable_units` reaches 0 and every unit in
   the reconciled total carries a family (already true for everything currently tracked, per the
   shape ledger's `unclassified_count: 0`).

## Disk

`df -h /`: `/dev/sda1  968G  366G  603G  38% /` (measured before this cycle's writes; ample
headroom, no pruning needed).
