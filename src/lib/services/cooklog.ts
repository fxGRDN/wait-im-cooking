import { v4 as uuid } from 'uuid'
import { dbSelect, dbExecute, dbTransaction } from '$lib/db'
import { getRecipeWithTree } from './recipes'
import type {
  RecipeHistory,
  RecipeHistoryImage,
  RecipeHistoryWithImages,
  RecipeHistoryInput,
  AvailabilityResult,
  RecipeWithTree,
} from '$lib/types'

// ─────────────────────────────────────────
// Pantry / availability
// ─────────────────────────────────────────

// checks recursively whether pantry covers all ingredients in a recipe tree
export async function checkAvailability(recipeId: string): Promise<AvailabilityResult> {
  const tree = await getRecipeWithTree(recipeId)
  if (!tree) throw new Error(`Recipe ${recipeId} not found`)
  return checkTreeAvailability(tree)
}

async function checkTreeAvailability(recipe: RecipeWithTree): Promise<AvailabilityResult> {
  const missing = []

  for (const ri of recipe.ingredients) {
    if (ri.is_optional) continue

    const rows = await dbSelect<{ quantity: number; unit: string }>(
      `SELECT quantity, unit FROM ingredient_inventory WHERE ingredient_id = ?`,
      [ri.ingredient_id]
    )

    const available = rows[0]?.quantity ?? 0
    const sufficient = available >= ri.quantity

    if (!sufficient) {
      missing.push({
        ingredient: ri.ingredient,
        required: ri.quantity,
        unit: ri.unit,
        available,
        sufficient,
      })
    }
  }

  const componentResults = await Promise.all(
    recipe.components.map(c => checkTreeAvailability(c.child))
  )

  return {
    recipe_id: recipe.id,
    cookable: missing.length === 0 && componentResults.every(r => r.cookable),
    missing,
    components: componentResults,
  }
}

// deducts all ingredients from pantry after cooking
export async function consumeIngredients(
  recipeId: string,
  servingsCooked: number
): Promise<void> {
  const tree = await getRecipeWithTree(recipeId)
  if (!tree) throw new Error(`Recipe ${recipeId} not found`)

  const baseServings = tree.servings ?? 1
  const ratio = servingsCooked / baseServings

  await dbTransaction(async (db) => {
    await deductTree(db, tree, ratio)
  })
}

async function deductTree(
  db: any,
  recipe: RecipeWithTree,
  ratio: number
): Promise<void> {
  for (const ri of recipe.ingredients) {
    if (ri.is_optional) continue
    await db.execute(
      `UPDATE ingredient_inventory
       SET quantity = MAX(0, quantity - ?)
       WHERE ingredient_id = ?`,
      [ri.quantity * ratio, ri.ingredient_id]
    )
  }

  for (const comp of recipe.components) {
    const childRatio = (comp.servings_needed / (comp.child.servings ?? 1)) * ratio
    await deductTree(db, comp.child, childRatio)
  }
}

// ─────────────────────────────────────────
// Cook log
// ─────────────────────────────────────────

export async function getCookLogs(recipeId?: string): Promise<RecipeHistory[]> {
  if (recipeId) {
    return dbSelect<RecipeHistory>(
      'SELECT * FROM recipe_history WHERE recipe_id = ? ORDER BY created_at DESC',
      [recipeId]
    )
  }
  return dbSelect<RecipeHistory>(
    'SELECT * FROM recipe_history ORDER BY created_at DESC'
  )
}

export async function getCookLog(id: string): Promise<RecipeHistoryWithImages | null> {
  const rows = await dbSelect<RecipeHistory>(
    'SELECT * FROM recipe_history WHERE id = ?',
    [id]
  )
  if (!rows[0]) return null

  const images = await dbSelect<RecipeHistoryImage>(
    'SELECT * FROM recipe_history_images WHERE history_id = ? ORDER BY created_at ASC',
    [id]
  )

  return { ...rows[0], images }
}

export async function createCookLog(
  data: RecipeHistoryInput,
  imagePaths: string[] = [],
  consumeFromPantry = false
): Promise<string> {
  const id = uuid()
  const now = new Date().toISOString()

  await dbTransaction(async (db) => {
    await db.execute(
      `INSERT INTO recipe_history
        (id, recipe_id, servings_made, duration_min, rating, notes, created_at)
       VALUES (?, ?, ?, ?, ?, ?, ?)`,
      [id, data.recipe_id, data.servings_made, data.duration_min,
       data.rating, data.notes, now]
    )

    for (const path of imagePaths) {
      await db.execute(
        `INSERT INTO recipe_history_images (id, history_id, file_path, created_at)
         VALUES (?, ?, ?, ?)`,
        [uuid(), id, path, now]
      )
    }
  })

  if (consumeFromPantry && data.servings_made) {
    await consumeIngredients(data.recipe_id, data.servings_made)
  }

  return id
}

// deletes log entry + cleans up image files from disk
export async function deleteCookLog(
  id: string,
  removeFiles: (paths: string[]) => Promise<void>
): Promise<void> {
  const images = await dbSelect<RecipeHistoryImage>(
    'SELECT * FROM recipe_history_images WHERE history_id = ?',
    [id]
  )

  // files first — if this fails we still have the DB rows to retry
  await removeFiles(images.map(i => i.file_path))

  // CASCADE handles recipe_history_images rows
  await dbExecute('DELETE FROM recipe_history WHERE id = ?', [id])
}
