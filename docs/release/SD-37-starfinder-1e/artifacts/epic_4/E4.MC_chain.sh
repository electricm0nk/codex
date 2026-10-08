#!/usr/bin/env bash
# SD-37 E4.MC: the SF package across the whole of E4 (46f0acb5b4 = E3.MC, the E4 start -> HEAD),
# checked link by link with each converter card's own delta classifier, and every gap between
# card commits checked byte-identical. Run from the repo root; SCRATCH outside the repo.
# Exit codes are printed per link; every link must be exit=0 / other_moves=0 and every gap diff_exit=0.
set -u
eval "$(scripts/fetch-pcgen-oracle.sh --check --quiet)"
A=docs/release/SD-37-starfinder-1e/artifacts/epic_4
S=${SCRATCH:?}/chain
P=data/starfinder-1e/sheet_rules
rm -rf "$S"; mkdir -p "$S"
for c in 46f0acb5b4 419501b49a 22e9eb74e4 c9a31b35bf 9e6b2cdbfb 989353a2eb 00bd1332ba 5a59355425 HEAD; do
  mkdir -p "$S/$c" && git archive "$c" "$P" | tar -x -C "$S/$c"
done
echo "## link 0: 46f0acb5b4 (E4 start) -> 419501b49a (E4.1)"
python3 $A/E4.1_race_hp_delta.py "$S/419501b49a/$P" "$S/46f0acb5b4/$P"; echo exit=$?
echo "## gap 419501b49a -> 22e9eb74e4"; diff -rq "$S/419501b49a/$P" "$S/22e9eb74e4/$P"; echo diff_exit=$?
echo "## link E4.2: 22e9eb74e4 -> c9a31b35bf"
python3 $A/E4.2_delta.py "$S/c9a31b35bf/$P" "$S/22e9eb74e4/$P"; echo exit=$?
echo "## gap c9a31b35bf -> 9e6b2cdbfb"; diff -rq "$S/c9a31b35bf/$P" "$S/9e6b2cdbfb/$P"; echo diff_exit=$?
echo "## link E4.4: 9e6b2cdbfb -> 989353a2eb"
python3 $A/E4.4_delta.py "$S/989353a2eb/$P" "$S/9e6b2cdbfb/$P"; echo exit=$?
echo "## gap 989353a2eb -> 00bd1332ba"; diff -rq "$S/989353a2eb/$P" "$S/00bd1332ba/$P"; echo diff_exit=$?
echo "## link E4.5: 00bd1332ba -> 5a59355425"
python3 $A/E4.5_delta.py "$S/5a59355425/$P" "$S/00bd1332ba/$P" "$PCGEN_REPO_DIR/data"; echo exit=$?
echo "## gap 5a59355425 -> HEAD"; diff -rq "$S/5a59355425/$P" "$S/HEAD/$P"; echo diff_exit=$?
echo "## PF package across E4"; git diff --stat 46f0acb5b4 HEAD -- data/sheet_rules | awk 'END{print NR" lines of diffstat"}'
