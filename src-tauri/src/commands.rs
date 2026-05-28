use crate::models::*;
use crate::AppState;
use tauri::State;

// ─────────────────────────────────────────
// Ingredients
// ─────────────────────────────────────────

#[tauri::command]
pub async fn get_ingredients(state: State<'_, AppState>) -> Result<Vec<Ingredient>, String> {
    state.ingredients.find_all().await
}

#[tauri::command]
pub async fn get_inventory(
    state: State<'_, AppState>,
) -> Result<Vec<IngredientWithInventory>, String> {
    state.ingredients.find_inventory().await
}

#[tauri::command]
pub async fn get_ingredient(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<IngredientWithInventory>, String> {
    state.ingredients.find_with_inventory(&id).await
}

#[tauri::command]
pub async fn search_ingredients(
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<Ingredient>, String> {
    state.ingredients.search(&query).await
}

#[tauri::command]
pub async fn create_ingredient(
    input: CreateIngredientInput,
    state: State<'_, AppState>,
) -> Result<Ingredient, String> {
    state.ingredients.create(input).await
}

#[tauri::command]
pub async fn update_ingredient(
    id: String,
    input: UpdateIngredientInput,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.ingredients.update(&id, input).await
}

#[tauri::command]
pub async fn delete_ingredient(id: String, state: State<'_, AppState>) -> Result<(), String> {
    state.ingredients.delete(&id).await
}

#[tauri::command]
pub async fn upsert_inventory(
    input: UpsertInventoryInput,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.ingredients.upsert_inventory(input).await
}

#[tauri::command]
pub async fn delete_inventory(
    ingredient_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.ingredients.delete_inventory(&ingredient_id).await
}

// ─────────────────────────────────────────
// Recipes
// ─────────────────────────────────────────

#[tauri::command]
pub async fn get_recipes(state: State<'_, AppState>) -> Result<Vec<Recipe>, String> {
    state.recipes.find_all().await
}

#[tauri::command]
pub async fn get_recipe(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<RecipeWithTree>, String> {
    state.recipes.find_with_tree(&id).await
}

#[tauri::command]
pub async fn create_recipe(
    input: CreateRecipeInput,
    state: State<'_, AppState>,
) -> Result<String, String> {
    state.recipes.create(input).await
}

#[tauri::command]
pub async fn update_recipe(
    id: String,
    input: UpdateRecipeInput,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.recipes.update(&id, input).await
}

#[tauri::command]
pub async fn delete_recipe(id: String, state: State<'_, AppState>) -> Result<(), String> {
    state.recipes.delete(&id).await
}

#[tauri::command]
pub async fn toggle_favourite(id: String, state: State<'_, AppState>) -> Result<(), String> {
    state.recipes.toggle_favourite(&id).await
}

#[tauri::command]
pub async fn check_cycle(
    parent_id: String,
    child_id: String,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    state
        .recipes
        .would_create_cycle(&parent_id, &child_id)
        .await
}

// ─────────────────────────────────────────
// Tags
// ─────────────────────────────────────────

#[tauri::command]
pub async fn get_tags(state: State<'_, AppState>) -> Result<Vec<Tag>, String> {
    state.tags.find_all().await
}

#[tauri::command]
pub async fn create_tag(name: String, state: State<'_, AppState>) -> Result<Tag, String> {
    state.tags.create(&name).await
}

#[tauri::command]
pub async fn delete_tag(id: String, state: State<'_, AppState>) -> Result<(), String> {
    state.tags.delete(&id).await
}

// ─────────────────────────────────────────
// Cook log
// ─────────────────────────────────────────

#[tauri::command]
pub async fn get_cook_logs(
    recipe_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<RecipeHistory>, String> {
    state.history.find_all(recipe_id.as_deref()).await
}

#[tauri::command]
pub async fn get_cook_log(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<RecipeHistoryWithImages>, String> {
    state.history.find_by_id(&id).await
}

#[tauri::command]
pub async fn create_cook_log(
    input: CreateHistoryInput,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let consume = input.consume_from_pantry;
    let recipe_id = input.recipe_id.clone();
    let servings = input.servings_made;

    let id = state.history.create(input).await?;

    if consume {
        if let Some(s) = servings {
            state
                .pantry
                .consume_ingredients(&recipe_id, s as f64)
                .await?;
        }
    }

    Ok(id)
}

#[tauri::command]
pub async fn delete_cook_log(
    id: String,
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    // returns paths — frontend calls fs.remove on each
    state.history.delete(&id).await
}

// ─────────────────────────────────────────
// Pantry
// ─────────────────────────────────────────

#[tauri::command]
pub async fn check_availability(
    recipe_id: String,
    state: State<'_, AppState>,
) -> Result<AvailabilityResult, String> {
    state.pantry.check_availability(&recipe_id).await
}
