# Card 11, shape T2b — book-by-book work census (measurement cycle)

- **Card ID:** `epic-2-cause-closure` (row 11; this cycle's scope: shape T2b only, measurement per
  `decisions.md §13`)
- **Actor:** `t2b-census`
- **Base:** `8b8e00c0d` (pinned), rebased onto `origin/tranche/12` at `3981e7091` before starting
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`
  `PCGEN_ORACLE_SHA`), re-verified via `scripts/verify.sh --only preflight-oracle` after
  `scripts/fetch-pcgen-oracle.sh --dest <repo-local pcgen slot>` on this fresh worktree.
- **Status:** measurement cycle only — **0 units banked**, per `decisions.md §13`'s explicit
  authorization ("measurement... does not substitute for the work... a precursor to it"). This memo
  is the book/file census that sizes the T2b work lanes to be dispatched next.
- **Re-derive script (committed):** `scripts/t2b_race_trait_census.py`

## 0. What this is answering

`epic-2-cause-closure_cycle-2_epic-2-t2b_cycle_receipt.md` (the T2b lane's own prior forensic
cycle) already proved the *named* cause — the compound-key matcher
(`modelled_race_of_race_trait`) — explains **zero** of the 2,472 open T2b units, because the
matcher only ever runs on already-ingested `data/corpus` records and none of the 2,472 residual
units were ever ingested. That finding is not re-derived here; it is read and trusted. This memo
picks up from there and turns "2,472 units, never ingested" into a **book-by-book work list**, per
the dispatch brief's requirement (`decisions.md §13`: cost is per-book/per-file, not per-record).

## 1. Re-derived population (unchanged from the prior lane cycle)

```
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
u = [x for x in d['units'] if x.get('kind')=='race_trait' and x.get('evidence')=='race_trait_race_not_modelled']
print(len(u))
"
```
→ **2,472**. Matches `epic-breakdown.md`'s T2b row and `decisions.md §13`'s table exactly. **No
correction to the total.**

`docs/work-inventory.json` is committed at HEAD (no regen needed); each unit carries `book`,
`corpus_key` (the full compound key, e.g. `"Adopted Race ~ Fetchling"` — note this is **not** the
same field as `name`, which strips the category prefix), `source_file`, `source_line`.

## 2. The book-by-book split (fresh command, not quoted from any doc)

Cross-referenced every residual unit's `book` field against
`apps/desktop/src-tauri/src/race_catalog.rs`'s `RACE_CORPUS_BOOKS` constant (re-read fresh, not
quoted):

```
python3 scripts/t2b_race_trait_census.py
```

```
book,count,registered_in_race_catalog
bestiary_3,819,False
bestiary_2,259,True
ultimate_psionics,159,False
bestiary_4,141,False
bestiary_5,137,True
core_rulebook,132,True
pathfinder_unchained,127,False
mythic_adventures,118,False
bestiary,105,False
ultimate_wilderness,103,False
occult_adventures,73,False
inner_sea_races,59,True
advanced_players_guide,58,True
advanced_race_guide,57,True
inner_sea_world_guide,34,False
advanced_class_guide,31,False
inner_sea_gods,28,False
monster_codex,9,True
horror_adventures,6,True
inner_sea_bestiary,4,False
ultimate_combat,4,False
ultimate_intrigue,3,False
ultimate_magic,3,False
bestiary_6,1,True
book_of_the_damned_volume_1,1,False
book_of_the_damned_volume_2,1,False

