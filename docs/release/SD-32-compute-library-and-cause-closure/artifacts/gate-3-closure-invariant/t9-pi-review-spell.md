# T9 PI review — `spell` kind (352 uncertain units)

**Actor:** `spell` (per-record review lane, `decisions.md §18`). **Scope:** read-only. Transcribes
nothing, ingests nothing, changes no corpus data, does not amend `docs/governance/ogl-pi-blacklist.md`
(status stays `DRAFT`). This memo's `proposed_rule` is a proposal for the operator, clearly not
applied. Extends `scripts/sd32_t9_pi_exposure_audit.py` and its memo
(`t9-pi-exposure-audit.md`) — does not redo them.

**Base:** `b4192a7128843ec43ab854fe5926e3d498b13483` (`decisions.md §18` committed, the pinned base
for this cycle) = `origin/tranche/12` tip at the start of this cycle. Oracle:
`PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6`, bootstrapped fresh into this worktree's
git-ignored repo-local slot via `scripts/fetch-pcgen-oracle.sh` and confirmed matching the pin
before any figure below was trusted.

**Read this memo without reading any other file first.** Every figure carries its re-derive
command. Classification script: `scripts/sd32_t9_pi_review_spell.py`.

## 1. Re-derivation — spell population and bucket split, unchanged

```bash
cargo build --locked --release --bin v06_work_inventory
"$CARGO_TARGET_DIR/release/v06_work_inventory" --stdout-only > fresh_inventory.json
python3 scripts/sd32_t9_pi_review_spell.py fresh_inventory.json --corpus-root "$PCGEN_CORPUS_ROOT"
```

Re-derived on a fresh oracle fetch at the tranche/12 tip: **spell kind, total=732, blocked=31,
clear=349, uncertain=352.** This byte-matches `t9-pi-exposure-audit.md §3`'s spell row exactly —
**no correction against the audit's own spell-kind figures.**

**One correction filed, not against spell** — `python3 scripts/retro.py correction` run this
cycle: the audit's *T9-wide total* (2,712) does not re-derive clean on this branch tip any more.
`scripts/sd32_t9_census.py` on the same fresh inventory now reports **3,573** T9 units, entirely
because `monster_ability` grew from 517 to 1,378 (a later, unrelated commit on `tranche/12`
between the audit's base and this cycle's pin — outside this lane's scope to diagnose). **The
spell-kind figure itself is unaffected and re-derives identically to the audit's**; logged so
whichever lane owns `monster_ability`'s uncertain bucket knows its own denominator has moved.
(`docs/retro/events/spell.jsonl`, `--verified-by "python3 scripts/sd32_t9_census.py
fresh_inventory.json"`.)

## 2. Per-record review of the 352 `uncertain` spell units

**Method:** for every one of the 352 (all `DESC:`-bearing, no `NAMEISPI:`/`DESCISPI:` declaration,
no exact-substring term hit — exactly `ogl-pi-blacklist.md §2.3`'s named `SpellCacheData.description`
category), the script extracts the full `DESC:` text and scans it for any capitalized word that
is (a) not one of ~180 ordinary D&D/Pathfinder mechanical-vocabulary words established by running
this pass over the full set and reading every word's real sentence context (ability scores, skill
names, condition names, PCGen row-format tokens, citation-footer tokens like `Core Rulebook`,
`Pathfinder RPG`), and (b) not a roman numeral (spell-level suffixes like `Summon Monster IX`).
Anything left over is a genuine candidate proper noun the 57-term blacklist does not enumerate,
and gets read by hand in its full sentence context — not auto-classified.

**Result: 350 clear, 2 still_undecidable, 0 blocked.**

```
reviewed=352 -> blocked=0 clear=350 still_undecidable=2
  STILL_UNDECIDABLE: inner_sea_races:Bleaching Resistance -- unlisted candidate proper noun(s) in DESC: ['Bleaching', "Bleaching's"]
  STILL_UNDECIDABLE: monster_codex:Gift of the Deep -- unlisted candidate proper noun(s) in DESC: ['Molenti']
```

**The two still-undecidable records, read in full:**

- **`inner_sea_races:Bleaching Resistance`** — `DESC:` reads: *"Ability score drain from the
  Bleaching doesn't affect your ability score modifiers while you're affected by this spell. This
  spell does not remove the Bleaching's effect on your appearance, nor does it prevent you from
  suffering further ability penalties, dying, or becoming a bleachling as a result of the
  Bleaching."* **"The Bleaching"** (and the derived noun "bleachling") is a specific named
  in-world phenomenon from Paizo's own Golarion setting (Inner Sea Races' elven-curse content),
  not a generic mechanical description — it reads as OGL §1(e) "storylines... thematic elements...
  incidents," the same category the blacklist's own `deity`/`place_name` entries are drawn from.
  **This reviewer's judgment leans PI**, but it is not on the 57-term list and the blacklist gives
  no rule for "named setting phenomenon," so it is reported `still_undecidable` rather than forced
  — per this bundle's explicit instruction not to guess a licensing call.
