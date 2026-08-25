# Cycle t12-class-feature-pool-population, cycle 7 — Gate 3 (closure invariant) / row 18 (`epic-8-pool-shaped-class-features`)

- **Card ID:** row 18 (`epic-8-pool-shaped-class-features`), governed by `decisions.md §27b`
  (*"EVERYTHING"*). Dispatched scope: investigate the real remaining blocker cycle 6 named
  (`PREABILITY`/`PREMULT` header gates), checking `feat_prereqs`/`choice_selection` first per
  `§17` before building; wire Hunter Animal Focus and/or Cavalier Order (cycle 5/6 named, not
  attempted); leave Oracle Mystery withdrawn.
- **Base:** `git merge-base --is-ancestor "$PIN" HEAD` returned FALSE at session start
  (`PIN=265ec7ca0a9ad47499d3be61fdd2aa5c8517a607`, worktree started on a stale lineage). Fixed:
  `git reset --hard "$PIN"` then `git rebase origin/tranche/12` (fast-forward — `origin/tranche/12`
  == `$PIN` after fetch). `BASE_OK` re-verified after.
- **Oracle:** fresh worktree, empty git-ignored slot as expected. Bootstrapped via
  `scripts/fetch-pcgen-oracle.sh --dest <repo-local artifacts/corpus/operator-supplied/pcgen>`;
  `scripts/verify.sh --only preflight-oracle` → `PASS` (pin `7f818006e371`).
- **Files touched:**
  - `src/rules_core/pilot_compute/mod.rs` — `real_pool_group_for_selection_slug` widened with a
    THIRD real naming/ownership shape (see §2 below); `resolve_pool_member_sole_magnitude` gained
    an `owning_class_override: Option<&str>` parameter (see §3); new Cavalier Order call site in
    `ground_cavalier_named_features`, purely additive alongside the existing hand-modelled
    `ORDER_OF_THE_SWORD_SELECTION` branch; `ability_modifiers` threaded through
    `compute_apg_class_chassis` -> `ground_cavalier_mount_and_defer_the_rest` ->
    `ground_cavalier_named_features` (previously absent at this call depth — mechanical signature
    widening, no behaviour change to any existing caller); 3 new tests in
    `generic_pool_group_selection_wiring_tests`.
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 18 only (cycle field
    5 → 6, Notes appended). Rows 19/20 left untouched and still parse: exactly one `^| 18 |`,
    `^| 19 |`, `^| 20 |` row, 7 cells each, verified by script before commit.
  - `docs/retro/events/{sd31-transcribe,t9-onboarding}.jsonl` — auto-appended by
    `scripts/verify.sh` (2 derived `preflight-oracle` events for this worktree). Not hand-edited.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — `git diff --unified=0` over
  `pilot_compute/mod.rs`, `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'` → 0 hits.
- **Wired-integration audit result:** `OK_NO_TOKENS` — same scope, `grep -inE
  "STUB|MOCK|placeholder|not.?yet.?implemented|todo|fixme|hack"` → 0 hits.
- **PI audit:** `pi_scrub.normalized_term_hits(...)` against this cycle's full diff text (Rust
  file) → `[]` (0 hits). `data/corpus/**` untouched throughout (`git status --porcelain --
  data/corpus` — 0 changes).
- **Status:** in-progress — genuine, multi-cycle epic, unchanged from cycles 1–6's own framing.

---

## 1. `PREABILITY`/`PREMULT` header gates — investigated, real blocker found one hop deeper, zero groups closable this way

Per `§17`, checked what already exists before building anything: `feat_prereqs::pre_tokens`
already evaluates `PREABILITY`/`PREMULT` clauses for real, against a `CharacterPrereqFacts`
snapshot built from `CharacterInput` (`CharacterPrereqFacts::from_character`) — including
`CATEGORY=FEAT` items checked against `selected_feats` via `feat_identity`. No third prerequisite
evaluator was built.

