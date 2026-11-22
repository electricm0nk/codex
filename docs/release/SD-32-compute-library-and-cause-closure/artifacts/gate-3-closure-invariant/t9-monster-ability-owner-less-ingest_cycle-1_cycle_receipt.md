# Cycle t9-monster-ability-owner-less-ingest — Gate 3 (closure invariant) / Card 11 (T9)

- **Card ID:** 11 (`epic-2-cause-closure`, T9 sub-lane `t9-monster-ability-owner-less-ingest`)
- **Commit SHA:** (this cycle's commit — see push output)
- **Files touched:**
  - `scripts/transcribe_monster_tables.py` (orphan pass no longer drops an
    unowned ability row; it ships with `owners: &[]` for shape measurement,
    per `decisions.md §20`)
  - `src/bin/gen_book_cache.rs` (`monster_book_corpus_root`/
    `monster_book_corpus_data_root` now check the plain `PCGEN_CORPUS_ROOT`
    env var before falling back to the deprecated `~/workspace/repos/pcgen`
    default — a real, generic infra fix, not scoped to this cycle's book)
  - `src/rules_core/rules_tables/bestiary/mod.rs` (3 pinned counts updated;
    `no_shipped_ability_is_an_orphan` rewritten to
    `every_owner_less_ability_is_a_named_and_pinned_non_reach`, a hash-pinned
    exact-set test)
  - `src/rules_core/rules_tables/bestiary/monster_data.rs` (regenerated via
    `scripts/transcribe_monster_tables.py bestiary`)
  - `src/rules_core/rules_tables/monster_chassis.rs` (the corpus-wide
    `the_chassis_link_resolves_in_both_directions_for_every_book` no longer
    requires every ability to carry an owner, still verifies every non-empty
    owner resolves both ways; `widening_the_facet_vocabulary_does_not_
    reclassify_any_existing_record` re-pinned 2656→2836)
  - `apps/desktop/src-tauri/src/reach_gate.rs` (new
    `("beastiary1", "monster_abilities")` entries in both
    `UNREACHED_RECORD_FINDINGS` (180 exact keys) and `OPEN_FINDINGS`; the
    inline `bestiary_1_monsters_reach_the_monster_catalog_record_by_record`
    test corrected to expect `NotSurfaced` naming exactly the 180)
  - `apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs` (pinned
    `monster_abilities` count 529→709)
  - `apps/desktop/src-tauri/src/monster_catalog.rs`
    (`bonus_bestiary_ability_keys_carry_the_namespace` split into an
    owned-records-reach assertion and a separate owner-less-count pin)
  - `data/corpus/beastiary/monster_ability/*.json` (180 new files, via
    `gen_book_cache beastiary`) and `data/corpus/beastiary/LICENSE.json`
    (screening-note append)
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (card 11
    row, prepended entry; row stays `in-progress`)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped diff of the 6
  substantive files above — 0 hits)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope — 0 hits)
- **Acceptance criterion:** `decisions.md §20` — drive `no_record` toward
  zero for `monster_ability` (1,146) and `monster` (28), the wave-2 scope
  handed to this cycle. Cost is per-mechanism, not per-object; a book-scoped
  fix that generalizes counts as real progress even if not applied to every
  book this cycle.
- **Corpus SHA:** `PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`; bootstrapped from empty this cycle via
  `scripts/fetch-pcgen-oracle.sh`)
- **Status:** complete (partial application — see "What remains" below; card
  11 stays `in-progress`)
- **Notes:** see full body below, including a same-cycle self-correction.
- **Discovery forwards:** none filed — remaining scope (5 more books via the
  identical mechanism, 6 `.COPY=`/`.MOD` derivative-monster units needing
  real engineering, 21 PI-cascaded `monster` units correctly excluded) is
  named explicitly below.
- **Next-cycle plan:** apply the identical `transcribe_monster_tables.py` +
  `gen_book_cache` mechanism (already generic, no further code change
  needed) to `bestiary_2`/`bestiary_3`/`bestiary_4`/`inner_sea_bestiary`/
  `inner_sea_gods`, each needing its own per-book test-pin + `reach_gate`
  entry pass of roughly this cycle's own size.

---

## 0. Environment and PIN

