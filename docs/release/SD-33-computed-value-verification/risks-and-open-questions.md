---
canonical: true
owner: god-emporer
bundle_id: SD-33
date: 2026-08-24
---

# SD-33 Risks and Open Questions

## 1. The bundle's defining risk

**R-1 — Path A may not be achievable, and the fallback is slower.**

Epic 2 asks whether the pinned PCGen can run headless and emit computed values. Three named risks, **none of them tested at authoring time** — they are risks, not findings:

| Risk | Why it might bite |
|---|---|
| Gradle build vs Java 25 | PCGen is an older codebase; the pinned tree may require an earlier JDK |
| `BatchExporter` imports `pcgen.gui2.UIPropertyContext` | GUI coupling inside the intended headless path |
| Batch export consumes **existing** characters | `.pcg` inputs must be authored before anything can be exported |

**Mitigation:** Path B — read PCGen's Java source per shape — is named in advance (`decisions.md §5`) and is **proven in this repo**: it produced SD-32's `MaxCommand.java` finding, which corrected a pinned test that encoded a false assumption about single-argument `max()`.

**The failure mode to guard against is not Path A failing. It is Path A failing quietly**, Epic 5 shrinking to fit, and the bundle becoming "coverage only" without anyone deciding that. AT-33-E2-004 makes the ruling an explicit, escalated operator decision point.

## 2. Self-healable vs non-self-healable

**Self-healable — resolve inline, exit GREEN:**
- dirty tree, single-token audit violation, unrelated test-setup breakage
- build-counter out of sync
- a stale incremental cache serving a stale binary → set `CARGO_INCREMENTAL=0`
- disk pressure → prune merged worktrees (never a `locked` one)

**Non-self-healable — write `## Open blockers`, exit FAIL, pause the bundle:**
- working tree diverged, needing manual rebase
- two live cycles on conflicting files
- a launch gate not actually met
- RED→GREEN not preserved in the receipt
- a stub, inline mock, `success: true` from a fake operation, or a `"Would …"` string in shipping code
- **a disagreement resolved by adjusting the expectation to match our output** (AT-33-E5-003)

## 3. Open questions

**Q-1 — How many kinds have no probe at all?**
Unknown at authoring time. AT-33-E1-003 answers it by execution. The number is load-bearing: kinds with no probe can never leave `unverifiable`, and their unit count sets a **ceiling on G4's reach**. Stating that ceiling honestly is a bundle deliverable, not a footnote.

**Q-2 — Are the 15,022 already-`done` units trustworthy?**
8,330 are blessed by fixture or literal check only. Epic 5 examines those. **The remaining 6,692** (`text-complete` 4,669 + `grounded` 2,023) are not in Epic 5's population: `text-complete` is done by `SD-32-.../decisions.md §7`'s ruling and `grounded` carries a probe. **If Epic 1's box finds either claim does not hold, that is a finding with a count**, escalated — not a silent widening of Epic 5.

**Q-3 — Does closing the engine-coverage gap change the recognition rate?**
The 41% that ran recognised 97.9%. Whether the un-run 59% recognises at the same rate is **unknown and must not be assumed**. A drop is a legitimate finding, not a regression.

**Q-4 — What causes the per-family unevenness?**
F1 sits at 28% coverage and F8 at 21%, while F2 reaches 64%. AT-33-E3-001 requires the cause before anything runs — the unevenness suggests a mechanism, not a sampling accident.

## 4. Known hazards carried from SD-32

Each cost real cycles. All still apply.

- **A shallow glob lies.** `data/corpus/*/*/x.json` found zero where eight existed a level deeper. Use recursive search.
- **`find -newermt` lies on this box.** Agent-file mtimes run ahead of system time. Use a Python mtime comparison or `git status --porcelain`.
- **Omitting `model` on an `agent()` call** inherits the orchestrator's model. One wave burned 97% of a week's Opus quota.
- **A generator's `main()` may `remove_dir_all` a shared output tree.** One SD-32 run wiped 540 files owned by a sibling pipeline. `git status --porcelain` immediately after any generator run.
- **Two lanes at one diagnosis** both fix it; one is discarded on rebase. Name the owner of a *diagnosis*, not only of files.
- **`~/workspace/repos/pcgen` is forbidden** as an oracle path — `preflight-oracle` PASSes against it silently.
- **`apps/desktop/src-tauri` is a separate cargo workspace.** A root sweep does not cover it.

## 5. Standing defects this bundle must not inherit

Tracked because `workflow-instruction.md §12` marks them **UNENFORCED**, and under `decisions.md §4` that marking is a defect to close, not a resting state:

| Row | Lesson | Closing move |
|---|---|---|
| 3 | Dispatch first, report second | make it a receipt field the closure scan checks |
| 8 | Carve-out sweeps grep **code**, not only prose | add a `verify.sh` stage that greps closure-figure scripts for hardcoded exclusion lists |

**Row 8 is how `EXCLUDED_BOOKS = ['beginner_box']` survived every `§27b` prose sweep in SD-32.** Nineteen units — but it sat inside the function every closure figure was computed over, which is the one place a carve-out is invisible.
