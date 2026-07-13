import type { Relationship, WorkspaceIndex } from "../types";
import { resolveItemTitle } from "../utils";
import { t } from "../i18n";

interface RelationshipLinksProps {
  index: WorkspaceIndex;
  itemId: string;
  onNavigate: (id: string) => void;
}

export function RelationshipLinks({
  index,
  itemId,
  onNavigate,
}: RelationshipLinksProps) {
  const outgoing = index.relationships.filter((r) => r.fromId === itemId);
  const incoming = index.relationships.filter((r) => r.toId === itemId);

  if (outgoing.length === 0 && incoming.length === 0) {
    return null;
  }

  return (
    <div className="relationships">
      <h4>{t.relationships.title}</h4>
      {outgoing.length > 0 && (
        <RelList
          title={t.relationships.linksTo}
          rels={outgoing}
          direction="out"
          index={index}
          onNavigate={onNavigate}
        />
      )}
      {incoming.length > 0 && (
        <RelList
          title={t.relationships.linkedFrom}
          rels={incoming}
          direction="in"
          index={index}
          onNavigate={onNavigate}
        />
      )}
    </div>
  );
}

function RelList({
  title,
  rels,
  direction,
  index,
  onNavigate,
}: {
  title: string;
  rels: Relationship[];
  direction: "in" | "out";
  index: WorkspaceIndex;
  onNavigate: (id: string) => void;
}) {
  return (
    <div className="rel-group">
      <span className="rel-title">{title}</span>
      <ul>
        {rels.map((r) => {
          const targetId = direction === "out" ? r.toId : r.fromId;
          return (
            <li key={`${r.fromId}-${r.toId}-${r.label}`}>
              <button
                type="button"
                className="link-btn"
                onClick={() => onNavigate(targetId)}
              >
                {resolveItemTitle(index, targetId)}
              </button>
              <span className="rel-label">{r.label}</span>
            </li>
          );
        })}
      </ul>
    </div>
  );
}
