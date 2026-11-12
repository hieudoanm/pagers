import type { Metadata } from 'next';
import Image from 'next/image';
import Link from 'next/link';
import { notFound } from 'next/navigation';
import { DOC_SECTIONS, getDocPages, type DocPage } from '@/lib/docs';
import Footer from '@/components/Footer';
import type { JSX } from 'react';

// next/image with unoptimized does not apply basePath — mirror next.config.ts.
const LOGO = process.env.NODE_ENV === 'production' ? '/pagers/logo.svg' : '/logo.svg';

export const generateStaticParams = () =>
  getDocPages().map((p) => ({
    slug: p.slug === '' ? [] : p.slug.split('/'),
  }));

const findDoc = (slug: string[]): DocPage | undefined => {
  const key = slug.join('/');
  return getDocPages().find((p) => p.slug === key);
};

interface DocsPageProps {
  params: Promise<{ slug?: string[] }>;
}

export const generateMetadata = async ({ params }: DocsPageProps): Promise<Metadata> => {
  const { slug = [] } = await params;
  const doc = findDoc(slug);
  return { title: doc ? doc.title : 'Docs' };
};

const DocsPage = async ({ params }: DocsPageProps): Promise<JSX.Element> => {
  const { slug = [] } = await params;
  const pages = getDocPages();
  const doc = findDoc(slug);
  if (!doc) notFound();
  const overview = pages.find((p) => p.slug === '');

  return (
    <>
      <header className='border-border bg-bg/90 sticky top-0 z-50 border-b backdrop-blur'>
        <div className='mx-auto flex h-16 w-full max-w-[72rem] items-center gap-6 px-5'>
          <Link
            href='/'
            className='text-text flex items-center gap-2 text-lg font-bold no-underline'>
            <Image src={LOGO} alt='' width={28} height={28} className='rounded-md' />
            <span>pagers</span>
          </Link>
          <nav className='ml-auto flex items-center gap-5 text-[0.9375rem]'>
            <Link href='/' className='text-text-muted hover:text-text no-underline'>
              Home
            </Link>
            <Link href='/docs/' className='text-accent font-semibold no-underline'>
              Docs
            </Link>
            <Link
              href='https://github.com/hieudoanm/pagers'
              target='_blank'
              rel='noreferrer'
              className='btn btn-primary btn-sm'>
              GitHub
            </Link>
          </nav>
        </div>
      </header>

      <div className='mx-auto w-full max-w-[72rem] px-5 py-10'>
        <div className='docs-layout grid grid-cols-1 gap-10 md:grid-cols-[16rem_1fr]'>
          <aside className='docs-sidebar md:sticky md:top-24 md:self-start'>
            <Link href='/docs/' className='text-text text-sm font-bold no-underline'>
              Documentation
            </Link>
            {overview && (
              <Link
                href='/docs/'
                className={`mt-2 block rounded-lg px-3 py-1.5 text-sm no-underline ${
                  doc.slug === ''
                    ? 'bg-accent-soft text-accent font-semibold'
                    : 'text-text-muted hover:bg-bg-alt hover:text-text'
                }`}>
                {overview.title}
              </Link>
            )}
            {DOC_SECTIONS.map((s) => {
              const items = pages.filter((p) => p.section === s.id);
              if (items.length === 0) return null;
              return (
                <div key={s.id}>
                  <div className='text-text-tertiary mt-5 mb-1.5 text-[0.75rem] font-semibold tracking-wider uppercase'>
                    {s.label}
                  </div>
                  {items.map((p) => (
                    <Link
                      key={p.slug}
                      href={`/docs/${p.slug}/`}
                      className={`block rounded-lg px-3 py-1.5 text-sm no-underline ${
                        p.slug === doc.slug
                          ? 'bg-accent-soft text-accent font-semibold'
                          : 'text-text-muted hover:bg-bg-alt hover:text-text'
                      }`}>
                      {p.title}
                    </Link>
                  ))}
                </div>
              );
            })}
          </aside>

          <article>
            <div className='prose max-w-none' dangerouslySetInnerHTML={{ __html: doc.html }} />
          </article>
        </div>
      </div>

      <Footer />
    </>
  );
};

export default DocsPage;
