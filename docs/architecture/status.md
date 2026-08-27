# Status

> Scope: what is real, working product surface today across the whole repo, and what is stubbed, partially wired, or deferred — superseding the root README's "Current state" section.
> Last verified: **2026-08-19 against `tranche/11`** (SD-31 wave 17, `SD31-W17-INTEGRATE-001`) for the §"Corpus coverage, corpus-wide — re-derived 2026-08-19 (SD-31 wave 17, integration cycle)" section (the live figures — waves 14/15/16's own sections are kept below for history, unchanged); the 2026-08-18 wave-14 pass for the §"Corpus coverage, corpus-wide — re-derived 2026-08-18" section, the `RuleSetId::Ce` row, and the companion-ceiling row; every other row still carries its 2026-08-13 `tranche/9` verification and is unchanged. Prior full pass: 2026-08-13 against `tranche/9` (SD-29 **real** closure, Epic 11 run 3). The 2026-08-11 pass belonged to a closure the operator rescinded the same day (`SD-29 decisions.md §42`); every figure it wrote has been re-derived here rather than carried. The rows re-derived in full this pass are the `RuleSetId` catalog count, the JSON-corpus-cache count, the monster/companion/race-trait chassis rows, and the whole §"Corpus coverage, corpus-wide" section; every other row carries its prior 2026-08-07/tranche-8 verification and is unchanged by SD-29. **Touched again 2026-08-21 (SD-31 wave 29, integration cycle)**: `RuleSetId` variant count 32→33 (`AdventurersGuide` added, this book's first compiled rule set — see the updated row below); `class_feature_pool_catalog.rs`'s option-pool render catalog now refuses any record carrying a `PREABILITY ... CATEGORY=Archetype` token (Ruling §18, `is_archetype_locked()`) — the `class-field-fix` note two paragraphs below ("its only live consumer was never scoped to 3,047 records") still stands unchanged, this is a narrower, later guard on the SAME consumer, not a reopening of that finding. Every other row is unchanged and still carries its prior verification date.
> **Path correction 2026-08-22** (SD-32 closure epilogue): src/rules_core/pilot_compute.rs cite
> updated to `src/rules_core/pilot_compute/mod.rs` — the module became a directory during SD-31;
> no other content in this doc re-verified.
> **Touched again 2026-08-25 (SD-33 closure epilogue)**: new §"SD-33: `unknown` reaches zero, and
> `docs/work-inventory.json` grows to 49,438 units" section added, and the stale `unknown` **3,547**
> figure in the tranche/9-era corpus-coverage paragraph above is marked superseded (now `0` of
> 49,438) with a pointer to the new section; no other row re-verified this pass.
> Maintenance: pre-PR truth-up cycle per [README.md](./README.md) §Maintenance contract — fires before every PR via the architecture-truth-up skill

## Posture

Codex today is a developer proof-harness and a buildable desktop workbench,
not a finished character-management product. The corpus-ingest pipeline, the
deterministic compute chassis, the boundary contract, and every persistence
store are real, tested, and exercised end to end by `cargo test --locked`
and `npm test`. But character coverage is narrow: **single-class Fighter at
levels 1-3, for any race, is the only path that reaches a fully `Computed`
receipt today** — every other class/level combination returns real
claim-blocking diagnostics from the engine (two `apps/desktop/src-tauri/src/character_hub.rs`
tests prove this jointly:
`compose_character_input_reaches_computed_status_for_supported_fighter_levels_1_to_3`
covers the Fighter levels-1-3 `Computed` half, and
`claim_blocking_diagnostic_ids_match_the_catalogued_support_shape_per_class`
asserts the per-class claim-blocking diagnostic sets for the other classes;
verified directly against both test bodies).
Several desktop-facing actions that look interactive are session-local or
inert by design, named individually below — this is the fail-honest
convention (see [conventions.md](./conventions.md)) applied at the product
level: a stub says so rather than pretending to work.

## Real today

| Area | What works | Where |
|---|---|---|
| Corpus-ingest pipeline | `.pcc`/`.lst` parsing through canonical `SourcePackageContent` projection, six of seven record kinds fully wired | [corpus-ingest.md](./corpus-ingest.md) |
| Pilot compute + boundary contract | `compute_pilot_base_chassis` → `compute_pilot_with_corpus` → `to_pilot_receipt` → `printed_sheet_cell_map`, fail-honest throughout | [rules-engine.md](./rules-engine.md) |
| Per-domain engines | Spellbook (9/9 schools), skill allocation, feat prerequisites (4/4 categories), equipment effects (4/4 categories), damage total, level-up (11/11 classes) | [rules-engine.md](./rules-engine.md) |
| Rule-table catalogs | **Grown past "four" (correction, 2026-08-07):** CRB (full), APG (6/6 classes), ACG (10/10 classes), Bestiary 1 (41 monsters across 8 subsets, plus its own small equipment table), plus Advanced Race Guide and Pathfinder Unchained (SD-27/28 ingest) and a new `ultimate_campaign` (`Uca`) rule set carrying 23 feats (SD28-E13) — seven `RuleSetId` variants total (`src/rules_core/rules_tables/mod.rs`). **Corrected again 2026-08-13 (SD-29 real closure): 30, not 14 and not seven.** The 2026-08-11 figure was written before SD-29's monster, race-trait and companion lanes ran to their ceilings; sixteen further rounds registered sixteen more rule sets. The 30 variants, in declaration order: `Crb Apg Acg Bestiary1 Arg Pu Uca Ui Ue Uw Uc Um Upsi BonusBestiary MonsterCodex Isr Ha Botd1 Botd2 Iswg Ce Isc Isi B5 B6 B2 B3 B4 Isb Isg`. Re-derived with `sed -n '/pub enum RuleSetId/,/^}/p' src/rules_core/rules_tables/mod.rs` | [rules-data-tables.md](./rules-data-tables.md) |
| Monster + monster_ability chassis | **New in SD-29 (Epic 5), and run to its ceiling by the reopened bundle.** The merged `monster`/`monster_ability` kind chassis — rules-table module, generator arm, wire DTO, `CORPUS_KIND_NAMES` entry, reach claims, diagnostic row, and frontend path — was piloted on Bonus Bestiary and then extended over eleven rounds across every monster-bearing book that has a chassis. **`monster` is 1,242 of 1,270 `grounded` (97.8%); `monster_ability` is 1,629 of 3,107 (52.4%).** The lane is `DRY`: its remaining 1,506 raw units carry **0** workable rows (1,406 are orphan `monster_ability` rows, 703 of them in books with no monster row at all; 32 Product Identity; 2 `.COPY=` deltas; the 66 the classifier still calls reachable are 54 cross-table owners, 4 `.MOD`-only overlays, 7 PI residue and 1 negated-PCC-gate row). Re-derive with `python3 scripts/classify_monster_ability_rows.py` and `python3 scripts/screen_pcc_load_gates.py monster monster_ability`. A further **229** rows are mechanism-blocked on the `ABILITY:Internal|AUTOMATIC|` bundle hop, owned forward at `successor-forward-scope-register.md` C1.5 | [rules-data-tables.md](./rules-data-tables.md) §`RuleSetId` |
| Character Hub | Create, load, clone, portrait upload/load/delete, JSON export, recompute — all real engine compute + real persistence | [desktop-app.md](./desktop-app.md) |
| Rule-system adapter seam (hub-of-hubs) | `RuleSystemAdapter` trait is the object-safe seam the Character Hub's mutation commands (`append_to_character`/`recompute_character`/`re_save_character`) dispatch through on a `rule_system_id`: `"pf1"` resolves to the real `Pf1Adapter` (wraps the extracted PF1 free functions); any other id resolves to the governed `StubAdapter`, which reports an honest "not yet implemented" diagnostic — never fabricated data (registered exception 0002 in `docs/governance/wired-integration-stubs-registry.md`) | [desktop-app.md](./desktop-app.md) §"Rule-system adapter seam" |
| Corpus-ingest diagnostic | `corpus_ingest_diagnostic` Tauri command reports the real ingested state (record-kind counts + last-touched git timestamp) of every populated `rules_tables` book, counted from the tables actually compiled into the binary — reachable from the Character Hub landing via the `CorpusIngestDiagnosticPanel`. Sketch-scoped to four fields; SD-26 fans out the full status table | [desktop-app.md](./desktop-app.md) |
| PCGen runner scaffolding | `scripts/pcgen-run-character.sh` drives the real headless PCGen Gradle batch-export; `scripts/pcgen-normalize-output.py` normalizes its XML into the golden-fixture comparison shape. Real, invocable, smoke-tested end-to-end (`tests/pcgen_runner_smoke.rs`), and now wrapped into one Rust call by `oracle_validation::pcgen_runner::run_pcgen_character` (SD-26 Epic 2). The in-crate comparator that consumes its output now exists too — see the oracle-parity comparator row below | [testing.md](./testing.md) |
| Campaign manager (local) | Create/edit/list campaigns and their assets, backed by `CampaignStore` on disk; nonce-based conflict detection with local-wins + preserved-conflict-copy resolution | [persistence.md](./persistence.md) |
| Update eligibility / restore / verify | `is_install_eligible`, `perform_restore_previous`, `verify_relaunch_artifact` — all real, tested Tauri commands | [update-and-feedback.md](./update-and-feedback.md) |
| Feedback composers + browser handoff | Bug/enhancement draft composition, evidence capture/redaction, and the governed GitHub-issue browser handoff | [update-and-feedback.md](./update-and-feedback.md) |
| Release pipeline | Multi-platform publish, dual manifest validation, channel-index push, branch-promotion gates — the machinery is real and has shipped releases; the `test` job's frontend-typecheck step passes cleanly (see [testing.md](./testing.md)) | [release-pipeline.md](./release-pipeline.md) |
| Support-state matrix | 34-row typed truth ledger, read-only bridged to the desktop tester workbench | [support-state-matrix.md](./support-state-matrix.md) |
| IPC bridge liveness | `load_backend_health` returns the real crate version and compile-time git SHA; reaching it at all proves the Tauri bridge is alive | [desktop-app.md](./desktop-app.md) |
| Homebrew authoring workbench | The Guard Stance proof package's validate/persist/preview round trip, read-only bridged to the desktop tester workbench | [homebrew-and-oracle.md](./homebrew-and-oracle.md) |
| Encounter difficulty / party CR compute | `Encounter::new` and `party_challenge_rating` are real, grounded compute — but see the DM Toolkit UI row below | [rules-engine.md](./rules-engine.md) |
| Fighter+Wizard multiclass base-chassis dispatch | `compute_multiclass_base_chassis` grounds BAB/save stacking + per-class named-feature explanations for any Fighter+Wizard split, total level 1-10, deterministically proven at every level and both transition directions (SD-24 Epic 5) — but this grounds the base-chassis layer only, not a full `Computed` receipt end-to-end (see the Class/level compute coverage row below) | [rules-engine.md](./rules-engine.md) §"Multiclass base-chassis dispatch" |
| Repo-resident JSON corpus cache | `data/corpus/<book>/**/*.json` — **26** book directories holding **9,354** JSON files as of 2026-08-13, re-derived with `ls -d data/corpus/*/ | wc -l` and `find data/corpus -name '*.json' | wc -l`. The 2026-08-11 rescinded-closure figure was **seven**; SD-29's three reopened lanes added nineteen book directories through the same `gen_book_cache.rs` writer — no new writer. Largest: core_rulebook 3,485, beastiary 832, bestiary_4 828, bestiary_2 732, advanced_race_guide 651, advanced_players_guide 647, advanced_class_guide 424, bestiary_3 374, ultimate_wilderness 328. The 2026-08-07 six-book text is kept below for its per-book record detail: **six** book directories now, not four: core_rulebook (3326 records), advanced_players_guide (641), advanced_class_guide (423), beastiary (45), plus advanced_race_guide (637 files) and pathfinder_unchained (129 files) added by SD-27/28. Written by **eight** distinct writer binaries/modules (see [rules-data-tables.md](./rules-data-tables.md) §"JSON corpus cache" for the full enumeration); each generator *dumps* the compiled Rust module's runtime state and never re-parses raw LST for values (only for line-number citations). Every writer now runs its output through `rules_core::pi_screening` (a shared 55-term blacklist) and stamps a GE-01 `wiring_class` on every record. Round-trip-tested by `tests/sd26_cache_core_rulebook.rs`/`apg`/`acg`/`beastiary` and `tests/pi_screening_regeneration_round_trip.rs` | [rules-data-tables.md](./rules-data-tables.md) |
| CRB/APG/ACG/Bestiary 1 equipment + spell record ingestion | 100% record coverage (equipment and spells) across all four books; `weight`/`description` fields on every book's `EquipmentTableEntry`, populated toward each book's honest ceiling. SD-25 Epic 7 raised those ceilings via cited web second-source passes: CRB `description` 2021/2977 (67.9%, was 61.2%); APG `description` 331/338 (was 0% — the APG corpus itself carries no `DESC:` token, every value identity-matched from `aonprd.com`/`d20pfsrd.com`); APG spell full-text 284/297 (was 261); Bestiary 1 equipment newly ingested at 4/4 records with full cost/weight/description. Remaining gaps are honest, undispatched residue, not silently accepted (per-book counts asserted exactly by `tests/sd24_equipment_coverage_audit.rs` / `tests/sd24_equipment_field_completion.rs`) | [rules-data-tables.md](./rules-data-tables.md) §"Equipment/spell content completeness" |

## Corpus coverage, corpus-wide (re-derived 2026-08-13 — SD-29 real closure)

SD-29 was the first bundle to derive the *whole* corpus's shape in one pass
rather than book-by-book, so this is the section that states repo-wide
coverage honestly. **Every figure below was re-derived at closure run 3**
from `docs/work-inventory.json` (`generated_at` `2026-08-13T09:33:16Z`) —
none is carried from the 2026-08-11 pass, which belonged to a rescinded
closure and is stale in every row:

```
python3 -c "import json,collections; d=json.load(open('docs/work-inventory.json')); \
a=collections.defaultdict(collections.Counter); \
[a[u['kind']].update([u['status']]) for u in d['units']]; \
[print(k, dict(a[k])) for k in sorted(a)]"
```

**38,540 units across 38 book directories** (37 in scope, 38,521 in-scope
units; `beginner_box`'s 19 units are excluded per
`corpus-work-channels.md §10.2`). By status: `grounded` **4,726**,
`ingested-magnitude` **6,518**, `text-complete` **2,391**, `not-ingested`
**17,209**, `not-started` **4,113**, `unknown` **3,547**,
`deferred-with-reason` **36**. **Superseded (SD-33): `unknown` is now `0`** —
see §"SD-33: `unknown` reaches zero, and `docs/work-inventory.json` grows to
49,438 units" below; every other row in this paragraph belongs to a much
smaller, tranche/9-era population (38,540) and is kept here as history, not
re-derived by SD-33.

Grounded moved **491 → 4,699** across SD-29 — a gain of **4,208**, and
**all 4,208 of it** is the three lanes the operator reopened on 2026-08-11:
`companion` +922 (0 → 922), `monster_ability` +1,612 (17 → 1,629), `monster`
+1,182 (60 → 1,242), `race_trait` +492 (21 → 513). The reopen is the single
largest coverage event in the bundle, and the rescinded closure would have
shipped none of it.

