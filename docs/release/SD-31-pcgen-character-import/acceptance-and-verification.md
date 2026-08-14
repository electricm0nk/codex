---
canonical: true
owner: god-emporer
status: planning-ready (operator directive 2026-08-11)
date: 2026-08-11
canonical_branch: tranche/11
companion_to: ./technical-requirements.md
---

# SD-31 Acceptance and Verification

## 1. Per-cycle gate

Every cycle records the output of:

```sh
cargo test --locked pcgen_character
cd apps/desktop && npm test && npx tsc --noEmit
```

A gate returning exit 0 with **zero matched tests** is a hard failure (`TR-31-009`). Test
counts are recorded per cycle but the shared root baseline is not edited (`TR-31-003`).

## 2. The definition of done: dual-oracle parity

Most import features can only be tested against their own expectations — you assert that the
importer produces what you believe the file means, which proves the importer agrees with the
test author. This repo can do better, because it already runs the real program.

`src/oracle_validation/pcgen_runner.rs` drives the real PCGen Gradle wrapper in headless batch
mode against a `.pcg` and normalizes the output into typed `case_id`/`dimensions`/`diagnostics`
JSON, which `comparator.rs` already knows how to diff.

So an imported character is verified against **PCGen's own computation of the same file**:

```
   fixture.pcg ──┬── SD-31 import → CreateCharacterRequest → Codex engine ──┐
                 │                                                          ├── comparator → parity
                 └── pcgen_runner → real PCGen → normalized dimensions ─────┘
```

A divergence means one of three things, and the cycle must say which:

1. **The importer mapped something wrong** — fix the importer.
2. **Codex's engine differs from PCGen** — pre-existing, not SD-31's; record it and route it.
3. **The dimension is not comparable** — narrow the selected dimension set, with a reason.

Silently loosening a tolerance to make parity pass is a cycle defect.

### Why this matters more than usual here

Import bugs are uniquely bad: they are invisible. A character that imports with the wrong feat
parameter or a dropped equipmod looks perfectly normal and computes wrong forever. There is no
error message and no crash. The oracle is the only thing standing between a plausible import
and a correct one.

## 3. Acceptance criteria

| # | Criterion | Verified by |
|---|---|---|
| AC-1 | Both vendored `.pcg` fixtures parse with byte-identical re-serialization | Epic 2 round-trip test |
| AC-2 | Malformed input yields typed, line-numbered errors — never a panic | Epic 2 truncation sweep |
| AC-3 | An unknown token kind is retained and reported, never discarded | Epic 3 test |
| AC-4 | Parameterized feats resolve as a pair or not at all | Epic 4 test |
| AC-5 | No PCGen computed value reaches `CreateCharacterRequest` | Epic 4 structural test |
| AC-6 | A lossy import without acknowledgement is refused and persists nothing | Epic 5 post-refusal storage inspection |
| AC-7 | The fidelity report reaches the player before persistence | Epic 6 live run + 4-grep audit |
| AC-8 | Both fixtures reach oracle parity on the selected dimensions | Epic 7 |
| AC-9 | No write outside `TR-31-001`'s partition, across the whole bundle | `git diff --name-only develop...tranche/11` at closure |

**AC-9 is the one that protects the other two bundles**, and it is checked by command at
closure, not assumed from per-cycle discipline.

## 4. What would falsify a "done" claim

Stated explicitly so a reviewer has something to aim at:

- A `.pcg` token that appears in a fixture and in neither `resolved` nor `unresolved`.
- Any code path reaching `save` with a non-empty `unresolved` list and no acknowledgement.
- A parity pass achieved by removing a dimension without a recorded reason.
- A hand-authored `.pcg` in the test suite (`TR-31-007`).
- An import affordance whose handler does not invoke `import_pcgen_character`.
