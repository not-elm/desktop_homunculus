import type { Meta, StoryObj } from "@storybook/react-vite";
import { fn } from "storybook/test";
import { Slider } from "./slider";
import { Label } from "./label";

const meta = {
  title: "UI/Forms/Slider",
  component: Slider,
  args: {
    onValueChange: fn(),
    onValueCommit: fn(),
    defaultValue: [50],
    min: 0,
    max: 100,
    step: 1,
  },
  argTypes: {
    min: {
      control: "number",
    },
    max: {
      control: "number",
    },
    step: {
      control: "number",
    },
    disabled: {
      control: "boolean",
    },
  },
} satisfies Meta<typeof Slider>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default slider at 50% */
export const Default: Story = {};

/** Slider with a label showing the value range */
export const WithLabel: Story = {
  render: (args) => (
    <div className="grid w-64 gap-3">
      <Label>Volume</Label>
      <Slider {...args} defaultValue={[75]} />
    </div>
  ),
};

/** Range slider with two thumbs */
export const Range: Story = {
  args: {
    defaultValue: [25, 75],
  },
  render: (args) => (
    <div className="grid w-64 gap-3">
      <Label>Price Range</Label>
      <Slider {...args} />
    </div>
  ),
};

/** Slider with custom step and min/max values */
export const CustomRange: Story = {
  args: {
    defaultValue: [5],
    min: 0,
    max: 10,
    step: 0.5,
  },
  render: (args) => (
    <div className="grid w-64 gap-3">
      <Label>Rating (0 - 10, step 0.5)</Label>
      <Slider {...args} />
    </div>
  ),
};

/** Disabled slider */
export const Disabled: Story = {
  args: {
    disabled: true,
    defaultValue: [30],
  },
};

/** Multiple sliders showing different default positions */
export const Positions: Story = {
  render: (args) => (
    <div className="flex flex-col gap-6 w-64">
      <div className="grid gap-2">
        <Label>Low (25%)</Label>
        <Slider {...args} defaultValue={[25]} />
      </div>
      <div className="grid gap-2">
        <Label>Mid (50%)</Label>
        <Slider {...args} defaultValue={[50]} />
      </div>
      <div className="grid gap-2">
        <Label>High (75%)</Label>
        <Slider {...args} defaultValue={[75]} />
      </div>
    </div>
  ),
};
