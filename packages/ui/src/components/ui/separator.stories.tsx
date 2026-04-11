import type { Meta, StoryObj } from '@storybook/react-vite';
import { fn } from 'storybook/test';
import { Separator } from './separator';

const meta = {
  title: 'UI/Layout/Separator',
  component: Separator,
  args: {
    onClick: fn(),
  },
  argTypes: {
    orientation: {
      control: 'select',
      options: ['horizontal', 'vertical'],
    },
    decorative: {
      control: 'boolean',
    },
  },
} satisfies Meta<typeof Separator>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default horizontal separator dividing content */
export const Default: Story = {
  render: () => (
    <div className="w-[300px]">
      <p className="text-sm text-muted-foreground">Content above</p>
      <Separator className="my-4" />
      <p className="text-sm text-muted-foreground">Content below</p>
    </div>
  ),
};

/** Horizontal separator between stacked sections */
export const Horizontal: Story = {
  render: () => (
    <div className="w-[300px] space-y-4">
      <div>
        <h4 className="text-sm font-medium">Section One</h4>
        <p className="text-sm text-muted-foreground">First section content goes here.</p>
      </div>
      <Separator />
      <div>
        <h4 className="text-sm font-medium">Section Two</h4>
        <p className="text-sm text-muted-foreground">Second section content goes here.</p>
      </div>
      <Separator />
      <div>
        <h4 className="text-sm font-medium">Section Three</h4>
        <p className="text-sm text-muted-foreground">Third section content goes here.</p>
      </div>
    </div>
  ),
};

/** Vertical separator between inline elements */
export const Vertical: Story = {
  render: () => (
    <div className="flex h-5 items-center gap-4 text-sm">
      <span>Home</span>
      <Separator orientation="vertical" />
      <span>Settings</span>
      <Separator orientation="vertical" />
      <span>Profile</span>
    </div>
  ),
};

/** Separator used with a label in between */
export const WithLabel: Story = {
  render: () => (
    <div className="w-[300px]">
      <div className="flex items-center gap-4">
        <Separator className="flex-1" />
        <span className="text-xs text-muted-foreground">OR</span>
        <Separator className="flex-1" />
      </div>
    </div>
  ),
};
