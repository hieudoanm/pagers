#!/usr/bin/env ts-node
// Regenerate the themed markdown examples for the md.css module:
//   public/html/cheat-sheet.{light,dark,...}.html <- from public/cheat-sheet.md (pandoc + post-process)
//   public/images/cheat-sheet.{light,dark,...}.png <- full-page screenshots via headless Chromium
//
// Requires: pandoc (hard), sass (only when dist/ is missing), playwright (optional).

import { execFileSync, spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, readdirSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';

const ROOT = dirname(__dirname);
const PUBLIC_DIR = join(ROOT, 'public');
const HTML_DIR = join(PUBLIC_DIR, 'html');
const IMAGES_DIR = join(PUBLIC_DIR, 'images');
const MARKDOWN = join(PUBLIC_DIR, 'cheat-sheet.md');
const DIST_CSS = join(ROOT, 'dist', 'md.css');
const BUILD_SH = join(ROOT, 'scripts', 'build.sh');

const THEMES = [
  { name: 'light', mdClass: 'md md-light', scheme: 'light', pageBg: '#ffffff' },
  { name: 'dark', mdClass: 'md md-dark', scheme: 'dark', pageBg: '#1b1b1f' },
  {
    name: 'crimson',
    mdClass: 'md md-crimson',
    scheme: 'dark',
    pageBg: '#1b1b1f',
  },
  {
    name: 'cerulean',
    mdClass: 'md md-cerulean',
    scheme: 'light',
    pageBg: '#ffffff',
  },
  {
    name: 'emerald',
    mdClass: 'md md-emerald',
    scheme: 'light',
    pageBg: '#ffffff',
  },
  {
    name: 'saffron',
    mdClass: 'md md-saffron',
    scheme: 'light',
    pageBg: '#ffffff',
  },
  {
    name: 'lagoon',
    mdClass: 'md md-lagoon',
    scheme: 'dark',
    pageBg: '#1b1b1f',
  },
  {
    name: 'fuchsia',
    mdClass: 'md md-fuchsia',
    scheme: 'dark',
    pageBg: '#1b1b1f',
  },
];

interface PageLike {
  goto(url: string, options?: object): Promise<void>;
  waitForTimeout(ms: number): Promise<void>;
  screenshot(options: { path: string; fullPage: boolean }): Promise<void>;
  close(): Promise<void>;
}

interface BrowserLike {
  newPage(options: object): Promise<PageLike>;
  close(): Promise<void>;
}

const ensureDist = (): void => {
  if (existsSync(DIST_CSS)) return;
  console.log('dist CSS is missing; rebuilding via scripts/build.sh');
  execFileSync('bash', [BUILD_SH], { stdio: 'inherit' });
};

const findPlaywright = (startDir: string): string | null => {
  let dir = startDir;
  for (;;) {
    const pnpmStore = join(dir, 'node_modules', '.pnpm');
    if (existsSync(pnpmStore)) {
      const matches = readdirSync(pnpmStore).filter((entry) =>
        entry.startsWith('playwright@')
      );
      for (const entry of matches) {
        const candidate = join(pnpmStore, entry, 'node_modules', 'playwright');
        if (existsSync(candidate)) return candidate;
      }
    }
    const parent = dirname(dir);
    if (parent === dir) return null;
    dir = parent;
  }
};

const highlightJson = (body: string): string => {
  const keys: string[] = [];
  return body
    .replace(/&quot;[^&]*&quot;(?=\s*:)/g, (s: string) => {
      keys.push(s);
      return '\x00A';
    })
    .replace(
      /&quot;[^&]*&quot;/g,
      (s: string) => `<span class="hljs-string">${s}</span>`
    )
    .replace(
      /\b(\d+)\b/g,
      (_s: string, n: string) => `<span class="hljs-number">${n}</span>`
    )
    .replace(
      /\x00A/g,
      () => `<span class="hljs-attribute">${keys.shift() ?? ''}</span>`
    );
};

const convertTaskLists = (doc: string): string => {
  return doc
    .replace('<ul class="task-list">', '<ul class="contains-task-list">')
    .replace(
      /<li><label><input type="checkbox" checked="" \/>([\s\S]*?)<\/label><\/li>/g,
      '<li class="task-list-item"><input type="checkbox" checked disabled="" />$1</li>'
    )
    .replace(
      /<li><label><input type="checkbox" \/>([\s\S]*?)<\/label><\/li>/g,
      '<li class="task-list-item"><input type="checkbox" disabled="" />$1</li>'
    );
};

const convertFootnotes = (doc: string): string => {
  return doc
    .replace(
      /<a href="#fn1"\s*class="footnote-ref" id="fnref1" role="doc-noteref"><sup>1<\/sup><\/a>/,
      '<sup id="fnref1" data-footnote-ref><a href="#fn1">1</a></sup>'
    )
    .replace(
      'class="footnote-back" role="doc-backlink">',
      'data-footnote-backref class="footnote-back" role="doc-backlink">'
    );
};

const convertInline = (doc: string): string => {
  return doc
    .replace('H<del>2</del>O', 'H<sub>2</sub>O')
    .replace('X^2^', 'X<sup>2</sup>')
    .replace('==very important words==', '<mark>very important words</mark>')
    .replace(
      '<p>term : definition</p>',
      '<dl><dt>term</dt><dd>definition</dd></dl>'
    )
    .replace(
      '<h3 id="my-great-heading-custom-id">My Great Heading {#custom-id}</h3>',
      '<h3 id="custom-id">My Great Heading</h3>'
    );
};

const highlightJsonBlock = (doc: string): string => {
  return doc.replace(
    /<pre><code>([\s\S]*?)<\/code><\/pre>/g,
    (match: string, body: string) => {
      if (!body.includes('firstName')) return match;
      return `<pre><code class="hljs language-json">${highlightJson(body)}</code></pre>`;
    }
  );
};

const addAnchors = (doc: string): string => {
  return doc.replace(
    /<h([1-6]) id="([^"]+)">/g,
    '<h$1 id="$2"><a class="anchor" href="#$2" aria-hidden="true">#</a>'
  );
};

