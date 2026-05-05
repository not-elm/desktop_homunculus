import { audio, Webview } from '@hmcs/sdk';
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  Toolbar,
} from '@hmcs/ui';
import { useCallback, useRef, useState } from 'react';
import CreateForm from './components/CreateForm';
import DetailView from './components/DetailView';
import Sidebar from './components/Sidebar';
import { usePersonaManagement } from './hooks/usePersonaManagement';

const panelClasses =
  'relative box-border flex h-screen max-h-screen max-w-screen flex-col overflow-hidden rounded-xl bg-panel/92 animate-settings-in motion-reduce:animate-none motion-reduce:opacity-100';

export default function App() {
  const mgmt = usePersonaManagement();
  const dirtyRef = useRef(false);
  const [pendingId, setPendingId] = useState<string | null>(null);
  const [discardOpen, setDiscardOpen] = useState(false);

  const handleClose = useCallback(() => {
    audio.se.play('se:close');
    Webview.current()?.close();
  }, []);

  const handleSelectPersona = useCallback(
    (id: string) => {
      if (dirtyRef.current && id !== mgmt.selectedId) {
        setPendingId(id);
        setDiscardOpen(true);
      } else {
        mgmt.selectPersona(id);
      }
    },
    [mgmt.selectedId, mgmt.selectPersona],
  );

  const handleCancelDiscard = useCallback(() => {
    setDiscardOpen(false);
    setPendingId(null);
  }, []);

  const handleCreateClick = useCallback(() => {
    if (dirtyRef.current) {
      setPendingId(null);
      setDiscardOpen(true);
    } else {
      mgmt.enterCreateMode();
    }
  }, [mgmt.enterCreateMode]);

  const handleConfirmDiscardForCreate = useCallback(() => {
    setDiscardOpen(false);
    if (pendingId) {
      mgmt.selectPersona(pendingId);
      setPendingId(null);
    } else {
      mgmt.enterCreateMode();
    }
  }, [pendingId, mgmt.selectPersona, mgmt.enterCreateMode]);

  const handleDirtyChange = useCallback((dirty: boolean) => {
    dirtyRef.current = dirty;
  }, []);

  const handleDelete = useCallback(async () => {
    dirtyRef.current = false;
    if (mgmt.selectedId) {
      await mgmt.deletePersona(mgmt.selectedId);
    }
  }, [mgmt.selectedId, mgmt.deletePersona]);

  if (mgmt.loading) {
    return (
      <div className={panelClasses}>
        <Toolbar title="Persona" onClose={handleClose} />
        <div className="flex flex-1 items-center justify-center">
          <div className="text-xs uppercase tracking-[0.12em] text-primary/50">Loading...</div>
        </div>
      </div>
    );
  }

  return (
    <div className={panelClasses}>
      <Toolbar title="Persona" onClose={handleClose} />
      <div className="flex min-h-0 flex-1 flex-row">
        <Sidebar
          personas={mgmt.personas}
          selectedId={mgmt.createMode ? null : mgmt.selectedId}
          onSelect={handleSelectPersona}
          onCreateClick={handleCreateClick}
        />
        <div className="flex flex-1 min-w-0 flex-col">
          {mgmt.createMode ? (
            <CreateForm onCreate={mgmt.createPersona} onCancel={mgmt.exitCreateMode} />
          ) : mgmt.selectedId ? (
            <DetailView
              key={mgmt.selectedId}
              personaId={mgmt.selectedId}
              onDirtyChange={handleDirtyChange}
              onSaved={mgmt.refresh}
              onDelete={handleDelete}
            />
          ) : (
            <div className="flex flex-1 flex-col items-center justify-center gap-3">
              <div className="text-sm text-muted-foreground">No personas yet</div>
              <Button variant="hud" size="hud" onClick={mgmt.enterCreateMode}>
                + Create
              </Button>
            </div>
          )}
        </div>
      </div>

      <DiscardDialog
        open={discardOpen}
        onConfirm={handleConfirmDiscardForCreate}
        onCancel={handleCancelDiscard}
      />
    </div>
  );
}

function DiscardDialog({
  open,
  onConfirm,
  onCancel,
}: {
  open: boolean;
  onConfirm: () => void;
  onCancel: () => void;
}) {
  return (
    <Dialog open={open} onOpenChange={(v) => !v && onCancel()}>
      <DialogContent showCloseButton={false}>
        <DialogHeader>
          <DialogTitle>Unsaved Changes</DialogTitle>
          <DialogDescription>You have unsaved changes. Discard?</DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <Button variant="hud-ghost" size="hud" onClick={onCancel}>
            Cancel
          </Button>
          <Button variant="hud-destructive" size="hud" onClick={onConfirm}>
            Discard
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
