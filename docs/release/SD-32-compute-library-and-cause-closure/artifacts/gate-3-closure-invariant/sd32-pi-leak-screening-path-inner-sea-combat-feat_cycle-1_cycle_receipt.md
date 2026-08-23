# Cycle sd32-pi-leak-screening-path-inner-sea-combat-feat — Epic 2 / Card 11 (`epic-2-cause-closure`)

- **Card ID:** `epic-2-cause-closure`
- **Commit SHA:** (recorded after push, see push output)
- **Files touched:** `src/rules_core/cache_gen/feat_gap.rs`,
  `src/rules_core/cache_gen/hand_authored_feat_dump.rs`,
  `data/corpus/inner_sea_combat/feat/falling_water_gambit.json`,
  `data/corpus/inner_sea_combat/feat/duelist_of_the_shrouded_lake.json`,
  `data/corpus/inner_sea_combat/feat/duelist_of_the_roaring_falls.json`,
  `data/corpus/inner_sea_gods/feat/protective_channel.json`
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion:** close the 3 (re-derived: 4) pre-existing `feat` PI leaks logged-not-fixed
  by `sd32-integrity-sweep-stale-pair-scan-and-pi-blacklist-sync` (commit `ec060ad20c`); find and fix
  the screening-path defect that let them ship, not just the 3 records; regenerate through the guarded
  path; prove the class closed by re-scanning every kind for unredacted hits against the full 61-term
  list.
- **Corpus SHA:** `7f818006e371188e5717fd18d74d18a420747fc6` (`PCGEN_ORACLE_SHA`, confirmed on-pin via
  `scripts/verify.sh --only preflight-oracle` after bootstrapping this fresh worktree's empty oracle
  slot with `scripts/fetch-pcgen-oracle.sh`)
- **Status:** complete
- **Notes:** see below.
- **Discovery forwards:** `class_feature` kind carries confirmed unredacted hits against the same
  61-term list — named by coordinate/count below, out of this cycle's territory (`class_feature`
  ingest + `cache_gen/class_feature.rs` is a sibling lane's named territory). Handed off, not fixed.
- **Next-cycle plan:** the `class_feature` lane closes its own confirmed hits (see "Class-closure scan"
  below); no further `feat`-kind work is open.

## 1. Re-derivation (`§17a`) — the population was 3 claimed, 4 actual

The dispatch brief's 3 coordinates (`falling_water_gambit.json`, `duelist_of_the_shrouded_lake.json`,
`duelist_of_the_roaring_falls.json`, all `inner_sea_combat/feat`) were confirmed live and unchanged.
Re-scanning `data/corpus/**/feat/*.json` against every `data.*` string/list/dict field (not just
`name`+`description`, the fields the brief's own upstream lane had checked) using
`pi_scrub.normalized_term_hit` (word-bounded, OCR-normalized — the same scan the signed-off blacklist
requires) surfaced a **fourth** record the brief did not name:
`data/corpus/inner_sea_gods/feat/protective_channel.json`, hit in `data.prerequisites`.

Command: a Python walk of `data/corpus/**/feat/*.json`, screening every `data` field (string, or
joined list/dict values) via `scripts/pi_scrub.normalized_term_hit`, keeping only records whose
top-level `license` was not already `pi_redacted`/`PiRedacted`. **4 confirmed leaked records, 0 false
negatives found in the `feat` kind.** Logged as `scripts/retro.py correction`
(`docs/retro/events/t9-onboarding.jsonl`, id `1787512582838-t9-onboarding-0dc310`).

Every one of the 4 records' matching term is the same one the prior cycle identified by index (61-term
list, index 57) — the one added to both the Rust and Python blacklist copies before this cycle
(`decisions.md §19a` amendment 3d), and NOT a new or different term. The 4th record's hit is an
upstream PCGen spelling variant of the same term (lowercase first letter for a capital), which only
the OCR-normalized scan — not a bare-substring scan — catches; see root cause below.

## 2. Root cause — the write path

All 4 leaking records trace to `cache_gen::feat_gap::generate()` (`src/rules_core/cache_gen/feat_gap.rs`,
the `gen_cache_feat_gap` binary), git-blamed to commit `1410424cf3` (`Sun Aug 23 07:50:01 2026`),
which mirrors `cache_gen::equipment_gap`'s already-proven shape for the same kind. **Two independent
defects compound:**

