import { audio, Webview } from '@hmcs/sdk';
import {
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
import {
  loadingTextClasses,
  managementBtnClasses,
  managementBtnDangerClasses,
  managementBtnSecondaryClasses,
  panelClasses,
} from './styles';

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
        <div className="flex min-h-[200px] h-full items-center justify-center">
          <div className={loadingTextClasses}>Loading...</div>
        </div>
      </div>
    );
  }

  return (
    <div className={panelClasses}>
      <Toolbar title="Persona" onClose={handleClose} />
      <div className="flex min-h-0 flex-1">
        <Sidebar
          personas={mgmt.personas}
          selectedId={mgmt.createMode ? null : mgmt.selectedId}
          onSelect={handleSelectPersona}
          onCreateClick={handleCreateClick}
        />
        <div className="no-scrollbar min-w-0 flex-1 overflow-y-auto">
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
            <div className="flex h-full flex-col items-center justify-center gap-4">
              <div className="text-xs uppercase tracking-[0.1em] text-[oklch(0.55_0.02_250)]">
                No personas yet
              </div>
              <button type="button" className={managementBtnClasses} onClick={mgmt.enterCreateMode}>
                + Create
              </button>
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
          <button
            type="button"
            className={`${managementBtnClasses} ${managementBtnSecondaryClasses}`}
            onClick={onCancel}
          >
            Cancel
          </button>
          <button
            type="button"
            className={`${managementBtnClasses} ${managementBtnDangerClasses}`}
            onClick={onConfirm}
          >
            Discard
          </button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
