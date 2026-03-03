import type { Ocean } from "@hmcs/sdk";
import { RadarChart } from "./RadarChart";

const TRAITS: { key: keyof Ocean; label: string; low: string; high: string }[] = [
  { key: "openness", label: "Openness", low: "Conservative", high: "Curious" },
  { key: "conscientiousness", label: "Conscientiousness", low: "Spontaneous", high: "Organized" },
  { key: "extraversion", label: "Extraversion", low: "Introverted", high: "Extroverted" },
  { key: "agreeableness", label: "Agreeableness", low: "Independent", high: "Cooperative" },
  { key: "neuroticism", label: "Neuroticism", low: "Stable", high: "Sensitive" },
];

interface OceanTabProps {
  ocean: Ocean;
  onChange: (ocean: Ocean) => void;
}

export function OceanTab({ ocean, onChange }: OceanTabProps) {
  const handleChange = (key: keyof Ocean, value: number) => {
    onChange({ ...ocean, [key]: value });
  };

  return (
    <div className="settings-section settings-ocean">
      <RadarChart ocean={ocean} />

      <div className="settings-ocean-sliders">
        {TRAITS.map((trait) => (
          <div key={trait.key} className="settings-ocean-trait">
            <div className="settings-ocean-header">
              <span className="settings-ocean-label">{trait.label}</span>
              <span className="settings-ocean-value">
                {(ocean[trait.key] ?? 0.5).toFixed(2)}
              </span>
            </div>
            <div className="settings-slider-row">
              <span className="settings-ocean-pole">{trait.low}</span>
              <input
                type="range"
                className="settings-slider"
                min={0}
                max={1}
                step={0.01}
                value={ocean[trait.key] ?? 0.5}
                onChange={(e) => handleChange(trait.key, parseFloat(e.target.value))}
              />
              <span className="settings-ocean-pole">{trait.high}</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
