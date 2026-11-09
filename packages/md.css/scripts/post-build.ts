import { stat, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const files = ['md.css', 'md.css.map', 'md.min.css', 'md.min.css.map'];

type FileMetadata = Record<string, { size: number; kb: number }>;

const toTwoDecimals = (value: number): number => Math.round(value * 100) / 100;

const collect = async (): Promise<FileMetadata> => {
  const metadata: FileMetadata = {};
  for (const file of files) {
    const { size } = await stat(join(process.cwd(), 'dist', file));
    metadata[file] = { size, kb: toTwoDecimals(size / 1024) };
  }
  return metadata;
};

const main = async (): Promise<void> => {
  const distDir = join(process.cwd(), 'dist');
  const metadata = await collect();

  const outputPath = join(distDir, 'md.metadata.json');
  await writeFile(outputPath, `${JSON.stringify(metadata, null, 2)}\n`);
  console.log(JSON.stringify(metadata, null, 2));
};

main().catch((error: unknown) => {
  console.error(error);
  process.exit(1);
});
