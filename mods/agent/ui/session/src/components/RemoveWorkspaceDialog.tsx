import {
  AlertDialog,
  AlertDialogContent,
  AlertDialogHeader,
  AlertDialogFooter,
  AlertDialogTitle,
  AlertDialogDescription,
  AlertDialogAction,
  AlertDialogCancel,
} from "@hmcs/ui";

interface RemoveWorkspaceDialogProps {
  path: string;
  worktreeCount: number;
  onConfirm: () => void;
  onCancel: () => void;
}

export function RemoveWorkspaceDialog({
  path,
  worktreeCount,
  onConfirm,
  onCancel,
}: RemoveWorkspaceDialogProps) {
  return (
    <AlertDialog open onOpenChange={(open) => { if (!open) onCancel(); }}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Remove workspace from list?</AlertDialogTitle>
          <AlertDialogDescription>
            <span className="font-mono text-xs">{path}</span>
            {worktreeCount > 0 && (
              <>
                <br />
                This will also remove {worktreeCount} associated worktree
                {worktreeCount > 1 ? "s" : ""} from the list. Files on disk are
                not deleted.
              </>
            )}
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel onClick={onCancel}>Cancel</AlertDialogCancel>
          <AlertDialogAction
            className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
            onClick={onConfirm}
          >
            Remove from list
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}
