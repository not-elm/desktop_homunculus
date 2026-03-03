import { Vrm, Webview, audio, signals, isWebviewSourceInfoLocal, webviewSource } from "@hmcs/sdk";
const menuUIAssetId = "menu:ui";
let isProcessing = false;
const eventSources = new Map();

const existsLinkedWebview = async (vrmEntity: number) => {
  const webviews = await Webview.list();
  for (let webview of webviews) {
    const linked = webview.linkedVrm;
    if (linked === vrmEntity) {
      return true;
    }
  }
  return false;
};

const openedMenu = async () => {
  const webviews = await Webview.list();
  for (let webview of webviews) {
    if (isWebviewSourceInfoLocal(webview.source) && webview.source.id === menuUIAssetId) {
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
  const es = vrm.events();
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
      if (await existsLinkedWebview(vrm.entity)) {
        return;
      }

      await Webview.open({
        source: webviewSource.local("menu:ui"),
        size: [0.8, 1],
        viewportSize: [500, 600],
        offset: [1, -0.3],
        linkedVrm: vrm.entity,
      });
      await audio.se.play("se:open");
    } catch (err) {
      console.error("Failed to open menu:", err);
    } finally {
      isProcessing = false;
    }
  });
});
