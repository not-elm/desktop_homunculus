import type { Meta, StoryObj } from "@storybook/react-vite";
import { fn } from "storybook/test";
import * as React from "react";
import { NumericSlider } from "./NumericSlider";

const meta = {
  title: "Custom/NumericSlider",
  component: NumericSlider,
  args: {
    onValueChange: fn(),
    label: "Value",
    value: [50],
    min: 0,
    max: 100,
    step: 1,
  },
  argTypes: {
    label: {
      control: "text",
    },
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
} satisfies Meta<typeof NumericSlider>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default numeric slider with label and value display */
export const Default: Story = {
  render: function DefaultStory() {
    const [value, setValue] = React.useState([50]);
    return (
      <div className="w-80">
        <NumericSlider
          label="Brightness"
          value={value}
          onValueChange={setValue}
          min={0}
          max={100}
          step={1}
        />
      </div>
    );
  },
};

/** Numeric slider with a custom range from 0 to 360 */
export const WithRange: Story = {
  render: function RangeStory() {
    const [value, setValue] = React.useState([180]);
    return (
      <div className="w-80">
        <NumericSlider
          label="Rotation (degrees)"
          value={value}
          onValueChange={setValue}
          min={0}
          max={360}
          step={5}
        />
      </div>
    );
  },
};

/** Disabled numeric slider that cannot be interacted with */
export const Disabled: Story = {
  render: () => (
    <div className="w-80">
      <NumericSlider
        label="Locked Setting"
        value={[30]}
        onValueChange={fn()}
        min={0}
        max={100}
        disabled
      />
    </div>
  ),
};

/** Numeric slider with fine-grained decimal steps */
export const FineGrained: Story = {
  render: function FineGrainedStory() {
    const [value, setValue] = React.useState([1.5]);
    return (
      <div className="w-80">
        <NumericSlider
          label="Scale Factor"
          value={value}
          onValueChange={setValue}
          min={0.1}
          max={3.0}
          step={0.1}
        />
      </div>
    );
  },
};
