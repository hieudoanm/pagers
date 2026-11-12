import { marked } from 'marked';
import { readdirSync, readFileSync, statSync } from 'node:fs';
import path from 'node:path';

const DOCS_DIR = path.join(process.cwd(), 'docs');
const REPO_BLOB = 'https://github.com/hieudoanm/pagers/blob/master';
// Must match basePath in next.config.ts (GitHub Pages project site).
// Used in raw HTML hrefs, so basePath is not applied automatically.
const BASE_PATH = process.env.NODE_ENV === 'production' ? '/pagers' : '';

export interface DocPage {
  /** URL slug, e.g. "guide/commands"; "" for docs/README.md */
  slug: string;
  /** Top-level section: guide | content | project | "" */
  section: string;
  /** Display title from frontmatter or the first `# ` heading */
  title: string;
  /** Rendered HTML */
  html: string;
}

const walk = (dir: string, prefix: string, out: string[]): void => {
  for (const entry of readdirSync(dir)) {
    const full = path.join(dir, entry);
    if (statSync(full).isDirectory()) {
      walk(full, prefix ? `${prefix}/${entry}` : entry, out);
    } else if (entry.endsWith('.md')) {
      out.push(prefix ? `${prefix}/${entry}` : entry);
    }
  }
};

const slugFor = (file: string): string => {
  return file === 'README.md' ? '' : file.replace(/\.md$/, '');
};

interface Frontmatter {
  title?: string;
  order: number;
  body: string;
}

const FRONTMATTER = /^---\r?\n([\s\S]*?)\r?\n---\r?\n?/;

const parseFrontmatter = (markdown: string): Frontmatter => {
  const m = markdown.match(FRONTMATTER);
  if (!m) return { order: 999, body: markdown };
  const meta = m[1];
  const title = meta.match(/^title:\s*(.+)$/m)?.[1].trim();
  const order = Number(meta.match(/^order:\s*(\d+)/m)?.[1] ?? 999);
  return { title, order, body: markdown.slice(m[0].length) };
};

const titleFor = (fm: Frontmatter, markdown: string, file: string): string => {
  if (fm.title) return fm.title;
  const m = fm.body.match(/^#\s+(.+)$/m);
  return m ? m[1].trim() : file;
};

/** Resolve a relative `.md` href against the current doc's directory. */
const resolveHref = (href: string, fromSlug: string): string => {
  const [filePart, anchor] = href.split('#');
  const fromDir = fromSlug ? fromSlug.split('/').slice(0, -1) : [];
  const targetParts = filePart.split('/');
  const stack = [...fromDir];
  let escaped = false;
  for (const part of targetParts) {
    if (part === '..') {
      if (stack.length === 0) escaped = true;
      else stack.pop();
    } else if (part !== '.' && part !== '') {
      stack.push(part);
    }
  }
  const suffix = anchor ? `#${anchor}` : '';
  if (escaped) {
    // Link escapes the docs tree — point at the repo blob.
    const repoPath = targetParts
      .filter((p) => p !== '..' && p !== '.' && p !== '')
      .join('/')
      .replace(/\.md$/, '');
    return `${REPO_BLOB}/${repoPath}${suffix}`;
  }
  const resolved = stack.join('/').replace(/\.md$/, '');
  const prefix = resolved
    ? `${BASE_PATH}/docs/${resolved}/`
    : `${BASE_PATH}/docs/`;
  return `${prefix}${suffix}`;
};

const rewriteLinks = (html: string, fromSlug: string): string => {
  return html.replace(/href="([^"]*?\.md(?:#[^"]*)?)"/g, (_m, href: string) => {
    return `href="${resolveHref(href, fromSlug)}"`;
  });
};

