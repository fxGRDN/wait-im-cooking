import { test as base, expect } from "@playwright/test";

// This helper will mock Tauri internals in the browser
export const test = base.extend({
  page: async ({ page }, use) => {
    await page.addInitScript(() => {
      // Mock window.__TAURI_INTERNALS__
      (window as any).__TAURI_INTERNALS__ = {
        invoke: async (cmd: string, args: any) => {
          console.log(`[Mock Tauri] invoke: ${cmd}`, args);

          switch (cmd) {
            case "get_recipes":
              return [];
            case "get_tags":
              return [];
            case "get_ingredients":
              return [
                {
                  id: "ing-1",
                  name: "Tomato",
                  default_unit: "g",
                  restock_threshold: 100,
                },
              ];
            case "get_inventory":
              return [
                {
                  id: "ing-1",
                  name: "Tomato",
                  default_unit: "g",
                  restock_threshold: 100,
                  inventory: { quantity: 500, unit: "g" },
                },
              ];
            case "get_cook_logs":
              return [];
            case "create_recipe":
              return "mock-recipe-id";
            case "get_recipe":
              return {
                id: "mock-recipe-id",
                title:
                  args.id === "mock-recipe-id" ? "My Test Recipe" : "Unknown",
                description: "Test Description",
                servings: 2,
                prep_time: 10,
                cook_time: 20,
                is_favourite: false,
                cover_image: null,
                ingredients: [],
                components: [],
                steps: [],
                tags: [],
              };
            case "create_cook_log":
              return "mock-history-id";
            case "update_cook_log":
              return [];
            case "update_recipe":
              return null;
            case "get_cook_log":
              return {
                id: "mock-history-id",
                recipe_id: "mock-recipe-id",
                servings_made: 2,
                duration_min: 30,
                rating: 5,
                notes: "Tastes great!",
                created_at: new Date().toISOString(),
                images: [],
              };
            default:
              return null;
          }
        },
        metadata: {
          name: "Wait, I'm Cooking!",
          version: "0.1.0",
        },
      };

      // Mock other possible globals
      (window as any).__TAURI__ = {
        invoke: (window as any).__TAURI_INTERNALS__.invoke,
        convertFileSrc: (p: string) => p,
      };
    });
    await use(page);
  },
});

export { expect };
