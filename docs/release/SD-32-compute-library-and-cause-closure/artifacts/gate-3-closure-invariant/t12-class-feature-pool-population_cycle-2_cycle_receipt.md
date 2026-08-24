# Cycle t12-class-feature-pool-population, cycle 2 — Gate 3 (closure invariant) / row 18 (`epic-8-pool-shaped-class-features`)

- **Card ID:** row 18 (`epic-8-pool-shaped-class-features`), governed by `decisions.md §27b`
  (*"EVERYTHING"*). Scope this cycle: the **5,981 numeric-magnitude pool records** cycle 1 sized
  and left as next-cycle scope (`t12-class-feature-pool-population_cycle-1_cycle_receipt.md`,
  commit `69cd7537a8`).
- **Base:** started on a WRONG worktree lineage (`HEAD` at `1bb523773d`, a `tranche/11`-merge
  commit, unrelated to the pinned `PIN=69cd7537a8`) — footgun 1 fired. Recovered per
  `workflow-instruction.md §6` step 1: `git reset --hard "$PIN"`, re-verified
  `git merge-base --is-ancestor "$PIN" HEAD`, then `git rebase origin/tranche/12` (no-op — HEAD
  already equalled `origin/tranche/12` at the pin).
- **Files touched:**
  - `src/rules_core/pilot_compute/mod.rs` — new generic pool-choice magnitude resolver
    (`resolve_pool_member_sole_magnitude`, `formula_names_identifier`,
    `resolve_pool_selection_corpus_key`, `push_generic_pool_choice_magnitude`,
    `ability_modifiers_from_scores`), one wired call site (Alchemist Discovery, purely additive
    alongside the existing hand-modelled Feral Mutagen path), 9 new tests.
  - `src/rules_core/pilot_compute/class_feature_grant_consumer.rs` — two new sibling lookup
    tables (`parse_bonus_var_tokens_pre_gate_safe`, `class_feature_record_tokens_pre_gate_safe`,
    `class_feature_bonus_vars_any_record`), reusing the existing `repo_root`/`walk_json_files`/
    `is_real_description_value` machinery already in this file; no existing function's behaviour
    changed.
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 18 only (cycle bumped
    0 -> 1, Notes appended), verified still 9-pipe-delimited and the file's own line count
    unchanged (86) before and after.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — scoped to this cycle's own touched files
  (`git diff --unified=0 -- src/rules_core/pilot_compute/mod.rs
  src/rules_core/pilot_compute/class_feature_grant_consumer.rs`), `grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'` -> 0 hits. Kanban row addendum separately
  checked the same way -> 0 hits.
- **Wired-integration audit result:** `OK_NO_TOKENS` — same diff scope, `grep -inE
  "STUB|MOCK|placeholder|not.?yet.?implemented|todo|fixme|hack"` -> 0 hits (this cycle's own new
  code contains no stub/placeholder markers; it is a real evaluator wired to a real chassis call
  site with real tests, not a scaffold).
- **PI audit:** `grep -rniE` of this cycle's diff against `scripts/pi_scrub.py`'s blacklist terms
  -> 0 hits. No corpus record name, blacklist term, or PI item name appears in any receipt, test
  name, test constant, kanban row, or this file (`§24b`-2). `data/corpus/**` untouched throughout
  (`git status --porcelain -- data/corpus` — 0 changes).
- **Acceptance criterion (this cycle's stated scope):** re-derive the 5,981-record residual per
  `§17a`; build the generic (group-agnostic) compute mechanism the population needs, per `§17`;
  wire and prove it end-to-end for at least one whole group; report the true remainder honestly,
  by group and count, per `§16`/`§12c`.
- **Corpus SHA:** no oracle re-fetch needed (`data/corpus/` already-ingested cache, unchanged);
  `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`, confirmed present via
  `scripts/verify.sh --only preflight-oracle` (`PASS`).
- **Status:** in-progress — genuine, multi-cycle epic, unchanged from cycle 1's own framing.
  3 records closed end-to-end this cycle; the reusable mechanism that makes every future cycle's
  marginal cost small (no new evaluator, no new safety-gate design) is the real deliverable.
- **Next-cycle plan:** wire `push_generic_pool_choice_magnitude` at the remaining
  `CLASS_FEATURE_POOLS`-registered pools' own call sites (Witch Hex, Oracle Curse/Mystery/
  Revelation, Shaman Spirit, Cavalier Order, Hunter Animal Focus, Inquisitor Judgment, Warpriest
  Blessing, Summoner Evolution, Bloodrager Bloodline, Investigator Talent, Slayer Talent) — each
  needs its OWN real corpus verification of (a) the exact group-prefix name (this cycle found
  `class_feature_pool_group_matches`'s fuzzy suffix match diverges from this census's exact-prefix
  parsing — e.g. `CLASS_FEATURE_POOLS`'s registered `"Spirit"` group has ZERO `"Spirit ~ "`-prefixed
  corpus records; the real Shaman ability prefix differs and needs re-deriving before wiring) and
  (b) whether a pre-existing shared-formula Rust function already covers most of the group (Witch
  Hex precedent below) before assuming the whole numeric-magnitude count is unclosed. Then classify
  the true remainder into the four shapes and either close it the same way or escalate genuine
  4-shape resistance per `§27b`.

