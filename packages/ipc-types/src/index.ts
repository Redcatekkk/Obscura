// Generated IPC contract for obscura.deck.
// Keep in sync with apps/desktop/src-tauri/src/{bus,commands,modules}.rs.

export type DeckCategory = "Automation" | "Utility" | "Privacy" | "Stats" | "Fun";

export type ModuleSummary = {
  id: string;
  name: string;
  category: DeckCategory;
  description: string;
  enabled: boolean;
};

export type SimStatus = {
  seed: number;
  intensity: number;
  event_count: number;
};

export type LogLevel = "Info" | "Warn" | "Error";

export type LogLine = {
  ts: string;
  level: LogLevel;
  source: string;
  text: string;
};

export type ModuleConfig = {
  capture_window_minutes?: number;
  channel_scope?: string;
  storage_mode?: string;
  redaction?: boolean;
};

export type DeckEventPayload = LogLine;

export type Commands = {
  logs_recent: {
    args: { limit?: number };
    result: LogLine[];
  };
  modules_get_config: {
    args: { id: string };
    result: ModuleConfig;
  };
  modules_list: {
    args: Record<string, never>;
    result: ModuleSummary[];
  };
  modules_set_config: {
    args: { id: string; config: ModuleConfig };
    result: ModuleConfig;
  };
  modules_set_enabled: {
    args: { id: string; enabled: boolean };
    result: null;
  };
  modules_trigger_test: {
    args: { id: string };
    result: LogLine;
  };
  sim_set_intensity: {
    args: { intensity: number };
    result: SimStatus;
  };
  sim_status: {
    args: Record<string, never>;
    result: SimStatus;
  };
};
