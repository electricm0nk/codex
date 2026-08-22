---
canonical: true
wave: 30
lane: 5 — book sizing + blocked.md audit
purpose: >
  PART A sizes the four no-compiled-rule-set books precisely, from real landed commits and a
  direct read of docs/work-inventory.json — no build, no run, nothing banked. PART B re-derives
  every blocked.md unit count and checks whether each question is still live and answerable in
  one reply, plus two items found elsewhere that are not yet filed.
board_at_start: "13,458 / 38,372 (35.08%) — confirmed unchanged at close, see §C"
---

# VISIBILITY — books sizing + blocked.md audit (wave 30, lane 5)

## Reproduction note, read first

Every count below comes from one of three sources, named at each table: (1) a direct read of
`docs/work-inventory.json` (population is enumerated from the raw PCGen oracle regardless of
whether a book has a compiled rule set — see §A.0), (2) `git show <sha> --stat` against a real
landed commit, or (3) `find`/`ls` against the checked-out corpus. No code was built or run; no
file under `docs/`, `data/`, `src/`, `scripts/`, or `apps/` was touched. `git status --porcelain`
is empty and `docs/work-inventory.json` is byte-identical (`d64ddfc677fd1683f5b7638889a25c54`)
at both start and close of this lane's work.

---

# PART A — sizing the four no-compiled-rule-set books

## A.0 The population is already known; no ingest is needed to count it

`v06_work_inventory.rs`'s own doc comment (lines 21-29) states the enumerator walks every `.lst`
file under `PCGEN_CORPUS_ROOT`, for **all** books, including ones the engine has never compiled —
"a book the engine knows nothing about still contributes real, named units — at `not-started` —
rather than being silently skipped." This means the exact unit population for all four books is
*already sitting in the committed inventory*, with no ingest, build, or oracle checkout required
to read it:

```
python3 -c "
import json,collections
d=json.load(open('docs/work-inventory.json'))
U=d['units']
books=['adventurers_guide','inner_sea_magic','inner_sea_temples','inner_sea_taverns']
for b in books:
    Ub=[u for u in U if u.get('book')==b]
    c=collections.Counter(u.get('kind') for u in Ub)
    print(b, len(Ub), dict(c))
"
```

```
adventurers_guide  973  {class 25, class_feature 699, race 3, feat 81, spell 49, equipment 115, equipment_modifier 1}
inner_sea_magic    335  {class 3, class_feature 218, feat 7, spell 39, equipment 6, equipment_modifier 62}
inner_sea_temples   64  {spell 21, equipment 43}
inner_sea_taverns   20  {class_feature 11, feat 9}
```

**Grand total: 1,392 units, exact, across 7 kind columns**
(`class 28, class_feature 928, race 3, feat 97, spell 109, equipment 164, equipment_modifier 63`
— sums to 1,392; `class_feature`'s 928 = `699+218+11` matches `THE-BOX.md` §3's G4 figure exactly).

**Current doneness, same command with `pf1e_dashboard_producer.doneness_verdict`:**

```
not-started 1,239 | unmeasurable 105 | held 45 | deferred 1 | done 2
```

The 2 `done` are both `adventurers_guide class_feature` (`Careful Stab`, `Hairpin Trick`) — the
wave-29-corrected net from the `is_archetype_locked()` fix (`blocked.md`'s "Recently closed" B3
entry). No other unit in any of the four books is `done`. **1,390 of 1,392 (99.9%) are not-done.**

### Correction to THE-BOX §3's "≥2,300" figure

THE-BOX's ranked-lever #3 cited "≥2,300" for this cluster by adding class_feature's G4 (928) to
lane 5's corpus-wide spell/feat/equipment/equipment_modifier not-done pile (1,372) and calling the
non-overlapping sum "≥2,300." That 1,372 was never scoped to only these four books — it is lane
5's entire not-done population across all four of *those kinds*, corpus-wide, of which these four
books are only a subset. **The real, book-scoped total is 1,392, not ≥2,300.** THE-BOX's own §2.5
already flagged "book-onboarding reach within this lane's own 4 kinds is closer to 424" as a
correction in the same direction; this re-derivation supersedes both figures with the exact,
book-scoped sum. This does not shrink the lever's value — 1,392 real units behind one gate shape
is still the single largest lever after #1/#2 — it corrects the number attached to it.

