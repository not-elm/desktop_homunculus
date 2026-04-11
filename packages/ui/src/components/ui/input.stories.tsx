import type { Meta, StoryObj } from '@storybook/react-vite';
import { Mail, Search } from 'lucide-react';
import { fn } from 'storybook/test';
import { Input } from './input';
import { Label } from './label';

const meta = {
  title: 'UI/Forms/Input',
  component: Input,
  args: {
    onChange: fn(),
    placeholder: 'Enter text...',
  },
  argTypes: {
    type: {
      control: 'select',
      options: ['text', 'email', 'password', 'number', 'search', 'tel', 'url'],
    },
    disabled: {
      control: 'boolean',
    },
    placeholder: {
      control: 'text',
    },
  },
} satisfies Meta<typeof Input>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default text input with placeholder */
export const Default: Story = {};

/** Input field types side by side */
export const Types: Story = {
  render: () => (
    <div className="flex flex-col gap-4 w-80">
      <Input type="text" placeholder="Text input" />
      <Input type="email" placeholder="Email input" />
      <Input type="password" placeholder="Password input" />
      <Input type="number" placeholder="Number input" />
      <Input type="search" placeholder="Search input" />
    </div>
  ),
};

/** Input with a label above it */
export const WithLabel: Story = {
  render: () => (
    <div className="grid w-80 gap-2">
      <Label htmlFor="email-input">Email address</Label>
      <Input id="email-input" type="email" placeholder="you@example.com" />
    </div>
  ),
};

/** Input showing an icon next to it */
export const WithIcon: Story = {
  render: () => (
    <div className="flex flex-col gap-4 w-80">
      <div className="relative">
        <Search className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground" />
        <Input className="pl-9" type="search" placeholder="Search..." />
      </div>
      <div className="relative">
        <Mail className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground" />
        <Input className="pl-9" type="email" placeholder="Email address" />
      </div>
    </div>
  ),
};

/** Disabled state */
export const Disabled: Story = {
  args: {
    disabled: true,
    placeholder: 'Disabled input',
  },
};

/** Input with aria-invalid for error styling */
export const WithError: Story = {
  render: () => (
    <div className="grid w-80 gap-2">
      <Label htmlFor="error-input">Username</Label>
      <Input id="error-input" aria-invalid="true" defaultValue="ab" placeholder="Username" />
      <p className="text-destructive text-sm">Username must be at least 3 characters.</p>
    </div>
  ),
};

/** File input variant */
export const FileInput: Story = {
  args: {
    type: 'file',
    placeholder: undefined,
  },
};
