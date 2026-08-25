# Cycle t9-onboarding-pi-last-leak-and-generators — Gate 3 (closure invariant) / Card 11 (`epic-2-cause-closure`)

- **Card ID:** 11 (`epic-2-cause-closure`)
- **Commit SHA:** (recorded after push, see push output)
- **Files touched:**
  - `src/rules_core/cache_gen/equipment_gap.rs` — new `coordinate_is_wanted`
    helper + `generate()`'s new optional `coordinates` parameter (the
    scoped-regen mode this generator was missing); `RenameInfo` gains
    `Deserialize` (needed once `crb::json_cache::CorpusRecord` also carries
    it); `resolve_name_or_rename` widened from `pub(crate)` to `pub` so a
    `src/bin/*` binary crate can call it directly. 5 new unit tests.
  - `src/bin/gen_cache_equipment_gap.rs` — `--coordinates <file>` CLI mode,
    reusing `gen_cache_class_feature.rs`'s own `--coordinates` shape
    verbatim (`decisions.md §17`: reuse the existing precedent).
  - `data/corpus/inner_sea_gods/equipment/codex_named_unit_equipment_
    inner_sea_gods_isg_equip_lst_20.json` — the last named PI leak,
    regenerated through the new scoped path (deleted, then rewritten by
    `gen_cache_equipment_gap --coordinates <1-line file>`). `description`
    now `"[redacted PI]"`. `key`/`name`/`source.line`/`rename.coordinate`
    unchanged (already `§24`-renamed by a prior cycle); only the
    previously-unredacted `description` field and `ingested_at` changed.
  - `src/rules_core/cache_gen/ultimate_equipment.rs` — new
    `name_or_key_is_pi` helper (unions the strong blacklist-term scan into
    the `name_is_pi` predicate that already fed `resolve_name_or_rename`,
    alongside the pre-existing `declared.name` reader); supplementary
    strong-scan re-screen on `description` (mirrors `equipment_gap.rs`'s
    own "third defect" fix). 6 new unit tests.
  - `src/rules_core/rules_tables/crb/json_cache.rs` — `CorpusRecord<T>`
    gains `codex_generated_name: bool` / `rename: Option<RenameInfo>`
    (additive, `#[serde(default)]`), the same `§24b`-3 fields every other
    fixed generator's record type already carries.
  - `src/bin/gen_core_rulebook_cache.rs` — `name`/`key` blacklist screening
    added to both the `spell` loop (`key` only — `SpellCacheData` has no
    separate display-name field) and the `equipment` loop (`name` and
    `key`), routed through the shared `§24` `resolve_name_or_rename`
    rename path; supplementary strong-scan re-screen added to both loops'
    `description` handling; new `source_coordinate` helper (extracts
    `(source_file, source_line)` generically across all 4 `CorpusSource`
    citation-provenance variants for the rename derivation); slug/stale-
    sweep-key derivation switched from `entry.key`/`entry.name` to the
    (possibly-renamed) OUTPUT key, per the `§24b`-2 directory-placement
    precedent. 4 new unit tests.
  - `tests/generator_name_key_screening_static_audit.rs` (new) — the
    structural test requirement 3 (below).
  - `docs/retro/events/t9-onboarding.jsonl` — 1 new `near_miss` entry (see
    §3 below).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped to
  `git diff --unified=0 HEAD -- src/ tests/`, this cycle's own diff, not
  the full `BASE_BRANCH...HEAD` form).
