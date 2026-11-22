---
canonical: true
owner: operator
bundle_id: SD-38
slug: cyberpunk-red
canonical_branch: tranche-18
target_version: 0.18.0
date: 2026-10-13
status: draft — authored under operator-away defaults; operator rulings on licence and scope are on record
---

# SD-38 Technical Requirements

## 1. Pre-loop prerequisites

| # | Prerequisite | Check | State on 2026-10-13 | Closes at |
|---|---|---|---|---|
| P-1 | PR #398 merged | `gh pr view 398 --json state -q .state` → `MERGED` | `OPEN` | C1 waits (SD-k) |
| P-2 | `tranche-18` on origin from `origin/develop` after #398 | `git ls-remote --heads origin tranche-18` | absent | C1 |
| P-3 | Version `0.18.0` + notes pointer | `decisions.md §2` commands | `0.17.0`; pointer at SD-37 notes | C1 |
| P-4 | Source present | `ls /home/ubuntu/workspace/inbox/cyberpunk-red/pages \| wc -l` → 459; `ls /home/ubuntu/workspace/inbox/cyberpunk-red/cpr-corebook-v122.pdf` | present | — |
| P-5 | Page-image tools | `which pdftoppm pdftotext` → `/usr/bin/pdftoppm`, `/usr/bin/pdftotext` | present | SD-n if lost |
| P-6 | Disk for ≥ 1 full sweep (~24 G) and 3 lanes | `df -h /` | 651 G free of 1.5 T (56% used) | re-checked before every sweep |
| P-7 | Memory for 3 cargo lanes | `free -g \| sed -n 2p` | 122 G total, 52 G free, 117 G available | re-checked before long runs |
| P-8 | Lane target root | `ls -d /home/ubuntu/cargo-target` | present, empty | each lane creates `sd38-<lane>` |
| P-9 | Clean tree | `git status --porcelain \| wc -l` → 0 | 0 (lead's checkout) | every commit |
| P-10 | Package committed | `ls docs/release/SD-38-cyberpunk-red/kanban.md` | in staging | C0 |

## 2. Normative requirements

- **NR-1 (PF/SF byte-identical).** Aldric/Elowen render hashes and the four SF seeds' printed lines are
  unchanged from the C1 base at E0.6, E2.MC, E4.MC and E7.4 (G-1). A diff is a defect (SD-e).
- **NR-2 (engine single source).** Every number on a CPR surface is a `cpr.` explanations row; no TSX
  re-derives one (R2, `decisions.md §10`).
- **NR-3 (paper sheet).** Compute only values that land on the sheet as numbers (`decisions.md §6`).
- **NR-4 (source of truth).** Records come from the page text and page images; every record has
  `source_page`; counts are pinned by MANIFEST through two implementations (G-6).
- **NR-5 (nothing from the source in git).** No PDF, page text or PNG is committed
  (`git ls-files | grep -ciE '\.(pdf|png)$'` unchanged from C1).
- **NR-6 (licence).** LICENSE.json carries the operator's ruling and sign-off; the licence matrix
  carries the DEVIATION row (`decisions.md §4`).
- **NR-7 (no PCGen).** No CPR card uses `crates/codex-ingest` beyond E0.1's named-refusal match arms
  (`decisions.md §23`).
- **NR-8 (fixtures).** A fixture's expected values come from the page or the inventories, never from
  `data/cyberpunk-red/corpus/`.
- **NR-9 (packaging).** Every runtime read of `data/cyberpunk-red/` has a `bundle.resources` entry and a
  `packaged_resources.rs` assertion (G-7).
- **NR-10 (isolated app root).** Any app-launching harness runs under a per-run `XDG_DATA_HOME` (R5).
- **NR-11 (tiering).** Every `agent()` sets `model` (`decisions.md §15`).
- **NR-12 (wired integration).** The four-check audit and the identifier audit pass on every code card;
  `StubAdapter` stops serving `cyberpunk-red` (registry 0002 update line).

## 3. Inherited items

| Item | Source | SD-38 disposition |
|---|---|---|
| `GameSystem` seam, per-system roots | SD-37 E1 | Widened (E0.1) |
| Adapter resolver, envelope router | SD-37 E4.6, E6.1 | Widened (E0.4) |
| PF render-hash harness, SF seed lines | SD-37 E1.4, E5/E6 | Reused unchanged for G-1 |
| Layout registry, restore-starters | tranche-17-ui briefs 2 and 3 (PR #398) | Reused (E3.2, E4.1, E7.1) |
| SD-37 retrospective "Changes for the next bundle" | `docs/retro/sd37-retrospective.md:273` | Rules R8–R13 |
| SD-37 candidates (FSR-C1…C10, incl. the graphify node-count guard and the template §11 fix) | SD-37 `forward-scope-register.md` | Not adopted; listed as inherited candidates in this bundle's FSR §3 |
