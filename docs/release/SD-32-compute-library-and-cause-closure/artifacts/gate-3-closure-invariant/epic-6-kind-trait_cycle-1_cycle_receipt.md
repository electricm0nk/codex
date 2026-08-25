# Cycle epic-6-kind-trait/1 — Gate 3 closure invariant / Epic 6, `kind: trait` (`decisions.md §25`)

- **Card ID:** `epic-6-kind-trait` (row 16)
- **Actor:** `t9-onboarding`
- **Base:** `0a1982061e93592697a12c60450182b0c88d860c` (pinned `PIN`, the decisions.md §25 commit itself).
- **Commit SHA:** (this cycle's own commit, see push log)
- **Files touched:**
  - `src/bin/v06_work_inventory.rs` — `Kind::Trait` variant + `id()`/`ALL` entries; `refine_kind`'s
    `Kind::Ability` arm gains a `TYPE:Trait`/`TYPE:Trait.*` redirect, checked before the
    `CATEGORY:FEAT` redirect; verdict-table arm (`not_ingested("trait_content_has_no_engine_table")`);
    4 new fixture tests in `kind_ability_tests`.
  - `scripts/census_independent.py` — `_row_type_tag`/`_row_is_pf1_trait` helpers (byte-identical rule
    to the Rust side, `decisions.md §12b`); `"trait"` added to `ADDED_KINDS`; `row_dependent` branch
    checks `_row_is_pf1_trait` before the `CATEGORY:FEAT` redirect.
  - `scripts/tests/test_census_independent.py` — 2 new fixture tests
    (`test_type_trait_row_in_a_bare_abilities_file_counts_as_kind_trait`,
    `test_type_trait_row_is_checked_before_the_feat_redirect`).
  - `src/bin/ingest_race_traits.rs` — `parse_row` gains a 4th row shape, the "Adopted Race" selector
    (`ADOPTED_RACE_SELECTOR_TYPE`/`ADOPTED_RACE_SELECTOR_CHOOSE_PREFIX` constants,
    `is_adopted_race_choose_selector` detection and `TraitRow` field); `ingest_book`'s scope filter
    admits a selector row past `IN_SCOPE_RACES` even for a race with no chassis. 3 new fixture tests.
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — new row 16
    (`epic-6-kind-trait`, `in-progress`); rows 11/15 untouched.
  - `docs/retro/events/t9-onboarding.jsonl` — 1 incident logged (see below).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own diff, `git diff --unified=0` over the 4
  touched source/test files).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope).
