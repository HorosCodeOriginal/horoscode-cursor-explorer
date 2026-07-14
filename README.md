# HorosCode Cursor Explorer

Eine Windows-Desktop-App (Tauri + React), die deinen Workspace-Ordner `.cursor/` scannt und erklärt, wie du Regeln, Skills, Agenten, Befehle, Hooks und MCP-Konfiguration nutzt.

Entwickelt von **HorosCode** für HorosCloud-Template-Workspaces.

## Screenshots

### Dashboard — Einstieg

Neues Start-Dashboard mit **Metrik-Karten** (Regeln, Skills, Agenten, Scan-Status), **Erste 5 Minuten**-Onboarding, **Workspace Health** und **gruppierter Sidebar** (Verstehen → Konfiguration → Ausführen → Finden).

![Dashboard — Metrik-Karten, Onboarding und Workspace Health](docs/screenshots/einstieg-dashboard.png)

### Live-Ansicht

Dieselbe Oberfläche im laufenden HorosCode-Workspace: Fortschrittsbalken der Onboarding-Checkliste, Health-Checks pro Kategorie und Schnellkarten zu `.cursor/`, Team-Avataren und **Einfach** vs. **Erweitert**.

![Live-Ansicht — Dashboard im HorosCode-Template-Workspace](docs/screenshots/einstieg-live.png)

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

## Lizenz

HorosCode internes Template-Tooling.
