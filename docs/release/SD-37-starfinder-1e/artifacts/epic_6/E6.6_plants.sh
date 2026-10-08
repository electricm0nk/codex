#!/usr/bin/env bash
# E6.6 planted mutations: each edits spec.json in place, starts one ui-smoke run (the run reads spec.json once,
# at start), restores spec.json byte-for-byte (sha256 checked), and the run must come out RED.
#   plant A: open-starfinder-envoy expects 'Credits remaining\t2331' (hand value 2330) -> expected text missing
#   plant B: build-starfinder-soldier loses its '+1 rank Survival' clicks -> Survival total moves -> open row red
set -u
T=/home/ubuntu/workspace/worktrees/codex-sd37
export SPEC=$T/apps/desktop/scripts/ui-smoke/spec.json
S=${S:?scratch dir}
ORIG=$S/spec.orig.json
cp $SPEC $ORIG
SHA=$(sha256sum < $ORIG)
python3 - <<'P'
import os, re
p = os.environ['SPEC']; s = open(p).read()
i = s.index('"id": "open-starfinder-envoy"')
j = s.index('Credits remaining\\t2330', i)
s = s[:j] + 'Credits remaining\\t2331' + s[j + len('Credits remaining\\t2330'):]
open(p, 'w').write(s)
P
($S/run.sh plantA open-starfinder-envoy,landing-select-pathfinder > /dev/null 2>&1 &)
sleep 20
cp $ORIG $SPEC
python3 - <<'P'
import json, os
p = os.environ['SPEC']; s = open(p).read()
i = s.index('"id": "build-starfinder-soldier"'); k = s.index('"id": "open-starfinder-soldier"')
body = s[i:k]
needle = '{\n          "op": "click",\n          "target": "+1 rank Survival"\n        },\n        {\n          "op": "wait",\n          "ms": 250\n        },\n        '
assert body.count(needle) == 3, body.count(needle)
s = s[:i] + body.replace(needle, '') + s[k:]
open(p, 'w').write(s)
P
($S/run.sh plantB open-starfinder-soldier,landing-select-pathfinder > /dev/null 2>&1 &)
sleep 20
cp $ORIG $SPEC
echo "spec restored: $( [ "$(sha256sum < $SPEC)" = "$SHA" ] && echo identical || echo DIFFERENT )"
