# Cycle 2 — Epic 7 Closure epilogue / AT-35-E7-000-POPULATION-CENSUS

**THE ONE-SENTENCE ANSWER.** Cycle 1's remainder `record_absent_from_inventory_population=255` is
an artefact of the join, not a count of lost rules: **193 of the 255 are the same `.lst` row under a
re-attributed book label**, **48 are reached by the `.COPY=` twin row `acg_equipmods` enumeration
deliberately prefers**, **4 carry prose byte-identical to a record an inventory unit already
claims**, and **only 10 reach no rendered sheet line at all** — 9 `pathfinder_unchained` "Champion
of X" feats and 1 `mythic_adventures` spell, all 10 failing the *same* enumeration test — so the
corrected corpus-wide figure is **48,854 of 48,864 real corpus rules records = 99.9795%**, not the
48,609 of 48,864 = 99.478% cycle 1 recorded.

- **Commit SHA:** `3cc3b384a8` (cycle start `9af5cd311e`)
- **Scope gate:** `SCOPE_GATE: EXEMPT (measurement cycle — it moves no unit and writes no rule; its deliverable is a census and a verdict)`
  (`decisions.md §2` floor exemption, the same ground cycle 1 stood on and for the same reason: this
  cycle closes ZERO units **by design** and is forbidden by its own dispatch from moving any. The
  remainder it is scoped to is furthermore **not expressible as `cycle_scope_gate.py` flags at all**
  — the gate's population is `docs/work-inventory.json`, and these records are by definition the
  ones *not in it*. Run for the record, the gate reads
  `scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER`, which is the inventory
  being wholly DONE, not a statement about this cycle's scope. The mechanical `--receipt` rows below
  confirm `closed=0 relabeled=0 rust_lines_changed=0`.)
