# Cycle sd33-r6-method-rerun — Epic 5 Re-verification / AT-33-E5-003 (method-change re-run obligation)

- **Commit SHA:** recorded on landing (see `progress.md` entry `sd33-r6-method-rerun`)
- **Files touched:**
  - `src/bin/e6_identity_rerun_ours.rs` (new) — repo-local "ours" probe for the 5 identity-resolve-affected units, calling the SAME `equipment_id_resolve` / `compute_var_effect` / `compute_equipment_effects` functions every other `AT-33-E5-00x` probe binary calls (no new resolver logic).
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/method-rerun-wave6.oracle-results.json` (new — this cycle's primary deliverable, 21 rows).
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json` (21 rows corrected in place; row count unchanged, 8,291 before and after).
  - `docs/release/SD-33-computed-value-verification/progress.md` (Cycles entry, prepended).
  - `docs/retro/events/sd33-r6-method-rerun.jsonl` (new).
  - **`kanban.md` NOT touched this cycle** — per this dispatch's own coordination note, this lane re-runs already-judged rows only; it does not own the row 16/17/18 kanban call.
- **Identifier audit result:**
  ```
  $ grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' src/bin/e6_identity_rerun_ours.rs \
      docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/method-rerun-wave6.oracle-results.json \
    || echo OK_NO_BUNDLE_TAGS
  OK_NO_BUNDLE_TAGS
  $ git diff --unified=0 -- docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json \
      | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo OK_NO_BUNDLE_TAGS
  OK_NO_BUNDLE_TAGS
  ```
- **Wired-integration audit result:**
  ```
  $ grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' src/bin/e6_identity_rerun_ours.rs \
      docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/method-rerun-wave6.oracle-results.json \
    || echo OK_NO_TOKENS
  OK_NO_TOKENS
  $ git diff --unified=0 -- docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json \
      | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo OK_NO_TOKENS
  OK_NO_TOKENS
  ```
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-003 — every disagreement is a named defect, fixed or escalated
  >
  > A disagreement is **never** closed by adjusting the expectation to match our output. Each is
  > root-caused: either our computation is wrong (fix it) or the oracle comparison is wrong (fix the
  > harness, **and re-run everything it already judged**).

## Task

