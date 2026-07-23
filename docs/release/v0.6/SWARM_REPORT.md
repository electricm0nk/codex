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
| Durability | **Gap — needs definition** | No test or production code found under `durability`/`hardness`. Alpha-bar item 4 doesn't define whether this means character HP (covered, see below) or item/object hardness+breakage (not found anywhere in `src/rules_core`). Flagging for lead/operator clarification. |
| Carry capacity | **Gap — no production code** | No `carry_capacity`/`carrying_capacity` computation found anywhere in `src/rules_core`. Not a test gap — the calculation itself doesn't exist. |
| Encumbrance | **Gap — no production code** | Same as carry capacity — no encumbrance/load computation found in `src/rules_core`. |
| Money conversion | **Gap — no production code** | No currency-conversion, starting-gold, or wealth-by-level logic found in `src/rules_core`. Only per-item `cost_gp` pricing exists on equipment records. Corroborates frontend's independent finding (see `SWARM_STATUS.md` "Happened" log) that money/currency has no schema field anywhere in the engine. |
| Level-up hit points | Covered | `tests/sd13_fighter_level1_hit_point_baseline.rs`, `tests/sd20_levelup_*.rs` per class |
| Multiclass stacking (general) | Partial | Base chassis (BAB/save) stacking covered for Fighter/Wizard (see above). Skill points / feats / spell-slot stacking under multiclass not independently verified in this survey — needs a follow-up pass once BAB/save gap is closed. |

**Bottom line:** 3 of the 12 alpha-bar calculations (carry capacity, encumbrance,
money conversion) have **zero production implementation**, not just zero tests —
these are backend build items, not QA test-authoring items. Multiclass BAB/save
stacking has real tests but only for one class pair; widening that is a QA task
(no backend blocker) once backend's wave-1 unblocking queue clears.

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

## QA attestation

**Not yet signed.** This section is filled in only when the alpha bar in §1
of `release-swarm.md` genuinely holds, per §4.4's "Done" criteria. Until
then, this document is a living gap-tracker, not a sign-off.
