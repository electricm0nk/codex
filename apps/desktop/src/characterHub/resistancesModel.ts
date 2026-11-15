import type { ResistancesDto } from '../boundary/loadSavedCharacterDetail';

/** The five energies, in the printed block's order (engine `resistances::ENERGIES`). */
const ENERGIES = ['Acid', 'Cold', 'Electricity', 'Fire', 'Sonic'] as const;

/** The screen's no-value mark (paper prints an empty box instead). */
const NONE = '—';

/**
 * The Defense tab's Resistances tiles from the load response's `resistances`, the boxes and words
 * the printed Resistances block (`print/blocks/core.rs` `resistances`) prints: each energy's amount,
 * Damage Reduction as `amount/bypass` or the record's words, Spell Resistance as the value or the
 * words, Immunities by name. Formats the engine's values; adds no number. `undefined` (no read yet)
 * → no tiles.
 */
export function resistanceTiles(resistances: ResistancesDto | undefined): { label: string; value: string }[] {
  if (!resistances) return [];
  const list = (values: string[]) => values.join(', ') || NONE;
  return [
    ...ENERGIES.map((energy) => {
      const held = resistances.energy.find((e) => e.energy === energy);
      return { label: energy, value: held ? String(held.amount) : NONE };
    }),
    { label: 'Damage Reduction', value: list(resistances.damageReduction.map((dr) => (dr.amount !== null ? `${dr.amount}/${dr.bypass}` : (dr.words ?? '')))) },
    { label: 'Spell Resistance', value: list(resistances.spellResistance.map((sr) => (sr.value !== null ? String(sr.value) : (sr.words ?? '')))) },
    { label: 'Immunities', value: list(resistances.immunities.map((immunity) => immunity.name)) },
  ];
}
