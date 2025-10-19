import React, { useRef, useEffect } from "react";
import ReactMarkdown from "react-markdown";
import { MARKDOWN_COMPONENTS } from "src/config/markdown";

function MessageBubble({ message }) {
  const isUser = message.role === "user";
  const isError = !!message.error;
  return (
    <div className={`flex ${isUser ? "justify-end" : "justify-start"} py-2`}>
      <div
        className={`p-3 rounded-xl ${isUser ? "bg-indigo-600 text-white rounded-tr-sm" : "bg-gray-700 text-slate-100 rounded-tl-sm"} ${isError ? "border border-red-500/60 bg-red-900/50" : ""}`}
      >
        {isUser ? (
          <div className="text-sm">{message.content}</div>
        ) : (
          <ReactMarkdown components={MARKDOWN_COMPONENTS}>
            {message.content}
          </ReactMarkdown>
        )}
      </div>
    </div>
  );
}

export default function ChatWindow({ messages }) {
  const bottomRef = useRef(null);

  useEffect(() => {
    if (bottomRef.current) {
      bottomRef.current.scrollIntoView({ behavior: "smooth" });
    }
  }, [messages]);

  return (
    <div className="flex-1 overflow-auto px-2 pt-4 pb-32">
      <div className="">
        {messages.map((m, i) => (
          <MessageBubble key={m.id ?? i} message={m} />
        ))}
        <div ref={bottomRef} />
      </div>
    </div>
  );
}
