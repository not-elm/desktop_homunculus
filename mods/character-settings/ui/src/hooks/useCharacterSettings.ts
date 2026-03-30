import { useCallback, useEffect, useState } from "react";
import { Webview, Vrm, entities, type Ocean, audio } from "@hmcs/sdk";

export type Tab = "persona" | "ocean" | "appearance";

export function useCharacterSettings() {
  const [vrm, setVrm] = useState<Vrm | null>(null);
  const [entity, setEntity] = useState<number | null>(null);
  const [tab, setTab] = useState<Tab>("persona");
  const [name, setName] = useState("");
  const [displayName, setDisplayName] = useState("");
  const [scale, setScale] = useState(1);
  const [profile, setProfile] = useState("");
  const [personality, setPersonality] = useState("");
  const [ocean, setOcean] = useState<Ocean>({});
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const webview = Webview.current();
    if (!webview) return;
    let cancelled = false;

    (async () => {
      const linked = await webview.linkedVrm();
      if (cancelled || !linked) return;
      setVrm(linked);
      setEntity(linked.entity);

      const [persona, vrmName, transform] = await Promise.all([
        linked.persona(),
        linked.name(),
        entities.transform(linked.entity),
      ]);
      if (cancelled) return;

      setName(vrmName);
      setDisplayName(persona.displayName ?? "");
      setScale(transform.scale[0]);
      setProfile(persona.profile);
      setPersonality(persona.personality ?? "");
      setOcean(persona.ocean);
      setLoading(false);
    })();

    return () => {
      cancelled = true;
    };
  }, []);

  const handleClose = useCallback(() => {
    audio.se.play("se:close");
    Webview.current()?.close();
  }, []);

  const handleSave = useCallback(async () => {
    if (!vrm || entity == null || saving) return;
    setSaving(true);
    try {
      await vrm.setPersona({
        displayName: displayName || null,
        profile,
        personality: personality || null,
        ocean,
        metadata: {},
      });
      const currentTransform = await entities.transform(entity);
      await entities.setTransform(entity, {
        scale: [scale, scale, scale],
        translation: currentTransform.translation,
        rotation: currentTransform.rotation,
      });
      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
    } catch (err) {
      console.error("Save failed:", err);
    } finally {
      setSaving(false);
    }
  }, [vrm, entity, displayName, profile, personality, ocean, scale, saving]);

  return {
    loading,
    name,
    displayName,
    setDisplayName,
    tab,
    setTab,
    scale,
    setScale,
    profile,
    setProfile,
    personality,
    setPersonality,
    ocean,
    setOcean,
    saving,
    saved,
    handleSave,
    handleClose,
  };
}
