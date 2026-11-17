import { useEffect, useState } from 'react';
import { MarkdownEditor } from '../../editor/MarkdownEditor';
import { campaignImageUrl, readCampaignFile, statCampaignFile, writeCampaignFile } from '../../boundary/campaignFolder/campaignFiles';
import { saveAttachment } from '../../boundary/campaignFolder/attachments';
import { listAdventureLog, newAdventureLogEntry, type AdventureLogEntry } from '../../boundary/campaignFolder/adventureLog';
import type { CampaignTreeEntry } from '../../boundary/campaignFolder/types';
import { localIsoDate, nextSessionTitle, sessionLabel, statusMark } from '../adventureLogModel';
import { readYourName, rememberYourName } from '../campaignMembersModel';
import { linkTarget, markdownStems } from './partyResourcesModel';

/**
 * Adventure Log (SD-42 `decisions.md §15.2`): one file per session per author under
 * `Adventure Log/`, listed newest session first. "+" writes a file with the default frontmatter;
 * the selected entry opens in the editor, frontmatter included, so every field stays editable.
 */

const message = (cause: unknown) => (cause instanceof Error ? cause.message : String(cause));
const button = { background: 'none', border: '1px solid var(--color-border)', borderRadius: 6, cursor: 'pointer', padding: '0.35rem 0.8rem' } as const;
const input = { border: '1px solid var(--color-border)', borderRadius: 6, padding: '0.3rem 0.5rem' } as const;
const cell = { borderBottom: '1px solid var(--color-border)', fontSize: '0.85rem', padding: '0.35rem 0.4rem', textAlign: 'left' } as const;

export function AdventureLogTab(props: { campaignDir: string; tree: readonly CampaignTreeEntry[]; onTreeChanged: () => void }) {
  const { campaignDir, tree } = props;
  const [entries, setEntries] = useState<AdventureLogEntry[] | null>(null);
  const [selected, setSelected] = useState<string | null>(null);
  const [draft, setDraft] = useState<{ author: string; title: string } | null>(null);
  const [notice, setNotice] = useState<string | null>(null);

  // Re-read on folder change and on every selection, so frontmatter edits made in the editor show.
  useEffect(() => {
    let live = true;
    listAdventureLog(campaignDir)
      .then((fresh) => live && setEntries(fresh))
      .catch((cause: unknown) => live && setNotice(message(cause)));
    return () => {
      live = false;
    };
  }, [campaignDir, tree, selected]);

  const create = () => {
    if (!draft) return;
    setNotice(null);
    newAdventureLogEntry(campaignDir, draft.author, draft.title, localIsoDate(new Date()))
      .then((path) => {
        rememberYourName(draft.author);
        setDraft(null);
        setSelected(path);
        props.onTreeChanged();
      })
      .catch((cause: unknown) => setNotice(message(cause)));
  };

  return (
    <div data-testid="adventure-log">
      <div style={{ alignItems: 'center', display: 'flex', flexWrap: 'wrap', gap: '0.6rem', marginBottom: '0.6rem' }}>
        <button
          type="button"
          aria-label="New Adventure Log entry"
          style={button}
          disabled={!entries || draft !== null}
          onClick={() => setDraft({ author: readYourName(), title: nextSessionTitle(entries ?? []) })}
        >
          +
        </button>
        {draft && (
          <>
            <input aria-label="Author (your name)" style={input} value={draft.author} onChange={(e) => setDraft({ ...draft, author: e.target.value })} />
            <input aria-label="Title" style={input} value={draft.title} onChange={(e) => setDraft({ ...draft, title: e.target.value })} />
            <button type="button" style={button} disabled={!draft.author.trim() || !draft.title.trim()} onClick={create}>
              Create
            </button>
            <button type="button" style={button} onClick={() => setDraft(null)}>
              Cancel
            </button>
          </>
        )}
      </div>
      {notice && <p role="alert" style={{ color: 'var(--color-error)', marginTop: 0 }}>{notice}</p>}
      {entries === null ? (
        <p style={{ color: 'var(--color-text-muted)', margin: 0 }}>Reading the Adventure Log…</p>
      ) : entries.length === 0 ? (
        <p style={{ color: 'var(--color-text-muted)', fontSize: '0.85rem', margin: 0 }}>No sessions logged yet. "+" starts one.</p>
      ) : (
        <table data-testid="adventure-log-list" style={{ borderCollapse: 'collapse', marginBottom: '1rem', width: '100%' }}>
          <thead>
            <tr>
              <th style={cell}>Session</th>
              <th style={cell}>In-game date</th>
              <th style={cell}>Author</th>
              <th style={cell}>Title</th>
            </tr>
          </thead>
          <tbody>
            {entries.map((entry) => {
              const mark = statusMark(entry);
              const active = entry.path === selected;
              return (
                <tr
                  key={entry.path}
                  onClick={() => setSelected(entry.path)}
                  style={{ background: active ? 'var(--color-surface-raised, rgba(127,127,127,0.12))' : undefined, cursor: 'pointer' }}
                >
                  <td style={cell}>{sessionLabel(entry.session)}</td>
                  <td style={cell}>{entry.inGameDate ?? ''}</td>
                  <td style={cell}>{entry.author ?? ''}</td>
                  <td style={{ ...cell, overflowWrap: 'anywhere' }}>
                    {entry.title}
                    {mark && <span style={{ color: 'var(--color-text-muted)', fontSize: '0.75rem', marginLeft: '0.5rem' }}>({mark})</span>}
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      )}
      {selected && (
        <MarkdownEditor
          key={selected}
          campaignDir={campaignDir}
          path={selected}
          readFile={readCampaignFile}
          writeFile={writeCampaignFile}
          statFile={statCampaignFile}
          saveAttachment={saveAttachment}
          resolveImage={(image) => campaignImageUrl(campaignDir, image)}
          pages={async () => markdownStems(tree)}
          onOpenLink={(target) => {
            const found = linkTarget(tree, target);
            setNotice(found ? null : `No page named "${target}" in this campaign yet.`);
            if (found) setSelected(found);
          }}
        />
      )}
    </div>
  );
}
