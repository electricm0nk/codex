import { useState, type CSSProperties, type ReactNode } from 'react';
import type { CharacterHubListRowSurface } from './buildCharacterHubListSurface';
import type { LoadSavedCharacterResponse } from '../boundary/loadSavedCharacterDetail';
import type { AbilityScoresDto } from '../boundary/loadCreateCharacter';
import { classLevelBenefit, parseHeldClasses, totalSkillPoints, type LevelBenefit } from './characterProgression';

/**
 * Pathfinder 1e character sheet, patterned after Pathbuilder 2e's three-column
 * layout: a left level-progression rail, a center stat column, and a right
 * tabbed content area. Real values (name, level, ability modifiers, AC, saves)
 * come from the loaded snapshot; slots we do not yet compute (feats, per-skill
 * training, weapons) render as "Not Selected" scaffolding.
 *
 * The sheet is game-system specific — this is the Pathfinder 1e variant; other
 * rule sets will supply their own sheet later.
 */

// ---------- shared bits ----------

function fmt(value: number): string {
  return value >= 0 ? `+${value}` : `${value}`;
}

function parseClassLevel(classSummary: string): { className: string; level: number } {
  const parts = classSummary.replace(/^class:/, '').split(':');
  const level = Number(parts.length > 1 ? parts[parts.length - 1] : '1') || 1;
  const className = parts.slice(0, -1).join(' ').replace(/\b\w/g, (c) => c.toUpperCase()) || 'Adventurer';
  return { className, level };
}

const ZERO_ABILITIES: AbilityScoresDto = {
  strength: 0,
  dexterity: 0,
  constitution: 0,
  intelligence: 0,
  wisdom: 0,
  charisma: 0,
};

const panel: CSSProperties = {
  backgroundColor: 'var(--color-surface)',
  border: '1px solid var(--color-border)',
  borderRadius: 10,
};

function ProficiencyBadge(props: { rank: 'U' | 'T' | 'E' }) {
  const trained = props.rank !== 'U';
  return (
    <span
      aria-hidden
      style={{
        alignItems: 'center',
        backgroundColor: trained ? 'var(--color-accent)' : 'var(--color-surface-2)',
        borderRadius: '50%',
        color: trained ? 'var(--color-on-accent)' : 'var(--color-text-muted)',
        display: 'inline-flex',
        flexShrink: 0,
        fontSize: '0.62rem',
        fontWeight: 800,
        height: 16,
        justifyContent: 'center',
        width: 16,
      }}
    >
      {props.rank}
    </span>
  );
}

function NavCard(props: { label: string; value: string }) {
  return (
    <div style={{ ...panel, padding: '0.55rem 0.75rem' }}>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.68rem', letterSpacing: '0.04em', margin: 0, textTransform: 'uppercase' }}>
        {props.label}
      </p>
      <p style={{ color: 'var(--color-text)', fontWeight: 700, margin: '0.1rem 0 0' }}>{props.value}</p>
    </div>
  );
}

function LevelBenefitCard(props: { benefit: LevelBenefit; skillPoints: number; variant: 'current' | 'next' }) {
  const { benefit, variant } = props;
  return (
    <div
      style={{
        ...panel,
        backgroundColor: variant === 'next' ? 'var(--color-surface-2)' : 'var(--color-surface)',
        borderStyle: variant === 'next' ? 'dashed' : 'solid',
        padding: '0.55rem 0.7rem',
      }}
    >
      <p style={{ color: 'var(--color-accent)', fontSize: '0.78rem', fontWeight: 800, letterSpacing: '0.02em', margin: 0 }}>
        Level {benefit.level} {benefit.classLabel}
      </p>
      <p style={{ color: 'var(--color-text-muted)', fontSize: '0.72rem', margin: '0.15rem 0 0.25rem' }}>
        Skill points: {props.skillPoints}
      </p>
      <ul style={{ margin: 0, paddingLeft: '1.1rem' }}>
        {benefit.features.map((feature) => (
          <li key={feature} style={{ color: 'var(--color-text-secondary)', fontSize: '0.8rem', marginBottom: '0.1rem' }}>
            {feature}
          </li>
        ))}
      </ul>
    </div>
  );
}

// ---------- center column ----------

const ABILITY_COLUMNS: ReadonlyArray<{ key: keyof AbilityScoresDto; label: string }> = [
  { key: 'strength', label: 'STR' },
  { key: 'dexterity', label: 'DEX' },
  { key: 'constitution', label: 'CON' },
  { key: 'intelligence', label: 'INT' },
  { key: 'wisdom', label: 'WIS' },
  { key: 'charisma', label: 'CHA' },
];

