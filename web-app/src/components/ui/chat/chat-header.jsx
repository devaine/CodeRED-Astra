import React from "react";
import ActionButton from "src/components/ui/Button/ActionButton.jsx";

export default function ChatHeader({ title = "AI Assistant" }) {
  // Delete chat log (frontend + backend)
  const handleDeleteChat = async () => {
    if (!window.confirm("Delete all messages?")) return;
    await fetch(`/api/chat/${conversationId}`, { method: "DELETE" });
    setMessages([]);
  };

  // Restart chat (new conversation)
  const handleNewChat = async () => {
    const res = await fetch("/api/chat/new", { method: "POST" });
    const data = await res.json();
    if (data.success) {
      setConversationId(data.conversationId);
      setMessages([]);
    }
  };

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
        <ActionButton type="add" onClick={handleNewChat}></ActionButton>
        <ActionButton type="delete" onClick={handleDeleteChat}></ActionButton>
      </div>
    </header>
  );
}
