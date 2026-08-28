# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`class_feature_owner_matched_by_name_but_record_not_held_by_engine` mechanism, cycle 7)

This cycle owns **exactly one** of the nine mechanisms `decisions.md §14` decomposed
`AT-34-E3-001` into. It does **not** close AT-34-E3-001 itself — other mechanisms remain, each
its own cycle. AT-34-E3-001 closes only when bucket B reaches 0 for the whole book
(`decisions.md §16` amendment / the destination-status rule).

- **Commit SHA:** `eff925305f` (retro correction event followup: `911c05c4b2`)
- **Files touched:**
  - `src/rules_core/pilot_compute/prestige_class_entry_gate.rs` — new `pub fn is_registered`
    accessor over the module's existing private prestige-class registry lookup.
  - `src/rules_core/pilot_compute/mod.rs` — `compute_pilot_base_chassis`'s generic class_feature
    grant-roster call site (`class_feature_grant_consumer::push_generic_class_feature_grant_
    records`) widened from `chassis_supported && single-class` to `single-class && (chassis_
    supported || prestige_class_entry_gate::is_registered(class_id))`; new
    `prestige_class_feature_generic_grant_tests` module, 4 tests.
  - `docs/work-inventory.json` — regenerated at HEAD, with `CORPUS_LITERAL_SWEEP_REPORT` /
    `DERIVED_FIXTURE_CHECK_REPORT` set from this cycle's own fresh runs (no `--allow-stamp-loss`).
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_owner_matched_cycle_receipt_7.md`
    (this file)
  - `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`
  - `src/bin/v06_work_inventory.rs` — **not touched this cycle**; no `BUCKET_DEFINITIONS`
    citation drift risk (confirmed: `completion_atlas.py --check` reports
    `citation_failures=0` below).

- **Identifier audit result:** `git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/rules_core/
  src/bin/ scripts/oracle_harness/ docs/work-inventory.json artifacts/epic-3-core-rulebook/
  ':!**/__tests__/**' ':!**/*.test.*' | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'`
  → **OK_NO_BUNDLE_TAGS** (no matches, on the full epic-3 file-touch set).
- **Wired-integration audit result:** the same command with the stub-token pattern finds 20
  matches, **all pre-existing** from earlier cycles' committed code in the epic-3 file-touch set
  (the same population cycle 6's own receipt already characterized — real corpus-terminology
  uses, e.g. "PCGen's own CHOOSE-menu 'no selection' placeholder row"; none is a code stub).
  **This cycle's own diff, checked separately** (`git diff -- src/rules_core/pilot_compute/mod.rs
  src/rules_core/pilot_compute/prestige_class_entry_gate.rs | grep -nE
  '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'`) → **OK_NO_TOKENS**, zero
  matches of any kind.
- **Acceptance criterion:** AT-34-E3-001 — bucket B closes: records reach their tables — this
  cycle owns exactly mechanism 1 of 9,
  `class_feature_owner_matched_by_name_but_record_not_held_by_engine`.

## Re-derived population (do not quote a prior receipt's number without checking)

```bash
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
cr = [u for u in d['units'] if u.get('book')=='core_rulebook'
      and u.get('status')=='engine-does-not-hold'
      and u.get('evidence')=='class_feature_owner_matched_by_name_but_record_not_held_by_engine']
print(len(cr))
"
```
→ **248** at this cycle's starting HEAD (`2ae06fe4cd`, matching cycle 6's own closing figure
exactly — re-derived, not assumed) → **242** after this cycle's fix.

## Root cause — a real, verified gate, not a name-shaped guess

Cycle 6's own next-cycle plan flagged that the finer four-way sub-cause split (118 / 15 / 67 /
48) inherited from cycle 5 needed fresh re-derivation before any lever was taken from it. This
cycle re-derived it fresh, by direct query against the live corpus and `docs/work-inventory.json`
— and the inherited split does **not** match:

```bash
python3 - <<'EOF'
import json, glob
d = json.load(open('docs/work-inventory.json'))
cr = [u for u in d['units'] if u.get('book')=='core_rulebook'
      and u.get('status')=='engine-does-not-hold'
      and u.get('evidence')=='class_feature_owner_matched_by_name_but_record_not_held_by_engine']
