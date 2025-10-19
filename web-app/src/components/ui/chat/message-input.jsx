import React, { useState } from "react";
import DeleteButton from "src/components/ui/button/delete-button";
import DownButton from "src/components/ui/button/down-button";
import { MessageCirclePlus } from "lucide-react";

export default function MessageInput({ onSend }) {
  const [text, setText] = useState("");

  function handleSubmit(e) {
    e.preventDefault();
    if (!text.trim()) return;
    onSend(text.trim());
    setText("");
  }

  return (
    <div className="w-full flex justify-center">
      <footer className="fixed bottom-2 max-w-2xl w-full px-4">
        <div className="flex flex-col gap-2">
          <div className="flex justify-between">
            <DeleteButton></DeleteButton>
            <DownButton></DownButton>
          </div>
          <form onSubmit={handleSubmit} className="bg-gray-900 rounded-2xl">
            <div className="flex p-2 shadow-xl">
              <input
                value={text}
                onChange={(e) => setText(e.target.value)}
                placeholder="Type a message..."
                className="flex-1 mx-2 rounded-md shadow-2sx border-none focus:border-none focus:outline-none"
              />
              <button
                type="submit"
                className="flex gap-2 px-4 py-2 bg-gray-700 rounded-xl ml-4"
              >
                Send
              </button>
            </div>
          </form>
        </div>
      </footer>
    </div>
  );
}
