// The "Reading" view (decisions.md §18, SD-g): marked with every raw-HTML token escaped and
// script-capable URLs dropped, so no sanitiser dependency is needed.
import { Marked, type Tokens } from 'marked';

const escapeHtml = (text: string) =>
  text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');

// Allowed: http(s), mailto and scheme-less (relative) URLs. The scheme is read as a browser reads it:
// a URL parser drops tab/LF/CR anywhere and C0 controls or spaces at the ends (`java<TAB>script:`).
const safeUrl = (href: string) => {
  const url = href.replace(/[\t\n\r]/g, '').replace(/^[\u0000-\u0020]+/, '');
  return /^(https?:|mailto:)/i.test(url) || !/^[a-z][a-z0-9+.-]*:/i.test(url);
};

const reader = new Marked({
  gfm: true,
  renderer: {
    html(token: Tokens.HTML | Tokens.Tag) {
      return escapeHtml(token.text);
    },
    link(token: Tokens.Link) {
      const text = this.parser.parseInline(token.tokens);
      return safeUrl(token.href) ? `<a href="${escapeHtml(token.href)}">${text}</a>` : text;
    },
    image(token: Tokens.Image) {
      return safeUrl(token.href) ? `<img src="${escapeHtml(token.href)}" alt="${escapeHtml(token.text)}">` : escapeHtml(token.text);
    },
  },
});

export function renderReading(markdown: string): string {
  return reader.parse(markdown, { async: false });
}
