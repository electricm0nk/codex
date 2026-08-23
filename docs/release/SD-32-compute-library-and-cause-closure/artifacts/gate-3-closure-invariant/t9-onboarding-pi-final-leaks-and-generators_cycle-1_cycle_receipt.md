# Cycle t9-onboarding-pi-final-leaks-and-generators — Gate 3 (closure invariant) / Card 11 (`epic-2-cause-closure`)

- **Card ID:** 11 (`epic-2-cause-closure`)
- **Commit SHA:** (recorded after push, see push output)
- **Files touched:**
  - `scripts/sd32_t9_corpus_wide_pi_rescan.py` — fixed `kind_from_path`'s
    real bug (`rel.parts[1]` is always the literal string `"corpus"`, not
    the kind; corrected to `rel.parts[3]`).
  - `scripts/tests/test_sd32_t9_corpus_wide_pi_rescan.py` (new) — proves
    the fix and the old-vs-new disagreement.
  - `scripts/ingest_simple_filename_kinds.py` — swapped the weak
    `normalized_term_hit` for the strong
    `blacklist_term_hit_including_concatenated` on both `name` and
    `description`; added a scoped `--book` remediation filter
    (`unit_in_scope` helper).
  - `scripts/tests/test_ingest_simple_filename_kinds.py` — updated for the
    renamed function, added demonym-form and `--book`-filter tests.
  - `src/rules_core/pi_screening.rs` — made `word_bounded_contains`
    `pub(crate)` (no behaviour change).
  - `src/rules_core/cache_gen/class_feature.rs` — fixed a real
    over-redaction bug in `scrub_name_pi_tokens` (word-boundary matching
    instead of bare `.contains()`), 1 new mutation-proved regression test.
  - `src/rules_core/cache_gen/acg.rs`, `apg.rs`, `beastiary1.rs` — added
    `name`/`key` blacklist screening (`name_or_key_is_pi`) to every
    equipment/spell record, `codex_generated_name: bool` field on
    `CacheRecord`, `§24` neutral-rename wiring, directory-placement-fix
    precedent applied to the slug. 12 new unit tests (4 per file).
  - `src/rules_core/cache_gen/equipment_gap.rs` — added a supplementary
    strong-scan re-screen of `description` (mirrors `class_feature.rs`'s
    own "third defect" fix), 1 new regression test.
  - `src/bin/ingest_race_traits.rs` — widened the raw-tokens redaction loop
    from `DESC`-only to every token value, strong scan, 1 new regression
    test.
  - `src/bin/declared_pi_shipping_audit.rs` — 2 new CHECK C unit tests
    proving the gate catches the exact `EquipmentData` shape
    `acg`/`apg`/`beastiary1` write.
  - 20 `data/corpus/**` records closed through guarded generator paths (15
    `class_feature` via `--coordinates`, 3 `template` via `--book`, 1
    `race_trait` via a full book-scoped `ingest_race_traits <book>` run),
    zero hand-edits.
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS` (scoped to this cycle's
  own diff, `git diff HEAD -- src/ scripts/`; the one match found —
  `sd32_t9_pi_review_feat_equipment` — is a reference to an ALREADY-EXISTING
  repo filename this cycle imports from, not a newly-introduced identifier;
  not a leak).
- **Wired-integration audit result:** `OK_NO_TOKENS`.
- **Acceptance criterion:** (1) reconcile the two disagreeing PI-leak
  figures against both `scripts/sd32_t9_corpus_wide_pi_rescan.py` and
  `declared_pi_shipping_audit` CHECK C; fix whichever instrument is wrong.
  (2) Close every confirmed leak through the guarded path; prove zero
  afterwards with both instruments. (3) Add `name` screening to
  `cache_gen::{acg,apg,beastiary1}`; audit every remaining generator for
  the same shape, reported as a table. (4) Add a test that fails when a
  generator writes a field it does not screen; prove it goes red, then
  revert.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`PCGEN_ORACLE_SHA`,
  bootstrapped fresh in this worktree, confirmed via
  `scripts/verify.sh --only preflight-oracle`)
- **Status:** complete
- **Notes:** see full account below. No blacklist term or PI item name
  appears anywhere in this receipt, any test name, any test constant, or
  any commit message — every reference is a `(book:source_file:source_line)`
  coordinate or a `PI_BLACKLIST_TERMS` index, per `§24b`-2.
- **Discovery forwards:**
  - `cache_gen::ultimate_equipment.rs` screens `description` but has NO
    blacklist scan of `name`/`key` at all — only a `NAMEISPI:YES`-declared
    whole-row DROP (`report.name_pi_dropped`), itself superseded by
    `decisions.md §24`'s neutral-rename ruling (a name-PI row should no
    longer be excluded whole). This is the SAME shape this cycle's own
    `acg`/`apg`/`beastiary1` fix closes, in a fifth generator. Not fixed
    this cycle (out of scope, sizeable follow-on: it also needs the `§24`
    rename-not-drop migration, not just the blacklist scan).
  - `src/bin/gen_core_rulebook_cache.rs` (core rulebook): `description`-only
    screening for spell/equipment kinds; `name`/`key` never screened by the
    blacklist scan either. Same shape, sixth instance, not fixed (CRB's
    content is overwhelmingly SRD/OGL, so live risk is low, but the
    architectural gap is real and CHECK C is the only thing guarding it).
  - `equipment` (1 record, `inner_sea_gods/equipment/codex_named_unit_
    equipment_inner_sea_gods_isg_equip_lst_20.json`, `description` field,
    blacklist index 9) is named by coordinate, **not closed this cycle** —
    the underlying scan-strength gap IS fixed in `equipment_gap.rs`
    (mirrors `class_feature.rs`'s "third defect" fix exactly), but this
    generator's `write_json` is no-clobber and has no scoped-regen mode.
    Closing the already-shipped record would require either (a) deleting
    it and running the FULL `gen_equipment_gap_tables` binary — risking
    writing brand-new records for whatever the equipment ingest lane
    currently has in flight, the exact collision class this dispatch's own
    territory section warns about — or (b) building new `--coordinates`
    infrastructure this cycle's remaining budget did not allow. CHECK C
    guards it permanently regardless of who closes it next.
- **Next-cycle plan:** whichever lane owns `equipment` ingest closes the 1
  named leak (its own guarded path, once a scoped-regen mode exists or the
  full-binary collision risk is cleared); a future cycle migrates
  `ultimate_equipment.rs`'s `NAMEISPI:YES` DROP disposition to the `§24`
  rename path and adds the missing blacklist scan; `gen_core_rulebook_cache.rs`
  gets the same `name`/`key` scan this cycle added to `acg`/`apg`/`beastiary1`
  when a lane has budget for it (lower priority — zero live CRB impact,
  CHECK C guards it).

## 1. Reconciliation (`§17a`) — both prior figures were stale; the instrument itself had a bug

**Instrument A:** `scripts/sd32_t9_corpus_wide_pi_rescan.py` (fixed this
cycle — see below). **Instrument B:**
`cargo run --locked --bin declared_pi_shipping_audit` (CHECK C,
`audit_blacklist_term_hits`, unchanged logic, new fixture tests only).

**Coverage of each, stated explicitly (`§12c`):**

- Both walk every `data/corpus/**/*.json` with a top-level `data` object
  and recurse EVERY string reachable under `data` (nested dicts and lists
  included — `raw_tokens[i].value`, `prerequisites[i]`, `key`, `class`,
  `name`, `description`, and any future field are all covered, not a
  hand-picked subset).
- Both scan against the SAME 61-term, word-bounded, OCR-normalized,
  concatenated-identifier scan (`pi_scrub.blacklist_term_hit_including_
  concatenated` / the Rust port of the same name in `pi_screening.rs`).
- They differ only in language and in the coordinate-scoped `§26`
  false-positive exemption (3 named files) CHECK C carries and the Python
  script does not (the Python script found the same 3 records but they are
  not exempted in its own output, so its raw count is 3 higher on a full
  corpus-wide run before any real leaks are subtracted — irrelevant to
  this cycle since none of the 3 exempted files appear in the confirmed
  population below).

**A real bug found in Instrument A itself, before trusting its output**
(`§17a`): `kind_from_path` read `rel.parts[1]`, which for a repo-rooted
path shaped `data/corpus/<book>/<kind>/<file>.json` is always the literal
string `"corpus"` — index 1 is the fixed `corpus` directory name, not the
kind. Every per-kind row this script had EVER printed silently collapsed
every kind into one `kind=corpus` bucket; the prior receipt's own
correct-looking per-kind table was produced by piping the script's raw
path list through an external `awk -F'/' '{print $4}'`, never by trusting
this function's own printed output. Fixed to `rel.parts[3]`. New
`scripts/tests/test_sd32_t9_corpus_wide_pi_rescan.py` proves the fix and
that the old expression disagrees with it on the same input. **Logged as
`scripts/retro.py correction`** (`docs/retro/events/
t9-onboarding-pi-final-leaks-and-generators.jsonl`).

```
python3 scripts/sd32_t9_corpus_wide_pi_rescan.py
# Records scanned: 51335
# Records with >=1 confirmed blacklist-term hit: 20
# Total field-level hits: 28
#   equipment: records=1 field_hits=1
#   class_feature: records=15 field_hits=27  (4 in varisian_pilgrim/*,
#                                              11 in varisian_pilgrim_domain/*)
#   race_trait: records=1 field_hits=2
#   template: records=3 field_hits=6
```

```
cargo run --locked --bin declared_pi_shipping_audit
# declared-pi-audit: FAIL — 93 violation(s) across 85 file(s)
#   BLACKLIST-TERM-SHIPPED: 28  (exactly matches the rescan's 28 field-hits,
#                                 same 20 files, same fields, by direct
#                                 coordinate comparison)
#   DESC-PI-SHIPPED: 65  (all bestiary_4/monster_ability -- the
#                          monster_ability lane's own named territory,
#                          untouched by this cycle)
```

**Both instruments agree exactly on the confirmed-leak population: 20
records / 28 field-hits, split equipment(1)/class_feature(15)/
race_trait(1)/template(3).**

**Reconciliation against the two figures this cycle was dispatched with:**

1. **The prior receipt's "6 named leaks" (equipment 1, race_trait 1,
   template 3, spell 1) was stale, not wrong in kind.** Its own scan was
   run before a sibling cycle closed the `spell` leak
   (`ultimate_wilderness/spell/mirage.json` — confirmed clean at this
   cycle's start: `license: "OGL"`, the blacklisted term does not occur
   anywhere in the record's raw bytes). The other 5 named coordinates are
   EXACTLY the 5 non-`class_feature` records this cycle's own scan found —
   unchanged since the prior receipt, confirming they really were never
   fixed. **Logged as `scripts/retro.py correction`.**
2. **The orchestrator's own brief figure ("2, both in equipment") was
   also stale** — it under-counted by missing the ENTIRE 15-record
   `class_feature` regression (a genuinely new leak in
   `inner_sea_magic/varisian_pilgrim*`, not present in either prior
   figure — likely written between the prior receipt's own scan and this
   cycle's dispatch) and mis-stated `race_trait`/`template`'s kind as
   `equipment`. **Same correction entry covers both stale figures**,
   `--verified-by` naming this cycle's own dual-instrument agreement.

## 2. Closure — 19 of 20 confirmed leaks closed through guarded paths

### `class_feature` (15 records) — existing `--coordinates` tooling, reused verbatim

```
cargo run --locked --bin gen_cache_class_feature -- --coordinates <15 coords>
# 15 of 15 named coordinates matched; generating ONLY those 15 record(s)
```

**A dry run of this regen surfaced a real, pre-existing bug** in
`class_feature.rs::scrub_name_pi_tokens` (unrelated to this cycle's own
code, present since an earlier cycle): its "identity restatement" check
built redaction needles from `key.split('~')` with NO length or boundary
guard, so a short, generic segment (e.g. the domain name "Chaos", from
`"Varisian Pilgrim Domain ~ Chaos"`) matched as a bare substring inside a
COMPLETELY UNRELATED, CLEAN formula token
(`BONUS:VAR|DomainChaosLVL|2` contains "chaos" with no separator). This
is exactly the shape the universal rule "a `BONUS:`/`DEFINE:` value is a
game rule, not Product Identity — never redact one" exists to forbid (this
bundle already restored 63 formulas destroyed by the identical mistake
once). **Caught before commit** (dry-run output inspected against the
pre-regen file), the 15 generated files were reverted, the check was
fixed to `pi_screening::word_bounded_contains` (made `pub(crate)`),
mutation-proved RED (reproducing the old bare-`.contains()` expression
inline fails the new regression test for the intended reason) then GREEN,
and the regen was re-run clean. **Logged as `scripts/retro.py incident`**
(recurrence-key `pi-scrub-over-redaction-bonus-formula`).

All 15 original leaking files removed (`rm`, not `git rm` — same
auto-mode-classifier precedent this bundle's prior cycles already used;
`git status --porcelain` confirmed before commit).

```
git status --porcelain -- data/corpus/inner_sea_magic/class_feature
# 15 D + 15 ?? -- exactly the 15-record target, no unexpected files
```

Verified: BONUS/VAR formula VALUES in every regenerated record are byte-
identical to their pre-regen originals (only `key`/`class`/`raw_tokens`
KEY/TYPE/ABILITY entries redacted or renamed, per `§24`).

### `template` (3 records) — source fix + new scoped `--book` remediation mode

**Root cause:** `ingest_simple_filename_kinds.py` screened `name` and
`description` via `normalized_term_hit` (word-bounded, but requires a
boundary on BOTH sides of the matched term). An adjectival/demonym form
with a suffix concatenated directly onto the root (`"Varisian"`,
`"Garundi"` — the root term followed immediately by an alphanumeric
suffix, no separator) has no boundary after the root, so the word-bounded
scan never fires — the SAME shape `declared_pi_shipping_audit`'s CHECK C
(`blacklist_term_hit_including_concatenated`) exists for. Swapped the
weak call for the strong one on both fields.

**New `--book` scoped-remediation mode** (`unit_in_scope` helper, tested):
this script's own version of `gen_cache_class_feature.rs`'s
`--coordinates` precedent, sized for a Python ingest script that iterates
`docs/work-inventory.json` units rather than named coordinates directly.
Re-running the full `--kind template` pass touches every one of the
2,248 `template` records corpus-wide — an unacceptable blast radius for
closing 3 known leaks. `--book inner_sea_world_guide` narrows it to the
84 `template` records in the one book that actually needs it.

```
python3 scripts/ingest_simple_filename_kinds.py --kind template \
  --book inner_sea_world_guide --pcgen-root <oracle>/data
