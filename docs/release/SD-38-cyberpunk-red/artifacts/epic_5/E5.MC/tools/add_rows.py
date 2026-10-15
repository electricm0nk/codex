#!/usr/bin/env python3
"""E5.MC: insert the E5.2-E5.5 ui-smoke rows after improve-cyberpunk-skill in spec.json.

    python3 add_rows.py <spec.json> <rows.json>

rows.json holds the new rows (a JSON list). The rows are written in spec.json's hand style (one step
per line, compact objects) right after the improve-cyberpunk-skill row; the rest of the file is kept
byte for byte. Refuses to run twice (an id already present).
"""
import json
import sys

spec_path, rows_path = sys.argv[1], sys.argv[2]
raw = open(spec_path, encoding="utf-8").read()
rows = json.load(open(rows_path, encoding="utf-8"))
existing = {r["id"] for r in json.loads(raw)["rows"]}
for row in rows:
    if row["id"] in existing:
        sys.exit(f"{row['id']} is already in spec.json")


def compact(value):
    text = json.dumps(value, ensure_ascii=False)
    return "{ " + text[1:-1] + " }" if isinstance(value, dict) else text


def render(row):
    out = ["    {", f'      "id": {json.dumps(row["id"])},', f'      "screen": {json.dumps(row["screen"])},', '      "steps": [']
    out += [f"        {compact(step)}" + ("," if i < len(row["steps"]) - 1 else "") for i, step in enumerate(row["steps"])]
    out.append("      ],")
    out.append(f'      "marker": {json.dumps(row["marker"], ensure_ascii=False)},')
    out.append('      "expect": [')
    out += [f"        {json.dumps(e, ensure_ascii=False)}" + ("," if i < len(row["expect"]) - 1 else "") for i, e in enumerate(row["expect"])]
    out.append("      ],")
    out.append(f'      "forbid": {json.dumps(row["forbid"], ensure_ascii=False)},')
    out.append(f'      "notes": {json.dumps(row["notes"], ensure_ascii=False)}')
    out.append("    },")
    return "\n".join(out)


lines = raw.split("\n")
anchor = next(i for i, line in enumerate(lines) if line.strip() == '"id": "improve-cyberpunk-skill",')
close = next(i for i in range(anchor, len(lines)) if lines[i] == "    },")
block = "\n".join(render(r) for r in rows)
lines[close + 1:close + 1] = block.split("\n")
new = "\n".join(lines)
parsed = json.loads(new)
ids = [r["id"] for r in parsed["rows"]]
assert len(ids) == len(set(ids)), "duplicate row id"
assert len(ids) == len(existing) + len(rows)
open(spec_path, "w", encoding="utf-8").write(new)
print(f"rows {len(existing)} -> {len(ids)}")
