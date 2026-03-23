import { useState } from "react";
import { dialog } from "@hmcs/sdk";

interface DirectoryListFieldProps {
  label: string;
  description?: string;
  paths: string[];
  defaultIndex: number;
  onAdd: (path: string) => void;
  onRemove: (index: number) => void;
  onSetDefault: (index: number) => void;
}

export function DirectoryListField({
  label,
  description,
  paths,
  defaultIndex,
  onAdd,
  onRemove,
  onSetDefault,
}: DirectoryListFieldProps) {
  const [inputValue, setInputValue] = useState("");
  const [browsing, setBrowsing] = useState(false);

  function handleAdd() {
    const trimmed = inputValue.trim();
    if (!trimmed) return;
    onAdd(trimmed);
    setInputValue("");
  }

  function handleKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
    if (e.key === "Enter") handleAdd();
  }

  async function handleBrowse() {
    setBrowsing(true);
    try {
      const path = await dialog.pickFolder();
      if (path) {
        onAdd(path);
      }
    } finally {
      setBrowsing(false);
    }
  }

  return (
    <label className="settings-label">
      {label}
      {description && (
        <span className="settings-label-desc">{description}</span>
      )}
      <div className="agent-dir-list">
        {paths.map((path, i) => (
          <DirectoryItem
            key={i}
            path={path}
            isDefault={i === defaultIndex}
            onSetDefault={() => onSetDefault(i)}
            onRemove={() => onRemove(i)}
          />
        ))}
      </div>
      <div className="agent-add-row">
        <input
          className="agent-add-input"
          type="text"
          placeholder="Add directory path..."
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
          onKeyDown={handleKeyDown}
        />
        <button className="agent-add-btn" type="button" onClick={handleAdd}>
          Add
        </button>
        <button
          className="agent-add-btn"
          type="button"
          onClick={handleBrowse}
          disabled={browsing}
        >
          Browse
        </button>
      </div>
    </label>
  );
}

interface DirectoryItemProps {
  path: string;
  isDefault: boolean;
  onSetDefault: () => void;
  onRemove: () => void;
}

function DirectoryItem({
  path,
  isDefault,
  onSetDefault,
  onRemove,
}: DirectoryItemProps) {
  return (
    <div className="agent-dir-item">
      <input
        className="agent-dir-radio"
        type="radio"
        checked={isDefault}
        onChange={onSetDefault}
        aria-label={`Set ${path} as default`}
      />
      <span className="agent-dir-path" title={path}>
        {path}
      </span>
      <button
        className="agent-dir-remove"
        type="button"
        onClick={onRemove}
        aria-label={`Remove ${path}`}
      >
        ×
      </button>
    </div>
  );
}
