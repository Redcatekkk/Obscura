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

export async function modulesList(): Promise<ModuleSummary[]> {
  const tauri = await resolveTauriCore();
  if (!tauri) {
    return [];
  }

  return tauri.invoke<ModuleSummary[]>("modules_list");
}

export async function simStatus(): Promise<SimStatus | null> {
  const tauri = await resolveTauriCore();
  if (!tauri) {
    return null;
  }

  return tauri.invoke<SimStatus>("sim_status");
}

export async function logsRecent(limit = 12): Promise<LogLine[]> {
  const tauri = await resolveTauriCore();
  if (!tauri) {
    return [];
  }

  return tauri.invoke<LogLine[]>("logs_recent", { limit });
}

async function resolveTauriCore(): Promise<null | { invoke<T>(command: string, args?: Record<string, unknown>): Promise<T> }> {
  if (!("__TAURI_INTERNALS__" in window)) {
    return null;
  }

  return import("@tauri-apps/api/core");
}
