PRAGMA foreign_keys = ON;
PRAGMA user_version = 1;

-- ─────────────────────────────────────────
-- Ingredients
-- ─────────────────────────────────────────

CREATE TABLE IF NOT EXISTS ingredients (
  id           TEXT PRIMARY KEY ,
  name         TEXT NOT NULL UNIQUE,
  default_unit TEXT
);

CREATE TABLE IF NOT EXISTS ingredient_inventory (
  id            TEXT PRIMARY KEY,
  ingredient_id TEXT NOT NULL REFERENCES ingredients(id) ON DELETE CASCADE,
  quantity      REAL NOT NULL,
  unit          TEXT NOT NULL,
  expires_at    TEXT
);

-- ─────────────────────────────────────────
-- Recipes
-- ─────────────────────────────────────────

CREATE TABLE IF NOT EXISTS recipes (
  id            TEXT PRIMARY KEY,
  title         TEXT NOT NULL,
  description   TEXT,
  servings      INTEGER,
  prep_time     INTEGER,  -- minutes
  cook_time     INTEGER,  -- minutes
  is_favourite  INTEGER NOT NULL DEFAULT 0 CHECK (is_favourite IN (0, 1)),
  cover_image   TEXT,     -- relative path under appDataDir
  created_at    TEXT NOT NULL,
  updated_at    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS recipe_ingredients (
  recipe_id     TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  ingredient_id TEXT NOT NULL REFERENCES ingredients(id) ON DELETE RESTRICT,
  quantity      REAL NOT NULL,
  unit          TEXT NOT NULL,
  is_optional   INTEGER NOT NULL DEFAULT 0 CHECK (is_optional IN (0, 1)),
  PRIMARY KEY (recipe_id, ingredient_id)
);

CREATE TABLE IF NOT EXISTS recipe_components (
  parent_id      TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  child_id       TEXT NOT NULL REFERENCES recipes(id) ON DELETE RESTRICT,
  servings_needed REAL NOT NULL,
  PRIMARY KEY (parent_id, child_id),
  CHECK (parent_id != child_id)   -- no self-reference; full cycle detection in app layer
);

-- ─────────────────────────────────────────
-- Steps
-- ─────────────────────────────────────────

CREATE TABLE IF NOT EXISTS steps (
  id           TEXT PRIMARY KEY,
  recipe_id    TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  step_order   INTEGER NOT NULL,
  step_type    TEXT NOT NULL CHECK (step_type IN ('prep', 'cook')),
  description  TEXT NOT NULL,
  duration_min INTEGER,
  UNIQUE (recipe_id, step_order)
);

-- ─────────────────────────────────────────
-- Tags
-- ─────────────────────────────────────────

CREATE TABLE IF NOT EXISTS tags (
  id   TEXT PRIMARY KEY,
  name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS recipe_tags (
  recipe_id TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  tag_id    TEXT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
  PRIMARY KEY (recipe_id, tag_id)
);

-- ─────────────────────────────────────────
-- Cook history
-- ─────────────────────────────────────────

CREATE TABLE IF NOT EXISTS recipe_history (
  id           TEXT PRIMARY KEY,
  recipe_id    TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  servings_made INTEGER,
  duration_min INTEGER,
  rating       INTEGER CHECK (rating BETWEEN 1 AND 5),
  notes        TEXT,
  created_at   TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS recipe_history_images (
  id         TEXT PRIMARY KEY,
  history_id TEXT NOT NULL REFERENCES recipe_history(id) ON DELETE CASCADE,
  file_path  TEXT NOT NULL,  -- relative path under appDataDir
  created_at TEXT NOT NULL
);
