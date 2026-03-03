import type { Ocean } from "@hmcs/sdk";

const LABELS = ["O", "C", "E", "A", "N"];
const KEYS: (keyof Ocean)[] = [
  "openness",
  "conscientiousness",
  "extraversion",
  "agreeableness",
  "neuroticism",
];

const SIZE = 200;
const CENTER = SIZE / 2;
const RADIUS = 80;

function polarToXY(angle: number, r: number): [number, number] {
  const rad = ((angle - 90) * Math.PI) / 180;
  return [CENTER + r * Math.cos(rad), CENTER + r * Math.sin(rad)];
}

function pentagonPoints(r: number): string {
  return Array.from({ length: 5 }, (_, i) => polarToXY((360 / 5) * i, r))
    .map(([x, y]) => `${x},${y}`)
    .join(" ");
}

interface RadarChartProps {
  ocean: Ocean;
}

export function RadarChart({ ocean }: RadarChartProps) {
  const values = KEYS.map((k) => ocean[k] ?? 0.5);

  const dataPoints = values
    .map((v, i) => polarToXY((360 / 5) * i, v * RADIUS))
    .map(([x, y]) => `${x},${y}`)
    .join(" ");

  return (
    <div className="radar-chart-container">
      <svg viewBox={`0 0 ${SIZE} ${SIZE}`} className="radar-chart">
        <defs>
          <filter id="glow">
            <feGaussianBlur stdDeviation="3" result="coloredBlur" />
            <feMerge>
              <feMergeNode in="coloredBlur" />
              <feMergeNode in="SourceGraphic" />
            </feMerge>
          </filter>
        </defs>

        {/* Grid pentagons */}
        {[0.33, 0.66, 1].map((scale) => (
          <polygon
            key={scale}
            points={pentagonPoints(RADIUS * scale)}
            fill="none"
            stroke="oklch(0.72 0.14 192 / 0.15)"
            strokeWidth="0.5"
          />
        ))}

        {/* Axis lines */}
        {Array.from({ length: 5 }, (_, i) => {
          const [x, y] = polarToXY((360 / 5) * i, RADIUS);
          return (
            <line
              key={i}
              x1={CENTER}
              y1={CENTER}
              x2={x}
              y2={y}
              stroke="oklch(0.72 0.14 192 / 0.1)"
              strokeWidth="0.5"
            />
          );
        })}

        {/* Data polygon */}
        <polygon
          points={dataPoints}
          fill="oklch(0.72 0.14 192 / 0.15)"
          stroke="oklch(0.72 0.14 192 / 0.8)"
          strokeWidth="1.5"
          filter="url(#glow)"
        />

        {/* Data points */}
        {values.map((v, i) => {
          const [x, y] = polarToXY((360 / 5) * i, v * RADIUS);
          return (
            <circle
              key={i}
              cx={x}
              cy={y}
              r="3"
              fill="oklch(0.72 0.14 192)"
              filter="url(#glow)"
            />
          );
        })}

        {/* Labels */}
        {LABELS.map((label, i) => {
          const [x, y] = polarToXY((360 / 5) * i, RADIUS + 16);
          return (
            <text
              key={i}
              x={x}
              y={y}
              textAnchor="middle"
              dominantBaseline="central"
              className="radar-label"
            >
              {label}
            </text>
          );
        })}
      </svg>
    </div>
  );
}
