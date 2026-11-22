# Cycle t12-class-feature-pool-population, cycle 5 — Gate 3 (closure invariant) / row 18 (`epic-8-pool-shaped-class-features`)

- **Card ID:** row 18 (`epic-8-pool-shaped-class-features`), governed by `decisions.md §27b`
  (*"EVERYTHING"*). Dispatched scope: fan out cycle 4's name-resolved pools at scale using the
  existing generic resolver — Bloodline 391 · Domain 310 · Mystery 234 · Blessing 111 · Spirit 73,
  largest first, whole pools not fragments.
- **Base:** worktree started on a STALE lineage (footgun 1 fired again — `git merge-base
  --is-ancestor "$PIN" HEAD` returned false at session start, `PIN=f461e742f3`). Fixed:
  `git reset --hard "$PIN"` then `git rebase origin/tranche/12` (fast-forward — `origin/tranche/12`
  == `$PIN`, cycle 4's own commit). `BASE_OK` re-verified after.
- **Oracle:** fresh worktree, empty git-ignored slot as expected. Bootstrapped via
  `scripts/fetch-pcgen-oracle.sh --dest <repo-local artifacts/corpus/operator-supplied/pcgen>`;
  `scripts/verify.sh --only preflight-oracle` → `PASS` (pin `7f818006e371`).
- **Files touched:**
  - `src/rules_core/pilot_compute/mod.rs` — new `push_generic_pool_group_selection_magnitude` +
    `real_pool_group_for_selection_slug` (the "select ONE group, inherit every member" pool shape);
    widened `pool_header_record_by_normalized_suffix` with a second real header-key shape (bare
    group name, class-checked); wired 4 call sites (Sorcerer Bloodline, Cleric Domain, Warpriest
    Blessing, Shaman Spirit); wired and then DELIBERATELY WITHDRAWN a 5th (Oracle Mystery, see §3);
    8 new tests (`generic_pool_group_selection_wiring_tests`).
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 18 only (cycle field
    3 → 4, Notes appended). Row 19 (line 57 pre-edit line count) and row 11 left untouched and
    still parse (verified: exactly one `^| 18 |` row, row 19's own line unchanged, `git diff --stat`
    shows a single 1-line change to this file).
  - `docs/retro/events/t9-onboarding.jsonl` — auto-appended by `scripts/verify.sh` (2 derived
    verification events, oracle-bootstrap FAIL then PASS). Not hand-edited.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — `git diff --unified=0 --
  src/rules_core/pilot_compute/mod.rs`, `grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'`
  → 0 hits outside the literal `SD-32`/`SD-31` doc-comment citations already present in the
  surrounding file.
- **Wired-integration audit result:** `OK_NO_TOKENS` — same scope, `grep -inE
  "STUB|MOCK|placeholder|not.?yet.?implemented|todo|fixme|hack"` → 0 hits.
- **PI audit:** `pi_scrub.normalized_term_hits(...)` against this cycle's full diff text → `[]`
  (0 hits). No corpus record name, blacklist term, or PI item name in any receipt, test name, test
  constant, kanban row, or this file (`§24b`-2). `data/corpus/**` untouched throughout
  (`git status --porcelain -- data/corpus` — 0 changes).
- **Status:** in-progress — genuine, multi-cycle epic, unchanged from cycles 1–4's own framing.

---

## 1. The new mechanism — "select ONE group, inherit every member" (`§17`, one mechanism)

Cycle 2's `push_generic_pool_choice_magnitude` serves pools shaped as "player picks N individual
members from one flat list" (Discovery, Hex, Slayer Talent — the recorded `selection_id` names the
member itself, e.g. `"discovery:feral_mutagen"`). Bloodline/Domain/Mystery/Blessing/Spirit are a
DIFFERENT shape: the player picks exactly ONE named group (a bloodline, a domain, a mystery, a
blessing, a spirit), and PF1's own rules automatically grant every one of that group's own powers —
no per-power sub-choice exists for Domain/Bloodline/Blessing/Spirit powers (confirmed: this file's
own pre-existing hand-modelled Cleric/Sorcerer branches never require a second "which power" pick;
Oracle Mystery is the one exception, see §3).

