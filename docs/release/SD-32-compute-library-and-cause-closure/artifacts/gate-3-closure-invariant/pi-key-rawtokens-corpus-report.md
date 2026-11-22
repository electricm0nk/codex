# `data.key` / `data.raw_tokens` PI screen — corpus-wide report

**Cycle:** `pi-key-rawtokens-screen` (SD-32 card 11). **Date:** 2026-08-23.
**Follows up:** `scripts/retro.py` deferral `1787491744623-sd32-t9-onboarding-957b2f`
(the `§24` ability-rename cycle's own discovery of the 503-record unratified-vocabulary
figure). Read that event first — its `revisit` field is this cycle's brief.

## The gap this cycle closes

Every PI screen in this codebase before this cycle compared a record's bare `name` /
`description` against a blacklist and against the row's own `NAMEISPI:`/`DESCISPI:`
declaration. Nothing compared `data.key` or the non-`DESC` entries of `data.raw_tokens`.
A record can pass every existing gate — clean name, no declaration — and still ship a
published campaign-setting deity or proper-noun name inside another token's value. Two already-shipped
`ability` records proved this live and are the two CONFIRMED leaks this cycle fixes (see
"1. The two confirmed leaks" below).

**Two vocabularies. Keep them apart. This report never conflates them:**

| Vocabulary | Authority | What a hit means |
|---|---|---|
| The 60-term `PI_BLACKLIST_TERMS` list | **Operator-SIGNED-OFF**, `decisions.md §19`, `ogl-pi-blacklist.md` | A **confirmed** PI leak — actionable now. |
| Capitalized-word-shaped tokens not on that list | **Unratified.** A heuristic this cycle built, approved by nobody | A **candidate** — reported for an operator decision, never acted on. |

## Tooling

`scripts/pi_key_rawtokens_audit.py` (new, this cycle). Generic, not per-kind
(`decisions.md §17`): walks every `data/corpus/<book>/<kind>/*.json` file, and for every
record whose `data.name` is genuinely clean (see the `§17a` correction below for exactly
what "clean" means), scans `data.key` and every non-already-redacted `data.raw_tokens`
value with the same word-boundary/case-fold/OCR-normalized scan (`normalized_term_hit`)
every other T9-era PI tool already uses.

Re-derive:

```bash
python3 scripts/pi_key_rawtokens_audit.py --json-out report.json
```

## `§17a` correction — the first run was wrong, and by a lot

The first version of this screen treated `data.name == "[redacted PI]"` as "clean" (the
literal marker string contains no blacklist term), so a record whose name an EARLIER
screen had ALREADY correctly redacted was wrongly re-reported as a fresh leak, because its
surviving mechanical raw_tokens (e.g. a `REGION:` field on an already-redacted
`template` record) were mistaken for a new leak a clean name was hiding.

- **First run:** 37 confirmed records (`domain` 3, `equipment` 1, `language` 1, `spell` 1,
  `template` 31).
