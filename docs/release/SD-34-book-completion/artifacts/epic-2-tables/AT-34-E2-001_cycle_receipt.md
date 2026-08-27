# Cycle 1 — Epic 2 (Build eight of the nine tables) / AT-34-E2-001

- **Commit SHA:** (filled below, after commit)
- **Files touched:** `src/rules_core/rules_tables/simple_kind_tables.rs` (new), `src/rules_core/rules_tables/mod.rs`, `src/bin/v06_work_inventory.rs`, `docs/release/SD-34-book-completion/artifacts/epic-2-tables/AT-34-E2-001_table_transcript.txt` (new), this receipt
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "For each kind, either an engine table exists and holds records, or a proof by execution that the kind needs none — e.g. every unit of that kind is `display`-class and its terminal state is a rendered description. **Evidence:** per kind, either the table's location and a transcript of it holding a named record, or the counts showing no magnitude is involved. 'No table needed' is a finding that must be proven, never assumed to save work."

## What this cycle built

Epic 2's eight kinds all needed a table (`technical-design.md §4`: "Epic 2 builds 8 of 9" — none
of the eight is a "no table needed" case; `power`, the ninth, is the one left for Epic 5, out of
this criterion's population). One of the eight — `companion` — already has a real table, built in
SD-29 (`rules_core::rules_tables::companion_chassis`, `COMPANION_BOOKS` registry). This cycle
builds the other **seven**: `ability`, `template`, `trait`, `deity`, `domain`, `skill`, `language`.

