# Monk's Remaining 4 Restricted Bonus Feats (Combat Reflexes, Deflect
# Arrows, Improved Grapple, Scorpion Style) — Scoping Plan

> Requested by the lead after this session's ACG scan produced no further
> cheap wins (4/4 recent candidates — Skald, Bloodrager, Brawler, Hunter —
> landed; the follow-on scan of Arcanist/Investigator/6 APG classes found
> nothing else cheap). This is the "genuinely new engine work" the lead
> flagged as correctly deferred until the cheap-win vein ran dry. Per the
> lead's explicit instruction, this doc is written before any code, and a
> formal adversarial review is requested given the scale — the same
> process the original combat-time-activation-state plan (which produced
> Barbarian Rage, Bard Inspire Courage, Cleric Touch of Good, Skald
> Inspired Rage, Bloodrager Bloodrage) went through before that pattern
> was trusted to repeat without review.

## Re-confirming the premise before assuming the worst (this session's
## own core discipline) — the finding is more nuanced than "all 4 need a
## brand-new opponent engine"

The prior characterization ("each of these 4 needs a genuinely new
opponent-interaction engine, no cheap shortcut") is TRUE for one of the
four, but not uniformly true for the other three. Re-reading each feat's
real PF1 text directly (Core Rulebook via the PCGen corpus, `cr_feats.lst`)
turned up a real distinction worth acting on:

- **Combat Reflexes** (`cr_feats.lst:34`): "You may make %1 additional
  attacks of opportunity per round... equal to your Dexterity bonus, if
  any" (corpus: `BONUS:VAR|CombatReflexesAttacks|DEX`). This grants a
  flat, standalone NUMBER (the extra-AoO capacity) derived purely from
  the character's own Dexterity modifier -- not an opponent-dependent
  resolution. The genuinely unrepresented piece is WHETHER an attack of
  opportunity is ever actually triggered (an opponent's own movement/
  action, which this codebase has zero representation of at all,
  correctly ruled out from being "closed" the same way Deflect Arrows'
  incoming attack was). But the CAPACITY itself -- "how many additional
  AoOs could this character make if the opportunity arose" -- is exactly
  the same shape as grounding Sneak Attack's die COUNT without resolving
  an actual attack roll, or Fascinate's DC without resolving the actual
  save. Groundable honestly, without fabricating any opponent behavior.
- **Scorpion Style** (`cr_feats.lst:142`): "the target's base land speed
  is reduced to 5 feet for a number of rounds equal to your Wisdom
  modifier unless it makes a Fortitude saving throw (DC %1)" with the
  corpus formula `10+(TL/2)+WIS` (TL = total/monk level). This grounds
  TWO flat, standalone numbers purely from the Monk's own level and
  Wisdom modifier: the save DC and the speed-reduction duration. Neither
  requires resolving the actual unarmed strike attack roll or the
  target's own saving throw -- both stay genuinely unrepresented (an
  opponent's AC to hit, an opponent's own Fortitude save bonus), but the
  DC/duration VALUES the Monk would present are real, computable numbers
  about the Monk alone.
- **Improved Grapple** (`cr_feats.lst:98`): "You do not provoke an attack
  of opportunity when performing a grapple combat maneuver check. In
  addition, you receive a +2 bonus on checks made to grapple a foe. You
  also receive a +2 bonus to your Combat Maneuver Defense" (corpus:
  `BONUS:VAR|CMB_Grapple,CMD_Grapple|2`). Two flat +2 bonuses -- but
  **this codebase currently computes no Combat Maneuver Bonus (CMB) or
  Combat Maneuver Defense (CMD) baseline at all, for any class**
  (confirmed by direct grep: zero hits for `compute_cmb`/`compute_cmd`/
  `combat_maneuver` anywhere in `pilot_compute.rs`). CMB/CMD are
  themselves purely derived from the character's own stats (CMB = BAB +
  Strength modifier + size modifier; CMD = 10 + BAB + Strength modifier +
  Dexterity modifier + size modifier, PF1 Core Rulebook p.20 — no
  opponent-dependent term in the baseline formula itself), so building
  this baseline is new architecture, but NOT opponent-interaction
  modeling -- the same category of work as building the AC/save/attack
  baselines already in this codebase, just a pillar that hasn't been
  started yet. Improved Grapple's own +2/+2 then layers on top exactly
  like Weapon Focus's +1 already layers onto the melee attack baseline.
