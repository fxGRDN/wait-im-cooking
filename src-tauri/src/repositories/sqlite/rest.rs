use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;
use crate::repositories::sqlite::recipe::SqliteRecipeRepository;
use crate::repositories::traits::RecipeRepository;
use crate::repositories::traits::{
    PantryRepository, RecipeHistoryRepository, RepoResult, TagRepository,
};

// ─────────────────────────────────────────
// Tags
// ─────────────────────────────────────────

pub struct SqliteTagRepository {
    pool: SqlitePool,
}

impl SqliteTagRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TagRepository for SqliteTagRepository {
    async fn find_all(&self) -> RepoResult<Vec<Tag>> {
        sqlx::query_as!(
            Tag,
            r#"SELECT id as "id!", name FROM tags ORDER BY name ASC"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn find_by_id(&self, id: &str) -> RepoResult<Option<Tag>> {
        sqlx::query_as!(
            Tag,
            r#"SELECT id as "id!", name FROM tags WHERE id = ?"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn create(&self, name: &str) -> RepoResult<Tag> {
        let id = Uuid::new_v4().to_string();
        sqlx::query!("INSERT INTO tags (id, name) VALUES (?, ?)", id, name)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(Tag {
            id,
            name: name.to_string(),
        })
    }

    async fn update(&self, id: &str, name: &str) -> RepoResult<()> {
        sqlx::query!("UPDATE tags SET name = ? WHERE id = ?", name, id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn delete(&self, id: &str) -> RepoResult<()> {
        // recipe_tags cascade automatically
        sqlx::query!("DELETE FROM tags WHERE id = ?", id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

// ─────────────────────────────────────────
// Cook log
// ─────────────────────────────────────────

pub struct SqliteRecipeHistoryRepository {
    pool: SqlitePool,
}

impl SqliteRecipeHistoryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RecipeHistoryRepository for SqliteRecipeHistoryRepository {
    async fn find_all(&self, recipe_id: Option<&str>) -> RepoResult<Vec<RecipeHistory>> {
        let sql = if recipe_id.is_some() {
            r#"SELECT id, recipe_id, servings_made, duration_min, rating, notes, created_at FROM recipe_history WHERE recipe_id = ? ORDER BY created_at DESC"#
        } else {
            r#"SELECT id, recipe_id, servings_made, duration_min, rating, notes, created_at FROM recipe_history ORDER BY created_at DESC"#
        };

        let mut query = sqlx::query_as::<_, RecipeHistory>(sql);
        if let Some(id) = recipe_id {
            query = query.bind(id);
        }

        query.fetch_all(&self.pool).await.map_err(|e| e.to_string())
    }

    async fn find_by_id(&self, id: &str) -> RepoResult<Option<RecipeHistoryWithImages>> {
        let history = sqlx::query_as!(
            RecipeHistory,
            r#"SELECT id as "id!", recipe_id, servings_made, duration_min, rating, notes, created_at FROM recipe_history WHERE id = ?"#,
            id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let Some(history) = history else {
            return Ok(None);
        };

        let images = sqlx::query_as!(
            RecipeHistoryImage,
            r#"SELECT id as "id!", history_id, file_path, created_at FROM recipe_history_images WHERE history_id = ? ORDER BY created_at ASC"#,
            id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Some(RecipeHistoryWithImages { history, images }))
    }

    async fn create(&self, input: CreateHistoryInput) -> RepoResult<String> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        sqlx::query!(
            "INSERT INTO recipe_history (id, recipe_id, servings_made, duration_min, rating, notes, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
            id, input.recipe_id, input.servings_made, input.duration_min,
            input.rating, input.notes, now
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        for path in &input.image_paths {
            let img_id = Uuid::new_v4().to_string();
            sqlx::query!(
                "INSERT INTO recipe_history_images (id, history_id, file_path, created_at) VALUES (?, ?, ?, ?)",
                img_id, id, path, now
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        if input.consume_from_pantry {
            // Note: Pantry deduction is handled at the command level (commands.rs)
            // to keep repositories focused on their primary data types.
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(id)
    }

    async fn update(&self, id: &str, input: UpdateHistoryInput) -> RepoResult<Vec<String>> {
        let now = chrono::Utc::now().to_rfc3339();
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        sqlx::query!(
            "UPDATE recipe_history SET servings_made = ?, duration_min = ?, rating = ?, notes = ? WHERE id = ?",
            input.servings_made, input.duration_min, input.rating, input.notes, id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let mut deleted_paths = Vec::new();

        for img_id in &input.remove_image_ids {
            let row = sqlx::query!(
                "SELECT file_path FROM recipe_history_images WHERE id = ?",
                img_id
            )
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

            if let Some(r) = row {
                deleted_paths.push(r.file_path);
                sqlx::query!("DELETE FROM recipe_history_images WHERE id = ?", img_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        for path in &input.add_image_paths {
            let img_id = Uuid::new_v4().to_string();
            sqlx::query!(
                "INSERT INTO recipe_history_images (id, history_id, file_path, created_at) VALUES (?, ?, ?, ?)",
                img_id, id, path, now
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(deleted_paths)
    }

    // returns file paths — caller removes files from disk
    async fn delete(&self, id: &str) -> RepoResult<Vec<String>> {
        let paths: Vec<String> = sqlx::query!(
            "SELECT file_path FROM recipe_history_images WHERE history_id = ?",
            id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|r| r.file_path)
        .collect();

        // CASCADE removes recipe_history_images rows
        sqlx::query!("DELETE FROM recipe_history WHERE id = ?", id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(paths)
    }
}

// ─────────────────────────────────────────
// Pantry
// ─────────────────────────────────────────

struct UnitConverter;

impl UnitConverter {
    fn get_family(unit: &str) -> Option<&'static str> {
        match unit {
            "g" | "kg" => Some("weight"),
            "ml" | "l" => Some("volume"),
            _ => None,
        }
    }

    fn normalize(quantity: f64, unit: &str) -> Option<f64> {
        match unit {
            "g" | "ml" => Some(quantity),
            "kg" | "l" => Some(quantity * 1000.0),
            _ => None,
        }
    }

    fn convert(quantity: f64, from_unit: &str, to_unit: &str) -> Option<f64> {
        if from_unit == to_unit {
            return Some(quantity);
        }

        let from_family = Self::get_family(from_unit)?;
        let to_family = Self::get_family(to_unit)?;

        if from_family != to_family {
            return None;
        }

        let q_base = Self::normalize(quantity, from_unit)?;
        match to_unit {
            "g" | "ml" => Some(q_base),
            "kg" | "l" => Some(q_base / 1000.0),
            _ => None,
        }
    }
}

pub struct SqlitePantryRepository {
    pool: SqlitePool,
    recipes: SqliteRecipeRepository,
}

impl SqlitePantryRepository {
    pub fn new(pool: SqlitePool, recipes: SqliteRecipeRepository) -> Self {
        Self { pool, recipes }
    }

    async fn check_tree(&self, recipe_id: &str) -> RepoResult<AvailabilityResult> {
        let tree = self
            .recipes
            .find_with_tree(recipe_id)
            .await?
            .ok_or_else(|| format!("Recipe {} not found", recipe_id))?;

        let mut missing = Vec::new();
        for ri in &tree.ingredients {
            if ri.is_optional {
                continue;
            }
            let row = sqlx::query!(
                "SELECT quantity, unit FROM ingredient_inventory WHERE ingredient_id = ?",
                ri.ingredient_id
            )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let (available_in_recipe_unit, sufficient) = match row {
                Some(r) => {
                    let conv = UnitConverter::convert(r.quantity, &r.unit, &ri.unit);
                    match conv {
                        Some(q) => (q, q >= ri.quantity),
                        None => (0.0, false), // Incompatible units
                    }
                }
                None => (0.0, false),
            };

            if !sufficient {
                missing.push(IngredientAvailability {
                    ingredient: ri.ingredient.clone(),
                    required: ri.quantity,
                    unit: ri.unit.clone(),
                    available: available_in_recipe_unit,
                    sufficient,
                });
            }
        }

        // recurse by fetching each component fresh
        let mut component_results = Vec::new();
        for comp in &tree.components {
            component_results.push(Box::pin(self.check_tree(&comp.child_id)).await?);
        }

        Ok(AvailabilityResult {
            recipe_id: tree.id.clone(),
            cookable: missing.is_empty() && component_results.iter().all(|r| r.cookable),
            missing,
            components: component_results,
        })
    }

    async fn deduct_tree(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        recipe_id: &str,
        ratio: f64,
    ) -> RepoResult<()> {
        let tree = self
            .recipes
            .find_with_tree(recipe_id)
            .await?
            .ok_or_else(|| format!("Recipe {} not found", recipe_id))?;

        for ri in &tree.ingredients {
            if ri.is_optional {
                continue;
            }
            let deduct = ri.quantity * ratio;

            let row = sqlx::query!(
                "SELECT quantity, unit FROM ingredient_inventory WHERE ingredient_id = ?",
                ri.ingredient_id
            )
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;

            if let Some(r) = row {
                let deduct_in_inv_unit =
                    UnitConverter::convert(deduct, &ri.unit, &r.unit).unwrap_or(deduct);

                sqlx::query!(
                    "UPDATE ingredient_inventory SET quantity = MAX(0, quantity - ?) WHERE ingredient_id = ?",
                    deduct_in_inv_unit, ri.ingredient_id
                )
                .execute(&mut **tx)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        for comp in &tree.components {
            let child_ratio = (comp.servings_needed / tree.servings.unwrap_or(1) as f64) * ratio;
            Box::pin(self.deduct_tree(tx, &comp.child_id, child_ratio)).await?;
        }

        Ok(())
    }
}

#[async_trait]
impl PantryRepository for SqlitePantryRepository {
    async fn check_availability(&self, recipe_id: &str) -> RepoResult<AvailabilityResult> {
        self.check_tree(recipe_id).await
    }

    async fn consume_ingredients(&self, recipe_id: &str, servings_cooked: f64) -> RepoResult<()> {
        let tree = self
            .recipes
            .find_with_tree(recipe_id)
            .await?
            .ok_or_else(|| format!("Recipe {} not found", recipe_id))?;
        let ratio = servings_cooked / tree.servings.unwrap_or(1) as f64;
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        self.deduct_tree(&mut tx, recipe_id, ratio).await?;
        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_converter_normalize() {
        assert_eq!(UnitConverter::normalize(100.0, "g"), Some(100.0));
        assert_eq!(UnitConverter::normalize(1.0, "kg"), Some(1000.0));
        assert_eq!(UnitConverter::normalize(100.0, "ml"), Some(100.0));
        assert_eq!(UnitConverter::normalize(1.0, "l"), Some(1000.0));
        assert_eq!(UnitConverter::normalize(1.0, "pcs"), None);
    }

    #[test]
    fn test_unit_converter_convert() {
        // Same units
        assert_eq!(UnitConverter::convert(100.0, "g", "g"), Some(100.0));
        assert_eq!(UnitConverter::convert(1.0, "pcs", "pcs"), Some(1.0));

        // Weight to Weight
        assert_eq!(UnitConverter::convert(1000.0, "g", "kg"), Some(1.0));
        assert_eq!(UnitConverter::convert(1.5, "kg", "g"), Some(1500.0));

        // Volume to Volume
        assert_eq!(UnitConverter::convert(500.0, "ml", "l"), Some(0.5));
        assert_eq!(UnitConverter::convert(2.0, "l", "ml"), Some(2000.0));

        // Weight to Volume (RESTRICTED)
        assert_eq!(UnitConverter::convert(100.0, "g", "ml"), None);
        assert_eq!(UnitConverter::convert(100.0, "ml", "g"), None);

        // Incompatible families
        assert_eq!(UnitConverter::convert(1.0, "pcs", "g"), None);
        assert_eq!(UnitConverter::convert(100.0, "g", "pcs"), None);
    }
}
