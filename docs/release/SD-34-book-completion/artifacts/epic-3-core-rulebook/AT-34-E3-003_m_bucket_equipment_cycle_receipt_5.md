# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-003 (bucket `M` — EQUIPMENT sub-causes, cycle 5)

- **Commit SHA:** `6574786ca1` (source fix + tests, checkpointed and pushed before this receipt;
  this receipt's own commit follows on top, same pattern cycles 2–4 used).

- **Continuation of, not a duplicate of,** `AT-34-E3-003_m_bucket_equipment_cycle_receipt.md`
  (cycle 1, `7147fd86ab`), `_2.md` (cycle 2, `0519220786`/`3ffa80cc20`), `_3.md` (cycle 3,
  `ac1cd80dfc`), `_4.md` (cycle 4, `3822c0c1d8`), all already merged into `tranche/14` before
  this cycle started. Worktree opened stale (at the tranche cut, `ea2b3396f2`); `git fetch
  origin && git rebase origin/tranche/14` moved HEAD to the real tip, `0b87ef300c`
  (`AT-34-E3-002` cycle 7's receipt self-SHA fixup) — six AT-34-E3-003 cycles, three shared
  regen waves (16/18/19) and two other epics' work had landed since this wave's own dispatch
  brief was written. The brief's own figures (`core_rulebook` M = 972, split 276+147 = 423)
  were stale by construction; every figure below was independently re-derived at the real
  rebase base before any code was read or written (`decisions.md §12` L2).

- **Re-derived at cycle start (never trusting the inherited figure):**
  `python3 scripts/completion_atlas.py --book core_rulebook --check` at the rebase base →
  `core_rulebook` M = **812** (matches cycle 4's own post-fix figure and the wave-19 shared
  regen's own closing figure exactly, live-confirmed). Split, read directly off
  `docs/work-inventory.json` (a Python script, not inherited prose):
  `equipment_table_entry_with_corpus_magnitude` **164**,
  `ability_content_table_holds_record_magnitude_not_yet_computed` 217 (sibling lane, off
  limits), `equipment_own_line_has_no_magnitude_but_closure_wiring_class_does` **99**,
  `race_trait_generic_table_holds_record_magnitude_not_yet_computed` 119 (sibling lane, off
  limits), `template_content_table_holds_record_magnitude_not_yet_computed` 96 (sibling lane,
  off limits), `in_catalog_with_corpus_magnitude_but_no_observed_consumer` 47,
  `domain_content_table_holds_record_magnitude_not_yet_computed` 34,
  `skill_content_table_holds_record_magnitude_not_yet_computed` 19,
  `spell_list_entry_with_resolved_level` 15,
  `race_trait_states_a_universal_sheet_modifier_pending_compute` 2. Sum = 812, confirmed.
  **This cycle's territory (the same two EQUIPMENT sub-causes) is 164 + 99 = 263** at cycle
  start — cycle 4's own stated remainder, independently reproduced.

- **Fresh qualifier-shape census, re-derived by script against real corpus JSON (not
  inherited from cycle 4's table)** — for every one of the 263 units, resolved its real
  on-disk corpus record (by `corpus_key` + `source_line`) and read `raw_bonus_chains`
  directly:

  | Sub-shape (qualifier, own corpus line) | same-line | closure-shape | Disposition, this cycle's own read |
  |---|---:|---:|---|
  | *(no chain)* | 119 | 70 | 182 of 189 carry ONLY `COST`/`WT` as their driving `MAGNITUDE_TOKENS` field — nothing beyond price/weight to compute, same disposition cycles 2/3 gave. Swept every OTHER `MAGNITUDE_TOKENS` prefix (`DR`/`SR`/`SPELLS`/`TEMPBONUS`/`PLUS`/`CRITMULT`/`CRITRANGE`/`RANGE`/`REACH`/`ACCHECK`/`AC`/`DAMAGE`) across all 189: exactly 7 carry a real, non-COST/WT magnitude — 3 `SPELLS` (spell-like-ability grants; corpus-wide sweep shows only 7 of 41 `SPELLS`-bearing equipment records anywhere are even in bucket `M`, the rest already `V`/`text-complete` — too small a realized population to justify a new subsystem this cycle), 2 `SR` (both `%CHOICE`-valued, same boundary as below), 1 `TEMPBONUS\|COMBAT\|AC` (**this cycle's fix**), 1 `DR`+`PLUS` (`Special Ability ~ Invulnerability ~ Armor`, `DR:5/Magic` — no `DR` field exists anywhere in `ResolvedEquipmentEffect`; new subsystem, 1 unit corpus-wide-unconfirmed, too small alone). |
  | `VAR` | 9 | 9 | Correctly declined (cycle 4): formula-valued, `PRE`-gated `Intelligent Item ~ Alignment` chassis family — needs `formula_interpreter` + real character-alignment context, a different criterion's scope. |
  | `COMBAT` | 8 | 8 | **Investigated this cycle (cycle 4 flagged this as "the single highest-value next investigation," suspecting a resolver bug).** Read all 16 real records by hand: every one is `BONUS:COMBAT\|AC\|%CHOICE\|TYPE=...` (`Special Ability ~ Bonus AC / Deflection`, `BNS_AC_DEFL`, and 14 siblings) — a player-chosen value, not a resolver gap. **Correction, not confirmation, of cycle 4's plan item** (retro `correction` event logged). |
  | `SAVE` | 6 | 6 | Same `%CHOICE` boundary, confirmed this cycle (`BONUS:SAVE\|FORTITUDE,REFLEX,WILL\|%CHOICE\|TYPE=...` and siblings). Not "no wired resolver, real candidate" as cycle 4's own table framed it — no resolver can answer a `%CHOICE` without new character-input plumbing (cross-subsystem, matching `VAR`'s own class). |
  | `ITEMCOST` | 7 | 2 | No compute-path gap; a material/masterwork PRICE surcharge, not a combat/skill/save magnitude any `ResolvedEquipmentEffect` field models — same disposition cycles 2/3 gave. |
  | `EQM` | 3 | 0 | New field + new subsystem (`WEIGHTADD`/`HANDS`), too small to justify alone — same disposition cycle 3 gave. |
  | `ITEMCOST, WEAPON` | 3 | 0 | Same as `ITEMCOST` above. |
  | `EQMWEAPON` | 3 | 0 | `RANGEMULT`/`CRITRANGEDOUBLE`/`RANGEADD` — three distinct new mechanics, 1 unit each; same disposition cycle 3 gave. |
  | `SKILLRANK` | 2 | 2 | **Investigated this cycle.** Confirmed corpus-wide: exactly 4 records total (all `core_rulebook`). The magnitude itself is literal (`99`, a "maximize ranks" flag, not `%CHOICE`-gated) but the mechanic — a skill-rank ceiling override — has no field anywhere in `ResolvedEquipmentEffect`; new subsystem, 4 units corpus-wide, too small alone. |
  | `STAT` | 1 | 1 | `%CHOICE`-gated (`BNS_ENHC_AB`, `Special Ability ~ Bonus Ability / Enhancement`) — same boundary as `COMBAT`/`SAVE` above, confirmed by direct read. |
  | `SKILL` | 1 | 1 | `%CHOICE`-gated (`BNS_SKL_CMP`, `Special Ability ~ Bonus Skill / Competence`) — same boundary, confirmed by direct read. |
  | `SPELLCAST` | 1 | 0 | `Special Ability ~ Bonus Spell`, `SPELLCAST\|CLASS.ANY;LEVEL.%CHOICE\|1` — `%CHOICE`-gated (which class level, not a fixed magnitude). |
  | `WEAPON` | 1 | 0 | `Special Ability ~ Speed ~ Weapon`, `WEAPON\|ATTACKS\|1` — an extra-attack grant, no field for it anywhere; new subsystem, 1 unit. |

  Sum same-line: 119+9+8+7+6+3+3+3+2+1+1+1+1 = **164**. Sum closure-shape:
  70+9+8+2+6+0+0+0+2+1+1+0+0 = **99**. Both match cycle 4's own closing table exactly — no
  drift across the wave-19 shared regen.

- **What this cycle found, that cycle 4's own next-cycle plan got wrong:** cycle 4's plan
  item 1 named `COMBAT`/`STAT`/`SKILL` (20 units, its own count) as records with "an
  already-wired resolver for their own token family, yet stay unclosed — the single
  highest-value next investigation," speculating a category-mismatch or `PRE`-gate bug in
  the existing `arms_armor`/`magic_items`/`general` resolvers. **That speculation is wrong.**
  Every one of those 16+2+2 = 20 records (plus the 6 `SAVE`/`SPELLCAST`-shaped records this
  cycle's own wider read also checked) carries a `%CHOICE` magnitude — PCGen's own
  "player selects a value when attaching this modifier" grammar — not a fixed literal any
  resolver could read regardless of category wiring. This is the SAME correctness boundary
  cycle 4's own `VAR`/`PRE`-gated finding already named and correctly declined, not a new
  one: `str::parse::<i16>()` on the literal string `"%CHOICE"` fails and correctly returns
  `None` rather than fabricating a number (`equipmods.rs`'s own doc comment, `SR:%CHOICE`,
  already stated this exact discipline for the sibling `SR` field before this cycle started —
  this cycle's finding is that the SAME discipline, not a bug, explains `COMBAT`/`STAT`/
  `SKILL`/`SAVE`/`SPELLCAST` too). Retro `correction` event logged, `--verified-by` naming the
  direct corpus read this cycle ran.

- **The one real, closable gap this cycle found:** `armor_class_bonus_from_bonus_chains`
  (`arms_armor.rs`) reads only `BONUS:COMBAT|AC|<n>|...` chains — never
  `TEMPBONUS:<target>|COMBAT|AC|<n>|...`, PCGen's temporary/consumable-triggered sibling
  token. Cycle 3 already found and fixed this exact shape for `general.rs`'s `SKILL` field
  and `magic_items.rs`'s `STAT` field (`tempbonus_skill_fallback` / the `TEMPBONUS|STAT`
  fallback) but never extended it to `arms_armor`'s `AC` field — the third and, per this
  cycle's own corpus-wide sweep, last unhandled `TEMPBONUS` target family.
  `core_rulebook:equipment:cloak_of_the_manta_ray` carries no `BONUS:COMBAT|AC` chain at all
  (`raw_bonus_chains` is empty) — its real +3 natural armor bonus (worn in salt water) is
  stated only as `TEMPBONUS:PC|COMBAT|AC|3|TYPE=NaturalArmor`, which `wiring_class` already
  correctly tags `computed:tempbonus` while `compute_arms_armor_effect` had nothing to answer
  with. Sweep for the realized population: `grep -rl '"TEMPBONUS"' data/corpus/*/equipment/*/*.json
  | xargs grep -l COMBAT` → exactly **1** record corpus-wide. Small, but generic by
  construction: the fix reaches any future or currently-unswept record with this shape,
  corpus-wide, with no per-book code (same idiom cycle 3 established).

- **Fix:** `tempbonus_combat_ac_fallback` (new function, `arms_armor.rs`), consulted via
  `.or_else` only when no explicit `BONUS:COMBAT|AC` chain wins (the same ordering discipline
  every sibling fallback in this file already follows — `eqmarmor_chain_value`,
  `tempbonus_skill_fallback`), gated to target `PC`/`ANYPC` only — an `EQ`-targeted
  `TEMPBONUS` is a different, equipment-side effect, the same distinction
  `tempbonus_skill_fallback` already draws for the identical reason. `compute_arms_armor_
  effect` is already called unconditionally on every equipped item by
  `compute_equipment_effects`, so the fix reaches every book with no dispatch change.
  `equipment_key_is_wired` already checks `item.armor_class_bonus.is_some()` — no probe
  change needed (unlike cycles 2/4, which had to widen the probe itself).

- **Files touched:**
  - `src/rules_core/equipment_effects/arms_armor.rs` — `tempbonus_combat_ac_fallback` +
    3 new tests (positive real-corpus-tokens case, explicit-chain-outranks-fallback negative
    control, `EQ`-target negative control).
  - `src/bin/v06_work_inventory.rs` — 1 new integration test
    (`equipment_probe_promotes_a_real_cloak_via_its_tempbonus_combat_ac_token`), mirroring
    the existing `..._tempbonus_skill_token`/`..._tempbonus_stat_token` precedents exactly.
    No production-code change in this file this cycle (unlike cycles 2/4).

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — run against this cycle's own diff
  (`git diff --unified=0 HEAD~1..HEAD -- src/rules_core/ src/bin/`) in isolation. The
  §6-literal command (whole-tranche diff since `merge-base HEAD origin/develop`, scoped to
  Epic 3's file-touch set) reports 486 matches, none introduced by this cycle — every sampled
  hit is a pre-existing `sd32_class_ingest`-shaped wiring-signal string constant from work
  already committed before this cycle started (the exact legitimate-identifier shape the
  pattern's own doc comment names as intentionally still caught, "to keep catching
  `sd19_class_catalog`"), confirmed by sampling and by this cycle's own isolated diff being
  clean.

- **Wired-integration audit result:** `OK_NO_TOKENS` (both the isolated diff and, by
  inspection, the cumulative one — no `STUB`/`MOCK`/`placeholder`/`todo`/`fixme`/`hack` token
  in any hunk this cycle added).

- **Acceptance criterion (verbatim, `epic-breakdown.md` AT-34-E3-003):** "buckets M, V, D, U,
  X close ... Evidence: per bucket, the atlas reporting zero for `core_rulebook`, with
  movement in four buckets. A count that drops because measurement changed is
  instrument-correction, not closure." This cycle moves `M` toward zero by 1 real unit (a
  genuine closure, not an instrument correction) and reports the remainder honestly.

- **RED → GREEN:** reverted the `.or_else(|| tempbonus_combat_ac_fallback(record))` call,
  re-ran `tempbonus_combat_ac_token_resolves_when_no_bonus_chain_exists` alone → FAILED for
  the intended reason (`left: None, right: Some(3)`). Restored the call, re-ran the same test
  → PASSED. Then the full scoped suites: `cargo test -p codex --lib
  rules_core::equipment_effects::` — **89/89 pass** (was 86; +3 new). `cargo test --bin
  v06_work_inventory equipment` — **30/30 pass** (was 29; +1 new).

- **Figures + their re-derive commands:**
  - `core_rulebook` M at cycle start: `python3 scripts/completion_atlas.py --book
    core_rulebook --check` → **812**, denominator `core_rulebook` population 6,701.
  - Territory split at cycle start: Python script against `docs/work-inventory.json`
    (`u.get('book')=='core_rulebook' and u.get('status')=='ingested-magnitude' and
    u.get('evidence')=='equipment_table_entry_with_corpus_magnitude'` / `'...closure_
    wiring_class_does'`) → **164** / **99**, denominator `core_rulebook` M 812.
  - Qualifier-shape census: Python script cross-referencing the same 263 units against their
    real `data/corpus/core_rulebook/equipment/**/*.json` records' `raw_bonus_chains` — see
    the table above, denominator 263 (164 + 99).
  - Post-fix, this cycle's own regen (`v06_work_inventory`, no `--allow-stamp-loss`, fed the
    real `corpus_literal_sweep --json-out` + `derived_evaluator_fixture_check --json-out`
    reports below): `python3 scripts/completion_atlas.py --book core_rulebook --check` →
    `core_rulebook` M → **811** (−1, exactly this cycle's own closure). Split, same script:
    `equipment_table_entry_with_corpus_magnitude` **163** (−1), `equipment_own_line_has_no_
    magnitude_but_closure_wiring_class_does` **99** (unchanged, confirmed — the fix reaches
    only the same-line shape). `core_rulebook` DONE 4613→4616 (+3: this cycle's own +1, plus
    +2 co-mingled — see Movement below).

- **Build scope verified:** `cargo test --locked --no-run` — **exit 0**, full workspace, run
  at `6574786ca1`. `apps/desktop/src-tauri` tested explicitly — **exit 0** (`Finished \`test\`
  profile [unoptimized + debuginfo] target(s) in 6m 39s`), run at `6574786ca1`.

- **Sweep population:** `cargo run --locked --release --bin corpus_literal_sweep --
  --json-out …` → **48,708 examined of 51,482 read, 0 findings, CLEAN** — identical to cycle
  4's own baseline (`git status --porcelain -- data/corpus/` confirms empty: no corpus records
  touched this cycle, so the examined-population must be unchanged, and is). `derived_
  evaluator_fixture_check --json-out …` → **1,839 units cleared over 2,580 fixture rows, 0
  failed, 0 not ingested**.

- **Oracle pin:** `PCGEN_ORACLE_SHA` per `scripts/pcgen-oracle-pin.env` — `7f818006e371188e5717fd18d74d18a420747fc6`
  (not consulted this cycle; this fix's figure comes from `wiring_class`'s own real corpus
  read, not an oracle round-trip).

- **Row-count command output (whole-inventory id-diff, pre = `0b87ef300c`'s committed
  `docs/work-inventory.json`, post = this cycle's own regen — the artifact this cycle's
  status is set from):**
  ```
  pre count: 49438 post count: 49438
  added: 0 removed: 0
  total changed: 3
    core_rulebook:class_feature:ranger_combat_style_archery: engine-does-not-hold -> grounded
      (AT-34-E3-002 cycle 7's own already-committed fix, NOT this cycle's own)
    core_rulebook:class_feature:ranger_combat_style_two_weapon_combat: engine-does-not-hold
      -> grounded (same, NOT this cycle's own)
    core_rulebook:equipment:cloak_of_the_manta_ray: ingested-magnitude -> grounded
      (equipment_table_entry_with_corpus_magnitude -> equipment_effect_probe_observed_
      computed_delta) — THIS CYCLE'S OWN, its one real closure
  ```
  This cycle's own closed count is exactly **1** — the size of its own `-> grounded` set,
  matching the `M` delta above exactly (812 → 811).

- **Status:** partial.

- **Movement, four buckets:** **Closure: 1** (this cycle's own — `core_rulebook:equipment:
  cloak_of_the_manta_ray`, `ingested-magnitude` → `grounded`, evidence
  `equipment_effect_probe_observed_computed_delta`). **Co-mingled in this same regen, NOT this
  cycle's own claim:** 2 more closures (`core_rulebook:class_feature:ranger_combat_style_
  archery` and `..._two_weapon_combat`, `engine-does-not-hold` → `grounded`) — these are
  `AT-34-E3-002` cycle 7's already-committed, already-verified fix (`f0d724d2c8`, deferred its
  own `docs/work-inventory.json` write per this wave's file-ownership rule; its own receipt,
  `4ff99d62b5`, already states this exact delta, `core_rulebook` C 201→199). Confirmed by
  reading the diff directly (`git diff docs/work-inventory.json`): whole-inventory id-diff
  shows **0 added, 0 removed, exactly 3 changed** — 1 `equipment` (mine) + 2 `class_feature`
  (cycle 7's), no other units moved. Reclassification: 0. Reachability: 0.
  Instrument-correction: 0 (this cycle touched no corpus records and no fixture/sweep
  disagreement surfaced). `completion_atlas.py --check` (whole corpus): DONE 24724→24727,
  M 4679→4678, C 4182→4180 (2 of that 2-unit move is cycle 7's, not mine — `M` is the only
  bucket this cycle's own criterion claims), `unclassified=0`, `citation_failures=0`.
  `scripts/box_ledger.py --check` — 7 pre-existing stale-count WARNINGs against `THE-BOX.md`
  (inherited SD-33 artifact, unowned by this mechanism, already documented by prior SD-34
  cycles, e.g. `AT-34-E3-001_class_feature_option_pool_with_magnitude_cycle_receipt_3.md`),
  `uncovered=28655 oracle_disagreement=0 unverifiable_done=0 stale=False` — exit 1 on the
  stale-count WARNINGs alone, not a new defect this cycle introduced.

- **`docs/work-inventory.json` / `completion-atlas.json` deliberately NOT committed this
  cycle** — cycles 1–4's own established file-ownership pattern (`decisions.md §9`; the shared
  end-of-wave regeneration cycle owns both files). Every figure in this receipt comes from
  this cycle's own local regen; both files `git restore`d before the commit that ships this
  receipt. The next shared regen will pick up this cycle's own real closure together with the
  `AT-34-E3-002` cycle 7 co-mingled units already named above.

- **Notes (judgment calls):**
  - This cycle deliberately did NOT attempt `SKILLRANK` (4 units), `EQM`/`EQMWEAPON`/`WEAPON`
    (7 units, 5 distinct new mechanics), `SPELLS` (3 `core_rulebook` of 7 corpus-wide-in-`M`),
    or `DR` (1 unit) — each requires a genuinely new field/subsystem for a population too
    small, on its own, to meet this bundle's own stated generic-fix ROI bar (~345 units/hour
    vs ~20 for book-scoped work). Named, not silently dropped.
  - The 34-unit `%CHOICE` family (`COMBAT` 16 + `STAT` 2 + `SKILL` 2 + `SAVE` 12 +
    `SPELLCAST` 1 + `SR` 2, all within this cycle's own census table above — the 2 `SR` units
    are counted inside the "(no chain)" row, not double-counted here) and the 18-unit
    `VAR`/`PRE`-gated family are BOTH correctly stuck on the same class of correctness
    boundary: a magnitude that only exists at the point of a specific character's own
    build-time choice, not a corpus fact. Closing either needs new `EquipmentSelection`-level
    plumbing (a chosen numeric value per attached modifier) plus, for `VAR`,
    `formula_interpreter` + real alignment context — real, substantial, cross-subsystem
    engineering, correctly out of a single cycle's scope.
  - Territory this cycle closed exactly 1 of its assigned 263 — the smallest per-cycle count
    in this lineage (cycles 1–4 closed 14, 1, 20, 132 respectively). This is not a productivity
    regression relative to those cycles' own remaining, pre-picked-over population: cycles
    1–4 progressively closed every generically-closable shape (`BASEITEM` chase, `DAMAGE:`,
    `TEMPBONUS`×2, `VAR` dispatcher wiring) this territory had. What is left, confirmed by
    this cycle's own exhaustive re-read of all 263 real records, is dominated by genuine
    correctness boundaries and sub-10-unit new-subsystem shapes — reported honestly rather
    than forcing a low-value fix to inflate the count.

- **Next-cycle plan:**
  1. **Nothing in this cycle's own two EQUIPMENT sub-causes is generically closable by an
     existing instrument without new subsystem work.** A future cycle should treat the
     remaining 262 as Epic 5's forward-plan territory (per-shape projected cost, matching
     cycles 2–4's own precedent), not re-attempt the same census a fifth time.
  2. **If a future cycle DOES want to close the `%CHOICE` family (34 units) or `VAR` (18
     units, `AT-34-E3-003`'s own remainder, not this territory alone — `ability_content`'s
     217 sibling-lane units may share the same boundary)**, the real prerequisite is
     `EquipmentSelection`-level "chosen modifier value" plumbing — a genuine new feature
     (character-build-time input), not a compute-path widening. Scope and cost it before
     attempting.
  3. **`ability_content` (217, sibling lane) remains the largest overall `M` sub-cause**,
     unchanged by this cycle (out of territory).
