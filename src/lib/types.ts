// ─────────────────────────────────────────
// Primitives
// ─────────────────────────────────────────

export type UUID = string;
export type ISODate = string; // "2024-01-15"
export type ISOTimestamp = string; // "2024-01-15T14:32:00Z"
export type StepType = "prep" | "cook";

// ─────────────────────────────────────────
// DB row types  (mirror schema exactly)
// ─────────────────────────────────────────

export interface Ingredient {
  id: UUID;
  name: string;
  default_unit: string | null;
  restock_threshold: number | null;
}

export interface IngredientInventory {
  id: UUID;
  ingredient_id: UUID;
  quantity: number;
  unit: string;
  expires_at: ISODate | null;
}

export interface Recipe {
  id: UUID;
  title: string;
  description: string | null;
  servings: number | null;
  prep_time: number | null; // minutes
  cook_time: number | null; // minutes
  is_favourite: boolean;
  cover_image: string | null; // relative path
  created_at: ISOTimestamp;
  updated_at: ISOTimestamp;
  tags: Tag[];
}

export interface RecipeIngredient {
  recipe_id: UUID;
  ingredient_id: UUID;
  quantity: number;
  unit: string;
  is_optional: boolean;
}

export interface RecipeComponent {
  parent_id: UUID;
  child_id: UUID;
  servings_needed: number;
}

export interface Step {
  id: UUID;
  recipe_id: UUID;
  step_order: number;
  step_type: StepType;
  description: string;
  duration_min: number | null;
}

export interface Tag {
  id: UUID;
  name: string;
}

export interface RecipeTag {
  recipe_id: UUID;
  tag_id: UUID;
}

export interface RecipeHistory {
  id: UUID;
  recipe_id: UUID;
  servings_made: number | null;
  duration_min: number | null;
  rating: 1 | 2 | 3 | 4 | 5 | null;
  notes: string | null;
  created_at: ISOTimestamp;
}

export interface RecipeHistoryImage {
  id: UUID;
  history_id: UUID;
  file_path: string; // relative path
  created_at: ISOTimestamp;
}

// ─────────────────────────────────────────
// Composed types  (used in service layer)
// ─────────────────────────────────────────

export interface IngredientWithInventory extends Ingredient {
  inventory: IngredientInventory | null;
}

export interface RecipeIngredientDetail extends RecipeIngredient {
  ingredient: Ingredient;
}

export interface RecipeComponentDetail extends RecipeComponent {
  child: RecipeWithTree;
}

export interface RecipeWithTree extends Recipe {
  ingredients: RecipeIngredientDetail[];
  components: RecipeComponentDetail[];
  steps: Step[];
  tags: Tag[];
}
export interface RecipeHistoryIngredient {
  id: UUID;
  history_id: UUID;
  ingredient_id: UUID;
  name: string;
  quantity: number;
  unit: string;
  was_deducted: boolean;
}

export interface RecipeHistoryWithImages {
  history: RecipeHistory;
  images: RecipeHistoryImage[];
  ingredients: RecipeHistoryIngredient[];
}

// ─────────────────────────────────────────
// Input types  (for create / update)
// ─────────────────────────────────────────

export type RecipeInput = Omit<Recipe, "id" | "created_at" | "updated_at">;

export type StepInput = Omit<Step, "id" | "recipe_id">;

export type RecipeIngredientInput = Omit<RecipeIngredient, "recipe_id">;

export type RecipeComponentInput = Omit<RecipeComponent, "parent_id">;

export type RecipeHistoryInput = Omit<RecipeHistory, "id" | "created_at">;

// ─────────────────────────────────────────
// Share format
// ─────────────────────────────────────────

export interface SharePayload {
  v: number; // schema version, for forward compat
  recipe: RecipeWithTree;
}

export interface ImportPreview {
  recipe: RecipeWithTree;
  conflicts: {
    ingredients: Array<{ incoming: Ingredient; matched: Ingredient | null }>;
    recipes: Array<{ incoming: Recipe; exists: boolean }>;
  };
}

// ─────────────────────────────────────────
// Availability check
// ─────────────────────────────────────────

export interface IngredientAvailability {
  ingredient: Ingredient;
  required: number;
  unit: string;
  available: number;
  sufficient: boolean;
}

export interface AvailabilityResult {
  recipe_id: UUID;
  cookable: boolean;
  missing: IngredientAvailability[];
  components: AvailabilityResult[];
}
