#!/usr/bin/env python3
"""E6.4 planted mutations: each must turn its test red; every file is restored (sha256 checked)."""
import hashlib, os, subprocess, sys
T = '/home/ubuntu/workspace/worktrees/codex-sd37'
RS = 'apps/desktop/src-tauri/src/sf_catalog.rs'
LS = 'apps/desktop/src/characterHub/LandingScreen.tsx'
ENV = dict(os.environ, PATH=os.path.expanduser('~/.cargo/bin') + ':' + os.environ['PATH'],
           CARGO_TARGET_DIR='/home/ubuntu/workspace/worktrees/cargo-target/sd37-codex-sd37')
RUST = ['cargo', 'test', '--locked', '-j', '8', '--bin', 'codex-desktop', 'sf_catalog', '--', '--test-threads=8']
FRONT = ['node_modules/.bin/tsx', 'src/starfinderCatalog/starfinderCatalog.test.ts']
PLANTS = [
    ('P1 theme catalog loses its Theme Selection filter', RS,
     'r.pool == "theme" && r.tags.iter().any(|t| t == THEME_SELECTION_TAG)', 'r.pool == "theme"', 'rust'),
    ('P2 the module imports the Pathfinder tables', RS,
     'use crate::sf_adapter;\n', 'use crate::sf_adapter;\n#[allow(unused_imports)]\nuse codex::rules_core::rules_tables as _pf;\n', 'rust'),
    ('P3 the hit-die row is printed', RS,
     '.map(|d| (without_hit_die(&d.text), tier_name(d.tier).to_owned()))', '.map(|d| (d.text.clone(), tier_name(d.tier).to_owned()))', 'rust'),
    ('P4 races named by the record label', RS,
     'if kind == SfCatalogKind::Race { race_name(package, rule) } else { rule.label.clone() }', 'rule.label.clone()', 'rust'),
    ('P5 a sibling row dropped from the totals', RS,
     '.filter(|r| r.target.is_some())\n        .filter_map', '.filter(|r| r.target.is_some() && !r.id.ends_with("#race_hp"))\n        .filter_map', 'rust'),
    ('P6 Starfinder landing keeps the Pathfinder links', LS,
     "props.selectedRuleSet === 'starfinder-1e' ?", "props.selectedRuleSet === ('none' as RuleSetId) ?", 'front'),
]
def sha(p): return hashlib.sha256(open(os.path.join(T, p), 'rb').read()).hexdigest()
red = 0
for name, path, old, new, which in PLANTS:
    full = os.path.join(T, path); before = sha(path); text = open(full).read()
    assert text.count(old) == 1, f'{name}: anchor not unique'
    open(full, 'w').write(text.replace(old, new))
    try:
        if which == 'rust':
            r = subprocess.run(RUST, cwd=os.path.join(T, 'apps/desktop/src-tauri'), env=ENV, capture_output=True, text=True)
        else:
            r = subprocess.run(FRONT, cwd=os.path.join(T, 'apps/desktop'), env=ENV, capture_output=True, text=True)
    finally:
        open(full, 'w').write(text)
    out = r.stdout + r.stderr
    why = [l for l in out.splitlines() if 'panicked' in l or 'FAILED' in l or l.startswith('Error') or 'error[' in l][:3]
    ok = r.returncode != 0
    red += ok
    print(f"{name}: {'RED' if ok else 'GREEN (plant survived!)'} exit={r.returncode} {why}")
    assert sha(path) == before, f'{name}: not restored'
print(f"verdict={'PASS' if red == len(PLANTS) else 'FAIL'} red={red} of {len(PLANTS)}")
sys.exit(0 if red == len(PLANTS) else 1)
