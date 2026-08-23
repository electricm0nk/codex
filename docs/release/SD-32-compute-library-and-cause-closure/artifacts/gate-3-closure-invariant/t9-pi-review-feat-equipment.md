# T9 PI per-record review — `feat` and `equipment` (decisions.md §18)

**Actor:** `feat-equipment`. **Scope:** read-only, per `decisions.md §15`/`§18`. Transcribes
nothing, ingests nothing, changes no corpus data, does not amend
`docs/governance/ogl-pi-blacklist.md` (status stays `DRAFT`). Every classification below is a
**proposal** for the operator, not a decision. T9's onboarding stays paused; row 11 stays
`in-progress`.

**Base:** worktree reset to pin `b4192a7128843ec43ab854fe5926e3d498b13483`, rebased onto
`origin/tranche/12` tip `b4192a712` (no further tranche/12 commits at rebase time). Oracle:
`PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6` (matches
`scripts/pcgen-oracle-pin.env`), bootstrapped fresh via `scripts/fetch-pcgen-oracle.sh` into the
repo-local slot (empty on this fresh worktree, confirmed by `scripts/verify.sh --only
preflight-oracle` FAIL before, PASS after).

**Script:** `scripts/sd32_t9_pi_review_feat_equipment.py`, committed alongside this memo. It
extends, and does not redo, `scripts/sd32_t9_pi_exposure_audit.py` — takes that script's own
`--json-out`, and adds the normalized re-check and `.COPY=`/`.MOD` inheritance trace this lane
was dispatched to do.

## 0. TL;DR for the operator

- **Re-derivation confirms the audit's `feat` and `equipment` figures exactly** — 487/52/249/186
  and 222/77/141/4 — unchanged since `decisions.md §15`'s audit. (The bundle's overall T9
  population grew 2,712 → 3,573 between the audit and this cycle, but that is entirely
  `monster_ability` — a Decision 16/17 classifier fix landing after the audit reclassified
  mistyped `race_trait` rows into `monster_ability`. Feat and equipment are untouched; no
  correction needed against my kinds' counts.)
- **The normalized (case-fold + OCR 1-edit) re-check of all three `clear` buckets in my lane
  (feat 249, equipment 141, monster 7 = 397 records) found zero newly-blocked and zero
  newly-uncertain.** No `Cayden CaiLean`/`lrori`-shaped miss in this lane's clear population.
- **But manual per-record reading of `equipment/clear` found a real, distinct gap the normalized
  scan cannot catch: 5 units are `.COPY=` rows of already-`NAMEISPI:YES`-declared base items**
  (`Hellknight Plate`, `Hellknight Half-Plate`, `Hellknight Leather`, `Gray Maiden Plate`) that
  the base audit script classifies purely by their own line, missing the inherited PI. **This is
  this lane's real "newly_blocked" finding** — proposed reclassification, 5 units, §3 below.
- **`feat/clear`'s entire 249 units are not feat content.** Every single one is a
  racial/class-special-ability `.MOD` template overlay row (`CATEGORY=Special Ability|<Race> ~
  <Trait>.MOD`) mistyped into the `feat` evidence family by the same `v06_work_inventory
  refine_kind` classifier defect `decisions.md §16`/`§17` already diagnosed for `race_trait`. They
  are correctly PI-clear (generic OGL race labels — Drow, Dwarf, Elf, Kitsune, Nagaji, etc., all
  SRD/Paizo-open race-type names, no named character/place content) but they are not feats.
  Logged as `scripts/retro.py correction` (event
  `docs/retro/events/feat-equipment.jsonl`); see §4.
- **`feat/uncertain`'s 186 records are almost entirely clean, generic, mechanical rules text** —
  145 `mythic_adventures` "(X) (Mythic)" upgrades and 34 `adventurers_guide` combat/social feats
  read in full, no proper nouns in any `DESC:`/`BENEFIT:` field. **Two records cite a
  Golarion-specific proper noun in a prerequisite (`PREABILITY`) field that is not in the current
  57-term list** (`Aldori`, `Magaambya`) — flagged `still_undecidable`, not forced, §2 below.
