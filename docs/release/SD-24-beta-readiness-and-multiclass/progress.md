# SD-24 — Progress

> **Operating method:** see `./scope-draft.md` and `./loop-instruction.md`. This file is created on cycle 0 of Epic 2 with the deterministic seed of 35 criteria. Cycle picker reads `## TODO` + `## DISCOVERED` per the deterministic-seeded-then-dynamic model.

This file is the bundle's runtime state. The loop's `progress.md` is the canonical cycle-log + status matrix; the kanban board is the durable receipt; the per-cycle `artifacts/<epic>/<cycle>_cycle_receipt.md` is the per-cycle truth.

`## TODO` — the deterministic seed + dynamic entries, in dispatch-priority order. The picker reads this first.

`## DONE` — completed criteria, with commit SHA and cycle-id. Append-only.

`## DISCOVERED` — cycle-found items outside the deterministic list. Each entry has `<ISO-8601> | <epic-of-origin> | <criterion-of-origin> | <priority-bump-tag> | <description> | <suggested-epic-and-criterion>`. Priority-bump items go to the front of `## TODO` on operator call.

`## Status matrix` — per-criterion state in epic+criterion-number order (1.1, 1.2, 2.1, ..., 8.4). Update on every cycle's Step 9.

`## Cycle log` — append-only cycle entries (per `./loop-instruction.md §3` schema).

`## Open blockers` — non-self-healable items requiring operator intervention.

## Status matrix (placeholder — populated by cycle 0 of Epic 2)

| Criterion | State | Cycle ID | Commit SHA | Notes |
|---|---|---|---|---|
| 1.1 Source-code identifier audit | complete | identifier-audit-cycle | 3c3882a | Corrected-pattern audit found + remediated 6 sd16_/sd19_ modules, a resource dir, 13 SD19_* consts; see ## DISCOVERED |
| 1.2 Per-cycle tests pass | complete | identifier-audit-cycle | 3c3882a | cargo test --locked --tests (root) + cargo test --locked (src-tauri) both 0 failures |
| 2.1 board reachable | not-started | — | — | — |
| 2.2 branch pushed | not-started | — | — | — |
| 2.3 SD-23 closure PR merged | not-started | — | — | Tier-1 launch gate |
| 2.4 working tree clean | not-started | — | — | — |
| 2.5 doctrines loaded | not-started | — | — | — |
| 3.1 Wired-Integration Audit | not-started | — | — | — |
| 3.2 forbidden tokens remediation | not-started | — | — | — |
| 3.3 noop handlers remediation | not-started | — | — | — |
| 3.4 mock leaks + "Would …" remediation | not-started | — | — | — |
| 4.1 per-class audit CRB | not-started | — | — | — |
| 4.2 per-class audit APG | not-started | — | — | — |
| 4.3 per-class audit ACG | not-started | — | — | — |
| 4.4 remediation plan | not-started | — | — | — |
| 4.5 APG/ACG multiclass deferral | not-started | — | — | — |
| 5.1 F+W multiclass dispatch | not-started | — | — | gated on 4.x |
| 5.2 30 character-advancement cycles | not-started | — | — | gated on 5.1 |
| 5.3 integration test | not-started | — | — | gated on 5.2 |
| 5.4 multiclass four-check audit | not-started | — | — | gated on 5.3 |
| 5.5 APG/ACG multiclass deferred | not-started | — | — | — |
| 6.1 Equipment coverage audit | not-started | — | — | — |
| 6.2 Equipment content completion: cost | not-started | — | — | gated on 6.1 |
| 6.3 Equipment content completion: weight | not-started | — | — | gated on 6.1 |
| 6.4 Equipment content completion: description | not-started | — | — | gated on 6.1 |
| 6.5 Spell content completion: full text | not-started | — | — | gated on 6.1 |
| 7.1 appendToCharacter Tauri command | not-started | — | — | gated on 5/6 |
| 7.2 recomputeCharacter Tauri command | not-started | — | — | — |
| 7.3 reSaveCharacter Tauri command | not-started | — | — | — |
| 7.4 Add Weapon/Armor/Spell onClick | not-started | — | — | — |
| 7.5 loadout hardcoding removed | not-started | — | — | — |
| 8.1 Final criterion scan | not-started | — | — | fires LAST |
| 8.2 Architecture closure pipeline | not-started | — | — | fires LAST |
| 8.3 Release notes | not-started | — | — | fires LAST |
| 8.4 Build version increment | not-started | — | — | fires LAST |

