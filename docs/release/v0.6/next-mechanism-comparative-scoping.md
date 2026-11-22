# Next Mechanism Comparative Scoping (Risks Item 8, Post-Sorcerer)

> Requested by the lead after Sorcerer's Arcane bloodline closure: unlike
> the diagnostic-level comparison that picked Sorcerer over Cleric/Druid,
> every remaining option now requires building a genuinely NEW mechanism,
> not reusing an already-proven pattern verbatim. This doc compares four
> mechanism candidates — Cleric's roll-bonus application, Cleric's
> dice-based healing, Druid's animal-companion subsystem, and Monk's four
> remaining opponent-interaction feats — on real implementation cost, and
> finds one candidate (Cleric's Touch of Good, self-scoped) is cheap enough
> to reuse the existing `ClassAbilityActivation` pattern directly rather
> than inventing anything new. Flagging the scope-narrowing that finding
> depends on explicitly, since it's a real, debatable design choice, not a
> free lunch.

## Central finding: Touch of Good, scoped to self-application only, needs
## no new mechanism at all — it reuses Rage/Inspire Courage's own pattern

PF1 Core Rulebook Good Domain granted power, Touch of Good (verified
against the raw PCGen corpus, `cr_abilities_class.lst`, by the lead
directly): "touch a creature... granting it a +X sacred bonus on attack
rolls, skill checks, ability checks, and saving throws for 1 round" — all
four roll categories simultaneously, not a player's choice of one (a
correction to this doc's own first draft, caught by the lead's corpus
cross-check rather than assumed).

The `ClassAbilityActivation` schema (`ability_id`/`active_state`/
`rounds_consumed_today`) has no `target` field at all — it was built for
Rage and Inspire Courage, both of which only ever affect the ACTING
character's own rolls. Reusing it for Touch of Good means the only honest
interpretation is: **the cleric touches herself**, applying the sacred
bonus to her own attack rolls (`combat.baseline_melee_attack_bonus`), skill
checks (`selected_skill_modifiers`), and saving throws (`total_saves`) for
this snapshot. This is a real, deliberate narrowing of Touch of Good's
actual PF1 use (in real play, it's usually used to buff an ally mid-melee)
— but it mirrors exactly how Inspire Courage was scoped to "attack bonus
only, not damage/saves" and how every prepared-caster slice scoped to "the
fixture's own specific spell list," not a limitation unique to this
finding. Ability checks have no separate integrated total in this codebase
(no distinct "ability check" pillar beyond the ability modifier itself),
so that fourth roll category stays a flat, unintegrated magnitude the same
way Inspire Courage's damage/save bonus did.

**If this self-scoping is accepted**, Touch of Good needs:
1. A new `ability_id` value (`"touch_of_good"` or similar) — purely
   additive, zero schema change.
2. A new `active_bard_inspire_courage_attack_bonus`-shaped query function
   (class-ownership-gated on Cleric + Good domain chosen), reusing
   `ability_modifier_for`/`choice_selection` exactly as Rage/Inspire
   Courage did.
3. Three pillar touches (`compute_combat_baseline`'s attack side,
   `compute_selected_skill_modifiers`, `compute_total_saves`) — each
   already takes `input: &CharacterInput` directly, same as the two prior
   closures.
4. No rounds-per-day CONSUMPTION tracking needed by the same reasoning the
   lead endorsed for Arcane Bond (the existing `touch_of_good_uses_per_day`
   informational record already exists; whether a per-use consumption
   count is worth adding is a smaller, separable question, not a blocker).

This is genuinely comparable in size to Bard's Inspire Courage slice, not
a new mechanism — the one thing that makes it NOT a repeat of the
Barbarian/Bard pattern is the explicit self-target narrowing above, which
needs sign-off before coding (see Open Questions).

## Why the other three candidates are NOT comparably cheap

- **Rebuke Death (Healing domain)**: verified against the raw corpus
  directly by the lead — the heal amount is an explicit dice roll (1d4 +
  cleric level / 2), not a deterministic formula, AND its target ("a
  living creature below 0 hit points") is a DIFFERENT creature's HP state.
  This codebase's `CharacterInput` has no concept of any creature other
  than the one being computed — no "ally" or "target" entity exists
  anywhere in the schema. Unlike Touch of Good (which CAN be honestly
  self-scoped, since the caster's own rolls are a real value this codebase
  already computes), there is no honest "self-scoped" version of healing
  ANOTHER creature — the caster cannot meaningfully heal herself with a
  power whose entire textual predicate names a separate target. This
  stays a genuine gap, the same category as Monk's opponent-dependent
  feats below, not vacuous and not cheaply self-scopable.
- **Druid's animal companion**: needs an entirely new entity type (a
  Companion struct: ability scores, HD progression, natural attacks by
  companion type — PF1 Core Rulebook names roughly 9 companion types, each
  with its own base statistics and advancement table) plus a new
  `CharacterInput` schema surface to carry it. The single largest lift of
  the four by a wide margin; nothing here reuses an existing pattern.
- **Monk's four remaining feats** (Combat Reflexes, Deflect Arrows,
  Improved Grapple, Scorpion Style): all four require modeling AN
  OPPONENT'S behavior or state (a provoked attack of opportunity, an
  incoming ranged attack, an opposed grapple check, a hit-and-save
  resolution against a target) — the same "no other-creature concept
  exists" gap Rebuke Death has, except none of these four even has a
  self-scoping escape hatch the way Touch of Good does (their entire
  benefit is inherently about interacting with something this codebase
  has never represented). Confirmed via the earlier Monk scoping pass:
  zero opponent-action/incoming-attack/grapple-resolution concept exists
  anywhere in `pilot_compute.rs` or `character_input.rs`.