- **Root cause found by manual spot-check of the sample** (per `AGENTS.md`'s "validate a
  proxy against a known case before trusting a confident claim it produces"): 26 of the
  first 30 sampled records had `data.name == "[redacted PI]"` already.
- **Fixed:** `name_already_flagged()` now treats the literal redaction marker as
  already-flagged, not clean.
- **Corrected run:** **4** confirmed records. Logged as `scripts/retro.py correction`
  `1787493549497-t9-onboarding-01846b`.

**Every number below the line is from the corrected run.** Command:
`python3 scripts/pi_key_rawtokens_audit.py --json-out report.json` (60-term list,
`scanned_records: 24051`, `name_already_pi_skipped: 61`).

## 1. The two confirmed leaks — fixed this cycle

| Record | Field | Term | Fix |
|---|---|---|---|
| `data/corpus/inner_sea_gods/ability/adept.json` | `raw_tokens[SPELLLEVEL]` | 1 blacklist deity name | Redacted; `pi_field` now includes `raw_tokens` |
| `data/corpus/inner_sea_magic/ability/diplomatic_student.json` | `raw_tokens[PREABILITY]` | 1 blacklist institution name | Redacted; `pi_field` now `description,raw_tokens` |

Fixed through the guarded generator path (`scripts/ingest_ability.py`'s new
`scrub_blacklist_pi_tokens`, applied to every non-renamed record, not only the 576 whose
own name is PI) — never by hand-editing `data/corpus/**`. Re-running the generator over
the full 4,824-record `ability` population changed exactly these 2 files
(`"changed": 2, "unchanged": 4822"` in the generator's own `--out` report); every other
already-shipped `ability` record is confirmed byte-unchanged.

## 2. Confirmed corpus-wide, under the signed-off 60-term list — NOT remediated this cycle

| Kind | Book | Record | Term |
|---|---|---|---|
| `domain` | `core_rulebook` | `domain/death.json` | 1 blacklist deity name |
| `equipment` | `inner_sea_gods` | `equipment/wayfinder_of_zephyrs.json` | 1 blacklist deity name |
| `language` | `inner_sea_temples` | `language/nightsong.json` | 1 blacklist deity name |
| `spell` | `advanced_players_guide` | `spell/bard_s_escape.json` | 1 blacklist proper-noun name |

**By kind:** `domain` 1, `equipment` 1, `language` 1, `spell` 1 — 4 total.
**By book:** `core_rulebook` 1, `inner_sea_gods` 1, `inner_sea_temples` 1,
`advanced_players_guide` 1 — 4 total.

Out of this cycle's explicitly named scope (the 2 `ability` records only). Each of these 4
kinds has its own generator/enrichment script this cycle did not inspect; a safe fix needs
the same guarded-generator-path discipline as the `ability` fix, per kind, plus a
`corpus_literal_sweep` re-verification per write path. Logged as `scripts/retro.py`
deferral `1787493585450-t9-onboarding-bcf0ca` for a follow-up cycle. **Never hand-edit
these files** — the same rule that governs the 2 already fixed.

## 3. Unratified-vocabulary candidates — report only, no action taken

**23,062 of 24,051 scanned records** show at least one capitalized, proper-noun-SHAPED
token not on the 60-term list, via a heuristic regex+stoplist scan
(`candidate_terms()` in the new tool).

**This number is not a usable estimate of real PI exposure and this report does not claim
it is.** Spot-checking the top 30 terms by frequency shows the heuristic is dominated by
ordinary game-mechanical vocabulary, not proper nouns:

```
3237 Base       2840 Weapon     2247 Melee      2203 Internal
2175 Magic      2124 Racial     2062 Natural    1966 Finesseable
1757 Walk       1573 Group      1489 Psychic    1419 Bludgeoning
1242 Trait      1217 Wondrous   1195 Bite       1064 Traits
 903 Piercing    888 Combat      865 Power       845 Sorcerer
 805 Slashing    804 Psionic     800 Medium      779 Companion
```

Candidate population by kind (heuristic, unratified):

```
ability 4801, class 141, companion 1478, domain 180, equipment 3652, feat 1219,
language 116, monster 1242, monster_ability 1902, power 421, race 39,
race_trait_generic 1877, skill 148, spell 3748, template 2098
```

Full per-book breakdown and the 60-term top list are in
`scripts/pi_key_rawtokens_audit.py --json-out`'s machine-readable output (not
committed verbatim here — it is 23,062 records' worth of samples, reproducible on demand
from the command above).

### The exact question for the operator

This is **not** "should these 23,062 records be redacted" — the sample above shows most of
that population is ordinary mechanic vocabulary, not Product Identity, exactly the outcome
`ogl-pi-blacklist.md`'s own standing caution warns against guessing past. The real
question is narrower:

> **Is it worth a follow-up cycle building a higher-precision candidate detector (e.g.
> cross-referencing hits against a real English/game-term dictionary, or scoping the scan
> to prose-shaped fields only rather than every mixed-case mechanical token) before this
> 23,062 figure means anything actionable — and if so, what precision bar clears it for
> operator review?**

Until that ruling, the blacklist stays exactly the operator-approved 60 terms
(`decisions.md §19`); this report changes no code path's redaction behavior and does not
touch `ogl-pi-blacklist.md`.

## `declared-pi-audit` — confirmed unrelated, pre-existing

`cargo run --bin declared_pi_shipping_audit` reports **28 violations across 28 files**,
all `NAME-PI-SHIPPED` in `language`/`template` kinds (e.g.
`bestiary_4/language/mi_go.json`, `inner_sea_races/template/human_ethnicity_arcadian.json`).
**Different defect shape from this cycle's screen**: `declared-pi-audit` finds a record
whose OWN `NAMEISPI:YES` declaration was never honored — the record's declared-PI **name**
shipped unredacted. This cycle's screen finds a **clean, non-declared** name with a
blacklist term hiding in a different field. Confirmed pre-existing and untouched by this
cycle's diff: this cycle's changes touch only `scripts/ingest_ability.py`,
`scripts/pi_key_rawtokens_audit.py`, `src/rules_core/pi_screening.rs`, their tests, and
the 2 named `ability` corpus files — no `language` or `template` file, and no code path
`declared_pi_shipping_audit` reads.
