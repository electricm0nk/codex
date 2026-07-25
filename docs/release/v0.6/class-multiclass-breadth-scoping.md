# Class/Multiclass Breadth (Risks Item 8) — Scoping Plan

> **Scope correction (operator, 2026-07-24):** this is not a bounded
> "8 CRB classes, future epic" item — it is the swarm's primary active
> work. Every class across the four books this alpha bar already names
> (CRB, Bestiary 1, APG, ACG) needs real BAB/save/HP/skill/spellcasting/
> feature computation, not a data-only stand-in. Bestiary 1 carries no
> player classes (monster/equipment data only) and is dropped from this
> scope entirely. That leaves **27 total classes** (11 CRB + 6 APG + 10
> ACG), **24 without a working chassis** (Fighter/Wizard/Rogue already
> reach `Computed`).
>
> This plan sizes and sequences the work, then leads directly into
> continuous implementation — the same shape as `items-1-and-27-scoping.md`,
> which led straight into 6 shipped sub-tasks in one session. No
> permission checkpoint per class; each bounded cycle lands, gets
> verified, and the next one starts.

## The central finding: the BAB/save chassis-resolver infrastructure already exists for all three books

Verified directly (not assumed from the backlog framing, and corrected
once already after an initial pass under-verified the APG/ACG story):

- **CRB**: `rules_tables::crb::class_tables()` already carries real,
  data-verified BAB/save/hit-die/level-ceiling rows for all 11 classes.
  `compute_generic_table_chassis` (`pilot_compute.rs`) is already built
  and already used for Rogue. The only gate is `table_class_id`
  (`pilot_compute.rs:6747`), a 3-entry allowlist (Fighter/Wizard/Rogue).
- **APG**: `rules_tables::apg::mod.rs` already defines `ApgClassId`
  (Alchemist, Cavalier, Inquisitor, Oracle, Summoner, Witch), a local
  `ClassTableRow`, and a real `class_chassis_resolve(class_id, level,
  rule_set)` function. Each per-class file (`class_alchemist.rs`, etc.)
  transcribes its real `BONUS:COMBAT|BASEAB|...`/`BONUS:SAVE|...` tokens
  from the actual `apg_classes.lst` corpus record, cited verbatim in the
  file's own doc comment. This was built in a past cycle (SD-22 Epic 3)
  and deliberately left unwired — there is already a coverage-audit test
  (`tests/sd24_apg_class_coverage_audit.rs`) confirming these classes
  correctly trip `class_chassis.unsupported` today, i.e. the gap was
  already tracked, not undiscovered.
- **ACG**: `rules_tables::acg::mod.rs` mirrors APG exactly —
  `AcgClassId` with all 10 real classes (Arcanist, Bloodrager, Brawler,
  Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest),
  same `ClassTableRow`/`class_chassis_resolve` shape.

**What this means for sequencing**: the BAB/save pillar is not "CRB is a
dispatch fix, APG/ACG need new infrastructure built from scratch" — all
three books already have real, working resolver infrastructure; the
missing piece for all 24 unwired classes is the same shape of gap,
**wiring `pilot_compute.rs`'s dispatch to consult the right book's
resolver for a given class id.** Since real class names are unique across
all three rosters today (no CRB/APG/ACG name collisions), a unified
dispatch can resolve purely by class-id string without needing an
explicit book selector.

**One real asymmetry, not a contradiction**: APG/ACG's per-class files
carry BAB/save only — no hit-die token. `durability.rs::compute_max_hp`
depends on both `table_class_id` and CRB's `hit_die_for` (`class_tables.rs`'s
own `CLASS_META`). HP for the 16 APG/ACG classes needs a small, real,
corpus-sourced addition (each class's real `HD:` token, not yet
transcribed) alongside the dispatch wiring — a bounded, per-class-file
one-line addition, not new engineering.

## Why the CRB dispatch gate isn't a one-line fix either

