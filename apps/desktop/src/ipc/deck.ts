import { deckFunctions } from "../data/functions";

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

export type DeckEventPayload = LogLine;

export type ModuleConfig = {
  capture_window_minutes?: number;
  channel_scope?: string;
  storage_mode?: string;
  redaction?: boolean;
};

export async function modulesList(): Promise<ModuleSummary[]> {
  const tauri = await resolveTauriCore();
  if (!tauri) {
    return deckFunctions;
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

export async function simSetIntensity(intensity: number): Promise<SimStatus | null> {
  const tauri = await resolveTauriCore();
  if (!tauri) {
    return null;
  }

  return tauri.invoke<SimStatus>("sim_set_intensity", { intensity });
}

export async function logsRecent(limit = 12): Promise<LogLine[]> {
  const tauri = await resolveTauriCore();
  if (!tauri) {
    return [];
  }

  return tauri.invoke<LogLine[]>("logs_recent", { limit });
}

export async function modulesSetEnabled(id: string, enabled: boolean): Promise<void> {
  const tauri = await resolveTauriCore();
  if (!tauri) {
    return;
  }

  await tauri.invoke<void>("modules_set_enabled", { id, enabled });
}

export async function modulesGetConfig(id: string): Promise<ModuleConfig> {
  const tauri = await resolveTauriCore();
  if (!tauri) {
    return {};
  }

  return tauri.invoke<ModuleConfig>("modules_get_config", { id });
}

export async function modulesSetConfig(id: string, config: ModuleConfig): Promise<ModuleConfig> {
  const tauri = await resolveTauriCore();
  if (!tauri) {
    return config;
  }

  return tauri.invoke<ModuleConfig>("modules_set_config", { id, config });
}

export async function modulesTriggerTest(id: string): Promise<LogLine | null> {
  const tauri = await resolveTauriCore();
  if (!tauri) {
    return null;
  }

  return tauri.invoke<LogLine>("modules_trigger_test", { id });
}

export async function listenDeckEvents(onEvent: (event: DeckEventPayload) => void): Promise<() => void> {
  const tauriEvent = await resolveTauriEvent();
  if (tauriEvent) {
    return tauriEvent.listen<DeckEventPayload>("deck:event", (event) => onEvent(event.payload));
  }

  const handler = (event: Event) => {
    onEvent((event as CustomEvent<DeckEventPayload>).detail);
  };
  window.addEventListener("deck:event", handler);
  return () => window.removeEventListener("deck:event", handler);
}

async function resolveTauriCore(): Promise<null | { invoke<T>(command: string, args?: Record<string, unknown>): Promise<T> }> {
  if (!("__TAURI_INTERNALS__" in window)) {
    return null;
  }

  return import("@tauri-apps/api/core");
}

async function resolveTauriEvent(): Promise<null | {
  listen<T>(event: string, handler: (event: { payload: T }) => void): Promise<() => void>;
}> {
  if (!("__TAURI_INTERNALS__" in window)) {
    return null;
  }

  return import("@tauri-apps/api/event");
}
