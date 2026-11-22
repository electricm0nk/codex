---
cycle_id: SD31-E3-F3-001
card: epic-3-measurement (feature seed F3)
status: DESIGN DECISION, not yet built
---

# The chooser-interaction primitive — design (Oracle, Arcanist, Sorcerer)

## Problem statement

`archetype_claims_slot(input, subject, slot_id)` (`src/rules_core/archetype_resolver.rs:86`)
answers one boolean: *"has the character's selected archetype swapped out this named slot?"*
That works cleanly for the 25 supersession-shape classes (Fighter's `AlchemistPoisonResistance`-style
named slots) because each slot corresponds to exactly one fixed computation — claiming the slot means
"don't run that computation," full stop.

Oracle, Arcanist and Sorcerer break that 1:1 assumption. Their archetype tables *do* use the identical
corpus encoding — Arcanist's `Blood Arcanist` archetype literally declares
`replaces: Some(&["ArcanistExploit1", "ArcanistExploit3", "ArcanistExploit9", "ArcanistExploit15", "ArcanistMagicalSupremacy"])`
in `src/rules_core/rules_tables/acg/archetype_tables.rs:85` — but the thing named by
`"ArcanistExploit1"` is not a formula, it is a **tier of a choice**: *"the exploit the character picked
at 1st level."* The same slot name can resolve to any of the 46 corpus-declared `Arcanist Exploit ~ *`
records, each independently grounded or not. `archetype_claims_slot` can correctly say "tier 1 was
swapped," but nothing today asks the second, load-bearing question: *given that tier 1 was NOT swapped,
which of the 46 real exploits did the character actually pick, and does pilot_compute.rs compute that
one?* That second question is what `ground_or_block_arcanist_metamagic_knowledge` already answers, by
hand, for exactly one of the 46 (`ARCANIST_METAMAGIC_KNOWLEDGE_CHOICE_ID`) — the same shape repeats for
Oracle's `ORACLE_REVELATION_CHOICE_ID`/`ORACLE_MYSTERY_CHOICE_ID` pair and Sorcerer's
`SORCERER_BLOODLINE_CHOICE_ID`.

## Two questions, not one

The card's own framing — *"which options remain choosable, and does the substitute grant compute"* — is
two separable questions, and conflating them is the design mistake to avoid:

1. **Availability** — is a given option still in the character's choosable pool, or has an archetype
   removed the tier it belongs to? (`archetype_claims_slot` already answers this correctly and needs no
   new code — it is subject/slot-id generic and was proven on Fighter/Alchemist before this class family
   existed.)
