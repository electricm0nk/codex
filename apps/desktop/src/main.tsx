import React from 'react';
import ReactDOM from 'react-dom/client';
import './theme.css';
import App from './App';
import { installUiProbe } from './testSupport/uiProbe';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);

// Dev-only headless UI smoke harness hook (apps/desktop/scripts/ui-smoke/).
// `installUiProbe()` itself no-ops outside a real Tauri runtime, but the
// `import.meta.env.DEV` gate keeps this out of production builds entirely.
if (import.meta.env.DEV) {
  installUiProbe();
}
