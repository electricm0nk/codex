# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-003 (bucket `M` — EQUIPMENT sub-causes, cycle 7)

- **Commit SHA:** `7e59387d9f` (this file's own commit — no production code changed this
  cycle, so it is the cycle's only commit; SHA filled in this follow-up commit, matching cycle
  3's own established pattern for this receipt-cites-its-own-SHA case).

- **Continuation of, not a duplicate of,** `AT-34-E3-003_m_bucket_equipment_cycle_receipt.md`
  through `_6.md`, all already merged into `tranche/14` before this cycle started, plus the
  wave-21 shared regeneration (`a0cbc2388a`) that applied cycle 6's fix to the committed
  `docs/work-inventory.json`. Worktree opened at `ea2b3396f2` (the tranche cut); `git fetch
  origin && git rebase origin/tranche/14` moved HEAD to the real tip, `e5d4598f2a`
  (`AT-34-E3-002` cycle 9's own progress/kanban row) — the wave-21 regen, two `AT-34-E4-002`
  UC-lane cycles, and one more `AT-34-E3-002` cycle had all landed since this wave's dispatch
  brief was authored. The brief's own figures (`core_rulebook` M = 972, split 276+147=423)
  were stale by construction; every figure below was independently re-derived at the real
  rebase base before any investigation began (`decisions.md §12` L2).

- **Retro `correction` logged** against the dispatch brief's own stale split (423 vs. the real
  229 at HEAD) — `docs/retro/events/sd34-at-34-e3-003.jsonl`, `--verified-by` naming the
  `completion_atlas.py`/`docs/work-inventory.json` re-derivation below.

- **Re-derived at cycle start (never trusting the inherited figure):**
  `python3 scripts/completion_atlas.py --book core_rulebook --check` at HEAD (`e5d4598f2a`) →
  `core_rulebook` M = **778** (matches wave-21 regen's own closing figure exactly — cycle 6's
  221-corpus-wide/33-`core_rulebook` fix is now live in the committed board). Split, read
  directly off `docs/work-inventory.json` (`u['book']=='core_rulebook' and
  u['status']=='ingested-magnitude' and u['evidence'] in
  {'equipment_table_entry_with_corpus_magnitude',
  'equipment_own_line_has_no_magnitude_but_closure_wiring_class_does'}`):
  `equipment_table_entry_with_corpus_magnitude` **130**,
  `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does` **99** — sum **229**,
  matching cycle 6's own predicted post-regen remainder (262−33=229) exactly, independently
  reproduced. Corpus-wide, same two evidence strings, all books: **347 + 195 = 542**, across
  21 books (`core_rulebook` 229 of it — matching the book-scoped count exactly).

- **This cycle's job, per the wave's own dispatch note ("this bundle has had a cycle disprove
  another's stated reason"): re-derive cycle 6's own next-cycle-plan claim that the 229-unit
  remainder is "dominated by `%CHOICE`-gated, `VAR`/`PRE`-gated, `COST:`-only-no-`WT:`, and
  small new-subsystem shapes" — fresh, not inherited.** Read the real, on-disk corpus record
  for every one of the 229 `core_rulebook` units (cross-referenced against
  `docs/work-inventory.json`'s own `wiring_class_signals`, `corpus_key`, and `book` fields —
  never a name-only match) and the compute-path source for every candidate consumer
  (`equipment_effects.rs`, `arms_armor.rs`, `general.rs`, `magic_items.rs`, `equipmods.rs`,
  `intelligent_item.rs`, `encumbrance.rs`, `equipment_resolver.rs`) before writing any code.

  **Finding: cycle 6's own remainder characterization holds, and is now precise rather than
  approximate.** No generically-closable-by-existing-instrument gap remains in the EQUIPMENT
  sub-causes after cycle 6's fix. Every one of the 229 `core_rulebook` units decomposes into
  one of seven real, independently-verified mechanisms, **none of which has an already-wired
  consumer the probe was merely blind to** (the shape cycles 3/4/6 each found and fixed) —
  every remaining mechanism needs either new selection-level state, a new formula evaluator,
  or corpus data that was never captured:

  | # | Sub-cause | `core_rulebook` | corpus-wide | Real gap |
  |---|---|---:|---:|---|
  | 1 | `choice_gated_needs_selection_plumbing` | **104** | 140 | Magnitude is `%CHOICE`/`CHOOSE:`-driven (a player-picked numeric value, e.g. `BONUS:COMBAT\|AC\|%CHOICE\|TYPE=DEFLECTION`). `character_input::EquipmentSelection` carries no chosen-value field at all — confirmed by direct read of its struct definition. Same boundary cycles 2–6 already declined; re-confirmed, not re-discovered. |
  | 2 | `cost_only_no_weight_deliberately_excluded` | **44** | 164 | Own line's only magnitude is `COST:` with no `WT:`. `compute_encumbrance`'s own rule (weight required, cost supplementary) correctly excludes these — cycle 6's own probe widening is gated on weight specifically for exactly this reason, confirmed by re-reading `encumbrance::equipment_key_resolves_a_carried_weight`'s own gate. Not a probe gap; a correct exclusion matching the real consumer's own rule. |
  | 3 | `no_magnitude_no_wt_no_cost_untraced_closure` | **28** | 36 | Own line carries no magnitude, no `WT:`, no `COST:` at all (e.g. `MightyFist_AMF`, `VISIBLE:NO` only) — the unit's closure-computed `wiring_class_signals` (`derived:cost` etc.) must come from a **different** corpus record's `.MOD`/reference chain this cycle's own-record-only script did not trace. Reported honestly, not force-classified: the closure-tracing this needs is scoped work for a follow-up cycle, not solved here. |
  | 4 | `var_formula_reference_needs_evaluator` | **18** | 18 | Own-line `BONUS:VAR\|<name>\|<formula>` chain where `<formula>` is not a literal integer (e.g. `Intelligent Item Alignment (CE)`'s `1+var("IntItemNegativeLevel")`). `intelligent_item::compute_intelligent_item_effect` already exists and is already a real, wired consumer, but by its own documented, deliberate discipline reads only literal `qualifiers[2]` integers — it would return `None` even if attached, confirmed by direct inspection of its known-VAR-name match set (none of the 18 records' own VAR names are in it) and its literal-parse-only rule. Needs a real formula evaluator resolving nested `var()` references against other bonus chains — new-subsystem work, not a probe gap. |
  | 5 | `thin_no_raw_tokens` | **17** | 135 | The corpus JSON record carries no `raw_tokens` array at all (only `key`/`name`/`cost_gp`/`weight_lbs`/`description` survived ingestion) — confirmed for all 17 by direct read of every file. Even where the record's own `wiring_class_signals` field (computed at a different point, from the original `.lst` line, before reduction to this thin JSON shape) shows a real magnitude existed, that magnitude's actual token text is gone from the corpus and cannot be reconstructed without a guarded corpus regen from source — out of this cycle's write scope (`workflow-instruction.md §6`: "Never hand-edit `data/corpus/**`"). |
  | 6 | `itemcost_pricing_formula_no_consumer` | **9** | 12 | Own-line chain is `BONUS:ITEMCOST\|...` only (a material/quality cost-multiplier formula, e.g. `Material ~ Cold Iron`'s `ITEMCOST\|TYPE.WEAPON\|(BASECOST)\|!PRETYPE:1,Double`) — `equipmods.rs`'s own module doc comment explicitly names `BONUS:ITEMCOST` as "an internal cost/formula token", deliberately never read by any of the four category resolvers. `equipment_resolver.rs` confirms no consumer anywhere evaluates `ITEMCOST` formulas into a final price (grep for `ITEMCOST` across `src/` finds only this same doc-comment naming and the enumeration table, not an evaluator). A real gap, but a new pricing-formula subsystem, not a probe widening. |
  | 7 | `real_type_but_excluded_shape_or_new_mechanic` | **9** | 25 | Own-line chain names a real player-facing type (`WEAPON`, `SAVE`, `EQM\|WEIGHTADD`) but is either a **deliberate, already-documented exclusion** (`equipmods.rs`'s own doc comment: `TYPE=CONDITION` broken-weapon penalties and `TYPE=Enhancement.STACK` armor-vs-material penalties are correctly excluded from the enhancement-bonus field, confirmed for `Special Quality ~ Broken ~ Weapon` and `Material ~ Alchemical Silver` by direct source read) or names a mechanic with **no existing field at all** (`Special Ability ~ Speed ~ Weapon`'s `WEAPON\|ATTACKS\|1` extra-attack grant; `Holy Symbol (Silver)`'s `EQM\|WEIGHTADD\|1` applied-modifier weight delta — confirmed by a corpus-wide grep of `src/` finding zero `WEIGHTADD` consultation anywhere). Each is real, small, and would need its own new field/consumer — not a shared mechanism, so not a single cycle's generic win. |
  | | **Sum** | **229** | **530** + 10 NOTFOUND + 2 uncategorized = **542** | `104+44+28+18+17+9+9 = 229` (`core_rulebook`, exact). `140+164+36+18+135+12+25 = 530` (corpus-wide named rows) `+10` `NOTFOUND` (a book/key lookup gap in this cycle's own Python index, not a corpus defect — every `core_rulebook` unit resolved cleanly) `+2` `uncategorized` (`VISION`/`MOVEADD` own-line types, not yet classified) `= 542`, matching the re-derived corpus-wide total with no gap. |

  **The `core_rulebook` column (this cycle's actual territory) is exact: 229, no unresolved
  remainder. Corpus-wide figures are directionally confirmed with a named 12-record gap (10
  `NOTFOUND` + 2 `uncategorized`) outside `core_rulebook`, reported honestly rather than
  silently folded into a bucket.**

- **Why no code was written this cycle.** Every one of the seven mechanisms above genuinely
  needs either (a) new `EquipmentSelection`-level chosen-value state threaded through the
  wire contract and desktop UI (row 1, the largest, matching prior cycles' correct
  disposition), (b) a PCGen formula evaluator that resolves nested `var()` cross-references
  against other bonus chains (row 4), (c) a guarded corpus regeneration this cycle has no
  write scope for (row 5), (d) a pricing-formula evaluator with no existing consumer at all
  (row 6), or (e) a genuinely new field/consumer per shape with no shared mechanism (row 7, 9
  `core_rulebook` units split across at least 3 distinct sub-shapes, too small individually to
  be "the largest sub-cause"). Row 2 (44 units) is not a gap at all — it is a correct exclusion
  matching the real consumer's own rule, re-confirmed rather than fixed. Row 3 (28 units) is a
  real, scoped follow-up (multi-file closure tracing) but was discovered too late in this
  cycle's own turn budget to implement and TDD-prove safely; named honestly as the best
  next-cycle candidate rather than rushed.

  **This directly engages the wave's own instruction to disprove a stated reason where
  warranted, and finds the opposite result from cycle 6: cycle 6 disproved cycle 5's "generically
  exhausted" claim by finding a real, already-wired consumer (`encumbrance`) the probe had
  missed. This cycle looked for the same shape again — another already-wired consumer the
  probe is blind to — across all seven remaining mechanisms, read every relevant compute-path
  source file, and found none. The brief's own framing ("the instrument already exists" for
  bucket `M`, unlike bucket `B`) does not hold for what remains of the EQUIPMENT sub-causes
  after six cycles of closure — confirmed, not assumed.** Per `decisions.md §12` L6
  (workflow-instruction row 6): "measurement waves that bank zero units are legitimate
  deliverables." Per `decisions.md §16`: "Only the count grounds" — no fix was forced to make
  a number move.

- **Files touched:** none under `src/`, `scripts/`, or `data/corpus/` — no production code or
  corpus data changed this cycle. This receipt, `progress.md`, `kanban.md`, and
  `docs/retro/events/sd34-at-34-e3-003.jsonl` only.

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` on this cycle's own isolated diff (empty —
  no files under the epic's file-touch set changed). The whole-tranche `§6` literal command
  (scoped to `src/rules_core/`, `src/bin/`, `scripts/oracle_harness/`,
  `data/corpus/core_rulebook/`, `docs/work-inventory.json`, since `merge-base HEAD
  origin/develop`) reports pre-existing hits from work already committed before this cycle
  started (`sd32_class_ingest`/`sd32_simple_filename_kind_ingest` evidence strings baked into
  the committed `docs/work-inventory.json`, and `SD-`/`sd-` mentions in prior cycles' own doc
  comments) — none introduced by this cycle, matching cycle 6's own precedent for reading this
  command's output.

- **Wired-integration audit result:** `OK_NO_TOKENS` on this cycle's own isolated diff (empty).
  The whole-tranche command's `placeholder` hits are all pre-existing, real corpus data
  (PCGen's own "no selection" `CHOOSE:`-menu placeholder rows, correctly named as such in
  already-committed evidence strings and doc comments) — not a stub token in shipping code.

- **Acceptance criterion (verbatim, `epic-breakdown.md` AT-34-E3-003):** "buckets M, V, D, U,
  X close ... Evidence: per bucket, the atlas reporting zero for `core_rulebook`, with
  movement in four buckets. A count that drops because measurement changed is
  instrument-correction, not closure." This cycle's real contribution is a corrected,
  exhaustive sub-cause census (reachability work, not closure) — reported honestly below, not
  dressed as a closure it is not.

- **RED → GREEN:** N/A this cycle — no production code changed, so no test was written or
  needed to move. This is a measurement-only cycle (`decisions.md §12` L6).

- **Figures + their re-derive commands:**
  - `core_rulebook` M at cycle start: `python3 scripts/completion_atlas.py --book
    core_rulebook --check` → **778**, denominator `core_rulebook` population 6,701.
  - Territory split: Python script against `docs/work-inventory.json` filtering
    `book=='core_rulebook' and status=='ingested-magnitude' and evidence in
    {equipment_table_entry_with_corpus_magnitude, equipment_own_line_has_no_magnitude_but_
    closure_wiring_class_does}` → **130** / **99**, sum **229**, denominator `core_rulebook`
    M 778.
  - Corpus-wide territory, same script, all books: **347** / **195**, sum **542**, denominator
    corpus-wide M (see `completion_atlas.py --check` corpus-wide row below).
  - Seven-mechanism census: script cross-referencing each of the 229 `core_rulebook` units'
    `docs/work-inventory.json` fields (`corpus_key`, `book`, `wiring_class_signals`) against
    its on-disk `data/corpus/core_rulebook/**/*.json` record's `raw_tokens`/`raw_bonus_chains`
    — table above; sum 104+44+28+18+17+9+9 = **229**, exact, no remainder.
  - `WEIGHTADD` consumer check: `grep -rln "WEIGHTADD" src/` → **0 matches**, confirming no
    existing consumer reads applied-modifier weight-delta chains.
  - Full-workspace build scope: `cargo test --locked --no-run` at `e5d4598f2a` → **exit 0**.
  - Desktop crate build scope: `cd apps/desktop/src-tauri && cargo test --locked --no-run` at
    `e5d4598f2a` → **exit 0** ("Finished `test` profile [unoptimized + debuginfo] target(s) in
    2m 43s", separate `CARGO_TARGET_DIR`, no shared-slot contention).

- **Row-count command output (this cycle's own artifact — no `docs/work-inventory.json`
  change this cycle, so the count is the census table's own sum, not an inventory id-diff):**
  ```
  CENSUS_SUM core_rulebook: 104+44+28+18+17+9+9 = 229 (matches re-derived M-split total 130+99=229 exactly)
  CENSUS_SUM corpus_wide: 140+164+36+18+135+12+25 = 530, +10 NOTFOUND +2 uncategorized = 542 (matches re-derived corpus-wide M-split total 347+195=542 exactly)
  ```

- **Build scope verified:** `cargo test --locked --no-run` — **exit 0**, full workspace, run
  at `e5d4598f2a` (this cycle's own HEAD — no commit in this cycle moves any figure this
  receipt's assertions depend on, since no code or corpus file changed). `apps/desktop/src-tauri`
  tested explicitly — **exit 0**, run at `e5d4598f2a`, separate `CARGO_TARGET_DIR`.

- **Sweep population:** not re-run this cycle — `git status --porcelain -- data/corpus/`
  confirms empty (no corpus records touched), so `corpus_literal_sweep`'s examined-population
  is unchanged from wave-21's own baseline (**48,708 examined of 51,482 read, 0 findings,
  CLEAN**, `AT-34-E3-001_wave9_regen_receipt.md` ninth section) — no corpus change this cycle
  to move it (`decisions.md §12` L8 applies only when records are added or regenerated).

- **Oracle pin:** `PCGEN_ORACLE_SHA` per `scripts/pcgen-oracle-pin.env` — not consulted this
  cycle; every figure above comes from direct corpus/source reads, not an oracle round-trip.

- **Status:** partial.

- **Movement, four buckets:** **Closure: 0** (no code or corpus change, no bucket boundary
  crossed). **Reclassification: 0.** **Reachability: 0** — the census does not move any unit's
  bucket; it names the existing 229/542 population's real sub-mechanisms with exact
  populations, for the first time at this precision. **Instrument-correction: 0** — no
  fixture/sweep disagreement surfaced, and the retro `correction` above corrects the *dispatch
  brief's own stale figure* (423 vs. 229), not a measured count this cycle produced.

- **`docs/work-inventory.json` / `completion-atlas.json` NOT touched this cycle** (no local
  regen run; nothing to `git restore` — both files are byte-identical to HEAD, `md5sum`
  confirmed against the working tree, since this cycle wrote no code and ran no generator).

- **Notes (judgment calls):**
  - This cycle deliberately did **not** force a code change to produce a nonzero closure
    figure. `decisions.md §16` ("Only the count grounds") and `§12` L6 ("measurement waves
    that bank zero units are legitimate deliverables") both apply directly: after
    investigating every remaining mechanism against the real compute-path source, none is
    safely closable within one cycle without either building new selection-level state, a
    formula evaluator, or a corpus regeneration outside this cycle's write scope. Rushing any
    of those under time pressure would risk exactly the "narrow a gate to pass" / stub-shaped
    failure the doctrine forbids.
  - Row 3 (`no_magnitude_no_wt_no_cost_untraced_closure`, 28 `core_rulebook` units) is this
    cycle's own best-named next-cycle candidate: these units' closure-computed
    `wiring_class_signals` prove a real magnitude exists SOMEWHERE in their token closure (a
    `.MOD`-row or cross-reference this cycle's own-record-only script did not trace), which
    means at least some of them may resolve to one of rows 1/2/4/5/6 once traced — or may
    reveal a seventh, real mechanism. Reported honestly as untraced rather than
    force-classified into an existing bucket.
  - Corpus-wide figures (rows 1/2/5/6 in the table, and the two-way script cross-check) carry
    a **10-record `NOTFOUND` + 2-record `uncategorized`** gap outside `core_rulebook` — this
    cycle's Python book/key index missed 10 records (likely a book-directory-name mismatch for
    a book whose corpus directory name differs from its `docs/work-inventory.json` `book`
    field, the same class of hazard `workflow-instruction.md §4`'s "shallow glob lies here"
    warns about) and left 2 own-line chain types (`VISION`, `MOVEADD`) uncategorized. Named
    honestly rather than silently absorbed into a bucket; **does not affect the
    `core_rulebook` column, which is this cycle's actual territory and is exact.**
  - The dispatch brief named a sibling-lane boundary ("ability_content, race_trait_generic and
    template_content are OFF LIMITS this cycle") which this cycle respected — no file under
    those kinds' compute paths was read or touched.

- **Next-cycle plan:**
  1. **Trace row 3's 28 `core_rulebook` units' closures** (the `.MOD`/cross-reference chain
     `v06_work_inventory`'s own closure computation follows but this cycle's own-record-only
     script did not) to determine whether they resolve into one of the six named mechanisms
     above or constitute a real seventh one. Best-ROI next step: smallest population, real
     compute-path knowledge already built this cycle.
  2. **If EQUIPMENT is judged closed-out for this wave** (no further generically-closable gap
     without new subsystems), the wave should pivot the equipment lane's remaining budget to
     `ability_content` (217, the largest overall `core_rulebook` M sub-cause, sibling lane's
     territory) or accept the seven named mechanisms above as the real forward-plan input for
     Epic 5's costing (`AT-34-E5-001`), rather than spending further cycles re-deriving the
     same remainder.
  3. **`ability_content` (217) and `race_trait_generic` (119) and `template_content` (96)**
     remain the sibling lanes' territory, unchanged and untouched by this cycle.