- **Files touched:**
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/population_census_255.py` (new — the method)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/population-census-255.json` (new — the deliverable)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/population-census-255-detail.json` (new — per-record disposition for all 255)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/population-census.json` (cycle 1's artifact, **numbers unchanged**, one `cycle_2_correction` pointer key added)
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/AT-35-E7-000-POPULATION-CENSUS_cycle2_receipt.md` (this file)
  - `docs/release/SD-35-corpus-sheet-completion/progress.md`, `docs/release/SD-35-corpus-sheet-completion/kanban.md`
  - `docs/retro/events/at-35-e7-000-population-census.jsonl`
  - **Nothing else.** `git diff --name-only 9af5cd311e..HEAD` lists exactly the above; `data/corpus`,
    `docs/work-inventory.json`, `data/sheet_rules/` and `src/` are byte-identical to the cycle-start
    SHA (`git diff --stat 9af5cd311e..HEAD -- src data docs/work-inventory.json` is empty).
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS **on this cycle's own diff**
  (`git diff --unified=0 9af5cd311e..HEAD -- <scoped paths>`). The wider `${BASE_BRANCH}...HEAD`
  grep reports the same three pre-existing, non-stub hits cycle 1 documented and now a fourth that
  is cycle 1's own receipt quoting them: the word `hack` inside published Pathfinder prose
  (`bestiary_3:monster_ability:tophet_swallow_whole`, `core_rulebook:spell:plant_growth`) and the
  word `placeholder` inside three `docs/work-inventory.json` lines this bundle **removed** (`-`
  lines, PCGen's own `empty_selection_*` CHOOSE-menu rows). None is this cycle's and none is a stub.
- **Acceptance criterion:** `AT-35-E7-000-POPULATION-CENSUS` is **not** a section in
  `epic-breakdown.md` — re-verified this cycle with
  `grep -n 'E7-000' docs/release/SD-35-corpus-sheet-completion/epic-breakdown.md` (no output). It is
  a criterion the orchestrator dispatched into Epic 7 ahead of `AT-35-E7-001`, so its dispatch text
  is the acceptance bar. Cycle 2's dispatch scopes it: *"This is a REMAINDER cycle. The previous
  cycle on AT-35-E7-000-POPULATION-CENSUS returned partial with refused tokens:
  `record_absent_from_inventory_population=255`. Scope this cycle to that named remainder."* — the
  criterion line itself being *"how many corpus records never reach the inventory — measure, do not
  fix."*
- **Receipt rows (mechanical):**
  `closed=0 relabeled=0 rust_lines_changed=0 ratio=n/a builds_recorded=0 pcgen_live_files=0`
- **PCGen residue:** `live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS`
  (also `shipped_data_files=0 shipped_data_hits=0 shipped_scanned=11`,
  `identifier_files=0 identifier_hits=0`). Identical to the cycle-start reading and to cycle 1's —
  this cycle writes no Rust and no shipped data.
- **Oracle parity:** N/A — no `Number` mapping added, no live path touched. The pinned tree was read
  **read-only**, to display the `.lst` rows behind the root cause.
- **Movement, four buckets:**
  - closure: **none** (id-set empty)
  - relabel: **none**
  - reachability: **none** — and this is the row worth reading twice. **No record moved.** 245 of the
    255 were *already* reaching a DONE unit at the cycle-start SHA and at HEAD; what changed is the
    census's ability to see it.
  - instrument-correction: **this whole cycle.** It corrects a *reading* of two existing figures —
    cycle 1's `255` and its `48,609 of 48,864 = 99.478%` — without moving either instrument. No number in
    `docs/work-inventory.json`, `completion_atlas.py`, `token_coverage.json` or `data/sheet_rules/`
    changed.
- **Refused tokens:** `record_absent_from_inventory_population=10`
  (`pathfinder_unchained/feat=9` + `mythic_adventures/spell=1` = **10**). Emitted as a `deferral`
  retro event. This is a **correction of** cycle 1's `=255`, not a second remainder beside it: the
  245 difference is not deferred, it is **not absent**.
- **Discoveries:** three, each emitted as a `correction` retro event —
  1. the headline: cycle 1's `255` real-gap verdict is **10**;
  2. cycle 1's corpus-wide figure `48,609 of 48,864 = 99.478%` is **48,854 of 48,864 = 99.9795%**;
  3. this cycle's own first pass, caught inside the cycle: a `(book, source_file)` presence check
     reported 14 ARG per-race ability files and `ce_feats.lst` as "enumerated by no inventory unit",
     which is false — they are enumerated under a **re-attributed** book label. Had it not been
     caught, this receipt would have said 120 instead of 10.

  None is a `token-coverage.json` refusal shape and none is a new atlas remaining-step category, so
  neither instrument needs re-deriving. `token_coverage.py --check` and `completion_atlas.py --check`
  were run anyway and are recorded below.

## Figures + their re-derive commands

Every figure re-derives from
`python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/population_census_255.py`
(runtime 11 s) unless another command is named.

### Why 255 was the wrong number for the question the sheet rule asks

`load_population`'s two keys — `(book, source_file, source_line)` and `(book, kind, id-tail)` — are
**coordinate** keys: they ask *where a row sits*. The sheet rule (`decisions.md §1`) asks *whether
the rule's words reach a sheet line*. A record can miss both coordinate keys and still have its exact
published paragraph printed. Both coordinate keys also carry a `book` component that
`v06_work_inventory` **deliberately rewrites**: `attributed_book` re-attributes a row to the book of
its newest printing (the supersession ruling). So ARG's `samsaran_abilities_race.lst:14` *is*
enumerated — as `bestiary_4:race_trait:samsaran_ability_scores` — and a book-carrying key cannot see
it.

This cycle adds three content keys, applied in this order (most specific mechanism first):

| # | Key | What it proves | n |
| --- | --- | --- | --- |
| 0 | the same `(basename, source_line)` of the same pinned `.lst`, **book label dropped** | the two are one row of one file — no text test is meaningful, they are the same rule | **193** |
| 3 | the `<New Key>.COPY=<Base>` pairing inside the record's own `.lst`, parsed from the pinned tree | PCGen's equipment-modifier idiom: the rule is written once and copied; our ingest takes the visible original, the enumerator takes the COPY row | **48** |
| 1+2 | `corpus_key`/name across all books **confirmed by verbatim prose identity**, then prose identity against the whole reached population with the key ignored | the same published paragraph is already carried by a record a unit claims | **4** |
| — | none of the above | reaches nothing | **10** |

**193 + 48 + 4 + 10 = 255**, exactly.

**The collision guard.** A shared KEY is *never* credited on its own — memory's standing
`corpus-identifier-scope-collisions` rule. Every key candidate must additionally match the other
record's published `data.description` verbatim (normalised for whitespace and case **only**, never
punctuation), so `Amorphous` (protean anatomy, `bestiary_2:race_trait:amorphous`) can never stand in
for `Amorphous` (ACG armor special ability), and a twin differing in one numeral, die expression or
bonus compares different and stays a gap.

### Hand-verification — 27 samples across the three reached classes, plus all 10 absent

*`reached_via_reattributed_same_row` — 8 sampled, 8 confirmed row-identical (`row_identical=True`):*

```
advanced_race_guide/nagaji_languages          nagaji_abilities_race.lst:22  -> bestiary_4:race_trait:nagaji_languages          [text-complete]
advanced_race_guide/catfolk_vision            catfolk_abilities_race.lst:18 -> bestiary_3:race_trait:catfolk_vision            [sheet-complete]
core_essentials/wingover                      ce_feats.lst:24               -> bestiary:feat:wingover                          [text-complete]
advanced_race_guide/div_spawn_ability_scores  tiefling_..._subrace.lst:80   -> bestiary:race_trait:div_spawn_ability_scores    [sheet-complete]
core_essentials/snatch                        ce_feats.lst:23               -> bestiary:feat:snatch                            [text-complete]
advanced_race_guide/changeling_claws          changeling_abilities_race.lst:20 -> bestiary_4:race_trait:changeling_claws       [text-complete]
advanced_race_guide/oni_spawn_ability_scores  tiefling_..._subrace.lst:100  -> bestiary:race_trait:oni_spawn_ability_scores    [sheet-complete]
advanced_race_guide/wayang_size               wayang_abilities_race.lst:16  -> bestiary_4:race_trait:wayang_size               [sheet-complete]
```

`changeling_claws` and `samsaran_vision` are two of the twelve records **cycle 1 hand-verified as
lost**. They are not lost; they are `bestiary_4` units.

*`reached_via_copy_row_twin_same_file` — 5 sampled, 5 confirmed the COPY row targets the corpus
record's own base row:*

```
special_ability_restful_armor        acg_equipmods.lst:16 -> ...:equipment_modifier:restful      via :89   Special Ability ~ Restful ~ Armor.COPY=Restful
special_ability_distracting_weapon   acg_equipmods.lst:34 -> ...:equipment_modifier:distracting  via :100  Special Ability ~ Distracting ~ Weapon.COPY=Distracting
special_ability_flying_amulet...     acg_equipmods.lst:68 -> ...:equipment_modifier:flying       via :107  Special Ability ~ Flying ~ Melee.COPY=Flying
special_ability_vouchsafing_armor    acg_equipmods.lst:21 -> ...:equipment_modifier:vouchsafing  via :94   Special Ability ~ Vouchsafing ~ Armor.COPY=Vouchsafing
special_ability_blood_hunting_...    acg_equipmods.lst:56 -> ...:equipment_modifier:bloodhunting via :96   Special Ability ~ Blood-Hunting ~ Weapon.COPY=Blood-Hunting
```

This is **the repo's own documented disposition**, independently rediscovered: `v06_work_inventory.rs`
(`book_equipmod_copy_base_targets`, the `TRAP_RULES` note) already records that "every existing
`equipment/equipmods/` record under `advanced_class_guide` cites a line in the 85–132 `.COPY=` block,
never the 10–84 primary block". Cycle 1 counted the primary block as lost.
`special_ability_jarring_armor` and `special_ability_distracting_weapon` are two more of cycle 1's
twelve.

*`reached_via_identical_prose_record` — 4 of 4 checked, all `identical=True`:*

| record | prose twin | chars |
| --- | --- | --- |
| `advanced_players_guide/spell/threefold_aspect_young_adult` | `advanced_players_guide/spell/threefold_aspect` | 1,456 |
| `advanced_players_guide/spell/threefold_aspect_adulthood` | same | 1,456 |
| `advanced_players_guide/spell/threefold_aspect_elderly` | same | 1,456 |
| `advanced_players_guide/spell/wall_of_thorms` | `core_rulebook/spell/level_5/wall_of_thorns` | 1,851 |

`wall_of_thorms` is the record cycle 1 named first among its "already-confirmed" gaps. Its APG row
(`apg_spells.lst:1555`) carries only `DOMAINS:Blood Subdomain=5` — no rule text of its own — and its
1,851 characters are the Core Rulebook *Wall of Thorns* paragraph, character-for-character. Those
words print.

*`absent_no_unit_anywhere` — all 10, not a sample:*

```
pathfinder_unchained/feat/champion_of_anarchy        pu_feats.lst:6   33 chars
pathfinder_unchained/feat/champion_of_balance        pu_feats.lst:7   55
pathfinder_unchained/feat/champion_of_destruction    pu_feats.lst:8   57
pathfinder_unchained/feat/champion_of_freedom        pu_feats.lst:9   50
pathfinder_unchained/feat/champion_of_grace          pu_feats.lst:10  50
pathfinder_unchained/feat/champion_of_malevolence    pu_feats.lst:11  58
pathfinder_unchained/feat/champion_of_righteousness  pu_feats.lst:12  89
pathfinder_unchained/feat/champion_of_tranquility    pu_feats.lst:13  41
pathfinder_unchained/feat/champion_of_tyranny        pu_feats.lst:14  49
mythic_adventures/spell/elemental_body_iiimod        ma_spells.lst:98 649
```

Each was additionally searched for by name across the whole inventory and found nowhere:
`Champion of Anarchy` → 0 units, `Champion of Balance` → 0 units (contrast `Combat Stamina` → 1,
`pathfinder_unchained:feat:combat_stamina`).

### The root cause of the 10 — one mechanism, not ten

`src/bin/v06_work_inventory.rs:3172-3173`, `has_classifying_token`:

```rust
Kind::Feat  => has_token(fields, "TYPE:"),
Kind::Spell => has_token(fields, "SCHOOL:") || has_token(fields, "CLASSES:"),
```

A row failing its kind's test is never enumerated; it lands in the `missing_classifying_token` trap.

| Evidence | Command |
| --- | --- |
| all 9 `pu_feats.lst` rows 6–14 carry `CATEGORY:FEAT`, `DESC:` and `BENEFIT:` and **no** `TYPE:` | `awk -F'\t' 'NR>=6&&NR<=14' "$PCGEN_CORPUS_ROOT/pathfinder/paizo/roleplaying_game/pathfinder_unchained/pu_feats.lst" \| grep -c 'TYPE:'` → `0` |
| all 8 `pu_feats.lst` rows the inventory **does** hold (18,19,20,25,26,27,29,32) carry `TYPE:` | same file, those lines → `8` |
| `ma_spells.lst:98` (`Elemental Body IIIMOD`) carries `DESC:` and `PREABILITY:` only — no `SCHOOL:`, no `CLASSES:` | `sed -n '98p' "$PCGEN_CORPUS_ROOT/.../mythic_adventures/ma_spells.lst" \| tr '\t' '\n'` |

Under the sheet rule all 10 would render as words — 9 short feat benefits and one 649-character
mythic augmentation paragraph. **This is not a carve-out and not an impossibility**; it is a
one-predicate enumeration gap with a named fix.

### The corpus-wide figure, corrected

| Statement | Value | Derivation |
| --- | --- | --- |
| the atlas's figure | `DONE 49,438 of 49,438` = 100% **of the inventory** | `python3 scripts/completion_atlas.py --check` → `population=49438 buckets=10 unclassified=0 overlap=0` |
| real corpus rules records | **48,864** | unchanged from cycle 1 |
| …reached by a DONE inventory unit | **48,854** | `48,609 + (255 − 10)` |
| **corpus-wide completion** | **48,854 of 48,864 = 99.9795%** | cycle 1 recorded 48,609 of 48,864 = 99.478% |
| records reaching no sheet line | **10** | `absent_total` |

Every inventory unit is DONE at HEAD (`population=49438`, `DONE=49438`), so "reached" and "reached
by a DONE unit" are the same set here — stated rather than assumed.

### What this proof does **not** cover

Recorded in the artifact as `what_this_proof_does_not_cover`, and repeated here because
`AGENTS.md §7` makes the omission the load-bearing part:

1. It does **not** re-verify the 2,657 records cycle 1 placed in the three not-a-record buckets
   (`chassis_only` 1,998, `duplicate_ingest` 612, `_settled` 47). Those keep cycle 1's disposition;
   this cycle's scope is the 255 remainder only. For information: the book-agnostic row join reaches
   **835** of the whole 2,912 never-reached population, of which 193 are this cycle's — consistent
   with duplicates naturally hitting their twin's row.
2. `reached_via_identical_prose_record` proves the **words** reach a sheet line; it does not prove
   the reached record is filed under the name a player would look up. `Threefold Aspect (Adulthood)`
   prints as part of `Threefold Aspect`.
3. Prose identity normalises whitespace and case **only**. That is deliberate — it means the test
   cannot silently launder a changed magnitude — but it also means a twin that reformats a table or
   re-punctuates a sentence compares different and stays counted as a gap. The 10 are therefore a
   **ceiling**, not a floor.

## Build scope verified

**No build was run this cycle, and none is load-bearing for its claim.** The cycle changes no Rust,
no `data/corpus` record, no `data/sheet_rules` file and no inventory unit — `closed=0
rust_lines_changed=0`, and `git diff --stat 9af5cd311e..HEAD -- src data docs/work-inventory.json` is
empty. Per `decisions.md §3` there is nothing for a build to prove. Cycle 1 ran
`cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0` at `83dfcbcfa1` over the identical `src/`; that
result stands unchanged at this HEAD by construction.

The instruments that **do** bear on the claim were all run at HEAD:

- `python3 scripts/completion_atlas.py --check` → `population=49438 buckets=10 unclassified=0 overlap=0`, `done_evidence_violations=0`, `missing_clearing_mechanisms=0`, `stale_derived_at=False`, `citation_failures=0` — **PASS**
- `python3 scripts/token_coverage.py --check` → `non_done=0 tokened=0 token_less=0 refused=142 refused_non_done=0 token_types=233 shapes=1 verdict=PASS`
- `python3 scripts/pcgen_residue_gate.py --check` → `live_files=0 live_hits=0 baseline_files=260 baseline_hits=12736 verdict=PASS` (run at cycle start **and** at HEAD; identical)
- `python3 scripts/shape_engine_boundary.py --check` → `magnitude_bearing=26396 not_held_by_engine=0 citation_ok=True`
- `python3 scripts/missing_engine_tables.py --check` → `population=0 kinds=0 citation_failures=0`
- `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l` → `0`
- `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-corpus-sheet-completion/*.md' 'docs/release/SD-35-corpus-sheet-completion/artifacts/**/*.md'` → `files_checked=163 violations=0`
- `scripts/verify.sh --only pi-sweep` → `RESULT: PASS`
- **Skipped, with the reason:** `cargo run --locked --bin corpus_literal_sweep` (runs only when
  corpus records changed — none did); `cargo run --locked --bin sheet_rule_convert` and
  `-- --check`, and the oracle comparison (run only when the converter, its mapping table or the
  corpus moved — none did; `data/sheet_rules/` is byte-identical to the cycle-start SHA);
  `cargo clippy` (no Rust target touched); the desktop crate and frontend (`apps/` untouched).

## Sweep population

N/A — no corpus record changed, so `corpus_literal_sweep` has nothing to re-examine.

## Oracle pin

`7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`; confirmed in the
checkout with `git -C "$PCGEN_REPO_DIR" rev-parse HEAD`). The pinned tree was read **read-only**, and
two figures *are* derived from it: the `.COPY=` pairing behind the 48, and the `TYPE:`/`SCHOOL:`
token counts behind the root cause of the 10. Both are stated with the command that re-derives them.

