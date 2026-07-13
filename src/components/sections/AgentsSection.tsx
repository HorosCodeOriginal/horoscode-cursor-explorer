import { MarkdownView } from "../MarkdownView";
import { RelationshipLinks } from "../RelationshipLinks";
import { roleLabel, sourceLabel } from "../../i18n";
import type { AgentEntry, DetailLevel, WorkspaceIndex } from "../../types";
import { t } from "../../i18n";

interface AgentsSectionProps {
  index: WorkspaceIndex;
  level: DetailLevel;
  selectedId?: string;
  onSelect: (id: string) => void;
  onNavigate: (id: string) => void;
}

export function AgentsSection({
  index,
  level,
  selectedId,
  onSelect,
  onNavigate,
}: AgentsSectionProps) {
  const selected =
    index.agents.find((a) => a.id === selectedId) ?? index.agents[0];

  return (
    <div className="split-section">
      <aside className="item-list">
        <h2>{t.agents.title}</h2>
        <p className="hint">{t.agents.hint}</p>
        <ul>
          {index.agents.map((a) => (
            <li key={a.id}>
              <button
                type="button"
                className={selected?.id === a.id ? "active" : ""}
                onClick={() => onSelect(a.id)}
              >
                <span className="item-name">{a.name}</span>
                {a.role && (
                  <span className="badge tiny">{roleLabel(a.role)}</span>
                )}
              </button>
            </li>
          ))}
        </ul>
      </aside>
      {selected && (
        <AgentDetail
          agent={selected}
          index={index}
          level={level}
          onNavigate={onNavigate}
        />
      )}
    </div>
  );
}

function AgentDetail({
  agent,
  index,
  level,
  onNavigate,
}: {
  agent: AgentEntry;
  index: WorkspaceIndex;
  level: DetailLevel;
  onNavigate: (id: string) => void;
}) {
  return (
    <div className="detail-panel">
      <header>
        <h2>{agent.name}</h2>
        <div className="meta-row">
          <span className="badge">{sourceLabel(agent.source)}</span>
          {agent.role && (
            <span className="badge accent">{roleLabel(agent.role)}</span>
          )}
          {agent.model && <span className="badge muted">{agent.model}</span>}
          <code>{agent.relativePath}</code>
        </div>
        {agent.description && level === "basic" && (
          <p className="description">{agent.description}</p>
        )}
      </header>
      <RelationshipLinks
        index={index}
        itemId={agent.id}
        onNavigate={onNavigate}
      />
      <MarkdownView
        content={agent.body}
        summary={agent.summary}
        level={level}
      />
    </div>
  );
}
