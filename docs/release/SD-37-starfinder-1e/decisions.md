---
canonical: true
owner: operator
bundle_id: SD-37
date: 2026-10-02
status: operator-away defaults — every ruling below is a default-and-flag, pending operator review on return
---

# SD-37 Decisions

Bundle-specific ADRs for SD-37 (Starfinder 1e). **The operator is away.** Every decision below
was taken as an **operator-away default** at authoring time (2026-10-02), from the orchestrating
session's brief, and is flagged for review. Each one says what it decides, what enforces it, and
what to change if the operator disagrees. All of them are listed in `README.md §6 Open rulings`.

Figures are cited by ID from `content-unit-inventory.md §1` (`CUI F-n`). Each of those IDs carries
its command and its predicate.

---

## §1 — Package location: `docs/release/SD-37-starfinder-1e/` (flagged deviation)

**Operator-away default (2026-10-02).** The package lives at `docs/release/SD-37-starfinder-1e/`.

**Why.** The repo keys on `docs/release/SD-NN-<slug>/`: the repo's own chassis
(`docs/release/template/template.md`, `docs/governance/workflow-instruction-template.md`), the
repo-local skill `.claude/skills/stc-authoring/SKILL.md` and the bundle scripts all use that path,
and `docs/stc/` does not exist (`ls docs/stc` → "No such file or directory").

**Flagged deviation.** The user-level overlay `~/.claude/skills/stc-codex-overlay/SKILL.md` says new
Codex bundles live at `docs/stc/stc-<##>-<description>/`. This package does **not** follow the
overlay. Moving it would break every script and skill that keys on `docs/release/`, and the overlay
does not say how to migrate them.

**Enforced by:** `ls docs/release/SD-37-starfinder-1e/README.md`.

**Revisit if the operator disagrees:** a dedicated cycle moves the package and every reference to
it (`grep -rn 'SD-37-starfinder-1e' docs scripts .claude`), or the overlay is amended to match the
repo. No cycle should do this without a ruling.

---

## §2 — Branch, board, version

**Operator-away default (2026-10-02).**

- **Branch:** `tranche/17`, cut from `origin/develop` at `20bf84a3b2` (PR #394's merge commit,
  2026-09-30). It is **not** cut from `tranche/16`. PR #393 (tranche/16 → develop) merged on
  2026-09-29, so tranche/16 carries nothing develop lacks. Local `develop` is stale (`50572eebad`):
  run `git fetch origin` first and cut from `origin/develop`.
- **Board:** local file `./kanban.md`, paired with `./progress.md`. The Hermes board was retired
  on 2026-08-01.
