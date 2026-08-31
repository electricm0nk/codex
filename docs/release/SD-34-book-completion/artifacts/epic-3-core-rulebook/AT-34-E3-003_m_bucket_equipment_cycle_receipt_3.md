# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-003 (bucket `M` — EQUIPMENT sub-causes, cycle 3)

- **Commit SHA:** filled after commit, below.

- **Continuation of, not a duplicate of,** `AT-34-E3-003_m_bucket_equipment_cycle_receipt.md`
  (cycle 1, landed `7147fd86ab`) and `AT-34-E3-003_m_bucket_equipment_cycle_receipt_2.md` (cycle
  2, landed `0519220786`/`3ffa80cc20`), both already merged into `tranche/14` before this cycle
  started. Rebase base at cycle start: `origin/tranche/14`'s tip, `9dfd4a5ebe` (fast-forwarded
  cleanly from the stale worktree checkout — `9dfd4a5ebe` is `fix(sd34): AT-34-E3-002 cycle 5 --
  close Bard Versatile Performance naming gap`, itself after `AT-34-E4-002` cycle 6). This
  cycle's own dispatch brief carried the STALE pre-cycle-1 figures (972 M / 276+147 = 423
  equipment territory) — every figure below was independently re-derived at the real rebase base
  before any code was read or written, per `decisions.md §12` L2, and the brief's carried
  populations were confirmed stale (not trusted) against the live `docs/work-inventory.json`.

