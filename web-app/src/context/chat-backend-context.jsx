import React, { createContext, useContext, useState } from "react";

const ChatBackendContext = createContext(null);

export function ChatBackendProvider({ children }) {
  const [backend, setBackend] = useState("gemini"); // default

  function toggleBackend() {
    setBackend((b) => (b === "gemini" ? "rust" : "gemini"));
  }

  return (
    <ChatBackendContext.Provider value={{ backend, setBackend, toggleBackend }}>
      {children}
    </ChatBackendContext.Provider>
  );
}

export function useChatBackend() {
  const ctx = useContext(ChatBackendContext);
  if (!ctx) throw new Error("useChatBackend must be used within ChatBackendProvider");
  return ctx;
}

export default ChatBackendContext;
