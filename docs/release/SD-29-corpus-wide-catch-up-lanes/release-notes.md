# SD-29 Release Notes — Corpus-Wide Catch-Up Lanes

**Populated at closure, 2026-08-11 (Epic 11, Closure Epilogue).** The per-cycle receipts in
`progress.md` are the per-record evidence; this document is the rollup. Every figure below was
re-derived at closure by the command shown beside it, not transcribed from a receipt.

## Summary

| | |
|---|---|
| Bundle | SD-29 — Corpus-Wide Catch-Up Lanes |
| Branch | `tranche/9` |
| Build version | `0.9.<build>` (`0.9.${GITHUB_RUN_NUMBER}`, `.github/workflows/publish-tester-release.yml:97`); repo version files stamped `0.9.0` by Epic 9 (`ebc5c25a`) |
| Cards | 17 on `kanban.md`: **10 COMPLETE**, **3 PARTIAL** (lane half complete, ingest half `decision-blocked`), **1 DECISION-BLOCKED**, **2 not started** (companion lane), **1** this closure card |
| Gate at closure | `./scripts/verify.sh` full — exit code recorded in `progress.md`'s closure receipt |

SD-29 was the first bundle to treat the corpus as a whole rather than a book list. It re-cut itself
twice mid-flight — from per-book epics to **kind lanes** (`decisions.md §37`), then from seven books
to **all 37 in-scope books** (`decisions.md §38`) — and its most durable output is an honest,
corpus-wide picture of what is and is not ingested, together with the two structural ceilings that
stop the remainder.

## User-visible changes

Per-kind coverage at closure, re-derived from `docs/work-inventory.json`
(`generated_at 2026-08-11T10:38:33Z`) with:

```
python3 -c "import json,collections; d=json.load(open('docs/work-inventory.json')); \
a=collections.defaultdict(collections.Counter); \
[a[u['kind']].update([u['status']]) for u in d['units']]; \
[print(k, dict(a[k])) for k in sorted(a)]"
```

**38,540 units, 38 book directories** (37 in scope; `beginner_box`'s 19 units excluded per
`corpus-work-channels.md §10.2`). Status totals: `grounded` **491**, `text-complete` **2,402**,
`ingested-magnitude` **6,548**, `not-ingested` **14,582**, `not-started` **11,190**, `unknown`
**3,291**, `deferred-with-reason` **36**.

### Epic 4 — Proven-Path Lanes (equipment, equipment_modifier, spell, feat, race, class)

| Kind | Total | Grounded | Magnitude/text-complete |
|---|---|---|---|
| `equipment` | 6,227 | 133 | 4,817 ingested-magnitude + 299 text-complete |
| `equipment_modifier` | 1,580 | 40 | 471 ingested-magnitude + 841 text-complete |
| `spell` | 2,843 | — | 1,260 ingested-magnitude + 22 text-complete |
| `feat` | 2,610 | 77 | 1,240 text-complete |
| `race` | 103 | 7 | — |
| `class` | 185 | 27 | — |

### Epic 5 — Monster / Monster-Ability Chassis Lane

The merged kind chassis is **real and pilot-proven**: `RuleSetId::BonusBestiary` plus its
rules-table module, generator arm, wire DTO, `CORPUS_KIND_NAMES` entry, reach claims, diagnostic
row, and frontend path. Bonus Bestiary's **14** monster + **17** monster_ability units are
`grounded`. The chassis is once-per-*kind*: every remaining monster-bearing book inherits it.
Corpus-wide extend (1,270 monster / 3,107 monster_ability total) is `decision-blocked` per book.

### Epic 6 — Race-Trait Lane

Two classifier defects fixed and shipped; the **ingest** half is `decision-blocked` on a chassis
outside this bundle. `race_trait` stands at 3,447 units, 21 grounded.

### Epic 7 — Companion Lane

**Not started.** All 1,696 `companion` units remain `not-ingested`/`not-started`, 0 grounded. Stated
plainly rather than implied by a `READY` row — see §Known issues.

## Operational changes

- **Local-file dispatch.** The Hermes board is retired; `kanban.md` + `progress.md` in this package
  are the work queue and the receipt file (operator-pinned 2026-08-01).
- **Provenance gate (Epic 3).** PI-blacklist screening is wired into each lane's extraction step,
  with `docs/governance/license-matrix.md` cited for OGL/attribution across all 37 in-scope books.
  `verify.sh`'s `pi-sweep` stage enforces it on every run.
- **Function-based naming (Epic 1 + Epic 1b).** Both `SD-NN` and `GE-NN` tag families are banned
  from source identifiers, file names, and directory names.
  `scripts/identifier-discipline-audit.sh` now catches path tags and PascalCase/infix forms, not
  only prefixes, and returns `OK_NO_BUNDLE_TAGS` for `src/`, `apps/`, and `scripts/` (0 tagged file
  names in each). The `tests/` + `docs/` remainder is the audit's documented exclusion class.