- **`equipment/uncertain`'s 4 records** (all `inner_sea_magic` "Tattoo" items) are generic
  mechanical magic items with no PI content. **Classified `clear`.**
- **`.MOD`/`.COPY` recommendation for this lane: YES, a `.COPY=`/`.MOD` row should inherit its
  base item's declared PI status.** §5.

## 1. Population re-derivation

```bash
CARGO_TARGET_DIR=/home/ubuntu/.cache/codex-targets/sd32-feat-equipment cargo build --locked --release --bin v06_work_inventory
PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data \
    /home/ubuntu/.cache/codex-targets/sd32-feat-equipment/release/v06_work_inventory --stdout-only > fresh_inventory.json
python3 scripts/sd32_t9_census.py fresh_inventory.json
```

Result: T9 total **3,573** (spell 732, companion 726, feat **487**, monster_ability 1,378,
equipment **222**, monster 28) — up from the audit's 2,712 total. **Only `monster_ability` moved**
(517 → 1,378); every other kind, including both of this lane's, is byte-identical to
`t9-pi-exposure-audit.md §1`. `monster_ability`'s jump is consistent with, and expected from,
`decisions.md §16`'s classifier fix (monster/creature special-ability rows previously mistyped as
`race_trait` correctly reclassified) landing on `tranche/12` between the audit's base commit
(`59b04472304482949a2633cf3aeb8f4fde423d50`) and this cycle's. **No correction filed against
`feat`/`equipment`'s population** — both re-derive exactly.

```bash
python3 scripts/sd32_t9_pi_exposure_audit.py fresh_inventory.json --corpus-root <same> --json-out t9_pi_classified.json
```

Re-derived bucket sizes for this lane's kinds, byte-identical to `t9-pi-exposure-audit.md §3`:

| Kind | Total | Blocked | Clear | Uncertain |
|---|---:|---:|---:|---:|
| feat | 487 | 52 | 249 | 186 |
| equipment | 222 | 77 | 141 | 4 |
| monster (recheck-only, already fully resolved) | 28 | 21 | 7 | 0 |

## 2. Per-record review — `feat/uncertain` (186 units)

Read every one of the 186 raw rows in full (`scripts/sd32_t9_pi_review_feat_equipment.py`'s
`read_row`, whole tab-separated line, not a filtered field) — 145 `mythic_adventures`, 34
`adventurers_guide`, 7 `inner_sea_magic`.

**184 of 186 — `clear`.** The `mythic_adventures` block is a uniform, mechanical "(Mythic)"
upgrade of an existing SRD-or-earlier-Paizo feat (`Toughness (Mythic)`, `Power Attack (Mythic)`,
`Weapon Focus (Mythic)`, …) — every `DESC:` is generic flavor ("Your attacks are truly
devastating.", "You have legendary resilience and durability.") and every `BENEFIT:` is pure
numeric/mechanical text. None references a deity, place, faction, or named NPC. The
`adventurers_guide` block (mounted-combat, social, and Nameless-One-linked feats) and
`inner_sea_magic` block (tattoo/shadow-metamagic feats) are the same shape — generic rules text,
no proper nouns. `Nameless One` (cited as a prerequisite in four `adventurers_guide` feats) is
itself another feat's name in the same book, not a person — the design intent is anonymity — and
is not `NAMEISPI`-declared anywhere in the corpus; treated as an OGL mechanic name like `Diva
Style`/`Diva Strike`.

**2 of 186 — `still_undecidable`.** Both cite a Golarion-specific proper noun in a `PREABILITY`
prerequisite field, not in `DESC:`/`BENEFIT:` flavor text, and neither term is in the current
57-term `PI_BLACKLIST_TERMS` list:

- **`Redistributed Might`** (`adventurers_guide`) — `PREABILITY:2,CATEGORY=FEAT,Exotic Weapon
  Proficiency (Aldori dueling sword),Iron Will`. `Aldori Dueling Sword` is itself
  `NAMEISPI:YES`-declared in this same book's equipment table
  (`ag_equip_arms_armor.lst:1`) — this feat's own text cites that declared-PI proper name
  verbatim as a prerequisite. `Aldori` is not in `PI_BLACKLIST_TERMS`.
- **`Extra Spontaneous Spell Mastery`** (`adventurers_guide`) — `PREABILITY:1,CATEGORY=Special
  Ability,Magaambyan Arcanist ~ Spontaneous Spell Mastery`. `Magaambyan` names the Magaambya, a
  specific Golarion institution (Mwangi Expanse) — a place/organization name of the same shape as
  the 34 place/nation terms already blacklisted, and not itself in the list.

Neither record's own `DESC:`/`BENEFIT:` text carries PI — the exposure, if any, is narrowly the
prerequisite citation of another record's (or another book's) proper name. This is exactly the
class of case the DRAFT banner says to stop on rather than guess: I am not the authority on
whether citing a declared-PI item's name in a mechanical prerequisite field itself constitutes PI
exposure under the OGL, and it changes the disposition of at least these 2 records plus (per §3)
widens the term list in a way that could reclassify other records not in my lane's scope. Flagged
`still_undecidable`, not forced either way.

