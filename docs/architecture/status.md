# Status

> Scope: what is real, working product surface today across the whole repo, and what is stubbed, partially wired, or deferred — superseding the root README's "Current state" section.
> Last verified: 2026-08-13 against `tranche/9` (SD-29 **real** closure, Epic 11 run 3). The 2026-08-11 pass belonged to a closure the operator rescinded the same day (`SD-29 decisions.md §42`); every figure it wrote has been re-derived here rather than carried. The rows re-derived in full this pass are the `RuleSetId` catalog count, the JSON-corpus-cache count, the monster/companion/race-trait chassis rows, and the whole §"Corpus coverage, corpus-wide" section; every other row carries its prior 2026-08-07/tranche-8 verification and is unchanged by SD-29.
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
`deferred-with-reason` **36**.

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
`SD-31-corpus-closure-grind`'s and `SD-32-engine-capability-builds`'s to perform as they land the
per-record work this section tracks.

### The three structural ceilings, each measured by a checked-in classifier

A remainder is not a workload. Each of the three lanes SD-29 ran to its
ceiling has a checked-in row classifier that splits its raw remainder into
workable rows and structurally-unreachable ones, and each classifier is the
citation for the split — not a receipt's prose.

- **Race chassis is the `race_trait` ceiling.** `scripts/race_trait_ceiling.py`
  derives a ceiling of **571** rows (553 `TYPE:<Race> Racial Trait` + 18
  `TYPE:<Race> Subrace` heritage selectors) over the **18** races the ingest
  chassis models (`src/bin/ingest_race_traits.rs`'s
  `IN_SCOPE_RACES: [&str; 18]`, asserted by that file's own test). **513 of
  the 571 are `grounded`**; the 58 that are not each carry a recorded
  finding, and **2,876** of the 3,447 units are chassis-blocked residue that
  no race-trait ingest can ever ground. The engine's separate
  `crb::race_tables::race_traits()` still models 7 races — that is the
  *compute* surface, not the ingest surface, and the two must not be
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
  remaining row corpus-wide (`core_essentials` / `Pseudodragon ~ Tail`), and
  that row needs an `ASPECT:` chassis no table in this program models.

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


## Stubbed / partially wired / deferred today

Grouped by the plane each item lives in. Every row was re-verified directly
against the cited source, not carried over from a sibling doc unchecked.

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
| Oracle-parity comparator | **Graduated (SD-26 Epic 2): the in-crate harness now exists and is tested.** `oracle_validation::comparator::compare` aligns a normalized PCGen output against Codex's selected dimensions and reports per-dimension matches/mismatches; `normalization` reduces raw PCGen text into the comparator's input shape; `parity_report` renders a real `PASS`/`FAIL` `parity_report_<case-id>.md`; `pcgen_runner::run_pcgen_character` wraps the two real PCGen scripts into one Rust call. What is still deferred is a *passing* parity claim: the pilot end-to-end run (`tests/sd26_pilot_case_verification.rs`) currently produces a real **FAIL** — two genuine `skill.selected_modifier.{climb,swim}` mismatches because `pilot_compute::compute_ability_modifiers` does not yet apply the chosen racial ability bonus (the open CG-03 blocker). `SelectedParityDimensions` still carries only a `Computed` `ClaimTierFloor` (no `OracleChecked` variant), so no fixture can yet assert oracle-checked parity. The harness is real; a green parity verdict is not, pending CG-03. | `src/oracle_validation/comparator.rs`; `src/oracle_validation/normalization.rs`; `src/oracle_validation/parity_report.rs`; `src/oracle_validation/pcgen_runner.rs`; `tests/sd26_pilot_case_verification.rs`; `src/rules_core/pilot_compute.rs` (CG-03) |
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
