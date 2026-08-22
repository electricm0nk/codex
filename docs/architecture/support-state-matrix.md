# Support-state matrix

> Scope: The typed control-plane carrier that records what the rules engine currently, honestly supports — documentary truth, not computed mechanics.
> Last verified: 2026-07-22 against tranche/5-3 (SD-25 closure). **Path correction 2026-08-22**
> (SD-32 closure epilogue): pilot_compute.rs (old path src/rules_core/pilot_compute.rs, no longer valid) updated to
> `src/rules_core/pilot_compute/mod.rs` — the module became a directory during SD-31; no other
> content in this doc re-verified.
> Maintenance: updated at SD closure — see [README.md](./README.md) §Maintenance contract

## What the matrix is

`src/rules_core/support_state_matrix.rs` is a typed, machine-usable control-plane surface that
carries the bounded support-state matrix and its seeded current-truth rows. Its own module doc
comment states the boundary precisely: it "carries the bounded support-state matrix and its seeded
current-truth rows so later breadth claims update typed truth instead of improvising folklore." It
computes no character mechanics, parses no external files, projects no UI, serializes/persists
nothing, and never promotes a row on its own — every row transition is a hand-authored source edit
landed alongside the runtime evidence that justifies it.

The seeding function, `seeded_current_truth() -> SupportStateMatrix`, returns exactly 34
rows as of this verification: 7 race, 12 class, 2 interaction, 9 school, 4 equipment — a count the
function's own doc comment states explicitly and that this document's authors re-verified directly
against the source (`grep -c "row_id:"` inside the function body). Race/class/interaction rows are
fixed content from the SD-13 seed; SD-19-and-later cycles append one school or equipment row per
landed cycle, and the doc comment is explicit that a widening cycle never rewrites an existing row's
identity — only its `support_state`/`evidence_tier`/notes fields change in place.

This module is documentary: nothing in `rules_core`'s compute path (`src/rules_core/pilot_compute/mod.rs`,
`src/rules_core/spellbook.rs`, etc.) reads the matrix to decide what to compute. The matrix is a truth ledger read
by humans, tests, and the desktop bridge — not an input to the engine itself.

## The four orthogonal axes

`SupportStateRow` carries four independent classification axes, each its own enum. They are
deliberately kept independent — a row's position on one axis never implies a position on another.

### `SupportState`

How complete the bounded claim is, regardless of how it was proven:

| Variant | Meaning |
|---|---|
| `Supported` | Proven at the required evidence floor with no known missing semantics inside the bounded claim. |
| `Partial` | Some required semantics are proven, but one or more named required semantics remain incomplete and visible. |
| `Lossy` | The path works only by discarding or approximating named semantics. |
| `Blocked` | Known missing semantics or explicit claim-blocking diagnostics prevent the claim. |
| `Unverified` | No direct evidence yet exists for the named dimension. |

### `EvidenceTier`

The highest evidence tier achieved for a row, on the Codex quality-gate scale: `Observed`, `Parsed`,
`Converted`, `Computed`, `OracleChecked`, `ProductVisible` (the enum's declaration order in
`src/rules_core/support_state_matrix.rs` is also its ascending-strength order).

### `MatrixSubjectType`

What kind of thing a row classifies: `Race`, `Class`, `Interaction`, `School(Pf1SchoolId)`,
`Equipment(EquipmentCategory)`. `School` and `Equipment` are the two data-carrying variants: the
type's doc comment marks them with a note that "closure requires a
matrix row transition, and neither spell schools nor equipment categories were modeled as rows
before this" — i.e. the axis carries a distinct payload shape for each claim type rather than
overloading `Interaction`.

### `EvidenceFreshness`

A separate audit axis recording whether a row's breadth claim can currently be trusted as refreshed
against its grounding evidence — independent of both `SupportState` and `EvidenceTier`. Two
variants:

- `RefreshableFromLiveProof` — the row is anchored to a live, re-runnable proof surface (an executable test or the deterministic compute seam); it *could* be refreshed by re-running the cited proof, but no completed refresh checkpoint is recorded yet.
- `AwaitingInitialEvidence` — the row rests only on bounded scope naming with no runtime evidence yet; there is nothing to refresh from.

