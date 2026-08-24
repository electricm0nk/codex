# Cycle receipt — sd32-site-kind-labels

RETRO_ACTOR: sd32-site-kind-labels
Branch: tranche/12 (worktree `worktree-wf_c05a8b48-f61-2`)
Territory: `scripts/site/build_public_status.py`, `site/status-data.json`, `scripts/site/**`, added tests

## Defect

A sibling lane regenerated `site/dashboard/PF1e-dashboard.json` because the
committed shard index was genuinely stale — 11 unit kinds committed, 8 more
now classified in the live corpus. `scripts/site/build_public_status.py`'s
second stage (`load_units_by_kind`) then crashed with:

```
KeyError: unit kind 'ability' has no curated label in KIND_LABELS
```

`site/status-data.json` was never written (crash predates the write), so
nothing was corrupted — but the public status pipeline was broken
end-to-end.

## Step 1 — enumerate the real live kind set (do not assume it's just `ability`)

Command:

```
python3 -c "
import json, pathlib
d = pathlib.Path('site/dashboard/units')
kinds = set()
for p in sorted(d.glob('PF1e-units-*.json')):
    kinds.add(json.loads(p.read_text())['kind'])
print(sorted(kinds)); print(len(kinds))
"
```

Result: **19 kinds** total in `site/dashboard/units/PF1e-units-*.json`
(denominator: every `PF1e-units-*.json` file under that directory, one
kind per file, 19 files excluding `index.json`).

Previously-curated (11, all still correct/still present):
`class, class_feature, companion, equipment, equipment_modifier, feat,
monster, monster_ability, race, race_trait, spell`

**Missing (8)** — confirms the sibling lane's count, independently
re-derived rather than trusted:
`ability, deity, domain, language, power, skill, template, trait`

Row counts per missing kind (`len(doc["rows"])` from each ledger file):
`ability` 4337, `deity` 459, `domain` 183, `language` 136, `power` 421,
`skill` 149, `template` 2248, `trait` 487 — 8,220 previously-uncounted
rows total.

## Step 2 — fix shape decision

**Chose: add curated labels for all 8 missing kinds. Rejected: a derived
fallback label.**

