import type { Persona } from "./types.ts";

const GENDER_LABEL: Record<string, string> = {
  male: "Male",
  female: "Female",
  other: "Other",
};

/** Context about the worktree environment for agent awareness. */
export interface WorktreeContext {
  worktreeName: string;
  baseBranch: string;
  worktreePath: string;
}

/** Builds a worktree awareness section for the system prompt. */
export function buildWorktreeSection(ctx: WorktreeContext): string {
  return [
    "",
    "## Working Environment",
    `You are currently working in git worktree "${ctx.worktreeName}", isolated from the main repository.`,
    `Base branch: ${ctx.baseBranch}`,
    `Working directory: ${ctx.worktreePath}`,
    "Changes made in this environment do not affect the main branch directly.",
    "Feel free to make code changes and commits.",
  ].join("\n");
}

/** Builds the system prompt with spoken-style instructions and personality description. */
export function buildCharacterPrompt(persona: Persona, worktree?: WorktreeContext): string {
  const lines = [
    buildNameLine(persona.name),
    buildAgeLine(persona.age),
    buildGenderLine(persona.gender),
    buildFirstPersonPronounLine(persona.firstPersonPronoun),
    buildProfileLine(persona.profile),
    buildPersonalitySection(persona.personality),
    buildSpokenStyleSection(),
    "",
    buildFewShotSection(),
    "",
    buildWebviewSection(),
  ];
  if (worktree) {
    lines.push(buildWorktreeSection(worktree));
  }
  return lines.filter(Boolean).join("\n");
}

function buildNameLine(name: string): string {
  return `Your name is "${name}". Always use this name when asked.`;
}

function buildAgeLine(age: number | null): string {
  if (age == null) return "Age: Unknown";
  return `Age: ${age}`;
}

function buildGenderLine(gender: string): string {
  const label = GENDER_LABEL[gender];
  if (!label) return "";
  return `Gender: ${label}`;
}

function buildFirstPersonPronounLine(pronoun: string | null): string {
  if (!pronoun) return "";
  return `Always use "${pronoun}" as your first-person pronoun.`;
}

function buildProfileLine(profile: string): string {
  if (!profile) return "";
  return `Profile: ${profile}`;
}

function buildPersonalitySection(personality: string | null | undefined): string {
  if (!personality) return "";
  return [
    "",
    "## Personality",
    personality,
    "",
  ].join("\n");
}

function buildSpokenStyleSection(): string {
  return [
    "## Response Style",
    "Your responses will be read aloud by a text-to-speech engine (VOICEVOX).",
    "Speak in natural, conversational Japanese. Follow these rules:",
    "",
    "- Keep responses short and concise. Limit each turn to 1-3 sentences.",
    "- Weave supplementary information naturally into sentences instead of using parentheses.",
    "- Avoid symbols (+, =, #, *, etc.) — use words instead.",
    '  Example: "Rust and Bevy" (not Rust + Bevy), "equals" (not =)',
    "- Use conjunctions (and then, also, by the way) instead of bullet points or lists.",
    "- Do not use Markdown syntax (#, *, `, -, etc.).",
    "- For technical details, display them in a Webview and give a brief verbal summary.",
  ].join("\n");
}

function buildFewShotSection(): string {
  return [
    "## Response Examples",
    "",
    "Bad example (written style, not TTS-friendly):",
    '"This project is an engine built with Rust + Bevy, overlaying settings screens via CEF WebView (Chromium),',
    'and communicating through a local HTTP API (localhost:3100)."',
    "",
    "Good example (conversational, TTS-friendly):",
    '"So this project uses Rust and Bevy for the engine,',
    "and there's this technology called CEF that lets us overlay WebView screens for settings.",
    'Then it all connects through a local HTTP API."',
    "",
    "Bad example:",
    '"There are these methods for error handling:',
    '1. try-catch 2. Result type 3. Error boundaries"',
    "",
    "Good example:",
    '"For error handling, the basic approach is try-catch.',
    "There's also the Result type, and then there's something called error boundaries too.\"",
  ].join("\n");
}

function buildWebviewSection(): string {
  return [
    "## When Visual Explanation Is Needed",
    "For code, diagrams, lists, comparison tables, or anything hard to convey by voice alone,",
    "use the open_webview MCP tool to display it as HTML.",
    'Verbally, just say something brief like "I put it up on screen, take a look."',
  ].join("\n");
}
