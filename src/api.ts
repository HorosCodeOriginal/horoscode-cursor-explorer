import { tauriInvoke } from "./tauri";
import type {
  ExportOptions,
  ExportResult,
  ItemSource,
  ItemType,
  SearchResult,
  WorkspaceIndex,
} from "./types";

export async function getDefaultWorkspace(): Promise<string> {
  return tauriInvoke<string>("get_default_workspace");
}

export async function scanWorkspace(workspaceRoot: string): Promise<WorkspaceIndex> {
  return tauriInvoke<WorkspaceIndex>("scan_workspace_cmd", { workspaceRoot });
}

export async function searchWorkspace(
  index: WorkspaceIndex,
  query: string,
  types: ItemType[],
  sources: ItemSource[],
): Promise<SearchResult[]> {
  return tauriInvoke<SearchResult[]>("search_workspace", {
    index,
    query,
    types,
    sources,
  });
}

export async function exportWorkspace(
  index: WorkspaceIndex,
  options: ExportOptions,
): Promise<ExportResult> {
  return tauriInvoke<ExportResult>("export_workspace", { index, options });
}

export async function openPath(path: string): Promise<void> {
  return tauriInvoke("open_path", { path });
}