Since SD-29 closed, grounded moved **4,699 → 4,726** (+27): the equipment
consumer-delta probe stopped asking its question of only four hand-listed
compiled tables and now asks it of every key the engine catalog holds
(`probe_equipment_key_universe`, derived from
`equipment_resolver::equipment_catalog_rows()` — 6,395 keys, of which 3,123
had never been examined at all). `equipment` +12 (133 → 145),
`equipment_modifier` +15 (40 → 55). The bar itself — `equipment_key_is_wired`,
"resolves against the real on-disk corpus and produces a non-`None` mechanical
stat effect" — is byte-for-byte unchanged; the same widening also made the
observation **book-scoped**, which is strictly stricter (see
`probe_equipment_effect_wiring`'s `Celestial Shield` note).

Per kind (`grounded` / total):

| Kind | Total | Grounded | Note |
|---|---|---|---|
| `class_feature` | 15,472 | 109 | Tier-3 deferral, out of SD-29 scope (`decisions.md §38.4`); owned by SD-30 |
| `equipment` | 6,227 | 145 | 4,802 `ingested-magnitude` — the deepest proven-path kind |
| `race_trait` | 3,447 | 513 | Lane `DRY`. 2,876 of the remainder is chassis-blocked residue — see below |
| `monster_ability` | 3,107 | 1,629 | Lane `DRY`. 1,406 of the remainder is orphan rows no monster can own |
| `spell` | 2,843 | 0 | 1,260 `ingested-magnitude`, 22 `text-complete` |
| `feat` | 2,610 | 77 | 1,229 `text-complete` |
| `companion` | 1,696 | 922 | Lane `DRY`. Built from nothing in this bundle — see below |
| `equipment_modifier` | 1,580 | 55 | 841 `text-complete` |
| `monster` | 1,270 | 1,242 | Lane `DRY` at 97.8% |
| `class` | 185 | 27 | |
| `race` | 103 | 7 | The 7 `race_tables::race_traits()` rows; the *ingest* chassis models 18 races (below) |

**SD-30 CLOSED, 2026-08-15 (`SD30-E9-F2-001`, `epic-9-closure`).** This table's own `grounded`/status
figures above are unchanged from 2026-08-13 and are **not** re-derived by this closure — SD-30's own
scope (narrowed `decisions.md §51`: Epics 0/1/2/3/7/8/9 only) never touched per-record `grounded`
status; that is SD-31's and SD-32's remit going forward (per-class chassis-sweep ingest, corpus-wide
ingest lanes, race chassis, verdict-path capability — see `SD-30-.../release-notes.md` "What SD-30
handed to SD-31 and SD-32"). SD-30's Epic 0 (instrument-apply) moved the board's **`done`** metric (a
different, stricter verdict combining `wiring_class`+`status`+`kind` via
`pf1e_dashboard_producer.py`'s `doneness_verdict()`, not this table's raw ingest `status`), which this
table does not track: `done` moved corpus-wide **3,464 → 5,837** (re-confirmed at closure, unchanged
since Epic 0's own close — see `SD-30-class-feature-archetype-bundle/state-goals-and-lessons.md §1.1`
for the full per-kind `done` table and its exact re-derivation command). SD-30 also closed its own
PI-screening gate (declared `NAMEISPI`/`DESCISPI` reading, now wired into `class_feature`'s one
production ingest binary and backfilled into the monster/companion transcribers —
`SD-30-.../decisions.md §52-54`), its identifier-cleanup epic, its build-version-numbering epic
(`0.10.0`, green full gate at `4630fec2`), and its bundle code-review epic (three real defects fixed:
a dangling PI-dropped grant reference, a citation-resolution bug in the trap-audit self-check, and the
**same** citation-resolution bug already live in `gen_book_cache.rs`'s generator — which had shipped 3
wrong `wiring_class` stamps into production `inner_sea_gods` monster data, now regenerated correctly;
`v06_corpus_trap_report -- --audit` re-derived clean at closure, `TRAP_AUDIT_EXIT=0`, `259 0
mod-record`). **A full re-pass of this section's own numbers (re-deriving `grounded` per kind, the
three structural-ceiling classifiers below, and the JSON-corpus-cache/`RuleSetId` rows above) remains
owed to whichever pass next changes them** — SD-30's own closure did not touch per-record `grounded`
status, so re-deriving it here would not have reflected this bundle's actual diff; that re-pass is
`SD-31-corpus-closure-grind`'s to perform as it lands the per-record work this section tracks.
(`SD-32-engine-capability-builds` was absorbed into SD-31 and deleted 2026-08-15 —
`SD-31-corpus-closure-grind/decisions.md §2` — after its capability builds were found to be
prerequisites of SD-31's own lanes rather than a parallel package; SD-31 now owns both.)

### The three structural ceilings, each measured by a checked-in classifier

A remainder is not a workload. Each of the three lanes SD-29 ran to its
ceiling has a checked-in row classifier that splits its raw remainder into
workable rows and structurally-unreachable ones, and each classifier is the
citation for the split — not a receipt's prose.

- **Race chassis is the `race_trait` ceiling.** `scripts/race_trait_ceiling.py`
  derives a ceiling of **833** rows (815 `TYPE:<Race> Racial Trait` + 18
  `TYPE:<Race> Subrace` heritage selectors) over the **34** races the ingest
  chassis models (`src/bin/ingest_race_traits.rs`'s
  `IN_SCOPE_RACES: [&str; 34]`, asserted by that file's own test) —
  re-derived 2026-08-20 (SD-31 wave-21) after correcting the ceiling
  script's own `IN_SCOPE_RACES`, which had drifted stale at 18 races across
  three later widenings (18→24→30→34) that never touched this Python copy;
  it now reads the Rust declaration at import time instead of
  re-transcribing it. Of the 833-row ceiling, the script's own join against
  `docs/work-inventory.json` by `(book, source_file, source_line)` matches
  only **439** to a recorded board unit — the other 394 ceiling rows are all
  `core_essentials`, a book with **zero** units of any kind in
  `docs/work-inventory.json` (Decision 9 already rules it out of the board's
  scope; corrected 2026-08-20 wave-21 integration, which found the prior
  wording's "open, unresolved join gap" framing was refuted by the script's
  own per-book breakdown). So **3,065** of the 3,504 `race_trait` units are
  chassis-blocked residue that no race-trait ingest can ever ground
  (3,504 − 439, not 3,504 − 833: the 394 `core_essentials` ceiling rows were
  never members of the 3,504 `race_trait` board population and cannot be
  subtracted from it). Among the 439 matched units: 224 `grounded`, 119
  `ingested-magnitude`, 74 `not-ingested`, 14 `text-complete`, 8
  `literal-verified`. The engine's
  separate `crb::race_tables::race_traits()` still models 7 races — that is
  the *compute* surface, not the ingest surface, and the two must not be
  conflated as the 2026-08-11 pass did.
- **Orphan ability rows are the `monster_ability` ceiling.**
  `scripts/classify_monster_ability_rows.py` splits the 1,506 remaining
  `monster` + `monster_ability` units into 1,406 orphan rows (703 of them in
  ten books that carry no monster row at all, so nothing can ever own them),
  32 Product Identity rows, 2 `.COPY=` delta rows, and a 66-row "reachable"
  remainder that is itself entirely non-workable on inspection: 54
  cross-table owners, 4 `.MOD`-only overlays (`origin: mod_only` in the work
  inventory), 7 PI residue, 1 row behind a negated `PRECAMPAIGN` gate that
  `scripts/screen_pcc_load_gates.py` proves PCGen would not load.
- **`ASPECT:` is the `companion` ceiling.**
  `scripts/classify_companion_rows.py` leaves exactly **1** reachable-and-
  remaining row corpus-wide (`Pseudodragon ~ Tail`), and that row needs an
  `ASPECT:` chassis no table in this program models. (That row was cited as
  `core_essentials` until 2026-08-18; `SD31-CE-COMPANION-001` re-filed the
  whole `core_essentials` companion population under the books their own
  `.lst` `SOURCELONG:` headers name — see the SD-31 wave-14 section below.)

**Companion went 0 → 922 grounded inside SD-29.** The lane did not exist at
the 2026-08-11 rescinded closure — that document recorded it as "never
started". Nine rounds later it carries a real chassis
(`src/rules_core/rules_tables/companion_chassis.rs`), a served catalog
(`companion_catalog.rs` + `CompanionCatalogScreen.tsx`), and per-book
companion data across seventeen books.

**The 229 mechanism-blocked monster rows are owned, not orphaned.** The
`ABILITY:Internal|AUTOMATIC|` bundle-ownership hop is scanned, counted and
checked in (`scripts/scan_monster_ability_bundle_rows.py`) and routed to
`SD-29 successor-forward-scope-register.md` C1.5 with a named owner. It is a
ceiling correction — following the hop widens an ownership pass and changes
what every registered book ships — not a backlog line.


## Corpus coverage, corpus-wide — re-derived 2026-08-18 (SD-31 wave 14, `SD31-W14-INTEGRATE-001`)

The section above is SD-29's closure snapshot and is kept for its history. **These are the live
figures**, re-derived at this wave's integration tip on `tranche/11`, never transcribed from a
lane receipt. Oracle pin `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.

| quantity | value | how |
|---|---:|---|
| book directories in the JSON corpus cache | **34** | `ls -d data/corpus/*/ \| wc -l` |
| JSON records on disk | **26,773** | `find data/corpus -name '*.json' \| wc -l` |
| `RuleSetId` variants | **33** | `sed -n '/pub enum RuleSetId/,/^}/p' src/rules_core/rules_tables/mod.rs` — the SD-29-closure list plus `Oa` (Occult Adventures), `Mythic`, and `AdventurersGuide` (SD-31 wave 29 — Adventurer's Guide's first compiled rule set, spell family only; feat/equipment/class_feature-chassis families and 3 sibling books, `inner_sea_magic`/`inner_sea_temples`/`inner_sea_taverns`, still carry no `RuleSetId` at all) |
| board units (in scope, `beginner_box` excluded) | **38,521** | `docs/work-inventory.json`, replayed through `pf1e_dashboard_producer.doneness_verdict()` |
| board `done` | **12,277 (31.8709 %)** | same replay; was 11,829 / 30.7079 % before this wave |
| reachable ceiling | **98.95 %** (38,115 / 38,521) | `python3 scripts/reachability_audit.py` |

By doneness bucket: `done` 12,277 · `held` 1,677 · `in-progress` 1,389 · `not-started` 18,030 ·
`unmeasurable` 5,110 · `deferred` 38.

### What wave 14 changed in the architecture, not just in the counts

* **`core_essentials` is no longer an engine book for companions or races.** `decisions.md §9`
  ruled it is a PCGen packaging bundle, not a Pathfinder book. `SD31-CE-COMPANION-001` deleted
  `rules_tables/core_essentials/` entirely, retired its `CompanionBook` registration, its
  `CompanionBookSpec` in `gen_book_cache.rs`, its `BookSource`, its `corpus_ingest_diagnostic`
  row and its entry in `race_catalog::RACE_CORPUS_BOOKS`, and re-transcribed the 102 companion
  rows into the four books their own `.lst` `SOURCELONG:` headers name — `beastiary1` 59 → 126,
  `ultimate_magic` 32 → 59, `apg` 4 → 17, `crb` 84 → 84. `reach_gate`'s companion claim went from
  102 records reaching no player surface to zero. **`RuleSetId::Ce` survives only as the feat-gap
  host for `ce_feats.lst`'s 15 rows**, and its 128 remaining board units are unattributable from
  the oracle and await an operator ruling (`OPEN-ISSUES.md` row 263).
* **A record's `source` and its `description_source` are now two different claims.**
  `shape_b_v1::CorpusRecordV1` carries an optional `description_source: Option<CorpusSource>`.
  `SD31-E6-F5-005` narrowed 412 already-shipped equipment records whose `source` said
  `web_second_source` — their identity, `cost_gp` and `weight` were always corpus-derived and only
  the description came from the web page — to the pinned oracle's own `.lst` row, moving the web
  citation intact into `description_source` rather than discarding it. That put those records
  inside `corpus_literal_sweep`'s population (`lst_token` + `raw_tokens`) for the first time.
  New module `rules_core::cache_gen::lst_provenance_repair` + `bin/repair_lst_provenance` own
  this; `tests/sd31_lst_provenance_repair_is_durable.rs` pins the population against reversion.
  **Known gap:** neither `cache_gen::apg::generate_equipment` nor
  `gen_core_rulebook_cache::equipment_source` emits the narrowed shape yet, so re-running either
  generator reverts it — re-run `repair_lst_provenance` after any equipment regeneration
  (`OPEN-ISSUES.md` row 264).
* **The `race` kind's doneness verdict now reads the product, not the seven-variant CRB enum.**
  The character-creation chassis predicate moved out of
  `apps/desktop/src-tauri/src/character_hub.rs` into headless
  `src/rules_core/race_creation.rs`, so `v06_work_inventory` calls the SAME function that builds
  the player's race picker instead of testing `crb::race_tables::RaceId::ALL`. `race` went 7 → 34
  `done` and its reachable ceiling is 100 %. A `computed`-wiring-class race additionally requires
  a real `pilot_compute` magnitude consumer, because `computed` + `grounded` maps straight to
  `done` with no second check.
* **The `decisions.md §9` re-attribution widening in `v06_work_inventory::classify` requires the
  unit's own key.** `EngineFacts::holds_unit_by_key` is the strict twin of `holds_unit`; the
  name fallback is a convenience for kinds whose identity really is their display name and is
  wrong as an attribution signal for a `<Group> ~ <Facet>`-keyed row.

## Corpus coverage, corpus-wide — re-derived 2026-08-19 (SD-31 wave 15, `SD31-W15-INTEGRATE-001`)

The section above is wave 14's snapshot and is kept for its history. **These are the live
figures**, replayed at this wave's integration tip on `tranche/11` — never transcribed from a lane
receipt. Oracle pin `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.

| quantity | value | how |
|---|---:|---|
| board units (in scope, `beginner_box` excluded) | **38,521** | `docs/work-inventory.json` (`generated_at` 2026-08-19T14:06:36Z), replayed through `pf1e_dashboard_producer.doneness_verdict()` |
| board `done` | **12,748 (33.0936 %)** | same replay; was 12,277 / 31.8709 % before this wave |
| verification stamps | **8,052** (6,436 literal-verified + 1,616 fixture-verified) | `Counter(u['status'] …)` over the same document; was 7,629 |
| `derived` fixture coverage | **1,699 units cleared over 2,364 fixture rows**, 0 failed, 0 not ingested | `cargo run --locked --bin derived_evaluator_fixture_check` |
| corpus literal sweep | 26,105 records examined, **0 findings** | `cargo run --locked --bin corpus_literal_sweep` |
| reachable ceiling | **98.95 %** | `python3 scripts/reachability_audit.py` |

By doneness bucket: `done` 12,748 · `held` 1,254 · `in-progress` 1,342 · `not-started` 18,030 ·
`unmeasurable` 5,109 · `deferred` 38. The denominator did not move: 38,521 in, 38,521 out, zero
unit ids added or removed, **zero demotions**.

### What wave 15 changed in the architecture, not just in the counts

* **The `derived` done rung tripled its reach: three new evaluator seams.** See
  [testing.md](./testing.md) §"The derived-evaluator fixture seam" for the full contract.
  `monster_sla` (spell-like-ability save DC run BACKWARDS over PF1's Universal Monster Rule to
  derive the granted spell's LEVEL), `monster_ability` (the same rule's `10 + ½ racial HD` base,
  cross-checked against the owner's own `MONSTERCLASS:` row in a different file), and `companion`
  (PF1 CRB p.182's single-natural-attack 1½× Strength rule, encoded by PCGen as
  `max(0,(STR/2))`). Eight families now; a unit failing any seam is removed from `cleared`.
* **New chassis fields, all carrying the corpus token VERBATIM rather than a computed number.**
  `monster_chassis::MonsterStatBlock.spell_like_abilities` (with `save_dc_token`),
  `companion_chassis::NaturalAttackDamageBonus` on `CompanionRecord`, and
  `race_creation::RaceCreationChassis.ability_adjustments_source_trait_key`.
* **A consumer can now NAME the record whose magnitude it read.** That last field is the pattern:
  `race_creation_chassis` returns the key of the `~ Ability Scores` row it actually used, so
  `v06_work_inventory`'s probe credits THAT record and nothing else. It is the record-level
  precedent for narrowing the remaining coarse race-level credits (`OPEN-ISSUES.md` row 272).
* **Two player surfaces render a derived formula, and refuse to resolve it.**
  `MonsterCatalogScreen` prints `3/day — blade barrier (6th, DC 16 + Cha)` — the DC ships as the
  FORMULA, never a resolved number, because a monster's ability MODIFIER is not a corpus fact in
  this repo; that anti-fabrication rule is pinned as a test. `CompanionCatalogScreen` prints
  `Bite +1/2 Str modifier (minimum +0)`, and prints an uninterpretable or `PRE`-gated token
  verbatim with `(formula not interpreted)` rather than guessing.
* **`probe_equipment_effect_wiring` no longer iterates a hand-maintained book list.** It reads
  `data/corpus/*/equipment/` directly (`equipment_probe_book_dirs()`). Thirteen books carrying 903
  real cited equipment records had never been opened by it — the THIRD recurrence of
  `OPEN-ISSUES.md` row 12's shape. Widening what is ASKED, never what counts as an answer:
  `equipment_key_is_wired` is untouched and `mythic_adventures`, the largest newly-opened book at
  252 catalog keys, still observes ZERO.
* **`classify()`'s equipment arm consults the consumer-delta probe ABOVE the `text-complete`
  rung**, so a unit whose computed delta was actually observed is no longer reported on strictly
  weaker evidence.
* **`wiring_class_reason` is a LEXICOGRAPHIC tie-break and must never be used as a filter.**
  `wiring_class.rs:1290` takes `sigs.iter().filter(…).min()` over a `BTreeSet`, so a unit carrying
  both `derived:prose_expr` and `derived:range_keyword` can never REPORT `range_keyword`
  ("p" < "r") however plainly its own row reads `RANGE:Close`. 151 spell units were invisible to
  their own generator on that alphabetical accident alone.
* **Published feeds are checkout-independent.** `pf1e_dashboard_producer.publishable_document_path()`
  records `unit_index.source_document` relative to the enclosing git checkout. It previously held
  the absolute path of whichever tree published, which made `verify.sh`'s `site-dashboard-check`
  passable only from that one tree and committed a home directory into the Cloudflare-published
  `site/` (`OPEN-ISSUES.md` row 285).

## Corpus coverage, corpus-wide — re-derived 2026-08-19 (SD-31 wave 16, integration cycle)

The section above is wave 15's snapshot and is kept for its history. **These are the live
figures**, re-derived at this wave's integration tip on `tranche/11` after merging all six wave-16
lanes, building the merged tree, and re-running the guarded regen pipeline — never transcribed from
a lane receipt. Oracle pin `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.

| quantity | value | how |
|---|---:|---|
| board units (in scope, `beginner_box` excluded) | **38,372** (down 149 from 38,521) | `docs/work-inventory.json`, replayed through `pf1e_dashboard_producer.doneness_verdict()` |
| board `done` | **12,864 (33.5244 %)** | same replay; was 12,748 / 33.0936 % before this wave |
| verification stamps | **8,103** (6,436 literal-verified + 1,667 fixture-verified) | `Counter(u['status'] …)` over the same document; was 8,052 |
| `derived` fixture coverage | **1,750 units cleared over 2,504 fixture rows**, 0 failed, 0 not ingested | `cargo run --locked --bin derived_evaluator_fixture_check`; was 1,699 / 2,364 |
| corpus literal sweep | 26,105 records examined, **0 findings** | `cargo run --locked --bin corpus_literal_sweep` |
| reachable ceiling | **98.94 %** | `python3 scripts/reachability_audit.py`; was 98.95 % (denominator moved) |

By doneness bucket: `done` 12,864 · `held` 1,204 · `in-progress` 1,280 · `not-started` 17,910 ·
`unmeasurable` 5,076 · `deferred` 38.

**The denominator moved for the first time since it was frozen, and by exactly two named causes —
never blurred together:**

* **-116, operator ruling §16 (`decisions.md §9`/§16, `artifacts/OPERATOR-RULINGS-2026-08-19.md`):
  `core_essentials` residuals not found in print are deleted, not flagged.** 128 residual units
  split: 12 `race_trait` re-attributed to `ultimate_wilderness` (Ghoran's own native declaration in
  `uw_races.lst`), and 116 deleted outright as hallucinations until they appear in print (`main`'s
  classify loop, `is_core_essentials_residual`) — the file's 23 pre-directive rows and 6
  `SOURCELONG:Universal Rules` rows in `ce_abilities_race.lst`, Ghoran's own duplicate `race`-kind
  chassis row (withheld rather than re-attributed, since `ultimate_wilderness:race:ghoran` already
  exists natively — re-attributing would have minted a second unit for one game object), and 86
  units across the 7 remaining races with no race declaration in any book but `core_essentials`
  (Android, Aquatic Elf, Gathlain, Lashunta, Monkey Goblin, Syrinx, Triaxian). **`core_essentials`
  no longer appears as a key in `docs/work-inventory.json`'s `books` map at all — `decisions.md
  §9`'s condition is discharged.** None of the 128 was `done`-capable (128/128 `not-ingested`), so
  zero credit moved. A production-path ceiling (`CORE_ESSENTIALS_RESIDUAL_DELETION_CEILING = 117`,
  asserted every real run of `v06_work_inventory`, not just `cargo test`) was added this cycle after
  adversarial review found the original predicate unbounded — see below.
* **-33, operator ruling §17: confirmed duplicate-chooser display names removed.** Case-by-case
  confirmation over the 787-unit chooser-facet population (not the 180-unit heuristic upper bound)
  found real pairs in exactly one shape — the Sorcerer/Bloodrager bloodline chooser+feature idiom —
  all 33 `class_feature`, none `companion` or `race_trait`. None was `done`-capable, so zero credit
  moved. Landed as a bounded, evidenced id list (`DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS`) with a
  hard `exit(1)` drift guard, never a live heuristic filter.

**Doneness movement is unrelated to the denominator change and moved on three independent seams,
summing to the full +116 `done`:**

* **+62 `in-progress`→`done` and +3 `unmeasurable`→`done` (equipment_modifier), +1
  `unmeasurable`→`held`.** `arms_armor.rs`'s `armor_class_bonus` recognition widened to
  `TYPE=ArmorEnhancement`/`ShieldEnhancement`, plus an `EQMARMOR`-chain fallback for
  `max_dex`/`spell_failure`/`armor_check_penalty` consulted only when the bare token is absent —
  masterwork/material/magic armor-enhancement records the probe previously could not resolve
  standalone.
* **+45 `held`→`done` (companion).** A new `BONUS:SKILL|Climb,Swim|DEX-STR` evaluator seam
  (`companion_skill_entries`, 134 fixture rows) — full chassis field, transcriber, Rust
  parse/evaluate/format, bar-check, DTO and `CompanionCatalogScreen.tsx` render. Adversarial review
  found the bar check compared arithmetic only, never which two abilities the shipped record named
  (a mutated `DEX-STR`→`CHA-INT` left the gate green) — fixed this integration cycle:
  `CompanionSkillFixture` now carries `plus_ability`/`minus_ability` and the bar check fails if the
  evaluator's parsed abilities disagree with the fixture's pinned pair.
* **+6 `held`→`done` (monster_ability).** A second save-DC evaluator sub-seam
  (`monster_ability_formula_entries`, 6 fixture rows) for rows stating the FULL Universal Monster
  Rule formula (`10+(HD/2)+<STAT>`) rather than a summed literal — reuses (and mutation-proves it
  reuses) the same `universal_monster_rule_save_dc_base` the flat sub-seam already gates.

### What wave 16 changed in the architecture, not just in the counts

* **`is_core_essentials_residual`'s production path is now bounded, not just tested.** Wave-16
  adversarial review mutated the predicate to `book.starts_with("core")` and found the full test
  suite stayed green while the mutated `main()` would silently delete every `core_rulebook` unit
  (5,223) on a real run — the pinned-baseline test only ever walks `core_essentials`'s own book
  directory, so it cannot see an over-broad predicate pulling in units enumerated from elsewhere.
  `CORE_ESSENTIALS_RESIDUAL_DELETION_CEILING` (117, matching the existing test's own pin) is now
  asserted in `main()` itself against every real regen, closing the hole between "tests pass" and
  "a production run is safe."
* **The `companion` skill-ability-diff bar check now verifies WHICH abilities, not just the
  arithmetic.** See the movement note above — this was a real gate-vacuity finding (Decision 1(a):
  "a gate that cannot fail is worse than no gate"), fixed in the same cycle it was found rather than
  logged for later.
* **Two fabricated/inaccurate citations in `decisions.md`'s §17 execution record were corrected**
  (a quoted "§17 itself anticipated" sentence that appears nowhere in the actual ruling text, and a
  "within 3 lines" claim contradicted by the section's own worked example four lines apart) — the
  underlying 33-unit conclusion was unaffected in both cases; only the cited authority was wrong.
* **`docs/work-inventory.json`'s denominator changed for the first time since it was frozen.**
  Every prior wave since the freeze reported "the denominator did not move." This wave's two
  changes are both operator-directed deletions of content ruled never to have existed in scope
  (hallucinated `core_essentials` residuals, duplicate display-name artifacts of a picker+feature
  idiom) — never a cost-based exclusion, which remains forbidden.



## Corpus coverage, corpus-wide — re-derived 2026-08-19 (SD-31 wave 17, integration cycle)

The section above is wave 16's snapshot and is kept for its history. **These are the live
figures**, re-derived at this wave's integration tip on `tranche/11` after merging all five
wave-17 lane branches carrying a commit, building the merged tree (both the root workspace and
`apps/desktop/src-tauri` as a separate crate), and re-running the guarded regen pipeline — never
transcribed from a lane receipt. Oracle pin `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.

| quantity | value | how |
|---|---:|---|
| board units (in scope, `beginner_box` excluded) | **38,372** (unchanged — required this wave) | `docs/work-inventory.json`, replayed through `pf1e_dashboard_producer.doneness_verdict()` |
| board `done` | **12,892 (33.5974 %)** | same replay; was 12,864 / 33.5244 % before this wave |
| verification stamps | **8,130** (6,436 literal-verified + 1,694 fixture-verified) | `Counter(u['status'] …)` over the same document; was 8,103 |
| `derived` fixture coverage | **1,777 units cleared over 2,506 fixture rows**, 0 failed, 0 not ingested | `cargo run --locked --bin derived_evaluator_fixture_check`; was 1,750 / 2,504 |
| corpus literal sweep | 26,105 records examined, **0 findings** | `cargo run --locked --bin corpus_literal_sweep` |
| reachable ceiling | **98.94 %** | `python3 scripts/reachability_audit.py`; unchanged (denominator did not move) |

By doneness bucket: `done` 12,892 · `held` 1,177 · `in-progress` 1,279 · `not-started` 18,711 ·
`unmeasurable` 4,270 · `deferred` 43.

**The denominator did NOT move this wave — 38,372 = 38,372, confirmed at both total and per-kind
level for all 11 kinds.** No lane proposed, and no reviewer found, any content that should leave
the denominator; `core_essentials` remains absent from the `books` map (`decisions.md §9`'s
wave-16 discharge unregressed).

**Doneness moved +28, on three independent seams, corpus-wide unit-id diff confirming exactly 837
units changed status and zero units were added or removed:**

* **+25 `held`→`fixture-verified` (companion).** A new DESC-embedded save-DC formula evaluator
  seam (`<base>[+HD/2]+<ability>`) — the formula lives only in the ability's own `DESC:` argument
  list, with no separate `BONUS:` field, so `render_pcgen_desc`'s formula-blind renderer was
  silently dropping the DC number from player-facing prose before this seam.
* **+2 `grounded`→`fixture-verified` (monster_ability).** The shared owner-monster-row resolver
  (`find_owner_row`) widened to accept a bare-leading-field name when no `KEY:` token exists (two
  of wave 16's three named orphans; the third, `spine_dragon_spines`, stays correctly unresolved —
  a real name-identity mismatch, not a missing-`KEY:` gap).
* **+1 `in-progress`→`grounded` (equipment).** `equipmods.rs` widened to accept the `TOHIT,DAMAGE`
  affected-roll order (the reverse of the canonical `DAMAGE,TOHIT` records) — grounds
  `maul_of_the_titans` off its own real corpus token.

**One proposed movement was refused at merge time, not shipped.** A wave-17 lane also widened
`equipmods.rs` to accept a `WEAPONPROF=TYPE.Natural` qualifier subject (targeting the Amulet of
Mighty Fists family, 5 `equipment_modifier` units). Adversarial review proved live that this
applies the item's natural-attack-scoped bonus to every equipped weapon — `WeaponEnhancementBonus`
carries no field for the scope the corpus token states, and the live consumer
(`damage_total::resolve_weapon_enhancement_modifier`) does not discriminate by weapon type. This
half of the commit was reverted before merge; the 5 units are `held`, not `done`. See
`OPEN-ISSUES.md` row 309 for the full trace and the correct fix (a new scope field plus a
scope-aware consumer, not attempted this cycle).

### What wave 17 changed in the architecture, not just in the counts

* **A magnitude-applying regression was caught before merge, by design.** The equipment_modifier
  finding above is this wave's proof that "build the merged tree, run the guarded regen, gate it,
  and have an adversarial reviewer prove reachability with the real driver" catches exactly the
  class of defect DoD-8 exists to prevent — a plausible-looking widening that is wrong in a way no
  unit test in the lane's own module caught (its own negative-control test could not fail; see
  `OPEN-ISSUES.md` row 309).
* **`class_feature`'s owner-resolution fallback (`class_feature_type_facet_owner_candidates`) now
  recognizes PCGen's plural `"<Class> Class Features"` taxonomy spelling, not just the singular
  form** — closing a gap between a lane's own reported figure (811 units) and what the shipped
  code actually recovered (510) before this wave's merge-time fix. 809 `class_feature` units moved
  `unknown`→`not-ingested`/`deferred-with-reason` as a result — zero done-eligible movement, by the
  architecture's own safety guard (a pool-group name can never equal a class's own name, so this
  fallback can only ever produce `not-ingested`/`deferred-with-reason`, never a false `grounded`).
