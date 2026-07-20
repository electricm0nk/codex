---
title: SD-20 — Decision Record ("Why we did that")
status: approved (operator review 2026-07-16; changes noted: §2 broadened to any class/any level, Q2 revised to class-selection trigger mechanic, Q3 revised to print-ready data; SD-20 launches on tranche/4 branch)
date: 2026-07-15
companion_to: /home/ubuntu/workspace/SD-20-rules-engine-completeness-scope-draft.md
---
dec
# SD-20 — Decision Record

This file captures the deliberate choices made in the SD-20 planning conversation on 2026-07-15. Each item includes the decision, the displaced alternative(s) with reasons they lost, and the reason the winner won. Future sessions asking "why is SD-20 shaped this way" should find the answer here without re-litigating it.

SD-20 exists because the operator explicitly named tranche-4's deliverable on 2026-07-14: *"my expectation for tranche 3 is that someone can use codex to build a new character, print out a character sheet, and then use it on a real life tabletop role playing session with dice and everything. That everything on the character sheet is accurate. All calculations are true. All information about the generated character is available."* Reading that against what SD-18 + SD-19 actually ship (chassis grounding + corpus reachability with bounded baseline stats, but no spell effects, no feat effects, no damage totals, no skill ranks, no equipment-effect breadth, no Level Up grant model, no canonical Paizo-table store), the gap is structural and named: the engine cannot produce a printed sheet a user can take to a real PF1 table. SD-20 closes that gap.

## 1. SD-20 is the integration STC for tranche-4; epics within it, not sub-bundle SD-Ns

**Decision:** SD-20 is the full 9-file STC covering tabletop-readiness acceptance criteria, the engine-side boundary contract, the wire-fixture parity tests, and the Level Up grant integration doctrine. Tranche-4's per-character deliverable work lives as **epics inside SD-20**, each with its own section in `epic-breakdown.md` and (if it grows large enough) its own nested decision/technical-design doc inside the SD-20 bundle directory. Campaign manager + Drive persistence + APG + ACG ingestion are NOT an SD-20 epic — they're promoted to their own top-level bundle SD-21 (per operator directive 2026-07-15).

**Displaced alternative:** Promoting each tranche-4 subsystem to its own top-level STC bundle (SD-21 spellbook engine, SD-22 feat prerequisite engine, SD-23 skill-rank allocation engine, SD-24 damage-total and equipment-effect engine, SD-25 Level Up grants; with SD-21 reserved for campaign manager + Drive persistence + APG + ACG ingestion).

**Reason:** Per operator directive 2026-07-15 ("i thought we have all of Tranche-4 in SD-20. If it was split, all the STC files and folders are missing. Remember, SD is spec domain, it's top level and done numerically. Below that we have epics and stories"): the SD-N numbering is a program-level convention; the spec domain sits at the top level and below it live epics and stories. Tranche-4's per-character work is one product-surface deliverable (a printable tabletop sheet a user can take to a real PF1 table with dice); one spec-domain bundle (SD-20) captures it. Each piece of work inside SD-20 is an epic with its own acceptance criteria, not a separate SD. Campaign manager + Drive persistence + APG + ACG ingestion are large enough to warrant their own top-level lifecycle and have been promoted to SD-21 in a separate bundle.

This also matches how SD-18 and SD-19 are shaped: SD-18's acceptance criteria live as epic-stories in `epic-breakdown.md` (race rows, class rows, interaction rows, spell schools, equipment categories), and SD-19's foundation slice + main capability slice live as two pre-loop epic-stories in the same file. SD-20 follows the same pattern at one level larger.

## 2. Tabletop-readiness is the user-facing acceptance criterion, not "every engine feature implemented"

