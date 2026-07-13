import { MarkdownView } from "../MarkdownView";
import { RelationshipLinks } from "../RelationshipLinks";
import { sourceLabel } from "../../i18n";
import type { DetailLevel, RuleEntry, WorkspaceIndex } from "../../types";
import { t } from "../../i18n";

interface RulesSectionProps {
  index: WorkspaceIndex;
  level: DetailLevel;
  selectedId?: string;
  onSelect: (id: string) => void;
  onNavigate: (id: string) => void;
}

export function RulesSection({
  index,
  level,
  selectedId,
  onSelect,
  onNavigate,
}: RulesSectionProps) {
  const selected =
    index.rules.find((r) => r.id === selectedId) ?? index.rules[0];

  return (
    <div className="split-section">
      <aside className="item-list">
        <h2>{t.rules.title}</h2>
        <p className="hint">{t.rules.hint(index.rules.length)}</p>
        <ul>
          {index.rules.map((r) => (
            <li key={r.id}>
              <button
                type="button"
                className={selected?.id === r.id ? "active" : ""}
                onClick={() => onSelect(r.id)}
              >
                <span className="item-name">{r.name}</span>
                {r.alwaysApply && (
                  <span className="badge">{t.rules.always}</span>
                )}
              </button>
            </li>
          ))}
        </ul>
      </aside>
      {selected && (
        <RuleDetail
          rule={selected}
          index={index}
          level={level}
          onNavigate={onNavigate}
        />
      )}
    </div>
  );
}

function RuleDetail({
  rule,
  index,
  level,
  onNavigate,
}: {
  rule: RuleEntry;
  index: WorkspaceIndex;
  level: DetailLevel;
  onNavigate: (id: string) => void;
}) {
  return (
    <div className="detail-panel">
      <header>
        <h2>{rule.name}</h2>
        <div className="meta-row">
          <span className="badge">{sourceLabel(rule.source)}</span>
          {rule.alwaysApply && <span className="badge accent">alwaysApply</span>}
          <code>{rule.relativePath}</code>
        </div>
        {level === "advanced" && rule.description && (
          <p className="description">{rule.description}</p>
        )}
      </header>
      <RelationshipLinks
        index={index}
        itemId={rule.id}
        onNavigate={onNavigate}
      />
      <MarkdownView
        content={rule.body}
        summary={rule.summary}
        level={level}
      />
    </div>
  );
}
