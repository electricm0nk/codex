# Cycle epic-6-kind-trait/2 — Gate 3 closure invariant / Epic 6, `kind: trait` (`decisions.md §25`)

- **Card ID:** `epic-6-kind-trait` (row 16)
- **Actor:** `t9-onboarding`
- **Base:** started at pinned `PIN=3c7834101cf152cc86e016513a4e382248c833f5`; rebased twice onto
  `origin/tranche/12` as it moved (final rebase picked up
  `978d215227` — the sibling `corpus_literal_sweep` fix and `docs/work-inventory.json` regen).
- **Commit SHA:** (this cycle's own commit, see push log)
- **Files touched:**
  - `src/bin/ingest_race_traits.rs` — `BookSource` gains `selector_only: bool` +
    `extra_clear_races: &'static [&'static str]`; 4 new `selector_only` `BookSource` entries
    (`bestiary_2`/`bestiary_3`/`bestiary_5`/`bestiary_6`) pointing at the 13 real
    `core_essentials/races/<race>/<race>_abilities_race.lst` files that carry the 14 target
    selector rows; `ingest_book`'s scope filter and both scoped-clear loops widened accordingly;
    `ADOPTED_RACE_SELECTOR_TYPE`/`ADOPTED_RACE_SELECTOR_CHOOSE_PREFIX` moved to `race_resolver.rs`
    (imported, not duplicated — `ADOPTIVE_PARENTAGE_CATEGORY`'s own precedent); 2 new fixture
    tests; the corpus-wide pinned-count test's `expected` map and total updated (569→669, +100
    across the 4 new books — see §2 for why the delta is 100, not 14).
  - `src/rules_core/race_resolver.rs` — `traits_by_type_token`, `AdoptedRaceSelector`,
    `adopted_race_choose_selectors` (parallels `adoptive_parentage_options`); 2 corpus-wide pinned
    tests updated (`no_corpus_trait_is_left_without_a_readable_gate`,
    `the_whole_corpus_classifies_into_the_four_roles_with_no_leftovers`: Unclassified 10→24, total
    831→845); 1 new integration test proving the real 14-unit population resolves correctly, and
    that all 14 now have a real chassis record (correcting cycle 1's now-stale "3 races with no
    chassis" finding — Dhampir landed via a sibling lane since cycle 1's pin).
  - `src/rules_core/trait_pool.rs` — **new module.** `TraitPoolRecord`/`TraitPool`/
    `load_trait_pool` (book-agnostic loader modelled on `corpus_loader.rs`'s own
    `load_equipment_corpus`), `resolve_adopted_race_options` (the selector↔pool combining step,
    modelled on `adoptive_parentage_options`). Reads **two** source directories per book —
    `trait_generic/` (the `ingest_generic_kind.py --kind trait` convention, empty today) and
    `ability/` (a real fallback — see §3) — deduplicated by key. 7 unit tests, including one
    against the real, on-disk `inner_sea_races` corpus proving a real pool member resolves.
  - `src/rules_core/mod.rs` — registers `trait_pool`.
  - `apps/desktop/src-tauri/src/race_catalog.rs` — `bestiary_3` added to `RACE_CORPUS_BOOKS` +
    `book_code` (contributes 0 catalog rows, so deliberately NOT added to `RACE_CATALOG_BOOKS`).
  - `apps/desktop/src-tauri/src/race_trait_picker.rs` — `AdoptedRaceTraitGrantDto`/
    `AdoptedRaceOptionDto`; `AlternateRacialTraitsResponse.adopted_race_options` (additive field);
    `build_menu` wires `adopted_race_choose_selectors` + `load_trait_pool` +
    `resolve_adopted_race_options`. 1 new integration test proving all 14 keys, 13 with a real
    resolved grant (pinned by exact prose for one), 1 (Rougarou) honestly empty.
  - `apps/desktop/src-tauri/src/reach_gate.rs` — `("bestiary_3", "race_traits")` dispatch arm;
    `race_traits_reach` extended to count `adopted_race_options` (mirrors the
    `adoptive_parentage_options` block exactly); `BARE_RECORD_FINDINGS` gains exactly **1** entry
    (`Adopted Race ~ Rougarou`) — the only one of the 14 that stays `identity_only`.
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
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope; the only raw substring hits are
  negations — "never fabricating a placeholder trait", "not a fabricated placeholder" — and a
  doc-path reference inside pre-existing, unmodified text the diff only reflowed).
- **Acceptance criterion (verbatim, `decisions.md §25`):** the 14 `adopted_race_choose_selector`
  units (`bestiary_2` 7, `bestiary_3` 5, `bestiary_5` 1, `bestiary_6` 1) close by real ingest — a new
  `kind: trait` schema, an ingest tool (extending an existing generic path), a reach-gate family, a
  character-builder picker, and `player_companion` book onboarding.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`, bootstrapped
  fresh this cycle — a fresh worktree's oracle slot is git-ignored and empty; confirmed via
  `scripts/verify.sh --only preflight-oracle` → PASS after `scripts/fetch-pcgen-oracle.sh`).
- **Status:** `in-progress` — **13 of the 14 units now reach a real, corpus-real, resolved grant**
  end-to-end through the real menu command (proven by integration test, §4). The 14th
  (Rougarou) is honestly empty because PCGen's own oracle grants it no Race Trait pool anywhere
  (cycle 1's own corpus-wide-proven-empty finding, re-confirmed). Under `decisions.md §16`'s
  discipline this receipt still reports **0 of 14 formally "closed by real ingest"** — the pool
  content the 13 real grants read from was found via a fallback read of `kind: ability` records
  (§3), not written under `kind: trait` by this cycle's own ingest, so the schema half of the
  acceptance criterion ("a new `kind: trait` schema... ingest tool") is not yet satisfied by a
  committed write this cycle can point to. Kanban row 16 stays `in-progress`; the operator/next
  cycle should read §3 and §6 before deciding whether 13/14 real-reachable is a closure or whether
  the `kind: trait` write must land first.
- **Notes:** see full account below.
- **Discovery forwards:** one new finding, superseding cycle 1's `hidden_wand.json` escalation
  (that finding is now FIXED — see §2). **`shape_ledger.py`'s `(book, source_file, source_line)`
  join is kind-blind**, so it reports every one of the 487 `kind: trait` census units as
  `matched`/`no_formula_tokens` (never `no_record`) because a PRE-`Kind::Trait` ingest pass already
  wrote a `kind: ability` record at the identical coordinate. This makes
  `ingest_generic_kind.py --kind trait`'s `no_record`-ledger-gated discovery see **zero** units to
  ingest, even though no `kind: trait` record exists anywhere. Logged:
  `scripts/retro.py incident` id `1787508202265-t9-onboarding-b6d30e`,
  `docs/retro/events/t9-onboarding.jsonl`, recurrence-key
  `shape-ledger-kind-blind-join-hides-trait-population`.
- **Next-cycle plan:** a real decision, not a mechanism gap — this cycle's own machinery (resolver,
  DTO, reach-gate) already reads whichever directory holds the content, so no further code is
  strictly required for reachability. What remains is a choice: (a) fix `shape_ledger.py`'s join to
  be kind-aware (repo-wide blast radius — affects every kind's Gate-1 measurement, not just
  `trait`, so this needs its own dedicated, adversarially-reviewed cycle, not a drive-by fix here),
  then run `ingest_generic_kind.py --kind trait` for a real `kind: trait` write that formally
  retires the stale `ability` duplicates; or (b) an operator ruling that the `ability/`-fallback
  read this cycle built is an acceptable, permanent second source for `trait_pool` (in which case
  row 16 can close on today's 13/14-real-reachable state, with Rougarou's genuine emptiness named
  and accepted). Both are named; neither was picked unilaterally by this cycle.

## 0. Re-derivation of the brief's own figures (`decisions.md §17a`)

Re-ran `python3 scripts/t2b_adoptive_parentage_census.py` against the freshly-bootstrapped pinned
oracle: **14** `adopted_race_choose_selector` units, `bestiary_2` 7 / `bestiary_3` 5 / `bestiary_5` 1
/ `bestiary_6` 1 — unchanged from both prior cycles' own re-derivations.

## 1. `ingest_races.rs`'s file-ownership boundary — read before writing anything

Read `ingest_races.rs::main()`'s clear loop directly (the actual ownership boundary, not a doc
comment): it `remove_dir_all`s `<book>/race/` and `<book>/race_trait/` **only** for
`["core_rulebook", "beastiary", "bestiary_2", "bestiary_5", "bestiary_6"]` — **`bestiary_3` is
absent**, confirmed by direct read. `ingest_races.rs::IN_SCOPE_RACES` also confirmed **all 13
non-Rougarou target races now have a real chassis record** (Dhampir landed via a sibling SD-32
card-11 T2b lane between cycle 1 and this one; Skinwalker/Rougarou were already there from SD-31).
Cycle 1's "3 races with no chassis" finding is stale as of this cycle — corrected here and in a
code comment (`race_resolver.rs`'s `adopted_race_choose_selectors_finds_the_real_fourteen_unit_
population` test).

**The real collision risk, verified directly:** all 14 selector rows physically live in the SAME
`<race>_abilities_race.lst` files `ingest_races.rs` already reads for standard-trait content.
**Fix: `BookSource::selector_only`** — a `selector_only: true` book's scope filter drops the
`in_scope.contains(...)` branch entirely. **Proved by mutation, not assumed:** temporarily bypassed
the flag in the live production code, rebuilt, ran `cargo run --bin ingest_race_traits --
bestiary_2` for real. Result: **57 pre-existing `ingest_races.rs`-owned files** under
`bestiary_2/race_trait/{oread,sylph,undine}/` were **destructively overwritten**. `git checkout --
data/corpus/bestiary_2` reverted the corruption; the source mutation was reverted next; the real
fix re-verified clean.

`extra_clear_races` closes a second hazard: `ingest_book`'s scoped-clear/self-check loops are keyed
off `IN_SCOPE_RACES`, which never included Dhampir/Skinwalker/Rougarou — without this field their
records would never be cleared on a re-run, and the binary's own `assert_eq!(on_disk, written)`
self-check panicked immediately when caught live.

## 2. `docs/work-inventory.json` — the OLD blocker is fixed; re-confirmed, not re-worked

Cycle 1 escalated `data/corpus/inner_sea_magic/ability/hidden_wand.json` (a suspected PI-redaction
inconsistency blocking every regen). **A sibling lane fixed it this cycle**
(`origin/tranche/12` commit `978d215227`'s predecessor, landed mid-cycle): `corpus_literal_sweep`
now reports **CLEAN, 0 findings**, and `docs/work-inventory.json` was regenerated through the
guarded path with the full before/after stamp-set diffed identical (8,247 literal+fixture-verified
ids unchanged). Picked up by this cycle's second rebase; re-confirmed by direct read of the
regenerated file (487 `kind: trait` units now present, none in it before).

## 3. Real ingest, run for real, against the pinned oracle — the selector rows

`cargo run --locked --bin ingest_race_traits -- bestiary_2` / `bestiary_3` / `bestiary_5` /
`bestiary_6` (run individually — the bare invocation would also re-stamp `ingested_at` on all 786
pre-existing records across every other book; caught by running it once, seeing 456 files touched,
reverting via `git checkout -- data/corpus`, and re-running scoped). Real output:

```
bestiary_2: records emitted 7 (Dhampir Fetchling Grippli Ifrit Oread Sylph Undine), PI-dropped 0
bestiary_3: records emitted 5 (Catfolk Ratfolk Suli Vanara Vishkanya), PI-dropped 0
bestiary_5: records emitted 1 (Skinwalker), PI-dropped 0
bestiary_6: records emitted 1 (Rougarou), PI-dropped 0
```

Total: **14 new corpus records**, exactly `decisions.md §25`'s population. `git status --porcelain`
after the scoped run: exactly the 14 new files, zero modifications, zero deletions.

## 4. `trait_pool` — the resolver half, and a real new finding

Built `src/rules_core/trait_pool.rs` to read `data/corpus/<book>/trait_generic/*.json` — the exact
convention `ingest_generic_kind.py --kind trait` writes (confirmed by reading that script's
`kind_dir_name = f"{kind}_generic"` line). With §2's blocker cleared, ran
`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output <ledger>` for real to
drive `ingest_generic_kind.py --kind trait --ledger <ledger>`.

**Found: `shape_ledger.py`'s join is kind-blind.** All 487 `kind: trait` census units report
`join_status` of `matched` (180) or `no_formula_tokens` (307) — **zero** `no_record`. Verified this
is a false positive, not real coverage: `find data/corpus -type d -name trait_generic` returns
nothing anywhere, but `data/corpus/inner_sea_races/ability/loner_of_the_rocks.json` (a real,
pre-existing corpus record, `ingested_at: 2026-08-23T14:17:00Z`, predating this cycle) carries
`KEY:Trait ~ Loner of the Rocks`, `TYPE:Trait.RaceTrait.Oread Race Trait` — **the exact real
content `epic-6-kind-trait_cycle-1_cycle_receipt.md §1` named** (`isr_abilities.lst:78`), sitting
under `kind: ability` from an ingest pass that ran before `Kind::Trait` existed. `shape_ledger.py`'s
join key is `(book, source_file, source_line)` alone (confirmed by direct read of
`build_corpus_index`), so it matches the trait census unit against this wrong-kind record and never
reports it `no_record` — meaning `ingest_generic_kind.py --kind trait` would find **zero** units to
process, permanently, until this is fixed.

**Considered and rejected:** fixing `shape_ledger.py`'s join to be kind-aware in this cycle. That
function is shared by every kind's Gate-1 measurement — a blast radius far outside this epic's
scope, and the kind of repo-wide change that needs its own dedicated, adversarially-reviewed cycle,
not a drive-by fix buried in a 14-unit epic.

**What this cycle did instead: read the real content anyway.** `load_trait_pool` scans **two**
directories per book — `trait_generic/` (the correct future home, empty today) and `ability/` (a
real, honest fallback for TYPE-`RaceTrait`-tagged records already on disk), deduplicated by key,
`trait_generic` winning on collision. This is a read, not a write: `data/corpus/**` stays untouched
by this cycle (guarded path discipline unbroken), and the loader makes no claim the `ability` filing
is correct — only that the bytes are real and worth surfacing rather than leaving invisible.

## 5. `race_trait_picker.rs` / `reach_gate.rs` — the DTO and reach-gate halves, proven real

`AdoptedRaceOptionDto` (modelled on `AdoptiveParentageOptionDto`) wired into `build_menu`. Real,
corpus-real integration test result: **13 of 14 options resolve exactly 1 real grant each** (the
inner_sea_races pool member for that race), 1 (Rougarou) honestly empty. Pinned by exact prose for
one (`Adopted Race ~ Oread` → `"Loner of the Rocks"`, full description text). `reach_gate.rs`'s
`race_traits_reach` extension (mirrors `adoptive_parentage_options`'s own block) now correctly
counts 13 as `with_payload`; `BARE_RECORD_FINDINGS` shrank to its true size — 1 entry
(`Adopted Race ~ Rougarou`).

## 6. What is NOT closed, and why (`decisions.md §16`)

**Closed by real ingest under a NEW `kind: trait` schema: 0 of 14.** **Reclassified: 0.**
**Reachability: 13 of 14 with a real resolved grant, 1 (Rougarou) identity-only-but-genuinely-
proven-empty.** This is real, substantial progress beyond cycle 1's 0/0/0 and beyond this cycle's
own mid-point state (14 real selector records, but 0 grants) — but it is reached via a fallback read
of pre-existing `kind: ability` records, not via this cycle writing `kind: trait` records the way
`decisions.md §25` literally specifies ("a new `kind: trait` schema... an ingest tool"). Whether
13-of-14-real-reachable via this route satisfies the epic, or whether the formal `kind: trait` write
must still land (blocked on the `shape_ledger.py` fix named in §4), is an operator-level call this
receipt surfaces rather than makes. Kanban row 16 stays `in-progress` either way.

No stub was written: `adopted_race_options` returns real corpus data for all 14 keys, and 13 of them
carry a real, resolved, corpus-real grant — never a fabricated one.

## 7. PI discipline (`decisions.md §15`/`§19`/`§24`)

`python3 scripts/pi_key_rawtokens_audit.py`: zero hits on any of this cycle's 14 new records.
`python3 -c "... pi_scrub.normalized_term_hits ..."` over every touched source file: hits only in
PRE-EXISTING lines this cycle did not add (`git diff --unified=0 HEAD` confirms zero blacklist terms
in any line this cycle's own diff introduces). No PI exposure found or judged this cycle.

## 8. Verification run (this cycle)

```
cargo build --locked --bin ingest_race_traits                    # clean
cargo test  --locked --bin ingest_race_traits                    # 21 passed (5 new), 0 failed
cargo test  --locked --lib race_resolver                         # 28 passed (2 new + 2 updated), 0 failed
cargo test  --locked --lib trait_pool                             # 7 passed (new module), 0 failed
cd apps/desktop/src-tauri && cargo build --locked                # clean
cargo test --locked race_trait_picker                             # 19 passed (1 new), 0 failed
cargo test --locked race_catalog                                  # 18 passed, 0 failed
cargo test --locked reach_gate                                    # 23 passed / 8 pre-existing failures
                                                                    # unrelated to this cycle (companion/
                                                                    # class/content-kind gaps from other
                                                                    # lanes' already-committed work,
                                                                    # confirmed via `git log` predating
                                                                    # this session; the one failure this
                                                                    # cycle itself introduced was fixed,
                                                                    # 9 -> 8, then stayed fixed after the
                                                                    # BARE_RECORD_FINDINGS shrink)
cargo run --bin corpus_literal_sweep -- --json-out <report>       # CLEAN, 0 findings (§2)
git status --porcelain                                            # 14 new corpus files, 6 modified
                                                                    # source files, 0 deletions
```

RED→GREEN, twice, both mutation-proved live and reverted: (1) `ADOPTED_RACE_SELECTOR_TYPE` mutated
to a non-matching string — `adopted_race_choose_selectors_finds_the_real_fourteen_unit_population`
failed for the intended reason; reverted, GREEN. (2) `BookSource::selector_only`'s guard bypassed in
live production code — a real corpus run destructively overwrote 57 pre-existing files; reverted,
GREEN (§1).

## 9. Rebase discipline

Rebased onto `origin/tranche/12` twice (once picking up a doc-only commit, once picking up the
`corpus_literal_sweep` fix + regen — §2). Re-ran `cargo test --locked --bin ingest_race_traits` and
`cargo test --locked --lib race_resolver trait_pool` after each rebase, per this bundle's own
"a test can pass on a stale binary" warning — own `CARGO_TARGET_DIR` used throughout.