## A.1 What "no compiled rule set" actually means, mechanically

`rule_set_for(book_dir)` (`v06_work_inventory.rs`) looks up `book_dir` in the exhaustive
`COMPILED_RULE_SETS` list; `inner_sea_magic`, `inner_sea_temples`, and `inner_sea_taverns` are
**absent from that list entirely** (confirmed by reading the full match arm, not inferred).
`adventurers_guide` is present (`RuleSetId::AdventurersGuide`, landed wave 29, commit `7c8602f10`).

**Registering a book's `RuleSetId` does not by itself finish any kind.** It only stops
`classify()`'s book-level gate from routing every unit of that book to the crude catch-all
`no_compiled_rule_set_for_book` (`status: not-started`), and lets each kind's own arm run — which
then still needs a real per-kind catalog (`facts.feat_keys`, `facts.spell_catalog`, a class
chassis table, etc.) to move the unit past `not-ingested`. This is directly observable in AG's own
post-registration numbers:

```
python3 -c "
import json,collections
d=json.load(open('docs/work-inventory.json'))
U=[u for u in d['units'] if u.get('book')=='adventurers_guide']
print(collections.Counter(u.get('status') for u in U))
"
# -> {'not-ingested': 820, 'unknown': 105, 'deferred-with-reason': 1, 'text-complete': 4, 'ingested-magnitude': 43}
```

Zero units are still `not-started`/gated by the book-level catch-all — every one of AG's 973 units
now carries an honest per-kind status. But 820 of them are `not-ingested`, because only the
**spell** family (49 units) got a per-kind catalog this wave; `class_feature`/`feat`/`equipment`/
`class`/`race` did not. By contrast, the three still-unregistered books show a single flat status:

```
inner_sea_magic    {'not-started': 335}
inner_sea_temples  {'not-started': 64}
inner_sea_taverns  {'not-started': 20}
```

**Two-stage cost model, confirmed against a real landed commit, not estimated:**
1. **Book registration** (once per book) — wires the `RuleSetId`, unblocks the book-level gate for
   every kind at once, moves units from the flat catch-all to honest per-kind attribution. Fixed
   cost, independent of book size.
2. **Per-kind family ingest** (once per kind the book contains) — builds the actual catalog that
   lets `classify()`'s per-kind arm find the unit. Cost scales with kind, not with book.

## A.2 The book-registration file pattern — 7 files, proven, not estimated

`adventurers_guide`'s registration (commit `7c8602f10`, "adventurers_guide book onboard — spell
family, first RuleSetId", wave 29 lane 5) is the only book in this cluster that has actually gone
through stage 1. Its diff touches exactly these production files (verified via
`git show 7c8602f10 --stat`; the two new files — the ingest binary and the new `rules_tables`
module — are content, not count-pinning, and are excluded from the "7"):

| # | File | Role |
|---|---|---|
| 1 | `apps/desktop/src-tauri/src/corpus_ingest_diagnostic.rs` | book-order/diagnostic pin |
| 2 | `apps/desktop/src-tauri/src/reach_gate.rs` | new reach claim (mutation-proven RED without it) |
| 3 | `apps/desktop/src-tauri/src/spell_catalog.rs` | desktop-side book registry entry |
| 4 | `src/bin/v06_content_state_dump.rs` | count pin |
| 5 | `src/bin/v06_work_inventory.rs` | `COMPILED_RULE_SETS` + `corpus_dir_for`/`rule_set_id` arms |
| 6 | `src/rules_core/rules_tables/mod.rs` | re-export / module wiring |
| 7 | `src/rules_core/spell_resolver.rs` | chains into `spell_catalog_rows()` as the Nth book |

This is the "~7 count-pinning files" figure from the `e13-book-ingest-cost-calibration` memory,
now traced to an exact file list from a real diff rather than a recalled estimate. **Every one of
the three unregistered books needs this same 7-file pattern once**, substituting whichever kind
family is cheapest as the anchor (AG anchored on spell; the anchor need not be spell for the other
three — see A.4).

