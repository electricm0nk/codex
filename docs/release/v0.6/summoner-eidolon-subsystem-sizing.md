# Summoner Eidolon — Subsystem Sizing (Background Research for a Future Decision)

> Directed by the lead after Summoner was held as genuinely Eidolon-
> blocked: before the "build the Eidolon subsystem or declare full-build
> closures done" call comes up, have real sizing information ready — read
> the actual PF1 Eidolon rules from the corpus, compare its shape against
> the Wolf companion stat block that already exists, and give a concrete
> effort estimate relative to the closures already landed, not a vague
> guess. This is background research, not a build brief; no MVP is
> proposed for immediate execution.

## What the Eidolon actually is (verified against the APG corpus)

The Eidolon is **not a class ability** — it is a full creature-shaped
subsystem, represented in the corpus as three stacked layers:

1. **A leveled monster class** — `CLASS:Eidolon HD:10 TYPE:Monster.Companion
   MAXLEVEL:20` (`apg_classes.lst:207`). Full BAB (`ClassBABFull =
   classlevel`), and — unlike the Wolf's fixed good/poor saves — its save
   progression is **configurable per base form** via `EidolonFortGood`/
   `EidolonReflexGood`/`EidolonWillGood` flags (each save is good or poor
   depending on which flag the chosen base form sets). HD scales with the
   owning Summoner's level.
2. **A base creature race** — `Eidolon` in `apg_races_companion.lst:7`
   (`MONSTERCLASS:Eidolon:1`, `HANDS:7`, base `AC_Natural_Armor 2`,
   `Con +2 Int -4`) plus a follower-scaling harness in
   `apg_companionmods.lst` (`FOLLOWER:EidolonCompanionLVL`,
   `EidolonCompanionBonusHD`, and evolution-pool `BONUS:ABILITYPOOL`).
3. **A base-form choice** — three forms in `apg_abilities_companion.lst`
   (`KEY:Eidolon Type ~ {Biped,Quadruped,Serpentine}`), each setting
   ability bonuses, which saves are good, base natural attacks, movement,
   and a set of **fixed AUTOMATIC free evolutions**. Verified:
   - **Biped**: `STAT|STR|6`, `STAT|DEX|2`, good Fort+Will, free
     evolutions `Arms` / `Clawed Hands` / `Legs` (granted `AUTOMATIC`,
     i.e. fixed, not chosen), base 2 claws.
   - **Quadruped**: `STAT|STR|4`, `STAT|DEX|4`, good Fort+Reflex, base
     bite.
4. **The evolution point-buy system** — the marquee mechanic and the
   dominant cost. **104 real `KEY:Evolution ~ ...` records** verified
   (`grep -oE "KEY:Evolution ~ [^\t]+" apg_abilities_companion.lst |
   sort -u | wc -l` → 104, raw count also 104, no dupes; the only file in
   the APG corpus carrying them). A real point-buy economy: each evolution
   has a point cost (1–4), most carry base-form restrictions and/or
   minimum Summoner-level or minimum-evolution prereqs, and many actually
   mutate the stat block (extra attacks, ability boosts, natural armor,
   extra movement modes, energy attacks, breath weapons, DR, reach, etc.).
   The pool that funds it scales per Summoner level via a real formula
   (`BONUS:VAR|EidolonEvolution|3+(SummonerLVL>=2)+(SummonerLVL>=3)+…`),
   with a separate max-attacks cap. (There is also a parallel set of ~102
   `KEY:Temp Evolution ~ ...` records — the temporary-evolution variants
   used by the Aspect/Greater Aspect and evolution-surge mechanics — plus
   the ability-category/choice scaffolding `TYPE:…Evolution` records; none
   of those are additional base evolutions. An earlier draft's "353" count
   was wrong: it came from a broad pattern that swept the Temp Evolution
   duplicates and the category TYPE lines in with the real evolutions. The
   correct base-evolution count is 104.)

Supporting linkage records (`KEY:Eidolon ~ Link`, `~ Share Spells`,
`~ Skills`, and the Summoner-side Life Link / Bond Senses / Aspect /
Shield Ally) sit on top of this.

## The existing Wolf companion stat block, as the reuse baseline

`ground_wolf_companion_stat_block` (`pilot_compute.rs:4835`, reused by
Druid Nature Bond and Hunter) is the honest reference point for
"companion stat block work already done." Its actual shape (read
directly):

- **One fixed creature** (Wolf), base ability scores as constants
  (`WOLF_COMPANION_STRENGTH_SCORE`, etc.), corpus-verified.
- **~7 derived standalone explanation records**: base attack bonus
  (`HD*3/4 + Str`), Fort/Reflex (`HD/2+2`), Will (`HD/3`), AC
  (`10 + natural armor`), bite (`1d6 + 1.5×Str`), HP (max first HD +
  average second + Con). Scales by companion HD only.
- **No choices, no branches, no per-instance variety** — one species,
  one advancement axis. Roughly 120 lines including the verification-
  citation prose. A known, landed, moderate quantity.

## The size comparison — where the Eidolon is and isn't bigger

