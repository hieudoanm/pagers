import Link from 'next/link';
import type { FC } from 'react';

const CTA: FC = () => (
  <section className='bg-bg-alt py-20'>
    <div className='mx-auto w-full max-w-[72rem] px-5 text-center'>
      <h2>Ship your docs anywhere.</h2>
      <p className='text-text-muted mt-3 text-[1.0625rem]'>
        One binary, one <code>pagers build</code>, one static folder — ready for GitHub Pages,
        any CDN, or a plain web server.
      </p>
      <div className='mt-8 flex flex-wrap justify-center gap-3'>
        <Link href='/docs/deploy/' className='btn btn-primary btn-lg'>
          Deploy guide
        </Link>
        <Link href='/docs/' className='btn btn-ghost btn-lg'>
          Browse the docs
        </Link>
      </div>
    </div>
  </section>
);

export default CTA;
