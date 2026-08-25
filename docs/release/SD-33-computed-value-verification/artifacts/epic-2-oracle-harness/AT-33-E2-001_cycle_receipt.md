# Cycle AT-33-E2-001 — Epic 2 Oracle harness / AT-33-E2-001

- **Commit SHA:** (recorded post-commit — see `progress.md`'s pointer entry for this cycle, added in the same commit)
- **Files touched:**
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/README.md` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/.gitignore` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/build-transcript-01-gradlew-version.log` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/build-transcript-02-compileJava-first-attempt-FAILED.log` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/build-transcript-03-compileJava-SUCCESS.log` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/build-transcript-04-jar-SUCCESS.log` (new)
  - `docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/AT-33-E2-001_cycle_receipt.md` (this file)
- **Identifier audit result:** `OK_NO_BUNDLE_TAGS`
- **Wired-integration audit result:** `OK_NO_TOKENS`
- **Acceptance criterion (verbatim, `epic-breakdown.md`):**
  > ### AT-33-E2-001 — Path A feasibility is established by execution
  >
  > The pinned PCGen (`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`) either builds headless on this box or does not, **proven by running the build**, with the failure mode named if it fails.
  >
  > **Evidence:** build transcript in `artifacts/epic-2-oracle-harness/`. The three named risks (`decisions.md §5`) are each resolved to a fact: Gradle-vs-Java-25, `pcgen.gui2.UIPropertyContext` coupling, `.pcg` input authoring.

## What landed

Fetched the pinned PCGen commit into a fresh, cone-mode sparse git checkout
(`code gradle system PCGen-base PCGen-Formula outputsheets plugins preview`,
plus `data/pathfinder` copied from the already-pinned SD-32 repo-local
slot, plus `data/homebrew`/`data/_universal` — see `README.md`'s
"Oracle checkout provenance" section for the exact reproduction commands),
inside this cycle's own granted write scope
(`artifacts/epic-2-oracle-harness/pcgen-oracle-checkout/`, gitignored — never
`~/workspace/repos/pcgen`, per `workflow-instruction.md §1.7`). Ran
`./gradlew --version`, `./gradlew compileJava`, and `./gradlew jar` for
real, on `OpenJDK 25 Temurin` (the box's only installed JDK).

**All three named risks resolved to facts, by execution:**

1. **Gradle vs Java 25 — not a conflict.** `build.gradle` at the pinned SHA
   sets `javaVersion = 25` via a Gradle toolchain block; the wrapper
   resolved Gradle `9.5.1`, which ran cleanly against the box's Java 25.
2. **`pcgen.gui2.UIPropertyContext` coupling — real, non-blocking.**
   `Main.startupWithoutGUI()` (the batch/headless path) calls
   `loadProperties(false)`, which unconditionally registers
   `UIPropertyContext` regardless of GUI mode — confirmed by reading
   `code/src/java/pcgen/system/Main.java`. Reading
   `code/src/java/pcgen/gui2/UIPropertyContext.java` shows its properties
   are `javafx.scene.paint.Color` value objects, not
   `java.awt.Toolkit`/display calls — no display server is touched.
   Confirmed empirically in `AT-33-E2-002`: the batch export completed on
   this box, which has no display server.
3. **`.pcg` input authoring — solved.** `BatchExporter.exportCharacter`
   consumes an existing `.pcg`; none is generated at runtime. Authored one
   by hand (`AT-33-E2-002`), using the repo's own
   `code/testsuite/PCGfiles/*.pcg` samples only to confirm the tag
   vocabulary, not copied from them.

First `compileJava` attempt failed for the intended reason — the initial
sparse-checkout cone omitted the `PCGen-Formula` subproject
`settings.gradle` declares, and Gradle correctly refused to configure a
missing project directory
(`build-transcript-02-compileJava-first-attempt-FAILED.log`). Widening the
cone to include `PCGen-base`/`PCGen-Formula` fixed it
(`build-transcript-03-compileJava-SUCCESS.log`). `jar`/`jarAllPlugins`
(needed at runtime by `PluginClassLoader`) also succeeded
(`build-transcript-04-jar-SUCCESS.log`).

## Figures + their re-derive commands

| Figure | Value | Denominator | Command |
|---|---:|---|---|
| Named risks resolved to a fact | 3 | of 3 named in `decisions.md §5` | manual: read `Main.java`/`UIPropertyContext.java`, then run the commands below |
| `./gradlew --version` exit code | 0 | of 1 attempt | `./gradlew --version` (from the checkout root) |
| `./gradlew compileJava` first attempt exit code | 1 | of 1 attempt, failed for the intended reason (missing subproject dir) | `build-transcript-02-...FAILED.log` |
| `./gradlew compileJava` corrected attempt exit code | 0 | of 1 attempt | `build-transcript-03-...SUCCESS.log` |
| `./gradlew jar` exit code | 0 | of 1 attempt | `build-transcript-04-...SUCCESS.log` |
| Plugin jars produced | 11 | of 11 `createJarTask` calls in `code/gradle/plugins.gradle` | `ls pcgen-oracle-checkout/plugins/*.jar \| wc -l` (11 of 11 present after `jarAllPlugins`) |

## Status: complete

## Movement, four buckets

- **closure:** 0 — this cycle proves a build feasibility fact; it moves no inventory unit.
- **reclassification:** 0
- **reachability:** 0
- **instrument-correction:** 0

## Notes

- The pinned PCGen repo/pin SHA is read only from `scripts/pcgen-oracle-pin.env`
  (never hardcoded), matching `AGENTS.md`'s "PCGen oracle is pinned, never
  cited by literal local path" rule for the *pin*; the oracle *checkout
  path itself* is necessarily a literal path in this receipt/README because
  it lives inside this cycle's own scratch artifacts directory, not at
  `$PCGEN_REPO_DIR`'s conventional default — documented explicitly in
  `README.md`'s provenance section so a future cycle does not mistake it
  for the repo-wide `$PCGEN_CORPUS_ROOT` convention.
- `pcgen-oracle-checkout/` is gitignored (this cycle's own
  `artifacts/epic-2-oracle-harness/.gitignore`) and was never `git add`ed —
  confirmed via `git status --porcelain` before every commit this cycle;
  only the transcripts, the `.pcg`, the template, and the emitted output
  are committed.

## Next-cycle plan

`AT-33-E2-002` (same cycle, same commit) authors the `.pcg` + template and
runs the real export; `AT-33-E2-003` builds the comparison harness;
`AT-33-E2-004` records the Path A/B ruling.
