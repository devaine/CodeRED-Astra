import React, { useState, useRef, useEffect } from "react";
import DownButton from "src/components/ui/button/down-button";
import BackendToggle from "src/components/ui/button/backend-toggle";
import ChatBackendContext from "src/context/chat-backend-context";
import { motion } from "motion/react";
import { BotMessageSquare } from "lucide-react";

export default function MessageInput({ onSend, onMessage }) {
  const [text, setText] = useState("");
  const textareaRef = useRef(null);

  useEffect(() => {
    // ensure correct initial height
    if (textareaRef.current) textareaRef.current.style.height = "auto";
  }, []);

  async function handleSubmit(e) {
    e.preventDefault();
    if (!text.trim()) return;

    // send user message locally
    onSend(text.trim());

    // create query on backend
    try {
      if (onMessage)
        onMessage("assistant", "Queued: sending request to server...");
      const createRes = await fetch(`/api/query/create`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ q: text, top_k: 5 }),
      });
      const createJson = await createRes.json();
      const id = createJson.id;
      if (!id) throw new Error("no id returned");

      // poll status
      let status = "Queued";
      if (onMessage) onMessage("assistant", `Status: ${status}`);
      while (status !== "Completed" && status !== "Failed") {
        await new Promise((r) => setTimeout(r, 1000));
        const sRes = await fetch(`/api/query/status?id=${id}`);
        const sJson = await sRes.json();
        status = sJson.status;
        if (onMessage) onMessage("assistant", `Status: ${status}`);
        if (status === "Cancelled") break;
      }

      if (status === "Completed") {
        const resultRes = await fetch(`/api/query/result?id=${id}`);
        const resultJson = await resultRes.json();
        const final =
          resultJson?.result?.final_answer ||
          JSON.stringify(resultJson?.result || {});
        if (onMessage) onMessage("assistant", final);
      } else {
        if (onMessage)
          onMessage("assistant", `Query status ended as: ${status}`);
      }
    } catch (err) {
      console.error(err);
      if (onMessage) onMessage("assistant", `Error: ${err.message}`);
    }
    setText("");
  }

  return (
    <div className="w-full flex justify-center">
      <footer className="fixed bottom-6 max-w-3xl w-full px-4">
        <div className="flex flex-col gap-4">
          <div className="flex justify-between items-center">
            <div className="flex items-center gap-2">
              <DownButton />
              <BackendToggle />
            </div>
          </div>

          <form
            onSubmit={handleSubmit}
            className="bg-gray-900 rounded-2xl border-2 border-gray-800 shadow-lg shadow-indigo-600"
          >
            <div className="flex p-2 shadow-xl items-center">
              <textarea
                ref={textareaRef}
                value={text}
                onChange={(e) => {
                  setText(e.target.value);
                  // auto-resize
                  const ta = textareaRef.current;
                  if (ta) {
                    ta.style.height = "auto";
                    ta.style.height = `${ta.scrollHeight}px`;
                  }
                }}
                onKeyDown={(e) => {
                  // Enter to submit, Shift+Enter for newline
                  if (e.key === "Enter" && !e.shiftKey) {
                    e.preventDefault();
                    handleSubmit(e);
                  }
                }}
                placeholder="Type a message..."
                rows={1}
                className="flex-1 mx-2 rounded-md shadow-2sx border-none focus:border-none focus:outline-none resize-none overflow-auto max-h-40"
              />
              <motion.button
                type="submit"
                className="flex gap-2 px-4 py-2 bg-gray-700 rounded-xl ml-4 items-center"
                whileHover={{ scale: 1.1 }}
                whileTap={{ scale: 0.9 }}
              >
                <BotMessageSquare />
              </motion.button>
            </div>
          </form>
        </div>
      </footer>
    </div>
  );
}
