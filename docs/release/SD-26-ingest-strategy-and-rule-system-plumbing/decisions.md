---
canonical: true
owner: god-emporer
status: planning-ready (operator directives 2026-07-21; bundle authored from /governance/loop-instruction-template.md + skill workflow-orchestrated-dispatch)
date: 2026-07-21
canonical_branch: tranche/5-4 (operator directive 2026-07-21)
kanban_board: codex-tranche-5 (reused after SD-25 closure PR lands)
companion_to: ./scope-draft.md
mirror_of: ./scope-draft.md
---

# SD-26 — Decision Record

## 1. SD-26 scope is four-load + canonical governance + closure (operator directive 2026-07-21)

**Decision (operator-pinned 2026-07-21):** SD-26 ships four loads:

1. **Oracle-harness comparator** (Epic 2) — the missing piece of `src/oracle_validation/`; consumes SD-25's PCGen runner + the SD-26 JSON cache.
2. **JSON cache build, 4 in-scope books** (Epic 3) — durable artifacts for core_rulebook + APG + ACG + Bestiary 1.
3. **Book stub manifest, 21 future-state books** (Epic 4) — operator-granted stubs for the remaining 21 PF1 books.
4. **Doctrine-cost reduction** (Epic 5) — audit + cut over-spent per-class gates.

Plus E1 + E6 (governance + closure). **Scope-cross posture** per operator directive 2026-07-21 17:39:26 ("this is work that should not be deferred any longer"). Bundle covers PF1 + future-state rule systems.

## 2. SD-26 inherits the Workflow-orchestrated dispatch (operator directive 2026-07-21)

**Decision:** SD-26's dispatch shape is the `Workflow` orchestrator per `/governance/loop-instruction-template.md §2` + skill `workflow-orchestrated-dispatch`. Same shape as SD-25's orchestrator with different per-epic concurrency + tiering.

## 3. Per-epic concurrency + tiering map

| Epic | Parallel? | Subagent tier | Notes |
|---|---|---|---|
| E1 Identifier Cleanup | no | Sonnet | Single cycle |
| E2 Oracle-Harness Comparator | no | Sonnet | Comparator serial; touches multiple `src/oracle_validation/` files |
| E3 JSON Cache Build | yes (3.1, 3.2, 3.3, 3.4) | Sonnet | Per-book parallel with `isolation: worktree` |
| E4 Book Stub Manifest | yes after E4.1 | Sonnet | 21 cycles, one per future-state book |
| E5 Doctrine-Cost Reduction | no | Sonnet | Single audit cycle |
| E6 Closure Epilogue | no | Haiku (6.3, 6.4); Sonnet (6.1, 6.5); Opus (6.2) | Subagent tiering per-criterion |

## 4. Build counter inheritance

**Decision (per `/governance/loop-instruction-template.md §1 item 7`):** SD-26's first concrete value lands at **`0.5.99`** (develop at `0.5.97` post-SD-24; SD-25 closure bumps to `0.5.98`; SD-26's per-criterion tiering gets Housekeeping-Haiku on the version-bump step).

## 5. Publish mode is move-not-copy (operator directive 2026-07-21)

Same as SD-25: workspace-side copy deleted on the publish commit.

## 6. Tier-1 launch-gate dependency

**Decision:** SD-26 cannot dispatch Epic 3+ until SD-25 closure PR is merged to develop. The tier-1 gate is enforced by E2's criterion 2.5 (verification cycle) reading SD-25's tier-1 gate. SD-25 ships the Hub-of-Hubs interface + PCGen runner scaffolding; SD-26 consumes those.

## 7. JSON schema: Shape B (progressive completeness)

**Decision (per prior conversation):** the JSON cache uses the Shape B schema:

```
data/corpus/{book}/{content_kind}/{content_id}.json

{
  "population": "in_scope" | "future_state" | "rule_system_stub",
  "completeness": "chassis_only" | "chassis_plus_extract" | "full",
  "data": { ... content-type-specific fields ... },
  "source_lst": {
    "path": "pathfinder/paizo/roleplaying_game/<book>/<file>.lst",
    "sha256": "...",
    "line": <int>
  }
}
```

