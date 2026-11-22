---
title: SD-22 Content-Source Ingest — Process Doctrine
status: active (corrected 2026-07-19 — rewritten to match the proven pipeline; supersedes the 2026-07-19 initial version's fictional API examples)
scope: docs/release/SD-22
artifact_type: process-doctrine
canonical_branch: tranche/5
purpose: |
  How a coding harness ingests SD-22's content-source from real PCGen
  `.lst` data into hand-transcribed Rust chassis modules + cycle
  artifacts. Every Epic 3 / 4 / 5 / 6 cycle reads this file before RED
  phase.
date: 2026-07-19
companion_to: ../corpus-source-inventory.md
mirror_of: ~/workspace/programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/ingest.md
---

# SD-22 Content-Source Ingest — Process Doctrine

This file is the **canonical ingest pipeline** for SD-22's content-source work. Every Epic 3 / 4 / 5 / 6 cycle reads `corpus-source-inventory.md` (the per-content-unit four-tuple), then this `ingest.md` (the per-cycle pipeline), then runs the pipeline's commands.

**Corrected 2026-07-19.** This file's original version (authored the same day, before four real ingest cycles landed) described a fictional pipeline: a `rules_core::corpus::parse_lst` function and `SourcePackageContent`-based corpus loader that do not exist in this codebase, plus an "operator-supplied stub-swap" workflow (gitignored licensed files swapped in at cycle-launch). Four real cycles (Alchemist `9c187a7`, Cavalier `675ca65`, Inquisitor `a18e73b`, Oracle `aa9b924`) proved a simpler pipeline that needs none of that indirection, because the real corpus is already directly reachable — locally at `/home/ubuntu/workspace/repos/pcgen/data/`, and in a cloud sandbox by adding `https://github.com/PCGen/pcgen` as a second git source (`decisions.md §5`). This rewrite documents what those four cycles actually did. The `operator-supplied/` gitignored slot (§5 below) is kept as a **documented fallback only**, for a future book that genuinely isn't in the public PCGen corpus — it is not part of the default pipeline.

## 1. What "ingest" means here

For a single content unit (e.g. APG class Alchemist), the ingest cycle produces three artifacts plus one registration:

1. **A Rust module** at `<rust_module_path>` (e.g. `src/rules_core/rules_tables/apg/class_alchemist.rs`) — a small, hand-transcribed constants module. It does **not** parse the `.lst` file at runtime; the LST record is read once, at authoring time, and its BAB/save formula tokens are transcribed directly into Rust functions, with the source file + line + exact token cited in the module's doc comment (mirroring `rules_tables::crb::class_tables`'s established scope boundary and provenance convention).
2. **A test fixture** at `<test_fixture_path>` (e.g. `tests/sd22_apg_class_alchemist_resolves.rs`) — asserts the chassis resolves correctly at a boundary level (1), a representative level, and past `MAXLEVEL`; asserts the cross-book invariant (`RuleSetId::Apg` → `Some`, every other `RuleSetId` → `None`); and carries one `#[ignore]`-gated *real-corpus grounding test*, opt-in via `PCGEN_CORPUS_ROOT`, that re-reads the real `.lst` line and asserts the hand-transcribed constants still match it (this is what keeps the module tied to the source instead of to memory).
3. **A registered `ApgClassId` variant** (or `AcgClassId` / the Bestiary 1 equivalent) wired into the book's `class_chassis_resolve` dispatcher in `<book>/mod.rs`.
4. **A cycle artifact** at `<cycle_artifact_path>` (e.g. `docs/release/SD-22/artifacts/apg/class_alchemist_cycle_receipt.md`) recording the RED→GREEN transition.

The input is the real `.lst` file itself — no intermediate corpus-loader type, no stub file. `corpus-source-inventory.md`'s `corpus_input_path` column (once corrected per-row) should point directly at the real path, e.g. `pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst:CLASS:Alchemist`.