1. **`prerequisites` was never screened at all.** `FeatData.prerequisites` was written straight from
   the compiled table's `entry.prerequisites` with zero call into `pi_screening`. `name` was screened
   (whole-record exclusion) and `description` was screened (whole-value redaction) — the same
   "screens one branch, not every shipped field" shape a sibling lane already named in
   `cache_gen::class_feature.rs`'s own `redact_concatenated_blacklist_tokens` doc comment, for a
   different field (`raw_tokens`) in a different generator. `protective_channel.json`'s own
   `description` was correctly redacted at generation time (the term was already on the blacklist);
   its `prerequisites` — carrying the exact same fact, an upstream-typo'd spelling of the same deity
   name — shipped raw, because nothing ever screened that field. This is the defect that would recur
   for any book, regardless of blacklist-term timing.
2. **No re-screen on regeneration.** `write_json`'s no-clobber policy (by design — protects a
   different already-committed record at the same slug) means a record written once is never
   rescreened when the blacklist term list later grows. The 3 `inner_sea_combat` records' own
   `description` field *would* now correctly redact under current code (bare-substring `.contains`
   does match the term as spelled in those 3 records) — it did not at generation time because the
   term in question (index 57 of the 61) was added to `pi_screening.rs` at commit `a3d9f066a7`
   (`10:04:23`), roughly two hours *after* `1410424cf3` (`07:50:01`) first wrote these files. This is a
   process gap, not a code bug — the fix for (1) plus a guarded regeneration (below) closes both.

`src/rules_core/cache_gen/hand_authored_feat_dump.rs` (the `feat` kind's other cache generator, same
`FeatData` shape, same `prerequisites`-unscreened gap) was checked and fixed identically as defense in
depth, even though the corpus-wide scan found **zero** currently-leaked records from that path — the
same defect class, caught before it produced a live leak.

## 3. Fix (TDD, RED proved for the intended reason)

New `cache_gen::feat_gap::screen_prerequisites(&[String]) -> (Vec<String>, bool)`: screens each
`prerequisites` line against `pi_screening::blacklist_term_hit_including_concatenated` (word-bounded,
OCR-normalized, same scan the description/name screens already use elsewhere in this codebase) and
redacts only the offending line(s), leaving unrelated `PRE*` lines untouched. Wired into both
`feat_gap::generate()` and `hand_authored_feat_dump::generate()`; the record-level `license`/`pi_field`
now unions across `description` and `prerequisites` (mirrors `cache_gen::class_feature.rs`'s existing
multi-field union pattern — comma-joined `pi_field`, e.g. `"description,prerequisites"`).

4 new unit tests added to `feat_gap.rs`'s existing test module: a plainly-spelled term, the real
upstream-typo'd variant (proves the OCR-normalized scan is required — a bare-substring scan would
have missed the 4th record), a mixed clean+hit line (proves only the hit line is redacted), and a
no-hit control.

**RED proved for the intended reason:** temporarily replaced `screen_prerequisites`'s body with a
pass-through no-op (`(prerequisites.to_vec(), false)`), ran the 4 new tests — 3 of 4 failed
(`assertion failed: any_redacted`) for exactly the missing-screening reason, the 4th (the no-hit
control) correctly still passed. Restored the real implementation; all 4 tests green, along with the
existing 10 `feat_gap` tests (14/14) and both `hand_authored_feat_dump` tests (2/2).

```
CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-pi-leak-screening-path CARGO_INCREMENTAL=0 \
  cargo test --locked --lib cache_gen::feat_gap
# 14 passed; 0 failed
CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-pi-leak-screening-path CARGO_INCREMENTAL=0 \
  cargo test --locked --lib hand_authored_feat_dump
# 2 passed; 0 failed
CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-pi-leak-screening-path CARGO_INCREMENTAL=0 \
  cargo build --locked --lib
# clean
```

## 4. Regeneration through the guarded path (never hand-edited)

`git rm` on the 4 confirmed-leaking files (a deletion, not an edit — the no-clobber write policy
means the generator will not touch a file that already exists, so a fresh, correctly-screened write
requires the slot to be empty first). Then:

```
CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-pi-leak-screening-path \
  PCGEN_CORPUS_ROOT=<repo-local pcgen oracle>/data \
  cargo run --locked --bin gen_cache_feat_gap
# Feat gap cache generated: 4 feat records; ingested_at=2026-08-23T19:08:27Z
# NOTE: 645 record(s) skipped -- a different record already claims that slug: [...]
```

