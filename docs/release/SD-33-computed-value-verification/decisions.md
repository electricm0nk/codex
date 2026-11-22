---
canonical: true
owner: god-emporer
bundle_id: SD-33
date: 2026-08-24
---

# SD-33 Decisions

Bundle-specific ADRs. §1–§4 are derived directly from `../../retro/sd32-compute-library-and-cause-closure-retrospective.md`'s four findings and the operator retrospective conversation of 2026-08-24. **Each is expressed as a mechanism with an exit code, not as a lesson in prose** — that is §4's whole point.

---

## §1 — SD-33 builds its own `THE-BOX.md`, rebuilt rather than inherited, and it is enforced by a tool

**Decision.** Epic 1 ships `docs/release/SD-33-computed-value-verification/THE-BOX.md` plus `scripts/box_ledger.py` (name provisional until Epic 1 cycle 1 verifies no collision). The document is a **living partition** of the full inventory, and the tool **exits non-zero** when:

1. any unit is in no group (`uncovered != 0`),
2. any unit is in two groups (`overlap != 0`),
3. any group's computed value **disagrees** with the oracle,
4. any unit is **unverifiable** and that fact is not recorded as its own visible bucket, or
5. the document's recorded `derived_at` SHA is not an ancestor of current `HEAD` (**staleness gate**).

**Why.** SD-31's THE-BOX was a 377-line canonical artifact amended every wave, enforced by `scripts/coverage_ledger.py`. SD-32 inherited **a citation to it** and never built its own — five references, all backward-pointing and past-tense, one of them an anecdote inside a lessons list. The bundle's goal stopped being a thing anyone had to update, and the operator had to restate it repeatedly. Verified by mention-count trace, 2026-08-24.

**Explicitly rejected:** inheriting SD-31's 46-group partition. It was cut for a world where objects were not yet ingested. Its groups answer a question SD-32 closed.

**Revisit condition** (per `../../governance/deferral-revisit-doctrine.md`): none — this is a build obligation, not a deferral.

---

## §2 — The denominator gate: a percentage over a partial population fails the build

**Decision.** Epic 1 ships a check, wired into `scripts/verify.sh` as a real stage, that **fails** when a percentage is reported in a receipt, brief, or generated artifact without its denominator stated in the same construct. Every headline figure names the command that produces it and, where it derives from the pinned corpus, the `PCGEN_ORACLE_SHA`.

**Why.** The same defect appeared **three independent times in one session** (2026-08-24), and not one of the three was a false number:

| # | Figure | Reported as | Actually |
|---|---|---|---|
| 1 | `retro.py`'s `deferrals.open` | "10 open, all resolved" | `deferrals[-limit:]` — the last N. **29 total; 19 never checked** |
| 2 | Gate 2's corpus-wide engine run | "97.9% recognised" | 97.9% of the **4,798 it ran** = **41%** of the 11,652 that exist |
| 3 | the orchestrator's own scope figure | "8,446 units remaining" | one row of a nine-row cross-tab |

**A true number over the wrong denominator is the most expensive error shape in this program, precisely because the figure is correct and therefore survives review.**

**Corollary, binding on every cycle:** when a field is named `open`, `remaining`, or `outstanding`, **read its implementation before quoting it.** Two of the three above were mislabeled fields, not bad arithmetic.

---

## §3 — Version and branch placeholders, and where they resolve

**Decision.** `target_version: 0.13.0` (resolved). The placeholder was deliberately deferred per `workflow-instruction-template.md §1` item 7 (which forbids shipping `0.N.<current_build>` as live text). Resolution: on `tranche/13` cut, read `apps/desktop/package.json` and `apps/desktop/src-tauri/tauri.conf.json` and write the literal value into `README.md §7` and all workflow docs.

`develop` was at **`0.12.0`** (both files, verified 2026-08-24). The tranche digit bumped to `13` because SD-33 **cuts a new tranche branch** — per the standing rule that the digit moves on a new `tranche/N` cut, not on a bundle's own closure.

---

## §4 — Lessons carried forward must arrive with their enforcing tool

**Decision.** No lesson enters this bundle as prose alone. Every standing lesson in `workflow-instruction.md §12` either (a) names the command that enforces it, or (b) is marked `UNENFORCED` in the text, which is itself a defect to close.

**Why.** The operator's 2026-08-24 diagnosis was that lessons were not reaching the workflow-instructions. **The evidence showed the opposite, which is worse.** SD-32's `workflow-instruction.md §9` carries **seven standing lessons transcribed verbatim from `docs/retro/sd31-retrospective.md`** (line 506 cites lines 126–153) plus five named footguns; the file cites the retrospective 14 times. **Capture worked perfectly and the lessons were ignored anyway.**

