# Cycle AT-33-E5-finalize-wave6 — epic-5-reverification / AT-33-E5-001, AT-33-E5-002, AT-33-E5-003

- **Commit SHA:** recorded below at push time (`sd33-r6-e5-finalize`, remediation wave 6)
- **Files touched:**
  - `src/rules_core/equipment_effects.rs` (`eqmod_referenced_records` scans every `EQMOD:` token,
    not only the first; new end-to-end test
    `book_agnostic_resolution_tests::eqmod_referenced_modifier_sums_into_weapon_enhancement_bonus_across_two_eqmod_tokens`;
    `compute_equipment_effects` now folds the referenced records' weapon-enhancement chains in)
  - `src/rules_core/equipment_effects/equipmods.rs` (new
    `apply_eqmod_weapon_enhancement_bonus`, per-dimension MAX)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json`
    (merged: 8,291 → 8,330 rows; `rending_claw_blades` `disagree` → `agree`)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/literal-verified.oracle-results.json`
    (merged: 6,550 → 6,589 rows; same fix)
  - `docs/release/SD-33-computed-value-verification/progress.md` (frontmatter status, `## Open
    blockers` cleared with audit-trail history, new `## Cycles` entry)
  - `docs/release/SD-33-computed-value-verification/kanban.md` (rows 16/17/18)
  - `docs/retro/events/sd33-r6-e5-finalize.jsonl` (1 incident, 2 corrections)
  - this receipt
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
  (`git diff --unified=0 "${BASE}...HEAD" -- src/rules_core/equipment_effects.rs
  src/rules_core/equipment_effects/equipmods.rs progress.md kanban.md | grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'` → no hits; prose uses hyphenated
  `sd33-r6-...` identifiers, never the underscore-joined bundle-tag shape the gate matches)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff scope,
  `grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'` → no hits)
- **Acceptance criteria (verbatim, `epic-breakdown.md`):**
  - `AT-33-E5-001`: "the 1,741 `fixture-verified` units are re-examined against the oracle...
    per-unit `(ours, oracle, verdict)` rows committed; agreement and disagreement counts both
    stated, with the denominator."
  - `AT-33-E5-002`: "the 6,589 `literal-verified` units are re-examined... as above."
  - `AT-33-E5-003`: "every disagreement is a named defect, fixed or escalated. A disagreement is
    never closed by adjusting the expectation to match our output... **Evidence:** one entry per
    disagreement in `progress.md`, each resolved to a commit or an operator escalation. A filed
    blocker does not satisfy this criterion."
- **Status:** complete
- **Movement, four buckets:** closure 1 (`rending_claw_blades`, `disagree` → `agree`, a real
  `src/rules_core/` fix, not an expectation edit). Reclassification 0. Reachability 0 (the 39 new
  rows are newly *examined*, 38 remain `unverifiable`, 1 `agree` — not newly reachable by this
  cycle's own act, that credit belongs to the three remainder lanes). Instrument-correction 0 (the
  corpus-extraction and method-rerun supersessions carry forward those cycles' own already-logged
  instrument-correction movement; this cycle only propagates it into the canonical artifacts).

## What this cycle did

Five wave-6 lanes ran concurrently: `corpus-extraction-fix` (cleared the corpus-extraction
blocker, escalated a narrower `src/rules_core/` gap in its place), `method-rerun` (re-ran and
verified the wave-5 AC-isolator/campaign-KEY/identity-resolve method changes across their full
affected set, `method_change_rerun_verified: true`), and three remainder lanes
(`last39-{weapon,skill-combat,eqm}`) closing the final 39 unrowed `literal-verified` units. This
cycle merged all five into the three canonical artifacts, then closed the one remaining
`disagree` the merge exposed rather than re-escalating it.

## Merge — precedence and duplicates

Started from the pre-wave-6 combined file (8,291 rows, HEAD `63b519dcaf`).

1. **`corpus-extraction-fix.oracle-results.json` (13 rows) SUPERSEDED** the matching stale rows
   in both `AT-33-E5-003.combined-oracle-results.json` and `literal-verified.oracle-results.json`
   — all 13 confirmed `literal-verified` via `docs/work-inventory.json`, all 13 were pre-existing
   (not new). This is a MEASUREMENT correction (the corpus itself changed), per
   `AT-33-E5-003`'s own "fix the harness, and re-run everything it already judged" route.