`EvidenceFreshness::is_refresh_confirmed(self) -> bool` always returns `false` for both variants
today — the type's doc comment is explicit that this is deliberate: "no variant asserts a row is
currently fresh... the downstream audit surface can only ever conclude 'refresh-required' from this
seed — never 'all fresh'." A future slice that adds a genuine refreshed-checkpoint variant only has
to flip this one method.

## Why `SupportState` and `EvidenceTier` are separate axes

The type's own doc comment states the reason directly: keeping the axes independent means "a
`Computed` row is never silently read as `Supported`." A row can be `Partial` support with
`Computed` evidence — real runtime computation exists, but named semantics are still missing — and
that combination must stay visibly distinguishable from a row that is genuinely `Supported`.
Collapsing the two axes into one would let a caller (or a careless reviewer) treat "we have computed
evidence" as equivalent to "this is done," which is exactly the folklore-over-typed-truth failure
mode the module exists to prevent. Note that the current seed does not exercise the mid-band
combinations: all 34 rows carry one of exactly two pairs — `SupportState::Supported` with
`EvidenceTier::ProductVisible` (33 rows) or `SupportState::Unverified` with
`EvidenceTier::Observed` (the `interaction.non_human_any_class.progression_pressure` row) — so
today the axis independence is a typed guarantee rather than something visible in the seed data.
(The module's own top-of-file comment still describes the Paladin hybrid row as
`Blocked`/`Computed`; the row literal itself is `Supported`/`ProductVisible` — trust the row
literals over the header comment.)

## Row structure and the grounding-reference pattern

`SupportStateRow` fields (all `&'static str` except the four enum axes, because the seed is a fixed,
deterministic, in-source carrier with no parsing or runtime construction):

| Field | Purpose |
|---|---|
| `row_id` | Stable identifier, e.g. `class.fighter.level_1_pilot`. |
| `subject_type` | One `MatrixSubjectType` variant. |
| `subject_id` | Subject identity, e.g. `race:human`, `class:fighter`. |
| `dimension` | The semantic/progression dimension this row classifies, in prose. |
| `support_state` | One `SupportState` variant. |
| `evidence_tier` | One `EvidenceTier` variant. |
| `evidence_freshness` | One `EvidenceFreshness` variant. |
| `grounding_ref` | Real doc or repo evidence grounding the row — never chat prose or an invented receipt. |
| `blocker_or_lossiness_note` | Per the struct's doc comment: non-empty for `Blocked`/`Lossy` rows, empty when the state needs no note. In the current seed, however, every row — including all `Supported` ones — carries a non-empty note, used in practice as a running per-cycle evidence narrative. |
| `next_required_uplift` | The next required uplift, or the owning future slice. |

