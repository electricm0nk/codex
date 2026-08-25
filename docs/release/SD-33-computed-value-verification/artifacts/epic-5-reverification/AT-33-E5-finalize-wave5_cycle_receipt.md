# Cycle AT-33-E5-finalize-wave5 — Epic 5 Re-verification / AT-33-E5-001, 002, 003 (totals + kanban call)

- **Commit SHA:** recorded on landing (see `progress.md` entry `AT-33-E5-finalize-wave5`)
- **Files touched:**
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/finalize-wave5-merge.py` (new — the merge script, run for real, output in Step 3)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/literal-verified.oracle-results.json` (merged in place, 6,522 → 6,550 rows)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json` (merged in place, 8,263 → 8,291 rows)
  - `src/rules_core/equipment_effects/equipmods.rs` (real fix — `WeaponEnhancementBonus` split `affects`/`bonus` into `tohit_bonus`/`damage_bonus`, `compute_equipmods_effect` sums every qualifying chain instead of the first)
  - `src/rules_core/equipment_effects.rs`, `src/rules_core/damage_total.rs` (consumers updated for the new fields)
  - `src/bin/e5_last67_weapon_ours.rs` (weapon lane's own probe binary, JSON output updated for the new fields)
  - `docs/release/SD-33-computed-value-verification/progress.md` (Disagreement ledger addition, front-matter status, Cards-complete correction, `## Open blockers` entry, `## Cycles` entry prepended)
  - `docs/release/SD-33-computed-value-verification/kanban.md` (rows 16, 17, 18 updated in place)
  - `docs/retro/events/sd33-r5-e5-finalize.jsonl` (new — 2 `correction`, 1 `deferral`)
