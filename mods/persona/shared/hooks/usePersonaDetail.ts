import { Persona, type PersonaSnapshot } from '@hmcs/sdk';
import {
  type BehaviorAnimations,
  DEFAULT_ANIMATIONS,
  resolveBehaviorConfig,
} from '@persona/shared/behavior-config';
import type { PersonaFormValues } from '@persona/shared/components/PersonaFields';
import { useCallback, useEffect, useMemo, useRef, useState } from 'react';

export interface UsePersonaDetailReturn {
  snapshot: PersonaSnapshot | null;
  formValues: PersonaFormValues | null;
  vrmAssetId: string | null;
  thumbnail: string | null;
  setThumbnail: (id: string | null) => void;
  saving: boolean;
  saved: boolean;
  isDirty: boolean;
  setFormValues: (values: PersonaFormValues) => void;
  setVrmAssetId: (id: string | null) => void;
  save: () => Promise<void>;
  toggleSpawn: () => Promise<void>;
  toggleAutoSpawn: () => Promise<void>;
  behaviorProcess: string | null;
  behaviorAnimations: BehaviorAnimations;
  setBehaviorProcess: (process: string | null) => void;
  setBehaviorAnimations: (animations: BehaviorAnimations) => void;
}

function snapshotToFormValues(snapshot: PersonaSnapshot): PersonaFormValues {
  return {
    name: snapshot.name ?? '',
    age: snapshot.age != null ? { type: 'specify' as const, age: snapshot.age } : { type: 'unknown' as const },
    gender: snapshot.gender,
    firstPersonPronoun: snapshot.firstPersonPronoun ?? '',
    profile: snapshot.profile,
    personality: snapshot.personality ?? '',
  };
}

/**
 * Manages all state and actions for the persona detail view.
 */
