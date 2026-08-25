# Cycle t9-onboarding-class-feature-abilities-race-closure/1 — Gate 3 closure invariant / `class_feature`'s last 25 `no_record` units (`decisions.md §20`/`§17a`, discovery-forward from `t9-onboarding-kind-aware-join_cycle-1` §7's "Instrument correction — 1,024 units")

- **Card ID:** `gate-1-shape-closure` (row 5) / `gate-3-closure-invariant` (row 9) — per-kind closure, `class_feature`'s territory per this dispatch.
- **Actor:** `t9-onboarding`
- **Base:** started at pinned `PIN=11a84bced5529c3e7c2c90db8945a99ce891e653`, `origin/tranche/12`'s own tip at dispatch; no rebase needed this cycle (single-cycle turn, pushed once).
- **Commit SHA:** (this cycle's own commit, see push log)
- **Files touched:**
  - `src/rules_core/cache_gen/class_feature.rs` — new `EXTRA_CLASS_FEATURE_SOURCE_FILES` const (2 `(book, source_file)` pairs), `units_from_inventory_json`'s scope check widened to admit them, 1 new unit test.
  - `data/corpus/{advanced_class_guide,advanced_players_guide}/class_feature/**` — 25 new record files (real `cargo run --locked --bin gen_cache_class_feature` output); the other 17,954 pre-existing `class_feature` records each changed by exactly 1 line (`ingested_at` timestamp only — verified, see §3).
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-1-shape-closure/{ledger.json,family-vocabulary.json,family-vocabulary.md}` — regenerated against the live population, real commands (§4).
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 5 entry prepended.
  - `docs/release/SD-32-compute-library-and-cause-closure/progress.md` — cycle entry appended.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff --unified=0 HEAD -- src/rules_core/cache_gen/class_feature.rs`).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope).
- **Acceptance criterion:** dispatch brief, verbatim — close `class_feature`'s 25 `no_record` units (15 `advanced_players_guide` + 10 `advanced_class_guide`), re-deriving first, checking for a stale-twin/already-exists-under-another-kind before concluding un-ingested, and establishing WHY (missing routing row vs. genuine shape refusal) before building.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`; bootstrapped fresh this worktree via `scripts/fetch-pcgen-oracle.sh`, confirmed via `scripts/verify.sh --only preflight-oracle` → PASS after bootstrap).
- **Status:** `complete` — 25/25 closed by real ingest, `class_feature`'s own `no_record` is now 0, Gate 3 budget untouched and still passing on its own merit.
- **Notes:** see full account below.
- **Discovery forwards:** none new this cycle.
- **Next-cycle plan:** none required for `class_feature`'s own territory — its `no_record` is 0. `equipment_modifier`'s 999-unit sibling correction from the same join fix stays a separate lane's territory, untouched here per this dispatch's Territory section.

## 0. Re-derivation of the brief's own figures (`decisions.md §17a`)

The brief's headline ("`class_feature`'s 25 `no_record` units, 15 `advanced_players_guide` + 10
`advanced_class_guide`") re-confirmed unchanged, not assumed: bootstrapped the oracle fresh in this
worktree (empty by default per the brief's own warning), ran
`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/ledger_recheck.json`,
then filtered `kind == "class_feature" and join_status == "no_record"`:

```
count: 25
advanced_class_guide 10
advanced_players_guide 15
```

Exact match. Bundle-wide `no_record` also re-confirmed at 1,251/34,631 before this cycle's fix,
matching the brief's headline number exactly (not the pre-join-fix 227 the brief warns against
trusting).

## 1. Stale-twin / already-exists-under-another-kind check — none found

The brief warns this generator had a prior stale-record-cleanup defect (fresh + stale record at the
same coordinate) and that the kind-blind join collision itself proves a record CAN already exist
under a different kind at the same `(book, source_file, source_line)`. Checked directly, not
assumed: walked every `*.json` under `data/corpus/{advanced_class_guide,advanced_players_guide}/**`
looking for any record (of ANY kind) whose `source_file`/`source_line` matched one of the 25 cited
coordinates (`acg_abilities_race.lst` lines 294–302, 471; `apg_abilities_race.lst` lines 274–279,
284–292). **Zero hits.** These 25 were genuinely un-ingested, not a stale twin and not already
covered under `race_trait_generic` or any other kind at the same lines (the kind-blind join's old
behavior *answered* these units with a `race_trait_generic` record from a DIFFERENT coordinate that
happened to share the join key under the old 3-tuple index — it never wrote a record at these
lines). This is real closure, not instrument correction.

## 2. Why — direct read of both cited files, both shapes confirmed genuine `class_feature`

