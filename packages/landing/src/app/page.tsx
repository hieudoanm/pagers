import type { Metadata } from 'next';
import Image from 'next/image';
import Link from 'next/link';
import type { JSX } from 'react';

export const metadata: Metadata = {
  title: { absolute: 'pagers — static site generator for Markdown docs' },
};

const LOGO = process.env.NODE_ENV === 'production' ? '/pagers/logo.png' : '/logo.png';
const GITHUB = 'https://github.com/hieudoanm/pagers';

const FEATURES: { title: string; body: string; href: string }[] = [
  {
    title: 'Zero configuration',
    body: 'Point pagers at a folder of Markdown and it builds a complete site — sidebar navigation, clean trailing-slash URLs, and an "On this page" outline are generated for you.',
    href: '/docs/getting-started/',
  },
  {
    title: 'GitHub-flavoured Markdown',
    body: 'Tables, task lists, footnotes, alerts, fenced code with per-language accents, and {#custom-ids} — rendered by pulldown-cmark, no JavaScript required.',
    href: '/docs/markdown/',
  },
  {
    title: 'Eight themes, light & dark',
    body: 'A compiled md.css with light, dark and six colour palettes. Theme follows the OS by default, with a one-click toggle persisted in localStorage.',
    href: '/docs/theming/',
  },
  {
    title: 'Built-in page graph',
    body: 'Every build emits graph.html — an interactive map of how your pages link to each other — plus graph.json and Graphviz DOT for further analysis.',
    href: '/docs/graph/',
  },
  {
    title: 'Wikipedia importer',
    body: 'pagers wiki convert turns any Wikipedia article into clean local Markdown, rewriting internal links to filenames so whole wikis become browsable sites.',
    href: '/docs/wiki/',
  },
  {
    title: 'MCP server',
    body: 'pagers-mcp exposes build, serve and site inspection as MCP tools — let your editor or agent work with the docs directly.',
    href: '/docs/mcp/',
  },
];

const COMPARISON: { feature: string; pagers: string; docsify: string }[] = [
  {
    feature: 'Rendering',
    pagers: 'Ahead of time → static HTML',
    docsify: 'In the browser at runtime',
  },
  { feature: 'JavaScript required', pagers: 'None for content', docsify: 'Required' },
  { feature: 'Hosting', pagers: 'Any static host', docsify: 'Needs correct SPA fallback' },
  { feature: 'Offline / cacheable', pagers: 'Fully', docsify: 'Only the shell' },
  { feature: 'Search engines', pagers: 'Full HTML + sitemap', docsify: 'Limited' },
  { feature: 'Runtime', pagers: 'One Rust binary', docsify: 'Node at build time' },
];