---

## 1. Re-derivation (§17a) — unchanged from cycle 1, re-confirmed live

```bash
python3 scripts/census_class_feature_pool_population.py
```
```
class_feature pool-shaped population census (decisions.md §17/§17a)
  files scanned                                18076
  malformed JSON                                    0
  distinct ' ~ '-group-qualified names           1913
  total group-qualified records                 16350
  catalog-servable text-only (no engine token)    7423
  any engine-effect token (ABILITY/CSKILL/SELECT/AUTO/SAB/BONUS/DEFINE/ADD/SPELLS/DR/SR)
    records                                       8927
  numeric magnitude (BONUS/DEFINE var math, or %N-substituted DESC)
    records                                        6306
  already modeled elsewhere (groups: Domain Power, Inquisitor Domain, Rage Power, Rogue Talent)
    records                                         596
    of which numeric-magnitude                      325
  RESIDUAL numeric-magnitude needing compute       5981
```

Exact match to cycle 1's own figures (no drift on this branch since cycle 1's commit). Starting
residual for this cycle: **5,981**.

## 2. The generic mechanism (§17, not per-object)

Reused, unmodified: `class_feature_grant_consumer::resolve_pcgen_var_chain` (the real
`PcgenFormulaEvaluator`, fixed-point over a record's own `BONUS:VAR` chain, seeded with the real
class level and real six ability modifiers — the same, already-fixture-checked mechanism SD-32
Epic 1's `resolve_class_feature_bonus_var` uses for its own hand-picked records, e.g. Ranger Master
Hunter's save DC).

**New, generic (`pilot_compute/mod.rs`):**

- `resolve_pool_member_sole_magnitude(key, pool_group, level, ability_modifiers) ->
  Option<(String, i64)>` — finds a `" ~ "`-qualified pool member's own "terminal" `BONUS:VAR`/
  `DEFINE` target (the one target never referenced by another of the SAME record's own formulas —
  an intermediate chain hop is excluded automatically), refusing (not guessing) when zero or more
  than one terminal exists. Merges the pool's own HEADER record's chain
  (`"<class> ~ <pool_group>"`, e.g. `"Alchemist ~ Discovery"`) into the resolution environment
  first, because many members scale on a pool-specific level variable (`AlchemistDiscoveryLVL`)
  the header alone defines (`BONUS:VAR|AlchemistDiscoveryLVL|AlchemistLVL`) — confirmed live, this
  is the difference between the resolver returning `None` and `Some(10)` for
  `Discovery ~ Spontaneous Healing` (see §4, RED-proof 2).
- `resolve_pool_selection_corpus_key(pool_group, namespace, selection_id) -> Option<String>` —
  reverse-maps a real, recorded `SelectedChoice::selection_id` back to its exact corpus key by
  forward-slugging every candidate member name through `class_feature_id_slug` (the SAME transform
  this codebase's own hand-picked selection constants were built with) and matching. Never guesses;
  an invented id resolves to `None`.
- `push_generic_pool_choice_magnitude(...)` — the consumer: for every real selection under a given
  `choice_set_id`, resolves the key, resolves the magnitude, and pushes ONE
  `ComputationExplanation` (id `<id_prefix>.<member_slug>.<target_slug>`) if and only if both
  resolve and `level >= min_level` (the pool's own grant gate — Alchemist Discovery's own
  `AlchemistDiscoveryLVL/2`, so a selection recorded below level 2 grounds nothing, exactly
  mirroring every hand-picked `ground_*` function's own level check).

**New safety gate (`class_feature_grant_consumer.rs`), found necessary mid-cycle, not
anticipated:** the existing `parse_bonus_var_tokens`/`class_feature_record_tokens` (SD-32 Epic 1)
silently keeps only the LAST `BONUS:VAR` row when a target name repeats — safe for a human
independently verifying one hand-picked record at a time, unsafe for this cycle's own generic,
per-record-unverified pass. Confirmed live: `Discovery ~ Force Bomb` carries
`VAR|ForceBombDieSize|3|PREVAREQ:...,1` AND `VAR|ForceBombDieSize|4|PREVAREQ:...,0` — the existing
parser would silently pick `4` regardless of which `PREVAREQ` gate the character actually
satisfies, a genuinely wrong number, exactly the failure `§1a` exists to prevent.
`parse_bonus_var_tokens_pre_gate_safe` + `class_feature_record_tokens_pre_gate_safe` (member
lookup, description-gated) + `class_feature_bonus_vars_any_record` (header lookup, NOT
description-gated — many header records carry `description: null` in this corpus, confirmed live
for `Alchemist ~ Discovery`) refuse any target with more than one raw row or a PRE-gate tail,
dropping it from the map entirely rather than guessing. This is genuinely new library surface, not
a modification of any existing, already-verified caller's behaviour — `resolve_class_feature_bonus_
var`'s two existing call sites (Ranger Master Hunter and one more) are untouched and still use the
original, un-gated table.

## 3. Wired end-to-end, one whole pool as proof-of-mechanism

Alchemist Discovery (`ALCHEMIST_DISCOVERY_CHOICE_ID`, `compute_apg_class_chassis`), purely
additive alongside the existing hand-modelled Feral Mutagen closure (unchanged, still the only
dice-notation record — `resolve_pool_member_sole_magnitude` correctly refuses dice notation since
`formula_interpreter.rs` does not parse it, so no collision is possible).

Real, NEW closures reaching `compute_pilot_base_chassis -> compute_class_chassis` this cycle:

| Corpus key | Target var | Formula | Shape |
|---|---|---|---|
| `Discovery ~ Spontaneous Healing` | `SpontaneousHealingAmount` | `floor(AlchemistDiscoveryLVL/2)*5` | level-scaled (floor-division) |
| `Discovery ~ Healing Touch` | `SpontaneousHealingAmount` | `floor((AlchemistDiscoveryLVL+1)/2)*5` | level-scaled (floor-division) |
| `Discovery ~ Tumor Familiar` | `FamiliarMasterLVL` | `AlchemistDiscoveryLVL` | level-scaled (linear, pool-header pass-through) |

A Python simulation of the exact algorithm against the live `Discovery ~ *` corpus (101 member
records carrying a real description) found 3 close cleanly, 90 carry no `BONUS:VAR`/`DEFINE` token
at all (text-only, already served by `class_feature_pool_catalog.rs`'s cycle-1 widening or genuinely
un-magnitude-bearing), 4 correctly refuse for a multi-terminal target (`Cognatogen`, `Grand
Mutagen`, `Greater Mutagen`, `True Mutagen` — each defines more than one independent magnitude),
and 4 correctly refuse for an unresolvable identifier (`Bottled Ooze`, `Confusion Bomb`, `Plague
Bomb`, `Wings` — all reference `charbonusto(...)`, a real PCGen function `formula_interpreter.rs`
does not implement, named in that module's own "not covered" list). No fabrication anywhere in
this set — every refusal is a real grammar/shape gap, not a guess.

## 4. Tests, RED->GREEN at BOTH altitudes (§1a, per dispatch)

9 new tests, `rules_core::pilot_compute::spellcasting_shaped_class_closure_tests`:

```bash
cargo test --locked --lib -- rules_core::pilot_compute::spellcasting_shaped_class_closure_tests
```
```
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 2720 filtered out
```

- `spontaneous_healing_discovery_resolves_generically_through_the_pool_header_chain` — level 4 ->
  10, level 10 -> 25, proven through the real `compute_pilot_base_chassis`.
- `the_generic_resolver_stays_silent_below_the_pools_own_grant_level` — a real selection at level 1
  (below `AlchemistDiscoveryLVL/2`'s own gate) grounds nothing.
- `an_invented_discovery_selection_never_grounds_a_generic_magnitude` — mutation-proof, mirrors the
  Rage Power precedent.
- `true_mutagen_discovery_refuses_a_multi_terminal_target_rather_than_guess`,
  `force_bomb_discovery_refuses_a_pre_gated_multi_row_target` — the two safety-gate proofs.
- `the_headless_pilot_receipt_carries_the_generic_discovery_magnitude` — reachability proof through
  the real `build_pilot_headless_receipt` entry point (the desktop app's own reach-gate calls the
  same function).

**Mutation altitude 1 (chassis call site):** temporarily wrapped the `push_generic_pool_choice_
magnitude(...)` call at the Alchemist chassis site in `if false { ... }` -> re-ran
`spontaneous_healing_discovery_resolves_generically_through_the_pool_header_chain` ->
```
thread '...' panicked: assertion `left == right` failed
  left: None
 right: Some(10)
```
RED confirmed. Reverted (`if false { ... }` removed, call site restored verbatim); re-ran the full
15-test suite -> green again.

**Mutation altitude 2 (library logic):** temporarily disabled the multi-terminal refusal
(`if false && terminals.next().is_some()`) -> re-ran `true_mutagen_discovery_refuses_a_multi_
terminal_target_rather_than_guess` -> FAILED, with the panic output showing the mutated build had
started emitting `class_feature.apg.alchemist.discovery.generic.true_mutagen.mutagenacbonus` value
`2` — a real, live demonstration of exactly the fabrication `§1a` exists to prevent. Reverted;
re-ran the full suite -> green again.

Both mutations reverted before commit; `git status --porcelain -- src/rules_core/pilot_compute`
shows only the intended two files touched throughout.

**Regression check**, the two touched files' full existing suites plus the shared Alchemist
dispatch suite:
```bash
cargo test --locked --lib -- rules_core::pilot_compute::spellcasting_shaped_class_closure_tests \
  class_feature_pool_catalog class_feature_grant_consumer
```
```
test result: ok. 66 passed; 0 failed; 0 ignored; 0 measured; 2669 filtered out
```
```bash
cargo test --locked --lib -- alchemist
```
```
test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 2705 filtered out
```
No pre-existing test's behaviour changed.

## 5. A real finding that narrows future cycles' work

Investigating Witch Hex (`CLASS_FEATURE_POOLS`'s `("Hex", "witch", "choice:witch_hex", "hex:")`,
real corpus group name `"Witch Hex"`) to wire it the same way found that **51 of the 53 base hex
records already ground through a PRE-EXISTING, hand-written shared-DC function
(`witch_hex_save_dc`, `pilot_compute/mod.rs`, unconditionally pushed for every Witch regardless of
which hex is selected)** — each of those 51 records' own `BONUS:VAR` is only a per-hex ALIAS of
that one shared variable (confirmed by the existing code's own comment: "ONE formula covers every
hex"). This mechanism was never registered in `census_class_feature_pool_population.py`'s
`ALREADY_MODELED_ELSEWHERE` set (which currently only names Domain Power, Inquisitor Domain, Rogue
Talent, Rage Power), so the census's 5,981 residual likely OVER-counts genuinely-unclosed work by
some amount already covered by a pattern like this elsewhere in `pilot_compute/mod.rs`. This
cycle did not chase down every such pre-existing shared-formula function (that is itself a real,
scoped sub-task for a future cycle: audit every `fn ..._save_dc`/`fn ..._dc`/shared-magnitude
function in `pilot_compute/mod.rs` against the census's group list before assuming a group's whole
count needs new work) — reported here as a real, load-bearing discovery-forward rather than
silently left for someone else to re-find.

Separately: `CLASS_FEATURE_POOLS`'s registered group name for Shaman (`"Spirit"`) does not match
any real corpus `" ~ "` prefix (`grep` for `"Spirit ~ "` in `data/corpus/*/class_feature/` returns
zero hits) — `v06_work_inventory.rs`'s own `class_feature_pool_group_matches` uses a fuzzy suffix
match, not this census's exact-prefix parsing, so the two disagree on what "the Spirit group" even
is. Wiring Shaman (and likely several other `CLASS_FEATURE_POOLS` entries) needs the real corpus
group name re-derived first, not assumed from the registry's own display name.

## 6. Sweep (§3)

```bash
grep -rn "resolve_pool_member_sole_magnitude\|resolve_pool_selection_corpus_key\|push_generic_pool_choice_magnitude\|class_feature_record_tokens_pre_gate_safe\|class_feature_bonus_vars_any_record" tests/ src/ scripts/ apps/ 2>/dev/null | grep -v "pilot_compute/mod.rs\|class_feature_grant_consumer.rs"
```
No hit outside the two files this cycle touched — no other file's pinned assertion references
these new names.
```bash
grep -rn "5,981\|5981\b" docs/release/SD-32-compute-library-and-cause-closure/*.md tests/ src/ scripts/ apps/ 2>/dev/null
```
Only this cycle's own kanban addendum and this receipt cite the figure — no other file has a
stale/competing count to reconcile.

## 7. Scope discipline

Did not attempt: wiring the other 12 `CLASS_FEATURE_POOLS`-registered pools (named as next-cycle
plan above); the `cache_gen::class_feature::generate` `[not implemented]`-marker root cause named
by cycle 1's own discovery-forward (still unaddressed, out of this cycle's file territory); rows
11/15 (left `in-progress`, untouched); `tests/sd26_cache_core_rulebook.rs`,
`tests/pi_screening_regeneration_round_trip.rs`, or the two pre-existing reds named in the dispatch
(`e14_harness_tests::a_key_two_books_share_grounds_only_the_book_whose_corpus_was_read`,
`race_trait_grounding_tests::the_t2b_residual_population_is_never_ingested_not_a_matcher_miss`) —
sibling unowned-reds lane, not reproduced or touched this cycle; `apps/desktop`'s
`equipment_catalog`, `forward-scope-register.md`, `declared_pi_shipping_audit` — sibling
follow-ups lane; row 17's provisional-default categorization — sibling lane. `data/corpus/**`
untouched throughout.

`df -h /`: reported in the dispatch's final report.
