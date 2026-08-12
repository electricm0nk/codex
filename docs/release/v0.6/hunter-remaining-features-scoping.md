# Hunter (#2) — Remaining-Features Scoping (corrected standalone-grounding bar)

> Directed by the lead, next in the approved Skald → Hunter → Investigator
> sequence: scope Hunter's remaining features (spellcasting, Wild Empathy,
> Animal Focus, Nature Training) under the **corrected canonical bar** just
> established — a feature grounds as a standalone record whenever its
> magnitude is real and verifiable, no live consumer total required (the
> Bard Bardic Knowledge / Slayer Track precedent). The lead specifically
> asked to (a) re-check Wild Empathy holds, (b) take a fresh look at Animal
> Focus and Nature Training under the corrected bar (my prior pass deferred
> them), and (c) re-confirm the spellcasting reuse claim directly against
> the corpus. All three done; two real corrections to my prior pass.

## Wild Empathy — flat standalone, groundable (holds)

- Corpus: `BONUS:VAR|HunterWildEmpatyBonus|CHA+HunterLVL` (a CHA + level
  check bonus to influence animals). Flat, self-contained magnitude.
- Under the corrected bar it grounds cleanly as a standalone record (no
  "wild empathy" total needed), the same idiom as Bard Bardic Knowledge /
  Slayer Track. **Groundable, cheap.** Holds from the prior pass.

## Animal Focus — CORRECTION: groundable via canonical narrowing (prior pass wrongly deferred it)

My prior pass deferred Animal Focus as "a chooser-list." Under the
corrected bar that was wrong — **each focus option is a flat, self-scoped,
verifiable magnitude**, so it grounds exactly like Oracle's Mystery
(pick Life → Healing Hands) and Shaman's Spirit narrowing: recognize one
canonical focus, ground its flat value, defer the other twelve.

- 13 options verified (`KEY:Hunter Animal Focus ~ …`): Bat, Bear, Bull,
  Falcon, Frog, Monkey, Mouse, Owl, Snake, Stag, Tiger, Wolf, plus a
  "No Ability" sentinel. Each is a flat self-scoped bonus, e.g.:
  - **Bull** — `BONUS:STAT|STR|HunterAnimalFocusBullBonus` = **+2**, rising
    to +4 at level 8 and +6 at level 15 (`PREVARGTEQ:…LVL,8/15`), a STR
    enhancement bonus.
  - **Tiger** — `BONUS:STAT|DEX|HunterAnimalFocusTigerBonus`, same +2/+4/+6
    scaling, a DEX enhancement bonus.
  - Owl (+Stealth), Falcon (+Perception), Bat (+60 ft low-light sight),
    Snake (+2), etc.
- **Recommended canonical pick: Bull (+STR) or Tiger (+DEX)** — a clean,
  flat, self-scoped ability-score enhancement bonus (+2/+4/+6 by level 1/8/
  15), the cleanest magnitude of the set, and one that even has a live
  consumer (the ability modifier feeds everything) though the corrected bar
  no longer requires that. Owl/Falcon (skill bonuses) also ground fine
  standalone but land on uncomputed skills, so Bull/Tiger read cleaner.
- **Build-time note:** verify the +2/+4/+6 scaling from the focus's own
  record (`2` + `PREVARGTEQ:…LVL,8` + `PREVARGTEQ:…LVL,15`), and note Animal
  Focus can be applied to the Hunter *or* the companion — ground the
  self-applied case. `named_features_wired` gains 1 (the canonical focus).

## Nature Training — correctly deferred, but for the RIGHT reason (no magnitude exists)

- Corpus: **no numeric `BONUS` of any kind** (verified — empty). Nature
  Training only sets feat/option-qualification flags
  (`FighterWeaponQualify`-style: counts as a Druid/Ranger of Hunter level
  for prerequisites). There is **no magnitude to ground** — nothing to
  emit even as a standalone record.