- **Build version (Epic 9).** `0.8.x -> 0.9.x`, one advance per tranche cut.
- **Bundle code review (Epic 10)** reviewed the whole bundle diff against its branch point, fixed
  two unowned count pins (`b4cff429`), and closed the gate green.

## Defects fixed

1. **Race-trait classifier name-coincidence defect** (`corpus-work-channels.md §9.3`) — grounded
   race traits corrected 44 → **21**; the inflated figure came from name collisions, not content.
2. **Companion mis-classification.** `file_kind()` now types an `_abilities_race*` basename that
   also carries a `companion`/`familiar` marker as `Companion`. Moved **9** units out of
   `race_trait` (`inner_sea_intrigue`, all 9) and added **13** companion units
   (`inner_sea_intrigue` 9 + `bestiary_4` 4).
3. **Spell two-list divergence.** `build_spell_catalog` (5 books) and `v06_work_inventory`'s
   `spell_levels` (3 books) disagreed, reporting **192** already-ingested, already-on-screen ARG+UI
   spell units as `not-ingested`. Closed; the spell lane's remaining figure corrected 1,754 → 1,561.
4. **Equipment denominator.** 1,163 → **1,144** — the old figure counted `beginner_box`'s 19
   excluded units.
5. **Feat denominator.** 1,350 → **1,348** — a predicate difference (the kind's 2
   `deferred-with-reason` units), recorded rather than silently folded in.
6. **Two unowned count pins** left RED by a lane whose actor ran out of turn budget (`b4cff429`).

## Operational notes

- Tranche promotion PR: `tranche/9` → `develop`, opened by this closure card. **The operator merges
  it.** Cutting a new tranche branch is explicitly *not* part of this closure.
- Post-closure version state: repo files at `0.9.0`; publish stamps `0.9.<run_number>`.

## Verification evidence

- Per-cycle receipts in `progress.md` (one `## Cycle` heading per card).
- `./scripts/verify.sh` full at closure — exit code captured directly, never through a pipe;
  recorded in the closure receipt.
- `verify.sh` auto-emits a `verification` retro event per run; per-cycle retro shards live in
  `docs/retro/events/sd29-*.jsonl`.

## Known issues

1. **Companion kind entirely unstarted** — 1,696 units, 0 grounded. The Epic 7 pilot refused at
   `preflight-disk` (disk at 91%, below the gate's floor) and the card was deliberately left
   unclaimed. The disk condition has cleared (80% used, 97G free at closure); this is a ready
   re-dispatch for a successor bundle, not a corpus finding.
2. **Race chassis is the race-trait ceiling.** `crb::race_traits()` models exactly **7** races. Of
   3,447 `race_trait` units, **805** carry `race_trait_race_not_modelled` and **144**
   `race_trait_absent_from_race_traits`. No book's race traits can ground until a real race chassis
   lands — work outside SD-29's epic structure.
3. **Monster/monster-ability extend is `decision-blocked` per book.** The chassis is proven; the
   remaining books' ingest is not dispatched. A published-text dice-grounding decision is also
   outstanding: 13 of the pilot book's 14 named natural attacks carry no dice in the corpus.
4. **`class_feature` Tier-3 deferral** — 15,472 units corpus-wide, deferred by `decisions.md §38.4`
   / `successor-forward-scope-register.md C1.3`, owned by SD-30's class_feature/archetype bundle.
5. **DM Toolkit extension (Epic 8) is `decision-blocked` to C3.1** — it does not land inside SD-29.
   The criterion ("a lane cycle needed the consumer surface to satisfy its reach claim") was unmet:
   both Epic 5 pilot reach claims assess the already-shipped `list_monster_catalog`.
6. **`beastiary1/race_traits` `OPEN_FINDINGS` entry still stands.** DoD item 6 expected Epic 5's
   Monster Codex cycle-batch to retire it; Monster Codex was never ingested (Epic 5 extend is
   `decision-blocked`), so the entry is correct as written and stays. Its designed closure mechanism
   — `tests/sd27_duergar_invisibility_sla_is_upstream_blocked.rs` going red the day
   `monster_codex/mc_abilities_race.lst` lands — is intact.
7. **Seven `<book>/archetypes` `OPEN_FINDINGS` entries** stay standing per DoD item 6; they belong
   to SD-30.
8. **Baseline headroom, not movement.** `BASELINE_ROOT_LIB_TESTS` is 1604; the tree measures
   **1615**. These are floors, so the gate passes and no baseline commit was required. Flagged so a
   successor re-pins deliberately (DoD item 7) rather than discovering the drift.

## Update eligibility

No operator-on-file override indicators were set during this bundle. Standard tester-release
eligibility applies at `0.9.<build>`.
