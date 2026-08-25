---
canonical: true
owner: god-emporer
status: planning-ready (chassis completed 2026-08-22 from SD-31 session)
date: 2026-08-22
---

# SD-32 Risks and Open Questions

## Primary risks

1. **Gate 0 cannot be fully achieved on every kind.** Some books hold objects PCGen itself does
   not model uniformly. If a category genuinely cannot be enumerated, the gate is met by
   *naming and counting* it (AT-32-G0-002's "kind-unenumerable" handling), not by pretending it
   does not exist. **Mitigation:** AT-32-G0-002 requires a written object-definition rule per
   kind, with explicit "unenumerable" markers where they apply. A cycle that tries to make the
   denominator smaller by silently dropping a kind is out of protocol.
2. **A shape family closes with proof width stated, but the proof is too narrow.** SD-31 wave 21
   passed its own mutation proof, reproduced 64 records exactly, and still fabricated 73.4% of
   its output — the reference set never exercised the shapes it got wrong. **Mitigation:**
   AT-32-G1-003 requires every family to state which shapes its proof does **not** cover, not just
   which it does. AT-32-G2-003 carries the requirement forward into the engines. A cycle that
   emits a "100% covered" line without naming the uncovered shapes is out of protocol.
3. **Gate 2 engines emit values that pass `derived_evaluator_fixture_check` but fail
   downstream.** The fixture check is necessary, not sufficient — it catches fabrication against
   bytes the engine reads but not against corpus semantics the engine does not parse. **Mitigation:**
   AT-32-G2-004 requires the corpus-wide run as a separate cycle with its own receipt, and the
   closure gate (§7's prose bar: "shown to a player, proven on screen with the real driver")
   applies to any engine whose output reaches the desktop app.
4. **A engine's fixture was transcribed from bytes it does read, not bytes it does not.** A
   fixture check that transcribes from the same file the engine reads is not a check — it is a
   mirror. SD-31 §20 authorised the formula interpreter on the explicit condition that the
   expected value is "transcribed from bytes the evaluator never reads." **Mitigation:** the
   verification command for AT-32-G2-002 names `--expected-from` pointing at a corpus-side file
   the engine's read path does not touch. A cycle that writes fixtures without that source is
   out of protocol.
5. **Gate 3's standing gate is added to `scripts/verify.sh` but disabled by a flag.** Standing
   gates that are easy to skip get skipped. **Mitigation:** AT-32-G3-001 requires the gate to
   run on every `scripts/verify.sh` invocation with no opt-out flag. A PR that adds the stage
   with `--skip shape-coverage-standing-gate` is out of protocol.
6. **The Epic 5 protective sweep misses a vulnerable generator.** Three of the twelve SD-31
   checked generators are vulnerable; 17 of the 29 (`ls src/bin/{gen_,ingest_,enrich_}*.rs | wc -l`;
   HANDOFF's "~30" was an estimate) have never been checked. Scaling Gate 2
   over an unchecked generator is the failure mode this epic prevents. **Mitigation:** the
   sweep's first cycle enumerates *every* generator (not just the suspected ones), runs the
   self-erasure assertion against each, and the receipt names the count of generators that
   passed, failed, and were skipped (zero of the last).
7. **~~The build counter is left as a template marker.~~ Resolved 2026-08-22.** The tranche-cut bump
   to `0.12.0` landed on `tranche/12` (SD-31 precedent `147f1c2b7`), the literal is written in every
   site `workflow-instruction.md §11` lists, and the marker no longer appears in the bundle (`§10`
   placeholder gate). Published builds stamp `0.12.<build>`.

## Five footguns from the SD-31 session

Mirrored from `artifacts/HANDOFF.md` for visibility. The full pattern lives in the HANDOFF;
`workflow-instruction.md §9` carries the cycle-procedure-relevant subset. Each of these has bitten
this program in real cycles; a SD-32 cycle that hits any of them is operating on a known landmine,
not a surprise.

1. **Wrong-base worktrees.** Pin the base SHA in every dispatch; `workflow-instruction.md §6` step 1
   is the mechanical check (nonzero exit, `git reset --hard "$PIN"` if wrong). The poison was spent
   site-publish commits; the stale local site branches at the tranche/12 boundary are `site-deploy`
   and `fix/site-deploy-page-workflow` (card 2 dispositions them).
2. **`find -newermt` lies.** Use a Python mtime comparison when freshness matters.
3. **Omitted `model` on `agent()` calls inherits the orchestrator's model.** Set it explicitly
   every time: Sonnet for build and integration, Opus only for adversarial verifiers.
4. **Never `git stash` in this repo.** Use `git show HEAD:<file> > /tmp/<file>` to read a HEAD
   baseline; use a separate worktree if a write baseline is needed.
5. **A ruling is not in force until it is committed.** A cycle that asks a lane to obey a ruling
   that exists only in the orchestrator's working tree is out of protocol.

## Open operator rulings (carried from SD-31 `todo/blocked.md`)

These four items would shrink the honest denominator without changing a line of code. They are
filed here as live operator questions, not bundled into doctrinal closure (`decisions.md §7`).

* **B1** — `mod_only_rescue`: a 249-unit cross-kind phantom-duplicate population that would shrink
  both the `feat` kind and the denominator. Proposed, never ruled.
* **B2** — per-race branch 1/2/3 classification. Race attribution stays frozen until this is
  answered. Affects how SD-32's chassis builds resolve race-trait compound keys (Epic 2 T2b,
  2,472 measured units).
* **B4** — do the 48 structurally-non-PC-class `class` units belong under the class doneness gate
  at all? Monster hit-dice progressions, Eidolon, psionic power-list menus. **A ruling here would
  lower the denominator by 48 units.**
* **B5** — are the 5 `Ex-*` records real classes, or PCGen alignment-violation bookkeeping?
  **A ruling here would lower the denominator by 5 units.**

(No B3: SD-31's B3, "prerequisites in open pools", was closed in SD-31 wave 29 — `decisions.md §7`.)

B4 + B5 together would lower the denominator by 53 units with no engine work. They are the most
leveraged rulings on this list.

## Resolved risks (kept as record, not live risks)

- **~~Plumbing beats rules complexity, by 3.3:1 to 4.4:1, not 4:1–5:1 as first filed.~~**
  Measured in SD-31 wave 31, adversarially reviewed, recorded in `epic-breakdown.md`. The
  correction ran in both directions, which is the lesson — every figure must survive independent
  re-derivation.
- **~~Ten compute families, ceiling 3,201 units (12.8%) — not 4,948 (19.9%) as first filed.~~**
  Same. The single largest family is flat literal constants, 1,747 units, and it gets zero
  benefit from any shared function. A library cannot help with values that do not compute.
- **~~The bulk-ingest thesis (wave 19) and the generic-roster-without-grant-data thesis
  (wave 20).~~** Both refuted. Bulk ingest fails because "not-ingested" does not mean the text is
  missing; generic roster fails because the emission loop is generic but the data it needs is
  not. Neither is retried in SD-32.

## Self-healable vs. non-self-healable split (mirrored from `workflow-instruction.md §8`)

- **Self-healable (resolve inline, exit GREEN):** dirty tree, single-token audit violation,
  unrelated test-setup breakage, build-counter out of sync, `## DISCOVERED` duplicates.
- **Non-self-healable (write `## Open blockers`, exit FAIL):** working tree diverged needing manual
  rebase; two live cycles on conflicting files; a launch-gate dependency not actually merged;
  `## DISCOVERED` queue > 10 entries; RED → GREEN not preserved in the cycle receipt; a cycle
  finds `success: true` from a fake operation, an inline mock in a shipping module, or a "Would
  …" string in shipping code; a Gate 0/1/2/3 verifier that passes on an empty input (gate is
  not "closed-on-empty"); a fixture transcribed from bytes the engine reads.
