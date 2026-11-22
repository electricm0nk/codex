# operator-supplied/ — Licensed corpus slot

This directory is the load-bearing slot for the actual Paizo/PcGen licensed corpus files that SD-22 ingests. **It is empty at seed** because the licensed content is operator-supplied, not bundled in the repo.

## What goes here

The operator copies licensed LST/PCC files into per-book subdirectories at SD-22 cycle-launch:

```bash
cd /home/ubuntu/workspace/repos/codex/docs/release/SD-22/artifacts/corpus/operator-supplied

# Paizo APG-licensed class LSTs
mkdir -p apg
cp /path/to/licensed/apg/class_alchemist.lst     apg/
cp /path/to/licensed/apg/class_cavalier.lst     apg/
cp /path/to/licensed/apg/class_gunslinger.lst   apg/
cp /path/to/licensed/apg/class_inquisitor.lst   apg/
cp /path/to/licensed/apg/class_magus.lst        apg/
cp /path/to/licensed/apg/class_oracle.lst       apg/
cp /path/to/licensed/apg/class_summoner.lst     apg/
cp /path/to/licensed/apg/class_witch.lst        apg/

# Paizo ACG-licensed class LSTs
mkdir -p acg
cp /path/to/licensed/acg/*.lst acg/

# Paizo Bestiary 1 licensed roster
mkdir -p beastiary1
cp /path/to/licensed/beastiary1/*.lst beastiary1/

# Optional: a PcGen PCC campaign file that pre-rolls encounters / party compositions
mkdir -p pcgen
cp /path/to/pcgen/yourcampaign.pcc pcgen/
```

## Why this lives in `docs/release/SD-22/artifacts/corpus/operator-supplied/` and not at repo root

Three reasons:

1. **Self-sufficiency.** The coding harness reads only the repo; if licensed files are in `~/workspace/`, the harness can't reach them when the operator is offline.
2. **Per-bundle ownership.** SD-22's licensed files belong to SD-22. If SD-23 ingests Ultimate Combat, it gets its own `docs/release/SD-23/artifacts/corpus/operator-supplied/`. No cross-bundle contamination.
3. **Clear gitignore frontier.** `.gitignore` excludes `**/corpus/operator-supplied/**` (see the repo's `.gitignore`) so licensed content is never accidentally committed. Paizo EULA is enforced by exclusion, not by operator self-discipline.

## What's published to origin's branch

Only the **stubs** under `docs/release/SD-22/artifacts/corpus/{apg,acg,beastiary1,spell-list,equipment-table}/` ship to origin's `tranche/5`. The `operator-supplied/` directory's contents are operator-local; the placeholder `README.md` ships so the cold-clone reader knows what the operator populates at cycle-launch.

## The shape contract

The operator-supplied file MUST match the column-count schema of the corresponding stub in the parent directories:

| Stub file | Operator-supplied filename | What changes |
|---|---|---|
| `apg/class_alchemist.lst.md` | `apg/class_alchemist.lst` | Body (between `# === operator-replace point ===` markers) is replaced with the licensed LST; column-count contract from `[header]` block stays. |
| (same for all 18 APG/ACG/Bestiary 1 stubs) | | |

Rust parsers written against the stubs parse the licensed file unchanged. The cycle's `corpus_input_path` in `corpus-source-inventory.md` references the *stub* (the path the parser knows); when the operator swaps the licensed file in, it MUST have the same filename (just stripped of the `.md` suffix) so the parser finds it without code change.

## Recorded

Added 2026-07-19 per operator directive ("source that content in an artifacts folder local to the repo"). Sibling to `docs/release/SD-22/artifacts/corpus/{apg,acg,beastiary1,spell-list,equipment-table}/` stubs (which are the on-disk shape the operator's licensed files must satisfy).
