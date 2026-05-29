import { test, expect } from "./tauri-mock";

test("Recipe full lifecycle: create, view, cook and save result", async ({ page }) => {
    // 1. Start at Home
    await page.goto("/");
    await expect(page.locator("h1")).toContainText("Hi, Kacper");

    // 2. Navigate to Recipes
    await page.click('a:has-text("Recipes")');
    await expect(page).toHaveURL("/recipes");

    // 3. Click Add Recipe FAB
    await page.click('a[aria-label="Add Recipe"]');
    await expect(page).toHaveURL("/recipes/add");

    // 4. Fill in Recipe details
    await page.fill('input[placeholder="Recipe Title"]', "My Test Recipe");
    await page.fill('textarea[placeholder="What is this recipe about?"]', "A delicious test recipe");

    // 5. Add an ingredient
    await page.click('button:has-text("Add Ingredient")');
    await page.selectOption('select', { index: 2 }); // Select first real ingredient
    await page.fill('input[placeholder="Qty"]', "500");

    // 6. Save Recipe
    // Note: In our mock, create_recipe returns "mock-recipe-id"
    await page.click('button:has-text("Save Recipe")');

    // 7. Verify redirect to recipe detail
    await expect(page).toHaveURL("/recipes/mock-recipe-id");
    await expect(page.locator("h2")).toContainText("My Test Recipe");

    // 8. Start Cooking
    await page.click('button:has-text("Start Cooking")');
    await expect(page).toHaveURL("/recipes/mock-recipe-id/cook");

    // 9. Complete Cooking
    // Note: In our mock, create_cook_log returns "mock-history-id"
    await page.click('button:has-text("Complete")');

    // 10. Verify redirect to history edit view
    await expect(page).toHaveURL(/\/recipes\/history\/mock-history-id\?edit=true/);
    await expect(page.locator("h1")).toContainText("Edit Result");

    // 11. Save History Result
    await page.click('button:has-text("Save")');

    // 12. Should be back at history list
    await expect(page).toHaveURL("/recipes/history");
});
