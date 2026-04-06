import { MessageSquareText, ShieldCheck } from "lucide-react";
import type { SettingsCategory } from "../types";

interface SettingsNavProps {
  activeCategory: SettingsCategory | null;
  onSelect: (category: SettingsCategory) => void;
}

const NAV_ITEMS: { category: SettingsCategory; label: string; icon: React.ElementType }[] = [
  { category: "phrases", label: "Phrases", icon: MessageSquareText },
  { category: "permissions", label: "Permissions", icon: ShieldCheck },
];

export function SettingsNav({ activeCategory, onSelect }: SettingsNavProps) {
  return (
    <nav className="stg-nav" aria-label="Settings">
      <div className="stg-nav-label">Settings</div>
      <ul role="list" className="stg-nav-list">
        {NAV_ITEMS.map(({ category, label, icon: Icon }) => (
          <li key={category}>
            <button
              className={`stg-nav-item${activeCategory === category ? " stg-nav-item--active" : ""}`}
              type="button"
              aria-current={activeCategory === category ? "true" : undefined}
              onClick={() => onSelect(category)}
            >
              <Icon className="stg-nav-icon" />
              <span>{label}</span>
            </button>
          </li>
        ))}
      </ul>
    </nav>
  );
}
