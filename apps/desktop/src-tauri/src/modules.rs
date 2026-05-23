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
            id: "f17",
            name: "Stealth Mode",
            category: Category::Privacy,
            description: "Removes the operator from mutual visibility computations.",
            enabled: false,
        },
        ModuleSummary {
            id: "f23",
            name: "VC Time Logger",
            category: Category::Stats,
            description: "Accumulates simulated voice-channel dwell time per member.",
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

        assert_eq!(ids, vec!["f01", "f02", "f03", "f17", "f23"]);
    }
}