// Governing ability per skill — used only to seed a plausible untrained modifier
// in the scaffold. Real per-skill training is not yet computed.
const SKILLS: ReadonlyArray<{ name: string; ability: keyof AbilityScoresDto; rank: 'U' | 'T' | 'E' }> = [
  { name: 'Acrobatics', ability: 'dexterity', rank: 'U' },
  { name: 'Arcana', ability: 'intelligence', rank: 'U' },
  { name: 'Athletics', ability: 'strength', rank: 'U' },
  { name: 'Crafting', ability: 'intelligence', rank: 'U' },
  { name: 'Deception', ability: 'charisma', rank: 'U' },
  { name: 'Diplomacy', ability: 'charisma', rank: 'U' },
  { name: 'Intimidation', ability: 'charisma', rank: 'U' },
  { name: 'Medicine', ability: 'wisdom', rank: 'U' },
  { name: 'Nature', ability: 'wisdom', rank: 'U' },
  { name: 'Occultism', ability: 'intelligence', rank: 'U' },
  { name: 'Performance', ability: 'charisma', rank: 'U' },
  { name: 'Religion', ability: 'wisdom', rank: 'U' },
  { name: 'Society', ability: 'intelligence', rank: 'U' },
  { name: 'Stealth', ability: 'dexterity', rank: 'U' },
  { name: 'Survival', ability: 'wisdom', rank: 'U' },
  { name: 'Thievery', ability: 'dexterity', rank: 'U' },
];

function StatColumn(props: { name: string; level: number; abilities: AbilityScoresDto; ac: number; saves: { fortitude: number; reflex: number; will: number } }) {
  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '0.75rem' }}>
      {/* Identity header */}
      <div style={{ ...panel, display: 'flex', gap: '0.5rem', padding: '0.6rem' }}>
        <div style={{ ...panel, backgroundColor: 'var(--color-surface-2)', flex: '0 0 auto', padding: '0.3rem 0.5rem', textAlign: 'center' }}>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.6rem', margin: 0 }}>Level</p>
          <p style={{ fontWeight: 800, margin: 0 }}>{props.level}</p>
        </div>
        <div style={{ ...panel, backgroundColor: 'var(--color-surface-2)', flex: '0 0 auto', padding: '0.3rem 0.5rem', textAlign: 'center' }}>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.6rem', margin: 0 }}>XP</p>
          <p style={{ fontWeight: 800, margin: 0 }}>0</p>
        </div>
        <div style={{ flex: 1, minWidth: 0 }}>
          <p style={{ color: 'var(--color-text-muted)', fontSize: '0.6rem', margin: 0 }}>Character Name</p>
          <p style={{ fontWeight: 700, margin: '0.1rem 0 0', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{props.name}</p>
        </div>
      </div>

      {/* Size / speed / abilities */}
      <div style={{ ...panel, padding: '0.6rem' }}>
        <div style={{ display: 'flex', gap: '1rem', marginBottom: '0.5rem' }}>
          <span style={{ fontSize: '0.8rem' }}><strong style={{ color: 'var(--color-accent)' }}>SIZE</strong> Medium</span>
          <span style={{ fontSize: '0.8rem' }}><strong style={{ color: 'var(--color-accent)' }}>SPEED</strong> 25ft.</span>
        </div>
        <div style={{ display: 'grid', gap: '0.25rem', gridTemplateColumns: 'repeat(6, 1fr)', textAlign: 'center' }}>
          {ABILITY_COLUMNS.map((col) => (
            <div key={col.key}>
              <p style={{ color: 'var(--color-text-muted)', fontSize: '0.6rem', fontWeight: 700, margin: 0 }}>{col.label}</p>
              <p style={{ fontWeight: 800, margin: 0 }}>{fmt(props.abilities[col.key])}</p>
            </div>
          ))}
        </div>
      </div>

      {/* Defense */}
      <div style={{ ...panel, display: 'flex', gap: '0.75rem', padding: '0.6rem' }}>
        <div
          style={{
            alignItems: 'center',
            border: '2px solid var(--color-accent)',
            borderRadius: '8px 8px 20px 20px',
            display: 'flex',
            flexDirection: 'column',
            justifyContent: 'center',
            padding: '0.35rem 0.6rem',
          }}
        >
          <span style={{ color: 'var(--color-text-muted)', fontSize: '0.6rem', fontWeight: 700 }}>AC</span>
          <span style={{ fontSize: '1.4rem', fontWeight: 800 }}>{props.ac}</span>
        </div>
        <div style={{ display: 'flex', flex: 1, flexDirection: 'column', gap: '0.35rem' }}>
          <div style={{ ...panel, backgroundColor: 'var(--color-surface-2)', fontSize: '0.8rem', padding: '0.25rem 0.5rem' }}>HP 19/19</div>
          <div style={{ ...panel, backgroundColor: 'var(--color-surface-2)', color: 'var(--color-text-muted)', fontSize: '0.8rem', padding: '0.25rem 0.5rem' }}>No Shield</div>
        </div>
        <div style={{ display: 'flex', flexDirection: 'column', fontSize: '0.8rem', gap: '0.25rem', justifyContent: 'center' }}>
          <span><strong>{fmt(props.saves.fortitude)}</strong> Fortitude</span>
          <span><strong>{fmt(props.saves.reflex)}</strong> Reflex</span>
          <span><strong>{fmt(props.saves.will)}</strong> Will</span>
        </div>
      </div>

      {/* Perception + DC */}
      <div style={{ ...panel, display: 'flex', gap: '0.5rem', justifyContent: 'space-between', padding: '0.5rem 0.75rem' }}>
        <span style={{ fontSize: '0.85rem' }}><ProficiencyBadge rank="T" /> <strong> 16</strong> Class DC</span>
        <span style={{ fontSize: '0.85rem' }}><ProficiencyBadge rank="E" /> <strong> {fmt(5)}</strong> Perception</span>
      </div>

      {/* Skills */}
      <div style={{ ...panel, padding: '0.5rem 0.75rem' }}>
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.68rem', letterSpacing: '0.06em', margin: '0 0 0.4rem', textTransform: 'uppercase' }}>
          Skills
        </p>
        <div style={{ display: 'flex', flexDirection: 'column', gap: '0.15rem' }}>
          {SKILLS.map((skill) => (
            <div key={skill.name} style={{ alignItems: 'center', display: 'flex', fontSize: '0.85rem', gap: '0.4rem' }}>
              <ProficiencyBadge rank={skill.rank} />
              <span style={{ color: 'var(--color-text-secondary)', width: 34 }}>{fmt(props.abilities[skill.ability])}</span>
              <span>{skill.name}</span>
            </div>
          ))}
        </div>
        <p style={{ color: 'var(--color-text-faint)', fontSize: '0.7rem', margin: '0.5rem 0 0' }}>
          Untrained scaffold — per-skill training is not yet computed.
        </p>
      </div>
    </div>
  );
}

