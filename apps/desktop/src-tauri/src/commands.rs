use crate::bus::LogLine;
use crate::db;
use crate::modules::{reference_modules, ModuleSummary};
use crate::simverse::Simverse;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;

pub struct DeckState {
    pub pool: SqlitePool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimStatus {
    pub seed: u64,
    pub intensity: u8,
    pub event_count: usize,
}

#[tauri::command]
pub async fn modules_list(state: State<'_, DeckState>) -> Result<Vec<ModuleSummary>, String> {
    list_modules_inner(Some(&state.pool)).await
}

#[tauri::command]
pub fn sim_status() -> SimStatus {
    let simverse = Simverse::new(42);
    SimStatus {
        seed: simverse.seed(),
        intensity: 50,
        event_count: simverse.sample_events().len(),
    }
}

#[tauri::command]
pub async fn logs_recent(
    state: State<'_, DeckState>,
    limit: Option<i64>,
) -> Result<Vec<LogLine>, String> {
    logs_recent_inner(Some(&state.pool), limit).await
}

#[tauri::command]
pub async fn modules_set_enabled(
    state: State<'_, DeckState>,
    id: String,
    enabled: bool,
) -> Result<(), String> {
    db::set_module_enabled(&state.pool, &id, enabled)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn modules_trigger_test(
    state: State<'_, DeckState>,
    id: String,
) -> Result<LogLine, String> {
    let log = LogLine {
        ts: "14:03:00".to_string(),
        level: crate::bus::LogLevel::Info,
        source: format!("{id}.self-test"),
        text: format!("triggered local test run for {id}"),
    };
    db::insert_log(&state.pool, &log)
        .await
        .map_err(|error| error.to_string())?;
    Ok(log)
}

async fn list_modules_inner(pool: Option<&SqlitePool>) -> Result<Vec<ModuleSummary>, String> {
    if let Some(pool) = pool {
        return db::list_modules(pool)
            .await
            .map_err(|error| error.to_string());
    }

    Ok(reference_modules())
}

async fn logs_recent_inner(
    pool: Option<&SqlitePool>,
    limit: Option<i64>,
) -> Result<Vec<LogLine>, String> {
    let limit = limit.unwrap_or(20).clamp(1, 100);
    if let Some(pool) = pool {
        return db::recent_logs(pool, limit)
            .await
            .map_err(|error| error.to_string());
    }

    Ok(crate::engine::run_reference_modules(
        &Simverse::new(42).sample_events(),
    ))
}

#[cfg(test)]
mod tests {
    use super::{list_modules_inner, logs_recent_inner, sim_status};
    use crate::db::{connect_memory, list_modules, set_module_enabled};

    #[tokio::test]
    async fn modules_list_returns_reference_batch_without_state() {
        let modules = list_modules_inner(None).await.expect("modules should load");

        assert_eq!(modules.len(), 5);
        assert_eq!(modules[0].id, "f01");
    }

    #[test]
    fn sim_status_reports_deterministic_seed_and_events() {
        let status = sim_status();

        assert_eq!(status.seed, 42);
        assert_eq!(status.intensity, 50);
        assert_eq!(status.event_count, 5);
    }

    #[tokio::test]
    async fn logs_recent_returns_reference_logs_without_state() {
        let logs = logs_recent_inner(None, Some(20))
            .await
            .expect("logs should load");

        assert!(logs.iter().any(|log| log.source == "f01.message_sniper"));
    }

    #[tokio::test]
    async fn module_enabled_state_updates_in_sqlite() {
        let pool = connect_memory().await.expect("db should initialize");
        set_module_enabled(&pool, "f01", false)
            .await
            .expect("module should update");
        let modules = list_modules(&pool).await.expect("modules should load");
        let sniper = modules
            .iter()
            .find(|module| module.id == "f01")
            .expect("f01 should exist");

        assert!(!sniper.enabled);
    }
}
