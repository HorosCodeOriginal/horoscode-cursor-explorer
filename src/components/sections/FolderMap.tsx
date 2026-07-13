import type { TreeNode } from "../../types";
import { t } from "../../i18n";

interface FolderMapProps {
  tree: TreeNode;
  onSelectPath?: (path: string) => void;
}

export function FolderMap({ tree, onSelectPath }: FolderMapProps) {
  return (
    <div className="section-panel">
      <h2>{t.folderMap.title}</h2>
      <p className="lead">{t.folderMap.lead}</p>
      <div className="tree-root">
        <TreeNodeView node={tree} depth={0} onSelectPath={onSelectPath} />
      </div>
    </div>
  );
}

function TreeNodeView({
  node,
  depth,
  onSelectPath,
}: {
  node: TreeNode;
  depth: number;
  onSelectPath?: (path: string) => void;
}) {
  return (
    <div
      className={`tree-node ${node.excluded ? "excluded" : ""}`}
      style={{ paddingLeft: depth * 16 }}
    >
      <button
        type="button"
        className="tree-label"
        onClick={() => onSelectPath?.(node.path)}
        title={node.path}
      >
        <span className="tree-icon">{node.isDir ? "📁" : "📄"}</span>
        {node.name}
        {node.excluded && (
          <span className="badge muted">{t.folderMap.excluded}</span>
        )}
      </button>
      {node.note && <div className="tree-note">{node.note}</div>}
      {node.children.map((child) => (
        <TreeNodeView
          key={child.path}
          node={child}
          depth={depth + 1}
          onSelectPath={onSelectPath}
        />
      ))}
    </div>
  );
}
