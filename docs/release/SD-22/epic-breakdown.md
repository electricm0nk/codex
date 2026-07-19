---
canonical: true
owner: god-emporer
status: approved (operator review 2026-07-15 scope; operator directives 2026-07-17 expanded scope to APG + ACG; operator clarification 2026-07-18: ACG + APG are "the two advanced guides"; branch + board pinned 2026-07-18 to tranche/5 / codex-tranche-5; 9 epics / 31-criteria final shape (Epic 9 added 2026-07-19; bundle marked planning-ready)
date: 2026-07-15
canonical_branch: tranche/5 (operator directive 2026-07-18)
kanban_board: codex-tranche-5 (operator directive 2026-07-18)
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/decisions.md
mirror_of: /home/ubuntu/workspace/SD-22-content-source-ingest-and-dm-toolkit-scope-draft.md
---

# SD-22 — Epic Breakdown

Maps the **31 acceptance criteria** for SD-22 (Code-Side Identifier Cleanup + Operator Pre-Launch + APG content-source ingest + ACG content-source ingest + Bestiary 1 content-source ingest + DM Toolkit + Build Version Numbering + Closure Readiness + Closure Epilogue) into **9 epics** inside the SD-22 bundle. Each epic has its own acceptance criteria; each epic lands via the same loop-routed-cycle pattern SD-21 used.

The 9 epics follow SD-21's 7-epic layout (Code-Side Identifier Cleanup at Epic 1, governance epics first, content-source epics in the middle, closure epilogue at the end). The order mirrors SD-21's structure so an operator reading both bundles sees the same operator pattern: governance first, content next, build-version numbering, closure-readiness gate, then closure last. **Epic 9 (Closure Readiness) is the new entrant added 2026-07-19** to decouple the eval-and-self-heal step from Epic 7's actual PR and release-notes work.

## Execution lane split

- Epic 1: Code-Side Identifier Cleanup        (governance base requirement; fires FIRST on shared files)
- Epic 2: Operator Pre-Launch                   (board-exists, branch-pushed, OAuth-credentials — gating epic; SD-22's novel piece)
- Epic 3: APG content-source ingest           (per-class cycles; one cycle per APG class table)
- Epic 4: ACG content-source ingest           (per-class cycles; one cycle per ACG class table)
- Epic 5: Bestiary 1 content-source ingest     (per-monster-block cycles)
- Epic 6: DM Toolkit                            (encounter builder + party-CR math; consumes Epic 3+4+5 outputs)
- Epic 7: Closure Epilogue                    (final scan + PR + worktree cleanup + release notes + version increment — fires LAST, only after Epic 9 dispatches)
- Epic 8: Build Version Numbering             (three-position `<major>.<tranche-base>.<build>` scheme — operator-pinned 2026-07-17; fires before Epic 9's eval)
- Epic 9: Closure Readiness                   (eval all 30 prior criteria against artifact evidence; self-heal shortfalls; dispatch Epic 7 — fires between Epic 8 and Epic 7)

Total: **31 acceptance criteria grouped into 9 epics + 2 promotion gates** (gate 1 is prerequisite; gate 9 is promotion). Epic 1 (Code-Side Identifier Cleanup) is listed first in the cycle-priority order below because Epic 3's content-source ingest work depends on Epic 1 having removed `sd22_*` (and the like) identifiers from source. Epic 1 does not interfere with Epic 3's data files (the source-book data files are populated separately from the source-code identifier renames). Epic 9 (Closure Readiness) gates Epic 7 (Closure Epilogue); the loop's cycle picker refuses Epic 7's eligibility until Epic 9's criterion-31 is `complete`. Epic 2 (Operator Pre-Launch) is SD-22's novel gating epic — it has no SD-21 equivalent because SD-21 inherited `codex-tranche-5` dead-state from the prior launch; SD-22 doesn't have that inheritance, so it needs explicit board-exists / branch-pushed / credentials-pinned pre-flight before the loop can fire its first cycle.

## Linear dependency (per decisions.md)

```
Epic 1 — Code-Side Identifier Cleanup — fires FIRST on shared files
└── Epic 2 — Operator Pre-Launch — board-exists, branch-pushed, OAuth-credentials-pinned
└── Epic 3 — APG content-source ingest — one cycle per APG class table
└── Epic 4 — ACG content-source ingest — one cycle per ACG class table (parallel with Epic 3 if operator hosts two streams)
└── Epic 5 — Bestiary 1 content-source ingest — one cycle per monster-block subset
└── Epic 6 — DM Toolkit — encounter builder + party-CR math (consumes Epic 3+4+5 outputs)
└── Epic 8 — Build Version Numbering — fires before Epic 9's eval so the version commit is in Epic 7's closure PR's history
└── Epic 9 — Closure Readiness — evaluate every criterion 1-30 against artifact evidence, self-heal shortfalls, dispatch Epic 7 only when 30/30 clean
└── Epic 7 — Closure Epilogue — depends on Epics 1-6 + Epic 8 + Epic 9 all complete
```

Epic 1 lands **first**. Epic 2's pre-launch checklist validates the launch infrastructure before any cycle runs. Epic 3 + Epic 4 are interleavable in cycle order (the file-touch partition permits concurrent cycles on disjoint source-book directories). Epic 5 is interleavable with Epic 3 + Epic 4. Epic 6 (DM Toolkit) requires at least one book ingested to have encounter data to consume. Epic 8 fires before Epic 9 so the version commit is in Epic 7's closure PR's history. **Epic 9 fires between Epic 8 and Epic 7**: Epic 9 evaluates every prior criterion 1-30 + Epic 8's own outputs, self-heals any shortfall (the self-heal is open-ended until 30/30 is clean), then dispatches Epic 7 by opening the Epic 7 cycle on the kanban board. Epic 7 fires LAST and is now narrowly scoped to: open the PR, generate release notes, run closure test suite, increment the version on tranche promotion.

## Acceptance criteria (31, across 9 epics)

### Epic 1 — Code-Side Identifier Cleanup (governance base requirement; fires FIRST)

**Scope doctrine (operational rule):** under the identifier-discipline doctrine (`../../doctrine-external/identifier-discipline.md`), source-code identifiers must describe what the artifact does, not which release or spec domain it came from. Epic 1 is the SD-22 cycle that fires to clean up the load-bearing identifier leaks already in the codebase: Tauri command names with the `sd22_` prefix (defensive; since SD-22 doesn't ship any new Tauri commands but the codebase may have remnants from earlier sessions that should not propagate), TypeScript functions and constants with `Sd22` / `SD22_` text, `data-testid` attributes with `sd22-` prefixes, inline doc-comments citing `SD-22-Ex...` identifiers, and any `t_<hex>` kanban tokens / `AV-PAY-N` audit-IDs embedded in source.

**Out of scope** for Epic 1 (recorded explicitly to prevent scope creep): directory tree renames. Those are Epic 7 follow-on work because directory rename churns every relative import, every release-channel JSON, and every electron-vite config.

1. **Source-code identifier audit runs** before Epic 3 lands: `grep -rE "sd22_|SD22_|Sd22|Sd22|sd[0-9]+_|SD-[0-9]+-[A-Z][0-9]|Tranche [0-9]+ chassis lane|AV-PAY-[0-9]+|t_[a-f0-9]{8,}" apps/desktop/ apps/desktop/src-tauri/ src/rules_core/ 2>&1 | head -50` reports zero hits in identifier-or-string-literal positions, with the exception of docstring prose references in the SD-22 bundle's own decisions.md / epic-breakdown.md (which are correct doctrine-of-record references).

2. **Per-cycle tests pass after every rename**: each renamed identifier gets a follow-up test cycle that exercises the new name and asserts the new behavior. CI runs green on `tranche/5` after each rename; no regression on existing functionality.

### Epic 2 — Operator Pre-Launch (board-exists, branch-pushed, OAuth-credentials-pinned)

**Scope doctrine (operational rule):** SD-22's loop can't fire its first cycle until the launch infrastructure is operator-validated. Epic 2's three criteria are operator-only pre-flight checks that don't require cycle work; they're a yes/no checklist.

3. **`codex-tranche-5` kanban board is operator-pinned as the SD-22 default** (per operator directive 2026-07-18). The loop's Step 10 mint uses `--board codex-tranche-5` explicitly.

4. **`tranche/5` branch is pushed to origin** before the loop's Step 3 fetch succeeds. Operator runs `git push origin tranche/5` once after the SD-22 launch-branch decision is recorded.

5. **No `claude` processes are in-flight on `tranche/5`** at cycle-1 launch. Verification: `ps -eo pid,etime,stat,cmd | grep claude` returns zero hits.

### Epic 3 — APG content-source ingest (per-class cycles)

**Scope doctrine (operational rule):** under SD-19 §9 source-book subdirectory pattern, APG populates `src/rules_core/rules_tables/apg/` as sibling directories to SD-19's `rules_tables/crb/`. The APG cycle's per-cycle landing shape is one APG class's table entries (per-cycle shape mirrors SD-19's per-school loop pattern). Each class table includes structured data for that class's features, spells (if any), equipment (if any), and any class-specific race interactions.

6. **`src/rules_core/rules_tables/apg/mod.rs` is populated** with the APG class table index and `RuleSetId::Apg` variant registration. The CRB-side `equipment_id_resolve` and `spell_id_resolve` resolvers accept `RuleSetId::Apg` as a parameter.

7. **Per-cycle tests for APG class tables** assert every level row resolves via `RuleSetId::Apg`. The test pattern follows SD-19's: parse each `apg/class_<class>.rs` via the SD-19 parser surface and assert the resolver returns the expected record. One cycle per APG class.

8. **Cross-book resolution tests for APG** assert that APG-only items return `Some` for `RuleSetId::Apg` queries but `None` for `RuleSetId::Crb` queries, and vice-versa. Per the SD-21 §12 doctrine (cross-book fallback at the resolver layer), the priority order is **APG → CRB → ACG**.

9. **Per-cycle tests for APG spell and equipment resolution** assert representative samples resolve to the expected table cell. APG content includes classes (Alchemist, Cavalier, Gunslinger, Inquisitor, Magus, Oracle, Summoner, Witch), spells (Alchemist extracts, Cavalier challenges, etc.), and equipment (Alchemist bombs, etc.).

### Epic 4 — ACG content-source ingest (per-class cycles)

**Scope doctrine (operational rule):** same shape as Epic 3, for the ACG content. Per-class cycle shape; populates `rules_tables/acg/` as sibling directory to `rules_tables/apg/`.

10. **`src/rules_core/rules_tables/acg/mod.rs` is populated** with the ACG class table index and `RuleSetId::Acg` variant registration.

11. **Per-cycle tests for ACG class tables** mirror Epic 3's per-cycle shape with `RuleSetId::Acg` queries. ACG content includes classes (Alchemist, Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Swashbuckler, Warpriest, plus Alchemist's APG side), spells (Arcanist exploits, Shaman spirits, etc.), and equipment (Shaman's spirit-animal items, Warpriest's sacred armor, etc.).

12. **Cross-book resolution tests for ACG** add ACG-vs-CRB and ACG-vs-APG resolution assertions to the existing cross-book test fixture.

13. **Per-cycle tests for ACG spell and equipment resolution** mirror Epic 3 with `RuleSetId::Acg`. The cross-book test asserts that spells and equipment unique to ACG (e.g. Arcanist exploits, Investigator inspiration, Shaman spirit magic) resolve correctly through the ACG code path.

### Epic 5 — Bestiary 1 content-source ingest (per-monster-block cycles)

**Scope doctrine (operational rule):** Bestiary 1 populates `src/rules_core/rules_tables/beastiary1/` per SD-19 §9. Per-monster-block cycle shape (one cycle per monster-block subset of the 300+ Bestiary 1 monsters).

14. **`src/rules_core/rules_tables/beastiary1/mod.rs` is populated** with the Bestiary 1 monster index and `RuleSetId::Bestiary1` variant registration.

15. **Per-cycle tests for Bestiary 1 monster stats** assert the resolved monster data has the expected CR, ability scores, and stat block shape. The test pattern follows SD-19's: parse each `beastiary1/monster_<subset>.rs` and assert the resolver returns the expected record.

16. **Cross-book resolution tests for Bestiary 1** assert Bestiary 1 monsters return `Some` for `RuleSetId::Bestiary1` queries but `None` for `RuleSetId::Crb` queries.

17. **DM-toolkit consumption**: at least one Bestiary 1 monster-block cycle's resolved data is consumable by the DM toolkit (Epic 6). Verification: `encounters.rs` (Epic 6 surface) reads at least one `beastiary1/monster_<subset>.rs` and produces a valid encounter from it.

### Epic 6 — DM Toolkit (encounter builder + party-CR math)

**Scope doctrine (operational rule):** the DM toolkit consumes content-source ingest output (Epics 3+4+5) and produces the encounter-math + party-CR surface that SD-21's Epic 2 (Campaign Manager + Drive) campaign-shape boundary contract consumes. SD-22 is downstream of nothing within the content-source lane but is the producer of the encounter/party-CR math surface.

18. **`src/rules_core/encounters.rs` lands** with the encounter-math core: `Encounter::new(party: &[CharacterSnapshot], monsters: &[MonsterRef]) -> EncounterResult` computes the encounter's difficulty rating (Easy / Medium / Hard / Deadly per PF1's "Encounter Building" rules).

19. **`src/rules_core/party_cr.rs` lands** with the party-CR computation: `party_challenge_rating(party: &[CharacterSnapshot]) -> f32` computes the party's CR (per PF1's "Determining Party Strength" rules).

