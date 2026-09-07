# Cycle 5 — Epic 4 (Ultimate Campaign) / AT-34-E4-002 — third trait/drawback slice: open-subtype-family `BONUS:SKILL|%LIST` traits

- **Commit SHA:** `5e3c000c8e` (source + retro log), pushed to `tranche/14`.
- **Provenance.** This cycle starts from `origin/tranche/14` at `1e6a67390e` (wave 17) — the
  dispatch prompt's stated `HEAD 651966b83e`/`DONE 151` was stale by four whole cycles (cycle 3
  landed 31 units, cycle 4 landed 5 more). Re-verified at the real HEAD before writing any code:
  `python3 scripts/completion_atlas.py --book ultimate_campaign --check` → `DONE=187, M=53 (of
  which the receipt trail names 4 open-subtype `%LIST` records as the cheapest next-cycle
  sub-cause), U=21, D=2, X=2, V=0` — matches `AT-34-E4-002_cycle_receipt_4.md`'s own stated
  end-state exactly. Whole-tree grep re-confirmed no second trait/drawback capability exists
  outside `trait_effects.rs`/`trait_picker.rs` (`grep -rniE 'selected_traits|character_traits|CharacterTrait\b' src/ apps/desktop/src-tauri/src/ apps/desktop/src/`
  shows only the existing cycle-3/4 machinery). No salvage branch existed for this criterion at
  dispatch time (`origin/salvage/wave14-lane1` referenced in the dispatch brief was not present
  on this box — checked with `git branch -a`; this cycle's build starts clean from committed
  history, not an uncommitted checkpoint).
