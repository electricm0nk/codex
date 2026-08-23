# Cycle epic-6-kind-trait/3 — Gate 3 closure invariant / Epic 6, `kind: trait` (`decisions.md §25`)

- **Card ID:** `epic-6-kind-trait` (row 16)
- **Actor:** `t9-onboarding`
- **Base:** worktree reset to pinned `PIN=80329736f49de74ed6659c452d9f07b355500b40` (this cycle's own
  worktree had drifted onto an unrelated pre-tranche/12 lineage — `git merge-base --is-ancestor`
  failed on first check; `git reset --hard "$PIN"` recovered it, re-verified `PIN_OK`,
  `git rebase origin/tranche/12` then a no-op since PIN == `origin/tranche/12` HEAD at cycle start).
  Picks up commit `11a84bced5` (`fix(sd32): shape_ledger.py kind-aware join; run ingest_generic_kind
  --kind trait`) — the blocker cycle 2 named.
- **Commit SHA:** (this cycle's own commit, see push log)
- **Files touched:**
  - `src/rules_core/trait_pool.rs` — `load_trait_pool` no longer reads an `ability/` fallback
    directory; reads only `trait_generic/` (the modelled `kind: trait` write). Doc comments updated
    to record the retirement and why it is safe (§2). 2 tests renamed/rewritten to match the current
    real corpus state (no functional test-behavior regression — same assertions, updated rationale).
  - `apps/desktop/src-tauri/src/race_trait_picker.rs` — doc comment on
    `the_menu_command_carries_all_fourteen_adopted_race_options_thirteen_with_real_grants` updated;
    the test's own assertions were unchanged (already correct against real `kind: trait` content).
  - `apps/desktop/src-tauri/src/reach_gate.rs` — `BARE_RECORD_FINDINGS`'s Rougarou entry comment
    updated to cite the retired fallback and the direct oracle evidence (§3); the finding tuple
    itself (`("bestiary_6", "race_traits", &["Adopted Race ~ Rougarou"])`) is unchanged.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own diff, `git diff --unified=0 HEAD -- <touched
  files>`, scoped per §6 step 2's own warning).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope; `python3 .scratch-sd32/pi_check_diff.py`
  and a direct grep for todo/fixme/unimplemented/stub/placeholder over the added lines both clean).
- **Acceptance criterion (verbatim, `decisions.md §25`):** the 14 `adopted_race_choose_selector`
  units close by real ingest — a new `kind: trait` schema, an ingest tool, a reach-gate family, a
  character-builder picker, and `player_companion` book onboarding.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
  bootstrapped fresh this cycle — a fresh worktree's oracle slot is git-ignored/empty; confirmed via
  `scripts/verify.sh --only preflight-oracle` → PASS after `scripts/fetch-pcgen-oracle.sh`).
- **Status:** `complete`. All 14 units now resolve through the real, modelled `kind: trait` write
  (`data/corpus/*/trait_generic/*.json`), with the `ability/`-fallback retired — the blocking
  condition cycle 2's receipt named as the open question is now closed by the sibling join fix +
  real ingest, and this cycle finished the wiring: point the consumer at the real write, retire the
  workaround, re-prove all 14 end-to-end. 13 of 14 carry a real resolved grant; the 14th (Rougarou)
  is a hard impossibility of source data (`decisions.md §27b`), evidenced directly (§3).
- **Notes:** see full account below.

## 0. Re-derivation of every figure this cycle was handed (`decisions.md §17a`)

