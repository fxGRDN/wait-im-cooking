mod commands;
mod models;
mod repositories;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;
use tauri::Manager;

use crate::repositories::sqlite::{
    ingredient::SqliteIngredientRepository,
    recipe::SqliteRecipeRepository,
    rest::{SqlitePantryRepository, SqliteRecipeHistoryRepository, SqliteTagRepository},
};
use crate::repositories::traits::{
    IngredientRepository, PantryRepository, RecipeHistoryRepository, RecipeRepository,
    TagRepository,
};

pub struct AppState {
    pub ingredients: Box<dyn IngredientRepository>,
    pub recipes: Box<dyn RecipeRepository>,
    pub tags: Box<dyn TagRepository>,
    pub history: Box<dyn RecipeHistoryRepository>,
    pub pantry: Box<dyn PantryRepository>,
}

async fn init_db(app: &tauri::App) -> SqlitePool {
    let app_dir = app
        .path()
        .app_data_dir()
        .expect("failed to resolve app data dir");

    std::fs::create_dir_all(&app_dir).expect("failed to create app data dir");

    let db_path = app_dir.join("recipes.db");

    log::info!("Database path: {}", db_path.display());
    let options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path.display()))
        .unwrap()
        .create_if_missing(true)
        .pragma("foreign_keys", "ON")
        .pragma("journal_mode", "WAL");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .expect("failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed to run migrations");

    pool
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let pool = tauri::async_runtime::block_on(init_db(app));

            let recipes = SqliteRecipeRepository::new(pool.clone());
            let pantry_recipes = SqliteRecipeRepository::new(pool.clone());

            app.manage(AppState {
                ingredients: Box::new(SqliteIngredientRepository::new(pool.clone())),
                recipes: Box::new(recipes),
                tags: Box::new(SqliteTagRepository::new(pool.clone())),
                history: Box::new(SqliteRecipeHistoryRepository::new(pool.clone())),
                pantry: Box::new(SqlitePantryRepository::new(pool.clone(), pantry_recipes)),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // ingredients
            commands::get_ingredients,
            commands::get_inventory,
            commands::get_ingredient,
            commands::search_ingredients,
            commands::create_ingredient,
            commands::update_ingredient,
            commands::delete_ingredient,
            commands::upsert_inventory,
            commands::delete_inventory,
            // recipes
            commands::get_recipes,
            commands::get_recipe,
            commands::search_recipes,
            commands::create_recipe,
            commands::update_recipe,
            commands::delete_recipe,
            commands::toggle_favourite,
            commands::check_cycle,
            // tags
            commands::get_tags,
            commands::create_tag,
            commands::update_tag,
            commands::delete_tag,
            // history
            commands::get_cook_logs,
            commands::get_cook_log,
            commands::create_cook_log,
            commands::update_cook_log,
            commands::delete_cook_log,
            // pantry
            commands::check_availability,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
