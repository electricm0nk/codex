# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-003 (bucket `M` — EQUIPMENT sub-causes, cycle 4)

- **Commit SHA:** `3822c0c1d8` (source fix + tests, checkpointed and pushed before this receipt;
  this receipt's own commit follows on top, same pattern cycles 2/3 used).

- **Continuation of, not a duplicate of,** `AT-34-E3-003_m_bucket_equipment_cycle_receipt.md`
  (cycle 1, `7147fd86ab`), `_2.md` (cycle 2, `0519220786`/`3ffa80cc20`), `_3.md` (cycle 3,
  `ac1cd80dfc`), all already merged into `tranche/14` before this cycle started. Rebase base at
  cycle start: `origin/tranche/14`'s tip, `adcbbc8695` (`docs(sd34): AT-34-E3-002 cycle 6 --
  fill final commit SHAs in receipt`), fast-forwarded cleanly. This wave's dispatch brief
  carried STALE figures (`core_rulebook` M = 972, split 276+147 = 423) inherited from BEFORE
  cycle 3 landed — every figure below was independently re-derived at the real rebase base
  before any code was read or written, per `decisions.md §12` L2.

- **Re-derived at cycle start (never trusting the inherited figure):**
  `python3 scripts/completion_atlas.py --book core_rulebook --check` at the rebase base →
  `core_rulebook` M = **944** (matches cycle 3's own post-fix figure exactly, live-confirmed).
  Split: `equipment_table_entry_with_corpus_magnitude` **249**,
  `ability_content_table_holds_record_magnitude_not_yet_computed` 217 (sibling lane, off
  limits), `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does` **146**,
  `race_trait_generic_table_holds_record_magnitude_not_yet_computed` 119 (sibling lane, off
  limits), `template_content_table_holds_record_magnitude_not_yet_computed` 96 (sibling lane,
  off limits), `in_catalog_with_corpus_magnitude_but_no_observed_consumer` 47,
  `domain_content_table_holds_record_magnitude_not_yet_computed` 34,
  `skill_content_table_holds_record_magnitude_not_yet_computed` 19,
  `spell_list_entry_with_resolved_level` 15,
  `race_trait_states_a_universal_sheet_modifier_pending_compute` 2. Sum = 944, confirmed. **This
  cycle's territory (the same two EQUIPMENT sub-causes) is 249 + 146 = 395** at cycle start —
  cycle 3's own stated remainder, confirmed exactly.

- **What this cycle found, that cycles 1–3 did not: `general::compute_var_effect` /
  `general::apply_eqmod_var_bonus` already existed — fully unit-tested, and already
  oracle-validated by SD-33's own `src/bin/e5_var_shape_ours.rs` round-trip comparison against
  the pinned PCGen oracle — but were never called from `compute_equipment_effects`, the real
  dispatcher every other category resolver (`arms_armor`/`general::compute_general_effect`/
  `magic_items`/`equipmods`) is wired into.** Confirmed by reading every call site
  (`grep -rn 'compute_var_effect\|apply_eqmod_var_bonus'`): both functions were reachable ONLY
  from four SD-33 oracle-comparison bin tools (`e5_eqm_final_ours.rs`, `e5_var_shape_ours.rs`,
  `e5_disagreement_fixes_ours.rs`, `e6_identity_rerun_ours.rs`) and their own `#[cfg(test)]`
  module — genuinely dead code with respect to the production compute path, despite already
  being correct (its own unit tests: `climbers_kit_has_no_var_bonus`,
  `muleback_cords_yields_a_single_loadscore_var_bonus`,
  `eqmod_referenced_material_var_chain_sums_into_the_base_items_var_bonus`, all passing before
  this cycle touched anything). Cycle 2's own 9-shape census had classified this same
  population as `VAR cross-subsystem, new engineering, not closable by existing instrument` —
  **that classification was wrong, and is corrected here**: this is the SAME shape as cycles
  1 and 3's own findings (`BASEITEM:`/`TEMPBONUS:` widening), a real compute path that already
  existed and only needed to be consulted, not a new mechanism.

- **Fix (same shape as cycles 1 and 3's own precedent — consult a real, already-computed field
  the dispatcher never read, not a new subsystem):**
  - `src/rules_core/equipment_effects.rs` — `ResolvedEquipmentEffect` gains
    `var_bonus: Vec<VarBonus>` (same zero-vs-honest-absence convention every sibling field
    already uses). `compute_equipment_effects` now calls `general::compute_var_effect(record)`
    then `general::apply_eqmod_var_bonus(&mut var_bonus, &weapon_eqmod_records)` — reusing the
    ALREADY-RESOLVED `weapon_eqmod_records` (despite its name, `eqmod_referenced_records`'s
    generic, category-agnostic result, already computed for the weapon-enhancement dimension a
    few lines above) rather than a second EQMOD resolution pass.
  - `src/bin/v06_work_inventory.rs` — `equipment_key_is_wired`'s probe gains
    `|| !item.var_bonus.is_empty()`. Three new tests: a real on-disk positive (Amulet of Mighty
    Fists, `BONUS:VAR|MightyFistValue|5` on its own line), a hand-built positive proving the
    EQMOD-referenced-chain-summed-by-name shape `apply_eqmod_var_bonus`'s own doc comment names
    (`panoply_of_the_fierani_knight`'s real disagreement, SD-33 remediation wave 4), and a
    negative control (no `VAR` chain, no other checked field → stays unwired).
  - This receipt, `docs/release/SD-34-book-completion/progress.md`,
    `docs/release/SD-34-book-completion/kanban.md`.
  - **Deliberately NOT touched/committed:** `docs/work-inventory.json` and
    `artifacts/epic-1-atlas/completion-atlas.json` — reserved to the shared end-of-wave
    regeneration, same rule cycles 1–3 followed. Figures below come from a local three-pass
    regen (`corpus_literal_sweep --json-out` → `derived_evaluator_fixture_check --json-out` →
    `CORPUS_LITERAL_SWEEP_REPORT=… DERIVED_FIXTURE_CHECK_REPORT=… cargo run --locked --release
    --bin v06_work_inventory`, no `--allow-stamp-loss`), read via a whole-corpus id-diff against
    the committed file, then `git restore`-d before this cycle's commit.

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — `git diff --unified=0 HEAD -- src/rules_core/
  src/bin/ | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` → zero matches on this
  cycle's own diff. (The `§6`-mandated `merge-base HEAD origin/develop` scope instead surfaces
  ~28 hits, all pre-existing `sd13_*`/`sd25_*` real test-name references and domain-legitimate
  "placeholder" prose already shipped by OTHER already-merged cycles since `tranche/14` branched
  from `develop` — confirmed zero on this cycle's own diff, the narrower scope every prior
  equipment cycle's receipt used for the same reason.)

- **Wired-integration audit result:** `OK_NO_TOKENS` — same command, second pattern
  (`\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b`) → zero matches on this
  cycle's own diff.

- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "AT-34-E3-003 — buckets M, V, D, U, X
  close" (per-bucket, atlas reporting zero for `core_rulebook`, movement in four buckets). This
  cycle is a further slice of the same territory cycles 1–3 worked — the criterion as a whole
  stays open (M's other sibling sub-causes and buckets V/D/U/X are untouched by design, per this
  wave's no-collision territory boundary).

- **Figures + their re-derive commands (post-fix, from this cycle's own local three-pass
  regen):**
  - `core_rulebook` bucket `M`: **944 → 812** (**−132, this cycle's own closure**). Re-derive:
    `python3 scripts/completion_atlas.py --book core_rulebook --check` (against a fresh local
    regen — the committed `docs/work-inventory.json` at HEAD still reads 944 until the wave's
    shared regen cycle folds this cycle's source change in).
  - `core_rulebook` `equipment_table_entry_with_corpus_magnitude`: **249 → 164** (−85).
  - `core_rulebook` `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does`:
    **146 → 99** (−47). **Unlike cycles 2 and 3, this fix DOES reach the closure-only shape** —
    every one of the 47 closures on this evidence string carries a `BONUS:VAR` chain the
    classifier's own narrower `text_only`/`magnitude_token_count` check did not recognise as a
    magnitude (see Notes below), which `wiring_class`'s fuller signal aggregation had already
    correctly flagged as `computed`. This retires cycles 2/3's own repeated finding that "one
    fix does not cover both shapes" — for the `VAR` token family specifically, it does.
  - **Whole-corpus id-diff by unit id, this cycle's own local regen vs. the committed
    `docs/work-inventory.json` at the rebase base (49,438 → 49,438, 0 added, 0 removed, 337
    changed):**
    - **256 `equipment`/`equipment_modifier`, `ingested-magnitude → grounded`
      (`equipment_effect_probe_observed_computed_delta` evidence) — every one of these is THIS
      CYCLE'S OWN closure**, corpus-wide, across **9 books**: 132 `core_rulebook` (85 same-line
      + 47 closure-shape), 78 `mythic_adventures`, 16 `ultimate_equipment`, 13
      `ultimate_psionics`, 11 `advanced_players_guide`, 2 `advanced_race_guide`, 2
      `book_of_the_damned_volume_2`, 1 `advanced_class_guide`, 1 `inner_sea_gods`. Sum:
      124+78+11+8+8+8+7+6+2+1+1+1+1 = **256**, matching the `core_rulebook` M delta (−132) plus
      every other book's own M delta exactly.
    - **46 `equipment_modifier`, `text-complete → grounded`** — a pure **DONE→DONE
      reclassification**, NOT a bucket move (both statuses map to bucket `DONE`): records that
      were already correctly `text-complete` (their own line carries no magnitude and no real
      prose either) now also carry a real `var_bonus` reached via an `EQMOD:`-referenced
      record, so `observed()` fires before the `text_only` fallback ever runs and stamps the
      stronger `equipment_effect_probe_observed_computed_delta` evidence instead. 27
      `core_rulebook`, 13 `mythic_adventures`, 4 `ultimate_psionics`, 1 `advanced_class_guide`,
      1 `advanced_race_guide` — this cycle's own side effect, not claimed as closure.
    - **32 `class_feature`, `engine-does-not-hold → grounded`, NOT this cycle's work** —
      co-mingled, already-committed `AT-34-E3-002` cycle 6's own domain-header display-record
      closure (landed `b713035f2b`/`41918cccb7`, evidence
      `no_explanation_id_and_no_diagnostic_names_this_feature →
      generic_pool_group_selection_probe_observed_a_real_computed_magnitude_for_the_display_record`/
      `domain_power_probe_observed_a_real_computed_magnitude_for_the_display_record`), all
      `core_rulebook`, exactly matching that cycle's own stated "32 closed" — named here, not
      silently absorbed (`decisions.md §12` L3).
    - **3 `trait`, `ingested-magnitude → grounded`, NOT this cycle's work** — all
      `ultimate_campaign`, evidence `trait_content_table_holds_record_magnitude_not_yet_computed
      → trait_content_magnitude_computed_and_verified_by_fixture_execution_flat_N`. A side
      effect of running the standard `DERIVED_FIXTURE_CHECK_REPORT` pass (same 3-pass pipeline
      cycles 1–3 used) picking up 3 fixture matches not folded into the last committed regen —
      unrelated to `equipment_effects.rs` (a totally different `Kind::Trait` code path,
      untouched by this cycle's diff) and unrelated to any code this cycle wrote. Named as
      instrument-correction/reachability movement, not claimed.
    - **Reconciliation: 256 + 46 + 32 + 3 = 337**, matching the whole-corpus diff's own total
      changed count exactly. Bucket-level cross-check: `core_rulebook` DONE
      **4,449 → 4,613 (+164)** = 132 (this cycle's own M→DONE) + 32 (the co-mingled class_feature
      closure, not mine) — the 27 `core_rulebook` text-complete→grounded rows do NOT add to
      DONE (already counted there before this cycle). `core_rulebook` bucket `C`:
      **233 → 201 (−32)**, exactly matching `AT-34-E3-002` cycle 6's own already-claimed figure,
      confirming that co-mingled delta is fully accounted for and not double-claimed here.
  - `corpus_literal_sweep`, this cycle's own baseline run (release, this cycle's own commit's
    corpus state, no `data/corpus/**` file touched): **48,708 examined of 51,482 read**, CLEAN,
    0 findings — unchanged from cycles 2/3's figure (`decisions.md §12` L8 — 0 delta expected, 0
    delta confirmed: only `src/rules_core/equipment_effects.rs` and
    `src/bin/v06_work_inventory.rs` (test-only) changed).
  - `derived_evaluator_fixture_check` (release, same corpus state): **1,839 units cleared over
    2,580 fixture rows, 0 failed, 0 not ingested** — unchanged from cycles 2/3's figure.

- **Row-count command output (this cycle's own local regen whole-corpus id-diff, the artifact
  this cycle's status is set from):**
  ```
  $ python3 <ad hoc id-diff script>   # pre = committed docs/work-inventory.json at rebase base,
                                       # post = this cycle's own local regen output
  pre count: 49438 post count: 49438
  added: 0 removed: 0
  total changed: 337

  real M->DONE closures (ingested-magnitude -> grounded): 259
  by book+kind (equipment/equipment_modifier only, this cycle's own): 256
    core_rulebook: 132 (85 equipment_table_entry_with_corpus_magnitude
                        + 47 equipment_own_line_has_no_magnitude_but_closure_wiring_class_does)
    mythic_adventures: 78, ultimate_equipment: 16, ultimate_psionics: 13,
    advanced_players_guide: 11, advanced_race_guide: 2, book_of_the_damned_volume_2: 2,
    advanced_class_guide: 1, inner_sea_gods: 1
  by book+kind (trait, NOT this cycle's own): 3 (ultimate_campaign, fixture-check side effect)

  text-complete -> grounded (DONE->DONE reclass, this cycle's own side effect): 46
    core_rulebook 27, mythic_adventures 13, ultimate_psionics 4,
    advanced_class_guide 1, advanced_race_guide 1

  class_feature engine-does-not-hold -> grounded (NOT this cycle's own): 32, all core_rulebook
  ```
  Sum 256 + 3 + 46 + 32 = **337**, matching the whole-corpus diff's own total exactly. This
  cycle's real closure is **exactly 256 units corpus-wide (132 `core_rulebook`)**, none of it
  reclassification of an already-DONE unit into a *different bucket* (the 46 text-complete→
  grounded rows stay inside DONE throughout).

- **Build scope verified (at the final commit SHA `3822c0c1d8`):**
  - `cargo test --locked --lib rules_core::equipment_effects::` — **83/83 pass** (0 new #[test]
    fns added directly to this module this cycle — the 3 new proofs live in
    `v06_work_inventory.rs`'s own `e14_harness_tests` module; the +1 vs. cycle 3's stated 82/82
    baseline is an already-landed sibling cycle's own addition, confirmed unrelated to this
    diff). RED confirmed first for the intended reason: temporarily reverted
    `let mut var_bonus = general::compute_var_effect(record);` to
    `let mut var_bonus: Vec<VarBonus> = Vec::new();`, reran
    `equipment_probe_promotes_a_real_magic_item_via_its_bonus_var_token` alone — FAILED with the
    exact stated assertion message (`"Amulet of Mighty Fists carries BONUS:VAR|
    MightyFistValue|5 on its own line…"`), confirming the test fails for the intended reason
    (the wiring being absent), not an unrelated defect — then reverted the revert and reran:
    GREEN.
  - `cargo test --locked --bin v06_work_inventory` — **482/482 pass** (3 new:
    `equipment_probe_promotes_a_real_magic_item_via_its_bonus_var_token`,
    `equipment_probe_promotes_a_hand_built_item_via_an_eqmod_referenced_var_chain`,
    `equipment_probe_does_not_promote_a_record_with_no_var_chain_and_no_other_effect`; +9 vs.
    cycle 3's stated 473/473 baseline — the other 6 are already-landed sibling cycles' own
    additions, confirmed unrelated by diff scope).
  - `cargo test --locked --lib` (full lib suite) — **2,977 passed / 5 failed / 14 ignored**
    (pre-existing, confirmed unrelated: `class_feature_pool_catalog`/
    `formula_interpreter_corpus_wide` (3 tests)/`companion_chassis` all fail on the SAME
    unmapped `(wiring_class="derived", status="oracle-agree")` pair in
    `scripts/observer/pf1e_dashboard_producer.py` cycle 1 already named as an out-of-scope
    `incident` in this package's own `progress.md` — identical failure set to cycle 3's own
    stated baseline, none in `rules_core::equipment_effects::` or touching this cycle's diff).
  - `cargo test --locked --no-run` (full workspace, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-003`)
    — **exit 0**.
  - `apps/desktop/src-tauri` (separate cargo workspace,
    `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-003-desktop`): `cargo test --locked --no-run
    --manifest-path apps/desktop/src-tauri/Cargo.toml` — tested explicitly because
    `compute_equipment_effects` (whose `ResolvedEquipmentEffect` this cycle widens with a new
    field) is a real, already-wired production path the desktop crate's `character_hub.rs`
    consumes (`codex = { path = "../../.." }`) — the change is additive-only (a new struct
    field, no signature change to any existing field or function). **Exit 0.**
  - `python3 scripts/completion_atlas.py --check` (local regen) — confirmed run, exit 0
    (`unclassified=0 overlap=0`), `citation_failures=0`, `missing_clearing_mechanisms=0`,
    `stale_derived_at=False`.

- **Sweep population:** no `data/corpus/**` records added or regenerated — only
  `src/rules_core/equipment_effects.rs` and `src/bin/v06_work_inventory.rs` (test-only addition)
  changed. `corpus_literal_sweep`: **48,708 examined of 51,482 read**, CLEAN, 0 findings,
  unchanged before/after (`decisions.md §12` L8 — 0 delta expected, 0 delta confirmed).

- **Denominator gate against this package:**
  `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` →
  `files_checked=15 violations=8`, all 8 pre-existing verbatim-quoted corpus prose in
  `progress.md` (`FRT_HVY`'s "75% chance…" prose), already flagged by prior cycles — confirmed
  before this cycle's own new prose (this receipt, the `progress.md` entry, the `kanban.md` row
  addendum) was added, and re-confirmed after: this cycle's own new prose contains no bare
  number formatted as a percentage.

- **Oracle pin:** `PCGEN_ORACLE_SHA` per `scripts/pcgen-oracle-pin.env`
  (`7f818006e371188e5717fd18d74d18a420747fc6`) — not directly consulted this cycle (no
  oracle-pinned corpus read live), but `general::compute_var_effect`'s own correctness was
  already established against this exact pin by SD-33's `e5_var_shape_ours.rs` round-trip
  comparison, prior to this cycle — this cycle's contribution is the wiring, not new,
  unvalidated logic.

- **Status:** partial

- **Movement, four buckets:** closure (**256** units corpus-wide, `ingested-magnitude →
  grounded`, `132` of them `core_rulebook` — a genuinely newly-promoted, already-wired-but-
  never-called compute path, isolated by kind and evidence transition in the id-diff above, not
  co-mingled with any other cycle's work); reclassification (**46** units, `text-complete →
  grounded`, DONE→DONE, a side effect of the same fix — the record was already correctly done,
  now carries stronger evidence; **32** more, `engine-does-not-hold → grounded`, NOT this
  cycle's own work, named and excluded above); reachability (0 — no new record became
  addressable that was previously absent from any engine table); instrument-correction (**3**
  units, `ingested-magnitude → grounded` via the standard `DERIVED_FIXTURE_CHECK_REPORT` pass,
  NOT this cycle's own work, named and excluded above).

- **Notes:**
  - **The two equipment shapes are NOT categorically different for every mechanism — cycles 2/3
    were right for `BASEITEM:`/`TEMPBONUS:`, and this cycle shows the `VAR` token family is the
    exception.** Every prior receipt in this lineage stated "one fix does not cover both
    shapes" as a confirmed, repeated finding. That finding was correct for the token families
    those cycles tested. This cycle's own 47 closure-shape closures are the counter-example: a
    `BONUS:VAR|<name>|<value>` chain the OLDER, narrower magnitude-detection heuristic behind
    `text_only`/`unit.magnitude_token_count` did not recognise as carrying a literal magnitude
    (that heuristic appears to have been built before `VAR` chains were considered a "magnitude
    token" at all), while `wiring_class`'s own fuller signal aggregation (built separately,
    reading the FULL token closure) already correctly flagged these records `computed`. Both
    disagreements resolve the same way: the OLDER classifier check was the one that was wrong,
    not `wiring_class`.
  - **Real correctness boundary found and respected, not silently worked around.** 18 units
    (9 same-line + 9 closure-shape, all `core_rulebook`, all the `Intelligent Item ~ Alignment /
    <alignment>` chassis family) carry a `BONUS:VAR|NegLevels|1+var("IntItemNegativeLevel")|
    !PREALIGN:<code>` chain — a formula-valued, `PRE`-gated magnitude
    (`compute_var_effect`'s `.parse::<i16>().ok()?` correctly declines the non-literal string
    `"1+var(\"IntItemNegativeLevel\")"` rather than fabricating a value) that is also genuinely
    character-alignment-conditional, not a flat item property. Confirmed by direct corpus read
    of all 9 same-line instances (`intelligent_item_alignment_chaotic_evil` through
    `_true_neutral`); the 9 closure-shape instances are presumed the same family
    (equipment_modifier records referencing these same chassis rows via `EQMOD:`) but were not
    individually re-read this cycle — named as a next-cycle verification item, not asserted as
    fact.
  - **Generic by construction, corpus-wide, confirmed a fourth time.**
    `compute_equipment_effects` already calls every per-category resolver unconditionally on
    every record regardless of book (confirmed by reading the dispatcher, same finding cycles
    1–3 each independently confirmed for their own token family) — 9 books moved from this
    single change with no per-book dispatch code anywhere in the diff.

- **Remainder — every unit in this cycle's two EQUIPMENT sub-causes, named by real mechanism, at
  HEAD (post this cycle's own local regen, `core_rulebook`-scoped):**

  | Sub-cause (evidence string) | Population before this cycle | Closed this cycle | Population after |
  |---|---:|---:|---:|
  | `equipment_table_entry_with_corpus_magnitude` | 249 | 85 | **164** |
  | `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does` | 146 | 47 | **99** |

  Sum: 164 + 99 = **263**, `core_rulebook`-scoped; this cycle's own closure (132) plus this
  remainder (263) = 395, matching this cycle's own start-of-cycle territory exactly.
  Corpus-wide, this cycle closed 256 (132 `core_rulebook` + 124 other-book); the remainder in
  every other book of these two sub-causes is not re-derived here (Epic 5's forward-plan
  territory, not this criterion's, matching cycles 2/3's own precedent).

  **Fresh qualifier-shape census of the `core_rulebook` remainder (263), re-derived this cycle
  (not inherited from cycle 2's now-partially-stale 9-shape table — the `VAR` row below is no
  longer "cross-subsystem, new engineering" as that table stated; it is now the 18-unit
  formula/`PRE`-gated remainder named above):**

  | Sub-shape (`raw_bonus_chains` qualifier-type set, own corpus line) | same-line pop. | closure-shape pop. | Disposition |
  |---|---:|---:|---|
  | *(no chain at all)* | 119 | 70 | Prose-only or internal chassis plumbing — same disposition cycle 2/3 gave this shape; the closure-shape 70 is new to this census (not previously isolated by sub-cause) and is next cycle's own investigation target. |
  | `VAR` (formula-valued/`PRE`-gated, per Notes above) | 9 | 9 | Correctly declined this cycle; would need `formula_interpreter` + character-alignment context — a materially different, per-character-conditional mechanism, not a flat item property. |
  | `COMBAT` | 8 | 8 | Already-wired token family (`arms_armor.rs`) but NOT closing — worth a follow-up read to learn why `equipment_key_is_wired`'s existing `COMBAT` consultation misses these. |
  | `ITEMCOST` | 7 | 2 | No compute-path gap; nothing to compute (same disposition cycles 2/3 gave this shape). |
  | `SAVE` | 6 | 6 | Not yet consulted by any resolver — a real, un-investigated next-cycle candidate. |
  | `EQM` | 3 | 0 | New field + new subsystem, too small to justify alone (same disposition cycle 3 gave this shape). |
  | `ITEMCOST, WEAPON` | 3 | 0 | Same as `ITEMCOST` above. |
  | `EQMWEAPON` | 3 | 0 | Equipment-modifier composition onto a weapon (same disposition cycle 3 gave this shape). |
  | `SKILLRANK` | 2 | 2 | Not yet consulted by any resolver — un-investigated. |
  | `STAT` | 1 | 1 | Already-wired token family (`magic_items.rs`) but NOT closing — same follow-up as `COMBAT` above. |
  | `SKILL` | 1 | 1 | Already-wired token family (`general.rs`) but NOT closing — same follow-up. |
  | `SPELLCAST` | 1 | 0 | Not yet consulted by any resolver. |
  | `WEAPON` | 1 | 0 | Same shape as `EQMWEAPON` above. |

  Sum same-line: 119+9+8+7+6+3+3+3+2+1+1+1+1 = **164**. Sum closure-shape:
  70+9+8+2+6+0+0+0+2+1+1+0+0 = **99**. Both match the table above exactly.

  **Every other `M` sub-cause, and buckets V/D/U/X, are untouched by this cycle** (out of
  territory by the dispatch brief's own no-collision rule) — unchanged from cycle 3's own
  statement: `ability_content` 217, `race_trait_generic` 119, `template_content` 96,
  `in_catalog_with_corpus_magnitude_but_no_observed_consumer` 47, `domain_content` 34,
  `skill_content` 19, `spell_list_entry_with_resolved_level` 15,
  `race_trait_states_a_universal_sheet_modifier_pending_compute` 2, `D` 366, `U` 10, `X` 115.
  `V` and `C` DID move this regen but from OTHER already-committed cycles' own work
  (`AT-34-E3-002` cycle 6's `C` 233→201, named and reconciled above) — not this cycle's own
  claim.

- **Next-cycle plan:**
  1. **`COMBAT`/`STAT`/`SKILL` (8+8+1+1+1+1 = 20 units) already have a wired resolver for their
     own token family, yet stay unclosed** — the single highest-value next investigation:
     read a handful of real corpus records in each to learn what specifically the existing
     `arms_armor`/`magic_items`/`general` resolvers decline about them (a different qualifier
     shape within the same family, a category mismatch, a `PRE`-gate) before assuming a new
     mechanism is needed.
  2. **`SAVE`/`SKILLRANK` (6+2+6+2 = 16 units) have NO wired resolver at all** — genuinely new
     engineering, real candidates once costed.
  3. **The closure-shape `(no chain)` 70 units are newly isolated this cycle** (cycle 2's own
     9-shape census never separated same-line from closure-shape populations) — worth its own
     qualifier-shape re-census before assuming it matches the same-line 119's disposition.
  4. **The 18-unit formula/`PRE`-gated `VAR` family** (`Intelligent Item ~ Alignment / *`) is
     correctly out of this cycle's reach; would need `formula_interpreter` integration plus
     real character-alignment context, likely a different criterion's scope entirely (a
     character-conditional effect, not a flat equipment property).
  5. **The shared regen cycle** must pick up this cycle's source change (the `VAR` wiring) the
     next time it commits `docs/work-inventory.json`'s three-pass pipeline — this cycle already
     ran that regen locally and confirmed the exact effect: `core_rulebook` M 944→812 (−132),
     corpus-wide 256 real closures across 9 books, co-mingled in the local regen's raw output
     with 32+3 more changes from `AT-34-E3-002` cycle 6 and the standard fixture-check pass,
     both already committed/expected and isolated by kind above, not this cycle's own claim.
  6. **`ability_content` (217, a sibling lane's territory) remains the largest overall `M`
     sub-cause.**
