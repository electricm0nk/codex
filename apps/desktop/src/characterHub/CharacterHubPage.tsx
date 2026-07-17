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
import { RaceCatalogScreen } from '../raceCatalog/RaceCatalogScreen';
import { StubScreen } from './StubScreen';
import { isGoogleDriveConfigured } from '../settings/googleDrive';
import { CampaignManagerScreen } from '../campaign/CampaignManagerScreen';
import { CreateCampaignScreen } from '../campaign/CreateCampaignScreen';
import { EditCampaignScreen } from '../campaign/EditCampaignScreen';
import { CampaignSheet } from '../campaign/CampaignSheet';

type Mode =
  | 'landing'
  | 'load'
  | 'create'
  | 'sheet'
  | 'equipmentCatalog'
  | 'spellCatalog'
  | 'classCatalog'
  | 'raceCatalog'
  | 'dm-toolkit'
  | 'campaign-list'
  | 'campaign-create'
  | 'campaign-edit'
  | 'campaign-sheet';

export function CharacterHubPage(props: { onOpenTool?: (tool: 'update' | 'bug' | 'enhancement') => void }) {
  const [mode, setMode] = useState<Mode>('landing');
  const [ruleSet, setRuleSet] = useState<RuleSetId>('pathfinder-1e');
  const [sheet, setSheet] = useState<{ row: CharacterHubListRowSurface; detail: LoadSavedCharacterResponse | null } | null>(null);
  // Where the ✕ on the character sheet should return to — Load Character
  // normally, but the campaign screen when opened from a party member there.
  const [sheetReturnMode, setSheetReturnMode] = useState<Mode>('load');
  const [activeCampaignId, setActiveCampaignId] = useState<string | null>(null);
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
        onBrowseRaces={() => setMode('raceCatalog')}
        onCampaignManager={() => setMode('campaign-list')}
        campaignManagerEnabled={isGoogleDriveConfigured()}
        onDmToolkit={() => setMode('dm-toolkit')}
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

  if (mode === 'raceCatalog') {
    return <RaceCatalogScreen onClose={() => setMode('landing')} />;
  }

  if (mode === 'dm-toolkit') {
    return (
      <StubScreen
        title="DM Toolkit"
        description="Encounter building, initiative tracking, and other GM-side tools. Not built yet."
        onBack={() => setMode('landing')}
      />
    );
  }

  if (mode === 'campaign-list') {
    return (
      <CampaignManagerScreen
        onCancel={() => setMode('landing')}
        onCreate={() => setMode('campaign-create')}
        onEdit={(campaignId) => {
          setActiveCampaignId(campaignId);
          setMode('campaign-edit');
        }}
        onLoad={(campaignId) => {
          setActiveCampaignId(campaignId);
          setMode('campaign-sheet');
        }}
      />
    );
  }

  if (mode === 'campaign-create') {
    return (
      <CreateCampaignScreen
        onCancel={() => setMode('campaign-list')}
        onCreated={(campaignId) => {
          setActiveCampaignId(campaignId);
          setMode('campaign-edit');
        }}
      />
    );
  }

  if (mode === 'campaign-edit' && activeCampaignId) {
    return (
      <EditCampaignScreen
        campaignId={activeCampaignId}
        onCancel={() => setMode('campaign-list')}
        onSaved={() => setMode('campaign-list')}
      />
    );
  }

  if (mode === 'campaign-sheet' && activeCampaignId) {
    return (
      <CampaignSheet
        campaignId={activeCampaignId}
        onClose={() => setMode('campaign-list')}
        onOpenCharacterSheet={(row, detail) => {
          setSheet({ row, detail });
          setSheetReturnMode('campaign-sheet');
          setMode('sheet');
        }}
      />
    );
  }

  if (mode === 'sheet' && sheet) {
    return <CharacterSheet row={sheet.row} detail={sheet.detail} onClose={() => setMode(sheetReturnMode)} onOpenTool={props.onOpenTool} />;
  }

  if (mode === 'load') {
    return (
      <LoadCharacterScreen
        surface={surface}
        error={error}
        onCancel={() => setMode('landing')}
        onOpenSheet={(row, detail) => {
          setSheet({ row, detail });
          setSheetReturnMode('load');
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