**Decision:** SD-20's closure gate is "any class at any level 1-20 can be built, have feats selected, skill ranks allocated, a chosen weapon's damage computed, and a chosen spell's effect described — all using the rules engine — then the resulting sheet prints, and a user takes it to a real PF1 table and uses it with dice." The canonical scenario is not 'Human Fighter at level 1' specifically; it is **any of the 11 core classes at any level 1-20**, with the integration closure epic shipping one canonical fixture per core class at level 1 plus a smaller sample of higher-level fixtures to ground multi-level mechanics (feat picks at level 3+ for Fighter, spell picks at level 2+ for prepared casters, ASI eligibility at level 4/8/12/16/20, capstone thresholds at level 20). This is the load-bearing user acceptance criterion; subsystem-level "spellbook engine implemented" or "damage engine implemented" are intermediate progress markers, not closure gates. Per operator directive 2026-07-16: "decisions §2 says a level-1 fighter. in reality, it should be any class, any level."

**Displaced alternative:** Closure per subsystem-engine completeness (e.g. "spellbook engine has tests passing for all prepared-spell mechanics across all 9 schools").

**Reason:** Per operator framing 2026-07-14: "i am speaking to you as a product owner at this point. My interest is in what is being delivered for the users, not in what is completed from an engineering perspective." Tabletop-readiness is the user-facing outcome. Closing on subsystem-engine completeness would pass code review but leave the user without a printable sheet, which is exactly the gap the operator named. The subsystem engines exist to serve the tabletop-readiness deliverable, not the other way around.

## 3. The canonical Paizo-table store from SD-19's foundation slice is the load-bearing authority surface

**Decision:** SD-20 reads the `src/rules_core/rules_tables/crb/` directory populated by SD-19's foundation slice as the authority for every class table cell, every spell description, every equipment stat, every feat text, and every skill entry. No subsystem engine hard-codes its own copy of any PF1 table cell. When the SD-20 epics (boundary contract, spellbook engine, feat prerequisite engine, skill-rank allocation engine, damage-total engine, equipment-effect engine, Level Up grants) need a value, they look it up in the table store. SD-21 (campaign manager + Drive persistence + APG + ACG ingestion) reads from sibling directories `src/rules_core/rules_tables/apg/` and `src/rules_core/rules_tables/acg/` per the SD-19 §9 source-book subdirectories decision; SD-21 does not consume SD-20's epic outputs, it consumes SD-19's table store directly.

**Displaced alternative:** Each subsystem engine owns its own copy of the table cells it needs (spellbook engine ships spell descriptions, feat engine ships feat text, etc.).

**Reason:** Per operator doctrine 2026-07-14: "I'm inclined to side with the table format vs the computed version just because that is how the rules are written by Paizo. They don't give formulas, they provide tables." The table store is the single source of truth; subsystem engines consume from it. This avoids the failure mode the operator named (backend's integer-division formulas diverging from Paizo's actual tables once multiclassing or expanded rule sets enter), and it makes future rule books (UM, APG, whatever lands this weekend) a matter of populating a new sibling directory under `rules_tables/` rather than re-architecting every engine.

## 4. Backend owns every cell; GUI renders

**Decision:** Every value on a printed character sheet — ability mods, BAB, saves, HP, AC, attack bonus, skill totals, spell DCs, damage expressions, equipment stats — comes from the rules engine (`pilot_compute.rs` + the SD-19 `pilot_compute_corpus.rs` seam + the four subsystem engines). The GUI renders; it does not compute. The wire-fixture parity tests in SD-20 are the dovetail mechanism that proves the GUI renders the same values the engine produces.

**Displaced alternative:** GUI continues to compute some cells locally (the `characterProgression.ts` hand-typed table pattern) and the engine computes others; the two are reconciled by best-effort review.

**Reason:** Per operator framing 2026-07-14: "the backend needs to do all the computations, not just at level up." And on Paizo tables vs. computed formulas: "the math in the formulas may work now, but it might break when we get further into multiclassing or expanded rules sets. It would be best to stick with what is published from paizo." Splitting compute between GUI and engine guarantees divergence the moment multiclassing enters; both sources can be wrong in the same way, and no test catches it. Backend-owned, GUI-rendered means one source of truth, one test surface, divergence caught at the boundary.

## 5. Boundary contract + wire-fixture parity tests are the dovetail mechanism between backend and GUI

