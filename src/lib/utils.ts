import { mkdir, readFile, writeFile } from "@tauri-apps/plugin-fs";
import { open } from "@tauri-apps/plugin-dialog";
import { appDataDir, join } from "@tauri-apps/api/path";

export const COMMON_UNITS = ["g", "kg", "ml", "l", "pcs"] as const;

export const saveFiles = async (
  folder: string,
  filters: { name: string; extensions: string[] }[],
  multiple: boolean = false,
): Promise<string[]> => {
  try {
    const appDir = await appDataDir();
    const targetDir = await join(appDir, folder);

    await mkdir(targetDir, { recursive: true });

    const picked = await open({
      multiple,
      filters,
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

      // Safely get extension
      let ext = "";
      const parts = sourcePath.split(".");
      if (parts.length > 1) {
        const lastPart = parts.pop()!;
        // Ensure extension doesn't contain path separators
        if (!lastPart.includes("/") && !lastPart.includes("\\")) {
          ext = `.${lastPart}`;
        }
      }

      const filename = `${crypto.randomUUID()}${ext}`;
      const destPath = await join(targetDir, filename);

      console.log(`[UTILS] Saving ${sourcePath} -> ${destPath}`);
      const bytes = await readFile(sourcePath);
      await writeFile(destPath, bytes);

      results.push(destPath);
    }
    return results;
  } catch (err) {
    console.error("[UTILS] Failed to save files:", err);
    throw err;
  }
};

export const saveImages = async (
  folder: string,
  multiple: boolean = false,
): Promise<string[]> => {
  return saveFiles(
    folder,
    [{ name: "Image", extensions: ["png", "jpg", "jpeg", "webp"] }],
    multiple,
  );
};
