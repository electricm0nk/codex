# v0.6 Alpha Release Swarm — Report

Status: DRAFT (living document, updated as waves complete). Not an attestation yet.

Owner of this document: lead (orchestrator) collates; QA owns the attestation
content per §4.4 and §7.1 of `docs/release/v0.6/release-swarm.md`.

## Agent Status (operator directive, 2026-07-24: updated on every write to
this report, not just SWARM_STATUS.md)

| Agent | Status | Detail |
|---|---|---|
| backend | idle | items 7, 17, item-1 design pass, shape (c), attack-bonus slice, and the corpus-fixture unresolved-selection fix (`647e52aa`) all landed and independently verified; standing by |
| frontend | working | Defense-tab render-staleness fix landed (`7360fe4a`); now dispatched to render the honest "outside demo corpus" indicator on top of backend's new DTO fields |
| qa | working | root-caused the Spells tab finding to the bundled-corpus-fixture issue (fixed by backend); verifying frontend's Defense-tab fix now, backend's fix next |

---

## (a) Red-green test catalogue completeness — alpha bar §1 item 4

Per-calculation status against `tests/` as of 2026-07-23 (QA baseline survey,
`tranche/6` @ commit `43f8d46`, before wave-1 landed):

| Calculation | Status | Evidence |
| :--- | :--- | :--- |
| Ability scores | Covered | Base scores threaded through `character_input.rs`; asserted across `tests/sd13_*_level1_*baseline.rs` |
| Attack rolls | Covered | `base_attack_bonus` computed and asserted per class/level in `tests/sd13_*` (levels 1-10), `tests/sd18_*` (levels 11-20) |
| BAB/save progression (single-class) | Covered | `tests/sd13_*_base_attack_and_saves.rs` per class |
| BAB/save progression (multiclass stacking) | **Partial** | `tests/sd21_multiclass_fighter_wizard_chassis_computes.rs`, `tests/sd24_multiclass_fighter_wizard_split.rs`, `tests/sd24_multiclass_deterministic.rs` all assert real numeric BAB/save-fraction stacking — but **only for the Fighter/Wizard pair** (good-BAB+good-Fort vs poor-BAB+poor-Fort/Reflex+good-Will). No coverage for 3/4-BAB classes (Rogue, Cleric, Bard, etc.), two-good-save stacking, or 3-class multiclass. |
| Skill allocation | Covered | `tests/sd20_skill_allocation_{class_skill,cross_class,max_rank_cap,parity,untrained}.rs` |
| Spell slot allocation | Covered | `tests/sd13_*_spells_per_day_counts.rs`, `tests/sd20_spellbook_*.rs` (per school) |
| AC | Covered, with a caveat | `defense.baseline_armor_class` asserted with real values (e.g. `tests/ge08_preview_bridge.rs`, `tests/sd21_wizard_chassis_computes.rs` — value 17); equipment AC delta asserted (`tests/sd20_tabletop_readiness_integration.rs:1473` — Chain Shirt +4). Caveat: `src/rules_core/pilot_compute.rs` comments indicate AC stays `claim_blocking`-gated outside certain class-chassis paths — scope of "every reachable AC calc" needs re-verification once frontend widens which chassis are reachable. |
| Durability | **Gap — no production code (definition resolved)** | Lead ruling (recorded in `risks-and-open-questions.md` item 4, 2026-07-23): durability = character survivability display (max/current/temp HP, nonlethal damage, dying/unconscious/death thresholds), not item hardness. Follow-up survey against that definition: `src/rules_core` has only a single isolated `class_chassis.fighter.level_1_hit_points` explanation value (`pilot_compute.rs:7408-7425`); there is no aggregate `max_hit_points` rolled up across a full level-up chain (unlike AC, which has a `sheet.armor_class` cell), no `current_hp`/`temp_hp` fields anywhere in `contract.rs` or `character_input.rs`, and no nonlethal/dying/unconscious/death state machine. Same shape of gap as carry capacity/encumbrance/money — needs backend build, not QA test-authoring, until an aggregate HP field and state machine exist. See appendix below for sourced threshold rules. |
| Carry capacity | **Gap — no production code** | No `carry_capacity`/`carrying_capacity` computation found anywhere in `src/rules_core`. Not a test gap — the calculation itself doesn't exist. |
| Encumbrance | **Gap — no production code** | Same as carry capacity — no encumbrance/load computation found in `src/rules_core`. |
| Money conversion | **Gap — no production code** | No currency-conversion, starting-gold, or wealth-by-level logic found in `src/rules_core`. Only per-item `cost_gp` pricing exists on equipment records. Corroborates frontend's independent finding (see `SWARM_STATUS.md` "Happened" log) that money/currency has no schema field anywhere in the engine. |
| Level-up hit points | Covered | `tests/sd13_fighter_level1_hit_point_baseline.rs`, `tests/sd20_levelup_*.rs` per class |
| Multiclass stacking (general) | Partial | Base chassis (BAB/save) stacking covered for Fighter/Wizard (see above). Skill points / feats / spell-slot stacking under multiclass not independently verified in this survey — needs a follow-up pass once BAB/save gap is closed. |

**Bottom line (as of the original 2026-07-23 wave-1 survey, before wave-1 landed):**
3 of the 12 alpha-bar calculations (carry capacity, encumbrance, money
conversion) had **zero production implementation**, not just zero tests —
these were backend build items, not QA test-authoring items. Multiclass
BAB/save stacking had real tests but only for one class pair.

### Refresh (2026-07-23, after backend's wave-1 close)

Requested by the lead after skill/level-up/bio/feat/money persistence,
BAB/save widening to Fighter/Wizard/Rogue, and the carry-capacity/encumbrance
calc all landed. Updated rows only; unlisted rows are unchanged from above.

