# Cycle t9-monster-ability-name-pi-desc-pi-round7 — Gate 3 (closure invariant) / Card 11 (T9)

- **Card ID:** 11 (`epic-2-cause-closure`, T9 sub-lane
  `t9-monster-ability-name-pi-desc-pi-round7`)
- **Files touched:**
  - `scripts/transcribe_monster_tables.py` — imports
    `codex_neutral_name.{neutral_name,neutral_key,divergence_entry}`;
    `ability_pi_reason` now screens name/description/other-field hits
    separately: a hit confined to the name/key renames under `§24`
    (`name_renamed` map) instead of dropping; a hit confined to
    `DESCRIPTION` (declared `DESCISPI:YES` or an undeclared term-list hit)
    redacts-and-ships via the existing `desc_redacted` path, widened to
    also fire on the undeclared case; a hit anywhere else (owner,
    trait/variable, `SOURCEPAGE`) still drops the row, unchanged. Emission
    loop substitutes the renamed name/key and adds
    `codex_generated_name`/`rename_reason`/`rename_coordinate` fields;
    `MONSTER_ROSTER`'s own `ability_keys` cross-reference now maps through
    the rename table too (`emitted_ability_key`), so an owning monster's
    roster never emits the pre-rename key. Header-comment blocks rewritten:
    a new renamed-units block (coordinate + reason only, never the
    original string) and the desc-redacted block's wording widened.
  - `scripts/tests/test_transcribe_monster_tables.py` — new
    `NamePiAndDescPiShipInsteadOfDropping` test class (8 tests): name-PI
    renames and the original string is gone; an owning monster's
    `ability_keys` cross-reference uses the renamed key; desc-only-PI ships
    redacted with a clean name; a hit confined to `SOURCEPAGE` still drops
    (control); a clean ability is unaffected (control); the neutral name
    cannot be influenced by the original name (determinism/no-influence
    proof at this integration point). Every fixture indexes into
    `pi_blacklist_terms()[14]` — never types a real term literally.
  - `src/rules_core/rules_tables/monster_chassis.rs` — `MonsterAbilityRecord`
    gains `codex_generated_name: bool`, `rename_reason: Option<&'static
    str>`, `rename_coordinate: Option<&'static str>`. The corpus-wide
    no-reclassification pin
    (`widening_the_facet_vocabulary_does_not_reclassify_any_existing_record`)
    re-derived from a live failing run: 3706 → 3721,
    `0x38f4aedd6de1caf3` → `0x4a7c1eac4a1819f8`.
  - `src/bin/gen_book_cache.rs` — `verified_citation_line` takes a new
    `codex_generated_name: bool` param; when `true` it skips the
    row-vs-emitted-name exact-match assertion (the row's own first column
    is still the original name by design) but still bounds-checks the line
    exists. The `monster_ability` JSON blob gains `codex_generated_name`/
    `rename` fields, mirroring `ingest_ability.py`'s own shape. The three
    other call sites (monster, companion monster, companion ability) pass
    `false`, unaffected.
  - `src/rules_core/rules_tables/{bestiary,bestiary_2}/mod.rs` — **pre-existing,
    unrelated pins found stale, fixed as part of the required cross-file
    sweep**: round 6 (05b87cc276) bumped the identical 709→710/571→572/656→657
    deltas in `apps/desktop/src-tauri/src/{reach_gate,corpus_ingest_diagnostic}.rs`
    but missed these two `src/rules_core` files' own copies of the same
    counts — confirmed pre-existing by diffing `monster_data.rs` content
    (zero deletions either book, only the 3 new trailing fields per record;
    identical `MonsterAbilityRecord {` counts at `git show HEAD` and after
    this cycle's regen).
  - `src/rules_core/rules_tables/{inner_sea_bestiary,inner_sea_gods,inner_sea_world_guide}/mod.rs`
    — pinned counts re-derived (owned/owner-less/total abilities, all
    documented with the delta and reason); PI-rows tests split into a
    monster-row half (unchanged, still dropped) and a new ability-row half
    proving the rename (`codex_generated_name`, `rename_reason`, name
    format, name==key) rather than absence.
  - `src/rules_core/rules_tables/{inner_sea_bestiary,inner_sea_gods,inner_sea_world_guide}/monster_data.rs`,
    all other 18 registered books' `monster_data.rs` — regenerated via
    `transcribe_monster_tables.py <book>` for every registered book (struct
    field addition requires every emitted literal to carry it); only the 3
    affected books' *content* changed, the other 18 gained the 3 trailing
    fields per record with no other diff.
  - `src/bin/v06_work_inventory.rs`, `tests/derived_evaluator_fixture_check_monster_ability.rs`
    — the two hand-written `MonsterAbilityRecord` literals outside the
    generated files updated with the 3 new fields (`false`/`None`).
  - `apps/desktop/src-tauri/src/monster_catalog.rs` — the corpus-wide
    owner-less-records pin re-derived: 1048 → 1061 (+13, the 13 name-PI
    units, all orphans; the 2 desc-PI units are owned, no move).
  - `apps/desktop/src-tauri/src/reach_gate.rs` — `verified_citation_line`
    call sites updated for the new param; `OPEN_FINDINGS` entries for the 3
    books re-derived (owner-less/total counts); `UNREACHED_RECORD_FINDINGS`
    gained the 13 new orphan keys across the 3 books' entries;
    `inner_sea_world_guide_reaches_the_catalog_for_every_linked_record`'s
    own pins (27→30, 13→16 owner-less) re-derived.
  - `data/corpus/{inner_sea_bestiary,inner_sea_gods,inner_sea_world_guide}/monster_ability/*.json`
    (15 new files, additive only) and each book's `LICENSE.json`
    (screening-note append, same generator-owned mechanism every prior
    round used) — via `cargo run --bin gen_book_cache -- <book>` for the 3
    affected books.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped to this cycle's
  own diff, `git diff <PIN>`, not the full `BASE_BRANCH...HEAD` form which
  returns unrelated pre-existing tagged lines from the whole bundle
  history).
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scoped diff).
- **PI grep result:** the full 66-term blacklist (`pi_blacklist_terms()`)
  scanned against this cycle's own scoped diff twice — once before the
  final commit and again after a self-caught defect (see Notes) — both
  times clean except 3 harmless substring false positives (`Nex` matching
  inside `.next()`, `OGL` matching inside license-field text, `Geb` in
  unrelated pre-existing context), all individually verified by reading
  the matched line.
