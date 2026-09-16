# Cycle 9 — Epic 6 (PCGen exit) / AT-35-E6-003-SWEEP

Cycle 7 measured the companion `description_variants` conditions, found them the
same shape it had just converted for feats, and **refused them for a mechanism**:
`gen_book_cache.rs:1938` serialises `v.conditions` straight into the book cache
that `AT-35-E6-002`'s Evidence pins byte-identical. That refusal was one level
too shallow. The wire format carries the ingest string; the *live table* does not
have to. This cycle types the guards on the live side and rebuilds the wire
string **on the converter side**, so the cache is byte-identical by construction
and by proof. Two more mechanisms went with it: the Unchained Rogue and Summoner
were printing `TYPE=Craft, TYPE=Perform, TYPE=Profession` into a shipped
`explanation` a player reads, and five `description:`/`benefit:` strings ended
with an ingest tail glued to the sentence. **511 → 458 code hits, 53 cleared,
5 files to zero, none risen.**

- **Commit SHA:** `f4583db504` carries the cycle's work; this line is written
  into the immediately following docs commit (a receipt cannot name the commit
  that carries it). Cycle start `319918f7c19dbd6e8a0bbff6c350d4db674f4aab`.

- **Cycle number:** this is cycle **9**, not the 8 the dispatch prompt named.
  Cycle 8 was already committed at `1f425d5124` with its receipt tracked at HEAD
  (`319918f7c1`), so writing a `cycle8` receipt would have overwritten a landed
  one. Recorded as `correction 1789171411293-at-35-e6-003-sweep-c356ea`. The
  *remainder* the prompt named (`BONUS:=128; TYPE==122; PRE[A-Z]+:=108;
  DESC:=66; render_pcgen_desc=39; %LIST=29; %CHOICE=13; raw_tokens=5;
  DEFINE:=1`) is cycle 8's **end** state, and `pcgen_residue_gate.py --check`
  reproduced `live_files=64 live_hits=511` exactly at cycle start.

- **Scope gate:** `SCOPE_GATE: EXEMPT (Epic 6 cycle — closes zero corpus units
  by design, decisions.md §2)`. Run anyway, for the record —
  `python3 scripts/cycle_scope_gate.py --min 500`:
  ```
  inventory=docs/work-inventory.json
  scope=(whole remainder)
  scoped_by_bucket=
  scoped_by_kind=
  scoped=0 remaining_non_done=0 floor=500 verdict=PASS_WHOLE_REMAINDER
  ```
  This criterion's **own** floor — the 500-code-hit currency cycle 1 established
  — is **missed**: **53 code hits cleared of the 500 the floor asks for**
  (511 → 458). The scope taken was the **whole remainder** (511 hits in 64
  files, examined mechanism by mechanism), which is what the floor rule requires
  of a cycle that cannot reach 500 any other way. The shortfall is named with
  its cause under **Refused tokens**: **352 of the 458 that remain are behind an
  operator ruling asked for by cycles 5 and 6 and still open**, and no code work
  of any size reaches them.

