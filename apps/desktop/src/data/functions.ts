export type Category = "Automation" | "Utility" | "Privacy" | "Stats" | "Fun";

export type DeckFunction = {
  id: string;
  name: string;
  category: Category;
  description: string;
  enabled: boolean;
};

export const categories: Category[] = ["Automation", "Utility", "Privacy", "Stats", "Fun"];

export const deckFunctions: DeckFunction[] = [
  { id: "f01", name: "Message Sniper", category: "Automation", description: "Caches deleted simverse messages with channel and author metadata.", enabled: true },
  { id: "f02", name: "Regex Auto-Reply", category: "Automation", description: "Matches simulated DM traffic and queues safe operator replies.", enabled: true },
  { id: "f03", name: "Status Rotator", category: "Automation", description: "Cycles local presence states on a scheduler cadence.", enabled: false },
  { id: "f04", name: "Reaction Matrix", category: "Automation", description: "Applies configured reaction sets to matching simulated events.", enabled: false },
  { id: "f05", name: "DM Cleaner", category: "Utility", description: "Purges local mock history from the current simulation session.", enabled: false },
  { id: "f06", name: "Server Clone", category: "Utility", description: "Snapshots guild channels, roles, and permission maps into JSON.", enabled: true },
  { id: "f07", name: "Invite Scanner", category: "Utility", description: "Extracts invite-like strings from generated message streams.", enabled: false },
  { id: "f08", name: "Token Guard", category: "Utility", description: "Rejects user-token shaped values before they reach any adapter.", enabled: true },
  { id: "f09", name: "Anti-Tracker", category: "Privacy", description: "Suppresses read receipts and typing signals in the simulator.", enabled: true },
  { id: "f10", name: "Stealth Mode", category: "Privacy", description: "Removes the operator from mutual visibility computations.", enabled: false },
  { id: "f11", name: "Self-Destruct", category: "Privacy", description: "Expires outbound simulated messages after a configured delay.", enabled: false },
  { id: "f12", name: "Link Scanner", category: "Privacy", description: "Flags suspicious URLs before local browser handoff.", enabled: true },
  { id: "f13", name: "Guild Tracker", category: "Stats", description: "Charts joins, leaves, deletes, and rate-limit pulses.", enabled: true },
  { id: "f14", name: "Keyword Alerts", category: "Stats", description: "Raises desktop-safe alerts for watched terms in sim logs.", enabled: false },
  { id: "f15", name: "VC Time Logger", category: "Stats", description: "Accumulates simulated voice-channel dwell time per member.", enabled: true },
  { id: "f16", name: "Ping Analyzer", category: "Stats", description: "Builds a frequency map of simulated mentions and replies.", enabled: false },
  { id: "f17", name: "Text to Mock", category: "Fun", description: "Transforms local draft text into alternating-case output.", enabled: false },
  { id: "f18", name: "Ghost Ping", category: "Fun", description: "Emits and retracts simulated mention events for testing.", enabled: false },
  { id: "f19", name: "Fake Typing", category: "Fun", description: "Broadcasts local typing indicators inside the simverse.", enabled: true },
  { id: "f20", name: "Theme Forge", category: "Utility", description: "Swaps CSS variables and records theme presets locally.", enabled: true },
  { id: "f21", name: "Presence Ledger", category: "Stats", description: "Records simulated online, idle, and offline state transitions.", enabled: false },
  { id: "f22", name: "Channel Heatmap", category: "Stats", description: "Aggregates local event density by simulated channel.", enabled: false },
  { id: "f23", name: "Mention Sentinel", category: "Stats", description: "Tracks mention-like events and ranks them by source.", enabled: false },
  { id: "f24", name: "VC Pie Chart", category: "Stats", description: "Summarizes simulated voice dwell time by channel.", enabled: false },
  { id: "f25", name: "Role Watch", category: "Utility", description: "Detects simulated role and permission changes.", enabled: false },
  { id: "f26", name: "Emoji Vault", category: "Fun", description: "Indexes simulated emoji usage for local previews.", enabled: false },
  { id: "f27", name: "Thread Tracker", category: "Utility", description: "Observes simulated thread creation and archival events.", enabled: false },
  { id: "f28", name: "Attachment Mirror", category: "Utility", description: "Stores local metadata for simulated attachment events.", enabled: false },
  { id: "f29", name: "Webhook Watch", category: "Utility", description: "Flags webhook-shaped messages inside the simverse.", enabled: false },
  { id: "f30", name: "Quiet Hours", category: "Privacy", description: "Suppresses noncritical local alerts during configured windows.", enabled: false },
  { id: "f31", name: "Panic Mute", category: "Privacy", description: "Disables noisy simulated reactions and typing indicators.", enabled: false },
  { id: "f32", name: "Redaction Rules", category: "Privacy", description: "Applies local masking rules to saved simulator logs.", enabled: false },
  { id: "f33", name: "Local Archive", category: "Utility", description: "Exports selected simulator messages into local JSON bundles.", enabled: false },
  { id: "f34", name: "Replay Queue", category: "Automation", description: "Replays deterministic simverse events for module testing.", enabled: false },
  { id: "f35", name: "Rate Limit Lab", category: "Automation", description: "Stress-tests module behavior against simulated cooldown events.", enabled: false },
  { id: "f36", name: "Noise Injector", category: "Automation", description: "Adds synthetic chatter to deterministic simulation runs.", enabled: false },
  { id: "f37", name: "Desktop Toast", category: "Utility", description: "Raises OS-safe notifications from selected local events.", enabled: false },
  { id: "f38", name: "Webhook Bridge", category: "Utility", description: "Posts selected simulator events to a user-provided webhook URL.", enabled: false },
  { id: "f39", name: "Command Palette", category: "Utility", description: "Indexes deck actions for fast keyboard access.", enabled: false },
  { id: "f40", name: "Theme Scheduler", category: "Fun", description: "Cycles local visual presets on a timed cadence.", enabled: false },
  { id: "f41", name: "Palette Forge", category: "Fun", description: "Generates deck color presets from local theme tokens.", enabled: false },
  { id: "f42", name: "Alias Lab", category: "Fun", description: "Creates simulated nicknames for local identity testing.", enabled: false },
  { id: "f43", name: "Persona Switcher", category: "Fun", description: "Swaps local operator personas inside simulator output.", enabled: false },
  { id: "f44", name: "Audit Trail", category: "Stats", description: "Builds an append-only activity trail for deck actions.", enabled: false },
  { id: "f45", name: "Health Probe", category: "Stats", description: "Samples local queue depth and command latency.", enabled: false },
  { id: "f46", name: "Snapshot Diff", category: "Utility", description: "Compares two local guild snapshots for structural changes.", enabled: false },
  { id: "f47", name: "Import Wizard", category: "Utility", description: "Validates simulator bundles before local import.", enabled: false },
  { id: "f48", name: "Guild Snapshot", category: "Utility", description: "Exports a gzipped local snapshot of the simulated guild tree.", enabled: false },
  { id: "f49", name: "Backup Verifier", category: "Utility", description: "Checks exported simulator snapshots for schema drift.", enabled: false },
  { id: "f50", name: "Settings Sync", category: "Utility", description: "Exports and imports all local module settings as JSON.", enabled: false }
];

export function moduleIds() {
  return deckFunctions.map((module) => module.id);
}

export function enabledModuleCount() {
  return deckFunctions.filter((module) => module.enabled).length;
}

export function categoryCount(category: Category) {
  return deckFunctions.filter((module) => module.category === category).length;
}

export function getModuleById(id: string) {
  return deckFunctions.find((module) => module.id === id);
}

export const moduleSeedCount = deckFunctions.length;