20. **DM-toolkit tests** cover both modules' deterministic cases against canonical Paizo examples (e.g. "4 level-3 PCs vs. 1 CR-2 monster = Easy encounter"; "party of 4 level-3 PCs has CR ~3.5").

21. **DM-toolkit consumes ingested content** in a happy-path integration test: a campaign-shaped fixture (PartySnapshot) + a monster-block fixture (MonsterRef) → EncounterResult → assertion against the canonical Paizo encounter-table result.

### Epic 7 — Closure Epilogue (final scan + PR + worktree cleanup + release notes + version increment)

**Scope doctrine (operational rule):** per `governance/spec-domain-lifecycle.md`, every closed bundle gets a final-cycle epilogue that scans all acceptance criteria, opens the develop-merge PR, cleans up worktrees and stale branches, generates release notes, and increments the version number. SD-22's Epic 7 is the second worked example after SD-21's Epic 4. SD-22's `tranche/5 → develop` promotion PR is what closure runs.

22. **Final criterion scan**: walks the SD-22 progress matrix and asserts every criterion (1-30) is `Status: complete` OR has an `## Open blockers` entry.

23. **Open the closure PR**: opens the `tranche/5 → develop` promotion PR via `gh pr create`. The PR's description references all 31 acceptance criteria (criteria 1-30 + Epic 9's criterion-31 dispatch), cycle-receipt SHAs, and a one-line summary of which epics landed which criteria.