**Scope boundary (mirrors `rules_tables::crb::class_tables`):** only BAB/save chassis — a formula-derivable table — is transcribed per cycle. Named per-level features (Bombs, Discoveries, Mutagen, spell lists, etc.) require going back through the LST's per-level feature blocks (`apg_abilities_class.lst` and similar) in a dedicated future ingest slice; transcribing them without that would risk exactly the fabricated-data problem `AGENTS.md` and the CRB precedent both rule out. This is why `corpus-source-inventory.md`'s criterion 9 (spell/equipment resolution) stays open after a class's chassis criteria (6-8) land.

## 2. The per-cycle pipeline (RED → GREEN → REFACTOR)

Each step is mandatory per the operator-pinned 2026-07-19 red-green TDD mandate.

### 2.1 RED — verify the real record, then write the failing test

```bash
# 1. Locate the real .lst file. Locally:
ls /home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst
# In a cloud sandbox with only `codex` cloned, find the second git source's checkout:
find / -maxdepth 5 -iname 'pcgen' -type d 2>/dev/null

# 2. Read the real CLASS:<name> record directly — do not trust
#    corpus-source-inventory.md's "Content shape" prose column; it is
#    illustrative only (see that file's corrective banner).
grep -n "^CLASS:Alchemist" .../advanced_players_guide/apg_classes.lst

# 3. Confirm which of the two existing pcgen_import parsers should own
#    this class, by checking for SPELLSTAT/MEMORIZE/SPELLBOOK tokens:
#      - present  -> src/pcgen_import/lst_parser/spellcasting_class.rs
#      - absent   -> src/pcgen_import/lst_parser/class.rs
#    If the class name isn't yet in that parser's allowlist
#    (MARTIAL_CLASS_NAMES or SPELLCASTING_CLASS_NAMES), this cycle widens
#    it by exactly one name (see §6 below) as part of its own RED step.

# 4. Write tests/sd22_<book>_class_<class>_resolves.rs, mirroring
#    tests/sd22_apg_class_alchemist_resolves.rs's shape exactly:
#      - <class>_level_1_chassis_resolves_via_ruleset_<book>
#      - <class>_level_<maxlevel>_chassis_resolves_via_ruleset_<book>
#      - <class>_chassis_is_none_for_level_beyond_maxlevel_<N>
#      - <class>_chassis_returns_none_for_ruleset_crb  (cross-book invariant)
#      - a #[ignore]-gated PCGEN_CORPUS_ROOT test re-reading the real line

# 5. Confirm RED — fails because the ApgClassId variant / module doesn't exist yet
cargo test --locked --test sd22_apg_class_<class>_resolves 2>&1 | tail -40
```

The `cargo test` output (a compile error citing the missing variant/module, or a missing-record panic) is RED evidence. **Persist it** to the cycle artifact's "Red-phase evidence" section. If the test fails for an *un*intended reason, that's a Bucket-B shortfall — fix the test setup, don't carry the cycle forward.

### 2.2 GREEN — transcribe the chassis, wire the resolver

```bash
# 1. Write the production module, transcribing the BAB/save formula
#    tokens directly from the real line read in RED step 2. Mirror
#    src/rules_core/rules_tables/apg/class_alchemist.rs's shape exactly:
#      - a module doc comment citing the source file, line, and the
#        exact BONUS:COMBAT / BONUS:SAVE / MAXLEVEL tokens
#      - a MAX_SUPPORTED_LEVEL constant from the real MAXLEVEL token
#      - base_attack_bonus(level) / save_bonus(level, good) functions
#        implementing the real formula (e.g. level*3/4 for three-quarter
#        BAB, level/2+2 for a good save, level/3 for a poor save)
#      - a class_table() -> Vec<ClassTableRow> function

# 2. If this class's name isn't yet recognized by the pcgen_import
#    parser identified in RED step 3, widen that parser's allowlist by
#    exactly one name (see §6) — this is a small, bounded, per-cycle
#    widening, not a parser rewrite.

# 3. Register the new ApgClassId variant and match arm in
#    src/rules_core/rules_tables/apg/mod.rs's class_chassis_resolve.

# 4. Confirm GREEN — full tests, clippy clean
cargo test --locked 2>&1 | tail -20
cargo clippy --locked --tests -- -D warnings 2>&1 | tail -20
```

