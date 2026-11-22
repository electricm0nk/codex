# Cycle t9-monster-ability-provisional-facet-default-round8 — Gate 3 (closure invariant) / Card 11 (T9)

- **Card ID:** 11 (`epic-2-cause-closure`, T9 sub-lane
  `t9-monster-ability-provisional-facet-default-round8`)
- **Files touched:**
  - `scripts/transcribe_monster_tables.py` — new `PROVISIONAL_FACET_DEFAULT`,
    `provisional_facet_reason(row)` (classifies WHY a row's `TYPE:` segments
    name no modeled facet into one of five named shapes) and
    `parse_type_or_provisional_default(row)` (the `§27`-widened `parse_type`:
    a genuinely-resolved row is untouched, a facet-gap row ships with
    `facet=SpecialQuality` and the classified reason). `ability_pi_reason`'s
    pre-pass and the emission loop both call the new function; the old
    `unmodelled_facet` drop set and its emission-time filter block are
    removed (rows are no longer dropped for this reason). `transcribe()`
    gains an optional `provisional_facets: dict[str,str] | None` out-param,
    filled in place, defaulting to a throwaway dict so every existing
    `transcribe(book) -> str` call site is unchanged. New
    `corpus_output_dir(book)` (reuses `CROSS_TABLE_MONSTER_RECORDS` rather
    than a second hand-written book-dir map) and
    `provisional_facet_units(book)` (read-only re-introspection, used by the
    stamping script and tests). Header-comment block added, naming every
    defaulted key and its reason.
  - `scripts/tests/test_transcribe_monster_tables.py` — new
    `ProvisionalFacetDefaultRound8` (9 tests: one per named shape using the
    real coordinate that shape came from, a control proving a genuinely-
    modelled row is unaffected, and a mutation-style proof that the OLD
    `parse_type` alone still raises on every synthetic row) and
    `ProvisionalFacetDefaultShipsInsteadOfDropping` (2 tests: the optional
    out-param's default-to-fresh-dict behavior, and a no-crash/no-omission
    check against the `bonus_bestiary` fixture).
  - `scripts/stamp_monster_ability_provisional_facets.py` (new) — the
    post-`gen_book_cache` step that stamps `decisions.md §27`'s marker on
    the shipped JSON records. Required because this pipeline is Rust-
    generated JSON, not a Python ingest path that could call
    `stamp_provisional_default` directly at write time: `gen_book_cache.rs`
    writes the JSON, this script then loads exactly the records
    `provisional_facet_units(book)` names (matched by `data.corpus_key`,
    never by filename-guessing) and calls
    `shape_provisional_marker.stamp_provisional_default` on each — the one
    sanctioned call path, `workflow-instruction.md §6a`.
  - `src/bin/gen_book_cache.rs` — `verified_citation_line`'s exact-match
    assertion widened with a second, structural bypass:
    `first_col.contains(".COPY=")`. A `.COPY=` overlay ability row's own
    first column is a compound directive
    (`CATEGORY=Special Ability|Rake.COPY=Rake`), never the emitted display
    name, so the pre-existing assertion panicked on every real `.COPY=`
    ability row (`Aurumvorax ~ Rake`, `Carnivorous Blob ~ Split`) before
    this fix. A `.COPY=` MONSTER row never reaches this function (dropped
    before emission, pre-existing screen), so the bypass only ever fires
    for an ABILITY row. Two new unit tests:
    `verified_citation_line_exempts_a_copy_overlay_rows_compound_first_column`
    and `verified_citation_line_still_catches_a_genuinely_stale_citation`
    (mutation-style negative control — a genuinely stale, non-`.COPY=`
    citation still panics).
  - `src/rules_core/rules_tables/{bestiary,bestiary_2,bestiary_3,
    inner_sea_bestiary,inner_sea_gods}/monster_data.rs` — regenerated via
    `transcribe_monster_tables.py <book>`; each book's ability table gains
    exactly its provisionally-defaulted rows, plus header-comment updates.
    No pre-existing record's fields changed (verified: `git diff --stat`
    shows only new `MonsterAbilityRecord` blocks and header-comment lines,
    zero deletions in any of the 5 files).
  - `src/rules_core/rules_tables/{bestiary,bestiary_2,bestiary_3,
    inner_sea_bestiary,inner_sea_gods}/mod.rs` — owned/owner-less/total
    count pins re-derived from live failing runs (never guessed), each with
    a dated comment naming the delta and its cause. `bestiary_3`'s
    owner-less-key-set digest re-derived the same way (0xf294_251a_43b5_b6ae
    -> 0x01b4_2774_3381_b829).
  - `src/rules_core/rules_tables/monster_chassis.rs` — the corpus-wide
    no-reclassification pin
    (`widening_the_facet_vocabulary_does_not_reclassify_any_existing_record`)
    re-derived from a live failing run: 3726 -> 3749 records, digest
    `0xc7f5_5369_ed18_7098` -> `0xfc51_2110_6900_558e`.
  - `apps/desktop/src-tauri/src/monster_catalog.rs` — the corpus-wide
    owner-less-records pin re-derived: 1066 -> 1076 (+10, all in
    `bestiary_3`; the other 13 defaulted rows are owned).
  - `apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs` — one line,
    `beastiary1_monster_count_matches_the_documented_real_total`'s pin
    (710 -> 711).
  - `apps/desktop/src-tauri/src/reach_gate.rs` — `bestiary_1`/`bestiary_2`/
    `bestiary_3`'s own book-level tests' owned/owner-less/total pins
    re-derived; `bestiary_3`'s `UNREACHED_RECORD_FINDINGS` entry gains its
    10 new owner-less keys (inserted alphabetically, matching the existing
    convention) plus its digest reference and Gap-count prose updated.
  - `data/corpus/{beastiary,bestiary_2,bestiary_3,inner_sea_bestiary,
    inner_sea_gods}/monster_ability/*.json` (23 new files, additive only —
    22 the real `no_record` population, plus `bestiary_2`'s
    `Bunyip ~ Blood Rage`, a bonus unit already counted `text-complete` by
    inventory evidence alone) and each book's `LICENSE.json`
    (screening-note append, same generator-owned mechanism every prior
    round used).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped to this cycle's
  own diff, `git diff HEAD`, not the full `BASE_BRANCH...HEAD` form).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scoped diff).
