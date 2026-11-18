import { useEffect, useState } from 'react';
import { MarkdownEditor } from '../../editor/MarkdownEditor';
import { campaignImageUrl, readCampaignFile, statCampaignFile, writeCampaignFile } from '../../boundary/campaignFolder/campaignFiles';
import { saveAttachment } from '../../boundary/campaignFolder/attachments';
import type { CampaignTreeEntry } from '../../boundary/campaignFolder/types';
import {
  addWikiFolder,
  deleteWikiFolder,
  listWikiFolders,
  listWikiPages,
  newWikiPage,
  renameWikiFolder,
  resolveWikiLink,
  searchCampaign,
  wikiBacklinks,
  type WikiPage,
  type WikiSearchHit,
} from '../../boundary/campaignFolder/wiki';
import { WIKI_DIR } from '../mapsModel';
import { completionNames, deleteRefusal, wikiRows } from '../wikiModel';

/**
 * Wiki (SD-42 `decisions.md §15.4`, Q18): free folders under `Wiki/` (add, rename, delete when
 * empty), "+" writes a page with a small frontmatter, `[[links]]` open their page (title first,
 * then filename stem) or offer to create it (SD-p), the open page lists the pages that link to it,
 * and one search box finds titles first, then text, in every `.md` of the campaign.
 */

const message = (cause: unknown) => (cause instanceof Error ? cause.message : String(cause));
const button = { background: 'none', border: '1px solid var(--color-border)', borderRadius: 6, cursor: 'pointer', padding: '0.3rem 0.7rem' } as const;
const input = { border: '1px solid var(--color-border)', borderRadius: 6, padding: '0.3rem 0.5rem' } as const;
const muted = { color: 'var(--color-text-muted)', fontSize: '0.85rem' } as const;
const link = { background: 'none', border: 'none', color: 'var(--color-accent)', cursor: 'pointer', fontSize: '0.85rem', padding: 0, textAlign: 'left' } as const;

type Form = { mode: 'page' | 'folder' | 'rename'; value: string };
const FORM_LABEL: Record<Form['mode'], string> = { page: 'New page name', folder: 'New folder name', rename: 'Rename folder to' };

