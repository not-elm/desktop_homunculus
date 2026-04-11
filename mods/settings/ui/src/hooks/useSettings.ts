import { audio, settings, shadowPanel, Webview } from '@hmcs/sdk';
import { useCallback, useEffect, useRef, useState } from 'react';

export function useSettings() {
  const [fps, setFps] = useState(60);
  const [alpha, setAlpha] = useState(0.5);
  const [loading, setLoading] = useState(true);
  const initialised = useRef(false);

  useEffect(() => {
    let cancelled = false;

    (async () => {
      const [currentFps, currentAlpha] = await Promise.all([settings.fps(), shadowPanel.alpha()]);
      if (cancelled) return;
      setFps(currentFps);
      setAlpha(currentAlpha);
      setLoading(false);
      initialised.current = true;
    })();

    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    if (!initialised.current) return;
    const id = setTimeout(() => {
      settings.setFps(fps).catch(console.error);
    }, 500);
    return () => clearTimeout(id);
  }, [fps]);

  useEffect(() => {
    if (!initialised.current) return;
    const id = setTimeout(() => {
      shadowPanel.setAlpha(alpha).catch(console.error);
    }, 500);
    return () => clearTimeout(id);
  }, [alpha]);

  const handleClose = useCallback(() => {
    audio.se.play('se:close');
    Webview.current()?.close();
  }, []);

  return { loading, fps, setFps, alpha, setAlpha, handleClose };
}
