---
title: SD-20 — Risks and Open Questions
status: approved (operator review 2026-07-16; changes noted: §2 broadened to any class/any level, Q2 revised to class-selection trigger mechanic, Q3 revised to print-ready data; SD-20 launches on tranche/4 branch)
date: 2026-07-15
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-20-rules-engine-completeness/decisions.md
---

# SD-20 — Risks and Open Questions

This file enumerates the risks, blockers, and open questions specific to SD-20. Structured to mirror SD-18 and SD-19's risks docs so a future operator reading all three can navigate them with the same mental model.

## Self-healable conditions (resolve inline, exit GREEN)

| Condition | Detection | Self-heal |
|---|---|---|
| Working tree dirty at cycle start (an in-flight cycle left uncommitted work) | `git status --porcelain \| wc -l` returns non-zero | Run `git stash` (if previous unfinished attempt) or `git checkout -- .` (stray edit noise); re-verify clean; retry |
| Feature branch diverged from `tranche/4` mid-cycle *(retained for documentation; SD-20 has no ephemeral branches per the no-branches convention; per operator directive 2026-07-16: SD-20 launches on `tranche/4`, not `tranche/3`)* | (n/a) | (n/a — no branches) |
| Progress doc snapshot drift between SD-20 cycle work and the SD-19/SD-18 shared progress doc | Progress doc > 5 commits behind the live-cycle verdict | Append a `## SD-20 cycles — snapshot at <sha>` block; reset snapshotting to that state |
| A fixture `expected_output` for wire-fixture parity diverges from the engine's actual output during RED testing | Cycle's RED test fails on assertion `pilot_receipt == fixture.expected_output` | Recompute the engine's output against the boundary contract; if the engine is right, update the fixture (with an audit comment on the cycle's card); if the contract is wrong, fix the contract first |
| A spellbook school cycle discovers a corpus record whose `TableCellRef` lookup returns `None` (the table store doesn't have an entry for that KEY) | Cycle's `TableCellRef` assertion fails | Extend SD-19's `rules_tables/crb/spell_list.rs` to add the missing KEY-to-row mapping; mark the cycle's PR with audit comment per SD-19 closure pattern |
| A feat cycle discovers a prerequisite path that the engine models partially but doesn't fully ground | Cycle's `PrerequisiteEvaluation` differs from fixture's expected output by exactly one prereq path | Read the upstream epic's output to confirm whether the path is grounded elsewhere; if not, escalate to the operator (this is the boundary contract's territory) |

## Non-self-healable conditions (write to `## Open blockers`, exit FAIL)

| Condition | Detection | Why not self-heal |
|---|---|---|
| The `Cargo` build doesn't compile because a partial epic's seam signature doesn't match its parent module's expectations | `cargo build 2>&1 | tail` shows error | This is a slice-bug, not a cycle-bug; the slice needs to be amended, not the cycle |
| Two epics produce contradictory `PilotReceipt` shapes (e.g. one writes `equipment.attack_bonus`, another reads `equipment.attack_bonus_delta`) | Compile error or wire-fixture parity test fails across all fixtures | Boundary-contract drift — the epic's seam signature has diverged from the canonical contract |
| The SD-19 foundation slice's table store has a missing entry the SD-20 epic needs | RED test fails because `TableCellRef` returns `None` on a guaranteed-present corpus record | The foundation slice is incomplete; the foundation slice itself is out of SD-20's scope (SD-19 owns the table store) |
| A spell *effect* is needed for tabletop-readiness but the engine's spellbook epic produces only spell *coverage* without *effects* | Wire-fixture parity test for a tablet-relevance spell fails because the receipt has the spell name and DC but no effect text or dice expression | The engine's spellbook output is `SpellbookCoverage` which holds spell metadata, not effects. SD-20's epic 2 may need to extend to produce effect-text-and-dice; escalate to operator |
| A feat *effect* is needed for tablet-readiness but the engine's feat prerequisite epic produces only prerequisite eligibility without *effects* | Wire-fixture parity test for a chosen-feat scenario fails | Same shape as the spell-effect gap; epic 3 needs to extend `FeatEffects` to produce the actual deltas; escalate |
| A user-picked Level Up selection goes into the next `CharacterInput` but the engine's receipt doesn't reflect it (the engine grant works fine; the integration with user selections breaks) | Wire-fixture parity test for a post-Level-Up scenario with explicit picks fails | Integration closure failure; epic 7's boundary with `CharacterInput` needs an explicit contract |
| Cargo test regresses on a row other than the one the cycle touched | Full suite regresses after a cycle's change | Sibling-preservation is a hard rule |
| Progress doc and live matrix disagree on a row's `evidence_tier` (not just stale snapshot) | `support_state_matrix.rs` says `Supported/Product-visible` but the progress doc's row status is `open` (or vice versa) | Manual operator reconciliation required |
| Two live `claude` processes would both touch `pilot_compute.rs` or any per-epic module file | `ps -eo pid,etime,stat,cmd \| grep claude` shows multiple in-flight on the same file set | Structural: one-lane-at-a-time rule (per SD-18/S SD-19's choke-point file partition) |

## Override flags (durable; patched when operator accepts a default)

When the operator doesn't answer a clarifying question within the time limit, the bundle defaults and flags the option here. Override cost estimates are in minutes of patch work.

### Flag A — Boundary contract shape

**Default chosen**: contract landed as a markdown artifact at `docs/release/SD-20/boundary-contract.md` with three sections (CharacterInput shapes, PilotReceipt fields, printed-sheet cell map). The contract is the authority for what the engine produces and what the GUI renders; subsystem engines produce into it, never around it.

**Override alternatives**:
- *Contract as Rust types in code*, generated or hand-written, with the GUI consuming them via FFI/JSON serialization rather than via a markdown document. Lower drift risk, but requires codegen tooling or hand-written type definitions to be the source of truth in two places (Rust + a separate spec).
- *Contract as JSON Schema or Protobuf*. More formal validation, more tooling overhead; would add a code-generation step to the loop.

**Override cost**: ~30–60 minutes if operator picks one of the alternatives; the markdown artifact's structure is what everything in the bundle references.

### Flag B — Wire-fixture parity JSON format

**Default chosen**: each fixture is one JSON file at `tests/fixtures/wire/sd20/<criterion>.json` with three top-level fields (`name`, `input`, `expected_output`). JSON because the engine and the GUI both consume JSON; no translation required.

**Override alternatives**:
- *YAML fixtures* (matches the rules_tables foundation slice's structured-data format). Same content, different syntax.
- *TOML fixtures*. Same content, different syntax; Rust-native, less engrained in web stack.
- *Pair format: one file per direction* (`expected_engine.json` + `expected_gui.json`). Splits the contract; more files.

**Override cost**: ~5 minutes; changing the format only affects how the loop reads fixtures, not what the fixtures contain.

### Flag C — GUI implementation scope outside the bundle

**Default chosen**: GUI implementation (character sheet rendering, CreateCharacter form, campaign manager) stays outside the bundle per the operator's directive 2026-07-14 ("let's leave it outside for now. if need be we can add a tranche-4-1"). The boundary contract is the only GUI-facing artifact in the bundle.

**Override alternatives**:
- *Bundle owns GUI contract spec*. The bundle documents component-level specs, accessibility requirements, design tokens. Pros: makes the GUI explicit. Cons: the operator loses the freedom to vibe-code the GUI.

**Override cost**: significant — moving the GUI into the bundle means re-doing significant work the operator is doing outside.

### Flag D — SD-20 epic-to-loop-lane assignment

**Default chosen**: 8 epics listed in `decisions.md` §7 with the dependency ordering cited. Epics 2 (spellbook), 3 (feat prereqs), 4 (skill ranks), and 5 (equipment effects) all depend only on SD-19's table store and can run as concurrent loops if the operator hosts them that way. Epic 6 (damage total) is sequential after epic 5. Epic 7 (Level Up grants) integrates after epics 2–6 close. Epic 8 (integration closure) is the integration milestone.

**Override alternatives**:
- *All epics sequential* (no concurrent loops; loop runs epic 1 → 2 → 3 → 4 → 5 → 6 → 7 → 8).
- *Epics grouped differently* (e.g. epic 2 and 3 grouped, epic 4 and 5 grouped, epic 6 standalone).

**Override cost**: ~10 minutes; affects the loop brief's lane assignment, not the per-epic deliverable list.

## Architectural questions (Q1 pinned; Q2–Q4 open)

These are SD-20-shaped design calls where the operator has pinned (Q1) or hasn't yet picked a default (Q2–Q4). Pinned answers are recorded in `decisions.md` and mirrored here as locked defaults; the open ones remain candidate-option write-ups for future pinning.

### Q1 — How strict is the "epic may only produce into the boundary contract" rule? (PINNED: soft)

The technical-design §3 ("per-epic authority surface") implies a hard rule: an epic may not invent a new `PilotReceipt` field; it must extend the boundary contract. The question is whether to enforce this hard (compile-time check or contract-validation step before epic merge) or soft (parity test catches it at integration). Hard is safer; soft is more pragmatic and matches SD-19's style.

**Pinned to soft enforcement** per operator directive 2026-07-16. The boundary contract remains a markdown artifact (`docs/release/SD-20/boundary-contract.md`) plus the wire-fixture parity test fixtures (`tests/fixtures/wire/sd20/*.json`). No compile-time check, no codegen, no contract-validation binary. Drift surfaces as either compile errors when one epic reads a field another writes inconsistently, or as parity-test failures when the contract doc and the actual `PilotReceipt` shape diverge. This matches SD-19's posture (no codegen, prose + tests). Recorded as `decisions.md` §10.

### Q2 — Does Level Up grant interaction with multiclass need a special shape? (PINNED: yes, class-selection trigger; Level Up is the same mechanic as level-0-to-1)

PF1 multiclassing changes when a class feature lands (e.g. a Fighter 2 / Wizard 1 gets Bravery +2 at total character level 2, not Fighter class level 2). The foundation slice's class tables may not encode this. Epic 7 needs a contract surface for "given this character, what does this total-level gain mean for each class?"

**Pinned to class-selection trigger** per operator directive 2026-07-16: "yes, level up needs to trigger a selection on which new class is being selected and from there identify what additions are specified and which are options. this is the same mechanic as a new character that is progressing from level 0 to level 1." The Level Up mechanic is the same mechanic as character creation (level 0 → level 1): the engine triggers a class-selection event when the user picks a class (or hits a level transition that requires class-feature resolution), and from there the engine identifies which additions are auto-grants and which are pick-from-lists. The seam signature stays single-delta: `compute_level_up_grants(character, from_level, to_level, rules_tables) -> LevelUpPlan`. What changes is the input shape: Level Up takes the new `CharacterInput` (with the class selection already made via the class-selection trigger) and produces the `LevelUpPlan` describing what auto-grants and what pick-from-lists apply given the level transition AND the class selection. Multiclass correctness is verified by epic 8's integration fixture set (which includes one multiclass character per the broadened acceptance criterion). The `LevelUpPlan.automatic_features` and `pick_from_lists` are populated with `TableCellRef` provenance that includes the class and class level. Recorded as `decisions.md` §11 (revised from prior pin "status quo, single delta").

### Q3 — Should the engine produce the printed sheet's render-ready HTML/PDF, or only the structured data? (PINNED: print-ready data; UI plugs values into cell locations)

The boundary contract currently names structured data (CharacterInput, PilotReceipt). The GUI renders. But for tabletop-readiness, the user wants "print sheet and use it at the table" — does the engine include a render-ready output? My current default is "engine produces structured data; GUI renders" per SD-20 §4. If the answer flips, epic 8 grows a rendering capability slice.

**Pinned to print-ready data the UI plugs into cell locations** per operator directive 2026-07-16: "no. it needs to produce the data that will be called by the printed sheet. when the ui is built to do printing, it should be able to simply call the calculated data to plug in to the various locations on the printed page." The engine produces **print-ready data** — every printed-sheet cell has a corresponding field in the boundary contract's printed-sheet cell map, every field is populated, every value has `TableCellRef` provenance to the canonical Paizo-table cell. The boundary contract is shaped around the printed-sheet cell locations: the `PilotReceipt` (or `PrintSheetData` sub-shape) contains a per-cell map that maps 1:1 to printed-sheet cells. The GUI does **not** render; the GUI **plugs values into cell locations** — when the UI is built to do printing, it iterates the cell map and writes each value to the corresponding cell on the printed page. No HTML/PDF templating in the engine, no HTML/PDF templating in the GUI either; the printed page is the GUI's only responsibility, and the GUI consumes the engine's cell map directly. The engine's "data the printed sheet will call" is structurally different from generic structured data: it is shaped to map 1:1 to printed-sheet cell locations, not to a general-purpose API. Recorded as `decisions.md` §12 (revised from prior pin "status quo, structured data; PRINTING-COMPLETE cell map").

### Q4 — Where does the campaign-shape `CampaignSnapshot` live if SD-21 needs it for epic 2 (the campaign manager + Drive persistence acceptance criterion)? (PINNED: status quo, SD-21 owns `CampaignSnapshot`; SD-20 doesn't)

SD-21's epic for campaign manager needs a `CampaignSnapshot` shape with party composition, per-character summaries, etc. SD-21 reads SD-19's table store directly (not SD-20's epic outputs) for spell/class data. The campaign-shape boundary contract is SD-21's, not SD-20's. But if SD-20's Epic 8 (integration closure) wants to test "campaign member sheet rendering," it might need a campaign-shape view too. Proposed resolution: SD-21 owns the campaign-shape contract; SD-20 doesn't; epic 8 only tests single-character sheets. Open until SD-21's bundle skeleton lands.

**Pinned to status quo (SD-21 owns `CampaignSnapshot`; SD-20 doesn't)** per operator directive 2026-07-16. SD-21's bundle is now closed (2026-07-15) and the question is moot — the answer is already in SD-21's `decisions.md` §1 / `technical-design.md` §1.1 / `acceptance-and-verification.md`: SD-21 defines `CampaignSnapshot`, `CampaignMetadata`, `Party`, `PartyMember`, `CharacterSummary`, `PartyResources`, `AdventureLogEntry`, `MapRef`, `WikiPage` in its own `src/rules_core/campaign.rs` module. SD-20's epic 8 (tabletop-readiness integration closure) tests single-character sheets only; it does NOT touch campaign-shape. During the parallel window (SD-20 and SD-21 running concurrently), characters in a campaign have chassis-only `CharacterSummary` (per SD-21's chassis-during-parallel-window decision); they auto-upgrade to full-detail when SD-20 closes and the user re-loads the character. SD-20's bundle just cross-references SD-21's resolved answer; no campaign-shape extension to SD-20. Recorded as `decisions.md` §13.

## All four architectural questions resolved

Q1 (boundary-contract strictness): PINNED soft. Q2 (multiclass Level Up seam signature): PINNED status quo, single delta. Q3 (engine-render vs. structured data): PINNED status quo, structured data must be PRINT-READY for the CRB. Q4 (campaign-shape `CampaignSnapshot` ownership): PINNED status quo, SD-21 owns it.

## Cross-reference

- `acceptance-and-verification.md` — closure gates.
- `decisions.md` — the 9-item decision record.
- `epic-breakdown.md` — 15 acceptance criteria grouped into 8 epics.
- `technical-design.md` — per-epic seam signatures, boundary contract shape.
- `technical-requirements.md` — pre-loop prerequisites.
- `./scope-draft.md` — canonical handoff.
- `./loop-instruction.md` — loop body.
