import ReactMarkdown from "react-markdown";
import type { Components } from "react-markdown";
import remarkGfm from "remark-gfm";
import type { DetailLevel } from "../types";
import { t } from "../i18n";

interface MarkdownViewProps {
  content: string;
  level: DetailLevel;
  summary?: string;
}

const markdownComponents: Components = {
  table: ({ children }) => (
    <div className="md-table-wrap">
      <table>{children}</table>
    </div>
  ),
  a: ({ href, children }) => (
    <a href={href} target="_blank" rel="noopener noreferrer">
      {children}
    </a>
  ),
};

export function MarkdownView({ content, level, summary }: MarkdownViewProps) {
  const display =
    level === "basic" && summary
      ? `${summary}\n\n---\n\n_${t.markdown.advancedHint}_`
      : content;

  return (
    <div className="markdown-body">
      <ReactMarkdown remarkPlugins={[remarkGfm]} components={markdownComponents}>
        {display}
      </ReactMarkdown>
    </div>
  );
}