The output is GREEN evidence. **Persist it** to the cycle artifact's "Green-phase evidence" section.

### 2.3 REFACTOR (optional; only after green)

Refactor is permitted only after a cycle's GREEN phase. A cycle that refactors first is a Bucket-B shortfall (the cycle artifact must show RED → GREEN in that order; refactor moves are post-GREEN with `cargo test --locked` + clippy held green throughout).

```
# Common refactor operations:
#  - extract a shared helper for a formula shape reused across classes
#    (e.g. a shared three_quarter_bab(level) free function)
#  - update the cross-book-invariant table in corpus-source-inventory.md
#    if the invariant was mis-stated (operator-pinned at end of cycle)
```

### 2.4 MINT the cycle artifact

```bash
cat > docs/release/SD-22/artifacts/apg/class_alchemist_cycle_receipt.md << 'EOF'
# Alchemist cycle receipt — 2026-07-19T13:14:28Z

## Red-phase evidence
cargo test --locked --test sd22_apg_class_alchemist_resolves 2>&1 | tail -40
(…fails: E0433, ApgClassId::Alchemist does not exist yet…)

## Green-phase evidence
cargo test --locked 2>&1 | tail -20
cargo clippy --locked --tests -- -D warnings 2>&1 | tail -20
(…all green, clippy clean…)

## Files touched
- src/rules_core/rules_tables/apg/class_alchemist.rs (NEW)
- src/rules_core/rules_tables/apg/mod.rs (MODIFIED; ApgClassId::Alchemist + match arm)
- src/pcgen_import/lst_parser/spellcasting_class.rs (MODIFIED; SPELLCASTING_CLASS_NAMES widened by one)
- tests/sd22_apg_class_alchemist_resolves.rs (NEW)
- tests/sd17_b_spellcasting_class.rs (MODIFIED; real-corpus grounding test for the widening)

## Cycle metadata
- cycle_id: 2026-07-19T13:14:28Z
- bundle_criterion: criteria 6-8 (APG per-class cycles)
- corpus_input_path: pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst:CLASS:Alchemist (real corpus; decisions.md §5)
- RuleSetId: Apg

## kanban
- card: no card (hermes unavailable in this session; see receipts.md / progress.md)
EOF
```

A cycle that ships without a cycle artifact is a Bucket-B shortfall — Epic 9's evaluator (criterion-31) cannot conclude the criterion `complete` without the receipt existing with RED→GREEN transitions persisted.

### 2.5 COMMIT + PUSH

```bash
git add src/rules_core/rules_tables/apg/class_alchemist.rs \
        src/rules_core/rules_tables/apg/mod.rs \
        src/pcgen_import/lst_parser/spellcasting_class.rs \
        tests/sd22_apg_class_alchemist_resolves.rs \
        tests/sd17_b_spellcasting_class.rs \
        docs/release/SD-22/artifacts/apg/class_alchemist_cycle_receipt.md \
        docs/release/SD-22/progress.md

git -c user.name='Todd Hintzmann' \
    -c user.email='todd@hintzmann.net' \
    commit -m "feat(sd22): APG Alchemist class chassis lands (criteria 6-8)"

git push origin tranche/5
```

## 3. Cross-book resolution (Epic 3+4+5+6 cross-cutting cycles)

Cross-book-invariant cycles (Epic 3 criteria 8, Epic 4 criteria 12, Epic 5 criteria 16) assert that a class chassis resolves `Some` for its own book's `RuleSetId` and `None` for every other one — see `alchemist_chassis_returns_none_for_ruleset_crb` in `tests/sd22_apg_class_alchemist_resolves.rs` for the established shape. No corpus-loader object is threaded through these tests; each book's `mod.rs` exposes its own `class_chassis_resolve(class_id, level, rule_set)` function that early-returns `None` when `rule_set` doesn't match the book.