/** GitHub-style heading slug: lowercase, spaces→dashes, drop punctuation. */
const slugify = (text: string): string => {
  return text
    .toLowerCase()
    .replace(/<[^>]+>/g, '')
    .replace(/\{#[^}]+\}/g, '')
    .replace(/[^\p{L}\p{N}\s_-]/gu, '')
    .trim()
    .replace(/[\s_]+/g, '-');
};

/** Add id attributes to h1–h6; strip pagers' `{#custom-id}` syntax. */
const addHeadingIds = (html: string): string => {
  const used = new Map<string, number>();
  return html.replace(
    /<h([1-6])>([\s\S]*?)<\/h\1>/g,
    (match, level: string, inner: string) => {
      const text = inner.replace(/\s*\{#[^}]+\}\s*$/, '');
      let id = slugify(text);
      const n = used.get(id) ?? 0;
      used.set(id, n + 1);
      if (n > 0) id = `${id}-${n}`;
      return `<h${level} id="${id}">${text}</h${level}>`;
    }
  );
};

const ALERT = /^\s*<p>\[!(NOTE|TIP|IMPORTANT|WARNING|CAUTION)\]\s*(<\/p>)?/;

const ALERT_STYLES: Record<string, string> = {
  note: 'border-l-info bg-info/10',
  tip: 'border-l-success bg-success/10',
  important: 'border-l-secondary bg-secondary/10',
  warning: 'border-l-warning bg-warning/10',
  caution: 'border-l-error bg-error/10',
};

const ALERT_BASE =
  'my-4 rounded-r-lg border border-l-4 border-base-300 px-4 py-3 text-sm [&_p]:my-1';

/** Generated SVG diagrams can't take utilities — frame them via class injection. */
const SVG_CLASS =
  'my-4 max-w-full rounded-2xl border border-gray-200 bg-gray-50 p-4';

const frameSvgs = (html: string): string => {
  return html.replace(/<svg\b([^>]*)>/g, (match, attrs: string) => {
    const cls = attrs.match(/class="([^"]*)"/);
    if (cls) {
      return match.replace(
        `class="${cls[1]}"`,
        `class="${SVG_CLASS} ${cls[1]}"`
      );
    }
    return `<svg class="${SVG_CLASS}"${attrs}>`;
  });
};

/** Convert `> [!KIND]` blockquotes into styled alert boxes. */
const rewriteAlerts = (html: string): string => {
  return html.replace(
    /<blockquote>([\s\S]*?)<\/blockquote>/g,
    (match, inner: string) => {
      const m = inner.match(ALERT);
      if (!m) return match;
      const body = inner.slice(m[0].length);
      const open = m[2] ? '' : '<p>';
      const kind = m[1].toLowerCase();
      return `<aside class="${ALERT_BASE} ${ALERT_STYLES[kind] ?? ''}">${open}${body}</aside>`;
    }
  );
};

interface DocEntry extends DocPage {
  order: number;
}

export const getDocPages = (): DocPage[] => {
  const files: string[] = [];
  walk(DOCS_DIR, '', files);
  files.sort();
  const entries: DocEntry[] = files.map((file) => {
    const slug = slugFor(file);
    const markdown = readFileSync(path.join(DOCS_DIR, file), 'utf8');
    const fm = parseFrontmatter(markdown);
    const raw = marked.parse(fm.body, { async: false }) as string;
    return {
      slug,
      section: slug.split('/')[0] ?? '',
      title: titleFor(fm, markdown, file),
      order: fm.order,
      html: frameSvgs(rewriteAlerts(addHeadingIds(rewriteLinks(raw, slug)))),
    };
  });
  entries.sort((a, b) => a.order - b.order || a.title.localeCompare(b.title));
  return entries.map((entry) => {
    const { order, ...page } = entry;
    void order;
    return page;
  });
};

export interface DocSection {
  id: string;
  label: string;
}

export const DOC_SECTIONS: DocSection[] = [
  { id: 'guide', label: 'Guide' },
  { id: 'content', label: 'Content' },
  { id: 'project', label: 'Project' },
];
