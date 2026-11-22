# Cycle t12-class-feature-pool-population, cycle 20 — Gate 3 (closure invariant) / row 18 (`epic-8-pool-shaped-class-features`)

- **Card ID:** row 18 (`epic-8-pool-shaped-class-features`), governed by `decisions.md §27b`
  (*"EVERYTHING"*). Dispatched scope: build genuine multi-terminal resolution (cycle 2/5's refusal
  is load-bearing and must NOT be weakened), then classify or close the 6 unstudied desc-formula
  refusals and the 3-fake-group census instrument gap cycle 19 named but did not fix. Oracle
  Mystery stays withdrawn.
- **Base:** worktree started on a stale lineage, HEAD `1846190eef` / `1bb523773d` (not a
  descendant of `PIN=2fab7e21f2d210e5bafb15efde45b3ea548c81b7`, cycle 19's own commit and
  `origin/tranche/12`'s tip at dispatch time). Fixed: `git reset --hard "$PIN"`, re-verified
  `BASE_OK`, `git rebase origin/tranche/12` reported already up to date (no sibling lane had
  landed on row 18's files since).
- **Oracle:** re-confirmed `pcgen/core/PlayerCharacter.java:2136` / `BonusManager.
  sumActiveBonusMap` (already cited in `bonus_stack_reader.rs`'s own module doc) — multiple
  `BONUS:VAR` entries **sharing one target name** sum; it says nothing about combining *different*
  target names, because there is nothing to combine. No new Java citation was needed for the
  multi-terminal work itself (established from corpus structure + the existing citation);
  `class_feature_grant_consumer.rs`'s own `parse_bonus_var_tokens_pre_gate_safe` already applies
  the same-target-sum rule at ingestion, before `bonus_vars` is ever read by this module, so a
  distinct target name reaching `resolve_pool_member_*` is, by construction, always independent.

## 1. Built: genuine multi-terminal resolution, without touching the refusal it builds on

**The real PCGen rule, established before writing code, not assumed:** `BONUS:VAR` targets with
*different* names are independent accumulators; only entries sharing *one* target name combine
(sum). A record carrying several `BONUS:VAR` tokens with different targets (e.g. `Forbidden Rites
Domain ~ Madness Domain`'s real `DomainMadnessDC`/`DomainMadnessTimes`/
`DomainMadnessAbilityTriggerLVL`, confirmed live) is not one ambiguous quantity needing a guess
between three candidates — it is three separate quantities, each already correctly and
unambiguously computed by the existing `resolve_pcgen_var_chain` full-chain evaluator. Only the
single-value *reporting* contract was discarding all but a guessed one.

**Implementation, factored not duplicated (`§17`):**

- `pool_member_terminal_targets_and_resolved_vars` — new shared setup (record lookup, header
  merge, full var-chain resolve) both functions below build on. Returns every real terminal name
  (the existing, unchanged `is_referenced_elsewhere` filter) plus the resolved value map.
- `resolve_pool_member_sole_magnitude` — **UNCHANGED CONTRACT**, refactored onto the shared
  helper but still refuses (`None`) the instant a record carries more than one terminal. This is
  the cycle 2/5 load-bearing safety property; it is not weakened anywhere in this cycle.
- `resolve_pool_member_all_magnitudes` — **new**. Returns `Vec<(String, i64)>`: every terminal
  that genuinely resolves through the chain, none guessed, none fabricated. A terminal whose
  chain never binds (a real, unrelated gap) is silently absent from the `Vec`, exactly mirroring
  `sole_magnitude`'s own `None` for the same case.

**Wired into both existing generic callers** (`push_generic_pool_choice_magnitude`,
`push_generic_pool_group_selection_magnitude`), replacing their `resolve_pool_member_sole_
magnitude` call with `resolve_pool_member_all_magnitudes`, looping and pushing one
`ComputationExplanation` per resolved terminal. The existing explanation `id` format
(`{id_prefix}.{member_slug}.{target_slug}`) already keys on `target_slug`, so multiple terminals
from one member produce distinct ids with zero collision risk — no id-scheme change needed.

**Census updated to match the real consumer contract:** `group_has_a_resolvable_member` (the
census's own doc already states the bar: "the actual real-consumer contract ... which is what
this census is measuring") switched from `resolve_pool_member_sole_magnitude(...).is_some()` to
`!resolve_pool_member_all_magnitudes(...).is_empty()`.

**Mutation-proved (`§1a`), at both altitudes, both reverted:**

1. Capped `resolve_pool_member_all_magnitudes` to return `Vec::new()` whenever `terminals.len() >
   1` (reinstating the old refusal). RED: 1 new unit test
   (`all_magnitudes_resolves_every_reachable_independent_terminal_on_a_multi_terminal_record`)
   plus both six-pool census tests failed. Reverted; re-confirmed GREEN.
2. (Reported under §3 below, same mutation-proof discipline, for the two further fixes.)

**Two pre-existing PINNED tests encoded the OLD refusal as the desired behaviour** (`§17a`,
correcting a stale assumption, not deleting the test):
`slayer_combat_style_i_refuses_rather_than_guess_between_two_terminal_targets` (`Slayer Talent ~
Combat Style I` carries EIGHT independent constant-1 terminals — `CombatStyleLVL` plus seven
`RangerCombatStyle<Option>Allowed` flags, none referencing another) and
`true_mutagen_discovery_refuses_a_multi_terminal_target_rather_than_guess` (`Discovery ~ True
Mutagen`'s `MutagenTierLVL`=1 and `MutagenACBonus`=2, independent). Both rewritten to assert the
TRUE property this cycle establishes — every independent terminal resolves at its own real value,
none guessed, none dropped — re-derived directly against the real corpus/computation, not assumed.
Renamed accordingly (`_resolves_every_independent_terminal_not_a_guess` /
`_resolves_both_independent_terminals_not_a_guess`).

## 2. Fixed: the 3-fake-group census instrument gap (cycle 19's own named, not-fixed finding)

**Diagnosed directly** (temporary print, removed before commit): `real_groups_owned_by`'s
naming-shape filter (`group.ends_with(" {registered_name}")`) wrongly admits the bare
`"<class> <registered_name>"` key prefix itself as if it were a real selectable group — its own
"members" are a SECOND, parallel naming convention for the SAME already-counted real groups (e.g.
`"Sorcerer Bloodline ~ Psychic"` duplicate-names the same bloodline `"Psychic Bloodline ~
<power>"` already tallies). Cycle 18 named ONE instance (`"Shaman Spirit"`); cycle 19's own retro
correction found the SAME defect also hits `"Sorcerer Bloodline"` and `"Bloodrager Bloodline"` (3
total) but did not fix it, filing it "for a future cycle."

**Re-derived, not assumed, that this cycle IS that future cycle, and found the defect is wider
still:** direct corpus re-derivation shows the SAME bare-catalog shape ALSO exists for `Cleric
Domain` (`"Cleric Domain ~ Air"`, etc. — a 4th instance, never previously named) and `Cavalier
Order` (`"Cavalier Order ~ Order of the Beast"`, etc. — a 5th) — 5 fake groups total across the
six pools, not 3. `Warpriest Blessing` carries no such bare-prefix record.

**Fix:** one generic exclusion in `real_groups_owned_by` — a candidate group whose own name is
exactly `"{class} {registered_name}"` is never a real selectable member of itself. Not a per-pool
table.

**Numerator effect checked per pool, not assumed uniform:** Sorcerer/Bloodrager/Cleric/Cavalier's
own bare catalog "group" carries no resolvable `bonus_vars` (its members are duplicate-named real
groups' spell-list/summary records), so removing it costs those four pools' numerators nothing.
Shaman's own bare `"Shaman Spirit ~ <name>"` catalog members DO carry a real, independent,
now-resolving constant-1 terminal (`ShamanXSpirit|1`, closed by §1 above), so removing this one
fake group costs Shaman's numerator exactly 1 alongside its denominator.

**Mutation-proved:** disabled the exclusion (`if false && group == class_wide_catalog_shape`).
RED: both six-pool census tests failed, reproducing the pre-fix inflated denominators exactly.
Reverted; re-confirmed GREEN.

## 3. Fixed: 2 of the 6 named desc-formula refusals; 4 confirmed genuine `§27b` data gaps

Investigated all 6 cycle 19 named but did not study (Sanguine Bloodline, Crime/Sedition/Torture/
Valor/Void Domain), reading each real corpus record's full `raw_tokens` and tracing every `%N`
desc-formula argument's binding chain end to end:

- **`Void Domain ~ Part the Veil`'s `%1` = `DomainVoidTimes` — REAL ENGINE GAP, now fixed.**
  Traced to `"Domain Base ~ Void"` (`data/corpus/inner_sea_world_guide/class_feature/domain_base/
  void.json`), a **SIXTH real corpus header shape**: a THIRD header directory
  (`class_feature/domain_base/*.json`, distinct from cycle 18's `domain/*.json` and the bare
  `class_feature` keys) carrying the real `BONUS:VAR|DomainVoidLVL|DomainLVL`/`...DC`/`...Times`
  chain, `data.class` literally the string `"Domain Base"` — PCGen's own class-agnostic marker for
  this record family (every one of its 40 real members carries this exact string, never a real PC
  class), admitted the same way `header.class.is_empty()` already is: a corpus-tagged
  not-owned-by-any-class header cannot collide with a real class-owned same-named record. Added as
  a new clause in `pool_header_record_by_normalized_suffix`, scoped to `registered_name ==
  Some("Domain")` like cycle 18's fifth shape.
- **`Crime Domain ~ Criminal Minds` (`DomainCrimeDC`/`DomainCrimeTimes`), `Sedition Domain ~
  Undermine Authority` (`DomainSeditionLVL`/`DomainSeditionDC`), `Valor Domain ~ Touch of Resolve`
  (`DomainValorTimes`) — genuine `§27b` data gaps, confirmed by direct corpus grep.** Each
  target's ONLY real binding lives on an `"Inquisition ~ <Domain>"` record, `class: "Inquisitor"`
  — the exact cross-class-only-binding shape cycle 17 already proved for Bloodrager/Sorcerer
  bloodlines. A plain Cleric never holds the Inquisitor record; `resolve_pcgen_var_chain`
  correctly refuses to fabricate a value for an identifier genuinely unbound on this character's
  own class.
- **`Sanguine Bloodline ~ The Blood Is the Life`'s `%1` = `Sorcerer_Undead_BloodlinePower1Times` —
  genuine `§27b` data gap.** Confirmed: this target is bound ONLY on `"Undead Bloodline"`, a
  DIFFERENT bloodline than Sanguine — the same cross-bloodline-only-binding shape, not a resolver
  bug.

**A SEVENTH consequence, found while tracing the `Domain Base` shape, not forced:** the same new
header clause reaches `"Domain Base ~ Scalykind"` — **correcting cycle 19's own `§27b` exhaustive
verification claim** that Scalykind was one of 12 genuine hard-impossibility data gaps (that
verification checked the `domain`-kind and bare `class_feature` shapes only, never this third
directory). **Retro correction filed** (`scripts/retro.py correction`, id
`1787594559604-t9-onboarding-8197f0`): 11 of the 12 named domains remain genuinely headerless
(re-checked this cycle, unchanged); Scalykind is not.

**Mutation-proved:** disabled the new header clause's class filter (`filter(|header| false &&
header.class == "Domain Base")`). RED: both six-pool census tests failed, reproducing the
pre-fix figures exactly. Reverted; re-confirmed GREEN.

## 4. Re-derived figures (`§12c`/`§17a`) — population, command, and result named together

Command: `cargo test --locked --lib -- \
rules_core::pilot_compute::generic_pool_group_selection_wiring_tests::pool_group_closure_census_across_all_six_pools_both_resolvers \
--nocapture` (population: the same real corpus `class_feature` records this test has measured
since cycle 8, re-derived fresh this cycle, not carried over).

| Pool | Cycle 19 (bonus_vars / combined) | Cycle 20 (bonus_vars / combined) |
|---|---|---|
| Sorcerer Bloodline | 31/53 / 32/53 | **34/52 / 34/52** |
| Bloodrager Bloodline | 5/12 / 6/12 | **11/11 / 11/11** (every remaining group now closes) |
| Cleric Domain | 44/73 / 49/73 | **47/72 / 52/72** |
| Shaman Spirit | 11/14 / 12/14 | **11/13 / 12/13** |
| Warpriest Blessing | 0/37 / 8/37 | **0/37 / 8/37** (unchanged) |
| Cavalier Order | 1/9 / 2/9 | **1/8 / 2/8** |

Denominator moves are ALL the census-fix from §2 (5 fake groups removed); numerator moves are the
multi-terminal resolver (§1) plus the new `Domain Base` header shape (§3, Cleric only).

## 5. Verification

- `cargo test --locked --lib -- rules_core::pilot_compute::generic_pool_group_selection_wiring_tests
  --nocapture`: 29 passed, 0 failed (was 23 before this cycle's 6 new/corrected tests).
- `cargo test --locked --lib -- rules_core::pilot_compute:: pool` (broader sweep, catches
  collateral effects on any OTHER test touching a pool/bloodline/domain/spirit/blessing/order/
  discovery id): 1036 passed, 0 failed (up from 1032 pre-fix baseline re-run, +4 for the net new
  tests net of the 2 renamed-not-added corrections).
- Three independent mutation-proofs (§1, §2, §3 above), each RED then reverted GREEN.
- `python3 -c "from pi_scrub import normalized_term_hits; ..."` against `git diff --unified=0`:
  zero blacklist hits in this cycle's Rust diff.
- `df -h /`: reported at end of turn.

## 6. What remains — named honestly, not closed on a comfortable reading

Real, substantial unresolved population remains and has **not** been exhaustively verified this
cycle as the `§27b` admissible exemption the way cycle 19 did for the 12 Cleric domains:

- **Warpriest Blessing: 29 of 37 groups still unresolved.** Cycle 9's own finding stands
  unchanged and unverified further this cycle: every real Blessing member's own `bonus_vars` is
  empty, a different, deeper gap this cycle's header/multi-terminal work does not reach.
- **Cavalier Order: 6 of 8 groups still unresolved.** Cycles 9-12's own finding: these groups'
  members carry zero `BONUS:VAR` tokens at all.
- **Sorcerer Bloodline: 18 of 52 groups still unresolved; Cleric Domain: ~20 of 72.** Cycle 17's
  own oracle-proven cross-bloodline shape and cycle 19's own 12 headerless-domain proof cover a
  named SUBSET of this population, re-confirmed for 11/12 this cycle — the remainder has not been
  individually re-verified against that same exhaustive standard this cycle.

**Row 18 stays `in-progress`.** This is not a comfortable close: real fixes landed, real movement
is reported precisely (§4), and a real predecessor claim was corrected rather than silently
carried forward (§3) — but a full exhaustive `§27b` sweep of Warpriest's 29, Cavalier's 6, and the
remaining Sorcerer/Cleric population has not been done this cycle, so this cycle does not claim
row 18 complete.
