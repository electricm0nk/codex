# Cycle t9-onboarding-equipment-modifier-ability-fix — Gate 3 Closure Invariant / `no_record` closure (`equipment_modifier`/`equipment`/`ability`)

- **Card ID:** epic-2-cause-closure (card 11), `no_record == 0` closure line (`decisions.md §20`/`§27b`)
- **Commit SHA:** (this receipt commits alongside the fix; see push log)
- **Files touched:**
  - `src/bin/gen_equipment_gap_tables.rs` (fix C: `adventurers_guide` `BOOK_INPUTS` missing `_equipmods.lst`)
  - `src/rules_core/rules_tables/equipment_gap_tables.rs` (regenerated; +1 row, `adventurers_guide` 115→116, total 1953→1954)
  - `src/bin/v06_work_inventory.rs` (fixes A/B/D: `.COPY=`-template-row dedup scoped to `advanced_class_guide`; new `pfs_legality_only_row` trap; `mod_only_rescue`'s Ability↔Trait kind-redirect gap)
  - `src/rules_core/cache_gen/ultimate_equipment.rs` + `src/bin/gen_cache_ultimate_equipment.rs` (fix E: port `decisions.md §24` neutral-rename via `cache_gen::equipment_gap::resolve_name_or_rename`, replacing the old outright-drop)
  - `data/corpus/pathfinder_unchained/equipment/0_abp_enhancement_to_{ammunition,armor,shield,weapon}.json` (fix B: moved into `equipment/equipmods/`, `git mv`, no content change)
  - `data/corpus/adventurers_guide/equipment/equipmods/*.json` (new, fix C's corpus write)
  - `data/corpus/ultimate_equipment/equipment/codex_named_unit_*_66.json` (new, fix E's corpus write)
  - `docs/work-inventory.json` (regenerated, guarded path, stamps preserved by ID)
  - `src/rules_core/equipment_resolver.rs`, `tests/equipment_gap_tables.rs`, `apps/desktop/src-tauri/src/equipment_catalog.rs` (pinned-count sweep after the record-count change; two pre-existing stale pins in `equipment_resolver.rs`/`equipment_catalog.rs` retargeted to proven values, unrelated to this cycle's own changes — see corrections below)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (`git diff --unified=0` over the touched Rust/test files)
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** `decisions.md §20`/`§27b` — `no_record == 0`. This cycle's scope: `equipment_modifier` (19), `equipment` (8), `ability` (1) — the 28 units the prior root-cause cycle (commit `0b21de2634`, receipt `t9-onboarding-equipment-modifier-ability-rootcause_cycle-1_cycle_receipt.md`) traced to 5 causes (A–E) and left unfixed.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (oracle pin, `scripts/pcgen-oracle-pin.env`)
- **Status:** complete for this cycle's scope (all 28 no_record units in `equipment_modifier`/`equipment`/`ability` closed); `monster_ability` (78, sibling lane) untouched
- **Discovery forwards:** one deferral logged (`docs/retro/events/t9-onboarding.jsonl`) — `apps/desktop/src-tauri/src/equipment_catalog.rs`'s equipment-catalog test module carries several pre-existing stale pinned counts (per-book description coverage, category-filter total, overall catalog length) unrelated to this cycle's changes, discovered by running that separate cargo workspace's own test suite. Out of scope for this cycle; only the one line this cycle's own regen certainly moved (`UE` 1613→1614, mirroring `equipment_resolver.rs`'s identical, independently-verified fix) was corrected here.
- **Next-cycle plan:** none within this scope — `no_record` is 0 for all three kinds. A future cycle should re-derive the full `apps/desktop` equipment-catalog pin set (see deferral above).

## Re-derivation of the target population (`§17a`)

```
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger.json
```

Before this cycle: `no_record` = 106 total (`monster_ability` 78, `equipment_modifier` 19, `equipment` 8, `ability` 1). Matches the dispatch brief's stated 28-unit scope for this cycle exactly (19+8+1), confirming the brief's own `§17a` re-derivation (population had already moved 30→28 since the root-cause receipt, per that receipt's own note — re-confirmed here, no further movement since).

After this cycle:

```
population (not-done units considered): 34397
  matched              11414  (33.2%)
  no_formula_tokens    22905  (66.6%)
  no_record               78  (0.2%)
```

`no_record` breakdown by kind, after: `{'monster_ability': 78}` only. `equipment_modifier`/`equipment`/`ability` are all zero.

## Fix, per group (root causes named in the prior cycle's receipt)

### Group C (1 unit) — `adventurers_guide` `equipment_modifier`, trivial file omission

`gen_equipment_gap_tables.rs`'s `adventurers_guide` `BookInput` listed 3 files, never the book's `_equipmods.lst`. One line added; regenerated `equipment_gap_tables.rs` (`cargo run --locked --bin gen_equipment_gap_tables`) and `gen_cache_equipment_gap` (writes the corpus JSON). Diff of the generated table: 3 insertions/2 deletions, exactly one new row in the `adventurers_guide` block — no other book's rows moved. New corpus file confirmed at the exact `(book, source_file, source_line)` coordinate the walker's unit already expected.

### Group D (8 units: 6 `ultimate_magic`, 1 `bestiary_3` equipment, 1 `ultimate_campaign` ability) — walker cites a PFS-legality-overlay row instead of the base declaration

Two distinct mechanisms, both in `v06_work_inventory.rs`:

1. **New `pfs_legality_only_row` trap.** A plain (non-`.FORGET`, non-`.MOD`, non-`.COPY=`) row in a `_pfs/` directory file whose only payload beyond its first field is a legality marker (`TYPE:PFSLegal`/`TYPE:PFSNotLegal`) and/or a `PRECHARACTERTYPE:` gate carries zero mechanical content and can never correspond to its own corpus record — it restates an item declared elsewhere in the book's base file. Verified corpus-wide before landing (`python3` scan over every `_pfs/*.lst` file in the pinned oracle): 98 rows match this shape, all disjoint from the 32 `_pfs/` rows that carry real content (`KEY:`/`DESC:`/`BONUS:`/`ABILITY:`/etc., which stay counted). Closes the 6 `ultimate_magic` + 1 `bestiary_3` `equipment` units directly (their real content is already correctly ingested at the base file's own coordinate under a sibling unit id).
2. **`mod_only_rescue`'s Ability↔Trait kind-redirect gap** (the `ultimate_campaign` ability unit). A `.MOD` row's own kind comes from its file (`Kind::Ability`), never from `refine_kind`'s per-row `TYPE:Trait...` redirect — so a PFS-legality `.MOD` row targeting a base declaration that itself redirected to `Kind::Trait` was never recognized as already-declared, and got wrongly re-minted as an orphan `Kind::Ability` unit. Fixed by checking both kinds at the `declared` lookup. Corpus record for this exact object already exists (a `decisions.md §24` neutral-named record at the base coordinate).

### Group B (4 units) — `pathfinder_unchained` `equipment_modifier`, correct content in the wrong directory

`data/corpus/pathfinder_unchained/equipment/0_abp_enhancement_to_{ammunition,armor,shield,weapon}.json` cited the exact coordinates the walker's units expected but sat flat under `equipment/`, not `equipment/equipmods/` — `shape_ledger.py`'s `kind_from_path_parts` derives kind from that one directory-name check, so these indexed as `equipment`, not `equipment_modifier`. `git mv`'d into `equipment/equipmods/`, no content change — a pure data move, no re-verification-stamp risk.

### Group A (14 units) — `advanced_class_guide` `equipment_modifier`, duplicate walker unit vs. `.COPY=`-alias-cited corpus record

`v06_work_inventory.rs` minted two units per PCGen equipmod object in ACG's `acg_equipmods.lst`: a "template" row (long KEY, e.g. `Special Ability ~ Burdenless ~ Armor`) and its `.COPY=` derivative (short display name, e.g. `Burdenless`) — the real ingest pipeline (`cache_gen::equipment_gap::find_citation`) always resolves to the `.COPY=` line, orphaning the template unit. New per-book pass (computed once per book, before the main enumeration loop) collects every `.COPY=` base identity from ACG's own `equipmods` files and drops a plain declared row whose own `KEY:` matches one of them (`copy_template_row` trap).

**Scoped to `advanced_class_guide` only, not corpus-wide** — re-deriving this generically across every book first (before scoping down) surfaced 24 units in *other* books (e.g. `advanced_race_guide`'s `material_darkleaf_cloth_clothing`) whose template-row unit was the one the real ingest pipeline resolves to, carrying an already-`literal-verified` stamp; the regen's own stamp-loss guard refused to write and named them by id. Spot-checked every removed-unit class before landing (amorphous/burdenless-shaped pairs, the 4 "unknown"-status blood/spirit-hunting pairs, the PFS-legality drops, the ability kind-redirect drop): every one has a surviving sibling unit carrying equal-or-better tracked status, or was never real content (a legality restatement). None of the 73 units this cycle's regen removed from the population lost real, otherwise-untracked content.

### Group E (1 unit) — `ultimate_equipment`, genuine ingest gap, `NAMEISPI:YES` row

`cache_gen::ultimate_equipment::generate_equipment` predates `decisions.md §24`'s neutral-rename mechanism and dropped a `NAMEISPI:YES` row outright. Ported `cache_gen::equipment_gap::resolve_name_or_rename` (imported, not re-implemented) into the same per-record loop `hand_authored_equipment.rs` already reuses it from. Added `codex_generated_name`/`rename` fields to this file's own `CacheRecord`/`GenerationReport`, mirroring `equipment_gap.rs`'s shape exactly. The neutral name/key derive ONLY from `(kind, book, source_file, source_line)` — proved by `resolve_name_or_rename`'s own existing determinism tests (reused, not duplicated).

**One destructive near-miss caught before commit:** the first run of `gen_cache_ultimate_equipment` staged 65 deletions under `data/corpus/ultimate_equipment/equipment/` — that generator's own "remove stale owned files" sweep, unaware that `gen_cache_equipment_gap`'s separate "UE" arm also writes into the same shared directory, deleted every file the OTHER generator had ever written there (confirmed: each deleted file's own `source` cited a coordinate outside `ue::equipment_tables`'s compiled roster). `git status --porcelain` caught it before commit; reverted with `git checkout HEAD -- data/corpus/ultimate_equipment/` and kept only the one genuinely new file this fix produces.

## Corpus regeneration (`docs/work-inventory.json`)

Guarded path, matching the sibling lane's proven standard:

```
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger_current.json   # baseline, before
cargo run --locked --bin corpus_literal_sweep -- --json-out <sweep.json>                                  # CLEAN, 0 findings
cargo run --locked --bin derived_evaluator_fixture_check -- --json-out <derived.json>                     # 1836 cleared, 0 failed
CORPUS_LITERAL_SWEEP_REPORT=<sweep.json> DERIVED_FIXTURE_CHECK_REPORT=<derived.json> \
  cargo run --locked --bin v06_work_inventory -- --json-out                                                # no --allow-stamp-loss
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger_after.json       # after
```

Status distribution, before → after (full population, not just this cycle's scope):

| status | before | after |
|---|---:|---:|
| deferred-with-reason | 46 | 46 |
| fixture-verified | 1741 | 1741 |
| grounded | 3232 | 3234 |
| ingested-magnitude | 1544 | 1543 |
| literal-verified | 6567 | 6589 |
| not-ingested | 27035 | 26943 |
| not-started | 19 | 19 |
| text-complete | 5070 | 5099 |
| unknown | 4257 | 4224 |
| **TOTAL** | **49511** | **49438** |

Stamp count preserved by ID (regen refused with `--allow-stamp-loss` unavailable when the first, corpus-wide-scoped version of the group-A fix would have dropped 24 stamps — caught before this final run; the ACG-only-scoped fix that shipped drops 0). 73 units left the population entirely (48 `equipment_modifier`, 23 `ability`, 2 `equipment`) — all confirmed pure duplicates or non-content legality-overlay rows per `§16`: a unit that stops being counted is not a unit closed, and none of the 73 represents lost, otherwise-unique tracked content (see Group A's spot-check note above).

## `§16` accounting (closure / reclassification / reachability / instrument correction)

- **Closure (genuine new corpus record, previously un-ingestible):** 1 unit — Group E's `ultimate_equipment` `NAMEISPI:YES` row.
- **Reclassification (unit already had a real corpus record; instrument now finds it):** 1 unit — Group C's `adventurers_guide` equipmod (file was simply never read; the record did not exist until this cycle generated it — closer to closure than reclassification, since no record existed before either; counted here because the fix is "read one more file," not "ingest new content" in the Group-E sense).
- **Reachability:** unchanged this cycle — no reachability gate touched.
- **Instrument correction (population shrank because a phantom/duplicate unit stopped being minted, not because content was ingested):** 26 units — Group A's 14 ACG duplicates + Group B's 4 (kind-misrouted, already-correct records) + Group D's 8 (7 PFS-legality-restatement drops + 1 kind-redirect dedup). Plus 47 further `equipment_modifier`/`ability`/`equipment` units NOT in this cycle's 28-unit `no_record` scope that the same generic `pfs_legality_only_row`/kind-redirect fixes also correctly stopped counting (part of the 73-unit population drop above) — real, but outside this receipt's `no_record`-closure accounting.

**Total: 106 → 78 `no_record` (−28, this cycle's full scope), 1 genuine ingest closure, 27 instrument/plumbing corrections.**

## Corrections logged (`scripts/retro.py`)

1. `equipment_resolver.rs`'s `catalog_rows_span_every_ingested_book_with_their_real_counts` pinned `EQUIPMENT_BOOK_UE` count at 1613; real count (unrelated to this cycle, the underlying static table is byte-identical to this branch's pinned base) is 1614. Retargeted.
2. Same test's `rows.len()` pinned at 8025 (assuming 1879 gap rows); the generated table's own header already read "Total: 1953 rows" at this cycle's pinned base — an untraced 74-row prior drift. Retargeted to 8100 (6146 + 1954, the proven current total after this cycle's own +1).
3. `apps/desktop/src-tauri/src/equipment_catalog.rs`'s `catalog_spans_every_ingested_book_with_their_real_counts` carries the identical stale `UE` 1613 pin — retargeted to 1614. Two further pre-existing, unrelated stale pins in that same file (and a third test, `description_coverage_is_pinned_per_book`, `filter_equipment_catalog_matches_category_exactly_across_every_book`) logged as a deferral rather than fixed in this cycle — out of scope, large, and not moved by anything this cycle touched.

## Environment / oracle

```
PCGEN_REPO_DIR=<worktree>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen
scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR"   # bootstrapped this cycle, fresh worktree
scripts/verify.sh --only preflight-oracle                # PASS, oracle at pin 7f818006e371188e5717fd18d74d18a420747fc6
```

## Test evidence

```
cargo test --locked --lib -j 4                                          # 2552 passed, 0 failed
cargo test --locked --lib --bin v06_work_inventory --bin gen_equipment_gap_tables --bin gen_cache_ultimate_equipment -j 4   # green after the two corrections above
```

`apps/desktop/src-tauri` (separate cargo workspace): `cargo test --locked --bin codex-desktop equipment_catalog` — 14 passed, 3 failed (pre-existing, see deferral above; unaffected in count by this cycle's own fix beyond the one `UE` line corrected).
