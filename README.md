# HorosCode Cursor Explorer

Eine Windows-Desktop-App (Tauri + React), die deinen Workspace-Ordner `.cursor/` scannt und erklärt, wie du Regeln, Skills, Agenten, Befehle, Hooks und MCP-Konfiguration nutzt.

Entwickelt von **HorosCode** für HorosCloud-Template-Workspaces.

## Screenshots

### Einstieg — Dashboard

Start-Dashboard mit **Metrik-Karten** (Regeln, Skills, Agenten, Scan-Status), **Erste 5 Minuten**-Onboarding, **Workspace Health** und **gruppierter Sidebar** (Verstehen → Konfiguration → Ausführen → Finden).

![Einstieg — Metrik-Karten, Onboarding und Workspace Health](docs/screenshots/einstieg.png)

### Regeln

Regel-Explorer im **Erweitert**-Modus: alle `.mdc`-Dateien mit Frontmatter, Beziehungsgraph und Volltext — hier die Orchestrator-Regel mit Team-Avatar-Verknüpfungen.

![Regeln — orchestrator.mdc mit Beziehungen und Erweitert-Ansicht](docs/screenshots/regeln.png)

### Agenten

Agenten-Roster mit Typ-Badges (Agent, Koordinator) und Detailansicht inklusive Beziehungslinks — Beispiel **HELPER-AGENT-QUICKSTART** mit Feature-Checkliste.

![Agenten — HELPER-AGENT-QUICKSTART mit Beziehungen](docs/screenshots/agenten.png)

### Befehle & Modi

Slash-Befehle und Modus-Parameter auf einen Blick — `/build` mit Workflow (Explore → Plan → Execute → Verify) und Spickzettel für `/lean`, `/wide`, `/doc-epic` u. a.

![Befehle & Modi — /build und Modus-Parameter](docs/screenshots/befehle-modi.png)

### Suche

Volltextsuche über Regeln, Skills, Agenten und Befehle mit **Typ-** und **Quellen-Filtern** (HorosCode lokal, Cursor eingebaut, Upstream).

![Suche — Volltext mit Typ- und Quellen-Facetten](docs/screenshots/suche.png)

### Kurzübersicht

Gescrolltes Einstieg-Dashboard mit **Kurzübersicht** aller `.cursor/`-Konventionen (`rules/*.mdc`, `skills/**/SKILL.md`, `agents/*.md`, Hooks, MCP).

![Einstieg — Kurzübersicht der .cursor-Konventionen](docs/screenshots/einstieg-kurzuebersicht.png)

## Funktionen

- **Dashboard** — Metrik-Karten für Regeln, Skills, Agenten und letzten Scan; Sparklines und Health-Gauge auf einen Blick
- **Erste 5 Minuten** — Onboarding-Checkliste mit Fortschritt (Workspace öffnen, scannen, Regeln prüfen, Skills entdecken, Export)
- **Workspace Health** — Kategorie-Checks (Regeln, Skills, Agenten, Hooks) mit klarem Status
- **Gruppierte Sidebar** — Navigation in vier Bereichen: Verstehen, Konfiguration, Ausführen, Finden
- **Ordnerkarte** — Live-Baum mit Ausschluss-Hinweisen aus `.gitignore`
- **Regeln** — parst `rules/*.mdc` Frontmatter + vollständige Inhalte
- **Skills** — lokale HorosCode-Skills, Cursor-Eingebaute (`skills-cursor/`), Upstream-Lockfile-Metadaten
- **Agenten** — Team-Avatar-Roster mit Beziehungslinks
- **Befehle & Modi** — Slash-Befehle + Modus-Parameter-Spickzettel
- **Hooks / MCP** — `hooks.json`, Skripte, `mcp.json`, `permissions.json`
- **Suche** — Volltext mit Typ-/Quellen-Facetten
- **Einfach / Erweitert** — Zusammenfassungen vs. vollständige Inhalte; Einfach-Export schwärzt offensichtliche Geheimnisse
- **Export** — Markdown-Bundle nach `export/` + druckfertiges HTML (Drucken → Als PDF speichern)

## Voraussetzungen

