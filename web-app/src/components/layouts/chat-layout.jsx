import React, { useCallback, useEffect, useMemo, useRef, useState } from "react";
import ChatHeader from "src/components/ui/chat/chat-header";
import ChatWindow from "src/components/ui/chat/chat-window";
import MessageInput from "src/components/ui/chat/message-input";
import {
  createQuery,
  getQueryResult,
  getQueryStatus,
  listFiles,
} from "src/lib/api";

const createId = () =>
  (globalThis.crypto?.randomUUID?.() ?? `id-${Date.now()}-${Math.random()}`);

const INTRO_MESSAGE = {
  id: "intro",
  role: "assistant",
  content:
    "Ask me about the demo PDFs and I'll respond with the best matches pulled from the processed files.",
};

export default function ChatLayout() {
  const [messages, setMessages] = useState([INTRO_MESSAGE]);
  const [isProcessing, setIsProcessing] = useState(false);
  const [files, setFiles] = useState([]);
  const [errorToast, setErrorToast] = useState("");
  const pollAbortRef = useRef(null);

  const showError = useCallback((message) => {
    setErrorToast(message);
    window.setTimeout(() => setErrorToast(""), 5000);
  }, []);

  const refreshFiles = useCallback(async () => {
    try {
      const latest = await listFiles();
      setFiles(latest);
    } catch (error) {
      showError(error.message ?? "Failed to load files");
    }
  }, [showError]);

  useEffect(() => {
    refreshFiles();
  }, [refreshFiles]);

  useEffect(() => {
    return () => {
      if (pollAbortRef.current) {
        pollAbortRef.current.aborted = true;
      }
    };
  }, []);

  const buildAssistantMarkdown = useCallback((result) => {
    if (!result || typeof result !== "object") {
      return "I could not find a response for that request.";
    }

    const finalAnswer = result.final_answer?.trim();
    const relationships = result.relationships?.trim();
    const relatedFiles = Array.isArray(result.related_files)
      ? result.related_files
      : [];

    const fileLines = relatedFiles
      .filter((f) => f && typeof f === "object")
      .map((file) => {
        const filename = file.filename || file.id || "download";
        const linkTarget = `/storage/${encodeURIComponent(filename)}`;
        const description = file.description?.trim();
        const score =
          typeof file.score === "number"
            ? ` _(score: ${file.score.toFixed(3)})_`
            : "";
        const detail = description ? ` — ${description}` : "";
        return `- [${filename}](${linkTarget})${detail}${score}`;
      });

    let content =
      finalAnswer ||
      "I could not determine an answer from the indexed documents yet.";

    if (fileLines.length) {
      content += `\n\n**Related Files**\n${fileLines.join("\n")}`;
    }

    if (relationships && relationships !== finalAnswer) {
      content += `\n\n---\n${relationships}`;
    }

    if (!fileLines.length && (!finalAnswer || finalAnswer.length < 10)) {
      content +=
        "\n\n_No analyzed documents matched yet. Try seeding demo data or wait for processing to finish._";
    }

    return content;
  }, []);

  const waitForResult = useCallback(async (id) => {
    const abortState = { aborted: false };
    pollAbortRef.current = abortState;
    const timeoutMs = 120_000;
    const intervalMs = 1_500;
    const started = Date.now();

    while (!abortState.aborted) {
      if (Date.now() - started > timeoutMs) {
        throw new Error("Timed out waiting for the query to finish");
      }

      const statusPayload = await getQueryStatus(id);
      const status = statusPayload?.status;

      if (status === "Completed") {
        const resultPayload = await getQueryResult(id);
        return resultPayload?.result;
      }

      if (status === "Failed") {
        const resultPayload = await getQueryResult(id);
        const reason = resultPayload?.result?.error || "Query failed";
        throw new Error(reason);
      }

      if (status === "Cancelled") {
        throw new Error("Query was cancelled");
      }

      if (status === "not_found") {
        throw new Error("Query was not found");
      }

      await new Promise((resolve) => window.setTimeout(resolve, intervalMs));
    }

    throw new Error("Query polling was aborted");
  }, []);

  const handleSend = useCallback(
    async (text) => {
      if (isProcessing) {
        showError("Please wait for the current response to finish.");
        return;
      }

      const userEntry = {
        id: createId(),
        role: "user",
        content: text,
      };
      setMessages((prev) => [...prev, userEntry]);

      const placeholderId = createId();
      setMessages((prev) => [
        ...prev,
        {
          id: placeholderId,
          role: "assistant",
          content: "_Analyzing indexed documents..._",
          pending: true,
        },
      ]);

      setIsProcessing(true);

      try {
        const payload = { q: text, top_k: 5 };
        const created = await createQuery(payload);
        const result = await waitForResult(created.id);
        const content = buildAssistantMarkdown(result);
        setMessages((prev) =>
          prev.map((message) =>
            message.id === placeholderId
              ? { ...message, content, pending: false }
              : message
          )
        );
      } catch (error) {
        const message = error?.message || "Something went wrong.";
        setMessages((prev) =>
          prev.map((entry) =>
            entry.id === placeholderId
              ? {
                  ...entry,
                  content: `⚠️ ${message}`,
                  pending: false,
                  error: true,
                }
              : entry
          )
        );
        showError(message);
      } finally {
        pollAbortRef.current = null;
        setIsProcessing(false);
        refreshFiles();
      }
    },
    [
      isProcessing,
      showError,
      refreshFiles,
      waitForResult,
      buildAssistantMarkdown,
    ]
  );

  const handleDeleteAll = useCallback(() => {
    if (!window.confirm("Delete all messages?")) {
      return;
    }
    setMessages([INTRO_MESSAGE]);
  }, []);

  const latestFileSummary = useMemo(() => {
    if (!files.length) return "No files indexed yet.";
    const pending = files.filter((f) => f.pending_analysis).length;
    const ready = files.length - pending;
    return `${ready} ready • ${pending} processing`;
  }, [files]);

  return (
    <div className="flex flex-col flex-start w-full max-w-3xl gap-4 p-4">
      <ChatHeader
        onClear={handleDeleteAll}
        busy={isProcessing}
        fileSummary={latestFileSummary}
        errorMessage={errorToast}
      />
      <ChatWindow messages={messages} />
      <MessageInput onSend={handleSend} disabled={isProcessing} />
    </div>
  );
}
