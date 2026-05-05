interface ThumbnailCardProps {
  thumbnailUrl: string | null;
  onThumbnailChange: () => void;
  className?: string;
}

const baseClasses =
  'group relative aspect-square w-full overflow-hidden rounded-md border border-primary/20 [background:linear-gradient(135deg,oklch(0.22_0.03_250),oklch(0.17_0.02_260))] transition-all duration-200 hover:border-primary/40 hover:shadow-holo-sm focus-visible:border-primary/50 focus-visible:shadow-holo focus-visible:outline-hidden';

const overlayClasses =
  'absolute inset-0 flex items-center justify-center bg-[oklch(0.1_0.01_250/0.9)] text-xs uppercase tracking-[0.1em] text-foreground opacity-0 transition-opacity duration-200 group-hover:opacity-100';

export function ThumbnailCard({ thumbnailUrl, onThumbnailChange, className }: ThumbnailCardProps) {
  return (
    <button
      type="button"
      className={[baseClasses, className].filter(Boolean).join(' ')}
      onClick={onThumbnailChange}
    >
      {thumbnailUrl ? (
        <img src={thumbnailUrl} alt="Thumbnail" className="h-full w-full object-cover" />
      ) : (
        <div className="h-full w-full" />
      )}
      <div className={overlayClasses}>
        <span>Change Image...</span>
      </div>
    </button>
  );
}
