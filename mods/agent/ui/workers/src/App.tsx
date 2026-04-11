import { Toolbar } from '@hmcs/ui';
import { audio, Webview } from '@hmcs/sdk';

export function App() {
  async function handleClose() {
    await audio.se.play('se:close');
    await Webview.current()?.close();
  }

  return (
    <div className="flex h-screen flex-col">
      <Toolbar title="Workers" onClose={handleClose} />
      <div className="flex flex-1 items-center justify-center">
        <span className="text-[var(--hud-text-muted)] text-xs">Workers WebView</span>
      </div>
    </div>
  );
}