- **PI grep result:** `pi_scrub.normalized_term_hits` run over every added
  line of this cycle's own diff and every byte of all 23 new corpus
  records: zero hits both times.
- **Acceptance criterion:** `decisions.md §27`/`§27a`/`§27b` — a
  delivery-only (and, per `§27a`/`§27b`'s widened scope, any other
  not-genuinely-derived) `TYPE:` row defaults to `SpecialQuality`,
  provisionally, stamped via the one sanctioned marker function; this
  brief's own "round 7 grouped them; two groups are now unblocked" framing,
  the `TYPE:`-facet-gap half.
- **Corpus SHA:** `PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`; bootstrapped from empty this cycle via
  `scripts/fetch-pcgen-oracle.sh`).
- **Status:** complete (this cycle's own scope; card 11 stays `in-progress`
  — see "What remains" below).
- **Notes:**
  - **Near-miss, caught and reverted before commit.** `cargo run --bin
    enrich_monster_ability_raw_tokens` is documented as mandatory (a
    `monster_ability` record with no `raw_tokens` can never reach
    `literal-verified`), so it was run once. It is book-agnostic by design
    and enriched **1,829** records corpus-wide — 1,806 of them pre-existing
    records in books/units this cycle never touched. `git status
    --porcelain` caught this before commit; the 1,806 out-of-territory
    files were reverted via `git checkout --` (never `git stash`), leaving
    only this cycle's own 23 new records enriched. Logged as the kind of
    incident `AGENTS.md`'s "warning is not a control" rule names — the tool
    itself is correct and needed, but running it wholesale inside a
    narrowly-scoped cycle is not; a future cycle that needs it should scope
    it to the specific books/files it just touched, not run it bare.
  - **`decisions.md §22` applied to a design tension, not a data defect.**
    The `.COPY=`-row citation-verification fix (`gen_book_cache.rs`) is a
    generator-code fix, not a corpus-data resolution — filed here because
    it is the same shape `§22` names in spirit (Codex resolving an
    inconsistency the pipeline itself produces, with the divergence visible
    in code and tests) even though `§22`'s letter is about corpus data.
  - **The `.COPY=`-base-unresolved classification is verified, not
    assumed.** Before defaulting `Aurumvorax ~ Rake`/`Carnivorous Blob ~
    Split`/`Bunyip ~ Blood Rage`, a corpus-wide search for a bare-named
    (non-namespaced) `Rake`/`Split`/`Blood Rage` ability row across every
    book's `*abilities*.lst` file found none — every real ability sharing
    those bare names in this corpus is itself namespaced
    (`Bandersnatch ~ Rake`, `Amphisbaena ~ Split`, …), confirming the
    `.COPY=` target these three rows cite does not exist anywhere this
    script reads, not merely that this script does not resolve it.
  - **The Internal-only novel shape (`Morlock ~ Sneak Attack`) decided with
    evidence, per `§27b`'s "novelty is grounds for sizing, not exclusion".**
    Its `TYPE:Internal` segment is neither a facet nor a delivery in this
    chassis's vocabulary; its `VISIBLE:NO` marks it a hidden bonus-granter
    (`BONUS:VAR|SneakAttackDice|1`), not a player-facing ability card. Given
    a real facet cannot be derived without inventing one, and `§27`/`§27a`
    explicitly widen the provisional-default mechanism to cover exactly
    this kind of case, it is defaulted like the others, with its own
    distinguishing reason (`type_internal_only_no_facet_no_delivery`) so
    row 17's future pass finds it labeled, not lost inside a generic bucket.
  - **`occult_adventures`'s 5 units and `advanced_race_guide`'s 2 companion
    units are not part of this cycle's remaining-56 count** — re-confirmed
    live (`0` `monster_ability` `no_record` rows under `occult_adventures`
    in the post-cycle ledger) that an earlier commit in this history
    (`916228e9a7`, `§27b` EVERYTHING) already closed them; this cycle did
    not need to re-touch them, and the brief's own note that "occult_
    adventures is no longer an exception" was correct but already acted on.
