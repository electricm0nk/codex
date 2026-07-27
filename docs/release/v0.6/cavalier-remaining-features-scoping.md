# Cavalier (#6) — Fresh Scope Under the Corrected Standalone-Grounding Bar

> Cavalier was deferred on the grounds that "Cavalier's Order is a 26-entry
> chooser, bigger than any prior narrowing precedent." **That figure is a
> cross-book count. APG defines exactly 6 Orders** — fewer than Oracle's 10
> Mysteries, Hunter's 13 Animal Focus options, or Warpriest's 33 Blessings,
> all of which have already been narrowed successfully. Cavalier is a
> confirmed non-caster, so there is no spell-list cost hiding behind it
> either. The deferral does not survive contact with the corpus.
>
> **Result: a genuine medium slice, with an unusually clean standout —
> Challenge's self-applied AC penalty, which has exact in-repo precedent.**

## The count correction

| scope | distinct `KEY:Order of the …` records |
|---|---|
| entire PCGen tree (12+ books) | **29** |
| **APG only — the only ones reachable here** | **6** |

Cockatrice, Dragon, Lion, Shield, Star, Sword. The other 23 live in
Ultimate Combat, Ultimate Wilderness, ACG, ARG, Inner Sea Combat, and six
player-companion books, none of which this repo ingests. This is the third
stale cross-book count found in a row (Bloodrager's "201"→183, Oracle's
"4 Curses"→5), so it is worth treating any inherited corpus count as
unverified until re-derived.

Mount is already grounded (`d256bc9c`). 13 named `KEY:Cavalier ~ …` records
remain.

## Real missed wins

### Challenge — 2 of its 3 magnitudes ground; the third correctly defers

`KEY:Cavalier ~ Challenge` carries three real values:

- `BONUS:VAR|CavalierChallengeTimes|(CavalierLVL+2)/3` — **uses/day pool**,
  self-scoped. Same idiom as Warpriest's Fervor pool, Panache, Judgment.
  Grounds.
- **A flat `-2` Armor Class penalty on the cavalier himself** while a
  challenge is active ("The cavalier takes a -2 penalty to his Armor Class,
  except against attacks made by the target of his challenge"). This is the
  standout: a flat, self-applied, activation-gated penalty with **exact
  in-repo precedent** — `BLOODRAGER_BLOODRAGE_ARMOR_CLASS_PENALTY` is
  already grounded and already applied to the baseline Armor Class the same
  way. Grounds.
- `BONUS:VAR|CavalierChallengeLVL|CavalierLVL` — `+level` extra melee damage
  **against the target of the challenge**. This is precisely the
  persistent-tracked-relationship-with-a-specific-opponent shape ruled
  deferred for Slayer's Studied Target and Investigator's Studied Combat.
  **Defers**, consistently.

### Class skills — a 7th instance of the widening bug

`KEY:Cavalier ~ Class Skills`:
`CSKILL:Bluff|Climb|TYPE=Craft|Diplomacy|Handle Animal|Intimidate|
TYPE=Profession|Ride|Sense Motive|Swim` — **all three of Climb, Intimidate,
and Swim.** Cavalier appears in none of `selected_skill_climb_is_class_skill`
/ `…_intimidate_…` / `…_swim_…` (`pilot_compute.rs:28561`, `:28583`,
`:28604`). Same fix as Warpriest / Slayer / Brawler.

### Expert Trainer — flat self-scoped skill fact

`+CavalierLVL/2` on Handle Animal checks involving a mount. Same shape as
Investigator's Alchemy (`+level` Craft-alchemy) and Bard's Bardic
Knowledge, both already grounded. **Evidentiary caveat:** this lives only
in the `DESC:` parameter (`|CavalierLVL/2`), with no `BONUS:SKILL` token —
the weaker Panache/Skald evidentiary path, worth naming rather than
glossing.

### Bonus-feat counts — three flat quantities

- `BONUS:ABILITYPOOL|Cavalier Feat|CavalierLVL/6` — bonus combat feats:
  1 at 6th, 2 at 12th, 3 at 18th.
- Tactician / Greater Tactician / Master Tactician each add
  `BONUS:ABILITYPOOL|Tactician Teamwork Feat|1` at levels 1 / 9 / 17 → 3
  teamwork feats total. **The count is self-scoped and grounds**; the
  *granting the feat to allies within 30 feet* half defers (no ally model).

### Order, narrowed to one canonical — the Warpriest-Blessing shape

All 6 Orders share an identical structure, which makes narrowing cheap:

- `BONUS:VAR|OrderChallengeBonus|CavalierLVL/4` — **byte-identical across
  all six**.
- Two extra class skills apiece (`CSKILL:` on the order record).
- One flat order skill bonus, in the order's own `DESC:`.
- Three order abilities at levels 2 / 8 / 15.

**Recommended canonical: Order of the Sword.** Its own bonus is
`1/2 cavalier level (minimum +1)` on Sense Motive when opposing a Bluff
check — flat, self-scoped, and the opposed-check shape you just ruled
groundable for Oracle's Deaf. Plus its two class skills (Knowledge
(Nobility), Knowledge (Religion)).

## Correctly blocked

