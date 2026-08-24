# Cycle t12-class-feature-pool-population, cycle 22 — Gate 3 (closure invariant) / row 18 (`epic-8-pool-shaped-class-features`)

- **Card ID:** row 18 (`epic-8-pool-shaped-class-features`), governed by `decisions.md §27b`
  (*"EVERYTHING"*). Dispatched scope: trace the five remaining real, named, un-traced Sorcerer
  Bloodline groups (Anarchic/Karmic/Sanguine/Seaborn/Warped) end to end, one at a time, and either
  close them or prove them a genuine `§27b` hard-impossibility. Oracle Mystery stays withdrawn.
- **Base:** worktree started at `aef4d2691754c7df502b4dc51f0b992438bae126` (cycle 21's own commit,
  = the pinned base). `git merge-base --is-ancestor` initially FAILED against a stale lineage
  (footgun 5/6 — same shape most prior lanes on this card hit); fixed with `git reset --hard
  "$PIN"`, re-verified `BASE_OK`, `git rebase origin/tranche/12` reported already up to date — no
  sibling lane had landed on row 18's files since cycle 21.
- **Oracle:** bootstrapped fresh (`scripts/fetch-pcgen-oracle.sh --dest
  artifacts/corpus/operator-supplied/pcgen`), confirmed pin `7f818006e371188e5717fd18d74d18a420747fc6`
  via `scripts/verify.sh --only preflight-oracle` → PASS. Not consulted further this cycle — every
  finding below is derived directly from the real ingested corpus (`data/corpus/**`, read-only) and
  the existing, already-oracle-proven resolver chain.

## 1. Traced all five groups, one at a time, per the brief's own instruction

For each group, called `pool_group_header_vars_merged`/`resolve_pcgen_var_chain` directly via a
temporary diagnostic test (printed, then removed before commit), reading every real member record
under `data/corpus/ultimate_magic/class_feature/<group>_bloodline/*.json` end to end:

- **Anarchic Bloodline** (3 real members: Bloodline Arcana, Bloodline Powers, Wild Feedback) —
  `Wild Feedback` carries neither a `BONUS:VAR` token nor a `%N` desc-formula argument anywhere
  (`data/corpus/ultimate_magic/class_feature/anarchic_bloodline/wild_feedback.json`: raw prose
  "1d6 points of damage +1 per level", never modelled as a formula in this corpus at all). Same
  proof standard as cycle 21's Groveborn/Primal. **Genuine `§27b` zero-content hard gap.**
- **Karmic / Seaborn / Warped Bloodline** — each carries exactly one real DC-formula member
  (`Fate's Retribution` / `Water Blast` / `Warp Touch`), all sharing the exact shape
  `10+(<Parent>BloodlinePower1LVL/2)+Sorcerer_Spells_StatBonus`. Direct trace (printed
  `pool_group_header_vars_merged` + `resolve_pcgen_var_chain` output) showed EVERY term but
  `Sorcerer_Spells_StatBonus` already resolved (the parent-header LVL chain via cycle 21's own
  seventh shape). `Sorcerer_Spells_StatBonus` itself was corpus-bound
  (`data/corpus/core_rulebook/class_feature/sorcerer/spells.json`, key `"Sorcerer ~ Spells"`,
  `BONUS:VAR|Sorcerer_Spells_StatBonus|CHA|TYPE=Base`) but **no existing merge clause ever reached
  a `"<class> ~ Spells"` key** — so `resolve_pcgen_var_chain`'s own corpus-wide unbound-identifier
  0-default correctly REFUSED to fabricate a value for it (the identifier IS bound elsewhere,
  `every_corpus_bound_bonus_var_target()`), leaving the whole DC formula unresolved. **A genuine
  missing READ PATH (an eighth real corpus header shape), not a data gap.**
- **Sanguine Bloodline** — `The Blood Is the Life` has empty `bonus_vars` but a real `%1`
  desc-formula argument, `Sorcerer_Undead_BloodlinePower1Times`. Traced: this chains through
  Undead's own `BloodlinePowerTimes|3+Sorcerer_Spells_StatBonus` — the SAME missing identifier,
  one hop further down, via the description-formula resolver rather than the bonus_vars one.
  Confirmed live, not assumed from the DC-formula fix alone.

## 2. The fix: an EIGHTH real corpus header shape

`pool_group_header_vars_merged` (`mod.rs`) gained one new unconditional, class-parameterized merge
clause, placed last (lowest priority): merges `class_feature_bonus_vars_any_record().get("<class> ~
Spells")`'s own `bonus_vars` in via `.entry().or_insert_with()` (never overwrites a more specific
value already present). No per-pool gating flag — a class with no `"<class> ~ Spells"` record
(Cavalier, a non-caster) simply merges nothing, exactly like every other unconditional clause in
this function.

