use rusqlite::{Connection, params};
use std::sync::Mutex;
use tracing;

use crate::commands::history::HistoryItem;

/// SQLite-backed transcription history store
pub struct HistoryStore {
    db: Mutex<Connection>,
}

impl HistoryStore {
    pub fn new() -> Result<Self, String> {
        let db_path = get_db_path()?;

        // Ensure parent directory exists
        if let Some(parent) = std::path::Path::new(&db_path).parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create history directory: {}", e))?;
        }

        let conn = Connection::open(&db_path)
            .map_err(|e| format!("Failed to open history database: {}", e))?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS history (
                id TEXT PRIMARY KEY,
                raw_text TEXT NOT NULL,
                final_text TEXT NOT NULL,
                stt_provider TEXT NOT NULL DEFAULT '',
                llm_provider TEXT NOT NULL DEFAULT '',
                app_name TEXT NOT NULL DEFAULT 'Unknown',
                duration_ms INTEGER NOT NULL DEFAULT 0,
                mode TEXT NOT NULL DEFAULT 'cleanup',
                created_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_history_created
                ON history(created_at DESC);

            CREATE INDEX IF NOT EXISTS idx_history_mode
                ON history(mode);"
        ).map_err(|e| format!("Failed to create history table: {}", e))?;

        tracing::info!("History store initialized at {}", db_path);

        Ok(Self {
            db: Mutex::new(conn),
        })
    }

    pub fn add(&self, item: &HistoryItem) -> Result<(), String> {
        let conn = self.db.lock().map_err(|e| e.to_string())?;

        conn.execute(
            "INSERT INTO history (id, raw_text, final_text, stt_provider, llm_provider, app_name, duration_ms, mode, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                item.id,
                item.raw_text,
                item.final_text,
                item.stt_provider,
                item.llm_provider,
                item.app_name,
                item.duration_ms,
                item.mode,
                item.created_at,
            ],
        ).map_err(|e| format!("Failed to insert history: {}", e))?;

        Ok(())
    }

    pub fn get_all(&self, limit: Option<usize>, search: Option<&str>) -> Result<Vec<HistoryItem>, String> {
        let conn = self.db.lock().map_err(|e| e.to_string())?;

        let limit_val = limit.unwrap_or(100);
        let mut query = String::from(
            "SELECT id, raw_text, final_text, stt_provider, llm_provider, app_name, duration_ms, mode, created_at
             FROM history "
        );

        if let Some(q) = search.filter(|s| !s.is_empty()) {
            query.push_str("WHERE raw_text LIKE ?1 OR final_text LIKE ?1 ");
            let pattern = format!("%{}%", q);
            query.push_str("ORDER BY created_at DESC LIMIT ?2");

            let mut stmt = conn.prepare(&query)
                .map_err(|e| e.to_string())?;
            let rows = stmt.query_map(params![pattern, limit_val], |row| {
                Ok(HistoryItem {
                    id: row.get(0)?,
                    raw_text: row.get(1)?,
                    final_text: row.get(2)?,
                    stt_provider: row.get(3)?,
                    llm_provider: row.get(4)?,
                    app_name: row.get(5)?,
                    duration_ms: row.get(6)?,
                    mode: row.get(7)?,
                    created_at: row.get(8)?,
                })
            }).map_err(|e| e.to_string())?;

            let mut items = Vec::new();
            for row in rows {
                items.push(row.map_err(|e| e.to_string())?);
            }
            Ok(items)
        } else {
            query.push_str("ORDER BY created_at DESC LIMIT ?1");

            let mut stmt = conn.prepare(&query)
                .map_err(|e| e.to_string())?;
            let rows = stmt.query_map(params![limit_val], |row| {
                Ok(HistoryItem {
                    id: row.get(0)?,
                    raw_text: row.get(1)?,
                    final_text: row.get(2)?,
                    stt_provider: row.get(3)?,
                    llm_provider: row.get(4)?,
                    app_name: row.get(5)?,
                    duration_ms: row.get(6)?,
                    mode: row.get(7)?,
                    created_at: row.get(8)?,
                })
            }).map_err(|e| e.to_string())?;

            let mut items = Vec::new();
            for row in rows {
                items.push(row.map_err(|e| e.to_string())?);
            }
            Ok(items)
        }
    }

    pub fn remove(&self, id: &str) -> Result<(), String> {
        let conn = self.db.lock().map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM history WHERE id = ?1", params![id])
            .map_err(|e| format!("Failed to delete history: {}", e))?;
        Ok(())
    }
}

