# Cycle row19-cycle2 — Epic 9 (`epic-9-desktop-reach-and-catalog-reds`) / Row 19

- **Card ID:** `epic-9-desktop-reach-and-catalog-reds`
- **Commit SHA:** (see push log in the terminal receipt this file accompanies)
- **Files touched:**
  - `apps/desktop/src-tauri/src/reach_gate.rs`
  - `apps/desktop/src-tauri/src/companion_catalog.rs`
  - `src/bin/gen_book_cache.rs`
  - `data/corpus/bestiary_5/companion/familiar_brain_mole.json` (regenerated)
  - `data/corpus/bestiary_5/companion/familiar_chuspiki.json` (regenerated)
  - `data/corpus/bestiary_5/LICENSE.json` (append-only note from the regen)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **PI scrub:** `pi_scrub.normalized_term_hits()` on the full diff — zero hits.
- **Acceptance criterion:** kanban row 19 — close as many of the 7 named
  `apps/desktop/src-tauri` reds cycle 1 left as genuine evidence supports;
  set `complete` only when the whole desktop workspace is green.
- **Corpus SHA:** `scripts/pcgen-oracle-pin.env` (oracle bootstrapped fresh
  this cycle via `scripts/fetch-pcgen-oracle.sh --dest`, confirmed populated).
- **Status:** `in-progress` (NOT `complete` — 5 of 7 reds cycle 1 left remain
  red, with evidence; desktop workspace is not green).
- **Notes:**

## Starting state (verified, not assumed)

Worktree started on a **stale lineage** (`git merge-base --is-ancestor $PIN
HEAD` failed against a `tranche/11` ancestor tip) — footgun 1, fired a fifth
time. Recovered via `git reset --hard $PIN`; `git rebase origin/tranche/12`
reported "up to date" (HEAD already equalled `origin/tranche/12` after the
reset). Re-verified before any edit.

Reproduced all 7 named reds first: `cd apps/desktop/src-tauri && cargo test
--locked --bin codex-desktop` (own `CARGO_TARGET_DIR`, `CARGO_INCREMENTAL=0`)
→ **512 passed, 7 failed**, exact names matching cycle 1's receipt.

## Closed this cycle (2 of 7) — evidence per test

### 1. `reach_gate::tests::every_ingested_companion_book_reaches_the_catalog_record_by_record` — real (non-pin) reachability fix

`bestiary_5`'s 2 `Familiar` companions (`familiar_brain_mole`,
`familiar_chuspiki`) carried a raw, un-slugged `data.key` in their corpus
JSON (`"Familiar (Brain Mole)"` instead of
`"bestiary_5:companion:familiar_brain_mole"`), traced by cycle 1 to
`docs/work-inventory.json`'s own `corpus_key` field for these units. Deeper
root cause found this cycle: `corpus_key` carries the raw display name for
**every** bestiary_5 companion creature record, not only these two — the
real defect is that these two files were written by the **wrong generator**.
Every other bestiary_5 companion creature record (`familiar_aether_wysp.json`
and 33 siblings) was written by `gen_book_cache.rs`'s `gen_companion_book`
(`cargo run --bin gen_book_cache -- companion:bestiary_5`), which reads the
transcribed `CompanionRecord` table and writes
`"key": format!("{book_id}:companion:{}", slugify(companion.key))` — the
correct, slugged format, plus a separate `data.corpus_key` field carrying the
raw name. These two files instead carried the flat shape
`scripts/ingest_companion.py` writes (`"key": key` verbatim, no
`data.corpus_key` field at all), with an `ingested_at` timestamp matching
cycle 1's own run — confirming `ingest_companion.py`, not the guarded
`gen_book_cache` generator, wrote them when cycle 1 emptied
`UNINGESTED_CAMPAIGN_GATES`.

