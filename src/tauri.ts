import { invoke, isTauri } from "@tauri-apps/api/core";

export class TauriUnavailableError extends Error {
  constructor(message?: string) {
    super(
      message ??
        "Tauri-Kontext nicht verfügbar. Bitte die Desktop-App mit „npm run tauri dev“ starten.",
    );
    this.name = "TauriUnavailableError";
  }
}

type TauriWindow = Window & {
  __TAURI_INTERNALS__?: { invoke?: unknown };
};

/** True when running inside a Tauri webview (not Vite-only browser preview). */
export function isTauriAvailable(): boolean {
  if (typeof window === "undefined") return false;
  if (isTauri()) return true;
  const internals = (window as TauriWindow).__TAURI_INTERNALS__;
  return typeof internals?.invoke === "function";
}

export async function tauriInvoke<T>(
  cmd: string,
  args: Record<string, unknown> = {},
): Promise<T> {
  if (!isTauriAvailable()) {
    throw new TauriUnavailableError();
  }
  return invoke<T>(cmd, args);
}
