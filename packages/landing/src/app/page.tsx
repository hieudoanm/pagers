import type { Metadata } from 'next';
import Image from 'next/image';
import Link from 'next/link';
import type { FC, JSX } from 'react';
import Footer from '@/components/Footer';
import Features from '@/components/Features';
import CTA from '@/components/CTA';

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
  prompt: 'prompt',
  cmd: 'tk-cmd',
  flag: 'tk-flag',
  str: 'tk-str',
  key: 'tk-key',
  url: 'tk-url',
  dim: 'tk-dim',
  ok: 'tk-ok',
  err: 'tk-err',
};

interface TermProps {
  caption: string;
  lines: Tok[][];
}

const Term: FC<TermProps> = ({ caption, lines }) => (
  <figure className='terminal'>
    <figcaption>{caption}</figcaption>
    <pre>
      {lines.map((line, i) => (
        <span key={i} className='tk-line'>
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

const LandingPage = (): JSX.Element => (
  <>
    <header className='border-border bg-bg/90 sticky top-0 z-50 border-b backdrop-blur'>
      <div className='mx-auto flex h-16 w-full max-w-[72rem] items-center gap-6 px-5'>
        <Link
          href='#top'
          className='text-text flex items-center gap-2 text-lg font-bold no-underline'>
          <Image src={LOGO} alt='' width={28} height={28} className='rounded-md' />
          <span>pagers</span>
        </Link>
        <nav className='ml-auto hidden items-center gap-5 lg:flex'>
          <a
            href='#features'
            className='text-text-muted hover:text-text text-[0.9375rem] no-underline'>
            Features
          </a>
          <a
            href='#compare'
            className='text-text-muted hover:text-text text-[0.9375rem] no-underline'>
            vs docsify
          </a>
          <a
            href='#quickstart'
            className='text-text-muted hover:text-text text-[0.9375rem] no-underline'>
            Quickstart
          </a>
          <Link
            href='/docs/'
            className='text-text-muted hover:text-text text-[0.9375rem] no-underline'>
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
      <section className='pt-20 pb-16'>
        <div className='mx-auto grid w-full max-w-[72rem] items-center gap-12 px-5 text-left lg:grid-cols-[1.05fr_1fr]'>
          <div>
            <h1>
              Markdown docs, <span className='text-accent'>compiled to static HTML</span> — in one
              Rust binary
            </h1>
            <p className='text-text-muted mt-6 max-w-xl text-[1.1875rem]'>
              pagers turns a folder of <code>.md</code> files into a fast, self-contained static
              site: navigation, themes, clean URLs and a page graph included. No Node, no runtime,
              no server.
            </p>
            <div className='mt-8 flex flex-wrap justify-start gap-3'>
              <Link href='/docs/getting-started/' className='btn btn-primary btn-lg'>
                Get started
              </Link>
              <a href={GITHUB} target='_blank' rel='noreferrer' className='btn btn-ghost btn-lg'>
                View on GitHub
              </a>
            </div>
            <dl className='mt-10 grid grid-cols-3 gap-6'>
              <div>
                <dt className='text-2xl font-bold tabular-nums'>1</dt>
                <dd className='text-text-muted m-0 text-sm'>Rust binary</dd>
              </div>
              <div>
                <dt className='text-2xl font-bold tabular-nums'>8</dt>
                <dd className='text-text-muted m-0 text-sm'>built-in themes</dd>
              </div>
              <div>
                <dt className='text-2xl font-bold tabular-nums'>0</dt>
                <dd className='text-text-muted m-0 text-sm'>JavaScript required</dd>
              </div>
            </dl>
          </div>
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
        </div>
      </section>

      {/* features */}
      <Features />

      {/* comparison */}
      <section id='compare' className='bg-bg-alt py-20'>
        <div className='mx-auto w-full max-w-[72rem] px-5'>
          <div className='mx-auto mb-12 max-w-[44rem] text-center'>
            <h2>pagers vs docsify.</h2>
            <p className='text-text-muted mt-3 text-[1.0625rem]'>
              Same Markdown-in, docs-out idea — opposite rendering model.
            </p>
          </div>
          <div className='border-border overflow-x-auto rounded-[var(--radius-card)] border'>
            <table className='w-full border-collapse text-[0.9375rem] [&_tbody_tr:last-child>td]:border-b-0'>
              <thead>
                <tr>
                  <th className='border-border bg-bg-alt w-44 border-b px-5 py-3.5 text-left align-bottom font-semibold' />
                  <th className='border-border bg-bg-alt border-b border-l px-5 py-3.5 text-left align-bottom font-semibold'>
                    docsify
                  </th>
                  <th className='border-border bg-accent-soft border-b border-l px-5 py-3.5 text-left align-bottom'>
                    <span className='text-accent flex items-center gap-2 font-bold'>
                      pagers
                      <span className='bg-accent rounded-full px-2 py-0.5 text-[0.6875rem] font-semibold tracking-wide text-white uppercase'>
                        pick
                      </span>
                    </span>
                  </th>
                </tr>
              </thead>
              <tbody>
                {COMPARISON.map((row) => (
                  <tr key={row.feature} className='group'>
                    <td className='border-border text-text-muted border-b px-5 py-3.5 align-top font-medium'>
                      {row.feature}
                    </td>
                    <td className='border-border text-text-muted border-b border-l px-5 py-3.5 align-top'>
                      {row.docsify}
                    </td>
                    <td className='border-border bg-accent-soft/50 text-text group-hover:bg-accent-soft border-b border-l px-5 py-3.5 align-top font-medium'>
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
        <div className='mx-auto w-full max-w-[72rem] px-5'>
          <div className='mx-auto mb-12 max-w-[44rem] text-center'>
            <h2>Up and running in under a minute.</h2>
            <p className='text-text-muted mt-3 text-[1.0625rem]'>
              Install the binary, point it at a Markdown folder, serve the result — or scaffold a
              landing page with <code>pagers landing new</code>.
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
          <p className='text-text-muted mt-8 text-center text-[0.9375rem]'>
            Or generate a landing page: <code>pagers landing new --type product</code> →{' '}
            <code>pagers landing build -o public/</code>. Full walkthrough in{' '}
            <Link href='/docs/guide/getting-started/' className='text-accent'>
              getting started
            </Link>
            .
          </p>
        </div>
      </section>

      <CTA />
    </main>

    <Footer />
  </>
);

export default LandingPage;