`in_scope` + `future_state` + `rule_system_stub` discriminator lets the cache schema unify real content + operator-granted stubs + future-system stubs under one shape. The cycle picker and the visibility surface read it without branching.

## 8. Book stub manifest: 21 entries in Stubs Registry

Per operator directive 2026-07-21 15:41:03 ("stubs visibility for future-state books only, not in-scope books"). SD-26's Epic 4 introduces the `book_stub` kind to `governance/wired-integration-stubs-registry.md`:

```
book_id: <book-slug>
book_name: <book-display-name>
status: stubbed
planned_resolution_bundle: <next-bundle-id>
registered_by: claude-code
registered_at: <ISO-8601>
operator_granted: true
```

21 entries land for: advanced_race_guide, adventurers_guide, beginner_box, bestiary_2..6, bonus_bestiary, core_essentials, horror_adventures, monster_codex, mythic_adventures, occult_adventures, pathfinder_unchained, ultimate_campaign, ultimate_combat, ultimate_equipment, ultimate_intrigue, ultimate_magic, ultimate_wilderness. The Stubs Registry's existing `codex-stub` kind stays for non-book-code stubs (entry #0001 browser-preview fallback).

## 9. Override flags

| Flag | Default | Set behavior |
|---|---|---|
| FLAG-A: STRICT-STOP-AT-DEADLINE | unset (grace-tail) | strict stop at operator's deadline |
| FLAG-B: BUDGET-MODE | unset (Anthropic-only) | enable Qwen / ollama model for mechanical fan-out (Epic 4's 21 cycles) |
| FLAG-C: STRICT-CACHE-COVERAGE | unset | require 100% field coverage; relax only if audit shows the threshold cannot be met (operator override per `loop-instruction.md §2`) |

## 10. Operator-deferred shape decisions

- **Concurrent-write protocol scope.** Extended from SD-25 to cover `data/**/*.json` + `governance/wired-integration-stubs-registry.md` per template §5.
- **Per-book ordering for E3.** Operator-pinned at cycle launch. Default: alphabetical by book name.
- **Oracle-harness comparator parity policy.** Per `tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt:current_claim_status=not_yet_grounded`, SD-26 E2 cycles upgrade the pilot case's status to `oracle_checked` only after the comparator asserts match.
- **Stubs Registry entries per book.** Each entry carries `planned_resolution_bundle: "SD-27"` (operator-pinned default; operator may override).

## 11. SD-25 corpus-intake findings incorporated into the JSON cache design (added 2026-07-22)

**Context.** SD-26's Shape B schema (§7) was drafted 2026-07-21 from a prior conversation, before SD-25's Epic 7 (`equipment/spell corpus intake`) actually executed real per-book field-completion work against CRB, APG, and Bestiary 1. **No dedicated "durable JSON cache research" document exists anywhere in SD-25** — this was checked directly (`find docs/release/SD-25-ui-evaluation-defect-closure -iname "*cache*" -o -iname "*json*"` returns nothing) and confirmed by an independent search agent. What exists instead is real, substantial, well-evidenced execution work — scattered across Epic 5's `corpus_ingest_diagnostic` prototype and Epic 7's four corpus-intake cycle receipts — that is directly relevant to Epic 3's JSON-cache build even though it was never packaged as a single research artifact. This section incorporates that real learning into SD-26's design, superseding the untested assumptions in §7/§8/`technical-design.md §3`/`content-unit-inventory.md §1` where they conflict.

All citations below are paths **inside the SD-25 release folder** at `../SD-25-ui-evaluation-defect-closure/` (the repo-relative form for the post-publish SD-26 location at `docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/`); the in-SD-25 filenames are then appended (e.g. `artifacts/epic_5/corpus-ingest-diagnostic-cycle_receipt.md:19`). All SD-25 citations in this section are sourced from the **published** repo copy at that path — SD-25 was published into `repos/codex/docs/release/SD-25-ui-evaluation-defect-closure/` before this SD-26 publishing pass, so the `../SD-25-...` form resolves correctly from the post-publish location of SD-26. The pre-publish (workspace-side) authoring version of this section used absolute paths into `/home/ubuntu/workspace/repos/codex/docs/release/SD-25-...` because the workspace-side docs/release/ tree was the only canonical source at authoring time; the publish-pass correction (this paragraph) is to replace those absolute paths with the `../SD-25-...` form used in §12 below, so future readers of the SD-26 repo copy can navigate by sibling-folder relative path.

### 11.1 Shape B is missing a persisted-timestamp field — hard requirement, not a nice-to-have

SD-25 criterion 5.1 built `corpus_ingest_diagnostic` computing `last_ingested_at` via `git log -1 --format=%cI -- <book-dir>` at runtime. This is a documented, deliberate stopgap — it returns `None`/`null` in any packaged production build (no `.git` checkout shipped), a real defect the cycle's own receipt flags for SD-26 by name:

> "SD-26's planned JSON ingest cache should replace this git-shell-out with a persisted ingest-time timestamp that survives packaging, since production builds will report `null` today." — `artifacts/epic_5/corpus-ingest-diagnostic-cycle_receipt.md:19`

**Decision:** Shape B (§7) gains a required `ingested_at: <ISO-8601>` field, stamped at JSON-file-write time by whichever E3 cycle generates it (not derived from git). `data/corpus/`'s book-level manifest (§11.3 below) aggregates these per-book so `corpus_ingest_diagnostic`'s successor can read a persisted value instead of shelling out to git.

### 11.2 Shape B's `source_lst` field cannot represent most of what SD-25 actually populated — schema gap, must fix before E3 builds anything

Shape B (§7) hard-requires every record to carry `source_lst: {path, sha256, line}` — implicitly assuming every field's value traces to a single real LST token. SD-25's four corpus-intake cycles prove this assumption is false for a large, real fraction of the now-completed CRB/APG/Bestiary-1 data:

| Provenance kind | Real count (this SD-25 pass) | Example | Receipt |
|---|---|---|---|
| Web second-source (d20pfsrd.com / aonprd.com) | 83 (CRB equipmods) + ~50 distinct-name lookups; 7 (APG, after 2 corrections) + 6 spell records; 1 (Bestiary-1) | `Flaming`, weapon/armor special-ability text | `artifacts/epic_7/corpus-intake-{crb,apg}-description_cycle_receipt.md`, `corpus-intake-apg-spell-text_cycle_receipt.md`, `corpus-intake-bestiary1_cycle_receipt.md` |
| `.COPY=`-inheritance (same-book, transitive) | 117 (CRB: 98 arms_armor + 19 general) | `Arrow` inherits `Arrow (Base)`'s description | `corpus-intake-crb-description_cycle_receipt.md` (register A11) |
| Same-line ingestion-bug fix (real LST token existed, prior codegen mis-captured it) | 67 (CRB `.CLEAR`-then-real-`DESC:` rows); 2 (APG spells, same-line `.MOD` concatenation) | `Bolt (Screaming)` | `corpus-intake-crb-description_cycle_receipt.md`, `corpus-intake-apg-spell-text_cycle_receipt.md` (register A11) |
| Same-book `PRESPELL` fallback (base record's own text narrates the variant by name) | 3 (APG `Threefold Aspect` sub-forms) | `Threefold Aspect (Young Adult)` | `corpus-intake-apg-spell-text_cycle_receipt.md` |
| `SPROP:`-token (register A10 convention; not `DESC:`) | 3 of 4 (Bestiary-1) | `Aklys` | `corpus-intake-bestiary1_cycle_receipt.md` |

None of these have a single real `path/sha256/line` LST citation in the sense Shape B assumes — a web-sourced field has no LST line at all; a `.COPY=`-inherited field's real citation is *another record's key*, not a line number; a same-line-concatenation fix's citation is "this LST line, but the existing parser mis-attributed it, corrected by inspection."

**Decision:** Shape B's `source` field becomes a discriminated union, not a single required shape:

```json
"source": {
  "kind": "lst_token" | "lst_inherited_copy" | "lst_corrected_ingest" | "web_second_source" | "same_book_fallback",
  // kind: lst_token
  "path": "...", "sha256": "...", "line": 0, "record_key": "...",
  // kind: lst_inherited_copy (additionally)
  "inherited_from_record_key": "...",
  // kind: lst_corrected_ingest (additionally)
  "original_ingest_defect": "<short description, e.g. 'first-DESC-only capture of a .CLEAR-then-real-DESC row'>",
  // kind: web_second_source (additionally)
  "url": "...", "fetched_at": "<ISO-8601>", "identity_match_basis": "<name+cost | name+weight | name+school+level | ...>",
  // kind: same_book_fallback (additionally)
  "fallback_basis": "<e.g. 'PRESPELL base record narrates this sub-form by name'>"
}
```

This is not a hypothetical concern — it is the *majority* provenance shape for the fields SD-25 actually closed this pass (CRB: 67+117+83=267 of the 200 newly-populated records are non-`lst_token`; APG description: 331/331 populated records are web-sourced, zero are `lst_token`; APG spell text: the 2 recovered + 6 web-sourced + 3 fallback records are all non-`lst_token`). **A cache built strictly to the original Shape B could not represent APG's equipment descriptions at all** (0/338 records have a native `DESC:` token — the entire populated field is web-sourced).

### 11.3 Cache generation strategy: dump from the completed Rust `rules_tables` modules, do not re-parse raw LST from scratch

SD-25 Epic 7 did its field-completion work **directly in the Rust `rules_tables` source modules** (`src/rules_core/rules_tables/{crb,apg,beastiary1}/...`), not in any JSON cache — because no cache existed yet. Those modules are now, as of SD-25's close, the actual source of truth for CRB/APG/Bestiary-1 equipment/spell completeness, already carrying every fix and every web-sourced value described in §11.2.

**Decision:** SD-26's Epic 3 cycles should generate `data/corpus/<book>/**/*.json` by serializing the current state of the completed Rust structs (`EquipmentTableEntry`, spell-list entries, etc.), not by re-running a fresh LST-to-JSON parse. Re-parsing raw LST from scratch would **silently lose every non-`lst_token` fix** in §11.2's table — the `.CLEAR`-bug fix, the `.COPY=`-inheritance, the `apg_spells.lst` same-line-concatenation fix, and all web-sourced content (which by definition cannot be re-derived from LST at all). A raw re-parse would regress CRB equipment-description coverage from 67.9% back to something close to SD-24's original 61.2% ceiling, and would zero out APG's 331/338 (97.9%) entirely.

**Consequence for E3's per-cycle procedure:** each of criteria 3.1–3.4's cycles reads the corresponding `rules_tables` module's current runtime state (via a small dump routine, or by iterating the module's own public `ALL`/`SPELL_LIST`/`equipment_tables()` accessors — the same accessors `corpus_ingest_diagnostic.rs` already calls) and serializes it to Shape B/§11.2's discriminated-union shape, carrying forward each field's real provenance kind from the corresponding SD-25 receipt (cited in §11.2's table) rather than re-deriving it. Where a field's real provenance isn't recoverable from the current Rust source alone (e.g. the struct doesn't currently store which of the five provenance kinds produced a given value), the E3 cycle should default `source.kind` conservatively to `"lst_token"` **only** if a real, checkable LST citation exists for that specific field, and flag anything it can't confidently attribute rather than guessing.

### 11.4 Real, validated coverage ceilings — informs FLAG-C's default posture

FLAG-C (`STRICT-CACHE-COVERAGE`, §9) already anticipates "relax only if audit shows the threshold cannot be met." SD-25 didn't just anticipate this — it **measured** it, per-field, for 3 of the 4 in-scope books:

| Book | Field | Real ceiling achieved | Genuine residual gap (not a "look harder" gap) | Receipt |
|---|---|---|---|---|
| core_rulebook | equipment `description` | 2021/2977 (67.9%, up from 61.2%) | 956 records: slot-type markers, pricing-formula bookkeeping categories (`Ability Score / Charisma 11` etc.), `.COPY=` chains whose own base also has no description | `corpus-intake-crb-description_cycle_receipt.md` |
| advanced_players_guide | equipment `description` | 331/338 (97.9%) | 7 records: no `COST`/`WT` token at all, one cost-identity mismatch with no weight cross-check available, 4 higher-tier variants with no distinct sourced sub-mechanic text | `corpus-intake-apg-description_cycle_receipt.md` |
| advanced_players_guide | spell `full_text` | 284/297 (95.6%, up from 87.9%); `description` 285/297 | 13 records: 12 cross-book `.COPY=` variants whose base spell doesn't exist in this book's own file (documented scope boundary, not re-litigated), 1 corpus typo (`Wall of Thorms`) referencing a correctly-spelled base spell that lives in a *different* book's module | `corpus-intake-apg-spell-text_cycle_receipt.md` |
| beastiary (Bestiary 1) | equipment (all fields) | 4/4 (100%) | none — but the real record count is **4**, not the ~7 the original SD-25 cycle doc estimated; verify counts against the real corpus before writing an E3 cycle doc, don't trust a prior estimate | `corpus-intake-bestiary1_cycle_receipt.md` |

**Decision:** E3's per-book cycles should expect and budget for a genuine, non-100% ceiling on CRB and (to a lesser extent) APG equipment/spell fields — this is proven, not hypothetical. FLAG-C's default (unset = require 100%) will trip on CRB/APG unless the operator either sets the flag's relax-path explicitly per book, or E3's cycle docs pre-declare each book's known-achievable ceiling (the table above) so a cycle isn't marked failed for reproducing an already-proven, honest gap. ACG (advanced_class_guide, already SD-24-complete at time of SD-26 authoring) is not covered by this pass and should be independently re-verified against its own real ceiling before E3.3 assumes any number for it.

### 11.5 Reusable web-second-source methodology (directly informs E3's per-book cycle docs)

SD-25 established, exercised, and documented a concrete methodology across 4 independent cycles — this should be written into E3's cycle docs verbatim rather than re-derived:

- **Allowed domains only:** `d20pfsrd.com`, `legacy.aonprd.com` / `aonprd.com`. No other source.
- **Identity-match discipline:** match by name + a second confirming field — normally cost (equipment) or school/level/classes (spells). **When the corpus's own `COST:` token is unreliable** (a real, confirmed corpus quirk: a subset of APG `ArmsArmor` "named specific magic weapon/armor" records carry a `COST:` far below the real market price — e.g. `Beaststrike Club` corpus `0` gp vs. real 7,300 gp — while `weight` matched exactly in 11/11 checked cases), cross-check **weight** instead of cost. Per `corpus-intake-apg-description_cycle_receipt.md`.
- **Reject same-named cross-book/edition-cousin false matches.** Real example: a `d20pfsrd.com` search for APG's "Malediction" first returned a later "Book of the Damned" reprint with different classes/level/school/mechanic — rejected once the corpus's real `CLASSES:`/level data didn't match, and the correct original-APG source was found on a different page. Per `corpus-intake-apg-spell-text_cycle_receipt.md`.
- **No confident match → leave the field unset, never guess.** Every one of SD-25's 4 receipts documents specific, named residual gaps left `None` for exactly this reason (see §11.4's table) — this is the expected, correct outcome for a genuine corpus ceiling, not a cycle failure.
- **Cite every web-sourced field**: URL + fetch date + the identity-match basis used, in the JSON's `source.url`/`source.fetched_at`/`source.identity_match_basis` fields (§11.2).
- **Reasonable pacing, category-page fetches over per-item scraping.** SD-25's APG-description cycle covered ~331 records from just 10 category-aggregate pages.