- **Target version:** `0.17.0`. The version bump is **card C1's single commit**, across all 14
  surfaces (CUI F-17):
  `.github/workflows/publish-tester-release.yml` (stamp line `VERSION="0.16.${GITHUB_RUN_NUMBER}"` →
  `0.17.`), `apps/desktop/package.json`, `apps/desktop/src-tauri/{Cargo.toml,Cargo.lock,tauri.conf.json}`,
  both `buildVersionTriple.test.ts` copies (`apps/desktop/src/release/` and
  `apps/desktop/src/releaseChecks/`: assert `startsWith('0.17.')` and anchor comment "cut as
  tranche/17"), `apps/desktop/src/testSupport/makeSurface.ts`, and the six build-label fixtures
  under `apps/desktop/src/{operatorTriage,testerWorkbench}/`. Root `Cargo.toml` stays `0.1.0`.
  C1 re-derives the list at bump time with both CUI F-17 commands, and they must name the same 14
  files.
- **Version rule.** The tranche digit moves only when a new bare `tranche/N` is cut. tranche/17 is
  such a cut, so 0.16 → 0.17 at C1 is correct. At SD-37's own closure only the build position
  moves (`0.17.${GITHUB_RUN_NUMBER}`, stamped at publish). Closure must not bump the tranche digit.
- **First concrete build value:** `0.17.0` in the repo files. The published triple is
  `0.17.<GitHub run number of the first tester publish after C1>`, and it **resolves at that
  publish**. C1's receipt records the run number once it exists.
- **PR #395** (`fix/finalize-no-msi-glob`, open, mergeable) does **not** gate the cut. If it merges
  before C1, C1 rebases `tranche/17` onto the new `origin/develop` before the bump commit. If it
  merges after, the closure PR resolves the expected small conflict in
  `publish-tester-release.yml` (#394 already touched the same file).
- **SD-36 loose ends are not a gate.** They are card **C0.1** (Haiku housekeeping): the stale
  SD-36 README `status:` line and the kanban D2–D6 row; 10 pre-existing worktrees (CUI F-19); 7
  merged remote `sd36/*` branches (CUI F-20); the uncommitted
  `docs/retro/events/root.jsonl` append in the main checkout. C0.1 touches only what no live
  session owns (`git worktree list` lock state, `git reflog --date=iso -3` in each tree,
  `git status --porcelain` empty, `git rev-list --count origin/develop..<branch>` = 0). It never
  deletes `test` or `update-index`. It never touches the main checkout's dirty file while another
  session holds that tree: it logs it and moves on.

**Enforced by:** C1's receipt (`git show --name-only --format= <bump-sha> | awk 'NF' | wc -l` → 14,
root `Cargo.toml` absent); `apps/desktop/src/release*/buildVersionTriple.test.ts`;
`git merge-base --is-ancestor 20bf84a3b2 tranche/17`.

**Revisit if the operator disagrees:** if the operator wants #395 to land first, C1 waits for it
(that card is then `blocked-escalated` only if #395 is still open after every independent card is
done). If the operator wants a different version, change C1 only.

---

## §3 — Epic spine

**Operator-away default (2026-10-02).** The spine comes from the engine-reuse report §8, with the
properties the brief requires:

```
C0 → C1 → { E0 ∥ E1 } → E2 → E3 → { E4 ∥ E5 } → E6 → E7
                    E1 → E4a  (own worktree, own CARGO_TARGET_DIR, runs alongside E3–E6)
E8 (starship) = planned capability deferral, not a card (§17)
```

- **E0** — oracle and licence admission. Extend `PCGEN_ORACLE_SPARSE_PATHS` with `data/starfinder`
  and `system/gameModes/Starfinder`, add an SF completeness probe to `scripts/fetch-pcgen-oracle.sh`,
  make `verify.sh preflight-oracle` check SF, and prove it in a fresh clone. Also: the licence
  matrix, the SF PI term set, the SF denominator, and the seed hand values.
- **E1** — partition the rule package by game system. Proven by **byte-identical** PF renders of
  Aldric (Fighter 3) and Elowen (Wizard 5) under the structural-diff protocol (49,450 records
  unmoved, CUI F-15).
- **E2** — additive schema extension, plus a published `schemas/rules/*.schema.json`.
- **E3** — the Starfinder converter (§8, §10).
- **E4** — SF chassis compute. Generic. **No per-class modules.**
- **E5** — SF print-path content.
- **E6** — desktop SF surfaces (§15).
- **E7** — verification and closure (§13).
- **E4a** — `rules_tables` → data package (§19). In scope, off the critical path, in parallel.

**Enforced by:** `epic-breakdown.md §0` (parallel/serial map) and `workflow-instruction.md §3`
(file-touch fences). The Workflow script's `phases` must match these titles.

**Revisit if the operator disagrees:** E4a is the movable part. If the operator wants a smaller
bundle, E4a can be split into its own SD-N. That is a scope ruling, and only the operator can make
it (`§19`).

---

## §4 — Book scope: tune on the Core Rulebook, then go wide

**Operator-away default (2026-10-02).** This applies the two-book-proof lesson (memory
`two-book-scope-is-a-deliberate-proof`): prove the method on one book, then run every remaining
book in one batch.

- **Proof book:** Core Rulebook (`paizo/core`, 5,974 rows; CUI §2). E3.4 tunes the converter, the
  mapping table and the gates on this book alone.
- **Go wide (one batch, E3.5):** Armory, Character Operations Manual, Pact Worlds, Near Space, and
  Alien Archive 1–3. Alien Archive content enters as the `.lst` carries it (playable races,
  abilities, spells, equipment). The print path is a character sheet, not a monster simulator.
  PCGen carries no monster stat-block rows for these books (engine-reuse report §1(C)). Every
  in-scope record is still **ingested and measured** (memory `no-carve-outs-close-dont-flag`):
  "not reachable from a sheet" is a reachability number, never an ingest exemption.
- **Excluded books:** see §6.

**Enforced by:** E3.4's and E3.5's acceptance commands (`epic-breakdown.md`); the denominator from
E0.3 sums to the in-scope books.

**Revisit if the operator disagrees:** add or remove a book in E0.3's registry, then re-run E3.5.

---

## §5 — Paper-sheet-generator rule

**Operator ruling (standing, 2026-09-07), restated as binding for SD-37.** Campaign Codex is a
paper character-sheet generator, not a video game. Only values that feed sheet totals are
computed: EAC, KAC, saves, BAB and attack bonuses, skill totals, HP, Stamina, Resolve, spells per
day and DCs, bulk, and credits. Everything else is **printed**. Every term that resolves for this
character is added into one number ("DC 15", not "10 + level + Wis"). Dice stay literal. A term the
character has not settled stays as words. No simulation engines, no consumer-delta proofs, no
per-unit proof harnesses.

**Enforced by:** each epic's criteria; reviewers reject any card whose diff builds compute machinery
for a print-only rule. The test question: does this value land in a sheet total?

**Revisit:** standing ruling; not an operator-away default.

---

## §6 — Exclusions and the licence matrix as a deliverable

**Operator-away default (2026-10-02).**

- **Starfinder Society rules (`paizo/starfinder_society_rules`, SSRGG)** — excluded. Its PCC reads
  "All Rights Reserved" and carries a Paizo trademark line (source-inventory report §1).
- **LPJ Design Infinite Space (`lpj_design/infinite_space`)** — excluded. It is third-party, it
  holds themes only, and its licence is not verified.
- The SF rows of `docs/governance/license-matrix.md` are an **E0.2 deliverable**, not an
  assumption. `grep -ni starfinder docs/governance/license-matrix.md` returned nothing on
  2026-10-02. E0.2 records, per book: licence, PI posture, `operator_sign_off` (false until the
  operator signs), and the SF PI term set (Pact Worlds proper nouns and so on). The exclusions above
  are measured exclusions: E0.3 reports their row counts (214 rows, CUI F-6) as a named number.
  They are not silent.
- **Safe default for a book whose licence or PI status E0.2 cannot settle:** exclude it, log it, do
  not ingest it (the SD-28 precedent).

**Enforced by:** E0.2's receipt; the converter's book registry, which lists only signed-off-or-OGL
books, so an excluded book cannot be read. A test asserts that the two excluded directories are
absent from the SF book registry and are named in the exclusion record.

**Revisit if the operator disagrees:** if the operator verifies the LPJ licence, add a go-wide
cycle for its 37 rows. SSRGG stays out unless Paizo's terms change.

---

## §7 — Data layout: PF paths unchanged; SF under `data/starfinder-1e/`

**Operator-away default (2026-10-02).** This is the smallest compliant change. PF stays where it
is (`data/sheet_rules/`, `data/corpus/`, `data/converted/`), registered as the `pathfinder-1e`
system root. Starfinder data goes under `data/starfinder-1e/{sheet_rules,corpus}`. The system id
matches the desktop's existing `RuleSetId` literal `'starfinder-1e'`
(`apps/desktop/src/characterHub/LandingScreen.tsx:11`) and the save envelope's `game_system`
field (`src/saved_character/mod.rs:34`). Moving 56,008 PF sheet-rule files (quoted, engine-reuse
report §2) would add churn without any player-visible gain.

**Enforced by:** E1.1's registry test (each system id resolves to exactly one root, and the PF
root is the current path); the `tauri-resources-tracked` stage, extended in E6.1 to the SF root.

