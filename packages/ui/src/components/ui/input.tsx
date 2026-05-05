import { cva, type VariantProps } from 'class-variance-authority';
import type * as React from 'react';

import { cn } from '@/lib/utils';

const inputVariants = cva(
  cn(
    'file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground',
    'flex w-full min-w-0 rounded-lg border border-border bg-input backdrop-blur-md transition-all duration-200 outline-none',
    'file:inline-flex file:border-0 file:bg-transparent file:font-medium',
    'disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50',
    'focus-visible:border-primary/50 focus-visible:ring-ring/50 focus-visible:ring-[3px] focus-visible:shadow-holo focus-visible:animate-holo-glow-pulse hover:shadow-holo-xs',
    'aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive',
  ),
  {
    variants: {
      size: {
        sm: 'h-9 px-3 py-1.5 text-sm file:h-6 file:text-xs',
        md: 'h-10 px-3 py-2 text-base md:text-sm file:h-7 file:text-sm',
        lg: 'h-11 px-4 py-2.5 text-base file:h-8 file:text-sm',
      },
    },
    defaultVariants: {
      size: 'md',
    },
  },
);

type InputProps = Omit<React.ComponentProps<'input'>, 'size'> & VariantProps<typeof inputVariants>;

function Input({ className, type, size, ...props }: InputProps) {
  return (
    <input
      type={type}
      data-slot="input"
      className={cn(inputVariants({ size }), className)}
      {...props}
    />
  );
}

export { Input, type InputProps, inputVariants };
