# Conventions

> Scope: the repo-wide idiom catalog — the structural patterns every plane converges on independently, and what to do when adding new code.
> Last verified: 2026-07-22 against tranche/5-3 (SD-25 closure). **Path correction 2026-08-22**
> (SD-32 closure epilogue): pilot_compute.rs (old path src/rules_core/pilot_compute.rs, no longer valid) updated to
> `src/rules_core/pilot_compute/mod.rs` — the module became a directory during SD-31; no other
> content in this doc re-verified.
> Maintenance: updated at SD closure — see [README.md](./README.md) §Maintenance contract

This is the doc to point an agent (or a new contributor) at for "how do we
do things here." Each section names one recurring idiom, cites at least one
real file, and states the rule to follow when adding new code. For the
narrative version of any one idiom, follow the linked sibling doc.

## Fail-honest computation

Every computed value carries an explanation record proving how it was
derived; every diagnostic carries `claim_blocking: bool`; a computation is
blocked iff at least one claim-blocking diagnostic exists. Nothing fabricates
a value it cannot prove. See `compute_total_saves` and
`build_pilot_headless_receipt` in `src/rules_core/pilot_compute/mod.rs`, and
`printed_sheet_cell_map` in `src/rules_core/contract.rs`. **When adding a new
computed field**: push a real explanation record on the supported path, or a
named claim-blocking diagnostic and a zeroed/absent value on the unsupported
path — never a value with no explanation. Full treatment in
[rules-engine.md](./rules-engine.md) §"The fail-honest pattern."

## Concrete zero-field `*Store` structs, no `*Backend` trait

Persistence stores are concrete, zero-field structs with associated
functions, not `dyn *Backend` trait objects. `src/campaign/local_store.rs`'s
module doc comment states the rule directly: no `*Backend` trait exists
anywhere in this codebase, because there is no second backend to justify
trait-object indirection. See `SavedCharacterStore`
(`src/saved_character/local_store.rs`) and `CampaignStore`
(`src/campaign/local_store.rs`). **When adding a new persistence backend**:
do not introduce a trait/dyn-dispatch seam until a second concrete
implementation genuinely needs to be swapped at runtime.

## Validate-before-persist

