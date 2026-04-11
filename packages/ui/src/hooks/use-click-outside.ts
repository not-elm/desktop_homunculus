import { type RefObject, useEffect } from 'react';

/**
 * Calls `handler` when a mousedown event occurs outside the referenced element.
 *
 * @param ref - Ref to the element to monitor
 * @param handler - Callback to invoke on outside click
 *
 * @example
 * ```tsx
 * const ref = useRef<HTMLDivElement>(null);
 * useClickOutside(ref, () => setOpen(false));
 * ```
 */
export function useClickOutside(ref: RefObject<HTMLElement | null>, handler: () => void): void {
  useEffect(() => {
    function listener(e: MouseEvent) {
      if (ref.current && !ref.current.contains(e.target as Node)) {
        handler();
      }
    }
    document.addEventListener('mousedown', listener);
    return () => document.removeEventListener('mousedown', listener);
  }, [ref, handler]);
}
