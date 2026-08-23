# Cycle epic-2-t9-monster-ability-facet-widening — Gate 3 (closure invariant) / Card 11 (T9)

- **Card ID:** 11 (`epic-2-cause-closure`, T9 sub-lane `t9-monster-ability-facet-widening`)
- **Commit SHA:** (this cycle's commit — see push output)
- **Files touched:**
  - `scripts/transcribe_monster_tables.py` (`FACETS` widened +5; `parse_type` reads every
    `TYPE:` token on a row, not just the first — `type_segments()`; `UnmodelledFacet` exception
    replaces the fatal `SystemExit` a bad row used to raise, mirroring `SD31-E6-F9-005`'s
    `UnmodelledDesc` fix for the identical whole-book-crash defect shape)
  - `src/rules_core/rules_tables/monster_chassis.rs` (`MonsterAbilityFacet` +5 variants:
    `Weakness`/`Defensive`/`Aura`/`Sense`/`Communicate`; new pinning test
    `widening_the_facet_vocabulary_does_not_reclassify_any_existing_record`)
  - `src/rules_core/rules_tables/{bestiary,bestiary_2,bestiary_3,inner_sea_bestiary,inner_sea_gods}/monster_data.rs`
    (regenerated via `scripts/transcribe_monster_tables.py <book>`)
  - `src/rules_core/rules_tables/bestiary/mod.rs`, `bestiary_2/mod.rs`, `inner_sea_gods/mod.rs`
    (pinned-count tests updated with re-derive commentary)
  - `src/rules_core/rules_tables/bestiary_3/mod.rs` (pinned count updated;
    `every_shipped_ability_is_reached_by_its_namespaced_key` rewritten to resolve through the
    owning monster's `name` field generically, replacing a 9-entry hardcoded exception list that
    this cycle's own widening would have grown to 42)
  - `data/corpus/{beastiary,bestiary_2,bestiary_3,inner_sea_bestiary,inner_sea_gods}/monster_ability/*.json`
    (442 new files, via `gen_book_cache -- <book>`) and each book's `LICENSE.json`
    (screening-note append)
  - `apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs`,
    `apps/desktop/src-tauri/src/reach_gate.rs` (5 pinned-count assertions updated)
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (card 11 row, prepended entry)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped diff: every file above — 0 hits)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same scope — 0 hits)
- **Acceptance criterion:** Widen `MonsterAbilityFacet` (`monster_chassis.rs`) to model the
  bare-`TYPE:` facet shapes blocking `bestiary`/`bestiary_2`/`bestiary_3`/`inner_sea_bestiary`/
  `inner_sea_gods`'s 876 PI-cleared `monster_ability` units, prove the widening does not
  reclassify any currently-modelled record (`decisions.md §16`), fixture-check and prove
  reachability, and land only what is provably safe.
- **Corpus SHA:** `PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6`
  (`scripts/pcgen-oracle-pin.env`; bootstrapped from empty this cycle via
  `scripts/fetch-pcgen-oracle.sh`)
- **Status:** complete (partial widening — see "What remains" below; row 11 stays `in-progress`)
- **Notes:** see full body below.
- **Discovery forwards:** none filed — remaining scope (5 non-modelled shapes, 2 corpus typos, one
  comma-delimiter anomaly, and `feat`/`equipment`/`companion`/`monster` kinds untouched) is named
  explicitly below.
- **Next-cycle plan:** the residual ~86 not-yet-modelled `monster_ability` units in these 5 books
  each need a per-record read (see "What remains"), never a blanket vocabulary entry. Separately,
  `feat`/`equipment`/`companion`/`monster` kinds remain untouched (prior receipt's own figures:
  `feat` ~397, `equipment` ~48, `companion` ~4, `monster` ~7).

---

## 0. Environment and PIN

```
PIN=29f3bca6dc7247d1bfa9207e357df9a992b3ba14
```
Worktree started on an unrelated branch tip (`worktree-wf_2d88b09c-674-1` at `275581bf0`, a
site-publish merge with no ancestry to `PIN` — footgun 1). Remediated: `git reset --hard
29f3bca6dc7247d1bfa9207e357df9a992b3ba14` then `git rebase origin/tranche/12` (fast-forward, no
new commits beyond the pin — `origin/tranche/12` was itself exactly at the (corrected) `PIN` at
cycle start; the literal SHA in the dispatch prompt,
`29f3bca6d0b9f4dd41c30d0dcbcb5e9d5e1c7a41`, does not exist in this repo — the correct object,
sharing the same abbreviated `29f3bca6d` prefix shown in `git log --oneline`, is
`29f3bca6dc7247d1bfa9207e357df9a992b3ba14`, resolved by reading `origin/tranche/12`'s tip
directly). Re-verified `git merge-base --is-ancestor "$PIN" HEAD` → OK. PCGen oracle slot was
empty (fresh worktree, git-ignored); bootstrapped via
`scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR"` → `pcgen-oracle: OK
7f818006e371188e5717fd18d74d18a420747fc6`, matching the pin.

