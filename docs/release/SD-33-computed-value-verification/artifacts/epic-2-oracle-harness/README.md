# Epic 2 — Oracle harness artifacts

Covers `AT-33-E2-001..004`. Produced 2026-08-25 on `tranche/13`.

## AT-33-E2-001 — Path A feasibility, by execution

**Ruling: Path A is feasible on this box.** The pinned PCGen
(`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`) builds headless
and exports a character with real computed values (see `AT-33-E2-002` below).

The three named risks (`decisions.md §5`), each resolved to a fact by running
the build/run, not by inspection:

1. **Gradle vs Java 25 — NOT a conflict.** `build.gradle` at this SHA pins
   `javaVersion = 25` via a Gradle toolchain block, and the wrapper resolves
   Gradle `9.5.1`, which launched and ran cleanly on the box's only installed
   JDK (`OpenJDK 25 Temurin`). `./gradlew --version` output:
   `build-transcript-01-gradlew-version.log`.
2. **`pcgen.gui2.UIPropertyContext` coupling — real, but non-blocking for
   batch mode.** `Main.loadProperties()` registers `UIPropertyContext` even
   on the headless (`-E`/batch) path (`startupWithoutGUI()` calls
   `loadProperties(false)`, which unconditionally calls
   `registerPropertyContext(UIPropertyContext.getInstance())`). Reading
   `UIPropertyContext.java`: its properties are `javafx.scene.paint.Color`
   value objects, not `java.awt.Toolkit`/display calls, so no windowing
   system or display server is required — confirmed by the batch export
   completing on a box with no display server (`build-transcript-05-batchexport-SUCCESS.log`).
   The coupling is an architecture smell (batch mode importing from `gui2`),
   not a headless blocker.
3. **`.pcg` input authoring — solved.** Batch mode consumes existing `.pcg`
   files (`BatchExporter.exportCharacter`); none is generated at runtime. A
   `.pcg` was hand-authored for this cycle (`fixtures/pf1_fighter_l1.pcg`),
   using the repo's own `code/testsuite/PCGfiles/*.pcg` samples only to
   confirm the tag vocabulary (`PCGVer2Creator.java`'s format comments), not
   copied from them.

**Compile transcripts:** first attempt failed for the intended reason (a
`settings.gradle`-declared subproject, `PCGen-Formula`, was outside the
initial sparse-checkout cone) —
`build-transcript-02-compileJava-first-attempt-FAILED.log`. Corrected cone,
`compileJava` succeeded — `build-transcript-03-compileJava-SUCCESS.log`. Full
plugin jar build (needed at runtime by `PluginClassLoader`) —
`build-transcript-04-jar-SUCCESS.log`.

## AT-33-E2-002 — a character round-trips through the oracle

**Real value came out.** Committed:

- **The `.pcg`:** `fixtures/pf1_fighter_l1.pcg` — hand-authored, Level 1
  Human Fighter, `CAMPAIGN:Core Rulebook` only (STR 16 / DEX 14 / CON 14 /
  INT 10 / WIS 10 / CHA 8).
- **The template:** `computed-values.txt.ftl` — hand-authored FreeMarker
  export sheet. Emits `pcstring(...)` tokens for PCGen's own **computed**
  variables (HP, AC, BAB, `VAR.CMB`/`VAR.CMD`, and the three saves via
  `pc.checks`), not literal LST text — token names were cross-checked
  against the stock `outputsheets/base.xml.ftl` at the same pinned SHA
  before authoring, to use the real export-token vocabulary rather than
  guessed names.
- **The emitted output:** `pf1_fighter_l1.computed.txt` (machine-readable
  `KEY=VALUE` lines, one token per line).
- **The command actually run** (from the checkout root, after `-s` settings
  dir existed and `run-settings/` had been created):

  ```bash
  export JAVA_HOME=/home/ubuntu/.sdkman/candidates/java/current
  cd <pcgen-oracle-checkout>
  ./gradlew run --console=plain --args="-s run-settings \
    -E <repo>/docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/computed-values.txt.ftl \
    -c <repo>/docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/fixtures/pf1_fighter_l1.pcg \
    -o <repo>/docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/pf1_fighter_l1.computed.txt"
  ```

  Full transcript: `build-transcript-05-batchexport-SUCCESS.log` (0 `SEVERE`
  lines; the only non-`INFO` lines are benign `Evaluation called on invalid
  variable` notices from an unrelated formula channel not exercised by this
  character). The **first** attempt at this export (before the checkout's
  sparse cone included `data/homebrew`/`data/_universal`) failed for the
  intended reason — `core_rulebook` transitively references
  `data/homebrew/conversion_support/conversion_support.pcc`, a top-level
  `data/` directory outside the pin's `data/pathfinder` sparse scope, and
  PCGen correctly refused to load with `Could not find campaign by
  filename` / `NoSuchFileException`. Widening the checkout's sparse cone to
  include both `data/homebrew` and `data/_universal` fixed it; that failing
  transcript was superseded by a clean re-run once the fixture identifiers
  were renamed to their current, non-bundle-prefixed form (see the
  identifier audit in this cycle's receipt) and is not separately
  committed, since the underlying cause is fully described here and the
  corrected, real success transcript is what's committed.

**Independent cross-check (not the harness's own claim):** the emitted
values were hand-derived from PF1e core rules *before* being compared to the
run's output — STR mod +3 / DEX mod +2 / CON mod +2; HP = 10 (Fighter d10,
max at level 1) + 2 (CON) = 12; AC = 10 + 2 (DEX) = 12 (touch 12,
flat-footed 10); BAB = +1 (full progression, level 1); Fort = 1/2+2 (int
div) + CON 2 = +4; Ref = 1/3 (int div) + DEX 2 = +2; Will = 1/3 (int div) +
WIS 0 = +0; CMB = BAB 1 + STR 3 = 4; CMD = 10 + BAB 1 + STR 3 + DEX 2 = 16.
All eight independently-derived values match `pf1_fighter_l1.computed.txt`
exactly.

## Oracle checkout provenance (not committed — see `.gitignore`)

`pcgen-oracle-checkout/` is a scratch git checkout of
`https://github.com/PCGen/pcgen.git` at
`7f818006e371188e5717fd18d74d18a420747fc6` (the pin), sparse-checked-out to
`code gradle system PCGen-base PCGen-Formula outputsheets plugins preview
data/homebrew data/_universal`, plus `data/pathfinder` copied byte-for-byte
from the already-pinned repo-local slot
(`docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data/pathfinder`,
itself at the same pinned SHA — confirmed via that checkout's `git rev-parse
HEAD`). It is **not** the `~/workspace/repos/pcgen` path forbidden by
`workflow-instruction.md §1.7` — it lives entirely under this cycle's
granted write scope and is gitignored, matching `scripts/pcgen-oracle-pin.env`'s
"public repo, never vendored" rule. Reproduce with:

```bash
cd docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness
mkdir pcgen-oracle-checkout && cd pcgen-oracle-checkout
git init -q
git remote add origin https://github.com/PCGen/pcgen.git
git sparse-checkout init --cone
git sparse-checkout set code gradle system PCGen-base PCGen-Formula outputsheets plugins preview data/homebrew data/_universal
git fetch --depth 1 --filter=blob:none origin 7f818006e371188e5717fd18d74d18a420747fc6
git checkout --detach FETCH_HEAD
cp -r ../../../../SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data/pathfinder data/pathfinder
```

## AT-33-E2-003 / AT-33-E2-004

See `oracle-comparison-fixtures.md` and this same README's closing section
below (added by the same cycle) for the comparison harness and the Path A
ruling.