- [Node.js](https://nodejs.org/) 18+ (getestet mit v24)
- [Rust](https://rustup.rs/) stable (für Tauri)
- Windows 10+

## Installation (Windows)

Nach `npm run tauri build` liegt der Windows-Installer unter:

```
cursor-explorer/dist-installer/HorosCode-Cursor-Explorer-Setup.exe
```

Im HorosCode-Template-Root liegt zusätzlich eine fertige `HorosCode-Cursor-Explorer-Setup.exe` (neben `starter.bat`). Doppelklick installiert die App; danach startet `starter.bat` die Release-Version automatisch.

## Starten (Entwicklung)

```powershell
cd cursor-explorer
npm install
npm run tauri dev
```

Beim ersten Start erkennt die App automatisch den Workspace-Root (übergeordneter Ordner mit `.cursor`). Nutze **Workspace wechseln**, um einen anderen Ordner zu wählen.

## Build (Produktion)

```powershell
cd cursor-explorer
npm install
npm run tauri build
```

Installer/EXE liegen unter `src-tauri/target/release/bundle/`.

## Starten mit starter.bat (HorosCloud-Template)

Im übergeordneten HorosCode-Template-Ordner liegt `starter.bat` (neben dem Ordner `cursor-explorer/`). Das Skript:

1. Wechselt nach `cursor-explorer/`
2. Startet `src-tauri\target\release\cursor-explorer.exe`, falls vorhanden
3. Sonst Debug-Build oder `npm run tauri dev`

```batch
cd C:\Pfad\zu\Templates\Folders
starter.bat
```

Ohne gebaute EXE zuerst Release bauen:

```powershell
cd cursor-explorer
npm install
npm run tauri build
```

Logs bei Startproblemen: `src-tauri\target\release\starter-launch.log`

## Export

Klicke in der Toolbar auf **Exportieren**. Dateien landen in `<workspace>/export/`:

| Pfad | Inhalt |
|------|--------|
| `README.md` | Bundle-Index |
| `rules/`, `skills/`, `agents/`, `commands/` | Abschnitts-Markdown |
| `configs/` | JSON-Konfigurationen (im Einfach-Modus geschwärzt) |
| `relationships.md` | Querverweis-Karte |
| `cursor-explorer-bundle.html` | Im Browser öffnen → Drucken → Als PDF speichern |

Runtime-Ordner wie `.cursor/projects/` werden **niemals** gescannt oder exportiert.

## Projektstruktur

```
cursor-explorer/
  docs/screenshots/    README-Galerie
  src/                 React-UI
  src-tauri/src/       Rust-Scanner + Export
    scanner.rs         Indexer
    export.rs          Markdown/HTML-Export
    parser.rs          Frontmatter + Referenz-Extraktion
```

## Topics & Hashtags

Kuratierte Tags für GitHub-Topics, Social Media und README — ohne Spam, ohne Token- oder API-Hashtags.

### Marke

| Hashtag | Verwendung |
|---------|------------|
| `#HorosCode` | Firma / Entwickler |
| `#HorosCloud` | Produktlinie / Template-Workspaces |

### Produkt

| Hashtag | Verwendung |
|---------|------------|
| `#CursorExplorer` | App-Name |
| `#DotCursor` | `.cursor/`-Ordner im Workspace |
| `#CursorWorkspace` | Workspace-Konfiguration erkunden |

### Tech

| Hashtag | Verwendung |
|---------|------------|
| `#Tauri` | Desktop-Framework (Rust + WebView) |
| `#React` | UI-Stack |
| `#DesktopApp` | Native Windows-App |
| `#Windows` | Zielplattform |

### Cursor / AI

| Hashtag | Verwendung |
|---------|------------|
| `#CursorIDE` | Cursor-Editor-Ökosystem |
| `#AIAgents` | Agenten-Roster & Orchestrierung |
| `#AgentOrchestration` | Dispatcher, Koordinatoren, Worker |
| `#CursorRules` | `rules/*.mdc` |
| `#CursorSkills` | `skills/**/SKILL.md` |
| `#MCP` | Model Context Protocol |
| `#SlashCommands` | Befehle & Modus-Parameter |

### Zielgruppe (DE)

| Hashtag | Verwendung |
|---------|------------|
| `#EntwicklerTools` | Dev-Tooling für Teams |
| `#Dokumentation` | Export, README-Bundles, Onboarding |
| `#OpenSource` | Öffentliches Repo & Community |

### GitHub-Topics

`horoscode`, `cursor-ide`, `cursor-explorer`, `tauri`, `react`, `desktop-app`, `developer-tools`, `ai-agents`, `agent-orchestration`, `dotcursor`, `workspace-explorer`, `rules`, `skills`, `hooks`, `mcp`, `windows`, `documentation`, `open-source`, `german`

### Social — Copy & Paste

Alle Hashtags in einer Zeile (DE/EN):

`#HorosCode` `#HorosCloud` `#CursorExplorer` `#DotCursor` `#CursorWorkspace` `#Tauri` `#React` `#DesktopApp` `#Windows` `#CursorIDE` `#AIAgents` `#AgentOrchestration` `#CursorRules` `#CursorSkills` `#MCP` `#SlashCommands` `#EntwicklerTools` `#Dokumentation` `#OpenSource`

Fertige Post-Vorlagen: [HASHTAGS.md](HASHTAGS.md)

## Lizenz

HorosCode internes Template-Tooling.
