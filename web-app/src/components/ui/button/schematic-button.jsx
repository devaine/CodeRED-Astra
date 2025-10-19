import React, { useState, useRef } from "react";
import { X } from "lucide-react";
import { motion } from "motion/react";
import { FilePlus2 } from "lucide-react";

export default function SchematicButton({ onFiles }) {
  const [filesList, setFilesList] = useState([]);
  const inputRef = useRef(null);

  function handleFiles(e) {
    const files = Array.from(e.target.files || []);
    if (files.length === 0) return;

    setFilesList((s) => [...s, ...files]);
    if (onFiles) onFiles(files);
    if (inputRef.current) inputRef.current.value = null;
  }

  function removeFile(index) {
    setFilesList((s) => {
      const copy = [...s];
      copy.splice(index, 1);
      return copy;
    });
  }

  return (
    <div className="flex items-center gap-2">
      <label className="relative inline-block">
        <motion.input
          ref={inputRef}
          type="file"
          accept="image/*,application/pdf"
          multiple
          onChange={handleFiles}
          className="file-input hidden"
          whileHover={{ scale: 1.02 }}
          whileTap={{ scale: 0.98 }}
        />
        <motion.div
          className="bg-gray-700 p-2 rounded-2xl cursor-pointer border-2 border-gray-600"
          whileHover={{ scale: 1.1 }}
          whileTap={{ scale: 0.9 }}
          onClick={() => inputRef.current && inputRef.current.click()}
        >
          <FilePlus2 />
        </motion.div>
      </label>

      {filesList.length > 0 && (
        <div className="flex gap-2 items-center max-w-xs flex-wrap">
          {filesList.map((f, i) => (
            <div
              key={i}
              className="flex items-center gap-2 bg-gray-800 text-sm text-slate-200 px-2 py-1 rounded"
            >
              <span className="truncate max-w-[10rem]">{f.name}</span>
              <button
                onClick={() => removeFile(i)}
                className="bg-gray-900 text-white rounded-full w-6 h-6 flex items-center justify-center text-xs border-2 border-gray-700"
                aria-label="Remove file"
              >
                <X />
              </button>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
