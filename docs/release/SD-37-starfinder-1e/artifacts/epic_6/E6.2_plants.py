#!/usr/bin/env python3
"""SD-37 E6.2 planted mutations: each must turn its test red; the file is restored after each.

Run from the repo root with PATH/CARGO_TARGET_DIR set. Prints `verdict=PASS` only when every
plant fails its test and every restored file's sha256 equals the original.
"""
import hashlib, subprocess, sys

CONV = 'crates/codex-ingest/src/pcgen_import/sheet_rule/convert.rs'
CREA = 'apps/desktop/src-tauri/src/sf_creation.rs'
INGEST = ['cargo', 'test', '--locked', '-j', '8', '-p', 'codex-ingest', '--test', 'sf_core_proof', 'a_selection_hop', '--', '--test-threads=8']
DESK = ['cargo', 'test', '--locked', '-j', '8', '--bin', 'codex-desktop', 'sf_creation', '--', '--test-threads=8']
PLANTS = [
    ('P1 hop pool count +1', CONV, 'value: SheetValue::Number(Expr::Const(n)),\n                                    also: Vec::new(),\n                                    target: Some(BonusTarget::Pool(slug(&pool))),',
     'value: SheetValue::Number(Expr::Const(n + 1)),\n                                    also: Vec::new(),\n                                    target: Some(BonusTarget::Pool(slug(&pool))),', INGEST, '.'),
    ('P2 hop pool lines dropped', CONV, 'lines.extend(std::mem::take(&mut acc.sf_hop_pool_lines));', 'let _ = std::mem::take(&mut acc.sf_hop_pool_lines);', INGEST, '.'),
    ('P3 pool record not held', CREA, 'Some(r) => auto_picks.push(r.id.clone()),', 'Some(r) => later.push(r.id.clone()),', DESK, 'apps/desktop/src-tauri'),
    ('P4 picks not saved', CREA, '            .picks\n            .iter()\n            .chain(answered)', '            .picks\n            .iter()\n            .take(0)\n            .chain(answered)', DESK, 'apps/desktop/src-tauri'),
    ('P5 point buy ignored', CREA, 'let abilities = SfAbilityBuild { point_buy: points, increases: BTreeMap::new() };', 'let abilities = SfAbilityBuild { point_buy: [0; 6], increases: BTreeMap::new() };', DESK, 'apps/desktop/src-tauri'),
    ('P6 key-ability pick asked twice', CREA, '                    key_ability_slots.push((rule.id.clone(), abilities));\n                    continue;', '                    let _ = abilities;', DESK, 'apps/desktop/src-tauri'),
]

def sha(path):
    return hashlib.sha256(open(path, 'rb').read()).hexdigest()

ok = True
for name, path, old, new, cmd, cwd in PLANTS:
    before = sha(path)
    text = open(path).read()
    if text.count(old) != 1:
        print(f'{name}: anchor found {text.count(old)} times'); ok = False; continue
    open(path, 'w').write(text.replace(old, new))
    try:
        r = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True)
    finally:
        open(path, 'w').write(text)
    failed = [l for l in r.stdout.splitlines() if l.endswith('FAILED') and l.startswith('test ')]
    red = r.returncode != 0
    restored = sha(path) == before
    print(f'{name}: {"FAIL" if red else "PASS (plant survived)"} {failed} restored={restored}')
    if not red or not restored:
        ok = False
print('verdict=' + ('PASS' if ok else 'FAIL'))
sys.exit(0 if ok else 1)
