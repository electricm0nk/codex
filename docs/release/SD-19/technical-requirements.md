---
title: SD-19 — Technical Requirements (Pre-Loop Prerequisites)
status: draft (operator review required)
date: 2026-07-14
companion_to: /home/ubuntu/workspace/SD-19-core-rules-spell-equipment-reachability-scope-draft.md
---

# SD-19 — Technical Requirements

The capability slices (foundation slice as the first SD-19 atomic commit, main capability slice as the second) cannot begin until every prerequisite in this file is satisfied. The loop cannot begin until both capability slices have landed on `tranche/3` AND SD-18's loop has completed. Each prerequisite below is independently verifiable; the verification command is the contract.

## 1. SD-18's §1.1 pre-loop gate must be shipped

The `ComposedCharacterInput` ship from SD-18 §1.1 (commit `5c982d6`, PR #301) is the structural predecessor SD-19 builds on. The capability slice's `compute_pilot_with_corpus` reads its corpus parameter as `&SourcePackageContent` (the same type `ComposedCharacterInput.corpus` carries); without SD-18 §1.1 shipped, that type may not be in its current shape.

**Verification**:
```bash
cd /home/ubuntu/workspace/repos/codex
git cat-file -e origin/tranche/3:src/rules_core/composed_input.rs && echo "OK"
grep -n "pub corpus" src/rules_core/composed_input.rs
```

## 2. SD-17 corpus-side coverage must be green

The corpus-side halves of §2.4 and §2.5 are already proven (per the SD-18 investigation cycles). The capability slices depend on those test suites being green at the time of slice-ship, not just historically.

**Verification**:
```bash
cd /home/ubuntu/workspace/repos/codex
CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data \
  cargo test --locked --test sd17_b_spells 2>&1 | tail -10
# Expect: 13/13 green

CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data \
  cargo test --locked --test sd17_b5_equipment 2>&1 | tail -10
# Expect: 21/21 green
```

## 3. The 13 sd19_seam_crb_*.txt fixtures must be hand-typed from real PCGen corpus records

Per `decisions.md` §9 and `technical-design.md` §5.2, the foundation slice's CRB structured data files and the main capability slice's `sd19_seam_crb_*.txt` fixtures are sourced verbatim from real PCGen corpus records (read from `pathfinder/paizo/roleplaying_game/core_rulebook/cr_spells.lst` for spells and the four `cr_equip_*.lst` files for equipment). Each fixture is prepended with a one-line source-naming comment. A future operator can verify currency by re-running the slice executor's grep against the corpus and confirming the source line still matches the fixture content.

**Verification**:
```bash
# At slice-ship time:
cd /home/ubuntu/workspace/repos/codex
ls tests/fixtures/rules_core/sd19_seam_crb_*.txt | wc -l   # expect: 13
for f in tests/fixtures/rules_core/sd19_seam_crb_*.txt; do
  head -1 "$f"  # every fixture has a source-naming comment
done
```

## 3. SD-18 §3.4/§3.5 dated blocker entries must be required reading

The capability slice's PR description and the loop brief's "required reading" section both reference the SD-18 progress doc's §3.4 and §3.5 entries (anchored under the dated cycle headers) by line number. The slice cannot proceed until the operator (and the slice executor) have acknowledged those entries as the design rationale. This is a documentation prerequisite, not a code one — but it is the difference between a slice that closes the gap and a slice that re-discovers it.

**Verification**:
```bash
# Capability-slice commit body must include this line under its "Why this slice" section:
# "Closes the structural gap documented in
#  ~/workspace/SD-18-core-rules-breadth-progress.md
#  under the headings 2026-07-15T0300 (§3.4) and 2026-07-15T0400 (§3.5)."
```

## 4. The `pilot_compute.rs` and `support_state_matrix.rs` files must be clean on the working tree

When the SD-19 loop begins, its working tree must be on `tranche/3` with no outstanding conflicts on the two choke-point files (`pilot_compute.rs`, `support_state_matrix.rs`). This is normally trivial since SD-19 commits directly to `tranche/3`, but is named here because the SD-18 loop may have just finished and may have left uncommitted notes or in-progress test scaffolding.

```bash
cd /home/ubuntu/workspace/repos/codex
git fetch origin tranche/3
git checkout tranche/3
git pull origin tranche/3
git status --porcelain | wc -l   # expect 0; if non-zero, exit and reconcile manually
```

## 5. The seam's required public types must exist in `pcgen_import::corpus`

The capability slice's resolvers reference `EquipmentRecord` and `SpellRecord`. These are SD-17 types and must already exist with the fields the resolvers expect (`EquipmentRecord.name: String`, `SpellRecord.key: String` or equivalent). The exact field names are finalized at slice-implementation time; the prerequisite is that the types exist at all.

**Verification**:
```bash
cd /home/ubuntu/workspace/repos/codex
grep -rn "pub struct EquipmentRecord" src/pcgen_import/corpus/ 2>/dev/null || \
  grep -rn "pub struct EquipmentRecord" src/pcgen_import/ 2>/dev/null
grep -rn "pub struct SpellRecord" src/pcgen_import/corpus/ 2>/dev/null || \
  grep -rn "pub struct SpellRecord" src/pcgen_import/ 2>/dev/null
# Both must return at least one match. Field-level shape is verified by the
# slice's own RED tests; this prerequisite only verifies the types exist.
```

## 6. The SD-18 loop must be complete before SD-19 begins

Per operator directive 2026-07-14 ("ok, lock it in"), SD-19 cannot begin until SD-18's loop completes its lane (or is otherwise paused by the operator). SD-19 cannot run concurrently with SD-18's loop on the same `tranche/3` branch — the timing is structural, not preference.

**Why the wait is structural, not preference.** SD-19's main capability slice's diff to `support_state_matrix.rs` (adding `MatrixSubjectType::School` and `::Equipment` row shapes alongside the existing `Race | Class | Interaction` shapes) is non-zero. SD-18's loop's cycles also touch `support_state_matrix.rs` for every §3.1 race, §3.2 class, and §3.3 interaction row update. Concurrent execution on the same `tranche/3` branch creates a real race: SD-18's cycle reading `MatrixSubjectType::School(...)` after SD-19's slice lands but before SD-18's matrix loader has seen the new variants would silently misroute or panic; SD-19's slice landing mid-loop creates rebase churn on every subsequent SD-18 cycle. The foundation slice, even though it doesn't touch `pilot_compute.rs` or `support_state_matrix.rs`, still lands on the same `tranche/3` branch and shows up in SD-18's `git fetch origin tranche/3` log on every subsequent cycle — coordination friction for zero time savings.

**What this means in practice.** If SD-19 fires the foundation slice while SD-18 is still running, the only saving is the ~5 minutes of foundation-slice commit work, while the cost is persistent rebase noise on every subsequent SD-18 cycle for the next 48+ hours. SD-19 starts when SD-18's loop completes; the operator spends the SD-18 wait window on the parallel GUI work (PR #316 campaign manager / theme integration) or on next-book table-store preparation instead.

**Verification** (operator-driven; not automatable):
```bash
# Operator's pre-slice checklist:
# 1. Confirm SD-18 /loop is no longer running (ps -eo pid,etime,stat,cmd | grep claude | grep -v grep returns nothing, OR the running claude process is not on tranche-3)
# 2. Confirm SD-18's last cycle entry in ~/workspace/SD-18-core-rules-breadth-progress.md is GREEN or FAIL (not in-flight)
# 3. Commit the SD-19 capability slice directly to tranche/3
# 4. Launch the SD-19 /loop invocation
```

## 7. Cross-reference

- `decisions.md` §1 (capability slice + loop pattern), §2 (equipment resolver), §3 (spell selection), §4 (additive seam), §5 (matrix extension), §6 (loop posture inheritance).
- `technical-design.md` §1 (seam signature), §2 (resolver signatures), §3 (CharacterInput extension), §4 (MatrixSubjectType extension), §5 (slice PR structure), §6 (cycle surface), §7 (branch base).
- `acceptance-and-verification.md` — closure gates.
- `risks-and-open-questions.md` — per-criterion risks; the two open override flags.
- `~/workspace/SD-18-core-rules-breadth-progress.md` the dated cycle-2026-07-15T0300 (§3.4) and cycle-2026-07-15T0400 (§3.5) headers — the blocker entries.