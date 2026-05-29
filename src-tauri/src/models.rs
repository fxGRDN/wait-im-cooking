use serde::{Deserialize, Serialize};

pub type Uuid = String;

// ─────────────────────────────────────────
// Enums
// ─────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StepType {
    Prep,
    Cook,
}

// ─────────────────────────────────────────
// DB row types
// ─────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub id: Uuid,
    pub name: String,
    pub default_unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngredientInventory {
    pub id: Uuid,
    pub ingredient_id: Uuid,
    pub quantity: f64,
    pub unit: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngredientWithInventory {
    pub id: Uuid,
    pub name: String,
    pub default_unit: Option<String>,
    pub inventory: Option<IngredientInventory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub servings: Option<i64>,
    pub prep_time: Option<i64>,
    pub cook_time: Option<i64>,
    pub is_favourite: bool,
    pub cover_image: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeIngredient {
    pub recipe_id: Uuid,
    pub ingredient_id: Uuid,
    pub quantity: f64,
    pub unit: String,
    pub is_optional: bool,
    pub ingredient: Ingredient,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeComponent {
    pub parent_id: Uuid,
    pub child_id: Uuid,
    pub servings_needed: f64,
    pub child: Box<RecipeWithTree>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub step_order: i64,
    pub step_type: StepType,
    pub description: String,
    pub duration_min: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeWithTree {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub servings: Option<i64>,
    pub prep_time: Option<i64>,
    pub cook_time: Option<i64>,
    pub is_favourite: bool,
    pub cover_image: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub ingredients: Vec<RecipeIngredient>,
    pub components: Vec<RecipeComponent>,
    pub steps: Vec<Step>,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeHistory {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub servings_made: Option<i64>,
    pub duration_min: Option<i64>,
    pub rating: Option<i64>,
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeHistoryImage {
    pub id: Uuid,
    pub history_id: Uuid,
    pub file_path: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeHistoryWithImages {
    #[serde(flatten)]
    pub history: RecipeHistory,
    pub images: Vec<RecipeHistoryImage>,
}

// ─────────────────────────────────────────
// Input types
// ─────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreateIngredientInput {
    pub name: String,
    pub default_unit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateIngredientInput {
    pub name: Option<String>,
    pub default_unit: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpsertInventoryInput {
    pub ingredient_id: Uuid,
    pub quantity: f64,
    pub unit: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RecipeIngredientInput {
    pub ingredient_id: Uuid,
    pub quantity: f64,
    pub unit: String,
    pub is_optional: bool,
}

#[derive(Debug, Deserialize)]
pub struct RecipeComponentInput {
    pub child_id: Uuid,
    pub servings_needed: f64,
}

#[derive(Debug, Deserialize)]
pub struct StepInput {
    pub step_order: i64,
    pub step_type: StepType,
    pub description: String,
    pub duration_min: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRecipeInput {
    pub title: String,
    pub description: Option<String>,
    pub servings: Option<i64>,
    pub prep_time: Option<i64>,
    pub cook_time: Option<i64>,
    pub is_favourite: bool,
    pub cover_image: Option<String>,
    pub ingredients: Vec<RecipeIngredientInput>,
    pub components: Vec<RecipeComponentInput>,
    pub steps: Vec<StepInput>,
    pub tag_ids: Vec<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRecipeInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub servings: Option<i64>,
    pub prep_time: Option<i64>,
    pub cook_time: Option<i64>,
    pub is_favourite: Option<bool>,
    pub cover_image: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateHistoryInput {
    pub recipe_id: Uuid,
    pub servings_made: Option<i64>,
    pub duration_min: Option<i64>,
    pub rating: Option<i64>,
    pub notes: Option<String>,
    pub image_paths: Vec<String>,
    pub consume_from_pantry: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateHistoryInput {
    pub servings_made: Option<i64>,
    pub duration_min: Option<i64>,
    pub rating: Option<i64>,
    pub notes: Option<String>,
    pub add_image_paths: Vec<String>,
    pub remove_image_ids: Vec<String>,
}

// ─────────────────────────────────────────
// Availability
// ─────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct IngredientAvailability {
    pub ingredient: Ingredient,
    pub required: f64,
    pub unit: String,
    pub available: f64,
    pub sufficient: bool,
}

#[derive(Debug, Serialize)]
pub struct AvailabilityResult {
    pub recipe_id: Uuid,
    pub cookable: bool,
    pub missing: Vec<IngredientAvailability>,
    pub components: Vec<AvailabilityResult>,
}
