import type { PttKey } from "../settings/hooks/useSettingsDraft";

const MODIFIER_DISPLAY: Record<string, string> = {
  ctrl: "Ctrl",
  shift: "Shift",
  alt: "Alt",
  meta: "Cmd",
};

/** Formats a PttKey for display, e.g. "Ctrl + Space". */
export function formatPttKeyName(key: PttKey): string {
  const modLabels = key.modifiers.map((m) => MODIFIER_DISPLAY[m] ?? m);
  const keyLabel = formatKeyCode(key.code);
  return [...modLabels, keyLabel].join(" + ");
}

/** Converts a browser KeyboardEvent.code to a user-friendly display name. */
function formatKeyCode(code: string): string {
  if (code.startsWith("Key")) return code.slice(3);
  if (code.startsWith("Digit")) return code.slice(5);
  return code.replace(/(Left|Right)$/, " ($1)");
}