**Re-derive:** `python3 scripts/sd32_t9_pi_review_feat_equipment.py t9_pi_classified.json
--corpus-root <same>` prints the bucket sizes; the two records above were found by reading, not by
the script (the script's `PROPOSED_TERM_ADDITIONS` list documents them for re-scanning once/if the
operator adds the terms).

## 3. Per-record review — `equipment/uncertain` (4 units)

All 4 are `inner_sea_magic` "Tattoo" wondrous items (`Caster's Tattoo (Lesser/Normal/Greater)`,
`Reservoir Tattoo`), flagged `uncertain` only because their `DESC:` field has real content per
§2.3's rule. Read in full (`ism_equip.lst:9-12`): every description is purely mechanical —
"infused with potential magical energy that aids in casting spells... Once per day... swift
action... Still Spell and Silent Spell feats..." No deity, place, faction, or named-NPC content in
any of the four. **All 4 — `clear`.**

## 4. Normalized re-check of `clear` buckets (feat 249 + equipment 141 + monster 7 = 397)

Per `decisions.md §18`'s mandate to re-check `clear`, not only `uncertain` (the `Cayden
CaiLean`/`lrori` gap `ogl-pi-blacklist.md §4` recorded). `normalized_recheck` in
`scripts/sd32_t9_pi_review_feat_equipment.py`: case-fold every one of the 57 terms against the
casefolded row (catches any-case variant), plus a single-edit OCR-substitution table (l↔I, rn↔m —
the two documented confusion classes behind the recorded incident) applied to each term and tested
against the raw row.

```
feat: rechecked=249 newly_blocked=0 newly_uncertain=0
equipment: rechecked=141 newly_blocked=0 newly_uncertain=0
monster: rechecked=7 newly_blocked=0 newly_uncertain=0
```

**Zero newly-blocked, zero newly-uncertain from the normalized scan across this lane's entire
clear population.** I additionally grepped both clear buckets for a small hand-picked set of
other well-known Golarion proper nouns not covered by either the term list or the OCR-confusion
classes (`Aldori`, `Sczarni`, `Hellknight`, `Aspis`, `Eagle Knight`, `Gray Maiden`, `Chelish`,
`Taldane`, `Varisian`, `Ustalavic`, `Vudran`, `Pathfinder Society`, `Rostland`, `Brevic`,
`Riverine`) — this is what surfaced the `.COPY=`/`.MOD` inheritance gap in §5, not the
case/OCR-normalized scan itself (that gap is a different failure mode: the base item's own PI
declaration doesn't propagate to its `.COPY`/`.MOD` derivative, independent of spelling).

**`feat/clear`'s 249 units are not feat content at all.** Every single row
(`grep -c` confirms 249/249) matches `CATEGORY=Special Ability|<Race> ~ <Trait>.MOD` —
racial/class special-ability template overlay rows, not Paizo-authored feats. This is
structurally why they landed in `clear` rather than `uncertain`: a real feat (as in §2) almost
always carries `DESC:`/`BENEFIT:` prose and lands `uncertain`; these template rows carry no
free-text tag at all, so `classify_row` correctly falls through to `clear`. Read through:
generic OGL/Paizo-open race-type labels only (`Drow`, `Duergar`, `Kitsune`, `Nagaji`, `Ratfolk`,
`Tengu`, `Vishkanya`, `Wayang`, …) — the same OGL-race-name treatment `ogl-pi-blacklist.md
§2.1`'s bestiary-name note already establishes for classic SRD monster/race names — so the PI
verdict (`clear`) is correct even though the *kind* label is wrong. Logged:

```
RETRO_ACTOR=feat-equipment python3 scripts/retro.py correction \
  --subject "t9-pi-exposure-audit.md / sd32_t9_pi_exposure_audit.py" \
  --claimed "feat kind: 249 'clear' units are feat records requiring routine PI sign-off" \
  --actual "all 249 are race/class special-ability .MOD template rows, not feat content -- the
            same v06_work_inventory refine_kind defect decisions.md §16/§17 diagnosed for
            race_trait, now also mistyping into the feat evidence family" \
  --verified-by "grep -c 'CATEGORY=Special Ability.*\.MOD' feat_clear_rows.txt == 249 == total feat/clear count"
