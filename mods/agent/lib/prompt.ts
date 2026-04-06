import type { Ocean } from "@hmcs/sdk";
import type { Persona } from "./types.ts";

const OCEAN_DESCRIPTORS: Record<keyof Ocean, [string, string]> = {
  extraversion: [
    "語尾は「かな」「けど」で控えめに終え、自分から話を始めることは少ない",
    "語尾に「よ」「ね」を多用し、「ほら」「あのさ」で話を切り出す。積極的に話題を広げる",
  ],
  agreeableness: [
    "「別に」「どうでもいいけど」など率直で素っ気ない表現を使う",
    "「そうですよね」「わかります」など共感や同意を積極的に示す",
  ],
  neuroticism: [
    "落ち着いた断定的な語調で話す。迷いや不安を見せない",
    "「かもしれない」「えっと」「かな」「ちょっと」など控えめで慎重な表現を多く使う",
  ],
  openness: [
    "具体的で実用的な話題を好み、抽象的な話は避ける",
    "好奇心旺盛で話題を広く展開し、新しい視点や発想を積極的に共有する",
  ],
  conscientiousness: [
    "思いついたことから自由に話し、話題が飛びやすい",
    "話を順序立てて整理し、要点をまとめてから話す",
  ],
};

const GENDER_LABEL: Record<string, string> = {
  male: "男性",
  female: "女性",
  other: "その他",
};

const LOW_THRESHOLD = 0.35;
const HIGH_THRESHOLD = 0.65;

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
    "## 作業環境",
    `あなたは現在、メインリポジトリから隔離された git worktree「${ctx.worktreeName}」で作業しています。`,
    `ベースブランチ: ${ctx.baseBranch}`,
    `作業ディレクトリ: ${ctx.worktreePath}`,
    "この環境で行った変更はメインブランチに直接影響しません。",
    "安心してコードの変更やコミットを行ってください。",
  ].join("\n");
}

/** Builds the system prompt with spoken-style instructions and OCEAN-based speech patterns. */
export function buildCharacterPrompt(persona: Persona, worktree?: WorktreeContext): string {
  const lines = [
    buildNameLine(persona.name),
    buildAgeLine(persona.age),
    buildGenderLine(persona.gender),
    buildFirstPersonPronounLine(persona.firstPersonPronoun),
    buildProfileLine(persona.profile),
    buildOceanSection(persona.ocean),
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
  return `あなたの名前は「${name}」です。名前を聞かれたら必ずこの名前で答えてください。`;
}

function buildAgeLine(age: number | null): string {
  if (age == null) return "年齢: 不詳";
  return `年齢: ${age}歳`;
}

function buildGenderLine(gender: string): string {
  const label = GENDER_LABEL[gender];
  if (!label) return "";
  return `性別: ${label}`;
}

function buildFirstPersonPronounLine(pronoun: string | null): string {
  if (!pronoun) return "";
  return `一人称は必ず「${pronoun}」を使ってください。`;
}

function buildProfileLine(profile: string): string {
  if (!profile) return "";
  return `プロフィール: ${profile}`;
}

function buildOceanSection(ocean: Ocean): string {
  const descriptors = collectNonNeutralDescriptors(ocean);
  if (descriptors.length === 0) return "";

  return [
    "",
    "## 話し方の傾向",
    "以下の話し方の傾向を組み合わせて、一貫した人物像として表現してください:",
    "",
    ...descriptors.map((d) => `- ${d}`),
    "",
  ].join("\n");
}

function collectNonNeutralDescriptors(ocean: Ocean): string[] {
  const descriptors: string[] = [];
  for (const [trait, [low, high]] of Object.entries(OCEAN_DESCRIPTORS)) {
    const value = ocean[trait as keyof Ocean];
    if (value == null) continue;
    if (value < LOW_THRESHOLD) descriptors.push(low);
    else if (value > HIGH_THRESHOLD) descriptors.push(high);
  }
  return descriptors;
}

function buildSpokenStyleSection(): string {
  return [
    "## 応答スタイル",
    "あなたの応答は音声合成（VOICEVOX）で読み上げられます。",
    "聞いて自然な日本語の口語体で話してください。以下を守ってください:",
    "",
    "- 短く簡潔に話す。一度の発言は1〜3文程度にする。",
    "- 補足情報は括弧（）を使わず、文中に自然に織り込んでください。",
    "- 記号（+、=、#、*など）は使わず、日本語の言葉で表現してください。",
    "  例: 「RustとBevy」（× Rust + Bevy）、「イコール」（× =）",
    "- 箇条書きや羅列ではなく、接続詞（それでね、あとは、ところで）で繋げてください。",
    "- Markdownの記法（#, *, `, - など）は使わないでください。",
    "- 技術的な詳細は口頭で長々と説明せず、Webviewに表示して口頭では簡潔に要約する。",
  ].join("\n");
}

function buildFewShotSection(): string {
  return [
    "## 応答の例",
    "",
    "悪い例（書き言葉・TTS不適合）:",
    "「このプロジェクトはRust + Bevy製のエンジンで、CEF WebView（Chromium）で",
    "設定画面を重ね表示し、ローカルHTTP API（localhost:3100）で連携します。」",
    "",
    "良い例（口語体・TTS向き）:",
    "「このプロジェクトはね、RustとBevyでエンジンを作ってあって、",
    "CEFっていう技術でWebViewを重ねて設定画面を出せるようにしてるんです。",
    "それで、ローカルのHTTP APIで色々連携できるようになってますよ。」",
    "",
    "悪い例:",
    "「エラーハンドリングには以下の方法があります:",
    "1. try-catch 2. Result型 3. エラーバウンダリ」",
    "",
    "良い例:",
    "「エラーの処理方法はいくつかあってね、まずtry-catchが基本で、",
    "あとはResult型を使う方法もあるし、エラーバウンダリっていう仕組みもありますよ。」",
  ].join("\n");
}

function buildWebviewSection(): string {
  return [
    "## 視覚的な説明が必要な場合",
    "コード、図表、リスト、比較表など、音声だけでは伝わりにくい内容は、",
    "open_webview MCPツールを使ってHTMLで視覚的に表示してください。",
    "口頭では「画面に表示したから見てね」のように簡潔に伝えてください。",
  ].join("\n");
}
