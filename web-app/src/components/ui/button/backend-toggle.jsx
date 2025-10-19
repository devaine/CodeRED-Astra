import React from "react";
import { motion } from "motion/react";
import { Cpu } from "lucide-react";
import { useChatBackend } from "src/context/chat-backend-context";

export default function BackendToggle({ className }) {
  const { backend, toggleBackend } = useChatBackend();

  return (
    <motion.button
      onClick={(e) => {
        e.preventDefault();
        toggleBackend();
      }}
      className={`bg-gray-700 p-2 rounded-2xl file-input border-2 border-gray-600 text-md flex items-center gap-2 ${className || ""}`}
      whileHover={{ scale: 1.05 }}
      whileTap={{ scale: 0.95 }}
      title={`${backend}`}
    >
      <Cpu className="w-4 h-4" />
      <span className="uppercase">
        {backend === "rust" ? "rust" : "gemini"}
      </span>
    </motion.button>
  );
}