- **Discovery forwards:** none new.
- **Next-cycle plan:** see §9 below.

---

## 1. Re-derived the population before touching anything (`§17a`)

Never trusted the brief's own 78/56/22 figures without re-deriving.
`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json`
confirmed `monster_ability` `no_record` = **78** (bundle-wide `no_record`
**106**, matching `equipment_modifier` 19 / `equipment` 8 / `ability` 1 /
`monster_ability` 78 exactly). Ran `transcribe()` in-process (not via the
CLI, to capture stderr without writing files) for the 9 non-`occult_
adventures` in-scope books and cross-referenced the printed refusal lists
against the ledger's own `no_record` set: 56 multi-`DESC:` parse refusals, 22
`TYPE:`-facet-gap rows (2 extra keys the transcriber refuses that the ledger
does NOT count as `no_record` — `Lycanthrope ~ Change Shape` and `Bunyip ~
Blood Rage` — investigated separately below). `occult_adventures` returned
zero `no_record` `monster_ability` rows, confirming the brief's "no longer an
exception" note was already true at this cycle's start.

## 2. `Bunyip ~ Blood Rage`: a bonus unit, not a scope error

`Lycanthrope ~ Change Shape` (bestiary) is a genuine member of the 56-unit
DESC-refusal group already counted there; excluding it from the FACET-gap
group's 22 was correct and is why the group totals reconcile exactly
(56 + 22 = 78). `Bunyip ~ Blood Rage` (bestiary_2) is different: its
`docs/work-inventory.json` entry carries `status: "text-complete"`,
`origin: "copy"`, evidence `monster_ability_held_and_corpus_record_carries_
real_description` — the inventory already counts it done, on the strength of
that evidence alone, with **no backing corpus JSON record**. It is therefore
outside the ledger's `no_record` population (which only considers
not-done units) even though the transcriber correctly still refuses to
assign it a real facet (a `.COPY=` row, same shape as `Aurumvorax ~ Rake`).
Defaulting and shipping it anyway is not scope creep: it is the same
mechanism, applied to a unit the corpus was already missing a record for.
Reported as its own number throughout (23 stamped, 22 of which are the real
`no_record` population) per `decisions.md §12c`.

## 3. Five named shapes, not one guessed default

Direct inspection of every refused row's raw `TYPE:` field (`type_segments`,
called against each row read live from the pinned oracle) found the 22-unit
group is not one shape:

