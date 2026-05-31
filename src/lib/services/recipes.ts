import { invoke } from "@tauri-apps/api/core";
import type {
  Recipe,
  RecipeWithTree,
  RecipeInput,
  RecipeIngredientInput,
  RecipeComponentInput,
  StepInput,
} from "$lib/types";

// ─────────────────────────────────────────
// Queries
// ─────────────────────────────────────────

export async function getRecipes(): Promise<Recipe[]> {
  return invoke("get_recipes");
}

export async function getRecipe(id: string): Promise<Recipe | null> {
  // get_recipe returns RecipeWithTree, but we can treat it as Recipe if needed
  return invoke("get_recipe", { id });
}

// resolves the full tree: ingredients, components (recursive), steps, tags
export async function getRecipeWithTree(
  id: string,
): Promise<RecipeWithTree | null> {
  return invoke("get_recipe", { id });
}

export async function searchRecipes(query: string): Promise<Recipe[]> {
  return invoke("search_recipes", { query });
}

// ─────────────────────────────────────────
// Mutations
// ─────────────────────────────────────────

export async function createRecipe(
  data: RecipeInput,
  ingredients: RecipeIngredientInput[] = [],
  components: RecipeComponentInput[] = [],
  steps: StepInput[] = [],
  tagIds: string[] = [],
): Promise<string> {
  return invoke("create_recipe", {
    input: {
      ...data,
      ingredients,
      components,
      steps,
      tag_ids: tagIds,
    },
  });
}

export async function updateRecipe(
  id: string,
  data: {
    title?: string;
    description?: string | null;
    servings?: number | null;
    prep_time?: number | null;
    cook_time?: number | null;
    is_favourite?: boolean;
    cover_image?: string | null;
    ingredients?: RecipeIngredientInput[];
    components?: RecipeComponentInput[];
    steps?: StepInput[];
    tag_ids?: string[];
  },
): Promise<void> {
  return invoke("update_recipe", { id, input: data });
}

export async function deleteRecipe(id: string): Promise<void> {
  return invoke("delete_recipe", { id });
}

export async function toggleFavourite(id: string): Promise<void> {
  return invoke("toggle_favourite", { id });
}

// ─────────────────────────────────────────
// Cycle detection
// ─────────────────────────────────────────

// call before inserting a recipe_component row
export async function wouldCreateCycle(
  parentId: string,
  childId: string,
): Promise<boolean> {
  return invoke("check_cycle", { parentId, childId });
}

// ─────────────────────────────────────────
// Tags
// ─────────────────────────────────────────

export async function getTags(): Promise<any[]> {
  return invoke("get_tags");
}

export async function createTag(name: string): Promise<any> {
  return invoke("create_tag", { name });
}

export async function updateTag(id: string, name: string): Promise<void> {
  return invoke("update_tag", { id, name });
}

export async function deleteTag(id: string): Promise<void> {
  return invoke("delete_tag", { id });
}
