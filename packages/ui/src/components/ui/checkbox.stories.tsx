import type { Meta, StoryObj } from "@storybook/react-vite";
import { fn } from "storybook/test";
import { Checkbox } from "./checkbox";
import { Label } from "./label";

const meta = {
  title: "UI/Forms/Checkbox",
  component: Checkbox,
  args: {
    onCheckedChange: fn(),
  },
  argTypes: {
    disabled: {
      control: "boolean",
    },
    defaultChecked: {
      control: "boolean",
    },
  },
} satisfies Meta<typeof Checkbox>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default unchecked checkbox */
export const Default: Story = {};

/** Checkbox pre-checked by default */
export const Checked: Story = {
  args: {
    defaultChecked: true,
  },
};

/** Checkbox with a label next to it */
export const WithLabel: Story = {
  render: (args) => (
    <div className="flex items-center gap-2">
      <Checkbox id="terms" {...args} />
      <Label htmlFor="terms">Accept terms and conditions</Label>
    </div>
  ),
};

/** Multiple checkboxes as a group */
export const Group: Story = {
  render: (args) => (
    <div className="flex flex-col gap-3">
      <div className="flex items-center gap-2">
        <Checkbox id="opt-email" defaultChecked {...args} />
        <Label htmlFor="opt-email">Email notifications</Label>
      </div>
      <div className="flex items-center gap-2">
        <Checkbox id="opt-sms" {...args} />
        <Label htmlFor="opt-sms">SMS notifications</Label>
      </div>
      <div className="flex items-center gap-2">
        <Checkbox id="opt-push" defaultChecked {...args} />
        <Label htmlFor="opt-push">Push notifications</Label>
      </div>
    </div>
  ),
};

/** Disabled checkbox states */
export const Disabled: Story = {
  render: () => (
    <div className="flex flex-col gap-3">
      <div className="flex items-center gap-2">
        <Checkbox id="disabled-unchecked" disabled />
        <Label htmlFor="disabled-unchecked">Disabled unchecked</Label>
      </div>
      <div className="flex items-center gap-2">
        <Checkbox id="disabled-checked" disabled defaultChecked />
        <Label htmlFor="disabled-checked">Disabled checked</Label>
      </div>
    </div>
  ),
};

/** Checkbox with aria-invalid for error styling */
export const WithError: Story = {
  render: (args) => (
    <div className="flex flex-col gap-2">
      <div className="flex items-center gap-2">
        <Checkbox id="error-terms" aria-invalid="true" {...args} />
        <Label htmlFor="error-terms">I agree to the terms</Label>
      </div>
      <p className="text-destructive text-sm">
        You must accept the terms to continue.
      </p>
    </div>
  ),
};