Read `acg_abilities_race.lst` and `apg_abilities_race.lst` at every cited line directly against the
pinned oracle checkout. Every one of the 25 carries real class-feature tokens:

```
Skald Spell Level 0   CATEGORY:Special Ability  TYPE:BonusSpellKnownSkald
  PREVARGTEQ:(charbonusto("PCLEVEL","Skald") + classlevel("Skald")),1
  STACK:YES  MULT:YES  CHOOSE:NOCHOICE  BONUS:SPELLKNOWN|CLASS=Skald;LEVEL=0|1

Blessings (Favored Class)  KEY:Warpriest ~ Favored Class Blessings
  CATEGORY:Special Ability  TYPE:Warpriest Class Feature.SpecialQuality.Supernatural
  ASPECT:CheckCount|%1|WarpriestBlessingFavoredClassUses  ASPECT:CheckType|Uses per day
```

(Inquisitor/Oracle bonus-spell-known rows in `apg_abilities_race.lst` are the identical shape.)
This is **not** a shape the generator refuses — it is the same `CATEGORY:Special Ability` /
`TYPE:...Class Feature...` / `BONUS:`/`DEFINE:`-counter shape `class_feature.rs` already handles for
every other book. **Root cause is (a): a missing routing row.** These two books' PCGen authors put a
small number of genuine class-feature rows (favored-class-bonus variants tied to spellcasting) in
each book's SECOND abilities file — `*_abilities_race.lst` — alongside that file's otherwise-genuine
race-ability content, rather than in the primary `*_abilities_class.lst` file
`class_feature.rs`'s `ABILITIES_CLASS_FILE_SUBSTRING` scope check exclusively matched. The census
(`v06_work_inventory.rs`) already typed these 25 correctly as `kind: class_feature` (never re-derived
by this generator, per its own doc comment) — the only gap was the generator's file-scope list never
admitting the second file.

## 3. The fix — a precise 2-pair allowlist, not a broadened substring

Added `EXTRA_CLASS_FEATURE_SOURCE_FILES: &[(&str, &str)]`, exactly the two verified pairs
(`("advanced_class_guide", "acg_abilities_race.lst")`, `("advanced_players_guide",
"apg_abilities_race.lst")`), and widened `units_from_inventory_json`'s scope check to
`source_file.contains(ABILITIES_CLASS_FILE_SUBSTRING) || EXTRA_CLASS_FEATURE_SOURCE_FILES.contains(&(book, source_file))`.
**Deliberately not** a broadened `"abilities_race"` substring match — that would sweep in every OTHER
book's `*_abilities_race.lst` (genuinely race content in every book not individually verified) without
the same per-file, per-line read this cycle did. Only these two exact files, for these two exact
books, are admitted.

**RED → GREEN:** new test `units_from_inventory_json_accepts_the_two_known_abilities_race_files_but_no_other_book`
— 3 units in the fixture (the two real coordinates plus a third book's own fabricated
`cr_abilities_race.lst` row); before the allowlist existed this test's assertion `units.len() == 2`
failed (`0` admitted — the exact `no_record` shape reproduced synthetically); after, exactly 2 admitted
(the two real coordinates), the third book's row still excluded. `cargo test --locked --lib
rules_core::cache_gen::class_feature` → **70/70 GREEN** (1 new, 69 unchanged).

## 4. Real regeneration against the pinned oracle, additive-only verified

`git status --porcelain` confirmed clean before running (per this bundle's "check before every
mutation" discipline). `cargo run --locked --bin gen_cache_class_feature`
(`PCGEN_CORPUS_ROOT` = the freshly-bootstrapped oracle checkout):

```
class_feature cache generated: 17979 records across 23 books
  (154 renamed under a Codex-generated neutral name, decisions.md §24)
