# Cycle t2b-refine-kind-fix-1 — Gate 3 closure invariant / Card 11, shape T2b — `refine_kind` classifier fix

- **Card ID:** `epic-2-cause-closure` (row 11; scope: `decisions.md §16` item 1 — fix
  `src/bin/v06_work_inventory.rs`'s `refine_kind`, the classifier-noise cause wave 1 (four
  independent lanes) traced T2b's population to)
- **Actor:** `t2b-refine-kind-fix`
- **Base:** `e2bbff32ca328fa3a0a76f0286b2f479f1ae0bc2` (pinned; rebased onto `origin/tranche/12`
  HEAD `d904eceb6` before starting — footgun 1 fired on this worktree too, `git reset --hard`
  applied and re-verified before any other work)
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`
  `PCGEN_ORACLE_SHA`)
- **Status:** complete for the unambiguous population this fix targets. Remaining ambiguous
  population named, counted, and escalated below (not forced, not silently deferred).

## 0. Environment note

`CARGO_TARGET_DIR` under `/home/ubuntu/.cache/codex-targets/` was writable in this sandbox (no
fallback to an in-worktree target dir was needed). The oracle was absent on the fresh worktree
(`scripts/verify.sh --only preflight-oracle` FAILed) and was bootstrapped with
`scripts/fetch-pcgen-oracle.sh --dest <repo-local pcgen slot>`, matching wave 1's own experience.

## 1. What was fixed, and why the naive version was refused

`refine_kind` (`src/bin/v06_work_inventory.rs`) matched a `_abilities_race.lst` row's `TYPE:`
**first dot-segment** against `MONSTER_ABILITY_TYPE_FACETS`'s bare vocabulary
(`NaturalAttack`/`SpecialAttack`/`SpecialQuality`/`Universal Monster Rule`). Bestiary-style books
name their special-ability rows with **compound, race-specific** first segments instead
(`AghashRacialAbility`, `BearLordRacialTrait`, `RacialAbility`, `AdletSelection`, ...), so those
rows stayed typed `race_trait` — the classifier-noise cause all four wave-1 lanes independently
traced (`decisions.md §16`).

**The known trap ruled out a naive fix before this cycle started:** the w1-d lane
(`t2b-bestiary_3-measurement-receipt.md` §6 item 1) verified that every real player race's own
`Favored Enemy ~ Humanoid (<Race>)` row carries
`TYPE:RangerClassFeatures.FavoredEnemy.SpecialAttack.Extraordinary.AttackOption` — sharing the
inner `SpecialAttack` dot-segment with the monster-only facet vocabulary. Any TYPE-segment
widening (first-segment-only relaxed to any-segment, or a broader facet list) would wrongly
reclassify this genuine content in **every** book that carries it.

**This cycle does not widen the TYPE-segment check at all.** It adds a second, independent
signal that reads the row's `KEY:` prefix instead:

> A `_abilities_race.lst` row whose `KEY:` prefix (the text before the first ` ~ `) exactly names
> a `CR:`-bearing entry in the **same book's own** `*_races.lst` is a monster's own sub-ability,
> regardless of what its `TYPE:` looks like.

Because this reads the KEY, not the TYPE, it shares no vocabulary with the Favored-Enemy trap —
`Favored Enemy ~ Humanoid (<Race>)`'s KEY prefix is the literal string `Favored Enemy`, never a
race or monster name, so it can never collide with a `*_races.lst` entry regardless of what
`<Race>` the row is for. Proved by a dedicated regression test (§4).

**Deliberately excludes `*_templates.lst`.** The dispatch brief's own §6 item 1 named
`*_templates.lst` as a candidate second source. Stress-tested before building on it
(`scripts/t2b_refine_kind_key_prefix_stress_test.py`, corpus-wide, `off` vs `on` mode): matching
`*_templates.lst` names too introduces a real false positive —
`advanced_race_guide/arg_templates.lst`'s `Feral` row (`SUBRACE:Feral`, an Orc subrace template)
shares its name with `arg_abilities_race.lst`'s genuine, player-facing `Feral ~ Languages` racial
trait row. A `*_templates.lst` name is not always monster-owned the way a `CR:`-bearing
`*_races.lst` name always is (that is the corpus's own pre-existing `Kind::Race -> Kind::Monster`
discriminator, reused here, not invented). `book_cr_bearing_race_names` therefore reads
`*_races.lst` only.

## 2. Corpus-wide stress test of the discriminator, before wiring it in

`scripts/t2b_refine_kind_key_prefix_stress_test.py` (committed):

```
python3 scripts/t2b_refine_kind_key_prefix_stress_test.py off
```

Checked every one of the 154 directories in the pinned oracle that carry an `*_abilities_race.lst`
file (all publishers, not just this program's 26 in-scope T2b books), 5,954 rows total. Cross-
referenced every hit against 10 known real-race-book directories (`core_rulebook`,
`advanced_players_guide`, `advanced_race_guide`, `advanced_class_guide`, `bestiary_2`,
`bestiary_5`, `bestiary_6`, `inner_sea_races`, `core_essentials`, `ultimate_wilderness`):

- **0** hits in `core_rulebook`/`advanced_players_guide`/`advanced_race_guide`/
  `advanced_class_guide`/`bestiary_5`/`bestiary_6`/`inner_sea_races`/`core_essentials`/
  `ultimate_wilderness`.
- **296** hits in `bestiary_2` — all confirmed monster stat-block content
  (Achaierai/Akata/Amphisbaena/... — this book carries both genuine playable races AND CR:-bearing
  monster entries in the same `*_races.lst`, exactly the w1-a lane's own finding).
- **0** hits against any of this corpus's own playable-race names anywhere (Grippli, Ifrit, Oread,
  Sylph, Undine, Fetchling, Ratfolk, Skinwalker, Rougarou, Dhampir, Dwarf, Elf, Gnome, Halfling,
  Human, Half-Orc, Half-Elf — grepped explicitly against the script's own output).
- **0** `Favored Enemy` rows caught by the false-positive scan (the named trap, confirmed absent).

`bestiary` (bestiary_1) is deliberately excluded from the "known real-race book" cross-reference:
it is not a real playable-race book (confirmed corpus-wide, `not-ingested-figures-are-classifier-
noise` memory note) — its 528 CR:-only hits (visible when `bestiary` is included) are correct
reclassifications, not false positives; the script's own comment documents why it is excluded from
the false-positive check specifically.

## 3. Fix, TDD'd

**RED, for the intended reason:** deliberately widened `book_cr_bearing_race_names` to also read
`*_templates.lst` rows unconditionally (simulating the naive fix the dispatch brief warned
against), then ran the new regression test
`book_cr_bearing_race_names_tests::templates_lst_names_never_enter_the_monster_race_name_set`:

```
test book_cr_bearing_race_names_tests::templates_lst_names_never_enter_the_monster_race_name_set ... FAILED
thread '...' panicked at src/bin/v06_work_inventory.rs:1789:9:
{"Feral"}
```

Confirmed RED for the exact named reason (the ARG `Feral` false positive), not an unrelated
compile error. Reverted the deliberate widening.

**GREEN:**

```
cargo test --bin v06_work_inventory --locked
...
test result: ok. 308 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.65s
```

New tests added this cycle (12 total): 4 unit tests on `refine_kind` itself (compound-TYPE-segment
row now promotes via KEY prefix; the Favored-Enemy trap stays `RaceTrait` even when the book's own
monster-name set coincidentally contains the named race; a Favored-Class-Bonus choice row with a
matching KEY prefix still stays `RaceTrait`, proving the two gates share one guard, not two that
could disagree; a templates.lst-only name never causes reclassification), 3 tests on
`book_cr_bearing_race_names` itself (recursive collection incl. the `core_essentials` per-race
nesting shape, companion-file exclusion, the templates.lst regression guard above), plus the 6
pre-existing `refine_kind` tests updated for the new third parameter (unaffected in outcome).

## 4. Movement, reported in both directions

`scripts/t2b_refine_kind_fix_movement_report.py` (committed), joining `docs/work-inventory.json`
before/after by `(book, source_file, source_line)` — coordinate-stable across a kind change even
though the unit's own `id` embeds `kind` and therefore changes:

```
python3 scripts/t2b_refine_kind_fix_movement_report.py <before.json> docs/work-inventory.json
```

```
=== Kind transitions (before -> after) ===
race_trait -> monster_ability: 864

