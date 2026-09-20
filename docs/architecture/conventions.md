# Conventions

> Scope: naming standards and cross-cutting code conventions — the rules a new contributor
> follows to name a file, a function, a test, a branch, or a commit the way this repo already
> does, plus the structural idioms every plane converges on independently.
> Last verified: **2026-09-20 against `tranche/16`** (SD-36 Epic D truth-up, HEAD `b22ea9e113` +
> this cycle's Epic C2 working-tree state — see `docs/architecture/README.md`'s provenance note).
> Maintenance: updated at SD closure — see [README.md](./README.md) §Maintenance contract

This is the doc to point an agent (or a new contributor) at for "what do I name this" and "how
do we do things here." [testing.md](./testing.md) is its sibling for everything about how this
repo tests a change; this file covers naming and the structural idioms a change should follow.
Every claim here cites at least one real, currently-existing file — check the citation, not the
prose, if you're unsure whether a rule still holds.

## Quick reference

| Surface | Pattern | Example |
|---|---|---|
| Root crate | `codex`, package at repo root, no `[workspace]` on itself | `Cargo.toml` |
| PCGen tool crate | `codex-ingest` under `crates/`, depends on `codex`, never the reverse | `crates/codex-ingest/Cargo.toml` |
| Desktop shell crate | `codex-desktop` under `apps/desktop/src-tauri`, path-deps on `codex`; `codex-ingest` only as a `[dev-dependencies]` | `apps/desktop/src-tauri/Cargo.toml` |
| Rules-engine module split | one `mod.rs` per family, `pub use self::{...}` re-exporting every submodule so call sites never change | `src/rules_core/pilot_compute/mod.rs` |
| Rust fn families | `compute_*`, `explain_*`/`explain_*_seam`, `load_*`, `build_*`, `list_*`, `supported_*_level` | `compute_total_saves`, `explain_dwarf_race_seam`, `load_character_input_fixture`, `build_pilot_headless_receipt` |
| Root integration test file | `<origin>_<full-sentence-claim>.rs`, flat under `tests/` | `tests/sd35_class_feature_catalogs_read_converted_prose.rs` |
| Test fn name | the claim itself, as a sentence, snake_case | `every_record_the_class_feature_catalogs_serve_has_converted_prose` |
| Corpus/rule id | `<book>:<kind>:<slug>`, opaque, slugged from source names | `crb:class:fighter`, `apg:feat:power_attack` |
| Tauri command | `snake_case` fn name, `#[tauri::command]` | `add_equipment_selection` (`character_hub.rs`) |
| TS boundary wrapper | camelCase file + fn, same name as its command, one per command family | `apps/desktop/src/boundary/addEquipmentSelection.ts` |
| React screen/panel | `*Screen.tsx` (routed views), `*Panel.tsx` (composed sub-views), colocated `*.test.ts` | `classCatalog/ClassCatalogScreen.tsx`, `*Panel.tsx` |
| CSS theme token | `--color-<role>` (semantic) mirrored under a theme-scoped `--wasp-color-<role>` alias | `--color-surface`, `--wasp-color-surface` (`apps/desktop/src/theme.css`) |
| Script | verb-first, hyphenated shell/`.mjs`, underscored Python module | `scripts/gen-corpus-bundle.mjs`, `scripts/denominator_gate.py` |
| Release package dir | `docs/release/SD-N-<slug>/` with the fixed chassis file set | `docs/release/SD-36-consolidation/` |
| Release/tranche branch | `tranche/N` | `tranche/16` |
| Ad-hoc work branch | `<type>/<slug>` | `fix/trait-choice-set-id-roundtrip` |
| Commit subject | `<type>(sdNN[,epic-x]): <what, present tense>` | `refactor(sd36,epic-c1): split pilot_compute into submodules` |

## Naming standards

### Crates, modules, and files

The repo is three Cargo packages, not one workspace member list you'd guess from `src/`:

- **`codex`** (root `Cargo.toml`) — the rules engine and persistence layers (`src/rules_core/`,
  `src/saved_character/`, `src/campaign/`, `src/homebrew_authoring/`, `src/support/`). It has no
  `[workspace]` table of its own; the *root* `Cargo.toml` (a separate file position) declares
  `[workspace] members = ["crates/*"]` with **no `default-members`**, so a bare `cargo build`/
  `cargo test` at repo root builds only `codex` (`Cargo.toml`'s own comment states this is
  deliberate — SD-36 Epic A's "wall's cheapest proof").
- **`codex-ingest`** (`crates/codex-ingest/Cargo.toml`) — the PCGen `.pcc`/`.lst` converter and
  oracle-parity harness. Depends on `codex`; `codex` never depends on it. This is the PCGen wall:
  see [corpus-ingest.md](./corpus-ingest.md) for what lives here and why.
  `crates/codex-ingest/tests/` holds the corpus-conversion integration suites moved out of root
  `tests/*.rs` by SD-36 Epic A.
- **`codex-desktop`** (`apps/desktop/src-tauri/Cargo.toml`) — the Tauri shell. Path-deps on
  `codex` for normal builds; `codex-ingest` is declared **only** under `[dev-dependencies]`
  (`cargo tree --locked -e normal,build` in `apps/desktop/src-tauri` must show zero
  `codex-ingest` lines — the `crate-wall` verify stage checks this structurally, not just by
  convention).

**A large module that outgrows one file becomes a directory named after the original file, with
`mod.rs` re-exporting every submodule.** `src/rules_core/pilot_compute/mod.rs` (SD-31: became a
directory; SD-36 Epic C1: split its once-88,828-line body into ~36 `class_*.rs`/topic-named
private submodules — `class_barbarian.rs`, `feat_pillars.rs`, `race_seams.rs`, `combat.rs`, and
siblings) is the exemplar: every submodule is a private `mod foo;`, and `mod.rs` closes with
`pub use self::{foo::*, bar::*, ...};` so all ~62 external call sites (`pilot_compute::compute_x`)
keep working unchanged. **When a module file crosses roughly 5-6k lines**, split it this way —
by class/topic, never by "first half / second half" — and re-export from `mod.rs` rather than
making callers learn the new submodule paths.

**Shared path/filesystem helpers live in exactly one place per crate, not duplicated per file.**
`src/support/paths.rs` (`repo_root`, `corpus_root`, `corpus_root_if_set`, `pcgen_corpus_root`,
`find_json_files`) replaced six to eight byte-identical copies that had drifted independently
across `src/rules_core/` (SD-36 Epic C1.3; `git grep -c 'fn repo_root' -- src` must show exactly
1 hit). `tests/support/paths.rs` is the parallel set for integration tests (brought in per-file via
`#[path = "support/paths.rs"] mod paths;`, because a `tests/*.rs` binary compiles as its own crate
and cannot reach `codex`'s `pub(crate)` items) — it replaced 32 local copies across 26 files (SD-36
Epic C2.3; as of this pass it is real and in use but not yet committed to git — see
[testing.md](./testing.md) §"Path helpers for tests" for that caveat). **When you need a
repo-root-relative path in a new file, `use` one of these — do not write a ninth copy of
`PathBuf::from(env!("CARGO_MANIFEST_DIR"))`.**

### Rust function families

Five recurring name prefixes carry specific meaning across `src/`:

- **`compute_*`** — pure calculation, returns a value plus (per the fail-honest pattern below) the
  proof of how it was derived. E.g. `compute_total_saves` (`pilot_compute/feat_pillar_and_pool_aggregation.rs`).
- **`explain_*` / `explain_*_seam`** — produces `ComputationExplanation` records for one bounded
  slice of rules support; the `_seam` suffix names a race/class boundary explicitly grounded and
  explicitly limited. E.g. `explain_dwarf_race_seam`, `explain_human_pilot_race_seam`
  (`pilot_compute/race_seams.rs`).
- **`load_*`** — parses an on-disk or in-memory representation into a typed record, no network,
  read-only. E.g. `load_character_input_fixture` (`src/rules_core/character_input.rs`).
- **`build_*`** — assembles a composite output (often a receipt or DTO) from already-computed
  pieces. E.g. `build_pilot_headless_receipt` (`pilot_compute/class_shared_core.rs`).
- **`list_*`** — enumerates a persisted collection; see the `list_all` idiom below for the exact
  missing-root/isolated-corrupt-entry contract every `list_*` should match.
- **`supported_<class>_level(input) -> Option<u8>`** — the gate half of the gate-then-explain
  pairing (see below): answers "is this input in the grounded population" before an `explain_*`
  is even attempted.

**When adding a new computed field or seam, name the function by which of these five things it
does** — a function that both loads and computes, for example, is a sign the load half belongs in
its own `load_*` function first.

### Types, constants, and ids

- **Opaque product ids are strings, never re-derived from display text.** A rule id is
  `"<book>:<kind>:<slug>"` (`src/rules_core/sheet_rule.rs`'s `RuleId` — e.g. `crb:class:fighter`),
  the same id `docs/work-inventory.json` keys a unit by. A converter-minted variable id is `"v"` +
  16 hex of SHA-256 over the upper-cased source name (`sheet_rule::var_id`) — a pure hash that
  carries no source-format name outside `provenance`.
- **A "book id" is the `rules_tables` module name, not always the literal book title.** Each
  Paizo book gets its own directory under `src/rules_core/rules_tables/` (`crb`, `apg`, `acg`,
  `bestiary_2` .. `bestiary_6`, `inner_sea_world_guide`, ...) and a matching `RuleSetId` variant
  documented in `rules_tables/mod.rs`. **Known exception, kept for compatibility rather than
  fixed**: `beastiary1` is a legacy misspelling of "bestiary 1" that predates the convention and
  is not renamed, because the id is load-bearing (persisted data references it) — treat an
  established book id as a proper noun, not a typo to correct in passing.
- **Corpus identifier scope is per-package, not global.** Two books can use the same short name
  for unrelated things; `src/rules_core/corpus_loader.rs` and sibling resolvers key records by
  `(source_package_id, kind, slug)`, never by slug alone. When adding a new book, verify its ids
  don't silently collide with an existing one under the same kind before assuming a shared name
  means a shared thing.
- **Const names are `SCREAMING_SNAKE_CASE`** for allowlists and fixed tables — e.g.
  `MARTIAL_CLASS_NAMES`, `SPELLCASTING_CLASS_NAMES` (see the allowlist-widening idiom below).

### Test file and function names

Covered in full in [testing.md](./testing.md) §Test conventions; the naming rule specifically:

- **A `tests/*.rs` file name is a claim about originating slice**, `<origin>_<subject>.rs` — e.g.
  `tests/sd35_class_feature_catalogs_read_converted_prose.rs`,
  `tests/derived_evaluator_fixture_check_monster_sla.rs`. `<origin>` is the `sdNN`/`geNN`/`atNN`
  bundle prefix, or a bare descriptive name for cross-cutting audits with no single origin
  (`no_foreign_home_paths.rs`, `pi_table_sweep.rs`).
- **A test *function* name is the claim itself, written as a full sentence in snake_case**, not an
  abbreviation of it: `every_record_the_class_feature_catalogs_serve_has_converted_prose`,
  `no_record_the_class_feature_catalogs_serve_prints_a_stub_marker`
  (`tests/sd35_class_feature_catalogs_read_converted_prose.rs`). A reader should be able to tell
  what broke from `cargo test`'s failure line alone, without opening the file.
- **A large templated family becomes one binary with a roster, not one file per case.**
  `tests/sd18_widening/main.rs`'s `roster!` macro declares one `mod <class>_level<N>;` per row,
  each still its own file with its own `#[test]` fns (unchanged from before the fold) — the fold
  is the one binary/one link, not a rewrite of the individual case files. See
  [testing.md](./testing.md)'s tax-cut section for why this matters at build-time scale.

### Tauri commands and their TypeScript boundary twins

- **Command names are `snake_case`, matching the Rust fn wrapped by `#[tauri::command]`** — e.g.
  `create_character`, `add_equipment_selection`, `level_up_character`
  (`apps/desktop/src-tauri/src/character_hub.rs`).
- **Every command family gets exactly one `apps/desktop/src/boundary/<commandNameCamelCase>.ts`
  wrapper**, named by converting the command's `snake_case` to `camelCase` — `add_equipment_selection`
  → `boundary/addEquipmentSelection.ts`, exporting an `addEquipmentSelection()` function that calls
  `invoke('add_equipment_selection', { request })` with the **literal snake_case command string**
  passed to `invoke()`, never a re-derived or partial name. The wrapper is gated on
  `boundary/runtime.ts`'s `hasTauriRuntime()` and throws a named `Error` on failure — never returns
  a silently-empty success. See the Boundary wrapper rule idiom below for the three named
  exceptions to "one wrapper per command."
- A sibling `<commandNameCamelCase>.test.ts` lives next to the wrapper when it has meaningful
  logic to unit-test (most do): `boundary/addEquipmentSelection.test.ts`.

### React file naming

- **`*Screen.tsx`** — a routed, top-level view the app's navigation renders directly (e.g. a
  `classCatalog/ClassCatalogScreen.tsx`-shaped file per feature area). **Exception:** the
  character-hub feature's own top-level component is `characterHub/CharacterHubPage.tsx` —
  named `*Page`, not `*Screen`, predating this convention; its sibling routed views in the same
  directory (`LandingScreen.tsx`, `LoadCharacterScreen.tsx`, `StubScreen.tsx`) do follow it. Do
  not use `CharacterHubPage.tsx` as the pattern to copy for a NEW feature area — copy
  `ClassCatalogScreen.tsx` or one of the other `*CatalogScreen.tsx` files instead.
- **`*Panel.tsx`** — a composed sub-view embedded inside a screen, not independently routed.
- **`*.test.ts`** (not `.test.tsx`, even for a component's test) — self-executing test files under
  `apps/desktop/src/**`, run by `apps/desktop/scripts/run-tests.mjs` (see
  [testing.md](./testing.md)). Colocated next to the file under test.
- Feature areas are directories named after the domain, camelCase (`characterHub/`,
  `classCatalog/`, `settings/`), each holding its `*Screen.tsx`/`*Panel.tsx` files, its
  `compose*`/`build*Surface` pure builders, and its `*Runtime.ts` DI seam together — see the
  `build*Surface`/`*Runtime` idiom below.

### CSS / theme tokens

`apps/desktop/src/theme.css` defines semantic custom properties `--color-<role>` (`--color-surface`,
`--color-text`, `--color-border`, `--color-accent`, `--color-error`, `--color-warn`, plus `-hover`/
`-bg`/`-border`/`-muted`/`-faint`/`-secondary`/`-strong` role suffixes) and mirrors every one of
them under a `--wasp-color-<role>` alias of the identical name for the community-theme surface
(`apps/desktop/src/settings/communityTheme.ts`, `obsidianThemeCatalog.ts`). **When adding a new
semantic color**, add both the `--color-*` and `--wasp-color-*` entries together — a token defined
in only one namespace is invisible to whichever consumer reads the other one.

### Scripts

- **Shell and Node scripts are hyphenated, verb-first**: `scripts/gen-corpus-bundle.mjs`,
  `scripts/fetch-pcgen-oracle.sh`, `scripts/reclaim.sh`.
- **Python modules are underscored** (PEP 8): `scripts/denominator_gate.py`,
  `scripts/pcgen_residue_gate.py`, `scripts/box_ledger.py`.
- **A generator's language follows what its output must run on, not the author's default.**
  `scripts/gen-corpus-bundle.mjs` is Node, not Python, specifically because
  `.github/workflows/publish-tester-release.yml` builds the desktop app on `ubuntu-latest`,
  `macos-latest`, and `windows-latest` runners that all guarantee Node (via `npm ci`) but do not
  all pin Python (`decisions.md` §8). Check what the consuming CI job already has before picking a
  script's language.
- **A tool that must name forbidden vocabulary to remove it cannot live under a directory that
  vocabulary check also scans.** `scripts/gen-corpus-bundle.mjs` lives at repo-root `scripts/`, not
  `apps/desktop/scripts/`, because it is a `LIVE_ROOT` for the PCGen residue gate with no
  carve-outs, and the generator's own source must name PCGen token patterns (`raw_tokens`,
  `BONUS:`, `PRE[A-Z]+:`) to strip them — verified directly: an earlier draft under
  `apps/desktop/scripts/` made the gate fail on the generator's own pattern array, not on anything
  it copied (`decisions.md` §8). **When a script must reference forbidden syntax to enforce a
  wall against it, place it outside every directory that wall scans.**

### Docs and release-package file names

- **A bundle's canonical surface is `docs/release/SD-N-<slug>/`** with a fixed chassis:
  `README.md` (index), `scope-draft.md` (handoff), `epic-breakdown.md` (acceptance criteria per
  epic), `technical-design.md` (architectural rationale), `decisions.md` (operator ADRs),
  `workflow-instruction.md` (per-cycle dispatch procedure), `kanban.md` (epic status board),
  `progress.md`, `receipts.md` (cycle/graphify receipts), `release-notes.md`,
  `artifacts/<epic-slug>/` (per-epic working evidence). See `docs/release/SD-36-consolidation/`
  for a live example of the full set.
- **`docs/architecture/*.md`** describes current-state function only — see
  [README.md](./README.md)'s Maintenance contract for the update-on-PR procedure and the rule
  that obsolete statements are removed, never appended-to with a changelog line.
- **`docs/retro/*.md`** and `docs/retro/events/*.jsonl` are historical and dated — the one place
  in this repo's docs where "what used to be true" is the point, not a defect.

### Branches, commit scope, and PR titles

- **A release/tranche branch is `tranche/N`** (`tranche/16`, current), cut once per SD-N bundle,
  merged to `develop` at closure. Never delete a merged tranche branch's remote sibling casually —
  see `docs/architecture/release-pipeline.md` for the branch-promotion gate this feeds.
- **An ad-hoc work branch is `<type>/<slug>`**, mirroring the commit-type vocabulary below:
  `fix/trait-choice-set-id-roundtrip`, `fix/ci-fetch-pcgen-oracle`, and a `docs/`-prefixed slug for
  a documentation-only correction.
- **Commit subjects are `<type>(<scope>): <what, present tense, no trailing period>`.** `<type>` is
  one of `feat`/`fix`/`docs`/`refactor`/`chore` (conventional-commit vocabulary); `<scope>` names
  the originating bundle and, where useful, the epic or area: `refactor(sd36,epic-c1): split
  pilot_compute into submodules, consolidate path helpers, lock clippy to -D warnings`,
  `fix(sd36,desktop): ship a generated PCGen-free corpus bundle`. A scope-less commit (`docs(sd36):
  mark C1 cycle complete...`) is fine once the bundle id alone disambiguates enough.
  **When adding to the commit body**: state what changed and why in the imperative, not a diff
  recap — the subject line already says what files moved.
- **A tranche-closure PR title names the bundle and its headline claim in plain language**, not
  the conventional-commit grammar: `"SD-35 — corpus sheet completion: every record renders a sheet
  line; PCGen out of live code"`. Smaller PRs (a single fix, not a bundle closure) use the same
  `<type>(<scope>): <what>` grammar as their commit subject.

### Diagnostics and errors: loud, named, never silently empty

- **A diagnostic names the exact missing piece, not a generic failure.** `perform_install`
  (`apps/desktop/src-tauri/src/update/transaction.rs`) always returns `Err("perform_install is
  registered but not wired: downloading the AppImage artifact requires an HTTP client this crate
  does not carry as a dependency yet; ...")` — see the Honest stubs idiom below.
- **A store/loader that hits a missing environment precondition says which one, with the fix
  command**, rather than returning an empty result that looks like "no data yet." `preflight-oracle`
  (`scripts/verify.sh`) fails RED with the exact `scripts/fetch-pcgen-oracle.sh` invocation when the
  pinned PCGen checkout is absent — never a silently-skipped corpus-gated suite pretending to be
  green.
- **`unwrap`/`expect`/`panic!` state what invariant failed, in the message**, not just that one
  did — `pcgen_corpus_root()`'s `.expect("HOME must be set to locate the pinned corpus checkout")`
  (`src/support/paths.rs`) is the pattern: a future reader hitting the panic learns the missing
  precondition immediately.
- **An unknown/malformed input is a pushed diagnostic, not a dropped line.** `apply_fixture_field`'s
  `unknown` match arm (`src/rules_core/character_input.rs`) records a diagnostic for an unrecognized
  fixture key rather than silently ignoring it — see the fixture grammar in
  [testing.md](./testing.md).

## Doctrine every change must respect

| Doctrine | One-line rule | Enforced by |
|---|---|---|
| No-stub / Wired Integration | Shipping code paths do what they claim; a deliberate stub names the exact missing piece in its own return value. | `docs/governance/no-stub-mvp-doctrine.md`; `wired-integration-discipline` skill; `tests/sd24_wired_integration_audit.rs` |
| PCGen wall | Nothing under `src/rules_core`, `apps/desktop/**`, or other `LIVE_ROOT`s carries `raw_tokens`, `PRE...:`, `%LIST...`, or other PCGen token syntax; the converter/oracle live only in `crates/codex-ingest`. | `python3 scripts/pcgen_residue_gate.py --check`; `crate-wall` verify stage |
| Print-the-rule, not simulate | The sheet-rule engine renders the FINAL number, dice expression, or the rule's own words a player writes on a paper sheet — it does not run a game-state simulation or model consumer-by-consumer deltas. | `src/rules_core/sheet_rule.rs`'s module doc; `sheet-rules-check` verify stage |
| Generated artifacts are not hand-edited | A file a script produces (`docs/work-inventory.json`, `site/status-data.json`, the corpus bundle under `apps/desktop/src-tauri/resources/corpus_bundle/`) is only ever changed by re-running its generator, never by direct edit — a hand-edit invalidates the regeneration proof. | Each generator's own `--check` mode; `corpus-bundle` verify stage |
| Clean tree means unfiltered `git status` empty | "Clean" is the literal, unfiltered output of `git status`, not that output with known-noise lines grepped away. Untracked scratch, retro-log shards, and dispatch scripts all get folded into the commit or removed before calling a tree clean. | Human/agent discipline; checked at every wave/cycle commit |
| One writer per shared tree | Two agents never hold uncommitted work in the same working tree; `git status --porcelain` before your first write, and stop if it lists a file you didn't touch. | `AGENTS.md` §Concurrency and Measurement |
| Every figure states its denominator and re-derive command | A percentage or "N of M" claim in a doc or receipt carries, on the same line or in the same construct, the population it's over and (in a "Figures" section) the exact command that reproduces it. | `python3 scripts/denominator_gate.py --check` / `--check-provenance` |

## Idiom catalog

Each entry below names one recurring structural pattern, cites at least one real file, and states
the rule for adding new code in that shape.

### Fail-honest computation

Every computed value carries an explanation record proving how it was derived; every diagnostic
carries `claim_blocking: bool`; a computation is blocked iff at least one claim-blocking
diagnostic exists. Nothing fabricates a value it cannot prove. See `compute_total_saves`
(`src/rules_core/pilot_compute/feat_pillar_and_pool_aggregation.rs`),
`build_pilot_headless_receipt` (`src/rules_core/pilot_compute/class_shared_core.rs`), and
`printed_sheet_cell_map` in `src/rules_core/contract.rs`. **When adding a new computed field**:
push a real explanation record on the supported path, or a named claim-blocking diagnostic and a
zeroed/absent value on the unsupported path — never a value with no explanation. Full treatment in
[rules-engine.md](./rules-engine.md) §"The fail-honest pattern."

### Concrete zero-field `*Store` structs, no `*Backend` trait

Persistence stores are concrete, zero-field structs with associated functions, not `dyn *Backend`
trait objects. `src/campaign/local_store.rs`'s module doc comment states the rule directly: no
`*Backend` trait exists anywhere in this codebase, because there is no second backend to justify
trait-object indirection. See `SavedCharacterStore` (`src/saved_character/local_store.rs`) and
`CampaignStore` (`src/campaign/local_store.rs`). **When adding a new persistence backend**: do not
introduce a trait/dyn-dispatch seam until a second concrete implementation genuinely needs to be
swapped at runtime.

### Validate-before-persist

A store refuses to write a record it cannot honestly read back. `SavedCharacterStore::save` calls
`validate_character_input` (rejects newlines, enforces the fixture grammar's colon-segment shape)
before writing; `PackageStore::save` calls `validate_persistable` for the same reason (both in
`src/saved_character/local_store.rs` and `src/homebrew_authoring/package_store.rs` respectively).
**When adding a new on-disk write path**: validate the in-memory record against the exact grammar
your own loader will re-parse, before writing any file.

### The `list_all` idiom

`SavedCharacterStore::list_all` and `CampaignStore::list_all` (`src/saved_character/local_store.rs`,
`src/campaign/local_store.rs`) both follow: a missing root directory returns an empty listing, not
an error (no records yet is not a failure); one unreadable subdirectory is isolated into
`*Listing::unreadable_entries` without failing the rest of the listing. **When adding a new
`list_all`-shaped function**: match this exact missing-root/isolated-corrupt-entry behavior rather
than propagating the first error.

### Two distinct hand-rolled fixture grammars

Two separate line-oriented text grammars coexist, and they are not interchangeable.
`src/rules_core/character_input.rs`'s `load_character_input_fixture` (also used by
`saved_character` and `oracle_validation::golden_fixture`) reads flat `key=value` lines, no
nesting. `src/homebrew_authoring/package_store.rs`'s `render_manifest`/`parse_manifest` reads a
YAML-like `key: value` / `- item` grammar with indentation-sensitive list sections
(`ManifestSection`/`RecordListSection`), no `=`. **When adding a new file format**: pick the
grammar that matches the consumer it mirrors — don't invent a third, and don't assume the two
existing ones share a parser.

### `build*Surface`/`*Runtime` DI + browser-preview fallback

Nearly every desktop screen pairs a pure `build*Surface`/`compose*` function (no I/O, fully
unit-testable) with a `*Runtime.ts` module that supplies the real boundary loader under a Tauri
runtime and a hardcoded preview fallback otherwise. Screens call only the `*Runtime` function. See
`characterHub/composeCreateCharacterRequest.ts` + `characterHub/characterHubRuntime.ts`, and
`classCatalog/classCatalogRuntime.ts`'s `buildPreviewCatalog()` fallback. **When adding a new
screen with backend data**: write the pure builder first, then the `*Runtime` DI seam, then wire
the screen to the runtime only — never call `invoke()` or a boundary file from inside a component.
Full worked example in [desktop-app.md](./desktop-app.md) §"The surface/runtime DI pattern."

### Boundary wrapper rule

Components never call `invoke()` inline; each Tauri command family gets one wrapper under
`apps/desktop/src/boundary/` (74 files as of this verification —
`ls apps/desktop/src/boundary/*.ts | wc -l`, wrapper + test pairs), gated on `boundary/runtime.ts`'s
`hasTauriRuntime()`. **When adding a new Tauri command**: add a `boundary/<command>.ts` wrapper
unless you have a testability seam like one of the three verified direct-`invoke()` exceptions, in
which case say so in a comment. Full accounting of the three named exceptions and exactly what
testability seam each one substitutes for the missing wrapper:
[desktop-app.md](./desktop-app.md) §"The boundary rule."

### Command / pure-fn split

Tauri commands are thin `#[tauri::command]` shims over a unit-testable `foo_impl` core. See
`perform_restore_previous` → `perform_restore_previous_impl`, `perform_retention_sweep` →
`perform_retention_sweep_impl` (both `apps/desktop/src-tauri/src/update/transaction.rs`). **When
adding a new command**: write the `_impl` function first, test it without Tauri, then add the
one-line `#[tauri::command]` wrapper.

### `makeSurface`/`make*` canonical-fixture factories

A single exported factory returns one complete, valid object; tests shallow-spread `overrides` on
top rather than each carrying its own copy. See `apps/desktop/src/testSupport/makeSurface.ts`
(built specifically because independent copies drifted when a new required field landed) and
`makeCharacterSummary.ts`. **When a DI surface needs a test fixture**: add one `make*` factory
under `testSupport/`, not a per-test-file literal. Full treatment in [testing.md](./testing.md)
§"Desktop test support."

### Allowlist widening pattern

`MARTIAL_CLASS_NAMES`/`SPELLCASTING_CLASS_NAMES` (`crates/codex-ingest/src/pcgen_import/lst_parser/class.rs`,
`crates/codex-ingest/src/pcgen_import/lst_parser/spellcasting_class.rs` — moved here from a former
root src/pcgen_import path by the SD-36 Epic A crate wall) widen one class at a time, each verified
against the real corpus `CLASS:` line shape before being added; a class outside the current
allowlist is skipped silently, not mis-parsed. **When adding a class to an allowlist**: verify its
real corpus record shape first, and put it on the correct list (martial vs. spellcasting) — the
module doc comments call this a correctness bug class of its own if gotten wrong.

### Guard-then-dispatch resolver shape

Every `rules_tables` book resolver starts with `if rule_set != RuleSetId::X { return None; }`
before dispatching on the book-local id enum. See `apg::class_chassis_resolve`,
`acg::class_chassis_resolve`, `beastiary1::monster_resolve` (all under
`src/rules_core/rules_tables/`). **When adding a new book or resolver**: copy this guard-then-dispatch
shape so a wrong-book query is a defined `None`, never a panic or silent wrong answer. Full
treatment, including the cross-book acceptance-test pattern, in
[rules-data-tables.md](./rules-data-tables.md) §"`RuleSetId` and per-book resolution."

### Gate-then-explain pairing

Per-class/per-race compute functions in `src/rules_core/pilot_compute/` (now split across its
`class_*.rs`/`race_seams.rs` submodules — see §Naming standards above) pair a
`supported_<class>_level(input) -> Option<u8>` gate with an `explain_*`/`compute_*` function that
either produces real explanation records or pushes a named claim-blocking diagnostic and stops.
**When adding a new class/race/level band**: add the gate function first, then the explain
function, following an existing pair (e.g. `supported_fighter_level`/`explain_fighter_class_features`)
rather than open-coding a new shape. Full treatment in [rules-engine.md](./rules-engine.md)
§"The compute spine, end to end."

### Adapt doctrine types to real codebase shape

Where an upstream doctrine artifact's illustrative type signature doesn't match this repo's real
modules (e.g. a generic `RulesTables` indirection type that doesn't exist here), the module doc
comments in `src/rules_core/contract.rs` and elsewhere state explicitly that the doctrine's shape
was adapted, not imported verbatim. **When implementing against a doctrine/spec artifact**: adapt
its types to the codebase's real shape and document the deviation in a doc comment — don't invent
a parallel type just to match an illustrative signature literally.

### Corpus-gated test pattern — prefer graceful-skip for new tests

Two patterns coexist, both now under `crates/codex-ingest/tests/` (moved there by SD-36 Epic A):
`#[ignore]`-attributed hard-skip (majority of `sd22_*_resolves.rs`) and plain-`#[test]`
runtime-existence-check with `eprintln!` graceful skip (`sd17_b5_equipment.rs`,
`sd17_b_races_and_abilities.rs`'s `corpus_root()` helper). **New corpus-gated tests should
prefer the graceful-skip variant** — it runs clean under a plain `cargo test --locked` with no
extra flags — unless you are adding to a file that already uses `#[ignore]`, in which case match
that file's existing pattern. Full detail in [testing.md](./testing.md) §"Corpus-gated tests."

### Provenance naming (`sdNN`/`geNN` prefixes)

Module and test file names carry the originating spec-domain/grand-epic bundle as a prefix
(`authoring_workbench.rs`, `tests/sd25_sorcerer_level_up_explanation_coverage.rs`) — one behavior
per file. These prefixes are proper nouns naming provenance, not documentation of current function
(see [README.md](./README.md)'s provenance note). **When adding a new test or module born from a
specific bundle**: prefix it the same way so `grep`-by-origin keeps working, and write the file to
prove exactly one behavior. Note the prefix convention is applied to *newly born* files, not
retroactively frozen: SD-25 criterion 1.1 renamed several already-shipped frontend surfaces whose
`sdNN` prefix had become misleading as living identifiers (`sd11/` → `testerWorkbench/`, `sd15/`
→ `operatorTriage/`, `sd22/` → `releaseChecks/`, `sd13_support_state_matrix.rs` →
`support_state_matrix_bridge.rs`, itself retired entirely by SD-36 D3) — the originating bundle is
recorded in each file's own doc comment instead.

### TDD / red-green mandate

`AGENTS.md`'s non-negotiable rule 1: write or update a failing test before changing production
code, confirm it fails for the intended reason, then implement the smallest change to pass. **This
applies to every plane** — Rust, TypeScript, and the standalone Python/bash release scripts alike.
Full treatment, including non-Rust examples of the same discipline, in
[testing.md](./testing.md) §"Test conventions."

### Honest stubs

A stub must say it's a stub in its own return value or comment, not just in a doc comment nobody
reads at runtime. The exemplar is `perform_install`
(`apps/desktop/src-tauri/src/update/transaction.rs`): it always returns `Err("perform_install is
registered but not wired: downloading the AppImage artifact requires an HTTP client this crate
does not carry as a dependency yet; ...")` — the caller sees the exact missing dependency, not a
generic failure. **When landing a deliberate stub**: make its error/return value name the specific
missing piece, the same way. Operator-granted exceptions to the wider no-stub doctrine are tracked
in `docs/governance/wired-integration-stubs-registry.md`, never landed silently.

## How to extend

Adding a new idiom-catalog entry or naming rule to this file: find the closest existing section
(naming standard vs. doctrine vs. idiom), add your entry in the same shape as its neighbors —
one-paragraph rule, at least one real file citation, a bolded **When adding...** imperative — and
cross-link to the sibling doc that carries the narrative version if one exists (usually
[rules-engine.md](./rules-engine.md), [desktop-app.md](./desktop-app.md), or
[testing.md](./testing.md)). Worked example: SD-36 Epic C1's `src/support/paths.rs` consolidation
added both a "Crates, modules, and files" naming rule above (the module-split shape) and the path-
helper paragraph — the same PR that changed the code updated this file in the same commit family,
per the README's update-on-PR procedure.

## Pitfalls

- **Citing a path without checking it still exists.** This doc's own prior verification (2026-08-22)
  cited a stale single-file path for `pilot_compute` after it had already become a directory
  during SD-31 (and was split further into ~36 submodules by SD-36 Epic C1); the correction
  shipped a full cycle later. Run the cited-path verification one-liner in [README.md](./README.md)
  before treating any citation here as current.
- **Freezing a provenance prefix as a status flag.** `sdNN`/`geNN` name *who wrote this*, not
  *what state it's in* — do not read `sd13_*` as "old" or `sd36_*` as "current"; check the code.
- **Adding a `*Backend` trait "for future flexibility."** This repo has shipped every persistence
  surface as a concrete struct for its entire history; a trait with one implementor is pure
  indirection here, not a convention to follow.
- **Duplicating a path helper instead of importing `src/support/paths.rs` / `tests/support/paths.rs`.**
  This is exactly the defect SD-36 Epic C1/C2 spent two full passes cleaning up (eight-plus and
  32 independently-drifted copies, respectively) — a tenth copy recreates the problem those passes
  just closed.
- **Writing a new PCGen-vocabulary-scanning tool under a directory that vocabulary check also
  scans.** See `scripts/gen-corpus-bundle.mjs`'s placement rationale above — verify a new
  generator's directory against `python3 scripts/pcgen_residue_gate.py --check` before trusting it
  passes.