```

17,954 → 17,979, **+25 exact**, matching the population exactly. None of the 25 new records needed a
`§24` neutral-name rename (checked: 0 of the 25 appear in `CLASS_FEATURE_RENAME_REPORT`'s
`renamed_records`; all 25 ship under license `OGL`).

**Additive-only, verified not assumed:** `git status --porcelain` shows 25 new files plus 17,954
pre-existing files marked modified. `git diff --numstat -- data/corpus | awk '$1+$2>4'` → **0 rows**
— every one of the 17,954 pre-existing records changed by exactly 1 line each (sample-diffed
`aberrant_bloodline.json`: the `ingested_at` timestamp only, no other field). **0 deletions.**

## 5. PI discipline (`decisions.md §15`/`§19`/`§24`)

`python3 -c "...scripts.pi_scrub.normalized_term_hit..."` swept over all 25 new record files:
**0 hits.** All 25 ship under license `OGL`, no `NAMEISPI`/`DESCISPI` declaration on any of the 25
cited rows (confirmed by direct read, §2). No blacklist term appears in this receipt, any test name,
or the commit message this cycle produced (grepped the diff before this write).

## 6. Gate 1 / Gate 3 — honest number, budget constants untouched

`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json` after the regen:

```
population: 34631   unclassified: 0
matched=11413 (33.0%)  no_formula_tokens=21992 (63.5%)  no_record=1226 (3.5%)
```

Bundle-wide `no_record` 1,251 → **1,226**, exactly **−25**. `class_feature`'s own `no_record`
(filtered from the regenerated ledger): **25 → 0.**

`python3 scripts/shape_coverage_standing_gate.py --inventory docs/work-inventory.json`:

```
no_record budget: 1226/34631 vs. baseline 21521/36028 -- exceeded: False
```

`NO_RECORD_BUDGET_COUNT`/`NO_RECORD_BUDGET_POPULATION`: **not edited this cycle** — the honest
post-fix `no_record` (1,226) is well inside the existing, unrepinned budget (`§1a`).

`ledger.json`/`family-vocabulary.{md,json}` regenerated against the live population, corpus SHA
unchanged `7f818006e371188e5717fd18d74d18a420747fc6`.

## 7. Test sweep, scoped

`cargo test --locked --lib rules_core::cache_gen::class_feature` → 70/70 GREEN (10 ignored,
oracle-gated, not run without the env var in this pass — re-run separately with
`PCGEN_CORPUS_ROOT` set where relevant). `cargo test --locked --lib rules_core::cache_gen` (the
whole module family, all sibling generators) → **150/150 GREEN**, 0 failed. No pinned count of
`17954`/`17,954` (the pre-cycle `class_feature` total) found anywhere in `tests/`, `src/`, `scripts/`,
`apps/` (`grep -rln`), so no sweep-fix needed for that figure. `NO_RECORD_BUDGET_COUNT` (21,521) in
`scripts/shape_coverage_standing_gate.py` — untouched, still passes.

## 8. What moved, and why — closure / reclassification / reachability / instrument correction, kept separate (`§16`)

**Closure — 25 units, kind `class_feature`:** written for real under the existing `class_feature`
schema via the existing generic-ingest mechanism (`class_feature.rs` + `gen_cache_class_feature`),
satisfying `decisions.md §20`'s "close by real ingest." These 25 no longer appear as `no_record` in
the post-fix ledger (§6).

**Reclassification — 0 units.** No unit changed its own `kind` field this cycle.

**Reachability — 0.** This cycle writes the corpus cache record only (`data.raw_tokens`,
`data.class`, chassis metadata); it does not add or change any chassis/wiring consumer of these 25
features. No player-reachability claim is made.

**Instrument correction — 0, this cycle's own.** The `+25` `class_feature` instrument correction
this brief opens with was the PRIOR cycle's finding (`t9-onboarding-kind-aware-join_cycle-1` §7); this
cycle's own contribution is the closure that finding named as future work, not a new correction.

**Reconciliation, exact:** bundle-wide `no_record` 1,251 → 1,226 = **−25**, all attributable to
`class_feature`'s own 25 → 0 movement; `matched` 11,389 → 11,413 (+24) and `no_formula_tokens`
21,991 → 21,992 (+1) sum to +25, the mirror image.

## 9. Rebase discipline

Single-cycle turn; pushed once via the §5 retry protocol (see push log). No rebase mid-cycle was
needed.

## 10. Post-rebase re-derivation (`§17a`)

`git fetch origin tranche/12 && git rebase origin/tranche/12` picked up sibling lanes' own
concurrent closures (spell/ability/equipment_modifier/companion/equipment stragglers,
monster_ability round 6, PI-neutral-name closures). Re-derived, not assumed: `class_feature`'s own
`no_record` is still **0** after the rebase (unaffected — no sibling lane touches `class_feature`).
Bundle-wide `no_record` moved further, past this cycle's own contribution, to **132**/34,631
(siblings' own closures, not this cycle's). `ledger.json`/`family-vocabulary.{md,json}` regenerated
fresh against the post-rebase combined corpus state (not merged from conflict markers — the git
auto-merge on these two files produced valid JSON but was not trusted; regenerated for real via
`shape_ledger.py`/`family_vocabulary_reconcile.py` instead). Gate 3: `132/34,631` vs. baseline
`21,521/36,028`, `exceeded: False`. `cargo test --locked --lib rules_core::cache_gen::class_feature`
re-run post-rebase: 70/70 GREEN, unchanged.
