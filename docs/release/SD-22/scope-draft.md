---
title: SD-22 — Content-Source Ingest (APG + ACG + Bestiary 1) + DM Toolkit — Scope Draft (Tranche-5 Bundle)
status: approved (operator review 2026-07-15 scope; operator directives 2026-07-17 expanded scope to APG + ACG + Bestiary 1 + DM toolkit; operator clarification 2026-07-18: "ACG, APG are the two advanced guides" (not Ultimate Combat / Ultimate Magic); branch + board pinned 2026-07-18 to tranche/5 / codex-tranche-5; 9 epics / 31-criteria final shape (Epic 9 — Closure Readiness added 2026-07-19); bundle marked approved with operator directives 2026-07-15/17/18/19)
date: 2026-07-15
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/decisions.md
mirror_of: /home/ubuntu/workspace/programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/decisions.md §1
canonical_branch: tranche/5 (operator directive 2026-07-18)
kanban_board: codex-tranche-5 (operator directive 2026-07-18)
---

# SD-22 — Scope Draft (Tranche-5 release)

> ## ⚠️  OPERATING METHOD — REQUIRED FOR THIS BUNDLE  ⚠️
> 
> **This bundle is operated via the `/loop 60m /batch /goal` invocation model — NOT a one-shot task.** After exiting plan mode, the coding harness is **required** to launch SD-22 as:
> 
> ```bash
> /loop 60m /batch /goal ./loop-instruction.md
> ```
> 
> The full per-cycle procedure, file-touch partition, post-mortem card mint, and progress-doc update live in the loop-instruction file body. The scope-draft (this file) is the canonical handoff *what* — the loop-instruction is the *how*. See the loop-instruction's leading `⚠️ OPERATING METHOD — REQUIRED FOR THIS BUNDLE` block for the verbatim launch instruction and pre-launch checklist.

## 0. Preamble

This is the canonical handoff for SD-22. The `/loop 60m /batch /goal ./loop-instruction.md` invocation reads this file plus its sibling doctrine files and runs to closure.

