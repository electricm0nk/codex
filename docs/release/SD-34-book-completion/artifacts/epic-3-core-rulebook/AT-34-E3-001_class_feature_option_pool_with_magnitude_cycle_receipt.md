# Cycle 1 — Epic 3 (Core Rulebook to zero) / AT-34-E3-001 (`class_feature_option_pool_record_with_magnitude_not_held_by_engine` mechanism)

- **Commit SHA:** `<filled by follow-up commit>` (parent `aaec01730c`)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):** "**970** Core Rulebook units whose
  table exists but which are not in it. Evidence: the atlas reporting bucket B at zero for
  `core_rulebook`, and the mechanism that placed them named — by mechanism, not per record."
  (970 is `epic-breakdown.md`'s own stale, whole-bucket figure — re-derived per-mechanism below,
  per this cycle's own dispatch instruction.)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Status:** partial

Bucket B for `core_rulebook` is nine distinct mechanisms (`decisions.md §14`). This cycle owns
exactly one: `class_feature_option_pool_record_with_magnitude_not_held_by_engine`.

## Population, re-derived (not quoted)

```
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
u = [x for x in d['units'] if x['book']=='core_rulebook' and x['status']=='engine-does-not-hold'
     and x['evidence']=='class_feature_option_pool_record_with_magnitude_not_held_by_engine']
print(len(u))
"
```
Before this cycle: **333** of 1,006 `core_rulebook` bucket-B units (re-derived at this cycle's
own start SHA `aaec01730c`, not quoted from the dispatch brief — the brief's own 333 figure
matched). Corpus-wide (37 books): 3,052 units across 21 books, own re-derive:
`python3 -c "import json; d=json.load(open('docs/work-inventory.json')); print(len([u for u in
d['units'] if u['evidence']=='class_feature_option_pool_record_with_magnitude_not_held_by_engine']))"`.

## What this mechanism is

Every unit in this evidence class is a magnitude-bearing `class_feature` corpus record whose
group prefix (`"Domain Power"`, `"Bardic Performance"`, `"Favored Enemy Bonus"`, ...) names no
engine-modelled class directly and resolves to no owner via any of `classify()`'s three
owner-resolution fallbacks (`class_feature_owner`, `class_feature_owner_via_type_facet`,
`class_feature_owner_via_pool_catalog`) — see `src/bin/v06_work_inventory.rs:9787`'s own
`else` arm.

## Discovery: the Domain Power `CLASS_FEATURE_POOLS` registration gap does NOT ground anything by itself

The prior cycle in this wave (`class_feature_option_pool_record_not_held_by_engine`, its own
57-unit sibling mechanism, `8e7aecc855`) flagged this population for follow-up, noting that
`domain_power.rs` already computes real magnitudes for five domains (Good/War/Strength/
Destruction/Glory) but `"Domain Power"` has no `CLASS_FEATURE_POOLS` entry.

**Checked before building anything (per this cycle's own dispatch instruction): no earlier
cycle in this wave had registered it.** `grep -n '"Domain Power"' src/bin/v06_work_inventory.rs`
at this cycle's start SHA returns only the prior cycle's own doc-comment mentions and test
fixtures, never a `CLASS_FEATURE_POOLS` entry.

Read directly before building anything, rather than assumed: `class_feature_exact_suffix_grounded`
and `suffix_stripped_grounded` (`src/bin/v06_work_inventory.rs:7920-7934`, `:9784-9796`) both
require `group.eq_ignore_ascii_case(&class_name_as_group_text(owner))` as a hard, unconditional
guard. `"Domain Power"` can never equal `"cleric"`. **So registering `"Domain Power"` in
`CLASS_FEATURE_POOLS` alone would never ground a single record through the owner-resolution
path** — it would only reclassify every one of these 61 units from bucket B's
`class_feature_option_pool_record_with_magnitude_not_held_by_engine` to bucket B's OWN sibling
evidence `class_feature_owner_matched_by_name_but_record_not_held_by_engine` (owned by a
different, already-dispatched cycle) or bucket D's `no_explanation_id_and_no_diagnostic_names_
this_feature` — moving the number between mechanisms without moving a single record's real
state. Confirmed this claim is true by reading the guard, not by trusting the prior cycle's
receipt.

## The real fix: a dedicated, verified probe

Built `probe_domain_power_effect_wiring` (`src/bin/v06_work_inventory.rs`), the same shape as
the pre-existing `probe_class_effect_wiring`/`probe_class_feature_key`: for each of
`domain_power::DOMAIN_POWER_CATALOG`'s five real entries, selects that EXACT domain on a real
cleric (`choice:cleric_domain` -> `domain:<slug>`), sweeps `SWEEP_LEVELS` through the real
`compute_pilot_base_chassis` pipeline, and keeps only granted-power names whose own explanation
id (`domain_power::domain_power_explanation_id`) is genuinely observed. This is a live
computation, never a static reflection of the catalog's membership — the canonical per-class
sweep that fills `EngineFacts::explanation_ids` only ever selects Good's own domain
(`canonical_seeds_for`'s fixed `"domain:good"` seed for cleric), so War/Strength/Destruction/
Glory's own ids were never previously observed anywhere in this file's own instrumentation.

`domain_power.rs` gained one new `pub` bridge function, `domain_power_probe_catalog()`, returning
`(selection_id, granted_power_name, [explanation ids])` triples — read-only, no behavior change
to the module itself. `pilot_compute::mod.rs`'s `mod domain_power;` became `pub mod domain_power;`
(the same visibility shape `crb_untabled_class_chassis` already uses) so the bin crate can call it.

`classify()`'s `Kind::ClassFeature` arm gained one new early check, immediately after the
pre-existing `class_feature_effect_wired` observation and before owner resolution: if
`group == "Domain Power"` and `facts.domain_power_effect_wired` contains this record's own
granted-power name, the record is `grounded` (`evidence:
"domain_power_probe_observed_a_real_computed_magnitude"`). Every other `"Domain Power ~ *"`
record — including the 56 this catalog carries no formula for — falls through completely
unaffected, exactly as before this cycle.

## RED -> GREEN

RED (confirmed for the intended reason): before this cycle's `classify()` edit, `"Domain Power ~
Touch of Good"` fell through the same owner-resolution path every other Domain Power record does
(`class_feature_owner_via_pool_catalog` returns `None` for `"Domain Power"`, confirmed by the
prior cycle's own `class_feature_owner_via_pool_catalog_refuses_an_unmodelled_owner`-style test),
landing at `engine-does-not-hold` /
`class_feature_option_pool_record_with_magnitude_not_held_by_engine` — the live corpus data
itself is the RED proof (`docs/work-inventory.json` at this cycle's start SHA lists exactly this
verdict for all 5 of Good/War/Strength/Destruction/Glory's own Domain Power records).

GREEN: two new tests added to `class_feature_text_complete_rung_tests`:
- `a_domain_power_record_the_probe_observed_reaches_grounded` — `facts.domain_power_effect_wired`
  seeded with `"Touch of Good"`; asserts `status == "grounded"`,
  `evidence == "domain_power_probe_observed_a_real_computed_magnitude"`.
- `a_domain_power_record_the_probe_never_observed_is_unaffected` — negative control,
  `"Domain Power ~ Acid Dart"` (a real multi-`DESC`-token/level-gated record this catalog
  deliberately does not cover), empty `domain_power_effect_wired`; asserts the pre-existing
  `engine-does-not-hold` / `class_feature_option_pool_record_with_magnitude_not_held_by_engine`
  verdict is unchanged.

```
cargo test --locked --bin v06_work_inventory domain_power
running 2 tests
test class_feature_text_complete_rung_tests::a_domain_power_record_the_probe_observed_reaches_grounded ... ok
test class_feature_text_complete_rung_tests::a_domain_power_record_the_probe_never_observed_is_unaffected ... ok
test result: ok. 2 passed; 0 failed; ...
```

## Files touched

- `src/rules_core/pilot_compute/domain_power.rs` — new `pub fn domain_power_probe_catalog()`.
  No change to any existing function's behavior.
- `src/rules_core/pilot_compute/mod.rs` — `mod domain_power;` -> `pub mod domain_power;`.
- `src/bin/v06_work_inventory.rs` — new `domain_power` import; new `EngineFacts::
  domain_power_effect_wired: BTreeSet<String>` field, populated in `gather_engine_facts` via new
  `probe_domain_power_effect_wiring`; one new early-return branch in `classify()`'s
  `Kind::ClassFeature` arm; two new tests.
- `scripts/completion_atlas.py`, `scripts/missing_engine_tables.py` — this cycle's own ~30-line
  net insertion into `v06_work_inventory.rs` shifted every hardcoded `file:line` citation both
  files carry (the brief's own named hazard). Re-derived each one fresh by grepping the exact
  quoted string this cycle's insertion sits above/below (not by adding a flat offset), confirmed
  by re-running both `--check` gates: `citation_failures=0` on both.
- `docs/work-inventory.json` — regenerated at HEAD, guarded path (`CORPUS_LITERAL_SWEEP_REPORT`/
  `DERIVED_FIXTURE_CHECK_REPORT` set from this cycle's own fresh sweep/fixture-check runs, no
  `--allow-stamp-loss` used or needed).
- `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json`,
  `.../missing-engine-tables.json` — regenerated by their own `--check` runs at HEAD.
- `docs/release/SD-34-book-completion/artifacts/epic-3-core-rulebook/AT-34-E3-001_class_feature_option_pool_with_magnitude_cycle_receipt.md`
  (this file).
- `docs/release/SD-34-book-completion/progress.md`, `docs/release/SD-34-book-completion/kanban.md`.
- `docs/retro/events/sd34-at-34-e3-001.jsonl` — one `correction` event (below).
- `src/rules_core/pilot_compute/formula_interpreter_corpus_wide.rs` — see "Discoveries".

## Discoveries

**A pre-existing WRONG pin, not a stale one, found while re-running `cargo test --locked --lib`
after this cycle's own inventory regen (`decisions.md §12` L7).** The immediately-prior cycle in
this wave (`class_feature_owner_matched`, same-day) re-pinned
`formula_interpreter_corpus_wide::tests::f1_population_matches_the_current_true_formula_bearing_
count_not_the_stale_sd32_census` from 6,257 to **5,563**, its own doc comment claiming
`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus`
returned 5,563 at its own commit `ae25d75d7d`. Running that exact command against
`git show ae25d75d7d:docs/work-inventory.json` returns **5,445**, not 5,563 — confirmed twice:
once before this cycle's own edits touched anything, once after this cycle's own regeneration
(which moved 5 units out of bucket B, none of them F1-shaped: `max(.../2,1)`/bare-`LVL` formulas
are F5/F2, never F1's bare-literal shape). The prior cycle's own re-derivation was simply wrong,
not stale. Re-pinned to **5,445** with corrected provenance in the test's own doc comment;
`scripts/retro.py correction` recorded below.

```
python3 scripts/retro.py correction --subject AT-34-E3-001-class_feature_owner_matched-cycle \
  --claimed "F1 population 5,563" \
  --actual "F1 population 5,445 (both before and after AT-34-E3-001's own domain-power cycle; the prior re-derivation was itself wrong, not stale)" \
  --verified-by "python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --corpus-root data/corpus"
```

Also self-healed a same-class mistake in myself, mid-cycle: my first `scripts/retro.py`
invocation ran without `RETRO_ACTOR` set in that shell call (harness shell state does not persist
across `Bash` tool calls — every call needs its own `export`), so it wrote to the wrong,
FORBIDDEN file (`docs/retro/events/sd31-transcribe.jsonl`, explicitly named off-limits by this
cycle's own dispatch brief, "another lane's dirty file"). Caught by `git status --porcelain`
before the next git write (`decisions.md §5`'s own discipline), diffed to confirm the file had
5 pre-existing lines from `sd31-transcribe`'s own actor dirtying it before this session started,
removed exactly the one line this cycle's own mistaken call appended (leaving the pre-existing
5 lines from the other lane untouched, confirmed byte-identical to the pre-session `git diff` by
re-running `git diff --stat`), and re-ran the correction with `RETRO_ACTOR` exported in the SAME
command as the `python3 scripts/retro.py` invocation.

`apps/desktop/src-tauri`: 26 of 548 tests FAIL (`companion_catalog`, `race_trait_picker`,
`reach_gate` modules) — confirmed pre-existing and unrelated to this cycle by running the
identical `cargo test --locked` in a throwaway `git worktree add --detach` checkout of this
cycle's own start SHA (`aaec01730c`), before any of this cycle's edits: identical 26 failures,
identical test names, `522 passed; 26 failed` both times. Not caused by, and not fixed by, this
cycle. Outside SD-33's own documented 29-suite/46-failure baseline (a distinct, larger,
already-existing regression this cycle did not introduce and was not asked to fix) — named here
rather than silently absorbed.

## Figures + their re-derive commands

| Figure | Value | Command | Denominator |
|---|---|---|---|
| Mechanism population before | 333 | `python3 -c "..."` (above) against `docs/work-inventory.json` at start SHA `aaec01730c` | of 1,006 `core_rulebook` bucket-B units |
| Mechanism population after | 328 | same command against the regenerated `docs/work-inventory.json` | of 1,006 `core_rulebook` bucket-B units (pre-cycle denominator; post-cycle bucket B total is 757, see atlas) |
| Corpus-wide mechanism population before | 3,052 | same command, no `book` filter | of 49,438 units |
| Corpus-wide mechanism population after | 3,047 | same command, no `book` filter | of 49,438 units |
| `core_rulebook` bucket B (whole book, all 9 mechanisms) before | 762 | `python3 scripts/completion_atlas.py --book core_rulebook --check` | of 6,701 `core_rulebook` units |
| `core_rulebook` bucket B (whole book, all 9 mechanisms) after | 757 | same | of 6,701 `core_rulebook` units |
| Units closed | 5 | `Domain Power ~ {Battle Rage, Destructive Smite, Strength Surge, Touch of Good, Touch of Glory}`, verified individually against `docs/work-inventory.json` | of 333 |
| `completion_atlas.py --check` (corpus-wide) | `unclassified=0 overlap=0 citation_failures=0` | `python3 scripts/completion_atlas.py --check` | of 49,438 |
| `corpus_literal_sweep` | `48708 examined of 51482 read, 0 findings` (before and after — no `data/corpus/**` file touched this cycle) | `/tmp/cargo-.../release/corpus_literal_sweep --json-out ...` | of 51,482 |
| `derived_evaluator_fixture_check` | `1839 unit(s) cleared over 2580 fixture row(s); 0 failed; 0 not ingested` | `/tmp/cargo-.../release/derived_evaluator_fixture_check --json-out ...` | of 2,580 fixture rows |
| `cargo test --locked --bin v06_work_inventory` | `383 passed; 0 failed` | `cargo test --locked --bin v06_work_inventory` | of 383 |
| `cargo test --locked --lib` | `2874 passed; 0 failed; 14 ignored` | `cargo test --locked --lib` | of 2888 |
| `cargo test --locked --no-run` (workspace) | exit 0 | `cargo test --locked --no-run` | — |

## Row-count command output (this cycle's own artifact)

```
$ python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
u = [x for x in d['units'] if x['book']=='core_rulebook' and x['status']=='engine-does-not-hold'
     and x['evidence']=='class_feature_option_pool_record_with_magnitude_not_held_by_engine']
print(len(u))
"
328
```
Not zero. **Status: partial**, remainder named below.

## Build scope verified

`cargo test --locked --no-run` exit 0, workspace-wide, run at SHA (this cycle's own commit, see
`commit_sha` in the structured return). `cargo test --locked --lib`: 2874 passed, 0 failed, 14
ignored. `apps/desktop/src-tauri` (separate cargo workspace): `cargo test --locked`, 522 passed,
26 failed — confirmed pre-existing (see Discoveries), unaffected by this cycle.

## Sweep population

`corpus_literal_sweep`: 48708 examined before -> 48708 examined after, delta 0 (no
`data/corpus/**` file touched or regenerated this cycle — only `src/rules_core/` and `src/bin/`
Rust source).

## Movement, four buckets

- **Closure:** 5 — `Domain Power ~ {Battle Rage, Destructive Smite, Strength Surge, Touch of
  Good}` -> `DONE` (`grounded`, `wiring_class: computed`); `Domain Power ~ Touch of Glory` ->
  bucket `V` (`literal-verified`, `wiring_class: static` — the downstream literal-verification
  stamp claimed it since its own corpus row states its formula as a literal DESC token). All
  five carry a REAL computed/verified magnitude a player's sheet renders, proven by a live
  probe against the real compute pipeline, not asserted.
- **Reclassification:** 0 — no unit changed bucket without a genuine holds change.
- **Reachability:** 0 — no previously-unreachable unit became reachable.
- **Instrument-correction:** 1 (adjacent, not this mechanism's own population) —
  `formula_interpreter_corpus_wide`'s F1 pin corrected from a wrong 5,563 to the true 5,445 (see
  Discoveries); moves no bucket-B count.

## Remainder — 328 units, named by sub-cause (`decisions.md §15`)

Grouped by corpus-key group prefix, re-derived fresh against the post-cycle inventory (populations sum exactly to 328):

| Sub-cause | Units | Notes |
|---|---|---|
| `Domain Power` (the other 56 domains) | 56 | `domain_power.rs`'s own doc comment names why each is excluded: dice notation (Healing's Rebuke Death), multi-`DESC`/level-gated variants (Acid Dart, Icicle, Blast Rune, Fire Bolt, Storm Burst, Lightning Arc, Artificer's Touch), enemy-facing effects (Evil/Darkness/Madness/Chaos/Law), no header chain (Void/Scalykind — not CRB), and every domain this catalog simply has not reached yet (Air, Animal, Artifice, ...). Real new-formula work per domain, `domain_power.rs`'s own scope. |
| `Domain Base` | 33 | The domain HEADER record (`DEFINE:Domain<X>LVL`, flavor `DESC`) — a different corpus shape from `Domain Power`, carries no per-power magnitude of its own; needs its own scoped disposition (is this record's `magnitude_token_count>0` even correct, or is it a `D`/text-complete miscount upstream of this mechanism?), not built here. |
| `Favored Enemy Bonus` / `Favored Terrain Bonus` | 31 + 11 = 42 | Ranger tracking: no `favored_enemy`/`favored_terrain`-keyed per-record magnitude consumer exists in `src/rules_core/` today (confirmed by grep). New engine subsystem, not this cycle's scope. |
| `Bardic Performance` | 10 | Bard performance-type roster; no per-performance-type magnitude consumer exists (bard performance is currently a flat, unnamed pool in the engine). |
| `Draconic Bloodline Choice` | 10 | Sorcerer bloodline sub-choice roster (dragon type selection); no per-type magnitude consumer. |
| `Secret Lore` | 10 | Oracle Lore mystery's sub-roster; no per-record consumer. |
| `New Arcana` | 9 | Wizard/sorcerer bonus-spell-slot roster; no per-record consumer. |
| Wizard opposition/arcane school cluster (`{Abjuration,Conjuration,...} {Opposition }School`, `Arcane School Tracker`, `Universal School`) | ~34 | Wizard school-restriction tracking absent entirely — the SAME gap the sibling `class_feature_option_pool_record_not_held_by_engine` mechanism's own receipt already named (37-unit "largest remaining share"); this is that gap's `with_magnitude` twin. |
| `Ki Stat Choice`, `Physical Enhancement`, `Damage Reduction`, `Divine Bond`, `Favored Class Bonus`, `Hunter's Bond`, `Nature's Bond`, `Precise Strike`, `Wildshape 2`, `Wildshape 3` | 22 | Small (2-3 unit) per-class roster groups, each its own narrow new-consumer gap. |
| Long tail (single-unit groups: `Aura of {Chaos,Evil,Good,Law}`, `Bloodline {Feat,}Tracker`, `CMB`/`CMB Output`, `{...} Qualify`, `Death Attack`, `Domains`, `Equipment`, `Default`, ...) | ~106 | One-off tracker/bookkeeping rows, each needing individual per-record inspection before disposition — `decisions.md §14`'s own decomposition precedent, one level deeper still. |

129 distinct group prefixes total, summing exactly to 328 (`sum(groups.values()) == 328`,
verified by the same script that produced the table).

## Next-cycle plan

Cheapest-first for whichever cycle next owns this mechanism: `Domain Base` (33, needs a
disposition ruling on whether it's even magnitude-bearing) and the wizard-school cluster (~34,
shares its root cause with the already-scoped 37-unit sibling gap) are the next-largest single
sub-causes with a clear, scoped fix shape. `Favored Enemy/Terrain Bonus` (42) and `Bardic
Performance`/`Draconic Bloodline Choice`/`Secret Lore`/`New Arcana` (39) are all "new per-record
consumer" work, each its own scoped investigation.

- **Status:** partial
