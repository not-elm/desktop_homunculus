import { useState, useEffect, useCallback } from "react";
import { Persona, type PersonaSnapshot } from "@hmcs/sdk";

export function usePersonaManagement() {
  const [personas, setPersonas] = useState<PersonaSnapshot[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [createMode, setCreateMode] = useState(false);

  const refresh = useCallback(async () => {
    try {
      const list = await Persona.list();
      setPersonas(list);
      setError(null);
      return list;
    } catch (e) {
      console.error("Failed to load personas:", e);
      setError((e as Error).message);
      return [];
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    refresh().then((list) => {
      if (list.length > 0 && selectedId == null) {
        setSelectedId(list[0].id);
      }
    });
  }, [refresh]);

  const selectPersona = useCallback((id: string) => {
    setCreateMode(false);
    setSelectedId(id);
  }, []);

  const enterCreateMode = useCallback(() => {
    setCreateMode(true);
  }, []);

  const exitCreateMode = useCallback(() => {
    setCreateMode(false);
  }, []);

  const createPersona = useCallback(
    async (id: string, name: string) => {
      await Persona.create({ id, name });
      await refresh();
      setCreateMode(false);
      setSelectedId(id);
    },
    [refresh],
  );

  const deletePersona = useCallback(
    async (id: string) => {
      const p = new Persona(id);
      await p.delete();
      const list = await refresh();
      if (id === selectedId) {
        setSelectedId(list.length > 0 ? list[0].id : null);
      }
    },
    [refresh, selectedId],
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
          metadata: { ...(snapshot.metadata ?? {}), "auto-spawn": value },
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
    selectedId,
    createMode,
    refresh,
    selectPersona,
    enterCreateMode,
    exitCreateMode,
    createPersona,
    deletePersona,
    spawnPersona,
    despawnPersona,
    setAutoSpawn,
  };
}
