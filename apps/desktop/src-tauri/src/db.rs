use crate::bus::{LogLevel, LogLine};
use crate::engine::run_reference_modules;
use crate::modules::{Category, ModuleSummary};
use crate::simverse::Simverse;
use serde_json::Value;
use sqlx::{sqlite::SqlitePoolOptions, Row, SqlitePool};

type DbResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn connect_memory() -> DbResult<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await?;
    run_migrations(&pool).await?;
    seed_modules(&pool).await?;
    bootstrap_logs(&pool).await?;
    Ok(pool)
}

pub async fn bootstrap_logs(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let events = Simverse::new(42).sample_events();
    let logs = run_reference_modules(&events);
    for log in &logs {
        insert_log(pool, log).await?;
    }
    Ok(())
}

pub async fn run_migrations(pool: &SqlitePool) -> DbResult<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

pub async fn seed_modules(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    for module in crate::modules::reference_modules() {
        sqlx::query(
            r#"
            INSERT INTO modules (id, name, category, description, enabled, config)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                category = excluded.category,
                description = excluded.description,
                enabled = excluded.enabled,
                config = modules.config,
                updated_at = CURRENT_TIMESTAMP
            "#,
        )
        .bind(module.id)
        .bind(module.name)
        .bind(category_name(module.category))
        .bind(module.description)
        .bind(module.enabled)
        .bind("{}")
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn set_module_enabled(
    pool: &SqlitePool,
    id: &str,
    enabled: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE modules
        SET enabled = ?, updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
        "#,
    )
    .bind(enabled)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn list_modules(pool: &SqlitePool) -> Result<Vec<ModuleSummary>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT id, name, category, description, enabled
        FROM modules
        ORDER BY id ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| ModuleSummary {
            id: Box::leak(row.get::<String, _>("id").into_boxed_str()),
            name: Box::leak(row.get::<String, _>("name").into_boxed_str()),
            category: parse_category(&row.get::<String, _>("category")),
            description: Box::leak(row.get::<String, _>("description").into_boxed_str()),
            enabled: row.get::<i64, _>("enabled") != 0,
        })
        .collect())
}

pub async fn get_module_config(pool: &SqlitePool, id: &str) -> Result<Value, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT config
        FROM modules
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    let config = row.get::<String, _>("config");
    Ok(serde_json::from_str(&config).unwrap_or_else(|_| serde_json::json!({})))
}

pub async fn set_module_config(
    pool: &SqlitePool,
    id: &str,
    config: &Value,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE modules
        SET config = ?, updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
        "#,
    )
    .bind(config.to_string())
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn insert_log(pool: &SqlitePool, log: &LogLine) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO logs (ts, level, source, text)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(&log.ts)
    .bind(log_level_name(&log.level))
    .bind(&log.source)
    .bind(&log.text)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn recent_logs(pool: &SqlitePool, limit: i64) -> Result<Vec<LogLine>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT ts, level, source, text
        FROM logs
        ORDER BY id DESC
        LIMIT ?
        "#,
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| LogLine {
            ts: row.get("ts"),
            level: parse_log_level(&row.get::<String, _>("level")),
            source: row.get("source"),
            text: row.get("text"),
        })
        .collect())
}

fn category_name(category: Category) -> &'static str {
    match category {
        Category::Automation => "Automation",
        Category::Utility => "Utility",
        Category::Privacy => "Privacy",
        Category::Stats => "Stats",
        Category::Fun => "Fun",
    }
}

fn parse_category(value: &str) -> Category {
    match value {
        "Automation" => Category::Automation,
        "Utility" => Category::Utility,
        "Privacy" => Category::Privacy,
        "Stats" => Category::Stats,
        "Fun" => Category::Fun,
        _ => Category::Utility,
    }
}

fn log_level_name(level: &LogLevel) -> &'static str {
    match level {
        LogLevel::Info => "Info",
        LogLevel::Warn => "Warn",
        LogLevel::Error => "Error",
    }
}

fn parse_log_level(value: &str) -> LogLevel {
    match value {
        "Info" => LogLevel::Info,
        "Warn" => LogLevel::Warn,
        "Error" => LogLevel::Error,
        _ => LogLevel::Info,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        connect_memory, get_module_config, insert_log, list_modules, recent_logs, seed_modules,
        set_module_config,
    };
    use crate::{engine::run_reference_modules, simverse::Simverse};

    #[tokio::test]
    async fn sqlite_memory_db_seeds_modules() {
        let pool = connect_memory().await.expect("db should initialize");
        let modules = list_modules(&pool).await.expect("modules should load");

        assert_eq!(modules.len(), 20);
        assert_eq!(modules[0].id, "f01");
        assert_eq!(modules.last().map(|module| module.id), Some("f20"));
    }

    #[tokio::test]
    async fn seeded_module_config_defaults_to_empty_object() {
        let pool = connect_memory().await.expect("db should initialize");
        let config = get_module_config(&pool, "f01")
            .await
            .expect("config should load");

        assert_eq!(config, serde_json::json!({}));
    }

    #[tokio::test]
    async fn reseeding_preserves_unique_rows() {
        let pool = connect_memory().await.expect("db should initialize");
        seed_modules(&pool)
            .await
            .expect("seed should be idempotent");
        let modules = list_modules(&pool).await.expect("modules should load");

        assert_eq!(modules.len(), 20);
    }

    #[tokio::test]
    async fn module_logs_persist_to_sqlite() {
        let pool = connect_memory().await.expect("db should initialize");
        let before = recent_logs(&pool, 20)
            .await
            .expect("logs should load")
            .len();
        let logs = run_reference_modules(&Simverse::new(42).sample_events());
        for log in &logs {
            insert_log(&pool, log).await.expect("log should persist");
        }

        let stored = recent_logs(&pool, 20).await.expect("logs should load");

        assert_eq!(stored.len(), before + logs.len());
        assert!(stored.iter().any(|log| log.source == "f01.message_sniper"));
    }

    #[tokio::test]
    async fn module_config_round_trips_through_sqlite() {
        let pool = connect_memory().await.expect("db should initialize");

        let initial = get_module_config(&pool, "f01")
            .await
            .expect("config should load");
        assert_eq!(initial, serde_json::json!({}));

        let next = serde_json::json!({
            "capture_window_minutes": 15,
            "channel_scope": "simverse.guild.*"
        });
        set_module_config(&pool, "f01", &next)
            .await
            .expect("config should persist");

        let stored = get_module_config(&pool, "f01")
            .await
            .expect("config should reload");
        assert_eq!(stored, next);
    }
}
