import React from "react";
import DeleteButton from "src/components/ui/button/delete-button";
import SchematicButton from "../button/schematic-button";

export default function ChatHeader({ title = "Title of Chat" }) {
  return (
    <div className="w-full flex justify-center">
      <header className="text-slate-100 fixed top-4 max-w-3xl w-full px-4">
        <div className="flex justify-between items-center gap-4">
          <SchematicButton></SchematicButton>
          <h1 className="text-lg font-semibold shadow-md shadow-indigo-600 bg-gray-900 px-6 py-2 rounded-4xl border-2 border-gray-800">
            {title}
          </h1>
          <DeleteButton></DeleteButton>
        </div>
      </header>
    </div>
  );
}