- **Files touched:** `src/rules_core/skill_allocation.rs` (+26/-1: `skill_family_member_ids` made
  `pub(crate)` so `trait_effects.rs` can reuse the crate's existing closed Craft/Perform/Profession
  rosters directly rather than hand-duplicating a second one; folds
  `trait_effects::family_choice_bonuses_from_traits` into `allocate_skill_ranks`'s
  `trait_skill_bonuses` map, third of three now-summed maps), `src/rules_core/trait_effects.rs`
  (+375/-18: module doc comment corrected — see "Instrument/doc correction" below —
  `TraitSkillFamilyChoiceBonus`, `FAMILY_CHOICE_TRAIT_BONUSES` (4 entries), `find_family_choice_by_
  trait_id`, `family_choice_skill_options` (resolves a trait's family union live via
  `skill_family_member_ids`, no static slice-concatenation needed), `family_choice_bonuses_from_
  traits`, `family_choice_trait_magnitude_is_grounded_for_corpus_key`, 10 new tests), `src/bin/
  v06_work_inventory.rs` (+67/-22: `Kind::Trait`'s classifier gets a third `.or_else` fallback;
  repoints the existing negative-control fixture from `Trait ~ Artisan` (now covered) to `Trait ~
  Bruising Intellect` (a real, still-uncovered ability-formula shape); 1 new positive test proving
  the new fallback promotes a held record to `grounded`), `apps/desktop/src-tauri/src/trait_picker.rs`
  (+88/-15: third `.map()`/`.chain()` using the existing generic `CharacterTraitOptionDto` shape,
  `skill_options` built from `family_choice_skill_options`'s resolved union; 2 new tests, 2 existing
  tests widened to also exercise the family-choice compute path), `scripts/completion_atlas.py`
  (+1/-1: instrument-correction, the `V`-bucket citation line pin shifted by this cycle's own line
  insertions), `docs/retro/events/sd34-at-34-e4-002.jsonl` (+2: two corrections, see below).
  **`docs/work-inventory.json` and `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/
  completion-atlas.json` were regenerated locally to derive this receipt's figures, then
  `git restore`d before committing** — per this wave's `GENERATED_FILE_BAN`, this cycle does not
  own either file.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`. `git diff --unified=0 1e6a67390e -- src/rules_core/
  src/bin/ apps/desktop/src-tauri/src apps/desktop/src docs/work-inventory.json ':!**/__tests__/**'
  ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` → no match (grep
  exit 1). Base is `1e6a67390e`, this cycle's own pre-change `tranche/14` tip (not the
  `develop` merge-base, which would fold in 226 unrelated prior-cycle commits).
- **Wired-integration audit result:** `OK_NO_TOKENS` (one token). Same diff run against
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` returns exactly one
  line: a **removed** (`-`) line from the old module doc comment being corrected away
  ("...open-subtype-Craft-skill placeholder..." — PCGen's own `%LIST` terminology, the same
  doctrine-clean usage cycle 3/4's own receipts already established), not a stub introduced by
  this cycle. No `STUB`/`MOCK`/`"Would..."` string, no empty handler, in any line this cycle
  *added*.
- **Acceptance criterion (verbatim, epic-breakdown.md AT-34-E4-002):** `python3
  scripts/completion_atlas.py --book ultimate_campaign --check` exits 0 with `DONE=265 of 265`,
  every other bucket zero, plus `artifacts/epic-4-ultimate-campaign/ultimate-campaign-completion-
  manifest.json`. **Not met this cycle** — real, incremental progress: `DONE=191 of 265` (was
  `187 of 265` at this cycle's start), remainder `M:49 U:21 X:2 D:2` = 74. The completion manifest
  artifact remains out of this cycle's scope (Epic 4's closing artifact once all buckets clear).

## Figures + their re-derive commands

| Figure | Value | Command / denominator |
|---|---:|---|
| `ultimate_campaign` bucket split, re-derived at cycle start | `DONE=187, M=53 (trait_content 23 + ability_content 30, unchanged from cycle 4's own end-state), U=21, D=2, X=2, V=0` of 265 | `python3 scripts/completion_atlas.py --book ultimate_campaign --check` at real HEAD `1e6a67390e` (wave 17) |
| The 4 open-subtype-family `%LIST` trait records, census | `trait_artisan` (`CHOOSE:SKILL\|TYPE=Craft`, bonus +2), `trait_mentored` (`CHOOSE:SKILL\|TYPE=Craft\|TYPE=Perform\|TYPE=Profession`, bonus +1), `trait_simple_disciple` (`CHOOSE:SKILL\|TYPE=Craft\|TYPE=Profession`, bonus +1, magnitude taken from description text — the corpus `BONUS` token omits it, same "never invented" discipline as cycle 4's `trait_harvester`/`trait_simple_disciple` precedent), `trait_talented` (`CHOOSE:SKILL\|TYPE=Perform`, bonus +1) | direct read of `data/corpus/ultimate_campaign/trait_generic/trait_{artisan,mentored,simple_disciple,talented}.json`'s own `raw_tokens` |
| Craft/Perform/Profession family rosters this crate already recognizes | `CRAFT_SKILL_IDS` 23, `PERFORM_SKILL_IDS` 9, `PROFESSION_SKILL_IDS` 31 — pre-existing, not created this cycle | `src/rules_core/skill_allocation.rs`, `skill_family_member_ids`; `assert_eq!(mentored.skill_options.len(), 23 + 9 + 31)` (`trait_picker.rs::mentored_option_carries_the_union_of_all_three_named_families`) |
| Units genuinely promoted M → DONE (`grounded`), this cycle | **4**, all in `ultimate_campaign` — no shared-corpus-`KEY` payoff in another book this time (checked, not assumed: `grep -rl` for all 4 corpus `KEY` strings across `data/corpus/` outside `ultimate_campaign` returns nothing) | id-set diff of `docs/work-inventory.json` before (committed) vs. after (this cycle's own local, uncommitted regen): `0 added, 0 removed`, exactly 4 changed, all `ingested-magnitude → grounded`: `trait_artisan`, `trait_mentored`, `trait_simple_disciple`, `trait_talented` |
| `ultimate_campaign` bucket state after this cycle | `DONE 187→191, M 53→49`, all other buckets unchanged (`U:21 D:2 X:2 V:0`) | `python3 scripts/completion_atlas.py --book ultimate_campaign --check` (post-local-regen, pre-restore) |
| `completion_atlas.py --check` corpus-wide, before this cycle's citation-pin fix | `citation_failures=1` (`V`: `src/bin/v06_work_inventory.rs:12437` no longer contains `literal-verified` — this cycle's own doc-comment insertion above `Kind::Trait`'s match arm shifted every later line by +8) | `python3 scripts/completion_atlas.py --check`, pre-fix |
| `completion_atlas.py --check` corpus-wide, after the fix | `population=49438 unclassified=0 overlap=0 citation_failures=0` — bucket `M` moved `4965→4961` (−4, this cycle's own 4 closures; no other lane's change is co-mingled — no other file besides the 4 named units and the 2 aggregate summary lines changed in the local regen's diff) | `python3 scripts/completion_atlas.py --check`, post-fix, post-local-regen |
| `denominator_gate.py --check` | `files_checked=15 violations=8` — pre-existing `FRT_HVY`/quoted-corpus-prose baseline in `progress.md`, unchanged by this cycle's own new prose (no bare percentage added) | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` |
| `box_ledger.py --check` | pre-existing drift (THE-BOX.md's static counts vs. live `docs/work-inventory.json`), unchanged in kind, inherited read-only per `decisions.md §2` | `python3 scripts/box_ledger.py --check` |

## Row-count command output

```
$ awk '/pub static FAMILY_CHOICE_TRAIT_BONUSES/,/^\];/' src/rules_core/trait_effects.rs | grep -c 'trait_id:'
4
```

This cycle's own artifact is the new `FAMILY_CHOICE_TRAIT_BONUSES` table — its row count (4)
is exactly the DONE-bucket delta this cycle claims (`187→191`), per `decisions.md §4`.

## Sweep population

No `data/corpus/**` file was touched this cycle (`decisions.md §12` L8 does not apply — the
delta must be zero, and it is):

1. `corpus_literal_sweep --json-out` → `CLEAN, 48708 records examined of 51482 read` — identical
   to `AT-34-E4-002_cycle_receipt_4.md`'s own baseline (48708). Before/after: **unchanged, 0
   delta**, N/A movement.
2. `derived_evaluator_fixture_check --json-out` → `1839 unit(s) cleared over 2580 fixture row(s);
   0 failed; 0 not ingested` — identical to cycle 4's baseline.
3. Local, **uncommitted** `v06_work_inventory` regen (both reports set, no `--allow-stamp-loss`
   needed — no stamp loss occurred): `docs/work-inventory.json` id-set stable (`0 added, 0
   removed`), exactly 4 units' `status`/`evidence` changed. `git restore`d immediately after
   deriving the figures above; not committed (`GENERATED_FILE_BAN`).

## Oracle pin

Not applicable — no figure here came from the pinned PCGen oracle checkout; every figure was
derived from the live repo's `data/corpus/` tree and this cycle's own executed fixture tests
(`trait_effects.rs`'s `every_family_choice_entry_is_genuinely_grounded_by_fixture_execution`,
which actually builds a fixture character, records a real recognized family-resolved choice, and
runs it through `allocate_skill_ranks`).

## Build scope verified

`cargo build --locked --lib`: exit 0 (only pre-existing warnings, no new ones).
`cargo test --locked --lib -- trait_effects skill_allocation`: **43/43 passed** (10 new tests for
the family-choice table/compute path, including `every_family_choice_entry_is_genuinely_grounded_
by_fixture_execution` (executes the engine for all 4 entries) and
`no_trait_id_appears_in_more_than_one_table` (proves the family table shares no `trait_id` with
either of the other two, so the three summed maps in `allocate_skill_ranks` never double-apply)).
`cargo test --locked --bin v06_work_inventory -- trait`: **58/58 passed** (1 new positive test —
`a_family_choice_skill_trait_bonus_promotes_a_held_trait_record_to_grounded`, proving the third
`.or_else` fallback reaches `grounded` specifically via `Trait ~ Artisan`; the repointed negative
control — `a_trait_outside_the_flat_slice_stays_ingested_magnitude`, now keyed on `Trait ~
Bruising Intellect` — confirmed still honestly `ingested-magnitude`). `cargo test --locked
--manifest-path apps/desktop/src-tauri/Cargo.toml -- trait_picker`: **8/8 passed** (2 new:
`artisan_option_carries_the_full_craft_family_as_skill_options`,
`mentored_option_carries_the_union_of_all_three_named_families`; 2 widened:
`returns_every_flat_and_choice_skill_trait` (31+5+4=40), `every_choice_option_round_trips_through_
the_compute_path` (now sums both choice-based compute paths' contributions, since a trait id is a
member of exactly one)); the one pre-existing desktop-crate failure in the same run
(`race_trait_picker::...the_menu_command_carries_all_fourteen_adopted_race_options...`) reproduced
identically, confirmed pre-existing and unrelated (outside `race_trait_picker.rs`, a file this
cycle never touched). `cargo build --locked --manifest-path apps/desktop/src-tauri/Cargo.toml`:
exit 0. `apps/desktop`: not touched this cycle (the frontend needed zero changes — see Notes) —
`npx tsc --noEmit`/`npm test` not re-run, no `.ts`/`.tsx` file in this cycle's diff.

`cargo test --locked --no-run` (full workspace, widest build scope, `decisions.md §10`): run at
this cycle's final HEAD (`5e3c000c8e`) — `0` `error[`/`error:` lines, every test binary in the
workspace linked successfully, exit 0. `apps/desktop/src-tauri` (separate cargo workspace, tested
explicitly, full run not just the scoped `trait_picker` slice): `cargo test --locked
--manifest-path apps/desktop/src-tauri/Cargo.toml`: **528 passed, 28 failed**, `finished in
33.35s` — the identical 28-test failure set cycle 3/4's own receipts recorded
(`companion_catalog.rs` 15, `feat_catalog.rs` 1, `race_trait_picker.rs` 1, `reach_gate.rs` 11),
confirmed by name-for-name comparison; none of the 28 are in `trait_picker.rs`, `character_hub.rs`,
`pf1_adapter.rs`, or any other file this cycle touched. (528 vs. cycle 4's 526: +2, exactly this
cycle's own 2 new `trait_picker` tests.)

## Status: partial

## Movement, four buckets (`decisions.md §9`)

- **Closure:** 4 units, all `ultimate_campaign`, `M → DONE`, via a real, fixture-executed
  open-subtype-family `BONUS:SKILL|%LIST` compute path — genuine closure, not a relabelling:
  every one of the 4 entries is re-verified by `every_family_choice_entry_is_genuinely_grounded_
  by_fixture_execution`, which builds a real character that both selects the trait AND records a
  genuine, family-resolved skill choice, then runs it through the real skill-allocation engine.
- **Reclassification:** 0.
- **Reachability:** 0 (this cycle widens the compute path itself and its desktop selection
  surface, not a display/explanation wire onto an already-computed value).
- **Instrument-correction:** 1 (`completion_atlas.py`'s `V`-bucket citation line pin, shifted by
  this cycle's own line insertions; `citation_failures` 1→0, no bucket population moved by the
  fix itself).

## Instrument/doc correction (retro-logged, 2 events in `docs/retro/events/sd34-at-34-e4-002.jsonl`)

1. **Cycle 4's own module doc comment mischaracterized this shape as out of scope.** It stated
   the 4 open-subtype-family traits need "a genuinely open-ended text-entry chooser... a
   materially different UI/input shape" than the closed-list second slice. That is not true of
   this app: `skill_allocation.rs` already carries a closed, corpus-derived enumeration of every
   Craft/Perform/Profession subtype this crate recognizes, used for its own `TYPE=<Family>`
   class-skill-wildcard expansion — the same closed list is the legal option set for these traits
   too. Corrected in `trait_effects.rs`'s module doc comment (new "Third slice" section) and
   retro-logged as a `correction` (`--subject` the prior receipt, `--verified-by` the new fixture
   tests).
2. **A negative-control test fixture referenced a record this same cycle's own new compute path
   went on to cover.** `v06_work_inventory.rs::a_trait_outside_the_flat_slice_stays_ingested_
   magnitude` used `Trait ~ Artisan` as its "stays uncovered" example; the moment the third
   `.or_else` fallback was wired in, the test failed loudly (as a negative control should) rather
   than silently drifting. Repointed to `Trait ~ Bruising Intellect` (a genuine ability-formula
   shape with no near-term compute path), retro-logged as a `correction`.

## Notes

- **This is a third slice of a capability build, not a book-scoped patch.** `TraitSkillFamily
  ChoiceBonus`, `FAMILY_CHOICE_TRAIT_BONUSES`, `family_choice_skill_options`, and the desktop
  `trait_picker.rs` third `.chain()` are all general machinery — any book's open-subtype-family
  `BONUS:SKILL` trait record benefits the moment its corpus `KEY` is added to the table, the same
  kind-keyed payoff cycles 3/4 already proved. This cycle's own inventory diff shows no *other*
  book currently shares one of these 4 exact records' corpus `KEY` (checked, not assumed), so the
  corpus-wide count is 4 this time.
- **No stub, no half-wired compute path, and — notably — no frontend change was needed at all.**
  `family_choice_bonuses_from_traits` genuinely computes and applies (proven by fixture
  execution), and `CreateCharacterForm.tsx`'s existing picker already renders any trait option
  generically off `skillOptions.length > 0` / `choiceSetId` (built in cycle 4 for the fixed-list
  second slice) — the third slice's larger, family-resolved `skill_options` list reaches the
  desktop UI with zero `.tsx`/`.ts` changes, a stronger form of "no compute path with no UI
  reaching it" than cycle 4's own closing note claimed, because the UI layer was already fully
  generic.
- **U(21), D(2), X(2) were not touched, reopened, or reclassified.** Verified by the inventory
  diff: zero `ultimate_campaign` units with those starting statuses appear in the 4-unit changed
  set.
- **`git status --porcelain` before every write; no `git add -A`; no `git stash`.** One
  `git fetch origin tranche/14 && git rebase origin/tranche/14` before pushing — fast-forwarded
  cleanly (`Current branch ... is up to date`), no manual conflict resolution; nobody else pushed
  to `tranche/14` between this cycle's fetch and push.
- **Checkpoint discipline:** one commit (source + retro log together, `5e3c000c8e`), pushed
  within the same turn it was made, per the box's hard-kill risk; this receipt + progress/kanban
  land in a second commit immediately after, per §6/§5.
- **`origin/salvage/wave14-lane1`** (named in the dispatch brief as this criterion's rescued
  checkpoint) **did not exist on this box** at dispatch time — `git branch -a` and
  `git fetch origin salvage/wave14-lane1` both came back empty. This cycle built from committed
  history (`AT-34-E4-002_cycle_receipt_4.md`'s own landed work), not a rescued checkpoint; nothing
  was inherited un-re-derived.

## Next-cycle plan

Remainder is `M:49 U:21 X:2 D:2` = 74 of 265 non-`DONE`, named by sub-cause (unchanged from
cycle 4 except the 4 now closed): (a) 3 ability-score-difference-formula trait records — need a
formula evaluator this crate does not have (`Trait ~ Bruising Intellect` and 2 siblings); (b) 15
mixed-bonus-type trait records (`BONUS:VAR/SAVE/SITUATION/ABILITYPOOL/COMBAT/CONCENTRATION`) —
different pillars, each its own compute path; (c) 1 corpus data gap (`trait_shadow_whispers`);
(d) 17 narrative Drawback + 1 cross-skill-guarded Drawback + 12 `Retrain` `ability_content`
records — all unchanged from cycle 3's own naming, none of this cycle's scope. The cheapest next
sub-cause is likely (b)'s `BONUS:SAVE` or `BONUS:COMBAT` records if either turns out to share an
existing compute path the way this cycle's family-choice slice reused `skill_allocation.rs`'s
existing rosters — worth a direct-read census before assuming each of the 6 sub-shapes needs a
wholly new mechanism. Re-run `python3 scripts/completion_atlas.py --book ultimate_campaign
--check` after each sub-wave.
