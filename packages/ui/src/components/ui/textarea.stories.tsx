import type { Meta, StoryObj } from '@storybook/react-vite';
import { fn } from 'storybook/test';
import { Label } from './label';
import { Textarea, TextareaAutosize } from './textarea';

const meta = {
  title: 'UI/Forms/Textarea',
  component: Textarea,
  args: {
    onChange: fn(),
    placeholder: 'Type your message...',
  },
  argTypes: {
    disabled: {
      control: 'boolean',
    },
    placeholder: {
      control: 'text',
    },
    rows: {
      control: 'number',
    },
  },
} satisfies Meta<typeof Textarea>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default textarea with placeholder */
export const Default: Story = {};

/** Textarea with a label */
export const WithLabel: Story = {
  render: () => (
    <div className="grid w-96 gap-2">
      <Label htmlFor="message">Your message</Label>
      <Textarea id="message" placeholder="Write something..." />
    </div>
  ),
};

/** Textarea in disabled state */
export const Disabled: Story = {
  args: {
    disabled: true,
    placeholder: 'This textarea is disabled',
  },
};

/** Textarea with aria-invalid for error styling */
export const WithError: Story = {
  render: () => (
    <div className="grid w-96 gap-2">
      <Label htmlFor="bio">Bio</Label>
      <Textarea
        id="bio"
        aria-invalid="true"
        defaultValue="Hi"
        placeholder="Tell us about yourself"
      />
      <p className="text-destructive text-sm">Bio must be at least 10 characters.</p>
    </div>
  ),
};

/** Textarea with a pre-filled default value and custom rows */
export const WithDefaultValue: Story = {
  args: {
    defaultValue: 'This is a textarea with some pre-filled content.\nIt spans multiple lines.',
    rows: 5,
  },
};

/** Autosize textarea that grows with content */
export const Autosize: Story = {
  render: () => (
    <div className="grid w-96 gap-2">
      <Label htmlFor="autosize">Auto-growing textarea</Label>
      <TextareaAutosize
        id="autosize"
        placeholder="Start typing and the textarea will grow..."
        minRows={2}
        maxRows={8}
      />
    </div>
  ),
};

/** Size variants: sm / md (default) / lg */
export const Sizes: Story = {
  render: () => (
    <div className="flex flex-col gap-3 w-80">
      <Textarea size="sm" placeholder="Small (min-h-12)" />
      <Textarea size="md" placeholder="Medium (min-h-16, default)" />
      <Textarea size="lg" placeholder="Large (min-h-20)" />
    </div>
  ),
};
