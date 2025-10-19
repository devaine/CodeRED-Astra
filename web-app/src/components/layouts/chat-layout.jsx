import React, { useState } from "react";
import ChatHeader from "src/components/ui/chat/chat-header";
import ChatWindow from "src/components/ui/chat/chat-window";
import MessageInput from "src/components/ui/chat/message-input";
import { GoogleGenAI } from "@google/genai";

let userInput = [];

const ai = new GoogleGenAI({ apiKey: import.meta.env.GEMINI_API_KEY });

async function AIResponse(userInputArray) {
  const response = await ai.models.generateContent({
    model: "gemini-2.5-flash",
    contents: userInputArray,
  });

  return response.text;
}

export default function ChatLayout() {
  const [messages, setMessages] = useState([
    {
      role: "assistant",
      content: "Hello — I can help you with code, explanations, and more.",
    },
  ]);

  async function handleSend(text) {
    const userMsg = { role: "user", content: text };
    userInput.push(text);
    const res = await AIResponse(userInput);
    setMessages((s) => [...s, userMsg]);
    setTimeout(() => {
      setMessages((s) => [...s, { role: "assistant", content: res }]);
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
      <MessageInput onSend={handleSend} onDeleteAll={handleDeleteAll} />
    </div>
  );
}