A store refuses to write a record it cannot honestly read back.
`SavedCharacterStore::save` calls `validate_character_input` (rejects
newlines, enforces the fixture grammar's colon-segment shape) before writing;
`PackageStore::save` calls `validate_persistable` for the same reason (both
in `src/saved_character/local_store.rs` and `src/homebrew_authoring/package_store.rs`
respectively). **When adding a new on-disk write path**: validate the
in-memory record against the exact grammar your own loader will re-parse,
before writing any file.

## The `list_all` idiom

`SavedCharacterStore::list_all` and `CampaignStore::list_all`
(`src/saved_character/local_store.rs`, `src/campaign/local_store.rs`) both
follow: a missing root directory returns an empty listing, not an error (no
records yet is not a failure); one unreadable subdirectory is isolated into
`*Listing::unreadable_entries` without failing the rest of the listing.
**When adding a new `list_all`-shaped function**: match this exact
missing-root/isolated-corrupt-entry behavior rather than propagating the
first error.

## Two distinct hand-rolled fixture grammars

Two separate line-oriented text grammars coexist, and they are not
interchangeable. `src/rules_core/character_input.rs`'s
`load_character_input_fixture` (also used by `saved_character` and
`oracle_validation::golden_fixture`) reads flat `key=value` lines, no
nesting. `src/homebrew_authoring/package_store.rs`'s `render_manifest`/
`parse_manifest` reads a YAML-like `key: value` / `- item` grammar with
indentation-sensitive list sections (`ManifestSection`/`RecordListSection`),
no `=`. **When adding a new file format**: pick the grammar that matches the
consumer it mirrors — don't invent a third, and don't assume the two
existing ones share a parser.

## `build*Surface`/`*Runtime` DI + browser-preview fallback

Nearly every desktop screen pairs a pure `build*Surface`/`compose*` function
(no I/O, fully unit-testable) with a `*Runtime.ts` module that supplies the
real boundary loader under a Tauri runtime and a hardcoded preview fallback
otherwise. Screens call only the `*Runtime` function. See
`characterHub/composeCreateCharacterRequest.ts` +
`characterHub/characterHubRuntime.ts`, and
`classCatalog/classCatalogRuntime.ts`'s `buildPreviewCatalog()` fallback.
**When adding a new screen with backend data**: write the pure builder first,
then the `*Runtime` DI seam, then wire the screen to the runtime only — never
call `invoke()` or a boundary file from inside a component. Full worked
example in [desktop-app.md](./desktop-app.md) §"The surface/runtime DI
pattern."

## Boundary wrapper rule

Components never call `invoke()` inline; each Tauri command family gets one
wrapper under `apps/desktop/src/boundary/`, gated on
`boundary/runtime.ts`'s `hasTauriRuntime()`. **When adding a new Tauri
command**: add a `boundary/<command>.ts` wrapper unless you have a
testability seam like one of the three verified direct-`invoke()`
exceptions, in which case say so in a comment. Full accounting of the three
named exceptions and exactly what testability seam each one substitutes for
the missing wrapper: [desktop-app.md](./desktop-app.md) §"The boundary
rule."

## Command / pure-fn split

Tauri commands are thin `#[tauri::command]` shims over a unit-testable
`foo_impl` core. See `perform_restore_previous` →
`perform_restore_previous_impl`, `perform_retention_sweep` →
`perform_retention_sweep_impl` (both `apps/desktop/src-tauri/src/update/transaction.rs`).
**When adding a new command**: write the `_impl` function first, test it
without Tauri, then add the one-line `#[tauri::command]` wrapper.

## `makeSurface`/`make*` canonical-fixture factories

A single exported factory returns one complete, valid object; tests
shallow-spread `overrides` on top rather than each carrying its own copy.
See `apps/desktop/src/testSupport/makeSurface.ts` (built specifically because
independent copies drifted when a new required field landed) and
`makeCharacterSummary.ts`. **When a DI surface needs a test fixture**: add
one `make*` factory under `testSupport/`, not a per-test-file literal. Full
treatment in [testing.md](./testing.md) §"Desktop test support."

## Allowlist widening pattern

`MARTIAL_CLASS_NAMES`/`SPELLCASTING_CLASS_NAMES`
(`src/pcgen_import/lst_parser/class.rs`,
`src/pcgen_import/lst_parser/spellcasting_class.rs`) widen one class at a
time, each verified against the real corpus `CLASS:` line shape before being
added; a class outside the current allowlist is skipped silently, not
mis-parsed. **When adding a class to an allowlist**: verify its real corpus
record shape first, and put it on the correct list (martial vs.
spellcasting) — the module doc comments call this a correctness bug class of
its own if gotten wrong.

## Guard-then-dispatch resolver shape

Every `rules_tables` book resolver starts with `if rule_set != RuleSetId::X {
return None; }` before dispatching on the book-local id enum. See
`apg::class_chassis_resolve`, `acg::class_chassis_resolve`,
`beastiary1::monster_resolve` (all under `src/rules_core/rules_tables/`).
**When adding a new book or resolver**: copy this guard-then-dispatch shape
so a wrong-book query is a defined `None`, never a panic or silent wrong
answer. Full treatment, including the cross-book acceptance-test pattern, in
[rules-data-tables.md](./rules-data-tables.md) §"`RuleSetId` and per-book
resolution."

## Gate-then-explain pairing

Per-class/per-race compute functions in `src/rules_core/pilot_compute/mod.rs`
pair a `supported_<class>_level(input) -> Option<u8>` gate with an
`explain_*`/`compute_*` function that either produces real explanation
records or pushes a named claim-blocking diagnostic and stops. **When adding
a new class/race/level band**: add the gate function first, then the
explain function, following an existing pair (e.g.
`supported_fighter_level`/`explain_fighter_class_features`) rather than
open-coding a new shape. Full treatment in [rules-engine.md](./rules-engine.md)
§"The compute spine, end to end."

## Grounding-ref combined-literal idiom

`src/rules_core/support_state_matrix.rs`'s `grounding_ref` fields concatenate
every test file proving a row's claim with `+`-joined string literals (e.g.
`SD13_ROGUE_LEVEL1_TEST`), so each cited file stays independently
substring-checkable via `grounding_ref.contains(...)`. **When a widening
cycle adds new grounding evidence to an existing row**: extend the
`+`-joined literal, never replace or drop a prior citation that still holds.
Full treatment in [support-state-matrix.md](./support-state-matrix.md)
§"Row structure and the grounding-reference pattern."

