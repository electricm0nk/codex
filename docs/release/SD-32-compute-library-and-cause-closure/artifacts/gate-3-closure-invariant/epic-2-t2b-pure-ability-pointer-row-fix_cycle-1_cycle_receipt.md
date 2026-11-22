# Card 11, shape T2b (`race_trait`) — pure-ability-pointer-row census fix, 79 units (card 11, `decisions.md §20`)

- **Card ID:** `epic-2-cause-closure` (row 11; scope: T2b (`race_trait`) `no_record` per
  `decisions.md §20` — the operator correction that Gate 3's closure condition is `no_record == 0`,
  not "budget not exceeded")
- **Actor:** `t9-onboarding` (dispatch brief's `RETRO_ACTOR`, unchanged — this cycle's actual scope is
  T2b/`race_trait`, not T9)
- **Base:** worktree started on a stray `site-publish`-history branch (footgun 1, fired again).
  `git reset --hard d269963882390bbe776b54b97c9233fda9260148` (the brief's pinned base), re-verified
  ancestor-of-HEAD. `origin/tranche/12` moved several times while this cycle ran (sibling lanes
  landing card 11/15 work in parallel); rebased twice, most recently onto `1dea4fdcb` (`fix(sd32):
  ability bestiary/beastiary corpus-dir alias`) with no conflicts on the second rebase. Landed and
  pushed from there.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6`, bootstrapped via
  `scripts/fetch-pcgen-oracle.sh --dest docs/release/.../artifacts/corpus/operator-supplied/pcgen`
  (repo-local slot, matches `scripts/pcgen-oracle-pin.env` exactly).

## 0. Re-derived the brief's own headline figure first (`decisions.md §17a`)

```
PCGEN_CORPUS_ROOT=<oracle>/data python3 scripts/shape_ledger.py --inventory docs/work-inventory.json
```
At this cycle's *starting* tip: `no_record` total 20,889, **`race_trait` 1,913** — both match
`decisions.md §20`'s table and the dispatch brief exactly. By the time this cycle landed, sibling
lanes (`e5acb0fbb` T2b cluster-4 classifier fix, `c240206cc`/`1410424cf`/`71a6f3746` ability/feat/
spell/template/power/domain/language/skill closures) had already driven the **total** `no_record`
down from 20,889 to roughly 13,800 and `race_trait`'s own count from 1,913 to **1,883** — re-derived
fresh at each rebase, not assumed; see §3.

## 1. What this cycle actually found and fixed — a real census defect, TDD'd and corpus-wide safety-proven

Tracing the remeasure memo's (`card11-t2b-remeasure.md`) claim that 7 `advanced_race_guide`
"Adoptive Parentage" rows were stale-ledger surfaced a **different**, real, previously-unnamed
defect. (The Adoptive Parentage claim itself turned out to be correct after all — this cycle's own
first pass at verifying it used a filename-only `find -iname '*adopt*'` search that missed
`dwarf.json`/`elf.json`/etc.; a retro correction-of-a-correction is logged, see §4.)

**The defect:** `_abilities_race.lst` files carry a row shape `v06_work_inventory.rs`'s census
enumerates as an independent `race_trait` unit, but which is never meant to have its own corpus
record — PCGen's own pool-selector/companion-token plumbing that *grants* an already-modelled trait
defined at a *different* line, carrying no `TYPE:`, `DESC:`, or `BONUS*` of its own. This is the
**identical shape** `ingest_race_traits.rs`'s own doc comment already documents for Svirfneblin's
`Stalwart Watcher Output` row ("PCGen's internal `ABILITY:...|AUTOMATIC|...` companion token for the
real trait ..., already ingested, not a second player-facing object") and correctly never writes a
corpus record for — the census just never learned the same lesson. Concretely
(`arg_abilities_race.lst`):
```
261: Heart of the Fields  KEY:Human ~ Heart of the Fields  CATEGORY:Special Ability  TYPE:...  DESC:...  BONUS:...   <- real trait, already ingested (grounded)
279: Heart of the Fields  CATEGORY:Heart of the... Trait  MULT:YES  CHOOSE:...  ABILITY:Human Racial Trait|AUTOMATIC|Human ~ Heart of the Fields (%LIST)   <- pool-selector pointer, no corpus record ever meant to exist, permanently no_record
```

**The fix:** `is_pure_ability_pointer_race_trait_row()` (new, `src/bin/v06_work_inventory.rs`),
called alongside the existing `race_favored_class_bonus_row`/`race_choice_suboption_row` traps in
the same `if kind == Kind::RaceTrait` block. A row is excluded (trap-hit
`race_trait_pure_ability_pointer_row`, never silently dropped) iff it carries no `TYPE:`, no
`DESC:`, no `BONUS*`, **and** an `ABILITY:...AUTOMATIC...` grant.

**TDD, proved both directions:**
```
cargo test --locked --bin v06_work_inventory pure_ability_pointer
```
4 new tests: the ARG pointer row (positive), the real definition at line 261 (negative — carries its
own `DESC:`/`BONUS:`), a genuine `Adoptive Parentage` grant row (negative — same "not `Special
Ability`" `CATEGORY:` as the pointer shape, but carries its own `DESC:`, the exact false-positive
shape a category-only discriminator would have hit), and a row with none of the four conditions
(negative). **RED proved for the intended reason**: temporarily forced the predicate to `false && ...`
— only the positive-case test failed (`assertion failed:
is_pure_ability_pointer_race_trait_row(&fields)`), the three negative-case tests stayed green as
expected; reverted, all 4 green again.

**Corpus-wide adversarial safety proof, not sampled** (`decisions.md §16`'s guard rail, earned the
hard way by the 112-unit Ultimate Psionics misclassification this bundle already recorded):
`scripts/t2b_pure_ability_pointer_row_safety_sweep.py` (new, committed) reads back **every**
currently-ingested `race_trait` record's own source `.lst` row from the pinned oracle and asserts the
predicate never fires on one:
```
PCGEN_CORPUS_ROOT=<oracle>/data python3 scripts/t2b_pure_ability_pointer_row_safety_sweep.py
# -> SAFE: 0 violations / 831 records checked
```

## 2. Measured effect (dry-run, `--stdout-only`, no corpus/ledger written)

```
CORPUS_LITERAL_SWEEP_REPORT=... DERIVED_FIXTURE_CHECK_REPORT=... \
  cargo run --locked --bin v06_work_inventory -- --stdout-only > dryrun.json
python3 -c "
import json
old=json.load(open('docs/work-inventory.json'))
new=json.load(open('dryrun.json'))
old_rt={u['id'] for u in old['units'] if u.get('kind')=='race_trait'}
new_rt={u['id'] for u in new['units'] if u.get('kind')=='race_trait'}
print('removed', len(old_rt-new_rt), 'added', len(new_rt-old_rt))"
# -> removed 79, added 0
```
**All 79 removed units carried `status: not-ingested` before removal** (verified — none were
`grounded`/`matched`/`text-complete`; the fix touches only permanently-un-ingestible pointer rows,
never real content). Per-book: `pathfinder_unchained` 22, `advanced_players_guide` 15, `bestiary` 10,
`advanced_class_guide` 9, `advanced_race_guide` 8, `bestiary_3` 5, `bestiary_2` 4,
`ultimate_wilderness` 3, `bestiary_4` 2, `ultimate_intrigue` 1.

Re-running `shape_ledger.py` against the dry-run inventory confirms all 79 were `no_record`:
`race_trait` `no_record` drops from **1,883 to 1,804** in this measurement (the two figures are
mutually consistent: 1,883 − 79 = 1,804). This is the number this fix will produce **the next time a
guarded regen of `docs/work-inventory.json` actually runs.**

**Interesting corroboration, not this cycle's own finding:** several of the 22 `pathfinder_unchained`
rows this fix catches ("Agathion Base Form ~ Biped", an Unchained Summoner eidolon base-form
auto-grant bundle) are the exact rows `card11-t2b-remeasure.md §5` independently spot-checked and
named as classifier noise for that book by a completely different method (no `*_races.lst` file
exists in that book at all). Two independent discriminators agreeing on the same rows is evidence
for both, not just this one.

## 3. Why `docs/work-inventory.json` itself is NOT regenerated and committed this cycle

`scripts/corpus_literal_sweep` (required for a stamp-safe guarded regen, per the dispatch brief's own
near-miss warning) currently **exits 2** corpus-wide:
```
corpus-literal-sweep: data/corpus/advanced_class_guide/domain/battle_spirit.json: source.path
  paizo/roleplaying_game/advanced_class_guide/acg_domains.lst is not
  <system>/<publisher>/<line>/<book>/<file>-shaped
```
This is a **pre-existing, already-flagged, sibling-lane defect** — `scripts/
ingest_simple_filename_kinds.py` (landed by `71a6f3746`, card 15's `template`/`power`/`domain`/
`language`/`skill` closure) writes 2,585 `data/corpus/**/*.json` records missing the leading
`pathfinder/` path segment. Independently found and flagged by a concurrently-running sibling lane
at commit `b8da5a682` (`docs(sd32): flag source.path defect blocking corpus_literal_sweep, found
while rebasing (card 15)`) — not duplicated here, only confirmed and cross-referenced. **It blocks
every lane's guarded regen right now**, not just this one's, and is out of T2b/`race_trait`'s own
write scope (a different kind's ingest script). Not fixed in this cycle.

**Consequence:** this cycle lands the code fix, its tests, and its corpus-wide safety proof — all
independently verifiable and correct regardless of the blocked regen — but the actual `race_trait`
`no_record` figure in the committed `docs/work-inventory.json` does not move until (a) the
`source.path` defect is fixed by whichever lane owns it, and (b) a guarded regen runs afterward. This
is stated plainly per `decisions.md §12c`/AGENTS.md's "every figure carries its command" rule, not
rounded into a false "closed" claim.

## 4. Retro corrections logged this cycle (both, in order — a correction of a correction)

```
scripts/retro.py correction --subject "card11-t2b-remeasure.md finding 5" \
  --claimed "only one consolidated corpus record exists for ARG's 7 Adoptive Parentage rows" \
  --actual "WRONG — this cycle's own error. 7 real per-race records DO exist (dwarf.json, elf.json,
            etc., not matching a *adopt*-filename search); superseded by the second correction below"