* **A `monster`-kind seam lane's own "exhaustive, none viable" census of its 253-unit held
  population was found materially miscounted by review** (16 of 236 units in the wrong bucket, a
  second arithmetic error, and an 11-unit previously-unexamined flat-literal-constant
  sub-population) — logged as `OPEN-ISSUES.md` row 310 for the next `monster`-kind cycle to start
  from an accurate count.

## Corpus coverage, corpus-wide — re-derived 2026-08-20 (SD-31 wave 19, integration cycle)

The section above is wave 17's snapshot (wave 18 shipped no corpus-doneness movement recorded
here — its integration cycle's own work, +5 `done` off an `intelligent_item` reachability fix,
predates this refresh and is folded into this wave's "before" baseline below). **These are the
live figures**, re-derived at this wave's integration tip on `tranche/11` after merging the two
wave-19 lane branches carrying a commit (`ultimate_combat`, a docs-only retro-event commit with no
code/data change; `ultimate_wilderness`, a real feature commit), building the merged tree (both
the root workspace and `apps/desktop/src-tauri` as a separate crate), fixing confirmed
adversarial-review findings, and re-running the guarded regen pipeline in a fresh isolated
`CARGO_TARGET_DIR` — never transcribed from a lane receipt. Oracle pin
`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.

| quantity | value | how |
|---|---:|---|
| board units (in scope, `beginner_box` excluded) | **38,372** (unchanged — required this wave) | `docs/work-inventory.json`, replayed through `pf1e_dashboard_producer.doneness_verdict()` |
| board `done` | **12,903 (33.63 %)** | same replay; was 12,897 / 33.61 % before this wave |
| corpus literal sweep | 26,166 records examined, **0 findings** | `cargo run --locked --bin corpus_literal_sweep`; was 26,105 |
| `derived` fixture coverage | **1,777 units cleared over 2,506 fixture rows**, 0 failed, 0 not ingested | `cargo run --locked --bin derived_evaluator_fixture_check`; unchanged |
| reachable ceiling | **98.94 %** | `python3 scripts/reachability_audit.py`; unchanged (denominator did not move) |

By doneness bucket: `done` 12,903 · `held` 1,224 · `in-progress` 1,282 · `not-started` 18,650 ·
`unmeasurable` 4,270 · `deferred` 43 (was `done` 12,897 · `held` 1,177 · `in-progress` 1,274 ·
`not-started` 18,711 · `unmeasurable` 4,270 · `deferred` 43).

**The denominator did NOT move this wave — 38,372 = 38,372.** No lane proposed, and no reviewer
found, any content that should leave the denominator; `core_essentials` remains absent from
`docs/work-inventory.json` (0 units) and from `site/status-data.json`'s book list, both
re-confirmed directly this wave, not carried over from a prior claim.

**All movement (+6 `done`, +47 `held`, +8 `in-progress`, -61 `not-started`) traces to exactly one
cause: `ultimate_wilderness`'s 61 spells, newly reachable this wave.** The wave-19
`ultimate_wilderness` lane wired the book's 61 spells into `spell_resolver::spell_catalog_rows()`
and the desktop Spell Catalog screen but had not yet built a `data/corpus/ultimate_wilderness/
spell/` cache — without one, no `static`/`derived` spell unit could reach the
`literal-verified`/`fixture-verified` `done` rung regardless of data completeness, since
`corpus_literal_sweep` had nothing on disk to byte-compare against. The integration cycle closed
this: added an `ultimate_wilderness` `BookSpec` to `cache_gen::spell_lane_dump`, generated the 61
records for real against the pinned oracle, restored `raw_tokens` via `enrich_spell_raw_tokens.rs`
(widened 9→10 books), and registered the book in `derived_evaluator_fixture_check.rs`'s
`SPELL_CORPUS_BOOK_DIRS`/`spell_book_corpus_dir_for_short_code`. Net: 6 of the book's `static`
spells (`bleed_for_your_master`, `green_caress`, `sea_of_dust`, `signs_of_the_land`,
`vigilant_rest`, `wandering_weather`) reached `literal-verified`/`done`; the remaining 55 reached
`held`/`in-progress` (magnitude exists, no wired consumer has yet computed a matching delta) — an
honest reachability improvement, not a claimed `done`. **Found live while wiring this**:
`inner_sea_gods` had a real, already-`raw_tokens`-enriched spell cache (92 files) but was ALSO
absent from both lookup tables — the identical gap shape, silently serving `duration: null`/
`range: null` for every ISG catalog row with no gate ever firing. Fixed in the same commit and
closed with a new coverage test (`spell_book_corpus_dir_coverage_tests`, mirroring the sibling
`spell_book_slug_for_covers_every_catalog_book`'s shape) so an unmapped book code cannot silently
recur; this fix moved no `done` count (ISG's spells were already reaching `literal-verified` via a
separate path) — it only stopped two previously-null player-facing fields from being served blank.

### What wave 19's integration cycle changed in the architecture, not just in the counts

* **Answered the wave's own thesis, and the answer is a clear negative result: attacking the
  `status == not-ingested` mass directly does NOT out-produce the seam-grinding waves it was
  dispatched to replace.** All six wave-19 lanes (`advanced_class_guide`, `advanced_players_guide`,
  `core_rulebook`, `ultimate_combat`, `ultimate_psionics`, `ultimate_wilderness`) independently
  investigated their book's `not-ingested` population before ingesting anything and found the SAME
  root cause every time: for `class_feature`/`class`/`race_trait`/`companion`/`monster_ability`,
  "not-ingested" measures whether the ENGINE holds a record (an explanation id, a roster
  membership, a modelled chassis), not whether `data/corpus/<book>/**/*.json` exists with real
  prose. In four of the six books, the cited corpus JSON already carries real, non-empty
  description text for the large majority of the "not-ingested" population — ingesting more would
  not move a single unit, because the doneness classifier for these kinds never reads that JSON at
  all (confirmed by direct code trace for `class_feature`'s `has_real_description`, which reads the
  raw `.lst` `DESC:` field via `closure_has_real_description`, never `corpus_json_has_real_
  description` — that fallback is hard-scoped to `equipment`/`spell` only). The wave's dispatched
  yield across the 6 lanes: 0 + 0 + 0 + 0 + 0 + 6 = **6 `done` units**, all from the one lane
  (`ultimate_wilderness`) whose blocker turned out to be a `spell`-kind ingest gap, the one kind
  where corpus JSON genuinely does gate doneness. Compare waves 15-18's seam-grinding yields:
  +471, +116, +28, +5. **Bulk `not-ingested` ingest is not a productive lane shape for
  `class_feature`/`race_trait`/`companion`/`monster_ability`/`class`** — those kinds' remaining
  populations need engine-side wiring (a generic per-class roster mechanism, new class chassis, new
  race registrations, a chooser-interaction primitive for option-pool records), not book onboarding.
  A future wave aimed at `not-ingested` should scope itself to `spell`-kind gaps specifically (the
  one kind this wave confirmed the thesis for) or to a named engine-wiring epic, not another
  book-per-lane ingest sweep. See `progress.md` `SD31-W19-INTEGRATE-001` for the full per-lane
  accounting and `OPEN-ISSUES.md` rows 298-300/320-324 for the supporting corrections and fixes.
* **A live PI-exposure defect was found and fixed in a generator that had shipped clean data by
  accident.** `cache_gen::class_feature`'s generator screened `data.description` for declared/
  blacklisted Product Identity but shipped `data.raw_tokens`' own `DESC` entry completely
  unscreened — reproduced live (a `DESCISPI:Yes`-declared record's full PI prose re-exposed through
  `raw_tokens` on a fresh regen, while `data.description` correctly showed `[redacted PI]`). The
  already-committed corpus file was not affected (an earlier on-disk repair had already fixed it),
  but any future regen of this generator would have re-shipped the leak. Fixed and mutation-proved
  (`OPEN-ISSUES.md` row 323).

## Corpus coverage, corpus-wide — re-derived 2026-08-20 (SD-31 wave 20, integration cycle)

**These supersede the wave-19 snapshot above.** Re-derived at this wave's integration tip on
`tranche/11` after merging four sound-or-partial lane branches (`progression`, `race_trait`,
`monster`/`monster_ability`, `spell`/`feat`) plus one investigation-only lane
(`class_feature` empty-description/no-corpus-record), rejecting one (`roster-engine`, GAMED —
see below), building the merged tree, fixing every confirmed adversarial-review finding reachable
within this cycle's scope, and re-running the guarded regen pipeline in a fresh isolated
`CARGO_TARGET_DIR` — never transcribed from a lane receipt. Oracle pin
`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.

