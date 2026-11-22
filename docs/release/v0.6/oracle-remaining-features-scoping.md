# Oracle (#10) — Fresh Scope Under the Corrected Standalone-Grounding Bar

> Directed by the lead: Oracle stays permanently Blocked regardless (proven
> via a dedicated test) — this is about honestly growing
> `named_features_wired`, the same as Warpriest's Blessings. Oracle's own
> precedent is already 2-for-2: Life Mystery's Healing Hands (`+4` Heal)
> and Clouded Vision (30-ft cap) were BOTH flat standalone facts pulled
> out of a chooser shape. Re-derive every formula directly from
> `apg_abilities_class.lst`; give an honest split, not a forced verdict.
>
> **Result: Oracle is the largest genuine missed-win surface re-scoped this
> session — but it is also the one where the blocked remainder is largest
> in absolute terms.** Roughly a dozen real flat magnitudes are groundable
> now, several of them near-free reuse of code that already shipped; and
> 34 of the 100 revelation records carry no numeric token at all, which is
> a genuine no-op, not a scoping artifact.

## Corpus scale, re-derived (not carried from the original build doc)

- **10 Mysteries × 10 revelations = 100 revelation records**, plus 10
  Final Revelations (level 20). Of the 100: **24 carry a `PRECLASS:1,Oracle=N`
  selection gate**, and **34 carry zero numeric `BONUS` token of any kind**.
- **6 Curse records, not 5.** `Oracle ~ Tongues` is itself
  `TYPE:...Communicate.OracleCurse` — the original build doc listed it
  separately from "5 real types," and task #10's own description says
  "remaining 4 Curses." The real remainder is **5**: Deaf, Haunted, Lame,
  Wasting, Tongues.
- **Revelation budget (load-bearing scoping constraint):**
  `OracleMysteryLevel` grants `1` at level 1, then `+1` each at Oracle
  3 / 7 / 11 / 15 / 19. **A level-1 Oracle holds exactly ONE revelation.**
  So a canonical seed must pick exactly one; grounding two simultaneously
  at level 1 would be an illegal character. This caps what any single
  character shows, though not what the engine implements.
- Shared vars, verified: `OracleLVL|CL`;
  `OracleCurseLVL|OracleLVL+((TL-OracleLVL)/2)`;
  `OracleRevelationDC|10+classlevel("Oracle")/2+CHA`.

## Real missed wins — Curses (3 of the 5 remaining)

### Lame — flat, level-1, self-scoped

- `BONUS:VAR|OracleLameEffect|10|PREMOVE:1,Walk=30` and
  `|5|!PREMOVE:1,Walk=30`, applied via `BONUS:MOVEADD|TYPE.Walk|-OracleLameEffect`.
  Base land speed −10 ft for a 30-ft race, −5 ft for a 20-ft race.
- Base land speed by race is already modelled here, so this is a clean flat
  magnitude — and the exact mirror image of Cinder Dance below.
- `UNENCUMBEREDMOVE:HeavyLoad` and the fatigue immunities at curse level
  5/10/15 are non-numeric — defer those halves, name them.

### Wasting — flat, level-1, and carries a real trap

