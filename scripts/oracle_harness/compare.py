"""The per-unit comparison harness (`AT-33-E2-003`).

`compare_unit` answers, for one unit, `(ours, oracle, verdict)` where
`verdict` is one of `"agree"`, `"disagree"`, or `"unverifiable"`.
`unverifiable` is returned as data, never raised as an exception and never
folded into `"agree"` -- a missing or blank oracle value is exactly the
"cannot be verified" case `decisions.md` §7 requires stay a visible bucket.

`run_comparison` is the batch entry point: given an `ours` mapping and an
oracle export (raw text or an already-parsed dict from
`oracle_export.parse_oracle_export`), it returns one record per unit in the
exact `{"unit_id", "ours", "oracle", "verdict"}` shape
`scripts/box_ledger.py::load_oracle_results` reads.
"""

from __future__ import annotations

from . import oracle_export as OE


def normalize_numeric(value):
    """Coerce an int/float, or a numeric string (including PCGen's signed
    `"+3"`/`"-2"` export convention), to a Python number. Returns `None` for
    anything that is not numeric (a plain string like `"Fighter"`) or blank."""
    if value is None:
        return None
    if isinstance(value, bool):
        return None
    if isinstance(value, (int, float)):
        return value
    s = str(value).strip()
    if not s:
        return None
    try:
        return int(s)
    except ValueError:
        pass
    try:
        return float(s)
    except ValueError:
        return None


def _is_blank(value):
    return isinstance(value, str) and value.strip() == ""


def compare_unit(unit_id, ours, oracle):
    """Compare one unit's `ours` value against its `oracle` value.

    `oracle=None` (no oracle value obtainable at all -- e.g. the export
    template never captured that token, or the oracle run itself failed)
    and `oracle=""` (the token fired but the oracle produced no content for
    it) are both treated as `unverifiable`, never as a disagreement and
    never silently treated as agreement.
    """
    if oracle is None or _is_blank(oracle):
        return {"unit_id": unit_id, "ours": ours, "oracle": None, "verdict": "unverifiable"}

    ours_num = normalize_numeric(ours)
    oracle_num = normalize_numeric(oracle)

    if ours_num is not None and oracle_num is not None:
        ours_val, oracle_val = ours_num, oracle_num
        verdict = "agree" if ours_num == oracle_num else "disagree"
    else:
        ours_val, oracle_val = ours, oracle
        verdict = "agree" if str(ours).strip() == str(oracle).strip() else "disagree"

    return {"unit_id": unit_id, "ours": ours_val, "oracle": oracle_val, "verdict": verdict}


def run_comparison(ours, oracle_source):
    """Batch-compare a set of units against one oracle export.

    `ours` maps `unit_id -> (oracle_export_key, our_computed_value)`.
    `oracle_source` is either the raw export text (parsed here via
    `oracle_export.parse_oracle_export`) or an already-parsed dict (e.g.
    from `oracle_export.load_oracle_export`).

    Returns a list of records in the shape
    `scripts/box_ledger.py::load_oracle_results` consumes.
    """
    parsed = OE.parse_oracle_export(oracle_source) if isinstance(oracle_source, str) else oracle_source

    records = []
    for unit_id, (oracle_key, ours_value) in ours.items():
        oracle_value = parsed.get(oracle_key)
        records.append(compare_unit(unit_id, ours_value, oracle_value))
    return records
