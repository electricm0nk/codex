# Cycle C0 — Epic C / package + unattended-mode receipts committed

- **Card ID:** C0
- **Commit SHA:** 1102f34aab (package), a53be2166a (Workflow script), 36d7c51beb (run id in progress.md)
- **Files touched:** `docs/release/SD-38-cyberpunk-red/**` (16 chassis files, `artifacts/cycle_0/{inventory/*,sd38-workflow.js,gen_sd38_workflow.py}`)
- **Dual-audit result:** not applicable (planning docs only; no `src/`, `apps/`, `crates/`, `scripts/`, `tests/`, `data/` paths): `git show --name-only --format= 1102f34aab a53be2166a 36d7c51beb | grep -vc '^docs/release/SD-38-cyberpunk-red/'` → 0
- **Acceptance criterion:** The package is committed at `docs/release/SD-38-cyberpunk-red/` with the unattended-mode protocol in its three places.
- **Figures + their re-derive commands:**
  - 16 chassis files: `ls docs/release/SD-38-cyberpunk-red/{README,scope-draft,decisions,workflow-instruction,progress,kanban,epic-breakdown,acceptance-and-verification,content-unit-inventory,risks-and-open-questions,forward-scope-register,technical-design,technical-requirements,release-notes}.md docs/release/SD-38-cyberpunk-red/{artifacts,references}/README.md | wc -l` → 16
  - 68 cards: `awk -F'|' '$2 ~ /^ (C|E)[0-9]/{n++} END{print n}' docs/release/SD-38-cyberpunk-red/kanban.md` → 68
  - unattended mirror: `grep -c 'UNATTENDED MODE' docs/release/SD-38-cyberpunk-red/workflow-instruction.md` ≥ 1; `grep -c '^## Cycle 0' docs/release/SD-38-cyberpunk-red/progress.md` → 1; `grep -c '^## §16' docs/release/SD-38-cyberpunk-red/decisions.md` → 1
  - placeholder gate: `grep -rn '<[a-z_-]*>' docs/release/SD-38-cyberpunk-red/*.md | wc -l` → 62, all classified in `workflow-instruction.md §9`
- **Row-count command output:** 68 (kanban) / 68 (epic-breakdown §0): `awk -F'|' '/^## §0/{s=1} /^## Epic C/{s=0} s && /^\| (C|E)[0-9]/{n++} END{print n}' docs/release/SD-38-cyberpunk-red/epic-breakdown.md`
- **Build scope verified:** none (no code). `node --check docs/release/SD-38-cyberpunk-red/artifacts/cycle_0/sd38-workflow.js` → exit 0 at a53be2166a; `awk '/agent\(/ && !/model:/' <script>` prints nothing.
- **Movement, four buckets:** closure 1 card (C0); reclassification 0; reachability 0; measurement-correction 4 CUI commands fixed by the pre-commit audit (F-3, F-4, F-62, F-63)
- **Status:** complete
- **Notes:** package authored by an Opus 5.5 lane from the planner's brief + seven Sonnet inventories; audited by a second Opus lane (15 fixes, 3 lead rulings). Branch `tranche-18` cut by the lead from `tranche-17-ui` tip bb6107aee0 (PR #398 head; `git merge-base --is-ancestor origin/develop bb6107aee0` → 0). SD-k revised accordingly.
- **Next-cycle plan:** C1 (Haiku): 0.18.0 on the 14 surfaces + `current-release.json`, one commit.
