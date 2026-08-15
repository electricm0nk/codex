---
canonical: true
owner: god-emporer
status: planning-ready — not yet executed
date: 2026-08-11
canonical_branch: tranche/11
build_version_target: 0.11.<build>
---

# SD-33 Release Notes

> **Status: not yet executed.** This file is the closure surface; it is filled in at Epic 9.
> Nothing below the line has shipped.

## Planned headline

Codex can open a PCGen character file.

Until now Codex could read PCGen's *data* (`.lst`/`.pcc`) but not PCGen's *characters*. A user
with an existing PF1e character had no path in except retyping it. SD-33 builds that path, with
one rule that shapes everything else: **an import either brings your character across intact, or
tells you exactly what it could not bring and refuses.** No silent losses.

## To be filled at closure

- [ ] Final build version.
- [ ] Both vendored fixtures at oracle parity — dimensions compared, divergences and their causes.
- [ ] Fidelity-report coverage: which token kinds resolve, which are declared unsupported.
- [ ] `AC-9` partition proof: `git diff --name-only develop...tranche/11`.
- [ ] Test-count delta, reconciled once per `TR-31-003`.
- [ ] Architecture-docs refresh (template §6).
- [ ] Forward-scope entries actually deferred, versus closed in flight.

## Known limitations to state plainly at closure

Drafted now so closure does not quietly omit them:

- Fixture coverage is two level-1 Human CRB characters (`R-2`). Correctness on multiclass,
  non-Human, high-level, or non-CRB characters is bounded by the fidelity report's honesty, not
  demonstrated by test.
- Templates are unsupported (`decisions.md §8`).
- Import only — no export.
- Content SD-29 and SD-30 have not yet ingested will surface as `RecordNotIngested`. That is the
  designed behaviour, not a defect, and it improves as those bundles land.
