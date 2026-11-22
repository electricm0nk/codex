import { useState } from 'react';
import { MarkdownEditor } from '../../editor/MarkdownEditor';
import { campaignImageUrl, createPartyResourcesStarter, readCampaignFile, statCampaignFile, writeCampaignFile } from '../../boundary/campaignFolder/campaignFiles';
import { saveAttachment } from '../../boundary/campaignFolder/attachments';
import type { CampaignTreeEntry } from '../../boundary/campaignFolder/types';
import { markdownStems, PARTY_RESOURCES_PATH } from './partyResourcesModel';
import { resolveWikiLink } from '../../boundary/campaignFolder/wiki';

/**
 * Party Resources (SD-42 `decisions.md §15.1`): `Party Resources.md` in the campaign editor. Nothing
 * parses it. A folder made by hand has no such file; the tab offers to write the starter. A clicked
 * `[[link]]` opens the linked page here, with a way back.
 */

const button = { background: 'none', border: '1px solid var(--color-border)', borderRadius: 6, cursor: 'pointer', padding: '0.35rem 0.8rem' } as const;

export function PartyResourcesTab(props: { campaignDir: string; tree: readonly CampaignTreeEntry[]; onTreeChanged: () => void }) {
  const { campaignDir, tree } = props;
  const [path, setPath] = useState(PARTY_RESOURCES_PATH);
  const [notice, setNotice] = useState<string | null>(null);
  const [creating, setCreating] = useState(false);
  const exists = tree.some((entry) => !entry.isDir && entry.path === PARTY_RESOURCES_PATH);

  if (!exists && path === PARTY_RESOURCES_PATH) {
    return (
      <div data-testid="party-resources-missing">
        <p style={{ marginTop: 0 }}>This folder has no {PARTY_RESOURCES_PATH}. Codex can write the starter: a shared-items table and a party-gold table you then edit freely.</p>
        <button
          type="button"
          disabled={creating}
          style={button}
          onClick={() => {
            setCreating(true);
            setNotice(null);
            createPartyResourcesStarter(campaignDir)
              .then(props.onTreeChanged)
              .catch((cause: unknown) => setNotice(cause instanceof Error ? cause.message : String(cause)))
              .finally(() => setCreating(false));
          }}
        >
          Create starter file
        </button>
        {notice && <p role="alert" style={{ color: 'var(--color-error)' }}>{notice}</p>}
      </div>
    );
  }

  return (
    <div data-testid="party-resources">
      {path !== PARTY_RESOURCES_PATH && (
        <div style={{ alignItems: 'center', display: 'flex', gap: '0.75rem', marginBottom: '0.6rem' }}>
          <button type="button" style={button} onClick={() => setPath(PARTY_RESOURCES_PATH)}>
            ← Party Resources
          </button>
          <span style={{ color: 'var(--color-text-muted)', fontSize: '0.85rem' }}>{path}</span>
        </div>
      )}
      {notice && <p role="status" style={{ color: 'var(--color-text-muted)', marginTop: 0 }}>{notice}</p>}
      <MarkdownEditor
        key={path}
        campaignDir={campaignDir}
        path={path}
        readFile={readCampaignFile}
        writeFile={writeCampaignFile}
        statFile={statCampaignFile}
        saveAttachment={saveAttachment}
        resolveImage={(image) => campaignImageUrl(campaignDir, image)}
        pages={async () => markdownStems(tree)}
        onOpenLink={(target) =>
          resolveWikiLink(campaignDir, target)
            .then((found) => {
              setNotice(found ? null : `No page named "${target}" in this campaign yet.`);
              if (found) setPath(found);
            })
            .catch((cause: unknown) => setNotice(cause instanceof Error ? cause.message : String(cause)))
        }
      />
    </div>
  );
}
