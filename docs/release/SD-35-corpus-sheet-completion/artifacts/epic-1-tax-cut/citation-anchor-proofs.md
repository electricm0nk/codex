# Citation-anchor proofs — AT-35-E1-002 cycle 1

The RED→GREEN evidence `epic-breakdown.md` AT-35-E1-002 names, run for real on the live
`src/bin/v06_work_inventory.rs` at `53296d80f0` (31,871 lines) and restored with
`git checkout -- src/bin/v06_work_inventory.rs` afterwards. Every block below is pasted
command output.

Re-derive the whole transcript: the mutator and the check runner are two-file scripts (kept
out of the tree — they mutate the live engine source); the equivalent permanent proofs run on
synthetic source text in `scripts/tests/test_shape_engine_boundary.py`
(`TestContentAnchorRedGreen`), `scripts/tests/test_completion_atlas.py`
(`TestResolveContentAnchorRedGreen`) and `scripts/tests/test_missing_engine_tables.py`
(`TestContentAnchorRedGreen`) — `scripts/verify.sh --only shape-engine-boundary-selftest`.

## 0. Baseline — unmutated file, all three green

```
## completion_atlas.py --check
population=49438 buckets=10 unclassified=0 overlap=0
citation_failures=0
ATLAS_EXIT=0
## shape_engine_boundary.py --check
magnitude_bearing=26396 not_held_by_engine=8784 citation_ok=True
SEB_EXIT=0
## missing_engine_tables.py --check
population=449 kinds=2
citation_failures=0
MET_EXIT=0
## resolved lines (derived by search at check time, never pinned)
atlas {'DONE': 13147, 'A': 16442, 'B': 16041, 'C': 16347, 'D': 13375, 'M': 13158, 'V': 17281, 'U': 13463, 'X': 13423, 'Z': 13284}
seb 16271 16274 None
met {'companion': 16442, 'power': 16542}
```

## 1. GREEN under a move — 50-line comment block inserted above `fn simple_kind_verdict`

Every cited site sits below the insertion point, so every one moved by exactly +50. All three
`--check`s stayed green and every resolved line reads +50:

```
mutated: move
## completion_atlas.py --check
population=49438 buckets=10 unclassified=0 overlap=0
citation_failures=0
ATLAS_EXIT=0
## shape_engine_boundary.py --check
magnitude_bearing=26396 not_held_by_engine=8784 citation_ok=True
SEB_EXIT=0
## missing_engine_tables.py --check
population=449 kinds=2
citation_failures=0
MET_EXIT=0
## resolved lines
atlas {'DONE': 13197, 'A': 16492, 'B': 16091, 'C': 16397, 'D': 13425, 'M': 13208, 'V': 17331, 'U': 13513, 'X': 13473, 'Z': 13334}
seb 16321 16324 None
met {'companion': 16492, 'power': 16592}
```

Under the retired `file:line` pins this same move would have failed all 16 pins (10 atlas, 4
ladder lines, 2 engine-surface arms) — the failure shape SD-34 waves 32–51 re-derived by hand
eleven times.

## 2. RED under a content change — one cited condition changed per instrument, on top of the move

