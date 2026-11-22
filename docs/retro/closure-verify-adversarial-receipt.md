# closure-verify — adversarial verification of the 2026-08-13 closure run

Window verified: `fb4b043b..ece501f2` (14 commits). Verified from an isolated worktree at
`ece501f2`, `CARGO_TARGET_DIR=/home/ubuntu/workspace/codex-target-closure-verify`.

## Headline

**The run's true net is `done` +2 (class_feature 18 → 20; total 3,464 → 3,466).**
That is exactly the sum of the four cards' claims (+0, +0, +0, +2). No card overstated.

Re-derived by importing the **real** dashboard producer's `doneness_verdict()` — not a
transcription — and replaying it over the committed inventory at each end of the window:

```
python3 <scratch>/true_net.py fb4b043b ece501f2
# imports doneness_verdict / EXCLUDED_BOOKS from
#   ~/.hermes/profiles/god-emporer/skills/release-swarm-observer/scripts/pf1e_dashboard_producer.py
# reads git show <ref>:docs/work-inventory.json
```

| ref | units | done | held | in-progress | not-started | unmeasurable | deferred |
|---|---|---|---|---|---|---|---|
| `fb4b043b` | 38,521 | **3,464** | 9,455 | 716 | 21,303 | 3,547 | 36 |
| `ece501f2` | 38,521 | **3,466** | 9,457 | 716 | 21,300 | 3,546 | 36 |

`done` by kind, before → after (only one kind moved):

```
feat 1178→1178 · equipment_modifier 896→896 · companion 416→416 · monster_ability 334→334
equipment 277→277 · race_trait 264→264 · spell 47→47 · class 27→27 · monster 7→7
class_feature 18→20        <-- the entire movement
```

## No bar moved

- **Zero `wiring_class` reclassifications** across the window. Enumerated on the composite key
  `(book, kind, id, name, corpus_key, source_file, source_line, type_facet)`.
- `contract`, `status_vocabulary`, `trap_rules`, `magnitude_tokens`, `schema_version`,
  `units_omitted` are all **equal** between the two committed inventories.
- No `#[ignore]`, no `should_panic`, no removed test, no weakened assertion, no `--skip` anywhere
  in the window's `.rs`/`.sh`/`.py` diff. Only 16 deleted `.rs` lines in total, all of them the
  old class membership test being replaced by the consumer-delta probe.
- `scripts/verify.sh` and `scripts/verify-baselines.env` are **untouched** in this window.
- Exactly 31 units changed. 27 are `class` units whose *evidence string only* changed
  (`class_modelled_and_swept_through_the_real_compute_pipeline` →
  `class_probe_observed_computed_delta_on_the_rendered_snapshot`) with verdict `done → done`.
  That is a bar **raise** with zero count movement: the old string named a sweep the arm never
  read back; the new one is earned. 4 are the newly-grounded `class_feature` units.

## The +2 is real, and the +2 that was declined is the better result

The 4 newly-grounded `class_feature` units, and what each was worth:

```
advanced_players_guide:class_feature:discovery_feral_mutagen    computed  unknown      -> grounded -> DONE
advanced_class_guide:class_feature:bloodrager_bloodline_arcane  computed  not-ingested -> grounded -> DONE
core_rulebook:class_feature:rogue_talent_resiliency             static    not-ingested -> grounded -> held
advanced_class_guide:class_feature:slayer_talent_foil_scrutiny  static    not-ingested -> grounded -> held
```

Two of the four are `wiring_class: static`, which has no `done` rung. Reclassifying them to
`computed` — which the probe's own observation would have superficially justified, since a
consumer delta was in fact observed — would have made the headline +4. **The run did not do it,
and it was right not to.** That is the second time this week this program declined a number it
could have taken.

Probe ceiling reproduced independently from my own build:

```
$CARGO_TARGET_DIR/release/v06_work_inventory --class-feature-probe
# keys examined: 15225
#   no_choice_slot_offers_it              14553
#   delta_not_attributable_to_the_record    431
#   no_consumer_delta                        237
#   wired                                      4
```

The 431 refusals are the load-bearing figure: those pools move a number when *any* pick is spent,
so the record under test is indistinguishable from its control sibling. Crediting them would have
reported a number no player can see.

```
$CARGO_TARGET_DIR/release/v06_work_inventory --class-probe
# TOTAL wired: 27   (every one with >=1 attributed explanation, at level 1)
```

