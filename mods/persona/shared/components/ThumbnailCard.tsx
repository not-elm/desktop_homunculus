import "../thumbnail.css";

interface ThumbnailCardProps {
  thumbnailUrl: string | null;
  onThumbnailChange: () => void;
  className?: string;
}

export function ThumbnailCard({ thumbnailUrl, onThumbnailChange, className }: ThumbnailCardProps) {
  return (
    <div className={`detail-thumb${className ? ` ${className}` : ""}`} onClick={onThumbnailChange}>
      {thumbnailUrl ? (
        <img src={thumbnailUrl} alt="Thumbnail" />
      ) : (
        <div className="detail-thumb-placeholder" />
      )}
      <div className="change-overlay">
        <span>Change Image...</span>
      </div>
    </div>
  );
}
