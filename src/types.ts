export type ItemType =
  | "rule"
  | "skill"
  | "agent"
  | "command"
  | "config"
  | "folder"
  | "hook";

export type ItemSource =
  | "horosCodeLocal"
  | "cursorBuiltin"
  | "upstream"
  | "unknown";

export interface TreeNode {
  name: string;
  path: string;
  isDir: boolean;
  children: TreeNode[];
  excluded: boolean;
  note?: string;
}

export interface RuleEntry {
  id: string;
  name: string;
  path: string;
  relativePath: string;
  description?: string;
  globs?: string;
  alwaysApply: boolean;
  summary: string;
  body: string;
  source: ItemSource;
  references: string[];
}

export interface SkillEntry {
  id: string;
  name: string;
  path: string;
  relativePath: string;
  description?: string;
  version?: string;
  sourceUrls: string[];
  lockSource?: string;
  lockSourceType?: string;
  summary: string;
  body: string;
  source: ItemSource;
  references: string[];
}

export interface AgentEntry {
  id: string;
  name: string;
  path: string;
  relativePath: string;
  description?: string;
  model?: string;
  summary: string;
  body: string;
  source: ItemSource;
  references: string[];
  role?: string;
}

export interface CommandEntry {
  id: string;
  name: string;
  path: string;
  relativePath: string;
  summary: string;
  body: string;
  source: ItemSource;
  references: string[];
}

export interface HookScriptEntry {
  name: string;
  path: string;
  relativePath: string;
  body: string;
}

export interface ConfigBundle {
  hooksJson?: unknown;
  permissionsJson?: unknown;
  mcpJson?: unknown;
  skillsLockJson?: unknown;
  gitignoreContent?: string;
  hookScripts: HookScriptEntry[];
}

export interface Relationship {
  fromId: string;
  fromType: ItemType;
  toId: string;
  toType: ItemType;
  label: string;
}

export interface WorkspaceIndex {
  workspaceRoot: string;
  cursorRoot: string;
  scannedAt: string;
  tree: TreeNode;
  rules: RuleEntry[];
  skills: SkillEntry[];
  agents: AgentEntry[];
  commands: CommandEntry[];
  configs: ConfigBundle;
  relationships: Relationship[];
  excludePatterns: string[];
}

export interface SearchResult {
  id: string;
  itemType: ItemType;
  source: ItemSource;
  title: string;
  snippet: string;
  path: string;
}

export interface ExportOptions {
  sections: string[];
  basicMode: boolean;
  includePdfHtml: boolean;
}

export interface ExportResult {
  exportDir: string;
  filesWritten: string[];
  message: string;
}

export type SectionId =
  | "start"
  | "folder"
  | "rules"
  | "skills"
  | "agents"
  | "commands"
  | "hooks"
  | "search";

export type DetailLevel = "basic" | "advanced";

export interface Selection {
  section: SectionId;
  itemId?: string;
}
