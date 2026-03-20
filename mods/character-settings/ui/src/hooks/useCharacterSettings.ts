import { useCallback, useEffect, useState } from "react";
import { Webview, Vrm, entities, type Ocean, audio, host } from "@hmcs/sdk";

export type Tab = "basic" | "persona" | "ocean";

export function useCharacterSettings() {
  const [vrm, setVrm] = useState<Vrm | null>(null);
  const [entity, setEntity] = useState<number | null>(null);
  const [tab, setTab] = useState<Tab>("basic");
  const [scale, setScale] = useState(1);
  const [vrmNames, setVrmNames] = useState<{ metadata: string; names: Record<string, string> }>({
    metadata: "",
    names: {},
  });
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

      const [persona, namesResponse, transform] = await Promise.all([
        linked.persona(),
        host.get(host.createUrl(`vrm/${linked.entity}/names`)),
        entities.transform(linked.entity),
      ]);
      if (cancelled) return;

      const namesData = await namesResponse.json();
      setVrmNames(namesData);
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
  }, [vrm, entity, profile, personality, ocean, scale, saving]);

  const handleNameSave = useCallback(async (lang: string, newName: string) => {
    if (!vrm) return;
    await vrm.setName(newName, lang);
  }, [vrm]);

  const handleNameDelete = useCallback(async (lang: string) => {
    if (!vrm) return;
    await vrm.deleteName(lang);
    setVrmNames(prev => {
      const { [lang]: _, ...rest } = prev.names;
      return { ...prev, names: rest };
    });
  }, [vrm]);

  return {
    loading,
    vrmNames,
    setVrmNames,
    handleNameSave,
    handleNameDelete,
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
