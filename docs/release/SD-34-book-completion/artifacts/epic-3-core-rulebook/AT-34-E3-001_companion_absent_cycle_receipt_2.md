# Cycle 2 — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`companion_absent_from_core_rulebook_companion_tables` mechanism)

Mechanism owned this cycle: `companion_absent_from_core_rulebook_companion_tables` — one of
the nine bucket-B mechanisms `decisions.md §14` decomposed AT-34-E3-001 into. This is the
SECOND cycle on this mechanism; the FIRST (`AT-34-E3-001_companion_absent_cycle_receipt.md`)
took it 100 → 28 with Shape 7, book-wide grant, and named the 28-unit remainder by sub-cause.
This cycle's mandate, verbatim: *"16 of your 28 are cross-book-owned rows (14 familiar
ability-pool, 2 monster-class) that a prior cycle judged to need a new record type; re-derive
that judgement rather than inheriting it — if a narrower fix closes them, take it."*

Population re-derived at HEAD (not transcribed from the prior receipt):

```
python3 -c "
import json
d=json.load(open('docs/work-inventory.json'))
u=d['units']
cr=[x for x in u if x['book']=='core_rulebook' and x['status']=='engine-does-not-hold' and x['evidence']=='companion_absent_from_core_rulebook_companion_tables']
print(len(cr))"
-> 28
```

Matches the filing cycle's own after-figure exactly. Confirmed unchanged before doing anything
else.

