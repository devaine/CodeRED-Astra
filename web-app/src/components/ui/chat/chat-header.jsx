import React from "react";

export default function ChatHeader({ title = "Title of Chat" }) {
  return (
    <header className="flex justify-center text-slate-100">
      <h1 className="text-lg font-semibold shadow-xl bg-gray-900 px-4 py-2 rounded-4xl">
        {title}
      </h1>
    </header>
  );
}
