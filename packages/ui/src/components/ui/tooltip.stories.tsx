import type { Meta, StoryObj } from '@storybook/react-vite';
import { Info, Plus, Save, Settings, Trash2 } from 'lucide-react';
import { fn } from 'storybook/test';
import { Button } from './button';
import { Tooltip, TooltipContent, TooltipTrigger } from './tooltip';

const meta = {
  title: 'UI/Overlays/Tooltip',
  component: Tooltip,
  args: {
    onOpenChange: fn(),
  },
} satisfies Meta<typeof Tooltip>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default tooltip wrapping a button */
export const Default: Story = {
  render: (args) => (
    <Tooltip {...args}>
      <TooltipTrigger asChild>
        <Button variant="outline">Hover Me</Button>
      </TooltipTrigger>
      <TooltipContent>
        <p>This is a tooltip</p>
      </TooltipContent>
    </Tooltip>
  ),
};

/** Tooltip on an icon button */
export const IconButton: Story = {
  render: (args) => (
    <Tooltip {...args}>
      <TooltipTrigger asChild>
        <Button size="icon" variant="outline">
          <Plus />
        </Button>
      </TooltipTrigger>
      <TooltipContent>
        <p>Add New Item</p>
      </TooltipContent>
    </Tooltip>
  ),
};

/** Tooltips on all four sides of a button */
export const AllSides: Story = {
  render: () => (
    <div className="flex flex-col items-center gap-16 py-16">
      <Tooltip>
        <TooltipTrigger asChild>
          <Button variant="outline">Top</Button>
        </TooltipTrigger>
        <TooltipContent side="top">
          <p>Tooltip on top</p>
        </TooltipContent>
      </Tooltip>
      <div className="flex items-center gap-16">
        <Tooltip>
          <TooltipTrigger asChild>
            <Button variant="outline">Left</Button>
          </TooltipTrigger>
          <TooltipContent side="left">
            <p>Tooltip on left</p>
          </TooltipContent>
        </Tooltip>
        <Tooltip>
          <TooltipTrigger asChild>
            <Button variant="outline">Right</Button>
          </TooltipTrigger>
          <TooltipContent side="right">
            <p>Tooltip on right</p>
          </TooltipContent>
        </Tooltip>
      </div>
      <Tooltip>
        <TooltipTrigger asChild>
          <Button variant="outline">Bottom</Button>
        </TooltipTrigger>
        <TooltipContent side="bottom">
          <p>Tooltip on bottom</p>
        </TooltipContent>
      </Tooltip>
    </div>
  ),
};

/** Toolbar with multiple icon buttons, each with a tooltip */
export const Toolbar: Story = {
  render: () => (
    <div className="flex items-center gap-1 rounded-lg border border-border p-1">
      <Tooltip>
        <TooltipTrigger asChild>
          <Button size="icon" variant="ghost">
            <Save />
          </Button>
        </TooltipTrigger>
        <TooltipContent>
          <p>Save (Cmd+S)</p>
        </TooltipContent>
      </Tooltip>
      <Tooltip>
        <TooltipTrigger asChild>
          <Button size="icon" variant="ghost">
            <Settings />
          </Button>
        </TooltipTrigger>
        <TooltipContent>
          <p>Settings</p>
        </TooltipContent>
      </Tooltip>
      <Tooltip>
        <TooltipTrigger asChild>
          <Button size="icon" variant="ghost">
            <Info />
          </Button>
        </TooltipTrigger>
        <TooltipContent>
          <p>About</p>
        </TooltipContent>
      </Tooltip>
      <Tooltip>
        <TooltipTrigger asChild>
          <Button size="icon" variant="ghost">
            <Trash2 />
          </Button>
        </TooltipTrigger>
        <TooltipContent>
          <p>Delete (Del)</p>
        </TooltipContent>
      </Tooltip>
    </div>
  ),
};
