# Cycle 010 — Epic 1 (compute library) / Criterion AT-32-E1-001 (F3: wire the library behind a consumer)

- **Card ID:** `epic-1-compute-library` (kanban.md #10)
- **Commit SHA:** `eab89b08e` (feat commit; the receipt-fixup commit `52b0b3485` is on top of it,
  both pushed to `origin/tranche/12`)
- **Files touched:**
  - `src/rules_core/pilot_compute/mod.rs` — new `resolve_class_feature_bonus_var` helper
    (reuses `class_feature_grant_consumer::{class_feature_record_tokens, resolve_pcgen_var_chain}`,
    unchanged); `explain_rogue_level1_chassis` gains an `ability_modifiers` parameter; the Rogue
    Master Strike (level ≥ 20) and Ranger Master Hunter (level ≥ 20) explanation branches now
    compute their save DC through the corpus's own `BONUS:VAR` formula instead of a fabricated
    `value: 0`.
  - `tests/sd18_rogue_level20_widening.rs`, `tests/sd18_ranger_level20_widening.rs` — widened
    `rogue_level20_gains_master_strike` / `ranger_level20_master_hunter_is_granted` to assert the
    real computed DC (21 for both, at these fixtures' ability scores) instead of the pre-wiring
    `value: 0`.
  - `tests/sd20_levelup_rogue.rs`, `tests/sd20_levelup_ranger.rs` — same widening for the
    level-up-plan surface's own master-strike/master-hunter grant assertions.
  - This file (`010_cycle_receipt.md`).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (see RED→GREEN section below for the exact
  command, run against the committed diff)
- **Wired-integration audit result:** `OK_NO_TOKENS` (see below)
- **Acceptance criterion (verbatim, `acceptance-and-verification.md` AT-32-E1-001):**
  > **Compute library delivers 3,201 ceiling.** (Epic 1 F1/F2/F3 deliver.) The ceiling figure is
  > the union of unit populations across the ten families, minus the 1,747-unit flat-constant
  > family (which gets zero benefit from any shared function — see `epic-breakdown.md Epic 1`).
  > The ceiling is not a target; it is the measured upper bound.

## Scope of this cycle, stated plainly

`epic-breakdown.md` Epic 1's three work items:

- **F1** (extract the general form of each family from the ~166 hand-modelled functions) and
  **F2** (generalise `bonus_stack_reader.rs` to F10) are **already delivered** by Gate 2's own
  cards (`gate-2-engines-f1-f9` #6, `gate-2-engines-f10-binding` #7, `gate-2-corpus-wide-runs`
  #8) — `formula_interpreter.rs` reaches F1..F9 corpus-wide (4,696/4,798 recognised,
  fixture-checked) and the generalised `bonus_stack_reader.rs` reaches F10's 77.2% ceiling
  corpus-wide (3,519/4,736 resolved, fixture-checked). This cycle does not re-derive or re-claim
  either — citing, not re-closing, per the same convention `AT-32-E2-001` uses for T5/T3.
- **F3** ("Wire the library behind the consumers, every value clearing
  `derived_evaluator_fixture_check`") is the genuinely open item: neither engine, once built and
  corpus-wide-proven, was actually reachable by a real player-facing consumer before this cycle.
  `resolve_pcgen_var_chain` (the interpreter-backed generic class-feature-description resolver,
  `class_feature_grant_consumer.rs`, SD-31 waves 26/27) already resolves the Rogue Master Strike
  and Ranger Master Hunter save-DC formulas correctly and is already fixture-checked for both
  (`tests/fixtures/rules_core/derived-evaluator-fixtures.json`'s `class_feature_description_entries`,
  `rogue_master_strike` / `ranger_master_hunter`) — but the resolved value never reached the
  player, because `push_generic_class_feature_grant_records`'s own `already_computed_slugs` guard
  correctly suppresses it in favor of a pre-existing hand-modelled `value: 0` explanation in
  `pilot_compute/mod.rs` that predates the interpreter. This exact gap was found and named as
  concrete, scoped future-wave work by SD-31's own integration cycle
  (`SD-31-corpus-closure-grind/artifacts/OPEN-ISSUES.md` row 375, `SD31-W27-INTEGRATE-005`): *"wire
  `push_generic_class_feature_grant_records`'s resolved DC/value into the existing hand-modelled
  ... explanation `detail` strings at `pilot_compute/mod.rs`."*

**This is the smallest, safest, real instance of that gap**, closed this cycle. It does not claim
the 3,201-unit ceiling — see "What this cycle does NOT claim" below.

## What this cycle built

`resolve_class_feature_bonus_var(record_key, class_level_var, target_var, level,
ability_modifiers)` in `pilot_compute/mod.rs`: looks up a real corpus `class_feature` record's own
`BONUS:VAR` tokens (via the already-committed, already-tested `class_feature_grant_consumer::
class_feature_record_tokens()`), resolves the named target variable through
`class_feature_grant_consumer::resolve_pcgen_var_chain` (the SAME `PcgenFormulaEvaluator`-backed
fixed-point resolver operator ruling §20 authorised and `decisions.md §3` restates), and returns
`None` — never a guess — if the chain does not fully resolve.

Two real call sites:

1. **Rogue Master Strike** (`class_feature.rogue.master_strike`, level ≥ 20): now computes
   `MasterStrikeDC` from `Rogue ~ Master Strike`'s own `BONUS:VAR|MasterStrikeDC|10+(MasterStrikeLVL/2)+INT`
   (`cr_abilities_class.lst:1619`), seeded with the character's real rogue level and real
   Intelligence modifier.
2. **Ranger Master Hunter** (`class_feature.ranger.master_hunter`, level ≥ 20): now computes
   `MasterHunterDC` from `Ranger ~ Master Hunter`'s own `BONUS:VAR|MasterHunterDC|10+(MasterHunterLVL/2)+WIS`
   (`cr_abilities_class.lst:1427`), seeded with the character's real ranger level and real Wisdom
   modifier.

Both branches keep a `None`-resolves-to-`value: 0` fallback (refuse rather than guess) if the
chain ever fails to resolve — the pre-wiring behaviour is preserved as the safety net, not
replaced outright.

## Why this specific pair, and why it is safe

- **Independently pinned twice already.** Both records' formulas and expected values at several
  sample levels are already committed, oracle-provenance-verified fixtures
  (`class_feature_description_entries`, `derived_evaluator_fixture_check_class_feature_description.rs`,
  5/5 green, unchanged by this cycle) — this cycle's new call sites reuse that already-cleared
  evaluator path rather than adding a second, uncoordinated one. Decision 3 ("every interpreted
  value clears `derived_evaluator_fixture_check`") is satisfied by construction: the value this
  cycle now surfaces to the player is the SAME value that family's fixture already independently
  checks, not a new unchecked computation.
- **Hand-verified a third time, by hand, for these tests' own ability scores** (not just the
  fixture's ability scores): Rogue level 20, INT 13 (modifier +1, `(13-10)/2=1`, PF1 standard
  floor-division rule): `10 + 20/2 + 1 = 21`. Ranger level 20, WIS 12 (modifier +1): `10 + 20/2 +
  1 = 21`. Both match what the widened tests now assert and what the engine actually returned
  (RED→GREEN below).
- **No allowlist collision.** Grepped every test file for `master_strike`/`master_hunter` before
  touching anything (`grep -rln "master_strike\|master hunter\|Master Strike\|Master Hunter"
  tests/`); the only assertions on the numeric `value` were the four updated in this cycle
  (`sd18_rogue_level20_widening.rs`, `sd18_ranger_level20_widening.rs`, `sd20_levelup_rogue.rs`,
  `sd20_levelup_ranger.rs`) — none of the CLOSED-ALLOWLIST-shaped gates `class_feature_grant_
  consumer.rs`'s own module doc names (Wizard/Bard/Paladin/Cleric/Sorcerer) touch the Rogue or
  Ranger namespace at all.

## What this cycle does NOT claim

- **Not the 3,201-unit ceiling.** This cycle wires exactly 2 units (both already `derived`+
  `grounded` before this cycle — their `wiring_class` does not change; what changes is that the
  explanation's own `value` field is now a real magnitude instead of a fabricated 0, matching the
  gap SD-31 row 375 named). The remaining F1..F9/F10 population reachable by the same
  "generic-consumer-vs-pre-existing-hand-modelled-explanation" suppression pattern is NOT
  inventoried by this cycle — that inventory (how many other `already_computed_slugs` collisions
  exist corpus-wide, and how many resolve to a real DC/magnitude the way these two did) is
  legitimate, concrete next-cycle scope, named below.
- **Not a board-percentage claim.** No `v06_work_inventory`/`doneness_verdict` re-derivation was
  run this cycle; both units were already counted as `held`/`grounded` before this cycle under
  their pre-existing `derived`+`grounded` wiring class (SD-31 row 366/375's own precedent: a
  `derived`-class fixture can be `done` without a new production consumer). This cycle changes
  what the player actually sees, not the denominator or the counted-done set.

## RED → GREEN evidence

**RED (both widened tests fail for the intended reason — the value is still the pre-wiring 0):**
```
CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-epic-1-compute-library \
  cargo test --locked --test sd18_rogue_level20_widening -- rogue_level20_gains_master_strike
# left: 0
# right: 21
CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-epic-1-compute-library \
  cargo test --locked --test sd18_ranger_level20_widening -- ranger_level20_master_hunter_is_granted
# left: 0
# right: 21
```

**GREEN (after wiring `resolve_class_feature_bonus_var` into both branches):**
```
CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-epic-1-compute-library \
  cargo test --locked --test sd18_rogue_level20_widening --test sd18_ranger_level20_widening \
  --test sd18_rogue_level19_widening --test sd18_ranger_level19_widening
# test result: ok. 17 passed  (level 20 files)
# test result: ok. 9 passed   (rogue level 19, unaffected)
# test result: ok. 11 passed  (rogue level 20)  [re-listed by cargo per binary]
CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-epic-1-compute-library \
  cargo test --locked --test sd20_levelup_rogue --test sd20_levelup_ranger
# test result: ok. 3 passed; 0 failed  (each)
CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-epic-1-compute-library \
  cargo test --locked --test sd27_pu_class_feature_descriptions_carry_the_characters_numbers \
  --test sd27_pu_class_features_reach_by_corpus_key
# test result: ok. 10 passed  (each)
CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-epic-1-compute-library \
  cargo test --locked --test derived_evaluator_fixture_check_class_feature_description
# test result: ok. 5 passed  (unchanged — the already-committed fixture still clears)
CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-epic-1-compute-library \
  cargo test --locked --lib rules_core::pilot_compute
# test result: ok. 856 passed; 0 failed
CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-epic-1-compute-library \
  cargo test --locked --lib
# test result: ok. 2356 passed; 0 failed; 13 ignored
```

## Discovery during this cycle: two other pre-existing tests asserted the old fabricated `0`

`tests/sd20_levelup_rogue.rs::rogue_level_19_to_20_crosses_the_capstone_threshold_and_grants_
master_strike` and `tests/sd20_levelup_ranger.rs::ranger_level_19_to_20_crosses_the_capstone_
threshold_with_master_hunter` (the level-up-plan surface, a separate consumer of the same
`ComputationExplanation` list) both independently asserted `effects[0].value == 0` for these two
records. Found by running the widest reasonable test net before declaring green (not assumed
clean from the two files initially touched) — widened both, same derivation (INT/WIS modifier +1
from each fixture's own `human_rogue_input`/`human_ranger_input` ability scores, `10 + 20/2 + 1 =
21`), not weakened.

## Identifier audit result

```
git diff --unified=0 1bb523773d32705d1b7387fd4c494861523f55ba..HEAD \
  -- src/rules_core/pilot_compute/mod.rs tests/sd18_ranger_level20_widening.rs \
     tests/sd18_rogue_level20_widening.rs tests/sd20_levelup_ranger.rs tests/sd20_levelup_rogue.rs \
  > /tmp/sd32-epic1-committed-diff.txt
grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' /tmp/sd32-epic1-committed-diff.txt \
  || echo 'OK_NO_BUNDLE_TAGS'
```
Raw run surfaces 8 matches, all on `diff --git a/... b/...` / `---` / `+++` file-header lines
naming the four pre-existing `sd18_*`/`sd20_*` test filenames (unchanged by this cycle, not new
identifiers) — the same known false-positive shape card 008's own receipt documented and excluded
(that card's own `007_cycle_receipt.md` self-matching its own documented grep pattern). Filtering
those header lines confirms zero real matches:
```
grep -vE '^(diff --git|---|\+\+\+|index )' /tmp/sd32-epic1-committed-diff.txt \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})' || echo 'OK_NO_BUNDLE_TAGS'
```
`OK_NO_BUNDLE_TAGS`.

## Wired-integration audit result

```
git diff --unified=0 1bb523773d32705d1b7387fd4c494861523f55ba..HEAD \
  -- src/rules_core/pilot_compute/mod.rs tests/sd18_ranger_level20_widening.rs \
     tests/sd18_rogue_level20_widening.rs tests/sd20_levelup_ranger.rs tests/sd20_levelup_rogue.rs \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
```
`OK_NO_TOKENS`.

## Corpus SHA

`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
bootstrapped fresh into this worktree's git-ignored slot via `scripts/fetch-pcgen-oracle.sh`,
confirmed at the same SHA Gate 3's own receipt cites). No new figure was re-derived from the
oracle directly this cycle (the two formulas and their upstream `.lst` citations were already
independently pinned by the pre-existing `class_feature_description_entries` fixture); the DC
arithmetic (`10 + level/2 + ability_modifier`) is hand-verified directly from that already-pinned
formula text, not re-walked against the oracle checkout.

## Status: complete

## Notes

- Footgun 1 fired at the start of this cycle: the fresh worktree's `HEAD` (`275581bf0`, a
  site-publish merge commit) was NOT a descendant of `PIN=0ef8dd5cf5`. Tree was clean
  (`git status --porcelain` empty), so self-healed per §8: `git reset --hard origin/tranche/12`
  (verified `PIN` is an ancestor of `origin/tranche/12` first), re-verified the mechanical check
  passes, then proceeded. No `scripts/retro.py correction` filed for this one — it is exactly the
  known, named, self-healable case §8 already covers (empty precondition, mechanical fix, no
  judgment call), not a new finding.
- PCGen oracle slot was empty in this fresh worktree (self-healable per §8); bootstrapped via
  `scripts/fetch-pcgen-oracle.sh` before any corpus-touching work, landed at the pinned SHA.
- `docs/retro/events/sd31-transcribe.jsonl` picked up a stray auto-appended `verify.sh`-generated
  event line (from the pre-fetch `preflight-oracle` FAIL, auto-logged by `verify.sh` itself under
  an unrelated actor name — each `Bash` tool call in this harness is a fresh shell, so an
  `export RETRO_ACTOR=...` from an earlier call does not persist into a later one, and the
  fallback actor resolution picked up an unrelated pre-existing env value). Reverted via
  `git checkout -- docs/retro/events/sd31-transcribe.jsonl` before committing, to keep this
  cycle's diff scoped to its own real work — not a suppressed finding, just a housekeeping
  side-effect with no bearing on this card's own criterion.

## Discovery forwards

- **`## DISCOVERED`, `progress.md`:** how many other `class_feature.<class>.<slug>` explanations
  in `pilot_compute/mod.rs` carry a hand-modelled `value: 0` "grant-only identity record" note
  whose corpus record ALSO carries a fully-resolvable `BONUS:VAR` chain (i.e. the same
  `already_computed_slugs`-suppression shape this cycle closed for exactly two units) is not
  inventoried. A systematic sweep — grep `pilot_compute/mod.rs` for `"identity record"` /
  `"bounded grant-only"` value-0 explanations, cross-reference each against
  `class_feature_record_tokens()`'s own resolvable-chain population — is real, scoped follow-on
  work for the next Epic 1 cycle, closer to Epic 1's actual 3,201-unit ceiling than this cycle's
  own 2-unit proof-of-mechanism.

## Next-cycle plan

The systematic sweep named above (DISCOVERED forward), then extending the same
`resolve_class_feature_bonus_var` pattern to every resolvable hit it finds, each with its own
RED→GREEN test widening and its own confirmation that the resolved value is already covered by
an existing (or newly added) `derived_evaluator_fixture_check` fixture per Decision 3. Measuring
how close the corpus-wide sweep gets to the 3,201 ceiling (and naming, honestly, what fraction of
that ceiling this consumer-wiring pattern alone can reach vs. what needs a different consumer
shape) is the next cycle's own deliverable, not assumed here.
