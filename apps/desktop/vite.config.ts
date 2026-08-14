import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

/**
 * The dev-server port, per agent rather than per machine.
 *
 * This was a hardcoded `1420` with `strictPort: true` until SD-29 Epic 7
 * round 8 (`docs/release/SD-29-corpus-wide-catch-up-lanes/decisions.md §65.7`).
 * Under two concurrent agents that combination fails in a way that looks like
 * success: the second agent's vite cannot bind, dies, and `tauri dev` attaches
 * to the FIRST agent's server instead — so agent B's Rust backend gets painted
 * by agent A's frontend source. Every record renders and the screenshot is
 * evidence about someone else's tree.
 *
 * `strictPort` stays true deliberately. Falling back to a free port is what
 * would re-introduce the ambiguity: tauri is told exactly one `devUrl`, so a
 * vite that quietly moved elsewhere is indistinguishable from one that never
 * started. Failing loudly on a taken port is the property that makes the pair
 * trustworthy.
 *
 * `run-desktop/driver.sh` sets `CODEX_DEV_PORT` and passes the matching
 * `--config build.devUrl` to tauri; the two must always move together. The
 * default keeps 1420 so a solo run is unchanged.
 */
const devPort = Number(process.env.CODEX_DEV_PORT ?? 1420);

export default defineConfig({
  plugins: [react()],
  server: {
    port: devPort,
    strictPort: true,
  },
  preview: {
    port: devPort,
    strictPort: true,
  },
});
