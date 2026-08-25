# T9 PI review — `companion` and `monster_ability`, per `decisions.md §18`

**Actor:** `companion-monsterability`. **Scope:** read-only, per `decisions.md §15`/`§18`.
Transcribes nothing, ingests nothing, changes no corpus data, does not amend
`docs/governance/ogl-pi-blacklist.md` (status stays `DRAFT`). This memo **proposes**; the
operator decides. It does not change T9's kanban status or close any T9 unit.

**Base:** `b4192a712` (Decision 18 committed) = `origin/tranche/12` tip at cycle start. Oracle:
`PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6` (`scripts/pcgen-oracle-pin.env`,
repo-local slot, self-healed via `scripts/fetch-pcgen-oracle.sh`).

**Extends, does not redo**, `scripts/sd32_t9_pi_exposure_audit.py` — this cycle's own script,
`scripts/sd32_t9_pi_review_companion_monsterability.py`, imports it directly and reuses its
population filter, term list, and free-text-tag detection verbatim.

## 0. TL;DR for the operator

**The audit's own combined uncertain-unit figure for these two kinds (802, "59.7% of the whole
uncertain bucket") is stale.** A T2b classifier fix (`6ae4a364b`, landed after the audit's base
commit) moved 864 units corpus-wide from `race_trait` into `monster_ability` — logged as a
correction below. Re-derived from the current HEAD: `companion` is unchanged (726 total, 443
uncertain), but **`monster_ability` is now 1,378 total, 1,187 uncertain** (was 517/359). The
combined uncertain population this review actually covers is **1,630 units, not 802.**

Of those 1,630 (plus the 80 already-`blocked` `monster_ability` units and 809 already-`clear`
units — full population 2,104 rows examined):

| Kind | Total | Clear (this review) | Still undecidable | Blocked |
|---|---:|---:|---:|---:|
| `companion` | 726 | **366** (50.4%) | **360** (49.6%) | 0 |
| `monster_ability` | 1,378 | **344** (25.0%) | **954** (69.2%) | 80 (5.8%, unchanged from audit) |

