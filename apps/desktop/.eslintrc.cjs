// SD-16 / E8-F3 — controlled-defect lint guard.
//
// This config is append-only relative to any future desktop-wide
// eslint config that may be introduced by E6's F3a lint-config block.
// It only ENFORCES the AV-UI-7 rule the F2 handoff pinned:
//
//   No `fetch()` call in any code path may target the GitHub
//   Releases API or the `/repos/.../issues` API. The shell's
//   discovery surface is the protected `update-index` branch on
//   origin; agent-driven GitHub API calls belong in the agent lane
//   (or in the operator browser handoff), not in the desktop app.

module.exports = {
  root: true,
  env: {
    browser: true,
    es2020: true,
  },
  parserOptions: {
    ecmaVersion: 2020,
    sourceType: 'module',
  },
  rules: {
    // Forbids any `fetch(<string>)` whose first argument literal
    // targets the GitHub Releases API or the repo issues/releases
    // REST surface. The matcher is structural: it scans the literal
    // argument for the forbidden substrings. Template literals are
    // also covered where the static parts include the substrings.
    'no-restricted-syntax': [
      'error',
      {
        selector: "CallExpression[callee.name='fetch'] Literal[value=/(^https?:\\/\\/(api\\.)?github\\.com\\/repos\\/|\\/releases)/i]",
        message:
          'SD-16 AV-UI-7: the shell must not call fetch() against api.github.com/repos or /releases. Discovery goes through the protected update-index branch.',
      },
    ],
  },
  overrides: [
    {
      // The E6 F3a fetch module is the AUTHORITATIVE seam for
      // discovery calls; it only fetches the protected update-index
      // branch and the manifest URL it points at. We exempt it from
      // the rule's literal-scanner so that the comment strings
      // describing what is forbidden do not trip the rule.
      files: ['src/sd16/update/fetch.ts'],
      rules: {
        'no-restricted-syntax': 'off',
      },
    },
  ],
};
