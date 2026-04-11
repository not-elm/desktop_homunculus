import type { Meta, StoryObj } from '@storybook/react-vite';
import { Info } from 'lucide-react';
import { fn } from 'storybook/test';
import { Checkbox } from './checkbox';
import { Input } from './input';
import { Label } from './label';

const meta = {
  title: 'UI/Forms/Label',
  component: Label,
  args: {
    children: 'Label text',
    onClick: fn(),
  },
  argTypes: {
    children: {
      control: 'text',
    },
  },
} satisfies Meta<typeof Label>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default standalone label */
export const Default: Story = {};

/** Label paired with an input field */
export const WithInput: Story = {
  render: () => (
    <div className="grid w-80 gap-2">
      <Label htmlFor="name">Full Name</Label>
      <Input id="name" placeholder="Enter your name" />
    </div>
  ),
};

/** Label paired with a checkbox */
export const WithCheckbox: Story = {
  render: () => (
    <div className="flex items-center gap-2">
      <Checkbox id="agree" />
      <Label htmlFor="agree">I agree to the privacy policy</Label>
    </div>
  ),
};

/** Label with an icon inside (uses gap-2 from base styles) */
export const WithIcon: Story = {
  render: () => (
    <Label>
      <Info className="size-4" />
      Informational label
    </Label>
  ),
};

/** Label appearance when associated with a disabled peer input */
export const DisabledPeer: Story = {
  render: () => (
    <div className="grid w-80 gap-2">
      <Label htmlFor="disabled-field">Disabled field</Label>
      <Input id="disabled-field" disabled placeholder="Cannot edit" />
    </div>
  ),
};

/** Multiple labels in a form-like layout */
export const FormLayout: Story = {
  render: () => (
    <div className="flex flex-col gap-4 w-80">
      <div className="grid gap-2">
        <Label htmlFor="first">First Name</Label>
        <Input id="first" placeholder="Jane" />
      </div>
      <div className="grid gap-2">
        <Label htmlFor="last">Last Name</Label>
        <Input id="last" placeholder="Doe" />
      </div>
      <div className="grid gap-2">
        <Label htmlFor="email-form">Email</Label>
        <Input id="email-form" type="email" placeholder="jane@example.com" />
      </div>
    </div>
  ),
};
