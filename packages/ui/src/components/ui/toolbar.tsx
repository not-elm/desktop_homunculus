import { ChevronLeft, ChevronRight } from 'lucide-react';
import type * as React from 'react';

import { cn } from '@/lib/utils';

/** Navigation button configuration for the Toolbar. */
export interface ToolbarNavigationProps {
  /** Whether the back button is enabled. */
  canGoBack: boolean;
  /** Whether the forward button is enabled. */
  canGoForward: boolean;
  /** Callback fired when the back button is clicked. */
  onBack: () => void;
  /** Callback fired when the forward button is clicked. */
  onForward: () => void;
}

/** Props for the {@link Toolbar} component. */
export interface ToolbarProps {
  /** Title text displayed in the toolbar. */
  title: string;
  /** Callback fired when the close button is clicked. */
  onClose: () => void;
  /** Optional content rendered between the title and the close button. */
  children?: React.ReactNode;
  /** Additional CSS class names for the toolbar root element. */
  className?: string;
  /** Optional navigation buttons. When provided, shows Back/Forward buttons before the title. */
  navigation?: ToolbarNavigationProps;
}

const toolbarButtonClass = cn(
  'flex size-[22px] cursor-pointer items-center justify-center rounded-[4px]',
  'border border-primary/15 bg-primary/4 text-primary/35',
  'transition-all duration-200',
  'hover:border-primary/40 hover:bg-primary/8 hover:text-primary hover:shadow-[0_0_8px_oklch(0.72_0.14_192/0.15)]',
  'active:scale-[0.92]',
);

const disabledButtonClass = 'opacity-30 cursor-not-allowed hover:border-primary/15 hover:bg-primary/4 hover:text-primary/35 hover:shadow-none active:scale-100';

/**
 * A draggable toolbar with a title, optional navigation, optional children, and a close button.
 *
 * Designed for use at the top of WebView panel UIs. The toolbar root is a
 * CSS drag region (`-webkit-app-region: drag`); interactive elements opt out
 * so they remain clickable.
 *
 * @example
 * ```tsx
 * <Toolbar title="Settings" onClose={handleClose} />
 *
 * <Toolbar title="Agent" onClose={handleClose} navigation={{
 *   canGoBack: true, canGoForward: false,
 *   onBack: handleBack, onForward: handleForward,
 * }} />
 * ```
 */
function Toolbar({ title, onClose, children, className, navigation }: ToolbarProps) {
  return (
    <div
      data-slot="toolbar"
      className={cn(
        'flex shrink-0 select-none items-center justify-between [app-region:drag] [-webkit-app-region:drag]',
        'border-b border-primary/12 bg-primary/4 px-3.5 py-1.5',
        className,
      )}
    >
      <div className="flex items-center gap-2 [app-region:no-drag] [-webkit-app-region:no-drag]">
        {navigation && <NavigationButtons navigation={navigation} />}
        <span className="text-[10px] font-medium uppercase tracking-[0.08em] text-primary/45">
          {title}
        </span>
      </div>

      <div className="flex items-center gap-2 [app-region:no-drag] [-webkit-app-region:no-drag]">
        {children}
        <CloseButton onClick={onClose} />
      </div>
    </div>
  );
}

function NavigationButtons({ navigation }: { navigation: ToolbarNavigationProps }) {
  return (
    <div className="flex items-center gap-0.5">
      <button
        type="button"
        onClick={navigation.canGoBack ? navigation.onBack : undefined}
        disabled={!navigation.canGoBack}
        aria-label="Go back"
        className={cn(toolbarButtonClass, !navigation.canGoBack && disabledButtonClass)}
      >
        <ChevronLeft size={12} />
      </button>
      <button
        type="button"
        onClick={navigation.canGoForward ? navigation.onForward : undefined}
        disabled={!navigation.canGoForward}
        aria-label="Go forward"
        className={cn(toolbarButtonClass, !navigation.canGoForward && disabledButtonClass)}
      >
        <ChevronRight size={12} />
      </button>
    </div>
  );
}

function CloseButton({ onClick }: { onClick: () => void }) {
  return (
    <button
      type="button"
      onClick={onClick}
      aria-label="Close"
      className={toolbarButtonClass}
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
  );
}

export { Toolbar };