Justification: the existing hard failure at `load_units_by_kind` (line
~218) is deliberate, not accidental — its own message says "add one
before regenerating," `KIND_LABELS`'s module comment says "Fail-loud: an
unrecognized kind raises instead of silently omitting it," and the
sibling `LoadUnitsByKindTests.test_unknown_kind_raises_loud` in the
existing test suite proves this behavior was already intentionally
covered. A silent derived-label fallback would convert a real signal
("a new kind entered the corpus, a human needs to name it for the public
page") into permanently-invisible drift — every future new kind would
just render under some auto-generated label with zero review. The crash
itself was doing its job (stopping an unreviewed kind from shipping under
a guessed label); the actual bug was that the curated list fell behind
the real ledger, not that the check exists. Preserving the fail-loud
posture and populating the map is the fix that matches the code's own
stated intent.

Labels added (`scripts/site/build_public_status.py` `KIND_LABELS`):

```
"ability": "Abilities",
"deity": "Deities",
"domain": "Domains",
"language": "Languages",
"power": "Powers",
"skill": "Skills",
"template": "Templates",
"trait": "Traits",
```

## Step 3 — RED → GREEN

Added `LiveKindCoverageTests` to
`scripts/tests/test_build_public_status.py` — reads the REAL, checked-in
`site/dashboard/units/*.json` ledger (not a scratch fixture, since the
defect is specifically the curated map falling behind the real committed
kind set) and proves (a) every live kind has a `KIND_LABELS` entry and
(b) `load_units_by_kind` actually succeeds against the real directory.

RED (before the fix), `python3 -m unittest
scripts.tests.test_build_public_status.LiveKindCoverageTests -v`:

```
KeyError: "unit kind 'ability' (PF1e-units-ability.json) has no curated label in KIND_LABELS — add one before regenerating."
...
AssertionError: Lists differ: ['ability', 'deity', 'domain', 'language', 'power', 'skill', 'template', 'trait'] != []
Ran 2 tests in 0.093s
FAILED (failures=1, errors=1)
```

Both failures reproduce the real crash for the intended reason (the same
`KeyError` the sibling lane hit, plus the enumerated missing set).

GREEN (after adding the 8 labels), `python3 -m unittest
scripts.tests.test_build_public_status -v`:

```
Ran 37 tests in 0.281s
OK
```

(35 pre-existing + 2 new, all pass.)

Also re-ran the adjacent PI-gate suites (unchanged by this fix, confirmed
still green): `scripts.tests.test_site_public_status_pi_gate` (7/7 OK),
`scripts.tests.test_pi_redaction` (49/49 OK), and
`scripts/tests/test_publish_site_dashboard.sh` (6/6 PASS — a different
pipeline, `site/dashboard/`, not touched by this fix, checked because it
shares `publish-site-dashboard.sh`'s freshness-gate posture).

## Step 4 — full pipeline run, before/after

Command (from repo root, `PCGEN_CORPUS_ROOT` set to the repo-local oracle
slot):

```
python3 scripts/site/build_public_status.py
```

Output:

```
Wrote .../site/status-data.json (30 books, overall 33.5%) and 30 book-detail files under .../site/status-data (46074 items total)
```

`--check` immediately after: `OK: status-data.json and status-data/*.json
are up to date` (deterministic, second run matches first modulo
`generated_at`).

Before/after `overall` rollup
(`json.load(open('site/status-data.json'))['overall']`):

| field | before | after | delta |
|---|---|---|---|
| done | 11,695 | 12,674 | +979 |
| partial | 2,087 | 2,602 | +515 |
| not_started | 15,728 | 22,604 | +6,876 |
| denominator | 29,510 | 37,880 | +8,370 |
| pct | 39.6% | 33.5% | −6.1pp |
| excluded_from_percentage | 8,862 | 11,558 | +2,696 |

Book count unchanged (30 — none of the 8 new kinds introduced a new
`book` id outside `BOOK_TITLES`, so `build()`'s own
"BOOK_TITLES has entries with no units in the ledger" guard did not fire
and no book-detail file was orphaned).

This is a real, visible public-facing change, not just an unblock: 8
previously-invisible content kinds (led by `ability` at 4,337 rows) are
now counted in the public status page, and because most of that newly-
counted material is not yet ingested (`not_started` +6,876 vs `done`
+979), the honest headline percentage drops 6.1 points. That is the
correct direction per the same "under-claiming, not the flattering
direction" posture already documented in this file's `DONENESS_TO_PUBLIC`
comment — the previous 39.6% was an artifact of 8 kinds being silently
absent from the denominator, not a true measure.

## Step 5 — did not touch `site/dashboard/PF1e-dashboard.json`

Confirmed via `git status --porcelain` before and after this cycle's work:
only `scripts/site/build_public_status.py`,
`scripts/tests/test_build_public_status.py`, `site/status-data.json`, and
`site/status-data/*.json` are modified. `site/dashboard/PF1e-dashboard.json`
does not appear in the diff — the sibling lane's regeneration of that file
is untouched.

## Out-of-territory observation (not fixed, reported only)

While checking PI redaction on the newly-included kinds, found `"Iomedae"`
published unredacted as a `class_feature` `name`/`display_name` in
`site/status-data/inner_sea_combat.json`. Verified via `git show
<merge-base>:site/status-data/inner_sea_combat.json` that this predates
this cycle's regeneration (same string present in the previously-committed
file) — it is not a regression introduced by adding the 8 kinds, and
`class_feature` was already a curated kind before this fix. PI redaction
logic lives in `scripts/observer/pi_redaction.py`, outside this lane's
territory (`scripts/site/**` only) — reporting, not fixing.

## Dual-audit gate

```
BASE=$(git merge-base HEAD origin/develop)   # 1bb523773d32705d1b7387fd4c494861523f55ba
git diff --unified=0 "$BASE...HEAD" -- scripts/site/build_public_status.py scripts/tests/test_build_public_status.py site/status-data.json site/status-data/ \
  | grep -nE '\b(sd[0-9]+_|SD[0-9]+_|Sd[0-9]+|t_[0-9a-f]{8,})'   # no output -> OK_NO_BUNDLE_TAGS
git diff --unified=0 "$BASE...HEAD" -- scripts/site/build_public_status.py scripts/tests/test_build_public_status.py site/status-data.json site/status-data/ \
  | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'   # no output -> OK_NO_TOKENS
```

Both clean.

## Files changed

- `scripts/site/build_public_status.py` — 8 curated labels added to
  `KIND_LABELS`
- `scripts/tests/test_build_public_status.py` — new `LiveKindCoverageTests`
  class (2 tests)
- `site/status-data.json` + `site/status-data/*.json` (30 book-detail
  files) — regenerated output
