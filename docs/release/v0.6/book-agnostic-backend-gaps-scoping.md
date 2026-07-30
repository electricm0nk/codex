# Book-agnostic backend gaps — scoping

> Author: Claude (SD-27 session, `tranche/7`). Not a build authorization — scoping only, per
> operator request. Three real, code-verified gaps found while executing SD-27 (Advanced Race
> Guide + Pathfinder Unchained content ingestion) that contradict the "once v0.6 closes wiring
> gaps, remaining future-state books are simple number crunching" assumption. All three block that
> assumption from holding for **any** future book, not just ARG/PU.

## Context

SD-27 ingested 2 books' worth of spell/equipment/feat data as pure Shape B v1 JSON — no new
engine code, per its own scope-draft. That data-only ingestion nonetheless surfaced three places
where the compute engine is CRB-only or otherwise not actually book-agnostic, despite the
architecture intending it to be (see `equipment_id_resolve`'s own doc comment: it searches the
*whole* corpus, book-agnostic, by design). All three were found via real usage (PCGen parity
testing, direct LST source inspection), not speculation.

## Finding 1 — Equipment weight resolution is hardcoded to CRB

**Severity: Low effort, concrete, ready to build.**

`src/rules_core/encumbrance.rs:159-192` (`compute_encumbrance`):

```rust
let Some((record, _table_cell)) =
    equipment_id_resolve(&selection.item_id, RuleSetId::Crb, corpus)   // <- book-agnostic already
else { ... };
let key = equipment_key_token(record).unwrap_or(&record.name).to_string();
let weight_lbs = equipment_tables()          // <- CRB-ONLY compiled static table (rules_tables::crb)
    .iter()
    .find(|entry| entry.key == key)
    .and_then(|entry| entry.weight_lbs);
```

`equipment_id_resolve` (`src/rules_core/equipment_resolver.rs:55-88`) already searches
`corpus.records_by_kind(SourceContentKind::Equipment)` — the *whole* loaded corpus, every book,
by design (its own doc comment). `RuleSetId::Crb` here is decorative — used only to build a
`TableCellRef` citation, not to filter which records are visible. **The record resolution step is
already book-agnostic.** The bug is the second, redundant lookup: instead of reading weight off
the resolved `record` directly, the code re-looks-up the item by key in
`rules_tables::crb::equipment_tables()` — a table compiled at build time from CRB's own LST only.
Any non-CRB item's key isn't in that table, so `weight_lbs` resolves to `None` and the item is
silently dropped into `unresolved_item_ids` (no weight counted, no error surfaced to the user).

**Verified live:** ARG's Dogslicer (`arg_equip_arms_armor.lst`) carries `WT:1` in its raw source
line — confirmed by direct read. SD-27's parity run showed PCGen: 30 lbs, Codex: 29 lbs for the
same build — **exactly** the Dogslicer's own 1 lb, confirming this is the sole cause, not a
rounding or multi-item issue.

**The fix is already available on the resolved record.** `EquipmentRecord`
(`src/pcgen_import/lst_parser/equipment.rs:220+`) carries `tokens: Vec<EquipmentToken>` — every
raw `KEY:VAL` pair from the source LST line, in order, for *every* book (the parser is
book-agnostic; only the compiled static tables are CRB-only). A `WT:` token is present on every
equipment record with a real weight, for every book, because it's parsed generically at ingestion
time.

**Proposed fix:** in `compute_encumbrance`, after resolving `record`, read weight directly from
`record.tokens` (find the token with `key == "WT"`, parse its `value` as `f64`) instead of the
second `equipment_tables()` lookup. Falls back to `unresolved_item_ids` only if the record itself
has no `WT:` token (a real data gap, not a book-scoping gap). Remove the now-unused
`equipment_tables()` import once cutover is verified against the existing CRB parity fixtures
(zero regression expected — CRB records carry the same `WT:` token, just via a different, slower
path today).

