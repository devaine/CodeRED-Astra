import React, { useState } from "react";
import DeleteButton from "src/components/ui/button/delete-button";
import DownButton from "src/components/ui/button/down-button";
import SchematicButton from "src/components/ui/button/schematic-button";
import { motion } from "motion/react";
import { BotMessageSquare } from "lucide-react";

export default function MessageInput({ onSend, onDeleteAll }) {
  const [text, setText] = useState("");

  function handleSubmit(e) {
    e.preventDefault();
    if (!text.trim()) return;
    onSend(text.trim());
    setText("");
  }

  return (
    <div className="flex flex-col gap-2">
      <div className="flex justify-between">
        <DeleteButton onClick={onDeleteAll}></DeleteButton>
        <DownButton></DownButton>
      </div>
      <form onSubmit={handleSubmit} className="bg-gray-900 rounded-2xl">
        <div className="flex p-2 shadow-xl">
          <input
            value={text}
            onChange={(e) => setText(e.target.value)}
            placeholder="Type a message..."
            className="flex-1 mx-2 rounded-md shadow-2sx border-none focus:border-none focus:outline-none"
          />
          <button
            type="submit"
            className="px-4 py-2 bg-gray-700 rounded-xl ml-4"
          >
            <div className="flex p-2 shadow-xl">
              <input
                value={text}
                onChange={(e) => setText(e.target.value)}
                placeholder="Type a message..."
                className="flex-1 mx-2 rounded-md shadow-2sx border-none focus:border-none focus:outline-none"
              />
              <motion.button
                type="submit"
                className="flex gap-2 px-4 py-2 bg-gray-700 rounded-xl ml-4"
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
