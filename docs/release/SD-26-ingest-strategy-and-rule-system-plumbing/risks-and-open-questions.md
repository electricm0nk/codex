# SD-26 — Risks and Open Questions

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

## 2. Non-self-healable conditions (write to `## Open blockers`)

| Condition | Action |
|---|---|
| Working tree diverged from `tranche/5-4` needs manual rebase | `## Open blockers`; exit FAIL |
| Two live orchestrators on conflicting files | First wins; second writes `CLAIM-EXISTS`; exit FAIL |
| SD-25 closure PR not merged to develop | Loop refuses to start (Tier-1 launch gate) |
| `## DISCOVERED` queue > 10 entries | Operator override required; pause |
| RED → GREEN not preserved in artifact | Cycle re-run with RED → GREEN captured |
| `success: true` from fake operation; inline mock in shipping module; "Would …" in shipping code | Cycle rejected; cannot mark `complete` |
| Concurrent-write protocol fails 5 times (CLAIM-EXISTS) | Stop orchestrator; operator intervention |
| Pilot case comparator asserts MISMATCH on a dimension SD-23/SD-24 claim `oracle_checked` | Defer to `## Open blockers`; operator decision on engine-side vs PCGen-side bug |

## 3. Override flags

| Flag | Default | Set behavior |
|---|---|---|
| FLAG-A: STRICT-STOP-AT-DEADLINE | unset (grace-tail) | strict stop at operator's deadline |
| FLAG-B: BUDGET-MODE | unset (Anthropic-only) | enable Qwen / ollama for E4 mechanical fan-out |
| FLAG-C: STRICT-CACHE-COVERAGE | unset | require 100% field coverage; relax only if audit shows the threshold cannot be met — **SD-25 already proved this relax-path is the common case for CRB (67.9% ceiling) and, to a lesser extent, APG (95.6–97.9%), not a rare exception; see `decisions.md §11.4`** |

## 4. Open questions

| Q | Question | Default |
|---|---|---|
| Q1 | E3 per-book ordering | Alphabetical by book name (core_rulebook → advanced_players_guide → advanced_class_guide → bestiary 1) |
| Q2 | `book_stub` entries' `planned_resolution_bundle` | `SD-27` (operator-pinned default) |
| Q3 | Oracle-harness comparator parity policy on partial-failure dimensions | First mismatch aborts the cycle; operator decides whether to fix Codex-side, fix PCGen-side, or relax the comparator's tolerance |
| Q4 | ACG (`advanced_class_guide`)'s real per-field completion ceiling — SD-25's corpus-intake pass covered CRB/APG/Bestiary-1 but not ACG | Unknown; E3.3 should re-verify ACG's real ceiling against the corpus directly (same method as `decisions.md §11.4`'s table) before assuming any number, rather than assuming it matches CRB/APG's shape |
| Q5 | `content-unit-inventory.md §2.2` listed 21 real future-state book names but every other reference in this bundle said "22" — is there a missing 22nd book, or is "22" simply wrong everywhere? | **RESOLVED 2026-07-22 publishing pass:** the 21 names are canonical (consistent with `technical-requirements.md §3.3` + `content-unit-inventory.md §1.1`'s table); "22" was wrong everywhere. All references now consistent at "21 future-state books" / "Criterion 4.2..4.22" (21 cycles). |

## 5. Deferrals

- **Rule-system implementations beyond Pf1Adapter** (D&D 5e, Pathfinder 2e, etc.). Per operator scope-cross posture, the trait exists in SD-25; concrete implementations land in SD-27+.
- **Equipment corpus extension beyond PF1 core + APG + ACG + Bestiary 1** (deferred).
- **Storage-tier structural convergence** (deferred).
- ~~**Identifier-discipline directory renames** (`apps/desktop/src/sd<N>/` → descriptive).~~ **RESOLVED by SD-25 criterion 1.1** (2026-07-22): `sd11/`→`testerWorkbench/`, `sd15/`→`operatorTriage/`, `sd22/`→`releaseChecks/`, `sd13_support_state_matrix.rs`→`support_state_matrix_bridge.rs`, plus ~45 `SD13_*` Rust const-name strips. Not SD-26's job anymore. (One residual: `.github/workflows/check-release-manifest.yml`'s path filter was also independently fixed to match — no action needed here.)
- **Inline mocks / "Would …" strings outside bundle's file-touch**.