`table_class_id`'s own doc comment is first-hand evidence this was
already tried: widening it to all 11 CRB classes in one step broke ~60
pre-existing negative-control assertions across ~15 `tests/**` files
(confirmed via `cargo test --test '*' --no-fail-fast` in this codebase's
own history) — those files assert, verbatim, that each class's own
standalone chassis explanation "must not be wired into the integrated
total" (e.g. `tests/sd13_ranger_base_attack_and_saves.rs` checks the
explanation text literally contains `"standalone"`). Any widening needs
the same per-class QA test-coordination protocol as sub-task 1's 42-site
handoff — compute the exact site list, hand off, don't touch `tests/**`
unilaterally.

Beyond the dispatch gate, **7 of CRB's 8** unsupported classes (all but
Ranger) each push their own separate, unconditional claim-blocking
diagnostic inside their `explain_<class>_...` function — a second,
independent gate naming a real, specific missing execution engine:

| Class | Self-blocking diagnostic(s) | What's missing |
|---|---|---|
| Barbarian | rage-execution unsupported | Rage *execution* (activation, round consumption, fatigue) — constants/rounds-per-day already grounded |
| Monk | bonus-feat unsupported | The level-1 bonus feat's real mechanics (needs a feat-effect, same engine as risks item 17). Also capped at level 12, not 20 |
| Cleric | domain-power burden + prepared-divine unsupported | Domain *powers* execution; full prepared-divine spell posture |
| Druid | nature-bond (ungrounded) + prepared-divine unsupported | Nature bond entirely; same prepared-divine posture as Cleric. Smallest/least-built of the 8; capped at level 15 |
| Paladin | partial-caster posture unsupported | Only the spell posture is out of scope — Smite Evil/Lay on Hands/Divine Grace/Mercy/Channel Positive Energy/late auras/Holy Champion all grounded through level 20 |
| **Ranger** | **none** | Its explain function structurally has no `diagnostics` parameter — cannot self-block. BAB/saves/Track/combat style/Favored Enemy+Terrain/Hunter's Bond/full partial-caster spell ladder all grounded through level 20 |
| Sorcerer | bloodline burden + spontaneous-spell unsupported | Bloodline power progression; spontaneous known-spell/slot posture |
| Bard | performance-execution burden + spontaneous-spell unsupported | Bardic Performance *execution*; spontaneous known-spell/slot posture |

Four classes (Cleric, Druid, Sorcerer, Bard) name the identical remaining
shape — a "prepared/spontaneous spell posture" burden — worth treating as
one shared future slice, not four separate ones.

APG's 6 and ACG's 10 have not had this same per-class explain-function
investment yet at all — no standalone chassis explanations, no named
class features wired, no spellcasting. Their gap after BAB/save dispatch
wiring is the full remaining stack (skill lists, class features,
spellcasting) with nothing yet built, the same as CRB's classes but
starting one step further back (no `explain_*` scaffolding exists to
un-gate).

## Two real prerequisite gaps, not to discover mid-implementation

1. **Zero ingested corpus data for 4 non-CRB classes**: Gunslinger,
   Magus, Ninja, Samurai. QA confirmed no local corpus data exists for
   any of them (they live in Ultimate Combat/Ultimate Magic/other books
   not yet ingested). These 4 cannot be scoped from what's on disk today
   — a prerequisite ingest task, not chassis work, and out of this plan's
   scope until that lands. (Note: `money::starting_wealth_gp` already
   recognizes these 4 by id for wealth purposes only, sourced from the
   d20pfsrd character-creation page, which is a separate content source
   from the per-class chassis corpus — the wealth citation does not
   imply chassis data exists.)
2. **ACG wealth was never covered by the operator's citation.** The
   d20pfsrd table already implemented only spans CRB + APG. If ACG
   starting wealth is wanted, it needs its own fresh operator citation —
   not guessed, not inferred from a pattern.

## The uniform, largest remaining bucket: skill lists, class features, spellcasting

Regardless of book, every one of the 24 classes needs, before reaching a
real `Computed` status at any level:

- **Class-skill lists**: zero corpus presence anywhere for any of these
  24 (Fighter/Rogue/Wizard's bounded 5-skill slice was hand-authored
  per-class, the same shape needed here, times 24).
