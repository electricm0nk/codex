---
canonical: true
owner: god-emporer
bundle_id: SD-33
date: 2026-08-24
---

# SD-33 Forward-Scope Register

Successor work depending on this package's output. **No unowned tidiness entries** — every row names a home.

**This register is not a parking lot.** Scope that was in SD-33's Definition of Done at launch cannot appear here; that would be the laundering `../../governance/blocker-closure-doctrine.md` removes. A row here is either (a) work that only becomes possible *because* SD-33 shipped, or (b) a research question SD-33's posture deliberately does not answer.

## C1.x — Owned by an SD-33 successor epic

| ID | Item | Owner |
|---|---|---|
| C1.1 | Whatever Epic 1's probe-surface census (AT-33-E1-003) finds has **no probe at all**. Those kinds cap G4's reach and their unit count is a bundle-level figure. Building a probe for a kind that has none is net-new capability, not SD-33 scope — but it is **named, counted, and owned**, never silently absorbed into `unverifiable`. | Unassigned pending the census's actual finding. Do not pre-assign a successor for a population not yet named. |
| C1.2 | If Epic 2 rules **Path B**, the per-shape source-reading backlog for shapes Epic 5 could not reach at Path B's throughput. **This row does not exist if Path A succeeds**, and its size is unknowable before AT-33-E2-004. | The cycle that records the Path B ruling, per `decisions.md §5`'s escalation. |

## C2.x — Future SD-N ownership

| ID | Item | Owner |
|---|---|---|
| C2.1 | **The second PCGen-format reader** (Starfinder is the obvious candidate — already in the pinned checkout at `data/starfinder`, same `.lst` format, different `.pcc` include structure). Inherited unchanged from `../SD-32-.../forward-scope-register.md` C2.1. | A future SD-N. |
| C2.2 | **Traveller, Cyberpunk Red, World of Darkness, Solarus Arcanum.** Inherited from SD-32 C2.2. SD-33's oracle harness is the first instrument in this program that is **explicitly single-system** — it depends on PCGen being the authority. Any of these systems needs its own answer to "what is the oracle", and that question is now visibly separate from "what is the reader". | A future SD-N per system. |
| C2.3 | **The form-interpreter PMMG build** ("Edge of the Sea" tranche). Inherited from SD-32 C2.3; `scripts/verify.sh` still carries the warning every cycle. | A future SD-N. |

## C3.x — Research-grade forward scope

| ID | Item | Owner |
|---|---|---|
| C3.1 | **How to verify a system whose rules exist only as prose.** SD-32 raised the ingest half of this (its own C3.1). SD-33 sharpens it: the whole anti-gaming apparatus bottoms out in *the oracle says X, verifiably against a pinned SHA*. **A prose-sourced system has no oracle at all** — the extraction *is* the corpus and the extraction is the thing most likely to be wrong. A replacement for pinned ground truth must be designed before such a system is attempted. | A research spike. Not a bundle item until the discipline question is answered. |
| C3.2 | **Whether oracle agreement is the right definition of correct.** PCGen is an implementation, not the rulebook. Where PCGen itself diverges from the printed rules, SD-33's harness will report `agree` on a shared error. SD-32 already found the inverse case — a pinned test that was wrong *and* the oracle that disproved it. **Quantifying how often the oracle is itself wrong is a distinct question** this bundle does not ask. | A research spike, worth scoping only once SD-33 has a body of agreement data. |

## D1.x — Inherited debt, verified not assumed

Not successor scope in the C1/C2/C3 sense above — this section exists so a genuinely pre-existing
failure does not simply vanish from the record once its bundle closes. Each row was **proven**, not
claimed: re-derived independently across three separate final-acceptance-scan attempts
(`AT-33-E6-001` attempts 8, 9, and 10), each in its own clean worktree, each reaching the identical
result.