`push_generic_pool_group_selection_magnitude` (new) and its helper
`real_pool_group_for_selection_slug` (new) implement this: given a recorded `choice_set_id ->
selection_id` (e.g. `choice:cleric_domain -> domain:plant`), strip the namespace to get the bare
slug (`"plant"`), find the real corpus group whose OWNER is majority-`class`-owned and whose own
trailing word-boundary/plural-normalized adjective slugs to the same value (`"Plant Domain"`) — the
SAME suffix rule `pool_header_record_by_normalized_suffix` and cycle 4's census script already use,
reused rather than reinvented — then resolve EVERY one of that group's own `"<group> ~ <member>"`
records through the unchanged `resolve_pool_member_sole_magnitude`. A member this resolver cannot
ground is silently skipped, never fabricated, exactly like the flat-pool shape.

## 2. The header-key widening (`§17`, same function widened, not a new one)

Direct corpus inspection (not assumed) found Domain/Bloodline/Mystery/Blessing/Spirit header records
use a THIRD real key shape beyond the two `pool_header_record_by_normalized_suffix` already tried:
the bare group name itself, with no `"<class> ~ "` prefix at all —

```
data/corpus/core_rulebook/class_feature/air/air.json      KEY:"Air Domain"        class:"Cleric"
data/corpus/advanced_class_guide/class_feature/aberrant_bloodline/aberrant_bloodline.json
                                                            KEY:"Aberrant Bloodline" class:"Sorcerer"
```

Added as a third, class-checked fallback (never trusted from the bare key alone, so a same-named
group owned by a different class can never be picked up by mistake) — tried last, changing nothing
for Slayer/Alchemist/Witch's own already-exact or already-suffix-matched lookups (proven by the full
regression run below, all unchanged).

## 3. Wired at 4 of the 5 named pools; Oracle Mystery caught its own live regression and was withdrawn

Wired, purely additive, alongside every pre-existing hand-modelled branch:

- **Sorcerer Bloodline** (`explain_sorcerer_level1_spell_baseline`) — `SORCERER_BLOODLINE_CHOICE_ID`,
  registered name `"Bloodline"`, namespace `"bloodline:"`.
- **Cleric Domain** (`explain_cleric_level1_spell_baseline`) — `CLERIC_DOMAIN_CHOICE_ID`, registered
  name `"Domain"`, namespace `"domain:"`.
- **Warpriest Blessing** (`ground_or_block_warpriest_class_features`) — `WARPRIEST_BLESSING_CHOICE_ID`,
  registered name `"Blessing"`, namespace `"blessing:"`.
- **Shaman Spirit** (`ground_or_block_shaman_class_features`) — `SHAMAN_SPIRIT_CHOICE_ID`, registered
  name `"Spirit"`, namespace `"spirit:"`.

**Oracle Mystery was wired identically, then DELIBERATELY WITHDRAWN** (`§1a`: never weaken a
regression to force a closure). Wiring it live tripped this file's own pre-existing safety test,
`oracle_dispatch_widening_safety_tests::a_mystery_pick_alone_grounds_no_tier_one_revelation`: a bare
`mystery:lore` selection with NO explicit `ORACLE_REVELATION_CHOICE_ID` pick started grounding
`Lore Mystery ~ Sidestep Secret` anyway. Root cause: unlike Domain/Bloodline/Blessing/Spirit powers
(all automatically granted), most Mystery members are REVELATIONS — a genuinely budgeted PF1
sub-choice this codebase already, correctly, gates on a second explicit pick
(`oracle_level_with_revelation`'s own doc comment explains why the Mystery-only gate would describe
an illegal character). The "select group, inherit everything" shape is factually WRONG for this one
pool. Reverted the call site rather than the test; the withdrawal itself is proven by a new test
(`oracle_generic_mystery_pass_is_deliberately_not_wired`) asserting zero generic explanations for
both a never-hand-modelled mystery (Ancestor) and an already-hand-modelled one (Lore). This is a
genuine, named "cannot be wired without a budgeted-choice-aware mechanism" finding for the remaining
11 Mystery groups, not a silent skip — the 10 pre-existing hand-modelled mysteries are unaffected and
unchanged.

## 4. What actually closes — a direct corpus survey, not the name-resolution count alone

