use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;
use crate::repositories::traits::{RecipeRepository, RepoResult};

pub struct SqliteRecipeRepository {
    pool: SqlitePool,
}

impl SqliteRecipeRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RecipeRepository for SqliteRecipeRepository {
    async fn find_all(&self) -> RepoResult<Vec<Recipe>> {
        sqlx::query_as!(
            Recipe,
            "SELECT id as \"id!\", title, description, servings, prep_time, cook_time, is_favourite as \"is_favourite!\", cover_image, created_at, updated_at FROM recipes ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn find_by_id(&self, id: &str) -> RepoResult<Option<RecipeWithTree>> {
        let recipe = sqlx::query_as!(
            Recipe,
            "SELECT id as \"id!\", title, description, servings, prep_time, cook_time, is_favourite as \"is_favourite!\", cover_image, created_at, updated_at FROM recipes WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match recipe {
            Some(r) => Ok(Some(self.resolve_tree(r).await?)),
            None    => Ok(None),
        }
    }

    async fn resolve_tree(&self, recipe: Recipe) -> RepoResult<RecipeWithTree> {
        let ingredients = sqlx::query!(
            r#"
            SELECT ri.ingredient_id, ri.quantity, ri.unit, ri.is_optional,
                   i.name, i.default_unit, i.restock_threshold
            FROM recipe_ingredients ri
            JOIN ingredients i ON i.id = ri.ingredient_id
            WHERE ri.recipe_id = ?
            "#,
            recipe.id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|r| RecipeIngredient {
            recipe_id: recipe.id.clone(),
            ingredient_id: r.ingredient_id.clone(),
            quantity: r.quantity,
            unit: r.unit,
            is_optional: r.is_optional != 0,
            ingredient: Ingredient {
                id: r.ingredient_id,
                name: r.name,
                default_unit: r.default_unit,
                restock_threshold: r.restock_threshold,
            },
        })
        .collect();

        let component_rows = sqlx::query!(
            r#"
            SELECT rc.child_id, rc.servings_needed,
                   r.id as r_id, r.title, r.description, r.servings, r.prep_time, r.cook_time, r.is_favourite, r.cover_image, r.created_at, r.updated_at
            FROM recipe_components rc
            JOIN recipes r ON r.id = rc.child_id
            WHERE rc.parent_id = ?
            "#,
            recipe.id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut components = vec![];
        for r in component_rows {
            components.push(RecipeComponent {
                parent_id: recipe.id.clone(),
                child_id: r.child_id.clone(),
                servings_needed: r.servings_needed,
                child: Box::new(RecipeWithTree {
                    id: r.r_id.expect("Missing id"),
                    title: r.title,
                    description: r.description,
                    servings: r.servings,
                    prep_time: r.prep_time,
                    cook_time: r.cook_time,
                    is_favourite: r.is_favourite != 0,
                    cover_image: r.cover_image,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                    ingredients: vec![],
                    components: vec![],
                    steps: vec![],
                    tags: vec![],
                }),
            });
        }

        let steps = sqlx::query!(
            "SELECT id, recipe_id, step_order, step_type, description, duration_min FROM steps WHERE recipe_id = ? ORDER BY step_order ASC",
            recipe.id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|r| Step {
            id: r.id.expect("Missing step ID"),
            recipe_id: r.recipe_id,
            step_order: r.step_order,
            step_type: match r.step_type.as_str() {
                "cook" => StepType::Cook,
                _      => StepType::Prep,
            },
            description: r.description,
            duration_min: r.duration_min,
        })
        .collect();

        let tags = sqlx::query!(
            "SELECT t.id, t.name FROM tags t JOIN recipe_tags rt ON rt.tag_id = t.id WHERE rt.recipe_id = ?",
            recipe.id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|r| Tag { id: r.id.expect("Missing id"), name: r.name })
        .collect();

        Ok(RecipeWithTree {
            id: recipe.id,
            title: recipe.title,
            description: recipe.description,
            servings: recipe.servings,
            prep_time: recipe.prep_time,
            cook_time: recipe.cook_time,
            is_favourite: recipe.is_favourite,
            cover_image: recipe.cover_image,
            created_at: recipe.created_at,
            updated_at: recipe.updated_at,
            ingredients,
            components,
            steps,
            tags,
        })
    }

    async fn search(&self, query: &str) -> RepoResult<Vec<Recipe>> {
        let pattern = format!("%{}%", query);
        sqlx::query_as!(
            Recipe,
            "SELECT id as \"id!\", title, description, servings, prep_time, cook_time, is_favourite as \"is_favourite!\", cover_image, created_at, updated_at FROM recipes WHERE title LIKE ? OR description LIKE ? ORDER BY created_at DESC",
            pattern, pattern
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn create(&self, input: CreateRecipeInput) -> RepoResult<String> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        sqlx::query!(
            "INSERT INTO recipes (id, title, description, servings, prep_time, cook_time, is_favourite, cover_image, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            id, input.title, input.description, input.servings, input.prep_time, input.cook_time, input.is_favourite, input.cover_image, now, now
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        for ing in &input.ingredients {
            sqlx::query!(
                "INSERT INTO recipe_ingredients (recipe_id, ingredient_id, quantity, unit, is_optional) VALUES (?, ?, ?, ?, ?)",
                id, ing.ingredient_id, ing.quantity, ing.unit, ing.is_optional
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        for comp in &input.components {
            sqlx::query!(
                "INSERT INTO recipe_components (parent_id, child_id, servings_needed) VALUES (?, ?, ?)",
                id, comp.child_id, comp.servings_needed
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        for step in &input.steps {
            let step_id = Uuid::new_v4().to_string();
            let step_type = match step.step_type {
                StepType::Cook => "cook",
                StepType::Prep => "prep",
            };
            sqlx::query!(
                "INSERT INTO steps (id, recipe_id, step_order, step_type, description, duration_min) VALUES (?, ?, ?, ?, ?, ?)",
                step_id, id, step.step_order, step_type, step.description, step.duration_min
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        for tag_id in &input.tag_ids {
            sqlx::query!(
                "INSERT INTO recipe_tags (recipe_id, tag_id) VALUES (?, ?)",
                id, tag_id
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(id)
    }

    async fn update(&self, id: &str, input: UpdateRecipeInput) -> RepoResult<()> {
        println!("[RUST] Updating recipe id: {}", id);
        let now = chrono::Utc::now().to_rfc3339();
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        // 1. Update top-level fields
        let mut sets = vec!["updated_at = ?".to_string()];
        let mut params: Vec<String> = vec![now];

        if let Some(val) = &input.title {
            println!("[RUST] Setting title: {}", val);
            sets.push("title = ?".into());
            params.push(val.clone());
        }
        if let Some(val) = &input.description {
            println!("[RUST] Setting description: {:?}", val);
            sets.push("description = ?".into());
            params.push(val.clone());
        }
        if let Some(val) = &input.cover_image {
            println!("[RUST] Setting cover_image: {:?}", val);
            sets.push("cover_image = ?".into());
            params.push(val.clone());
        }
        if let Some(s) = input.servings {
            println!("[RUST] Setting servings: {}", s);
            sets.push("servings = ?".into());
            params.push(s.to_string());
        }
        if let Some(p) = input.prep_time {
            println!("[RUST] Setting prep_time: {}", p);
            sets.push("prep_time = ?".into());
            params.push(p.to_string());
        }
        if let Some(c) = input.cook_time {
            println!("[RUST] Setting cook_time: {}", c);
            sets.push("cook_time = ?".into());
            params.push(c.to_string());
        }
        if let Some(f) = input.is_favourite {
            println!("[RUST] Setting is_favourite: {}", f);
            sets.push("is_favourite = ?".into());
            params.push((f as i64).to_string());
        }

        if sets.len() > 1 {
            params.push(id.to_string());
            let query_str = format!("UPDATE recipes SET {} WHERE id = ?", sets.join(", "));
            println!("[RUST] Executing query: {}", query_str);
            println!("[RUST] With params: {:?}", params);
            let mut query = sqlx::query(&query_str);
            for param in params {
                query = query.bind(param);
            }
            let result = query.execute(&mut *tx).await.map_err(|e| {
                println!("[RUST] Error updating recipes table: {}", e);
                e.to_string()
            })?;
            println!("[RUST] Rows affected: {}", result.rows_affected());
        }

        // 2. Update Ingredients (sync)
        if let Some(ings) = &input.ingredients {
            println!("[RUST] Updating {} ingredients", ings.len());
            sqlx::query!("DELETE FROM recipe_ingredients WHERE recipe_id = ?", id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
            for ing in ings {
                sqlx::query!(
                    "INSERT INTO recipe_ingredients (recipe_id, ingredient_id, quantity, unit, is_optional) VALUES (?, ?, ?, ?, ?)",
                    id, ing.ingredient_id, ing.quantity, ing.unit, ing.is_optional
                )
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        // 3. Update Components (sync)
        if let Some(comps) = &input.components {
            println!("[RUST] Updating {} components", comps.len());
            sqlx::query!("DELETE FROM recipe_components WHERE parent_id = ?", id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
            for comp in comps {
                sqlx::query!(
                    "INSERT INTO recipe_components (parent_id, child_id, servings_needed) VALUES (?, ?, ?)",
                    id, comp.child_id, comp.servings_needed
                )
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        // 4. Update Steps (sync)
        if let Some(steps) = &input.steps {
            println!("[RUST] Updating {} steps", steps.len());
            sqlx::query!("DELETE FROM steps WHERE recipe_id = ?", id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
            for step in steps {
                let step_id = Uuid::new_v4().to_string();
                let step_type = match step.step_type {
                    StepType::Cook => "cook",
                    StepType::Prep => "prep",
                };
                sqlx::query!(
                    "INSERT INTO steps (id, recipe_id, step_order, step_type, description, duration_min) VALUES (?, ?, ?, ?, ?, ?)",
                    step_id, id, step.step_order, step_type, step.description, step.duration_min
                )
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        // 5. Update Tags (sync)
        if let Some(tag_ids) = &input.tag_ids {
            println!("[RUST] Updating {} tags", tag_ids.len());
            sqlx::query!("DELETE FROM recipe_tags WHERE recipe_id = ?", id)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
            for tag_id in tag_ids {
                sqlx::query!(
                    "INSERT INTO recipe_tags (recipe_id, tag_id) VALUES (?, ?)",
                    id, tag_id
                )
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        tx.commit().await.map_err(|e| {
            println!("[RUST] Error committing transaction: {}", e);
            e.to_string()
        })?;
        println!("[RUST] Recipe update committed successfully");
        Ok(())
    }

    async fn delete(&self, id: &str) -> RepoResult<()> {
        sqlx::query!("DELETE FROM recipes WHERE id = ?", id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn toggle_favourite(&self, id: &str) -> RepoResult<()> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query!(
            "UPDATE recipes SET is_favourite = CASE WHEN is_favourite = 1 THEN 0 ELSE 1 END, updated_at = ? WHERE id = ?",
            now, id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn would_create_cycle(&self, parent_id: &str, child_id: &str) -> RepoResult<bool> {
        let rows = sqlx::query!(
            r#"
            WITH RECURSIVE ancestors(id) AS (
                SELECT parent_id FROM recipe_components WHERE child_id = ?
                UNION ALL
                SELECT rc.parent_id FROM recipe_components rc
                JOIN ancestors a ON a.id = rc.child_id
            )
            SELECT id FROM ancestors
            "#,
            parent_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows
            .iter()
            .any(|r| r.id.as_deref().expect("Missing id") == child_id))
    }
}
