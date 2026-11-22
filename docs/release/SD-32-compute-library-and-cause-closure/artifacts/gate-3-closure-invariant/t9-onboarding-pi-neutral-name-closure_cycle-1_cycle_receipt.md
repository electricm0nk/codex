# Cycle t9-onboarding-pi-neutral-name-closure — Gate 3 closure invariant / `decisions.md §20`/`§24`

- **Card ID:** `epic-2-cause-closure` (row 11, kanban.md — left `in-progress` per dispatch
  instruction; row 15 also left `in-progress`, untouched by this cycle).
- **Commit SHA:** (see push result, appended after commit)
- **Files touched:**
  - `src/rules_core/codex_neutral_name.rs` — reused unchanged (already ported/proven by a prior
    `class_feature` cycle).
  - `src/rules_core/cache_gen/equipment_gap.rs` — new `RenameInfo`/`resolve_name_or_rename`;
    `CacheRecord` gains `codex_generated_name`/`rename`; the name-PI exclusion branch now renames
    and writes instead of excluding; slug/report/citation logic updated to use the neutral identity.
  - `src/rules_core/cache_gen/hand_authored_equipment.rs` — sibling generator, same fix, reusing
    `equipment_gap::resolve_name_or_rename` (not a duplicate copy).
  - `src/bin/gen_equipment_gap_tables.rs` — `ScreenOutcome` no longer excludes a name-PI row; it
    renames it in-place (`neutral_name`/`neutral_key`) and carries its real citation forward via a
    new `EquipmentGapRow.name_pi_citation: Option<(&'static str, u32)>` field (needed because the
    row's own `key`/`name` can no longer be used to re-locate it in the real `.lst` text).
  - `src/rules_core/rules_tables/equipment_gap_tables.rs` — **regenerated**
    (`cargo run --bin gen_equipment_gap_tables`), 1879 → 1953 rows.
  - `src/bin/ingest_spells.rs` — `pi_screen` no longer drops a name-PI record; it renames it and
    returns `SpellEntry.name_pi_line: Option<u32>` carrying the real citation line.
  - `src/rules_core/cache_gen/spell_lane_dump.rs` — `NormalizedEntry`/`CacheRecord`/`SpellData`
    gain the neutral-rename fields; the citation lookup prefers `name_pi_line` over the
    name-keyed `lines_by_name` map; the name-PI branch renames and writes instead of dropping.
  - `src/rules_core/rules_tables/{21 books}/spell_list.rs` — **regenerated**
    (`cargo run --bin ingest_spells`, all books) to carry the new `name_pi_line` field and the
    renamed entries.
  - `src/rules_core/rules_tables/{bestiary_6,ultimate_intrigue,ultimate_equipment}/spell_list.rs` —
    hand-authored (not `ingest_spells.rs`-managed) siblings `cache_gen::spell_lane_dump` also
    reads; given the additive `name_pi_line: Option<u32>` field (`None` for all — no PI-blocked
    row in these three books) so the shared type shape compiles.
  - `src/rules_core/spell_resolver.rs` — one pre-existing test
    (`inner_sea_gods_drops_the_deity_name_blacklisted_records`) rewritten:
    it asserted 4 real deity-possessive spell names as literal string constants, a live
    `decisions.md §24b`-2 violation ("the PI original appears nowhere... not in a test") that
    predates this cycle. Replaced with a count-and-marker-prefix check
    (`inner_sea_gods_never_ships_a_deity_possessive_name_unrenamed`) that proves the same claim
    (the original name never ships) without transcribing it. Not part of this cycle's assigned
    scope, but directly inside the blast radius of turning "excluded" into "renamed and shipped" —
    fixed under `§17`'s "if your scope is the bottleneck, widen it" and `§24b`-2's binding
    condition, which this repo's own test suite was violating before this cycle touched it.
  - `tests/equipment_gap_tables.rs` — pinned per-book and total row counts re-derived and updated
    (1879 → 1953; 10 books' figures widened).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped to this cycle's own diff against `$PIN`,
  not the full `BASE_BRANCH...HEAD` form — the latter returns pre-existing tagged lines from
  earlier SD-32 cycles between `origin/develop`'s merge-base and `$PIN`, not a per-cycle signal).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope).
- **Acceptance criterion:** `decisions.md §20` — Gate 3's closure condition is `no_record == 0`.
  `decisions.md §24` — PI-name-blocked units close by real ingest under a Codex-generated neutral
  name, not by permanent exclusion. Scope handed to this cycle: the PI-name-blocked units within
  `equipment`/`equipment_modifier` (the residual 87, ~82 previously reported PI-excluded) and
  `spell` (the ~23-unit population named in the wave3 spell receipt), across both kinds.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`;
  re-verified via `scripts/fetch-pcgen-oracle.sh --check` after bootstrapping a fresh worktree's
  empty oracle slot).
- **Status:** complete (mechanism ported to both kinds, real content ingested, real closure proven
  by ledger diff; residuals named explicitly below, none silently narrowed).

## §17a re-derivation before planning

Fresh worktree — this cycle reset to `$PIN` (`41b01d41a3451137a3ac7e27644d9b65861b3dd5`, which is
`origin/tranche/12`'s own current tip) and bootstrapped the empty oracle slot.

```
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger_now0.json
# join-status split (no_record by kind):
[('monster_ability', 100), ('equipment', 87), ('spell', 29), ('ability', 5), ('equipment_modifier', 4), ('companion', 2)]
```

Matches the brief's headline figures exactly (`monster_ability` 100, `equipment` 87, `spell` 29,
`ability` 5, `equipment_modifier` 4, `companion` 2 — sum 227).

**Re-derived the PI-name-blocked sub-populations directly** (not trusted from the brief or prior
receipts):

- `equipment` 87, per book, matches the prior receipt's own trace exactly (`inner_sea_gods` 25,
  `adventurers_guide` 18, `inner_sea_intrigue` 8, `ultimate_magic` 8, `inner_sea_combat` 7,
  `inner_sea_world_guide` 7, `bestiary_4` 3, `mythic_adventures` 3, `advanced_class_guide` 2,
  `ultimate_equipment` 2, `bestiary_2` 1, `bestiary_3` 1, `book_of_the_damned_volume_2` 1,
  `inner_sea_races` 1). Of these, **82 are PI-name-blocked** (the prior receipt's own trace); the
  other 5 are genuine, non-PI gaps (2 `.FORGET` directives, 2 name-shorthand mismatches, 1
  untraced) already named by that receipt and untouched by this cycle.
- `spell` 29, per book: `inner_sea_magic` 5, `inner_sea_world_guide` 5, `adventurers_guide` 4,
  `inner_sea_gods` 4, `inner_sea_intrigue` 4, `bestiary` 1, `bestiary_4` 1,
  `book_of_the_damned_volume_1` 1, `book_of_the_damned_volume_2` 1, `inner_sea_faiths` 1,
  `occult_adventures` 1, `ultimate_combat` 1. Of these, **23 are the wave3 receipt's own
  PI-name-blocked population** (`inner_sea_magic`/`inner_sea_world_guide`/`adventurers_guide`/
  `inner_sea_gods`/`inner_sea_intrigue`/`inner_sea_faiths` = 5+5+4+4+4+1). The other 6
  (`bestiary`/`bestiary_4`/`book_of_the_damned_volume_1`/`book_of_the_damned_volume_2`/
  `occult_adventures`/`ultimate_combat`, 1 each) are genuine non-PI gaps that receipt already
  traced (a `.lst` file mismatch, PFS-legality variant files, cross-book collisions) — **except**
  `bestiary_4`'s 1, which that receipt's own text names as `Summon Monster IX (Cthulhu)`,
  "confirmed genuine `NAMEISPI:YES` drop per `decisions.md §19b`" — under the pre-`§24` regime that
  was correctly excluded; under `§24` it is exactly this cycle's population and closes the same way.

## §24 correction of the prior lane's disposition

The equipment-citation-redirect-instrument-fix receipt reported its residual 82 as **"verified
correct Product-Identity exclusions"** — sound screening work (which this cycle reuses, both the
`NAMEISPI:YES`-declared and undeclared-blacklist-hit findings), but **"excluded" is not a
disposition `decisions.md §24` accepts**. That receipt predates `§24`'s ruling and could not have
known it; this is not an error in that cycle's own work, only in a disposition the operator has
since superseded. Correcting it here, by reference, per the dispatch brief's instruction (not
rewriting that receipt's own text).

## The mechanism: ported, not invented, and unified across two generators

`src/rules_core/codex_neutral_name.rs` (`neutral_name`/`neutral_key`) already exists, proven by a
prior `class_feature` cycle. This cycle reused it directly — no fork, no reimplementation — from
three call sites:

1. **`cache_gen::equipment_gap::resolve_name_or_rename`** (new, `pub(crate)`) — a single function
   factored out of `generate()`'s loop body specifically so the rename decision is unit-testable
   without needing to inject synthetic rows into the static `equipment_gap_tables::
   equipment_gap_rows()` table `generate()` reads. **Reused verbatim by `cache_gen::
   hand_authored_equipment::generate()`** (the sibling generator for `ultimate_psionics`/
   `ultimate_combat`/`ultimate_intrigue`/`ultimate_magic`'s hand-authored tables) — one function,
   two call sites, not a duplicate copy (`decisions.md §17`'s "one mechanism, many units").
2. **`gen_equipment_gap_tables.rs`'s `main()` loop** (the table-generation binary, upstream of
   `cache_gen::equipment_gap`) — calls `neutral_name`/`neutral_key` directly to bake the neutral
   identity into the COMPILED table, because `equipment_gap_tables::equipment_gap_rows()` feeds
   `equipment_resolver::equipment_catalog_rows()` **directly** (the player-facing desktop equipment
   catalog reads this compiled table, not `data/corpus/`) — excluding a row here, not just at the
   corpus-dump stage, was the REAL reason these rows never reached `data/corpus/` at all (traced
   below).
3. **`ingest_spells.rs`'s `pi_screen`** — same shape: `SPELL_LIST` (this binary's own compiled
   output) feeds `spell_resolver::spell_catalog_rows()` directly, so the rename must happen here,
   not only at `cache_gen::spell_lane_dump`'s corpus-dump stage.

## The real defect this cycle found: renaming ONLY at the corpus-dump stage is not enough

Before touching anything, ran the equipment corpus-dump generator (`gen_cache_equipment_gap`)
against the UNMODIFIED `equipment_gap_tables.rs` with only `cache_gen::equipment_gap.rs`'s rename
fix applied. Result: **0 equipment records written**, all 1880 pre-existing rows reported as
`skipped_pre_existing`/`disambiguated_collision`/`excluded_non_content_directive` — **none reached
the new rename branch at all**.

Root cause: `gen_equipment_gap_tables.rs` (a SEPARATE binary from `cache_gen::equipment_gap`)
already excludes a name-PI row **before it is ever compiled into `EquipmentGapRow`** — via its own
`screen_record`/`ScreenOutcome::ExcludedDeclaredName`/`ExcludedBlacklistName` arms. A row excluded
there can never reach `cache_gen::equipment_gap::generate()`'s loop, because that loop only ever
iterates `equipment_gap_tables::equipment_gap_rows()` — the compiled table's own contents. The same
shape exists for spell: `ingest_spells.rs`'s `pi_screen` (a THIRD binary upstream of `cache_gen::
spell_lane_dump`) dropped a name-PI record before it ever entered `SPELL_LIST`.

**Both upstream generators needed the fix too**, not only the corpus-dump generators the prior
spell/equipment receipts' own next-cycle plans named. Fixed in both places; see "Files touched".

### The citation-forwarding problem this created, and its fix

Once a row is renamed **before** it reaches the corpus-dump stage, that stage's own citation
resolution — which searches the real `.lst` text for the record's ORIGINAL `key`/`name` — can no
longer find it (the real text still says the PI name; the row's `key`/`name` now say
`Codex-Named Unit (...)`). Fixed by carrying the real `(source_file, source_line)` forward
explicitly:

- `EquipmentGapRow.name_pi_citation: Option<(&'static str, u32)>` — `cache_gen::equipment_gap::
  generate()` uses this directly instead of `find_citation(&book_dir, entry.key, entry.name)` when
  present.
- `SpellListEntry.name_pi_line: Option<u32>` — `cache_gen::spell_lane_dump::generate()` uses this
  directly instead of `lines_by_name.get(entry.key)` when present.

Both are `None` for every ordinary (non-renamed) row — zero behavior change for anything not
renamed, proven by the pinned-count tests below staying green for every OTHER book/figure.

**`decisions.md §22`'s idempotence property holds by construction**: the corpus-dump stage's own
`declared_pi_at`/blacklist re-screen still fires on a renamed row (it reads the REAL corpus line via
the carried-forward citation), finds `declared.name == true` again, and calls the SAME
`neutral_name`/`neutral_key` with the SAME coordinates — producing the byte-identical neutral
identity a second time. No special-casing needed; the two screens agree by determinism, not by
trust.

## RED → GREEN

**Equipment** (`src/rules_core/cache_gen/equipment_gap.rs`, `#[cfg(test)] mod tests`): 5 new tests
(`resolve_name_or_rename_renames_a_name_pi_row_instead_of_excluding_it`,
`resolve_name_or_rename_passes_a_clean_row_through_unchanged`,
`resolve_name_or_rename_output_is_unchanged_when_the_original_name_is_swapped`,
`write_json_of_a_renamed_record_never_carries_the_original_name`, plus one existing test renamed/
updated). RED proved by reverting the file to its pre-fix committed state (`git show HEAD:<path>`
into a temp path, never `git stash`) with only the new tests appended: 7 compile errors, all
`cannot find function 'resolve_name_or_rename'`/`no field 'codex_generated_name'` — the intended
reason. Restored the fix; GREEN:

```
cargo test --locked --lib rules_core::cache_gen::equipment_gap:: -- 22 passed; 0 failed; 1 ignored
cargo test --locked --lib rules_core::cache_gen::hand_authored_equipment:: -- 6 passed; 0 failed
```

**`gen_equipment_gap_tables.rs`**: `ScreenOutcome`'s two `Excluded*` variants removed (replaced with
`Kept { name_is_pi, .. }`); every call site and 3 tests updated to the new API (one renamed:
`a_declared_name_hit_is_flagged_for_rename_even_with_no_blacklist_term`); 2 new tests
(`a_renamed_row_carries_a_neutral_identity_and_its_real_citation`,
`book_relative_path_strips_everything_before_the_book_slug_directory`). GREEN:

```
cargo test --locked --bin gen_equipment_gap_tables -- 24 passed; 0 failed
```

**Spell** (`ingest_spells.rs`): `PiOutcome` enum removed (`pi_screen` now always returns
`SpellEntry`, carrying `name_pi_line`); 6 tests updated/added (2 renamed to `..._renames_...`, 1
new determinism proof `pi_screen_output_is_unchanged_when_the_original_name_is_swapped`). GREEN:

```
cargo test --locked --bin ingest_spells -- 20 passed; 0 failed
```

**`spell_lane_dump.rs`**: GREEN, 9/9, including the real-oracle
`generation_against_the_real_pinned_corpus_resolves_every_citation` proof (asserts
`unresolved_citations.is_empty()` for every `BookSpec` against the pinned oracle — a renamed
entry's citation resolving correctly would fail this test if the `name_pi_line` bypass were wrong).

```
cargo test --locked --lib rules_core::cache_gen:: -- 150 passed; 0 failed; 11 ignored
```

## Real corpus regeneration — additive-only, guarded path, verified

```
git status --porcelain -- data/corpus | wc -l   # 0, before any run
cargo run --locked --bin gen_equipment_gap_tables   # rewrites equipment_gap_tables.rs (1879 -> 1953 rows)
cargo run --locked --bin gen_cache_equipment_gap    # 74 equipment records written
cargo run --locked --bin gen_cache_hand_authored_equipment  # 8 equipment records written (ultimate_magic)
cargo run --locked --bin ingest_spells              # rewrites 21 books' spell_list.rs (24 renamed)
cargo run --locked --bin gen_cache_spell_lane_dump  # 1306 spell records generated (was 1279)
git status --porcelain -- data/corpus | wc -l   # 82, all "?? " (new files), zero " D"/" M"
```

**Zero deletions, zero modifications** — 82 new `data/corpus/**/{equipment,spell}/*.json` files
(76 equipment/equipment_modifier + wait: 74+8=82 equipment, plus 24 spell = 106 total; the two
`git status --porcelain -- data/corpus | wc -l` counts above are per-run snapshots, re-verified
combined below). Full combined count re-verified:

```
git status --porcelain -- data/corpus | wc -l   # 106 (82 equipment + 24 spell), all "??", 0 "D"/"M"
```

Inspected sample files by hand (both kinds): `codex_generated_name: true`, `data.name`/`data.key`
both start with `Codex-Named Unit (`, `rename.coordinate` is the ONLY divergence trace
(`book:basename:line`, e.g. `adventurers_guide:ag_equip_arms_armor.lst:1`), and the original PI
string appears nowhere in either file (`grep` against the written JSON, confirmed by hand for both
samples — not paginated here per `§15`).

## `no_record`, before/after (`decisions.md §12c` — population + command named)

Command: `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output <path>`,
run fresh before this cycle's changes and again after both regens.

| Kind | Before | After | Delta |
|---|---:|---:|---:|
| `equipment` | 87 | 11 | **-76** |
| `spell` | 29 | 5 | **-24** |
| `monster_ability` | 100 | 100 | 0 (sibling lane's territory, untouched — confirmed) |
| `ability` | 5 | 5 | 0 (untouched) |
| `equipment_modifier` | 4 | 4 | 0 (untouched — none of this cycle's renamed equipment rows were
  `Equipmods`-category; confirmed by category field on every written file) |
| `companion` | 2 | 2 | 0 (untouched) |
| **Bundle-wide `no_record`** | **227** | **127** | **-100** |

Zero-regression check (population diff, not just totals): for every kind, `(before_no_record_ids −
after_no_record_ids)` closed cleanly and `(after − before)` is **empty** — no unit anywhere in the
34,631-unit population moved INTO `no_record` as a side effect of this cycle's regens. Command
re-derivable: dump both ledgers' `id` sets per `join_status == "no_record"` and diff.

## Closure figures — three separate numbers (`decisions.md §16`)

- **Closed (real ingest, new corpus record, `no_record` → `matched`/`no_formula_tokens`):** **100**
  units — **76** `equipment`/`equipment_modifier` and **24** `spell` (see the reconciliation note
  below for why 82 equipment records were WRITTEN but only 76 equipment `no_record` UNITS actually
  closed). All 106 written records are genuinely new corpus content (real oracle bytes, real
  citations, Codex-generated identity), not a reclassification and not an instrument fix.
- **Reclassification:** 0. No unit changed `kind`.
- **Instrument correction:** 0 this cycle (the citation-redirect/alias-directory instrument defects
  were already fixed by prior waves; this cycle's residual is entirely a real-content gap).
- **Reachability (Gate 2):** honest **0** for all 106 new records — not wired into
  `equipment_resolver`'s cost calculators or `spell_resolver`'s casting mechanics this cycle
  (Gate-1 measurability only), same precedent every prior widening cycle in this bundle has set.
  They DO reach `equipment_catalog_rows()`/`spell_catalog_rows()` (the player-facing catalog
  chains), proven by `tests/equipment_gap_tables.rs::every_gap_row_reaches_the_shared_catalog` and
  `spell_resolver`'s own `*_is_chained_into_the_catalog` tests, both green.

**Reconciliation note on the 76 equipment closures vs. 82 written:** 82 equipment records were
written (74 + 8), but only 76 of the 87 pre-cycle `no_record` equipment units actually closed —
`ultimate_magic`'s hand-authored path wrote 8 real, renamed, non-PI records, but 6 of the 8
corresponding `docs/work-inventory.json` UNITS still show `no_record`, because their own citation
(via a PFS-legality-overlay routing gap the equipment-citation-redirect receipt already partially
diagnosed) doesn't match either of `shape_ledger.py`'s two join strategies (strict
`(book, source_file, source_line)`, or the `(book, kind, data.key)` fallback — the latter can't
match either, since the new record's `data.key` is now the neutral identity, not the original).
Traced explicitly below ("What is NOT done").

## PI screening

No PI item name, blacklist term, or original spell/equipment name appears in this receipt, in any
commit message, in any test name, or in any kanban row this cycle wrote. Coordinates only
(`book:source_file:source_line`), per `§15`/`§24b`-4. Grepped this cycle's own full diff against
`scripts/pi_scrub.PI_BLACKLIST_TERMS` before writing this receipt: 2 residual hits ("Iomedae"/
"Desna"), both traced to PRE-EXISTING test literals this cycle was forced to touch (call-signature
changes only, not new occurrences) — verified against `$PIN`'s own copy of both files, both terms
already present there, unchanged by this cycle's edits. Re-grepped after adding the new test in
`spell_resolver.rs`: 0 hits in the FINAL file content (the removed literals only appear as `-`
lines in the diff, never in the committed file).

## Fixture discipline (`decisions.md §3`)

Every new test's RED confirmed before its GREEN (see "RED → GREEN" above — reverted-file compile
errors for equipment/hand_authored_equipment, and the pre-fix `0 equipment written` real-corpus run
for the upstream-generator defect). No emitted-value fixture applies to these records (Shape B
population/completeness records, not `formula_interpreter` magnitudes) — `raw_tokens`/description
fields are transcribed verbatim from the real corpus line via the SAME `declared_pi_at`/
`classify_optional_field_declared` pipeline every other record in these generators already uses,
unchanged by this cycle.

## What is NOT done, named explicitly (no silent narrowing)

- **`equipment`'s 11 residual `no_record`:**
  - **5 non-PI stragglers**, unchanged from the prior receipt's own trace (2 `.FORGET` directives
    in `advanced_class_guide`, 2 name-shorthand mismatches in `bestiary_2`/`bestiary_3`, 1 untraced
    gap in `mythic_adventures`) — not attempted this cycle, out of `§24`'s scope (they are not
    PI-blocked).
  - **6 `ultimate_magic` units**, newly traced this cycle: their real content IS now shipped (as
    one of the 8 renamed `hand_authored_equipment` records each), but `docs/work-inventory.json`'s
    own per-unit citation for these 6 diverges from the citation `hand_authored_equipment::
    find_citation` resolved (a PFS-legality-overlay-routing shape, same class the equipment-
    citation-redirect receipt already named for `ultimate_magic`'s NON-PI residual). A real fix
    needs `docs/work-inventory.json`'s own equipment enumeration (`v06_work_inventory.rs`, out of
    this cycle's file grant — large, shared, no lane in this wave claims it) to either alias these
    6 units' citations to the base row `hand_authored_equipment` actually resolved, or `shape_ledger
    .py`'s join to gain a THIRD fallback strategy. Not attempted this cycle; named for the next
    owner of either file.
- **`spell`'s 5 residual `no_record`:** unchanged from the wave3 receipt's own trace — `bestiary`'s
  `Veil (self only)` (a `source.kind` the ledger's LST-only join structurally cannot match, per that
  receipt's own discovery), the two `book_of_the_damned` PFS/campaign-gated variant files
  (deliberately not this pipeline's target), and `occult_adventures`/`ultimate_combat`'s cross-book-
  collision-vs-ledger-scoping question (a `v06_work_inventory.rs` census question). None are
  PI-blocked; none attempted this cycle, all already named as someone else's open item.
- **`monster_ability`'s 100, `ability`'s 5, `companion`'s 2:** explicitly out of scope (sibling
  lanes' territory) and confirmed unchanged before/after.
- **Reachability (Gate 2):** the 106 new records are measurable (Gate 1) but not wired into any
  cost/casting mechanism this cycle — named above, same precedent as every prior widening cycle.

## Discoveries

- **A generic instrument gap, corrected:** renaming ONLY at the corpus-dump stage (`cache_gen::
  equipment_gap`/`cache_gen::spell_lane_dump`) is insufficient whenever an UPSTREAM compiled table
  (`equipment_gap_tables.rs` via `gen_equipment_gap_tables.rs`; `SPELL_LIST` via `ingest_spells.rs`)
  ALSO screens PI and feeds the player-facing resolver directly. Both upstream screens needed the
  identical fix; this is now a documented, reusable shape (`resolve_name_or_rename`,
  `name_pi_citation`/`name_pi_line` citation-forwarding) for any future generator with the same
  two-stage structure.
- **A real, pre-existing `decisions.md §24b`-2 violation, found and fixed:** `spell_resolver.rs`'s
  own test suite carried 4 real deity-possessive spell names as literal string constants (predates
  this cycle; not introduced by it) — exactly the "not in a test" binding condition `§24b`-2 states.
  Rewritten to prove the same claim (the original name never ships) without transcribing it.
- **`hand_authored_equipment.rs` shares `equipment_gap.rs`'s exact PI-exclusion shape** and was
  fixed identically as part of this cycle's generic pass — `§17`'s "one mechanism, many units," not
  a per-book shim.

## Next-cycle plan

1. `ultimate_magic`'s 6-unit citation-redirect gap (traced above) — needs
   `v06_work_inventory.rs`'s own equipment enumeration or a third `shape_ledger.py` join strategy.
2. `equipment`'s 5 genuine non-PI stragglers (`.FORGET` filter, name-shorthand alias table,
   `mythic_adventures`'s untraced gap) — all previously named, still open.
3. `spell`'s 5 genuine non-PI stragglers (`WebSecondSource`/`LstCorrectedIngest` join-key
   extension, PFS/campaign-gated variant files, cross-book-collision ledger scoping) — all
   previously named, still open.
4. `monster_ability`'s 100 — sibling lane's territory.

## Retro log

Logged via `scripts/verify.sh`'s own auto-append (`preflight-oracle` checks,
`docs/retro/events/t9-onboarding.jsonl`) during environment setup this cycle. This cycle's own
findings are captured in full in this receipt and the kanban entry; no additional `retro.py` event
required.

## Post-rebase re-derivation (`§17a`)

Rebasing onto `origin/tranche/12` landed several concurrent sibling commits, including
`8eaabfbabc` ("shape_ledger.py equipmods-nested kind derivation -- equipment_modifier no_record
1003->19") and `4ed1024309` ("t9 straggler wave -- spell/ability/equipment_modifier no_record
38->28, cross-book instrument fallback") — both INSTRUMENT/content work on `spell`/
`equipment_modifier`, overlapping this cycle's own kinds but not its files (`git diff --stat` of
this cycle's `$PIN` against `origin/tranche/12` before rebasing showed only `equipment_gap.rs`
touched among this cycle's files, non-overlapping: an unrelated `find_citation` disabled-line fix
that auto-merged cleanly). Rebase produced exactly one conflict, `kanban.md` row 11 (expected —
every concurrent T9 cycle appends to the same cell); resolved by hand, preserving BOTH sides'
cycle-id lists and Notes paragraphs (mine prepended after the upstream's own newest entry, not
overwriting it).

Re-ran this cycle's full test scope post-rebase (`rules_core::cache_gen::{equipment_gap,
hand_authored_equipment, spell_lane_dump}`, `rules_core::spell_resolver`, `--bin ingest_spells`,
`--bin gen_equipment_gap_tables`, `--test equipment_gap_tables`) — every suite still green,
identical pass counts to the pre-rebase run above. `git status --porcelain -- data/corpus`
post-rebase: no new deletions or modifications beyond this cycle's own 106 additions.

Re-derived the ledger fresh post-rebase (sibling cycles moved `spell`/`equipment_modifier` further
via their own overlapping work, composing on top of this cycle's own closures):

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger.json
```
→ `monster_ability 98, class_feature 25, equipment_modifier 19, equipment 10, companion 2, spell 2,
ability 1` — bundle `no_record` total **157**. `equipment_modifier`'s new 19 and `class_feature`'s
new 25 are populations two SIBLING commits newly exposed (a shape_ledger.py kind-derivation fix
and unrelated `class_feature` work) — not this cycle's territory, not investigated here.
`equipment`'s 10 (down from this cycle's own post-fix 11) and `spell`'s 2 (down from this cycle's
own post-fix 5) reflect further composition with `4ed1024309`'s own straggler-closure work landing
on top of this cycle's — a real, independently-verified additional improvement this cycle did not
itself perform, named honestly rather than claimed.

## Disk

`df -h /` reported at the end of this cycle (see final message): 968G total, 368G used, 601G
available, 38% used.
