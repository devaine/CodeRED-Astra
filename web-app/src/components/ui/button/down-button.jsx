import React from "react";
import { ArrowDown } from "lucide-react";
import { motion } from "motion/react";

export default function DownButton({ onClick }) {
  return (
    <motion.button
      onClick={onClick}
      className="bg-gray-700 p-2 rounded-2xl"
      whileHover={{ scale: 1.1 }}
      whileTap={{ scale: 0.9 }}
    >
      <ArrowDown />
    </motion.button>
  );
}
