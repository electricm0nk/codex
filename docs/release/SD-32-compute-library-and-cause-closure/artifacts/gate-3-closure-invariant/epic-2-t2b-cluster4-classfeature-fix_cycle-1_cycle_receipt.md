# Cycle epic-2-t2b-cluster4-classfeature-fix/1 — Gate 3 closure invariant / Epic 2 / Card 11, T2b cluster 4

- **Card ID:** `epic-2-cause-closure` (row 11; scope: T2b, `decisions.md §17` cluster 4 — the
  book-level classifier-noise cluster named by `card11-t2b-remeasure.md §5/§7`)
- **Actor:** `t2b-remeasure-remediation`
- **Commit SHA:** (filled in at push — see this file's own commit in `git log`)
- **Files touched:** `src/bin/v06_work_inventory.rs` (new `book_pc_class_names()`, `refine_kind`
  extended with a 4th parameter and a new `Kind::ClassFeature` reclassification arm, 7 new unit
  tests, 20 existing call sites updated); `scripts/t2b_pc_class_prefix_stress_test.py` (new,
  corpus-wide safety check); this receipt; `progress.md`; `kanban.md` (row 11, stays `in-progress`).
  **`docs/work-inventory.json` is deliberately NOT regenerated this cycle** — see §4.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (own diff, `git diff --unified=0 HEAD -- src/bin/v06_work_inventory.rs`)
- **Wired-integration audit result:** `OK_NO_TOKENS` (same diff)
- **Acceptance criterion:** `acceptance-and-verification.md` AT-32-E2-001 (T2b shape); `decisions.md
  §13` ("do the work"); `§16` ("a unit moved out of T2b is not a unit closed... name the kind it
  moved to and prove it"); `§17` ("generic passes, not per-object work"); `§17a` ("re-derive every
  figure you are handed").
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
  re-confirmed this cycle via `scripts/fetch-pcgen-oracle.sh --dest <repo-local slot>` after
  `scripts/verify.sh --only preflight-oracle` FAILed on this fresh worktree — matches the pin
  exactly).
- **Status:** complete for its own stated scope (a real, generic, corpus-wide classifier fix,
  verified). **Not** a closure of T2b or of card 11 — see §5 for the honest disposition.
- **Base:** `PIN=16300bde7d30429584e8d5fed00ad807c565cfc1` resolved via `git merge-base
  --is-ancestor` against the dispatched worktree's stale `HEAD` (footgun 1 fired again — the
  worktree's checkout was behind `origin/tranche/12`, not ahead of a nonexistent SHA this time);
  fast-forwarded via `git merge --ff-only origin/tranche/12` to land on `16300bde7` itself, `PIN`
  confirmed an ancestor.

## 0. §17a — re-deriving the brief's own figures before touching anything

The dispatch brief (citing `card11-t2b-remeasure.md`) stated T2b at **1,578**, unchanged since the
`decisions.md §16` classifier fix, with a 7-unit stale-ledger gap named as an open finding (finding
5) that would bring the "true" figure to 1,571 once the ledger was regenerated.

**Re-derived at this cycle's own start (`HEAD 16300bde7`, before any change):**

```
python3 -c "
import json
d = json.load(open('docs/work-inventory.json'))
u = [x for x in d['units'] if x.get('kind')=='race_trait' and x.get('evidence')=='race_trait_race_not_modelled']
print(len(u))"
```
→ **1,571**, not 1,578. `git log` shows why: commit `004bbe8c2` ("regenerate work-inventory.json
fresh post-rebase (card 15)") landed between the memo's own commit (`adac1fb22`) and this cycle's
start, and its own commit message confirms it closed exactly the stale-ledger gap the memo's
finding 5 named. **Correction logged** (`docs/retro/events/t2b-remeasure-remediation.jsonl`) —
this is `decisions.md §17a` working as intended: the handed figure was stale by one intervening
commit, caught before it propagated into this cycle's own claims.

## 1. What was investigated before writing any code (`§17`'s own standing control)

Per `decisions.md §17`'s standing control — "before any lane is scoped to `<X>` only, check whether
that restriction is the bottleneck" — and the brief's cluster-4 recommendation ("extend
`decisions.md §16`'s classifier fix with a second discriminator... does this book's
`abilities_race.lst` file have a sibling `*_races.lst` with any content at all"), the FIRST thing
checked was whether that literal recommendation is actually safe. **It is not**, and this cycle
would have shipped a real defect had it not been checked:

`core_rulebook`'s own `cr_races.lst` carries **zero** `CR:` tokens (it is a pure player-race book —
0 monsters). Under the brief's literal recommendation ("book has zero `CR:`-bearing race names →
reclassify its `_abilities_race.lst` rows"), `core_rulebook`'s **entire real race-trait population**
(the `Racial Spell-Like Ability`/`AbilityBonus` content the current classifier already gets right)
would have been reclassified as noise. Re-derived corpus-wide:

```
grep -c "CR:" <oracle>/.../core_rulebook/cr_races.lst   # -> 0
```

So the true discriminator is not "book has zero `CR:`-bearing races" — `core_rulebook` also has
zero, and it is the single largest genuinely-real T2b-adjacent book in the corpus. Investigating
the actual content of the target books' residual rows (`mythic_adventures`'s `Mythic Aboleth ~
Mucus Cloud`, `advanced_class_guide`'s `Arcanist Exploit ~ Arcane Barrier`, `advanced_players_guide`'s
`Bard Spell Level 0`) found the root cause is **heterogeneous, not one classifier gap**:

- `advanced_class_guide`'s and part of `advanced_players_guide`'s residual is genuinely **class
  bookkeeping filed in the wrong file** — `ABILITY:Arcanist Class Feature|AUTOMATIC|...` and
  `TYPE:BonusSpellKnownSkald`/`TYPE:Warpriest Class Feature....` name a real player class
  (`CLASS:Arcanist`, `TYPE:Base.PC`, confirmed present in the book's own `*_classes.lst`), not a
  race, monster, or a race-book at all.
- `mythic_adventures`'s and `pathfinder_unchained`'s residual is monster-template content
  (`Mythic Aboleth`/`Unchained Evolution`) filed the same way `decisions.md §16` item 1 already
  fixed for bestiary books, but with a **compound TYPE first segment referencing a creature name
  that is declared NOWHERE ELSE in the book's own corpus** (not in a `*_races.lst`, not in a
  `*_templates.lst`, not in a `*_classes.lst`) — the KEY-prefix book-name-cross-reference mechanism
  `decisions.md §16` already built has no name to check against for these, because the creature
  itself lives in a different book entirely (the Bestiary). This needs a genuinely different
  mechanism (cross-book monster-name resolution, or a TYPE-shape-only rule proven safe against the
  `Favored Enemy` trap some other way), not a variant of the existing KEY-prefix check.
- `occult_adventures`'s residual (`Emotional Focus / Anger`, Spiritualist's own class feature) does
  not even KEY-prefix-match its owning class name, so neither this cycle's fix nor a naive
  extension of it reaches it.

**This cycle scoped itself to the first, safely-provable sub-cause** (class bookkeeping mis-filed
by a whole-file filename guess) rather than shipping the brief's literal recommendation, which
would have been the exact `decisions.md §1a` "gate that cannot fail" failure mode — a discriminator
that looked plausible and was unsafe on inspection.

## 2. The fix — `book_pc_class_names()`, corpus-wide, provably safe

