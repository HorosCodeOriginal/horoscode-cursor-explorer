import type { DetailLevel } from "../../types";
import { t } from "../../i18n";

export function StartHere({ level }: { level: DetailLevel }) {
  return (
    <div className="section-panel">
      <h2>{t.startHere.title}</h2>
      <p className="lead">{t.startHere.lead}</p>

      <div className="card-grid">
        <Card title={t.startHere.whatIsCursor.title}>
          {t.startHere.whatIsCursor.body}
        </Card>
        <Card title={t.startHere.teamAvatar.title}>
          {t.startHere.teamAvatar.body}
        </Card>
        <Card title={t.startHere.basicVsAdvanced.title}>
          {t.startHere.basicVsAdvanced.body}
        </Card>
      </div>

      {level === "advanced" && (
        <div className="info-block">
          <h3>{t.startHere.quickMap}</h3>
          <ul>
            {t.startHere.quickMapItems.map((item) => (
              <li key={item.path}>
                <code>{item.path}</code> — {item.desc}
              </li>
            ))}
          </ul>
          <p>{t.startHere.runtimeNote}</p>
        </div>
      )}
    </div>
  );
}

function Card({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <article className="card">
      <h3>{title}</h3>
      <p>{children}</p>
    </article>
  );
}