## Status

**partial.**

The criterion's deliverable is complete and its remainder is down from 255 records to 10 — but the population is **not
zero at HEAD**: **10 corpus records carrying published rules prose reach no sheet line**. Returning
`complete` would repeat, at one twenty-fifth the size, the exact error this cycle was dispatched to
correct.

- **Refused-token remainder, named and summing:** `record_absent_from_inventory_population=10`
  (`pathfinder_unchained/feat=9` + `mythic_adventures/spell=1` = **10**).
- **Not `blocked-escalated`:** nothing on `workflow-instruction.md §8`'s non-self-healable list
  fired. Clean tree, one writer, no gate regressed, `pcgen_live_files=0` did not rise, one refused
  token type (limit 10).
- **This is not a carve-out.** The 10 are ordinary feats and one mythic spell augmentation with
  printed text; each renders as words under the sheet rule. They are absent because one enumeration
  predicate requires a `TYPE:`/`SCHOOL:` token they do not carry — a population defect with a named
  fix, not an impossibility.

## Notes

The trap cycle 1 fell into is worth one line, because it is the same shape as memory's
`every-figure-states-its-denominator` rule pointed one level deeper: cycle 1 correctly avoided the
naive path→id join (12,659) and correctly used the converter's real join (2,912 → 255) — and the
converter's real join is still not the right instrument for *this* question, because it carries a
`book` label the enumerator rewrites on purpose. **Challenging the count twice and never challenging
the key is how 245 reached records got counted as lost.**