// ---------- right column ----------

const TABS = ['Weapons', 'Defense', 'Gear', 'Spells', 'Pets', 'Details', 'Feats', 'Actions'] as const;
type Tab = (typeof TABS)[number];

function toolbarButton(label: string): ReactNode {
  return (
    <button
      key={label}
      type="button"
      style={{
        backgroundColor: 'var(--color-surface)',
        border: '1px solid var(--color-border)',
        borderRadius: 8,
        color: 'var(--color-text)',
        cursor: 'pointer',
        flex: 1,
        fontSize: '0.9rem',
        padding: '0.55rem 1rem',
      }}
    >
      {label}
    </button>
  );
}

function WeaponsTab() {
  return (
    <div>
      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '1.25rem', justifyContent: 'center', marginBottom: '1rem' }}>
        {([['E', 'Simple Weapons'], ['E', 'Martial Weapons'], ['T', 'Advanced Weapons'], ['E', 'Unarmed Attacks']] as const).map(([rank, label]) => (
          <span key={label} style={{ alignItems: 'center', display: 'flex', fontSize: '0.9rem', gap: '0.4rem' }}>
            <ProficiencyBadge rank={rank} /> {label}
          </span>
        ))}
      </div>
      <div style={{ display: 'flex', gap: '0.6rem', justifyContent: 'center', marginBottom: '1.25rem' }}>
        <button type="button" style={{ backgroundColor: 'var(--color-surface)', border: '1px solid var(--color-border)', borderRadius: 8, color: 'var(--color-text)', cursor: 'pointer', padding: '0.5rem 1.5rem' }}>
          Add Weapon
        </button>
        <button type="button" style={{ backgroundColor: 'var(--color-surface)', border: '1px solid var(--color-border)', borderRadius: 8, color: 'var(--color-text)', cursor: 'pointer', padding: '0.5rem 1.5rem' }}>
          Print
        </button>
      </div>
      <p style={{ color: 'var(--color-text-faint)', margin: 0, textAlign: 'center' }}>No weapons added yet.</p>
    </div>
  );
}

// ---------- root ----------

