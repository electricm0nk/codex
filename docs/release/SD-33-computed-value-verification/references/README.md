---
canonical: true
owner: god-emporer
bundle_id: SD-33
date: 2026-08-24
---

# SD-33 References

Doctrine, skills, sibling bundles, and the retrospectives this package is derived from. Every path is repo-relative.

## 1. Retrospectives — the source of this bundle's decisions

- **`../../../retro/sd32-compute-library-and-cause-closure-retrospective.md`** — the direct predecessor's retrospective, including the **operator retrospective of 2026-08-24** whose four findings become `decisions.md §1`, `§2`, `§4`, `§6` and `§7`. Read the "Operator retrospective" section before any Epic 1 cycle: it explains why `THE-BOX.md` and the denominator gate are build obligations rather than lessons.
- **`../../../retro/sd31-retrospective.md`** — SD-31's retrospective, the source of the seven standing lessons SD-32 transcribed and then ignored. **Read as a case study in why capture is not enough** (`decisions.md §4`), not as a lesson list to re-transcribe.

**This citation is a closure obligation, not a courtesy.** SD-32's chassis review found its own predecessor's retrospective carried into the package's *content* while the *source document* was never linked. AT-33-E6-002 requires SD-33's own retrospective to be cited here in the same cycle that writes it.

## 2. Doctrine of record

- `../../../governance/blocker-closure-doctrine.md` — a blocker on the Definition of Done is **cleared or escalated, never deferred**. Gates `acceptance-and-verification.md §1` and AT-33-E6-001.
- `../../../governance/deferral-revisit-doctrine.md` — the sibling rule for a *planned capability deferral*. The test that separates them: **was this scope in the Definition of Done at launch?**
- `../../../governance/no-stub-mvp-doctrine.md` — enforced by `workflow-instruction.md §6`'s wired-integration grep.
- `../../../doctrine-external/identifier-discipline.md` — enforced by the same section's identifier grep.
- `../../../governance/workflow-instruction-template.md` — the template `workflow-instruction.md` is authored from.
- `../../../governance/STC-Skill-Creation.md` — the interface contract the authoring skill implements.

## 3. Sibling bundles

- `../../SD-32-compute-library-and-cause-closure/` — direct predecessor. SD-33 consumes its closed corpus, its F0–F10 shape families, and its engines. Of particular note:
  - `decisions.md §7` — the zero-magnitude-feature ruling that makes `text-complete` a legitimate `done`
  - `decisions.md §27b` — the no-carve-outs ruling that `EXCLUDED_BOOKS` survived
  - `artifacts/gate-2-engines/formula_interpreter.corpus-wide.json` — the source of SD-33's 41%-coverage figure
  - `forward-scope-register.md` — C2.1 (second PCGen reader) and C3.1 (prose-sourced systems) are explicitly **out of SD-33's scope**
- `../../SD-31-corpus-closure-grind/` — source of the THE-BOX pattern. `artifacts/THE-BOX.md` is the 377-line worked example `decisions.md §1` rebuilds rather than inherits.

## 4. Instruments consumed

- `scripts/shape_ledger.py` — family assignment and join status; consumed unchanged.
- `scripts/coverage_ledger.py` — **not** SD-33's box. Its `not_done_population()` deliberately drops 15,041 units; see `technical-design.md §5` for why it is left alone.
- `scripts/verify.sh` — gains the `denominator-gate` stage.
- `scripts/pcgen-oracle-pin.env` — `PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.
- `scripts/retro.py`, `docs/retro/schema.json` — event logging. **Its `deferrals.open` field is `deferrals[-limit:]`, not open deferrals** (`decisions.md §2`); never quote it as a closure figure.

## 5. Skills

- `../../../../.claude/skills/stc-authoring/SKILL.md` — the skill this package was authored with.
- `.claude/skills/publish-site/`, `.claude/skills/swarm-status-sync/` — project skills, not used by this bundle's dispatch.

## 6. Oracle

- **Repo-local slot:** `../../SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen`
- **Pin:** `7f818006e371188e5717fd18d74d18a420747fc6` (upstream commit date 2026-06-17)
- **Sparse cone:** `data/pathfinder`, `system/gameModes/Pathfinder` — but all `.java` files are readable via `git show HEAD:<path>` without widening it
- **`~/workspace/repos/pcgen` is forbidden.** `scripts/fetch-pcgen-oracle.sh`'s default `--dest` resolves there and `preflight-oracle` PASSes against it **silently**.