- **Acceptance criterion (verbatim, `decisions.md §25`):** the 14 `adopted_race_choose_selector` units
  (`bestiary_2` 7, `bestiary_3` 5, `bestiary_5` 1, `bestiary_6` 1) close by real ingest — a new
  `kind: trait` schema, an ingest tool (extending an existing generic path), a reach-gate family, a
  character-builder picker, and `player_companion` book onboarding.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`)
- **Status:** `in-progress` — real, tested progress landed; the 14 units are **not** closed this
  cycle. Two genuine blockers, both pre-existing and out of this cycle's scope, are named below rather
  than worked around.
- **Notes:** see full account below.
- **Discovery forwards:** the two blockers below, plus the `ingest_races.rs` cross-tool ownership
  question for `bestiary_2/3/5/6`'s `core_essentials/races/*` directories.
- **Next-cycle plan:** see kanban row 16's own text (mirrored below).

## 0. Re-derivation of the brief's own figures (`decisions.md §17a`)

`find data/corpus -type d -name trait` — **zero** directories, confirmed before writing anything
(matches the brief). `python3 scripts/t2b_adoptive_parentage_census.py` — **14** `adopted_race_choose_selector`
units, `bestiary_2` 7 / `bestiary_3` 5 / `bestiary_5` 1 / `bestiary_6` 1, confirmed unchanged from the
prior cycle's own re-derivation (`epic-2-t2b-adoptive-parentage_cycle-1_cycle_receipt.md`). The prior
receipt is the authoritative population source for this cycle; both counts re-verified, not assumed.

## 1. `kind: trait` schema — the shape recognition half

A bare (non-`_race`/`_class`/`_companion`/`_familiar`) `*abilities*.lst` row whose `TYPE:` value is
exactly `Trait` or starts with `Trait.` is PF1e's chargen Trait mechanic. Two real corpus shapes
confirmed: `TYPE:Trait.RaceTrait.Oread Race Trait` (`inner_sea_races/isr_abilities.lst:78`, the
brief's own worked example) and bare `TYPE:Trait` (`ultimate_campaign/uca_abilities_traits.lst`).

Re-derived corpus-wide against the pinned oracle, over every book currently registered in
`v06_work_inventory.rs` (`roleplaying_game/*` + `EXTRA_BOOK_DIRS`):

```
advanced_players_guide/apg_abilities.lst        : 90
core_rulebook/cr_abilitycategories.lst          : 1  (a bare TYPE:Trait row in the categories file)
ultimate_campaign/uca_abilities_traits.lst      : 231
ultimate_psionics/up_abilities_apg.lst          : 32
inner_sea_gods/isg_abilities.lst                : 116
inner_sea_races/isr_abilities.lst               : 96
                                          TOTAL  : 566
```

**`inner_sea_races` alone gives real content for 13 of the 14 target selector races** — 1 trait per
race (`Dhampir`, `Fetchling`, `Grippli`, `Ifrit`, `Oread`, `Sylph`, `Undine`, `Catfolk`, `Ratfolk`,
`Suli`, `Vanara`, `Vishkanya`, `Skinwalker` all have exactly 1 `RaceTrait.<X> Race Trait` row in
`isr_abilities.lst`). `Rougarou` has **zero** hits corpus-wide against every book carrying a
`RaceTrait.<Race> Race Trait` marker — matching the prior cycle's own "proven empty" finding
unchanged.

The corpus-wide scan also surfaced 7 `player_companion` books carrying real content for the same 13
races (`people_of_the_sands`, `blood_of_the_elements`, `bastards_of_golarion`, `agents_of_evil`,
`blood_of_the_night`, `dirty_tactics_toolbox`, `blood_of_the_moon` — none currently registered), so
`inner_sea_races` alone is not the whole trait pool, but it is sufficient to make every non-Rougarou
target race's selector genuinely resolvable once the `Kind::Trait` corpus write path is unblocked.

### Landed, tested

- `src/bin/v06_work_inventory.rs`: `Kind::Trait`, `refine_kind`'s `Kind::Ability` arm redirect, 4
  fixture tests (`type_trait_dotted_row_redirects_to_trait`, `type_trait_bare_row_redirects_to_trait`,
  `type_value_naming_trait_in_a_later_segment_does_not_redirect` — the false-positive guard — and the
  pre-existing `non_feat_ability_row_stays_ability_under_refine_kind` re-verified unaffected).
- `scripts/census_independent.py`: `_row_is_pf1_trait`, byte-identical rule, checked before the FEAT
  redirect in the `row_dependent` branch; `"trait"` added to `ADDED_KINDS`. 2 fixture tests.
- **RED→GREEN, Rust:** neutered the `TYPE:` check (`type_value == "Trait" ...` replaced with `false`
  ad hoc during the proof), both new redirect tests failed for the intended reason
  (`assertion failed: left == Kind::Trait`), reverted.
- `cargo test --locked --bin v06_work_inventory`: 353 → 357 passed, 0 failed.
- `python3 -m unittest scripts.tests.test_census_independent`: 26 → 28 passed, 0 failed (run with
  `PCGEN_CORPUS_ROOT` set to the repo-local pinned oracle).

### Ingest tool choice (`decisions.md §17`, "extend a generic path")

Read all three candidates before writing anything, per the brief:

- **`scripts/ingest_simple_filename_kinds.py`** — its `SIMPLE_FILENAME_KINDS` table is
  filename-substring-only (one row picks a whole file). `trait` rows are *mixed inside* the same
  `*abilities*.lst` files as ordinary `Ability`/`Feat` content — a per-row test, not a filename rule —
  so this table genuinely cannot express `trait`. Not a fit.
- **`scripts/ingest_ability.py`** — this is the reference implementation `trait` most resembles (a
  bare-abilities per-row disposition test), but it is hard-coded to write `kind: ability` records and
  screen/report against that one kind's own vocabulary. Extending it to a second kind would either
  fork it (the `§17` failure mode) or need real generalization work this cycle did not reach.
- **`scripts/ingest_generic_kind.py`** — **the fit.** It is already `--kind`-parameterized, already
  reads `docs/work-inventory.json` for `(kind, join_status=="no_record")` units, already runs the
  shared PI screen + `§24` neutral-name path, and already writes the generic
  `population/completeness/ingested_at/data{key,name,description,raw_tokens}/source/wiring_class`
  shape `decisions.md §25` item 1 asks for ("modelled on existing kind schemas, not invented fresh").
  `python3 scripts/ingest_generic_kind.py --kind trait --ledger <shape_ledger output> ` is the exact
  next command once `docs/work-inventory.json` carries `kind: trait` units — **not yet run this
  cycle**, blocked by §3 below.

**Chosen: `ingest_generic_kind.py --kind trait`, once `docs/work-inventory.json` is safely
regeneratable.** No new ingest code is needed beyond registering the kind in the two walkers above.

## 2. Selector-row parsing — the `adopted_race_choose_selector` shape itself

The 14 units are the *selector* rows (`KEY:Adopted Race ~ <X>`, `TYPE:AdoptiveRace`,
`CHOOSE:ABILITYSELECTION|Special Ability|TYPE=<X> Race Trait`), distinct from the Trait *pool* content
in §1. The prior cycle's receipt (`epic-2-t2b-adoptive-parentage_cycle-1_cycle_receipt.md`) found these
rows un-parseable by `ingest_race_traits.rs::parse_row` (no `TYPE:<Race> Racial Trait`/`Subrace`
component, and `CATEGORY:Special Ability` — not `Adoptive Parentage` — so neither existing branch
matches).

**Landed:** a 4th `parse_row` shape (`is_adopted_race_choose_selector`), gated on the exact,
dot-free `TYPE:AdoptiveRace` plus a `CHOOSE:ABILITYSELECTION|Special Ability|TYPE=` token, resolving
`race_key` to the row's own display name — the same "the selector resolves to the race it names"
convention `is_adoptive_parentage_option` already uses. No new `RaceTraitCacheData` schema field: the
`CHOOSE:` pool token already ships verbatim in `raw_tokens`, so a future resolver re-derives the pool
type from those bytes rather than needing a new committed field.

**`IN_SCOPE_RACES` bypass.** 3 of the 14 target races (`Dhampir`, `Skinwalker`, `Rougarou`) have no
chassis record in this project (`ingest_races.rs` never modelled them) and so are absent from
`IN_SCOPE_RACES`. The selector's pool is resolved against the separate `Kind::Trait` population, never
`RaceCorpus::traits_for`, so — unlike every other row shape this file gates — it needs no chassis.
`ingest_book`'s scope filter now reads `in_scope.contains(...) || row.is_adopted_race_choose_selector`.

**RED→GREEN.** Neutered the detection (`false && racial_trait_race.is_none() && ...`); both new
fixture tests failed for the intended reason (`panicked ... Adopted Race selector row is not dropped`,
`src/bin/ingest_race_traits.rs:1818`/`:1851`); restored, re-ran, GREEN.

`cargo test --locked --bin ingest_race_traits`: 16 → 19 passed, 0 failed.

**Not yet wired to a `BookSource`.** `bestiary_2`/`bestiary_3`/`bestiary_5`/`bestiary_6` are absent
from `BOOK_SOURCES` today. Their `_abilities_race.lst` files physically live under
`core_essentials/races/<race>/` — the SAME directories `ingest_races.rs` already writes each race's
own standard-trait chassis into (`data/corpus/bestiary_2/race_trait/oread/oread_type.json` and
siblings already exist, written by that other tool). Adding a `BookSource` here blind risks a
cross-tool write collision into a directory another generator owns — exactly the incident class card
1's own correction (this file, prepended entry) found and fixed for a *different* pair of generators
(`gen_cache_spell_lane_dump`/`gen_cache_spell_mod_access` sharing `data/corpus/{occult_adventures,
ultimate_magic}/spell/`, 1,580 near-miss deletions). Reading `ingest_races.rs`'s own file-ownership
boundary for these 13 race directories, and proving no collision before adding the `BookSource` rows,
is real work this cycle did not have the budget to do safely — named as the next-cycle's first item
rather than risked.

## 3. `docs/work-inventory.json` regen — blocked, escalated, not worked around

Bootstrapped the oracle (empty in this fresh worktree, per `§2.1`), confirmed populated
(`scripts/verify.sh --only preflight-oracle` → PASS, `7f818006e371188e5717fd18d74d18a420747fc6`).

Ran `cargo run --bin corpus_literal_sweep -- --json-out <report>` (required before any regen, per this
bundle's own near-miss lesson). Result: **`clean: false`, 1 finding**, unrelated to this cycle's diff
(nothing here touches `data/corpus/**`):

```
corpus-literal-sweep: MISMATCH data/corpus/inner_sea_magic/ability/hidden_wand.json:
  token not byte-present in corpus token closure: DESC:[redacted PI]
```

Inspected the record (read-only, per `decisions.md §15` — a cycle that reaches a suspected
Product-Identity record stops on it, names it by coordinate, never transcribes or judges its content):
`raw_tokens`'s `DESC` entry is `"[redacted PI]"`, `pi_field: "raw_tokens"` (not `"description,raw_tokens"`),
and the record's **top-level `data.description` field carries the full, un-redacted text**. This is a
declared-vs-actual inconsistency independent of whether the content is genuinely PI: either the
redaction is real and `data.description` is a live leak, or the redaction was a false positive that
should never have fired. **Neither call is this cycle's to make** — named by coordinate only, content
not reproduced here, escalated via `scripts/retro.py incident`
(`1787503285569-t9-onboarding-1d4c46`, `docs/retro/events/t9-onboarding.jsonl`,
`recurrence-key: corpus-literal-sweep-pi-exemption-gap`).

**Why this blocks every regen, not just this cycle's:** `corpus_literal_sweep`'s own `compare_tokens`
only credits a record's tokens as "verified" for the WHOLE sweep when the sweep is clean (one book's
mismatch tells you nothing about another book's records — the discipline `decisions.md §20`/this
bundle's own near-miss already established). A `clean: false` report therefore has an **empty**
`verified` list. `v06_work_inventory`'s stamp-loss guard reads that list to decide which currently
`literal-verified`/`fixture-verified` units may keep that status on the new candidate JSON; an empty
list means **every** such unit would be downgraded. Confirmed live:

```
refusing to write docs/work-inventory.json: this run would drop 8247 of the 8247
verification stamp(s) (literal-verified/fixture-verified) it currently carries.
```

`docs/work-inventory.json`'s own current status distribution: `literal-verified: 6506`,
`fixture-verified: 1741` — **6506 + 1741 = 8247**, matching the refusal message exactly. This is a
**real** loss the regen would cause, not a false positive of an over-strict guard: `--allow-stamp-loss`
was correctly **not** used. `docs/work-inventory.json` itself is untouched
(`git status --porcelain` — clean on that path) — the write was refused before touching disk.

**Consequence:** the `Kind::Trait` classifier landed in §1 cannot yet enumerate any unit into
`docs/work-inventory.json`, so `ingest_generic_kind.py --kind trait` has nothing to read yet, and the
566-unit Trait pool (§1) plus the 11-of-14 selector rows this cycle's parser can now recognize (§2)
both stay un-ingested. Both are real, tested, ready-to-run the moment this blocker clears.

## 4. What is NOT closed, and why (`decisions.md §16`)

**Closed by real ingest: 0 of 14.** **Reclassified: 0.** **Reachability: 0** — no picker, reach-gate,
or `Kind::Trait` corpus record exists for any of the 14 units yet. This cycle's real, tested output is
entirely in the *shape-recognition* and *row-parsing* layers (§1-2), both blocked from reaching the
corpus by the two findings in §3 and §2's cross-tool-ownership question, neither of which this cycle
introduced or could safely resolve within its own scope.

No stub was written: nothing claims `success: true` for un-performed work, no picker handler exists
that returns fabricated data, and the kanban card is `in-progress`, not `complete`.

## 5. PI discipline (`decisions.md §15`/`§19`/`§24`)

One suspected PI-consistency record found and named by coordinate (§3), not transcribed, not judged.
No other PI exposure identified in this cycle's own diff (no `data/corpus/**` write happened at all).

## 6. Verification run (this cycle)

```
cargo build --locked --bin v06_work_inventory        # clean
cargo test  --locked --bin v06_work_inventory kind_ability_tests   # 16 passed (4 new), 0 failed
cargo test  --locked --bin v06_work_inventory                       # 357 passed, 0 failed
cargo build --locked --bin ingest_race_traits         # clean
cargo test  --locked --bin ingest_race_traits         # 19 passed (3 new), 0 failed
python3 -m py_compile scripts/census_independent.py   # clean
PCGEN_CORPUS_ROOT=<pinned oracle> python3 -m unittest scripts.tests.test_census_independent
                                                       # 28 passed (2 new), 0 failed
cargo run --bin corpus_literal_sweep -- --json-out <report>   # clean:false, 1 finding (§3, unrelated)
cargo run --bin v06_work_inventory                    # refused write (§3), docs/work-inventory.json untouched
```

## 7. Discovery forwards

- `## DISCOVERED`: `data/corpus/inner_sea_magic/ability/hidden_wand.json` — suspected PI-redaction
  inconsistency (§3), blocking every `docs/work-inventory.json` regen bundle-wide until resolved.
- `## DISCOVERED`: `bestiary_2/3/5/6`'s `_abilities_race.lst` files (physically under
  `core_essentials/races/<race>/`) are read by `ingest_races.rs` (chassis) but not by
  `ingest_race_traits.rs` (alternate traits / this epic's selector rows) — adding a `BookSource` for
  the selector rows needs that other tool's file-ownership boundary read first.

## 8. Next-cycle plan

1. Resolve the `hidden_wand.json` finding (operator ruling on the content, or a `corpus_literal_sweep`
   exemption-logic fix — whichever the operator directs) so `docs/work-inventory.json` can regenerate
   safely with a full before/after status-distribution diff.
2. Once unblocked: regenerate, then `python3 scripts/ingest_generic_kind.py --kind trait --ledger ...`
   for the 566-unit pool.
3. Read `ingest_races.rs`'s ownership of `core_essentials/races/{oread,sylph,undine,ifrit,fetchling,
   grippli,dhampir,catfolk,ratfolk,suli,vanara,vishkanya,skinwalker}/`; add the `BookSource` rows for
   `bestiary_2`/`bestiary_3`/`bestiary_5`/`bestiary_6` only once proven collision-free.
4. Build `trait_pool` (new `src/rules_core/` module, reads `data/corpus/<book>/trait_generic/*.json`,
   matches a selector's `CHOOSE:...TYPE=<X> Race Trait` token against a Trait record's own
   `TYPE:Trait.RaceTrait.<X> Race Trait` third dot-segment), the `race_trait_picker.rs` DTO (model:
   `AdoptiveParentageOptionDto`), and the `reach_gate.rs` family (model: `race_traits_reach`'s
   `adoptive_parentage_options` loop) — all three have a working precedent in this same codebase.
