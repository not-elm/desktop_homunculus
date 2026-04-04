import { useCallback, useRef } from "react";

interface UseTreeKeyboardOptions {
  onSelect: (element: HTMLElement) => void;
}

interface UseTreeKeyboardReturn {
  treeRef: React.RefObject<HTMLDivElement | null>;
  handleKeyDown: (e: React.KeyboardEvent) => void;
  focusItem: (element: HTMLElement) => void;
}

export function useTreeKeyboard({ onSelect }: UseTreeKeyboardOptions): UseTreeKeyboardReturn {
  const treeRef = useRef<HTMLDivElement | null>(null);

  const getVisibleItems = useCallback((): HTMLElement[] => {
    if (!treeRef.current) return [];
    const all = Array.from(treeRef.current.querySelectorAll<HTMLElement>('[role="treeitem"]'));
    return all.filter(isVisible);
  }, []);

  const focusItem = useCallback((element: HTMLElement) => {
    if (!treeRef.current) return;
    const items = treeRef.current.querySelectorAll<HTMLElement>('[role="treeitem"]');
    for (const item of items) {
      item.setAttribute("tabindex", item === element ? "0" : "-1");
    }
    element.focus();
  }, []);

  const handleKeyDown = useCallback((e: React.KeyboardEvent) => {
    const items = getVisibleItems();
    if (items.length === 0) return;

    const current = document.activeElement as HTMLElement;
    const index = items.indexOf(current);

    switch (e.key) {
      case "ArrowDown": {
        e.preventDefault();
        const next = items[index + 1] ?? items[0];
        focusItem(next);
        break;
      }
      case "ArrowUp": {
        e.preventDefault();
        const prev = items[index - 1] ?? items[items.length - 1];
        focusItem(prev);
        break;
      }
      case "ArrowRight": {
        e.preventDefault();
        handleArrowRight(current, items);
        break;
      }
      case "ArrowLeft": {
        e.preventDefault();
        handleArrowLeft(current);
        break;
      }
      case "Enter": {
        e.preventDefault();
        onSelect(current);
        break;
      }
      case " ": {
        const target = e.target as HTMLElement;
        if (target.getAttribute("role") !== "treeitem") break;
        e.preventDefault();
        if (current.getAttribute("aria-expanded") != null) {
          current.click();
        }
        break;
      }
      case "Home": {
        e.preventDefault();
        if (items.length > 0) focusItem(items[0]);
        break;
      }
      case "End": {
        e.preventDefault();
        if (items.length > 0) focusItem(items[items.length - 1]);
        break;
      }
    }
  }, [getVisibleItems, focusItem, onSelect]);

  function handleArrowRight(current: HTMLElement, items: HTMLElement[]) {
    const expanded = current.getAttribute("aria-expanded");
    if (expanded === "false") {
      current.click();
      return;
    }
    if (expanded === "true") {
      const idx = items.indexOf(current);
      const next = items[idx + 1];
      if (next && getAriaLevel(next) > getAriaLevel(current)) {
        focusItem(next);
      }
    }
  }

  function handleArrowLeft(current: HTMLElement) {
    const expanded = current.getAttribute("aria-expanded");
    if (expanded === "true") {
      current.click();
      return;
    }
    const parent = findParentTreeItem(current);
    if (parent) focusItem(parent);
  }

  return { treeRef, handleKeyDown, focusItem };
}

// Supports 2-level tree (workspace > worktree). For deeper nesting, walk all ancestor groups.
function isVisible(el: HTMLElement): boolean {
  const group = el.closest('[role="group"]');
  if (!group) return true;
  const parentItem = group.closest('[role="treeitem"]');
  if (parentItem && parentItem.getAttribute("aria-expanded") === "false") return false;
  return true;
}

function getAriaLevel(el: HTMLElement): number {
  return Number(el.getAttribute("aria-level")) || 0;
}

function findParentTreeItem(el: HTMLElement): HTMLElement | null {
  const group = el.closest('[role="group"]');
  if (!group) return null;
  const parent = group.closest('[role="treeitem"]');
  return parent as HTMLElement | null;
}
