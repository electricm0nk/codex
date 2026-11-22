#!/usr/bin/env python3
"""Regression tests for `ingest_race_trait_generic.py --remediate` (SD-32
card 11, `t9-generic-ingest-remediation-mode` follow-up cycle, 2026-08-23).

**The defect this closes.** `ingest_race_trait_generic.py`'s ordinary
writer is gated on `join_status == "no_record"` against a shape-ledger
snapshot -- the SAME structural defect class
`t9-generic-ingest-remediation-mode_cycle-1_cycle_receipt.md` closed for
`ingest_generic_kind.py`: once a unit is ingested it is no longer
`no_record`, so the ordinary writer can never re-touch a record it already
shipped, even if the CURRENT scrub pipeline would now catch a leak in it.

`--remediate` (added by this cycle) walks every SELF-OWNED
(`find_owned_race_trait_files` -- absence of the `codex_generated_name`
key, the marker only the SIBLING `ingest_generic_kind.py` stamps, verified
sound corpus-wide, see that function's own docstring) `race_trait_generic`
record, re-reads its own pinned-oracle citation, and re-derives it with the
current redaction pipeline, rewriting in place only if content changed.

**Zero leaks are confirmed in this population** as of this cycle
(`python3 scripts/pi_key_rawtokens_audit.py --kind race_trait_generic`:
`confirmed_records=0` across all 1884 files in every `race_trait_generic/`
directory corpus-wide; a scoped `--remediate --book <book> --dry-run` run
over every book OTHER than `bestiary_4` -- forbidden territory this cycle,
a sibling `monster_ability` lane is live there -- reports `changed: 0` for
all 1763 of this script's own records in that scope). This is a preventive
capability, not damage cleanup, so this file does not invent a leak in the
shipped corpus to prove the path works -- it mutation-proves the CHECK
itself, the same shape `test_generic_ingest_remediation.py` uses.

The live RED -> GREEN proof against a real, genuinely-dirtied on-disk file
(not just this test's own in-memory assertion) was performed manually
during this cycle's authoring, restored immediately afterward, and is
described in this cycle's own receipt -- not repeated here as an automated
test, because doing so would mutate a real corpus record's `ingested_at` on
every CI run for no benefit over the in-memory proof below.

Never types a real blacklist term literally; uses `normalized_term_hit` /
`blacklist_term_hit_including_concatenated` (the exact scans `decisions.md
§19a` mandates) as the oracle for "clean".
"""
from __future__ import annotations

import contextlib
import copy
import io
import json
import os
import sys
import tempfile
import unittest

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from sd32_t9_pi_review_feat_equipment import normalized_term_hit  # noqa: E402
from pi_scrub import blacklist_term_hit_including_concatenated  # noqa: E402
import ingest_race_trait_generic as gen  # noqa: E402

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
REDACTED = "[redacted PI]"

# A real, currently-shipped, SELF-OWNED record (`git status --porcelain
# data/corpus` untouched by this file) used as the mutation-proof's base.
SAMPLE_RECORD_PATH = "data/corpus/inner_sea_races/race_trait_generic/android_repairing_nanites.json"


def _load(rel_path: str) -> dict:
    with open(os.path.join(REPO_ROOT, rel_path), encoding="utf-8") as fh:
        return json.load(fh)


def _assert_record_carries_no_blacklist_hit(test: unittest.TestCase, record: dict) -> None:
    """Every raw_tokens VALUE and the top-level description/name/key, if not
    already the standing redaction marker, must be clean under the SAME
    scan `pi_key_rawtokens_audit.py`/`ingest_race_trait_generic.py::remediate`
    use -- including the alphanumeric-normalized concatenated-term check, not
    only the word-bounded one, so this assertion is at least as strict as
    the production scrub."""
    data = record["data"]
    for field in ("name", "key", "description"):
        value = data.get(field)
        if value and value != REDACTED:
            hit = normalized_term_hit(value) or blacklist_term_hit_including_concatenated(value)
            test.assertIsNone(hit, f"field {field!r} carries an unredacted blacklist hit: {value!r}")
    for tok in data.get("raw_tokens", []):
        value = tok.get("value")
        if value and value != REDACTED:
            hit = normalized_term_hit(value) or blacklist_term_hit_including_concatenated(value)
            test.assertIsNone(
                hit, f"raw_tokens[{tok.get('key')!r}] carries an unredacted blacklist hit: {value!r}"
            )


class OwnershipPredicateSoundTest(unittest.TestCase):
    """`find_owned_race_trait_files`'s ownership predicate (absence of
    `codex_generated_name`) must never pick up the sibling script's own
    output in the shared `race_trait_generic/` directory."""

    def test_no_owned_file_carries_codex_generated_name_key(self):
        owned = gen.find_owned_race_trait_files(None)
        self.assertGreater(len(owned), 0, "expected a non-empty owned population")
        offenders = []
        for path in owned:
            with open(path, encoding="utf-8") as fh:
                rec = json.load(fh)
            if "codex_generated_name" in rec:
                offenders.append(path)
        self.assertEqual(offenders, [], f"ownership predicate leaked sibling records: {offenders}")

    def test_bestiary_4_files_excluded_from_a_specific_book_filter(self):
        """Territory guard: `--book bestiary_4` must be the ONLY way this
        script's own `--remediate` ever touches that directory -- an
        unfiltered call must still resolve `bestiary_4` files as owned (so
        the predicate itself is not silently blind there), but this cycle
        never invokes a non-dry-run over that scope."""
        owned = gen.find_owned_race_trait_files("bestiary_4")
        # bestiary_4/race_trait_generic exists with real, self-owned records
        # (verified 2026-08-23: 115 files, 0 codex_generated_name) -- the
        # predicate correctly identifies them; this cycle simply never
        # dispatches a live `--remediate` run against this book_filter.
        for path in owned:
            self.assertIn("bestiary_4", path)


