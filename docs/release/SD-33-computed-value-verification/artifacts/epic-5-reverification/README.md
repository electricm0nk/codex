# Epic 5 — Re-verification of the blessed

`AT-33-E5-001`'s population: the **1,741** `fixture-verified` units in `docs/work-inventory.json`
(`THE-BOX.md`'s `fixture-verified` group, disposition "fixture-verified, oracle-pending" —
verified against our own artifacts, never against PCGen, per `decisions.md` §7).

## Path A / Path B input (`AT-33-E2-004`)

Epic 2's closing receipt ruled **Path A**: the pinned PCGen builds headless and a real
character round-trips through `BatchExporter`. This cycle extends that proven mechanism —
the same `./gradlew run --args="-s run-settings -E <template.ftl> -c <input.pcg> -o <output>"`
invocation, the same `scripts/oracle_harness/` comparison code — from Epic 2's one
hand-authored fighter to a real, population-scoped batch.

`AT-33-E2-004`'s own receipt named the sizing question explicitly: "sizing and executing
that authoring effort across 8,330 units is `AT-33-E5-001`'s and `AT-33-E5-002`'s own
scope, not pre-decided here." This cycle answers a first slice of that sizing question by
execution, not by estimate.

## What this cycle actually re-verified against the live oracle

**11 of 1,741** — the entire `equipment` kind, the smallest and most tractable slice of the
population (fixed `BONUS:STAT|<ability>|<n>|TYPE=Enhancement` magic items, no character level
or class dependency, one PCGen equip-slot per item).

For each of the 11 items:

1. **`ours`** — the real, live output of `codex::rules_core::equipment_effects::compute_equipment_effects`
   (the same function `AT-33-E1-003`'s `probe_equipment_effect_wiring` calls to establish these
   units are engine-wired at all), run against the real `data/corpus/ultimate_equipment/equipment/`
   records via a small scratch Rust program (`ours-derivation/equipment-ours-probe.rs`, kept
   outside the codex repo tree — `AT-33-E5-001`'s granted write scope is this `artifacts/` directory
   plus Epic 2's harness plus an append-only `THE-BOX.md`, not `src/`; the program only *reads*
   `codex` as a Cargo path dependency, it writes nothing into the repo). Output:
   `ours-derivation/equipment-ours-probe.output.json`.
2. **`oracle`** — a real, live PCGen export. One `.pcg` per item (`fixtures/equipment-pcg/`,
   a Level-1 Human Fighter, `CAMPAIGN:Core Rulebook` + `CAMPAIGN:Ultimate Equipment`, the one
   item `EQUIPNAME`d and `EQUIPSET`-equipped into its real PCGen slot — `Belt` or `Headband`,
   confirmed against `code/testsuite/PCGfiles/pf_Rogue.pcg`'s real `EQUIPSET` usage before
   authoring), run through `./gradlew run` with a small new template
   (`fixtures/e5-equip-stats.txt.ftl`) that emits all six ability scores generically
   (`STAT.0..5.NAME`/`.SCORE`/`.MOD`, modeled on `AT-33-E2-002`'s `<#list pc.checks as check>`
   generic-loop pattern so no per-item token authoring was needed). Raw per-item output:
   `fixtures/equipment-oracle-txt/<slug>.txt`. All 11 gradle runs exited 0
   (`build-transcript-equipment-ALL-11-tails.txt`; one full transcript committed as a sample,
   `build-transcript-equipment-headband_cha_2-SUCCESS.log`).
