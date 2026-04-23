import type { Meta, StoryObj } from '@storybook/react-vite';
import { Check, Star, Zap } from 'lucide-react';
import { fn } from 'storybook/test';
import { Badge } from './badge';

const meta = {
  title: 'UI/Display/Badge',
  component: Badge,
  args: {
    onClick: fn(),
    children: 'Badge',
  },
  argTypes: {
    variant: {
      control: 'select',
      options: ['default', 'secondary', 'destructive', 'outline'],
    },
    children: {
      control: 'text',
    },
  },
} satisfies Meta<typeof Badge>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default badge with primary styling */
export const Default: Story = {};

/** All variant styles side by side */
export const Variants: Story = {
  render: () => (
    <div className="flex flex-wrap items-center gap-4">
      <Badge variant="default">Default</Badge>
      <Badge variant="secondary">Secondary</Badge>
      <Badge variant="destructive">Destructive</Badge>
      <Badge variant="outline">Outline</Badge>
    </div>
  ),
};

/** Badges with icons to convey status */
export const WithIcon: Story = {
  render: () => (
    <div className="flex flex-wrap items-center gap-4">
      <Badge variant="default">
        <Check /> Verified
      </Badge>
      <Badge variant="secondary">
        <Star /> Featured
      </Badge>
      <Badge variant="destructive">
        <Zap /> Critical
      </Badge>
    </div>
  ),
};

/** Badge rendered as a child link element via asChild */
export const AsLink: Story = {
  render: () => (
    <Badge asChild variant="outline">
      <a href="https://example.com">Clickable Badge</a>
    </Badge>
  ),
};