24. **Worktree cleanup and stale-branch sweep**: removes worktrees whose branch is not `tranche/5`, `develop`, or `main`; deletes stale branches merged into `tranche/5` or `develop` more than 30 days ago.

25. **Generate release notes**: produces `programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/release-notes.md` with sections: "New content" (from Epics 3+4+5), "DM toolkit" (from Epic 6), "Maintenance" (from Epic 1), "Versioning" (from Epic 8).

26. **Increment the version** *(tranche promotion only)*: reads the current `<major>.<tranche-base>.<build>` triple from the three version files (Epic 8's locks the values), increments only the **tranche** position by `1` and resets build to `0` on tranche promotion (`0.5.<last_build>` → `1.0.0` if SD-22's release is the first main-publish; or `0.5.<last_build>` → `0.6.0` if there's a Tranche-6 follow-on). The *concrete* `0.5.<current_build>` value lands in **Epic 8** (Build Version Numbering) as a separate, focused cycle.

### Epic 8 — Build Version Numbering (`<major>.<tranche-base>.<build>` + build-label format)

**Scope doctrine (operational rule):** the displayed build version follows the same three-position scheme as SD-21's Epic 5 (per operator's 2026-07-17 amendment): **`<major>.<tranche-base>.<build>`** with `major = 0` until first main-publish, `tranche-base = 5` for `tranche/5` (the active branch), and `build` is a monotonic counter across all builds across all branches — never resets. SD-22's first concrete release value lands as `0.5.<current_build>` (e.g. `0.5.<next_build>` where `<next_build>` is the build counter value at SD-22 cycle launch).

