# Glossary

> Scope: every project-specific term a newcomer meets in this codebase or its docs, each with a
> short definition and a link to the doc that treats it in full.
> Merge pass: **2026-10-07, `tranche/17` merged with `develop` @ `fd68740f60`** (SD-37 E7.9a): both term sets kept; the Data root entry now names the per-system roots.
> Last verified: **2026-10-07 against `tranche/17` (`b99c3d4b02`)** for the SD-37 terms (game system, held set,
> data package, rules catalog, golden digests, Starfinder chassis).
> Also verified: **2026-10-07 against branch `tranche-16-ui`** for the four terms it added (data root, installed state, release-notes pointer, version stamp). Earlier: **2026-09-26 against `tranche/16` (`e70a8745ed`)** for the SD-36 Epic F terms (carrier mix,
> class census, closure-complete attestation, class roster, mechanism, sabotage parity, status parity,
> structural diff). Earlier: **2026-09-20 against `tranche/16` (`b22ea9e113`, SD-36 Epic D)**. New this pass
> (`docs/architecture/glossary.md` did not exist before SD-36 Epic D).
> Maintenance: updated at SD closure — see [README.md](./README.md) §Maintenance contract

Terms are alphabetical. A term used only inside one doc and defined fully there is still listed
here with a one-line pointer, so a search for the word always lands somewhere.

## Baseline (floor / ceiling)

A recorded number in `scripts/verify-baselines.env`, one `BASELINE_*` variable per gate, each
tagged with a **direction**: a *floor* (test/binary counts — a drop below it fails the stage; a
rise is fine and should be re-recorded) or a *ceiling* (clippy warning counts — a rise above it
fails; a drop should be re-recorded to lock in the improvement). The file is append-only; the
**last** assignment of a name wins. See [testing.md](./testing.md).

## Book / book id

One Paizo sourcebook, keyed by a lowercase-underscore identifier matching its `data/corpus/<book
id>/` directory name (e.g. `core_rulebook`, `advanced_players_guide`, `bestiary_2`). See
[rules-data-tables.md](./rules-data-tables.md) and [corpus-ingest.md](./corpus-ingest.md).

## Boundary contract

