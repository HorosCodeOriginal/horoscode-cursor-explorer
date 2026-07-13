import { MarkdownView } from "../MarkdownView";
import { RelationshipLinks } from "../RelationshipLinks";
import type { CommandEntry, DetailLevel, WorkspaceIndex } from "../../types";
import { t } from "../../i18n";

interface CommandsSectionProps {
  index: WorkspaceIndex;
  level: DetailLevel;
  selectedId?: string;
  onSelect: (id: string) => void;
  onNavigate: (id: string) => void;
}

export function CommandsSection({
  index,
  level,
  selectedId,
  onSelect,
  onNavigate,
}: CommandsSectionProps) {
  const selected =
    index.commands.find((c) => c.id === selectedId) ?? index.commands[0];

  return (
    <div className="split-section">
      <aside className="item-list">
        <h2>{t.commands.title}</h2>
        <p className="hint">{t.commands.hint}</p>
        <ul>
          {index.commands.map((c) => (
            <li key={c.id}>
              <button
                type="button"
                className={selected?.id === c.id ? "active" : ""}
                onClick={() => onSelect(c.id)}
              >
                <span className="item-name">/{c.name}</span>
              </button>
            </li>
          ))}
        </ul>
        <div className="mode-cheatsheet">
          <h3>{t.commands.modeParameters}</h3>
          <ul className="compact">
            {t.commands.modeItems.map((item) => (
              <li key={item.cmd}>
                <code>{item.cmd}</code> — {item.desc}
              </li>
            ))}
          </ul>
          <p className="hint">{t.commands.modeHint}</p>
        </div>
      </aside>
      {selected && (
        <CommandDetail
          command={selected}
          index={index}
          level={level}
          onNavigate={onNavigate}
        />
      )}
    </div>
  );
}

function CommandDetail({
  command,
  index,
  level,
  onNavigate,
}: {
  command: CommandEntry;
  index: WorkspaceIndex;
  level: DetailLevel;
  onNavigate: (id: string) => void;
}) {
  return (
    <div className="detail-panel">
      <header>
        <h2>/{command.name}</h2>
        <code>{command.relativePath}</code>
      </header>
      <RelationshipLinks
        index={index}
        itemId={command.id}
        onNavigate={onNavigate}
      />
      <MarkdownView
        content={command.body}
        summary={command.summary}
        level={level}
      />
    </div>
  );
}