Cycle 4 named 391/310/111/73 REAL records now name-resolved for Bloodline/Domain/Blessing/Spirit.
Not all of them are runtime-resolvable yet: most level-scaled PF1 CRB/APG-era powers chain through
either (a) a `classlevel("X")` call this resolver's own documented cross-class-safety gap
(`formula_interpreter.rs`) deliberately refuses to bank through, or (b) a per-group header chain
variable (e.g. `Sorcerer_Aberrant_BloodlinePower1LVL`) whose OWN corpus header record carries
multi-row PRE-gated `BONUS:VAR` tokens `parse_bonus_var_tokens_pre_gate_safe` correctly drops rather
than guess the wrong PRE-gated variant. A direct, live corpus survey (script below, not assumed) of
every real group in each wired pool:

```bash
python3 - <<'PY'
# scans every "<group> ~ <member>" class_feature record majority-owned by the target class,
# applies the SAME pre-gate-safe single-row-no-tail rule the Rust resolver uses, and reports
# which groups carry at least one member resolvable WITHOUT the missing header chain.
PY
```

| Pool | Groups surveyed | Groups with ≥1 directly-resolvable member | Example |
|---|---:|---:|---|
| Sorcerer Bloodline | 53 | **15** | Celestial `Ascension` = 10 |
| Cleric Domain (67 unmodelled) | 67 | **5** | Plant `Wooden Fist` = 3+WIS |
| Warpriest Blessing (36 unmodelled) | 36 | **0** | — |
| Shaman Spirit (4 unmodelled) | 4 | **0** | — |

**Bloodline and Domain genuinely close new records** — proven live below. **Blessing and Spirit
genuinely close nothing new right now** — proven as a SAFE, unweakened refusal (not a bug, not a
silent miss), the honest "cannot be wired without extending the resolver itself" case `§27b` asks to
name rather than force. Extending the resolver to bank through `classlevel(...)` safely (resolving
its cross-class argument) or to parse PRE-gated multi-row headers correctly are both real, scoped,
larger mechanism widenings — out of THIS cycle's `§17` "use the existing resolver" scope, named as
follow-on rather than attempted half-built.

## 5. Tests, RED→GREEN, both altitudes (`§1a`)

`generic_pool_group_selection_wiring_tests` (8 tests):

- `sorcerer_generic_bloodline_pass_grounds_a_never_hand_modelled_bloodline` — Celestial Bloodline
  (never hand-modelled; only Arcane/Draconic are) grounds ≥1 generic explanation.
- `cleric_generic_domain_pass_grounds_a_never_hand_modelled_domain` — Plant Domain (not in
  `DOMAIN_POWER_CATALOG`) grounds ≥1.
- `oracle_generic_mystery_pass_is_deliberately_not_wired` — proves the withdrawal (§3), 0 on both
  Ancestor and Lore.
- `warpriest_generic_blessing_pass_correctly_refuses_every_unmodelled_blessing` — Air Blessing (a
  real, recognized group) grounds 0, proving safe refusal not non-recognition.
- `shaman_generic_spirit_pass_correctly_refuses_every_unmodelled_spirit` — Wood Spirit, same proof.
- `invented_selections_ground_nothing_on_any_wired_pool` — an invented selection on each of the 4
  wired pools grounds 0.

**Mutation altitude 2 (library logic):** `real_pool_group_for_selection_slug` forced `if true {
return None; }` at its top → re-ran the 6 non-Oracle tests:

```
3 failed (sorcerer_generic_bloodline_pass, cleric_generic_domain_pass,
          [oracle_generic_mystery_pass_grounds — this test's ORIGINAL, pre-withdrawal form, run
           before the Oracle withdrawal was applied])
3 passed (the two "correctly refuses" tests were already asserting 0; invented-selection test
          unaffected)
```

RED confirmed (the 3 positive-closure assertions fail exactly as expected). Reverted.

**Mutation altitude 1 (chassis call site):** Sorcerer's own
`push_generic_pool_group_selection_magnitude(...)` call wrapped in `if false { ... }` → re-ran:

```
1 failed: sorcerer_generic_bloodline_pass_grounds_a_never_hand_modelled_bloodline
5 passed: every other test unaffected (proves the call sites are independent — disabling
          Sorcerer's changes nothing for Cleric/Warpriest/Shaman)
```

RED confirmed, isolated to exactly the mutated call site. Reverted; call site restored verbatim.

**Regression check**, scoped (not the repo-wide suite):