**Decision:** SD-20 ships a boundary contract (a structured specification of every `CharacterInput` shape the engine accepts, every `PilotReceipt` shape it returns, and every cell the printed sheet displays) plus a set of wire-fixture parity tests. The parity tests are golden JSON files checked into the repo: each file is a complete `CharacterInput` + the exact `PilotReceipt` the engine must produce for it. Both the engine test suite and the GUI test suite read the same fixtures; a divergence between what the engine computes and what the GUI renders fails tests on both sides.

**Displaced alternatives:**

- *Backend STC owns the contract; GUI implements to it without shared fixtures.* Drift surfaces as visual bugs the operator catches manually.
- *Backend and GUI STCs each own their own contract docs; integration is by review.* Drift surfaces as reviewer disagreement.

**Reason:** Per operator framing 2026-07-14: "I don't think I can let you go off and build the ui on your own through loop sessions. Nor do i think i can handle front-end to back-end wiring through a vibe coding session on my own. We need to define a method to dovetail those efforts together." The fixtures are the source of truth; the contract doc is the human-readable mirror. The operator's GUI work (vibe-coded outside the bundle, per "let's leave it outside for now") consumes the fixtures directly. The engine's loop cycles consume the same fixtures for RED tests. Both sides fail when they diverge.

## 6. Frontend stays outside the bundle

**Decision:** SD-20 owns the engine-side boundary contract and the wire-fixture parity tests. The GUI implementation (character sheet rendering, CreateCharacter form) stays outside the bundle, vibe-coded by the operator per their established workflow. Campaign manager GUI work (PR #316's `apps/desktop/src/campaign/` screens) lives outside the bundle too — it belongs to SD-21, which is its own top-level spec-domain bundle with its own doctrine capture. If the GUI implementation needs its own doctrine capture beyond what SD-20's boundary contract provides, that becomes a separate `tranche-4-1` bundle at the operator's call.

**Displaced alternative:** SD-20 owns the GUI STC too — frontend contract, component-level specs, accessibility requirements, design tokens.

**Reason:** Per operator directive 2026-07-14: "let's leave it outside for now. if need be we can add a tranche-4-1." The operator does the GUI work via vibe coding sessions because that's what works for intensive UI design work; the bundle cannot own what the operator needs to control themselves. SD-20's contract-and-fixtures model gives the operator the right seams to wire the GUI against without the bundle having to specify how the GUI looks or behaves. Campaign manager GUI work flows through SD-21 (which has its own bundle, decisions, and contract surface for the campaign epic).

## 7. Bundle split: epics inside SD-20, ordered by dependency

**Decision:** Per the dependency analysis from 2026-07-14 (revised 2026-07-15 after the operator clarified the SD-N convention and then promoted campaign manager to its own top-level bundle), the per-character tranche-4 epic ordering inside SD-20 is:

- **Epic 1: Boundary contract + wire-fixture parity tests** — the engine-side boundary contract (every `CharacterInput` shape the engine accepts, every `PilotReceipt` shape it returns, every printed-sheet cell) plus the wire-fixture parity tests (golden JSON fixtures both engine and GUI consume). Other epics design against this contract. Lands first because every downstream epic produces into the boundary contract.
- **Epic 2: Spellbook engine** — spell effects, prepared-spell mechanics, spell save DCs, bonus slots from high ability. Depends on SD-19's table store only.
- **Epic 3: Feat prerequisite engine** — feat eligibility checks (prerequisites met vs. unmet), feat effects applied to derived stats. Depends on SD-19's table store only.
- **Epic 4: Skill-rank allocation engine** — per-level skill rank totals, class-skill bonuses, untrained vs. trained split, max ranks at level. Depends on SD-19's table store only.
- **Epic 5: Equipment-effect engine** — full derived stats beyond SD-19's bounded baseline (every field on every Paizo equipment entry, not just AC / attack / max dex / spell failure). Depends on SD-19's table store only.
- **Epic 6: Damage-total engine** — weapon damage rolls (incl. critical hits), depends on Epic 5's equipment stat breadth. Lands after equipment.
- **Epic 7: Level Up grant model** — what the user gets at each level transition (free features, pick-from lists, ASI eligibility, feat picks, spell picks, skill rank pool). Depends on Epics 2, 3, 4, 5, 6 (spell picks, feat picks, skill rank pool, equipment picks). Lands after the compute epics close.
- **Epic 8: Tabletop-readiness integration closure** — the full `CharacterInput → PilotReceipt → printed sheet` pipeline works end-to-end for **any of the 11 core classes at any level 1-20** (per the broadened acceptance criterion), with feats selected, skill ranks allocated, equipment equipped, and spell picks made — exercising the integration test fixture set defined in `epic-breakdown.md` Epic 8. Closure requires all earlier epics.

**Campaign manager + Drive persistence + APG + ACG ingestion is its own top-level bundle, SD-21, not an epic inside SD-20.** Per operator directive 2026-07-15 ("we are designing a system by which a user can build and manage their characters in whatever games they are playing... My thought was to allow externally saved character files to be written to a common central area... This campaign would/should hold more than just characters... i think this gets split. the campaign manager can be done as a part of SD-21, possibly in parallel with SD-20"): the campaign/party/Drive/DM-toolkit product surface is large enough to warrant its own top-level SD-N. SD-21 lives at `programs/codex/requirements/SD-21-campaign-manager-and-persistence/` and carries its own doctrine (campaign-shape boundary contract, Drive persistence adapter, APG + ACG ingestion epics). Tranche-5's SD-22 (DM toolkit + encounter builder + Bestiary 1 ingestion) follows downstream of both.

**Displaced alternative:** Keeping the campaign manager as an epic inside SD-20 (the original 9-epic enumeration from earlier in this conversation).

**Reason:** Per operator directive 2026-07-15: "i think this gets split. the campaign manager can be done as a part of SD-21, possibly in parallel with SD-20." The campaign epic is parallelizable with SD-20 (it consumes the boundary contract rather than producing into it), and the campaign + Drive + APG + ACG scope is large enough to warrant its own lifecycle. Per the same directive: "SD is spec domain, it's top level and done numerically. Below that we have epics and stories." SD-21 is the next program-level spec-domain bundle after SD-20.

The eight-epic ordering inside SD-20 runs as follows: Epic 1 lands first (boundary contract), then Epics 2, 3, 4, 5 all depend on SD-19's table store only and can run as concurrent loops if the operator chooses to host them that way (three independent lanes: Epic 2 spellbook, Epic 3 feat prereqs, Epic 4 skill ranks + Epic 5 equipment as a paired cycle since Epic 5's outputs feed Epic 6). Epic 6 (damage) is sequentially after Epic 5 (equipment). Epic 7 (Level Up grants) integrates after Epics 2–6 close. Epic 8 (tabletop-readiness closure) is the integration milestone.

