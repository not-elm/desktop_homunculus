import type { Meta, StoryObj } from "@storybook/react-vite";
import { fn } from "storybook/test";
import { AspectRatio } from "./aspect-ratio";

const meta = {
  title: "UI/Layout/AspectRatio",
  component: AspectRatio,
  args: {
    onClick: fn(),
  },
} satisfies Meta<typeof AspectRatio>;

export default meta;
type Story = StoryObj<typeof meta>;

/** Default 16:9 aspect ratio container */
export const Default: Story = {
  render: () => (
    <div className="w-[450px]">
      <AspectRatio ratio={16 / 9}>
        <div className="flex size-full items-center justify-center rounded-lg bg-primary/20 border border-border backdrop-blur-sm">
          <span className="text-sm text-muted-foreground">16:9</span>
        </div>
      </AspectRatio>
    </div>
  ),
};

/** 4:3 aspect ratio, common for classic displays */
export const Ratio4By3: Story = {
  render: () => (
    <div className="w-[400px]">
      <AspectRatio ratio={4 / 3}>
        <div className="flex size-full items-center justify-center rounded-lg bg-primary/20 border border-border backdrop-blur-sm">
          <span className="text-sm text-muted-foreground">4:3</span>
        </div>
      </AspectRatio>
    </div>
  ),
};

/** 1:1 square aspect ratio */
export const Square: Story = {
  render: () => (
    <div className="w-[300px]">
      <AspectRatio ratio={1}>
        <div className="flex size-full items-center justify-center rounded-lg bg-primary/20 border border-border backdrop-blur-sm">
          <span className="text-sm text-muted-foreground">1:1</span>
        </div>
      </AspectRatio>
    </div>
  ),
};

/** Multiple aspect ratios displayed side by side for comparison */
export const Comparison: Story = {
  render: () => (
    <div className="flex flex-wrap items-start gap-4">
      <div className="w-[200px]">
        <p className="mb-2 text-xs text-muted-foreground">21:9 Ultrawide</p>
        <AspectRatio ratio={21 / 9}>
          <div className="flex size-full items-center justify-center rounded-lg bg-primary/20 border border-border backdrop-blur-sm">
            <span className="text-xs text-muted-foreground">21:9</span>
          </div>
        </AspectRatio>
      </div>
      <div className="w-[200px]">
        <p className="mb-2 text-xs text-muted-foreground">16:9 Widescreen</p>
        <AspectRatio ratio={16 / 9}>
          <div className="flex size-full items-center justify-center rounded-lg bg-primary/20 border border-border backdrop-blur-sm">
            <span className="text-xs text-muted-foreground">16:9</span>
          </div>
        </AspectRatio>
      </div>
      <div className="w-[200px]">
        <p className="mb-2 text-xs text-muted-foreground">4:3 Classic</p>
        <AspectRatio ratio={4 / 3}>
          <div className="flex size-full items-center justify-center rounded-lg bg-primary/20 border border-border backdrop-blur-sm">
            <span className="text-xs text-muted-foreground">4:3</span>
          </div>
        </AspectRatio>
      </div>
      <div className="w-[200px]">
        <p className="mb-2 text-xs text-muted-foreground">1:1 Square</p>
        <AspectRatio ratio={1}>
          <div className="flex size-full items-center justify-center rounded-lg bg-primary/20 border border-border backdrop-blur-sm">
            <span className="text-xs text-muted-foreground">1:1</span>
          </div>
        </AspectRatio>
      </div>
    </div>
  ),
};
