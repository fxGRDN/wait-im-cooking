-- Record which ingredients were used and if they were successfully deducted at the time of cooking
CREATE TABLE IF NOT EXISTS recipe_history_ingredients (
  id            TEXT PRIMARY KEY,
  history_id    TEXT NOT NULL REFERENCES recipe_history(id) ON DELETE CASCADE,
  ingredient_id TEXT NOT NULL REFERENCES ingredients(id) ON DELETE RESTRICT,
  name          TEXT NOT NULL, -- Denormalized name snapshot
  quantity      REAL NOT NULL,
  unit          TEXT NOT NULL,
  was_deducted  INTEGER NOT NULL DEFAULT 0 CHECK (was_deducted IN (0, 1))
);
