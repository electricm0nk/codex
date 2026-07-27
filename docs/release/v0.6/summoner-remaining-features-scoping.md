# Summoner (#35) — Remaining Features After the Eidolon Chassis

> Surfaced by the corrected roster survey: with the Eidolon MVP landed,
> Summoner's remaining class features are no longer subsystem-blocked. My
> own Eidolon doc deferred them as "hangs off the Eidolon" — under the
> scope-condition test that is wrong for most of them.
>
> **Five are flat level-derived pools needing nothing from the eidolon's
> stat block. Aspect/Greater Aspect is genuinely different and gets its own
> treatment. One hazard is serious enough to gate the build.**

## The five flat pools

All verified against `apg_abilities_class.lst`, and independently
re-verified by the lead:

| feature | formula | unit |
|---|---|---|
| Bond Senses | `BondSensesRounds = classlevel("Summoner")` | rounds/day |
| Maker's Call | `MakersCallTimes = (classlevel("Summoner")-2)/4` | uses/day |
| Merge Forms | `MergeFormsRounds = classlevel("Summoner")` | rounds/day |
| Twin Eidolon | `TwinEidolonMinutes = classlevel("Summoner")` | minutes/day |
| Summon Monster | duration `= classlevel("Summoner")`; uses `= CHA+3`; spell level `= min(9,(classlevel+1)/2)` | rounds / per-day / level |

**None reads any property of the eidolon.** Each is the
Fervor/Panache/Challenge-uses shape this codebase has grounded a dozen
times — a self-scoped quantity whose *effect* is unmodelled but whose
*magnitude* is a verified fact. This is the fourth instance of the same
pattern this segment, after Studied Target's opponent, the familiar's
master benefit, and the Witch spell store.

Summon Monster splits the same way Bomb does: **ground the duration, uses,
and spell-level access; defer the summoning itself.**

## Aspect / Greater Aspect — a point-diversion chooser, not a flat fact

*(Corrected by the lead; my survey described this as a flat `ABILITYPOOL`
of 1, which is wrong.)*

The real mechanic diverts points **from the eidolon's own evolution pool to
the summoner**:

- Aspect: "divert up to **2** points"; Greater Aspect: "up to **6**", with
  "your eidolon loses 1 point for every 2 points (or fraction thereof)".
- Selectable aspects carry real corpus costs:
  `BONUS:VAR|EidolonAspect|{1,1,1,2,2,2,3,3}` — eight options at 1/2/3
  points.
- The eidolon's side is already encoded:
  `FOLLOWER:EidolonAspect=1/2/3 → BONUS:ABILITYPOOL|Eidolon Evolution|-1`
  each, implementing the per-threshold deduction.
- The umbrella records also grant the *summoner* evolution-bearing anatomy:
  `EidMaxAttacks 5` (Aspect) / `6` (Greater), `+1` at Summoner 14 / 19,
  plus `EidArms 1` and `EidLegs 1`.

