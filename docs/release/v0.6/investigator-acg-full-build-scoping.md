# ACG Investigator — Full Class Build Scoping (10th ACG/APG Closure Target)

> Follow-on to `third-full-class-build-comparative-scoping.md`, which
> flagged Investigator as "a real positive surprise, but NOT recommended
> first (needs a new spell list)" and asked for "a follow-on scoping pass
> of its own if picked up later." This is that pass. Ran the same
> corpus-verification discipline as every prior closure — re-verified
> every claim directly against the ACG corpus and the real
> `pilot_compute.rs` code rather than trusting the comparative pass's
> summary — and found one real correction to the comparative doc's own
> framing of the class-skill question (see the Class-skill section: the
> "ground Climb/Intimidate as standalone facts outside the shared
> function" option the prior doc floated does NOT fit the actual code
> shape). Investigator's BAB/save chassis is **already built**
> (`rules_tables::acg::class_investigator`, `AcgClassId::Investigator`,
> `class_chassis_resolve` all present since the SD-22 Epic 4 ingest) — the
> only missing structural piece for a first closure is the gate widening
> plus grounding a few flat named features.

## Corpus findings (verified against `acg_classes.lst` / `acg_abilities_class.lst`)

### Chassis — confirmed exactly, and already transcribed

- `CLASS:Investigator HD:8 ... SPELLSTAT:INT MEMORIZE:YES SPELLBOOK:YES
  BONUS:CASTERLEVEL|Investigator|Caster_Level_Investigator
  SPELLLIST:1|Alchemist` (`acg_classes.lst:168` and `:172`, verified
  directly). 3/4 BAB (`BONUS:COMBAT|BASEAB|classlevel(...)*3/4`), poor
  Fortitude (`.../3`), good Will+Reflex (`.../2+2`), `MAXLEVEL:20`.
- **Genuinely a PREPARED caster** — `MEMORIZE:YES`, `SPELLBOOK:YES`,
  `SPELLSTAT:INT`, the SAME simpler shape as Wizard/Arcanist/Warpriest,
  **not** the harder spontaneous shape Oracle needed. Confirmed, not
  assumed.
- The chassis is **already built**: `rules_tables::acg::class_investigator`
  transcribes the full per-level BAB/save table (`class_table()`,
  `MAX_SUPPORTED_LEVEL = 20`, `HIT_DIE = 8`), `AcgClassId::Investigator`
  exists (`acg/mod.rs:74`), and `acg::class_chassis_resolve` already
  resolves it. So — unlike some earlier closures — this closure does NOT
  need any new chassis-table work; it only needs the gate widening plus
  feature grounding.

### Named features — 95 real records, three cheap flat ones for the MVP

- `grep -oE "KEY:Investigator ~ [^\t]+" acg_abilities_class.lst | sort -u
  | wc -l` → **95**, matching `named_features_expected(Investigator) = 95`
  already recorded in `acg/mod.rs:330` exactly.
- **Trapfinding** (`KEY:Investigator ~ Trapfinding`):
  `BONUS:VAR|InvestigatorTrapfindingBonus|max(InvestigatorLVL/2,1)` —
  verified directly. A flat bonus on Perception-to-find-traps and Disable
  Device. **Has** the `max(...,1)` floor.
- **Trap Sense** (`KEY:Investigator ~ Trap Sense`):
  `BONUS:VAR|TrapSenseBonus|InvestigatorLVL/3` — verified directly. **No**
  `max(1,...)` floor. This is genuinely different from Slayer's own Trap
  Sense, whose real record is
  `BONUS:VAR|TrapSenseBonus|max(1,SlayerTrapSenseLVL/3)` (verified in the
  same corpus) — Slayer's has the floor, Investigator's does not. **Note
  a real implementation hazard**: both classes' records use the *same*
  `BONUS:VAR` name `TrapSenseBonus`; the two must not be conflated, and
  Investigator's formula must be transcribed from Investigator's own
  record, not copied from Slayer's. (Symmetric hazard on Trapfinding too:
  Investigator's Trapfinding *has* the `max(...,1)` floor while Slayer's
  Trapfinding, `SlayerTrapfindingLVL/2` at `pilot_compute.rs:10037-10043`,
  does *not* — the floors are effectively swapped between the two
  features across the two classes. Verify each formula against its own
  record; do not copy.)