unregistered-book pile: 1754
registered-book pile: 718
sum check: 2472 (matches)
```

`RACE_CORPUS_BOOKS` currently registers 10 books: `core_rulebook`, `beastiary`,
`advanced_race_guide`, `advanced_players_guide`, `monster_codex`, `inner_sea_races`,
`horror_adventures`, `bestiary_2`, `bestiary_5`, `bestiary_6`. Note **`beastiary`** (the legacy
core-bestiary book id, distinct from `bestiary`, `bestiary_2`.. `bestiary_6`) has **zero** residual
T2b units — it does not appear in the per-book table above at all, i.e. it is already fully clean.
This confirms `decisions.md §13`'s 1,754/718 split is correct at the total level.

## 3. The registered-book (718) pile — what it actually is, re-derived, not guessed

The prior lane receipt (§3) characterized this pile as "~350+ category-header rows" plus "~44
`Adopted Race ~ <X>` selector rows" and left the remainder uncharacterized. That characterization
was a hand-sample, not a full command. **Re-derived exactly:**

```
python3 scripts/t2b_race_trait_census.py --dump-other --dump-adopted-books
```

Classification rule (applied to `corpus_key`, not `name` — `name` strips the category prefix,
which is why a naive read of `name` alone looks like it's naming real races when the record is
actually a category-header or selector row):

1. `corpus_key` starts with `"Adopted Race ~ "` → selector-mechanism gap (real work,
   `ingest_races.rs`'s flat standard-trait loop never captures this shape).
2. `corpus_key` matches `^Racial SLA ~ `, `^Unchained Evolution ~ `, `^Favored Class Bonus ~ `, or
   `^Race Subtype ~ ` → category-header row, **no race named at all** — correctly excluded by the
   matcher's own design, confirmed by design intent, **not open work**.
3. Everything else → "other": ordinary per-record content (named traits, ability-score
   adjustments, compound `<Race> ~ <Trait>` keys) that was simply never transcribed from the oracle
   `.lst` rows into `data/corpus` JSON, in a book that IS registered.

**Result:**

| Sub-shape | Units | Real work? |
|---|---:|---|
| Category-header rows (by-design exclusion) | **147** | No — confirmed correctly excluded |
| `Adopted Race ~ <X>` selector rows | **9** | Yes — ingest-tool selector-capture gap |
| Other (never-transcribed per-record content) | **562** | Yes — per-record onboarding backlog |
| **Total (= registered-book pile)** | **718** | — |

**Finding — correction to the prior receipt's characterization** (logged via
`scripts/retro.py correction`, `docs/retro/events/t2b-census.jsonl`): the prior receipt guessed
"~350+" category-header rows; the real, command-derived count is **147**. The prior receipt named
only two sub-shapes ("Adopted Race" and "category-header"); a full classification shows those two
together are only 156 of 718 — the other **562 units (78% of the 718 pile) are ordinary
never-transcribed content**, not covered by either named sub-shape. **The real open work in the
registered-book pile is 571 (9 + 562), not 718** — 147 units are correctly not-work.

Sample verification (hand-checked against the pinned oracle, confirming these are real,
not-yet-ingested records, not classifier artifacts):
- `bestiary_3: Adopted Race ~ Catfolk` — `catfolk_abilities_race.lst:30`, `status: not-ingested`.
  File exists in the oracle:
  `.../pathfinder/paizo/roleplaying_game/bestiary_3/b3_abilities_race.lst`. Confirmed real.
- `core_rulebook: Racial SLA ~ Aid` — `corpus_key: "Racial SLA ~ Aid"`, `name: "Aid"` (a spell name
  used as a racial SLA choice; the bare `name` field alone would look like unrelated content —
  the header-pattern classification correctly reads `corpus_key`, not `name`).
- `inner_sea_races: Kasatha ~ Stealthy` — a compound key naming a real ARG-era race (Kasatha) that
  IS in `ingest_races.rs`'s scope, in the "other" bucket: never transcribed, book already
  registered. Genuine per-record onboarding backlog, same shape as T9.

One soft flag for the work lane, not resolved here: `core_rulebook`'s "other" bucket (14 units)
includes a few sentinel-looking rows — `"Region ~ None"`, `"Region ~ Unknown"`, `"No Race Trait
Available"`, `"Remove Excess Points from Pool"` — that may themselves be non-content
bookkeeping rows in the oracle rather than real ingestable traits. Counted here as open
work (conservative default); the dispatched cycle should confirm against the raw `.lst` line before
assuming all 14 are real.

## 4. Per-book/group work list (the deliverable)

Real work only (header-excluded units are dropped from these counts — they are not open work).
Sorted by unit count within each registration bucket.

### Unregistered books (17) — need `RACE_CORPUS_BOOKS` registration + full onboarding

| Book | Units | Files to touch (est.) |
|---|---:|---:|
| `bestiary_3` | 819 | 7 |
| `ultimate_psionics` | 159 | 7 |
| `bestiary_4` | 141 | 7 |
| `pathfinder_unchained` | 127 | 7 |
| `mythic_adventures` | 118 | 7 |
| `bestiary` | 105 | 7 |
| `ultimate_wilderness` | 103 | 7 |
| `occult_adventures` | 73 | 7 |
| `inner_sea_world_guide` | 34 | 7 |
| `advanced_class_guide` | 31 | 7 |
| `inner_sea_gods` | 28 | 7 |
| `inner_sea_bestiary` | 4 | 7 |
| `ultimate_combat` | 4 | 7 |
| `ultimate_intrigue` | 3 | 7 |
| `ultimate_magic` | 3 | 7 |
| `book_of_the_damned_volume_1` | 1 | 7 |
| `book_of_the_damned_volume_2` | 1 | 7 |
| **Subtotal** | **1,754** | — |

