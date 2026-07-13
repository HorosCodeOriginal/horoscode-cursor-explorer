import type { Relationship, WorkspaceIndex } from "./types";

export function getRelationshipsFor(
  index: WorkspaceIndex,
  itemId: string,
): { outgoing: Relationship[]; incoming: Relationship[] } {
  const outgoing = index.relationships.filter((r) => r.fromId === itemId);
  const incoming = index.relationships.filter((r) => r.toId === itemId);
  return { outgoing, incoming };
}

export function resolveItemTitle(index: WorkspaceIndex, id: string): string {
  const rule = index.rules.find((r) => r.id === id);
  if (rule) return rule.name;
  const skill = index.skills.find((s) => s.id === id);
  if (skill) return skill.name;
  const agent = index.agents.find((a) => a.id === id);
  if (agent) return agent.name;
  const cmd = index.commands.find((c) => c.id === id);
  if (cmd) return `/${cmd.name}`;
  return id;
}

export { sourceLabel, typeLabel, roleLabel } from "./i18n";
