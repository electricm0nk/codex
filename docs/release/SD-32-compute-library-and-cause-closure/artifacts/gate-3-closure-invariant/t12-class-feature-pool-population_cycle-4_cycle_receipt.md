# Cycle t12-class-feature-pool-population, cycle 4 — Gate 3 (closure invariant) / row 18 (`epic-8-pool-shaped-class-features`)

- **Card ID:** row 18 (`epic-8-pool-shaped-class-features`), governed by `decisions.md §27b`
  (*"EVERYTHING"*). Scope this cycle, per dispatch: **Deliverable 1 is making group-name
  resolution generic** — cycle 3 found most registered pool names in
  `src/bin/v06_work_inventory.rs::CLASS_FEATURE_POOLS` match zero or near-zero real corpus
  records; fix the resolution mechanism (one mechanism, not a per-pool table), report the real
  name/count per pool, then fan out with cycle 2's resolver now that names resolve.
- **Base:** verified against `PIN=e5a7687e5729c5d2f4bb60a8a35fd1713945352c` — the worktree started
  on a STALE lineage (footgun 1 fired; `git merge-base --is-ancestor` returned false against
  `HEAD` at session start). Fixed: `git reset --hard "$PIN"` then `git rebase origin/tranche/12`
  (fast-forward, no conflicts) -> `HEAD` = `e5a7687e57` (== `origin/tranche/12`, cycle 3's own
  commit). `BASE_OK` re-verified after.
- **Oracle:** fresh worktree, empty git-ignored slot as expected. Bootstrapped via
  `scripts/fetch-pcgen-oracle.sh --dest <repo-local artifacts/corpus/operator-supplied/pcgen>`;
  `scripts/verify.sh --only preflight-oracle` -> `PASS`.
- **Files touched:**
  - `scripts/census_class_feature_pool_group_names.py` — NEW. Deliverable 1's re-derivation tool
    (below).
  - `src/rules_core/pilot_compute/mod.rs` — added `pool_header_record_by_normalized_suffix`
    (generic singular/plural header lookup, replacing the byte-exact
    `format!("{class} ~ {pool_group}")` construction inside `resolve_pool_member_sole_magnitude`);
    replaced one old test (`slayer_generic_resolver_refuses_rather_than_fabricates_a_missing_class_
    level_binding`, whose asserted `None` was the BUG this cycle fixes) with 4 new tests: the two
    corrected closures it proves (Assassinate, Slowing Strike, Hard to Fool) plus one proving the
    separate multi-terminal refusal (Combat Style I) still holds unweakened.
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` — row 18 only (cycle field
    2 -> 3, Notes appended).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` — `git diff --unified=0 --
  src/rules_core/pilot_compute/mod.rs scripts/census_class_feature_pool_group_names.py`, `grep -nE
  '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b'` -> 0 hits (also checked the new untracked
  file directly since `git diff` does not cover it).
- **Wired-integration audit result:** `OK_NO_TOKENS` — same scope, `grep -inE
  "STUB|MOCK|placeholder|not.?yet.?implemented|todo|fixme|hack"` -> 0 hits.
- **PI audit:** `pi_scrub.normalized_term_hits(...)` against this cycle's full diff text plus the
  new script's full contents -> `[]` (0 hits). No corpus record name, blacklist term, or PI item
  name in any receipt, test name, test constant, kanban row, or this file (`§24b`-2).
  `data/corpus/**` untouched throughout (`git status --porcelain -- data/corpus` — 0 changes).
