# SD-24 Epic 4 — Criterion 4.5: APG/ACG-Class Multiclass Scope Decision

## Decision

**APG/ACG-class multiclass is deferred out of SD-24 Epic 5.** Epic 5 ("Multiclass Stacking Real and Full") ships scoped to **Fighter + Wizard only**, level 1 → 10 advancement, per operator directive 2026-07-21 (`decisions.md §4`). No APG or ACG class participates in SD-24's multiclass dispatch.

This decision is not new — it is the bundle's pre-existing, operator-pinned scope (`epic-breakdown.md`'s Epic 5 purpose statement, `loop-instruction.md §4.2`'s hard-stop row, and `risks-and-open-questions.md §2`'s non-self-healable-conditions table all state it in advance of any audit running). Criterion 4.5's role is to **formally close the loop**: confirm, using this bundle's own Epic 4 audit evidence (criteria 4.1–4.3), that the deferral's stated precondition — "APG/ACG classes are not fully wired" — is in fact true, and record that confirmation as this bundle's canonical decision artifact.

## Evidence (from criteria 4.2 and 4.3)

Per `./per-class-coverage-matrix.md`'s APG and ACG sections, for all 16 real APG/ACG classes (Alchemist, Cavalier, Inquisitor, Oracle, Summoner, Witch, Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest):

1. **Chassis (BAB/saves) is fully wired and independently re-verified correct** — 20/20 levels for every class, cross-checked against the real PCGen corpus's `BONUS:COMBAT|BASEAB`/`BONUS:SAVE` tokens.
2. **Zero named class features are wired for any of the 16 classes** — corpus-derived expected counts (24/16/19/19/17/7 for the six APG classes; 9/19/14/21/95/10/20/15/29/18 for the ten ACG classes) versus 0 wired, in every case.
3. **`pilot_compute.rs`'s live `compute_class_chassis` dispatch recognizes none of the 16 classes** — proven empirically (not just by inspection): driving a real `CharacterInput` for each class through `compute_pilot_base_chassis` returns the honest, claim-blocking `class_chassis.unsupported` diagnostic (`base_attack_bonus: 0`) rather than fabricated data. This matches the pre-existing behavior for every non-Fighter/Wizard **CRB** class too — the live pilot-compute seam is Fighter/Wizard-only across every book today, not an APG/ACG-specific gap.
4. **No `level_up::<class>` module exists for any of the 16 classes.** `src/rules_core/level_up/` (the SD-20 Epic 7 per-level automatic-feature-grant model) contains only the 11 CRB classes.

This is exactly the "chassis-only, zero named features, zero live-compute integration, zero level-up modules" condition `loop-instruction.md §4.2`'s hard-stop row anticipates and pre-resolves. **No `## Open blockers` entry is raised** — the audit confirmed the expected, already-decided outcome rather than surfacing a new blocker.

## Scope consequence for Epic 5

Epic 5's Fighter+Wizard-only multiclass dispatch (criterion 5.1) is **not blocked** by this deferral: its dependency is Epic 4's coverage-matrix output for Fighter and Wizard specifically (criterion 4.1), which is complete and gap-free as of the `fighter-wizard-audit-cycle` (commit `66f9be8`). The APG/ACG deferral recorded here applies only to the 16 classes outside Epic 5's scope; it does not gate Epic 5's own dispatch.

Criterion 5.5 ("APG/ACG multiclass deferred") is the Epic-5-side echo of this same decision (per `acceptance-and-verification.md` row for 5.5, artifact `./artifacts/epic_5/apg-acg-multiclass-deferred.md`) and should reference this document as its source decision rather than re-deriving it.

## Follow-on delivery vehicle

Per `risks-and-open-questions.md §4 Q1` ("APG/ACG-class multiclass delivery vehicle: SD-24 Epic 5 deferred; is the follow-on bundle a SD-25 immediately after closure, or an operator-pinned later bundle?"), the default is **SD-25, immediately following SD-24 closure**. The operator may pin a different bundle. `./remediation-plan.md §5` records that none of the 16 classes' named-feature gaps have an SD-24 cycle-id assigned; the follow-on bundle is where those cycle-ids get minted.

## Override

`risks-and-open-questions.md §3` Flag `FLAG-B: APG-MULTICLASS-DEFER` remains **unset** (default), meaning the cycle picker continues to refuse dispatching any APG/ACG-class multiclass criterion. Setting `FLAG-B` is the operator's mechanism to reverse this deferral within SD-24 itself, should the operator choose to do so before bundle closure.

## Cross-references

- `./remediation-plan.md` — criterion 4.4's remediation plan (this decision's sibling artifact)
- `./per-class-coverage-matrix.md` — the coverage data this decision is derived from
- `../../risks-and-open-questions.md §5 Deferrals` — bundle-level deferral ledger entry (updated by this cycle)
- `../../risks-and-open-questions.md §2` — non-self-healable-conditions table row pre-anticipating this decision
- `../../risks-and-open-questions.md §3` — `FLAG-B` override flag
- `../../decisions.md §4` — Epic 5's Fighter+Wizard-only multiclass scope ADR
