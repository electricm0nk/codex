# SD-27 — Risks and Open Questions

## 1. Self-healable conditions

| Condition | Self-heal |
|---|---|
| Working tree dirty | `git checkout -- <file>` or `git reset --hard HEAD~1` |
| Single identifier-audit leak | rename inline; re-audit; commit |
| Single wired-integration four-check failure | remove token; re-audit; commit |
| Cycle's tests fail for unrelated reason | fix the test setup |
| Build counter out of sync | re-read `apps/desktop/package.json`; update `decisions.md §4` |
| `## DISCOVERED` duplicates | merge duplicates; mark de-dup |
| `data/corpus/<book>/*.json` SHA-256 frontmatter doesn't match its source LST | recompute SHA; re-run normalize; re-audit |
| Per-book pre-build cycle hits a missing LST file for one of the 2 in-scope future-state books | Stop cycle; record blocker; operator routes (find the LST, defer the book, or skip the missing content kind) |
| Per-book parity fixture (`pf_<book>_human_<class>_level1_golden.pcg`) fails PCGen Gradle | Sanitize pipeline error; re-run; if deterministic, treat as 2nd-class failure for the cycle receipt |

## 2. Non-self-healable conditions (write to `## Open blockers`)

| Condition | Action |
|---|---|
| Working tree diverged from `tranche/6` needs manual rebase | `## Open blockers`; exit FAIL |
| Two live orchestrators on conflicting files | First wins; second writes `CLAIM-EXISTS`; exit FAIL |
| SD-26 closure PR not merged to develop | Loop refuses to start Epic 2.1+ (Tier-1 launch gate) |
| `## DISCOVERED` queue > 10 entries | Operator override required; pause |
| RED → GREEN not preserved in artifact | Cycle re-run with RED → GREEN captured |
| `success: true` from fake operation; inline mock in shipping module; "Would …" in shipping code | Cycle rejected; cannot mark `complete` |
| Per-book cycle's PCGen parity baseline produces a >2-dimension mismatch against the comparator's normalization library | sd-26's 7-of-9 baseline is the worst-case ceiling; >2 mismatches is a corpus-content quality issue, not a per-book cycle issue. Defer to `## Open blockers`; operator decides engine-side vs PCGen-side vs corpus-side |
| `data/corpus/<book>/LICENSE.json` redaction policy produces records that fail PCGen `comparator::compare` (e.g. a `[redacted PI]` marker in a comparison-dimension value) | Document in receipt; treat as inherited CG-03-style baseline; do not retro-fit the comparator |
| v0.6's class-skill / equipment-attachment / feat-effects work causes a discrete change to `data/corpus/<book>/` for any of the 4 in-scope books that conflicts with SD-27's per-book ingestion | The bundle's partition is the binding; v0.6's overlap is the trigger. Defer to `## Open blockers`; operator routes |
| A per-book cycle fails the dual-audit gate twice in a row (schema-level defect, not a per-book issue) | Stop all in-flight E2.x cycles; operator reviews the audit output across both attempts; likely fix is in `src/rules_core/shape_b_v1.rs`, not in the per-book `data/corpus/` |

## 3. Override flags

| Flag | Default | Set behavior |
|---|---|---|
| FLAG-A: STRICT-STOP-AT-DEADLINE | unset (grace-tail) | strict stop at operator's deadline |
| FLAG-B: BUDGET-MODE | unset (Sonnet default) | enable free / discounted model for per-book cycle bodies (operator-authorized per `decisions.md §11`); the dual-audit gate is the load-bearing enforcement, the model cannot skip it |
| FLAG-C: STRICT-CACHE-COVERAGE | unset | require 100% field coverage on per-book Shape B v1 records; relax only if audit shows the threshold cannot be met — **SD-26 already proved this relax-path is the common case for CRB (67.9% ceiling) and, to a lesser extent, APG (97.9%), not a rare exception; see SD-26 `decisions.md §11.4`** |
| FLAG-D: OGL-PI-LICENSE-STRICT-MODE | unset (redact-to-marker default) | require per-record `license: "OGL" \| "PI" \| "PI-REDACTED"` field on every Shape B v1 record; the 5th dual-audit (PI-blacklist grep) returns 0 defects. Default behavior matches SD-22's Shape B v0 + the 2026-07-25 OGL-licensing review directive |

## 4. Open questions

