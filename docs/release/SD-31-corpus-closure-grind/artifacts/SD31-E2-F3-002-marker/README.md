# `SD31-E2-F3-002-marker` — the `ambiguous` dead-end evidence pack

Supporting data for `progress.md`'s `SD31-E2-F3-002-marker` receipt and `OPEN-ISSUES.md` rows
235–239. Generated at commit `4b9262fb8`, oracle pin
`PCGEN_ORACLE_SHA=7f818006e371188e5717fd18d74d18a420747fc6`.

## `ambiguous-candidate-sets.json`

Every one of the 406 `wiring_class == "ambiguous"` units (`beginner_box` excluded, as the live
producer excludes it), with the fields the operator's ruling on rows 236/237 turns on:

| field | meaning |
|---|---|
| `verdict_today` | replayed through `pf1e_dashboard_producer.doneness_verdict()`, never asserted |
| `wiring_class_signals` | the signal set `closure_signals` emitted for the unit's token closure |
| `candidate_classes` | the classes the signal set actually leaves open — `{static, derived}` when the closure carries a magnitude token (`static:literal_magnitudes_only`), `{display, derived}` when it carries none (`display:no_magnitude_token`). `computed` is never a candidate: a `computed:` signal would have won in `classify()` before `ambiguous` was reached. |
| `candidate_rows_identical` | whether both candidates map to the SAME row of `doneness_verdict` — true exactly for `{static, derived}`, which is why the ambiguity is immaterial to the verdict for those units and material for the others |
| `corpus_literal_sweep_verified` | whether `corpus_literal_sweep --json-out` (CLEAN this run: 25,688 examined / 25,628 verified, exit 0) byte-verified this unit's shipped record |
| `in_refused_80` | the 80 units that a one-arm change to `apply_done_rung_stamps` would move `held → done` — **measured and refused this cycle**, see `OPEN-ISSUES.md` row 236 |

Totals carried in the file's own `totals` block: 406 ambiguous, 304 `{display, derived}`,
102 `{static, derived}`, 169 evidence-bearing, 80 refused.

## The lower-bound table the refusal rests on

Every concrete class's verdict at every evidence status, and the minimum over each candidate set.
Reproduce by calling `doneness_verdict(wc, status, kind)` directly for each cell:

```
status                 display        static         derived       computed     | ambiguous(today) | lb{display,derived} | lb{static,derived}
grounded                held           held           held           done       |       held       |        held        |        held
text-complete           done           held           held        in-progress   |       held       |        held        |        held
ingested-magnitude   in-progress       held           held        in-progress   |       held       |        held        |        held
literal-verified     in-progress       done           done        in-progress   |       held       |    in-progress     |        done
fixture-verified     in-progress       done           done        in-progress   |       held       |    in-progress     |        done
```

There is **no status at which every candidate class bottoms out at `done`** for the
`{display, derived}` set — 304 of the 406 units — so `AT-31-010`'s requirement that `ambiguous`
reach `done` from at least one status cannot be met for them by any verdict-path change that keeps
the lower-bound rule. That is the contradiction with Decision 1(e) item 4 logged as row 237.

## What this pack does NOT contain

No proposed patch. This cycle deliberately did not write the `WiringClass::Ambiguous` arm that
would bank the 80 units; the operator's ruling on row 236 decides whether it is written at all.