```
(event `docs/retro/events/feat-equipment.jsonl`). **This does not change any PI verdict** — the
249 units stay `clear` — but it means the `feat` kind's real per-record backlog for future T9
onboarding is `487 - 249 = 238` units at most, not 487, and that correction belongs to a classifier
cycle (`decisions.md §16` step 3's re-measurement), not to this PI review. Out of this lane's write
scope; reported, not fixed.

## 5. `.COPY=`/`.MOD` inheritance — the gap `t9-pi-exposure-audit.md §8` gap 3 named

**Recommendation: yes, a `.COPY=`/`.MOD` row should inherit its base item's declared
`NAMEISPI`/`DESCISPI` status.** Found by manual review of `equipment/clear`, confirmed by
`scripts/sd32_t9_pi_review_feat_equipment.py`'s inheritance trace (searches every `.COPY=`/`.MOD`
row in feat+equipment clear/uncertain for a same-file base row sharing its pre-`.COPY=`/`.MOD` key
that itself declares PI):

| Unit | Book | Current bucket | Base item | Base's own declaration |
|---|---|---|---|---|
| `Hellknight Half-Plate Barding` | adventurers_guide | clear | `Hellknight Half-Plate` | `NAMEISPI:YES` (`ag_equip_arms_armor.lst:9`) |
| `Hellknight Leather Barding` | adventurers_guide | clear | `Hellknight Leather` | `NAMEISPI:YES` (`ag_equip_arms_armor.lst:10`) |
| `Hellknight Plate Barding` | adventurers_guide | clear | `Hellknight Plate` | `NAMEISPI:YES` (`ag_equip_arms_armor.lst:11`) |
| `Gelugon Plate` | adventurers_guide | clear | `Hellknight Plate` | `NAMEISPI:YES` (`ag_equip_arms_armor.lst:11`) |
| `Maiden's Panoply` | adventurers_guide | clear | `Gray Maiden Plate` | `NAMEISPI:YES` (`ag_equip_arms_armor.lst:7`) |

**Proposed reclassification: all 5 move `clear` → `blocked`.** Three of the five
(`Hellknight * Barding`) additionally carry the base's own PI-declared word in their own compiled
output name, which strengthens the call; the other two (`Gelugon Plate`, `Maiden's Panoply`) carry
no term-list hit in their own name and rest purely on base-item inheritance — I judge this the
correct call because PCGen's `.COPY=` semantics mean the derived item is mechanically the base
item (inherits `COST`/`WT`/`PROFICIENCY`/`TYPE` unless overridden) with only the enhancement and
name changed, so a base declared "this proper name and everything definitionally tied to it is
Product Identity" carries forward to a derivative that is still, mechanically, that named item.