- **`monster_codex:Gift of the Deep`** — `DESC:` describes optional sahuagin-mutation benefits,
  one bracketed option named `[Molenti]`: *"The sahuagin's features shift to resemble those of an
  aquatic elf..."* The sibling bracketed options in the same spell (`[Four-Armed]`,
  `[Prehistoric]`, `[Shark-Blooded]`, `[Sightless]`, `[Spined]`) are all plain descriptive English
  compounds — generic OGL-shaped labels. `Molenti` is not a descriptive English word; it reads as
  a specific named creature-variant/culture label, which OGL §1(e) explicitly lists under
  "creatures... personas." **Same disposition as above: leans PI, reported `still_undecidable`.**

**No other candidate survived hand-reading.** An earlier, less-refined pass of this same script
flagged 8 candidates (adding `Climb`, `Drain`, `Unholy`, `Ultimate`, `Perception`, `Strands`,
`Shark`, `Dousing` — all ordinary mechanical/skill vocabulary once read in context, e.g.
`Perception` is the Pathfinder skill name and `Strands` in `Searching Shadows`'s `DESC:` is plain
English, "strands of information"). Those 6 were corrected into the common-vocabulary allowlist
after reading their full sentences and are `clear` in the final run; the two above survived that
same read and did not resolve to ordinary vocabulary.

**Cross-check against known Golarion place names not on the 57-term list** (`Thassilon`,
`Kaer Maga`, `Magnimar`, `Riddleport`, `Cassomir`, `Westcrown`, `Egorian`, `Korvosa`, `Hermea`,
`Katheer`, `Jistka`, `Shory`, `Azlant`, `Shackles`, `Sargava`, `Belkzen`, `Iobaria`, and several
regional/demonym variants) found **zero** additional hits across all 352 `DESC:` fields — this is
a targeted supplementary check, not exhaustive, but it found nothing beyond the two above.

## 3. Clear-bucket recheck — normalized (case-fold + bounded OCR) scan, 349 units

