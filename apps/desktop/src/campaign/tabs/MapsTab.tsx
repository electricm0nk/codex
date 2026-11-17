import { useEffect, useRef, useState, type MouseEvent as ReactMouseEvent, type PointerEvent as ReactPointerEvent } from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import { MarkdownEditor } from '../../editor/MarkdownEditor';
import { campaignImageUrl, readCampaignFile, statCampaignFile, writeCampaignFile } from '../../boundary/campaignFolder/campaignFiles';
import { saveAttachment } from '../../boundary/campaignFolder/attachments';
import { addMap, listMaps, setMapPins, type MapEntry, type MapPin } from '../../boundary/campaignFolder/maps';
import type { CampaignTreeEntry } from '../../boundary/campaignFolder/types';
import { clickFraction, newPageText, pinTarget, wikiPages } from '../mapsModel';
import { markdownStems } from './partyResourcesModel';

/**
 * Maps with pins (SD-42 `decisions.md §15.3`, Q17): `Maps/<name>.png|jpg` with `Maps/<name>.md`.
 * A click on the image drops a pin linked to a wiki page; a pin opens its page; pins drag to move
 * and delete from the list under the map; the notes file opens in the editor below. Pins live in
 * the notes file's frontmatter, written by Rust (`set_map_pins`) so the rest of the file stays.
 */

const message = (cause: unknown) => (cause instanceof Error ? cause.message : String(cause));
const button = { background: 'none', border: '1px solid var(--color-border)', borderRadius: 6, cursor: 'pointer', padding: '0.35rem 0.8rem' } as const;
const input = { border: '1px solid var(--color-border)', borderRadius: 6, padding: '0.3rem 0.5rem' } as const;
const muted = { color: 'var(--color-text-muted)', fontSize: '0.85rem' } as const;

type Pending = { x: number; y: number; page: string; label: string; newPage: string };
type Drag = { index: number; startX: number; startY: number; x: number; y: number; moved: boolean };

