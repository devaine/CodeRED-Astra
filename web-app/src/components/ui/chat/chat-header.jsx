import React from "react";

export default function ChatHeader({ title = "Title of Chat" }) {
  return (
    <header className="flex justify-center bg-blue-600">
      <h1 className="text-lg font-semibold">{title}</h1>
    </header>
  );
}
