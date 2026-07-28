# CRB-class diagnostic audit (task #79)

Companion to the #76 audit, covering the 11 CRB classes whose diagnostics that
audit deliberately left out. Higher stakes than #76: these are the classes users
actually reach `Computed` with, so wrong text here is visible on a working
character rather than latent behind a block.

Docs-only. No code changed.

## Method — id-set-first

Unlike #76, which read each diagnostic's claims and then looked for the code,
this audit derived **the shipped explanation-id set per class first**, then read
the prose and diffed in both directions. That ordering matters: it surfaces
*omissions* (a shipped feature the message never mentions) as readily as
*misstatements*, and omission is the failure mode that nearly escaped #76 —
Summoner's Merge Forms was only caught because team-lead asked about it.

The id set was taken from string literals in the production section of
`pilot_compute.rs`, grouped by namespace segment.

## Headline: there is a systematic cause, not 11 independent oversights

**The generalization work moved features into shared namespaces, and the owning
classes' diagnostics still describe only their own namespace.**

- Task #64 generalized Touch of Good to Cleric/Druid/Inquisitor. It ships under
  `class_feature.domain.good_touch_of_good_*` (4 ids) — **not** under
  `class_feature.cleric.*`.
- Task #66 grounded the Abjuration School. It ships under
  `class_feature.school.abjuration.*` (5 ids) — **not** under
  `class_feature.wizard.*`.

Both owning classes' diagnostics were written when the feature would have lived
in the class's own namespace, and neither was revisited when the feature landed
somewhere shared. This predicts where the *next* drift will appear: any future
generalization pass will do the same thing again unless the owning classes'
diagnostics are updated in the same change.

**Recommendation:** treat "generalize a feature into a shared namespace" as
carrying a mandatory follow-on edit to every owning class's diagnostic. This is
cheaper as a habit than as a periodic audit.

## Verdicts

| Diagnostic | Verdict |
|---|---|
| `class_feature.cleric.domain_powers` | **Stale** — false absolute claim |
| `class_feature.druid.animal_companion` | **Stale** — false absolute claim |
| `class_feature.wizard.school_powers_and_opposed_school_cost` | **Omission** — accurate on what it says, silent on Abjuration |
| `class_feature.cleric.healing_domain.rebuke_death` | **Contradicts its neighbour**; minor overreach |
| `class_feature.hybrid.{paladin,ranger}` | **Misleading, not false** |
| `class_feature.sorcerer.arcane_bond_and_bloodline_progression` | Accurate — and the model the others should follow |
| `class_spell.bard.spontaneous_known_and_per_day` | Accurate |
| `class_chassis.barbarian.rage_rounds_per_day` | Accurate (pure validation) |
| `class_spell.*` posture set (cleric, druid, wizard, sorcerer, paladin, ranger, hybrid) | Accurate by construction |

Calibration note: I predicted when proposing this that a meaningful fraction
would come back clean and that I'd be checking 8–10 real claims rather than
finding 11 stale ones. That held — most of these are honest, and two of the
three genuine defects trace to the single structural cause above.

---

## Stale

### `class_feature.cleric.domain_powers.unsupported` (line 29637)

> "domain selection, domain spell-list contents, and the granted powers of any
> domain (**e.g. Good's Touch of Good**, Healing's Rebuke Death, whose heal
> amount is not a flat number) **are not implemented anywhere in this
> codebase**"

Shipped: `class_feature.domain.good_touch_of_good_bonus`,
`_uses_per_day`, `_self_application`, `_not_active`, driven by
`cleric_touch_of_good_bonus` (line 30150, 6 references).

The diagnostic fires correctly — it is the `else` branch for a domain that
isn't Good — but the *wording* makes a codebase-wide absolute claim, and picks
as its worked example the one domain power that is implemented. A user on a
Cleric with a non-Good domain sees text asserting Touch of Good doesn't exist.

**Corrected wording:** scope the claim to the branch — the granted powers of
domains *other than Good* are unimplemented — and drop Touch of Good as the
example, or recast it as the exception.

### `class_feature.druid.animal_companion.unsupported` (line 30640)

> "the companion's **stat block**, its advancement, and its **link and share
> spells** abilities are **not implemented anywhere in this codebase**"

Shipped: `ground_wolf_companion_stat_block` (line 5583),
`ground_wolf_companion_link_and_share_spells_vacuous` (line 5716), and the id
`class_chassis.druid.animal_companion.wolf_stat_block`. Witch's own diagnostic
refers to "the already-built Animal Companion Wolf stat block", so a second
in-repo surface already contradicts this one.

Two of the three things it names are implemented. Honest nuance for the
correction: link and share spells are grounded *vacuously* (the function name
says so) — they are recognized as no-ops rather than executed. That is still
"implemented" in this repo's own idiom, which uses vacuous/standalone records
throughout, but the corrected text should say so rather than imply full
execution.

Advancement genuinely is absent, and already has its own honest non-blocking
record (`class_feature.druid.animal_companion.advancement_absent`).