export function MapsTab(props: {
  campaignDir: string;
  tree: readonly CampaignTreeEntry[];
  onTreeChanged: () => void;
  /** Opens a wiki page (a path inside the campaign) on the Wiki tab. */
  onOpenPage: (path: string) => void;
}) {
  const { campaignDir, tree } = props;
  const [maps, setMaps] = useState<MapEntry[] | null>(null);
  const [selected, setSelected] = useState<string | null>(null);
  const [imageUrl, setImageUrl] = useState<string | null>(null);
  const [pending, setPending] = useState<Pending | null>(null);
  const [drag, setDrag] = useState<Drag | null>(null);
  const [missing, setMissing] = useState<{ name: string; createPath: string } | null>(null);
  const [notice, setNotice] = useState<string | null>(null);
  // Bumped after every pins write so the notes editor re-reads the file it shares with the pins.
  const [epoch, setEpoch] = useState(0);
  const imageBox = useRef<HTMLDivElement>(null);
  const dragged = useRef(false);

  const reload = () =>
    listMaps(campaignDir)
      .then((fresh) => {
        setMaps(fresh);
        setSelected((current) => current ?? fresh.find((map) => map.image)?.notes ?? fresh[0]?.notes ?? null);
      })
      .catch((cause: unknown) => setNotice(message(cause)));

  useEffect(() => {
    void reload();
  }, [campaignDir, tree]);

  const map = maps?.find((entry) => entry.notes === selected) ?? null;
  const image = map?.image ?? null;

  useEffect(() => {
    if (!image) {
      setImageUrl(null);
      return;
    }
    let live = true;
    let url: string | null = null;
    campaignImageUrl(campaignDir, image)
      .then((made) => {
        url = made;
        if (live) setImageUrl(made);
        else URL.revokeObjectURL(made);
      })
      .catch((cause: unknown) => live && setNotice(message(cause)));
    return () => {
      live = false;
      if (url) URL.revokeObjectURL(url);
    };
  }, [campaignDir, image]);

  const savePins = (pins: MapPin[]) => {
    if (!map) return Promise.resolve();
    setNotice(null);
    setMissing(null);
    return setMapPins(campaignDir, map.notes, pins)
      .then(() => {
        setEpoch((n) => n + 1);
        if (!map.notesExists) props.onTreeChanged();
        return reload();
      })
      .catch((cause: unknown) => setNotice(message(cause)));
  };

  const createPage = async (name: string, createPath: string) => {
    const stat = await statCampaignFile(campaignDir, createPath);
    if (!stat.exists) await writeCampaignFile(campaignDir, createPath, new TextEncoder().encode(newPageText(name)));
    props.onTreeChanged();
  };

  const openPin = (pin: MapPin) => {
    const target = pinTarget(tree, pin.page);
    setMissing(null);
    if (target.kind === 'open') props.onOpenPage(target.path);
    else if (target.kind === 'missing') setMissing({ name: target.name, createPath: target.createPath });
    else setNotice('This pin links to no wiki page.');
  };

  const addPin = async () => {
    if (!pending || !map) return;
    try {
      const newName = pending.newPage.trim();
      if (newName) {
        const target = pinTarget(tree, newName);
        if (target.kind === 'missing') await createPage(target.name, target.createPath);
      }
      const page = newName || pending.page;
      await savePins([...map.pins, { x: pending.x, y: pending.y, label: pending.label.trim() || page, page }]);
      setPending(null);
    } catch (cause: unknown) {
      setNotice(message(cause));
    }
  };

  const pick = async () => {
    setNotice(null);
    try {
      const picked = await open({ multiple: false, directory: false, title: 'Add a map image', filters: [{ name: 'Map image', extensions: ['png', 'jpg', 'jpeg'] }] });
      if (typeof picked !== 'string') return;
      const path = await addMap(campaignDir, picked);
      setSelected(path.replace(/\.[^./]+$/, '.md'));
      props.onTreeChanged();
    } catch (cause: unknown) {
      setNotice(message(cause));
    }
  };

  const where = (event: ReactPointerEvent | ReactMouseEvent) => clickFraction(event.clientX, event.clientY, imageBox.current!.getBoundingClientRect());

  const pages = wikiPages(tree);

  return (
    <div data-testid="maps-tab">
      <div style={{ alignItems: 'center', display: 'flex', flexWrap: 'wrap', gap: '0.5rem', marginBottom: '0.75rem' }}>
        <button type="button" style={button} onClick={() => void pick()}>
          Add map…
        </button>
        {maps?.map((entry) => {
          const active = entry.notes === selected;
          return (
            <button
              key={entry.notes}
              type="button"
              onClick={() => {
                setSelected(entry.notes);
                setPending(null);
                setMissing(null);
              }}
              style={{ ...button, background: active ? 'var(--color-accent)' : 'none', color: active ? 'var(--color-on-accent)' : 'var(--color-text)' }}
            >
              {entry.title}
              <span style={{ fontSize: '0.75rem', marginLeft: '0.4rem', opacity: 0.8 }}>
                {entry.image ? `${entry.pins.length} pin${entry.pins.length === 1 ? '' : 's'}` : 'no image'}
                {entry.pinsOk ? '' : ', pins not read'}
              </span>
            </button>
          );
        })}
      </div>
      {notice && <p role="alert" style={{ color: 'var(--color-error)', marginTop: 0 }}>{notice}</p>}
      {maps === null ? (
        <p style={{ ...muted, margin: 0 }}>Reading the maps…</p>
      ) : maps.length === 0 ? (
        <p style={{ ...muted, margin: 0 }}>No maps yet. "Add map…" copies a .png or .jpg into Maps/ and makes its notes file.</p>
      ) : map ? (
        <>
          {!map.pinsOk && (
            <p style={muted}>The pins in {map.notes} are not in a form Codex reads, so they are not shown and not changed. Fix them in the notes below.</p>
          )}
          {image ? (
            <div
              ref={imageBox}
              data-testid="map-view"
              style={{ display: 'inline-block', lineHeight: 0, maxWidth: '100%', position: 'relative' }}
            >
              {imageUrl ? (
                <img
                  src={imageUrl}
                  alt={map.title}
                  title={map.pinsOk ? 'Click to drop a pin' : undefined}
                  onClick={(event) => {
                    if (!map.pinsOk) return;
                    const at = where(event);
                    setMissing(null);
                    setPending({ ...at, page: pages[0] ?? '', label: '', newPage: pages.length ? '' : 'New page' });
                  }}
                  style={{ cursor: map.pinsOk ? 'crosshair' : 'default', display: 'block', maxHeight: '70vh', maxWidth: '100%' }}
                />
              ) : (
                <p style={muted}>Loading {image}…</p>
              )}
              {imageUrl &&
                map.pins.map((pin, index) => {
                  const live = drag?.index === index ? drag : null;
                  return (
                    <button
                      key={index}
                      type="button"
                      data-testid="map-pin"
                      title={`${pin.label} → ${pin.page || 'no page'} (drag to move)`}
                      aria-label={`Pin ${pin.label}: open ${pin.page || 'no page'}`}
                      onPointerDown={(event) => {
                        event.currentTarget.setPointerCapture(event.pointerId);
                        dragged.current = false;
                        setDrag({ index, startX: event.clientX, startY: event.clientY, x: pin.x, y: pin.y, moved: false });
                      }}
                      onPointerMove={(event) => {
                        if (!drag || drag.index !== index) return;
                        const moved = drag.moved || Math.hypot(event.clientX - drag.startX, event.clientY - drag.startY) > 4;
                        setDrag({ ...drag, ...where(event), moved });
                      }}
                      onPointerUp={() => {
                        if (drag?.moved) {
                          dragged.current = true;
                          void savePins(map.pins.map((p, i) => (i === index ? { ...p, x: drag.x, y: drag.y } : p)));
                        }
                        setDrag(null);
                      }}
                      onClick={() => {
                        if (dragged.current) {
                          dragged.current = false;
                          return;
                        }
                        openPin(pin);
                      }}
                      style={{
                        background: 'var(--color-accent)',
                        border: '2px solid var(--color-surface)',
                        borderRadius: '50% 50% 50% 0',
                        boxShadow: '0 1px 4px rgba(0,0,0,0.5)',
                        color: 'var(--color-on-accent)',
                        cursor: live ? 'grabbing' : 'pointer',
                        fontSize: '0.7rem',
                        fontWeight: 700,
                        height: 22,
                        left: `${(live ?? pin).x * 100}%`,
                        lineHeight: 1,
                        padding: 0,
                        position: 'absolute',
                        top: `${(live ?? pin).y * 100}%`,
                        touchAction: 'none',
                        transform: 'translate(-50%, -100%) rotate(-45deg)',
                        width: 22,
                      }}
                    >
                      <span style={{ display: 'inline-block', transform: 'rotate(45deg)' }}>{index + 1}</span>
                    </button>
                  );
                })}
            </div>
          ) : (
            <p style={muted}>{map.notes} has no image beside it. "Add map…" with an image of the same name pairs them.</p>
          )}

          {pending && (
            <div role="dialog" aria-label="Link to wiki page" style={{ border: '1px solid var(--color-border)', borderRadius: 8, display: 'flex', flexWrap: 'wrap', gap: '0.5rem', margin: '0.75rem 0', padding: '0.75rem' }}>
              <strong style={{ flexBasis: '100%' }}>Link to wiki page</strong>
              {pages.length > 0 && (
                <label style={{ fontSize: '0.85rem' }}>
                  Page{' '}
                  <select style={input} value={pending.page} disabled={pending.newPage.trim() !== ''} onChange={(e) => setPending({ ...pending, page: e.target.value })}>
                    {pages.map((page) => (
                      <option key={page} value={page}>
                        {page}
                      </option>
                    ))}
                  </select>
                </label>
              )}
              <label style={{ fontSize: '0.85rem' }}>
                Or a new page <input style={input} value={pending.newPage} onChange={(e) => setPending({ ...pending, newPage: e.target.value })} />
              </label>
              <label style={{ fontSize: '0.85rem' }}>
                Label (default: the page) <input style={input} value={pending.label} onChange={(e) => setPending({ ...pending, label: e.target.value })} />
              </label>
              <button type="button" style={button} disabled={!pending.newPage.trim() && !pending.page} onClick={() => void addPin()}>
                {pending.newPage.trim() ? 'Create page and pin' : 'Add pin'}
              </button>
              <button type="button" style={button} onClick={() => setPending(null)}>
                Cancel
              </button>
            </div>
          )}

          {missing && (
            <p role="status" style={{ ...muted, color: 'var(--color-text)' }}>
              {missing.name}: page not found —{' '}
              <button
                type="button"
                style={button}
                onClick={() =>
                  createPage(missing.name, missing.createPath)
                    .then(() => {
                      setMissing(null);
                      props.onOpenPage(missing.createPath);
                    })
                    .catch((cause: unknown) => setNotice(message(cause)))
                }
              >
                create it
              </button>
            </p>
          )}

          {map.pins.length > 0 && (
            <ol data-testid="map-pins" style={{ fontSize: '0.85rem', margin: '0.75rem 0', paddingLeft: '1.5rem' }}>
              {map.pins.map((pin, index) => (
                <li key={index} style={{ marginBottom: '0.3rem' }}>
                  {pin.label} → {pin.page ? `[[${pin.page}]]` : 'no page'}{' '}
                  <button type="button" style={{ ...button, padding: '0.1rem 0.5rem' }} onClick={() => openPin(pin)}>
                    Open
                  </button>{' '}
                  <button
                    type="button"
                    style={{ ...button, padding: '0.1rem 0.5rem' }}
                    aria-label={`Delete pin ${index + 1} ${pin.label}`}
                    onClick={() => void savePins(map.pins.filter((_, i) => i !== index))}
                  >
                    Delete
                  </button>
                </li>
              ))}
            </ol>
          )}

          <div style={{ marginTop: '1rem' }}>
            {map.notesExists ? (
              <MarkdownEditor
                key={`${map.notes}#${epoch}`}
                campaignDir={campaignDir}
                path={map.notes}
                readFile={readCampaignFile}
                writeFile={writeCampaignFile}
                statFile={statCampaignFile}
                saveAttachment={saveAttachment}
                resolveImage={(path) => campaignImageUrl(campaignDir, path)}
                pages={async () => markdownStems(tree)}
                onOpenLink={(target) => {
                  const found = pinTarget(tree, target);
                  if (found.kind === 'open') props.onOpenPage(found.path);
                  else if (found.kind === 'missing') setMissing({ name: found.name, createPath: found.createPath });
                }}
              />
            ) : (
              <button type="button" style={button} onClick={() => void savePins([])}>
                Create notes file {map.notes}
              </button>
            )}
          </div>
        </>
      ) : null}
    </div>
  );
}
