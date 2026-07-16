import { useEffect, useState } from 'react';
import { loadCharacterHubListSurfaceRuntime } from './characterHubRuntime';
import type { CharacterHubListSurface, CharacterHubListRowSurface } from './buildCharacterHubListSurface';
import type { LoadSavedCharacterResponse } from '../boundary/loadSavedCharacterDetail';
import { CreateCharacterForm } from './CreateCharacterForm';
import { LandingScreen, type RuleSetId } from './LandingScreen';
import { LoadCharacterScreen } from './LoadCharacterScreen';
import { CharacterSheet } from './CharacterSheet';
import { EquipmentCatalogScreen } from '../equipmentCatalog/EquipmentCatalogScreen';
import { SpellCatalogScreen } from '../spellCatalog/SpellCatalogScreen';
import { ClassCatalogScreen } from '../classCatalog/ClassCatalogScreen';

export function CharacterHubPage(props: { onOpenTool?: (tool: 'update' | 'bug' | 'enhancement') => void }) {
  const [mode, setMode] = useState<'landing' | 'load' | 'create' | 'sheet' | 'equipmentCatalog' | 'spellCatalog' | 'classCatalog'>('landing');
  const [ruleSet, setRuleSet] = useState<RuleSetId>('pathfinder-1e');
  const [sheet, setSheet] = useState<{ row: CharacterHubListRowSurface; detail: LoadSavedCharacterResponse | null } | null>(null);
  const [surface, setSurface] = useState<CharacterHubListSurface | null>(null);
  const [error, setError] = useState<string | null>(null);

  function reload() {
    loadCharacterHubListSurfaceRuntime()
      .then(setSurface)
      .catch((cause: unknown) => {
        setError(cause instanceof Error ? cause.message : 'Unknown character hub failure');
      });
  }

  useEffect(() => {
    reload();
  }, []);

  const hasCharacters = Boolean(surface && !surface.isEmpty && surface.rows.length > 0);

  if (mode === 'landing') {
    return (
      <LandingScreen
        selectedRuleSet={ruleSet}
        onSelectRuleSet={setRuleSet}
        onCreate={() => setMode('create')}
        onLoad={() => setMode('load')}
        onLoadMostRecent={() => setMode('load')}
        onBrowseEquipment={() => setMode('equipmentCatalog')}
        onBrowseSpells={() => setMode('spellCatalog')}
        onBrowseClasses={() => setMode('classCatalog')}
        hasCharacters={hasCharacters}
      />
    );
  }

  if (mode === 'equipmentCatalog') {
    return <EquipmentCatalogScreen onClose={() => setMode('landing')} />;
  }

  if (mode === 'spellCatalog') {
    return <SpellCatalogScreen onClose={() => setMode('landing')} />;
  }

  if (mode === 'classCatalog') {
    return <ClassCatalogScreen onClose={() => setMode('landing')} />;
  }

  if (mode === 'sheet' && sheet) {
    return <CharacterSheet row={sheet.row} detail={sheet.detail} onClose={() => setMode('load')} onOpenTool={props.onOpenTool} />;
  }

  if (mode === 'load') {
    return (
      <LoadCharacterScreen
        surface={surface}
        error={error}
        onCancel={() => setMode('landing')}
        onOpenSheet={(row, detail) => {
          setSheet({ row, detail });
          setMode('sheet');
        }}
      />
    );
  }

  return (
    <section style={{ marginTop: '2rem' }}>
      <div style={{ alignItems: 'center', display: 'flex', justifyContent: 'space-between', marginBottom: '1rem' }}>
        <h2 style={{ margin: 0 }}>Create a character</h2>
        <button
          type="button"
          onClick={() => setMode('landing')}
          style={{ background: 'none', border: '1px solid var(--color-border)', borderRadius: 8, cursor: 'pointer', padding: '0.5rem 1rem' }}
        >
          Back
        </button>
      </div>
      {/* Refresh the list data in the background so it's current whenever the
          user chooses to go back — but stay on the form so they can see the
          computed character sheet (or blocked diagnostics) the submit produced. */}
      <CreateCharacterForm onCreated={reload} />
    </section>
  );
}
