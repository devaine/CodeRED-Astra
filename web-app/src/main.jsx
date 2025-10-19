import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./app/index.jsx";
import { ChatBackendProvider } from "./context/chat-backend-context";

createRoot(document.getElementById("root")).render(
  <StrictMode>
    <ChatBackendProvider>
      <App />
    </ChatBackendProvider>
  </StrictMode>
);
