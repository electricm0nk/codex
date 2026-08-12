# Third Comparative Scoping Pass — the 5 Remaining Untouched Classes (9th ACG/APG Closure Target)

> Directed by the lead: "your own read that they're all genuinely harder
> still holds... your call whether to scope one of those next or hold
> here." Ran the same corpus-verification discipline as the two prior
> comparative passes (before Arcanist, before Slayer) rather than trusting
> the prior "all confirmed harder" label at face value — and found a real
> correction, the same kind of self-caught error the Shaman/Familiar
> finding already established: **Swashbuckler is NOT the same cost class
> as Investigator/Shaman/Summoner/Witch.** The original "harder" framing
> was accurate for 4 of the 5 (Familiar/Eidolon subsystem gaps, or a real
> new spell-list ingestion cost) but not for Swashbuckler, which was
> lumped in only because its own marquee mechanic (Deeds) is "a real
> chooser-list" — true, but Deeds staying deferred (like Slayer Talents)
> doesn't block the same cheap "one real feature, defer the rest" MVP
> shape every other partial closure this segment already uses.

## Corpus findings (verified against `acg_classes.lst` / `acg_abilities_class.lst`)

### Swashbuckler — the real recommendation, corrected from "harder" to "cheapest remaining"

- **Chassis**: full BAB, good Reflex, poor Fortitude/Will. HD 10. **No
  `SPELLSTAT` token at all** — confirmed directly, a non-caster exactly
  like Slayer/Cavalier/Brawler/Hunter. Zero spellcasting scope, the
  cheapest structural shape on the whole roster.
- **29 real `KEY:Swashbuckler ~ ...` records** (verified via
  `grep -oE "KEY:Swashbuckler ~ [^\t]+" acg_abilities_class.lst | sort -u | wc -l`
  → 29, matching the already-recorded `named_features_expected` in
  `rules_tables::acg::mod`).
