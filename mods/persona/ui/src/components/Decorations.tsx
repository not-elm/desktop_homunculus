import { cn } from '@hmcs/ui';

type CornerPosition = 'tl' | 'tr' | 'bl' | 'br';

const cornerPositionClass: Record<CornerPosition, string> = {
  tl: 'hud-corner--tl',
  tr: 'hud-corner--tr',
  bl: 'hud-corner--bl',
  br: 'hud-corner--br',
};

function HudCorner({ position }: { position: CornerPosition }) {
  return (
    <span
      aria-hidden="true"
      className={cn('hud-corner pointer-events-none', cornerPositionClass[position])}
    />
  );
}

function HudHighlight() {
  return <div aria-hidden="true" className="hud-highlight pointer-events-none" />;
}

function HudBottomLine() {
  return <div aria-hidden="true" className="hud-bottom-line pointer-events-none" />;
}

function HudScanline() {
  return <div aria-hidden="true" className="hud-scanline pointer-events-none" />;
}

export function Decorations() {
  return (
    <>
      <HudHighlight />
      <HudBottomLine />
      <HudScanline />
      <HudCorner position="tl" />
      <HudCorner position="tr" />
      <HudCorner position="bl" />
      <HudCorner position="br" />
    </>
  );
}