2. **Grounding** — for an option the character actually holds (recorded via `SelectedChoice` on the
   class's own `choice:*` id, e.g. `choice:oracle_revelation`), does a real, corpus-verified computation
   exist for that *specific* selection? This is genuinely new per option — there is no way to answer it
   generically, because each option (each revelation, each bloodline, each exploit) is a distinct rules
   effect with its own magnitude, and fabricating a shared "average" computation would be exactly the
   invented-number failure this program's doctrine forbids.

## Candidate designs considered

**Design A — one unified `chooser_slot` abstraction merging (1) and (2).** A single function taking
`(input, subject, tier_slot_id, corpus_option_pool)` that internally re-derives both "was this tier
swapped" and "which option was picked, does it ground" in one call, returning a tri-state
(`Superseded`/`GroundedAs(id)`/`Blocked`). Rejected: it re-implements `archetype_claims_slot`'s already-
proven logic a second time inside a new function, doubling the surface that can drift, for no benefit —
the two questions have completely different inputs (one reads `choice:archetype`, the other reads the
class's own `choice:*_mystery`/`choice:*_bloodline`/`choice:*_exploit` id) and answering them together
buys nothing a thin composition doesn't already buy.

**Design B (recommended) — reuse `archetype_claims_slot` verbatim for availability; add one new, per-
class-family primitive for grounding.** `archetype_claims_slot(input, "Arcanist", "ArcanistExploit1")`
already works today, unmodified, for a chooser class — it was never actually supersession-specific, only
untested against one. What is missing is a small, uniform second primitive:

```rust
/// True when `input` holds a recorded selection on `pool_choice_id` whose
/// `selection_id` is `option_id`, AND `option_id` is drawn from the real,
/// corpus-declared option pool for this tier (never inferred, never a
/// generated placeholder) -- the second half is what stops a
/// hand-maintained id list from silently drifting away from the corpus,
/// the exact failure mode `equipment_keys` once had (see
/// `archetype_resolver.rs`'s own "Aggregation, not a new parallel list"
/// framing).
pub fn chooser_option_selected(
    input: &CharacterInput,
    pool_choice_id: &str,
    option_id: &str,
    corpus_pool: &[&str],
) -> bool
```

Whether a *selected* option grounds (has a real formula) is then, as today, each option's own
hand-built function — `ground_or_block_arcanist_metamagic_knowledge`,
`ground_oracle_tier_one_revelations`, and Sorcerer's per-bloodline blocks are not replaced by this
primitive, they are what it composes with. The primitive only removes the two failure modes that are
otherwise easy to introduce by hand: (a) checking a tier that has actually been archetype-swapped (fixed
by reusing `archetype_claims_slot` first), and (b) accepting a `selection_id` that is not a real corpus
option (fixed by `corpus_pool` being sourced from the same rules_tables catalogs `archetype_resolver.rs`
already chains, never a fresh hand-typed list).

**Tradeoff accepted:** Design B does not produce a single number describing "how done" a chooser class
is, the way `archetype_claims_slot` does for the 25 supersession classes. There is no way to avoid this
honestly — "how done is Sorcerer" depends on which specific bloodline a character takes, and collapsing
that into one program-wide percentage is exactly the blended-percentage anti-pattern Decision 34/64
already rules out. The `wired_able`/`named` figures below are therefore reported **per option-pool**
(mysteries wired / mysteries named, bloodlines wired / bloodlines named, exploits wired / exploits
named), matching the granularity the corpus itself uses, not a further-collapsed single fraction.

## Measurement under this design (this cycle's own re-derivation)

| class | book measured | pool | wired / named |
|---|---|---:|---:|
| Oracle | advanced_players_guide | mysteries | 5 / 10 (6 tier-1 revelations grounded; Lore mystery has 2) |
| Arcanist | advanced_class_guide | exploits | 1 / 46 |
| Sorcerer | core_rulebook | bloodlines | 2 / 10 (floor — at least 31 known corpus-wide across 5 of 23 books checked, see clearance table) |

Full commands and per-option detail: `SD31-E3-F1-001-clearance-table.json`'s
`chooser_based_classes` block.

## What this unblocks

Once `chooser_option_selected` lands (Epic 4's job, not this card's), the option-pool figures above
become directly actionable the same way the 25 supersession classes' slot figures are: Epic 4 can wire
one option at a time (one bloodline, one mystery's revelation, one exploit) without re-deriving the
availability half each time, and Epic 5's chassis-sweep can report per-option `done` status instead of
leaving the whole class permanently `unmeasurable`.

## What this cycle deliberately did NOT do

- Did not write `chooser_option_selected` — this is a design decision (F3's own acceptance criterion),
  not implementation; Epic 4 owns the build.
- Did not exhaustively count every mystery/bloodline/exploit across all 23 books — the per-class rows
  above name exactly which books were checked and flag the remainder as a known, reproducible gap (the
  Sorcerer bloodline floor in particular is known to undercount by at least 3x).
- Did not attempt a tier-collapse pass (matching Decision 64's Ranger/Cleric-style mechanism counting)
  on the chooser classes' option pools — a "collapsed" bloodline/mystery count is not a meaningful
  concept the way a collapsed archetype-slot count is, since each option is already a maximally-distinct
  real id (unlike `ChannelEnergy1..10` naming the same formula ten times).
