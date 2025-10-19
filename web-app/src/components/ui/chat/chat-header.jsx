import React from "react";

export default function ChatHeader({ title = "Title of Chat" }) {
  return (
    <div className="w-full flex justify-center">
      <header className="text-slate-100 fixed top-2">
        <div>
          <h1 className="text-lg font-semibold shadow-xl bg-gray-900 px-6 py-2 rounded-4xl">
            {title}
          </h1>
        </div>
      </header>
    </div>
  );
}
