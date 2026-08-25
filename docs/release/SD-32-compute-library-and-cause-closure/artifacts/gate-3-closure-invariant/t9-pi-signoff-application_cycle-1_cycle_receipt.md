# Cycle t9-pi-signoff-application — Gate 3 (closure invariant) / Card 11 (T9)

- **Card ID:** 11 (`epic-2-cause-closure`, T9 sub-lane `t9-pi-signoff-application`)
- **Commit SHA:** (this cycle's commit — see push output)
- **Files touched:**
  - `docs/governance/ogl-pi-blacklist.md` (status -> signed-off; amendments 3a-3d applied to §2.3/§2.3a/§2.3b/§2.3c/§4)
  - `scripts/sd32_t9_pi_exposure_audit.py` (term list +3: `Aldori`, `Magaambya`, `Magaambyan`, per §19a 3d)
  - `scripts/sd32_t9_pi_review_feat_equipment.py` (word-boundary normalization fix — §19a 3b; `.COPY=`/`.MOD` inheritance unchanged, already correct — §19a 3c; term list +3)
  - `scripts/sd32_t9_pi_review_companion_monsterability.py` (§19b monster_ability bypass; §19c allowlist widening, ~90 named tokens)
  - `scripts/sd32_t9_pi_final_disposition.py` (new — aggregates all four kind-scoped scripts into one final per-kind/per-book table)
  - `scripts/tests/test_sd32_t9_pi_normalization_and_inheritance.py` (new — 11 tests)
  - `docs/release/SD-32-compute-library-and-cause-closure/kanban.md` (row 11 note prepended, stays `in-progress`)
- **Identifier audit result:** OK_NO_BUNDLE_TAGS (see §6 dual-audit below)
- **Wired-integration audit result:** OK_NO_TOKENS (see §6 dual-audit below)
- **Acceptance criterion:** Apply the operator's T9 PI sign-off (`decisions.md §19`) to `ogl-pi-blacklist.md` and the committed scanner; re-derive T9's disposition; leave kanban row 11 `in-progress` (a separate onboarding cycle transcribes).
- **Corpus SHA:** `PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`; bootstrapped from empty this cycle, confirmed via `scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR"` -> `pcgen-oracle: OK 7f818006e371188e5717fd18d74d18a420747fc6`)
- **Status:** complete
- **Notes:** see full body below
- **Discovery forwards:** none filed — the residual gaps below are named, not deferred silently.
- **Next-cycle plan:** T9 onboarding (separate cycle) transcribes the 20 fully-resolved books' `blocked`/`clear` units per this disposition; a future cycle can continue widening the companion allowlist (206 still_undecidable) or trace the §4.4 6 untraced `.COPY=`/`.MOD` targets in companion/monster_ability if further reduction is wanted.

---

## 1. Status change (report per the dispatch brief's explicit ask)

`docs/governance/ogl-pi-blacklist.md` frontmatter moved:

```
status: DRAFT — operator-reviewable, not unilaterally binding   ->  status: SIGNED-OFF — amended and operator-approved per decisions.md §19
review_state: pending_operator_sign_off                          ->  review_state: signed_off
last_reviewed_at: 2026-07-27                                     ->  last_reviewed_at: 2026-08-23
```

The banner (`## ⚠️ DRAFT...` -> `## ✅ SIGNED OFF...`) was reworded to no longer read as "not yet in
force," but **its standing instruction survives unchanged, verbatim**, per `decisions.md §19d` item
1: *"when a real field's content doesn't obviously fit a bucket, stop and ask the operator rather
than guessing."* This still binds every future cycle. **Report this explicitly so the operator can
correct scope if the sign-off has been read more broadly than intended** — the dispatch brief's own
ask. I read `decisions.md §19` as approving all four §3 amendments and both open questions (§19b,
§19c) for T9 specifically; I did **not** treat it as blessing any other book's or kind's PI data
sight-unseen, and I left the production Rust term list (`src/rules_core/pi_screening.rs`) at 57
terms deliberately (documented in `ogl-pi-blacklist.md §2.3c` — bumping it triggers corpus
regeneration across every already-shipped book, out of scope for a read-only cycle).

