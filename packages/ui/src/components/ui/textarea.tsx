import type * as React from 'react';
import TextareaAutosizeComponent from 'react-textarea-autosize';
import { cn } from '@/lib/utils';

const textareaStyles =
  'placeholder:text-muted-foreground ' +
  'focus-visible:border-primary/50 focus-visible:ring-ring/50 focus-visible:shadow-holo focus-visible:animate-holo-glow-pulse hover:shadow-holo-xs ' +
  'aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 ' +
  'aria-invalid:border-destructive flex field-sizing-content min-h-16 w-full rounded-lg border ' +
  'bg-input border-border px-3 py-2 text-base backdrop-blur-md transition-all duration-200 outline-none ' +
  'disabled:cursor-not-allowed disabled:opacity-50 md:text-sm';

function Textarea({ className, ...props }: React.ComponentProps<'textarea'>) {
  return <textarea data-slot="textarea" className={cn(textareaStyles, className)} {...props} />;
}

// TextareaAutosize component that uses the same styling as the Textarea component
function TextareaAutosize({
  className,
  ...props
}: React.ComponentProps<typeof TextareaAutosizeComponent>) {
  return (
    <TextareaAutosizeComponent
      data-slot="textarea"
      autoComplete="off"
      autoCorrect="off"
      className={cn(textareaStyles, className)}
      {...props}
    />
  );
}

export { Textarea, TextareaAutosize };
