import { useCallback, useRef, useState } from "react";
import { Webview, mods, signals } from "@hmcs/sdk";
import type { MenuItemData } from "./useMenuData";

export function useMenuActions(entity: number | null) {
  const [closing, setClosing] = useState(false);

  const handleClose = useCallback(async () => {
    if (closing) return;
    setClosing(true);
    setTimeout(async () => {
      const webviewEntity = Webview.current()?.entity;
      if (webviewEntity != null) {
        try {
          await signals.send("menu:close", { entity: webviewEntity });
        } catch (err) {
          console.error("Failed to send close signal:", err);
        }
      }
    }, 180);
  }, [closing]);

  const selectedRef = useRef(false);

  const handleSelect = useCallback(
    (item: MenuItemData) => {
      if (selectedRef.current) return;
      selectedRef.current = true;
      handleClose();
      if (item.command && entity != null) {
        mods
          .executeCommand({
            command: item.command,
            stdin: JSON.stringify({ linkedVrm: entity }),
          })
          .catch((err) => {
            console.error("Command execution failed:", err);
          });
      }
    },
    [entity, handleClose],
  );

  return { closing, handleClose, handleSelect };
}
