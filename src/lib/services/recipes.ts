import { v4 as uuid } from 'uuid'
import { dbSelect, dbExecute, dbTransaction } from '$lib/db'
import type {
  Recipe,
  RecipeWithTree,
  RecipeInput,
  RecipeIngredientInput,
  RecipeComponentInput,
  StepInput,
  Step,
  Tag,
} from '$lib/types'

// ─────────────────────────────────────────
// Queries
// ─────────────────────────────────────────

export async function getRecipes(): Promise<Recipe[]> {
  return dbSelect<Recipe>(`
    SELECT * FROM recipes ORDER BY created_at DESC
  `)
}

export async function getRecipe(id: string): Promise<Recipe | null> {
  const rows = await dbSelect<Recipe>(
    'SELECT * FROM recipes WHERE id = ?',
    [id]
  )
  return rows[0] ?? null
}

// resolves the full tree: ingredients, components (recursive), steps, tags
export async function getRecipeWithTree(id: string): Promise<RecipeWithTree | null> {
  const recipe = await getRecipe(id)
  if (!recipe) return null
  return resolveTree(recipe)
}

async function resolveTree(
  recipe: Recipe,
  visited = new Set<string>()
): Promise<RecipeWithTree> {
  if (visited.has(recipe.id)) {
    throw new Error(`Cycle detected at recipe "${recipe.title}" (${recipe.id})`)
  }
  visited.add(recipe.id)

  const [ingredients, components, steps, tags] = await Promise.all([
    dbSelect<any>(`
      SELECT ri.*, i.name, i.default_unit
      FROM recipe_ingredients ri
      JOIN ingredients i ON i.id = ri.ingredient_id
      WHERE ri.recipe_id = ?
    `, [recipe.id]),

    dbSelect<{ child_id: string; servings_needed: number }>(`
      SELECT child_id, servings_needed
      FROM recipe_components
      WHERE parent_id = ?
    `, [recipe.id]),

    dbSelect<Step>(`
      SELECT * FROM steps
      WHERE recipe_id = ?
      ORDER BY step_order ASC
    `, [recipe.id]),

    dbSelect<Tag>(`
      SELECT t.*
      FROM tags t
      JOIN recipe_tags rt ON rt.tag_id = t.id
      WHERE rt.recipe_id = ?
    `, [recipe.id]),
  ])

  const resolvedComponents = await Promise.all(
    components.map(async ({ child_id, servings_needed }: { child_id: string; servings_needed: number }) => {
      const child = await getRecipe(child_id)
      if (!child) throw new Error(`Component recipe ${child_id} not found`)
      return {
        parent_id: recipe.id,
        child_id,
        servings_needed,
        child: await resolveTree(child, new Set(visited)),
      }
    })
  )

  return {
    ...recipe,
    is_favourite: Boolean(recipe.is_favourite),
    ingredients: ingredients.map((row: any) => ({
      recipe_id: row.recipe_id,
      ingredient_id: row.ingredient_id,
      quantity: row.quantity,
      unit: row.unit,
      is_optional: Boolean(row.is_optional),
      ingredient: {
        id: row.ingredient_id,
        name: row.name,
        default_unit: row.default_unit,
      },
    })),
    components: resolvedComponents,
    steps,
    tags,
  }
}

// ─────────────────────────────────────────
// Mutations
// ─────────────────────────────────────────

export async function createRecipe(
  data: RecipeInput,
  ingredients: RecipeIngredientInput[] = [],
  components: RecipeComponentInput[] = [],
  steps: StepInput[] = [],
  tagIds: string[] = []
): Promise<string> {
  const id = uuid()
  const now = new Date().toISOString()

  await dbTransaction(async (db) => {
    await db.execute(
      `INSERT INTO recipes
        (id, title, description, servings, prep_time, cook_time, is_favourite, cover_image, created_at, updated_at)
       VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
      [id, data.title, data.description, data.servings, data.prep_time,
       data.cook_time, data.is_favourite ? 1 : 0, data.cover_image, now, now]
    )

    for (const ing of ingredients) {
      await db.execute(
        `INSERT INTO recipe_ingredients (recipe_id, ingredient_id, quantity, unit, is_optional)
         VALUES (?, ?, ?, ?, ?)`,
        [id, ing.ingredient_id, ing.quantity, ing.unit, ing.is_optional ? 1 : 0]
      )
    }

    for (const comp of components) {
      await db.execute(
        `INSERT INTO recipe_components (parent_id, child_id, servings_needed)
         VALUES (?, ?, ?)`,
        [id, comp.child_id, comp.servings_needed]
      )
    }

    for (const step of steps) {
      await db.execute(
        `INSERT INTO steps (id, recipe_id, step_order, step_type, description, duration_min)
         VALUES (?, ?, ?, ?, ?, ?)`,
        [uuid(), id, step.step_order, step.step_type, step.description, step.duration_min]
      )
    }

    for (const tagId of tagIds) {
      await db.execute(
        'INSERT INTO recipe_tags (recipe_id, tag_id) VALUES (?, ?)',
        [id, tagId]
      )
    }
  })

  return id
}

export async function updateRecipe(
  id: string,
  data: Partial<RecipeInput>
): Promise<void> {
  const fields: string[] = []
  const params: unknown[] = []

  const mapped: Record<string, unknown> = { ...data }
  if (data.is_favourite !== undefined) mapped.is_favourite = data.is_favourite ? 1 : 0

  for (const [key, value] of Object.entries(mapped)) {
    if (value !== undefined) {
      fields.push(`${key} = ?`)
      params.push(value)
    }
  }

  if (!fields.length) return

  fields.push('updated_at = ?')
  params.push(new Date().toISOString(), id)

  await dbExecute(
    `UPDATE recipes SET ${fields.join(', ')} WHERE id = ?`,
    params
  )
}

export async function deleteRecipe(id: string): Promise<void> {
  // steps, recipe_ingredients, recipe_tags, recipe_history cascade automatically
  // cover_image file deletion is handled by the caller
  await dbExecute('DELETE FROM recipes WHERE id = ?', [id])
}

export async function toggleFavourite(id: string): Promise<void> {
  await dbExecute(
    `UPDATE recipes
     SET is_favourite = CASE WHEN is_favourite = 1 THEN 0 ELSE 1 END,
         updated_at = ?
     WHERE id = ?`,
    [new Date().toISOString(), id]
  )
}

// ─────────────────────────────────────────
// Cycle detection
// ─────────────────────────────────────────

// call before inserting a recipe_component row
export async function wouldCreateCycle(
  parentId: string,
  childId: string
): Promise<boolean> {
  // if childId is an ancestor of parentId, adding parent→child creates a cycle
  const ancestors = await getAncestors(parentId)
  return ancestors.has(childId)
}

async function getAncestors(recipeId: string): Promise<Set<string>> {
  const rows = await dbSelect<{ parent_id: string }>(`
    WITH RECURSIVE ancestors(id) AS (
      SELECT parent_id FROM recipe_components WHERE child_id = ?
      UNION ALL
      SELECT rc.parent_id FROM recipe_components rc
      JOIN ancestors a ON a.id = rc.child_id
    )
    SELECT id AS parent_id FROM ancestors
  `, [recipeId])

  return new Set(rows.map(r => r.parent_id))
}