| Reason | Units | Example |
|---|---:|---|
| `delivery_only_no_facet_segment` | 7 | `Denizen of Leng ~ Planar Fast Healing` (`ModifyHP.Supernatural`) — `decisions.md §27`'s own cited example |
| `book_specific_type_label_no_facet_vocabulary_gap` | 11 | `Unfettered Eidolon ~ Str` (`Unfettered Eidolon Stat Selection`) |
| `copy_row_base_ability_type_unresolved` | 3 | `Aurumvorax ~ Rake` (`.COPY=` overlay, no `TYPE:` token at all, base row absent corpus-wide) |
| `type_internal_only_no_facet_no_delivery` | 1 | `Morlock ~ Sneak Attack` (`TYPE:Internal`, round 6's "genuinely novel shape") |
| `missing_type_token_no_facet` | 1 | `Lamia Matriarch ~ Spells` (no `TYPE:` token, no `.COPY=` either) |

Every defaulted record carries its reason as a distinct, machine-countable
string — `scripts/row17_census.py --check` verifies the marker is never set
without one.

## 4. RED → GREEN (`AGENTS.md` non-negotiable rule 1)

`python3 -m unittest scripts.tests.test_transcribe_monster_tables.
ProvisionalFacetDefaultRound8` — all 9 new tests pass against the fix.
Mutation-style proof (`test_mutation_proof_reverting_to_parse_type_alone_
reproduces_the_drop`): calling the OLD `parse_type` directly on every
synthetic row used by the shape tests still raises `UnmodelledFacet`,
proving `parse_type_or_provisional_default` is doing real work. Full module:
`python3 -m unittest scripts.tests.test_transcribe_monster_tables` — 34
tests, 33 pass, 1 pre-existing failure (`InternalBundleAbilityHopIsResolved
::test_an_ability_no_bundle_names_stays_an_orphan_and_is_not_shipped`,
confirmed present and unrelated to this diff — round 5/6/7's own receipts
already named this test as pre-existing and out of this lane's territory).

`gen_book_cache.rs`'s two new tests
(`verified_citation_line_exempts_a_copy_overlay_rows_compound_first_column`,
`verified_citation_line_still_catches_a_genuinely_stale_citation`) both
pass; the second is the negative control proving the fix is not a blanket
bypass — a genuinely stale, non-`.COPY=` citation still panics.

## 5. Corpus regeneration — additive only, verified before AND after, with one caught near-miss

`git status --porcelain` before every commit. `transcribe_monster_tables.py
<book>` for the 5 affected books (zero deletions, only new
`MonsterAbilityRecord` blocks and header comments). `cargo run --bin
gen_book_cache -- <book>` for the same 5 (`beastiary` via its on-disk-dir
alias): 23 new JSON files total, matching the printed
`N new monster abilities` count exactly per book, zero deletions.
`scripts/stamp_monster_ability_provisional_facets.py <book>` for the same 5:
23 records stamped, matching `provisional_facet_units(book)`'s own count per
book exactly (the script raises if any named unit has no matching JSON
file, so a silent mismatch is not possible).

**Near-miss:** `cargo run --bin enrich_monster_ability_raw_tokens` (run once,
as its own doc comment requires) enriched 1,829 records corpus-wide — every
`monster_ability` record anywhere lacking `raw_tokens`, not just this
cycle's 23. `git status --porcelain` immediately after showed 1,806 modified
files this cycle never intended to touch, across books this lane does not
own. Reverted via `git checkout --` on exactly those 1,806 paths (computed
as "every modified `data/corpus/**/monster_ability/*.json` file", since none
of this cycle's own 23 new records are ever `M` — they are `??`, untracked).
Re-verified after revert: exactly the 23 new files remain, each still
carrying both `raw_tokens` and the `shape_provisional_default`/
`shape_provisional_reason` marker.

No `--allow-stamp-loss` used anywhere in this cycle.

## 6. What was actually closed this cycle: 22 units (+1 bonus), by five named, generic mechanisms

**Closure this cycle: 22 units, real ingestion via a provisional default (not
a measured shape — see `§16`/`§27` below), 0 reclassified, 0 reachability
gained beyond the ordinary owned/orphan split.** Plus 1 bonus unit (`Bunyip ~
Blood Rage`) outside the `no_record` population, same mechanism.

`monster_ability` `no_record`: **78 → 56** (re-derived:
`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json` →
`monster_ability 56`). Bundle-wide `no_record`: **106 → 84**.

`decisions.md §27`'s provisional-default count (the fourth, separately-
reported number `§16`/`§27a`/`§27b` require): **22** in the not-done
population (`scripts/row17_census.py --check` prints "§27 provisional
default 22 (corpus-wide total incl. done units: 23)"). **This is NOT a unit
of shape closure** — every one of the 22 (23) still carries `family: F0`
via `f0_reached_by: "fallthrough"` in the shape ledger, and will be revisited
by row 17's own future pass. Reported here as its own bucket, never booked
as Gate 1 progress.

`§16`'s four numbers: **closure 22** (+1 bonus), **reclassification 0**,
**reachability**: 13 newly OWNED (join a monster's `ability_keys`, reachable
through `list_monster_catalog`), 10 newly owner-less (shape-measured,
reachability explicitly NOT claimed, pinned by exact key in
`reach_gate.rs::UNREACHED_RECORD_FINDINGS`) — **instrument correction 0**.

## 7. Tests

```
python3 -m unittest scripts.tests.test_transcribe_monster_tables
  34 tests, 33 passed, 1 failed (pre-existing, confirmed unrelated)
python3 scripts/row17_census.py --check
  clean (no malformed-marker output)
cargo build --locked --lib                                          clean, 10 warnings (pre-existing shape)
cargo test --locked --lib monster_chassis::                          8 passed, 0 failed (pin re-derived: 3749 / 0xfc51_2110_6900_558e)
cargo test --locked --lib rules_tables::                             561 passed, 0 failed, 3 ignored
cargo test --locked --bin gen_book_cache verified_citation_line      2 passed, 0 failed
cargo build --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins   clean
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins monster_catalog::
  26 passed, 0 failed (pin re-derived: owner_less_records_held 1066 -> 1076)
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins corpus_ingest_diagnostic::
  14 passed, 1 failed (pin re-derived: beastiary1 count 710 -> 711; the 1 remaining failure is
  `advanced_race_guide`, pre-existing, confirmed unrelated to this diff -- sibling lane's own
  named, pre-existing, untouched territory per round 6/7's receipts)
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins reach_gate::
  23 passed, 8 failed — IDENTICAL split to round 4/5/6/7's own recorded baseline (re-verified:
  none of the 8 failing tests' own printed detail names `bestiary`, `bestiary_2`, `bestiary_3`,
  `inner_sea_bestiary`, `inner_sea_gods`, or `monster_ability` -- every one is
  `advanced_race_guide`/`apg`/`bestiary_4`/`bestiary_5`/`inner_sea_races` `companions`, unrelated
  to this cycle's scope or diff). `bestiary_1`/`bestiary_2`/`bestiary_3`'s own book-level tests
  (3 pins moved by this cycle's own diff) fixed inline before this run, plus `bestiary_3`'s
  `UNREACHED_RECORD_FINDINGS` entry gained its 10 new keys.
```

## 8. What remains (three separate figures per `decisions.md §16`)

Remaining **56**, unchanged shape from round 6/7's own grouping (this cycle
closed the FACET-gap group; the DESC group is untouched):

1. **Multi-`DESC:` parse refusals — 56 units, unchanged.** `PRERULE`/
   `PREVAREQ`-gated variant-text rows, each needing its own `BONUS:VAR`
   value traced. `parse_desc`'s own docstring already names the fifth
   refused shape this would need to generalise — round 6's own assessment,
   re-confirmed live this cycle by re-running `provisional_facet_units`
   over all 9 books and cross-checking the union of DESC+FACET refusals
   against the ledger's real `no_record` set: exactly 56 + 22 = 78, with no
   third undiscovered group.
2. **`occult_adventures`, `advanced_race_guide` companions — 0 units,
   already closed.** Not this cycle's work; an earlier commit
   (`916228e9a7`) already closed them per `§27b`, re-confirmed live.

## 9. Next-cycle plan

1. **Multi-`DESC:` `PREVAREQ`/`PREVARGT` shape (56 units, the entire
   remaining `monster_ability` `no_record` population).** Trace each row's
   own `BONUS:VAR` value; a generalised sixth `parse_desc` branch, per round
   6's own assessment, re-confirmed this cycle. Real per-record work, but
   likely substantially fewer than 56 distinct cycles once the sixth branch
   exists — report how many distinct sub-shapes the 56 actually need before
   estimating further.
2. **Row 17's real categorization pass** (kanban row 17,
   `epic-7-shape-categorization-100`) now has 22 more units in its honest
   population (`fallthrough` + `provisional_default`, sequenced after
   bundle-wide `no_record` reaches zero) — this cycle's 5 named shapes are a
   starting point for that pass, not a substitute for it.
