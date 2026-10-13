---
canonical: true
bundle_id: SD-38
slug: cyberpunk-red
canonical_branch: tranche-18
target_version: 0.18.0
date: 2026-10-13
board: local-file ./kanban.md (Hermes board retired 2026-08-01)
status: draft — authored under operator-away defaults; operator rulings on licence and scope are on record
---

# SD-38 Kanban

One row per card; one criterion per card (`epic-breakdown.md`). The dispatched agent that closes a
card edits that card's row in its own closing commit (`workflow-instruction.md §6` step 9). The
orchestrator never moves a row to `complete` from memory: a row moves only together with a receipt
and its commit SHA.

**Status vocabulary:** `ready` (dependencies met) · `waiting` (dependencies unmet) · `in-progress` ·
`complete` · `partial` (blocks closure) · `blocked-escalated` (only an operator ruling clears it; the
ruling needed is under `progress.md ## Open blockers`; blocks closure).

**Column order is load-bearing:** E7.5's scan reads ID as awk field `$2` and Status as `$5`.
**`Depends on` is load-bearing:** the Workflow script refuses to dispatch a card whose dependencies
are not all `complete` (rule R8). Row order is dispatch order.

**Closure rule:** no PR while any row other than E7.11 is anything but `complete`. DEF-1…DEF-5 are not
cards (`decisions.md §22`).

| ID | Title | Tier | Status | Depends on | Receipt / notes |
|---|---|---|---|---|---|
| C0 | Package + unattended-mode receipts committed | lead session | complete | — | `artifacts/cycle_0/C0_cycle_receipt.md`; package 1102f34aab, script a53be2166a, run id 36d7c51beb |
| C1 | Cut `tranche-18` after PR #398; 0.18.0 on 14 surfaces + `current-release.json`, one commit; push | haiku | waiting | C0 | SD-k revised: tranche-18 already cut from PR #398's head bb6107aee0; E7.11 re-checks develop |
| E0.1 | `GameSystem::CyberpunkRed`; `sheet_rules_relative` → `Option` | opus | waiting | C1 |  |
| E0.2 | LICENSE.json + licence-matrix section (DEVIATION) | opus | waiting | E0.1 |  |
| E0.3 | CPR corpus root bundled + `packaged_resources.rs` | opus | waiting | E0.2 |  |
| E0.4 | Envelope field (A-11) + `CyberpunkAdapter` routing; STAT rows | opus | waiting | E0.1 |  |
| E0.5 | Landing `cyberpunk-red` selectable; Load list; New Character closed | opus | waiting | E0.4 |  |
| E0.6 | PF + SF byte-identical gate (base = C1 SHA) | opus | waiting | E0.1, E0.2, E0.3, E0.4, E0.5 |  |
| E0.MC | E0 adversarial merge check | opus | waiting | E0.6 | writes the E0 epic report (`decisions.md §16` rule 6) |
| E1.0 | MANIFEST.json + `cpr_manifest_check.py` | sonnet | waiting | E0.MC |  |
| E1.1 | Roles lane | sonnet | waiting | E1.0 |  |
| E1.2 | Creation lane | sonnet | waiting | E1.0 |  |
| E1.3 | Skills lane | sonnet | waiting | E1.0 |  |
| E1.4 | Gear lane | sonnet | waiting | E1.0 |  |
| E1.5 | Cyberware lane | sonnet | waiting | E1.0 |  |
| E1.6 | Combat lane (D-1 image check) | sonnet | waiting | E1.0 |  |
| E1.7 | Netrunning lane | sonnet | waiting | E1.0 |  |
| E1.8 | GM-tables lane (inventory first) | sonnet | waiting | E1.0 |  |
| E1.V | Verify lane: 10% sample vs page image; MANIFEST pinned; Rust twin | opus | waiting | E1.1, E1.2, E1.3, E1.4, E1.5, E1.6, E1.7, E1.8 | writes the E1 epic report |
| E2.1 | Lane A: stats (HP table 126 cells) | opus | waiting | E1.2, E1.V |  |
| E2.2 | Lane A: skills (10 lists = 86) | opus | waiting | E1.3, E2.1 |  |
| E2.3 | Lane A: armour | opus | waiting | E1.4, E2.2 |  |
| E2.4 | Lane A: wounds + criticals | opus | waiting | E1.6, E2.3 |  |
| E2.5 | Lane B: cyberware (D-5, D-6, D-11) | opus | waiting | E1.5, E1.V |  |
| E2.6 | Lane B: roles (rank-derived values) | opus | waiting | E1.1, E2.5 |  |
| E2.7 | Lane B: improvement (IP, A-1) | opus | waiting | E1.2, E2.6 |  |
| E2.8 | Lane B: reputation | opus | waiting | E1.2, E2.7 |  |
| E2.9 | Lane C: netrunning (A-7, D-2) | opus | waiting | E1.7, E1.1, E1.V |  |
| E2.10 | Sheet assembly + adapter; p121 check | opus | waiting | E2.4, E2.8, E2.9, E1.V |  |
| E2.MC | E2 adversarial merge check | opus | waiting | E2.10 | writes the E2 epic report |
| E6.1 | Catalog + deck-calculator commands | sonnet | waiting | E1.V, E2.9 |  |
| E6.2 | Catalog screens + landing links | sonnet | waiting | E6.1 |  |
| E6.3 | Netrunning tools screen | sonnet | waiting | E6.2 |  |
| E6.4 | GM reference pages (print-only) | sonnet | waiting | E6.3, E1.8 |  |
| E6.MC | E6 adversarial merge check | opus | waiting | E6.3, E6.4 | writes the E6 epic report |
| E3.1 | Creation commands (3 methods, seedable roller) | opus | waiting | E2.4, E6.1 |  |
| E3.2 | `CyberpunkCreateForm`; New Character opens it | opus | waiting | E3.1, E6.MC |  |
| E3.3 | Lifepath panel; Cultural language 4 | opus | waiting | E3.2 |  |
| E3.4 | Skills allocator | opus | waiting | E3.3 |  |
| E3.5 | Role Ability rank 4 + sub-choices (D-7, A-2) | opus | waiting | E3.4, E2.6 |  |
| E3.6 | Starting kit + cash (A-9) | opus | waiting | E3.5 |  |
| E3.7 | Cyberware pack + Humanity preview + slots | opus | waiting | E3.6, E2.5 |  |
| E3.MC | E3 adversarial merge check (3 methods through the form) | opus | waiting | E3.7, E2.MC | writes the E3 epic report |
| E4.1 | Sheet core + layout registry; no Encumbrance box | opus | waiting | E2.MC, E3.2 |  |
| E4.2 | Skills block | opus | waiting | E4.1 |  |
| E4.3 | Weapons + range DV + armour block | opus | waiting | E4.2 |  |
| E4.4 | Cyberware + Role Ability + deck block | opus | waiting | E4.3 |  |
| E4.5 | Reputation, IP, Lifepath, criticals checklist | opus | waiting | E4.4 |  |
| E4.6 | Print path `cpr_sheet_print.rs` | opus | waiting | E4.5 |  |
| E4.7 | ui-smoke rows + `ui-smoke:doc` | opus | waiting | E4.6, E3.MC |  |
| E4.MC | E4 adversarial merge check | opus | waiting | E4.7 | writes the E4 epic report |
| E5.1 | IP spend (skills, Role rank) | opus | waiting | E3.MC, E4.MC |  |
| E5.2 | Role switch (A-1) | opus | waiting | E5.1 |  |
| E5.3 | Cyberware install with Humanity Loss (A-3, D-11) | opus | waiting | E5.2 |  |
| E5.4 | Therapy (D-5 cap) | opus | waiting | E5.3 |  |
| E5.5 | Reputation change | opus | waiting | E5.4 |  |
| E5.MC | E5 adversarial merge check | opus | waiting | E5.5 | writes the E5 epic report |
| E7.1 | Three CPR starter seeds restorable | opus | waiting | E5.MC |  |
| E7.2 | Packaged resources + clean-checkout build | opus | waiting | E7.1 |  |
| E7.3 | Fresh-install acceptance (.deb, AppImage) | sonnet | waiting | E7.2 | SD-m if the box cannot install |
| E7.4 | Widest verify (both workspaces, npm, `verify.sh`, PF/SF byte-identical) | sonnet | waiting | E7.3 |  |
| E7.5 | Final-acceptance scan (stop if short) | opus | waiting | E7.4 |  |
| E7.6 | Retrospective written + cited | sonnet | waiting | E7.5 |  |
| E7.7 | Worktree/branch sweep | haiku | waiting | E7.6 |  |
| E7.8 | Release notes (marker removed; resolver passes) | sonnet | waiting | E7.7 |  |
| E7.9 | Architecture truth-up + claims critic | sonnet + opus | waiting | E7.8 |  |
| E7.10 | Graphify LAST | sonnet | waiting | E7.9 |  |
| E7.11 | PR `tranche-18 → develop` (final action) + `pr-tests` | sonnet | waiting | E7.10 | operator merges |

