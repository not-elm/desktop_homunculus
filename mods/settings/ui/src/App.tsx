import { Toolbar } from '@hmcs/ui';
import { GeneralTab } from './components/GeneralTab';
import { useSettings } from './hooks/useSettings';

export function App() {
  const { loading, fps, setFps, alpha, setAlpha, handleClose } = useSettings();

  if (loading) {
    return (
      <div className="settings-panel settings-loading">
        <div className="settings-loading-text">Loading...</div>
      </div>
    );
  }

  return (
    <div className="settings-panel holo-refract-border holo-noise">
      <div className="settings-highlight" />
      <div className="settings-bottom-line" />
      <div className="settings-scanline" />
      <span className="settings-corner settings-corner--tl" />
      <span className="settings-corner settings-corner--tr" />
      <span className="settings-corner settings-corner--bl" />
      <span className="settings-corner settings-corner--br" />

      <Toolbar title="Settings" onClose={handleClose} />

      <div className="settings-content">
        <GeneralTab fps={fps} setFps={setFps} alpha={alpha} setAlpha={setAlpha} />
      </div>
    </div>
  );
}
