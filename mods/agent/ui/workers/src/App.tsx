import { Toolbar } from '@hmcs/ui';
import { audio, Webview } from '@hmcs/sdk';
import { WorkerList } from './components/WorkerList';
import { WorkerLog } from './components/WorkerLog';
import { usePositionPersist } from './hooks/usePositionPersist';
import { useWorkers } from './hooks/useWorkers';

export function App() {
  const { workers, selectedId, setSelectedId, personaId } = useWorkers();
  usePositionPersist(personaId);

  const selectedWorker = selectedId ? workers.get(selectedId) : undefined;
  const runningCount = countRunning(workers);
  const title = personaId ? `Workers \u2014 ${personaId}` : 'Workers';

  async function handleClose() {
    await audio.se.play('se:close');
    await Webview.current()?.close();
  }

  return (
    <div className="flex h-screen flex-col">
      <Toolbar title={title} onClose={handleClose}>
        <RunningBadge count={runningCount} />
      </Toolbar>
      <div className="flex min-h-0 flex-1">
        <div className="w-[45%] shrink-0 overflow-y-auto border-r border-[var(--hud-border-decorative)]">
          <WorkerList
            workers={workers}
            selectedId={selectedId}
            onSelect={setSelectedId}
          />
        </div>
        <div className="flex min-w-0 flex-1 flex-col">
          <WorkerLog worker={selectedWorker} />
        </div>
      </div>
    </div>
  );
}

function RunningBadge({ count }: { count: number }) {
  if (count === 0) return null;

  return (
    <span className="rounded-full bg-[oklch(0.72_0.14_192/0.2)] px-2 py-0.5 text-[10px] font-medium text-[oklch(0.72_0.14_192)] [app-region:no-drag] [-webkit-app-region:no-drag]">
      {count} running
    </span>
  );
}

function countRunning(workers: Map<string, { status: string }>): number {
  let count = 0;
  for (const w of workers.values()) {
    if (w.status === 'running') count++;
  }
  return count;
}
