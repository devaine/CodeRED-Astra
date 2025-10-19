import React, { useState, useRef, useEffect } from "react";
import DownButton from "src/components/ui/button/down-button";
import { motion } from "motion/react";
import { BotMessageSquare } from "lucide-react";

export default function MessageInput({ onSend }) {
  const [text, setText] = useState("");
  const textareaRef = useRef(null);

  useEffect(() => {
    // ensure correct initial height
    if (textareaRef.current) textareaRef.current.style.height = "auto";
  }, []);

  function handleSubmit(e) {
    e.preventDefault();
    if (!text.trim()) return;
    onSend(text.trim());
    setText("");
  }

  return (
    <div className="w-full flex justify-center">
      <footer className="fixed bottom-6 max-w-3xl w-full px-4">
        <div className="flex flex-col gap-4">
          <div>
            <DownButton></DownButton>
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
