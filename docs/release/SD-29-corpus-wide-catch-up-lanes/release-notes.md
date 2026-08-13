# SD-29 Release Notes — Corpus-Wide Catch-Up Lanes

> ## 🔴 THESE NOTES ARE NOT FINAL — THE BUNDLE WAS REOPENED, 2026-08-11
>
> **Operator directive, verbatim:** "this is part of sd-29's scope. sd-29 isn't done. let's get
> after it." Recorded as `decisions.md` **Decision 42**.
>
> The closure these notes were written for is **rescinded**. SD-29 was closed with **three lanes
> unfinished**, and this document disposed of that unfinished work by writing it up as shipped
> "Known issues" of a released bundle. It was not shipped and it is not a known issue — **it is
> outstanding work, and it is SD-29's**, not SD-30's and not any successor's.
>
> Everything below describes the bundle's state **as of the rescinded closure**. It is retained as
> an accurate snapshot of that moment, with the two mis-framed §Known issues corrected in place.
> These notes are re-written by `epic-11-closure` when the bundle reaches *real* closure.
>
> **PR #360 (`tranche/9` → `develop`) stays OPEN and unmerged.** The operator merges it at real
> closure.
>
> ### Still not final after closure RUN 2 — 2026-08-13 (`sd29-closure-r2`, card `epic-11-closure-run2`)
>
> Closure run 2 ran, re-derived every lane denominator, and **ruled the bundle NOT CLOSED**. These
> notes are therefore **not** rewritten by it, deliberately: rewriting "Known issues" into shipped
> issues now would make this document describe a state the bundle has not reached, which is the
> precise failure the banner above records.
>
> **63 units of workable work remain**, in two lanes whose chassis are already registered —
> `companion` **53** (card 12) and `monster`/`monster_ability` **10** (card 8), plus **229** monster
> rows mechanism-blocked on `decisions.md §64.1`'s `ABILITY:Internal|AUTOMATIC|` bundle hop.
> `race_trait` is **DRY** (3 residual, all needing a race-variant chassis that is not that card).
> Evidence and the exact commands: `progress.md` `## Cycle SD29-E11-F1-002`.
>
> **Every per-kind figure below is now STALE.** It was re-derived against
> `docs/work-inventory.json` at `generated_at 2026-08-11T10:38:33Z`; sixteen lane rounds have landed
> since. At tip `0ddfc126` the same commands give `companion` 870 grounded / 826 remaining,
> `monster` 1,239 / 31, `monster_ability` 1,623 / 1,484, `race_trait` 513 / 2,934. The figures below
> are left unedited because they are an accurate snapshot of the moment they were written and
> re-stating them piecemeal is how a rollup starts disagreeing with itself — read them as history,
> not as current state.

**Populated at the (since-rescinded) closure of 2026-08-11 (Epic 11, Closure Epilogue).** The per-cycle receipts in
`progress.md` are the per-record evidence; this document is the rollup. Every figure below was
re-derived at closure by the command shown beside it, not transcribed from a receipt.

## Summary

| | |
|---|---|
| Bundle | SD-29 — Corpus-Wide Catch-Up Lanes |
| Branch | `tranche/9` |
| Build version | `0.9.<build>` (`0.9.${GITHUB_RUN_NUMBER}`, `.github/workflows/publish-tester-release.yml:97`); repo version files stamped `0.9.0` by Epic 9 (`ebc5c25a`) |
| Cards | **As re-stated 2026-08-11 after the reopen:** 18 on `kanban.md` — **10 COMPLETE**, **1 DECISION-BLOCKED** (`epic-8-toolkit`, a genuine ruling), **6 READY/reopened** (cards 8-12 lanes + cards 15-16 review/closure), **1 COMPLETE** reopen-correction card (`epic-12-reopen`). The rescinded closure counted the five reopened lane cards as terminal |
| Gate at rescinded closure | `./scripts/verify.sh` full — exit code recorded in `progress.md`'s closure receipt. **A green gate was never the thing in question**: the bundle's code was verifiable; the lanes were undelivered |

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

**Not started — and, per the 2026-08-11 reopen, still owed by SD-29.** All 1,696 `companion` units
remain `not-ingested`/`not-started`, **0** grounded, across 17 books. Stated plainly rather than
implied by a row status — see §Known issues 1.

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

- Tranche promotion PR: `tranche/9` → `develop` (**PR #360**), opened by the rescinded closure card.
  **It stays OPEN and unmerged; the operator merges it at real closure** (`decisions.md §42`).
  `epic-11-closure`, when it re-runs, must NOT open a second PR. Cutting a new tranche branch is
  explicitly *not* part of closure.
- Post-closure version state: repo files at `0.9.0`; publish stamps `0.9.<run_number>`.

## Verification evidence

- Per-cycle receipts in `progress.md` (one `## Cycle` heading per card).
- `./scripts/verify.sh` full at closure — exit code captured directly, never through a pipe;
  recorded in the closure receipt.
- `verify.sh` auto-emits a `verification` retro event per run; per-cycle retro shards live in
  `docs/retro/events/sd29-*.jsonl`.

## Known issues

> **Reframed 2026-08-11 (`decisions.md §42`).** Items 1 and 3 below were written as shipped known
> issues. They are **not** known issues — they are **outstanding SD-29 work**, restored to `READY`
> on `kanban.md` (cards 8, 11, 12). Items 2, 4, 5, 6, 7 and 8 are genuine findings and stand as
> written. The distinction: a known issue is something the bundle *decided* about; an outstanding
> lane is something the bundle *never dispatched*.

1. **🔴 OUTSTANDING SD-29 WORK, NOT A KNOWN ISSUE — the companion lane was never started.** 1,696
   `companion` units, **0** grounded, across 17 books (re-derived 2026-08-11; see `progress.md`
   `## Cycle SD29-E12-F1-001` for the exact command). The Epic 7 pilot refused at `preflight-disk`
   (disk at 91%, below the gate's floor) and the card was correctly left unclaimed. **That refusal
   was an environmental condition, never a scope ruling** — and nothing re-queued the card when the
   disk cleared (80% used, 97G free, re-verified 2026-08-11). Both companion cards are back to
   `READY` and are **SD-29 scope**; the prior text's "a ready re-dispatch for a successor bundle" is
   rescinded by operator directive.
2. **Race chassis is the race-trait ceiling.** `crb::race_traits()` models exactly **7** races. Of
   3,447 `race_trait` units, **805** carry `race_trait_race_not_modelled` and **144**
   `race_trait_absent_from_race_traits`. No book's race traits can ground until a real race chassis
   lands — work outside SD-29's epic structure.
3. **🔴 OUTSTANDING SD-29 WORK, NOT A KNOWN ISSUE — the monster/monster-ability extend ingest was
   never dispatched.** The chassis is proven and merged; the remaining books' **ingest** is the
   lane, and a chassis is not a lane. Re-derived 2026-08-11: `monster` 1,270 total / 60 grounded /
   **1,210 remaining**; `monster_ability` 3,107 total / 17 grounded / **3,090 remaining** (the
   grounded 60 = `bestiary` 46 from SD-22 + `bonus_bestiary` 14 from this bundle's pilot). Card 8 is
   back to `READY` and is SD-29 scope. The **race-trait ingest halves** (cards 9 and 10) are
   likewise outstanding, not shipped — see item 2 for the real structural ceiling underneath them,
   which SD-29 now owns confronting. A published-text dice-grounding decision is genuinely
   outstanding within this lane: 13 of the pilot book's 14 named natural attacks carry no dice in
   the corpus.
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
