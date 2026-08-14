---
canonical: true
owner: god-emporer
status: planning-ready (operator directive 2026-08-11)
date: 2026-08-11
canonical_branch: tranche/11
---

# SD-31 Risks and Open Questions

## R-1 — Collision with SD-29 / SD-30 (the reason this package exists in this shape)

**Risk:** three bundles writing one repo.

**Mitigation:** `TR-31-001`'s partition is the subtraction of the sibling partitions, and Epic 1
re-derives it at cycle time rather than trusting this document. `AC-9` checks it by command at
closure.

**Residual:** the two `TR-31-002` registration sites and the `TR-31-003` test-count baseline.
Both are handled by protocol (append-only single line; delta recorded, reconciled once at
merge) rather than by avoidance, because neither can be avoided.

**Live, not closed:** SD-29 and SD-30 can widen their partitions after this file was written.
Step 2 of the loop instruction exists specifically for this.

## R-2 — Fixture coverage is two characters wide

**Risk:** the vendored fixtures are a Human Fighter L1 and a Human Wizard L1. Both are level 1,
both Human, both CRB. A parser and resolver proven against them may fail on the first
multiclass, non-human, high-level, or non-CRB character a real user opens.

**Why the bundle proceeds anyway:** `TR-31-007` forbids hand-authoring `.pcg` files, and those
two are what exists. Two real files beat ten invented ones.

**Mitigation:** Epic 3's unknown-token-survival requirement and Epic 2's truncation sweep are
designed to make *unfamiliar* input degrade honestly rather than silently. The importer's
correctness on unseen shapes is bounded by the fidelity report, not by the fixtures.

**Open question for the operator:** can real `.pcg` files be generated from the PCGen checkout
(a multiclass character, a non-CRB character) and vendored with pinned sha256s? That would
materially widen coverage. It is not assumed available.

## R-3 — Oracle parity may diverge for reasons that are not SD-31's

**Risk:** Codex's engine and PCGen already disagree on some dimensions, independent of import.
A parity failure could stall the bundle on a pre-existing defect.

**Mitigation:** `acceptance-and-verification.md §2` requires the cycle to classify a divergence
into one of three causes and route the non-SD-31 ones out. Narrowing the dimension set is
allowed **with a recorded reason**; loosening a tolerance silently is a cycle defect.

## R-4 — PCGen runner availability

**Risk:** Epic 7 depends on `scripts/pcgen-run-character.sh` driving a real PCGen Gradle
wrapper, which needs the PCGen checkout at `~/workspace/repos/pcgen` and a working JVM
toolchain. If headless PCGen cannot run in this environment, the bundle's strongest
verification is unavailable.

**Status:** the checkout is present. The runner has **not** been executed as part of authoring
this package.

**Mitigation:** Epic 1 SHOULD smoke-test the runner against a vendored fixture before Epics 2–6
build on the assumption. Discovering this in Epic 7 would be discovering it late.

## R-5 — The "looks right, computes wrong" failure mode

**Risk:** the defining hazard of import features. A dropped equipmod or a mis-parameterized
feat produces a character that renders normally and is wrong forever, with no error.

**Mitigation:** this is what `TR-31-004` (no silent loss), `AC-5` (no computed values carried),
Epic 4's paired-parameter requirement, and the whole oracle-parity epic exist for. It is also
the explicit target of Epic 8's adversarial review.

## R-6 — CPU contention

**Risk:** SD-31 is a cargo bundle and competes with SD-29's builds on a 4-core box.

**Mitigation:** scratch `CARGO_TARGET_DIR` per cycle, capped parallelism, and the frontend
gate (`npm test`, `tsc --noEmit`) is cheap. Note that Epics 2–4 are pure-Rust unit work with a
small compile surface; Epic 7's PCGen runs are the expensive part and are serial by design.

## Open questions for the operator

1. **R-2** — may additional real `.pcg` fixtures be generated and vendored?
2. **Lossy-import policy** — `TR-31-006` allows an acknowledged lossy import. Is that wanted at
   all, or should v1 refuse unconditionally? The stricter reading is safer; the looser one is
   kinder to a user whose character uses one un-ingested feat.
3. **`TEMPLATESAPPLIED`** — `§8` defers all templates. If the two fixture values are known-inert,
   an allowlist would let both fixtures import without an acknowledgement. Worth doing in v1?