New module `src/rules_core/rules_tables/simple_kind_tables.rs`: `load_simple_kind_table(repo_root,
kind)` loads every corpus record for one kind, across every book, from the live
`data/corpus/<book>/<dir>/*.json` tree, keyed by `(book, corpus key)`. `resolve(book, key)` returns
the real record for a present key, or `None` for an absent one — never a fabricated or defaulted
entry (`AT-34-E2-002`'s fail-closed bar, proven here for the RED half too — see below).

**The one directory-name hazard, caught before shipping (`workflow-instruction.md §4`'s "a shallow
glob lies" warning):** `trait`'s 487 corpus units all live under `data/corpus/*/trait_generic/*`,
not `data/corpus/*/trait/*`. A naive `kind == dir name` assumption returns zero records for a kind
with 487 real ones. `kind_dir_for("trait")` resolves this explicitly; a unit test
(`trait_kind_resolves_to_the_trait_generic_directory`) pins it so a future edit that reverts the
mapping fails loudly.

Wired into `src/bin/v06_work_inventory.rs` via a new `--epic2-table-transcript` flag (same
early-return, read-only, "moves no unit on any board" contract as the existing `--spell-probe` /
`--class-probe` flags) — this is what produced the committed transcript, not a hand-written
example.

## RED → GREEN (TDD, `workflow-instruction.md §6` step 3)

**RED, confirmed for the intended reason:** with `SEVEN_KIND_DIRS`'s `trait` entry temporarily
mutated from `("trait", "trait_generic")` to `("trait", "trait")`,
`trait_table_holds_trait_adopted` failed with `trait table loaded zero records from "trait"` — the
directory-mismatch bug, not an unrelated panic. Reverted, then GREEN:

```
$ cargo test --locked --lib rules_core::rules_tables::simple_kind_tables
running 11 tests
test rules_core::rules_tables::simple_kind_tables::tests::an_unknown_kind_yields_an_empty_table_not_a_panic ... ok
test rules_core::rules_tables::simple_kind_tables::tests::trait_kind_resolves_to_the_trait_generic_directory ... ok
test rules_core::rules_tables::simple_kind_tables::tests::language_table_holds_xenophobic ... ok
test rules_core::rules_tables::simple_kind_tables::tests::skill_table_holds_craft_rope ... ok
test rules_core::rules_tables::simple_kind_tables::tests::domain_table_holds_battle_spirit ... ok
test rules_core::rules_tables::simple_kind_tables::tests::deity_table_holds_a_pi_masked_codex_named_record ... ok
test rules_core::rules_tables::simple_kind_tables::tests::trait_table_holds_trait_adopted ... ok
test rules_core::rules_tables::simple_kind_tables::tests::template_table_holds_arcanist_spellbook ... ok
test rules_core::rules_tables::simple_kind_tables::tests::an_absent_key_is_refused_not_fabricated ... ok
test rules_core::rules_tables::simple_kind_tables::tests::ability_table_holds_aberrant_bloodline ... ok
test rules_core::rules_tables::simple_kind_tables::tests::every_seven_kind_table_is_non_empty_at_head ... ok
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 2859 filtered out
```

Each of the seven kind-specific tests both resolves a real, named `(book, key)` record from that
kind's corpus directory **and**, in the same test, proves a fabricated key on the same table
refuses (`AT-34-E2-002`'s two halves, per kind, not just once globally).

## Figures + their re-derive commands (every one, with its denominator)

| Figure | Command | Denominator |
|---|---|---|
| `ability` table: 4,824 corpus records | `/tmp/cargo-sd34-at-34-e2-001/debug/v06_work_inventory --epic2-table-transcript \| grep kind=ability` | of `data/corpus/*/ability/*.json` (all books; **not** the same population as the `ability` kind's 4,337-unit bucket-A count in `docs/work-inventory.json` — this table's population is every corpus record under the directory, not the atlas's unit population; the two numbers answer different questions and reconciling them is Epic 3/4's job, not this criterion's) |
| `template` table: 2,248 records | same command, `grep kind=template` | of `data/corpus/*/template/*.json` |
| `trait` table: 487 records | same command, `grep kind=trait` | of `data/corpus/*/trait_generic/*.json` — matches `docs/work-inventory.json`'s `trait` kind population (487) exactly |
| `deity` table: 459 records | same command, `grep kind=deity` | of `data/corpus/*/deity/*.json` — matches the atlas's `deity` population (459) exactly |
| `domain` table: 183 records | same command, `grep kind=domain` | of `data/corpus/*/domain/*.json` — matches (183) exactly |
| `skill` table: 149 records | same command, `grep kind=skill` | of `data/corpus/*/skill/*.json` — matches (149) exactly |
| `language` table: 136 records | same command, `grep kind=language` | of `data/corpus/*/language/*.json` — matches (136) exactly |
| `companion` table (pre-existing, SD-29): 4 companions in the one book sampled (`inner_sea_combat`) | same command, `grep kind=companion` | of `COMPANION_BOOKS[inner_sea_combat].companions`; the full registry spans 9 books (`companion_chassis.rs COMPANION_BOOKS`) |
| 8 of 8 kinds hold a named record | `grep "HELD" docs/release/SD-34-book-completion/artifacts/epic-2-tables/AT-34-E2-001_table_transcript.txt \| grep -oE "kind=[a-z]+" \| sort -u \| wc -l` | of the 8 kinds Epic 2 builds (`technical-design.md §4`) |

## Row-count command output

```
$ grep "HELD" docs/release/SD-34-book-completion/artifacts/epic-2-tables/AT-34-E2-001_table_transcript.txt | grep -oE "kind=[a-z]+" | sort -u | wc -l
8
```
8 of 8 kinds hold a named record in the committed transcript. All 8 report `HELD`, zero report
`REFUSED` on their sample key, and every one of the 8 also demonstrates a `REFUSED` result on a
key no corpus record carries (the fail-closed half, printed alongside).

## Build scope verified

`cargo test --locked --no-run` exits 0 at the widest workspace scope, run at
`660129880db0d366ecb4c8622ae563089a4f1f6f` (the commit this cycle rebased onto, before its own
commit). `apps/desktop/src-tauri` not touched this cycle, not run.

## Sweep population

N/A — this cycle wrote no corpus records (`data/corpus/**` untouched;
`git status --porcelain data/corpus` empty). `corpus_literal_sweep`'s examined population is
unmoved.

## Oracle pin

N/A — no figure in this cycle came from the pinned PCGen oracle corpus; all figures are read
directly from this repo's own `data/corpus/`.

- **Status:** complete
- **Movement, four buckets:** **closure** — none this cycle (no unit reclassified; that is
  `AT-34-E2-004`'s job, a separate criterion). **reclassification** — none. **reachability** — none
  (companion's pre-existing reachability rules are exercised, not extended). **instrument-correction**
  — none.

## Notes (judgment calls)

- `companion` was **not** rebuilt: SD-29 already built a real, fail-closed table for it
  (`companion_chassis::COMPANION_BOOKS`). Rebuilding it would duplicate a working instrument for no
  reason. Its transcript line here demonstrates the existing table, not a new one.
- The seven new tables are **runtime** loaders (read `data/corpus/` at call time), not
  compile-time-baked literal Rust tables like `companion_chassis`'s. Both are legitimate patterns
  already in this codebase (`race_resolver::load_race_corpus` is the runtime precedent); a
  compile-time bake for all 4,337+2,248+... records was judged out of proportion to this
  criterion's evidence bar, which asks only that the table exist and hold a named record.
- This cycle deliberately does **not** attempt `AT-34-E2-004` (bucket A to zero for both vehicle
  books) — a separate criterion needing reachability/reclassification wiring these tables don't
  provide by themselves.

## Next-cycle plan

`AT-34-E2-002` (fail-closed proof, formalized per-table — this cycle's tests already demonstrate
it inline but the criterion is its own row) and `AT-34-E2-003` (measured build rate per table,
`table-build-rate.json`) are the natural next cycles in this epic.
