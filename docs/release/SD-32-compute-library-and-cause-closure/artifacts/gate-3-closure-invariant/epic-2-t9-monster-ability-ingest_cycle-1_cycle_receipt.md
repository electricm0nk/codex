# Cycle epic-2-t9-monster-ability-ingest — Gate 3 (closure invariant) / Card 11 (T9)

- **Card ID:** 11 (`epic-2-cause-closure`, T9 sub-lane `t9-monster-ability-ingest`)
- **Commit SHA:** (this cycle's commit — see push output)
- **Files touched:**
  - `src/bin/gen_book_cache.rs` (`bestiary_4`'s `MonsterBookSpec::abilities_lsts` widened by one file, `b4_abilities_races_ce.lst`)
  - `src/rules_core/rules_tables/bestiary_4/monster_data.rs` (regenerated via `scripts/transcribe_monster_tables.py bestiary_4`: 577 → 619 `MonsterAbilityRecord`s, +42)
  - `src/rules_core/rules_tables/bestiary_4/mod.rs` (two pinned-count tests updated: 577→619 abilities, 783→825 total)
  - `src/rules_core/rules_tables/ultimate_psionics/monster_data.rs` (regenerated via `scripts/transcribe_monster_tables.py ultimate_psionics`: 15 → 127 `MonsterAbilityRecord`s, +112)
  - `src/rules_core/rules_tables/ultimate_psionics/mod.rs` (one pinned-count test updated: 15→127 abilities)
  - `data/corpus/bestiary_4/monster_ability/*.json` (76 new files, via `gen_book_cache -- bestiary_4`)
  - `data/corpus/ultimate_psionics/monster_ability/*.json` (114 new files, via `gen_book_cache -- ultimate_psionics`)
  - `data/corpus/bestiary_4/LICENSE.json`, `data/corpus/ultimate_psionics/LICENSE.json` (screening-note append, existing `compose_screening_note` mechanism)
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (card 11 row, `in-progress`, prepended entry)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped diff: `gen_book_cache.rs`, both books' `mod.rs`/`monster_data.rs`, all new corpus JSON — 0 hits)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope — 0 hits)
- **Acceptance criterion:** Build the generic, config-driven `monster_ability` corpus ingest pass for T9's PI-cleared books (card 11, `decisions.md §19` sign-off, `§17` generic-pass discipline); transcribe only `clear` units; fixture-check against the pinned oracle; prove reachability; prove RED→GREEN; stop and report by name on any record believed to carry PI despite its disposition (`§15`); sweep pinned counts; report Gate 3's new `no_record` figure without touching budget constants.
- **Corpus SHA:** `PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`; bootstrapped from empty this cycle via `scripts/fetch-pcgen-oracle.sh`)
- **Status:** complete (partial — see "What remains" below; row 11 stays `in-progress` per its own multi-shape acceptance bar)
- **Notes:** see full body below — the central finding is a correction of the prior `t9-onboarding` cycle's own claim that no generic ingest path exists for `monster_ability`.
- **Discovery forwards:** none filed — remaining scope (facet widening, 4 untouched kinds) is named explicitly below, not deferred silently.
- **Next-cycle plan:** widen `MonsterAbilityFacet` (`src/rules_core/rules_tables/monster_chassis.rs`) to model the bare-`TYPE:SpellLike`/`Weakness.Extraordinary`/`Internal`/`Communicate.Supernatural` shapes found blocking `bestiary`/`bestiary_2`/`bestiary_3`/`inner_sea_bestiary`/`inner_sea_gods` (876 clear units combined) — real, corpus-wide-blast-radius engineering, adversarially verified per `decisions.md §16`'s own caution, not attempted this cycle. Then re-run `transcribe_monster_tables.py`/`gen_book_cache` for those 5 books. `feat`/`equipment`/`companion`/`monster` kinds are untouched; the prior `t9-onboarding` receipt's own per-kind counts (`feat` ~397, `equipment` ~48, `companion` ~4, `monster` ~7) stand as the next lever for those.

---

## 0. Environment and PIN

```
PIN=ca82102d84f60d78ee925514f667f7ef04a59deb
```
Worktree started on an unrelated branch tip (`worktree-wf_a65ba9be-131-2` at `275581bf0`, a
site-publish merge with no ancestry to `PIN` — footgun 1). Remediated: `git reset --hard
ca82102d84f60d78ee925514f667f7ef04a59deb` then `git rebase origin/tranche/12` (fast-forward, no
new commits beyond the pin — `origin/tranche/12` was itself exactly at `PIN` at cycle start),
re-verified `git merge-base --is-ancestor "$PIN" HEAD` → OK. PCGen oracle slot was empty (fresh
worktree, git-ignored); bootstrapped via `scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR"` →
`pcgen-oracle: OK 7f818006e371188e5717fd18d74d18a420747fc6`, matching the pin.

## 1. Re-derived T9's `monster_ability` disposition fresh (`decisions.md §17a`)

Did not trust the brief's pasted `~1,342` estimate. Ran the committed pipeline fresh:

```bash
cargo build --locked --release --bin v06_work_inventory
"$CARGO_TARGET_DIR/release/v06_work_inventory" --stdout-only > fresh_inventory.json
python3 scripts/sd32_t9_pi_exposure_audit.py fresh_inventory.json \
    --corpus-root "$PCGEN_CORPUS_ROOT" --json-out t9_pi_classified.json
python3 scripts/sd32_t9_pi_final_disposition.py fresh_inventory.json t9_pi_classified.json \
    --corpus-root "$PCGEN_CORPUS_ROOT"
```

**Result: `monster_ability` total=1,378, blocked=80, clear=1,298, still_undecidable=0** (whole T9
population: total=3,615, blocked=332, clear=3,074, still_undecidable=209 — smaller than the prior
`t9-onboarding` cycle's own 3,685, itself smaller than the brief's original 3,573; concurrent-lane
drift on the shared branch continues, confirmed not an error either run since the set of 20
fully-resolved books is again unchanged).

Isolated `monster_ability`'s 1,298 `clear` units to their per-book split (own script, reusing the
same `sd32_t9_pi_exposure_audit`/`sd32_t9_pi_review_companion_monsterability` modules the committed
pipeline uses — `decisions.md §19b`'s "row's own declaration governs" rule applied identically):

```
bestiary              blocked=0   clear=92
bestiary_2            blocked=0   clear=117
bestiary_3            blocked=0   clear=629
bestiary_4            blocked=65  clear=168
horror_adventures     blocked=0   clear=65
inner_sea_bestiary    blocked=7   clear=33
inner_sea_gods        blocked=5   clear=5
inner_sea_world_guide blocked=3   clear=13
ultimate_psionics     blocked=0   clear=176
TOTAL clear = 1298
```

**Every one of these 9 books is already registered in `monster_chassis::MONSTER_BOOKS`** (the
compiled-table registry `v06_work_inventory`/`monster_catalog`/`reach_gate` all iterate generically).
This is the pivot for everything below.

## 2. Correction of the prior cycle's own finding — a generic ingest path already exists

The `epic-2-t9-onboarding_cycle-1` receipt (its own §2, "next remains") concluded: *"No generic
raw-`.lst`-to-`data/corpus/**/*.json` ingest path exists yet for [`monster_ability` and four other
kinds]... Building one... is real, separate engineering work per kind."* This is **wrong for
`monster_ability`**, and the error is checkable in one command:

```bash
grep -n '^\s*"bestiary"\|"bestiary_2"\|"bestiary_3"\|"bestiary_4"\|"inner_sea_bestiary"\|"inner_sea_gods"\|"horror_adventures"\|"inner_sea_world_guide"\|"ultimate_psionics"' \
    scripts/transcribe_monster_tables.py
```

All 9 books are already `BOOKS`-dict entries. `scripts/transcribe_monster_tables.py` (raw `.lst` →
compiled `rules_tables::<book>::monster_data.rs`, config-driven, PI-screened, orphan-aware) plus
`src/bin/gen_book_cache.rs::gen_monster_book` (compiled table → `data/corpus/**/*.json`,
config-driven via `MonsterBookSpec`, also already registered for all 9 books) **together are** the
generic raw-`.lst`-to-corpus-JSON path for `monster`/`monster_ability` — the exact shape the T9
onboarding brief asked this cycle to build. Adding a book to either config is the "8-line entry"
`ingest_spells.rs` set as the bar; these 9 books already have that entry each.

**Why the prior cycle missed this**: it correctly established that no such path existed for the
*five kinds other than spell* as a **blanket** claim, without checking whether `monster`/
`monster_ability` specifically already had one under a different name (`transcribe_monster_tables.py`
is not named `ingest_*`, so a name-based search for "ingest" tooling misses it). Logged here per
`decisions.md §17a` rather than silently corrected — the same failure shape as `§17a`'s own
`pi_screen`-drift correction: a confident claim, unvalidated against the actual mechanism.

## 3. Why the population is still "not-ingested" if the path already exists

`transcribe_monster_tables.py` reads `docs/work-inventory.json` (committed) and **deliberately
excludes ability rows no monster row of the book owns** — `orphans = [u for u in abilities if not
owners[u["corpus_key"]]]`, its own documented, tested behaviour (`monster_chassis.rs`'s own module
doc: "Only ability rows WITH an owner are registered... an ability row no monster row claims is a
record that loads and is never shown"). Running `scripts/classify_monster_ability_rows.py` over the
9 books confirms most of the 1,298 T9-`clear` population is exactly this — orphans the mechanism is
*correctly* refusing to ship, not a gap in the mechanism:

```
book                    mon  abil row-named prefix ORPHAN   PI COPY
bestiary_3                0   660         0    341    319    0    0
bestiary_4               14   236         0     42    194   14    0
bestiary                  4   209         9      3    197    0    0
ultimate_psionics         0   176         0    112     64    0    0
bestiary_2                2   154        70      0     84    0    2
horror_adventures         0    65         0      0     65    0    0
inner_sea_bestiary        2    40         6      1     26    7    0
inner_sea_world_guide     5    16         0      0     13    8    0
inner_sea_gods            0    10         0      3      4    3    0
```

Reachable-but-untranscribed (`row-named + prefix`, the real scope of this cycle):
`bestiary`=12, `bestiary_2`=70, `bestiary_3`=341, `bestiary_4`=42, `horror_adventures`=0,
`inner_sea_bestiary`=7, `inner_sea_gods`=3, `inner_sea_world_guide`=0, `ultimate_psionics`=112.
Item 5's brief instruction ("prove reachability, not just ingestion") is answered by the mechanism's
own orphan screen, not by a separate check this cycle had to build.

## 4. Re-ran the existing pipeline per book — 5 succeeded/no-op, 5 real-blocked

```bash
python3 scripts/transcribe_monster_tables.py <book>   # per book, PCGEN_CORPUS_ROOT set
```

- **`ultimate_psionics`**: succeeded. 15 → 127 abilities (+112). 64 `Astral_`-namespaced rows named
  explicitly on stderr as still-orphan (no monster row owns an `Astral` bundle) — correctly excluded.
- **`bestiary_4`**: succeeded (after §5's spec widening). 577 → 619 abilities (+42). PI screen
  independently dropped the same 14 monster rows (Demon Lords/Empyreal Lords/Great Old Ones/Kaiju/
  Star-Spawn personas) this book's `MonsterBookSpec::product_identity_source` already documents;
  194 orphans named on stderr.
- **`inner_sea_world_guide`**: ran clean, **byte-identical output, 0 new content** — its whole
  reachable set (14 abilities) was already fully shipped; the 3 PI-blocked + 13 orphan rows account
  for all 16 remaining corpus rows exactly. Confirms the mechanism is idempotent.
- **`bestiary`, `bestiary_2`, `bestiary_3`, `inner_sea_bestiary`, `inner_sea_gods`**: **refused**,
  correctly, on a real corpus shape the chassis does not model — `MonsterAbilityFacet` (`monster_
  chassis.rs`) only defines `SpecialAttack`/`SpecialQuality`, and a bare `TYPE:SpellLike` (no
  `SpecialAttack.`/`SpecialQuality.` prefix — the row states only the *delivery*, not the facet) or
  `TYPE:Weakness.Extraordinary` / `TYPE:Internal` / `TYPE:Communicate.Supernatural` has no facet
  segment to match. This is `transcribe_monster_tables.py`'s own stated contract ("nothing here is
  computed, defaulted, or inferred; a token the row does not carry becomes `None`" — and here the
  row states a shape the chassis has no slot for at all, so it hard-stops rather than guess). **Not
  attempted this cycle**: widening a shared enum every `monster_ability` consumer reads is a
  corpus-wide-blast-radius change `decisions.md §16` explicitly cautions needs adversarial
  verification, not a same-cycle add-on to a data-regen cycle.

## 5. One real code change: `bestiary_4`'s `MonsterBookSpec` was missing a second abilities file

`gen_book_cache -- bestiary_4` panicked on the newly-transcribed rows:
`"bestiary_4:Immunity to Calm Emotions cites b4_abilities_races_ce.lst, which is not in this book's
MonsterBookSpec::abilities_lsts (["b4_abilities_race.lst"])"`. Verified `b4_abilities_races_ce.lst`
is loaded by the SAME `_bestiary_4.pcc`/`_bestiary_4_for_players.pcc` (`grep -n
b4_abilities_races_ce.lst *.pcc` → line 59 both files, plain `ABILITY:` token, no `PRECAMPAIGN` or
other gate) that already loads `b4_abilities_race.lst` — same directory, same OGL declaration, no
new provenance question. Widened `abilities_lsts: &["b4_abilities_race.lst",
"b4_abilities_races_ce.lst"]`, matching the exact precedent `beastiary`'s own entry already sets
(`&["b1_abilities_race.lst", "ce_abilities_race.lst"]`) and `inner_sea_gods`'s plural-file entry.

## 6. RED → GREEN, concretely

Both books' own pinned shipped-count tests failed for the intended reason **immediately after** the
regen and **before** any test edit:

```
thread '...bestiary_4::tests::the_book_ships_two_hundred_six_monsters_and_five_hundred_seventy_seven_abilities' panicked:
  assertion `left == right` failed
    left: 619
   right: 577

thread '...ultimate_psionics::monster_tests::the_shipped_counts_are_the_reachable_ones' panicked:
  assertion `left == right` failed
    left: 127
   right: 15
```

Diagnosed to the real cause (the transcription genuinely shipped more content, not a test bug),
fixed by updating each test's pinned numbers with a doc comment carrying the re-derive command,
reran GREEN (see §7).

## 7. Suites run

```
cargo build --locked --lib                                                          # clean
cargo test  --locked --lib monster                                                  # 82 passed, 0 failed (was 80/2 RED before the fix)
cargo test  --locked --lib                                                          # 2409 passed, 0 failed, 13 ignored
cargo test  --locked --bin v06_work_inventory                                       # 335 passed, 0 failed
cargo test  --locked --bin gen_book_cache                                           # 3 passed, 0 failed
cargo build --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins       # clean
cargo test  --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins monster     # 31 passed, 0 failed
cargo test  --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins reach_gate  # 31 passed, 0 failed (incl. the 3 corpus-wide invariant tests)
cargo run   --locked --release --bin corpus_literal_sweep                           # 26538 records examined, 255839 tokens compared, 0 findings — CLEAN
cargo run   --locked --release --bin pi_sweep_rules_tables                          # 10 hits, 10 baseline rows, 0 new — CLEAN
cargo run   --locked --release --bin v06_corpus_trap_report -- --audit              # 0 of the 190 new files flagged (checked by filename cross-reference against `git status --porcelain`)
```

## 8. Reachability, proven live not asserted (brief item 5)

`reach_gate.rs`'s `chassis_monster_abilities_reach` is already fully generic — it reads
`corpus_record_keys(corpus_book, "monster_ability")` live off disk and cross-checks against a live
`build_monster_catalog()` IPC-shaped response; no per-book wiring exists to add. Re-running its own
test suite after the regen (`reach_gate` bin tests above) exercised exactly the 190 new records with
**zero new findings required** — `every_declared_claim_actually_carries_the_records`,
`unreached_records_are_exactly_the_recorded_findings`, and
`unsurfaced_families_are_exactly_the_recorded_findings` all stayed GREEN, meaning every one of the
190 new records reaches the live catalog response with no gap needing an `OPEN_FINDINGS`-style
carve-out (unlike the T9 spell cycle's 2-of-72 cross-book dedup finding — no analogous case here).

## 9. §15 — no Product Identity record encountered outside the signed-off disposition

Both regenerated books' PI screens ran independently of the T9 policy disposition and agreed with
it: `bestiary_4` dropped the same 14 monster rows (and cascaded their abilities via the orphan pass)
its own `MonsterBookSpec::product_identity_source` already documents; `ultimate_psionics` dropped
none (matching its own documented "zero PI rows" finding). No record was reached this cycle that
this cycle believed carried Product Identity despite its `clear` disposition; nothing was stopped
on.

## 10. Gate 3's `no_record` figure, re-derived (brief item 7) — NOT repinned

```bash
scripts/verify.sh --only shape-coverage-standing-gate
```
```
PASS  shape-coverage-standing-gate  (population=36028 unclassified=0 no_record=21349 corpus_sha=7f818006e371188e5717fd18d74d18a420747fc6)
```

Reads the **committed** `docs/work-inventory.json`, which this cycle did NOT regenerate — same
reason the prior `t9-onboarding` cycle and the standing near-miss warning both give:
`v06_work_inventory` fail-closed-refuses without `CORPUS_LITERAL_SWEEP_REPORT`/
`DERIVED_FIXTURE_CHECK_REPORT` set, and forcing past that on a prompt's authority alone is exactly
the shortcut this program's own near-miss incident forbids. This `no_record=21349` is the
**pre-existing** state (already improved from the prior cycle's `21497` by concurrent sibling-lane
work on the shared branch, not by this cycle), not moved by this cycle's 190-record addition, which
is not reflected in the checked-in inventory until a future regen cycle runs. **Budget constants in
`shape_coverage_standing_gate.py` left untouched**, as instructed — that repin belongs to a
dedicated, evidence-gated cycle.

## 11. What remains (explicit)

- **5 `monster_ability` books need `MonsterAbilityFacet` widened**: `bestiary` (92 clear, 12
  reachable-without-widening + 80 needing it — recompute after), `bestiary_2` (117), `bestiary_3`
  (629, the single largest remaining population in this shape), `inner_sea_bestiary` (33),
  `inner_sea_gods` (5). Combined 876 clear units. The chassis needs new facet variant(s) for a bare
  `TYPE:SpellLike` (delivery stated, no facet — likely defaults to `SpecialAttack` but that is a
  modelling call this cycle did not make unilaterally) and for `Weakness.Extraordinary`/`Internal`/
  `Communicate.Supernatural` (need their own read against the corpus to classify correctly). This is
  a real, separate, corpus-wide-blast-radius cycle — every consumer of `MonsterAbilityFacet` (corpus
  JSON `data.facet`, `monster_catalog.rs`'s rendering, any downstream `facet` match) needs
  re-verification against the widening, per `decisions.md §16`'s own caution against a naive
  widening being unsafe without adversarial checking.
- **`feat` (~397), `equipment` (~48), `companion` (~4), `monster` (~7)**: untouched this cycle. The
  prior `t9-onboarding` cycle's own per-kind figures stand; whether a `transcribe_monster_tables.py`-
  shaped generic mechanism already exists for any of them (this cycle's central correction suggests
  checking before building) is the first question a next cycle on those kinds should answer.