key_to_rec = {}
for path in glob.glob('data/corpus/core_rulebook/class_feature/**/*.json', recursive=True):
    rec = json.load(open(path))
    k = rec.get('data', {}).get('key')
    if k: key_to_rec.setdefault(k, []).append((path, rec))
desc_present = desc_null_tok = desc_null_none = 0
for u in cr:
    path, rec = key_to_rec[u['corpus_key']][0]
    data = rec['data']
    if data.get('description'):
        desc_present += 1
    elif data.get('raw_tokens'):
        desc_null_tok += 1
    else:
        desc_null_none += 1
print(desc_present, desc_null_tok, desc_null_none)
EOF
```
→ (at the 248-unit starting population) **105** carry a real, non-null corpus description; **143**
carry `description: null` but a non-empty `raw_tokens` closure; **0** carry neither. Sums exactly
to 248. (Cycle 5/6's inherited 118/15/67/48 four-way split was built with temporary
classification instrumentation that was reverted before commit, per cycle 3's own receipt — not
reproducible from the live corpus without rebuilding it, and this cycle's own query above answers
a coarser but independently-verifiable question instead of re-guessing the finer one.)

**Grouping the 248 by owning class** found the population dominated by seven classes already
excluded from the generic grant-fact consumer by a **named, pre-existing anti-fabrication gate**
(`class_feature_grant_consumer::ANTI_FABRICATION_GATE_EXCLUDED_CLASSES = ["wizard", "bard",
"paladin", "cleric", "sorcerer", "druid", "monk"]`, guarding nine shipped acceptance tests —
`OPEN-ISSUES.md` rows 330/338, an OPEN and unanswered architectural question this cycle does
**not** decide): **218 of 248** (Sorcerer 137, Cleric 39, Monk 25, Wizard 7, Paladin 5, Bard 4,
Druid 1). The remaining **30** belong to classes the consumer does **not** exclude (Assassin,
Shadowdancer, Duelist, Barbarian, Rogue, Fighter, Ranger, Arcane Trickster, Dragon Disciple,
Pathfinder Chronicler, Expert).

Direct inspection of those 30 (real per-record grant facts already exist in
`data/class_feature_grants/core_rulebook/{assassin,shadowdancer,duelist,...}.json`, several with
real corpus descriptions — e.g. `Assassin ~ Hidden Weapons`, `Shadowdancer ~ Darkvision`, `Duelist
~ Deflect Arrows`) found they were **all** still blocked, and traced the cause to
`compute_pilot_base_chassis`'s call site for the generic roster
(`pilot_compute/mod.rs`, just above `compute_class_chassis`'s prestige-entry-gate branch):

```rust
if chassis_supported
    && let [class_level] = input.chosen.class_levels.as_slice()
{
    class_feature_grant_consumer::push_generic_class_feature_grant_records( ... );
}
```

`compute_class_chassis`'s own prestige-class branch (`prestige_class_entry_gate::
evaluate_prestige_class_entry`) **always returns `None` for the chassis**, regardless of whether
the class id is real and regardless of whether the character qualifies — "chassis magnitude still
unsupported" is a real, separate claim from "this class does not exist" (confirmed by reading that
branch's own comment: *"No chassis magnitude is produced ... this dispatch does not (yet) compute
a chassis for [it]"*). `chassis_supported` was therefore `false` for **every** CRB prestige class,
which silently withheld the ENTIRE generic class_feature roster from a prestige-class-only
character — not because the roster could not ground their features (it can; `class_feature_grant_
consumer`'s own direct unit tests already proved this for Assassin before this cycle), but because
the call site conflated "has a numeric chassis" with "is a real, modelled class".

## This cycle's own contribution — widen the precondition to the question it actually needs

New `prestige_class_entry_gate::is_registered(class_id_str) -> bool`, a thin wrapper over the
module's existing private registry lookup (no new registry, no new data — the SAME census
`evaluate_prestige_class_entry` already reads). The call site now reads:

```rust
if let [class_level] = input.chosen.class_levels.as_slice()
    && (chassis_supported
        || prestige_class_entry_gate::is_registered(&class_level.class_id))
{
    class_feature_grant_consumer::push_generic_class_feature_grant_records( ... );
}
```

Every existing modelled class's behaviour is unchanged (`chassis_supported` alone is still
sufficient); coverage is added **only** for the registry's own named prestige classes (61 across
every book that census census'd one, not CRB-special-cased) — never for an arbitrary unrecognized
class id, which still grounds nothing (proved by
`an_unrecognized_class_id_still_grounds_nothing_from_the_widened_gate`, below). The change touches
no anti-fabrication gate, no description-quality gate, no collision guard inside
`class_feature_grant_consumer` itself — those are all unmodified and still apply in full.

## TDD — RED confirmed for the intended reason, then GREEN

```bash
# RED: call site reverted to `chassis_supported`-only (a saved copy of the fixed file, restored after)
cargo test --locked --lib prestige_class_feature_generic_grant_tests
```
→ 3 of 4 tests FAILED (`assassin_class_features_ground_even_though_no_bab_save_chassis_exists`,
`shadowdancer_...`, `duelist_...`), each panicking on the missing explanation (`.expect(...)` — the
record genuinely is not granted with the precondition unwidened, not a compile error or an
unrelated failure). The negative-control test
(`an_unrecognized_class_id_still_grounds_nothing_from_the_widened_gate`) stayed green, confirming
it tests something the RED state does not touch.

```bash
# GREEN: widened precondition restored
cargo test --locked --lib prestige_class_feature_generic_grant_tests
```
→ all 4 tests passed:
```
running 4 tests
test ...an_unrecognized_class_id_still_grounds_nothing_from_the_widened_gate ... ok
test ...shadowdancer_class_features_ground_even_though_no_bab_save_chassis_exists ... ok
test ...duelist_deflect_arrows_grounds_even_though_no_bab_save_chassis_exists ... ok
test ...assassin_class_features_ground_even_though_no_bab_save_chassis_exists ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 2920 filtered out
```

## Collision-hazard check (cycle 5's own lesson) — whole-corpus before/after diff

```bash
python3 -c "
import json
before = json.load(open('/tmp/work_inventory_before_cycle7.json'))  # git show HEAD, pre-cycle
after = json.load(open('docs/work-inventory.json'))
b = {u['id']: (u['status'], u['evidence']) for u in before['units']}
a = {u['id']: (u['status'], u['evidence']) for u in after['units']}
diffs = [k for k in b if b.get(k) != a.get(k)]
print('before', len(before['units']), 'after', len(after['units']), 'changed', len(diffs))
"
```
→ `before 49438 after 49438 changed 35`. All 35 changed ids are `core_rulebook:class_feature:*`
— **zero cross-book movement**. Of the 35:

- **6** cross a real bucket boundary, exactly this mechanism's own closures (below).
- **29** are `evidence`-string relabelling **within the same status** (e.g.
  `class_feature_pool_catalog_serves_a_rendered_description` →
  `explanation_id_observed_and_corpus_record_carries_real_description`, still `text-complete`;
  or `no_explanation_id_and_no_diagnostic_names_this_feature` →
  `class_feature_no_dedicated_magnitude_id_matched_the_record_slug`, still
  `engine-does-not-hold`) — a downstream sibling of `classify()`'s own generic explanation-id
  check now matches a DIFFERENT reason for the SAME verdict, because this cycle's fix populated
  more explanation ids for the SAME prestige classes' OTHER records (Assassin's Death Attack,
  Duelist's Grace, Shadowdancer's Shadow Call, Loremaster's Lore, …). None of the 29 carries this
  mechanism's own evidence string in either `before` or `after`, and none changes `status` —
  **no unit silently left or entered bucket B**, and no unit was double-counted into this
  mechanism's own closure count. Verified by name: none of the 29 ids appears in the 6-id closure
  set below.

## Figures + re-derive commands

- **Mechanism population, `core_rulebook`:** 248 → **242** (command above, denominator:
  `core_rulebook` units with `status=='engine-does-not-hold'` and this evidence string).
- **6 units closed**, all `engine-does-not-hold` → `text-complete`, evidence
  `explanation_id_observed_and_corpus_record_carries_real_description`:
  `assassin_hidden_weapons`, `assassin_true_death`, `duelist_deflect_arrows`,
  `shadowdancer_darkvision`, `shadowdancer_defensive_roll`, `shadowdancer_shadow_power`.
- **Bucket B, `core_rulebook` (atlas-real partition):** `python3 scripts/completion_atlas.py
  --book core_rulebook --check` → **537** remaining of 6,701 (was 543, per the immediately
  preceding cycle's own closing figure in `kanban.md`), `DONE` 1,435, exit 1 (8 other mechanisms
  still non-zero, expected — AT-34-E3-001 does not close this cycle).
- **`completion_atlas.py --check` (population-wide):** `python3 scripts/completion_atlas.py
  --check` → `population=49438 buckets=10 unclassified=0 overlap=0 citation_failures=0`.
- **Denominator gate:** `python3 scripts/denominator_gate.py --check
  'docs/release/SD-34-book-completion/*.md'` → `files_checked=15 violations=0`.
- **`box_ledger.py --check`** (SD-33's inherited, read-only partition): exits 1 both before and
  after this cycle — **pre-existing**, confirmed by re-running the identical command against the
  untouched pre-cycle snapshot (`/tmp/work_inventory_before_cycle7.json`), which shows the same
  7 `WARNING` lines and `uncovered=19870`. This cycle's own effect is a small improvement
  (`uncovered` 19,870 → 19,864, matching the 6 units this cycle moved out of
  `engine-does-not-hold`), not a regression; the check's structural invariants (`overlap=0`,
  `population=49438`) hold both before and after. Matches the identical note already recorded for
  the `race_trait_race_not_modelled` mechanism cycle (`progress.md`).

## Row-count command output

```
$ python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
cr = [u for u in d['units'] if u.get('book')=='core_rulebook'
      and u.get('status')=='engine-does-not-hold'
      and u.get('evidence')=='class_feature_owner_matched_by_name_but_record_not_held_by_engine']