## A.3 The per-kind family pattern — also proven, not estimated, from books already registered

Two more real commits, chosen because they add a kind family to books that already had a
`RuleSetId` (isolating the per-kind cost from the registration cost):

**Feat family** (`6a2ee54c8`, "onboard Mythic Adventures as feat catalog's 18th book",
`SD31-E6-F2-007`) — 8 production files: `src/bin/gen_feat_gap_tables.rs`,
`src/bin/v06_content_state_dump.rs`, `src/bin/v06_work_inventory.rs`,
`src/rules_core/feat_identity.rs`, `src/rules_core/feat_prereqs.rs`,
`src/rules_core/feat_prereqs/pre_tokens.rs`, `src/rules_core/rules_tables/feat_gap_tables.rs`,
`src/rules_core/rules_tables/feats_all.rs`, `src/rules_core/rules_tables/mod.rs` (9 listed; one,
`mod.rs`, overlaps A.2's list) — plus 3 test files updated for the new totals.

**Equipment family** (`7341cc3f4`, "equipment gap lane widens to 8 more books",
`SD31-E6-F10-003`) — 7 production files: `src/bin/v06_work_inventory.rs`,
`src/bin/enrich_equipment_raw_tokens.rs`, `src/bin/gen_equipment_gap_tables.rs`,
`src/rules_core/cache_gen/equipment_gap.rs`, `src/rules_core/equipment_resolver.rs`,
`src/rules_core/rules_tables/equipment_gap_tables.rs` — plus 440 new per-item JSON records (this
commit ingested real equipment content for 8 books in one pass, which is why its file count is
dominated by data files, not code files) and 1 test file.

**Class_feature family has no equivalent "add one more book" commit in this cluster to cite** —
every class_feature-bearing book onboarded so far (`occult_adventures`, `mythic_adventures`) came
in through prestige-class or archetype work that touched class chassis machinery at the same time,
so its file count cannot be cleanly isolated the way feat/equipment can. Flagged as **could not
determine to file-level precision** — see §A.5.

## A.4 Per-book plan, kind by kind, real blockers named

### `adventurers_guide` — RuleSetId DONE (wave 29). 973 total, 2 done, 971 remaining.

| Kind | Units | State | Real blocker |
|---|---:|---|---|
| spell | 49 | **DONE** (wave 29) | — |
| class_feature | 699 | not-ingested (820 combined w/ others) | The corpus JSON layer already exists (`data/corpus/adventurers_guide/class_feature/`, 178 named feature directories on disk, confirmed by `find`). Needs the per-kind family wiring (A.3-shaped, no isolated precedent) **plus** a live-fire risk already surfaced: wave 29's own book-onboard work exposed 3 archetype-locked Rage Power records inside this exact book that would have been wrongly credited `done` without the `is_archetype_locked()` fix (now landed). Any further class_feature work on this book must run through the same Ruling §18 OPEN/EXCLUSIVE gate — not a new risk, but a real one, since this book's own content already tripped it once. |
| feat | 81 | not-ingested | A.3-shaped (proven pattern, ~8-9 files) |
| equipment | 115 | not-ingested | A.3-shaped (proven pattern, ~7 files + per-item JSON) |
| equipment_modifier | 1 | not-ingested | Rides the equipment family above; too small to plan separately |
| class | 25 | not-ingested | **Expensive.** This book's 25 `class` units are archetypes/prestige classes (Aldori Swordlord, Argent Dramaturge, Asavir, Aspis Agent, Cyphermage, Death Slayer, Golden Legionnaire, Gray Corsair, Hellknight, Hellknight Signifer, Lantern Bearer, Magaambyan Arcanist, and more) layered on L0's prestige-class gating mechanism, which THE-BOX §5 already names as "does not exist anywhere in the codebase." This is L0's cost, not a book-onboarding cost — do not plan it as cheap. |
| race | 3 | not-ingested | Small; needs a real race chassis entry per race |

### `inner_sea_magic` — RuleSetId NOT REGISTERED. 335 total, 0 done.

| Kind | Units | Real blocker |
|---|---:|---|
| spell | 39 | Cheapest anchor for stage-1 registration (same shape as AG's own choice) |
| class_feature | 218 | Corpus JSON layer already exists (`data/corpus/inner_sea_magic/class_feature/`, 46 named feature directories, confirmed by `find`) — same shape as AG's remaining class_feature work |
| feat | 7 | Small, A.3-shaped |
| equipment | 6 | Small, A.3-shaped |
| equipment_modifier | 62 | Largest non-class_feature kind in this book; rides the equipment family |
| class | 3 | Needs L0/L1-shaped chassis work; small count, same expensive mechanism |

No `race`/`class` chassis-table precedent exists for this book's 3 `class` units without checking
each by name — **could not determine** which of the 3 are prestige vs. base-class shaped without a
per-name read this lane did not do (out of scope for a sizing pass; flagged for the executing wave).

### `inner_sea_temples` — RuleSetId NOT REGISTERED. **No `data/corpus/inner_sea_temples/` directory
exists at all** (`ls data/corpus | grep -i temple` returns nothing). 64 total, 0 done.

| Kind | Units | Real blocker |
|---|---:|---|
| spell | 21 | Cheapest anchor — but unlike AG/ISM, there is no existing on-disk corpus JSON for ANY kind in this book, so the raw `.lst` read for the spell family starts from zero, same as AG's did |
| equipment | 43 | A.3-shaped, but same zero-JSON starting point |

**This is the cheapest of the three unregistered books by unit count (64) but the only one with
zero pre-existing corpus JSON of any kind** — every other book in this cluster already has at
least a `class_feature` JSON tree from an earlier, unrelated ingest pass. That head start does not
exist here. Two kinds only, both already have real precedent (spell via AG, equipment via the
8-book equipment lane), so this is not a harder book to build — just one with no partial credit
banked yet.

### `inner_sea_taverns` — RuleSetId NOT REGISTERED. 20 total, 0 done. Smallest book in the cluster.

| Kind | Units | Real blocker |
|---|---:|---|
| class_feature | 11 | Corpus JSON layer already exists (`data/corpus/inner_sea_taverns/class_feature/`, 4 named directories: `mixologist`, `alchemist_archetype`, `brawler_archetype`, `bouncer`) |
| feat | 9 | Small, A.3-shaped |

**Cheapest full book to close in this cluster** — 20 units, 2 kinds, both with existing precedent,
one with existing JSON. No `spell`/`equipment`/`class`/`race` units at all means no L0-shaped
expensive work is needed here. `class_feature` is the natural stage-1 anchor for this one book
only (unlike the other two, which should anchor on spell) since it is both the cheapest kind AND
the one with pre-existing corpus JSON.

## A.5 What this lane could not determine

- **The exact per-file count for a class_feature-family "add one book" commit.** No isolated
  precedent exists in this repo's history (every past class_feature book-onboard bundled prestige
  or archetype chassis work in the same commit). The 7/8/7-file patterns for
  registration/feat/equipment are real; a class_feature-family number would need to be measured
  the first time it is actually done in isolation, not estimated from these three.
- **Which of `inner_sea_magic`'s 3 `class` units are prestige-shaped vs. base-class-shaped** — a
  per-name read against L0/L1's existing 157-unit census (`sweeps.md`'s wave-27 table) was not
  done this pass.
