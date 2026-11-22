#!/usr/bin/env python3
"""SD-32 T9-onboarding-cause-closure — scoped regenerator for row 17's
PI-over-redaction defect (`decisions.md §24b`-2, `kanban.md` row 17).

`scripts/pi_scrub.py::scrub_name_pi_tokens` had an over-broad needle
generator: splitting a `~`-delimited `KEY` segment into individual WORDS
turned ordinary PCGen/Pathfinder rules vocabulary ("Trait", "Temp", "Bonus",
"Evangelist", "Sentinel", "Exalted", ...) into standalone redaction needles,
so a genuinely PI-free `BONUS`/`DEFINE` formula value got wiped to
`[redacted PI]` purely because it happened to contain the same ordinary
word as an UNRELATED part of the record's own (separately, correctly
PI-blocked) name. `scripts/row17_census.py` names this population
(`fallthrough`, `pi_redacted_formula=true`) -- re-derive it fresh with
`python3 scripts/row17_census.py` rather than trusting a stale count here.

**Scope, deliberately narrower than `regen_all_renamed_pi_scrub.py`.** That
driver reprocesses EVERY `codex_generated_name: true` record corpus-wide
(856 on this run) -- most of which are `equipment`/`ability`/`deity`/`spell`/
`monster` records this cycle's territory note names as live sibling-lane
ground. This script touches ONLY the units `scripts/row17_census.py`'s own
join currently reports as `fallthrough` with `pi_redacted_formula=true` --
the exact population the over-redaction defect produced -- so a concurrent
sibling lane's in-flight corpus writes are never at risk of collision.

**Near-miss found and fixed live in this cycle, before any commit.** An
earlier version of this script re-derived ONLY `data.raw_tokens` through
`scrub_name_pi_tokens`, mirroring `regen_all_renamed_pi_scrub.py`'s own
shape. Running `cargo run --bin declared_pi_shipping_audit` against its
output caught a real, NEW leak: for a record whose `DESC` prose does not
happen to literally contain the record's own PI name as a substring (an
ordinary narrative sentence, not a restatement of the key),
`scrub_name_pi_tokens`'s identity/blacklist checks never fire on that DESC
token, so the freshly re-parsed oracle line's FULL, un-redacted description
text was about to ship in `data.raw_tokens` even though `data.description`
itself stayed correctly redacted -- `DESC-PI-SHIPPED-IN-RAW-TOKENS`, exactly
the shape `declared_pi_shipping_audit.rs` exists to catch. Confirmed the
SAME gap reproduces against `regen_all_renamed_pi_scrub.py`'s own logic
directly (that driver's `raw_tokens` re-derivation also omits the DESC
pre-redaction step `ingest_generic_kind.py::remediate` performs before its
own `scrub_name_pi_tokens` call) -- a latent, pre-existing defect in that
driver too, reported separately rather than silently worked around here.
This script now reproduces `remediate()`'s FULL pipeline (declared-PI
detection -> DESC blanking -> blacklist scan -> identity scan), not just
the identity scan alone, closing the gap before any write ships.

Re-derives each affected record's `data.raw_tokens` (and, for a `DESC`
token whose prose is independently PI-bearing, `data.description`) from the
SAME pinned oracle byte source (`source.path`/`source.line`) the record
already cites. Never computes a new path, slug, key, or name. Only
reprocesses `codex_generated_name: true` records (the only records
`scrub_name_pi_tokens` -- as opposed to the separate, unaffected
`blacklist_term_hit_including_concatenated`-only screen -- ever ran on).

Run: `python3 scripts/regen_row17_pi_over_redaction.py [--dry-run]`
`PCGEN_CORPUS_ROOT` must point at a pinned PCGen `data/` checkout.
"""
from __future__ import annotations

import glob
import json
import os
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))

from ingest_ability import corpus_root, read_row, row_tokens  # noqa: E402
from ingest_generic_kind import declared_pi, desc_value  # noqa: E402
from sd32_t9_pi_review_feat_equipment import (  # noqa: E402
    extract_free_text,
    normalized_term_hit,
)
from pi_scrub import (  # noqa: E402
    REDACTED_PI_MARKER,
    blacklist_term_hit_including_concatenated,
    scrub_name_pi_tokens,
)
from codex_neutral_name import neutral_name as codex_neutral_name  # noqa: E402
import coverage_ledger as CL  # noqa: E402
from shape_ledger import (  # noqa: E402
    DEFAULT_CORPUS_ROOT,
    build_corpus_index,
    build_corpus_key_index,
    build_cross_book_key_index,
    build_ledger,
    load_inventory_or_die,
)

INVENTORY_PATH = os.path.join(REPO_ROOT, "docs/work-inventory.json")


def normalize_kind_dir_candidates(kind: str) -> list[str]:
    return [kind, f"{kind}_generic"]


def find_corpus_path(book: str, kind: str, source_file: str, line: int) -> str | None:
    for kdir in normalize_kind_dir_candidates(kind):
        pattern = os.path.join(REPO_ROOT, "data/corpus", book, kdir, "**", "*.json")
        for path in glob.glob(pattern, recursive=True):
            try:
                with open(path, encoding="utf-8") as fh:
                    rec = json.load(fh)
            except (OSError, json.JSONDecodeError):
                continue
            src = rec.get("source", {})
            if src.get("line") == line and src.get("path", "").endswith(source_file):
                return path
    return None


