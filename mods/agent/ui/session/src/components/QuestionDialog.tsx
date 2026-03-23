import type { PendingQuestion } from "../hooks/useAgentSession";

interface AskUserQuestion {
  question: string;
  options?: string[];
}

interface QuestionDialogProps {
  question: PendingQuestion | null;
  onAnswer: (requestId: string, answers: Record<string, string>) => void;
}

export function QuestionDialog({ question, onAnswer }: QuestionDialogProps) {
  if (!question) return null;

  const parsed = parseQuestions(question.questions);

  return (
    <div className="hud-dialog">
      {parsed.map((q, index) => (
        <QuestionItem
          key={index}
          question={q}
          onSelect={(answer) => onAnswer(question.requestId, { [q.question]: answer })}
        />
      ))}
    </div>
  );
}

function QuestionItem({
  question,
  onSelect,
}: {
  question: AskUserQuestion;
  onSelect: (answer: string) => void;
}) {
  return (
    <div>
      <div className="hud-question-text">{question.question}</div>
      {question.options && question.options.length > 0 && (
        <div className="hud-question-options">
          {question.options.map((option) => (
            <button
              key={option}
              className="hud-btn hud-btn--option"
              onClick={() => onSelect(option)}
            >
              {option}
            </button>
          ))}
        </div>
      )}
    </div>
  );
}

function parseQuestions(raw: unknown): AskUserQuestion[] {
  if (Array.isArray(raw)) {
    return raw.map(normalizeQuestion);
  }
  if (typeof raw === "string") {
    return [{ question: raw, options: [] }];
  }
  if (raw && typeof raw === "object") {
    return [normalizeQuestion(raw)];
  }
  return [];
}

function normalizeQuestion(item: unknown): AskUserQuestion {
  if (typeof item === "string") {
    return { question: item, options: [] };
  }
  const obj = item as Record<string, unknown>;
  return {
    question: typeof obj["question"] === "string" ? obj["question"] : String(obj),
    options: Array.isArray(obj["options"])
      ? obj["options"].map(String)
      : [],
  };
}
