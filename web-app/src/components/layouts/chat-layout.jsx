import React, { useState } from "react";
import ChatHeader from "src/components/ui/chat/chat-header";
import ChatWindow from "src/components/ui/chat/chat-window";
import MessageInput from "src/components/ui/chat/message-input";

export default function ChatLayout() {
  const [messages, setMessages] = useState([
    {
      role: "assistant",
      content: "Hello — I can help you with code, explanations, and more.",
    },
  ]);

  function addMessage(role, content) {
    const msg = { role, content };
    setMessages((s) => [...s, msg]);
  }

  function handleSend(text) {
    const userMsg = { role: "user", content: text };
    setMessages((s) => [...s, userMsg]);

    // fake assistant reply after short delay
    setTimeout(() => {
      setMessages((s) => [
        ...s,
        { role: "assistant", content: `You said: ${text}` },
      ]);
    }, 600);
  }

  function handleDeleteAll() {
    if (!window.confirm("Delete all messages?")) return;
    setMessages([]);
  }

  return (
    <div className="flex flex-col flex-start w-full max-w-3xl gap-4 p-4">
      <ChatHeader />
      <ChatWindow messages={messages} />
      <MessageInput
        onSend={handleSend}
        onMessage={addMessage}
        onDeleteAll={handleDeleteAll}
      />
    </div>
  );
}
