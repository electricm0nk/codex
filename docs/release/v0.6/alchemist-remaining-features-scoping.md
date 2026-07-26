# Alchemist (#4) — Fresh Scope (post-Investigator leverage + corrected bar)

> Directed by the lead: Alchemist was assessed "not cheap" back when the
> Alchemist formula-list ingestion itself was the blocker (no list existed).
> Investigator (#8) has since built the shared `alchemist_spell_list`
> module (the 104 real `Alchemist=N` records), so that specific cost is
> gone. Re-scope Alchemist fresh against current state, not the old
> assumption: (1) confirm its real casting shape and whether spellcasting
> is now a big lift or just a validation function reusing the shared list,
> (2) check each remaining feature against the corrected standalone-
> grounding bar, (3) give an honest boundedness read. **Result: Alchemist
> flips from "too big" to a genuinely tractable, even meaty slice —
> Bomb is fully groundable, Poison Resistance is a shared helper, and
> spellcasting is now a modest reuse of Investigator's own machinery.**

## Casting shape — SAME as Investigator, not a different mechanic (task-note correction)

The task note frames Alchemist's casting as "formula-based … different
mechanic from Investigator's prepared shape." Verified against the corpus,
that's imprecise: **Alchemist is `HD:8 SPELLSTAT:INT MEMORIZE:YES
SPELLBOOK:YES` — byte-identical spellcasting shape to Investigator**, and
Investigator's whole spellcasting closure *reused Alchemist's own formula
list* (`SPELLLIST:1|Alchemist`). Extracts ARE the "formula" mechanic, and
Investigator already grounds exactly that mechanic:
- The shared `rules_tables::apg::alchemist_spell_list` module exists
  (`ALCHEMIST_SPELL_LIST` / `alchemist_spell_level`), and Investigator's
  validation already calls it (`parse_investigator_extract_id` →
  `alchemist_spell_level`, `pilot_compute.rs:11100`).
- So Alchemist's spellcasting is **not a new mechanic and not a new list** —
  it reuses Investigator's prepared-extract validation shape directly on
  the same list. The only Alchemist-specific piece is its own per-level
  extract-count table (Alchemist casts a bit more than Investigator), which
  — like Investigator/Arcanist/Warpriest — has **zero `CAST:` rows in the
  corpus** (verified), so it's an external-source transcription. Modest.
- **Net: spellcasting drops from "the blocker" to a modest wiring lift.**
  The old "not cheap" assessment is obsolete.

## Feature-by-feature under the corrected bar

### Bomb — fully groundable, the marquee win

The signature Alchemist feature decomposes into flat, self-contained,
verifiable magnitudes (all verified in `apg_abilities_class.lst`):
- **Damage dice**: `AlchemistBombAdditionalDice|(AlchemistBombLVL-1)/2`
  over a base 1 die of `AlchemistBombDiceSize|6` → **`1 + (level-1)/2` d6**
  (1d6 at 1st, 2d6 at 3rd, +1d6 per two levels), plus
  `AlchemistBombDamageBonus|INT`. Grounds standalone exactly like Sacred
  Weapon dice / the Wolf-companion bite damage — a weapon-like damage
  magnitude, and crucially **not opponent-dependent** (it's the bomb's own
  damage vs any target, not a bonus conditioned on a "studied" enemy — so
  it's on the groundable side of the Studied-Combat line).
- **Save DC**: `AlchemistBombDC|10+(AlchemistBombLVL/2)+INT` — a flat DC
  magnitude, groundable standalone (Blessing-DC / Mutagen-DC idiom).
- **Uses/day**: `AlchemistBombTimes|AlchemistBombLVL+INT` — a self-
  contained pool quantity, groundable standalone (Panache / Blessing-uses
  idiom).
- **Groundable in full** as standalone facts — no consumer required. This
  is the biggest single win in Alchemist's remaining scope.

### Poison Resistance — groundable, SHARED with Investigator (re-derived, not assumed)

- Re-derived directly (not carried over from Investigator's numbers):
  `AlchemistPoisonLVL` increments at `PREVARGTEQ:AlchemistLVL,2/5/8`, with
  the level-10 gate flipping to full immunity — i.e. **None <2, +2 (2–4),
  +4 (5–7), +6 (8–9), immunity at 10+**. This is **identical to
  Investigator's Poison Resistance** (same tiers), so the shared-helper
  idea holds literally: one situational-save standalone-magnitude helper
  serves both classes. Groundable.

### Mutagen — already grounded (original closure)

The +4 physical / −2 mental / +2 natural-armor mutagen is already wired
from the first Alchemist closure. No new work.

### Discovery — chooser-list (narrow to one canonical, or defer)

The Alchemist Discovery list is a real chooser-list (the Rogue-Talent /
Rage-Power idiom). Some discoveries are flat self-scoped magnitudes (e.g.
elemental-bomb damage-type swaps, Precise Bombs' splash exclusion) and
could be narrowed to one canonical pick the same way Oracle's Mystery /
Hunter's Animal Focus were; the rest defer. **Not a blocker either way.**
(Exact per-discovery magnitudes to be re-derived at narrowing time, not
assumed.)

### Swift Alchemy / Swift Poisoning — correctly deferred (no magnitude)

Both verified to carry **no numeric `BONUS`** — Swift Alchemy (craft in
half time) and Swift Poisoning (apply poison as a swift action) are pure
action/time modifiers with no magnitude to ground, even standalone. Same
honest bucket as Hunter's Nature Training / Investigator's own Swift
Alchemy. Correctly deferred, for the "no magnitude exists" reason.

## Honest boundedness read

**Alchemist is now genuinely tractable — a real multi-feature slice, not
"too big."** The re-assessment vs the old verdict:
- **Spellcasting**: was the blocker (a 100+-record list ingestion); now a
  modest reuse of the shared `alchemist_spell_list` + Investigator's
  prepared-extract validation, plus Alchemist's own external-source
  extract-count table. **The single biggest cost reduction.**
- **Bomb**: fully groundable as standalone facts (damage dice + DC +
  uses/day) — a marquee win the old assessment never credited.
- **Poison Resistance**: groundable, and a shared helper with Investigator.
- **Mutagen**: already done.
- **Discovery**: narrow-or-defer chooser (non-blocking).
- **Swift Alchemy / Swift Poisoning**: honest no-magnitude defers.

So the current-state scope is: Bomb (marquee, groundable) + Poison
Resistance (shared, groundable) + spellcasting (now modest, reuse-backed),
on top of the already-grounded Mutagen — comparable to Investigator's own
just-completed slice, and squarely in the tractable range. The "not cheap"
label was entirely an artifact of the pre-Investigator missing list.

## Open questions for the lead

1. Greenlight Alchemist as a real slice now — **Bomb + Poison Resistance
   (shared helper) + spellcasting (reuse Investigator's validation)** — with
   Discovery narrowed-or-deferred and Swift Alchemy/Swift Poisoning
   honestly deferred? This is a meatier-than-expected but bounded closure.
2. Sequencing for the **shared Poison Resistance helper**: factor it out so
   Investigator and Alchemist share one implementation (Investigator's is
   already built — Alchemist could reuse it directly since the tiers are
   identical), or let Alchemist ground its own parallel record?
3. Bomb scope: ground the full trio (damage dice + DC + uses/day) this
   slice, or is the uses/day pool + DC enough for a first pass with the
   damage-dice progression as a follow-on (it's the one piece that scales
   across all 20 levels and wants careful per-level verification, à la the
   Sacred Weapon dice-count lesson)?
