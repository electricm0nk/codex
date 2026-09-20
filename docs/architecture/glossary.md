# Glossary

> Scope: every project-specific term a newcomer meets in this codebase or its docs, each with a
> short definition and a link to the doc that treats it in full.
> Last verified: **2026-09-20 against `tranche/16` (`b22ea9e113`, SD-36 Epic D)**. New this pass
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

## Chassis

Two related but distinct senses, both meaning "the load-bearing structural skeleton, not the
decoration":
1. **Class chassis** — the compute-side dispatch (`compute_class_chassis`,
   `compute_fighter_chassis`, ...) that produces base attack bonus and base saves for a
   class/level. See [rules-engine.md](./rules-engine.md).
2. **Rules-table chassis** — the hand-transcribed per-book/per-class BAB/save/skill-points tables
   in `src/rules_core/rules_tables/`, which carry a source TOKEN (never a pre-computed number) for
   the compute side to read. See [rules-data-tables.md](./rules-data-tables.md) §"What it holds".

## Closure

The state a bundle (or one of its epics) reaches when every acceptance criterion is done, its
architecture docs are refreshed, graphify has run against the final tree, its PR is open and
merged, and its worktree/branches are swept — a fixed, sequential pipeline, never a partial
subset. See `docs/release/<bundle>/workflow-instruction.md §11` for the exact steps a given
bundle runs, and [README.md](./README.md) §Maintenance contract for what architecture-docs closure
specifically requires.

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

## Grand epic (GE-NN)

An older naming lineage predating the `SD-NN` convention, still visible as a file-name prefix
(`ge06_*`, `ge08_*` test files) — a proper noun naming provenance, not a live organizing unit. See
[README.md](./README.md)'s provenance note.

## Kanban

`docs/release/<bundle>/kanban.md` — the per-bundle task board tracking each epic/criterion's
dispatch status (READY, blocked, complete). Read alongside `workflow-instruction.md`, not a
substitute for it.

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

## Residue gate

`scripts/pcgen_residue_gate.py` (`verify.sh` stage `pcgen-residue-gate`) — greps the live-side
crate for PCGen token syntax (`BONUS:`, `DEFINE:`, `PRE*:`, `%CHOICE`, `CL=`) and fails when the
hit count rises above zero; `--check --closure` requires exactly zero. See
[overview.md](./overview.md) §"The converter/live boundary".

## Retro event

One append-only JSON line in `docs/retro/events/<actor-slug>.jsonl`, emitted via `scripts/retro.py`
to record a correction, incident, deferral, or rework — the things git itself never captures.
See `AGENTS.md` §Retrospective Logging and `docs/retro/schema.json`.

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

## STC package

The standard chassis every `docs/release/<bundle>/` folder follows: `scope-draft.md`,
`decisions.md`, `epic-breakdown.md`, `technical-design.md`, `workflow-instruction.md`,
`progress.md`, `receipts.md`, `release-notes.md`, and a per-cycle `artifacts/` directory — see
`docs/release/README.md`'s `layout_rule` and `docs/governance/STC-Skill-Creation.md`. **Not
independently verified**: this repo's own docs use "STC" as an established proper noun (e.g. "STC
chassis," "STC package") without spelling out the acronym anywhere in-repo; treat "STC" as a name
for this chassis shape, not as an expansion this doc can confirm.

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

## Wiring class

The GE-01 taxonomy every corpus record carries — `Display`, `Static`, `Derived`, `Computed` (a
strict lattice, highest-bar-wins), or `Ambiguous` — determined once, corpus-wide, by
`crates/codex-ingest/src/pcgen_import/wiring_class.rs` from a record's full token closure. See
[rules-data-tables.md](./rules-data-tables.md) §"`wiring_class` (GE-01 taxonomy)".

## See also

- [overview.md](./overview.md) — where most of these terms are first used in context.
- [getting-started.md](./getting-started.md) — the practical commands behind several of these terms.
- [README.md](./README.md) — the doc set's index.
