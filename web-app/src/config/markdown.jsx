export const MARKDOWN_COMPONENTS = {
  h1: ({ node, ...props }) => (
    <h1 className="text-xl font-semibold mt-2 mb-1" {...props} />
  ),
  h2: ({ node, ...props }) => (
    <h2 className="text-lg font-semibold mt-2 mb-1" {...props} />
  ),
  h3: ({ node, ...props }) => (
    <h3 className="text-md font-semibold mt-2 mb-1" {...props} />
  ),
  p: ({ node, ...props }) => (
    <p className="text-sm leading-relaxed mb-2" {...props} />
  ),
  a: ({ node, href, ...props }) => (
    <a
      href={href}
      className="text-indigo-300 hover:underline"
      target="_blank"
      rel="noopener noreferrer"
      {...props}
    />
  ),
  code: ({ node, inline, className, children, ...props }) => {
    if (inline) {
      return (
        <code
          className={`bg-slate-800 px-1 py-0.5 rounded text-sm ${className || ""}`}
          {...props}
        >
          {children}
        </code>
      );
    }
    return (
      <pre
        className="bg-slate-800 p-2 rounded overflow-auto text-sm"
        {...props}
      >
        <code className={className || ""}>{children}</code>
      </pre>
    );
  },
  blockquote: ({ node, ...props }) => (
    <blockquote
      className="border-l-2 border-slate-600 pl-4 italic text-slate-200 my-2"
      {...props}
    />
  ),
  ul: ({ node, ...props }) => (
    <ul className="list-disc list-inside ml-4 mb-2" {...props} />
  ),
  ol: ({ node, ...props }) => (
    <ol className="list-decimal list-inside ml-4 mb-2" {...props} />
  ),
  li: ({ node, ...props }) => <li className="mb-1" {...props} />,
  strong: ({ node, ...props }) => (
    <strong className="font-semibold" {...props} />
  ),
  em: ({ node, ...props }) => <em className="italic" {...props} />,
};