- **Class skills**:
  `CSKILL:Acrobatics|Bluff|Climb|TYPE=Craft|Diplomacy|Escape Artist|Intimidate|Knowledge (Local)|Knowledge (Nobility)|Perception|TYPE=Perform|TYPE=Profession|Ride|Sense Motive|Sleight of Hand|Swim`
  — includes **all three** of Climb/Intimidate/Swim, the same clean
  binary "all three" shape Warpriest/Slayer already needed (not
  Investigator's messier partial-match case, see below). Needs the same
  now-familiar widening to `selected_skill_class_skill_bonus_applies`,
  the fourth instance of this exact bug.
- **Panache** (`KEY:Swashbuckler ~ Panache`): "a swashbuckler gains a
  number of panache points equal to her Charisma modifier (minimum 1)"
  — `BONUS:VAR|Panache_Cap|max(1,CHA)` (need to confirm exact tag name
  at implementation time, description text confirms the formula). Flat,
  choice-free, always-on daily resource, mirroring Cleric's Channel
  Energy / Warpriest's Blessing-uses-per-day "no gate" shape exactly —
  no `class_ability_activations` entry needed for the pool size itself.
- **Charmed Life** (`KEY:Swashbuckler ~ Charmed Life`): "X times per day
  ... she can add [CHA] to the result of a saving throw" —
  `BONUS:VAR|SwashbucklerCharmedLifeBonus|CHA`,
  `BONUS:VAR|SwashbucklerCharmedLifeTimes|((SwashbucklerLVL-2)/4)+3`.
  An activation-gated save bonus with a real per-day budget — the same
  Rage/Judgment shape this session's own
  `activation-gated-budget-enforcement-two-spot-fix` memory covers
  (ground_or_block + a separate active_*_bonus helper, both re-checking
  the budget). **Open verification item, not yet resolved**: the
  `((SwashbucklerLVL-2)/4)` term is negative at level 1
  (`(1-2)/4 = -1/4`) — PCGen's own integer-division truncation direction
  for negative operands needs to be confirmed directly (floor vs.
  truncate-toward-zero) before coding the formula, not assumed either way.
- **Nimble** (`KEY:Swashbuckler ~ Nimble`): "+X dodge bonus to AC while
  wearing light or no armor" — `BONUS:VAR|SwashbucklerDodgeBonus|(SwashbucklerLVL+1)/4`.
  Confirmed via `grep` that this codebase computes **no player AC total
  anywhere** (matches `risks-and-open-questions.md` item 1's own
  documented AC/attack-bonus architecture gap) — grounds as a standalone
  flat record, the same idiom as Trapfinding/Track/Destructive Attacks.
- **Swashbuckler Finesse**: grants the Weapon Finesse feat's effect via
  `SERVESAS:ABILITY=FEAT` plus a CHA-for-INT feat-prerequisite
  substitution — a real mechanic, but this codebase has no feat-prereq
  substitution surface to hook it into cleanly; candidate for staying
  deferred alongside Deeds, named honestly rather than half-built.
- **Deeds** (the class's own marquee choice list — Derring-Do, Dodging
  Panache, Menacing Swordplay, Precise Strike, and the rest): a real
  chooser-list, the same shape Slayer Talents/Investigator Talents stay
  deferred under. Not attempted this slice.
- **Proposed MVP scope**: gate widening
  (`is_supported_swashbuckler_single_class`), Panache pool size (flat),
  Charmed Life uses-per-day/bonus (activation-gated, two-spot budget
  enforcement), Nimble's dodge bonus (flat, standalone), the class-skill-
  bonus widening fix (fourth instance). Deeds, Swashbuckler Finesse, and
  every other named feature stay deferred under a new, narrower
  `other_features_deferred` diagnostic — Swashbuckler stays permanently
  `Blocked`, the same honest shape as 7 of the 8 closures so far (only
  Arcanist reached `Computed`). `named_features_wired` would land at 3
  (Panache, Charmed Life, Nimble — three structurally independent
  mechanics, no shared table linking them, the same reasoning that gave
  Slayer 4 rather than folding like Arcanist/Warpriest/Oracle's 2).

### Investigator — a real positive surprise, but NOT recommended first (needs a new spell list)

- **Chassis**: 3/4 BAB, good Will/Reflex, poor Fortitude. HD 8.
- **Genuinely a PREPARED caster** (`SPELLSTAT:INT`, `MEMORIZE:YES`,
  `SPELLBOOK:YES`) — a real positive surprise, since this is the SAME
  simpler shape as Wizard/Arcanist/Warpriest, not the harder spontaneous
  shape Oracle needed. **But** `SPELLLIST:1|Alchemist` means it reuses
  the **Alchemist formula list**, and confirmed directly (`grep`/`find`)
  that **no Alchemist/formula spell list exists anywhere in this
  codebase yet** (Alchemist's own closure only ever grounded Mutagen,
  explicitly deferring spellcasting — `alchemist_stays_blocked...` test
  in `apg_class_chassis_dispatch_tests` confirms this). Building
  Investigator's real spellcasting would mean ingesting a genuinely new
  spell list first, a real data cost none of the 8 closures so far have
  needed (they all reused an already-built list: Cleric's for
  Oracle/Warpriest/Inquisitor, or built their own bounded 1-3 table for
  Arcanist).
- **Trapfinding** (`max(InvestigatorLVL/2,1)`) and **Trap Sense**
  (`InvestigatorLVL/3`, no `max(1,...)` floor — genuinely different from
  Slayer's own `max(1,SlayerLVL/3)`, verify directly, don't copy) are
  both real, flat, already-precedented standalone records.
- **Inspiration** (the marquee mechanic): a pool
  (`max(1,InvestigatorLVL/2+INT)`) spent on skill/ability checks (and,
  at higher cost, attack rolls/saves) — the pool SIZE is a flat, no-gate
  fact like Panache, but the pool's actual USE ties into skill/attack/
  save resolution in a way this codebase would need real per-roll
  integration to ground meaningfully, a bigger lift than Panache's own
  flat-fact-only MVP.
- **Class skills**: `CSKILL:...Climb|...Intimidate|...` — includes Climb
  and Intimidate but **NOT Swim** (confirmed absent from the list). This
  is a genuinely NEW situation: `selected_skill_class_skill_bonus_applies`
  is a single binary per-class flag (does this class get the flat +3 at
  all), not a per-skill toggle — Investigator's partial 2-of-3 match
  doesn't fit either existing branch (Wizard/Arcanist's "none of the
  three" or Warpriest/Slayer's "all three") and would need a real design
  decision (a per-skill bitmask, or accept the imprecision either
  direction) before any class-skill work here, not a mechanical copy of
  the existing fix.
- **Conclusion**: a real, honest MVP (Trapfinding + Trap Sense + Panache-
  shaped pool-size fact, deferring spellcasting and the partial class-
  skill question) is buildable, but costs more design decisions than
  Swashbuckler and doesn't get the "avoid a new subsystem" win Panache/
  Charmed Life/Nimble give Swashbuckler for free. Worth a follow-on
  scoping pass of its own if picked up later, not bundled into this one.

### Shaman, Summoner, Witch — unchanged from the prior pass

- **Shaman**: Spirit Animal is a real `TYPE:...Familiar`, not an Animal
  Companion (corrected in the second comparative scoping doc) — the same
  unbuilt Familiar subsystem gap as Witch. Not re-verified again this
  pass; no new finding.
- **Summoner**: needs a real Eidolon subsystem, unbuilt anywhere in this
  codebase.
- **Witch**: needs the same unbuilt Familiar subsystem as Shaman, plus
  its own ~20-entry Hex chooser-list (a separate, not-yet-audited
  `CATEGORY:Special Ability` chooser list per `ApgClassCoverage`'s own
  doc comment).

## Recommendation

Build **Swashbuckler** next (9th ACG/APG class-specific closure): full
BAB non-caster, zero spellcasting scope (the same "cheapest structural
shape" Slayer already proved out), three real flat/activation-gated
features (Panache, Charmed Life, Nimble) with clean precedent for each
shape, and the now-routine fourth instance of the class-skill-bonus
widening fix. Investigator is a real, buildable follow-on candidate but
costs strictly more (a partial class-skill design decision, and — if
spellcasting is ever wanted — a brand-new spell-list ingestion effort
none of the 8 closures so far have needed). Shaman/Summoner/Witch remain
correctly deferred pending their own subsystem work (Familiar/Eidolon),
unchanged from the prior pass.

## Open questions for the lead

1. Swashbuckler as scoped (Panache/Charmed Life/Nimble MVP, Deeds/
   Swashbuckler Finesse deferred, `named_features_wired = 3`) — greenlit,
   or a different MVP subset preferred?
2. The Charmed Life formula's negative-operand integer-division direction
   (`(SwashbucklerLVL-2)/4` at level 1) needs to be confirmed directly
   against a working formula-evaluation reference before coding — flagging
   this now rather than discovering it mid-build and guessing.
