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

#[derive(sqlx::FromRow)]
struct IngredientInventoryRow {
    id: String,
    name: String,
    default_unit: Option<String>,
    restock_threshold: Option<f64>,
    inv_id: Option<String>,
    inv_quantity: Option<f64>,
    inv_unit: Option<String>,
    inv_expires_at: Option<String>,
}

impl From<IngredientInventoryRow> for IngredientWithInventory {
    fn from(r: IngredientInventoryRow) -> Self {
        IngredientWithInventory {
            id: r.id.clone(),
            name: r.name,
            default_unit: r.default_unit,
            restock_threshold: r.restock_threshold,
            inventory: r.inv_id.map(|inv_id| IngredientInventory {
                id: inv_id,
                ingredient_id: r.id,
                quantity: r.inv_quantity.unwrap_or(0.0),
                unit: r.inv_unit.unwrap_or_default(),
                expires_at: r.inv_expires_at,
            }),
        }
    }
}

#[async_trait]
impl IngredientRepository for SqliteIngredientRepository {
    async fn find_all(&self) -> RepoResult<Vec<Ingredient>> {
        let rows = sqlx::query!(
            r#"SELECT id as "id!", name, default_unit, restock_threshold FROM ingredients ORDER BY name ASC"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows
            .into_iter()
            .map(|r| Ingredient {
                id: r.id,
                name: r.name,
                default_unit: r.default_unit,
                restock_threshold: r.restock_threshold,
            })
            .collect())
    }

    async fn find_inventory(&self) -> RepoResult<Vec<IngredientWithInventory>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                i.id as "id!",
                i.name,
                i.default_unit,
                i.restock_threshold,
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
            .map(|r| IngredientWithInventory {
                id: r.id.clone(),
                name: r.name,
                default_unit: r.default_unit,
                restock_threshold: r.restock_threshold,
                inventory: Some(IngredientInventory {
                    id: r.inv_id.expect("IngridientInventory id is missing!"),
                    ingredient_id: r.id,
                    quantity: r.inv_quantity,
                    unit: r.inv_unit,
                    expires_at: r.inv_expires_at,
                }),
            })
            .collect())
    }

    async fn find_by_id(&self, id: &str) -> RepoResult<Option<Ingredient>> {
        let row = sqlx::query!(
            r#"SELECT id as "id!", name, default_unit, restock_threshold FROM ingredients WHERE id = ?"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.map(|r| Ingredient {
            id: r.id,
            name: r.name,
            default_unit: r.default_unit,
            restock_threshold: r.restock_threshold,
        }))
    }

    async fn find_with_inventory(&self, id: &str) -> RepoResult<Option<IngredientWithInventory>> {
        let row = sqlx::query!(
            r#"
            SELECT
                i.id as "id!",
                i.name,
                i.default_unit,
                i.restock_threshold,
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

        Ok(row.map(|r| IngredientWithInventory {
            id: r.id.clone(),
            name: r.name,
            default_unit: r.default_unit,
            restock_threshold: r.restock_threshold,
            inventory: r.inv_id.map(|inv_id| IngredientInventory {
                id: inv_id,
                ingredient_id: r.id,
                quantity: r.inv_quantity,
                unit: r.inv_unit,
                expires_at: r.inv_expires_at,
            }),
        }))
    }

    async fn search(&self, query: &str) -> RepoResult<Vec<Ingredient>> {
        let pattern = format!("%{}%", query);
        let rows = sqlx::query!(
            r#"SELECT id as "id!", name, default_unit, restock_threshold FROM ingredients WHERE name LIKE ? ORDER BY name ASC"#,
            pattern
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows
            .into_iter()
            .map(|r| Ingredient {
                id: r.id,
                name: r.name,
                default_unit: r.default_unit,
                restock_threshold: r.restock_threshold,
            })
            .collect())
    }

    async fn create(&self, input: CreateIngredientInput) -> RepoResult<Ingredient> {
        let id = Uuid::new_v4().to_string();
        sqlx::query!(
            "INSERT INTO ingredients (id, name, default_unit, restock_threshold) VALUES (?, ?, ?, ?)",
            id,
            input.name,
            input.default_unit,
            input.restock_threshold
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Ingredient {
            id,
            name: input.name,
            default_unit: input.default_unit,
            restock_threshold: input.restock_threshold,
        })
    }

    async fn update(&self, id: &str, input: UpdateIngredientInput) -> RepoResult<()> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let mut sets = vec![];
        let mut params: Vec<String> = vec![];

        if let Some(name) = &input.name {
            sets.push("name = ?".to_string());
            params.push(name.clone());
        }
        if let Some(unit) = &input.default_unit {
            sets.push("default_unit = ?".to_string());
            params.push(unit.clone());

            // Propagate unit change to all recipes using this ingredient
            sqlx::query!(
                "UPDATE recipe_ingredients SET unit = ? WHERE ingredient_id = ?",
                unit,
                id
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

            // Also update inventory unit
            sqlx::query!(
                "UPDATE ingredient_inventory SET unit = ? WHERE ingredient_id = ?",
                unit,
                id
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }
        if let Some(threshold) = input.restock_threshold {
            sets.push("restock_threshold = ?".to_string());
            params.push(threshold.to_string());
        }

        if !sets.is_empty() {
            params.push(id.to_string());
            let query_str = format!("UPDATE ingredients SET {} WHERE id = ?", sets.join(", "));
            let mut query = sqlx::query(&query_str);
            for param in params {
                query = query.bind(param);
            }

            query.execute(&mut *tx).await.map_err(|e| e.to_string())?;
        }

        tx.commit().await.map_err(|e| e.to_string())?;
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
