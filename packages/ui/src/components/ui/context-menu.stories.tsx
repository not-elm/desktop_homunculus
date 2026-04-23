import type { Meta, StoryObj } from '@storybook/react-vite';
import { Clipboard, Copy, Eye, RotateCcw, Scissors, Share2, Trash2 } from 'lucide-react';
import { fn } from 'storybook/test';
import {
  ContextMenu,
  ContextMenuCheckboxItem,
  ContextMenuContent,
  ContextMenuGroup,
  ContextMenuItem,
  ContextMenuLabel,
  ContextMenuRadioGroup,
  ContextMenuRadioItem,
  ContextMenuSeparator,
  ContextMenuShortcut,
  ContextMenuSub,
  ContextMenuSubContent,
  ContextMenuSubTrigger,
  ContextMenuTrigger,
} from './context-menu';

const meta = {
  title: 'UI/Overlays/ContextMenu',
  component: ContextMenu,
  args: {
    onOpenChange: fn(),
  },
} satisfies Meta<typeof ContextMenu>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default context menu triggered by right-clicking an area */
export const Default: Story = {
  render: (args) => (
    <ContextMenu {...args}>
      <ContextMenuTrigger className="flex h-[200px] w-[350px] items-center justify-center rounded-lg border border-dashed border-border text-sm text-muted-foreground">
        Right-click here
      </ContextMenuTrigger>
      <ContextMenuContent className="w-56">
        <ContextMenuItem>
          <Copy />
          Copy
          <ContextMenuShortcut>Cmd+C</ContextMenuShortcut>
        </ContextMenuItem>
        <ContextMenuItem>
          <Scissors />
          Cut
          <ContextMenuShortcut>Cmd+X</ContextMenuShortcut>
        </ContextMenuItem>
        <ContextMenuItem>
          <Clipboard />
          Paste
          <ContextMenuShortcut>Cmd+V</ContextMenuShortcut>
        </ContextMenuItem>
        <ContextMenuSeparator />
        <ContextMenuItem variant="destructive">
          <Trash2 />
          Delete
          <ContextMenuShortcut>Del</ContextMenuShortcut>
        </ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  ),
};

/** Context menu with a submenu for sharing options */
export const WithSubmenu: Story = {
  render: (args) => (
    <ContextMenu {...args}>
      <ContextMenuTrigger className="flex h-[200px] w-[350px] items-center justify-center rounded-lg border border-dashed border-border text-sm text-muted-foreground">
        Right-click for options
      </ContextMenuTrigger>
      <ContextMenuContent className="w-56">
        <ContextMenuGroup>
          <ContextMenuItem>
            <Eye />
            View Details
          </ContextMenuItem>
          <ContextMenuItem>
            <Copy />
            Duplicate
          </ContextMenuItem>
        </ContextMenuGroup>
        <ContextMenuSeparator />
        <ContextMenuSub>
          <ContextMenuSubTrigger>
            <Share2 />
            Share
          </ContextMenuSubTrigger>
          <ContextMenuSubContent className="w-48">
            <ContextMenuItem>Email</ContextMenuItem>
            <ContextMenuItem>Link</ContextMenuItem>
            <ContextMenuItem>Export as File</ContextMenuItem>
          </ContextMenuSubContent>
        </ContextMenuSub>
        <ContextMenuSeparator />
        <ContextMenuItem>
          <RotateCcw />
          Undo
          <ContextMenuShortcut>Cmd+Z</ContextMenuShortcut>
        </ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  ),
};

/** Context menu with checkbox items for toggling visibility */
export const WithCheckboxItems: Story = {
  render: (args) => (
    <ContextMenu {...args}>
      <ContextMenuTrigger className="flex h-[200px] w-[350px] items-center justify-center rounded-lg border border-dashed border-border text-sm text-muted-foreground">
        Right-click to toggle layers
      </ContextMenuTrigger>
      <ContextMenuContent className="w-56">
        <ContextMenuLabel>Visible Layers</ContextMenuLabel>
        <ContextMenuSeparator />
        <ContextMenuCheckboxItem checked>Background</ContextMenuCheckboxItem>
        <ContextMenuCheckboxItem checked>Character</ContextMenuCheckboxItem>
        <ContextMenuCheckboxItem>Grid Overlay</ContextMenuCheckboxItem>
        <ContextMenuCheckboxItem checked>UI Elements</ContextMenuCheckboxItem>
      </ContextMenuContent>
    </ContextMenu>
  ),
};

/** Context menu with radio items for selecting a single option */
export const WithRadioItems: Story = {
  render: (args) => (
    <ContextMenu {...args}>
      <ContextMenuTrigger className="flex h-[200px] w-[350px] items-center justify-center rounded-lg border border-dashed border-border text-sm text-muted-foreground">
        Right-click to select quality
      </ContextMenuTrigger>
      <ContextMenuContent className="w-56">
        <ContextMenuLabel>Render Quality</ContextMenuLabel>
        <ContextMenuSeparator />
        <ContextMenuRadioGroup value="high">
          <ContextMenuRadioItem value="low">Low</ContextMenuRadioItem>
          <ContextMenuRadioItem value="medium">Medium</ContextMenuRadioItem>
          <ContextMenuRadioItem value="high">High</ContextMenuRadioItem>
          <ContextMenuRadioItem value="ultra">Ultra</ContextMenuRadioItem>
        </ContextMenuRadioGroup>
      </ContextMenuContent>
    </ContextMenu>
  ),
};