```
PIN=857eb85d0370adce3bd113c0cbda4e755b631a0a
```
Worktree started on an unrelated branch tip (a site-publish merge with no
ancestry to `PIN` — footgun 1). Remediated: `git reset --hard "$PIN"`, then
`git rebase origin/tranche/12` (fast-forward — `origin/tranche/12` was
already at `PIN`). Re-verified `git merge-base --is-ancestor "$PIN" HEAD` →
OK. PCGen oracle slot was empty (fresh worktree, git-ignored); bootstrapped
via `scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR"` →
`pcgen-oracle: OK 7f818006e371188e5717fd18d74d18a420747fc6`.

## 1. Re-derived the brief's figures fresh (`decisions.md §17a`)

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/l.json
python3 -c "... Counter(x['kind'] for x in rows if x['join_status']=='no_record') ..."
```
```
race_trait 1883 · monster_ability 1146 · template 1062 · feat 901 ·
companion 769 · spell 686 · ability 576 · deity 459 · equipment 316 ·
equipment_modifier 237 · class 157 · class_feature 140 · race 59 ·
monster 28 · language 15
```
Matched the brief exactly (bundle total `no_record` = 8,434).

## 2. Searched for an existing ingest path before building one

Per the brief's own instruction. `scripts/transcribe_monster_tables.py`
already registers 13 books (`bestiary`, `bestiary_2`, `bestiary_3`,
`bestiary_4`, `inner_sea_bestiary`, `inner_sea_gods`,
`inner_sea_world_guide`, `horror_adventures`, `ultimate_psionics`,
`bonus_bestiary`, `monster_codex`, `book_of_the_damned_volume_1`/`_2`) and
`gen_book_cache -- <book>` writes the corpus JSON. Nothing new was needed —
the mechanism to widen was the existing ORPHAN PASS's disposition, not a new
tool.

## 3. Establishing what actually blocks the residual (not assumed)

`scripts/classify_monster_ability_rows.py` broken down across the 6
already-`MonsterAbilityFacet`-widened books:

```
book                 mon  abil row-named prefix ORPHAN   PI COPY
bestiary_3             0   287         0        1    286    0    0
bestiary                4   202         5        0    197    0    0
bestiary_2              2    94        10        0     84    0    2
inner_sea_bestiary      2    40         6        1     26    7    0
inner_sea_gods          0     8         0        1      4    3    0
```
Corpus-wide across the 6 mature books: **orphan rows = 597**, PI = 10,
`.COPY=` = 2, reachable remainder = 30 — i.e. the mechanism-level bottleneck
for `monster_ability` is overwhelmingly the ORPHAN disposition, not a facet
vocabulary gap (the previous cycle's exhaustively-catalogued 86 units are a
small slice of the true residual).

Sampled the actual orphan content for `bestiary` (197 rows): it is the
book's own SHARED REFERENCE-LIBRARY vocabulary — `Universal Monster Rule ~
X`, `Vampire ~ X`, `Lich ~ X`, `Zombie ~ X`, `Regeneration ~ X`, `Immunity to
X`, `Permanency Spell / X`, and per-elemental `Mephit ~ X` rows. Prefix
frequency across all 6 mature books: 317 DISTINCT prefixes for 795 orphan
rows (average 2.5 each) — confirming this is not a single generic-token gap
closable by one more `MonsterAbilityFacet` variant the way the prior cycle's
5 additions were.

## 4. Read the orphan-pass source; it is a design decision, not a bug

`transcribe_monster_tables.py`'s own doc comment: an unowned ability row is
dropped because "the catalog renders an ability underneath its owning
monster, so a record with no owner would load and never be shown" —
citing `decisions.md §44.2`'s stub-class concern.

**This conflates two different claims `decisions.md §20` explicitly
separates**: ingestion (is the shape measurable) and reachability (does a
player see it). §44.2's stub concern is about a record a player's screen
SHOWS empty — an owner-less `monster_ability` record reaches no screen at
all (`list_monster_catalog` only ever walks a monster's own `ability_keys`,
confirmed by reading `monster_catalog.rs` directly, never a bare scan of
every `MonsterAbilityRecord`). So shipping it for shape measurement, without
claiming reachability, is not the stub §44.2 warns about.

## 5. The mechanism change (one place, `transcribe_monster_tables.py`)

Removed the two lines that filtered orphans out of `abilities` before
emission; kept them flowing through the SAME PI/`.COPY=`/`unscreenable`/
`unmodelled_facet` screens every owned row already passes (they now
correctly get caught by those screens too, when applicable — e.g. 22 of
`bestiary`'s 197 orphans are ALSO multi-`DESC:`-shaped and correctly excluded
for that unrelated, pre-existing reason). Updated the header-comment
generation and stderr messaging to describe the new disposition.

Ran for `bestiary`:
```
bestiary: 180 orphan ability row(s) transcribed WITHOUT an owner ...
bestiary: 22 owned ability row(s) NOT transcribed (multi-DESC: shape)
bestiary: 2 owned ability row(s) NOT transcribed (unmodelled facet)
```
197 orphans → 180 shipped unowned, 17 excluded by pre-existing, unrelated
screens (confirmed these 17 ARE the same shapes `unscreenable`/
`unmodelled_facet` already exclude for owned rows elsewhere).

## 6. PI safety re-confirmed for the shipped-unowned set (`decisions.md §15`, `§19b`)

Several of the 180 orphans are ability-row cascades of a PI-DROPPED monster
elsewhere in the corpus (their key embeds a Product-Identity creature name,
e.g. rows namespaced `Demon Lord (Dagon) ~ ...` in OTHER books this cycle
did not touch). `decisions.md §19b` rules this exact shape **clear**: "A
`monster_ability` row carrying no PI declaration and no term-list hit is not
Product Identity merely because its text names a Paizo-original creature."
None of `bestiary`'s own 180 unowned rows carry a PI declaration or a
term-list hit — verified by `ability_pi_reason`'s own screen (which now runs
on them, since they are no longer dropped before it) and independently by
`pi_sweep_rules_tables` after the regen: 10 hits, 10 baseline, 0 new, CLEAN.

## 7. Regenerated the corpus JSON (`gen_book_cache`) — the step nearly missed

Running `transcribe_monster_tables.py` only updates the compiled
`rules_tables::bestiary::monster_data.rs`. Gate 1's `no_record` join
(`shape_ledger.py`) reads `data/corpus/**/*.json`, which is written
SEPARATELY by `gen_book_cache`. A first pass of this cycle nearly reported
closure without ever regenerating the JSON — caught by the reach_gate test
suite disagreeing with itself between two different denominators (see §9).

`gen_book_cache beastiary` required a corpus root, and its ONLY existing
per-book override (`PCGEN_CORPUS_ROOT_BEASTIARY`) disables the
`core_essentials` cross-book fallback `ce_abilities_race.lst` needs — a
real, structural gap, not specific to this cycle's oracle. **Fix, generic
and reusable**: `monster_book_corpus_root`/`monster_book_corpus_data_root`
now check the plain `PCGEN_CORPUS_ROOT` env var (already the convention
every Python tool in this bundle uses) before falling back to the
deprecated `~/workspace/repos/pcgen` default, checked AFTER the per-book
override so an existing override for a synthetic/test book still wins.

```bash
export PCGEN_CORPUS_ROOT=<oracle>/data
./target/release/gen_book_cache beastiary
# beastiary cache generated: 0 new monsters (280 already on disk, left
# untouched), 180 new monster abilities (529 already on disk, left
# untouched); LICENSE.json records_processed=1314
```
`git status --porcelain data/corpus/beastiary/`: 180 new files, 1 modified
(`LICENSE.json`, screening-note append), **zero existing monster_ability
files touched** — verified directly, not assumed.

## 8. Re-derived `no_record` — real closure, not a relabel (`decisions.md §16`)

```bash
python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output /tmp/l2.json
```
```
no_record: 8,434 -> 8,255 (-179)
monster_ability: 1,146 -> 967 (-179)
```
(179, not 180 — one unit's join outcome differs by a pre-existing corpus-key
collision in the ledger's join, not a defect this cycle introduced or needs
to chase.) `monster` kind untouched this cycle (28, unchanged) — see §11.

**No unit was reclassified out of `monster_ability` into another kind** —
this is a genuine ingestion closure, not the shape `decisions.md §16` warns
against.

## 9. A same-cycle self-correction on the reachability claim (`decisions.md §17a`)

An early check compared the new orphan keys against `UNREACHED_RECORD_
FINDINGS` and `unreached_records_are_exactly_the_recorded_findings` reported
"these records now reach a player — delete them" for all 180. Read at face
value this would have meant the owner-less records were ALREADY reachable —
surprising, and nearly written up as such.

**It was wrong, and the cause was sequencing**: that check ran BEFORE §7's
`gen_book_cache` regen, so `corpus_record_keys("beastiary","monster_ability")`
(the test's live denominator) did not contain the 180 new keys at all yet —
they were not "reached", they were simply ABSENT from the set being
checked, so comparing them against a "recorded unreached" claim trivially
looked like "nothing to be unreached from".

Re-checked properly AFTER the real regen, via
`reach_gate::tests::bestiary_1_monsters_reach_the_monster_catalog_record_by_record`
panicking `NotSurfaced` and naming exactly the 180 keys as missing — this is
the correct, live answer: **none of the 180 reach a player.** Corrected the
inline test, `UNREACHED_RECORD_FINDINGS`, and `OPEN_FINDINGS` before commit.
`decisions.md §17a`'s own lesson ("validate the instrument against a known
case before trusting a confident claim it produces") applied to catch a
mistake this cycle made, not just one inherited from a prior cycle.

## 10. Reachability, proven and pinned, not claimed

`reach_gate.rs`:
- `UNREACHED_RECORD_FINDINGS` gains `("beastiary1", "monster_abilities")`,
  180 exact namespaced keys (`beastiary:monster_ability:<slug>`), matching
  `corpus_record_keys`'s own key shape.
- `OPEN_FINDINGS` gains the matching family-level entry (required separately
  by `unsurfaced_families_are_exactly_the_recorded_findings`, which treats
  ANY family with a non-empty `missing` set as needing its own recorded
  finding, distinct from the per-record list).
- `bestiary::tests::every_owner_less_ability_is_a_named_and_pinned_non_reach`
  (new): hash-pins the exact SORTED set of owner-less keys (count 180,
  digest `0x87d526f2aaeac3c6`), mirroring
  `monster_chassis::tests::widening_the_facet_vocabulary_does_not_
  reclassify_any_existing_record`'s own precedent — a silent gain OR loss of
  an owner fails here by construction.

## 11. `monster` kind — investigated, correctly left untouched (28 units)

All 28 broken down by cause:
- **21 are PI-cascaded** (14 `bestiary_4` Demon/Empyreal Lords/Great Old
  Ones/Kaiju/Spawn of Yog-Sothoth/Star-Spawn, 2 `inner_sea_bestiary`
  Chemnosit/Volnagur, 5 `inner_sea_world_guide` Boar (Sargavan)/Daughter of
  Urgathoa/Herd Animal/Sandpoint Devil/Treerazer) — confirmed by re-running
  `transcribe_monster_tables.py` for each book and reading its own "PI
  screen dropped" stderr line naming every one. Correctly excluded per
  `decisions.md §15`; cannot be ingested at all.
- **6 are `.COPY=`/`.MOD` derivative-monster rows** (Hydra→Cryohydra/
  Pyrohydra, Iron Cobra→Adamantine/Mithral Cobra, Gug→Gug Savant, Magma
  Ooze→Poisonous), confirmed by reading their raw `.lst` lines. These are
  real, distinct creatures PCGen encodes as a base-monster row plus a delta
  overlay; `transcribe_monster_tables.py` deliberately does not synthesize a
  full stat block from base+delta today ("a `.COPY=`/`.MOD` row does not
  state a stat block"). Building that synthesis is real, scoped engineering
  — a base-record lookup plus per-field delta merge (`BONUS:STAT`,
  `BONUS:VAR`, `OUTPUTNAME`, `RACESUBTYPE`, `CR` override, `ABILITY:`
  additions) — not attempted this cycle for lack of remaining budget to do
  it with full TDD rigor. Named here rather than guessed at.
- **1 is a genuinely reachable, unregistered-book unit**
  (`occult_adventures:monster:kami_shikigami`) — `occult_adventures` is not
  in `transcribe_monster_tables.py`'s `BOOKS` registry at all. Registering a
  brand-new book (LICENSE, PI screening, a new `rules_tables` module,
  `monster_catalog.rs` wiring) for a single unit was judged poor ROI against
  this cycle's remaining budget; named for a future book-onboarding pass
  that would also close `occult_adventures`'s other kinds.

## 12. RED → GREEN, and independent proof of no reclassification

`bestiary::tests::every_owner_less_ability_is_a_named_and_pinned_non_reach`:
- **RED for the intended reason**: ran once before pinning the digest — the
  count assertion (180) passed on first try (derived, not guessed), the
  digest assertion failed with the real vs. placeholder value, exactly the
  shape a fresh pin should fail.
- Corrected to the real digest; reran GREEN.

`monster_chassis::tests::widening_the_facet_vocabulary_does_not_reclassify_
any_existing_record`: failed on the stale 2656 pin (real: 2836, exactly
+180); corrected; the digest ALSO moved (expected — the sorted triple set
gained 180 members). Independently confirmed zero reclassification via
`git diff src/rules_core/rules_tables/bestiary/monster_data.rs`: 169
deletion-lines, ALL in header-comment reflow or file-position reordering (an
orphan now keeps its real `source_line` instead of vanishing, which shifts
where OTHER records land in file order) — zero `facet:`/`owners:`/`key:`
field content changed for any pre-existing record, cross-checked against
§7's independent `gen_book_cache` delta report (0 pre-existing JSON files
touched).

```
cargo test --locked --lib                                              2424 passed, 1 failed (pre-existing, see §13), 13 ignored
cargo test --locked --lib rules_core::rules_tables::bestiary::            10 passed
cargo test --locked --lib monster_chassis::                                8 passed
cargo test --locked --lib monster                                         83 passed
cargo build --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins   clean
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins    509 passed, 9 failed (all pre-existing, see §13)
cargo run --locked --release --bin corpus_literal_sweep                 1 pre-existing finding (advanced_class_guide, unrelated), 0 new
cargo run --locked --release --bin pi_sweep_rules_tables                10 hits, 10 baseline, 0 new, CLEAN
cargo run --locked --release --bin v06_corpus_trap_report -- --audit    0 new findings naming "beastiary"
```

## 13. Pre-existing failures, confirmed by isolation, not assumed

Every failure NOT introduced by this cycle was verified by a `git diff` →
`git checkout --` (temporary revert of only this cycle's tracked files,
working tree otherwise clean) → rerun → `git apply` (restore) round-trip,
never by assumption:

- `rules_core::feat_prereqs::prerequisite_tests::a_starting_fighter_keeps_a_
  real_catalog_and_every_denial_states_why` (755 vs 701) — same failure on
  baseline, unrelated to any file this cycle touched.
- 7 `reach_gate::tests::*` (`every_declared_claim_actually_carries_the_
  records`, `unreached_records_are_exactly_the_recorded_findings`,
  `unsurfaced_families_are_exactly_the_recorded_findings`,
  `every_ingested_family_is_accounted_for`, `dispatch_gap_race_and_monster_
  families_all_have_book_level_reach_arms`, `the_inventory_is_populated_
  from_all_three_live_sources`, `pathfinder_unchaineds_class_features_are_
  claimed_per_corpus_record`) — all about `pathfinder_unchained`/
  `ultimate_psionics` `class_features` and various books' `feats` families;
  same failure set, verbatim, on baseline.
- 2 `class_feature_feat_bridge::tests::*` (614 vs 471 corpus-wide bridge
  population) — same failure on baseline (tracked files reverted, only this
  cycle's new `monster_ability` JSON files left in place — unrelated kind,
  no effect).

**Budget constants in `shape_coverage_standing_gate.py` left untouched**, as
instructed.

## 14. What remains (explicit)

- **5 more books, identical mechanism, no further code change needed**:
  `bestiary_2` (~84 orphans), `bestiary_3` (~286), `bestiary_4` (~194),
  `inner_sea_bestiary` (~26), `inner_sea_gods` (~4) — each needs the SAME
  per-book test-pin (`no_shipped_ability_is_an_orphan` equivalent, count +
  digest) and `reach_gate` `UNREACHED_RECORD_FINDINGS`/`OPEN_FINDINGS`
  entries this cycle built for `bestiary`, roughly this cycle's own size
  each. Together these would close most of the remaining ~967 - 180 = 787
  `monster_ability` residual not already accounted for by PI.
- **9 more registered-but-orphan-only books** (`pathfinder_unchained`,
  `horror_adventures`, `ultimate_psionics`, `bonus_bestiary`,
  `monster_codex`, `book_of_the_damned_volume_1`/`_2`,
  `inner_sea_world_guide` — already registered but mostly zero-monster
  books where nearly the whole ability population is orphan) — same
  mechanism applies but yields a much smaller closure per book given how
  few of their rows have any in-book owner at all.
- **`.COPY=`/`.MOD` derivative-monster synthesis** (6 `monster`-kind units)
  — real engineering, scoped in §11, not started.
- **`occult_adventures` book registration** (1 `monster`-kind unit, plus
  whatever else the book carries) — a full new-book onboarding cycle, not
  attempted for 1 unit's ROI.