### 11.6 Corpus-hygiene defects E3 should account for or fix before/during cache generation

- **CRB `equipmods.rs` has 314 of 658 entries sharing a duplicate `key` with another entry (344 truly-unique keys).** If E3.1 generates `data/corpus/core_rulebook/equipment/` by dumping this module's current state (per §11.3), it will inherit this duplication into the JSON cache unless corrected first. Not yet fixed anywhere — flagged as a real, open, pre-existing defect. Per `corpus-intake-crb-description_cycle_receipt.md`'s Discovered Finding #1.
- **`apg_spells.lst` (the raw corpus, not the Rust module) has at least 3 physical lines with two `.MOD` stanzas concatenated with no line break**, a genuine upstream PCGen data-quality defect (not a codex-side bug). SD-25's own pass fixed 2 of the 3 real hits; if any future work re-parses `apg_spells.lst` directly (rather than following §11.3's "dump from Rust source" strategy), it must re-scan for this defect class rather than trusting a naive last-`DESC:`-wins parser. Per `corpus-intake-apg-spell-text_cycle_receipt.md`.
- **A newly-found, still-open APG spell-parsing gap:** `CLASSES:X=N[PREVAREQ:...]`-suffixed single-pipe-group tokens parse to `level: None` even though a real numeric level is present in the corpus (the level-extraction rule only succeeds when at least one pipe-delimited `CLASSES:` group has no bracket-suffixed prerequisite tag). Out of SD-25's own file-touch scope; not yet fixed. Per `corpus-intake-apg-spell-text_cycle_receipt.md`'s discovery-forwards.
- **`beastiary1::mod.rs`'s `MonsterId` enum has no public `ALL`/count constant** (unlike `ClassId::ALL`/`ApgClassId::ALL`/`AcgClassId::ALL` on the other three books) — SD-25's `corpus_ingest_diagnostic.rs` had to hand-maintain its own duplicate 41-entry list to work around this. E3.4 (Bestiary-1 cache build) will likely hit the same friction; consider adding the real `ALL` constant as part of that cycle rather than duplicating the workaround a second time. Per `artifacts/epic_5/corpus-ingest-diagnostic-cycle_receipt.md`.