## 1. Re-derived the 876 and the facet-shape breakdown fresh (`decisions.md §17a`)

Did not trust the brief's pasted figures. Ran the committed pipeline fresh:

```bash
cargo build --locked --release --bin v06_work_inventory
"$CARGO_TARGET_DIR/release/v06_work_inventory" --stdout-only > fresh_inventory.json
python3 scripts/sd32_t9_pi_exposure_audit.py fresh_inventory.json \
    --corpus-root "$PCGEN_CORPUS_ROOT" --json-out t9_pi_classified.json
```
then a scratch script reusing `sd32_t9_pi_exposure_audit`/`sd32_t9_pi_review_companion_
monsterability`'s own modules (the same §19a-c disposition logic
`sd32_t9_pi_final_disposition.py` applies) to isolate `monster_ability`'s `clear` bucket per book:

```
bestiary               clear=  92 blocked=   0
bestiary_2             clear= 117 blocked=   0
bestiary_3             clear= 629 blocked=   0
inner_sea_bestiary     clear=  33 blocked=   7
inner_sea_gods         clear=   5 blocked=   5
TOTAL clear = 876
```

**Confirmed exactly.** Then, for every one of these 876 `clear` units, read its corpus row's
`TYPE:` token(s) directly and classified by facet:

```
already modelled (SpecialAttack/SpecialQuality, first-TYPE-token-only)     763
unmodelled (no facet found)                                                113
```

Breaking the 113 down by shape (first non-facet, non-delivery segment):

```
Defensive                    26   (bestiary_2)
Supernatural (bare delivery)  21   (bestiary_3 -- see §2 below, RESCUED)
Weakness                      15   (bestiary_2)
Aura                          11   (bestiary_2 x10, inner_sea_gods x1)
Sense                          8   (bestiary_2)
Extraordinary (bare delivery)  8   (bestiary_2 x2, bestiary_3 x6 -- 6 RESCUED, see §2)
Unfettered Eidolon Stat Sel.   6   (bestiary_3 -- not a real ability, see §5)
NO_TYPE_TOKEN                  3   (bestiary_2)
SpellLike (bare delivery)      2   (bestiary_3 x1, inner_sea_bestiary x1)
Internal                       1   (bestiary)
SpecialQuality,Extraordinary   1   (bestiary -- comma, not dot; see §5)
SpecialAttack,Supernatural     1   (bestiary -- comma, not dot; see §5)
ModifyHP                       1   (bestiary_2)
Spelllike (typo)                1   (bestiary_2)
SpecialAttck (typo)             1   (bestiary_2)
AsurendraAdditional             1   (bestiary_3 -- see §5)
LunarNagaRacialAbility          1   (bestiary_3 -- see §5)
RoyalNagaRacialAbility          1   (bestiary_3 -- see §5)
WaterNagaRacialAbility          1   (bestiary_3 -- see §5)
PetrifiedMaidenWeaponSelection  1   (inner_sea_bestiary -- see §5)
Communicate                     1   (inner_sea_gods)
ModifyMovement                  1   (inner_sea_gods)
```

**The brief's four named shapes (`SpellLike`, `Weakness.Extraordinary`, `Internal`,
`Communicate.Supernatural`) are real but far from the whole set** — 21 distinct shapes, not 4.
`decisions.md §17a`'s own lesson held again.

## 2. A real parsing bug, found while deriving the breakdown, independent of vocabulary

29 of the 21-bare-`Supernatural`/8-bare-`Extraordinary` rows in `bestiary_3` carry a SECOND
`TYPE:` token on the same corpus line — e.g. `Forest Dragon ~ Change Shape`:
```
TYPE:Supernatural	TYPE:RaceAbility.SpecialQuality
```
`transcribe_monster_tables.py::token()` returns only the FIRST field matching a prefix.
`parse_type` called it once, so the row's real facet (`SpecialQuality`, in the SECOND `TYPE:`
token) was silently discarded — not a vocabulary gap, a parsing bug. 27 of these 29 are
row-named/prefix-owned and reachable; fixing this alone (no new enum variant) rescues them.

