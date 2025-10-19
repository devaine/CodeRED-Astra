import React from "react";
import { motion } from "motion/react";
import { FilePlus2 } from "lucide-react";

export default function SchematicButton({ onClick }) {
  return (
    <motion.button
      onClick={onClick}
      className=" bg-gray-700 p-2 rounded-2xl"
      whileHover={{ scale: 1.1 }}
      whileTap={{ scale: 0.9 }}
    >
      <FilePlus2 />
    </motion.button>
  );
}
