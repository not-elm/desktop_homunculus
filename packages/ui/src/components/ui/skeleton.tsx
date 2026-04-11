import { cn } from '@/lib/utils';

function Skeleton({ className, ...props }: React.ComponentProps<'div'>) {
  return (
    <div
      data-slot="skeleton"
      className={cn('bg-muted/50 animate-holo-pulse rounded-md backdrop-blur-sm', className)}
      {...props}
    />
  );
}

export { Skeleton };