The `grounding_ref` field is where the module's evidence discipline is most visible. Every citation
is a real, checked-in path — usually a `const` string defined earlier in the same file (e.g.
`SD13_ROGUE_LEVEL1_TEST`, `SD13_FIGHTER_LEVEL9_LEVEL10_TEST`) that concatenates every test file
proving that row's claim, one `+`-joined literal per widening cycle. This "combined literal" idiom
(the file's own comments call it "the paladin-row idiom") exists because a single row's grounding
accumulates test files across many widening cycles, and every one of those files needs to stay
independently substring-checkable by any test that asserts `grounding_ref.contains(...)`.
`grounding_ref` values seen in the file point at real `tests/*.rs` integration test files, real
`docs/release/SD-*/artifacts/*.md` doctrine artifacts, or real `src/rules_core/*.rs` source modules
— never at a description of evidence that doesn't exist on disk.

## How the matrix reaches the UI

`SupportStateMatrix::row(&self, row_id: &str) -> Option<&SupportStateRow>` is the only lookup helper
the carrier itself exposes — a narrow `find` by `row_id`.

The desktop app's read-only bridge lives in two files, not one:

- `apps/desktop/src-tauri/src/support_state_matrix_bridge.rs` (renamed from `sd13_support_state_matrix.rs` by SD-25 criterion 1.1's identifier cleanup) — `build_support_state_matrix_snapshot() -> SupportStateMatrixSnapshot`. This is a presentation adapter: it calls `seeded_current_truth()`, and for every row projects a `SupportStateRowPresentation` that mirrors the upstream fields verbatim plus two SD-13-owned derived-wording fields (`tester_facing_state_label`, `refresh_audit_label`) computed purely from `support_state`/`evidence_freshness` — the module's own doc comment is explicit that this bridge "deliberately does **not** compute rules, persist, mutate, promote/demote, recompute, filter, aggregate" anything.
- `apps/desktop/src-tauri/src/main.rs` — the actual `#[tauri::command] fn load_support_state_matrix() -> SupportStateMatrixSnapshot` lives here, and its body is a one-line call to `build_support_state_matrix_snapshot()`. This is the command the frontend invokes; it is registered in `apps/desktop/src-tauri/src/main.rs`'s `tauri::generate_handler!` list alongside the app's other commands.

The round trip is: `support_state_matrix::seeded_current_truth()` (upstream typed truth)
→ `support_state_matrix_bridge::build_support_state_matrix_snapshot()` (serializable presentation
projection) → `main.rs::load_support_state_matrix` (the Tauri IPC command) → the desktop frontend.
No layer in that chain recomputes or reorders rows; the doc comment on the presentation struct
states every field "mirrors upstream truth; nothing here is recomputed or promoted."

## Relationship to the fail-honest diagnostics

The matrix and the engine's fail-honest diagnostics (see [rules-engine.md](./rules-engine.md) §"The
fail-honest pattern") are two different honesty mechanisms operating at two different scopes, and it
is worth keeping them distinct:

- A **claim-blocking diagnostic** (`ComputationDiagnostic.claim_blocking: bool` in `src/rules_core/pilot_compute/mod.rs`) is a per-request, runtime signal: it fires (or doesn't) each time `compute_pilot_base_chassis` runs against a specific `CharacterInput`, and it governs whether *that one receipt* is blocked.
- A **matrix row** is a per-capability, static, hand-authored signal: it records the engine's current overall posture for a named dimension (e.g. "Paladin hybrid chassis") independent of any single request, and it only changes when a contributor lands new grounding evidence and edits the source.

They are consistent with each other by construction, not by any code-level link: a row can only
honestly move to `Supported` once the corresponding runtime path stops producing claim-blocking
diagnostics for the bounded claim in question, and several `grounding_ref` values cite the exact
test files that exercise that runtime path. But there is no live coupling — moving a row to
`Supported` in `src/rules_core/support_state_matrix.rs` does not change engine behavior, and a diagnostic firing at
runtime does not automatically demote a row. Keeping them apart is deliberate: the matrix is
expected to lag runtime evidence by exactly as long as it takes a contributor to land the
corresponding row edit.

## What a contributor must do to land support for something new

Landing new support for a race, class, level band, spell school, or equipment category is a two-part
change, and both parts are required before a row may move:

1. **Land the runtime evidence first.** Add the real computation (in `src/rules_core/pilot_compute/mod.rs` or the relevant per-domain engine — see [rules-engine.md](./rules-engine.md)) and a test file that exercises it end to end. The row's eventual `grounding_ref` must name this real file; a row's `grounding_ref` is never written before the file it cites exists.
2. **Then edit the row, or append a new one, in `seeded_current_truth()`.** What changes depends on the evidence tier reached:
   - Moving from `Unverified`/`Observed` to any tier with real runtime evidence requires updating `evidence_tier` (to at least `Computed`) and `evidence_freshness` (to `RefreshableFromLiveProof`, since a live test now exists to refresh from).
   - Moving `support_state` toward `Supported` requires that `blocker_or_lossiness_note` either becomes empty or is rewritten to name only the semantics that are genuinely still missing — never left stale from a prior, less-complete state.
   - `next_required_uplift` must be rewritten to describe the next real gap, or explicitly state there is none.
   - `grounding_ref` is extended (following the combined-literal `+`-joined idiom) to also cite the new test file, never replacing the prior citations that still hold.
   - A genuinely new subject (a race, class, spell school, or equipment category with no existing row) gets a new `SupportStateRow` appended, not folded into an existing row's `dimension` text — the module's doc comment states each new `MatrixSubjectType` shape (e.g. `School`, `Equipment`) required its own row family, not an overload of `Interaction`.
3. **Never promote a row past what the cited evidence proves.** The carrier has no verification of its own — the `grounding_ref` string is not machine-checked against the enum values at compile time, so the discipline is entirely contributor-enforced. Reviewers should open every cited `grounding_ref` path before accepting a `SupportState` upgrade.

See [rules-engine.md](./rules-engine.md) for how the underlying compute engines produce the runtime
evidence a row's `grounding_ref` cites, and [testing.md](./testing.md) for the test-organization
convention the per-widening-cycle test files follow.