## Relative cost ranking

1. **Touch of Good (self-scoped)** — cheap, reuses the proven
   `ClassAbilityActivation` pattern verbatim, no new mechanism.
2. **Rebuke Death, Druid's companion, Monk's four feats** — all
   genuinely need a NEW concept this codebase has never had (a
   dice-resolution engine, a second-creature entity type, or an
   opponent-interaction engine respectively) — none is a small next step;
   picking among these three is a much bigger commitment than anything
   built this session, closer in kind to the original combat-time-
   activation-state plan's own scale, arguably larger since none of them
   has ANY existing partial infrastructure to build on (Rage/Performance
   at least had `ActiveState`/Power Attack as a weak precedent; these three
   have no precedent of any kind).

## Proposed approach (if Touch of Good is greenlit)

1. Add `"touch_of_good"` as a recognized `ability_id`, gated on Cleric +
   Good domain (mirrors the `active_barbarian_rage_bonus`/
   `active_bard_inspire_courage_attack_bonus` class-ownership-gate shape
   exactly).
2. Layer the flat sacred bonus onto `compute_combat_baseline`'s attack
   side, `compute_selected_skill_modifiers`, and `compute_total_saves` —
   three call sites, each already gated appropriately.
3. Hoist the gate-ordering check into the already-existing hoisted
   Cleric block (domain-powers is already unconditional/hoisted per the
   existing code), so no new gate-ordering bug is introduced.
4. Leave Rebuke Death's diagnostic wording accurate: even with Touch of
   Good resolved, Healing domain's heal-amount burden stays claim-blocking
   (mirrors Sorcerer's own level-3+ bonus-spells split), so a Human Cleric
   with Good+Healing domains reaches `Computed` only if the diagnostic's
   Healing-specific half is ALSO addressed or the fixture is narrowed to
   Good domain alone. Confirming which is intended is an open question
   below.
5. New tests mirroring the Barbarian/Bard dispatch-widening-safety shape:
   self-touch active applies the bonus to all three pillars, inactive/
   absent stays valid, class-ownership gate holds for non-Cleric
   characters.

## Open questions for the lead / adversarial review

- **Is self-scoped Touch of Good an acceptable narrowing, or does it
  misrepresent a power whose real PF1 use is buffing an ally?** This is
  the one place this finding's honesty depends on an explicit judgment
  call, not just verification — recommend treating it the same as Inspire
  Courage's "attack only" narrowing (documented, bounded, not silently
  assumed), but flagging it here for a second opinion given "you touch
  yourself with an ally-buff power" reads slightly differently than
  Inspire Courage's own scope-narrowing did.
- **Does resolving only Touch of Good (not Rebuke Death) get the fixture's
  existing Good+Healing Cleric to `Computed`, or does the diagnostic need
  splitting into two independent diagnostics (one per domain) the way
  Barbarian's rage-execution and Bard's spell-posture stayed separate?**
  Recommend splitting: a genuine per-domain diagnostic shape, mirroring
  how Sorcerer's bloodline diagnostic and spell-posture diagnostic already
  stay independent. Under that split, a Good-only Cleric (no Healing
  domain) could reach `Computed`; the existing Good+Healing fixture would
  not, since Healing's Rebuke Death stays a real, separate, permanent
  blocker — an honest, bounded partial win, the same shape as Bard's own
  Inspire-Courage-closes/spell-posture-stays-blocked split.
- **Is the size ranking among Rebuke Death / Druid companion / Monk's four
  feats correct, or does one of them have its own hidden cheap angle the
  way Sorcerer did?** Recommend at least a targeted re-check of Rebuke
  Death specifically before ruling it out entirely — "heal amount is a
  dice roll" is solid, but is there a narrower sub-case (e.g., recognizing
  the choice + uses-per-day without ever resolving the heal amount, mirror
  Arcane Bond's exact shape) that closes the diagnostic even though the
  heal itself stays unexecuted? This doc assumed no, on the grounds that
  Rebuke Death's target (another creature) is unrepresentable the same way
  Deflect Arrows' opponent is — but this deserves the same scrutiny this
  doc gave Touch of Good before ruling it out for good.

