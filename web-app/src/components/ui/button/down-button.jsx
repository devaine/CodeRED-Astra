import React from "react";
import { ArrowDown } from "lucide-react";
import { motion } from "motion/react";

export default function DownButton({ onClick }) {
  function handleClick(e) {
    if (onClick) return onClick(e);
    // default behavior: scroll to bottom of page smoothly
    const doc = document.documentElement;
    const top = Math.max(doc.scrollHeight, document.body.scrollHeight);
    window.scrollTo({ top, behavior: "smooth" });
  }

  return (
    <motion.button
      onClick={handleClick}
      className="bg-gray-700 p-2 rounded-2xl file-input border-2 border-gray-600"
      whileHover={{ scale: 1.1 }}
      whileTap={{ scale: 0.9 }}
    >
      <ArrowDown />
    </motion.button>
  );
}