## 8. Tranche-4 is the integration milestone; SD-20 is its per-character integration STC

**Decision:** Tranche-4 closes when SD-20's tabletop-readiness closure is met (per-character printable sheet a user takes to a real PF1 table) AND SD-21's campaign manager + Drive persistence + APG + ACG ingestion epic is shipped AND a `tranche/4 → develop` promotion PR has been merged. SD-20 is the per-character integration STC for tranche-4; SD-21 is the campaign/persistence/ingestion STC for tranche-4; SD-22 (tranche-5) is the DM toolkit + encounter builder + Bestiary 1 STC. Per operator directive 2026-07-16: SD-20's promotion is `tranche/4 → develop`, not `tranche/3 → develop` (the `tranche/3 → develop` promotion is the chassis-lane promotion, separately tracked under the SD-18 chassis-lane STC).

**Displaced alternative:** Tranche-4 = "every subsystem engine has tests passing" (engineering-completeness milestone).

**Reason:** Same as decision §2 — the user-facing outcome is the milestone. Tranche-3 closes with engine chassis + corpus reachability; tranche-4 closes with per-character tabletop-readiness plus the campaign/persistence product surface that makes the gaming pod usable in practice. SD-22's DM toolkit + encounter builder are tranche-5 deliverables, downstream of the rules-engine completeness that SD-20 ships.

## 9. CRB-only scope; future rule books get their own bundles

