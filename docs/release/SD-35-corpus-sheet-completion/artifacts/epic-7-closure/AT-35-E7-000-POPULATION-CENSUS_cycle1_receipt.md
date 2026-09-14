# Cycle 1 — Epic 7 Closure epilogue / AT-35-E7-000-POPULATION-CENSUS

**THE ONE-SENTENCE ANSWER.** `completion_atlas.py --check`'s `DONE: 49,438 of 49,438` is an
**INVENTORY-wide figure, not a corpus-wide one** — the atlas's denominator is
`docs/work-inventory.json` (`scripts/completion_atlas.py:71`), which is enumerated from the
**pinned PCGen tree**, not from `data/corpus` (`src/bin/v06_work_inventory.rs:19949`); the
**corpus-wide figure is 48,609 of 48,864 real corpus rules records (99.478%)**, because **255
`data/corpus` records with `completeness: full` and published rules prose reach no inventory unit
at all** and are therefore absent from both the atlas's numerator and its denominator.

- **Commit SHA:** see `Build scope verified` below (filled at push; the cycle's head commit)
- **Scope gate:** `SCOPE_GATE: EXEMPT (measurement cycle — it moves no unit and writes no rule; its deliverable is a census and a verdict)`
  (`decisions.md §2` floor exemption: this cycle closes ZERO units **by design** and is forbidden by
  its own dispatch from moving any. `python3 scripts/cycle_scope_gate.py --receipt` rows below
  confirm `closed=0 relabeled=0 rust_lines_changed=0`.)
- **Files touched:**
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/population_census.py` (new)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/population-census.json` (new, the deliverable)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/population-census-unreached.json` (new, per-record backing data for all 2,912)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/AT-35-E7-000-POPULATION-CENSUS_cycle1_receipt.md` (this file)
  - `docs/release/SD-35-corpus-sheet-completion/progress.md`, `docs/release/SD-35-corpus-sheet-completion/kanban.md`
  - `docs/retro/events/at-35-e7-000-population-census.jsonl`
  - **Nothing else.** No unit added, nothing converted, no classifier edited, no gate changed —
    `data/corpus`, `docs/work-inventory.json`, `data/sheet_rules/`, `src/` are byte-identical to
    the cycle-start SHA.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS **on this cycle's own diff**. The
  `${BASE_BRANCH}...HEAD` scoped-path grep does report three pre-existing hits, none of them this
  cycle's and none of them a stub: the word `hack` inside published Pathfinder prose in
  `data/sheet_rules/` (`bestiary_3:monster_ability:tophet_swallow_whole`,
  `core_rulebook:spell:plant_growth` — "hack or smash its way out", "hack or force a way through"),
  and the word `placeholder` inside three `docs/work-inventory.json` lines this bundle **removed**
  (`-` lines: PCGen's own `empty_selection_*` CHOOSE-menu rows). Re-derive:
  `git diff --unified=0 $(git merge-base HEAD origin/develop)...HEAD -- data/corpus docs/work-inventory.json data/sheet_rules src/bin/v06_work_inventory.rs src/pcgen_import/sheet_rule docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`
- **Acceptance criterion:** `AT-35-E7-000-POPULATION-CENSUS` is **not** a section in
  `epic-breakdown.md` — it is a criterion the orchestrator dispatched into Epic 7 ahead of
  `AT-35-E7-001`, and its dispatch text is therefore the acceptance bar, quoted here verbatim:
  *"THE QUESTION, and it is the only one: HOW MANY CORPUS RECORDS NEVER REACH
  docs/work-inventory.json? … DELIVERABLE:
  docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/population-census.json plus a
  receipt that states, in one sentence, WHETHER '49,438 of 49,438' is a corpus-wide figure or an
  inventory-wide figure — and if the latter, what the corpus-wide figure actually is."*
- **Receipt rows (mechanical):**
  `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=0`
- **PCGen residue:** `live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS`
  (also `shipped_data_files=0 shipped_data_hits=0 shipped_scanned=11`, `identifier_files=0
  identifier_hits=0`). Unchanged from cycle start — this cycle writes no Rust and no shipped data.
- **Oracle parity:** N/A — no `Number` mapping added, no live path touched. The pinned oracle tree
  was read **read-only**, to display the source `.lst` row behind each hand-sampled record.
- **Movement, four buckets:**
  - closure: **none** (id-set empty)
  - relabel: **none**
  - reachability: **none**
  - instrument-correction: **this whole cycle**. It corrects the *reading* of an existing figure
    without moving it: `DONE 49,438 of 49,438` is re-stated with its true denominator. No number in
    any instrument changed.
- **Refused tokens:** none refused by this cycle (it converts nothing). The **deferred** population
  it names for a successor criterion is `record_absent_from_inventory_population=255`.
- **Discoveries:** two, both emitted as `correction` retro events —
  1. the `data/corpus` population is **51,521** records, not the 27,681 the dispatch brief carried;
  2. the naive corpus-path→inventory-id join's **12,659** is wrong by a factor of ~4.3 in the
     count *and* wrong in kind: the true never-reached figure is **2,912**, of which only **255**
     are real rules records.
  A third `correction` records the headline's denominator itself.
  Nothing here is a `token-coverage.json` refusal shape and nothing is a new atlas remaining-step
  category, so neither instrument needs re-deriving.

## Figures + their re-derive commands

Every figure below re-derives from
`python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/population_census.py`
unless another command is named. Its method is stated in the artifact and repeated here because it
is the part that makes the numbers trustworthy: **it inverts the only code path that actually joins
the two populations** — `src/pcgen_import/sheet_rule/mod.rs::walk_corpus` (`:105`) and
`::load_population` (`:256`) — rather than comparing two naming schemes.

### The two populations, and why they are not the same population

| Figure | Value | Denominator / command |
| --- | --- | --- |
| `data/corpus` records | **51,521** | one `data/corpus/<book>/**/*.json` file is one record; `_parity/` and `LICENSE.json` excluded exactly as `walk_corpus` excludes them. `find data/corpus -name '*.json' -not -path '*/_parity/*' -not -name LICENSE.json \| wc -l` → `51521` |
| `docs/work-inventory.json` units | **49,438** | `python3 -c "import json;print(len(json.load(open('docs/work-inventory.json'))['units']))"` |
| `completion_atlas.py --check` population | **49,438**, `DONE: 49438` | `python3 scripts/completion_atlas.py --check` → `population=49438 buckets=10 unclassified=0 overlap=0` |
| atlas denominator's source | `docs/work-inventory.json` | `scripts/completion_atlas.py:71` `INVENTORY_PATH` |
| inventory's own source | the **pinned PCGen tree**, `$PCGEN_CORPUS_ROOT/…/roleplaying_game/` | `src/bin/v06_work_inventory.rs:19949` (`corpus_root` ← `PCGEN_CORPUS_ROOT`), `:20007` (`book_paths` ← `read_dir(books_dir)`) |

**This is the whole finding in one row:** the two populations are built from **different sources**.
`data/corpus` is our ingested schema; the inventory is enumerated from the pinned `.lst` tree.
Nothing in the repo forces them to agree, and they do not.

### Direction 1 — corpus records that never reach the inventory

| Figure | Value |
| --- | --- |
| corpus records reached by ≥1 inventory unit | **48,609** |
| corpus records **never** reached | **2,912** |
| …of which are **real rules records** | **255** |
| …of which are **not records at all** | **2,657** |

The 2,912, in four buckets, each hand-verified:

| Bucket | n | What it is | Is it a lost player-facing rule? |
| --- | --- | --- | --- |
| `chassis_only_mod_row_no_rule_text` | 1,998 | the record's own `source.line` in the pinned tree is a `<Name>.MOD` row that only adds `TYPE:`/`CLASSES:` to a record that already exists. `completeness: chassis_only`, `data.description: null`. | **No.** It carries no rule of its own, so under the sheet rule (`decisions.md §1`) it has no sheet line of its own. |
| `duplicate_ingest_of_a_row_already_reached` | 612 | a **second** corpus JSON for the same `(book, source file, source line)` as a record an inventory unit already claimed. | **No.** The rule reaches a sheet through the twin. |
| `FULL_with_prose_no_row_reached` | **255** | `completeness: full`, carries published rules prose in `data.description`, and **no inventory unit resolves to its source row under either join key**. | **YES. This is the real gap.** |
| `generated_artifact_no_completeness` | 47 | `data/corpus/<book>/_settled/<kind>.json` — an aggregate `{kind, records:{…}}` map written by the settle pass. No `completeness`, no `source`, no `data`. | **No.** Not a record. |
| `FULL_without_prose_no_row_reached` | 0 | — | — |

**Hand-verification — 28 records, by hand, against the pinned `.lst` rows (the dispatch asks for
≥20).**

*`chassis_only` — 10 sampled, 10 confirmed `.MOD` list-assignment rows, 0 real records:*

```
spell  cushioning_bands          | cushioning bands.MOD      TYPE:Psychic  CLASSES:Psychic=2
spell  vampiric_touch-2          | vampiric touch.MOD        TYPE:Psychic  CLASSES:Psychic=3
spell  glue_seal                 | glue seal.MOD             TYPE:Psychic  CLASSES:Psychic=1
spell  unbearable_brightness     | unbearable brightness.MOD TYPE:Psychic  CLASSES:Psychic=4
spell  displacement              | Displacement.MOD                        CLASSES:Magus=3
spell  shield_of_fortification   | shield of fortification.MOD TYPE:Psychic CLASSES:Psychic=2
spell  delay_pain                | delay pain.MOD            TYPE:Psychic  CLASSES:Psychic=2
spell  curse_of_magic_negation   | curse of magic negation.MOD TYPE:Psychic CLASSES:Psychic=4
spell  endure_elements-2         | endure elements.MOD       TYPE:Psychic  CLASSES:Psychic=1
spell  twilight_knife            | Twilight Knife.MOD        TYPE:Psychic  CLASSES:Spiritualist=3
```

Re-derive one: `sed -n '284p' "$PCGEN_CORPUS_ROOT/pathfinder/paizo/roleplaying_game/occult_adventures/oa_spells.lst"`
→ `Endure Elements.MOD  TYPE:Psychic  CLASSES:Spiritualist=1`, matching
`data/corpus/occult_adventures/spell/endure_elements.json`'s `source.line: 284`. The real
*Endure Elements* is a Core Rulebook record and is reached.

*`duplicate_ingest` — 6 sampled, 6 confirmed twins of a claimed row.* The worked case:
`data/corpus/advanced_players_guide/ability/child_of_the_streets.json` and
`data/corpus/advanced_players_guide/trait_generic/trait_child_of_the_streets.json` are **both**
`apg_abilities.lst:122`. The unit `advanced_players_guide:trait:trait_child_of_the_streets`
(`status: sheet-complete`, `evidence: sheet_rule_rendered:number`) claims the first, because
`walk_corpus` sorts `ability/` before `trait_generic/` and `load_population`'s index is
first-wins. Re-derive: `grep -rln '"line": 122' data/corpus/advanced_players_guide/ | xargs grep -l apg_abilities.lst`
→ exactly those two files.

*`_settled` — 3 sampled, 3 confirmed generated aggregates.*
`data/corpus/advanced_class_guide/_settled/equipment.json` top-level keys are `["kind","records"]`,
not the `population/completeness/data/source/license/…` shape every ingested record has.

*`FULL_with_prose` — 12 sampled, **12 confirmed real player-facing rules records**, 0 artifacts:*

| record | source row | published text (head) |
| --- | --- | --- |
| `advanced_race_guide/race_trait/samsaran/samsaran_vision` | `samsaran_abilities_race.lst:18` | "Samsarans can see twice as far as humans in conditions of dim light." |
| `advanced_race_guide/race_trait/aasimar/aasimar_garuda_blooded` | `aasimar_abilities_race_subrace.lst:14` | "Their shimmering avian features make plumekith instantly recognizable…" |
| `advanced_race_guide/race_trait/changeling/changeling_claws` | `changeling_abilities_race.lst:20` | "Changelings' fingernails are hard and sharp, granting them two claw attacks (1d4 points of damage each)." |
| `advanced_class_guide/equipment/special_ability_jarring_armor` | `acg_equipmods.lst:15` | "3/day daze attacker for 1 round (DC 16 Fortitude negates)" |
| `advanced_race_guide/race_trait/tiefling/qlippoth_spawn_ability_scores` | `tiefling_abilities_race_subrace.lst:110` | "The most hideous of their kind, these tieflings instinctively despise all other forms of mortal life." |
| `core_essentials/feat/empower_spell_like_ability_spell` | `ce_feats.lst:13` | "One of this creature's spell-like abilities is particularly potent and powerful…" |
| `advanced_race_guide/race_trait/aasimar/agathion_blooded_skilled` | `aasimar_abilities_race_subrace.lst:25` | "Idyllkin have a +2 racial bonus on Handle Animal and Survival checks." |
| `advanced_race_guide/race_trait/wayang/wayang_light_and_dark` | `wayang_abilities_race.lst:19` | "Once per day as an immediate action, a wayang can treat positive and negative energy effects as if she were an…" |
| `advanced_race_guide/race_trait/aasimar/archon_blooded_skilled` | `aasimar_abilities_race_subrace.lst:47` | "Lawbringers have a +2 racial bonus on Intimidate and Sense Motive checks." |
| `advanced_race_guide/race_trait/aasimar/aasimar_angel_blooded` | `aasimar_abilities_race_subrace.lst:11` | "Angelkin are mortal paragons of exceptional beauty…" |
| `advanced_class_guide/equipment/special_ability_distracting_weapon` | `acg_equipmods.lst:34` | "increase target's concentration DC by 5 for 1 minute; can't be affected again for 24 hours" |
| `advanced_race_guide/race_trait/aasimar/agathion_blooded_spell_like_ability` | `aasimar_abilities_race_subrace.lst:26` | "Idyllkin gain summon nature's ally II as a spell-like ability." |

Every one is a rule a player would expect on a sheet: ARG variant-heritage racial traits
(aasimar, tiefling, samsaran, changeling, wayang), ACG armor/weapon special abilities with their
printed effect line, Core Essentials and Unchained feats. **The sample contains zero generated
artifacts and zero duplicates.** The verdict on the 255 is therefore: **a real gap.**

**The 255, by book and kind** (`unreached_by_book_kind` in the artifact):

| book / kind | n |
| --- | --- |
| `advanced_race_guide` / `race_trait` | 178 |
| `advanced_class_guide` / `equipment` | 48 |
| `core_essentials` / `feat` | 15 |
| `pathfinder_unchained` / `feat` | 9 |
| `advanced_players_guide` / `spell` | 4 |
| `mythic_adventures` / `spell` | 1 |
| **total** | **255** |

**The three records the dispatch named as already-confirmed are all in this bucket, re-verified
from disk, not taken on trust:**
- `advanced_players_guide:spell:wall_of_thorms` — `data/corpus/advanced_players_guide/spell/wall_of_thorms.json`,
  `completeness: full`, **1,851 characters** of published rules prose, `apg_spells.lst:1555`; no
  inventory unit under any spelling.
- `mythic_adventures:spell:elemental_body_iiimod` — same shape, **649 characters**.
- `AT-35-E6-003`'s deferral ("24 carry no converted rule at all … these corpus records are not in
  it") is **confirmed by mechanism**: the converter's population *is* `docs/work-inventory.json`
  (`load_population` iterates `inv.units`, never `walk_corpus`'s output), so a corpus record with
  no unit is structurally unreachable by the converter. That is exactly what this census measures.

### Direction 2 — inventory units with no corpus record, reconciled with the known 142

| Figure | Value |
| --- | --- |
| inventory units the `data/corpus` join resolves to nothing | **829** |
| …rescued by `sheet_rule/mod.rs::source_row_in_tree` (falls back to the unit's own row in the pinned tree) | **687** (685 `feat`, 2 `spell`) |
| …refused `no_corpus_record` | **142** (104 `race_trait`, 27 `race`, 11 `feat`) |

**829 = 687 + 142, exactly.** The 142 in `data/sheet_rules/_refused.json`
(`{"records":49438,"converted":49296,"refused":142,"by_token_type":{"no_corpus_record":142}}`) are a
**strict subset** of the 829 — verified `refused ⊆ unjoined` is `True` with 0 ids outside. The other
687 render their words from the pinned row and are DONE. This direction is **fully reconciled with
the known instance and holds no surprise.**

### The corpus-wide figure

| Statement | Value |
| --- | --- |
| the atlas's figure | `DONE 49,438 of 49,438` = **100% of the inventory** |
| real corpus rules records (reached + the 255) | **48,864** |
| of those, reached by a DONE inventory unit | **48,609** |
| **corpus-wide completion** | **48,609 of 48,864 = 99.478%** |
| all `data/corpus` JSON files accounted for (reached, or provably not a record) | 51,266 of 51,521 |

## Build scope verified

`cargo test --locked --no-run -j 6` ran in this cycle's own
`CARGO_TARGET_DIR=/tmp/cargo-sd35-AT-35-E7-000-POPULATION-CENSUS` → **`NO_RUN_EXIT=0`**, every test
target compiled (log tail through `tests/wizard_abjuration_school_powers.rs`). The mechanical
`--receipt` row reads `builds_recorded=0` because this cycle recorded no build-counter entry — the
`--no-run` exit is stated here directly rather than left to be inferred.
**The build is not load-bearing for this cycle's claim:** the cycle changes no Rust,
no `data/corpus` record, no `data/sheet_rules` file and no inventory unit — `closed=0
rust_lines_changed=0`, and the only writes are the four new artifact files plus `progress.md`,
`kanban.md` and one retro event log. Per `decisions.md §3` there is nothing for a build to prove
here. The instruments that *do* bear on the claim were all run at HEAD and are recorded below.

- `python3 scripts/completion_atlas.py --check` → `population=49438 buckets=10 unclassified=0 overlap=0`, `DONE: 49438`, `done_evidence_violations=0`, `citation_failures=0` — **PASS**
- `python3 scripts/token_coverage.py --check` → `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=233 shapes=1 verdict=PASS`
- `python3 scripts/pcgen_residue_gate.py --check` → `live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS`
- `python3 scripts/shape_engine_boundary.py --check` → `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True`
- `python3 scripts/missing_engine_tables.py --check` → `population=0 kinds=0 citation_failures=0`
- `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → `0`
- `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` → `files_checked=161 violations=0`
- `scripts/verify.sh --only pi-sweep` → `RESULT: PASS`
- **Skipped, with the reason:** `cargo run --locked --bin corpus_literal_sweep` (runs only when
  corpus records changed — none did); `cargo run --locked --bin sheet_rule_convert -- --check` and
  the oracle comparison (run only when the converter, its mapping table or the corpus moved — none
  did; `data/sheet_rules/` is byte-identical to the cycle-start SHA); `cargo clippy` (no Rust
  target touched); the desktop crate and frontend (`apps/` untouched).

## Sweep population

N/A — no corpus record changed, so `corpus_literal_sweep` has nothing to re-examine.

## Oracle pin

`7f818006e371188e5717fd18d74d18a420747fc6` (`data/sheet_rules/_report.json`'s `oracle_pin`, and
`scripts/pcgen-oracle-pin.env`). The pinned tree was read **read-only**, to print the `.lst` row
behind each hand-sampled record; no figure in this receipt is *derived from* the oracle, only
*illustrated by* it.

## Status

**partial.**

The criterion's own deliverable is complete: the census exists, the join is the real one, the
sample is 28 records deep, both directions reconcile, and the one-sentence verdict is stated. But
this bundle's `partial` rule is a population rule, and the population this cycle establishes is
**not zero at HEAD**: **255 real corpus rules records sit outside the atlas's denominator
entirely**. Reporting `complete` would restate the exact error the cycle was dispatched to find.

- **Refused-token remainder, named and summing:** `record_absent_from_inventory_population=255`
  (`advanced_race_guide/race_trait=178` + `advanced_class_guide/equipment=48` +
  `core_essentials/feat=15` + `pathfinder_unchained/feat=9` +
  `advanced_players_guide/spell=4` + `mythic_adventures/spell=1` = **255**). Emitted as a
  `deferral` retro event.
- **Not `blocked-escalated`:** nothing on `workflow-instruction.md §8`'s non-self-healable list
  fired. The tree is clean, one writer, no gate regressed, `pcgen_live_files` did not rise.
- **This is not a carve-out.** Under the standing ruling, "the engine cannot model X" is never an
  exemption — and this is not even that: these are ordinary racial traits, weapon special
  abilities, feats and spells with published text, and under the sheet rule every one of them
  renders as words. They are absent because the **inventory's enumerator never saw them**, which is
  a population defect with a fix, not an impossibility.

## Notes

The dispatch's warning was correct and the trap is worth recording precisely: the naive
path→id join returns **12,659** because it compares `data/corpus`'s directory vocabulary
(`feat_generic`, `race_trait_generic`, `trait_generic`, `_settled`, `ability`) with the inventory's
kind vocabulary (`feat`, `race_trait`, `trait`). The real join is **(book, source-file basename,
source-line) first, (book, kind, id-tail) second** — coordinates both populations carry honestly —
and it returns **2,912**. Two of the four buckets then exist only because `load_population`'s
indexes are **first-wins**: a second corpus JSON for a row another JSON already holds is
structurally unreachable, which is why 612 records look lost and are not.

## Next-cycle scope

Criterion at **255**, not zero. The fix is a **separate criterion**, as the dispatch requires —
measuring and fixing in one cycle is how a population error gets buried. What that criterion needs:

1. An **operator ruling** on whether the 255 enter `docs/work-inventory.json` (widening
   `v06_work_inventory`'s enumerator, which changes the atlas denominator from 49,438 and moves
   every bundle figure that quotes it), or are recorded as a stated out-of-population set with
   their reason.
2. Scope flags are **not** expressible as `cycle_scope_gate.py` bucket/kind filters, because these
   records are **not in the inventory the gate reads**. The successor cycle claims the §2 floor
   exemption the same way this one did, or takes the whole 255 as its named remainder.
3. **`AT-35-E7-001`'s final-acceptance scan and SD-35's closure should not treat `49,438 of 49,438`
   as corpus-wide.** Either the bundle's headline is restated with its denominator
   ("100% of the 49,438-unit inventory; 99.478% of the corpus's 48,864 rules records"), or the 255
   are brought in first.