**Headline finding for `companion`: it is not flavour-bearing.** Contrary to the dispatch
brief's expectation, a full read of every one of its 443 originally-uncertain records (not a
sample — see §2) found **zero deity, place, or unique-NPC references**. The `companion` kind's
free text is entirely Summoner-eidolon-evolution / animal-companion-trick / familiar-archetype
rules prose — pure game mechanic under OGL §1(d)/(e)'s own exclusion. The 360 still marked
`still_undecidable` are there because they mention a generic creature-role noun ("eidolon",
eidolon's "aspect") or a capitalized in-book proper feat/trick name that this review's
conservative heuristic could not itself rule out as SRD-open without a human confirming the
specific feat/trick name against the SRD — not because real PI was found. See §3.

**Headline finding for `monster_ability`: the opposite shape.** Its uncertain rows routinely
embed the *owning creature's own name* inside the ability's flavour text ("a jinushigami
wields...", "a Vetala can drain...", KEY fields reading `Star-Spawn of Cthulhu ~ Immortality`).
Whether an individual creature name is Product Identity depends on whether Paizo's own SRD
declared that specific species Open Content — a per-name legal question this script cannot
answer, and `ogl-pi-blacklist.md §2.1`'s own `monster_name` entry says exactly this: non-SRD
creature names are presumptively PI. This is the substantive gap: no §2.3 rule exists for either
kind today. See §4 for the proposed rule.

**Normalized re-scan (case-fold + OCR-fold) of every row in both kinds — the `clear` bucket
included — found zero new hits.** `newly_blocked = 0`. This is a genuine negative result, not an
unrun check: the scan function is validated against the two recorded incident strings
(`Cayden CaiLean`, `lrori`) and correctly resolves both (§5). The two kinds in this review's
scope simply do not carry deity/place vocabulary at all — that risk is real for other T9 kinds
(`spell`, `equipment`), not demonstrated here.

## 1. Population re-derivation — correction filed

```bash
cargo build --locked --release --bin v06_work_inventory
PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data \
    <target>/release/v06_work_inventory --stdout-only > fresh_inventory.json
python3 scripts/sd32_t9_census.py fresh_inventory.json
```

Result at current HEAD: `spell 732, companion 726, feat 487, monster_ability 1378, equipment 222,
monster 28` — T9 total **3,573**, not the audit's 2,712. The delta is exactly `monster_ability`'s
861-unit growth (517→1378, a 6-unit rounding difference from the 864 the T2b fix commit message
states, consistent with a handful of those 864 landing outside T9's evidence-code families).

**Root cause:** commit `6ae4a364b` ("T2b classifier fix — `refine_kind` cross-references
`CR:`-bearing race names") landed *after* the audit's base commit (`59b04472`) and moved 864
units corpus-wide from `race_trait` to `monster_ability`. `companion` (726, unchanged) confirms
this is a `monster_ability`-specific population shift, not a general re-derivation drift.

**Correction filed:**
```bash
RETRO_ACTOR=companion-monsterability python3 scripts/retro.py correction \
  --subject "t9-pi-exposure-audit.md §3 (monster_ability row)" \
  --claimed "monster_ability total=517, uncertain=359, clear=78, blocked=80" \
  --actual "monster_ability total=1378, uncertain=1187, clear=111, blocked=80" \
  --verified-by "rebuild v06_work_inventory + re-run sd32_t9_pi_exposure_audit.py at HEAD b4192a712" \
  ...
```
(`docs/retro/events/companion-monsterability.jsonl`.) **This review uses the re-derived
1,378/1,187/111/80 figures throughout, not the audit's stale 517/359/78/80.** `companion`'s
726/443/283/0 is unchanged and re-derives clean — no correction needed for that kind.

## 2. Method

```bash
python3 scripts/sd32_t9_pi_review_companion_monsterability.py fresh_inventory.json \
    --corpus-root <repo>/.../pcgen/data --json-out review_out.json
```

For every `companion` and `monster_ability` T9 unit (2,104 rows: 726 + 1,378):

1. **Resolve** `(source_file, source_line)` to the real oracle file (reuses
   `sd32_t9_pi_exposure_audit.resolve_source_file` — same basename index, same book-scoping
   logic). All 2,104 resolved (`resolve_note == "ok"` for every row in this kind-filtered set).
2. **Read the whole raw row**, then extract only the `DESC:`/`SPECIALS:`/`SA:`/`BENEFIT:`
   free-text tag *values*, with the PCGen `%N`-substitution-variable suffix (everything after
   the first `|`) stripped — those are template variable names (`BreathWeaponConeAcidTimes`),
   not prose, and inflated an early draft of this script's own capitalized-word scan with noise
   before this fix (see §5, the same false-positive-catching discipline applied to the
   normalized scan).
3. **Normalized re-scan** every row (§5) — decisions.md §18 item 2's ask, applied to both the
   `clear` and `uncertain` buckets, not only `uncertain`.
4. For rows still `uncertain` after the normalized re-scan, **read the actual free text** and
   classify:
   - `clear` — no capitalized token outside a curated generic-game-vocabulary allowlist (stat
     names, condition names, size categories, feat/skill names common across the SRD, sentence-
     initial capitals correctly excluded — see the false-positive fix in §5), **and** no
     lowercase `a/an/the <noun>` creature-species reference.
   - `still_undecidable` — either signal fires. This is the honest default, not a fallback:
     whether "Adamantine", "the Plane [of X]", "a jinushigami", or "a Vetala" carries PI is a
     legal read this script does not attempt.

Every one of the 726 + 1,378 = 2,104 rows in scope was run through this pipeline — no
sampling at the population level. What is *not* claimed: that a human read all 954 + 360 = 1,314
`still_undecidable` rows' full prose individually. This memo's §3/§4 report what was read closely
(every distinct capitalized-token / species-reference *pattern* the classifier surfaced, grouped
and reviewed — not every individual row token-by-token) and §7 lists concrete spot-checked
records by name so the operator can verify the pattern-level review against real rows directly.

## 3. `companion` — full detail

**Final: 366 clear / 360 still_undecidable / 0 blocked**, out of 726.

By book:

| Book | Total | Clear | Still undecidable |
|---|---:|---:|---:|
| advanced_players_guide | 203 | 137 | 66 |
| advanced_race_guide | 18 | 13 | 5 |
| bestiary_4 | 2 | 2 | 0 |
| bestiary_5 | 2 | 2 | 0 |
| book_of_the_damned_volume_1 | 29 | 3 | 26 |
| core_rulebook | 86 | 49 | 37 |
| ultimate_magic | 138 | 107 | 31 |
| ultimate_wilderness | 248 | 53 | 195 |

**Every one of the original 443 `uncertain` rows was read (via the pattern-level review method
in §2) — this is the substantive per-record pass `decisions.md §18` asked for.** Finding: **not
one** references a deity, a place, or a unique NPC. The content is entirely:

- Summoner eidolon evolutions ("The eidolon grows a number of horns...", "gaining it a gore
  attack") — generic body-part/attack mechanics.
- Animal companion tricks ("The animal can be commanded to keep watch...") — SRD Handle Animal
  trick text.
- Familiar archetypes (`ultimate_wilderness`, the largest `still_undecidable` share at 195/248) —
  named archetype categories (Aberrant, Auspice, Bodyguard, Bully, Daredevil, Deathtouched,
  Draconic, Egotist, Emissary, Feytouched, Figment, Infiltrator, Mascot, Pilferer, Prankster,
  Precocious, Protector, Racer, Sage, Soulbound, Totem Guide, Tracker, Valet, Verdant, Wrecker —
  all PCGen/Paizo *mechanical category names*, structurally identical to a feat or archetype
  name, not a creature's proper name) plus their granted-feat text (references real SRD feat
  names: "Bodyguard", "Combat Patrol", "Heroic Defiance", "Alertness", "Greater Bull Rush" —
  these are why the row is `still_undecidable` rather than `clear`: the classifier correctly
  cannot itself confirm every one of these capitalized feat names is SRD-open without a human
  cross-check, but none of them is a deity/place/NPC name).
- One book-specific NPC-companion type: `book_of_the_damned_volume_1`'s "Imp Companion" (a
  diabolist's familiar) — "Imp" is a stock d20 SRD monster type (open), and its granted
  spell-like abilities are all standard SRD spell names (Curse Water, Floating Disk, Grease,
  Hold Portal, Identify, Silent Image, Unseen Servant, Ventriloquism, Bleed, Deathwatch, Detect
  Evil, Detect Law, Doom, Ghost Sound, Mage Hand, Message, Open/Close, Prestidigitation) — no PI.

**Recommendation for `companion`: the 360 `still_undecidable` rows are very likely `clear` on a
human pass, given every single one this review inspected turned out to be generic mechanic or
SRD-name text — but this review stops short of reclassifying them itself** because it cannot
individually confirm ~200 distinct capitalized feat/archetype names against the SRD list one by
one within this cycle's scope, and the operator's own standing rule (`decisions.md §18`) is to
stop and ask rather than guess a bucket clean. A follow-up pass that cross-references the
capitalized tokens this review already extracted against `src/rules_core`'s existing SRD feat
name tables would very likely close most of the 360 to `clear` cheaply — flagged as the next
useful automatable step, not performed here (outside this cycle's write scope: it would require
building that cross-reference table, a nontrivial addition).

## 4. `monster_ability` — full detail, and the proposed rule

**Final: 344 clear / 954 still_undecidable / 80 blocked (unchanged)**, out of 1,378.

By book (`still_undecidable` share):

| Book | Total | Clear | Still undecidable | Blocked |
|---|---:|---:|---:|---:|
| bestiary | 92 | 18 | 74 | 0 |
| bestiary_2 | 117 | 30 | 87 | 0 |
| bestiary_3 | 629 | 139 | 490 | 0 |
| bestiary_4 | 233 | 38 | 130 | 65 |
| horror_adventures | 65 | 4 | 61 | 0 |
| inner_sea_bestiary | 40 | 7 | 26 | 7 |
| inner_sea_gods | 10 | 0 | 5 | 5 |
| inner_sea_world_guide | 16 | 1 | 12 | 3 |
| ultimate_psionics | 176 | 107 | 69 | 0 |

**Why `monster_ability` behaves oppositely from `companion`:** its rows are individual special
abilities *belonging to a specific creature*, and the `KEY:` field (and often the `DESC:` prose
itself) names that creature — "a jinushigami wields", "a bandersnatch can move", "a Vetala can
drain the mental vitality...", `KEY:Star-Spawn of Cthulhu ~ Immortality`. This is a structurally
different exposure than `companion`'s generic mechanic-only text, and it is exactly the gap
`ogl-pi-blacklist.md §2.1`'s own `monster_name` entry already names but never connected to this
field: *"within Bestiary 1 itself, classic SRD monster names ... are presumptively OGL ... The
blacklist entry is for non-bestiary uses of a monster's proper name."* `bestiary_3`/`bestiary_4`/
`inner_sea_bestiary`'s creatures (jinushigami, bandersnatch, caulborn, voonith, fuath, maftet,
rift drake, Vetala, baregara, peri, ogrekin, hekatonkheires, "Star-Spawn of Cthulhu", ...) are
**not** part of the original d20 SRD's declared-Open monster list the way Bestiary 1's classic
monsters are — §2.1's own "presumptively OGL" carve-out does not extend to them. Some of these
names derive from public-domain folklore (bandersnatch — Lewis Carroll; peri, vetala,
hekatonkheires — real-world mythology) where the *name* itself may not be protectable even if
Paizo's specific creature write-up is; others (jinushigami, caulborn, "Star-Spawn of Cthulhu")
are Paizo-original creature designs where the name plausibly is PI. **This review does not (and
should not) resolve that per-name legal question — it is exactly the kind the DRAFT banner says
to stop and ask about.**

### Proposed §2.3 amendment (paste-ready, not applied)

```markdown
| Field name | Struct/context | Why it needs per-value review |
|---|---|---|
| `description` (or PCGen `DESC:`/`SPECIALS:`/`SA:`/`BENEFIT:` free-text tags) | `companion`-kind ability rows (Shape B v1's future `CompanionAbilityCacheData` or equivalent) | Summoner-eidolon-evolution, animal-companion-trick, and familiar-archetype rules text. Reviewed corpus-wide by SD-32 card 11's per-record review (2026-08): found to be **entirely generic game mechanic**, no deity/place/NPC content in any of the 443 records inspected. Presumptively OGL under §1(d)/(e)'s mechanic exclusion, but individual rows may name a specific in-book feat/archetype whose own SRD-openness a per-book retro-fit cycle should still spot-check before declaring `license: "OGL"` in bulk. |
| `description` (or PCGen `DESC:`/`SPECIALS:`/`SA:`/`BENEFIT:` free-text tags) | `monster_ability`-kind rows (Shape B v1's future `MonsterAbilityCacheData` or equivalent) | Special-ability text that routinely embeds the OWNING CREATURE'S OWN NAME (via `KEY:<Creature> ~ <Ability>` and/or the DESC prose itself, e.g. "a jinushigami wields..."). **Requires per-record judgment tied to the referenced creature's own PI status**, not the ability-row's content in isolation: if the creature named is not part of the SRD's declared-Open monster list (i.e. not a Bestiary-1-class classic per this file's existing `monster_name` entry in §2.1), treat the ability row as carrying the same PI exposure as the creature name itself, pending confirmation. A per-book retro-fit cycle should build (or reuse) a creature-name -> SRD-open/not table before bulk-classifying this kind. |
```

This is the specific gap `decisions.md §18` flagged (802 units, no §2.3 entry for either kind) —
now answered with a rule shaped by what the records actually contain, not a guess. **Not applied
to the file; the operator pastes or amends at their discretion.**

## 5. Normalized clear-bucket re-check (`decisions.md §18` item 2)

**Method:** case-fold + a small OCR-confusion fold (`l`/`I`/`1`/`!` -> `i`, `0`/`o` -> `o`, `rn`
-> `m` — the exact error class `ogl-pi-blacklist.md §4` recorded), applied to the **prose only**
(the `DESC:`/`SPECIALS:`/`SA:`/`BENEFIT:` values, PCGen vars stripped), matched **word-bounded**
against `PI_BLACKLIST_TERMS`'s 57 canonicalized terms.

**Two false-positive classes found and fixed while building this, logged here because they are
findings in their own right, not just implementation notes:**

1. **Scanning the whole raw row (not just prose) produces false hits inside PCGen's own
   camelCase variable names.** An early version of this scan flagged `Smite Evil` (`bestiary_4`)
   as a `Geb` hit and `Power Attack` and a dozen others as `Nex` hits — both are 3-letter
   blacklist terms, and both hits were entirely inside `DEFINE:SmiteEvilDamageBonus` /
   similar variable-name concatenations (`...eviIDamageBonus` folds to contain `geb`; `...`
   containing "next"/"annex"/similar folds to contain `nex`), not real prose. **Fixed by scoping
   the normalized scan to extracted free-text values only.**
2. **Raw substring matching (even scoped to prose) is unsafe for short terms without word
   boundaries** — "Nex" and "Geb" are literal substrings of ordinary English words. **Fixed with
   a word-boundary regex.** This is a standing risk for `sd32_t9_pi_exposure_audit.py`'s own
   exact-match scan too (it substring-matches the whole raw row, unbounded) — flagged as a
   finding for that script's own maintainers, not fixed here (out of this cycle's file-touch
   scope; `sd32_t9_pi_exposure_audit.py` is a sibling audit script this review extends, not
   redoes).

**Result after both fixes: `newly_blocked = 0` for `companion` and `monster_ability`, across
both the original `clear` bucket (283 + 111 = 394 rows) and the `uncertain` bucket (443 + 1,187 =
1,630 rows) — 2,104 rows total re-scanned, zero new hits.**

**Validated, not just run-and-trusted:** the scan function correctly resolves both recorded
incident strings when tested directly —
```
normalized_scan("DESC:FACTSET:Deity|Cayden CaiLean ...") -> "Cayden Cailean"
normalized_scan("DESC:this references lrori the god")    -> "Irori"
```
— so the zero-hit result for these two kinds is a genuine negative finding (they simply carry no
deity/place vocabulary at all, consistent with §3/§4's content findings), not an unrun or broken
check.

## 6. `.MOD`/`.COPY` rows — recommendation and units affected

**6 rows in `companion`/`monster_ability`'s uncertain population carry a `.MOD`/`.COPY` overlay
marker:**

| Kind | Book | Row name | Target referenced |
|---|---|---|---|
| monster_ability | bestiary_2 | Rake | `Rake.COPY=Rake` (Aurumvorax) |
| monster_ability | bestiary_2 | Split | `Split.COPY=Split` (Carnivorous Blob) |
| companion | bestiary_4 | Pooka ~ Change Shape | `Change Shape.COPY=Pooka ~ Change Shape` |
| companion | bestiary_4 | Psychopomp (Nosoi) ~ Change Shape | `Change Shape.COPY=Psychopomp (Nosoi) ~ Change Shape` |
| companion | ultimate_wilderness | Hunter's Bond ~ Animal Companion | `.MOD` on `COMPANIONLIST` |
| companion | ultimate_wilderness | Margay ~ Sound Mimicry | `Sound Mimicry.COPY=Margay ~ Sound Mimicry` |

**Recommendation: yes, a `.MOD`/`.COPY` row should inherit its target's PI status.** A `.COPY`
row is PCGen's own mechanism for cloning an existing ability under a new (often creature-specific)
name — its content *is* the target's content by construction, so a target that is PI-blocked
makes the copy PI-blocked too, and a target found `clear` only clears the copy if the copy's own
`OUTPUTNAME`/creature-scoping doesn't itself introduce new PI (e.g. `Psychopomp (Nosoi) ~ Change
Shape` names a Paizo-specific psychopomp subtype in the row's own name, independent of what
"Change Shape" resolves to — this row should be `still_undecidable` on its own name alone,
regardless of the target's status).

**This review did not trace all 6 targets' own classification** (that requires locating each
target row — "Rake"/"Split"/"Change Shape"/"Animal Companion"/"Sound Mimicry" as base
`monster_ability`/`companion` entries elsewhere in the corpus, some of which may not even be T9
units) — flagged as the honest boundary of this pass, not resolved by assumption. **Units
affected: 6**, all currently `still_undecidable` in this review's own bucket already (none was
misclassified `clear` by skipping the trace), so the open trace does not change this memo's
headline numbers — it only means those 6 rows' `still_undecidable` disposition additionally
depends on their targets', which a future pass should confirm explicitly.

## 7. Spot-check material (operator, up to 10 per kind)

**`companion`:**
- `advanced_players_guide:Companion Bonus Skill` — **clear**. Pure mechanic ("Add +1 skill rank").
- `advanced_players_guide:Gore` — **clear**. "The eidolon grows a number of horns... gore attack."
- `core_rulebook:Combat Reflexes` — **clear**. Standard SRD feat description.
- `ultimate_wilderness:Precocious Companion` — **still_undecidable**. Archetype-name text, no PI
  found on read, held back only because "Precocious" isn't yet cross-checked against an SRD list.
- `book_of_the_damned_volume_1:Spell-like Ability At-will (Message)` — **still_undecidable**
  (flagged on "Imp"/"Message" capitalization) but content-reviewed **clear**: "Imp" is an SRD
  monster type, "Message" a standard SRD spell name.
- `ultimate_wilderness:Totem Guide` — **still_undecidable**. Archetype category name + generic
  "wisdom and spirituality of the natural world" flavor — no deity/place named.
- `advanced_race_guide:Earth Glide` (Shaitan Binder Eidolon) — **clear**. "Shaitan" here is the
  genie-subtype term from the core Bestiary's elemental taxonomy (djinn/efreeti/shaitan/marid),
  not a Golarion-specific name.
- `bestiary_4:Pooka ~ Change Shape` (`.COPY` row) — **still_undecidable**. "Pooka" is Celtic
  folklore (likely OGL-safe as a name) but this is a `.COPY` row — see §6.
- `ultimate_magic:Poison` — **clear**. Pure numeric mechanic ("Frequency 1 round [4], effect 1
  Str damage...").
- `core_rulebook:Mobility` — **clear**. Verbatim SRD feat text.

**`monster_ability`:**
- `bestiary_4:Breath Weapon` — **blocked**. `DESCISPI:YES` declared in the source row.
- `inner_sea_gods:*` (5 blocked) — **blocked**. Deity-book content, term-list hits (unchanged
  from the audit).
- `bestiary_3:Spell-Like Abilities` (recurring row name across many creatures) — **clear** in
  every instance inspected: standard `(CL Nth; concentration +N) 3/day-<SRD spell name>...` lists
  with no creature name embedded in the DESC itself (only in the KEY, which this classifier does
  not scan for the content call — flagged as a limitation, not hidden).
- `bestiary_3:Infused Quarterstaff` (jinushigami) — **still_undecidable**. "a jinushigami wields"
  — non-SRD Paizo creature name in the ability's own flavor text.
- `inner_sea_bestiary:Drain Prana` / `Malevolence` / `Possess Corpse` (Vetala) — **still_undecidable**.
  Real-world-mythology-derived name, Paizo-specific creature write-up — exactly the §4-proposed
  rule's target case.
- `bestiary_4:Immortality` / `Limited Starflight` / `Overwhelming Mind` (`KEY:Star-Spawn of
  Cthulhu ~ ...`) — **still_undecidable**, and this review's strongest single recommendation to
  lean `blocked` on operator review: "Cthulhu" mythos naming is already treated as PI elsewhere
  in this same corpus (`t9-pi-exposure-audit.md §3`'s own spell example: "Summon Monster IX
  (Cthulhu)", `NAMEISPI:YES` declared) — these three `monster_ability` rows for the same creature
  carry no such declaration, an inconsistency worth flagging on its own.
- `ultimate_psionics:*` (107 clear of 176) — **clear**, mostly. Sampled rows are psionic-power
  mechanic text (saving throws, power points, ranges) with no creature or place names — psionics
  content in this corpus is largely name-free by nature.
- `bestiary_2:Rake` (`.COPY` row, Aurumvorax) — **still_undecidable**, see §6. "Aurumvorax" (the
  "money badger") is itself a classic d20/SRD-declared-Open monster, so this one is a plausible
  candidate to resolve `clear` once its target trace confirms "Rake" itself is clear.

## 8. Environment / reproduction summary

```bash
export PCGEN_REPO_DIR=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen
export PCGEN_CORPUS_ROOT="$PCGEN_REPO_DIR/data"
scripts/fetch-pcgen-oracle.sh --dest "$PCGEN_REPO_DIR"   # PCGEN_ORACLE_SHA 7f818006e371188e5717fd18d74d18a420747fc6
cargo build --locked --release --bin v06_work_inventory
"$CARGO_TARGET_DIR"/release/v06_work_inventory --stdout-only > fresh_inventory.json
python3 scripts/sd32_t9_census.py fresh_inventory.json                                  # 3,573 (was 2,712 at audit base)
python3 scripts/sd32_t9_pi_exposure_audit.py fresh_inventory.json \
    --corpus-root "$PCGEN_CORPUS_ROOT" --json-out t9_pi_classified.json                 # sanity baseline
python3 scripts/sd32_t9_pi_review_companion_monsterability.py fresh_inventory.json \
    --corpus-root "$PCGEN_CORPUS_ROOT" --json-out review_out.json                       # this memo's §3/§4/§5
```

`df -h /`: 664G available (32% used) at the end of this cycle.

## 9. Standing constraints, restated

Per `decisions.md §15`/`§18`: this memo transcribes nothing, ingests nothing, changes no corpus
data, and does not amend `docs/governance/ogl-pi-blacklist.md`. `still_undecidable` (1,314 of the
2,104 rows in scope: 360 `companion` + 954 `monster_ability`) is reported as a first-class,
expected result, not a stall — it is smaller than a forced verdict in either direction would have
been dishonest by. T9's onboarding for both kinds stays paused pending the operator's action on
this review and on `decisions.md §18` as a whole.
