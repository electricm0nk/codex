# Cycle t9-monster-ability-per-record-refusal-groups-round6 — Gate 3 (closure invariant) / Card 11 (T9)

- **Card ID:** 11 (`epic-2-cause-closure`, T9 sub-lane
  `t9-monster-ability-per-record-refusal-groups-round6`)
- **Files touched:**
  - `scripts/transcribe_monster_tables.py` — `type_segments()` now splits on
    `,` in addition to `.`, and applies a small, named, exact-match
    substitution table (`_TYPE_SEGMENT_TYPO_FOLDS`) for two confirmed
    upstream spelling defects, before facet/delivery classification.
  - `scripts/tests/test_transcribe_monster_tables.py` — new
    `TypeSegmentsUpstreamDivergenceCorrection` test class (6 tests): the
    comma-split, both typo folds, an end-to-end `parse_type` proof for both
    real rows, and a negative-control test proving a genuinely unmodelled
    book-specific `TYPE:` string is unaffected (not a fuzzy match).
  - `src/rules_core/rules_tables/bestiary/monster_data.rs`,
    `src/rules_core/rules_tables/bestiary_2/monster_data.rs` — regenerated
    via `transcribe_monster_tables.py`; each book's refusal-header comment
    now names one fewer TYPE-facet-gap row.
  - `src/rules_core/rules_tables/monster_chassis.rs` — the corpus-wide
    no-reclassification pin (`widening_the_facet_vocabulary_does_not_
    reclassify_any_existing_record`) re-derived from a live failing run,
    never guessed: 3704 → 3706 records, digest `0xd732c20ec4c2a946` →
    `0x38f4aedd6de1caf3`.
  - `data/corpus/beastiary/monster_ability/spectre_create_spawn.json`,
    `data/corpus/bestiary_2/monster_ability/tick_swarm_cling.json` (new,
    via `cargo run --bin gen_book_cache -- beastiary` /
    `-- bestiary_2`) and each book's `LICENSE.json` (screening-note append,
    same generator-owned mechanism every prior round used).
  - `apps/desktop/src-tauri/src/reach_gate.rs` — two pinned on-disk-count
    assertions this cycle's own diff moved
    (`bestiary_1_monsters_reach_the_monster_catalog_record_by_record`:
    709→710; `bestiary_2_reaches_the_catalog_for_every_linked_record`:
    656→657, owned_abilities 571→572), both new records OWNED (not
    owner-less), so `missing`/`owner_less` counts are unchanged.
  - `apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs` — **one line**,
    `beastiary1_monster_count_matches_the_documented_real_total`'s pin
    (709→710). This file is a sibling lane's named territory for its own
    **pre-existing** red tests (`the_two_ingested_books_totals_reconcile_
    with_their_license_artifacts`, `every_book_landed_in_rules_tables_is_
    reported`) — both left **untouched**, still red, exactly as round 5's
    receipt found them (neither depends on this cycle's diff: verified by
    reading each failure's own printed detail, neither names `bestiary`,
    `beastiary`, `bestiary_2`, `Spectre`, or `Tick Swarm`). The one line
    changed here is a NEW failure this cycle's own diff caused (adding a
    real on-disk record moved a pinned count), not the sibling lane's
    investigation — leaving it red would have left the branch red from this
    cycle's own change, which `decisions.md`'s cross-file-pin-sweep
    requirement forbids.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` with one explained,
  pre-existing, non-diff finding: `git diff --unified=0
  df03f7099af5197add0594907571e4652559efb7 -- <this cycle's files>` matches
  `SD28-E16`/`SD29-E7-F2-004`/`SD29-E5-F2-009` inside `beastiary/
  LICENSE.json`'s `screening_method_note` — these are `classified_by_cycle`
  provenance labels **hard-coded in `gen_book_cache.rs`'s pre-existing,
  untouched `CompanionBookSpec`/`MonsterBookSpec` rows** (confirmed:
  `grep -n "SD29-E5-F2-009" src/bin/gen_book_cache.rs` → line 2474, a
  `classified_by_cycle` field this cycle did not edit), emitted into the
  note the way every prior PASS line in that same field already was before
  this cycle ran. Not a bundle-tag this cycle's diff introduces into
  shipping code — the generator's designed provenance-tracking behaviour,
  pre-dating this cycle.
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scoped diff, 0
  hits).
- **Acceptance criterion:** `decisions.md §20` — drive `monster_ability`
  `no_record` toward zero, worked by refusal-reason group per this brief's
  `§17` grouping instruction, `decisions.md §22` (upstream data bugs
  resolved, not perpetuated, with the divergence recorded).