| ID | Item | Owner |
|---|---|---|
| D1.1 | **29 of 599 test suites, carrying 46 of 8,034 executed tests, are pre-existing failures at the `tranche/13` cut (`f652db7ac7`) — genuinely outside SD-33's Definition of Done.** Proof, re-run this cycle: `for f in <each of the 29 failing target paths>; do git log --oneline f652db7ac7..HEAD -- "$f" \| wc -l; done \| awk '{s+=$1} END {print s}'` → **0** — no failing target carries a single commit since the cut. Independently, the failing SET and its per-target `N passed; M failed` pairs are byte-identical between a clean run at `f652db7ac7` and current HEAD (normalized for target-dir paths/timings). Denominators: 29 of 599 built-and-executed suites fail; 46 of 8,034 executed tests fail (workspace totals: `cargo test --locked --no-fail-fast`, all 543 of 543 `tests/*.rs` targets plus lib/bin/doc-tests). The 29 targets (**31 through attempt 10; `src/bin/ingest_races.rs` and `tests/sd27_alternate_racial_trait_reachability.rs` were FIXED outright by the operator's 2026-08-26 Skinwalker fold and are green at attempt 12, and the executed denominator grew 8,026 -> 8,034 with the fold's own 8 new cases -- a shrink of inherited debt, re-derived by `cargo test --locked --no-fail-fast` attributed back to each `Running` line in `artifacts/epic-6-closure/AT-33-E6-001-attempt12_cycle_receipt.md`**): 29 `tests/*.rs` integration targets (`sd13_sorcerer_*` ×3, `sd18_cleric_level{11..20}_widening` ×10, `sd24_{identifier_discipline,wired_integration}_audit`, `sd26_{cache_acg,cache_apg,identifier_discipline_audit}`, `sd27_{ability_automatic_granted_race_traits,advanced_race_guide_cache_shape,book_license_record_counts,equipment_modifier_price_matches_corpus_cost_token,known_spells_must_be_on_the_class_spell_list}`, `sd30_declared_product_identity_in_shipped_class_features`, `sd31_class_feature_corpus_key_uniqueness`, `duergar_invisibility_sla_reaches_a_player_via_monster_codex`, `formula_interpreter_family_fixture_check`, `no_foreign_home_paths`, `v06_corpus_trap_report`). This bundle **verified** the inheritance rather than assuming it — see `artifacts/epic-6-closure/AT-33-E6-001-attempt10_cycle_receipt.md` Check 3 for the full re-derivation, or attempt 9's Check 2 for the first independent confirmation. | A future SD-N's own suite-green epic. Not this bundle's — its own commits added 0 of these 29 failures. |
| D1.2 | **`scripts/verify.sh`'s `site-dashboard-check` stage cannot bound its own runtime and hangs.** Root-caused this cycle, one level past prior waves' "environmental" note: `publish-site-dashboard.sh --check` invokes `cargo run --bin v06_work_inventory -- --summary` with **no timeout wrapper** in either `verify.sh` or the publish script; the producer's own internal 600s timeout had already fired once and the stage was killed mid-way through a second 100%-CPU attempt. Reproduced identically across three different diffs (attempt 9, the `sd33-r9-corpus-sweep` lane, and attempt 10) — a real, reproducible gate-plumbing defect, not noise. | A future cycle with write scope to `scripts/verify.sh` or `scripts/publish-site-dashboard.sh` to add the missing timeout wrapper. Not attempted here — outside `AT-33-E6-002`'s write scope (package docs only). |

## Carried forward from SD-32

| ID | Item | Owner |
|---|---|---|
| C1.8 (carry) | Wire `v06_corpus_trap_report -- --audit` into `scripts/verify.sh` as a real stage. Inherited unclosed through SD-31 and SD-32. **SD-33 has a natural home**: AT-33-E1-004 already opens `verify.sh` for the denominator gate, so the wiring pattern is established in the same epic. | Epic 1, if the cycle that lands AT-33-E1-004 can absorb it without scope creep; otherwise a named follow-on cycle. **Not another silent carry.** |
