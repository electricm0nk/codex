# Card 11, shape T9 — per-record onboarding backlog census

**Actor:** `t9-census`. **Scope:** measurement only, per `decisions.md §13` (authorises measurement
as a first step; measurement is a precursor to the work, never a substitute for it). No production
source, corpus data, or pinned count was changed this cycle.

**Base:** `8b8e00c0d` (Decision 13 committed), rebased onto `origin/tranche/12` at
`3981e7091` before this cycle's first command. Oracle: `PCGEN_ORACLE_SHA
7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`, repo-local slot,
self-healed via `scripts/fetch-pcgen-oracle.sh --dest <slot>` — empty on this fresh worktree).

## 1. Re-derivation — total confirmed at 2,712, unchanged from `decisions.md §13`

```
cargo build --locked --release --bin v06_work_inventory
PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data \
    <target>/release/v06_work_inventory --stdout-only > fresh_inventory.json
python3 scripts/sd32_t9_census.py fresh_inventory.json
```

Filtered to the six evidence-code families `epic-breakdown.md`/`THE-BOX.md` name for T9
(`spell_key_absent_from_spell_list`, `companion_absent_from_*`, `feat_key_absent_from_catalog`,
`monster_ability_absent_from_*`, `equipment_key_absent_from_equipment_tables`,
`monster_absent_from_*`):

| Kind | Units |
|---|---:|
| spell | 732 |
| companion | 726 |
| feat | 487 |
| monster_ability | 517 |
| equipment | 222 |
| monster | 28 |
| **Total** | **2,712** |

This matches `decisions.md §13`'s table exactly and the T9 lane's own already-corrected figure
(`artifacts/gate-3-closure-invariant/epic-2-t9_cycle-1_cycle_receipt.md`, which corrected the
original `epic-breakdown.md` figure of 2,651 up to 2,712, +61, and logged that correction). **No
new correction is filed here** — the figure that "looked right" (2,712, already corrected once)
re-derives clean against the current tip; the headline number is not stale.

## 2. Per-book breakdown — the real sizing unit

29 books carry T9 units. Per-book, per-kind counts (re-derive with
`scripts/sd32_t9_census.py`):

| Book | Total | Kinds (count) |
|---|---:|---|
| mythic_adventures | 365 | feat 353, spell 9, equipment 3 |
| occult_adventures | 330 | spell 329, monster 1 |
| bestiary_4 | 266 | monster_ability 191, spell 56, monster 14, equipment 3, companion 2 |
| ultimate_wilderness | 249 | companion 248, feat 1 |
| advanced_players_guide | 203 | companion 203 |
| adventurers_guide | 200 | equipment 115, feat 81, spell 4 |
| bestiary | 196 | spell 109, monster_ability 83, monster 4 |
| ultimate_magic | 160 | companion 138, spell 22 |
| horror_adventures | 154 | spell 72, monster_ability 65, feat 17 |
| core_rulebook | 86 | companion 86 |
| ultimate_psionics | 64 | monster_ability 64 |
| inner_sea_world_guide | 56 | equipment 7, feat 6, monster 5, monster_ability 16, spell 22 |
| bestiary_2 | 52 | monster_ability 49, monster 2, equipment 1 |
| inner_sea_races | 52 | spell 29, feat 22, equipment 1 |
| inner_sea_temples | 43 | equipment 43 |
| inner_sea_bestiary | 40 | monster_ability 38, monster 2 |
| inner_sea_gods | 36 | equipment 25, monster_ability 7, spell 4 |
| book_of_the_damned_volume_1 | 35 | companion 29, spell 6 |
| inner_sea_intrigue | 34 | spell 26, equipment 8 |
| monster_codex | 24 | spell 24 |
| advanced_race_guide | 18 | companion 18 |
| inner_sea_magic | 18 | spell 5, feat 7, equipment 6 |
| book_of_the_damned_volume_2 | 13 | spell 12, equipment 1 |
| inner_sea_combat | 7 | equipment 7 |
| bestiary_3 | 5 | monster_ability 4, equipment 1 |
| bestiary_5 | 2 | companion 2 |
| ultimate_equipment | 2 | spell 1, equipment 1 |
| inner_sea_faiths | 1 | spell 1 |
| ultimate_combat | 1 | spell 1 |
| **29 books** | **2,712** | |

## 3. Overlap with card 4 (book onboarding, this bundle)

Card 4 onboarded `inner_sea_faiths`, `inner_sea_magic`, `inner_sea_taverns`, `inner_sea_temples`
mid-bundle. `inner_sea_taverns` carries **0** T9 units in the fresh derivation (confirmed present
in the live corpus at `artifacts/corpus/operator-supplied/pcgen/data/pathfinder/paizo/
campaign_setting/inner_sea_taverns/_inner_sea_taverns.pcc`, so this is a real onboarded book with
no residual T9 gap, not a book absent from the corpus). The other three card-4 books still carry
residual T9 units (`inner_sea_faiths` 1, `inner_sea_magic` 18, `inner_sea_temples` 43) — card 4's
book-level `RuleSetId` registration did not itself close the per-record spell/equipment/feat
tables T9 measures; those are a separate onboarding step. **This population is re-derived fresh
against the post-card-4 corpus** (the fresh inventory build runs against current HEAD, which
already includes card 4's landings) — it is not a stale pre-card-4 quote.

## 4. Overlap with T2b — significant, must be reported to work lanes

**Superseding note:** while this census was in progress, a concurrent T2b-census cycle landed
(`ad7d7c157`, `artifacts/gate-3-closure-invariant/card11-t2b-census-census.md`) with a
command-derived, corrected T2b book list — 17 unregistered + 9 registered-but-untranscribed = 26
books (`beastiary`, the legacy core-bestiary id, has 0 residual units and drops out). This
supersedes the earlier T2b lane receipt's book list (which this memo originally compared against)
with a more accurate one — e.g. it adds `pathfinder_unchained` and `advanced_class_guide`, absent
from the earlier receipt's hand-derived list. **Comparing T9's fresh 29-book set against this
newer, committed T2b book list:**

```
python3 -c "
t9 = {...29 books from §2...}
t2b = {...26 books from card11-t2b-census-census.md §2's table...}
print(sorted(t9 & t2b))
"
```

**22 of T9's 29 books also appear in T2b's 26-book list:**

```
advanced_players_guide, advanced_race_guide, bestiary, bestiary_2, bestiary_3, bestiary_4,
bestiary_5, book_of_the_damned_volume_1, book_of_the_damned_volume_2, core_rulebook,
horror_adventures, inner_sea_bestiary, inner_sea_gods, inner_sea_races, inner_sea_world_guide,
monster_codex, mythic_adventures, occult_adventures, ultimate_combat, ultimate_magic,
ultimate_psionics, ultimate_wilderness
```

**T9-only books (7, not named in T2b's population):** `adventurers_guide`, `inner_sea_combat`,
`inner_sea_faiths`, `inner_sea_intrigue`, `inner_sea_magic`, `inner_sea_temples`,
`ultimate_equipment`.

**T2b-only books (4, not named in T9's population):** `advanced_class_guide`, `bestiary_6`,
`pathfinder_unchained`, `ultimate_intrigue`.

**Consequence for dispatch:** for those 22 shared books, one onboarding cycle touching that
book's count-pinning files closes units in *both* T2b (`race_trait`) and T9 (`spell`/`companion`/
`feat`/`monster_ability`/`equipment`/`monster`) simultaneously — different kinds, same book-level
fixed cost. Sequencing a T9 book cycle and a T2b book cycle for the same book as two independent
efforts double-pays the fixed cost `decisions.md §13` explicitly says is what determines size.
**A single per-book cycle scoped to "onboard book X across every kind with an open shape (T2b
and/or T9)" is the correct unit of work**, not one cycle per shape per book. 22 of T9's 29 books
(76%) fall in this shared-cost category — this is the single most consequential finding for
dispatch sequencing across both censuses.

## 5. Confirmed false-positive / non-closeable-by-data-cycle sub-population (monster kind)

The T9 lane's own cycle-1 receipt (`epic-2-t9_cycle-1_cycle_receipt.md`, retained here because it
already did this forensic work — not re-run from scratch, per the dispatch brief's "read the
lane's receipt before re-deriving") ran a full dry-run forensic pass on all 28 `monster`-kind
units across all 6 residual books:

- **21 of 28 — Product-Identity-excluded** by the transcription tool's own PI screen
  (`NAMEISPI:YES` / `PI_BLACKLIST_TERMS` hits in `bestiary_4`, `inner_sea_world_guide`,
  `inner_sea_bestiary`). These cannot be closed by a data-entry cycle at all — transcribing them
  would republish Paizo Product Identity, and `docs/governance/ogl-pi-blacklist.md` is explicitly
  DRAFT/operator-review-gated. **This is a real blocker a T9 work cycle will hit and must escalate
  under `AGENTS.md` Blocker Discipline** (clear or escalate, never defer) — it is not something
  this census cycle can resolve, but it must not be silently absorbed into a "closed by doing the
  work" cycle either.
- **6 of 28 — structurally non-standalone rows** (`.MOD`/`.COPY` overlay rows), correctly excluded
  by the tool's own logic. Not creatures in their own right; **not real T9 defects**, but they do
  currently show as `not-ingested` in the work-inventory, so they inflate the raw count by 6 units
  that no code or content change should ever "close" as new records.
- **1 of 28 — a genuine onboarding gap** (`occult_adventures:monster:kami_shikigami`) — no
  `RuleSetId` match arm for the `monster` kind's chassis registry in that book at all. This is the
  one unit in the whole 2,712 with a fully-identified, mechanically closeable fix named in the
  source receipt.

**This split (PI-excluded / structural-non-defect / genuine-gap) is verified only for the 28
`monster`-kind units (1.0% of 2,712) and the `companion`/`core_rulebook` spot-check (86 units,
3.2%) — total 114/2,712 (4.2%) forensically checked.** It is **not** assumed to generalize to the
other 2,598 units (`spell` 732, `feat` 487, `equipment` 222, `monster_ability` 517 minus the
already-checked slice, and the other 7 `companion` books' 640 units) — same caution the source
receipt itself states. Each of `spell`/`feat`/`equipment` additionally has **no transcription
tool at all today** (`ls scripts/transcribe_*.py` → only `transcribe_monster_tables.py` and
`transcribe_companion_tables.py` exist), so a PI-screen-equivalent check has to be built before
those three kinds' books can be onboarded safely.

## 6. Work-lane sizing — do not size by unit count

Per `decisions.md §13` and the E13 book-ingest calibration (`docs/retro/`, cited in
`acceptance-and-verification.md` AT-32-E4-001: "cost is ~1.5–2h per book, dominated by ~7
count-pinning files"), the correct sizing unit is **books, not the 2,712 record count**. 29 books
need touching. Grouping by the natural work unit ("onboard every open kind in this book in one
pass," folding in T2b's `race_trait` kind for the 21 shared books per §4 above) gives **29 book-level
cycles**, plus:

- **1 blocking precondition** (not a cycle this bundle can size cleanly): the operator PI ruling
  needed before any `monster`-kind PI-flagged record (21 confirmed, unknown count in the other 5
  kinds — not yet checked) can be transcribed. Flagged as a blocker for escalation by the first
  work cycle that reaches it, not resolved here.
- **3 new transcription tools** (`transcribe_spell_tables.py`, `transcribe_feat_tables.py`,
  `transcribe_equipment_tables.py`) needed before those three kinds' books can be onboarded with
  the same PI/structural-exclusion safety net `monster`/`companion` already have. Tool-building is
  its own fixed cost, separate from any one book's cycle.

## 7. Notes / findings summary

- Total re-derived at **2,712**, matching `decisions.md §13` exactly — the headline figure that
  "looked right" was checked and holds.
- **22-of-29 book overlap with T2b** (against T2b's own newer, corrected 26-book census) is the
  single most consequential finding for dispatch sequencing (§4) — not previously quantified in
  any prior receipt.
- **27 of 2,712 units (1.0%) are not real per-record gaps at all** (21 PI-excluded — blocked
  pending an operator ruling, not closeable by data entry; 6 structurally-correct exclusions —
  never should be "closed"). These are false positives in the raw evidence-code count in the same
  sense `decisions.md §13` flags for T12's ~47 phantom-class units — named here, not silently
  dropped and not silently kept as ordinary backlog.
- `inner_sea_taverns` (one of card 4's four onboarded books) carries zero T9 units — confirmed a
  real, corpus-present onboarded book, not evidence of a missing book.