| quantity | value | how |
|---|---:|---|
| board units (in scope, `beginner_box` excluded) | **38,372** (unchanged — required this wave) | `docs/work-inventory.json`, replayed through `pf1e_dashboard_producer.doneness_verdict()` |
| board `done` | **13,002 (33.88 %)** | same replay; was 12,903 / 33.63 % before this wave |
| corpus literal sweep | 26,368 records examined, **0 findings** | `cargo run --locked --bin corpus_literal_sweep`; was 26,166 (+202, monster/monster_ability raw_tokens enrichment) |
| `derived` fixture coverage | **1,821 units cleared over 2,561 fixture rows**, 0 failed, 0 not ingested | `cargo run --locked --bin derived_evaluator_fixture_check`; was 1,777/2,506 (+44 cleared) |
| reachable ceiling | **98.94 %** | `python3 scripts/reachability_audit.py`; unchanged (denominator did not move) |

By doneness bucket: `done` 13,002 · `held` 1,126 · `in-progress` 1,282 · `not-started` 18,649 ·
`unmeasurable` 4,270 · `deferred` 43 (was `done` 12,903 · `held` 1,224 · `in-progress` 1,282 ·
`not-started` 18,650 · `unmeasurable` 4,270 · `deferred` 43). `-98 held + -1 not-started + 99
done = 0` — every unit of movement accounted for, no residue.

**All +99 traces to four named causes, all in the four merged code lanes:**

| kind | done (wave 19) | done (wave 20) | delta | cause |
|---|---:|---:|---:|---|
| `class` | 27 | 28 | **+1** | `progression` lane: `ultimate_combat:class:gunslinger` — `has_supported_class_chassis` never grew a matching arm for UC's three classes despite `compute_uc_class_chassis` shipping real, corpus-verified BAB/save tables since `SD31-E4-F1-002`/`-005`; fixed, plus Gunslinger's own weapon-proficiency table entry. Ninja/Samurai deliberately left unclosed (genuinely ambiguous proficiency corpus shape). |
| `monster` | 973 | 989 | **+16** | `monster`/`monster_ability` lane: `enrich_monster_raw_tokens.rs`'s own copy of `book_dir_of` had drifted from `corpus_literal_sweep`'s (missing the 4-segment Dreamscarred Press branch), silently `CitationMiss`-ing all `ultimate_psionics` records. |
| `monster_ability` | 1,556 | 1,594 | **+38** | Same lane, same root cause plus running the (pre-existing) enrichment tool for the first time over books it had never touched — 168 of the 202 newly-`raw_tokens`-enriched records were a first-run gap, not the `book_dir_of` bug. |
| `spell` | 1,509 | 1,553 | **+44** | `spell`/`feat` lane: the RANGE/DURATION fixture generators' own `WORK_INVENTORY_BOOK_TO_SHORT` dicts were never widened to match wave 19's Rust-side `inner_sea_gods`/`ultimate_wilderness` registration — all 44 are `ultimate_wilderness`; `inner_sea_gods` gained candidacy but contributed 0 (none of its `derived`+`ingested-magnitude` spells carry a matching RANGE keyword or simple CASTERLEVEL-linear DURATION shape today, a real corpus fact). |
| all others | — | — | +0 | `race_trait`, `class_feature`, `equipment`, `equipment_modifier`, `feat`, `companion`, `race` unchanged. |

`core_essentials` re-confirmed absent — **0 units** in `docs/work-inventory.json`, absent from
`site/status-data.json`'s book list, both checked directly this wave.

### The wave's own thesis, answered plainly — a refuted thesis is the most important thing this receipt reports

**Wave 20 was dispatched to generalize `push_pu_class_feature_records` (Pathfinder Unchained's
per-class explanation-emission roster) beyond PU, against a measured pool of 7,505 `class_feature`
units carrying real corpus prose with no `%N` variable.** The `roster-engine` lane built exactly
that generalization, wired it to 19 Core Rulebook base-class records, and its grounding is real —
independently spot-checked by this integration cycle against the pinned oracle, not merely
reviewed on trust: `class_feature.wizard.corpus_record.spells` genuinely cites Wizard's
`PREVARGTEQ:Wizard_CFP_Level,1` progression row, carries real non-empty non-`%N` prose, and reaches
the real `build_pilot_headless_receipt` driver.

**It is GAMED anyway, and NOT merged.** The commit makes the repository assert two contradictory
things about the same explanation ids in the same test run: `tests/sd13_wizard_level1_prepared_
spell_baseline.rs`'s pre-existing, unmodified `wizard_level1_fabricates_no_spell_math` forbids any
explanation id containing `"spell"` outside a narrow whitelist, and the roster lane's own new
explanation id (`class_feature.wizard.corpus_record.spells`) violates it directly. The same shape
regresses `sd13_paladin_level8_progression.rs` and five `sd13_bard_level4..8_progression.rs`
binaries. This integration cycle independently re-confirmed the regression by running the FULL
test suite (not just `--lib`, which is all the lane's own "2143 passed, 0 failed" verification
ever ran) on the lane's own branch: **9 failing integration-test binaries**, against **zero** on
base `5adedce63`. A second, independent confirmation: `sd25_monk_level_up_explanation_filter_
audit`/`..._druid_...` (also pre-existing, unmodified) show 3 of the 19 credited units are
silently dropped from every real `LevelUpPlan`, refuting the lane's own `prose_reaches_player`
claim on a screen it never checked. **0 of the 19 units are banked. The commit is not merged.**
Full reasoning: `OPEN-ISSUES.md` row 330.

