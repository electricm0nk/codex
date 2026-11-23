import { readFileSync } from 'node:fs';
import { assert } from '../testSupport/asserts';

// SD-43 E2.2: the Starfinder and Cyberpunk RED sheets render inside the shared frame (SheetFrame),
// not a page-flow <section> that scrolls the whole window. No DOM on this box: pin the source.
for (const file of ['StarfinderCharacterSheet.tsx', 'CyberpunkCharacterSheet.tsx']) {
  const source = readFileSync(new URL(`./${file}`, import.meta.url), 'utf8');
  assert(source.includes("import { SheetFrame } from './SheetFrame'"), `${file} imports SheetFrame`);
  assert(source.includes('<SheetFrame'), `${file} renders inside SheetFrame`);
  assert(!source.includes("marginTop: '1.5rem' }}>\n      <div style={{ alignItems"), `${file} no longer opens a page-flow section`);
}
console.log('SheetFrameSfCpr.test.ts: all assertions passed');
