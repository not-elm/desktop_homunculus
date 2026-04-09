import { useCallback, useEffect, useState } from "react";
import { Webview, type Persona, type Gender, audio } from "@hmcs/sdk";

export type Tab = "persona" | "appearance";

export function useCharacterSettings() {
  const [personaInstance, setPersonaInstance] = useState<Persona | null>(null);
  const [tab, setTab] = useState<Tab>("persona");
  const [name, setName] = useState("");
  const [scale, setScale] = useState(1);
  const [profile, setProfile] = useState("");
  const [personality, setPersonality] = useState("");
  const [age, setAge] = useState<number | null>(null);
  const [gender, setGender] = useState<Gender>("unknown");
  const [firstPersonPronoun, setFirstPersonPronoun] = useState("");
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const webview = Webview.current();
    if (!webview) return;
    let cancelled = false;

    (async () => {
      const linked = await webview.linkedPersona();
      if (cancelled || !linked) return;
      setPersonaInstance(linked);

      const [snapshot, transform] = await Promise.all([
        linked.snapshot(),
        linked.transform(),
      ]);
      if (cancelled) return;

      setName(snapshot.name ?? "");
      setScale(transform.scale[0]);
      setProfile(snapshot.profile);
      setPersonality(snapshot.personality ?? "");
      setAge(snapshot.age ?? null);
      setGender(snapshot.gender ?? "unknown");
      setFirstPersonPronoun(snapshot.firstPersonPronoun ?? "");
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
    if (!personaInstance || saving) return;
    setSaving(true);
    try {
      await personaInstance.patch({
        name: name || undefined,
        age: age,
        gender,
        firstPersonPronoun: firstPersonPronoun || undefined,
        profile,
        personality: personality || undefined,
      });
      const currentTransform = await personaInstance.transform();
      await personaInstance.setTransform({
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
  }, [personaInstance, name, age, gender, firstPersonPronoun, profile, personality, scale, saving]);

  return {
    loading,
    name,
    setName,
    tab,
    setTab,
    scale,
    setScale,
    profile,
    setProfile,
    personality,
    setPersonality,
    age,
    setAge,
    gender,
    setGender,
    firstPersonPronoun,
    setFirstPersonPronoun,
    saving,
    saved,
    handleSave,
    handleClose,
  };
}