- **Ally-scoped** — Banner and Greater Banner (all allies within 60 ft),
  Tactician's granting half, and most order abilities: Dragon's Aid
  Allies / Strategy / Act as One, Lion's Call / For the King / Shield of
  the Liege, Star's For the Faith. No ally model, same line as Skald's
  Raging Song and Oracle's Battlecry.
- **Opponent-relationship-conditioned** — Challenge's damage bonus,
  Demanding Challenge, Cockatrice's Steal Glory, Star's Retribution, and
  **every one of the six orders' own challenge riders** (see hazard 2).
- **Charge-action-conditioned** — Cavalier's Charge (+4 attack on a mounted
  charge), Mighty Charge, Supreme Charge. The magnitudes are flat and need
  nothing about an opponent, but they are conditioned on a charge action
  this engine does not model — the same unmodelled-context reasoning that
  deferred Oracle's Guiding Star. **Flagging rather than deciding**: if you
  read "mounted charge" as closer to an activation than to an environment,
  Cavalier's Charge's flat `+4` would ground. I recommend deferring for
  consistency, but it is a real gray zone.
- **A named exception worth your attention** — Order of the Sword's *By My
  Honor* (2nd level) carries a genuine `BONUS:SAVE|%LIST|2|TYPE=Morale`,
  landing on a **computed total**, which is rare for this class. But it is a
  double chooser (pick an alignment, then pick which save). Under the Skill
  Focus precedent — never silently seed a choice whose entire value *is* the
  choice — it needs an explicit recorded selection, not a canonical default.
  A real candidate if you want a live-consumer win here, but it is
  mechanism-B work, not a flat fact.

## Build-time hazards

1. **The 6-vs-29 Order count** — scope every Order lookup to APG.
2. **One formula, six different meanings.** `OrderChallengeBonus` is
   `CavalierLVL/4` on all six order records, but each order attaches its own
   `Cavalier ~ Challenge.MOD` giving `1+OrderChallengeBonus` a *completely
   different* referent: Cockatrice = melee damage, Dragon = **allies'**
   attack rolls, Lion = the cavalier's AC, Shield = attack rolls, Star = all
   saving throws, Sword = attack rolls while mounted. Do not ground "the
   order challenge bonus" generically — the number is shared, the semantics
   are not, and five of the six are opponent- or ally-conditioned anyway.
3. **Banner's base value is in the `DESC:` parameters, not the `BONUS:VAR`.**
   `CavalierBannerBonus` is `(CavalierLVL-5)/5`, which is **0 at level 5** —
   the real values are `2+CavalierBannerBonus` and `1+CavalierBannerBonus`,
   supplied as DESC params. Reading only the `BONUS:VAR` yields +0/+0 at
   5th level instead of +2/+1. Exactly the partial-read shape that produced
   the Sacred Weapon dice-count bug. (Moot if Banner stays deferred as
   ally-scoped, but worth recording so it is not re-derived wrong later.)
4. **`Cavalier ~ Bonus Feat` carries three `.MOD` records** each subtracting
   1 from the pool for a specific archetype
   (`TYPE.CavalierCavaliersBonusFeat6/12/18`). Confirm they are provably
   vacuous in this repo's ingested data before grounding the raw
   `CavalierLVL/6`, the same check that cleared Alchemist's Gnome-only and
   Ultimate-Magic-gated Bomb terms.
5. **Expert Trainer and Cavalier's Charge are DESC-only** — no `BONUS`
   token at all. Weaker evidentiary path; verify against RAW as a second
   source, as Swashbuckler's Panache did.
6. **`ALLOWBASECLASS:NO`** sits on Cavalier's class line — an unusual token
   not present on the other APG/ACG classes closed so far. Worth a look
   before assuming the chassis dispatch behaves identically.

## Recommended bounded MVP

1. Class-skill list (7th widening instance) — cheapest, mechanical.
2. Challenge: uses/day pool + the self-applied `-2` AC penalty, reusing
   Bloodrage's own already-grounded AC-penalty path. Damage bonus deferred
   and named.
3. Expert Trainer (`+level/2` Handle Animal).
4. Bonus-feat and teamwork-feat counts.
5. Order narrowed to Order of the Sword: its two class skills + its
   `1/2 level` Sense Motive bonus.

**Honest status expectation:** stays Blocked. Banner, the charge family, the
challenge riders, and 5 of 6 orders remain genuinely deferred, so Cavalier
keeps an `other_features_deferred`-shaped diagnostic — same posture as
Oracle and Witch. `named_features_wired` 1 → roughly 5-6 depending on your
cluster-collapsing call on the two feat-count mechanisms.

## Open questions for the lead

1. **The charge family** — does a flat bonus conditioned on "a mounted
   charge" sit on the groundable side (an activation) or the deferred side
   (an unmodelled context, like Guiding Star's night sky)? Three features
   turn on this, and it will recur for any mounted or charge-based class.
2. **Order of the Sword vs Order of the Star as the canonical.** I picked
   Sword for the cleanest flat self-scoped bonus. Star's is equally flat but
   carries an "as long as the check involves his chosen faith" context
   rider, which I read as weaker under your own Guiding Star line.
3. **Is *By My Honor* worth pulling in** as a mechanism-B explicit-choice
   feature? It is the only Cavalier feature that lands on a computed total,
   which makes it disproportionately valuable — but it is a double chooser
   and genuinely more work than everything else here combined.
