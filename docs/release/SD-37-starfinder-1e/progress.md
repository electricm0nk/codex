---
canonical: true
bundle_id: SD-37
---

# SD-37 Progress

This is the live cycle log, and the surface the operator reviews on return. Every dispatched card
appends one row to the cycle log, **in its own closing commit**. Re-fetch this file before editing
it (`workflow-instruction.md §5`).

## Run handle

| Field | Value |
|---|---|
| Workflow run id | *(resolves at launch — the orchestrator writes it here the moment the run starts)* |
| Script path | *(resolves at launch)* |
| Prefix backup | `artifacts/cycle_0/workflow-prefix.backup.js` *(written at launch)* |
| Pinned `tranche/17` SHA for wrong-base resets | *(resolves at C1 — the bump commit)* |
| Oracle SHA | `7f818006e371188e5717fd18d74d18a420747fc6` |

## Summary

| Epic | Status | Cards complete | Notes |
|---|---|---|---|
| C | in progress | 1 of 4 (C0.0) | |
| E0 | waiting | 0 of 4 | |
| E1 | waiting | 0 of 5 | |
| E2 | waiting | 0 of 3 | |
| E3 | waiting | 0 of 6 | |
| E4a | waiting | 0 of 5 | |
| E4 | waiting | 0 of 7 | |
| E5 | waiting | 0 of 5 | |
| E6 | waiting | 0 of 7 | |
| E7 | waiting | 0 of 9 | |
| **Total** | | **1 of 55** | `awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/ && $5 ~ /complete/' kanban.md \| awk 'END{print NR}'` |

---

## Cycle 0 — package authored; unattended mode acknowledged (2026-10-02)

**Operator on record.** The operator is away while this bundle runs. Directive of 2026-08-01
(verbatim): *"include instructions to all 3 that indicate they will be running in unnattended mode
since i will be out of town while this runs. They may not stop to ask questions - it might be days
before i notice."* This receipt is the third of the three mirrors. The other two are
`workflow-instruction.md`'s OPERATING METHOD callout and `decisions.md §12`.

- **Card:** C0.0. **Author:** orchestrating session's planning sub-agent, Sonnet 5.5 (see
  `decisions.md §11` planning disclosure; card C0.2 is the Opus review).
- **Tree:** `/home/ubuntu/workspace/worktrees/codex-sd37`, branch `tranche/17` @ `20bf84a3b2`
  (= `origin/develop`). `git status --porcelain` was empty before authoring.
- **Write scope honoured:** only `docs/release/SD-37-starfinder-1e/**`. No code, no version bump,
  no push. The main checkout `/home/ubuntu/workspace/repos/codex` (another session's tree) was read
  from only through git plumbing (`git worktree list`, `git branch -r`), never written.
- **Inputs read:** the three research reports (source inventory, engine-reuse map, predecessor
  lessons); `docs/release/template/template.md`; `docs/governance/workflow-instruction-template.md`;
  `.claude/skills/stc-authoring/SKILL.md`; the unattended-mode doctrine; `AGENTS.md`; the memory
  index and the notes named in the brief; and SD-35/SD-36 packages as shape siblings.
- **Figures re-derived:** CUI F-1 … F-20, each with two implementations
  (`content-unit-inventory.md §1`).
- **Status:** complete.

### Likely decision points and their safe defaults

These are the points that historically trigger questions, each with the default to take. Full
table: `decisions.md §12.1`.

| Decision point | Safe default |
|---|---|
| Book licence unclear (E0.2) | Exclude and log (SD-a) |
| SRD unreachable for hand values (E0.4) | `blocked-escalated` on E0.4 and its fixture dependants only; continue E1/E2/E3.1–E3.2/E4a |
| Unmapped overloaded PCGen field (E3.3) | Named refusal (SD-d) |
| PF render byte-diff (E1.4, E4a.4) | Fix it; never re-baseline (SD-i) |
| Seed build impossible in the oracle (E0.4) | Swap to another CRB option of the same class and log it (SD-g) |
| Quota at or above the threshold | Stop dispatching, write a resume receipt (SD-l) |
| A fenced file is needed by the wrong lane | `declined` with `ownedBy` (R4) |
| Starship content met in records | Print its prose; no starship sheet (SD-f) |

### Pending retro events (to be emitted by C0.1; outside this package's write scope)

```bash
scripts/retro.py correction --subject 'SD-36 FS-3 / scope-draft / memory rules-tables-stay-rust-until-starfinder' \
  --claimed '181,797 lines in src/rules_core/rules_tables' --actual '180,883 lines (250 .rs files)' \
  --verified-by "find src/rules_core/rules_tables -name '*.rs' -print0 | xargs -0 cat | wc -l; Python os.walk agrees"
scripts/retro.py correction --subject 'SD-35 forward-scope-register C2.1 owner column' \
  --claimed 'owner SD-36' --actual 'SD-36 has no C2.1 row; adopted by SD-37 as E3' \
  --verified-by "grep -n 'C2.1' docs/release/SD-36-consolidation/forward-scope-register.md (no output)"
scripts/retro.py correction --subject 'SD-37 source-inventory research report' \
  --claimed 'system/gameModes/Starfinder has 12 .lst files' --actual '10 .lst + base.xml.ftl + bio/' \
  --verified-by "find \$PCGEN_REPO_DIR/system/gameModes/Starfinder -maxdepth 1 -name '*.lst' | wc -l"
```

The source inventory's own "Caveats" section notes that its counts were not cross-checked. The
corrected figures are in CUI §1, "Stale figures found during authoring".

---

## Cycle log

| Card | Date | Model | Commit | Status | Seed deltas (Aldric / Elowen / Soldier / Mystic / Technomancer / Envoy) | Receipt |
|---|---|---|---|---|---|---|
| C0.0 | 2026-10-02 | Sonnet 5.5 (planning) | this package's commit | complete | n/a (no code) | this section |

## Decisions taken on safe defaults

| Date | Card | Default (SD-x) | Alternative not taken | Why |
|---|---|---|---|---|
| 2026-10-02 | C0.0 | `decisions.md §1`–`§19` (all operator-away defaults) | listed per decision under "Revisit if the operator disagrees" | operator away at authoring |

## Open blockers

None. Each entry here is a request for an operator ruling. It pauses only the named card and its
dependents, and it is **never** a closure path.

## DISCOVERED

None yet. A discovery becomes a new `kanban.md` card in the discovering cycle's commit.
