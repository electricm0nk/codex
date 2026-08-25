# Cycle row19-cycle1 — Epic 9 (`epic-9-desktop-reach-and-catalog-reds`) / Row 19

- **Card ID:** `epic-9-desktop-reach-and-catalog-reds`
- **Commit SHA:** (see push log in the terminal receipt this file accompanies)
- **Files touched:**
  - `apps/desktop/src-tauri/src/reach_gate.rs`
  - `apps/desktop/src-tauri/src/spell_catalog.rs`
  - `apps/desktop/src-tauri/src/character_hub.rs`
  - `apps/desktop/src-tauri/src/class_feature_feat_bridge.rs`
  - `apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs`
  - `apps/desktop/src-tauri/src/intelligent_item_catalog.rs`
  - `scripts/classify_companion_rows.py`
  - `src/rules_core/rules_tables/bestiary_5/companion_data.rs` (regenerated)
  - `src/rules_core/rules_tables/bestiary_5/mod.rs`
  - `src/rules_core/rules_tables/mod.rs`
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **PI scrub:** `pi_scrub.normalized_term_hits()` on the full diff — zero hits.
- **Acceptance criterion:** kanban row 19 — close as many of the 15 named
  `apps/desktop/src-tauri` reds as genuine evidence supports; set `complete`
  only when the whole desktop workspace is green.
