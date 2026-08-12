---
title: Tranche-3 — Core Rules Breadth Scope
status: draft (operator review required)
date: 2026-07-12
operator: Todd Hintzmann
owner: god-emporer (architect), tech-priest (pre-loop gate slice), operator (loop execution)
parent: programs/codex/assumptions/tranche-3-starting-assumptions-2026-07-10.md
corpus_root: pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc
doctrine_refs:
  - programs/codex/doctrine/support-state-vocabulary.md
  - programs/codex/doctrine/program-doctrine-and-scope-charter.md
  - programs/codex/doctrine/documentation-control-plane.md
  - programs/codex/doctrine/quality-gate-policy.md
  - ~/workspace/sd13-class-uplift-loop-prompt.md (matured SD-13 model; see §6 for inheritance)
related_bundles:
  - docs/release/SD-13/ (relocated 2026-07-20 from programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/)
  - docs/release/SD-17/ (relocated 2026-07-20 from programs/codex/requirements/SD-17-pcgen-corpus-include-graph-resolution/)
  - programs/codex/requirements/tranche-2-7-pcgen-corpus-ingestion/ (does not exist in this repo clone)
technical_design_doc: /home/ubuntu/workspace/programs/codex/requirements/SD-18-core-rules-breadth/technical-design.md
corpus_root: /home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc
code_root: /home/ubuntu/workspace/repos/codex
execution_mode: operator-driven claude-code loop on breadth (churn); pre-loop tech-priest card on the consumer-side composition gate.
objective: support + product-visible (per operator directive, 2026-07-12).
---

# Tranche-3 — Core Rules Breadth Scope

## 0. Premise

Tranche-3 ships the full Pathfinder 1e Core Rulebook (corpus rooted at `pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc`) at **levels 1 through 20** for all 7 core races, all 11 core classes, and the 2 named interaction rows in the SD-13 matrix, plus spell corpus coverage across all 9 PF1 strict schools (plus Universal) and equipment corpus coverage across all 4 core-rulebook equipment categories.

