import Database, { type QueryResult } from 'tauri-plugin-sql-api';

/**
 * The path is relative to `tauri::api::path::BaseDirectory::App`.
 *
 * mysql:
 *  const db = await Database.load("mysql://user:pass@host/database");
 * postgres:
 *  const db = await Database.load("postgres://postgres:password@localhost/test");
 */
// const db: Database = await Database.load("sqlite:tauri-skeleton.db");

/**
 * 
 * @returns 
 */
export const loadDb = async ():Promise<Database> => {
    return await Database.load("sqlite:tauri-skeleton.db");
}

/**
 * eg:
 *   execute("insert into test(id, title, status) values($1,$2,$3)", [1, 'test', '01'])
 * @param sql
 * @param params
 */
export const execute = async (sql: string, params?: unknown[]): Promise<QueryResult> => {
    const db = await loadDb();
    return await db.execute(sql, params);
}

/**
 *eg:
 *   execute("update test set title = $1, status = $2 where id = $3", ['test', '01', 1])
 * @param sql
 * @param params
 * @param exceptType
 */
export const select = async <T>(sql: string, params?: unknown[]): Promise<T[]>=>{
    const db = await loadDb();
    return await db.select<T[]>(sql, params);
}

/**
 * 
 * @param sql 
 * @param params 
 * @returns 
 */
export const selectOne = async <T>(sql: string, params?: unknown[]): Promise<T|undefined>=>{
    const db = await loadDb();
    const result: T[] = await db.select<T[]>(sql, params);
    return result && result.length ? result[0]: undefined;
}