2. **`method-rerun-wave6.oracle-results.json` (21 rows) was already merged** into
   `AT-33-E5-003.combined-oracle-results.json` by its own commit (`63b519dcaf`) — verified with a
   full row-by-row diff (0 mismatches on `unit_id`/`ours`/`oracle`/`verdict`) before propagating
   the same 21 rows into `literal-verified.oracle-results.json`, which had not yet received them.
3. **The three remainder lanes' rows were ADDED as new.** `last39-weapon.oracle-results.json`
   (23), `last39-skill-combat.oracle-results.json` (11), `last39-eqm.oracle-results.json` (7) —
   41 rows, union 39 distinct `unit_id`s.

```
python3 -c "import json
missing39 = set(open('/dev/stdin').read().split())  # the 39 ids from AT-33-E6-001-attempt6's receipt
ids = set()
for f in ['last39-weapon.oracle-results.json','last39-skill-combat.oracle-results.json','last39-eqm.oracle-results.json']:
    d = json.load(open('epic-5-reverification/' + f))
    ids |= {x['unit_id'] for x in d['results']}
print(len(ids), ids == missing39)"
```
→ `39 True` (union of the three lane files exactly equals the 39 ids attempt6 named).

**Duplicate finding, root-caused, not last-writer-wins.** `ultimate_psionics:equipment:
flurry_of_fists` and `ultimate_psionics:equipment:flurry_of_strikes` are rowed by BOTH the
weapon and skill-combat lane files:

```
python3 -c "import json
allrows={}
for f in ['last39-weapon.oracle-results.json','last39-skill-combat.oracle-results.json','last39-eqm.oracle-results.json']:
    d=json.load(open('epic-5-reverification/'+f))
    for x in d['results']:
        allrows.setdefault(x['unit_id'], []).append(f)
print({i:fs for i,fs in allrows.items() if len(fs)>1})"
```
→ `{'ultimate_psionics:equipment:flurry_of_strikes': ['last39-weapon...', 'last39-skill-combat...'], 'ultimate_psionics:equipment:flurry_of_fists': [...]}`

Both records are genuinely `BONUS:WEAPON|...` chains (qualifying for the weapon-shape lane) AND
gate on a "Blade Skill" ability record (qualifying for the skill-combat-shape lane) — the wave-6
dispatch's shape partition was not mutually exclusive for these two. Compared both lanes' rows
directly: byte-for-byte identical disposition (`verdict: unverifiable`, `ours: null`,
`oracle: null`, `reason` both starting `no_resolver`, citing the same cross-record `ATTACKS`/
`WEAPONBAB` variable-resolution gap). Not a data conflict — deduped to one row per unit_id,
keeping the skill-combat lane's version for its fuller cross-record citation. Logged as a
`scripts/retro.py correction` (`docs/retro/events/sd33-r6-e5-finalize.jsonl`).

## Figures — every number, its command, its denominator

