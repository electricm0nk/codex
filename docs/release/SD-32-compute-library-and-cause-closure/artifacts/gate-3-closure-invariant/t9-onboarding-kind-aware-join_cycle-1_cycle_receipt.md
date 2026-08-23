# Cycle t9-onboarding-kind-aware-join/1 — Gate 3 closure invariant / `shape_ledger.py` kind-blind join (`decisions.md §17a`/`§20`, discovery-forward from `epic-6-kind-trait_cycle-2_cycle_receipt.md` §4)

- **Card ID:** `gate-1-shape-closure` (row 5) / `gate-3-closure-invariant` (row 9) — instrument fix, not a per-kind closure card
- **Actor:** `t9-onboarding`
- **Base:** started at pinned `PIN=930cc4c3d37967a0d3c6af63502a93ec1a0a4bf2`; that SHA was also `origin/tranche/12`'s own tip, no rebase needed this cycle.
- **Commit SHA:** (this cycle's own commit, see push log)
- **Files touched:**
  - `scripts/shape_ledger.py` — `build_corpus_index`'s and `build_corpus_key_index`'s index key
    gains `kind` (both now keyed `(book, kind, ...)`, not `(book, ...)`); `classify_unit`'s primary
    join key gains the unit's own `kind`; new `normalize_kind_dir()` helper strips a trailing
    `_generic` suffix so the deliberate `<kind>_generic` sibling-directory convention
    (`ingest_generic_kind.py`/`ingest_race_trait_generic.py`) still counts as a real answer for its
    base kind — the ONLY normalization applied, so a genuinely different kind never matches.
  - `scripts/tests/test_shape_ledger.py` — 3 new tests (the kind-blind collision reproduction at
    both `build_corpus_index` and `classify_unit` level, and the `_generic`-sibling-still-matches
    proof); 6 existing tests' fixture key literals updated from 3-tuple to 4-tuple (no behavior
    change, only the shape their own synthetic indexes must now carry).
  - `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-1-shape-closure/{ledger.json,family-vocabulary.json,family-vocabulary.md}`
    — regenerated against the live population, real commands (§4/§5 below).
  - `data/corpus/{advanced_players_guide,inner_sea_gods,inner_sea_races,ultimate_campaign,ultimate_psionics}/trait_generic/*.json`
    — 487 new files, real `python3 scripts/ingest_generic_kind.py --kind trait` output (§6).
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 5 entry prepended.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (`git diff --unified=0 HEAD -- scripts/shape_ledger.py scripts/tests/test_shape_ledger.py`, the correctly-scoped own-diff form).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope).
- **PI audit:** zero blacklist-term hits introduced in the source diff (`normalized_term_hit` swept over every added line, 0 hits) and zero *confirmed* hits in the 487 new `trait_generic` records (`python3 scripts/pi_key_rawtokens_audit.py --kind trait_generic` → `confirmed_records=0`; 487 `candidate_unratified_vocabulary` hits are the tool's own documented proper-noun-shaped heuristic, never actionable by itself, and 131 of the 487 records ship under a Codex-neutral name via `ingest_generic_kind.py`'s existing §24 handling — not a finding this cycle, the reference implementation already proven for `ability`).
- **Acceptance criterion:** dispatch brief, verbatim — "`shape_ledger.py`'s join is KIND-BLIND. Fix it. This is a repo-wide instrument defect," plus the required true-blast-radius measurement, the kind-aware fix propagated to all four production callers, honest (not repinned) Gate 3 reporting, and running `ingest_generic_kind.py --kind trait` if the cycle had room.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`; bootstrapped fresh this worktree via `scripts/fetch-pcgen-oracle.sh`, confirmed via `scripts/verify.sh --only preflight-oracle` → PASS).
- **Status:** `complete` — join fixed and kind-aware, propagated to all four callers with no per-caller code changes needed (§3), true blast radius measured and reported (§2), Gate 3 re-derived honestly with no budget-constant edit (§7), AND `ingest_generic_kind.py --kind trait` run for real (§6) — the `kind: trait` epic's own formal-write blocker (`epic-6-kind-trait_cycle-2_cycle_receipt.md`'s "Next-cycle plan" option (a)) is now cleared.
- **Notes:** see full account below.
- **Discovery forwards:** none new this cycle (this cycle *resolves* the discovery forward `epic-6-kind-trait_cycle-2` logged: `shape-ledger-kind-blind-join-hides-trait-population`, `docs/retro/events/t9-onboarding.jsonl` id `1787508202265-t9-onboarding-b6d30e`). One self-correction logged against this cycle's own first-pass measurement (§2): `scripts/retro.py correction`, `docs/retro/events/sd31-transcribe.jsonl` (RETRO_ACTOR env var did not propagate in this worktree's shell; logged actor is the fallback default, not a misattribution of the correction's content).
- **Next-cycle plan:** none required for this card. Row 16 (`kind: trait`, a sibling lane's territory) can now be re-evaluated against real `kind: trait` content instead of the `ability/`-fallback read — that re-evaluation is that lane's own call, not made here.

## 0. Re-derivation of the brief's own figures (`decisions.md §17a`)

The brief's own headline number ("487 `kind: trait` census units," "the trait epic found 487 for one
pair by accident") re-confirmed unchanged: `docs/work-inventory.json` still carries exactly 487
`kind: trait` not-done units (`python3 -c "... kind=='trait' ..."`, matches
`epic-6-kind-trait_cycle-2_cycle_receipt.md §0`'s own re-derivation). The brief's claim that
`shape_ledger.py` has **four** production callers was independently re-confirmed by grep
(`shape_coverage_standing_gate.py`, `family_vocabulary_reconcile.py`, `card15_reconcile.py`, and the
CLI's own `main()`) before any code was touched — all four call only `build_corpus_index` /
`build_corpus_key_index` / `build_ledger`, never construct a key literal themselves, so the fix
required zero caller-side edits.

## 1. The join, read directly

`build_corpus_index` keyed its index `(book, basename, source_line)` — no `kind` component at all.
`data/corpus/inner_sea_races/ability/loner_of_the_rocks.json` (a real, pre-existing record, `kind:
ability`, `TYPE:Trait.RaceTrait.Oread Race Trait`) sits at the identical `(inner_sea_races,
isr_abilities.lst, 78)` coordinate a `kind: trait` census unit cites — before this fix, the join
returned that record for the `trait` unit's query, reporting it `matched`/`no_formula_tokens`, never
`no_record`. Exactly the collision `epic-6-kind-trait_cycle-2`'s receipt named.

## 2. True blast radius — measured, not assumed, and self-corrected once

**First pass (over-counted, corrected before it left this cycle):** a naive re-derivation — build a
`(book, source_file, source_line) -> kind` index from the real corpus, cross-reference every not-done
unit's own `kind` against it, count mismatches — found **4,091** units across 8 kind-pairs. That
number was wrong. Reading `scripts/ingest_race_trait_generic.py`'s own docstring before trusting the
measurement (`§17a`) surfaced the reason: `race_trait_generic`, `feat_generic`, `race_generic`,
`monster_generic`, and `class_generic` are **deliberate sibling directories**
(`ingest_generic_kind.py`/`ingest_race_trait_generic.py`'s own stated design), and their entire
mechanism *depends on* the old kind-blind join treating them as equivalent to their base kind — "a
sibling directory is exactly as measurable for Gate-1 purposes" is those scripts' own words, not an
accident. Counting those five pairs as collisions would have been wrong on two counts: it would have
overstated the true defect by 2.7×, and — had the naive fix (bare kind equality, no `_generic`
normalization) shipped — it would have *broken* those two scripts' own already-landed, already-tested
design.

**Re-measured, excluding the five intentional sibling pairs** (`python3
<repo-scratch>/blast_radius2.py`, logic: walk the real corpus building `(book, source_file,
source_line) -> kind` with a trailing `_generic` suffix stripped before comparison, cross-reference
against every not-done unit's own `kind`, excluding this cycle's own new `trait_generic/` writes so
the measurement reflects the defect as it stood *before* this cycle's fix+write):

```
not-done population: 34,631
units whose kind-blind join answers with a genuinely DIFFERENT kind's record: 1,511

  equipment_modifier  -> equipment            999
  trait               -> ability              487
  class_feature       -> race_trait (via its race_trait_generic sibling)  25
