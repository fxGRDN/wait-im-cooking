import { invoke } from "@tauri-apps/api/core";
import type {
    RecipeHistory,
    RecipeHistoryWithImages,
    RecipeHistoryInput,
    AvailabilityResult,
} from "$lib/types";

// ─────────────────────────────────────────
// Pantry / availability
// ─────────────────────────────────────────

// checks recursively whether pantry covers all ingredients in a recipe tree
export async function checkAvailability(
    recipe_id: string,
): Promise<AvailabilityResult> {
    return invoke("check_availability", { recipe_id });
}

// ─────────────────────────────────────────
// Cook log
// ─────────────────────────────────────────

export async function getCookLogs(
    recipe_id?: string,
): Promise<RecipeHistory[]> {
    return invoke("get_cook_logs", { recipe_id });
}

export async function getCookLog(
    id: string,
): Promise<RecipeHistoryWithImages | null> {
    return invoke("get_cook_log", { id });
}

export async function createCookLog(
    data: RecipeHistoryInput,
    imagePaths: string[] = [],
    consumeFromPantry = false,
): Promise<string> {
    return invoke("create_cook_log", {
        input: {
            ...data,
            image_paths: imagePaths,
            consume_from_pantry: consumeFromPantry,
        },
    });
}

// deletes log entry + cleans up image files from disk
export async function deleteCookLog(
    id: string,
    removeFiles: (paths: string[]) => Promise<void>,
): Promise<void> {
    const paths: string[] = await invoke("delete_cook_log", { id });
    await removeFiles(paths);
}
