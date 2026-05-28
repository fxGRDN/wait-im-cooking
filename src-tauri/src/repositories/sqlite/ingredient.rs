use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;
use crate::repositories::traits::{IngredientRepository, RepoResult};

pub struct SqliteIngredientRepository {
    pool: SqlitePool,
}

impl SqliteIngredientRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IngredientRepository for SqliteIngredientRepository {
    async fn find_all(&self) -> RepoResult<Vec<Ingredient>> {
        sqlx::query_as!(
            Ingredient,
            r#"SELECT id as "id!", name, default_unit FROM ingredients ORDER BY name ASC"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn find_inventory(&self) -> RepoResult<Vec<IngredientWithInventory>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                i.id as "id!",
                i.name,
                i.default_unit,
                ii.id         AS inv_id,
                ii.quantity   AS inv_quantity,
                ii.unit       AS inv_unit,
                ii.expires_at AS inv_expires_at
            FROM ingredients i
            INNER JOIN ingredient_inventory ii ON ii.ingredient_id = i.id
            ORDER BY i.name ASC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let id = r.id;
                IngredientWithInventory {
                    id: id.clone(),
                    name: r.name,
                    default_unit: r.default_unit,
                    inventory: Some(IngredientInventory {
                        id: r.inv_id.expect("IngridientInventory id is missing!"),
                        ingredient_id: id,
                        quantity: r.inv_quantity,
                        unit: r.inv_unit,
                        expires_at: r.inv_expires_at,
                    }),
                }
            })
            .collect())
    }

    async fn find_by_id(&self, id: &str) -> RepoResult<Option<Ingredient>> {
        sqlx::query_as!(
            Ingredient,
            r#"SELECT id as "id!", name, default_unit FROM ingredients WHERE id = ?"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn find_with_inventory(&self, id: &str) -> RepoResult<Option<IngredientWithInventory>> {
        let row = sqlx::query!(
            r#"
            SELECT
                i.id as "id!",
                i.name,
                i.default_unit,
                ii.id         AS inv_id,
                ii.quantity   AS inv_quantity,
                ii.unit       AS inv_unit,
                ii.expires_at AS inv_expires_at
            FROM ingredients i
            LEFT JOIN ingredient_inventory ii ON ii.ingredient_id = i.id
            WHERE i.id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.map(|r| {
            let id = r.id;
            IngredientWithInventory {
                id: id.clone(),
                name: r.name,
                default_unit: r.default_unit,
                inventory: r.inv_id.map(|inv_id| IngredientInventory {
                    id: inv_id,
                    ingredient_id: id.clone(),
                    quantity: r.inv_quantity,
                    unit: r.inv_unit,
                    expires_at: r.inv_expires_at,
                }),
            }
        }))
    }

    async fn search(&self, query: &str) -> RepoResult<Vec<Ingredient>> {
        let pattern = format!("%{}%", query);
        sqlx::query_as!(
            Ingredient,
            r#"SELECT id as "id!", name, default_unit FROM ingredients WHERE name LIKE ? ORDER BY name ASC"#,
            pattern
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn create(&self, input: CreateIngredientInput) -> RepoResult<Ingredient> {
        let id = Uuid::new_v4().to_string();
        sqlx::query!(
            "INSERT INTO ingredients (id, name, default_unit) VALUES (?, ?, ?)",
            id,
            input.name,
            input.default_unit
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Ingredient {
            id,
            name: input.name,
            default_unit: input.default_unit,
        })
    }

    async fn update(&self, id: &str, input: UpdateIngredientInput) -> RepoResult<()> {
        // only update fields that were provided
        if let Some(name) = &input.name {
            sqlx::query!("UPDATE ingredients SET name = ? WHERE id = ?", name, id)
                .execute(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        }
        if let Some(unit) = &input.default_unit {
            sqlx::query!(
                "UPDATE ingredients SET default_unit = ? WHERE id = ?",
                unit,
                id
            )
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    async fn delete(&self, id: &str) -> RepoResult<()> {
        sqlx::query!("DELETE FROM ingredients WHERE id = ?", id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn upsert_inventory(&self, input: UpsertInventoryInput) -> RepoResult<()> {
        let existing = sqlx::query!(
            "SELECT id FROM ingredient_inventory WHERE ingredient_id = ?",
            input.ingredient_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if existing.is_some() {
            sqlx::query!(
                "UPDATE ingredient_inventory SET quantity = ?, unit = ?, expires_at = ? WHERE ingredient_id = ?",
                input.quantity, input.unit, input.expires_at, input.ingredient_id
            )
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        } else {
            let id = Uuid::new_v4().to_string();
            sqlx::query!(
                "INSERT INTO ingredient_inventory (id, ingredient_id, quantity, unit, expires_at) VALUES (?, ?, ?, ?, ?)",
                id, input.ingredient_id, input.quantity, input.unit, input.expires_at
            )
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    async fn delete_inventory(&self, ingredient_id: &str) -> RepoResult<()> {
        sqlx::query!(
            "DELETE FROM ingredient_inventory WHERE ingredient_id = ?",
            ingredient_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}
