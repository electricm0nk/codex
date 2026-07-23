# v0.6 Alpha Release Swarm — Report

Status: DRAFT (living document, updated as waves complete). Not an attestation yet.

Owner of this document: lead (orchestrator) collates; QA owns the attestation
content per §4.4 and §7.1 of `docs/release/v0.6/release-swarm.md`.

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

## (b) PCGen-delta defects found and fix/ticket status

None found yet — wave-1 in flight, no landed calculation changes to diff
against PCGen yet. This section updates as backend/frontend land work.

## (c) Four-check wired-integration audit results

**Stub — not yet run.** Per `docs/release/v0.6/risks-and-open-questions.md`,
the receipt ceremony is waived but the audit itself is not. This section will
be filled with the raw grep output (per the four checks in the QA teammate
brief) against `git diff origin/develop...HEAD` immediately before the
closure PR opens — not before, since the diff is empty/trivial until wave
work lands.

Checks to run at closure:
1. Forbidden tokens: `\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b` (case-insensitive), expect no hits outside tests/docs.
2. No-op handlers: `onClick={()=>{}}` / `onClick={undefined`, expect no hits.
3. Mock-library leaks outside tests: `mockResolvedValue|mockReturnValue(|vi.mock(|__mocks__`, expect no hits outside test files.
4. `"Would ..."` strings: `"Would [^"]*"`, expect no hits.

## (d) Alpha-bar items 1-3 and 7 confirmation

- **Item 1** (installer without intervention, past SmartScreen): Not yet
  re-verified this run — CI already builds unsigned MSI/NSIS per
  `publish-tester-release.yml`; no regression expected from wave-1 scope, but
  will be confirmed post-version-bump.
- **Item 2** (create a character from any of CRB/B1/APG/ACG, load from disk):
  Not yet re-verified; depends on frontend's stub-tab burn-down landing.
- **Item 3** (advance 6 levels, multiclass, spells/feats/equipment/bio/money):
  Blocked on backend's wave-1 unblocking queue (skill/level-up/bio/feat/money
  persistence commands) and frontend's consuming UI — in flight.
- **Item 7** (PR lands green on CI, four-check audit re-run, SWARM_REPORT.md
  recorded): Pending — this document is that artifact, currently in draft.

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
  - `nonlethal_damage >= current_hp` (current HP still `> 0`) → **staggered**.
  - `nonlethal_damage > current_hp` → **unconscious** (stable, not dying,
    since the excess is nonlethal).

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
  `GOLD:5d6`) and found none in that file. **Unresolved** — either starting
  wealth lives in a different PCGen data file I didn't check, or PCGen leaves
  it as a manual/optional step. Needs a follow-up lookup before backend
  builds a starting-gold formula; don't guess a value here. If v0.6 scope is
  just "track and spend money the player already has" rather than "roll
  starting gold automatically," this may not even be needed for alpha.

---

## QA attestation

**Not yet signed.** This section is filled in only when the alpha bar in §1
of `release-swarm.md` genuinely holds, per §4.4's "Done" criteria. Until
then, this document is a living gap-tracker, not a sign-off.