**The point costs are flat and corpus-real, so this is not blocked the way
the full evolution economy is** — but it is a chooser over a *shared* pool,
not an independent fact. Treat it as its own canonical-narrowing candidate
(pick one aspect, same shape as Oracle's Mysteries), scoped separately from
the five pools above.

## The hazard that gates this build: no grant levels in the corpus

**Summoner's feature grant levels are not encoded where every other class
this segment carried them.** Verified:

- **No `PRE` gate on any of the eleven `KEY:Summoner ~ …` feature records** —
  Bond Senses, Maker's Call, Merge Forms, Twin Eidolon, Aspect, Greater
  Aspect, Summon Monster, Life Link, Shield Ally, Greater Shield Ally,
  Transposition all come back clean.
- **No usable per-level `ABILITY:` rows** on the `CLASS:Summoner` block
  beyond level 1 — unlike Bloodrager/Brawler, whose grant levels I read
  straight off the class table.

**Why this matters concretely:** these formulas compute non-zero at level 1
for features a level-1 summoner does not have. `BondSensesRounds =
classlevel` gives 1 round/day at 1st; Twin Eidolon gives 1 minute/day at
1st; Merge Forms 1 round/day at 1st. RAW grants them at 4th, 20th and 16th.
Building the formulas without gates ships exactly the Shaman
Healer's-Touch failure — a formula that evaluates fine for a feature that
does not exist yet.

**Maker's Call is the one exception and shows the pattern:**
`(classlevel-2)/4` is 0 below 6th, so it self-gates — and its first
non-zero value lands at 6th, matching RAW's own grant level. Where the
formula encodes the gate, trust it; where it doesn't (the other four),
the gate must come from somewhere.

**RESOLVED by the lead, 2026-07-27 — the grant levels ARE in the ingested
corpus.** They are not on the feature's own `KEY:Summoner ~ <Feature>`
record (where this doc's own search looked and correctly found nothing);
they are on a separate set of `.MOD` grant lines attached to
`Summoner ~ Standard Class`, one per feature, each carrying a real
`PREVARGTEQ:Summoner_CFP_Level,<N>` gate — and `Summoner_CFP_Level` itself
resolves via `BONUS:VAR|Summoner_CFP_Level|classlevel("Summoner")`, so it
is a genuine live proxy for the character's real Summoner level, not a
dead variable. Full gate table, read directly off those `.MOD` lines:

| feature | gate |
|---|---|
| Eidolon, Life Link, Summon Monster | level 1 |
| Bond Senses | level 2 |
| Shield Ally | level 4-11 (`PREVARLT:...,12` AND `PREVARGTEQ:...,4`) |
| Maker's Call | level 6 (confirms the formula's own self-gating independently) |
| Transposition | level 8 |
| Aspect | level 10-17 (`PREVARGTEQ:...,10` AND `PREVARLT:...,18`) |
| Greater Shield Ally | level 12 |
| Life Bond | level 14 |
| Merge Forms | level 16 |
| Greater Aspect | level 18 |
| Gate | level 19 |
| Twin Eidolon | level 20 |

Slice A's five members now have real, corpus-confirmed gates: Bond Senses
2, Maker's Call 6 (already self-gating, now doubly confirmed), Merge Forms
16, Twin Eidolon 20, Summon Monster 1 (immediately available, matching its
own signature-ability status). **No independent source needed, no
Swashbuckler-shaped gap here — build with these gates directly.**

## Other build-time hazards

1. **Aspect writes eidolon-named variables onto the *summoner*.**
   `EidMaxAttacks`, `EidArms`, `EidLegs` are set by the Aspect records on
   the master, and they are the *same variable names* the eidolon's own
   chassis uses. If the implementation shares those between master and
   companion, taking an Aspect would silently corrupt the eidolon's own
   max-attacks. Real namespace collision with already-shipped code.
2. **Aspect's pool deduction must now fire.** My Eidolon doc flagged
   "make sure the deduction doesn't fire while Aspect is deferred." With
   Aspect in scope the inverse applies: the eidolon's already-grounded
   `evolution_pool` must reflect the diversion, or the two features will
   disagree.
3. **Maker's Call at levels 1-2** evaluates `(1-2)/4` — a negative operand,
   the same truncation question Swashbuckler's Charmed Life raised. Moot if
   the grant level is resolved (hazard above), but confirm rather than
   assume.
4. **Merge Forms and Twin Eidolon are very high level** (RAW 16th/20th),
   so they will be inert across the level band the tests exercise. Ground
   them honestly rather than pretending coverage.

## Explicitly deferred

Life Link, Shield Ally, Greater Shield Ally, Transposition, Life Bond —
these genuinely depend on the eidolon *as a creature* (its HP, its position,
its ability to act), which is the Tier-2 slice the Eidolon doc deferred and
this pass does not reopen. The full evolution economy stays deferred too.

## Recommendation

**Slice A (build now):** the five flat pools — Bond Senses, Maker's Call,
Merge Forms, Twin Eidolon, Summon Monster's duration/uses/spell-level —
**gated on resolved grant levels**.

**Slice B (follow-on):** Aspect narrowed to one canonical selection, with
the eidolon pool deduction wired and the variable-namespace collision
resolved.

**Honest status expectation:** Summoner stays **Blocked** — the deferred
eidolon-dependent features and the evolution economy keep its diagnostic
alive. `named_features_wired` goes from 1 (Eidolon) to ~6, and Summoner
stops being the roster's thinnest class.

## Open questions for the lead — resolved

1. **Grant levels** — RESOLVED above; real corpus gates found, build Slice
   A now with them.
2. **Slice A and B together, or A first?** RULED: A first. Clean,
   self-contained, fully unblocked. B (Aspect) waits on the namespace
   question below being confirmed against shipped code.
3. **`EidMaxAttacks` collision** — not yet confirmed against the shipped
   `ground_summoner_eidolon` implementation; whoever builds Slice B should
   check this directly before assuming either way, same discipline as
   everything else in this doc.