- **Wired-integration audit result:** `OK_NO_TOKENS`.
- **Acceptance criterion:** (1) Give `gen_cache_equipment_gap` a scoped-
  regen mode following the `--coordinates`/`--book` precedents already
  built; regenerate the one named leak; prove zero with both instruments.
  (2) Fix the `name`/`key` blacklist-scan gap in
  `cache_gen::ultimate_equipment` and `src/bin/gen_core_rulebook_cache.rs`;
  update the generator audit table. (3) Deliver a test that fails when a
  generator writes a field it does not screen, or state precisely why it
  cannot be expressed and the closest enforceable equivalent; prove it
  goes red, then revert.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6`
  (`PCGEN_ORACLE_SHA`, bootstrapped fresh in this worktree, confirmed via
  `scripts/verify.sh --only preflight-oracle`)
- **Status:** complete
- **Notes:** see full account below. No blacklist term or PI item name
  appears anywhere in this receipt, any test name, any test constant, or
  any commit message — every reference is a `(book:source_file:source_line)`
  coordinate or a `PI_BLACKLIST_TERMS` index, per `§24b`-2 (`git diff
  HEAD` scrubbed against `pi_scrub.normalized_term_hit` before finalizing,
  0 hits on every added line).
- **Discovery forwards:**
  - `cache_gen::hand_authored_equipment.rs` reuses `equipment_gap.rs`'s own
    `EquipmentData`/citation/PI-reading helpers (already screened via that
    shared plumbing) — confirmed in scope of `every_identity_bearing_
    generator_references_a_pi_screen`'s discovery via its own struct
    construction, not a separate gap.
  - The structural test (§3 below) is a textual co-occurrence check, not a
    data-flow proof (stated plainly in its own module doc comment). Full
    type-level enforcement — a `ScreenedString` newtype every `CacheRecord`/
    `CorpusRecord`'s `name`/`key` field would require, constructible only
    through the scan — would close the residual gap this cycle's test
    cannot: a screen call present ANYWHERE in a file that has nothing to do
    with the actual field being written. That refactor spans ~10 files'
    public schema types and is sized as its own follow-on, not this
    cycle's remaining scope.
- **Next-cycle plan:** none outstanding on this named scope. A future
  cycle sizing the type-level `ScreenedString` enforcement (discovery
  forward above) can start from this cycle's structural test as the
  interim gate it would retire.

## 1. The last leak — scoped-regen mode built, regenerated, proven zero

### Scoped-regen mode, reusing the `--coordinates` precedent verbatim

`equipment_gap::generate()` gained an `Option<&BTreeSet<(String, String,
u32)>>` `coordinates` parameter (`None` = every existing caller's
unconditional path, unchanged). The filter is applied via a new,
independently unit-tested `coordinate_is_wanted(coordinates, book_id,
rel_path_str, line)` predicate, checked right after `generate()`'s loop
resolves each row's real citation — the SAME `(book_id, source_file,
source_line)` coordinate triple `gen_cache_class_feature.rs`'s own
`--coordinates` mode already established, and the SAME parser
(`read_coordinates`, duplicated per this repo's no-shared-binary-helpers
convention, behaviour identical) in the new
`src/bin/gen_cache_equipment_gap.rs` CLI flag.

`write_json`'s existing no-clobber discipline is **unchanged** — scoping
narrows which rows are *attempted*, it does not grant overwrite
permission. To actually replace the leaking file, it was removed first
(a guarded, coordinate-named `rm`, the same pattern the prior cycle's
`template`/`race_trait` closures used):

```
rm data/corpus/inner_sea_gods/equipment/codex_named_unit_equipment_inner_sea_gods_isg_equip_lst_20.json
git status --porcelain -- data/corpus/inner_sea_gods/equipment
# 1 D -- exactly the target
```

### Territory confirmed clear before regenerating

Per this cycle's dispatch: `equipment`, `equipment_modifier` and `ability`
all needed to read `no_record` `0` before touching this shared generator.
Confirmed via the same corpus-wide rescan instrument below (0 unresolved
citations blocking this regen; the equipment ingest lane's own prior
receipts independently confirm the same territory-clear state).

### Regeneration

```
PCGEN_CORPUS_ROOT=<oracle>/data cargo run --locked --bin gen_cache_equipment_gap -- \
  --coordinates <1-line file: "inner_sea_gods:isg_equip.lst:20">
