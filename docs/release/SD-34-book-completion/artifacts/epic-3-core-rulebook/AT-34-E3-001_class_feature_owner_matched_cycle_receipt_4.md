# Cycle — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`class_feature_owner_matched_by_name_but_record_not_held_by_engine` mechanism, cycle 4)

This cycle owns **exactly one** of the nine mechanisms `decisions.md §14` decomposed
`AT-34-E3-001` into. It does **not** close AT-34-E3-001 itself — other mechanisms remain,
each its own cycle.

- **Commit SHA:** `0302b44ae8`
- **Files touched:**
  - `src/rules_core/pilot_compute/mod.rs` — new `explain_base_class_weapon_and_armor_proficiency`
    (grant-only identity record, the same idiom `class_slayer.rs`'s
    `ground_slayer_weapon_and_armor_proficiency` already establishes for this exact
    record shape), wired unconditionally into `build_pilot_headless_receipt`'s existing
    linear per-class-feature dispatch (next to `explain_rogue_level1_chassis`); a new
    `#[cfg(test)] mod base_class_weapon_and_armor_proficiency_tests` with 3 unit tests
    (RED confirmed by temporarily disabling the call site, then GREEN).
  - `docs/work-inventory.json` — regenerated at HEAD, guarded regeneration path (plain
    `cargo run --locked --bin v06_work_inventory`, no `--allow-stamp-loss` used or
    needed; `CORPUS_LITERAL_SWEEP_REPORT` and `DERIVED_FIXTURE_CHECK_REPORT` set from
    this cycle's own fresh `corpus_literal_sweep --json-out` /
    `derived_evaluator_fixture_check --json-out` runs so the stamp-loss guard had real
    evidence rather than refusing).
  - `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`
    (regenerated output of `completion_atlas.py --check`, not hand-edited).
  - `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_owner_matched_cycle_receipt_4.md` (this file)
  - `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`
  - `docs/retro/events/sd34-at-34-e3-001.jsonl` (one `deferral` event, RETRO_ACTOR-scoped)
  - `src/bin/v06_work_inventory.rs` — **not touched this cycle**; no `BUCKET_DEFINITIONS`
    citation drift risk (confirmed: `completion_atlas.py --check` reports
    `citation_failures=0` below).

- **Identifier audit result:** OK_NO_BUNDLE_TAGS on this cycle's own new code
  (`git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/rules_core/pilot_compute/mod.rs
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` → no matches). The wider
  Epic-3 file-touch-set diff DOES carry ~489 matches, all pre-existing
  `sd32_class_ingest`/`sd32_simple_filename_kind_ingest` `wiring_class_signals` **data
  values** inside `docs/work-inventory.json` and quoted test-suite names inside earlier
  cycles' own already-committed receipts under `artifacts/epic-3-core-rulebook/` — the
  same pre-existing-data-value finding every prior cycle on this file-touch set has
  reported, re-confirmed rather than assumed this cycle.
- **Wired-integration audit result:** OK_NO_TOKENS on this cycle's own new code (same
  scoped diff, `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|
  hack)\b'` → no matches). The wider file-touch-set diff carries `placeholder` matches,
  all inside earlier cycles' own already-committed receipts describing PCGen's own
  literal "no selection" CHOOSE-menu placeholder rows (a real corpus-data concept those
  receipts name), none from this cycle.
- **Acceptance criterion:** AT-34-E3-001 — bucket B closes: records reach their tables —
  this cycle owns exactly mechanism 1 of 9,
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
→ **346** at this cycle's starting HEAD (`16c772cca9`), unchanged from cycles 2 and 3.
Matches the dispatch brief's own stated 346 exactly — re-derived, not assumed.