## 4. Epic 6 — DM Toolkit happy-path integration

Epic 6's criterion 21 is the load-bearing surface for Epic 9's evaluation: it consumes Epic 3+4+5 output into a campaign-shape fixture and runs the DM-toolkit encounter math against it. It calls each book's `class_chassis_resolve` / the Bestiary 1 monster resolver directly — the same real, already-landed modules Epic 3/4/5 cycles produce, not a separate corpus-loader abstraction. This test is RED until Epic 3+4+5 ship at least one ingested class chassis and one ingested monster block. Epic 6's cycle picker enforces this dependency per `loop-instruction.md` Step 1.

## 5. Fallback: operator-supplied corpus (only if the real corpus is unreachable)

The default pipeline (§§1-4) reads the real PCGen corpus directly and needs no operator action beyond the one-time setup in `decisions.md §5` (local sibling repo, or a second git source in a cloud sandbox). The `docs/release/SD-22/artifacts/corpus/operator-supplied/` slot exists **only** for the rare case where a future book genuinely isn't in the public PCGen corpus (`https://github.com/PCGen/pcgen`) and no other reachable source exists. In that case, and only then:

1. The operator places the licensed file at `docs/release/SD-22/artifacts/corpus/operator-supplied/<book>/<file>.lst` (gitignored — EULA content never commits).
2. The cycle reads that path instead of the public corpus for that one content unit, following the same §§1-4 pipeline otherwise (still no `SourcePackageContent`/`parse_lst` — hand-transcribe from whatever file is actually there).
3. The module's doc-comment provenance notes the operator-supplied source instead of the public corpus path.

APG, ACG, and Bestiary 1 are all confirmed present in the public PCGen corpus (`decisions.md §5`), so this fallback is not expected to be needed for SD-22's current scope.

## 6. Widening a `pcgen_import` parser allowlist (bounded, per-cycle)

`src/pcgen_import/lst_parser/class.rs`'s `MARTIAL_CLASS_NAMES` and `spellcasting_class.rs`'s `SPELLCASTING_CLASS_NAMES` are hard allowlists (originally the 11 CRB classes only). A cycle whose class isn't yet in either list widens the correct one by **exactly one name**, per `loop-instruction.md`'s file-touch partition, and adds a small real-corpus-gated test proving the widening (mirroring `parses_real_alchemist_record_from_apg_classes_lst` / `parses_real_inquisitor_record_from_apg_classes_lst` in `tests/sd17_b_spellcasting_class.rs`). This is not a parser rewrite — the underlying tab-delimited `KEY:VAL` tokenizer is already name-agnostic; only the scope filter changes.

## 7. Where this pipeline lands in the bundle

- **`corpus-source-inventory.md`** is the per-cycle "what to ingest + where to land it" reference (routing columns authoritative; "Content shape" prose illustrative only).
- **`ingest.md`** (this file) is the per-cycle pipeline.
- **`loop-instruction.md`** Step 4-5 enforces the red-green TDD mandate + cross-references this `ingest.md`, and documents the §6 parser-widening pattern in its file-touch partition.
- **`decisions.md §5`** is the corpus-sourcing decision record (corrected 2026-07-19).
- **`artifacts/corpus/`** retains the on-disk file-shape stubs as schema documentation (not a required input to the default pipeline).
- **`artifacts/corpus/operator-supplied/`** is the fallback-only slot per §5 above.

## 8. Recorded

Corrected 2026-07-19 to match the pipeline four real ingest cycles proved (Alchemist, Cavalier, Inquisitor, Oracle), after the original same-day version's fictional `parse_lst`/`SourcePackageContent` API examples and default operator-supplied-swap workflow were found to not match the codebase.

## 9. Per-content-type extensions (operator directive 2026-07-19)

