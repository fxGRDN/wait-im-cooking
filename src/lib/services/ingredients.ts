import { v4 as uuid } from "uuid";
import { dbSelect, dbExecute } from "$lib/db";
import type { Ingredient, IngredientWithInventory } from "$lib/types";

// ─────────────────────────────────────────
// Queries
// ─────────────────────────────────────────

export async function getIngredients(): Promise<Ingredient[]> {
    return dbSelect<Ingredient>(`
    SELECT * FROM ingredients
    ORDER BY name ASC
  `);
}

export async function getIngredient(id: string): Promise<Ingredient | null> {
    const rows = await dbSelect<Ingredient>(
        "SELECT * FROM ingredients WHERE id = ?",
        [id],
    );
    return rows[0] ?? null;
}

export async function getIngredientWithInventory(
    id: string,
): Promise<IngredientWithInventory | null> {
    const rows = await dbSelect<IngredientWithInventory>(
        `
    SELECT
      i.*,
      ii.id        AS inv_id,
      ii.quantity  AS inv_quantity,
      ii.unit      AS inv_unit,
      ii.expires_at AS inv_expires_at
    FROM ingredients i
    LEFT JOIN ingredient_inventory ii ON ii.ingredient_id = i.id
    WHERE i.id = ?
  `,
        [id],
    );

    if (!rows[0]) return null;

    const row = rows[0] as any;
    return {
        id: row.id,
        name: row.name,
        default_unit: row.default_unit,
        inventory: row.inv_id
            ? {
                  id: row.inv_id,
                  ingredient_id: id,
                  quantity: row.inv_quantity,
                  unit: row.inv_unit,
                  expires_at: row.inv_expires_at,
              }
            : null,
    };
}

export async function getInventory(): Promise<IngredientWithInventory[]> {
    const rows = await dbSelect<IngredientWithInventory>(
        `
    SELECT
      i.*,
      ii.id        AS inv_id,
      ii.quantity  AS inv_quantity,
      ii.unit      AS inv_unit,
      ii.expires_at AS inv_expires_at
    FROM ingredients i
    INNER JOIN ingredient_inventory ii ON ii.ingredient_id = i.id
    ORDER BY i.name ASC
  `
    );

    return rows.map((row: any) => ({
        id: row.id,
        name: row.name,
        default_unit: row.default_unit,
        inventory: {
            id: row.inv_id,
            ingredient_id: row.id,
            quantity: row.inv_quantity,
            unit: row.inv_unit,
            expires_at: row.inv_expires_at,
        }
    }));
}

export async function searchIngredients(query: string): Promise<Ingredient[]> {
    return dbSelect<Ingredient>(
        `SELECT * FROM ingredients WHERE name LIKE ? ORDER BY name ASC`,
        [`%${query}%`],
    );
}

// ─────────────────────────────────────────
// Mutations
// ─────────────────────────────────────────

export async function createIngredient(
    name: string,
    default_unit: string | null = null,
): Promise<Ingredient> {
    const ingredient: Ingredient = { id: uuid(), name, default_unit };

    await dbExecute(
        "INSERT INTO ingredients (id, name, default_unit) VALUES (?, ?, ?)",
        [ingredient.id, ingredient.name, ingredient.default_unit],
    );

    return ingredient;
}

export async function getOrCreateIngredient(name: string, unit: string | null = null): Promise<Ingredient> {
    const existing = await dbSelect<Ingredient>("SELECT * FROM ingredients WHERE name = ?", [name]);
    if (existing[0]) return existing[0];
    return createIngredient(name, unit);
}

export async function updateIngredient(
    id: string,
    data: Partial<Pick<Ingredient, "name" | "default_unit">>,
): Promise<void> {
    const fields: string[] = [];
    const params: unknown[] = [];

    if (data.name !== undefined) {
        fields.push("name = ?");
        params.push(data.name);
    }
    if (data.default_unit !== undefined) {
        fields.push("default_unit = ?");
        params.push(data.default_unit);
    }

    if (!fields.length) return;

    params.push(id);
    await dbExecute(
        `UPDATE ingredients SET ${fields.join(", ")} WHERE id = ?`,
        params,
    );
}

export async function deleteIngredient(id: string): Promise<void> {
    // will throw if ingredient is used in any recipe (ON DELETE RESTRICT)
    await dbExecute("DELETE FROM ingredients WHERE id = ?", [id]);
}

// ─────────────────────────────────────────
// Inventory
// ─────────────────────────────────────────

export async function upsertInventory(
    ingredient_id: string,
    quantity: number,
    unit: string,
    expires_at: string | null = null,
): Promise<void> {
    const existing = await dbSelect<{ id: string }>(
        "SELECT id FROM ingredient_inventory WHERE ingredient_id = ?",
        [ingredient_id],
    );

    if (existing[0]) {
        await dbExecute(
            `UPDATE ingredient_inventory
       SET quantity = quantity + ?, unit = ?, expires_at = ?
       WHERE ingredient_id = ?`,
            [quantity, unit, expires_at, ingredient_id],
        );
    } else {
        await dbExecute(
            `INSERT INTO ingredient_inventory (id, ingredient_id, quantity, unit, expires_at)
       VALUES (?, ?, ?, ?, ?)`,
            [uuid(), ingredient_id, quantity, unit, expires_at],
        );
    }
}

export async function deleteInventory(ingredient_id: string): Promise<void> {
    await dbExecute(
        "DELETE FROM ingredient_inventory WHERE ingredient_id = ?",
        [ingredient_id],
    );
}
