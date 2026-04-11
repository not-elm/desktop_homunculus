import { useCallback, useEffect, useRef, useState } from "react";
import { Persona } from "@hmcs/sdk";

export interface UseScaleReturn {
  scale: number;
  setScale: (scale: number) => void;
  saveScale: () => Promise<void>;
  loading: boolean;
}

/**
 * Loads the persona's current scale from its transform and provides a save action.
 * Separate from usePersonaDetail because scale lives on the transform, not the persona snapshot.
 */
export function useScale(personaId: string): UseScaleReturn {
  const [scale, setScale] = useState(1);
  const [loading, setLoading] = useState(true);
  const loadedRef = useRef(false);

  useEffect(() => {
    if (!personaId) return;
    let cancelled = false;
    (async () => {
      try {
        const transform = await new Persona(personaId).transform();
        if (cancelled) return;
        setScale(transform.scale[0]);
        loadedRef.current = true;
      } catch (e) {
        console.error("Failed to load transform:", e);
      } finally {
        if (!cancelled) setLoading(false);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [personaId]);

  const saveScale = useCallback(async () => {
    if (!loadedRef.current) return;
    try {
      const persona = new Persona(personaId);
      const current = await persona.transform();
      await persona.setTransform({
        scale: [scale, scale, scale],
        translation: current.translation,
        rotation: current.rotation,
      });
    } catch (e) {
      console.error("Failed to save scale:", e);
    }
  }, [personaId, scale]);

  return { scale, setScale, saveScale, loading };
}
