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

    async fn fetch_recipe_row(&self, id: &str) -> RepoResult<Option<Recipe>> {
        let row = sqlx::query!(
            "SELECT id, title, description, servings, prep_time, cook_time, is_favourite, cover_image, created_at, updated_at FROM recipes WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.map(|r| Recipe {
            id: r.id.expect("Missing recipe ID"),
            title: r.title,
            description: r.description,
            servings: r.servings,
            prep_time: r.prep_time,
            cook_time: r.cook_time,
            is_favourite: r.is_favourite != 0,
            cover_image: r.cover_image,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    async fn resolve_tree(&self, recipe: Recipe) -> RepoResult<RecipeWithTree> {
        let ingredients = sqlx::query!(
            r#"
            SELECT ri.ingredient_id, ri.quantity, ri.unit, ri.is_optional,
                   i.name, i.default_unit
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
            },
        })
        .collect();

        let component_rows = sqlx::query!(
            "SELECT child_id, servings_needed FROM recipe_components WHERE parent_id = ?",
            recipe.id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut components = Vec::new();
        for row in component_rows {
            let child = self
                .fetch_recipe_row(&row.child_id)
                .await?
                .ok_or_else(|| format!("Component recipe {} not found", row.child_id))?;
            components.push(RecipeComponent {
                parent_id: recipe.id.clone(),
                child_id: row.child_id.clone(),
                servings_needed: row.servings_needed,
                child: Box::new(RecipeWithTree {
                    id: child.id.clone(),
                    title: child.title,
                    description: child.description,
                    servings: child.servings,
                    prep_time: child.prep_time,
                    cook_time: child.cook_time,
                    is_favourite: child.is_favourite,
                    cover_image: child.cover_image,
                    created_at: child.created_at,
                    updated_at: child.updated_at,
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
}

#[async_trait]
impl RecipeRepository for SqliteRecipeRepository {
    async fn find_all(&self) -> RepoResult<Vec<Recipe>> {
        let rows = sqlx::query!(
            "SELECT id, title, description, servings, prep_time, cook_time, is_favourite, cover_image, created_at, updated_at FROM recipes ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows
            .into_iter()
            .map(|r| Recipe {
                id: r.id.expect("Missing recipe ID"),
                title: r.title,
                description: r.description,
                servings: r.servings,
                prep_time: r.prep_time,
                cook_time: r.cook_time,
                is_favourite: r.is_favourite != 0,
                cover_image: r.cover_image,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect())
    }

    async fn find_by_id(&self, id: &str) -> RepoResult<Option<Recipe>> {
        self.fetch_recipe_row(id).await
    }

    async fn find_with_tree(&self, id: &str) -> RepoResult<Option<RecipeWithTree>> {
        let recipe = match self.fetch_recipe_row(id).await? {
            Some(r) => r,
            None => return Ok(None),
        };
        let tree = self.resolve_tree(recipe).await?;
        Ok(Some(tree))
    }

    async fn search(&self, query: &str) -> RepoResult<Vec<Recipe>> {
        let pattern = format!("%{}%", query);
        let rows = sqlx::query!(
            "SELECT id, title, description, servings, prep_time, cook_time, is_favourite, cover_image, created_at, updated_at FROM recipes WHERE title LIKE ? OR description LIKE ? ORDER BY created_at DESC",
            pattern, pattern
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows
            .into_iter()
            .map(|r| Recipe {
                id: r.id.expect("Missing recipe ID"),
                title: r.title,
                description: r.description,
                servings: r.servings,
                prep_time: r.prep_time,
                cook_time: r.cook_time,
                is_favourite: r.is_favourite != 0,
                cover_image: r.cover_image,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect())
    }

    async fn create(&self, input: CreateRecipeInput) -> RepoResult<String> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        sqlx::query!(
            "INSERT INTO recipes (id, title, description, servings, prep_time, cook_time, is_favourite, cover_image, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            id, input.title, input.description, input.servings, input.prep_time,
            input.cook_time, input.is_favourite, input.cover_image, now, now
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
                StepType::Prep => "prep",
                StepType::Cook => "cook",
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
                id,
                tag_id
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(id)
    }

    async fn update(&self, id: &str, input: UpdateRecipeInput) -> RepoResult<()> {
        let now = chrono::Utc::now().to_rfc3339();

        // build query dynamically only for provided fields
        let mut sets = vec!["updated_at = ?".to_string()];
        let mut params: Vec<String> = vec![now];

        macro_rules! push_field {
            ($field:expr, $col:expr) => {
                if let Some(val) = $field {
                    sets.push(format!("{} = ?", $col));
                    params.push(val.to_string());
                }
            };
        }

        push_field!(input.title, "title");
        push_field!(input.description, "description");
        push_field!(input.cover_image, "cover_image");
        if let Some(s) = input.servings {
            sets.push("servings = ?".into());
            params.push(s.to_string());
        }
        if let Some(p) = input.prep_time {
            sets.push("prep_time = ?".into());
            params.push(p.to_string());
        }
        if let Some(c) = input.cook_time {
            sets.push("cook_time = ?".into());
            params.push(c.to_string());
        }
        if let Some(f) = input.is_favourite {
            sets.push("is_favourite = ?".into());
            params.push((f as i64).to_string());
        }

        params.push(id.to_string());
        let query_str = format!("UPDATE recipes SET {} WHERE id = ?", sets.join(", "));
        let mut query = sqlx::query(&query_str);
        for param in params {
            query = query.bind(param);
        }
        query.execute(&self.pool).await.map_err(|e| e.to_string())?;

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
