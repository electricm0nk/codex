import { resistanceTiles } from './resistancesModel';
import type { ResistancesDto } from '../boundary/loadSavedCharacterDetail';
import { assertEqual } from '../testSupport/asserts';

// SD-40 E2.26 (decisions.md §24.3.8): the Defense tab's Resistances panel shows the load
// response's `resistances` (E2.21) as the printed Resistances block does (print/blocks/core.rs
// `resistances`): five energy tiles, Damage Reduction, Spell Resistance, Immunities. It formats the
// engine's values; it adds no number. The screen's no-value mark is `—` (paper keeps a blank box).

const empty = (): ResistancesDto => ({ energy: [], damageReduction: [], spellResistance: [], immunities: [] });
const show = (r: ResistancesDto | undefined) => resistanceTiles(r).map((t) => `${t.label}=${t.value}`).join('|');

// Nothing held: eight tiles, every one the no-value mark.
assertEqual(
  show(empty()),
  'Acid=—|Cold=—|Electricity=—|Fire=—|Sonic=—|Damage Reduction=—|Spell Resistance=—|Immunities=—',
  'empty: five energy tiles and three more, all —',
);

// An Ifrit: fire resistance 5 from its race trait.
assertEqual(
  show({ ...empty(), energy: [{ energy: 'Fire', amount: 5, source: 'race_trait.ifrit.energy_resistance' }] }),
  'Acid=—|Cold=—|Electricity=—|Fire=5|Sonic=—|Damage Reduction=—|Spell Resistance=—|Immunities=—',
  'Ifrit: Fire 5, the rest —',
);

// A Barbarian 7: DR 1/— (the engine's bypass word, as given).
assertEqual(
  resistanceTiles({ ...empty(), damageReduction: [{ amount: 1, bypass: '—', words: null, source: 'class_feature.barbarian.damage_reduction' }] })[5].value,
  '1/—',
  'Barbarian: DR 1/—',
);

// An SR the engine cannot evaluate prints the record's words; a number prints as is; several join with ", ".
assertEqual(
  resistanceTiles({
    ...empty(),
    spellResistance: [
      { value: null, words: 'spell resistance equal to 6 + class level', source: 'x' },
      { value: 13, words: null, source: 'y' },
    ],
  })[6].value,
  'spell resistance equal to 6 + class level, 13',
  'SR: words when unevaluated, then the number',
);

// DR in words; immunities by name, one comma list.
const words = resistanceTiles({
  ...empty(),
  damageReduction: [{ amount: null, bypass: '', words: 'DR equal to half your level', source: 'z' }],
  immunities: [{ name: 'Immunity to Fire', source: 'a' }, { name: 'Poison Immunity', source: 'b' }],
});
assertEqual(words[5].value, 'DR equal to half your level', 'DR words');
assertEqual(words[7].value, 'Immunity to Fire, Poison Immunity', 'immunities joined');

// No read yet: no tiles (the panel renders nothing rather than claiming "none").
assertEqual(resistanceTiles(undefined).length, 0, 'absent: no tiles');

console.log('all assertions passed');
