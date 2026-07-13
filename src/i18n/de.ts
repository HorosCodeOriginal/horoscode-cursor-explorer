/** Deutsche UI-Texte (du-Stil) für HorosCode Cursor Explorer */
export const de = {
  app: {
    title: "HorosCode Cursor Explorer",
    subtitle: "Deinen .cursor-Ordner verstehen",
    quickSearchPlaceholder: "Schnellsuche…",
    basic: "Einfach",
    advanced: "Erweitert",
    changeWorkspace: "Workspace wechseln",
    export: "Exportieren",
    pickWorkspaceTitle:
      "Workspace-Root wählen (Ordner mit .cursor)",
    scanning: ".cursor wird gescannt…",
    tauriUnavailable:
      "Tauri-Kontext nicht verfügbar — Browser-Vorschau ohne Desktop-Backend. Bitte die Desktop-App mit „npm run tauri dev“ starten.",
    tauriRequiredForAction:
      "Diese Aktion ist nur in der Desktop-App verfügbar. Bitte „npm run tauri dev“ starten.",
    scannedMeta: (date: string, rules: number, skills: number, agents: number) =>
      `Gescannt ${date} · ${rules} Regeln · ${skills} Skills · ${agents} Agenten`,
  },
  nav: {
    start: "Einstieg",
    folder: "Ordnerkarte",
    rules: "Regeln",
    skills: "Skills",
    agents: "Agenten",
    commands: "Befehle & Modi",
    hooks: "Hooks / MCP",
    search: "Suche",
  },
  startHere: {
    title: "Einstieg",
    lead:
      "Willkommen beim HorosCode Cursor Explorer — einer Desktop-Anleitung für deinen Workspace-.cursor-Ordner: wie Cursor-Regeln, Skills, Agenten, Befehle und Hooks zusammenspielen.",
    whatIsCursor: {
      title: "Was ist .cursor?",
      body: "Der .cursor-Ordner ist das Projektgedächtnis der Cursor IDE: dauerhafte Anweisungen (Regeln), wiederverwendbare Workflows (Skills), eigene Subagenten, Slash-Befehle und Automatisierungs-Hooks.",
    },
    teamAvatar: {
      title: "HorosCode Team Avatar",
      body: "Dieses Template nutzt orchestrierte Agenten (Sokka, Aang, Katara, …) mit Regeln wie orchestrator.mdc, die Arbeit über das Task-Tool routen, statt alles im Root-Thread zu erledigen.",
    },
    basicVsAdvanced: {
      title: "Einfach vs. Erweitert",
      body: "Einfach zeigt Zusammenfassungen und sichere Exporte (Geheimnisse geschwärzt). Erweitert zeigt vollständige Dateiinhalte, Hook-Skripte und Lockfile-Metadaten.",
    },
    quickMap: "Kurzübersicht",
    quickMapItems: [
      { path: "rules/*.mdc", desc: "immer aktiv oder glob-basierte Anweisungen" },
      { path: "skills/**/SKILL.md", desc: "HorosCode- und Drittanbieter-Skill-Bibliotheken" },
      { path: "skills-cursor/**", desc: "Cursor-eigene Skills (synchronisiert)" },
      { path: "agents/*.md", desc: "Team-Avatar-Subagenten-Definitionen" },
      { path: "commands/*.md", desc: "Slash-Befehl-Spickzettel" },
      { path: "hooks.json + hooks/*", desc: "Shell-Schutz vor Befehlen & Lint nach Edits" },
      { path: "mcp.json", desc: "MCP-Server-Endpunkte (z. B. Figma)" },
      { path: "permissions.json", desc: "Auto-Run-Erlauben/Blockieren-Listen" },
    ],
    runtimeNote:
      "Runtime-Ordner wie projects/ sind standardmäßig von Scans und Exporten ausgeschlossen — sie können Transkripte und Terminal-Ausgaben enthalten.",
  },
  folderMap: {
    title: "Ordnerkarte",
    lead: "Live-Baum von .cursor. Graue Knoten sind laut .gitignore von Scan/Export ausgeschlossen.",
    excluded: "ausgeschlossen",
  },
  rules: {
    title: "Regeln",
    hint: (count: number) => `${count} .mdc-Regeldateien`,
    always: "immer",
  },
  skills: {
    title: "Skills",
    hint: (count: number) => `${count} Skills (lokal + eingebaut)`,
    lockSource: "Lock-Quelle:",
  },
  agents: {
    title: "Agenten",
    hint: "Team-Avatar-Roster",
  },
  commands: {
    title: "Befehle & Modi",
    hint: "Slash-Befehle + Modus-Parameter",
    modeParameters: "Modus-Parameter",
    modeItems: [
      { cmd: "/lean", desc: "minimale Orchestrierung" },
      { cmd: "/wide", desc: "vollständige Ende-zu-Ende-Builds" },
      { cmd: "/doc-epic", desc: "Feature-Dokumentations-Suite" },
      { cmd: "/fix /debug /gui /perf", desc: "Elite-Team" },
      { cmd: "/deep /critical", desc: "Analyse-Split" },
    ],
    modeHint: "Siehe Regel agent-context-modes.mdc für den vollständigen Index.",
  },
  hooks: {
    title: "Hooks, MCP & Tooling",
    hooksJson: "hooks.json",
    hooksJsonHint:
      "Cursor-Lifecycle-Hooks — Shell-Schutz vor Befehlen, Lint nach Edits.",
    hookScripts: "Hook-Skripte",
    linesHint: (lines: number) =>
      `${lines} Zeilen — wechsle zu Erweitert, um den Inhalt zu sehen.`,
    mcpJson: "mcp.json",
    mcpHint: "Model-Context-Protocol-Server, die Agenten nutzen können.",
    permissionsJson: "permissions.json",
    permissionsHint:
      "Auto-Run-Erlauben/Blockieren-Anweisungen für Shell-Befehle.",
    skillsLockJson: "skills-lock.json",
    skillsLockHint:
      "Upstream-Herkunft und Hashes für installierte Skills.",
    gitignorePolicy: ".gitignore-Richtlinie",
    gitignoreBody:
      "Schließt projects/, Runtime-Artefakte und lokale Plugin-Caches von der Versionskontrolle aus. Scans respektieren dieselben Ausschlüsse.",
    notPresent: "In diesem Workspace nicht vorhanden.",
  },
  search: {
    title: "Suche",
    placeholder: "Regeln, Skills, Agenten, Befehle durchsuchen…",
    type: "Typ",
    source: "Quelle",
    searching: "Suche läuft…",
    noMatches: (query: string) => `Keine Treffer für „${query}"`,
  },
  relationships: {
    title: "Beziehungen",
    linksTo: "Verweist auf",
    linkedFrom: "Verknüpft von",
  },
  markdown: {
    advancedHint:
      "Wechsle in der Toolbar zu Erweitert, um das vollständige Dokument zu lesen.",
  },
  sources: {
    horosCodeLocal: "HorosCode lokal",
    cursorBuiltin: "Cursor eingebaut",
    upstream: "Upstream (Lockfile)",
    unknown: "Unbekannt",
  },
  types: {
    rule: "Regel",
    skill: "Skill",
    agent: "Agent",
    command: "Befehl",
    config: "Konfiguration",
  },
  roles: {
    Coordinator: "Koordinator",
    Worker: "Worker",
    "HorosCloud web UI": "HorosCloud Web-UI",
  },
  common: {
    redacted: "[GESCHWÄRZT]",
    lines: (n: number) => `${n} Zeilen`,
  },
} as const;

export type DeStrings = typeof de;