Direct corpus tracing of the actual blocking chain cycle 6 named (Marid Bloodline's own header)
found the record-level `PREABILITY` gate itself is **not** what blocks resolution:

```
Marid Bloodline ~ Elemental Resistance  (data/corpus/ultimate_magic/class_feature/
                                          marid_bloodline/elemental_resistance.json)
  PREABILITY: 1,CATEGORY=Special Ability,Sorcerer Bloodline ~ Marid
  PREVARGTEQ: Sorcerer_Marid_BloodlineProgressionLVL,3
  BONUS:VAR|Sorcerer_MaridElemetnalResistance_ResistanceBonus|
      min(floor((Sorcerer_Marid_BloodlinePower3LVL+3)/6),2)*10
  BONUS:VAR|ColdResistanceBonus|Sorcerer_MaridElemetnalResistance_ResistanceBonus|TYPE=Resistance
```

The `PREABILITY` gate reads "does the character hold Sorcerer Bloodline ~ Marid" — but
`push_generic_pool_group_selection_magnitude` only ever iterates members of the group the player
has ALREADY recorded picking (`real_pool_group_for_selection_slug` resolved that group from the
player's own `selected_choices` entry). The gate is therefore satisfied BY CONSTRUCTION for every
member this resolver ever looks at; evaluating it would change nothing.

The real blocker is one hop deeper, in the var chain: `Sorcerer_Marid_BloodlinePower3LVL` needs
the HEADER record's own `BONUS:VAR|Sorcerer_Marid_BloodlinePower3LVL|Sorcerer_Marid_BloodlineLVL+
BloodlinePower3LVLBonus`, which in turn needs `Sorcerer_Marid_BloodlineLVL` (the header's own
`BONUS:VAR|Sorcerer_Marid_BloodlineLVL|BloodlineLVL`) and `BloodlinePower3LVLBonus` (never bound
anywhere in this corpus — a feat-granted bonus, correctly left unresolved). `BloodlineLVL` itself
is PCGen's real per-bloodline level tracker, bound via `BONUS:VAR|BloodlineLVL|SorcererLVL|
TYPE=Base` — but on an `ABILITY:Internal|AUTOMATIC|Bloodline Tracker` record, NOT the Bloodline
choice header. Tracing that record found a **corpus ingestion gap**, not a prerequisite-modelling
gap:

```
core_rulebook/cr_abilities_class.lst:1704-1707 (real PCGen source, 3 separate .MOD append lines
  on the SAME "Bloodline Tracker" ability):
    CATEGORY=Internal|Bloodline Tracker.MOD   BONUS:VAR|BloodlineLVL|SorcererLVL|TYPE=Base
    CATEGORY=Internal|Bloodline Tracker.MOD   BONUS:VAR|BloodlineCasterLVL|SorcererLVL|TYPE=Base
    CATEGORY=Internal|Bloodline Tracker.MOD   BONUS:VAR|BloodlineProgressionLVL|SorcererLVL|TYPE=Base

data/corpus/core_rulebook/class_feature/bloodline_tracker/bloodline_tracker.json (ingested):
    only BloodlineLVL survives -- BloodlineCasterLVL/BloodlineProgressionLVL are GONE.

data/corpus/{advanced_class_guide,advanced_players_guide,advanced_race_guide,monster_codex,
  occult_adventures,ultimate_combat,ultimate_magic}/class_feature/bloodline_tracker/
  bloodline_tracker.json (7 more books, SAME "Bloodline Tracker" key):
    each keeps a DIFFERENT single leftover DEFINE (one per book), confirming several real
    .MOD append-lines collided under one JSON key across books and only one field per file
    survived ingestion.
```

Bloodrager's own equivalent (`Bloodrager ~ Bloodline Tracker`, `acg_abilities_class.lst:355-363`,
7 real `.MOD` lines) lost the same way — the ingested JSON keeps only `BloodragerBloodlineBonusFeats`
(a `DEFINE`, not even a `BONUS:VAR`); all six numeric `BONUS:VAR` lines (`BloodragerBloodlineLVL`,
`...CasterLVL`, `...ProgressionLVL`, `...PowerTimes`, `...FeatProgression`) are gone.

Cleric/Shaman's own `DomainLVL`/spirit-LVL equivalent is a SEPARATE, larger gap still: its real
binding (`BONUS:VAR|DomainLVL|ClericLVL`, confirmed live at `cr_classes.lst:55`) sits on the CLASS
record itself. `data/corpus/*/class/*.json` (confirmed directly, `core_rulebook/class/cleric.json`)
carries only `class_id`/`maxlevel`/`bab`/`save_fort`/`save_ref`/`save_will` — no `raw_tokens` field
at all. This is out of `class_feature`'s ingestion scope entirely, not merely PRE-gated.

**Escalated here by name, per `§27b`'s own instruction** ("a cycle that believes it has found a
genuine impossibility escalates it by coordinate ... it does not write its own exemption"): fixing
PCGen `.MOD`-line merging, or adding `class`-record `raw_tokens` capture, is `data/corpus`-adjacent
ingestion work — forbidden to this cycle's guarded-path-only write scope (`data/corpus/**` is
never hand-edited) and a genuinely different, larger mechanism than any pool-resolver widening.
Not attempted.

**Measured effect: 0 groups unblocked** across Sorcerer Bloodline, Cleric Domain, Bloodrager
Bloodline, Shaman Spirit. No code was changed for this item — a real zero, reported honestly
rather than papered over with an unrelated diff.

## 2. Cavalier Order — wired via the SAME resolver, a THIRD real naming/ownership shape

Per `§17`, reused `push_generic_pool_group_selection_magnitude` unchanged — the exact mechanism
cycles 5/6 already proved for Sorcerer Bloodline/Cleric Domain/Warpriest Blessing/Shaman
Spirit/Bloodrager Bloodline. No new resolver was built.

`real_pool_group_for_selection_slug`'s existing two branches both assume `"<Adjective>
<RegisteredName>"` (a group ending in `" Order"`). Cavalier's real corpus groups are shaped
`"Order of the X"` instead — the mirror image. Direct corpus survey of every real, `" ~ "`-
qualified Cavalier group found 8 real orders (Beast, Guard, Eastern Star, Shroud, Blue Rose,
Green, Seal, Tome — `python3` one-off scan over `data/corpus/*/class_feature/**/*.json`), plus a
9th, "Order of the Sword", already hand-modelled via `ORDER_OF_THE_SWORD_SELECTION`. Of the 8:

- Beast, Guard, Eastern Star, Shroud — member records tagged `class: "Cavalier"` directly (the
  EXISTING majority-tally ownership check already passes these unaided).
- Blue Rose, Green, Seal, Tome — member records tagged `class` SELF-REFERENTIALLY with the
  group's own name (e.g. `"Order of the Blue Rose"`, never `"Cavalier"`). Only admitted because a
  real, independently-verified `"Cavalier Order ~ <Group>"` CHOOSER header record ALSO exists and
  is itself tagged `class: "Cavalier"` — confirmed live for all 4 (`Cavalier Order ~ Order of the
  Blue Rose`/`Green`/`Seal`/`Tome`, `data/corpus/*/class_feature/cavalier_order/*.json`) — never
  trusted from the member's own self-tag alone (that would let an unrelated same-shaped group from
  a different class/archetype through).
- Order of the Sword has NO `"Cavalier Order ~ Order of the Sword"` header at all (its members are
  tagged `class: "Order of the Sword"`, matching neither check) — correctly, safely excluded, no
  collision with the hand-modelled branch.

`real_pool_group_for_selection_slug` gained this THIRD branch (tried after the existing exact-
suffix branch, changing nothing for any pool already served by the other two shapes): a
case-insensitive `"<RegisteredName> of the "` prefix strip, plus an ownership fallback (`majority
class == class` OR a `"<class> <registered_name> ~ <group>"` header exists and is itself
`class == class`).

### 2a. A second, load-bearing fix: `owning_class_override`

Green's own resolvable member, `Order of the Green ~ Favored Terrain`:

```json
{"key": "Order of the Green ~ Favored Terrain", "class": "Order of the Green", ...
 "raw_tokens": [
   ...
   {"key": "BONUS", "value": "VAR|FavoredTerrainPool|(CavalierFavoredTerrainLVL+4)/6"},
   {"key": "BONUS", "value": "VAR|CavalierFavoredTerrainLVL|CavalierLVL"}
 ]}
```

A genuine, self-contained two-hop chain (`FavoredTerrainPool` <- `CavalierFavoredTerrainLVL` <-
seeded class level) needing no missing header var at all — EXCEPT `resolve_pool_member_sole_
magnitude` derives `class_level_var` from `record.class`, which here is the self-referential
`"Order of the Green"`, not `"Cavalier"` — `class_level_variable_name("Order of the Green")` =
`"OrderoftheGreenLVL"`, a var the formula never references, so `CavalierLVL` (what the formula
actually names) was never seeded and the chain silently refused despite being complete.

`resolve_pool_member_sole_magnitude` gained `owning_class_override: Option<&str>`, used
preferentially over `record.class` for BOTH the header-chain lookup and the class-level-var
derivation. `push_generic_pool_group_selection_magnitude` (the group-selection caller, which has
ALREADY independently verified the real owning class via `real_pool_group_for_selection_slug`'s
own majority-tally-or-header-ownership-proof) now passes `Some(class)`. `push_generic_pool_choice_
magnitude` (the flat-pool caller, which has never needed this and whose records are all reliably
tagged) passes `None`, preserving its exact original behaviour, unchanged.

### 2b. Real survey of all 8 groups' own numeric members

```
Order of the Beast        11 members, 0 numeric (BONUS:VAR or BONUS:SKILL)
Order of the Guard          5 members, 1 numeric -- BONUS:SKILL, Knowledge (nobility)
Order Of The Eastern Star   8 members, 0 numeric
Order Of The Shroud         7 members, 0 numeric
Order of the Blue Rose      7 members, 0 numeric
Order of the Green          7 members, 1 numeric -- BONUS:VAR, Favored Terrain (RESOLVES)
Order of the Seal           7 members, 0 numeric
Order of the Tome           7 members, 1 numeric -- BONUS:SKILL, Linguistics
```

Guard's and Tome's own numeric members are real (`BONUS:SKILL|Knowledge (nobility)|
max(1,(CavalierLVL/2))`, `BONUS:SKILL|Linguistics|CavalierLVL/2|PRESKILL:...`) but correctly
refuse: `parse_bonus_var_tokens_pre_gate_safe` only ever reads `BONUS:VAR` tokens (this module's
own documented, unchanged scope) — a `BONUS:SKILL`-only member is out of scope, the same shape as
every other pool's own documented refusals, not a new gap. The other 5 groups' every member is
pure DESC-only display text — real `§7` DONE already (the same correction cycle 6 made for
Warpriest Blessing), not a resolver gap.

**Real NEW closure this cycle: 1 Cavalier Order group (Order of the Green's own `Favored Terrain`
member) now reaches `compute_pilot_base_chassis` -> `compute_class_chassis` with a genuinely
computed magnitude from real level input.**

## 3. Tests, RED→GREEN, both altitudes (`§1a`)

3 new tests in `generic_pool_group_selection_wiring_tests`:

- `cavalier_generic_order_pass_grounds_a_never_hand_modelled_order` (Order of the Green) — proves
  both the THIRD naming/ownership shape AND `owning_class_override` are load-bearing together
  (removing either one alone breaks this test — confirmed by both mutations below).
- `cavalier_generic_order_pass_does_not_collide_with_the_hand_modelled_order_of_the_sword` — the
  generic pass's `order:sword` selection grounds nothing generically (Order of the Sword has no
  Cavalier-owned CHOOSER header), while the existing hand-modelled branch still fires.
- `invented_cavalier_order_selection_grounds_nothing` — an invented selection ids nothing.

**Mutation altitude 1 (library logic):** `owning_class_override` forced to always fall through to
`record.class` (`let owning_class = if true { &record.class } else { ... };`) → re-ran:

```
1 failed: cavalier_generic_order_pass_grounds_a_never_hand_modelled_order
10 passed: every other test in the module unaffected
```

RED confirmed, isolated to exactly the intended test. Reverted.

**Mutation altitude 2 (chassis call site):** the new Cavalier `push_generic_pool_group_selection_
magnitude(...)` call wrapped in `if false { ... }` → re-ran:

```
1 failed: cavalier_generic_order_pass_grounds_a_never_hand_modelled_order
10 passed: every other test unaffected
```

RED confirmed, isolated to exactly the mutated call site. Reverted; call site restored verbatim.

**Regression check**, scoped:

```bash
cargo test --locked --lib -- rules_core::pilot_compute::generic_pool_group_selection_wiring_tests
```
```
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 2755 filtered out
```
```bash
cargo test --locked --lib -- rules_core::pilot_compute::
```
```
test result: ok. 926 passed; 0 failed; 0 ignored; 0 measured; 1840 filtered out
```
```bash
cargo test --locked --lib -- rules_core::derived_evaluator_fixture_check::
```
```
test result: ok. 121 passed; 0 failed; 0 ignored; 0 measured; 2642 filtered out
```
```bash
cargo test --locked --lib -- cavalier
```
```
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 2750 filtered out
```

926/926 (up from cycle 6's 923/923: +3 net new tests, 0 broken). Every pre-existing hand-modelled
Cavalier test (Challenge, Expert Trainer, bonus/teamwork feat counts, Order of the Sword's own
branch) still passes unchanged, confirming the new `ability_modifiers` threading through
`compute_apg_class_chassis` -> `ground_cavalier_mount_and_defer_the_rest` ->
`ground_cavalier_named_features` is a pure signature widening with no behaviour change for any
existing caller.

## 4. Sweep (`§3`) and residual re-derivation (`§17a`)

```bash
grep -rn "5,927\|5927\b" docs/release/SD-32-compute-library-and-cause-closure/*.md tests/ src/ scripts/ apps/
```
Only the kanban.md narrative cell (this cycle's own note, and every prior cycle's) cites the
figure; no test/src/scripts pinned-count assertion needed a matching update.

```bash
python3 scripts/census_class_feature_pool_population.py
```
```
RESIDUAL numeric-magnitude needing compute       5927
```
**Unchanged, and correct.** This cycle's Order-of-the-Green closure is RUNTIME reachability for a
specific recorded selection through the already-existing generic resolver — the same `§16`
distinction cycles 4-6 already drew — not a change to the static catalog count.

## 5. Scope discipline

**Did not attempt**, real scoped follow-on, named rather than silently deferred:

- **Hunter Animal Focus** (21 real records, exact match) — activation-gated, needs careful
  activation-state integration; cycles 3/4/5/6 all flagged this and it remains untouched, unchanged
  reasoning: this cycle's time went to the PRE-gate investigation (§1, a real zero worth reporting
  precisely) and Cavalier Order (§2, a real one-group closure).
- **The corpus-ingestion gap named in §1** (`.MOD`-line merge loss on `bloodline_tracker.json` and
  its Bloodrager sibling; no `raw_tokens` capture at all for `data/corpus/*/class/*.json`) —
  escalated by name, out of this cycle's guarded-path-only write scope.
- **`PREABILITY`/`PREMULT`-gated remainder of Sorcerer Bloodline (38/53), Bloodrager Bloodline
  (8/12), Cleric Domain/Shaman Spirit** — investigated this cycle (§1) and found to be blocked one
  hop deeper by the corpus-ingestion gap, not by prerequisite evaluation itself; 0 groups
  unblockable without that separate, larger, out-of-scope fix.
- **Oracle Mystery** — untouched, stays withdrawn. `§1a`'s safety test
  (`oracle_dispatch_widening_safety_tests::a_mystery_pick_alone_grounds_no_tier_one_revelation`)
  not touched; the budgeted-revelation modelling gap cycle 5 found is not closed by this cycle's
  work.
- Rows 11/15 (left `in-progress`, untouched); `apps/desktop`'s row 19/20 lanes not touched (no
  changes outside `pilot_compute/mod.rs` and this row's own kanban cell; `pilot_compute` was
  rebase-checked against `origin/tranche/12` at session start, no upstream landing to rebase onto
  at push time — see final report). `data/corpus/**` untouched throughout.

`df -h /`: reported in the dispatch's final report.
