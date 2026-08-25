#!/usr/bin/env python3
"""Field-by-field pre/post regen diff check for `class_feature` records --
excludes `ingested_at` and `data.class`/`data.classes` (the fields this
cycle's regen is expected to change), reports any other field that moved.
"""
import json
import subprocess
import sys

def git_show(rel_path):
    out = subprocess.run(
        ["git", "-C", "/home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_2656fbd3-1ec-1",
         "show", f"HEAD:{rel_path}"],
        capture_output=True, text=True,
    )
    if out.returncode != 0:
        return None
    return json.loads(out.stdout)

def strip(d):
    d = json.loads(json.dumps(d))
    d.pop("ingested_at", None)
    d.get("data", {}).pop("class", None)
    d.get("data", {}).pop("classes", None)
    return d

def main():
    files = subprocess.run(
        ["git", "-C", "/home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_2656fbd3-1ec-1",
         "status", "--porcelain", "data/corpus"],
        capture_output=True, text=True,
    ).stdout.splitlines()
    other_field_changed = []
    class_or_classes_changed = 0
    classes_added = 0
    checked = 0
    for line in files:
        status = line[:2]
        rel = line[3:].strip()
        if status.strip() != "M":
            continue
        checked += 1
        pre = git_show(rel)
        try:
            post = json.load(open(f"/home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_2656fbd3-1ec-1/{rel}"))
        except Exception as e:
            other_field_changed.append((rel, f"post-parse-error: {e}"))
            continue
        if pre is None:
            continue
        if pre.get("data", {}).get("class") != post.get("data", {}).get("class") or \
           pre.get("data", {}).get("classes") != post.get("data", {}).get("classes"):
            class_or_classes_changed += 1
        if post.get("data", {}).get("classes") and not pre.get("data", {}).get("classes"):
            classes_added += 1
        if strip(pre) != strip(post):
            other_field_changed.append(rel)
    print(f"checked {checked} modified files")
    print(f"class/classes changed: {class_or_classes_changed}")
    print(f"classes newly added: {classes_added}")
    print(f"OTHER field changed (should be 0): {len(other_field_changed)}")
    for r in other_field_changed[:20]:
        print("  ", r)

if __name__ == "__main__":
    main()
