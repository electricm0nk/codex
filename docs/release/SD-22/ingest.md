---
title: SD-22 Content-Source Ingest — Process Doctrine
status: active (operator directive 2026-07-19)
scope: docs/release/SD-22
artifact_type: process-doctrine
canonical_branch: tranche/5
purpose: |
  How a coding harness on a cold cloud clone ingests SD-22's content-source
  from the operator-supplied licensed files into Rust modules + cycle artifacts.
  Every Epic 3 / 4 / 5 / 6 cycle reads this file before RED phase.
date: 2026-07-19
companion_to: ../corpus-source-inventory.md
mirror_of: ~/workspace/programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/ingest.md
---

# SD-22 Content-Source Ingest — Process Doctrine

This file is the **canonical ingest pipeline** for SD-22's content-source work. Every Epic 3 / 4 / 5 / 6 cycle reads `corpus-source-inventory.md` (the per-content-unit four-tuple), then this `ingest.md` (the per-cycle pipeline), then runs the pipeline's commands.

## 1. What "ingest" means here

For a single content unit (e.g. APG class Alchemist), the ingest cycle produces four artifacts:

1. **A Rust module** at `<rust_module_path>` (e.g. `src/rules_core/rules_tables/apg/class_alchemist.rs`).
2. **A test fixture** at `<test_fixture_path>` (e.g. `tests/sd22_apg_class_alchemist_resolves.rs`).
3. **A cycle artifact** at `<cycle_artifact_path>` (e.g. `docs/release/SD-22/artifacts/apg/class_alchemist_cycle_receipt.md`).
4. **A registered `RuleSetId` variant** (e.g. `RuleSetId::Apg`) wired into the resolver chain.

The input is a corpus source file at `<corpus_input_path>` (e.g. `docs/release/SD-22/artifacts/corpus/apg/class_alchemist.lst`).

For SD-22's 30 ingest criteria (Epics 3+4+5+6, criteria 6-21), that means **30 cycles × ~4 artifacts = ~120 artifact files** if every cycle produces a fresh receipt; in practice the per-class cycles can collapse to "one class_table.rs + one test + one receipt," so the actual artifact count lands around 50 (per `corpus-source-inventory.md` §3 of the cycle-artifact contract).

## 2. The per-cycle pipeline (RED → GREEN → REFACTOR)

This is the canonical pipeline. Each step is mandatory per operator-pinned 2026-07-19 red-green TDD mandate.

### 2.1 RED — write the failing test

```
# 1. Read the inventory row for this cycle's content unit
corpus-source-inventory.md §1.1 (APG) / §2.1 (ACG) / §3.1 (Bestiary 1) / §4.1 (DM Toolkit)
# 2. Identify rust_module_path, test_fixture_path, cycle_artifact_path, RuleSetId

# 3. Write the test fixture (test_fixture_path) with at least these assertions:
cargo new --lib test_fixture  # only first cycle per epic; subsequent share structure

cat > tests/sd22_apg_class_alchemist_resolves.rs << 'EOF'
#[test]
fn apg_class_alchemist_resolves_via_apg_rule_set() {
    assert!(RuleSetId::Apg.resolve("apg:class:alchemist").is_some());
}
#[test]
fn apg_class_alchemist_does_not_resolve_via_crb_rule_set() {
    assert!(RuleSetId::Crb.resolve("apg:class:alchemist").is_none());
}
#[test]
fn apg_alchemist_discovery_count_at_level_6_is_2() {
    let table = apg::class_alchemist::table();
    assert_eq!(table.discovery_known_at_level(6), 2);
}
EOF

# 4. Confirm RED — test fails for the intended reason
cargo test --locked --test sd22_apg_class_alchemist_resolves 2>&1 | tail -40
```

The `cargo test` output is RED evidence. **Persist it** to the cycle artifact's "Red-phase evidence" section. If the test fails for an *un*intended reason (compile error in production code, missing dependency), that's a Bucket-B shortfall — fix the test setup, don't carry the cycle forward.

### 2.2 GREEN — implement the module