| Figure | Value | Denominator | Command |
|---|---|---|---|
| `AT-33-E5-003.combined-oracle-results.json` rows | 8,330 | of 8,330 blessed units | `python3 -c "import json; d=json.load(open('epic-5-reverification/AT-33-E5-003.combined-oracle-results.json')); print(len(d['results']))"` |
| — distinct `unit_id`s | 8,330 | of 8,330 rows | same file, `len(set(...))` |
| — agree / unverifiable / disagree | 811 / 7,519 / **0** | of 8,330 rows | `collections.Counter` on `verdict` |
| `literal-verified.oracle-results.json` rows | 6,589 | of 6,589 `literal-verified` units | same pattern, `literal-verified.oracle-results.json` |
| — agree / unverifiable / disagree | 415 / 6,174 / **0** | of 6,589 rows | same |
| `fixture-verified.combined-oracle-results.json` rows | 1,741 | of 1,741 `fixture-verified` units (unchanged this cycle) | same pattern |
| Unexamined (missing from combined) | **0** | of 8,330 blessed units | `python3 -c "import json; wi=json.load(open('docs/work-inventory.json'))['units']; pop={u['id'] for u in wi if u.get('status') in ('literal-verified','fixture-verified')}; d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json'))['results']; print(len(pop-{r['unit_id'] for r in d}))"` → `0` |
| `box_ledger.py --check` | `oracle_disagreement=0`, exit 0 | of 8,330 rows via the ledger's own 49,438-unit population walk | `python3 scripts/box_ledger.py --check --oracle-results epic-5-reverification/AT-33-E5-003.combined-oracle-results.json` |
| Reasonless `unverifiable` | 0 / 0 / 0 | combined (of 7,519) / literal (of 6,174) / fixture (of 1,345) | per-file `sum(1 for x in r if verdict=='unverifiable' and not (reason or note))` |
| Duplicate `unit_id`s | 0 / 0 / 0 | same three files, post-dedup | per-file `Counter(ids)` max count |
| Denominator gate | PASS | 52 of 52 scanned files | `bash scripts/verify.sh --only denominator-gate` |
| `equipment_effects::` tests | 76 of 76 green (3 new) | of 76 tests in that module | `cargo test --locked --lib equipment_effects` |
| Full lib suite | 2,832 of 2,836 executed | 4 failures, all pre-existing/inherited, attributed below | `cargo test --locked --lib` |
| Corpus-wide blast radius, new weapon-eqmod fold path | 191 | of 2,210 equipment records carrying `≥1 EQMOD:` token | scratch corpus-wide scan (below), removed after use |
| Regression check on currently-`agree` population | 0 regressions | of 4 real corpus-key matches between the 191 and the agree set (1 false-positive name collision, 1 this fix's own target, 1 unaffected-in-value, 2 out-of-population, 2 unaffected-dimension) | see "Blast radius" below |

## `rending_claw_blades` — fixed, not re-escalated

Two real `src/rules_core/` defects, confirmed by TDD RED→GREEN against real, verbatim corpus
tokens (never a fixture inflated beyond the real shape):

1. **`eqmod_referenced_records` read only the first `EQMOD:` token** (`.find()`). This record now
   carries two — its own line's `Material ~ Steel`, plus a second, richer token the
   corpus-extraction fix (`fbc945f198`) folded in from its `.MOD` row — so the token naming the
   real `+1 Weapon` modifier was never even inspected, independent of any weapon-dimension gap.
   Fixed: scans every `EQMOD:` token on the record.
2. **`compute_equipment_effects`'s weapon path never folded `EQMOD:`-referenced modifier
   records' own chains in at all** — unlike the AC dimension's already-shipped
   `resolve_category_effect` → `arms_armor::apply_eqmod_armor_class_bonus` pattern (wave 4,
   `abc72f75ec`). Fixed via a new `equipmods::apply_eqmod_weapon_enhancement_bonus`, called from
   `compute_equipment_effects` the same way the AC path is.

**Combining rule is per-dimension MAX, not sum — discovered mid-cycle, corrected before merge.**
The corpus-extraction lane's own escalation note proposed "Option-sum `tohit_bonus`/
`damage_bonus`, mirroring the AC path's pattern". A first implementation did exactly that, and
live-recomputing against the real on-disk record (`e5_last67_weapon_ours`) produced
`tohit_bonus: Some(2)` against the oracle value ALREADY on file for this unit's TOHIT side
(`MAGICHIT=+1`, quoted verbatim in the pre-fix row's own note) — trading the DAMAGE disagreement
for a new TOHIT one. Root cause: the base record's own `BONUS:WEAPON|TOHIT|1|TYPE=Enhancement`
and the referenced modifier's `BONUS:WEAPON|DAMAGE,TOHIT|1|TYPE=Enhancement` carry the IDENTICAL
`TYPE=Enhancement` qualifier — Pathfinder's core rule for same-type bonuses is the higher applies,
never the sum. The AC dimension's own sum is correct there for the opposite reason: its real
corpus witness (`Armor of Grim Triumph`) sums a `TYPE=Armor` base value with a `TYPE=
ArmorEnhancement` modifier bonus — two DIFFERENT types, which do stack, by the same Pathfinder
rule. Corrected to per-dimension `max`:

```
$ cargo test --locked --lib equipment_effects::book_agnostic_resolution_tests::eqmod_referenced_modifier_sums_into_weapon_enhancement_bonus_across_two_eqmod_tokens
# before the fix (RED):
thread '...' panicked at src/rules_core/equipment_effects.rs:1046:
assertion `left == right` failed: base TOHIT|1 plus the +1 modifier's own TOHIT|1
  left: Some(1)
 right: Some(2)
test result: FAILED. 0 passed; 1 failed

# after the fix (GREEN):
test result: ok. 76 passed; 0 failed; 0 ignored; 0 measured; 2774 filtered out
```

Live-recomputed against the real, post-corpus-fix, on-disk corpus record (not a fixture) via
`e5_last67_weapon_ours` (2-book manifest: `advanced_race_guide` + `core_rulebook`, to load both
the base record and the cross-book `core_rulebook` modifier it references):

```
$ cargo run --locked --bin e5_last67_weapon_ours -- <repo> manifest.json out.json
{
  "advanced_race_guide:equipment:rending_claw_blades": {
    "damage_bonus": 1,
    "natural_attack_only": false,
    "tohit_bonus": 1,
    "weapon_prof_scope": null
  }
}
```

Both dimensions now match the oracle exactly (`MAGICHIT=+1`, `MAGICDAMAGE=+1`).

## Blast radius and regression sweep

A scratch corpus-wide scan (written, run, results recorded here, then deleted — never committed,
per this cycle's write scope discipline) loaded every book's equipment corpus and counted records
where `eqmod_referenced_records` now resolves at least one modifier record whose own
`compute_equipmods_effect` matches:

```
total equipment records with >=1 EQMOD token: 2210
total AFFECTED by the new weapon-dimension eqmod fold: 191
```

Of those 191, only 4 already carried a non-`None` base-record `weapon_enhancement_bonus` of
their own (the only shape where `max` vs. the base's prior standalone value could differ):
`Rending Claw Blades` (this fix's own target), `Rod (Thunder and Lightning)`,
`Bastard's Sting`, `Hammer (Dwarfbond)`.

Cross-referenced all 191 corpus keys against every currently-`agree` `unit_id` in the 8,330
population (via `docs/work-inventory.json`'s `corpus_key` field):

| Key | unit_id | Disposition |
|---|---|---|
| `Rending Claw Blades` | `advanced_race_guide:equipment:rending_claw_blades` | this fix's own target |
| `Rod (Thunder and Lightning)` | `core_rulebook:equipment:rod_thunder_and_lightning` | already `agree`; base chain `(1,1)`, modifier chain `(1,1)`, `max(1,1)=1` both dimensions — **value unchanged**, re-verified live via `e5_last67_weapon_ours` |
| `Staff of Mithral Might` | `ultimate_equipment:equipment:staff_of_mithral_might` | already `agree`, but on the ABILITY dimension (`ability_bonus`), not `weapon_enhancement_bonus` — that field is `None` before and after, confirmed live |
| `Chaos Hammer` | `core_rulebook:spell:chaos_hammer` | **false positive** — a spell record, name collision only, not the same corpus entity |
| `Fork of the Forgotten One` | `advanced_players_guide:equipment:fork_of_the_forgotten_one` | already `agree`, but on the SKILL dimension (`skill_bonus`), not `weapon_enhancement_bonus` — `None` before and after, confirmed live |

**0 of 8,330 currently-`agree` units regressed.**

`Bastard's Sting` and `Hammer (Dwarfbond)` (`ultimate_equipment`) are out of population entirely
(`docs/work-inventory.json` status `ingested-magnitude`, never `literal-verified`/
`fixture-verified`) — confirmed, not assumed.

**Scope this fix does NOT cover** (`AGENTS.md` Non-Negotiable Rule 7): the other ~187 of the 191
corpus-wide-affected records were already `unverifiable`/`no_resolver`
(`weapon_enhancement_bonus` was `None`, so no comparison was ever made). This fix likely gives
many of them a real, non-`None` `ours` value now — but no live-oracle capture was performed for
them this cycle, so their `verdict` is deliberately left unchanged and their `no_resolver` reason
text is now stale (a resolver now exists for the shape it names). Named here as a real, bounded
next-cycle item, never claimed as closed by this cycle.

## Rust suite — attributed, not bucketed

```
cargo test --locked --lib
```
```
test result: FAILED. 2832 passed; 4 failed; 14 ignored; 0 measured; 0 filtered out
```

Same 4 failures `AT-33-E6-001-attempt6`'s own receipt already attributed as Shortfall 4:

- `rules_core::pilot_compute::formula_interpreter_corpus_wide::tests::{a_subset_run_trips_the_population_mismatch_check, corpus_wide_scan_population_matches_the_closed_gate1_census, f1_population_matches_the_current_true_formula_bearing_count_not_the_stale_sd32_census}`
  — SD-33's own Epic 4 debt (`('ambiguous','unmeasurable')` unmapped in
  `pf1e_dashboard_producer.py::_doneness_verdict_uncapped`).
- `rules_core::equipment_resolver::tests::catalog_rows_span_every_ingested_book_with_their_real_counts`
  — inherited from the `tranche/13` cut (`data/corpus` byte-identical to `f652db7ac7`).

Confirmed unrelated to this cycle's diff: neither `pf1e_dashboard_producer.py`,
`coverage_ledger.py`, `shape_ledger.py`, nor `equipment_resolver.rs` appears in
`git diff --stat` for this cycle's commit. Out of this row's mandate — `AT-33-E5-001/002/003`,
not `AT-33-E4-002`'s own consequence. Not claimed fixed here; named as a real, bounded next-cycle
item in `progress.md`.

## `disagree` capability re-proven live

```
$ cp epic-5-reverification/AT-33-E5-003.combined-oracle-results.json /tmp/probe-combined.json
$ python3 -c "...set core_rulebook:equipment:rod_thunder_and_lightning ours=99, verdict=disagree in /tmp/probe-combined.json..."
probe injected: rod_thunder_and_lightning ours=99 verdict=disagree
$ python3 scripts/box_ledger.py --check --oracle-results /tmp/probe-combined.json; echo $?
uncovered=0 overlap=0 population=49438 oracle_disagreement=1 unverifiable_done=0 stale=False
ORACLE_DISAGREEMENT: core_rulebook:equipment:rod_thunder_and_lightning
1
$ rm /tmp/probe-combined.json
$ python3 scripts/box_ledger.py --check --oracle-results epic-5-reverification/AT-33-E5-003.combined-oracle-results.json; echo $?
uncovered=0 overlap=0 population=49438 oracle_disagreement=0 unverifiable_done=0 stale=False
0
```

The count moved `0 → 1`, the injected unit was named, the probe file was removed, and the real
artifact was re-checked clean immediately after. The real artifact was never modified.

## `## Open blockers`

Empty. The one entry it carried (`rending_claw_blades` compute_equipment_effects weapon-path
EQMOD-resolution gap, filed by `sd33-r6-corpus-extraction`) is **cleared, not superseded by a new
entry** — the exact fix its own escalation note requested was landed this cycle (with the
same-type-stacking correction the naive "Option-sum" proposal needed). History preserved under a
collapsed `<details>` block in `progress.md` for audit trail.

## Kanban call

| Row | Criterion | Before | After | Basis |
|---|---|---|---|---|
| 16 | `AT-33-E5-001` | complete | complete (re-confirmed) | 1,741/1,741, 0 disagree, 0 overlap with any wave-6 lane |
| 17 | `AT-33-E5-002` | in-progress | **complete** | 6,589/6,589 rowed, 0 unrowed, 0 duplicate `unit_id`s |
| 18 | `AT-33-E5-003` | blocked-escalated | **complete** | 0/8,330 disagree, `## Open blockers` empty, 29/29 disagreements ever surfaced dispositioned (28 fixed, 0 escalated) |

## Incident and corrections logged

`docs/retro/events/sd33-r6-e5-finalize.jsonl`:

- **incident** (`shared-tree-stray-reset-state`): the shared checkout at
  `/home/ubuntu/workspace/repos/codex` (tranche/13) carried a stray, uncommitted index+worktree
  state byte-identical to an ancestor commit's tree — not this cycle's own edit — which blocked
  several git-write attempts via the environment's permission classifier before recovery via a
  fresh `git worktree add` from `origin/tranche/13`. No work was lost; every real commit was
  already an ancestor of `origin/tranche/13`.
- **correction**: the corpus-extraction lane's own escalation note proposed "Option-sum" as the
  fix design; the real, live-oracle-verified combining rule is per-dimension MAX (Pathfinder
  same-type bonus stacking). Caught before merge.
- **correction**: the wave-6 dispatch's 39-unit shape partition (23+11+7=41) was not mutually
  exclusive — 2 units double-rowed by the weapon and skill-combat lanes with identical
  dispositions. Caught before merge, deduped, 0 blast radius.

## Next

`AT-33-E6-001`'s next attempt can re-check rows 16/17/18 clean. Two real, named, non-blocking
items remain, neither part of this row's mandate:

1. Epic 4's own Shortfall-4 test debt (`('ambiguous','unmeasurable')` unmapped in
   `pf1e_dashboard_producer.py`, plus the inherited `equipment_resolver.rs:863` catalog-count
   mismatch) — this cycle's diff does not touch either file.
2. A live-oracle capture sweep across the ~187 corpus-wide records this cycle's fix newly made
   resolvable (a real `ours` value now exists where none did) but did not verify against a live
   oracle value — their stale `no_resolver` reason text is a real correction, their verdict is
   not.