- **Corpus SHA:** `scripts/pcgen-oracle-pin.env` (oracle bootstrapped fresh
  this cycle via `scripts/fetch-pcgen-oracle.sh --dest`, confirmed populated
  — a fresh worktree's slot is git-ignored).
- **Status:** `in-progress` (NOT `complete` — 7 of 15 named tests remain red,
  with evidence; desktop workspace is not green).
- **Notes:**

## Starting state (verified, not assumed)

Worktree started on a **stale lineage** (tranche/11 tip, footgun 3): `git
merge-base --is-ancestor $PIN HEAD` failed. Recovered via `git reset --hard
$PIN` + `git rebase origin/tranche/12`; re-verified before any edit.

Reproduced all 15 named reds first:
`cd apps/desktop/src-tauri && cargo test --locked --bin codex-desktop` (own
target dir, `CARGO_INCREMENTAL=0`) → **504 passed, 15 failed**, exact test
names matching the brief's list.

## Closed this cycle (8 of 15) — evidence per test

1. **`reach_gate::tests::the_inventory_is_populated_from_all_three_live_sources`**
   — **instrument correction**, not a stale pin. `corpus_inventory()` panicked
   on 163 (book, kind) directory pairs the classifier could not name (13
   distinct new kind directories: `ability`, `class_generic`, `deity`,
   `domain`, `feat_generic`, `language`, `monster_generic`, `power`,
   `race_generic`, `race_trait_generic`, `skill`, `template`, `trait_generic`
   — across ~30 books — plus 4 unnamed books: `core_essentials`,
   `inner_sea_faiths`, `inner_sea_temples`, `ultimate_campaign`). Classified
   each by direct inspection of sampled corpus JSON, not assumed:
   - `race_trait_generic` → `NON_CONTENT_CORPUS_DIRS`. Every sampled record
     (core_rulebook, beastiary, bestiary_2/3/4/5/6) carries `VISIBLE:NO` and
     composes a creature TYPE's universal traits into the monster chassis
     compute — never rendered standalone.
   - The other 12 kinds → `CORPUS_KIND_NAMES` (real, named, described
     content distinct from the primary kind sharing the book — e.g.
     `beastiary/race_generic/hydra_pyrohydra.json` is a genuine Pyrohydra
     variant; `ultimate_psionics/ability/deafening_static.json` carries the
     real key `Dissonance ~ Deafening Static` with its own mechanical
     description). **None has a `reach_of` arm yet** — naming a kind is not
     closing its reach (`decisions.md §16`); this is real, newly-discovered
     forward scope, reported below, not silently absorbed into "accounted
     for."
   - 4 books → `CORPUS_BOOK_IDS`, same-name pairing matching
     `corpus_ingest_diagnostic.rs`'s existing book ids.
2. **`reach_gate::tests::dispatch_gap_race_and_monster_families_all_have_book_level_reach_arms`**
   — closed by the same classifier fix (the test only required
   `corpus_inventory()` to name every directory and the pre-existing
   race/monster book-count thresholds, both already true).
3. **`spell_catalog::tests::the_catalog_serves_every_ingested_book_not_only_crb`**
   — **instrument correction** (stale pin). Actual 2197 vs pinned 2183.
   Re-derived via a one-shot debug print of every book's `book_entries(..)
   .len()`: ISG 92→96, AG 45→49, ISF 2→3, ISM 34→39 (all corpus growth from
   the T12 census/class-feature lanes); the other 11 books unchanged. Debug
   print removed before commit.
4. **`intelligent_item_catalog::tests::every_served_record_carries_no_declared_pi_marker`**
   — **instrument correction** (stale pin). 171 vs 169. The per-file loop
   already asserts no `pi_field`/`pi_marker` before incrementing `checked`,
   so reaching 171 without a panic is itself the proof all 171 are clean —
   not a loosened check.
5. **`corpus_ingest_diagnostic::tests::the_two_ingested_books_totals_reconcile_with_their_license_artifacts`**
   — **instrument correction** (stale pin), TWO branches, re-derived per
   `decisions.md §17a` (`reported + corpus_only_records == live_on_disk`):
   `advanced_race_guide` 1699→1701 (+2, `reported` unchanged at 506, live
   2207) and, once ARG stopped panicking, the loop reached
   `pathfinder_unchained` for the first time this session and it was ALSO
   stale: 1137→1144 (+7, `reported` unchanged at 127, live 1271).
6. **`character_hub::tests::every_equipmods_row_the_picker_offers_is_recognized_by_the_attach_gate`**
   — **instrument correction** (stale pin). 1894 vs 1831, re-derived fresh
   from the built picker (not adjusted by delta, matching this test's own
   established discipline). The load-bearing assertion (`refused.is_empty()`)
   still runs against the fresh 1894 and passes.
7. **`class_feature_feat_bridge::tests::class_feature_feat_bridge_serves_the_full_corpus_wide_population`**
   and **8. `class_feature_feat_bridge::tests::every_bridged_record_corpus_wide_carries_its_granted_feat`**
   — **instrument correction** (stale pin), same root cause, both closed
   together. 613 vs 471. `granted_feat: Some(..)` is set by construction for
   every record `load_class_feature_feat_bridge_descriptions` returns (its
   filter already matched the exact `feat_target` it stores), so the two
   assertions are one structural guarantee, not two pins that could drift
   apart independently.

## Real (non-stale-pin) reachability fix: bestiary_5 companions

`every_ingested_companion_book_reaches_the_catalog_record_by_record` and
`companion_catalog::every_served_key_matches_a_corpus_record_file` both
named `bestiary_5` failing 55 vs 57. Traced to a **genuine, evidenced
defect**, not a stale pin (matching the brief's warning that at least one of
the 15 would be this shape): `scripts/classify_companion_rows.py`'s
`UNINGESTED_CAMPAIGN_GATES = ("Occult Adventures",)` excluded `Familiar
(Brain Mole)` / `Familiar (Chuspiki)` (gated `PRECAMPAIGN:1,Occult
Adventures`) on the premise that Occult Adventures was not an ingested book
(`decisions.md §47.2`). **That premise is false today** —
`reach_gate.rs::CORPUS_BOOK_IDS` already carries `("occult_adventures",
"occult_adventures")` (an SD-31 wave-4 lane) — and `decisions.md §27b`
("EVERYTHING", 2026-08-23) separately overturned this exact exclusion shape.
Fixed at the root: emptied `UNINGESTED_CAMPAIGN_GATES`, regenerated
`bestiary_5/companion_data.rs` (`scripts/transcribe_companion_tables.py
bestiary_5` — both rows transcribed with real corpus fields, verified via
`git diff`), updated `bestiary_5/mod.rs`'s two unit tests (35 companions not
33; the "gated familiars are NOT in this rule set" test inverted to "ARE in
this rule set", by name) and `rules_tables::mod.rs`'s `RuleSetId::B5` doc
comment. Root-workspace `cargo test --lib bestiary_5` reconfirms.

**Not fully closed**: `every_ingested_companion_book_reaches_the_catalog_record_by_record`
still shows `bestiary_5` red for a SECOND, narrower reason found only after
the above landed — the two `data/corpus/bestiary_5/companion/familiar_*.json`
files' own `data.key` field is the literal display name (`"Familiar (Brain
Mole)"`), not the `bestiary_5:companion:familiar_brain_mole` slug format
every other companion record in this book carries. Traced to
`docs/work-inventory.json`'s own unit for this record: `"corpus_key":
"Familiar (Brain Mole)"` (not slug-formatted) and `"status": "not-ingested"`
(itself now stale — the corpus JSON exists). `scripts/ingest_companion.py`
line 301 reads `corpus_key` first, so it wrote the un-slugged key verbatim.
**Left red rather than hand-edited**: `data/corpus/**` is a guarded path
(never hand-edit) and `docs/work-inventory.json` is itself a generated
artifact this bundle has been burned by mutating post-hoc; the safe fix is
re-deriving `corpus_key` in the generator that produced it and re-running
`ingest_companion.py`, not a one-line patch under time pressure. Named here
by exact file and field so the next cycle does not have to re-diagnose it.

## Genuine remainders — reachability defects, not stale pins (7 of 15 still red)

- `reach_gate::tests::pathfinder_unchaineds_class_features_are_claimed_per_corpus_record`
  — PU's `class_feature` corpus population is 604, not the pinned 64.
  `corpus_ingest_diagnostic.rs`'s own comment already states why: the extra
  540 are "class_features belonging to classes the engine does not model"
  (Automatic Bonus Progression toggles, Unchained skill-system variant
  rules, Background Skills, Combat Trick/Skill Unlock pool entries — bulk
  `class_feature`-kind content that is not part of any of the 4 Unchained
  classes' real progression). `pu_class_features_reach()` runs the real IPC
  path against a level-20 fixture and returns a binary
  `Surfaced`/`else-panic` verdict with no partial-credit branch — unlike
  every sibling reach test, it has no `missing`/recorded-findings path. This
  is very likely a genuine, large non-reach (540 records), not an
  instrument bug, but confirming that for certain and writing the volume of
  evidence this bundle's standard requires (or building the partial-credit
  branch this test needs) is real new work, sized beyond this cycle.
- `reach_gate::tests::companion_catalog::tests::every_served_key_matches_a_corpus_record_file`
  and **`reach_gate::tests::every_declared_claim_actually_carries_the_records`** /
  **`unreached_records_are_exactly_the_recorded_findings`** / **`unsurfaced_families_are_exactly_the_recorded_findings`**
  / **`every_ingested_family_is_accounted_for`** — cover, corpus-wide:
  - ~38 remaining (book, kind) families (of the ~40 originally named) with
    no `reach_of` arm and no `OPEN_FINDINGS` entry — classes/feats/spells/
    equipment/class_features across ~25 books.
  - Hundreds of individual companion "Evolution ~ …" eidolon-evolution
    records across `apg`/`crb`/`ultimate_magic`/`ultimate_wilderness`/
    `advanced_race_guide`/`book_of_the_damned_volume_1`/`beastiary1`/
    `bestiary_4`/`bestiary_5`/`pathfinder_unchained` — no eidolon-evolution
    picker UI exists in this desktop app at all, so none of these reach a
    player; this is one root cause, many families.
  - `beastiary1`/companions: 28 records (`.COPY=`/`.MOD` delta rows —
    Celestial/Fiendish creature-template variants, `Universal Monster Rule`
    rows) that `scripts/transcribe_companion_tables.py` correctly refuses
    to transcribe (a `.COPY=`/`.MOD` row states a delta on another record,
    not a standalone record) — confirmed by re-running the regenerator,
    which reported the same 27 refusals and produced a byte-identical file.
    This needs a delta-application mechanism, not a transcription fix.
  - The 12 newly-classified corpus-kind directories from item 1 above
    (`ability`/`class_generic`/`deity`/`domain`/`feat_generic`/`language`/
    `monster_generic`/`power`/`race_generic`/`skill`/`template`/
    `trait_generic`) surface as new unaccounted families the moment they
    are named — real, newly-discovered forward scope this cycle's own
    classification fix exposed, not previously visible because
    `corpus_inventory()` could not even enumerate them.

  Each of these is a genuine "no UI surface exists" or "no engine mechanism
  exists" gap — real product-surfacing work (new catalog screens for
  domains/deities/skills/languages/templates/psionic powers; an eidolon
  evolution picker; delta/COPY-row engine support), sized at a full epic
  exactly as `docs/retro/events/t9-onboarding-unowned-reds.jsonl` originally
  scoped it, now larger than that estimate once the newly-classified kinds
  are counted. **Naming every family precisely (via `OPEN_FINDINGS` and
  `UNREACHED_RECORD_FINDINGS`, both keyed per exact record) so the ~38+
  families and hundreds of companion records get written up honestly rather
  than rushed is itself a multi-cycle undertaking** — attempting to fabricate
  that volume of per-record evidence under this cycle's remaining budget
  would violate `decisions.md §17a`/`§1a`'s own standard, so it is left red
  and named here rather than gamed.

## Full-sweep re-run

- `apps/desktop/src-tauri`: `cargo test --locked --bin codex-desktop` →
  **512 passed, 7 failed** (list above), down from 15 failed at the start of
  this cycle.
- Root workspace: `cargo test --locked --lib bestiary_5` (scoped per §13's
  "scope your test runs" — the full unscoped root suite was not run, per
  the brief's own warning that it may never finish on this box) →
  confirms the `bestiary_5` module's own tests pass after this cycle's fix.

## Territory

Confirmed clean before every commit: `git status --porcelain` touched only
the 10 files listed above, none overlapping row 15 (`equipment_catalog` —
confirmed already fixed 17/17 green, not touched here), row 17, or row 18's
files.

## Next-cycle plan

Pick up the 7 remaining reds in this priority order: (1) the
`bestiary_5` familiar key-format defect (narrow, single root cause,
`corpus_key` regen); (2) `beastiary1`'s 28 delta-row companions (needs a
scoped decision: build delta/COPY application, or write the `OPEN_FINDINGS`
entry and adjust `every_served_key_matches_a_corpus_record_file` to accept a
documented exception the way other reach tests already do); (3) work
through the ~38+ remaining families methodically, batching by shared root
cause (eidolon evolution picker covers the largest single cluster) rather
than per-book; (4) PU class_features — first give
`pathfinder_unchaineds_class_features_are_claimed_per_corpus_record` a
partial-credit branch matching its sibling tests' shape, then determine the
true reach of the 540 non-core records.
