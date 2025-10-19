import { Flame } from "lucide-react";
import { motion } from "motion/react";

export default function FlameButton({ onClick, disabled = false }) {
  return (
    <motion.button
      onClick={onClick}
      className={`bg-gray-700 p-2 rounded-2xl border-2 border-gray-600 ${
        disabled ? "cursor-not-allowed" : "cursor-pointer"
      }`}
      whileHover={disabled ? undefined : { scale: 1.1 }}
      whileTap={disabled ? undefined : { scale: 0.9 }}
      disabled={disabled}
      style={{ opacity: disabled ? 0.5 : 1 }}
    >
      <Flame />
    </motion.button>
  );
}
