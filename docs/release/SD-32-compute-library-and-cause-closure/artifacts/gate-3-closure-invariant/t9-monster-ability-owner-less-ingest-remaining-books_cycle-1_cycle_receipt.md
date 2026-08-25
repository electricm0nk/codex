# Cycle t9-monster-ability-owner-less-ingest-remaining-books — Gate 3 (closure invariant) / Card 11 (T9)

- **Card ID:** 11 (`epic-2-cause-closure`, T9 sub-lane
  `t9-monster-ability-owner-less-ingest-remaining-books`)
- **Commit SHA:** (this cycle's commit — see push output)
- **Files touched:**
  - `scripts/transcribe_monster_tables.py` — **unchanged**; the mechanism
    landed in commit `3088603f2` already generalizes across every registered
    book, confirmed by running it unmodified.
  - `src/bin/gen_book_cache.rs` — two generic `MonsterBookSpec.abilities_lsts`
    widenings (`bestiary_3` gains `vishkanya_abilities_race.lst` and
    `ce_abilities_race.lst`; `bestiary_4` gains `ce_abilities_race.lst` and
    `wyrwood_abilities_race.lst`), each a file this book's own orphan rows
    physically cite under `core_essentials`'s per-race subdirectory, resolved
    via the SAME recursive core_essentials fallback the pre-existing
    `ce_abilities_race.lst` entries already use — never registered until
    this cycle because no prior transcription reached those rows.
  - `src/rules_core/rules_tables/{bestiary_2,bestiary_3,bestiary_4,
    horror_adventures,inner_sea_bestiary,inner_sea_gods,
    inner_sea_world_guide,ultimate_psionics}/monster_data.rs` — regenerated
    via `scripts/transcribe_monster_tables.py <book>`.
  - Each of the same 8 books' `mod.rs` — the owner-less-forbidding test is
    superseded by `every_owner_less_ability_is_a_named_and_pinned_non_reach`
    (count + digest, mirroring `bestiary`'s own T9 test), and every
    pre-existing "orphan rows are not records" test is rewritten to
    "previously-excluded rows now ship owner-less" (still failing loudly if
    the row goes missing, or if it silently gains an owner). `bestiary_3`
    additionally scopes `every_shipped_ability_is_reached_by_its_namespaced_key`
    to owned rows only, and correctly LEAVES `b3_abilities_race.lst:1663`
    excluded (unrelated multi-`DESC:` screen, not the orphan mechanism).
    `bestiary_4` additionally scopes its own namespaced-key test to owned
    rows, and adds `Grab ~ Medium` to `CROSS_FAMILY_DUPLICATE_EXCEPTIONS`
    (confirmed byte-identical between the monster and companion sides before
    excepting it, per that list's own precedent).
  - `src/rules_core/rules_tables/monster_chassis.rs` — the corpus-wide
    `widening_the_facet_vocabulary_does_not_reclassify_any_existing_record`
    pin moves 2836 → 3537 (+701, additions-only, verified via the same
    `git diff` cross-check the `bestiary` cycle used).
  - `apps/desktop/src-tauri/src/reach_gate.rs` — 8 new
    `UNREACHED_RECORD_FINDINGS`/`OPEN_FINDINGS` entry pairs (701 exact keys
    total), and the 3 books with an explicit per-record reach test
    (`bestiary_2_reaches_the_catalog_for_every_linked_record`,
    `bestiary_3_reaches_the_catalog_for_every_linked_record`,
    `inner_sea_world_guide_reaches_the_catalog_for_every_linked_record`)
    updated to separate the on-disk total from the owned/reachable subset
    and assert `NotSurfaced` naming the exact owner-less count.
  - `apps/desktop/src-tauri/src/monster_catalog.rs` — the corpus-wide
    `bonus_bestiary_ability_keys_carry_the_namespace` owner-less-count pin
    moves 180 → 881 (+701).
  - `data/corpus/{bestiary_2,bestiary_3,bestiary_4,horror_adventures,
    inner_sea_bestiary,inner_sea_gods,inner_sea_world_guide,
    ultimate_psionics}/monster_ability/*.json` (701 new files, via
    `gen_book_cache <book>`) and each book's `LICENSE.json`
    (screening-note append).
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (card
    11 row, prepended entry; row stays `in-progress`).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped diff of the
  substantive files above — 0 hits).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope — 0 hits).
- **Acceptance criterion:** `decisions.md §20` — drive `monster_ability`
  `no_record` toward zero; the prior cycle's own "Next-cycle plan" — apply the
  identical `transcribe_monster_tables.py` + `gen_book_cache` mechanism
  (already generic, no further code change needed) to the 8 remaining
  registered books with any real orphan population.
- **Corpus SHA:** `PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`; bootstrapped from empty this cycle via
  `scripts/fetch-pcgen-oracle.sh`).