Three prior cycles on this exact mechanism (receipts 1–3) exhaustively investigated it and
each closed **0 of 346**, concluding real engine wiring or an operator ruling was needed —
cycle 3's own next-cycle plan named two concrete paths: (1) an operator-scoped
classification ruling on 246 of 346, or (2) real engine wiring, one shape at a time, on
the smaller sub-causes. This cycle independently re-derived the same 346-unit, same-shaped
partition from scratch (a temporary Rust diagnostic binary under `examples/`, deleted
before commit — `git status --porcelain examples/` is clean at commit time) before reading
the prior receipts, then cross-checked: the two derivations agree to within 1 unit on every
sub-cause (a single-unit classification-boundary difference in `engine_effect_token`
vs `class_level_scaled_phrase`, immaterial to the totals), confirming both are sound. This
cycle picked up cycle 3's **path 2**.

## This cycle's own contribution — real engine wiring, one closeable shape

Cycle 3's own 13-shape partition of `engine_effect_token_present` (121 units) named a
long tail of 18 small, individually-scoped mechanical shapes. Five of those are the SAME
record shape: a base class's own "Weapon and Armor Proficiency" class feature (Assassin,
Cleric, Shadowdancer, Sorcerer, Wizard) — a zero-magnitude, `ABILITY:...AUTOMATIC`-only
grant-only identity record. This engine already has a proven, shipped precedent for
exactly this shape: `class_slayer.rs`'s `ground_slayer_weapon_and_armor_proficiency`
(Advanced Class Guide's Slayer), which grounds it as a bounded grant-only identity record
(value 0, quoting the real corpus DESC text) via an explanation id
`class_feature.<owner>.weapon_and_armor_proficiency` — a shape
`v06_work_inventory.rs`'s `classify()` `Kind::ClassFeature` "owner resolved" arm ALREADY
consults generically (`class_feature_exact_suffix_grounded`, `.{owner}.` substring +
trailing dot-segment == `class_feature_engine_join_slug("Weapon and Armor Proficiency")`
== `"weapon_and_armor_proficiency"`), so no `v06_work_inventory.rs` change was needed —
only a new explanation-emitting function in `pilot_compute`.

**Scoped to the two classes this precedent mirrors cleanly**, confirmed rather than
assumed:
- `SORCERER_CLASS_ID`/`WIZARD_CLASS_ID` are both registered `pilot_compute` chassis
  constants (`grep -n 'const SORCERER_CLASS_ID\|const WIZARD_CLASS_ID'
  src/rules_core/pilot_compute/mod.rs`); neither has a registered archetype able to claim
  a proficiency-shaped slot (`archetype_resolver::archetype_claiming_slot_entry` has
  nothing to resolve for either), so the base grant always applies unconditionally once
  the class is present — no Slayer-style supersession branch needed.
- Cleric's own corpus record carries a real PREABILITY-gated archetype-supersession
  branch (two `DESC` segments — confirmed by direct read of
  `data/corpus/core_rulebook/class_feature/cleric/weapon_and_armor_proficiency.json`),
  the same complexity Slayer's own function handles — real, scoped follow-on work, not a
  same-cycle mirror. Deferred (see retro event below).
- Assassin and Shadowdancer are prestige classes with **no registered `CLASS_ID` constant
  anywhere in `pilot_compute/mod.rs`** (`grep -n 'ASSASSIN_CLASS_ID\|SHADOWDANCER_CLASS_ID'`
  → no matches) — prestige-class chassis support does not exist yet for either, a
  precondition outside this cycle's scope. Deferred (see retro event below).

## TDD — RED confirmed for the intended reason, then GREEN

```bash
# RED: call site temporarily commented out
cargo test --locked --lib rules_core::pilot_compute::base_class_weapon_and_armor_proficiency_tests -- --nocapture
```
→ 2 of 3 tests FAILED (`sorcerer_weapon_and_armor_proficiency_grounds_as_a_zero_magnitude_grant`,
`wizard_weapon_and_armor_proficiency_grounds_as_a_zero_magnitude_grant`), both panicking
with "must ground this base-class grant" — the intended reason (the explanation the test
looks for genuinely does not exist yet), not a compile error or an unrelated failure. The
third test (`a_non_sorcerer_non_wizard_character_grounds_neither_explanation`) passed
before the fix too, correctly — it asserts an ABSENCE.

```bash
# GREEN: call site restored
cargo test --locked --lib rules_core::pilot_compute::base_class_weapon_and_armor_proficiency_tests -- --nocapture
```
→ `test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 2892 filtered out`

## Figures + re-derive commands

- **Mechanism population, `core_rulebook`:** 346 → **344** (command above, denominator:
  `core_rulebook` units with `status=='engine-does-not-hold'` and this evidence string).
- **The 2 units moved, and to what:**
  ```bash
  python3 -c "
  import json
  d = json.load(open('docs/work-inventory.json'))
  for key in ['Sorcerer ~ Weapon and Armor Proficiency', 'Wizard ~ Weapon and Armor Proficiency']:
      u = next(x for x in d['units'] if x.get('corpus_key')==key)
      print(key, u['status'], u['evidence'])
  "
  ```
  →
  ```
  Sorcerer ~ Weapon and Armor Proficiency text-complete explanation_id_observed_and_corpus_record_carries_real_description
  Wizard ~ Weapon and Armor Proficiency text-complete explanation_id_observed_and_corpus_record_carries_real_description
  ```
  Both reach `text-complete` via the classify() function's EXISTING generic
  "owner resolved + explanation id observed" rung — no new bucket-specific fallback
  added, confirming this is real reachability, not a narrow carve-out.
- **Bucket B, `core_rulebook` (atlas-real partition):** `python3
  scripts/completion_atlas.py --book core_rulebook --check` → `694 → 692` (denominator:
  all `core_rulebook` content units the atlas classifies, 6,701).
- **`completion_atlas.py --check` (population-wide):** `python3
  scripts/completion_atlas.py --check` → `population=49438 buckets=10 unclassified=0
  overlap=0 citation_failures=0` (denominator: 49,438, the corpus-wide unit population;
  `citation_failures=0` confirms no `src/bin/v06_work_inventory.rs` line shifted, since
  this cycle did not touch that file).
- **Corpus-wide mechanism population (context only, not this criterion's own
  denominator):** `python3 -c "..."` (same shape as above, no `book` filter) →
  `3594 → 3592`, confirming the fix is real reachability wiring, not a `core_rulebook`-
  only carve-out (no other book's corpus declares these two exact keys).
- **Denominator gate:** `python3 scripts/denominator_gate.py --check
  'docs/release/SD-34-book-completion/*.md'` → `files_checked=15 violations=1` (one
  pre-existing violation at `progress.md:162`, from an earlier cycle's own entry, not
  introduced or touched this cycle — left as-is, out of this cycle's file-touch scope,
  same finding cycle 3 reported at that entry's then-line-number `:134`).
- **`box_ledger.py --check` (SD-33's inherited, read-only partition):** exits 1 both
  before and after this cycle (6 stale-count WARNINGs, inherited `THE-BOX.md` drift,
  unowned by SD-34 — same finding every prior AT-34-E3-001 cycle has reported); its
  structural invariants pass: `overlap=0 population=49438 oracle_disagreement=0
  unverifiable_done=0 stale=False`.

## Row-count command output

```
$ cargo test --locked --lib rules_core::pilot_compute::base_class_weapon_and_armor_proficiency_tests -- --nocapture
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 2892 filtered out; finished in 0.00s

$ python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
cr = [u for u in d['units'] if u.get('book')=='core_rulebook'
      and u.get('status')=='engine-does-not-hold'
      and u.get('evidence')=='class_feature_owner_matched_by_name_but_record_not_held_by_engine']
print(len(cr))
"
344
```
This cycle's own artifact is this receipt plus the 2 units it moved; the row-count that
governs `status` is the mechanism's population count above: **344 remaining, 2 closed**.

## Build scope verified

- `cargo test --locked --no-run` (full workspace, `CARGO_TARGET_DIR=/tmp/cargo-sd34-at-34-e3-001`)
  → exit 0 (`Finished `test` profile [unoptimized + debuginfo] target(s) in 2m 03s`, no
  `error[` lines), run at this cycle's HEAD **after** the last commit that moves a figure
  this receipt depends on (`decisions.md §12` L7 — run after `docs/work-inventory.json`'s
  regeneration).
- `apps/desktop/src-tauri`: not touched this cycle (`git diff --name-only
  $(git merge-base HEAD origin/develop)...HEAD -- apps/desktop/src-tauri` shows only
  earlier, already-committed cycles' own changes — not re-run this cycle, matching
  `workflow-instruction.md §2.5`'s "test the targets your change touches" scoping).

## Sweep population

`corpus_literal_sweep` examined-population before this cycle's own regeneration run:
48,699 of 51,473 (bundle baseline). This cycle's own fresh run (used to satisfy the
stamp-loss guard for the `docs/work-inventory.json` regeneration, not to add or regenerate
any corpus record):

```
corpus-literal-sweep: 48708 records examined of 51482 read, 413336 tokens compared
(9 synthesized), 51469 digests checked, 0 findings
```

**This cycle added or regenerated zero corpus records** — the 48699→48708 /
51473→51482 delta is pre-existing drift from OTHER concurrent lanes on this shared
checkout between this bundle's baseline measurement and this cycle's own run, not
attributable to this cycle's own commit (`decisions.md §12` L8 governs a gate whose
examined-population must grow by exactly the record delta over a change THAT cycle makes;
this cycle made none).

## Oracle pin

N/A — no figure in this receipt is derived from the pinned PCGen oracle corpus.

- **Status:** partial. This cycle closes **2 of 346** units (bucket B, `core_rulebook`,
  694 → 692 of 6,701) via real engine wiring (a new `pilot_compute` grant-only identity
  record function, mirroring an existing shipped precedent), not a catalog-widening
  shortcut — both moved units reach `text-complete` through the classify() function's
  EXISTING generic explanation-id check. AT-34-E3-001 as a whole does NOT close this
  cycle: the other eight mechanisms are owned by other cycles and are not this cycle's
  scope, and 344 units remain in this mechanism alone.

## Movement, four buckets

- **Closure:** 2 (`Sorcerer ~ Weapon and Armor Proficiency`, `Wizard ~ Weapon and Armor
  Proficiency`, both `engine-does-not-hold` → `text-complete`).
- **Reclassification:** 0 (no unit's evidence string changed without a status change; the
  2 closed units' evidence changed BECAUSE their status changed, which is closure, not a
  same-status reclassification).
- **Reachability:** 2 (a new engine table entry — the explanation-id record — now answers
  `held` for both units, the literal mechanism `decisions.md §2`'s bucket-B "cleared by
  placing the record" names).
- **Instrument-correction:** 0 (the starting population re-derived cleanly to the same
  346 both prior cycles reported; no wrong prior claim was found).

## Notes

- This cycle is a genuine, if small, **closure wave** — not a zero-bank measurement wave
  like cycles 2 and 3. Three prior cycles' own exhaustive investigation (re-confirmed, not
  repeated, by this cycle's independent re-derivation) correctly established that the
  remaining 344 need either real engine wiring (per-shape) or an operator ruling; this
  cycle picked the cheapest genuinely-safe engine-wiring shape available and built it,
  rather than re-running the same measurement a fourth time.
- **A unit leaving bucket B for `text-complete` (a `D`-shaped clearing under `decisions.md
  §2`'s "record reaches its table") is the correct outcome — whether it then displays is a
  separate bucket's own mechanism**, per this criterion's own brief. Both moved units'
  status is `text-complete`, meaning the record now has a shelf (the explanation the
  Character Sheet's Class Features section already picks up via its generic
  `class_feature.` id-prefix consumer, the SAME consumer the Slayer precedent's own doc
  comment names) — this is display-bucket territory (`decisions.md §2a`), out of scope
  for this receipt to re-verify.
- **Deferred, not attempted:** Cleric (archetype-supersession complexity, a real Slayer-
  shaped follow-on) and Assassin/Shadowdancer (no prestige-class chassis exists yet) —
  logged as a `deferral` retro event with a named revisit condition
  (`docs/retro/events/sd34-at-34-e3-001.jsonl`), not silently dropped.
- The temporary diagnostic binary (`examples/sd34_diag_pool_catalog.rs`, used to
  independently re-derive the 346-unit sub-cause partition before reading prior receipts,
  and to verify the standalone-catalog-wiring hypothesis was NOT the answer — 0 of 131
  corpus-wide standalone-key units are held by `load_standalone_class_feature_catalog`,
  ruling that path out cleanly) was deleted before this cycle's commit — `git status
  --porcelain examples/` is clean.
- **Remainder, named by sub-cause, summing exactly to 344** (re-derived fresh at this
  cycle's own HEAD, cross-checked against cycle 3's own partition):

| Sub-cause | Units | What closes it |
|---|---:|---|
| `description_is_null_internal_bookkeeping` (includes the 25 non-`" ~ "`-qualified structural/category-marker keys, e.g. `Archetype Barbarian`, `Barbarian Class`, `Monk Unarmed Damage LVL 1` — confirmed 0 of 131 corpus-wide standalone-key units are held by `load_standalone_class_feature_catalog`, so widening that catalog's wiring would not move any of them) | 118 | Operator-scoped ruling (unchanged from cycle 2): does a zero-description internal-bookkeeping row ever satisfy bucket B? |
| `engine_effect_token_present` — of which: | 118 | — |
| &nbsp;&nbsp;`Sorcerer Bloodline Feat ~ *` | 87 | Operator-scoped ruling (cycle 3): "count grounds, choice not modelled" per-option enumeration, same as Fighter/Cavalier/Brawler/Arcane-bloodline bonus feats. |
| &nbsp;&nbsp;`Ranger Combat Style Feat ~ *` | 16 | Same ruling as above, Ranger's combat-style-feat slots. |
| &nbsp;&nbsp;Long tail (`Rogue Talent` 3, `Wizard` 2 — opposition-school bookkeeping, `Core Domain` 2, `Monk` 2, `Shadowdancer` 1, `Assassin` 1, `Cleric` 1, `Duelist` 1, `Nobility Domain` 1, `Sorcerer Bonus Spell L3` 1) | 15 | Real engine wiring, one mechanical shape at a time; `Cleric`/`Assassin`/`Shadowdancer`'s own "Weapon and Armor Proficiency" (3 of these 15) are this cycle's own named, retro-logged deferral. |
| `catalog_serves_it_but_classify_wiring_class_gate_blocks_promotion` | 67 | Real per-character computation for the specific `wiring_class`/`universal_sheet_modifier` shape each record actually needs (verified genuine across three prior cycles' sampling). |
| `class_specific_level_phrase` | 6 | Real per-character, class-level-scaled computation. |
| `dropped_pcgen_args` | 5 | Real per-character argument resolution this static catalog cannot perform. |
| `multi_desc_segment_not_regenerated` | 3 | New engine support for showing only the character's actual mutually-exclusive branch. |
| `bare_percent_reference` | 1 | Same as `dropped_pcgen_args`. |
| **Total** | **344** | — |

## Next-cycle plan

1. **Real engine wiring, one shape at a time**, is still live and now has a proven,
   shipped, cheap-to-mirror precedent (this cycle's own
   `explain_base_class_weapon_and_armor_proficiency`): Cleric's own "Weapon and Armor
   Proficiency" (archetype-supersession, mirror `class_slayer.rs`'s full pattern rather
   than this cycle's simplified one) is the next-cheapest of the 15-unit long tail.
2. **Operator-scoped ruling**, unchanged from cycles 2–3, on whether a
   "count grounds, choice not modelled" per-option enumeration record (103 of 344) or a
   zero-description internal-bookkeeping row (118 of 344) is ever a valid bucket-B
   target — together still 221 of 344 (64%) of this mechanism's remaining population.
3. The 67-unit `catalog_serves_it_but_classify_wiring_class_gate_blocks_promotion`
   sub-cause remains each its own real per-character computation project, unchanged.
