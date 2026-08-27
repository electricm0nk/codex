# Cycle AT-34-E3-001-companion — Epic 3 (Core Rulebook to zero) / AT-34-E3-001

Mechanism owned this cycle: `companion_absent_from_core_rulebook_companion_tables` — one of
the nine bucket-B mechanisms `decisions.md §14` decomposed AT-34-E3-001 into. Population
re-derived at HEAD (not transcribed from the filing cycle's figure):

```
python3 -c "
import json
d=json.load(open('docs/work-inventory.json'))
u=d['units']
cr=[x for x in u if x['book']=='core_rulebook' and x['status']=='engine-does-not-hold' and x['evidence']=='companion_absent_from_core_rulebook_companion_tables']
print(len(cr))"
-> 100
```

Matches `decisions.md §14`'s stated 100 exactly.

- **Commit SHA:** (this cycle's own commit; see `git log -1` after push, recorded in the
  dispatch return value)
- **Files touched:**
  - `scripts/transcribe_companion_tables.py` (Shape 7, book-wide grant — new)
  - `src/rules_core/rules_tables/crb/companion_data.rs` (regenerated)
  - `src/rules_core/rules_tables/companion_chassis.rs` (module-doc figures updated; two
    count-pinned tests updated to the new corpus-true counts, one structural assertion widened
    from equality to membership+count; two new named structural assertions added for the two
    new multi-DESC rows)
  - `docs/work-inventory.json` (regenerated, sequential per `workflow-instruction.md §3`)
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    (re-derived by the same regen; not hand-edited)
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_companion_absent_cycle_receipt.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md`, `kanban.md` (prepend/update, §5)

## The fix

`crb::companion_data` (SD-29's `companion_chassis` extension, per this cycle's mechanism-specific
direction — extended, not duplicated) previously shipped only 46 of Core Rulebook's 130 ability
rows: every generic `Animal Companion ~ …` / `Animal Companion Feat ~ …` / `Animal Trick ~ …` /
`Animal Training ~ …` / `Companion Stat ~ …` record was an orphan under shapes 1-6, because the
corpus states this table exactly ONCE for the whole `CLASS:Companion` chassis
(`cr_classes_companion.lst`) every one of the book's 38 registered creatures shares — no ONE
creature row claims it the way shapes 1-6 all require.

**Shape 7, book-wide grant** (`scripts/transcribe_companion_tables.py`): an exact, closed,
84-key set (never a prefix heuristic) attributed to ALL 38 of this book's registered creatures.
This is a real, corpus-backed fact, not an invented link — PF1's own Animal Companion rules
(CRB p.52-55) grant this identical progression table to every companion regardless of species.
72 of the 84 carry real modelled content (a `TYPE:`, `DESC:`, or `BONUS:` token) and now ship;
the other 12 are `Base Companion ~ …` / `Companion ~ …` internal PCGen plumbing rows (only an
`ABILITY:` grant token, no player-facing content) and are correctly dropped by the pre-existing
empty-payload screen, exactly like any other book's zero-content row — same disposition, not a
regression.

## Row-count command output (before -> after, this mechanism)

```
BEFORE: 100   (companion_absent_from_core_rulebook_companion_tables, core_rulebook, engine-does-not-hold)
AFTER:   28   (12 empty-payload + 2 class-definition rows + 14 cross-book familiar-pool rows remain)
```

Re-derive command (same as above, run against HEAD after this cycle's regen):
`python3 -c "import json; d=json.load(open('docs/work-inventory.json')); u=d['units']; print(len([x for x in u if x['book']=='core_rulebook' and x['status']=='engine-does-not-hold' and x['evidence']=='companion_absent_from_core_rulebook_companion_tables']))"` -> `28`

## Figures + re-derive commands

| Figure | Value | Command |
|---|---|---|
| Mechanism population (before) | 100 of 100 | see above |
| Mechanism population (after) | 28 of 100 | see above |
| Units closed this cycle | 72 | 100 - 28 |
| Ability rows now shipped, core_rulebook | 118 (was 46) | `grep -c 'CompanionAbilityRecord {' src/rules_core/rules_tables/crb/companion_data.rs` -> 118 (creature block excluded by context; verified against module doc's own `"118 companion ability rows"` line) |
| Total core_rulebook companion units (all statuses) | 184 | `python3 -c "import json;d=json.load(open('docs/work-inventory.json'));print(len([x for x in d['units'] if x['book']=='core_rulebook' and x['kind']=='companion']))"` |
| Destination statuses of the 72 closed units | text-complete 40, grounded 29, literal-verified 3 | see cycle log query in progress.md; re-run: filter `docs/work-inventory.json` units by the 84-key `BOOK_WIDE_GRANTS["core_rulebook"]` set and tabulate `(status, evidence)` |
| `core_rulebook` bucket B, atlas-partitioned, corpus-wide (all 9 mechanisms) | 894 of 6,701 | `python3 scripts/completion_atlas.py --by-book` (grep `core_rulebook`) |
| Total corpus population (unchanged by this cycle) | 49,438 | `python3 -c "import json;print(json.load(open('docs/work-inventory.json'))['totals']['units'])"` |
| `corpus_literal_sweep` examined population | 48,708 of 51,482, CLEAN, 0 findings | `/tmp/cargo-sd34-at-34-e3-001/release/corpus_literal_sweep --json-out <path>` (this cycle added 0 `data/corpus` records, so this movement is inherited from prior cycles' merges, not this one) |
| `derived_evaluator_fixture_check` | 1,839 units cleared over 2,580 fixture rows, 0 failed | `/tmp/cargo-sd34-at-34-e3-001/release/derived_evaluator_fixture_check --json-out <path>` |
| `completion_atlas.py --check` | population=49438 buckets=10 unclassified=0 overlap=0, citation_failures=0 | `python3 scripts/completion_atlas.py --check` |
| `denominator_gate.py` against this package | violations=0 (15 files checked) | `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` |

## Remainder (28 units), named by sub-cause — none of it is a shrug

| Sub-cause | Units | Why it is not closed this cycle |
|---|---:|---|
| Zero-content internal PCGen plumbing rows (`Base Companion ~ Animal Companion`, `Base Companion ~ Special Mount`, `Companion ~ Ability Score Increase`/`Bonus Tricks`/`Devotion`/`Evasion`/`Improved Evasion`/`Link`/`Multiattack`/`Share Spells`/`Spell Resistance (AC)`/`Spell Resistance (SM)`) | 12 | Each row carries ONLY an `ABILITY:` grant token — no `TYPE:`, `DESC:`, or `BONUS:` — so every field this chassis models transcribes empty. Same disposition the empty-payload screen (`decisions.md §63.3`) already applies corpus-wide; shipping these would be exactly the stub-class card the screen exists to prevent. |
| `cr_classes_companion.lst` PCGen monster-class definitions (`Companion`, `Shadow Companion`) | 2 | A monster class is a hit-dice level-progression construct — no `SIZE:`, `MOVE:`, or natural attacks — not a creature and not an ability. Modelling it is a genuinely new record type (a level-progression table), a real widening this cycle does not take, per the standing architecture decision this book's own module doc already states (`decisions.md §65.1`). |
| `ce_abilities_familiar_cr.lst` master-side familiar special-ability pool, reattributed to `core_rulebook` (`Familiar ~ Alertness`/`Deliver Touch Spells`/`Empathic Link`/`Improved Evasion`/`Intelligence Score`/`Natural Armor Bonus`/`Scry on Familiar`/`Share Spells`/`Speak One Language`/`Speak with Animals of Its Kind`/`Speak with Master`/`Spell Resistance`, `Familiar Alertness Choice ~ Alertness Active`/`Alertness Inactive`) | 14 | This is a real, generic Familiar special-ability table (CRB's own Familiar rules) — but `core_rulebook`'s own creature roster is all 38 Animal Companions; it registers **no familiar creature**. `companion_chassis`'s ownership invariant (`the_chassis_link_resolves_in_both_directions_for_every_book`) requires every ability's owner to resolve via `companion_resolve` to a `CompanionRecord` of the SAME book — familiars are drawn from OTHER books' chassis tables, so no true, same-book owner exists to attribute this to without fabricating one. A pinned unit test (`a_companion_reattributed_to_a_chassis_book_that_does_not_hold_it_is_bucket_b_not_a`, `AT-34-E2-004`) already fixes this shape's INTENDED disposition as "must be truly placed, never reclassified" — confirming bucket D is not an available escape hatch here; it must be placed for real, which needs either a cross-book ownership shape (Shape 8, not built this cycle) or a master-side ability-pool record type this chassis does not have. |

**28 + 72 = 100.** Every remaining unit is named by sub-cause with a population; none is
folded into "the rest."

## Verification

- TDD: RED confirmed by re-running `cargo test --lib rules_core::rules_tables::companion_chassis`
  immediately after the transcriber regen and BEFORE editing the two count-pinned tests — 2 of 15
  failed for the intended reason (stale counts 39/11 vs the new corpus-true 93/13), one further
  structural failure surfaced on the next run (the `vec!["Crocodile ~ Tail Slap"]` equality) and
  was fixed the same way. GREEN: all 15 `companion_chassis` tests pass; all 126 `companion`-scoped
  lib tests pass; full `cargo test --lib` (workspace lib target) — 2,872 passed, 0 failed, 14
  ignored.
- **Build scope verified:** `cargo test --locked --no-run` exits 0 (workspace, all bin/test
  targets built), run at commit `<this cycle's SHA>`. `apps/desktop/src-tauri` not touched this
  cycle — not built.
- Identifier audit: `OK_NO_BUNDLE_TAGS`.
- Wired-integration audit: no stub/mock/placeholder/todo/fixme/hack token introduced by this
  cycle's own diff (a handful of `placeholder` matches appear in the epic's cumulative
  `${BASE_BRANCH}...HEAD` diff over `src/bin/`/`src/rules_core/`, all from an EARLIER,
  already-merged AT-34-E3-001 sub-cycle's human-ethnicity corpus rows — real corpus content
  describing PCGen "Placeholder objects", not an implementation stub, and not this cycle's
  diff).