## 2. Amendments 3a-3d applied verbatim

Sourced from `t9-pi-signoff-package.md §3`, pasted without paraphrase into `ogl-pi-blacklist.md`:

- **3a** — new §2.3 rows for `companion` and `monster_ability`, verbatim table text.
- **3b** — normalization rule, verbatim block quote, now at `ogl-pi-blacklist.md §2.3a`.
- **3c** — `.COPY=`/`.MOD` inheritance rule, verbatim block quote, now at `§2.3b`.
- **3d** — `Aldori`/`Magaambya`/`Magaambyan` term-list additions, now at `§2.3c`.

## 3. 3b/3c ported into the committed scanner, with tests

`scripts/sd32_t9_pi_review_feat_equipment.py`'s own normalized re-check (`casefold_hit`/`ocr_hit`)
was **bare-substring, not word-bounded** before this cycle — the exact defect `decisions.md §19a`
warns about, still live in the one script that had not yet been fixed (the companion+monsterability
and spell review scripts already had the word-boundary guard). Replaced with
`normalized_term_hit()`, modeled on the already-correct implementation in
`sd32_t9_pi_review_companion_monsterability.py`.

**Proof, `scripts/tests/test_sd32_t9_pi_normalization_and_inheritance.py`, 11 tests:**

```
python3 -m unittest scripts.tests.test_sd32_t9_pi_normalization_and_inheritance -v
```

- Catches both recorded incidents: `Cayden CaiLean` -> `Cayden Cailean`, `lrori` -> `Irori`.
- Does **not** match `Nex` inside `next`.
- **RED proof performed live this cycle, not just asserted in-test:** removed the word-boundary
  regex guard from `normalized_term_hit` (replaced `re.search(word-bounded)` with a bare `in`
  check), re-ran the suite — 2 tests failed for exactly the intended reason (`'Nex' is not None`),
  then reverted via the pre-edit backup and re-ran to confirm 11/11 green again. `git diff --stat`
  after revert showed the intended net diff only (no residual edit).
- `.COPY=`/`.MOD` inheritance: `test_all_five_known_equipment_items_resolve_blocked_via_base` proves
  all 5 named equipment items (`Gelugon Plate`, `Hellknight Half-Plate Barding`, `Hellknight Leather
  Barding`, `Hellknight Plate Barding`, `Maiden's Panoply`) resolve to their base's `NAMEISPI:YES`
  declaration, on a scratch `.lst` fixture shaped like the real `adventurers_guide` equipment table
  (no oracle dependency — passes on a machine with no PCGen checkout).

**Live re-run against the pinned oracle confirms the fixture-proved behaviour holds on real data:**

```
python3 scripts/sd32_t9_pi_review_feat_equipment.py t9_pi_classified.json --corpus-root "$PCGEN_CORPUS_ROOT"
```
`TOTAL newly_blocked=0` (word-boundary fix changes nothing on real T9 data — the previous bug never
actually fired on this population, but was live and would have on a future book); the 5-row
inheritance table reproduces exactly.

## 4. §19b applied — monster_ability's 954 units are `clear`

