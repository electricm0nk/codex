#!/usr/bin/env bash
# Prove the derived evaluator-vs-fixture check can actually FAIL.
# Each mutation is applied to the committed fixture, the suite is run, the
# result recorded, and the fixture restored from git before the next one.
set -u
FIX=tests/fixtures/rules_core/derived-evaluator-fixtures.json
export CARGO_TARGET_DIR=/home/ubuntu/workspace/codex-target-closure-derived

restore() { git checkout -- "$FIX"; }
trap restore EXIT

run_case() {
    local name="$1"
    cargo test --locked --test derived_evaluator_fixture_check \
        > /home/ubuntu/workspace/codex-target-closure-derived/mut.log 2>&1
    local status=$?
    local line
    line=$(grep -E "^test result:" /home/ubuntu/workspace/codex-target-closure-derived/mut.log | head -1)
    echo "MUTATION: $name"
    echo "  exit=$status  $line"
    grep -E "^test .* FAILED|^failures:$" -A 4 /home/ubuntu/workspace/codex-target-closure-derived/mut.log \
        | grep -E "^test [a-z_]+ \.\.\. FAILED|^    [a-z_]+$" | sed 's/^/  /' | head -4
    echo
    restore
}

echo "=== baseline (unmutated) ==="
run_case "none — committed fixture"

python3 - <<'PY'
import json
p="tests/fixtures/rules_core/derived-evaluator-fixtures.json"
d=json.load(open(p)); d["entries"][0]["expected"]["bonus"] += 1
json.dump(d,open(p,"w"),indent=2); open(p,"a").write("\n")
PY
run_case "expected.bonus +1, corpus_field untouched"

python3 - <<'PY'
import json
p="tests/fixtures/rules_core/derived-evaluator-fixtures.json"
d=json.load(open(p)); d["entries"][0]["upstream_lst_sha256"]="0"*64
json.dump(d,open(p,"w"),indent=2); open(p,"a").write("\n")
PY
run_case "upstream_lst_sha256 zeroed"

python3 - <<'PY'
import json
p="tests/fixtures/rules_core/derived-evaluator-fixtures.json"
d=json.load(open(p)); d["entries"][0]["upstream_line"] += 1
json.dump(d,open(p,"w"),indent=2); open(p,"a").write("\n")
PY
run_case "upstream_line off by one"