All 27 already-`done` classes survive the *stricter* probe. The bar rose; the count did not.

## Instruments still have teeth (proved against deliberately corrupted input, never committed)

`corpus_literal_sweep`, baseline `exit=0, 3516 records examined, CLEAN`:

| mutation on `data/corpus/pathfinder_unchained/equipment/1_attuned_armor.json` | result |
|---|---|
| one-byte magnitude drift in a raw token (`+1` → `+2`) | `exit=1` — `token not byte-present in corpus token closure` |
| `source.sha256` zeroed | `exit=1` — `digest drift: record claims 000…, file is 79adaa8f…` |
| `source.line` off by one | `exit=1` — two tokens no longer byte-present |

`derived_evaluator_fixture_check`, baseline `exit=0, 5 passed`:

| mutation on the committed fixture | result |
|---|---|
| `expected.bonus` +1, `corpus_field` untouched | `exit=101` — `fixture_expected_values_are_re_derivable_from_the_pinned_corpus_field` FAILED |
| `upstream_lst_sha256` zeroed | `exit=101` — `pinned_corpus_field_is_byte_identical_to_the_upstream_lst` FAILED |
| `upstream_line` off by one | `exit=101` — same test FAILED |

Every mutation was restored from git; `git status` is clean of both.

## Fixture provenance: independently confirmed, not taken on trust

All 94 entries of `tests/fixtures/rules_core/derived-evaluator-fixtures.json` checked directly
against the **upstream PCGen** `.lst` bytes at `~/workspace/repos/pcgen/data`, with no engine
module or engine output involved:

```
entries=94  corpus_field found VERBATIM on the named upstream line = 94/94
            upstream_lst_sha256 matches the real file                = 94/94
            expected {abilities,bonus} re-derived by my own regex over
            the upstream token text, independent of the fixture       = 94/94
```

No entry could have been authored from the output of the code it tests.

## Determinism contract holds

```
v06_work_inventory --stdout-only  (twice)
sed 's/"generated_at": *"[^"]*"/"generated_at":"X"/' on both, then cmp
-> BYTE IDENTICAL apart from generated_at
```

Stronger check, and the one that rules out hand-editing: the **committed** inventory at
`ece501f2` is byte-for-byte what the generator at `ece501f2` produces.

```
git show ece501f2:docs/work-inventory.json | (normalise generated_at) | cmp <fresh generation>
-> BYTE IDENTICAL
```

## Full gate

`./scripts/verify.sh --full`, exit code captured directly to a file, not through a pipe:
**exit 0, RESULT: PASS, 16/16 stages.** `root-lib` 1776 passed; `root-full` 6390 passed across
546 suites with all 526 `tests/*.rs` suites executed; `desktop`, `reach`, `corpus-sweep`,
`corpus-sweep-selftest` (15/15), `frontend-test` 99/99, `frontend-typecheck`, `clippy`
(root 46 / desktop 7 warnings, 0 errors), `class-dump` 31/31 computing.

Disclosure: a first launch of the gate was not killed as I intended, so **two** `--full` runs
executed concurrently on this worktree. Both recorded PASS (1547 s and 1516 s). The exit code
reported above is from the run whose `$?` I captured to a file.

Two baseline notes were reported and I deliberately did **not** act on them, because moving a
threshold is exactly what this card exists to police and neither is a failure:

- `BASELINE_ROOT_FULL_TESTS` is stale — 6371 recorded, 6390 measured (a floor that is too low).
- `BASELINE_CLIPPY_WARNINGS_ROOT` ceiling is loose — 54 recorded, 46 measured.

Both are *tightenings* and both belong in their own `--show-actuals` commit by whoever owns
`scripts/verify-baselines.env`.

## Corrections to the brief

Every figure below was re-derived; the command is given with each.

1. **`held` is not a `done` lever at all.** The brief calls the ~9,455 `held` units "the largest
   cheap lever in the program", with `static ~4,617` and `derived ~6,175` converting to `done`
   once the instruments are applied. **7,682 of the 9,457 held units are `static` or `derived`,
   and the producer's verdict table had no `done` rung for either class at any status when this
   run's agents read it.** Applying the sweep and the check to every one of them would have moved
   `done` by exactly zero. `apply-static-sweep: decision-blocked` and `apply-derived-check:
   blocked` are therefore the *correct* outcomes, not failures to execute.
   *Verified by:* replaying the producer's own `doneness_verdict()` over the tip inventory —
   held by wiring_class = `{static 4803, derived 2879, display 1259, ambiguous 400, computed 116}`.