```bash
cargo test --locked --lib -- rules_core::pilot_compute::generic_pool_group_selection_wiring_tests
```
```
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 2752 filtered out
```
```bash
cargo test --locked --lib -- \
  rules_core::pilot_compute::opponent_conditioned_tier_zero_tests::slayer \
  rules_core::pilot_compute::slayer_dispatch_widening_safety_tests \
  rules_core::pilot_compute::spellcasting_shaped_class_closure_tests \
  class_feature_pool_catalog class_feature_grant_consumer \
  rules_core::pilot_compute::sorcerer_draconic_bloodline_dragon_resistances_ac_wiring_tests \
  rules_core::pilot_compute::apg_canonical_choice_path_a_tests \
  rules_core::pilot_compute::shaman_dispatch_widening_safety_tests
```
```
test result: ok. 106 passed; 0 failed; 0 ignored; 0 measured; 2652 filtered out
```
```bash
cargo test --locked --lib -- rules_core::pilot_compute::
```
```
test result: ok. 920 passed; 0 failed; 0 ignored; 0 measured; 1838 filtered out
```

920/920 (up from cycle 4's 914/914: +6 net new tests, 0 broken). The Oracle regression this cycle's
own Mystery wiring introduced was found and fixed by withdrawal BEFORE this commit — it was never
merged red, per the same "never end your turn on red" and "prove RED→GREEN, then revert" discipline
every prior cycle followed.

## 6. Sweep (`§3`) and residual re-derivation (`§17a`)

```bash
grep -rn "5,981\|5981\b\|5,927\|5927\b" docs/release/SD-32-compute-library-and-cause-closure/*.md tests/ src/ scripts/ apps/
```
All hits are prior cycles' own historical citations (kanban text, cycle 3/4's own receipts and
doc-comments), unchanged.

```bash
python3 scripts/census_class_feature_pool_population.py
```
```
RESIDUAL numeric-magnitude needing compute       5927
```
**Unchanged from cycle 4's own re-derivation, and this is correct, not a miss.** This census
measures the STATIC catalog population still needing a real compute function to exist at all; this
cycle's work is RUNTIME reachability for a SPECIFIC character's recorded selection through the
already-existing generic resolver — the exact `§16` distinction cycle 4's own 3-Slayer-member closure
already drew ("moves units from refused to resolved WITHIN the already-measured 5,927 residual; it
does not change the census count"). No new numeric-magnitude residual figure is introduced.

## 7. Scope discipline

**Did not attempt**, real scoped follow-on, named rather than silently deferred:

- **Hunter Animal Focus** (21 real records, exact match) — activation-gated, needs careful
  activation-state integration; cycle 3 and cycle 4 both flagged this and it remains untouched.
- **Cavalier Order** — confirmed again (by this cycle's own group survey finding no
  `" ~ "`-qualified `"<X> Order"` suffix shape among Cavalier's per-order groups) to need the
  two-level dispatcher walk cycle 3 first found; not attempted.
- **Bloodrager Bloodline** (110 records) — the SAME mechanism this cycle built would wire it
  identically to Sorcerer's Bloodline (both use `push_generic_pool_group_selection_magnitude` with
  registered name `"Bloodline"`); not wired at its own `BLOODRAGER_BLOODLINE_CHOICE_ID` call site
  this cycle, purely a time-boxing choice, not a mechanism gap.
- **The 38/53 Bloodline, 62/67 Domain, 36/36 Blessing and 4/4 Spirit groups this cycle's own survey
  found have no directly-resolvable member** — each needs either the missing PRE-gated
  multi-row-header parsing or the documented `classlevel(...)` cross-class-argument widening
  (`formula_interpreter.rs`'s own module doc names this as a real, currently-unfixed gap) to close.
  Both are real, scoped, larger mechanism extensions — out of this cycle's `§17` "use the existing
  resolver, do not build a third mechanism" scope, escalated here by name per `§27b`'s own
  instruction ("a cycle that believes it has found a genuine impossibility escalates it by
  coordinate ... it does not write its own exemption").
- Rows 11/15 (left `in-progress`, untouched); `apps/desktop`'s row 19 lane, `bestiary_4/
  monster_ability`'s lane — both sibling territory, not touched. `data/corpus/**` untouched
  throughout.

`df -h /`: reported in the dispatch's final report.
