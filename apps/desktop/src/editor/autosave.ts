// Auto-save with a disk-change check (decisions.md §18, Q25): a write 1 s after the last edit
// and on blur; before each write the file's mtime is compared with the one loaded, and a
// difference raises "Changed on disk" (Reload / Keep mine) and blocks every write until answered.

export interface AutosaveIO {
  getText(): string;
  /** Writes the text; resolves to the file's new mtime. */
  write(text: string): Promise<number>;
  /** The file's mtime on disk now. */
  stat(): Promise<number>;
  read(): Promise<{ text: string; modifiedMs: number }>;
  /** Puts `text` in the editor; the change it causes is not an edit to save. */
  replaceDoc(text: string): void;
  onState(state: AutosaveState): void;
}

export type AutosaveState =
  | { kind: 'idle' }
  | { kind: 'saved' }
  | { kind: 'conflict' }
  | { kind: 'error'; message: string };

export const AUTOSAVE_DELAY_MS = 1000;

export function createAutosave(io: AutosaveIO, loadedMtime: number, delayMs = AUTOSAVE_DELAY_MS) {
  let loaded = loadedMtime;
  let dirty = false;
  let conflict = false;
  let replacing = false;
  let timer: ReturnType<typeof setTimeout> | undefined;
  let running: Promise<void> = Promise.resolve();

  const cancel = () => {
    if (timer !== undefined) clearTimeout(timer);
    timer = undefined;
  };
  const fail = (err: unknown) => io.onState({ kind: 'error', message: `Save failed: ${err instanceof Error ? err.message : String(err)}` });

  // Writes run one at a time, so a blur during a debounced write never races it.
  const serial = (step: () => Promise<void>) => (running = running.then(step, step));

  const writeNow = async () => {
    const text = io.getText();
    dirty = false;
    try {
      loaded = await io.write(text);
      io.onState({ kind: 'saved' });
    } catch (err) {
      dirty = true;
      fail(err);
    }
  };

  const flush = () => {
    cancel();
    return serial(async () => {
      if (!dirty || conflict) return;
      try {
        if ((await io.stat()) !== loaded) {
          conflict = true;
          io.onState({ kind: 'conflict' });
          return;
        }
      } catch (err) {
        fail(err);
        return;
      }
      await writeNow();
    });
  };

  return {
    /** Call on every document change. */
    changed() {
      if (replacing) return;
      dirty = true;
      if (conflict) return;
      cancel();
      timer = setTimeout(() => void flush(), delayMs);
    },
    /** Blur, Mod-s, and unmount. */
    flush,
    /** "Keep mine": write the editor's text over the disk's. */
    keepMine() {
      cancel();
      return serial(async () => {
        conflict = false;
        await writeNow();
      });
    },
    /** "Reload": the disk's text replaces the editor's. */
    reload() {
      cancel();
      return serial(async () => {
        try {
          const file = await io.read();
          replacing = true;
          try {
            io.replaceDoc(file.text);
          } finally {
            replacing = false;
          }
          loaded = file.modifiedMs;
          dirty = false;
          conflict = false;
          io.onState({ kind: 'idle' });
        } catch (err) {
          fail(err);
        }
      });
    },
    dispose: cancel,
  };
}

export type Autosave = ReturnType<typeof createAutosave>;
