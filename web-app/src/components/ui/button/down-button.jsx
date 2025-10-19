import React from "react";
import { ArrowDown } from "lucide-react";

export default function DownButton({ onClick }) {
  return (
    <button onClick={onClick} className="bg-gray-700 p-2 rounded-2xl">
      <ArrowDown />
    </button>
  );
}
