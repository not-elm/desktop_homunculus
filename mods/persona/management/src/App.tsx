import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@hmcs/ui';
import { useCallback, useRef, useState } from 'react';
import CreateForm from './components/CreateForm';
import DetailView from './components/DetailView';
import Sidebar from './components/Sidebar';
import Toolbar from './components/Toolbar';
import { usePersonaManagement } from './hooks/usePersonaManagement';

export default function App() {
  const mgmt = usePersonaManagement();
  const dirtyRef = useRef(false);
  const [pendingId, setPendingId] = useState<string | null>(null);
  const [discardOpen, setDiscardOpen] = useState(false);

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
      <div className="management-panel">
        <Toolbar />
        <div className="main-loading">
          <div className="main-loading-text">Loading...</div>
        </div>
      </div>
    );
  }

  return (
    <div className="management-panel">
      <Toolbar />
      <div className="management-body">
        <Sidebar
          personas={mgmt.personas}
          selectedId={mgmt.createMode ? null : mgmt.selectedId}
          onSelect={handleSelectPersona}
          onCreateClick={handleCreateClick}
        />
        <div className="main-area">
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
            <div className="main-empty">
              <div className="main-empty-text">No personas yet</div>
              <button type="button" className="management-btn" onClick={mgmt.enterCreateMode}>
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
            className="management-btn management-btn--secondary"
            onClick={onCancel}
          >
            Cancel
          </button>
          <button
            type="button"
            className="management-btn management-btn--danger"
            onClick={onConfirm}
          >
            Discard
          </button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
