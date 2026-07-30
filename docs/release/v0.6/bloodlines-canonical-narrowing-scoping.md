# Bloodlines (#59) — Canonical-Narrowing Scoping

> Fourth and final shared chooser family from the #51 survey. The brief
> asked me to say plainly if it came back thin, and whether the
> shared-chooser vein is exhausted.
>
> **It did not come back thin — the opposite. Bloodlines is by far the
> richest family on the roster, and my own "vein exhausted" call after #57
> was wrong. But two things in my #51 survey were also wrong, and both
> reduce what this is worth.**

## Correction 1: this is the richest family, not the thinnest

Counting records carrying a **non-plumbing** magnitude (excluding the
per-bloodline `…LVL` level-wiring vars, which would otherwise inflate this
badly):

| family | magnitude-bearing records |
|---|---|
| **Bloodlines** | **149** (Sorcerer 112, Bloodrager 37) |
| Arcanist Exploits | 21 |
| Rage Powers | 18 |
| Rogue Talents | 10 |

**Seven times the next richest.** 21 Sorcerer-side bloodlines and 11
Bloodrager-side. My post-#57 read that "each successive family is thinner,
so the vein is exhausted" was drawn from three samples that happened to
descend — an ordering artifact, not a trend. Stating that plainly because I
recommended acting on it.

## Correction 2: these are *parallel* families, not a shared one

My #51 survey listed Bloodlines as "shared by Sorcerer + Bloodrager", which
is what put it in the shared-chooser bucket. **That classification is
wrong.**

- Sorcerer's records are `<Name> Bloodline ~ <Power>`.
- Bloodrager's are `<Name> Bloodrager Bloodline ~ <Power>`.
- **10 bloodline names overlap** (Aberrant, Abyssal, Arcane, Celestial,
  Destined, Draconic, Elemental, Fey, Infernal, Undead) — **but the records
  and the powers behind them are entirely distinct.**

So there is **no two-classes-from-one-build leverage here.** A canonical pick
serves one class. This is the shared-name-is-not-a-shared-thing trap —
the same one I have flagged four times this session — landing on my own
survey's classification. The overlapping *names* are exactly what made it
look shared.

## Correction 3: the narrowing unit is a bloodline, not a power

Unlike Rage Powers (individually selected from a pool), **bloodline powers
arrive as a package**: you choose a bloodline and its whole ladder is
granted automatically (`ABILITY:…|AUTOMATIC|` per power, gated on
`PREABILITY` for the bloodline itself).

So the canonical narrowing unit is **one bloodline and its ~6-power
ladder**, not one power. That makes this a materially bigger build than any
of the previous three families' single-power picks.

## Computed-total landing is rare here

Despite 149 magnitude records, **only 4 land on a total this engine
computes** — all Sorcerer-side, all `PREABILITY`-gated:

| bloodline | power | lands on |
|---|---|---|
| **Draconic** | Dragon Resistances | `BONUS:COMBAT\|AC` (natural armor) |
| Aquatic | Aquatic Adaptation | `BONUS:COMBAT\|AC` |
| Serpentine | Snakeskin | `BONUS:COMBAT\|AC` |
| Abyssal | Strength of the Abyss | `BONUS:STAT\|STR` (ability score) |

The other 145 are energy resistances, reach, blindsight, damage-size and
similar — real magnitudes, but grounding standalone.

## Recommendation: **Sorcerer's Draconic Bloodline**, if this is built

Most iconic of the 21, its Dragon Resistances lands on **computed AC**, and
its ladder is the conventional shape (resistances → natural armor → claws →
breath weapon → wings). Abyssal is the alternative on the strength of
`BONUS:STAT|STR` hitting an ability score, which is arguably a more
load-bearing total than AC.

## Honest assessment — and a recommendation against sequencing it now

**This is the largest remaining seam on the roster and it is genuinely worth
doing eventually.** But it is not the same kind of task as #54/#56/#57:

- **No shared leverage** — one class per build, unlike Rage Powers' two.
- **Bigger unit of work** — a whole bloodline ladder, not one power.
- **Low computed-total density** — 4 of 149.
- **Chooser-in-chooser** — 21 bloodlines each containing ~6 powers, so
  "narrowing" here still leaves 20 bloodlines named and deferred, a much
  larger honest remainder than the other families left behind.

**My recommendation: let #54, #56 and #57 land first, then treat Bloodlines
as its own scoped workstream rather than a fourth quick canonical pick.** It
does not fit the pattern the previous three shared, and squeezing it into
that shape would either understate the work or overstate the coverage.

## On the "is the vein exhausted" question

**No — but the *shared*-chooser vein is.** Of the four families surveyed,
only Rage Powers turned out to be genuinely shared across classes. Arcanist
Exploits is single-class, Rogue Talents is shared but thin, and Bloodlines
is rich but parallel rather than shared. **The seam worth naming is not
"chooser families" but "class-specific ladders", of which Bloodlines is by
far the largest instance.**
