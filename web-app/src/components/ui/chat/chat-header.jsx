import React, { useEffect, useMemo, useState } from "react";
import { motion } from "motion/react";
import { Rocket } from "lucide-react";
import DeleteButton from "src/components/ui/button/delete-button";
import SchematicButton from "../button/schematic-button";

export default function ChatHeader({
  title = "Title of Chat",
  onClear,
  busy = false,
  fileSummary,
  errorMessage,
}) {
  const isDebug = useMemo(() => {
    const p = new URLSearchParams(window.location.search);
    return p.get("debug") === "1";
  }, []);
  const [ingesting, setIngesting] = useState(false);
  const [toast, setToast] = useState("");
  const [externalToast, setExternalToast] = useState("");

  useEffect(() => {
    if (!errorMessage) return;
    setExternalToast(errorMessage);
    const timer = window.setTimeout(() => setExternalToast(""), 5000);
    return () => window.clearTimeout(timer);
  }, [errorMessage]);

  async function triggerDemoIngest() {
    try {
      setIngesting(true);
      const res = await fetch("/api/files/import-demo", { method: "POST" });
      const json = await res.json().catch(() => ({}));
      const imported = json.imported ?? "?";
      const skipped = json.skipped ?? "?";
      const summary = `Imported: ${imported}, Skipped: ${skipped}`;
      setToast(json.error ? `${summary} - ${json.error}` : summary);
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
          <SchematicButton />
          <div className="flex items-center gap-3">
            <h1 className="text-lg font-semibold shadow-md shadow-indigo-600 bg-gray-900 px-6 py-2 rounded-4xl border-2 border-gray-800">
              {title}
            </h1>
            {fileSummary && (
              <div className="text-xs text-slate-300 bg-gray-800/80 border border-gray-700 rounded px-3 py-1">
                {fileSummary}
              </div>
            )}
            <DeleteButton onClick={onClear} disabled={busy} />
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
        </div>
        {toast && (
          <div className="mt-2 text-xs text-slate-300 bg-gray-800/80 border border-gray-700 rounded px-2 py-1 inline-block">
            {toast}
          </div>
        )}
        {externalToast && (
          <div className="mt-2 text-xs text-red-300 bg-red-900/40 border border-red-700 rounded px-2 py-1 inline-block">
            {externalToast}
          </div>
        )}
      </header>
    </div>
  );
}
