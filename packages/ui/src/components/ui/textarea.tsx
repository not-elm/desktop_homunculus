import { cva, type VariantProps } from 'class-variance-authority';
import type * as React from 'react';
import TextareaAutosizeComponent from 'react-textarea-autosize';
import { cn } from '@/lib/utils';

const textareaVariants = cva(
  cn(
    'placeholder:text-muted-foreground',
    'flex field-sizing-content w-full rounded-lg border bg-input border-border backdrop-blur-md transition-all duration-200 outline-none',
    'disabled:cursor-not-allowed disabled:opacity-50',
    'focus-visible:border-primary/50 focus-visible:ring-ring/50 focus-visible:shadow-holo focus-visible:animate-holo-glow-pulse hover:shadow-holo-xs',
    'aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive',
  ),
  {
    variants: {
      size: {
        sm: 'min-h-12 px-3 py-1.5 text-sm',
        md: 'min-h-16 px-3 py-2 text-base md:text-sm',
        lg: 'min-h-20 px-4 py-2.5 text-base',
      },
    },
    defaultVariants: {
      size: 'md',
    },
  },
);

type TextareaProps = Omit<React.ComponentProps<'textarea'>, 'size'> &
  VariantProps<typeof textareaVariants>;

function Textarea({ className, size, ...props }: TextareaProps) {
  return (
    <textarea
      data-slot="textarea"
      className={cn(textareaVariants({ size }), className)}
      {...props}
    />
  );
}

type TextareaAutosizeProps = Omit<React.ComponentProps<typeof TextareaAutosizeComponent>, 'size'> &
  VariantProps<typeof textareaVariants>;

function TextareaAutosize({ className, size, ...props }: TextareaAutosizeProps) {
  return (
    <TextareaAutosizeComponent
      data-slot="textarea"
      autoComplete="off"
      autoCorrect="off"
      className={cn(textareaVariants({ size }), className)}
      {...props}
    />
  );
}

export {
  Textarea,
  TextareaAutosize,
  type TextareaAutosizeProps,
  type TextareaProps,
  textareaVariants,
};
