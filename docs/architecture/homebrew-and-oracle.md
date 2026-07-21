# Homebrew authoring and oracle validation

> Scope: the headless GE-08 package-authoring surface and the GE-05 oracle-parity surface, as they exist today.
> Last verified: 2026-07-20 against ef9012bf5de8
> Maintenance: updated at SD closure — see [README.md](./README.md) §Maintenance contract

Both modules covered here are deliberately narrow, bounded proof slices, not
general subsystems. Each module's own doc comment states its non-goals
explicitly — this file repeats those non-goals verbatim because they are load
-bearing for anyone tempted to widen either surface without reading the code
first.

## `homebrew_authoring/`: headless package-authoring surface

`src/homebrew_authoring/mod.rs`'s module doc comment: "This slice deliberately
stays small: one authored package, one feat, one effect, one optional
prerequisite, provenance, diagnostics, and deterministic file persistence. No
UI, plugin runtime, or broad rules-authoring breadth is claimed here."

### Types and validation (`mod.rs`)

`SourcePackage` bundles a `PackageManifest` (from `package_manifest.rs`) with
optional `feat: Option<FeatRecord>`, `effect: Option<EffectRecord>`,
`prerequisite: Option<PrerequisiteRecord>`, a `Vec<ProvenanceEntry>`, and a
`Vec<PackageDiagnostic>`. The only shipped content is the "Guard Stance" proof
package (`SourcePackage::guard_stance_shell()` for an empty Draft package,
`SourcePackage::guard_stance_proof()` for the fully authored one) —
constants like `GUARD_STANCE_PACKAGE_ID` and `GUARD_STANCE_FEAT_ID` are
proof-case identifiers, not a general package-authoring vocabulary.

`SourcePackage::recompute_validation` is the validation engine: it checks
manifest completeness (non-empty `package_id`/`package_title`/
`game_system_id`/`package_version`, and a `depends_on` entry for
`"pf1.crb"`), then structural coherence between the feat/effect/prerequisite
records (ids matching, `effect.target_family == "armor_class"`,
`effect.modifier_value == 1`), then that every authored record has a
matching `ProvenanceEntry` pointing at the expected authored path. Every
failure becomes a `PackageDiagnostic { class, severity, message, subject_ref,
claim_blocking }`; `finalize_diagnostics` sorts/dedups them and derives the
resulting `PackageValidationState` (`Valid` only if there are zero
diagnostics; `Invalid` if any diagnostic is `claim_blocking`; otherwise
`Deferred`).

### Deterministic file persistence (`package_store.rs`)

`PackageStore` (a concrete unit struct, same shape as the persistence stores
described in [persistence.md](./persistence.md)) has `save`, `load`, `diff`,
`diff_roots`, and `export`. The on-disk layout that `save()` produces
(directories created by `ensure_layout`, files written from `file_map`):

```text
<root>/
  manifest.yaml
  objects/feats/<stable_id>.yaml        (at most one file)
  rules/effects/<stable_id>.yaml        (at most one file)
  rules/prerequisites/<stable_id>.yaml  (at most one file)
  metadata/provenance.yaml
  metadata/diagnostics.yaml
```

Every file is rendered through a hand-rolled `key: value` / `- item` YAML-like
grammar (`render_manifest`, `render_feat_record`, etc.) and parsed back with a
matching line-oriented parser (`parse_manifest`, `parse_feat_record`, ...) —
this is a distinct grammar from the `key=value` fixture format `saved_character`
and `oracle_validation` use (no `=`, indentation-sensitive list sections via
`ManifestSection`/`RecordListSection`). `PackageStore::save` calls
`package.normalized_for_persistence()` (which recomputes validation and
diagnostics before writing) and `validate_persistable`, which rejects any
field that is empty or contains a newline — the doc comment above
`validate_persistable` states why: `save()` deliberately persists Draft/
Invalid packages, so an unloadable field would otherwise leave a corrupt
bundle on disk with no warning at write time. `PackageStore::export` refuses
to write anything unless `validation_state == PackageValidationState::Valid`.

### `preview_bridge.rs`: validated package + proof binding → headless envelope

`PreviewBridge::preview` (and `preview_from_root`, which loads via
`PackageStore::load` first) takes a `SourcePackage` and produces a
`PreviewEnvelope` whose `preview_status: PreviewStatus` is exactly one of
`Success`, `Blocked`, or `Unsupported` — the module doc comment is explicit
that this three-way split exists so the bridge "never" returns "a counterfeit
success." `classify_posture` decides which: `Blocked` if the manifest's
`ProofBinding` doesn't match the one fixed GE08-E1 binding
(`GE08_E1_CASE_ID`/`SLOT`/`REMOVE`/`ADD` constants), or the feat/effect
records are missing, or (checked after the widening test below) the package
still carries any other claim-blocking diagnostic from `recompute_validation`;
`Unsupported` if a structurally valid effect targets a recognized
derived-value family (`RECOGNIZED_DERIVED_FAMILIES`) other than
the one this slice supports, `"armor_class"`; `Supported` otherwise.

On `Supported`, the bounded AC computation is:
`GE06_BASE_ARMOR_CLASS_WITHOUT_BONUS_FEAT_SLOT (16) + effect.modifier_value`.
The doc comment on that constant and the module-level doc comment both explain
why 16, not a re-run of the real combat path: GE-06's deterministic AC
baseline of 17 is grounded with Dodge already selected into the bonus-feat
slot, and that combat path (`src/rules_core/pilot_compute.rs`) is "locked to
the exact Dodge posture" (it requires `feat:dodge` and hard-codes the Dodge AC
bonus) — so 16 is that same baseline with Dodge's contribution subtracted out,
not an independent guess. Every outcome (including `Blocked`/`Unsupported`)
still returns populated `diagnostics`, `provenance_refs`, `explanation_refs`,
and `oracle_dimension_status` — an explanation graph is never blank, even when
the preview itself is refused.