## 6. Latent risks

- **PCGen library build throughput.** 21 cycles for E4 + 4 cycles for E3 = 25 cycles of mechanical fan-out. Per operator's plan, Anthropic-me does the architecture; a budget model like Qwen (if available) handles the fan-out. If neither option scales, E4's 21 cycles are the bottleneck.
- **Pilot case oracle-checked transition.** SD-26's E2 verification cycle (criterion 2.5) upgrades `current_claim_status` from `not_yet_grounded` to `oracle_checked`. If the comparator asserts MISMATCH (which the operator's "20-min per class" framing implies is plausible), the cycle reverts to `not_yet_grounded` and the operator decides which side is wrong.
- **Stubs Registry format change.** Adding the `book_stub` kind to `docs/governance/wired-integration-stubs-registry.md` is a doctrine-of-record change; that file now has 2 entries as of SD-25 (`codex-stub` #0001 browser-preview fallback, and #0002 `StubAdapter` future-rule-system placeholder, widened during SD-25 criterion 3.4 to cover 3 additional call-site files). E4.1 must add the new `book_stub` kind without breaking either existing entry's semantics.
- **JSON cache Shape B, as originally drafted, cannot represent most of what SD-25 already populated.** See `decisions.md §11.2` — a real, substantial fraction of CRB/APG/Bestiary-1's now-completed fields (web-sourced, `.COPY=`-inherited, same-book-fallback, corrected-ingestion-bug) have no single-LST-token provenance. The schema must be the corrected discriminated union (`decisions.md §11.2`, `technical-design.md §3.1`) before E3 writes anything, or E3's cache will either be unable to represent large parts of the real data (e.g. 100% of APG's populated equipment descriptions) or will silently misrepresent their provenance.
- **Re-parsing raw LST from scratch for E3 would regress real, already-shipped completions.** See `decisions.md §11.3` — CRB equipment description would regress from 67.9% back toward SD-24's original 61.2%, and APG's 331/338 (97.9%) would zero out entirely, since none of that is re-derivable from LST tokens alone. E3 must generate from the completed Rust `rules_tables` modules, not from a fresh corpus parse.
- **CRB `equipmods.rs` has 314 of 658 entries sharing a duplicate key (344 truly-unique).** If E3.1 dumps this module as-is (per the corrected generation strategy), the JSON cache inherits the duplication. Real, open, pre-existing defect — not yet fixed anywhere. See `decisions.md §11.6`.
- **`apg_spells.lst`'s same-line `.MOD`-concatenation defect** (a genuine upstream PCGen data-quality issue, 1 of 3 known instances still unresolved) risks silently misattributing spell text if any future work re-parses this file directly rather than following the corrected "dump from Rust source" strategy. See `decisions.md §11.6`.
- **A real, still-open APG spell `level`-field parsing gap:** `CLASSES:X=N[PREVAREQ:...]`-suffixed single-pipe-group tokens parse to `level: None` even though a real level is present. Out of SD-25's own scope; not yet fixed. See `decisions.md §11.6`.
- **`beastiary1::mod.rs`'s `MonsterId` enum has no public `ALL` constant**, forcing hand-maintained duplicate lists in every consumer (SD-25's `corpus_ingest_diagnostic.rs` already had to do this). E3.4 will likely hit the same friction. See `decisions.md §11.6`.

## 7. Cross-reference

- `./scope-draft.md §5 Hard-stop conditions`
- `./decisions.md §3` — per-epic concurrency + tiering
- `./decisions.md §6` — tier-1 launch gate
- `./decisions.md §11` — SD-25 corpus-intake findings incorporated into the JSON cache design (schema fix, generation strategy, real coverage ceilings, methodology, corpus-hygiene defects)
- `./loop-instruction.md §5` — concurrent-write protocol
- `./loop-instruction.md §8` — self-heal posture
