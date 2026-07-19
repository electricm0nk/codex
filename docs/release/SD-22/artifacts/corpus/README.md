---
title: SD-22 Corpus Source (representative stubs + operator-supplied slot)
status: representative-content (operator directive 2026-07-19: coding-harness snags surfaced; the prior docs only said "operator-supplied structured-data file" abstractly. The handoff now lands concrete file shapes here so a coding harness reading `docs/release/SD-22/corpus-source-inventory.md` finds what it needs without `~/workspace/` access.)
purpose: "Each per-cycle `corpus_input_path` from `corpus-source-inventory.md` resolves to a file in this directory (or to a file the operator places under `operator-supplied/`). Stub files cover every APG class, every ACG class, every Bestiary 1 default subset, and the shared spell/equipment tables. The operator-supplied slot is where the actual Paizo/PcGen LST or PCC files land at cycle-launch time."
date: 2026-07-19
canonical_branch: tranche/5
kanban_board: codex-tranche-5
mirror_of: ~/workspace/programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/artifacts/corpus/README.md
---

# SD-22 Corpus Source

This directory is the **on-disk shape** of every `corpus_input_path` named in `../../corpus-source-inventory.md`. A coding harness operating on a cold cloud clone reads this directory to know exactly what the cycle expects the source data to look like.

## 1. What lives here

| Subdir | Coverage | Files | Owner |
|---|---|---|---|
| `apg/` | Advanced Player's Guide | `class_alchemist.lst.md`, `class_cavalier.lst.md`, `class_inquisitor.lst.md`, `class_oracle.lst.md`, `class_summoner.lst.md`, `class_witch.lst.md` (six real APG classes; aligns 1:1 with `corpus-source-inventory.md` §1.1. Corrected 2026-07-19: Gunslinger and Magus stubs removed — verified against the real `apg_classes.lst` that neither is APG content; both are Ultimate Combat / Ultimate Magic, out of scope per `decisions.md §1`) | stub (representative content; operator replaces at cycle-launch) |
| `acg/` | Advanced Class Guide | `class_alchemist.lst.md`, `class_arcanist.lst.md`, `class_bloodrager.lst.md`, `class_brawler.lst.md`, `class_hunter.lst.md`, `class_investigator.lst.md`, `class_shaman.lst.md`, `class_skald.lst.md`, `class_swashbuckler.lst.md`, `class_warpriest.lst.md` (ten ACG classes; aligns 1:1 with `corpus-source-inventory.md` §2.1) | stub (representative content; operator replaces at cycle-launch) |
| `beastiary1/` | Bestiary 1 | `subset_01_sample.md`, `subset_02_sample.md`, `subset_03_sample.md` (three stub subsets — Goblin+Cobold CR-1, Orc+Gnoll CR-1/2, Skeleton+Zombie CR-1 undead). Tarrasque lives in `tarrasque_edge_case.md` for the rule-cycle table's extreme-CR coverage. The operator-pinned "default 8 subsets" at launch expands this to subsets 4 through N following the default alphabetical-by-CR-band-then-name ordering. | stub subset, plus the canonical MonsterRef shape |
| `spell-list/` | APG + ACG shared spell lists | `apg_spell_list.lst.md`, `acg_spell_list.lst.md` | stub (canonical spell-table row shape) |
| `equipment-table/` | APG + ACG shared equipment tables | `apg_equipment.lst.md`, `acg_equipment.lst.md` | stub (canonical equipment-row shape) |
| `operator-supplied/` | Licensed Paizo/PcGen LST + PCC files | **empty at seed**; the operator populates this directory at SD-22 cycle-launch with the licensed corpus files | operator-only |

## 2. Why stubs and not the real Paizo data

The Codex repo is published to an internal-git-host where (a) Paizo's licensed content cannot be checked in (EULA), and (b) the operator's licensed bundle is supplied separately. What **can** land in the repo is the *shape* of the file — the column names, the row schema, the key conventions, the field types — so a coding harness reads the stub, writes a Rust parser that consumes it, and at operator-supplied cycle-launch swaps the stub for the licensed file with no parser change. The inventory's `corpus_input_path` is a *path*, not a *binding* to the stub; the operator's launch-time replacement works as long as the schema matches.

## 3. Reading order for a coding harness on a cold clone

1. `../../corpus-source-inventory.md` — the four-tuple per content unit (rust_module, test_fixture, cycle_artifact, RuleSetId), plus the corpus_input_path.
2. `../../ingest.md` — the operator-pinned doctrine: how raw source files become Rust modules, step by step.
3. **This directory** — the file-shape stubs that the harness reads as the "shape of the source." The number of `apg/class_*.lst.md` files equals `corpus-source-inventory.md` §1.1's rows; same for ACG and Bestiary 1.
4. `../../loop-instruction.md` Step 5 — the per-cycle procedure (the harness runs RED on the stub, swaps to operator-supplied at cycle-launch, runs GREEN).
5. `../../acceptance-and-verification.md` §"Per-criterion closure gate → artifact map" — the per-criterion artifact path table; the per-cycle artifacts in `../../artifacts/` (sibling directory) satisfy gates 1-31 when Epic 9 evaluates.

## 4. Operator-supplied workflow

At cycle-launch, the operator runs:

```bash
# Replace a stub with the licensed file when the operator has the licensed bundle
cd /home/ubuntu/workspace/repos/codex
mkdir -p docs/release/SD-22/artifacts/corpus/operator-supplied/apg
cp /path/to/licensed/apg-class-alchemist.lst \
   docs/release/SD-22/artifacts/corpus/operator-supplied/apg/class_alchemist.lst

# Alternative: re-name the stub to .pending and the loop won't read it
mv docs/release/SD-22/artifacts/corpus/apg/class_alchemist.lst.md \
   docs/release/SD-22/artifacts/corpus/apg/class_alchemist.lst.md.pending
```

The operator pins the binding in `loop-instruction.md` Step 5's cycle-card (the `corpus_input_path` value), and the loop's per-cycle Step-5 implementation runs the parser against that file. The swap is operator-driven; the harness never reads or copies licensed content out of the operator-supplied slot.

## 5. Why this directory is NOT checked into a publisher-only mirror

Each `*.lst.md` stub ships as a `.md` file (markdown-wrapped structured record) so the GitHub-repo viewer shows them as diffable artifacts rather than as opaque binary blobs. The actual `.lst` files are *not* markdown-wrapped; they're plain Paizo-LST format. The `.md` suffix on the stubs is a **schema-preview** convention, not a content-type indicator. When the operator swaps in the licensed file, the file is renamed from `.lst.md` (preview) to `.lst` (canonical).

## 6. Recorded

Added 2026-07-19 per operator directive ("coding harness ran into some snags ... need to provide information how that is done, and source that content in an artifacts folder local to the repo. Any lst of pcc files that we needed have to be in that folder. references to those files need to be made in the handover"). 8 APG + 10 ACG + 3 Bestiary 1 + 2 spell-list + 2 equipment-table + the operator-supplied slot = 26 stub files + 1 README. Total +26 files (added in a single repo commit); mirror at the operator-workspace `programs/cod.../requirements/SD-22-content-source-ingest-and-dm-toolkit/artifacts/corpus/`.