The Eidolon is **meaningfully bigger than the Wolf overall**, but the
excess is almost entirely concentrated in the evolution point-buy layer.
Decomposed:

| Layer | vs. Wolf work | Deferrable? |
|---|---|---|
| Base monster-class stat table (BAB/saves/HD/natural armor) | ~Wolf-sized, plus form-configurable saves (a small branch) | No — it's the core |
| One canonical base form (Biped): fixed Str/Dex, good-save set, base attacks, 3 AUTOMATIC free evolutions | Modest increment (fixed facts, not choices) | No — but bounded |
| Evolution **pool size** (flat per-level formula) | Trivial — a single flat fact, exactly the Summon-Monster-pool idiom | Groundable as a flat fact; spending deferred |
| Evolution **point-buy** (104 records, point economy, prereqs, form/level gates, stat-block mutation, max-attacks cap) | **Bigger than any chooser-list a closure has deferred** — a real point-buy selection engine, though a bounded/countable one | Yes — defer wholesale under a narrowed diagnostic |
| All-forms generality + Link/Share Spells/Aspect/etc. | Incremental each | Yes — defer |

**The key finding: the Eidolon is NOT strictly all-or-nothing.** A bounded
"one canonical base form, base statistics + its fixed free evolutions,
evolution-pool-size as a flat fact, zero player-chosen evolutions" MVP is
honest and groundable — the base form's free evolutions are `AUTOMATIC`
(fixed, not chosen), so grounding them is not fabrication, and the
chosen-evolution economy defers cleanly under the same chooser-list-defer
idiom every closure already uses. That MVP would deliver a real Eidolon
companion with real base stats — genuine Summoner value, and independent
of (and worth more than) the thin Summon-Monster-pool-only fact.

Caveat on that MVP's honesty: a zero-evolution Eidolon is a deliberately
*base* creature. The base-form free evolutions must be grounded (else it's
an incomplete creature), and the deferred-spending diagnostic must name
the evolution pool as unspent so the output never implies a fully-built
Eidolon. This is the same honest-partial posture as Hunter's companion
(which grounds the level-1 stat block and defers advancement past HD 1).

## Does an Eidolon MVP make the Summon-Monster fact more useful?

No — they are independent mechanics. The Summoner's Summon Monster SLA
(`SummonMonsterTimes|CHA+3` uses/day) and the Eidolon share nothing; an
Eidolon MVP does not touch Summon Monster. But that reframes the value
question favorably: the Eidolon base-form MVP is itself the higher-value
Summoner content (its marquee feature), so it is worth pursuing on its own
merits, not as a way to prop up the Summon-Monster pool. The earlier
"Summoner has no cheap MVP" conclusion was about a *flat, self-scoped,
subsystem-free* power (which Summoner genuinely lacks, unlike Shaman/Witch)
— it does not rule out a bounded *companion-stat-block* MVP, which is a
different and larger shape.

## Effort estimate, relative to landed work

- **Bounded Eidolon MVP** (one base form, base stat block + fixed free
  evolutions + pool-size fact, chosen evolutions deferred): **roughly
  Wolf-companion-sized plus a base-form branch — comparable to a single
  landed closure, at the larger end (call it Warpriest-ish), not a
  multi-slice project.** The companion-stat-block pattern is already
  proven twice (Druid, Hunter), so this reuses a known shape; the new work
  is the form-configurable saves and one form's fixed freebies.
- **Full Eidolon** (the evolution point-buy system, all forms, links):
  **bigger than a single closure — a standalone subsystem, not a closure —
  but a bounded one.** The 104-record evolution economy is a real
  point-buy engine: unlike a flat chooser-list a closure just defers with
  grant-only records (Slayer Talents, Witch's other hexes), each evolution
  actually *mutates the stat block* (extra attacks, ability boosts, natural
  armor, energy attacks, breath weapons, …) and carries point costs,
  base-form restrictions, and level/prereq gates that a point economy must
  enforce. That combination — a countable 104-entry list plus a
  cost/prereq economy plus stat-block application — sits **above a normal
  closure's chooser-defer but below a from-scratch spellcasting system**;
  104 is a mid-size list (comparable to Investigator's own ~95 KEY
  records), so this is a substantial-but-scopeable project, not an
  open-ended one. This is the part that warrants the lead's "deliberate
  subsystem investment" framing — just not an unbounded one.

## Open questions for the lead

1. Is a **bounded base-form-only Eidolon MVP** (Warpriest-ish size,
   honest, defers the evolution economy) worth a slice — giving Summoner
   its first real content — or should Summoner stay untouched until (if
   ever) the full evolution subsystem is funded?
2. If the MVP is of interest: **Biped** is the natural canonical base form
   (most iconic, cleanest free-evolution set), mirroring the Wolf-as-
   canonical-companion pick — agreed, or a different form preferred?
3. The full evolution point-buy system (104 records) is the real
   subsystem cost. Is that a project you'd want scoped separately later
   (the way a Familiar subsystem would be), or is it firmly out of scope
   for this epic — in which case the base-form MVP is the *ceiling* of
   Summoner value, not a first step toward a fuller build?