const postProcess = (raw: string): string => {
  let doc = raw;
  doc = convertTaskLists(doc);
  doc = convertFootnotes(doc);
  doc = convertInline(doc);
  doc = highlightJsonBlock(doc);
  doc = addAnchors(doc);
  return doc;
};

const shell = (
  mdClass: string,
  scheme: string,
  pageBg: string,
  doc: string
): string => {
  return `<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Markdown Cheat Sheet</title>
    <link rel="stylesheet" href="../../dist/md.css" />
    <style>
      :root {
        color-scheme: ${scheme};
        --page-bg: ${pageBg};
      }
      body {
        margin: 0;
        background: var(--page-bg, #ffffff);
      }
      .md {
        max-width: 860px;
        margin: 0 auto;
        padding: 40px 32px 64px;
      }
    </style>
  </head>
  <body><div class="${mdClass}">\n${doc}\n</div>\n</body>
</html>
`;
};

const renderScreenshots = async (playwrightPath: string): Promise<void> => {
  let chromium: { launch(): Promise<BrowserLike> };
  try {
    ({ chromium } = require(playwrightPath));
  } catch (error) {
    console.warn(
      `warning: playwright failed to load, skipping PNG rendering: ${(error as Error).message}`
    );
    return;
  }

  let browser: BrowserLike;
  try {
    browser = await chromium.launch();
  } catch (error) {
    console.warn(
      `warning: could not launch chromium, skipping PNG rendering: ${(error as Error).message}`
    );
    console.warn('install it with: pnpm exec playwright install chromium');
    return;
  }

  try {
    for (const { name } of THEMES) {
      const png = `cheat-sheet.${name}.png`;
      const page = await browser.newPage({
        viewport: { width: 920, height: 800 },
        deviceScaleFactor: 2,
      });
      await page.goto(`file://${join(HTML_DIR, `cheat-sheet.${name}.html`)}`, {
        waitUntil: 'networkidle',
      });
      await page.waitForTimeout(400);
      await page.screenshot({ path: join(IMAGES_DIR, png), fullPage: true });
      console.log('screenshot', png);
      await page.close();
    }
  } finally {
    await browser.close();
  }
};

const main = (): void => {
  if (spawnSync('which', ['pandoc'], { stdio: 'ignore' }).status !== 0) {
    console.error('error: pandoc is required but was not found on PATH');
    process.exit(1);
  }

  ensureDist();
  mkdirSync(HTML_DIR, { recursive: true });
  mkdirSync(IMAGES_DIR, { recursive: true });

  const raw = execFileSync('pandoc', ['-f', 'gfm', '-t', 'html5', MARKDOWN], {
    encoding: 'utf-8',
  });
  const doc = postProcess(raw);

  for (const { name, mdClass, scheme, pageBg } of THEMES) {
    const file = `cheat-sheet.${name}.html`;
    writeFileSync(join(HTML_DIR, file), shell(mdClass, scheme, pageBg, doc));
    console.log('wrote', file);
  }

  const playwright = findPlaywright(ROOT);
  if (!playwright) {
    console.warn('warning: playwright not found; skipping PNG rendering');
    return;
  }
  renderScreenshots(playwright).catch((error: unknown) => {
    console.error(error);
    process.exit(1);
  });
};

main();
