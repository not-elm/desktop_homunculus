import { useState, useEffect, useCallback } from "react";
import { Persona, type PersonaSnapshot } from "@hmcs/sdk";

export function usePersonaManagement() {
  const [personas, setPersonas] = useState<PersonaSnapshot[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    try {
      const list = await Persona.list();
      setPersonas(list);
      setError(null);
    } catch (e) {
      console.error("Failed to load personas:", e);
      setError((e as Error).message);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    refresh();
  }, [refresh]);

  const createPersona = useCallback(
    async (id: string, name: string) => {
      await Persona.create({ id, name });
      await refresh();
      return id;
    },
    [refresh],
  );

  const deletePersona = useCallback(
    async (id: string) => {
      const p = new Persona(id);
      await p.delete();
      await refresh();
    },
    [refresh],
  );

  const spawnPersona = useCallback(
    async (id: string) => {
      const p = new Persona(id);
      await p.spawn();
      await refresh();
    },
    [refresh],
  );

  const despawnPersona = useCallback(
    async (id: string) => {
      const p = new Persona(id);
      await p.despawn();
      await refresh();
    },
    [refresh],
  );

  const setAutoSpawn = useCallback(
    async (id: string, value: boolean) => {
      const p = new Persona(id);
      const snapshot = personas.find((s) => s.id === id);
      if (snapshot) {
        await p.patch({
          metadata: { ...snapshot.metadata, "auto-spawn": value },
        });
        await refresh();
      }
    },
    [personas, refresh],
  );

  return {
    personas,
    loading,
    error,
    refresh,
    createPersona,
    deletePersona,
    spawnPersona,
    despawnPersona,
    setAutoSpawn,
  };
}
