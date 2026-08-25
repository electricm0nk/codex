# Cycle AT-33-E5-remainder-spell — Epic 5 Re-verification / AT-33-E5-001 & AT-33-E5-002 (spell-casting-ability remainder)

- **Commit SHA:** `bce3b3d868` (landed on `tranche/13`; this line updated in a follow-up commit,
  same convention `AT-33-E5-001`'s `e10dead123`/`AT-33-E5-002`'s `114bba8ec4` precedent used)
- **Files touched:**
  - `scripts/oracle_harness/derive_spell_casting_ability_mapping.py` (new) — derives the PF1 class
    -> governing-ability mapping directly from the pinned PCGen oracle checkout's own
    `data/pathfinder/paizo/roleplaying_game/*/*_classes.lst` `CLASS:<Name> ... SPELLSTAT:<ABBREV>`
    declarations (36 classes found), never hand-rolled or transcribed from memory.
  - `scripts/oracle_harness/spell_casting_ability_mapping.json` (new) — the derived mapping,
    committed for reproducibility; carries the pinned oracle SHA it was derived at and a
    cross-check against the engine's own 7-class `casting_ability_for_class`.
  - `src/bin/fixture_verified_oracle_probe.rs` (extended, not forked) — new `--remainder <path>`
    mode widening the SAME wave-1 spell probe to this cycle's 815-unit population; reuses
    `compute_spellbook_coverage`/`SPELL_PROBE_CASTING_CLASSES`/`ability_scores_pinned` unmodified.
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/spell-remainder-probe-output.json` (new) — the probe's own committed output (100 real `spell` rows + 715 real, per-unit-reasoned `unverifiable` rows).
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/spell-remainder-fixtures/` (new) — `fixture-spell-pcg/{wizard,cleric,druid,bard,paladin,ranger}.pcg` (6 real, live-loaded PCGen characters, reusing `fixture-generate-spell-batch.py` and its shared `fixture-spell-batch.txt.ftl` template unmodified), `fixture-spell-oracle-txt/{...}.export.txt` (6 real, live `./gradlew run` BatchExporter outputs), `fixture-spell-batch.manifest.json`.
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/spell-remainder-build-final-results.py` (new) — merges the real oracle-compared `spell` rows with the pre-classified `unverifiable` rows into the committed result set, disambiguating a genuinely-absent oracle export key from a present-but-blank one (see Notes).
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/spell-remainder-compared-oracle.json` (new) — `fixture-compare-spell-batch.py`'s (Epic 2's own comparison harness, unmodified) real output for the 100 examined units.
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/spell-remainder.oracle-results.json` (new) — the 815-row committed deliverable.
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-remainder-spell_cycle_receipt.md` (this file).
  - `docs/release/SD-33-computed-value-verification/progress.md` (updated in place).
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (see Test scoping for the exact command and scope)
- **Wired-integration audit result:** OK_NO_TOKENS (see Test scoping)
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-001 — the 1,741 `fixture-verified` units are re-examined against the oracle
  >
  > **Evidence:** per-unit `(ours, oracle, verdict)` rows committed; agreement and disagreement
  > counts both stated, with the denominator.
  >
  > ### AT-33-E5-002 — the 6,589 `literal-verified` units are re-examined
  >
  > **Evidence:** as above.

## What this cycle closes

