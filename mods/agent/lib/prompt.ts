import type { Persona } from "./types.ts";

/** Builds the system prompt from persona information. */
export function buildCharacterPrompt(persona: Persona): string {
  return [
    `あなたは「${persona.name}」です。`,
    persona.personality && `性格: ${persona.personality}`,
    `Desktop Homunculusのキャラクターとして、ユーザーの指示に従って作業を行ってください。`,
    `応答は簡潔にしてください。`,
  ]
    .filter(Boolean)
    .join("\n");
}
