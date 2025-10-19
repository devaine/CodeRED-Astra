import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./app/index.jsx";
import { ChatBackendProvider } from "./context/chat-backend-context";

createRoot(document.getElementById("root")).render(<App />);