- **Inspiration** (`KEY:Investigator ~ Inspiration`, the marquee mechanic):
  pool size `BONUS:VAR|InvestigatorInspirationPoolBonus|max(1,InvestigatorLVL/2+INT)`,
  die `BONUS:VAR|InvestigatorInspirationDice|1` / `InvestigatorInspirationDieSize|6`
  (i.e. 1d6) — all verified directly. The pool **size** is a flat,
  no-gate, choice-free fact (same shape as Swashbuckler's Panache /
  Cleric's Channel Energy pool). The pool's **use** (expend one use as a
  free action to add 1d6 to a skill/ability check; two uses for an attack
  roll or saving throw) ties into per-roll skill/attack/save resolution
  this codebase has no surface for — that USE stays deferred; only the
  pool-size fact is grounded. The DESC also notes Knowledge/Linguistics/
  Spellcraft checks get Inspiration for free (trained) — a per-skill-check
  interaction, likewise deferred.

### Class skills — the genuinely-new partial-match, and a correction to the comparative doc

- `KEY:Investigator ~ Class Skills` carries
  `CSKILL:Acrobatics|Appraise|Bluff|Climb|TYPE=Craft|Diplomacy|Disable
  Device|Disguise|Escape Artist|Heal|Intimidate|TYPE=Knowledge|
  Linguistics|Perception|TYPE=Perform|TYPE=Profession|Sense Motive|
  Sleight of Hand|Spellcraft|Stealth|Use Magic Device` (verified
  directly). Of the three skills the selected-skill posture tracks:
  **Climb — present. Intimidate — present. Swim — absent.** A real 2-of-3
  partial match, the first on the whole roster.
- **Why this is forced by the gate widening, not optional.** I read the
  real code (`pilot_compute.rs:25250-25419`, plus the corpus mirror at
  `pilot_compute_corpus.rs:460-480`). The facts:
  - `selected_skill_class_skill_bonus_applies(input) -> bool`
    (`pilot_compute.rs:25250`) is a **single binary per-class flag**:
    `true` iff the character has Fighter/Rogue/Warpriest/Slayer/
    Swashbuckler levels — every one of which has **all three** of
    Climb/Intimidate/Swim as class skills. Wizard (the only currently-
    supported "none of three" class) correctly returns `false`.
  - `compute_selected_skill_modifiers` applies that single bool as one
    `CLASS_SKILL_BONUS` scalar **uniformly to all three** skills
    (`pilot_compute.rs:25301-25302`, then climb/intimidate/swim at
    `:25324/:25340/:25353`). The corpus-mirror path
    (`pilot_compute_corpus.rs:474`) does the same.
  - The posture gate `unmet_selected_skill_posture_conditions`
    (`pilot_compute.rs:25382`) keys on **`has_supported_class_chassis`**
    (widened at SD-21 E6b.1 from a Fighter-only gate). So the instant
    this closure adds `is_supported_investigator_single_class` to
    `has_supported_class_chassis` (which is the whole point of the
    closure), an Investigator character in the exact selected-skill
    posture (Chain Shirt equipped, Climb/Intimidate/Swim each rank 1, no
    other allocations) becomes computable — and the binary model produces
    a **demonstrably wrong number** for at least one skill:
    - add Investigator to the applies-list → **Swim wrongly gains +3**
      (Swim is not an Investigator class skill), or
    - don't add it → **Climb and Intimidate wrongly lose +3** (both ARE
      Investigator class skills).
  - Every currently-supported class is cleanly all-three or none-of-three,
    which is exactly why the binary flag has been correct until now.
    Investigator is the first partial match, so the binary model cannot
    represent it correctly. This is a genuine design decision, forced.
