import React, { useMemo, useState } from "react";
import { motion } from "motion/react";
import { Rocket } from "lucide-react";
import DeleteButton from "src/components/ui/button/delete-button";
import FileList from "src/components/ui/file/file-list";

export default function ChatHeader({
  title = "Schematic Spelunker",
  onDeleteAll,
}) {
  const isDebug = useMemo(() => {
    const p = new URLSearchParams(window.location.search);
    return p.get("debug") === "1";
  }, []);
  const [ingesting, setIngesting] = useState(false);
  const [toast, setToast] = useState("");

  async function triggerDemoIngest() {
    try {
      setIngesting(true);
      const res = await fetch("/api/files/import-demo", { method: "POST" });
      const json = await res.json().catch(() => ({}));
      setToast(
        `Imported: ${json.imported ?? "?"}, Skipped: ${json.skipped ?? "?"}`
      );
      setTimeout(() => setToast(""), 4000);
    } catch (e) {
      setToast("Import failed");
      setTimeout(() => setToast(""), 4000);
    } finally {
      setIngesting(false);
    }
  }

  return (
    <div className="w-full flex justify-center">
      <header className="text-slate-100 fixed top-4 max-w-3xl w-full px-4">
        <div className="flex justify-between items-center gap-4">
          <FileList />
          <h1 className=" text-sm lg:text-lg font-semibold shadow-md shadow-indigo-600 bg-gray-900 px-6 py-2 rounded-4xl border-2 border-gray-800">
            {title}
          </h1>
          <DeleteButton onClick={onDeleteAll} />
          {isDebug && (
            <motion.button
              onClick={triggerDemoIngest}
              className="bg-gray-800 border-2 border-gray-700 rounded-xl px-3 py-2 flex items-center gap-2"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              disabled={ingesting}
            >
              <Rocket size={16} />
              {ingesting ? "Seeding…" : "Seed Demo Data"}
            </motion.button>
          )}
        </div>
        {toast && (
          <div className="mt-2 text-xs text-slate-300 bg-gray-800/80 border border-gray-700 rounded px-2 py-1 inline-block">
            {toast}
          </div>
        )}
      </header>
    </div>
  );
}