## Revision after adversarial review (2026-07-25)

The lead's review confirmed Touch of Good is worth building but found the
plan above under-scoped it — three real additions, not one, plus a
messaging correction. Recorded here before any code, per the lead's
request.

**1. Messaging correction (accepted as stated)**: the self-scoped
grounding must say explicitly that this is Touch of Good's SELF-
application only — the RAW-secondary use case, not the RAW-primary one
(buffing an ally). The review's framing: this narrows by TARGET, not by
FACET the way Inspire Courage's attack-only scoping did, and the
explanation text must say so plainly ("granting this to another creature
is unmodeled; no target-creature entity exists anywhere in this
codebase"), not merely omit the ally case silently.

**2. The diagnostic split needs a preserved catch-all, verified for real**:
the current diagnostic (`pilot_compute.rs:17427`, confirmed by the lead
directly) is unconditional for ANY cleric, regardless of domain. Splitting
it by Good/Healing recognition without a catch-all would let a Cleric with
an unrecognized domain (Fire, Travel, Trickery, etc. — 19 more domains
this seam has never named) slip through with domain powers entirely
unmodeled and no diagnostic at all — the exact false-Computed shape the
Ranger dispatch-widening review caught originally. The revised structure:

```
let domain_selections: Vec<&str> = /* unchanged existing collection */;
let good_domain_chosen = domain_selections.contains(&GOOD_DOMAIN_SELECTION);
let healing_domain_chosen = domain_selections.contains(&HEALING_DOMAIN_SELECTION);
let unrecognized_other_domain = domain_selections.iter()
    .any(|d| *d != GOOD_DOMAIN_SELECTION && *d != HEALING_DOMAIN_SELECTION);

if good_domain_chosen && !unrecognized_other_domain {
    // ground Touch of Good (self-scoped, honestly worded)
    // push a non-blocking "domain spell-list contents unmodeled" diagnostic (see #3)
    if healing_domain_chosen {
        // push Healing's OWN still-blocking Rebuke Death diagnostic
        // (claim_blocking: true) -- this Cleric stays Blocked overall
    }
    // if healing_domain_chosen is false and no unrecognized domain is
    // present, this Cleric (Good + nothing/one slot only) can reach
    // Computed on the domain-powers front specifically
} else {
    // UNCHANGED existing unconditional diagnostic -- the real catch-all,
    // covering "no domain chosen," "an unrecognized domain chosen," and
    // "Good not chosen at all"
}
```

This preserves the exact original behavior (full generic block) for every
input this seam doesn't specifically improve, mirroring how Sorcerer's own
"bloodline not recognized" branch stayed the unchanged catch-all.

**3. Domain spell-list contents must be addressed, not silently dropped**:
the existing diagnostic names "domain spell-list contents" as part of what
it blocks. Building a real Good-domain spell list (verified against 2
primary sources, the same discipline every other spell list this session
used) is real, additional, in-scope work — but scoping it OUT explicitly
(rather than building it) is the cheaper, still-honest option the review
offered: a new PERMANENT, NON-BLOCKING diagnostic naming "which domain
spell fills the domain slot" as still unmodeled for every domain
(mirroring Bard's own "other six performances, permanently unmodeled,
non-blocking" idiom exactly) — the domain slot COUNT stays grounded
(already real, unchanged), only its CONTENT stays honestly named-but-
absent. Recommend this cheaper path over building a new spell list, to
keep this slice's size proportionate to Sorcerer's, not open a fourth new
body of verification work in the same cycle.

**Net revised scope**: messaging fix + widened domain-choice recognition
(Good-alone, not just Good+Healing-together) + preserved catch-all +
one new non-blocking spell-list-contents diagnostic + the three pillar
touches (attack/skill/save) already planned. A Human Cleric with ONLY
Good domain (or Good + an unrecognized domain is explicitly excluded by
the `!unrecognized_other_domain` guard above) reaches `Computed` on the
domain-powers front; Good + Healing stays honestly `Blocked` on Rebuke
Death alone, the same partial-closure shape Bard's Inspire-Courage-closes/
spell-posture-stays-blocked split already established.

**Rebuke Death**: confirmed correctly ruled out by the review — its heal
amount is real-but-unrepresented, not provably-vacuous the way Arcane
Bond's spell-cast was, so no Arcane-Bond-style recognition-only shortcut
applies here. Stays blocked, no further scoping needed on this piece.
