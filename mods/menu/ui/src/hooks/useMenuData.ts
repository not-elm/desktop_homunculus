import { useEffect, useState } from "react";
import { Webview, mods } from "@hmcs/sdk";

export interface MenuItemData {
  id: string;
  modName: string;
  text: string;
  command?: string;
}

export function useMenuData() {
  const [characterId, setCharacterId] = useState<string | null>(null);
  const [characterName, setCharacterName] = useState<string>("");
  const [items, setItems] = useState<MenuItemData[]>([]);

  useEffect(() => {
    const webview = Webview.current();
    if (!webview) return;

    let cancelled = false;

    (async () => {
      const character = await webview.linkedCharacter();
      if (cancelled) return;
      if (character) {
        setCharacterId(character.characterId);
        try {
          const name = await character.name();
          if (!cancelled) setCharacterName(name);
        } catch {
          /* name is optional for the HUD */
        }
      }

      const menuItems = await mods.menus();
      if (cancelled) return;
      setItems(menuItems);
    })();

    return () => {
      cancelled = true;
    };
  }, []);

  return { characterId, characterName, items };
}