- **Whether any of the four books' feat/equipment content is PI-blocked** at a rate different from
  AG's own 4-of-49 (8.2%) — not spot-checked for ISM/IST/ISTav this pass; AG's own ~8% ratio is
  the only real data point in this cluster.

---

# PART B — auditing `blocked.md`

## B.0 Method

For every row currently in `blocked.md`, re-derive the unit count from `docs/work-inventory.json`
or the corpus directly (command shown), and answer three questions: is it still live (no operator
ruling has landed against it since it was filed), is it stated precisely enough to answer in one
reply, and does the filed count still reproduce.

## B.1 — `mod_only_rescue`, 249 units — **REPRODUCES EXACTLY, still live, precisely stated**

```
python3 -c "
import json,collections
d=json.load(open('docs/work-inventory.json'))
mo=[u for u in d['units'] if u.get('kind')=='feat' and u.get('origin')=='mod_only']
print(len(mo))
print(collections.Counter(u.get('book') for u in mo).most_common())
"
# -> 249  [('mythic_adventures', 208), ('inner_sea_races', 22), ('horror_adventures', 17),
#          ('adventurers_guide', 1), ('ultimate_wilderness', 1)]
```

Exact match to the filed 249 and its per-book split. The wave-28 sub-claim ("36 of the 249 now
also duplicate content Ruling §16 already deleted") also reproduces: joining each of the 249 by
`corpus_key` against every other unit in the corpus finds a same-key, different-kind duplicate for
213 of them (`race_trait` 195, `monster_ability` 16, `companion` 3, `class_feature` 2 — some units
have more than one dupe, so these don't sum to 213) and **zero surviving duplicate at all for the
other 36** — consistent with those 36 having duplicated `core_essentials` content that Ruling §16
already deleted, leaving no dupe left to find. **No operator ruling has landed on this question.
Still live, precisely stated (one yes/no: does the 249-unit `feat` phantom population get removed
from the denominator), unit count confirmed exact.**

## B.2 — Per-race branch classification, "unknown, gates race_trait" — **FILED IMPRECISELY; the
evidence already exists and the real gated population is far smaller than "gates race_trait" implies**

`artifacts/RACE-EVIDENCE-D13.md` (306 lines, already committed) is a complete, ready-to-rule
47-race evidence table built specifically to answer this question. **Reading it changes what B2
should say:**

- **46 of 47 multi-book races are branch-1 (identical base, first print owns it) with zero
  disagreement** — every single row in the table's §5 shows "0 non-citation field diffs." This is
  not an open classification question per race; it is a single structural fact, already proven
  corpus-wide (`0 of 139 RACE:.MOD citations across 47 races carry a mechanical VALUE override`).
- **The real open items are exactly two, both already isolated by the evidence table:**
  1. **A 5-race correction**: Changeling, Kitsune, Nagaji, Samsaran, Wayang are currently
     attributed to `bestiary_4`, but Advanced Race Guide's `SOURCEDATE` predates Bestiary 4's by
     16 months for all five — under `§13` branch-1 (first print owns it) they belong to
     `advanced_race_guide`, not `bestiary_4`.
  2. **One anomaly, unresolved even by the evidence table**: Skinwalker is currently attributed to
     `inner_sea_races`, but `bestiary_5` predates it by only 3 months — real, but flagged as
     needing a policy call ("does a citation-only compendium count as a printing?") the table does
     not itself answer.

**Re-derived unit count for the part that actually moves attribution** (not "race_trait" broadly):

```
python3 -c "
import json,collections
d=json.load(open('docs/work-inventory.json'))
U=d['units']
for r in ['changeling','kitsune','nagaji','samsaran','wayang']:
    us=[u for u in U if u.get('kind') in ('race','race_trait') and r in (u.get('name') or '').lower()]
    print(r, len(us), collections.Counter(u.get('book') for u in us).most_common())
sk=[u for u in U if u.get('kind') in ('race','race_trait') and 'skinwalker' in (u.get('name') or '').lower()]
print('skinwalker', len(sk), collections.Counter(u.get('book') for u in sk).most_common())
"
# -> changeling 4 (bestiary_4) | kitsune 5 (bestiary_4) | nagaji 2 (bestiary_4)
#    samsaran 5 (bestiary_4) | wayang 4 (bestiary_4) | skinwalker 4 (bestiary_5)
```

**24 units total gate on B2 as a re-attribution question** (20 across the 5-race correction + 4
for Skinwalker) — not "race_trait" as a whole. Per Decision 13/14, this is a **book-attribution
change, not a doneness change**: the denominator does not move and no unit's `done`/`not-done`
status is affected, only which book's ledger reports the credit. **The other 42 of 47 races in the
table need no attribution change at all under branch-1.**

**Recommendation for the reconciling integration cycle**: rewrite B2 to cite the evidence table
directly and ask the operator the two narrow, precisely-answerable questions above (confirm the
5-race correction; rule on the Skinwalker citation-only-compendium question), rather than leaving
it filed as an open-ended "unknown, gates race_trait" placeholder. **The question is live, but as
currently filed it is not stated precisely enough to answer in one reply — the fix is a rewrite,
not a re-ask.**

## B.3 — 48 structurally-non-PC-class units — **REPRODUCES EXACTLY, still live, precisely stated**

```
python3 -c "
import json,collections
d=json.load(open('docs/work-inventory.json'))
cls=[u for u in d['units'] if u.get('kind')=='class' and u.get('status')!='grounded']
tf=collections.Counter(u.get('type_facet') for u in cls)
print(tf.most_common())
"
```

| Group | Count | Evidence |
|---|---:|---|
| Monster (non-companion `type_facet` starting `Monster`) | 33 | exact match |
| Monster.Companion | 7 | Eidolon (APG), Eidolon (Fey) (ISM), Astral Warrior + 4 Horror variants (all Ultimate Psionics) |
| bare `Psionic` facet (power-list menus) | 3 | Gifted Blade, Gifted Blade Marksman Power List, Unlocked Talent |
| `type_facet: None` (untyped edge records) | 3 | Sorcerer/Cleric (Arcane) [bestiary], Undead Phantom [horror_adventures], Psychic Detective [occult_adventures] |
| `Support` facet (Vigilante identity records) | 2 | VCabalist, VWarlock (both `ultimate_intrigue`) |
| **Sum** | **48** | **exact match to filed B4** |

**No operator ruling has landed. Still live, precisely stated (does this 48-unit group belong under
the `class` doneness gate at all), unit count confirmed exact.**

## B.4 — 5 `Ex-*` variants — **REPRODUCES EXACTLY, still live, precisely stated**

```
python3 -c "
import json
d=json.load(open('docs/work-inventory.json'))
ex=[u for u in d['units'] if u.get('kind')=='class' and (u.get('name') or '').lower().startswith('ex-')]
for u in ex: print(u.get('name'), u.get('book'), u.get('status'))
"
# -> Ex-Warpriest (advanced_class_guide) | Ex-Antipaladin (advanced_players_guide)
#    Ex-Inquisitor (advanced_players_guide) | Ex-Barbarian (core_rulebook) | Ex-Paladin (core_rulebook)
```

Exactly 5, all `not-ingested`, none `done` (so no credit is at stake either way). **No operator
ruling has landed. Still live, precisely stated, unit count confirmed exact.**

## B.5 through B.10 — spot-checked, mixed confidence

Re-derivation attempts for the remaining six items, honestly reported:

| # | Filed count | Spot-check result | Confidence |
|---|---:|---|---|
| B6 (Eidolon evolution/SLA) | 310 | A `type_facet` substring match on `Evolution`/`Eid` inside not-done `companion` units returns **344**, not 310. This is a naive proxy, not the lane's original predicate — the gap is real but the exact figure was **not reproduced**. `companion` not-done total (825) itself reproduces exactly against THE-BOX. | LOW — flagged, not confirmed |
| B7 (Core Essentials glossary) | 233 (190+43) | A `source_file` substring check for `core_essentials` on `monster_ability`/`companion` returns 0 for both — Ruling §16's re-attribution already moved these units' `book` field off `core_essentials`, so this predicate is wrong for the post-ruling state. **Not reproduced with an available proxy this pass.** | LOW — flagged, not confirmed |
| B8 (equipment_modifier `VISIBLE:NO`) | 504 | **REPRODUCES EXACTLY**: `equipment_modifier` not-done total is 1,064 (matches THE-BOX exactly); of those, `visible == False` is **504**. | HIGH — exact match |
| B9 (`.FORGET` rows) | 2 confirmed | A `source_file` substring check for `.FORGET` returns 0 — likely the wrong field (probably lives in `corpus_key` or a raw token, not the filename). **Not reproduced.** Population is tiny (2) either way; low materiality regardless of exact re-derivation. | LOW — flagged, not confirmed, low stakes |
| B10 (`pfs_*.lst`) | 30 | A `source_file` substring check for `pfs_` on `equipment` returns **34**, not 30 — close but not exact; may include units resolved by a later wave or a slightly different predicate. | MEDIUM — close, not exact |
| B11 (ABP) | 12 | A `name` substring check for "abp" on `class_feature` returns 0 — wrong predicate (likely named "Automatic Bonus Progression" in full, not abbreviated). **Not reproduced.** | LOW — flagged, not confirmed |

**None of B6–B11 has been overtaken by an operator ruling** (checked against both
`OPERATOR-RULINGS-2026-08-19.md` and `-2026-08-21.md` — neither mentions any of these six). All
six questions are still live and, as filed, are precisely enough stated to answer in one reply
(each is a single yes/no "does this population belong under the doneness gate"). **What this pass
adds**: B8's count is now confirmed exact with a clean, cheap reproduction command; the other five
need the *original* lane's own predicate re-run to close the gap between this pass's proxy
attempts and the filed figures — flagged honestly rather than asserted.

## B.11 (new) — Two blocked items found elsewhere, not yet in `blocked.md`

**New-1: "First Boon"-shaped feat-owned content modelled as a class-owned option pool.**
THE-BOX.md §7 item 7 (wave 28) raised this and it was never carried into `blocked.md`: *"Should
feat-owned content wrongly modelled as class-owned option pools (e.g. 'First Boon' demon-lord
Obedience boons, 10 units) get a distinct classification mechanism?"* Precisely stated, one-reply
answerable, small (10 units), real — a genuine gap in the reconciliation this directory exists to
prevent (a finding named and then dropped between waves).

**New-2: `decisions.md` Decision 14's own OPEN QUESTION — branch-3 origin ownership convention.**
Decision 14 ("Provenance status: one fixed classification per (object, book)") is filed as
PROPOSED, awaiting the operator, and contains a fully-formed, precisely-stated open question that
is not tracked anywhere in `blocked.md`: *under `§13` branch 3 (same object, later book changes
the values — e.g. darkvision 60ft → 90ft), does `origin` stay with the first printing (with the
later book as `errata-source`), or does `origin` MOVE to the book holding the current values?* The
decisions.md draft recommends (a) but the operator has not confirmed it. This is not a small
question — it gates the entire Supersession Register mechanism (currently PROPOSED, NOT APPLIED)
and, transitively, every branch-3-shaped race/spell/equipment reprint in the corpus. **This belongs
in `blocked.md` as its own row; it is currently visible only inside a 250-line decisions.md entry,
which is exactly the "named but not tracked" failure this directory exists to catch.**

---

# PART C — board integrity check

```
git status --porcelain   # -> empty
md5sum docs/work-inventory.json
# -> d64ddfc677fd1683f5b7638889a25c54  docs/work-inventory.json  (unchanged, start and close)
```

No file was wired, fixed, reclassified, or regenerated. This lane's entire output is this one new
file under `artifacts/`. Board: **13,458 / 38,372 (35.08%), unchanged.**

---

# Summary for the reconciling integration cycle

**Part A**: the four-book cluster is precisely 1,392 units (not "≥2,300"), 2 already `done`
(AG's wave-29 Rogue Talent fix), 1,390 not-done. The 7-file book-registration pattern and the
~7-9-file per-kind-family pattern are both now traced to real commits, not estimates. Real
blockers: `inner_sea_temples` has zero pre-existing corpus JSON of any kind (starts from scratch);
AG's own `class` kind rides the expensive, unbuilt L0 prestige-gating mechanism; class_feature has
no isolated single-book-addition precedent to cost from.

**Part B**: B1/B4/B5 all reproduce exactly at their filed counts and remain live, unruled,
precisely-stated one-reply questions. B2 is live but imprecisely filed — the evidence table that
would let the operator answer it in one reply already exists (`RACE-EVIDENCE-D13.md`) and the real
gated population is 24 units, not "race_trait" broadly; recommend a rewrite. B8 reproduces exactly
(504). B6/B7/B9/B10/B11 could not be reproduced with the proxies available this pass — flagged
honestly, not asserted. Two new items found and not yet filed: the 10-unit "First Boon" feat/pool
question (THE-BOX §7, dropped between waves 28 and 29) and Decision 14's branch-3
origin-ownership question (never filed at all, gates the whole Supersession Register).
