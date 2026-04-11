import type { ComponentPropsWithoutRef } from 'react';
import { memo, useEffect, useRef } from 'react';
import Markdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import type { LogEntry, LogType } from '../hooks/useAgentSession';

interface ActivityLogProps {
  entries: LogEntry[];
}

export function ActivityLog({ entries }: ActivityLogProps) {
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, []);

  return (
    <div className="hud-log">
      {entries.length === 0 ? (
        <div className="hud-log-empty">Ready to listen...</div>
      ) : (
        entries.map((entry) => <LogRow key={entry.id} entry={entry} />)
      )}
      <div ref={bottomRef} />
    </div>
  );
}

function LogRow({ entry }: { entry: LogEntry }) {
  if (entry.type === 'user') return <UserBubble entry={entry} />;
  if (isSystemEvent(entry.type)) return <SystemRow entry={entry} />;
  if (entry.type === 'assistant') return <AssistantRow entry={entry} />;
  return <StandardRow entry={entry} />;
}

function UserBubble({ entry }: { entry: LogEntry }) {
  return (
    <div
      className="hud-log-entry hud-log-entry--user"
      style={{ animation: 'fade-in 300ms ease both' }}
    >
      <span className="hud-log-text hud-log-text--user">{entry.message}</span>
      <span className="hud-log-icon">
        {entry.source === 'text' ? <KeyboardIcon /> : <MicIcon />}
      </span>
    </div>
  );
}

function StandardRow({ entry }: { entry: LogEntry }) {
  return (
    <div className="hud-log-entry hud-log-entry--standard">
      <span className="hud-log-icon">
        <LogIcon type={entry.type} />
      </span>
      <span className={`hud-log-text ${textClass(entry.type)}`}>{entry.message}</span>
      <span className="hud-log-ts">{formatTime(entry.timestamp)}</span>
    </div>
  );
}

function SystemRow({ entry }: { entry: LogEntry }) {
  return (
    <div className="hud-log-entry hud-log-entry--system">
      <span className="hud-log-icon">
        <LogIcon type={entry.type} />
      </span>
      <span className={`hud-log-text hud-log-text--system ${textClass(entry.type)}`}>
        {entry.message}
      </span>
      <span className="hud-log-ts">{formatTime(entry.timestamp)}</span>
    </div>
  );
}

function isSystemEvent(type: LogType): boolean {
  return type === 'read' || type === 'edit' || type === 'run' || type === 'tool';
}

function textClass(type: LogType): string {
  switch (type) {
    case 'assistant':
      return 'hud-log-text--assistant';
    case 'done':
      return 'hud-log-text--done';
    case 'error':
      return 'hud-log-text--error';
    case 'interrupt':
      return 'hud-log-text--interrupt';
    default:
      return '';
  }
}

function LogIcon({ type }: { type: LogType }) {
  switch (type) {
    case 'read':
      return <ReadIcon />;
    case 'edit':
      return <EditIcon />;
    case 'run':
      return <RunIcon />;
    case 'tool':
      return <ToolIcon />;
    case 'assistant':
      return <DiamondIcon />;
    case 'done':
      return <DoneIcon />;
    case 'error':
      return <ErrorIcon />;
    case 'warning':
      return <WarningIcon />;
    case 'interrupt':
      return <InterruptIcon />;
    case 'user':
      return <MicIcon />;
    default:
      return <DotIcon />;
  }
}

function MicIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
      <rect x="4" y="1" width="4" height="6" rx="2" fill="oklch(0.75 0.18 30)" />
      <path
        d="M2.5 5.5V6a3.5 3.5 0 0 0 7 0V5.5"
        stroke="oklch(0.75 0.18 30)"
        strokeWidth="1.1"
        strokeLinecap="round"
      />
      <path d="M6 9.5V11" stroke="oklch(0.75 0.18 30)" strokeWidth="1.1" strokeLinecap="round" />
    </svg>
  );
}

function KeyboardIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
      <rect
        x="1"
        y="3"
        width="10"
        height="7"
        rx="1"
        stroke="oklch(0.72 0.18 250)"
        strokeWidth="1.1"
      />
      <path
        d="M3 5.5h1M5.5 5.5h1M8 5.5h1M4 7.5h4"
        stroke="oklch(0.72 0.18 250)"
        strokeWidth="0.8"
        strokeLinecap="round"
      />
    </svg>
  );
}

function ReadIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 10 10" fill="none">
      <path
        d="M1 2h8M1 5h5M1 8h3"
        stroke="oklch(0.72 0.18 192)"
        strokeWidth="1.2"
        strokeLinecap="round"
      />
    </svg>
  );
}

function EditIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 10 10" fill="none">
      <path
        d="M7 1L9 3L3 9H1V7L7 1Z"
        stroke="oklch(0.78 0.16 75)"
        strokeWidth="1.1"
        strokeLinejoin="round"
      />
    </svg>
  );
}

function RunIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 10 10" fill="none">
      <path d="M2 1.5L8 5L2 8.5V1.5Z" fill="oklch(0.78 0.16 75)" />
    </svg>
  );
}

function ToolIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 10 10" fill="none">
      <circle cx="5" cy="5" r="3.5" stroke="oklch(0.72 0.06 250 / 0.6)" strokeWidth="1.1" />
      <path
        d="M5 3v2.5M5 7v.3"
        stroke="oklch(0.72 0.06 250 / 0.6)"
        strokeWidth="1.1"
        strokeLinecap="round"
      />
    </svg>
  );
}

function DiamondIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 10 10" fill="none">
      <path d="M5 1L9 5L5 9L1 5L5 1Z" stroke="oklch(0.72 0.18 192)" strokeWidth="1.1" />
    </svg>
  );
}

function DoneIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 10 10" fill="none">
      <path
        d="M1.5 5L4 7.5L8.5 2.5"
        stroke="oklch(0.65 0.18 145)"
        strokeWidth="1.3"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  );
}

function ErrorIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 10 10" fill="none">
      <path
        d="M5 1L9.5 9H.5L5 1Z"
        stroke="oklch(0.65 0.2 20)"
        strokeWidth="1.1"
        strokeLinejoin="round"
      />
      <path
        d="M5 4.5V6.5M5 7.5V8"
        stroke="oklch(0.65 0.2 20)"
        strokeWidth="1.1"
        strokeLinecap="round"
      />
    </svg>
  );
}

function WarningIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 10 10" fill="none">
      <path
        d="M5 2L8.5 8H1.5L5 2Z"
        stroke="oklch(0.78 0.16 75)"
        strokeWidth="1.1"
        strokeLinejoin="round"
      />
      <path
        d="M5 4.5V6M5 7V7.5"
        stroke="oklch(0.78 0.16 75)"
        strokeWidth="1.1"
        strokeLinecap="round"
      />
    </svg>
  );
}

function InterruptIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 10 10" fill="none">
      <circle cx="5" cy="5" r="4" stroke="oklch(0.75 0.15 55)" strokeWidth="1.1" />
      <rect x="3.5" y="3.5" width="3" height="3" rx="0.5" fill="oklch(0.75 0.15 55)" />
    </svg>
  );
}

function DotIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 10 10" fill="none">
      <circle cx="5" cy="5" r="2" fill="oklch(0.55 0.02 250 / 0.5)" />
    </svg>
  );
}

function formatTime(timestamp: number): string {
  const d = new Date(timestamp);
  const h = String(d.getHours()).padStart(2, '0');
  const m = String(d.getMinutes()).padStart(2, '0');
  const s = String(d.getSeconds()).padStart(2, '0');
  return `${h}:${m}:${s}`;
}

const UNSAFE_PROTOCOL = /^(javascript|file|data|vbscript):/i;

function isSafeUrl(href: string | undefined): boolean {
  if (!href) return false;
  return !UNSAFE_PROTOCOL.test(href);
}

const REMARK_PLUGINS = [remarkGfm];
const DISALLOWED_ELEMENTS = ['table', 'thead', 'tbody', 'tr', 'th', 'td', 'img'];

const mdComponents = {
  p: ({ children }: ComponentPropsWithoutRef<'p'>) => (
    <p className="hud-md-paragraph">{children}</p>
  ),
  h1: ({ children }: ComponentPropsWithoutRef<'h1'>) => (
    <h1 className="hud-md-heading">{children}</h1>
  ),
  h2: ({ children }: ComponentPropsWithoutRef<'h2'>) => (
    <h2 className="hud-md-heading">{children}</h2>
  ),
  h3: ({ children }: ComponentPropsWithoutRef<'h3'>) => (
    <h3 className="hud-md-heading">{children}</h3>
  ),
  code: ({ className, children, ...props }: ComponentPropsWithoutRef<'code'>) => {
    const isBlock =
      className?.includes('language-') || (typeof children === 'string' && children.includes('\n'));
    return isBlock ? (
      <code className={`hud-md-code-block ${className ?? ''}`} {...props}>
        {children}
      </code>
    ) : (
      <code className="hud-md-code-inline" {...props}>
        {children}
      </code>
    );
  },
  pre: ({ children }: ComponentPropsWithoutRef<'pre'>) => <>{children}</>,
  blockquote: ({ children }: ComponentPropsWithoutRef<'blockquote'>) => (
    <blockquote className="hud-md-blockquote">{children}</blockquote>
  ),
  a: ({ href, children }: ComponentPropsWithoutRef<'a'>) => {
    if (!isSafeUrl(href)) {
      return <span className="hud-md-link--disabled">{children}</span>;
    }
    return (
      <a
        className="hud-md-link"
        href={href}
        rel="noopener noreferrer"
        onClick={(e) => {
          e.preventDefault();
          window.open(href, '_blank');
        }}
      >
        {children}
      </a>
    );
  },
  ul: ({ children }: ComponentPropsWithoutRef<'ul'>) => <ul className="hud-md-list">{children}</ul>,
  ol: ({ children }: ComponentPropsWithoutRef<'ol'>) => <ol className="hud-md-list">{children}</ol>,
  li: ({ children }: ComponentPropsWithoutRef<'li'>) => (
    <li className="hud-md-list-item">{children}</li>
  ),
  strong: ({ children }: ComponentPropsWithoutRef<'strong'>) => (
    <strong className="hud-md-strong">{children}</strong>
  ),
  em: ({ children }: ComponentPropsWithoutRef<'em'>) => <em className="hud-md-em">{children}</em>,
};

const AssistantRow = memo(function AssistantRow({ entry }: { entry: LogEntry }) {
  return (
    <div className="hud-log-entry hud-log-entry--standard">
      <span className="hud-log-icon">
        <DiamondIcon />
      </span>
      <div className="hud-log-text hud-log-text--assistant">
        <Markdown
          remarkPlugins={REMARK_PLUGINS}
          components={mdComponents}
          disallowedElements={DISALLOWED_ELEMENTS}
        >
          {entry.message}
        </Markdown>
      </div>
      <span className="hud-log-ts">{formatTime(entry.timestamp)}</span>
    </div>
  );
});