- So it stays deferred, but the reason matters under the corrected bar:
  not "no live consumer" (which is no longer disqualifying), but "no
  verifiable magnitude exists at all." Same honest bucket as Alchemist's
  Martial Training / Investigator's own feat-prereq no-ops — a
  qualification flag with no numeric effect and no hook. **Correctly
  deferred; confirmed, not a Bardic-Knowledge-style missed win.**

## Spellcasting — reuse claim confirmed WITH one real caveat

Re-verified directly against the corpus (`CLASS:Hunter` record):
- `SPELLSTAT:WIS`, **`MEMORIZE:NO`** (spontaneous, not prepared),
  `SPELLLIST:2|Druid|Ranger`. **The spell-LIST reuse is genuine and
  confirmed** — both `druid_spell_list.rs` and `ranger_spell_list.rs`
  exist and are built, so *which spells* Hunter can cast reuses two
  already-built modules directly.
- The **spontaneous known-spell machinery already exists to reuse** —
  `sorcerer_spells_known_table` / `ground_sorcerer_known_spells`
  (`:21555/:21666`), `bard_spells_known_table`, `oracle_spells_known_table`
  — Hunter mirrors that spontaneous-known shape (like Skald did with Bard),
  not the prepared shape.
- **Caveat (flag, don't assume):** Hunter's class block has **zero
  per-level `CAST:`/`KNOWN:` rows** in the corpus (verified). So the
  per-level *known-count / spells-per-day table* is **not corpus-derivable**
  — it is an external-source transcription, exactly the caveat I flagged
  for Arcanist and Warpriest. The reuse is of the spell *list content*, not
  the slot table. Hunter is a **6-level caster** (max 6th-level spells,
  like Ranger), so the table is smaller than a 9-level caster's.
- One Hunter-specific detail: `KNOWNSPELLS:Summon Nature's Ally I|…` — a
  Hunter automatically knows the full Summon Nature's Ally line as bonus
  known spells (a fixed auto-known set on top of chosen known spells);
  worth grounding as part of the known set.
- **Net:** a real, bounded spellcasting lift — genuine list reuse +
  existing spontaneous-known machinery, offset by an external-source slot
  table. Comparable to Skald's spellcasting closure in shape, a bit larger
  (two lists, 6 spell levels vs Skald's Bard reuse).

## Net sizing (corrected)

Hunter is a **meatier slice than Skald** — and the corrected bar upgraded
Animal Focus from "defer" to "groundable":
- **2 cheap standalone/narrowed features** — Wild Empathy (flat) + Animal
  Focus (one canonical focus, e.g. Bull/Tiger +2/+4/+6).
- **Plus a real spellcasting pillar** with genuine Druid+Ranger list reuse
  and existing spontaneous-known machinery (external slot-table caveat).
- Nature Training correctly deferred (no magnitude).

This reinforces the open question from the Skald pass: **Hunter delivers
more than Skald** (two flat features *and* a full spellcasting pillar vs
Skald's 2–3 flat/pool facts), so the Skald-then-Hunter order may be worth
flipping if backend wants the bigger-value slice first.

## Open questions for the lead

1. **Canonical Animal Focus pick** — Bull (+STR) or Tiger (+DEX) as the
   grounded focus (cleanest flat self-scoped magnitude), or a different one
   (all 12 real options are structurally similar; Bull/Tiger picked for the
   cleanest ability-enhancement value)?
2. **Spellcasting scope** — ground the full spontaneous-known pillar this
   slice (reusing Druid+Ranger lists + Sorcerer/Oracle known-spell
   machinery, accepting the external-source slot table like Arcanist/
   Warpriest), or split spellcasting into its own follow-on and land Wild
   Empathy + Animal Focus first as a cheap slice (the Skald-spellcasting-
   split precedent)?
3. **Ordering** — given Hunter is meatier than Skald, keep Skald→Hunter, or
   let backend take Hunter first for the bigger value? (Both are scoped and
   ready.)
