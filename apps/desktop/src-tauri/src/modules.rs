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

        assert_eq!(ids.len(), 20);
        assert_eq!(ids.first(), Some(&"f01"));
        assert_eq!(ids.last(), Some(&"f20"));
        assert!(ids.contains(&"f15"));
    }
}
