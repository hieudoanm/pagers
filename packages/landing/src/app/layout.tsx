import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import type { FC, ReactNode } from 'react';
import './globals.css';

const inter = Inter({ subsets: ['latin'], variable: '--font-inter' });

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
  <html lang='en' className={inter.variable}>
    <body>{children}</body>
  </html>
);

export default RootLayout;
