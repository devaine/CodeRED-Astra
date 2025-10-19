import React, { useState } from "react";
import ChatHeader from "src/components/ui/chat/chat-header";
import ChatWindow from "src/components/ui/chat/chat-window";
import MessageInput from "src/components/ui/chat/message-input";
import '../../index.css'
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
    <div className="flex flex-col">
      <ChatHeader />
      <ChatWindow messages={messages} />
      <MessageInput onSend={handleSend} />
    </div>
  );
}