The attempt-6 final-acceptance scan (`AT-33-E6-001-attempt6_cycle_receipt.md`) recorded
`method_change_rerun_verified: false`. Wave 5 landed **three** corrections carrying the "fix the
harness / fix the engine, re-run everything it already judged" obligation: the AC isolator
(`a68fbeea3d`), the campaign-KEY fix (`9df1c0b514`, `scripts/oracle_harness/campaign_key.py`), and
the `equipment_id_resolve` identity fix (`9df1c0b514`, `src/rules_core/corpus_loader.rs`). Only the
first had a proven, complete re-run (66 of 66, already verified by `a68fbeea3d`'s own receipt) — and
even that one had a **propagation** gap (Shortfall 3: 2 of 66 re-run rows never reached the combined
file). The other two corrections' blast radius had never been derived or re-run at all.

## Blast-radius derivation, by execution, per correction

### 1. AC isolator (`a68fbeea3d`) — already fully re-run; propagation gap only

Wave 5's own receipt derived this correction's population by execution (`grep -rl "AC.TOTAL\|
baseline_diff\|item_AC.Total" scripts/oracle_harness docs/release/SD-33-computed-value-verification/
artifacts/epic-5-reverification/*.py` → only `combat-shape-work/ac_build_results.py`) and re-ran all
66 of that script's own already-judged manifest units live
(`full-rerun-wave5.oracle-results.json`, 66/66, 0 unresolved). Re-verified this cycle by re-reading
that file and re-confirming the grep still returns exactly one hit. **No new PCGen invocation needed
for this correction** — the gap was `AT-33-E6-001`'s Shortfall 3: 2 of the 66 re-run rows
(`ring_of_unquenchable_passions`, `goblin_plate`) were never merged into
`AT-33-E5-003.combined-oracle-results.json`, which still carried their pre-re-run values. **Closed
this cycle** — both rows re-affirmed verbatim from `full-rerun-wave5.oracle-results.json` and merged.

### 2. Campaign-key fix (`9df1c0b514`) — 14 of 14, derived and fully re-run live

**Population, derived by execution:**
```
$ python3 -c "
import json
d=json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json'))['results']
camp=[r for r in d if 'campaign' in (r.get('reason') or '').lower()]
print(len(camp))
for r in camp: print(r['unit_id'])"
15
```
(15 hits: 14 `ultimate_psionics` AC-shape units carrying
`oracle_harness_ultimate_psionics_campaign_load_failure`, plus 1 unrelated false-positive substring
match — `book_of_the_damned_volume_2:equipment:demon_senses`, whose reason merely *mentions* "the
pre-existing 'Could not find campaign' failure class" while describing a genuinely different,
already-live-tested `Implant`-slot defect. Excluded — confirmed by reading its full reason text.)

**Denominator: 14 of 14 rows carrying `oracle_harness_ultimate_psionics_campaign_load_failure`, all
re-run.** All 14 already had a real, live-computed `ours` value in the combined file (from the
general engine, unaffected by the campaign bug — none of the 14 are KEY-less/OUTPUTNAME-bearing
records; confirmed: `data.name == data.key` for all 14, direct corpus read) and all 14 already had a
**committed `.pcg` fixture** at `combat-shape-work/ac-pcg/<slug>.pcg` from an earlier lane's attempt
that hit the campaign bug (confirmed by the fixture's own `.txt.log` transcript: `SEVERE main
Globals:130 Could not find campaign: Ultimate Psionics`).

**Re-run live this cycle**, in a scratch copy (`campaignfix-pcg/`, committed fixtures on disk
untouched), `sed 's/CAMPAIGN:Ultimate Psionics/CAMPAIGN:DSP - Ultimate Psionics/'`
(`scripts/oracle_harness/campaign_key.py`'s own fix) + the `ac-isolate.txt.ftl` absolute per-type
isolator (`a68fbeea3d`), against the pinned oracle
(`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, repo-local checkout):

```
ultimate_psionics:equipment:improved_mind_armor_heavy: rc=0 severe=False AC.ISOLATED=9
ultimate_psionics:equipment:improved_mind_armor_medium: rc=0 severe=False AC.ISOLATED=6
ultimate_psionics:equipment:goggles_of_far_sight: rc=0 severe=False AC.ISOLATED=-1
ultimate_psionics:equipment:leather_of_confined_spaces: rc=0 severe=False AC.ISOLATED=5
ultimate_psionics:equipment:mind_armor_heavy: rc=0 severe=False AC.ISOLATED=7
ultimate_psionics:equipment:improved_mind_armor_light: rc=0 severe=False AC.ISOLATED=4
ultimate_psionics:equipment:mind_armor_light: rc=0 severe=False AC.ISOLATED=4
ultimate_psionics:equipment:mind_armor_medium: rc=0 severe=False AC.ISOLATED=6
ultimate_psionics:equipment:mind_shield: rc=0 severe=False AC.ISOLATED=2
ultimate_psionics:equipment:mind_shield_heavy: rc=0 severe=False AC.ISOLATED=2
ultimate_psionics:equipment:mind_shield_tower: rc=0 severe=False AC.ISOLATED=4
ultimate_psionics:equipment:plate_of_the_juggernaut: rc=0 severe=False AC.ISOLATED=11
ultimate_psionics:equipment:shadow_shirt: rc=0 severe=False AC.ISOLATED=6
ultimate_psionics:equipment:skinwalker_s_leather: rc=0 severe=False AC.ISOLATED=5
```
(`severe` here means "'SEVERE' or 'Could not find campaign' present anywhere in stdout+stderr", not
just the unrelated `IsOrc` FACT-parsing LSTERROR every run in this pinned oracle emits regardless of
campaign — independently confirmed clean per-unit: 0 of 14 stdout+stderr tails contain "Could not
find campaign" or "Could not add equipment".) **Independently reproduced live** this cycle for one
unit outside the batch script, same result: `mind_shield` → `AC.ISOLATED=2` (matches both the batch
run and the engine's `ours=2`).

**14 of 14 agree.** 4 of the 14 (`plate_of_the_juggernaut`, `shadow_shirt`, `skinwalker_s_leather`,
`leather_of_confined_spaces`) carry a STALE `ours` in the combined file (9/4/3/2) that differs from
the CURRENT engine's own output (`combat-shape-work-wave5/e5_ac_isolator.output.json`: 11/6/5/5) —
each of these 4 has an `EQMOD`-referenced `ArmorEnhancement` chain, the same
never-re-run-after-`abc72f75ec` staleness class `AT-33-E6-001-attempt6`'s own Shortfall-2 audit found
on `full_plate_of_the_corpse`/`goblin_plate`. Corrected here to the current engine value — using the
stale `ours` would have manufactured a false `disagree` against the newly-live oracle value.

### 3. Identity-resolve fix (`9df1c0b514`, `corpus_loader.rs` KEY synthesis) — corpus-wide population derived, examined-population intersected, 5 stale rows re-run

**Corpus-wide population, derived by execution** (every `equipment`/`equipment_modifier` corpus
record whose identity resolution outcome the fix changes — no literal `KEY:` token in `raw_tokens`
AND `data.name != data.key`):
```
$ python3 -c "
import json, glob
affected = []
for path in glob.glob('data/corpus/*/equipment*/**/*.json', recursive=True) + glob.glob('data/corpus/*/equipment*.json'):
    d = json.load(open(path)); data = d.get('data')
    if not isinstance(data, dict): continue
    tokens = data.get('raw_tokens') or []
    has_key = any(t.get('key')=='KEY' for t in tokens)
    if not has_key and data.get('name') != data.get('key') and data.get('key'):
        affected.append(path)
print(len(affected))"
436
```
**Denominator: 436 of 7,807 equipment/equipment_modifier corpus records.** Intersected with the
already-EXAMINED (rowed) population in the combined file: **209 of 436** carry a row. Of those 209,
the large majority (`no_bonus_chain: raw_bonus_chains is empty`) never invoke
`equipment_id_resolve` at all (that classification short-circuits directly on the corpus JSON, before
any resolve call) and are structurally unaffected. Filtering to rows whose reason names an
identity-resolve failure specifically:
```
$ grep -c "equipment_id_resolve_no_match_keyless_outputname_record\|engine_id_resolve_fails_templated_variant_record" \
    <(python3 -c "import json; [print(r.get('reason','')) for r in json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json'))['results']]")
5
```
**5 of 5 rows carrying a pre-fix identity-resolve-failure reason, all re-run.** (The remaining
overlap — `companion_stone_diplomacy`, 4 `crystal_mask_*`, 2 `meld_stone_*`, `ring_self_sufficiency`,
3 `psychoactive_skin_{chameleon,nimbleness,spider}` — is `sd33-r5-skillcombat`'s own already-current
14-row `agree` population from `9df1c0b514`, post-fix; no staleness, not re-run again.)

**None of these 5 had ever reached a live oracle invocation before** (every prior lane recorded
`ours=None, oracle=None`, dropped before any PCGen call since `equipment_id_resolve` itself failed).
Confirmed engine-side resolution first (`e6_identity_rerun_ours`, no new resolver logic — calls the
SAME `equipment_id_resolve`/`compute_var_effect`/`compute_equipment_effects` functions):
```
$ cargo run --locked --bin e6_identity_rerun_ours -- . <out.json>
e6_identity_rerun_ours: 5 items -> <out.json>
{"unit_id":"advanced_players_guide:equipment:backpack_masterwork","equipment_id_resolve_now_succeeds":true,"ours":1,"var_name":"LOADSCORE",...}
{"unit_id":"ultimate_psionics:equipment:companion_stone_electrical_protection","equipment_id_resolve_now_succeeds":true,"ours":10,"var_name":"ElectricityResistanceBonus",...}
{"unit_id":"ultimate_psionics:equipment:psychoactive_skin_psion","equipment_id_resolve_now_succeeds":true,"ours":7,"var_name":"BonusPowerPoints",...}
{"unit_id":"ultimate_psionics:equipment:psychoactive_skin_defender","equipment_id_resolve_now_succeeds":true,"ours_ac":4,...}
{"unit_id":"ultimate_psionics:equipment:psychoactive_skin_hero","equipment_id_resolve_now_succeeds":true,"ours_ac":3,"ours_tohit":null,...}
```
**5 of 5 now resolve** (was 0 of 5). Then built new single-item `.pcg` fixtures (never previously
attempted for these 5) and ran them live against the pinned oracle:

- `backpack_masterwork` (VAR, `LOADSCORE`): matching-context-baseline diff (bare L1 Human Fighter,
  same convention `AT-33-E5-shape-var`'s own 44-unit population uses) — item `VAR.LOADSCORE=17.0`,
  baseline `16.0`, diff `+1`, matching `BONUS:VAR|LOADSCORE|1|TYPE=Masterwork` exactly. **agree.**
- `companion_stone_electrical_protection` (VAR, `ElectricityResistanceBonus`): absolute (baseline
  confirmed `0.0`, no confound) — item `10.0`, matching `BONUS:VAR|ElectricityResistanceBonus|10`
  exactly. **agree.**
- `psychoactive_skin_psion` (VAR, `BonusPowerPoints`): PCGen exports `0.0` both with and without the
  item equipped on a bare Fighter — the SAME `var_gated_by_unbuilt_class_feature_zero_on_generic_
  baseline` shape `AT-33-E5-shape-var` already established for 60 other units (a psionics
  power-point-pool variable a non-psionic baseline never populates; real PF1 rule, not a defect).
  ours=7 (real, computed, unconditional token). **unverifiable, reason
  `var_gated_by_unbuilt_class_feature_zero_on_generic_baseline`** — not fabricated as agree/disagree.
- `psychoactive_skin_defender` (single `COMBAT|AC|4|TYPE=NaturalArmor` chain): `ac-isolate.txt.ftl`,
  no baseline needed — `AC.ISOLATED=4`, matching `armor_class_bonus=4` exactly. **agree.**
- `psychoactive_skin_hero` (THREE chains: `AC|3|TYPE=Deflection`, `TOHIT|3|TYPE=Enhancement`,
  `SAVE|...|3|TYPE=Resistance`): AC dimension isolator gives `AC.ISOLATED=3`, matching `ours_ac=3`
  exactly (**agree** on that dimension) — but `TOHIT`/`SAVE` have no live resolver in
  `equipment_effects/*.rs` (bare `COMBAT|TOHIT` outside a WEAPON context; no `SAVE` resolver at all,
  matching `crystal_mask_mindarmor`'s own `no_probe_surface(shape: SAVE)` finding elsewhere in this
  population). Per `AT-33-E5-finalize-wave3`'s multi-shape merge rule (worst of the per-dimension
  verdicts, never a fabricated whole-record `agree`), merged verdict: **unverifiable, reason
  `multi_shape_partial_resolver_gap`**, `multi_shape_sources` recording all 3 dimensions.

## Coverage — the re-run actually covered the full derived set, not a subset

| Correction | Affected-set size, denominator | Rows re-run | Coverage |
|---|---:|---:|---|
| AC isolator (`a68fbeea3d`) | 66 of its own 82-item manifest (already re-run by wave 5) | 66 (re-affirmed) + 2 merge-propagated | **66 of 66**, 2 of 2 propagation-gap rows closed |
| Campaign-key (`9df1c0b514`) | 14 of 14 rows carrying `oracle_harness_ultimate_psionics_campaign_load_failure` | 14 | **14 of 14** |
| Identity-resolve (`9df1c0b514`) | 5 of 5 examined rows carrying a pre-fix identity-resolve-failure reason (of 209 examined rows intersecting the corpus-wide 436-record affected population — the other 204 are either `no_bonus_chain` short-circuits, unaffected, or already-current post-fix rows) | 5 | **5 of 5** |
| **Total this cycle** | **21 of 21** | **21** | **21 of 21 — full coverage, no subset** |

## What moved

- **Rows re-run:** 21 of 21 in the derived affected set (2 Shortfall-3 propagation + 14 campaign-key
  + 5 identity-resolve).
- **Rows whose `(ours, oracle)` value changed:** 20 of 21 (all except `psychoactive_skin_hero`, whose
  merged row stays `(None, None)` — only its verdict/reason text changed).
- **Rows whose verdict changed:** 17 of 21 — all 17 moved `unverifiable` → `agree` (14 campaign-key +
  `backpack_masterwork` + `companion_stone_electrical_protection` + `psychoactive_skin_defender`).
  The other 4 (2 Shortfall-3 rows, already `agree`; `psychoactive_skin_psion` and
  `psychoactive_skin_hero`, both correctly staying `unverifiable` for an honestly different, now-
  accurate reason) did not change verdict.
- **New disagreements surfaced: 0.** Re-derived live after the merge:
  ```
  $ python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json
  uncovered=0 overlap=0 population=49438 oracle_disagreement=1 unverifiable_done=0 stale=False
  ORACLE_DISAGREEMENT: advanced_race_guide:equipment:rending_claw_blades
  EXIT=1
  ```
  `oracle_disagreement` is unchanged at 1 (the pre-existing, already-escalated
  `rending_claw_blades`) — this cycle's re-run genuinely surfaced no new wrong answer, but it DID
  surface and correct **4 stale `ours` values** (the `ArmorEnhancement`-EQMOD-referenced campaign-key
  units above) that would have produced a false `disagree` against the newly-obtained real oracle
  values had they been left uncorrected — the exact failure mode this criterion's re-run obligation
  exists to catch, even though in this instance both sides were corrected together and no bucket
  count changed.
- **Buckets, before → after** (same combined file, same 8,291-row denominator, re-derived):
  `agree` 786 → **803** (+17). `unverifiable` 7,504 → **7,487** (−17). `disagree` 1 → **1**
  (unchanged). Row count 8,291 → **8,291** (unchanged — no row added or dropped, confirmed
  `len(before)==len(after)` in the merge script's own assertion).

## `method_change_rerun_verified`

**Now `true`** for all three wave-5 corrections carrying the re-run obligation: the AC isolator's own
66-unit re-run (already complete, propagation gap closed this cycle), the campaign-key fix (14 of 14
re-run live), and the identity-resolve fix (5 of 5 examined-and-affected rows re-run live, corpus-wide
436-record population derived and stated). No affected row remains un-re-run.

## Test scoping

`cargo test --locked --lib corpus_loader::` → 6/6 (unchanged, no `src/rules_core/` file was modified
this cycle — `e6_identity_rerun_ours.rs` is a new `src/bin/` probe, not a library change). Did not run
the full `cargo test --locked --lib` sweep — the 4 pre-existing failures named in
`AT-33-E6-001-attempt6` (`equipment_resolver`, `formula_interpreter_corpus_wide` ×3) are unrelated to
this cycle's files (no `equipment_resolver.rs`, no `pilot_compute/*` touched) and are a separate
lane's/attempt's scope.

## Figures + their re-derive commands

- 21 of 21 derived-affected rows re-run — `method-rerun-wave6.oracle-results.json`'s own row count,
  `python3 -c "import json; print(len(json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/method-rerun-wave6.oracle-results.json'))['results']))"` → `21`.
- 17 of 21 verdict-changed, 20 of 21 value-changed — `merge_into_combined.py`'s own assertion output
  (printed above, re-derivable by re-running the same merge logic against a fresh copy of the
  pre-merge combined file — the pre-merge file is not separately retained, but every changed row
  carries its `prior_combined_value`/old-reason in `method-rerun-wave6.oracle-results.json` for
  audit).
- `oracle_disagreement=1` (unchanged) —
  `python3 scripts/box_ledger.py --check --oracle-results docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json`.
- `agree=803, unverifiable=7487, disagree=1` of 8,291 rowed units —
  `python3 -c "import json,collections; print(collections.Counter(r['verdict'] for r in json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json'))['results']))"`.
- 436 of 7,807 corpus-wide identity-resolve-affected records; 209 of 436 examined; 5 of 209 stale —
  commands inline above, in the blast-radius derivation section.

- **Status:** complete
- **Movement, four buckets:** closure 0 (no `docs/work-inventory.json` `status` field changed).
  Reclassification 0 (no unit moved kind/population). Reachability 0 (no unit newly rowed — all 21
  were already-examined units this cycle re-ran, per its own mandate; row 17's unrowed-39 population
  is a disjoint, sibling-lane scope this cycle does not touch). Instrument-correction 21 (all 21 rows'
  method/value/reason corrected to reflect the current, fixed harness/engine — 17 of which flipped
  verdict from `unverifiable` to `agree` as a direct, honest consequence).
- **Notes:** The single most valuable possible finding this task named — a former `agree` row now
  `disagree` — did not occur. This is reported as a real, checked negative result, not assumed: every
  one of the 21 affected rows was individually re-derived live, cross-checked against the current
  engine (`e5_ac_isolator.output.json`, `e6_identity_rerun_ours`) and, for the 4
  previously-stale-`ours` campaign-key rows, corrected on BOTH sides at once rather than compared
  naively (which would have manufactured a false `disagree`, the opposite failure mode this cycle
  exists to catch).
- **Next-cycle plan:** none owned by this lane — row 17's 39-unit residual and row 18's
  `rending_claw_blades` escalation are disjoint, sibling-lane scope (three concurrent lanes named in
  this dispatch's own coordination note). This cycle's own mandate (re-run everything the wave-5
  method changes already judged) is fully discharged: 21 of 21.
