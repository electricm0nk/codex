---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-07-21; bundle marked planning-ready with scope tightening 2026-07-21; branch pinned tranche/5-2 / codex-tranche-5 board reused)
date: 2026-07-21
canonical_branch: tranche/5-2 (operator directive 2026-07-21)
kanban_board: codex-tranche-5 (reused from SD-22; SD-24 mint uses --board codex-tranche-5 explicitly)
companion_to: ./scope-draft.md
mirror_of: ./scope-draft.md
---

# SD-24 — Decision Record ("Why we did that")

## 1. SD-24 scope is beta-readiness + multiclass real-and-full + equipment completeness (operator directives 2026-07-21)

**Decision (original 2026-07-21):** per operator directive 2026-07-21 ("It will be tying up loose ends so that we can get ready for a beta release. i expect that we will find more defects related to unwired functions, unimplemented logic, and stubbed out things"), SD-24's primary surface is **beta-readiness remediation**: confirming no stubs, no unwired affordances, no unimplemented logic between now and the beta release. SD-24 owns **8 epics / 35 acceptance criteria** (final shape verified 2026-07-21 during package drop; the previously-circulated "30 criteria" figure was a planning-round miscount).

**Decision (scope expansion 2026-07-21 — multiclass):** per operator directive 2026-07-21 ("Full support for multiclass is a good first"), SD-24 owns **multiclass stacking real and full** scoped to **Fighter + Wizard only**, advancing each to level 10. APG/ACG-class multiclass is explicitly out of scope.

**Decision (scope expansion 2026-07-21 — per-class coverage skepticism):** per operator directive 2026-07-21 ("I'm also skeptical that the classes we brought in with APG and ACG were fully implemented - it should have taken longer than it did unless you've managed major throughput improvements in our ingest process"), SD-24 owns a **per-class coverage audit + remediation plan** for every class in the PF1 core rules + APG + ACG. The audit reports `class_features_wired` vs `class_features_expected` against canonical PF1 sources for each class.

**Decision (scope expansion 2026-07-21 — equipment completeness, strict):** per operator directive 2026-07-21 ("I also suspect that a lot of information is missing from equipment, armor and spells. last i looked there was a lot of missing information like cost, weight, etc. Full descriptions need to be present as well for all of these things"), the equipment/armor/spells corpus gets **strict 100% field coverage** for cost (where applicable), weight (where applicable), and full descriptions. Beta tester scope is full PF1 core rules + APG + ACG + Bestiary 1. No stubs. Nothing left unwired.

**Net scope-of-record:** SD-24 owns (a) beta-readiness remediation including Wired-Integration Audit Cycle + Per-Class Coverage Audit; (b) multiclass stacking Fighter+Wizard only; (c) equipment/armor/spells content completion (strict); (d) Tauri command-surface repair for iterative character mutation; (e) unwired user-facing workflow cleanup (Add Weapon/Armor/Spell onClick); (f) closure machinery (build version increment, architecture-truth-up, release notes, closure PR).

**Operator-recorded open calls (deferred from first issuance):**

- *Per-class ordering for Epic 4 (Per-class audit).* Operator-pinned at cycle launch. Default: alphabetical by class name within book.
- *Epic 6 equipment-corpus audit phasing.* Per criterion 20: `audit-and-matrix` (read-only) is mandatory; per criterion 21-24: `remediation-cycles` may proceed concurrently under the file-touch partition.
- *Equipment-corpus field-coverage threshold.* Operator-pinned at "100% strict" per the §1 directive. Any relaxation requires operator-granted override flag in `risks-and-open-questions.md`.
- *Tauri command-surface extension beyond `appendToCharacter` etc.* Default-and-flag: SD-24 covers the four-command surface extension needed for iterative character mutation. Any additional Tauri command surface work (e.g. multi-character CRUD, character-share, etc.) is out of scope.

## 2. Cycle dispatch model is deterministic-seeded-then-dynamic (operator directive 2026-07-21)

