---
title: Markdown
order: 4
---

# Markdown features

Rendering is GitHub-flavoured Markdown via `pulldown-cmark`, extended with
heading attributes, alerts and site-aware link rewriting.

## Supported syntax

- Headings `#`–`######` with **automatic ids** for anchor links.
- Emphasis `*italic*`/`_italic_`, strong `**bold**`, `~~strikethrough~~`.
- Ordered and unordered lists, including **nested** lists and **task lists**
  (`- [x] done`).
- GFM **tables** with alignment row.
- **Code blocks** with language identifiers, inline `` `code` ``.
- **Blockquotes**, horizontal rules, images `![alt](src)`.
- **Footnotes** `[^1]` with a rendered references section.
- **Raw HTML** passes through untouched.

## Heading ids

Headings get slug ids automatically (`## Install steps` → `#install-steps`).
Override with the attribute syntax:

```markdown
## Install steps {#setup}
```

## Alerts

GitHub-style alert blockquotes render as styled callouts (the markup the
`md.css` module styles):

```markdown
> [!NOTE] Rendered as a styled callout.

> [!TIP] Five supported kinds: NOTE, TIP, IMPORTANT, WARNING, CAUTION.
```

## Internal links

Links between content files are rewritten to the site's clean URLs at build
time:

```markdown
[Install](guide/install.md) → /guide/install/ [Section](guide/) → /guide/
[Absolute](/guide/install.md) → /guide/install/
```

Relative `../` paths resolve against the source file's location. Links that
don't point at a known page are left untouched so broken references stay
visible. External `https://` links are preserved as-is.

## What you get for free

- Table of contents data (heading outline) is extracted per page for the theme's
  use.
- `draft: true` frontmatter excludes a page without deleting it.
- Non-Markdown files (`png`, `pdf`, `css`, …) are copied through verbatim and
  can be linked normally.
