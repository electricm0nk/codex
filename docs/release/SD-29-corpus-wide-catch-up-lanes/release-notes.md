# SD-29 Release Notes — Corpus-Wide Catch-Up Lanes

> **Rename note (2026-08-14).** `Owner: SD-31` below refers to `SD-31-pcgen-character-import`, the
> package now renamed `SD-33-pcgen-character-import` (operator ruling 2026-08-14). Updated in place;
> not reassigned to the new SD-31/SD-32 split out of SD-30.

**Populated at the bundle's REAL closure, 2026-08-13 (Epic 11, Closure Epilogue, run 3).** The
per-cycle receipts in `progress.md` are the per-record evidence; this document is the rollup. Every
figure below was re-derived at closure by the command shown beside it, not transcribed from a
receipt.

> ### Why this document was written three times
>
> SD-29 was closed once on 2026-08-11 with **three lanes unfinished**, and this file disposed of that
> unfinished work by writing it up as shipped "Known issues" of a released bundle. The operator
> rescinded that closure the same day (`decisions.md §42`, verbatim: *"this is part of sd-29's scope.
> sd-29 isn't done. let's get after it."*). Closure **run 2** (2026-08-13) then re-derived every lane
> denominator and correctly **refused to close** with 63 workable units outstanding, leaving this
> file deliberately un-rewritten. Closure **run 3** — this one — re-derived the same figures after
> the two remaining lanes ran to their ceilings, found **0 workable units**, and closed.
>
> **The reopen is the largest coverage event in the bundle.** `grounded` moved **491 → 4,699**, and
> **all 4,208 units of that gain** are the three lanes the rescinded closure would have shipped as
> "known issues". The lesson is recorded rather than implied: *a remainder is not a ceiling, and a
> ceiling is not a remainder* — neither is legible without a row classifier.

## Summary

| | |
|---|---|
| Bundle | SD-29 — Corpus-Wide Catch-Up Lanes |
| Branch | `tranche/9` (branch point `a1295856`) |
| Bundle diff | **272 commits, 4,741 files, +316,061 −15,969** (`git diff --shortstat $(git merge-base HEAD origin/develop)..HEAD`) |
| Build version | `0.9.<build>` (`0.9.${GITHUB_RUN_NUMBER}`, `.github/workflows/publish-tester-release.yml:97`); repo version files stamped `0.9.0` by Epic 9 (`ebc5c25a`) |
| Cards | 19 on `kanban.md` — **15 COMPLETE**, **3 DRY** (cards 8, 10, 12: lanes run to their real ceilings), **1 DECISION-BLOCKED** (`epic-8-toolkit`, a genuine ruling routed to retrofit C3.1) |
| Gate at closure | `./scripts/verify.sh` FULL — exit code captured directly, never through a pipe; recorded in `progress.md`'s closure receipt (`## Cycle SD29-E11-F1-003`) |
| On-screen evidence | DoD item 8 satisfied at the closing tip for all three reopened lanes — `monster`, `companion`, `race_trait` — machine-verdicted by `verify-on-screen.sh` (`artifacts/SD29-E11-F1-003/item8/`) |

SD-29 was the first bundle to treat the corpus as a whole rather than a book list. It re-cut itself
twice mid-flight — from per-book epics to **kind lanes** (`decisions.md §37`), then from seven books
to **all 37 in-scope books** (`decisions.md §38`) — was closed prematurely, reopened by operator
directive, and then ran three kind lanes to measured ceilings. Its two most durable outputs are the
**coverage** and the **instruments**: five checked-in row classifiers that make "how much of this
remainder is actually workable?" a command rather than an argument.

## User-visible changes

Per-kind coverage at closure, re-derived from `docs/work-inventory.json`
(`generated_at 2026-08-13T09:33:16Z`) with:

```
python3 -c "import json,collections; d=json.load(open('docs/work-inventory.json')); \
a=collections.defaultdict(collections.Counter); \
[a[u['kind']].update([u['status']]) for u in d['units']]; \
[print(k, dict(a[k])) for k in sorted(a)]"
```

**38,540 units across 38 book directories** (37 in scope, 38,521 in-scope units; `beginner_box`'s 19
units excluded per `corpus-work-channels.md §10.2`). Status totals: `grounded` **4,699**,
`ingested-magnitude` **6,545**, `text-complete` **2,391**, `not-ingested` **17,209**, `not-started`
**4,113**, `unknown` **3,547**, `deferred-with-reason` **36**.

| Kind | Total | Grounded at open | **Grounded at close** | Δ |
|---|---|---|---|---|
| `monster_ability` | 3,107 | 17 | **1,629** | **+1,612** |
| `monster` | 1,270 | 60 | **1,242** | **+1,182** |
| `companion` | 1,696 | 0 | **922** | **+922** |
| `race_trait` | 3,447 | 21 | **513** | **+492** |
| `equipment` | 6,227 | 133 | 133 | — (4,814 `ingested-magnitude`) |
| `class_feature` | 15,472 | 109 | 109 | — (Tier-3 deferral, SD-30) |
| `feat` | 2,610 | 77 | 77 | — (1,229 `text-complete`) |
| `equipment_modifier` | 1,580 | 40 | 40 | — (841 `text-complete`) |
| `spell` | 2,843 | 0 | 0 | — (1,260 `ingested-magnitude`) |
| `class` | 185 | 27 | 27 | — |
| `race` | 103 | 7 | 7 | — |

### Epic 4 — Proven-Path Lanes (equipment, equipment_modifier, spell, feat, race, class)

Ingest at magnitude/text depth across the corpus; three denominator defects closed (see §Defects
fixed 3-5). These kinds are *breadth-complete at their proven depth*, not `grounded` — grounding
them needs per-record magnitude wiring that is a different class of work from ingest.

### Epic 5 — Monster / Monster-Ability Chassis Lane — **DRY, at its measured ceiling**

The merged `monster`/`monster_ability` kind chassis was piloted on Bonus Bestiary and then extended
over **eleven rounds** across every monster-bearing book with a chassis. `monster` closes at
**1,242 / 1,270 grounded (97.8%)**; `monster_ability` at **1,629 / 3,107 (52.4%)**.

**The 1,506-unit raw remainder contains 0 workable rows**, and the split is a command, not a claim:

```
python3 scripts/classify_monster_ability_rows.py
  -> remaining monster+monster_ability units : 1506
     orphan monster_ability rows             : 1406  (703 in ten books with NO monster row at all)
     Product Identity rows                   :   32
     `.COPY=` delta rows                      :    2
     reachable remainder                     :   66

python3 scripts/screen_pcc_load_gates.py monster monster_ability
  -> TOTAL remaining units excluded by a PCC load gate: 10
```

The 66 the classifier still calls reachable are, on inspection, all non-workable: 54 cross-table
owners + 4 `.MOD`-only overlays in `bestiary` (`decisions.md §60.2`; the `.MOD` class is confirmed
independently by the work inventory's own `origin` field — `mod_only 4` corpus-wide), 7 Product
Identity residue in `inner_sea_bestiary` (`§57.2`/`§58.1`), and 1 row in `occult_adventures` behind
a **negated** `PRECAMPAIGN` gate that PCGen would not load (`§68.1`).

### Epic 6 — Race-Trait Lane — **DRY, at its measured ceiling**

`race_trait` closes at **513 grounded**, up from 21. The lane's ceiling is the race chassis, and it
is measured rather than asserted:

```
python3 scripts/race_trait_ceiling.py
  -> CEILING 571 (553 `TYPE:<Race> Racial Trait` rows + 18 `TYPE:<Race> Subrace` heritage selectors)
     over the 18 races the ingest chassis models
     by status {'grounded': 513, 'not-ingested': 58}
     chassis-blocked residue: 3447 - 571 = 2876
```

**2,876 of the 3,447 units can never ground from a race-trait ingest** — their race has no chassis,
so `RaceCorpus::resolve` returns `None` whatever the ingest writes. The 58 inside the ceiling that
did not ground each carry a recorded finding (see §Known issues 2).

### Epic 7 — Companion Lane — **DRY, built from nothing in this bundle**

At the rescinded closure this lane read "never started, 0 grounded". It closes at **922 / 1,696
grounded** across seventeen books, with a real chassis
(`src/rules_core/rules_tables/companion_chassis.rs`), a served catalog
(`apps/desktop/src-tauri/src/companion_catalog.rs` + `CompanionCatalogScreen.tsx`), and per-book
companion data. `scripts/classify_companion_rows.py`, intersected with unit status, leaves exactly
**1** reachable-and-remaining row corpus-wide (`core_essentials` / `Pseudodragon ~ Tail`), and that
row needs an `ASPECT:` chassis no table in this program models (§Known issues 3).

### Rule sets registered

`RuleSetId` grew **7 → 30** populated variants across SD-29
(`sed -n '/pub enum RuleSetId/,/^}/p' src/rules_core/rules_tables/mod.rs`). Sixteen of the 30 were
registered by the three reopened lanes. **Registering a rule set is not compiling a whole book**:
five of the new arms compile a `companion` family and nothing else, two compile a disk-served
`race_trait` family and nothing else, and two (`B5`, `B6`) carry **zero** monsters despite the name.

## Operational changes

- **Local-file dispatch.** The Hermes board is retired; `kanban.md` + `progress.md` in this package
  are the work queue and the receipt file (operator-pinned 2026-08-01).
- **Provenance gate (Epic 3).** PI-blacklist screening is wired into each lane's extraction step,
  with `docs/governance/license-matrix.md` cited for OGL/attribution across all 37 in-scope books.
  `verify.sh`'s `pi-sweep` stage enforces it on every run.
- **Five row classifiers are now checked in**, and they are the bundle's most reusable output:
  `scripts/classify_monster_ability_rows.py`, `scripts/classify_companion_rows.py`,
  `scripts/classify_race_trait_rows.py`, `scripts/race_trait_ceiling.py`,
  `scripts/screen_pcc_load_gates.py`, plus `scripts/scan_monster_ability_bundle_rows.py`. Each one
  exists because a throwaway derivation is not a citation (`decisions.md §45.1`).
- **On-screen verification is a gated harness, not a screenshot.**
  `apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh` drives the real app, filters to a
  record, and reads the X clipboard back to machine-verdict the expected strings; its driver's
  self-test is a `verify.sh` stage (Epic 13).
- **Function-based naming (Epic 1 + Epic 1b).** Both `SD-NN` and `GE-NN` tag families are banned
  from source identifiers, file names, and directory names.
  `scripts/identifier-discipline-audit.sh` catches path tags and PascalCase/infix forms, not only
  prefixes, and returns `OK_NO_BUNDLE_TAGS` for `src/`, `apps/`, and `scripts/`.
- **Build version (Epic 9).** `0.8.x → 0.9.x`, one advance per tranche cut.
- **Bundle code review (Epic 10)** ran twice — run 1 against the rescinded state, run 2 against the
  reopened bundle — fixed two unowned count pins and two review findings in-bundle, and routed four
  deferrals to owners.

## Defects fixed

1. **Race-trait classifier name-coincidence defect** (`corpus-work-channels.md §9.3`) — grounded
   race traits corrected 44 → 21; the inflated figure came from name collisions, not content.
2. **Companion mis-classification.** `file_kind()` now types an `_abilities_race*` basename that
   also carries a `companion`/`familiar` marker as `Companion`. Moved **9** units out of
   `race_trait` and added **13** companion units.
3. **Spell two-list divergence.** `build_spell_catalog` (5 books) and `v06_work_inventory`'s
   `spell_levels` (3 books) disagreed, reporting **192** already-ingested, already-on-screen ARG+UI
   spell units as `not-ingested`. Closed; the spell lane's remaining figure corrected 1,754 → 1,561.
4. **Equipment denominator.** 1,163 → **1,144** — the old figure counted `beginner_box`'s 19
   excluded units.
5. **Feat denominator.** 1,350 → **1,348** — a predicate difference (the kind's 2
   `deferred-with-reason` units), recorded rather than silently folded in.
6. **`gen_monster_book` would have deleted 46 shipped, grounded, player-visible SD-22 records** on
   every run, silently — caught by reading the generator before running it (`decisions.md §60.3`).
   Its sweep now removes only files whose `data.key` is namespaced to the book and kind it sits
   under, and its screening note is append-only rather than overwriting a four-pass history.
7. **A player-visible rendering defect eleven books could not produce** — found and fixed by the
   monster lane's round 10 (`progress.md §4c`).
8. **The `root-full` test floor had been counting 7 tests twice**, exposed by retiring a `#[path]`
   duplicate (`231ee0d5`).
9. **Two unowned count pins** left RED by a lane whose actor ran out of turn budget (`b4cff429`).
10. **Desktop driver fix (Epic 13)** — DoD item 8 had become unsatisfiable; `driver.sh` was repaired
    and its self-test added to the gate so the harness cannot silently rot again.

## Known issues

> These are **shipped** issues and **structurally-unreachable residue**, each with an owner. Nothing
> in this section is undispatched SD-29 work — that distinction is what the 2026-08-11 closure got
> wrong, and this section is written to the corrected standard: *a known issue is something the
> bundle decided about; an outstanding lane is something the bundle never dispatched.*

1. **`scripts/wired-integration-audit.sh` is RED at bundle scope, and this closure does not report
   it clean.** `./scripts/wired-integration-audit.sh` exits **1** on **13** `placeholder` hits
   (exit code captured directly). All 13 are hand-classified as not-stubs — 2 JSX `placeholder=`
   attributes, 10 doc comments *about* upstream corpus placeholders, 1 `#[cfg(test)]` assertion
   message — and the Rust repo-wide sweep `tests/sd24_wired_integration_audit.rs` encodes three
   reviewed exclusion filters for exactly these and is green in the gate. **The two instruments
   disagree, and the remedy is parity, not leniency:** port the Rust gate's filters into the shell
   script with a self-test. **Owner: SD-33** (`successor-forward-scope-register.md` C1.4b). Closure
   run 1 reported this clean; that was wrong and is corrected here.

2. **Race chassis is the `race_trait` ceiling — 2,876 units are structurally unreachable.** The
   ingest chassis models **18** races (`src/bin/ingest_race_traits.rs`'s
   `IN_SCOPE_RACES: [&str; 18]`); the compute surface `crb::race_tables::race_traits()` models 7.
   Of the 571 rows inside the ingest ceiling, 513 ground and 58 do not: 49 APG `KEY:`
   republications (republished, not new — `decisions.md §39`), 3 Drow Noble rows needing a
   **race-variant** chassis, 2 `PREABILITY`-gated subrace selectors in `core_essentials`, 1
   `horror_adventures` row, 2 `inner_sea_races` rows (one declared Product Identity, one an upstream
   corpus gap), and 1 mechanism-blocked `monster_codex` row. **Owner: unassigned — a race-variant
   chassis is outside any SD-29 or SD-30 epic** and needs an operator scope decision before it has
   one. It is stated here as a real ceiling rather than parked in a register under a false owner.

3. **`ASPECT:` is modelled by no chassis in this program.** 34 `grounded` companion rows are
   *diminished* by it and 1 is emptied by it (`core_essentials` / `Pseudodragon ~ Tail`, the
   companion lane's single reachable-and-remaining row). **Owner: SD-33**
   (`successor-forward-scope-register.md` C1.6).

4. **229 monster rows are mechanism-blocked on the `ABILITY:Internal|AUTOMATIC|` bundle hop**,
   across six books. Scanned, counted and checked in
   (`python3 scripts/scan_monster_ability_bundle_rows.py`) with the exact call sites a successor
   must change. This is a **ceiling correction**, not a backlog line: following the hop widens an
   ownership pass and changes what every registered book ships. **Owner: SD-33**
   (`successor-forward-scope-register.md` C1.5).

5. **Frontend preview fixtures are hand-authored rules data with nothing pinning them to the
   corpus.** `companionCatalogRuntime.ts` and `monsterCatalogRuntime.ts` both build a browser-preview
   catalog by hand and both declare full transcription fidelity; the companion one is not faithful
   (`Familiar (Clockwork Spy)` serves 1 of 6 stat adjustments and 1 of 3 abilities). **Not
   shipped-path** — the branch sits behind `if (!hasTauriRuntime())` and is never taken in the
   desktop product. The fix is a fixture pipeline the frontend does not have. **Owner: SD-33**
   (`successor-forward-scope-register.md` C1.4a).

6. **`equipment` and `spell` have never been verified on screen by the harness.** Epic 4 predates
   `verify-on-screen.sh`, so those two families' `SEARCH_Y` constants are by-analogy and have never
   been exercised. **The first equipment or spell cycle after this bundle must calibrate them before
   citing a PASS.** A calibration debt, not a defect in anything shipped. **Owner: SD-33**
   (`successor-forward-scope-register.md` C1.4d).

7. **Decision 41 does not say whether NEW `tests/` files may carry a bundle tag.** The audit's
   self-test encodes the `tests/` exemption unconditionally, and Epic 6 then added a bundle-tagged
   test file the gate permitted but the convention's stated intent forbids. This needs a **ruling**,
   not an edit to a tested gate. Severity low. **Owner: SD-33**
   (`successor-forward-scope-register.md` C1.4c).

8. **`class_feature` Tier-3 deferral** — 15,472 units corpus-wide, 109 grounded, deferred by
   `decisions.md §38.4`. **Owner: SD-30**, the class_feature/archetype bundle
   (`successor-forward-scope-register.md` C1.3).

9. **DM Toolkit extension (Epic 8) is `decision-blocked` to retrofit C3.1** — it does not land
   inside SD-29. The criterion ("a lane cycle needed the consumer surface to satisfy its reach
   claim") was unmet: both Epic 5 pilot reach claims assess the already-shipped
   `list_monster_catalog`. **Owner: operator-on-request** (`successor-forward-scope-register.md`
   C3.1).

10. **Retroactive magnitude-fidelity sweep over already-landed `static` units** is open and routed
    (`successor-forward-scope-register.md` C3.3).

11. **`OPEN_FINDINGS` entries that remain standing by design.** The seven `<book>/archetypes`
    entries belong to SD-30. The `beastiary1/race_traits` entry that DoD item 6 expected Monster
    Codex to retire **was** retired by Epic 6's pilot; entries added since (e.g. `inner_sea_world_guide`'s
    5 template-namespaced ability rows) are deliberate non-ingests of records nothing could reach,
    each with its remedy named in its own entry.

12. **Baseline headroom, not movement.** Test floors are floors, so the gate passes; where a lane's
    own measurement exceeded a pinned floor it was ratcheted from that lane's gate run. Where slack
    remains (the clippy ceiling is recorded at 54 against a lower measurement) it was deliberately
    not ratcheted by a lane that did not own the number. Flagged so a successor re-pins deliberately
    (DoD item 7) rather than discovering the drift.

## Operational notes

- Tranche promotion PR: `tranche/9` → `develop` (**PR #360**), opened by the rescinded closure and
  **kept open and updated in place, never replaced**, through both later closure runs. Closure run 3
  updates it to the true final state. **The operator merges it.** Cutting a new tranche branch is
  explicitly *not* part of closure.
- Post-closure version state: repo files at `0.9.0`; publish stamps `0.9.<run_number>`.
- Nothing is stranded. `git branch -a --no-merged tranche/9` returns exactly two refs and both are
  accounted for in the closure receipt: `worktree-wf_9029acd8-6b0-6` (the companion pilot's
  crash-orphaned branch — **verified merged BY CONTENT**, every file of its one commit is present at
  `tranche/9` and its 10 `inner_sea_combat` companion units are `grounded`) and
  `origin/update-index` (the release channel's long-lived manifest branch, unrelated to any bundle).

## Verification evidence

- Per-cycle receipts in `progress.md` (one `## Cycle` heading per card; 19 cards, and the lanes'
  rounds are numbered within them).
- `./scripts/verify.sh` FULL at closure — exit code captured directly, never through a pipe;
  recorded in the closure receipt `## Cycle SD29-E11-F1-003`.
- DoD item 8 on-screen artifacts at the closing tip for all three reopened lanes:
  `artifacts/SD29-E11-F1-003/item8/` — `closure-monster-hive-queen`, `closure-companion-um-giant-leech`,
  `closure-race-trait-elf-alternates`. Three earlier attempts in that same directory are named
  `*.FAILED.verify.md` and are **not** evidence of anything passing; they are kept because the
  harness names failures so they can never be cited as passes.
- `verify.sh` auto-emits a `verification` retro event per run; per-cycle retro shards live in
  `docs/retro/events/sd29-*.jsonl`.

## Update eligibility

No operator-on-file override indicators were set during this bundle. Standard tester-release
eligibility applies at `0.9.<build>`.
