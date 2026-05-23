import { useEffect, useMemo, useState } from "react";
import {
  Activity,
  Bell,
  Bot,
  Boxes,
  Gauge,
  Lock,
  Network,
  Radio,
  Search,
  Settings,
  Shield,
  SlidersHorizontal,
  Sparkles,
  Terminal,
  Zap
} from "lucide-react";
import type { LucideIcon } from "lucide-react";
import {
  listenDeckEvents,
  logsRecent,
  modulesList,
  modulesSetEnabled,
  modulesTriggerTest,
  simSetIntensity,
  simStatus,
  type LogLine,
  type ModuleSummary
} from "./ipc/deck";
import {
  categories,
  deckFunctions,
  type Category,
  type DeckFunction
} from "./data/functions";

const navItems: Array<[string, LucideIcon]> = [
  ["Control", Gauge],
  ["Library", Boxes],
  ["Simverse", Network],
  ["Privacy", Shield],
  ["Logs", Terminal],
  ["Settings", Settings]
];

const kpis = [
  { label: "Active Modules", value: "09", detail: "of 50 planned" },
  { label: "Sim Events", value: "1.8k", detail: "last hour" },
  { label: "Latency", value: "14ms", detail: "local bus" },
  { label: "Uptime", value: "03:42:19", detail: "operator session" }
];

function Toggle({ enabled, label, onToggle }: { enabled: boolean; label: string; onToggle: () => void }) {
  return (
    <button
      className={`toggle ${enabled ? "is-on" : ""}`}
      type="button"
      onClick={onToggle}
      aria-pressed={enabled}
      aria-label={`${label} ${enabled ? "enabled" : "disabled"}`}
      data-testid={`toggle-${label.toLowerCase().replace(/\s+/g, "-")}`}
    >
      <span />
    </button>
  );
}