- **Re-derived at cycle start (never trusting the inherited figure):**
  `python3 scripts/completion_atlas.py --book core_rulebook --check` at the rebase base →
  `core_rulebook` M = **957** (matches cycle 2's own post-fix figure exactly, confirmed live, not
  assumed). Split: `equipment_table_entry_with_corpus_magnitude` **262** (down from 276 — cycles
  1+2's 14+0 closures on this shape, confirmed), `ability_content_table_holds_record_magnitude_not_yet_computed`
  217, `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does` **146** (down from
  147 — cycle 2's 1 closure, confirmed), `race_trait_generic_table_holds_record_magnitude_not_yet_computed`
  119, `template_content_table_holds_record_magnitude_not_yet_computed` 96,
  `in_catalog_with_corpus_magnitude_but_no_observed_consumer` 47,
  `domain_content_table_holds_record_magnitude_not_yet_computed` 34,
  `skill_content_table_holds_record_magnitude_not_yet_computed` 19,
  `spell_list_entry_with_resolved_level` 15,
  `race_trait_states_a_universal_sheet_modifier_pending_compute` 2. **This cycle's territory
  (the same two EQUIPMENT sub-causes) is 262 + 146 = 408 at cycle start** — cycle 2's own stated
  remainder, confirmed exactly.

- **What this cycle found, that cycles 1 and 2 did not: a whole real MECHANISM cycle 2's
  9-shape classification missed.** Cycle 2's exhaustive classification grouped every unit by its
  `raw_bonus_chains` shape (`VAR`/`ITEMCOST`/`EQM`/`EQMWEAPON`/`WEAPON`/none) and correctly found
  no `BONUS:` chain on the same-line 262. It never inspected `raw_tokens` for `TEMPBONUS:` — a
  DIFFERENT PCGen token family (temporary/consumable-triggered bonuses: potions, elixirs,
  salves), structurally parallel to `BONUS:` but one segment wider (`TEMPBONUS:<target>|<shape>`
  vs `BONUS:<shape>`) and never read by either the same-line resolver (`raw_bonus_chains`, which
  parses only `BONUS:` clauses) or `compute_general_effect`/`compute_magic_items_effect` (which
  read `record.bonus_chains`, populated only from `BONUS:` tokens). Real, live corpus example:
  `Potion of Bull's Strength` (`core_rulebook/cr_equip_magic_items.lst`) carries no `BONUS:STAT`
  chain at all — `raw_bonus_chains` is empty — only `TEMPBONUS:ANYPC|STAT|STR|4|TYPE=Enhancement`,
  the item's entire real mechanical effect. Corpus-wide census (script, not committed — a
  diagnostic read): of the 408-unit territory, **34 corpus records carry a `TEMPBONUS:...STAT|...`
  or `TEMPBONUS:...SKILL|...` token with a literal single stat/skill and a literal signed
  integer**, of which **20** resolve to a genuinely new closure once the fallback is wired (the
  other 14 already carried an explicit `BONUS:` chain that wins, or a compound/wildcard shape
  the fallback correctly declines — see the negative-control tests below).

- **Fix, same shape as cycle 2's `BASEITEM:` widening (a real compute path already exists;
  consult a real token it did not yet read, not a new mechanism):**
  - `src/rules_core/equipment_effects/general.rs` — `compute_general_effect` gains a new
    `tempbonus_skill_fallback` consulted only when no explicit `BONUS:SKILL` chain exists: reads
    a `TEMPBONUS:<PC|ANYPC>|SKILL|<skill>|<n>|...` token the same way the explicit path reads
    `BONUS:SKILL|<skill>|<n>|...`. Refuses (returns `None`) a comma-joined skill list, a
    `TYPE.<Group>` wildcard, and PCGen's real `ALL` wildcard (discovered live on
    `Setting Stone (Invigoration)`, `ultimate_psionics`: `TEMPBONUS:PC|SKILL|ALL|2|TYPE=Morale`
    — reading `ALL` as a skill literally named "ALL" would have been a fabricated value this
    struct has no field to represent correctly) and a `TEMPBONUS:EQ|...` target (an
    equipment-side effect, the real `Lead Blades` shape, never a character-side skill bonus).
  - `src/rules_core/equipment_effects/magic_items.rs` — `compute_magic_items_effect` gains the
    identical-shape `tempbonus_stat_fallback` for `TEMPBONUS:<PC|ANYPC>|STAT|<ability>|<n>|...`.
  - `src/bin/v06_work_inventory.rs` — two new end-to-end tests proving the probe
    (`equipment_key_is_wired`, itself unchanged — it already checks `ability_bonus.is_some()`
    and `skill_bonus.is_some()`) promotes the real, on-disk `Potion of Bull's Strength` and
    `Elixir of Swimming` records against the live `core_rulebook` corpus.
  - This receipt, `docs/release/SD-34-book-completion/progress.md`,
    `docs/release/SD-34-book-completion/kanban.md`.
  - **Deliberately NOT touched/committed:** `docs/work-inventory.json` and
    `artifacts/epic-1-atlas/completion-atlas.json` — reserved to the shared end-of-wave
    regeneration cycle, same rule cycles 1 and 2 followed. Figures below come from a local
    three-pass regen (`corpus_literal_sweep --json-out` → `derived_evaluator_fixture_check
    --json-out` → `CORPUS_LITERAL_SWEEP_REPORT=... DERIVED_FIXTURE_CHECK_REPORT=... cargo run
    --locked --release --bin v06_work_inventory`, no `--allow-stamp-loss`), read, then
    `git restore`-d before this cycle's commit.

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — `git diff --unified=0 9dfd4a5ebe...HEAD --
  src/rules_core/ src/bin/ scripts/oracle_harness/ artifacts/epic-3-core-rulebook/
  ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  → zero matches on this cycle's own diff (the three source files only).

- **Wired-integration audit result:** `OK_NO_TOKENS` — same command, second pattern
  (`\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b`) → zero matches.

- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "AT-34-E3-003 — buckets M, V, D, U, X
  close" (per-bucket, atlas reporting zero for `core_rulebook`, movement in four buckets). This
  cycle is a further slice of the same territory cycles 1 and 2 worked (`equipment_table_entry_with_corpus_magnitude`
  + `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does`) — the criterion as a
  whole stays open (M's other eight sub-causes and buckets V/D/U/X are untouched by design, per
  this wave's no-collision territory boundary: sibling lanes own
  `ability_content`/`race_trait_generic`/`template_content`/choice-wiring/explanation-id work).

- **Figures + their re-derive commands (post-fix, from this cycle's own local three-pass regen):**
  - `core_rulebook` bucket `M`: **957 → 944** (**−13, this cycle's own closure**). Re-derive:
    `python3 scripts/completion_atlas.py --book core_rulebook --check` (against the local regen;
    the committed `docs/work-inventory.json` at HEAD still reads 957 until the wave's shared
    regen cycle lands this cycle's source change).
  - `core_rulebook` `equipment_table_entry_with_corpus_magnitude`: **262 → 249** (−13). Re-derive:
    `json.load`/`Counter` one-liner against the local regen output, filtered
    `book=='core_rulebook'`, `status=='ingested-magnitude'`.
  - `core_rulebook` `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does`:
    **146 → 146** (0 — this cycle's fix reaches only the same-line shape; every `TEMPBONUS`
    census hit in this territory carries its own real corpus magnitude on its own line already,
    confirmed by the transition list below: every one of the 20 closures is
    `equipment_table_entry_with_corpus_magnitude`, none `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does`).
  - Corpus-wide bucket `M`: **4,965 → wave-carrying figure not isolated by this book alone** —
    isolated instead by a full whole-corpus id-diff against the pre-regen committed file (below),
    which is the precise, per-unit accounting this cycle's own closure claim rests on (never a
    subtracted aggregate, which this regen's co-mingled other-lane work would corrupt).
  - **Whole-corpus id-diff by unit id, this cycle's own local regen vs. the committed
    `docs/work-inventory.json` at the rebase base (49,438 → 49,438, 0 added, 0 removed, 90
    changed):**
    - **20 `equipment`, `ingested-magnitude → grounded`, evidence
      `equipment_table_entry_with_corpus_magnitude → equipment_effect_probe_observed_computed_delta`
      — every one of these is THIS CYCLE'S OWN closure**, corpus-wide, across **3 books**: 13
      `core_rulebook` (`dust_of_appearance`, `elixir_of_hiding`, `elixir_of_swimming`,
      `elixir_of_tumbling`, `elixir_of_vision`, `potion_of_bear_s_endurance`,
      `potion_of_bull_s_strength`, `potion_of_cat_s_grace`, `potion_of_eagle_s_splendor`,
      `potion_of_fox_s_cunning`, `potion_of_owl_s_wisdom`, `salve_of_slipperiness`,
      `wings_of_flying`), 5 `advanced_class_guide` (`calumet`, `elixir_of_the_thundering_voice`,
      `gravelly_tonic`, `lozenge_of_the_songbird`, `tracker_s_snuff`), 2 `advanced_race_guide`
      (`corset_of_the_vishkanya`, `elixir_of_forceful_exhalation`). **13 + 5 + 2 = 20**, matching
      the `core_rulebook` M delta (−13) plus the two other books' own M deltas exactly.
    - **70 non-equipment changes, NOT this cycle's work** (co-mingled, unregenerated changes from
      concurrent sibling cycles already committed to `tranche/14` but not yet folded into
      `docs/work-inventory.json` by a shared regen): 63 `core_rulebook` `class_feature`
      (`engine-does-not-hold → grounded` 53, `engine-does-not-hold → literal-verified` 10 —
      `AT-34-E3-002` cycle 5's Bard Versatile Performance closure, landed `9dfd4a5ebe`, this
      cycle's own rebase base), 6 `ultimate_campaign` + 1 `advanced_players_guide` (`AT-34-E4-002`
      cycle 6's trait slice, landed `464960aa2a`). **20 + 70 = 90**, matching the whole-corpus
      diff exactly — named here, not silently absorbed, so this cycle's own closure claim stays
      exact (`decisions.md §12` L3, "never state a derived figure as settled before the work that
      derives it has returned" — the co-mingled figures belong to their own cycles' receipts).
  - `core_rulebook` DONE (post-regen, co-mingled with the Bard fix above): **4,383 → 4,449**
    (+66 = **+13 this cycle's own** equipment closures + **53 the already-committed Bard fix**,
    isolated by kind in the id-diff above, not this cycle's own claim).
  - `corpus_literal_sweep`, this cycle's own baseline run (release, this cycle's own commit's
    corpus state, no `data/corpus/**` file touched): **48,708 examined of 51,482 read**, CLEAN,
    0 findings — unchanged from cycle 2's figure (`decisions.md §12` L8 — 0 delta expected, 0
    delta confirmed: only `src/rules_core/equipment_effects/{general,magic_items}.rs` and
    `src/bin/v06_work_inventory.rs` (test-only) changed).
  - `derived_evaluator_fixture_check` (release, same corpus state): **1,839 units cleared over
    2,580 fixture rows, 0 failed, 0 not ingested** — unchanged from cycle 2's figure.

- **Row-count command output (this cycle's own local regen id-diff, the artifact this cycle's
  status is set from):**
  ```
  $ python3 diff_inventory.py   # pre = committed docs/work-inventory.json at rebase base,
                                 # post = this cycle's own local regen output
  pre count: 49438 post count: 49438
  added: 0 removed: 0
  total changed: 90

  equipment/equipment_modifier changes: 20
  Counter({('ingested-magnitude', 'grounded'): 20})
  [13 core_rulebook + 5 advanced_class_guide + 2 advanced_race_guide unit ids, listed above]

  non-equipment changes (other lanes, not mine): 70
  Counter({'class_feature': 63, 'trait': 7})
  ```
  Sum 20 + 70 = **90**, matching the whole-corpus diff's own total exactly. This cycle's real
  closure is **exactly 20**, corpus-wide, none of it reclassification.

- **Build scope verified (at the final commit SHA):**
  - `cargo test --locked --lib rules_core::equipment_effects::` — **82/82 pass** (10 new:
    3 magic_items STAT-fallback tests, 6 general SKILL-fallback tests, 1 explicit-wins negative
    control already counted in the 6). Confirmed RED first for the two positive-fallback tests
    (`elixir_of_swimming_yields_a_real_swim_skill_bonus_from_tempbonus`,
    `dust_of_appearance_yields_a_real_negative_stealth_tempbonus`, `potion_of_bulls_strength_yields_a_real_str_bonus_from_tempbonus`)
    for the intended reason — `left: None` before the fallback existed — then GREEN after;
    every negative control (`explicit_bonus_skill_wins_over_a_tempbonus_on_the_same_record`,
    `tempbonus_targeting_eq_not_pc_is_never_read_as_a_skill_bonus`,
    `tempbonus_skill_all_wildcard_is_never_read_as_a_single_skill`,
    `explicit_bonus_stat_wins_over_a_tempbonus_on_the_same_record`) passes by construction of the
    guard it proves.
  - `cargo test --locked --bin v06_work_inventory` — **473/473 pass** (2 new end-to-end tests
    against the real on-disk `core_rulebook` corpus:
    `equipment_probe_promotes_a_real_potion_via_its_tempbonus_stat_token`,
    `equipment_probe_promotes_a_real_elixir_via_its_tempbonus_skill_token`).
  - `cargo test --locked --lib` (full lib suite) — **2,971 passed / 5 failed** (pre-existing,
    confirmed unrelated: `class_feature_pool_catalog`/`formula_interpreter_corpus_wide`
    (3 tests)/`companion_chassis` all fail on an unmapped `(wiring_class="derived",
    status="oracle-agree")` pair in `scripts/observer/pf1e_dashboard_producer.py`, a real
    regression from `AT-34-E3-005`'s own already-landed bucket-`V` oracle-disposition work
    (`fef202a566`), already named as an out-of-scope `incident` in this package's own
    `progress.md` by cycle 1 — none of the 5 failing tests are in
    `rules_core::equipment_effects::` or touch this cycle's diff).
  - `cargo test --locked --no-run` (full workspace, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-003`)
    — **exit 0**.
  - `apps/desktop/src-tauri` (separate cargo workspace,
    `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-003-desktop`): `cargo test --locked --no-run
    --manifest-path apps/desktop/src-tauri/Cargo.toml` — tested explicitly because
    `compute_equipment_effects` (whose per-category resolvers this cycle widens) is a real,
    already-wired production path the desktop crate's `character_hub.rs` consumes (`codex = {
    path = "../../.." }`) — the change is additive-only (a new fallback branch, no signature
    change). **Exit 0.**
  - `python3 scripts/completion_atlas.py --check` (local regen) — confirmed run, exit 0
    (`unclassified=0 overlap=0`), `citation_failures=0`, `missing_clearing_mechanisms=0`,
    `stale_derived_at=False`.

- **Sweep population:** no `data/corpus/**` records added or regenerated — only
  `src/rules_core/equipment_effects/general.rs`, `src/rules_core/equipment_effects/magic_items.rs`,
  and `src/bin/v06_work_inventory.rs` (test-only addition) changed.
  `corpus_literal_sweep`: **48,708 examined of 51,482 read**, CLEAN, 0 findings, unchanged
  before/after (`decisions.md §12` L8 — 0 delta expected, 0 delta confirmed).

- **Denominator gate against this package:**
  `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` →
  `files_checked=15 violations=8`, all 8 pre-existing verbatim-quoted corpus prose in
  `progress.md` (`FRT_HVY`'s prose about negating critical hits), already flagged by prior
  cycles; this cycle's own new prose (this receipt, the `progress.md` entry, the `kanban.md` row
  addendum) contains no bare number formatted as a percentage.

- **Oracle pin:** `PCGEN_ORACLE_SHA` per `scripts/pcgen-oracle-pin.env`
  (`7f818006e371188e5717fd18d74d18a420747fc6`) — not consulted this cycle (no oracle-pinned
  corpus read; the `TEMPBONUS:`/`BONUS:` tokens this cycle reads come from the repo's own
  `data/corpus/`, already ingested).

- **Status:** partial

- **Movement, four buckets:** closure (**20** units corpus-wide, `ingested-magnitude → grounded`,
  every one a genuinely newly-promoted, already-wired compute path widened by one real token type
  — not a reclassification of already-DONE evidence, and not co-mingled with any other cycle's
  work, isolated by kind in the id-diff above); reclassification (0 — unlike cycle 2's
  `BASEITEM:` widening, which incidentally corrected 16 stale bucket-`V` evidence strings, this
  cycle's `TEMPBONUS:` widening produced no such side effect: every unit it touched moved
  `ingested-magnitude → grounded` directly, confirmed by the id-diff's single transition-shape
  Counter above); reachability (0); instrument-correction (0 — no citation line shifted; this
  cycle's own new tests live entirely inside existing `#[cfg(test)]` modules, below every cited
  production-code line, confirmed by `completion_atlas.py --check`'s `citation_failures=0` and
  `stale_derived_at=False`).

- **Notes:**
  - **A real, cheap, wired closure found by widening what a compute path READS, not by building
    a new one** — the same shape cycle 2's `BASEITEM:` chase used, applied to a different token
    family (`TEMPBONUS:` vs `BONUS:`). `compute_equipment_effects` already calls both
    `general::compute_general_effect` and `magic_items::compute_magic_items_effect`
    unconditionally on every record (category is a descriptive label derived AFTER the fact from
    which resolver matched, confirmed by reading `equipment_effects.rs` lines 298–348) — so
    widening either function reaches every book's corpus by construction, with no per-book or
    per-category dispatch change needed. This is genuinely corpus-wide, not core_rulebook-scoped:
    3 books moved (`core_rulebook`, `advanced_class_guide`, `advanced_race_guide`).
  - **A real correctness catch found during corpus-wide verification, before shipping**: PCGen's
    `ALL` skill wildcard (`Setting Stone (Invigoration)`, `ultimate_psionics`,
    `TEMPBONUS:PC|SKILL|ALL|2|TYPE=Morale`) would have been silently misread as a bonus to one
    skill literally named "ALL" without the explicit guard added — this unit correctly stays in
    bucket `M` (out of this cycle's own closure count) rather than being closed with a fabricated
    value. Named here per `AGENTS.md` rule 7 ("a proof is only as wide as the cases it covers") —
    the corpus-wide census this cycle ran before writing code is what surfaced this shape at all;
    a `core_rulebook`-only census would never have found it (`Setting Stone` is
    `ultimate_psionics`).
  - **Two different equipment shapes stay genuinely different, as the dispatch brief asked to
    confirm.** This cycle's fix reaches ONLY the same-line shape
    (`equipment_table_entry_with_corpus_magnitude`, 262 → 249) — every `TEMPBONUS`-bearing unit in
    this territory already carried its magnitude token on its own corpus line (a potion IS the
    record that states its own effect; there is no alias/closure indirection for a consumable the
    way `Crossbow (Light)`'s `BASEITEM:` alias needed one). The closure-only shape
    (`equipment_own_line_has_no_magnitude_but_closure_wiring_class_does`, 146, unchanged this
    cycle) needs a DIFFERENT mechanism per cycle 2's own finding — confirming, a third time now,
    that one fix does not cover both shapes.
  - **Why 20, not more.** Every `TEMPBONUS`-bearing equipment/equipment_modifier record still in
    bucket `M`, corpus-wide, was read (not assumed): 30 total carry some `TEMPBONUS` token; 6 of
    those carry only a compound shape (`VAR`, `CASTERLEVEL`, `SAVE|Fortitude,Reflex,Will`,
    `SITUATION`) this cycle's literal-single-target guard correctly declines rather than guesses;
    1 (`Setting Stone (Invigoration)`) is the `ALL` wildcard named above; the remaining 23 all
    carry a literal single-stat/single-skill `TEMPBONUS`, of which 20 had no competing explicit
    `BONUS:` chain already winning and so moved, and the other 3 already had every checked field
    populated by some other already-wired mechanism (confirmed against the id-diff: exactly 20
    status transitions occurred, not 23 — the 3 not counted here were already `grounded`/`V`
    before this cycle via a different field, so this fallback's own contribution to them was a
    no-op, not a missed closure).

- **Remainder — every unit in this cycle's two EQUIPMENT sub-causes, named by real mechanism, at
  HEAD (post this cycle's own local regen):**

  | Sub-cause (evidence string) | Population before this cycle | Closed this cycle | Population after |
  |---|---:|---:|---:|
  | `equipment_table_entry_with_corpus_magnitude` | 262 | 13 | **249** |
  | `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does` | 146 | 0 | **146** |

  Sum: 249 + 146 = **395**, `core_rulebook`-scoped; this cycle's own closure (13) plus this
  remainder (395) = 408, matching this cycle's own start-of-cycle territory exactly.
  Corpus-wide, this cycle closed 20 (13 `core_rulebook` + 5 `advanced_class_guide` +
  2 `advanced_race_guide`); the remainder in every other book of these two sub-causes is
  unaffected by this cycle (its exact corpus-wide count is not re-derived here — cycle 2's own
  remainder statement, `core_rulebook`-scoped as this criterion is `core_rulebook`-scoped, is the
  authoritative per-book accounting; a corpus-wide re-derive of every book's remainder is
  Epic 5's forward-plan territory, not this criterion's).

  **Real sub-shapes composing the `core_rulebook` remainder (395), re-using cycle 2's own
  9-mechanism census, now updated by this cycle's own 20-item removal (13 of them
  `core_rulebook`) and this cycle's own new discovery (the `TEMPBONUS`-bearing subset, now
  exhausted):**

  | # | Sub-shape (mechanism) | Population (corpus-wide territory, post this cycle) | Disposition |
  |---:|---|---:|---|
  | 1 | `CHOICE_GATED` — `%CHOICE`/`CHOOSE:` present | 99 | Sibling lane's territory (choice-selection/explanation-id wiring), unchanged. |
  | 2 | `VAR` bonus chain, no choice, no compound `TEMPBONUS` alternative | 121 | New engineering, bucket-B-shaped: cross-subsystem (equipment ↔ class-feature) wiring, per-target. Unchanged. |
  | 2b | `TEMPBONUS` compound/wildcard shapes (`VAR`, `CASTERLEVEL`, `SAVE|<list>`, `SITUATION`, `ALL`) | 7 | New engineering (multi-target semantics this cycle's literal-single guard correctly declined) OR (the `ALL` case) needs a genuinely different field this struct does not have. Discovered this cycle, not fixed. |
  | 3 | No bonus chain, no real prose magnitude — internal PCGen chassis/plumbing | 99 (100 minus the 1 `Crossbow (Light)` cycle 2 already closed) | Genuinely empty of mechanical content. |
  | 4 | No bonus chain, real prose magnitude only (named artifacts, non-`TEMPBONUS` potions) | 71 | New engineering: a description-linked magnitude probe, cycle 2's own named best-ROI candidate for a future cycle. |
  | 5 | `ITEMCOST` only | 9 | No compute-path gap; nothing to compute. |
  | 6 | `EQM` (no `ResolvedEquipmentEffect` slot) | 3 | New field + new subsystem, too small to justify alone. |
  | 7 | `EQMWEAPON` range/crit modifiers | 3 | New engineering (equipment-modifier composition onto a weapon). |
  | 8 | `ITEMCOST,WEAPON` | 2 | Same as row 5. |
  | 9 | `WEAPON` fixed effect, no field | 1 | Same shape as row 7. |

  Sum: 99+121+7+99+71+9+3+3+2+1 = **415** — 20 more than the 395 `core_rulebook`-only figure
  above because this table (inherited from cycle 2) is corpus-wide territory-scoped, not
  `core_rulebook`-scoped; the 20 removed this cycle (13 `core_rulebook` + 7 other-book) are
  subtracted from rows 2/2b relative to cycle 2's own corpus-wide count of 409 + non-`core_rulebook`
  units not previously enumerated in cycle 2's `core_rulebook`-only table — this row is provided
  as a discovery record for the next cycle, not as this receipt's own audited population (the
  audited, exactly-summing population is the `core_rulebook`-scoped 249+146=395 table above,
  matching this criterion's own `core_rulebook` scope).

  **Every other `M` sub-cause, and buckets V/D/U/X, are untouched by this cycle** (out of
  territory by the dispatch brief's own no-collision rule) — unchanged from cycle 2's own
  statement: `ability_content` 217, `race_trait_generic` 119, `template_content` 96,
  `in_catalog_with_corpus_magnitude_but_no_observed_consumer` 47, `domain_content` 34,
  `skill_content` 19, `spell_list_entry_with_resolved_level` 15,
  `race_trait_states_a_universal_sheet_modifier_pending_compute` 2, `V` 114 (this cycle's own
  regen — `core_rulebook` V shifted from 104 to 114 by the co-mingled, already-committed
  `AT-34-E3-002` cycle 5 fix, +10, not this cycle's own change), `D` 366, `U` 10, `X` 115.

- **Next-cycle plan:**
  1. **Row 4 (71 units, description-linked prose magnitude) remains cycle 2's own
     highest-ROI-remaining candidate** — a genuinely generic new probe shape (potions/artifacts
     whose real effect lives only in `description`/`DESC:` prose), plausibly closing many at
     once, corpus-wide.
  2. **Row 2b (7 units, `TEMPBONUS` compound/wildcard shapes this cycle declined) is a small,
     well-scoped follow-up**: `SAVE|Fortitude,Reflex,Will`/`CASTERLEVEL`/`VAR`-shaped `TEMPBONUS`
     tokens on `Otherworldly Kimono` and `Setting Stone (Invigoration)` would need either a
     multi-target `SkillCheckBonus`/`AbilityScoreBonus` shape (a real struct change, not a
     one-line fallback) or dedicated new fields — worth costing before committing, given the
     population is tiny.
  3. **Rows 6/7/9 (7 units) remain too small to justify new `ResolvedEquipmentEffect` fields
     alone** — same disposition cycle 2 gave them.
  4. **The shared regen cycle** must pick up this cycle's source change (the two `TEMPBONUS`
     fallbacks) the next time it commits `docs/work-inventory.json`'s three-pass pipeline — this
     cycle already ran that regen locally and confirmed the exact effect (not merely predicted):
     `core_rulebook` M 957→944 (−13), corpus-wide 20 real closures across 3 books (13
     `core_rulebook` + 5 `advanced_class_guide` + 2 `advanced_race_guide`), DONE +20 corpus-wide
     from this cycle's own change (co-mingled in the local regen's raw output with 70 more
     changes from `AT-34-E3-002` cycle 5 and `AT-34-E4-002` cycle 6, both already committed and
     isolated by kind above, not this cycle's own claim). The shared cycle should treat any
     mismatch from these exact 20 unit ids as a real regression, not noise.
  5. **`ability_content` (217, a sibling lane's territory) remains the largest overall `M`
     sub-cause.**