=== race_trait -> * (moved OUT of race_trait), by book ===
  bestiary_3: 625
  ultimate_psionics: 112
  bestiary_2: 69
  bestiary_4: 42
  bestiary: 9
  inner_sea_gods: 3
  inner_sea_bestiary: 2
  occult_adventures: 2
TOTAL moved out of race_trait: 864

=== * -> race_trait (moved IN to race_trait), by book ===
TOTAL moved into race_trait: 0
```

**Zero units moved into `race_trait`.** This fix is a one-directional widening of the
`MonsterAbility` match (adds a second signal that can promote a row `RaceTrait -> MonsterAbility`;
never demotes one the other way) — confirmed by a full join over both files, not assumed from the
design.

All 864 moved units carried `evidence: race_trait_race_not_modelled` before (i.e. all 864 were
counted in T2b). After the fix: 861 land on `monster_ability_absent_from_<book>_monster_abilities`
(T9's own evidence family, `scripts/sd32_t9_census.py`'s `EVIDENCE_FAMILIES["monster_ability"]`
regex `^monster_ability_absent_from_`), 2 land on `monster_ability_has_no_engine_table` (a
different not-ingested-shaped bucket, outside T9's regex), and **1 unit is already fully closed**:
`bestiary_2:b2_abilities_race.lst:208` (`Bunyip ~ Blood Rage`) resolves to
`status: text-complete`, `evidence: monster_ability_held_and_corpus_record_carries_real_description`
— a genuine, already-ingested monster-ability record this cycle's fix correctly re-attributed, not
new content authored by this cycle.

## 5. Shape totals re-derived, before and after, with commands

Corpus-wide kind totals:

```
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
from collections import Counter
c = Counter(u.get('kind') for u in d['units'])
for k,v in sorted(c.items()): print(k,v)
"
```

| kind | before | after | delta |
|---|---:|---:|---:|
| `race_trait` | 3,504 | 2,640 | -864 |
| `monster_ability` | 2,942 | 3,806 | +864 |

**T2b** (`kind==race_trait AND evidence==race_trait_race_not_modelled`):

```
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
u = [x for x in d['units'] if x.get('kind')=='race_trait' and x.get('evidence')=='race_trait_race_not_modelled']
print(len(u))
"
```

- **Before this cycle's rebase:** 2,472 (matches `decisions.md §13`'s table exactly).
- **After rebase, before this cycle's fix:** 2,472 unchanged (no code between the two states
  touched T2b's classifier; confirmed by regenerating against the unmodified post-rebase HEAD
  before writing any fix code).
- **After this cycle's fix:** **1,578**. Net movement -894 = -864 (this cycle's fix) + -30
  (18 Dhampir chassis + 12 `inner_sea_races` stale-regen units the wave-1 `epic-2-t2b-*` lanes
  already closed via real ingestion between this cycle's rebase and its own run — reconciled by
  the movement report's coordinate join, which shows 0 units this cycle moved that were not
  `race_trait -> monster_ability`).

**T9** (`scripts/sd32_t9_census.py`'s six evidence-family filter, shares the `monster_ability`
kind with T2b):

```
python3 scripts/sd32_t9_census.py <inventory.json>
```

| | before | after | delta |
|---|---:|---:|---:|
| T9 `monster_ability` sub-total | 517 | 1,378 | +861 |
| T9 grand total (all 6 kinds) | 2,712 | 3,573 | +861 |

**This is the honest cost of the fix, named explicitly per this cycle's own brief:** T2b's
population shrinks because 864 units were never race content at all — but 861 of those 864 are
real, un-ingested `monster_ability` content that now correctly counts against T9 instead. This is
not a net reduction in open work; it is a **reclassification of which shape's ledger the work sits
on**, which is exactly what `decisions.md §16`'s guard rail requires this receipt to say plainly
rather than let read as work vanishing.

## 6. Remaining T2b population — what is genuinely resolved vs. still ambiguous

Re-running the wave-1 w1-d lane's own classification script against the new inventory
(`scripts/t2b_bestiary_3_row_classify.py`, unmodified) on `bestiary_3` specifically:

```
python3 scripts/t2b_bestiary_3_row_classify.py
```

```
# bestiary_3 T2b units (race_trait_race_not_modelled): 194   (was 819)
category_header: 9        (by-design exclusion, unaffected -- correct)
adopted_race: 5           (real work, blocked on the AdoptiveRace selector mechanism,
                            decisions.md §16 item 2 -- unaffected by this cycle, not this
                            cycle's scope)
monster_or_template_owned: 58   (KEY prefix matches a b3_templates.lst NAME, not a
                                  CR:-bearing races.lst name -- deliberately NOT moved by
                                  this cycle, per §1's templates.lst exclusion; e.g. "Animal
                                  Lord ~ Bear Lord (Grizzly Bear)" whose KEY prefix "Animal
                                  Lord" names a b3_templates.lst template)
unresolved: 122            (KEY prefix is a NAMED VARIANT of a monster/template name, not an
                            exact string match -- e.g. "Confounding Bandersnatch" vs.
                            races.lst's "Bandersnatch"; "Awakened Demilich" vs. "Demilich";
                            "Bear Lord ~ Bear Hug" vs. templates.lst's "Animal Lord")
sum check: 194 (expect 194)
```

**This cycle's fix closes the unambiguous population only** (exact `KEY:` prefix match against a
`CR:`-bearing `*_races.lst` name, in the same book) — 625 of `bestiary_3`'s 819, and 864 corpus-
wide. The remaining 194 in `bestiary_3` (and the analogous residuals in `bestiary_2`/`bestiary_4`/
`bestiary`/`inner_sea_gods`/`inner_sea_bestiary`/`occult_adventures`/`ultimate_psionics`, not
separately re-run this cycle for budget reasons but structurally the same shape by the movement
report's own per-book breakdown in §4) is **not forced**, per this cycle's own brief ("a partial
fix that is provably safe beats a complete fix that is not"):

1. **58 `monster_or_template_owned`-in-`bestiary_3` (template-name matches).** Resolving these
   safely needs a matching rule that distinguishes a genuinely monster-only template
   (`Animal Lord`) from a subrace template that grants real player content (ARG's `Feral`) —
   the exact ambiguity §1 stress-tested and refused to force. **What would resolve it:** a
   per-template classification (is this template's own row shape a player subrace selector, like
   `SUBRACE:Feral`, or a pure creature template?) — genuinely new discriminator work, not a
   one-line widening of this cycle's fix.
2. **122 `unresolved`-in-`bestiary_3` (name-variant matches).** Resolving these needs fuzzy or
   prefix-family matching (`"Confounding Bandersnatch"` contains `"Bandersnatch"`) rather than
   exact equality — genuinely riskier (a looser match could catch real content this cycle proved
   must stay excluded) and out of this cycle's scope.
3. **The `AdoptiveRace` selector mechanism** (5 units in `bestiary_3`, ~9 more across the
   registered-book pile per the census memo) — unrelated to the classifier, a real missing ingest
   mechanism, already escalated in `t2b-bestiary_3-measurement-receipt.md` §6 item 2 and unchanged
   by this cycle.

**Escalated per Blocker Discipline**, not filed as a disposition: items 1 and 2 above are named,
counted (58 + 122 = 180 in `bestiary_3` alone; the movement report's per-book breakdown implies a
comparable residual in the other six moved-from books), and given the exact next step. This is
disposition 2 ("raise your hand") — the work is not silently deferred, and card 11 does not read as
closed on this shape.

## 7. Sweep for pinned counts

```
grep -rn "2472\|2,472\|819\|2325\|2,325" tests/ src/ scripts/ apps/ 2>/dev/null | grep -v "/target/" | grep -v "/artifacts/corpus/"
```

No hardcoded `assert`/`assert_eq` in `src/`, `tests/`, or `apps/` pins any of T2b's old or new
totals — every hit found is prose (doc comments, `docs/release/` receipts, `scripts/t2b_*.py`
docstrings), none of which is a gate. `apps/desktop/src-tauri` (separate Cargo workspace) checked
explicitly:

```
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml
```

Result and full output logged in §8.

**One pre-existing, unrelated pinned-test breakage found and fixed** (not introduced by this
cycle): `rule_set_mapping_tests::uncompiled_books_stay_none` asserted `inner_sea_temples` stays
uncompiled, but card 15's already-merged `d904eceb6` added `RuleSetId::InnerSeaTemples` to
`COMPILED_RULE_SETS` without updating this test — left the branch red on `origin/tranche/12` itself
before this cycle touched anything. Retargeted to `guide_to_the_river_kingdoms` (confirmed genuinely
uncompiled: absent from every `corpus_dir_for` arm, real corpus directory present in the pinned
oracle). Logged: `scripts/retro.py correction`, `docs/retro/events/t2b-refine-kind-fix.jsonl`.

## 8. Verification run (full)

```
cargo test --bin v06_work_inventory --locked
   -> test result: ok. 308 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

cargo test --locked --lib
   -> test result: ok. 2388 passed; 0 failed; 13 ignored; 0 measured; 0 filtered out

cargo test --locked --bins
   -> every bin's own suite green (308 in v06_work_inventory's own bin target; every other
      bin's suite unaffected and green; grepped `test result:` across all bin outputs, zero
      `FAILED`)

cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml
   -> test result: ok. 517 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

scripts/verify.sh --only reach
   -> PASS  reach  (31 passed)
```

## 9. Discovery forwards

- `## DISCOVERED`: `bestiary_2`/`bestiary_4`/`bestiary`/`inner_sea_gods`/`inner_sea_bestiary`/
  `occult_adventures`/`ultimate_psionics` each carry a residual T2b population this cycle did not
  individually re-run `t2b_bestiary_3_row_classify.py`-style classification against (only
  `bestiary_3` was, since it already had the wave-1 script committed) — the movement report's
  per-book breakdown in §4 gives each book's exact moved-out count; whoever picks up T2b's
  remaining residual should re-run the same classification per book before assuming the shape.
- `## DISCOVERED`: T9's `monster_ability` sub-population jumped from 517 to 1,378 as a direct,
  correctly-attributed consequence of this fix (§5) — any T9-scoped cycle must re-derive its own
  population with the command above before sizing work, not use the pre-fix 2,712 total.

## 10. Next-cycle plan

This cycle's scope (`decisions.md §16` item 1, the classifier fix itself) is complete for the
unambiguous population. Two named, counted, escalated residuals remain (§6 items 1-2) plus the
already-escalated `AdoptiveRace` selector mechanism (§6 item 3, unchanged from wave 1). The next
T2b cycle should pick up `decisions.md §16` item 2 (`AdoptiveRace` selector, spans 5 books) next,
per the corrected 3-cycle plan §16 itself lays out, then re-measure T2b's true residual once both
land.