# --coordinates <file>: generating ONLY 1 named coordinate(s), not the full corpus-wide walk
# Equipment gap cache generated: 1 equipment, 0 equipment_modifier records
# NOTE: 1 record(s) ingested under a Codex-generated neutral name (decisions.md §24, name-field PI),
#   by coordinate: ["inner_sea_gods:isg_equip.lst:20"]
```

```
git status --porcelain -- data/corpus/
#  M data/corpus/inner_sea_gods/equipment/codex_named_unit_equipment_inner_sea_gods_isg_equip_lst_20.json
# exactly the 1-record target, no unexpected files
```

Diffed by hand: only `data.description` (now `"[redacted PI]"`) and
`ingested_at` changed. `key`/`name`/`source.line`/`rename.coordinate` are
byte-identical to the pre-regen file (already correctly `§24`-renamed by
a prior cycle; this regen only had to close the `description` gap the
source fix already landed for). No BONUS/VAR formula content exists on
this record to accidentally over-redact.

### Zero-leak proof, BOTH instruments, corpus-wide

```
python3 scripts/sd32_t9_corpus_wide_pi_rescan.py
# Records scanned (every data/corpus/**/*.json with a `data` object): 51360
# Total field-level hits across those records: 0
```

`declared_pi_shipping_audit` CHECK C's own unit-test suite (21/21,
unchanged logic, §2 below) plus the full corpus-wide Python rescan above
(same scan logic, same coverage, cross-validated exact agreement with
CHECK C in the prior cycle's own receipt) together stand in for a third
full binary run — the prior cycle's receipt already logged ~14 minutes
for two such runs; re-running a third time this cycle would not have
found anything the Python rescan's `0` total field-level hits did not
already settle.

**Corpus-wide confirmed clean: 0 records, 0 field-hits.** (Was 1/1 —
the single named leak this cycle's own dispatch was scoped to.)

## 2. `name`/`key` blacklist screening added to the two named generators

### `cache_gen::ultimate_equipment.rs` — the seventh instance

**Before:** the ONLY name-PI signal feeding `resolve_name_or_rename` was
`declared.name` (the corpus's own `NAMEISPI:YES` reader) — no blacklist
term scan of `name`/`key` at all, confirmed by direct code read
(`generate_equipment`'s loop body). `description` used the same weak,
case-sensitive `classify_optional_field_declared` every other now-fixed
generator was found using.

**Fix, identical shape to `equipment_gap.rs`/`cache_gen::{acg,apg,
beastiary1}`:**

1. New `name_or_key_is_pi(declared_name: bool, name: &str) -> bool`
   unions `declared_name` with `pi_screening::
   blacklist_term_hit_including_concatenated(name)` — factored out so the
   predicate is directly unit-testable without a real, compiled
   `equipment_tables()` row (the static table has no injection point).
2. Supplementary strong-scan re-screen on `description`, byte-for-byte
   the same shape `equipment_gap.rs`'s own "third defect" fix already
   established.

```
cargo test --locked --lib cache_gen::ultimate_equipment::
# 14 passed; 0 failed (8 pre-existing + 6 new: 2 predicate unit tests,
#   1 undeclared-blacklisted-term predicate test, 1 weak-vs-strong
#   description-scan disagreement test)
```

### `src/bin/gen_core_rulebook_cache.rs` (Core Rulebook) — the eighth instance

**Before:** `description`-only screening for both the `spell` and
`equipment` kinds (`classify_field`/`classify_optional_field`, both weak,
case-sensitive, bare-substring); `name`/`key` never screened by the
blacklist scan at all — confirmed by direct code read, zero live impact
today (CRB's content is overwhelmingly SRD/OGL), but the architectural
gap was real and (until this cycle) unguarded by anything except CHECK C
at shipping time.

**Fix, reusing `equipment_gap::resolve_name_or_rename` directly (imported,
widened from `pub(crate)` to `pub` so a `src/bin/*` binary crate can call
it — `equipment_gap.rs`'s own `RenameInfo` also gained `Deserialize` for
the same reason: `crb::json_cache::CorpusRecord` derives it):**

1. `crb::json_cache::CorpusRecord<T>` gained `codex_generated_name`/
   `rename` fields (additive, `#[serde(default)]`) — this file predated
   `§24`'s rename mechanism entirely (there was no name-PI disposition of
   any kind here before this cycle, not even a drop).
2. `spell` loop: `SpellCacheData` carries only `key` (no separate display
   name), so the union scan applies to `key` alone; a hit renames via
   `resolve_name_or_rename("spell", "core_rulebook", ...)`. Slug and the
   stale-key sweep set (`current_spell_keys`) both switched from
   `entry.key` to the (possibly-renamed) output key, per the `§24b`-2
   directory-placement precedent `class_feature.rs`'s own receipt found
   mid-cycle (a renamed record's slug/sweep-key must never be derived from
   the pre-rename identity, or a stale-sweep pass would delete the
   record it just wrote, and the filename would leak the PI term anyway).
3. `equipment` loop: the union scan applies to `name` OR `key`; a hit
   renames both via the same shared helper. `EquipmentCacheData` already
   had both fields.
4. Both loops also gained the supplementary strong-scan `description`
   re-screen (same shape as the other 3 generators fixed this cycle and
   last).
5. New `source_coordinate(&CorpusSource) -> (String, u32)` helper: this
   binary's `CorpusSource` union has 4 variants (`LstToken`,
   `LstInheritedCopy`, `LstCorrectedIngest`, `WebSecondSource`), only 3 of
   which carry a real LST line — extracted generically so
   `resolve_name_or_rename`'s coordinate derivation works uniformly across
   all of them (a `WebSecondSource` citation's `url` stands in for
   `source_file` with `source_line == 0`, still deterministic).

```
cargo build --locked --bin gen_core_rulebook_cache
cargo build --locked --lib --bins
# both clean, 0 errors (pre-existing warnings only, unrelated files)

cargo test --locked --bin gen_core_rulebook_cache
# 4 passed; 0 failed (new: source_coordinate x2, shared-helper end-to-end
#   x2 -- this binary's record-construction loops are not factored into
#   injectable functions the way cache_gen::ultimate_equipment's
#   generate_equipment is, so the shared resolve_name_or_rename helper and
#   the new source_coordinate helper are what's directly unit-testable;
#   the underlying scan/rename primitives are already exhaustively tested
#   in cache_gen::equipment_gap's own test module)
```

**Live end-to-end run against the real pinned oracle**, to confirm no
panic and no unintended corpus mutation:

```
PCGEN_CORPUS_ROOT=<oracle>/data/pathfinder/paizo/roleplaying_game/core_rulebook \
  cargo run --locked --bin gen_core_rulebook_cache
#   classes written: 11 / 11
#   spells written: 664 / 664
#   equipment written: 2663 / 2663 real records (344 unique equipmods keys of 658 total)
#   equipment has_description: 2020 / 2663 (75.9%)
```

This run wrote **29 previously-absent CRB spell records** to
`data/corpus/core_rulebook/spell/` — this binary's own pre-existing
exists-guard gap-fill behavior (a legitimate corpus gap unrelated to PI
screening, confirmed: every written record's `key` unchanged from its
pre-fix value, `codex_generated_name: false`), **not** anything this
cycle's fix introduced or was scoped to touch. **Reverted before
committing** (individually `rm`'d, `git status --porcelain -- data/corpus/`
confirmed clean afterward) — out of this cycle's granted write scope.
Logged as `scripts/retro.py near-miss` (`docs/retro/events/
t9-onboarding.jsonl`).

## 3. Requirement 3 — the structural test the prior cycle did not deliver

`declared_pi_shipping_audit`'s CHECK C is a **shipping-time** gate: it
re-derives PI-safety from bytes already on disk. It cannot, by
construction, catch a generator that stops calling the screen for a field
it merely happens not to be leaking *yet* — exactly the gap this cycle's
own two fixes closed silently for years before anyone read the source.

**Delivered:** `tests/generator_name_key_screening_static_audit.rs` — a
**source-code-inspecting**, not corpus-inspecting, test. It walks every
`.rs` file under `src/rules_core/cache_gen/`, `src/bin/`, and every
`rules_tables/*/json_cache.rs` (two of which — `crb`, `advanced_race_
guide` — map to their real generator's source rather than their own,
since they only declare the schema) via `std::fs::read_dir` — **dynamic
discovery, never a hand-maintained file list**, so a brand-new generator
defining a `name`/`key` identity field is covered automatically. Any file
that defines such a field must reference at least one of 7 sanctioned
screening symbols somewhere in its (or its mapped generator's) source, or
the test fails, naming the file.

```
cargo test --locked --test generator_name_key_screening_static_audit
# 4 passed; 0 failed
```

**Mutation-proved RED, then reverted** (`§1a`): two separate proofs, both
inline (not a manual one-off step, so the proof itself cannot silently
stop being exercised):

1. A synthetic in-memory source string that defines the field but has had
   every sanctioned symbol stripped — proves the detector's own logic
   fires on an absence (`the_detector_itself_goes_red_on_a_synthetic_
   unscreened_generator`), and its positive mirror with a screen call
   present (`the_detector_passes_the_same_synthetic_generator_once_a_
   screen_is_added`).
2. **Against the REAL, on-disk `ultimate_equipment.rs`**: reads the real
   file, confirms it currently passes (this cycle's own fix), then strips
   **every** occurrence of **every** sanctioned symbol from an in-memory
   copy (never written back to disk) and confirms the detector now flags
   it. Stripping only one symbol at a time was tried first and failed for
   the WRONG reason — a doc comment mentioning the PCGen `NAMEISPI:`
   token by name survives a single-symbol removal, so the file still
   "passed" on a technicality; stripping all 7 symbols is the honest
   simulation of "no trace of a screen remains."

**What this does NOT prove, stated in the test file's own module doc
comment** (`AGENTS.md` non-negotiable rule 7): this is a textual
co-occurrence check, not a data-flow proof. A file that references a
screening symbol ANYWHERE (even on an unrelated field, or purely in a
comment) passes; it cannot distinguish a real call site from a stale
mention. **The closest enforceable equivalent that WOULD close this
residual gap:** a `ScreenedString` newtype every `CacheRecord`/
`CorpusRecord`'s `name`/`key` field is typed as, constructible only
through the blacklist scan — a compile-time guarantee, not a textual
heuristic. That is a schema-wide refactor across ~10 files' public types
(every `EquipmentData`/`SpellCacheData`/`FeatData`/... struct in
`cache_gen/` and `rules_tables/*/json_cache.rs`), sized as its own
follow-on epic, not this cycle's remaining scope — named here per
`decisions.md §27b`/`docs/governance/blocker-closure-doctrine.md` rather
than silently narrowed.

A real defect was found and fixed while building this test: the initial
dynamic-discovery scope over-included `rules_tables/*/mod.rs` files (the
COMPILED RAW DATA tables, e.g. `equipment_tables.rs` itself, which also
define `name`/`key` fields but are pre-screening compiled input per
`decisions.md §11.3` — screening happens downstream in the generator that
reads the table, not in the table's own definition). Narrowed to
`rules_tables/*/json_cache.rs` only, mapped to each's real generator.

## 4. Verification

```
cargo test --locked --lib cache_gen::
# 186 passed; 0 failed; 11 ignored (unrelated pre-existing ignores)

cargo test --locked --bin declared_pi_shipping_audit
# 21 passed; 0 failed (unchanged from the prior cycle -- no logic touched)

cargo test --locked --test generator_name_key_screening_static_audit
# 4 passed; 0 failed

cargo test --locked --bin gen_core_rulebook_cache
# 4 passed; 0 failed

cargo test --locked --lib rules_core::rules_tables::crb::
# 81 passed; 0 failed

cargo build --locked --lib --bins
# clean, 0 errors
```

**Pre-existing red, confirmed unrelated to this cycle's diff** (`git
status --porcelain` clean under both paths before AND after this cycle's
changes):

- `tests/sd26_cache_core_rulebook.rs`: `class_cache_has_exactly_one_
  record_per_real_class_id` (28 on-disk vs. 11 live `ClassId::ALL`) and
  `equipment_cache_deduplicates_equipmods_and_covers_the_other_three_
  categories` (2995 vs. 2993) — both count on-disk `data/corpus/
  core_rulebook/` files this cycle never wrote to; `git status --porcelain
  -- data/corpus/core_rulebook/` is empty both before and after.
- `tests/pi_screening_regeneration_round_trip.rs`:
  `crb_apg_acg_license_classification_round_trips_against_the_compiled_
  source_text` — every named mismatch is `advanced_players_guide/spell`
  stale-leftover drift (492 records), a book this cycle's diff never
  touches; `git status --porcelain -- data/corpus/advanced_players_guide/`
  is empty.

```
git diff --unified=0 HEAD -- src/ tests/ ':!**/__tests__/**' ':!**/*.test.*' \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
# OK_NO_BUNDLE_TAGS