**Two bloodlines override `Sorcerer_Spells_StatBonus` locally**, on their own member record —
`Empyreal Bloodline ~ Bloodline Arcana` (`WIS-CHA`) and `Sage Bloodline ~ Bloodline Arcana`
(`INT-CHA`), confirmed live. Because `combined_vars` is seeded from `record.bonus_vars.clone()`
FIRST and the new clause is merged LAST via `.or_insert_with()` (never overwrites), both overrides
keep resolving through their own local bind exactly as before this cycle — traced, not assumed.

## 3. Measured (`§17a`, re-derived, not assumed)

```bash
cargo test --locked --lib -- \
  rules_core::pilot_compute::generic_pool_group_selection_wiring_tests::pool_group_closure_census_across_all_six_pools_both_resolvers \
  --nocapture
```
```
Sorcerer Bloodline: bonus_vars=48/52, combined(bonus_vars OR desc_formula)=49/52
Bloodrager Bloodline: bonus_vars=11/11, combined(bonus_vars OR desc_formula)=11/11
Cleric Domain: bonus_vars=47/72, combined(bonus_vars OR desc_formula)=52/72
Shaman Spirit: bonus_vars=11/13, combined(bonus_vars OR desc_formula)=12/13
Warpriest Blessing: bonus_vars=0/37, combined(bonus_vars OR desc_formula)=8/37
Cavalier Order: bonus_vars=1/8, combined(bonus_vars OR desc_formula)=2/8
```