## Adapt doctrine types to real codebase shape

Where an upstream doctrine artifact's illustrative type signature doesn't
match this repo's real modules (e.g. a generic `RulesTables` indirection type
that doesn't exist here), the module doc comments in `src/rules_core/contract.rs`
and elsewhere state explicitly that the doctrine's shape was adapted, not
imported verbatim. **When implementing against a doctrine/spec artifact**:
adapt its types to the codebase's real shape and document the deviation in a
doc comment — don't invent a parallel type just to match an illustrative
signature literally.

## Corpus-gated test pattern — prefer graceful-skip for new tests

Two patterns coexist: `#[ignore]`-attributed hard-skip (majority of
`sd22_*_resolves.rs`) and plain-`#[test]` runtime-existence-check with
`eprintln!` graceful skip (`tests/sd17_b5_equipment.rs`,
`tests/sd17_b_races_and_abilities.rs`'s `corpus_root()` helper). **New
corpus-gated tests should prefer the graceful-skip variant** — it runs clean
under a plain `cargo test --locked` with no extra flags — unless you are
adding to a file that already uses `#[ignore]`, in which case match that
file's existing pattern. Full detail in [testing.md](./testing.md) §"Corpus-
gated tests."

## Provenance naming (`sdNN`/`geNN` prefixes)

Module and test file names carry the originating spec-domain/grand-epic
bundle as a prefix (`authoring_workbench.rs`,
`tests/sd25_sorcerer_level_up_explanation_coverage.rs`) — one behavior per file. These
prefixes are proper nouns naming provenance, not documentation of current
function (see [README.md](./README.md)'s provenance note). **When adding a
new test or module born from a specific bundle**: prefix it the same way so
`grep`-by-origin keeps working, and write the file to prove exactly one
behavior. Note the prefix convention is applied to *newly born* files, not
retroactively frozen: SD-25 criterion 1.1 renamed several
already-shipped frontend surfaces whose `sdNN` prefix had become
misleading as living identifiers (`sd11/` → `testerWorkbench/`, `sd15/`
→ `operatorTriage/`, `sd22/` → `releaseChecks/`,
`sd13_support_state_matrix.rs` → `support_state_matrix_bridge.rs`) — the
originating bundle is recorded in each file's own doc comment instead.

## TDD / red-green mandate

`AGENTS.md`'s non-negotiable rule 1: write or update a failing test before
changing production code, confirm it fails for the intended reason, then
implement the smallest change to pass. **This applies to every plane** —
Rust, TypeScript, and the standalone Python/bash release scripts alike. Full
treatment, including non-Rust examples of the same discipline, in
[testing.md](./testing.md) §"Test conventions."

## Honest stubs

A stub must say it's a stub in its own return value or comment, not just in
a doc comment nobody reads at runtime. The exemplar is
`perform_install` (`apps/desktop/src-tauri/src/update/transaction.rs`):
it always returns `Err("perform_install is registered but not wired:
downloading the AppImage artifact requires an HTTP client this crate does
not carry as a dependency yet; ...")` — the caller sees the exact missing
dependency, not a generic failure. **When landing a deliberate stub**: make
its error/return value name the specific missing piece, the same way.
