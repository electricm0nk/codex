# Operator rulings — 2026-08-19

Recorded by the orchestrator during wave 15. Written to its own file rather than into
`decisions.md`/`OPEN-ISSUES.md` because wave 15's integration cycle is the designated writer for
those two files and two writers on one shared branch is a standing hazard. Fold these in at the
next integration.

---

## Ruling §16 — Core Essentials residuals not found in print are DELETED, not flagged

**Operator, verbatim:** *"for any unit from core essentials that you can not find in printed books
- remove them completely. dont just flag them as escaped or unmeasurable - delete them. I consider
them hallucinations until they appear in print."*

This supersedes `OPEN-ISSUES.md` row 263's options (b) and (c). Deletion is the disposition, and no
Structural Exclusion Register entry is needed for the deleted population: an entry excludes a real
unit from the denominator, whereas these are being ruled not to exist.

**"Found in print" is decided by evidence, not memory** (`decisions.md §10`'s binding rule). The
test applied below: does any book OTHER than `core_essentials` carry a real race DECLARATION row
for it in the pinned oracle (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`)? A
mention inside a `PRETEMPLATE:`/prerequisite on some other book's feat row is NOT a declaration —
`mythic_adventures/ma_feats.lst` names seven of these eight races only in `Racial Heritage ~ <race>`
mythic feat rows, which reference a race without printing one.

### The 128 residual units split four ways

Re-derived at ruling time from `docs/work-inventory.json` (`book == "core_essentials"`), all 128
`not-started`, none `done` — so no unit of credit is lost by any disposition here, and the board
percentage can only rise as the denominator shrinks.

| group | units | evidence | disposition |
|---|---|---|---|
| **Ghoran** | 13 (12 `race_trait` + 1 `race`) | Declared in `ultimate_wilderness/uw_races.lst` — a real printed race in a book already on the 37-book roster | **RE-ATTRIBUTE** to `ultimate_wilderness`. Not a hallucination; do not delete. |
| **`ce_abilities_race.lst` rows naming a Bestiary** | ~28 (of the file's 29) | The file's own `SOURCELONG:` headers name Bestiary, Bestiary 2 (×3 each), and Bestiary 3/4/5/6 (×1 each) — all six on the roster | **RE-ATTRIBUTE** per each row's own `SOURCELONG:` |
| **`ce_abilities_race.lst` rows under `SOURCELONG:Universal Rules`** | ~1 | "Universal Rules" is a PCGen construct, not a Paizo book | **DELETE** — not in print |
| **Android, Aquatic Elf, Gathlain, Lashunta, Monkey Goblin, Syrinx, Triaxian** | 86 (79 `race_trait` + 7 `race`) | No race declaration in ANY book outside `core_essentials` at the pinned oracle. Searched `*races*.lst` corpus-wide | **DELETE** |

Net: roughly **87 deleted, 41 rescued**. Execute against re-derived figures, not these — the exact
Bestiary/Universal-Rules split within the 29 must be read per row.

**Consequence to check on completion:** once every residual is deleted or re-attributed, the
`core_essentials` label reaches zero and `decisions.md §9`'s condition is finally discharged.

**Caveat the operator should know:** Gathlain and Monkey Goblin have real printed PF1e sources
(Ultimate Wilderness and a Player Companion respectively) that this repo's pinned PCGen oracle does
not carry as race declarations. They are being deleted for absence of evidence in our oracle, not
proof of absence in print. If either book's ingest later declares them, they return through the
normal path — which is exactly what "until they appear in print" allows.

---

## Ruling §17 — investigate duplicate display names for double-counting

**Operator, verbatim:** *"I was looking at the dashboard drill down and noticed some things listed
twice. ie: core rules class features lists aberrant bloodline twice and not started. this kind of
duplication could be skewing our numbers if they are not legit"*

**Investigated at ruling time. The headline concern is mostly NOT a counting defect, but a real
subset is.**

Corpus-wide, 2,325 `(book, kind, name)` groups hold more than one unit, an excess of **4,266 units
= 11.07% of the denominator**. That figure is not the double-count, and must not be quoted as one:

- **0** of those groups share a `corpus_key`.
- **0** share a `source_file` + `source_line`.

Every unit is a distinct printed row. Most collisions are display-name collisions where the rows
are genuinely different things — e.g. `advanced_class_guide` `class_feature` "Bloodline Powers"
appears 11 times because eleven different bloodlines each print their own Bloodline Powers row.
Collapsing those would destroy real content. This is a **drill-down display defect** — the page
shows a bare `name` that is not unique — and the fix is to show a disambiguator (the `type_facet`
or corpus key), not to merge rows.

**The operator's own example is the real subset.** `core_rulebook` `class_feature`:

| line | corpus key | facet |
|---|---|---|
| `cr_abilities_class.lst:2333` | `Sorcerer Bloodline ~ Aberrant` | the class feature |
| `cr_abilities_class.lst:2334` | `Aberrant Bloodline` | `SorcererBloodlineChoice` — the picker entry that selects it |

Adjacent lines in one file: PCGen's paired feature + chooser shape. Two printed rows, arguably one
game concept.

**Bounded population:** 787 units carry a chooser facet; **180** have an adjacent matching feature
row and are the double-count candidates (`companion` 108, `class_feature` 70, `race_trait` 2). All
180 are `not-started` (121) or `unmeasurable` (59) — **none is `done`** — so removing them shrinks
the denominator without withdrawing any credit, and the percentage rises. The other 607 chooser-facet
units have no such pair and are likely genuine standalone content.

**Owed work:** confirm the 180 case by case (adjacency is a heuristic, not proof — a chooser whose
paired feature is a genuinely separate mechanic must stay), then remove the confirmed ones from the
unit ledger; and separately fix the drill-down to disambiguate same-named rows so the page stops
looking wrong where it is right.
