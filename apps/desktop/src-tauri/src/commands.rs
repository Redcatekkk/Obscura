use crate::bus::LogLine;
use crate::db;
use crate::modules::{reference_modules, ModuleSummary};
use crate::simverse::Simverse;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::SqlitePool;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};

pub struct DeckState {
    pub pool: SqlitePool,
}

#[derive(Debug)]
pub struct SimControl {
    intensity: Mutex<u8>,
}

impl Default for SimControl {
    fn default() -> Self {
        Self {
            intensity: Mutex::new(50),
        }
    }
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
pub fn sim_status(sim: State<'_, SimControl>) -> Result<SimStatus, String> {
    sim_status_inner(&sim)
}

#[tauri::command]
pub async fn sim_set_intensity(
    sim: State<'_, SimControl>,
    intensity: u16,
) -> Result<SimStatus, String> {
    set_sim_intensity_inner(&sim, intensity).await
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
pub async fn modules_get_config(state: State<'_, DeckState>, id: String) -> Result<Value, String> {
    modules_get_config_inner(&state.pool, &id).await
}

#[tauri::command]
pub async fn modules_set_config(
    state: State<'_, DeckState>,
    id: String,
    config: Value,
) -> Result<Value, String> {
    modules_set_config_inner(&state.pool, &id, config).await
}

#[tauri::command]
pub async fn modules_trigger_test(
    state: State<'_, DeckState>,
    app: AppHandle,
    id: String,
) -> Result<LogLine, String> {
    let log = module_test_log(&id);
    db::insert_log(&state.pool, &log)
        .await
        .map_err(|error| error.to_string())?;
    emit_deck_log(&app, &log)?;
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

async fn modules_get_config_inner(pool: &SqlitePool, id: &str) -> Result<Value, String> {
    db::get_module_config(pool, id)
        .await
        .map_err(|error| error.to_string())
}

async fn modules_set_config_inner(
    pool: &SqlitePool,
    id: &str,
    config: Value,
) -> Result<Value, String> {
    db::set_module_config(pool, id, &config)
        .await
        .map_err(|error| error.to_string())?;
    modules_get_config_inner(pool, id).await
}

fn sim_status_inner(sim: &SimControl) -> Result<SimStatus, String> {
    let simverse = Simverse::new(42);
    let intensity = *sim
        .intensity
        .lock()
        .map_err(|_| "sim control lock poisoned".to_string())?;

    Ok(SimStatus {
        seed: simverse.seed(),
        intensity,
        event_count: simverse.sample_events().len(),
    })
}

async fn set_sim_intensity_inner(sim: &SimControl, intensity: u16) -> Result<SimStatus, String> {
    if intensity > 100 {
        return Err("sim intensity must be in 0..=100".to_string());
    }

    *sim.intensity
        .lock()
        .map_err(|_| "sim control lock poisoned".to_string())? = intensity as u8;
    sim_status_inner(sim)
}

fn module_test_log(id: &str) -> LogLine {
    LogLine {
        ts: "14:03:00".to_string(),
        level: crate::bus::LogLevel::Info,
        source: format!("{id}.self-test"),
        text: format!("triggered local test run for {id}"),
    }
}

fn emit_deck_log(app: &AppHandle, log: &LogLine) -> Result<(), String> {
    app.emit("deck:event", log.clone())
        .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::{
        list_modules_inner, logs_recent_inner, module_test_log, modules_get_config_inner,
        modules_set_config_inner, set_sim_intensity_inner, sim_status_inner,
    };
    use crate::db::{connect_memory, list_modules, set_module_enabled};

    #[tokio::test]
    async fn modules_list_returns_reference_batch_without_state() {
        let modules = list_modules_inner(None).await.expect("modules should load");

        assert_eq!(modules.len(), 50);
        assert_eq!(modules[0].id, "f01");
        assert_eq!(modules.last().map(|module| module.id), Some("f50"));
    }

    #[test]
    fn sim_status_reports_deterministic_seed_and_events() {
        let state = super::SimControl::default();
        let status = sim_status_inner(&state).expect("sim status should load");

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

    #[tokio::test]
    async fn sim_intensity_state_updates_and_rejects_out_of_range_values() {
        let state = super::SimControl::default();

        let updated = set_sim_intensity_inner(&state, 0)
            .await
            .expect("zero intensity should be valid");
        assert_eq!(updated.intensity, 0);

        let updated = set_sim_intensity_inner(&state, 75)
            .await
            .expect("midrange intensity should be valid");
        assert_eq!(updated.intensity, 75);

        let error = set_sim_intensity_inner(&state, 101)
            .await
            .expect_err("out-of-range intensity should fail");
        assert!(error.contains("0..=100"));
    }

    #[test]
    fn module_test_log_uses_tauri_event_payload_shape() {
        let log = module_test_log("f01");
        let json = serde_json::to_value(&log).expect("deck event should serialize");

        assert_eq!(log.source, "f01.self-test");
        assert_eq!(json["text"], "triggered local test run for f01");
    }

    #[tokio::test]
    async fn module_config_commands_round_trip_json() {
        let pool = connect_memory().await.expect("db should initialize");
        let config = serde_json::json!({
            "capture_window_minutes": 20,
            "redaction": true
        });

        let stored = modules_set_config_inner(&pool, "f01", config.clone())
            .await
            .expect("config should persist");
        assert_eq!(stored, config);

        let reloaded = modules_get_config_inner(&pool, "f01")
            .await
            .expect("config should load");
        assert_eq!(reloaded, config);
    }
}
