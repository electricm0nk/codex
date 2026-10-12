import type { CSSProperties } from 'react';
import { buildAttackTiles } from '../attackPanelModel';
import { SHEET_LAYOUT, layoutBox } from '../sheetLayout';
import { StatBox, StatTile, fmt, fmtOrAbsent } from './StatBox';

/**
 * The sheet's combat boxes, each its own titled box (Brief 2 L1: AC, Attack, Speed, Saves stack
 * one per box, in the order `SHEET_LAYOUT` gives). `boxStyle` defaults to the sheet registry's.
 */

/**
 * `touch` is the engine's `defense.touch_armor_class` and `flatFooted` is its
 * `defense.flat_footed_armor_class` — neither is derived here. This panel used
 * to compute both: touch as `10 + dexMod`, which is why it could display
 * `AC 19` beside `TOUCH 14` with a 4-point armor bonus, and flat-footed as
 * `ac - Math.max(0, dexMod)`, which ignored PF1's rule that a flat-footed
 * character loses dodge bonuses along with the Dexterity bonus. See SD-27
 * `decisions.md §28`.
 */
export function ArmorClassPanel(props: { ac: number; touch: number | null; flatFooted: number | null; boxStyle?: CSSProperties }) {
  return (
    <StatBox title="Armor Class">
      <div style={props.boxStyle ?? layoutBox(SHEET_LAYOUT, 'armorClass').boxStyle}>
        <StatTile label="AC" value={props.ac} emphasize />
        <StatTile label="Touch" value={fmtOrAbsent(props.touch, false)} />
        <StatTile label="Flat-Footed" value={fmtOrAbsent(props.flatFooted, false)} />
      </div>
    </StatBox>
  );
}

export function SavingThrowsPanel(props: { saves: { fortitude: number; reflex: number; will: number }; boxStyle?: CSSProperties }) {
  return (
    <StatBox title="Saving Throws">
      <div style={props.boxStyle ?? layoutBox(SHEET_LAYOUT, 'saves').boxStyle}>
        <StatTile label="Fortitude" value={fmt(props.saves.fortitude)} />
        <StatTile label="Reflex" value={fmt(props.saves.reflex)} />
        <StatTile label="Will" value={fmt(props.saves.will)} />
      </div>
    </StatBox>
  );
}

/**
 * `land` is the race's real base land speed, read off the same
 * corpus-derived roster the creation picker is built from.
 *
 * It used to be the literal string `"30 ft."` for every character, which was
 * already wrong for Dwarf, Gnome and Halfling (20 ft.) among the races that
 * shipped, and is wrong for Duergar and Svirfneblin (20 ft.) and Merfolk
 * (5 ft.) among the Bestiary 1 races creation now offers. The other three
 * movement modes stay `—`: no ingested race declares a fly, swim or climb
 * speed, and this panel will not invent one.
 */
export function SpeedPanel(props: { land: string; boxStyle?: CSSProperties }) {
  return (
    <StatBox title="Speed">
      <div style={props.boxStyle ?? layoutBox(SHEET_LAYOUT, 'speed').boxStyle}>
        <StatTile label="Land" value={props.land} />
        <StatTile label="Fly" value="—" />
        <StatTile label="Swim" value="—" />
        <StatTile label="Climb" value="—" />
      </div>
    </StatBox>
  );
}

/**
 * `cmb`/`cmd` are the engine's `combat.combat_maneuver_bonus` /
 * `defense.combat_maneuver_defense`. They used to be computed here as
 * `bab + str` and `10 + bab + str + dexMod` — two PF1 formulas living in a
 * React component, neither aware of creature size, so every Small character's
 * CMB and CMD were a point too high. See SD-27 `decisions.md §28`.
 */
export function AttackPanel(props: { baseAttackBonus: number; melee: number; cmb: number | null; cmd: number | null; boxStyle?: CSSProperties }) {
  // v0.8 F-9: `melee` is the engine's `baselineMeleeAttackBonus`, which was
  // loaded on every sheet and never rendered. v0.8 F-13: the old "Spell
  // Res." tile here was a hard-coded "—" nothing could populate; the real
  // spell resistance renders on the Defense tab from
  // `equipmentEffects.spellResistanceTotal` when a build has one.
  const tiles = buildAttackTiles({ baseAttackBonus: props.baseAttackBonus, melee: props.melee, cmb: props.cmb, cmd: props.cmd });
  return (
    <StatBox title="Attack">
      <div style={props.boxStyle ?? layoutBox(SHEET_LAYOUT, 'attack').boxStyle}>
        {tiles.map((tile) => (
          <StatTile key={tile.label} label={tile.label} value={tile.value} />
        ))}
      </div>
    </StatBox>
  );
}
