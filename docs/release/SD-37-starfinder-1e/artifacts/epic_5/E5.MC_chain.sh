#!/usr/bin/env bash
# SD-37 E5.MC: the SF package across the whole of E5 (b9797f1392 = E4.MC end, the E5 start -> HEAD),
# checked link by link with each converter card's own delta classifier; every gap between card
# commits checked byte-identical; then one planted unclassified move per classifier, which must
# turn that classifier red (the per-card plant scripts' delta plants run against a stale base on
# today's tree, so they are re-planted here on their own link). Repo root; SCRATCH outside the repo.
set -u
eval "$(scripts/fetch-pcgen-oracle.sh --check --quiet)"
A=docs/release/SD-37-starfinder-1e/artifacts/epic_5
S=${SCRATCH:?}/chain
P=data/starfinder-1e/sheet_rules
O="$PCGEN_REPO_DIR/data"
rm -rf "$S"; mkdir -p "$S"
for c in b9797f1392 a94024c402 465a6c7228 e49a1669d1 ef2589c219 7df1e26d57 c73912b57d 4c50ff7752 d87611ea43 HEAD; do
  mkdir -p "$S/$c" && git archive "$c" data/starfinder-1e | tar -x -C "$S/$c"
done
echo "## link E5.1: b9797f1392 -> a94024c402"
python3 $A/E5.1_delta.py "$S/a94024c402/$P" "$S/b9797f1392/$P" "$O"; echo exit=$?
echo "## gap a94024c402 -> a94024c402 (E5.2 started on E5.1's commit)"
echo "## link E5.2: a94024c402 -> 465a6c7228"
python3 $A/E5.2_delta.py "$S/465a6c7228/$P" "$S/a94024c402/$P"; echo exit=$?
echo "## gap 465a6c7228 -> e49a1669d1"; diff -rq "$S/465a6c7228/data" "$S/e49a1669d1/data"; echo diff_exit=$?
echo "## link E5.3: e49a1669d1 -> ef2589c219"
python3 $A/E5.3_delta.py "$S/ef2589c219/$P" "$S/e49a1669d1/$P" "$O"; echo exit=$?
echo "## gap ef2589c219 -> 7df1e26d57 (incl. E5.3 attempt 2, 12835aa82c: claimed no package change)"; diff -rq "$S/ef2589c219/data" "$S/7df1e26d57/data"; echo diff_exit=$?
echo "## link E5.4 attempt 1: 7df1e26d57 -> c73912b57d"
python3 $A/E5.4_delta.py "$S/c73912b57d/$P" "$S/7df1e26d57/$P"; echo exit=$?
echo "## gap c73912b57d -> 4c50ff7752"; diff -rq "$S/c73912b57d/data" "$S/4c50ff7752/data"; echo diff_exit=$?
echo "## link E5.4 attempt 2: 4c50ff7752 -> d87611ea43"
python3 $A/E5.4_r2_delta.py "$S/d87611ea43/$P" "$S/4c50ff7752/$P"; echo exit=$?
echo "## gap d87611ea43 -> HEAD"; diff -rq "$S/d87611ea43/data" "$S/HEAD/data"; echo diff_exit=$?
echo "## HEAD snapshot = working tree package"; diff -rq "$S/HEAD/data/starfinder-1e" data/starfinder-1e; echo diff_exit=$?
echo "## PF package across E5 (b9797f1392 -> HEAD)"; git diff --stat b9797f1392 HEAD -- data/sheet_rules | awk 'END{print NR" lines of diffstat"}'

# Planted unclassified moves: a label change on a record the link did not otherwise move.
plant() { # <new-snapshot sha> <rel json under sheet_rules> <rule id>
  python3 - "$S/$1/$P/$2" "$3" <<'PY'
import json, sys
p, rid = sys.argv[1], sys.argv[2]
rs = json.load(open(p, encoding="utf-8"))
hit = [r for r in rs if r["id"] == rid]
assert hit, rid
hit[0]["label"] = hit[0]["label"] + " (planted)"
open(p, "w", encoding="utf-8").write(json.dumps(rs, ensure_ascii=False, separators=(",", ":")))
print("planted label move on", rid)
PY
}
# E5.2's plant first: a94024c402 is E5.2's baseline, so E5.1's plant on it must come after.
echo "## plant E5.2 link"; plant 465a6c7228 core/feat/quick_draw.json core:feat:quick_draw
python3 $A/E5.2_delta.py "$S/465a6c7228/$P" "$S/a94024c402/$P" | tail -2; echo plant_exit=${PIPESTATUS[0]}
echo "## plant E5.1 link"; plant a94024c402 core/feat/quick_draw.json core:feat:quick_draw
python3 $A/E5.1_delta.py "$S/a94024c402/$P" "$S/b9797f1392/$P" "$O" | tail -2; echo plant_exit=${PIPESTATUS[0]}
echo "## plant E5.3 link"; plant ef2589c219 core/feat/quick_draw.json core:feat:quick_draw
python3 $A/E5.3_delta.py "$S/ef2589c219/$P" "$S/e49a1669d1/$P" "$O" | tail -2; echo plant_exit=${PIPESTATUS[0]}
echo "## plant E5.4 attempt-1 link"; plant c73912b57d core/feat/quick_draw.json core:feat:quick_draw
python3 $A/E5.4_delta.py "$S/c73912b57d/$P" "$S/7df1e26d57/$P" | tail -2; echo plant_exit=${PIPESTATUS[0]}
echo "## plant E5.4 attempt-2 link"; plant d87611ea43 core/feat/quick_draw.json core:feat:quick_draw
python3 $A/E5.4_r2_delta.py "$S/d87611ea43/$P" "$S/4c50ff7752/$P" | tail -2; echo plant_exit=${PIPESTATUS[0]}
echo CHAIN_DONE
