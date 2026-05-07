import { cn } from '@hmcs/ui';

interface ThumbnailCardProps {
  thumbnailUrl: string | null;
  onThumbnailChange: () => void;
  className?: string;
}

export function ThumbnailCard({ thumbnailUrl, onThumbnailChange, className }: ThumbnailCardProps) {
  return (
    <button
      type="button"
      className={cn(
        'relative mb-3 flex aspect-square w-full cursor-pointer items-center justify-center overflow-hidden rounded-lg border border-[oklch(0.72_0.14_192/0.2)] bg-gradient-to-br from-[oklch(0.22_0.03_250)] to-[oklch(0.17_0.02_260)]',
        className,
      )}
      onClick={onThumbnailChange}
    >
      {thumbnailUrl ? (
        <img src={thumbnailUrl} alt="Thumbnail" className="h-full w-full object-cover" />
      ) : (
        <div className="flex h-full w-full items-center justify-center opacity-30" />
      )}
      <div className="absolute bottom-0 left-0 right-0 bg-gradient-to-b from-transparent to-[oklch(0.1_0.01_250/0.9)] p-2 text-center">
        <span className="cursor-pointer text-[11px] tracking-[0.08em] text-[oklch(0.72_0.14_192/0.4)] transition-colors duration-200 hover:text-[oklch(0.72_0.14_192)]">
          Change Image...
        </span>
      </div>
    </button>
  );
}
