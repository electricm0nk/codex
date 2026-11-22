# Cycle epic-2-t12-attribution-gap-shape2 — Gate 3 (closure invariant) / Card 11, shape T12 (continuation)

- **Card ID:** `epic-2-cause-closure` (row 11)
- **Commit SHA:** see `git log -1` at push time (this cycle rebases before pushing per §5)
- **Files touched:**
  - `scripts/census_untabled_base_class_feature_roster.py` — added shape 2 (`CLASS:` level-table
    row) extraction alongside the existing shape 1 (`.MOD` virtual ability), one generic pass, no
    per-class branching; doc comment rewritten to describe both shapes and their honest coverage.
  - `src/rules_core/pilot_compute/untabled_base_class_feature_roster.rs` — module doc comment
    updated for 13/20-class coverage; swapped the "zero-data" example test from `cryptic` (now
    covered) to `psion` (still confirmed absent); added a shape-2 fixture-check test
    (`cryptic_altered_defense_matches_the_oracle_s_shape_2_level_1_grant`).
  - `src/rules_core/pilot_compute/mod.rs` — swapped the "no roster data" wiring test's example
    class from `cryptic` to `psion`; added two new wiring tests proving shape-2 end-to-end
    (`cryptic_level_1_reaches_altered_defense_via_shape_2_but_not_the_level_2_gated_feature`,
    `cryptic_level_2_gains_the_level_2_gated_shape_2_feature`).
  - `tests/fixtures/rules_core/untabled-base-class-feature-roster.json` (regenerated) — grew from
    40 records / 3 classes to 135 records / 13 classes. Antipaladin/Magus/Vigilante's own 40
    shape-1 rows are byte-identical to the prior commit (confirmed: `git diff` shows only
    additions, zero modified/removed lines for those three classes' existing keys).

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  (`git diff --unified=0 -- src/rules_core/pilot_compute/mod.rs src/rules_core/pilot_compute/untabled_base_class_feature_roster.rs scripts/census_untabled_base_class_feature_roster.py | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — no match)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff, `STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack` — no match)

- **Acceptance criterion (verbatim, this dispatch brief):** "Close T12's attribution gap... Pick one
  [of (a) chassis emits attributable ids, or (b) `classify()` learns to attribute the generic ones],
  say why, and say what the other would have cost... Fixture-check every emitted value... Claim only
  what reaches `grounded`/`text-complete`." **MET, with the decision re-scoped by re-derivation** —
  see "The decision, and why neither (a) nor (b)" below. **40 real `class_feature` units now reach
  `text-complete`**, fixture-checked against the pinned oracle, RED→GREEN proven (module tests,
  wiring tests, mutation of the census script's shape-2 extraction).

- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
  matches the pin; oracle bootstrapped fresh into this worktree's git-ignored slot via
  `scripts/fetch-pcgen-oracle.sh`, confirmed against the pin before use).

- **Status:** complete (this lane's own bounded scope — see "What this cycle closes" below; row 11
  stays `in-progress`, T12 is one of card 11's five open sub-shapes and this cycle does not close it
  in full)

## §17a re-derivation — the brief's premise was stale, and the "two zero-closing cycles" framing was too

The dispatch brief said "Two T12 cycles have now landed real generic mechanism and closed zero
units" and cited only `epic-2-t12-modelled-class-books_cycle-1_cycle_receipt.md` as evidence. Before
writing any code, re-read the full artifact directory (`decisions.md §17a`'s standing rule) and found
a **third**, already-committed T12 cycle the brief did not name:
`epic-2-t12-roster-mechanism_cycle-1_cycle_receipt.md` (commit `9838c344d`, already on `origin/tranche/12`
at this dispatch's own base). That cycle **already closed 15 units** to `text-complete` by building a
generic corpus-derived class-feature roster (`untabled_base_class_feature_roster.rs` +
`push_untabled_base_class_feature_records`) for 3 of the 20 chassis-registered classes
(`antipaladin`/`magus`/`vigilante`) whose own-named grants use PCGen's `CATEGORY=Class|<X>.MOD`
shape. **The attribution gap the brief describes (`class_chassis.*` ids with no class-name segment)
was not the blocker this cycle needed to solve — it had already been solved**, by a mechanism neither
option (a) nor (b) named: see below.

Re-derived T12's own live population fresh (`python3 scripts/census_t12_class_feature.py`, pinned
oracle unchanged): **1,009** (not 2,397 as the brief stated), across **75** real unmodelled classes
after false positives (98, not 118 — the false-positive vocabulary itself shifted as concurrent
sibling lanes on this branch landed between the brief's own figures and this cycle). Logged as a
correction per AGENTS.md rule 9 / `decisions.md §17a` — the population collapsed because the
`modelled_class_books()` fix (already landed, prior cycle) moved 1,564 units out of the
`class_feature_of_unmodelled_corpus_class` evidence code entirely, and this figure is untouched by
this cycle's own work (confirmed below — the 10 classes this cycle newly covers are **already**
excluded from the T12/"unmodelled class" bucket, since their owner is modelled; their gap was a
different, more specific one: modelled owner, no grounded feature).

## The decision, and why neither (a) nor (b) — a third option, already built, extended here

The brief asked for a choice between (a) making `untabled_base_class_chassis`'s `class_chassis.*`
ids carry a class-name segment, or (b) teaching `classify()` to attribute the existing generic ids
to their owning class by another route. **Both options were bypassed by a third, already in the
codebase before this cycle started:** a brand-new, purely additive id namespace
(`class_feature.untabled.<class>.corpus_record.<slug>`) emitted by a **new** push function
(`push_untabled_base_class_feature_records`), reusing the same zero-magnitude promotion rule
`decisions.md §7` already grants Pathfinder Unchained's `push_pu_class_feature_records`. The
existing `class_chassis.*` ids are **never touched** — not renamed, not reinterpreted.

**Why this is strictly better than either named option, with evidence:**

- **(a) would have widened `class_chassis.*`'s blast radius.** Those 4 ids
  (`class_chassis.base_attack_bonus`/`base_save.{fortitude,reflex,will}`) are shared across **all
  20** registered classes' `Kind::Class` chassis computation — the exact shape the dispatch brief's
  own warning names ("a sibling lane nearly destroyed 8,247 stamps this way"). Verified no
  verification-stamp reference to these 4 ids changed this cycle: `git diff` touches none of
  `untabled_base_class_chassis.rs`'s own id-emission code.
- **(b) would have widened `classify()`'s own blast radius**, a function reused corpus-wide across
  every kind, not just T12's classes — the same risk class `decisions.md §16` names for T2b's
  `refine_kind`.
- **The additive third option has zero blast radius on anything pre-existing**, because the ids it
  emits did not exist before either T12 cycle: no verification stamp, receipt, dashboard, or test
  could reference `class_feature.untabled.*` before it was built, so nothing could regress. Confirmed
  mechanically: `grep -rn 'class_feature.untabled' tests/ src/ apps/` before this cycle's own diff
  returns only the prior cycle's own module/wiring tests — no external consumer, no stamp to protect
  or corrupt.

**This cycle's own job, correctly scoped from that finding:** the roster mechanism (option-(c)) was
real but incomplete — it found data for only 3 of the 20 registry classes and named the remaining 17
as "a different progression shape, not investigated" (next-cycle plan, prior receipt). This cycle
investigated that shape, found it (PCGen's `CLASS:` level-table row, level read from the row's own
leading tab column rather than a `PREVARGTEQ` clause), confirmed it as a **second generic shape** (no
per-class parsing — one substring match, one leading-field parse, reused across every class the shape
covers), and extended the existing mechanism to it.

## The generic mechanism, shape 2

`scripts/census_untabled_base_class_feature_roster.py` now extracts **two** shapes in one pass, no
per-class branching:

- **Shape 1** (unchanged): `CATEGORY=Class|<ClassName>.MOD` line, level from
  `PREVARGTEQ:<Var>_CFP_Level,<N>`.
- **Shape 2** (new): any line containing `ABILITY:<ClassName> Class Feature|AUTOMATIC|<ClassName> ~
  <Feature>`, level read from that line's own **leading tab-separated field** — the row's own
  position in the class's `CLASS:<ClassName>` level table, PCGen's primary convention for
  automatically-granted class abilities. Confirmed by direct read of the oracle
  (`pathfinder/dreamscarred_press/ultimate_psionics/up_classes.lst` lines 296-330: each row's first
  field, e.g. `1`, `6`, `8`, `9`, is the class level; `ABILITY:Psychic Warrior Class Feature|
  AUTOMATIC|Psychic Warrior ~ Warrior's Path` sits on the level-1 row).

**Coverage found, honestly bounded, not assumed universal:** 10 of the remaining 17 classes use
shape 2 for their own-named grants — `aegis` (9), `cryptic` (12), `dread` (9), `marksman` (7),
`psychic_warrior` (7), `shifter` (17), `soulknife` (7), `tactician` (11), `vitalist` (10), `wilder`
(6) — 95 new records, zero overlap with shape 1's 3 classes (confirmed: the probe script that found
this ran independently of, and before editing, the census script, then the census script's own
merged run reproduced the identical per-class counts). **7 classes remain at zero: `kineticist`,
`medium`, `mesmerist`, `occultist`, `psion`, `psychic`, `spiritualist`** — confirmed absent under
both shapes by direct scan, not merely unchecked (the census script's own `--summary` output; the
zero-data test moved from `cryptic` to `psion`, a class this run re-confirmed has neither shape's
data). **Total fixture: 135 records across 13 of the 20 registered classes** (up from 40/3).

## RED → GREEN, proven

1. Mutated the census script's shape-2 detection (`is_shape2 = shape2_marker in line` →
   `is_shape2 = False`), regenerated the fixture (40 records / 3 classes, matching the pre-cycle
   state exactly — confirmed byte-identical to the prior commit's fixture via `diff`), re-ran
   `cargo test --locked --lib untabled_base_class_feature_roster`:
   ```
   cryptic_altered_defense_matches_the_oracle_s_shape_2_level_1_grant ... FAILED
     "Altered Defense must be in the fixture"
   cryptic_level_1_reaches_altered_defense_via_shape_2_but_not_the_level_2_gated_feature ... FAILED
     "level-1 Cryptic must carry the shape-2 Altered Defense roster id; got: [...no untabled ids...]"
   cryptic_level_2_gains_the_level_2_gated_shape_2_feature ... FAILED
     "level-2 Cryptic must carry Hidden Pattern; got: [...no untabled ids...]"
   ```
   All three failed for the intended reason (fixture/roster genuinely empty for Cryptic, not a typo
   or unrelated break); the other 7 pre-existing tests stayed GREEN throughout, confirming the
   mutation was isolated to shape 2.
2. Reverted the mutation, regenerated the fixture: byte-identical to the pre-revert working state
   (`diff` — no output). Re-ran the same suite: **10/10 GREEN**.

## Suites run

- `cargo test --locked --lib untabled_base_class_feature_roster` (targeted, scoped per this
  dispatch's "scope your test runs" instruction): **10/10** (was 7 before this cycle; +3 new).
- `cargo build --locked --lib`: clean (1 pre-existing, unrelated dead-code warning, unchanged from
  the branch baseline).
- `cargo test --locked --bin v06_work_inventory`: **335/335** (unchanged from the branch's own
  pre-cycle baseline — this cycle touches no code in that binary).
- `cargo test --locked --lib` (full, scoped attempt, foregrounded and awaited): **2,412 passed, 1
  failed, 13 ignored.** The one failure —
  `rules_core::feat_prereqs::prerequisite_tests::a_starting_fighter_keeps_a_real_catalog_and_every_denial_states_why`
  (`left: 755, right: 701`) — is **pre-existing, not caused by this cycle**: `git status --porcelain`
  before this cycle's own edits showed a clean tree at the rebased base (commit `16300bde7`,
  "T9 feat/equipment via existing gap lanes"), and this cycle's diff touches only the four files
  listed above, none of which is `feat_prereqs.rs` or any feat-catalog source. The prior commit on
  this branch (`fb4f28dad`, T9 feat/equipment lane) grew the feat catalog by design (109 new feat
  gap rows) and evidently left this one pinned-count assertion red — an out-of-scope, different
  subsystem (feat prerequisites, not T12 class-feature attribution) this cycle does not fix, per
  AGENTS.md rule 3 ("do not expand scope"). Flagged here per the branch's own standing "left red
  three times" caution rather than silently absorbed.
- Desktop crate (`apps/desktop/src-tauri`, separate cargo workspace) **not re-run this cycle** —
  this cycle's change touches only `rules_core::pilot_compute` and a Python census script, no
  desktop-crate dependency; `git status --porcelain -- apps/desktop` confirms no file under that
  tree changed.

## Live re-derive: what actually closed, fixture-checked, not fabricated (`decisions.md §16`)

Regenerated the full inventory in-memory (`cargo run --bin v06_work_inventory -- --stdout-only`,
pinned oracle, **not written to `docs/work-inventory.json`** — a measurement; `git status
--porcelain` confirms only this cycle's own files changed):

```
python3 -c "
import json
from collections import Counter
d = json.load(open('inventory_after.json'))
shape2 = ('Aegis ~ ','Cryptic ~ ','Dread ~ ','Marksman ~ ','Psychic Warrior ~ ','Shifter ~ ',
          'Soulknife ~ ','Tactician ~ ','Vitalist ~ ','Wilder ~ ')
rows = [u for u in d['units'] if u.get('kind')=='class_feature'
        and (u.get('corpus_key') or '').startswith(shape2)]
print('total own-named units under the 10 shape-2-covered classes:', len(rows))
print(Counter((u['status'], u['evidence']) for u in rows))
"
# -> total 236
# -> Counter({('not-ingested','no_explanation_id_and_no_diagnostic_names_this_feature'): 132,
#             ('not-ingested','class_feature_no_dedicated_magnitude_id_matched_the_record_slug'): 55,
#             ('text-complete','explanation_id_observed_and_corpus_record_carries_real_description'): 40,
#             ('not-ingested','class_feature_owner_matched_by_name_but_record_not_held_by_engine'): 9})
```

**40 real `class_feature` units reach `text-complete` this cycle**, across 10 classes: Aegis (2),
Cryptic (6), Dread (3), Marksman (2), Psychic Warrior (4), Shifter (10), Soulknife (3), Tactician
(5), Vitalist (4), Wilder (1) — full list in the receipt commit's own script output (deterministic,
re-derivable by the command above).

**Before-state, proven by mechanism not assumed:** the RED→GREEN mutation above regenerated the
fixture with shape-2 disabled and reproduced the exact pre-cycle 40-record/3-class state
(byte-identical `diff`), and this module's own `--summary` output before this cycle listed all 10 of
these classes among the "no `.MOD`-shaped own-named grant found" 17 — so 0 of these 236 units could
carry a `class_feature.untabled.*` id before this cycle. No unit was double-counted or already
closed by a different route: none of the 40 appear in the `class_feature_of_unmodelled_corpus_class`
evidence-code population (all 10 classes are already in `modelled_class_books()`'s registry from the
first T12 cycle — confirmed by direct membership check against
`tests/fixtures/rules_core/untabled-base-class-chassis.json`'s own 20-entry list).

**No unit was promoted to `grounded`** — the same STRICT check that excludes every `.corpus_record.`
roster id from `grounded` (confirmed by direct read of `non_roster_ids()`'s filter, unmodified by
this cycle) applies identically to shape-2 ids; this cycle never claims a magnitude the engine does
not compute. The other 196 of the 236 remain honestly `not-ingested`: 55 are magnitude-bearing
records needing a real per-feature compute function (the roster id alone cannot credit them, proven
live by the same strict-check exclusion); 141 are records this fixture's own scope does not cover
(no `.MOD` or level-table-row shape found for that specific key, or belonging to one of the 7
zero-coverage classes).

**Gate 3's `no_record`/`not_ingested` budget is unaffected in aggregate** (this cycle does not
modify the budget constants, does not write `docs/work-inventory.json`, per this dispatch's own
hard rule — measurement only).

## Mechanism-sized plan for the remainder (not a per-class list)

1. **7 classes still uncovered by either shape**: `kineticist`, `medium`, `mesmerist`, `occultist`,
   `psion`, `psychic`, `spiritualist`. A third progression convention, not investigated this cycle —
   real next-cycle scoping question, not guessed at here.
2. **The 55 magnitude-bearing records under the 10 now-shape-2-covered classes** need real
   per-feature compute functions — genuinely per-feature work, not a generic pass, matching the
   prior cycle's identical finding for Antipaladin/Magus/Vigilante's own 25.
3. **Pool-shaped groups** (`Aegis Customization`, `Shifter's Aspect` pool entries, ...) remain
   entirely out of this mechanism's scope by design, same as the prior cycle.
4. **The 82-small tier vs 11-large tier split named in the original brief no longer describes the
   corpus.** Re-derive T12's own population and false-positive vocabulary before trusting either
   figure in a future dispatch — this cycle found both had drifted materially (2,397→1,009 real
   population; 118→98 false positives) from concurrent sibling-lane activity on this branch, not
   from this cycle's own work.

## What this cycle did NOT do

No corpus data or `docs/work-inventory.json` changed (measurement only, per §16). Kanban row 11
stays `in-progress`. The `class_chassis.*` ids and `Kind::Class`-layer attribution (already fixed by
the first T12 cycle) are untouched. The other four open sub-shapes under card 11 (T2b, T9,
T2a-residual, T4-L9) are untouched.

## Next-cycle plan

1. Investigate the progression shape for the 7 still-uncovered registry classes (start with
   `psion`/`psychic`/`spiritualist` as a plausible shared psionics-manifester convention distinct
   from `psychic_warrior`'s own, then `kineticist`/`medium`/`mesmerist`/`occultist` individually).
2. Build real magnitude functions for the highest-value of the now-80 total identified
   magnitude-bearing records (25 from the prior cycle + 55 from this one) across the 13 covered
   classes.
3. Escalate the 11-large tier's magnitude-bearing cost as a named, mechanism-sized plan once the
   remaining classes' progression shapes are known — do not start eleven subsystems inside a
   measurement-scoped cycle.

`df -h /`: 651G available / 968G total, 33% used (unchanged order of magnitude from prior receipts).