```

This is the true blast radius the brief asked for. The self-correction (4,091 → 1,511, with the
reasoning for the 5 excluded pairs) is logged: `scripts/retro.py correction`,
`docs/retro/events/sd31-transcribe.jsonl`.

## 3. The fix — kind-aware, with the sibling convention preserved by construction

`normalize_kind_dir(raw_kind)` strips a trailing `_generic` suffix (only that suffix, nothing else).
Applied at the point both `build_corpus_index` and `build_corpus_key_index` derive `kind` from a
record's own directory (`rel.split(os.sep)[0]`), so `race_trait_generic/`'s records index under key
`race_trait`, matching a `race_trait` unit's own `kind` field exactly as they did before (by accident)
— while `ability/`'s records still index under `ability`, never matching a `trait` unit.
`classify_unit`'s primary join key gains the unit's own `kind` (`(book, kind, basename, line)`,
was `(book, basename, line)`); its `key_index`-fallback lookup already carried `kind` (the
citation-redirect fix, `978d215227`) and needed no change beyond the same `normalize_kind_dir` at its
own index-build site, for consistency.

**Four production callers, checked, not assumed:** `shape_coverage_standing_gate.py`,
`family_vocabulary_reconcile.py`, `card15_reconcile.py`, and `shape_ledger.py`'s own CLI all call
`build_corpus_index`/`build_corpus_key_index`/`build_ledger` generically and construct no key
literals of their own (confirmed by grep before editing) — the fix propagates to all four with zero
caller-side edits, so the standing gate's own `no_record` figure cannot silently disagree with the
CLI's, the same discipline the citation-redirect fix (`978d215227`) established. Ran all three:
`shape_coverage_standing_gate.py` (§5), `family_vocabulary_reconcile.py` (§4), `card15_reconcile.py`
(ran to completion against the real `PCGEN_CORPUS_ROOT`-backed census, no crash, no hardcoded key
shape found in its own source).

**RED → GREEN, both proved:** (1) the exact real-world collision, reproduced synthetically at both
`build_corpus_index` and `classify_unit` level (a `trait` unit's join answering with an `ability`
record at the identical coordinate) — failed before the fix (`AssertionError: no_record != matched`
shape), passed after. (2) the `_generic`-sibling-still-matches guarantee — reproduced synthetically
(a `trait_generic/` record answering a `trait` unit), failed before `normalize_kind_dir` existed
(`KeyError`-shaped: `('b','trait',...)` not in index, only `('b','trait_generic',...)` was), passed
after. 6 pre-existing tests' fixture key literals updated 3-tuple → 4-tuple (their own synthetic
indexes now must carry the shape the real index carries; no assertion's *meaning* changed).
`python3 -m unittest scripts.tests.test_shape_ledger` → **41/41 GREEN** (3 new, 6 updated, 32
unchanged).

## 4. Full test-suite sweep, scoped

`python3 -m unittest discover -s scripts/tests -p "test_*.py"` → **547 passed, 5 errors, 2 skipped**
(552 total). The 5 errors are all in `test_transcribe_monster_tables.py`
(`InternalBundleAbilityHopIsResolved`/`UnscreenableRowIsDroppedNotFatal`), a PI-blacklist-loading
defect in the `monster_ability` lane's own territory (explicitly out of scope per this dispatch's
Territory section) — traced by direct read of the traceback to a `pi_blacklist_terms()` call
unrelated to any file this cycle touched, not caused by this diff. Every `shape_ledger`/`family_
vocabulary_reconcile`/`shape_coverage_standing_gate` test: green (61 tests across those three files
alone).

## 5. Gate 3 — honest number, budget constants untouched

`python3 scripts/shape_coverage_standing_gate.py --inventory docs/work-inventory.json`:

```
population: 34,631   unclassified: 0
matched=11,389 (32.9%)  no_formula_tokens=21,991 (63.5%)  no_record=1,251 (3.6%)
no_record budget: 1,251/34,631 vs. baseline 21,521/36,028 -- exceeded: False
```

`NO_RECORD_BUDGET_COUNT`/`NO_RECORD_BUDGET_POPULATION` in `shape_coverage_standing_gate.py`: **not
edited this cycle.** The honest post-fix `no_record` (1,251) is well inside the existing,
unrepinned budget — the gate passes on its own merit, not because it was widened to accommodate the
fix (`§1a`).

## 6. `ingest_generic_kind.py --kind trait` — run for real, the epic's own blocker cleared

With the join fixed, re-generated the ledger (`python3 scripts/shape_ledger.py --inventory
docs/work-inventory.json --output artifacts/gate-1-shape-closure/ledger.json`) and ran the ingest for
real against the pinned oracle (`git status --porcelain` confirmed clean before running, per this
bundle's own "check before every mutation" discipline):

```
python3 scripts/ingest_generic_kind.py --kind trait --ledger <ledger.json> --out <report.json>
kind=trait  population=487  written=487  name_pi_renamed=131
written_by_book: advanced_players_guide 90, inner_sea_gods 115, inner_sea_races 96,
                 ultimate_campaign 154, ultimate_psionics 32