2. **`static` held is 4,803 at the tip (4,801 at the branch point), not ~4,617.** Independently
   reproduces `closure-static`'s correction exactly.
   *Verified by:* count of `wiring_class=='static'` with status in
   {ingested-magnitude, grounded, text-complete}, excluding `beginner_box`, at both refs.
3. **`derived` *held* is 2,879, not ~6,175.** 6,175 is the total `derived` population including
   not-ingested/unknown/not-started. Reproduces `closure-derived`'s correction.
4. **Races cannot reach `done` by any instrument.** 103 race units: 96 not-started, 7 held. All
   7 grounded races (Dwarf, Elf, Gnome, Half-Elf, Half-Orc, Halfling, Human) are
   `wiring_class: ambiguous`, which caps at `held`. A race consumer-delta probe would move
   `done` by 0. `probe-race-and-class: complete, +0` is honest.
5. **Spells are 47 `done`, not ~46.** Unchanged across the window.
6. **`id` is unique in this inventory.** The brief warns it "has been observed non-unique"; at
   both `fb4b043b` and `ece501f2` all 38,540 units have distinct `id`s (0 repeats). I enumerated
   on the composite key regardless, and it produced the identical partition.
7. **This box has 8 cores, not 4.** `verify.sh` still chose `-j 2`.
8. **Minor, in a receipt rather than the brief:** `closure-cf`'s per-row table lists
   `bloodrager_bloodline_arcane`'s prior status as `unknown`; it was `not-ingested`. The same
   receipt's aggregates (`not-ingested 17,209 → 17,206`, `unknown 3,547 → 3,546`) are correct.

## One bar-adjacent change did happen — outside this repo, and it moved nothing

`~/.hermes/profiles/god-emporer/skills/release-swarm-observer/scripts/pf1e_dashboard_producer.py`
(mtime **2026-08-13 20:10**, inside this run's window; not under version control) gained a `done`
rung for `static`/`derived` in `_doneness_verdict_uncapped()`:

```python
if status in ("literal-verified", "fixture-verified"):
    return DONENESS_DONE
```

Diffed against the `.bak-2026-08-13-reconciliation-audit` snapshot from 05:15 the same day.
`doneness_meaning` and the `held` prose were edited to match.

**It moved zero units, and it is not a bar *lowering*:**

- The generator at `ece501f2` cannot emit either status — `grep -rn 'literal-verified'
  src/ tests/ scripts/` in the worktree returns nothing.
- `status_vocabulary` in the committed inventory contains neither word.
- Ordinary `ingested-magnitude`/`grounded`/`text-complete` still map to `held`, unchanged.
- My replay with the *current* producer reproduces 3,464 at the branch point, which is the figure
  the board was already showing — so nothing was retroactively re-scored.

I did not revert it: it is outside this repo, has no VCS, and reverting an inert change while a
sibling is mid-flight on it would be the more destructive act. **But it must be watched**, because
it arms the largest single number in this program:

- 2,322 `static` held units are TOKEN-COMPARED — the sweep byte-compared their magnitudes in
  full, which is the `static` bar met exactly. *Verified by:* `static-sweep-coverage.py` →
  `TOKEN-COMPARED 2322 · DIGEST-ONLY 138 · UNREACHED 2343` (= 4,803).
- The uncommitted working tree of the shared checkout (`src/bin/v06_work_inventory.rs`,
  `src/bin/corpus_literal_sweep.rs`, held by the `done-rung` agent) is adding the code that
  stamps `literal-verified`.

So a **+2,322 `done` jump is one commit away**, and its entire legitimacy rests on
`literal-verified` being stamped only for units the sweep actually token-compared — never for the
138 DIGEST-ONLY units, whose file-level sha256 proves only that the source did not drift, not that
the record's own magnitudes match the literal. The uncommitted diff appears to guard exactly that
(`matches!(item.verdict.status, "ingested-magnitude" | "grounded" | "text-complete")` plus a
digest-only exclusion), but it is unreviewed and unlanded. **That guard is the single thing the
next verification card should attack.**

## Verdict

Nothing to revert. No number in this window moved because a bar moved. `done` +2 stands.
