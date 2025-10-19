import React, { useState } from "react";
import ChatHeader from "src/components/ui/chat/chat-header";
import ChatWindow from "src/components/ui/chat/chat-window";
import MessageInput from "src/components/ui/chat/message-input";
import { GoogleGenAI } from "@google/genai";
import { useChatBackend } from "src/context/chat-backend-context";

const ai = new GoogleGenAI({ apiKey: import.meta.env.GEMINI_API_KEY });

async function AIResponse(userInputArray) {
  const response = await ai.models.generateContent({
    model: "gemini-2.5-flash",

    contents: userInputArray,
  });

  return response.text;
}

let userInput = [];

export default function ChatLayout() {
  const [messages, setMessages] = useState([
    {
      role: "assistant",
      content: "Hello — I can help you with code, explanations, and more.",
    },
  ]);

  return (
    <div className="flex flex-col flex-start w-full max-w-3xl gap-4 p-4">
      <ChatHeader onDeleteAll={handleDeleteAll} />
      <ChatWindow messages={messages} />
      <MessageInput
        onSend={handleSend}
        onMessage={addMessage}
        onDeleteAll={handleDeleteAll}
      />
    </div>
  );
}

function addMessage(role, content) {
  const msg = { role, content };
  setMessages((s) => [...s, msg]);
}

async function handleSend(text) {
  const { setMessages } = useChatBackend();
  const userMsg = { role: "user", content: text };

  switch (setMessages) {
    case "gemini":
      userInput.push(text);
      const res = await AIResponse(userInput);
      setMessages((s) => [...s, userMsg]);
      setTimeout(() => {
        setMessages((s) => [...s, { role: "assistant", content: res }]);
      }, 600);
      break;
    case "rust":
      setMessages((s) => [...s, userMsg]);

      // fake assistant reply after short delay
      setTimeout(() => {
        setMessages((s) => [
          ...s,
          { role: "assistant", content: `You said: ${text}` },
        ]);
      }, 600);
      break;
    default:
      break;
  }
}

function handleDeleteAll() {
  if (!window.confirm("Delete all messages?")) return;
  setMessages([]);
}