- **Deflect Arrows** (`cr_feats.lst:49`): "Once per round when you would
  normally be hit with an attack from a ranged weapon, you may deflect
  it so that you take no damage from it." **This is the one feat with NO
  standalone numeric value at all** -- the entire benefit IS the
  resolution (an opponent's ranged attack occurring and being negated).
  There is nothing about this feat that reduces to a flat number derived
  from the Monk's own stats the way the other three do. This one
  genuinely needs the full opponent-interaction engine (an incoming-
  attack representation this codebase has zero framework for) and stays
  claim-blocked with an honest, permanent gap diagnostic -- the correct
  category this feat was already placed in, unchanged.

## Proposed scope for this slice

1. **New CMB/CMD baseline pillar** (`compute_combat_maneuver_baseline` or
   similar): CMB = base attack bonus + Strength modifier + size modifier
   (0 for Medium, the only size this codebase's fixtures use); CMD = 10 +
   base attack bonus + Strength modifier + Dexterity modifier + size
   modifier. This is the one piece of genuinely new architecture in this
   slice -- a new pillar. **Real wiring complication, confirmed by direct
   code read before proposing this**: Monk is NOT currently part of
   `has_supported_class_chassis` at all -- `explain_monk_level1_chassis`
   is called directly and unconditionally from `compute_pilot_base_chassis`
   as its own bespoke, standalone path (mirroring no other class), and
   Monk never reaches `compute_total_saves`/`compute_combat_baseline`
   today (it stays `Blocked` on those pillars via the generic
   "missing supported Fighter/Wizard chassis" diagnostic, the same as
   any unrecognized class). So gating the new CMB/CMD pillar on
   `has_supported_class_chassis` alone would NOT make it reach Monk --
   this pillar needs to be called explicitly from
   `explain_monk_level1_chassis` too (or Monk needs its own
   `is_supported_monk_single_class`-style addition to
   `has_supported_class_chassis`, a separate, larger decision this doc
   does not propose). Recommend building the CMB/CMD computation as a
   standalone function callable from both places, called explicitly from
   Monk's own bespoke path for this slice, without folding Monk into the
   shared gate (that's a bigger, separate question -- Monk currently
   never reaches `Computed` at all, and folding it into
   `has_supported_class_chassis` would have implications for total
   saves/combat baseline/skills this doc hasn't scoped). Recommend this
   be the specific focus of the requested adversarial review, since it's
   the first time this session adds a new baseline pillar rather than a
   per-class bonus layered onto an existing one.
2. **Combat Reflexes**: ground the additional-AoO capacity
   (`max(Dexterity modifier, 0)`) as a standalone explanation record when
   selected as Monk's bonus feat, mirroring the Sneak-Attack-die-count /
   Fascinate-DC idiom -- a real number, explicitly NOT claiming any AoO
   is ever actually triggered (that stays honestly unmodeled, named the
   same way Deflect Arrows' own triggering event is).
3. **Scorpion Style**: ground the save DC (`10 + monk_level/2 +
   wisdom_modifier`) and the speed-reduction duration
   (`wisdom_modifier` rounds) as standalone explanation records when
   selected, explicitly NOT claiming the unarmed strike attack roll or
   the target's own save is ever resolved.
4. **Improved Grapple**: ground the +2/+2 CMB-grapple/CMD-grapple bonuses
   on top of the new baseline pillar when selected, explicitly NOT
   claiming a grapple check or an opponent's own CMB is ever resolved.
   Also grounds the "no attack of opportunity provoked while grappling"
   as an honest recognition record (a real rule, but with nothing to
   compute -- AoO provocation isn't tracked at all in this codebase, so
   this is a vacuous-correction note, not a fabricated mechanic).
5. **Deflect Arrows**: stays exactly as claim-blocked as it is today --
   no change, explicitly re-confirmed rather than silently left alone.
6. Each of the 3 groundable feats replaces its own slice of the existing
   monk bonus-feat "no execution engine" diagnostic (mirrors every
   diagnostic-narrowing this session has done for every other class) --
   the catch-all preserved byte-for-byte for every unrecognized/
   unselected/out-of-range case.
7. New tests mirroring the established shape: each of the 3 groundable
   feats reaches a real, non-fabricated value when selected; Deflect
   Arrows' selection still trips the unchanged claim-blocking diagnostic;
   an unrecognized bonus-feat selection still falls through to the
   original catch-all; the new CMB/CMD pillar is exercised for at least
   one already-admitted class (Fighter) as a baseline sanity check before
   Monk's own bonus-feat layer is added on top.

## What stays explicitly out of scope, named honestly

- Any actual attack-of-opportunity trigger, grapple-check resolution, or
  ranged-attack-deflection event -- none of these are simulated; only the
  flat numbers a Monk would bring TO such an event are grounded.
- Deflect Arrows' own benefit entirely (no groundable static value
  exists for it, per the analysis above).
- CMB/CMD's own further modifiers beyond the base formula (size-specific
  maneuver bonuses/penalties for maneuvers other than grapple, magic
  item bonuses, etc.) -- this slice only grounds the bounded base formula
  plus Improved Grapple's own named +2/+2, the same "smallest defensible
  slice" discipline as every other pillar this session built.

## Open questions for the lead / adversarial review

- **Is the CMB/CMD baseline pillar in scope for this slice, or should it
  be split into its own separate cycle** (build the pillar generically
  first, verified independently for Fighter/Wizard/etc., THEN layer
  Improved Grapple's own bonus on in a follow-on slice)? Leaning toward
  bundling them (the pillar has no independent value without at least
  one class-specific consumer to prove it's wired correctly), but this
  is exactly the kind of scope question the lead's own review process
  exists to catch.
- **Is grounding a capacity/DC/duration number without ever resolving
  the event it feeds into a legitimate closure, or does it risk reading
  as more "done" than it is?** This mirrors the Sneak-Attack-die-count
  and Fascinate-DC precedents exactly, but those preceded a UI that
  might display "you have Combat Reflexes: +2 additional attacks of
  opportunity" in a way a player could misread as "this is being
  tracked in combat" rather than "this is the number, tracking it is on
  you." Recommend the same disclaimer discipline as every other
  standalone-value record this session has produced, named explicitly in
  the explanation text.
- **Should Deflect Arrows get its own dedicated "why this one stays
  fully blocked, unlike its siblings" diagnostic/explanation**, now that
  3 of its 4 former identical siblings have moved forward? Recommend yes,
  narrowing its own message the same "diagnostic honesty" way every
  other partially-resolved diagnostic was narrowed this session, even
  though Deflect Arrows itself doesn't change -- the SURROUNDING claim
  ("none of the 5 restricted bonus feats have any execution logic") would
  otherwise become stale for the other 3.

## Revision after adversarial review (2026-07-25)

The review found a major scope simplification plus 3 named honesty
fixes -- all addressed before/during the build, no second review needed
per the lead's own read of how targeted the fixes were.

**1. No new CMB/CMD baseline pillar needed at all.** The review found an
existing precedent this doc missed: `pilot_compute.rs`'s already-landed
Dwarf Stability record (`race.dwarf.trait_bundle.stability`) already
grounds a grapple-adjacent CMD bonus MAGNITUDE (+4
`CMD_BullRush`/`CMD_Trip`) as a standalone recognition record with an
explicit "no CMD-total engine exists" disclaimer -- the exact same shape
Improved Grapple's own `+2 CMB_Grapple,CMD_Grapple` needs. Improved
Grapple closes the same way Dodge/Catch Off-Guard/Throw Anything did:
no new pillar, no new architecture, just a flat magnitude record
mirroring Dwarf Stability's own idiom. This slice needed zero new
architecture -- the CMB/CMD baseline pillar (item 1's own original
proposal) is deferred to its own, separately-scoped, separately-reviewed
future slice, with the honest note that when it IS built, size modifier
must not be baked in as a silent 0 (only valid for Medium; Small races
this codebase's Fighter path already admits would need -1, following the
same disclosure pattern `pilot_compute.rs:6127` already uses for its own
"no size-modifier term exists" gap), and it should be verified via a
direct-call unit test, not by injecting a record into any class's actual
production explanation vector (which would change that class's golden
output and needs its own deliberate scope).

**2. Fixed a stale blanket-claim bug, same class of bug as a stale
test-fixture assumption, just in prose.** The choice-recognition
record's own detail text (`pilot_compute.rs`, level-1 bonus feat slot)
asserted "{feat_name}'s own mechanics are not grounded here... no
attack-resolution, grapple-check, or DC/save engine exists" unconditionally
for every recognized feat -- already false for Dodge/Catch Off-Guard/
Throw Anything before this fix, and now also false for Combat Reflexes/
Scorpion Style/Improved Grapple. Made conditional on whether the named
feat is one of the six this seam separately closes, so the claim is never
asserted about a feat whose mechanics genuinely are grounded elsewhere.

**3. Three quote-fidelity fixes applied**: Combat Reflexes' own
explanation text cites the literal corpus formula token
(`BONUS:VAR|CombatReflexesAttacks|DEX`) and names the "max(Dexterity
modifier, 0)" framing explicitly as this codebase's own gloss, not a
direct quote of the feat's BENEFIT text (which only says "You may make
%1 additional attacks of opportunity per round"). Improved Grapple's own
text preserves "whenever an opponent tries to grapple you" (the CMD bonus
is grapple-specific, not general) and says "grapple combat maneuver"
rather than "grapple combat maneuver check" (matching the corpus's own
wording exactly). Scorpion Style's DC explanation names the formula's
`TL` term as "total level, standing in for this bounded single-class
seam's own Monk level since no multiclass mix is admitted here" rather
than silently substituting "monk level" for "total level" without
comment.

**Built**: `monk_combat_reflexes_additional_attacks_of_opportunity`,
`monk_scorpion_style_dc`, and inline handling for Improved Grapple's flat
`MONK_IMPROVED_GRAPPLE_BONUS` constant, all three wired into
`explain_monk_level1_chassis`'s existing bonus-feat-selection branch,
mirroring the Dodge/Catch-Off-Guard/Throw-Anything closure shape exactly
(each returns early after grounding its own real value, skipping the
trailing "still blocked" diagnostic). Deflect Arrows is unchanged --
still the one restricted-list feat with zero standalone value, confirmed
correctly categorized. New tests: 3 positive closures (verified exact
values: Combat Reflexes +2 AoO capacity from Dexterity +2; Scorpion
Style DC 11 / duration 1 round from level 1 + Wisdom +1; Improved Grapple
+2 flat bonus) plus a renamed negative test confirming Deflect Arrows
alone still blocks (replacing the now-obsolete
`monk_with_combat_reflexes_bonus_feat_still_blocks` test, which asserted
the pre-closure behavior).
