import { de } from "./de";

export const t = de;

export function typeLabel(type: string): string {
  const key = type as keyof typeof de.types;
  return de.types[key] ?? type.charAt(0).toUpperCase() + type.slice(1);
}

export function sourceLabel(source: string): string {
  switch (source) {
    case "horosCodeLocal":
      return de.sources.horosCodeLocal;
    case "cursorBuiltin":
      return de.sources.cursorBuiltin;
    case "upstream":
      return de.sources.upstream;
    default:
      return de.sources.unknown;
  }
}

export function roleLabel(role: string): string {
  const key = role as keyof typeof de.roles;
  return de.roles[key] ?? role;
}

export { de };