**The true ceiling, as the `progression` lane found it (independently verified by this
integration cycle, not merely transcribed):** the emission LOOP `push_pu_class_feature_records`
runs is genuinely class-agnostic — proven by the (rejected) roster lane's own extraction, which
ran unchanged for both PU and CRB. But the DATA it needs (each record's real grant level) is
**not free**: PU's own four `*_features.rs` tables are each a hand-transcribed oracle progression
table, and no other book's `class_feature` corpus record carries a level-grant field at all — that
fact lives only in a separate, never-ingested `.MOD` ability-grant line in the raw `.lst`. Reaching
a new class costs the same oracle-transcription work PU's four classes already paid, not a free
unlock of the 7,505-unit pool. Of that pool, **this wave banked 0** (the only lane that attempted
it was rejected); the `progression` lane separately sized the honest near-term ceiling on the
*adjacent* `class`-kind chassis problem the roster mechanism depends on: of 15,305 not-done
`class_feature` units, 2,194 sit on classes with no chassis at all (a hard floor), 6,503 are
option/choice-pool records no progression table can close (need a "catalog of choices" mechanism),
928 are in a book with no compiled rule set — leaving **~2,396 units** on already-chassis-supported
classes as the genuine near-term target for a future, correctly-reconciled attempt at this same
mechanism. **A future attempt must reconcile the nine anti-fabrication gates deliberately** (get
an explicit ruling on whether a level-1-granted, level-N-still-present class_feature explanation
is definitionally not "fabricated spell math," and widen `is_monk_pillar_id`/the Druid `LevelUpPlan`
filter's id-prefix allowlist) **before** banking any unit, not after.

### What wave 20's integration cycle changed in the architecture, not just in the counts

* **A stale generator doc comment was corrected before it could cost a future cycle real
  engineering time.** `cache_gen::class_feature.rs`'s module doc comment claimed
  `ultimate_psionics` (1,422 units, the single largest sub-bucket of the 2,991-unit
  no-corpus-record population) was blocked because `corpus_literal_sweep::book_dir_of` "hard-
  requires a 5-segment `source.path`" and is shared infrastructure this generator's card may not
  edit. `book_dir_of` has carried an explicit 4-segment Dreamscarred Press branch since a prior,
  unrelated wave-19 fix; 473 `ultimate_psionics` records already ship today under exactly that
  shape with the sweep reporting CLEAN over them. The real, in-scope, low-risk blocker is simply
  that `ultimate_psionics` is absent from the generator's own `BOOK_PRIMARY_FILES` list — not a
  shared cross-kind edit at all. Corrected in `OPEN-ISSUES.md` row 328 rather than left to mislead
  a future cycle into either avoiding a safe fix or (worse) editing shared infrastructure it never
  needed to touch.
* **Two anti-gaming test gaps were closed in the one lane this wave banked credit from
  (`progression`).** `is_supported_uc_single_class`'s level-ceiling check could be mutated to
  accept any level with the full lib suite staying green (a gate that cannot fail, Decision 1(a));
  a new test mirroring the APG/ACG sibling ceiling tests closes it.
  `gunslinger_alone_reaches_computed_status`'s "alone" was unguarded (it never asserted Ninja/
  Samurai do NOT reach `Computed`); the same test now does.
* **The public status feed's per-book unit shards turned out to already be correct for three of
  the four merged code lanes**, because each lane's own committed `site/dashboard/units/*.json`
  auto-merged cleanly (no other lane touched the same book's shard) and already matched what this
  cycle's fresh regen independently reproduced. Only the `class` shard, the cross-book `index.json`,
  and the two aggregate dashboard documents needed this integration cycle's own write.

Grouped by the plane each item lives in. Every row was re-verified directly
against the cited source, not carried over from a sibling doc unchecked.


## Corpus coverage, corpus-wide — re-derived 2026-08-20 (SD-31 wave 21, integration cycle)

**These supersede the wave-20 snapshot above.** Re-derived at this wave's integration tip on
`tranche/11` after merging four SOUND-or-PARTIAL lane branches (`roster-v2` deferral,
`monster`/`monster_ability`, `race_trait`, `equipment_modifier`/`feat`, `spell`/`feat`), rejecting
one (`class_feature-grant-data-ingest`, GAMED — see below), building the merged tree, fixing every
confirmed adversarial-review finding reachable within this cycle's scope, and re-running the
guarded regen pipeline in a fresh isolated `CARGO_TARGET_DIR` — never transcribed from a lane
receipt. Oracle pin `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.

**This wave's own central question, answered first because it is the most important thing this
receipt reports: of the 15,305 not-done `class_feature` records, how many can now have a grant fact
resolved from ingested data, and how many did the roster ground? Zero resolved, zero grounded, that
this integration cycle will bank.** The enabling lane built a real, generic ingest of PCGen's
`ABILITY:...\|PREVARGTEQ:` progression-grant tokens, but adversarial review — independently
reproduced by this integration cycle against the pinned oracle before excluding it (instruction 3),
not accepted on trust — found it discards the tab-field-0 granting class and the `PRECLASS:` gate
variant it does not parse, fabricating a level-1 grant for the 73.4% of records where the real gate
is higher (one spot-checked example: `adventurers_guide`'s `Sigilus ~ Inscribe Sihedron` ships as
class `"Sigilus"` level 1; the oracle's own `ag_abilities_class.lst:882` names the real class
**Magus** at level **7**). Not merged. `class_feature` stays at **134 done of 15,439 (0.8679%)**,
unchanged for the fourth consecutive wave. See `OPEN-ISSUES.md` row 334 and §9 below.

| quantity | value | how |
|---|---:|---|
| board units (in scope, `beginner_box` excluded) | **38,372** (unchanged — required this wave) | `docs/work-inventory.json`, replayed through `pf1e_dashboard_producer.doneness_verdict()` |
| board `done` | **13,174 (34.33 %)** | same replay; was 13,002 / 33.88 % before this wave |
| corpus literal sweep | 26,368 records examined, **0 findings** | `cargo run --locked --bin corpus_literal_sweep`; unchanged (no new corpus record written this wave) |
| `derived` fixture coverage | **1,821 units cleared over 2,561 fixture rows**, 0 failed, 0 not ingested | `cargo run --locked --bin derived_evaluator_fixture_check`; unchanged |
| `core_essentials` | **0 units**, confirmed absent | direct `python3` count over `docs/work-inventory.json`, and absent from `site/status-data.json`'s book list |

By doneness bucket: `done` 13,174 · `held` 1,188 · `in-progress` 1,253 · `not-started` 18,444 ·
`unmeasurable` 4,270 · `deferred` 43 (was `done` 13,002 · `held` 1,126 · `in-progress` 1,282 ·
`not-started` 18,649 · `unmeasurable` 4,270 · `deferred` 43). `-205 not-started -29 in-progress
+172 done +62 held = 0` — every unit of movement accounted for, no residue.

**All +172 traces to two named causes, both in the two SOUND-or-fixed lanes that banked units:**

| kind | done (wave 20) | done (wave 21) | delta | cause |
|---|---:|---:|---:|---|
| `monster_ability` | 1,594 | 1,737 | **+143** | `monster`/`monster_ability` lane: wired the `CATEGORY:Internal` bundle-row ownership hop (a monster row naming its abilities indirectly via an `ABILITY:Internal\|AUTOMATIC\|<bundle_key>` reference) into `transcribe_monster_tables.py` — +208 ability records across 6 books, 0 removed. 97 of the 143 clear the `text-complete` prose bar with real distinct evidence; the other 46 ride the pre-existing `computed`+`grounded` rung 359 baseline units already use (see §9). |
| `spell` | 1,553 | 1,573 | **+20** | `spell`/`feat` lane: widened 8 of 9 per-school spellbook resolvers to fall back to APG's then ACG's own `SPELL_LIST` when CRB's table has no entry, tagging `TableCellRef.rule_set` to the resolving book — the `ForeignBookTable` cross-book guard was proven against real non-CRB data for the first time. |
| `equipment_modifier` | 508 | 516 | **+8** | `equipment_modifier`/`feat` lane: widened the `equipmods` evaluator to read armor-slot `SR:<n>` tokens (Decision 7 REFINED's UNIVERSAL paradigm case) — `special_ability_spell_resistance_{13,15,17,19}_armor` + their `.COPY=` aliases; wired end-to-end through the DTO, TypeScript types and a real Spell Resistance `StatTile` on `CharacterSheet.tsx`'s Defense tab. |
| `equipment` | 5,312 | 5,313 | **+1** | Same lane, same mechanism: `ultimate_equipment:equipment:resplendent_robe_of_the_thespian` carries its own literal `SR:18` token, grounded by the same widened probe (a real magic item, same spillover shape as prior waves' equipment/equipment_modifier crossovers). |
| all others | — | — | +0 | `class`, `class_feature`, `companion`, `feat` (0 `feat` units banked — only `equipment_modifier` this wave), `monster`, `race`, `race_trait` unchanged. |

`race_trait`'s lane (roster-drift fix in `scripts/race_trait_ceiling.py`/`classify_race_trait_rows.py`)
and `roster-v2`'s deferral both independently proved — not merely claimed — zero board movement.

### The wave's own thesis, answered plainly — a refuted thesis is the most important thing this receipt reports

**Wave 21 was dispatched to attack the reason `class_feature` (134 done of 15,439, 0.87%, 60% of
everything still remaining, unmoved for three waves) is stuck: the per-record grant fact — is this
feature granted, and at what class level — does not live in the `class_feature` corpus record at
all, only in a never-ingested `.MOD` ability-grant line in the raw `.lst`.** The `class_feature-
grant-data-ingest` lane built exactly the missing ingest: a real, generic parser reading PCGen's
`ABILITY:<pool>\|AUTOMATIC\|<Class> ~ <Feature>\|PREVARGTEQ:<Var>,<N>` progression tokens, writing
6,252 resolved grant facts across 20 of 21 books. Its correctness proof reproduced all 64 of
Pathfinder Unchained's hand-curated records exactly.

**It is GAMED anyway, and NOT merged.** That proof exercises only the ~1% `CATEGORY=Class`
progression-row shape; 96.5% of what the parser actually shipped comes from a different row shape
(mostly archetype object rows) whose field-0 granting class the parser never reads — it uses the
key's own `~`-split fragment as the class instead, which is frequently an archetype or ability
category name, not a class at all. Worse, the parser recognises only `PREVARGTEQ:`; PCGen's more
common `PRECLASS:1,<Class>=<N>` gate is silently discarded, so 2,098 of 6,252 shipped facts
(33.6%) carry a fabricated level-1 grant where the record's own segment names a real, higher gate.
This integration cycle independently re-derived one of adversarial review's spot checks against the
pinned oracle directly, not on the reviewer's word alone (instruction 3): `adventurers_guide`'s
`Sigilus ~ Inscribe Sihedron` ships as class `"Sigilus"` (the archetype name), level 1,
`level_explicit: false`; `ag_abilities_class.lst:882` — the very row the parser read — carries
`ABILITY:Special Ability\|AUTOMATIC\|Sigilus ~ Inscribe Sihedron\|PRECLASS:1,Magus=7`: the real
granting class is Magus, the real gate is level 7. Mutation-proving the PU reproduction gate
(injecting one fabricated `GrantFact` per Unchained-class key) left all 4 tests and the full
2,149-test lib suite GREEN — the gate cannot fail on exactly the fabrication this data risked.
**Grounded: 0. Resolved-and-trustworthy: 0.** Consuming this data as-is downstream would have
grounded thousands of `class_feature` units at a fabricated grant level — the same manufactured-
credit shape wave 20's `GAMED` roster-engine lane was rejected for.

**The parser is salvageable, not a dead end.** Both missing facts — the true granting class, the
true gate level — are already inside the strings it reads (tab field 0; a `PRECLASS:` arm beside
the `PREVARGTEQ:` arm it already handles). A corrected re-attempt, with a reproduction proof that
actually samples the archetype-row/non-Class-row shape that is 96.5% of the real population (not
just Pathfinder Unchained's own narrow `CATEGORY=Class` shape), is owed to a future wave. Until
then, **wave 20's own narrower, independently-derived ~2,396-unit "immediately groundable by the
engine today" estimate remains the only figure any future attempt should plan against** — not the
7,505 wave 20 hoped for, and not the wider 4,856-unit ceiling this lane proposed, which does not
survive its own data (77.5% of that population's own facts carry an unread-from-any-token level-1
default). Full reasoning: `OPEN-ISSUES.md` row 334.

**The sibling roster lane correctly declined to build on this data and banked 0 units instead** —
verifying, before writing any code, that no non-hand-transcribed grant data source existed in its
worktree this cycle, and logging a `retro.py` deferral rather than falling back to the exact
per-class hand-transcription cost this wave exists to eliminate. Retroactively confirmed the right
call by this rejection (though its own "no sibling has started" claim rested on a branch-tip poll
that could not actually distinguish "not started" from "in progress" — a LOW finding logged, not
disputed as an outcome).

### What wave 21's integration cycle changed in the architecture, not just in the counts

* **A live data-fabrication risk was caught before it could become a downstream grounding
  mechanism's silent input.** Had `class_feature_grants.rs` merged un-reviewed, the next wave's
  roster lane would have had a ready-made, plausible-looking, 6,252-fact data source to build
  against — one that is wrong in the majority of its non-trivial cases. Rejected at the data layer,
  before any consumer was ever wired to it (the sibling `roster-v2` lane deliberately built no
  consumer this wave, so the blast radius was contained to zero board units).
* **`scripts/observer/pf1e_dashboard_producer.py`'s `CODEX_REPO_ROOT` defect (`OPEN-ISSUES.md` row
  325) recurred one wave later and is now fixed at the source**, not merely worked around again:
  `WORK_INVENTORY_FULL_DOC` now falls back to `CODEX_REPO_ROOT`-relative before the hardcoded
  shared-checkout path, so any worktree lane's own dashboard regen publishes its own board rather
  than a concurrently-running sibling's.
* **A 394-unit arithmetic error was caught and corrected in this file itself before it could stand
  as current-state truth**: the wave-21 `race_trait` lane's own commit subtracted `core_essentials`
  ceiling rows (a book with zero board units, Decision 9) from the `race_trait` total as though they
  were board members. Corrected chassis-blocked residue from 2,671 to the real 3,065.
* **A monster_ability classifier-consistency question was surfaced, not silently ridden.** 46 of
  this wave's 143 new `monster_ability` `done` units clear the board's `computed`+`grounded` bar on
  table-membership evidence alone, identical to 359 pre-existing baseline units and 62 correctly-
  `held` siblings in the same batch — a real, long-standing question about whether `grounded`
  requires an actually-observed consumer delta, logged for an explicit ruling (`OPEN-ISSUES.md` row
  335) rather than either quietly banked as-is or unilaterally (and inconsistently) demoted.

## Corpus coverage, corpus-wide — re-derived 2026-08-20 (SD-31 wave 22, integration cycle)

**These supersede the wave-21 snapshot above.** Re-derived at this wave's integration tip on
`tranche/11` after merging five lane branches (`class-feature-grant-parser-rebuild`, `option-pool
class_feature reference catalog`, `monster_ability + monster` investigation, `spell +
equipment_modifier`, `race_trait`), rejecting one (`class_feature anti-fabrication gate
reconciliation`, GAMED — see below), building the merged tree, fixing every confirmed
adversarial-review finding reachable within this cycle's scope, and re-running the guarded regen
pipeline TWICE in a fresh isolated `CARGO_TARGET_DIR` (the first run caught an integrator process
defect — a fix made directly in a worktree but never `git commit`-ed before merging — corrected
before the second, trusted run). Oracle pin `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.

**This wave's own central question, answered first: after THREE prior failed attempts (wave 19's
misdiagnosis, wave 20 GAMED, wave 21 GAMED), does `class_feature` now have a trustworthy grant-fact
source?** NO, not fully — but materially closer than any prior attempt. The wave-22
`class-feature-grant-parser-rebuild` lane was reviewed PARTIAL, not GAMED: it banks 0 board units
(no consumer exists — `pilot_compute`/`wiring_class.rs` both untouched), and this integration cycle
independently spot-checked 8 facts by hand against the pinned oracle across 6 different
actually-shipped books (not just Pathfinder Unchained, the narrow slice the lane's own tests
covered) — all 8 correct, 5 now permanent oracle-gated regression tests. Fixed 4 of 5 reviewer-
confirmed defects before merge (a fabricated `TYPE.PC`-as-a-class defect, a silently-dropped-
negated-gate defect, a many-to-many dedup collapse that lost real multi-class grants, and a stale-
output bug the integrator found while verifying the first fix). **Honest coverage with refusals
counted: 3,483 facts resolved (3,305 with a real corpus record to attach to), 2,969 grant tokens
explicitly REFUSED rather than defaulted.** Two residuals remain unfixed and logged (`OPEN-
ISSUES.md` row 339): a single genuine PCGen oracle typo this parser cannot structurally catch, and
— the consequential one — archetype-conditional grants shipping as unconditional class facts with
NO cross-book conflict detection (confirmed: `Druid ~ Wild Shape` ships contradictory levels 4 and
6 from two different books, both shipped, neither reconciled). `class_feature` moves from **134
done of 15,439 (0.8679%)** to **213 done of 15,439 (1.3796%)** — the option-pool reference-catalog
lane's contribution (below), not the grant parser's; the grant parser itself still bank 0.

| quantity | value | how |
|---|---:|---|
| board units (in scope, `beginner_box` excluded) | **38,372** (unchanged — required this wave) | `docs/work-inventory.json`, replayed through `pf1e_dashboard_producer.doneness_verdict()` |
| board `done` | **13,253 (34.5382 %)** | same replay; was 13,174 / 34.3323 % before this wave |
| corpus literal sweep | 26,368 records examined, **0 findings** | `cargo run --locked --bin corpus_literal_sweep`; unchanged (no lane touched `data/corpus/` this wave) |
| `derived` fixture coverage | **1,821 units cleared over 2,561 fixture rows**, 0 failed, 0 not ingested | `cargo run --locked --bin derived_evaluator_fixture_check`; unchanged |
| `core_essentials` | **0 units**, confirmed absent | direct `python3` count over `docs/work-inventory.json` |

All +79 traces to one cause: the `option-pool class_feature reference catalog` lane built, for ONE
option pool (Rogue Talent, 130 corpus records), a browsable reference catalog serving every clean-
rendering, prose-only member's description on the Character Sheet's Class Features tab — the same
"browsable menu of every real option, shown whether or not selected" shape that already banked
+146 `race_trait` units under Decision 7's REFINED ruling. Adversarial review confirmed 9 of the
lane's originally-banked 88 units carried a real `raw_tokens` engine-effect entry (not prose-only
after all) and withdrew them; the integrator applied that withdrawal before merge, landing the
corrected **+79**. Extending the same mechanism to the other 26 registered option pools is the
next cycle's headline, each gated on its own per-pool corpus spot-check (`OPEN-ISSUES.md` row 340).

