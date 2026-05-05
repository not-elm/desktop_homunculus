import { Slot } from '@radix-ui/react-slot';
import { cva, type VariantProps } from 'class-variance-authority';
import type * as React from 'react';

import { cn } from '@/lib/utils';

const buttonVariants = cva(
  "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-lg text-sm font-semibold transition-all duration-200 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] focus-visible:animate-holo-glow-pulse aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive active:scale-[0.97] active:shadow-holo-intense",
  {
    variants: {
      variant: {
        default:
          'bg-primary/20 backdrop-blur-md border border-primary/30 text-foreground hover:bg-primary/30 hover:border-primary/50 hover:shadow-holo-intense',
        destructive:
          'bg-destructive/20 backdrop-blur-md border border-destructive/30 text-foreground hover:bg-destructive/30 hover:border-destructive/50 hover:shadow-glow-destructive focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40',
        outline:
          'border backdrop-blur-md bg-input border-border text-foreground hover:bg-accent hover:border-primary/30 hover:shadow-holo',
        secondary:
          'bg-secondary/20 backdrop-blur-md border border-secondary/30 text-foreground hover:bg-secondary/30 hover:border-secondary/50',
        ghost: 'text-foreground hover:bg-accent hover:text-accent-foreground active:shadow-none',
        link: 'text-primary underline-offset-4 hover:underline active:shadow-none',
        hud: 'border border-primary/30 bg-primary/15 text-primary hover:border-primary/50 hover:bg-primary/25 hover:shadow-holo-sm',
        'hud-success':
          'border border-success/40 bg-success/15 text-success hover:border-success/50 hover:bg-success/20 hover:shadow-holo-sm',
        'hud-destructive':
          'border border-destructive/30 bg-destructive/15 text-destructive hover:border-destructive/50 hover:bg-destructive/25',
        'hud-ghost':
          'border border-muted-foreground/25 bg-transparent text-muted-foreground hover:border-muted-foreground/45 hover:text-foreground',
        'hud-rose':
          'border border-holo-rose/30 bg-holo-rose/15 text-holo-rose hover:border-holo-rose/50 hover:bg-holo-rose/25',
      },
      size: {
        default: 'h-10 px-5 py-2 has-[>svg]:px-3',
        sm: 'h-9 rounded-md gap-1.5 px-4 has-[>svg]:px-2.5',
        lg: 'h-11 rounded-md px-7 has-[>svg]:px-4',
        icon: 'size-10',
        hud: 'h-auto rounded-md px-5 py-2 text-xs font-medium uppercase tracking-[0.08em]',
        'hud-sm': 'h-auto rounded-md px-4 py-1.5 text-xs font-medium uppercase tracking-[0.08em]',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'default',
    },
  },
);

function Button({
  className,
  variant,
  size,
  asChild = false,
  ...props
}: React.ComponentProps<'button'> &
  VariantProps<typeof buttonVariants> & {
    asChild?: boolean;
  }) {
  const Comp = asChild ? Slot : 'button';

  return (
    <Comp
      data-slot="button"
      className={cn(buttonVariants({ variant, size, className }))}
      {...props}
    />
  );
}

export { Button, buttonVariants };