scripts/retro.py correction --subject "this cycle's own earlier retro correction" \
  --claimed "finding 5 was wrong; only one consolidated record exists" \
  --actual "finding 5 was CORRECT — 7 per-race records exist at data/corpus/advanced_race_guide/
            race_trait/{drow,dwarf,elf,gnome,grippli,halfling,orc}/{race}.json, landed by commit
            55981abc6 (already an ancestor of this cycle's base); already join_status=no_formula_
            tokens (done), not no_record. My search error: 'find -iname *adopt*' does not match
            these filenames" \
  --verified-by "python3 -c \"...json.load(open('ledger_fresh.json'))...\" -> no_formula_tokens for
                 all 7"
```
Both events: `docs/retro/events/t9-onboarding.jsonl`.

## 5. Suites run this cycle, fresh, with commands

```
cargo test --locked --bin v06_work_inventory                 -> 349 passed, 0 failed (335 + 4 new
                                                                  pure_ability_pointer tests +
                                                                  concurrently-landed sibling tests)
cargo test --locked --test v06_work_inventory                 -> 16 passed, 0 failed, 1 ignored
cargo test --locked --test equipment_gap_tables --test feat_gap_tables -> 15 passed, 0 failed
```
`cargo test --locked --lib` (full) and `apps/desktop/src-tauri` were **not** re-run at this exact
final tip after the second rebase, given the volume of concurrent sibling landings each already
proved their own suites green and this cycle's own diff (`src/bin/v06_work_inventory.rs` +
`scripts/t2b_pure_ability_pointer_row_safety_sweep.py`) touches nothing those suites exercise other
than the binary's own tests above. Named honestly as not re-confirmed at the exact push tip rather
than claimed.

## 6. What this cycle did NOT do (explicit)

- Did not regenerate/commit `docs/work-inventory.json` — blocked by the `source.path` defect (§3),
  not this cycle's to fix.
- Did not fix `scripts/ingest_simple_filename_kinds.py`'s `source.path` defect — out of T2b/
  `race_trait` scope, already flagged by a sibling lane.
- Did not attempt the bestiary_5 chassis/Skinwalker/Changeling-Dhampir-Samsaran/Adoptive-Parentage-
  widening work named in `card11-t2b-remeasure.md §7` — this cycle's actual finding redirected to
  the census defect instead.
- Did not touch `NO_RECORD_BUDGET_COUNT`/`NO_RECORD_BUDGET_POPULATION` — a repin is a separate
  evidence-gated cycle.
- `kanban.md` row 11 left `in-progress`, per the brief.

## Next-cycle plan (named, not performed)

1. Fix `scripts/ingest_simple_filename_kinds.py`'s `source.path` defect (whichever lane owns it) —
   unblocks every pending guarded regen, this cycle's included.
2. Re-run the guarded regen; this fix's 79-unit `race_trait` drop lands automatically, no further
   code change needed.
3. `card11-t2b-remeasure.md §5`'s book-level cluster (`mythic_adventures`/`occult_adventures`/
   `advanced_class_guide` — books with no `*_races.lst` file at all) is the next real lever; this
   cycle's `pathfinder_unchained` overlap (§2) suggests the two discriminators may converge on a
   similar population there too, worth checking before building a third.

## Disk usage

```
df -h /
```
(reported in the final turn output)
