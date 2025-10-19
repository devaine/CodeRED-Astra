import React, { forwardRef, useRef } from "react";
import { motion } from "motion/react";

// Hidden file input that exposes an open() method via ref
const SchematicButton = forwardRef(function SchematicButton({ onFiles }, ref) {
  const inputRef = useRef(null);

  React.useImperativeHandle(ref, () => ({
    open: () => inputRef.current && inputRef.current.click(),
  }));

  function handleFiles(e) {
    const files = Array.from(e.target.files || []);
    if (files.length === 0) return;
    if (onFiles) onFiles(files);
    if (inputRef.current) inputRef.current.value = null;
  }

  return (
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
  );
});

export default SchematicButton;
