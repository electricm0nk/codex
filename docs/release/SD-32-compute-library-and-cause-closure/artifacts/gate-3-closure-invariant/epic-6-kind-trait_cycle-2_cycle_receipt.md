# Cycle epic-6-kind-trait/2 — Gate 3 closure invariant / Epic 6, `kind: trait` (`decisions.md §25`)

- **Card ID:** `epic-6-kind-trait` (row 16)
- **Actor:** `t9-onboarding`
- **Base:** `origin/tranche/12` at rebase time, `a32e235321` (post-rebase; pinned `PIN` for the
  dispatch was `3c7834101cf152cc86e016513a4e382248c833f5`).
- **Commit SHA:** (this cycle's own commit, see push log)
- **Files touched:**
  - `src/bin/ingest_race_traits.rs` — `BookSource` gains `selector_only: bool` +
    `extra_clear_races: &'static [&'static str]`; 4 new `selector_only` `BookSource` entries
    (`bestiary_2`/`bestiary_3`/`bestiary_5`/`bestiary_6`) pointing at the 13 real
    `core_essentials/races/<race>/<race>_abilities_race.lst` files that carry the 14 target
    selector rows; `ingest_book`'s scope filter and both scoped-clear loops widened accordingly;
    `ADOPTED_RACE_SELECTOR_TYPE`/`ADOPTED_RACE_SELECTOR_CHOOSE_PREFIX` moved to
    `race_resolver.rs` (imported, not duplicated — `ADOPTIVE_PARENTAGE_CATEGORY`'s own precedent);
    2 new fixture tests; the corpus-wide pinned-count test's `expected` map and total updated
    (569→669, +100 across the 4 new books — see receipt §2 for why the delta is 100, not 14).
  - `src/rules_core/race_resolver.rs` — `traits_by_type_token`, `AdoptedRaceSelector`,
    `adopted_race_choose_selectors` (parallels `adoptive_parentage_options`); 2 corpus-wide pinned
    tests updated (`no_corpus_trait_is_left_without_a_readable_gate`,
    `the_whole_corpus_classifies_into_the_four_roles_with_no_leftovers`: Unclassified 10→24, total
    831→845); 1 new integration test proving the real 14-unit population resolves correctly.
  - `src/rules_core/trait_pool.rs` — **new module.** `TraitPoolRecord`/`TraitPool`/
    `load_trait_pool` (book-agnostic `data/corpus/<book>/trait_generic/*.json` loader, modelled on
    `corpus_loader.rs`'s own `load_equipment_corpus`), `resolve_adopted_race_options` (the
    selector↔pool combining step, modelled on `adoptive_parentage_options`). 6 unit tests
    (empty-pool honesty, populated-pool exact match, malformed-selector flagging, real-corpus
    no-panic-on-empty-state).
  - `src/rules_core/mod.rs` — registers `trait_pool`.
  - `apps/desktop/src-tauri/src/race_catalog.rs` — `bestiary_3` added to `RACE_CORPUS_BOOKS` +
    `book_code` (contributes 0 catalog rows, so deliberately NOT added to `RACE_CATALOG_BOOKS`).
  - `apps/desktop/src-tauri/src/race_trait_picker.rs` — `AdoptedRaceTraitGrantDto`/
    `AdoptedRaceOptionDto`; `AlternateRacialTraitsResponse.adopted_race_options` (additive field);
    `build_menu` wires `adopted_race_choose_selectors` + `load_trait_pool` +
    `resolve_adopted_race_options`. 1 new integration test.
  - `apps/desktop/src-tauri/src/reach_gate.rs` — `("bestiary_3", "race_traits")` dispatch arm;
    `race_traits_reach` extended to count `adopted_race_options` (mirrors the
    `adoptive_parentage_options` block exactly); `BARE_RECORD_FINDINGS` gains 4 entries (14 keys
    total) — every one of this cycle's 14 new selector records, honestly `identity_only` today.
  - `data/corpus/bestiary_2/race_trait/{dhampir,fetchling,grippli,ifrit,oread,sylph,undine}/adopted_race_*.json`
    (7 new files, real `cargo run --bin ingest_race_traits -- bestiary_2` output).
  - `data/corpus/bestiary_3/race_trait/{catfolk,ratfolk,suli,vanara,vishkanya}/adopted_race_*.json`
    (5 new files).
  - `data/corpus/bestiary_5/race_trait/skinwalker/adopted_race_skinwalker.json` (1 new file).
  - `data/corpus/bestiary_6/race_trait/rougarou/adopted_race_rougarou.json` (1 new file).
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own diff, `git diff --unified=0 HEAD -- <touched
  files>` — the correctly-scoped form; the `BASE_BRANCH...` form returns unrelated pre-existing
  tagged lines from tranche/12's own prior work on these same files and is not a per-cycle signal,
  per §6 step 2's own warning).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope; the two raw substring hits on
  "placeholder"/"todo" are both negations — "never fabricating a placeholder trait",
  "must carry real corpus prose, not a fabricated placeholder", and a doc-path reference
  `.../todo/sweeps.md` inside a PRE-EXISTING comment the diff only reflowed — none is a stub marker).
- **Acceptance criterion (verbatim, `decisions.md §25`):** the 14 `adopted_race_choose_selector`
  units (`bestiary_2` 7, `bestiary_3` 5, `bestiary_5` 1, `bestiary_6` 1) close by real ingest — a new
  `kind: trait` schema, an ingest tool (extending an existing generic path), a reach-gate family, a
  character-builder picker, and `player_companion` book onboarding.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`, bootstrapped
  fresh this cycle — a fresh worktree's oracle slot is git-ignored and empty; confirmed via
  `scripts/verify.sh --only preflight-oracle` → PASS after `scripts/fetch-pcgen-oracle.sh`).
- **Status:** `in-progress` — real, tested, corpus-real progress landed on two of the epic's three
  named halves (selector-row ingest; the trait_pool/picker/reach-gate mechanism). The third half
  (the 566-unit Trait pool's own content) is still blocked by the same pre-existing,
  cross-bundle `corpus_literal_sweep` finding cycle 1 escalated. **0 of 14 units close this cycle**
  under the honest `decisions.md §16` standard: every option's `grants` is empty until real Trait
  pool content is ingested. Kanban row 16 stays `in-progress`.
- **Notes:** see full account below.
- **Discovery forwards:** none new. The `hidden_wand.json` finding cycle 1 escalated is unchanged
  (re-confirmed this cycle, see §5) and is still the one blocker on real closure.
- **Next-cycle plan:** regenerate `docs/work-inventory.json` (with fresh
  `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT`, no `--allow-stamp-loss`, full
  status-distribution diff) the moment `corpus_literal_sweep` reports clean, then run
  `python3 scripts/ingest_generic_kind.py --kind trait --ledger <shape_ledger output>` for the
  566-unit pool. No further mechanism work is needed — the resolver, DTO and reach-gate wiring built
  this cycle read whatever the pool loader finds; a re-run with real pool content in place will move
  every one of these 14 options from `identity_only` to `with_payload` with no further code change.

## 0. Re-derivation of the brief's own figures (`decisions.md §17a`)

Re-ran `python3 scripts/t2b_adoptive_parentage_census.py` against the freshly-bootstrapped pinned
oracle: **14** `adopted_race_choose_selector` units, `bestiary_2` 7 / `bestiary_3` 5 / `bestiary_5` 1
/ `bestiary_6` 1 — unchanged from both prior cycles' own re-derivations. Exact source file + line for
every one of the 14 confirmed directly against `docs/work-inventory.json`'s existing entries (all 14
are pre-existing `arg_flat_grant`/`adopted_race_choose_selector`-shape units the T2b census already
enumerated, so reading them needed no regen).

## 1. `ingest_races.rs`'s file-ownership boundary — read before writing anything (per the brief's own
   requirement)

Cycle 1 refused to add the 4 `BookSource` rows blind. Read `ingest_races.rs::main()`'s clear loop
(the actual ownership boundary, not a doc comment): it `remove_dir_all`s `<book>/race/` and
`<book>/race_trait/` **only** for `["core_rulebook", "beastiary", "bestiary_2", "bestiary_5",
"bestiary_6"]` — **`bestiary_3` is absent from this list**, confirmed by direct read, not assumed.
`ingest_races.rs::IN_SCOPE_RACES` also confirmed **all 13 non-Rougarou target races now have a real
chassis record** (Dhampir/Skinwalker/Rougarou all landed since cycle 1's pin — Dhampir by a sibling
SD-32 card-11 T2b lane, 2026-08-22/23; Skinwalker/Rougarou already landed in SD-31). Cycle 1's own
"3 races with no chassis" finding is therefore stale as of this cycle; corrected in both the receipt
(here) and in a code comment (`race_resolver.rs`'s new
`adopted_race_choose_selectors_finds_the_real_fourteen_unit_population` test).

**The real collision risk, verified directly:** all 14 selector rows physically live in the SAME
`<race>_abilities_race.lst` files `ingest_races.rs` already reads for standard-trait content
(`data/corpus/bestiary_2/race_trait/oread/oread_type.json` etc. already exist, written by that other
tool). Naively adding these files as `BookSource.lst_relatives` and admitting rows the ordinary way
(`in_scope.contains(race_key) || is_selector`) would ALSO admit every standard-trait row for the 11
of 14 races already in `IN_SCOPE_RACES` — re-parsing and re-writing content `ingest_races.rs` already
owns, into the SAME directory.

**Fix: `BookSource::selector_only`.** A `selector_only: true` book's scope filter drops the
`in_scope.contains(...)` branch entirely — only `row.is_adopted_race_choose_selector` can ever admit
a row. **Proved by mutation, not assumed:** temporarily bypassed the flag in the live production code
(`let admit = if false && selector_only { ... } else { in_scope.contains(...) || is_selector }`),
rebuilt, ran `cargo run --bin ingest_race_traits -- bestiary_2` for real. Result: **57 pre-existing
`ingest_races.rs`-owned files under `bestiary_2/race_trait/{oread,sylph,undine}/` were destructively
overwritten** with wrong-shape (`is_racial_default: false`) content — exactly the 1,580-near-miss
incident class card 1 found for a different generator pair. `git checkout -- data/corpus/bestiary_2`
reverted the corruption; the source mutation was reverted next; the real fix (`selector_only: true`)
re-verified clean (`git status --porcelain` shows only the 14 intended new files after the real run —
see §6).

`extra_clear_races` closes a second, smaller hazard the same read surfaced: `ingest_book`'s own
scoped-clear and on-disk self-check loops are keyed off `IN_SCOPE_RACES`, which never included
Dhampir/Skinwalker/Rougarou — so without this field their selector records would never be cleared on
a re-run with different content, and the binary's own `assert_eq!(on_disk, written, ...)` self-check
would panic the moment a `selector_only` book wrote a record for a race outside `IN_SCOPE_RACES` (it
did, immediately — this was caught by running the tool for real, not by inspection).

## 2. Real ingest, run for real, against the pinned oracle

`cargo run --locked --bin ingest_race_traits -- bestiary_2` / `bestiary_3` / `bestiary_5` /
`bestiary_6` (run individually, not via the bare `ingest_race_traits` invocation, which would also
re-stamp `ingested_at` on all 786 pre-existing records across every other book — confirmed by running
it once, seeing 456 files touched, and reverting via `git checkout -- data/corpus` before re-running
scoped). Output, all real:

```
bestiary_2: records emitted 7, distinct races 7 (Dhampir Fetchling Grippli Ifrit Oread Sylph Undine),
            PI-dropped 0, DESCISPI 0
bestiary_3: records emitted 5, distinct races 5 (Catfolk Ratfolk Suli Vanara Vishkanya),
            PI-dropped 0, DESCISPI 0
bestiary_5: records emitted 1 (Skinwalker), PI-dropped 0
bestiary_6: records emitted 1 (Rougarou), PI-dropped 0
```

Total: **14 new corpus records**, matching `decisions.md §25`'s own population figure exactly.
`git status --porcelain` after the scoped run: exactly the 14 new files, zero modifications, zero
deletions.

**Why the pinned corpus-wide count test's delta is 100, not 14.** That test
(`no_committed_trait_description_leaks_pcgen_syntax_in_any_declared_book`) walks the WHOLE
`<book>/race_trait/` directory with no ownership filter (unlike `ingest_book`'s own
`count_own_json`), the same way it already counts `advanced_race_guide`'s 421 (this binary's own
alternates PLUS `ingest_races.rs`'s standard-tier content sharing that directory). Re-derived by
direct count: `find data/corpus/bestiary_2/race_trait -name '*.json' | wc -l` → 76 (7 new selector +
69 pre-existing standard), `bestiary_3` → 5 (0 pre-existing — confirmed by §1's own read, this binary
owns the whole directory here), `bestiary_5` → 10 (1 + 9), `bestiary_6` → 9 (1 + 8). 76+5+10+9=100.

## 3. `trait_pool` — new module, the resolver half

`src/rules_core/trait_pool.rs`. Reads `data/corpus/<book>/trait_generic/*.json` (the exact directory
convention `ingest_generic_kind.py --kind trait` would write, confirmed by reading that script's own
`kind_dir_name = f"{kind}_generic"` line — matched, not guessed). Indexes by the record's own `TYPE:`
token's `Trait.RaceTrait.` prefix strip, matching `v06_work_inventory.rs::refine_kind` and
`census_independent.py::_row_is_pf1_trait`'s identical detection rule (cycle 1's own, re-used, not
re-derived). `resolve_adopted_race_options` combines a loaded `TraitPool` with
`race_resolver::adopted_race_choose_selectors`'s output — the same two-step shape
`adoptive_parentage_options` already uses for its own (different) pool.

**Honest by construction.** `resolve_adopted_race_options` never fabricates a grant: an empty pool
resolves to an empty `grants` list, proved by `resolving_an_empty_pool_is_honest_never_fabricated`. A
malformed `CHOOSE:` token is flagged (`malformed_choose_token: true`), never silently treated as an
empty pool. 6 unit tests total, including one against the REAL, current, contentless
`data/corpus/**/trait_generic/` state (0 records, no panic) — this is not a mocked assertion; it is
what the repo genuinely holds today.

## 4. `race_trait_picker.rs` / `reach_gate.rs` — the DTO and reach-gate halves

`AdoptedRaceOptionDto` (modelled directly on `AdoptiveParentageOptionDto`, the brief's own named
precedent) added to `AlternateRacialTraitsResponse.adopted_race_options` — additive field, no
existing consumer affected. `build_menu` wires `adopted_race_choose_selectors` +
`load_trait_pool(RACE_CORPUS_BOOKS)` + `resolve_adopted_race_options`, reading the SAME corpus root
the race corpus itself just loaded from.

`reach_gate.rs::race_traits_reach` extended with the exact block-shape
`adoptive_parentage_options` already uses (mirrors it line for line): an option with a non-empty
`grants` list is `with_payload`; empty is `identity_only`, never silently dropped. New
`("bestiary_3", "race_traits")` dispatch arm (bestiary_3 had none before — it contributed no
standard-tier content, so cycle 1/prior cycles never needed one).

**Corpus-real proof, not a synthetic fixture:** `the_menu_command_carries_all_fourteen_adopted_race_
options_with_no_pool_content_ingested_yet` calls the REAL `build_menu`/`race_corpus()` machinery
against the real, just-ingested 14 corpus files and asserts the exact 14 keys, correct book codes
(`B2`/`B3`/`B5`/`B6`), zero `malformed_choose_token`, and — honestly — zero `grants` (no pool content
ingested yet).

## 5. `docs/work-inventory.json` — still blocked, re-confirmed, not touched, not worked around

Re-ran `cargo run --bin corpus_literal_sweep -- --json-out <report>`: **same** `clean: false`, same
single finding at `data/corpus/inner_sea_magic/ability/hidden_wand.json`, same coordinate, same
recurrence key. This cycle's own diff touches **zero** files under `data/corpus/inner_sea_magic/**`
or `src/bin/corpus_literal_sweep.rs`, per the brief's explicit instruction (a sibling lane owns this
fix). `docs/work-inventory.json` itself: `git status --porcelain` clean on that path throughout this
cycle — no write attempted.

**Consequence for this epic, stated honestly:** the 566-unit Trait pool content
(`inner_sea_races/isr_abilities.lst` etc., cycle 1's own §1 finding) still cannot be ingested via
`ingest_generic_kind.py --kind trait`, because that path's `load_units` reads
`docs/work-inventory.json`'s `kind: trait` units, which do not exist in the committed JSON until it
regenerates. This is the ONE thing standing between "mechanism built and tested" and "all 14 units
genuinely closed." Considered and rejected: hand-rolling a parallel, non-`docs/work-inventory.json`
ingest path for the Trait pool specifically (analogous to how `ingest_race_traits.rs` itself bypasses
the census). Rejected because it would duplicate id-generation/corpus_key-derivation/join-semantics
logic the Rust census tool already owns, with real risk of producing a subtly wrong Gate-1 signal —
exactly the anti-gaming hazard `decisions.md §1a` exists to refuse. The safer, correct path is the
one already planned: regenerate for real once the sibling lane's fix lands, then run the existing
generic tool once.

## 6. What is NOT closed, and why (`decisions.md §16`)

**Closed by real ingest: 0 of 14.** **Reclassified: 0.** **Reachability: 0 with real payload, 14
identity-only** — every one of the 14 selector records is now a real, corpus-real, player-facing
`adoptedRaceOptions` row (proven by the integration test in §4), but every one's `grants` is honestly
empty pending the Trait pool's own regen-blocked ingest. This is real, measurable progress beyond
cycle 1's 0/0/0 — the shape recognition, the row parsing, AND the corpus write, the resolver, the DTO
and the reach-gate wiring are now all real and tested — but per `decisions.md §16`'s discipline, a
unit reaching a surface with only its key (no resolved grant) is not a unit closed, and this receipt
does not claim it is.

No stub was written: `adopted_race_options` returns real corpus data (14 real keys, real book
attribution), never a fabricated grant; the empty `grants` list is the honest current state of the
underlying data, not a placeholder the code invented. Kanban row 16 stays `in-progress`.

## 7. PI discipline (`decisions.md §15`/`§19`/`§24`)

`python3 scripts/pi_key_rawtokens_audit.py` scanned corpus-wide (unchanged scan, no code touched):
zero hits on any of this cycle's 14 new records. No PI exposure found or judged this cycle beyond the
pre-existing, already-escalated `hidden_wand.json` finding (§5, re-confirmed, not re-litigated).

## 8. Verification run (this cycle)

```
cargo build --locked --bin ingest_race_traits                                  # clean
cargo test  --locked --bin ingest_race_traits                                  # 21 passed (5 new), 0 failed
cargo test  --locked --lib race_resolver                                       # 28 passed (2 new + 2 updated), 0 failed
cargo test  --locked --lib trait_pool                                          # 6 passed (new module), 0 failed
cd apps/desktop/src-tauri && cargo build --locked                              # clean
cargo test --locked race_trait_picker                                          # 19 passed (1 new), 0 failed
cargo test --locked race_catalog                                               # 18 passed, 0 failed
cargo test --locked reach_gate                                                 # 23 passed / 8 pre-existing failures
                                                                                 # unrelated to this cycle's scope
                                                                                 # (companion/class/content-kind
                                                                                 # gaps from other lanes' already-
                                                                                 # committed work -- confirmed by
                                                                                 # `git log` on the affected corpus
                                                                                 # dirs predating this session;
                                                                                 # the one failure THIS cycle
                                                                                 # introduced -- bare_records_are_
                                                                                 # exactly_the_recorded_findings --
                                                                                 # was fixed, 9 -> 8)
cargo run --bin corpus_literal_sweep -- --json-out <report>                    # clean:false, 1 finding (§5, unrelated)
git status --porcelain                                                         # 14 new corpus files, 6 modified
                                                                                 # source files, 0 deletions
```

RED→GREEN, twice, both mutation-proved live and reverted: (1) `ADOPTED_RACE_SELECTOR_TYPE` mutated to
a non-matching string in `race_resolver.rs` — `adopted_race_choose_selectors_finds_the_real_fourteen_
unit_population` failed for the intended reason (empty result); reverted, re-ran, GREEN. (2)
`BookSource::selector_only`'s guard bypassed in `ingest_race_traits.rs`'s live production code — a
real corpus run destructively overwrote 57 pre-existing files; `git checkout` + source revert
restored clean state, re-ran, GREEN (full account in §1).

## 9. Rebase discipline

Rebased onto `origin/tranche/12` twice this cycle (once at start, picking up a doc-only commit; once
before finalizing — see push log for the SHA). Re-ran `cargo test --locked --bin ingest_race_traits`
and `cargo test --locked --lib race_resolver trait_pool` after each rebase per this bundle's own
"a test can pass on a stale binary" warning — own `CARGO_TARGET_DIR` used throughout
(`/home/ubuntu/.cache/codex-targets/sd32-t9-onboarding`), never a shared one.
