# T9 Product-Identity exposure audit

**Actor:** `t9-pi-audit`. **Scope:** read-only, per `decisions.md §15`. Transcribes nothing,
ingests nothing, changes no corpus data, does not amend `docs/governance/ogl-pi-blacklist.md`
(status stays `DRAFT`). This memo is the audit's evidence; the operator's ruling on it is a
separate act.

**Base:** `59b04472304482949a2633cf3aeb8f4fde423d50` (Decision 15 committed), which was also
`origin/tranche/12`'s tip at the start of this cycle. Oracle:
`PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6`
(`scripts/pcgen-oracle-pin.env`, repo-local slot, self-healed via
`scripts/fetch-pcgen-oracle.sh --dest <repo-local pcgen slot>` — empty on this fresh worktree).

**Read this memo without reading any other file.** Every figure below carries the command that
reproduces it.

## 0. TL;DR for the operator

**The exposure is much larger, and much less certain, than the July draft or the 4.2% sample
suggested — and most of it is not resolvable by the blacklist as drafted.**

| Bucket | Units | % of 2,712 |
|---|---:|---:|
| **blocked** — clearly carries Product Identity | 261 | 9.6% |
| **clear** — clearly free of PI, transcribe-safe under the draft as written | 1,107 | 40.8% |
| **uncertain** — the drafted blacklist does not resolve it | **1,344** | **49.6%** |

**Half of T9 is `uncertain`, not `blocked`.** The census's 96%-in-monster-kind framing (itself an
arithmetic overstatement of its own 21/28 — see §7) does not generalize: `blocked` is only 9.6%
corpus-wide. But `clear` is not 90% either — it is 40.8%. The real story is the third bucket:
**1,344 units carry free-text content (`DESC:`/`BENEFIT:`/`SPECIALS:`/`SA:` prose) that the
drafted blacklist's 57-term scan does not hit and does not declare PI, and §2.3 of the draft
itself says a no-hit scan is not proof of cleanliness for exactly this shape of field.** This
script does not, and cannot, resolve those 1,344 by guessing — see §5.

**Two books are fully clear and can start immediately once the operator rules on the blacklist as
a whole:** `occult_adventures` (330/330 clear) and `bestiary_5` (2/2 clear). See §4.

## 1. Population re-derivation — 2,712, confirmed unchanged

```bash
cargo build --locked --release --bin v06_work_inventory
PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data \
    <target>/release/v06_work_inventory --stdout-only > fresh_inventory.json
python3 scripts/sd32_t9_census.py fresh_inventory.json   # confirms 2,712, unchanged
```

