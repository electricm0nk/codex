# SD-25 — Progress

> **Operating method:** see `./scope-draft.md` and `scripts/workflow-dispatch.sh`. This file is created on cycle 0 of Epic 2 with the deterministic seed. The orchestrator reads `## TODO` + `## DISCOVERED` and dispatches the highest-priority unclaimed item.

This file is the bundle's runtime state. The orchestrator's `progress.md` is the canonical cycle-log + status matrix; the kanban board is the durable receipt; the per-cycle `artifacts/<epic>/<cycle>_cycle_receipt.md` is the per-cycle truth.

## Status matrix (placeholder; populated by cycle 0 of Epic 2)

| Criterion | State | Cycle ID | Commit SHA | Notes |
|---|---|---|---|---|
| 1.1 Source-code identifier audit | complete | 1.1 | `62c4785d098c3c288093f5076130ec44efdce23f` | 404 residual hits, all documented exclusion (real tests/-file citations); see artifacts/epic_1/identifier-audit-cycle_receipt.md |
| 2.1 board reachable | complete | 2.1 | verification-only, no code commit | `hermes kanban boards` confirms `codex-tranche-5` reachable; see artifacts/epic_2/board-reachable-cycle_receipt.md |
| 2.2 branch pushed | complete | 2.2 | verification-only, no code commit | local HEAD matches `origin/tranche/5-3` tip; see artifacts/epic_2/branch-pushed-cycle_receipt.md |
| 2.3 SD-24 closure PR merged | complete | 2.3 | verification-only, no code commit | Tier-1 launch gate; PR #331 `state: MERGED`; see artifacts/epic_2/sd24-pr-merged-cycle_receipt.md |
| 2.4 working tree clean | complete | 2.4 | verification-only, no code commit | remediated (leftover 1.1-adjacent doc edits + Epic 2 receipts batch-committed); see artifacts/epic_2/tree-clean-cycle_receipt.md |
| 2.5 doctrines loaded | complete | 2.5 | verification-only, no code commit | all required doctrine/skill surfaces present and readable; see artifacts/epic_2/doctrines-loaded-cycle_receipt.md |
| 3.1 RuleSystemAdapter trait | complete | 3.1 | `490332d` | trait + inline dyn-dispatch test wired to real functions; see artifacts/epic_3/rule-system-adapter-trait-cycle_receipt.md |
| 3.2 Pf1Adapter extraction | complete | pf1-adapter-extraction | `dccfcb87673e2980b09fc9ba50f7bf68b7945f19` | Pf1Adapter extracted from character_hub.rs; register A5 (revision_id fold-in) + A2 (multiclass level-up dispatch) resolved; impl RuleSystemAdapter for Pf1Adapter completed after 3.1 landed mid-cycle; see artifacts/epic_3/pf1-adapter-extraction-cycle_receipt.md |
| 3.3 StubAdapter | complete | 3.3 | `c41aedc` | StubAdapter implements full RuleSystemAdapter surface; wired-integration-stubs-registry.md entry 0002 landed same commit; see artifacts/epic_3/stub-adapter-cycle_receipt.md |
| 3.4 Tauri command routing | complete | 3.4 | `49097b4` | appendToCharacter/recomputeCharacter/reSaveCharacter accept rule_system_id and dispatch through RuleSystemAdapter (Pf1Adapter for "pf1", StubAdapter otherwise); wired-integration-stubs-registry.md entry 0002 widened same commit; see artifacts/epic_3/command-routing-cycle_receipt.md |
| 3.5 UI panel adapter-aware | complete | 3.5 | `83e8197` | recompute_character wired end-to-end via new boundary/recomputeCharacter.ts + characterHubRuntime.ts's resolveRuleSystemId/buildRecomputeCharacterRequest (register A3); CharacterSheet.tsx's Open/Save/Clone no-op handlers closed (register A4: Open→Load screen nav, Clone→existing cloneCharacter boundary, Save→Recompute); Epic 3 fully closed; see artifacts/epic_3/ui-adapter-aware-cycle_receipt.md |
| 4.1 pcgen-run-character.sh | complete | 4.1 | `4c5d8d8` | drives real `./gradlew run --args=...` batch-export against a genuine `.pcg` (no mock); see artifacts/epic_4/pcgen-run-script-cycle_receipt.md |
| 4.2 pcgen-normalize-output.py | complete | pcgen-normalize | `a9b28da` | verified against genuine real captured PCGen XML (pf_Paladin.pcg run via 4.1's script, not just a hand-written sample); see artifacts/epic_4/pcgen-normalize-cycle_receipt.md |
| 4.3 pcgen_runner_smoke.rs | complete | 4.3 | `93003f67cd2dc5ebe72b8e040ee3511b5bb27021` | flat `tests/pcgen_runner_smoke.rs` (drift from grant's nested `tests/oracle_validation/` path — crate has no nested-integration-test convention); unignored test verifies 4.1's real script; `#[ignore]`-gated pipeline test manually verified passing end-to-end against 4.2's script (both the in-flight copy and, after 4.2 landed concurrently, the real committed script); see artifacts/epic_4/pcgen-smoke-test-cycle_receipt.md |
| 4.4 verification cycle | complete | pcgen-runner-verification | `80ce33d` | card `t_1817068a`; unignored 4.3's pipeline smoke test, parameterized normalizer call from the real pilot fixtures (case_id/source_package_id/legacy_route read at test time, not hardcoded); real end-to-end run verified: all 9 mandatory dimensions populated, zero diagnostics; Epic 4 fully closed; see artifacts/epic_4/pcgen-runner-verification-cycle_receipt.md |
| 5.1 corpus_ingest_diagnostic | complete | corpus-ingest-diagnostic | `f2c4a3e258ab7f94ebdede4e54131200bab416a0` | real per-book counts from rules_tables' own APIs (crb/apg/acg/beastiary1) + git-derived last_ingested_at; see artifacts/epic_5/corpus-ingest-diagnostic-cycle_receipt.md |
| 6.1 UI-eval defect cycle shape | complete | 6.1 | `bec34ec97b51ed3fdc2b4374eda429514d92575a` | shape documented in `## DISCOVERED` header comment below + `artifacts/epic_6/defect-cycle-shape_receipt.md`; vacuously satisfied (zero real defects exist as of 2026-07-21 — operator has not yet run a UI-eval session) |
| 6.2..6.N per-defect | dynamic-pending | — | — | spawned dynamically; not directly dispatchable until spawned |
| 7.1 residue intake | not-started | — | — | — |
| 7.2..7.M per-feature | dynamic-pending | — | — | spawned dynamically; not directly dispatchable until spawned |
| 7.N equipment/spell corpus intake | not-started | — | — | SD-24 carry-forward (register A8, A10–A17); 4 cycles (CRB-desc, APG-desc, APG-spell-text, Bestiary-1); parallel: yes, isolation: worktree |
| 7.O GE-07 pilot-shell-snapshot real implementation | blocked (awaiting operator design decision) | 7.O | docs-only, no code commit | SD-24 carry-forward (register A1); design-decision request surfaced to operator per `decisions.md §12` (deferred this bundle); open question Q5 (`risks-and-open-questions.md §4`) reviewed and confirmed accurate/unanswered; implementation cycle cannot dispatch until operator answers Q5; see artifacts/epic_7/ge07-snapshot-cycle_receipt.md |
| 7.P SD-24 documentation-staleness batch | not-started | — | — | SD-24 carry-forward (register §B: B1–B4, B6–B7, B9–B11, B14); docs-only, one batched cycle; Haiku |
| 8.1 Final criterion scan | not-started | — | — | fires LAST; Sonnet |
| 8.2 Architecture closure pipeline | not-started | — | — | fires LAST; Opus |
| 8.3 Release notes | not-started | — | — | fires LAST; Haiku |
| 8.4 Build version increment (→ 0.5.98) | not-started | — | — | fires LAST; Haiku |
| 8.5 PR + merge | not-started | — | — | fires LAST; Sonnet |

## TODO (deterministic seed)

- 7.1, 7.N (×4 corpus-intake cycles), 7.O (design-decision request first; register A1), 7.P (SD-24 doc batch; register §B), 8.1–8.5
- 6.2..6.N: not directly dispatchable — spawned only when the operator's UI-eval session logs a `## DISCOVERED` row (see Epic 6 shape below). None pending as of 2026-07-21.

## DONE

- 1.1 Source-code identifier audit — commit `62c4785d098c3c288093f5076130ec44efdce23f` — receipt `artifacts/epic_1/identifier-audit-cycle_receipt.md`
- 2.1 Board reachable (`codex-tranche-5`) — verification-only, no code commit — receipt `artifacts/epic_2/board-reachable-cycle_receipt.md`
- 2.2 Branch pushed (`tranche/5-3` matches origin) — verification-only, no code commit — receipt `artifacts/epic_2/branch-pushed-cycle_receipt.md`
- 2.3 SD-24 closure PR merged (Tier-1 gate; PR #331 `MERGED`) — verification-only, no code commit — receipt `artifacts/epic_2/sd24-pr-merged-cycle_receipt.md`
- 2.4 Working tree clean — verification-only, no code commit — receipt `artifacts/epic_2/tree-clean-cycle_receipt.md`
- 2.5 Doctrines loaded — verification-only, no code commit — receipt `artifacts/epic_2/doctrines-loaded-cycle_receipt.md`
- 3.1 RuleSystemAdapter trait — commit `490332d` — receipt `artifacts/epic_3/rule-system-adapter-trait-cycle_receipt.md`
- 3.2 Pf1Adapter extraction — commit `dccfcb87673e2980b09fc9ba50f7bf68b7945f19` (extraction+A5+A2: `4fe7703`; RuleSystemAdapter impl: `dccfcb8`) — receipt `artifacts/epic_3/pf1-adapter-extraction-cycle_receipt.md`
- 3.3 StubAdapter — commit `c41aedc` — receipt `artifacts/epic_3/stub-adapter-cycle_receipt.md`
- 3.4 Tauri command routing — commit `49097b4` — receipt `artifacts/epic_3/command-routing-cycle_receipt.md`
- 3.5 UI panel adapter-aware — commit `83e8197` — receipt `artifacts/epic_3/ui-adapter-aware-cycle_receipt.md`
- 5.1 corpus_ingest_diagnostic — commit `f2c4a3e258ab7f94ebdede4e54131200bab416a0` — receipt `artifacts/epic_5/corpus-ingest-diagnostic-cycle_receipt.md`
- 4.1 pcgen-run-character.sh — commit `4c5d8d8` (receipt commit `83063f8`) — card `t_dbbbdb9f` — receipt `artifacts/epic_4/pcgen-run-script-cycle_receipt.md`
- 4.2 pcgen-normalize-output.py — commit `a9b28da` (receipt commits `0395d40`/`1002c2c`/`b2cf6f0`) — card `t_265eb8be` — receipt `artifacts/epic_4/pcgen-normalize-cycle_receipt.md`
- 4.3 pcgen_runner_smoke.rs — commit `93003f67cd2dc5ebe72b8e040ee3511b5bb27021` (receipt commit `41bd637`) — card `t_fdf81197` — receipt `artifacts/epic_4/pcgen-smoke-test-cycle_receipt.md`
- 4.4 verification cycle — commit `80ce33d` — card `t_1817068a` — receipt `artifacts/epic_4/pcgen-runner-verification-cycle_receipt.md`
- 6.1 UI-eval defect cycle shape — commit `bec34ec97b51ed3fdc2b4374eda429514d92575a` — card `t_6c03eb36` — receipt `artifacts/epic_6/defect-cycle-shape_receipt.md`

## DISCOVERED

> **Epic 6 per-defect UI-eval cycle shape (established by criterion 6.1, 2026-07-21):**
>
> 1. **Intake:** the operator's UI-evaluation session logs a defect as one row appended to this `## DISCOVERED` list, in the form:
>    `- <ISO-8601 timestamp> — origin: <ui-eval session/ref> — priority: <tag, e.g. none | [P0-bump]> — <description> — suggested criterion: `6.<next>``
> 2. **Dispatch:** the dispatcher picks the row → spawns exactly one cycle for it (criterion `6.<next>`, dynamic) → RED (a failing test reproducing the defect) → GREEN (the fix) → dual-audit (identifier-discipline + wired-integration four-check, scoped to that cycle's diff) → one cycle receipt at `artifacts/epic_6/<defect-id>_cycle_receipt.md` (schema: `artifacts/README.md`).
> 3. **Closure gate:** Epic 6 is closed when every row logged under this heading has either a `complete` cycle receipt (linked from `## DONE`) or an explicit `## Open blockers` entry. An **empty** `## DISCOVERED` list for Epic 6 satisfies this gate vacuously — there is nothing to close.
> 4. **Queue cap:** per `loop-instruction.md §8` (inherited from SD-24), more than 10 unprocessed Epic-6 rows under this heading is NON-self-healable — stop and surface rather than keep dispatching.
>
> **Vacuous-satisfaction note (criterion 6.1, 2026-07-21):** as of this cycle, the operator has not yet run a UI-evaluation session, so zero real defects have been discovered and zero rows exist below this comment for Epic 6. This is expected, not a blocker: Epic 6's closure gate ("every UI-discovered defect has either a complete cycle receipt or an `## Open blockers` entry") holds trivially over the empty set. No defect was fabricated to exercise the shape — criteria 6.2..6.N remain `dynamic-pending` until the operator's session produces real rows, at which point each new row is processed per the shape above.
- Criterion 3.4 wired real (non-test) call sites to `Pf1Adapter` via each command file's own `resolve_rule_system_adapter` (`Box::new(Pf1Adapter)` for `"pf1"`), confirmed by `cargo build -p codex-desktop` dropping from 7 dead-code warnings to 5 pre-cycle vs. post-cycle. The two `#[allow(dead_code)]` annotations on `Pf1Adapter`'s struct/inherent-impl in `pf1_adapter.rs` are now cosmetically stale (the struct is genuinely used; the attribute is merely inert, not causing a warning either way) but were **not** removed — `pf1_adapter.rs` is outside criterion 3.4's file-touch grant (`cycles/3_4.md` names only the three command files + conditionally `main.rs`). Small, bounded, cosmetic-only; a future cycle that already has `pf1_adapter.rs` in its own grant (or an explicit housekeeping cycle) can drop the two annotations. Not blocking.
- 3.4's own carry-forward note (register A3, from `cycles/3_4.md`) is reconfirmed still true post-cycle: `grep -rn` across `apps/desktop/src/` for any of the three command names still returns nothing — zero frontend callers exist yet. Criterion 3.5 owns closing that gap. **Resolved by 3.5** (see below).
- Criterion 3.5: `revisionId` never crosses the wire to the frontend (`CharacterSummaryDto` / `LoadSavedCharacterResponse` in `character_hub.rs` never expose it, even though every mutate-op advances it server-side). This blocks any honest UI caller of `re_save_character` (which needs `expectedRevisionId` for its write-conflict guard) — register A3 was closed via `recompute_character` instead, which needs no such value. A follow-on cycle touching `character_hub.rs`'s response DTOs should add `revisionId` so `re_save_character` can eventually get a real frontend caller too. See `artifacts/epic_3/ui-adapter-aware-cycle_receipt.md`'s own `## DISCOVERED` section for the full note.
- Criterion 5.1: `beastiary1::mod.rs`'s `MonsterId` enum has no public `ALL`/count constant (unlike `ClassId::ALL`/`ApgClassId::ALL`/`AcgClassId::ALL` on the other three books) — `corpus_ingest_diagnostic.rs` carries its own compiler-checked-exhaustive 41-entry list out of file-touch-grant necessity. A future cycle with `beastiary1::mod.rs` in its grant should add a real `MonsterId::ALL` constant mirroring the other books, so this and future consumers don't need a duplicate list.
- Criterion 5.1: `last_ingested_at` is computed via `git log -1` against each book's `rules_tables` directory at runtime (mirrors `build.rs`'s existing `git_short_sha()` graceful-degradation idiom) — a packaged production build (no `.git` checkout shipped) will report `null` for every book. SD-26's planned JSON ingest cache should replace this with a persisted ingest-time timestamp that survives packaging.
- Criterion 5.1: pre-existing, unrelated failing test `apps/desktop/src/sd21/buildVersionTriple.test.ts` (`Cargo.toml` 0.5.97 vs `package.json` 0.5.98 version drift) — confirmed failing both with and without this cycle's diff via `git stash`; not caused by this cycle; belongs to whichever cycle/epic owns the version-increment-cycle (likely 8.4).
- Criterion 4.1: no real PCGen-native `.pcg` character file exists anywhere in either repo corresponding to the SD-25 pilot case (`tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt` / `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt`). Codex's `tests/fixtures/rules_core/pf1_*.txt` fixtures are its own `key=value` deterministic-input-contract format, not PCGen's `.pcg` format — PCGen's real CLI only accepts `.pcg` via `-c`, and no converter between the two formats exists in either repo. `pcgen-run-character.sh` was built format-agnostic and verified end-to-end against a real bundled PCGen fixture instead (`code/testsuite/PCGfiles/pf_Paladin.pcg`). Criterion 4.4 (verification cycle, gated on 4.1–4.3) needs a real pilot-case `.pcg` to exist before it can close the loop against the golden fixture per its own doc's "Inputs" section — flagging forward, not attempted here (outside 4.1's file-touch grant). See `artifacts/epic_4/pcgen-run-script-cycle_receipt.md` for full detail.
- Criterion 4.2: reconfirms 4.1's pilot-`.pcg` gap (still open) but closes the loop one step further — chained 4.1's real script against `pf_Paladin.pcg` end-to-end and fed the genuine captured XML export through `pcgen-normalize-output.py`; all 9 `SelectedDimension`-shaped outputs matched the raw XML exactly with zero diagnostics. This is real PCGen runtime output (not the pilot character, but not hand-written either), so criterion 4.2 itself is proven against real captured data ahead of 4.4. 4.4 (or a follow-on cycle) still needs a real pilot-case `.pcg` for the golden-fixture comparison to be about the actual SD-25 pilot character specifically. See `artifacts/epic_4/pcgen-normalize-cycle_receipt.md`.
- Criterion 4.3: `cycles/4_3.md`'s file-touch grant path (`tests/oracle_validation/pcgen_runner_smoke.rs`) is unusable as literally written — this crate's Cargo integration-test discovery only auto-compiles `.rs` files directly under `tests/`, never files in an un-marked subdirectory; a file at that exact path would never run under `cargo test`. Corrected to the crate's real flat `tests/<name>.rs` convention (`tests/pcgen_runner_smoke.rs`). Future cycles authoring new Rust integration tests in this bundle should use the flat convention directly rather than copying a nested path from grant text. See `artifacts/epic_4/pcgen-smoke-test-cycle_receipt.md`.
- Criterion 4.3: while this cycle was mid-push-retry, criterion 4.2 (`scripts/pcgen-normalize-output.py`) landed on `tranche/5-3` concurrently. Re-running the `#[ignore]`-gated pipeline test against the now-real, committed 4.2 script confirms it passes end-to-end for real (not just against the temporary pre-landing copy used for this cycle's own verification). 4.4 can remove the `#[ignore]` immediately — its only remaining real blocker is the pilot-`.pcg` gap noted above (4.1's discovery).
- Criterion 4.4: closed the pilot-`.pcg` gap by explicit re-scope (4.3's second authorized option) rather than hand-authoring a new `.pcg` (forbidden by 4.4's "no new production files" grant): the live PCGen run still uses the `pf_Paladin.pcg` substitute, but the normalizer invocation is now parameterized from the real pilot fixtures' `case_id`/`source_package_id`/`legacy_route` fields (read at test time) and the output is asserted identity-comparable against the golden fixture, consistent with the golden fixture's own `codex_output_state=unresolved`/`current_claim_status=not_yet_grounded` non-parity disclaimer. A future cycle (SD-26 oracle-harness work or a dedicated Epic 4 follow-on) should hand-author or GUI-build a genuine `pf1-crb-human-fighter-level1.pcg` so the pipeline can eventually run the literal pilot character. Also flagged: the golden fixture's `source_package=core_rulebook`/`source_system=pathfinder-1e` split-naming differs from the deterministic-input fixture's dotted `source_package_id=pf1.core_rulebook` — non-blocking, future fixture-schema reconciliation. See `artifacts/epic_4/pcgen-runner-verification-cycle_receipt.md`.

## Cycle log

(empty)

## Open blockers

(empty)

---

*Per `loop-instruction.md §6 step 7`: the orchestrator updates this file in place on every cycle via the concurrent-write protocol (`§5`).*
