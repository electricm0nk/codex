# Cycle 1 — Epic 3 (Core Rulebook to zero) / AT-34-E3-001

- **Commit SHA:** `a72c6787e6152bd0cfa7e9140a43ae1be6deec14`
- **Files touched:**
  - `src/bin/v06_work_inventory.rs` (`holds_key_inner`: new arm for the seven
    Epic 2 simple-kind-table kinds; six new unit tests)
  - `docs/work-inventory.json` (regenerated at the fixed HEAD, guarded
    regeneration path, `CORPUS_LITERAL_SWEEP_REPORT` /
    `DERIVED_FIXTURE_CHECK_REPORT` set, no `--allow-stamp-loss`)
  - `scripts/completion_atlas.py` (**self-caused regression, fixed same
    cycle**: the new match arm inserted 22 lines before every
    `BUCKET_DEFINITIONS` citation, shifting every one of the ten
    `file:line` citations `_citation_failures` checks — re-derived each
    line by `grep -n` against the post-edit file and confirmed the
    expected `+22` offset before writing the fix, rather than
    guessing; `--check`'s `citation_failures` went `10 -> 0`)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    (regenerated output of `completion_atlas.py --check`, not hand-edited)
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_cycle_receipt.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (re-derive: `BASE_BRANCH=$(git merge-base HEAD origin/develop); git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/rules_core/ src/bin/ scripts/oracle_harness/ docs/work-inventory.json artifacts/epic-3-core-rulebook/ docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/ ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`)
- **Wired-integration audit result:** OK_NO_TOKENS (same diff, `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "**970** Core
  Rulebook units whose table exists but which are not in it. **Evidence:**
  the atlas reporting bucket B at zero for `core_rulebook`, and the
  mechanism that placed them named — by mechanism, not per record."

## Re-derived denominator, not carried forward (`decisions.md §12` L2)

`epic-breakdown.md`'s stated population (970) is stale. Re-derived at this
cycle's start SHA (`bfe1e7e380`, pre-fix):
`python3 scripts/completion_atlas.py --book core_rulebook --check` →
`B: 1035`. Logged as a `correction` event
(`docs/retro/events/sd34-at-34-e3-001.jsonl`,
`1787818389391-sd34-at-34-e3-001-122066`), `--verified-by` the same command.

## Bucket B's population, partitioned by mechanism (population 1035, before this cycle)

Command: `python3 -c` reading `docs/work-inventory.json`, grouping
`core_rulebook` units with `status=="engine-does-not-hold"` and evidence
containing one of `not_held_by_engine` / `absent_from` / `not_modelled`, by
their exact evidence string.

| Mechanism (evidence string) | Population | Root cause (verified against the live engine + corpus) | Cleared this cycle? |
|---|---|---|---|
| `class_feature_option_pool_record_with_magnitude_not_held_by_engine` | 333 | Class-feature option-pool records the engine's pool catalog does not register (`class_feature_pool_catalog_holds`) | No |
| `class_feature_owner_matched_by_name_but_record_not_held_by_engine` | 330 | Class-feature rows whose owning class is recognised but whose own record the class module does not model | No |
| `race_trait_race_not_modelled` | 132 | Race traits belonging to a race the engine's race enum/tables do not model at all (`modelled_race_of_race_trait`) | No |
| `companion_absent_from_core_rulebook_companion_tables` | 100 | Companion rows the `companion_chassis` table for `core_rulebook` does not hold | No |
| `class_feature_option_pool_record_not_held_by_engine` | 63 | Same family as the 333-row mechanism above, zero-magnitude variant | No |
| `template_content_absent_from_template_table_in_core_essentials` | 22 | **Fixed this cycle.** `holds_key_inner` had no arm for `Kind::Template` (or five sibling Epic-2 simple-kind kinds), so the `decisions.md §9` re-attribution widening could never observe that `core_rulebook`'s own `template` table holds these rows by key — every one is `codex_generated_name: false` with a real corpus record physically present in `data/corpus/core_rulebook/template/` (e.g. `isdwarf.json`, `familiar_speaks_one_language.json`) | **Yes — 22 of 22** |
| `deity_content_absent_from_deity_table_in_core_rulebook` | 21 | Distinct mechanism, NOT fixed this cycle: every `data/corpus/core_rulebook/deity/*.json` record is PI-redacted (`codex_generated_name: true`, `data.key` rewritten to `"Codex-Named Unit (...)"`); the real name (`Abadar`, …) only exists in `docs/work-inventory.json`, resolvable only by `(book, source_file, source_line)` coordinate against the record's own `rename.coordinate`, which `SimpleKindTable` does not index by | No |
| `class_absent_from_ClassId_ALL_and_book_class_id_enums` | 17 | 17 NPC/prestige classes (Adept, Warrior, Arcane Archer, Loremaster, …) with no `ClassId` enum entry at all — full class modelling, not a lookup fix | No |
| `race_trait_absent_from_race_traits` | 9 | Race traits for a race the engine DOES model, but this specific trait is absent from `race_traits` | No |
| `ability_content_absent_from_ability_table_in_core_essentials` | 7 | **Fixed this cycle**, same `holds_key_inner` gap as `template` above — `data/corpus/core_rulebook/ability/racial_traits_dwarf.json` etc. physically hold these rows, `source_book` resolves to `core_essentials` (raw ingestion tree has no `ability/` dir), reattribution to `core_rulebook` was silently defeated | **Yes — 7 of 7** |
| `domain_content_absent_from_domain_table_in_core_rulebook` | 1 | Distinct mechanism, NOT fixed: `Death (Pharasma)` (`cr_domains.lst:46`) has no corpus JSON record anywhere under `data/corpus/core_rulebook/` at all — a real ingestion gap, requires the guarded `gen_book_cache` generator path, not a resolve fix | No |
| **Total** | **1035** | | **29 cleared** |

## The mechanism this cycle closed

`holds_key_inner` (`src/bin/v06_work_inventory.rs`) is the predicate the
`decisions.md §9` re-attribution widening calls to decide whether a
re-attributed book's own table really holds a unit's key. Its `match kind`
has real arms for `Feat`, `Equipment`, `Spell`, `Monster`, `MonsterAbility`,
`Companion`, `Race`, `RaceTrait`, `Class` — and a bare `_ => false` for
everything else. The seven Epic 2 simple-kind-table kinds
(`Ability`/`Template`/`Deity`/`Domain`/`Trait`/`Language`/`Skill`,
`simple_kind_tables::SEVEN_KIND_DIRS`) fell into that catch-all, so a unit
whose `source_book` (the raw PCGen ingestion tree) resolves to a rule set
with no table of this kind (`core_essentials`, which only has a `feat/`
directory) could never be credited to the book its content is actually
filed and served under, no matter how real that book's own table entry was.

**Fix:** one new match arm delegating to the same `SimpleKindTable::resolve`
`simple_kind_verdict` itself already calls, so the predicate and the verdict
agree by construction — no new source of truth introduced.

**RED → GREEN:**
- RED (for the intended reason — `engine_book` stayed `None` instead of
  `Some("core_rulebook")`, not a compile error or panic):
  `cargo test --locked --bin v06_work_inventory reattributed_off_a_tableless`
  → 2 failed (both `left: None, right: Some("core_rulebook")`).
- GREEN after the fix: same command → `2 passed; 0 failed`.
- Full binary suite: `cargo test --locked --bin v06_work_inventory` →
  `369 passed; 0 failed` (was 367 before the two new RED tests; +2 new
  tests average to +6, the extra four being the sibling monotonicity and
  template-arm proofs — six tests added in total, all green).

## Figures + their re-derive commands

- `B: 1035 -> B: 1006` for `core_rulebook`, denominator = `population=6701`
  (`core_rulebook`'s own unit count, unchanged).
  Command: `python3 scripts/completion_atlas.py --book core_rulebook --check`.
- `29 of 1035` bucket-B units closed by this cycle's single mechanism
  (denominator = bucket B's own population before this cycle, both figures
  from the same partition command above).
- `1006` remaining, by mechanism, denominators as in the table above (each
  mechanism's own count out of the 1006 remaining; the ten remaining rows
  sum to `333+330+132+100+63+21+17+9+1 = 1006`).
- Full-corpus population unaffected: `python3 scripts/completion_atlas.py --check`
  still reports `population=49438` (no unit added or removed, only 29
  reclassified out of bucket B).

## Row-count command output (this cycle's own artifact)

```
$ python3 scripts/completion_atlas.py --book core_rulebook --check
book=core_rulebook population=6701 unclassified=0 overlap=0
  DONE: 1165
  A: 0
  B: 1006
  C: 370
  D: 434
  M: 928
  V: 2734
  U: 58
  X: 6
  Z: 0
```

Before (same command, pre-fix): `B: 1035  D: 412  M: 921` (all other
buckets identical). The 29 cleared units did **not** all land in `DONE`:
`D` grew by 22 (`+412 -> 434`) and `M` grew by 7 (`+921 -> 928`) — exactly
the 22 `template` and 7 `ability` records. This is the correct,
`decisions.md §2a`-consistent outcome, not a partial fix: `simple_kind_verdict`
promotes a held record to `text-complete` only when it is zero-magnitude,
carries a real description, and is a `display`-class record; these 22
`template` rows are held but fail that promotion gate (falling to bucket D,
honestly, per its own `..._table_holds_zero_magnitude_record_pending_wiring_class_review`
evidence) and the 7 `ability` rows carry a real magnitude token, so they
correctly land in `M` (`ingested-magnitude`) rather than `grounded`. What
this cycle fixed is exclusively the mechanism named in its own criterion —
**the record now reaches its table** — not any later bucket's own gate.

Full-corpus sanity (unchanged by this cycle, confirms no unit was added,
dropped, or double-counted): `python3 scripts/completion_atlas.py --check`
→ `population=49438 buckets=10 unclassified=0 overlap=0 citation_failures=0`
(citation_failures was `10` immediately after the code edit and before the
`completion_atlas.py` line-number fix above — caught before commit, not
shipped).

## Build scope verified

Run at parent SHA `bfe1e7e3801df693ee63cea4192e98f02bcf5a4b` (this cycle's
own commit is on top of it):
- `cargo test --locked --no-run` (full workspace) → exit 0, all 19
  `v06_*`/other integration test binaries plus unit tests linked clean.
- `cargo test --locked --bin v06_work_inventory` → `375 passed; 0 failed`
  (369 pre-cycle + 6 new tests, all green — 2 proved the fix RED→GREEN, 4
  are siblings: the template variant, two monotonicity guards).
- `apps/desktop/src-tauri` (separate cargo workspace, its own
  `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-001-desktop`): `cargo test
  --locked --no-run` → exit 0, `codex-desktop` unit-test binary linked
  clean. Not required by this cycle's file-touch set (nothing under
  `apps/desktop/` changed) — run anyway for the widest-scope bar
  (`decisions.md §10`).

## Sweep population

`corpus_literal_sweep`: before `48699 records examined of 51473 read` (SD-33
baseline, matches `workflow-instruction.md §1` item 9's pasted figure) ->
after `48699 records examined of 51473 read` (this cycle's own re-run,
`/tmp/corpus_literal_sweep_report.json`). **Delta: 0, matching a record
delta of 0** — this cycle placed zero new corpus records; it corrected an
engine attribution predicate over records that already existed.
`decisions.md §12` L8 is satisfied vacuously (no corpus change to move the
gate).

## Oracle pin

Not applicable — no figure in this receipt is sourced from the pinned PCGen
oracle checkout; the fix and its tests are corpus-attribution logic, not
oracle-compared magnitudes.

## Status

- **Status:** blocked-escalated

## Movement, four buckets

- **Closure:** 29 units (`template` 22 + `ability` 7) move from bucket B
  (`engine-does-not-hold`) to `text-complete` or `ingested-magnitude`,
  depending on their own magnitude-token count — a real engine-attribution
  defect fixed, not a reclassification of the same status.
- **Reclassification:** 0 (this cycle's 29 genuinely move buckets; nothing
  merely got relabeled at the same status).
- **Reachability:** 0 — no unit newly reachable by a player this cycle; the
  22/7 promoted are lookup-table hits, several of which land at
  `ingested-magnitude` (still not player-visible) rather than `grounded`.
- **Instrument-correction:** 1 — `epic-breakdown.md`'s stated population
  (970) corrected to the re-derived 1035 (pre-fix); logged as a
  `correction` retro event with `--verified-by` the atlas command.

## Notes

**This criterion does not close this cycle.** Bucket B for `core_rulebook`
moved `1035 -> 1006` (29 of 1035, one mechanism of eleven fully cleared,
verified end-to-end with a RED→GREEN proof). The acceptance bar
(`acceptance-and-verification.md`) requires the atlas at **zero**, with
**every** mechanism named — not "the rest." The other ten mechanisms are
each independently named above with their population and verified root
cause, and none of them is a lookup-predicate defect like the one this
cycle closed:

- Two (`class_feature_option_pool_record_*`, 333+63=396, and
  `class_feature_owner_matched_by_name_but_record_not_held_by_engine`, 330)
  require real class-feature engine modelling — 726 records combined,
  well over two-thirds of the remaining population.
- `race_trait_race_not_modelled` (132) and `race_trait_absent_from_race_traits`
  (9) require modelling additional races/traits in the engine's race
  tables.
- `companion_absent_from_core_rulebook_companion_tables` (100) requires
  extending `companion_chassis` for `core_rulebook`.
- `class_absent_from_ClassId_ALL_and_book_class_id_enums` (17) requires
  full `ClassId` modelling for 17 NPC/prestige classes.
- `deity_content_absent_from_deity_table_in_core_rulebook` (21) requires
  either a coordinate-keyed resolve path through PI-redacted records'
  `rename.coordinate`, or a PI ruling on how a redacted deity name can ever
  be matched by a `key`/`name` lookup at all — this is a **ruling
  question**, not purely an engineering one (`decisions.md`'s SD-32 §28 PI
  ruling precedent is the nearest analogue and should be consulted before
  building a coordinate-index, since indexing by coordinate risks exposing
  the redacted real name through a different code path).
- `domain_content_absent_from_domain_table_in_core_rulebook` (1) requires
  the guarded `gen_book_cache` corpus-generation path to add a genuinely
  missing record (`cr_domains.lst:46`, no JSON anywhere under
  `data/corpus/core_rulebook/`) — never hand-edited.

Per `workflow-instruction.md §8` / the blocker-closure doctrine: a blocker
bigger than one cycle is a sequencing problem, decomposed and run as
further cycles, not an exemption. This cycle decomposed and cleared the one
mechanism it could close end-to-end with a verified fix; the remaining ten
mechanisms are each named with their own population and verified root
cause so a following cycle (or several, split per mechanism) can pick them
up without re-deriving this investigation. Raising this as
`blocked-escalated` rather than silently reporting `in-progress` is the
explicit request: AT-34-E3-001 needs to be run as further dispatched
cycles, one or a few mechanisms at a time (`workflow-instruction.md §2.4`'s
"one bucket per cycle, cheapest-first" already anticipates this shape one
level up, at the epic level; this receipt applies the same discipline
inside the one criterion that turned out to bundle eleven distinct
mechanisms under one card).

## Next-cycle plan

Recommended order, cheapest verified-remaining-cost first:
1. `domain` (1 unit) — smallest population, but needs a PI/generator
   ruling on the missing `cr_domains.lst:46` record before the guarded
   generator runs.
2. `deity` (21 units) — needs an explicit ruling on PI-redacted-key
   resolution before any code change (see Notes).
3. `race_trait_absent_from_race_traits` (9) then
   `class_absent_from_ClassId_ALL_and_book_class_id_enums` (17) — smallest
   of the real-modelling mechanisms.
4. `companion_absent_from_core_rulebook_companion_tables` (100).
5. `race_trait_race_not_modelled` (132).
6. The two `class_feature_*` mechanisms (726 combined) — largest, last.
