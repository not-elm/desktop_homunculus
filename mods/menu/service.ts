import {
  audio,
  host,
  isWebviewSourceInfoLocal,
  Persona,
  signals,
  Webview,
  WebviewLayer,
  type WebviewSourceInfo,
  webviewSource,
} from '@hmcs/sdk';
import { EventSource } from 'eventsource';

const menuUIAssetId = 'menu:ui';
let isProcessing = false;
const eventSources = new Map<string, ReturnType<Persona['events']>>();

const NON_BLOCKING_SOURCES = new Set(['agent:session-ui']);

async function setupSpawnedPersonas() {
  const snapshots = await Persona.list();
  for (const snapshot of snapshots) {
    if (snapshot.spawned) {
      await setupPersonaEvents(new Persona(snapshot.id));
    }
  }
}

function listenPersonaLifecycle() {
  const personaStream = new EventSource(host.createUrl('personas/stream').toString());

  personaStream.addEventListener('persona-spawned', async (event) => {
    const data = JSON.parse(event.data) as { personaId: string };
    await setupPersonaEvents(new Persona(data.personaId));
  });

  personaStream.addEventListener('persona-despawned', async (event) => {
    const data = JSON.parse(event.data) as { personaId: string };
    const es = eventSources.get(data.personaId);
    if (es) {
      es.close();
      eventSources.delete(data.personaId);
    }
  });
}

async function setupPersonaEvents(p: Persona) {
  const oldEs = eventSources.get(p.id);
  if (oldEs) oldEs.close();

  const es = p.events();
  eventSources.set(p.id, es);

  es.on('pointer-click', async (e) => {
    if ((e as unknown as { button: string }).button !== 'Secondary') return;
    if (isProcessing) return;
    isProcessing = true;
    try {
      const currentMenu = await openedMenu();
      if (currentMenu) {
        await audio.se.play('se:close');
        await currentMenu.close();
        return;
      }
      if (await existsLinkedWebview(p.id)) {
        return;
      }

      await Webview.open({
        source: webviewSource.local('menu:ui'),
        size: [0.8, 1],
        viewportSize: [500, 600],
        transform: { translation: [1.0, 0.8, WebviewLayer.UI] },
        linkedPersona: p.id,
      });
      await audio.se.play('se:open');
    } catch (err) {
      console.error('Failed to open menu:', err);
    } finally {
      isProcessing = false;
    }
  });
}

async function existsLinkedWebview(personaId: string) {
  const webviews = await Webview.list();
  for (const webview of webviews) {
    if (webview.linkedPersona !== personaId) continue;
    if (isNonBlockingSource(webview.source)) continue;
    return true;
  }
  return false;
}

async function openedMenu() {
  const webviews = await Webview.list();
  for (const webview of webviews) {
    if (isWebviewSourceInfoLocal(webview.source) && webview.source.id === menuUIAssetId) {
      return new Webview(webview.entity);
    }
  }
  return null;
}

function isNonBlockingSource(source: WebviewSourceInfo): boolean {
  return isWebviewSourceInfoLocal(source) && NON_BLOCKING_SOURCES.has(source.id);
}

signals.stream<{ entity: number }>('menu:close', async (payload) => {
  try {
    const webview = new Webview(payload.entity);
    await webview.close();
  } catch (err) {
    console.error('Failed to close menu:', err);
  }
});

await setupSpawnedPersonas();
listenPersonaLifecycle();
