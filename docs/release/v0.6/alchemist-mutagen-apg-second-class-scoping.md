# APG Alchemist Mutagen — Second APG Class-Specific Closure (Risks Item 8) — Scoping Plan

> Directed by the lead after Cavalier's Mount closure, with two things
> flagged upfront to size honestly: this is bigger than the 5 closures
> so far (a genuine choice dimension, not just activation-gating), and
> it will very likely land in the same headless-only reachability bucket
> as Sorcerer/Cleric/Druid. Both confirmed true below before any code.

## Corpus findings (verified against `apg_classes.lst` / `apg_abilities_class.lst`)

- **Chassis**: 3/4 BAB, good Fortitude/Reflex, poor Will (matches
  `EXPECTED_LEVEL_1`'s `("class:alchemist", 0, 2, 2, 0, 8)` row). HD 8.
  `SPELLSTAT:INT`, `MEMORIZE:YES`, `SPELLBOOK:YES` -- Alchemist genuinely
  casts (extracts function as spells prepared from a spellbook-like
  list), unlike Brawler/Cavalier. Spellcasting stays deferred this slice
  regardless, the same shape as every other partial-caster class closed
  so far.
- **Mutagen** (`KEY:Alchemist ~ Mutagen`, `apg_abilities_class.lst:75`):
  full DESC quoted verbatim below. Corpus formulas:
  `AlchemistMutagenDuration = AlchemistLVL*10` (minutes), `AlchemistMutagenDC
  = 10+(AlchemistLVL/2)+INT` (the DC a NON-alchemist must beat to avoid
  nausea if THEY drink it -- not relevant to the alchemist's own use, not
  built this slice), `MutagenStatBonus = 4`, `MutagenStatPenalty = -2`,
  `MutagenACBonus = 2`. The `Mutagen ~ Stat Selection First/Second/Third/
  Final` sub-abilities and `SecondMutagenStatBonus`/`ThirdMutagenStatBonus`
  are gated on `MutagenTierLVL` values only reachable via later
  Discoveries -- a level-1-only single-stat mutagen (this slice's own
  bound) never reaches them, so they're out of scope, named honestly.

## Full corpus DESC (for the record)

> "You know how to create a mutagen that you can imbibe in order to
> heighten your physical prowess at the cost of your personality. It
> takes 1 hour to brew a dose of mutagen, and once brewed, it remains
> potent until used. You can only maintain one dose of mutagen at a time
> -- if you brew a second dose, any existing mutagen becomes inert... It's
> a standard action to drink a mutagen. Upon being imbibed, the mutagen
> causes you to grow bulkier and more bestial, granting you a +2 natural
> armor bonus and a +4 alchemical bonus to the selected ability score for
> [level*10] minutes. In addition, while the mutagen is in effect, you
> take a -2 penalty to one of your mental ability scores. If the mutagen
> enhances your Strength, it applies a penalty to your Intelligence. If
> it enhances your Dexterity, it applies a penalty to your Wisdom. If it
> enhances your Constitution, it applies a penalty to your Charisma...
> The effects of a mutagen do not stack. Whenever an alchemist drinks a
> mutagen, the effects of any previous mutagen immediately end."

## Why this is genuinely bigger than the 5 closures so far

Every prior activation-gated closure this session (Barbarian Rage, Bard
Inspire Courage, Cleric Touch of Good, Skald Inspired Rage, Bloodrager
Bloodrage) affects a FIXED set of stats -- the character never chooses
WHICH ability gets boosted. Mutagen genuinely requires a choice: "you
select one physical ability score -- either Strength, Dexterity, or
Constitution" -- and the mental-score PENALTY target is determined BY
that choice (Str->Int, Dex->Wis, Con->Cha), not fixed. This needs BOTH
patterns this session has separately proven, combined for the first
time:

- **Choice recognition** (mirrors Sorcerer's Arcane Bond / Cleric's
  domain choice): a new `choice:alchemist_mutagen_stat` choice set,
  recognizing `ability:strength`/`ability:dexterity`/`ability:constitution`
  as the three valid selections.
- **Activation gating** (mirrors Barbarian Rage/Skald/Bloodrager): a new
  `class_ability_activations` entry (`ability_id: "mutagen"`) recognizing
  whether the mutagen is currently drunk/active. Unlike Rage, there is no
  rounds-per-day BUDGET to validate (a mutagen has no daily-use cap in
  the corpus text -- only "you can only maintain one dose at a time,"
  a brewing-supply constraint this codebase's activation-state schema
  doesn't need to model to ground the ability-score/AC values honestly).

**Proposed combination logic** (not yet reviewed, flagging explicitly):
a character with NO activation entry, or one present but not
`EquippedActive`, is a genuinely valid "hasn't brewed/isn't currently
mutated" posture (mirrors Barbarian's own "not raging" branch) -- grounds
an honest recognition record, no claim-block. A character with an
ACTIVE mutagen activation but NO recognized `choice:alchemist_mutagen_stat`
selection (or an unrecognized one) is a genuine posture violation --
claiming to be mutated without saying which stat is enhanced is
inconsistent input, mirroring Sorcerer's own "recognized bloodline but
no bond choice given" claim-blocking shape. Only a genuinely active
activation PLUS a recognized stat choice together ground the real
+4/-2/+2 values.

## Reachability finding, confirmed directly (per the lead's explicit ask)

Grepped the desktop frontend before writing this doc. `CharacterSheet.tsx`'s
own `handleLevelUpAccept`/level-up-choice-persistence code confirms
there is still no generic choice-submission UI: the level-up flow only
ever submits one FIXED, hardcoded choice
(`choice:level_N_hit_points -> hp:average`), and its own doc comment
states plainly: "the level-up's own `additionalChoices` only ever lands
in the inert `chosen.selected_choices` provenance bag (nothing reads it
as a gate or a grant)." There is no picker anywhere in the creation or
level-up UI for an arbitrary choice set like
`choice:alchemist_mutagen_stat`. **Confirmed: this closure will land in
the exact same headless-only reachability bucket as Sorcerer/Cleric/
Druid's own choice-gated mechanics**, not the always-on/activation-only
bucket Skald/Bloodrager/Brawler/Hunter/Cavalier landed in. Naming this
upfront in this doc, per the lead's explicit request, rather than
letting it surface later the way it did for the CRB casters.

## Proposed scope

1. `is_supported_alchemist_single_class` -- exact `ApgClassId::Alchemist`
   match, mirroring Cavalier's own gate exactly (second APG-side use).
2. `ALCHEMIST_MUTAGEN_STAT_CHOICE_ID`, `ALCHEMIST_MUTAGEN_ABILITY_ID`
   constants plus the three selection-id constants
   (`ability:strength`/`ability:dexterity`/`ability:constitution` --
   reusing the exact `ABILITY_SELECTION_PREFIX` idiom already used for
   Human's own ability-bonus choice, not inventing a new naming scheme).
3. `alchemist_mutagen_duration_minutes(level) -> i16` (`level * 10`,
   informational only -- this codebase tracks no elapsed-time state, the
   same "named but not simulated" shape Barbarian Rage's own
   rounds-consumed-today budget uses for a different resource).
4. `ground_or_block_alchemist_mutagen` mirroring
   `ground_or_block_barbarian_rage`'s three-branch shape (not-active /
   active-with-recognized-choice / active-without-recognized-choice),
   applying +4/+2 ability-score-to-modifier halving (identical math to
   every Rage-shaped bonus this session already built) to the CHOSEN
   physical score, -2/-1 to the CORRESPONDING mental score (a small
   lookup table: Str->Int, Dex->Wis, Con->Cha), and +2 natural armor to
   the shared Armor Class total (mirroring Brawler's own AC-Bonus
   integration point).
5. New, narrower `class_feature.apg.alchemist.spellcasting_deferred
   .unsupported` diagnostic (Alchemist genuinely casts, confirmed above)
   replacing the generic one, naming Alchemist's remaining named
   features (Bomb, Discovery, Poison Resistance, Swift Alchemy, Swift
   Poisoning, and the rest).
6. Tests mirroring the Rage-shaped closures' own shape (not-mutated,
   actively-mutated-with-recognized-choice for each of the 3 stats,
   actively-claimed-but-unrecognized-choice stays blocked), plus the
   standard diagnostic-swap/positive-leak/negative-leak trio (now "the
   other 4 APG classes," Cavalier already admitted).

## What stays explicitly out of scope, named honestly

- The non-alchemist-drinker DC/nausea rule (`AlchemistMutagenDC`) -- not
  relevant to grounding the alchemist's own use, and would need a
  second-character/opponent representation this codebase doesn't have.
- Tier-scaling to a second/third simultaneous stat (Discovery-gated,
  never reachable at level 1).
- Every other named Alchemist feature (Bomb, Discovery, Poison
  Resistance, Swift Alchemy, Swift Poisoning) and Alchemist's own
  extract/spellcasting posture.

## Open questions for the lead / adversarial review

- **Is the choice+activation combination logic above correct**,
  specifically the "active-but-unrecognized-choice claim-blocks" branch?
  This is the one genuinely new interaction shape (Rage-shaped mechanics
  never needed a companion choice; Sorcerer/Cleric's own choices never
  needed an activation-state check on top). Flagging for a second look
  even though the lead doesn't expect a full review, since this is the
  one piece without a direct one-to-one precedent.
- **Mental-score penalty lookup table**: Str->Int, Dex->Wis, Con->Cha,
  verified directly from the corpus DESC text quoted above -- worth a
  quick spot-check given it's the one piece of this mechanic with no
  numeric ambiguity but real branching logic to get right.
