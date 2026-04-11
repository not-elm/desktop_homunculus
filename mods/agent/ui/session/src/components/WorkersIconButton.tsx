interface WorkersIconButtonProps {
  runningCount: number;
  hasPendingPermission: boolean;
  disabled: boolean;
  onClick: () => void;
}

export function WorkersIconButton({
  runningCount,
  hasPendingPermission,
  disabled,
  onClick,
}: WorkersIconButtonProps) {
  const showBadge = runningCount > 0 || hasPendingPermission;

  return (
    <button
      className={`hud-icon-btn hud-workers-btn${disabled ? ' hud-workers-btn--disabled' : ''}`}
      type="button"
      onClick={onClick}
      disabled={disabled}
      title={badgeTitle(runningCount, hasPendingPermission)}
    >
      <WrenchIcon />
      {showBadge && (
        <span
          className={`hud-workers-badge${hasPendingPermission ? ' hud-workers-badge--alert' : ''}`}
        >
          {hasPendingPermission ? '!' : runningCount}
        </span>
      )}
    </button>
  );
}

function badgeTitle(count: number, pending: boolean): string {
  if (pending) return 'Workers: permission needed';
  if (count > 0) return `Workers: ${count} running`;
  return 'Workers';
}

function WrenchIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 12 12" fill="none" aria-hidden="true">
      <path
        d="M7.5 1.5a3 3 0 0 0-2.8 4.1L1.5 8.8a1 1 0 0 0 0 1.4l.3.3a1 1 0 0 0 1.4 0l3.2-3.2A3 3 0 1 0 7.5 1.5Z"
        stroke="currentColor"
        strokeWidth="1.1"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  );
}
