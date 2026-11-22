# Criterion 26 — Architecture truth-up cycle (cycle 12)

`architecture_truth_up.py` run against `origin/develop` (integration target), bundle SD-23, receipts at `docs/release/SD-23-character-mutation-and-wired-integration/receipts.md`.

## Real gap found and fixed first

The script's cited-path existence check initially failed: `MISSING: src/testSupport/makeSurface.ts`. This was **not** an SD-23-introduced defect — it's a self-inflicted, pre-existing false positive in `docs/architecture/README.md`'s own maintenance-contract text. The doc explicitly documented that its own illustrative "abbreviated citation" sentence names `src/testSupport/makeSurface.ts` in deliberately abbreviated form and calls that hit "the one expected, permanent exception... not a doc defect" — but wrote it as a bare, checker-matchable path instead of using the placeholder convention (`<xyz>`) the checker's own exclusion regex is documented to skip.

Fixed at the root: reworded the illustrative sentence to `<src-dir>/testSupport/makeSurface.ts`, using the documented placeholder pattern so the checker's own exclusion logic (`grep -vE '[* <]'`) correctly skips it — turning a standing manual exception into a real pass, not a worked-around one. Committed separately (`d4b7523`) before re-running the script.

## Script run (real, not dry-run)

```
[truth-up] working tree: clean
[truth-up] branch: d4b75238
[truth-up] integration target: develop
[truth-up] diff path count: 438 (in arch scope: 424, out of scope: 14)
[truth-up] docs touched: none — no architecture impact
[truth-up] verification: cited-path check + relative-link check both pass
[truth-up] appended receipt to receipts.md
[truth-up] architecture truth-up complete
```

- **Docs touched:** none — SD-23's diff (Epics 1-6, all TypeScript/Rust under `apps/desktop/` and `src/`) doesn't fall under any `docs/architecture/*.md` doc's `Source dirs` scope requiring an edit. This is an expected, legitimate "no architecture impact" outcome, not a skipped check.
- **Stub graduations / regressions:** none.
- **Verification one-liners:** both pass (after the fix above).
- **Receipt:** appended to `receipts.md` at cycle_id `2026-07-21T03:31:09Z`, `row_or_kind: architecture:truth_up`.

Commit SHA (truth-up receipt): `3c1e004`. Commit SHA (README false-positive fix): `d4b7523`.
