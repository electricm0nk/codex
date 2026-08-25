---
canonical: true
owner: god-emporer
bundle_id: SD-33
date: 2026-08-24
---

# SD-33 Acceptance and Verification

Every criterion's proof is a **command with an exit code or a committed artifact**, never a description of work performed.

## 1. Closure gates

SD-33 is closed when all four hold. **None may be satisfied by a filed blocker** (`../../governance/blocker-closure-doctrine.md`).

| Gate | Statement | Command |
|---|---|---|
| **G1 — Box integrity** | The full inventory is partitioned; nothing uncovered, nothing doubled, nothing `done` on an absent check | `python3 scripts/box_ledger.py --check` → exit 0, `uncovered=0 overlap=0 population=49438` |
| **G2 — Engine coverage** | Every formula-bearing unit has been through an engine | corpus-wide run population == 11,652 (both numbers stated) |
| **G3 — No unmeasured bucket** | No unit sits at `status: unknown` | `jq '[.units[]\|select(.status=="unknown")]\|length' docs/work-inventory.json` → `0` |
| **G4 — Oracle agreement** | Every unit the harness can reach agrees with the oracle, or its disagreement is a resolved defect | `python3 scripts/box_ledger.py --check` fails on any unresolved disagreement |

**G4's honest form:** it does **not** claim every unit is verified. It claims every unit the harness *can* reach agrees, and that everything it cannot reach is counted in a visible `unverifiable` bucket with a named reason (`decisions.md §7`). A gate that hides its own blind spot is not a gate.

## 2. Per-criterion artifact map

| Criterion | Artifact | Verification command |
|---|---|---|
| AT-33-E1-001 | `THE-BOX.md` | `python3 scripts/box_ledger.py --check` |
| AT-33-E1-002 | `artifacts/epic-1-instruments/box-ledger-mutation-proofs.md` | five RED→GREEN proofs, one per fail condition |
| AT-33-E1-003 | `artifacts/epic-1-instruments/probe-surface-census.json` | the generating command, committed alongside |
| AT-33-E1-004 | `scripts/verify.sh` stage | `scripts/verify.sh --only denominator-gate` |
| AT-33-E2-001 | `artifacts/epic-2-oracle-harness/build-transcript.md` | the build command and its real output |
| AT-33-E2-002 | the `.pcg`, the template, the emitted output | the export command |
| AT-33-E2-003 | `artifacts/epic-2-oracle-harness/harness-fixtures/` | fixture set exercising `agree`/`disagree`/`unverifiable` |
| AT-33-E2-004 | Epic 2 closing receipt + `progress.md` entry | the Path A/B ruling, stated |
| AT-33-E3-001 | `artifacts/epic-3-engine-coverage/coverage-gap-rootcause.md` | traced coordinates, per family |
| AT-33-E3-002/003 | per-family coverage table in the receipt | run-population vs true-population, both stated |
| AT-33-E3-004 | regenerated `formula_interpreter.corpus-wide.json` | `README.md §4` row G comparison → `0` |
| AT-33-E4-001 | `artifacts/epic-4-unknown-classification/unknown-rootcause.md` | cause established before any count moves |
| AT-33-E4-002 | updated `docs/work-inventory.json` | G3's command → `0` |
| AT-33-E4-003 | `THE-BOX.md` groups | `box_ledger.py --check` |
| AT-33-E5-001/002 | `artifacts/epic-5-reverification/` per-unit rows | agreement + disagreement counts, with denominator |
| AT-33-E5-003 | `progress.md` entries | one per disagreement, each to a commit or an escalation |
| AT-33-E6-001 | final-acceptance scan receipt | every criterion and every kanban card `complete` |
| AT-33-E6-002 | `docs/retro/sd33-computed-value-verification-retrospective.md` | cited from `references/README.md` in the same cycle |
| AT-33-E6-003 | `receipts.md` | architecture-truth-up + graphify receipts; sweep counts |

## 3. Standing verification commands

Run in any cycle that touches the corpus or the inventory:

```bash
# The full gate suite
scripts/verify.sh

# Individual stages seen in SD-32 and expected to remain live
scripts/verify.sh --only pi-sweep
scripts/verify.sh --only shape-coverage-standing-gate
scripts/verify.sh --only corpus-literal-sweep
scripts/verify.sh --only preflight-oracle

# SD-33's own additions
scripts/verify.sh --only denominator-gate        # AT-33-E1-004
python3 scripts/box_ledger.py --check            # AT-33-E1-001/002
```

**Oracle root:** `PCGEN_CORPUS_ROOT` must point at the repo-local slot
`docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data`.
**`~/workspace/repos/pcgen` is forbidden** — `preflight-oracle` PASSes against it silently.

**Separate cargo workspace:** `apps/desktop/src-tauri` is its own workspace. A root sweep does not cover it. Test it explicitly:
```bash
cd apps/desktop/src-tauri && cargo test --locked --bin codex-desktop
```

## 4. What does not count as verification

Recorded because each has occurred in this program:

- **A dispatched agent's self-report.** Check `git log` and the target files.
- **A count that dropped.** Establish *why* first — instrument correction is not closure.
- **A fixture built from the file the engine reads.** That is a mirror, not a check; the expected value must come from bytes the read path does not touch.
- **A percentage without its denominator.** Fails `--only denominator-gate` (`decisions.md §2`).
- **A green suite, a passing budget, or a finished wave.** None of these is closure.
- **`retro.py`'s `deferrals.open`.** It is `deferrals[-limit:]`, not open deferrals.
