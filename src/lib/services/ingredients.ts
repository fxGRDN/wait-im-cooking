import { invoke } from "@tauri-apps/api/core";
import type { Ingredient, IngredientWithInventory } from "$lib/types";

// ─────────────────────────────────────────
// Queries
// ─────────────────────────────────────────

export async function getIngredients(): Promise<Ingredient[]> {
  return invoke("get_ingredients");
}

export async function getIngredient(id: string): Promise<Ingredient | null> {
  const res = await invoke<IngredientWithInventory | null>("get_ingredient", {
    id,
  });
  return res;
}

export async function getIngredientWithInventory(
  id: string,
): Promise<IngredientWithInventory | null> {
  return invoke("get_ingredient", { id });
}

export async function getInventory(): Promise<IngredientWithInventory[]> {
  return invoke("get_inventory");
}

export async function searchIngredients(query: string): Promise<Ingredient[]> {
  return invoke("search_ingredients", { query });
}

// ─────────────────────────────────────────
// Mutations
// ─────────────────────────────────────────

export async function createIngredient(
  name: string,
  default_unit: string | null = null,
  restock_threshold: number | null = null,
): Promise<Ingredient> {
  return invoke("create_ingredient", {
    input: { name, default_unit, restock_threshold },
  });
}

export async function getOrCreateIngredient(
  name: string,
  unit: string | null = null,
  restock_threshold: number | null = null,
): Promise<Ingredient> {
  const existing = await searchIngredients(name);
  const exact = existing.find(
    (i) => i.name.toLowerCase() === name.toLowerCase(),
  );
  if (exact) return exact;
  return createIngredient(name, unit, restock_threshold);
}

export async function updateIngredient(
  id: string,
  data: Partial<
    Pick<Ingredient, "name" | "default_unit" | "restock_threshold">
  >,
): Promise<void> {
  return invoke("update_ingredient", { id, input: data });
}

export async function deleteIngredient(id: string): Promise<void> {
  return invoke("delete_ingredient", { id });
}

// ─────────────────────────────────────────
// Inventory
// ─────────────────────────────────────────

export async function upsertInventory(
  ingredient_id: string,
  quantity: number,
  unit: string,
  expires_at: string | null = null,
): Promise<void> {
  return invoke("upsert_inventory", {
    input: { ingredient_id, quantity, unit, expires_at },
  });
}

export async function deleteInventory(ingredient_id: string): Promise<void> {
  return invoke("delete_inventory", { ingredient_id });
}
