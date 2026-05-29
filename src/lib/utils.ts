import { readFile } from "@tauri-apps/plugin-fs";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

export async function pickAndSaveImage(
  folder: string = "recipe_covers",
): Promise<string | undefined> {
  const picked = await open({
    multiple: false,
    filters: [{ name: "Image", extensions: ["png", "jpg", "jpeg", "webp"] }],
  });
  if (!picked || Array.isArray(picked)) return;

  const bytes = await readFile(picked);
  const ext = picked.split(".").pop();
  const fileName = await invoke<string>("save_image", {
    bytes: Array.from(bytes),
    ext,
    folder,
  });
  return fileName;
}

export async function pickAndSaveImages(
  folder: string = "recipe_covers",
): Promise<string[]> {
  const picked = await open({
    multiple: true,
    filters: [{ name: "Images", extensions: ["png", "jpg", "jpeg", "webp"] }],
  });
  if (!picked) return [];

  const paths = Array.isArray(picked) ? picked : [picked];
  const results: string[] = [];

  for (const path of paths) {
    const bytes = await readFile(path);
    const ext = path.split(".").pop();
    const fileName = await invoke<string>("save_image", {
      bytes: Array.from(bytes),
      ext,
      folder,
    });
    results.push(fileName);
  }

  return results;
}
