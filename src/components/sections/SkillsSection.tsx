import { MarkdownView } from "../MarkdownView";
import { RelationshipLinks } from "../RelationshipLinks";
import { sourceLabel } from "../../i18n";
import type { DetailLevel, SkillEntry, WorkspaceIndex } from "../../types";
import { t } from "../../i18n";

interface SkillsSectionProps {
  index: WorkspaceIndex;
  level: DetailLevel;
  selectedId?: string;
  onSelect: (id: string) => void;
  onNavigate: (id: string) => void;
}

export function SkillsSection({
  index,
  level,
  selectedId,
  onSelect,
  onNavigate,
}: SkillsSectionProps) {
  const selected =
    index.skills.find((s) => s.id === selectedId) ?? index.skills[0];

  return (
    <div className="split-section">
      <aside className="item-list">
        <h2>{t.skills.title}</h2>
        <p className="hint">{t.skills.hint(index.skills.length)}</p>
        <ul>
          {index.skills.map((s) => (
            <li key={s.id}>
              <button
                type="button"
                className={selected?.id === s.id ? "active" : ""}
                onClick={() => onSelect(s.id)}
              >
                <span className="item-name">{s.name}</span>
                <span className="badge tiny">{sourceLabel(s.source)}</span>
              </button>
            </li>
          ))}
        </ul>
      </aside>
      {selected && (
        <SkillDetail
          skill={selected}
          index={index}
          level={level}
          onNavigate={onNavigate}
        />
      )}
    </div>
  );
}

function SkillDetail({
  skill,
  index,
  level,
  onNavigate,
}: {
  skill: SkillEntry;
  index: WorkspaceIndex;
  level: DetailLevel;
  onNavigate: (id: string) => void;
}) {
  return (
    <div className="detail-panel">
      <header>
        <h2>{skill.name}</h2>
        <div className="meta-row">
          <span className="badge">{sourceLabel(skill.source)}</span>
          {skill.version && (
            <span className="badge muted">v{skill.version}</span>
          )}
          <code>{skill.relativePath}</code>
        </div>
        {level === "advanced" && (
          <div className="meta-block">
            {skill.lockSource && (
              <p>
                <strong>{t.skills.lockSource}</strong> {skill.lockSource} (
                {skill.lockSourceType})
              </p>
            )}
            {skill.description && <p>{skill.description}</p>}
          </div>
        )}
      </header>
      <RelationshipLinks
        index={index}
        itemId={skill.id}
        onNavigate={onNavigate}
      />
      <MarkdownView
        content={skill.body}
        summary={skill.summary}
        level={level}
      />
    </div>
  );
}