- **Acceptance criterion:** `decisions.md §24` — PI-name-blocked units are
  ingested under a Codex-generated neutral name; `decisions.md §19a`/`§19c`
  — extend the redact-and-ship path to an undeclared description-only
  term-list hit; this brief's own T9 round 6 receipt, "next-cycle plan"
  item 1 (highest-value target).
- **Corpus SHA:** `PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`; bootstrapped from empty this cycle via
  `scripts/fetch-pcgen-oracle.sh`, same as round 6's own worktree).
- **Status:** complete (this cycle's own scope; card 11 stays `in-progress`
  — see "What remains" below).
- **Notes:**
  - **Self-caught PI leak, fixed before commit.** My first draft of the
    code comments explaining the fix (in `reach_gate.rs` and two book
    `mod.rs` files) quoted the two creatures' PI names directly ("Spawn of
    ...", "Daughter of ..."), reasoning by example rather than by
    coordinate. A full-diff scan against the live 66-term blacklist caught
    this before commit; every instance was rewritten to cite `(book, file,
    line)` only, never the name, matching this codebase's own established
    convention (`transcribe_monster_tables.py`'s own PI-screen section
    header explicitly documents this same discipline). Logged as a
    `scripts/retro.py correction` (see below). The two ability names that
    remain in comments (`Grim White Stag ~ Bugle`, `Thyrlien ~ Starlight
    Blast`) are CLEAN names — not blacklist terms themselves, verified
    individually against the term list — so naming them is not a leak; only
    the two creature/deity names that ARE blacklist terms were redacted.
  - **The 22-unit `TYPE:`-facet-gap group is untouched.** Its
    delivery-only-default sub-group is still blocked on the pending
    operator ruling (round 6's next-cycle-plan item 2); the rest is genuine
    per-record book-specific work `decisions.md §17` forbids doing casually
    in this cycle's remaining scope.
  - **The 56-unit multi-`DESC:` parse-refusal group is untouched.** Round
    6's own assessment (a generalised sixth branch, `PREVAREQ`/`PREVARGT`
    tracing each row's own `BONUS:VAR` value) stands unverified by this
    cycle; it is the next highest-value target.
- **Discovery forwards:** none new.
- **Next-cycle plan:** see §7 below.

---

## 1. Re-derived the population before touching anything (`§17a`)

Never trusted the brief's own 98/13/2/56/22/5 figures without re-deriving.
`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json`
confirmed `monster_ability` `no_record` = **98** (bundle-wide `no_record`
1249), and the exact same 13+2 coordinate set round 6's receipt named,
cross-checked directly against `docs/work-inventory.json`'s `monster_ability`
`no_record` rows filtered to `inner_sea_bestiary`/`inner_sea_gods`/
`inner_sea_world_guide` (13 name-PI at `isb_abilities_race.lst:312-318`,
`isg_abilities_races.lst:43-45`, `iswg_abilities_race.lst:24/25/27`; 2
desc-only-PI at `isg_abilities_races.lst`'s two remaining owned rows in that
book's `DESCISPI:YES`-redacted set).

## 2. Why the 13+2 could ship under §24: the owning monster's own status is orthogonal

Checked live, before writing any code: none of the 15 units' ability rows
have a SURVIVING non-PI owning monster in this book. The 13 name-PI units
are either pure orphans (`inner_sea_bestiary`, `inner_sea_gods`'s 3 of them)
or owned by a monster row that is ITSELF `NAMEISPI:YES`-declared and already
dropped by the pre-existing `monster_pi_reason` screen (`inner_sea_world_guide`).
This meant the fix could stay narrowly scoped to `ability_pi_reason` alone —
no change to `monster_pi_reason`, no monster-kind renaming, and (confirmed by
direct check of `monster_ability_keys`/`owners` construction order) no
surviving cross-reference anywhere emits the pre-rename ability key, EXCEPT
one case worth widening for correctness even though it doesn't fire in this
population: a monster that owns a renamed ability directly via its
`ABILITY:` token. Fixed generically (`emitted_ability_key`, applied to
`MONSTER_ROSTER`'s own `ability_keys` slice) rather than leaving it
correct-by-luck for this population and wrong for the next one — proven by a
dedicated hermetic test (`test_owning_monsters_ability_keys_list_uses_the_neutral_key`)
since the real 98-unit population never exercises this path.

## 3. RED → GREEN (`AGENTS.md` non-negotiable rule 1)

`scripts/tests/test_transcribe_monster_tables.py::NamePiAndDescPiShipInsteadOfDropping`
run against the pre-fix module (`git show <PIN>:scripts/transcribe_monster_tables.py`
loaded into a scratch path, never `git stash`, module's own `tmt` reference
swapped at test-module level): 5 of 6 substantive tests failed/errored
(2 `AttributeError` for `neutral_name`/`neutral_key` not existing yet, 3
assertion failures for the rename/redact behaviour not existing), the
6th (`test_a_hit_confined_to_source_page_still_drops_the_row`, a control)
passed both before and after, as it should. Against the fix: all 8 pass.

Full module: `python3 -m unittest scripts.tests.test_transcribe_monster_tables`
— 23/24 pass. The 1 failure
(`InternalBundleAbilityHopIsResolved::test_an_ability_no_bundle_names_stays_an_orphan_and_is_not_shipped`)
is pre-existing: reproduced identically against `git show <PIN>` of the
unmodified module with the exact same synthetic fixture (round 5/6's own
prior receipts already named this test as pre-existing and out of this
lane's territory).

## 4. Corpus regeneration — additive only, verified before AND after

`git status --porcelain` before every commit: 15 new `monster_ability` JSON
files (all in the 3 affected books, filenames derived from the neutral
coordinate slug, no PI), 3 `LICENSE.json` screening-note appends, zero
deletions (`git status --porcelain | grep '^ D\|^D '` → empty throughout).
`cargo run --bin gen_book_cache -- <book>` for exactly the 3 affected books
(`inner_sea_bestiary`, `inner_sea_gods`, `inner_sea_world_guide`), each
reporting `0 new monsters ... already on disk, left untouched` and `N new
monster abilities` matching the expected 7/5/3 split. No `--allow-stamp-loss`,
no full-corpus regen attempted.

## 5. `verified_citation_line`'s design tension, resolved

`gen_book_cache.rs`'s `verified_citation_line` re-reads the cited row live
and asserts the emitted `name` matches the row's own first column — the
exact defect class `decisions.md §22`/`SD31-E6-F9-002` exists to catch (a
stale citation). A renamed record's emitted name is BY DESIGN not the row's
first column, so the first `cargo run --bin gen_book_cache` attempt panicked
on all 15 renamed/redacted-adjacent rows (panic message itself proved the
citation is real and correctly pointed — "names X, not Y" — confirming the
line exists and is the right one, just under the new naming rule). Fixed by
threading `codex_generated_name` through: the exact-match assertion is
skipped ONLY for a renamed record, and the line-exists bounds check (a
citation this generator cannot verify at all is not a citation, renamed or
not) still runs unconditionally.

## 6. What was actually closed this cycle: 15 units, by two generic mechanisms

**Closure this cycle: 15 units, real ingestion, 0 reclassified, 0
reachability gained.**

- **13 name-PI units** (`inner_sea_bestiary` 7, `inner_sea_gods` 3,
  `inner_sea_world_guide` 3) ship under a Codex-generated neutral name/key,
  `codex_generated_name: true`, `rename_reason: "name_pi_blocked"`. All 13
  are orphans (`owners: []`) -- no monster row of any book claims them,
  confirmed live, not assumed.
- **2 description-only-PI units** (`inner_sea_gods`) ship with a clean
  name/key and `description` replaced by `REDACTED_PI_MARKER`, the same
  path `DESCISPI:YES`-declared rows already use, now also firing on an
  undeclared term-list hit confined to the description field. Both are
  OWNED.

`monster_ability` `no_record`: **98 → 83** (re-derived:
`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json` →
`monster_ability 83` in the join-status-by-kind breakdown). Bundle-wide
`no_record`: **1249 → 1234**.

`§16`'s four numbers: **closure 15**, **reclassification 0**,
**reachability 0** (every one of the 15 was already unreachable — 13 newly
orphaned, 2 already owned-and-reachable before this cycle in the sense that
their OWNER already reached the catalog; the units THEMSELVES gain no new
reach, only shape-measurement), **instrument correction 0**.

## 7. Tests

```
python3 -m unittest scripts.tests.test_transcribe_monster_tables
  24 tests, 23 passed, 1 failed (pre-existing, confirmed against the
  unmodified module too, named in §3 above)
cargo build --locked --lib                                          clean, 9 warnings (pre-existing shape)
cargo test --locked --lib monster_chassis::                          8 passed, 0 failed (pin re-derived: 3721 / 0x4a7c1eac4a1819f8)
cargo test --locked --lib rules_tables::                             506 passed, 0 failed, 3 ignored
cargo build --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins   clean
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins monster_catalog::
  26 passed, 0 failed (was 25/1 before this cycle's own monster_catalog.rs pin fix -- the ONE
  pre-existing failure this cycle's diff itself moved, fixed inline before this run, per §5's own
  cross-file-pin-sweep requirement)
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins corpus_ingest_diagnostic::
  15 passed, 0 failed (was 13/2 at round 6's own baseline per that round's receipt -- both
  pre-existing failures are gone at this PIN, not caused by or fixed by this cycle's diff:
  `git diff --stat` for this cycle touches no file corpus_ingest_diagnostic.rs itself reads beyond
  what the shared crate already carried)
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins reach_gate::
  23 passed, 8 failed — IDENTICAL split to round 4/5/6's own recorded baseline (re-verified: none
  of the 8 failing tests' own printed detail names `inner_sea_bestiary`, `inner_sea_gods`,
  `inner_sea_world_guide`, or `monster_ability` -- the surfaced finding in every case is
  `advanced_race_guide/companions`, unrelated to this cycle's scope or diff). One NEW failure this
  cycle's own diff caused (`inner_sea_world_guide_reaches_the_catalog_for_every_linked_record`,
  27→30/13→16) was found, fixed inline (its own pins + the `UNREACHED_RECORD_FINDINGS` entries for
  all 3 affected books), and re-verified green before this final run.
```

## 8. What remains (three separate figures per `decisions.md §16`)

Remaining **83**, unchanged shape from round 6's own grouping (this cycle
closed the highest-value 15 and did not touch the other two groups):

1. **Multi-`DESC:` parse refusals — 56 units, unchanged.** `PRERULE`/
   `PREVAREQ`-gated variant-text rows, each needing its own `BONUS:VAR`
   value traced. `parse_desc`'s own docstring already names the fifth
   refused shape this would need to generalise.
2. **`TYPE:`-facet-vocabulary gaps — 22 units, unchanged.** ~11 book-specific
   one-off labels needing a real per-record policy call; the delivery-only
   default sub-group (2+ units) is blocked on the pending operator ruling
   round 6 escalated (default a bare `SpellLike`/`ModifyHP`-shaped
   delivery-only row to `SpecialQuality`?); one `TYPE:Internal`-only row
   (`VISIBLE:NO`) round 6 called "a genuinely novel shape" and did not rule
   on — still open, still not this cycle's to decide unilaterally.
3. **`occult_adventures` — 5 units, correctly out of scope.** Not
   re-verified this cycle (round 6 already re-verified it three times); no
   reason to expect the negated `PRECAMPAIGN` gate changed.

## 9. Next-cycle plan

1. **Multi-`DESC:` `PREVAREQ`/`PREVARGT` shape (56 units, highest-value
   remaining target).** Trace each row's own `BONUS:VAR` value; a
   generalised sixth `parse_desc` branch, per round 6's own assessment.
2. **`TYPE:`-facet delivery-only default** needs the operator ruling
   escalated in round 6 before any further work in that group.
3. **The `TYPE:Internal`-only novel-shape row** likewise needs a decision
   before it can close, one way or the other.
