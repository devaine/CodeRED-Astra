import { Flame } from "lucide-react";

export default function FlameButton({ onClick }) {
  return (
    <button onClick={onClick} className="bg-gray-700 p-2 rounded-2xl">
      <Flame />
    </button>
  );
}
