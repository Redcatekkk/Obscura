import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const target = join(root, "packages", "ipc-types", "src", "index.ts");

const contents = `// Generated IPC contract for obscura.deck.
// Source: scripts/generate-ipc-types.mjs.

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
`;

if (process.argv.includes("--check")) {
  const current = readFileSync(target, "utf8");
  if (current !== contents) {
    console.error("Generated IPC types are out of date. Run `pnpm generate:ipc-types`.");
    process.exit(1);
  }
  console.log("Generated IPC types are up to date.");
} else {
  writeFileSync(target, contents);
}