**Revisit if the operator disagrees:** a later cycle can move PF under `data/pathfinder-1e/` behind
the same byte-identical gate.

---

## §8 — The overloaded-field hazard: Starfinder gets its own mapping table, one oracle row per overloaded field

**Operator-away default (2026-10-02). Known hazard.** PCGen's SF data reuses PF field names with
different meanings. If you read it through the PF mapping (`mapping-table.v1.json`, 273 rows, CUI
F-14), it prints numbers that **look valid and are wrong**. Observed in the pinned oracle
(`grep -E '^CLASS:(Soldier|Envoy|Mystic|Technomancer)\b' $S/paizo/core/scr_classes.lst | tr '\t' '\n' | grep -E 'HP\||HD:'`):

| Class | `HD:` | `BONUS:HP\|CURRENTMAX` | `BONUS:HP\|ALTHP` |
|---|---|---|---|
| Soldier | 1 | `6*SoldierLVL` | `7*SoldierLVL` |
| Envoy | 1 | `5*EnvoyLVL` | `6*EnvoyLVL` |
| Mystic | 1 | `5*MysticLVL` | `6*MysticLVL` |
| Technomancer | 1 | `4*TechnomancerLVL` | `5*TechnomancerLVL` |

- `HP|ALTHP` is **Hit Points**. `HP|CURRENTMAX` is **Stamina**, and `CURRENTMAX` is one less than
  the per-level Stamina that the planner recalls from the Core Rulebook for each class. The
  planner's hypothesis: PCGen adds the `HD:1` "die" (one point per level) on top, so Stamina per
  level = `HD` + `CURRENTMAX` coefficient + Con modifier. **This is unverified.** A reader that
  takes `CURRENTMAX` alone would print a Stamina total short by one per level. That is exactly the
  confidently-wrong shape AGENTS.md rule 7 describes. E3.3 must settle it against the SRD's class
  tables and a PCGen SF oracle run before any Stamina number ships.