**Decision (operator-pinned 2026-07-21):** SD-24's cycle picker is **deterministic-seeded-then-dynamic**. The seed list walks the 35-criterion deterministic list during cycles 1-N; as `## DISCOVERED` entries accrue, they priority-bump into the dispatcher queue. The picker reads `## TODO` + `## DISCOVERED` together. The next-best criterion by §2.2 priority rule is dispatched.

**Reason:**

1. **Deterministic-seeded:** the 35-criterion epic/criterion list is the rules-of-engagement. The seed makes the bundle reproducible cycle-by-cycle and gives the operator a clear acceptance ledger.
2. **Then-dynamic:** real-world beta-readiness work is "find what's wrong, fix it." A pure-deterministic picker would skip over discovered-but-still-valid problems. A pure-dynamic picker would lose the seed's audit trail. The hybrid lets the cycle both walk the canonical list *and* respond to discoveries.
3. **Discovery threshold:** the `## DISCOVERED` queue growing past 10 triggers a hard stop (§2 of the loop-instruction). This prevents the cycle from chasing rabbit holes without operator attention.

**Operational consequence:** the loop-instruction's eligibility check (§2.2) reads `## TODO` + `## DISCOVERED`. The progress doc has `## TODO`, `## DONE`, and `## DISCOVERED` as top-level sections. The first cycle creates `progress.md` and seeds `## TODO` with the 35-criterion list. Each cycle that finds work writes the discovery to `## DISCOVERED`. The picker priority rule resolves ties.

## 3. Loop cadence is `/loop 1m /batch /goal` (operator directive 2026-07-21)

**Decision (operator-pinned 2026-07-21):** SD-24's loop cadence is `/loop 1m /batch`. This replaces the prior bundles' `60m` default.

**Reason:** per operator directive 2026-07-21 ("a 1-minute loop cadence is tight enough for the cycle"; "I want that loop to be dynamic, and not use a timer. Each loop should pick up and run as soon as the previous finishes"):

1. `/loop 1m` is the tightest "no perceptible timer" Hermes allows. The 1-minute floor matches the operator's hard-stop 5am deadline with 20 hours of dense run-time.
2. `/batch` partitions cycles across worker batches by file-touch — one worker per file at a time. Per the SD-23 retrospective's respawn-guard footgun, only one cycle runs at any moment on a given file.
3. The cycle picks up as soon as the prior finishes. The 1-minute floor is the minimal `sleep` between cycles; the actual cycle time is whatever the picker takes to dispatch.
4. **Discovery-forwarding rules.** A cycle that finds work outside the deterministic list writes it to `## DISCOVERED` with priority bump. The next cycle reads both queues. The discovery threshold (10 entries) prevents runaway.

**Hard-stop shape at 5am (operator option):** grace-tail is the default — at 5am the current cycle completes (no new cycles spawn), and the harness writes "stopped at cycle N" to `## Open blockers`. Absolute ceiling 5:30am regardless of cycle progress. Strict-stop at 5am is also supported if the operator pins `STRICT-STOP-AT-5AM` in `risks-and-open-questions.md` `## Override Flags`.

**Build counter inheritance (operator directive 2026-07-17 applied symmetrically):** SD-24's first concrete value lands as `0.5.<current_build>` (major `0` until first main-publish, tranche `5` because `tranche/5-2` carries tranche-base `5`, build is the next monotonic counter value after the last committed build on `develop`).

**Build counter captured 2026-07-21 (pre-launch checklist item 7):** develop's committed build is `0.5.97` (source of truth: `apps/desktop/package.json` and `apps/desktop/src-tauri/tauri.conf.json` — NOT root `Cargo.toml`, which stays pinned at `0.1.0` and is not the version-stamped surface). SD-24's first concrete build value is therefore **`0.5.98`**, landed by Epic 8 criterion 8.4.

## 4. Multiclass scope is restricted to Fighter + Wizard only (operator directive 2026-07-21)