**Decision:** SD-20 ships tabletop-readiness for the PF1 Core Rulebook scope (the same scope SD-18 and SD-19 cover). Future rule books (UM, APG, Starfinder 1e, World of Darkness, Traveller MGT 2e, Cyberpunk Red — all named in PR #316's PR body) get their own sub-bundles that populate `src/rules_core/rules_tables/<book>/` and extend the subsystem engines. SD-20 ships CRB; the next book is a separate scope.

**Displaced alternative:** SD-20 ships multi-rule-set support out of the box.

**Reason:** Per operator directive 2026-07-14: "we are doing core rules now, but will be doing more rule books as soon as this weekend." Source-book subdirectories under one module is the shape that makes the next book's landing zone obvious without committing to it now. Each book ships its own STC sub-bundle; SD-20 stays honest about CRB-only tabletop-readiness.

## Cross-reference

- `README.md` — bundle overview, posture summary, reading rule.
- `acceptance-and-verification.md` — closure gates including tabletop-readiness.
- `epic-breakdown.md` — 15 acceptance criteria grouped into 8 epics (boundary contract + wire-fixture parity tests; spellbook engine; feat prerequisite engine; skill-rank allocation engine; equipment-effect engine; damage-total engine; Level Up grant model; tabletop-readiness integration closure).
- `risks-and-open-questions.md` — self-healable vs. non-self-healable split + open override flags.
- `technical-design.md` — boundary contract shape, wire-fixture parity test format, per-epic seam signatures.
- `technical-requirements.md` — pre-loop prerequisites for SD-20 (SD-19's foundation slice shipped; SD-19's main capability slice shipped; SD-18's loop completed).
- `./scope-draft.md` — canonical handoff.
- `./loop-instruction.md` — loop body.
- `~/workspace/programs/codex/requirements/SD-18-core-rules-breadth/` — chassis grounding.
- `../SD-19/` — corpus-aware seam + canonical Paizo-table store.
- `~/workspace/programs/codex/requirements/SD-17-pcgen-corpus-include-graph-resolution/` — corpus-side parsing work SD-20's spellbook and equipment engines consume.
- `../SD-21/` (sibling STC, parallelizable) — campaign manager + Drive persistence + APG + ACG ingestion. SD-20's boundary contract is one input to SD-21's campaign-shape boundary contract; SD-20 does not otherwise depend on SD-21.

## 10. Boundary-contract strictness pinned to soft enforcement (operator directive 2026-07-16)

**Decision:** The boundary contract (`docs/release/SD-20/boundary-contract.md`) plus the wire-fixture parity test fixtures (`tests/fixtures/wire/sd20/*.json`) are the sole authority surface for what the engine produces and what the GUI renders. No compile-time check, no codegen, no contract-validation binary, no enforcement hook in `cargo build`. An epic that adds a new `PilotReceipt` field without extending the contract doc and adding a parity-test fixture is not blocked at build time; divergence surfaces at integration (either as a compile error when one epic reads a field another writes inconsistently, or as a wire-fixture parity test failure when the contract doc and the actual `PilotReceipt` shape diverge).

**Displaced alternative:** Hard enforcement — a contract-validation binary, a `cargo build` hook, or a CI step that verifies every `PilotReceipt` field has a corresponding entry in the boundary contract doc and a parity-test fixture. An epic that adds a new field without extending the contract fails to land.

**Reason:** Per operator directive 2026-07-16: "soft." Hard enforcement adds a build hook or codegen layer for a problem SD-19 already solved by convention. SD-19's seam (`pilot_compute_corpus.rs`) was added without a compile-time check; the contract was prose + tests, not enforced invariants. SD-20 follows the same pattern. The wire-fixture parity test catches divergent fields within the epic's own cycle (the per-epic test reads the fixture and asserts `pilot_receipt == fixture.expected_output`). Cost of hard: a contract-validation binary, a build hook, and the maintenance burden of keeping it in sync with the prose contract. Benefit: catching a divergence one cycle earlier. Not worth it for CRB scope.

Mirrored as `risks-and-open-questions.md` Q1 (PINNED: soft).

## 11. Multiclass Level Up interaction uses class-selection trigger mechanic (operator directive 2026-07-16, revised)

**Decision:** Epic 7's seam signature is `compute_level_up_grants(character: &CharacterInput, from_level: u8, to_level: u8, rules_tables: &RulesTables) -> LevelUpPlan`. Single-delta. The Level Up mechanic is the **same mechanic as character creation** (level 0 → level 1): the engine triggers a class-selection event when the user picks a class (or hits a level transition that requires class-feature resolution), and from there the engine identifies which additions are auto-grants and which are pick-from-lists. The new `CharacterInput` carries the class selection already made via the class-selection trigger, and the engine produces the `LevelUpPlan` describing what auto-grants and what pick-from-lists apply given the level transition AND the class selection. The engine handles per-class iteration internally: it reads the character's `class_summary` (already populated by SD-18's chassis work, which encodes per-class levels) and iterates over each class the character has, applying that class's per-class-level grants in the range `[class_level_at(from_level), class_level_at(to_level)]`. The `LevelUpPlan.automatic_features` and `pick_from_lists` are populated with `TableCellRef` provenance that includes the class name and class level. Multiclass correctness is verified by epic 8's integration fixture set (which includes one multiclass character per the broadened acceptance criterion: one Fighter 2 / Wizard 1 at total level 3 fixture).

**Displaced alternative (prior pin, superseded):** Status quo, single-delta seam signature with no explicit class-selection trigger mechanic — the engine iterates per-class internally, and multiclass correctness is verified by epic 8's integration test alone. (This was the prior §11 pin from 2026-07-16 morning; superseded by the operator's class-selection-trigger directive later the same day.)

**Reason:** Per operator directive 2026-07-16: "yes, level up needs to trigger a selection on which new class is being selected and from there identify what additions are specified and which are options. this is the same mechanic as a new character that is progressing from level 0 to level 1." The class-selection trigger makes the Level Up mechanic explicit at the boundary contract — the engine has a contract surface for "this level transition requires class-feature resolution, please pick a class" rather than silently inheriting the prior class selection from the input. This matches the character-creation mechanic (level 0 → level 1 also triggers a class selection), so the engine has one mechanic for both flows rather than two near-duplicate paths. The seam signature stays single-delta; what changes is the input shape's relationship to the class-selection trigger.

Mirrored as `risks-and-open-questions.md` Q2 (PINNED: yes, class-selection trigger; Level Up is the same mechanic as level-0-to-1).

## 12. Engine produces print-ready data; UI plugs values into cell locations (operator directive 2026-07-16, revised)

**Decision:** The engine produces **print-ready data** — every printed-sheet cell has a corresponding field in the boundary contract's printed-sheet cell map, every field is populated, every value has `TableCellRef` provenance to the canonical Paizo-table cell. The boundary contract is shaped around the printed-sheet cell locations: `CharacterInput` in, `PrintSheetData` (or a `PilotReceipt` containing a `PrintSheetData` sub-shape) out, plus a per-cell map that maps 1:1 to printed-sheet cells. The engine has no HTML output, no PDF output, no templating layer, no rendering library, no font handling. The GUI does **not** render in the templating sense; the GUI **plugs values into cell locations** — when the UI is built to do printing, it iterates the cell map and writes each value to the corresponding cell on the printed page. The printed page is the GUI's only responsibility. The GUI consumes the engine's cell map directly.

**Tabletop-readiness (gate 10) is the engine-completeness criterion.** It is met when `PrintSheetData` for the canonical scenario (any of the 11 core classes at any level 1-20 per the broadened acceptance criterion) has every cell populated with values matching `TableCellRef`-referenced table cells. The cell map is complete; the cell map values are correct; the GUI can plug the cell map into a printed sheet. Physical print workflow (the page layout, the OS print dialog, the click-Print-and-paper-comes-out experience) is downstream and operator-owned.

**Displaced alternative (prior pin, superseded):** Status quo, structured data with PRINT-READY cell map — the engine produces generic structured data (`PilotReceipt`) and the GUI does the rendering. (This was the prior §12 pin from 2026-07-16 morning; superseded by the operator's print-ready-data directive later the same day.)

**Displaced alternative (also displaced):** Engine produces render-ready HTML/PDF. The boundary contract extends to `PilotReceipt.rendered_html: String` and `PilotReceipt.rendered_pdf: Option<Vec<u8>>`. The engine grows a templating layer.

**Reason:** Per operator directive 2026-07-16: "no. it needs to produce the data that will be called by the printed sheet. when the ui is built to do printing, it should be able to simply call the calculated data to plug in to the various locations on the printed page." The engine's contract is shaped around the printed-sheet cell locations, not around a general-purpose API. The GUI's job is the printed page; the engine's job is "what value goes in each cell." This separation makes the boundary contract a 1:1 mirror of the printed-sheet layout, which is the simplest possible contract surface for the printed-sheet use case. No HTML/PDF templating in the engine (operator-rejected; engine doesn't grow a templating dependency). No HTML/PDF templating in the GUI either (the operator's framing is "plug values into cell locations," not "render a styled document"); the GUI's work is page layout and cell positioning, both of which are operator-vibe-coded work outside the bundle per `decisions.md` §6.

**What this means for epic 8.** Epic 8's tabletop-readiness integration closure verifies the engine produces a complete, correct `PrintSheetData` for the broadened canonical fixture set (11 core classes at level 1 + sample at higher levels + one multiclass character). Each fixture's `expected_output` matches the values printed by Pathbuilder 2e for the same character. The GUI's print plug-in code consumes the same fixtures and verifies the rendered page has the same values in the right cells.

Mirrored as `risks-and-open-questions.md` Q3 (PINNED: print-ready data; UI plugs values into cell locations).

## 13. Campaign-shape `CampaignSnapshot` lives in SD-21, not SD-20 (operator directive 2026-07-16)

**Decision:** SD-21 owns `CampaignSnapshot`. SD-20 does not grow a campaign-shape view. SD-21's bundle (`../SD-21/`, closed 2026-07-15) defines `CampaignSnapshot`, `CampaignMetadata`, `Party`, `PartyMember`, `CharacterSummary`, `PartyResources`, `AdventureLogEntry`, `MapRef`, `WikiPage` in its own `src/rules_core/campaign.rs` module. SD-21's `decisions.md` §1 / `technical-design.md` §1.1 / `acceptance-and-verification.md` gate 8 document how SD-21 composes `CharacterSummary` from SD-20's `PilotReceipt` once SD-20 closes. SD-20's epic 8 (tabletop-readiness integration closure) tests single-character sheets only; it does NOT touch campaign-shape.

**During the parallel window (SD-20 and SD-21 running concurrently):** characters in a campaign have chassis-only `CharacterSummary` (per SD-21's chassis-during-parallel-window decision; see SD-21's `acceptance-and-verification.md` gate 8 and `risks-and-open-questions.md` Flag A). They auto-upgrade to full-detail `CharacterSummary` when SD-20 closes and the user re-loads the character. SD-21's `src/rules_core/campaign.rs::CharacterSummary` reads from `src/rules_core/contract.rs::PilotReceipt` once that contract lands; before that, it reads from SD-18's chassis-only data.

**SD-20's contribution:** SD-20's boundary contract (`docs/release/SD-20/boundary-contract.md`) is one input to SD-21's campaign-shape boundary contract — specifically, `PilotReceipt` fields become `CharacterSummary` fields once SD-20 closes. SD-20 does not otherwise depend on SD-21. SD-20's loop runs cycles that ship `PilotReceipt` fields; SD-21's loop runs cycles that consume those fields. The two bundles share `tranche/4` (per operator directive 2026-07-16: SD-20 launches on `tranche/4`; SD-21's launch branch is a separate operator call) and have separate lifecycles. SD-20's kanban board is `codex-tranche-4`; SD-21's kanban board is the operator's separate call (per the operator's directive: codex-tranche-4 is SD-20's lane specifically).

**Displaced alternative:** SD-20 grows a campaign-shape view. SD-20's epic 8 includes a campaign-shape test fixture; SD-20's `technical-design.md` documents how `PilotReceipt` flows into `CampaignSnapshot.CharacterSummary`; SD-20 owns the cross-bundle shape, and SD-21 reads SD-20's campaign-shape view as one of its inputs.

**Reason:** Per operator directive 2026-07-16: "status quo." The Q4 question was originally raised on 2026-07-15 as a forward-looking concern ("what if SD-21 needs `CampaignSnapshot` before SD-20 closes?"). SD-21's bundle has since closed and settled the question — SD-21 owns the campaign-shape contract; SD-20 doesn't grow it. From SD-20's perspective, the question is moot. Cross-referencing SD-21's resolved answer keeps both bundles internally consistent without duplicating the campaign-shape contract surface in two bundles. If a future cycle discovers SD-20 needs to grow a campaign-shape view (e.g. SD-21's `CharacterSummary` needs a field that lives in SD-20's contract), the fix is a patch to SD-21 to read from SD-20's boundary contract — not a campaign-shape extension to SD-20.

Mirrored as `risks-and-open-questions.md` Q4 (PINNED: status quo, SD-21 owns `CampaignSnapshot`; SD-20 doesn't).

## 14. All architectural questions resolved (closure summary)

Q1 (boundary-contract strictness): PINNED soft — markdown contract + JSON fixtures only; no compile-time enforcement.
Q2 (multiclass Level Up seam signature): PINNED class-selection trigger — single-delta seam signature `compute_level_up_grants(character, from_level, to_level, rules_tables) -> LevelUpPlan`; the input `CharacterInput` carries the class selection already made via the class-selection trigger (same mechanic as level-0-to-1 character creation); engine handles per-class iteration internally once the selection is in the input.
Q3 (engine-render vs. structured data): PINNED print-ready data — engine produces `PrintSheetData` shaped around the printed-sheet cell map (every cell has a field, every field has `TableCellRef` provenance); GUI plugs values into cell locations; no HTML/PDF templating in either engine or GUI.
Q4 (campaign-shape `CampaignSnapshot` ownership): PINNED status quo — SD-21 owns `CampaignSnapshot`; SD-20 doesn't grow a campaign-shape view; cross-references SD-21's resolved answer.

No remaining open architectural questions for SD-20. The next-class concerns (multiclass Level Up correctness, GUI cell-map plug-in, campaign-shape composition) are verified by the wire-fixture parity tests, the integration-closure epic, and the operator's GUI work — not by additional bundle doctrine.

## 15. SD-20 launches on `tranche/4` branch (operator directive 2026-07-16)

**Decision:** SD-20 commits go to a new integration branch `tranche/4`, not to `tranche/3`. The `tranche/3` branch remains the chassis lane (SD-18 + SD-19) integration branch; `tranche/4` is the per-character-rules-engine lane integration branch. SD-21's launch posture on `tranche/3` vs. `tranche/4` is a separate operator decision (out of scope for this §15). Per operator directive 2026-07-16 (later same day): use the slash form `tranche/4` to keep in line with the prior `tranche/3` naming convention.

**Reason:** Per operator directive 2026-07-16: "SD-20 should use a new branch: tranche-4." (and the later refinement to use slash form). The chassis lane and the per-character-rules-engine lane are different lifecycles with different closure criteria; a fresh branch gives the per-character-rules-engine lane its own commit history, its own promotion cadence, and its own rollback surface. `tranche/3 → develop` promotion remains the chassis-lane promotion; `tranche/4 → develop` promotion is the per-character-rules-engine-lane promotion, separately operator-driven.

**Operational consequence for the loop.** The loop-instruction's Step 3 (working-tree check) and Step 6 (commit and push) use `tranche/4`, not `tranche/3`. The kanban board for SD-20 cycle records is `codex-tranche-4` (operator directive 2026-07-16: new board to separate SD-20 cycles from the chassis-lane `codex-tranche-3` boards; operator creates the board after `tranche/3` is merged). The promotion PR at SD-20 closure is `tranche/4 → develop`, not `tranche/3 → develop`.

**Operational consequence for SD-21.** Out of scope here — separate operator call. If SD-21 also moves to `tranche/4`, the SD-21 loop-instruction needs the same `tranche/3` → `tranche/4` patch; the SD-21 kanban board stays at `codex-tranche-3` (per the original directive: codex-tranche-4 is SD-20's lane specifically) unless the operator decides otherwise; the cross-bundle §2 of SD-20's scope draft ("SD-21 has landed its campaign manager + Drive persistence + APG + ACG ingestion epics") needs to be re-read against SD-21's actual launch branch.