- **Correction to `third-full-class-build-comparative-scoping.md`.** That
  doc floated "grounding Climb/Intimidate individually as standalone
  facts outside that shared function, the same way Trapfinding/Track are
  grounded standalone." Having read the real code, that option does **not**
  fit: Trapfinding/Track ground bonuses to skills the totals-function
  does **not** track, so they compose cleanly. Climb and Intimidate ARE
  tracked — `compute_selected_skill_modifiers` is the single authoritative
  computer of their totals for the supported posture — so a parallel
  "standalone" Climb/Intimidate fact would either double-count or
  contradict that function's own output. The real fork is not
  "shared-function vs standalone-fact"; it is (A) vs (B') below.

## The two real decisions this closure must make

### Decision 1 — the class-skill partial match: (A) per-skill widening, or (B') honest block

- **(A) Per-skill widening (recommended).** Change the class-skill bonus
  from a single scalar into a per-skill determination: compute a separate
  bonus for Climb, Intimidate, and Swim from whether each is a class skill
  for the character's class. Scope is **small and well-bounded** — exactly
  two call sites (`pilot_compute.rs:25301` and `pilot_compute_corpus.rs:474`),
  each of which already computes the three skills as separate `let`
  bindings, so splitting one bool into three is a natural change, not a
  structural rewrite. Crucially it is **behavior-preserving for every
  existing class**: Fighter/Rogue/Warpriest/Slayer/Swashbuckler are all
  "all three" (per-skill still yields +3 on each), Wizard is "none" (still
  0 on each) — identical outputs, zero regression risk — while being the
  **only correct model** for Investigator (Climb +3, Intimidate +3, Swim
  +0). Because it touches a shared function five classes rely on, it is
  the one "real design decision" of the slice and worth the lead's
  explicit sign-off, but the risk is genuinely low.
- **(B') Honest block (fallback).** Keep the binary model; add a narrow
  Investigator-specific exclusion to
  `unmet_selected_skill_posture_conditions` that blocks **only** the
  Climb/Intimidate/Swim slice for Investigator with a claim-blocking
  diagnostic ("selected-skill posture not yet supported for Investigator:
  its class-skill list is a partial Climb+Intimidate/no-Swim match
  pending per-skill class-skill modeling"). The chassis/saves/HD and the
  Trapfinding/Trap Sense/Inspiration facts flow through separate paths and
  stay grounded; only the selected-skill posture is honestly deferred, no
  wrong number emitted. This defers Decision 1 into its own later slice,
  matching the segment's "block honestly, ship the rest" rhythm.
- **Recommendation: (A).** It is small, zero-regression for all six
  existing classes, and the only fully-correct model; (B') ships an
  Investigator whose selected-skill posture is visibly blocked for a gap
  that (A) closes cheaply. Recommend (A) unless the lead prefers to keep
  this closure minimal and hand the shared-function change to a dedicated
  follow-on.

### Decision 2 — spellcasting this slice, or a smaller no-spellcasting MVP first

- Investigator's `SPELLLIST:1|Alchemist` reuses the **Alchemist formula
  (extract) list**. Confirmed by grep/find that **no Alchemist class
  spell-list mapping exists anywhere in this codebase**: the only spell
  data present is (a) the CRB per-class mappings
  (`cleric_spell_list.rs::CLERIC_SPELL_LIST: &[(&str, u8)]` and the bard/
  druid/sorcerer/paladin/ranger equivalents), and (b) `apg/spell_list.rs`
  / `acg/spell_list.rs`, which are per-spell *catalogs* (school/level/
  description), **not** class-to-spell mappings. Alchemist's own earlier
  closure grounded only Mutagen and explicitly deferred spellcasting
  (`push_alchemist_spellcasting_deferred_diagnostic`,
  `pilot_compute.rs:8585`), so no Alchemist list was ever built.
- The **raw source exists**: `apg_spells.lst` has **104 records** carrying
  an `Alchemist=N` class token (extracts, levels 1–6) — **corrected
  2026-07-27: the true count is 121**, this figure undercounted every
  spell where Alchemist wasn't the last name in its `CLASSES:` comma
  group; see task #24 (`5f9f23aa`). The two further "~104-record" mentions
  below (decision framing, not a re-derivation) share the same correction.
  So an
  `alchemist_spell_list.rs` (the `(&str, u8)` mapping shape of
  `cleric_spell_list.rs`) is buildable via the same one-off-ingestion
  discipline that produced the CRB lists — parsing each `CLASSES:...
  Alchemist=N` token into `(spell_key, alchemist_level)` pairs — but that
  is **a genuinely new data-ingestion cost none of the 9 prior closures
  incurred** (Oracle/Warpriest/Inquisitor reused Cleric's existing list;
  Arcanist built its own bounded 1–3 table; the rest are non-casters).
- **Recommendation: smaller no-spellcasting MVP first.** Ground the
  chassis gate + Trapfinding + Trap Sense + Inspiration pool-size, defer
  spellcasting to a dedicated follow-on slice. Rationale: (1) it keeps the
  closure to the segment's one-honest-unit rhythm and avoids bundling a
  ~104-record new-list ingestion into the same slice as chassis + features
  + the shared skill-function change; (2) the non-spellcasting features are
  all flat, already-precedented standalone facts, a clean verifiable MVP;
  (3) there is direct precedent for splitting spellcasting into its own
  closure — `skald-spellcasting-closure-scoping.md` did exactly this, and
  Cavalier/Alchemist/Brawler/Hunter/Bloodrager all started as smaller
  closures. The eventual spellcasting slice would build
  `alchemist_spell_list.rs` (reusing the `cleric_spell_list.rs` ingestion
  pattern) and then reuse Wizard/Arcanist/Warpriest's **prepared-shape**
  validation (not Oracle's spontaneous shape) — a natural standalone unit.
  Bonus: the Alchemist list, once built, is reusable to later close
  Alchemist's own deferred spellcasting too.

## Proposed scope (no-spellcasting MVP)

1. `is_supported_investigator_single_class` — exact
   `AcgClassId::Investigator` match at a level within
   `acg::class_chassis_resolve`'s ceiling, mirroring
   `is_supported_swashbuckler_single_class` exactly (same single-class,
   exact-match discipline). Add it to `has_supported_class_chassis`.
2. **Trapfinding** — ground the flat `max(InvestigatorLVL/2,1)` bonus as a
   standalone `ComputationExplanation` (id
   `class_feature.acg.investigator.trapfinding_bonus`), the exact idiom of
   `slayer_trapfinding_bonus` (`pilot_compute.rs:10043-10099`), transcribed
   from Investigator's own record (with the floor).
3. **Trap Sense** — ground the flat `InvestigatorLVL/3` bonus (no floor)
   as a standalone fact (id `class_feature.acg.investigator.trap_sense_bonus`),
   transcribed from Investigator's own record — explicitly not copied from
   Slayer's floored `max(1,...)` variant despite the shared `TrapSenseBonus`
   var name.
4. **Inspiration pool size** — ground the flat
   `max(1,InvestigatorLVL/2+INT)` pool size (die 1d6) as a standalone
   no-gate fact, the Panache-shaped "pool size only" MVP. The pool's use
   (per-roll spend), the free Knowledge/Linguistics/Spellcraft uses, and
   the attack/save spends stay deferred.
5. **Decision 1**: either (A) the per-skill class-skill widening (correct
   Climb +3 / Intimidate +3 / Swim +0 for Investigator, behavior-
   preserving for all existing classes) — recommended — or (B') a narrow
   Investigator exclusion in `unmet_selected_skill_posture_conditions`
   that honestly blocks only the selected-skill slice.
6. New, narrower `class_feature.acg.investigator.other_features_deferred
   .unsupported` diagnostic naming the deferred remainder (see below).
   Investigator stays **`Blocked`** on this MVP (like 8 of the 9 closures
   so far; only Arcanist reached `Computed`, and only because it grounded
   real spellcasting — Investigator can reach `Computed` only via the
   Decision-2 spellcasting slice).
7. `named_features_wired = 3` — Trapfinding, Trap Sense, and Inspiration
   (pool-size), three structurally independent standalone facts with no
   shared table linking them (the same reasoning that gave Slayer 4 rather
   than folding to Arcanist/Warpriest/Oracle's 2 — see the
   `named-features-wired-counting-methodology` discipline).

## What stays explicitly deferred, named honestly

- **Spellcasting** (the prepared extract list) — pending the new
  `alchemist_spell_list.rs` ingestion slice (Decision 2).
- **Inspiration's use** — per-roll spend on skill/ability checks, the free
  Knowledge/Linguistics/Spellcraft uses, and the 2-use attack/save spends
  (no per-roll resolution surface in this codebase).
- **Investigator Talents** — the class's own chooser-list, plus the large
  Rogue-Talent and Discovery sub-lists (the `KEY:Investigator ~ Rogue
  Talent ~ ...` / `~ Discovery ~ ...` families, the bulk of the 95
  records) — deferred like Slayer Talents / Deeds.
- **Alchemy, Studied Combat, Studied Strike, Keen Recollection, Poison
  Lore/Resistance, Swift Alchemy, True Inspiration** and the remaining
  named features — real mechanics, named but not built.
- If Decision 1 is resolved as (B'), the **selected-skill posture for
  Investigator** stays blocked pending per-skill class-skill modeling.

## Open questions for the lead

1. **Decision 1** — take (A) the per-skill class-skill widening now
   (recommended: small, two call sites, behavior-preserving for all six
   existing classes, the only correct model for Investigator), or (B')
   honestly block just the selected-skill slice for Investigator and hand
   the shared-function change to a later dedicated slice?
2. **Decision 2** — greenlight the smaller no-spellcasting MVP first
   (recommended), with the Alchemist-formula-list ingestion + prepared-
   caster wiring as its own follow-on slice (mirroring the Skald
   spellcasting split)? Or is landing spellcasting in the same slice
   wanted, accepting the new ~104-record list-ingestion cost and a larger
   single closure — which is also the only way Investigator reaches
   `Computed` rather than staying honestly `Blocked`?
3. `named_features_wired = 3` (Trapfinding / Trap Sense / Inspiration
   pool-size) — is that the right count under the
   `named-features-wired-counting-methodology` discipline, or should the
   Inspiration pool-size fact (whose USE is deferred) be counted
   differently given only its size, not its effect, is grounded?
