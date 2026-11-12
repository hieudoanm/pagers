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