**Scope check — is this gap wider than these 5?** I traced every `.COPY=`/`.MOD` row in
`feat/clear`, `feat/uncertain`, `equipment/clear`, and `equipment/uncertain` (script's §2 pass,
217 `.MOD` rows in `feat/clear` alone — see §4 — plus 23 `.COPY=` rows in `equipment/clear`, 0 in
either `uncertain` bucket). Every `feat/clear` `.MOD` row's base is a generic OGL race/class label
(never PI-declared) — 0 inheritance hits there. Of `equipment/clear`'s 23 `.COPY=` rows, exactly
5 trace to a PI-declared base (all `Hellknight`/`Gray Maiden` derivatives); the other 18 trace to
generic PCGen base items (`Rapier (Base)`, `Chainmail (Base)`, `Dagger (Base)`, …) that are never
PI-declared, so their creatively-named derivatives (`Banshee's Howl`, `Elven Vengeance`, `Hateful
Sting`, …) correctly stay `clear` — Paizo's own data does not mark these as PI, and per
`ogl-pi-blacklist.md §2.1`'s established pattern (PCGen's own declared tags are the authoritative
per-record signal, not a blanket "any creative name is PI" rule), absence of a base declaration is
not itself evidence of PI. **One borderline case not reclassified:** `Mantis Blade`
(`Sawtooth Sabre.COPY=Mantis Blade`) carries an `SPROP:` referencing "Red Mantis assassin" (a
named Golarion prestige-class organization) with flavor text ("prayer attack", "red shroud",
"fading abilities") — its base (`Sawtooth Sabre`) is not PI-declared, and `Red Mantis Assassin` is
itself an OGL-published Paizo prestige class name (mechanic, not proper-noun setting content), so
I lean `clear`, but flagging it `still_undecidable` rather than deciding — it is the one equipment
record in this lane's full population where I am not confident either way.

**This lane's `.MOD`/`.COPY` answer, stated for the operator to apply or reject:** *Yes, inherit.*
A `.COPY=`/`.MOD` derivative row should carry forward its base item's `NAMEISPI:YES`/`DESCISPI:YES`
declaration during any future retro-fit pass, resolved by same-file base-key lookup (the method
`scripts/sd32_t9_pi_review_feat_equipment.py::build_key_pi_index` implements). **Units affected in
this lane: 5** (all equipment, all `adventurers_guide`). I did not check whether this pattern
recurs at scale in `spell`/`companion`/`monster_ability`/`monster` — outside this lane's kind
scope; the method generalizes and the committed script is kind-agnostic if another lane wants to
run it.

## 6. Summary disposition (proposed, not applied)

| Kind | Was blocked | Was clear | Was uncertain | Proposed blocked | Proposed clear | Proposed still_undecidable |
|---|---:|---:|---:|---:|---:|---:|
| feat | 52 | 249 | 186 | 52 | 249 + 184 = 433 | 2 |
| equipment | 77 | 141 | 4 | 77 + 5 = 82 | 141 − 5 + 4 = 140 | 1 |
| monster (recheck only) | 21 | 7 | 0 | 21 | 7 | 0 |

Re-derive the base three-bucket split: `python3 scripts/sd32_t9_pi_exposure_audit.py
fresh_inventory.json --corpus-root <same> --json-out t9_pi_classified.json`. Re-derive this lane's
findings on top of it: `python3 scripts/sd32_t9_pi_review_feat_equipment.py t9_pi_classified.json
--corpus-root <same>`.

## 7. Proposed §2.3 field-classification rule (for the operator to paste, not applied)

`ogl-pi-blacklist.md §2.3`'s table names only `SpellCacheData`/`EquipmentCacheData`/
`FeatTableEntry.description`/`RaceTraitEntry.detail`. It already names `FeatTableEntry.description`
— this review's finding is that the existing rule is sound for feat/equipment content, and the gap
is elsewhere (term-list coverage and `.COPY=`/`.MOD` inheritance, not the field-classification rule
itself). Proposed additions, in the file's own template shape:

```markdown
### Per-book override: Adventurer's Guide (proposed by cycle sd32-t9-pi-review-feat-equipment, 2026-08-23)

- New terms discovered not in §2 above, both cited only in PREABILITY/prerequisite fields (not in
  DESC:/BENEFIT: flavor text) rather than in a record's own name: `Aldori` (the term underlying
  the already-PI-declared "Aldori Dueling Sword"; found in feat `Redistributed Might`'s
  prerequisite) and `Magaambya`/`Magaambyan` (a Golarion institution name, same shape as the
  existing 34 place/nation terms; found in feat `Extra Spontaneous Spell Mastery`'s prerequisite).
  NOT folded in by this review -- operator decision needed on whether a prerequisite-field
  citation of a declared-PI or place-shaped proper noun carries the same redaction obligation as
  the same term in flavor text, which this review is not authorized to decide.
- .COPY=/.MOD inheritance rule (applies corpus-wide, not book-specific): a `.COPY=`/`.MOD` PCGen
  row inherits its base item's declared `NAMEISPI:YES`/`DESCISPI:YES` status. Found 5 equipment
  units in this book (`Hellknight Half-Plate Barding`, `Hellknight Leather Barding`, `Hellknight
  Plate Barding`, `Gelugon Plate`, `Maiden's Panoply`) currently `clear` under the field-only scan
  that are `.COPY=` derivatives of `NAMEISPI:YES`-declared base armor (`Hellknight Half-Plate`,
  `Hellknight Leather`, `Hellknight Plate`, `Gray Maiden Plate`).
```

`companion` and `monster_ability` (802 of the 1,344 uncertain units, per `decisions.md §18`) are
**out of this lane's scope** — reported to the operator/orchestrator as a gap this review does not
close; a separate lane's remit per the dispatch brief.

## 8. Spot-check material (up to 10 real records, this lane's call + one-line reason)

1. `Redistributed Might` (feat, adventurers_guide) — **still_undecidable**: cites the declared-PI
   `Aldori dueling sword` name in a prerequisite field; term not in the blacklist.
2. `Extra Spontaneous Spell Mastery` (feat, adventurers_guide) — **still_undecidable**: cites
   `Magaambyan Arcanist`, a Golarion institution name not in the blacklist, in a prerequisite field.
3. `Caster's Tattoo (Greater)` (equipment, inner_sea_magic) — **clear**: generic mechanical
   wondrous item, no proper nouns anywhere in `DESC:`.
4. `Hellknight Plate Barding` (equipment, adventurers_guide) — **proposed blocked** (was clear):
   `.COPY=` of `NAMEISPI:YES` base `Hellknight Plate`; own compiled name also carries the term.
5. `Gelugon Plate` (equipment, adventurers_guide) — **proposed blocked** (was clear): `.COPY=` of
   `NAMEISPI:YES` base `Hellknight Plate`; own name carries no term hit, inheritance-only call.
6. `Maiden's Panoply` (equipment, adventurers_guide) — **proposed blocked** (was clear): `.COPY=`
   of `NAMEISPI:YES` base `Gray Maiden Plate`.
7. `Mantis Blade` (equipment, adventurers_guide) — **still_undecidable**: `.COPY=` of a non-PI
   base but its `SPROP:` cites "Red Mantis assassin" flavor content; base class name is OGL but
   the flavor text's specificity gives me pause.
8. `Toughness (Mythic)` (feat, mythic_adventures) — **clear**: representative of 145 uniform
   mechanical "(Mythic)" feat upgrades, zero proper nouns.
9. `Favored Class Bonus ~ Hit Point` (feat, adventurers_guide, `.MOD`) — **clear, but not a feat**:
   representative of all 249 `feat/clear` units — a race/class special-ability template row
   misclassified into the feat evidence family; logged as a retro correction, not a PI finding.
10. `Agile Maiden` (feat, adventurers_guide) — **blocked (already correct, unchanged)**: base
    audit's own `DESCISPI:YES` declaration; included here to confirm this lane did not touch or
    second-guess the already-`blocked` 52+77 units, only `clear`/`uncertain`.

## 9. Cross-references

- `docs/release/SD-32-compute-library-and-cause-closure/decisions.md §15, §18` — the rulings this
  review answers.
- `docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-3-closure-invariant/t9-pi-exposure-audit.md`
  — the audit this review extends.
- `docs/governance/ogl-pi-blacklist.md` — the DRAFT this review proposes additions to; unamended.
- `scripts/sd32_t9_pi_exposure_audit.py` — base classification, re-run unmodified.
- `scripts/sd32_t9_pi_review_feat_equipment.py` — this lane's committed extension script.
- `docs/retro/events/feat-equipment.jsonl` — the `feat/clear` classifier-noise correction logged
  in §4.
