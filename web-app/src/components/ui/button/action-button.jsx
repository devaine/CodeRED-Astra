import "./action-button.css";

export default function ActionButton({
  onClick,
  children,
  type = "add", // 'add' or 'delete'
  ...props
}) {
  return (
    <button
      onClick={onClick}
      className="action-btn"
      style={{ "--btn-color": color }}
      {...props}
    >
      {type === "add" ? "New Chat" : "Delete Chat"}
      {svg}
    </button>
  );
}