## Movement, four buckets

- **Closure:** 72 units (bucket B -> DONE-tier: 40 `text-complete`, 29 `grounded`, 3
  `literal-verified` — the 3 already-verified ones ride the `corpus_literal_sweep`'s existing
  CLEAN sweep, not a new run this cycle triggered).
- **Reclassification:** 0 (no unit moved bucket without being genuinely placed/held).
- **Reachability:** 0 new `reach_gate` findings — these rows reach the player through the same
  `companion_catalog` render path every other `crb::companion_data` row already uses; no new
  wiring was needed beyond the chassis table placement itself.
- **Instrument-correction:** 0. This is real work (record placement), never a count-only fix.

- **Status:** partial

## Notes

- The mechanism-specific direction ("EXTEND `companion_chassis`, do not build a second table")
  was followed literally: no new struct, no new `CompanionBook` field. Shape 7 lives entirely in
  the transcriber's ownership-resolution pipeline, the same place shapes 1-6 already live.
- `book_wide_applied` (the transcriber's own counter) printed `84` to stderr on this book's
  regen, confirmed against the 84-key literal set.
- The remainder's second and third sub-causes are BOTH genuine architecture gaps this cycle
  does not close, named honestly rather than forced: the class-row gap was already a standing,
  documented SD-29 decision; the familiar-pool gap is newly surfaced by this cycle's own
  investigation (no prior receipt named it this precisely) and is real future work, not a
  shrug — a cross-book ownership shape (Shape 8) or a new master-side ability-pool record type,
  either genuinely new engine capability.

## Next-cycle plan

Two remaining fully-unstarted `AT-34-E3-001` mechanisms per `decisions.md §14`:
`race_trait_race_not_modelled` (132) and the two `class_feature_*` mechanisms (330, 333) not yet
picked up, plus this cycle's own 28-unit remainder (12 zero-content rows are permanently
excluded by design, matching every other book's empty-payload screen — not further "closable"
without inventing content the corpus does not state; the 2 class rows and 14 familiar rows are
real future engine-capability work, named above). A future cycle that wants to close the 14
familiar-pool units should design Shape 8 (cross-book ownership) or a dedicated
`FamiliarAbilityPool` record type before attempting a corpus-wide sweep for other cross-book
familiar shapes this same gap likely affects (`beastiary`/`bestiary_2` etc. familiar creatures
may face the mirror-image gap: real familiar creatures with no core-rulebook-table access).