Working in bounded cycles against the integration branch `tranche/5` (per operator directive 2026-07-18; SD-22's branch is `tranche/5`, NOT `tranche/3` or `tranche/4` or `tranche/4-1`; SD-22 doesn't inherit from SD-21's `tranche/4-1` lane). Each cycle lands one acceptance criterion.

The progress doc `./progress.md` (created on first cycle by the loop) carries the cycle-log + status matrix (per `governance/spec-domain-lifecycle.md`'s plan A on the SD-status transcription surface).

## 1. SD-22 — 31 criteria across 9 epics on `tranche/5`

SD-22 ships nine epics, each with its own capability slice. Code-Side Identifier Cleanup (Epic 1) fires FIRST on shared files; Closure Epilogue (Epic 7) fires LAST, only after Epic 9 (Closure Readiness) has dispatched it. **Epic 9 was added 2026-07-19** to decouple the eval-and-self-heal step from Epic 7's actual PR and release-notes work — Epic 9 evaluates every prior criterion 1-30 against artifact evidence, self-heals shortfalls (open-ended until clean), then dispatches Epic 7. Per operator directives 2026-07-15 + 2026-07-17 + 2026-07-18, SD-22 owns content-source ingest for **APG + ACG + Bestiary 1** + the **DM toolkit**. SD-21 reads from SD-19's `rules_tables/crb/` only; SD-22 owns every other `RuleSetId::*` content.

### 1.1 Epic 1 — Code-Side Identifier Cleanup (governance base requirement; fires FIRST)

Under the identifier-discipline doctrine (`../../doctrine-external/identifier-discipline.md`), source-code identifiers must describe what the artifact does, not which release or spec domain it came from. Epic 1 audits source for `sd22_*` / `Sd22` / `SD-22-Ex...` / `t_<hex>` leaks and removes them (defensive cleanup since SD-22 doesn't ship new Tauri commands but may inherit old identifiers from earlier sessions). Detailed acceptance criteria 1-2 in `programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/epic-breakdown.md` §"Epic 1 — Code-Side Identifier Cleanup."

### 1.2 Epic 2 — Operator Pre-Launch (board-exists, branch-pushed, OAuth-credentials-pinned)

SD-22's gating epic. Three pre-flight checks the operator runs once before the loop's first cycle: (3) `codex-tranche-5` kanban board pinned; (4) `tranche/5` branch pushed to origin; (5) no `claude` processes in-flight on `tranche/5`. Detailed acceptance criteria 3-5 in `epic-breakdown.md` §"Epic 2 — Operator Pre-Launch."

### 1.3 Epic 3 — APG content-source ingest (per-class cycles)

Per-class cycle shape (one cycle per APG class). APG populates `src/rules_core/rules_tables/apg/` per SD-19 §9 source-book subdirectory pattern. APG classes: Alchemist, Cavalier, Gunslinger, Inquisitor, Magus, Oracle, Summoner, Witch (plus any APG printing additions). Each cycle lands one class table plus its spell/equipment integration. The `RuleSetId::Apg` variant ships; cross-book resolution tests assert APG-only items return `Some` for `RuleSetId::Apg` queries and `None` for `RuleSetId::Crb` queries. Detailed acceptance criteria 6-9 in `epic-breakdown.md` §"Epic 3 — APG content-source ingest."

### 1.4 Epic 4 — ACG content-source ingest (per-class cycles)

Symmetric to Epic 3, for ACG content. ACG populates `src/rules_core/rules_tables/acg/`. ACG classes: Alchemist (ACG side), Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Swashbuckler, Warpriest. The `RuleSetId::Acg` variant ships; cross-book resolution tests assert ACG-vs-CRB and ACG-vs-APG resolution. Detailed acceptance criteria 10-13 in `epic-breakdown.md` §"Epic 4 — ACG content-source ingest."

### 1.5 Epic 5 — Bestiary 1 content-source ingest (per-monster-block cycles)

Per-monster-block cycle shape. Bestiary 1 populates `src/rules_core/rules_tables/beastiary1/`. 300+ monsters distributed across CR bands; one cycle per monster-block subset (operator-pinned ordering). The `RuleSetId::Bestiary1` variant ships; at least one resolved monster-block cycle is consumable by Epic 6's encounter-math. Detailed acceptance criteria 14-17 in `epic-breakdown.md` §"Epic 5 — Bestiary 1 content-source ingest."

### 1.6 Epic 6 — DM Toolkit (encounter builder + party-CR math)

Consumes Epic 3 + Epic 4 + Epic 5 output. `src/rules_core/encounters.rs` lands with `Encounter::new(party, monsters) -> EncounterResult` (Easy / Medium / Hard / Deadly per PF1 rules); `src/rules_core/party_cr.rs` lands with `party_challenge_rating(party) -> f32`. DM-toolkit tests cover both modules' deterministic cases against canonical Paizo examples. The happy-path integration test consumes ingested content (PartySnapshot + MonsterRef → EncounterResult → canonical-table assertion). Detailed acceptance criteria 18-21 in `epic-breakdown.md` §"Epic 6 — DM Toolkit."

### 1.7 Epic 7 — Closure Epilogue (final scan + PR + worktree cleanup + release notes + version increment; fires LAST)

The standard part-of-handoff-doctrine for every spec-domain closure going forward (per `governance/spec-domain-lifecycle.md`): Epic 7 fires LAST in the SD-22 cycle-ordering. Its cycle scans every prior criterion (1-30) for `complete` or `## Open blockers` status; opens the `tranche/5 → develop` closure PR via `gh pr create`; cleans up worktrees and stale branches; generates release notes under `programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/release-notes.md`; runs the closure test suite as the final gate. The *generic* version-increment logic in Epic 7 reads the current `<major>.<tranche>.<build>` triple, increments only the **tranche** position by `1` (and resets build to `0`) on tranche promotion; the *specific* `0.5.<current_build>` value comes from Epic 8's first-cycle bump. Detailed acceptance criteria 22-26 in `epic-breakdown.md` §"Epic 7 — Closure Epilogue."

### 1.9 Epic 9 — Closure Readiness (evaluate + self-heal + dispatch; fires between Epic 8 and Epic 7)

The new epic (operator directive 2026-07-19; added 2026-07-19 mid-conversation). Epic 9's job is the final-acceptance gate that previous patterns bundled into the closure epic itself. Epic 9:

- evaluates every criterion 1-30 against the `artifacts/` evidence in `docs/release/SD-22/artifacts/`, cross-checking `progress.md` claims against artifact presence;
- if any shortfall is detected (a criterion marked `complete` *without* an artifact, a missing test fixture, a broken parity test), fires self-healing cycles to address each shortfall — self-healing is **open-ended** until the goal is met (per operator directive);
- dispatches Epic 7 by transitioning Epic 7's kanban card from `pending` to `ready`, only when every criterion 1-30 has artifact evidence;
- defers operator-judgment calls (state that *looks* suspicious but isn't a clean shortfall) to `risks-and-open-questions.md` §"Open judgments deferred to next SD" rather than self-healing them in-bundle (per operator directive 2026-07-19).

Epic 9's only criterion is **criterion-31**: a composite criterion requiring all four conditions above. The cycle log records every self-heal cycle's input-shortfall and output-state. The cycle log also records every operator-judgment call as a deferred item so the next bundle's audit can pick them up.

Detailed acceptance criterion 31 in `epic-breakdown.md` §"Epic 9 — Closure Readiness."

### 1.8 Epic 8 — Build Version Numbering (`<major>.<tranche-base>.<build>` + build-label format)

The display-build-version amendment applies symmetrically to SD-22. The version scheme is a three-position triple `<major>.<tranche-base>.<build>` (per operator directive 2026-07-17; replaces the prior `0.0.X` patch-only scheme):

- **`major`** (first number) is `0` until the first publish to `main`; increments by `1` per main-publish.
- **`tranche-base`** (second number) is the **base** of the active working tranche. SD-22's `tranche/5` carries `5`.
- **`build`** (third number) is a **monotonic counter across all builds across all branches — never resets**. Increments by `1` on every merge.

The first concrete SD-22 release value lands as **`0.5.<current_build>`** (e.g. `0.5.<next_build>` where `<next_build>` is the build counter value at SD-22 cycle launch). The build-label format is `Codex 0.5.<build>` (capitalized product name, space-separated; the `@` is dropped per the same amendment). Epic 8 cycles touch three files for the version field (`apps/desktop/package.json`, `apps/desktop/src-tauri/tauri.conf.json`, `apps/desktop/src-tauri/Cargo.toml`); `apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts` for the build-label format change; three test-fixture files; and a new `docs/SD-22/release-closure-checklist.md`. **Epic 8 fires BEFORE Epic 7** so the version commit is in Epic 7's closure PR's history. Detailed acceptance criteria 27-30 in `epic-breakdown.md` §"Epic 8 — Build Version Numbering."

## 2. Promotion gate

After all nine epics close AND `codex-tranche-5` board shows every acceptance criterion `complete` (or `## Open blockers` documented) AND Epic 9 has dispatched Epic 7, the loop opens a `tranche/5 → develop` promotion PR per the existing cadence. **Epic 7's PR is only fired when Epic 9 has dispatched it**; Epic 9's dispatch is gated by criterion-31 (a clean 30/30 eval). Epic 7's cycle IS the closure PR open — Epic 7 criterion 23 runs `gh pr create`. The PR's description references all 31 acceptance criteria (1-30 + the Epic 9 dispatch criterion-31), the cycle-merge receipt SHAs, the release notes preview, and the worktree/branch summary from Epic 7 criterion 24. The PR body includes audit-trail comments per the codex-tranche-2-5 respawn-guard pattern.

## 3. Cross-reference

- `./decisions.md` — 4-item decision record; required reading for understanding SD-22's shape. §1 documents the APG + ACG + Bestiary 1 + DM toolkit scope (per operator directives 2026-07-15 + 2026-07-17 + 2026-07-18); §2 documents the `tranche/5` + `codex-tranche-5` launch-branch decision (per operator directive 2026-07-18); §3 closes the operator-deferred shape decisions; §4 documents Epic 9 — Closure Readiness (per operator directive 2026-07-19).
- `./technical-design.md` — content-source ingest patterns + DM-toolkit architecture.
- `./epic-breakdown.md` — 31 acceptance criteria grouped into 9 epics.
- `./acceptance-and-verification.md` — closure gates (gates 1-13).
- `./risks-and-open-questions.md` — self-healable vs. non-self-healable split + open override flags.
- `./technical-requirements.md` — pre-loop prerequisites.
- `../../doctrine-external/spec-domain-lifecycle.md` — sibling lifecycle doctrine; governs Epic 7's closure flow.
- `../../doctrine-external/identifier-discipline.md` — sibling identifier-discipline doctrine; governs Epic 1's identifier-cleanup criteria.
- `~/.hermes/profiles/god-emporer/skills/devops/identifier-discipline/SKILL.md` — procedural skill for Epic 1 cycles; loaded by the SD-22 loop when Epic 1 fires.
- `../SD-19/` — corpus-aware seam + canonical Paizo-table store (CRB); SD-22's per-book ingest pattern inherits from SD-19 §9.
- `../SD-20/` — sibling bundle; per-character rules-engine surface that SD-22's content-source ingest feeds into.
- `../SD-21/` — sibling bundle; SD-21's Epic 2 (Campaign Manager + Drive) consumes the party-CR math that SD-22 will provide.
- `./loop-instruction.md` — loop body, the `/loop` invocation reads this.
