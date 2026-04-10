import "../thumbnail.css";

interface ThumbnailCardProps {
  thumbnailUrl: string | null;
  onThumbnailChange: () => void;
}

export function ThumbnailCard({ thumbnailUrl, onThumbnailChange }: ThumbnailCardProps) {
  return (
    <div className="detail-thumb" onClick={onThumbnailChange}>
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
