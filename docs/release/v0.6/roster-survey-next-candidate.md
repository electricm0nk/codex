# Roster Survey (#40) — Next Scoping Candidate

> Run with the corrected enumerate-the-prefixes discipline (item 60), and
> with every count re-derived from the corpus rather than from any status
> cell's prose.
>
> **Top candidate: Bloodrager. It has exactly ONE real grounded feature —
> the thinnest coverage on the 27-class roster — and roughly six remaining
> features that are all flat, all with known grant levels, and two of which
> are tier extensions of a mechanism that already ships.**

## Method note — the enumeration found a fourth prefix

Enumerated fresh rather than reusing my own prior list:

```
class_feature 687 | class_chassis 349 | class_spell 117 | class 12
```

**`class.<name>.…` is a fourth class-bearing prefix** (e.g.
`class.barbarian.bounded_progression`, `class.fighter.level_1_pilot`) —
progression/burden identity records rather than feature magnitudes, so it is
counted separately below and excluded from the feature tally deliberately,
not silently. (`chassis.ability_modifiers.*` is global, not class-scoped.)

This is why the rule has to be *enumerate*, not *name a list*: the
previously-corrected "three prefixes" would already have been one short.

## Grounded-feature counts, all prefixes

Lowest coverage first (diagnostics like `unsupported` /
`*_deferred` included in the raw count, so the real gap is *worse* than it
looks for the leaders):

| class | feature ids | of which real features |
|---|---|---|
| **Bloodrager** | 5 | **1** — `bloodrage_execution` (rest are diagnostics + a level-gate note) |
| Inquisitor | 6 | 4 |
| Hunter | 6 | 3 |
| Summoner | 8 | 8 (Slice A landed) |
| Arcanist / Skald | 9 | 8 / 5 |

Everything from Witch (11) upward is comfortably covered; Ranger tops out at
60.

## #1 — BLOODRAGER. Recommended.

One grounded feature. Every remaining item is flat, and **all grant levels
are already verified** from the class table (levels 1/2/3/5/7/11/14/17/20):

| feature | level | formula | note |
|---|---|---|---|
| Fast Movement | 1 | `BloodrageMovementBonus = 10` → **+10 ft walk** | `BONUS:MOVEADD\|TYPE=Walk`, encumbrance-gated |
| Blood Sanctuary | 3 | `BloodragerBloodSanctuaryBonus = 2` → **+2** | flat |
| Uncanny Dodge | 2 | `UncannyDodgeFlankingLevel = BloodragerLVL` | flat, level-derived |
| Improved Uncanny Dodge | 5 | same var, level-5 gate | flat |
| **Greater Bloodrage** | 11 | **+2 Str, +2 Con, +1 save** | **tier extension of shipped Bloodrage** |
| **Mighty Bloodrage** | 20 | **+2 Str, +2 Con, +1 save** | **second tier extension** |
| Damage Reduction | 7 | `(BloodragerLVL-4)/3` → 1/2/3/4/5 at 7/10/13/16/19 | **already task #39** |
| Standard Bloodline | 1 | `BONUS:ABILITYPOOL\|Bloodline\|1` | 10-bloodline chooser, narrowable |

**Why this is unusually cheap:** Greater and Mighty Bloodrage are not new
mechanisms. `bloodrage_execution` already grounds the base tier (+4 Str,
+4 Con, −2 AC, +2 saves); these two add `+2/+2/+1` each on top, taking a
20th-level bloodrager to +8/+8/+4. That is **widening a shipped formula, not
writing one** — the same shape as the incremental level-table widenings this
codebase does routinely.

**Not subsystem-blocked.** Nothing here needs an absent engine: the
movement, save and ability magnitudes are all self-scoped flat values, and
the DR has three in-codebase precedents (Barbarian, Skald, Fighter).

**Correctly stays deferred:** Indomitable Will and Tireless Bloodrage carry
**zero numeric tokens** — verified directly, the Nature Training / Awesome
Blow family. Name them, don't ground them.

## #2 — HUNTER. Strong runner-up, different shape.

3 real features grounded (Animal Companion, Animal Focus, Wild Empathy).
Its main remaining item is **spellcasting, which is now reuse-backed rather
than a build**: Hunter draws on the Druid and Ranger lists, and **both were
corrected and expanded this session** (#26 Ranger 51→114, #29 Druid
169→271). So the lift is wiring against two freshly-verified lists rather
than ingesting anything.

Ranked second only because Bloodrager's win is broader (six features vs
essentially one) and needs no list plumbing.

## #3 — INQUISITOR / SKALD. Thin remainders.

Both sit at 4-5 real grounded features with small, largely chooser-shaped
remainders. Real but low-yield; not worth displacing either candidate above.

## What this survey does *not* claim

- Counts of *ids* are not counts of *features*; I separated diagnostics by
  hand for the leaders only, so the mid-table numbers are indicative rather
  than exact.
- Chooser families (bloodlines, judgments, focus options) are keyed
  `<Name> <Class> Bloodline ~ <Power>` and similar, so a
  `KEY:<Class> ~` sweep under-counts them by construction. That affects
  ranking only if a chooser-heavy class were close to the top; none is.

## Recommendation

**Scope Bloodrager next.** Suggested shape: one slice covering Fast
Movement, Blood Sanctuary, both Uncanny Dodge tiers, and the Greater/Mighty
Bloodrage tier extensions — with DR left to the already-filed #39 and the
bloodline chooser as an optional canonical narrowing.

Expected outcome: Bloodrager goes from **1 real grounded feature to ~7**,
and stops being the roster's thinnest class. It stays Blocked (bloodlines,
Indomitable Will, Tireless Bloodrage), same posture as its peers.
