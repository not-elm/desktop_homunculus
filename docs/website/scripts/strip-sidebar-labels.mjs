// Remove sidebar_label from generated API MDX files
// so that sidebar.ts labels (path-based) are used instead.
import { readdir, readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";

const apiDir = join(import.meta.dirname, "../docs/reference/api");
const files = await readdir(apiDir);

for (const file of files) {
  if (!file.endsWith(".api.mdx")) continue;
  const path = join(apiDir, file);
  const content = await readFile(path, "utf-8");
  const updated = content.replace(/^sidebar_label:.*\n/m, "");
  if (updated !== content) await writeFile(path, updated);
}
