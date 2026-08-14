---
canonical: true
owner: god-emporer
status: planning-ready (operator directive 2026-08-11)
date: 2026-08-11
canonical_branch: tranche/11
companion_to: ./technical-design.md
---

# SD-31 Epic Breakdown

Dependency-ordered. Each epic's proof is executable; none closes on prose.

---

## Epic 1 (SD31-E1) — Preflight and Partition Proof

**Objective:** Establish that SD-31 can run beside SD-29 and SD-30 without collision, before
writing feature code.

### Feature seeds

#### SD31-E1-F1 — Partition verification

Acceptance:

- `tranche/11` cut from `develop` and pushed to origin.
- The `TR-29-001` and `TR-30-001` partitions are re-read **at cycle time** (not inherited from
  this document) and the subtraction in `TR-31-001` re-confirmed. Both bundles are live; their
  partitions can widen.
- The two `TR-31-002` registration sites are located by command and their exact line numbers
  recorded.

#### SD31-E1-F2 — Fixture inventory

Acceptance:

- Every `.pcg` file in the repo enumerated fresh, with sha256s verified against their pinning tests.
- Each fixture's `GAMEMODE`, `CAMPAIGN`, and record inventory recorded in
  `artifacts/fixture-inventory.md`.
- Identifier-discipline audit returns 0 findings (no `sd31_*` in surface code).

---

## Epic 2 (SD31-E2) — Layer 1: The Parser

**Objective:** Turn `.pcg` bytes into a faithful syntactic document. No game meaning.

### Feature seeds

#### SD31-E2-F1 — Tokenizer

Acceptance:

- Handles all four nesting levels named in `technical-design.md §1`: `KEY:value`, pipe
  subtokens, bracketed groups, repeated keys.
- Repeated keys within a line preserve **order** (the `TYPE` twice case).
- Round-trip property: tokenizing a fixture and re-serializing yields byte-identical output.
  This is the proof that nothing was silently dropped at Layer 1.

#### SD31-E2-F2 — Malformed-input behaviour

Acceptance:

- Unterminated brackets, empty keys, and non-UTF-8 bytes each produce a **typed** parse error
  naming the line number — never a panic, never a silent skip.
- A fuzz-ish corpus of truncated fixture prefixes parses or errors cleanly at every cut point.

---

## Epic 3 (SD31-E3) — Layer 2: The Typed Record Model

**Objective:** `PcgDocument` → `PcgCharacter`, still in PCGen's vocabulary.

### Feature seeds

#### SD31-E3-F1 — Core records

Acceptance: `STAT`, `RACE`, `CLASS`/`CLASSABILITIESLEVEL`, `ALIGN`, `SKILL`, `ABILITY`,
`LANGUAGE`, `EQUIPNAME` typed, with both vendored fixtures parsing to fully-populated models.

#### SD31-E3-F2 — The `EQUIPSET` tree

Acceptance:

- Dotted IDs (`0.1`, `0.1.1`) reconstruct the containment tree; `CALCEQUIPSET` selects the
  active set.
- An orphaned child ID (parent absent) is a typed error, not a dropped item.

#### SD31-E3-F3 — Unknown-token survival

Acceptance: a `.pcg` containing a token kind the model does not know parses successfully and
**retains the token** for the fidelity report. Unknown ≠ discarded (`TR-31-004`).

---

## Epic 4 (SD31-E4) — Layer 3: Reference Resolution (GATES Epics 5 and 6)

**Objective:** PCGen names → corpus keys, with an honest fidelity report. This is the bundle's
substance and its risk.

### Feature seeds

#### SD31-E4-F1 — Resolver per token kind

Acceptance: each row of `technical-design.md §3`'s table implemented with its declared failure
mode. No nearest-match guessing anywhere.

#### SD31-E4-F2 — Parameterized feats resolve as a pair

Acceptance: `Weapon Focus|APPLIEDTO:Longsword` resolves the feat **and** its parameter, or
resolves neither. A test pins that dropping the parameter is impossible by construction.

#### SD31-E4-F3 — Fidelity report

Acceptance: every `UnresolvedReason` variant reachable and covered; `RecordNotIngested` names
the owning book; raw token text preserved verbatim for user matching.

#### SD31-E4-F4 — `CreateCharacterRequest` construction

Acceptance:

- Output is the existing struct, consumed by the existing path.
- A test pins that **no PCGen computed value** (`HITPOINTS`, `SKILLSGAINED`, …) reaches the
  request — the §2 "inputs, never outputs" rule, enforced rather than documented.

---

## Epic 5 (SD31-E5) — IPC Command (gated on Epic 4)

### Feature seeds

#### SD31-E5-F1 — `import_pcgen_character`

Acceptance:

- Registered per `TR-31-002` (single-line append, diff hunk in the receipt).
- Reads the path itself, mints a fresh id, recomputes via the real engine, returns
  `Saved` | `Blocked` — reusing `import_character`'s invariants rather than re-implementing them.
- Blocked imports persist **nothing**, proven by a test that inspects storage after a refusal.

#### SD31-E5-F2 — Acknowledged lossy import

Acceptance: `TR-31-006`'s flag exists, defaults false at every layer, and a test pins that a
lossy import without it is refused.

---

## Epic 6 (SD31-E6) — Player Surface (gated on Epic 5)

### Feature seeds

#### SD31-E6-F1 — Import affordance

Acceptance: `.pcg` selectable from the existing import entry point; invokes the real command
(4-grep audit per `TR-31-008`).

#### SD31-E6-F2 — Mapping-review screen

Acceptance: renders the real `FidelityReport` before persistence; unresolved entries show raw
token text and reason; the acknowledgement control is the only path to a lossy import.

#### SD31-E6-F3 — Imported character reaches the sheet

Acceptance: a full import of the Fighter fixture is loadable and its sheet renders, verified
live rather than by unit test.

---

## Epic 7 (SD31-E7) — Oracle Parity (the real definition of done)

**Objective:** Prove imports are *correct*, not merely parsed. See
`acceptance-and-verification.md §2`.

### Feature seeds

#### SD31-E7-F1 — Parity harness

Acceptance: for a fixture, run Codex's import → recompute, and the same `.pcg` through real
headless PCGen via `oracle_validation::pcgen_runner`, and compare through the existing
comparator. Read-only use of `oracle_validation` (`TR-31-001`).

#### SD31-E7-F2 — Both vendored fixtures at parity

Acceptance: Fighter L1 and Wizard L1 both pass on the selected dimensions. Any divergence is
either fixed or recorded as a named, explained deviation — never silently accepted.

---

## Epic 8 (SD31-E8) — Bundle Code Review

Acceptance: adversarial review of the whole import path, with the reviewer explicitly
tasked to find a token that can be silently dropped (`TR-31-004`) and a path that persists a
lossy character without acknowledgement (`TR-31-005`).

---

## Epic 9 (SD31-E9) — Closure Epilogue

Acceptance: architecture docs refreshed per the template's §6 obligation; forward-scope
register updated with what was deferred; release notes; test-count delta reconciled once per
`TR-31-003`.

---

## Recommended sequencing

1 → 2 → 3 → 4 → (5 → 6) → 7 → 8 → 9. Epic 7 can begin as soon as Epic 5 lands; it does not
wait on the UI.

## Completion gate

The bundle closes when both vendored fixtures import to oracle parity, a lossy import is
provably refused without acknowledgement, and the 4-grep audit is clean on the player surface.