| Q | Question | Default |
|---|---|---|
| Q1 | E3 per-book ordering for the 2 in-scope future-state books | Alphabetical by book name: advanced_race_guide (E3.1) → pathfinder_unchained (E3.2). The two are file-disjoint, so ordering is a reporting convenience, not a constraint |
| Q2 | `book_stub` entries' `planned_resolution_bundle` (the label discrepancy) | `SD-27` (operator-pinned default); the discrepancy against `data/stubs/*.json`'s `SD-27+ (unscheduled)` is the bundle's first cycle (Criterion 2.0) — operator pulls either direction, lead propagates |
| Q3 | Oracle-harness comparator parity policy on partial-failure dimensions | First mismatch aborts the cycle; operator decides whether to fix Codex-side, fix PCGen-side, or relax the comparator's tolerance |
| Q4 | Pathfinder Unchained (PU) per-field completion ceiling — SD-25/SD-26's corpus-intake pass covered CRB/APG/ACG/Bestiary-1 but not PU | Unknown; E3.2 (PU parity baseline) should re-verify PU's real ceiling against the corpus directly (same method as SD-26 `decisions.md §11.4`'s table) before assuming any number. PU is also mechanically atypical — it is a variant-rules book, so its LST set (11 files) is smaller and less class-shaped than ARG's (23) |
| Q5 | Advanced Race Guide (ARG) per-field completion ceiling | Unknown; E3.1 (ARG parity baseline) should re-verify ARG's real ceiling against the corpus directly |
| Q6 | Shape B v1's PI-blacklist exhaustiveness — initial blacklist is per-field (deity, deity_name, npc, npc_name, monster_name (non-bestiary), place_name, faction_name, deity_portfolio, art_url, fiction_text, book_cover, monster_description (flavor)); the 2.0.10 dual-audit verifies the blacklist is exhaustive across all 23 books | If a per-record field is matched by the PI-pattern regex but not in the initial blacklist, add it to the blacklist with provenance (`versioned per cycle`); do not silently auto-rewrite |
| Q7 | Which per-book content kinds (classes, spells, equipment, feats, races, bestiary, archetypes, traits, domains) does each of the 2 in-scope future-state books (ARG, PU) actually have? | Templated; per-book inventory step in E2.x cycle 1 reads `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/<book>/` and records `content_kind_counts`. A book with no archetype LST does not get an `archetypes/` directory; the absence is honest, not a missed cycle |
| Q8 | Beginner Box + Core Essentials (removed from scope per operator directive 2026-07-27) — should their `data/stubs/beginner_box.json` and `data/stubs/core_essentials.json` (if they exist on disk) be physically deleted in the closure epilogue, or left as orphaned stubs for a future audit pass? | Document in closure-readiness-report.md; operator routes. The bundle's `decisions.md §9` + `scope-draft.md` carry the load-bearing "removed from scope" language; the on-disk stub files are out-of-scope and may be deleted by E4 with operator authorization |

## 5. Deferrals

- **17 deferred future-state books** (Bestiary 2-6, Bonus Bestiary, Horror Adventures, Monster Codex, Mythic Adventures, Occult Adventures, Pathfinder Unchained, the 6 Tier-2 Ultimate books). Deferred to SD-28+, operator-gated on SD-27 closing cleanly. Beginner Box and Core Essentials were removed from scope per operator directive 2026-07-27 and will NOT be brought in.
- **Rule-system implementations beyond Pf1Adapter** (D&D 5e, Pathfinder 2e, etc.). Per operator scope-cross posture, the trait exists in SD-25; concrete implementations land in SD-27+ (and only if a per-content-kind pre-build surfaces them).
- **Companion / animal / familiar stat-block engine.** Out of scope per `forward-scope-register.md §"Class 2.5"`. v0.6 documented as non-goal for alpha; SD-27 inherits.
- **Parameterized feats (Skill Focus, Teamwork feats).** Carved out until a general feat-effects engine exists. SD-27's per-book `feats/` directories are content-only, not engine-side wiring.
- **Multiclass durability-level ordering.** Shape B JSON cache does not encode level-ordering ambiguity. v0.6's lane.
- **Inline mocks / "Would …" strings outside bundle's file-touch.** Per `wired-integration-discipline` doctrine; any defect is a cycle-rejection event, not a deferred.

## 6. Latent risks

- **The 2 in-scope future-state books' LST corpus may not be on disk at the operator-expected path.** `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/<book>/` is the path-of-record; if the LST corpus is at a different location (e.g. a different operator workstation, a different mount), every per-book cycle blocks at the LST-inventory step. Mitigate: the bundle's loop-instruction.md §3.3 has a "dispatcher verifies directory exists" precondition; cycles block on `FileNotFoundError` and route to `## Open blockers`.
- **Bundle label discrepancy (Q2) is real and operator-gated.** If the operator doesn't pull at cycle 2.0, every subsequent cycle that touches a `data/stubs/*.json` file or a registry entry blocks. This is by design; the operator's lever pull at 2.0 is the binding decision.
- **The Shape B v1 schema bump (cycle 2.0.5) is load-bearing for every per-book cycle (E2.1-2.2).** If 2.0.5 doesn't land cleanly, the 2.0.6-2.0.9 in-scope retro-fit and the E2.1+ per-book pre-build both block. No partial-pre-build allowed.
- **CG-03 (Human ability-modifier bug at `src/rules_core/pilot_compute.rs:4743-4767`) is v0.6's lane, not SD-27's.** SD-27 inherits the 7-of-9 baseline. The per-book parity baseline assertion is "match rate at cycle close," not "9-of-9 oracle-checked." Document in each E3.x parity-cycle receipt.
- **v0.6 is actively working class/race breadth (Fighter/Wizard/Rogue + 8 remaining CRB classes).** SD-27's file-touch partition restricts SD-27 cycles to `data/corpus/<book>/` for the 19 future-state books and to `docs/governance/wired-integration-stubs-registry.md`. v0.6 may be modifying `src/rules_core/rules_tables/{crb,apg,acg,beastiary1}/` and `data/corpus/{core_rulebook,advanced_players_guide,advanced_class_guide,beastiary}/` in parallel (note the corpus tree spells it `beastiary`, the rules-engine tree `beastiary1`). The 4-grep dual-audit is the load-bearing enforcement; cycles that breach the partition fail the audit and return to the operator.
- **The bundle's pre-loop gate dependency on SD-26's `tranche/5-4 → develop` PR is a hard stop.** Per `decisions.md §7` + `loop-instruction.md §2.1`: SD-27 cannot dispatch Epic 2.1+ until SD-26's closure PR lands. If SD-26's closure is delayed, SD-27's launch slips correspondingly. This is not a "we'll fix it later" — it's a structural dependency.
- **E4 (closure epilogue) requires Tier-1 (E1.1) and Tier-2 (E1.1) both clean.** The bundle's per-cycle audit failures are *cycle* failures, not *epic* failures — every cycle must pass its dual-audit independently. E4.1's final-criterion scan cross-checks against 3 independent sources (cycle receipts, kanban board, status matrix); any disagreement blocks E4.5's PR+merge.