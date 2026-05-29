import {
  mkdir,
  writeFile,
  readFile,
  BaseDirectory,
} from "@tauri-apps/plugin-fs";
import { open } from "@tauri-apps/plugin-dialog";

export const saveImages = async (
  folder: string,
  multiple: boolean = false,
): Promise<string[]> => {
  await mkdir(folder, { baseDir: BaseDirectory.AppLocalData, recursive: true });
  const picked = await open({
    multiple,
    filters: [{ name: "Image", extensions: ["png", "jpg", "jpeg", "webp"] }],
  });

  if (!picked) return [];

  const paths = Array.isArray(picked) ? picked : [picked];
  const results: string[] = [];

  for (const path of paths) {
    const bytes = await readFile(path);
    const filename = path.split("/").pop();
    await writeFile(`${folder}/${filename}`, bytes, {
      baseDir: BaseDirectory.AppLocalData,
    });
    results.push(filename);
  }

  return results;
};
