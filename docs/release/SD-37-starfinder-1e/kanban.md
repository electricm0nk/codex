---
canonical: true
bundle_id: SD-37
board: local-file ./kanban.md (Hermes board retired 2026-08-01)
---

# SD-37 Kanban

There is one row per card. The dispatched agent that closes a card edits that card's row in its
own closing commit (`workflow-instruction.md §6` step 8). The orchestrator never edits a row to
`complete` from memory: a row moves only together with a receipt and its commit SHA.

**Status vocabulary:**
- `ready`: dependencies met, dispatchable.
- `waiting`: dependencies unmet.
- `in-progress`.
- `complete`.
- `partial`: a criterion was met only in part. This status blocks closure.
- `blocked-escalated`: only an operator ruling can clear it. It blocks closure, and the ruling
  needed is under `progress.md ## Open blockers`.

**Column order is load-bearing:** E7.3's scan reads ID as awk field `$2` and Status as `$5`
(`epic-breakdown.md` E7.3 fenced command). Row order is the dispatch order since C0.2 (E4a after E7.1).

**Closure rule:** no PR while any row is anything other than `complete`. E8 (starship) is **not** a
card (`decisions.md §17`).

| ID | Title | Tier | Status | Depends on | Receipt / notes |
|---|---|---|---|---|---|
| C0.0 | Package authored + unattended-mode receipt | (authoring session, Sonnet 5.5) | complete | — | `progress.md` Cycle 0; commit = this package's commit |
| C0.1 | SD-36 loose ends (README status, kanban D2–D6 row, worktrees, 7 `sd36/*` remotes, dirty retro log) + pending retro corrections | haiku | ready | — | Not a gate. SD-j applies. |
| C0.2 | Opus review of this package (planning was Sonnet) | opus | complete | — | `artifacts/cycle_0/C0.2_cycle_receipt.md`; package fixes in the C0.2 commit |
| C1 | Version bump 0.17.0 (14 surfaces, one commit) + push `tranche/17` | haiku | complete | C0.2 | `artifacts/cycle_0/C1_cycle_receipt.md`; commit 9d03a76996 |
| E0.1 | Oracle sparse paths + SF completeness probe + fresh-clone proof | sonnet | waiting | C1 | `artifacts/epic_0/` |
| E0.2 | Licence matrix SF rows + SF PI term set | opus | waiting | C1 | No registry test here (moved to E3.1); uses cargo (`pi_screening.rs`) |
| E0.3 | SF work inventory (denominator), fail-closed sum | sonnet | waiting | E0.1, E0.2 | |
| E0.4 | Seed builds + hand values from SRD, Opus-reviewed | opus | waiting | C1 | Network needed (SD-c). Seed builds first (`seed-builds.md`), then hand values; independent Opus reviewer |
| E1.1 | `GameSystem`-keyed package roots, runtime resolution | opus | waiting | C1 | Batched with E1.2, E1.3 |
| E1.2 | Per-system book registries (19 consts) | opus | waiting | C1 | Batched |
| E1.3 | Converter system parameter | opus | waiting | C1 | Batched |
| E1.4 | PF byte-identical gate (Aldric, Elowen) + structural diff | opus | waiting | E1.1–E1.3 | |
| E1.MC | E1 adversarial merge check | opus | waiting | E1.4 | |
| E2.1 | Additive schema variants | opus | waiting | E1.MC | |
| E2.2 | Published `schemas/rules/*.schema.json` + `rules-schema-check` stage | sonnet | waiting | E2.1 | |
| E2.MC | E2 adversarial merge check | opus | waiting | E2.2 | |
| E3.1 | SF `.pcc` include structure + game mode (C2.1) | opus | waiting | E2.MC, E0.1, E0.2 | Also owns the SF-registry licence test (moved from E0.2) |
| E3.2 | Formula-system reader (`MODIFY*`/`CHANNEL`/`DATATABLE`) | opus | waiting | E3.1 | |
| E3.3 | SF mapping table, oracle rows, planted mutations | opus | waiting | E3.2, E0.4 | Overloaded-field hazard; mutations M1–M4 (`decisions.md §8`) |
| E3.4 | Core Rulebook proof generation | opus | waiting | E3.3, E0.2, E0.3 | |
| E3.5 | Go wide: 7 books, one batch | opus | waiting | E3.4 | |
| E3.MC | E3 adversarial merge check | opus | waiting | E3.5 | |
| E4.1 | Generic SF chassis: BAB, saves, HP, Stamina, Resolve, key ability | opus | waiting | E3.MC | |
| E4.2 | EAC/KAC, initiative, skills, ACP | opus | waiting | E4.1 | |
| E4.3 | Themes, point buy, ability increases | opus | waiting | E4.1 | |
| E4.4 | Spellcasting 0–6 | opus | waiting | E4.1 | |
| E4.5 | Credits, bulk, encumbrance | opus | waiting | E4.1 | |
| E4.6 | `StarfinderAdapter`; retire stub 0002 for SF | opus | waiting | E4.1–E4.5 | |
| E4.MC | E4 adversarial merge check | opus | waiting | E4.6 | |
| E5.1 | Races, themes, class features print | opus | waiting | E4.MC | C0.2: was ∥ E4 (shared converter + `data/starfinder-1e/**`) |
| E5.2 | Feats, spells print | opus | waiting | E5.1 | |
| E5.3 | Equipment, augmentations, upgrades, fusions print | opus | waiting | E5.1 | |
| E5.4 | Drone print | opus | waiting | E5.1 | |
| E5.MC | E5 adversarial merge check | opus | waiting | E5.2–E5.4 | |
| E6.1 | System picker → real adapter; SF data bundled | opus | waiting | E4.MC, E5.MC | |
| E6.2 | SF creation flow | opus | waiting | E6.1 | |
| E6.3 | SF sheet layout, engine-single-source | opus | waiting | E6.1 | |
| E6.4 | SF catalogs from `data/starfinder-1e/sheet_rules` | opus | waiting | E6.1 |  |
| E6.5 | SF level-up | opus | waiting | E6.2 | |
| E6.6 | Six seeds open in the real app, isolated `XDG_DATA_HOME` | sonnet | waiting | E6.3–E6.5 | Creates the 4 SF seeds via the real creation flow |
| E6.MC | E6 adversarial merge check | opus | waiting | E6.6 | |
| E7.1 | SF oracle parity roster + "not covered" list | opus | waiting | E6.MC | |
| E4a.1 | Data-package format, loader, schema, bundle path, licence/PI | opus | waiting | E7.1 | Serial after E7.1; no other code lane in flight (C0.2 re-sequencing) |
| E4a.2 | Re-point all 252 importers | sonnet | waiting | E4a.1 | One dispatch; may edit any importer |
| E4a.3 | `.lst` citation burn-down (re-derive SD-36 D6 first) | sonnet | waiting | E4a.2 | Target 0 (`decisions.md §19`) |
| E4a.4 | PF parity + Rust table removal | opus | waiting | E4a.3 | |
| E4a.MC | E4a adversarial merge check | opus | waiting | E4a.4 | |
| E7.2 | Widest-scope verify, baselines (CI = E7.9's `pr-tests`, SD-n) | sonnet | waiting | E4a.MC | |
| E7.3 | Final-acceptance scan (stop if short) | opus | waiting | E7.2 | |
| E7.4 | Retrospective written + cited | sonnet | waiting | E7.3 | |
| E7.5 | Worktree/branch sweep | haiku | waiting | E7.4 | |
| E7.6 | Release notes | sonnet | waiting | E7.5 | |
| E7.7 | Architecture truth-up + claims critic | sonnet + opus | waiting | E7.6 | |
| E7.8 | Graphify LAST | sonnet | waiting | E7.7 | |
| E7.9 | PR `tranche/17 → develop` (final action) | sonnet | waiting | E7.8 | Operator merges. Waits for `pr-tests` (SD-n); C0.2: was haiku |

**Row check (55 cards at authoring; C0.2 re-ran both on 2026-10-02 → 55, 55, and the diff below → `SAME_IDS`).** `awk -F'|' '$2 ~ /^ (C|E)[0-9]/{n++} END{print n}' kanban.md`
printed 55, and Python `sum(1 for l in open('kanban.md') if re.match(r'^\| (C|E)[0-9]', l))` printed 55.
The card IDs here must be exactly the criterion IDs in `epic-breakdown.md` (§0's map has 53 rows
because it folds E1.1–E1.3 into one):

```bash
diff <(awk -F'|' '/^\| (C|E)[0-9]/{gsub(/ /,"",$2);print $2}' kanban.md | sort) \
     <(awk -F'|' '/^## Epic C/{s=1} s && /^\| (C|E)[0-9]/{gsub(/ /,"",$2);print $2}' epic-breakdown.md | sort) \
  && echo SAME_IDS
```
