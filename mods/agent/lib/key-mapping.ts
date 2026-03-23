/**
 * Maps browser `KeyboardEvent.code` strings to `uiohook-napi` keycodes.
 *
 * Browser codes follow the UI Events spec (e.g., `"Space"`, `"KeyA"`, `"ShiftLeft"`).
 * uiohook keycodes come from libuiohook and match `UiohookKey` in `uiohook-napi`.
 */
const BROWSER_CODE_TO_UIOHOOK: Record<string, number> = {
  // Navigation & editing
  Backspace: 14,
  Tab: 15,
  Enter: 28,
  CapsLock: 58,
  Escape: 1,
  Space: 57,
  PageUp: 3657,
  PageDown: 3665,
  End: 3663,
  Home: 3655,
  Insert: 3666,
  Delete: 3667,

  // Arrow keys
  ArrowLeft: 57419,
  ArrowUp: 57416,
  ArrowRight: 57421,
  ArrowDown: 57424,

  // Digits
  Digit0: 11,
  Digit1: 2,
  Digit2: 3,
  Digit3: 4,
  Digit4: 5,
  Digit5: 6,
  Digit6: 7,
  Digit7: 8,
  Digit8: 9,
  Digit9: 10,

  // Letters
  KeyA: 30,
  KeyB: 48,
  KeyC: 46,
  KeyD: 32,
  KeyE: 18,
  KeyF: 33,
  KeyG: 34,
  KeyH: 35,
  KeyI: 23,
  KeyJ: 36,
  KeyK: 37,
  KeyL: 38,
  KeyM: 50,
  KeyN: 49,
  KeyO: 24,
  KeyP: 25,
  KeyQ: 16,
  KeyR: 19,
  KeyS: 31,
  KeyT: 20,
  KeyU: 22,
  KeyV: 47,
  KeyW: 17,
  KeyX: 45,
  KeyY: 21,
  KeyZ: 44,

  // Numpad
  Numpad0: 82,
  Numpad1: 79,
  Numpad2: 80,
  Numpad3: 81,
  Numpad4: 75,
  Numpad5: 76,
  Numpad6: 77,
  Numpad7: 71,
  Numpad8: 72,
  Numpad9: 73,
  NumpadMultiply: 55,
  NumpadAdd: 78,
  NumpadSubtract: 74,
  NumpadDecimal: 83,
  NumpadDivide: 3637,
  NumpadEnter: 3612,

  // Function keys
  F1: 59,
  F2: 60,
  F3: 61,
  F4: 62,
  F5: 63,
  F6: 64,
  F7: 65,
  F8: 66,
  F9: 67,
  F10: 68,
  F11: 87,
  F12: 88,
  F13: 91,
  F14: 92,
  F15: 93,
  F16: 99,
  F17: 100,
  F18: 101,
  F19: 102,
  F20: 103,
  F21: 104,
  F22: 105,
  F23: 106,
  F24: 107,

  // Punctuation & symbols
  Semicolon: 39,
  Equal: 13,
  Comma: 51,
  Minus: 12,
  Period: 52,
  Slash: 53,
  Backquote: 41,
  BracketLeft: 26,
  Backslash: 43,
  BracketRight: 27,
  Quote: 40,

  // Modifier keys
  ControlLeft: 29,
  ControlRight: 3613,
  AltLeft: 56,
  AltRight: 3640,
  ShiftLeft: 42,
  ShiftRight: 54,
  MetaLeft: 3675,
  MetaRight: 3676,

  // Lock & misc
  NumLock: 69,
  ScrollLock: 70,
  PrintScreen: 3639,
};

/** Maps modifier names to their Left/Right uiohook keycodes. */
const MODIFIER_TO_UIOHOOK: Record<string, readonly number[]> = {
  ctrl: [29, 3613],
  shift: [42, 54],
  alt: [56, 3640],
  meta: [3675, 3676],
};

export interface PttKeyInput {
  code: string;
  modifiers: string[];
}

export interface ResolvedPttKey {
  primaryKeycode: number;
  modifiers: ReadonlyArray<readonly number[]>;
}

/** Converts a browser `KeyboardEvent.code` to the corresponding uiohook keycode. */
export function resolveUiohookKeycode(browserCode: string): number | null {
  return BROWSER_CODE_TO_UIOHOOK[browserCode] ?? null;
}

/** Resolves a PttKey (primary + modifiers) to uiohook keycodes. */
export function resolvePttKeycodes(pttKey: PttKeyInput): ResolvedPttKey | null {
  const primary = BROWSER_CODE_TO_UIOHOOK[pttKey.code];
  if (primary === undefined) return null;
  const modifiers = pttKey.modifiers
    .map((m) => MODIFIER_TO_UIOHOOK[m])
    .filter((v): v is readonly number[] => v !== undefined);
  return { primaryKeycode: primary, modifiers };
}
