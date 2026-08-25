# Cycle epic-2-t12-modelled-class-books — Gate 3 (closure invariant) / Card 11, shape T12

- **Card ID:** `epic-2-cause-closure` (row 11)
- **Commit SHA:** see `git log -1` at push time (this cycle rebases before pushing per §5)
- **Files touched:**
  - `src/bin/v06_work_inventory.rs` — `modelled_class_books()` now registers the 20 entries of
    `pilot_compute::untabled_base_class_chassis::untabled_base_class_registry()` generically (one
    data-driven loop, zero per-class code); two new tests
    (`all_twenty_untabled_base_classes_are_registered_from_the_chassis_registry_itself`,
    `a_kind_class_record_for_a_newly_registered_class_reaches_grounded`) in
    `modelled_class_books_registry_tests`.
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 11 entry prepended,
    left `in-progress`.
  - `docs/retro/events/card11-t12.jsonl` — one `correction` (see below).

- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  (`git diff --unified=0 -- src/bin/v06_work_inventory.rs | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — no match)
- **Wired-integration audit result:** `OK_NO_TOKENS`
  (`git diff --unified=0 -- src/bin/v06_work_inventory.rs | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` — no match)

- **Acceptance criterion (verbatim, this dispatch brief):** "T12's population is literally
  unchanged by T2a's earlier fix... Attack the simpler tier generically first. If 80 classes share
  a chassis shape, build that shape once and drive it from data — the way `SIMPLE_FILENAME_KINDS`
  landed five kinds in one commit. Do not hand-model 80 classes." — **MET for the generic-mechanism
  half** (one data-driven registration, 20 classes, zero per-class code); **NOT MET for unit
  closure** — see "What this cycle did NOT close" below, reported honestly per `decisions.md §16`.

- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
  matches the pin; fresh worktree, oracle bootstrapped via `scripts/fetch-pcgen-oracle.sh` per §2.1,
  confirmed via `scripts/verify.sh --only preflight-oracle` equivalent SHA check before use).

- **Status:** complete (measurement + generic-mechanism cycle; row 11 stays `in-progress` — T12 is
  one of five sub-shapes under card 11 and is not itself closed)

## §17a re-derivation — the brief's own figures were stale

Every figure in the dispatch brief and in the prior census artifact
(`artifacts/gate-3-closure-invariant/card11-t12-census-census.md`) was re-derived fresh, per
`decisions.md §17a`'s standing rule, before being trusted:

```
python3 scripts/census_t12_class_feature.py
```

Result, against the pinned oracle (SHA unchanged) and the currently-committed
`docs/work-inventory.json`:

| Figure | Brief/census said | Re-derived (this cycle) | Delta |
|---|---:|---:|---|
| T12 evidence-code total | 2,453 | **2,515** | +62 |
| False positives (class A + B) | 118 | **118** (re-confirmed, unchanged) | 0 |
| Real T12 population | 2,335 | **2,397** | +62 |
| Distinct unmodelled classes | 92 | **93** | +1 |
| 11-large tier (units) | 1,331 | **1,332** (Magus 135→136) | +1 |
| Small tier (classes / units) | 81 / 1,004 | **82 / 1,065** | +1 / +61 |

**New class not in the prior census at all: Divine Scion, 46 units, `inner_sea_magic`.** The
false-positive figure held exactly (both class A's 80 and class B's 38 line up unit-for-unit against
the prior census's own table) — that finding is confirmed solid. The population drift is real, not
measurement noise: this branch carries many concurrent SD-32 lanes and the corpus/inventory moved
between the census cycle and this one. Logged: `scripts/retro.py correction`,
`docs/retro/events/card11-t12.jsonl`, id `1787467658748-card11-t12-e40d7d`.

**Consequence for this cycle's own scope:** the "11 large / 80 small" split in the dispatch brief is
retracted in favor of "11 large / 82 small", re-derivable by the command above at any time.

## The generic mechanism: wire the already-built chassis registry, don't hand-model

`src/rules_core/pilot_compute/untabled_base_class_chassis.rs` (landed earlier on this same branch,
card 12) is a real `compute_pilot_base_chassis` dispatch arm computing genuine BAB/save-progression
chassis for 20 classes from a corpus-derived fixture — but `v06_work_inventory.rs`'s
`modelled_class_books()` (the classifier's own "does the engine model a class of this name at all"
gate) never learned about it, so the classifier reported all 20 classes' `Kind::Class` AND
`Kind::ClassFeature` records `not-ingested` under the "engine models nothing" reason even though the
engine genuinely does hold and compute a chassis for them.

**Fix:** one loop in `modelled_class_books()` over
`untabled_base_class_chassis::untabled_base_class_registry()`'s own 20 entries, keyed off the
registry's own `class_id`/`source_book` fields. Zero per-class code — a 21st class registered in the
chassis registry costs nothing here. This is the `SIMPLE_FILENAME_KINDS` shape the brief asked for:
build the shape once (already built, by card 12), drive the classifier from the SAME data rather than
hand-adding 20 match arms.

11 of the 20 registered classes are in T12's own 11-large tier (Vigilante, Medium, Psychic, Magus,
Aegis, Occultist, Mesmerist, Shifter, Kineticist, Spiritualist, Psychic Warrior); the other 9
(Antipaladin, Cryptic, Dread, Marksman, Psion, Soulknife, Tactician, Vitalist, Wilder) are in the
small tier. So this single generic change touches **~65% of T12's real population by unit count**
(1,332 + a fraction of the small tier's 1,065, exact figure pending the roster mechanism below).

## RED → GREEN, proven and reverted

Both new tests in `modelled_class_books_registry_tests` proved RED first: temporarily replaced the
registration loop's iterator with a same-typed empty one
(`[].into_iter().chain(registry().iter().take(0))`), re-ran
`cargo test --locked --bin v06_work_inventory -- modelled_class_books_registry_tests`:

```
test all_twenty_untabled_base_classes_are_registered_from_the_chassis_registry_itself ... FAILED
  left: None  right: Some("ultimate_psionics")   (Aegis)
test a_kind_class_record_for_a_newly_registered_class_reaches_grounded ... FAILED
  left: "not-ingested"  right: "grounded"
```

Reverted to the real loop, re-ran: both GREEN. Both failures were for the intended reason (the map
missing the entry / the classify() dispatch never reaching the modelled branch), not an unrelated
break.

## Suites run

- `cargo test --locked --bin v06_work_inventory` (targeted, scoped per this dispatch's "scope your
  test runs" instruction): **318/318** (was 316 before this cycle; +2 new).
- `cargo test --locked --lib`: **2,397/2,397 passed, 13 ignored** (unchanged from the branch's own
  pre-cycle baseline — this change is additive-only at the classifier layer, confirmed by the full
  pass with no new failures).
- Desktop crate (`apps/desktop/src-tauri`, separate cargo workspace) **not re-run this cycle** — this
  cycle's change touches only `src/bin/v06_work_inventory.rs` (an offline census/classifier tool with
  no desktop-crate dependency); `git status --porcelain -- apps/desktop` confirms no file under that
  tree changed.

## Live re-derive: what actually moved, honestly reported (`decisions.md §16`)

Regenerated the full inventory in-memory (`cargo run --bin v06_work_inventory -- --stdout-only`,
same pinned oracle, **not written to `docs/work-inventory.json`** — this is a measurement, the
committed inventory is untouched, `git status --porcelain` confirms only the two files above
changed):

```
python3 -c "
import json
d = json.load(open('inventory_after.json'))
t12 = [u for u in d['units'] if (u.get('evidence') or '').startswith('class_feature_of_unmodelled_corpus_class')]
print(len(t12))
"
# -> 951   (down from 2,515 pre-change)
```

**1,564 units left the `class_feature_of_unmodelled_corpus_class:*` evidence code.** Per
`decisions.md §16` ("a unit moved out of a shape is not a unit closed... the receipt must say which
kind and prove it"), this is reported as a move, not a close:

```
python3 -c "
import json
from collections import Counter
d = json.load(open('inventory_after.json'))
c = Counter((u['status'], u['evidence']) for u in d['units'] if u.get('kind')=='class_feature')
for k,v in c.most_common(6): print(v, k)
"
# 4239 (not-ingested, no_explanation_id_and_no_diagnostic_names_this_feature)
# 4120 (not-ingested, class_feature_owner_matched_by_name_but_record_not_held_by_engine)
# 3091 (not-ingested, class_feature_option_pool_record_not_held_by_engine)
# 2587 (unknown, class_feature_group_names_no_class_at_all)
#  190 (text-complete, class_feature_pool_catalog_serves_a_rendered_description)
#   64 (text-complete, explanation_id_observed_and_corpus_record_carries_real_description)
```

**Why no `class_feature` unit reaches `grounded`/`text-complete` from this cycle alone:**
`classify()`'s `Kind::ClassFeature` arm requires an explanation id whose dot-segments include the
class name (`explanation_names_class`), or a `class_feature_pool_catalog` entry, before promoting a
now-owner-resolved record past a more specific `not-ingested` reason.
`untabled_base_class_chassis`'s dispatch arm emits only the generic ids
`class_chassis.base_attack_bonus` / `class_chassis.base_save.{fortitude,reflex,will}` — no class-name
segment — confirmed by direct read of `pilot_compute/mod.rs` lines ~26037-26069. This is the SAME
shape the CRB/APG/ACG dispatch arms use for their own base chassis (identical generic ids); those
classes' `class_feature` grounding comes entirely from their SEPARATE `ground_*_class_features`
calls, which this cycle's chassis-only registration does not add. So the honest gain this cycle is
confined to the `Kind::Class` layer's evidence reason (verified: `Kineticist`'s own class record
moves from `class_absent_from_ClassId_ALL_and_book_class_id_enums` to the more specific, and equally
`not-ingested`, `class_modelled_but_no_observed_delta_on_the_rendered_snapshot` — no false `grounded`
claim; `Kind::Class`'s `grounded` count is unaffected by this cycle, 28 both before and after,
confirmed by direct count).

**No unit is claimed closed by this cycle.** Gate 3's `no_record`/`not_ingested` budget is unaffected
in aggregate (same total not-ingested population, redistributed across more specific, more actionable
reasons) — this cycle does not modify the budget constants, per this dispatch's own hard rule.

## Mechanism-sized plan for the remainder (not a per-class list)

The next real lever, named and not guessed: a generic roster mechanism shaped exactly like
Pathfinder Unchained's own `push_pu_class_feature_records` (`pilot_compute/mod.rs`) — one
per-corpus-record explanation id (`class_feature.<class>.corpus_record.<slug>`), emitted from a
corpus-derived fixture (record key + granted level; text-only records need no magnitude function at
all under `decisions.md §7`'s zero-magnitude rule, so this alone reaches `text-complete` for the
text-only majority). Wired at the SAME dispatch site this cycle's registry loop touches
(`untabled_base_class_chassis`'s arm in `compute_pilot_base_chassis`). This is a second generic,
data-driven build (one fixture schema, one push function, reused across every class it covers), not
93 per-class cycles.

- **11-large tier:** unchanged risk assessment from the census — no shared mechanic across the 11
  (spellstrike, burn economy, implement points, spirit possession, customization points, ...); each
  still needs its own subsystem for its magnitude-bearing features. The roster mechanism above lowers
  the TEXT-ONLY share of their cost but does not remove the 11 separate subsystem builds. Still the
  single largest remaining content-build risk in the bundle.
- **82-small tier:** the roster mechanism is a plausible full-closure path for most of it (narrow,
  largely auto-granted feature progressions). **Recommended next-cycle proof case:** build the roster
  mechanism against 1-2 already-chassis-registered small classes (Antipaladin or Cryptic) before
  batching the rest — proves the mechanism against real data before committing to it at scale, the
  same discipline this bundle applied to the chassis registry itself.

## What this cycle did NOT do

No corpus data, `docs/work-inventory.json`, or Gate 3 budget constant changed. Kanban row 11 stays
`in-progress` (T12 is one of card 11's five open sub-shapes; the other four — T2b, T9, T2a-residual,
T4-L9 — are untouched by this cycle). No fixture regeneration attempted (this cycle's change is
classifier-side, not corpus-ingestion-side, so no guarded-generator regen applies).

## Next-cycle plan

1. Build the roster-explanation mechanism (fixture + push function) against Antipaladin or Cryptic as
   the proof case.
2. Re-run this cycle's own re-derive command; confirm real `text-complete`/`grounded` closures this
   time (not just an evidence-reason move), fixture-checked against the pinned oracle.
3. Batch the remaining small-tier classes once the mechanism is proven.
4. Escalate the 11-large tier as a named, mechanism-sized plan (one subsystem per class, no shared
   shortcut) rather than attempting per-class builds inside a measurement-scoped cycle.

`df -h /`: see final report.
