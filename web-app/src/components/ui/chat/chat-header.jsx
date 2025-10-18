import React from "react";

export default function ChatHeader({ title = "AI Assistant" }) {
  return (
    <header className="flex items-center justify-between px-4 py-3 bg-gradient-to-r from-slate-800 to-slate-900 text-white">
      <div className="flex items-center gap-3">
        <div className="w-10 h-10 bg-indigo-500 rounded flex items-center justify-center font-bold">
          AI
        </div>
        <div>
          <h1 className="text-lg font-semibold">{title}</h1>
          <p className="text-sm text-slate-300">
            Ask anything — AI is listening
          </p>
        </div>
      </div>
    </header>
  );
}
