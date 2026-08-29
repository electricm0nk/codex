# Cycle AT-34-E5-002 — Epic 5 (Price the remaining 35 books) / AT-34-E5-002

- **Commit SHA:** TBD (fixed in a same-cycle follow-up commit, per AT-34-E5-001's precedent — this receipt is written before its own commit exists)
- **Files touched:**
  - `docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/build_capability_register.py` (new)
  - `docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/verify_capability_register.py` (new)
  - `docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/capability-register.json` (new, generated)
  - `docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/AT-34-E5-002_cycle_receipt.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md` (updated)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "Beyond the `power` table: anything
  Epics 3 or 4 proved is required and does not exist. **Evidence:**
  `artifacts/epic-5-forward-plan/capability-register.json` — per capability: what it is, which
  buckets and books it unblocks, its population, and whether SD-34 built it. This is the
  operator's second explicit question answered in machine-readable form."

## What this cycle built

`build_capability_register.py` re-derives `capability-register.json` at HEAD, every run. **10
named capabilities**, none built by SD-34 (`built_by_sd34: false` on every row — this register's
job is to name what still must be built, not to build it; `epic-5-forward-plan`'s file-touch set
is read-only against the rest of the repo, `workflow-instruction.md §3`).

**8 of 10 carry a live, mechanically re-derived population** (checked fresh against
`docs/work-inventory.json` / `missing-engine-tables.json` on every run, never a frozen number):

| Capability | Population | Buckets | Books | Source |
|---|---|---|---|---|
| `power_engine_table` | 421 | A | `ultimate_psionics` | live: `missing-engine-tables.json`, cross-checked against a second, independent evidence-string query |
| `companion_table_shape_widening` | 28 | A | `bestiary` | live, same cross-check |
| `per_character_choice_filter` | 113 | X | `core_rulebook` 96, `inner_sea_combat` 13, `advanced_class_guide` 2, `ultimate_magic` 1, `ultimate_wilderness` 1 | live: bucket-X evidence matching the ranger combat-style / sorcerer bloodline / arcane-bloodline-subchoice option-pool shapes |
| `companion_mount_advancement_table` | 9 | X | 8 books (druid/cavalier/hunter/etc. companion+mount classes) | live: bucket-X evidence containing `advancement_absent` |
| `class_feature_deep_subsystem_modelling` | 32 (18 named sub-mechanisms) | X | 8 books | live: remaining bucket-X `engine_diagnostic:class_feature.*` evidence not matched by the two rows above |
| `marker_stripping_for_pcgen_editorial_markers` | 21 (confirmed) + ~392 unsized project-wide candidate, named separately, not blended | U | `ultimate_campaign` | cited: AT-34-E4-001/E4-002 receipts |
| `monster_class_hit_dice_progression_modelling` | 2 | B | `core_rulebook` | cited: AT-34-E3-001 companion_absent receipt (flagged: not independently re-derivable by a live evidence-string query this cycle) |
| `master_side_ability_pool_record_type_or_cross_book_ownership` | 14 | B | `core_rulebook` | cited: same receipt, same caveat |
| `corpus_content_extraction_for_uncaptured_records` | **UNMEASURED** | B-candidate | `advanced_players_guide` (at least) | cited: `atlas-defects.md` #2, meaning 3 — population explicitly not sized, never blended into a total |
| `cross_record_content_ownership_resolution` | **UNMEASURED** | B-candidate | `core_rulebook` (at least) | cited: `atlas-defects.md` #2, meaning 2 — same discipline |

**X-bucket reconciliation is total, not partial:** every one of the live bucket-X population's
171 units is accounted for by exactly one row — the three named capabilities above (113 + 9 + 32
= 154) plus two "no capability needed" shapes that atlas-defects.md already correctly resolved
into `X` with no further engine work (`grant_token_only_dispatch_row`, 12; `vacuous_placeholder_row`,
3) plus the 2 `ultimate_campaign` marker-shaped `X` units = 154 + 12 + 3 + 2 = **171 of 171**,
proven by `verify_capability_register.py` against a live `completion_atlas.py` partition, not
asserted.

**Scope discipline (the acceptance bar's own boundary):** ordinary bucket-B content-placement
work — a record that only needs to be placed in an *already-existing* table — is already named
and priced generically by `AT-34-E5-001`'s `forward-plan.json` and is **not** repeated here. This
register names *new engine machinery that does not exist yet*, matching the criterion's own
"anything Epics 3 or 4 proved is required and does not exist" — the 634 corpus-wide
`companion_absent_from_<book>_companion_tables` bucket-B units (ordinary placement into an
already-built table) are deliberately excluded on this ground.

## RED → GREEN (TDD, `workflow-instruction.md §6` step 3)

**RED, confirmed for the intended reason:** mutated the committed artifact directly — set
`power_engine_table.population` to `999` (wrong; live re-derivation says `421`) and
`master_side_ability_pool_record_type_or_cross_book_ownership.built_by_sd34` to `true` (an
illegal claim — a built capability must be removed from the register, not flagged). Live run:

```
FAIL: 2 violation(s)
 - master_side_ability_pool_record_type_or_cross_book_ownership: built_by_sd34=true -- a built capability must be REMOVED from this register, not flagged true
 - power_engine_table: register says 999, live re-derivation says 421
```

Both failures fired for the planted defect, not an unrelated crash. Reverted by re-running
`build_capability_register.py` (which re-derives every field from HEAD, discarding both
mutations).

**GREEN:**

```
$ python3 docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/verify_capability_register.py
PASS: 10 capabilities named, X-bucket reconciliation sums to live population (171), 0 flagged built_by_sd34=true
```

## Figures + their re-derive commands

- `population=49438 buckets=10 unclassified=0 overlap=0` — `python3 scripts/completion_atlas.py --check` (denominator for every figure below).
- Bucket X live population: **171 of 49,438** — same command, `X:` row.
- `power` kind, `ultimate_psionics`: **421** — status==`engine-does-not-hold` AND evidence contains `has_no_engine_table` AND kind==`power`, over `docs/work-inventory.json`; cross-checked against `artifacts/epic-1-atlas/missing-engine-tables.json`'s own `power.count`.
- `companion` kind, `bestiary`: **28** — same query, kind==`companion`; cross-checked against `missing-engine-tables.json`'s `companion.count`.
- `per_character_choice_filter`: **113 of 171** X units — status==`deferred-with-reason` AND evidence matches `combat_style_feat_pool.option.` / `bloodline_feat_pool.option.` / `progression_subchoices_unresolved`, over `docs/work-inventory.json`.
- `companion_mount_advancement_table`: **9 of 171** X units — same partition, evidence contains `advancement_absent`.
- `class_feature_deep_subsystem_modelling`: **32 of 171** X units, 18 named sub-mechanisms — the remainder of bucket X's `engine_diagnostic:class_feature.*` evidence not matched by the two rows above.
- `no_capability_needed` rows: **12** (`grant_token_only_dispatch_row...`) + **3** (`vacuous_placeholder_row...`) — same partition; these are atlas-defects.md's own already-resolved shapes, named here only so the reconciliation totals 171, not treated as capabilities to build.
- `marker_stripping_for_pcgen_editorial_markers`: **21** confirmed (`ultimate_campaign` bucket U, AT-34-E4-001's own corpus-wide test), **~392** project-wide is an ESTIMATE quoted from that same receipt's own corpus-wide grep, not independently re-run this cycle and not blended into the 21.
- `monster_class_hit_dice_progression_modelling`: **2** — cited from `AT-34-E3-001_companion_absent_cycle_receipt.md`; not independently re-derivable by a live evidence-string query this cycle (named limitation, not a fabricated re-derivation).
- `master_side_ability_pool_record_type_or_cross_book_ownership`: **14** — same source, same limitation.
- Denominator gate against this package: `python3 scripts/denominator_gate.py --check 'docs/release/SD-34-book-completion/*.md'` → `files_checked=15 violations=4` — all 4 pre-existing in `progress.md` (lines 147, 190, 247, 253), all inside verbatim-quoted corpus prose ("75% chance...") already flagged and explained by the already-merged `AT-34-E3-004` cycle; none introduced by this cycle (this cycle touched no `.md` file's prose besides its own receipt, `progress.md`'s prepended entry, and `kanban.md`'s row — none of which contain a bare percentage).

## Row-count command output

```
$ python3 -c "import json; d=json.load(open('docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/capability-register.json')); print('capabilities:', len(d['capabilities']), 'sized:', d['summary']['sized_capability_count'], 'unsized:', d['summary']['unsized_capability_count'], 'built_by_sd34_count:', d['summary']['built_by_sd34_count'])"
capabilities: 10 sized: 8 unsized: 2 built_by_sd34_count: 0
```

## Build scope verified

`cargo test --locked --no-run` exit 0 at `0be7d54a8d2c6c0b879744a2ed3325acbba1f594` (the base SHA
this cycle started and stayed at — no Rust source touched, Python/JSON-only change).
`apps/desktop/src-tauri` not touched, not run.

## Sweep population

N/A — this cycle added no corpus records and regenerated none. `corpus_literal_sweep`'s
examined-population is unaffected.

## Oracle pin

N/A — no figure in this register came from the pinned PCGen oracle corpus.

- **Status:** complete

## Movement, four buckets

**Instrument-correction / naming, zero unit movement.** This cycle moves no unit on any bucket
board (`docs/work-inventory.json` untouched, confirmed by `git status --porcelain` showing no
diff to that file). It is a **naming** artifact: 10 capabilities, 0 built, matching the
criterion's own bar exactly (name what must still be built — do not build it here).

## Notes

- Two capabilities (`monster_class_hit_dice_progression_modelling`,
  `master_side_ability_pool_record_type_or_cross_book_ownership`) are sourced from an
  already-verified prior cycle's receipt rather than independently re-derived by a live
  evidence-string query this cycle — their current live evidence keys did not resolve by a
  direct grep. Flagged plainly in the register's own `verification_note` field rather than
  silently presented with the same confidence as the 8 live-derived rows. The next lane that
  touches the `companion_absent` mechanism should pin a live, `file:line`-cited evidence key for
  these two shapes the way `AT-34-E1-002` condition 6 requires for atlas buckets.
- Two capabilities (`corpus_content_extraction_for_uncaptured_records`,
  `cross_record_content_ownership_resolution`) are named with an explicitly **unsized**
  population, per `atlas-defects.md` #2's own finding that splitting the 517-record shape into
  its three meanings by evidence (not by shape) has not run. Naming an unsized capability
  honestly (rather than omitting it, or guessing a number) is itself required by the acceptance
  bar's "anything Epics 3 or 4 proved is required and does not exist" — both are proved required,
  neither is sized, and the register says so in both places rather than picking one.
- Deliberately excluded: the 634 corpus-wide `companion_absent_from_<book>_companion_tables`
  bucket-B units. These need an *existing* table's ordinary placement mechanism, already named
  and priced generically by `AT-34-E5-001`, not a new capability — including them here would
  double-count against that artifact and blur the distinction the criterion itself draws
  ("**anything** ... **does not exist**" — an existing table's unused rows are not "does not
  exist").

## Next-cycle plan

1. `AT-34-E5-003` (power table costed) consumes this register's `power_engine_table` row directly
   rather than re-deriving the 421 figure a third time.
2. A future cycle wanting to close the two "cited" capabilities' population gap should pin a live
   `file:line` evidence citation for the monster-class and familiar-pool shapes, the same
   discipline `AT-34-E1-002` condition 6 already enforces for atlas buckets.
3. Sizing `corpus_content_extraction_for_uncaptured_records` and
   `cross_record_content_ownership_resolution` requires the per-record split of the 517-record
   "no description, structural tokens only" shape that `atlas-defects.md` #2 names as unstarted
   work — out of this cycle's scope (naming, not sizing an unsized shape further than "unsized").
