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
  { id: "f20", name: "Theme Forge", category: "Utility", description: "Swaps CSS variables and records theme presets locally.", enabled: true }
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