fn get_db_path() -> Result<String, String> {
    let data_dir = dirs_data_dir()?;
    Ok(format!("{}/history.db", data_dir))
}

fn dirs_data_dir() -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
        Ok(format!("{}/Library/Application Support/com.voxtype.app", home))
    }

    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("APPDATA").unwrap_or_else(|_| "C:\\".into());
        Ok(format!("{}\\VoxType", appdata))
    }

    #[cfg(target_os = "linux")]
    {
        let xdg = std::env::var("XDG_DATA_HOME").unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
            format!("{}/.local/share", home)
        });
        Ok(format!("{}/voxtype", xdg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_store() -> HistoryStore {
        // Use in-memory DB for testing
        HistoryStore {
            db: Mutex::new(Connection::open_in_memory().unwrap()),
        }
    }

    fn test_item(id: &str, text: &str) -> HistoryItem {
        HistoryItem {
            id: id.to_string(),
            raw_text: text.to_string(),
            final_text: format!("[polished] {}", text),
            stt_provider: "groq".to_string(),
            llm_provider: "deepseek".to_string(),
            app_name: "TestApp".to_string(),
            duration_ms: 5000,
            mode: "cleanup".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    #[test]
    fn test_add_and_get_history() {
        let store = test_store();
        // Manually create table
        store.db.lock().unwrap().execute_batch(
            "CREATE TABLE IF NOT EXISTS history (
                id TEXT PRIMARY KEY,
                raw_text TEXT NOT NULL,
                final_text TEXT NOT NULL,
                stt_provider TEXT NOT NULL DEFAULT '',
                llm_provider TEXT NOT NULL DEFAULT '',
                app_name TEXT NOT NULL DEFAULT 'Unknown',
                duration_ms INTEGER NOT NULL DEFAULT 0,
                mode TEXT NOT NULL DEFAULT 'cleanup',
                created_at TEXT NOT NULL
            )"
        ).unwrap();

        store.add(&test_item("1", "hello world")).unwrap();
        let items = store.get_all(None, None).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].raw_text, "hello world");
    }

    #[test]
    fn test_search_history() {
        let store = test_store();
        store.db.lock().unwrap().execute_batch(
            "CREATE TABLE IF NOT EXISTS history (
                id TEXT PRIMARY KEY,
                raw_text TEXT NOT NULL,
                final_text TEXT NOT NULL,
                stt_provider TEXT NOT NULL DEFAULT '',
                llm_provider TEXT NOT NULL DEFAULT '',
                app_name TEXT NOT NULL DEFAULT 'Unknown',
                duration_ms INTEGER NOT NULL DEFAULT 0,
                mode TEXT NOT NULL DEFAULT 'cleanup',
                created_at TEXT NOT NULL
            )"
        ).unwrap();

        store.add(&test_item("a", "hello")).unwrap();
        store.add(&test_item("b", "goodbye")).unwrap();

        let found = store.get_all(None, Some("hello")).unwrap();
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].raw_text, "hello");
    }

    #[test]
    fn test_remove_history() {
        let store = test_store();
        store.db.lock().unwrap().execute_batch(
            "CREATE TABLE IF NOT EXISTS history (
                id TEXT PRIMARY KEY,
                raw_text TEXT NOT NULL,
                final_text TEXT NOT NULL,
                stt_provider TEXT NOT NULL DEFAULT '',
                llm_provider TEXT NOT NULL DEFAULT '',
                app_name TEXT NOT NULL DEFAULT 'Unknown',
                duration_ms INTEGER NOT NULL DEFAULT 0,
                mode TEXT NOT NULL DEFAULT 'cleanup',
                created_at TEXT NOT NULL
            )"
        ).unwrap();

        store.add(&test_item("x", "delete me")).unwrap();
        store.remove("x").unwrap();
        let items = store.get_all(None, None).unwrap();
        assert_eq!(items.len(), 0);
    }
}