def redact_tokens(
    raw_line: str, orig_name: str, orig_key: str, neutral_name_hint: str | None = None
) -> tuple[list[dict], str | None, bool]:
    """Mirrors `ingest_generic_kind.py::remediate`'s redaction pipeline
    exactly (declared-PI detection -> DESC blanking -> blacklist scan ->
    identity/blacklist scan via `scrub_name_pi_tokens`), so a record's
    `DESC` token is never left carrying un-redacted prose merely because
    the identity/blacklist checks alone did not happen to match it.

    `neutral_name_hint`, when the record already carries a `§24`
    Codex-generated name, is threaded into `scrub_name_pi_tokens` so a
    value whose ONLY PI content is a plain self-reference to the record's
    own (now-redacted) name/key is narrowed to that neutral name instead of
    wiped whole (`decisions.md` row-17-remaining-21 fix, `scripts/pi_scrub.py`
    docstring: "a BONUS:/DEFINE: value is a game rule, not Product Identity").

    Returns (final_tokens, stored_description, any_raw_tokens_redacted)."""
    tokens = row_tokens(raw_line)
    _name_declared, desc_declared = declared_pi(tokens)

    description = desc_value(tokens)
    free_text = extract_free_text(raw_line)
    desc_hit = normalized_term_hit(free_text) if free_text else None
    pi_redacted = desc_declared or bool(desc_hit)

    stored_description = description
    if pi_redacted and description is not None:
        stored_description = REDACTED_PI_MARKER
        for t in tokens:
            if t["key"] == "DESC":
                t["value"] = REDACTED_PI_MARKER

    blacklist_extra_redacted = False
    scrubbed = []
    for t in tokens:
        value = t["value"]
        if pi_redacted and t["key"] == "DESC" and value == REDACTED_PI_MARKER:
            scrubbed.append(dict(t))
            continue
        if value and blacklist_term_hit_including_concatenated(value):
            scrubbed.append({"key": t["key"], "value": REDACTED_PI_MARKER})
            blacklist_extra_redacted = True
        else:
            scrubbed.append(dict(t))
    tokens = scrubbed

    final_tokens, identity_extra_redacted = scrub_name_pi_tokens(
        tokens, orig_name, orig_key, neutral_name=neutral_name_hint
    )
    any_raw_redacted = blacklist_extra_redacted or identity_extra_redacted
    return final_tokens, stored_description, any_raw_redacted


def main() -> int:
    dry_run = "--dry-run" in sys.argv
    root = corpus_root()
    if not os.path.isdir(root):
        print(f"PCGEN_CORPUS_ROOT ({root}) is not a directory", file=sys.stderr)
        return 1

    inventory = load_inventory_or_die(INVENTORY_PATH)
    units = CL.not_done_population(inventory)
    units_by_id = {u["id"]: u for u in units}
    books = {u.get("book") for u in units if u.get("book")}
    corpus_index = build_corpus_index(DEFAULT_CORPUS_ROOT, books)
    key_index = build_corpus_key_index(DEFAULT_CORPUS_ROOT, books)
    cross_book_key_index = build_cross_book_key_index(DEFAULT_CORPUS_ROOT)
    ledger = build_ledger(units, corpus_index, key_index, cross_book_key_index)

    fallthrough_rows = [
        r for r in ledger["rows"]
        if r.get("f0_reached_by") == "fallthrough" and r.get("pi_redacted_formula")
    ]

    report = {
        "row17_pi_redacted_fallthrough_population": len(fallthrough_rows),
        "not_codex_generated_skipped": 0,
        "unresolved_path": [],
        "changed": 0,
        "unchanged": 0,
        "changed_paths": [],
    }

    for row in fallthrough_rows:
        unit = units_by_id.get(row["id"])
        if unit is None:
            report["unresolved_path"].append(row["id"])
            continue
        path = find_corpus_path(unit["book"], row["kind"], unit["source_file"], unit["source_line"])
        if path is None:
            report["unresolved_path"].append(row["id"])
            continue

        with open(path, encoding="utf-8") as fh:
            rec = json.load(fh)

        if not rec.get("codex_generated_name"):
            report["not_codex_generated_skipped"] += 1
            continue

        orig_name = unit["name"]
        orig_key = unit.get("corpus_key") or unit.get("key") or unit["name"]

        src_path = os.path.join(root, rec["source"]["path"])
        src_line = rec["source"]["line"]
        raw_line = read_row(src_path, src_line)

        # This record already carries a `§24` Codex-generated identity
        # (checked above) -- re-derive the SAME coordinate-only name (never
        # re-mint a new one) so a self-reference-only value narrows to it
        # instead of being wiped whole.
        neutral_name_hint = codex_neutral_name(row["kind"], unit["book"], unit["source_file"], unit["source_line"])

        scrubbed_tokens, stored_description, extra_redacted = redact_tokens(
            raw_line, orig_name, orig_key, neutral_name_hint=neutral_name_hint
        )

        old_tokens = rec.get("data", {}).get("raw_tokens")
        old_description = rec.get("data", {}).get("description")
        if scrubbed_tokens != old_tokens or stored_description != old_description:
            report["changed"] += 1
            report["changed_paths"].append(os.path.relpath(path, REPO_ROOT))
            rec["data"]["raw_tokens"] = scrubbed_tokens
            rec["data"]["description"] = stored_description
            fields_redacted = (rec.get("pi_field") or "").split(",") if rec.get("pi_field") else []
            fields_redacted = [f for f in fields_redacted if f]
            if stored_description == REDACTED_PI_MARKER and "description" not in fields_redacted:
                fields_redacted.append("description")
            if extra_redacted and "raw_tokens" not in fields_redacted:
                fields_redacted.append("raw_tokens")
            rec["pi_field"] = ",".join(fields_redacted) if fields_redacted else rec.get("pi_field")
            if not dry_run:
                with open(path, "w", encoding="utf-8") as fh:
                    json.dump(rec, fh, indent=2, ensure_ascii=False)
                    fh.write("\n")
        else:
            report["unchanged"] += 1

    print(json.dumps(report, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
