import { useRef } from "react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@hmcs/ui";
import { useMenuData } from "./hooks/useMenuData";
import { useMenuActions } from "./hooks/useMenuActions";

export function App() {
  const { personaId, characterName, items } = useMenuData();
  const { closing, handleClose, handleSelect } = useMenuActions(personaId);
  const contentRef = useRef<HTMLDivElement>(null);

  if (items.length === 0) return null;

  const useGrid = items.length < 7;

  return (
    <DropdownMenu open={!closing} modal={false}>
      <DropdownMenuTrigger className="sr-only" />
      <DropdownMenuContent
        ref={contentRef}
        className="menu-hud holo-refract-border holo-noise shadow-none"
        onEscapeKeyDown={handleClose}
        onPointerDownOutside={handleClose}
        sideOffset={0}
        align="start"
      >
        {/* Decorative layers */}
        <div className="menu-hud-highlight" />
        <div className="menu-hud-bottom-line" />
        <span className="menu-hud-corner menu-hud-corner--tl" />
        <span className="menu-hud-corner menu-hud-corner--tr" />
        <span className="menu-hud-corner menu-hud-corner--bl" />
        <span className="menu-hud-corner menu-hud-corner--br" />

        {/* Character status bar */}
        {characterName && (
          <>
            <div className="menu-status-bar">
              <span className="menu-status-name">{characterName}</span>
            </div>
            <div className="menu-separator" />
          </>
        )}

        {/* Action card grid */}
        <div
          className={
            useGrid ? "menu-card-grid" : "menu-card-grid menu-card-grid--list"
          }
        >
          {items.map((item, i) => (
            <DropdownMenuItem
              key={item.id}
              className="menu-card menu-card-stagger"
              style={{ "--i": i } as React.CSSProperties}
              onSelect={() => handleSelect(item)}
            >
              <span className="menu-card-label">{item.text}</span>
            </DropdownMenuItem>
          ))}
        </div>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