```
# 1. Write the production module at rust_module_path, consuming the corpus file
cat > src/rules_core/rules_tables/apg/class_alchemist.rs << 'EOF'
use crate::rules_core::corpus::{SourcePackageContent, parse_lst};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AlchemistClassTable {
    pub name: String,
    pub bab_progression: BabProgression,
    pub save_progression: SaveProgression,
    pub starting_gold: u32,
    pub hd: HitDie,
    pub level_features: HashMap<u8, Vec<AlchemistFeature>>,
    pub spells_per_day_by_level: HashMap<u8, SpellSlots>,
    pub extracts_known_by_level: HashMap<u8, u8>,
    pub discovery_count_by_level: HashMap<u8, u8>,
    pub mutagen_bomb_mutex_at_level: HashMap<u8, bool>,
}

pub fn load_alchemist_class_table(corpus: &SourcePackageContent) -> AlchemistClassTable {
    let raw = parse_lst(&corpus.apg_class_alchemist_lst).expect("apg:class:alchemist must parse");
    AlchemistClassTable {
        // ... parses the [header], [level_features], [spells] sections from the stub
        // ... applies the discovery-count + mutagen-bomb-mutex rules
        ..Default::default()
    }
}
EOF

# 2. Implement the parser if the corpus-type is non-standard (e.g., PCGen PCC format)
# For LST (tab-separated per the stub), use a small crate or write a hand parser.
# The parser's column-count validation lives at the entry point.

# 3. Wire the RuleSetId::Apg resolver — append to src/rules_core/rules_tables/apg/mod.rs:
pub mod class_alchemist;
pub mod spell_list;
pub mod equipment_tables;
pub fn resolve_apg(key: &str, corpus: &SourcePackageContent) -> Option<ApgRecord> {
    match key {
        "apg:class:alchemist" => Some(ApgRecord::Alchemist(class_alchemist::load_alchemist_class_table(corpus))),
        // ... other keys per cross-book invariants
        _ => None,
    }
}

# 4. Confirm GREEN — full tests, clippy clean
cargo test --locked 2>&1 | tail -20
cargo clippy --locked --tests -- -D warnings 2>&1 | tail -20
```

The output is GREEN evidence. **Persist it** to the cycle artifact's "Green-phase evidence" section.

### 2.3 REFACTOR (optional; only after green)

Refactor is permitted only after a cycle's GREEN phase. A cycle that refactors first is a Bucket-B shortfall (the cycle artifact must show RED → GREEN in that order; refactor moves are post-GREEN with `cargo test --locked` + clippy held green throughout).

```
# Common refactor operations:
#  - extract a small helper (e.g., parse_lst_section) and reuse across class parsers
#  - move the dispute cell-lookup logic (e.g., MutagenBombMutexAtLevel) into a generic-named fn
#  - update the cross-book-invariant table in corpus-source-inventory.md if the invariant
#    was mis-stated (operator-pinned at end of cycle)
```

### 2.4 MINT the cycle artifact

```
# Write the cycle artifact per corpus-source-inventory.md §6 contract
cat > docs/release/SD-22/artifacts/apg/class_alchemist_cycle_receipt.md << 'EOF'
# Alchemist cycle receipt — 2026-07-19T14:32:18Z

## Red-phase evidence
cargo test --locked --test sd22_apg_class_alchemist_resolves 2>&1 | tail -40
(…test fails because src/rules_core/rules_tables/apg/class_alchemist.rs does not exist yet…)

## Green-phase evidence
cargo test --locked 2>&1 | tail -20
cargo clippy --locked --tests -- -D warnings 2>&1 | tail -20
(…test passes, clippy clean…)

## Files touched
- src/rules_core/rules_tables/apg/class_alchemist.rs (NEW; load_alchemist_class_table)
- src/rules_core/rules_tables/apg/mod.rs (MODIFIED; pub mod class_alchemist; resolve_apg)

## Cycle metadata
- cycle_id: 2026-07-19T14:32:18Z
- duration: 117 seconds
- bundle_criterion: criterion-7 (APG per-class cycles)
- upstream reference: docs/release/SD-22/artifacts/corpus/apg/class_alchemist.lst.md (stub) -> docs/release/SD-22/artifacts/corpus/operator-supplied/apg/class_alchemist.lst (operator-supplied swap at cycle-launch)
- RuleSetId: Apg

## kanban
- card: <hermes kanban card id, e.g. t_a824a37b>
- audit_comment: <comment id>
EOF

# Mint the kanban post-mortem card (per loop-instruction.md Step 10)
hermes kanban --board codex-tranche-5 create \
  "SD22 class_alchemist ingest (Epic 3 cycle) [cycle <cycle-id>]" \
  --assignee operator --workspace scratch \
  --initial-status done --created-by operator --priority 3 \
  --body "<card body per loop-instruction.md Step 10 schema>"
```

A cycle that ships without a cycle artifact is a Bucket-B shortfall — Epic 9's evaluator (criterion-31) cannot conclude criterion-7 `complete` without `apg/class_alchemist_cycle_receipt.md` existing with RED→GREEN transitions persisted.

### 2.5 COMMIT + PUSH

```
git add src/rules_core/rules_tables/apg/class_alchemist.rs \
        src/rules_core/rules_tables/apg/mod.rs \
        tests/sd22_apg_class_alchemist_resolves.rs \
        docs/release/SD-22/artifacts/apg/class_alchemist_cycle_receipt.md

git -c user.name='Todd Hintzmann' \
    -c user.email='todd@hintzmann.net' \
    commit -m "feat(sd22): APG Alchemist ingest (Epic 3 cycle, criterion-7)"

git push origin tranche/5
```

Commit message convention follows existing per-cycle pattern: `feat(sd22): <class-or-monster-or-table> ingest (Epic N cycle, criterion-NN)`.

## 3. Cross-book resolution (Epic 3+4+5+6 cross-cutting cycles)