## Omission

### `class_feature.wizard.school_powers_and_opposed_school_cost.unsupported` (line 28930)

Everything it asserts is true: the Evocation intense-spells bonus damage and the
force-missile uses-per-day pool are grounded, the opposed-school preparation
cost is grounded, and the execution paths (the 1d4 roll, automatic-hit
targeting) genuinely are not.

What it never mentions is the **Abjuration School** — 5 shipped ids from task
#66 (`protective_ward_deflection_bonus`, `_duration`, `_uses_per_day`,
`energy_absorption`, `resistance`). A message that enumerates which school
powers are grounded and omits an entire grounded school understates the state of
the codebase.

Same defect shape as Summoner's Merge Forms in #76: **a fix that only edits
existing sentences will miss this. Abjuration has to be added.**

## Contradicts its neighbour

### `class_feature.cleric.healing_domain.rebuke_death.unsupported` (line 29623)

The message reads "**unlike Touch of Good**, there is no honest self-scoped
version of healing another creature" — i.e. it is explicitly *aware* that Touch
of Good is grounded. It sits roughly fourteen lines from the `domain_powers`
message that asserts Touch of Good is not implemented anywhere.

Two Cleric diagnostics, adjacent in the same function, disagree about whether
Touch of Good exists. This is the same failure as Shaman's contradictory pair in
#76, and it is useful evidence for the fix: the `rebuke_death` text is the one
that is right.

Minor, separate overreach: it concludes "no Rebuke Death support is claimed"
while `class_chassis.cleric.domain_power_healing_rebuke_death_uses_per_day`
ships. The *heal amount* reasoning is sound (a dice roll and another creature's
hit-point state, both genuinely unmodelled); the blanket "no support" is too
broad given uses-per-day is grounded.

## Misleading, not false

### `class_feature.hybrid.{paladin,ranger}.unsupported` (line 19483)

For Paladin the burden string is "smite evil, lay on hands, divine grace, and
mercy"; for Ranger, "favored enemy, combat style, and skill/tracking". Every one
of those is grounded on the dedicated per-class path —
`class_chassis.paladin.smite_evil_*`, `lay_on_hands_*`, `divine_grace_save_bonus`,
`mercy_choice`/`mercy_granted`; `class_chassis.ranger.favored_enemy_*`,
`combat_style_*`, `track`.

**Why this is not filed as stale:** the wording is scoped — "not implemented **in
this bounded hybrid chassis baseline**" — which is literally defensible, since
the hybrid seam itself implements none of it.

**Why it still needs attention:** `explain_hybrid_level1_chassis` is still live
on the main compute path (called at 6469), and `supported_paladin_level` accepts
levels 1–10. Smite Evil is ungated (`1 + (level - 1) / 3`), so it grounds at
level 1. For a Human Paladin 1 both surfaces fire on the same run: one grounds
Smite Evil, the other says Smite Evil is not implemented. The reader has no way
to know the second is scoped to a legacy seam.

I verified this by reading the gates rather than executing the compute path, so
treat "both fire on the same input" as a strong inference from the code, not an
observed run. Worth confirming empirically before editing.

## Accurate — no correction needed

**`class_feature.sorcerer.arcane_bond_and_bloodline_progression`** is the model
the rest should follow. Its message is composed at runtime from branch-aware
clauses, and it explicitly retires grounded parts from its own blocker: "the
bloodline class skill grant … is grounded separately above as a recognition
record and **is no longer part of this blocker**". That sentence is exactly the
discipline whose absence produced every other finding in this audit and in #76.

**`class_spell.bard.spontaneous_known_and_per_day`** correctly lists what *is*
grounded (level-access ladder, per-day counts, save-DC arithmetic, spells-known
counts, Charisma bonus slots) before naming execution as out of scope.

**`class_chassis.barbarian.rage_rounds_per_day`** fires only when the computed
value is non-positive, and reports the computation. Pure input validation.

**The `class_spell.*` posture set** (Cleric prepared_divine, Druid
prepared_divine, Wizard prepared_spellbook, Sorcerer spontaneous, Paladin/Ranger
partial_caster, both hybrid spell ids) are all `unmet.is_empty()` else-branches
that interpolate the actual unmet conditions. Honest by construction — they
cannot drift, because they report computed state rather than asserting a
standing fact.

That last point is the constructive lesson: **diagnostics that interpolate
computed state don't rot; diagnostics that hard-code a prose claim about the
codebase do.** Where a blocker can be rewritten to report what it checked rather
than assert what exists, it should be.

## Scope limits

- Fighter and Rogue have no `.unsupported` class diagnostics and were not
  audited beyond confirming that.
- Monk is CRB but was covered as a Blocked class in #76.
- I did not execute the test suite or the compute path; "shipped" means present
  in the production section with a live call site.
- No overlap found with featmate's #76 work — disjoint class sets and disjoint
  diagnostic ids.
