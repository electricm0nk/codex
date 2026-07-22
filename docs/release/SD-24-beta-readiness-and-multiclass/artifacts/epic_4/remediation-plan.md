# SD-24 Epic 4 — Remediation Plan (Criterion 4.4)

> Consumes the coverage findings from criteria 4.1 (CRB, Fighter+Wizard scope), 4.2 (APG, all 6 classes), and 4.3 (ACG, all 10 classes) recorded in `./per-class-coverage-matrix.md`. Per the criterion's verbatim acceptance text (`epic-breakdown.md`): "For each class where `class_features_wired < class_features_expected`, list the missing features with the cycle-id that fixes them."

## 1. CRB — Fighter + Wizard (audited scope, criterion 4.1)

| Class | `class_features_wired` / `expected` | Missing features | Fix cycle-id | Priority | Status |
|---|---|---|---|---|---|
| Fighter | 10 / 10 | none | — | — | No remediation needed |
| Wizard | 12 / 12 (after fix) | `class_spell.wizard.*` family (spellbook contents, daily preparation, base/Intelligence-bonus/total spells-per-day per spell level 0-3) — was 7/12 before this cycle | `fighter-wizard-audit-cycle` (commit `66f9be8`) | P1 | **Remediated in-cycle** — no carry-forward |

No open remediation items for the audited CRB scope (Fighter + Wizard). Both classes are fully wired as of `fighter-wizard-audit-cycle`.

## 2. CRB — unaudited classes (out of this bundle's granted file-touch scope)

Cleric, Rogue, Sorcerer, Barbarian, Bard, Druid, Monk, Paladin, Ranger each already have a landed `level_up/<class>.rs` module (SD-20 Epic 7), but auditing them against the same `class_spell.*`/`class_feature.*`-prefix-coverage methodology criterion 4.1 used for Fighter/Wizard was outside this bundle's granted per-cycle file-touch partition (`loop-instruction.md §2.4` names only `class_fighter.rs`/`class_wizard.rs` for Epic 4).

| Class | `class_features_wired` / `expected` | Missing features | Fix cycle-id | Priority | Status |
|---|---|---|---|---|---|
| Cleric, Rogue, Sorcerer, Barbarian, Bard, Druid, Monk, Paladin, Ranger (9 classes) | unknown (not audited) | unknown — the Wizard-class bug (`level_up/<class>.rs`'s explanation-filter predating a later `pilot_compute.rs` grounding and never widening to catch it) is a plausible recurring shape per any class with a multi-source baseline grounding (e.g. Sorcerer/Cleric/Bard's own spell groundings) | unassigned — no cycle-id minted in SD-24 | LOW (not gating Epic 5, which is Fighter+Wizard-only) | Deferred — forwarded as `## DISCOVERED` item `epic-4-follow-on` (2026-07-21T00:15:00Z) in `progress.md` |

**Disposition:** does not block Epic 5 (Fighter+Wizard-only multiclass scope per operator directive 2026-07-21). Recommended as a follow-on Epic 4 cycle, one `level_up/<class>.rs` file at a time per the file-touch partition, only if the operator later requires full-11-CRB-class breadth.

## 3. APG — all 6 classes (audited scope, criterion 4.2)

Chassis (BAB/saves) is fully wired and independently verified correct for all 6 classes (20/20 levels each) — no remediation needed on that pillar. Named class features are 0-wired for every class:

| Class | `class_features_wired` / `expected` | Missing features | Fix cycle-id | Priority | Status |
|---|---|---|---|---|---|
| Alchemist | 0 / 24 | Bombs, Discoveries, Mutagen, and all remaining `KEY:Alchemist ~ ...` corpus records | unassigned | P1 | Deferred (see §5, criterion 4.5) |
| Cavalier | 0 / 16 | Order, Challenge, Banner, and all remaining `KEY:Cavalier ~ ...` corpus records | unassigned | P1 | Deferred (see §5, criterion 4.5) |
| Inquisitor | 0 / 19 | Judgment, Monster Lore, Bane, and all remaining `KEY:Inquisitor ~ ...` corpus records | unassigned | P1 | Deferred (see §5, criterion 4.5) |
| Oracle | 0 / 19 | Mystery, Revelations, Curse, and all remaining `KEY:Oracle ~ ...` corpus records | unassigned | P1 | Deferred (see §5, criterion 4.5) |
| Summoner | 0 / 17 | Eidolon, Summon Monster, Shield Ally, and all remaining `KEY:Summoner ~ ...` corpus records | unassigned | P1 | Deferred (see §5, criterion 4.5) |
| Witch | 0 / 7 | Hex, Patron, and all remaining `KEY:Witch ~ ...` corpus records | unassigned | P1 | Deferred (see §5, criterion 4.5) |

Additionally, for every APG class: no `level_up::<class>` module exists, and `pilot_compute.rs`'s `compute_class_chassis` dispatch does not recognize the class (proven empirically — the honest `class_chassis.unsupported` diagnostic fires rather than fabricated data). Both are remediation prerequisites bundled with the named-feature work above, not separate line items.

## 4. ACG — all 10 classes (audited scope, criterion 4.3)

Chassis (BAB/saves) is fully wired and independently verified correct for all 10 classes (20/20 levels each) — no remediation needed on that pillar. Named class features are 0-wired for every class:

| Class | `class_features_wired` / `expected` | Missing features | Fix cycle-id | Priority | Status |
|---|---|---|---|---|---|
| Arcanist | 0 / 9 | Arcane Reservoir, Arcane Exploit, and all remaining `KEY:Arcanist ~ ...` corpus records | unassigned | P1 | Deferred (see §5, criterion 4.5) |
| Bloodrager | 0 / 19 | Bloodline, Bloodrage, and all remaining `KEY:Bloodrager ~ ...` corpus records | unassigned | P1 | Deferred (see §5, criterion 4.5) |
| Brawler | 0 / 14 | Martial Flexibility, Brawler's Cunning, and all remaining `KEY:Brawler ~ ...` corpus records | unassigned | P1 | Deferred (see §5, criterion 4.5) |
| Hunter | 0 / 21 | Hunter's Trick, Animal Focus, and all remaining `KEY:Hunter ~ ...` corpus records | unassigned | P1 | Deferred (see §5, criterion 4.5) |
| Investigator | 0 / 95 | Studied Combat, Studied Strike, Inspiration, and all remaining `KEY:Investigator ~ ...` corpus records (largest APG/ACG feature surface of any audited class) | unassigned | P1 | Deferred (see §5, criterion 4.5) |
| Shaman | 0 / 10 | Spirit, Spirit Animal, Wandering Hex, and all remaining `KEY:Shaman ~ ...` corpus records | unassigned | P1 | Deferred (see §5, criterion 4.5) |
| Skald | 0 / 20 | Raging Song, and all remaining `KEY:Skald ~ ...` corpus records | unassigned | P1 | Deferred (see §5, criterion 4.5) |
| Slayer | 0 / 15 | Sneak Attack, Slayer Talents, and all remaining `KEY:Slayer ~ ...` corpus records | unassigned | P1 | Deferred (see §5, criterion 4.5) |
| Swashbuckler | 0 / 29 | Panache, Deeds, and all remaining `KEY:Swashbuckler ~ ...` corpus records (2nd-largest feature surface) | unassigned | P1 | Deferred (see §5, criterion 4.5) |
| Warpriest | 0 / 18 | Blessings, Fervor, and all remaining `KEY:Warpriest ~ ...` corpus records | unassigned | P1 | Deferred (see §5, criterion 4.5) |

Additionally, for every ACG class: no `level_up::<class>` module exists, and `pilot_compute.rs`'s `compute_class_chassis` dispatch does not recognize the class (proven empirically, identical shape to APG's finding). Both are remediation prerequisites bundled with the named-feature work above, not separate line items.

## 5. Remediation disposition (feeds criterion 4.5)

None of the 16 APG/ACG named-class-feature gaps above have an assigned fix cycle-id in SD-24. Per the operator's own hard-stop anticipation (`loop-instruction.md §4.2`: "Epic 4 (Per-class audit) finds APG/ACG classes are *not* fully wired → Multiclass Epic 5 scope is restricted to Fighter + Wizard only … defer APG/ACG-class multiclass to a follow-on bundle; document in `risks-and-open-questions.md`") and per operator directive 2026-07-21, this remediation plan does **not** assign SD-24 cycle-ids to the APG/ACG named-feature gaps. Criterion 4.5 (`./apg-acg-multiclass-deferred.md`) formally records the resulting multiclass-scope deferral. The delivery vehicle for these 16 classes' remediation is an operator-decided follow-on bundle (default: SD-25 immediately following SD-24 closure, per `risks-and-open-questions.md` Q1; operator may pin a different bundle).

## 6. Summary table

| Book | Classes audited | Classes fully wired (chassis + features) | Classes with open named-feature gaps | Remediated this bundle | Deferred to follow-on |
|---|---|---|---|---|---|
| CRB | 2 (Fighter, Wizard — this bundle's granted scope) | 2 / 2 (after Wizard fix) | 0 | 1 (Wizard) | 0 |
| CRB | 9 (unaudited: Cleric, Rogue, Sorcerer, Barbarian, Bard, Druid, Monk, Paladin, Ranger) | unknown | unknown | 0 | 9 (audit itself, LOW priority, non-gating) |
| APG | 6 (Alchemist, Cavalier, Inquisitor, Oracle, Summoner, Witch) | 0 / 6 (chassis-only) | 6 | 0 | 6 |
| ACG | 10 (Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest) | 0 / 10 (chassis-only) | 10 | 0 | 10 |

**Epic 5 dependency status:** satisfied. Epic 5's multiclass work is scoped to Fighter + Wizard only (per `decisions.md §4`), and both classes are fully wired per §1 above. Epic 5 may proceed without waiting on the APG/ACG remediation deferred here.

## 7. Cross-references

- `./per-class-coverage-matrix.md` — the coverage data this plan is derived from
- `./apg-acg-multiclass-deferred.md` — criterion 4.5's formal deferral decision record
- `../../risks-and-open-questions.md §5 Deferrals` — bundle-level deferral ledger entry
- `../../risks-and-open-questions.md §4 Q1` — follow-on bundle delivery-vehicle open question
- `../../progress.md ## DISCOVERED` — `epic-4-follow-on` entries this plan consolidates
