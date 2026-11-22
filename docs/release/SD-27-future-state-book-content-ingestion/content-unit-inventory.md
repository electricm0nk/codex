# SD-27 — Content Unit Inventory

> **Per-content-unit N-tuple.** For SD-27, content units are the per-book / per-content-kind artifacts that flow into `data/corpus/<book>/` (Shape B v1 JSON cache) and `data/stubs/<book>.json` (stub manifest).
>
> **⚠️ 2026-07-30 — corpus figures in this file are now derived, not maintained here.** See §0.

## 0. The generated inventory is the source of truth for corpus figures

> **Merge note (2026-07-30 cross-copy merge):** §0, §4, and §5 below were merged in from the planning-tree copy at `programs/codex/requirements/SD-27-future-state-book-content-ingestion/`. §1.3's bullet list has *not* been replaced with that copy's content-kind mapping table — that hunk rewrites rather than adds, and is flagged for separate operator review rather than silently applied. See the merge report.

The repo now generates the whole-corpus work inventory mechanically:

```sh
cargo run --locked --bin v06_work_inventory     # writes docs/work-inventory.json
```

It walks every `.lst` file under `PCGEN_CORPUS_ROOT`'s
`pathfinder/paizo/roleplaying_game/` — **including books no code has ever read**
— and cross-references each unit against the compiled tables and the real
compute pipeline. Its contract, verbatim from the generated file:

> Every field below is derived from the corpus or observed from the engine.
> Nothing here is hand-maintained; two consecutive runs over an unchanged corpus
> and engine differ only in `generated_at`.

**Division of labour with this document.** This file keeps what it is uniquely
good at — SD-27's *routing*: which book is in this bundle's scope, which path on
disk each artifact lands at, which registry slot maps to which stub manifest,
what the Shape B v1 record must carry. Those are decisions, and decisions belong
in the planning package.

It no longer keeps **corpus figures** — per-book record counts, per-content-kind
inventories, book totals. Those are generated. Every hand-maintained figure in
this project's history has drifted and then actively misled; the full record is
in `docs/governance/book-ingestion-playbook.md` §6. Where this file previously
carried such a figure, it now names the field in `docs/work-inventory.json` that
supplies it.

**What to read for a book:** `books[]` for that book gives `scope`,
`engine_rule_set`, `pcc_includes` / `included_by` (the PCC include graph),
`files_enumerated` / `files_not_enumerated`, `kinds` (units per kind, each with
a `by_status` map), `trap_hits` (per-trap firing counts for that book), and
`reconciliation` (per-kind `corpus_units_total` vs `engine_records` vs `delta`).
`totals` gives `by_kind`, `by_status` and `by_book`.

**Status vocabulary.** The generator's `status_vocabulary` block is normative and
deliberately finer-grained than "done / not done". In particular `grounded`
(a computed magnitude was *observed reaching a consumer*) is strictly stronger
than `ingested-magnitude` (the engine holds the record with its real numeric
fields, but no consumer delta was observed). Calling the second one "grounded"
is the over-claim the inventory exists to prevent, and SD-27 receipts should use
the generator's words rather than coining their own.

**If a figure in this file and the generator disagree, investigate — do not
overwrite either.** §5 records the reconciliation done on 2026-07-30.

## 1. JSON cache (Epic 2 — pre-build + verify)

### 1.1 Per-book routing (per-book cycle E2.x, file-disjoint)

| Book | Path on disk | Source LST path | In-scope for SD-27? |
|---|---|---|---|
| advanced_race_guide | `data/corpus/advanced_race_guide/{class,spell,equipment,feat,race,archetype,bestiary,...}/*.json` | `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/advanced_race_guide/*.lst` | **YES** — E2.1 (pre-build) + E2.1' (verify) |
| adventurers_guide | (not created in SD-27) | `$PCGEN_DATA_ROOT/adventurers_guide/*.lst` | **NO** — deferred; the operator's dashboard routes it to SD-30 |
| beginner_box | (not created) | `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/beginner_box/*.lst` | **NO** — removed from scope per operator directive 2026-07-27 |
| bestiary_2..6 | (not created in SD-27) | per-book LST corpus | NO — deferred to SD-28+ |
| bonus_bestiary | (not created) | per-book LST corpus | NO — deferred to SD-28+ |
| core_essentials | (not created) | `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/*.lst` | **NO** — removed from scope per operator directive 2026-07-27 |
| pathfinder_unchained | `data/corpus/pathfinder_unchained/{class,spell,equipment,feat,race,archetype,...}/*.json` | `$PCGEN_DATA_ROOT/pathfinder_unchained/*.lst` (11 files) | **YES** — E2.2 (pre-build) + E2.2' (verify) |
| horror_adventures, monster_codex, mythic_adventures, occult_adventures | (not created in SD-27) | per-book LST corpus | NO — deferred to SD-28+ |
| the 6 Tier-2 Ultimate books (ultimate_campaign, ultimate_combat, ultimate_equipment, ultimate_intrigue, ultimate_magic, ultimate_wilderness) | (not created in SD-27) | per-book LST corpus | NO — deferred to SD-28+ |

