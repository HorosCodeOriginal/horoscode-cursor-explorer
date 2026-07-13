import { useEffect, useState } from "react";
import { searchWorkspace } from "../../api";
import { sourceLabel, typeLabel } from "../../i18n";
import type {
  DetailLevel,
  ItemSource,
  ItemType,
  SearchResult,
  WorkspaceIndex,
} from "../../types";
import { t } from "../../i18n";

interface SearchSectionProps {
  index: WorkspaceIndex;
  level: DetailLevel;
  onNavigate: (id: string) => void;
  initialQuery?: string;
}

const ALL_TYPES: ItemType[] = [
  "rule",
  "skill",
  "agent",
  "command",
  "config",
];

const ALL_SOURCES: ItemSource[] = [
  "horosCodeLocal",
  "cursorBuiltin",
  "upstream",
];

export function SearchSection({
  index,
  onNavigate,
  initialQuery = "",
}: SearchSectionProps) {
  const [query, setQuery] = useState(initialQuery);
  const [types, setTypes] = useState<ItemType[]>([]);
  const [sources, setSources] = useState<ItemSource[]>([]);
  const [results, setResults] = useState<SearchResult[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    setQuery(initialQuery);
  }, [initialQuery]);

  useEffect(() => {
    if (!query.trim()) {
      setResults([]);
      return;
    }

    const timer = setTimeout(async () => {
      setLoading(true);
      try {
        const hits = await searchWorkspace(index, query, types, sources);
        setResults(hits);
      } finally {
        setLoading(false);
      }
    }, 200);

    return () => clearTimeout(timer);
  }, [query, types, sources, index]);

  const toggleType = (type: ItemType) => {
    setTypes((prev) =>
      prev.includes(type) ? prev.filter((x) => x !== type) : [...prev, type],
    );
  };

  const toggleSource = (source: ItemSource) => {
    setSources((prev) =>
      prev.includes(source)
        ? prev.filter((x) => x !== source)
        : [...prev, source],
    );
  };

  return (
    <div className="section-panel search-panel">
      <h2>{t.search.title}</h2>
      <input
        className="search-input large"
        type="search"
        placeholder={t.search.placeholder}
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        autoFocus
      />

      <div className="facet-row">
        <span className="facet-label">{t.search.type}</span>
        {ALL_TYPES.map((type) => (
          <button
            key={type}
            type="button"
            className={`facet ${types.includes(type) || types.length === 0 ? "on" : ""}`}
            onClick={() => toggleType(type)}
          >
            {typeLabel(type)}
          </button>
        ))}
      </div>

      <div className="facet-row">
        <span className="facet-label">{t.search.source}</span>
        {ALL_SOURCES.map((source) => (
          <button
            key={source}
            type="button"
            className={`facet ${sources.includes(source) || sources.length === 0 ? "on" : ""}`}
            onClick={() => toggleSource(source)}
          >
            {sourceLabel(source)}
          </button>
        ))}
      </div>

      {loading && <p className="hint">{t.search.searching}</p>}

      <ul className="search-results">
        {results.map((r) => (
          <li key={r.id}>
            <button type="button" onClick={() => onNavigate(r.id)}>
              <div className="result-head">
                <strong>{r.title}</strong>
                <span className="badge tiny">{typeLabel(r.itemType)}</span>
                <span className="badge tiny muted">{sourceLabel(r.source)}</span>
              </div>
              <p>{r.snippet}</p>
              <code>{r.path}</code>
            </button>
          </li>
        ))}
      </ul>

      {query && !loading && results.length === 0 && (
        <p className="muted">{t.search.noMatches(query)}</p>
      )}
    </div>
  );
}
