import type { FC } from 'react';
import type { IconType } from 'react-icons';
import {
  LuBot,
  LuFileText,
  LuGlobe,
  LuLayoutTemplate,
  LuPalette,
  LuWorkflow,
  LuZap,
} from 'react-icons/lu';

interface Feature {
  icon: IconType;
  title: string;
  body: string;
}

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

const Features: FC = () => (
  <section id='features' className='py-20'>
    <div className='mx-auto w-full max-w-[72rem] px-5'>
      <div className='mx-auto mb-12 max-w-[44rem] text-center'>
        <h2>Everything a docs site needs.</h2>
        <p className='text-text-muted mt-3 text-[1.0625rem]'>
          Sensible defaults, real output — each feature is one flag or zero flags away.
        </p>
      </div>
      <div className='grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3'>
        {FEATURES.map((f) => (
          <article
            key={f.title}
            className='border-border bg-surface rounded-[var(--radius-card)] border p-6'>
            <div className='bg-accent-soft text-accent mb-3.5 flex h-11 w-11 items-center justify-center rounded-lg'>
              <f.icon className='h-[1.375rem] w-[1.375rem]' />
            </div>
            <h3 className='mb-2'>{f.title}</h3>
            <p className='text-text-muted text-[0.9375rem]'>{f.body}</p>
          </article>
        ))}
      </div>
    </div>
  </section>
);

export default Features;
