import { test, expect } from "./tauri-mock";

test("Recipe full lifecycle: create, view, cook and save result", async ({
  page,
}) => {
  // 1. Start at Home
  await page.goto("/");
  await expect(page.locator("h1")).toContainText("Wait, I'm Cooking!");

  // 2. Navigate to Recipes
  await page.click('a:has-text("Recipes")');
  await expect(page).toHaveURL("/recipes");

  // 3. Click Add Recipe FAB
  await page.click('a[aria-label="Add Recipe"]');
  await expect(page).toHaveURL("/recipes/add");

  // 4. Fill in Recipe details
  await page.getByLabel("Recipe Title").fill("My Test Recipe");
  await page.getByLabel("Description").fill("A delicious test recipe");

  // 5. Add an ingredient
  await page.getByRole("button", { name: "Add Ingredient" }).click();
  await page.locator("select").first().selectOption({ index: 2 });
  await page.getByPlaceholder("Qty").fill("500");

  // 6. Save Recipe
  // Note: In our mock, create_recipe returns "mock-recipe-id"
  await page.getByRole("button", { name: "Save", exact: true }).click();

  // 7. Verify redirect to recipe detail
  await expect(page).toHaveURL("/recipes/mock-recipe-id");
  await expect(page.getByRole("heading", { level: 1 })).toContainText(
    "My Test Recipe",
  );

  // 8. Start Cooking
  await page.getByRole("button", { name: "Start Cooking" }).click();
  await expect(page).toHaveURL("/recipes/mock-recipe-id/cook");

  // 9. Complete Cooking
  // Note: In our mock, create_cook_log returns "mock-history-id"
  await page.getByRole("button", { name: "Complete" }).click();

  // 10. Verify redirect to history edit view
  await expect(page).toHaveURL(
    /\/recipes\/history\/mock-history-id\?edit=true/,
  );
  await expect(page.getByRole("heading", { level: 1 })).toContainText(
    "Edit Result",
  );

  // 11. Save History Result
  await page.getByRole("button", { name: "Save" }).click();

  // 12. Should be back at history list
  await expect(page).toHaveURL("/recipes/history");
});
