---
canonical: true
owner: sd31-wave28-race-trait-lane
purpose: Wave 28 visibility census for the race_trait pile. NO BANKING THIS WAVE — every number
  below is a classification of the standing population with a reproduction command, not a code
  change. Findings are written here for the integration cycle to fold into todo/sweeps.md,
  todo/defects.md, todo/blocked.md and todo/levers.md.
started: 2026-08-21
worktree_base: e90ba9ec1 (tranche/11 tip)
---

# VISIBILITY — race_trait, SD-31 wave 28

## 0. Reproduction

Every count below was produced by one of three commands, re-runnable from repo root
(`docs/work-inventory.json` was read-only; nothing here regenerated it):

```
# (A) doneness/evidence cross-tab, over the live work-inventory
python3 -c "
import json, sys, collections
sys.path.insert(0,'scripts/observer'); import pf1e_dashboard_producer as P
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('kind')=='race_trait']
def v(u): return P.doneness_verdict(u.get('wiring_class'),u.get('status'),u.get('kind'))
notdone=[u for u in U if v(u)!='done']
print(len(U), len(notdone))
print(collections.Counter(u.get('evidence') for u in notdone).most_common())
"

# (B) the corpus-native discriminator, over the pinned PCGen oracle
# (the SAME predicate ingest_race_traits.rs::parse_row and race_resolver::classify use --
#  scripts/classify_race_trait_rows.py's own module doc: "where they disagree, the Rust is
#  the truth and this script is the bug")
python3 scripts/classify_race_trait_rows.py <basename.lst>
```

(B) was run against all 82 distinct `source_file` basenames that appear anywhere in
`work-inventory.json`'s `race_trait` population (`python3 -c` listing
`sorted(set(u['source_file'] for u in units if u['kind']=='race_trait'))`), resolved under
`$PCGEN_CORPUS_ROOT` (pinned SHA `7f818006e371188e5717fd18d74d18a420747fc6`, confirmed present
and matching `scripts/pcgen-oracle-pin.env`), one file at a time. Per-file scripts are not
reproduced inline for space; every count quoted below names the file(s) it came from so the same
`classify_race_trait_rows.py <file>` invocation reproduces it.

## 1. Population

```
total race_trait units (all statuses): 3,504
done:                                     550
not-done:                               2,954
```

**Correction to the dispatch brief:** the dispatch stated "2,712 not-started plus 251 in-progress,
2,963 total." Re-derived today: **2,712 not-started + 241 in-progress + 1 `held` = 2,954**, a
9-unit drift from the dispatch figure (board moves between dispatch authoring and lane start; no
code in this repo changed the count — `git diff` against the read-only `work-inventory.json` is
empty). This document uses the re-derived 2,954 throughout; every group count below sums to it
exactly.

Evidence-token breakdown of the 2,954 (command A above):

| status | evidence | count |
|---|---|---:|
| not-ingested | `race_trait_race_not_modelled` | 2,472 |
| not-ingested | `race_trait_absent_from_race_traits` | 238 |
| ingested-magnitude | `race_trait_applied_by_the_race_corpus_but_no_verified_consumer` | 234 |
| ingested-magnitude | `race_trait_states_a_universal_sheet_modifier_pending_compute` | 8 |
| not-ingested | `race_trait_record_loaded_but_never_applies` | 2 |
| | **total** | **2,954** |

## 2. Why has race_trait resisted six waves? Answered directly

Read: `sweeps.md` S3, `levers.md` L4, `BESTIARY-6-LEDGER.md`'s race_trait rows, `progress.md`'s
wave 12/15/19/20/22/25/25b/26/27 race_trait entries, `OPEN-ISSUES.md` row 365.

**The honest answer is a mix of two real capability gaps that were correctly worked and one
question nobody asked.**

