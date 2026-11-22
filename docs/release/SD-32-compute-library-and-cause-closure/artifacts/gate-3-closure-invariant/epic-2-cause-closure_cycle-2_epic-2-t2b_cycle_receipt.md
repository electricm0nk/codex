# Cycle — Epic 2 (cause closure) / Card 11 `epic-2-cause-closure`, lane T2b

- **Card ID:** `epic-2-cause-closure` (shared row 11; this lane's own scope: shape T2b only)
- **Actor:** `epic-2-t2b`
- **Base:** `08c9da3d3a53c6f4e18f1dd6c7a208f826dc39b2` (self-heal not needed; worktree started clean on
  the pin after `git reset --hard "$PIN" && git rebase origin/tranche/12`)
- **Files touched:** `src/bin/v06_work_inventory.rs` (one new helper,
  `ingested_race_trait_source_coordinates`, and one new standing regression test,
  `race_trait_grounding_tests::the_t2b_residual_population_is_never_ingested_not_a_matcher_miss`),
  `docs/retro/events/epic-2-t2b.jsonl` (new — 1 correction),
  `docs/release/SD-32-compute-library-and-cause-closure/progress.md` (this entry),
  `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (card 11 status, lane note).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  (`git diff --unified=0 1bb523773d32705d1b7387fd4c494861523f55ba...HEAD -- src/bin/v06_work_inventory.rs`
  — no `sd[0-9]+_`/`SD[0-9]+_`/`Sd[0-9]+`/`t_[0-9a-f]{8,}` matches).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff — no
  `STUB`/`MOCK`/`placeholder`/`not yet implemented`/`todo`/`fixme`/`hack` tokens).
- **Acceptance criterion:** `acceptance-and-verification.md` AT-32-E2-001 — cause closure by class,
  not by instance, for T2b (race-trait compound-key matcher, ~2,472 units per `epic-breakdown.md`).
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
  `PCGEN_ORACLE_SHA`) — read from the repo-local slot after self-heal fetch
  (`scripts/fetch-pcgen-oracle.sh --dest <repo-local pcgen slot>`; empty slot on this fresh
  worktree, PASS after fetch, re-verified `scripts/verify.sh --only preflight-oracle`).

## Status: closed as a measurement/re-scoping cycle — 0 units banked, cause proven not operative

**T2b's named cause does not explain any unit of its current population.** Full findings below.

### 1. Re-derived population

`docs/work-inventory.json` is committed at HEAD (`git log -1 -- docs/work-inventory.json` →
`7a40154b10a54549969e849d1bbc1fac6ce8edab`, 2026-08-21, clean working tree — no regen needed to
re-derive; the file already reflects the pinned corpus). Command:

```
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
u = [x for x in d['units'] if x.get('kind')=='race_trait' and x.get('evidence')=='race_trait_race_not_modelled']
print(len(u))
"
```
→ **2,472**, matching `epic-breakdown.md`'s T2b row and card 11's cycle-1 receipt exactly. No
correction to the population size itself.

### 2. Cause investigation — the named cause is stale

`epic-breakdown.md`'s T2b row and `SD-31-corpus-closure-grind/todo/sweeps.md` S3 name
`modelled_race_of_race_trait()`'s compound-key matching (the leading-segment-only, exact-match
comparison) as the cause: `"Elf Shaman Hex Range Choice ~ Chant"` reports `race_trait_race_not_
modelled` though a race is named, because the old matcher required the FULL leading segment to
equal a bare race name.

That defect was already fixed, twice, before this cycle:

- **Wave 20** widened the per-segment test to a word-boundary-anchored PREFIX match — the exact
  `"Elf Shaman Hex Range Choice ~ Chant"` example S3 names is the function's own doc comment's
  worked example, and is pinned by
  `race_trait_grounding_tests::a_modelled_race_leading_a_compound_segment_with_trailing_descriptive_words_is_found`
  (pre-existing test, unmodified this cycle).
- **SD-29 `decisions.md §43.5`** added a PRIMARY probe (`probe_race_trait_corpus`,
  `EngineFacts::race_trait_engine_book`) that resolves race attribution from the REAL race corpus
  the app loads (18–25 races, not CRB's 7) via source-coordinate join, for every record that IS
  ingested. This probe runs BEFORE the CRB-table matcher and, per its own doc comment, "overrules
  the CRB-table rule below rather than supplementing it." The CRB matcher is now a narrow secondary
  fallback that only ever matters for a record the PRIMARY probe already failed to place.

**Empirical re-check, this cycle:** re-implemented the current matcher logic in Python (prefix
match + hyphen/space normalization + the `Adopted Race` trailing-segment exception, faithfully
transcribed from the current `modelled_race_of_race_trait` source) and ran it against all 2,472
residual keys' leading segments. **Zero** match a CRB race. The matcher, as it stands today, is not
being asked a question any of the 2,472 units could answer differently.

**The real cause, re-derived by cross-referencing provenance:** every `data/corpus/*/race_trait/
**/*.json` record carries its own `source.path`/`source.line` — the exact `.lst` file and line it
was ingested from. Built the set of every currently-ingested race_trait record's `(basename(source.
path), source.line)` coordinate (794 records, all books) and intersected it against the 2,472
residual units' own `(source_file, source_line)` coordinates (the SAME join key
`EngineFacts::race_trait_engine_book` uses):

```
python3 -c "
import json, glob, os
ingested = set()
for f in glob.glob('data/corpus/*/race_trait/**/*.json', recursive=True):
    d = json.load(open(f))
    p = d.get('source', {}).get('path'); l = d.get('source', {}).get('line')
    if p: ingested.add((os.path.basename(p), l))
wd = json.load(open('docs/work-inventory.json'))
u = [x for x in wd['units'] if x.get('kind')=='race_trait' and x.get('evidence')=='race_trait_race_not_modelled']
print(len(ingested), sum(1 for x in u if (x.get('source_file'), x.get('source_line')) in ingested))
"
```
→ `794 0` — **zero overlap.** Not one of the 2,472 residual units was EVER ingested into
`data/corpus` at all. `modelled_race_of_race_trait` is called from `classify`'s `Kind::RaceTrait`
arm on a `CorpusUnit` built from a real `data/corpus` record — it only ever runs against something
already ingested. If a coordinate was never ingested, the matcher never gets a chance to misjudge
it. **The matcher is not the operative gate for any of the 2,472 units.**

### 3. What the 2,472 units actually are

Breaking the population down by cause (all commands re-derivable from
`docs/work-inventory.json` + `apps/desktop/src-tauri/src/race_catalog.rs`):

- **1,754 of 2,472 (71%)** are in books never registered in `race_catalog.rs`'s
  `RACE_CORPUS_BOOKS` list at all (`bestiary_3`, `bestiary_4`, `ultimate_psionics`,
  `pathfinder_unchained`, `mythic_adventures`, `occult_adventures`, `ultimate_wilderness`,
  `inner_sea_world_guide`, `inner_sea_gods`, `ultimate_combat`, `ultimate_intrigue`,
  `ultimate_magic`, `book_of_the_damned_volume_1/2`). No `.lst` row from these books is scanned by
  `load_race_corpus` at all — a **book-onboarding-for-race-content** gap, the same shape as T9
  (`epic-breakdown.md`: "per-record onboarding backlog in registered books"), except these books
  are not even registered for race content specifically.
- **718 of 2,472** are in books that ARE in `RACE_CORPUS_BOOKS` (`core_rulebook`, `beastiary`,
  `advanced_race_guide`, `advanced_players_guide`, `monster_codex`, `inner_sea_races`,
  `horror_adventures`, `bestiary_2`, `bestiary_5`, `bestiary_6`) but were never transcribed from
  the pinned oracle's raw `.lst` rows into `data/corpus` JSON at all. Two sub-shapes sampled and
  hand-verified:
  - Category-header rows with no race in the key at all (`Racial SLA ~ <ability>`,
    `Unchained Evolution ~ <evolution>`, `Favored Class Bonus ~ <type>`, `Race Subtype ~
    <creature subtype>`) — correctly excluded by the matcher's design (no race is named), not a
    defect. ~350+ units of the 718.
  - `"Adopted Race ~ <RaceName>"` selector rows naming a REAL, chassis-modelled non-CRB race
    (sampled: Fetchling, Grippli, Ifrit, Oread, Sylph, Undine, Dhampir — all in-scope races per
    `ingest_races.rs`'s `IN_SCOPE_RACES`) that `ingest_races.rs`'s flat standard-trait ingest loop
    never captures — a selector-mechanism ingestion gap in the RAW ingest tool, not in the
    matcher or the probe.

None of these are closeable by "fixing the matcher at the cause" — there is no single code change
in `modelled_race_of_race_trait` (or its callers) that would move any of the 2,472 units, because
none of them ever reach that function with ingested data behind them. Closing this population for
real means ingesting ~2,472 more records across ~20 books (new `RACE_CORPUS_BOOKS` registrations
plus new/extended ingest tooling for the selector-mechanism gap) — a multi-thousand-unit content
project structurally identical in shape to T9's own separately-measured 2,651-unit backlog, not a
narrow matcher fix, and far beyond this lane's reasonable one-cycle scope. Fixture-checking a
partial subset (e.g., just the ~44 `Adopted Race` rows) would violate `decisions.md §1a/§3`'s
anti-gaming bar — it would be an easy-subset instance-close, not a class-close, and could not prove
coverage of the other 2,428 units under any single cause.

### 4. Standing regression added, RED→GREEN proved

Added `ingested_race_trait_source_coordinates` (walks every `data/corpus/*/race_trait/**/*.json`
record's own provenance, not just books in `RACE_CORPUS_BOOKS` — so a book the app's loader
doesn't read yet still can't hide a false "never ingested" claim) and
`race_trait_grounding_tests::the_t2b_residual_population_is_never_ingested_not_a_matcher_miss`,
pinning three hand-verified samples (one per un-ingested cause named above:
`acg_abilities_race.lst:9` "Arcanist Exploit ~ Arcane Barrier", `arg_abilities_race.lst:2204`
"Fins to Feet", `fetchling_abilities_race.lst:32` "Adopted Race ~ Fetchling") as never present in
the ingested-coordinate set.

RED→GREEN: inverted the assertion (`ingested.contains` instead of `!ingested.contains`), re-ran —
failed for the intended reason (`acg_abilities_race.lst:9 must NOT be present in data/corpus/*/
race_trait -- if it now is, ...`), confirming the assertion is load-bearing, not decorative.
Reverted; re-ran green. Command: `CARGO_TARGET_DIR=... cargo test --locked --bin v06_work_inventory
the_t2b_residual_population_is_never_ingested_not_a_matcher_miss`.
No regression: `race_trait_grounding_tests` module (29 tests, up from 28) passes in full.

**Pre-existing, out-of-scope failure discovered (not caused by this cycle's diff, not fixed by
it):** `rule_set_mapping_tests::uncompiled_books_stay_none` fails on this same base commit
(`left: Some(InnerSeaTemples), right: None`) — `inner_sea_temples` has apparently been compiled by
a concurrent Epic 4/Gate 0 lane without this test being updated. Confirmed unrelated: `git diff
--stat` for this cycle touches only `src/bin/v06_work_inventory.rs` (the T2b addition) and two
`docs/retro/events/*.jsonl` files; `rule_set_mapping_tests` is untouched. Left as-is — Epic 4's
territory, not T2b's; reported here so it isn't mistaken for a regression this cycle introduced.

### 5. Discovery forward

Logged `scripts/retro.py correction` (`docs/retro/events/epic-2-t2b.jsonl`) against
`epic-breakdown.md`'s T2b row and card 11's cycle-1 receipt: the "compound-key matcher" causal
framing is stale (fixed by SD-31 wave 20 + SD-29 §43.5) and does not explain the current 2,472-unit
population, which is a raw-content ingestion gap requiring book-registration + ingest-tooling work,
not a matcher fix.

### 6. Ruling needed (per the dispatch brief's "if you truly cannot close a shape" clause)

**Population:** 2,472 (re-derived above, command given). **What blocks closing T2b as literally
named:** there is no code fix under "fix the compound-key matcher" that moves any unit — the named
defect is already fixed and the real, measured cause is a large multi-book content-ingestion gap.
**Precise ruling/precondition needed:** the operator (or a consolidation cycle) should either —
(a) accept T2b as CLOSED at 0 units-fixed on the grounds that its named cause is proven fixed and
fully non-operative corpus-wide (this receipt's own evidence), with the residual 2,472 explicitly
reclassified out of T2b into a new book-onboarding-shaped scope (naturally adjacent to T9, likely
warranting its own card rather than folding into T9's already-measured 2,651), or (b) authorize a
dedicated multi-cycle ingestion effort under card 11/T2b's own name sized like T9's, if the
population must literally close under this card. Not filed under `progress.md`'s `## Open
blockers` — per `decisions.md §10` that is a request for an operator ruling, not a disposition, and
this receipt already states the finding and the two concrete options directly.

### 7. Next-cycle plan

If ruling (a): T2b closes here; no further T2b-lane cycle needed. If ruling (b), or if a
consolidation cycle instead expands T9/opens a new card: the next cycle should (1) register the
13 currently-unregistered books in `RACE_CORPUS_BOOKS`, verify each has a race chassis worth
loading (some, like `book_of_the_damned`, may have near-zero real race content — check before
assuming the full 1,754 is real work), then (2) extend `ingest_races.rs`/`ingest_race_traits.rs`
to capture the `Adopted Race ~ <non-CRB race>` selector mechanism and any other systematic
un-ingested shape found along the way, fixture-checking every emitted record against the pinned
oracle per `decisions.md §3`.