3. **Comparison** — `equipment.ours.json` maps each real inventory `unit_id` to
   `(oracle_key, ours_expected_score)` where `ours_expected_score` is the item's declared base
   ability score (16/14/14/10/10/8 for STR/DEX/CON/INT/WIS/CHA on this `.pcg`) plus the real
   engine's resolved bonus — i.e. this checks the *applied* total, not merely that the raw bonus
   number was parsed correctly (which `literal-verified` already establishes for a different
   population). `equipment.oracle-export.txt` merges all 11 raw exports (slug-prefixed keys, so
   one file holds all 11 without key collisions — `scripts/oracle_harness/oracle_export.py`'s
   parser is a flat `KEY=VALUE` reader with no other coupling to this run's shape).
   `scripts/oracle_harness/run.py` (`AT-33-E2-003`'s CLI, unmodified) produced
   `equipment.oracle-results.json`; `scripts/box_ledger.py --check --oracle-results
   equipment.oracle-results.json` (`AT-33-E1-002`'s condition-3 gate) read it and exited 0.

**Result: 11 of 11 agree, 0 disagree, 0 unverifiable.**

## The remaining 1,730 — not folded into a false 100%, split by real structural reason

| Sub-population | Count | Probe status (`AT-33-E1-003`, re-cited not re-derived) | This cycle's disposition |
|---|---:|---|---|
| `equipment` | 11 | magnitude probe exists (`probe_equipment_effect_wiring`) | **examined this cycle — see above** |
| `spell` | 1,288 | magnitude probe exists (`probe_spell_effect_wiring`) | not yet examined — real PCGen re-verification needs a spellcasting character per class/level plus `SPELLMEM.*` template tokens (`base.xml.ftl`'s `<@loop ... COUNT[SPELLSINBOOK...]>` pattern); queued, next-cycle plan below |
| `class_feature` | 15 | magnitude probe exists (`probe_class_feature_effect_wiring`) | not yet examined — needs one high-level character per source class (Rogue/Paladin/Ranger/Barbarian/Bloodrager/Slayer/Ninja/Samurai) plus feature-specific export tokens (DR/sneak-attack-dice/channel-energy tokens, not plain `STAT.*`); queued, next-cycle plan below |
| `companion` | 187 | **no probe** — `probe_exists: false`, `presence_only` | structurally not comparable to a PCGen numeric export this cycle's harness shape checks; `holds_key()` presence lookup, not a magnitude |
| `monster` | 140 | **no probe** — `probe_exists: false`, `presence_only` | same structural reason |
| `monster_ability` | 100 | **no probe** — `probe_exists: false`, `presence_only` | same structural reason |
| **Total** | **1,741** | | 11 examined, 1,730 not yet examined |

**427 of 1,741 (`companion`+`monster`+`monster_ability`) carry a probe-surface gap that
predates this cycle** (`AT-33-E1-003`, executed 2026-08-24/25) — our own engine holds no
magnitude-producing computation for these kinds to compare against an oracle value at all
(`monster_resolve()`/`holds_key()` are table lookups, not formula evaluation). This is not
this cycle's effort gap; it is named here because `AT-33-E5-001`'s population statement
("the 1,741") would otherwise silently imply all 1,741 are equally re-verifiable, which
`AT-33-E1-003`'s own finding already contradicts. Re-verifying these 427 against PCGen would
first require widening the engine's own probe surface — out of this criterion's scope, and
not silently absorbed into "examined" or "unverifiable" here.

**1,303 of 1,741 (`spell`+`class_feature`) are genuinely re-verifiable and not yet attempted**
this cycle — real per-unit (or per-batch) `.pcg`/template authoring cost, exactly the sizing
question `AT-33-E2-004` named. This is a `decisions.md §5`-shaped authoring-cost issue, not an
operator ruling — the next cycle's plan is concrete and mechanical (see below), so this is
**not** filed as an `## Open blockers` entry (`workflow-instruction.md` §8): decomposing and
running the next cycles, not escalating.

## Next-cycle plan (concrete, not a restated goal)

1. **`class_feature` (15 of 1,741):** one L20 `.pcg` per source class already present in the
   pinned oracle's `data/pathfinder` (Rogue/Paladin/Ranger/Bloodrager/Slayer/Ninja/Samurai cover
   14 of 15 features for free via class progression — no per-feature authoring, matching this
   cycle's discovery that PCGen grants inherent class features automatically); Barbarian +
   `rage_power_superstition` needs one explicit rage-power selection. Export tokens differ per
   feature (DR, sneak-attack dice, channel-energy dice/uses, trap-sense bonus) — reading each
   token name out of `outputsheets/base.xml.ftl` first, the same discipline `AT-33-E2-002` used,
   is the bulk of the remaining work, not character authoring.
2. **`spell` (1,288 of 1,741):** batch via `SPELLMEM.${class}.${spellbook}.${level}.${spell}.*`
   generic iteration (confirmed present in `base.xml.ftl`, not requiring one template line per
   spell) against one or a few high-level prepared-caster `.pcg`s with spells added to their
   spellbook — sizing the number of characters needed by class-spell-list coverage is this
   sub-population's own first step.
3. **`companion`/`monster`/`monster_ability` (427 of 1,741):** not an Epic 5 authoring task —
   requires Epic 1's probe surface to widen first (a `holds_key()` presence check has no
   magnitude to compare). Named here so a future cycle does not silently fold these into
   "examined" or re-litigate whether they are in scope.
