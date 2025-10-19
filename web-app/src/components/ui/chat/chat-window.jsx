import React, { useRef, useEffect } from "react";

function MessageBubble({ message }) {
  const isUser = message.role === "user";
  return (
    <div className={`flex ${isUser ? "justify-end" : "justify-start"} py-2`}>
      <div
        className={`p-3 rounded-xl  ${isUser ? "bg-indigo-600 text-white rounded-tr-sm" : "bg-gray-700 text-slate-100 rounded-tl-sm"}`}
      >
        <div className="text-sm">{message.content}</div>
      </div>
    </div>
  );
}

export default function ChatWindow({ messages }) {
  return (
    <div className="flex-1 overflow-auto px-2 pt-4 pb-32">
      <div className="">
        {messages.map((m, i) => (
          <MessageBubble key={i} message={m} />
        ))}
      </div>
    </div>
  );
}
