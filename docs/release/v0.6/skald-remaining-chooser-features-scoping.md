# Skald (#48) — Remaining Features, Second Pass

> Supersedes the reasoning in `skald-remaining-features-scoping.md` (task
> #7). **That doc's central open question — strict consumer-existence bar
> vs. the established standalone-magnitude idiom — was resolved in favour of
> the standalone idiom** (task #18, later sharpened as risks item 52). Its
> Bardic Knowledge and Damage Reduction candidates were subsequently
> grounded. Read this one for current state; the older doc is now historical.
>
> **Five flat magnitudes remain, none subsystem-blocked. But the more urgent
> finding is shipped: Skald's Raging Song rounds-per-day does not match the
> corpus formula it derives from — uniformly 2 lower at every level.**

## Shipped-code discrepancy — flag before anything else

| source | formula | at level 1 |
|---|---|---|
| **corpus** (`KEY:Skald ~ Raging Song`) | `SkaldRagingSongRoundsPerDay = 3 + CHA + (2 * SkaldLVL)` | `5 + CHA` |
| **shipped** (`skald_inspired_rage_rounds_per_day`, `:15442`) | `SKALD_RAGING_SONG_BASE_ROUNDS_PER_DAY (= 3) + CHA + 2*(level − 1)` | `3 + CHA` |

Expanding the shipped form: `3 + CHA + 2·level − 2` = **`1 + CHA + 2·level`**
against the corpus's **`3 + CHA + 2·level`** — **a uniform −2 at every
level**, not an edge case. This is the Sacred Weapon shape: a formula
transcribed with a structural error that evaluates cleanly and is wrong
everywhere.

**What I am not claiming: which one is correct.** The shipped
`base + CHA + 2*(level−1)` shape is the standard PF1 "X at 1st, +2 per level
after" idiom, and I have a recollection of the published base value — but
that is exactly the class of recollection that has been wrong repeatedly
this session, and this pool feeds a live budget check
(`rounds_exceeded`).

**Verifiable without any RAW source: the code and the corpus disagree, so at
least one is wrong.** And note a third possibility — if the published table
says `4 + CHA` at 1st, then *both* are off (corpus +1, code −1). **Resolve
all three against one primary source rather than reconciling pairwise.**

## Grounded today (4)

Inspired Rage (`2 + floor(SkaldLVL/8)*2`, with a real ability-modifier
integration), Bardic Knowledge (`max(1, SkaldLVL/2)`), Damage Reduction
(`min((SkaldLVL−4)/5, 3)`), and bounded spontaneous spellcasting (spell
level 1 only).

## Remaining — five flat magnitudes, all confirmed ungrounded *for Skald*

Several of these names collide with other classes' features — Bard has its
own Lore Master and Versatile Performance, Barbarian owns rage powers — so
each was checked with a Skald-scoped filter rather than a bare name grep.
**All four colliding names return zero Skald-scoped hits**; the matches in
`pilot_compute.rs` belong to those other classes.

| feature | formula | shape |
|---|---|---|
| **Well-Versed** | `SkaldWellVersedBonus = 4` | flat `+4`, self-scoped |
| **Spell Kenning** | `(1 + SkaldLVL) / 6` | uses/day pool — zero hits anywhere |
| **Lore Master** | `min((SkaldLVL − 1) / 6, 3)` | uses/day pool |
| **Versatile Performance** | `min((SkaldLVL + 3) / 5)` | count — chooser |
| **Rage Powers** | `RagePowersLVL / 3` | count — **chooser shared with Barbarian** |

**Well-Versed is the cleanest single win** — flat `+4`, no gate, no chooser,
and Bard's identical `BARD_WELL_VERSED_BONUS = 4` is already shipped as a
standalone situational-save magnitude, so the precedent is exact.

**Rage Powers carries cross-class leverage** — the same rage-power list
Barbarian uses, so a canonical narrowing is the two-class kind, like
Investigator's Discovery list sharing with Alchemist.

## No numeric magnitude — correctly deferred

Class Skills, Cantrips (folds into spellcasting, same reasoning as
Arcanist's), Improved Uncanny Dodge, Master Skald, and the four other
Raging Song types — **Song of Marching, Song of Strength, Dirge of Doom,
Song of the Fallen**. Those four are the chooser-shaped family the brief
anticipated; they carry no tokens because their effects are ally-scoped or
non-numeric, consistent with Skald's already-shipped ally-extension deferral
on Damage Reduction.

## Transcription oddity worth resolving before transcribing

`BONUS:ABILITYPOOL|Skald Versatile Performance|min((SkaldLVL+3)/5)` is a
**single-argument `min()`**, which is not meaningful as written. Either the
corpus dropped a second operand (Bard's own equivalent caps at a maximum) or
PCGen tolerates it as a no-op. Do not transcribe it as a cap without
resolving what the second operand should be.

## Recommendation

1. **Resolve the Raging Song discrepancy first** — shipped, wrong on at
   least one side, feeding a live budget check. A fix, not a scoping item.
2. **Well-Versed** — trivial flat `+4` with an exact shipped precedent.
3. **Spell Kenning and Lore Master** — two flat uses/day pools.
4. **Versatile Performance and Rage Powers** — flat counts; Rage Powers best
   done alongside any Barbarian rage-power narrowing.

**Honest expectation:** items 2-4 take Skald from 4 grounded features to
~8-9. It stays Blocked on the four tokenless Raging Song types and the
bounded spellcasting.

## Open question

**Only the Raging Song formula.** Three candidate values are in play at
level 1 — `3+CHA` (shipped), `5+CHA` (corpus), and whatever the published
table says. It needs a primary source rather than either of us reasoning
from the two disagreeing artifacts.