- **Named class features**: the biggest bucket by far — Rage, Wild
  Shape, Bardic Performance, Smite Evil, Favored Enemy, Ki pool, Channel
  Energy, Domains, Turn Undead, Bombs, Discoveries, Mutagen, Judgment,
  Hexes, Eidolon, Challenge, and dozens more, each a unique,
  non-formulaic mechanic requiring its own small engine (same shape as
  `feat_effects.rs`'s per-feat pattern, at class-feature scale).
- **Spellcasting** (6 of the 24 classes CRB-side alone: Cleric, Druid,
  Sorcerer, Bard, Paladin, Ranger — more once APG/ACG casters are
  counted): per-class spells-known/per-day/save-DC projection.
  `apg/spell_list.rs`'s own doc comment cites the exact PCGen checkout
  path its generation script used — worth checking whether that checkout
  still exists locally before hand-reconstructing anything; if it does,
  real per-class spell levels may be directly re-derivable rather than
  transcribed by hand.

This bucket is the same size no matter how CRB vs. APG vs. ACG is
sliced — it is real, un-touched engineering work for all 24 classes, and
sizing it further needs to happen per-class as each class's cycle is
scoped, not all at once here.

## Sequencing recommendation

**Immediate first slice: Ranger (CRB).** Zero self-block, largest
existing investment (2,425 lines, 33 test files, full level-20 chassis +
partial-caster spell ladder already grounded as standalone explanations).
Needs only: (1) the `table_class_id` dispatch entry, (2) the QA
test-coordination handoff for its own standalone-assertion test file(s),
(3) multi-level re-verification (1, 5, 11, 20 breakpoints, mirroring
sub-task 4's own discipline), (4) an explicit call on whether
`durability.rs`'s max-HP widens for Ranger in the same cycle or is
deliberately deferred.

**Immediately after, in parallel-shaped bounded cycles, not batched:**

- **BAB/save dispatch unification across APG + ACG** (16 classes at
  once, since it's the same mechanical wiring repeated 16 times once the
  unified-by-name dispatch shape is built) — real, valuable, bounded
  progress on the shared pillar, though it does not alone reach
  `Computed` for any of the 16 (skill lists/features/spellcasting still
  missing). Include the hit-die transcription for all 16 in the same
  cycle since it's the same small addition per file.
- **CRB's remaining 7**, one bounded cycle per class, each resolving that
  class's own named execution-engine gap (table above) plus its own QA
  test-coordination handoff. Sequence by whichever the operator/lead
  prioritizes — no dependency between them (a Cleric mistake doesn't
  block a Barbarian cycle).
- **Skill lists + class features + spellcasting**, per class, as the
  long tail after each class's BAB/save/HP pillar is real — this is
  where the bulk of the actual remaining engineering lives, and each
  class's own cycle should size that class's specific feature list
  before starting (matching how Barbarian's rage-execution gap is
  already named precisely, not guessed).

**Blocked, not scheduled**: Gunslinger, Magus, Ninja, Samurai chassis
work, until corpus ingestion lands. ACG starting wealth, until a fresh
operator citation exists.

## Small, separate fixes surfaced along the way

- `multiclass_class_level_supported`'s doc comment claims it supports
  "every core class" the table carries data for; the function body still
  only recognizes 3 via `table_class_id`. Stale relative to the code —
  worth a one-line comment fix alongside the first dispatch-widening
  cycle so it doesn't mislead a future reader.
- `support_state_matrix.rs` independently tracks these same classes and
  shows several as `Computed` on its own axis — that's a value of the
  unrelated `EvidenceTier` enum ("this dimension has been exercised in
  code"), not `HeadlessReceiptStatus::Computed` ("no claim-blocking
  diagnostic present"). Same variant name, unrelated types. Not a
  contradiction of this plan's framing, just a naming trap worth knowing
  about so it isn't misread.

## What this plan is not

Not a decision to build all 24 in one cycle, and not a claim that BAB/save
wiring alone finishes any class — it is real dispatch integration work
that unlocks visible progress fast, with the much larger skill/feature/
spellcasting engineering still ahead per class. Ranger is the one class
close enough to the finish line that its own cycle should reach a real,
additional `Computed` status; every other class's first cycle only
advances its BAB/save/HP pillar, not full completion.