### 11.7 A `pcgen_import`-backed shared codegen path is the right tool for bulk ingest, not for gap-closing — informs E3's tooling decision, does not block it

SD-25's register A8 (shared codegen path decision) was deliberately deferred in all 4 corpus-intake cycles, for reasons that generalize: a generic LST-token→JSON codegen path is well-suited to **bulk extraction of well-formed corpus tokens** (the shape of SD-26 Epic 3's actual job — building a cache from scratch for 4 books), but is *not* well-suited to the kind of surgical gap-closing SD-25 did (web-second-sourcing has no LST token to extract at all; ingestion-bug fixes are one-off data-integrity corrections, not repeatable extraction rules). **This does not argue against building a shared codegen path for E3** — it argues that whatever E3 builds must, per §11.3, generate from the already-corrected Rust source (which already carries the surgical fixes), and should expect to still need a manual/scripted fix-up layer for the non-`lst_token` provenance kinds in §11.2, not assume a single generic parser closes 100% of every book automatically. `src/pcgen_import/lst_parser/{equipment,spell}.rs`'s tokenizers were independently verified this SD-25 pass to already correctly preserve every raw token per record (including multiple `DESC:` tokens per row) — the tokenizer is not the bottleneck; the downstream codegen-to-Rust-struct step is. Per all four `corpus-intake-*_cycle_receipt.md` files' "Register A8" sections.

## 12. Cross-references

- `/governance/loop-instruction-template.md` (REPO-LOCAL CANONICAL).
- `governance/no-stub-mvp-doctrine.md` + skill `wired-integration-discipline`.
- `governance/identifier-discipline.md` + skill `identifier-discipline`.
- `governance/wired-integration-stubs-registry.md`.
- `~/.hermes/profiles/god-emporer/skills/orchestration/workflow-orchestrated-dispatch/SKILL.md`.
- `../docs/release/SD-25-ui-evaluation-defect-closure/decisions.md` — Tier-1 launch-gate dependency (consumes Hub-of-Hubs + PCGen runner).