export function WikiTab(props: {
  campaignDir: string;
  tree: readonly CampaignTreeEntry[];
  onTreeChanged: () => void;
  /** The open page (a path inside the campaign), kept by the sheet so a map pin can open one. */
  page: string | null;
  onOpenPage: (path: string | null) => void;
}) {
  const { campaignDir, tree, page } = props;
  const [folders, setFolders] = useState<string[]>([]);
  const [pages, setPages] = useState<WikiPage[] | null>(null);
  const [folder, setFolder] = useState(WIKI_DIR);
  const [form, setForm] = useState<Form | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  const [missing, setMissing] = useState<string | null>(null);
  const [backlinks, setBacklinks] = useState<string[] | null>(null);
  const [query, setQuery] = useState('');
  const [hits, setHits] = useState<WikiSearchHit[] | null>(null);

  useEffect(() => {
    let live = true;
    Promise.all([listWikiFolders(campaignDir), listWikiPages(campaignDir)])
      .then(([freshFolders, freshPages]) => {
        if (!live) return;
        setFolders(freshFolders);
        setPages(freshPages);
        setFolder((current) => (current === WIKI_DIR || freshFolders.includes(current) ? current : WIKI_DIR));
      })
      .catch((cause: unknown) => live && setNotice(message(cause)));
    return () => {
      live = false;
    };
  }, [campaignDir, tree]);

  // The index is rebuilt on every folder change (the tree) and every page opened.
  useEffect(() => {
    if (!page) return setBacklinks(null);
    let live = true;
    wikiBacklinks(campaignDir, page)
      .then((found) => live && setBacklinks(found))
      .catch((cause: unknown) => live && setNotice(message(cause)));
    return () => {
      live = false;
    };
  }, [campaignDir, page, tree]);

  useEffect(() => {
    if (!query.trim()) return setHits(null);
    let live = true;
    searchCampaign(campaignDir, query)
      .then((found) => live && setHits(found))
      .catch((cause: unknown) => live && setNotice(message(cause)));
    return () => {
      live = false;
    };
  }, [campaignDir, query, tree]);

  const run = (work: Promise<unknown>) => {
    setNotice(null);
    work.catch((cause: unknown) => setNotice(message(cause)));
  };

  const open = (path: string) => {
    setMissing(null);
    props.onOpenPage(path);
  };

  const submit = () => {
    if (!form) return;
    const name = form.value.trim();
    run(
      (form.mode === 'page'
        ? newWikiPage(campaignDir, folder, name).then(open)
        : form.mode === 'folder'
          ? addWikiFolder(campaignDir, folder, name).then(setFolder)
          : renameWikiFolder(campaignDir, folder, name).then((renamed) => {
              if (page?.startsWith(`${folder}/`)) props.onOpenPage(renamed + page.slice(folder.length));
              setFolder(renamed);
            })
      ).then(() => {
        setForm(null);
        props.onTreeChanged();
      })
    );
  };

  const remove = () =>
    run(
      deleteWikiFolder(campaignDir, folder).then((held) => {
        if (held.length > 0) return setNotice(deleteRefusal(folder, held));
        setFolder(WIKI_DIR);
        props.onTreeChanged();
      })
    );

  const followLink = (target: string) =>
    run(
      resolveWikiLink(campaignDir, target).then((found) => {
        if (found) open(found);
        else setMissing(target);
      })
    );

  const createMissing = (name: string) =>
    run(
      newWikiPage(campaignDir, WIKI_DIR, name).then((path) => {
        open(path);
        props.onTreeChanged();
      })
    );

  const titleOf = (path: string) => pages?.find((p) => p.path === path)?.title ?? path.split('/').pop()!.replace(/\.md$/i, '');
  const rows = wikiRows(folders, pages ?? []);

  return (
    <div data-testid="wiki-tab">
      <label style={{ alignItems: 'center', display: 'flex', fontSize: '0.85rem', gap: '0.5rem', marginBottom: '0.75rem' }}>
        Search
        <input
          type="search"
          aria-label="Search the campaign by title and text"
          style={{ ...input, flex: 1 }}
          value={query}
          onChange={(event) => setQuery(event.target.value)}
        />
      </label>
      {hits && (
        <div data-testid="wiki-search-results" style={{ marginBottom: '1rem' }}>
          {hits.length === 0 ? (
            <p style={{ ...muted, margin: 0 }}>Nothing in this campaign matches "{query.trim()}".</p>
          ) : (
            <ul style={{ listStyle: 'none', margin: 0, padding: 0 }}>
              {hits.map((hit) => (
                <li key={hit.path} style={{ borderBottom: '1px solid var(--color-border)', padding: '0.35rem 0.2rem' }}>
                  <button type="button" style={{ ...link, fontWeight: 700 }} onClick={() => open(hit.path)}>
                    {hit.title}
                  </button>
                  <span style={{ ...muted, fontSize: '0.75rem', marginLeft: '0.5rem' }}>
                    {hit.path}
                    {hit.titleMatch ? ' — title' : ''}
                  </span>
                  {hit.snippet && <p style={{ ...muted, margin: '0.15rem 0 0', overflowWrap: 'anywhere' }}>{hit.snippet}</p>}
                </li>
              ))}
            </ul>
          )}
        </div>
      )}
      {notice && <p role="alert" style={{ color: 'var(--color-error)', marginTop: 0 }}>{notice}</p>}

      <div style={{ display: 'flex', gap: '1.25rem' }}>
        <div style={{ flex: '0 0 250px', minWidth: 0 }}>
          <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.35rem', marginBottom: '0.5rem' }}>
            <button type="button" style={button} aria-label={`New page in ${folder}`} onClick={() => setForm({ mode: 'page', value: '' })}>
              + Page
            </button>
            <button type="button" style={button} aria-label={`New folder in ${folder}`} onClick={() => setForm({ mode: 'folder', value: '' })}>
              + Folder
            </button>
            <button type="button" style={button} disabled={folder === WIKI_DIR} onClick={() => setForm({ mode: 'rename', value: folder.split('/').pop()! })}>
              Rename
            </button>
            <button type="button" style={button} disabled={folder === WIKI_DIR} onClick={remove}>
              Delete
            </button>
          </div>
          {form && (
            <form
              onSubmit={(event) => {
                event.preventDefault();
                submit();
              }}
              style={{ display: 'flex', flexWrap: 'wrap', gap: '0.35rem', marginBottom: '0.5rem' }}
            >
              <label style={{ fontSize: '0.8rem', width: '100%' }}>
                {FORM_LABEL[form.mode]} ({folder}){' '}
                <input autoFocus style={{ ...input, width: '100%' }} value={form.value} onChange={(event) => setForm({ ...form, value: event.target.value })} />
              </label>
              <button type="submit" style={button} disabled={!form.value.trim()}>
                {form.mode === 'rename' ? 'Rename' : 'Create'}
              </button>
              <button type="button" style={button} onClick={() => setForm(null)}>
                Cancel
              </button>
            </form>
          )}
          {pages === null ? (
            <p style={muted}>Reading the wiki…</p>
          ) : (
            <ul data-testid="wiki-folders" style={{ listStyle: 'none', margin: 0, padding: 0 }}>
              {rows.map((row) => {
                const active = row.kind === 'folder' ? row.path === folder : row.path === page;
                return (
                  <li key={`${row.kind}:${row.path}`} style={{ paddingLeft: `${row.depth * 0.9}rem` }}>
                    <button
                      type="button"
                      onClick={() => (row.kind === 'folder' ? setFolder(row.path) : open(row.path))}
                      style={{
                        ...link,
                        background: active ? 'var(--color-surface-raised, rgba(127,127,127,0.15))' : 'none',
                        borderRadius: 4,
                        color: row.kind === 'folder' ? 'var(--color-text)' : 'var(--color-accent)',
                        fontWeight: row.kind === 'folder' ? 700 : 400,
                        padding: '0.15rem 0.3rem',
                        width: '100%',
                      }}
                    >
                      {row.kind === 'folder' ? `▸ ${row.name}/ (${row.pages})` : row.title}
                    </button>
                  </li>
                );
              })}
            </ul>
          )}
        </div>

        <div style={{ flex: 1, minWidth: 0 }}>
          {missing && (
            <p role="status" style={{ ...muted, color: 'var(--color-text)', marginTop: 0 }}>
              {missing}: page not found —{' '}
              <button type="button" style={button} onClick={() => createMissing(missing)}>
                create it in {WIKI_DIR}/
              </button>
            </p>
          )}
          {page ? (
            <div data-testid="wiki-open-page">
              <p style={{ fontWeight: 700, margin: '0 0 0.5rem', overflowWrap: 'anywhere' }}>{page}</p>
              <MarkdownEditor
                key={page}
                campaignDir={campaignDir}
                path={page}
                readFile={readCampaignFile}
                writeFile={writeCampaignFile}
                statFile={statCampaignFile}
                saveAttachment={saveAttachment}
                resolveImage={(image) => campaignImageUrl(campaignDir, image)}
                pages={async () => completionNames(pages ?? [], tree)}
                onOpenLink={followLink}
              />
              <div data-testid="wiki-backlinks" style={{ marginTop: '1rem' }}>
                <p style={{ fontWeight: 700, margin: '0 0 0.35rem' }}>Pages that link here</p>
                {backlinks === null ? (
                  <p style={{ ...muted, margin: 0 }}>Reading the links…</p>
                ) : backlinks.length === 0 ? (
                  <p style={{ ...muted, margin: 0 }}>No page links here yet.</p>
                ) : (
                  <ul style={{ margin: 0, paddingLeft: '1.2rem' }}>
                    {backlinks.map((from) => (
                      <li key={from}>
                        <button type="button" style={link} onClick={() => open(from)}>
                          {titleOf(from)}
                        </button>
                        <span style={{ ...muted, fontSize: '0.75rem', marginLeft: '0.4rem' }}>{from}</span>
                      </li>
                    ))}
                  </ul>
                )}
              </div>
            </div>
          ) : (
            <p style={{ ...muted, margin: 0 }}>Pick a page on the left, or "+ Page" to write one in {folder}/.</p>
          )}
        </div>
      </div>
    </div>
  );
}
