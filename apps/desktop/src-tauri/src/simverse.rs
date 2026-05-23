use crate::bus::SimEvent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Simverse {
    seed: u64,
}

impl Simverse {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn sample_events(&self) -> Vec<SimEvent> {
        let suffix = self.seed % 100;
        vec![
            SimEvent::MessageCreate {
                channel: format!("ops-{suffix:02}"),
                author: "nullbyte.operator".to_string(),
                content: "gn from simverse".to_string(),
            },
            SimEvent::MessageDelete {
                channel: format!("ops-{suffix:02}"),
                author: "ghost.user".to_string(),
                content: "cache this deletion".to_string(),
            },
            SimEvent::MemberJoin {
                guild: "blacksite".to_string(),
                user: format!("member-{suffix:02}"),
            },
            SimEvent::VcJoin {
                channel: "voice-ops".to_string(),
                user: format!("member-{suffix:02}"),
            },
            SimEvent::RateLimit {
                route: "/sim/messages".to_string(),
                retry_after_ms: 250 + suffix,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::Simverse;

    #[test]
    fn sample_events_are_deterministic_for_seed() {
        let first = Simverse::new(42).sample_events();
        let second = Simverse::new(42).sample_events();

        assert_eq!(first, second);
        assert_eq!(first.len(), 5);
    }
}
