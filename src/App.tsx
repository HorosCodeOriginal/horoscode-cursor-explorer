import { useCallback, useEffect, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import {
  exportWorkspace,
  getDefaultWorkspace,
  openPath,
  scanWorkspace,
} from "./api";
import { AgentsSection } from "./components/sections/AgentsSection";
import { CommandsSection } from "./components/sections/CommandsSection";
import { FolderMap } from "./components/sections/FolderMap";
import { HooksMcpSection } from "./components/sections/HooksMcpSection";
import { RulesSection } from "./components/sections/RulesSection";
import { SearchSection } from "./components/sections/SearchSection";
import { SkillsSection } from "./components/sections/SkillsSection";
import { StartHere } from "./components/sections/StartHere";
import { t } from "./i18n";
import { isTauriAvailable } from "./tauri";
import type { DetailLevel, SectionId, WorkspaceIndex } from "./types";
import "./App.css";

const NAV: { id: SectionId; label: string }[] = [
  { id: "start", label: t.nav.start },
  { id: "folder", label: t.nav.folder },
  { id: "rules", label: t.nav.rules },
  { id: "skills", label: t.nav.skills },
  { id: "agents", label: t.nav.agents },
  { id: "commands", label: t.nav.commands },
  { id: "hooks", label: t.nav.hooks },
  { id: "search", label: t.nav.search },
];

function App() {
  const [workspace, setWorkspace] = useState("");
  const [index, setIndex] = useState<WorkspaceIndex | null>(null);
  const [section, setSection] = useState<SectionId>("start");
  const [level, setLevel] = useState<DetailLevel>("basic");
  const [selectedIds, setSelectedIds] = useState<Record<string, string>>({});
  const [error, setError] = useState<string | null>(null);
  const [tauriWarning, setTauriWarning] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [exportMsg, setExportMsg] = useState<string | null>(null);
  const [query, setQuery] = useState("");

  const load = useCallback(async (root: string) => {
    setLoading(true);
    setError(null);
    try {
      const data = await scanWorkspace(root);
      setIndex(data);
      setWorkspace(root);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    if (!isTauriAvailable()) {
      setTauriWarning(t.app.tauriUnavailable);
      setLoading(false);
      return;
    }
    getDefaultWorkspace()
      .then(load)
      .catch((e) => {
        setError(String(e));
        setLoading(false);
      });
  }, [load]);

  const pickWorkspace = async () => {
    if (!isTauriAvailable()) {
      setTauriWarning(t.app.tauriRequiredForAction);
      return;
    }
    const selected = await open({
      directory: true,
      multiple: false,
      title: t.app.pickWorkspaceTitle,
    });
    if (selected && typeof selected === "string") {
      await load(selected);
    }
  };

  const navigateToItem = (id: string) => {
    if (!index) return;
    if (id.startsWith("rule:")) {
      setSection("rules");
      setSelectedIds((s) => ({ ...s, rules: id }));
    } else if (id.startsWith("skill:")) {
      setSection("skills");
      setSelectedIds((s) => ({ ...s, skills: id }));
    } else if (id.startsWith("agent:")) {
      setSection("agents");
      setSelectedIds((s) => ({ ...s, agents: id }));
    } else if (id.startsWith("command:")) {
      setSection("commands");
      setSelectedIds((s) => ({ ...s, commands: id }));
    }
  };

  const handleExport = async () => {
    if (!index) return;
    if (!isTauriAvailable()) {
      setTauriWarning(t.app.tauriRequiredForAction);
      return;
    }
    setExportMsg(null);
    try {
      const result = await exportWorkspace(index, {
        sections: ["all"],
        basicMode: level === "basic",
        includePdfHtml: true,
      });
      setExportMsg(result.message);
      await openPath(result.exportDir);
    } catch (e) {
      setExportMsg(String(e));
    }
  };

  return (
    <div className="app">
      <header className="topbar">
        <div className="brand">
          <span className="logo">◈</span>
          <div>
            <h1>{t.app.title}</h1>
            <p className="subtitle">{t.app.subtitle}</p>
          </div>
        </div>
        <div className="topbar-actions">
          <input
            className="search-input"
            type="search"
            placeholder={t.app.quickSearchPlaceholder}
            value={query}
            onChange={(e) => {
              setQuery(e.target.value);
              setSection("search");
            }}
          />
          <div className="toggle-group">
            <button
              type="button"
              className={level === "basic" ? "active" : ""}
              onClick={() => setLevel("basic")}
            >
              {t.app.basic}
            </button>
            <button
              type="button"
              className={level === "advanced" ? "active" : ""}
              onClick={() => setLevel("advanced")}
            >
              {t.app.advanced}
            </button>
          </div>
          <button type="button" className="btn secondary" onClick={pickWorkspace}>
            {t.app.changeWorkspace}
          </button>
          <button
            type="button"
            className="btn primary"
            onClick={handleExport}
            disabled={!index}
          >
            {t.app.export}
          </button>
        </div>
      </header>

      {workspace && (
        <div className="workspace-bar">
          <code>{workspace}</code>
          {index && (
            <span className="scan-meta">
              {t.app.scannedMeta(
                new Date(index.scannedAt).toLocaleString(),
                index.rules.length,
                index.skills.length,
                index.agents.length,
              )}
            </span>
          )}
        </div>
      )}

      {tauriWarning && <div className="banner warning">{tauriWarning}</div>}
      {error && <div className="banner error">{error}</div>}
      {exportMsg && <div className="banner success">{exportMsg}</div>}

      <div className="layout">
        <nav className="sidebar">
          <ul>
            {NAV.map((item) => (
              <li key={item.id}>
                <button
                  type="button"
                  className={section === item.id ? "active" : ""}
                  onClick={() => setSection(item.id)}
                >
                  {item.label}
                </button>
              </li>
            ))}
          </ul>
        </nav>

        <main className="content">
          {loading && <p className="loading">{t.app.scanning}</p>}
          {!loading && !index && tauriWarning && (
            <StartHere level={level} />
          )}
          {!loading && index && (
            <>
              {section === "start" && <StartHere level={level} />}
              {section === "folder" && <FolderMap tree={index.tree} />}
              {section === "rules" && (
                <RulesSection
                  index={index}
                  level={level}
                  selectedId={selectedIds.rules}
                  onSelect={(id) =>
                    setSelectedIds((s) => ({ ...s, rules: id }))
                  }
                  onNavigate={navigateToItem}
                />
              )}
              {section === "skills" && (
                <SkillsSection
                  index={index}
                  level={level}
                  selectedId={selectedIds.skills}
                  onSelect={(id) =>
                    setSelectedIds((s) => ({ ...s, skills: id }))
                  }
                  onNavigate={navigateToItem}
                />
              )}
              {section === "agents" && (
                <AgentsSection
                  index={index}
                  level={level}
                  selectedId={selectedIds.agents}
                  onSelect={(id) =>
                    setSelectedIds((s) => ({ ...s, agents: id }))
                  }
                  onNavigate={navigateToItem}
                />
              )}
              {section === "commands" && (
                <CommandsSection
                  index={index}
                  level={level}
                  selectedId={selectedIds.commands}
                  onSelect={(id) =>
                    setSelectedIds((s) => ({ ...s, commands: id }))
                  }
                  onNavigate={navigateToItem}
                />
              )}
              {section === "hooks" && (
                <HooksMcpSection index={index} level={level} />
              )}
              {section === "search" && (
                <SearchSection
                  index={index}
                  level={level}
                  onNavigate={navigateToItem}
                  initialQuery={query}
                />
              )}
            </>
          )}
        </main>
      </div>
    </div>
  );
}

export default App;