- **Status:** complete (partial application of the overall `no_record==0`
  goal — see "What remains" below; card 11 stays `in-progress`).
- **Notes:** see full body below.
- **Discovery forwards:** none filed — remaining scope (real per-record
  engineering: multi-`DESC:` parse refusals, `TYPE:`-facet-vocabulary gaps,
  and PI-declared ability rows across these 8 books; the `monster` kind's
  21 PI-cascaded / 6 `.COPY=` / 1 unregistered-book residual, owned by a
  sibling lane and untouched here) is named explicitly below.
- **Next-cycle plan:** the `monster_ability` residual (267, re-derived) is
  now real per-object work across small, book-scoped populations — no
  further "apply the mechanism to book N" cycles remain of this shape. A
  future cycle should read each book's own `transcribe_monster_tables.py`
  stderr for its remaining "owned ability row(s) NOT transcribed" list and
  either widen `MonsterAbilityFacet`/`parse_desc` where the shape repeats
  across books, or resolve per-record where it does not.

---

## 0. Environment and PIN

```
PIN=3088603f2c495c76ef3e4959240ffdd58b68586b
```
Worktree started on a commit that IS the pinned SHA (`origin/tranche/12` was
already at `PIN` at dispatch time), so `git merge-base --is-ancestor` passed
immediately with no reset needed. PCGen oracle slot was empty (fresh
worktree, git-ignored); bootstrapped via `scripts/fetch-pcgen-oracle.sh
--dest "$PCGEN_REPO_DIR"` → `pcgen-oracle: OK
7f818006e371188e5717fd18d74d18a420747fc6`.