**Row check (68 cards).**

```bash
cd docs/release/SD-38-cyberpunk-red
awk -F'|' '$2 ~ /^ (C|E)[0-9]/{n++} END{print n}' kanban.md                     # → 68
python3 -c "import re;print(sum(1 for l in open('kanban.md') if re.match(r'^\| (C|E)[0-9]', l)))"   # → 68
diff <(awk -F'|' '/^\| (C|E)[0-9]/{gsub(/ /,"",$2);print $2}' kanban.md | sort) \
     <(awk -F'|' '/^## Epic C/{s=1} s && /^\| (C|E)[0-9]/{gsub(/ /,"",$2);print $2}' epic-breakdown.md | sort) \
  && echo SAME_IDS
awk -F'|' '/^\| (C|E)[0-9]/{gsub(/ /,"",$2);print $2}' kanban.md | sort | uniq -d   # → nothing (a rebase that kept both sides duplicates rows; SD-l)
```

Run by the auditor on 2026-10-13 over the staging copy: `68`, `68`, `SAME_IDS`, no duplicate, the
dependency check below printed nothing, and a depth-first search over `Depends on` found no cycle.

Every `Depends on` ID must be a row ID (or the named external precondition on C1):

```bash
awk -F'|' '/^\| (C|E)[0-9]/{gsub(/ /,"",$2); id[$2]=1; d[$2]=$6} END{for(k in d){n=split(d[k],a,/, */); for(i=1;i<=n;i++){x=a[i]; gsub(/^ +| +$/,"",x); if(x!="—" && x !~ /PR #398/ && !(x in id)) print k" -> "x}}}' kanban.md
```

Pass = no output.