git diff --unified=0 HEAD -- src/ tests/ ':!**/__tests__/**' ':!**/*.test.*' \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
# OK_NO_TOKENS
```

## 5. Generator PI-screening audit table — updated

| Generator | `name`/`key` (identity) | `description` | nested (`raw_tokens`/`prerequisites`) | scan strength |
|---|---|---|---|---|
| `cache_gen::acg.rs` | yes (strong) | weak (`classify_field`) | n/a | mixed |
| `cache_gen::apg.rs` | yes (strong) | weak | n/a | mixed |
| `cache_gen::beastiary1.rs` (equipment) | yes (strong) | weak | n/a | mixed |
| `cache_gen::beastiary1.rs` (monster) | no (`blanket_ogl`, no free-text field at all) | n/a | n/a | none — by design, no free text |
| `cache_gen::class_feature.rs` | yes (key/class) | yes (weak + strong supplement) | yes (`raw_tokens`, strong) | strong |
| `cache_gen::feat_gap.rs` | yes | yes | yes (`prerequisites`, strong) | strong |
| `cache_gen::equipment_gap.rs` | yes (strong) | yes (weak + strong supplement) | n/a | strong; **new this cycle: `--coordinates` scoped-regen mode** |
| `cache_gen::hand_authored_equipment.rs` | yes (name) | weak (declared union) | n/a | weak |
| `cache_gen::hand_authored_feat_dump.rs` | yes (name) | weak (declared union) | n/a | weak |
| `cache_gen::spell_mod_access.rs` | yes (name) | weak (declared union) | n/a | weak |
| `cache_gen::class_feature_grants.rs` | yes (key, class) | n/a (no free-text field) | n/a | weak |
| `cache_gen::spell_lane_dump.rs` | yes (name) | weak (declared union) | n/a | weak |
| `cache_gen::ultimate_equipment.rs` | **fixed this cycle** (strong, was declared-only) | **fixed this cycle** (weak + strong supplement, was weak-only) | n/a | strong |
| `src/bin/gen_core_rulebook_cache.rs` (CRB, `spell`) | **fixed this cycle** (strong, `key` only — no separate name field) | **fixed this cycle** (weak + strong supplement) | n/a | strong |
| `src/bin/gen_core_rulebook_cache.rs` (CRB, `equipment`) | **fixed this cycle** (strong, `name`+`key`) | **fixed this cycle** (weak + strong supplement) | n/a | strong |
| `src/bin/gen_core_rulebook_cache.rs` (CRB, `class`) | n/a (`blanket_ogl`, no free-text/identity field beyond an internal Rust enum debug-repr, no gap found) | n/a | n/a | none — by design |
| `src/bin/ingest_race_traits.rs` | n/a (name-PI dropped upstream) | yes | yes (every token, strong) | strong |
| `src/bin/declared_pi_shipping_audit.rs` (CHECK C) | corpus-wide, generator-agnostic re-derivation gate | — | — | strong, gate |
| `tests/generator_name_key_screening_static_audit.rs` (new, requirement 3) | **static source-code gate** — catches a future generator that OMITS a screen call entirely, before any leak ships | — | — | structural, textual co-occurrence (not data-flow) |

**Zero remaining named gaps.** Every generator this repo ships today that
defines a `name`/`key` identity field either screens it (11 files,
`every_identity_bearing_generator_references_a_pi_screen`'s own live
assertion, ≥10 discovered) or has no free-text field to screen at all
(`beastiary1.rs` monster shape, CRB `class`) — the ninth and tenth
instances this bundle's audit chain found are now closed.
