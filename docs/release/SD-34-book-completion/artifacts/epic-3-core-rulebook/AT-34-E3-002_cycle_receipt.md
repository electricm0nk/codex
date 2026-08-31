# Cycle 6 — Epic 3 (Core Rulebook to zero) / AT-34-E3-002 (bucket C, "held and computed, never surfaced")

- **Commit SHA:** `9519106f17` (this cycle's own `classify()` fix + 4 tests, a 20-minute
  checkpoint commit made mid-cycle per the dispatch's clock discipline, pushed to `tranche/14`
  before this cycle's own live regen ran) plus this cycle's own follow-up commit (this receipt,
  `progress.md`, `kanban.md`, and `scripts/completion_atlas.py`'s citation re-pins) — both on
  `tranche/14` tip `07678e0601` (wave 19) at cycle start, no rebase needed for either.
- **Files touched:** `src/bin/v06_work_inventory.rs` (one new `classify()` rung for the bare
  `"<Domain> Domain"` HEADER shape — 73 lines, zero new `EngineFacts` fields, zero new probes,
  reusing two already-shipped fact sets; plus 4 new tests, RED confirmed then GREEN),
  `scripts/completion_atlas.py` (4 citation line pins re-derived after this cycle's own
  73-line insertion shifted every one below it — exact-line-content grep against
  `git show HEAD~1:...`, never guessed), this receipt,
  `docs/release/SD-34-book-completion/progress.md`,
  `docs/release/SD-34-book-completion/kanban.md`. **`docs/work-inventory.json` and
  `docs/release/SD-34-book-completion/artifacts/epic-1-atlas/completion-atlas.json` are
  deliberately NOT committed this cycle** — this dispatch's own file-ownership rule assigns
  their regeneration to the wave's single shared regeneration cycle. Every figure below comes
  from a real, local, uncommitted, FULL three-stage regen (`corpus_literal_sweep` →
  `derived_evaluator_fixture_check` → `v06_work_inventory`, `--allow-stamp-loss` never passed)
  of this cycle's own committed source, restored (`git restore`) before this commit.
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (`git diff --unified=0 -- src/bin/v06_work_inventory.rs
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` — zero matches, run against this
  cycle's own working-tree diff before the checkpoint commit. Also re-run against the full
  `merge-base(HEAD, origin/develop)...HEAD` range on the same file per the dispatch's own audit
  template — zero matches there too).
- **Wired-integration audit result:** OK_NO_TOKENS (same diffs, same
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` — zero matches on
  this cycle's own diff; the wide-range diff surfaces 16 pre-existing `placeholder` mentions,
  all from prior cycles' own already-audited "%N-placeholder" / "vacuous-placeholder" corpus
  vocabulary — a real PF1 corpus concept, not a code stub, and none inside this cycle's own
  hunks).
- **Acceptance criterion (verbatim, `epic-breakdown.md` §AT-34-E3-002):** "**370** units the
  engine holds and computes but never surfaces. **Evidence:** per unit, the explanation or
  display path that now carries it. A unit the player still cannot see is not cleared, whatever
  the engine holds." (370 is stale, already retired by wave 16/17; re-derived fresh at this
  cycle's start, `core_rulebook` bucket C was **233**, matching this dispatch's own brief
  exactly — the committed inventory already carried cycles 4 and 5's own folded-in fixes via
  the wave-18 shared regen, so no unregenerated-source gap existed at this cycle's start,
  unlike cycle 5's.)
- **Status:** partial

## Population, re-derived (not quoted)

At this cycle's start, the **committed** `docs/work-inventory.json` (last regenerated at
wave-18, `accb12b14d`) reads `core_rulebook` bucket C = **233**
(`python3 scripts/completion_atlas.py --book core_rulebook --check`) — matching cycle 5's own
receipt exactly, confirmed live rather than trusted on citation.

This cycle re-derived cycle 5's own remainder table fresh, by direct corpus read against
`docs/work-inventory.json`: all 11 named sub-causes reproduce cycle 5's own stated populations
exactly (23 + 42 + 41 + 36 + 31 + 21 + 13 + 10 + 10 + 0 + 6 = 233). Cycle 5's own next-cycle
plan named `domain_power_display_record_not_wired` (41) as the next-cheapest CANDIDATE within
this territory's own naming-only shape — this cycle confirmed that framing by direct corpus
read before building anything, and found it decomposes into exactly three real,
independently-attributable shapes (matching cycle 5's own receipt text verbatim):

- **33** bare `"<Domain> Domain"` HEADER records (e.g. `"Air Domain"`, `"Good Domain"` — the
  domain SELECTION feature itself, `type_facet ClericClassFeatures.Domain.ClericDomain`,
  confirmed by direct corpus read).
- **7** `"Druid Domain ~ <X>"` records (the Druid Nature Bond domain sub-choice, `type_facet
  DruidDomainSelection.SpecialQuality`).
- **1** `"Nobility Domain ~ Inspiring Word"` record, `magnitude_token_count: 0`.

33 + 7 + 1 = 41, matching cycle 5's own figure exactly.

## Mechanism: the SAME paired display/chassis pattern the Favored Enemy/Favored Terrain checks
## establish, spanning TWO already-wired sibling shapes

This cycle targets ONLY the 33 bare domain-header records. The 7 Druid sub-choices and the 1
zero-token record have **no existing compute path at all** to reuse — genuine engine gaps, not
naming-only fixes — confirmed unchanged by this cycle's own live regen and named in the
remainder below rather than force-closed.

**Why the header records are bucket C, not something else:** a bare `"<Domain> Domain"` record
(e.g. `"Air Domain"`) carries no magnitude formula of its own — it represents choosing that
domain, not the granted power. `group = unit.key.split(" ~ ").next()` for this key returns the
WHOLE key (no `" ~ "` present), so it can never equal `"Domain Power"` or match any owner
resolution path. The engine has, in fact, already computed a real explanation for the SAME
domain's granted power — the gap is purely that `classify()` never asked the header record's
own sibling group a question, the identical "gap" shape the Favored Enemy/Favored Terrain
display checks close for Ranger.

**The corpus carries the granted power under one of TWO different key shapes, confirmed by
direct read, not assumed:**

1. **Most domains:** `"<Domain> Domain ~ <Power>"` (e.g. `"Air Domain ~ Lightning Arc"`), the
   SAME sibling shape cycle 3's `cleric_domain_generic_member_wired` already observes for real.
   `group` for both the bare header AND its tilde'd sibling is identical (`"Air Domain"`), so a
   simple prefix check (`facts.cleric_domain_generic_member_wired.iter()
   .any(|k| k.starts_with(&format!("{} ~ ", unit.key)))`) reuses that fact set directly — no new
   probe. **This path's own reach turned out wider than a naive JSON-status read suggested**:
   the LIVE probe re-derives wiring by executing the real compute pipeline, so it credits a
   sibling regardless of whether the corpus's currently-committed JSON shows that sibling as
   `grounded` or already restamped to `literal-verified` (bucket V, `apply_done_rung_stamps`'s
   static-wiring-class stamp) — a unit's OWN static/derived restamping never touches whether the
   ENGINE computes it, only which bucket the atlas files it under. This path alone reaches 30 of
   the 33 headers, including `Glory Domain` (whose own two `"Glory Domain ~ *"` siblings both
   sit at `literal-verified`, not `grounded`, in the currently-committed JSON — still a real,
   live-observed wiring, still credited).
2. **Good, War (2 of 33):** the corpus carries NO `"<Domain> Domain ~ *"` record at all for
   these two — zero `"Good Domain ~ *"` or `"War Domain ~ *"` unit exists, confirmed by direct
   `docs/work-inventory.json` read. Their granted power is ingested ONLY as
   `"Domain Power ~ <Power>"`, already grounded via `domain_power_effect_wired` (AT-34-E3-001's
   own probe). This cycle derives the catalog's own `"domain:<slug>"` selection id losslessly
   from the header's own name text (every real PF1 Core Rulebook domain name is a single word,
   confirmed by reading all 33 header keys), looks it up in
   `domain_power::domain_power_probe_catalog()` (a static lookup, no live compute of its own),
   and checks `facts.domain_power_effect_wired` for the resolved `granted_power_name` — the SAME
   real, live-probed fact set `"Domain Power ~ *"` records already reuse; a static catalog
   lookup never fabricates a credit on its own.

`Strength`, `Destruction`, and `Glory` also carry `DOMAIN_POWER_CATALOG` entries, but all three
are reached by path 1 first (they DO carry `"<Domain> Domain ~ *"` siblings the generic
pool-group pass grounds), so path 2's own net-new contribution is exactly the 2 domains path 1
cannot reach at all: **Good, War**.

**30 (path 1) + 2 (path 2, net-new) = 32 of 33 headers close.** The remaining **1**
(`Nobility Domain`) has NEITHER a live-wired `"Nobility Domain ~ *"` sibling (both real corpus
members — `"~ Inspiring Word"`, a genuine zero-magnitude gap, and `"~ Leadership"`, bucket B,
a different sub-cause entirely — stay `engine-does-not-hold`, confirmed unchanged by this
cycle's own live regen) NOR a `domain_power::DOMAIN_POWER_CATALOG` entry — genuinely no
existing engine mechanism computes anything for it. Confirmed by direct read of both fact sets
and the catalog's own 5-entry membership, not assumed; named in the remainder below.

**Territory respected:** zero changes to `pilot_compute::mod.rs` or `domain_power.rs` — both
already-shipped probes and the catalog are reused exactly as they already compute; this cycle
only teaches the CLASSIFIER to consult explanations that already existed. Book-scoped to
`core_rulebook`: a direct corpus scan found exactly one same-shaped `"* Domain"` bare key
elsewhere (`ultimate_psionics:ability:psionics_domain`, `kind: "ability"`, a completely
different corpus shape/bucket already at `ingested-magnitude`), a different mechanism this
territory does not touch, guarded by `unit.book == "core_rulebook"`.

## RED → GREEN

RED (confirmed for the intended reason): temporarily changed the new rung's own key match from
`unit.key.ends_with(" Domain")` to `unit.key.ends_with("RED-CHECK-NEVER-MATCHES")` and re-ran the
two positive proof tests — both failed with `left: "engine-does-not-hold", right: "grounded"`
(the pre-existing fallthrough this cycle closes), confirming the tests fail because the fix is
absent, not for an unrelated reason. Restored the match; all four tests pass.

```
$ cargo test --locked --bin v06_work_inventory a_domain_header_record
running 4 tests
test class_feature_text_complete_rung_tests::a_domain_header_record_reaches_grounded_off_its_generic_pool_sibling_wiring ... FAILED
left: "engine-does-not-hold"
right: "grounded"
test class_feature_text_complete_rung_tests::a_domain_header_record_reaches_grounded_off_its_domain_power_catalog_sibling_wiring ... FAILED
left: "engine-does-not-hold"
right: "grounded"
```

After restoring the match:

```
$ cargo test --locked --bin v06_work_inventory a_domain_header_record
running 4 tests
test class_feature_text_complete_rung_tests::a_domain_header_record_reaches_grounded_off_its_generic_pool_sibling_wiring ... ok
test class_feature_text_complete_rung_tests::a_domain_header_record_in_a_different_book_is_not_credited ... ok
test class_feature_text_complete_rung_tests::a_domain_header_record_reaches_grounded_off_its_domain_power_catalog_sibling_wiring ... ok
test class_feature_text_complete_rung_tests::a_domain_header_record_with_neither_sibling_wired_is_unaffected ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 473 filtered out; finished in 0.00s
```

Full `class_feature`-scoped suite: `cargo test --locked --bin v06_work_inventory class_feature`
— **146 passed, 0 failed** (142 pre-existing + this cycle's own 4). Full bin suite: **477
passed, 0 failed** (473 + 4).

## Live regen (local, uncommitted — see file-ownership note above)

**Full three-stage pipeline run, in order, `--allow-stamp-loss` never passed** (a first attempt
running `v06_work_inventory` alone was correctly REFUSED by its own stamp-loss guard — it would
have dropped 9,591 of 9,591 pre-existing `literal-verified`/`fixture-verified` stamps, since
neither report was pointed at; the guard worked exactly as designed):

```
$ corpus_literal_sweep --json-out /tmp/sweep-report.json
corpus-literal-sweep: 48708 records examined of 51482 read, 413336 tokens compared (9 synthesized),
51469 digests checked, 0 findings
corpus-literal-sweep: 3138 tokens exempted under decisions.md §24 redaction across 1058
codex_generated_name records
corpus-literal-sweep: CLEAN

$ derived_evaluator_fixture_check --json-out /tmp/fixture-report.json
derived-evaluator-fixture-check: 1839 unit(s) cleared over 2580 fixture row(s); 0 failed; 0 not ingested

$ CORPUS_LITERAL_SWEEP_REPORT=/tmp/sweep-report.json DERIVED_FIXTURE_CHECK_REPORT=/tmp/fixture-report.json \
  v06_work_inventory
(writes docs/work-inventory.json; exit 0)
```

Both reports match wave-18's own baseline exactly — unchanged, since this cycle touches no
`data/corpus/**` file (48,708 examined both before and after; 1,839/2,580 fixture rows cleared
both before and after).

**Isolation confirmed by a whole-inventory before/after diff keyed on unit id** (not sampled —
a real Python diff over both full 49,438-unit JSON documents, before = the COMMITTED HEAD
inventory, after = this cycle's own local regen against HEAD's committed source plus this
cycle's own edit):

```
before count: 49438 after count: 49438
added: 0 removed: 0
changed: 32
changed by book: {'core_rulebook': 32}
changed by new evidence: {
  'generic_pool_group_selection_probe_observed_a_real_computed_magnitude_for_the_display_record': 30,
  'domain_power_probe_observed_a_real_computed_magnitude_for_the_display_record': 2
}
```

All 32 changes carry one of this cycle's own two new evidence strings — **zero changes outside
this cycle's own 32 domain-header ids**, and zero changes from any other concurrently-committed
lane between this cycle's checkpoint push and this regen (confirmed: `origin/tranche/14` had
not moved). The 32 closed ids, individually:

`air_domain, animal_domain, artifice_domain, chaos_domain, charm_domain, community_domain,
darkness_domain, death_domain, destruction_domain, earth_domain, evil_domain, fire_domain,
glory_domain, good_domain, healing_domain, knowledge_domain, law_domain, liberation_domain,
luck_domain, madness_domain, magic_domain, plant_domain, protection_domain, repose_domain,
rune_domain, strength_domain, sun_domain, travel_domain, trickery_domain, war_domain,
water_domain, weather_domain` (all `core_rulebook:class_feature:*`).

`good_domain` and `war_domain` carry the `domain_power_probe_observed_...` evidence (path 2);
the other 30 carry the `generic_pool_group_selection_probe_observed_...` evidence (path 1).
Every one of the 32 moved `engine-does-not-hold` (bucket C) → `grounded` (bucket **DONE**)
directly, confirmed by direct post-regen read: all 32 carry `status: "grounded"`,
`wiring_class: "computed"` — none eligible for the static/derived V-reclassification
`apply_done_rung_stamps` applies, so none were restamped away from DONE.

**Corpus-wide isolation** (same regen, same before/after documents):

```
before: DONE=24433 A=449 B=11769 C=4214 D=2955 M=4938 U=202 V=289 X=170 Z=19
after:  DONE=24465 A=449 B=11769 C=4182 D=2955 M=4938 U=202 V=289 X=170 Z=19
```

DONE +32, C −32, every other bucket unchanged — exactly this cycle's own 32 closures, nothing
else moved corpus-wide.

## Figures + their re-derive commands

| Figure | Value | Command | Denominator |
|---|---:|---|---|
| `core_rulebook` bucket C at cycle start | 233 | `python3 scripts/completion_atlas.py --book core_rulebook --check` against the committed `docs/work-inventory.json` | of 6,701 |
| `core_rulebook` bucket C after this cycle's own fix | **201** | same command, live regen including this cycle's edit | of 6,701 (delta −32) |
| `core_rulebook` bucket DONE after this cycle | **4,481** | same command | of 6,701 |
| `core_rulebook` buckets A/B/D/M/V/U/X/Z after this cycle | unchanged (0/470/366/944/114/10/115/0) | same command | of 6,701 — confirms isolation |
| Corpus-wide bucket C before/after this cycle's own regen | 4,214 / **4,182** | `python3 scripts/completion_atlas.py --check` | of 49,438 (delta −32) |
| Corpus-wide bucket DONE before/after | 24,433 / **24,465** | same command | of 49,438 (delta +32) |
| Whole-inventory before/after diff, keyed on unit `id` | 0 added, 0 removed, exactly 32 changed, all `core_rulebook` | (Live regen section) | of 49,438 |
| This cycle's own isolated closures | **32**, all `core_rulebook`, all bare `"<Domain> Domain"` headers | whole-inventory diff filtered on this cycle's own two evidence strings | of 33 (targeted population) |
| `domain_power_display_record_not_wired` sub-cause, header component | 33 targeted, 32 closed, 1 remains (`Nobility Domain`) | direct `docs/work-inventory.json` read, bucket-C units ending `" Domain"` with no `" ~ "` | of 41 |
| `corpus_literal_sweep` (before/after, unchanged) | 48,708 examined, 0 findings | `corpus_literal_sweep --json-out` | of 51,482 read |
| `derived_evaluator_fixture_check` (before/after, unchanged) | 1,839 cleared of 2,580 rows, 0 failed | `derived_evaluator_fixture_check --json-out` | of 2,580 |
| `completion_atlas.py --check` (corpus-wide, post-regen) | `population=49438 unclassified=0 overlap=0` | `python3 scripts/completion_atlas.py --check` | of 49,438 |
| `completion_atlas.py --check` `citation_failures` | 0 (4→0, re-derived this cycle after this cycle's own 73-line insertion shifted 4 pins by exactly +73 each) | `python3 scripts/completion_atlas.py --check` | of 10 citations |
| `cargo test --locked --bin v06_work_inventory` (full) | `477 passed; 0 failed` | `cargo test --locked --bin v06_work_inventory` | of 477 |
| `cargo test --locked --bin v06_work_inventory class_feature` | `146 passed; 0 failed` | `cargo test --locked --bin v06_work_inventory class_feature` | of 146 |
| `cargo test --locked --no-run` (workspace) | exit 0 | `cargo test --locked --no-run` | — |

## Row-count command output (this cycle's own live artifact, uncommitted per file-ownership rule)

```
$ python3 scripts/completion_atlas.py --book core_rulebook --check
book=core_rulebook population=6701 unclassified=0 overlap=0
  DONE: 4481
  A: 0
  B: 470
  C: 201
  D: 366
  M: 944
  V: 114
  U: 10
  X: 115
  Z: 0
```

Bucket C: **201**, not zero. **Status: partial**, remainder named below (populations sum
exactly to 201). This live command output was produced by the local, uncommitted regen and is
NOT reflected in the currently-committed `docs/work-inventory.json` (restored via
`git restore` before this commit, per the file-ownership rule) — the committed inventory still
reads C=233 until the wave's shared regeneration cycle re-runs the pipeline against this
cycle's own committed source.

## Build scope verified

`cargo test --locked --no-run` (workspace) exits **0**, run at commit `9519106f17` — this
cycle's own last commit that can move a figure a test assertion depends on
(`decisions.md §12` L7; the local regen that follows is never committed, so it cannot un-verify
this run). Desktop crate (`apps/desktop/src-tauri`) not tested this cycle: no file under that
tree, nor any file it depends on, was touched by this cycle's own diff (confirmed:
`grep -rl "domain_header\|domain_power_probe_catalog_sibling\|for_the_display_record" apps/` —
zero matches).

## Sweep population

`corpus_literal_sweep`: 48,708 examined, before and after — unchanged, since no
`data/corpus/**` file was added or regenerated this cycle.

## Oracle pin

N/A — no figure in this receipt came from the pinned PCGen oracle corpus.

## Movement, four buckets

- **Closure:** **32** — 32 of the 33 bare `"<Domain> Domain"` header units, all carrying
  `wiring_class: "computed"`, moved `engine-does-not-hold` (bucket C) → `grounded` (bucket
  **DONE**) directly. Nothing remains for these; each is a genuine `+0` choice-selection
  record (choosing a domain is itself a real, real-cost-free game action) whose real granted
  power is already grounded on the sibling record the corresponding compute pass emits — no
  further magnitude work is owed by the header record itself.
- **Reclassification:** 0 this cycle (no unit moved between two non-DONE buckets).
- **Reachability:** **32** (one new `classify()` rung now answers `grounded` for these exact
  corpus keys, reusing two real, already-shipped, already-tested engine fact sets — no new
  compute path, no new formula, no engine change).
- **Instrument-correction:** 0 this cycle (the 4 `completion_atlas.py` citation re-pins are a
  bookkeeping side effect of this cycle's own insertion shifting line numbers, not a correction
  of a wrong prior figure).

**Bucket C's own delta (233 → 201, −32) equals this cycle's own Closure exactly** — the
row-count command's own output above is the ground truth this movement report is checked
against, not the other way around.

## Remainder — 201 of 233, named by sub-cause, populations sum exactly

Re-derived fresh at this cycle's own close (`decisions.md §12` L2) — the whole-inventory diff
above proves the ONLY `core_rulebook` changes this cycle made are its own 32, so restating cycle
5's other ten sub-cause figures unchanged is evidence-backed by this cycle's own proof, not
carried forward by assumption. The former `domain_power_display_record_not_wired` (41) is
decomposed into two real, differently-shaped remainders — a genuinely-uncomputable header/power
pair, and a genuinely-uncomputed Druid sub-choice mechanism — rather than restated as one row:

| Sub-cause | Population | Status / next step |
|---|---:|---|
| `bloodline_power_or_bloodline_feat_not_computed` | 23 | Unchanged (confirmed by this cycle's own isolation diff). |
| `monk_unarmed_damage_no_formula_in_engine` | 42 | Unchanged (confirmed). Genuine engine gap — needs a new formula, not a naming-only fix; now the largest remaining named sub-cause. |
| `base_class_standalone_feature_not_computed` | 36 | Unchanged (confirmed). Unstarted. |
| `prestige_class_standalone_feature_not_computed` | 31 | Unchanged (confirmed). Unstarted. |
| `other_named_group_or_standalone` | 21 | Unchanged (confirmed). Unstarted. |
| `rage_power_not_computed` | 13 | Unchanged (confirmed). Unstarted. |
| `npc_class_standalone_feature_not_computed` | 10 | Unchanged (confirmed). Unstarted. |
| `rogue_talent_not_computed` | 10 | Unchanged (confirmed). Unstarted. |
| `monk_unarmed_damage_small_cross_book_attribution_undecided` | 6 | Unchanged (confirmed). Still open, cross-book attribution question. |
| `druid_nature_bond_domain_selection_not_computed` (**new**, split out of `domain_power_display_record_not_wired`) | 7 | The 7 `"Druid Domain ~ <X>"` sub-choice records. Confirmed genuine engine gap by direct read of `pilot_compute::mod.rs`: Task #64's own comment states plainly that Nature Bond's domain option carries NO `DRUID_DOMAIN_CHOICE_ID` seam at all — the engine has never modelled this selection, in either direction (Good-domain wiring was explicitly checked and refused for a real rules reason: Good is not a legal Nature Bond domain). Real new engine work, not a naming-only fix — out of this territory's bar. |
| `domain_power_display_record_not_wired` (**narrowed** 41→2) | 2 | The last bare header this cycle could not reach (`"Nobility Domain"`) plus its own zero-token granted-power record (`"Nobility Domain ~ Inspiring Word"`). Neither has a live-wired sibling of either reusable shape, and Nobility carries no `domain_power::DOMAIN_POWER_CATALOG` entry — genuinely no existing engine mechanism to reuse; would need a new catalog entry (M-shaped engine work), not a naming-only fix. |
| `versatile_performance_not_computed` | 0 | Closed cycle 5; unchanged. |

**Sum check:** 23 + 42 + 36 + 31 + 21 + 13 + 10 + 10 + 6 + 7 + 2 + 0 = **201**, matching the
row-count command's own remainder exactly (233 − 32 = 201).

## Notes

- **This cycle's fix is deliberately minimal and additive**: one new `classify()` rung, zero new
  `EngineFacts` fields, zero new probes. It reuses TWO already-existing, already-tested probes'
  own fact sets (`cleric_domain_generic_member_wired`, `domain_power_effect_wired`) plus one
  already-existing static catalog (`domain_power::domain_power_probe_catalog()`) — the SAME
  paired display/chassis pattern the Favored Enemy/Favored Terrain checks establish, generalized
  across a book-wide 33-unit population in one change (the "generic pass" ROI shape this
  dispatch's own brief asks for): **32 real closures from 73 new lines**.
- **A genuine discovery, not assumed**: the FIRST attempt at this cycle's own live regen ran
  `v06_work_inventory` alone (no sweep/fixture reports pointed at it) and was correctly REFUSED
  by the tool's own stamp-loss guard (would have dropped 9,591 of 9,591 pre-existing
  `literal-verified`/`fixture-verified` stamps). The guard is doing exactly its job; the fix was
  to run the real three-stage pipeline, not to pass `--allow-stamp-loss` (never done).
- **A second genuine discovery this cycle's own live regen caught, that a naive JSON-status
  read would have missed**: an exploratory pre-check that filtered candidate siblings by
  `status == "grounded"` undercounted this rung's own real reach (predicted 29 closures,
  0 for `Glory`, `Charm`, `Liberation`, `Sun`). The REAL `classify()` rung does not read the
  JSON's status field at all — it consults the live probe's own fact set, which credits a
  sibling regardless of whether that sibling has since been restamped to `literal-verified`
  (bucket V). Trusting the live regen's own real output over the pre-check's own proxy is what
  caught 3 of the 32 real closures (`Charm`, `Liberation`, `Sun`) a proxy-based estimate would
  have wrongly left in the remainder, and correctly attributed `Glory`'s own closure to path 1
  (not path 2, as the pre-check predicted).
- **Territory respected:** no `CharacterInput` field was added or changed; no trait/ability
  compute path was touched; the EQUIPMENT magnitude sub-causes (owned by a sibling lane) were
  not touched; `pilot_compute::mod.rs` and `domain_power.rs` are completely untouched —
  confirmed by `git status --porcelain` before this commit showing only
  `src/bin/v06_work_inventory.rs` under this territory.
- **Not attempted this cycle**: every other named sub-cause in the 201-unit remainder table.
  `monk_unarmed_damage_no_formula_in_engine` (42) is now the largest, a genuine engine-formula
  gap, not a naming-only fix like this cycle's.

## Next-cycle plan

1. `monk_unarmed_damage_no_formula_in_engine` (42, largest remaining named sub-cause) needs a
   real new formula in the engine, not a classifier-naming fix — out of this territory's
   "naming-only" bar unless the engine-side formula work is dispatched separately first.
2. `base_class_standalone_feature_not_computed` (36) and `prestige_class_standalone_feature_
   not_computed` (31) are both unstarted and worth a direct corpus scan first — either could
   decompose into the same paired display/chassis shape this cycle and the Favored Enemy/
   Terrain cycle both proved out, or could be a genuinely different shape; re-derive before
   picking, do not assume.
3. `druid_nature_bond_domain_selection_not_computed` (7, new) and the narrowed
   `domain_power_display_record_not_wired` (2, new) are both genuine ENGINE gaps (a new
   `DRUID_DOMAIN_CHOICE_ID` seam; a new `DOMAIN_POWER_CATALOG` entry for Nobility), not
   naming-only fixes — out of this territory's bar as currently scoped.
4. Re-derive the remainder partition fresh before picking (`decisions.md §12` L2) — this
   receipt's own table is this cycle's fresh derivation, but the NEXT cycle must re-run it
   fresh again rather than trust this one, especially since the wave's shared regeneration
   cycle has not yet run against this cycle's own commit.
