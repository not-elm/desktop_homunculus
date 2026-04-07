import { useRef, useState } from "react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { fn } from "storybook/test";
import { Settings, MoreHorizontal, Info, Trash2 } from "lucide-react";
import { Popover, PopoverTrigger, PopoverContent, PopoverAnchor } from "./popover";
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
} from "./dropdown-menu";
import { Button } from "./button";
import { Label } from "./label";
import { Input } from "./input";

const meta = {
  title: "UI/Overlays/Popover",
  component: Popover,
  args: {
    onOpenChange: fn(),
  },
} satisfies Meta<typeof Popover>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Basic popover with trigger button and form content */
export const Default: Story = {
  render: (args) => (
    <Popover {...args}>
      <PopoverTrigger asChild>
        <Button variant="outline">
          <Settings /> Open Popover
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-80">
        <div className="grid gap-4">
          <div className="space-y-2">
            <h4 className="font-medium leading-none">Dimensions</h4>
            <p className="text-sm text-muted-foreground">
              Set the dimensions for the layer.
            </p>
          </div>
          <div className="grid gap-2">
            <div className="grid grid-cols-3 items-center gap-4">
              <Label>Width</Label>
              <Input className="col-span-2 h-8" defaultValue="100%" />
            </div>
            <div className="grid grid-cols-3 items-center gap-4">
              <Label>Height</Label>
              <Input className="col-span-2 h-8" defaultValue="25px" />
            </div>
          </div>
        </div>
      </PopoverContent>
    </Popover>
  ),
};

/** Popover using PopoverAnchor instead of PopoverTrigger (controlled externally) */
export const WithAnchor: Story = {
  render: () => {
    const [open, setOpen] = useState(false);
    return (
      <Popover open={open} onOpenChange={setOpen}>
        <PopoverAnchor asChild>
          <div className="flex items-center gap-2 rounded-md border border-white/20 bg-primary/30 p-3">
            <Info className="size-4" />
            <span className="text-sm">Anchored row — popover attaches here</span>
            <Button
              variant="ghost"
              size="sm"
              className="ml-auto"
              onClick={() => setOpen(true)}
            >
              Show Details
            </Button>
          </div>
        </PopoverAnchor>
        <PopoverContent align="end" sideOffset={8} className="w-auto min-w-[220px]">
          <div className="grid gap-2 text-sm">
            <h4 className="font-medium">Detail Info</h4>
            <div className="grid grid-cols-2 gap-x-4 gap-y-1">
              <span className="text-muted-foreground">Status</span>
              <span>Active</span>
              <span className="text-muted-foreground">Branch</span>
              <span className="font-mono text-xs">feature/demo</span>
              <span className="text-muted-foreground">Commits</span>
              <span>3</span>
            </div>
          </div>
        </PopoverContent>
      </Popover>
    );
  },
};

/**
 * Bug reproduction: DropdownMenu item opens a Popover.
 * This mirrors the WorktreeNode pattern where clicking "Details"
 * in a DropdownMenu should open a Popover on the same row.
 */
export const DropdownThenPopover: Story = {
  render: () => {
    const [showDetail, setShowDetail] = useState(false);
    const anchorRef = useRef<HTMLDivElement>(null);
    return (
      <Popover open={showDetail} onOpenChange={setShowDetail}>
        <PopoverAnchor asChild>
          <div
            ref={anchorRef}
            className="flex items-center gap-2 rounded-md border border-white/20 bg-primary/30 p-3"
          >
            <span className="text-sm font-medium">feature/my-branch</span>
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <button
                  className="ml-auto rounded p-1 hover:bg-white/10"
                  type="button"
                  onClick={(e) => e.stopPropagation()}
                >
                  <MoreHorizontal className="size-4" />
                </button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuItem
                  onClick={(e) => {
                    e.stopPropagation();
                    setShowDetail(true);
                  }}
                >
                  <Info className="size-4" />
                  Details
                </DropdownMenuItem>
                <DropdownMenuItem variant="destructive">
                  <Trash2 className="size-4" />
                  Remove
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        </PopoverAnchor>
        <PopoverContent
          align="end"
          sideOffset={8}
          className="w-auto min-w-[220px]"
          onInteractOutside={(e) => {
            if (anchorRef.current?.contains(e.target as Node)) {
              e.preventDefault();
            }
          }}
        >
          <div className="grid gap-2 text-sm">
            <h4 className="font-medium">Worktree Details</h4>
            <div className="grid grid-cols-2 gap-x-4 gap-y-1">
              <span className="text-muted-foreground">Branch</span>
              <span className="font-mono text-xs">feature/my-branch</span>
              <span className="text-muted-foreground">Base</span>
              <span className="font-mono text-xs">main</span>
              <span className="text-muted-foreground">Commits</span>
              <span>5</span>
              <span className="text-muted-foreground">Files</span>
              <span>3</span>
            </div>
          </div>
        </PopoverContent>
      </Popover>
    );
  },
};

/** Controlled popover with external open/close buttons */
export const Controlled: Story = {
  render: () => {
    const [open, setOpen] = useState(false);
    return (
      <div className="flex items-center gap-4">
        <Popover open={open} onOpenChange={setOpen}>
          <PopoverTrigger asChild>
            <Button variant="outline">Toggle Popover</Button>
          </PopoverTrigger>
          <PopoverContent>
            <p className="text-sm">
              This popover is controlled. Use the buttons to open and close it.
            </p>
          </PopoverContent>
        </Popover>
        <Button variant="secondary" size="sm" onClick={() => setOpen(true)}>
          Open
        </Button>
        <Button variant="secondary" size="sm" onClick={() => setOpen(false)}>
          Close
        </Button>
        <span className="text-xs text-muted-foreground">
          State: {open ? "open" : "closed"}
        </span>
      </div>
    );
  },
};
