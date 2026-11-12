import type { Metadata } from 'next';
import Image from 'next/image';
import Link from 'next/link';
import type { FC, JSX } from 'react';
import Footer from '@/components/Footer';
import Features, { type Feature } from '@/components/Features';
import Hero from '@/components/Hero';
import CTA from '@/components/CTA';
import {
  LuBot,
  LuFileText,
  LuGlobe,
  LuLayoutTemplate,
  LuPalette,
  LuWorkflow,
  LuZap,
} from 'react-icons/lu';

export const metadata: Metadata = {
  title: { absolute: 'pagers — static site generator for Markdown docs' },
};

const LOGO = process.env.NODE_ENV === 'production' ? '/pagers/logo.png' : '/logo.png';
const GITHUB = 'https://github.com/hieudoanm/pagers';

interface CompareRow {
  feature: string;
  pagers: string;
  docsify: string;
}

const COMPARISON: CompareRow[] = [
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

type TokKind = 'prompt' | 'cmd' | 'flag' | 'str' | 'key' | 'url' | 'dim' | 'ok' | 'err';
interface Tok {
  t: string;
  c?: TokKind;
}

const TK: Record<TokKind, string> = {
  prompt: 'text-accent select-none',
  cmd: 'font-semibold text-sky-300',
  flag: 'text-amber-400',
  str: 'text-green-300',
  key: 'text-violet-300',
  url: 'text-cyan-300',
  dim: 'text-slate-400',
  ok: 'text-gray-200',
  err: 'text-red-400',
};

const TERM_FIGURE =
  'm-0 overflow-hidden rounded-2xl border border-gray-800 bg-gray-900 shadow-xl';
const TERM_CAPTION =
  'border-b border-gray-800 bg-slate-950 px-3.5 pt-2 pb-2.5 text-xs text-slate-400';
const TERM_PRE =
  'm-0 overflow-x-auto px-5 py-4 font-mono text-xs leading-relaxed text-gray-200 whitespace-pre tabular-nums';

interface TermProps {
  caption: string;
  lines: Tok[][];
}

const Term: FC<TermProps> = ({ caption, lines }) => (
  <figure className={TERM_FIGURE}>
    <figcaption className={TERM_CAPTION}>{caption}</figcaption>
    <pre className={TERM_PRE}>
      {lines.map((line, i) => (
        <span key={i} className='block min-h-[1.7em]'>
          {line.map((tok, j) => (
            <span key={j} className={TK[tok.c ?? 'ok']}>
              {tok.t}
            </span>
          ))}
        </span>
      ))}
    </pre>
  </figure>
);

const FEATURES: Feature[] = [
  {
    icon: LuZap,
    title: 'Zero configuration',
    body: 'Point pagers at a folder of Markdown and it builds a complete site — sidebar navigation, clean trailing-slash URLs, and an "On this page" outline are generated for you.',
  },
  {
    icon: LuFileText,
    title: 'GitHub-flavoured Markdown',
    body: 'Tables, task lists, footnotes, alerts, fenced code with per-language accents, and {#custom-ids} — rendered by pulldown-cmark, no JavaScript required.',
  },
  {
    icon: LuPalette,
    title: 'Eight themes, light & dark',
    body: 'A compiled md.css with light, dark and six colour palettes. Theme follows the OS by default, with a one-click toggle persisted in localStorage.',
  },
  {
    icon: LuWorkflow,
    title: 'Built-in page graph',
    body: 'Every build emits graph.html — an interactive map of how your pages link to each other — plus graph.json and Graphviz DOT for further analysis.',
  },
  {
    icon: LuLayoutTemplate,
    title: 'Landing pages too',
    body: 'pagers landing builds a whole single-page site — marketing, waitlist, docs hub, linktree — from one landify.yaml, with 12 layouts and 64 themes.',
  },
  {
    icon: LuGlobe,
    title: 'Wikipedia importer',
    body: 'pagers wiki convert turns any Wikipedia article into clean local Markdown, rewriting internal links to filenames so whole wikis become browsable sites.',
  },
  {
    icon: LuBot,
    title: 'MCP server',
    body: 'pagers-mcp exposes build, serve and site inspection as MCP tools — let your editor or agent work with the docs directly.',
  },
];

const LandingPage = (): JSX.Element => (
  <>
    <header className='border-gray-200 bg-white/90 sticky top-0 z-50 border-b backdrop-blur'>
      <div className='mx-auto flex h-16 w-full max-w-6xl items-center gap-6 px-5'>
        <Link
          href='#top'
          className='text-gray-900 flex items-center gap-2 text-lg font-bold no-underline'>
          <Image src={LOGO} alt='' width={28} height={28} className='rounded-md' />
          <span>pagers</span>
        </Link>
        <nav className='ml-auto hidden items-center gap-5 lg:flex'>
          <a
            href='#features'
            className='text-gray-500 hover:text-gray-900 text-sm no-underline'>
            Features
          </a>
          <a
            href='#compare'
            className='text-gray-500 hover:text-gray-900 text-sm no-underline'>
            vs docsify
          </a>
          <a
            href='#quickstart'
            className='text-gray-500 hover:text-gray-900 text-sm no-underline'>
            Quickstart
          </a>
          <Link
            href='/docs/'
            className='text-gray-500 hover:text-gray-900 text-sm no-underline'>
            Docs
          </Link>
        </nav>
        <a href={GITHUB} target='_blank' rel='noreferrer' className='btn btn-primary btn-sm'>
          GitHub
        </a>
      </div>
    </header>

    <main id='top'>
      {/* hero */}
      <Hero
        title={
          <>
            Markdown docs,{' '}
            <span className='text-primary'>compiled to static HTML</span> — in
            one Rust binary
          </>
        }
        description={
          <>
            pagers turns a folder of{' '}
            <code className='rounded border border-base-300 bg-base-200 px-1.5 py-0.5 font-mono text-sm'>
              .md
            </code>{' '}
            files into a fast, self-contained static site: navigation, themes,
            clean URLs and a page graph included. No Node, no runtime, no
            server.
          </>
        }
        primary={{ label: 'Get started', href: '/docs/getting-started/' }}
        secondary={{ label: 'View on GitHub', href: GITHUB, external: true }}
        stats={[
          { value: '1', label: 'Rust binary' },
          { value: '8', label: 'built-in themes' },
          { value: '0', label: 'JavaScript required' },
        ]}>
                  <Term
            caption='first build'
            lines={[
              [
                { t: '$ ', c: 'prompt' },
                { t: 'pagers', c: 'cmd' },
                { t: ' build ', c: 'dim' },
                { t: './docs', c: 'url' },
                { t: ' --output ', c: 'dim' },
                { t: 'dist', c: 'flag' },
              ],
              [
                { t: '  ', c: 'dim' },
                { t: 'Built 13 pages and 8 assets into dist in 6.8ms', c: 'ok' },
              ],
              [
                { t: '$ ', c: 'prompt' },
                { t: 'pagers', c: 'cmd' },
                { t: ' serve ', c: 'dim' },
                { t: 'dist', c: 'url' },
              ],
              [
                { t: '➜ ', c: 'prompt' },
                { t: 'Serving dist at ', c: 'dim' },
                { t: 'http://localhost:8080', c: 'url' },
              ],
            ]}
          />
      </Hero>

      {/* features */}
      <Features
        title="Everything a docs site needs."
        description="Sensible defaults, real output — each feature is one flag or zero flags away."
        features={FEATURES}
      />

      {/* comparison */}
      <section id='compare' className='bg-gray-50 py-20'>
        <div className='mx-auto w-full max-w-6xl px-5'>
          <div className='mx-auto mb-12 max-w-2xl text-center'>
            <h2 className='m-0 mb-2 text-2xl sm:text-3xl lg:text-4xl leading-tight font-bold tracking-tight text-gray-900'>pagers vs docsify.</h2>
            <p className='text-gray-500 mt-3 text-lg'>
              Same Markdown-in, docs-out idea — opposite rendering model.
            </p>
          </div>
          <div className='border-gray-200 overflow-x-auto rounded-2xl border'>
            <table className='w-full border-collapse text-sm [&_tbody_tr:last-child>td]:border-b-0'>
              <thead>
                <tr>
                  <th className='border-gray-200 bg-gray-50 w-44 border-b px-5 py-3.5 text-left align-bottom font-semibold' />
                  <th className='border-gray-200 bg-gray-50 border-b border-l px-5 py-3.5 text-left align-bottom font-semibold'>
                    docsify
                  </th>
                  <th className='border-gray-200 bg-primary/10 border-b border-l px-5 py-3.5 text-left align-bottom'>
                    <span className='text-primary flex items-center gap-2 font-bold'>
                      pagers
                      <span className='badge badge-primary uppercase'>
                        pick
                      </span>
                    </span>
                  </th>
                </tr>
              </thead>
              <tbody>
                {COMPARISON.map((row) => (
                  <tr key={row.feature} className='group'>
                    <td className='border-gray-200 text-gray-500 border-b px-5 py-3.5 align-top font-medium'>
                      {row.feature}
                    </td>
                    <td className='border-gray-200 text-gray-500 border-b border-l px-5 py-3.5 align-top'>
                      {row.docsify}
                    </td>
                    <td className='border-gray-200 bg-primary/5 text-gray-900 group-hover:bg-primary/10 border-b border-l px-5 py-3.5 align-top font-medium'>
                      {row.pagers}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </section>

      {/* quickstart */}
      <section id='quickstart' className='py-20'>
        <div className='mx-auto w-full max-w-6xl px-5'>
          <div className='mx-auto mb-12 max-w-2xl text-center'>
            <h2 className='m-0 mb-2 text-2xl sm:text-3xl lg:text-4xl leading-tight font-bold tracking-tight text-gray-900'>Up and running in under a minute.</h2>
            <p className='text-gray-500 mt-3 text-lg'>
              Install the binary, point it at a Markdown folder, serve the result — or scaffold a
              landing page with <code className='rounded border border-gray-200 bg-gray-50 px-1.5 py-0.5 font-mono text-sm'>pagers landing new</code>.
            </p>
          </div>
          <div className='grid grid-cols-1 gap-6 md:grid-cols-2'>
            <Term
              caption='install'
              lines={[
                [
                  { t: '$ ', c: 'prompt' },
                  { t: 'curl', c: 'cmd' },
                  {
                    t: ' -fsSL https://raw.githubusercontent.com/hieudoanm/pagers/main/scripts/install.sh',
                    c: 'url',
                  },
                  { t: ' | sh', c: 'dim' },
                ],
                [
                  { t: '$ ', c: 'prompt' },
                  { t: 'cargo', c: 'cmd' },
                  { t: ' install ', c: 'dim' },
                  { t: '--path', c: 'flag' },
                  { t: ' crates/pagers-cli', c: 'dim' },
                ],
                [
                  { t: '➜ ', c: 'prompt' },
                  { t: 'installed pagers ', c: 'dim' },
                  { t: '~/.local/bin', c: 'ok' },
                ],
              ]}
            />
            <Term
              caption='build & serve'
              lines={[
                [
                  { t: '$ ', c: 'prompt' },
                  { t: 'pagers', c: 'cmd' },
                  { t: ' build ', c: 'dim' },
                  { t: './docs', c: 'url' },
                  { t: ' --output ', c: 'dim' },
                  { t: 'dist', c: 'flag' },
                ],
                [
                  { t: '  ', c: 'dim' },
                  { t: 'Built 13 pages and 8 assets into dist in 6.8ms', c: 'ok' },
                ],
                [
                  { t: '$ ', c: 'prompt' },
                  { t: 'pagers', c: 'cmd' },
                  { t: ' serve ', c: 'dim' },
                  { t: 'dist', c: 'url' },
                  { t: ' --port ', c: 'dim' },
                  { t: '8080', c: 'flag' },
                ],
                [
                  { t: '➜ ', c: 'prompt' },
                  { t: 'Serving dist at ', c: 'dim' },
                  { t: 'http://localhost:8080', c: 'url' },
                ],
              ]}
            />
          </div>
          <p className='text-gray-500 mt-8 text-center text-sm'>
            Or generate a landing page: <code className='rounded border border-gray-200 bg-gray-50 px-1.5 py-0.5 font-mono text-sm'>pagers landing new --type product</code> →{' '}
            <code className='rounded border border-gray-200 bg-gray-50 px-1.5 py-0.5 font-mono text-sm'>pagers landing build -o public/</code>. Full walkthrough in{' '}
            <Link href='/docs/guide/getting-started/' className='text-primary'>
              getting started
            </Link>
            .
          </p>
        </div>
      </section>

      <CTA
        title="Ship your docs anywhere."
        description={
          <>
            One binary, one{' '}
            <code className='rounded border border-gray-200 bg-gray-50 px-1 py-px font-mono text-sm'>
              pagers build
            </code>
            , one static folder — ready for GitHub Pages, any CDN, or a plain web
            server.
          </>
        }
        primary={{ label: 'Deploy guide', href: '/docs/deploy/' }}
        secondary={{ label: 'Browse the docs', href: '/docs/' }}
      />
    </main>

    <Footer />
  </>
);

export default LandingPage;