- **Identifier audit result:**
```
$ BASE_BRANCH=$(git merge-base HEAD origin/develop)
$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/rules_core/equipment_effects/equipmods.rs src/rules_core/equipment_effects.rs src/rules_core/damage_total.rs src/bin/e5_last67_weapon_ours.rs docs/release/SD-33-computed-value-verification/ ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
OK_NO_BUNDLE_TAGS
```
- **Wired-integration audit result:**
```
$ git diff --unified=0 "${BASE_BRANCH}...HEAD" -- src/rules_core/equipment_effects/equipmods.rs src/rules_core/equipment_effects.rs src/rules_core/damage_total.rs src/bin/e5_last67_weapon_ours.rs docs/release/SD-33-computed-value-verification/ ':!**/__tests__/**' ':!**/*.test.*' \
    | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
OK_NO_TOKENS
```
(the only near-matches are this receipt's own quoting of the audit command and prose describing "no resolver" — real prose, not a token in shipped code.)
- **Acceptance criteria (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E5-001 — the 1,741 `fixture-verified` units are re-examined against the oracle
  > **Evidence:** per-unit `(ours, oracle, verdict)` rows committed; agreement and disagreement counts both stated, with the denominator.
  >
  > ### AT-33-E5-002 — the 6,589 `literal-verified` units are re-examined
  > **Evidence:** as above.
  >
  > ### AT-33-E5-003 — every disagreement is a named defect, fixed or escalated
  > A disagreement is **never** closed by adjusting the expectation to match our output. Each is root-caused: either our computation is wrong (fix it) or the oracle comparison is wrong (fix the harness, and re-run everything it already judged).
  > **Evidence:** one entry per disagreement in `progress.md`, each resolved to a commit or an operator escalation. **A filed blocker does not satisfy this criterion.**

## Step 1 — environment, rebase, base check

```
$ export RETRO_ACTOR="sd33-r5-e5-finalize"
$ export CARGO_TARGET_DIR="/tmp/cargo-sd33-sd33-r5-e5-finalize"
$ export CARGO_INCREMENTAL=0
$ mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"
```
`git status --porcelain` at turn start showed one dirty tracked file, `docs/retro/events/codex.jsonl`
(a single appended line from the background `reclaim.sh` incident-reporting daemon, the same shape
prior cycles this wave found and committed separately before rebasing) plus the pre-existing,
out-of-scope `sd-33-*.workflow.js` untracked files. Committed the stray retro line alone
(`3817fa0e65`), then `git fetch origin tranche/13 && git rebase origin/tranche/13` — clean rebase, no
conflicts. `test -d docs && test -d data && test -d scripts` — real base confirmed.

## Step 2 — read the wave-5 lane reports and their committed artifacts

Read (in order): `workflow-instruction.md` (dispatch procedure), `epic-breakdown.md`'s
`AT-33-E5-002`/`003` criteria verbatim, `AT-33-E5-last75_cycle_receipt.md` (wave 4's own shape-table
map). Located the four wave-5 lanes' committed deliverables (all already on `origin/tranche/13`
after the rebase): `last67-weapon.oracle-results.json` (14 rows), `last67-skill-combat.oracle-results.json`
(14 rows), `last67-eqm.oracle-results.json` (0 rows), `full-rerun-wave5.oracle-results.json` (66 rows)
+ `disagreement-fixes-wave5.oracle-results.json` (4 rows, confirmed a byte-identical subset of the
66 — not merged separately).

## Step 3 — the merge, and a real hazard caught before landing

A first-draft merge script blindly replaced the WHOLE `literal-verified.oracle-results.json` row for
each of `full-rerun-wave5`'s 66 unit_ids with that lane's single-dimension value. Before committing,
diffed the naive output against `git show HEAD:.../literal-verified.oracle-results.json` per-record
and found: **11 of the 66 are `multi_shape_sources` records** (a single equipment item independently
examined by two different shape lanes for two different bonus-chain dimensions — the convention
`AT-33-E5-finalize-wave3` established). The naive replace would have (a) silently discarded the
OTHER, unrelated, already-verified dimension for all 11, and (b) fabricated apparent top-level
changes for 9 of them that never actually happened (the AC-isolator's re-derived value was
byte-identical to what those 9 already had — the "change" was an artifact of re-running an assumed
tie-break rule that didn't match whatever process actually produced the current file, not a real
data movement). Logged as a `correction` event, self-caught before merge
(`docs/retro/events/sd33-r5-e5-finalize.jsonl`).

**Corrected, algorithm-agnostic merge rule applied:** since every one of the 66 `full-rerun-wave5`
rows is verdict `agree` (no verdict-rank ever changes), the WINNING lane for a multi-shape record
cannot flip. So: only the `combat-weapon-shape` sub-entry inside `multi_shape_sources` is ever
touched by this data; the top-level `ours`/`oracle` is updated **only when it already equalled the
OLD `combat-weapon-shape` sub-entry** (i.e. that lane was already the displayed winner) — this holds
regardless of what tie-break rule originally produced the file, rather than re-guessing one.

```
$ python3 artifacts/epic-5-reverification/finalize-wave5-merge.py   # committed alongside this receipt
full-rerun-wave5: 66 rows examined (11 multi-shape, 61 no-op, 4 simple rows replaced, 5 rows with a genuine change)
  advanced_class_guide:equipment:full_plate_of_the_corpse  top-level  9/10/disagree -> 11/11/agree
  inner_sea_races:equipment:goblin_plate                   combat-sub-entry only  9/9/agree -> 10/10/agree (top-level unaffected, stays 6/6)
  inner_sea_world_guide:equipment:field_plate               top-level  7/6/disagree -> 7/7/agree
  inner_sea_world_guide:equipment:stoneplate                 top-level  9/8/disagree -> 9/9/agree
  ultimate_equipment:equipment:snakeskin_tunic                top-level  1/2/disagree -> 1/1/agree
```
Exactly **5 of 66 genuinely moved** — matching `sd33-r5-disagreements`'s own claim in its receipt
("5 of 66 oracle values moved") precisely, confirming the corrected merge (not the naive one) is the
one that reproduces their own independently-derived figure.

Then added the 14 `skill-combat` rows and 14 `weapon` rows (all confirmed 0 overlap with
`literal-verified`/`fixture-verified` before adding), with `ultimate_equipment:equipment:heavy_hammer`
corrected in place (see Step 4).

## Step 4 — a real engine fix: `compute_equipmods_effect` multi-chain summing

The `weapon-token-family` lane's own examination of `heavy_hammer` found `ours=0` against
`oracle=4` (DAMAGE) — a genuine defect, not a harness artifact: `heavy_hammer`'s corpus record
carries **two** separately-scoped qualifying chains, `BONUS:WEAPONPROF=Warhammer|TOHIT|-2` and
`BONUS:WEAPONPROF=Warhammer|DAMAGE|4`. `compute_equipmods_effect` used `.find_map(...)` — stops at
the FIRST qualifying chain — so the second, `DAMAGE|4`, was silently dropped; a real player-facing
gap (the item's own damage bonus would never render).

**Blast-radius check before touching anything:**
```
$ python3 -c "import json, glob
def is_roll_shape(q1): return q1 in ('TOHIT','DAMAGE','DAMAGE,TOHIT','TOHIT,DAMAGE')
multi=[]; total=0
for f in glob.glob('data/corpus/**/equipment*/*.json', recursive=True):
    d=json.load(open(f)); chains=d.get('data',{}).get('raw_bonus_chains',[])
    if not chains: continue
    total+=1
    matches=[]
    for c in chains:
        q=c.get('qualifiers',[])
        if len(q)<2: continue
        subject=q[0]
        if (subject=='WEAPON' or subject=='WEAPONPROF=TYPE.Natural') and is_roll_shape(q[1]):
            if len(q)>=4 and q[3]=='TYPE=Enhancement': matches.append(c)
        elif subject.startswith('WEAPONPROF=') and not subject[len('WEAPONPROF='):].startswith('TYPE.') and is_roll_shape(q[1]) and len(q)>=3:
            matches.append(c)
    if len(matches)>1: multi.append((f,matches))
print('records scanned', total); print('records with 2+ qualifying chains', len(multi))
for f,m in multi: print(f)"
records scanned 579
records with 2+ qualifying chains 1
data/corpus/ultimate_equipment/equipment/heavy_hammer.json
```
`heavy_hammer` is the ONLY record in the pinned corpus with 2+ qualifying chains — the fix is a pure
widening, not a risk to any other examined unit's already-verified value.

**Fix (TDD):** `WeaponEnhancementBonus`'s single `affects: String`/`bonus: i16` pair replaced with
independent `tohit_bonus: Option<i16>`/`damage_bonus: Option<i16>` fields; `compute_equipmods_effect`
rewritten to iterate ALL `bonus_chains` (was `find_map`, first match only) and sum each roll's
magnitude across every qualifying chain. New test, RED before the struct/logic change (the whole
existing test suite fails to compile against the new struct shape until every call site is updated —
confirmed live, see below):

```
$ cargo build --locked --lib      # BEFORE consumers updated
error[E0609]: no field `affects` on type `&WeaponEnhancementBonus`  (damage_total.rs:389, x2)
error[E0609]: no field `bonus` on type `&WeaponEnhancementBonus`    (damage_total.rs:390/393, equipment_effects.rs:429)
error[E0609]: no field `affects` on type `WeaponEnhancementBonus`   (equipment_effects.rs:428)
```
This IS the RED — the exact 6 call sites the new struct shape breaks, confirming the blast radius is
`damage_total::resolve_weapon_enhancement_modifier` and `equipment_effects::resolve_weapon_to_hit_bonus`
(both fixed, see Files touched) and nothing else in `src/`.

GREEN, after fixing both consumers and all 6 existing test assertions plus the new
`record_with_two_separately_scoped_chains_sums_both_rolls_independently` test (real verbatim
`heavy_hammer` tokens, including the unrelated `MOVEADD` chain to prove it's still correctly
skipped):

```
$ cargo build --locked --lib --bins   # clean, 0 errors
$ cargo test --locked --lib equipmods::           -> 16 passed; 0 failed
$ cargo test --locked --lib equipment_effects::   -> 71 passed; 0 failed
$ cargo test --locked --lib damage_total::        -> 27 passed; 0 failed
```

Live confirmation via the weapon lane's own probe binary (`e5_last67_weapon_ours`, updated for the
new field names):
```
$ cargo build --locked --bin e5_last67_weapon_ours
$ ./e5_last67_weapon_ours . /tmp/probe-manifest.json /tmp/probe-output.json
{
  "ultimate_equipment:equipment:heavy_hammer": {"damage_bonus": 4, "natural_attack_only": false, "tohit_bonus": -2, "weapon_prof_scope": "Warhammer"}
}
```
`tohit_bonus=-2` (unchanged, already agreed) / `damage_bonus=4` (was silently dropped, now matches
`oracle=4` exactly). `heavy_hammer` moved from `disagree` to `agree` in the merged
`literal-verified.oracle-results.json`.

## Step 5 — the one remaining disagreement: `rending_claw_blades`, root-caused, escalated

The weapon lane's own examination found `rending_claw_blades`: `ours=0`, `oracle=1` (DAMAGE).
Root-caused: the pinned PCGen source defines this record via a `.MOD`-attached line
(`advanced_race_guide/arg_equip_arms_armor.lst`) referencing two additional `Special Ability`
EQMODs (`+1 ~ Weapon`, `Keen ~ Weapon`) that this repo's corpus extraction pipeline never captured
into `data/corpus/advanced_race_guide/equipment/rending_claw_blades.json`'s `raw_tokens`. Confirmed
`compute_equipmods_effect` already correctly reads the ONE chain the record's JSON does carry
(`BONUS:WEAPON|TOHIT|1|TYPE=Enhancement` → `tohit_bonus=1`, matching `oracle`'s `MAGICHIT=+1`
exactly) — **no `src/rules_core/` change can fix this**; the defect is upstream, in corpus
extraction, and the fix (`data/corpus/**` is guarded-generator-only, `--allow-stamp-loss` forbidden)
is a bundle-wide-blast-radius change (the `.MOD`-merge logic is generic, likely shared across every
book) that needs its own dedicated audit before touching anything. Filed under `progress.md`'s
`## Open blockers` with the exact operator ask (audit the `.MOD`-attached-EQMOD-merge blast radius,
then regenerate via the guarded generator) and a `scripts/retro.py deferral` event.

## Step 6 — merged totals, re-derived live

```
$ python3 -c "import json,collections
for name in ['fixture-verified.combined-oracle-results.json','literal-verified.oracle-results.json','AT-33-E5-003.combined-oracle-results.json']:
    d=json.load(open('artifacts/epic-5-reverification/'+name))['results']
    ids=[r['unit_id'] for r in d]
    print(name, len(d), 'dupes', len(ids)-len(set(ids)), collections.Counter(r['verdict'] for r in d))"
fixture-verified.combined-oracle-results.json 1741 dupes 0 Counter({'unverifiable': 1345, 'agree': 396})
literal-verified.oracle-results.json 6550 dupes 0 Counter({'unverifiable': 6159, 'agree': 390, 'disagree': 1})
AT-33-E5-003.combined-oracle-results.json 8291 dupes 0 Counter({'unverifiable': 7504, 'agree': 786, 'disagree': 1})
```

**Unexamined set, re-derived (never inferred from a count):**
```
$ python3 -c "import json
wi=json.load(open('docs/work-inventory.json'))['units']
pop={u['id'] for u in wi if u.get('status') in ('literal-verified','fixture-verified')}
d=json.load(open('artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json'))['results']
miss=sorted(pop-{r['unit_id'] for r in d}); print(len(pop)); print(len(miss)); [print(m) for m in miss]"
8330
39
advanced_class_guide:equipment:brawler_s_flurry
... (39 total, printed in full in this cycle's terminal output; every one of the 39 is named by
    one of the three sibling wave-5 lanes' own shape tables: 23 weapon-token-family,
    9 skill-combat-token-family, 7 eqm-modifier-family — 23+9+7=39)
```

**Zero reasonless `unverifiable`, zero duplicate `unit_id`, across all three files:**
```
$ python3 -c "import json
for name in ['fixture-verified.combined-oracle-results.json','literal-verified.oracle-results.json','AT-33-E5-003.combined-oracle-results.json']:
    p='artifacts/epic-5-reverification/'+name
    d=json.load(open(p))['results']
    reasonless=[r['unit_id'] for r in d if r.get('verdict')=='unverifiable' and not (r.get('reason') or '').strip()]
    ids=[r['unit_id'] for r in d]
    print(name, 'rows', len(d), 'reasonless_unverifiable', len(reasonless), 'dupes', len(ids)-len(set(ids)))"
fixture-verified.combined-oracle-results.json rows 1741 reasonless_unverifiable 0 dupes 0
literal-verified.oracle-results.json rows 6550 reasonless_unverifiable 0 dupes 0
AT-33-E5-003.combined-oracle-results.json rows 8291 reasonless_unverifiable 0 dupes 0
```

**`box_ledger.py --check`, on the actual post-merge file:**
```
$ python3 scripts/box_ledger.py --check --oracle-results artifacts/epic-5-reverification/AT-33-E5-003.combined-oracle-results.json
uncovered=0 overlap=0 population=49438 oracle_disagreement=1 unverifiable_done=0 stale=False
ORACLE_DISAGREEMENT: advanced_race_guide:equipment:rending_claw_blades
$ echo $?
1
```

**`disagree` capability re-proven on the CURRENT batch path:** the run directly above IS the
re-proof — a real, genuinely-examined `disagree` row (`rending_claw_blades`, not suppressed, not
new-since-this-derivation) flows through `box_ledger.py --check` on the actual merged file and
correctly returns `disagree` at exit 1. No synthetic probe was needed or used, since a genuine
disagreeing case already exists in this batch — a stronger proof than a mutated fixture, since it
demonstrates the capability against real, load-bearing data rather than an injected one.

## Step 7 — denominator gate

```
$ bash scripts/verify.sh --only denominator-gate
    PASS  denominator-gate  (files_checked=45 violations=0)
RESULT: PASS
```
Re-ran after all of this receipt's + `progress.md`'s + `kanban.md`'s prose edits landed — still PASS.

## Figures + their re-derive commands

| Figure | Value | Command |
|---|---:|---|
| `fixture-verified` rows / population | 1,741 / 1,741 | `python3 -c "import json; print(len(json.load(open('artifacts/epic-5-reverification/fixture-verified.combined-oracle-results.json'))['results']))"` |
| `fixture-verified` disagree | 0 | (verdict Counter above) |
| `literal-verified` rows / population | 6,550 / 6,589 | same pattern, `literal-verified.oracle-results.json` |
| `literal-verified` disagree | 1 | (verdict Counter above) |
| combined rows / population | 8,291 / 8,330 | `AT-33-E5-003.combined-oracle-results.json` |
| combined disagree | 1 | `box_ledger.py --check` → `oracle_disagreement=1` |
| unexamined (39) | 39 | Step 6's Python snippet, `pop - {ids}` |
| reasonless `unverifiable` | 0 (all 3 files) | Step 6's Python snippet |
| duplicate `unit_id` | 0 (all 3 files) | Step 6's Python snippet |
| corpus records with 2+ qualifying `compute_equipmods_effect` chains | 1 (`heavy_hammer`) | Step 4's Python snippet |
| disagreements ever surfaced (waves 3-5) | 28 | 22 (wave 4 fixed) + 4 (wave 4→5 fixed) + 2 (this wave, weapon lane: 1 fixed, 1 escalated) |

## Status: blocked-escalated

**Not `complete`.** Row 16 (`AT-33-E5-001`) is genuinely `complete` — 1,741 of 1,741, 0 disagree,
confirmed unaffected by every wave-5 lane. Row 17 (`AT-33-E5-002`) is short by **39 of 6,589** — a
real, named gap (23 weapon-shape + 9 skill-combat-shape + 7 eqm-shape), not a false 100%. Row 18
(`AT-33-E5-003`) carries **1 unresolved `disagree`** (`rending_claw_blades`) — root-caused and
genuinely escalated with a named operator ask, but per this cycle's own conservative reading of
`decisions.md`'s "no carve-outs, close, do not flag" standing lesson, an escalation is recorded but
does **not** get treated as equivalent to closure for the kanban row while a real disagreement
remains open in the file.

## Movement, four buckets

- **Closure:** 0 — no `docs/work-inventory.json` `status` field changed.
- **Reclassification:** 0.
- **Reachability:** 28 — the weapon lane's 14 new rows + the skill-combat lane's 14 new rows, both
  genuinely new to the examined population.
- **Instrument-correction:** 67 — the AC-isolator lane's full 66-unit re-run (5 values genuinely
  moved, 61 confirmed unchanged) plus the `heavy_hammer` engine fix (1 unit, counted once, not
  double-counted against the 66 since it is a disjoint weapon-lane unit).

## Notes

The multi-shape merge hazard (Step 3) is the most valuable finding of this cycle: a naive
whole-row-replace merge would have passed every one of this cycle's OWN sanity checks (row counts
match, no duplicate `unit_id`s, denominator gate green) while silently corrupting 9 records'
already-correct data and discarding 11 records' second dimension. It was caught only by directly
diffing against the pre-merge committed file before writing anything — the same discipline
`AGENTS.md` rule 9 and `decisions.md §2` both require ("every figure carries its re-derive command",
"reports are not evidence").

## Next-cycle plan

1. `AT-33-E5-002`'s 39 remaining unrowed units — owned by the three sibling wave-5 lanes' own
   next-cycle plans (`AT-33-E5-last67-{weapon,skill-combat,eqm}_cycle_receipt.md`).
2. The `rending_claw_blades` corpus-extraction gap — awaiting the operator ruling filed this cycle
   under `progress.md`'s `## Open blockers`.
3. Once both close, `AT-33-E6-001` can re-run the final-acceptance scan as its next attempt.
