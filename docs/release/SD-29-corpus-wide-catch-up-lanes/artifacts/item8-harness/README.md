# item8-harness — on-screen verification harness evidence

Proof runs for the DoD item-8 harness
(`apps/desktop/.claude/skills/run-desktop/verify-on-screen.sh`), driven
against the live desktop app on tranche/9. Each verified record leaves a
`<slug>.png` screenshot plus a machine-verdicted `<slug>.verify.md` report;
failure-mode proofs live under `failure-modes/` and are deliberately named
`*.FAILED.*` — that naming is the harness's own output and is the guarantee
a failing run can never be cited as passing evidence.

## Proof runs in this directory (2026-08-11, live app on tranche/9)

- `race-trait-mc-duergar-ironskinned.{png,verify.md}` — PASS. Family
  `race_trait`, record `Ironskinned` (Duergar, Monster Codex), via the
  Alternate racial traits tab + Duergar chip; expects `ironskin once per
  day` and `Duergar` found in the rendered text.
- `monster-b1-ankheg.{png,verify.md}` — PASS. Family `monster`, record
  `Ankheg` (Bestiary 1), search-filtered to "1 matching monster."; expects
  `CR 3` and `Bestiary 1 p.15`.
- `failure-modes/fm1-nonexistent-record.FAILED.verify.md` — record absent
  from the corpus → exit 1, no counter/no pass artifact.
- `failure-modes/fm2-wrong-expect.FAILED.{png,verify.md}` — record rendered
  but expected value (`CR 17`) not on screen → exit 1.
- Also proven, artifact-free by design: `RUN_DESKTOP_AGENT` unset/`default`
  → refusal, exit 2; zero `--expect` strings → refusal, exit 2.

Lane cycles should write their own evidence to
`docs/release/SD-29-corpus-wide-catch-up-lanes/artifacts/<cycle>/item8/`
via the `--out` flag; this directory holds only the harness's own proof
runs. Invocation documented in the run-desktop `SKILL.md` §"On-screen
verification (DoD item 8)".