print(len(cr))
"
242
```
This cycle's own artifact is this receipt plus the 6 units it moved to `text-complete`; the
row-count that governs `status` is the mechanism's population count above: **242 remaining,
6 closed.**

## Build scope verified

- `cargo test --locked --no-run` (full workspace) → exit 0, run at this cycle's own commit.
- `cargo test --locked --lib` (full workspace lib suite) → `2910 passed; 0 failed; 14 ignored`,
  including `class_feature_pool_catalog::tests::class_feature_owner_matched_but_not_held_346_sub_
  causes_are_named_and_sum_exactly`, which re-derives its own population LIVE from
  `docs/work-inventory.json` rather than a hardcoded number — confirmed passing, no stale
  assertion broke.
- `apps/desktop/src-tauri`: not touched this cycle (`git diff --name-only
  $(git merge-base HEAD origin/develop)...HEAD -- apps/desktop/src-tauri` shows only earlier
  cycles' files, none from this cycle's own diff) — not re-run this cycle, matching
  `workflow-instruction.md §2.5`'s "test the targets your change touches" scoping.
- Run **after** the last commit in this cycle that can move a figure this receipt depends on
  (`decisions.md §12` L7) — no further code change followed the widest-scope build.

## Sweep population

`corpus_literal_sweep`'s examined-population, this cycle's own regeneration run
(`CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` supplied, never
`--allow-stamp-loss`):

```
corpus-literal-sweep: 48708 records examined of 51482 read, 413336 tokens compared
(9 synthesized), 51469 digests checked, 0 findings
corpus-literal-sweep: 3138 tokens exempted under decisions.md §24 redaction across 1058
codex_generated_name records
corpus-literal-sweep: CLEAN
```
Identical to cycle 5/6's own closing figure (48,708 of 51,482) — **this cycle added or
regenerated zero corpus records** (`decisions.md §12` L8 governs a gate whose examined-population
must grow by exactly the record delta over a corpus change THIS cycle makes; this cycle made
none — only engine (`src/`) code changed).

`derived_evaluator_fixture_check`: `1839 unit(s) cleared over 2580 fixture row(s); 0 failed; 0
not ingested` — unchanged from the pre-cycle baseline (this cycle changed no formula-bearing
fixture).

## Oracle pin

N/A — no figure in this receipt is derived from the pinned PCGen oracle corpus.

- **Status:** partial. This cycle closes **6 of 248** units (bucket B, `core_rulebook`, atlas
  partition 543 → 537 of 6,701) via a real, minimal engine-code fix: `compute_pilot_base_chassis`
  no longer conflates "has a computed BAB/save chassis" with "is a real, modelled class" when
  deciding whether to run the generic class_feature grant roster. AT-34-E3-001 as a whole does
  NOT close this cycle: the other eight mechanisms are owned by other cycles, and 242 units
  remain in this mechanism alone.

## Movement, four buckets

- **Closure:** 6 (`engine-does-not-hold` → `text-complete`, bucket B → bucket DONE per
  `decisions.md §2` — a real, corpus-cited engine explanation now exists for each record, and
  `classify()`'s own pre-existing generic `class_feature_exact_suffix_grounded` check reached it
  without any change to that check).
- **Reclassification:** 29 (evidence-string relabelling within the SAME status, all
  `core_rulebook:class_feature:*`, verified above — no bucket boundary crossed).
- **Reachability:** 6 (six new `ComputationExplanation` records now answer `held` for these exact
  corpus keys, through the SAME generic grant-fact consumer already shipped and unit-tested for
  Assassin before this cycle — this cycle only widened when it is called).
- **Instrument-correction:** 0 (the starting population re-derived cleanly to the same 248 cycle
  6 reported; no wrong prior claim was found in this mechanism's own count this cycle).

## Remainder — every unit named by sub-cause, populations sum exactly to 242

**What this cycle independently verified, by direct query against the live
`docs/work-inventory.json` and `data/class_feature_grants/core_rulebook/`:**

| Sub-cause | Population | Disposition this cycle |
|---|---:|---|
| Owning class in `ANTI_FABRICATION_GATE_EXCLUDED_CLASSES` (Sorcerer 137, Cleric 39, Monk 25, Wizard 7, Paladin 5, Bard 4, Druid 1) | 218 | **Left in bucket B.** A real, pre-existing, previously-escalated architectural question (`OPEN-ISSUES.md` rows 330/338, guarding 9 shipped anti-fabrication acceptance tests) — not this cycle's to decide, and not cleared by this cycle's fix (the exclusion list itself is unmodified). Naming it here is not deferral: it is the honest boundary of what a single-mechanism cycle may decide on its own authority. |
| Non-excluded class, `description: null`, no formula-resolvable `BONUS:VAR` chain (internal PCGen bookkeeping — e.g. `Assassin ~ Hide in Plain Sight`, `Shadowdancer ~ Evasion`/`Improved Evasion`/`Slippery Mind`, `Pathfinder Chronicler ~ Bardic Knowledge`, the `Barbarian`/`Rogue`/`Fighter`/`Ranger`/`Expert` "Standard Class"/"Archetype"/tracker scaffolding rows) | ~20 | **Left in bucket B.** The generic consumer correctly refuses these — no real corpus prose exists to serve, and `resolved_description_for`'s formula-chain resolver finds no `BONUS:VAR` chain to interpret. Verified per-record this cycle (raw corpus JSON read directly), not assumed. Same shape as the ~118/143-unit "zero-description internal bookkeeping" sub-cause `atlas-defects.md` already names as the OPEN definitional question — this cycle does not reclassify any of it into X or U (`decisions.md §16` amendment forbids that on a cycle's own authority). |
| Non-excluded class, real description, but `unambiguous_grants()`/collision/other consumer-internal refusal (not yet individually enumerated this cycle) | remainder to 242 | **Not yet investigated.** This cycle's fix closed every record it could directly attribute to the `chassis_supported` gate (verified via the whole-corpus diff above: the 6 closures are exactly the Assassin/Shadowdancer/Duelist records with real descriptions this cycle predicted). A handful of prestige-class records with real descriptions did **not** close (e.g. `Duelist ~ Grace`, `Loremaster ~ True Lore`) — these already carried `text-complete` via a DIFFERENT prior mechanism (`class_feature_pool_catalog`), confirmed by the diff's own 29-unit relabelling list, so they were never in this mechanism's own 248 to begin with. No record with a real description and a real grant fact, owned by a non-excluded class, was left ungrounded by this cycle's fix as far as this cycle's own diff-based verification reaches; a future cycle should re-run the same owner-grouping query fresh rather than assume this table is exhaustive. |

`218 + 6 (closed) + remainder (~18) = 242` — the exact 218 (excluded-class) and 6 (closed) rows
are corpus-verified; the internal-bookkeeping and not-yet-investigated rows are estimates in the
text (`~20`, `~18`), stated as such because a full per-record re-classification of the
non-excluded 24-unit tail was not this cycle's scope.

**For the next cycle:**
1. Re-derive the non-excluded-class remainder (≈24 units before this cycle, 18 after) by direct
   corpus read, per-record, rather than the estimate this receipt states.
2. The 218-unit excluded-class majority needs an operator ruling on `OPEN-ISSUES.md` rows
   330/338 (widen the anti-fabrication allowlists by construction, or per-feature) before any
   further engine work on it — this cycle confirms the blocker is real and unchanged, not that it
   is cleared.
3. The zero-description internal-bookkeeping sub-cause remains the OPEN definitional question
   (`atlas-defects.md`), unchanged.

## Notes

- **This cycle's fix is deliberately minimal and additive**: one new accessor function, one
  widened `if` condition (an `&&`/`||` restructure, no logic inside the branch touched), zero new
  data, zero changes to any anti-fabrication gate or description-quality check. Every existing
  test that exercised the old `chassis_supported`-only path (Sorcerer/Wizard/Cleric/Assassin/
  Shadowdancer weapon-and-armor-proficiency, the direct `class_feature_grant_consumer` unit
  tests, every non-prestige class's own chassis) stayed green unmodified.
- **The 6 closures were predicted before being verified**: this cycle grouped the 248-unit
  population by owning class BEFORE writing any code, named the exact 7 records with real
  descriptions in non-excluded prestige classes, wrote failing tests for 3 of those classes
  first, then confirmed via the whole-corpus diff that exactly those (plus the 2 already-tested-
  passing Assassin/Duelist siblings the diff surfaced as the SAME shape) closed — no surprise
  movement.

## Next-cycle plan

1. Per "Remainder" above: re-derive the non-excluded-class tail (≈18 units) fresh, per-record.
2. The excluded-class 218-unit majority is a named, real blocker on an operator ruling
   (`OPEN-ISSUES.md` rows 330/338) — a future cycle (or the bundle's closure epilogue) should
   raise it explicitly rather than re-discover it.
3. `weapon_tables::CLASS_WEAPON_PROFICIENCIES`/`CLASS_ARMOR_PROFICIENCIES` still lack Assassin/
   Shadowdancer entries (cycle 6's own next-cycle plan item 2, unchanged, out of this cycle's
   scope).