**Decision (operator-pinned 2026-07-21):** SD-24's multiclass-real-and-full Epic 5 ships **Fighter + Wizard** only, advancing both characters from level 1 to level 10.

**Reason:** per operator directive 2026-07-21 ("we should only test multiclass on fully wired classes. that said, i don't think we need to test every possible combination. The mechanics are not rocket science, i suggest using fighter + wizard to mix combat and spells. Walk though advancing each of them up to level 10 should give us a reasonable amount of comfort that multiclass is working"):

1. **Scope restriction: Fighter + Wizard only.** Both classes have prior work in this repo (Fighter is SD-20's per-character rules-engine baseline + SD-21 Epic 9's multiclass footing; Wizard is SD-21 Epic 6's single-class completion). Other classes — APG (Alchemist, Cavalier, Inquisitor, Oracle, Summoner, Witch), ACG (Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Swashbuckler, Warpriest), and CRB's other classes (Cleric, Rogue, Sorcerer, Barbarian, Bard, Druid, Monk, Paladin, Ranger) — are out of scope for SD-24.
2. **Test surface: advancing each to level 10.** This gives sufficient coverage to confirm multiclass mechanics work without testing every permutation.
3. **APG/ACG multiclass: deferred.** APG/ACG-class multiclass is not shipped in SD-24 because (a) the operator's per-class coverage skepticism (§1) means SD-24's Epic 4 may find APG/ACG classes are not fully wired, and (b) the operator explicitly capped the test scope to Fighter + Wizard. APG/ACG-class multiclass is recorded as a deferred-work item; if Epic 4's coverage report finds the relevant classes *are* fully wired, the follow-on cycle can promote APG/ACG-class multiclass to a follow-on bundle's scope.

## 5. Equipment-corpus scope is strict 100% field coverage (operator directive 2026-07-21)

**Decision (operator-pinned 2026-07-21):** SD-24 Epic 6 ships **strict** equipment-corpus field coverage — every equipment item, armor piece, weapon, and spell in the beta-tier corpus (full PF1 core rules + APG + ACG + Bestiary 1) must carry cost (where applicable), weight (where applicable), and a full description.

**Reason:** per operator directive 2026-07-21 ("strict … we need the full rule set, all content. no stubs, nothing left unwired"):

1. **Strict 100% field coverage** is the operator-chosen threshold. Operator-selected Option A.
2. **Beta-tier corpus definition:** PF1 core rules + APG + ACG + Bestiary 1. Full content; not a representative sample. (Sample-based coverage creates asymmetric defects and was explicitly rejected.)
3. **Field-coverage audit:** Epic 6's first cycle is an audit that produces the field-coverage matrix at `artifacts/epic_6/equipment-coverage-matrix.md`. Subsequent cycles remediate per-row. Closure requires 100% field coverage.

**Threshold relaxation:** if the audit reveals the strict threshold cannot be met within SD-24's cycle budget, the cycle writes to `## Open blockers`. The operator decides whether to (a) extend the bundle, (b) relax the threshold with an operator-granted override, or (c) defer the leftover to a follow-on bundle.

## 6. Publish mode is move-not-copy (operator directive 2026-07-21)

**Decision (operator-pinned 2026-07-21):** SD-24's package publishes to the repo by **moving**, not copying. The workspace-side planning directory is deleted on the publish commit.

**Reason:** per operator directive 2026-07-21 ("when you do that publish, you need to MOVE, not copy the package. this is our signal that the package is released to DEV. I don't want 2 copies laying around. we plan in workspace, we execute in the repo"):

1. **Move-not-copy is the canonical "released to DEV" signal.** When the workspace-side copy is gone and the repo-side copy exists, the package is in execution mode.
2. **Workspace becomes planning-only.** The workspace is the planning surface; the repo is the execution surface.
3. **No second copy.** A second copy = stale source-of-truth = wrong answers from the harness.
4. **Cleanup at publish:** as part of the publish operation, the operator also cleans up the workspace-side prior SD-22 / SD-23 / SD-21 / SD-20 / SD-19 files — those have already landed in the repo-side `docs/release/` tree and have no business in the workspace any longer.
5. **The SD-22 boundary:** per operator directive 2026-07-21, SD-22 has landed in develop; SD-22's workspace-side copy and its workspace-side mirror directory are deleted at publish.

**Historical-canonical mirror tree retention:** per operator directive 2026-07-21 ("keep this historical mirror, but i don't think there should be anything from SD-16 forward in there"), the workspace's `programs/.../requirements/` mirror tree is retained for SD-11 / SD-12 / SD-13 / SD-14 / SD-15 only. SD-16 / SD-17 / SD-18 / SD-19 / SD-20 / SD-21 / SD-22 / SD-23 are deleted from the mirror at publish. Future cleanup on the older mirror is operator-deferred.

## 7. Kanban is the durable receipt layer, not the dispatch layer (operator directive 2026-07-21)

**Decision (operator-pinned 2026-07-21):** SD-24 does not use kanban for work dispatch. Kanban cards are minted on `codex-tranche-5` *only* with a done receipt.

**Reason:** per operator directive 2026-07-21 ("kanban is not to dispatch work, this happened last run and we needed to stop it. kanban is the durable receipt layer, not the dispatch layer. cards are created on kanban only with a done receipt. claude code will handle the looping and dispatching, we just need to instruct claude how to get started and how to maintain itself"):

1. **No dispatch from kanban.** Kanban does not own the cycle picker; the loop-instruction picker does.
2. **No dispatch-state on kanban.** A card's status does not gate dispatch. The progress doc's `## TODO` + `## DISCOVERED` queues are the dispatch source-of-truth.
3. **Cards minted only with done receipt.** A cycle's Step 10 mints the card after Step 7 writes the artifact. The card exists *because* the cycle's receipt was already written.
4. **Claude Code is the loop driver.** `/loop /batch /goal ./loop-instruction.md` is the engine; the loop-instruction is the operator-edited maintenance manual; kanban is the audit trail.

**Respawn-guard footgun handling:** per duracon 2026-07-04 12:41:37, when a CODE slice lands a PR and writes a PR-URL into the kanban card's comment stream, the guard emits `respawn_guarded` on every subsequent ready cycle. SD-24's architecture obviates this: the dispatcher is in the loop-instruction's Step 2 picker, not in kanban's ready-queue. Even if a cycle mints a card with a PR-URL comment, the next dispatch reads `## TODO` + `## DISCOVERED`, not the card's status.

## 8. Operator-deferred shape decisions for SD-24

**Decision:** multiple operator-deferred shape decisions for SD-24 are recorded as scope-of-record open calls, not blocked on the bundle's first cycle:

- **Override-flag mechanism.** The `## Override Flags` block in `./risks-and-open-questions.md` is operator-pinned at cycle launch. Default: empty (no overrides); any criterion with an active override flag is suppressed from dispatch until the operator clears it.
- **Discovery-forwarding rule.** Each cycle's artifact lists its discoveries in the receipt; the cycle writes the discoveries to `## DISCOVERED` with a suggested epic-and-criterion. The picker handles the priority-bump automatically.
- **Bundle size budget.** Per SD-21's `decisions.md §20`, no bundle-size budget has been pinned. SD-24's size is 8 epics / 35 criteria. The budget is operator-pinned.
- **Files in this folder.** Per the SD-23 doc-tree shape (`docs/release/SD-23-character-mutation-and-wired-integration/`), SD-24 carries 11 canonical files plus `artifacts/` and `references/`. The promotion skill (`release-package-promotion`, per the system context) refuses to copy if any are missing.
- **Release notes shape.** Per the template's REQUIRED_NOTES_SECTIONS, release notes require: Summary, User-Visible Changes, Defects Fixed, Operational Notes, Verification Evidence, Known Issues, Update Eligibility.
