import { describe, expect, it } from 'vitest';
import { DOC_SECTIONS, getDocPages } from '@/lib/docs';

describe('getDocPages', () => {
  it('returns the docs overview and section pages', () => {
    const pages = getDocPages();
    expect(pages.length).toBeGreaterThan(0);
    expect(pages.some((p) => p.slug === '')).toBe(true);
    for (const page of pages) {
      expect(page.title.length).toBeGreaterThan(0);
      expect(page.html.length).toBeGreaterThan(0);
    }
  });
});

describe('DOC_SECTIONS', () => {
  it('has unique section ids', () => {
    const ids = DOC_SECTIONS.map((s) => s.id);
    expect(new Set(ids).size).toBe(ids.length);
  });
});