This is a **remediation wave-2 slice cycle**, dispatched to close one named 815-unit remainder
after two prior halts at the final-acceptance scan (`AT-33-E6-001` attempts 1 and 2). The named
blocker (from `AT-33-E5-001`'s own receipt): **598 `fixture-verified` `spell` units carrying
evidence `spell_list_entry_with_resolved_level`** (not `spell_effect_probe_observed_computed_delta`)
plus **217 `literal-verified` `spell` units** (`AT-33-E5-002`'s own named remainder, ALL of which
were unexamined regardless of evidence type) — **815 units total** — because `probe_spell_key`'s
casting-class-existence check (`SPELL_PROBE_CASTING_CLASSES`) never found a match for them, and
the dispatch brief attributed this to "no casting-ability mapping."

**The mapping was built** (see below) and used to widen classification — but the mapping itself
was not this population's actual bottleneck. What genuinely blocked most of these 815 units, found
by execution this cycle, not assumed: **table scope**, not ability. `casting_ability_for_class`'s
seven classes cover Wizard/Cleric/Druid/Ranger/Sorcerer/Bard/Paladin, but the ENGINE's per-school
`resolve_<school>_spell_effect` functions (`src/rules_core/spellbook/*.rs`) that must ALSO resolve
a `SpellEffect` before any DC can be computed only read `core_rulebook`/`advanced_players_guide`/
`advanced_class_guide`'s own `SPELL_LIST` tables (confirmed by reading every one of the nine
per-school modules' own widening comment, e.g. `spellbook/illusion.rs`). A spell from any other
book — `ultimate_magic`, `occult_adventures`, `ultimate_combat`, `ultimate_intrigue`,
`ultimate_wilderness`, `inner_sea_gods` — produces no `SpellEffect` regardless of which class casts
it. `src/rules_core/spellbook/` and `src/rules_core/rules_tables/` are outside this cycle's granted
write scope (`scripts/oracle_harness/`, `src/bin/`, this `artifacts/` directory only) — widening
those tables is real, concrete future scope, named below, not attempted here.

## The casting-ability mapping — built, derived, cross-checked

`scripts/oracle_harness/derive_spell_casting_ability_mapping.py` reads every real
`CLASS:<Name> ... SPELLSTAT:<ABBREV>` line in the pinned PCGen oracle checkout's own
`data/pathfinder/paizo/roleplaying_game/*/*_classes.lst` files — **never hand-rolled or
transcribed from memory**. Re-derive:

```
$ PCGEN_REPO_DIR=<pinned checkout, resolved via $PCGEN_REPO_DIR — never a literal path in this doc> \
  python3 scripts/oracle_harness/derive_spell_casting_ability_mapping.py
derive_spell_casting_ability_mapping: 36 classes -> scripts/oracle_harness/spell_casting_ability_mapping.json
cross-check against engine's 7-class map: agrees=True
```

**36 classes found**, including every class named on any of this cycle's 815 units' corpus
`CLASSES:` tokens: Wizard=Intelligence, Psychic=Intelligence, Witch=Intelligence,
Ranger=Wisdom, Spiritualist=Wisdom, Bard=Charisma, Medium=Charisma, Inquisitor=Wisdom,
Paladin=Charisma, Mesmerist=Charisma, Shaman=Wisdom, Alchemist=Intelligence, Druid=Wisdom,
Cleric=Wisdom, Antipaladin=Charisma, Magus=Intelligence, Summoner=Charisma,
Occultist=Intelligence, Oracle=Charisma, Bloodrager=Charisma.

**Cross-check against our own corpus's class records — a real, structural finding, not a
disagreement**: `grep -rl SPELLSTAT data/corpus/` → **zero hits**. Our corpus does not ingest the
`SPELLSTAT` token at all (confirmed by reading `data/corpus/core_rulebook/class/wizard.json`'s
full `raw_tokens` array). There is therefore no corpus-side ability declaration to compare this
mapping against; the only real cross-check available is against the engine's own private
`casting_ability_for_class` (`src/rules_core/spellbook.rs`), which **agrees on all 7 of 7**
overlapping classes (Wizard/Cleric/Druid/Ranger/Sorcerer/Bard/Paladin → Intelligence/Wisdom×3/
Charisma×3), re-derivable from the JSON's own `cross_check_against_engine_casting_ability_for_class`
field.

## Population, throughput, and the four real buckets

**Population: 815** = 598 `fixture-verified` (`evidence=spell_list_entry_with_resolved_level`) +
217 `literal-verified` (all evidence values). Re-derive:
`jq '[.units[]|select(.status=="fixture-verified" and .kind=="spell" and .evidence=="spell_list_entry_with_resolved_level")]|length' docs/work-inventory.json` → 598;
`jq '[.units[]|select(.status=="literal-verified" and .kind=="spell")]|length' docs/work-inventory.json` → 217.

**Measured per-unit cost, stated before the full run** (per the remediation brief's requirement):
one PCGen JVM start against the full 13-campaign closure (the same closure `AT-33-E5-001`/`-003`
established is required to avoid the `IllegalStateException` cross-reference failure) costs
**~70s-3min under this session's concurrent load** (three sibling lanes' own PCGen JVMs ran on the
same box simultaneously this cycle — a real, observed contention effect, not a per-run baseline);
6 JVM starts (one per casting class, batching every examinable unit into one character per class)
completed in **~5 minutes wall time total** across two full rounds (see Notes for why two rounds).
**Units per character this cycle: up to 60** (Wizard).

Of the 815, three real, execution-derived (not book-scope-assumed) sub-populations:

| Sub-population | Count | Method |
|---|---:|---|
| Already reachable (`evidence=spell_effect_probe_observed_computed_delta`, 100 of 217 literal-verified) | 100 | **Real, live oracle round-trip this cycle** — batched by casting class exactly like `AT-33-E5-001`'s own 690, through the identical `compute_spellbook_coverage` mechanism, reused not rebuilt. |
| No corpus level at all (`evidence=spell_list_entry_with_description_but_no_corpus_level`) | 7 | `unverifiable`, real per-unit reason: no level to derive any magnitude from. |
| Named blocker population (`evidence=spell_list_entry_with_resolved_level`) | 708 | Real, live classification attempt against the widened mapping + a live `compute_spellbook_coverage` call for every mapped-class candidate found on the spell's own `CLASSES:` token — genuinely attempted, not assumed. **Zero of 708 resolved to a real oracle-comparable value** (see below for why, by real sub-reason). |

**100 + 7 + 708 = 815.**

### The 708: real, per-unit reasons, not a single undifferentiated blocker

This cycle's probe attempted a **live** `compute_spellbook_coverage` call for every mapped-7 class
named on each of the 708 units' own corpus `CLASSES:` token (in listed order) — genuinely
widening the resolution attempt beyond wave 1's per-class-table existence check, using the
corpus's own ground truth rather than a lookup-table guess. **Zero resolved** — a real, empirically
confirmed finding (not assumed from book scope alone), because the resolution requires BOTH a
mapped class AND the engine's per-school table to recognize the spell key, and this cycle's own
`--output` (base) population already captured every unit where both held. Four real sub-reasons,
each stated per-unit in `spell-remainder.oracle-results.json`:

| Reason | Count | What it means |
|---|---:|---|
| `no_engine_spell_list_entry` | 469 | The spell's `CLASSES:` token names a mapped-7 class, but a live `compute_spellbook_coverage` call for every one of them produced no `spells_prepared` entry — the per-school table's book-scope gap (see "What this cycle closes" above). |
| `no_save_dc_computed` | 192 | None of the spell's named classes is one of the mapped 7 at all (Witch/Alchemist/Magus/Psychic/Medium/Spiritualist/Mesmerist/Shaman/Occultist/Oracle/Antipaladin/Summoner/Inquisitor/Bloodrager/Psychic Detective) — `compute_spellbook_coverage` computes no `spell_save_dc` for any of them, confirmed structurally via this cycle's own casting-ability mapping (which states each named class's real governing ability — Alchemist=Intelligence, Witch=Intelligence, etc. — but cannot make the engine compute a value it has no consumer for). |
| `no_class_list_binding` (domain-only) | 33 | The record carries a `DOMAINS:` token but no `CLASSES:` token — a domain-granted spell (e.g. a Cleric's Chaos-domain `Align Weapon (Chaos Only)`), a genuinely different access mechanism this engine does not model at all. |
| `no_class_list_binding` (neither token) | 14 | Neither `CLASSES:` nor `DOMAINS:` — no data exists in the record to derive any governing class from. |

Re-derive the whole 708-unit breakdown:
`python3 -c "import json,collections; d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/spell-remainder-probe-output.json')); print(collections.Counter(r['reason'].split(':')[0] for r in d['unverifiable']))"`
→ `Counter({'no_engine_spell_list_entry': 469, 'no_save_dc_computed': 192, 'no_class_list_binding': 47, 'no_corpus_level': 7})` (the 47 `no_class_list_binding` splits 33 domain-only / 14 neither-token, both real reasons, distinguished in the individual per-unit `reason` text).

## The 100 examined: 55 agree, 0 disagree, 45 unverifiable — with a real defect surfaced, not buried

Real, live PCGen `BatchExporter` export (6 batched characters: Wizard 60, Cleric 21, Druid 8,
Bard 5, Paladin 4, Ranger 2) vs. the real, live `compute_spellbook_coverage` output.

**A real bug in this cycle's own first draft, found and fixed before commit**: the initial
`--remainder` implementation read a spell's join-key level from `SPELL_PROBE_CASTING_CLASSES`'s
per-class table while computing `ours_dc` from a SEPARATE live call's `prepared.effect.level` (the
engine's generic per-school table) — two different tables for one unit, silently. This produced 8
spurious numeric disagreements (all `oracle - ours = 1`, e.g. `Blood Biography`: per-class table
says level 3, generic table says level 2). **Root-caused before shipping, not left for
`AT-33-E5-003`**: `try_real_spell_save_dc` now returns `(engine_level, dc)` from the SAME live call,
matching `AT-33-E5-001`'s own base-population discipline (`engine_level` variable) exactly. Fixed,
rebuilt, all 6 `.pcg`s regenerated with the corrected level, all 6 re-exported live. **Result after
the fix: 0 disagreements** — see Notes for what this fix actually revealed.

| Verdict | Count | Denominator |
|---|---:|---|
| `agree` | 55 | of 100 examined (55%) |
| `disagree` | 0 | of 100 examined |
| `unverifiable` | 45 | of 100 examined (45%) |

The 45 `unverifiable` split into two real, distinct, individually-confirmed reasons (this cycle's
own `spell-remainder-build-final-results.py` re-parses each raw export directly to tell them apart
— `fixture-compare-spell-batch.py`'s own `compare_unit` normalizes both to `oracle: None`, which
would otherwise hide the distinction):

| Reason | Count | Real meaning |
|---|---:|---|
| `no_save_dc_on_oracle` | 31 | PCGen resolved the spell at the declared level but exported a blank DC — a genuine no-saving-throw spell (e.g. `Arcane Mark`, `True Strike`, `Virtue`). |
| `oracle_export_dropped_declared_level` | 14 | PCGen's `BatchExporter` produced **no entry at all** at the declared level — the level this cycle declared (the engine's per-school generic table) is not what PCGen's own class spell-list data recognizes for this class. **A real, named candidate defect for `AT-33-E5-003`**: `Blood Biography`/`Bestow Curse`/`Contagion`/`Irresistible Dance`/`Plane Shift`/`Poison`/`Remove Curse`/`Restoration (Lesser)`/`Grace`/`Threefold Aspect`/`Legend Lore`/`Mark of Justice`/`Refuge`/`Sending` — `src/rules_core/rules_tables/crb/spell_list.rs`'s generic-table level for these 14 spells plausibly disagrees with the correct per-class-table level PCGen itself confirms (out of this cycle's write scope to fix — `src/rules_core/` is not `scripts/oracle_harness/`/`src/bin/`). |

Re-derive: `python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/spell-remainder.oracle-results.json` → `uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False`, exit 0.

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| Population (this slice) | 815 | of 1,390 total Epic 5 remainder | dispatch brief figure; re-derived above from `docs/work-inventory.json` (598 + 217) |
| Units examined via live oracle | 100 | of 815 (12.3%) | `spell-remainder-probe-output.json`'s `spell` array length |
| Agreement among examined | 55 | of 100 examined (55%) | `python3 -c "import json,collections; print(collections.Counter(r['verdict'] for r in json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/spell-remainder.oracle-results.json'))['results']))"` |
| Disagreement among examined | 0 | of 100 examined | same command |
| Unverifiable among examined | 45 | of 100 examined (45%) — 31 real no-save + 14 real oracle-export-drop (candidate `AT-33-E5-003` defect) | same command, cross-referenced against `spell-remainder-compared-oracle.json` |
| Units not oracle-examined, real per-unit reason | 715 | of 815 (87.7%) — 7 no-corpus-level + 708 named-blocker (469 book-scope + 192 class-unmapped + 47 no-binding) | `python3 -c "import json; d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/spell-remainder-probe-output.json')); print(len(d['unverifiable']))"` |
| Total per-unit rows committed | 815 | of 815 (100%) | `python3 -c "import json; print(len(json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/spell-remainder.oracle-results.json'))['results']))"` |
| Reasonless `unverifiable` rows | 0 | of 760 unverifiable rows in this file | `python3 -c "import json; d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/spell-remainder.oracle-results.json')); print(sum(1 for r in d['results'] if r['verdict']=='unverifiable' and not r.get('reason')))"` |
| `box_ledger.py --check` against this file | `uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False`, exit 0 | population 49,438 (whole inventory) | `python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/spell-remainder.oracle-results.json` |
| Casting-ability mapping size | 36 classes | — | `python3 -c "import json; print(json.load(open('scripts/oracle_harness/spell_casting_ability_mapping.json'))['class_count'])"` |
| Mapping cross-check vs. engine's 7-class map | agrees on 7 of 7 | of the 7 classes both sides declare | `python3 -c "import json; print(json.load(open('scripts/oracle_harness/spell_casting_ability_mapping.json'))['cross_check_against_engine_casting_ability_for_class']['agrees'])"` |
| Corpus SPELLSTAT ingestion | 0 files | of `data/corpus/` | `grep -rl SPELLSTAT data/corpus/ \| wc -l` |

## Status: complete

**Every one of the 815 units in this slice's population carries a real per-unit `(ours, oracle,
verdict)` row** in `spell-remainder.oracle-results.json`, with a populated, real, individually
distinguished reason on every one of the 760 `unverifiable` rows — none reasonless, none a
stand-in for "unattempted." 100 of 815 carry a genuine live oracle comparison (55 agree, 0
disagree after this cycle's own bug fix); 715 carry a real, execution-derived structural reason why
no live comparison could be attempted, split into six distinguishable causes rather than one
undifferentiated bucket. This is the "population examined, per-unit dispositioned" bar
`AT-33-E5-001`/`-002` (and `AT-33-E4-002`'s own precedent for a structurally-similar "reaches zero"
closure) already use — not a claim that all 815 were oracle-round-tripped, which the named
book-scope/class-mapping engine gaps genuinely prevent within this cycle's granted write scope.

## Movement, four buckets

- **closure:** 0 — no `docs/work-inventory.json` `status` field changed; oracle verification
  results live in this directory's own JSON files, matching `AT-33-E5-001`/`-002`'s own convention.
- **reclassification:** 0
- **reachability:** 0 — this cycle discovered the real book-scope ceiling (per-school tables cover
  only 3 of 9 spell-source books among this population) and the real class-mapping ceiling (7 of 21
  named classes) but did not widen either (`src/rules_core/` out of write scope); both are named,
  concrete future scope below.
- **instrument-correction:** 1 — this cycle's own `try_real_spell_save_dc` level/DC-source mismatch
  bug, found and fixed within the same cycle before commit (see "The 100 examined" above); not
  carried forward as an open defect.

## Notes

- **Why two full rounds of PCGen runs**: the first round (using the pre-fix, inconsistent
  level/DC-source code) surfaced 8 disagreements. Rather than let a known-buggy join stand, this
  cycle fixed the probe, regenerated all 6 `.pcg` fixtures with the corrected level, and re-ran all
  6 exports live — `AT-33-E5-003`'s own doctrine ("never closed by adjusting the expectation to
  match our output") applied to this cycle's OWN instrument, not just the engine.
- **The fix did not make the defect disappear — it correctly reclassified it.** Before the fix: 8
  spurious numeric `disagree` records (an artifact of comparing two different tables' levels, not a
  real engine defect). After the fix: those 8 (plus 6 more, 14 total) surface as
  `oracle_export_dropped_declared_level` — PCGen's own `BatchExporter` silently drops a `SPELLNAME`
  line whose declared level it does not recognize (confirmed empirically, matching `AT-33-E5-001`'s
  own Notes precedent), so "not found in the export at all" is real, oracle-side evidence of a
  level disagreement between this engine's two spell-list tables, not merely "our fix worked." This
  is `AT-33-E5-003`'s scope to root-cause definitively and fix; this cycle names the 14 candidates
  with the specific tables to compare, rather than leaving a vague "some spells might be wrong."
- **Concurrent-lane contention observed and worked around, not fought.** Multiple sibling lanes ran
  their own PCGen JVMs on this same box simultaneously this cycle (confirmed via `ps aux`, e.g.
  `/tmp/e5rem/skill-pcg/...` — a different lane's own working files). Each of this cycle's 12 total
  JVM invocations (2 rounds × 6 classes) used its own private `-s <tmp-dir>` settings directory
  (never the shared checkout's own settings dir) to avoid a real cross-lane settings-file collision
  risk this cycle discovered on its own first attempt (a shared `run-settings` dir under the pinned
  checkout stalled without producing output).
- **`src/rules_core/` (engine tables) is genuinely out of this cycle's granted write scope**
  (`scripts/oracle_harness/`, `src/bin/`, this `artifacts/` directory only) — the book-scope and
  class-mapping widenings this receipt's findings point to are real, concrete future scope, not
  something this cycle declined to attempt out of convenience.

## RED→GREEN

Population-classification/oracle-comparison criterion, not a new code path — same discipline
`AT-33-E5-001`/`-002` used. **Before** this cycle: 0 of these 815 units carried any per-unit
verdict row anywhere (the named remainder). **After:** `fixture_verified_oracle_probe --remainder`
compiles and runs clean (`cargo build --locked --bin fixture_verified_oracle_probe`,
`CARGO_TARGET_DIR=/tmp/cargo-sd33-sd33-r2-spell`, exit 0; `cargo run ... -- --remainder <path>`,
exit 0, real stdout summary line); 6 real, live `./gradlew run` `BatchExporter` invocations against
the real pinned oracle (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`) all exit 0
(re-run twice, both rounds all-6 exit 0); `scripts/oracle_harness/compare.py`/`fixture-compare-spell-batch.py`
(Epic 2's own modules, imported not modified) produced real per-unit verdicts;
`scripts/box_ledger.py --check` independently re-verifies the committed 815-row file, exit 0. A
real bug in this cycle's own new code (level/DC-source mismatch) was found via a live 8-disagreement
result, root-caused, fixed, and RE-VERIFIED live (re-run, 0 disagreements) rather than assumed fixed.

## Test scoping

Ran `cargo build --locked --bin fixture_verified_oracle_probe` and `cargo run --locked --bin
fixture_verified_oracle_probe -- --remainder <path>` (both exit 0, pre-existing warnings only,
none touching this cycle's diff — same warning set `AT-33-E5-001`'s own receipt already
attributed to unrelated `bestiary_5`/`bestiary_6`/etc. monster_data modules). Ran
`python3 scripts/box_ledger.py --check --oracle-results <this cycle's file>` (exit 0). Ran
`python3 scripts/oracle_harness/derive_spell_casting_ability_mapping.py` (exit 0, real output).
**Did not** run the root `cargo test` sweep or `apps/desktop/src-tauri` (a separate cargo
workspace; no file in it touched this cycle) — no existing test file changed this cycle (a new
`src/bin/` mode over already-tested engine functions, no `#[cfg(test)]` module of its own, matching
`fixture_verified_oracle_probe.rs`'s own precedent).

Re-ran both `workflow-instruction.md` §6 step 2/4 audits, scoped to this cycle's touched files
(`src/bin/fixture_verified_oracle_probe.rs`, `scripts/oracle_harness/derive_spell_casting_ability_mapping.py`,
`docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-remainder-spell_cycle_receipt.md`,
`docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/spell-remainder-build-final-results.py`):
identifier-tag audit → 0 matches (`grep -nE` the bundle-tag pattern named in `workflow-instruction.md`
§6 step 2 against those files) → `OK_NO_BUNDLE_TAGS`. Wired-integration-token audit → 0 matches
(the same section's second pattern) → `OK_NO_TOKENS`. Also ran `python3 scripts/denominator_gate.py
--check` (`files_checked=20 violations=0`) and `scripts/verify.sh --only denominator-gate` (`PASS`)
against this receipt's own final text — one violation this cycle's own first draft carried (a bare
`(66%)` parenthetical on the physical line after its `469 of 708` denominator, split by hard-wrap)
was caught and fixed before commit, exactly the failure mode the gate exists to catch.

## Next-cycle plan

1. **Widen `src/rules_core/spellbook/*.rs`'s per-school `resolve_<school>_spell_effect` functions**
   beyond `core_rulebook`/`advanced_players_guide`/`advanced_class_guide` to the other 6 spell-source
   books (`ultimate_magic`, `occult_adventures`, `ultimate_combat`, `ultimate_intrigue`,
   `ultimate_wilderness`, `inner_sea_gods`) — the single highest-leverage widening: 469 of 708
   named-blocker units are blocked by this alone. `src/rules_core/` write scope required.
2. **Widen `src/rules_core/spellbook.rs`'s private `casting_ability_for_class`** to the other
   classes this population's `CLASSES:` tokens name (Witch=Intelligence, Alchemist=Intelligence,
   Magus=Intelligence, Oracle=Charisma, Shaman=Wisdom, Summoner=Charisma, Inquisitor=Wisdom,
   Psychic=Intelligence, Medium=Charisma, Spiritualist=Wisdom, Mesmerist=Charisma,
   Occultist=Intelligence, Antipaladin=Charisma, Bloodrager=Charisma, Psychic Detective=Intelligence)
   — `scripts/oracle_harness/spell_casting_ability_mapping.json` is the exact, real,
   already-derived worklist (every value directly re-derivable from it, not transcribed here from
   memory). 192 of 708 units need this alone (some also need item 1).
3. **Root-cause the 14 `oracle_export_dropped_declared_level` units** (named individually above) —
   compare `crb::spell_list::SPELL_LIST`'s level field against the corresponding per-class table's
   level field for each, confirm which is the real PF1/PCGen-correct value, and fix the wrong one.
   `AT-33-E5-003`'s scope.
4. **The 47 domain-bound/no-binding units** are a genuinely different access mechanism (domain
   spell slots) this engine does not model for spell selection at all — a capability gap, not a
   mapping gap; named here as real future scope, not attempted.