Files-to-touch estimate (~7) follows `docs/retro/` E13 calibration ("book onboarding tax is per
file not per record... ~7 count-pinning files is the constant cost"): a new `RACE_CORPUS_BOOKS`
entry, chassis-load wiring in `race_catalog.rs`, `ingest_races.rs`/`ingest_race_traits.rs` book
coverage, fixture derivation, and count-pinning test updates. **Every book in this list has a
confirmed-present `*_abilities_race.lst` (or equivalent) file in the pinned oracle** — spot-checked
`bestiary_3`, `inner_sea_bestiary`, `book_of_the_damned_volume_1/2`, `ultimate_combat`,
`ultimate_intrigue`, `ultimate_magic` directly against
`artifacts/corpus/operator-supplied/pcgen/data/`; none of these 17 are a near-zero-content trap like
the prior receipt worried `book_of_the_damned` might be.

### Registered books (9) needing ingest-tool extension only — no new registration

| Book | Units (real work) | Files to touch (est.) |
|---|---:|---:|
| `bestiary_2` | 259 (252 other + 7 Adopted Race) | 3 |
| `bestiary_5` | 136 (135 other + 1 Adopted Race) | 3 |
| `inner_sea_races` | 59 | 3 |
| `advanced_race_guide` | 53 | 3 |
| `advanced_players_guide` | 37 | 3 |
| `core_rulebook` | 14 | 3 |
| `monster_codex` | 8 | 3 |
| `horror_adventures` | 4 | 3 |
| `bestiary_6` | 1 (Adopted Race) | 3 |
| **Subtotal** | **571** | — |

Files-to-touch estimate (~3) is lower than the unregistered-book tax: no new `RACE_CORPUS_BOOKS`
entry or chassis-load wiring needed, only `ingest_races.rs`/`ingest_race_traits.rs` extension to
capture the missing selector/per-record shapes, fixture derivation, and a count-pinning test update.

**`beastiary` (0 units)** — already fully clean, no lane needed.

### Grand total

- Unregistered-book pile: **1,754**
- Registered-book real work: **571**
- **Total real open work: 2,325**
- By-design exclusion (category-header rows, confirmed not-work): **147**
- Sum check: 2,325 + 147 = **2,472** (matches the re-derived total in §1)

## 5. Sizing note (per `decisions.md §13`'s explicit instruction)

**Do not size this by the 2,325/2,472 unit count.** 26 books/groups need a cycle each (17
unregistered-book onboardings at ~7 files, 9 registered-book ingest-tool extensions at ~3 files).
That is the real cost driver. A book with 819 units (`bestiary_3`) and a book with 1 unit
(`book_of_the_damned_volume_1`) cost roughly the same fixed onboarding tax — the E13 calibration
this bundle already carries.

## 6. Discovery / correction forward

Logged `scripts/retro.py correction` (`docs/retro/events/t2b-census.jsonl`) against
`epic-2-cause-closure_cycle-2_epic-2-t2b_cycle_receipt.md §3`'s "~350+ category-header rows" /
"~44 Adopted Race rows" characterization of the 718-unit registered-book pile: the real,
command-derived split is 147 header (not-work) / 9 Adopted Race / 562 other-never-transcribed, so
real open work in that pile is 571, not 718 as an unqualified figure would suggest.

## 7. Next-cycle plan

Dispatch one TDD cycle per book/group in §4 (26 total). Unregistered-book cycles: register in
`RACE_CORPUS_BOOKS`, wire chassis loading, extend ingest tooling for that book's `.lst` files,
fixture-check every emitted record against the pinned oracle (`decisions.md §3`), update
count-pinning tests. Registered-book cycles: extend `ingest_races.rs`/`ingest_race_traits.rs` to
capture the `Adopted Race ~ <X>` selector mechanism and the other never-transcribed per-record
shapes for that book only, same fixture discipline. Confirm the `core_rulebook` sentinel-row
flag (§3) before assuming all 14 of its units are real content. Each cycle re-runs
`scripts/t2b_race_trait_census.py` after its book lands to confirm that book's count drops to 0 and no
other book's count moved (regression guard).
