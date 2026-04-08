import { useState } from "react";
import { usePersonaManagement } from "./hooks/usePersonaManagement";
import PersonaList from "./components/PersonaList";
import PersonaDetail from "./components/PersonaDetail";
import CreatePersonaDialog from "./components/CreatePersonaDialog";

type Route = "list" | { type: "detail"; id: string };

export default function App() {
  const [route, setRoute] = useState<Route>("list");
  const [createOpen, setCreateOpen] = useState(false);
  const mgmt = usePersonaManagement();

  if (mgmt.loading) {
    return (
      <div className="management-panel holo-noise">
        <HudDecorations />
        <div className="management-loading">
          <div className="management-loading-text">Loading...</div>
        </div>
      </div>
    );
  }

  return (
    <div className="management-panel holo-noise">
      <HudDecorations />

      {route === "list" ? (
        <>
          <PersonaList
            personas={mgmt.personas}
            onEdit={(id) => setRoute({ type: "detail", id })}
            onDelete={mgmt.deletePersona}
            onSpawn={mgmt.spawnPersona}
            onDespawn={mgmt.despawnPersona}
            onAutoSpawnChange={mgmt.setAutoSpawn}
            onCreateClick={() => setCreateOpen(true)}
          />
          <CreatePersonaDialog
            open={createOpen}
            onOpenChange={setCreateOpen}
            onCreate={async (id, name) => {
              await mgmt.createPersona(id, name);
              setRoute({ type: "detail", id });
            }}
          />
        </>
      ) : (
        <PersonaDetail
          personaId={route.id}
          onBack={() => {
            mgmt.refresh();
            setRoute("list");
          }}
        />
      )}
    </div>
  );
}

function HudDecorations() {
  return (
    <>
      <div className="management-scanline" />
      <div className="management-highlight" />
      <div className="management-bottom-line" />
      <span className="management-corner management-corner--tl" />
      <span className="management-corner management-corner--tr" />
      <span className="management-corner management-corner--bl" />
      <span className="management-corner management-corner--br" />
    </>
  );
}