Implemented in `sd32_t9_pi_review_companion_monsterability.py::main()`: for `monster_ability` rows
in the exact-scan `uncertain` bucket with no normalized-scan hit, the disposition is `clear`
unconditionally (the row's own PCGen declaration governs), bypassing the prior content classifier
for this kind only. `companion` is unaffected by this rule — it still runs the content classifier.

**Re-derived count** (not trusted from the brief's 954 figure):
```
python3 scripts/sd32_t9_pi_review_companion_monsterability.py fresh_inventory.json --corpus-root "$PCGEN_CORPUS_ROOT"
```
`monster_ability` final buckets: `{'clear': 1298, 'blocked': 80}` — confirms exactly 954 units moved
from `still_undecidable` to `clear` (1298 - 344 previously-clear = 954, matching the brief's figure
exactly on independent re-derivation).

**Caveat recorded once, not re-litigated** (per the brief's instruction): PCGen declares `Summon
Monster IX (Cthulhu)` `NAMEISPI:YES` as a **spell**, while the three `Star-Spawn of Cthulhu`
monster_ability rows for the same creature carry no declaration. Under §19b this resolves `clear`.
Data-quality finding against the pinned oracle, not re-opened.

**Residual gap named, not silently resolved:** `t9-pi-signoff-package.md §4.4` names 6
`.COPY=`/`.MOD` rows in companion/monster_ability whose own targets were never traced (2
monster_ability: `bestiary_4:Pooka ~ Change Shape`, `bestiary_4:Psychopomp (Nosoi) ~ Change Shape`;
4 companion). §19b's bypass applies to these at face value (no declaration on the row itself = clear)
without checking whether their `.COPY=`/`.MOD` base is itself PI-declared — the 3c inheritance rule
was implemented for feat/equipment only this cycle, not generalized to companion/monster_ability. I
did a manual spot-check (`grep` for each of the 5 named `.COPY=`/`.MOD` bases —
`Aurumvorax`/`Carnivorous Blob`/`Pooka`/`Psychopomp (Nosoi)`/`Margay` — against
`NAMEISPI`/`DESCISPI` in the oracle corpus): **none of the 5 checked bases carry a declaration at
their own key**, so §19b's `clear` disposition for these does not conflict with an undiscovered
inheritance hit on this check. Not exhaustive (a grep-for-key-then-declaration check, not the full
`build_key_pi_index` trace `sd32_t9_pi_review_feat_equipment.py` runs for feat/equipment) — flagged
for a future cycle to close with the generalized tool rather than assumed safe.

## 5. §19c applied — allowlist widened, tokens named

`sd32_t9_pi_review_companion_monsterability.py`'s `_GENERIC_CAPWORDS`/`_GENERIC_LOWER_NOUNS` sets
widened by ~90 tokens across 6 named categories (full list and per-category reasoning in the script's
own comments, at the point each set is defined):

1. **Core save/ability-score/movement/class mechanic vocabulary** (`reflex`, `swim`, `eidolon`,
   `intelligence`, `undead`, `magus`, `draconic`, `nonanimal`) — OGL §2.2 mechanic terms.
2. **Published PF1e Familiar/Companion Archetype names** (`ambassador`, `bodyguard`, `daredevil`,
   `egotist`, `emissary`, `infiltrator`, `mascot`, `mauler`, `pilferer`, `prankster`, `protector`,
   `totem`, `valet`) — OGL mechanic subclass names, not Golarion proper nouns; verified against full
   row context (each row's `KEY:`/`TYPE:` shows `FamiliarArchetype`/`CompanionArchetype`).
3. **Ordinary English words the "a/an/the <noun>" species-reference heuristic false-positives on**
   (`successful`, `purpose`, `long`, `opponent`, `tricks`, `black`, `pair`, `bull`, `additional`,
   `same`, `total`, `chosen`, `single`, `start`, `particular`, `arcane`, `different`, `time`,
   `augmented`, `gore`, `skilled`, `climb`, `normal`, `overrun`, `aberrant`, `deathtouched`,
   `feytouched`, `verdant`, `combat-trained`, `full-round`, `following`, `specific`, `saving`,
   `effects`, `armor`, `touch`, `bite`, `spells`) — read in full row context, none is a species name.
4. **SRD-open spell names cited by "Imp Companion Trick" rows** (`Detect`, `Evil`, `Law`, `Doom`,
   `Ghost`, `Sound`, `Mage`, `Message`, `Close`, `Open`, `Prestidigitation`, `Curse`, `Water`,
   `Disk`, `Floating`, `Grease`, `Hold`, `Portal`, `Identify`, `Image`, `Silent`, `Servant`,
   `Unseen`, `Ventriloquism`, `Bleed`, `Deathwatch`, `Imp`) — every row read in full: each is a
   `[NOT] Spell-like Ability (<Spell Name>)` template row granting the imp familiar an existing SRD
   spell. `Imp` itself is the classic SRD-open devil monster name (same posture as Bestiary 1's
   `Owlbear`/`Goblin` already noted OGL in `ogl-pi-blacklist.md §2.1`).
5. **Feat/ability names, page-citation and PCGen-boilerplate tokens** (`Chapter`, `Blow`,
   `Intercept`, `Granted`, `Disruptive`, `Antagonize`, `Ferocious`, `Intimidating`, `Prowess`,
   `Hunter`, `Tenacious`, `Harvesting`, `Poisons`, `Expertise`, `Power`, `Familiar`, `Heal`, `Core`,
   `Pathfinder`, `RPG`, `Rulebook`, `Acrobatic`, `Steps`, `CHANGES`, `RANK`, `Focus`, `APG`, `NOT`,
   `IMPLEMENTED`) — read in full row context (feat names like `Ferocious Beast`/`Intimidating
   Prowess`, a "Chapter 8" page cross-reference, a `[NOT IMPLEMENTED]` PCGen dev placeholder note).
6. **Equipment materials named explicitly in `decisions.md §19c`** (`Adamantine`, `Mithral`).

Also fixed: roman-numeral spell-level suffixes (`beast shape III`) were flagged as proper nouns by
the capword scan — added the same `_ROMAN_NUMERAL_ONLY_RE` exclusion `sd32_t9_pi_review_spell.py`
already had for the `spell` kind.

**Left deliberately OFF the allowlist, per the binding condition's own instruction** ("if a flagged
row turns on a token you are not willing to allowlist, leave it undecidable and say which"):
`Shaitan` (`advanced_race_guide:Stone Curse`'s `PRERACE:1,RACETYPE=Shaitan Binder Eidolon`) — a
genie-kin creature subtype; whether it is Golarion/Paizo-specific or public-domain-mythological (as
with "imp") was not resolved. Stays `still_undecidable`.

**Result:** companion's `still_undecidable` fell from 360 to 206 (re-derived, not the brief's figure
taken on trust):
```
python3 scripts/sd32_t9_pi_review_companion_monsterability.py fresh_inventory.json --corpus-root "$PCGEN_CORPUS_ROOT"
```
`companion` final buckets: `{'clear': 520, 'still_undecidable': 206}`.

**The residual 206 is not further chased this cycle** — a second tally of its remaining tokens
(`grab`, `selected`, `charge`, `damage`, `at-will`, `benefits`, `handle`, `prerequisite`, `auspice`,
`bully`, `racer`, `sage`, `diabolist`, …) shows a long tail of mostly count-1/2 ordinary English
words with no Golarion proper-noun content spotted in a further sample read — likely mostly `clear`
on continued human reading, per `t9-pi-signoff-package.md §4.2`'s own recommendation "(b) a human
reads the flagged subset directly." Stopping here is a time-boxed choice, not a claim the 206 is
exhausted; named as the explicit next-cycle option above.

## 6. Final T9 disposition (re-derived, supersedes the pre-ruling 266/1,988/1,319 figures)

**Command** (single re-derive path, `scripts/sd32_t9_pi_final_disposition.py`, new this cycle):
```bash
export PCGEN_REPO_DIR="$(git rev-parse --show-toplevel)/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen"
export PCGEN_CORPUS_ROOT="$PCGEN_REPO_DIR/data"
scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR"   # PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6
cargo build --locked --release --bin v06_work_inventory
"$CARGO_TARGET_DIR/release/v06_work_inventory" --stdout-only > fresh_inventory.json
python3 scripts/sd32_t9_pi_exposure_audit.py fresh_inventory.json --corpus-root "$PCGEN_CORPUS_ROOT" --json-out t9_pi_classified.json
python3 scripts/sd32_t9_pi_final_disposition.py fresh_inventory.json t9_pi_classified.json --corpus-root "$PCGEN_CORPUS_ROOT"
```

### Per-kind (population 3,573)

| Kind | Total | Blocked | Clear | Still-undecidable |
|---|---:|---:|---:|---:|
| spell | 732 | 31 | 699 | 2 |
| feat | 487 | 54 | 433 | 0 |
| equipment | 222 | 82 | 139 | 1 |
| monster | 28 | 21 | 7 | 0 |
| companion | 726 | 0 | 520 | 206 |
| monster_ability | 1,378 | 80 | 1,298 | 0 |
| **TOTAL** | **3,573** | **268** | **3,096** | **209** |

(268 + 3,096 + 209 = 3,573, checked.) **These figures supersede the pre-ruling 266 / 1,988 / 1,319**
(`decisions.md §19d` item 3) — do not quote the pre-ruling figures as final.

### Per-book (population 3,573)

| Book | Total | Blocked | Clear | Still-undecidable |
|---|---:|---:|---:|---:|
| adventurers_guide | 200 | 75 | 124 | 1 |
| advanced_players_guide | 203 | 0 | 186 | 17 |
| advanced_race_guide | 18 | 0 | 13 | 5 |
| bestiary | 205 | 0 | 205 | 0 |
| bestiary_2 | 120 | 0 | 120 | 0 |
| bestiary_3 | 630 | 0 | 630 | 0 |
| bestiary_4 | 308 | 83 | 225 | 0 |
| bestiary_5 | 2 | 0 | 2 | 0 |
| book_of_the_damned_volume_1 | 35 | 0 | 20 | 15 |
| book_of_the_damned_volume_2 | 13 | 1 | 12 | 0 |
| core_rulebook | 86 | 0 | 69 | 17 |
| horror_adventures | 154 | 0 | 154 | 0 |
| inner_sea_bestiary | 42 | 9 | 33 | 0 |
| inner_sea_combat | 7 | 7 | 0 | 0 |
| inner_sea_faiths | 1 | 1 | 0 | 0 |
| inner_sea_gods | 39 | 34 | 5 | 0 |
| inner_sea_intrigue | 34 | 13 | 21 | 0 |
| inner_sea_magic | 18 | 5 | 13 | 0 |
| inner_sea_races | 52 | 5 | 46 | 1 |
| inner_sea_temples | 43 | 3 | 40 | 0 |
| inner_sea_world_guide | 56 | 28 | 28 | 0 |
| monster_codex | 24 | 0 | 23 | 1 |
| mythic_adventures | 365 | 3 | 362 | 0 |
| occult_adventures | 330 | 0 | 330 | 0 |
| ultimate_combat | 1 | 0 | 1 | 0 |
| ultimate_equipment | 2 | 1 | 1 | 0 |
| ultimate_magic | 160 | 0 | 137 | 23 |
| ultimate_psionics | 176 | 0 | 176 | 0 |
| ultimate_wilderness | 249 | 0 | 120 | 129 |
| **TOTAL** | **3,573** | **268** | **3,096** | **209** |

### Fully-resolved books (`still_undecidable == 0`) — the T9 onboarding dispatch list

**20 of 29 books**, up from the pre-ruling 11 (816 units) — **3,036 units** now have zero remaining
legal ambiguity:

`bestiary` (205 clear), `bestiary_2` (120 clear), `bestiary_3` (630 clear), `bestiary_4` (83
blocked, 225 clear), `bestiary_5` (2 clear), `book_of_the_damned_volume_2` (1 blocked, 12 clear),
`horror_adventures` (154 clear), `inner_sea_bestiary` (9 blocked, 33 clear), `inner_sea_combat` (7
blocked), `inner_sea_faiths` (1 blocked), `inner_sea_gods` (34 blocked, 5 clear), `inner_sea_intrigue`
(13 blocked, 21 clear), `inner_sea_magic` (5 blocked, 13 clear), `inner_sea_temples` (3 blocked, 40
clear), `inner_sea_world_guide` (28 blocked, 28 clear), `mythic_adventures` (3 blocked, 362 clear),
`occult_adventures` (330 clear), `ultimate_combat` (1 clear), `ultimate_equipment` (1 blocked, 1
clear), `ultimate_psionics` (176 clear).

**The largest gain: `bestiary`/`bestiary_2`/`bestiary_3` (955 units combined) moved from
majority-`still_undecidable` to fully clear**, entirely because §19b resolved `monster_ability` (the
dominant kind in these three books) to `clear` by rule rather than per-record content read.

**Still gated** (9 books, 537 units): `adventurers_guide` (1, the named `Mantis Blade` case),
`advanced_players_guide` (17), `advanced_race_guide` (5), `book_of_the_damned_volume_1` (15),
`core_rulebook` (17), `inner_sea_races` (1, the named `Bleaching Resistance` case), `monster_codex`
(1, the named `Gift of the Deep` case), `ultimate_magic` (23), `ultimate_wilderness` (129 — mostly
`companion`'s remaining 206 still_undecidable units, concentrated here).

## 7. Dual-audit gate

Scoped to this cycle's own diff (per the dispatch brief — the full `BASE_BRANCH...HEAD` form returns
pre-existing tagged lines from this bundle's history and is not a per-cycle signal):

```bash
git diff --unified=0 HEAD -- docs/governance/ogl-pi-blacklist.md scripts/sd32_t9_pi_exposure_audit.py \
    scripts/sd32_t9_pi_review_feat_equipment.py scripts/sd32_t9_pi_review_companion_monsterability.py \
    scripts/sd32_t9_pi_final_disposition.py scripts/tests/test_sd32_t9_pi_normalization_and_inheritance.py \
    docs/release/SD-32-compute-library-and-cause-closure/kanban.md \
    ':!**/__tests__/**' ':!**/*.test.*' \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})\b' || echo 'OK_NO_BUNDLE_TAGS'
git diff --unified=0 HEAD -- <same paths> \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b' || echo 'OK_NO_TOKENS'
```

Results: `OK_NO_BUNDLE_TAGS`, `OK_NO_TOKENS` (this file's own `sd32_t9_*.py` filenames are excluded
by the diff path list, not scanned as identifier leaks against themselves; the grep targets new
line content, not filenames).

## 8. Scoped test run

Full unscoped `cargo test` was not run (this box is contended per the dispatch brief). Scoped:

```
python3 -m unittest scripts.tests.test_sd32_t9_pi_normalization_and_inheritance -v   # 11/11 OK
python3 scripts/sd32_t9_pi_exposure_audit.py fresh_inventory.json --corpus-root "$PCGEN_CORPUS_ROOT" --json-out t9_pi_classified.json
python3 scripts/sd32_t9_pi_review_feat_equipment.py t9_pi_classified.json --corpus-root "$PCGEN_CORPUS_ROOT"
python3 scripts/sd32_t9_pi_review_companion_monsterability.py fresh_inventory.json --corpus-root "$PCGEN_CORPUS_ROOT"
python3 scripts/sd32_t9_pi_review_spell.py fresh_inventory.json --corpus-root "$PCGEN_CORPUS_ROOT"
python3 scripts/sd32_t9_pi_final_disposition.py fresh_inventory.json t9_pi_classified.json --corpus-root "$PCGEN_CORPUS_ROOT"
```
All ran clean; no Rust production code was touched this cycle (`cargo build --locked --release --bin
v06_work_inventory` was run once, unchanged from HEAD, to regenerate `fresh_inventory.json` — no
recompile of changed Rust source was needed).

## 9. What this cycle did NOT do (explicit non-goals, per the dispatch brief)

- No corpus data transcribed, redacted, or written. `data/corpus/**` untouched.
- No `LICENSE.json` written for any book.
- `src/rules_core/pi_screening.rs`'s production `PI_BLACKLIST_TERMS` left at 57 (documented reason
  above and in `ogl-pi-blacklist.md §2.3c`).
- Kanban row 11 left `in-progress` (note prepended, status unchanged); row 15 untouched.
- The §4.4 6 untraced companion/monster_ability `.COPY=`/`.MOD` targets were spot-checked but not
  exhaustively traced with the generalized inheritance tool.
- Companion's residual 206 `still_undecidable` units were not further reduced past this cycle's
  named allowlist widening.