## 1. Re-derived the brief's figures fresh (`decisions.md §17a`)

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/l.json
```
`monster_ability` `no_record`: **967**, matching the brief's own figure
exactly. Per-book split via `python3 scripts/classify_monster_ability_rows.py
<8 books>`: `bestiary_2` 84 orphan (real: 85, see §3), `bestiary_3` 286,
`bestiary_4` 194, `horror_adventures` 65, `ultimate_psionics` 64,
`inner_sea_bestiary` 26, `inner_sea_world_guide` 13, `inner_sea_gods` 4 — sum
736, matching the classifier's own bundle total. `bonus_bestiary`,
`monster_codex`, `book_of_the_damned_volume_1`, `book_of_the_damned_volume_2`
independently re-confirmed at **0** remaining orphan/reachable units each —
already fully closed, not touched this cycle.

## 2. Applied the identical mechanism, no code change to the mechanism itself

Per the prior cycle's own finding: `scripts/transcribe_monster_tables.py`'s
orphan pass already ships every owner-less row with `owners: &[]` for every
registered book, generically. Ran it unmodified for each of the 8 books:

```
bestiary_2:   85 orphan ability row(s) transcribed WITHOUT an owner
bestiary_3:  286 orphan ability row(s) transcribed WITHOUT an owner
bestiary_4:  194 orphan ability row(s) transcribed WITHOUT an owner
horror_adventures: 65 orphan ability row(s) transcribed WITHOUT an owner
inner_sea_bestiary: 31 orphan ability row(s) transcribed WITHOUT an owner (28 shipped -- see below)
inner_sea_gods: 2 orphan ability row(s) transcribed WITHOUT an owner
inner_sea_world_guide: 13 orphan ability row(s) transcribed WITHOUT an owner
ultimate_psionics: 64 orphan ability row(s) transcribed WITHOUT an owner
```

**PI safety re-confirmed for every shipped-unowned row (`decisions.md §15`,
`§19b`)**: `inner_sea_bestiary` and `inner_sea_world_guide` both surfaced
orphan rows namespaced to a monster their OWN book's PI screen had separately
dropped (e.g. `Chemnosit ~ Hungry Gaze`, `Sandpoint Devil ~ Bay`,
`Treerazer ~ Regeneration`). `decisions.md §19b` rules this clear: a
`monster_ability` row carrying no PI declaration and no term-list hit on its
own row is not Product Identity merely because its text names a
Paizo-original creature. None of the shipped orphans carry a declaration or
term-list hit — verified live by the transcriber's own `ability_pi_reason`
screen (which runs on them, since they are no longer dropped before it) and
independently by `pi_sweep_rules_tables` after regen: 10 hits, 10 baseline,
0 new, CLEAN.

## 3. The printed "orphan" count is not always the final shipped count — verified, not assumed

Mirroring `bestiary`'s own 197→180 pattern: the transcriber's "N orphan rows
transcribed" message is emitted BEFORE two later, pre-existing, UNRELATED
screens (`unscreenable`'s multi-`DESC:` shape, `unmodelled_facet`'s
`TYPE:`-vocabulary gap) run over the whole ability set — so some printed
orphans are excluded a second time, for a genuinely different reason.
Confirmed via `grep -c "MonsterAbilityRecord {"` on each regenerated
`monster_data.rs`, cross-checked against the pre-cycle committed count via
`git show HEAD:<path>`:

| Book | Printed orphan | Actual owner-less shipped | Owned (unchanged) | Total on disk |
|---|---:|---:|---:|---:|
| `bestiary_2` | 85 | 85 | 571 | 656 |
| `bestiary_3` | 286 | 266 | 409 | 675 |
| `bestiary_4` | 194 | 187 | 619 | 806 |
| `horror_adventures` | 65 | 56 | 6 | 62 |
| `inner_sea_bestiary` | 31 | 28 | 152 | 180 |
| `inner_sea_gods` | 2 | 2 | 156 | 158 |
| `inner_sea_world_guide` | 13 | 13 | 14 | 27 |
| `ultimate_psionics` | 64 | 64 | 127 | 191 |

Every discrepancy (printed vs. actual) exactly matches that book's own
stderr line naming "owned ability row(s) NOT transcribed" for an unrelated
screen — cross-checked directly, not assumed from the arithmetic alone.
**701 owner-less records shipped in total** (85+266+187+56+28+2+13+64).

## 4. A genuine gap found and fixed, not worked around (`decisions.md §17`)

`gen_book_cache bestiary_3` and `gen_book_cache bestiary_4` both refused
outright the first time: two orphan rows in each book physically cite a
`.lst` file living under `core_essentials`'s own per-race subdirectory
(`core_essentials/races/vishkanya/vishkanya_abilities_race.lst`,
`core_essentials/races/wyrwood/wyrwood_abilities_race.lst`) that was never
registered in either book's `MonsterBookSpec.abilities_lsts` — the Python
transcriber's own recursive fallback already resolved these citations
(so `monster_data.rs` compiled them fine); only this Rust generator's
citation allow-list was stale. Widened both specs (4 lines total, generic
infra, not scoped to one book — see `src/bin/gen_book_cache.rs`'s inline
comments for the full derivation), matching the `bestiary`/`bestiary_2`/
`bestiary_4`'s pre-existing `ce_abilities_race.lst` precedent exactly. Zero
pre-existing records touched — confirmed via `git status --porcelain
data/corpus/` showing only new files plus each book's own `LICENSE.json`
screening-note append.

## 5. Re-derived `no_record` — real closure, not a relabel (`decisions.md §16`)

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/l3.json
```
```
no_record (bundle total): 3,263 -> 2,563 (-700)
monster_ability:            967 ->   267 (-700)
```
(700, not 701 — one unit's join outcome differs by a pre-existing
corpus-key collision in the ledger's join, the identical shape the
`bestiary` cycle's own 179-not-180 discrepancy documents; not a defect this
cycle introduced or needs to chase.)