**Fixed at the root, through the guarded generator, not by hand-editing
`data/corpus/**`:**
1. `gen_book_cache.rs`'s `bestiary_5` `CompanionBookSpec.races_lsts` did not
   name `b5_races_companion_oa.lst` (deliberately excluded by a stale
   comment reciting the same overturned `PRECAMPAIGN:1,Occult Adventures`
   premise cycle 1's fix already falsified). Added it, and rewrote the
   comment to record why.
2. Removed the two wrongly-shaped files (`rm`, plain filesystem delete —
   never `git rm -f` over guarded content, and confirmed via `git status
   --porcelain` that only these two paths were touched).
3. Ran `cargo run --locked --bin gen_book_cache -- companion:bestiary_5`.
   Its `if !path.exists()` guard let it write fresh records for exactly the
   two missing files (`bestiary_5 companion cache generated: 35 creatures,
   22 abilities; LICENSE.json records_processed=279`) — the guard means it
   is a no-op for every one of the other 55 existing records, verified by
   `git status --porcelain` afterward showing only the two target files plus
   `LICENSE.json` changed.
4. Verified both records now carry `data.key ==
   "bestiary_5:companion:familiar_brain_mole"` /
   `"bestiary_5:companion:familiar_chuspiki"` and a proper `data.corpus_key`
   field, matching the other 55 records' shape exactly.

**Stamp/provenance check (universal requirement):** `LICENSE.json`'s diff is
append-only (`git diff data/corpus/bestiary_5/LICENSE.json`) — a new `PASS`
note appended (`SD29-E7-F2-003`, `records_processed=279` unchanged, zero PI
hits, 57 records = 35 creatures + 22 abilities). No `--allow-stamp-loss` used;
no other book's record count moved.

Verified: `reach_gate::tests::every_ingested_companion_book_reaches_the_catalog_record_by_record`
→ green (`bestiary_5` no longer a failing case in the 7-book loop).

### 2. `reach_gate::tests::pathfinder_unchaineds_class_features_are_claimed_per_corpus_record` — instrument correction, partial-credit branch added

Confirmed by direct inspection: `data/corpus/pathfinder_unchained/class_feature/`
holds **604** files on disk (`find ... -name '*.json' | wc -l`), but only
**64** are owned by one of the four Unchained class tables this engine
compiles (`barbarian_features::features()` +
`monk_features::features()` + `rogue_features::UnchainedRogueFeature::ALL` +
`summoner_features::UnchainedSummonerFeature::ALL`, exactly the sum
`corpus_ingest_diagnostic.rs::pu_class_feature_count()` already computes).
The old test used `corpus_record_keys("pathfinder_unchained",
"class_feature")` — a directory walk — as BOTH the assertion's pinned count
AND `pu_class_features_reach()`'s denominator, conflating "ingested for
shape-closure" with "owned by a real class table". The other 540 records are
genuine `class_feature`-kind corpus content for classes this engine does not
model (Automatic Bonus Progression toggles, the Unchained skill-system
variant rules, Background Skills, Combat Trick/Skill Unlock pool entries),
per `corpus_ingest_diagnostic.rs`'s own comment (unchanged, cited not
re-derived).

**Fix:** added `pu_class_owned_feature_keys()` (union of the four tables'
own keys, read by key rather than by count) and pointed
`pu_class_features_reach()`'s `ingested` denominator at it instead of the
directory walk. Rewrote the test with a **partial-credit branch matching its
sibling tests' shape** (the `733`-record ARG/ISF-style test two cases
above in the same file, which separates a family's reaching subset from an
open remainder rather than collapsing to `NotSurfaced`): it now asserts
**both** numbers explicitly and by name — `on_disk.len() == 604` (the real,
re-derived disk population) and `class_owned.len() == 64` (the real class
table union) — plus `non_class_owned == 540` as a **named, counted
residual**, not a silently-dropped population. `reach_of` is then asserted
`Surfaced { records: 64 }` against the corrected denominator.

**Mutation-proves-RED (per the universal requirement):** temporarily changed
`class_owned.len(), 64` to `65`, re-ran — RED
(`left: 64 right: 65`, at `src/reach_gate.rs:6897`), confirming the assertion
is live and not vacuous. Reverted before commit.

Verified: `reach_gate::tests::pathfinder_unchaineds_class_features_are_claimed_per_corpus_record`
→ green, in isolation and in the full desktop sweep.

## Narrowed with evidence, not closed (companion_catalog's own test)