## Next-cycle scope

Criterion at **10**, not zero, and not 255.

1. Not expressible as `cycle_scope_gate.py` flags — the gate's population is
   `docs/work-inventory.json` and these 10 records are by definition outside it. A successor claims
   the `decisions.md §2` floor exemption or takes the whole 10 as its named remainder.
2. The fix is **one predicate**, not a book-onboarding job: widen `has_classifying_token`
   (`v06_work_inventory.rs:3172-3173`) so a `Kind::Feat` row carrying `DESC:`/`BENEFIT:` but no
   `TYPE:`, and a `Kind::Spell` row carrying `DESC:` but neither `SCHOOL:` nor `CLASSES:`, is
   enumerated. That requires write scope to `src/bin/v06_work_inventory.rs`, which this
   measure-do-not-fix criterion is explicitly denied, **and an operator ruling**, because it moves
   the atlas denominator off 49,438 and every bundle figure quoting it.
3. **`AT-35-E7-001`'s final-acceptance scan and SD-35's closure must quote the corrected pair**:
   `49,438 of 49,438` is 100% **of the inventory**, and the corpus-wide figure is **99.9795%**
   (48,854 of 48,864) — not the 99.478% cycle 1 recorded. Cycle 1's receipt is left intact as
   history; its artifact carries a `cycle_2_correction` pointer.
