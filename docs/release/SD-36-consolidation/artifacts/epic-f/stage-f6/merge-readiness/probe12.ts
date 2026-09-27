import { readFileSync } from 'node:fs';
import { loadedClassFacts, summarizeWeaponProficiency, summarizeCasterLevel } from '/home/ubuntu/workspace/worktrees/codex-epic-f6/apps/desktop/src/characterHub/classFactsModel';
import { classSkillLookup, SKILLS } from '/home/ubuntu/workspace/worktrees/codex-epic-f6/apps/desktop/src/characterHub/skillsModel';
const wire = JSON.parse(readFileSync('/home/ubuntu/workspace/worktrees/codex-epic-f6/docs/release/SD-36-consolidation/artifacts/epic-f/stage-f6/f6a-class-facts-wire.json', 'utf8'));
const ids = (process.argv[2] ?? 'samurai,warrior,magus,kineticist,shaman,monk,unchained_rogue,cleric,sorcerer,psion,expert,commoner,bloodrager,ranger,paladin').split(',');
const levels = (process.argv[3] ?? '1,7').split(',').map(Number);
for (const slug of ids) for (const level of levels) {
  const rows = wire.classes.filter((r: any) => r.classId === 'class:' + slug && r.level === level);
  const state = loadedClassFacts({ classes: rows } as any);
  const held = [{ classId: 'class:' + slug, classLabel: slug, level }];
  const w = summarizeWeaponProficiency(held, state);
  const c = summarizeCasterLevel(held, state);
  const s = classSkillLookup(held, state);
  const cs = SKILLS.filter((k) => s.isClassSkill(k.name)).map((k) => k.name);
  console.log(JSON.stringify({ slug, level, tiers: w.tiers.map((t) => t.label + ':' + t.proficient).join(' '), also: w.alsoProficientWith, printed: w.printed, wUnknown: w.unknown, casterLevel: c.display, classSkills: cs, csUnanswered: s.unanswered }));
}