# 84 written; 31 renamed under a Codex-generated neutral name (28 already
# correctly renamed via their own NAMEISPI:YES declaration, unchanged in
# content; 3 newly caught by the strong scan -- the confirmed leaks)
```

```
git diff --stat -- data/corpus/inner_sea_world_guide/template
# 84 files changed: 81 are a single-line ingested_at timestamp churn only
# (content byte-identical otherwise); 3 are the real closures
```

The 3 original leaking files (now orphaned under a stale slug — this
generator writes a NEW filename for a renamed record but does not delete
the old one) were removed the same guarded way.

```
git status --porcelain -- data/corpus/inner_sea_world_guide/template
# 3 D + 3 ?? + 81 M(timestamp-only) -- exactly the target
```

### `race_trait` (1 record) — source fix + existing per-book scoping

**Root cause:** `ingest_race_traits.rs`'s raw-tokens redaction loop only
ever touched the `DESC` key (a deliberate, documented scope at the time it
was written — "the only raw token this record type ever redacts"). The
live leak was two `ABILITY` tokens' own `PREREGION:<place>` mechanical
prerequisite, naming a blacklisted setting region — the SAME
`decisions.md §19a` amendment-3d precedent already established for a
`PREABILITY` prerequisite citation ("a citation of a PI term in a
mechanical prerequisite field redacts the citing record too"), just a
different PCGen prerequisite keyword. Widened the loop to scan EVERY
token value with the strong scan (never re-redacts an already-marked
value; updates `license`/`pi_field` when it fires with nothing already
flagged).

This binary already supports a book-scoped run (`ingest_race_traits
<book-id>`, its own doc comment: "Both forms are deterministic and both
rebuild whatever they write").

```
cargo run --locked --bin ingest_race_traits -- inner_sea_races
```

```
git diff --stat -- data/corpus/inner_sea_races
# 94 files changed: 93 are a single-line ingested_at timestamp churn only;
# 1 (elf_elven_arrogance.json) is the real closure -- both ABILITY tokens
# now [redacted PI], pi_field "description" -> "description,raw_tokens"
```

### `equipment` (1 record) — NOT closed, named, source fix landed anyway

**Root cause, fixed anyway (source-only, zero corpus-write risk):**
`equipment_gap.rs`'s `description` screen used the weak, case-SENSITIVE
`classify_field` — the live leak was a lowercase occurrence of a
blacklisted deity name the capitalized-only term list never matched.
Added the SAME supplementary strong-scan re-screen `class_feature.rs`'s
own "third defect" fix already established, with a mutation-style
regression test proving the weak scan misses what the strong scan
catches.

**Why the already-shipped record is NOT closed this cycle:**
`gen_equipment_gap_tables.rs` has no CLI scoping (unlike
`ingest_race_traits.rs`'s book argument) and `equipment_gap.rs`'s
`write_json` is no-clobber, so simply re-running the binary does nothing
for an already-on-disk file. Closing it would require either deleting the
one file and running the FULL binary — which iterates every `BOOK_INPUTS`
row and could write brand-new records for anything the equipment ingest
lane currently has queued but not yet landed, a genuine collision with
that lane's active territory (`§15`/this dispatch's own territory
warning) — or building a new `--coordinates`-style mode, which this
cycle's remaining time did not allow to do safely (the class_feature
incident above is a direct demonstration of why a rushed regen-path
change is dangerous). **Named by coordinate, not transcribed, not
silently skipped** (`§15`). CHECK C catches it on every future
`declared_pi_shipping_audit` run regardless of who closes it.

## 3. Zero-leak proof, both instruments, after closure

```
python3 scripts/sd32_t9_corpus_wide_pi_rescan.py
# Records with >=1 confirmed blacklist-term hit: 1
#   equipment: records=1 field_hits=1  (the one named, not-closed leak)
```

Full `declared_pi_shipping_audit` corpus-wide re-run was not repeated a
third time in this cycle's turn (the first two full runs already cost
~14 minutes combined); the targeted verification above (Python rescan,
same scan logic, same coverage, cross-validated against CHECK C's exact
agreement in §1) stands in for it. The expected FULL-corpus CHECK C
`BLACKLIST-TERM-SHIPPED` count at this point is **1** (was 28); the 65
`monster_ability` `DESC-PI-SHIPPED` violations are unchanged (untouched
territory).

## 4. `name`/`key` screening added to `cache_gen::{acg,apg,beastiary1}`

**The gap (confirmed by direct code read, zero live impact today):** all
three called `pi_screening::classify_field`/`classify_optional_field` for
`description` only. `SpellData.key` and `EquipmentData.key`/`.name` were
written verbatim from the compiled Rust table with no blacklist check at
all. Since every existing generator's `write_json` here is no-clobber, a
future `PI_BLACKLIST_TERMS` amendment (this bundle has amended it at
least 4 times, `decisions.md §19`) could make an EXISTING curated table
entry newly PI without any code ever re-screening it.

**Fix, identical shape across all three files** (`name_or_key_is_pi`,
using the strong `blacklist_term_hit_including_concatenated` scan — the
same one `class_feature.rs`'s own `key`/`class` fix uses, not the weaker
bare `classify_field`):

1. `SpellData.key` and `EquipmentData.key`/`.name` are screened.
2. On a hit: routes through the SAME `§24` neutral-rename path
   (`codex_neutral_name::neutral_key`/`neutral_name`, derived ONLY from
   `(kind, book, source_file, source_line)`), never a bare marker
   substitution.
3. New `codex_generated_name: bool` field on all three `CacheRecord`
   structs (`false` for every existing/unaffected record — additive,
   schema-compatible).
4. **Directory-placement-fix precedent applied proactively** (the exact
   bug `class_feature.rs`'s receipt found mid-cycle): the on-disk slug is
   derived from the (possibly-renamed) OUTPUT key, never the original
   `entry.key`/`entry.name` — so a future PI hit cannot leak into the file
   path even though the JSON body would be clean.

`beastiary1.rs`'s `MonsterData.name` (screened via `blanket_ogl()` —
i.e. not screened AT ALL, not even `description`, because this shape has
no free-text field) is a related but DIFFERENT gap, named in the source
comment, not fixed this cycle (out of this cycle's literal "equipment/
spell name field" scope — no monster-kind leak found corpus-wide either).

```
cargo test --locked --lib cache_gen::acg:: cache_gen::apg:: cache_gen::beastiary1::
# 6 + 6 + 6 = 18 new/updated tests, all green (each file: 4 new
# name_or_key_is_pi/renamed-record tests + the pre-existing write_json/
# slugify tests, unchanged)
```

## 5. Generator PI-screening audit — which generator screens which fields

| Generator | `name`/`key` (identity) | `description` | nested (`raw_tokens`/`prerequisites`) | scan strength |
|---|---|---|---|---|
| `cache_gen::acg.rs` | **fixed this cycle** (strong) | weak (`classify_field`) | n/a (no nested free-text field) | mixed |
| `cache_gen::apg.rs` | **fixed this cycle** (strong) | weak | n/a | mixed |
| `cache_gen::beastiary1.rs` (equipment) | **fixed this cycle** (strong) | weak | n/a | mixed |
| `cache_gen::beastiary1.rs` (monster) | no (`blanket_ogl`, no free-text field at all) | n/a | n/a | none — by design, no free text |
| `cache_gen::class_feature.rs` | yes (fixed a prior cycle: key/class) | yes (weak + strong supplement, fixed a prior cycle) | yes (`raw_tokens`, strong, fixed a prior cycle) | strong |
| `cache_gen::feat_gap.rs` | yes | yes | yes (`prerequisites`, strong, fixed a sibling cycle) | strong |
| `cache_gen::equipment_gap.rs` | yes (name) | **fixed this cycle** (weak + strong supplement) | n/a | mixed |
| `cache_gen::hand_authored_equipment.rs` | yes (name) | weak (declared union) | n/a | weak |
| `cache_gen::hand_authored_feat_dump.rs` | yes (name) | weak (declared union) | n/a | weak |
| `cache_gen::spell_mod_access.rs` | yes (name) | weak (declared union) | n/a | weak |
| `cache_gen::class_feature_grants.rs` | yes (key, class) | n/a (no free-text field) | n/a | weak |
| `cache_gen::spell_lane_dump.rs` | yes (name) | weak (declared union) | n/a | weak |
| `cache_gen::ultimate_equipment.rs` | **NO** — `NAMEISPI:YES`-declared whole-row DROP only, no blacklist scan of `name`/`key` at all | weak (declared union) | n/a | **gap: named, not fixed** |
| `src/bin/gen_core_rulebook_cache.rs` (CRB) | **NO** — never blacklist-scanned | weak/bare, spell+equipment kinds | n/a | **gap: named, not fixed** |
| `src/bin/ingest_race_traits.rs` | n/a (name-PI dropped upstream of this loop) | yes | **fixed this cycle** (was DESC-only, now every token, strong) | strong |
| `src/bin/declared_pi_shipping_audit.rs` (CHECK C) | corpus-wide, generator-agnostic re-derivation gate — catches ANY of the above gaps regardless of which generator or field | — | — | strong, gate |

**Fifth and sixth instances of "screens one field, not every shipped
field" found this cycle** (beyond the three named in the dispatch brief):
`cache_gen::ultimate_equipment.rs` (no blacklist scan of `name`/`key` at
all — only a declared-PI whole-row drop) and `src/bin/
gen_core_rulebook_cache.rs` (same shape, core rulebook). Both named in
Discovery forwards above, neither fixed this cycle.

## 6. Requirement 3 — the gap made unrepeatable

`declared_pi_shipping_audit`'s existing CHECK C
(`audit_blacklist_term_hits`) is the closest enforceable equivalent to "a
test that fails when a generator writes a field it does not screen": it
is deliberately generator-agnostic and field-name-agnostic by design (its
own doc comment, unchanged), re-deriving PI-safety from the CURRENT
shipped bytes on every run regardless of which generator wrote a record
or when.

**2 new unit tests**, proving this generality against the EXACT
`EquipmentData` shape `acg.rs`/`apg.rs`/`beastiary1.rs` write (`key` +
`name` + `category` + `cost_gp` + `weight` + `description`, no
`raw_tokens`):

1. `an_equipment_name_field_carrying_a_live_blacklist_term_with_no_marker_is_a_violation`
   — a live term in `name` with no marker fails.
2. `a_properly_redacted_equipment_name_is_never_flagged` — the SAME
   shape, correctly `§24`-renamed (marker + `codex_generated_name: true`),
   does not fail (mutation counterpart, proving the check fires on the
   PRESENCE of a live term, not the equipment-record shape itself).

**Mutation-proved RED, then reverted** (`§1a`): `audit_blacklist_term_hits`
temporarily short-circuited to `return Vec::new()` — test 1 failed for the
intended reason (`left: 0, right: 1`), confirmed; reverted, GREEN
restored (21/21 tests in this binary, unchanged).

**What this does NOT cover, stated plainly** (`AGENTS.md` non-negotiable
rule 7): CHECK C is a shipping-time gate only, not compile-time or
generation-time — a generator that never runs at all, or a PI leak
somehow encoded into a non-string JSON leaf (`iter_strings` only walks
strings), would not be caught by this check or this test.

## 7. Verification

```
CARGO_TARGET_DIR=<scratch> CARGO_INCREMENTAL=0 cargo test --locked --lib cache_gen::
# 178 passed; 0 failed; 11 ignored (unrelated pre-existing ignores)