27. **Version fields set to `0.5.<current_build>`**: `apps/desktop/package.json`'s `"version"` field, `apps/desktop/src-tauri/tauri.conf.json`'s `"version"` field, and `apps/desktop/src-tauri/Cargo.toml`'s `version =` line are all set to `"0.5.<current_build>"`. Cargo.lock's embedded copy of the version updates automatically on the next `cargo check`.

28. **Build-label format updated**: `apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts:61` sets `BUILD_PREFIX = 'Codex'`; `createSd11WorkbenchStatus.ts:72-74` uses template `${BUILD_PREFIX} ${buildVersion}` (space-separated display). The format renders the `<major>.<tranche>.<build>` triple from the version files. Test fixtures update to assert/fixture the new `Codex 0.5.<build>` shape.

29. **Bump process established**: `docs/SD-22/release-closure-checklist.md` records the four-step process for future closure epilogues (mirroring SD-21's Epic 5 criterion 27). Per-position increment rules: build per-CI-build, tranche per-tranche-promotion, major per-main-publish.

30. **Per-cycle tests pass at closure**: full `cargo test --locked` (zero regressions), `cargo clippy --locked --tests -- -D warnings` (clean), and the SD-22 acceptance gate suite (all 31 criteria at `complete`; Epic 9's criterion-31 confirms the eval cycle's dispatch decision).

**Out of scope for Epic 8 (recorded explicitly):**

- Major-version or `major != 0` first-publish logic. SD-22 ships with `major = 0` until first main-publish.
- Automated build-counter increment as part of every CI commit. Per-CI-build increment is operator-pinned at cycle launch; automation is a future bundle's epic.
- Build-label parsing anywhere in the codebase. The format is presentation-only.

### Epic 9 — Closure Readiness (evaluate + self-heal + dispatch Epic 7)

**Scope doctrine (operational rule):** Epic 9 is the *gatekeeper* between "everything that was supposed to be built" and "the closure PR that ships it." The previous pattern (Epic 7 = Closure Epilogue doing both eval and PR) conflates two jobs that have different failure modes: a missed artifact surfaces in the eval lane where it can be self-healed; a missed artifact surfaces after the PR is open and the bundle is in the dispatcher's `merging` state — too late. Epic 9 separates the two so the eval is cheap to re-run and the PR is opened only when there's nothing left to self-heal.

**How Epic 9 fires.** Epic 9 fires once, after Epic 8 lands and before Epic 7 starts. Epic 9 is a multi-cycle epic by structure: it runs cycles until 30/30 criteria are clean, then dispatches Epic 7. The dispatcher doesn't gate Epic 9's "first cycle" by anything more than "Epic 8 criterion-30 is `complete`"; the dispatcher also doesn't gate Epic 9's "later cycles" — Epic 9's own cycle picker decides whether the next cycle is "another self-heal pass" or "dispatch Epic 7."

31. **Composite criterion (Epic 9's only criterion)**: every criterion 1-30 has artifact evidence in `docs/release/SD-22/artifacts/` cross-referenced from `progress.md`'s status matrix; `progress.md` claims match the artifact-evidence survey (a criterion marked `complete` *without* an artifact is a shortfall, not met); if any shortfall existed at any point in Epic 9's run, the self-healing cycle(s) that addressed it are recorded in the cycle log; when 30/30 are clean, Epic 9 dispatches Epic 7 by transitioning Epic 7's kanban card from `pending` to `ready` (the loop's normal cycle-pickup path then handles Epic 7 normally).

**Self-healing rule.** Self-healing is **open-ended** until the goal is met (per operator directive 2026-07-19). Each self-heal cycle is a normal cycle that lands one shortfall: write the missing artifact, fix the missing test, repair a broken parity fixture, etc. The cycle log records what was self-healed and why it was short. There is no per-cycle cap and no per-Epic-9 cap on self-heal cycles; the loop runs until the goal is met.

**Operator-judgment-call rule.** When Epic 9's evaluator encounters a state that looks suspicious but isn't a clean shortfall (e.g., "a rule-table entry looks wrong but a unit test passes"), Epic 9 does **not** self-heal it. Epic 9 logs the judgment to `docs/release/SD-22/risks-and-open-questions.md` §"Open judgments deferred to next SD" and continues. Remediation lives in the next bundle's audit (per operator directive 2026-07-19: judgment calls are deferred, not remediated in-bundle).

**What Epic 9 is NOT (recorded explicitly):**

- Is not a rewrite epic. Epic 9 doesn't change source-code behavior; it produces one artifact (`closure-readiness-report.md`) and dispatches Epic 7.
- Does not run the closure test suite. That's Epic 7's pre-PR gate.
- Does not open the `tranche/5 → develop` PR. That's Epic 7's `gh pr create`.
- Does not generate release notes. That's Epic 7.
- Does not increment the version on tranche promotion (`0.5.<last_build>` → `0.6.0`). That's Epic 7.
- Does not include Epic 8's outputs in the eval-cycle-count. Epic 8's outputs are reviewed by Epic 9's first cycle but Epic 8 isn't itself a 1-30 criterion; it's a precondition.

**Numerics.** Epic 9 carries 1 criterion (criterion-31). Total criteria across the bundle: 31. Total epics: 9.

## Cycle ordering (operator-prioritized)

The operator can prioritize per the dependency graph. Default ordering:

1. Epic 1 — Code-Side Identifier Cleanup (governance base requirement; must land before any other epic touches source)
2. Epic 2 — Operator Pre-Launch (gating; verifies board + branch + clean state before loop fires)
3. Epic 3 — APG content-source ingest (per-class; one cycle per APG class table)
4. Epic 4 — ACG content-source ingest (per-class; one cycle per ACG class table)
5. Epic 5 — Bestiary 1 content-source ingest (per-monster-block; one cycle per monster-block subset)
6. Epic 6 — DM Toolkit (encounter builder + party-CR math; consumes Epic 3+4+5 outputs)
7. Epic 8 — Build Version Numbering (after Epics 1-3+4+5+6 land; before Epic 9's eval so the version commit is in Epic 7's closure PR's history)
8. Epic 9 — Closure Readiness (eval all 30 prior criteria against artifact evidence; self-heal shortfalls; dispatch Epic 7 only when 30/30 clean)
9. Epic 7 — Closure Epilogue (fires LAST; only when Epic 9 has dispatched; opens the `tranche/5 → develop` PR)

## Cycle unit definition

A single loop cycle within an epic lands one acceptance criterion (or one representative sample for that criterion). Each cycle:

1. Picks one acceptance criterion from the epic's open list.
2. Verifies the working tree is on `tranche/5` (no feature branches; per the no-branches convention; per operator directive 2026-07-18: SD-22 launches on `tranche/5`, not `tranche/3` or `tranche/4`).
3. Reads the cycle's parity test fixture (for content-source ingest cycles) or the boundary-contract test (for DM-toolkit cycles).
4. Implements the smallest change that satisfies the criterion.
5. Runs `cargo test --locked` (zero regressions) and `cargo clippy --locked --tests -- -D warnings` (clean).
6. Commits directly to `tranche/5` with a `feat(sd22): <criterion> (<row transition>)` message.
7. Mints a kanban card on `codex-tranche-5` as a post-mortem record (`status=done`, with merge receipt, audit-trail comment per codex-tranche-2-5 respawn-guard pattern).
8. Updates the shared progress doc's `## SD-22 cycles` section.
9. Exits.

A cycle is a *unit of post-mortem*, not a unit of delivered scope. One cycle, one criterion, one card, one commit.

## What the breakdown does not specify

- Per-cycle implementation approach — the loop picks the smallest change that satisfies the criterion.
- Per-cycle timing — depends on content-volume, parser friction, behavioral complexity; the loop's self-healing handles friction.
- APG 9-class ordering — operator-pinned at SD-22 cycle launch (default-and-flag: alphabetical-by-class-name).
- ACG class ordering — operator-pinned at SD-22 cycle launch (default-and-flag: alphabetical-by-class-name).
- Bestiary 1 monster-block ordering — operator-pinned at SD-22 cycle launch (default-and-flag: alphabetical-by-monster-name within CR band).
- Whether the DM-toolkit GUI surface is shipped in this bundle — **NOT** in this bundle; that's a future `SD-23`.
- Whether the Ultimate-line books (Ultimate Combat, Ultimate Magic) are added later — **NOT** in this bundle; that's a future operator-pinned directive.
- First-main-publish — **NOT** in this bundle; that's a release-process decision.

## Cross-reference

- `decisions.md` — the 4-item decision record (§1 scope, §2 tranche/5 + codex-tranche-5, §3 deferred shape decisions, §4 Epic 9 — Closure Readiness added 2026-07-19).
- `acceptance-and-verification.md` — closure gates (gates 1-13).
- `risks-and-open-questions.md` — self-healable vs. non-self-healable split + open override flags.
- `technical-design.md` — content-source ingest patterns + DM-toolkit architecture.
- `technical-requirements.md` — pre-loop prerequisites.
- `./scope-draft.md` — canonical handoff; carries the prominent-early `/loop /batch /goal` OPERATING METHOD callout.
- `./loop-instruction.md` — loop body.
- `../SD-19/` — sibling bundle; the Tranche-3 corpus-source ingest pattern SD-22 inherits from.
- `../SD-20/` — sibling bundle; per-character rules-engine surface that SD-22's content-source ingest feeds into.
- `../SD-21/` — sibling bundle; SD-21's Epic 2 (Campaign Manager + Drive) consumes the party-CR math that SD-22 will provide.
