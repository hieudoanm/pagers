# AGENTS.md

## Conventions

- **Fonts:** Inter (sans) via `next/font` (`--font-inter`) in `packages/landing/src/app/layout.tsx`; mono stack `ui-monospace, 'SF Mono', SFMono-Regular, Menlo, Consolas, monospace`. Tokens: `--font-sans` / `--font-mono` (Tailwind v4 `@theme`). Do not add other typefaces.

## Footer

The landing footer follows the shared contract via `packages/landing/src/components/Footer.tsx`:
`© {year} {repo-name}` (lowercase repo name, `new Date().getFullYear()`),
a one-line tagline/license descriptor, and a right-aligned
`<nav aria-label="Footer">` with exactly two links — `GitHub` (repo root)
and `README` (`<repo>#readme`), both `target="_blank" rel="noreferrer"`.
Reuse the component on every page that shows a footer.

## Features & CTA sections

The landing page composes `Features` and `CTA` components (same directory
as `Footer`). Features: centered head (h2 + supporting p), responsive
card grid, card = icon chip + h3 + p. CTA: alt/surface background, centered
h2 + p, primary action button plus at most one ghost button. Keep copy and
link targets per-repo; keep the structure shared.

## Styling

Tailwind CSS only. `globals.css` contains just `@import 'tailwindcss';` plus
official plugins where required (`@tailwindcss/typography` for docs prose,
DaisyUI in fdr). No `@theme`, `@layer`, `@utility`, or custom selectors — style
with utilities in `className` — no shared class-string modules.
Inter comes from `next/font` `className` on `<html>`; smooth scroll uses
`scroll-smooth motion-reduce:scroll-auto`. Markdown-rendered HTML gets utility
classes emitted by `lib/docs.ts`, never custom CSS. Inline `style={{}}` is only
for genuinely dynamic values.

