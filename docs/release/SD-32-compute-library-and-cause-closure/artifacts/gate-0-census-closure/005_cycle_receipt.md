# Cycle 005 — Gate 0 / Card 15 (`census-scope-closure`) — measurement lane: template_row, deity, power, domain, language, untypeable files

- **Card ID:** 15 (`census-scope-closure`, Gate 0 + Gate 1, `decisions.md §12b`) — one of three
  concurrent measurement lanes on this card; this lane covers `kind_unenumerable` minus
  `class_feature`/`ability_category:*`, plus `unclassified:<file>` and `non_object_files`.
- **Commit SHA:** (this cycle's commit — see push output below)
- **Files touched:** `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-other-kinds-memo.md`
  (new), this receipt, `progress.md` (append), `kanban.md` (row 15 status),
  `docs/retro/events/card-15-other-kinds.jsonl` (new, one correction event).
- **Identifier audit result:** OK_NO_BUNDLE_TAGS
- **Wired-integration audit result:** OK_NO_TOKENS
- **Acceptance criterion:** `decisions.md §12b` — "Card 15 closes them. Closure means each object
  is either (a) enumerated as a unit in a tracked kind, classified into a shape family, and
  covered by Gate 3's standing gate, or (b) proven not to be an object … by class, with the
  committed command that proves it and the count it accounts for." This cycle is the measurement
  half (the decision memo); the integration cycle applies the dispositions to
  `docs/work-inventory.json`/`scripts/census_independent.py`/`scripts/shape_ledger.py`.
- **Corpus SHA:** `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`).
- **Status:** complete (this lane's memo) — card 15 itself stays `in-progress` (three lanes plus
  an integration cycle share the row; not set to `complete` by this cycle).
- **Notes:** Every one of this lane's 6 named buckets (`template_row` 2,343, `deity` 460, `power`
  421, `domain` 183, `language` 143, `kit` 1 — sum 3,551, matching `diff.json` exactly) is
  disposition (A) — real, currently-uncounted objects, 7 candidate new kinds
  (`template`/`deity`/`power`/`domain`/`language`/`kit`/`skill`). The `unclassified:<file>` 179
  units split: 170 (`*_skills.lst`, 10 files) are disposition (A) — a whole missing kind (`skill`)
  the filename classifier never had a branch for; 9 (`ce__sizes.lst`) are disposition (B) —
  PF1e's fixed 9-category size table, already engine-covered by `src/rules_core/size.rs`'s
  identical variant list, proven by class not filename. All 253 `non_object_files` are confirmed
  non-object by content, with `profs_weapon`/`profs_armor`/`profs_shield` (35 files, 450 non-`.MOD`
  rows) requiring a row-level duplicate-of-equipment proof (418/450 match an existing
  `equipment`-kind record's name/KEY; the remaining 32 are proficiency-group category labels, not
  instances) rather than a bare filename-token trust — the "reverse error" the brief warned about.
  Shape families assigned to every disposition-(A) unit using card 14's canonical vocabulary
  (`shape_ledger.FAMILIES`/`classify_formula`/`extract_formula_segment`, same priority order),
  applied directly to each row's own `DEFINE`/`BONUS*` fields (no corpus-JSON join available or
  needed — none of this lane's content is ingested anywhere, verified per-bucket). One
  `scripts/retro.py correction` logged for the `*_skills.lst` classifier-gap finding. Per this
  card's scope note, `docs/work-inventory.json`, `scripts/census_independent.py`,
  `scripts/shape_ledger.py`, and pinned-count files are untouched — the integration cycle applies
  these dispositions.
- **Discovery forwards:** none beyond the retro correction above (no `## DISCOVERED` entry needed
  — the skills-kind gap is fully resolved within this memo's own scope, not deferred).
- **Next-cycle plan:** the integration cycle (per this card's own design) reads this memo plus the
  two sibling lanes' memos (`class_feature`/`ability_category:*`), adds the 7 new kinds to
  `docs/work-inventory.json`'s tracked-kind list, extends `scripts/census_independent.py`'s
  `_classify_kind_by_filename` with the new branches this memo names (`template`, `deity`,
  `power`, `domain`, `language`, `skill`, and the `kit` single-row disposition), re-runs
  `scripts/shape_ledger.py` over the widened population, and re-verifies `unclassified_count`
  stays 0 and `total_kind_unenumerable_units` drops to the sibling lanes' residual only.