1. **Real capability gap #1 (worked, mostly closed):** the compound-key matcher
   `modelled_race_of_race_trait()` sweeps.md S3 names. Read directly (`src/bin/v06_work_inventory.rs`
   lines 4595-4634): as of wave 20 the per-segment test is a word-boundary-anchored **prefix**
   match, not exact equality (wave 22 added hyphen/space normalization on both sides; wave 27 added
   the `"Adopted Race"` trailing-segment special case). **S3's own worked example, `Elf Shaman Hex
   Range Choice ~ Chant`, is matched correctly by the current code** — verified directly:
   `python3 -c` confirms all 85 `Elf Shaman Hex Range Choice ~ *` units carry evidence
   `race_trait_absent_from_race_traits` (race matched, not ingested), never
   `race_trait_race_not_modelled` (race not matched). **Recommend `sweeps.md` S3 be marked CLOSED
   for the matcher mechanism** — six waves of naming it were right to keep naming it, and wave
   20/22/27 (never credited as "closing S3" in any wave receipt) actually did. What is NOT closed
   is a different, newly-found problem in the same neighborhood — see §3 Group G1 below: many of
   the records the matcher now correctly resolves to a real race are not race traits at all.

2. **Real capability gap #2 (worked, incomplete by design and honestly so):** the magnitude-consumer
   seam. `race_ids_with_a_magnitude_consumer()` (`src/rules_core/pilot_compute/mod.rs:9600`) lists
   18 races with a real per-record consumer. Waves 15/25/27 built it up in three disciplined,
   individually-verified steps (Rougarou/Gillman/Vanara → +Samsaran/Nagaji), each new race requiring
   **full per-record coverage of its own `computed` population**, a direct, stated response to
   wave 26's Undine incident (below). This is real, working, incremental progress — not stalled.

3. **The question nobody asked until this wave: is the "not modelled" population race-trait content
   at all?** Every lane from wave 19 to 27 treated `race_trait_race_not_modelled` as "a real race,
   the matcher/scope can't see it" and tried to fix the matcher or widen the seam list. **§3 Group G1
   shows at least 1,619 of the 2,954 not-done units (55%) are not player race traits in the first
   place** — monster special abilities, a Cleric/Inquisitor domain, a Summoner subsystem, and bare
   reference catalogs, all swept in by a file-level `_abilities_race.lst` → `Kind::RaceTrait` guess
   that a 4-facet partial fix (`MONSTER_ABILITY_TYPE_FACETS`) only catches when the monster-ability
   facet happens to be the FIRST dot-segment of the row's `TYPE:` — which for entire books (Bestiary
   3, Mythic Adventures) it structurally never is. No lane before this one ran the corpus-native
   discriminator (`scripts/classify_race_trait_rows.py`, which already existed, built for a
   different purpose in SD-29) against the FULL not-done population; each ran it only against a
   single candidate book before committing to ingest it, per its own stated purpose.

4. **Lane scoping was too narrow, and the Bestiary 6 ledger says so explicitly, in its own words**:
   *"a lane dispatch bounded to 'tables and matchers only, no chassis work' is close to a guaranteed
   zero-yield dispatch for `race_trait`"* (`BESTIARY-6-LEDGER.md`). Two of wave 24's four lanes
   reported zero yield for exactly this reason. This wave's own no-banking mandate sidesteps that
   trap by design — the visibility work below required reading `pilot_compute/mod.rs`,
   `v06_work_inventory.rs`, `ingest_race_traits.rs` and the raw oracle together, not one file in
   isolation.

5. **Frozen race attribution (B2) is NOT the primary blocker for this population.** Of the whole
   2,954, only a bounded, already-identified subset even reaches an attribution question — see §3
   Group G4's Advanced Player's Guide component (58 units) and §5 below. The bulk of the population
   never gets that far: it fails earlier, either because it is not race-trait content (G1) or
   because no ingest path for its race exists yet (G2/G3) or because no consumer seam exists yet
   (G6). **Lanes choosing easier work is visible but secondary**: the seam-building lever (L4) is
   real, disciplined work that DID happen; what did not happen is anyone asking whether the 83.7%-
   of-the-population bucket the seam-building can't even reach is real content.

6. **Wave 26's Undine finding is the sharpest evidence the wave-12 discipline is working, not
   failing.** `OPEN-ISSUES.md` row 365: a lane added `"undine"` to a race-level seam-membership
   list, credited 14 units, and two independent mutations proved the credit was real for only 3 of
   20 records — the other 11, including a bare `BONUS:VAR|MOVEBASE|30` override, were credited by
   race-name association alone, "the identical shape wave 12 demoted Gillman/Vanara Speed for."
   **Caught, not merged.** §3 Group G6 below is the direct re-application of that same test to
   today's population — see there for the answer.

## 3. The 2,954, classified with counts (sums to 2,954 exactly)

| Group | What it is | Count | Evidence tokens | Confidence |
|---|---|---:|---|---|
| **G1** | Misclassified non-race content — hand-verified TYPE facets, 7 whole files | **1,619** | mixed | **high — hand-verified** |
| **G2** | Real race content, hidden by an un-recognized TYPE-suffix naming convention | **159** | not_modelled | **high — hand-verified** |
| **G3** | Real, declared/plausible races entirely out of ingest scope | **284** | not_modelled | **high — corpus-native discriminator** |
| **G4** | Not_modelled remainder — in-scope races' content in unregistered book files | **518** | not_modelled | **could not fully determine** |
| **G5** | Absent-from-race-traits remainder — likely more catalog content, unconfirmed | **130** | absent_from_race_traits | **could not fully determine** |
| **G6** | Ingested, real magnitude, zero consumer seam of any kind (wave-12 test, reapplied) | **234** | ingested-magnitude | **high — exact** |
| **G7** | Universal sheet modifier pending compute (Decision 7 REFINED, size-trait shape) | **8** | ingested-magnitude | **high — exact** |
| **G8** | Loaded but never applies — needs individual investigation, 2 units | **2** | not-ingested | **high — exact, too small to generalize** |
| | **TOTAL** | **2,954** | | |

### G1 — Misclassified non-race content (1,619 units, 55% of the whole population)

**Method:** for every one of the 82 source files, ran `scripts/classify_race_trait_rows.py
<file>` (the tool's own predicate mirrors `race_resolver::classify`/`ingest_race_traits::parse_row`
by the tool's own documented purpose) and compared its "in-scope rows" count (rows whose `TYPE:`
carries a real `<Race> Racial Trait`/`<Race> Racial Default` suffix) against the number of
`race_trait`-kind units `work-inventory.json` attributes to that same file. Seven files show **zero**
race-trait-shaped rows in the pinned oracle while contributing hundreds of `race_trait` units each:

| file | book(s) | work-inventory `race_trait` units (all statuses) | rows matching `<Race> Racial Trait` shape | hand-verified content |
|---|---|---:|---:|---|
| `b3_abilities_race.lst` | bestiary_3 | 798 | 0 | Unique-monster special abilities (Animal Lord, Bandersnatch, Demilich, Fey Creature, Void Yai...), a Cleric/Inquisitor domain (`GravenGuardianDomain`), a Summoner subsystem (`UnfetteredEidolonEvolution`) |
| `b2_abilities_race.lst` | bestiary_2 | 162 | 0 | Same shape: monster special abilities (e.g. Adamantine Golem's `Indestructible`/`Immunity to Magic`, both TYPE `Defensive`/`Immunity` — outside the 4-facet monster-ability allowlist) |
| `ce_abilities_race.lst` | bestiary, bestiary_2, bestiary_3 (re-attributed per book) | 147 | 0 | Outsider/monster subtype traits ("Aeon Traits", "Agathion ~ Lay on Hands") |
| `acg_abilities_race.lst` | advanced_class_guide | 137 | 0 | Class-feature choice-suboption catalogs (Arcanist Exploit, Bloodrager, Brawler, Hunter — including S3's own worked example, `Elf Shaman Hex Range Choice`) |
| `cr_abilities_race.lst` | core_rulebook | 130 | 0 | Reference catalogs: `Racial SLA ~ <spell>` (137 corpus-wide), `Favored Enemy ~ Humanoid (<race>)`, bare `+2 <Ability>` menu entries, and the literal placeholder row `"No Race Trait Available"` |
| `ma_abilities_race.lst` | mythic_adventures | 118 | 0 | Mythic-template monster abilities (`Mythic Phoenix Trait`, `Mythic Aboleth Trait`, ...) |
| `pu_abilities_race.lst` | pathfinder_unchained | 127 | 0 | Unchained Summoner Eidolon Evolution content (`Unchained Evolution`, `Unchained Eidolon Subtype Selection`) |
| **total** | | **1,619** | **0** | |

Independently confirmed by `python3 -c` cross-tab: **0 of these 1,619 units are `done`** — none
ever could be, because nothing in the engine's own recognition logic will ever accept them as a
racial trait. Split by evidence: 1,511 report `race_trait_race_not_modelled`, 108 report
`race_trait_absent_from_race_traits` (the ACG choice-suboption rows that happen to name a real
CRB race as a leading word, e.g. `Elf Shaman Hex Range Choice ~ *`, and so pass the CRB-fallback
matcher even though they are not race content).

**Why this happened:** `file_kind()`'s whole-file `_abilities_race` → `Kind::RaceTrait` guess is a
file-level approximation, by its own doc comment (`v06_work_inventory.rs:1348-1356`).
`refine_kind()` corrects it for exactly 4 `TYPE:` first-dot-segment facets
(`MONSTER_ABILITY_TYPE_FACETS = ["NaturalAttack", "SpecialAttack", "SpecialQuality", "Universal
Monster Rule"]`), and only when that facet is the FIRST segment. PCGen's own convention frequently
puts a more specific descriptor first (`RaceAbility.SpecialQuality`, `GravenGuardianDomain.
Supernatural.RaceAbility.SpecialQuality`, `Defensive.Extraordinary`) — checking only the first
segment misses it. A deeper scan (`/tmp` scratch, not committed) found **1,253 rows corpus-wide
carry a monster-ability facet SOMEWHERE in the dot-chain but not first**, so widening the check from
"first segment" to "any segment" would reclassify a large share of this group to `MonsterAbility` —
but not correctly for the Domain/Eidolon-shaped rows also caught by that widening, which are neither
race trait nor monster ability. **This is not a one-line fix; see §4.**

### G2 — Real race content, hidden by a naming-convention gap (159 units)

`up_abilities_race.lst` (Ultimate Psionics) also shows 0 rows matching the space-separated
`<Race> Racial Trait` suffix — but hand inspection shows this file uses a DIFFERENT, real
convention: CamelCase-glued facets — `ElanRacialDefault`, `DromiteRacialDefault`,
`ForgebornRacialDefault`, `Half-GiantPsionics`, `DuergarDSPStability` — the same semantic pattern
(`<Race>` + `RacialDefault`/`RacialTrait`) with the space removed. Confirmed via:

```
grep -o "TYPE:[^	]*RacialTrait[^	]*" <path>/up_abilities_race.lst | sort -u
```

Elan, Dromite, Forgeborn, Half-Giant and Dreamscarred Press's own Duergar variant (`DuergarDSP`) are
genuine Pathfinder(-adjacent) player races with genuine racial-trait content — the tooling was
simply never taught to recognize this second, real naming convention. **0 of these 159 are `done`**
for the same reason as G1 (the tooling's own recognizer can't see them) but the fix here is real and
additive, not a reclassification.

### G3 — Real races, entirely out of ingest scope (284 units)

Corpus-native discriminator result, scoped to the 82 files: **284 rows carry a genuine `<Race>
Racial Trait`/`<Race> Racial Default` TYPE suffix for a race NOT in `ingest_race_traits.rs`'s
34-race `IN_SCOPE_RACES`.**

```
75 Skinwalker   19 Vine Leshy   17 Dhampir      15 Ghoran        12 Shabti
12 Changeling   12 Kasatha      11 Samsaran     11 Gathlain       10 Hag
10 Reptoid      10 Trox         10 Wyvaran       9 Deep One Hybrid  9 Orang-Pendak
 9 Wyrwood       8 Astomoi       8 Rougarou       7 Caligni          4 Phantom
 3 Oma           1 Android       1 Lashunta       1 Syrinx           1 Triaxian
```

**Cross-checked and partially reconciled:** Rougarou and Samsaran already have a real `race` chassis
AND a `pilot_compute` magnitude seam (waves 15/27) — their alternate-trait rows are the ONE piece
still unscoped, i.e. the roster gap is specifically in `ingest_race_traits.rs`'s `IN_SCOPE_RACES`
(alternate traits), not `ingest_races.rs`'s (chassis). **Skinwalker's exclusion from this list is
DELIBERATE, per that file's own doc comment** ("Skinwalker is not one of the races this project
models" — for the alternate-trait binary specifically; Skinwalker's own default chassis IS ingested
by `ingest_races.rs`, confirmed by a `kind=='race'` unit named "Skinwalker" existing in
`work-inventory.json`). The other ~20 races (Dhampir, Vine Leshy, Ghoran, Shabti, Changeling,
Kasatha, Gathlain, Hag, Reptoid, Trox, Wyvaran, Deep One Hybrid, Orang-Pendak, Wyrwood, Astomoi,
Caligni, Phantom, Oma, Android, Lashunta, Syrinx, Triaxian) have not been individually checked this
wave for whether their exclusion is deliberate or simply not-yet-done — **flagged as not fully
determined**, see §5.