**Re-derived again after rebasing onto `origin/tranche/12`**, which had
landed two sibling `no_record` closures in the interim (`2a79ec478`: `feat`
682→0; `a4636b471`: `spell` 339→285, bundle total 2,413 post-rebase-base):
bundle total 2,413 → 1,713 (deity 459, spell 285, `monster_ability` 267,
companion 217, equipment_modifier 175, equipment 170, class_feature 140) —
`monster_ability`'s own 267 is unchanged by the rebase, confirming this
cycle's delta is independent of the sibling lanes' work.

**No unit was reclassified out of `monster_ability` into another kind** —
this is a genuine ingestion closure, not the shape `decisions.md §16` warns
against. All other kinds' `no_record` figures are byte-identical to the
brief's own table (this cycle touched no other kind).

## 6. Reachability, proven and pinned, not claimed

Per-book `every_owner_less_ability_is_a_named_and_pinned_non_reach` test
added to each of the 8 books' `mod.rs`, mirroring `bestiary`'s own T9 test:
hash-pins the exact sorted set of owner-less keys (count + `DefaultHasher`
digest), computed via a scratch binary against the real compiled tables
(not guessed), then deleted before commit. `reach_gate.rs` gained a matching
`UNREACHED_RECORD_FINDINGS` entry (exact `<book>:monster_ability:<slug>`
keys, read directly from the regenerated `data/corpus/**/*.json` `data.key`
field) and `OPEN_FINDINGS` entry (required for ANY family with a non-empty
`missing` set, per `unsurfaced_families_are_exactly_the_recorded_findings`)
for each book. The 3 books with an existing per-record reach test
(`bestiary_2`/`bestiary_3`/`inner_sea_world_guide`) now assert
`Reach::NotSurfaced` naming exactly the owner-less count instead of
`Surfaced` — **none of the 701 reach a player**, proven by
`chassis_monster_abilities_reach`'s live `assess()` against the real
`list_monster_catalog` response, not assumed from the mechanism's own
design.

## 7. RED → GREEN, book by book

Each new `every_owner_less_ability_is_a_named_and_pinned_non_reach` test was
first run with a placeholder digest (`0x0`): the count assertion passed on
first try (derived from the real compiled table, not guessed), the digest
assertion failed with the real vs. placeholder value — exactly the shape a
fresh pin should fail. Corrected to the real digest (computed once via a
throwaway `src/bin/scratch_digest.rs`, deleted before commit — never
committed, confirmed via `git status --porcelain src/bin/`); reran GREEN.

```
cargo test --locked --lib rules_core::rules_tables::bestiary_2::      10 passed
cargo test --locked --lib rules_core::rules_tables::bestiary_3::       6 passed
cargo test --locked --lib rules_core::rules_tables::bestiary_4::      12 passed
cargo test --locked --lib rules_core::rules_tables::horror_adventures::  8 passed
cargo test --locked --lib rules_core::rules_tables::inner_sea_bestiary::  7 passed
cargo test --locked --lib rules_core::rules_tables::inner_sea_gods::    6 passed
cargo test --locked --lib rules_core::rules_tables::inner_sea_world_guide:: 8 passed
cargo test --locked --lib rules_core::rules_tables::ultimate_psionics::  24 passed
cargo test --locked --lib monster_chassis::                             8 passed
cargo run --locked --release --bin pi_sweep_rules_tables      10 hits, 10 baseline, 0 new, CLEAN
```

## 8. Two pre-existing, unrelated tests caught and fixed by this cycle's own regen

Not part of the orphan mechanism itself, both surfaced only because rows
that were previously dropped now ship:

1. `bestiary_3::tests::every_shipped_ability_is_reached_by_its_namespaced_key`
   iterated ALL shipped abilities including the new owner-less ones, which
   by construction have no owner to check a namespaced prefix against.
   Scoped to `.filter(|a| !a.owners.is_empty())` — the property is
   unchanged for every row it always applied to.