- **"487 `kind: trait` records now exist under `data/corpus/*/trait_generic/`"** — re-derived two
  ways: (1) `docs/work-inventory.json`'s own `totals.by_kind.trait` = **487**. (2) Direct filesystem
  count of files under exactly `data/corpus/<book>/trait_generic/*.json` (glob on the literal
  directory name, not a substring match — a substring match against `*trait_generic*` also catches
  the unrelated `race_trait_generic/` directory and over-counts to 2,371; caught and corrected before
  trusting it, per this bundle's own "validate the instrument" discipline). Exact match: **487**,
  broken down `ultimate_campaign` 154 / `inner_sea_gods` 115 / `inner_sea_races` 96 /
  `advanced_players_guide` 90 / `ultimate_psionics` 32. Confirmed correct.
- **The 14-unit selector population** — re-ran `python3 scripts/t2b_adoptive_parentage_census.py`
  against the freshly-bootstrapped pinned oracle: **14**, `bestiary_2` 7 / `bestiary_3` 5 /
  `bestiary_5` 1 / `bestiary_6` 1 — unchanged from all three prior re-derivations across cycles 1-2.
- **Cycle 1's stale "566 units across 6 books" figure** (`advanced_players_guide` 90, `core_rulebook`
  1, `ultimate_campaign` 231, `ultimate_psionics` 32, `inner_sea_gods` 116, `inner_sea_races` 96) is
  a pre-ingest CENSUS count (every row the classifier tagged `Kind::Trait`, before the ledger-gated
  ingest tool's own filtering — e.g. `no_formula_tokens` rows, or a licensing/dedup rule inside
  `ingest_generic_kind.py`, can legitimately drop a census-tagged row from the real write). This
  cycle does not own that gap (566 vs 487) and it is out of scope for this epic's acceptance
  criterion, which is about the 14 Adopted-Race selector units, not the Trait kind's total census
  count — named here only so a future reader does not mistake it for something this cycle missed.
- **13/14-pool-membership** — re-checked directly against the real `trait_generic/` corpus (not
  reused from the prior cycle's `ability/`-fallback finding): a script scanning every
  `trait_generic/*.json` record's `TYPE:Trait.RaceTrait.<X> Race Trait` token confirms **exactly 1**
  matching pool member for each of the 13 real target races (Dhampir, Fetchling, Grippli, Ifrit,
  Oread, Sylph, Undine, Catfolk, Ratfolk, Suli, Vanara, Vishkanya, Skinwalker) and **0** for Rougarou.

## 1. The `ability/` fallback is now provably redundant, not just superseded

Before touching `trait_pool.rs`, compared the full `RaceTrait`-tagged population under both
directories corpus-wide (a script reading every `data/corpus/*/trait_generic/*.json` and
`data/corpus/*/ability/*.json` record, filtering to `TYPE:` values containing `RaceTrait`, keyed by
`data.key`):

```
trait_generic RaceTrait-tagged keys: 124
ability        RaceTrait-tagged keys: 124
only in trait_generic (not in ability): 0
only in ability (not in trait_generic): 0
in both (exact duplicate): 124
```

Zero keys exist in `ability/` that are absent from `trait_generic/` — the fallback's entire
population is now a strict, exact duplicate of the real `kind: trait` write. Reading only
`trait_generic/` therefore loses no content. This is the concrete confirmation the `next-cycle plan`
in cycle 2's receipt asked for before retiring the fallback.

## 2. `trait_pool.rs` — fallback retired, single source directory

`load_trait_pool` now scans exactly one directory per book, `trait_generic/`. The `ability/` scan
branch, its dedup-by-key logic (no longer needed with one source), and the doc comment explaining why
a fallback existed are all removed/replaced with a comment recording the retirement and the §1
verification that makes it safe. Two integration tests renamed/rewritten to describe the current real
state (one book with no `trait_generic/` directory contributes nothing without panicking; the real
`inner_sea_races/trait_generic/trait_loner_of_the_rocks.json` record is found and resolved) — same
assertions as before, since the underlying corpus content at that key is byte-identical to what the
fallback used to read (§1).

## 3. Rougarou — hard impossibility of source data, evidenced directly (`decisions.md §27b`)

Read the pinned oracle's own Rougarou selector row directly (not re-used from a prior cycle's
finding):

```
$ grep -n "Rougarou Race Trait\|CHOOSE\|Adopted Race" \
    docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/rougarou/rougarou_abilities_race.lst

29:Rougarou    KEY:Adopted Race ~ Rougarou  ...  CHOOSE:ABILITYSELECTION|Special Ability|TYPE=Rougarou Race Trait  ABILITY:Traits|VIRTUAL|%LIST
30:CATEGORY=Special Ability|No Race Trait Available.MOD    TYPE:Rougarou Race Trait
```