const LandingPage = (): JSX.Element => (
  <>
    <header className='border-border bg-bg/90 sticky top-0 z-50 border-b backdrop-blur'>
      <div className='mx-auto flex h-16 w-full max-w-[72rem] items-center gap-6 px-5'>
        <Link href='/' className='text-text flex items-center gap-2 text-lg font-bold no-underline'>
          <Image src={LOGO} alt='' width={28} height={28} className='rounded-md' />
          <span>pagers</span>
        </Link>
        <nav className='ml-auto flex items-center gap-5 text-[0.9375rem]'>
          <Link href='/docs/' className='text-text-muted hover:text-text no-underline'>
            Docs
          </Link>
          <a href='#features' className='text-text-muted hover:text-text no-underline'>
            Features
          </a>
          <a href={GITHUB} target='_blank' rel='noreferrer' className='btn btn-primary btn-sm'>
            GitHub
          </a>
        </nav>
      </div>
    </header>

    {/* Hero */}
    <section className='mx-auto w-full max-w-[72rem] px-5 pt-20 pb-16 text-center'>
      <h1 className='mx-auto max-w-3xl'>
        Markdown docs, <span className='text-accent'>compiled to static HTML</span> — in one Rust
        binary
      </h1>
      <p className='text-text-muted mx-auto mt-6 max-w-2xl text-lg'>
        pagers turns a folder of <code>.md</code> files into a fast, self-contained static site:
        navigation, themes, clean URLs and a page graph included. No Node, no runtime, no server.
      </p>
      <div className='mt-8 flex items-center justify-center gap-3'>
        <Link href='/docs/getting-started/' className='btn btn-primary btn-lg'>
          Get started
        </Link>
        <a href={GITHUB} target='_blank' rel='noreferrer' className='btn btn-ghost btn-lg'>
          View on GitHub
        </a>
      </div>

      <figure className='terminal mx-auto mt-14 max-w-2xl text-left'>
        <figcaption>terminal</figcaption>
        <pre>
          <code>
            <span className='prompt'>$</span> <span className='tk-cmd'>pagers</span>{' '}
            <span className='tk-flag'>build</span> ./docs <span className='tk-flag'>--output</span>{' '}
            dist
            {'\n'}
            <span className='tk-dim'>Built 13 pages and 8 assets into dist in 6.8ms</span>
          </code>
        </pre>
      </figure>
    </section>

    {/* Features */}
    <section id='features' className='border-border scroll-mt-20 border-t'>
      <div className='mx-auto w-full max-w-[72rem] px-5 py-16'>
        <h2 className='text-center'>Everything a docs site needs</h2>
        <p className='text-text-muted mx-auto mt-3 max-w-xl text-center'>
          Sensible defaults, real output — each feature is one flag or zero flags away.
        </p>
        <div className='mt-10 grid gap-5 sm:grid-cols-2 lg:grid-cols-3'>
          {FEATURES.map((f) => (
            <Link
              key={f.title}
              href={f.href}
              className='border-border bg-surface hover:border-border-strong block rounded-[var(--radius-card)] border p-6 no-underline transition-colors'>
              <h3 className='text-base'>{f.title}</h3>
              <p className='text-text-muted mt-2 text-sm leading-relaxed'>{f.body}</p>
            </Link>
          ))}
        </div>
      </div>
    </section>

    {/* Comparison */}
    <section className='border-border bg-bg-alt scroll-mt-20 border-t'>
      <div className='mx-auto w-full max-w-[72rem] px-5 py-16'>
        <h2 className='text-center'>pagers vs docsify</h2>
        <p className='text-text-muted mx-auto mt-3 max-w-xl text-center'>
          Same Markdown-in, docs-out idea — opposite rendering model.
        </p>
        <div className='border-border bg-surface mx-auto mt-10 max-w-3xl overflow-hidden rounded-[var(--radius-card)] border'>
          <table className='w-full text-sm'>
            <thead>
              <tr className='border-border border-b text-left'>
                <th className='px-5 py-3 font-semibold' />
                <th className='text-accent px-5 py-3 font-semibold'>pagers</th>
                <th className='text-text-muted px-5 py-3 font-semibold'>docsify</th>
              </tr>
            </thead>
            <tbody>
              {COMPARISON.map((row) => (
                <tr key={row.feature} className='border-border border-b last:border-0'>
                  <td className='text-text-muted px-5 py-3 font-medium'>{row.feature}</td>
                  <td className='px-5 py-3'>{row.pagers}</td>
                  <td className='text-text-muted px-5 py-3'>{row.docsify}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </section>

    {/* CTA */}
    <section className='border-border scroll-mt-20 border-t'>
      <div className='mx-auto w-full max-w-[72rem] px-5 py-20 text-center'>
        <h2>Ship your docs anywhere</h2>
        <p className='text-text-muted mx-auto mt-4 max-w-xl'>
          One binary, one <code>pagers build</code>, one static folder — ready for GitHub Pages, any
          CDN, or a plain web server.
        </p>
        <div className='mt-8 flex items-center justify-center gap-3'>
          <Link href='/docs/deploy/' className='btn btn-primary btn-lg'>
            Deploy guide
          </Link>
          <Link href='/docs/' className='btn btn-ghost btn-lg'>
            Browse the docs
          </Link>
        </div>
      </div>
    </section>

    <footer className='border-border text-text-muted border-t py-10 text-sm'>
      <div className='mx-auto flex w-full max-w-[72rem] flex-wrap items-center justify-between gap-3 px-5'>
        <span>pagers — static sites from Markdown, in one Rust binary</span>
        <a href={GITHUB} target='_blank' rel='noreferrer' className='hover:text-text'>
          github.com/hieudoanm/pagers
        </a>
      </div>
    </footer>
  </>
);

export default LandingPage;
