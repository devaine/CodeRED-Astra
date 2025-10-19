import React, { useRef, useEffect } from "react";

function MessageBubble({ message }) {
  const isUser = message.role === "user";
  return (
    <div
      className={`flex ${isUser ? "justify-end" : "justify-start"} px-4 py-8`}
    >
      <div
        className={`max-w-[70%] p-3 rounded-lg ${isUser ? "bg-indigo-600 text-white" : "bg-slate-700 text-slate-100"}`}
      >
        <div className="text-sm">{message.content}</div>
      </div>
    </div>
  );
}

export default function ChatWindow({ messages }) {
  return (
    <div className="flex-1 overflow-auto p-2">
      <div className="space-y-2">
        {messages.map((m, i) => (
          <MessageBubble key={i} message={m} />
        ))}
      </div>
    </div>
  );
}
