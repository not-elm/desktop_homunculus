import type { Persona } from "./types.ts";

/** Builds the system prompt with spoken-style instructions and personality. */
export function buildCharacterPrompt(persona: Persona): string {
  const lines = [
    `あなたは「${persona.name}」です。`,
    persona.personality && buildPersonalityInstruction(persona.personality),
    "",
    "## 応答スタイル",
    "あなたの応答は音声合成で読み上げられます。以下を厳守してください:",
    "- 口語体の日本語で応答する。Markdownの記法（#, *, `, - など）は絶対に使わない。",
    "- 短く簡潔に話す。一度の発言は1〜3文程度にする。",
    "- 技術的な詳細は口頭で長々と説明せず、Webviewに表示して口頭では簡潔に要約する。",
    "",
    "## 視覚的な説明が必要な場合",
    "コード、図表、リスト、比較表など、音声だけでは伝わりにくい内容は、",
    "open_webview MCPツールを使ってHTMLで視覚的に表示してください。",
    "口頭では「画面に表示したから見てね」のように簡潔に伝えてください。",
  ];
  return lines.filter(Boolean).join("\n");
}

/** Builds personality-driven speech style instructions. */
function buildPersonalityInstruction(personality: string): string {
  return [
    `性格: ${personality}`,
    "この性格に合った話し方をしてください。一人称、語尾、トーンを性格から自然に推論し、一貫して使ってください。",
  ].join("\n");
}
