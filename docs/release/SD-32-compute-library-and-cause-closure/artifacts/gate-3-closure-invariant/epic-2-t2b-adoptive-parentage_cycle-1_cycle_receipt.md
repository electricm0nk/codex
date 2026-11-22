# Cycle epic-2-t2b-adoptive-parentage/1 — Gate 3 closure invariant / Epic 2, shape T2b, "Adoptive Parentage" / "Adopted Race" selector (`decisions.md §16` item 2)

- **Card ID:** `epic-2-cause-closure` (row 11)
- **Actor:** `t2b-adoptive-parentage`
- **Base:** `e2bbff32ca328fa3a0a76f0286b2f479f1ae0bc2` (pinned `PIN`); worktree was cut from a stray
  `site-publish` merge commit with no `docs/`/`data/`/`scripts/` tree (footgun 1, fired again) —
  `git reset --hard "$PIN"` then re-verified before doing anything else.
- **Commit SHA:** `55981abc6` (feature), `ac35f6bff` (retro-log append),
  `717db44f7` (this receipt + kanban/progress.md), all on `origin/tranche/12`.
- **Files touched:**
  - `src/bin/ingest_race_traits.rs` — `parse_row` gains a third row shape (Adoptive Parentage,
    no `TYPE:` token) alongside standard/alternate and heritage-selector; new fixture tests.
  - `src/rules_core/race_resolver.rs` — `RaceCorpus::traits_by_category`, `adoptive_parentage_options`,
    `AdoptiveParentageOption`/`AdoptiveParentageGrant`, `ADOPTIVE_PARENTAGE_CATEGORY`; updated the
    four pinned corpus-wide census tests (`no_corpus_trait_is_left_without_a_readable_gate`,
    `the_whole_corpus_classifies_into_the_four_roles_with_no_leftovers`) for the +7 records; new test
    `adoptive_parentage_resolves_all_seven_arg_options_to_a_modelled_race_with_real_grants`.
  - `apps/desktop/src-tauri/src/race_trait_picker.rs` — `AdoptiveParentageOptionDto`/
    `AdoptiveParentageGrantDto`, wired into `AlternateRacialTraitsResponse`/`build_menu`/
    `menu_or_error`; new test.
  - `apps/desktop/src-tauri/src/reach_gate.rs` — `race_traits_reach` now also asks the real
    `list_alternate_racial_traits` command for `adoptive_parentage_options`; updated the pinned
    414→421 ARG reach-gate assertion.
  - `tests/sd27_alternate_racial_trait_reachability.rs`, `tests/v06_work_inventory.rs`,
    `src/bin/ingest_apg_race_traits.rs`, `apps/desktop/src/characterHub/raceCreationCoverage.test.ts`
    — every other pinned corpus-wide count (414/824/589) that a record-count change compiles clean
    against and silently leaves red (per this bundle's own standing lesson), swept and updated with
    the 7-record addition named in each doc string.
  - `data/corpus/advanced_race_guide/race_trait/{dwarf,elf,gnome,halfling,orc,drow,grippli}/{dwarf,elf,gnome,halfling,orc,drow,grippli}.json`
    — 7 new corpus records (regenerated via `cargo run --bin ingest_race_traits -- advanced_race_guide`
    against the pinned oracle; the rest of the 421-record `advanced_race_guide/race_trait` tree is a
    byte-identical rebuild, confirmed by the pinned-count tests passing unchanged elsewhere).
  - `scripts/t2b_adoptive_parentage_census.py` — new, committed, re-derivable census + corpus-wide
    real-vs-empty proof script (see §"Re-derived population" below).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — one inspected false positive: the
  `diff --git a/tests/sd27_alternate_racial_trait_reachability.rs ...` header line matches
  `sd[0-9]+_` on the PRE-EXISTING FILENAME (a file this cycle edited, did not create; not a bundle
  tag inside content). No actual identifier leak in the diff body.
- **Wired-integration audit result:** `OK_NO_TOKENS` — one inspected false positive:
  `reach_gate.rs:3983`'s pre-existing doc comment `` `SD-31-corpus-closure-grind/todo/sweeps.md` ``
  (committed `1850277174`, 2026-08-22, predates this cycle) matches `\btodo\b` on the path segment
  `todo/`, not a real TODO marker; confirmed via `git blame` it is outside this cycle's own diff —
  it is only visible because `BASE_BRANCH = merge-base(HEAD, origin/develop)` is far behind
  `tranche/12`'s tip, so the whole file's prior-cycle content is in scope of the literal command.
- **Acceptance criterion (verbatim):** AT-32-E2-001 — T2b closed corpus-wide, by class.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`)
- **Status:** complete for this cycle's granted scope (build the Adoptive Parentage selector once);
  T2b as a whole remains open — see "What is NOT closed" below.

## 1. Re-derived population, by class, corpus-wide

Command: `python3 scripts/t2b_adoptive_parentage_census.py` (requires `PCGEN_CORPUS_ROOT` pointed at
the repo-local pinned oracle for the second half).

```
total population: 21  (expect 21)
  advanced_race_guide: 7
  bestiary_2: 7
  bestiary_3: 5
  bestiary_5: 1
  bestiary_6: 1

shape 'adopted_race_choose_selector' (CHOOSE:ABILITYSELECTION pool): 14
shape 'arg_flat_grant' (ABILITY:...AUTOMATIC flat grant): 7
```

Matches the dispatch brief's 21 exactly, split into **two structurally different PCGen row shapes**
the brief's own framing (one "Adoptive Parentage / Adopted Race selector") did not distinguish:

1. **`arg_flat_grant` (7 units, `advanced_race_guide`, `arg_abilities_race.lst:291-297`,
   `###Block: Adoptive Parentage Options`).** Each row is `<Race>\tCATEGORY:Adoptive Parentage\t
   DESC:...\tABILITY:<Race> Racial Trait|AUTOMATIC|<Race> ~ Weapon Familiarity|<Race> ~ Languages\t
   SOURCEPAGE:p.72` — a flat grant of exactly two already-modelled traits, no `CHOOSE` at all. These
   7 rows are the **CHOOSE pool** for a DIFFERENT, already-ingested ARG alternate trait,
   `Human ~ Adoptive Parentage` (`arg_abilities_race.lst:257`, `CHOOSE:ABILITYSELECTION|Adoptive
   Parentage|ANY`, `ABILITY:Adoptive Parentage|AUTOMATIC|%LIST`, `TraitRole::Alternate`,
   `sets_replace_flags: [Human_ReplaceBonusFeat]`) — confirmed by direct read of the pinned oracle,
   not assumed. **Only a Human character who has taken that alternate trait can pick one of these
   seven**; they are not "available to any race", correcting this cycle's own first-draft doc
   comments (fixed before commit, not shipped).
2. **`adopted_race_choose_selector` (14 units, `bestiary_2` 7 / `bestiary_3` 5 / `bestiary_5` 1 /
   `bestiary_6` 1, `KEY:Adopted Race ~ <X>` rows).** Each is
   `CHOOSE:ABILITYSELECTION|Special Ability|TYPE=<X> Race Trait` — a genuine selector-picker
   mechanic, structurally different from shape 1 (no flat grant list; the pool is discovered by
   TYPE, not named).

## 2. The correction to wave 1's finding — proven per row, corpus-wide, not by analogy

Two wave-1 receipts (`epic-2-t2b-bestiary2_cycle-1_cycle_receipt.md`,
`epic-2-t2b-bestiary6_cycle-1_cycle_receipt.md`) concluded **all 8** `Adopted Race ~ <X>` rows in
`bestiary_2`/`bestiary_6` are "the identical browse-only-stub shape" as Rougarou, by grepping each
row's own file only (`TYPE=<X> Race Trait` found nowhere else *in that file*). This cycle's guard
rail requires proving that per row, corpus-wide, not by analogy — re-derived:

```
python3 scripts/t2b_adoptive_parentage_census.py   # (corpus-wide half, PCGEN_CORPUS_ROOT set)
```

```
  bestiary_2  Dhampir     5 file(s) -- REAL CONTENT
  bestiary_2  Fetchling   4 file(s) -- REAL CONTENT
  bestiary_2  Grippli     2 file(s) -- REAL CONTENT
  bestiary_2  Ifrit       5 file(s) -- REAL CONTENT
  bestiary_2  Oread       5 file(s) -- REAL CONTENT
  bestiary_2  Sylph       5 file(s) -- REAL CONTENT
  bestiary_2  Undine      5 file(s) -- REAL CONTENT
  bestiary_3  Catfolk     3 file(s) -- REAL CONTENT
  bestiary_3  Ratfolk     3 file(s) -- REAL CONTENT
  bestiary_3  Suli        5 file(s) -- REAL CONTENT
  bestiary_3  Vanara      2 file(s) -- REAL CONTENT
  bestiary_3  Vishkanya   2 file(s) -- REAL CONTENT
  bestiary_5  Skinwalker  4 file(s) -- REAL CONTENT
  bestiary_6  Rougarou    1 file(s) -- PROVEN EMPTY (only its own file)
```

**Correction: 13 of the 14 `adopted_race_choose_selector` rows are NOT browse-only stubs.**
Corpus-wide, the `TYPE=<X> Race Trait` pool has real content elsewhere in the pinned oracle — mostly
PF1e "Trait" (character-creation trait, a mechanic distinct from racial traits) records in
`player_companion/` books this project has never onboarded (e.g. `Oread ~ Loner of the Rocks` in
`inner_sea_races/isr_abilities.lst:78`; `Sandy Ambush`/`Earthsense`/`Statuesque`/`Stoic Dignity` in
`people_of_the_sands`/`blood_of_the_elements`/`bastards_of_golarion`, none of which are registered
books). Worked example, `Oread`:

```
grep -n "Oread Race Trait" .../core_essentials/races/oread/oread_abilities_race.lst
# 30: ...CHOOSE:ABILITYSELECTION|Special Ability|TYPE=Oread Race Trait...
# 31: CATEGORY=Special Ability|No Race Trait Available.MOD  TYPE:Oread Race Trait
grep -rln "Oread Race Trait" <pinned oracle root>
# oread_abilities_race.lst (the placeholder, above) PLUS isr_abilities.lst,
# psand_abilities.lst, bote_abilities.lst, bog_abilities.lst
```

`isr_abilities.lst`'s `Trait ~ Loner of the Rocks` (`TYPE:Trait.RaceTrait.Oread Race Trait`,
`PREFACT:1,TEMPLATES,IsOread=true`) and `bog_abilities.lst`'s `Trait ~ Stoic Dignity`
(`PREMULT:1,[PREFACT:1,TEMPLATES,IsOread=true],[PREABILITY:1,CATEGORY=Special Ability,Adoptive
Race ~ Oread]`) are real, would-be-selectable content once ingested — the second literally names
`Adoptive Race ~ Oread` as an alternate prerequisite, confirming the mechanic is real and this
project's own selector row is the gate PCGen's design intends.

**Rougarou remains genuinely proven empty** — 1 file corpus-wide (itself), matching
`ingest_races.rs`'s own prior finding exactly (`grep -rn Rougarou_Replace` returns hits only in its
own file too). The existing precedent (no corpus record ingested for it) is correct and unchanged.

`scripts/retro.py correction` logged against `epic-2-t2b-bestiary2_cycle-1_cycle_receipt.md`'s and
`epic-2-t2b-bestiary6_cycle-1_cycle_receipt.md`'s "identical browse-only-stub shape" claim
(`--verified-by 'python3 scripts/t2b_adoptive_parentage_census.py'`).

## 3. What this cycle closes, and what it does NOT

**Closed: the 7 `arg_flat_grant` units.** `ingest_race_traits.rs`'s `parse_row` now recognizes the
Adoptive Parentage row shape (no `TYPE:` at all, `CATEGORY:Adoptive Parentage`, race key = the row's
own display name) and ingests it; `race_resolver::adoptive_parentage_options` resolves each one's two
grant targets against this project's own already-ingested standard traits
(`<Race> ~ Weapon Familiarity`, `<Race> ~ Languages`) — real content, not fabricated, proven by test
(`adoptive_parentage_resolves_all_seven_arg_options_to_a_modelled_race_with_real_grants`:
`unresolved_grants` is empty for all 7). This is `decisions.md §16` item 2's literal ask — "resolves
the selector to the race it adopts" — satisfied for every unit where the resolution is real.

**NOT closed: the 14 `adopted_race_choose_selector` units** (13 real-content-but-not-ingestable, 1
proven empty). Ingesting the 13 real ones would require modelling PF1e's chargen "Trait" mechanic as
a new content kind this project has never built (no `kind: trait` exists anywhere in
`data/corpus/`), plus onboarding several unregistered `player_companion` books to have anything for
the pool to resolve against — a new-kind epic, not "the selector, once". Per `decisions.md §1a`/§3,
fabricating a picker over content this corpus does not carry would manufacture false coverage; this
cycle refuses to. **Escalated, not silently deferred** (`AGENTS.md` Blocker Discipline disposition
2): the fix site is a new `kind: trait` ingest surface plus `player_companion` book onboarding,
named here for whichever cycle is granted that scope. Rougarou's 1 unit stays excluded (proven, not
assumed).

**T2b as a whole is therefore still open.** This cycle closes exactly the sub-population
`decisions.md §16` item 2 named as buildable now; the residual (14 units here, plus the much larger
per-book piles the classifier-fix cycle's own re-measurement will define) is real, honestly-stated,
un-fabricated work for a follow-on cycle.

## 4. Proof: reachability, not just ingestion

`apps/desktop/src-tauri/src/reach_gate.rs`'s `race_traits_reach` now also queries
`list_alternate_racial_traits`'s new `adoptive_parentage_options` field, marking a key `with_payload`
only when it resolved at least one real grant (never a bare identity). Executed live —
`scripts/verify.sh --only reach` (`cargo test --locked reach_gate`, desktop crate) — and the pinned
`args_alternate_racial_traits_are_visible_only_because_the_corpus_is_scanned` test asserts
`Reach::Surfaced { records: 421 }` for the whole `advanced_race_guide` family, including the 7 new
keys.

## 5. RED → GREEN evidence

Three independent mutations, each reverted after confirming the intended failure:

1. **Ingest (`ingest_race_traits.rs::parse_row`).** Reverted the `is_adoptive_parentage_option`
   branch to `None => return None` (dropping the row, as before this cycle). New fixture test
   `an_adoptive_parentage_row_resolves_to_the_race_it_adopts_despite_carrying_no_type` failed:
   `"Adoptive Parentage row is not dropped"`. Restored — GREEN.
2. **Resolver (`race_resolver.rs::adoptive_parentage_options`).** Ran the new test
   `adoptive_parentage_resolves_all_seven_arg_options_to_a_modelled_race_with_real_grants` against
   the corpus *before* re-running the ingest tool (function existed, data did not): `left: [] right:
   [7 keys]`. GREEN only after `cargo run --bin ingest_race_traits -- advanced_race_guide`.
3. **Reach (`reach_gate.rs::race_traits_reach`).** Neutered the new `adoptive_parentage_options` loop
   with `.filter(|_option| false)`. `args_alternate_racial_traits_are_visible_only_because_the_corpus_is_scanned`
   failed: `NotSurfaced { why: "7 of 421 ingested records never appear in
   \`list_alternate_racial_traits + resolve_race_alternate_selection\` (e.g. Drow, Dwarf, Elf,
   Gnome)", missing: {"Drow","Dwarf","Elf","Gnome","Grippli","Halfling","Orc"} }`. Restored — GREEN.

## 6. Fixture / audit discipline

- No formula interpreter engaged (`decisions.md §24` unaffected) — the two grants per row are a
  literal, transcribed `ABILITY:...AUTOMATIC|<key>|<key>` token, read the same way
  `RaceTraitRecord::automatic_trait_grants()` already reads every other book's identical shape.
- PI screen ran as part of the normal `ingest_race_traits.rs` pipeline (declared-PI scan +
  57-term scan) on all 7 new rows: zero hits (`DESC:You were adopted and raised by <race>s.` names
  no Product Identity term).
- No new interpreted magnitude, so `derived_evaluator_fixture_check` gains no new fixture
  obligation; the 7 new records are `wiring_class: "display"` (no magnitude token), matching every
  other zero-magnitude selector-shaped record already in this corpus.

## 7. Verification run (this cycle)

```
cargo test --locked --lib                       # 2390 passed, 0 failed, 13 ignored
cargo test --locked --bin ingest_race_traits     # 16 passed, 0 failed
cargo test --locked --bin ingest_apg_race_traits # 8 passed, 0 failed
cargo test --locked --test sd27_alternate_racial_trait_reachability   # 15 passed, 0 failed
cargo test --locked --test v06_work_inventory    # 16 passed, 1 failed (pre-existing, unrelated —
                                                  #   see §8)
(cd apps/desktop/src-tauri && cargo build --locked)   # clean
(cd apps/desktop/src-tauri && cargo test --locked)    # 518 passed, 0 failed (was 517 before this
                                                       #   cycle's new picker test)
scripts/verify.sh --only reach                   # RESULT: PASS (31 passed)
```

## 8. One pre-existing, unrelated failure found and NOT touched

`tests/v06_work_inventory.rs::sd30_campaign_setting_books_appear_in_the_inventory_as_not_started_books`
fails at the tip this cycle rebased onto (`d904eceb6`, the concurrent classifier/card-15 lane's own
landed commit) — `inner_sea_faiths` registered `in_scope` where the test expects `future_state`.
Confirmed pre-existing and out of this cycle's scope: `diff` of this cycle's own edit against
`git show HEAD:tests/v06_work_inventory.rs` touches only the two `arg_race_file_carries_...` lines
named in the file-touch list above; nothing here touches `docs/work-inventory.json` or
`data/stubs/inner_sea_faiths.json`. Matches the dispatch brief's own coordination note (the sibling
classifier lane owns `v06_work_inventory.rs`/the census). Not self-healed, not silently ignored —
named here so the next rebase onto this branch does not mistake it for this cycle's regression.

## 9. Discovery forwards

- `## DISCOVERED`: the "Adoptive Parentage" mechanism is actually **two structurally different
  PCGen row shapes**, not one — see §1. Any future cycle scoping "the rest of T2b's Adoptive
  Parentage work" should scope against the 14-unit `adopted_race_choose_selector` population (13
  real, 1 proven-empty), not the original 21, since the 7 `arg_flat_grant` units are closed here.
- `## DISCOVERED`: closing the 13 real `adopted_race_choose_selector` units needs (a) a new
  `kind: trait` content surface (PF1e chargen Traits have never been modelled in this corpus) and
  (b) onboarding the `player_companion` books that carry each race's actual trait pool
  (`inner_sea_races` is registered and partially covers some; `people_of_the_sands`,
  `blood_of_the_elements`, `bastards_of_golarion`, `agents_of_evil`, `blood_of_the_night` are not
  registered at all). This is a new-kind epic, not a T2b-shaped ingest-tool extension — named for an
  operator ruling on scope, per `AGENTS.md` Blocker Discipline disposition 2.

## 10. Next-cycle plan

None on this cycle's own granted scope (the selector mechanism is built, correctly bounded to what
it can honestly resolve). The 14-unit residual and the two named prerequisites above are the next
cycle's starting point if the operator grants the new-kind scope; otherwise they stay named,
re-derivable T2b residual for the classifier-fix cycle's own re-measurement
(`decisions.md §16`'s step 3, "Then re-measure T2b").
