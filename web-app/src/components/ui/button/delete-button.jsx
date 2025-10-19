import { Flame } from "lucide-react";
import { motion } from "motion/react";

export default function FlameButton({ onClick }) {
  return (
    <motion.button
      onClick={onClick}
      className="bg-gray-700 p-2 rounded-2xl"
      whileHover={{ scale: 1.1 }}
      whileTap={{ scale: 0.9 }}
    >
      <Flame />
    </motion.button>
  );
}