- **Files touched:**
  - `src/rules_core/rules_tables/companion_chassis.rs` — the chassis.
    `CompanionDescriptionVariant.conditions` and the new
    `NaturalAttackDamageBonus.conditions` become `&'static [EffectCondition]`;
    `CompanionClassRecord.ability_grants` becomes `&'static
    [CompanionAbilityGrant]` (a new type: `kind` / `mode` / `name` /
    `conditions`). `EffectCondition` and `ConditionItem` are **re-exported from
    `crb::feats`, not re-declared** — cycle 7 already owns that vocabulary and a
    companion guard is not a different grammar from a feat guard.
  - 13 `rules_tables/**/companion_data.rs` files — generated output,
    regenerated. 30 guards converted; every `NaturalAttackDamageBonus` literal
    gains `conditions` (`&[]` on all but three).
  - `src/pcgen_import/companion_pcgen_guards.rs` — **new**, generated. The 30
    verbatim tails addressed by (book, record key, field, index), the public
    `rebuild_condition`, and two round-trip tests.
  - `src/bin/gen_book_cache.rs` — `"conditions"` is now rendered from the typed
    form through `rebuild_condition`. Converter-side; the wire format is
    unchanged.
  - `src/rules_core/rules_tables/pathfinder_unchained/{rogue,summoner}_features.rs`
    — `class_skills()` returns `&'static [ClassSkillEntry]` (cycle 7's type,
    reused); both corpus-pinning tests retyped, both counts (21 / 9 / 3
    families) unmoved.
  - `src/rules_core/pilot_compute/mod.rs` — new `render_class_skill_list`, and
    the two shipped `explanation.detail` strings that were printing `TYPE=Craft`
    now print *"every Craft skill"*.
  - `src/rules_core/rules_tables/{ultimate_combat,ultimate_magic}/feat_tables.rs`,
    `{acg,ultimate_magic,ultimate_psionics}/archetype_tables.rs` — five prose
    strings lose their ingest tail.
  - `src/pcgen_import/prose_ingest_tails.rs` — **new**, generated. Those six
    rows (five guards plus one `%LIST` selection slot), prose half and tail half,
    with two tests.
  - `src/pcgen_import/mod.rs` — registers the two new modules.
  - `docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle9_type_companion_guards.py`
    and `…_cycle9_prose_tail_relocate.py` — **new**, the two converters and their
    `--check` modes.
  - `docs/release/SD-35-corpus-sheet-completion/` — this receipt, `progress.md`,
    `kanban.md`; `docs/retro/events/at-35-e6-003-sweep.jsonl`.
  - Folded from the shared checkout, not this cycle's own work:
    `docs/retro/events/root.jsonl` (one event-log append).

- **Identifier audit result:** **OK_NO_BUNDLE_TAGS.** Run on the cycle's own
  diff (`workflow-instruction.md §6` steps 2 and 4):
  ```bash
  git diff --unified=0 -- src/rules_core src/pcgen_import src/bin \
      apps/desktop/src-tauri/src tests docs/release/SD-35-corpus-sheet-completion \
      ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '^\+.*\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'
  ```
  → no match; the literal `OK_NO_BUNDLE_TAGS` fallback printed, before and after.

- **Wired-integration audit result:** **OK_NO_TOKENS.** Same diff,
  `grep -nE '^\+.*\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`
  → no match, before and after.