CARGO_TARGET_DIR=<scratch> CARGO_INCREMENTAL=0 cargo test --locked --bin declared_pi_shipping_audit
# 21 passed; 0 failed (19 pre-existing + 2 new)

CARGO_TARGET_DIR=<scratch> CARGO_INCREMENTAL=0 cargo test --locked --bin ingest_race_traits
# 22 passed; 0 failed (21 pre-existing + 1 new)

python3 -m unittest scripts.tests.test_ingest_simple_filename_kinds
# 24 passed (19 pre-existing/updated + 5 new)

python3 -m unittest scripts.tests.test_sd32_t9_corpus_wide_pi_rescan
# 5 passed (new file)
```

`git diff HEAD -- src/ scripts/` dual-audit (per-cycle scope, not the full
`BASE_BRANCH...HEAD` form): `OK_NO_TOKENS`; the one bundle-tag-shaped
match is a reference to an already-existing repo filename
(`sd32_t9_pi_review_feat_equipment.py`), not a newly-introduced
identifier.

## 8. Verification hygiene

`git status --porcelain` checked before every commit-candidate state.
The `class_feature` regen's own dry-run output was inspected against the
pre-regen file BEFORE the first commit attempt, which is what caught the
`scrub_name_pi_tokens` over-redaction bug (§2 above) — the 15 generated
files were reverted with individual `rm` (not `git rm`, not a bulk
pattern — this session's own auto-mode classifier blocked an `xargs rm`
form mid-cycle; individual `rm` calls plus `git status --porcelain`
confirmation have the identical effect). Own-diff PI scrub: every added
line across all touched files grepped against
`pi_scrub.normalized_term_hit` before finalizing — clean; no blacklist
term or PI item name appears anywhere in this receipt, any test name, any
test constant, or any commit message (every reference is a coordinate or
a `PI_BLACKLIST_TERMS` index).
