use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeckEvent {
    Log(LogLine),
    ModuleStateChanged { id: String, enabled: bool },
    KpiTick(KpiSnapshot),
    SimEvent(SimEvent),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogLine {
    pub ts: String,
    pub level: LogLevel,
    pub source: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KpiSnapshot {
    pub active_modules: usize,
    pub sim_events: usize,
    pub latency_ms: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimEvent {
    MessageCreate {
        channel: String,
        author: String,
        content: String,
    },
    MessageDelete {
        channel: String,
        author: String,
        content: String,
    },
    MemberJoin {
        guild: String,
        user: String,
    },
    VcJoin {
        channel: String,
        user: String,
    },
    RateLimit {
        route: String,
        retry_after_ms: u64,
    },
}

#[cfg(test)]
mod tests {
    use super::{DeckEvent, LogLevel, LogLine};

    #[test]
    fn log_event_round_trips_json() {
        let event = DeckEvent::Log(LogLine {
            ts: "14:02:01".to_string(),
            level: LogLevel::Info,
            source: "simverse".to_string(),
            text: "seed loaded".to_string(),
        });

        let json = serde_json::to_string(&event).expect("event should serialize");
        let decoded: DeckEvent = serde_json::from_str(&json).expect("event should deserialize");

        assert_eq!(decoded, event);
    }
}
