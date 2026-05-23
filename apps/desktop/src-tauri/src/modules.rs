use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Category {
    Automation,
    Utility,
    Privacy,
    Stats,
    Fun,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleSummary {
    pub id: &'static str,
    pub name: &'static str,
    pub category: Category,
    pub description: &'static str,
    pub enabled: bool,
}

pub fn reference_modules() -> Vec<ModuleSummary> {
    vec![
        ModuleSummary {
            id: "f01",
            name: "Message Sniper",
            category: Category::Automation,
            description: "Caches deleted simverse messages with channel and author metadata.",
            enabled: true,
        },
        ModuleSummary {
            id: "f02",
            name: "Regex Auto-Reply",
            category: Category::Automation,
            description: "Matches simulated DM traffic and queues safe operator replies.",
            enabled: true,
        },
        ModuleSummary {
            id: "f03",
            name: "Status Rotator",
            category: Category::Automation,
            description: "Cycles local presence states on a scheduler cadence.",
            enabled: false,
        },
        ModuleSummary {
            id: "f04",
            name: "Reaction Matrix",
            category: Category::Automation,
            description: "Applies configured reaction sets to matching simulated events.",
            enabled: false,
        },
        ModuleSummary {
            id: "f05",
            name: "DM Cleaner",
            category: Category::Utility,
            description: "Purges local mock history from the current simulation session.",
            enabled: false,
        },
        ModuleSummary {
            id: "f06",
            name: "Server Clone",
            category: Category::Utility,
            description: "Snapshots guild channels, roles, and permission maps into JSON.",
            enabled: true,
        },
        ModuleSummary {
            id: "f07",
            name: "Invite Scanner",
            category: Category::Utility,
            description: "Extracts invite-like strings from generated message streams.",
            enabled: false,
        },
        ModuleSummary {
            id: "f08",
            name: "Token Guard",
            category: Category::Utility,
            description: "Rejects user-token shaped values before they reach any adapter.",
            enabled: true,
        },
        ModuleSummary {
            id: "f09",
            name: "Anti-Tracker",
            category: Category::Privacy,
            description: "Suppresses read receipts and typing signals in the simulator.",
            enabled: true,
        },
        ModuleSummary {
            id: "f10",
            name: "Stealth Mode",
            category: Category::Privacy,
            description: "Removes the operator from mutual visibility computations.",
            enabled: false,
        },
        ModuleSummary {
            id: "f11",
            name: "Self-Destruct",
            category: Category::Privacy,
            description: "Expires outbound simulated messages after a configured delay.",
            enabled: false,
        },
        ModuleSummary {
            id: "f12",
            name: "Link Scanner",
            category: Category::Privacy,
            description: "Flags suspicious URLs before local browser handoff.",
            enabled: true,
        },
        ModuleSummary {
            id: "f13",
            name: "Guild Tracker",
            category: Category::Stats,
            description: "Charts joins, leaves, deletes, and rate-limit pulses.",
            enabled: true,
        },
        ModuleSummary {
            id: "f14",
            name: "Keyword Alerts",
            category: Category::Stats,
            description: "Raises desktop-safe alerts for watched terms in sim logs.",
            enabled: false,
        },
        ModuleSummary {
            id: "f15",
            name: "VC Time Logger",
            category: Category::Stats,
            description: "Accumulates simulated voice-channel dwell time per member.",
            enabled: true,
        },
        ModuleSummary {
            id: "f16",
            name: "Ping Analyzer",
            category: Category::Stats,
            description: "Builds a frequency map of simulated mentions and replies.",
            enabled: false,
        },
        ModuleSummary {
            id: "f17",
            name: "Text to Mock",
            category: Category::Fun,
            description: "Transforms local draft text into alternating-case output.",
            enabled: false,
        },
        ModuleSummary {
            id: "f18",
            name: "Ghost Ping",
            category: Category::Fun,
            description: "Emits and retracts simulated mention events for testing.",
            enabled: false,
        },
        ModuleSummary {
            id: "f19",
            name: "Fake Typing",
            category: Category::Fun,
            description: "Broadcasts local typing indicators inside the simverse.",
            enabled: true,
        },
        ModuleSummary {
            id: "f20",
            name: "Theme Forge",
            category: Category::Utility,
            description: "Swaps CSS variables and records theme presets locally.",
            enabled: true,
        },
        ModuleSummary {
            id: "f21",
            name: "Presence Ledger",
            category: Category::Stats,
            description: "Records simulated online, idle, and offline state transitions.",
            enabled: false,
        },
        ModuleSummary {
            id: "f22",
            name: "Channel Heatmap",
            category: Category::Stats,
            description: "Aggregates local event density by simulated channel.",
            enabled: false,
        },
        ModuleSummary {
            id: "f23",
            name: "Mention Sentinel",
            category: Category::Stats,
            description: "Tracks mention-like events and ranks them by source.",
            enabled: false,
        },
        ModuleSummary {
            id: "f24",
            name: "VC Pie Chart",
            category: Category::Stats,
            description: "Summarizes simulated voice dwell time by channel.",
            enabled: false,
        },
        ModuleSummary {
            id: "f25",
            name: "Role Watch",
            category: Category::Utility,
            description: "Detects simulated role and permission changes.",
            enabled: false,
        },
        ModuleSummary {
            id: "f26",
            name: "Emoji Vault",
            category: Category::Fun,
            description: "Indexes simulated emoji usage for local previews.",
            enabled: false,
        },
        ModuleSummary {
            id: "f27",
            name: "Thread Tracker",
            category: Category::Utility,
            description: "Observes simulated thread creation and archival events.",
            enabled: false,
        },
        ModuleSummary {
            id: "f28",
            name: "Attachment Mirror",
            category: Category::Utility,
            description: "Stores local metadata for simulated attachment events.",
            enabled: false,
        },
        ModuleSummary {
            id: "f29",
            name: "Webhook Watch",
            category: Category::Utility,
            description: "Flags webhook-shaped messages inside the simverse.",
            enabled: false,
        },
        ModuleSummary {
            id: "f30",
            name: "Quiet Hours",
            category: Category::Privacy,
            description: "Suppresses noncritical local alerts during configured windows.",
            enabled: false,
        },
        ModuleSummary {
            id: "f31",
            name: "Panic Mute",
            category: Category::Privacy,
            description: "Disables noisy simulated reactions and typing indicators.",
            enabled: false,
        },
        ModuleSummary {
            id: "f32",
            name: "Redaction Rules",
            category: Category::Privacy,
            description: "Applies local masking rules to saved simulator logs.",
            enabled: false,
        },
        ModuleSummary {
            id: "f33",
            name: "Local Archive",
            category: Category::Utility,
            description: "Exports selected simulator messages into local JSON bundles.",
            enabled: false,
        },
        ModuleSummary {
            id: "f34",
            name: "Replay Queue",
            category: Category::Automation,
            description: "Replays deterministic simverse events for module testing.",
            enabled: false,
        },
        ModuleSummary {
            id: "f35",
            name: "Rate Limit Lab",
            category: Category::Automation,
            description: "Stress-tests module behavior against simulated cooldown events.",
            enabled: false,
        },
        ModuleSummary {
            id: "f36",
            name: "Noise Injector",
            category: Category::Automation,
            description: "Adds synthetic chatter to deterministic simulation runs.",
            enabled: false,
        },
        ModuleSummary {
            id: "f37",
            name: "Desktop Toast",
            category: Category::Utility,
            description: "Raises OS-safe notifications from selected local events.",
            enabled: false,
        },
        ModuleSummary {
            id: "f38",
            name: "Webhook Bridge",
            category: Category::Utility,
            description: "Posts selected simulator events to a user-provided webhook URL.",
            enabled: false,
        },
        ModuleSummary {
            id: "f39",
            name: "Command Palette",
            category: Category::Utility,
            description: "Indexes deck actions for fast keyboard access.",
            enabled: false,
        },
        ModuleSummary {
            id: "f40",
            name: "Theme Scheduler",
            category: Category::Fun,
            description: "Cycles local visual presets on a timed cadence.",
            enabled: false,
        },
        ModuleSummary {
            id: "f41",
            name: "Palette Forge",
            category: Category::Fun,
            description: "Generates deck color presets from local theme tokens.",
            enabled: false,
        },
        ModuleSummary {
            id: "f42",
            name: "Alias Lab",
            category: Category::Fun,
            description: "Creates simulated nicknames for local identity testing.",
            enabled: false,
        },
        ModuleSummary {
            id: "f43",
            name: "Persona Switcher",
            category: Category::Fun,
            description: "Swaps local operator personas inside simulator output.",
            enabled: false,
        },
        ModuleSummary {
            id: "f44",
            name: "Audit Trail",
            category: Category::Stats,
            description: "Builds an append-only activity trail for deck actions.",
            enabled: false,
        },
        ModuleSummary {
            id: "f45",
            name: "Health Probe",
            category: Category::Stats,
            description: "Samples local queue depth and command latency.",
            enabled: false,
        },
        ModuleSummary {
            id: "f46",
            name: "Snapshot Diff",
            category: Category::Utility,
            description: "Compares two local guild snapshots for structural changes.",
            enabled: false,
        },
        ModuleSummary {
            id: "f47",
            name: "Import Wizard",
            category: Category::Utility,
            description: "Validates simulator bundles before local import.",
            enabled: false,
        },
        ModuleSummary {
            id: "f48",
            name: "Guild Snapshot",
            category: Category::Utility,
            description: "Exports a gzipped local snapshot of the simulated guild tree.",
            enabled: false,
        },
        ModuleSummary {
            id: "f49",
            name: "Backup Verifier",
            category: Category::Utility,
            description: "Checks exported simulator snapshots for schema drift.",
            enabled: false,
        },
        ModuleSummary {
            id: "f50",
            name: "Settings Sync",
            category: Category::Utility,
            description: "Exports and imports all local module settings as JSON.",
            enabled: false,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::reference_modules;

    #[test]
    fn reference_modules_have_required_ids() {
        let ids: Vec<&str> = reference_modules()
            .into_iter()
            .map(|module| module.id)
            .collect();

        assert_eq!(ids.len(), 50);
        assert_eq!(ids.first(), Some(&"f01"));
        assert_eq!(ids.last(), Some(&"f50"));
        assert!(ids.contains(&"f15"));
        assert!(ids.contains(&"f37"));
        assert!(ids.contains(&"f48"));
    }
}