```

`git status --porcelain` after: exactly 5 new `trait_generic/` directories, 487 new files, **zero**
modifications, **zero** deletions (`find data/corpus -path "*/trait_generic/*.json" | wc -l` → 487,
exact match to population). `corpus_literal_sweep --json-out` (whole-repo): 47,038 records examined,
**8 findings across 7 records** — all 7 are pre-existing `class_feature` PI-redaction mismatches in
books this cycle never touched (`advanced_players_guide`, `advanced_race_guide`, `horror_adventures`,
`inner_sea_combat`, `ultimate_wilderness` — none under `trait_generic/`), confirmed by direct path
inspection, not this cycle's regression. `cargo test --locked --lib trait_pool` (7/7) and `cargo test
--locked race_trait_picker` (19/19, `apps/desktop/src-tauri`) both still green against the new
on-disk content — `trait_pool.rs`'s existing loader (built in `epic-6-kind-trait_cycle-2`, reads
`trait_generic/` preferentially over its `ability/` fallback on key collision) now has real
`trait_generic/` content to prefer; that lane's own re-evaluation of row 16 against it is that lane's
call, not this cycle's.

## 7. What moved, and why — closure / reclassification / reachability / instrument correction, kept separate (`§16`)

**Closure — 487 units, kind `trait`:** written for real under a new `kind: trait` schema
(`trait_generic/`) via the existing generic-ingest mechanism, satisfying `decisions.md §25`'s "close
by real ingest" half this cycle's own file grant could reach. These units never surface as `no_record`
in the post-fix ledger (§5) — the fix and the write landed in the same cycle, so the honest
intermediate state (were the fix alone landed without the write) is reported for the record: they
would have moved `matched`/`no_formula_tokens` (masked, wrong-kind) → `no_record` (honest, un-ingested)
→ `matched`/`no_formula_tokens` (honest, real-kind) once written. `formula` family split for the 487:
re-derivable via `jq` over `ledger.json`'s rows filtered `kind=="trait"` — not reproduced here to keep
this receipt to the movement claim, not a second family-vocabulary table.

**Reclassification — 0 units.** No unit changed its own `kind` field this cycle.

**Reachability — unaffected by this instrument fix directly.** `epic-6-kind-trait_cycle-2`'s
`trait_pool`/`race_trait_picker` machinery already reached 13 of the 14 `adopted_race_choose_selector`
units through its `ability/` fallback before this cycle; that machinery is unchanged here. The 487
newly-written `trait_generic` records are a **different** population (the corpus-wide `kind: trait`
enumeration, not the 14-unit `adopted_race_choose_selector` subset) and this cycle made no
player-reachability claim for them beyond Gate-1 measurability (`ingest_generic_kind.py`'s own
documented posture, §"Why a sibling directory" in that script).

**Instrument correction — 1,024 units, two kinds, GENUINELY NEWLY `no_record`:**
`equipment_modifier` 4 → 1,003 (**+999**) and `class_feature` 0 → 25 (**+25**). These units were
previously reporting `matched`/`no_formula_tokens` against a WRONG kind's record (`equipment` and
`race_trait_generic` respectively) — the join now tells the truth about them. **This is not a
regression; it is the instrument correctly reporting a population that was never actually
ingested under its own kind.** Neither population is written or closed this cycle — named here, not
fabricated shut (`§1a`). `equipment_modifier`'s 999 and `class_feature`'s 25 are real, un-ingested
`no_record` work for a future cycle in those kinds' own territory (`equipment_modifier`/`class_feature`
stragglers are named in this dispatch's Territory section as a sibling lane's scope, not this one's).

**Reconciliation, exact:** total `no_record` 227 → 1,251 = **+1,024**, which equals exactly
999 (`equipment_modifier`) + 25 (`class_feature`) — the `trait` pair's 487 units do NOT appear in this
delta because they were closed (§ above), not merely corrected. `matched` 11,425 → 11,389 (−36) and
`no_formula_tokens` 22,979 → 21,991 (−988) sum to −1,024, the mirror image of the same movement.

## 8. PI discipline (`decisions.md §15`/`§19`/`§24`)

See header. `ingest_generic_kind.py`'s own §24 machinery (reused verbatim, not re-derived) renamed 131
of the 487 records to a Codex-generated neutral name; the run's own JSON report names each by
coordinate and reason (`name_pi_blocked`), never by the original name, per `§24b`-4. No blacklist term
appears in this receipt, any test name, or any commit message this cycle produced (grepped the diff
before this write, per the dispatch's own standing instruction).

## 9. Rebase discipline

No rebase needed — this cycle started at `origin/tranche/12`'s own tip and pushed before any
concurrent lane advanced it (§5's fetch immediately before push, in the push log).