### G4 — Not-yet-classified remainder of `race_trait_race_not_modelled` (518 units)

`2,472 total − 1,511 (G1) − 159 (G2) = 802` remaining; of those, `284` are the G3 out-of-scope-race
population (file-level cross-check confirms no overlap with G1/G2's 8 files). The final **518** sit
in files this wave did not fully triage row-by-row: `uw_abilities_race.lst` (91),
`oa_abilities_race.lst` (71), `b4_abilities_race.lst` (69), `isr_abilities_race.lst` (59, notable —
`inner_sea_races` IS a registered `BookSource`, so these 59 are either out-of-scope races within an
otherwise-covered book or a row shape the ingest binary's own filter drops), `apg_abilities_race.lst`
(58 — see §5, likely mostly the already-identified ARG-duplicate population), `arg_abilities_race.lst`
(57, notable for the same reason as isr — ARG IS registered), `b5_abilities_race_pc.lst` (55),
`pfs_iswg_abilities_race.lst` (30), `isg_abilities_races.lst` (27), `b1_abilities_race.lst` (21), and
fourteen smaller files. **Explicitly not determined this wave**: whether each of these is G1-shaped
(misclassified), G2-shaped (hidden convention), G3-shaped (real, out-of-scope race), or a genuine
in-scope-race BookSource gap. The exact same `classify_race_trait_rows.py` + per-row TYPE inspection
method used for G1/G2/G3 above generalizes directly — this is the single highest-value next
measurement, named specifically in §4's tool proposal.

### G5 — Not-yet-classified remainder of `race_trait_absent_from_race_traits` (130 units)

`238 total − 108 (G1's ACG/Horror-Adventures choice-suboption share)= 130`. Leading segments:
`half-orc`(15), `halfling`(11), `dwarf`(8), `gnome`(7), `human`(4), `elf`(9), `half-elf`(9),
`adopted race`(7), assorted `"<Race> Racial Subtype"` catalog rows (~28, same reference-catalog
shape §3 G1 already confirmed for `Race Subtype`/`Racial SLA`/`Favored Class Bonus`), `human
ethnicity`(2). **Not fully determined**: whether the ~63 bare CRB-race-named units are genuine
not-yet-ingested alternate traits (a real, cheap win — the race is already fully modelled) or
further catalog/reference entries wearing a CRB race's name coincidentally. Given `cr_abilities_
race.lst` itself carries zero real racial-trait-shaped rows (G1), these 63 must be sourced from a
DIFFERENT file (likely ARG/APG/UI compound-key rows naming a CRB race) — not yet traced to source
file this wave.

### G6 — Ingested, real magnitude, zero consumer seam (234 units) — the wave-12 test, reapplied

**Direct answer to the operator's third named thread**: *"Wave 12 demoted 262 units credited on a
load-only signal with no magnitude consumer. How many of the current population would fail that same
test?"* **Answer: 234, and all 234 are correctly held back today** — this population is exactly
`status=='ingested-magnitude'` +
`evidence=='race_trait_applied_by_the_race_corpus_but_no_verified_consumer'`, i.e. every unit the
wave-12 test would flag. It is not a hidden violation; the gate built at wave 12 is holding these
234 back exactly as designed, `in-progress`, never `done`.

**Every single one belongs to a race with ZERO `pilot_compute` seam of any kind** — cross-checked
against `race_ids_with_a_magnitude_consumer()`'s live 18-race set
(`the_union_is_exactly_the_eighteen_seamed_races`, `pilot_compute/mod.rs:9687`): none of Tiefling
(20), Drow (14), Undine (14), Ifrit (13), Oread (13), Sylph (13), Aasimar (11), Duergar (11), Tengu
(11), Catfolk (10), Fetchling (9), Merfolk (8), Ratfolk (8), Vishkanya (8), Kitsune (7), Orc (7),
Suli (7), Wayang (6), Changeling (5), Skinwalker (5), plus the 16 outsider-heritage
`<Type>-blooded`/`<Type>-spawn` variants (2 each, 32 total — Aasimar/Tiefling's planar-heritage
selection system) is among the 18 seamed races. This is the L4 lever's exact remaining target,
priced precisely: waves 15/25/27's disciplined per-record-verified expansion pattern (3 races → 5
races) generalizes to at least these ~20 more, at the same per-race cost.

### G7 — Universal sheet modifier pending compute (8 units)

`Goblin/Kobold/Svirfneblin ~ Size` (bestiary), `Grippli ~ Size` (bestiary_2), `Ratfolk ~ Size`
(bestiary_3), `Wayang ~ Size` (bestiary_4), `Gnome/Halfling ~ Size` (core_rulebook) — the exact
Decision 7 REFINED worked example. The 4-race `SIZE_ONLY_RACE_TRAIT_BUNDLE` seam
(`pilot_compute/mod.rs`) already covers Kobold/Svirfneblin/Goblin/Grippli's own `~ Size` record —
this is the OTHER four small races (Ratfolk, Wayang, Gnome, Halfling) needing the identical,
already-proven mechanism. A precise, cheap, 4-race extension of an existing seam, not new
architecture.

### G8 — Loaded but never applies (2 units)

`Human ~ Tribalistic Languages` (inner_sea_races), `Oversized Goblin` (monster_codex). Too small a
population to generalize a shape from; each needs individual investigation. Not attempted this wave
(no-banking mandate; this is diagnostic work, not measurement).

## 4. Tool evaluation

| Group closed | Tool | Cost estimate | Corpus-wide reach | Verdict |
|---|---|---|---|---|
| G1 (1,619) reclassification | A `race_trait`-row TYPE-facet triage script: for every not-done `race_trait` unit, resolve its raw `.lst` line, extract the full `TYPE:` dot-chain, and bucket by (a) matches `<Race> Racial Trait/Default` shape → real race content, (b) any segment is a monster-ability facet → `MonsterAbility`, (c) contains `Domain` → route to the domain-power family, (d) contains `Evolution`/`Eidolon` → Summoner subsystem, (e) else → named-but-unrouted, flagged for hand review. Built directly on `scripts/classify_race_trait_rows.py`'s already-proven `classify()` predicate plus a facet-widening pass. | Half a day: the predicate exists; the work is joining it against `work-inventory.json` by `(source_file, source_line)` and building 3-4 new buckets instead of 1. | **Corpus-wide, not book-scoped** — the SAME facet ambiguity (`RaceAbility`/`Extraordinary`/`Defensive` first, `SpecialQuality` second) recurs in every Bestiary and in Mythic Adventures; a single widened classifier closes the shape everywhere it appears, not just in `race_trait`. It is also the exact tool needed to finish G4/G5 (§3). **Build it** — this is the single highest-leverage item in this document. |
| G2 (159, generalizes further via G4) | One-line regex widening: add `<Race>RacialTrait`/`<Race>RacialDefault` (no space) as a second accepted suffix in both `classify_race_trait_rows.py` and (if the Rust mirrors it exactly) `ingest_race_traits.rs`'s own row classifier. | Trivial — under an hour including the fixture re-derivation this program's own conventions require. | Confirmed for Ultimate Psionics (159); untested against G4's 518 undetermined units, several of which may be Dreamscarred-Press-styled content sharing the same convention. | **Build it, then re-run G4's triage** — cheap, and directly informs how much of G4 it closes. |
| G3 (284) roster widening | Mechanical: add ~20 races to `IN_SCOPE_RACES` in both `ingest_races.rs` and `ingest_race_traits.rs`, following the exact wave-27 Samsaran/Nagaji pattern (chassis ingest, then alternate-trait ingest, then — separately, NOT bundled — a `pilot_compute` seam per race before any unit is credited `done`, per the Undine-incident discipline). | Per-race cost matches the wave-15/25/27 precedent: roughly a lane's worth of work per race for full, individually-verified coverage. Not a single afternoon for all 20; each race needs its own verification pass. | 284 units directly; each race's seam-building (a separate, later step) additionally unlocks whatever fraction of that race's population currently sits in G6-shaped "ingested-magnitude, no consumer" once ingested. | **A real, scoped, book-agnostic lever** — this is `levers.md` L4's own natural continuation, not a new lever. |
| G6 (234) seam building | `explain_<race>_flat_override_race_trait` functions, one per race, each individually verified for FULL per-record coverage before the race is added to `FLAT_OVERRIDE_RACE_TRAIT_RACES` — the exact discipline `pilot_compute/mod.rs`'s own doc comment states was adopted specifically in response to the Undine finding. | ~20 races' worth, same per-race cost as G3's seam-building step. | 234 directly, plus whatever G3/G4 ingest work feeds into races that don't have a seam yet. | **Continue L4 exactly as scoped** — no new mechanism needed, just more races through the existing, disciplined pipeline. |
| G7 (8) | Extend `SIZE_ONLY_RACE_TRAIT_BUNDLE` from 4 to 8 races (Ratfolk, Wayang, Gnome, Halfling), reusing the existing `explain_size_only_race_trait_bundle` function verbatim. | Under an hour — the mechanism already exists and is race-agnostic by construction. | 8 units, all named, all confirmed identical shape. | **Trivial, build it whenever a code-writing wave next touches race_trait.** |

**What NOT to build:** a corpus-wide "fix the file-level Kind guess" rewrite of `file_kind()` itself.
The guess is a cheap first pass BY DESIGN (`refine_kind`'s own doc comment) and correcting it
per-facet, incrementally, as this section proposes, is lower-risk than replacing the whole-file
heuristic — a wrong whole-file reclassification could silently move real race_trait content out of
the kind it belongs in, which is a worse failure mode than an inflated `not-done` count that a
careful classifier corrects.

## 5. Blocked on an operator ruling (candidate additions to `blocked.md`)

- **The Advanced Player's Guide 58-unit share of G4** is very likely, but not confirmed this wave,
  the SAME phantom-duplicate population `ingest_race_traits.rs`'s own `BOOK_SOURCES` comment already
  named and explicitly declined to ingest ("APG's true, non-duplicate, not-yet-ingested contribution
  is zero... a corpus-wide KEY scan found 49 of APG's 50 in-scope rows already ingested,
  byte-mechanically-identical... under `advanced_race_guide`"). If confirmed, this is the B2/
  Supersession-Register question in miniature: **the denominator counts APG's rows as 58 separate
  not-done units when at most 1 (`Half-Orc ~ Plagueborn`, already `done`) can ever ground** — a
  resolver-side de-dup or a Structural Exclusion Register entry, not a missing mechanism. **Ruling
  needed**: does the Supersession Register's "newest printing wins" principle (Decision 10) apply
  here even while the register itself stays PROPOSED-NOT-APPLIED, given `decisions.md §10` already
  settled the narrower cross-book-verbatim-reprint case (Ruling §19, 2026-08-21)? If yes, ~57 units
  leave the honest not-done count (not the denominator — Decision 10 is explicit that duplicates
  "do not inflate the denominator," they get marked superseded/complete, same disposition as
  Bestiary 6's 2 reprinted spells).
- **B2 (frozen race attribution / Supersession Register PROPOSED-NOT-APPLIED)** is NOT the primary
  blocker for the other groups in this census — see §2 point 5. It is directly relevant to the APG
  item above and, by the same mechanism, potentially to ARG's own 57-unit G4 share (also a
  registered `BookSource`, also showing unresolved rows) — **not traced to a specific duplicate this
  wave**, flagged for the same triage tool (§4) to check.

## 6. What this wave could not determine

- **G4 (518) and G5 (130)** — not classified to the G1/G2/G3 taxonomy. The method exists (§4's
  proposed tool) and was applied to 7 files by hand this wave; it was not run against the remaining
  ~30 files in G4/G5 for time reasons. Naming this honestly rather than guessing a split.
- **Whether the ~20 G3 races' exclusion from `IN_SCOPE_RACES` is deliberate (like Skinwalker,
  confirmed) or simply not-yet-reached** for each of Dhampir, Vine Leshy, Ghoran, Shabti, Changeling,
  Kasatha, Gathlain, Hag, Reptoid, Trox, Wyvaran, Deep One Hybrid, Orang-Pendak, Wyrwood, Astomoi,
  Caligni, Phantom, Oma, Android, Lashunta, Syrinx, Triaxian — individually. Only Skinwalker's
  exclusion is confirmed deliberate (own doc comment); the rest were not read one at a time.
- **Whether widening `refine_kind()`'s monster-ability check from "first TYPE segment" to "any
  segment"** would correctly reclassify the ~1,253-row corpus-wide surface identified in §3 G1
  without ALSO mis-sweeping the Domain-power and Eidolon-Evolution content that shares a
  `SpecialQuality`/`Extraordinary` co-facet — this needs the 3-4-way triage tool (§4), not a
  one-line widening, and was not built or tested this wave (no-banking mandate).
- **G8's 2 units** — not investigated individually; too small a population for this wave's
  corpus-wide mandate.

## 7. Summary for the integration cycle

- `sweeps.md` S3: recommend **CLOSED for the matcher mechanism** (wave 20/22/27 fixed it; verified
  against S3's own worked example). A NEW, related finding replaces it in scope — see next line.
- `sweeps.md`: recommend a **new entry** for the G1 finding (misclassified `_abilities_race.lst`
  content, 1,619+ units, `MONSTER_ABILITY_TYPE_FACETS` first-segment-only blind spot) — this is the
  real, larger, previously-unnamed sweep the compound-key story was standing in front of.
  Corpus-wide reach beyond `race_trait` itself is plausible (the same TYPE-facet-ordering blind spot
  could affect any `refine_kind()` arm) and was not checked outside `race_trait` this wave.
  **S9-shaped**: same "table and dispatch are separate, ask corpus-wide" pattern the dispatch's own
  S9 names for Ninja/Samurai.
  - `levers.md` L4: recommend status updated from "IN PROGRESS" to something reflecting the now-priced
  remainder — G6 (234 units, ~20 races) is the exact, exhaustively-enumerated remaining scope, at the
  same per-race cost the 5-race precedent already set.
- `todo/blocked.md`: recommend adding the APG-58/ARG-57 duplicate-attribution question (§5) as a new
  row.
- No `defects.md` entries proposed — nothing found this wave is a defect in previously-shipped,
  banked work; G1-G8 are all classification findings against not-done units.
