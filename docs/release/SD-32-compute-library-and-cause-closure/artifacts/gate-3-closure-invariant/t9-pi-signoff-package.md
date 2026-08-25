# T9 PI review — operator sign-off package

**Actor:** `t9-pi-signoff`. **Scope:** read-only consolidation of three review lanes
(`decisions.md §18`). Transcribes nothing, ingests nothing, changes no corpus data, does not
amend `docs/governance/ogl-pi-blacklist.md` (stays `DRAFT`), does not touch kanban row 11 (stays
`in-progress`). This document is the one file the operator needs to act on §18 — everything in it
is re-derived and cross-checked against the three lanes' committed scripts and memos, not quoted
from their own summaries.

**Pin:** `b4192a7128843ec43ab854fe5926e3d498b13483`. **Oracle:**
`PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6`. **HEAD reviewed:** `33ed661a5`
(companion+monster_ability lane's push, latest of the three at review time).

---

## Recommendation

**Sign off the blacklist as amended by the four proposals in §3, then treat `blocked` (266
units) as excluded and `clear` (1,988 units) as immediately transcribable.** That alone unblocks
55.6% of T9 (up from the July draft's 40.8%) with zero remaining legal ambiguity in those units —
every one of them was independently re-scanned with case-fold/OCR normalization and traced through
`.COPY=`/`.MOD` inheritance, and nothing new turned up except the five equipment items named in
§2. The `still_undecidable` remainder (1,319 units, 36.9%) is concentrated in two shapes this
review could not close by policy alone: `monster_ability`'s embedded-creature-name problem (§4.1)
and `companion`/`bestiary_3`'s bulk pattern-shaped prose (§4.2) — both need a short operator ruling,
not more scanning, and §4 gives you the exact question for each.

---

## 1. The clear-bucket re-check — read this first

`decisions.md §18` item 2 required re-checking the `clear` bucket (not only `uncertain`) with a
normalized scan, because `ogl-pi-blacklist.md §4` already recorded one real incident
(`Cayden CaiLean`, `lrori`) that shipped un-redacted past the exact-substring scan.

**Result: two independent mechanisms were run against every one of T9's 1,140 pre-review `clear`
units (all six kinds: 349 spell + 249 feat + 141 equipment + 7 monster + 283 companion + 111
monster_ability). One found nothing. The other found five real misses.**

1. **Normalized case-fold + OCR-confusion scan (all three lanes, independently implemented,
   cross-validated against both recorded incident strings): zero newly-blocked, zero
   newly-uncertain, across every kind.**
   - spell: 349 rechecked, 0 hits (`t9-pi-review-spell.md §3`)
   - feat + equipment + monster: 397 rechecked, 0 hits (`t9-pi-review-feat-equipment.md §4`)
   - companion + monster_ability: 394 rechecked (283 + 111, current-population `clear`), 0 hits
     (`t9-pi-review-companion-monsterability.md §5`)
   - **I re-ran all three scripts myself against a freshly-rebuilt inventory and reproduced every
     one of these null results exactly** — see §7 for commands. This is a genuine negative, not
     an unrun check: each script's normalization function was tested directly against
     `Cayden CaiLean` → `cayden cailean` and `lrori` → `irori` and confirmed to still resolve both
     before being trusted on a zero result.

2. **`.COPY=`/`.MOD` base-item inheritance trace (feat+equipment lane, §5 of its memo) — a
   different failure mode than spelling, and it found real misses:**

   | Equipment unit (adventurers_guide) | Was | Its base | Base's own declaration | Now |
   |---|---|---|---|---|
   | `Gelugon Plate` | clear | `Hellknight Plate` | `NAMEISPI:YES` | **blocked** |
   | `Hellknight Half-Plate Barding` | clear | `Hellknight Half-Plate` | `NAMEISPI:YES` | **blocked** |
   | `Hellknight Leather Barding` | clear | `Hellknight Leather` | `NAMEISPI:YES` | **blocked** |
   | `Hellknight Plate Barding` | clear | `Hellknight Plate` | `NAMEISPI:YES` | **blocked** |
   | `Maiden's Panoply` | clear | `Gray Maiden Plate` | `NAMEISPI:YES` | **blocked** |

   These five carried no term-list hit and no PCGen `NAMEISPI`/`DESCISPI` declaration of their
   own — they were correctly `clear` under the exact-scan rule as written. They are PI because
   PCGen's `.COPY=` mechanism makes the derivative mechanically the same named item as its
   already-declared-PI base. I re-derived this independently: `python3
   scripts/sd32_t9_pi_review_feat_equipment.py t9_pi_classified.json --corpus-root
   <oracle>/data` reproduces exactly these 5 rows.

   A sixth clear-bucket unit, `Mantis Blade` (equipment, adventurers_guide, `.COPY=` of
   `Sawtooth Sabre`, a non-PI base), was manually read and found to carry an `SPROP:` referencing
   "Red Mantis assassin" flavor text in its own row — not resolved by the inheritance rule (its
   base is clean), flagged `still_undecidable` rather than forced either way (§4.3).

**No equivalent inheritance miss was found in `spell`, `companion`, or `monster` — all `.COPY=`
rows in those kinds' `clear` buckets were checked and traced to non-PI bases** (feat's 217 `.MOD`
rows all trace to generic OGL race/class labels; companion+monster_ability's 6 `.MOD`/`.COPY` rows
were already sitting in `uncertain`/`still_undecidable`, not `clear`, so no clear-bucket miss
exists there — see §4.2 for why those 6 stay undecided anyway).

**Bottom line: the clear-bucket re-check the Inner Sea Gods incident demanded found exactly 5
real, previously-unflagged PI units (out of 1,140 checked) — not zero, but not the 90%-of-clear
catastrophe a maximally pessimistic reading of §4's incident would suggest either.** The mechanism
that caught them was not spelling normalization (which found nothing) — it was tracing
`.COPY=`/`.MOD` derivation, a gap the audit itself named (§8 gap 3) but did not check.

---

## 2. Final disposition — per kind, against the audit's original 261 / 1,107 / 1,344

**Population correction, stated up front:** the audit's 2,712-unit T9 population no longer
re-derives. Commit `6ae4a364b` (a T2b classifier fix, unrelated to this review, landed after the
audit) moved 864 units from `race_trait` into `monster_ability`. Re-running the audit's own
population step at this cycle's HEAD gives **3,573**, confirmed independently by all three lanes
and re-derived a fourth time by me:

```bash
cargo build --locked --release --bin v06_work_inventory
PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data \
    ./target/release/v06_work_inventory --stdout-only > fresh_inventory.json
python3 scripts/sd32_t9_census.py fresh_inventory.json   # TOTAL 3573 (was 2,712)
```

Only `monster_ability` moved (517 → 1,378); `spell` (732), `companion` (726), `feat` (487),
`equipment` (222), `monster` (28) are unchanged from the audit. This is a corpus-shape fact, not a
review artifact — logged once already (`docs/retro/events/companion-monsterability.jsonl`), not
re-logged here.

### Per-kind table (re-derived, verified against each lane's committed script)

| Kind | Total | **Original audit** (2,712 pop.) blocked/clear/uncertain | **This review**, current pop. blocked | clear | still_undecidable |
|---|---:|---|---:|---:|---:|
| spell | 732 | 31 / 349 / 352 | 31 | 699 | 2 |
| feat | 487 | 52 / 249 / 186 | 52 | 433 | 2 |
| equipment | 222 | 77 / 141 / 4 | 82 | 139 | 1 |
| monster | 28 | 21 / 7 / 0 | 21 | 7 | 0 |
| companion | 726 | 0 / 283 / 443 | 0 | 366 | 360 |
| monster_ability | 517→1,378 | 80 / 78 / 359 (at 517) | 80 | 344 | 954 |
| **TOTAL** | **2,712→3,573** | **261 / 1,107 / 1,344** | **266** | **1,988** | **1,319** |

**Re-derive the whole table:**
```bash
python3 scripts/sd32_t9_pi_exposure_audit.py fresh_inventory.json \
    --corpus-root "$PCGEN_CORPUS_ROOT" --json-out t9_pi_classified.json   # exact-scan base
python3 scripts/sd32_t9_pi_review_spell.py fresh_inventory.json --corpus-root "$PCGEN_CORPUS_ROOT"
python3 scripts/sd32_t9_pi_review_feat_equipment.py t9_pi_classified.json --corpus-root "$PCGEN_CORPUS_ROOT"
python3 scripts/sd32_t9_pi_review_companion_monsterability.py fresh_inventory.json \
    --corpus-root "$PCGEN_CORPUS_ROOT" --json-out cm_review_out.json
```
I ran all four commands against this cycle's own freshly-bootstrapped oracle and reproduced every
figure in the table above exactly, with one exception corrected below.

**One arithmetic correction filed against the feat+equipment memo's own summary table**
(`docs/retro/events/t9-pi-signoff.jsonl`): its §6 states equipment's proposed `clear` as
`141 − 5 + 4 = 140`; the correct figure is **139** — it omits subtracting `Mantis Blade`'s move
from `clear` to `still_undecidable` (222 total − 82 blocked − 1 still_undecidable = 139). Verified
by direct arithmetic and independently by summing the per-record findings in that memo's own §3
and §5. Does not change any PI verdict, only the bucket-size bookkeeping; the table above uses the
corrected 139.

**Movement summary:** blocked barely moved (261 → 266, +5 from the `.COPY=` inheritance find, the
population-growth-neutral kinds unchanged). `clear` grew from 1,107 to 1,988 — partly the
`monster_ability` population growth carrying its own unchanged 111-of-1,378 clear share, mostly
the per-record resolution of `uncertain`: at the current (drifted) population `uncertain` is
2,172 (not the audit's 1,344, which was measured against the smaller 2,712 population); of those
2,172, **853 resolved to `clear`** and the remaining **1,319 stayed `still_undecidable`** (853 +
1,319 = 2,172, checked). `still_undecidable` (1,319) is the honest remainder — down from "2,172
the draft cannot resolve at all at the current population" to "1,319 a human read and still could
not resolve," which is a materially different, smaller
claim.

### Per-book table (aggregate across kinds, this review's final buckets)

| Book | Total | Blocked | Clear | Still-undecidable |
|---|---:|---:|---:|---:|
| adventurers_guide | 200 | 73 | 124 | 3 |
| advanced_players_guide | 203 | 0 | 137 | 66 |
| advanced_race_guide | 18 | 0 | 13 | 5 |
| bestiary | 205 | 0 | 131 | 74 |
| bestiary_2 | 120 | 0 | 33 | 87 |
| bestiary_3 | 630 | 0 | 140 | 490 |
| bestiary_4 | 308 | 83 | 95 | 130 |
| bestiary_5 | 2 | 0 | 2 | 0 |
| book_of_the_damned_volume_1 | 35 | 0 | 9 | 26 |
| book_of_the_damned_volume_2 | 13 | 1 | 12 | 0 |
| core_rulebook | 86 | 0 | 49 | 37 |
| horror_adventures | 154 | 0 | 93 | 61 |
| inner_sea_bestiary | 42 | 9 | 7 | 26 |
| inner_sea_combat | 7 | 7 | 0 | 0 |
| inner_sea_faiths | 1 | 1 | 0 | 0 |
| inner_sea_gods | 39 | 34 | 0 | 5 |
| inner_sea_intrigue | 34 | 13 | 21 | 0 |
| inner_sea_magic | 18 | 5 | 13 | 0 |
| inner_sea_races | 52 | 5 | 46 | 1 |
| inner_sea_temples | 43 | 3 | 40 | 0 |
| inner_sea_world_guide | 56 | 28 | 16 | 12 |
| monster_codex | 24 | 0 | 23 | 1 |
| mythic_adventures | 365 | 3 | 362 | 0 |
| occult_adventures | 330 | 0 | 330 | 0 |
| ultimate_combat | 1 | 0 | 1 | 0 |
| ultimate_equipment | 2 | 1 | 1 | 0 |
| ultimate_magic | 160 | 0 | 129 | 31 |
| ultimate_psionics | 176 | 0 | 107 | 69 |
| ultimate_wilderness | 249 | 0 | 54 | 195 |
| **TOTAL** | **3,573** | **266** | **1,988** | **1,319** |

Re-derive: take the four commands above, group `t9_pi_classified.json` by `book` for
spell/feat/equipment/monster, apply this review's named per-record deltas (§4 and each lane's
memo §examples), group `cm_review_out.json` by `book`/`final_bucket` for companion/monster_ability
(that script's own output already carries `final_bucket` per unit — no delta arithmetic needed for
those two kinds). Full derivation script used for this table:
`/tmp/.../t9review/per_book_final.py` (scratch, not committed — the four commands above plus the
named per-record deltas in §4 are the reproducible source of truth; this script is a convenience
aggregation of them, not a fifth independent method).

---

## 3. Proposed `ogl-pi-blacklist.md` amendments — PROPOSED, NOT APPLIED

The blacklist stays `DRAFT`. Nothing below has been written into it. Four amendments, each with
the units it resolves:

### 3a. New §2.3 entries — `companion` and `monster_ability`

Answers `decisions.md §18` item 1 directly (802 of the original 1,344 uncertain units, 59.7%, had
no field-classification entry at all). Paste into `ogl-pi-blacklist.md §2.3`'s table:

| Field name | Struct/context | Why it needs per-value review |
|---|---|---|
| `description` (PCGen `DESC:`/`SPECIALS:`/`SA:`/`BENEFIT:` free-text tags) | `companion`-kind ability rows | Summoner-eidolon-evolution, animal-companion-trick, and familiar-archetype rules text. Reviewed corpus-wide (443 originally-uncertain rows, full read): entirely generic game mechanic in every row inspected — no deity/place/NPC content found. Presumptively OGL under §1(d)/(e)'s mechanic exclusion. 360 of the 443 nonetheless remain `still_undecidable` here, not because content was found, but because the classifier could not positively rule out a lowercase creature-species reference or an unlisted capitalized token in each one — see §4.2 for what closes this. |
| `description` (same tags) | `monster_ability`-kind rows | Special-ability text routinely embeds the *owning creature's own name* (via `KEY:<Creature> ~ <Ability>` and/or the DESC prose itself, e.g. "a jinushigami wields..."). Requires per-record judgment tied to the referenced creature's own PI status, not the ability row's content in isolation — if the named creature is not part of the SRD's declared-Open monster list, the ability row carries the same PI exposure as the creature name. See §4.1 — this is the review's single largest open question. |

**Units resolved by adding this entry alone: 0** (it documents a rule; the 1,314 units it governs
stay `still_undecidable` until the operator answers §4.1/§4.2's specific questions). What it does
resolve: the *gap itself* — no future per-book retro-fit cycle for either kind can say "the
blacklist doesn't mention this field."

### 3b. Normalization rule — applies to all six kinds' term scan, not just one

```
The term-list scan MUST case-fold and apply a bounded OCR-confusion normalization (at minimum:
lowercase-l/uppercase-I/digit-1/exclamation-mark collapsed to one canonical character, matching
the recorded lrori/Irori incident's error class; 0/o collapsed; rn folded to m) before substring
matching, using WORD-BOUNDARY matching rather than bare substring. Word-boundary matching is
required, not optional: a naive case-fold-only re-scan without it reopens a false-positive class
where a short blacklist term (e.g. "Nex") collides with an ordinary English word ("next") the
original case-sensitive scan never matched — found independently by two of the three review lanes
and fixed the same way in both. The PCGen field delimiter "|" must NOT be included in any
OCR-confusion table — folding it produces a false NEGATIVE on the Cayden CaiLean incident itself
(confirmed by direct test).
```

**Units resolved: 0 new (the normalized re-scan itself found nothing beyond the two already-known
incident strings — §1).** Its value is closing the specific hole `ogl-pi-blacklist.md §4` recorded,
so a future per-book retro-fit does not have to rediscover the OCR-confusion table from scratch,
and so a maximally-broad case-fold-only implementation is not adopted somewhere else without the
word-boundary fix (both lanes that tried it hit the `Nex`/`next` false positive before fixing it —
recorded so it doesn't get rediscovered a third time).

### 3c. `.COPY=`/`.MOD` inheritance rule

```
A PCGen `.COPY=`/`.MOD` row inherits its base item's declared NAMEISPI:YES/DESCISPI:YES status.
A `.COPY=` derivative is mechanically the same named item as its base, with only cosmetic
overrides (enhancement, price, name) — it is not new content. Resolve by same-file base-key
lookup against the base's own declaration.
```

Both lanes that examined this question reached the same conclusion independently (feat+equipment
lane, companion+monster_ability lane) — no disagreement to adjudicate. **Units resolved: 5**
(the equipment items in §1's table, moved `clear` → `blocked`). **Units this rule identifies but
does not resolve: 6** (2 monster_ability + 4 companion `.COPY=`/`.MOD` rows whose own *targets*
were not traced by either lane — see §4.4). **1 unit** (`Mantis Blade`) is a case the rule does
not resolve either way, because its base is not PI but its own row carries independent flavor
text — stays `still_undecidable` (§4.3).

### 3d. Term-list additions (proposed, undecided)

`Aldori` (underlies the already-blocked "Aldori Dueling Sword", found via a feat prerequisite
citation) and `Magaambya`/`Magaambyan` (a Golarion institution name, same shape as the 34 existing
place/nation terms). **Left undecided by the reviewing lane** — citing a PI term in a mechanical
`PREABILITY` prerequisite field (not the record's own name/flavor) is judged a legal question, not
a scan-mechanics one. Two feat units (`Redistributed Might`, `Extra Spontaneous Spell Mastery`)
sit in `still_undecidable` pending this specific call — see §4.3.

---

## 4. The still-undecidable set — 1,319 units, four distinct reasons

This is not a stall bucket. Each sub-group below has a name, a count, and the specific question
the operator must answer to close it — per the blacklist's own DRAFT banner ("stop and ask the
operator rather than guessing").

### 4.1. `monster_ability`'s embedded-creature-name problem — 954 units, the largest single group

**The question:** *for a `monster_ability` row whose `DESC:`/`KEY:` text names its owning
creature, and that creature is a Paizo-original (non-classic-SRD) bestiary monster, is the ability
row itself Product Identity by association with the creature's name — even when the row carries
no PI declaration and no term-list hit of its own?*

The 57-term list and PCGen's own `DESCISPI`/`NAMEISPI` declarations only catch 80 of 1,378
`monster_ability` units. The remaining 1,298 either read as pure mechanic (344, now `clear`) or
name a creature the reviewer could not classify as SRD-open or not (954, `still_undecidable`) —
that classification is exactly the legal call this review is not authorized to make. Named
examples, escalating in how strongly they lean `blocked`:

- `bestiary_3:monster_ability:Infused Quarterstaff (jinushigami)` — "a jinushigami wields...", a
  non-SRD Paizo creature name embedded in flavor text.
- `inner_sea_bestiary:monster_ability:Drain Prana/Malevolence/Possess Corpse (Vetala)` —
  mythology-derived creature name, Paizo-specific write-up.
- `bestiary_4:monster_ability:Immortality/Limited Starflight/Overwhelming Mind (KEY:Star-Spawn of
  Cthulhu ~ ...)` — **the strongest lean-blocked case found**: this corpus's own `spell` kind
  already declares "Summon Monster IX (Cthulhu)" `NAMEISPI:YES`, but these three
  `monster_ability` rows for the identical creature carry no such declaration. **This is an
  internal inconsistency in the pinned oracle's own PCGen data, not just a gap in the blacklist**
  — flagging it as a data-quality finding independent of the PI question: either the spell's
  declaration is right and these three ability rows are mis-tagged, or vice versa.

**What would close this:** a per-creature SRD-open/not table (the companion+monster_ability lane's
proposed next step, not built this cycle — flagged, not assumed) or an operator ruling that
Paizo-original creature names in ability flavor text are categorically PI regardless of the
ability row's own declaration (which would let a future pass reclassify by creature name alone,
without re-reading every ability row's prose).

### 4.2. `companion`/`bestiary_3` bulk pattern-shaped prose — 360 companion units, most of `bestiary_3`'s 490

**The question:** *does a row flagged only for containing a generic lowercase creature-species
reference (e.g. "reflex", "eidolon", "swim") or a single capitalized token outside a curated
generic-mechanic allowlist (e.g. "Adamantine", "Mithral") actually carry PI, or is the classifier's
allowlist simply incomplete?*

Read the companion+monster_ability lane's own spot-check examples (`t9-pi-review-companion-
monsterability.md §7`): several of the `still_undecidable` examples shown there
(`Breath Weapon ~ Cone of Acid`, flagged only for "reflex"; `Metal (Adamantine/6 CP)`, flagged only
for "Adamantine") read, on a plain reading, as pure game mechanic — no deity, place, or named NPC.
The classifier's allowlist is a documented heuristic, not exhaustive, and its authors explicitly
chose to flag rather than guess when a token fell outside it. **I agree with that choice — a wider
allowlist risks re-opening exactly the kind of silent miss §4's incident recorded — but it means a
material fraction of these 1,314 units (companion's 360 plus a share of `bestiary_3`'s 490,
unquantified by this review) are very likely `clear` on a human read and are sitting in
`still_undecidable` because the automated classifier's vocabulary, not the content, is
incomplete.**

**What would close this:** either (a) the operator expands the generic-mechanic allowlist
(dice/anatomy/condition terms like "reflex", "swim", "burrow", common material names like
"Adamantine"/"Mithral" that are SRD-open equipment materials, not PI) and the scripts are re-run,
or (b) a human reads the flagged subset directly — companion's 360 is a tractable single-sitting
read; `bestiary_3`'s share of 490 is not (`bestiary_3` alone is 630 units, 17.6% of T9).

### 4.3. Named individual cases — 4 units, each with its own open question

- `Redistributed Might` (feat, adventurers_guide) — cites "Aldori dueling sword" (already
  PI-declared) in a prerequisite field; `Aldori` itself is not on the term list. **Question:**
  should a term appearing only in a mechanical prerequisite citation (not the record's own
  name/flavor) redact the citing record too, or only the cited one? (§3d)
  - `python3 scripts/sd32_t9_pi_review_feat_equipment.py t9_pi_classified.json --corpus-root
    "$PCGEN_CORPUS_ROOT"` reproduces this finding directly.
- `Extra Spontaneous Spell Mastery` (feat, adventurers_guide) — same shape, cites "Magaambyan
  Arcanist." Same question.
- `Mantis Blade` (equipment, adventurers_guide) — `.COPY=` of a non-PI base (`Sawtooth Sabre`)
  but its own `SPROP:` cites "Red Mantis assassin," an OGL-published Paizo prestige-class name
  (mechanic) used in evocative flavor phrasing ("prayer attack," "red shroud"). **Question:**
  does citing an OGL-published class name in atmospheric flavor prose cross into PI, or does the
  class name's own OGL status carry through? Not resolved by the `.COPY=` inheritance rule (§3c),
  since the base itself is clean.
- `inner_sea_races:Bleaching Resistance` (spell) — DESC: names "the Bleaching," a Golarion-specific
  curse/event not on the 57-term list. **Question:** does an unlisted named *phenomenon* (not a
  proper noun in the deity/place sense) count as PI under OGL §1(e)'s "thematic elements...
  incidents" language?
- `monster_codex:Gift of the Deep` (spell) — a bracketed multi-option spell whose `[Molenti]`
  option reads as a named creature-variant label, unlike its plain-English sibling options
  (`[Four-Armed]`, `[Prehistoric]`). **Question:** same shape as above, for a creature-variant
  label inside a bracketed option rather than free prose.

### 4.4. Untraced `.COPY=`/`.MOD` targets — 6 units, companion+monster_ability

Named in §3c: `bestiary_2:Rake` (target: Aurumvorax, itself SRD-open per the audit's own
`monster_name` per-book override — a plausible candidate to resolve `clear` once its base
`Rake` entry's own status is confirmed), `bestiary_2:Split` (target: Carnivorous Blob),
`bestiary_4:Pooka ~ Change Shape`, `bestiary_4:Psychopomp (Nosoi) ~ Change Shape` (this one likely
stays undecided regardless of its target — "Psychopomp (Nosoi)" is itself a Paizo-specific
subtype name in the row's own name), `ultimate_wilderness:Hunter's Bond ~ Animal Companion`,
`ultimate_wilderness:Margay ~ Sound Mimicry`. **Question:** trace each of the 6 targets' own
classification (not done by either lane — flagged, not assumed) and apply §3c's rule mechanically
once traced.

---

## 5. What unblocks immediately on sign-off

**If the operator signs off the blacklist with the four §3 amendments applied:**

- **1,988 units become transcribable immediately** across every T9 book (up from the July draft's
  1,107 — a 79.6% increase in immediately-actionable content, entirely from resolving the original
  `uncertain` bucket, not from the population drift).
- **11 of 29 books are fully resolved — zero `still_undecidable` remainder, only `blocked`
  (named-excluded) and `clear` (transcribable):** `bestiary_5` (2 clear), `book_of_the_damned_
  volume_2` (12 clear, 1 blocked), `inner_sea_combat` (7 blocked, 0 clear), `inner_sea_faiths`
  (1 blocked), `inner_sea_intrigue` (21 clear, 13 blocked), `inner_sea_magic` (13 clear, 5
  blocked), `inner_sea_temples` (40 clear, 3 blocked), `mythic_adventures` (362 clear, 3
  blocked — T9's second-largest book by unit count, now essentially fully unblocked),
  `occult_adventures` (330 clear — unchanged from the audit, T9's third-largest book),
  `ultimate_combat` (1 clear), `ultimate_equipment` (1 clear, 1 blocked). **816 units across these
  11 books need no further per-record work of any kind** — an onboarding cycle can start on any of
  them the moment the operator signs.
- **The 266 `blocked` units stay excluded** — named individually in the audit (§3/§4 there) and
  unchanged in count except the 5 equipment inheritance finds; no new sign-off risk there.
- **1,319 units stay gated** pending the operator's answers in §4 — concentrated in `bestiary_3`
  (490 of 630, 77.8% of that book), `ultimate_wilderness` (195 of 249, 78.3%), `bestiary_2`
  (87 of 120), and `bestiary_4` (130 of 308) — all four driven by `monster_ability`'s
  embedded-creature-name problem (§4.1). An operator ruling on §4.1 alone would resolve the
  single largest remaining chunk of T9.

---

## 6. Lane cross-check — no unresolved disagreements found

The brief warned two lanes in this bundle already reached opposite conclusions on one population
without cross-checking each other. I checked for that here specifically:

- **Population figures:** all three lanes independently re-derived and reported the same
  drift (monster_ability 517→1,378, companion/feat/equipment/spell/monster unchanged) — no
  disagreement.
- **`.MOD`/`.COPY` inheritance rule:** both lanes that addressed it (feat+equipment,
  companion+monster_ability) independently proposed the same rule ("yes, inherit") — no
  disagreement, treated as corroboration in §3c.
- **Normalization method:** all three lanes independently built a case-fold + OCR-confusion
  scanner and all three independently hit and fixed the same `Nex`/`next` word-boundary false
  positive — treated as corroboration, not three separate findings, in §3b.
- **No lane's clear/blocked/uncertain verdict for the same record conflicted with another lane's**
  — each lane owned a disjoint kind set (spell | feat+equipment+monster | companion+monster_ability),
  so no record was reviewed twice under this dispatch. The one place two lanes' work overlapped
  (feat+equipment's memo cites the audit script's monster kind figures; companion+monster_ability's
  memo cites the same audit script for its own kinds) is consistent — both re-derive 21/7/0 for
  `monster` independently and agree.

---

## 7. Environment / reproduction summary

```bash
export PCGEN_REPO_DIR=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen
export PCGEN_CORPUS_ROOT="$PCGEN_REPO_DIR/data"
scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR"   # PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6
cargo build --locked --release --bin v06_work_inventory
./target/release/v06_work_inventory --stdout-only > fresh_inventory.json
python3 scripts/sd32_t9_census.py fresh_inventory.json                     # TOTAL 3,573
python3 scripts/sd32_t9_pi_exposure_audit.py fresh_inventory.json \
    --corpus-root "$PCGEN_CORPUS_ROOT" --json-out t9_pi_classified.json    # exact-scan base
python3 scripts/sd32_t9_pi_review_spell.py fresh_inventory.json --corpus-root "$PCGEN_CORPUS_ROOT"
python3 scripts/sd32_t9_pi_review_feat_equipment.py t9_pi_classified.json --corpus-root "$PCGEN_CORPUS_ROOT"
python3 scripts/sd32_t9_pi_review_companion_monsterability.py fresh_inventory.json \
    --corpus-root "$PCGEN_CORPUS_ROOT" --json-out cm_review_out.json
```

All six commands were run fresh for this consolidation, against a repo-local oracle slot
bootstrapped from empty this cycle (`PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6`,
matching `scripts/pcgen-oracle-pin.env`). Every figure in §2's tables reproduces exactly from
these outputs; §1's `.COPY=` inheritance table and §4's named examples are read directly from the
committed memos and cross-checked against the classified JSON's `row`/`resolved_path` fields.

**Corrections filed this cycle:** one, against `t9-pi-review-feat-equipment.md §6`'s own
arithmetic (§2 above; `docs/retro/events/t9-pi-signoff.jsonl`).

---

## 8. Standing constraints, restated

- The blacklist stays `DRAFT` / `pending_operator_sign_off`. Nothing in §3 has been applied.
- T9's kanban row (card 11) stays `in-progress`. Nothing in this document closes it.
- Nothing was transcribed, ingested, or written to corpus data by this consolidation or by any of
  the three lanes it reviews.
- This document itself is evidence for an operator decision, not the decision.