The oracle's own row 30 is a placeholder PCGen ships to make the `CHOOSE:` selection UI show a
literal "No Race Trait Available" instead of an empty list — PCGen's own authors modelled zero
Rougarou Race Trait pool members. A corpus-wide grep for `TYPE:Trait.RaceTrait.Rougarou` /
`RaceTrait.Rougarou` across the entire pinned oracle returns **zero** matches anywhere. This is not a
gap in ingest coverage; the source data itself contains no such content to ingest — the one
admissible exemption `decisions.md §27b` names ("the source data does not exist"). Rougarou's
`adopted_race_options` entry resolves to an empty `grants` list, correctly, and stays the one
`BARE_RECORD_FINDINGS` entry (`reach_gate.rs`) — not silently dropped, not fabricated, named.

## 4. Verification run (this cycle)

```
cargo build --locked --lib                                              # clean, pre-existing warnings only
cargo test  --locked --lib trait_pool                                   # 7 passed, 0 failed
cargo test  --locked --lib race_resolver                                # 28 passed, 0 failed
cargo test  --locked --bin ingest_race_traits                           # 21 passed, 0 failed
cargo build --locked --manifest-path apps/desktop/src-tauri/Cargo.toml  # clean
cargo test  --locked --manifest-path apps/desktop/src-tauri/Cargo.toml race_trait_picker
                                                                          # 19 passed, 0 failed
                                                                          #  (incl. the pinned
                                                                          #   all-14/13-real-grants
                                                                          #   integration test)
cargo test  --locked --manifest-path apps/desktop/src-tauri/Cargo.toml race_catalog
                                                                          # 18 passed, 0 failed
cargo test  --locked --manifest-path apps/desktop/src-tauri/Cargo.toml reach_gate
                                                                          # 23 passed / 8 failed --
                                                                          # IDENTICAL count to cycle
                                                                          # 2's own baseline
                                                                          # (occult_adventures/
                                                                          # companion-catalog gaps,
                                                                          # other lanes' territory,
                                                                          # confirmed by reading each
                                                                          # failure's own message --
                                                                          # none mention Trait/Adopted
                                                                          # Race/Rougarou)
git status --porcelain                                                  # 3 source files modified,
                                                                          # 0 corpus writes, 0
                                                                          # deletions (+ 2 pre-existing
                                                                          # retro-log auto-appends from
                                                                          # this cycle's own
                                                                          # verify.sh --only
                                                                          # preflight-oracle runs)
```

`data/corpus/**` was never written by this cycle (read-only throughout) — no guarded-path regen, no
`--allow-stamp-loss`, no stamp risk.

## 5. Closure/reclassification/reachability/instrument-correction (`decisions.md §16`)

- **Closed by real ingest under the modelled `kind: trait` schema: 14 of 14.** All 14 selector units'
  resolution path reads only `data/corpus/*/trait_generic/*.json` — no workaround, no fallback.
- **Reclassified: 0.**
- **Reachability: 13 of 14 with a real resolved grant end-to-end through the real menu command; 1
  (Rougarou) identity-only, genuinely and provably empty (§3) — a hard impossibility of source data,
  not a gap.**
- **Instrument correction: 1.** A naive substring glob (`*trait_generic*`) over-counts by matching
  the unrelated `race_trait_generic/` directory (2,371 vs the real 487) — caught before it was
  trusted (§0).

## 6. PI discipline (`decisions.md §15`/`§19`/`§24`/`§24b`-2)

`python3 .scratch-sd32/pi_check_diff.py` (imports `scripts/pi_scrub.normalized_term_hits`, never
re-implements the blacklist) over every added line in this cycle's own diff: **zero hits**. A direct
grep for `todo|fixme|unimplemented|not.?implemented|stub|placeholder` over the same added lines:
zero hits. No corpus record was created, renamed, or transcribed this cycle.

## 7. Kanban

Row 16 (`epic-6-kind-trait`) set to `complete` — all 14 units resolve through the real, modelled
`kind: trait` write; the `ability/` fallback cycle 2 built as a workaround is retired; Rougarou's
empty pool is evidenced directly against the pinned oracle as a hard impossibility of source data.