### What this module deliberately does not include

No UI beyond the one headless workbench-snapshot command described below; no
plugin runtime; no widened package-authoring surface beyond the one proof
package's feat/effect/prerequisite shape. The desktop reaches this substrate
through exactly one command: `load_ge08_authoring_workbench_snapshot` in
`apps/desktop/src-tauri/src/main.rs`, a thin wrapper around
`ge08_workbench::build_ge08_workbench_snapshot` (in
`apps/desktop/src-tauri/src/ge08_workbench.rs`), which itself calls
`PreviewBridge::preview_from_root` and `PackageStore::load` and maps the
result into desktop DTOs, deriving only two UI lifecycle-gate booleans
(`preview_allowed`, `export_allowed`) directly from the headless validation
state and preview status — no independent authoring logic (validation rules,
effect computation, provenance checks) lives in the desktop crate.

## `oracle_validation/`: the oracle-parity surface, today

`src/oracle_validation/mod.rs`'s entire module doc comment: "This module
currently exposes only the GE05-E2-F1 golden-case fixture schema. Comparator,
normalization, parity-report, and PCGen-runner behavior are intentionally out
of scope for this slice and live in later GE-05 slices." Two submodules exist:
`golden_fixture` and `selected_parity_dimensions`.

### `golden_fixture.rs`: typed golden-case fixture

`GoldenCaseFixture` binds old-system (PCGen) *runtime* evidence
(`LegacyOracleEvidence`, with `evidence_kind: OracleEvidenceKind` —
`StaticSourceTruth` / `RuntimeBehaviorEvidence` / `GuiDerivedEvidence` /
`Unknown` — the doc comment notes the pilot requires
`RuntimeBehaviorEvidence`, and static source files alone must not satisfy
parity) to an as-yet-unresolved `CodexOutputEvidence` (`state:
CodexOutputState` is only `Unresolved` or `Absent` — there is deliberately no
"resolved/passed" variant in this slice). `dimensions: Vec<ComparisonDimension>`
carries a `DimensionStatus` that is only `Candidate`, `Blocked`, or
`NotYetGrounded` — no `Passed` variant, because passing belongs to the
GE05-E4 comparator, explicitly out of scope. `claim_target` /
`current_claim_status: ClaimTier` (`NotYetGrounded` / `Computed` /
`OracleChecked`) let a fixture honestly target `OracleChecked` while
its current status stays `NotYetGrounded`; `parity_claimed()` only returns
`true` when `current_claim_status == ClaimTier::OracleChecked`.

`load_golden_case_fixture` parses a `key=value` text format (repeatable keys
for `dimension`, `provisional`, `normalization_ref`, `known_gap_ref`) —
the doc comment states this "mirrors the GE04 `rules_core::character_input`
loader conventions." Every required field missing or malformed produces a
`FixtureDiagnostic { claim_blocking: true, ... }`; the fixture is only
constructed (`FixtureLoadResult.fixture: Some(..)`) when there are zero
diagnostics.

### `selected_parity_dimensions.rs`: receipt → selected-dimension surface

`SelectedParityDimensions::from_receipt` adapts a
`crate::rules_core::pilot_compute::PilotHeadlessReceipt` into a bounded
`Vec<SelectedDimension>` covering exactly nine mandatory pilot dimensions
(`character.identity` plus eight numeric ones: baseline melee attack bonus,
baseline armor class, the three total saves, and three selected skill
modifiers). Every emitted carrier is stamped with `claim_tier_floor:
ClaimTierFloor::Computed` — the module doc comment states this "maintains a
`Computed` claim-tier floor without implying oracle-checked parity." There is
only one `ClaimTierFloor` variant today (`Computed`); nothing in this module
can produce an `OracleChecked` claim.

### Deferred (verified via each module's own doc comments)

Per `mod.rs`'s doc comment: the comparator, the normalization rule engine,
the parity-report writer, and the PCGen runner are all out of scope for this
slice and live in later GE-05 slices. `golden_fixture.rs` repeats the same
list of deliberate non-goals. Nothing under `oracle_validation/` today can
compare a Codex output against legacy evidence or emit a pass/fail parity
verdict — see [status.md](./status.md) for where this sits relative to the
rest of the release surface.

## Relationship to the fail-honest pattern and test locations

Both modules follow the same fail-honest discipline documented in
[rules-engine.md](./rules-engine.md): diagnostics are structured and
`claim_blocking`-typed rather than stringly-typed, and no function silently
substitutes a fabricated value when it cannot ground a claim (`preview_bridge.rs`'s
three-way `PreviewStatus` split and `golden_fixture.rs`'s `ClaimTier` gating
are both direct applications of it).

Tests live under the repo-root `tests/` directory, not inline in these
modules: `tests/golden_case_fixture_schema.rs` covers `golden_fixture.rs`;
`tests/ge06_selected_parity_dimensions.rs` covers
`selected_parity_dimensions.rs`; `tests/ge08_preview_bridge.rs`,
`tests/ge08_package_file_lifecycle.rs`, and
`tests/ge08_validation_and_diagnostics.rs` cover `homebrew_authoring/`, with
fixture data under `tests/fixtures/ge08/`. See
[testing.md](./testing.md) for the repo's test-layout conventions generally.
