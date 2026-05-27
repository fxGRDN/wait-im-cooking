import Database from "@tauri-apps/plugin-sql";
import schema from "./schema.sql?raw";

let db: Database;

export type SqlClient = Pick<Database, "select" | "execute">;

export function isTauriRuntime(): boolean {
    return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export async function getDb(): Promise<Database> {
    if (!isTauriRuntime()) {
        throw new Error("Tauri runtime not available. Run the app via `tauri dev`.");
    }

    if (!db) {
        db = await Database.load("sqlite:recipes.db");
        await db.execute(schema);
    }
    return db;
}

export async function dbSelect<T>(
    query: string,
    params: unknown[] = [],
): Promise<T[]> {
    const client = await getDb();
    return (await client.select(query, params)) as T[];
}

export async function dbExecute(
    query: string,
    params: unknown[] = [],
): Promise<unknown> {
    const client = await getDb();
    return client.execute(query, params);
}

export async function dbTransaction<T>(
    callback: (db: SqlClient) => Promise<T>,
): Promise<T> {
    const client = await getDb();
    await client.execute("BEGIN");

    try {
        const result = await callback(client);
        await client.execute("COMMIT");
        return result;
    } catch (error) {
        await client.execute("ROLLBACK");
        throw error;
    }
}