End-user feature proof per tranche-3 acceptance criterion (per Todd's directive 2026-07-12): the row or corpus element reaches `supported/Product-visible` — the operator's UI surfaces it; the rules engine produces the correct derived output for that row or element when driven against the imported Core Rulebook corpus. The objective is the *combination* of `SupportState::Supported` and evidence tier `Product-visible`. State alone, or visible-without-grounded, is counterfeit.

## 1. Pre-loop prerequisite (card-routed, NOT loop-routed)

This slice lands via the standard card-routing dispatch path (operator mint + tech-priest execution + kanban completion). It is the gate for tranche-3's loop execution. **No tranche-3 loop iteration begins until §1.1 ships.**

### 1.1 Consumer-side composition

**Scope:** Bridge the rules engine's chosen-state input (`CharacterInput` at `/home/ubuntu/workspace/repos/codex/src/rules_core/character_input.rs:18`) with the corpus-side records (`SourcePackageContent` in `src/rules_core/source_content.rs`, delivered by SD17-E). The bridge takes a chosen `CharacterInput` plus a `SourcePackageContent` and produces the input that `pilot_compute.rs` evaluates.

**Concrete surface the slice touches** (full inventory in `technical-design.md`):

- `src/rules_core/character_input.rs:18` — `CharacterInput` chosen-state shape (the bridge's input side).
- `src/rules_core/pilot_compute.rs:2168` — `build_pilot_headless_receipt(input: &CharacterInput) -> PilotHeadlessReceipt` (the bridge's output side; reads the composed input).
- `src/rules_core/pilot_compute.rs:2186` — `compute_pilot_base_chassis` (the call-site for chassis-level derivations).
- `src/rules_core/character_input.rs` — `load_character_input_fixture` at line ~125 (the existing chosen-state loader; may be extended or used as a parallel mechanism).

**Acceptance criterion:** A single round-trip test of the form "given `core_rulebook.pcc`, build a Human Fighter level 5 with chosen ability scores and a known set of feats/equipment selections, hand the composed input to `pilot_compute.rs`, observe that derived stats reflect the corpus-side class and race features (BAB, saves, racial ability bonus, favored class bonus) rather than just the seeded defaults." This card ships as a tech-priest CODE slice under `codex-tranche-3`.

**Owner:** tech-priest.

**Tranche status:** Once §1.1 ships, tranche-3's loop can run.

## 2. Loop-routed coverage (breadth, churn)

The operator-driven claude-code loop runs this section. The loop's instruction document (operator-authored) drives each iteration. Each iteration:

1. Reads this scope doc and the progress doc at the top of every cycle.
2. Creates a fresh feature branch off `tranche/3` (the tranche branch; see §6 for inheritance from the matured SD-13 model).
3. Lands the bounded slice on that branch.
4. Auto-merges the branch to `tranche/3`. Operator review is NOT required for tranche-branch PRs — this is the matured SD-13 doctrine (the as-written SD-13 prompt pre-dates the correction).
5. On merge failure, **handles the conflict inline if it can self-heal**, OR **writes the unresolved conflict to `## Open blockers` in the progress doc and exits with `FAIL`**. The loop does NOT advance to the next iteration until the current merge lands (or surfaces as a blocker).
6. After successful merge, deletes the feature branch from both local and origin. Keeps origin and local in sync at every step.
7. Mints a kanban card into `codex-tranche-3` as a completion record (status=done), with merge receipt, branch SHA, cargo test summary, clippy clean signal, cycle timing, and any cycle-specific context useful for post-mortem.
8. Updates the progress doc with the new completion. Exits.

The kanban board is a **post-mortem audit surface**, not a dispatch queue. Cards are minted after the fact, never pre-minted as work units.

## 3. Acceptance criteria (loop-routed)

Each section is one or more acceptance criteria. The loop picks a single criterion per iteration.

### 3.1 SD-13 matrix race rows (7 rows, levels 1–20)

For each of the 7 core races (Dwarf, Elf, Gnome, Half-Elf, Half-Orc, Halfling, Human), lift the SD-13 row from `unverified/Observed` or `partial/Computed` to `supported/Product-visible` at **levels 1 through 20**. Per row, the loop proves:

- The race's LST data round-trips through `SourcePackageContent` and into the consumer-side composition.
- The race's racial features apply correctly across levels (size, speed, ability-score modifiers, racial traits, favored class bonus when leveled).
- The named-class integration for one chosen core class against the race.
- The race's matrix row advances visibly in `core-roster-and-support-state-matrix.md`.

**Concrete corpus and code surface** (full detail in `technical-design.md`):

- Per-race LST source: `core_essentials/races/<race>/` directory, 6–10 LST files per race (e.g., `dwarf_abilities_race.lst`, `dwarf_races.lst`, `dwarf_templates.lst`).
- `cr_races.lst` carries the 7 core-race `.MOD` entries at lines 5–12, dispatching via `parse_lst_entry` in `src/pcgen_import/lst_parser/race_ability.rs:590`.
- Race seams already exist in `pilot_compute.rs`: `explain_dwarf_race_seam` at line 2529, `explain_elf_race_seam` at 2654, `explain_gnome_race_seam` at 2771, `explain_half_elf_race_seam` at 2884, `explain_half_orc_race_seam` at 3005, `explain_halfling_race_seam` at 3126, `explain_human_pilot_race_seam` at 2403. The race cycles *extend* these seams (one named family per cycle — Stonecunning, Hardy, Stability, weapon familiarity, Keen Senses, etc., drawn from the SD-13 row's `blocker_or_lossiness_note`).
- New cycle artifacts: `tests/sd18_<race>_<family>.rs` + `tests/fixtures/rules_core/pf1_<race>_fighter_level<N>_sd18_<family>.txt`.
- Matrix update: `src/rules_core/support_state_matrix.rs::seeded_sd13_e1_f1_current_truth()` (line 727) row for `race.<race>.bounded_semantics`.

**Acceptance criterion (per row):** race row at `supported/Product-visible` tier, with at least one end-user-visible character build demonstrating the race at L1, L10, and L20 against its exemplar class.

### 3.2 SD-13 matrix class rows (11 rows, levels 1–20)

For each of the 11 core classes (Barbarian, Bard, Cleric, Druid, Fighter, Monk, Paladin, Ranger, Rogue, Sorcerer, Wizard), lift to `supported/Product-visible` at **levels 1 through 20**. Per row, the loop proves:

- The class's LST data round-trips through the consumer-side composition.
- The class's BAB, saves, HP, skill points, class features, and (for casters) spell progression apply correctly across levels.
- The class's named race integration for one chosen core race.
- The class's matrix row advances visibly.

**Concrete corpus and code surface**:

- Class corpus: `cr_classes.lst` carries 87 CLASS: entries representing 27 distinct IDs. Use `parse_class_file` (`src/pcgen_import/lst_parser/class.rs:425`) for martial classes; `parse_spellcasting_class_file` (`src/pcgen_import/lst_parser/spellcasting_class.rs:815`) for spellcasting classes.
- Class seams already exist in `pilot_compute.rs`: `explain_barbarian_level1_chassis` (line 6509), `explain_monk_level1_chassis` (7079), `explain_rogue_level1_chassis` (7988), `explain_paladin_level1_chassis_and_spell_burden_separation` (4222), `explain_ranger_level1_chassis_and_class_feature_separation` (4987), `explain_sorcerer_level1_spell_baseline` (8487), `explain_wizard_level1_prepared_spell_baseline` (9247), `explain_cleric_level1_spell_baseline` (9656), `explain_druid_level1_spell_baseline` (10135), `explain_bard_level1_spell_baseline` (called from line 2293), `supported_fighter_level` (3361). The class cycles *extend* these to higher level bands.
- New cycle artifacts: `tests/sd18_<class>_<level>_<burden>.rs` + `tests/fixtures/rules_core/pf1_human_<class>_level<N>_sd18_<burden>.txt`.

**Acceptance criterion (per row):** class row at `supported/Product-visible`, with at least one end-user-visible character build demonstrating the class at L1, L10, and L20 against its exemplar race, including at least one leveled class feature or spell that has measurable effect on derived stats.

### 3.3 SD-13 matrix interaction rows (2 rows)

For each of the 2 interaction rows (Human bonus feat / ability-bonus seam; non-Human race × class progression beyond pilot), lift to the highest tier achievable from corpus coverage alone.

**Acceptance criterion (per row):** the interaction row's matrix state advances one or more tiers per the SD-13 quality gate vocabulary, with grounding artifacts cited.

### 3.4 Spell corpus coverage (9 school cards)

For each PF1 strict school (Abjuration, Conjuration, Divination, Enchantment, Evocation, Illusion, Necromancy, Transmutation, plus Universal — 9 cards total), prove the school's spells appear in the consumer-side composition correctly.

**Concrete corpus and code surface**:

- Spell corpus: `core_rulebook/cr_spells.lst`. ~652 spell records, identified via `SCHOOL:` tags (652 SCHOOL: line matches across the file).
- Per-school counts vary with the corpus; the loop derives per-school totals at runtime. Strict-school partition maps from the corpus's sub-school keywords (Transmutation → Polymorph/Calling; Illusion → Figment/Glamer/Shadow/Phantasm/Pattern; Conjuration → Summoning/Creation/Calling/Teleportation/Healing; etc.).
- Spell parser: `src/pcgen_import/lst_parser/spell.rs:488` (488 lines). Public surface likely includes `parse_spells_file` and `parse_spells_text`; final API settled by SD17-B-4.
- Spell cards and class cards cover distinct fields: a school's coverage proves the *corpus side* (every spell parses, reaches `SourcePackageContent`); a class's coverage proves the *behavior side* (the class casts and computes effects).

**Acceptance criterion (per school card):** every spell in the school's slice parses via SD17-B-4, lands in `SourcePackageContent`, is reachable from a chosen `CharacterInput` (via class spell list), and is consumable by the rules engine. End-user-visible character at L10 of a casting class demonstrating the school.

### 3.5 Equipment corpus coverage (4 category cards)

For each of the four `core_rulebook/cr_equip_*.lst` files (`arms_armor`, `general`, `magic_items`, `equipmods`), prove the equipment in that category is consumable by the rules engine through the consumer-side composition.

**Concrete corpus and code surface**:

| Category | File | Content shape | Parser |
|---|---|---|---|
| arms_armor | `cr_equip_arms_armor.lst` | weapons and armor (PCGen object-kind tags) | `parse_equipment_file` in `equipment.rs:781` |
| general | `cr_equip_general.lst` | adventuring gear (poisons, mounts, vehicles) | same |
| magic_items | `cr_equip_magic_items.lst` | scrolls (~634), wands (~351), potions (~87), rings (~59), belts (~22), etc. | same |
| equipmods | `cr_equipmods.lst` | equipment modifiers (BONUS: chains) | same |

Total equipment corpus: thousands of entries (cr_equip_magic_items.lst alone has the 634+351+87+59+22+... distribution).

**Acceptance criterion (per category card):** a representative sample of the category's items parse via SD17-B-5, land in `SourcePackageContent`, are selectable from a chosen `CharacterInput`, and affect derived stats when equipped. End-user-visible character demonstrating the category.

## 4. Loop execution posture

### 4.1 Per-iteration branch lifecycle

Each iteration creates one feature branch off `tranche/3` (e.g. `loop/tranche3-cycle-2026-07-12T1430-dwarf-favored-class`). Naming: `loop/<cycle-id>-<row-or-class-or-school-or-category>` so a `git fetch origin` + `git ls-remote` immediately shows what the cycle touched.

**Concrete command sequence per iteration** (full detail in `technical-design.md`):

```bash
# Step 1-2: read state (scope doc + live git state + in-flight detection)
cat /home/ubuntu/workspace/SD-18-core-rules-breadth-scope-draft.md
cd /home/ubuntu/workspace/repos/codex
git fetch origin tranche/3
ps -eo pid,etime,stat,cmd | grep -iE 'claude' | grep -v grep   # in-flight check

# Step 3: branch
git checkout -b loop/tranche3-cycle-<cycle-id>-<criterion> origin/tranche/3

# Step 4-7: TDD
cargo test --locked --test sd18_<criterion> 2>&1 | tail -40   # RED
# <implement>
cargo test --locked 2>&1 | tail -20                          # GREEN
cargo clippy --locked --tests -- -D warnings 2>&1 | tail -20 # CLEAN

# Step 8: commit (operator identity)
git add <specific files per the lane partition>
git -c user.name='Todd Hintzmann' -c user.email='todd@hintzmann.net' \
  commit -m "feat(sd18): <criterion> (<row transition>)"

# Step 9-10: push and auto-merge to tranche/3
git push -u origin loop/tranche3-cycle-<cycle-id>-<criterion>
git checkout tranche/3
git pull origin tranche/3
git merge --no-ff loop/tranche3-cycle-<cycle-id>-<criterion> -m "merge: sd18 <criterion>"
git push origin tranche/3

# Step 11: cleanup (delete ephemeral branch from local + origin)
git branch -d loop/tranche3-cycle-<cycle-id>-<criterion>
git push origin --delete loop/tranche3-cycle-<cycle-id>-<criterion>
```

### 4.2 Self-healing posture (per Todd's 2026-07-12 directive)

The loop self-heals wherever resolvable. The operator returns from a multi-day run to *a list of problems*, not a stopped loop. Self-healable conditions:

- **Branch diverged from `tranche/3` mid-iteration:** `git fetch origin tranche/3 && git rebase origin/tranche/3` in the worktree, re-run tests, re-push, re-merge.
- **Merge conflict in auto-merge:** if the conflict is mechanical and resolvable inline (e.g. import ordering, an unrelated addition), the loop resolves and re-commits the merge.
- **Cargo build cache corruption:** `cargo clean` and rebuild.
- **`target/` disk pressure:** strip worktree `target/` directories (SD-13 §5 step 1).
- **Stale worktree from a prior cycle:** remove with `git worktree remove --force`, retry.
- **Coordination file drift between cycle log and live matrix:** read the live matrix, refresh the coordination file's snapshot, retry.

Non-self-healable conditions (write to `## Open blockers` and exit `FAIL`):

- Conflict requires a domain decision (which side wins on a class-feature semantics question).
- A slice branch needs a manual rebase because `tranche/3` advanced in a way the auto-rebase can't handle.
- Two live claude processes would both touch `pilot_compute.rs` (per SD-13 §in-flight detection).
- A chosen burden needs a new subsystem (feat-prerequisite engine, spellbook engine, damage-total engine).
- Disk is at 100% with no `target/`-strip remedy.

### 4.3 Kanban card-as-post-mortem posture (per Todd's 2026-07-12 directive)

Each loop iteration that lands work mints a kanban card into `codex-tranche-3` as a completion record. Card body schema (operator-authorable additions welcome, but the floor is):

- `epic: SD-13` (or `SD-17-corpus-coverage` for §3.4/§3.5 cards)
- `row_or_kind`: the matrix row id (e.g. `race:dwarf`), or the school name, or the equipment category.
- `evidence_tier_before` / `evidence_tier_after`: the SD-13 row state before and after the merge.
- `branch`: the per-iteration feature branch that was merged.
- `merge_receipt_sha`: the merge commit SHA on `tranche/3`.
- `cargo_test_summary`: e.g. `cargo test --locked --test sd13_dwarf_favored_class green (N tests)`.
- `clippy_signal`: `clean` / `dirty`.
- `cycle_timing_seconds`: how long the iteration ran.
- `self_heals_applied`: list of self-heals performed this iteration, if any.
- `next_required_uplift`: the loop's recommendation for the next iteration's target, if applicable.

This card schema makes `codex-tranche-3` audit-ready: an operator returning after 3 days can `git log --oneline tranche/3 -N`, find the corresponding cycle-id in the progress doc, and read the kanban card for full context.

### 4.4 Progress doc structure (parallel to scope doc)

The progress doc mirrors scope doc structure 1:1. Each section title matches a scope doc section; under each, the loop writes one or more `done` / `in-flight` / `open-blocker` rows with the row id, branch, merge SHA, and card id.

```
## 3.1 Race rows (7)
- Dwarf @ supported/Product-visible | merge 6d6d7a7 | card t_xxxxx
- Elf @ supported/Product-visible | merge 14e9124 | card t_xxxxx
- ...

## 3.4 Spell schools (9)
- Abjuration @ supported/Product-visible | merge ... | card t_xxxxx
- ...

## ## Open blockers
- <rows that the loop could not self-heal, with reason and merge-conflict detail>
```

### 4.5 Loop termination

The loop terminates when the progress doc shows every acceptance criterion in §3 satisfies its criterion AND there are no unresolved `## Open blockers` from prior cycles. Operator reviews the final progress doc and the populated `codex-tranche-3` board as the closure posture for tranche-3.

## 5. Non-goals

- **No additional tomes** beyond the Core Rulebook. Bestiaries, Ultimate-*, Advanced Class Guide, Occult Adventures, Unchained, and homebrew are deferred to later tranches per the operator's 2026-07-12 directive.
- **No `core_essentials` duplication** — Core Essentials is a subset of Core Rulebook; CE is reached via PCC include, not as a separate ingestion target.
- **No PFS** — Pathfinder Society rules are a separate corpus under `pathfinder_pfs/` and are deferred.
- **No autonomous kanban dispatch** for §3 work. Loop only.
- **No new IR-shape work** — SD17-E types are the substrate; tranche-3 consumes them.
- **No UI authoring** — the operator builds UI surfaces directly per the operator's 2026-07-10 assumption.

## 6. Inheritance from the SD-13 model

Tranche-3 inherits the **matured** SD-13 operator-driven loop model rather than the as-written `sd13-class-uplift-loop-prompt.md` file. The corrections made between that file and the actual end-of-tranche-2-6 operating posture are:

- **Auto-merge to tranche branch is permitted.** The as-written prompt's "operator reviews every PR" was tightened to "operator reviews PRs to develop; tranche-branch PRs auto-merge." Tranche-3 inherits the tightened posture.
- **Self-healing requirements added.** The as-written prompt's hard-stop list has been tightened to distinguish self-healable conditions from non-self-healable conditions; tranche-3 inherits the self-healing posture.
- **Kanban card-as-post-mortem.** The as-written prompt's coordination file is the working memory; tranche-3 elevates the kanban cards to the post-mortem record (per Todd's 2026-07-12 directive), with the card schema in §4.3.
- **Branch cleanup timing.** The as-written prompt leaves branch cleanup as operator's option after merge; tranche-3 makes it part of the cycle (per Todd's 2026-07-12 directive): after successful merge, branch deleted from local and origin at once.

Tranche-3 does NOT need to duplicate `sd13-class-uplift-loop-prompt.md` in its loop instruction document. The matured SD-13 model is the reference pattern; the tranche-3 loop instruction inherits the structure and tightens the specific points above.

## 7. Tranche-3 closure definition

Tranche-3 closes when:

1. §1.1's pre-loop slice has shipped.
2. Every row in §3.1, §3.2, §3.3 has its SD-13 matrix state at `supported/Product-visible` (or at the highest tier achievable from corpus coverage, with `partial/Computed` accepted only with explicit grounding).
3. Every school in §3.4 has its school card landed with the §3.4 acceptance criterion met.
4. Every category in §3.5 has its category card landed with the §3.5 acceptance criterion met.
5. The final progress doc reflects every acceptance criterion as satisfied.
6. The `codex-tranche-3` board shows the post-loop populated ledger, every card `status=done`, with merge receipts for post-mortem audit.
7. `tranche/3 → develop` promotion PR is opened (operator-driven, per existing promotion cadence from tranche-2-5 / tranche-2-7).

Operator's three-day-return posture: §6 closure posture is reviewable in the progress doc + `codex-tranche-3` board. Anything that the loop could not self-heal is in `## Open blockers` with reason. The operator's first action on return is to read `## Open blockers` if non-empty.