`refine_kind` gained a fourth parameter, `book_pc_class_names: &BTreeSet<String>`, computed once
per book (same shape as the existing `book_monster_race_names`) by scanning the book's own
`*classes*.lst` file(s) for `CLASS:<Name>` declarations **gated on `TYPE:` containing the exact
dot-segment `PC`** — the corpus's own player-class-vs-monster-class discriminator. A row whose KEY
prefix (falling back to the row's bare first column when no `KEY:` field exists — the same
fallback `enumerate_file`'s own `display_name`/`key` resolution already uses) exactly names, or
begins with (`"<Name> "`), one of that book's genuine PC classes reclassifies from `Kind::RaceTrait`
to `Kind::ClassFeature`. Gated by the SAME `is_player_favored_class_choice_row` guard the existing
monster-ability arm already uses, so a Favored Class Bonus row (`advanced_players_guide`'s bare
`Alchemist`, `TYPE:FavoredClass`) — a third, distinct data shape — is correctly left untouched.

**Why the `.PC` gate matters (proven, not assumed):** the first, un-gated version of this check was
built and corpus-wide-tested before the `.PC` gate was added. It wrongly matched `bestiary`'s
`CLASS:Drider` (`TYPE:Monster`, a racial-hit-dice class, not a player class), `bonus_bestiary`'s
`Faerie Dragon`/`Water Naga`, and `core_essentials`'s `Dragon Age (N)` — all real content this fix
must not touch. The `.PC` gate removes every one of those false positives while keeping every
genuine player class. This is recorded as a unit test
(`book_pc_class_names_tests::collects_only_pc_gated_classes_and_excludes_monster_classes`) and as
the corpus-wide safety script's own doc comment.

**RED → GREEN, twice:**
1. The three new `refine_kind` tests (`book_class_name_prefix_row_reclassifies_to_class_feature`,
   `bare_class_name_key_reclassifies_to_class_feature`,
   `class_name_prefix_match_requires_a_word_boundary`) were written before the implementation arm
   and failed for the intended reason (`left: RaceTrait, right: ClassFeature` — the new arm did not
   exist yet). Confirmed, then implemented, then GREEN.
2. Mid-implementation, the first version of the class-feature arm used the SAME `key_prefix`
   variable the monster arm uses (`KEY:`-field-only, no bare-first-column fallback). All three new
   tests still failed (`left: RaceTrait, right: ClassFeature`) because ACG's real rows
   (`Skald Spell Level 0`, `Warpriest`) carry **no `KEY:` field at all** — the bare first column IS
   their identity, confirmed against the real corpus row
   (`grep "Skald Spell Level 0" acg_abilities_race.lst`, no `KEY:` token present). Added the
   bare-first-column fallback (scoped to the NEW arm only, so `decisions.md §16`'s already-signed-off
   monster-ability arm is byte-for-byte unchanged), re-ran: GREEN.

## 3. Corpus-wide safety proof — `§17a`'s "failure branch must cover the whole corpus"

`scripts/t2b_pc_class_prefix_stress_test.py` (new, committed) walks **every** `*_abilities_race.lst`-shaped
file under `PCGEN_CORPUS_ROOT` (found by `glob.glob(f"{CORPUS}/**/*.lst")`, not a hardcoded book
list — the exact defect the brief's own §0 named in the predecessor stress test:
`KNOWN_RACE_BOOKS_DIRS`'s 10-dir hardcode silently missed `dreamscarred_press`), mirrors the Rust
logic exactly, and asserts zero matches against 11 known-real playable-race books
(`core_rulebook`, `bestiary` through `bestiary_6`, `advanced_race_guide`, `inner_sea_races`,
`core_essentials`, `ultimate_wilderness`):

```
PCGEN_CORPUS_ROOT=<oracle>/data python3 scripts/t2b_pc_class_prefix_stress_test.py
```
```
advanced_players_guide  16  ['Inquisitor Spell Level 0', ..., 'Oracle Curse Bonus Tracker', 'Oracle Spell Level 0']
advanced_class_guide    10  ['Skald Spell Level 0', ..., 'Skald Spell Level 8']
path_of_iron             6  ['Vanguard Spell Level 0', ..., 'Vanguard Spell Level 5']
OK: no known real-race book was matched by the PC-class-prefix discriminator
```

`path_of_iron` is not a currently-ingested book in this ledger (0 units under T2b's evidence filter
either before or after — confirmed) so it does not move the T2b figure; named here because the
script scans the whole corpus, not just registered books, per its own design.

## 4. Measured effect — verified via an isolated run, NOT applied to the checked-in ledger

An isolated `cargo run --locked --bin v06_work_inventory -- --stdout-only` (writing to a scratch
path, `docs/work-inventory.json` untouched) confirms the real movement:

```
total units: 49,540 -> 49,541 (+1; one row's `has_classifying_token` result differs between
  Kind::RaceTrait and Kind::ClassFeature for a single edge-case row -- traced, not a bug: a row
  that failed RaceTrait's classifying-token test now passes ClassFeature's, so it is newly counted
  rather than dropped -- named here, not silently absorbed)
class_feature: 18,056 -> 18,081 (+25)
race_trait:     2,640 -> 2,616  (-24)
T2b (race_trait_race_not_modelled): 1,571 -> 1,547 (-24)
provenance rows (book, source_file, source_line) that were race_trait and are now class_feature: 25
  advanced_players_guide: 15
  advanced_class_guide:   10
```

**Per `decisions.md §16`: this is 25 units moved to `class_feature`, not 25 units closed.** They
were never race content; they are now correctly filed as `class_feature` bookkeeping, where they
join that kind's own ledger (not-ingested until a class_feature ingest pass reaches them). Named
here explicitly, not folded into a claimed T2b reduction.

**`docs/work-inventory.json` is deliberately NOT regenerated this cycle.** Comparing the isolated
run's stamp population against the checked-in ledger found the exact near-miss the dispatch brief
warned about:

```
literal-verified: checked-in 6,506 -> isolated-run 2   (-6,504)
fixture-verified:  checked-in 1,741 -> isolated-run 2   (-1,739)
```

A plain regen without `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT` set silently
drops nearly every provenance stamp — exactly the incident the brief's "near-miss" section
describes, caught here by diffing the full status distribution before committing anything, per
that section's own instruction. Regenerating those two reports first
(`corpus_literal_sweep --json-out`, `derived_evaluator_fixture_check --json-out`) is real,
non-trivial work out of this cycle's remaining budget — named as explicit next-cycle work, not
silently skipped. **The code fix (this cycle's real deliverable) is committed and proven correct
independent of the ledger regen** — the ledger will reflect it whenever the next cycle regenerates
it properly.

## 5. Honest disposition — what this cycle is and is not

**This cycle is:** a real, TDD'd, corpus-wide-safety-tested classifier fix that correctly moves 25
units out of `race_trait` misclassification, per `decisions.md §17`'s "generic passes, not
per-object work" — one code change reaching two books at once, no book-specific patch.

**This cycle is NOT:** a closure of T2b (1,547 of the re-derived figure remains open — the larger
`mythic_adventures`/`pathfinder_unchained` monster-template-shaped residual and
`occult_adventures`'s non-prefix-matching residual are real, structurally different sub-causes not
attempted this cycle, see §1) and NOT a closure of card 11 (T9, T12, T2a-residual, T4-L9 all remain
open per `decisions.md §13`). Card 11 stays `in-progress`.

## Suites re-run this cycle

```
cargo test --locked --bin v06_work_inventory        -> 341 passed, 0 failed (own bin's full suite,
                                                          includes all 7 new tests)
cargo test --locked --lib                            -> 2,409 passed, 1 FAILED (pre-existing,
                                                          unrelated -- rules_core::feat_prereqs's
                                                          own pinned Fighter feat-count assertion,
                                                          in a module this cycle's diff never
                                                          touches; `src/bin/v06_work_inventory.rs`
                                                          is not part of the `--lib` target at all)
```

## Sweep for pinned counts

```
grep -rn "2472\|2,472\|1578\|1,578\|1571\|1,571" tests/ src/ scripts/ apps/ 2>/dev/null | grep -v "/target/" | grep -v "/artifacts/corpus/"
```
No hardcoded `assert`/`assert_eq` anywhere in `src/`, `tests/`, or `apps/` pins T2b's total —
confirmed before and after this cycle's diff; every hit is prose in `docs/release/` receipts.

## Next-cycle plan

1. Regenerate `CORPUS_LITERAL_SWEEP_REPORT`/`DERIVED_FIXTURE_CHECK_REPORT`, then
   `docs/work-inventory.json` with both set, diff the full status distribution before/after, commit
   only if the diff shows the expected -24/+25 (`race_trait`/`class_feature`) movement plus no
   unexplained stamp loss.
2. Cluster 4's remaining sub-causes (§1): a cross-book monster-name resolution mechanism for
   `mythic_adventures`/`pathfinder_unchained`'s compound-TYPE rows; a non-prefix-matching
   discriminator for `occult_adventures`'s class-feature-shaped-but-not-class-name-prefixed rows.
3. Cluster 3 (`Adopted Race` selector, 35 real units across 9 books) — not attempted this cycle,
   time budget spent on cluster 4's investigation and fix; report mechanism-sized, per `decisions.md
   §17` item 3.
4. Clusters 1 and 2 (`bestiary_5`'s 8 chassis + Skinwalker heritage-selector, 133 units combined) —
   not attempted, real content builds, report mechanism-sized per the same instruction.

```
$ df -h /
Filesystem      Size  Used Avail Use% Mounted on
/dev/sda1       968G  317G  651G  33% /
```
No pressure, no cleanup needed.
