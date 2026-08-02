---
title: GE-04 Expression-Language Runtime Requirements
stc_id: STC-CODEX-GE-04
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-04-rules-engine-and-explainability-core/artifacts
source_stc: ../README.md
source_inputs:
  - ../../GE-02-canonical-rules-model-and-content-packages/artifacts/expression-language-decision-criteria.md
---

# GE-04 Expression-Language Runtime Requirements

## Purpose
Extend GE-02 expression-language decision criteria into GE-04 runtime requirements for formula and prerequisite evaluation without choosing a final evaluator.

## Required runtime qualities
Any future GE-04 evaluator MUST support deterministic evaluation, sandboxed execution with no filesystem/network/process/clock/random/mutable host side effects unless explicitly modeled, typed values or equivalent validation, structured expression representation, dependency discovery, provenance linkage, diagnostics for parse/type/reference/unsupported/unsafe failures, and deferred expression records when conversion is not safe.

## Formula result expectations
Formula evaluation SHOULD return a structured result resembling:

```yaml
expression_id: <stable id>
source_ref: <canonical/provenance ref>
status: pass | fail | unknown | diagnostic
value: <typed value or null>
dependencies: []
diagnostics: []
explanation_node_ref: <node id>
```

This is conceptual, not final schema authority.

## Prerequisite result expectations
Prerequisite evaluation SHOULD return a structured result resembling:

```yaml
prerequisite_id: <stable id>
subject_ref: <character/object/input ref>
status: satisfied | not_satisfied | unknown | diagnostic
expected: <condition description or structured expression>
actual: <observed value/state or null>
dependencies: []
diagnostics: []
explanation_node_ref: <node id>
```

Failed prerequisites MUST expose expected-versus-actual information when available.

## Prohibited evaluator behavior
A future evaluator MUST NOT execute arbitrary user/plugin code with host side effects, silently coerce invalid expressions into zero/false/default values, evaluate formulas without dependencies, return a naked prerequisite boolean when expected/actual detail exists, hide unsupported legacy fragments in generic failures, or claim PCGen parity without GE-05 evidence.

## Candidate-selection boundary
This artifact does not choose CEL, Rhai, a custom DSL, embedded Rust, or any other evaluator. Selection requires a later spike or decision record that tests pilot formula/prerequisite pressure and documents tradeoffs.

## Minimum pilot expression pressures
The pilot evaluator requirements should expect pressure from ability-score modifiers, class progression formulas, save and attack-related formulas, skill-related values, feat/proficiency prerequisites, equipment-affected derived values, and choice-set filters/selectors.

## Scalar-derived magnitude evaluation (added 2026-08-02)

GE-01 classifies every imported rule record on a `wiring_class` axis — `display`, `static`, `derived`, `computed`, `ambiguous` — defined at `../../GE-01-legacy-corpus-and-conversion-matrix/artifacts/wiring-class-determination.md`. That vocabulary is cited here, not restated. One of those classes places a requirement on this engine.

**Requirement.** GE-04 MUST provide a **scalar-derived magnitude evaluator**: the component that evaluates the magnitude of a `derived`-class record and returns a value with its dependencies and provenance. A `derived` record's magnitude is a deterministic function of scalars the engine already holds — caster level, class level, total levels, BAB, racial HD, an ability modifier, an item's own enhancement total. There is no bespoke wiring to build for these records and none should be built.

**Scale of the requirement, re-derived 2026-08-02 from `docs/work-inventory.json` (`generated_at 2026-08-02T04:02:12Z`):**

```
$ python3 docs/release/GE-01-legacy-corpus-and-conversion-matrix/artifacts/wiring-class-determination.py HELD
scope HELD  n=9828
  display      3882   39.5%
  static       3024   30.8%
  computed     1570   16.0%
  derived      1276   13.0%
  ambiguous      76    0.8%
```

1,276 currently-held units are `derived`; 959 of them are units presently stalled in `ingested-magnitude` and therefore not counted as proven at all.

**What the evaluator must handle, grounded in the corpus rather than assumed.** These are the concrete PCGen expression shapes the `derived` class is made of:

| shape | example | source |
|---|---|---|
| parenthesised expression embedded in display text | `DESC:…deals (min(10,CASTERLEVEL))d6 points of fire damage…` | `core_rulebook/cr_spells.lst`, *Fireball* |
| the same in a duration field | `DURATION:(CASTERLEVEL) minutes [D]` | `core_rulebook/cr_spells.lst`, *Antiplant Shell* |
| a keyword whose value is a caster-level function | `RANGE:Close` = 25 ft + 5 ft per 2 caster levels; `Medium` = 100 + 10/CL; `Long` = 400 + 40/CL | 474 of 1,067 stalled spell units |
| arithmetic over an item scalar | `COST:4000*PLUSTOTAL*PLUSTOTAL` | `core_rulebook/cr_equipmods.lst:263`, *Amulet of Mighty Fists* |
| a `BONUS:` value over an ability modifier | `BONUS:SKILL\|TYPE.Charisma\|max(0,WIS)\|TYPE=WisdomBonus` | `advanced_class_guide/acg_equip.lst:332` |

The first two matter most: the expression lives inside `DESC:`/`DURATION:`, which are **display fields, not magnitude tokens**. An evaluator wired only to `BONUS:`/`DEFINE:` will not see a single scaling spell in the corpus. The evaluator's input surface must include the prose fields GE-01 names (`DESC:`, `DURATION:`, `TARGETAREA:`, `SPROP:`, `RANGE:`, `SPECIALS:`), with the substitution rendered back into the text the player reads.

**Result shape.** A `derived` evaluation returns the standard formula result already specified above (`expression_id`, `source_ref`, `status`, `value`, `dependencies`, `diagnostics`, `explanation_node_ref`). `dependencies` MUST name every scalar consumed — this is what makes the result auditable and what lets GE-09 detect a value that was hardcoded rather than evaluated.

**Boundary — this is not a general LST interpreter.** The evaluator is scoped to records GE-01 has classified `derived`. It does not evaluate guarded, temporary, or choice-driven magnitudes: those are `computed`, and their bar remains a real consumer observing a delta. This preserves the existing prohibition on a general `BONUS:`/`DEFINE:`/`PREREQ:` formula interpreter (SD-27 `decisions.md §24.1`) and sits inside the display-value discriminator already ruled by the operator (SD-28 `decisions.md §27`): *compute the number and render it; do not build the subsystem its noun implies*. `wiring_class` is the mechanical form of that discriminator — §27 states the test in prose and requires a human to apply it per record; `derived` applies it from the token shape.

**Prohibited, in addition to the prohibitions above.**
- MUST NOT return a `derived` value without its `dependencies` populated. A magnitude with no declared inputs cannot be distinguished from a transcribed constant.
- MUST NOT evaluate a record classified `computed` through this path. Doing so would produce a plausible number for a magnitude that is conditional, and would silently lower that record's evidence bar.
- MUST NOT treat an `ambiguous` record as `derived` by guessing the formula from prose. `advanced_class_guide/acg_spells.lst:14` *Air Geyser* — *"hurls the target upward a number of feet equal to 5 x your caster level"* — is scaling stated only in English; it is a content-resolution work item, not evaluator input.

**Verification obligation.** A `derived` record is proven when the evaluator returns the correct value at sampled inputs against a fixture, per GE-09's per-class proven rule. Sampling, not a single level: a formula correct at level 1 and wrong at level 11 is the failure mode this class exists to catch, and `min(10,CASTERLEVEL)`-shaped caps make the boundary the interesting sample.
