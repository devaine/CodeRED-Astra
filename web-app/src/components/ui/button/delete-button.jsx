import { Flame } from "lucide-react";
import { motion } from "motion/react";

export default function FlameButton({ onClick }) {
  return (
    <motion.button
      onClick={onClick}
      className="bg-gray-700 cursor-pointer p-2 rounded-2xl border-2 border-gray-600"
      whileHover={{ scale: 1.1 }}
      whileTap={{ scale: 0.9 }}
    >
      <Flame />
    </motion.button>
  );
}
