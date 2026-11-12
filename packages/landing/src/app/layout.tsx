import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import type { FC, ReactNode } from 'react';
import Footer from '@/components/molecules/Footer';
import Navbar from '@/components/molecules/Navbar';
import './globals.css';

const inter = Inter({ subsets: ['latin'] });

const GITHUB = 'https://github.com/hieudoanm/pagers';
const LOGO =
  process.env.NODE_ENV === 'production' ? '/pagers/logo.png' : '/logo.png';
const NAV_LINKS = [
  { label: 'Features', href: '#features' },
  { label: 'vs docsify', href: '#compare' },
  { label: 'Quickstart', href: '#quickstart' },
  { label: 'Docs', href: '/docs/' },
];

export const metadata: Metadata = {
  metadataBase: new URL('https://hieudoanm.github.io/pagers/'),
  title: {
    default: 'pagers — static site generator for Markdown docs',
    template: '%s — pagers',
  },
  description:
    'A single Rust binary that turns a folder of Markdown into a fast, self-contained static site — clean URLs, navigation, themes, a page graph, and a Wikipedia importer. No Node, no runtime.',
  openGraph: {
    siteName: 'pagers',
    type: 'website',
    title: 'pagers — static site generator for Markdown docs',
    description:
      'Markdown folders → static HTML in one Rust binary. Clean URLs, themes, graphs, Wikipedia import, MCP server.',
    url: 'https://hieudoanm.github.io/pagers/',
    images: ['/og.png'],
  },
  twitter: { card: 'summary_large_image', images: ['/og.png'] },
};

interface RootLayoutProps {
  children: ReactNode;
}

const RootLayout: FC<RootLayoutProps> = ({ children }) => (
  <html
    lang="en"
    data-theme="pagers"
    className={`scroll-smooth antialiased motion-reduce:scroll-auto ${inter.className}`}>
    <body className="bg-base-100 text-base-content m-0 leading-relaxed">
      <Navbar
        name="pagers"
        logoSrc={LOGO}
        links={NAV_LINKS}
        githubUrl={GITHUB}
      />
      {children}
      <Footer
        name="pagers"
        tagline="Static sites from Markdown, in one Rust binary · MIT"
        githubUrl={GITHUB}
      />
      
    </body>
  </html>
);

export default RootLayout;
