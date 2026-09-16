---
canonical: true
owner: operator
bundle_id: SD-36
date: 2026-09-15
---

# SD-36 Risks and Open Questions

Self-healable vs. non-self-healable splits, and any open override flags.

---

## Type-identity trap (A — crate wall)

**Risk:** If `codex` dev-depends on `codex-ingest` during Epic A, Rust compiles codex twice with different `CARGO_MANIFEST_DIR` values. The pcgen_import module sees inconsistent symbols (different `include_str!` paths resolved to different values). This produces a plausible wrong number (tests pass), not an error.

**Mitigation:** Build gate enforces via `cargo metadata`. No dev-dep from codex to codex-ingest allowed.

**Self-healable?** YES. The `crate-wall` stage runs every epic; a red gate stops the next cycle immediately.

---

## CARGO_MANIFEST_DIR drift in crates/codex-ingest (A)

**Risk:** Moved code in `codex-ingest` may have `CARGO_MANIFEST_DIR` hard-coded in `include_str!` or path construction. If not rewritten to `repo_root()`, the ingest crate reads from the wrong location.

**Mitigation:** Gate: `git grep -c 'CARGO_MANIFEST_DIR' -- crates` must exit 0. Audited during A3 acceptance.

**Self-healable?** YES. Git grep during acceptance proves zero remaining references. Failure stops the cycle.

---

## Baseline floor changes (B, C1, C2)

**Risk:** Line-count and binary-count baselines drop significantly (B: 7 bins + 33k lines; A: 47 bins; C2: 49k lines). A mistake (e.g., failing to delete files but changing them to stubs, or a incomplete test rewrite) could produce a false-green baseline.

**Mitigation:** Every baseline is re-derived from git and build output, not computed via arithmetic. Epic-end receipt shows the commands run and their exact output.

**Self-healable?** YES. Each epic's receipt includes the baseline re-derive commands and output. A mismatch between claimed and commanded numbers surfaces immediately.

---

## Peer session writing to shared checkout (operational hazard, not code)

**Risk:** During B/A/C execution, another agent session writes to the same shared checkout tree. This causes git collisions, merge conflicts, or data loss.

**Mitigation:** Workflow discipline enforces one writer per tree. This session owns the shared tree (`/home/ubuntu/workspace/repos/codex/`) for the duration of dispatch. Other lanes (parallel work on disjoint territories) use `git worktree add` with dedicated `CARGO_TARGET_DIR`.

**Self-healable?** NO. This is an operational hazard, not a code hazard. If a collision is detected (stray commits, or `git status --porcelain` shows unexpected files), the dispatch pauses and the operator manually resolves the conflict.

---

## 95% → 100% projection change (B, public status)

**Risk:** D5 changes the public page denominator from "the SD-31 public-denominator rule" to "every unit in the DONE statuses." The old rule carve-out (~3% of units) disappears. If the logic is wrong (e.g., a single unit still in "in progress" state that wasn't caught by inventory updates), the page would show 99.99% instead of 100%.

**Mitigation:** Gate `scripts/site/check_frozen_status.py` asserts 100.0%. Acceptance criterion B2 requires the regenerated JSON to pass the gate.

**Self-healable?** DEPENDS. If a single unit is miscounted, the gate fails and stops the epic. If the inventory itself has a defect (a unit marked DONE but not actually complete), that is caught by the schema-level gate at freeze time, not later.

---

## No open override flags

This bundle has no `--force` or `--skip` flags. Every acceptance criterion is mandatory and gated.

---