Three edits: the ladder's fourth condition `class_feature_pool_catalog_holds` →
`class_feature_pool_catalog_might_hold` (shape_engine_boundary); the `Kind::Companion` arm's
marker `companion_content_has_no_engine_table` → `companion_content_has_a_table_now`
(missing_engine_tables, and the atlas's bucket A which cites the same arm); bucket Z's
evidence `no_compiled_rule_set_for_book` → `no_compiled_rule_set_for_this_book`
(completion_atlas). All three fail closed, each naming the function and the exact lines:

```
mutated: break-all
## completion_atlas.py --check
population=49438 buckets=10 unclassified=0 overlap=0
citation_failures=2
  citation_failure: A: src/bin/v06_work_inventory.rs: fn classify no longer contains ['Kind::Companion => engine_does_not_hold("companion_content_has_no_engine_table"),']
  citation_failure: Z: src/bin/v06_work_inventory.rs: fn classify no longer contains ['status: "not-started",', 'evidence: "no_compiled_rule_set_for_book".to_string(),']
ATLAS_EXIT=1
## shape_engine_boundary.py --check
STALE_CITATION: promotion-ladder citation no longer resolves at HEAD: src/bin/v06_work_inventory.rs: fn classify no longer contains ['if has_real_description', '&& is_display_wiring_class_for_promotion(wc_class)', '&& !universal_sheet_modifier', '&& facts.class_feature_pool_catalog_holds(&unit.source_book, &unit.key)']
SEB_EXIT=1
## missing_engine_tables.py --check
population=449 kinds=2
citation_failures=1
  citation_failure: companion: src/bin/v06_work_inventory.rs: fn classify no longer contains ['Kind::Companion => engine_does_not_hold("companion_content_has_no_engine_table"),']
MET_EXIT=1
## resolved lines
atlas {'DONE': 13197, 'A': None, 'B': 16091, 'C': 16397, 'D': 13425, 'M': 13208, 'V': 17331, 'U': 13513, 'X': 13473, 'Z': None}
seb None None src/bin/v06_work_inventory.rs: fn classify no longer contains [...]
met {'companion': None, 'power': 16592}
```

The new `verify.sh` stages fail on the same file:

```
    FAIL  shape-engine-boundary  (exit 1 — stale citation: promotion-ladder citation no longer resolves at HEAD: src/bin/v06_work_inventory.rs: fn classify no longer contains [...] — /tmp/codex-verify-bshwvf/shape-engine-boundary.log)
    FAIL  missing-engine-tables  (exit 1 population=449 kinds=2 citation_failures=1 — /tmp/codex-verify-bshwvf/missing-engine-tables.log)
  FAILED:  2  shape-engine-boundary missing-engine-tables
RESULT: FAIL — logs in /tmp/codex-verify-bshwvf
VERIFY_EXIT=1
```

## 3. GREEN again after `git checkout -- src/bin/v06_work_inventory.rs`

Identical to §0 (resolved lines back to 13147 … 17281, 16271–16274, 16442/16542; all exits 0).

## 4. The stages exist and pass

```
$ scripts/verify.sh --list | grep -nE "cycle-scope|shape-engine|missing-engine|pcgen-residue"   # after the rebase onto AT-35-E1-001 and E1-005
21:cycle-scope-gate-selftest yes   yes
22:shape-engine-boundary-selftest yes   yes
23:shape-engine-boundary yes   yes
24:missing-engine-tables yes   yes
27:pcgen-residue-gate   yes   yes
$ scripts/verify.sh --list | tail -n +2 | wc -l
45
$ scripts/verify.sh --only shape-engine-boundary-selftest --only shape-engine-boundary --only missing-engine-tables
    PASS  shape-engine-boundary-selftest  (15 cases passed)
    PASS  shape-engine-boundary  (magnitude_bearing=26396 not_held_by_engine=8784 citation_ok=True)
    PASS  missing-engine-tables  (population=449 kinds=2 citation_failures=0)
RESULT: PASS
VERIFY_EXIT=0
```

## 5. Anchor semantics (what "content anchor" means here)

`completion_atlas.resolve_content_anchor({file, context_fn, anchor})`: `anchor` is one line or
a list of consecutive lines, compared after stripping indentation; it must occur **exactly
once** inside the body of `fn context_fn` (body = from the definition line to the first `}` at
the definition's own indentation — rustfmt's invariant). Zero matches, two or more matches
(`ambiguous, lengthen the anchor`), a missing function, or a missing file all fail closed. The
resolved line is reported (`resolved_line` in the atlas and tables artifacts,
`promotion_ladder_anchor_line` in the boundary report) but never stored as an input. Multi-line
anchors are the minimum unique inside the named function — e.g. `status: "grounded",` occurs
twice in `simple_kind_verdict`, so DONE's anchor carries the `if let Some(bonus) =
grounded_magnitude {` line above it, and the promotion ladder needs all four lines because a
sibling block earlier in `classify` shares its first three.
