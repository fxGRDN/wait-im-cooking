use crate::models::*;
use async_trait::async_trait;

pub type RepoResult<T> = Result<T, String>;

// ─────────────────────────────────────────
// Ingredients
// ─────────────────────────────────────────

#[async_trait]
pub trait IngredientRepository: Send + Sync {
    async fn find_all(&self) -> RepoResult<Vec<Ingredient>>;
    async fn find_inventory(&self) -> RepoResult<Vec<IngredientWithInventory>>;
    async fn find_by_id(&self, id: &str) -> RepoResult<Option<Ingredient>>;
    async fn find_with_inventory(&self, id: &str) -> RepoResult<Option<IngredientWithInventory>>;
    async fn search(&self, query: &str) -> RepoResult<Vec<Ingredient>>;

    async fn create(&self, input: CreateIngredientInput) -> RepoResult<Ingredient>;
    async fn update(&self, id: &str, input: UpdateIngredientInput) -> RepoResult<()>;
    async fn delete(&self, id: &str) -> RepoResult<()>;

    async fn upsert_inventory(&self, input: UpsertInventoryInput) -> RepoResult<()>;
    async fn delete_inventory(&self, ingredient_id: &str) -> RepoResult<()>;
}

// ─────────────────────────────────────────
// Recipes
// ─────────────────────────────────────────

#[async_trait]
pub trait RecipeRepository: Send + Sync {
    async fn find_all(&self) -> RepoResult<Vec<Recipe>>;
    async fn find_by_id(&self, id: &str) -> RepoResult<Option<Recipe>>;
    async fn find_with_tree(&self, id: &str) -> RepoResult<Option<RecipeWithTree>>;
    async fn search(&self, query: &str) -> RepoResult<Vec<Recipe>>;

    async fn create(&self, input: CreateRecipeInput) -> RepoResult<String>;
    async fn update(&self, id: &str, input: UpdateRecipeInput) -> RepoResult<()>;
    async fn delete(&self, id: &str) -> RepoResult<()>;
    async fn toggle_favourite(&self, id: &str) -> RepoResult<()>;

    async fn would_create_cycle(&self, parent_id: &str, child_id: &str) -> RepoResult<bool>;
}

// ─────────────────────────────────────────
// Tags
// ─────────────────────────────────────────

#[async_trait]
pub trait TagRepository: Send + Sync {
    async fn find_all(&self) -> RepoResult<Vec<Tag>>;
    async fn find_by_id(&self, id: &str) -> RepoResult<Option<Tag>>;
    async fn create(&self, name: &str) -> RepoResult<Tag>;
    async fn delete(&self, id: &str) -> RepoResult<()>;
}

// ─────────────────────────────────────────
// Cook log
// ─────────────────────────────────────────

#[async_trait]
pub trait RecipeHistoryRepository: Send + Sync {
    async fn find_all(&self, recipe_id: Option<&str>) -> RepoResult<Vec<RecipeHistory>>;
    async fn find_by_id(&self, id: &str) -> RepoResult<Option<RecipeHistoryWithImages>>;
    async fn create(&self, input: CreateHistoryInput) -> RepoResult<String>;
    async fn update(&self, id: &str, input: UpdateHistoryInput) -> RepoResult<()>;
    async fn delete(&self, id: &str) -> RepoResult<Vec<String>>; // returns file paths to clean up
}

// ─────────────────────────────────────────
// Pantry
// ─────────────────────────────────────────

#[async_trait]
pub trait PantryRepository: Send + Sync {
    async fn check_availability(&self, recipe_id: &str) -> RepoResult<AvailabilityResult>;
    async fn consume_ingredients(&self, recipe_id: &str, servings_cooked: f64) -> RepoResult<()>;
}