`src/rules_core/contract.rs`'s `to_pilot_receipt -> PilotReceipt` and `printed_sheet_cell_map ->
PrintedSheetCell` — the machine-checked proof surface for what the desktop GUI is allowed to
render, exercised by `tests/sd20_contract_*.rs`. Distinct from the desktop **boundary layer**
(next entry) despite the shared word. See [rules-engine.md](./rules-engine.md).

## Boundary layer / boundary wrapper rule

`apps/desktop/src/boundary/*.ts` — one dedicated wrapper module per Tauri command family.
Components never call `invoke()` inline; they call a boundary wrapper, which is gated on
`hasTauriRuntime()` and falls back to a preview value outside a real Tauri runtime. See
[conventions.md](./conventions.md) §"Boundary wrapper rule" and [desktop-app.md](./desktop-app.md).

## Carrier mix

The multiclass build a prestige class is measured in, because a prestige class taken alone is not a
legal character (it is Blocked by the game rule, `prestige_class.requires_base_class_levels`). The
census picks the carrier its own converted entry gate names (e.g. a wizard for an arcane gate) at the
smallest level meeting every numeric entry term, and prints the entry requirements met/unmet — they
never block. See [rules-engine.md](./rules-engine.md) §3d and [status.md](./status.md).

## Chassis

Two related but distinct senses, both meaning "the load-bearing structural skeleton, not the
decoration":
1. **Class chassis** — the compute-side dispatch (`compute_class_chassis`,
   `compute_fighter_chassis`, ...) that produces base attack bonus and base saves for a
   class/level. See [rules-engine.md](./rules-engine.md).
2. **Rules-table chassis** — the per-book/per-class BAB/save/skill-points tables, stored as the JSON
   data package `data/rules_tables/` and read through `src/rules_core/rules_catalog/`; rows carry a source
   TOKEN (never a pre-computed number) for the compute side to read. See
   [rules-data-tables.md](./rules-data-tables.md) §"Rows carry the corpus token, never a computed number".

## Class census

`cargo run --locked -j 8 --bin class_census -- --json <path>` (`src/rules_core/class_census.rs`) —
the one instrument that measures class coverage corpus-wide: every engine registry merged into one
id set (137), each class swept at every level. status.md's class table is generated from it and the
desktop class roster is served from it. See [testing.md](./testing.md) §"The SD-36 Epic F
instruments".

## Class roster

The list of classes the desktop Create picker offers, served by `list_class_creation_roster` from
the census: every non-prestige class `Computed` at every level that states a hit die, less the Ex-*
states (census-only by ruling). Each class not offered carries a `roster_reason` (`prestige`,
`ex_state`, `not_computed`, `hit_die_absent`). See [desktop-app.md](./desktop-app.md).

## Closure

The state a bundle (or one of its epics) reaches when every acceptance criterion is done, its
architecture docs are refreshed, graphify has run against the final tree, its PR is open and
merged, and its worktree/branches are swept — a fixed, sequential pipeline, never a partial
subset. See `docs/release/<bundle>/workflow-instruction.md §11` for the exact steps a given
bundle runs, and [README.md](./README.md) §Maintenance contract for what architecture-docs closure
specifically requires.

## Closure-complete attestation

`SheetRule::closure_complete` on a converted class principal: true iff every rule the class line
reaches converted with zero closure defects. The proficiency reader answers Known only when it
holds, so "the class grants no weapon" is never confused with "the grant was lost at conversion".
Written by `crates/codex-ingest/src/pcgen_import/sheet_rule/attest.rs`. Not the bundle sense of
**Closure** above. See [corpus-ingest.md](./corpus-ingest.md).

## Corpus

The real PCGen open-source rule data (`.pcc` entry files + `.lst` object-data files) — an
**external, unvendored checkout**, never committed to this repo, located by `$PCGEN_CORPUS_ROOT`
(data) / `$PCGEN_REPO_DIR` (repo root) and pinned to one commit in
`scripts/pcgen-oracle-pin.env`. Not to be confused with `data/corpus/`, this repo's own **committed
JSON cache derived from** the corpus. See [corpus-ingest.md](./corpus-ingest.md) and
[getting-started.md](./getting-started.md) §4.

## Corpus-literal sweep

`corpus_literal_sweep` (a `codex-ingest` binary, `scripts/verify.sh`'s `corpus-sweep` stage) —
a byte-equality proof that every corpus-literal value this repo ships was transcribed verbatim
from the pinned oracle, not paraphrased or hand-typed. See [testing.md](./testing.md).

## Crate wall

The Cargo-workspace-enforced separation between the `codex` crate (ships in the desktop binary,
never reads PCGen's file format) and the `crates/codex-ingest` crate (the PCGen converter/oracle
tool side). `codex` has no dependency on `codex-ingest`, normal or dev; `codex-desktop` takes
`codex-ingest` as a **dev-dependency only**. Checked structurally by the `crate-wall` `verify.sh`
stage and by content via the residue gate (next entry). See [overview.md](./overview.md)
§"The converter/live boundary" and §"Workspace and crate dependency graph".

## Cycle

One dispatched unit of work inside a bundle's epic, closed by a **cycle receipt** (see Receipt)
following the schema in `docs/governance/workflow-instruction-template.md §7`.

## Data package (`rules_tables`)

The Pathfinder rules tables as one JSON file per table under `data/rules_tables/<table id>.json`, with a licence/PI
stamp per file and a published schema. It is the tables' only home: the compiled Rust module they were first
rendered from no longer exists. Loaded by `src/rules_core/rules_data_package.rs`, read through the
[rules catalog](#rules-catalog). See [rules-data-tables.md](./rules-data-tables.md).

## Data root

The directory the rules crate joins every runtime data path onto (`data/sheet_rules`, `data/class_feature_grants`, the roster fixture, …): `support::paths::repo_root()`. It is the repo checkout (Cargo's compile-time `CARGO_MANIFEST_DIR`) by default, and the packaged app's resource directory once the shell calls `codex::set_data_root` at startup. Each [game system](#game-system)'s package roots (`data/sheet_rules`, `data/starfinder-1e/sheet_rules`) and the [data package](#data-package-rules_tables) are joined onto the same root through `game_system::runtime_repo_root()`: the installed data root, else `CODEX_REPO_ROOT`, else the checkout. Reading the compile-time path in production is a defect, because an installed app does not have the build machine's checkout; `packaged_resources.rs` is the control. See [desktop-app.md](./desktop-app.md) §How the rules crate finds its data.

## Denominator gate

`scripts/denominator_gate.py --check` (`verify.sh` stage `denominator-gate`) — fails a bundle's
own headline docs and receipts if a line states a bare percentage without a same-line denominator
marker ("of N", "N/M", "out of N"). Exists because a **true number over the wrong population** is
the costliest error shape this program has recorded — it survives review because it is not false.
See `AGENTS.md` rule 9 and [testing.md](./testing.md).

## DOM probe

`apps/desktop/src/testSupport/uiProbe.ts`'s `installUiProbe()` — a DEV-only in-page command
channel the running React app exposes so `ui-smoke` can assert rendered state and drive
interactions without pixel-coordinate clicking. Registered on the Rust side via `ui_probe.rs`.
Distinct from the corpus classifier's unrelated "consumer-delta probe" heuristic mentioned in
[status.md](./status.md) — both are called "probe" in this codebase; context disambiguates. See
[getting-started.md](./getting-started.md) §"`npm run ui-smoke`".

## Epic

A named subdivision of an SD-N bundle (e.g. "Epic A — PCGen wall", "Epic C1 — source-side
refactor"), each with its own acceptance criteria in that bundle's `epic-breakdown.md`. See
[README.md](./README.md) and any `docs/release/<bundle>/epic-breakdown.md`.

## Fail-honest

The rule that nothing in this codebase fabricates a value it cannot prove: every computed field
carries a real explanation record on the supported path, or a named claim-blocking diagnostic and
a zeroed/absent value on the unsupported path. See [overview.md](./overview.md) §"The product
doctrine" and [conventions.md](./conventions.md) §"Fail-honest computation".

## Figure-provenance

`scripts/denominator_gate.py --check-provenance` (`verify.sh` stage `figure-provenance`) — a
**different check from `--check`** (the denominator gate above): it enforces that every figure in
a "Figures + their re-derive commands" section carries its own re-derive command on its own line.
See `AGENTS.md` rule 9 and [testing.md](./testing.md).

## Game system

`GameSystem` in `src/rules_core/game_system.rs`: `Pathfinder1e` (`pathfinder-1e`) or `Starfinder1e` (`starfinder-1e`).
Names a package root and a book list; an unknown id is an error, never a default. See
[overview.md](./overview.md) §"Two game systems, one engine".

## Golden digest

A sha256 in `src/rules_core/rules_catalog/golden_digests.txt` of the transcript of one rules table, view or lookup as
the compiled module last answered it. `rules_catalog::golden_tests` re-derives each from the package, so a changed
row or a type that serialises differently fails naming its table. See [rules-data-tables.md](./rules-data-tables.md).

## Grand epic (GE-NN)

An older naming lineage predating the `SD-NN` convention, still visible as a file-name prefix
(`ge06_*`, `ge08_*` test files) — a proper noun naming provenance, not a live organizing unit. See
[README.md](./README.md)'s provenance note.

## Held set

The records a Starfinder character holds: its race, its first class's level-1 `BaseClass` template, its theme and every
record those grant, to a fixpoint, plus the worn armour and its picks. Every `sf_*` reader folds the rows of the one held
set (`sf_defense::held`), so a total on the sheet is the number the package's own rows produce. See
[rules-engine.md](./rules-engine.md) §3e.

## Installed state

`installed-state.json` under `<config>/codex/update/`: the record of what build is installed (install kind, version, source commit, artifact sha256, managed path, eligibility). The Update panel and `is_install_eligible` read it; `update/seed.rs` writes it at every startup for `.deb`, dev and first-run AppImage installs, and `verify_relaunch_artifact` writes it after an AppImage self-update. Without it every installed field reads `unknown`. See [update-and-feedback.md](./update-and-feedback.md).

## Kanban

`docs/release/<bundle>/kanban.md` — the per-bundle task board tracking each epic/criterion's
dispatch status (READY, blocked, complete). Read alongside `workflow-instruction.md`, not a
substitute for it.

## Mechanism (unresolved-reference)

A letter naming WHY a converted reference did not resolve, so a remainder is named by cause, never
"the rest": A child category with a converted target, B child category with an unconverted target,
D plain category with a converted target the resolver still misses, E target in a book not
ingested, F target found nowhere; G/H/N name closure-defect causes in the proficiency reader's
remainder. Re-derive with `docs/release/SD-36-consolidation/artifacts/epic-f/scripts/unres2.py`;
open rows are in `docs/release/SD-36-consolidation/forward-scope-register.md`.

## Oracle

PCGen itself, treated as the **parity ground truth** for Codex's own computed output — never as a
run-time dependency. `crates/codex-ingest/src/oracle_validation/` runs Codex's computed values
against PCGen's own real behavior, dimension by dimension, and renders a PASS/FAIL parity report.
See [homebrew-and-oracle.md](./homebrew-and-oracle.md).

## PI (Product Identity) screening

The blacklist-driven scrub (`rules_core::pi_screening`, `docs/governance/ogl-pi-blacklist.md`)
that keeps declared Open Game License Product Identity terms out of shipped corpus records and
generated prose. See [rules-data-tables.md](./rules-data-tables.md).

## Pillar

A named, discrete class-feature mechanic (e.g. Barbarian Rage, Uncanny Dodge, a feat pillar in
`src/rules_core/pilot_compute/feat_pillars.rs`) computed once and read by multiple downstream
consumers (e.g. level-up planning) rather than re-derived per consumer. See
[rules-engine.md](./rules-engine.md).

## `pilot_compute`

`src/rules_core/pilot_compute/` — the deterministic chassis-compute module,
`compute_pilot_base_chassis` its entry point. Was a single file until SD-36 Epic C1 split it into
per-class submodules (`class_fighter.rs`, `class_wizard_prepared_spellbook.rs`, ...) for size;
call sites (`pilot_compute::...`) did not change. See [rules-engine.md](./rules-engine.md).

## Presence gate vs. correctness gate

A **presence gate** checks that something exists or was attempted (a field is non-null, a stage
ran); a **correctness gate** checks the value itself is right. "Every gate this program has built
so far checks presence... a wrong computed number looks exactly like a right one"
(`docs/release/SD-33-computed-value-verification/README.md`) — the distinction this program built
`denominator-gate`/`figure-provenance` and the derived-evaluator fixture seam to close. See
[testing.md](./testing.md).

## Probe

See DOM probe above — this codebase uses "probe" for more than one mechanism; check context.

## Receipt

Three distinct senses, disambiguated by context:
1. **Cycle receipt** — a per-criterion markdown proof file under
   `docs/release/<bundle>/artifacts/<epic>/<criterion>_cycle_receipt.md`, following the schema in
   `docs/governance/workflow-instruction-template.md §7`.
2. **`PilotHeadlessReceipt`/`PilotReceipt`** — the rules-engine's own computed-value struct
   (`build_pilot_headless_receipt`, `contract.rs::to_pilot_receipt`), carrying a `Computed`/
   `Blocked` status and its explanation records. See [rules-engine.md](./rules-engine.md).
3. **`provenance.json` receipt** — the release pipeline's per-build attestation written during
   `publish-tester-release.yml`. See [release-pipeline.md](./release-pipeline.md).

## Release-notes pointer

`docs/release/current-release.json` (`{"tranche": N, "notes_path": "docs/release/<spec-dir>/release-notes.md"}`): the only thing that tells the publish workflow which notes to ship. `tools/release/resolve_release_notes.py` reads it and fails when it disagrees with the app's tranche or names placeholder notes. See [release-pipeline.md](./release-pipeline.md) §How the release notes are chosen.

## Residue gate

`scripts/pcgen_residue_gate.py` (`verify.sh` stage `pcgen-residue-gate`) — greps the live-side
crate for PCGen token syntax (`BONUS:`, `DEFINE:`, `PRE*:`, `%CHOICE`, `CL=`) and fails when the
hit count rises above zero; `--check --closure` requires exactly zero. See
[overview.md](./overview.md) §"The converter/live boundary".

## Retro event

One append-only JSON line in `docs/retro/events/<actor-slug>.jsonl`, emitted via `scripts/retro.py`
to record a correction, incident, deferral, or rework — the things git itself never captures.
See `AGENTS.md` §Retrospective Logging and `docs/retro/schema.json`.

## Rules catalog

`src/rules_core/rules_catalog/`: the package-backed Rust layer over the [data package](#data-package-rules_tables). It
holds the row and id types, the pure functions, the `Table`/`Derived` accessors and the per-book resolvers; no table
row is compiled into it. See [rules-data-tables.md](./rules-data-tables.md).

## Sabotage parity

Proof that a test refactor or a flipped assertion still guards what it guarded: break the engine on
purpose in a named way, record which tests turn red, require the identical set after the change, and
0 red once restored. See [testing.md](./testing.md).

## Seam

A deliberate composition/extension boundary between two pieces of code that could otherwise be
tangled — used for several distinct boundaries in this codebase: the desktop's
`build*Surface`/`*Runtime` dependency-injection seam ([conventions.md](./conventions.md)), a
per-race trait-recognition seam (`explain_human_pilot_race_seam` and siblings,
[rules-engine.md](./rules-engine.md)), and the `RuleSystemAdapter`'s rule-system adapter seam
([desktop-app.md](./desktop-app.md)). Each instance is documented at its own site; there is no one
general "seam" abstraction in the code.

## Sheet-rule (converter and schema)

The SD-35 output of the corpus converter: one `SheetRule` JSON record per corpus entry, written to
`data/sheet_rules/<book>/<kind>/<key>.json` by `crates/codex-ingest/src/bin/sheet_rule_convert.rs`,
carrying no PCGen token or formula string — only our own resolved schema, read at run time by
`src/rules_core/sheet_rule.rs`. See [corpus-ingest.md](./corpus-ingest.md).

## Slug

A lowercase, punctuation-normalized identifier segment used as (part of) a record key, e.g. the
`<slug>` in `beastiary1:monster:<slug>` or `data/sheet_rules/<book>/<kind>/<key>.json`'s `<key>`.
See [rules-data-tables.md](./rules-data-tables.md).

## SD-NN (Spec Domain)

One numbered release bundle (e.g. SD-35, SD-36) — the unit of planning, dispatch, and closure this
whole program is organized around. Its full package lives at `docs/release/<bundle>/`. See
[README.md](./README.md) and [getting-started.md](./getting-started.md) §"Pointers".

## `_settled` bundle

`data/corpus/<book>/_settled/<kind>.json` — a **pre-resolved** per-record bundle computed once at
authoring time (`gen_settled_corpus`, now in `crates/codex-ingest`) so the live loader
(`src/rules_core/settled_corpus.rs`) never has to re-apply `.COPY=`/`.MOD` resolution at run time.
Regenerate-and-diff (`gen_settled_corpus --check`) is its own idempotency proof. See
[corpus-ingest.md](./corpus-ingest.md) and [rules-data-tables.md](./rules-data-tables.md).

## Status parity

The assertion the multiclass negative controls make since SD-36 F3d: a mix's receipt status equals
the class-alone status, and its claim-blocking set (re-scope stripped) equals the class-alone set.
See [testing.md](./testing.md) and `docs/release/SD-36-consolidation/decisions.md` §14.2.

## STC package

The standard chassis every `docs/release/<bundle>/` folder follows: `scope-draft.md`,
`decisions.md`, `epic-breakdown.md`, `technical-design.md`, `workflow-instruction.md`,
`progress.md`, `receipts.md`, `release-notes.md`, and a per-cycle `artifacts/` directory — see
`docs/release/README.md`'s `layout_rule` and `docs/governance/STC-Skill-Creation.md`. **Not
independently verified**: this repo's own docs use "STC" as an established proper noun (e.g. "STC
chassis," "STC package") without spelling out the acronym anywhere in-repo; treat "STC" as a name
for this chassis shape, not as an expansion this doc can confirm.

## Structural diff

The gate every converter change passes: regenerate the sheet-rule package into a scratch directory,
diff it against the committed package field by field, and require every delta to fall in a pinned
delta class, with records unmoved and planted mutations failing
(`docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py`). See
[testing.md](./testing.md).

## Tranche

A long-lived branch (`tranche/N`) that one or more SD-N bundles land on before merging into
`develop`; also the "tranche-base" digit in the desktop app's `0.<tranche>.x` version scheme,
bumped only when a **new** `tranche/N` branch is cut. See [release-pipeline.md](./release-pipeline.md)
and [getting-started.md](./getting-started.md) §"Branch model, commits, and PRs".

## ui-smoke

`apps/desktop/scripts/ui-smoke/run.mjs` (`npm run ui-smoke`) — a red/green regression harness that
drives the running desktop app through a spec of UI rows via the DOM probe (above), asserting
rendered state rather than screenshots. Supports `--only`, `--from`, and `--resume` for
incremental repair passes. See [getting-started.md](./getting-started.md).

## Version stamp

The `<major>.<tranche>.<build>` version minted by the publish workflow's `stamp` job (`0.16.${GITHUB_RUN_NUMBER}`) and written into `apps/desktop/package.json` and `src-tauri/tauri.conf.json` before every build. The committed files stay at `0.<tranche>.0`. `Cargo.toml` is not stamped, so display code uses Tauri's `package_info().version`, never `CARGO_PKG_VERSION`. See [release-pipeline.md](./release-pipeline.md) §Stamp delivery.

## Wiring class

The GE-01 taxonomy every corpus record carries — `Display`, `Static`, `Derived`, `Computed` (a
strict lattice, highest-bar-wins), or `Ambiguous` — determined once, corpus-wide, by
`crates/codex-ingest/src/pcgen_import/wiring_class.rs` from a record's full token closure. See
[rules-data-tables.md](./rules-data-tables.md) §"`wiring_class` (GE-01 taxonomy)".

## See also

- [overview.md](./overview.md) — where most of these terms are first used in context.
- [getting-started.md](./getting-started.md) — the practical commands behind several of these terms.
- [README.md](./README.md) — the doc set's index.

## Custom

The GM's grants and house-rule records for one character, saved as `custom.json` beside it. An
ability grant is applied to the saved ability score; hit point and skill point grants are added by
the sheet; custom feats, equipment, spells and magic devices are listed and printed but not computed
from. See [desktop-app.md](./desktop-app.md).

## Manage dialog

The Create screen's modal for choosing a group of options (racial traits, traits, feats, spells) in
Options / Selected (/ Innate) columns, with a Qualified filter and the remaining count on top.