What separated the lessons that held from the ones that broke was **mechanical form**, nothing else:

- **Held, all bundle:** `no_record == 0` (a gate with an exit code); the PI sweep (a test — it caught a real leak at closure); the base-SHA check (a command — the wrong-base failure fired 27 times in SD-31 and never recurred).
- **Broke:** lesson 2, *every figure carries its re-derive command* — violated five times in one session; lesson 7, *check deferral revisit conditions* — 19 deferrals unchecked, because the supporting tool was silently broken and nobody checked the tool.

**SD-32 mechanized its own new goals and de-mechanized the inherited ones. The handoff loses mechanisms, not lessons.** Lesson 1 already says *"recurring incidents get a mechanical control, not a better-worded warning"* — and lesson 1 is itself prose.

---

## §5 — The oracle harness is a timeboxed spike with its fallback named in advance

**Decision.** Epic 2 is bounded. Its exit is a **ruling**, not an open-ended effort:

- **Path A (preferred).** Build the pinned PCGen headless and drive `BatchExporter.exportCharacter(characterFilename, outputFile)` against authored `.pcg` inputs, exporting via a template that dumps computed variables.
- **Path B (fallback, proven).** Read PCGen's Java source per shape and derive the expected semantics — the method that produced SD-32's `MaxCommand.java` finding, which corrected a pinned test encoding a false assumption. Slower and per-shape, but it works in this repo today.

**Known facts, verified 2026-08-24:** `java -version` → OpenJDK 25 (Temurin). The pinned oracle carries `code/src/java/pcgen/system/BatchExporter.java`, `CommandLineArguments.java`, `Main.java`, `build.gradle`, `gradlew`. `BatchExporter` exposes `exportCharacter(String, String)`.

**Known risks, stated rather than assumed:** the Gradle build may not accept Java 25; `BatchExporter` imports `pcgen.gui2.UIPropertyContext`, so the headless path has GUI coupling; batch export consumes **existing** characters, so `.pcg` inputs must be authored. None of these has been tested — they are risks, not findings.

**Revisit condition:** Epic 2 declares Path A or Path B **by its own closing receipt**. If Path A fails, **Epic 5's throughput assumption changes and that is an operator decision point**, raised explicitly per `../../governance/blocker-closure-doctrine.md` — never absorbed silently, and never allowed to quietly reduce the bundle to "coverage only".

---

## §6 — SD-32's instrument debt closes inside SD-32, not here

**Decision.** Three named SD-32 items — the `retro.py` `deferrals.open` defect, the 7 genuinely unverified deferrals, and `EXCLUDED_BOOKS = ['beginner_box']` — are **SD-32 launch-gate work**, not SD-33 epics. SD-33 does not open until they are closed.

**Why.** Importing a predecessor's unfinished scope into a successor bundle is the exact laundering `../../governance/blocker-closure-doctrine.md` exists to remove, and the pattern the operator has rejected in every prior instance. Cost is not a disposition; neither is a change of bundle number.

**Note on `beginner_box`:** 19 units, status `not-started`, excluded in code inside `coverage_ledger.not_done_population()` — the function every SD-32 closure figure is computed over. It survived every `§27b` prose sweep because it lives in Python rather than in a document. **19 units is small; the precedent is not** — it sat in the one place a carve-out is invisible.

**Standing corollary for SD-33's own sweeps:** carve-out sweeps grep **code as well as prose** — `EXCLUD*`, `SKIP*`, `IGNORE*`, `_ALLOWLIST`, `_DENYLIST`, and any hardcoded book/kind list in the scripts that compute closure figures.

---

## §7 — "Cannot be verified" is a visible bucket, never folded into done

**Decision.** `THE-BOX.md`'s partition carries an explicit **`unverifiable`** group with its own count and its own named reason per unit. `scripts/box_ledger.py` fails if any unit's disposition is `done` on the strength of an absent check.

**Why.** SD-32's `doneness_verdict()` maps `fixture-verified` (1,741) and `literal-verified` (6,589) to `done` — **8,330 units blessed against artifacts we wrote ourselves, never against the oracle** — and being outside the ledger population means no future cycle re-examines them. Unprovable units want to look finished. This is the structural reason Epic 5 exists.

**Related standing finding:** the probe surface is thin. Epic 1's criterion 1.3 **enumerates it for real** rather than restating it from memory; kinds with no probe can never leave `unverifiable`, and their count is a bundle-level number, not a footnote.