- **Acceptance criterion (this cycle's stated scope):** re-derive real group names generically
  (one mechanism), report per-pool real name + count with command, then close what the resolver
  can now reach.
- **Status:** in-progress — genuine, multi-cycle epic, unchanged from cycles 1-3's own framing.

---

## 1. Deliverable 1 — generic real-group-name derivation (`§17a`)

**The finding cycle 3 left unaddressed:** the registry's display names are not corpus truth, and a
per-pool lookup table to patch them would itself be the "relabelled shape" anti-pattern (`§1a`).
This cycle built ONE mechanism instead — `scripts/census_class_feature_pool_group_names.py` — that
re-derives, for every `(registered_name, owner_class)` pair in `CLASS_FEATURE_POOLS`, the real
corpus group name(s) by:

1. exact match, or
2. a word-boundary suffix match (`group` ends with `" " + registered`), or
3. the SAME suffix rule after stripping one trailing `s` from both sides (singular/plural
   insensitive) — the identical normalization `pool_header_record_by_normalized_suffix` (§2 below)
   uses for the compute-time header lookup, so census and compute share one rule, not two.

Every candidate group is additionally required to be OWNED (by corpus-read majority `data.class`
across its own records, never assumed from text) by the registered pool's own class — this is what
correctly keeps `"Druid Domain"` / `"Inquisitor Domain"` / `"Core Domain"` out of Cleric's
`"Domain"` bucket (cycle 3 named this same guard as load-bearing for `class_feature_pool_group_
matches`, the analogous mechanism already proven in `v06_work_inventory.rs`; this script implements
the same rule independently in Python for census purposes, per this repo's existing
census/compute-are-separate-layers convention).

**Command:** `python3 scripts/census_class_feature_pool_group_names.py`

**Result (live, full table in the script's own `--json` output):**

| Registered name | Owner | Real groups found | Real records |
|---|---|---:|---:|
| `Rage Power` | Barbarian | 1 (exact) | 170 |
| `Unchained Rage Power` | Unchained Barbarian | 1 (exact) | 54 |
| `Discovery` | Alchemist | 3 | 133 |
| `Grand Discovery` | Alchemist | **0 — confirmed no match** | 0 |
| `Rogue Talent` | Rogue | 2 | 134 |
| `Advanced Talents` | Rogue | **0 — confirmed no match** | 0 |
| `Hex` | Witch | 3 | 89 |
| `Revelation` | Oracle | 1 (`Soothsayer Revelation` only — most revelations live inside their own per-mystery group, not a top-level `Revelation`-suffixed group) | 2 |
| `Mercy` | Paladin | 1 (exact) | 15 |
| `Investigator Talent` | Investigator | 1 (exact, but only 2 real members — cycle 3's finding confirmed: the real 118-record pool is named plain `Investigator`, a DIFFERENT owner-class-labeled group this script correctly does not fold in since it is not suffix-shaped) | 2 |
| `Slayer Talent` | Slayer | 1 (exact) | 46 |
| `Judgment` | Inquisitor | 4 | 18 |
| `Inquisition` | Inquisitor | 3 | 28 |
| `Blessing` | Warpriest | **38** (1 exact-normalized `Blessings` + 37 per-domain `<X> Blessing`) | **111** |
| `Evolution` | Summoner | 1 (exact) | 2 |
| `Bloodline` | Sorcerer | 53 | 391 |
| `Bloodrager Bloodline` | Bloodrager | 12 | 110 |
| `Domain` | Cleric | **73** per-domain groups | **310** |
| `Order` | Cavalier | 1 (`Cavalier Order`, the zero-magnitude dispatcher cycle 3 already named — the ~18 per-order groups are NOT suffix-shaped, `"Order of the Lion"` does not end in `" Order"`, so this script correctly does not fabricate a match for them; confirms cycle 3's two-level-walk finding rather than contradicting it) | 7 |
| `Mystery` | Oracle | **21** per-mystery groups | **234** |
| `Curse` | Oracle | **0 — confirmed no match** (`Curse Subdomain`/`Dual-Cursed`/etc. do not end in `" Curse"`) | 0 |
| `Spirit` | Shaman | 14 | 73 |
| `Animal Focus` | Hunter | 2 | 35 |
| `Favored Enemy` | Ranger | 1 (exact) | 36 |
| `Favored Terrain` | Ranger | 1 (exact) | 18 |
| `Versatile Performance` | Bard | 1 (exact) | 9 |
| `Arcane School` | Wizard | 1 | 17 |
| `Focused Arcane School` | Wizard | 1 (exact) | 17 |

**Total real records reachable across all 28 registered pools once name resolution is generic:
2,061** — versus the 71 records the two-entry `REGISTERED_POOL_GROUPS` allowlist in
`class_feature_pool_catalog.rs` originally served, and versus the near-zero exact matches cycle 3
found for `Mystery`/`Domain`/`Blessing`/`Order`/`Spirit` by hand. **This is `§16` instrument
correction, not closure** — these are candidate groups whose real name and count are now known;
`resolve_pool_member_sole_magnitude` must still be called per group (with the correct member-key
prefix, which per cycle 3's finding is NOT always the same string as what this table reports for
census purposes — e.g. Oracle members live under `"<Mystery Name> ~ <revelation>"`, not a
`"Mystery ~ <name>"` prefix) before any of these 2,061 records reach `compute_pilot_base_chassis`.
Three pools (`Grand Discovery`, `Advanced Talents`, `Curse`) are re-confirmed genuinely absent from
the corpus under any word-boundary or plural-normalized shape — not a script gap, a real absence.

## 2. The generic header-suffix fix (`§17`, one mechanism)

**Root cause, confirmed exactly as cycle 3 described it:** `resolve_pool_member_sole_magnitude`
looked up a pool's HEADER record (which carries the pool-specific level variable, e.g.
`SlayerTalentLVL|SlayerLVL`) by building `format!("{class} ~ {pool_group}")` — a byte-exact key.
For Slayer Talent, the correct MEMBER-key prefix (`"Slayer Talent ~ <name>"`) and the real HEADER
record's own name (`"Slayer ~ Slayer Talents"`, plural) are different strings, so the exact-match
lookup silently found nothing and every member formula needing `SlayerTalentLVL` refused.

**Fix — `pool_header_record_by_normalized_suffix` (`src/rules_core/pilot_compute/mod.rs`):** tries
the exact key first (the common case — `Alchemist ~ Discovery`, `Witch ~ Hex` are already exact,
unaffected), then falls back to a word-boundary suffix search over every `"<class> ~ *"` header
candidate, comparing the candidate's own suffix and the requested `pool_group` after stripping one
trailing `s` from each. One function, no per-pool table, reused unchanged by every existing and
future call site of `resolve_pool_member_sole_magnitude` — Alchemist Discovery's own header lookup
(`Alchemist ~ Discovery`) runs through the same function and is provably unaffected (its own
regression suite, `spellcasting_shaped_class_closure_tests`, all still green, verified below).

**What this closes, verified live (not assumed):** three Slayer Talent members whose formulas
chain through `SlayerTalentLVL` and previously refused now resolve to their real corpus-derived
value:

```bash
cargo test --locked --lib -- rules_core::pilot_compute::opponent_conditioned_tier_zero_tests::slayer
```
```
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 2731 filtered out
```

- `slayer_assassinate_dc_resolves_generically_once_the_header_binds_slayertalentlvl` — DC = `10 +
  (SlayerTalentLVL=6 / 2) + INT(0)` = **13**.
- `slayer_slowing_strike_dc_resolves_generically_once_the_header_binds_slayertalentlvl` — same
  shape, DC = **13**.
- `slayer_hard_to_fool_times_resolves_generically_once_the_header_binds_slayertalentlvl` —
  `SlayerTalentLVL=6 / 5 + 1` = **2** (references `SlayerTalentLVL` directly, no intermediate hop —
  proves the fix also closes a record with no chain, not only the Assassinate/Slowing-Strike
  two-hop shape).
- `slayer_combat_style_i_refuses_rather_than_guess_between_two_terminal_targets` — replaces the OLD
  test that asserted the bug's own symptom (`None` for Assassinate) with a proof that the
  SEPARATE multi-terminal safety refusal (two unrelated `BONUS:VAR` targets, neither referencing
  the other) still holds, unweakened by this fix.

**Not fabrication — the safety property that made cycle 3 refuse correctly is preserved and
re-tested**, not removed: `resolve_pcgen_var_chain` still errors on a genuinely unbound
identifier, and `resolve_pool_member_sole_magnitude` still returns `None` for any record with zero
or more than one terminal target (Combat Style I, proven above) or a formula this evaluator cannot
parse. What changed is that `SlayerTalentLVL` is now CORRECTLY bound (it always had a real,
corpus-defined value; the lookup just could not find where), so the values these three records now
emit are real, not guessed.

## 3. Tests, RED->GREEN, both altitudes (`§1a`)

**Mutation altitude 2 (library logic — the fix itself):** in
`pool_header_record_by_normalized_suffix`, inserted `if true { return None; }` immediately before
the fallback suffix search (leaving the exact-match fast path untouched) -> re-ran the new
Assassinate test:

```
thread '...' panicked: assertion `left == right` failed: 10 + (SlayerTalentLVL=6 / 2) + INT(0) = 13
  left: None
 right: Some(13)
```

RED confirmed. Reverted (`if true { return None; }` removed, function restored verbatim).

**Mutation altitude 1 (chassis call site):** wrapped `class_slayer.rs`'s existing
`push_generic_pool_choice_magnitude(...)` call for `"Slayer Talent"` in `if false { ... }` -> re-ran
the same test:

```
thread '...' panicked: assertion `left == right` failed: 10 + (SlayerTalentLVL=6 / 2) + INT(0) = 13
  left: None
 right: Some(13)
```

RED confirmed. Reverted; call site restored verbatim (byte-identical to cycle 3's own).

**Regression check**, the touched files' full existing suites plus the shared pool-catalog/
grant-consumer suites and the full `pilot_compute` module (scoped, not the repo-wide suite):

```bash
cargo test --locked --lib -- rules_core::pilot_compute::opponent_conditioned_tier_zero_tests::slayer \
  rules_core::pilot_compute::slayer_dispatch_widening_safety_tests \
  rules_core::pilot_compute::spellcasting_shaped_class_closure_tests \
  class_feature_pool_catalog class_feature_grant_consumer
```
```
test result: ok. 83 passed; 0 failed; 0 ignored; 0 measured; 2662 filtered out
```
```bash
cargo test --locked --lib -- rules_core::pilot_compute::
```
```
test result: ok. 914 passed; 0 failed; 0 ignored; 0 measured; 1831 filtered out
```
No pre-existing test's behaviour changed; Alchemist Discovery's own header-chain closure
(`spontaneous_healing_discovery_resolves_generically_through_the_pool_header_chain`) is unaffected
(its header key was already exact, so the fast path -- not the new fallback -- serves it, and the
fast path is byte-identical to the pre-cycle code).

## 4. Sweep (`§3`)

```bash
grep -rn "5,981\|5981\b\|5,927\|5927\b" docs/release/SD-32-compute-library-and-cause-closure/*.md tests/ src/ scripts/ apps/
```
All hits are prior cycles' own historical citations (cycle 1/2's kanban text, cycle 3's own receipt
and doc-comment, unchanged) — this cycle introduces no new numeric-magnitude residual figure (the
header-suffix fix moves 3 units from refused to resolved WITHIN the already-measured 5,927
residual; it does not change the census count, since those 3 records were already counted as
`numeric_magnitude` by `census_class_feature_pool_population.py`, just not yet closed).

```bash
grep -rn "pool_header_record_by_normalized_suffix\|census_class_feature_pool_group_names" tests/ src/ scripts/ apps/ | grep -v "pilot_compute/mod.rs\|census_class_feature_pool_group_names.py"
```
No hit outside the two files this cycle touched (plus this receipt and the kanban row).

## 5. Scope discipline

**Did not attempt** (real, scoped follow-on, named rather than silently deferred):

- Rewiring the other 27 registered pools' call sites at their now-known real group names/counts
  (§1 table above) — each still needs its own real MEMBER-key-prefix verification (which, per
  cycle 3's Oracle/Cavalier findings, is not always the same string this census reports for the
  HEADER-owning group) before `push_generic_pool_choice_magnitude` can be called safely. The
  dispatch's own named next targets:
  - **Hunter Animal Focus** (21 real records, exact match) — activation-gated, needs careful
    integration per cycle 3's own scope note; not attempted.
  - **Cavalier Order** — confirmed (again) to need the two-level dispatcher walk; the 7-record
    `Cavalier Order` group itself is zero-magnitude (ABILITY-only), so wiring it directly would
    close nothing real.
  - **Oracle's 21 real per-mystery groups (234 records)** and **Cleric's 73 real per-domain groups
    (310 records)** — now that their real names and counts are known (§1), a future cycle can wire
    them at scale without re-deriving names first. Real work, correctly not rushed here.
- Extending `pool_header_record_by_normalized_suffix`'s normalization beyond a single trailing `s`
  (e.g. `-es`/`-ies` plurals) — no corpus case observed needing it; adding an unexercised branch
  would be untested code, not a closed shape.
- Rows 11/15 (left `in-progress`, untouched); `apps/desktop`'s row 19 lane,
  `bestiary_4/monster_ability`'s `DESC-PI-SHIPPED` lane — both sibling territory, not touched.
  `data/corpus/**` untouched throughout.

`df -h /`: reported in the dispatch's final report.