- `BONUS:SKILL|STAT.CHA|-4` **plus** `BONUS:SKILL|Intimidate|4`.
- **The net effect on Intimidate is zero, not `+4`.** Intimidate is a
  CHA-based skill, so the `-4` hits it too and the `+4` only cancels it —
  matching RAW ("a −4 penalty on all Charisma-based skill checks except
  Intimidate"). Intimidate is one of the three *computed* selected-skill
  modifiers, so a naive `+4` here would land a wrong number on a live
  total. Worth stating explicitly in the build.
- Immunities at curse level 5/10/15 are non-numeric — defer, name.

### Deaf — flat, level-1, three separate magnitudes

- `OracleDeafInitPenalty`: base `-4`, `+2` at curse level ≥5, `+2` at ≥10
  (the two `.MOD` records) → **−4 (levels 1-4), −2 (5-9), 0 (10+)**, via
  `BONUS:COMBAT|INITIATIVE`.
- `BONUS:SKILL|Perception|3` at curse level ≥5 — flat competence bonus.
- `BONUS:SITUATION|Perception=Opposed|-4` — flat, but *situational*. My
  read is it still grounds (the magnitude is fixed and applies to your own
  roll; it needs nothing known about the opponent, unlike Studied Combat) —
  but this is the closest thing to the established gray zone in this pass,
  so flagging it for the lead rather than assuming.
- Scent at ≥10 and `VISION:Tremorsense (30')` at ≥15 — defer.

### Correctly blocked, not missed

- **Haunted**: zero tokens of any kind, DESC-only. Genuine no-op, the
  Nature Training / Alchemist Martial Training precedent exactly.
- **Tongues**: `BONUS:ABILITYPOOL|Oracle Tongues Choice|1` (+1 at ≥5) over
  an 8-language chooser. No magnitude worth grounding; stays deferred.

## Real missed wins — Mysteries, ranked by value and risk

### Tier 1 — near-free reuse or always-on flat self-buffs

**Life Mystery ~ Channel** — the single best-value record on the roster.
The corpus itself declares `SERVESAS:ABILITY=Special Ability|Channel
Positive Energy|Cleric ~ Channel Energy`, and all three formulas are
identical in shape to Shaman's Life Spirit Channel that landed in
`8574c80c`: uses `1+CHA`, dice `(level+1)/2`, die size `6`, DC
`10+(level/2)+CHA`. Near-zero marginal cost.

**Lore Mystery ~ Sidestep Secret** — `BONUS:SAVE|Reflex|max(CHA,DEX)-DEX`,
always-on, no gate, no activation. Both CHA and DEX are computed, and
Reflex save is a *real computed total* here — this is one of the very few
Oracle facts with a live consumer, not just a standalone record. (Also
carries `BONUS:VAR|ACAbilityStat` and `BONUS:EQMARMOR|ACCHECK` halves.)

**Nature Mystery ~ Nature's Whispers** — `BONUS:COMBAT|AC|(max(DEX,CHA)-DEX)`
plus `BONUS:VAR|CMD|(max(DEX,CHA)-DEX)`. Always-on, no gate. Same
stat-substitution idiom as Sidestep Secret, different target.

**Lore Mystery ~ Lore Keeper** — `BONUS:SKILL|<10 named Knowledge
skills>|(max(CHA,INT)-INT)`. Always-on, no gate; same standalone
skill-fact shape as Investigator's Alchemy / Bard's Bardic Knowledge.

**Bone Mystery ~ Near Death** — `+2` insight on saves vs disease,
mind-affecting, and poison; `+4` at level 11+. Investigator/Alchemist
Poison Resistance shape, one tier step.

**Flame Mystery ~ Cinder Dance** — `BONUS:MOVEADD|TYPE.WALK|10`, flat
`+10` ft base land speed, always-on, no level gate. The mirror of Lame.

### Tier 2 — two shared parameterized mechanisms covering 8 records

**Conjured armor bonus (4 records, ONE mechanism)** — Bone/Armor of Bones,
Heavens/Coat of Many Stars, Waves/Ice Armor, Wind/Air Barrier all carry
the identical `4+2*max(0,floor((classlevel("Oracle")-3)/4))` →
**+4 (L1-6), +6 (7-10), +8 (11-14), +10 (15-18), +12 (19-20)**, with
duration `= level` hours/day. Activation-gated with a genuine per-day
budget — the Sacred Armor / Animal Focus shape exactly.

**Energy resistance (4 records, ONE mechanism)** — Stone/Acid Skin,
Waves/Icy Skin, Wind/Spark Skin, Flame/Molten Skin. Inner var is `0`
below 5, `5` at 5-10, `5+10=15` at 11+ (both `PRECLASS` branches apply at
11+), over a base `5` → **resist 5 / 10 / 20**, immunity at 17.
Always-on, no activation.

### Tier 3 — real but low marginal value, recommend at most one canonical

A large **Bomb-shaped damage-and-pool cluster**: Bone/Death's Touch
(`1d6 + level/2`, `3+CHA`/day), Stone/Mighty Pebble, Stone/Shard
Explosion, Wind/Lightning Breath, Heavens/Spray of Shooting Stars,
Nature/Erosion Touch, Waves/Wintry Touch, Stone/Touch of Acid,
Wind/Touch of Electricity, Bone/Bleeding Wounds — all sharing
`OracleRevelationDC = 10+level/2+CHA`. These genuinely satisfy the
corrected bar (a damage magnitude, like Bomb — not a conditional bonus),
but they are ten near-identical records with one shape. Similarly a
**per-day pool cluster**: Battle/Battlefield Clarity and Battle/Surprising
Charge (`1/2/3` at levels 1/7/15), Life/Delay Affliction, Bone/Undead
Servitude (`3+CHA` uses, HD cap `= level`, DC `10+level/2+CHA`),
Heavens/Moonlight Bridge (`level*10` ft, `CHA`/day). Recommend picking
zero or one canonical rather than sweeping the cluster.

## Correctly still blocked — a genuinely large remainder

- **Ally-dependent** — Battle/Battlecry (allies within 100 ft),
  Nature/Friend to the Animals (animals within 30 ft),
  Nature/Transcendental Bond, Life/Life Link. This engine models no
  allies; consistent with Skald's Raging Song ally-extension deferral.
- **Environment/context-dependent** — Heavens/Guiding Star (only "when you
  can see the open sky at night"), Nature/Spirit of Nature (only "in a
  natural setting" below 10th). Same principled line the lead drew for
  opponent-dependent bonuses: a magnitude conditioned on a context the
  engine cannot evaluate would overstate what is verified.
- **Opponent-state-dependent** — Heavens/Awesome Display (subtracts your
  CHA from the *target's* HD), Stone/Clobbering Strike, Waves/Freezing
  Spell, Wind/Vortex Spells.
- **Summon subsystem** — Bone/Raise the Dead. **Exception worth naming:**
  Nature/Bonded Mount is an *Animal Companion*, not a summon
  (`SpecialMountLVL = classlevel("Oracle")`, "functions as a druid's
  animal companion, using your oracle level as your effective druid
  level") — that code exists and is already reused by Druid and Hunter, so
  this is a real reuse candidate, subject to the known
  `wolf_companion_hit_dice` companion-level-1-only boundary.
- **Feat/proficiency grants with no magnitude** — Battle/Resiliency,
  Battle/Skill at Arms, Battle/Weapon Mastery, and the bonus-feat halves
  of Cinder Dance, Stone Stability, and Fluid Nature.
- **Spell-effect wrappers with no independent magnitude** — Lore/Arcane
  Archivist, Lore/Spontaneous Symbology, Lore/Automatic Writing,
  Flame/Form of Flame, Waves/Water Form, Wind/Gaseous Form,
  Heavens/Dweller in Darkness, Nature/Undo Artifice.
- **34 of 100 revelations carry zero numeric `BONUS` token at all** —
  genuinely nothing to ground, the Nature Training precedent at scale.
  Flame is by far the weakest mystery on this axis (8 of its 10).
- **Battle/Maneuver Mastery** — a nested chooser (revelation → 1 of 10
  maneuvers), each `CMB_<maneuver> = classlevel("Oracle") - BAB`. BAB *is*
  computed, so it is tractable, but a chooser inside a chooser exceeds any
  narrowing precedent so far. Recommend deferring unless the lead wants it.

## Build-time hazards found in the corpus (flagged before, not after)

1. **`OracleCurseLVL = OracleLVL + ((TL-OracleLVL)/2)`** — for a
   single-class Oracle `TL == OracleLVL` and it collapses to the Oracle
   level, but it is a *half-progression* for a multiclass Oracle, and this
   engine does support multiclass. Do not hardcode "Oracle level."
2. **Energy resistance is 5/10/20, not 5/10/15** — both `PRECLASS`
   branches of the inner var apply at level 11+. Exactly the class of
   arithmetic the Sacred Weapon `/20`-vs-`/15` bug was; verify across the
   full level range, not a spot-check.
3. **Wasting's net-zero Intimidate** (see above) — the one place in this
   pass where a wrong reading would land on a live computed total.
4. **Cinder Dance carries `!PREABILITY:1,CATEGORY=Special Ability,Oracle ~ Lame`**
   — it is mutually exclusive with the Lame curse. A canonical seed that
   picks both Lame and Cinder Dance would be an illegal character.
5. **Two real corpus asymmetries in the energy-resistance cluster**:
   Molten Skin has no `ABILITY:...Immunity to Fire` token at all (its
   17th-level immunity is DESC-only, unlike its three siblings), and Acid
   Skin's `Resistance to Acid` lacks the `!PRECLASS:1,Oracle=17` guard
   that Icy Skin and Spark Skin both carry — so at 17+ Acid Skin grants
   Resistance *and* Immunity. Ground the shared tier formula; do not
   assume the four records' immunity halves behave alike.
6. **Corpus KEY typos that will break a naive lookup**: `Wind Mastery ~
   Thunderburst` and `Lore Mastery ~ Whirlwind Lesson` (should be
   "Mystery"); `Wind Mystery ~ Wortex Spells` (Vortex); and the Mystery
   itself is keyed `Oracle ~ Winds Mystery` while every one of its
   revelations is keyed `Wind Mystery ~ ...` and its pool is `Wind Mystery
   Revelation`. A lookup keyed on the mystery's own name misses all nine.
7. **Two corpus-incomplete records — do not ground from DESC alone**:
   Heavens/Interstellar Void `DEFINE`s both its damage-dice and uses/day
   vars and then never `BONUS`es either (they stay 0), and Stone/Rock
   Throwing sits directly under an explicit `# TODO: Implement Rock
   Throwing bonuses.` comment in the corpus.

## Honest boundedness read

Oracle is a **large real slice with a proportionally large honest
remainder** — the corrected bar upgrades it substantially, but the
"other 9 Mysteries + other 4 Curses" tag was never uniformly wrong:

- **Groundable now, high value:** 3 Curses (Lame, Wasting, Deaf) and 6
  Tier-1 revelations, one of which (Life/Channel) is close to free reuse
  of already-shipped Shaman code.
- **Groundable now, bounded, 2 mechanisms covering 8 records:** the
  conjured-armor tier and the energy-resistance tier.
- **Correctly blocked:** ally-, environment-, and opponent-dependent
  revelations; the summon subsystem; feat/proficiency and spell-wrapper
  grants; 34 records with no numeric token; and 2 records the corpus
  itself leaves unimplemented.

**On `named_features_wired`:** applying the established counting rule
(distinct KEY records with *separately-implemented* logic; facets of one
mechanism count once), the Tier-1 set is 6 genuinely distinct mechanisms
and Tier-2 is 2 more — so the honest bump from Oracle's current `2` is to
roughly `2 + 3 curses + 6 + 2`, not "+13 records." I have deliberately
not picked the final number; the cluster-collapsing calls (does
stat-substitution count as 3 or 1? does the armor tier count once or
four times?) are the lead's to rule on, and I would rather state the
mechanism inventory than assert a count.

## Open questions for the lead

1. Greenlight which tiers? My recommendation is **Curses (Lame + Wasting +
   Deaf) + Tier 1 (6 revelations)** as one bounded closure, with Tier 2's
   two shared mechanisms as an optional follow-on and Tier 3 deferred
   entirely. That is a real medium-large slice that touches no blocked
   subsystem.
2. **Deaf's `BONUS:SITUATION|Perception=Opposed|-4`** — does a fixed
   penalty applied in an opposed situation clear the corrected bar, or
   does "opposed" put it on the Studied-Combat side of the line? I lean
   groundable (nothing about the opponent is needed to state the
   magnitude), but this should be ruled once, not per-class.
3. **The revelation budget caps a character at one revelation at level 1.**
   Does the closure ground one canonical revelation per mystery in code
   (with only one live per character, Warpriest-Blessing style), or a
   single canonical revelation overall? The former is what makes the
   Tier-1 set worth six mechanisms rather than one.
4. **Nature/Bonded Mount** is a genuine Animal Companion reuse candidate,
   not a summon — worth folding in, or leave it for a follow-on given the
   companion-level-1-only boundary Hunter already hit?