### `companion_catalog::tests::every_served_key_matches_a_corpus_record_file`

Confirmed by re-deriving the loop's per-book disagreement (a Python
cross-check against the transcribed `key:` slices and the on-disk file
lists, independent of the Rust test's own slug formula, `§17a`): this test
iterates **all 16** `COMPANION_BOOKS`, not only the 7 the sibling reach_gate
test checks. It failed on the first book in iteration order (`beastiary`)
before this cycle, hiding every later book's own disagreement behind that
one panic.

**Closed, with citations already present in each book's own
`companion_data.rs` header comment (the transcriber's own "NOT transcribed"
list — not invented here):**
- `beastiary` — all 28 of its `.COPY=`/`.MOD`/orphan/unmodelled rows named
  individually (22 Celestial/Fiendish creature-template `.COPY=` rows, 4
  Universal Monster Rule `.MOD` ability rows, 1 orphan ability with no
  owning creature, 1 owned-but-`ASPECT:`-only ability). Added
  `KNOWN_UNTRANSCRIBED_COMPANION_RECORDS`, a named `(book, slug)` exception
  list with a citation to the exact `companion_data.rs` header-comment lines
  for each entry, asserted present-on-disk (so a future removal makes the
  exception stale and fails loudly) and subtracted from `on_disk` before the
  equality check — the same evidenced-exception shape `reach_gate.rs`'s own
  `OPEN_FINDINGS` const already uses elsewhere in this file, not a new
  pattern.
- `bestiary_4` — its 2 named `.COPY=` ability rows (`Pooka ~ Change Shape`,
  `Psychopomp (Nosoi) ~ Change Shape`), same shape, same citation discipline.

**Genuinely NOT closed — re-scoped, quantified fresh, and this is now the
largest single item for the next cycle:** with `beastiary` and `bestiary_4`
accounted for, the loop reaches 4 more books that disagree, at a scale far
past what a documented per-record exception list can honestly cover in one
cycle:

| book | on-disk `companion/` records | missing from the transcribed table |
|---|---:|---:|
| `ultimate_wilderness` | 575 | 248 |
| `ultimate_magic` | 198 | 139 |
| `advanced_race_guide` | 32 | 18 |
| `book_of_the_damned_volume_1` | 31 | 29 |

Sampled records confirm these are the shape cycle 1 named for item 4
(hundreds of "Evolution ~ …" eidolon-evolution records with no picker UI) —
`ultimate_wilderness/companion/aid.json`'s `data.corpus_key` is `"Animal
Trick ~ Aid"`, `origin: "declared"`, `owners: []`: a real, ingested,
shared-reference-library "Animal Trick" record no companion row of that book
claims, the identical shape `reach_gate.rs`'s existing `OPEN_FINDINGS`
entries already document for `monster_ability` (`beastiary1`, `bestiary_2`,
`bestiary_3`, `bestiary_4`, `horror_adventures`) — but for `companion` this
family has never been named before. **434 records across 4 books**, none of
them yet individually verified as "genuinely no consumer" vs. "a real defect
this cycle would be gaming by exception-listing blind" — writing that many
per-record findings without individually checking each would itself violate
`§17a`/`§1a`. Left red and named here, not gamed.

## Untouched, re-scoped for the next cycle (item 4 — the 4 remaining `reach_gate` tests)

`reach_gate::tests::every_declared_claim_actually_carries_the_records`,
`every_ingested_family_is_accounted_for`,
`unreached_records_are_exactly_the_recorded_findings`,
`unsurfaced_families_are_exactly_the_recorded_findings` are still red.
Re-derived their true scope this cycle (not carried forward from cycle 1's
"~38" estimate, per `§17a`):

- `every_ingested_family_is_accounted_for` currently names **~170**
  `(book, kind)` families with no declared `reach_of` claim and no
  `OPEN_FINDINGS` entry — the 12 newly-classified corpus kinds cycle 1's
  classifier fix surfaced (`abilities`, `domains`, `templates`, `languages`,
  `skills`, `deities`, `generic_feats`, `race_variants`, `class_variants`,
  `monster_variants`, `named_traits`, `powers`), each recurring across most
  of the ~30 books that carry it. This is larger than cycle 1's own "~38"
  estimate because that count predated the classifier fix landing across the
  full corpus.
