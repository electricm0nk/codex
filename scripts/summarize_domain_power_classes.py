import glob
import json
from collections import Counter

files = glob.glob(
    "/home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_2656fbd3-1ec-1/data/corpus/*/class_feature/domain_power/*.json"
)
c = Counter()
none_count = 0
for f in files:
    d = json.load(open(f))
    classes = d["data"].get("classes")
    if not classes:
        none_count += 1
        continue
    c[tuple(sorted(classes))] += 1

print("total domain_power records:", len(files))
for k, v in sorted(c.items(), key=lambda kv: -kv[1]):
    print(len(k), "owners:", k, "->", v, "records")
print("unresolved (no classes field):", none_count)