`fresh_inventory.json` carries 38,391 total units, matching `decisions.md §12c`'s inventory
denominator. T9-filtered: 2,712 (spell 732, companion 726, feat 487, monster_ability 517,
equipment 222, monster 28) — identical to `decisions.md §13` and
`card11-t9-census-census.md §1`. **No correction filed for the population** — it re-derives
clean. (One correction *was* filed against `decisions.md §15`'s own prose — see §7.)

## 2. The audit itself — full population, not a sample

```bash
python3 scripts/sd32_t9_pi_exposure_audit.py fresh_inventory.json \
    --corpus-root <repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data \
    --json-out t9_pi_classified.json
```

**Method** (full detail in the script's own module docstring, `scripts/sd32_t9_pi_exposure_audit.py`):

1. Every one of the 2,712 T9 units' `(source_file, source_line)` — a bare basename plus a
   1-indexed line, from `v06_work_inventory`'s own JSON — is resolved to a real file under the
   pinned oracle (basename index over the whole corpus tree; a book's own directory is not always
   where its file lives, confirmed by `transcribe_monster_tables.py::resolve_book_file`'s own
   documented finding for `inner_sea_gods`/`occult_adventures`). **All 2,712 resolved
   unambiguously** (`resolve_note == "ok"` for every unit — verify: `python3 -c "import json;
   d=json.load(open('t9_pi_classified.json')); from collections import Counter;
   print(Counter(r['resolve_note'] for r in d))"` → `Counter({'ok': 2712})`).
2. **The whole raw tab-separated row is read**, not one field — PI lives in `DESC:` flavour text
   as often as in a name (dispatch brief method point 7).
3. Classified into three buckets, mirroring the actual shipped screen
   (`src/rules_core/pi_screening.rs`) plus the blacklist's own §2.3 category the shipped screen
   quietly treats as an automatic pass:
   - **blocked**: `NAMEISPI:YES` or `DESCISPI:YES` declared (PCGen's own per-record token, read the
     same way `pi_screening::declared_product_identity` reads it), **or** any of the 57
     `PI_BLACKLIST_TERMS` (byte-identical copy of `pi_screening.rs`'s list — 20 deities + 34
     place/nation names + `Jarn` + the two Inner Sea Gods OCR-typo variants) appears as a substring
     anywhere in the row.
   - **uncertain**: the row carries a `DESC:`/`BENEFIT:`/`SPECIALS:`/`SA:` tag with real content,
     is not `blocked` by the rule above. This is `ogl-pi-blacklist.md §2.3`'s own "requiring
     per-record judgment, not blanket-classifiable" category, at scale.
   - **clear**: no PI declaration, no term-list hit, and no free-text tag at all — a purely
     mechanical row (`ogl-pi-blacklist.md §2.2`, blanket OGL).

**Validation against the existing 114-unit sample:** the `monster` kind reproduces exactly —
21 blocked / 7 clear / 0 uncertain, matching `card11-t9-census-census.md §5`'s already-verified
21-PI-excluded / 6-structural-non-defect / 1-genuine-gap split (the 6 `.MOD`/`.COPY` overlay rows
plus the 1 real gap land in `clear` here because a bare overlay row carries no PI declaration or
term hit of its own — a correct classification for *this audit's* question, "does this row carry
PI," which is separate from whether the row is a real T9 defect at all; the census memo already
resolved that separate question).

## 3. Results by kind (full population)

| Kind | Total | Blocked | Clear | Uncertain | Note |
|---|---:|---:|---:|---:|---|
| spell | 732 | 31 (4.2%) | 349 (47.7%) | 352 (48.1%) | |
| companion | 726 | 0 (0%) | 283 (39.0%) | 443 (61.0%) | No `companion` record declared PI or hit a term — but 61% carry `DESC:` prose the blacklist never named a rule for (companion is not one of §2.3's four named structs). |
| feat | 487 | 52 (10.7%) | 249 (51.1%) | 186 (38.2%) | |
| monster_ability | 517 | 80 (15.5%) | 78 (15.1%) | 359 (69.4%) | Largest uncertain share of any kind. |
| equipment | 222 | 77 (34.7%) | 141 (63.5%) | 4 (1.8%) | Highest blocked rate — Paizo equipment is heavily named-item-driven. |
| monster | 28 | 21 (75.0%) | 7 (25.0%) | 0 (0%) | Matches the existing sample exactly (§2). |

Re-derive: `python3 -c "import json,collections; d=json.load(open('t9_pi_classified.json'));
c=collections.defaultdict(lambda: collections.Counter()); [c[r['kind']].update([r['bucket']]) for
r in d]; print(dict(c))"`

**Example blocked records, up to 8 per kind (real corpus names, for spot-check):**

- **spell:** Aldori Alacrity (adventurers_guide, NAMEISPI:YES), Deivon's Parry
  (adventurers_guide, NAMEISPI:YES), Sarzari Shadow Memory (adventurers_guide, NAMEISPI:YES),
  Tieldlara's Feint (adventurers_guide, NAMEISPI:YES), Summon Monster IX (Cthulhu) (bestiary_4,
  NAMEISPI:YES), Blood Scent (Achaekek) (inner_sea_faiths, NAMEISPI:YES), Abadar's Truthtelling
  (inner_sea_gods, term hit "Abadar"), Gozreh's Trident (inner_sea_gods, term hit "Gozreh")
- **feat:** Agile Maiden (adventurers_guide, DESCISPI:YES), Al-Zabriti-Trained Horse
  (adventurers_guide, NAMEISPI:YES), Aldori Artistry (Disarm/Reposition/Steal/Sunder/Trip)
  (adventurers_guide, all NAMEISPI:YES), Aldori Dueling Disciple (adventurers_guide, NAMEISPI:YES)
- **equipment:** Aldori Dueling Sword, Aspis Badge of Last Resort, Eagle Knight Dress Uniform, Goz
  Mask, Gray Maiden Plate (×2 entries), Hellknight Half-Plate, Hellknight Leather (all
  adventurers_guide, all NAMEISPI:YES)
- **monster:** Dagon, Kostchtchie, Pazuzu, Cernunnos, Korada, Vildeis, Bokrug, Cthulhu (all
  bestiary_4, all NAMEISPI:YES)
- **monster_ability:** Breath Weapon, Command Aquatic Creature, Transformation, Clutch Foe,
  Crushing Blow, Favored Enemy, Powerful Slam, Vengeful Strike (all bestiary_4, all
  DESCISPI:YES)
- **companion:** none — 0 blocked in this kind corpus-wide (see table above; this is itself
  worth the operator's attention, not an omission — §5 explains why `uncertain` may be hiding
  real PI here that the term list and PCGen's own declaration both miss).

## 4. Results by book (full population)

| Book | Total | Blocked | Clear | Uncertain |
|---|---:|---:|---:|---:|
| advanced_players_guide | 203 | 0 | 123 | 80 |
| advanced_race_guide | 18 | 0 | 7 | 11 |
| adventurers_guide | 200 | 68 | 98 | 34 |
| bestiary | 196 | 0 | 9 | 187 |
| bestiary_2 | 52 | 0 | 15 | 37 |
| bestiary_3 | 5 | 0 | 1 | 4 |
| bestiary_4 | 266 | 83 | 5 | 178 |
| bestiary_5 | 2 | 0 | 2 | 0 |
| book_of_the_damned_volume_1 | 35 | 0 | 3 | 32 |
| book_of_the_damned_volume_2 | 13 | 1 | 0 | 12 |
| core_rulebook | 86 | 0 | 24 | 62 |
| horror_adventures | 154 | 0 | 19 | 135 |
| inner_sea_bestiary | 40 | 9 | 0 | 31 |
| inner_sea_combat | 7 | 7 | 0 | 0 |
| inner_sea_faiths | 1 | 1 | 0 | 0 |
| inner_sea_gods | 36 | 34 | 0 | 2 |
| inner_sea_intrigue | 34 | 13 | 0 | 21 |
| inner_sea_magic | 18 | 5 | 2 | 11 |
| inner_sea_races | 52 | 5 | 22 | 25 |
| inner_sea_temples | 43 | 3 | 40 | 0 |
| inner_sea_world_guide | 56 | 28 | 0 | 28 |
| monster_codex | 24 | 0 | 0 | 24 |
| mythic_adventures | 365 | 3 | 208 | 154 |
| occult_adventures | 330 | 0 | 330 | 0 |
| ultimate_combat | 1 | 0 | 0 | 1 |
| ultimate_equipment | 2 | 1 | 0 | 1 |
| ultimate_magic | 160 | 0 | 115 | 45 |
| ultimate_psionics | 64 | 0 | 56 | 8 |
| ultimate_wilderness | 249 | 0 | 28 | 221 |

Re-derive: same JSON, group by `book` instead of `kind`.

**Per-book, per-kind breakdown** (needed before dispatching a book-level onboarding cycle —
`card11-t9-census-census.md §6`'s finding that the fixed cost is per-book, per-kind still holds;
the operator needs to know which kind inside a book carries the risk, not just the book total):
re-derive with
`python3 -c "import json,collections; d=json.load(open('t9_pi_classified.json'));
c=collections.defaultdict(lambda: collections.defaultdict(lambda: collections.Counter()));
[c[r['book']][r['kind']].update([r['bucket']]) for r in d];
[print(b,k,dict(v)) for b in sorted(c) for k,v in c[b].items()]"` — full output retained in this
cycle's push (script + this memo); representative highlights:

- `adventurers_guide` (200 total): equipment 18 blocked/97 clear/0 uncertain; feat 46
  blocked/1 clear/34 uncertain; spell 4 blocked/0/0. **Highest-risk single book in absolute
  blocked count.**
- `bestiary_4` (266 total): monster 14 blocked/0/0; monster_ability 65 blocked/3 clear/123
  uncertain; equipment 3 blocked/0/0; spell 1 blocked/0/55 uncertain. Its own
  `monster_ability` alone carries more `uncertain` units (123) than 24 of T9's 29 books carry
  in total.
- `inner_sea_gods` (36 total): equipment 25 blocked/0/0; monster_ability 5 blocked/0/2
  uncertain; spell 4 blocked/0/0. **The single highest blocked-rate book (34/36, 94.4%)** —
  consistent with a deity-name-driven book (`PI_BLACKLIST_TERMS`' 20 deity names are exactly
  what this book's content is built from).

## 5. Why 1,344 units are `uncertain`, and why this audit does not resolve them

Forcing these into `blocked` or `clear` would defeat the point of this audit. Two honest facts
sit behind the number:

1. **The 57-term list is a documented sample, not a legal review**
   (`ogl-pi-blacklist.md`'s own DRAFT banner). `ogl-pi-blacklist.md §4`'s own "Inner Sea Gods"
   per-book override entry records a real incident of exactly this failure: two records shipped
   un-redacted because the term scan does not normalize case/OCR variants, caught only by a later
   adversarial review — not by the scan itself. A record that does not hit the 57-term list today
   is not proven free of Product Identity; it is unproven, which is a different thing.
2. **The blacklist's own §2.3 says so, for exactly this shape of field.** `description` (spell,
   equipment, feat) and `detail` (race trait) are explicitly named as "requiring per-record
   judgment, not blanket-classifiable" — the draft does not claim a no-hit scan settles them. This
   audit widens that same treatment to every T9 kind's free-text-shaped tags
   (`DESC:`/`BENEFIT:`/`SPECIALS:`/`SA:`), because the blacklist's four named structs
   (`SpellCacheData`, `EquipmentCacheData`, `FeatTableEntry`, `RaceTraitEntry`) do not cover
   `companion` or `monster_ability` at all, and those two kinds' rows plainly carry the same
   flavour-text risk (companion ability descriptions, monster special-ability descriptions).

**What would move a unit out of `uncertain`:** either (a) an operator or later cycle actually
reads the flagged row's `DESC:`/`BENEFIT:`/`SPECIALS:`/`SA:` content and rules it PI or OGL by
hand — the per-book retro-fit pattern `ogl-pi-blacklist.md §4` already establishes — or (b) the
operator expands the blacklist's term list or field-classification rules and this script is
re-run (it is designed to be: `scripts/sd32_t9_pi_exposure_audit.py` takes no book-specific
input, only the inventory JSON and the corpus root).

**This is not a stall.** `decisions.md §15` asked for the real blocked count, per kind and per
book, named — that is §3/§4 above, delivered in full. It also asked the audit not to guess. 1,344
units where the drafted blacklist does not resolve a verdict is the honest answer to "how much of
T9 is actually settled by the July draft," and it is smaller than the alternative dishonest
answer (declaring all of them `clear` because no term happened to hit).

## 6. Fully-clear books — unblock immediately

**`occult_adventures` (330/330, 100% clear) and `bestiary_5` (2/2, 100% clear)** have zero
`blocked` and zero `uncertain` units. Every other book in T9's 29 carries at least one `uncertain`
or `blocked` unit and needs the operator's ruling (or a per-record review pass) before an
onboarding cycle can safely transcribe its residual.

`occult_adventures` is also T9's second-largest book by unit count (330, second only to
`mythic_adventures`'s 365) — its 330 units are entirely `spell` (329) plus one `monster` — so this
is not a marginal finding: it is the single largest block of T9 work that can start today under
the blacklist exactly as drafted, no sign-off changes needed for this book specifically. (It still
needs the operator's general sign-off that the blacklist as a mechanism is in force — `status:
DRAFT` blocks every book equally per `decisions.md §15` — but no *content* risk in this book is
outstanding.)

Re-derive: `python3 -c "import json,collections; d=json.load(open('t9_pi_classified.json'));
c=collections.defaultdict(lambda: collections.Counter()); [c[r['book']].update([r['bucket']]) for
r in d]; print([b for b,v in c.items() if v['blocked']==0 and v['uncertain']==0])"` →
`['bestiary_5', 'occult_adventures']`.

## 7. Correction filed against `decisions.md §15`'s own prose

`decisions.md §15` states: *"the 96% rate observed in the monster kind is a sample, not a
measurement."* Its own cited source in the same paragraph is **21 of 28** units PI-blocked in the
monster kind — that is 75.0%, not 96%. `21/28*100 = 75.0`
(`python3 -c "print(21/28*100)"`). This audit's own full re-classification of the monster kind
(§3) independently reproduces 21/28 = 75.0% blocked. **Not corrected in `decisions.md` itself**
— that file is locked operator-pinned text and outside this audit's write scope — logged as
`scripts/retro.py correction` (`docs/retro/events/t9-pi-audit.jsonl`,
`RETRO_ACTOR=t9-pi-audit python3 scripts/retro.py correction --subject "decisions.md §15" ...`)
so the operator can amend the prose if desired. It does not change this audit's own findings,
which were re-derived independently rather than taken from that sentence.

## 8. Blacklist gaps — proposals only, not applied

The blacklist stays `DRAFT`; nothing below is written into it. These are what this audit's
cycle-scale sweep found that the draft as written does not cover, for the operator's
consideration:

1. **`companion` and `monster_ability` have no named field-classification entry at all.**
   `ogl-pi-blacklist.md §2.3`'s "requires per-record judgment" table names only
   `SpellCacheData`/`EquipmentCacheData`/`FeatTableEntry.description` and
   `RaceTraitEntry.detail`. This audit found 802 combined `uncertain` units across
   `companion` (443) and `monster_ability` (359) — 59.7% of the whole `uncertain` bucket — with
   no blacklist section to point the per-book retro-fit cycles at. **Proposed:** add
   `companion`-kind and `monster_ability`-kind description/detail fields to §2.3's table
   explicitly, rather than leaving them un-enumerated.
2. **The term list has no OCR/typo-normalization pass**, and `§4`'s own Inner Sea Gods entry is a
   recorded instance of this gap causing a real un-redacted shipment. **Proposed:** a documented
   normalization step (case-fold, common OCR substitution table for soft hyphens/ligatures)
   applied before the substring scan, not only ad hoc per-incident additions to the literal term
   list.
3. **No guidance for `monster`-kind rows whose block also carries an `ABILITY:` reference to a
   PI-blocked ability name** (e.g. a `.MOD` row referencing an ability defined elsewhere that is
   itself PI) — this audit classified purely by the target row's own content, per line, and did
   not trace cross-row references. **Proposed:** the operator confirm whether a `.MOD`/reference
   row inherits its target's PI status, since this audit did not check for that and it is a
   plausible additional exposure this method cannot see.

## 9. The one question the operator must answer

**Do you want to (a) sign off the blacklist as drafted now — which unblocks only the 1,107 `clear`
units (40.8% of T9) and leaves the other 59.2% (blocked + uncertain) still gated — or (b) direct a
per-record review pass on the 1,344 `uncertain` units (starting with the highest-value books named
in §4) before any further sign-off, given that half of T9 sits in a bucket the draft cannot
currently resolve either way?**

## 10. Environment / reproduction summary

```bash
export PCGEN_REPO_DIR=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen
export PCGEN_CORPUS_ROOT="$PCGEN_REPO_DIR/data"
scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR"   # PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6
cargo build --locked --release --bin v06_work_inventory
PCGEN_CORPUS_ROOT="$PCGEN_CORPUS_ROOT" ./target/release/v06_work_inventory --stdout-only > fresh_inventory.json
python3 scripts/sd32_t9_census.py fresh_inventory.json                     # 2,712, confirmed
python3 scripts/sd32_t9_pi_exposure_audit.py fresh_inventory.json \
    --corpus-root "$PCGEN_CORPUS_ROOT" --json-out t9_pi_classified.json    # this memo's §3/§4
```