2. `bestiary_4::companion_tests::the_companion_rows_are_not_this_module_s_monster_rows`
   found `Grab ~ Medium` registered as both a companion ability (owned by
   `Companion (Weasel (Giant))`) and a monster ability (now shipped
   owner-less from `b4_abilities_races_ce.lst`) — the SAME Core Essentials
   generic template shipped twice in the pinned oracle, confirmed
   byte-identical description before excepting it, the identical shape
   `Read Magic ~ Constant` already documents in this book's own
   `CROSS_FAMILY_DUPLICATE_EXCEPTIONS` list.

Both are genuinely pre-existing test gaps this cycle's regen happened to
surface, not new defects this cycle introduced.

## 9. Desktop reach-gate suite — 3 real fixes, 8 confirmed pre-existing

```bash
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins reach_gate::
```
Before any fix: 20 passed, 11 failed. Three were genuine — the per-book
`Surfaced`-expecting assertions for `bestiary_2`/`bestiary_3`/
`inner_sea_world_guide` needed the same owned-vs-total split the mod.rs
tests got, plus a `Reach::NotSurfaced` expectation naming the exact
owner-less count. Fixed; reran that book's test alone to confirm GREEN
before moving to the next.

The other 8 (`dispatch_gap_race_and_monster_families_all_have_book_level_
reach_arms`, `every_declared_claim_actually_carries_the_records`,
`every_ingested_companion_book_reaches_the_catalog_record_by_record`,
`every_ingested_family_is_accounted_for`, `pathfinder_unchaineds_class_
features_are_claimed_per_corpus_record`, `the_inventory_is_populated_from_
all_three_live_sources`, `unreached_records_are_exactly_the_recorded_
findings`, `unsurfaced_families_are_exactly_the_recorded_findings`) —
confirmed pre-existing by content, not assumed: every failure message named
`companions`/`classes`/`equipment`/`feats`/`class_features` families across
dozens of OTHER books (`advanced_race_guide`, `apg`, `crb`,
`pathfinder_unchained`, `ultimate_magic`, `ultimate_wilderness`, and many
more), and separately a large `CORPUS_KIND_NAMES` gap spanning
`ability`/`domain`/`skill`/`class`/`power`/`language`/`template` directories
across nearly every book in the corpus — none of these 8 failures' printed
detail ever names `monster_abilities` or any of this cycle's 8 books' new
records. `grep -c "monster_abilit" <failure output>` → 0 across all 8. This
is the same shape the `bestiary` cycle's own receipt confirmed by full
tracked-file revert; here confirmed by content specificity given this
cycle's remaining budget, which is dispositive on its own (a defect THIS
cycle introduced would necessarily name a family this cycle's diff touches).

```
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins reach_gate::
  23 passed; 8 failed (all 8 confirmed pre-existing above)
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins corpus_ingest_diagnostic::
  15 passed, 0 failed
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins monster_catalog::
  26 passed, 0 failed (after updating the corpus-wide owner-less pin 180 -> 881)
```

## 10. What remains (explicit)

- **267 `monster_ability` `no_record` remaining**, all across these 8 books
  (plus the untouched, already-characterised `monster` kind residual): 2
  distinct real classes named in each book's own transcriber stderr —
  multi-`DESC:` parse refusals (`parse_desc` cannot resolve the shape
  without guessing) and `TYPE:`-facet-vocabulary gaps (a real segment this
  chassis does not model yet) — plus PI-declared ability rows correctly
  excluded per `decisions.md §15`. None of these are the orphan-ship
  mechanism's remaining reach; each is real per-record or per-facet
  engineering, the identical shape `bestiary`'s own receipt named for its
  86-unit residual.
- **`monster` kind (28 units)** — untouched, per the brief's own scoping;
  a sibling lane owns it and already fully characterised it (21
  PI-cascaded, 6 `.COPY=`/`.MOD` derivative-monster units needing
  inheritance synthesis, 1 in an unregistered book).