**One reclassification, reported separately from doneness movement per standing instruction:** the
`race_trait` lane fixed a hyphen/space matcher defect and reclassified 1 unit's evidence (`race_
not_modelled` → `absent_from_race_traits`, status unchanged) — `doneness_verdict()` keys only on
`(wiring_class, status, kind)`, never `evidence`, so this provably could not and did not move the
board.

### What wave 22's integration cycle changed in the architecture, not just in the counts

* **A real player-facing defect was found and fixed outside the class_feature grind entirely.** 29
  `core_rulebook` name-suffixed spell variants (`Planar Binding (Devils and Fiendish Creatures
  Only)` and 28 siblings) shipped `level: 0` in the live-served spell catalog table — a genuine
  6th-level spell showed "Level 0" on the real Spell Catalog screen and Character Sheet Add Spell
  picker. Fixed by inheriting each variant's own base spell's level; independently re-verified
  against the pinned oracle for all 29. Zero board effect by construction (the table is not corpus
  JSON and is not read by the doneness computation) — a real fix the board cannot see.
* **A GAMED verdict caught false doc-comment prose before it could mislead a future cycle.** The
  `class_feature anti-fabrication gate reconciliation` lane's central claim — that all nine
  anti-fabrication gates named in `OPEN-ISSUES.md` row 330 are already compatible with a generic
  `class_feature.<class>.corpus_record.*` id "with zero weakening" — was refuted on two counts by
  direct execution: the five `sd13_bard_level4..8` gates ARE closed allowlists that regress on any
  new bard-namespaced id (exactly as wave 20 already demonstrated empirically), and the claimed
  "no 0→1 LevelUpPlan transition exists" premise is false — `compute_monk_level_up_grants(&input,
  0, 1)` returns 24 real grants. Not merged; the false prose never reached a shipped file.
  `OPEN-ISSUES.md` row 338 restates the correct facts for the next lane.
* **An integrator process defect was caught by its own consequence, not by inspection.** A fix made
  directly in a worktree (the option-pool 9-unit withdrawal) was never `git commit`-ed before that
  branch was merged into `tranche/11` — invisible until the first guarded regen produced +88
  instead of the reviewer-corrected +79, which is what caught it. Committed and re-merged before
  the trusted regen ran; a reminder that "the branch is merged" and "the fix is in the branch" are
  two different facts that both need checking.

## Corpus coverage, corpus-wide — re-derived 2026-08-20 (SD-31 wave 23, integration cycle)

**These supersede the wave-22 snapshot above.** Re-derived at this wave's integration tip on
`tranche/11` after merging five lane branches (`class-field-fix`, `roster-consume` — the same
physical branch the lane-results list's separate "gate-reconciliation" entry also describes,
confirmed by `git diff --numstat` — `option-pools`, `monster-spell`, `race_trait`), fixing every
confirmed adversarial-review finding reachable within this cycle's scope, and re-running the
guarded regen pipeline in a fresh isolated `CARGO_TARGET_DIR`. Oracle pin
`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.

