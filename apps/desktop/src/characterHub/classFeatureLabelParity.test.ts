import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { buildClassFeatureSurface } from './classFeaturesModel';
import { assertEqual } from '../testSupport/asserts';
import type { ExplanationDto } from '../boundary/loadSavedCharacterDetail';
import type { HeldClass } from './characterProgression';

// The Class Features panel and the printed sheet label a class record by one rule: the panel's
// `splitId`/`humanise` here, its port in the print (`src-tauri/src/print/blocks/feats.rs`). Both are
// held to the same fixture of every Pathfinder seed's panel rows, `print/golden/class_feature_labels.json`;
// the Rust side's `print_pf_blocks` test holds the fixture's ids and values to the seeds' own load.

interface FixtureRow {
  id: string;
  value: number;
  classLabel: string | null;
  label: string;
}

interface FixtureSeed {
  heldClasses: { classId: string; classLabel: string }[];
  features: FixtureRow[];
  notComputed: string[];
}

const fixture = JSON.parse(
  readFileSync(
    join(dirname(fileURLToPath(import.meta.url)), '..', '..', 'src-tauri', 'src', 'print', 'golden', 'class_feature_labels.json'),
    'utf8',
  ),
) as Record<string, FixtureSeed>;

const seeds = Object.keys(fixture);
assertEqual(seeds.join(','), 'aldric,elowen,tomas', 'the fixture holds the three Pathfinder seeds');

for (const seed of seeds) {
  const entry = fixture[seed];
  const heldClasses: HeldClass[] = entry.heldClasses.map((held) => ({ ...held, level: 1 }));
  const explanations: ExplanationDto[] = [
    ...entry.features.map((row) => ({ id: row.id, value: row.value, detail: 'engine detail' })),
    ...entry.notComputed.map((id) => ({ id, value: 0, detail: 'not grounded' })),
    // A record of another family, which the panel never lists.
    { id: 'skill.total.climb', value: 7, detail: 'not a class record' },
  ];
  const surface = buildClassFeatureSurface(explanations, heldClasses);
  assertEqual(surface.features.length, entry.features.length, `${seed}: the panel lists every fixture row`);
  surface.features.forEach((row, index) => {
    const expected = entry.features[index];
    assertEqual(row.id, expected.id, `${seed}: row ${index} is the fixture's, in order`);
    assertEqual(row.label, expected.label, `${seed}: ${row.id} carries the print's label`);
    assertEqual(row.classLabel, expected.classLabel, `${seed}: ${row.id} carries the print's class`);
    assertEqual(row.value, expected.value, `${seed}: ${row.id} carries its value`);
  });
}

console.log('classFeatureLabelParity.test.ts: all assertions passed');