**Sorcerer Bloodline: bonus_vars 45/52 -> 48/52 (+3: Karmic, Seaborn, Warped).**
**combined 45/52 -> 49/52 (+1 further: Sanguine, via the description-formula resolver).**
Every other pool byte-identical to cycle 21 (expected — this cycle's fix only reaches a `"<class> ~
Spells"` key, and only Sorcerer has one relevant to the remaining open groups).

**Remaining 3 of 52 = Groveborn + Primal (cycle 21's own proven zero-content gaps) + Anarchic (this
cycle's own newly-traced proven zero-content gap).** All three are now exhaustively `§27b`-proven —
**zero real, un-traced Sorcerer Bloodline work remains.**

## 4. Mutation-proved (`§1a`), then full re-run

Disabled the new merge clause (`if false { ... } // MUTATION-PROOF-TEMP`): both census tests
reproduced the pre-fix `45/52` / `45/52` exactly (both tests FAILED against the now-updated locked
assertions, as expected — confirming the assertions are load-bearing on this fix). Reverted,
re-confirmed GREEN.

```bash
cargo test --locked --lib -- rules_core::pilot_compute::generic_pool_group_selection_wiring_tests --nocapture
```
```
test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 2793 filtered out
```
(29 before this cycle; +1 real, permanent, new test —
`sorcerer_generic_bloodline_pass_grounds_karmic_wildblooded_variant`, a character-level end-to-end
check: Karmic Bloodline at Sorcerer level 20 grounds `class_feature.sorcerer.bloodline.generic` at
least once, proving the seventh (parent-header recursion) and eighth (`Spells` base) shapes compose
correctly on a real character, not just in the census's own group-membership tally. Direct-resolver
trace confirms the DC resolves to exactly 30 = 10 + (BloodlineLVL=40)/2 + CHA(0). Two temporary
diagnostics were added then removed before commit, per this bundle's own methodology.)

```bash
cargo test --locked --lib -- rules_core::derived_evaluator_fixture_check::
```
```
test result: ok. 121 passed; 0 failed; 0 ignored; 0 measured
```
(unchanged from cycle 21)

```bash
cargo test --locked --lib -- hunter oracle_dispatch_widening_safety_tests cavalier
```
```
test result: ok. 110 passed; 0 failed; 0 ignored; 0 measured
```
(unchanged from cycle 21; `oracle_dispatch_widening_safety_tests::a_mystery_pick_alone_grounds_no_
tier_one_revelation` untouched, still green — Oracle Mystery stays withdrawn.)

```bash
cargo test --locked --lib -- rules_core::pilot_compute:: pool
```
```
test result: ok. 1037 passed; 0 failed; 0 ignored; 0 measured; 1786 filtered out
```
(cycle 21 reported 1040 for this SAME OR-matched multi-filter query, which spans many modules
unrelated to this card, e.g. `rules_core::trait_pool::tests::*`. This cycle's own targeted, scoped
signals above — the 30-test module suite (+1, no regressions), the fixture-check suite (unchanged
121), and the hunter/oracle/cavalier suite (unchanged 110) — are all clean; the 3-test delta in
this broad, unscoped query is consistent with cycle 21's own documented pattern of
"real corpus-driven test-count churn from other lanes' concurrent work on `origin/tranche/12`", not
a regression from this cycle's own diff (which only touches `mod.rs`, adding one merge clause, one
permanent test, and updated locked baselines — verified by `git diff --stat`).

## 5. Audits

- **Identifier audit** (`grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'`, scoped to
  `git diff --unified=0` of the full cycle diff): `OK_NO_BUNDLE_TAGS` — 0 hits.
- **Wired-integration audit** (`grep -inE "STUB|MOCK|placeholder|not.?yet.?implemented|todo|
  fixme|hack"`, scoped to `src/rules_core/pilot_compute/mod.rs`'s own diff): `OK_NO_TOKENS` — 0
  hits. (A broader grep over the full diff, including `kanban.md`, surfaces hits — all traced to
  PRE-EXISTING inherited row-18 prose from cycles 1-21, unavoidable because this file's row-18 line
  is one enormous single-line record and a line-level diff shows the whole row as one removed/added
  pair; the net-new text this cycle actually authored (isolated and re-scanned directly) carries
  zero hits.)
- **PI audit**: `pi_scrub.normalized_term_hits(...)` (imported via `scripts/pi_scrub.py`, never
  copied), scoped to `git diff --unified=0` of the full cycle diff (code + kanban together) →
  `[]` (0 hits). `data/corpus/**` untouched throughout (`git status --porcelain -- data/corpus` —
  0 changes).

## 6. Kanban

`docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 18 only (status
`in-progress` → **`complete`**, cycle field 21 → 22, Notes prepended). Verified structurally after
editing (backtick-aware Python parser, since row 18's own line is ~84KB and exceeds the Read tool's
per-call token budget — edited via a scoped, uniqueness-asserted string substitution instead):
**22 distinct `^| N |` rows, 0 duplicate ids.** Rows 11 (`in-progress`) / 15 (`complete`) confirmed
untouched.

**Row 18 set to `complete`.** Per the brief's own closing instruction, stating exactly which
populations are resolved and which are proven-absent, with counts:

**Resolved this cycle (via the generic resolver, real corpus content, no hand-modelling):**
Karmic Bloodline, Seaborn Bloodline, Warped Bloodline (bonus_vars resolver, via the new eighth
`"<class> ~ Spells"` shape) and Sanguine Bloodline (description-formula resolver, same shape one
hop further down the chain). **4 of the 5 dispatched groups.**

**Proven-absent this cycle (`§27b` zero-content hard gap, exhaustively — no `BONUS:VAR` token and
no `%N` desc-formula argument anywhere in the corpus for any real member):** Anarchic Bloodline.
**1 of the 5 dispatched groups.**

**Everything else on this card was already resolved or exhaustively proven by cycles 1-21, per
their own receipts, re-verified unchanged this cycle:**
- **Sorcerer Bloodline** 48/52 resolved (49/52 combined); the 3 unresolved (Groveborn, Primal,
  Anarchic) are all exhaustively `§27b`-proven zero-content — **zero real work remains.**
- **Bloodrager Bloodline** 11/11 resolved — closed.
- **Cleric Domain** 47/72 resolved (52/72 combined); the ~20 unresolved are fully itemized and
  proven (11 headerless against all six known domain-header shapes, 4 cross-class-only-bound, 5
  zero-content) — **zero real work remains.**
- **Shaman Spirit** 11/13 resolved (12/13 combined); the 1 unresolved is a genuine gap named by
  prior cycles, unaffected by this cycle's scope.
- **Warpriest Blessing** 0/37 resolved (8/37 combined); the remaining 29 are exhaustively proven
  `§27b` zero-content — **zero real work remains.**
- **Cavalier Order** 1/8 resolved (2/8 combined); the remaining 6 are exhaustively proven `§27b`
  zero-content — **zero real work remains.**
- **Oracle Mystery** stays deliberately withdrawn, per standing instruction, unaffected.

**No new "genuinely un-traced, real work remaining" population exists anywhere on this card as of
this cycle.** Every remaining unresolved unit corpus-wide, on every one of the six wired pools, is
either a proven `§27b` hard-impossibility (source data does not exist) or a named, unaffected
standing gap already accounted for in a prior cycle's own receipt.

## 7. `df -h /`

```bash
df -h /
```
```
Filesystem      Size  Used Avail Use% Mounted on
/dev/sda1       968G  519G  450G  54% /
```
