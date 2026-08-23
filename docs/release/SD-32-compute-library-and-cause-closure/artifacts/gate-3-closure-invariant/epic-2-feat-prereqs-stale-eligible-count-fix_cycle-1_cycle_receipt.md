# Cycle epic-2-feat-prereqs-stale-eligible-count-fix — Gate 3 (closure invariant) / Card 11 (T9/T12 residual)

- **Card ID:** 11 (`epic-2-cause-closure`) — the standing `cargo test --locked --lib` RED flagged (not
  silently absorbed) by two prior cycles' own receipts (`epic-2-t9-feat-equipment-companion-monster`
  and `epic-2-t12-attribution-gap-shape2`) and left unfixed by both, per their own explicit,
  repeated notes.
- **Commit SHA:** see `git log -1` at push time (this cycle rebases before pushing per §5)
- **Files touched:**
  - `src/rules_core/feat_prereqs.rs` — retargeted one pinned assertion
    (`prerequisite_tests::a_starting_fighter_keeps_a_real_catalog_and_every_denial_states_why`)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped diff, `1bb523773...HEAD -- src/rules_core/feat_prereqs.rs`)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope)
- **Acceptance criterion:** Reproduce the pre-existing RED flagged twice in row 15/card 11's own notes
  (`feat_prereqs::...a_starting_fighter_keeps_a_real_catalog...`, `left: 755, right: 701`), determine
  with evidence whether the catalog or the assertion is wrong, and fix the true side — not repin
  blindly (`decisions.md §1a`).
- **Corpus SHA:** not re-derived this cycle — no corpus-dependent figure was touched; this is a
  Rust-only fixture/assertion fix over an already-committed, already-fixture-checked feat catalog.
- **Status:** complete (this narrow fix only — does not close row 15 or card 11)
- **Notes:** see full body below.
- **Discovery forwards:** none — the defect was already named by two prior receipts; this cycle closes
  the naming, not a new discovery.
- **Next-cycle plan:** row 15's real remaining scope (27,847 kind-unenumerable-object closure) and
  card 11's T12 remainder (7 zero-coverage classes, 80 magnitude-bearing compute functions) are
  untouched by this cycle — see the dispatch report for a full, evidence-based inventory of both.

---

## 1. Reproduction, exactly as reported

```bash
cargo test --locked --lib prerequisite_tests::a_starting_fighter_keeps_a_real_catalog_and_every_denial_states_why
```
```
thread '...a_starting_fighter_keeps_a_real_catalog_and_every_denial_states_why' panicked:
assertion `left == right` failed: a starting Fighter's real eligible-feat count
  left: 755
 right: 701
```

Reproduced byte-for-byte identical to both prior receipts' pasted figures
(`epic-2-t9-feat-equipment-companion-monster_cycle-1_cycle_receipt.md` §9,
`epic-2-t12-attribution-gap-shape2_cycle-1_cycle_receipt.md` §"Suites run") — real, not stale in the
brief.

## 2. Root cause, isolated by class not by trust

`assert_eq!(reports.len(), 2227, ...)` (the sibling assertion three lines above the failing one) was
already correctly updated by commit `fb4f28dad` (T9 feat/equipment lane, "swept 13 files' pinned
counts"). The `eligible` assertion three lines below it was not part of that sweep and was left at its
pre-commit value (`701`).

Isolated the exact 109 keys `fb4f28dad` added to the feat catalog
(`git show fb4f28dad -- src/rules_core/rules_tables/feat_gap_tables.rs | grep '^+' | grep -oP 'key: "\K[^"]+'`
→ 109 unique keys, matching `2227 - 2118`) and re-partitioned the test's own `reports` by that set with
a temporary debug assertion (added, run, then reverted before committing — never shipped):

```
DEBUG total_reports=2227 new_total=109 new_eligible=54 old_total_reports=2118 old_eligible=701
```

- `old_total_reports=2118` / `old_eligible=701` — the **exact** pre-`fb4f28dad` catalog size and
  eligible count, reproduced untouched among the 2,118 pre-existing rows. The pre-existing population
  did not move.
- `new_total=109` — every one of `fb4f28dad`'s added keys is accounted for, none missing or
  duplicated.
- `new_eligible=54` — of the 109 new rows, 54 are eligible for a level-1, 13-DEX, no-feats Fighter and
  55 are correctly denied (spot-checked several of the 55 directly against the corpus diff: Aldori
  swordplay feats gating on `PREABILITY:...,Aldori Dueling Disciple`, Rage-class-feature-gated
  performance-combat feats — genuine PF1e prerequisites a fresh Fighter does not meet).

`701 + 54 = 755` — the failing test's own observed `left` value, exactly. **The catalog grew
legitimately; the assertion was stale.** This is the fifth instance of this bundle's own named
stale-pinned-count-after-legitimate-growth pattern, not a real regression.

## 3. Fix and RED→GREEN, proven live

Retargeted the assertion to `755` with a derivation comment naming the exact mechanism above (not a
bare number). Full suite:

```bash
cargo test --locked --lib prerequisite_tests
```
```
running 7 tests
test ... a_fighter_6_with_dex_17_and_two_weapon_fighting_can_take_it ... ok
test ... a_fighter_1_cannot_take_improved_two_weapon_fighting_and_is_told_why ... ok
test ... an_arg_race_gate_is_enforced_in_both_directions ... ok
test ... an_unknown_feat_id_resolves_to_nothing ... ok
test ... records_with_no_corpus_prerequisite_are_always_eligible ... ok
test ... a_starting_fighter_keeps_a_real_catalog_and_every_denial_states_why ... ok
test ... a_stronger_build_is_eligible_for_a_superset_of_a_weaker_ones_feats ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 2502 filtered out
```

This run also exercises the same test's own denial-reason loop
(`for report in reports.iter().filter(|r| !r.is_eligible) { assert!(!reason.trim().is_empty(), ...) }`)
over the **full** 2,227-row `reports` — old and new rows alike — so every one of the 55 newly-denied
rows carries a stated reason. GREEN confirms this, not merely the count.

**RED proved live** (`decisions.md §3`/`§1a`): mutated the fixed assertion to `756`,
re-ran the single test:
```
assertion `left == right` failed: a starting Fighter's real eligible-feat count
  left: 755
 right: 756
```
Failed for the intended reason (a deliberately wrong pinned value against the same real computation).
Reverted to `755`, re-ran the full `prerequisite_tests` module: 7/7 GREEN again.

## 4. Sweep — no other file pins the stale `701`/related figures

```bash
grep -rn '\b701\b' tests/ src/ apps/ scripts/
grep -rn 'a_starting_fighter_keeps_a_real_catalog\|starting Fighter'"'"'s real eligible' tests/ apps/
```
Only hits for `701` are unrelated (`source_line: 701` in three monster-data fixture files, an
equipment-table inline comment `ue_equip_magic_items.lst:701`, and this cycle's own new derivation
comment). No other file references the fixed test's name or assertion text. No sweep required beyond
the one line changed.

## 5. Scope discipline

This cycle touches exactly the one stale assertion named by the dispatch brief. It does not attempt
row 15's own scope (27,847 kind-unenumerable-object closure) or card 11's T12 remainder (7
zero-coverage classes, 80 magnitude-bearing compute functions) — both are reported, not silently
absorbed, in the dispatch's final report per `AGENTS.md` rule 3.

`df -h /`: reported at end of dispatch.