**Blast radius:** `src/rules_core/encumbrance.rs` only. No `rules_tables/mod.rs` touch needed
(unlike Findings 2/3), so this doesn't hit the file-touch-partition constraint SD-27's own cycles
ran into. Existing `tests/sd27_advanced_race_guide_parity.rs` / `sd26_pcgen_runner`-family tests
should flip the Dogslicer mismatch from fail to pass and serve as the regression proof.

## Finding 2 — Race roster is a hardcoded 7-variant enum, not corpus-driven

**Severity: Medium effort, small blast radius, needs a design decision.**

`src/rules_core/rules_tables/crb/race_tables.rs:22-31`:

```rust
pub enum RaceId { Human, Dwarf, Elf, Gnome, HalfElf, HalfOrc, Halfling }
```

Only 2 files reference `RaceId` in the whole codebase: its own definition file, and
`src/rules_core/pilot_compute.rs:10703-10711`, which does a flat string match:

```rust
"race:human" => RaceId::Human,
"race:dwarf" => RaceId::Dwarf,
... // 7 arms total, one per core race
```

Race *trait recognition* (`race_traits()`, same file) is generically structured — not the
problem. The problem is the *roster itself*: a new race (e.g. ARG's Goblin, Grippli, etc. — real
new playable races the book adds) has no path to exist as a `RaceId` without a code change adding
a new enum variant + a new match arm + real per-race trait data. This is exactly why SD-27
excluded ARG's race content from ingestion entirely (see `advanced_race_guide_pre_build`-cycle
receipt) — there was nowhere generic to put it, matching this finding precisely.

**Two fix shapes, not yet decided between:**

1. **Minimal — keep the enum, extend it per book.** Add `RaceId` variants + match arms for each
   new book's races as that book's race content gets scoped (mirrors how classes are handled
   today — real, book-specific engine work per addition, not automatic). Low risk, proven pattern,
   but doesn't remove the "new book needs a code change" cost — contradicts "simple number
   crunching" for races specifically, permanently.
2. **Structural — make the race roster corpus-driven.** Resolve races generically the same way
   `equipment_id_resolve` already does for equipment: a `race_id_resolve(&str, &corpus) ->
   Option<&RaceRecord>` pulling from a Shape B `race` content-kind (which does not exist yet — see
   Finding 3, since Shape B has never carried a `race` content-kind for any book, including CRB).
   Real trait *values* (ability bonuses, size, speed) would need a corpus-derivable shape — PF1
   race traits are often flat, numeric, and directly transcribable from `.lst` `ABILITY:`/`BONUS:`
   tokens (verified: CRB's own 7 races' traits are simple enough that `race_traits()`'s existing
   entries look directly derivable from the corpus already). This removes the recurring
   per-book code-change cost permanently, but is real engine + schema work, not a one-file patch.

**Recommendation:** don't decide this in isolation — it's the same underlying question as Finding
3 (is there a `race` Shape B content-kind or not). Scope both together.

## Finding 3 — No Shape B content-kind exists for class-ability-formula content, for any book

**Severity: Largest, needs its own design pass before estimating effort.**

Both ARG and PU hit the identical exclusion, independently, for the identical reason:

- ARG pre-build receipt: race-builder / racial-ability-formula content excluded — "no book in this
  codebase, including Core Rulebook itself, has ever represented that content shape in Shape B
  JSON."
- PU pre-build receipt (`artifacts/epic_2/pathfinder_unchained_pre_build-cycle_receipt.md:39-42`):
  `pu_abilities_class.lst` (1,344 real lines) excluded — **"same PCGen ability/BONUS/DEFINE/PREREQ
  formula-engine syntax as ARG's equivalent file, no precedent anywhere in the codebase."**

**What's actually in that PU file, confirmed by direct read of the real source** (not assumed from
the receipt alone): predominantly `.MOD` patches onto existing class chassis (424 of ~1,344 lines
are `.MOD` entries) plus `DEFINE:`-only optional-rule blocks (fractional BAB, Eidolon subtypes,
etc.) — this is Pathfinder Unchained's headline content, the actual "Unchained Barbarian/Monk/
Rogue/Summoner" alternate class rules and optional variant-rule modules. **None of it is in the
Shape B corpus today.** PU's own `content_kind_counts` is `{"feat": 17, "equipment": 42}` only —
the book's own defining content isn't represented at all, the same way ARG's races aren't.

