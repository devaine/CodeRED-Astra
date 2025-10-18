import React, { useState } from "react";
import ChatHeader from "./ChatHeader";
import ChatWindow from "./ChatWindow";
import MessageInput from "./MessageInput";

export default function ChatLayout() {
  const [messages, setMessages] = useState([
    {
      role: "assistant",
      content: "Hello — I can help you with code, explanations, and more.",
    },
  ]);

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

  return (
    <div className="flex flex-col h-[80vh] w-full max-w-3xl mx-auto rounded-lg overflow-hidden shadow-lg border border-slate-700">
      <ChatHeader />
      <ChatWindow messages={messages} />
      <MessageInput onSend={handleSend} />
    </div>
  );
}
