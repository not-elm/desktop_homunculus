import * as React from "react"
import * as SwitchPrimitive from "@radix-ui/react-switch"

import { cn } from "@/lib/utils"

function Switch({
  className,
  ...props
}: React.ComponentProps<typeof SwitchPrimitive.Root>) {
  return (
    <SwitchPrimitive.Root
      data-slot="switch"
      className={cn(
        "peer data-[state=checked]:bg-primary/40 data-[state=checked]:border-primary/50 data-[state=checked]:shadow-holo data-[state=unchecked]:bg-input focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:shadow-holo focus-visible:animate-holo-glow-pulse hover:shadow-holo-xs inline-flex h-6 w-11 shrink-0 items-center rounded-full border border-border transition-all duration-200 outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 backdrop-blur-md",
        className
      )}
      {...props}
    >
      <SwitchPrimitive.Thumb
        data-slot="switch-thumb"
        className={cn(
          "pointer-events-none block size-5 rounded-full ring-0 transition-all duration-200 data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0",
          "bg-foreground/90 data-[state=checked]:bg-primary data-[state=checked]:shadow-holo-sm"
        )}
      />
    </SwitchPrimitive.Root>
  )
}

export { Switch }