## TODO (deterministic seed; populated by cycle 0 of Epic 2)

- 2.1, 2.2, 2.3, 2.4, 2.5 (Epic 2, gating epic)
- 3.1, 3.2, 3.3, 3.4 (Epic 3, audit + remediation)
- 4.1, 4.2, 4.3, 4.4, 4.5 (Epic 4, per-class audit)
- 5.1, 5.2, 5.3, 5.4, 5.5 (Epic 5, multiclass F+W)
- 6.1, 6.2, 6.3, 6.4, 6.5 (Epic 6, equipment 100%)
- 7.1, 7.2, 7.3, 7.4, 7.5 (Epic 7, unwired workflows + Tauri surface)
- 8.1, 8.2, 8.3, 8.4 (Epic 8, closure epilogue)

## DONE

- 1.1 Source-code identifier audit — cycle `identifier-audit-cycle`, commit `3c3882a`, receipt `./artifacts/epic_1/identifier-audit-cycle_receipt.md`
- 1.2 Per-cycle tests pass — cycle `identifier-audit-cycle`, commit `3c3882a`, receipt `./artifacts/epic_1/identifier-audit-cycle_receipt.md`

## DISCOVERED

- `2026-07-21T00:00:00Z` | epic-1 | criterion-1.1 | governance-pattern-bug | The canonical identifier-discipline grep in `epic-breakdown.md`/`loop-instruction.md §2.3 step 4` (`` \b(sd(16|19|22|23|24)_|SD(16|19|22|23|24)_|Sd(16|19|22|23|24)|t_[0-9a-f]{8,})\b `` ) has a trailing-`\b` boundary bug: `\b` never matches between an underscore and a following word character, so the pattern silently fails to catch any bundle-tag identifier that is followed by more identifier characters (e.g. `sd19_class_catalog`) and only matches a bare standalone token like `sd24_` on its own. Live proof: the literal command returned 0 hits against the pre-remediation repo while 6 production Rust modules, a bundled Tauri resource directory, and 13 constants were genuinely bundle-tag-prefixed. A corrected pattern (drop the trailing `\b`) is now load-bearing in `tests/sd24_identifier_discipline_audit.rs`. | suggested: harden `governance/identifier-discipline.md`'s canonical pattern (or `epic-breakdown.md`/`loop-instruction.md`'s embedded command) in a follow-up governance-doc cycle — out of this cycle's granted write scope.

## Cycle log

- **cycle_id:** identifier-audit-cycle (2026-07-21T00:00:00Z)
  **criterion:** 1.1 + 1.2 (Epic 1)
  **summary:** Ran the repo-wide identifier-discipline scan per criterion 1.1's own verbatim command; discovered the command's trailing-\b regex has a boundary bug that produces a false-clean 0-hit result. Ran a corrected (boundary-fixed) pattern instead, which found real bundle-tag leaks: 6 production Rust modules (`sd16_browser_handoff`, `sd19_class_catalog`, `sd19_corpus`, `sd19_equipment_catalog`, `sd19_race_catalog`, `sd19_spell_catalog`), a bundled Tauri resource directory (`resources/sd19_corpus_fixtures/`), and 13 `SD19_*_TEST` constants in `support_state_matrix.rs`. Remediated all of them (renamed files/modules/consts/resource dir, updated every call site, doc-comment citation, and `tauri.conf.json` path). Added `tests/sd24_identifier_discipline_audit.rs` as a standing regression test using the corrected pattern. RED→GREEN demonstrated live via `git stash`: stashed the remediation, confirmed the new test fails listing the real pre-remediation leaks, restored the remediation, confirmed the test (and the full `cargo test --locked --tests` root suite, and `cargo test --locked` in `apps/desktop/src-tauri`) all pass with 0 failures. Both dual-audit gates (`identifier-discipline`, `wired-integration-discipline`) are clean on the final diff.
  **commit:** `3c3882a`
  **receipt:** `./artifacts/epic_1/identifier-audit-cycle_receipt.md`
  **discovered:** 1 entry (governance-pattern-bug, see `## DISCOVERED`)
  **next:** Epic 2 (Operator Pre-Launch, criteria 2.1–2.5) is next in the deterministic seed.

## Open blockers

(empty — populated by non-self-healable failures)

---

*Per `./loop-instruction.md §2.3 step 9`: the cycle picker updates this file in place on every cycle. Do not rewrite from scratch; each cycle appends to its own section.*
