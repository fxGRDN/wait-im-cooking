import { mkdir, readFile, writeFile } from "@tauri-apps/plugin-fs";
import { open } from "@tauri-apps/plugin-dialog";
import { appDataDir, join } from "@tauri-apps/api/path";

export const saveImages = async (
  folder: string,
  multiple: boolean = false,
): Promise<string[]> => {
  const appDir = await appDataDir();
  const targetDir = await join(appDir, folder);

  await mkdir(targetDir, { recursive: true });

  const picked = await open({
    multiple,
    filters: [{ name: "Image", extensions: ["png", "jpg", "jpeg", "webp"] }],
  });

  if (!picked) return [];

  const paths: string[] = Array.isArray(picked) ? picked : [picked];
  const results: string[] = [];

  for (const path of paths) {
    let sourcePath = path;

    // For standard file paths, strip file:// and decode.
    // For content:// (Android), pass it directly to readFile.
    if (sourcePath.startsWith("file://")) {
      sourcePath = decodeURIComponent(sourcePath.substring(7));
    } else if (!sourcePath.startsWith("content://")) {
      sourcePath = decodeURIComponent(sourcePath);
    }

    const filename = sourcePath.split("/").pop();
    const destPath = await join(targetDir, filename!);

    console.log(`Reading from ${sourcePath} and writing to ${destPath}`);
    const bytes = await readFile(sourcePath);
    await writeFile(destPath, bytes);

    console.log(`Saved to ${destPath}`);
    results.push(destPath);
  }
  return results;
};