| Calculation | Status | Evidence |
| :--- | :--- | :--- |
| BAB/save progression (multiclass stacking) | **Covered, still partial breadth** | Backend's `d20a5b9` widened `compute_multiclass_base_chassis` to Fighter/Wizard/Rogue via the table-driven `compute_generic_table_chassis` path; QA's `8d814e8` adopted 40 downstream tests into the catalogue (verified against real computation output, not transcribed) across 12 files. Still only 3 of 11 core classes are in the multiclass allowlist (Barbarian/Bard/Cleric/Druid/Monk/Paladin/Ranger/Sorcerer remain out, per backend's own doc comment in `pilot_compute.rs` — each has its own pre-existing standalone-only chassis and would need the same coordinated catalogue-adoption pass this one got). |
| AC (equipment bonus) | Covered, caveat resolved for this angle | Backend audited equipment AC bonus wiring during the encumbrance task (`d475097`'s commit message: "equipment AC bonus itself... was already real and wired via equipment_effects.rs prior to this change — audited first, nothing to fix there"). The `claim_blocking`-gating-breadth caveat from the original survey (which classes/postures can reach a `Computed` AC at all) is unchanged and is really the same issue as risks-and-open-questions.md item 1 (the single hardcoded deterministic posture), not an AC-specific gap. |
| Carry capacity | **Covered** | `src/rules_core/encumbrance.rs` (commit `d475097`) implements `carrying_capacity_thresholds`, transcribed and cited from Archives of Nethys (`aonprd.com/Rules.aspx?ID=118`, fetched 2026-07-23) with the source's own >29 extrapolation rule. QA independently cross-checked the table against the real PCGen `load.lst` data twice — once during the original spec pass, once again as part of catalogue adoption. Catalogue entry: `tests/v06_encumbrance.rs` (commit `a7e8971`, 6 tests, independently authored — different fixture items and assertions than the module's own inline tests), plus 2 inline `#[cfg(test)]` tests remaining in the module itself. |
| Encumbrance | **Covered** | Same file/commit as carry capacity. `compute_encumbrance` sums real per-item corpus weight (`WT:` token) across every `EquippedActive`/`SelectedInactive` selection, flags unresolvable items rather than fabricating a zero, and is wired unconditionally into `PilotReceipt.encumbrance` in `contract.rs` (not gated behind the narrow deterministic-posture check that blocks combat/skill totals). Catalogue entry: `tests/v06_encumbrance.rs` (same file/commit as carry capacity — covers the Medium encumbrance tier and a true-zero-loadout case the inline tests didn't). |
| Money conversion | **Covered (conversion only)** | `src/rules_core/money.rs` (commit `67490ac`) implements `copper_to_denominations`/`denominations_to_copper`/`gp_to_copper` using QA's own formula-spec ratios, explicitly flagged (matching QA's own appendix) as not independently PCGen-source-verified. Deliberately scoped to conversion/spend-tracking only — starting-wealth-by-class (PCGen's `GOLD:` token) stays unresolved, per QA's original finding, not guessed. Catalogue entry: `tests/v06_money_conversion.rs` (commit `a7e8971`, 6 tests, including a real CRB corpus item's exact rounding — Torch, `cost_gp: 0.01`), plus 6 inline tests remaining in the module itself. |
| Durability | **Still a total gap** | Re-confirmed via fresh grep across `src/rules_core/` and `tests/`: zero hits for `max_hit_points`/`current_hp`/`temp_hp`/`dying`/`unconscious` beyond the same single isolated level-1 fighter value found in the original survey. Not touched by wave-1. Still the correct wave-2 target per the lead's ruling (risks-and-open-questions.md item 4) and QA's sourced spec appendix below. |

**Refreshed bottom line:** carry capacity, encumbrance, and money conversion
are no longer production gaps and are now fully in the official `tests/**`
catalogue (`tests/v06_money_conversion.rs`, `tests/v06_encumbrance.rs`,
commit `a7e8971`), independently authored and cross-checked, not adopted
by transcription. Durability remains the one calculation with zero
production surface. Multiclass BAB/save stacking breadth (8 of 11 classes
still outside the allowlist) is the other open item. The narrow
deterministic-`Computed`-posture gate (risks-and-open-questions.md item 1)
is a separate, larger issue: backend's fuller scoping found 4 independent
exactness gates. **AC-gate widening was dropped, not landed** — backend
found the headless compute layer (`compute_pilot_base_chassis`) structurally
has no corpus parameter, so real per-item AC math can't be spliced into the
gate cheaply; bridging that would mean either threading a corpus parameter
through ~347 call sites or moving the `Computed`/`Blocked` decision to the
corpus-aware layer entirely — a real architecture decision, flagged to the
operator as a possible future epic rather than a wave-2 item, not something
to assume is coming. Attack-bonus and general-skill posture widening remain
explicitly deferred too, and may share the same headless/corpus-aware split
once scoped. Backend has moved on to durability, money-conversion PCGen
verification, and comparator field-extraction instead, none of which share
this architectural constraint.

### Full resurvey (2026-07-23, wave-2 close / autonomous operation start)

Requested by the lead at the start of fully-autonomous operation (see
`risks-and-open-questions.md`'s operator directive). A great deal has
landed since the last refresh: durability grounded, the Wizard bootstrap/
first-spell/slot-budget three-layer investigation fully closed, and a
QA-found Wizard spell-save-DC gap fixed. Full table below supersedes both
tables above as the current state of truth; rows unchanged since the last
refresh are repeated for completeness rather than left to cross-reference.

| # | Calculation | Status | Evidence |
| :-- | :--- | :--- | :--- |
| 1 | Ability scores | Covered | Unchanged — `tests/sd13_*_level1_*baseline.rs` |
| 2 | Attack rolls | Covered | Unchanged — `tests/sd13_*`/`tests/sd18_*` per class/level |
| 3 | BAB/save progression (single-class) | Covered | Unchanged — `tests/sd13_*_base_attack_and_saves.rs` |
| 4 | BAB/save progression (multiclass) | **Covered, breadth still 3/11** | Fighter/Wizard/Rogue only (`table_class_id` in `pilot_compute.rs`, confirmed unchanged by direct grep this pass). 40 catalogue tests adopted (`8d814e8`). Barbarian/Bard/Cleric/Druid/Monk/Paladin/Ranger/Sorcerer remain outside the allowlist — each has its own pre-existing standalone-only chassis; widening any of them needs the same coordinated catalogue-adoption pass Rogue got, not a code-only change. |
| 5 | Skill allocation | Covered | Unchanged — `tests/sd20_skill_allocation_*.rs`. Note: real-world reachability is separately constrained by the narrow deterministic-posture gate (item 1 in the risks doc), not a skill-calculation defect. |
| 6 | Spell slot allocation | **Covered, and a real enforcement bug found + fixed this wave** | Base per-day counts unchanged (`tests/sd13_*_spells_per_day_counts.rs`, `tests/sd20_spellbook_*.rs`). New this wave: Wizard's slot-budget *enforcement* was silently broken for every real corpus spell (`parse_wizard_spellbook_spell_id` only recognized the one synthetic seed spell's id shape, `<school>.<level>.<name>`, so real spells like "Grease" were dropped from the consumed-slots sum before the over-budget check ever ran — a Wizard could add unlimited real spells with zero enforcement). Found by frontend live-testing, root-caused precisely, fixed by backend (`365b3a1a`, genuine RED→GREEN: fix disabled, reproduced the bug, re-enabled, confirmed green), live re-verified end-to-end through the real Add Spell UI (risks-and-open-questions.md item 13). Tested at the Tauri command layer (`apps/desktop/src-tauri`'s own suite, 188/188) and via `cargo test --lib` (197/197) — not `tests/**`, since the bug was in command-layer spell-id resolution, not a `rules_core` pure-function gap; no adoption action needed on my side, this is the correct home for that coverage. |
| 7 | AC | Covered (baseline + equipment delta), gate-widening dropped | `defense.baseline_armor_class` and equipment AC delta unchanged and covered. Equipment AC bonus itself independently audited as already real and wired (`d475097`'s commit message). Splicing that into the `Computed`/`Blocked` gate for arbitrary loadouts was scoped, greenlit, then **dropped** after backend found the headless compute layer has no corpus parameter (see the "AC-gate widening" note above) — flagged to the operator as a possible future epic, not attempted. |
| 8 | Durability | **Covered** | `src/rules_core/durability.rs` (commit `0aeed25a`) grounds `compute_max_hp` (maximized level-1 die + average-rounded-up every level after, PF1's named non-rolling default, floored at 1 HP/level) and `classify_durability` (Normal/Staggered/Disabled/Unconscious/Dying/Dead per standard PF1/d20 SRD thresholds). Scoped to single-class Fighter/Wizard/Rogue, same reason as the multiclass BAB/save dispatch (which single level was character-level-1, for the maximized die, is genuinely ambiguous from multiclass `CharacterClassLevel`'s cumulative-level shape). Wired into `SelectedParityDimensions::from_pilot_receipt` (not a `receipt.durability` field the way encumbrance got one) and independently PCGen-verified end-to-end (`tests/sd26_pilot_case_verification.rs`, max_hp=12 matched exactly against a real PCGen export). Catalogue entry: `tests/v06_durability.rs` (commit `ec48b501`, 12 tests, independently authored — different classes/levels than the module's own 13 inline tests, plus a direct second-path cross-check of the PCGen-verified max_hp=12 value), plus the 13 inline tests remaining in the module. |
| 9 | Carry capacity | Covered | Unchanged — `tests/v06_encumbrance.rs` |
| 10 | Encumbrance | Covered | Unchanged — `tests/v06_encumbrance.rs` |
| 11 | Money conversion | Covered (conversion only) | Unchanged — `tests/v06_money_conversion.rs`. Starting-wealth-by-class remains unresolved (risks item 7), correctly not guessed. |
| 12 | Level-up hit points | Covered | Unchanged — `tests/sd13_fighter_level1_hit_point_baseline.rs`, `tests/sd20_levelup_*.rs` |
| — | Wizard spell save DC (not a named bar-4 item, but the same DC family as Paladin/Ranger/Sorcerer/Bard) | **Covered, new this wave** | QA found zero `wizard_spell_save_dc` computation while doing item 11's LST cross-check (risks item 12) — the other 4 caster classes all had it, Wizard didn't; not a bug, an explicitly-disclaimed pre-existing gap. Fixed by backend (`3b397315`): `10 + spell_level + intelligence_modifier`, confirmed against real PCGen data (`cr_classes.lst` `SPELLSTAT:INT`). QA adopted (`e95112a1`): `tests/sd13_wizard_spell_save_dcs.rs` (7 tests, independently authored — caught and corrected one wrong assumption about multiclass behavior by running the real computation rather than guessing), plus fixed the one downstream negative-control test the new records made stale. |

**Resurvey bottom line (updated after durability's catalogue adoption,
`ec48b501`):** all 12 alpha-bar calculations are now either fully covered
or, for multiclass BAB/save breadth specifically, covered-but-narrow (see
below) — every production gap identified across every prior survey (carry
capacity, encumbrance, money conversion, durability) is closed and
catalogued, none left as backend-only inline tests. Multiclass
BAB/save breadth (3/11 classes) remains the largest *known* calculation gap
against the bar's "any class" framing, though it is a well-understood,
bounded, repeatable pattern (Rogue's own widening + 40-test adoption is the
template) rather than new design work. Beyond raw calculation coverage, the
bigger alpha-bar story this wave is Wizard becoming a **second genuinely
UI-reachable class** (class creation, first-spell bootstrap, and slot-budget
enforcement all live-verified end-to-end) — this is bar items 2/3 progress,
not item 4 calculation coverage, but it's the more consequential wave-2
result. The narrow deterministic-`Computed`-posture gate (risks item 1) and
the still-unwidened AC/attack-bonus/skill-posture architecture split remain
the single largest structural distance between current state and "any
class... reaches Computed for the choices a tester actually makes" — this
is a UI-reachability problem layered on top of calculation correctness, not
fixed by adding more calculations.

### Comprehensive consolidation (2026-07-23/24, fully-autonomous-session checkpoint)

Requested by the lead as a checkpoint after a long autonomous run covering
several real defects found and fixed beyond the original calculation-gap
surveys above. This section is the current, authoritative picture; it does
not repeat every historical detail already recorded above, only what has
materially changed or newly landed.

**Calculation coverage** (§a's 12-item table): unchanged from the "Full
resurvey" table above in outcome — all 12 are covered, multiclass BAB/save
breadth is still 3 of 11 classes (Fighter/Wizard/Rogue), same well-understood
bounded pattern, not attempted further this session (no new class was
greenlit for widening). What *did* change is the accuracy of several
already-"Covered" rows, captured as defects below — coverage existing is not
the same as coverage being *correct*, and this session found several real
gaps between the two.

**Alpha-bar items 1-3/7** (§d below is largely stale as of the original
wave-1 draft; the current truth):
- **Item 2/3** (create + advance a character of any class/race): materially
  further along than believed. Fighter, Wizard, and Rogue are all now
  confirmed UI-reachable end-to-end (creation, level-up, multiclass dip) —
  Wizard needed a real three-layer bootstrap fix (class acquisition, first
  spell, slot-budget enforcement, all live-verified), Rogue needed none
  (confirmed reachable with zero gap). **Race support for these three
  classes is not Human-gated** — this was a stale, never-verified assumption
  the swarm inherited and has now disproven: Elf Wizard and Elf Rogue both
  live-verified reaching `Computed`/`Saved` through the real creation UI.
  The other 8 core classes (Barbarian/Bard/Cleric/Druid/Monk/Paladin/
  Ranger/Sorcerer) still have zero chassis computation for any race — this
  is unchanged and is not a quick fix (each needs its own multi-epic
  calculation engine, not a UI/wiring fix).
- **Item 4** (every reachable calc matches PCGen): the 12 named calculations
  are covered per the table above, but "every reachable calculation" is
  narrower in practice than the bar's framing suggests — see the
  bar-distance assessment below.

**PCGen-delta defects found this session** (§b was empty this whole swarm
until now — populated for real below):

| # | Defect | Status |
| :-- | :--- | :--- |
| 1 | Wizard spell-save-DC: no computation existed at all (Paladin/Ranger/Sorcerer/Bard all had it) | **Fixed** (`3b397315`), catalogue-adopted (`e95112a1`) |
| 2 | Wizard spell-slot-budget enforcement: real corpus spells silently bypassed the over-budget check (only the one synthetic seed spell's id shape was recognized) | **Fixed** (`365b3a1a`), live re-verified through the real Add Spell UI |
| 3 | Class-skill-modifier bug: `compute_selected_skill_modifiers` applied the Climb/Intimidate/Swim class-skill `+3` unconditionally — silently wrong for Wizard (whose real class-skill list includes none of the three), coincidentally right for Rogue | **Fixed** (`93a0636d`), catalogue-adopted (`3b843add`), independently re-verified against the real PCGen corpus citations before adoption |
| 4 | Racial ability-modifier gap: Elf/Dwarf/Gnome/Halfling each silently missing one real `+2` mental-ability racial component (Elf: INT, Dwarf: WIS, Gnome: CHA, Halfling: CHA) — the code's own comment mischaracterized Elf's as an "out of scope alternate variant" when it's the CRB-standard default | **Fixed for all 4 races** (Elf `9ec0e036`, Dwarf/Gnome/Halfling `2f05dee4`), **catalogue-adopted for all 4** (Elf `e9d02c25`, Dwarf/Gnome/Halfling `fb01768d`) — each real PCGen citation independently re-verified before adoption, not trusted from the commit message |
| 5 | Racial Small-size effect miscategorization: Gnome's and Halfling's size explanations claim "no numeric effect to attack rolls, AC..." despite correctly citing `SIZE:SMALL` — real PF1 Small size grants +1 AC/+1 attack/-1 CMB-CMD/+4 Stealth | **Fixed** (`2f05dee4`, bundled with defect 4), **catalogue-adopted** (`fb01768d`) — text-only correction; `compute_combat_baseline` has no size-modifier term for *any* race today, so this doesn't change a computed value, only stops an incorrect claim |
| 6 | Feat-effects engine: verified concretely (built a real fixture, added Toughness to `selected_feats`, ran the real `build_pilot_headless_receipt` entry point) that **no feat outside the 3 hardcoded into the deterministic posture gate (Power Attack, Dodge, Weapon Focus) has any mechanical effect anywhere** — confirmed by grep this isn't Toughness-specific, there is no general feat-effects computation in `pilot_compute.rs` at all | **Not a quick fix** — logged as its own architecture gap, linked to the existing AC/attack-bonus/skill-posture widening item (risks-and-open-questions.md item 1) rather than assigned to backend as routine work. Not attempted this swarm. |
| 7 | **MAJOR — CreateCharacterForm never actually submitted racial ability adjustments, for any of the 4 fixed-adjustment races, since the form was first built.** `calculatedScore()` (raw + racial adjustment) was computed for the on-screen preview only; the submitted `abilityScores` used raw, unadjusted `rawScore()` instead — every non-Human Elf/Dwarf/Gnome/Halfling character ever created had silently wrong ability scores, independent of and predating this session's engine-side explanation-text fixes (defect 4 above only corrected what the text *described*, not what got *submitted*) | **Fixed** (`f2c616ed`), live-verified end-to-end for Elf (disk-confirmed correct DEX/CON/INT cascade) and Dwarf (disk-confirmed `constitution:16/wisdom:14/charisma:6` on a fresh character created through the real UI). Gnome/Halfling verified via real production-code execution (actual function + actual race-catalog data, not a reimplementation) after a session-scoped GUI environment blocker (since fixed, `f6fe0df2`) prevented completing their live-disk leg — accepted as sufficient given the mechanism is unconditional/race-agnostic and already twice disk-proven |
| 8 | Fighter multiclass/race level-lookup gap, 3 instances: `validate_fighter_feat_choice_legality` and two sibling checks in `unmet_combat_posture_conditions` used single-class-only or Human-only level lookups instead of the multiclass-aware `fighter_level_in_mix`, silently skipping validation for non-Human and/or multiclass Fighters — one instance empirically confirmed exploitable (a Human Fighter1/Rogue3 with a wrong bonus-feat choice produced zero diagnostics before the fix) | **Fixed, systematic sweep complete** (`0eb9ea65`, `32289cb4` follow-up, `68721ca0`) — all 4 `_legality`/`_conditions`/`validate_` gate functions in `pilot_compute.rs` checked, no further instances. Currently no live UI attack surface (the create/level-up flow hardcodes canonical choices for the slots these checks protect) — real defense-in-depth for the command/API layer, not an active user-visible bug today |
| 9 | `skill_allocation.rs`'s class-skill recognition was Fighter-only, so neither Wizard nor Rogue had ANY grounded class-skill posture — silently left the PF1 cross-class rank cap completely unenforced for both (confirmed empirically: a level-1 Wizard could dump 5 ranks into a cross-class skill with a real cap of 1, zero diagnostic) | **Fixed** (`21f815c1`), grounded against the real PCGen corpus (Rogue: all 5 bounded skills, `cr_abilities_class.lst:2838`; Wizard: genuinely empty, `cr_abilities_class.lst:2565`, checked not assumed), **catalogue-adopted** (`d35521ec`, `2ab19bc7`) — real fixture-driven tests through the actual parser, both citations independently re-verified, and a fresh-eyes re-check found and closed a real gap in an *existing* test that used the bare string `"wizard"` instead of the real `"class:wizard"` id and so never actually exercised Wizard recognition |
| 10 | Wire-serialization bug: `CreateCharacterResponse::Saved`'s `corpus_derived` field serialized literally as snake_case on the wire (the enum's `kind` tag deliberately keeps no `rename_all`, so a bare fix would have broken every `outcome.kind === 'Saved'` check), silently `undefined` on the TS side — the Spells/Gear tabs looked stale right after a real, successful mutation | **Fixed** (`498679d1`, per-field `#[serde(rename = "corpusDerived")]`, an identical latent bug in `PurchaseEquipmentResponse` caught and fixed proactively in the same commit) — independently re-verified by QA with a real RED reproduction (temporarily reverted the fix, watched the exact symptom reappear, restored, confirmed GREEN) and an independent re-sweep of every `#[serde(tag = ...)]` enum in the crate, not just the ones already named |

**Bar-distance assessment (honest current picture):** the alpha bar is
**not** met yet, and the remaining distance is now well-characterized rather
than vague:
1. **Multiclass breadth** — 3 of 11 classes in the BAB/save-stacking
   allowlist. Bounded, repeatable, not attempted for the other 8 this
   session (no explicit greenlight to widen further).
2. **Class-chassis breadth** — 8 of 11 classes have zero base-chassis
   computation for *any* race, not a UI-reachability problem, a genuine
   missing-engine problem per class.
3. **Posture narrowness** — even for the 3 working classes, only one exact
   equipment/skill/feat combination ever reaches `Computed`. AC-gate,
   attack-bonus, and general-skill-posture widening were all scoped and then
   **dropped** this swarm after backend found the real blocker is
   architectural (the headless compute layer has no corpus parameter) —
   flagged to the operator as a possible future epic, not a wave-2 item.
4. **Feat effects** — confirmed nonexistent beyond the 3 feats hardcoded
   into the posture gate itself. Same shape of problem as item 3 (a
   structural gap, not a missing calculation), newly discovered this
   session.
5. **What *is* solid**: the 12 named calculations for the 3 working classes
   are genuinely correct and PCGen-cross-verified once you're inside the one
   supported posture — the defects found this session were about *breadth*
   (which classes/races reach a correct answer) and *honesty* (comments
   claiming something is out of scope when it's real), not about the core
   arithmetic being wrong once a build is actually `Computed`. Money
   purchasing is now a real atomic transaction (`purchase_equipment`), and
   the render-staleness/corpus-derived wire bugs found along the way are
   both closed.

### Second checkpoint (2026-07-24, post-race-bundle and post-submission-bug)

A second round of real defects landed after the checkpoint above was
written, closing out the remaining open threads from it rather than
introducing new scope:

- **The 4-race ability-modifier gap (defect 4) is now fully closed, not
  partial.** Dwarf/Gnome/Halfling's engine-side fix landed (`2f05dee4`) and
  was catalogue-adopted the same session (`fb01768d`), each citation
  independently re-verified against the real PCGen corpus rather than
  trusted from the commit message. All 4 races now correctly ground their
  real 3-stat racial adjustment.
- **A materially bigger, independent bug was found and fixed underneath
  it**: the create-character form was never actually *submitting* any
  race's adjusted ability scores — only displaying them — since the form
  was first built (defect 7). This predates and is unrelated to the
  engine-text fixes; it means every non-Human character created through
  the shipped UI, this entire swarm and before, had silently wrong ability
  scores baked into the saved file. Fixed (`f2c616ed`) and verified: Elf
  and Dwarf both disk-confirmed correct through a real create-character
  UI walkthrough; Gnome/Halfling confirmed via direct execution of the
  real production function against the real race-catalog data (a session-
  scoped GUI environment collision between concurrent agents, since fixed,
  prevented completing their disk leg — accepted as sufficient given the
  fix is unconditional across races and twice disk-proven already).
- **A third, unrelated defect class was found and closed by a systematic
  sweep**: Fighter's feat-choice-legality gate had the same "single-class
  or Human-only level lookup" blind spot in 3 separate places, one of them
  empirically confirmed exploitable before the fix (defect 8). All 4
  candidate gate functions in the file were checked; sweep is complete.
- **Housekeeping resolved, not newly found**: DR exposure through the DTO,
  the money-panel/equipment-purchase atomic-transaction gap, and
  `load_saved_character` exposing `spells_selected` (risks-and-open-
  questions.md items 6/9/9a) all landed as real backend work this session
  — closing three previously-logged "backlog, non-blocking" items outright
  rather than leaving them to accumulate.
- **Deferred, not fixed, and correctly so**: a non-Human Wizard's
  spell-specific grounding (spell-save-DC, spellbook-slot ceiling) never
  runs at all — the one function that both grounds it and enforces its
  level-3 ceiling is itself Human-gated. BAB/saves/HP for non-Human Wizards
  remain correct (a separate, already-widened path). Ruled a completeness
  gap (nothing computes *wrong*, a subsystem simply doesn't run), not a
  correctness bug — filed alongside the feat-effects and AC/attack-bonus
  architecture items rather than fixed or blocked this wave.

None of this changes the bar-distance shape below — multiclass/class-chassis
breadth and the posture-narrowness/feat-effects architecture gaps are
untouched — but it meaningfully strengthens confidence in the *correctness*
of what the 3 working classes already claim, and closes out several
previously-open threads cleanly rather than leaving them to drift.

### Third checkpoint (2026-07-24, post-skill-allocation-fix, sweep, and scoping synthesis)

A short round focused on closing out the last silent-correctness bug this
swarm's sweep pattern found, plus a synthesis pass over the architecture
gaps that remain:

- **`skill_allocation.rs`'s Fighter-only class-skill recognition (defect 9)
  is fixed and catalogue-adopted.** Same failure shape as the earlier
  class-skill-modifier bug (defect 3) — a silently wrong number with no
  claim-blocking diagnostic — but on rank enforcement rather than a
  modifier value: neither Wizard nor Rogue had any grounded class-skill
  posture in this module, so the PF1 cross-class rank cap never engaged
  for either. Both PCGen citations independently re-verified. Catalogue
  coverage went through a real fresh-eyes re-check (requested by the lead
  after a quota-outage stewardship landing) that found and closed a real
  gap in an *existing* test — it used the bare string `"wizard"` rather
  than the real `"class:wizard"` id, so it never actually exercised Wizard
  recognition in a multiclass union despite its name — and caught its own
  mid-draft assertion error (asserted the cross-class cap value where the
  real class-skill cap value applied) by running before trusting it.
- **A systematic sweep for the same failure shape elsewhere came back
  clean** (risks item 21): backend checked `pilot_compute.rs`,
  `skill_allocation.rs`, `durability.rs`, and `money.rs` for any remaining
  Fighter-only-grounded computation with a silent downstream consequence.
  One candidate (`explain_rogue_level1_chassis`'s single-class-only gate)
  was traced and ruled out — its output has no downstream consumer, so a
  missing record there is cosmetic, not a silent wrong number, the same
  shape already established for non-Human Rogue elsewhere. **This is the
  signal that closes out the "keep watching for silent bugs" thread**: the
  three working classes' shared computation paths are now confirmed clean,
  not just unexamined.
- **`docs/release/v0.6/future-epic-scoping.md`** consolidates the three
  remaining gaps (risks items 1/17/18) side by side for the operator's
  eventual review: the headless/corpus-aware architecture wall (attack-
  bonus enhancement math, skill armor-check-penalty), the feat-effects
  engine absence, and the non-Human Wizard spell-math gap. Confirms they
  are independent — fixing the architecture wall would unlock the first
  gap's two sub-problems but buys nothing for feat effects or Wizard's
  spell math, so none of the three blocks starting on either of the others.
  No new facts, a cross-item synthesis of what's already independently
  established in `risks-and-open-questions.md`.
- **Two DTO-exposure fixes (defects from risks items 6/9/9a) got real
  fixture-driven catalogue coverage**: the DR-exposure DTO field
  (`PilotSnapshot.damage_reduction`) now has a test driving a real
  Barbarian fixture through the real compute pipeline (stronger than the
  synthetic-receipt shape backend's own inline tests use, since Barbarian
  can't reach `Computed` today and a real end-to-end proof needs one
  synthesized field rather than a fully fabricated receipt). The other two
  (`purchase_equipment` atomicity, `spells_selected` exposure) are
  confirmed structurally unreachable from `tests/**` — entirely in the
  separate `codex-desktop` crate with no `rules_core` equivalent to
  complement — already correctly covered by backend's own inline
  Tauri-layer tests, same crate boundary established for Rogue's UI
  reachability earlier this swarm.

Net effect: the bar-distance picture is unchanged in shape from the second
checkpoint, but is now backed by an explicit clean-sweep result rather than
an implicit absence of further findings, and the three remaining
architecture gaps have a single reference document instead of being spread
across several risks-doc entries.

### Fourth checkpoint (2026-07-24, independent-verification sweep)

Distinct from the checkpoints above: those document what *landed*. This one
documents what's been *independently re-checked*, by whom, and how deep —
the coverage of the verification itself, which is what a future close-out
pass needs to know it can rely on rather than re-derive from scratch.

**Method.** Per the lead's ask (following the pattern already established
for the 4-race ability-adjustment work), QA scanned the full commit history
(`git log --oneline origin/develop..origin/tranche/6`, 160+ commits) for
every real `feat`/`fix`/`frontend` commit — as opposed to `docs`/status
commits — and worked down the list picking whichever unverified item looked
highest-value or highest-risk, giving each the same standard: read the
actual code/diff directly rather than re-asserting the commit message, run
the real tests personally rather than trusting a reported pass count, and
reproduce RED before trusting a claimed fix wherever that was cheap to do.

**17 areas independently verified clean this session, each with its own
concrete method (not just "looks fine"):**

| Area | Commit(s) | What was independently confirmed |
| :--- | :--- | :--- |
| Defense-tab DR wiring | `26ac0704` | TS type genuinely added to the shared `PilotSnapshotDto` (not ad-hoc); absent-case JSX renders `null`, never a fabricated zero; Barbarian's unreachability traced through `compute_class_chassis` → `table_class_id` → `is_supported_multiclass_mix` — no code path lets it reach `Computed`, not assumed from the claim |
| Durability status thresholds | `durability.rs` | All 6 `classify_durability` states independently re-derived against real PF1/d20 SRD rules from first principles, not copied from the module's own doc comment; matched backend's own parallel check exactly |
| Class-support labeling (all 11 CRB classes) | `34635157` | Every single row checked individually against `pilot_compute.rs` source (not spot-checked) — `supported_wizard_level`/`supported_rogue_level` genuinely never check `race_id`; all 8 `human-diagnostics-only` classes independently confirmed to share the identical gate pattern and all fall outside `table_class_id`'s 3-class allowlist |
| Wire-serialization fix | `498679d1` | Ran the 2 new tests personally; **reproduced RED myself** (temporarily reverted the `#[serde(rename)]`, watched the exact snake-case symptom reappear, restored, confirmed GREEN); independently re-swept the crate for other `#[serde(tag = ...)]` enums rather than trusting the "swept the rest" claim — found the same 4 backend already named, confirmed the 2 unaffected ones genuinely have no underscored fields |
| Feat catalog exposure | `89c3710a` | Per-category counts (50/110/8/17=185) re-derived by grepping the raw data files directly, not trusted from the doc comment; confirmed the Tauri wrapper does a true 1:1 map with no filtering; confirmed the "safe to append" claim in `unmet_combat_posture_conditions`'s own source (a presence check, not an exact-set match) |
| Level-up choice/skill persistence | `7694b227` | The "exactly one colon" grammar constraint traced to its real origin (`git log -S`, predates the swarm by 3 days — not invented to justify this fix); atomicity proven by reading the actual round-trip test, which reloads from disk and checks all three mutated fields landed together |
| Bio field persistence | `0ab784df` | The "already-saved" check confirmed to use a real `SavedCharacterStore::load`, not the naive `root.exists()` the commit says it deliberately avoided; overwrite-not-append proven via the actual two-save-then-reload test |
| `set_skill_allocations` | `e0a0bda4` | Wholesale-replace confirmed in source (`= skill_allocations`, no merge); the "reordered set proves replacement" test's premise re-derived (traced the seed fixture's real default order first, then confirmed the reversed submission round-trips exactly) |
| Money balance persistence | `67490acb` | Negative-balance rejection confirmed in source before any write; confirmed the DTO derivation reuses the exact `money::copper_to_denominations` function QA's own `tests/v06_money_conversion.rs` already covers, not a parallel reimplementation that could drift |
| `skill_allocation.rs` cross-class fix | `21f815c1` | Both PCGen citations re-verified against the local corpus checkout directly; **fresh-eyes re-check** (requested after a quota-outage stewardship landing) found and closed a real gap in an *existing* test that used the bare string `"wizard"` instead of the real `"class:wizard"` id |
| `combat.base_attack_bonus` dimension | `cda3bf1c` + `b8eff433` | All 4 mechanically-specified test files updated and run personally, including both real PCGen engine invocations (not just the fast synthetic ones), before backend's commit was allowed to land per the cross-surface protocol |
| LevelUpDialog wiring | `e8e45976` | Mechanics (hit-die choice, skill-allocation omission) confirmed clean; traced a comment's staleness by comparing commit timestamps directly, then confirmed the gap it describes is live-reachable today (Fighter's `levelOptions` includes a real bonus-feat level), not theoretical |
| SkillAllocationDialog wiring | `75200fcb` | `skillIdFor`'s "5 confirmed ids" claim checked against `skill_key_ability_modifier` directly; the "unrecognized ids are inert" claim confirmed via the actual `continue`-on-`None` branch, not assumed from the absence of an error |
| Bio editor wiring | `94a38657` | Character-switch reload's `cancelled`-guard against a stale in-flight load read directly in the `useEffect`, not just claimed by the commit |
| Feat picker + Feats tab | `febf4d80` + `aa611ce1` | Every one of the 6 `toCharacterMutationRefresh` call sites individually traced to confirm each threads the correct feat list (unchanged vs. plus-the-new-feat) for its specific mutation |
| Wizard spell-pick routing | `d55a919a` | The Wizard-preference-over-`heldClasses[0]` logic and the atomic-vs-plain routing both confirmed directly in the diff |
| Actions tab + dead-tab removal | `743c358b` | Confirmed `ActionsTab` is a pure display component with no new computation or backend call; version-bump fixture fix confirmed genuine (`0.6.0-test` now reads correctly) |

**Money panel (`59d5bc0a`) — clean on the static half, live-UI leg
inconclusive.** The `gpToCopper`/`gp_to_copper` formula match and the
boundary wiring were confirmed directly. The live-UI leg hit a real but
unrelated environment quirk (this session's window reports 1920×1200 via
`xdotool` but genuinely renders/screenshots at 1280×900, and the Load
Character dialog's action-button row wasn't reachable at any coordinate
tried) — not a code bug, correctly not force-fit into a false pass. Confirms
the `RUN_DESKTOP_AGENT` fix (`f6fe0df2`) holds under real concurrent use
though: this session's `:98` display and frontend's simultaneous `:96`
session never interfered with each other.

**2 real findings surfaced by this sweep** (both filed to the correct
owner, not fixed by QA — see `risks-and-open-questions.md` items 22/23/25
for full detail):
- Item 22 (now RESOLVED): `characterProgression.ts` — the module behind the
  classSummary comma-separator fix (`d03bc89d`) — had zero dedicated test
  coverage at all; frontend closed it same-day with a real
  `characterProgression.test.ts` (12 functions covered, one genuine RED
  caught along the way in a title-case regex assumption).
- Item 23 (now RESOLVED, landed minutes after this checkpoint was written):
  LevelUpDialog's own comment about why feat picks aren't collected at
  level-up was stale (the blocker it names was closed hours after that
  commit landed), and the underlying gap — real, currently-reachable at
  Fighter's level 2 — has since been fixed (`ddfc66bb`): a new
  `levelGrantsFeat` predicate detects a feat-granting level and routes
  through the same real feat picker the Feats tab uses, live-verified both
  branches (a Dwarf Fighter 1→2 picked Cleave; a Wizard 1→2 with no feat at
  that level leveled up uninterrupted).
- Item 25 (open, backlog, systemic): a recurring pattern across 4 frontend
  persistence-wiring modules (`characterProgression.ts` — since resolved —,
  `skillsModel.ts`/`setSkillAllocations`, the LevelUpDialog module, and
  `characterBio.ts`) of sound logic shipped with zero dedicated test file.
  Confirmed as a real, systemic gap rather than four coincidences; not
  re-flagged a fifth/sixth time once the pattern was established.

**What this checkpoint changes**: nothing about the bar-distance shape —
still not signing. What it adds is a documented, itemized answer to "how
much of what landed has actually been independently re-checked, and how,"
for whenever the operator or a future close-out pass wants to know the
verification depth wasn't assumed.

**Not signing the attestation.** Per §4.4's "Done" criteria, this requires
every shipped calculation having red-green coverage (true) *and* the
operator's alpha bar in §1 holding (not true — items 2 and 4 above are real,
acknowledged gaps, not stub surfaces, but still gaps against "any class...
matches PCGen"). This checkpoint is for visibility, not closure.

## Current-state summary (2026-07-24, full closure of the bounded backlog)

The four checkpoints above are an incremental log of *how* the picture got
here; this section is the *destination* — one coherent current-state read,
so anyone (operator, future close-out pass, a teammate picking this back up
cold) can get the whole picture without walking the history. Nothing below
contradicts the checkpoints above; it supersedes them only in the sense of
being the up-to-date summary, not a new finding.

**Where things stand.** As of this pass, all 26 numbered items in
`risks-and-open-questions.md`'s "Open questions" section are resolved or
correctly deferred. Two consecutive backend self-directed scans (the
Fighter-only-grounding correctness sweep, the parity-comparator field
sweep) each came back clean on their own second pass — a genuine signal,
not an absence of looking. QA's completeness sweep independently
re-verified essentially every real `feat`/`fix`/`frontend` commit the swarm
produced (17 areas verified clean, 2 real gaps found and since closed).
The bounded, same-session backlog — bugs, wiring gaps, missing tests,
missing UI surfaces for already-computed data — is genuinely exhausted
right now. That is a narrower claim than "the alpha bar is met," addressed
directly below.

**Full defects table, brought current.** The 10-row table above (under
"Comprehensive consolidation") is the complete, current list of every real
PCGen-delta/correctness defect found across the whole swarm — nothing has
been found since defect 10 (the wire-serialization bug) that isn't already
in it. All 10 are fixed and catalogue-adopted where `tests/**` coverage
applies, or explicitly logged as architecture-level and not attempted,
never left ambiguous.

**Beyond correctness defects, the other real work this swarm closed**
(UI-reachability and wiring gaps, not PCGen-delta correctness bugs, so
tracked in `risks-and-open-questions.md` rather than the defects table
above): the full Wizard three-layer UI-bootstrap chain (class acquisition,
first-spell, slot-budget enforcement); Rogue's UI reachability (zero gap);
race-agnostic reachability for all three working classes, disproving a
stale "Human only" assumption; the feat catalog + picker + persisted feat
list; bio, money, skill-allocation, level-up, and durability persistence
end to end; a new PCGen parity dimension (`combat.base_attack_bonus`); a
feat-pick affordance at feat-gaining level-ups; a Load-list staleness fix;
and a full pass closing 5 frontend modules' test-coverage gaps that QA's
sweep surfaced (items 22/25 in the risks doc).

**Bar-distance assessment, restated plainly against what's verified now:**
1. **Multiclass breadth** — still 3 of 11 classes (Fighter/Wizard/Rogue) in
   the BAB/save-stacking allowlist. Unchanged all session; bounded and
   repeatable (Rogue's own widening is the template) but not attempted
   further — no greenlight to widen beyond these three this wave.
2. **Class-chassis breadth** — still 8 of 11 classes with zero base-chassis
   computation for *any* race. Confirmed multiple times this session, not
   assumed. Each needs its own multi-cycle calculation engine — not a
   wiring fix, a genuine missing-engine problem per class.
3. **Posture narrowness** — even for the 3 working classes, the
   `Computed`/`Blocked` gate still only accepts one exact combination.
   AC/attack-bonus/skill-ACP widening was scoped, then correctly dropped
   after backend found the real blocker is the headless/corpus-aware
   architecture split — see `future-epic-scoping.md`, not reattempted this
   wave, flagged for the operator as a real future epic.
4. **Feat effects** — confirmed nonexistent beyond the 3 feats hardcoded
   into the posture gate (Power Attack, Dodge, Weapon Focus). Same
   architecture-gap shape as item 3, not a missing calculation.
5. **Non-Human Wizard spell-math completeness** — spell-save-DC and the
   spellbook-slot ceiling never run for any non-Human Wizard (the one
   function grounding both is Human-gated). BAB/saves/HP remain correct
   for non-Human Wizards via a separate, already-widened path — this is a
   completeness gap (a subsystem that doesn't run), not a correctness bug
   (a value that's wrong).
6. **What *is* solid**: the 12 named alpha-bar calculations are genuinely
   correct and PCGen-cross-verified once a build reaches `Computed`, for
   all three working classes — every defect found this session was about
   *breadth* (which classes/races reach a correct answer) and *honesty*
   (claims that overstated or understated real scope), not the core
   arithmetic being wrong once inside the one supported posture.

**What's genuinely left** (matching the lead's own bounded-backlog
assessment, risks doc item 3):
- **Architecture-level, not bounded work** (full detail in
  `future-epic-scoping.md`): the headless/corpus-aware wall (blocks
  attack-bonus and skill-ACP widening), the feat-effects engine's total
  absence, and Wizard non-Human spell-math completeness. None of these are
  a same-session task; each is confirmed independent of the other two —
  fixing one buys nothing toward the others.
- **Class/multiclass breadth**: the other 8 CRB classes, each its own
  multi-cycle engine effort.
- **Operator-only, not an engineering call**: starting-wealth-by-class
  (risks item 7) — a content-provenance/licensing question, exhaustively
  searched and confirmed absent from every real corpus source available
  here, not an open lookup.
- **Outside this swarm's control**: the observer-lane status (risks-doc
  Risks §5) — operator-side infrastructure.

**Not claiming the alpha bar is met.** It isn't — on class/race breadth (2
of 4 books' worth of classes genuinely reachable) and on the three
architecture gaps above. What this summary says is narrower and, we
believe, fully substantiated: the bounded backlog reachable without an
architecture decision or an operator content call is genuinely exhausted,
not abandoned early or padded with busywork to look active.

**Not signing the attestation** — same reasoning as every checkpoint above,
restated once more for anyone reading only this section: §4.4's "Done"
criteria needs both red-green coverage on every shipped calculation (true)
and the operator's alpha bar genuinely holding (not true, for the breadth
and architecture reasons above). This summary is for visibility and
closure-readiness, not a substitute for that sign-off.

## (b) PCGen-delta defects found and fix/ticket status

See the consolidated table above (Comprehensive consolidation section) for
the authoritative, current list — kept there rather than duplicated here to
avoid two sources of truth drifting apart. Historical note: this section sat
empty through wave-1 close and the original wave-2 resurvey, since no
calculation-changing defect had landed yet at either checkpoint; the first
real defects (Wizard spell-save-DC, slot-budget enforcement) landed after
the wave-2 resurvey was already written, which is why they don't appear in
the tables above this one.

## (c) Four-check wired-integration audit results

### Interim audit checkpoint (2026-07-24)

Run early at the lead's request, since the swarm's remaining distance from
the alpha bar now looks architecture-bounded rather than "more bugs to
find" — a good point to surface any wired-integration violation while
backend/frontend can still fix it live, rather than as a surprise at actual
closure. Per the operator's ceremony waiver (this doc, top), the receipt
ceremony is waived but the audit itself is not — this is an *interim*
checkpoint, not the final one; the closure-time audit (§7.1 of
`release-swarm.md`) still runs separately against the final combined diff
before the closure PR opens.

**Method**: extracted every added line (`+` lines only, diff metadata
excluded) from `git diff origin/develop...origin/tranche/6` (116 files,
10,576 insertions across the full swarm to date), tagged by source file,
and ran all four greps by hand against that extraction.

**Result: clean. Zero real violations found.**

1. **Forbidden tokens** (`\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b`, case-insensitive, outside tests/docs): 2 files matched, 5 lines total, all false positives on inspection — `apps/desktop/src-tauri/src/pf1_adapter.rs` has two doc-comment lines explicitly describing a placeholder that was *removed* ("not a synthetic placeholder", "no reason to keep seeding a placeholder now that the real path [exists]"); `apps/desktop/src/characterHub/CharacterSheet.tsx` has a real `placeholder="gp amount"` HTML input attribute (the doctrine's target is stubbed logic, not input hint text), a comment explaining why a duplicate "coming soon" tab was deliberately *removed*, and a comment explicitly documenting a case where fabricating feat options *would* violate the doctrine — i.e. recording that they correctly did **not** stub it, not that they did.
2. **No-op handlers** (`onClick={()=>{}}` / `onClick={undefined`): 0 real hits. The only match was this document's own checklist line describing the check.
3. **Mock-library leaks outside tests** (`mockResolvedValue|mockReturnValue(|vi.mock(|__mocks__`): 0 real hits, same self-reference-only result.
4. **`"Would ..."` strings**: 0 real hits outside this doc's and `risks-and-open-questions.md`'s own descriptions of the check; broadened the search for near-miss phrasing (`Would compute/return/apply/resolve/handle/implement/support/do/be/have`, unquoted) to sanity-check the exact-match regex wasn't too narrow — still zero hits.

Raw grep commands (reproducible): each check run against a Python-extracted
`file\t<added-line>` table built from the diff, e.g. for check 1:
`grep -iE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' <extracted> | grep -vE '^(tests/|docs/|.*\.test\.(ts|tsx)|.*_test\.rs)'`.

**Re-verified independently (lead, 2026-07-24)**, per the doctrine's
"executed by QA, re-verified by the lead" requirement: extracted added
lines fresh from `git diff origin/develop...origin/tranche/6` myself
(without reading QA's extraction) and ran all four greps. Same 5 lines on
check 1, same zero-real-hits result on checks 2-4. Confirms QA's audit —
clean, no wired-integration violations in the swarm's diff to date.

## (d) Alpha-bar items 1-3 and 7 confirmation

**Superseded by the Comprehensive consolidation section above** — this
section's original wave-1-era text is left below for provenance only, since
it's now materially stale (items 2/3 in particular have real, live-verified
progress the text below doesn't reflect).

- **Item 1** (installer without intervention, past SmartScreen): Not
  re-verified this session — no installer-affecting change landed in this
  swarm's scope. CI already builds unsigned MSI/NSIS per
  `publish-tester-release.yml`; still expected to hold, not re-confirmed.
- **Item 2** (create a character from any of CRB/B1/APG/ACG, load from disk):
  Materially advanced — Fighter/Wizard/Rogue all confirmed creatable/
  loadable for any race (not just Human), live-verified for Elf
  specifically. The other 8 classes remain fully blocked (zero chassis
  computation), not a stub-surface problem — a missing-engine one.
- **Item 3** (advance 6 levels, multiclass, spells/feats/equipment/bio/money):
  Materially advanced — Wizard's full bootstrap chain (class acquisition,
  first spell, slot-budget enforcement) and Rogue's UI reachability are both
  closed and live-verified. Money purchasing is now a real atomic
  transaction. Feat *selection* works (recorded, persisted); feat *effect*
  does not (see defect 6 in the consolidation table) — a real, newly-found
  gap against this item's spirit, even though nothing about feat selection
  itself is stubbed.
- **Item 7** (PR lands green on CI, four-check audit re-run, SWARM_REPORT.md
  recorded): Pending — this document is that artifact, still in draft; the
  four-check audit itself has not been run yet (see §c above), correctly
  held until the closure PR is genuinely being opened.

---

## Appendix: formula spec for durability / carry capacity / encumbrance / money conversion (for backend wave 2)

QA prep work for the four calculations flagged above as having zero production
implementation. Sourced from the real PCGen engine checkout at
`/home/ubuntu/workspace/repos/pcgen` (the same repo the swarm's PCGen parity
tooling already shells out to — `scripts/pcgen-run-character.sh`), not from
memory, wherever an authoritative source file exists. Confidence level is
called out per item; anything not directly sourced from a PCGen file should be
treated as "needs verification against a real PCGen run" before being
hardcoded into a parity test.

### Durability (character survivability)

Per the lead's ruling, scope is: max HP, current HP, temporary HP, nonlethal
damage tracking, dying/unconscious/death thresholds. Standard PF1 rules
(open game content, not PCGen-sourced — high confidence, but not yet
cross-checked against a PCGen run):

- **Max HP** = sum, per class level in level order, of that level's Hit Die
  contribution + Constitution modifier, with a floor of **1 HP per level**
  regardless of Con penalty. Level 1 uses the **maximum** value of the class's
  Hit Die (already implemented for Fighter: `FIGHTER_LEVEL_1_MAX_HIT_DIE_HIT_POINTS
  + constitution_modifier` in `pilot_compute.rs:7418`) — every level after
  that uses either a rolled or (more commonly, and what PCGen/most digital
  tools default to) an **average/fixed** value per the class's Hit Die
  (already computed per-level and tested in `sd13_*_level*_progression.rs` /
  `sd20_levelup_*.rs` — those tests cover the per-level *increment*; there is
  no test or field for the *running total*). In a multiclass build, each
  class level contributes using its own class's Hit Die.
  - Favored Class Bonus: a level where the player chose +1 HP (instead of a
    skill point) adds 1 more HP at that level — check whether
    `sd13_fighter_favored_class_bonus_choice.rs` threads this into an HP
    total anywhere, since today it looks like it's tracked but not summed.
- **Current HP**: starts equal to max HP; decremented by damage taken during
  play. This is a live-tracking field, not a build-time derived calculation —
  needs a data field with `default = max_hp`, not a "formula."
- **Temporary HP**: granted by specific spells/effects (e.g. *false life*),
  not derived from chassis math. Likely out of v0.6 scope unless a specific
  spell/item that grants it is already selectable; flag to backend to confirm
  scope before building a general temp-HP resource system.
- **Nonlethal damage**: tracked as a separate running total against current
  HP, not a subtraction from it.
- **Thresholds** (standard PF1/d20 SRD rule, high confidence):
  - `current_hp == 0` → **disabled** (can take a single move or standard
    action per round; a standard action causes 1 more point of nonlethal
    damage and leaves the character at 0, not negative).
  - `current_hp < 0` and `current_hp > -constitution_score` → **dying**
    (unconscious, loses 1 HP/round unless stabilized).
  - `current_hp <= -constitution_score` → **dead**.
  - `nonlethal_damage == current_hp` (current HP still `> 0`) → **staggered**.
  - `nonlethal_damage > current_hp` → **unconscious** (stable, not dying,
    since the excess is nonlethal).

  **Correction (QA, 2026-07-24):** this appendix originally wrote the
  staggered threshold as `>=`, which overlaps with unconscious below —
  imprecise pre-implementation spec text, not what shipped. The actual
  `durability.rs::classify_durability` (and this session's independent
  re-derivation of all 6 states against real PF1/d20 SRD rules) uses exact
  equality for staggered; corrected above rather than left to mislead a
  future reader.

### Carry capacity / encumbrance

**Sourced directly from PCGen's own Pathfinder game-mode data file** —
`/home/ubuntu/workspace/repos/pcgen/system/gameModes/Pathfinder/load.lst`.
This is the exact table PCGen itself uses, so a parity test built from these
numbers should match PCGen output by construction (still worth a spot-check
run). Engine logic (extrapolation beyond the table) lives in
`pcgen/core/system/LoadInfo.java` in that same checkout.

**Trap for anyone hand-writing fixture corpus text** (found while writing
`tests/v06_encumbrance.rs`): this crate's real corpus `KEY:` tokens only
carry a `(Base)` suffix for items with real magical/enhancement variants
(armor, shields, weapons — e.g. `Chain Shirt (Base)`, `Longsword (Base)`).
Plain General-category items do not (e.g. `Backpack`, not
`Backpack (Base)`). A fixture item whose `KEY:` guesses wrong on this
silently resolves to `unresolved_item_ids` rather than erroring — the
equipment resolver is strict/exact-match, not fuzzy, so a wrong suffix
looks like "this item weighs 0" rather than a loud failure. Check the real
entry in `src/rules_core/rules_tables/crb/equipment_data/*.rs` before
hand-transcribing a `KEY:` token into fixture text.

- **Base table** (`LOAD:<Strength>|<max load in lbs, at 1x "Heavy" multiplier>`),
  Strength 0-29:
  `0|0, 1|10, 2|20, 3|30, 4|40, 5|50, 6|60, 7|70, 8|80, 9|90, 10|100, 11|115,
  12|130, 13|150, 14|175, 15|200, 16|230, 17|260, 18|300, 19|350, 20|400,
  21|460, 22|520, 23|600, 24|700, 25|800, 26|920, 27|1040, 28|1200, 29|1400`.
- **Beyond Strength 29**: multiply the value at `(score - 10)` by `LOADMULT:4`
  — i.e. every +10 Strength beyond the table quadruples the Str-29 baseline
  chain (`LoadInfo.getLoadScoreValue`, the `loadScoreMultiplier` /
  `loadMultStep=10` fields).
- **Encumbrance tiers**, each expressed as a multiplier of the base table
  value plus a skill-check-penalty-style modifier
  (`ENCUMBRANCE:<name>|<multiplier>||<penalty>`):
  - Light: `1/3` of table value, penalty `0`.
  - Medium: `2/3` of table value, penalty `-3`.
  - Heavy: `1x` of table value (this is literally the table value itself —
    "heavy load" *is* the tabulated max), penalty `-6`.
  - OverHead (max lift over head): `1x`, penalty `-6`.
  - OffGround (max lift/budge off the ground): `2x`, penalty `-6`.
  - PushDrag (max push or drag): `5x`, penalty `-6`.
- **Size adjustment** (`SIZEMULT:<size code>|<multiplier>`, relative to
  Medium = 1x): Fine `0.125`, Diminutive `0.25`, Tiny `0.5`, Small `0.75`,
  Large `2`, Huge `4`, Gargantuan `8`, Colossal `16`. Effective Strength for
  the load table lookup is the character's actual Strength score — the size
  multiplier is applied to the resulting load value, not to the Strength
  score used for table lookup.

### Money conversion

- **Denomination ratios** (standard d20/PF1 currency, open content — **not**
  independently confirmed against a PCGen source file in this pass; I found
  no explicit conversion-table data file in the PCGen checkout, which is
  consistent with these being simple linear arithmetic rather than tabulated
  data, but flagging as not-yet-source-verified): 1 platinum piece (pp) = 10
  gold pieces (gp) = 100 silver pieces (sp) = 1000 copper pieces (cp).
  Equipment `cost_gp` fields already price everything in gp; conversion is
  just `value_in_gp * {pp: 0.1, gp: 1, sp: 10, cp: 100}` and back.
- **Starting wealth by class**: searched `data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst`
  for a `GOLD:` token (PCGen's per-class starting-gold-roll field, e.g.
  `GOLD:5d6`) and found none in that file. **Unresolved at the time** —
  either starting wealth lives in a different PCGen data file not yet
  checked, or PCGen leaves it as a manual/optional step; don't guess a value
  here. **Follow-up completed (backend, risks-and-open-questions.md item
  7):** the deeper search this note asked for was done — the whole data
  tree, PCGen's gameMode-level `miscinfo.lst`, and the wider corpus — and
  found nothing real anywhere, plus caught a real trap (a stub-labeled
  `starting_gold` column in an unrelated closure artifact that looks
  citable but explicitly isn't licensed data). This genuinely doesn't exist
  in any real, licensed corpus source available in this environment — a
  content-provenance/licensing question for the operator now, not an open
  engineering lookup.

---

## QA attestation

**Not yet signed.** This section is filled in only when the alpha bar in §1
of `release-swarm.md` genuinely holds, per §4.4's "Done" criteria. Until
then, this document is a living gap-tracker, not a sign-off.