- **Acceptance criterion** (verbatim, `epic-breakdown.md` — this sweep criterion
  carries Epic 6's own closure bar, `AT-35-E6-004`):

  > ### AT-35-E6-004 — the gate reads zero
  >
  > **"Zero" means zero CODE hits** — operator ruling B14, 2026-09-11
  > (`decisions.md §17`). […]
  >
  > **Evidence:** `python3 scripts/pcgen_residue_gate.py --check --closure` →
  > `live_files=0 live_hits=0 verdict=PASS` […]

  **Not met.** `live_files=59 live_hits=458` at HEAD. The remainder is named and
  summing under **Refused tokens**.

- **Receipt rows (mechanical):**
  ```
  since=319918f7c19dbd6e8a0bbff6c350d4db674f4aab target_dir=/tmp/cargo-sd35-AT-35-E6-003-SWEEP residue_gate=present
  closed_by_kind=
  relabeled_moves=
  regressed=0 added=0 dropped=0
  closed=0 relabeled=0 rust_lines_changed=754 ratio=n/a builds_recorded=2 pcgen_live_files=59
  ```
  `closed=0` is correct and by design: Epic 6 moves no corpus unit.
  `pcgen_live_files` falls **64 → 59** — five files reached zero.

- **PCGen residue:** `python3 scripts/pcgen_residue_gate.py --check`, at end of
  cycle:
  ```
  pattern raw_tokens files=1 hits=5
  pattern raw_bonus_chains files=0 hits=0
  pattern PcgenFormulaEvaluator files=0 hits=0
  pattern render_pcgen_desc files=3 hits=39
  pattern bonus_stack_reader files=0 hits=0
  pattern pre_tokens files=0 hits=0
  pattern BONUS: files=17 hits=128
  pattern DEFINE: files=1 hits=1
  pattern PRE[A-Z]+: files=20 hits=71
  pattern SAB: files=0 hits=0
  pattern DESC: files=27 hits=66
  pattern %CHOICE files=3 hits=13
  pattern %LIST files=6 hits=28
  pattern TYPE= files=23 hits=107
  root src/rules_core files=58 hits=451
  root src/saved_character files=0 hits=0
  root src/campaign files=0 hits=0
  root src/homebrew_authoring files=0 hits=0
  root apps/desktop files=1 hits=7
  identifier_files=4 identifier_hits=44
  live_files=59 live_hits=458 baseline_files=260 baseline_hits=12736 verdict=PASS
  ```
  Never above the previous receipt's `live_files=64 live_hits=511`. The count
  fell **only** because tokens left the live side:
  `scripts/pcgen-residue-baseline.env` was **not** edited, `--rebaseline` was
  **not** run, and no pattern, root or exclusion was touched.

- **Oracle parity:** **N/A — no `Number` mapping was added and no converted rule
  moved.** `data/corpus/` was not written and `data/sheet_rules/` was not
  written; `sheet_rule_convert --check` reproduces cycles 3–8's line field for
  field (below), which is the standing proof. `PCGEN_ORACLE_SHA` unchanged and
  `scripts/pcgen-oracle-pin.env` untouched.

- **Movement, four buckets:**
  - **closure:** none by corpus unit (Epic 6 closes none). By this criterion's
    own currency: **53 code hits** across 14 files, 5 of them to zero. The
    test-region census separates code work from reclassification:
    `hits_outside` falls **150 → 106** (44) and `hits_inside_cfg_test` falls
    **361 → 352** (9). **That 9 is named, not quietly banked:** it is the
    corpus-pinning assertions in `companion_chassis.rs`,
    `rogue_features.rs` and `summoner_features.rs`, which asserted *on the
    ingest string* and now assert on the typed fields that carry the same fact —
    the same assertions, re-expressed, with `pcgen_import::companion_pcgen_guards`
    holding the string verbatim and proving the two agree. 44 + 9 = 53.
  - **relabel:** none.
  - **reachability:** none.
  - **instrument-correction:** none. The gate was not touched.

- **Per-file movement — every file that moved, summing to 53.** Re-derived by
  applying the gate's own regexes (comment lines excluded) to
  `git show 319918f7c1:<file>` and to the file at HEAD, over the union of every
  live-root source file at either tree. **No file rose.**

  | file | before | after | cleared |
  |---|---:|---:|---:|
  | `rules_tables/ultimate_wilderness/companion_data.rs` | 17 | **0** | 17 |
  | `rules_tables/pathfinder_unchained/rogue_features.rs` | 12 | 4 | 8 |
  | `rules_tables/ultimate_magic/companion_data.rs` | 8 | **0** | 8 |
  | `rules_tables/pathfinder_unchained/summoner_features.rs` | 5 | **0** | 5 |
  | `rules_tables/crb/companion_data.rs` | 6 | 3 | 3 |
  | `pilot_compute/mod.rs` | 58 | 56 | 2 |
  | `rules_tables/companion_chassis.rs` | 6 | 4 | 2 |
  | `rules_tables/ultimate_combat/feat_tables.rs` | 3 | 1 | 2 |
  | `rules_tables/acg/archetype_tables.rs` | 4 | 3 | 1 |
  | `rules_tables/bestiary_4/companion_data.rs` | 1 | **0** | 1 |
  | `rules_tables/bestiary_6/companion_data.rs` | 1 | **0** | 1 |
  | `rules_tables/ultimate_magic/archetype_tables.rs` | 3 | 2 | 1 |
  | `rules_tables/ultimate_magic/feat_tables.rs` | 5 | 4 | 1 |
  | `rules_tables/ultimate_psionics/archetype_tables.rs` | 3 | 2 | 1 |
  | | | | **53** |

- **What the player's sheet says now.**

  | was, shipped verbatim | is |
  |---|---|
  | `Unchained Rogue class skills, verbatim from the book's own CSKILL: token (21 entries): Acrobatics, …, TYPE=Craft, …, TYPE=Perform, TYPE=Profession, …` | `Unchained Rogue class skills, as the book's own class-skill list states them (21 entries): Acrobatics, …, every Craft skill, …, every Perform skill, every Profession skill, …` |
  | `…this class gets every Knowledge skill…` with `TYPE=Knowledge` in the list | `…every Knowledge skill` in the list itself |
  | `You gain a +4 bonus on driving checks with your chosen vehicle (chosen vehicle: %1).\|%LIST` | `You gain a +4 bonus on driving checks with your chosen vehicle.` |
  | `…rip magical defenses from your enemy.  PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike` | `…rip magical defenses from your enemy.` |
  | `…effective cleric level were 2 levels lower than normal.\|PREABILITY:1,CATEGORY=Special Ability,Versatile Channeler ~ Positive Energy` | `…effective cleric level were 2 levels lower than normal.` |
  | `…a +%1 enhancement bonus to her base speed.\|BeastmorphSpeed\|PREVAREQ:BeastmorphProgression,1` | `…a +%1 enhancement bonus to her base speed.` |

- **Refused tokens:** `BONUS:=128; TYPE==107; PRE[A-Z]+:=71; DESC:=66; render_pcgen_desc=39; %LIST=28; %CHOICE=13; raw_tokens=5; DEFINE:=1` — **458
  code hits in 59 files** (the per-pattern lines double-count a line matching
  two patterns; the gate's deduplicated `live_hits` is 458). Nine token types,
  under `workflow-instruction.md §8`'s limit of ten. By mechanism, largest
  first — re-derived at HEAD, not carried forward:

  | mechanism | files | code hits | shape |
  |---|---|---:|---|
  | hits inside a `#[cfg(test)]` module of a live file | 37 files are test-only | **352** | **Unmoved by any cycle that does not first get a ruling.** The residue gate scans whole files, so an assertion inside a live file counts while the identical assertion in `tests/` does not. Asked for by cycles 5 and 6, still open — see `progress.md`'s `## Open blockers`. **77% of what remains.** (The 9 this cycle cleared here were re-expressions of the conversion it was already making, not an attack on this mechanism, which needs the ruling.) |
  | `pilot_compute/mod.rs` unframed prose citation | 1 | 36 | A token named mid-sentence inside a shipped `explanation` string with no bounded frame. Three rules for these were built, measured against the real files and **removed** across cycles 5 and 6 for producing ungrammatical sheet prose. Hand work, record by record. |
  | `bestiary/monster_data.rs` `%CHOICE`/`%LIST` description slots | 1 | 20 | `description_variables: &["%CHOICE"]`. Cycle 8 measured and refused this for a mechanism and the measurement still holds: `&'static [&'static str]` with **4,378 literals repo-wide**, serialised into the book cache by `gen_book_cache.rs:1610` and `:1926`. A wire-format cycle. |
  | `src/rules_core/pcgen_desc.rs` + its two callers | 3 | ~7 outside test modules (45 in the file) | The live prose renderer `AT-35-E6-003` says to delete. Replacement exists and is proven (`sheet_rule_catalog::catalog_description_or_fields`); blocked on `class_feature_pool_catalog.rs` reading the sheet-rule package instead of the ingest cache. |
  | `race_trait_picker.rs` `raw_tokens` on the desktop side | 1 | 7 | Real `PREVAREQ`/`PREMULT`/`PREABILITY`/`!PREFACT` parsing deciding alternate-trait exclusion. Needs `SheetRule.applies` to carry the exclusion-guard relation; relocating the parse would launder the read, not end it. |
  | `derived_evaluator_fixture_check.rs` / `support_state_matrix.rs` diagnostic text | 2 | 11 | Assertion-failure and reason strings naming a token, outside any `#[cfg(test)]` region. Same unframed-prose shape as `pilot_compute/mod.rs`. |
  | `FeatEffectBonus.qualifiers` selection targets (`WEAPONPROF=%LIST`, `SCHOOL.%LIST`) | 3 | 11 | The bonus *target* is the player's chosen weapon/school. Cycle 7 typed the qualifier **tail**; this is the target the bonus engine matches on, so it moves `damage_total` / `skill_allocation` / `pilot_compute` matching, not a literal. |
  | `external_ability_refs` `!PRETEMPLATE:` tails | 1 | 3 | The same conversion this cycle made for `conditions`, **deliberately left**: `companion_catalog.rs:585` clones this field onto the desktop wire, so it pulls `apps/` in and with it the desktop crate + frontend suites, which this epic runs at wrap-up cadence, not per cycle. It is 3 hits and one `.map()`. |
  | `bestiary_3/monster_data.rs` `DESC:&nl;` markers, and the rest | ~12 | ~11 | Single `DESC:`/`PRE`/`TYPE=` citations scattered across book `mod.rs` and table files; the `bestiary_3` four are ingest line-break markers inside a shipped monster description, regenerable only through `transcribe_monster_tables.py`. No shared frame, so no single rule reaches them. |

- **Discoveries:** **one, and it is a correction of a refusal.** Cycle 7 refused
  the companion `conditions` family on the ground that `gen_book_cache.rs:1938`
  serialises `v.conditions` into a cache `AT-35-E6-002` pins byte-identical.
  That is true and it is not a blocker: the **wire format** must carry the
  ingest string, the **live table** need not, and the two are joined by a
  function that can live on either side. Moving `rebuild_condition` to
  `src/pcgen_import/` makes the cache byte-identical *by construction*, and the
  round-trip test makes it byte-identical *by proof*.
  **The lesson generalises:** when a conversion is refused because a downstream
  consumer needs the old form, ask which side the *rendering* belongs on before
  concluding the *storage* cannot move. A serialiser is not a reader.
  (`correction 1789171411441-at-35-e6-003-sweep-ae1dec` records the second,
  smaller finding: `NaturalAttackDamageBonus.formula`'s doc comment promised
  "the token's trailing formula half" and three rows stored a `PRE` guard there.)

- **Figures + their re-derive commands:**

  | figure | denominator | command |
  |---|---|---|
  | `live_files=59 live_hits=458` (end) | all source files under the five live roots, comment lines excluded | `python3 scripts/pcgen_residue_gate.py --check` |
  | `live_files=64 live_hits=511` (start) | same, at `319918f7c1` | same command, at that tree |
  | `53` cleared, `5` files to zero, `0` risen | of the 511 the gate counted at cycle start, over the 64 files carrying them | `511 - 458`; per-file from the gate's own regexes applied to `git show 319918f7c1:<file>` against the working file, over the union of live-root source files at both trees |
  | `hits_outside` 150 → 106, `hits_inside_cfg_test` 361 → 352 | the live files carrying hits | `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle5_test_region_census.py` (unchanged tool, run at both trees) |
  | `30` companion guards converted, `13` files changed | every `conditions` / `ability_grants` / `NaturalAttackDamageBonus.formula` entry carrying a `PRE<FAMILY>:` token, across all 16 `companion_data.rs` | `python3 …/AT-35-E6-003-SWEEP_cycle9_type_companion_guards.py --check` → `companion_files=16 files_changed=0 guards_converted=0` (converted; a non-zero `files_changed` means work remains) |
  | `6` prose rows relocated | the five guard tails plus one `%LIST` selection slot found in shipped `description:`/`benefit:` strings | `python3 …/AT-35-E6-003-SWEEP_cycle9_prose_tail_relocate.py --check` → `rows=6 rows_changed=0` |
  | class-skill counts unmoved at `21` / `9` / `3` families | the two PU `class_skills()` lists | `cargo test --locked --lib class_skill_list_is_the_verbatim_cskill_row` |
  | `0` files in `data/sheet_rules/` carrying ingest vocabulary | all of `data/sheet_rules/` | `grep -rlE 'BONUS:\|DEFINE:\|PRE[A-Z]+:\|%CHOICE\|CL=' data/sheet_rules/ \| wc -l` → 0 |
  | `rust_lines_changed=754`, `pcgen_live_files=59` | the cycle's own diff | `python3 scripts/cycle_scope_gate.py --receipt --since 319918f7c1… --before /tmp/wi-before-AT-35-E6-003-SWEEP.json --after docs/work-inventory.json` |

- **Build scope verified:** run **once**, at the end, after the last
  figure-moving edit.
  - `cargo test --locked --no-run -j 6` → `NO_RUN_EXIT=0`.
  - `cargo test --locked --lib -j 6` → `test result: ok. 3322 passed; 0 failed;
    15 ignored` (`LIB_EXIT=0`). 3,318 → 3,322 is exactly the four new tests.
  - `cargo test --locked --no-fail-fast -j 6` → `**414 targets (413 test binaries + 1 doc-test target), 8,840 passed, 0 failed, 68 ignored, 0 `test result: FAILED` lines**, `FULL_EXIT=0`. 8,836 -> 8,840 is exactly the four new tests.`.
  - `cargo clippy --locked --tests -j 6` → ``CLIPPY_EXIT=0`, 0 warnings.`.
  - `python3 scripts/pcgen_residue_gate.py --check` → `verdict=PASS`, not risen
    (fell by 53).
  - `cargo run --locked --bin sheet_rule_convert -- --check` → ``records=49438 converted=49296 refused=142 rules=70135 var_tables=5293 verdict=PASS (114.7s)` -- identical to cycles 3-8 on every field; the 142 refusals are all `no_corpus_record`. This is the standing proof that no converted rule moved.`.
  - `grep -rlE 'BONUS:|DEFINE:|PRE[A-Z]+:|%CHOICE|CL=' data/sheet_rules/ | wc -l`
    → `0`.
  - `python3 scripts/completion_atlas.py --check` → `done_evidence_violations=0
    missing_clearing_mechanisms=0 stale_derived_at=False citation_failures=0`,
    `EXIT=0`.
  - `python3 scripts/token_coverage.py --check` → `non_done=0 … token_types=233
    shapes=1 verdict=PASS`, `EXIT=0`.
  - `python3 scripts/shape_engine_boundary.py --check` → `magnitude_bearing=26396
    not_held_by_engine=0 citation_ok=True`, `EXIT=0`.
  - `python3 scripts/missing_engine_tables.py --check` → `population=0 kinds=0
    citation_failures=0`, `EXIT=0`.
  - `python3 scripts/denominator_gate.py --check 'docs/release/SD-35-…/*.md'
    'docs/release/SD-35-…/artifacts/**/*.md'` → `files_checked=117
    violations=0`.
  - `scripts/verify.sh --only pi-sweep` → `RESULT: PASS`.
  - **Desktop crate and frontend: not run — epic cadence.** This cycle touched
    **no** file under `apps/`; `git diff --stat 319918f7c1..HEAD -- apps/` is
    empty. That is also why the 3 `external_ability_refs` hits were left.
  - `v06_work_inventory` and `corpus_literal_sweep`: **not run.** No corpus
    record and no classifier changed; `docs/work-inventory.json` is
    byte-identical to `/tmp/wi-before-AT-35-E6-003-SWEEP.json` (`diff -q` →
    identical; `regressed=0 added=0 dropped=0` agrees).
  - **`gen_book_cache` was not re-run, and does not need to be.** Its
    `"conditions"` output is now `v.conditions.iter().map(rebuild_condition)`,
    and `rebuild_condition` is the exact inverse of the parse the converter
    used — proved for all 30 guards by
    `every_converted_guard_round_trips_from_the_live_typed_form`, which compares
    the rebuilt strings against the verbatim originals as a sorted multiset.
    `natural_attack_damage_bonuses` and `ability_grants` are **not serialised at
    all** (`grep -n 'natural_attack_damage_bonuses\|ability_grants'
    src/bin/gen_book_cache.rs` → no match), so their field changes cannot reach
    the wire.

- **Sweep population:** N/A — no corpus record changed.

- **Oracle pin:** `PCGEN_ORACLE_SHA` unchanged; no figure in this receipt came
  from the pinned corpus (every guard and every prose string was read out of the
  shipped tables, which carry their own source-file and source-line provenance).

- **RED→GREEN preserved, recorded.** Both new round-trip tests were run against
  a deliberately corrupted table and failed for the intended reason:
  ```
  ---- pcgen_import::companion_pcgen_guards::tests::every_converted_guard_round_trips_from_the_live_typed_form ----
  assertion `left == right` failed: … the conversion lost or invented something
    left:  …("core_rulebook", "Animal Companion Feat ~ Toughness", "conditions", "PREHD:MIN=3")…
    right: …("core_rulebook", "Animal Companion Feat ~ Toughness", "conditions", "PREHD:MIN=4")…

  ---- pcgen_import::prose_ingest_tails::tests::every_row_carries_prose_on_one_side_and_ingest_vocabulary_on_the_other ----
  Skilled Driver: the kept prose still carries ingest vocabulary:
    "You gain a +4 bonus on driving checks with your chosen vehicle.|%LIST"
  ```
  The companion test had already caught a real defect before that, unprompted:
  the generator keyed guards by the `companion_data.rs` **directory** name
  (`crb`) while `COMPANION_BOOKS` keys them by `corpus_book`
  (`core_rulebook`), and the first full-suite run went red on exactly those
  three rows. The generator now derives the mapping from
  `companion_chassis.rs`'s own registry, so a fourth divergence cannot appear
  silently. Recorded as `rework 1789174161873-at-35-e6-003-sweep-deb79d`; the
  suite was re-run from `--no-run` after the fix, and the figures above are all
  from that second, clean run.

- **Retro events emitted:**
  `correction 1789171411293-at-35-e6-003-sweep-c356ea` (the dispatch's cycle
  number), `correction 1789171411441-at-35-e6-003-sweep-ae1dec`
  (`NaturalAttackDamageBonus.formula`'s doc comment),
  `rework 1789174161873-at-35-e6-003-sweep-deb79d` (the book-name keying
  defect), `deferral 1789174137971-at-35-e6-003-sweep-d84727` (the 458-hit remainder, mechanism by
  mechanism).

- **Status: `partial`.** The criterion's population is not zero at HEAD.

- **Notes:** cycle 7's refusal was correct about the constraint and wrong about
  what the constraint forbade — the one judgment call here was deciding a
  serialiser is not a reader, and the round-trip test is what makes that
  decision checkable rather than asserted.

- **Next-cycle scope:** `SCOPE_GATE: EXEMPT (Epic 6 cycle)`; the whole
  remainder, **458 code hits in 59 files**. The 352 behind the `#[cfg(test)]`
  ruling cannot move without it, so the reachable batch is **106 hits in 22
  files**, and it is now dominated by two things no rule reaches: 36
  `pilot_compute/mod.rs` unframed prose citations plus 11 more of the same shape
  in `derived_evaluator_fixture_check.rs` / `support_state_matrix.rs` (47 hits
  of hand work, record by record), and 20 `bestiary/monster_data.rs`
  `description_variables` slots (a book-cache wire-format cycle, the same one
  cycle 8 named). The cheap remainder is **3 `external_ability_refs` hits plus
  the desktop suites** — worth folding into whichever cycle next touches
  `apps/`, which is where `race_trait_picker.rs`'s 7 live too.