Cross-book-invariant cycles (Epic 3 criteria 8, Epic 4 criteria 12, Epic 5 criteria 16) assert that keys present in one book resolve `Some` for the matching `RuleSetId` and `None` for the others:

```rust
#[test]
fn apg_alchemist_resolves_via_apg_but_none_for_crb() {
    let corpus = load_corpus("docs/release/SD-22/artifacts/corpus/operator-supplied/");
    assert!(RuleSetId::Apg.resolve("apg:class:alchemist", &corpus).is_some());
    assert!(RuleSetId::Crb.resolve("apg:class:alchemist", &corpus).is_none());
    assert!(RuleSetId::Acg.resolve("apg:class:alchemist", &corpus).is_none());
    assert!(RuleSetId::Bestiary1.resolve("apg:class:alchemist", &corpus).is_none());
}

#[test]
fn bestiary_goblin_resolves_via_bestiary1_only() {
    let corpus = load_corpus("docs/release/SD-22/artifacts/corpus/operator-supplied/");
    assert!(RuleSetId::Bestiary1.resolve("beastiary1:monster:goblin", &corpus).is_some());
    assert!(RuleSetId::Crb.resolve("beastiary1:monster:goblin", &corpus).is_none());
}
```

These tests **must pass** before Epic 6's happy-path integration test consumes them. `corpus-source-inventory.md` §1.3 / §2.3 / §3.2 carries the full cross-book invariants per book.

## 4. Epic 6 — DM Toolkit happy-path integration

Epic 6's criterion 21 is the load-bearing surface for Epic 9's evaluation: it consumes Epic 3+4+5 output into a campaign-shape fixture and runs the DM-toolkit encounter math against it.

```rust
#[test]
fn dm_toolkit_happy_path_4_level_3_pcs_vs_1_goblin_is_easy() {
    // Epic 6's happy-path ingestion: read APG class_Fighter (CRB), ACG class_Hunter (ACG),
    // Beastiary 1 monster_goblin, build a PartySnapshot from the Fighter + the Hunter,
    // and compute encounter difficulty.
    let corpus = load_corpus("docs/release/SD-22/artifacts/corpus/operator-supplied/");
    let party = build_party_from_classes(&["crb:class:fighter", "acg:class:hunter"], &[3, 3, 3, 3]);
    let monsters = vec![corpus.monster("beastiary1:monster:goblin").expect("goblin")];
    let result = encounter_difficulty(&party, &monsters);
    assert_eq!(result.difficulty, Difficulty::Easy);
}
```

The test is RED until Epic 3+4+5 ship at least one ingested `PartySnapshot` and one ingested `MonsterRef`. Epic 6's cycle picker enforces this dependency per `loop-instruction.md` Step 1.

## 5. Operator-supplied swap procedure (handoff between operator and the loop)

The operator populates `docs/release/SD-22/artifacts/corpus/operator-supplied/{apg,acg,beastiary1}/` with licensed files at cycle-launch time, *not* at bundle-launch. The swap is per-cycle:

```
# Before Epic 3 cycle-N for class_alchemist (operator-side):
cd /home/ubuntu/workspace/repos/codex
cp /path/to/licensed/apg/class_alchemist.lst \
   docs/release/SD-22/artifacts/corpus/operator-supplied/apg/

# Mark the stub as superseded (rename to .superseded so the loop won't read it)
mv docs/release/SD-22/artifacts/corpus/apg/class_alchemist.lst.md \
   docs/release/SD-22/artifacts/corpus/apg/class_alchemist.lst.md.superseded

# Run the loop's cycle
hermes loop sd22.alchemist.cycle

# After cycle: the cycle artifact is in docs/release/SD-22/artifacts/apg/class_alchemist_cycle_receipt.md
# Postmortem: leave the operator-supplied file in place for the next cycle's reload-license-cache
```

The `.gitignore` ensures the operator-supplied files never commit accidentally. The `.superseded` stub stays in the repo as the schema-of-record reference.

## 6. Where this pipeline lands in the bundle

- **`corpus-source-inventory.md`** is the per-cycle "what to ingest + where to land it" reference.
- **`ingest.md`** (this file) is the per-cycle pipeline.
- **`loop-instruction.md` Step 4-5** enforces the red-green TDD mandate + cross-references this `ingest.md`.
- **`artifacts/corpus/`** ships the on-disk file-shape stubs (this commit, 26 files).
- **`artifacts/corpus/operator-supplied/`** is the load-bearing slot the operator populates with licensed content (gitignored except for the README).
- **`corpus-source-inventory.md` §6** specifies the cycle-artifact reader's contract.

## 7. Recorded

Added 2026-07-19 per operator directive ("instructions on the ingest process we developed need to be explained as well"). Authored alongside the corpus-stubs seed (26 files) and the operator-supplied gitignore rule. The doctrine-of-record against which every future ingest bug is diagnosed; cycle authors writing parsers read this file before writing the parser. If the operator extends the bundle to a new book (Ultimate Combat, etc.) the same pipeline + a new `corpus/<book>/` stub directory + a new inventory section extends with no schema change.
