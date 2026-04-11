import type * as React from 'react';

import { cn } from '@/lib/utils';

/** Props for the {@link Toolbar} component. */
export interface ToolbarProps {
  /** Title text displayed on the left side of the toolbar. */
  title: string;
  /** Callback fired when the close button is clicked. */
  onClose: () => void;
  /** Optional content rendered between the title and the close button. */
  children?: React.ReactNode;
  /** Additional CSS class names for the toolbar root element. */
  className?: string;
}

/**
 * A draggable toolbar with a title, optional children, and a close button.
 *
 * Designed for use at the top of WebView panel UIs. The toolbar root is a
 * CSS drag region (`-webkit-app-region: drag`); the children area and close
 * button opt out so they remain interactive.
 *
 * @example
 * ```tsx
 * <Toolbar title="Settings" onClose={handleClose} />
 *
 * <Toolbar title="Persona" onClose={handleClose}>
 *   <TabButtons />
 * </Toolbar>
 * ```
 */
function Toolbar({ title, onClose, children, className }: ToolbarProps) {
  return (
    <div
      data-slot="toolbar"
      className={cn(
        'flex shrink-0 select-none items-center justify-between [app-region:drag] [-webkit-app-region:drag]',
        'border-b border-primary/12 bg-primary/4 px-3.5 py-1.5',
        className,
      )}
    >
      <span className="text-[10px] font-medium uppercase tracking-[0.08em] text-primary/45">
        {title}
      </span>

      <div className="flex items-center gap-2 [app-region:no-drag] [-webkit-app-region:no-drag]">
        {children}
        <button
          type="button"
          onClick={onClose}
          aria-label="Close"
          className={cn(
            'flex size-[22px] cursor-pointer items-center justify-center rounded-[4px]',
            'border border-primary/15 bg-primary/4 text-primary/35',
            'transition-all duration-200',
            'hover:border-primary/40 hover:bg-primary/8 hover:text-primary hover:shadow-[0_0_8px_oklch(0.72_0.14_192/0.15)]',
            'active:scale-[0.92]',
          )}
        >
          <svg
            aria-hidden="true"
            width="10"
            height="10"
            viewBox="0 0 10 10"
            fill="none"
            stroke="currentColor"
            strokeWidth="1.5"
          >
            <path d="M1 1l8 8M9 1L1 9" />
          </svg>
        </button>
      </div>
    </div>
  );
}

export { Toolbar };
