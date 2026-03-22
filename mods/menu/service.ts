import {
  Character,
  Vrm,
  Webview,
  audio,
  signals,
  isWebviewSourceInfoLocal,
  webviewSource,
} from "@hmcs/sdk";
const menuUIAssetId = "menu:ui";
let isProcessing = false;
const eventSources = new Map();

const existsLinkedWebview = async (characterId: string) => {
  const webviews = await Webview.list();
  for (let webview of webviews) {
    if (webview.linkedCharacter === characterId) {
      return true;
    }
  }
  return false;
};

const findCharacterIdByEntity = async (
  entity: number,
): Promise<string | undefined> => {
  const characters = await Character.findAll();
  console.log("COUNT", characters.length);
  console.log("Entity", entity);
  characters.forEach((s) => {
    console.log(s.name, s.entity);
  });
  return characters.find((c) => c.entity === entity)?.id;
};

const openedMenu = async () => {
  const webviews = await Webview.list();
  for (let webview of webviews) {
    if (
      isWebviewSourceInfoLocal(webview.source) &&
      webview.source.id === menuUIAssetId
    ) {
      return new Webview(webview.entity);
    }
  }
  return null;
};

signals.stream<{ entity: number }>("menu:close", async (payload) => {
  try {
    const webview = new Webview(payload.entity);
    await webview.close();
  } catch (err) {
    console.error("Failed to close menu:", err);
  }
});

Vrm.stream(async (vrm) => {
  // Close existing EventSource for this VRM before creating a new one.
  // Vrm.stream() replays existing VRMs on SSE reconnection, which would
  // otherwise accumulate duplicate listeners.
  const oldEs = eventSources.get(vrm.entity);
  if (oldEs) oldEs.close();

  const es = await vrm.events();
  eventSources.set(vrm.entity, es);

  es.on("pointer-click", async (e) => {
    if (e.button !== "Secondary") return;
    if (isProcessing) return;
    isProcessing = true;
    try {
      const currentMenu = await openedMenu();
      if (currentMenu) {
        await audio.se.play("se:close");
        await currentMenu.close();
        return;
      }
      const characterId = await findCharacterIdByEntity(vrm.entity);
      console.log("++++++++++", characterId);
      if (!characterId) return;
      if (await existsLinkedWebview(characterId)) {
        return;
      }

      await Webview.open({
        source: webviewSource.local("menu:ui"),
        size: [0.8, 1],
        viewportSize: [500, 600],
        offset: [1, -0.3],
        linkedCharacter: characterId,
      });
      await audio.se.play("se:open");
    } catch (err) {
      console.error("Failed to open menu:", err);
    } finally {
      isProcessing = false;
    }
  });
});