The default §1-§7 pipeline covers class chassis (`ApgClassId`/`AcgClassId`/`Bestiary1` enum + match-arm-dispatch chassis modules) and spell lists. For the additional content types the operator named (`races`/`mitems`/`spells`/`feats`/etc.), the same pipeline shape applies — the only per-content-type differences are (a) the resolver enum variant, (b) the file-shape stub location, and (c) the module name. Cross-book invariants follow the same `RuleSetId::*::resolve(key, corpus) -> Option<...>` shape as classes; product-visible per cycle.

### 9.1 Races

- **Module shape**: `src/rules_core/rules_tables/{apg,acg}/race_<lowercase>.rs` per race. Each module exposes `pub struct Race<...>` (size, base_speed, ability_mods, traits, favored_class_options) plus `pub fn rule_set_id(&self) -> RuleSetId` for cross-book dispatch.
- **Resolver enum**: races share the book's `RuleSetId` (no separate `RaceSetId`); key shape: `<apg|acg>:race:<lowercase-race-name>`. Resolver chain lives in `<book>/mod.rs`'s `race_resolve`.
- **Cross-book invariants**: see `corpus/races/{apg,acg}_races.lst.md` §"Notes."
- **Test fixture shape**: `tests/sd22_<book>_race_resolves.rs` (batched, multi-race assertions). Mirrors §2.4 conventions: boundary + representative + cross-book + `#[ignore]`-gated real-corpus grounding.
- **Cycle artifact path**: `docs/release/SD-22/artifacts/races/<book>_<race>_cycle_receipt.md` (per-class artifact per cycle; can batch 1-3 races per cycle artifact as documented in `corpus-source-inventory.md` §7).

### 9.2 Magic items

- **Module shape**: per-aisle (wondrous / weapons / armor / etc.). Aisle is too small to need a per-item Rust module; one `magic_items_<book>.rs` per book that exposes a `lookup` function keyed on `<book>:mitem:<key>` and a strong-typed return `Mitem` enum per aisle.
- **Resolver dispatch**: aisles are subrouted under the book's existing rule-set chain (`RuleSetId::*::resolve("apg:mitem:cape-of-feathers", ...)`).
- **Test fixture shape**: per-aisle assertion. Boundary check is "no boundary" (magic items have no level progression), so the test asserts (a) `Some(Mitem::Wondrous(_))` for the aisle's top-3 keys, (b) `None` for cross-book fall-through, (c) `#[ignore]`-gated real-corpus grounding.
- **Cycle artifact path**: `docs/release/SD-22/artifacts/magic-items/<book>_<aisle>_cycle_receipt.md` (one per aisle group; 6 total in the per-inventory §8 plan).

### 9.3 Feats

- **Module shape**: `feats_<book>.rs` per book; categories (combat, item-creation, racial, convergence) are sections within one module file. Per-feat `pub struct Feat { key: FeatKey, category: FeatCategory, prereq: PrereqExpr, benefit_summary: &'static str, description_key: DescriptionKey }`.
- **Resolver dispatch**: `<apg|acg>:feat:<lowercase-feat-name>`. Book-local lookup via `feats_<book>::lookup(key)`.
- **Test fixture shape**: per-category assertion. Many feats have prerequisite-boolean-expression requirements (`PrereqExpr::Bab(8)` etc.); the parser must surface unparsable prereqs as `PrereqExpr::Unknown(SyntaxError)` rather than fail the cycle. Tests assert (a) one canon feat per category returns `Some`, (b) cross-book returns `None`, (c) `#[ignore]` real-corpus grounding.
- **Cycle artifact path**: `docs/release/SD-22/artifacts/feats/<book>_<category>_cycle_receipt.md` (8 categories total in the per-inventory §9 plan).

### 9.4 Archetypes

