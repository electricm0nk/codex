#!/usr/bin/env python3
"""E6.5a planted mutations: each one must turn its test red (for the reason named), then the file
is restored byte-for-byte (sha256 checked). One cargo process at a time."""
import hashlib, os, subprocess, sys

T = '/home/ubuntu/workspace/worktrees/codex-sd37'
D = f'{T}/apps/desktop'
ENV = dict(os.environ, PATH=os.path.expanduser('~/.cargo/bin') + ':' + os.environ['PATH'],
           CARGO_TARGET_DIR='/home/ubuntu/workspace/worktrees/cargo-target/sd37-codex-sd37')
def rust(filt):
    return (['cargo', 'test', '--locked', '-j', '8', '--bin', 'codex-desktop', filt, '--', '--test-threads=8'], f'{D}/src-tauri')
CHOICES = rust('sf_choices')
PRINT = rust('sf_sheet_print')
FRONT = ([f'{D}/node_modules/.bin/tsx', 'src/characterHub/starfinderChoicesModel.test.ts'], D)
SC = f'{D}/src-tauri/src/sf_choices.rs'
CR = f'{D}/src-tauri/src/sf_creation.rs'
LU = f'{D}/src-tauri/src/sf_level_up.rs'
SP = f'{D}/src-tauri/src/sf_sheet_print.rs'
MODEL = f'{D}/src/characterHub/starfinderChoicesModel.ts'

PLANTS = [
    ('P1 every feat reads as eligible (engine verdict ignored)', SC,
     'eligible: evaluate_applies(&r.applies, &sf.held, package, &facts, EvalContext::default()).includes(),', 'eligible: true,', CHOICES),
    ('P2 the chassis base attack bonus is not fed to the prerequisite', SC,
     '    facts.base_attack = base_attack;\n', '    let _ = base_attack;\n', CHOICES),
    ('P3 an equipped item is saved as carried', SC,
     'let active_state = if g.equipped { ActiveState::EquippedActive } else { ActiveState::SelectedInactive };',
     'let active_state = ActiveState::SelectedInactive;', CHOICES),
    ('P4 one spell over the engine total is allowed', SC, 'if n > known.total {', 'if n > known.total + 1 {', CHOICES),
    ('P5 every modifier offered on every item', SC, "        .filter(|m| !m.id.contains('#') && modifier_fits(item, m).is_ok())\n        .map(", "        .filter(|m| !m.id.contains('#'))\n        .map(", CHOICES),
    ('P6 the preview returns no totals', SC, '                .filter_map(|id| rows.iter().find(|e| e.id == *id))', '                .filter_map(|id| rows.iter().find(|e| e.id == *id && false))', CHOICES),
    ('P7 creation ignores the choices it is given', CR, '                input = crate::sf_choices::apply(&input, choices);', '                let _ = choices;', CHOICES),
    ('P8 the level-up ignores the choices it is given', LU, '        Some(choices) => crate::sf_choices::apply(&leveled, choices),', '        Some(_) => leveled,', CHOICES),
    ('P9 a fusion fits armour (modifier_fits)', SP, '    } else if has_tag(modifier, FUSION_TAG) && stat_row(item, DAMAGE_ROW).is_none() {', '    } else if has_tag(modifier, FUSION_TAG) && false {', PRINT),
    ('P10 removing a feat keeps its own pick', MODEL, 'featPicks: choices.featPicks.filter((pick) => feats.includes(pick.slotId))', 'featPicks: choices.featPicks', FRONT),
    ('P11 the totals line prints credits spent, not remaining', MODEL, "'sf.credits.remaining', 'sf.bulk'];", "'sf.credits.spent', 'sf.bulk'];", FRONT),
]

def sha(p):
    return hashlib.sha256(open(p, 'rb').read()).hexdigest()

red = 0
for name, path, old, new, (cmd, cwd) in PLANTS:
    before = sha(path)
    text = open(path).read()
    assert text.count(old) == 1, f'{name}: anchor not unique/absent'
    open(path, 'w').write(text.replace(old, new))
    try:
        r = subprocess.run(cmd, cwd=cwd, env=ENV, capture_output=True, text=True)
    finally:
        open(path, 'w').write(text)
    restored = sha(path) == before
    out = r.stdout + r.stderr
    fails = [l for l in out.splitlines() if ' FAILED' in l or 'panicked' in l or l.startswith('Error') or l.startswith('error')][:3]
    ok = r.returncode != 0 and restored
    red += ok
    print(f'{name}: exit={r.returncode} {"RED" if r.returncode else "GREEN(!)"} restored={restored}')
    for l in fails:
        print('    ' + l[:220])
print(f'{red} of {len(PLANTS)} plants red, every file restored' if red == len(PLANTS) else f'verdict=FAIL {red} of {len(PLANTS)}')
sys.exit(0 if red == len(PLANTS) else 1)