- `COMBAT|AC` is split by `TYPE=EAC_Armor` / `TYPE=KAC_Armor`, with `SPROP:EAC` in prose.
- `FACT:KeyAbilityScore` holds strings such as `Str or Dex`, `CHA`, `WIS`, `INT` (choice vs fixed).
- **Resolve** has no direct row. E4.1 hand-transcribes the formula from the SRD (§18).

**Decision.** SF gets a **separate** mapping table (proposed path
`artifacts/epic_3/token-mapping/sf-mapping-table.v1.json`; E3.3 fixes the real path). Every PCGen
field whose SF meaning differs from its PF meaning gets one row, and each row cites two things:
the SRD rule (URL + section), and an oracle observation (a PCGen SF run's output for a named build).
A field with no oracle row is a **named refusal**, never a guess. Planted mutations prove the
mapping is load-bearing: swap `ALTHP`↔`CURRENTMAX` and the seed Stamina/HP fixtures must go red.

**Enforced by:** E3.3's acceptance; the structural-diff protocol on the SF package (`§14`).

**Revisit if the operator disagrees:** none expected. This is a correctness control.

---

## §9 — Seed characters

**Operator-away default (2026-10-02).** The merge-check roster is fixed:

- **PF:** Aldric (Fighter 3), Elowen (Wizard 5).
- **SF:** a Soldier 3, a Mystic 5 (spells), a Technomancer 5 (spells), an Envoy 3 (skill-heavy).

Builds are in `content-unit-inventory.md §4`. Every seed must open in the **real desktop app**
under an isolated `XDG_DATA_HOME`. The harness refuses to start on the real root, and its receipt
states the real store's entry count and sha256 before and after (SD-36 lesson 18). Every status
message reports **per-seed sheet deltas** (SD-36 lesson 20).

**Enforced by:** E6.6 and E7.1; `apps/desktop/scripts/ui-smoke/run.mjs` (isolated root);
`workflow-instruction.md` rules R5 and R7.

**Revisit if the operator disagrees:** replace any seed in `content-unit-inventory.md §4`. E0.4
re-transcribes that seed's hand values.

---

## §10 — Adoptions from predecessors: C2.1 adopted; FS rows are candidates, not gates

**Operator-away default (2026-10-02).**

- **SD-35 C2.1 ("the second PCGen-format reader")** was addressed to SD-36
  (`docs/release/SD-35-corpus-sheet-completion/forward-scope-register.md`, row C2.1). SD-36 has no
  C2.1 row: `grep -n 'C2.1' docs/release/SD-36-consolidation/forward-scope-register.md` returns
  nothing. **SD-37 adopts it explicitly as E3** (E3.1 include structure, E3.2 formula-system
  vocabulary). It is in SD-37's Definition of Done.
- **Registered as candidates in `forward-scope-register.md` (not gates):** the 8 open SD-34 P1s
  (SD-36 FS-2; count quoted, not re-derived), SD-36 FS-15, FS-27 and FS-28, and the SD-36 Epic F
  next-week list (Half-Orc/Half-Elf free +2, armour check penalty on Dex skills). Each one has a
  named revisit condition. None was in SD-37's Definition of Done at scoping, so none is a blocker
  (`docs/governance/deferral-revisit-doctrine.md`).
- FS-2's own text says "should land before Starfinder bundle". That is a soft gate. The
  operator-away default is that **it does not gate**, because the operator's request (Starfinder)
  does not include it. This is flagged.

**Enforced by:** `forward-scope-register.md` rows; the E7.3 closure scan checks every revisit
condition.

**Revisit if the operator disagrees:** promote any candidate to a card. It then becomes part of
the Definition of Done.

---

## §11 — Model tiering

**Operator-away default (2026-10-02),** per `~/.claude/CLAUDE.md` (updated 2026-09-29). That file
supersedes the Sonnet-first guidance in SD-36's `workflow-instruction.md` and in memory
`model-selection-tiering`.

| Work | Model |
|---|---|
| Build, implementation, debugging, review, **merge checks** | **Opus 5.5** (`model: 'opus'`) |
| Well-specified mechanical work (import re-pointing by a fixed recipe, generated-file regeneration, long-run verify waits) | **Sonnet 5.5** (`model: 'sonnet'`) |
| Housekeeping (version bump, worktree/branch sweep, PR open) | **Haiku 4.5** (`model: 'haiku'`) |

- Merge checks are **never** dropped to a cheaper tier. Under quota pressure, narrow their scope,
  never their model (SD-36 lesson 16).
- Long-run waits (anything that may run 20+ minutes) run on Sonnet or better. Never Haiku (SD-36
  lesson 19). A result of `green:false` with an empty failing list means "not finished", never
  "red".
- **Every** `agent()` sets `model` explicitly. An omitted `model` inherits the orchestrator's
  model.
- **Planning disclosure.** This package was authored by Sonnet 5.5. The operator's tiering puts
  planning on Fable 5.1 or Opus, but switching the planning model was not available in the
  authoring session. **The operator may want a Fable/Opus review pass of this package before
  launch.** Card C0.2 (Opus) is that review, and the launch waits for it unless the operator waives
  it.

**Enforced by:** the Workflow script's model map (`workflow-instruction.md §2.4`). A reviewer greps
the script: `awk '/agent\(/ && !/model:/' <script>` must print nothing.

**Revisit if the operator disagrees:** edit the tier column in `epic-breakdown.md §0`. The script
reads that column.

---

## §12 — Unattended-mode protocol (reconciled with blocker discipline)

**Operator directive on record (2026-08-01, verbatim):** *"include instructions to all 3 that
indicate they will be running in unnattended mode since i will be out of town while this runs.
They may not stop to ask questions - it might be days before i notice."* The operator is away for
this bundle (brief of 2026-10-02). The protocol appears in three places: the
`workflow-instruction.md` OPERATING METHOD callout, this decision, and `progress.md` Cycle 0.

**The reconciliation.** The chassis doctrine (`unattended-mode-doctrine.md` rule 3) says "the
bundle does not halt". AGENTS.md "Blocker Discipline" and `docs/governance/blocker-closure-doctrine.md`
say "an open blocker pauses the work". **Both hold, at different granularity:**

1. **Clear first.** A blocker on the Definition of Done is decomposed and attacked. Most SD-36
   blockers were sequencing problems that one converter step cleared.
2. **If only an operator ruling can clear it,** mark **only that card** `blocked-escalated`. Write
   the exact ruling, write scope or precondition needed under `progress.md ## Open blockers`, with
   the command and its output. Pause that card and its dependents. **Continue every independent
   card.**
3. **Never open the final PR while any card is open, partial or blocked-escalated.** An unattended
   run can end at "every independent card done, blocked cards listed". That is a correct stop, not
   a failure.
4. **`decision-blocked`** is for choices inside the scope where a safe default exists (§12.1):
   take the default, log it in `progress.md ## Decisions taken on safe defaults` together with the
   alternative not taken, and continue. It never leaves Definition-of-Done scope undone.
5. **No `AskUserQuestion` and no `clarify`.** Problems surface only in `progress.md`, cycle
   receipts and the end-of-turn status.

### §12.1 Safe defaults (written in advance)

| # | Decision point | Safe default |
|---|---|---|
| SD-a | Licence/PI unclear for a book or record | Exclude and log. Do not ingest (§6). |
| SD-b | Compute vs print for a unit | Print unless the value feeds a sheet total (§5). |
| SD-c | A rule the oracle lacks (Resolve, any formula with no row) | Hand-transcribe from the SRD with URL + section and an Opus review (§18). If the SRD is unreachable, the card is `blocked-escalated`. |
| SD-d | A PCGen field with no SF oracle row | Named refusal, never a guess (§8). |
| SD-e | `rules_tables` data-package format | JSON, one file per table, shape = the existing serde types, schema published under `schemas/rules/` (§19). |
| SD-f | Starship/vehicle content met in records | Print its prose where the record exists (gear, spells, abilities). Build no starship sheet (§17). |
| SD-g | A seed build the oracle cannot express | Swap in another Core Rulebook option of the same class and log it (§9). |
| SD-h | A `(kind, slug)` collision across systems | Namespace by system id. Never merge records across systems. |
| SD-i | A PF render byte-diff after E1/E4a | A defect. Fix it. **Never re-baseline PF** to make a diff pass. |
| SD-j | A worktree, branch or file that another session may own | Do not touch it. Log it. |
| SD-k | PR #395 merges before C1 | Rebase, then bump (§2). |
| SD-l | Weekly quota at or above the stop threshold | Stop dispatching and write a resume receipt (§12.3). Do not degrade the model. |
| SD-m | SF archetypes (COM) | Print their class-feature replacements at the stated levels. Compute only what feeds a total. |

### §12.2 Watchdog: one long Workflow run, not a cron

**Decision:** the bundle runs as **one long `Workflow` run** with a recorded run id. Recovery uses
`resumeFromRunId`. There is no hourly cron.
**Why (two lines):** a Workflow run carries the phase map, the explicit per-agent models and the
`ownedBy` step graph, so a resume replays nothing that already ran. A cron can only start fresh
sessions, and installing one is itself a recorded hazard (memory
`never-install-a-crontab-from-an-unverified-read`).
The run id and the script path are written into `progress.md` at launch. The run's own step
timeouts act as the stall detector.

### §12.3 Quota stop rule

Stop dispatching **new** lanes when either condition holds:

- (a) a quota reading less than 6 hours old shows weekly all-models usage **≥ 85%**; or
- (b) cumulative subagent tokens since the last weekly reset reach **10 M**. This threshold is an
  **estimate** scaled from SD-36 Epic F (≈10.7 M tokens at 87% weekly usage, quoted).

Lanes already in flight finish. Then write a **resume receipt** in `progress.md`: run id, the last
completed step, the unrun steps, and the reset time. Resume after the reset. **Never** downgrade a
merge check to stay under the threshold.

`site/dashboard/PF1e-dashboard.json`'s `usage` block is **frozen**: `captured_at` is
`2026-09-14T17:45:07Z`, and its producer was retired by SD-36 Epic B. It is **not** a quota
source.

### §12.4 Crash-resume recipe (VM stop or OOM)

1. Keep every dirty worktree. Do not reset, stash or clean.
2. Find the run id in `progress.md` and inspect `journal.jsonl` for the last completed step.
3. Patch **only the unrun prompts**. `diff` the script's shared prefix against its backup copy
   (`artifacts/cycle_0/workflow-prefix.backup.js`, written at launch); the prefix must be
   byte-identical.
4. Resume with `resumeFromRunId`.
5. Log a `retro.py incident`.

### §12.5 Environment guards (in every dispatch prefix)

These guards go in every dispatch prefix: MEMORY GUARD (one `cargo` per lane, `-j 8`, every
`cargo test … -- --test-threads=8`, `free -g` before a long run); `RETRO_ACTOR=<role>`;
`CARGO_TARGET_DIR` set per agent **per source tree**, never under `/tmp`, claimed with
`mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"`, and deleted on
finish; `df -h /` before any full sweep (a sweep needs about 24 G); at most **3** concurrent cargo
lanes; `nohup` + pid file + poll for anything past 10 minutes, with a not-run skeleton results
file; an explicit `PATH` export that includes `~/.cargo/bin`; cited logs copied from tmpfs into the
repo before the step returns. The full prefix text is in `workflow-instruction.md §2.1`.

**Enforced by:** `workflow-instruction.md` OPERATING METHOD callout and §2.1; `progress.md` Cycle 0.

**Revisit if the operator disagrees:** the operator may lower the quota threshold or switch the
watchdog to a cron. A cron install must count crontab jobs before and after.

---

## §13 — Closure order (operator-pinned) — and the template's defect

**Operator ruling (standing, 2026-09-14) applied:** all cards `complete` → retrospective written
and cited → full worktree/branch sweep → **release notes** → **architecture truth-up** →
**graphify LAST**, over a tree where unfiltered `git status --porcelain` prints nothing and HEAD
equals `origin/tranche/17` → **PR as the final action**. The operator merges.

**Template defect, flagged and not fixed here.** `docs/governance/workflow-instruction-template.md
§11` puts release notes at step 5, *after* graphify and the PR. That indexes a tree that never
ships. This bundle's `workflow-instruction.md §11` uses the corrected order. **The template needs
the same fix.** This package does not edit the template, because that is outside its write scope.
The fix is listed as candidate FSR-C9.

**Enforced by:** `epic-breakdown.md` E7.3–E7.9 dependencies; E7.8's receipt records the SHA that
graphify indexed and the empty `git status --porcelain`.

---

## §14 — Per-cycle discipline

**Operator-away default (2026-10-02), from standing doctrine.** Every code-bearing cycle runs:

- TDD (RED confirmed for the intended reason, then GREEN).
- The dual audit: the identifier audit and the four-check wired-integration audit from
  `docs/governance/no-stub-mvp-doctrine.md §"Per-cycle audit"`, with `OK_NO_TOKENS`,
  `OK_NO_NOOP_HANDLERS`, `OK_NO_MOCK_LEAKS` and `OK_NO_WOULD_STRINGS` pasted into the receipt.
- A receipt at `artifacts/epic_<n>/<cycle>_cycle_receipt.md` carrying figures + commands +
  denominators, raw row-count output, the build scope verified, the commit SHA and the oracle SHA.
- The three-status vocabulary: `complete | partial | blocked-escalated`.
- `retro.py` events emitted when they happen.
- Per-seed sheet deltas.

Converter cycles add the **structural-diff protocol**: pinned delta classes, planted mutations
that must fail, records unmoved where unmoved is the claim, `sheet_rule_convert -- --check` exit 0,
`pcgen_residue_gate.py --check --closure` PASS. Batching follows the batch-big rule: all of a
homogeneous remainder goes into one dispatch. There is one verify per wave, and no clean gate is
repeated. **Clean tree = unfiltered `git status --porcelain` empty** at every wave commit.

**Enforced by:** `workflow-instruction.md §6–§7`.

---

## §15 — SD-36 Epic F process lessons 1–7 are binding rules

Each lesson in memory `next-bundle-changes-from-sd36-epic-f` (retrospective lessons 14–20) is a
named, checkable rule, **R1–R7**, in `workflow-instruction.md §12`: converter lane in every
prefix; no hand-kept desktop tables; Opus merge checks never dropped; `ownedBy` declines; isolated
app-data roots; long waits on Sonnet or better; per-character status. Each rule names the command
or artifact that checks it.

---

## §16 — Acceptance must be falsifiable per criterion

Every criterion in `epic-breakdown.md` carries a command whose output can fail. Every criterion
that rests on a proof states what that proof does **not** cover (AGENTS.md rule 7).
`acceptance-and-verification.md` holds the bundle-level gates: PF byte-identical hash set, SF seed
hand values, widest-scope `cargo test` in both workspaces, and SF oracle parity with the sparse-path
fix proven in a fresh clone.

---

## §17 — Starship is a planned capability deferral (E8), not a blocker

**Operator-away default (2026-10-02).** There is no starship sheet in SD-37.

- **No oracle data exists.** The Core Rulebook PCC's starship add-on is commented out
  (`_starfinder_core_rulebook.pcc`, the `#ABILITY:starship/…` and `#EQUIPMENT:starship/…` lines),
  and `paizo/core/starship/` does not exist (`ls $S/paizo/core | grep -c starship` → 0).
- **The blocker-vs-deferral test** (`docs/governance/deferral-revisit-doctrine.md`): *was this
  scope in the Definition of Done when the work was scoped?* The operator asked for Starfinder 1e
  character sheets and did not ask for starships, so starship was **not** in the Definition of
  Done. That makes it a planned capability deferral, not a blocker. This is a real test of the
  line: the deferral is legitimate only because the scope was never in the DoD. If the operator
  says starships were meant, then it becomes a blocker and the bundle is not done without it.
- **Revisit condition (checked, not remembered):** at the E7.3 closure scan, and at every tranche
  cut, run
  `test -d "$PCGEN_REPO_DIR/data/starfinder/paizo/core/starship" || grep -lE '^ABILITY:starship|^EQUIPMENT:starship' "$PCGEN_REPO_DIR"/data/starfinder/paizo/core/*.pcc`.
  Any output means oracle data now exists, and the deferral is re-opened for an operator ruling.
  The row is FSR DEF-1.

---

## §18 — Seed hand values come from the Starfinder Reference Document, not the `.lst`

**Operator-away default (2026-10-02).** Fixtures must be transcribed from bytes the engine does not
read (STC-Skill-Creation Rule 6). The machine holds no Starfinder book: `find /home/ubuntu
-maxdepth 5 -iname '*.pdf'` → 0 (source-inventory report §2). So the hand values come from the
**Starfinder Reference Document** (the OGL text, published online). E0.4 transcribes every value
the seeds need: class progression rows (BAB, saves, HP, Stamina, skill ranks), race HP, armour
EAC/KAC and max Dex, theme ability bonus, skill rules, and the Resolve formula. Each value cites a
URL and a section. An Opus agent reviews the transcription before any fixture uses it.

**Safe default if the SRD cannot be reached** (no network, site down): E0.4 is
`blocked-escalated`, and so are the fixture-dependent criteria in E4/E7. Independent cards
continue.

**Enforced by:** `artifacts/epic_0/seed-hand-values.md`, where every row has a URL. A test fails if
any fixture value lacks a `source:` URL.

---

## §19 — `rules_tables` → data package (E4a): in scope, fenced, parallel

**Operator-away default (2026-10-02),** from memory `rules-tables-stay-rust-until-starfinder` and
SD-36 ruling 7 ("no `rules_tables` move before Starfinder"; this is the Starfinder bundle).

- **Scope:** move `src/rules_core/rules_tables/**` (180,883 lines, 250 files; CUI F-10) into a
  runtime data package. That needs a loader, a schema (`schemas/rules/`), a bundle path,
  licence/PI stamping, and re-pointing all 252 importing files (CUI F-12).
- **It carries the `.lst` citation burn-down** (SD-36 D6): 12,529 lines containing `.lst` (CUI
  F-11; predicate stated there). **SD-36's own D6 status must be re-derived before adoption.** The
  `lst_file` identifier half of D6 is present in the residue gate
  (`grep -n 'lst_file' scripts/pcgen_residue_gate.py` → lines 223, 229). The burn-down half is
  open. E4a.3's first step re-derives both halves and records them.
- **Runs in its own worktree** with its own `CARGO_TARGET_DIR`, after E1 lands, in parallel with
  E3–E6. It is **not** on SF's critical path: SF sheet output does not need it.
- **File-level fences:** E4a never edits `src/rules_core/corpus_loader.rs` (owned by E1, then
  read-only), `src/rules_core/sheet_rule.rs` (E2), or anything under `pilot_compute/` that E4
  owns. The 16 desktop `src-tauri` importers belong to E4a until E4a.2 merges, and E6 edits them
  only after that. The fence table is in `workflow-instruction.md §3`.
- **Gate:** PF byte-identical renders of Aldric and Elowen, plus byte-identical catalog output, at
  E4a.4. **Bestiary 1's monsters exist only in `rules_tables`** (memory; not re-derived), so they
  must survive the move. E4a.4 counts them before and after.

**Revisit if the operator disagrees:** E4a can split into its own SD-N without changing E0–E7.
That is the one scope cut this bundle names in advance. Only the operator can make it.
