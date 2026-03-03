import type { Meta, StoryObj } from "@storybook/react-vite";
import { fn } from "storybook/test";
import { Textarea, TextareaAutosize } from "./textarea";
import { Label } from "./label";

const meta = {
  title: "UI/Forms/Textarea",
  component: Textarea,
  args: {
    onChange: fn(),
    placeholder: "Type your message...",
  },
  argTypes: {
    disabled: {
      control: "boolean",
    },
    placeholder: {
      control: "text",
    },
    rows: {
      control: "number",
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
    placeholder: "This textarea is disabled",
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
      <p className="text-destructive text-sm">
        Bio must be at least 10 characters.
      </p>
    </div>
  ),
};

/** Textarea with a pre-filled default value and custom rows */
export const WithDefaultValue: Story = {
  args: {
    defaultValue:
      "This is a textarea with some pre-filled content.\nIt spans multiple lines.",
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
