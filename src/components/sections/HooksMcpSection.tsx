import type { DetailLevel, WorkspaceIndex } from "../../types";
import { t } from "../../i18n";

interface HooksMcpSectionProps {
  index: WorkspaceIndex;
  level: DetailLevel;
}

export function HooksMcpSection({ index, level }: HooksMcpSectionProps) {
  const { configs } = index;

  return (
    <div className="section-panel hooks-panel">
      <h2>{t.hooks.title}</h2>

      <section className="config-block">
        <h3>{t.hooks.hooksJson}</h3>
        <p className="hint">{t.hooks.hooksJsonHint}</p>
        <JsonBlock value={configs.hooksJson} level={level} />
      </section>

      <section className="config-block">
        <h3>{t.hooks.hookScripts}</h3>
        <ul className="script-list">
          {configs.hookScripts.map((s) => (
            <li key={s.path}>
              <strong>{s.name}</strong>
              <code>{s.relativePath}</code>
              {level === "advanced" ? (
                <pre>{s.body}</pre>
              ) : (
                <p className="hint">
                  {t.hooks.linesHint(s.body.split("\n").length)}
                </p>
              )}
            </li>
          ))}
        </ul>
      </section>

      <section className="config-block">
        <h3>{t.hooks.mcpJson}</h3>
        <p className="hint">{t.hooks.mcpHint}</p>
        <JsonBlock value={configs.mcpJson} level={level} />
      </section>

      <section className="config-block">
        <h3>{t.hooks.permissionsJson}</h3>
        <p className="hint">{t.hooks.permissionsHint}</p>
        <JsonBlock value={configs.permissionsJson} level={level} />
      </section>

      {level === "advanced" && (
        <section className="config-block">
          <h3>{t.hooks.skillsLockJson}</h3>
          <p className="hint">{t.hooks.skillsLockHint}</p>
          <JsonBlock value={configs.skillsLockJson} level={level} />
        </section>
      )}

      <section className="config-block">
        <h3>{t.hooks.gitignorePolicy}</h3>
        {level === "advanced" && configs.gitignoreContent ? (
          <pre>{configs.gitignoreContent}</pre>
        ) : (
          <p>{t.hooks.gitignoreBody}</p>
        )}
      </section>
    </div>
  );
}

function JsonBlock({
  value,
  level,
}: {
  value: unknown;
  level: DetailLevel;
}) {
  if (!value) {
    return <p className="muted">{t.hooks.notPresent}</p>;
  }

  const text = JSON.stringify(value, null, 2);
  const display = level === "basic" ? redactBasic(text) : text;

  return <pre className="json-block">{display}</pre>;
}

function redactBasic(json: string): string {
  return json.replace(
    /"(api[_-]?key|secret|token|password|auth)"\s*:\s*"[^"]*"/gi,
    '"$1": "[REDACTED]"',
  );
}