`git status --porcelain -- data/corpus/` before commit showed **exactly the 4 target files** touched
(4 stage-as-deleted + 4 untracked-new at the identical paths) — the no-clobber policy protected every
other one of the gap lane's 649 rows, confirmed by name in the binary's own "skipped" list. No
unexpected deletion or overwrite.

All 4 records' `license`/`pi_field`/`pi_marker` now correctly read `PI-REDACTED` /
`description,prerequisites` (or `prerequisites` alone for `protective_channel.json`, whose
`description` was already correctly redacted before this cycle) / `redacted`, and the previously-raw
prose is replaced with `[redacted PI]` in every offending field.

## 5. Class-closure scan (`§4` of the dispatch brief) — every kind, full 61-term list

Command: the same generic field-scan as §1 above, run over the **entire** `data/corpus/**` tree
(51,142 records across every kind, keyed by the directory 2 levels below `data/corpus/`), after the
fix and regeneration above:

| Kind | Confirmed unredacted hits |
|---|---:|
| every kind except `class_feature` (17 kinds: `ability`, `class`, `companion`, `deity`, `domain`, `equipment`, `equipment_modifier`, `feat`, `feat_generic`, `language`, `monster`, `monster_ability`, `power`, `race`, `race_trait`, `skill`, `spell`, `template`) | **0** |
| `class_feature` | **31** (28 real + 3 false-positive, see below) — **not this cycle's territory, handed off by coordinate** |

`feat`: **0** (was 4). `class_feature` is `cache_gen/class_feature.rs`'s and the `class_feature`
lane's named territory per this cycle's dispatch brief and is not touched here.

**`class_feature`'s 31, named by coordinate for the owning lane** (grouped by directory, all under
`data.class`/`data.name`/`data.description`):

- `adventurers_guide/class_feature/aldori_swordlord/*` — 12 records (the archetype's own name is the
  hit, in `data.class`)
- `adventurers_guide/class_feature/aldori_defender/*` — 4 records (same shape)
- `adventurers_guide/class_feature/magaambyan_arcanist/*` — 11 records (same shape, different term)
- `adventurers_guide/class_feature/magaambyan_initiate/*` — 2 records (same shape)
- `inner_sea_combat/class_feature/ranger_combat_style/cayden_callean.json` — 1 record (`data.name`)
- `inner_sea_combat/class_feature/ranger/codex_named_unit_class_feature_inner_sea_combat_isc_abilities_class_lst_256.json`
  — 1 record (`data.description`)

**3 of the 31 are a confirmed false positive, not a leak**, a distinct instance of the exact class
`decisions.md §26` already named for a different term pair (an OCR-fold canonical form colliding with
an ordinary English word): `horror_adventures/class_feature/dreadnought/steady_gait.json`,
`advanced_race_guide/class_feature/buccaneer/seadog_s_gait.json`, and
`advanced_players_guide/class_feature/shifter_s_blessing/form_of_the_cat.json` all hit on the
blacklisted nation name via its OCR-fold canonical form (`l`→`i`), which collides with the ordinary
English word "gait" ("Steady Gait", "Seadog's Gait", "...his gait more deliberate..." — none of the
three has any connection to the nation). Confirmed by direct canonicalization
(`scripts/pi_scrub.canonicalize("Galt") == scripts/pi_scrub.canonicalize("gait") == "gait"`) and by
reading each record's real prose. Not fixed here (out of territory, and not a leak — no action is the
correct disposition for a confirmed false positive); named so the `class_feature` lane does not
re-discover it from scratch, and so a future blacklist-fold change is checked against this collision
the same way `§26` already checks `Jarn`/`jam`.

**Cross-check, unaffected by this cycle's diff:** `cargo run --locked --bin
declared_pi_shipping_audit` — **65 violations, unchanged before and after** (all
`bestiary_4/monster_ability`, `DESC-PI-SHIPPED`, a `pi_field` metadata-tagging gap on already-redacted
content, not a raw-PI leak; pre-existing, named in `decisions.md §26`, `monster_ability` lane's
territory, not touched by this cycle's Python-and-`feat_gap.rs`-only diff).

## 6. Verification hygiene

`git status --porcelain` checked before every commit-candidate state; the only unexpected line seen
throughout this cycle was `docs/retro/events/sd31-transcribe.jsonl` (an append-only retro-log line
this cycle's own `scripts/verify.sh --only preflight-oracle` bootstrap run wrote automatically —
confirmed by content, not assumed) — no unexpected deletion or overwrite of another lane's work.
