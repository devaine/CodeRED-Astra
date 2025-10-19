import React, { useRef, useState } from "react";
import SchematicButton from "src/components/ui/button/schematic-button";
import { motion } from "motion/react";
import { Menu } from "lucide-react";
import { X } from "lucide-react";
import { FilePlus2 } from "lucide-react";

export default function FileList() {
  const pickerRef = useRef(null);
  const [open, setOpen] = useState(false);
  const [files, setFiles] = useState([]);

  function handleAdd() {
    if (pickerRef.current && pickerRef.current.open) pickerRef.current.open();
  }

  function handleFiles(selected) {
    setFiles((s) => [...s, ...selected]);
    setOpen(true);
  }

  function removeFile(i) {
    setFiles((s) => s.filter((_, idx) => idx !== i));
  }

  return (
    <div className="h-full flex flex-col gap-2">
      <div className="flex items-center justify-between px-2">
        <motion.button
          onClick={() => setOpen((v) => !v)}
          className="p-2 rounded-xl bg-gray-700 border-2 border-gray-600"
          aria-expanded={open}
          whileHover={{ scale: 1.1 }}
          whileTab={{ scale: 0.9 }}
        >
          {open ? <X /> : <Menu />}
        </motion.button>
      </div>

      {open && (
        <div className="fixed left-1/2 top-24 transform -translate-x-1/2 z-50 w-full max-w-3xl px-4">
          <div className="bg-gray-900 border-2 border-gray-800 rounded-2xl p-4 shadow-lg overflow-auto">
            <div className="flex items-center justify-between mb-2 pr-1">
              <div className="text-lg font-medium">Files</div>
              <div>
                <motion.button
                  onClick={handleAdd}
                  className="w-full bg-gray-700 text-sm p-2 rounded-full border-2 border-gray-600"
                  whileHover={{ scale: 1.1 }}
                  whileTap={{ scale: 0.9 }}
                >
                  <FilePlus2 />
                </motion.button>
                <SchematicButton ref={pickerRef} onFiles={handleFiles} />
              </div>
            </div>

            <div className="flex flex-col gap-2">
              {files.length === 0 ? (
                <div className="text-md text-slate-400">No files added</div>
              ) : (
                files.map((f, i) => (
                  <div
                    key={i}
                    className="flex items-center justify-between bg-gray-800 p-2 rounded-lg text-sm"
                  >
                    <span className="truncate max-w-[24rem]">{f.name}</span>
                    <button
                      onClick={() => removeFile(i)}
                      className="text-xs bg-gray-700 rounded-full p-2"
                    >
                      <X />
                    </button>
                  </div>
                ))
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