**This wave's own central question, answered first: did wave 22's grant-fact parser
(`data/class_feature_grants`, 3,483 facts, `NO CONSUMER` at wave 22's close) actually ground
`class_feature` units at scale once wave 23 built its first consumer?** NO — and the reason is
now measured, not suspected. The consumer (`class_feature_grant_consumer.rs`) shipped with a
CRITICAL, player-visible fabrication defect two independent adversarial reviews caught before
merge: a vanilla, no-archetype Rogue's roster carried two archetype-only replacement features
(`careful_disarm`, `poison_use`), because the consumer's only archetype guard checked the grant
KEY's text against the resolved class, and PCGen keys some archetype-replacement features
directly under the base class's own name. Fixed upstream, in the PARSER itself: `class_feature_
grants.rs` now resolves and ships a new `granted_via_archetype` boolean per fact (`true` only for
a `PRECLASS:`-gated token whose own row carries `CATEGORY:Archetype` — a `.MOD`-row-gated fact can
never be archetype-sourced by construction). Measuring with the real signal instead of a manual
read found **3,121 of 3,483 facts (89.6%) are archetype-scoped** — structurally unreachable by any
consumer trusting a grant fact's `class` field alone until `CharacterInput`/`pilot_compute` gains a
real archetype-selection model. What survives every guard (base-class-only, cross-book-conflict-
free, non-pool, non-archetype, across 20 non-excluded classes) is **7 genuinely new units**.
Wave 22's own "134 → 213, first movement after four flat waves" was NOT the grant parser paying
off — it was the option-pool reference-catalog mechanism (a SEPARATE, already-proven mechanism),
landing in the same wave as the (then-unconsumed) parser. This wave's own larger gain (below) is
that SAME reference-catalog mechanism, widened to a second pool — not the grant-fact consumer.

| quantity | value | how |
|---|---:|---|
| board units (in scope, `beginner_box` excluded) | **38,372** (unchanged — required this wave) | `docs/work-inventory.json`, replayed through `pf1e_dashboard_producer.doneness_verdict()` |
| board `done` | **13,422 (34.9786 %)** | same replay; was 13,253 / 34.5382 % before this wave |
| corpus literal sweep | 26,491 records examined, **0 findings** | `cargo run --locked --bin corpus_literal_sweep`; +123 examined over wave 22's 26,368 — this wave's own `raw_tokens` enrichment fix on the 123 new Bestiary-1 `monster_ability` records that could not previously be examined at all |
| `derived` fixture coverage | **1,821 units cleared over 2,561 fixture rows**, 0 failed, 0 not ingested | `cargo run --locked --bin derived_evaluator_fixture_check`; unchanged |
| `core_essentials` | **0 units**, confirmed absent | direct `python3` count over `docs/work-inventory.json` |

All +169 traces to two kinds, four causes, 0 losses. **`class_feature` +116** (213 → 329) = 109
(the `option-pools` lane's Rage Power reference catalog — a second pool on the SAME "browsable
menu of every real option, shown whether or not selected" mechanism wave 22's Rogue Talent
catalog already proved, corrected from the lane's overclaimed +125 after a reviewer found 16 of
the 125 carry a class-level-scaled magnitude modifying a value this engine already computes a
state for) + 7 (the grant-fact consumer's first-ever real credit, corrected DOWN from the
reviewer's own +8 once this cycle's more principled fix caught a THIRD live fabrication the
reviewer's manual list missed — `Summoner ~ Shield Ally` is Master-Summoner-archetype-only).
**`monster_ability` +53** (1,737 → 1,790) = 45 (the `monster-spell` lane's Bestiary-1 cross-table-
owner remedy, `decisions.md §58.3`, giving 55 cross-table ability rows their real owner for the
first time) + 8 (this integration cycle's own `raw_tokens` enrichment fix newly reaching
`literal-verified`). `class-field-fix`: 0 board movement, exactly as predicted — its only live
consumer (`class_feature_pool_catalog.rs`) was never scoped to any of the 3,047 records it
corrected. `race_trait`: 0 movement, a SECOND consecutive wave confirming this exact dispatch
scope (tables/matcher-only) is structurally incapable of moving the board.

**No reclassification this cycle** (reported separately from doneness movement per standing
instruction) — every gained id genuinely transitioned doneness, not merely its evidence string.

### What wave 23's integration cycle changed in the architecture, not just in the counts

* **A trustworthy-but-incomplete anti-fabrication guard was made trustworthy at its actual
  source, not patched around at the consumer.** `key_names_a_base_class_feature` (the guard the
  merged consumer relied on) checks TEXT, not STRUCTURE — PCGen can and does key an archetype's
  own replacement feature under the base class's literal name. The fix moved the signal one layer
  upstream, into the parser that already reads the real oracle row (`granted_via_archetype`),
  rather than trying to out-guess the shape from the consumer side — the "proof narrower than the
  data" failure mode that sank wave 21, caught one layer deeper this time by two independent
  adversarial reviews before merge, not after.
* **A destructive corpus regen was caught before its second-order effect could compound.** The
  `class-field-fix` lane's `true_class_by_key` regen silently overwrote two `class_feature` corpus
  records with a duplicate of an unrelated, same-directory, same-`name` sibling — real content
  loss with no raised error and no failing test. Restored both records, added a permanent
  corpus-wide `(book,key)` uniqueness + `raw_tokens`-floor ratchet
  (`tests/sd31_class_feature_corpus_key_uniqueness.rs`) neither `monster_chassis.rs` nor
  `companion_chassis.rs`'s own precedent covered for `class_feature`, and closed the sibling
  finding that NO test covered `generate()`'s actual use of the corrected class-resolution logic
  (a verbatim revert of the one load-bearing line had left the whole lib suite green).
* **A live-branch test regression reached this integration cycle without ever being reported as
  one.** The merged `roster-consume` branch shipped RED against a pre-existing, base-green
  acceptance gate (`tests/sd24_wired_integration_audit.rs`, the anti-stub-prose audit, tripped on
  the literal word "placeholder" inside an assertion-message string) — caught only by an
  adversarial review re-running the full suite the lane's own submission had described as
  "pending."

## Corpus coverage, corpus-wide — re-derived 2026-08-20 (SD-31 wave 24, integration cycle)

**Wave 24 was shaped differently, on the operator's direct instruction: instead of spreading lanes
across every kind, all lanes served ONE goal — drive a single book, Bestiary 6 (72 units, 26 done at
dispatch), toward closure, to distinguish a WIRING problem from an UNWIRED one.** The full per-unit
ledger is the wave's primary deliverable:
[`SD-31-corpus-closure-grind/artifacts/BESTIARY-6-LEDGER.md`](../release/SD-31-corpus-closure-grind/artifacts/BESTIARY-6-LEDGER.md).
Board: **13,422 → 13,428/38,372 (34.9786% → 34.9942%)**, +6, denominator unchanged. Bestiary 6:
26 → 32/72 (44.4%). Oracle pin unchanged, `7f818006e371188e5717fd18d74d18a420747fc6`.

**The wave's own question, answered plainly: is Bestiary 6's remaining work a WIRING problem or an
UNWIRED problem?** Of the 40 units still not done, **39 are WIRING GAP and 1 is UNWIRED — a 39:1
margin.** Every remaining `class_feature` (18) and `monster_ability` (16) unit has real,
oracle-verified content behind it; none needed a Structural Exclusion Register entry. One lane's
proposal to file all 16 `monster_ability` units `NOT PRESENT` (licensing denominator removal) was
independently reviewed **GAMED** and corrected — the abilities are already referenced by exact
corpus key from this book's own companion chassis data (`companion_data.rs`'s
`external_ability_refs`), which a live, no-new-UI render path already serves for this book's 12
advancement abilities today; what is missing is a `monster_chassis` registration bridging that
existing companion-ability path into the `monster_ability` kind's own classifier arm, not missing
content.

**What actually moved the board this wave did not come from any of the 4 dispatched lanes — it came
from the integration cycle itself, building the one fix every lane was explicitly barred from
touching.** 2 of 4 lanes (`class_feature`, `race_trait`) correctly diagnosed their entire remaining
population as outside their granted file scope and shipped zero code. The `monster_ability` lane
also shipped zero code, with the GAMED misdiagnosis above. The `spell` lane shipped real content
(Bestiary 6 registered as spell-catalog book 11) that broke a live product invariant
(`no_key_is_served_twice_so_a_selection_resolves_unambiguously`, 4 desktop-crate tests) and was
fixed forward in this same cycle by generalizing the existing cross-book-duplicate-suppression
pattern into a resolver-level dedup pass. Two independent adversarial reviews then refuted the
`race_trait` lane's headline diagnosis — that Rougarou (Bestiary 6's sole race) needed an unbuilt
heritage/subrace-selector ingest mechanism, the Dhampir shape — against the pinned oracle directly:
no `*_subrace.lst` file exists for Rougarou, and its `Rougarou_Replace*` flags are DEFINEd to `0`
with nothing anywhere in the corpus ever setting one `True`. Rougarou is a flat, single-tier race,
the identical shape already ingested for Bestiary 2's 6 races and Bestiary 5's Skinwalker. The
integration cycle built the ingest directly: +1 `race`, +5 `race_trait` (Ability Scores, via the
same automatic creation-chassis credit every modelled race's Ability Scores unit already uses;
Change Shape/Languages/Size/Type via Decision 7's text-complete rung), widening the
character-creation roster to 38 races. 3 more `race_trait` units (Speed/Vision/Natural Weapon) and
1 (`Adopted Race ~ Rougarou`, the APG selector row — genuinely UNWIRED, not ingested at all) remain
open, named in the ledger with their general fix.

**Priced for the remaining thirty books**: a lane dispatch bounded to "tables and matchers only, no
chassis work" is close to a guaranteed zero-yield dispatch for `race_trait` and near-zero-yield for
`class_feature`/`monster_ability` in their CURRENT shape. The real levers are a handful of subsystem
widenings that close units across MANY books at once, not more lane-parallelism at this file-scope
grain: (1) the Cleric/Inquisitor domain-power grounding subsystem, allowlisted to Good+Healing only
today; (2) a `monster_chassis` ↔ companion-ability bridge; (3) a `race_ids_with_a_magnitude_
consumer` flat-override seam (speed/vision/natural-weapon) for non-CRB races. All three are named,
with their exact blocking code cited, in the Bestiary 6 ledger.

| quantity | value | how |
|---|---:|---|
| board units (in scope, `beginner_box` excluded) | **38,372** (unchanged — required this wave) | `docs/work-inventory.json`, replayed through `pf1e_dashboard_producer.doneness_verdict()` |
| board `done` | **13,428 (34.9942%)** | same replay; was 13,422 / 34.9786% before this wave |
| corpus literal sweep | 26,500 records examined of 26,934 read, **0 findings** | `cargo run --locked --bin corpus_literal_sweep`, isolated `CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/w24b-regen` |
| `derived` fixture coverage | **1,821 units cleared over 2,561 fixture rows**, 0 failed, 0 not ingested | `cargo run --locked --bin derived_evaluator_fixture_check`; unchanged |
| `core_essentials` | **0 units**, confirmed absent | direct `python3` count over `docs/work-inventory.json` |
| Bestiary 6 | **32/72 (44.4%)**, was 26/72 (36.1%) | `docs/work-inventory.json`, filtered `book==bestiary_6` |
| in-scope races (`ingest_races::IN_SCOPE_RACES`) | **38**, was 37 | Rougarou (Bestiary 6) added this wave |

**All +6 traces to one cause**: the integration cycle's own Rougarou race ingest (1 `race` +
5 `race_trait`, all landing on already-accepted mechanisms — the character-creation roster and
Decision 7's text-complete rung — not a new one). 0 units moved from any of the 4 dispatched lanes.

### What wave 24's integration cycle changed in the architecture, not just in the counts

* **A cross-book verbatim spell reprint exposed a real gap in `spell_catalog_rows()`'s dedup
  discipline, generalized rather than patched around the one collision that found it.** The
  existing "thinner duplicate omitted" pattern (`ultimate_combat::spell_list` already omits `Share
  Language (Communal)` this way, at INGEST time, by hand) only covers a duplicate one lane
  remembers to omit. A later book chaining a spell BOTH books genuinely print in full (Bestiary 6
  and Ultimate Wilderness's shared Scalykind-subdomain spells) had no such omission and served the
  same key twice. Fixed at the resolver level instead: a book-agnostic, first-chained-wins dedup
  pass now runs after every book's rows are chained, protecting every FUTURE book widening from the
  same collision shape, not just this one.
* **A lane's root-cause diagnosis was refuted by re-reading the oracle it claimed to have read
  whole, not by re-running the same instrument that produced it.** The `race_trait` lane's claim
  that Rougarou needed an unbuilt heritage/subrace-selector mechanism traced, on inspection, to an
  unevidenced parenthetical in an earlier `OPEN-ISSUES.md` row that grouped Rougarou with Dhampir's
  REAL heritage gap without independently checking Rougarou's own oracle files. Two independent
  adversarial reviews (a wave-24 lane review and a separate ledger-honesty-lens review) each
  performed that direct check and reached the same refutation — the corrective instinct this
  program keeps re-learning: "read the whole corpus record, not filtered fields" and "shipped prose
  is not a source of truth" apply to a lane's OWN prior findings, not only to corpus content.
* **A book-level `NOT PRESENT` classification was caught before it could license a denominator
  removal it did not earn.** The `monster_ability` lane's instrument (`classify_monster_ability_
  rows.py`) models an ability's owner as `kind=='monster'` only, and printed an absolute claim
  ("nothing can ever own them") in exactly the place it is blind to companion-only ownership — a
  proxy making its confident claim precisely where it is wrong, the identical failure shape this
  program's own standing rule (`validate proxies against known truth`) exists to catch. Caught by
  cross-checking the proxy's claim against the compiled chassis table directly, not by trusting the
  proxy's own framing.

## Corpus coverage, corpus-wide — re-derived 2026-08-21 (SD-31 wave 26, integration cycle)

**Waves 25/25b built and proved `formula_interpreter.rs` (a real recursive-descent PCGen `BONUS:`/
`DEFINE:` formula evaluator, `SD-27 decisions.md §24.1`'s "no formula interpreter" ban overturned for
this package only, `OPERATOR-RULINGS-2026-08-21.md §20`) but wired zero production consumers — the
whole 22/22 hand-modelled-function reproduction proof banked 0 board units by design. Wave 26's job
was to plug it in.** Board: **13,443 → 13,444/38,372 (35.0334% → 35.0360%)**, **+1**, denominator
unchanged. Traced to exactly ONE unit by a full before/after diff of every unit's `(status,
wiring_class)` pair: `core_rulebook:class_feature:rogue_trapfinding`, `grounded` → `fixture-verified`
(`derived` wiring class's own bar — SD-29/SD-32's evaluator-fixture-correctness rung — cleared for
the first time by an interpreted value). `class_feature` 329 → 330 of 15,439. See §3a/§3b in
[rules-engine.md](./rules-engine.md) for the technical account of what got wired.

**The honest headline: tens, not thousands — and the gap between "the interpreter is now genuinely
readable for 84% of corpus arithmetic" and "one unit banked" is the wave's own most important
finding.** Four lanes fanned out to plug the interpreter into `class_feature` description
resolution, Cleric/Inquisitor domain powers, `race_trait`, and `monster`/`monster_ability`/`spell`;
a fifth widened the interpreter's own grammar (comparisons, `&&`, `skillinfo`). Every lane's
ARITHMETIC was sound — hand-re-verified against the pinned oracle by this integration cycle for
three separate banked/attempted values (Rogue Trapfinding, Demon (Vermlek)'s SLA caster level,
Cleric's Destructive Smite/Touch of Glory) with zero disagreement — but almost none of it reached the
board, for reasons that are structural, not arithmetic:

* **Chassis-support allowlist gaps.** 10 more `class_feature` records (Arcane Archer, Arcane
  Trickster, Assassin, Duelist, Vigilante) resolved correctly and were fixture-verified, but
  `compute_class_chassis`'s `table_class_id` allowlist has no entry at all for their 5 classes — a
  pre-existing, orthogonal chassis/BAB/save-table gap, not a description-resolution defect.
* **A measurement-instrument blind spot for an entire content family.** `v06_work_inventory.rs`'s
  `class_feature_owner` cannot attribute ANY `Domain Power ~ X` corpus row to Cleric or Inquisitor at
  all — confirmed not specific to the 2 domains wave 26 added, since Good's own `Touch of Good`
  (wired into real character totals since well before this wave) still reads `unknown` after the
  regen. Every domain-power lane's work, past and present, is invisible to the board until this is
  fixed.
* **A gaming vector in the one lane that DID show large board movement.** The `race_trait` lane
  reported +14; adversarial review proved by two independent mutations (disabling the seam entirely;
  emptying its whole fixture family) that the reported gain was unaffected either way — the credit
  traced to a hand-typed race-level allowlist const, not to any interpreted computation. **Not
  merged.** See `OPEN-ISSUES.md` row 365.
* **Standing preconditions correctly respected rather than routed around.** `classlevel(...)`'s
  known cross-class gap (no consumer may bank through it until fixed) blocked ~4 more units; several
  lanes explicitly declined to force a value through it.

**Interpreter refusal rate: 431/2,671 (16.1%) → 118/2,671 (4.4%)** — real, oracle-derived grammar
widening (bare/parenthesised comparisons, `&&`-chains, `skillinfo("TOTALRANK",...)`, all three
verified against the pinned `org.scijava:jep:2.4.2` dependency jar's decompiled bytecode). Zero of
this widening's own board effect is claimed — no consumer was wired to it this wave, matching wave
25b's own precedent for the base interpreter.

**No interpreted value is banked without a fixture whose expected value comes from bytes the
evaluator never reads.** Verified directly, not asserted: `derived_evaluator_fixture_check`'s 12 new
`class_feature_description_entries` (the only fixture family behind ANY board movement this wave) are
generated by `scripts/derive_class_feature_description_fixtures.py`, which reads only the pinned
upstream `.lst` bytes and evaluates them with its own from-scratch Python evaluator — structurally
independent of, and never calling, `formula_interpreter.rs`.

### What wave 26's integration cycle changed in the architecture, not just in the counts

* **A gate that cannot fail is worse than no gate, applied to the interpreter's own reported
  correctness.** Mutating `CmpOp::Gt`/`Lt`/`Le` one at a time (`>` → `>=`, `<` → `<=`, `<=` → `<`) —
  all three newly corpus-live this wave (72/24/6 distinct formula candidates) — left the whole
  `pilot_compute::` suite green under an adversarial review's own reproduction. Fixed by adding a
  boundary-straddling discriminating test per operator, mutation-proven RED then reverted, before
  this cycle's own commit.
* **A shipped module doc and a parse-time refusal message both asserted a false "confirmed" claim**
  — that `skillinfo`'s `TOTALRANK` is "the only [first argument] any corpus formula uses." The
  corpus uses `RANK` (4) and `TOTAL` (1) too; both correctly refuse (unimplemented, not silently
  defaulted), but the claim that they are absent was wrong and is corrected at all three sites this
  cycle (module doc, `Expr::SkillInfoTotalRank`'s doc, the refusal message itself).
* **A drift guard that degraded to a silent pass instead of a failure.** `bonus_stack_reader.rs`'s
  anti-drift test for its own hand-transcribed `ward.json` fixture fell back to an env var and
  `return`ed (test passes vacuously) when the corpus file could not be read — a no-op whenever that
  var was unset. Switched to this crate's own established `env!("CARGO_MANIFEST_DIR")` convention
  with a hard `.expect()`, so a missing/unreadable file now fails loudly.
* **A GAMED lane was excluded from the merge rather than partially salvaged under time pressure.**
  The `race_trait` lane's seam and fixtures are independently sound (verified twice by hand against
  the pinned oracle); only the board-credit mechanism (a coarse, hand-typed race-level allowlist) is
  the gaming vector. Rather than cherry-pick around that risk, the whole branch was left unmerged and
  the sound parts logged for a future wave to re-land cleanly — see `OPEN-ISSUES.md` row 365 for the
  two remediation paths a future wave can choose between.

## Corpus coverage, corpus-wide — re-derived 2026-08-21 (SD-31 wave 27, integration cycle)

**Wave 27's dispatch framed the program's real remaining wall as a class-dispatch problem — "we have
been building features for characters that cannot exist" — and asked how many of the 157 not-done
`class` units are Monk-shaped (chassis table present, only the `table_class_id` string mapping
missing). The census answer, independently re-derived by this integration cycle from
`docs/work-inventory.json`: ZERO.** `table_class_id`/`compute_class_chassis`/
`has_supported_class_chassis` already dispatch every one of the 34 classes that has a real chassis
table anywhere in the codebase (CRB 11, APG 6, ACG 10, Pathfinder Unchained 4, Ultimate Combat 3) —
the Monk-shaped pattern was exhausted by prior waves before wave 27 started. Board: **13,444 →
13,456/38,372 (35.0360% → 35.0673%)**, **+12**, denominator unchanged. `class` stays flat at
**28/185 — zero classes made buildable this wave**, confirmed independently by two lanes and this
integration cycle's own regen.

**Where the 157 not-done classes actually sit, corrected during this integration cycle after
adversarial review found 2 of 10 CRB prestige classes misclassified** (Arcane Archer wrongly filed
as needing no caster-stacking mechanism when 7 of its 10 levels carry `ADD:SPELLCASTER|Arcane`;
Pathfinder Chronicler wrongly filed as needing one when it has no spellcasting at all — both fixed
in-code, `7f2b0d4fd`): 77 prestige classes (CRB 10 + APG + Ultimate Psionics 19) need an
entry-requirement gating mechanism this codebase does not have at all, and 6 of the CRB 10
specifically (Arcane Archer, Arcane Trickster, Dragon Disciple, Eldritch Knight, Loremaster, Mystic
Theurge) additionally need a caster-level-stacking mechanism (advancing an EXISTING class's spell
progression from prestige levels) that also does not exist anywhere in this codebase —
`CharacterClassLevel` is a flat `{class_id, level}` with no cross-class link field. 48 units are
structurally not player-selectable base classes at all (33 Monster creature-type HD progressions, 7
Monster.Companion, 3 Psionic power-list menus, 3 untyped edge records, 2 Vigilante-identity Support
records) — kept in the denominator per standing no-scope-cuts precedent (`OPEN-ISSUES.md` row 372).
22 real base classes (Antipaladin, Magus, Vigilante, Shifter, the 6 Occult Adventures classes, the
10 Ultimate Psionics base classes) have zero chassis table anywhere and need net-new table
construction. 5 CRB NPC classes (Adept/Aristocrat/Commoner/Expert/Warrior) are real, untabled, and —
per two independent adversarial reviews this wave — the CHEAPEST remaining `class`-kind throughput in
the program: the pinned oracle already carries their complete BAB/save progressions in the exact
`classlevel()`-arithmetic shape the wave-25 interpreter reads, architecturally identical to the
~30-line Gunslinger chassis row wave 20 already shipped (`OPEN-ISSUES.md` row 373). 28 are in books
with no compiled rule set at all (`adventurers_guide` 25, `inner_sea_magic` 3). 2 (Ninja, Samurai) have
complete, correctly-dispatched chassis already and are blocked only by a missing weapon-proficiency
table row (`weapon_tables.rs`, outside every wave-27 lane's granted scope).

**Where the +12 actually came from — both units of movement genuinely landed, but neither is a
class-dispatch story:**

* **`class_feature` 330 → 332 (+2): Ranger Master Hunter, Rogue Master Strike.** Wires ability
  modifiers (STR/DEX/CON/INT/WIS/CHA) into `class_feature_grant_consumer.rs`'s formula-interpreter
  resolver, both riding on chassis that already existed (Ranger/Rogue are pre-existing CRB
  dispatches). Both are `derived`+`fixture-verified`, clearing Ruling §20's bar — but adversarial
  review drove the real production entry point (`compute_pilot_base_chassis`) across 165 synthetic
  characters and found the resolved value reaches ZERO of them: `already_computed_slugs` correctly
  suppresses the new row because a pre-existing hand-modelled `value:0` explanation already occupies
  each slug. The board credit is legitimate under the shipped bar (same shape as wave 26's row 366);
  the player-visible number is, and remains, the pre-existing hand-modelled one. `OPEN-ISSUES.md` row
  375 names the concrete, small unblock for a future wave.
* **`race_trait` 540 → 550 (+10): Samsaran (4/4 records) and Nagaji (6/6 records), full per-record
  coverage** — directly answering wave 26's Undine GAMED finding (row 365's 3-of-20 partial-coverage
  shape). One real defect was found and fixed during this integration cycle: Nagaji's Hypnotic Gaze
  is an ALTERNATE trait that replaces Serpent's Sense, but the merged seam emitted both unconditionally
  for every nagaji — fixed by gating both on `replaced_by_alternate_trait`, the same pattern
  `explain_gillman_flat_override_race_trait`/`explain_vanara_flat_override_race_trait` already use
  (`b49054eb9`). The board still banks the full +10 (not adversarial review's suggested +9) because
  the credit mechanism (`probe_race_trait_corpus`'s `is_seamed`) is race-level, not record-level —
  identical to the already-shipped, never-challenged Gillman Throwback / Vanara Tree Stranger
  precedent; mutation-proven this cycle (reverting the seam's race list reverts exactly these 10
  units and nothing else). `OPEN-ISSUES.md` row 380 has the full account.

### What wave 27's integration cycle changed in the architecture, not just in the counts

* **A durable, in-code architectural finding was corrected before it could become the next wave's
  false premise.** The prestige-class lane's stacking/non-stacking split (comment-only, `mod.rs`
  after `table_class_id`) had Arcane Archer and Pathfinder Chronicler backwards — caught by
  adversarial review re-checking the pinned oracle directly rather than trusting the lane's own
  `data/corpus/.../class_feature/<class>/` directory-listing probe, which is structurally blind to
  advancement encoded as `ADD:SPELLCASTER` directly on the `CLASS:` line. Corrected in both the
  in-code comment and the retro deferral event (append-only correction row, original left intact).
* **A real correctness bug was found and fixed in a unit already headed for the board.** Nagaji
  Hypnotic Gaze/Serpent's Sense mutual exclusivity (above) — the lane's own test built the exact
  input that should have caught this (no alternates selected) and asserted the bug's behavior instead
  of catching it; replaced with two tests, one per branch.
* **A false "no consumer exists" claim was corrected without over-claiming a fix that wasn't made.**
  Nagaji Armored Scales' doc comment said no natural-armor consuming total exists in this codebase;
  `FeatDerivedPillarContributions::natural_armor_bonus` is real and already sums three other sources.
  The record is not yet wired into it (a genuine, disclosed follow-on), and the doc comment now says
  so honestly instead of claiming a gap that isn't there.
* **A cross-cutting instrument blind spot was found and logged, not silently trusted.** A separate
  doneness dump (`v06_class_state_dump`, not the gate that actually banks units) omits all 3 Ultimate
  Combat classes from its roster and reports `blocked_count: 0` — a gate that cannot fail for classes
  it never enumerates. The real gate (`v06_work_inventory`'s `modelled_class_books()`) is unaffected
  and correct. `OPEN-ISSUES.md` row 374.

## SD-33: `unknown` reaches zero, and `docs/work-inventory.json` grows to 49,438 units

`docs/work-inventory.json`'s population grew from wave 27's 38,372 to
**49,438 units** between tranche/11 and tranche/13 (real SD-32 engine work —
companion grounding, bloodline/domain resolver widening, PI-audit fixes,
equipment-gap onboarding — landing on `develop` in the interim, plus SD-33's
own regenerations). `status: "unknown"` — a record `v06_work_inventory.rs`'s
`classify()` could not place into any other status — carried **4,224** units
at the start of SD-33's Epic 4. `classify()`'s own fallback logic had two
distinct shapes producing false `unknown` verdicts (`AT-33-E4-001`'s
root-cause finding): a `Kind::Feat`/`Kind::Equipment`/`Kind::EquipmentModifier`/
`Kind::Spell` unit whose token closure already proves a real magnitude
(`wc_class != "display"`) fell through a `text_only` branch that assumed no
magnitude existed, and `Kind::ClassFeature`'s owner-unresolved,
magnitude-bearing fallback had no disposition at all. Both are fixed at 5
`classify()` call sites (`AT-33-E4-002`, `src/bin/v06_work_inventory.rs`,
commit `00ca087775`). Per-unit movement of the exact 4,224, joined by `id`
against the pre-cycle inventory: **3,052 → `not-ingested`** (no consumer
holds this magnitude), **854 → `ingested-magnitude`** (a real magnitude the
engine holds, no observed consumer delta yet), **318 → `unmeasurable`** (the
genuinely-irreducible remainder — status string renamed from `unknown`,
disposition unchanged, every per-unit `reason` preserved verbatim per
`SD-33 decisions.md §7`'s permanent `unverifiable` bucket; the old name read as
"nobody looked" for a population that in fact carries a specific, stated
reason per unit).

```
$ jq '[.units[]|select(.status=="unknown")]|length' docs/work-inventory.json
0
$ jq '.units|length' docs/work-inventory.json
49438
```

**Doneness mapping.** Epic 4's reclassification required widening the
dashboard producer's doneness-verdict table (`scripts/pf1e_dashboard_
producer.py`, `verify.sh`'s `producer-selftest` stage) so every
`(wiring_class, status)` pair `classify()` can now emit maps to a defined
verdict — `unmeasurable` in particular needed a new `(ambiguous,
literal-/fixture-verified) -> held` mapping rather than falling through
`doneness_unmapped`. `AT-33-E6-001`'s own final-acceptance scan found this
mapping gap had briefly left `cargo test --locked --lib` red over 11 of
49,438 units (SD-33's own debt from `00ca087775`, closed same bundle,
`artifacts/epic-6-closure/AT-33-E6-001-suite-green_cycle_receipt.md`) — a
reclassification that changes a status string can leave a stale test
assertion behind even when the reclassification itself is correct.

### Desktop app: character sheet and update actions

| Item | Status | Where (re-verified) |
|---|---|---|
| `perform_install` | Always returns `Err("...not wired: downloading the AppImage artifact requires an HTTP client...")`; its TS caller `installAction.ts::performInstall` has zero production call sites — `Ui.tsx`'s `handleInstall` is a documented no-op. Doubly inert. | `apps/desktop/src-tauri/src/update/transaction.rs:763-771`; `apps/desktop/src/update/Ui.tsx:110-117` |
| `perform_retention_sweep` | Real, tested body (`perform_retention_sweep_impl`), but not in `main.rs`'s `generate_handler!` list — unreachable from the frontend. | `apps/desktop/src-tauri/src/update/transaction.rs:817`; `apps/desktop/src-tauri/src/main.rs:113-140` |
| `drive_list_campaigns` / `drive_load_campaign` / `drive_save_campaign` / `drive_delete_campaign` | Registered in `generate_handler!` and unit-tested, but no frontend file invokes any of them (confirmed: zero grep hits across `apps/desktop/src`). `campaignModel.ts` uses `localStorage` as the real source of truth; only `write_campaign_drive_artifacts` (one-way mirror) is called. | `apps/desktop/src-tauri/src/main.rs:132-135`; `apps/desktop/src/campaign/campaignModel.ts` |
| `append_to_character` / `re_save_character` | Registered in `generate_handler!` and unit-tested (SD-24 Epic 7, criteria 7.1/7.3), but no `boundary/*.ts` wrapper and zero `invoke()` call sites exist anywhere in `apps/desktop/src` — same "registered-but-unreachable" shape as the `drive_*` row above. (Their sibling `recompute_character` was wired to a real UI affordance by SD-25 Epic 3 — see the Real-today Character Hub row and `desktop-app.md`; these two were not, because SD-25 Criterion 3.5's own file-touch grant only wired the recompute call site.) | `apps/desktop/src-tauri/src/characterHub/appendToCharacter.rs`, `apps/desktop/src-tauri/src/characterHub/reSaveCharacter.rs`; `apps/desktop/src-tauri/src/main.rs` (registration) |
| Level-up acceptance | `LevelUpDialog`'s `onAccept` in `CharacterSheet.tsx` is an empty closure with a comment: "accepting is a no-op today." Nothing is persisted or recomputed. | `apps/desktop/src/characterHub/CharacterSheet.tsx` (`LevelUpDialog` `onAccept`) |
| Skill-allocation acceptance | `SkillAllocationDialog`'s own header comment: "Accepting only updates in-memory state (`onAccept`) — there is no backend [persistence]." Wired to a plain `useState` setter, lost on sheet close. | `apps/desktop/src/characterHub/SkillAllocationDialog.tsx` (header comment) |
| Character-sheet bio fields | Alignment/deity/sex/age/height/weight/hair/eyes are explicitly session-local; no persisted schema slot exists yet. | `apps/desktop/src/characterHub/CharacterSheet.tsx` (`DetailsPanel`) |
| Sheet `☰ Menu` | Graduated (SD-25 Epic 3, register A4): `Open` and `Clone` are now wired to real behavior, and `Save` — which had nothing session-local to persist — is replaced by a real `Recompute` action that calls `recompute_character` through the active rule-system adapter. `Print` (`window.print()`) is unchanged. No menu item is a bare no-op today. | `apps/desktop/src/characterHub/CharacterSheet.tsx` (`menuItems`, `handleRecompute`) |
| Campaign conflict merge | Conflict detection is real and tested (nonce-based); resolution is local-wins with both copies preserved under `conflicts/<timestamp>/` — there is no merge UI. | [persistence.md](./persistence.md) §"Conflict detection" |
| DM Toolkit UI | The Landing screen's "DM Toolkit" action routes to `StubScreen.tsx`, a generic "not built yet" placeholder — it does not call `encounters.rs`/`party_cr.rs` even though that compute is real (see the Real-today table above). | `apps/desktop/src/characterHub/CharacterHubPage.tsx:93-99`; `apps/desktop/src/characterHub/StubScreen.tsx` |

### Core engine: compute coverage and proof surfaces

| Item | Status | Where (re-verified) |
|---|---|---|
| Class/level compute coverage | Only single-class Fighter levels 1-3 reach `Computed` for any race; Wizard level 1 is closest but still blocked on spellbook/school-power diagnostics. | `apps/desktop/src-tauri/src/character_hub.rs:949-954` (test) |
| Oracle-parity comparator | **Graduated (SD-26 Epic 2): the in-crate harness now exists and is tested.** `oracle_validation::comparator::compare` aligns a normalized PCGen output against Codex's selected dimensions and reports per-dimension matches/mismatches; `normalization` reduces raw PCGen text into the comparator's input shape; `parity_report` renders a real `PASS`/`FAIL` `parity_report_<case-id>.md`; `pcgen_runner::run_pcgen_character` wraps the two real PCGen scripts into one Rust call. What is still deferred is a *passing* parity claim: the pilot end-to-end run (`tests/sd26_pilot_case_verification.rs`) currently produces a real **FAIL** — two genuine `skill.selected_modifier.{climb,swim}` mismatches because `pilot_compute::compute_ability_modifiers` does not yet apply the chosen racial ability bonus (the open CG-03 blocker). `SelectedParityDimensions` still carries only a `Computed` `ClaimTierFloor` (no `OracleChecked` variant), so no fixture can yet assert oracle-checked parity. The harness is real; a green parity verdict is not, pending CG-03. | `src/oracle_validation/comparator.rs`; `src/oracle_validation/normalization.rs`; `src/oracle_validation/parity_report.rs`; `src/oracle_validation/pcgen_runner.rs`; `tests/sd26_pilot_case_verification.rs`; `src/rules_core/pilot_compute/mod.rs` (CG-03) |
| Bestiary 1 monster parser | `monster_stat_block.rs`'s row parser is fully unwired: no `ParsedLstRecord`/`SourceContentPayload` variant exists for it, and its only callers outside its own module are its own test file. Bestiary 1 table content is hand-transcribed, not parsed through the canonical-IR path. | `src/pcgen_import/lst_parser/monster_stat_block.rs`; zero references in `ir_converter.rs`/`source_content_payload.rs`/`source_content.rs` |
| Failure-owner classifier | `pilot_failure.rs`'s `primary_owner` only ever returns `OracleGap` (on `Computed`) or `EngineFlaw` (on `Blocked`); `ModelFlaw`/`ImporterFlaw`/`UiGap` are unreachable from the current receipt surface. | `src/rules_core/pilot_failure.rs:61-66` |
| Spellbook magnitude — a third disconnected twin (**in-flight**) | `contract::build_pilot_receipt` wires `spellbook::compute_spellbook_coverage` into `PilotReceipt.spellbook`, but nothing in the shipped desktop app reaches it — `grep -rn build_pilot_receipt apps/desktop/src-tauri/src` returns 0 hits (re-confirmed 2026-08-07). The app instead gates on `pf1_adapter::resolve_unified_pilot_snapshot`, so no spell magnitude reaches a player surface through the receipt path. **This is being actively worked right now**: at write time, `apps/desktop/src-tauri/src/character_hub.rs`, `apps/desktop/src-tauri/src/pf1_adapter.rs`, and `src/rules_core/pilot_view_model.rs` all carry uncommitted changes adding spell save DC/slot-total surfacing directly through `resolve_unified_pilot_snapshot` (not by connecting `build_pilot_receipt`) — confirm current state before relying on this row; it may already be resolved. Same shape as `decisions.md §29.1`/`§29.2` (a magnitude not wired until it moves on the twin the player reads). | `src/rules_core/contract.rs` (`build_pilot_receipt`); `apps/desktop/src-tauri/src/pf1_adapter.rs` (`resolve_unified_pilot_snapshot`) |
| Per-item corpus equipment stats | `pilot_compute_corpus.rs`'s `DerivedEquipmentStats` is always `default()` — a permanent placeholder there; real per-item stats are computed separately by `equipment_effects.rs`. | `src/rules_core/pilot_compute_corpus.rs:80-147` |
| Homebrew content breadth | Guard Stance (`guard_stance_shell`/`guard_stance_proof`) is the only authored package content the authoring format ships; no second package constructor exists. | `src/homebrew_authoring/mod.rs:106-117` |
| Future-state books (`book_stub`) | 21 out-of-scope Paizo books (`data/stubs/*.json`, e.g. `bestiary_4`, `ultimate_magic`) are registered as honest `book_stub` future-state placeholders (SD-26 Epic 4) — each carries only `book_id`/`book_name`/`planned_resolution_bundle`/`registered_at`, `content_kind_counts: null`, and no rule data. They are declared, not implemented: the registry (`docs/governance/wired-integration-stubs-registry.md`, 21 `book_stub` entries) tracks them so the corpus-diagnostic surface can name a book as "known but unbuilt" rather than silently omit it. Concrete rule-system content lands in SD-27+. | `data/stubs/*.json`; `docs/governance/wired-integration-stubs-registry.md` |

### Release pipeline: CI coverage gaps

| Item | Status | Where (re-verified) |
|---|---|---|
| No concurrency guard on publish | `publish-tester-release.yml` declares no `concurrency:` block; two rapid pushes to `develop` can run two concurrent `finalize` jobs, each pushing to the shared `update-index` branch (mitigated only by each push being a fast-forward-or-fail `git push`, not by the workflow serializing runs itself). | `.github/workflows/publish-tester-release.yml` (no `concurrency` key anywhere in the file — re-confirmed by grep) |
| No tranche/5-scoped CI workflow | `tranche-3-ci.yml` is the only tranche-specific workflow present, and it is scoped to `tranche/3` only (refuses PRs targeting any other branch by design). No `tranche-5-ci.yml` or equivalent exists yet. | `.github/workflows/` (directory listing: only `tranche-3-ci.yml` matches `tranche*`) |

This doc is the first one every SD closure re-checks — a stub graduating to
real, tested behavior is the most common architectural-doc change, and it
must be reflected here before it is reflected anywhere else. See
[README.md](./README.md) §Maintenance contract for the update procedure.