- `unreached_records_are_exactly_the_recorded_findings` names (at minimum)
  18 individual `advanced_race_guide/companions` eidolon-evolution and
  Shaitan-binder-eidolon records with no `UNREACHED_RECORD_FINDINGS` entry —
  one sample of what is very likely hundreds once every affected book is
  walked (matching the `companion_catalog` finding above).

Closing these honestly needs per-family and (for the individual-record
tests) per-record evidence at a volume this cycle's remaining budget cannot
produce without violating `§17a`'s standard against fabricating volume under
time pressure — the same judgment cycle 1 made for the beastiary1 28, now
confirmed to generalize corpus-wide. **This is a hard-impossibility-adjacent
sizing question, not a scope cycle 1 or this cycle invented an exemption
for** (`§27b`: novelty/no-consumer is grounds for sizing, not exclusion) —
escalated by coordinate here for the next cycle, which should very likely
start with the eidolon-evolution picker UI (the single largest repeating
cause across both the `companion_catalog` and `reach_gate` findings above)
before attempting to enumerate the ~170 families individually.

## Full-sweep re-run

- `apps/desktop/src-tauri`: `cargo test --locked --bin codex-desktop` →
  **514 passed, 5 failed** (list above), down from 512 passed / 7 failed at
  the start of this cycle. Full failing list:
  `companion_catalog::tests::every_served_key_matches_a_corpus_record_file`,
  `reach_gate::tests::every_declared_claim_actually_carries_the_records`,
  `reach_gate::tests::every_ingested_family_is_accounted_for`,
  `reach_gate::tests::unreached_records_are_exactly_the_recorded_findings`,
  `reach_gate::tests::unsurfaced_families_are_exactly_the_recorded_findings`.
- Root workspace: `cargo test --locked --lib bestiary_5` (scoped per §13) →
  see terminal receipt for pass/fail count, run in parallel with the desktop
  sweep this cycle.

## Territory

Confirmed clean before every commit: `git status --porcelain` touched only
`apps/desktop/src-tauri/src/reach_gate.rs`,
`apps/desktop/src-tauri/src/companion_catalog.rs`, `src/bin/gen_book_cache.rs`,
and the two regenerated bestiary_5 companion JSON files plus its
`LICENSE.json` — none overlapping row 18's pool-magnitude files
(`pilot_compute/mod.rs`, `class_feature_pool_catalog.rs`,
`class_feature_grant_consumer.rs`) or the PI lane's `bestiary_4/monster_ability`
metadata work. Rebased on `origin/tranche/12` immediately before push and
re-ran the targeted tests after.

## Next-cycle plan

1. **Build the eidolon-evolution picker UI.** This is the single largest
   repeating root cause named above — it likely closes the bulk of both the
   `companion_catalog` book-level gaps (`ultimate_wilderness`,
   `ultimate_magic`, `advanced_race_guide`, `book_of_the_damned_volume_1`)
   and a large share of `unreached_records_are_exactly_the_recorded_findings`'s
   individual-record findings, in one mechanism rather than per-family shims.
2. **Batch the ~170 `every_ingested_family_is_accounted_for` families by
   kind, not by book.** 12 recurring corpus kinds (`abilities`, `domains`,
   `templates`, `languages`, `skills`, `deities`, `generic_feats`,
   `race_variants`, `class_variants`, `monster_variants`, `named_traits`,
   `powers`) account for nearly all ~170 rows; each kind is very likely one
   root cause (no catalog surface exists) repeated across every book that
   carries it, matching this bundle's own `§17`/`§16` precedent for
   `monster_ability`'s shared-reference-library shape.
3. **`beastiary1`'s `.COPY=`/`.MOD` creature-template delta rows remain
   genuinely unresolved** (22 of the 28 named exceptions this cycle
   documented rather than closed) — a real Celestial/Fiendish creature
   template application engine, sized as its own epic per cycle 1's original
   assessment, still stands as a hard-impossibility-adjacent item to
   escalate if the operator wants it attempted rather than left as a
   documented exception permanently.