export function CharacterSheet(props: {
  row: CharacterHubListRowSurface;
  detail: LoadSavedCharacterResponse | null;
  onClose: () => void;
}) {
  const [tab, setTab] = useState<Tab>('Weapons');

  const { className, level } = parseClassLevel(props.row.classSummary);
  const snapshot = props.detail?.snapshot ?? null;
  const abilities = snapshot?.abilityModifiers ?? ZERO_ABILITIES;
  const ac = snapshot?.baselineArmorClass ?? 10;
  const saves = snapshot?.totalSaves ?? { fortitude: 0, reflex: 0, will: 0 };

  const heldClasses = parseHeldClasses(props.row.classSummary);
  const isHuman = props.row.raceLabel.toLowerCase() === 'human';
  const skillPointsFor = (benefit: LevelBenefit) => totalSkillPoints(benefit.skillPointsBase, abilities.intelligence, isHuman);

  // Every level already taken, per held class, in order.
  const currentBenefits = heldClasses.flatMap((held) =>
    Array.from({ length: held.level }, (_, index) => classLevelBenefit(held.classId, held.classLabel, index + 1))
  );
  // The next level of each held class (the multiclass "what could I take next" set).
  const nextBenefits = heldClasses.map((held) => classLevelBenefit(held.classId, held.classLabel, held.level + 1));

  return (
    <div style={{ marginLeft: 'calc(50% - 50vw)', width: '100vw' }}>
      {/* Top menu bar */}
      <div
        style={{
          alignItems: 'center',
          borderBottom: '2px solid var(--color-accent)',
          display: 'flex',
          gap: '1.5rem',
          padding: '0.6rem 1.5rem',
        }}
      >
        <span style={{ fontSize: '1.1rem', fontWeight: 700 }}>☰ Menu</span>
        <button
          type="button"
          onClick={props.onClose}
          title="Close character"
          style={{ background: 'none', border: 'none', color: 'var(--color-text-muted)', cursor: 'pointer', fontSize: '1rem' }}
        >
          ✕
        </button>
        <span style={{ fontWeight: 600 }}>
          {props.row.displayLabel} — {className} {level}
        </span>
      </div>

      {/* Three columns */}
      <div style={{ display: 'flex', gap: '0.75rem', padding: '1rem 1.5rem' }}>
        {/* Left: level progression */}
        <div style={{ flex: '0 0 250px' }}>
          <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
            <NavCard label="Ancestry" value={props.row.raceLabel} />
            <NavCard label="Class" value={className} />
          </div>

          {/* Current levels — each level taken and what it granted */}
          <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem', marginTop: '1rem' }}>
            {currentBenefits.map((benefit) => (
              <LevelBenefitCard
                key={`${benefit.classId}-${benefit.level}`}
                benefit={benefit}
                skillPoints={skillPointsFor(benefit)}
                variant="current"
              />
            ))}
          </div>

          {/* Next — the next level available for each held class */}
          <div style={{ alignItems: 'center', display: 'flex', gap: '0.5rem', margin: '1rem 0 0.6rem' }}>
            <span style={{ borderTop: '1px solid var(--color-border)', flex: 1 }} />
            <span style={{ color: 'var(--color-accent)', fontSize: '0.8rem', fontWeight: 800, letterSpacing: '0.08em', textTransform: 'uppercase' }}>
              Next
            </span>
            <span style={{ borderTop: '1px solid var(--color-border)', flex: 1 }} />
          </div>
          <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
            {nextBenefits.map((benefit) => (
              <LevelBenefitCard
                key={`next-${benefit.classId}-${benefit.level}`}
                benefit={benefit}
                skillPoints={skillPointsFor(benefit)}
                variant="next"
              />
            ))}
          </div>
        </div>

        {/* Center: stats */}
        <div style={{ flex: '0 0 300px' }}>
          <StatColumn name={props.row.displayLabel} level={level} abilities={abilities} ac={ac} saves={saves} />
        </div>

        {/* Right: tabbed content */}
        <div style={{ flex: 1, minWidth: 0 }}>
          <div style={{ display: 'flex', gap: '0.6rem', marginBottom: '1rem' }}>
            {['Rest', 'Add Condition', 'Add Custom Buff'].map((label) => toolbarButton(label))}
          </div>

          <div style={{ borderBottom: '1px solid var(--color-border)', display: 'flex', gap: '1.25rem', marginBottom: '1.25rem' }}>
            {TABS.map((name) => {
              const active = name === tab;
              return (
                <button
                  key={name}
                  type="button"
                  onClick={() => setTab(name)}
                  style={{
                    background: 'none',
                    border: 'none',
                    borderBottom: `2px solid ${active ? 'var(--color-accent)' : 'transparent'}`,
                    color: active ? 'var(--color-text)' : 'var(--color-text-muted)',
                    cursor: 'pointer',
                    fontSize: '0.95rem',
                    fontWeight: active ? 700 : 500,
                    padding: '0.3rem 0.1rem 0.6rem',
                  }}
                >
                  {name}
                </button>
              );
            })}
          </div>

          <div style={{ ...panel, minHeight: 240, padding: '1.25rem' }}>
            {tab === 'Weapons' ? (
              <WeaponsTab />
            ) : (
              <p style={{ color: 'var(--color-text-faint)', margin: 0, textAlign: 'center' }}>{tab} — coming soon.</p>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
