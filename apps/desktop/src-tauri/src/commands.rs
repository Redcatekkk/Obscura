use crate::modules::{reference_modules, ModuleSummary};
use crate::simverse::Simverse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimStatus {
    pub seed: u64,
    pub intensity: u8,
    pub event_count: usize,
}

#[tauri::command]
pub fn modules_list() -> Vec<ModuleSummary> {
    reference_modules()
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

#[cfg(test)]
mod tests {
    use super::{modules_list, sim_status};

    #[test]
    fn modules_list_returns_reference_batch() {
        let modules = modules_list();

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
}