export default function App() {
  const [activeCategory, setActiveCategory] = useState<Category | "All">("All");
  const [query, setQuery] = useState("");
  const [modalOpen, setModalOpen] = useState(false);
  const [modules, setModules] = useState<DeckFunction[]>(deckFunctions);
  const [simSeed, setSimSeed] = useState<number | null>(null);
  const [simIntensity, setSimIntensity] = useState(50);
  const [logs, setLogs] = useState<LogLine[]>([]);

  const filteredFunctions = useMemo(() => {
    const normalized = query.trim().toLowerCase();
    return modules.filter((item) => {
      const categoryMatch = activeCategory === "All" || item.category === activeCategory;
      const queryMatch =
        normalized.length === 0 ||
        item.name.toLowerCase().includes(normalized) ||
        item.description.toLowerCase().includes(normalized) ||
        item.category.toLowerCase().includes(normalized);
      return categoryMatch && queryMatch;
    });
  }, [activeCategory, modules, query]);

  useEffect(() => {
    let mounted = true;

    void modulesList().then((items: ModuleSummary[]) => {
      if (!mounted || items.length === 0) {
        return;
      }

      setModules(
        items.map((item) => ({
          ...item,
          category: item.category,
          enabled: item.enabled
        }))
      );
    });

    void simStatus().then((status) => {
      if (mounted && status) {
        setSimSeed(status.seed);
        setSimIntensity(status.intensity);
      }
    });

    void logsRecent(12).then((items) => {
      if (mounted && items.length > 0) {
        setLogs(items);
      }
    });

    return () => {
      mounted = false;
    };
  }, []);

  useEffect(() => {
    let unlisten: null | (() => void) = null;
    let mounted = true;

    void listenDeckEvents((line) => {
      if (mounted) {
        setLogs((current) => [line, ...current].slice(0, 12));
      }
    }).then((cleanup) => {
      unlisten = cleanup;
      if (!mounted) {
        cleanup();
      }
    });

    return () => {
      mounted = false;
      unlisten?.();
    };
  }, []);

  function toggleModule(id: string, enabled: boolean) {
    setModules((current) =>
      current.map((module) => (module.id === id ? { ...module, enabled: !enabled } : module))
    );
    void modulesSetEnabled(id, !enabled);
  }

  function triggerModuleTest(id: string) {
    void modulesTriggerTest(id).then((line) => {
      if (line) {
        setLogs((current) => [line, ...current].slice(0, 12));
      }
    });
  }

  function toggleSimIntensity() {
    const nextIntensity = simIntensity > 0 ? 0 : 50;
    setSimIntensity(nextIntensity);
    void simSetIntensity(nextIntensity).then((status) => {
      if (status) {
        setSimIntensity(status.intensity);
        setSimSeed(status.seed);
      }
    });
  }

  const simEngaged = simIntensity > 0;

  return (
    <main className="app-shell" aria-label="obscura deck workspace" data-testid="app-shell">
      <div className="scanlines" aria-hidden="true" />
      <aside className="sidebar" data-testid="sidebar">
        <div className="brand-block">
          <div className="brand-mark">Ø</div>
          <div>
            <p>NULLBYTE</p>
            <h1>obscura.deck</h1>
          </div>
        </div>
        <nav className="nav-stack" aria-label="Deck sections">
          {navItems.map(([label, Icon]) => (
            <button className={label === "Library" ? "nav-item is-active" : "nav-item"} key={label} type="button">
              <Icon size={16} />
              {label}
            </button>
          ))}
        </nav>
        <section className="disclaimer-card" data-testid="disclaimer-card">
          <Lock size={16} />
          <strong>Simulation only</strong>
          <p>No real Discord automation. No user-token auth, scraping, or self-bot behavior.</p>
        </section>
      </aside>

      <section className="deck-canvas">
        <header className="topbar" data-testid="master-control">
          <div>
              <p className="overline">MASTER CONTROL / LOCAL SIMVERSE</p>
            <h2>Operator deck armed for dry-run telemetry{simSeed !== null ? ` / seed ${simSeed}` : ""}.</h2>
          </div>
          <div className="command-row">
            <label className="search-box" data-testid="search-box">
              <Search size={16} />
              <input
                aria-label="Search functions"
                onChange={(event) => setQuery(event.target.value)}
                placeholder="Search modules..."
                value={query}
              />
              <kbd>Ctrl K</kbd>
            </label>
            <button
              aria-pressed={simEngaged}
              className="engage-button"
              onClick={toggleSimIntensity}
              type="button"
              data-testid="engage-button"
            >
              <Zap size={16} />
              {simEngaged ? "ENGAGED" : "DISENGAGED"}
            </button>
          </div>
        </header>

        <section className="hero-grid">
          <div className="operator-panel panel">
            <div className="avatar-frame">
              <Bot size={44} />
            </div>
            <div>
              <p className="overline">OPERATOR PROFILE</p>
              <h3>Red Team Console</h3>
              <p>Local-only command surface with simulated message, guild, voice, and privacy events.</p>
            </div>
            <div className="status-strip">
              <span className={simEngaged ? "pulse-dot" : "idle-dot"} />
              {simEngaged ? "SIM ONLINE" : "SIM IDLE"}
            </div>
          </div>

          <div className="kpi-grid" data-testid="kpi-row">
            {kpis.map((kpi) => (
              <article className="kpi panel" key={kpi.label}>
                <span>{kpi.label}</span>
                <strong>{kpi.value}</strong>
                <small>{kpi.detail}</small>
              </article>
            ))}
          </div>
        </section>

        <div className="content-grid">
          <section className="library-panel panel" data-testid="function-library">
            <div className="section-heading">
              <div>
                <p className="overline">FUNCTION LIBRARY</p>
                <h3>Static module matrix</h3>
              </div>
              <button className="ghost-button" onClick={() => setModalOpen(true)} type="button" data-testid="open-config">
                <SlidersHorizontal size={16} />
                Configure f01
              </button>
            </div>

            <div className="category-row" data-testid="category-filter">
              {(["All", ...categories] as const).map((category) => (
                <button
                  className={activeCategory === category ? "category-chip is-active" : "category-chip"}
                  key={category}
                  onClick={() => setActiveCategory(category)}
                  type="button"
                >
                  {category}
                </button>
              ))}
            </div>

            <div className="function-grid">
              {filteredFunctions.map((item) => (
                <article className="function-card" data-testid={`function-card-${item.id}`} key={item.id}>
                  <div className="card-topline">
                    <span className="module-id">{item.id}</span>
                    <span className={`category-tag category-${item.category.toLowerCase()}`}>{item.category}</span>
                  </div>
                  <div className="card-title-row">
                    <h4>{item.name}</h4>
                    {item.enabled ? <span className="pulse-dot" aria-label="active module" /> : <span className="idle-dot" />}
                  </div>
                  <p>{item.description}</p>
                  <div className="card-footer">
                    <span>{item.enabled ? "ACTIVE BUS" : "STANDBY"}</span>
                    <button className="test-button" onClick={() => triggerModuleTest(item.id)} type="button">
                      TEST
                    </button>
                    <Toggle enabled={item.enabled} label={item.name} onToggle={() => toggleModule(item.id, item.enabled)} />
                  </div>
                </article>
              ))}
            </div>
          </section>

          <section className="right-rail">
            <div className="panel signal-panel" data-testid="signal-panel">
              <div className="section-heading compact">
                <div>
                  <p className="overline">SIGNAL MAP</p>
                  <h3>Live channels</h3>
                </div>
                <Radio size={18} />
              </div>
              {[
                ["Guild delta", "72%"],
                ["VC stream", "48%"],
                ["Privacy rail", "91%"]
              ].map(([label, value]) => (
                <div className="meter" key={label}>
                  <span>{label}</span>
                  <div><i style={{ width: value }} /></div>
                  <b>{value}</b>
                </div>
              ))}
            </div>

            <div className="panel terminal-panel" data-testid="terminal-feed">
              <div className="section-heading compact">
                <div>
                  <p className="overline">SYSLOG</p>
                  <h3>Terminal feed</h3>
                </div>
                <Activity size={18} />
              </div>
              <div className="terminal-lines">
                {(logs.length > 0
                  ? logs
                  : [
                      { ts: "14:02:01", level: "Info", source: "seed", text: "simverse seed loaded" },
                      { ts: "14:02:04", level: "Warn", source: "seed", text: "awaiting live log stream" }
                    ]
                ).map((line) => (
                  <p key={`${line.ts}-${line.source}-${line.text}`}>
                    <span>{line.ts}</span>
                    <b className={`level-${line.level.toLowerCase()}`}>{line.level.toUpperCase()}</b>
                    {line.text}
                  </p>
                ))}
              </div>
            </div>

            <div className="panel alert-panel">
              <Bell size={17} />
              <p>No external adapter connected. All activity is simulated locally.</p>
            </div>
          </section>
        </div>
      </section>

      {modalOpen ? (
        <div className="modal-backdrop" role="presentation" data-testid="config-modal">
          <section className="config-modal" role="dialog" aria-modal="true" aria-labelledby="config-title">
            <div className="modal-head">
              <div>
                <p className="overline">CONFIG / f01</p>
                <h3 id="config-title">Message Sniper</h3>
              </div>
              <button onClick={() => setModalOpen(false)} type="button" aria-label="Close configuration">
                ×
              </button>
            </div>
            <div className="modal-grid">
              <label>
                Capture window
                <input readOnly value="10 minutes" />
              </label>
              <label>
                Channel scope
                <input readOnly value="simverse.guild.*" />
              </label>
              <label>
                Storage mode
                <input readOnly value="local sqlite queue" />
              </label>
              <label>
                Redaction
                <input readOnly value="enabled" />
              </label>
            </div>
            <div className="modal-actions">
              <button className="ghost-button" onClick={() => setModalOpen(false)} type="button">
                Cancel
              </button>
              <button className="engage-button" onClick={() => setModalOpen(false)} type="button">
                Apply
              </button>
            </div>
          </section>
        </div>
      ) : null}
    </main>
  );
}