- **Corpus SHA:** `PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`; bootstrapped from empty this cycle via
  `scripts/fetch-pcgen-oracle.sh`).
- **Status:** complete (partial application of the overall `no_record==0`
  goal — card 11 stays `in-progress`; see "What remains" below).
- **Notes:** see full body below.
- **Discovery forwards:** none new; the two `corpus_ingest_diagnostic.rs`
  failures named above are the identical ones round 5's receipt already
  forwarded, re-confirmed present and re-confirmed out of this cycle's
  scope.
- **Next-cycle plan:** see §7 below.

---

## 1. Re-derived the 100, grouped by refusal reason (this brief's own `§17` instruction)

Never trusted the brief's own figures without re-deriving (`decisions.md
§17a`). `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json`
confirmed **100** `monster_ability` `no_record` units, split exactly as the
brief stated across 11 books (`occult_adventures`' 5 excluded as out of
scope, re-verified below). Then, for the 10 in-scope books, ran
`python3 scripts/transcribe_monster_tables.py <book>` for each (the actual
generator, not a synthetic re-derivation) and cross-referenced every
`no_record` unit's slug against the transcriber's own three named-refusal
stderr lists (parse_desc / TYPE-facet / PI-drop), by exact `(book, slug)`
match against `docs/work-inventory.json`'s corpus keys:

| Group | Units | Shape |
|---|---:|---|
| Multi-`DESC:` parse refusals | 56 | `parse_desc` cannot resolve the row's several `DESC:` tokens without guessing |
| `TYPE:`-facet-vocabulary gaps | 24 | row's `TYPE:` segments name no facet this chassis models |
| PI-declared exclusions | 15 | term-blacklist hit in the row's own emitted values (dropped, not shipped) |
| (out of scope) `occult_adventures` | 5 | negated `PRECAMPAIGN` gate, re-verified §5 below |
| **Total** | **100** | |

Every group total is directly re-derivable: `python3
/path/to/scratchpad/run_transcribe2.py` (cross-references the ledger's own
`no_record` ids against each book's transcriber stderr, per book, printed
per-group with the 15 PI-dropped units showing as the script's own
"unaccounted" bucket — its regex for the PI-drop stderr shape did not match,
but manual inspection of all 15 confirmed every one is a `PI_BLACKLIST_
TERMS hit(s) in emitted values` drop, not a third distinct shape).

## 2. What was actually closed this cycle: 2 units, by a generic mechanism, not per-object work

Within the 24-unit TYPE-facet-gap group, direct inspection of every
refused row's raw `TYPE:` field (`scripts.transcribe_monster_tables.
type_segments`, called against each row read live from the pinned oracle)
found the population is NOT 24 flavours of "needs a new facet word" — most
are genuinely book-specific, non-generic labels (`AsurendraAdditional`,
`LunarNagaRacialAbility`, `Unfettered Eidolon Stat Selection`, ×6, …) that
would need a real per-record policy call to assign a facet, exactly the
per-object work `decisions.md §17` forbids doing casually. But **two** rows
carry a different, generic shape: **confirmed upstream data defects**
(`decisions.md §22`), not vocabulary gaps at all:

1. `bestiary`'s `b1_abilities_race.lst:1138` (`Spectre ~ Create Spawn`) —
   `TYPE:SpecialAttack,Supernatural`. PCGen's own delimiter is `.`
   everywhere else this script has ever read; this one row uses `,`.
2. `bestiary_2`'s `b2_abilities_race.lst:1259` (`Tick Swarm ~ Cling`) —
   `TYPE:SpecialAttck.Extraordinary` (missing the `a`).

Both are the exact shape `decisions.md §22` already named without a fix
landed ("a monster-ability cycle found 2 corpus typos and a
comma-delimiter anomaly among its 86 unmodelled units") — re-derived live
against the round-6 refusal population, not assumed from that stale
reference. A third row, `bestiary_2`'s `b2_abilities_race.lst:851`
(`Mothman ~ Agent of Fate`, `TYPE:Spelllike`), carries the same misspelling
shape (`Spelllike` vs. `SpellLike`) and is corrected by the same named fold
for vocabulary correctness, but does NOT close — `SpellLike` is a
*delivery*, not a *facet*, and the row still carries no facet segment at
all, so it correctly still raises `UnmodelledFacet` (proven by
`test_a_genuinely_unmodelled_dotted_segment_is_unaffected`'s sibling
assertion pattern — not directly tested for this exact row, but the same
`parse_type` code path; re-confirmed live: `Mothman ~ Agent of Fate` is
absent from both books' post-fix `TYPE:-facet-gap` stderr shrinkage math
below, 9→8 not 9→7).

**§22's binding condition — divergence recorded, not silently mirrored:**
both corrections are named, single-row, exact-match substitutions in
`_TYPE_SEGMENT_TYPO_FOLDS`, each with a comment naming the exact row it was
found on; `type_segments()`'s own docstring states both shapes and why
Codex resolves rather than perpetuates them. Neither is a fuzzy/heuristic
match — `test_a_genuinely_unmodelled_dotted_segment_is_unaffected` proves a
real book-specific label (`Unfettered Eidolon Stat Selection`) passes
through byte-unchanged and still fails classification.

## 3. RED → GREEN (`AGENTS.md` non-negotiable rule 1)

`python3 -m unittest scripts.tests.test_transcribe_monster_tables.
TypeSegmentsUpstreamDivergenceCorrection` against the **pre-fix** module
(temporarily restored via `git show HEAD:scripts/transcribe_monster_tables.
py` into the tracked file, never `git stash`, then restored from a
scratchpad copy afterward — `git status --porcelain` confirmed clean
restoration): 3 failures, 2 errors — `SpecialAttck`/`Spelllike` unfolded,
comma not split, `parse_type` still raising `UnmodelledFacet` for the
comma row. Against the fix: all 6 green. Full module suite before/after:
identical 17/18 (one pre-existing, unrelated failure in
`InternalBundleAbilityHopIsResolved`, confirmed present against the
unmodified `HEAD` copy too, named in §6 below, not this cycle's territory
or diff).

## 4. Corpus regeneration — no unexpected deletions, no stamp loss

`git status --porcelain` before every commit: only the files named above,
**zero deletions** (`git status --porcelain | grep '^ D\|^D '` → empty),
matching this cycle's own explicit closed-file list. No
`--allow-stamp-loss` used; `CORPUS_LITERAL_SWEEP_REPORT`/
`DERIVED_FIXTURE_CHECK_REPORT` set for both `gen_book_cache` runs (env vars
present, no destructive full-corpus regen attempted — only the two
generator commands named above, each additive-only per its own printed
report: `0 new monsters ... 709/656 already on disk, left untouched, 1 new
monster ability`).

## 5. `occult_adventures` (5 units) re-confirmed out of scope

`grep -n 'PRECAMPAIGN' _occult_adventures.pcc` under the pinned oracle
still shows the negated `PRECAMPAIGN` gate this repo's registered campaign
set fails — unchanged from rounds 3/4/5's own finding. Left untouched;
reasoning still holds.

## 6. Tests

```
python3 -m unittest scripts.tests.test_transcribe_monster_tables
  18 tests, 17 passed, 1 failed (pre-existing, unrelated: InternalBundleAbilityHopIsResolved
  ::test_an_ability_no_bundle_names_stays_an_orphan_and_is_not_shipped — confirmed present
  against the unmodified module too, not this cycle's diff or territory)
cargo build --locked --lib                                    clean, 9 warnings (pre-existing shape)
cargo test --locked --lib monster_chassis::                   8 passed, 0 failed (pin re-derived: 3706 / 0x38f4aedd6de1caf3)
cargo build --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins   clean
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins monster_catalog::
  26 passed, 0 failed (unchanged from round 5's own baseline)
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins reach_gate::
  23 passed, 8 failed — IDENTICAL split to round 4/5's own recorded baseline (re-verified: none
  of the 8 remaining failures names `beastiary`, `bestiary`, `bestiary_2`, `Spectre`, or `Tick
  Swarm`, confirmed by grepping each failing test's own output). Two book-specific pins this
  cycle's diff itself moved (709->710, 656->657/571->572) were fixed inline before this run.
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins corpus_ingest_diagnostic::
  13 passed, 2 failed — IDENTICAL split to round 5's own recorded baseline, after fixing the
  ONE new failure this cycle's own diff caused (709->710 pin). The 2 remaining are the sibling
  lane's own named, pre-existing, untouched territory.
```

RED→GREEN evidence: §3 above (script-level), plus `monster_chassis::
widening_the_facet_vocabulary_does_not_reclassify_any_existing_record`
failed twice live on its own re-derivation path (first on `triples.len()`,
3704 vs. real 3706; then, after bumping the length assertion to reveal the
real digest, on the digest itself, `4104097426495884019` vs. the stale
pin) before being repinned from the live failure's own printed values,
never guessed — the same discipline round 5's receipt used for its own
repin.

## 7. What remains (three separate figures per `decisions.md §16`)

**Closure this cycle: 2 units, real ingestion (`Spectre ~ Create Spawn`,
`Tick Swarm ~ Cling`), 0 reclassified.** `monster_ability` `no_record`:
100 → **98** (re-derived: `python3 scripts/shape_ledger.py --inventory
docs/work-inventory.json` → `monster_ability 98` in the join-status-by-kind
breakdown). Bundle-wide `no_record`: 227 → 225.

Remaining **98**, by group (re-derive: re-run `run_transcribe2.py`
against the post-fix module):

1. **Multi-`DESC:` parse refusals — 56 units, unchanged.** Real shapes
   found live: `PRERULE`/`PREVAREQ`-gated variant-text rows (singular vs.
   plural phrasing keyed to a `BONUS:VAR` value the row itself sets) that
   `parse_desc`'s own docstring already names as a fifth, deliberately
   still-refused shape — resolving these needs each row's own variable
   value traced, not a generic parse widening.
2. **`TYPE:`-facet-vocabulary gaps — 22 units** (24 − 2 closed this
   cycle). Real shapes found live, none of them a further generic
   correction: book-specific one-off `TYPE:` strings with no dot segments
   at all (`AsurendraAdditional`, `*RacialAbility`, `Unfettered Eidolon
   Stat Selection` ×6, `PetrifiedMaidenWeaponSelection`) — 11 units;
   delivery-only `TYPE:` with no facet segment present at all
   (`SpellLike` alone, `ModifyHP.Supernatural`) — needs an operator-level
   default-facet ruling, not invented here per `§1a`; and one `TYPE:
   Internal`-only row (`Morlock ~ Sneak Attack`, `VISIBLE:NO`, a hidden
   internal bonus-granter, not a player-facing ability) that is a genuinely
   novel shape.
3. **PI-declared exclusions — 15 units.** Live inspection this cycle
   (needed for accurate grouping, not for remediation) found this is
   **two** sub-shapes, not one: **13** are *name*-embedded PI (`Spawn of
   Rovagug ~ *` ×10 across `inner_sea_bestiary`/`inner_sea_gods`,
   `Daughter of Urgathoa ~ *` ×3 in `inner_sea_world_guide` — both
   "Rovagug" and "Urgathoa" are on `pi_screening.rs`'s own term list,
   confirmed by direct grep) and squarely `decisions.md §24`'s "name itself
   is PI" case — closes by importing `codex_neutral_name`/`neutral_key`/
   `divergence_entry` (already used by `ingest_ability.py`/`ingest_generic_
   kind.py`; NOT re-implemented here) into `transcribe_monster_tables.py`'s
   PI-drop branch. **2** are *description*-only PI (`inner_sea_gods`'s
   `Thyrlien ~ Starlight Blast` mentions "Desna" in its `DESC:` prose,
   `Grim White Stag ~ Bugle` mentions "Erastil" — both names are clean,
   verified against the term list) and close by the SAME redact-and-ship
   path this script already applies to `DESCISPI:YES`-declared rows
   (`bestiary_4`'s 65), extended to also redact a term-list hit found by
   scanning rather than only a formal declaration.

**Not attempted this cycle**, named rather than silently dropped, per
`decisions.md §15`/`§19d`'s standing rule that a cycle reaching a suspected
PI record stops and reports it: the §24 neutral-name wiring for the 13
name-PI `monster_ability` units is real, multi-file engineering
(`transcribe_monster_tables.py`'s ability-emission pass, `gen_book_cache.rs`
or the equivalent monster generator, a new `MonsterAbilityRecord`
provenance field mirroring `ingest_ability.py`'s `codex_generated_name`/
`rename` fields) that this cycle's remaining scope could not respectably
rush through TDD, fixture-proof, and the §24b-6 determinism proof in the
time remaining. Landing 2 units correctly, verified end-to-end, is worth
more than landing 15 units unverified.

## 8. Next-cycle plan

1. **PI-declared group (15 units, highest-value next target — closes 13
   outright, redacts 2).** Wire `codex_neutral_name` into
   `transcribe_monster_tables.py`'s `ability_pi_reason`/emission path for
   the 13 name-PI rows (mirror `ingest_ability.py`'s branch structure
   exactly — import, never re-implement); extend the existing
   `DESCISPI:YES` redact-and-ship path to also fire on a term-list hit
   confined to the description field for the 2 description-only rows.
2. **TYPE-facet delivery-only default (2+ units)** needs an operator
   ruling on whether a bare `SpellLike`/`ModifyHP`-shaped delivery-only row
   defaults to `SpecialQuality` — do not invent this unilaterally.
3. **Multi-DESC `PREVAREQ`/`PREVARGT` shape (56 units)** needs each row's
   own `BONUS:VAR` value traced per record — real per-object work, but
   `parse_desc`'s docstring already isolates the exact predicate a
   generalised sixth branch would need, so likely still substantially
   fewer than 56 cycles.