**Total books in SD-27's JSON cache payload: 2** (advanced_race_guide, pathfinder_unchained) — matching the operator's `SD-27 (ARG + PU)` dashboard workchannel.
**Total future-state books removed or deferred: 19** (2 removed from scope, 17 deferred).

### 1.2 Required fields per JSON file (Shape B v1 schema, per `decisions.md §7`, extended per cycle 2.0.5)

| Field | Type | Required |
|---|---|---|
| `population` | `"in_scope" \| "future_state" \| "rule_system_stub"` | yes — **in-scope future-state books flip from `future_state` (SD-26 stub) to `in_scope` (SD-27 pre-build)** |
| `completeness` | `"chassis_only" \| "chassis_plus_extract" \| "full"` | yes — per-content-kind and per-record |
| `ingested_at` | string (ISO-8601, stamped at JSON-write time) | yes — same convention as SD-26 |
| `license` | `"OGL" \| "PI" \| "PI-REDACTED"` | yes — **new field for v1 (cycle 2.0.5 schema bump)**; every Shape B v1 record carries exactly one of these |
| `pi_field` | `<field_name> \| null` | yes — names the field whose value is Product Identity (when `license` is `PI` or `PI-REDACTED`) |
| `pi_marker` | `"redacted" \| null` | yes — for PI-tagged records, must be `"redacted"` (per the 2026-07-25 OGL review's redaction-to-marker policy) |
| `data` | content-type-specific | yes |
| `source_lst` | `{ path, sha256, line }` | yes — provenance chain back to the source LST record |

The `license` + `pi_field` + `pi_marker` triad is the schema bump cycle 2.0.5 lands. v0 records (Shape B v0 = SD-26's legacy shape) survive in the 4 in-scope books but are retro-fitted to v1 in cycles 2.0.6-2.0.9.

### 1.3 Per-book content-kind inventory (per `technical-design.md §2.1`)

Each book's per-content-kind inventory is derived from the source LST corpus, not assumed from a canonical list. The 19 future-state books' canonical content kinds are:

- `classes` — class records (CRB-style `_classes.lst`)
- `spells` — spell records (CRB-style `_spells.lst`)
- `equipment` — weapons, armor, gear (CRB-style `_equipment.lst`)
- `feats` — feat records (CRB-style `_feats.lst`)
- `bestiary` — monster stat blocks (for Bestiary 2-6 + bonus_bestiary; per-monster-block shape)
- `racial` — race records (CRB-style `_racialabilities.lst` + `_abilities_race.lst`)
- `archetypes` — archetype records (CRB-style `_archetypes.lst` + `_classarchetypes.lst`)
- `traits` — trait records (CRB-style `_traits.lst`)
- `domains` — domain records (CRB-style `_domains.lst`)

Per-book cycle inventories the source LST and populates whichever content kinds the source supports. A book with no archetype LST does not get an `archetypes/` directory; the absence is honest, not a missed cycle.

### 1.4 In-scope book content ceilings (per SD-26 `decisions.md §11.4` precedent)

For SD-27's per-book cycles, the per-field completion ceiling follows SD-26's precedent. The 4 in-scope books (CRB, APG, ACG, Bestiary 1) have known ceilings; the 2 in-scope future-state books (ARG, PU) have unknown ceilings until the E3.x parity cycle re-verifies against the corpus directly.

**These are SD-26-measured *field-coverage* percentages — a different metric from anything `v06_work_inventory` produces**, so the generator neither confirms nor contradicts them. They are kept here because they are a recorded measurement with a bundle of record, not a corpus figure. Re-measure rather than re-quote before asserting one in a cycle.

| Book (per SD-26's measured ceiling) | Real completion ceiling (per SD-26 E4) |
|---|---|
| core_rulebook | equipment `description` 67.9% — genuine corpus ceiling, not "look harder" |
| advanced_players_guide | equipment `description` 97.9%; spell `full_text` 95.6% |
| advanced_class_guide | not touched by SD-25's pass — verify real ceiling independently |
| beastiary (Bestiary 1) | equipment 4/4 (100%) — real record count is 4 |

> **Internal inconsistency, flagged 2026-07-30, not resolved here.** The ACG row above says "not touched by SD-25's pass — verify real ceiling independently", while `technical-design.md:41` states a measured `ACG equipment 98.1%` alongside the other three books' figures. One of the two is wrong. The operator or the E2.0.8 cycle resolves it; whoever does should record which SD-26 receipt the 98.1% came from, or delete it.

The Bestiary 1 row's record count **is** corroborated by the generator: `books[bestiary].kinds.equipment.units` = 4. Same number, independently derived.

For ARG + AG: the per-cycle assertion is "match rate at the time of cycle close," not a fixed percentage. Inherited CG-03 baseline (7-of-9 ceiling) is documented in each parity-cycle receipt.

## 2. Book stub manifest (Epic 4 — closure epilogue updates)

### 2.1 Per-book stub manifest shape

| Output | Path | Format |
|---|---|---|
| Per-book stub manifest | `data/stubs/<book_id>.json` | `{book_id, book_name, planned_resolution_bundle, content_kind_counts, registered_at, resolved_at, resolved_by, bundle_of_record, cycle_receipt}` |
| Stubs Registry entry | `docs/governance/wired-integration-stubs-registry.md` | `book_stub` kind (existing since SD-26 E4.1) |

For the 2 in-scope future-state books (ARG, PU), the cycle updates the stub manifest's `content_kind_counts` from `null` to a real number map and flips the registry entry's `Status` from "Registered stub" to "Resolved" with `resolved_at: <ISO-8601>`, `resolved_by: claude-code`, `bundle_of_record: SD-27`, `cycle_receipt: artifacts/epic_2/<book>_pre_build-cycle_receipt.md`.

For the 17 deferred future-state books, the stub manifests stay at `content_kind_counts: null` until SD-28+ lands.

For the 2 removed-from-scope books (Beginner Box, Core Essentials), their registry slots (#0005 and #0012) and stub manifests, if they exist on disk, are out-of-scope and may be deleted by the closure epilogue with operator authorization.

### 2.2 Books in SD-27's stub-manifest payload (19 stubs in scope)

advanced_race_guide, adventurers_guide, bestiary_2, bestiary_3, bestiary_4, bestiary_5, bestiary_6, bonus_bestiary, horror_adventures, monster_codex, mythic_adventures, occult_adventures, pathfinder_unchained, ultimate_campaign, ultimate_combat, ultimate_equipment, ultimate_intrigue, ultimate_magic, ultimate_wilderness.

> **Corroborated by the generator, 2026-07-30.** `v06_work_inventory` classifies each corpus directory independently of this document and reports **exactly these 19** at `scope: "future_state"`. The book list above and the generator's are the same set, name for name. This is the one place in this file where a hand-maintained figure was checked against the tool and came back clean.

**2 in scope for SD-27's closure update (E4.x):** advanced_race_guide, pathfinder_unchained.
**17 deferred to SD-28+:** the remaining 17.
**2 removed from scope:** beginner_box, core_essentials (per operator directive 2026-07-27).

Registry entries map 1:1 with `data/stubs/<book>.json` files:
- #0003 advanced_race_guide, #0004 adventurers_guide, #0005 beginner_box (out-of-scope; may be deleted), #0006 bestiary_2, #0007 bestiary_3, #0008 bestiary_4, #0009 bestiary_5, #0010 bestiary_6, #0011 bonus_bestiary, #0012 core_essentials (out-of-scope; may be deleted), #0013 horror_adventures, #0014 monster_codex, #0015 mythic_adventures, #0016 occult_adventures, #0017 pathfinder_unchained, #0018 ultimate_campaign, #0019 ultimate_combat, #0020 ultimate_equipment, #0021 ultimate_intrigue, #0022 ultimate_magic, #0023 ultimate_wilderness.

After the 2 removals (registry slots #0005 and #0012), the surviving 19 registry entries are #0003-#0004 + #0006-#0021 (with #0005 and #0012 deleted; #0013-#0021 shift down by 1, OR the slots are gap-leaved for audit-trail integrity). The closure epilogue resolves the deletion-vs-gap decision per operator authorization.

## 3. Oracle-harness comparator (Epic 3 — parity baseline)

### 3.1 Per-content-unit

| Component | Path | Source canonical |
|---|---|---|
| `comparator.rs` | `src/oracle_validation/comparator.rs` | Reads from `selected_parity_dimensions.rs` |
| `normalization.rs` | `src/oracle_validation/normalization.rs` | Rules from `pcgen-run-character.sh` outputs |
| `parity_report.rs` | `src/oracle_validation/parity_report.rs` | Per-case report |
| `pcgen_runner.rs` | `src/oracle_validation/pcgen_runner.rs` | Wraps `scripts/pcgen-run-character.sh` |
| Per-book pilot case | `tests/fixtures/oracle_validation/pf_<book>_human_<class>_level1_golden.pcg` (hand-authored, mirrors SD-26 pilot Fighter pattern) | Per-book parity baseline write target |

Per-book fixture authoring (E3.x) for the 2 in-scope future-state books:
- E3.1: `pf_advanced_race_guide_human_<class>_level1_golden.pcg`
- E3.2: `pf_pathfinder_unchained_human_<class>_level1_golden.pcg`

### 3.2 Comparator output (per-book parity baseline)

| Output | Path | Format |
|---|---|---|
| Per-book PCGen XML output | `‹tmp›/pf_<book>_<class>.xml` | PCGen Gradle headless run output |
| Per-book normalized baseline | `data/corpus/<book>/_parity/pf_<book>_human_<class>_level1.json` | Normalized PCGen output, comparable to Codex's receipt |
| Per-book parity-cycle receipt | `artifacts/epic_3/<book>_parity-cycle_receipt.md` | Per-dimension match/mismatch table; CG-03 inherited baseline documented |

The 7-of-9 baseline (CG-03 inherited) is the worst-case ceiling; the per-book assertion is "match rate at the time of cycle close," not "9-of-9 fully oracle-checked." Documented in `forward-scope-register.md §"Class 0.3"`.

## 4. Per-cycle repo tooling

Corpus figures are generated (§0); the surrounding cycle uses the repo's
ingestion tooling rather than a bundle-specific process. Full procedure:
`docs/governance/book-ingestion-playbook.md`.

| Step | Command | When |
|---|---|---|
| Book shape | `cargo run --locked --bin v06_work_inventory` | Before planning a book's cycles |
| Pre-ingest trap report | `cargo run --locked --bin v06_corpus_trap_report -- <book_dir>` | Before writing any ingest code for that book |
| Verification | `./scripts/verify.sh` | Every cycle, before commit |
| Reach gate | `./scripts/verify.sh --only reach` | Definition-of-done condition, where a cycle writes `src/rules_core/rules_tables/<book>/` |
| Citation audit | `cargo run --locked --bin v06_corpus_trap_report -- --audit` | Definition-of-done condition |

`--audit` is the one directly aimed at this bundle's payload: it reads the JSON
caches under `data/corpus/` and cross-checks each record's citation against the
corpus line it claims, exiting `2` on a contradiction. That is a stronger and
cheaper check than the Shape B key-set/key-order conformance tests alone, and it
is the natural verification for the per-book cache cycles (E2.1-2.2) and the
license retro-fits (E2.0.6-2.0.9).

## 5. Figure reconciliation against the generator (2026-07-30)

Every corpus figure in this file was checked against `docs/work-inventory.json`.
Recorded here rather than silently corrected, per §0.

| Figure in this document | Generator | Verdict |
|---|---|---|
| §2.2 "19 future-state books" | 19 books at `scope: "future_state"`, same names | **Match.** No change. |
| §1.4 Bestiary 1 equipment = 4 records | `books[bestiary].kinds.equipment.units` = 4 | **Match.** No change. |
| §1.1 / §2.2 beginner_box removed from scope | `scope: "out_of_scope"` | **Consistent.** No change. |
| §1.1 / §2.2 core_essentials removed from scope | `scope: "shared_library"`, with `included_by` naming nine books | **Divergent classification — flagged, see below.** |
| §1.3 nine canonical content kinds | ten generator unit kinds, not a 1:1 map | **Not reconciled in this merge** — the planning-tree copy proposes replacing §1.3's bullet list with a mapping table; that is a rewrite of existing content, not an addition, and was left out of this merge for separate operator review. |
| §1.4 ACG ceiling | not a generator metric | **Internal inconsistency inside SD-27** — flagged in §1.4. |
| "all 23 books" (elsewhere in this package) | 25 corpus directories: 19 future-state + 4 in-scope + 1 out-of-scope + 1 shared library | **Both defensible; the package uses them inconsistently** — see below. |

### Flag 1 — `core_essentials` is a shared library, not a standalone book

The operator directive of 2026-07-27 removed Beginner Box and Core Essentials
from scope as "redundant to other tomes". The generator agrees about Beginner
Box (`out_of_scope`, `included_by: []`). It classifies Core Essentials
differently: `scope: "shared_library"`, with

```
included_by: [advanced_race_guide, bestiary, bestiary_2, bestiary_3,
              bestiary_4, bestiary_5, bestiary_6, core_rulebook,
              ultimate_wilderness]
```

— i.e. the Core Rulebook's own `.pcc` includes it, as do Bestiary 1 and one of
this bundle's two in-scope books, Advanced Race Guide.

**This is not a contradiction of the directive and is not treated as one.**
"Redundant to other tomes; will not be brought in" is entirely consistent with
"its records arrive through the tomes that include it". The flag exists because
the *reason* differs from Beginner Box's, and the two are currently recorded as
one decision with one rationale. If a future cycle ever reads "removed from
scope" as "no Core Essentials record reaches the engine", that would be wrong.
**The operator resolves whether the directive's wording should distinguish the
two cases; nothing here changes the scope.**

### Flag 2 — "23 books" vs "25 books" is used inconsistently across the package

Both numbers are derivable and both appear:

- **23** = 19 future-state + 4 in-scope. `scope-draft.md:54` states this
  composition explicitly, and `loop-instruction.md` uses 23 throughout the body
  of criterion 2.0.10 (lines 182-210).
- **25** = every corpus directory the generator enumerates, i.e. 23 plus
  Beginner Box and Core Essentials. `loop-instruction.md:80` and the criterion's
  own **heading** at `loop-instruction.md:174` ("All-25-books") use this, and
  `loop-instruction.md:292` spells out "4 in-scope + 2 pre-built + 19 deferred".

So criterion 2.0.10's heading and its body disagree with each other, and
`acceptance-and-verification.md §2.2.10` calls it "All-23-books". **The operator
picks one and it is propagated; this document does not pick.** The generator's
own framing, if it helps: 25 directories total, of which 23 carry a bundle of
record and 2 are excluded by the 2026-07-27 directive — so "23" is the right
number for a *conformance sweep over in-scope content*, and "25" is the right
number for a *corpus enumeration*. The criterion is a conformance sweep.

## 6. Cross-reference

- `./scope-draft.md` §3 — per-book cycle map (19 books, 2 in-scope for SD-27).
- `./scope-draft.md` §4 — file-touch partition; the dual-audit gate is the load-bearing enforcement.
- `./decisions.md` §7 — partition doctrine (no `src/rules_core/pilot_compute.rs`, no `src/oracle_validation/`, no `docs/release/v0.6/`).
- `./decisions.md` §9 — Tier-1 / Tier-2 partition for the 19 future-state books.
- `./decisions.md` §11 — per-cycle tier model (Sonnet default; free/discounted operator-authorized for per-book bodies).
- `./technical-design.md` §2 — Shape B v1 schema application.
- `./technical-design.md` §3 — per-book ingestion pipeline (cycle 2.x abstract).
- `./technical-design.md` §4 — PCGen parity baseline (cycle 3.x abstract).
- `./acceptance-and-verification.md` §2.2-2.4 — per-criterion acceptance + verification commands.
- `./loop-instruction.md` §3 — cycle dispatch (E1.1, E2.0, E2.0.5, E2.x per-book, E3.x per-book, E2.0.10 all-23-verify, E4.x closure).
- `docs/governance/book-ingestion-playbook.md` — the mechanical per-book procedure; §1 shape assessment, §2 trap report, §3 reach, §4 verification, §6 counting discipline.
- `docs/work-inventory.json` + `src/bin/v06_work_inventory.rs` — the generated corpus figures this document now defers to (§0).
- `src/pcgen_import/corpus_traps.rs` + `src/bin/v06_corpus_trap_report.rs` — the trap catalogue and the pre-ingest report.
- `scripts/verify.sh` + `scripts/verify-baselines.env` — the verification contract.
- `apps/desktop/src-tauri/src/reach_gate.rs` — the content-reach gate.