`decisions.md §18` point 2 puts the `clear` bucket in scope, not only `uncertain`. Ran a
case-folded, word-boundary-matched scan with a small OCR-confusion table (`l`/`1` → canonical `i`,
matching the recorded `lrori`/`Irori` incident's error class; `rn`→`m`) over all 349 `spell`
`clear` rows and, for completeness, all 352 `uncertain` rows too.

```
clear bucket rechecked=349 newly_blocked=0
```

**`newly_blocked = 0`, `newly_uncertain = 0` for the `spell` kind.** No record currently believed
safe was found to carry a normalized term hit.

**A real false-positive class was found and fixed while building this scan, recorded here because
a future cycle rebuilding it from scratch will hit the same trap:**

1. **Naive case-folding alone reopens a hole the original scan's case-sensitivity closed.** The
   blacklist term `Nex` (a 3-letter Golarion place name) case-folds to `nex`, which is a substring
   of the ordinary word `next`. The *original* exact-substring scan is case-sensitive, so `"your
   next attack"` never matched `Nex` (capital `N` protects it) — but a naive case-folded re-scan
   matched 5 spell records purely on this collision (`Quickened True Strike`, `True Skill`,
   `Violent Accident`, `Endothermic Touch`, `Spellsteal` — all via ordinary phrases like "your
   **next** attack"). Fixed by requiring a word-boundary match on the normalized text, not a bare
   substring; re-ran clean (0 false positives, still catches both recorded incidents — see below).
2. **`|` must NOT be folded into the OCR-confusion set.** `|` is PCGen's own literal
   field/sub-value delimiter in these raw rows (`FACTSET:Deity|Cayden CaiLean`), not an OCR
   artifact of prose. An early version of this scan folded `|`→`i` alongside `l`/`1`, which glued
   `...Deity|` onto `Cayden` and produced a **false negative on the recorded `Cayden CaiLean`
   incident itself** — confirmed by testing the scanner directly against
   `FACTSET:Deity|Cayden CaiLean` before and after removing `|` from the table.
3. **Verified the scan still catches both `ogl-pi-blacklist.md §4`'s recorded incidents** after
   both fixes: `normalized_term_hits("FACTSET:Deity|Cayden CaiLean")` → `['Cayden Cailean',
   'Cayden CaiLean']`; `normalized_term_hits("...lrori...")` → `['Irori', 'lrori']`.

## 4. Proposed `§2.3` entry for `spell` (proposal only — not applied)

The blacklist already names `SpellCacheData.description` in §2.3; this cycle's finding is that the
existing entry's guidance ("per-record judgment... redact the specific offending sub-string") is
correct in shape but under-specified on *what a reviewer is looking for beyond the term list*.
Proposed addition to the existing `SpellCacheData` / `description` row:

> **Proposed §2.3 note, `SpellCacheData.description`:** In addition to the 57-term blacklist scan,
> a per-record review must read `DESC:` for (a) named in-world phenomena/curses/events specific to
> Golarion (OGL §1(e) "storylines... thematic elements... incidents") even when no proper noun
> from the term list appears — example found this cycle: `inner_sea_races:Bleaching Resistance`'s
> "the Bleaching" — and (b) named creature-variant/subrace labels inside bracketed
> multiple-choice-effect spells (OGL §1(e) "creatures... personas") — example found this cycle:
> `monster_codex:Gift of the Deep`'s `[Molenti]` option. Both examples are reported
> `still_undecidable` by this cycle, not pre-classified, because the blacklist gives no
> affirmative rule for either shape yet.

## 5. `.MOD`/`.COPY` question — spell kind

**0 of 732 spell units are `.MOD`/`.COPY`-shaped rows** (checked both the unit's `corpus_key` and
its display `name` for a `.MOD`/`.COPY`/`.FORGET` suffix — none found; re-derive:
`python3 scripts/sd32_t9_pi_review_spell.py fresh_inventory.json --corpus-root
"$PCGEN_CORPUS_ROOT"` prints `.MOD/.COPY-shaped spell units: 0`). **Recommendation for the
`spell` kind specifically:** no rule is needed — the population this review covers has no
`.MOD`/`.COPY` overlay rows to inherit a target's PI status from. (The audit's §8 point 3 records
this pattern for `monster`-kind rows; this review's own `spell`-kind population confirms it is not
also a `spell`-kind exposure.)

## 6. Spot-check material for the operator (10 records, this cycle's own call + reason)

| Record | Book | Call | Reason |
|---|---|---|---|
| `Bleaching Resistance` | inner_sea_races | **still_undecidable** (leans PI) | `DESC:` names "the Bleaching," a Golarion-specific curse/event, not on the 57-term list |
| `Gift of the Deep` | monster_codex | **still_undecidable** (leans PI) | `[Molenti]` bracketed option reads as a named creature-variant label, not descriptive English like its siblings |
| `Blur (self only)` | bestiary | clear | `DESC:` is the verbatim core SRD `Blur` mechanic ("...concealment (20% miss chance)") — no proper noun of any kind |
| `Summon Monster IX (Fiends Only)` | bestiary | clear | Monster-variant of a core SRD spell; `IX` is a spell-level roman numeral, not a name |
| `Hellfire Ray` | book_of_the_damned_volume_1 | clear | Mechanical damage/save text only; "Hellfire" is a generic descriptor, not a proper noun |
| `Curse Terrain (Supreme)` | horror_adventures | clear | Pure mechanical terrain-effect prose |
| `Swarm of Fangs` | monster_codex | clear | Generic swarm-attack mechanic text |
| `Plane Shift (self only)` | bestiary | clear | Core SRD spell mechanic, monster-restricted variant, no PI content |
| `Air of Authority` | inner_sea_races | clear | Mechanical `DESC:` (skill/social bonus text); "Air" and "Authority" are ordinary English, no setting-specific referent |
| `Aldori Alacrity` | adventurers_guide | **blocked** (already, audit §3) | `NAMEISPI:YES` declared — included for contrast with the `still_undecidable` calls above |

## 7. Environment / reproduction summary

```bash
export PCGEN_REPO_DIR="$(git rev-parse --show-toplevel)/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen"
export PCGEN_CORPUS_ROOT="$PCGEN_REPO_DIR/data"
scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR"   # PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6
cargo build --locked --release --bin v06_work_inventory
"$CARGO_TARGET_DIR/release/v06_work_inventory" --stdout-only > fresh_inventory.json
python3 scripts/sd32_t9_census.py fresh_inventory.json               # spell=732, confirmed; T9 total now 3,573 (see §1 correction)
python3 scripts/sd32_t9_pi_review_spell.py fresh_inventory.json \
    --corpus-root "$PCGEN_CORPUS_ROOT" --json-out spell_review.json  # this memo's §2/§3/§5
```

`df -h /` at the end of this cycle: **665G available, 32% used** (968G filesystem).

## 8. Summary for the operator

- **352 uncertain spell units reviewed, per record.** 350 resolve to `clear` (mechanical/generic
  OGL prose, no proper noun beyond the 57-term list). **2 remain `still_undecidable`** —
  `Bleaching Resistance` and `Gift of the Deep` — both leaning PI in this reviewer's judgment, but
  neither matches an existing blacklist rule, so neither is auto-classified.
- **0 newly blocked** in the 349-unit `clear` bucket under a normalized (case-fold + bounded OCR)
  re-scan — the `spell` kind's `clear` bucket holds up under the same class of check that caught
  the recorded `Cayden CaiLean`/`lrori` incident elsewhere. That check itself needed a
  word-boundary fix to avoid a `Nex`-in-`next` false-positive class, documented in §3 so a future
  cycle doesn't rediscover it the hard way.
- **0 `.MOD`/`.COPY` spell units** — no cross-reference question to answer for this kind.
- **Proposed `§2.3` addition** (§4) — not applied, blacklist stays `DRAFT`.
- **One correction filed** — the T9-wide population total has moved (2,712→3,573) since the
  audit's base, entirely in `monster_ability`, outside this lane's kind and scope; `spell`'s own
  figures are unaffected and reproduce exactly.