export function usePersonaDetail(
  personaId: string,
  callbacks: {
    onDirtyChange: (dirty: boolean) => void;
    onSaved: () => void;
  },
): UsePersonaDetailReturn {
  const [snapshot, setSnapshot] = useState<PersonaSnapshot | null>(null);
  const [formValues, setFormValues] = useState<PersonaFormValues | null>(null);
  const [vrmAssetId, setVrmAssetId] = useState<string | null>(null);
  const [thumbnail, setThumbnail] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);
  const initialValues = useRef<PersonaFormValues | null>(null);
  const initialVrm = useRef<string | null>(null);
  const initialThumbnail = useRef<string | null>(null);
  const [behaviorProcess, setBehaviorProcess] = useState<string | null>(null);
  const [behaviorAnimations, setBehaviorAnimations] =
    useState<BehaviorAnimations>(DEFAULT_ANIMATIONS);
  const initialBehaviorProcess = useRef<string | null>(null);
  const initialBehaviorAnimations = useRef<BehaviorAnimations>(DEFAULT_ANIMATIONS);

  const persona = useMemo(() => new Persona(personaId), [personaId]);

  const loadSnapshot = useCallback(async () => {
    try {
      const snap = await new Persona(personaId).snapshot();
      setSnapshot(snap);
      const values = snapshotToFormValues(snap);
      setFormValues(values);
      setVrmAssetId(snap.vrmAssetId ?? null);
      setThumbnail(snap.thumbnail ?? null);
      initialValues.current = values;
      initialVrm.current = snap.vrmAssetId ?? null;
      initialThumbnail.current = snap.thumbnail ?? null;
      const behavior = resolveBehaviorConfig(snap);
      setBehaviorProcess(behavior.process);
      setBehaviorAnimations(behavior.animations);
      initialBehaviorProcess.current = behavior.process;
      initialBehaviorAnimations.current = behavior.animations;
    } catch (e) {
      console.error('Failed to load persona:', e);
    }
  }, [personaId]);

  useEffect(() => {
    loadSnapshot();
  }, [loadSnapshot]);

  const isDirty = useCallback(() => {
    if (!formValues || !initialValues.current) return false;
    const iv = initialValues.current;
    return (
      formValues.name !== iv.name ||
      formValues.age.type !== iv.age.type ||
      (formValues.age.type === 'specify' && iv.age.type === 'specify' && formValues.age.age !== iv.age.age) ||
      formValues.gender !== iv.gender ||
      formValues.firstPersonPronoun !== iv.firstPersonPronoun ||
      formValues.profile !== iv.profile ||
      formValues.personality !== iv.personality ||
      vrmAssetId !== initialVrm.current ||
      thumbnail !== initialThumbnail.current ||
      behaviorProcess !== initialBehaviorProcess.current ||
      behaviorAnimations.idle !== initialBehaviorAnimations.current.idle ||
      behaviorAnimations.drag !== initialBehaviorAnimations.current.drag ||
      behaviorAnimations.sitting !== initialBehaviorAnimations.current.sitting
    );
  }, [formValues, vrmAssetId, thumbnail, behaviorProcess, behaviorAnimations]);

  useEffect(() => {
    callbacks.onDirtyChange(isDirty());
  }, [isDirty, callbacks]);

  const saveDraft = useCallback(
    async (options?: { reload?: boolean }): Promise<boolean> => {
      if (!formValues) return false;
      try {
        const vrmChanged = vrmAssetId !== initialVrm.current;
        const thumbnailChanged = thumbnail !== initialThumbnail.current;
        await persona.patch({
          name: formValues.name,
          age: formValues.age.type === 'specify' ? formValues.age.age : null,
          gender: formValues.gender,
          firstPersonPronoun: formValues.firstPersonPronoun || undefined,
          profile: formValues.profile,
          personality: formValues.personality || undefined,
          vrmAssetId: vrmChanged ? (vrmAssetId ?? undefined) : undefined,
          thumbnail: thumbnailChanged ? (thumbnail ?? undefined) : undefined,
          metadata: {
            ...(snapshot?.metadata ?? {}),
            behavior: {
              process: behaviorProcess,
              animations: behaviorAnimations,
            },
          },
        });

        if (vrmChanged && snapshot?.spawned) {
          if (vrmAssetId) {
            await persona.attachVrm(vrmAssetId);
          } else if (initialVrm.current) {
            await persona.detachVrm();
          }
        }

        if (options?.reload !== false) {
          await loadSnapshot();
        }
        return true;
      } catch (e) {
        console.error('Failed to save persona:', e);
        return false;
      }
    },
    [
      formValues,
      vrmAssetId,
      thumbnail,
      snapshot,
      persona,
      loadSnapshot,
      behaviorProcess,
      behaviorAnimations,
    ],
  );

  const save = useCallback(async () => {
    if (saving) return;
    setSaving(true);
    try {
      await saveDraft();
      callbacks.onSaved();
      setSaved(true);
      setTimeout(() => setSaved(false), 1500);
    } finally {
      setSaving(false);
    }
  }, [saving, callbacks, saveDraft]);

  const toggleSpawn = useCallback(async () => {
    if (!snapshot) return;
    try {
      if (snapshot.spawned) {
        await persona.despawn();
      } else {
        const ok = await saveDraft({ reload: false });
        if (!ok) return;
        await persona.spawn();
      }
      await loadSnapshot();
      callbacks.onSaved();
    } catch (e) {
      console.error('Failed to toggle spawn:', e);
    }
  }, [snapshot, persona, callbacks, loadSnapshot, saveDraft]);

  const toggleAutoSpawn = useCallback(async () => {
    if (!snapshot) return;
    const current = snapshot.metadata?.['auto-spawn'] === true;
    try {
      await persona.patch({
        metadata: { ...(snapshot.metadata ?? {}), 'auto-spawn': !current },
      });
      await loadSnapshot();
      callbacks.onSaved();
    } catch (e) {
      console.error('Failed to toggle auto-spawn:', e);
    }
  }, [snapshot, persona, loadSnapshot, callbacks]);

  return {
    snapshot,
    formValues,
    vrmAssetId,
    thumbnail,
    setThumbnail,
    saving,
    saved,
    isDirty: isDirty(),
    setFormValues,
    setVrmAssetId,
    save,
    toggleSpawn,
    toggleAutoSpawn,
    behaviorProcess,
    behaviorAnimations,
    setBehaviorProcess,
    setBehaviorAnimations,
  };
}