- **Module shape**: per-class + per-archetype; `src/rules_core/rules_tables/{apg,acg}/archetype_<class>_<arch>.rs`. Mirrors the per-class cycle shape but with *archetype-feature-swap* semantics: at level N the archetype replaces the parent's level-N feature with its own. The module's `feature_at_level(level: u8, ctx: &FeatureContext) -> ClassFeature` returns the swap or the parent's.
- **Cycle pacing**: per-archetype cycle (smaller scope than full-class cycles). Per the inventory §10, archetype cycles are extension-Epic 3/4 work that lands after the primary 31-criteria loop closes; archive numbering deferred until that.
- **Test fixture shape**: per-archetype one-shot `tests/sd22_<book>_archetype_<class>_<arch>_resolves.rs`. Three assertions: parent-feature-at-level-1 doesn't show when archetype-feature-at-level-1 takes over; cross-book invariant; compat-column check (e.g. Vivisectionist rejects Chirurgeon multi-archetype).
- **Cycle artifact path**: `docs/release/SD-22/artifacts/archetypes/<book>_<class>_<arch>_cycle_receipt.md` (~46 files across APG 22 + ACG 24; can be batched per-class).

### 9.5 Monster abilities (Bestiary 1 only)

- **Module shape**: `src/rules_core/rules_tables/beastiary1/monster_abilities.rs`. One `pub enum AbilityKind { Ex, Su, Sp, DamageResistance }`; per-ability `pub struct MonsterAbility { key, kind, save_dc_or_none, damage_dice_or_formula, trigger, source_class_feature_id }`.
- **Resolver dispatch**: `beastiary1:ability:<lowercase>`. Bestiary-only — no cross-book aliases. Lookup via `monster_abilities::lookup(key)`.
- **Test fixture shape**: per-ability-kind assertions. Cross-book check is the trivially-true `None` for non-Bestiary rulesets (ensures Bestiary-only).
- **Cycle artifact path**: `docs/release/SD-22/artifacts/monster-abilities/<kind>_cycle_receipt.md` (4 kinds total).

### 9.6 Monster templates (Bestiary 1 only)

- **Module shape**: `src/rules_core/rules_tables/beastiary1/monster_templates.rs`. `pub enum TemplateFamily { Undead, Construct, DragonDisciple, Noble, /* ~50 families total */ }`; per-template `pub struct Template { key, family, cr_mod: i8, hp_mod: HpModifier, feature_swap_summary, curse_or_affliction}`.
- **Resolver dispatch**: `beastiary1:template:<lowercase>`. Bestiary-only.
- **Test fixture shape**: per-family assertion. The schema's CR modifier column is `i8` (-1 to +4 typical range); verify the parser surfaces unparseable values as `Template::ParseError(SyntaxError)` rather than fail the cycle.
- **Cycle artifact path**: `docs/release/SD-22/artifacts/monster-templates/<family>_cycle_receipt.md` (4 families for current scope; extensible).

### 9.7 When a content type doesn't fit the pipeline's class-shape

For any content type whose struct shape doesn't fit the §1 chassis pattern (e.g. an item with internal state, or a shape that crosses multiple `RuleSetId`s without a primary owner), the operator logs `## Open judgments deferred to next SD` per Epic 9's evaluator and continues. The §"Operator-judgment-call rule" governs: don't fabricate. Don't over-extend. The next bundle picks up the deferred items.

## 10. Recorded

Authored 2026-07-19 per operator directive ("full coverage for every content type: races, classes, mitems, spells, feats, etc.; no stub-only ingest; expected per content type"). §9 extends §1-§7's class+spell pipeline to cover races (per-book module + book-local resolver key), magic items (per-aisle module per book), feats (per-book module with category sections), archetypes (per-class+archetype module with feature-swap semantics), monster abilities (Bestiary-only), monster templates (Bestiary-only). Cross-book invariants per stub `corpus/<content-type>/<book>_<type>.lst.md` §"Notes." Companion stub surfaces at `corpus/{races,magic-items,feats,archetypes,monster-abilities,monster-templates}/<book>_<type>.lst.md` (12 new files committed 2026-07-19).