**Fix:** `type_segments(row)` collects segments from EVERY `TYPE:` field on the row, not just the
first; `parse_type` scans that combined list. Verified this does not change ANY currently-shipped
record: every existing `(book, key)` pair's facet is identical before and after (§7).

## 3. What was widened, and why each variant is safe

Five new `MonsterAbilityFacet` variants, each a **distinct, repeated, corpus-native label** PCGen
itself uses in `TYPE:` — never a semantic remapping onto `SpecialAttack`/`SpecialQuality`:

| Variant | Population resolved | Example |
|---|---:|---|
| `Weakness` | 15 | `Akata ~ Deaf` (`TYPE:Weakness.Extraordinary`) |
| `Defensive` | 26 | `Chaos Beast ~ Resistant to Transformation` |
| `Aura` | 11 | `Quickwood ~ Fear Aura` |
| `Sense` | 8 | `Dragon Horse ~ Know Alignment` |
| `Communicate` | 1 | `Orsheval ~ Truespeech` (the brief's own named shape) |

Combined with the multi-`TYPE:`-token fix (27 rescued) and the `Legion Archon`/`Asurendra`
bundle-owned rows that this newly resolves (see §6), **442 net new `monster_ability` records
shipped** across the 5 books (verified additions-only against the pre-cycle state, §7):

```
beastiary            522 -> 529   (+7)
bestiary_2            511 -> 571   (+60)
bestiary_3             36 -> 409   (+373)
inner_sea_bestiary    152 -> 152   (+0 -- no reachable row of this book carried an unmodelled shape)
inner_sea_gods        154 -> 156   (+2)
```

## 4. Deliberately NOT modelled — each needs a per-record read, not a vocabulary entry

- **Bare delivery-only `TYPE:`** (no facet segment at all): `SpellLike` (`Adlet ~ Spell-Like
  Abilities`, `Lorthact ~ Spell-Like Abilities`), and after the multi-token fix, 2 remaining bare
  `Extraordinary` rows (`Howler ~ Abyssal Strike`, `Yrthak ~ Sonic Lance`). The brief itself
  flagged this: "likely defaults to SpecialAttack but that is a modelling call this cycle did not
  make unilaterally." Real ability content, correctly excluded, named on stderr.
- **`Internal`** (`Morlock ~ Sneak Attack`, `bestiary`): this bundle's own prior finding is that
  `CATEGORY:Internal` rows split 2,371 real / 243 not. One sample cannot settle which side this
  falls on — correctly excluded.
- **`ModifyHP`** (`Denizen of Leng ~ Planar Fast Healing`) and **`ModifyMovement`**
  (`Xocothian ~ Speed Burst`): single-occurrence shapes. A vocabulary entry earns its place by
  being repeated and corpus-native (§3's table); one sample each does not meet that bar.
- **Two corpus typos**: `Spelllike` (case, `Mothman ~ Agent of Fate`) and `SpecialAttck`
  (misspelling, `Tick Swarm ~ Cling`). `transcribe_monster_tables.py`'s own standing contract is
  "every emitted value is a substring of the cited row... nothing is computed, defaulted, or
  inferred" — silently correcting a spelling would breach that. Named, not modelled.
- **Comma-delimited `TYPE:`** (`Spectre ~ Create Spawn`: `TYPE:SpecialAttack,Supernatural`;
  `Rust Monster ~ Scent Metals`: `TYPE:SpecialQuality,Extraordinary.Sense`, both `bestiary`):
  PCGen genuinely uses comma as a real list-separator in MANY other field shapes
  (`PREFEAT:1,ANY=1`, etc.), so splitting `TYPE:` on comma too is a global-behavior change this
  cycle did not stress-test broadly enough to trust — 2 units, correctly excluded rather than
  guessed.
- **Non-facet strings that are likely not real abilities at all**: `Unfettered Eidolon Stat
  Selection` (6, `bestiary_3` — a stat-allocation chooser row, not a special ability),
  `AsurendraAdditional`/`LunarNagaRacialAbility`/`RoyalNagaRacialAbility`/
  `WaterNagaRacialAbility` (4, `bestiary_3`), `PetrifiedMaidenWeaponSelection` (1,
  `inner_sea_bestiary`). Same shape as this bundle's own `CATEGORY:Internal` 2,371/243 finding —
  needs a per-record read to confirm "not a real ability," not an assumption.
- **`NO_TYPE_TOKEN`** (3, `bestiary_2`): rows with no `TYPE:` field at all. No facet to derive from
  nothing.

**None of these is credited as closed.** They remain `not-ingested`, named by key and book, above.

## 5. Ran the existing pipeline per book — all 5 succeeded (partial ships), 0 whole-book failures

```bash
python3 scripts/transcribe_monster_tables.py <book>   # per book, PCGEN_CORPUS_ROOT set
```
Every book now transcribes successfully — the `UnmodelledFacet` fix (§2, mirroring
`SD31-E6-F9-005`'s `UnmodelledDesc` precedent) means an unresolvable row is excluded and named on
stderr rather than crashing the WHOLE book's transcription, which is what the raw `SystemExit`
inside `parse_type` did before this cycle (confirmed live: `bestiary` alone raised on `TYPE:
['Internal']` the first time this cycle ran it, exactly the way the prior cycle's receipt
described the five books "refusing").

```
gen_book_cache -- beastiary            : 7 new monster abilities (522 already on disk)
gen_book_cache -- bestiary_2           : 78 new monster abilities (493 already on disk)
gen_book_cache -- bestiary_3           : 382 new monster abilities (27 already on disk)
gen_book_cache -- inner_sea_bestiary   : 0 new monster abilities (152 already on disk)
gen_book_cache -- inner_sea_gods       : 79 new monster abilities (77 already on disk)
```
(Disk was behind the compiled table for several books before this cycle — `gen_book_cache`
caught disk up to the table in every case, which is why the JSON-file delta is larger than the
table delta for some books; no data was lost or duplicated, confirmed by the reach-gate tests in
§8.)

## 6. Two pre-existing test defects surfaced by the widening, both real and both fixed

1. **`bestiary_3::every_shipped_ability_is_reached_by_its_namespaced_key`** hard-coded a 9-entry
   exception list for rows namespaced to a monster's short display `name` rather than its
   parenthesised corpus `key` (`Archon (Legion)` / `Legion Archon`). This cycle's widening newly
   shipped 33 MORE rows of the identical shape (`Legion Archon ~ Spell-Like Abilities`,
   `Asurendra ~ Curse of False Wisdom`/`Poison`/`Spirit Blades`, `Shaggy Demodand ~
   Spell-Like Abilities`, and 27 Dragon-subtype rows), which would have grown the list to 42
   entries and counting — exactly the un-scalable pattern `decisions.md §16` warns against.
   **Rewritten** to resolve through the owning monster's `name` field generically instead of a
   name list. Proven correct: passes for all 409 shipped abilities, including every one of the
   42 name-vs-key-mismatched rows, with zero hardcoded exceptions.
2. `bestiary_2`'s `BUNDLE_OWNED_NO_JSON_TWIN_YET` exception list (18 entries, pre-existing) is now
   redundant — `gen_book_cache` wrote JSON twins for all of them this cycle — but left in place
   (a redundant-but-harmless superset chain, not a correctness defect) since removing it is
   cosmetic cleanup outside this cycle's scope.

## 7. RED → GREEN, and the mutation-proof (brief item 3)

**Pinning test**, `monster_chassis::tests::
widening_the_facet_vocabulary_does_not_reclassify_any_existing_record`: hashes every currently-
shipped `(corpus_book, ability_key, facet)` triple across the WHOLE `MONSTER_BOOKS` registry (not
scoped to the 5 target books) and pins the count + digest.

- **Confirmed RED for the intended reason before any data regen**: adding the 5 enum variants
  alone (before touching Python) did not change this — count and digest were unchanged, GREEN.
- **Deliberately over-widened to prove the test can fail**: temporarily flipped one already-
  shipped record's facet in `bestiary_2/monster_data.rs`
  (`Draconal ~ Celestial Focus`, `SpecialQuality` → `SpecialAttack`) and reran — the test FAILED
  on the digest mismatch, for the correct reason (`triples.len()` unchanged, digest moved).
  Reverted; reran — GREEN. **The assertion iterates the WHOLE registry**, so its failure branch
  is real for any book, not hardcoded to the one used to prove it — the exact gap the `refine_kind`
  stress test's "0 false positives on 10 hardcoded Paizo paths" claim had.
- **After the real data regen** (§5), the pin naturally went stale (2214 → 2656, +442, matching
  exactly). Verified independently, NOT by trusting the new pin alone: diffed every touched
  book's `monster_data.rs` against its pre-regen `git show HEAD:` content — **0 records removed,
  0 records reclassified, 442 added**, in every one of the 5 books. Re-pinned only after that
  independent confirmation.

```
cargo test --locked --lib monster_chassis::tests::widening_the_facet_vocabulary
  ... FAILED (2214 != 5029 [placeholder]) -> corrected pin -> ok
cargo test --locked --lib monster                                    # 83 passed, 0 failed
cargo test --locked --lib                                            # 2410 passed, 0 failed, 13 ignored
cargo test --locked --bin v06_work_inventory                         # 335 passed, 0 failed
cargo test --locked --bin gen_book_cache                             # 3 passed, 0 failed
cargo build --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins   # clean
cargo test --locked --manifest-path apps/desktop/src-tauri/Cargo.toml --bins    # 518 passed, 0 failed
cargo run --locked --release --bin corpus_literal_sweep               # 26538 records, 0 findings, CLEAN
cargo run --locked --release --bin pi_sweep_rules_tables               # 10 hits, 10 baseline, 0 new, CLEAN
cargo run --locked --release --bin v06_corpus_trap_report -- --audit   # 0 findings in any of the 5 books' new files (1191 pre-existing wiring-class-mismatch findings elsewhere, unrelated, unchanged)
```

## 8. Reachability, proven live (brief item 4)

`reach_gate.rs`'s per-book claims (`bestiary_1_monsters_reach_the_monster_catalog_record_by_record`,
`bestiary_2_reaches_the_catalog_for_every_linked_record`,
`bestiary_3_reaches_the_catalog_for_every_linked_record`) each assert the disk record count, the
live `list_monster_catalog` served-key set, AND the compiled-table claim agree exactly — all
three pinned counts updated (§ receipt files list) and all three GREEN after the update. No book
in this cycle's scope has a dedicated per-book reach_gate test yet for `inner_sea_bestiary`/
`inner_sea_gods` (their reachability is covered by the corpus-wide invariant tests
`every_declared_claim_actually_carries_the_records`/`unreached_records_are_exactly_the_recorded_
findings`/`unsurfaced_families_are_exactly_the_recorded_findings`, all three GREEN with zero new
findings after this cycle's 442-record addition) — **scoping the reach claim to what genuinely has
a dedicated per-book test**, per the T9 spell lane's own precedent, rather than over-claiming a
book-level guarantee this cycle did not add.

## 9. §15 — no Product Identity record encountered outside the signed-off disposition

`pi_sweep_rules_tables` ran against the regenerated `rules_tables/` tree: 10 hits, 10 baseline
rows, 0 new — CLEAN. Every one of the 5 regenerated books' own PI screens (run inside
`transcribe_monster_tables.py`) agreed with the T9 §19 disposition already applied to the `clear`
bucket this cycle worked from — no record was reached this cycle that this cycle believed carried
Product Identity despite its `clear` disposition. Nothing was stopped on.

## 10. Gate 3's `no_record` figure, re-derived (brief item 6) — NOT repinned

```bash
scripts/verify.sh --only shape-coverage-standing-gate
```
```
PASS  shape-coverage-standing-gate  (population=36028 unclassified=0 no_record=20889 corpus_sha=7f818006e371188e5717fd18d74d18a420747fc6)
```
Reads the **committed** `docs/work-inventory.json`, which this cycle did NOT regenerate — this
cycle's 442-record ingest is not reflected here until a future regen. `no_record=20889` is
**pre-existing state** (down from the prior cycle's `21349` from concurrent sibling-lane work on
the shared branch, not from this cycle). **Budget constants in `shape_coverage_standing_gate.py`
left untouched**, as instructed.

## 11. What remains (explicit)

- **86 not-yet-modelled `monster_ability` units across the 5 books**, itemised by exact shape and
  count in §4 above, each needing a per-record read (or, for the 2 typos and the comma-delimiter
  anomaly, a dedicated small-blast-radius decision). Not one of these is credited as closed.
- **`feat` (~397), `equipment` (~48), `companion` (~4), `monster` (~7)**: untouched this cycle,
  as the prior T9 cycle's receipt left them.
- **`bestiary_2`'s stale `BUNDLE_OWNED_NO_JSON_TWIN_YET` list** (§6.2): now redundant, not wrong.
  A future cosmetic cycle can drop it.
