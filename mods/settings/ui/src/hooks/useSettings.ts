import { useCallback, useEffect, useState } from "react";
import { Webview, audio, settings, shadowPanel } from "@hmcs/sdk";

export function useSettings() {
  const [fps, setFps] = useState(60);
  const [alpha, setAlpha] = useState(0.5);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    let cancelled = false;

    (async () => {
      const [currentFps, currentAlpha] = await Promise.all([
        settings.fps(),
        shadowPanel.alpha(),
      ]);
      if (cancelled) return;
      setFps(currentFps);
      setAlpha(currentAlpha);
      setLoading(false);
    })();

    return () => {
      cancelled = true;
    };
  }, []);

  const handleSave = useCallback(async () => {
    if (saving) return;
    setSaving(true);
    try {
      await Promise.all([
        settings.setFps(fps),
        shadowPanel.setAlpha(alpha),
      ]);
      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
    } catch (err) {
      console.error("Save failed:", err);
    } finally {
      setSaving(false);
    }
  }, [fps, alpha, saving]);

  const handleClose = useCallback(() => {
    audio.se.play("se:close");
    Webview.current()?.close();
  }, []);

  return {
    loading,
    fps,
    setFps,
    alpha,
    setAlpha,
    saving,
    saved,
    handleSave,
    handleClose,
  };
}