- **Commit SHA:** `3ab29e930af4e1ffe182e8d1cb4d6d8827af02a2`
- **Files touched:**
  - `src/rules_core/rules_tables/companion_chassis.rs` — one new, committed, passing
    regression test: `companion_absent_28_sub_causes_are_named_and_sum_exactly`. No production
    code changed. The test re-derives this mechanism's 28-unit `core_rulebook` population
    directly from the live `docs/work-inventory.json`, cross-checks each unit against the live
    ingested corpus (`data/corpus/core_rulebook/companion/*.json`), and — for the 14 familiar
    rows — additionally proves the true owner is a creature the corpus DOES register, just
    under a different book (`data/corpus/beastiary/companion/{bat,cat,...}.json`), which is
    this cycle's re-derived evidence for why the judgement stands.
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_companion_absent_cycle_receipt_2.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md`, `kanban.md` (prepend/update, §5)

## Re-derivation: the judgement was checked, not inherited

**The two monster-class rows (`Companion`, `Shadow Companion`).** Read the raw corpus rows
directly (`cr_classes_companion.lst:6`, `:15`): both are `CLASS:` rows with `VISIBLE:NO`, an
`HD:`/`MAXLEVEL:` progression, and a fistful of `BONUS:VAR|Class…` tokens — no `SIZE:`, no
`MOVE:`, no `NATURALATTACKS:`, nothing `CompanionRecord` or `CompanionAbilityRecord` models.
This IS a genuinely different record shape (a hit-dice level-progression table), not a
predicate `companion_chassis` can be widened to accept without inventing fields no other book's
record carries. The prior cycle's judgement is confirmed correct: no narrower fix exists inside
this chassis. Modelling PCGen monster classes is real, separate future engine capability.

**The 14 familiar-ability-pool rows.** This is where re-deriving (rather than inheriting)
produced NEW evidence the filing cycle's own receipt did not have:

1. `ce_abilities_familiar_cr.lst` (the 14 orphaned ability rows' source file) declares
   `SOURCELONG:Core Rulebook` — `decisions.md §9`'s re-attribution correctly files these rows
   under `core_rulebook`. This part is not in question.
2. `ce_races_familiar_cr.lst` — the file that actually DECLARES the 11 familiar creatures this
   ability pool describes (Bat, Cat, Hawk, Lizard, Monkey, Owl, Rat, Raven, Toad, Viper,
   Weasel; PF1's own Familiars table, CRB p.52-55) — declares `SOURCELONG:Bestiary`, so the
   SAME `decisions.md §9` re-attribution correctly files THOSE rows under `beastiary`, not
   `core_rulebook`. Verified directly against the live ingested corpus:
   `data/corpus/beastiary/companion/{bat,cat,hawk,lizard,monkey,owl,rat,raven,toad,viper,weasel}.json`
   all exist (this cycle's new test asserts this — see below) — every one of the 11 familiar
   creatures already ships, as a registered `CompanionRecord`, under `beastiary`.
3. This is **not a reattribution bug and not a "no such creature exists" gap** — it is a real
   split baked into the actual books: Core Rulebook states the *ability rules* a familiar gets
   (Magic chapter), Bestiary states the *creature stat blocks* (Bat, Cat, Hawk, ...). Two real
   books, two real halves of one PF1 mechanic.
4. `companion_chassis`'s own corpus-wide invariant,
   `every_shipped_ability_row_is_owned_by_a_creature_of_its_own_book`
   (`the_chassis_link_resolves_in_both_directions_for_every_book`), requires every ability's
   owner to resolve to a `CompanionRecord` of the SAME book, enforced across every one of the
   9 currently-registered companion books, not just `core_rulebook`. Attributing these 14 rows
   to `beastiary`'s familiar creatures would require widening that invariant to cross-book
   resolution (Shape 8) — a real, corpus-wide change to a shared safety property every other
   registered book currently relies on, not a narrow, single-book fix this cycle's scope
   covers. Doing it unsafely (e.g. hand-waiving the invariant for just these 14) would let a
   future orphan silently ride the same escape hatch, exactly the hazard the invariant exists
   to prevent.

**Conclusion: the prior cycle's judgement is correct, now with corpus-proof rather than
assertion.** No narrower fix closes either sub-cause without inventing content the corpus does
not state (the class rows) or weakening a corpus-wide safety invariant every other registered
book depends on (the familiar rows). Both remain real, named future engine-capability work —
Shape 8 (cross-book ownership) for the familiar pool, a level-progression record type for the
monster classes — not something this cycle forces through as a false closure.

## Row-count command output (before -> after, this mechanism)

```
BEFORE: 28   (companion_absent_from_core_rulebook_companion_tables, core_rulebook, engine-does-not-hold)
AFTER:  28   (unchanged — 0 units closed this cycle; judgement re-derived and confirmed, not a narrower fix)
```

Re-derive command (same as above): `python3 -c "import json; d=json.load(open('docs/work-inventory.json')); u=d['units']; print(len([x for x in u if x['book']=='core_rulebook' and x['status']=='engine-does-not-hold' and x['evidence']=='companion_absent_from_core_rulebook_companion_tables']))"` -> `28`

## Figures + re-derive commands

| Figure | Value | Command |
|---|---|---|
| Mechanism population (before, this cycle) | 28 of 28 | see above |
| Mechanism population (after, this cycle) | 28 of 28 | see above — unchanged |
| Units closed this cycle | 0 | 28 - 28 |
| Sub-cause partition (pinned regression test) | 12 + 2 + 14 = 28 | `cargo test --lib rules_core::rules_tables::companion_chassis::tests::companion_absent_28_sub_causes_are_named_and_sum_exactly -- --nocapture` |
| Familiar creatures already registered under `beastiary` (cross-book proof) | 11 of 11 | same test — asserts existence of `data/corpus/beastiary/companion/{bat,cat,hawk,lizard,monkey,owl,rat,raven,toad,viper,weasel}.json` |
| `core_rulebook` bucket B, atlas-partitioned, corpus-wide (all 9 mechanisms) | unchanged this cycle | `python3 scripts/completion_atlas.py --by-book` (grep `core_rulebook`) |
| Total corpus population (unchanged by this cycle) | 49,438 | `python3 -c "import json;print(json.load(open('docs/work-inventory.json'))['totals']['units'])"` |
| `completion_atlas.py --check` | population=49438 buckets=10 unclassified=0 overlap=0, citation_failures=0 | `python3 scripts/completion_atlas.py --check` |
| `denominator_gate.py` against this package | violations=0 (15 files checked) | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` |

## Verification

