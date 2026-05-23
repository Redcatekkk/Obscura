use crate::bus::{LogLevel, LogLine, SimEvent};

pub fn run_reference_modules(events: &[SimEvent]) -> Vec<LogLine> {
    let mut logs = vec![
        LogLine {
            ts: "14:02:00".to_string(),
            level: LogLevel::Info,
            source: "f03.status_rotator".to_string(),
            text: "cycled simulated presence to OPERATOR_ACTIVE".to_string(),
        },
        LogLine {
            ts: "14:02:00".to_string(),
            level: LogLevel::Info,
            source: "f10.stealth_mode".to_string(),
            text: "operator visibility hidden from mutuals".to_string(),
        },
        LogLine {
            ts: "14:02:00".to_string(),
            level: LogLevel::Info,
            source: "f15.vc_time_logger".to_string(),
            text: "voice-channel timers armed for simulated dwell tracking".to_string(),
        },
        LogLine {
            ts: "14:02:00".to_string(),
            level: LogLevel::Info,
            source: "f20.theme_forge".to_string(),
            text: "loaded preset chrome theme tokens".to_string(),
        },
    ];

    for event in events {
        match event {
            SimEvent::MessageDelete {
                channel,
                author,
                content,
            } => logs.push(LogLine {
                ts: "14:02:01".to_string(),
                level: LogLevel::Info,
                source: "f01.message_sniper".to_string(),
                text: format!("cached deleted message from {author} in #{channel}: {content}"),
            }),
            SimEvent::MessageCreate {
                channel,
                author,
                content,
            } if content.to_lowercase().contains("gn") => logs.push(LogLine {
                ts: "14:02:02".to_string(),
                level: LogLevel::Info,
                source: "f02.regex_auto_reply".to_string(),
                text: format!("matched /gn/i from {author} in #{channel}; queued safe reply"),
            }),
            SimEvent::MemberJoin { guild, user } => logs.push(LogLine {
                ts: "14:02:03".to_string(),
                level: LogLevel::Warn,
                source: "f10.stealth_mode".to_string(),
                text: format!("hid operator mutuals from {user} in {guild}"),
            }),
            SimEvent::VcJoin { channel, user } => logs.push(LogLine {
                ts: "14:02:04".to_string(),
                level: LogLevel::Info,
                source: "f15.vc_time_logger".to_string(),
                text: format!("started VC timer for {user} in {channel}"),
            }),
            SimEvent::RateLimit {
                route,
                retry_after_ms,
            } => logs.push(LogLine {
                ts: "14:02:05".to_string(),
                level: LogLevel::Warn,
                source: "simverse.ratelimit".to_string(),
                text: format!("{route} requested cooldown for {retry_after_ms}ms"),
            }),
            _ => {}
        }
    }

    logs
}

#[cfg(test)]
mod tests {
    use crate::{engine::run_reference_modules, simverse::Simverse};

    #[test]
    fn reference_modules_emit_expected_logs_from_sample_events() {
        let events = Simverse::new(42).sample_events();
        let logs = run_reference_modules(&events);
        let sources: Vec<&str> = logs.iter().map(|log| log.source.as_str()).collect();

        assert!(sources.contains(&"f01.message_sniper"));
        assert!(sources.contains(&"f02.regex_auto_reply"));
        assert!(sources.contains(&"f03.status_rotator"));
        assert!(sources.contains(&"f10.stealth_mode"));
        assert!(sources.contains(&"f15.vc_time_logger"));
        assert!(sources.contains(&"f20.theme_forge"));
    }
}
