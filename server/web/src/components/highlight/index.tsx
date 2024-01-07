import { useEffect } from "react";
import javascript from "highlight.js/lib/languages/javascript";
import json from "highlight.js/lib/languages/json";
import hljs from "highlight.js";
import DOMPurify from "dompurify";

import "./style.css";

hljs.registerLanguage("javascript", javascript);
hljs.registerLanguage("json", json);

export enum HighlighterLang {
  JSON = "json",
}

interface HighlighterProps {
  content: string;
  language: HighlighterLang;
}

export const Highlighter = ({
  content,
  language,
}: HighlighterProps): JSX.Element => {
  useEffect(() => {
    hljs.highlightAll();
  }, []);

  content = DOMPurify.sanitize(content);
  const highlighted = hljs.highlight(content, { language: language as string });

  return (
    <pre className="hljs">
      <code
        dangerouslySetInnerHTML={{
          __html: DOMPurify.sanitize(highlighted.value),
        }}
      />
    </pre>
  );
};
