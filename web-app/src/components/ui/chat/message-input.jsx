import React, { useState } from "react";

export default function MessageInput({ onSend }) {
  const [text, setText] = useState("");

  function handleSubmit(e) {
    e.preventDefault();
    if (!text.trim()) return;
    onSend(text.trim());
    setText("");
  }

  return (
    <form onSubmit={handleSubmit} className="bg-gray-900 rounded-2xl">
      <div className="flex p-4 shadow-xl">
        <input
          value={text}
          onChange={(e) => setText(e.target.value)}
          placeholder="Type a message..."
          className="flex-1 rounded-md shadow-2sx border-none focus:border-none focus:outline-none"
        />
        <button type="submit" className="">
          Send
        </button>
      </div>
    </form>
  );
}