- TDD: this is a proof/regression cycle, not a defect fix — RED is not applicable in the usual
  sense (no production behavior changed). The new test itself is the deliverable: it was
  written to FAIL if the 28-unit population, the 12/2/14 partition, or the cross-book
  creature-existence claim ever drift, and passes GREEN today against live `docs/work-inventory.json`
  and the live ingested corpus.
- `cargo test --lib rules_core::rules_tables::companion_chassis` — all 16 tests pass (15
  pre-existing + 1 new).
- Full `cargo test --lib` (workspace lib target): **2,875 passed, 0 failed, 14 ignored.**
- **Build scope verified:** `cargo test --locked --no-run` (workspace, all bin/test targets)
  exits 0 at this cycle's HEAD (`3ab29e930a`). `apps/desktop/src-tauri` (separate cargo
  workspace) also run explicitly this cycle: `cargo test --locked --no-run` exits 0 too
  (`CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-001-desktop`).
- Identifier audit: `OK_NO_BUNDLE_TAGS`.
- Wired-integration audit: `OK_NO_TOKENS`.
- `git status --porcelain` before this cycle's only write confirmed a clean working tree except
  the pre-existing, untouched `docs/retro/events/sd31-transcribe.jsonl` (another lane's dirty
  file, left alone) and the pre-existing untracked SD-33 `*.workflow.js` litter (also left
  alone).

## Movement, four buckets

- **Closure:** 0 units.
- **Reclassification:** 0 units.
- **Reachability:** 0 new `reach_gate` findings — no shipped table changed.
- **Instrument-correction:** 0 — this cycle neither found nor fixed a bad count; the population
  was already correctly named by the prior cycle. This cycle is a re-derivation/verification
  cycle (`decisions.md §14`'s "re-derive rather than inherit" instruction), which lands as a
  committed proof artifact, not a moved figure.

- **Status:** partial

## Remainder (28 units), named by sub-cause — unchanged, now proven by a committed test

| Sub-cause | Units | Why it is not closed this cycle |
|---|---:|---|
| Zero-content internal PCGen plumbing rows (`Base Companion ~ …`, `Companion ~ …`) | 12 | Carry only an `ABILITY:` grant token — no `TYPE:`, `DESC:`, or `BONUS:` — verified this cycle directly against each row's live `data/corpus` `raw_tokens`. Same disposition as every other book's empty-payload screen. |
| `cr_classes_companion.lst` PCGen monster-class definitions (`Companion`, `Shadow Companion`) | 2 | Verified this cycle against the live corpus record's `source.path` (`cr_classes_companion.lst`) and its `raw_tokens` (`VISIBLE:NO`, no `SIZE:`/`MOVE:`/natural attacks). A level-progression table, a genuinely new record type — confirmed, not assumed. |
| `ce_abilities_familiar_cr.lst` master-side familiar special-ability pool | 14 | Re-derived this cycle with NEW corpus proof: the pool's true owners (11 familiar creatures) already ship as registered `CompanionRecord`s — under `beastiary`, per the SAME §9 reattribution rule that correctly filed the pool itself under `core_rulebook`. A real cross-book split in the actual books, not a bug. Closing it needs Shape 8 (cross-book ownership), a corpus-wide widening of `companion_chassis`'s same-book invariant every other registered book relies on — not a narrow, single-book fix this cycle's scope covers. |

**28 = 12 + 2 + 14**, proven by a committed, re-runnable test rather than asserted in prose.

## Next-cycle plan

Unchanged from the filing cycle's own plan: this mechanism has no further narrow-scope work
available. A future cycle that wants to close the 14 familiar-pool units must design Shape 8
(cross-book ownership resolution) as a deliberate, corpus-wide widening — verified against
every currently-registered companion book, not just `core_rulebook` — or build a dedicated
`FamiliarAbilityPool` record type. The 2 class rows need a level-progression record type. The
12 zero-content rows are permanently excluded by design (same disposition as every other book's
empty-payload rows), not further closable without inventing content the corpus does not state.
AT-34-E3-001 itself remains open — the other eight mechanisms are owned by other cycles.
