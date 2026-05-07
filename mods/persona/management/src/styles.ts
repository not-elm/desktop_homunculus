/**
 * Shared Tailwind utility class strings for the persona/management mod UI.
 *
 * Centralized here so the same look (glassmorphism panel, HUD-style buttons,
 * loading text, etc.) can be reused across App.tsx and the components/ files
 * without duplicating long class lists.
 */

export const panelClasses =
  'relative box-border flex h-screen max-h-screen max-w-[100vw] flex-col overflow-hidden rounded-xl bg-[oklch(0.15_0.01_250/0.92)] animate-management-in motion-reduce:animate-none motion-reduce:opacity-100';

export const managementBtnClasses =
  'rounded-md border border-[oklch(0.72_0.14_192/0.3)] bg-[oklch(0.72_0.14_192/0.15)] px-6 py-2 font-[inherit] text-xs uppercase tracking-[0.08em] text-[oklch(0.72_0.14_192)] cursor-pointer transition-[background,border-color,box-shadow] duration-[var(--hud-duration-content)] ease-linear hover:bg-[oklch(0.72_0.14_192/0.25)] hover:border-[oklch(0.72_0.14_192/0.5)] hover:[box-shadow:0_0_12px_oklch(0.72_0.14_192/0.2)] active:scale-[0.97] disabled:opacity-50 disabled:cursor-not-allowed';

export const managementBtnSecondaryClasses =
  'border-[oklch(0.4_0.02_250/0.25)] bg-transparent text-[oklch(0.75_0.04_250/0.75)] hover:text-[oklch(0.88_0.08_250/0.9)] hover:border-[oklch(0.55_0.04_250/0.35)] hover:bg-transparent hover:[box-shadow:none] hover:[text-shadow:0_0_10px_oklch(0.72_0.14_192/0.2)]';

export const managementBtnDangerClasses =
  'border-[oklch(0.62_0.2_25/0.3)] bg-[oklch(0.62_0.2_25/0.1)] text-[oklch(0.62_0.2_25)] hover:bg-[oklch(0.62_0.2_25/0.2)] hover:border-[oklch(0.62_0.2_25/0.5)] hover:[box-shadow:0_0_12px_oklch(0.62_0.2_25/0.2)]';

export const managementBtnSuccessClasses =
  'border-[oklch(0.65_0.18_145/0.4)] bg-[oklch(0.65_0.18_145/0.15)] text-[oklch(0.65_0.18_145)] hover:bg-[oklch(0.65_0.18_145/0.2)] hover:border-[oklch(0.65_0.18_145/0.5)] hover:[box-shadow:0_0_16px_oklch(0.65_0.18_145/0.25)]';

export const loadingTextClasses =
  'text-xs uppercase tracking-[0.12em] text-[oklch(0.72_0.14_192/0.5)] motion-reduce:opacity-50';