This is not a per-book gap. It's a missing content-kind in the schema itself, shared by:
- ARG's race-builder formulas (Finding 2's real fix path, option 2)
- PU's unchained-class variant formulas
- Any future book with alternate class features, archetypes with mechanical formulas, or new
  optional rules (Occult Adventures' new base classes and most of the Ultimate line will hit this
  same wall)

**This needs a real design pass, not a scoping estimate here:** the PCGen `BONUS:`/`DEFINE:`/
`PREREQ:` formula-engine syntax is a small interpreter's worth of surface area (variable
definitions, conditional bonuses, prerequisite trees) — genuinely more than a schema field
addition. Recommend a dedicated scoping cycle that answers, at minimum: (a) is a literal
formula-engine interpretation needed, or can specific common patterns (flat bonuses, simple
gated grants) be modeled the same "parallel copy, not shared function" way v0.6's own class
closures already do it per-class; (b) what's the real size of the problem across all 19 deferred
future-state books, not just PU (**answered below, 2026-07-30**); (c) does this block SD-28+
entirely or can it be deferred per-book the way v0.6 already defers permanently-blocked classes.

**(b) Real sizing, direct read of the source corpus (not estimated):** this problem spans nearly
every deferred book, in more shapes than just "class abilities":

| Book | Formula file(s) | Real lines |
|---|---|---|
| Ultimate Combat | 24 files (archetypes, etc.) | 3,303 |
| Ultimate Magic | 3 files | 2,819 |
| Occult Adventures | 5 files — **includes genuinely new base classes** (Kineticist, Medium, Mesmerist, Occultist, Psychic, Spiritualist), not just alternate features | 2,304 |
| Ultimate Wilderness | 7 files | 2,017 |
| Ultimate Intrigue | 5 files | 1,614 |
| Horror Adventures | 6 files | 1,066 |
| Ultimate Campaign | 3 files — **traits, drawbacks, retraining rules**, a different shape than class abilities | 449 |
| Mythic Adventures | 1 file | 363 |
| Monster Codex | 4 files | 278 |
| Bonus Bestiary | 1 file | 52 |
| Ultimate Equipment | 1 file (`ue_abilities.lst`, general item abilities, not class-scoped) | 91 |
| ARG + PU (already known) | 2 files | ~2,688 |

**~17,000 real lines total**, not a handful of edge cases. Occult Adventures alone raises the
stakes past "alternate class features" into "the schema needs to represent entirely new base
classes" — a strictly bigger ask than anything ARG/PU needed. Ultimate Campaign shows the content
isn't even uniformly "class-ability" shaped — traits/drawbacks/retraining are a structurally
different PF1 mechanic that would need its own content-kind, not a shoehorn into whatever Finding
3's eventual schema covers for classes.

**(c), informed by (b):** given the scale and the real shape diversity found, recommend NOT
attempting a single universal formula-interpreter up front. Better fit with this codebase's own
proven pattern (v0.6's "parallel copy, not shared function" per-class closures): scope and land
book-by-book, feature-by-feature, as each future-state book comes up for real ingestion —
deferring cleanly (named gap, not silently dropped, matching how ARG/PU's exclusions were handled)
rather than blocking all 17 remaining books on one big interpreter landing first.

## Sequencing recommendation

1. **Finding 1 (equipment weight)** — small, isolated, no design decision needed, no partition
   conflict. Safe to build immediately as its own cycle.
2. **Findings 2+3 (race roster, class-ability-formula content-kind)** — scope together as one
   design pass before building either; they're the same underlying schema question, and Finding
   2's real fix depends on Finding 3's answer.
3. Recommend routing 2+3's design pass alongside — not blocking — the paused v0.6 class-breadth
   closure, since it's the same category of "generic engine vs. per-thing hardcoding" question
   the operator is already reviewing there.
