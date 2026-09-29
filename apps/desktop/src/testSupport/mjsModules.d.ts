/**
 * Ambient shim so `tsc --noEmit` can resolve relative `.mjs` imports (the
 * `apps/desktop/scripts/ui-smoke/**` runner and its libs are plain Node ESM,
 * outside `allowJs`'s scope) without trying to type-check them. `tsx`, which
 * actually executes these files for `npm test`, ignores this declaration
 * entirely and runs the real module.
 */
declare module '*.mjs';