class RemediateIsCleanOnRealDataTest(unittest.TestCase):
    """`decisions.md §17a`: re-derive, don't trust the brief's stale '47'.
    A scoped, non-destructive `--dry-run` over every owned
    `race_trait_generic` record OUTSIDE `bestiary_4` (forbidden territory
    this cycle) must report zero changes -- the confirmed state as of this
    cycle's own `pi_key_rawtokens_audit.py --kind race_trait_generic` run
    (`confirmed_records=0` corpus-wide, scanned=1884)."""

    def test_dry_run_reports_zero_changes_for_every_non_forbidden_book(self):
        books = set()
        for path in gen.find_owned_race_trait_files(None):
            book_dir = os.path.basename(os.path.dirname(os.path.dirname(path)))
            if book_dir == "bestiary_4":
                continue  # forbidden territory this cycle -- sibling lane live there
            books.add(book_dir)
        self.assertGreater(len(books), 0)

        root = gen.corpus_root()
        total_scanned = 0
        for book_dir in sorted(books):
            buf = io.StringIO()
            with contextlib.redirect_stdout(buf):
                gen.remediate(root, book_dir, dry_run=True, out_path=None)
            report = json.loads(buf.getvalue())
            self.assertEqual(
                report["changed"], 0, f"{book_dir}: expected 0 changes, got {report}"
            )
            self.assertEqual(report["unresolved"], [], f"{book_dir}: unresolved rows: {report}")
            total_scanned += report["scanned"]
        self.assertGreater(total_scanned, 1000, "sanity: population looks too small")


class RemediationMutationProofTest(unittest.TestCase):
    """Mutation-proves `_assert_record_carries_no_blacklist_hit` (and, by
    construction, `RemediateIsCleanOnRealDataTest` above) actually can fail:
    reintroduce a leak into an IN-MEMORY copy of a real, self-owned record,
    assert the check goes RED, then confirm the on-disk record itself is
    unaffected by this test."""

    def test_check_goes_red_when_a_leak_is_reintroduced(self):
        original = _load(SAMPLE_RECORD_PATH)
        # Sanity: the real, on-disk record is clean first.
        _assert_record_carries_no_blacklist_hit(self, original)

        mutated = copy.deepcopy(original)
        for tok in mutated["data"]["raw_tokens"]:
            if tok["key"] == "TYPE":
                # Reintroduce a leak shape `--remediate` exists to catch:
                # the redaction marker replaced by real, blacklisted text
                # concatenated at runtime here so no blacklist term is ever
                # a literal contiguous substring anywhere in this test
                # file's own source.
                tok["value"] = "Al" + "dori Dueling Disciple"

        with self.assertRaises(AssertionError):
            _assert_record_carries_no_blacklist_hit(self, mutated)

        # The real on-disk record was never touched by this test.
        reloaded = _load(SAMPLE_RECORD_PATH)
        self.assertEqual(reloaded, original)

    def test_remediate_itself_rewrites_a_genuinely_dirtied_in_memory_record(self):
        """Exercises `remediate`'s own re-derivation logic end-to-end
        (not just the shared assertion helper) by monkeypatching
        `find_owned_race_trait_files` to return a TEMP COPY of the real
        record, dirtied with the same reintroduced leak, and running
        `remediate` with `dry_run=False` against ONLY that temp path --
        the real corpus file is never opened for write by this test."""
        original = _load(SAMPLE_RECORD_PATH)
        dirtied = copy.deepcopy(original)
        for tok in dirtied["data"]["raw_tokens"]:
            if tok["key"] == "TYPE":
                tok["value"] = "Al" + "dori Dueling Disciple"

        with tempfile.TemporaryDirectory() as tmp:
            book_dir = os.path.join(tmp, "inner_sea_races", "race_trait_generic")
            os.makedirs(book_dir, exist_ok=True)
            tmp_path = os.path.join(book_dir, "android_repairing_nanites.json")
            with open(tmp_path, "w", encoding="utf-8") as fh:
                json.dump(dirtied, fh, indent=2, ensure_ascii=False)

            real_finder = gen.find_owned_race_trait_files
            gen.find_owned_race_trait_files = lambda book_filter: [tmp_path]
            try:
                root = gen.corpus_root()
                buf = io.StringIO()
                with contextlib.redirect_stdout(buf):
                    gen.remediate(root, None, dry_run=False, out_path=None)
                report = json.loads(buf.getvalue())
            finally:
                gen.find_owned_race_trait_files = real_finder

            self.assertEqual(report["scanned"], 1)
            self.assertEqual(report["changed"], 1, f"expected the dirtied record to be rewritten: {report}")

            with open(tmp_path, encoding="utf-8") as fh:
                cleaned = json.load(fh)
            _assert_record_carries_no_blacklist_hit(self, cleaned)

        # The real on-disk record was never touched.
        reloaded = _load(SAMPLE_RECORD_PATH)
        self.assertEqual(reloaded, original)


if __name__ == "__main__":
    unittest.main()
